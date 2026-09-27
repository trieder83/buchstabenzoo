//! WebGL2 renderer (TECH-ARCH §7): instanced static batches sharing the palette texture,
//! GPU-skinned characters, 2-tone cel shading into a G-buffer and a screen-space outline
//! pass. Raw `web-sys` WebGL2 — no engine.
//!
//! Frame: G-buffer pass (colour, normal + edge mask, depth) → outline pass to the canvas.
//! Draw calls ≈ one per distinct model + placeholder boxes + characters + 1 post pass.

use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::{IVec2, Mat4, Quat, Vec2, Vec3, Vec4};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as Gl, WebGlBuffer, WebGlFramebuffer, WebGlProgram,
    WebGlShader, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject,
};
use zoo_assets::{MeshData, Model, Pose, Skeleton};
use zoo_core::water::{bob_params, water_time, TileShape, WaterField};

use crate::camera::{project, FollowCamera};
use crate::night::{self, DayLight, PointLight, MAX_LIGHT_POOLS, MAX_POINT_LIGHTS};
use crate::shaders;

/// Sun direction (towards the sun): high, from the west and slightly behind the default
/// camera, so it reads as "upper left" on screen (art/style/style.md) while faces turned to
/// the default camera stay lit like in the style frame; east faces get the shadow tone.
pub const SUN_DIR: Vec3 = Vec3::new(-0.55, 0.8, 0.3);
/// Shadow tone multiplier (blue-violet tint of the concept sheets, palette `shadow_tint`).
pub const SHADOW_TINT: Vec3 = Vec3::new(0.70, 0.68, 0.84);
/// Outline colour, dark brown (#3B2314).
pub const OUTLINE: Vec3 = Vec3::new(
    0x3B as f32 / 255.0,
    0x23 as f32 / 255.0,
    0x14 as f32 / 255.0,
);
/// Clear colour outside the level (dark grass).
const CLEAR: [f32; 4] = [0.37, 0.55, 0.24, 1.0];
/// Render resolution cap (device pixels per CSS pixel) for mobile fill rate.
pub const MAX_PIXEL_RATIO: f64 = 2.0;
const MAX_JOINTS: usize = 128;
/// Instances per model drawn with one instanced skinned draw call (ambient animals).
pub const MAX_CROWD: usize = 32;
/// Obstacles with foam per frame (nearest to the camera target, TECH-WATER behaviour 6).
pub const MAX_WATER_OBSTACLES: usize = 4;
/// Duck / frog ripples per frame (GAME-AMBIENT 5).
pub const MAX_WATER_RIPPLES: usize = 8;

/// Per-instance data of a static batch (48 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Instance {
    /// Origin (world) + yaw (radians).
    pub pos_yaw: [f32; 4],
    /// Scale xyz + fadeable flag (1 = may be faded when occluding the player).
    pub scale_fade: [f32; 4],
    /// Flat colour with `a = 1`, or `a = 0` to sample the palette texture.
    pub color: [f32; 4],
}

impl Instance {
    pub fn model(pos: Vec3, yaw: f32, fadeable: bool) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [1.0, 1.0, 1.0, f32::from(u8::from(fadeable))],
            color: [1.0, 1.0, 1.0, 0.0],
        }
    }

    /// Emissive flat colour (lamp glass, lit windows, eyeshine, fireflies; GAME-NIGHT §10):
    /// drawn unlit with its colour.
    pub fn glow(pos: Vec3, yaw: f32, scale: Vec3, color: [f32; 3]) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [scale.x, scale.y, scale.z, 0.0],
            color: [color[0], color[1], color[2], night::EMISSIVE_ALPHA],
        }
    }

    pub fn flat(pos: Vec3, yaw: f32, scale: Vec3, color: [f32; 3], fadeable: bool) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [scale.x, scale.y, scale.z, f32::from(u8::from(fadeable))],
            color: [color[0], color[1], color[2], 1.0],
        }
    }
}

struct Program {
    program: WebGlProgram,
    uniforms: HashMap<&'static str, WebGlUniformLocation>,
}

impl Program {
    fn new(gl: &Gl, vs: &str, fs: &str, names: &[&'static str]) -> Result<Self, String> {
        let v = compile(gl, Gl::VERTEX_SHADER, vs)?;
        let f = compile(gl, Gl::FRAGMENT_SHADER, fs)?;
        let program = gl.create_program().ok_or("create_program")?;
        gl.attach_shader(&program, &v);
        gl.attach_shader(&program, &f);
        gl.link_program(&program);
        if !gl
            .get_program_parameter(&program, Gl::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Err(gl.get_program_info_log(&program).unwrap_or_default());
        }
        let uniforms = names
            .iter()
            .filter_map(|n| gl.get_uniform_location(&program, n).map(|l| (*n, l)))
            .collect();
        Ok(Self { program, uniforms })
    }

    fn u(&self, name: &str) -> Option<&WebGlUniformLocation> {
        self.uniforms.get(name)
    }
}

fn compile(gl: &Gl, kind: u32, src: &str) -> Result<WebGlShader, String> {
    let s = gl.create_shader(kind).ok_or("create_shader")?;
    gl.shader_source(&s, src);
    gl.compile_shader(&s);
    if gl
        .get_shader_parameter(&s, Gl::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(s)
    } else {
        Err(gl.get_shader_info_log(&s).unwrap_or_default())
    }
}

/// Geometry of a static mesh, shared by its batches of every region.
struct MeshInfo {
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
    index_count: i32,
    edge_mask: f32,
    height: f32,
    /// Horizontal/vertical extent from the origin (m), for region bounds.
    radius: f32,
    /// A water tile (drawn by the water program, TECH-WATER).
    water: bool,
    /// Bobbing on the water (`u_bob`, TECH-WATER behaviour 8).
    bob: [f32; 4],
}

/// A render region (chunk): a level part, a barrier or a building roof. Its batches are
/// skipped when it is hidden or its bounds are outside the view frustum (QA F12).
#[derive(Debug, Clone, Copy)]
struct Region {
    min: Vec3,
    max: Vec3,
    hidden: bool,
}

impl Default for Region {
    fn default() -> Self {
        Self {
            min: Vec3::splat(f32::MAX),
            max: Vec3::splat(f32::MIN),
            hidden: false,
        }
    }
}

/// Region 0: never culled (dynamic batches, characters' helpers).
pub const REGION_ALWAYS: u16 = 0;

/// One mesh drawn with instancing (per region).
struct Batch {
    region: u16,
    vao: WebGlVertexArrayObject,
    inst_vbo: WebGlBuffer,
    index_count: i32,
    instances: Vec<Instance>,
    /// Instance count on the GPU; `usize::MAX` = needs upload.
    uploaded: usize,
    capacity: usize,
    edge_mask: f32,
    /// Tallest point of the mesh (m), decides the fade flag of its instances.
    height: f32,
    /// Re-uploaded every frame with `buffer_sub_data` (characters, placeholders that move).
    dynamic: bool,
    water: bool,
    bob: [f32; 4],
    /// Bounds of the instances per [`CHUNK_M`] ground chunk (static regions only): a batch is
    /// drawn only when one of its chunks is in view, so the short far plane of the close
    /// views culls every mesh that has no instance nearby (GAME-CAMERA-VIEWS 6).
    chunks: Vec<(IVec2, Vec3, Vec3)>,
}

/// Ground chunk size for per-batch culling (m).
pub const CHUNK_M: f32 = 8.0;

/// One material range of a skinned mesh.
struct Part {
    first_index: i32,
    index_count: i32,
    texture: WebGlTexture,
    textured: bool,
    color: [f32; 4],
}

struct SkinnedModel {
    vao: WebGlVertexArrayObject,
    parts: Vec<Part>,
    skeleton: Skeleton,
    clips: Vec<zoo_assets::Clip>,
    idle: Option<usize>,
    walk: Option<usize>,
    pose_idle: Pose,
    pose_walk: Pose,
    pose: Pose,
    world: Vec<Mat4>,
    joints: Vec<Mat4>,
    joint_data: Vec<f32>,
    joint_tex: WebGlTexture,
    /// Instanced drawing (ambient animals): one row of model × joint matrices per instance.
    crowd_data: Vec<f32>,
    crowd_tex: WebGlTexture,
}

/// A skinned character to draw this frame.
#[derive(Debug, Clone, Copy)]
pub struct CharacterDraw {
    pub pos: Vec3,
    pub yaw: f32,
    /// Seconds into the idle clip.
    pub idle_time: f32,
    /// Seconds into the walk clip.
    pub walk_time: f32,
    /// 0 = idle, 1 = walk.
    pub walk_blend: f32,
    /// Clip used as the resting loop (e.g. `drink` for an animal at the river); falls back
    /// to `idle` when the model has no such clip.
    pub idle_clip: &'static str,
    /// Locomotion clip (e.g. `swim` for a hippo in the water); falls back to `walk`.
    pub walk_clip: &'static str,
    /// One-shot action clip (`eat`, `happy`, `refuse`, …) and seconds into it.
    pub action: Option<(&'static str, f32)>,
    /// Weight of the action over the idle/walk pose (0…1).
    pub action_blend: f32,
    /// Under a water surface (the goldfish): drawn through the water with a water tint
    /// (depth pulled towards the camera by [`UNDER_WATER_DEPTH_BIAS_M`]).
    pub under_water: bool,
    /// Extra rotation before the yaw (bobbing roll of ducks and frogs).
    pub tilt: Quat,
}

/// How far an under-water character is pulled towards the camera for the depth test, so it
/// shows through the water surface above it (m).
pub const UNDER_WATER_DEPTH_BIAS_M: f32 = 0.45;
/// Water tint of under-water characters (rgb, amount).
pub const UNDER_WATER_TINT: [f32; 4] = [0.36, 0.66, 0.90, 0.35];

impl CharacterDraw {
    /// Idle/walk only.
    pub fn locomotion(
        pos: Vec3,
        yaw: f32,
        idle_time: f32,
        walk_time: f32,
        walk_blend: f32,
    ) -> Self {
        Self {
            pos,
            yaw,
            idle_time,
            walk_time,
            walk_blend,
            idle_clip: "idle",
            walk_clip: "walk",
            action: None,
            action_blend: 0.0,
            under_water: false,
            tilt: Quat::IDENTITY,
        }
    }

    fn model(&self) -> Mat4 {
        Mat4::from_translation(self.pos)
            * Mat4::from_quat(self.tilt)
            * Mat4::from_rotation_y(self.yaw)
    }
}

struct GBuffer {
    fb: WebGlFramebuffer,
    color: WebGlTexture,
    normal: WebGlTexture,
    depth: WebGlTexture,
}

/// Per-frame statistics (debug getters, performance checks).
#[derive(Debug, Clone, Copy, Default)]
pub struct FrameStats {
    pub draw_calls: u32,
    pub instances: u32,
    pub triangles: u32,
    pub decals: u32,
    /// Static batches skipped by region culling (hidden or outside the frustum).
    pub culled_batches: u32,
    /// Draw calls of the instanced crowds (ambient animals, AMB-007).
    pub crowd_draw_calls: u32,
}

/// A decal quad on the GPU: texture name, face normal, index of its first vertex.
struct DecalDraw {
    texture: String,
    normal: Vec3,
    first_vertex: usize,
    /// Bounds of the quad (frustum culling, GAME-CAMERA-VIEWS 6).
    min: Vec3,
    max: Vec3,
}

/// Generous bounds of a skinned character around its origin (giraffe 4.5 m, elephant).
const CHARACTER_HALF_M: f32 = 2.5;
const CHARACTER_HEIGHT_M: f32 = 5.5;

fn character_visible(planes: &[Vec4; 6], pos: Vec3) -> bool {
    let h = Vec3::new(CHARACTER_HALF_M, 0.0, CHARACTER_HALF_M);
    aabb_visible(
        planes,
        pos - h - Vec3::Y * 0.5,
        pos + h + Vec3::Y * CHARACTER_HEIGHT_M,
    )
}

/// All decal quads in one buffer (4 vertices each: position + uv).
#[derive(Default)]
struct Decals {
    draws: Vec<DecalDraw>,
    vertices: Vec<f32>,
    vao: Option<WebGlVertexArrayObject>,
    vbo: Option<WebGlBuffer>,
    ibo: Option<WebGlBuffer>,
    uploaded: bool,
}

pub struct Renderer {
    gl: Gl,
    canvas: HtmlCanvasElement,
    static_prog: Program,
    water_prog: Program,
    crowd_prog: Program,
    skinned_prog: Program,
    post_prog: Program,
    decal_prog: Program,
    decal_textures: HashMap<String, WebGlTexture>,
    decals: Decals,
    palette: WebGlTexture,
    batches: Vec<Batch>,
    batch_index: HashMap<(String, u16), usize>,
    meshes: HashMap<String, MeshInfo>,
    regions: Vec<Region>,
    skinned: HashMap<String, SkinnedModel>,
    gbuf: Option<GBuffer>,
    width: i32,
    height: i32,
    pixel_ratio: f32,
    pub stats: FrameStats,
    /// Water clock (`water_time`, s in [0, 16)).
    time: f32,
    /// Baked water field on the GPU and its transform (`u_field_xf`).
    field_tex: Option<WebGlTexture>,
    field_xf: [f32; 4],
    /// Water animation on (off = water tiles drawn flat by the static program; WATER-008).
    pub water_animation: bool,
    /// All foam obstacles: world x, z, radius, s, c, river flag.
    obstacles: Vec<[f32; 6]>,
    /// Per-frame uniform scratch (no allocation in the render loop).
    obstacle_u: [f32; 16],
    obstacle_b: [f32; 16],
    ripple_u: [f32; 32],
    ripple_b: [f32; 32],
    ripple_count: i32,
    crowd_names: Vec<&'static str>,
    /// Night mode (GAME-NIGHT §10): global light, point lights and light pools of the frame.
    light: DayLight,
    light_u: [f32; MAX_POINT_LIGHTS * 4],
    light_col_u: [f32; MAX_POINT_LIGHTS * 4],
    light_count: i32,
    pool_u: [f32; MAX_LIGHT_POOLS * 4],
    pool_count: i32,
}

/// Name of the built-in unit box batch (placeholders).
pub const BOX: &str = "__box";
/// Name of the built-in capsule batch (player placeholder, 1.2 m).
pub const CAPSULE: &str = "__capsule";

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(&opts, &"antialias".into(), &false.into())?;
        js_sys::Reflect::set(&opts, &"depth".into(), &false.into())?;
        js_sys::Reflect::set(&opts, &"alpha".into(), &false.into())?;
        js_sys::Reflect::set(&opts, &"powerPreference".into(), &"high-performance".into())?;
        let gl: Gl = canvas
            .get_context_with_context_options("webgl2", &opts)?
            .ok_or_else(|| JsValue::from_str("WebGL2 is not available"))?
            .dyn_into()?;

        let err = |e: String| JsValue::from_str(&e);
        let common = [
            "u_view",
            "u_view_proj",
            "u_palette",
            "u_sun_dir",
            "u_shadow_tint",
            "u_edge_mask",
            "u_fade",
            "u_dither",
            "u_tint",
            "u_night",
            "u_lights",
            "u_light_colors",
            "u_light_count",
            "u_pools",
            "u_pool_count",
        ];
        let mut static_names = common.to_vec();
        static_names.extend(["u_time", "u_bob"]);
        let static_prog = Program::new(
            &gl,
            &shaders::static_vs(),
            &shaders::static_fs(),
            &static_names,
        )
        .map_err(err)?;
        let mut water_names = static_names.clone();
        water_names.extend([
            "u_field",
            "u_field_xf",
            "u_obstacles",
            "u_obstacles_b",
            "u_ripples",
            "u_ripples_b",
            "u_ripple_count",
        ]);
        let water_prog = Program::new(
            &gl,
            &shaders::water_vs(),
            &shaders::water_fs(),
            &water_names,
        )
        .map_err(err)?;
        let mut skinned_names = common.to_vec();
        skinned_names.extend(["u_model", "u_joint_tex", "u_color", "u_eye", "u_depth_bias"]);
        let skinned_prog = Program::new(
            &gl,
            &shaders::skinned_vs(),
            &shaders::static_fs(),
            &skinned_names,
        )
        .map_err(err)?;
        let crowd_prog = Program::new(
            &gl,
            &shaders::crowd_vs(),
            &shaders::static_fs(),
            &skinned_names,
        )
        .map_err(err)?;
        let post_prog = Program::new(
            &gl,
            &shaders::post_vs(),
            &shaders::post_fs(),
            &[
                "u_color",
                "u_normal",
                "u_depth",
                "u_texel",
                "u_px",
                "u_near_far",
                "u_line_color",
                "u_inv_view_proj",
                "u_eye",
                "u_fog",
                "u_sky_top",
                "u_sky_horizon",
                "u_sky_night",
            ],
        )
        .map_err(err)?;

        let decal_prog = Program::new(
            &gl,
            &shaders::decal_vs(),
            &shaders::decal_fs(),
            &[
                "u_view_proj",
                "u_tex",
                "u_normal",
                "u_sun_dir",
                "u_shadow_tint",
                "u_night",
            ],
        )
        .map_err(err)?;

        let palette = create_texture(&gl, 1, 1, &[255, 255, 255, 255])?;
        let mut r = Self {
            gl,
            canvas,
            static_prog,
            water_prog,
            crowd_prog,
            skinned_prog,
            post_prog,
            decal_prog,
            decal_textures: HashMap::new(),
            decals: Decals::default(),
            palette,
            batches: Vec::new(),
            batch_index: HashMap::new(),
            meshes: HashMap::new(),
            regions: vec![Region::default()],
            skinned: HashMap::new(),
            gbuf: None,
            width: 0,
            height: 0,
            pixel_ratio: 1.0,
            stats: FrameStats::default(),
            time: 0.0,
            field_tex: None,
            field_xf: [0.0; 4],
            water_animation: true,
            obstacles: Vec::new(),
            obstacle_u: [0.0; 16],
            obstacle_b: [0.0; 16],
            ripple_u: [0.0; 32],
            ripple_b: [0.0; 32],
            ripple_count: 0,
            crowd_names: Vec::with_capacity(4),
            light: DayLight::default(),
            light_u: [0.0; MAX_POINT_LIGHTS * 4],
            light_col_u: [0.0; MAX_POINT_LIGHTS * 4],
            light_count: 0,
            pool_u: [0.0; MAX_LIGHT_POOLS * 4],
            pool_count: 0,
        };
        r.add_mesh(BOX, &box_mesh(), 1.0)?;
        r.add_mesh(CAPSULE, &capsule_mesh(0.3, 1.2), 1.0)?;
        let i = r.batch_index[&(CAPSULE.to_owned(), REGION_ALWAYS)];
        r.batches[i].dynamic = true;
        Ok(r)
    }

    pub fn gl(&self) -> &Gl {
        &self.gl
    }

    /// Sets the shared palette texture (`assets/textures/palette.png`, NEAREST).
    pub fn set_palette_png(&mut self, png_bytes: &[u8]) -> Result<(), JsValue> {
        let (w, h, rgba) = decode_png(png_bytes).map_err(|e| JsValue::from_str(&e))?;
        self.palette = create_texture(&self.gl, w, h, &rgba)?;
        Ok(())
    }

    /// Resizes the drawing buffer to CSS size × device pixel ratio (capped).
    pub fn resize(&mut self, css_w: f64, css_h: f64, dpr: f64) {
        let ratio = dpr.clamp(1.0, MAX_PIXEL_RATIO);
        let w = (css_w * ratio).round().max(1.0) as i32;
        let h = (css_h * ratio).round().max(1.0) as i32;
        self.pixel_ratio = ratio as f32;
        if w == self.width && h == self.height && self.gbuf.is_some() {
            return;
        }
        self.canvas.set_width(w as u32);
        self.canvas.set_height(h as u32);
        self.width = w;
        self.height = h;
        self.gbuf = self.create_gbuffer(w, h).ok();
    }

    /// Device pixels per CSS pixel of the drawing buffer.
    pub fn pixel_ratio(&self) -> f32 {
        self.pixel_ratio
    }

    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn aspect(&self) -> f32 {
        self.width.max(1) as f32 / self.height.max(1) as f32
    }

    fn create_gbuffer(&self, w: i32, h: i32) -> Result<GBuffer, JsValue> {
        let gl = &self.gl;
        let tex = |internal: u32, format: u32, ty: u32| -> Result<WebGlTexture, JsValue> {
            let t = gl.create_texture().ok_or("create_texture")?;
            gl.bind_texture(Gl::TEXTURE_2D, Some(&t));
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                Gl::TEXTURE_2D,
                0,
                internal as i32,
                w,
                h,
                0,
                format,
                ty,
                None,
            )?;
            set_nearest(gl);
            Ok(t)
        };
        let color = tex(Gl::RGBA8, Gl::RGBA, Gl::UNSIGNED_BYTE)?;
        let normal = tex(Gl::RGBA8, Gl::RGBA, Gl::UNSIGNED_BYTE)?;
        let depth = tex(Gl::DEPTH_COMPONENT24, Gl::DEPTH_COMPONENT, Gl::UNSIGNED_INT)?;
        let fb = gl.create_framebuffer().ok_or("create_framebuffer")?;
        gl.bind_framebuffer(Gl::FRAMEBUFFER, Some(&fb));
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::COLOR_ATTACHMENT0,
            Gl::TEXTURE_2D,
            Some(&color),
            0,
        );
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::COLOR_ATTACHMENT1,
            Gl::TEXTURE_2D,
            Some(&normal),
            0,
        );
        gl.framebuffer_texture_2d(
            Gl::FRAMEBUFFER,
            Gl::DEPTH_ATTACHMENT,
            Gl::TEXTURE_2D,
            Some(&depth),
            0,
        );
        let bufs = js_sys::Array::of2(
            &JsValue::from(Gl::COLOR_ATTACHMENT0),
            &JsValue::from(Gl::COLOR_ATTACHMENT1),
        );
        gl.draw_buffers(&bufs);
        let status = gl.check_framebuffer_status(Gl::FRAMEBUFFER);
        gl.bind_framebuffer(Gl::FRAMEBUFFER, None);
        if status != Gl::FRAMEBUFFER_COMPLETE {
            return Err(JsValue::from_str(&format!(
                "G-buffer incomplete: {status:#x}"
            )));
        }
        Ok(GBuffer {
            fb,
            color,
            normal,
            depth,
        })
    }

    /// Creates or replaces a decal texture (RGBA8, straight alpha, top row first; LINEAR +
    /// mipmaps so text and silhouettes stay smooth when small). Used for sign silhouettes and
    /// for text textures rendered by the host (re-uploaded on language change).
    pub fn set_decal_texture_rgba(
        &mut self,
        name: &str,
        w: u32,
        h: u32,
        rgba: &[u8],
    ) -> Result<(), JsValue> {
        if w == 0 || h == 0 || rgba.len() != (w * h * 4) as usize {
            return Err(JsValue::from_str(&format!(
                "decal texture {name}: {w}×{h} does not match {} bytes",
                rgba.len()
            )));
        }
        let gl = &self.gl;
        let t = match self.decal_textures.get(name) {
            Some(t) => t.clone(),
            None => gl.create_texture().ok_or("create_texture")?,
        };
        gl.bind_texture(Gl::TEXTURE_2D, Some(&t));
        gl.pixel_storei(Gl::UNPACK_ALIGNMENT, 1);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl::TEXTURE_2D,
            0,
            Gl::RGBA8 as i32,
            w as i32,
            h as i32,
            0,
            Gl::RGBA,
            Gl::UNSIGNED_BYTE,
            Some(rgba),
        )?;
        gl.generate_mipmap(Gl::TEXTURE_2D);
        gl.tex_parameteri(
            Gl::TEXTURE_2D,
            Gl::TEXTURE_MIN_FILTER,
            Gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::LINEAR as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);
        self.decal_textures.insert(name.to_owned(), t);
        Ok(())
    }

    /// Decodes a PNG and stores it as a decal texture.
    pub fn set_decal_texture_png(&mut self, name: &str, png_bytes: &[u8]) -> Result<(), JsValue> {
        let (w, h, rgba) = decode_png(png_bytes).map_err(|e| JsValue::from_str(&e))?;
        self.set_decal_texture_rgba(name, w, h, &rgba)
    }

    pub fn has_decal_texture(&self, name: &str) -> bool {
        self.decal_textures.contains_key(name)
    }

    /// Adds a decal quad: corners top-left, top-right, bottom-right, bottom-left (world);
    /// the image's top row maps to the top edge. Drawn only while its texture exists.
    pub fn add_decal(&mut self, texture: &str, corners: [Vec3; 4], normal: Vec3) {
        let first_vertex = self.decals.vertices.len() / 5;
        for (c, uv) in corners
            .iter()
            .zip([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]])
        {
            self.decals
                .vertices
                .extend_from_slice(&[c.x, c.y, c.z, uv[0], uv[1]]);
        }
        let min = corners.iter().fold(Vec3::splat(f32::MAX), |m, c| m.min(*c));
        let max = corners.iter().fold(Vec3::splat(f32::MIN), |m, c| m.max(*c));
        self.decals.draws.push(DecalDraw {
            texture: texture.to_owned(),
            normal: normal.normalize_or_zero(),
            first_vertex,
            min: min - Vec3::splat(0.05),
            max: max + Vec3::splat(0.05),
        });
        self.decals.uploaded = false;
    }

    fn upload_decals(&mut self) -> Result<(), JsValue> {
        if self.decals.uploaded || self.decals.draws.is_empty() {
            return Ok(());
        }
        let gl = &self.gl;
        if self.decals.vao.is_none() {
            self.decals.vao = Some(gl.create_vertex_array().ok_or("create_vertex_array")?);
            self.decals.vbo = Some(gl.create_buffer().ok_or("create_buffer")?);
            self.decals.ibo = Some(gl.create_buffer().ok_or("create_buffer")?);
        }
        gl.bind_vertex_array(self.decals.vao.as_ref());
        gl.bind_buffer(Gl::ARRAY_BUFFER, self.decals.vbo.as_ref());
        gl.buffer_data_with_u8_array(
            Gl::ARRAY_BUFFER,
            bytemuck::cast_slice(&self.decals.vertices),
            Gl::STATIC_DRAW,
        );
        attrib(gl, 0, 3, 20, 0);
        attrib(gl, 1, 2, 20, 12);
        let quads = self.decals.vertices.len() / 20;
        let indices: Vec<u16> = (0..quads as u16)
            .flat_map(|q| {
                let b = q * 4;
                [b, b + 1, b + 2, b, b + 2, b + 3]
            })
            .collect();
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, self.decals.ibo.as_ref());
        gl.buffer_data_with_u8_array(
            Gl::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&indices),
            Gl::STATIC_DRAW,
        );
        gl.bind_vertex_array(None);
        self.decals.uploaded = true;
        Ok(())
    }

    /// Number of decals drawn in the last frame (their texture exists).
    pub fn decals_drawn(&self) -> u32 {
        self.stats.decals
    }

    pub fn has_model(&self, name: &str) -> bool {
        self.meshes.contains_key(name)
    }

    /// Creates a new render region (chunk) and returns its id (> 0).
    pub fn add_region(&mut self) -> u16 {
        self.regions.push(Region::default());
        (self.regions.len() - 1) as u16
    }

    /// Hides / shows every batch of a region (opened barriers, roofs while inside).
    pub fn set_region_hidden(&mut self, region: u16, hidden: bool) {
        if let Some(r) = self.regions.get_mut(region as usize) {
            r.hidden = hidden;
        }
    }

    pub fn region_hidden(&self, region: u16) -> bool {
        self.regions.get(region as usize).is_some_and(|r| r.hidden)
    }

    /// Batch of a mesh in a region (created on first use).
    fn batch_for(&mut self, name: &str, region: u16) -> Option<usize> {
        if let Some(&i) = self.batch_index.get(&(name.to_owned(), region)) {
            return Some(i);
        }
        let m = self.meshes.get(name)?;
        let gl = &self.gl;
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&m.vbo));
        attrib(gl, 0, 3, 32, 0);
        attrib(gl, 1, 3, 32, 12);
        attrib(gl, 2, 2, 32, 24);
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&m.ibo));
        let inst_vbo = gl.create_buffer()?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&inst_vbo));
        for (k, loc) in (3..6).enumerate() {
            attrib(gl, loc, 4, 48, k as i32 * 16);
            gl.vertex_attrib_divisor(loc, 1);
        }
        gl.bind_vertex_array(None);
        let batch = Batch {
            region,
            vao,
            inst_vbo,
            index_count: m.index_count,
            instances: Vec::new(),
            uploaded: usize::MAX,
            capacity: 0,
            edge_mask: m.edge_mask,
            height: m.height,
            dynamic: false,
            water: m.water,
            bob: m.bob,
            chunks: Vec::new(),
        };
        self.batch_index
            .insert((name.to_owned(), region), self.batches.len());
        self.batches.push(batch);
        Some(self.batches.len() - 1)
    }

    fn grow_region(&mut self, region: u16, pos: Vec3, radius: f32, height: f32) {
        if region == REGION_ALWAYS {
            return;
        }
        if let Some(r) = self.regions.get_mut(region as usize) {
            r.min = r.min.min(pos - Vec3::new(radius, 0.1, radius));
            r.max = r.max.max(pos + Vec3::new(radius, height.max(0.1), radius));
        }
    }

    /// Grows the bounds of a static batch (per-batch culling, GAME-CAMERA-VIEWS 6).
    fn grow_batch(&mut self, i: usize, pos: Vec3, radius: f32, height: f32) {
        let b = &mut self.batches[i];
        let key = IVec2::new(
            (pos.x / CHUNK_M).floor() as i32,
            (pos.z / CHUNK_M).floor() as i32,
        );
        let (lo, hi) = (
            pos - Vec3::new(radius, 0.1, radius),
            pos + Vec3::new(radius, height.max(0.1), radius),
        );
        match b.chunks.iter_mut().find(|c| c.0 == key) {
            Some(c) => {
                c.1 = c.1.min(lo);
                c.2 = c.2.max(hi);
            }
            None => b.chunks.push((key, lo, hi)),
        }
    }

    /// Duration in seconds of a clip of a skinned model.
    pub fn clip_duration(&self, model: &str, clip: &str) -> Option<f32> {
        self.skinned
            .get(model)?
            .clips
            .iter()
            .find(|c| c.name == clip)
            .map(|c| c.duration)
    }

    /// Whether a skinned model has a clip of that name.
    pub fn has_clip(&self, model: &str, clip: &str) -> bool {
        self.clip_duration(model, clip).is_some()
    }

    pub fn has_skinned(&self, name: &str) -> bool {
        self.skinned.contains_key(name)
    }

    /// Adds a static mesh as an instanced batch (props use the palette texture).
    pub fn add_mesh(&mut self, name: &str, mesh: &MeshData, edge_mask: f32) -> Result<(), JsValue> {
        let gl = &self.gl;
        let mut verts = Vec::with_capacity(mesh.positions.len() * 8);
        for i in 0..mesh.positions.len() {
            verts.extend_from_slice(&mesh.positions[i]);
            verts.extend_from_slice(&mesh.normals[i]);
            verts.extend_from_slice(&mesh.uvs[i]);
        }
        let vbo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_u8_array(
            Gl::ARRAY_BUFFER,
            bytemuck::cast_slice(&verts),
            Gl::STATIC_DRAW,
        );
        let ibo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_vertex_array(None);
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        gl.buffer_data_with_u8_array(
            Gl::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&mesh.indices),
            Gl::STATIC_DRAW,
        );
        let (lo, hi) = mesh.bounds();
        let radius =
            lo.x.abs()
                .max(hi.x.abs())
                .max(lo.z.abs())
                .max(hi.z.abs())
                .max(0.01);
        // replacing a mesh drops its old batches
        let old: Vec<(String, u16)> = self
            .batch_index
            .keys()
            .filter(|(n, _)| n == name)
            .cloned()
            .collect();
        for k in &old {
            if let Some(i) = self.batch_index.remove(k) {
                self.batches[i].instances.clear();
                self.batches[i].uploaded = usize::MAX;
            }
        }
        self.meshes.insert(
            name.to_owned(),
            MeshInfo {
                vbo,
                ibo,
                index_count: mesh.indices.len() as i32,
                edge_mask,
                height: hi.y,
                radius,
                water: TileShape::of_model(name).is_some(),
                bob: bob_params(name).uniform(),
            },
        );
        self.batch_for(name, REGION_ALWAYS)
            .ok_or_else(|| JsValue::from_str("create batch"))?;
        Ok(())
    }

    /// Adds a static model loaded from a `.glb`.
    pub fn add_model(&mut self, name: &str, model: &Model, edge_mask: f32) -> Result<(), JsValue> {
        self.add_mesh(name, &model.mesh, edge_mask)
    }

    /// Adds a model instance; returns `false` if the model is unknown.
    pub fn add_instance(&mut self, name: &str, pos: Vec3, yaw: f32) -> bool {
        self.add_instance_scaled(name, pos, yaw, 1.0)
    }

    /// Adds a uniformly scaled model instance; returns `false` if the model is unknown.
    pub fn add_instance_scaled(&mut self, name: &str, pos: Vec3, yaw: f32, scale: f32) -> bool {
        self.add_instance_in(name, REGION_ALWAYS, pos, yaw, scale)
    }

    /// Adds a model instance to a render region (culled with it); `false` for an unknown model.
    pub fn add_instance_in(
        &mut self,
        name: &str,
        region: u16,
        pos: Vec3,
        yaw: f32,
        scale: f32,
    ) -> bool {
        let Some((radius, height)) = self.meshes.get(name).map(|m| (m.radius, m.height)) else {
            return false;
        };
        let Some(i) = self.batch_for(name, region) else {
            return false;
        };
        let b = &mut self.batches[i];
        let mut inst = Instance::model(pos, yaw, b.height * scale > 1.5);
        inst.scale_fade[0] = scale;
        inst.scale_fade[1] = scale;
        inst.scale_fade[2] = scale;
        b.instances.push(inst);
        b.uploaded = usize::MAX;
        self.grow_region(region, pos, radius * scale, height * scale);
        self.grow_batch(i, pos, radius * scale, height * scale);
        true
    }

    /// Adds a flat-coloured placeholder box (`pos` = bottom centre).
    pub fn add_box(&mut self, pos: Vec3, size: Vec3, yaw: f32, color: [f32; 3], fadeable: bool) {
        self.add_box_in(REGION_ALWAYS, pos, size, yaw, color, fadeable);
    }

    /// Adds a placeholder box to a render region.
    pub fn add_box_in(
        &mut self,
        region: u16,
        pos: Vec3,
        size: Vec3,
        yaw: f32,
        color: [f32; 3],
        fadeable: bool,
    ) {
        let Some(i) = self.batch_for(BOX, region) else {
            return;
        };
        let b = &mut self.batches[i];
        b.instances
            .push(Instance::flat(pos, yaw, size, color, fadeable));
        b.uploaded = usize::MAX;
        let radius = (size.x * size.x + size.z * size.z).sqrt() * 0.5;
        self.grow_region(region, pos, radius, size.y);
        self.grow_batch(i, pos, radius, size.y);
    }

    /// Replaces the instances of a dynamic batch (e.g. the player capsule) for this frame.
    /// Reuses the batch's buffers; allocation-free once the capacity is reached.
    pub fn set_dynamic_instances(&mut self, name: &str, instances: &[Instance]) {
        if let Some(&i) = self.batch_index.get(&(name.to_owned(), REGION_ALWAYS)) {
            let b = &mut self.batches[i];
            b.dynamic = true;
            b.instances.clear();
            b.instances.extend_from_slice(instances);
            b.uploaded = usize::MAX;
        }
    }

    /// Adds a skinned model (GPU skinning, clips `idle` and `walk` blended by speed).
    pub fn add_skinned(&mut self, name: &str, model: &Model) -> Result<(), JsValue> {
        let gl = &self.gl;
        let skeleton = model
            .skeleton
            .clone()
            .ok_or_else(|| JsValue::from_str("model has no skin"))?;
        if skeleton.joint_count() > MAX_JOINTS {
            return Err(JsValue::from_str("too many joints"));
        }
        let m = &model.mesh;
        let mut verts = Vec::with_capacity(m.positions.len() * 16);
        for i in 0..m.positions.len() {
            verts.extend_from_slice(&m.positions[i]);
            verts.extend_from_slice(&m.normals[i]);
            verts.extend_from_slice(&m.uvs[i]);
            verts.extend(m.joints[i].iter().map(|&j| j as f32));
            verts.extend_from_slice(&m.weights[i]);
        }
        let vao = gl.create_vertex_array().ok_or("create_vertex_array")?;
        gl.bind_vertex_array(Some(&vao));
        let vbo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_u8_array(
            Gl::ARRAY_BUFFER,
            bytemuck::cast_slice(&verts),
            Gl::STATIC_DRAW,
        );
        attrib(gl, 0, 3, 64, 0);
        attrib(gl, 1, 3, 64, 12);
        attrib(gl, 2, 2, 64, 24);
        attrib(gl, 3, 4, 64, 32);
        attrib(gl, 4, 4, 64, 48);
        let ibo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        gl.buffer_data_with_u8_array(
            Gl::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&m.indices),
            Gl::STATIC_DRAW,
        );
        gl.bind_vertex_array(None);

        // One part per material range; PNG base colour textures, else the colour factor.
        let mut textures: HashMap<usize, (WebGlTexture, bool)> = HashMap::new();
        let mut parts = Vec::new();
        let subs = if m.submeshes.is_empty() {
            vec![zoo_assets::SubMesh {
                first_index: 0,
                index_count: m.indices.len() as u32,
                material: None,
            }]
        } else {
            m.submeshes.clone()
        };
        for sub in subs {
            let mat = sub.material.and_then(|i| model.materials.get(i));
            let key = sub.material.unwrap_or(usize::MAX);
            if let std::collections::hash_map::Entry::Vacant(slot) = textures.entry(key) {
                let decoded = mat
                    .and_then(|m| m.image.as_ref())
                    .filter(|i| i.mime == "image/png")
                    .map(|i| decode_png(&i.bytes));
                let entry = match decoded {
                    Some(Ok((w, h, rgba))) => (create_texture(gl, w, h, &rgba)?, true),
                    _ => (create_texture(gl, 1, 1, &[255, 255, 255, 255])?, false),
                };
                slot.insert(entry);
            }
            let (texture, textured) = textures[&key].clone();
            parts.push(Part {
                first_index: sub.first_index as i32,
                index_count: sub.index_count as i32,
                texture,
                textured,
                color: mat.map_or([1.0; 4], |m| m.base_color),
            });
        }

        let joint_count = skeleton.joint_count();
        let joint_tex = gl.create_texture().ok_or("create_texture")?;
        gl.bind_texture(Gl::TEXTURE_2D, Some(&joint_tex));
        gl.tex_storage_2d(Gl::TEXTURE_2D, 1, Gl::RGBA32F, (joint_count * 4) as i32, 1);
        set_nearest(gl);

        let crowd_tex = gl.create_texture().ok_or("create_texture")?;
        gl.bind_texture(Gl::TEXTURE_2D, Some(&crowd_tex));
        gl.tex_storage_2d(
            Gl::TEXTURE_2D,
            1,
            Gl::RGBA32F,
            (joint_count * 4) as i32,
            MAX_CROWD as i32,
        );
        set_nearest(gl);

        let find = |n: &str| model.clips.iter().position(|c| c.name == n);
        let rest = skeleton.rest_pose();
        let nodes = skeleton.node_count();
        let sm = SkinnedModel {
            vao,
            parts,
            idle: find("idle"),
            walk: find("walk"),
            clips: model.clips.clone(),
            pose_idle: rest.clone(),
            pose_walk: rest.clone(),
            pose: rest,
            world: vec![Mat4::IDENTITY; nodes],
            joints: vec![Mat4::IDENTITY; joint_count],
            joint_data: vec![0.0; joint_count * 16],
            joint_tex,
            crowd_data: vec![0.0; joint_count * 16 * MAX_CROWD],
            crowd_tex,
            skeleton,
        };
        self.skinned.insert(name.to_owned(), sm);
        Ok(())
    }

    /// Global light of the next frames (GAME-NIGHT §10): 0 = day … 1 = night, `warm` = dusk.
    pub fn set_daylight(&mut self, light: DayLight) {
        self.light = light;
    }

    pub fn daylight(&self) -> DayLight {
        self.light
    }

    /// Point lights of this frame (hard cartoon falloff, at most [`MAX_POINT_LIGHTS`]).
    pub fn set_point_lights(&mut self, lights: &[PointLight]) {
        self.light_count = 0;
        for (k, l) in lights.iter().take(MAX_POINT_LIGHTS).enumerate() {
            self.light_u[k * 4..k * 4 + 4].copy_from_slice(&[l.pos.x, l.pos.y, l.pos.z, l.radius]);
            self.light_col_u[k * 4..k * 4 + 4].copy_from_slice(&[
                l.color.x,
                l.color.y,
                l.color.z,
                l.strength + if l.tinted { 2.0 } else { 0.0 },
            ]);
            self.light_count = k as i32 + 1;
        }
    }

    /// Light-pool decals of this frame (lamps beyond the point-light budget, Q-114): flat
    /// warm pools on the ground (`pos.xz`, radius, strength).
    pub fn set_light_pools(&mut self, pools: &[PointLight]) {
        self.pool_count = 0;
        for (k, l) in pools.iter().take(MAX_LIGHT_POOLS).enumerate() {
            self.pool_u[k * 4..k * 4 + 4]
                .copy_from_slice(&[l.pos.x, l.pos.z, l.radius, l.strength]);
            self.pool_count = k as i32 + 1;
        }
    }

    /// Point lights / light pools set for this frame (stats, NIGHT perf report).
    pub fn light_counts(&self) -> (u32, u32) {
        (self.light_count as u32, self.pool_count as u32)
    }

    /// Adds an emissive box (lamp glass, lit window) to a render region.
    pub fn add_glow_box_in(
        &mut self,
        region: u16,
        pos: Vec3,
        size: Vec3,
        yaw: f32,
        color: [f32; 3],
    ) {
        let Some(i) = self.batch_for(BOX, region) else {
            return;
        };
        let b = &mut self.batches[i];
        b.instances.push(Instance::glow(pos, yaw, size, color));
        b.uploaded = usize::MAX;
        let radius = (size.x * size.x + size.z * size.z).sqrt() * 0.5;
        self.grow_region(region, pos, radius, size.y);
        self.grow_batch(i, pos, radius, size.y);
    }

    /// Sets the water clock from the elapsed game time (TECH-WATER behaviour 9).
    pub fn set_time(&mut self, elapsed_s: f64) {
        self.time = water_time(elapsed_s);
    }

    /// Current water clock (s in [0, 16)).
    pub fn water_clock(&self) -> f32 {
        self.time
    }

    /// Uploads the baked water field (RGBA16F, LINEAR, CLAMP_TO_EDGE; TECH-WATER §3).
    pub fn set_water_field(&mut self, field: &WaterField) -> Result<(), JsValue> {
        let gl = &self.gl;
        let t = match &self.field_tex {
            Some(t) => t.clone(),
            None => gl.create_texture().ok_or("create_texture")?,
        };
        gl.bind_texture(Gl::TEXTURE_2D, Some(&t));
        let flat: Vec<f32> = field.data.iter().flatten().copied().collect();
        // SAFETY: the view is consumed by the GL call before any allocation.
        unsafe {
            let view = js_sys::Float32Array::view(&flat);
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                Gl::TEXTURE_2D,
                0,
                Gl::RGBA16F as i32,
                field.width as i32,
                field.height as i32,
                0,
                Gl::RGBA,
                Gl::FLOAT,
                Some(&view),
            )?;
        }
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::LINEAR as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::LINEAR as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);
        let size = field.size_m();
        self.field_xf = [field.origin.x, field.origin.y, 1.0 / size.x, 1.0 / size.y];
        self.field_tex = Some(t);
        Ok(())
    }

    /// All foam obstacles: world XZ, radius, river coordinates `(s, c)` and whether they
    /// stand in flowing water (TECH-WATER behaviour 6, Q-068).
    pub fn set_water_obstacles(&mut self, obstacles: &[(Vec2, f32, f32, f32, bool)]) {
        self.obstacles = obstacles
            .iter()
            .map(|&(p, r, s, c, river)| [p.x, p.y, r, s, c, f32::from(u8::from(river))])
            .collect();
    }

    /// Ripples of this frame (world XZ, unit heading, wake strength, dip ring age or < 0).
    pub fn set_water_ripples(&mut self, ripples: impl Iterator<Item = (Vec2, Vec2, f32, f32)>) {
        self.ripple_count = 0;
        for (k, (p, h, wake, ring)) in ripples.take(MAX_WATER_RIPPLES).enumerate() {
            self.ripple_u[k * 4..k * 4 + 4].copy_from_slice(&[p.x, p.y, h.x, h.y]);
            self.ripple_b[k * 4..k * 4 + 4].copy_from_slice(&[wake, ring, 0.0, 0.0]);
            self.ripple_count = k as i32 + 1;
        }
    }

    /// Picks the obstacles nearest to `center` (world XZ) into the uniform scratch.
    fn pick_obstacles(&mut self, center: Vec2) {
        self.obstacle_u = [0.0; 16];
        self.obstacle_b = [0.0; 16];
        let mut best: [(f32, usize); MAX_WATER_OBSTACLES] = [(f32::MAX, 0); MAX_WATER_OBSTACLES];
        for (i, o) in self.obstacles.iter().enumerate() {
            let d = Vec2::new(o[0], o[1]).distance_squared(center);
            if let Some(k) = best.iter().position(|b| d < b.0) {
                best[k..].rotate_right(1);
                best[k] = (d, i);
            }
        }
        for (k, (d, i)) in best.iter().enumerate() {
            if *d == f32::MAX {
                continue;
            }
            let o = self.obstacles[*i];
            self.obstacle_u[k * 4..k * 4 + 4].copy_from_slice(&o[..4]);
            self.obstacle_b[k * 4..k * 4 + 2].copy_from_slice(&o[4..6]);
        }
    }

    fn upload(gl: &Gl, b: &mut Batch) {
        if b.uploaded == b.instances.len() {
            return;
        }
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&b.inst_vbo));
        let bytes: &[u8] = bytemuck::cast_slice(&b.instances);
        if b.instances.len() > b.capacity {
            b.capacity = b.instances.len().next_power_of_two();
            gl.buffer_data_with_i32(
                Gl::ARRAY_BUFFER,
                (b.capacity * std::mem::size_of::<Instance>()) as i32,
                if b.dynamic {
                    Gl::DYNAMIC_DRAW
                } else {
                    Gl::STATIC_DRAW
                },
            );
        }
        gl.buffer_sub_data_with_i32_and_u8_array(Gl::ARRAY_BUFFER, 0, bytes);
        b.uploaded = b.instances.len();
    }

    fn set_common(&self, p: &Program, view: &Mat4, view_proj: &Mat4, fade: Vec4) {
        let gl = &self.gl;
        gl.use_program(Some(&p.program));
        gl.uniform_matrix4fv_with_f32_array(p.u("u_view"), false, &view.to_cols_array());
        gl.uniform_matrix4fv_with_f32_array(p.u("u_view_proj"), false, &view_proj.to_cols_array());
        let sun = SUN_DIR.normalize();
        gl.uniform3f(p.u("u_sun_dir"), sun.x, sun.y, sun.z);
        gl.uniform3f(
            p.u("u_shadow_tint"),
            SHADOW_TINT.x,
            SHADOW_TINT.y,
            SHADOW_TINT.z,
        );
        gl.uniform4f(p.u("u_fade"), fade.x, fade.y, fade.z, fade.w);
        gl.uniform1f(p.u("u_dither"), self.outline_px() * 0.5);
        gl.uniform1i(p.u("u_palette"), 0);
        gl.uniform4f(p.u("u_tint"), 0.0, 0.0, 0.0, 0.0);
        gl.uniform4f(p.u("u_night"), self.light.night, self.light.warm, 0.0, 0.0);
        gl.uniform4fv_with_f32_array(p.u("u_lights"), &self.light_u);
        gl.uniform4fv_with_f32_array(p.u("u_light_colors"), &self.light_col_u);
        gl.uniform1i(p.u("u_light_count"), self.light_count);
        gl.uniform4fv_with_f32_array(p.u("u_pools"), &self.pool_u);
        gl.uniform1i(p.u("u_pool_count"), self.pool_count);
    }

    /// Outline sample offset in device pixels (even, so the fade pattern stays line-free).
    fn outline_px(&self) -> f32 {
        2.0 * self.pixel_ratio.round().max(1.0)
    }

    /// Renders one frame. `player` is the player's feet position (for the occluder fade);
    /// `characters` are skinned models to draw.
    pub fn render(
        &mut self,
        camera: &FollowCamera,
        player: Vec3,
        characters: &[(&str, CharacterDraw)],
        crowd: &[(&'static str, CharacterDraw)],
    ) {
        let (w, h) = (self.width, self.height);
        if w <= 0 || h <= 0 {
            return;
        }
        let mut stats = FrameStats::default();
        let view = camera.view();
        let view_proj = camera.projection(self.aspect()) * view;

        // Occluder fade (GAME-PLAYER §2): screen circle around the player's chest.
        let chest = player + Vec3::Y * 0.7;
        // (not in first person, GAME-CAMERA-VIEWS 8)
        let fade = match project(view_proj, chest).filter(|_| camera.occluder_fade()) {
            Some(ndc) => {
                let px = Vec2::new(
                    (ndc.x * 0.5 + 0.5) * w as f32,
                    (ndc.y * 0.5 + 0.5) * h as f32,
                );
                let px_per_m = h as f32
                    / (2.0 * camera.fade_distance() * (camera.fov_deg().to_radians() * 0.5).tan());
                let depth = -(view * chest.extend(1.0)).z;
                Vec4::new(px.x, px.y, 1.1 * px_per_m, depth)
            }
            None => Vec4::ZERO,
        };

        let _ = self.upload_decals();
        let target = camera.target;
        self.pick_obstacles(Vec2::new(target.x, target.z));
        let gl = &self.gl;
        for b in &mut self.batches {
            Self::upload(gl, b);
        }

        let Some(g) = &self.gbuf else { return };
        gl.bind_framebuffer(Gl::FRAMEBUFFER, Some(&g.fb));
        gl.viewport(0, 0, w, h);
        gl.enable(Gl::DEPTH_TEST);
        gl.depth_func(Gl::LEQUAL);
        gl.depth_mask(true);
        gl.enable(Gl::CULL_FACE);
        gl.cull_face(Gl::BACK);
        gl.disable(Gl::BLEND);
        let clear = Vec3::from_slice(&CLEAR[..3]).lerp(night::NIGHT_CLEAR, self.light.night);
        gl.clear_bufferfv_with_f32_array(Gl::COLOR, 0, &[clear.x, clear.y, clear.z, 1.0]);
        gl.clear_bufferfv_with_f32_array(Gl::COLOR, 1, &[0.5, 1.0, 0.5, 0.0]);
        gl.clear_bufferfi(Gl::DEPTH_STENCIL, 0, 1.0, 0);

        // Static batches (water tiles last, with the water program; TECH-WATER).
        self.set_common(&self.static_prog, &view, &view_proj, fade);
        let gl = &self.gl;
        gl.uniform1f(self.static_prog.u("u_time"), self.time);
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&self.palette));
        let planes = frustum_planes(&view_proj);
        let ground = visible_ground(&view_proj);
        let visible: Vec<bool> = self
            .regions
            .iter()
            .enumerate()
            .map(|(k, r)| {
                k == REGION_ALWAYS as usize
                    || (!r.hidden
                        && aabb_visible(&planes, r.min, r.max)
                        && ground.is_none_or(|(lo, hi)| {
                            r.min.x <= hi.x && r.max.x >= lo.x && r.min.z <= hi.y && r.max.z >= lo.y
                        }))
            })
            .collect();
        let water_prog = self.water_animation && self.field_tex.is_some();
        for pass in [false, true] {
            if pass && water_prog {
                self.set_common(&self.water_prog, &view, &view_proj, fade);
                let gl = &self.gl;
                let p = &self.water_prog;
                gl.uniform1f(p.u("u_time"), self.time);
                gl.uniform4f(p.u("u_bob"), 0.0, 0.0, 0.0, 0.0);
                gl.active_texture(Gl::TEXTURE2);
                gl.bind_texture(Gl::TEXTURE_2D, self.field_tex.as_ref());
                gl.uniform1i(p.u("u_field"), 2);
                gl.active_texture(Gl::TEXTURE0);
                let xf = self.field_xf;
                gl.uniform4f(p.u("u_field_xf"), xf[0], xf[1], xf[2], xf[3]);
                gl.uniform4fv_with_f32_array(p.u("u_obstacles"), &self.obstacle_u);
                gl.uniform4fv_with_f32_array(p.u("u_obstacles_b"), &self.obstacle_b);
                gl.uniform4fv_with_f32_array(p.u("u_ripples"), &self.ripple_u);
                gl.uniform4fv_with_f32_array(p.u("u_ripples_b"), &self.ripple_b);
                gl.uniform1i(p.u("u_ripple_count"), self.ripple_count);
            }
            let prog = if pass && water_prog {
                &self.water_prog
            } else {
                &self.static_prog
            };
            let gl = &self.gl;
            for b in &self.batches {
                if b.instances.is_empty() || b.water != pass {
                    continue;
                }
                if !visible.get(b.region as usize).copied().unwrap_or(true)
                    || (b.region != REGION_ALWAYS
                        && !b.chunks.iter().any(|c| aabb_visible(&planes, c.1, c.2)))
                {
                    stats.culled_batches += 1;
                    continue;
                }
                gl.uniform1f(prog.u("u_edge_mask"), b.edge_mask);
                if !pass {
                    let bob = if self.water_animation {
                        b.bob
                    } else {
                        [0.0; 4]
                    };
                    gl.uniform4f(prog.u("u_bob"), bob[0], bob[1], bob[2], bob[3]);
                }
                gl.bind_vertex_array(Some(&b.vao));
                gl.draw_elements_instanced_with_i32(
                    Gl::TRIANGLES,
                    b.index_count,
                    Gl::UNSIGNED_INT,
                    0,
                    b.instances.len() as i32,
                );
                stats.draw_calls += 1;
                stats.instances += b.instances.len() as u32;
                stats.triangles += (b.index_count as u32 / 3) * b.instances.len() as u32;
            }
        }

        // Skinned characters.
        if !characters.is_empty() {
            self.set_common(&self.skinned_prog, &view, &view_proj, fade);
        }
        for (name, draw) in characters {
            if !character_visible(&planes, draw.pos) {
                continue; // beyond the close views' far plane or off screen
            }
            let Some(sm) = self.skinned.get_mut(*name) else {
                continue;
            };
            pose_character(sm, draw);
            let gl = &self.gl;
            let p = &self.skinned_prog;
            gl.active_texture(Gl::TEXTURE1);
            gl.bind_texture(Gl::TEXTURE_2D, Some(&sm.joint_tex));
            // SAFETY: the view is consumed by the GL call before any allocation.
            unsafe {
                let view = js_sys::Float32Array::view(&sm.joint_data);
                let _ = gl
                    .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
                        Gl::TEXTURE_2D,
                        0,
                        0,
                        0,
                        (sm.joints.len() * 4) as i32,
                        1,
                        Gl::RGBA,
                        Gl::FLOAT,
                        Some(&view),
                    );
            }
            gl.uniform1i(p.u("u_joint_tex"), 1);
            gl.uniform1f(p.u("u_edge_mask"), 1.0);
            let eye = camera.eye();
            gl.uniform3f(p.u("u_eye"), eye.x, eye.y, eye.z);
            if draw.under_water {
                let t = UNDER_WATER_TINT;
                gl.uniform4f(p.u("u_tint"), t[0], t[1], t[2], t[3]);
                gl.uniform1f(p.u("u_depth_bias"), UNDER_WATER_DEPTH_BIAS_M);
            } else {
                gl.uniform4f(p.u("u_tint"), 0.0, 0.0, 0.0, 0.0);
                gl.uniform1f(p.u("u_depth_bias"), 0.0);
            }
            let model = draw.model();
            gl.uniform_matrix4fv_with_f32_array(p.u("u_model"), false, &model.to_cols_array());
            gl.bind_vertex_array(Some(&sm.vao));
            gl.active_texture(Gl::TEXTURE0);
            for part in &sm.parts {
                gl.bind_texture(Gl::TEXTURE_2D, Some(&part.texture));
                let c = if part.textured {
                    [1.0, 1.0, 1.0, 0.0]
                } else {
                    [part.color[0], part.color[1], part.color[2], 1.0]
                };
                gl.uniform4f(p.u("u_color"), c[0], c[1], c[2], c[3]);
                gl.draw_elements_with_i32(
                    Gl::TRIANGLES,
                    part.index_count,
                    Gl::UNSIGNED_INT,
                    part.first_index * 4,
                );
                stats.draw_calls += 1;
                stats.triangles += part.index_count as u32 / 3;
            }
        }

        // Instanced skinned crowds (ambient animals): one draw call per model and material.
        self.crowd_names.clear();
        for (name, _) in crowd {
            if !self.crowd_names.contains(name) && self.skinned.contains_key(*name) {
                self.crowd_names.push(name);
            }
        }
        if !self.crowd_names.is_empty() {
            self.set_common(&self.crowd_prog, &view, &view_proj, fade);
            let gl = &self.gl;
            let p = &self.crowd_prog;
            gl.uniform1f(p.u("u_edge_mask"), 1.0);
            gl.uniform1f(p.u("u_depth_bias"), 0.0);
            gl.uniform4f(p.u("u_tint"), 0.0, 0.0, 0.0, 0.0);
            gl.uniform_matrix4fv_with_f32_array(
                p.u("u_model"),
                false,
                &Mat4::IDENTITY.to_cols_array(),
            );
            gl.uniform1i(p.u("u_joint_tex"), 1);
        }
        for k in 0..self.crowd_names.len() {
            let name = self.crowd_names[k];
            let Some(sm) = self.skinned.get_mut(name) else {
                continue;
            };
            let n_joints = sm.joints.len();
            let mut n = 0usize;
            for (_, draw) in crowd
                .iter()
                .filter(|(m, d)| *m == name && character_visible(&planes, d.pos))
                .take(MAX_CROWD)
            {
                pose_character(sm, draw);
                let model = draw.model();
                for (j, m) in sm.joints.iter().enumerate() {
                    let o = (n * n_joints + j) * 16;
                    sm.crowd_data[o..o + 16].copy_from_slice(&(model * *m).to_cols_array());
                }
                n += 1;
            }
            if n == 0 {
                continue;
            }
            let gl = &self.gl;
            let p = &self.crowd_prog;
            gl.active_texture(Gl::TEXTURE1);
            gl.bind_texture(Gl::TEXTURE_2D, Some(&sm.crowd_tex));
            // SAFETY: the view is consumed by the GL call before any allocation.
            unsafe {
                let view = js_sys::Float32Array::view(&sm.crowd_data[..n * n_joints * 16]);
                let _ = gl
                    .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
                        Gl::TEXTURE_2D,
                        0,
                        0,
                        0,
                        (n_joints * 4) as i32,
                        n as i32,
                        Gl::RGBA,
                        Gl::FLOAT,
                        Some(&view),
                    );
            }
            gl.bind_vertex_array(Some(&sm.vao));
            gl.active_texture(Gl::TEXTURE0);
            for part in &sm.parts {
                gl.bind_texture(Gl::TEXTURE_2D, Some(&part.texture));
                let c = if part.textured {
                    [1.0, 1.0, 1.0, 0.0]
                } else {
                    [part.color[0], part.color[1], part.color[2], 1.0]
                };
                gl.uniform4f(p.u("u_color"), c[0], c[1], c[2], c[3]);
                gl.draw_elements_instanced_with_i32(
                    Gl::TRIANGLES,
                    part.index_count,
                    Gl::UNSIGNED_INT,
                    part.first_index * 4,
                    n as i32,
                );
                stats.draw_calls += 1;
                stats.crowd_draw_calls += 1;
                stats.triangles += part.index_count as u32 / 3 * n as u32;
            }
        }

        // Decals over their faces (sign silhouettes and texts, ART-ENVIRONMENT 6/7).
        if self.decals.uploaded {
            let gl = &self.gl;
            let p = &self.decal_prog;
            gl.use_program(Some(&p.program));
            gl.uniform_matrix4fv_with_f32_array(
                p.u("u_view_proj"),
                false,
                &view_proj.to_cols_array(),
            );
            let sun = SUN_DIR.normalize();
            gl.uniform3f(p.u("u_sun_dir"), sun.x, sun.y, sun.z);
            gl.uniform3f(
                p.u("u_shadow_tint"),
                SHADOW_TINT.x,
                SHADOW_TINT.y,
                SHADOW_TINT.z,
            );
            gl.uniform1i(p.u("u_tex"), 0);
            gl.uniform4f(p.u("u_night"), self.light.night, self.light.warm, 0.0, 0.0);
            gl.active_texture(Gl::TEXTURE0);
            gl.enable(Gl::BLEND);
            // colour and normal rgb blend by the decal alpha; the edge mask (dst alpha) stays
            gl.blend_func_separate(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA, Gl::ZERO, Gl::ONE);
            gl.depth_mask(false);
            gl.disable(Gl::CULL_FACE);
            gl.enable(Gl::POLYGON_OFFSET_FILL);
            gl.polygon_offset(-1.0, -4.0);
            gl.bind_vertex_array(self.decals.vao.as_ref());
            for d in &self.decals.draws {
                if !aabb_visible(&planes, d.min, d.max) {
                    continue;
                }
                let Some(t) = self.decal_textures.get(&d.texture) else {
                    continue;
                };
                gl.bind_texture(Gl::TEXTURE_2D, Some(t));
                gl.uniform3f(p.u("u_normal"), d.normal.x, d.normal.y, d.normal.z);
                gl.draw_elements_with_i32(
                    Gl::TRIANGLES,
                    6,
                    Gl::UNSIGNED_SHORT,
                    (d.first_vertex / 4 * 6 * 2) as i32,
                );
                stats.draw_calls += 1;
                stats.triangles += 2;
                stats.decals += 1;
            }
            gl.disable(Gl::POLYGON_OFFSET_FILL);
            gl.depth_mask(true);
            gl.disable(Gl::BLEND);
        }

        // Outline pass to the canvas.
        let gl = &self.gl;
        gl.bind_vertex_array(None);
        gl.bind_framebuffer(Gl::FRAMEBUFFER, None);
        gl.viewport(0, 0, w, h);
        gl.disable(Gl::DEPTH_TEST);
        gl.disable(Gl::CULL_FACE);
        let p = &self.post_prog;
        gl.use_program(Some(&p.program));
        for (unit, tex, name) in [
            (0, &g.color, "u_color"),
            (1, &g.normal, "u_normal"),
            (2, &g.depth, "u_depth"),
        ] {
            gl.active_texture(Gl::TEXTURE0 + unit);
            gl.bind_texture(Gl::TEXTURE_2D, Some(tex));
            gl.uniform1i(p.u(name), unit as i32);
        }
        gl.active_texture(Gl::TEXTURE0);
        gl.uniform2f(p.u("u_texel"), 1.0 / w as f32, 1.0 / h as f32);
        gl.uniform1f(p.u("u_px"), self.outline_px());
        gl.uniform2f(p.u("u_near_far"), camera.near(), camera.far());
        // sky + distance haze of the close views (GAME-CAMERA-VIEWS 5, 7; sky.rs)
        let inv = view_proj.inverse();
        gl.uniform_matrix4fv_with_f32_array(p.u("u_inv_view_proj"), false, &inv.to_cols_array());
        let eye = camera.eye();
        gl.uniform3f(p.u("u_eye"), eye.x, eye.y, eye.z);
        let fog = camera.fog();
        gl.uniform4f(
            p.u("u_fog"),
            fog.start,
            fog.end,
            fog.amount,
            camera.sky_amount(),
        );
        gl.uniform3f(p.u("u_line_color"), OUTLINE.x, OUTLINE.y, OUTLINE.z);
        let (top, horizon) = crate::sky::sky_colors(self.light.night);
        gl.uniform3f(p.u("u_sky_top"), top.x, top.y, top.z);
        gl.uniform3f(p.u("u_sky_horizon"), horizon.x, horizon.y, horizon.z);
        gl.uniform1f(p.u("u_sky_night"), self.light.night);
        gl.draw_arrays(Gl::TRIANGLES, 0, 3);
        stats.draw_calls += 1;
        self.stats = stats;
    }
}

/// The 6 planes (a, b, c, d; inside ≥ 0) of a view-projection matrix (Gribb/Hartmann).
fn frustum_planes(m: &Mat4) -> [Vec4; 6] {
    let r0 = m.row(0);
    let r1 = m.row(1);
    let r2 = m.row(2);
    let r3 = m.row(3);
    [r3 + r0, r3 - r0, r3 + r1, r3 - r1, r3 + r2, r3 - r2]
}

/// Tallest static geometry (m): the giant tree of level 2.
const MAX_SCENE_HEIGHT_M: f32 = 13.0;

/// World `(x, z)` bounds of what the camera can see between the ground and
/// [`MAX_SCENE_HEIGHT_M`] (the frustum corner rays cut with both planes, or their far end):
/// big regions whose box straddles the frustum planes are still culled when they lie outside
/// the visible ground (QA F12).
fn visible_ground(view_proj: &Mat4) -> Option<(Vec2, Vec2)> {
    let inv = view_proj.inverse();
    let mut lo = Vec2::splat(f32::MAX);
    let mut hi = Vec2::splat(f32::MIN);
    for (x, y) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
        let near = inv.project_point3(Vec3::new(x, y, -1.0));
        let far = inv.project_point3(Vec3::new(x, y, 1.0));
        if !near.is_finite() || !far.is_finite() {
            return None;
        }
        for h in [0.0, MAX_SCENE_HEIGHT_M] {
            let d = far - near;
            let p = if d.y.abs() > 1e-5 {
                let t = ((h - near.y) / d.y).clamp(0.0, 1.0);
                near + d * t
            } else {
                far
            };
            lo = lo.min(Vec2::new(p.x, p.z));
            hi = hi.max(Vec2::new(p.x, p.z));
        }
    }
    Some((lo, hi))
}

/// Whether an axis-aligned box intersects the frustum (conservative).
fn aabb_visible(planes: &[Vec4; 6], min: Vec3, max: Vec3) -> bool {
    if min.x > max.x {
        return false; // empty region
    }
    planes.iter().all(|p| {
        let v = Vec3::new(
            if p.x >= 0.0 { max.x } else { min.x },
            if p.y >= 0.0 { max.y } else { min.y },
            if p.z >= 0.0 { max.z } else { min.z },
        );
        p.x * v.x + p.y * v.y + p.z * v.z + p.w >= 0.0
    })
}

fn pose_character(sm: &mut SkinnedModel, d: &CharacterDraw) {
    let skel = &sm.skeleton;
    let w = d.walk_blend.clamp(0.0, 1.0);
    let idle = sm
        .clips
        .iter()
        .position(|c| c.name == d.idle_clip)
        .or(sm.idle);
    let walk = sm
        .clips
        .iter()
        .position(|c| c.name == d.walk_clip)
        .or(sm.walk);
    match (idle, walk) {
        (Some(i), Some(k)) => {
            skel.sample(&sm.clips[i], d.idle_time, &mut sm.pose_idle);
            skel.sample(&sm.clips[k], d.walk_time, &mut sm.pose_walk);
            Skeleton::blend(&sm.pose_idle, &sm.pose_walk, w, &mut sm.pose);
        }
        (Some(i), None) => skel.sample(&sm.clips[i], d.idle_time, &mut sm.pose),
        (None, Some(k)) => {
            skel.sample(&sm.clips[k], d.walk_time, &mut sm.pose_walk);
            let rest = &sm.pose_idle; // never sampled: stays the rest pose
            Skeleton::blend(rest, &sm.pose_walk, w, &mut sm.pose);
        }
        (None, None) => {}
    }
    let action = d
        .action
        .and_then(|(name, t)| sm.clips.iter().position(|c| c.name == name).map(|i| (i, t)));
    if let Some((i, t)) = action {
        // pose_walk and pose_idle are free again: action → pose_walk, blend → pose_idle.
        skel.sample(&sm.clips[i], t, &mut sm.pose_walk);
        Skeleton::blend(
            &sm.pose,
            &sm.pose_walk,
            d.action_blend.clamp(0.0, 1.0),
            &mut sm.pose_idle,
        );
        std::mem::swap(&mut sm.pose, &mut sm.pose_idle);
    }
    skel.joint_matrices(&sm.pose, &mut sm.world, &mut sm.joints);
    for (k, m) in sm.joints.iter().enumerate() {
        sm.joint_data[k * 16..k * 16 + 16].copy_from_slice(&m.to_cols_array());
    }
}

fn attrib(gl: &Gl, loc: u32, size: i32, stride: i32, offset: i32) {
    gl.enable_vertex_attrib_array(loc);
    gl.vertex_attrib_pointer_with_i32(loc, size, Gl::FLOAT, false, stride, offset);
}

fn set_nearest(gl: &Gl) {
    gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::NEAREST as i32);
    gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::NEAREST as i32);
    gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);
}

fn create_texture(gl: &Gl, w: u32, h: u32, rgba: &[u8]) -> Result<WebGlTexture, JsValue> {
    let t = gl.create_texture().ok_or("create_texture")?;
    gl.bind_texture(Gl::TEXTURE_2D, Some(&t));
    gl.pixel_storei(Gl::UNPACK_ALIGNMENT, 1);
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        Gl::TEXTURE_2D,
        0,
        Gl::RGBA8 as i32,
        w as i32,
        h as i32,
        0,
        Gl::RGBA,
        Gl::UNSIGNED_BYTE,
        Some(rgba),
    )?;
    set_nearest(gl);
    Ok(t)
}

/// Decodes a PNG into RGBA8 (palette, character textures).
pub fn decode_png(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(bytes));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = dec.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size().ok_or("png too large")?];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let (w, h) = (info.width, info.height);
    let px = (w * h) as usize;
    let rgba = match info.color_type {
        png::ColorType::Rgba => buf[..px * 4].to_vec(),
        png::ColorType::Rgb => buf[..px * 3]
            .chunks(3)
            .flat_map(|c| [c[0], c[1], c[2], 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => buf[..px * 2]
            .chunks(2)
            .flat_map(|c| [c[0], c[0], c[0], c[1]])
            .collect(),
        png::ColorType::Grayscale => buf[..px].iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err("indexed png not expanded".into()),
    };
    Ok((w, h, rgba))
}

/// Unit box: x, z in [-0.5, 0.5], y in [0, 1], flat normals, no bottom face.
/// Unit cylinder (radius 0.5, height 1, origin at the bottom centre) with `segments` flat
/// sides: optional top cap, optional inner walls (an open glass bowl seen from above).
pub fn cylinder_mesh(segments: u32, top: bool, inner: bool) -> MeshData {
    let mut m = MeshData::default();
    let n = segments.max(3);
    for k in 0..n {
        let a0 = k as f32 / n as f32 * std::f32::consts::TAU;
        let a1 = (k + 1) as f32 / n as f32 * std::f32::consts::TAU;
        let (p0, p1) = (
            Vec3::new(a0.cos() * 0.5, 0.0, a0.sin() * 0.5),
            Vec3::new(a1.cos() * 0.5, 0.0, a1.sin() * 0.5),
        );
        let mid = (a0 + a1) / 2.0;
        let normal = Vec3::new(mid.cos(), 0.0, mid.sin());
        for (sign, flip) in [(1.0f32, false), (-1.0, true)] {
            if flip && !inner {
                continue;
            }
            let base = m.positions.len() as u32;
            for p in [p0, p1, p1 + Vec3::Y, p0 + Vec3::Y] {
                m.positions.push(p.to_array());
                m.normals.push((normal * sign).to_array());
                m.uvs.push([0.0, 0.0]);
            }
            // outer walls wind counter-clockwise seen from outside
            if flip {
                m.indices
                    .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            } else {
                m.indices
                    .extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }
        }
    }
    if top {
        let c = m.positions.len() as u32;
        m.positions.push([0.0, 1.0, 0.0]);
        m.normals.push([0.0, 1.0, 0.0]);
        m.uvs.push([0.0, 0.0]);
        for k in 0..=n {
            let a = k as f32 / n as f32 * std::f32::consts::TAU;
            m.positions.push([a.cos() * 0.5, 1.0, a.sin() * 0.5]);
            m.normals.push([0.0, 1.0, 0.0]);
            m.uvs.push([0.0, 0.0]);
        }
        for k in 0..n {
            m.indices.extend_from_slice(&[c, c + 2 + k, c + 1 + k]);
        }
    }
    m
}

/// Butterfly (GAME-AMBIENT 8): two flat wings (a little oversized, comic style) in the XZ plane, facing up, body along +Z;
/// the wing beat is an instance scale on X (0 = folded, 1 = open).
pub fn butterfly_mesh() -> MeshData {
    let mut m = MeshData::default();
    for side in [-1.0f32, 1.0] {
        let base = m.positions.len() as u32;
        // comic-sized (≈ 0.34 m wingspan) so it reads from the high camera
        for (x, z) in [
            (0.0, 0.035),
            (0.085, 0.06),
            (0.095, -0.005),
            (0.06, -0.055),
            (0.0, -0.03),
        ] {
            m.positions.push([side * x * 1.8, 0.0, z * 1.8]);
            m.normals.push([0.0, 1.0, 0.0]);
            m.uvs.push([0.0, 0.0]);
        }
        let fan = [[0, 1, 2], [0, 2, 3], [0, 3, 4]];
        for t in fan {
            // counter-clockwise seen from above (+Y)
            let t = if side < 0.0 { [t[0], t[2], t[1]] } else { t };
            m.indices.extend(t.iter().map(|k| base + k));
        }
    }
    m
}

pub fn box_mesh() -> MeshData {
    let mut m = MeshData::default();
    let faces: [(Vec3, Vec3, Vec3); 5] = [
        (Vec3::Y, Vec3::X, Vec3::NEG_Z),
        (Vec3::X, Vec3::NEG_Z, Vec3::Y),
        (Vec3::NEG_X, Vec3::Z, Vec3::Y),
        (Vec3::Z, Vec3::X, Vec3::Y),
        (Vec3::NEG_Z, Vec3::NEG_X, Vec3::Y),
    ];
    for (n, u, v) in faces {
        let base = m.positions.len() as u32;
        let c = n * 0.5 + Vec3::Y * 0.5;
        for (a, b) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            let p = c + u * (0.5 * a) + v * (0.5 * b);
            m.positions.push(p.to_array());
            m.normals.push(n.to_array());
            m.uvs.push([0.0, 0.0]);
        }
        m.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    m
}

/// Low-poly capsule standing on the origin (player placeholder).
pub fn capsule_mesh(radius: f32, height: f32) -> MeshData {
    let mut m = MeshData::default();
    let seg = 12u32;
    let rings = 8u32; // latitude rings over the whole sphere
    let half = (height - 2.0 * radius).max(0.0);
    for r in 0..=rings {
        let phi = std::f32::consts::PI * r as f32 / rings as f32; // 0 = top
        let (sp, cp) = phi.sin_cos();
        let lift = if r <= rings / 2 { half } else { 0.0 };
        for s in 0..=seg {
            let th = std::f32::consts::TAU * s as f32 / seg as f32;
            let n = Vec3::new(sp * th.cos(), cp, sp * th.sin());
            let p = n * radius + Vec3::Y * (radius + lift);
            m.positions.push(p.to_array());
            m.normals.push(n.to_array());
            m.uvs.push([0.0, 0.0]);
        }
    }
    // Extra ring pair for the cylinder is implied by the duplicated equator rows.
    for r in 0..rings {
        for s in 0..seg {
            let a = r * (seg + 1) + s;
            let b = a + seg + 1;
            m.indices.extend_from_slice(&[a, a + 1, b, a + 1, b + 1, b]);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_mesh_is_unit_and_on_the_ground() {
        let (lo, hi) = box_mesh().bounds();
        assert!(lo.abs_diff_eq(Vec3::new(-0.5, 0.0, -0.5), 1e-6));
        assert!(hi.abs_diff_eq(Vec3::new(0.5, 1.0, 0.5), 1e-6));
    }

    #[test]
    fn butterfly_wings_face_up() {
        let m = butterfly_mesh();
        for t in m.indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|k| Vec3::from(m.positions[t[k] as usize]));
            assert!((b - a).cross(c - a).y > 0.0, "wing triangle faces down");
        }
    }

    #[test]
    fn capsule_is_player_sized() {
        let (lo, hi) = capsule_mesh(0.3, 1.2).bounds();
        assert!(lo.y.abs() < 1e-5);
        assert!((hi.y - 1.2).abs() < 1e-5);
    }

    #[test]
    fn palette_png_decodes() {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/textures/palette.png"
        ))
        .unwrap();
        let (w, h, rgba) = decode_png(&bytes).unwrap();
        assert_eq!((w, h), (256, 256));
        assert_eq!(rgba.len(), 256 * 256 * 4);
    }
}
