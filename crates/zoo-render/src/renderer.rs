//! WebGL2 renderer (TECH-ARCH §7): instanced static batches sharing the palette texture,
//! GPU-skinned characters, 2-tone cel shading into a G-buffer and a screen-space outline
//! pass. Raw `web-sys` WebGL2 — no engine.
//!
//! Frame: G-buffer pass (colour, normal + edge mask, depth) → outline pass to the canvas.
//! Draw calls ≈ one per distinct model + placeholder boxes + characters + 1 post pass.

use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3, Vec4};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as Gl, WebGlBuffer, WebGlFramebuffer, WebGlProgram,
    WebGlShader, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject,
};
use zoo_assets::{MeshData, Model, Pose, Skeleton};

use crate::camera::{project, FollowCamera, FAR_M, NEAR_M};
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

/// One mesh drawn with instancing.
struct Batch {
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
}

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
}

pub struct Renderer {
    gl: Gl,
    canvas: HtmlCanvasElement,
    static_prog: Program,
    skinned_prog: Program,
    post_prog: Program,
    palette: WebGlTexture,
    batches: Vec<Batch>,
    batch_index: HashMap<String, usize>,
    skinned: HashMap<String, SkinnedModel>,
    gbuf: Option<GBuffer>,
    width: i32,
    height: i32,
    pixel_ratio: f32,
    pub stats: FrameStats,
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
        ];
        let static_prog = Program::new(&gl, &shaders::static_vs(), &shaders::static_fs(), &common)
            .map_err(err)?;
        let mut skinned_names = common.to_vec();
        skinned_names.extend(["u_model", "u_joint_tex", "u_color"]);
        let skinned_prog = Program::new(
            &gl,
            &shaders::skinned_vs(),
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
            ],
        )
        .map_err(err)?;

        let palette = create_texture(&gl, 1, 1, &[255, 255, 255, 255])?;
        let mut r = Self {
            gl,
            canvas,
            static_prog,
            skinned_prog,
            post_prog,
            palette,
            batches: Vec::new(),
            batch_index: HashMap::new(),
            skinned: HashMap::new(),
            gbuf: None,
            width: 0,
            height: 0,
            pixel_ratio: 1.0,
            stats: FrameStats::default(),
        };
        r.add_mesh(BOX, &box_mesh(), 1.0)?;
        r.add_mesh(CAPSULE, &capsule_mesh(0.3, 1.2), 1.0)?;
        r.batches[r.batch_index[CAPSULE]].dynamic = true;
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

    pub fn has_model(&self, name: &str) -> bool {
        self.batch_index.contains_key(name)
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
        let vao = gl.create_vertex_array().ok_or("create_vertex_array")?;
        gl.bind_vertex_array(Some(&vao));
        let vbo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_u8_array(
            Gl::ARRAY_BUFFER,
            bytemuck::cast_slice(&verts),
            Gl::STATIC_DRAW,
        );
        attrib(gl, 0, 3, 32, 0);
        attrib(gl, 1, 3, 32, 12);
        attrib(gl, 2, 2, 32, 24);
        let ibo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        gl.buffer_data_with_u8_array(
            Gl::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&mesh.indices),
            Gl::STATIC_DRAW,
        );
        let inst_vbo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&inst_vbo));
        for (k, loc) in (3..6).enumerate() {
            attrib(gl, loc, 4, 48, k as i32 * 16);
            gl.vertex_attrib_divisor(loc, 1);
        }
        gl.bind_vertex_array(None);
        let height = mesh.bounds().1.y;
        let batch = Batch {
            vao,
            inst_vbo,
            index_count: mesh.indices.len() as i32,
            instances: Vec::new(),
            uploaded: usize::MAX,
            capacity: 0,
            edge_mask,
            height,
            dynamic: false,
        };
        match self.batch_index.get(name) {
            Some(&i) => self.batches[i] = batch,
            None => {
                self.batch_index.insert(name.to_owned(), self.batches.len());
                self.batches.push(batch);
            }
        }
        Ok(())
    }

    /// Adds a static model loaded from a `.glb`.
    pub fn add_model(&mut self, name: &str, model: &Model, edge_mask: f32) -> Result<(), JsValue> {
        self.add_mesh(name, &model.mesh, edge_mask)
    }

    /// Adds a model instance; returns `false` if the model is unknown.
    pub fn add_instance(&mut self, name: &str, pos: Vec3, yaw: f32) -> bool {
        let Some(&i) = self.batch_index.get(name) else {
            return false;
        };
        let b = &mut self.batches[i];
        b.instances.push(Instance::model(pos, yaw, b.height > 1.5));
        b.uploaded = usize::MAX;
        true
    }

    /// Adds a flat-coloured placeholder box (`pos` = bottom centre).
    pub fn add_box(&mut self, pos: Vec3, size: Vec3, yaw: f32, color: [f32; 3], fadeable: bool) {
        let i = self.batch_index[BOX];
        let b = &mut self.batches[i];
        b.instances
            .push(Instance::flat(pos, yaw, size, color, fadeable));
        b.uploaded = usize::MAX;
    }

    /// Replaces the instances of a dynamic batch (e.g. the player capsule) for this frame.
    /// Reuses the batch's buffers; allocation-free once the capacity is reached.
    pub fn set_dynamic_instances(&mut self, name: &str, instances: &[Instance]) {
        if let Some(&i) = self.batch_index.get(name) {
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
            skeleton,
        };
        self.skinned.insert(name.to_owned(), sm);
        Ok(())
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
        let fade = match project(view_proj, chest) {
            Some(ndc) => {
                let px = Vec2::new(
                    (ndc.x * 0.5 + 0.5) * w as f32,
                    (ndc.y * 0.5 + 0.5) * h as f32,
                );
                let px_per_m = h as f32
                    / (2.0
                        * camera.distance()
                        * (camera.params.vertical_fov_deg.to_radians() * 0.5).tan());
                let depth = -(view * chest.extend(1.0)).z;
                Vec4::new(px.x, px.y, 1.1 * px_per_m, depth)
            }
            None => Vec4::ZERO,
        };

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
        gl.clear_bufferfv_with_f32_array(Gl::COLOR, 0, &CLEAR);
        gl.clear_bufferfv_with_f32_array(Gl::COLOR, 1, &[0.5, 1.0, 0.5, 0.0]);
        gl.clear_bufferfi(Gl::DEPTH_STENCIL, 0, 1.0, 0);

        // Static batches.
        self.set_common(&self.static_prog, &view, &view_proj, fade);
        let gl = &self.gl;
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&self.palette));
        for b in &self.batches {
            if b.instances.is_empty() {
                continue;
            }
            gl.uniform1f(self.static_prog.u("u_edge_mask"), b.edge_mask);
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

        // Skinned characters.
        if !characters.is_empty() {
            self.set_common(&self.skinned_prog, &view, &view_proj, fade);
        }
        for (name, draw) in characters {
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
            let model = Mat4::from_translation(draw.pos) * Mat4::from_rotation_y(draw.yaw);
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
        gl.uniform2f(p.u("u_near_far"), NEAR_M, FAR_M);
        gl.uniform3f(p.u("u_line_color"), OUTLINE.x, OUTLINE.y, OUTLINE.z);
        gl.draw_arrays(Gl::TRIANGLES, 0, 3);
        stats.draw_calls += 1;
        self.stats = stats;
    }
}

fn pose_character(sm: &mut SkinnedModel, d: &CharacterDraw) {
    let skel = &sm.skeleton;
    let w = d.walk_blend.clamp(0.0, 1.0);
    match (sm.idle, sm.walk) {
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
