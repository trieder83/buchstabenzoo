//! WebGL2 renderer (TECH-ARCH §7): instanced static batches sharing the palette texture,
//! GPU-skinned characters, 2-tone cel shading into a G-buffer and a screen-space outline
//! pass. Raw `web-sys` WebGL2 — no engine.
//!
//! Frame: G-buffer pass (colour, normal + edge mask, depth) → outline pass to the canvas.
//! Draw calls ≈ one per distinct model + placeholder boxes + characters + 1 post pass.

use std::cell::Cell;
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

/// Per-instance data of a static batch (64 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Instance {
    /// Origin (world) + yaw (radians).
    pub pos_yaw: [f32; 4],
    /// Scale xyz + fadeable flag (1 = may be faded when occluding the player).
    pub scale_fade: [f32; 4],
    /// Flat colour with `a = 1`, or `a = 0` to sample the palette texture.
    pub color: [f32; 4],
    /// Moving parts (ARCH-007): x = open amount 0…1 (door leaves, gates), y = hide mask
    /// ([`HIDE_ROOF`] | [`HIDE_WALLS_UPPER`]), z, w unused.
    pub node: [f32; 4],
}

/// Hide-mask bit of a model's `roof` part (with its inner ceiling).
pub const HIDE_ROOF: u32 = 1;
/// Hide-mask bit of a model's `walls_upper` part.
pub const HIDE_WALLS_UPPER: u32 = 2;
/// Parts a mesh can move / hide on its own (`u_nodes` array size, part 0 = root).
pub const MAX_NODE_PARTS: usize = 8;

/// How a named part of a multi-node asset moves (README_night "Nodes", ARCH-007).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeBehaviour {
    /// Rotation axis: 0 none, 1 = +X, 2 = +Y, 3 = +Z (model space, about the pivot).
    pub axis: f32,
    /// Rotation at open = 1 (radians); for `spin` the turn per second.
    pub angle: f32,
    /// Hide-mask bit ([`HIDE_ROOF`], [`HIDE_WALLS_UPPER`]) or 0.
    pub hide_bit: f32,
    /// 0 = driven by the instance's open amount, 1 = spins with the clock (windmill sails),
    /// 2 = shown only while the glow slots are on (night sky of the moon window).
    pub mode: f32,
}

impl NodeBehaviour {
    pub const STATIC: NodeBehaviour = NodeBehaviour {
        axis: 0.0,
        angle: 0.0,
        hide_bit: 0.0,
        mode: 0.0,
    };

    fn turn(axis: f32, deg: f32) -> Self {
        Self {
            axis,
            angle: deg.to_radians(),
            ..Self::STATIC
        }
    }

    /// Behaviour of a part by its node name (kit conventions, README_night.md).
    pub fn of(name: &str) -> Self {
        match name {
            // leaves open to the back (−Z): left +90°, right −90° about +Y
            "leaf_l" => Self::turn(2.0, 90.0),
            "leaf_r" => Self::turn(2.0, -90.0),
            // key box door: −100° about +Y
            "door" => Self::turn(2.0, -100.0),
            // turnstile arms: a third of a turn per passage
            "arms" => Self::turn(2.0, 120.0),
            // toy chest lid: modelled open 70°, +70° about +X closes it (open = 1 → closed)
            "lid" => Self::turn(1.0, 70.0),
            // windmill sails: slow spin about the hub's +Z (one turn per 8 s)
            "sails" => Self {
                axis: 3.0,
                angle: std::f32::consts::TAU / 8.0,
                mode: 1.0,
                ..Self::STATIC
            },
            // water wheel: turns with the clock about +X (LAYOUT-L3-017), lower paddles with
            // the stream's flow
            "wheel" => Self {
                axis: 1.0,
                angle: zoo_core::scene::WATER_WHEEL_SPIN,
                mode: 1.0,
                ..Self::STATIC
            },
            // carousel rotor (level 3): turns about +Y with the clock, one turn per 20 s
            "rotor" => Self {
                axis: 2.0,
                angle: std::f32::consts::TAU / 20.0,
                mode: 1.0,
                ..Self::STATIC
            },
            "roof" => Self {
                hide_bit: HIDE_ROOF as f32,
                ..Self::STATIC
            },
            "walls_upper" => Self {
                hide_bit: HIDE_WALLS_UPPER as f32,
                ..Self::STATIC
            },
            "night_sky" => Self {
                mode: 2.0,
                ..Self::STATIC
            },
            _ => Self::STATIC,
        }
    }
}

/// Vertex data of a static mesh (ARCH-007): 16 floats per vertex (position, normal, uv,
/// slot colour + mode, part pivot + code; see [`static_vertices`]) and the indices with every
/// glass triangle moved to the end: `glass_first` is the index where the glass starts (drawn
/// blended after the opaque pass). Slot modes: 0 palette, 1 glow slot, 3 glass, 5 flat face
/// colour.
pub struct StaticVertices {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub glass_first: u32,
    /// `u_nodes` uniform of the mesh (4 floats per part) and whether any part moves / hides.
    pub nodes: [f32; MAX_NODE_PARTS * 4],
    pub has_nodes: bool,
}

impl StaticVertices {
    /// Whether instances can be merged into one static mesh: no glass (drawn apart) and no
    /// part that hides, spins or shows only at night (moving parts are baked at rest).
    pub fn is_bakeable(&self) -> bool {
        self.glass_first as usize == self.indices.len()
            && self.nodes.chunks(4).all(|b| b[2] == 0.0 && b[3] == 0.0)
    }

    /// Whether the mesh needs the rich vertex format: a material slot other than the
    /// palette, or a part that moves / hides.
    pub fn is_rich(&self) -> bool {
        self.has_nodes
            || self
                .vertices
                .chunks(STATIC_VERTEX_FLOATS)
                .any(|v| v[11] != 0.0)
    }
}

/// Merges static meshes placed at (position, yaw, scale) into one mesh around `origin`
/// (pure; ARCH-008): positions and normals transformed exactly like the instanced vertex
/// shader, parts baked at rest (part code 0), slot colours kept.
pub fn bake_vertices(items: &[(&StaticVertices, Vec3, f32, Vec3)], origin: Vec3) -> StaticVertices {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for (src, pos, yaw, scale) in items {
        let (s, c) = yaw.sin_cos();
        let base = (vertices.len() / STATIC_VERTEX_FLOATS) as u32;
        for v in src.vertices.chunks(STATIC_VERTEX_FLOATS) {
            let p = Vec3::new(v[0], v[1], v[2]) * *scale;
            let n = Vec3::new(v[3], v[4], v[5]) / *scale;
            let w = Vec3::new(c * p.x + s * p.z, p.y, -s * p.x + c * p.z) + *pos - origin;
            let wn = Vec3::new(c * n.x + s * n.z, n.y, -s * n.x + c * n.z).normalize_or_zero();
            vertices.extend_from_slice(&[w.x, w.y, w.z, wn.x, wn.y, wn.z, v[6], v[7]]);
            vertices.extend_from_slice(&v[8..12]);
            vertices.extend_from_slice(&[0.0; 4]);
        }
        indices.extend(src.indices.iter().map(|i| i + base));
    }
    let glass_first = indices.len() as u32;
    StaticVertices {
        vertices,
        indices,
        glass_first,
        nodes: [0.0; MAX_NODE_PARTS * 4],
        has_nodes: false,
    }
}

/// Floats per static vertex.
pub const STATIC_VERTEX_FLOATS: usize = 16;

/// Packs a static mesh with its material slots and parts (pure; ARCH-007).
pub fn static_vertices(
    mesh: &MeshData,
    materials: &[zoo_assets::Material],
    parts: &[zoo_assets::NodePart],
) -> StaticVertices {
    let n = mesh.positions.len();
    let mut glow = vec![[0.0f32; 4]; n];
    let mut glass_tri = vec![false; mesh.indices.len() / 3];
    for sub in &mesh.submeshes {
        let Some(mat) = sub.material.and_then(|i| materials.get(i)) else {
            continue;
        };
        let g = if mat.is_glass() {
            let c = mat.base_color.map(zoo_assets::linear_to_srgb);
            Some([c[0], c[1], c[2], 3.0])
        } else if mat.name.ends_with("_face") {
            // blank text face: its flat colour (UVs run 0..1 across it, not into the atlas)
            let c = mat.base_color.map(zoo_assets::linear_to_srgb);
            Some([c[0], c[1], c[2], 5.0])
        } else if mat.is_glow() {
            let e = mat.emissive_srgb();
            Some([e[0], e[1], e[2], 1.0])
        } else {
            None
        };
        let Some(g) = g else { continue };
        let (a, b) = (
            sub.first_index as usize,
            (sub.first_index + sub.index_count) as usize,
        );
        for &i in &mesh.indices[a.min(mesh.indices.len())..b.min(mesh.indices.len())] {
            glow[i as usize] = g;
        }
        if g[3] > 2.5 {
            for t in a / 3..(b / 3).min(glass_tri.len()) {
                glass_tri[t] = true;
            }
        }
    }
    let mut vertices = Vec::with_capacity(n * STATIC_VERTEX_FLOATS);
    for (i, g) in glow.iter().enumerate() {
        vertices.extend_from_slice(&mesh.positions[i]);
        vertices.extend_from_slice(&mesh.normals[i]);
        vertices.extend_from_slice(&mesh.uvs[i]);
        vertices.extend_from_slice(g);
        let code = mesh.node.get(i).copied().unwrap_or(0) as usize;
        let (pivot, code) = match parts.get(code) {
            Some(p) if code > 0 && code < MAX_NODE_PARTS => (p.pivot, code),
            _ => (Vec3::ZERO, 0),
        };
        vertices.extend_from_slice(&[pivot.x, pivot.y, pivot.z, code as f32]);
    }
    let mut indices = Vec::with_capacity(mesh.indices.len());
    for (t, tri) in mesh.indices.chunks(3).enumerate() {
        if !glass_tri[t] {
            indices.extend_from_slice(tri);
        }
    }
    let glass_first = indices.len() as u32;
    for (t, tri) in mesh.indices.chunks(3).enumerate() {
        if glass_tri[t] {
            indices.extend_from_slice(tri);
        }
    }
    let mut nodes = [0.0; MAX_NODE_PARTS * 4];
    let mut has_nodes = false;
    for (k, p) in parts.iter().enumerate().skip(1).take(MAX_NODE_PARTS - 1) {
        let b = NodeBehaviour::of(&p.name);
        nodes[k * 4..k * 4 + 4].copy_from_slice(&[b.axis, b.angle, b.hide_bit, b.mode]);
        has_nodes |= b != NodeBehaviour::STATIC;
    }
    StaticVertices {
        vertices,
        indices,
        glass_first,
        nodes,
        has_nodes,
    }
}

impl Instance {
    pub fn model(pos: Vec3, yaw: f32, fadeable: bool) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [1.0, 1.0, 1.0, f32::from(u8::from(fadeable))],
            color: [1.0, 1.0, 1.0, 0.0],
            node: [0.0; 4],
        }
    }

    /// A palette-textured model with a uniform scale.
    pub fn scaled(pos: Vec3, yaw: f32, scale: f32) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [scale, scale, scale, 0.0],
            color: [1.0, 1.0, 1.0, 0.0],
            node: [0.0; 4],
        }
    }

    /// Emissive flat colour (lamp glass, lit windows, eyeshine, fireflies; GAME-NIGHT §10):
    /// drawn unlit with its colour.
    pub fn glow(pos: Vec3, yaw: f32, scale: Vec3, color: [f32; 3]) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [scale.x, scale.y, scale.z, 0.0],
            color: [color[0], color[1], color[2], night::EMISSIVE_ALPHA],
            node: [0.0; 4],
        }
    }

    pub fn flat(pos: Vec3, yaw: f32, scale: Vec3, color: [f32; 3], fadeable: bool) -> Self {
        Self {
            pos_yaw: [pos.x, pos.y, pos.z, yaw],
            scale_fade: [scale.x, scale.y, scale.z, f32::from(u8::from(fadeable))],
            color: [color[0], color[1], color[2], 1.0],
            node: [0.0; 4],
        }
    }
}

/// Uniforms set per draw or per pass. The values shared by every scene program are in the
/// frame block ([`FrameBlock`], PERF-R-003); these locations are looked up once when a
/// program is linked (an array index per call, no name hashing).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum U {
    Palette,
    EdgeMask,
    Tint,
    LightMask,
    Bob,
    Nodes,
    Field,
    FieldXf,
    Obstacles,
    ObstaclesB,
    Ripples,
    RipplesB,
    RippleCount,
    Model,
    JointTex,
    Color,
    Eye,
    DepthBias,
    Emit,
    Tex,
    Normal,
    Depth,
    Texel,
    Px,
    NearFar,
    LineColor,
    InvViewProj,
    Fog,
    SkyTop,
    SkyHorizon,
    SkyNight,
    SkyClouds,
}

impl U {
    const ALL: [U; 32] = [
        U::Palette,
        U::EdgeMask,
        U::Tint,
        U::LightMask,
        U::Bob,
        U::Nodes,
        U::Field,
        U::FieldXf,
        U::Obstacles,
        U::ObstaclesB,
        U::Ripples,
        U::RipplesB,
        U::RippleCount,
        U::Model,
        U::JointTex,
        U::Color,
        U::Eye,
        U::DepthBias,
        U::Emit,
        U::Tex,
        U::Normal,
        U::Depth,
        U::Texel,
        U::Px,
        U::NearFar,
        U::LineColor,
        U::InvViewProj,
        U::Fog,
        U::SkyTop,
        U::SkyHorizon,
        U::SkyNight,
        U::SkyClouds,
    ];

    fn name(self) -> &'static str {
        match self {
            U::Palette => "u_palette",
            U::EdgeMask => "u_edge_mask",
            U::Tint => "u_tint",
            U::LightMask => "u_light_mask",
            U::Bob => "u_bob",
            U::Nodes => "u_nodes",
            U::Field => "u_field",
            U::FieldXf => "u_field_xf",
            U::Obstacles => "u_obstacles",
            U::ObstaclesB => "u_obstacles_b",
            U::Ripples => "u_ripples",
            U::RipplesB => "u_ripples_b",
            U::RippleCount => "u_ripple_count",
            U::Model => "u_model",
            U::JointTex => "u_joint_tex",
            U::Color => "u_color",
            U::Eye => "u_eye",
            U::DepthBias => "u_depth_bias",
            U::Emit => "u_emit",
            U::Tex => "u_tex",
            U::Normal => "u_normal",
            U::Depth => "u_depth",
            U::Texel => "u_texel",
            U::Px => "u_px",
            U::NearFar => "u_near_far",
            U::LineColor => "u_line_color",
            U::InvViewProj => "u_inv_view_proj",
            U::Fog => "u_fog",
            U::SkyTop => "u_sky_top",
            U::SkyHorizon => "u_sky_horizon",
            U::SkyNight => "u_sky_night",
            U::SkyClouds => "u_sky_clouds",
        }
    }
}

/// "Not set yet" in the uniform value cache (no real value has these bits in all lanes).
const UNSET: [u32; 4] = [u32::MAX; 4];

struct Program {
    program: WebGlProgram,
    /// Driver size of the frame block (bytes; 0 = the program does not use it).
    frame_size: usize,
    /// Location per [`U`] (`None`: not used by this program).
    loc: [Option<WebGlUniformLocation>; U::ALL.len()],
    /// Last value set per [`U`] (bit patterns): uniform values live in the program object,
    /// so a call is skipped when the value is unchanged (PERF-R-003).
    last: [Cell<[u32; 4]>; U::ALL.len()],
}

impl Program {
    fn new(gl: &Gl, vs: &str, fs: &str) -> Result<Self, String> {
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
        let loc = U::ALL.map(|u| gl.get_uniform_location(&program, u.name()));
        // the shared frame block (PERF-R-003): binding point and std140 layout check
        let block = gl.get_uniform_block_index(&program, shaders::FRAME_BLOCK_NAME);
        let mut frame_size = 0;
        if block != Gl::INVALID_INDEX {
            gl.uniform_block_binding(&program, block, shaders::FRAME_BLOCK_BINDING);
            frame_size = check_frame_block(gl, &program, block)?;
        }
        Ok(Self {
            program,
            frame_size,
            loc,
            last: std::array::from_fn(|_| Cell::new(UNSET)),
        })
    }

    fn u(&self, u: U) -> Option<&WebGlUniformLocation> {
        self.loc[u as usize].as_ref()
    }

    /// Whether `u` exists and `bits` differ from its last value (then records them).
    fn changed(&self, u: U, bits: [u32; 4]) -> bool {
        if self.loc[u as usize].is_none() {
            return false;
        }
        let c = &self.last[u as usize];
        if c.get() == bits {
            return false;
        }
        c.set(bits);
        true
    }

    fn set1f(&self, gl: &Gl, u: U, v: f32) {
        if self.changed(u, [v.to_bits(), 0, 0, 0]) {
            gl.uniform1f(self.u(u), v);
        }
    }

    fn set1i(&self, gl: &Gl, u: U, v: i32) {
        if self.changed(u, [v as u32, 0, 0, 0]) {
            gl.uniform1i(self.u(u), v);
        }
    }

    fn set2f(&self, gl: &Gl, u: U, x: f32, y: f32) {
        if self.changed(u, [x.to_bits(), y.to_bits(), 0, 0]) {
            gl.uniform2f(self.u(u), x, y);
        }
    }

    fn set3f(&self, gl: &Gl, u: U, v: Vec3) {
        if self.changed(u, [v.x.to_bits(), v.y.to_bits(), v.z.to_bits(), 0]) {
            gl.uniform3f(self.u(u), v.x, v.y, v.z);
        }
    }

    fn set4f(&self, gl: &Gl, u: U, v: [f32; 4]) {
        if self.changed(u, v.map(f32::to_bits)) {
            gl.uniform4f(self.u(u), v[0], v[1], v[2], v[3]);
        }
    }

    fn set2ui(&self, gl: &Gl, u: U, (x, y): (u32, u32)) {
        if self.changed(u, [x, y, 0, 0]) {
            gl.uniform2ui(self.u(u), x, y);
        }
    }

    /// Array / matrix uniforms: set every time (not cached).
    fn set4fv(&self, gl: &Gl, u: U, v: &[f32]) {
        if let Some(l) = self.u(u) {
            gl.uniform4fv_with_f32_array(Some(l), v);
        }
    }

    fn set_mat4(&self, gl: &Gl, u: U, m: &Mat4) {
        if let Some(l) = self.u(u) {
            gl.uniform_matrix4fv_with_f32_array(Some(l), false, &m.to_cols_array());
        }
    }
}

/// The shared per-frame uniform block (PERF-R-003), std140 — the CPU mirror of
/// `shaders::frame_block()`. Uploaded once per frame with one `bufferSubData`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct FrameBlock {
    pub view: [f32; 16],
    pub view_proj: [f32; 16],
    pub sun_dir: [f32; 3],
    pub dither: f32,
    pub shadow_tint: [f32; 3],
    pub glow_on: f32,
    pub fade: [f32; 4],
    pub night: [f32; 4],
    pub time: f32,
    pub _pad: [f32; 3],
    pub lights: [[f32; 4]; MAX_POINT_LIGHTS],
    pub light_colors: [[f32; 4]; MAX_POINT_LIGHTS],
    pub pools: [[f32; 4]; MAX_LIGHT_POOLS],
}

/// std140 offsets of the frame block members that follow padding (checked against the GL
/// driver when a program is linked, and unit-tested against [`FrameBlock`]).
const FRAME_OFFSETS: [(&str, usize); 6] = [
    ("u_sun_dir", std::mem::offset_of!(FrameBlock, sun_dir)),
    ("u_glow_on", std::mem::offset_of!(FrameBlock, glow_on)),
    ("u_time", std::mem::offset_of!(FrameBlock, time)),
    ("u_lights[0]", std::mem::offset_of!(FrameBlock, lights)),
    (
        "u_light_colors[0]",
        std::mem::offset_of!(FrameBlock, light_colors),
    ),
    ("u_pools[0]", std::mem::offset_of!(FrameBlock, pools)),
];

/// Fails when the driver's layout of the frame block differs from [`FrameBlock`]; returns
/// the driver's block size (may be padded beyond [`FrameBlock`]).
fn check_frame_block(gl: &Gl, program: &WebGlProgram, block: u32) -> Result<usize, String> {
    let size = gl
        .get_active_uniform_block_parameter(program, block, Gl::UNIFORM_BLOCK_DATA_SIZE)
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as usize;
    if size < std::mem::size_of::<FrameBlock>() {
        return Err(format!(
            "frame block size {size} < {}",
            std::mem::size_of::<FrameBlock>()
        ));
    }
    let names = js_sys::Array::new();
    for (n, _) in FRAME_OFFSETS {
        names.push(&JsValue::from_str(n));
    }
    let Some(indices) = gl.get_uniform_indices(program, &names) else {
        return Err("frame block: getUniformIndices".into());
    };
    let offsets =
        js_sys::Array::from(&gl.get_active_uniforms(program, &indices, Gl::UNIFORM_OFFSET));
    for (k, (n, want)) in FRAME_OFFSETS.iter().enumerate() {
        let got = offsets.get(k as u32).as_f64().unwrap_or(-1.0);
        if got != *want as f64 {
            return Err(format!("frame block: {n} at {got}, expected {want}"));
        }
    }
    Ok(size)
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
    /// Extra room (m) around the instance bounds that the mesh can reach (rotated corners,
    /// parts below the origin, moving parts, bobbing): per-draw light masks (PERF-R-001).
    margin: f32,
    /// Lowest point below the origin (m, ≤ 0 for most meshes) for the culling bounds.
    lo_y: f32,
    /// Largest horizontal distance of a vertex from the origin (m): culling at any yaw.
    radius_xz: f32,
    /// Culling at rest and in motion (PERF-R-015, [`static_reach`]): the square half-size for
    /// quarter turns and the top, both including the swept space of turning parts, and the
    /// extra room of bobbing props.
    cull_radius: f32,
    cull_height: f32,
    bob_margin: f32,
    /// A water tile (drawn by the water program, TECH-WATER).
    water: bool,
    /// Bobbing on the water (`u_bob`, TECH-WATER behaviour 8).
    bob: [f32; 4],
    /// First index of the glass triangles (= `index_count` without glass).
    glass_first: i32,
    /// Part behaviours (`u_nodes`) if any part moves or hides.
    nodes: Option<[f32; MAX_NODE_PARTS * 4]>,
    /// Has material slots or parts: 16-float vertices and the rich vertex shader; else the
    /// lean 8-float format (ARCH-007).
    rich: bool,
}

/// A render region (chunk): a level part, a barrier or a building roof. Its batches are
/// skipped when it is hidden or its bounds are outside the view frustum (QA F12).
#[derive(Debug, Clone, Copy)]
struct Region {
    min: Vec3,
    max: Vec3,
    /// How far (m) its meshes can reach beyond `min`..`max` (yawed corners, stretched
    /// instances, parts below the origin): the culling box is grown by it, so nothing pops
    /// at the frustum edge (PERF-R-015).
    margin: f32,
    hidden: bool,
}

impl Default for Region {
    fn default() -> Self {
        Self {
            min: Vec3::splat(f32::MAX),
            max: Vec3::splat(f32::MIN),
            margin: 0.0,
            hidden: false,
        }
    }
}

/// Region 0: never culled (dynamic batches, characters' helpers).
pub const REGION_ALWAYS: u16 = 0;

/// One mesh drawn with instancing (per region).
struct Batch {
    /// Mesh name (debug: which batches draw).
    name: String,
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
    /// Mesh extent from the origin (m) and the light-mask margin of the largest instance
    /// (PERF-R-001).
    radius: f32,
    mesh_margin: f32,
    light_margin: f32,
    /// Extra culling room (m) around the chunk boxes: moving / bobbing parts and stretched
    /// or re-yawed instances (PERF-R-015; the boxes already hold every instance at rest).
    cull_margin: f32,
    /// Re-uploaded every frame with `buffer_sub_data` (characters, placeholders that move).
    dynamic: bool,
    water: bool,
    bob: [f32; 4],
    glass_first: i32,
    nodes: Option<[f32; MAX_NODE_PARTS * 4]>,
    rich: bool,
    /// Instances with a non-zero scale (a batch of hidden instances is not drawn: garden
    /// plants of the other growth stages).
    live: usize,
    /// Bounds of the instances per [`CHUNK_M`] ground chunk (static regions only): a batch is
    /// drawn only when one of its chunks is in view, so the short far plane of the close
    /// views culls every mesh that has no instance nearby (GAME-CAMERA-VIEWS 6).
    chunks: Vec<(IVec2, Vec3, Vec3)>,
    /// Per instance: its index in `chunks` (static batches; PERF-R-002).
    inst_chunk: Vec<u16>,
    /// Chunk-sorted GPU order (PERF-R-002, budget 22): the instances of a static batch with
    /// several chunks are uploaded sorted by chunk (row-major), one [`ChunkRange`] per chunk,
    /// so only the chunk ranges in view are drawn. Rebuilt when instances are added
    /// (`sorted = false`); instance handles keep indexing `instances`.
    ranges: Vec<ChunkRange>,
    sorted: bool,
    /// GPU slot → index in `instances`, and the upload scratch in that order.
    order: Vec<u32>,
    gpu: Vec<Instance>,
    /// The mesh buffers (for the VAO of each chunk range).
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
}

impl Batch {
    /// Whether an instance of the batch in one of its chunks can be in view: the chunk
    /// boxes (every instance at rest, yawed corners and parts below the origin included,
    /// [`instance_extent`]) grown by the batch's culling margin (moving parts, stretched
    /// instances) — conservative, so nothing pops at the frustum edge (PERF-R-015).
    fn any_chunk_visible(&self, cull: &Cull) -> bool {
        let m = Vec3::splat(self.cull_margin);
        self.chunks.iter().any(|c| cull.visible(c.1 - m, c.2 + m))
    }

    /// Whether chunk range `i` can be in view (grown like [`Batch::any_chunk_visible`]).
    fn range_visible(&self, cull: &Cull, i: usize) -> bool {
        let m = Vec3::splat(self.cull_margin);
        let r = &self.ranges[i];
        cull.visible(r.lo - m, r.hi + m)
    }

    /// Whether the batch is drawn per chunk range (PERF-R-002): a never-moving batch of a
    /// culled region with instances in more than one chunk.
    fn chunked(&self) -> bool {
        self.region != REGION_ALWAYS
            && !self.dynamic
            && self.chunks.len() > 1
            && self.inst_chunk.len() == self.instances.len()
    }
}

/// Instances of one ground chunk in a chunk-sorted batch (PERF-R-002): GPU slots
/// `first .. first + count`, their bounds, and a VAO whose instance attributes start at
/// `first` (WebGL2 has no base instance; `None` = the batch's own VAO, `first = 0`).
struct ChunkRange {
    lo: Vec3,
    hi: Vec3,
    first: u32,
    count: u32,
    vao: Option<WebGlVertexArrayObject>,
}

/// Most instanced draws of one chunk-sorted batch per frame (PERF-R-002): more visible runs
/// are merged into the last one.
pub const MAX_CHUNK_RUNS: usize = 3;
/// Off-screen chunks between two visible runs are drawn along (one draw instead of two)
/// when their triangles are at most this many — about the cost of one more draw call.
pub const GAP_MERGE_TRIANGLES: u64 = 16_384;

/// Plans the draws of a chunk-sorted batch (PERF-R-002, pure): `n` chunk ranges in GPU
/// order, `visible(i)` / `count(i)` per range, `tris` triangles per instance. Writes the
/// inclusive range spans `(first, last)` to `out` and returns how many: consecutive visible
/// ranges form one run; a gap of off-screen ranges is drawn along when it costs at most
/// [`GAP_MERGE_TRIANGLES`]; at most [`MAX_CHUNK_RUNS`] runs (the last one absorbs the rest).
pub fn plan_chunk_runs(
    n: usize,
    visible: impl Fn(usize) -> bool,
    count: impl Fn(usize) -> u32,
    tris: u32,
    out: &mut [(usize, usize); MAX_CHUNK_RUNS],
) -> usize {
    let mut k = 0;
    let mut cur: Option<(usize, usize)> = None;
    let mut gap = 0u64;
    for i in 0..n {
        if visible(i) {
            cur = Some(match cur {
                None => (i, i),
                Some((s, e)) => {
                    if gap * u64::from(tris) <= GAP_MERGE_TRIANGLES || k + 1 >= MAX_CHUNK_RUNS {
                        (s, i)
                    } else {
                        out[k] = (s, e);
                        k += 1;
                        (i, i)
                    }
                }
            });
            gap = 0;
        } else if cur.is_some() {
            gap += u64::from(count(i));
        }
    }
    if let Some(c) = cur {
        out[k] = c;
        k += 1;
    }
    k
}

/// GPU order of a chunked batch (PERF-R-002, pure): instance indices sorted by their chunk
/// key row-major (rows from south = +z to north, then x ascending; stable, so instances of a
/// chunk keep their order), and per chunk in that order `(chunk index, first slot, count)`.
pub fn chunk_order(keys: &[IVec2], inst_chunk: &[u16]) -> (Vec<u32>, Vec<(usize, u32, u32)>) {
    let mut chunks: Vec<usize> = (0..keys.len()).collect();
    // south (+z) rows first: front to back for the default camera, which looks north (−z),
    // so early depth test rejects the hidden fragments as with the level's placement order
    chunks.sort_by_key(|&c| (-keys[c].y, keys[c].x));
    let mut rank = vec![0usize; keys.len()];
    for (r, &c) in chunks.iter().enumerate() {
        rank[c] = r;
    }
    let mut order: Vec<u32> = (0..inst_chunk.len() as u32).collect();
    order.sort_by_key(|&i| rank[inst_chunk[i as usize] as usize]);
    let mut spans = Vec::with_capacity(chunks.len());
    let mut first = 0u32;
    for &c in &chunks {
        let count = inst_chunk.iter().filter(|&&k| k as usize == c).count() as u32;
        if count > 0 {
            spans.push((c, first, count));
            first += count;
        }
    }
    (order, spans)
}

/// VAO of a static mesh with its instance buffer, the instance attributes starting at
/// instance `first` (lean or rich vertex format, ARCH-007).
fn static_vao(
    gl: &Gl,
    vbo: &WebGlBuffer,
    ibo: &WebGlBuffer,
    rich: bool,
    inst_vbo: &WebGlBuffer,
    first: u32,
) -> Option<WebGlVertexArrayObject> {
    let vao = gl.create_vertex_array()?;
    gl.bind_vertex_array(Some(&vao));
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(vbo));
    let stride = if rich {
        (STATIC_VERTEX_FLOATS * 4) as i32
    } else {
        32
    };
    attrib(gl, 0, 3, stride, 0);
    attrib(gl, 1, 3, stride, 12);
    attrib(gl, 2, 2, stride, 24);
    if rich {
        attrib(gl, 6, 4, stride, 32);
        attrib(gl, 7, 4, stride, 48);
    }
    gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(ibo));
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(inst_vbo));
    let inst_stride = std::mem::size_of::<Instance>() as i32;
    let base = first as i32 * inst_stride;
    let locs: &[u32] = if rich { &[3, 4, 5, 8] } else { &[3, 4, 5] };
    for (k, &loc) in locs.iter().enumerate() {
        attrib(gl, loc, 4, inst_stride, base + k as i32 * 16);
        gl.vertex_attrib_divisor(loc, 1);
    }
    gl.bind_vertex_array(None);
    Some(vao)
}

/// Ground chunk size for per-batch culling (m).
pub const CHUNK_M: f32 = 8.0;

/// Light-mask margin (m) of a mesh around its instance bounds `pos ± radius` (PERF-R-001):
/// the corners of a yawed mesh reach `radius × √2`, parts below the origin `-lo_y`, and
/// 1 m covers moving parts and bobbing.
fn light_margin(radius: f32, lo_y: f32) -> f32 {
    radius * (std::f32::consts::SQRT_2 - 1.0) + (-lo_y).max(0.0) + 1.0
}

/// Swept reach of a static mesh (pure; PERF-R-015): `(radius_xz, rotating_xz, top)` — the
/// largest horizontal distance from the origin any vertex can reach (a vertex of a turning
/// pivot), the same for the turning parts alone (0 without), the highest and the lowest
/// point.
pub fn static_reach(p: &StaticVertices) -> (f32, f32, f32, f32) {
    let (mut rxz, mut rot, mut top, mut bottom) = (0.01f32, 0.0f32, f32::MIN, f32::MAX);
    for v in p.vertices.chunks(STATIC_VERTEX_FLOATS) {
        let pos = Vec3::new(v[0], v[1], v[2]);
        let code = v[15] as usize;
        let turns = code > 0 && code < MAX_NODE_PARTS && p.nodes[code * 4] > 0.0;
        if turns {
            // the vertex circles the part's axis through its pivot (1 = X, 2 = Y, 3 = Z)
            let pivot = Vec3::new(v[12], v[13], v[14]);
            let d = pos - pivot;
            let (r, lo_y, hi_y) = match p.nodes[code * 4] as i32 {
                1 => {
                    let c = d.y.hypot(d.z);
                    (pos.x.hypot(pivot.z.abs() + c), pivot.y - c, pivot.y + c)
                }
                2 => (pivot.x.hypot(pivot.z) + d.x.hypot(d.z), pos.y, pos.y),
                _ => {
                    let c = d.x.hypot(d.y);
                    ((pivot.x.abs() + c).hypot(pos.z), pivot.y - c, pivot.y + c)
                }
            };
            rxz = rxz.max(r);
            rot = rot.max(r);
            top = top.max(hi_y);
            bottom = bottom.min(lo_y);
        } else {
            rxz = rxz.max(pos.x.hypot(pos.z));
            top = top.max(pos.y);
            bottom = bottom.min(pos.y);
        }
    }
    (rxz, rot, top, bottom)
}

/// Extra culling room (m) of a bobbing prop (`u_bob` = amplitude, tilt, drift; TECH-WATER).
fn bob_margin(bob: [f32; 4], radius: f32) -> f32 {
    if bob.iter().all(|&v| v == 0.0) {
        return 0.0;
    }
    bob[0].abs() + bob[2].abs() + bob[1].abs() * radius + 0.05
}

/// Whether a yaw is a whole number of quarter turns (a mesh then stays inside its
/// `±radius` square).
fn quarter_turn(yaw: f32) -> bool {
    let q = yaw / std::f32::consts::FRAC_PI_2;
    (q - q.round()).abs() < 1e-4
}

/// Culling box of one instance around its origin (pure; PERF-R-015): `radius` = the mesh's
/// largest |x| / |z| extent, `radius_xz` its largest horizontal distance from the origin
/// (≤ `radius × √2`), `lo_y` ≤ 0 its lowest point, `height` its top (unscaled). A yaw that
/// is not a quarter turn can bring any vertex round to `radius_xz`; the box keeps 0.1 m
/// below and above for flat meshes.
pub fn instance_extent(
    radius: f32,
    radius_xz: f32,
    lo_y: f32,
    height: f32,
    yaw: f32,
    scale: f32,
) -> (Vec3, Vec3) {
    let r = scale
        * if quarter_turn(yaw) {
            radius
        } else {
            radius_xz.min(radius * std::f32::consts::SQRT_2)
        };
    (
        Vec3::new(-r, lo_y.min(0.0) * scale - 0.1, -r),
        Vec3::new(r, (height * scale).max(0.1), r),
    )
}

/// Extra room (m) around a skinned character's culling box for its light mask.
const CHARACTER_LIGHT_MARGIN_M: f32 = 0.5;

/// One material range of a skinned mesh.
struct Part {
    first_index: i32,
    index_count: i32,
    texture: WebGlTexture,
    textured: bool,
    color: [f32; 4],
    /// `eye_glow` slot: its emission colour (sRGB).
    emit: Option<[f32; 3]>,
}

/// `u_emit.w` of an eye_glow part that shines (`#E6F7A0 × (0.25 + 0.75 × luminance)`).
const EMIT_EYE: f32 = 4.0;

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
    /// The `eye_glow` slot shines (night animal inside the lantern radius, NIGHT-006).
    pub eye_glow: bool,
    /// Uniform model scale (1 = as modelled): pair members without their own model (Q-308).
    pub scale: f32,
    /// Colour tint `[r, g, b, amount]` over the model colours (amount 0 = none); an under-water
    /// character uses the water tint instead.
    pub tint: [f32; 4],
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
            eye_glow: false,
            scale: 1.0,
            tint: [0.0; 4],
        }
    }

    fn model(&self) -> Mat4 {
        Mat4::from_translation(self.pos)
            * Mat4::from_quat(self.tilt)
            * Mat4::from_rotation_y(self.yaw)
            * Mat4::from_scale(Vec3::splat(self.scale))
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
    /// Hidden with this render region (a name board hidden with its roof).
    region: u16,
    /// Atlas decal (food box labels): consecutive visible neighbours with the same texture
    /// and normal merge into one draw call.
    batch: bool,
}

/// Generous bounds of a skinned character around its origin (giraffe 4.5 m, elephant).
const CHARACTER_HALF_M: f32 = 2.5;
const CHARACTER_HEIGHT_M: f32 = 5.5;

fn character_visible(cull: &Cull, pos: Vec3) -> bool {
    let h = Vec3::new(CHARACTER_HALF_M, 0.0, CHARACTER_HALF_M);
    cull.visible(
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
    /// Static models with material slots / parts (ARCH-007).
    rich_prog: Program,
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
    /// Pixel ratio cap of the quality tier (PERF-BUDGETS rule 5): [`MAX_PIXEL_RATIO`] or the
    /// low tier's 1.5; the last CSS size and device pixel ratio (re-applied on a change).
    max_pixel_ratio: f64,
    css: (f64, f64, f64),
    /// Clouds in the close-view sky (off in the low tier, PERF-BUDGETS rule 5).
    pub clouds: bool,
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
    light_count: i32,
    pool_count: i32,
    /// Shared per-frame uniforms (PERF-R-003): the CPU copy and its uniform buffer.
    frame: FrameBlock,
    frame_ubo: WebGlBuffer,
    /// Debug (e2e PERF-017): every draw gets every light (no per-draw light masks).
    pub full_light_masks: bool,
    /// Haze culling of the close views (PERF-R-018, Q-193 answered yes 2026-09-28): on.
    pub haze_cull: bool,
    /// Debug (e2e PERF-025): no frustum culling (regions, chunks, chunk ranges, characters,
    /// decals) — the picture must be the same as with culling. The haze culling of the close
    /// views stays on: it is part of the approved look (Q-193).
    pub no_culling: bool,
    /// Glow slots emissive (night).
    glow_on: bool,
    /// Debug: names of the static batches drawn in the last frame (only while recording).
    pub debug_draws: Option<Vec<String>>,
    /// CPU copies of the static meshes (static batching, [`Renderer::bake`]).
    cpu_meshes: HashMap<String, StaticVertices>,
}

/// A model instance in a batch ([`Renderer::add_instance_handle`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceHandle {
    batch: usize,
    index: usize,
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
        let static_prog =
            Program::new(&gl, &shaders::static_vs(), &shaders::static_fs()).map_err(err)?;
        let rich_prog =
            Program::new(&gl, &shaders::rich_vs(), &shaders::static_fs()).map_err(err)?;
        let water_prog =
            Program::new(&gl, &shaders::water_vs(), &shaders::water_fs()).map_err(err)?;
        let skinned_prog =
            Program::new(&gl, &shaders::skinned_vs(), &shaders::static_fs()).map_err(err)?;
        let crowd_prog =
            Program::new(&gl, &shaders::crowd_vs(), &shaders::static_fs()).map_err(err)?;
        let post_prog = Program::new(&gl, &shaders::post_vs(), &shaders::post_fs()).map_err(err)?;
        let decal_prog =
            Program::new(&gl, &shaders::decal_vs(), &shaders::decal_fs()).map_err(err)?;

        // the shared frame block (PERF-R-003): one buffer on binding point 0 for all programs
        let frame_size = [
            &static_prog,
            &rich_prog,
            &water_prog,
            &skinned_prog,
            &crowd_prog,
            &decal_prog,
        ]
        .iter()
        .map(|p| p.frame_size)
        .max()
        .unwrap_or(0)
        .max(std::mem::size_of::<FrameBlock>());
        let frame_ubo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::UNIFORM_BUFFER, Some(&frame_ubo));
        gl.buffer_data_with_i32(Gl::UNIFORM_BUFFER, frame_size as i32, Gl::DYNAMIC_DRAW);
        gl.bind_buffer_base(
            Gl::UNIFORM_BUFFER,
            shaders::FRAME_BLOCK_BINDING,
            Some(&frame_ubo),
        );

        // uniforms that never change: set once (they live in the program objects)
        for p in [&static_prog, &rich_prog, &water_prog] {
            gl.use_program(Some(&p.program));
            p.set1i(&gl, U::Palette, 0);
            p.set4f(&gl, U::Tint, [0.0; 4]);
        }
        gl.use_program(Some(&water_prog.program));
        water_prog.set1i(&gl, U::Field, 2);
        water_prog.set4f(&gl, U::Bob, [0.0; 4]);
        for p in [&skinned_prog, &crowd_prog] {
            gl.use_program(Some(&p.program));
            p.set1i(&gl, U::Palette, 0);
            p.set1i(&gl, U::JointTex, 1);
            p.set1f(&gl, U::EdgeMask, 1.0);
            p.set4f(&gl, U::Tint, [0.0; 4]);
            p.set1f(&gl, U::DepthBias, 0.0);
            p.set4f(&gl, U::Emit, [0.0; 4]);
        }
        gl.use_program(Some(&crowd_prog.program));
        crowd_prog.set_mat4(&gl, U::Model, &Mat4::IDENTITY);
        gl.use_program(Some(&decal_prog.program));
        decal_prog.set1i(&gl, U::Tex, 0);
        gl.use_program(Some(&post_prog.program));
        post_prog.set1i(&gl, U::Color, 0);
        post_prog.set1i(&gl, U::Normal, 1);
        post_prog.set1i(&gl, U::Depth, 2);
        post_prog.set3f(&gl, U::LineColor, OUTLINE);

        let palette = create_texture(&gl, 1, 1, &[255, 255, 255, 255])?;
        let mut r = Self {
            gl,
            canvas,
            static_prog,
            rich_prog,
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
            max_pixel_ratio: MAX_PIXEL_RATIO,
            css: (0.0, 0.0, 1.0),
            clouds: true,
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
            light_count: 0,
            pool_count: 0,
            frame: FrameBlock {
                sun_dir: SUN_DIR.normalize().to_array(),
                shadow_tint: SHADOW_TINT.to_array(),
                ..FrameBlock::zeroed()
            },
            frame_ubo,
            full_light_masks: false,
            no_culling: false,
            haze_cull: true,
            glow_on: false,
            debug_draws: None,
            cpu_meshes: HashMap::new(),
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
        self.css = (css_w, css_h, dpr);
        let ratio = dpr.clamp(1.0, self.max_pixel_ratio.max(1.0));
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

    /// Caps the pixel ratio (quality tier, PERF-BUDGETS rule 5) and resizes the drawing
    /// buffer with the last CSS size.
    pub fn set_max_pixel_ratio(&mut self, max: f64) {
        if max != self.max_pixel_ratio {
            self.max_pixel_ratio = max;
            let (w, h, dpr) = self.css;
            if w > 0.0 && h > 0.0 {
                self.resize(w, h, dpr);
            }
        }
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
        self.add_decal_in(texture, corners, normal, REGION_ALWAYS);
    }

    /// Adds a decal that is hidden with a render region.
    pub fn add_decal_in(&mut self, texture: &str, corners: [Vec3; 4], normal: Vec3, region: u16) {
        self.add_decal_uv(
            texture,
            corners,
            normal,
            region,
            [0.0, 0.0, 1.0, 1.0],
            false,
        );
    }

    /// Adds a decal showing the `uv` rectangle `[u0, v0, u1, v1]` of a (shared atlas) texture.
    /// With `batch`, consecutive decals of the same texture and normal share one draw call.
    pub fn add_decal_uv(
        &mut self,
        texture: &str,
        corners: [Vec3; 4],
        normal: Vec3,
        region: u16,
        uv: [f32; 4],
        batch: bool,
    ) {
        let first_vertex = self.decals.vertices.len() / 5;
        for (c, uv) in corners.iter().zip([
            [uv[0], uv[1]],
            [uv[2], uv[1]],
            [uv[2], uv[3]],
            [uv[0], uv[3]],
        ]) {
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
            region,
            batch,
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
        let inst_vbo = gl.create_buffer()?;
        let vao = static_vao(gl, &m.vbo, &m.ibo, m.rich, &inst_vbo, 0)?;
        let batch = Batch {
            name: name.to_owned(),
            region,
            vao,
            inst_vbo,
            index_count: m.index_count,
            instances: Vec::new(),
            uploaded: usize::MAX,
            capacity: 0,
            edge_mask: m.edge_mask,
            height: m.height,
            radius: m.radius,
            mesh_margin: m.margin,
            light_margin: m.margin,
            cull_margin: m.bob_margin,
            dynamic: false,
            water: m.water,
            bob: m.bob,
            glass_first: m.glass_first,
            nodes: m.nodes,
            rich: m.rich,
            live: 0,
            chunks: Vec::new(),
            inst_chunk: Vec::new(),
            ranges: Vec::new(),
            sorted: false,
            order: Vec::new(),
            gpu: Vec::new(),
            vbo: m.vbo.clone(),
            ibo: m.ibo.clone(),
        };
        self.batch_index
            .insert((name.to_owned(), region), self.batches.len());
        self.batches.push(batch);
        Some(self.batches.len() - 1)
    }

    fn grow_region(&mut self, region: u16, pos: Vec3, (lo, hi): (Vec3, Vec3)) {
        if region == REGION_ALWAYS {
            return;
        }
        if let Some(r) = self.regions.get_mut(region as usize) {
            r.min = r.min.min(pos + lo);
            r.max = r.max.max(pos + hi);
        }
    }

    /// Grows the bounds of a static batch (per-batch culling, GAME-CAMERA-VIEWS 6).
    fn grow_batch(&mut self, i: usize, pos: Vec3, (lo, hi): (Vec3, Vec3)) {
        let b = &mut self.batches[i];
        let key = IVec2::new(
            (pos.x / CHUNK_M).floor() as i32,
            (pos.z / CHUNK_M).floor() as i32,
        );
        let (lo, hi) = (pos + lo, pos + hi);
        let k = match b.chunks.iter().position(|c| c.0 == key) {
            Some(k) => {
                let c = &mut b.chunks[k];
                c.1 = c.1.min(lo);
                c.2 = c.2.max(hi);
                k
            }
            None => {
                b.chunks.push((key, lo, hi));
                b.chunks.len() - 1
            }
        };
        // the instance just pushed (PERF-R-002: chunk-sorted upload)
        if b.inst_chunk.len() + 1 == b.instances.len() {
            b.inst_chunk.push(k as u16);
        }
        b.sorted = false;
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
        self.add_mesh_parts(name, mesh, &[], &[], edge_mask)
    }

    /// Adds a static mesh with its material slots (glow, glass) and movable parts.
    fn add_mesh_parts(
        &mut self,
        name: &str,
        mesh: &MeshData,
        materials: &[zoo_assets::Material],
        parts: &[zoo_assets::NodePart],
        edge_mask: f32,
    ) -> Result<(), JsValue> {
        let packed = static_vertices(mesh, materials, parts);
        self.upload_static(name, packed, mesh.bounds(), edge_mask)
    }

    /// Uploads packed static vertices as mesh `name` (lean or rich format) with its batch in
    /// [`REGION_ALWAYS`]; keeps a CPU copy for [`Renderer::bake`].
    fn upload_static(
        &mut self,
        name: &str,
        packed: StaticVertices,
        (lo, hi): (Vec3, Vec3),
        edge_mask: f32,
    ) -> Result<(), JsValue> {
        let gl = &self.gl;
        let rich = packed.is_rich();
        let lean: Vec<f32>;
        let vertices: &[f32] = if rich {
            &packed.vertices
        } else {
            // the lean format: position, normal, uv (ARCH-007)
            lean = packed
                .vertices
                .chunks(STATIC_VERTEX_FLOATS)
                .flat_map(|v| v[..8].iter().copied())
                .collect();
            &lean
        };
        let vbo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_u8_array(
            Gl::ARRAY_BUFFER,
            bytemuck::cast_slice(vertices),
            Gl::STATIC_DRAW,
        );
        let ibo = gl.create_buffer().ok_or("create_buffer")?;
        gl.bind_vertex_array(None);
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        gl.buffer_data_with_u8_array(
            Gl::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&packed.indices),
            Gl::STATIC_DRAW,
        );
        let reach = static_reach(&packed);
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
                index_count: packed.indices.len() as i32,
                edge_mask,
                height: hi.y,
                radius,
                margin: light_margin(radius, lo.y),
                lo_y: lo.y.min(reach.3).min(0.0),
                radius_xz: reach.0,
                cull_radius: radius.max(reach.1),
                cull_height: hi.y.max(reach.2),
                bob_margin: bob_margin(bob_params(name).uniform(), radius),
                water: TileShape::of_model(name).is_some(),
                bob: bob_params(name).uniform(),
                glass_first: packed.glass_first as i32,
                nodes: packed.has_nodes.then_some(packed.nodes),
                rich,
            },
        );
        self.cpu_meshes.insert(name.to_owned(), packed);
        self.batch_for(name, REGION_ALWAYS)
            .ok_or_else(|| JsValue::from_str("create batch"))?;
        Ok(())
    }

    /// Whether a loaded static model can be merged by [`Renderer::bake`].
    pub fn can_bake(&self, name: &str) -> bool {
        self.cpu_meshes.get(name).is_some_and(|p| p.is_bakeable())
    }

    /// Static batching (TECH-ARCH "Multi-node assets", ARCH-008): merges model instances
    /// that never move, glow-toggle or hide into one new mesh `name` with one instance in
    /// `region` — one draw call for a whole group (a garden, a room's furniture). Returns
    /// `false` (nothing added) when a model is unknown or cannot be baked; the caller then
    /// places the instances one by one.
    pub fn bake(
        &mut self,
        name: &str,
        region: u16,
        items: &[(&str, Vec3, f32, Vec3)],
    ) -> Result<bool, JsValue> {
        if items.is_empty() {
            return Ok(false);
        }
        let mut sources = Vec::with_capacity(items.len());
        for (m, pos, yaw, scale) in items {
            match self.cpu_meshes.get(*m) {
                Some(p) if p.is_bakeable() => sources.push((p, *pos, *yaw, *scale)),
                _ => return Ok(false),
            }
        }
        let origin = items.iter().map(|i| i.1).sum::<Vec3>() / items.len() as f32;
        let origin = Vec3::new(origin.x, 0.0, origin.z);
        let packed = bake_vertices(&sources, origin);
        let n = packed.vertices.len() / STATIC_VERTEX_FLOATS;
        let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        for k in 0..n {
            let o = k * STATIC_VERTEX_FLOATS;
            let p = Vec3::new(
                packed.vertices[o],
                packed.vertices[o + 1],
                packed.vertices[o + 2],
            );
            lo = lo.min(p);
            hi = hi.max(p);
        }
        self.upload_static(name, packed, (lo, hi), 1.0)?;
        Ok(self.add_instance_in(name, region, origin, 0.0, 1.0))
    }

    /// Adds a static model loaded from a `.glb` (glow slots, glass, movable parts).
    pub fn add_model(&mut self, name: &str, model: &Model, edge_mask: f32) -> Result<(), JsValue> {
        self.add_mesh_parts(name, &model.mesh, &model.materials, &model.nodes, edge_mask)
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
        let Some((radius, radius_xz, height, lo_y)) = self
            .meshes
            .get(name)
            .map(|m| (m.cull_radius, m.radius_xz, m.cull_height, m.lo_y))
        else {
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
        b.light_margin = b.light_margin.max(b.mesh_margin * scale);
        let margin = b.cull_margin;
        if let Some(r) = self.regions.get_mut(region as usize) {
            r.margin = r.margin.max(margin);
        }
        let ext = instance_extent(radius, radius_xz, lo_y, height, yaw, scale);
        self.grow_region(region, pos, ext);
        self.grow_batch(i, pos, ext);
        true
    }

    /// Adds a model instance to a render region and returns its handle (to move its parts
    /// later with [`Renderer::set_instance`]); `None` for an unknown model.
    pub fn add_instance_handle(
        &mut self,
        name: &str,
        region: u16,
        pos: Vec3,
        yaw: f32,
        scale: f32,
    ) -> Option<InstanceHandle> {
        if !self.add_instance_in(name, region, pos, yaw, scale) {
            return None;
        }
        let batch = *self.batch_index.get(&(name.to_owned(), region))?;
        Some(InstanceHandle {
            batch,
            index: self.batches[batch].instances.len() - 1,
        })
    }

    /// The instance data behind a handle.
    pub fn instance(&self, h: InstanceHandle) -> Option<Instance> {
        self.batches.get(h.batch)?.instances.get(h.index).copied()
    }

    /// Changes one instance (yaw, open amount, hide mask); re-uploads its batch once.
    pub fn set_instance(&mut self, h: InstanceHandle, inst: Instance) {
        if let Some(b) = self.batches.get_mut(h.batch) {
            if let Some(i) = b.instances.get_mut(h.index) {
                if bytemuck::bytes_of(i) != bytemuck::bytes_of(&inst) {
                    let old_yaw = i.pos_yaw[3];
                    *i = inst;
                    b.uploaded = usize::MAX;
                    // a stretched instance (string lights) reaches beyond its chunk bounds
                    let s = inst.scale_fade[0].abs().max(inst.scale_fade[2].abs());
                    if s > 1.0 {
                        b.light_margin = b.light_margin.max(b.mesh_margin * s + b.radius * s);
                        b.cull_margin = b.cull_margin.max(b.radius * s * std::f32::consts::SQRT_2);
                    }
                    // turned to an angle its chunk box was not grown for (PERF-R-015)
                    if inst.pos_yaw[3] != old_yaw && !quarter_turn(inst.pos_yaw[3]) {
                        b.cull_margin = b
                            .cull_margin
                            .max(b.radius * s.max(1.0) * (std::f32::consts::SQRT_2 - 1.0));
                    }
                    let (region, margin) = (b.region, b.cull_margin);
                    if let Some(r) = self.regions.get_mut(region as usize) {
                        r.margin = r.margin.max(margin);
                    }
                }
            }
        }
    }

    /// Glow slots on (night: lamps lit, windows glowing, the moon window's night sky).
    pub fn set_glow(&mut self, on: bool) {
        self.glow_on = on;
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
        let ext = instance_extent(radius, radius, 0.0, size.y, 0.0, 1.0);
        self.grow_region(region, pos, ext);
        self.grow_batch(i, pos, ext);
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

        // One part per material range; PNG base colour textures (one upload per atlas image,
        // shared by `body` and `eye_glow`), else the colour factor.
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
            let key = mat
                .and_then(|m| m.image_index)
                .unwrap_or(usize::MAX - sub.material.unwrap_or(0));
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
                emit: mat
                    .filter(|m| m.is_eye_glow())
                    .map(|m| m.emissive_srgb())
                    .map(|e| {
                        if e.iter().all(|&c| c <= 0.0) {
                            night::EYE_GLOW.to_array()
                        } else {
                            e
                        }
                    }),
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
            self.frame.lights[k] = [l.pos.x, l.pos.y, l.pos.z, l.radius];
            self.frame.light_colors[k] = [
                l.color.x,
                l.color.y,
                l.color.z,
                l.strength + if l.tinted { 2.0 } else { 0.0 },
            ];
            self.light_count = k as i32 + 1;
        }
    }

    /// Light-pool decals of this frame (lamps beyond the point-light budget, Q-114): flat
    /// warm pools on the ground (`pos.xz`, radius, strength).
    pub fn set_light_pools(&mut self, pools: &[PointLight]) {
        self.pool_count = 0;
        for (k, l) in pools.iter().take(MAX_LIGHT_POOLS).enumerate() {
            self.frame.pools[k] = [l.pos.x, l.pos.z, l.radius, l.strength];
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
        let ext = instance_extent(radius, radius, 0.0, size.y, 0.0, 1.0);
        self.grow_region(region, pos, ext);
        self.grow_batch(i, pos, ext);
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
        let chunked = b.chunked();
        if chunked && !b.sorted {
            Self::sort_chunks(gl, b);
        }
        if !chunked && !b.ranges.is_empty() {
            Self::drop_ranges(gl, b);
        }
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&b.inst_vbo));
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
        let bytes: &[u8] = if chunked {
            // GPU order: sorted by chunk (PERF-R-002)
            b.gpu.clear();
            b.gpu
                .extend(b.order.iter().map(|&i| b.instances[i as usize]));
            bytemuck::cast_slice(&b.gpu)
        } else {
            bytemuck::cast_slice(&b.instances)
        };
        gl.buffer_sub_data_with_i32_and_u8_array(Gl::ARRAY_BUFFER, 0, bytes);
        b.uploaded = b.instances.len();
        b.live = b
            .instances
            .iter()
            .filter(|i| i.scale_fade[0] != 0.0)
            .count();
    }

    /// Rebuilds the chunk-sorted GPU order and the chunk ranges of a batch (PERF-R-002).
    fn sort_chunks(gl: &Gl, b: &mut Batch) {
        Self::drop_ranges(gl, b);
        let keys: Vec<IVec2> = b.chunks.iter().map(|c| c.0).collect();
        let (order, spans) = chunk_order(&keys, &b.inst_chunk);
        b.order = order;
        for (c, first, count) in spans {
            let vao = if first == 0 {
                None
            } else {
                static_vao(gl, &b.vbo, &b.ibo, b.rich, &b.inst_vbo, first)
            };
            b.ranges.push(ChunkRange {
                lo: b.chunks[c].1,
                hi: b.chunks[c].2,
                first,
                count,
                vao,
            });
        }
        b.sorted = true;
    }

    fn drop_ranges(gl: &Gl, b: &mut Batch) {
        for r in b.ranges.drain(..) {
            if let Some(v) = r.vao {
                gl.delete_vertex_array(Some(&v));
            }
        }
        b.order.clear();
        b.sorted = false;
    }

    /// Uploads the shared per-frame values (PERF-R-003): one `bufferSubData` per frame
    /// replaces ≈ 17 uniform calls per program switch.
    fn upload_frame_block(&mut self, view: &Mat4, view_proj: &Mat4, fade: Vec4) {
        let dither = self.outline_px() * 0.5;
        let glow_on = f32::from(u8::from(self.glow_on));
        let f = &mut self.frame;
        f.view = view.to_cols_array();
        f.view_proj = view_proj.to_cols_array();
        f.dither = dither;
        f.glow_on = glow_on;
        f.fade = fade.to_array();
        f.night = [self.light.night, self.light.warm, 0.0, 0.0];
        f.time = self.time;
        let gl = &self.gl;
        gl.bind_buffer(Gl::UNIFORM_BUFFER, Some(&self.frame_ubo));
        gl.buffer_sub_data_with_i32_and_u8_array(
            Gl::UNIFORM_BUFFER,
            0,
            bytemuck::bytes_of(&self.frame),
        );
    }

    /// The point lights / light pools that can reach a box (PERF-R-001, [`night::light_mask`]).
    fn box_light_mask(&self, min: Vec3, max: Vec3) -> (u32, u32) {
        let (nl, np) = (self.light_count as usize, self.pool_count as usize);
        if nl + np == 0 {
            return (0, 0);
        }
        if self.full_light_masks {
            return night::full_light_mask(nl, np);
        }
        night::light_mask(&self.frame.lights[..nl], &self.frame.pools[..np], min, max)
    }

    /// Light mask of a static batch: the union over its chunks (all lights for dynamic
    /// batches, whose instances move every frame).
    fn batch_light_mask(&self, b: &Batch) -> (u32, u32) {
        let (nl, np) = (self.light_count as usize, self.pool_count as usize);
        if nl + np == 0 {
            return (0, 0);
        }
        let full = night::full_light_mask(nl, np);
        if self.full_light_masks || b.dynamic || b.chunks.is_empty() {
            return full;
        }
        let m = Vec3::splat(b.light_margin);
        let mut acc = (0, 0);
        for c in &b.chunks {
            let k = self.box_light_mask(c.1 - m, c.2 + m);
            acc = (acc.0 | k.0, acc.1 | k.1);
            if acc == full {
                break;
            }
        }
        acc
    }

    /// Light mask of the chunk ranges `first..=last` of a chunk-sorted batch (PERF-R-002):
    /// the union over their chunks, like [`Renderer::batch_light_mask`].
    fn ranges_light_mask(&self, b: &Batch, first: usize, last: usize) -> (u32, u32) {
        let (nl, np) = (self.light_count as usize, self.pool_count as usize);
        if nl + np == 0 {
            return (0, 0);
        }
        let full = night::full_light_mask(nl, np);
        if self.full_light_masks {
            return full;
        }
        let m = Vec3::splat(b.light_margin);
        let mut acc = (0, 0);
        for r in &b.ranges[first..=last] {
            let k = self.box_light_mask(r.lo - m, r.hi + m);
            acc = (acc.0 | k.0, acc.1 | k.1);
            if acc == full {
                break;
            }
        }
        acc
    }

    /// Outline sample offset in device pixels (twice the screen-door cell, so the fade
    /// pattern stays line-free). Capped by the low tier (PERF-BUDGETS rule 5) the lines stay
    /// 2 CSS px: 3 device px at the pixel ratio 1.5.
    fn outline_px(&self) -> f32 {
        if self.max_pixel_ratio < MAX_PIXEL_RATIO && self.pixel_ratio >= self.max_pixel_ratio as f32
        {
            return 2.0 * self.pixel_ratio;
        }
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
        if self.gbuf.is_none() {
            return;
        }
        self.upload_frame_block(&view, &view_proj, fade);

        let gl = &self.gl;
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
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&self.palette));
        // frustum + haze (PERF-R-018, Q-193): in the full close views everything beyond the
        // fog end is exactly the sky colour (GAME-CAMERA-VIEWS 7), so boxes entirely beyond
        // it are skipped (a nearer silhouette's outline may lose 1–4 px inside the haze —
        // approved); never in the zoo view. `no_culling` (debug) keeps only the haze.
        let fog = camera.fog();
        let cull = Cull {
            planes: frustum_planes(&view_proj),
            all: self.no_culling,
            haze: (self.haze_cull && fog.amount >= 1.0 && camera.sky_amount() >= 1.0)
                .then(|| (camera.eye(), fog.end + HAZE_CULL_MARGIN_M)),
        };
        let ground = if self.no_culling {
            None
        } else {
            visible_ground(&view_proj)
        };
        let visible: Vec<bool> = self
            .regions
            .iter()
            .enumerate()
            .map(|(k, r)| {
                let (min, max) = (r.min - Vec3::splat(r.margin), r.max + Vec3::splat(r.margin));
                k == REGION_ALWAYS as usize
                    || (!r.hidden
                        && cull.visible(min, max)
                        && ground.is_none_or(|(lo, hi)| {
                            min.x <= hi.x && max.x >= lo.x && min.z <= hi.y && max.z >= lo.y
                        }))
            })
            .collect();
        let water_prog = self.water_animation && self.field_tex.is_some();
        let mut glass_batches = 0u32;
        // passes: 1 = models with slots / parts (rich shader), 0 = lean static meshes,
        // 2 = water tiles (water shader; the lean one without water animation)
        // (models first: big buildings hide the ground behind them before it is shaded)
        for pass in [1u8, 0, 2] {
            let water_pass = pass == 2;
            let prog = match pass {
                1 => &self.rich_prog,
                2 if water_prog => &self.water_prog,
                _ => &self.static_prog,
            };
            let gl = &self.gl;
            gl.use_program(Some(&prog.program));
            if water_pass && water_prog {
                gl.active_texture(Gl::TEXTURE2);
                gl.bind_texture(Gl::TEXTURE_2D, self.field_tex.as_ref());
                gl.active_texture(Gl::TEXTURE0);
                let xf = self.field_xf;
                prog.set4f(gl, U::FieldXf, xf);
                prog.set4fv(gl, U::Obstacles, &self.obstacle_u);
                prog.set4fv(gl, U::ObstaclesB, &self.obstacle_b);
                prog.set4fv(gl, U::Ripples, &self.ripple_u);
                prog.set4fv(gl, U::RipplesB, &self.ripple_b);
                prog.set1i(gl, U::RippleCount, self.ripple_count);
            }
            for b in &self.batches {
                if b.live == 0 || b.water != water_pass || (!water_pass && b.rich != (pass == 1)) {
                    continue;
                }
                if !visible.get(b.region as usize).copied().unwrap_or(true)
                    || (b.region != REGION_ALWAYS && !b.any_chunk_visible(&cull))
                {
                    stats.culled_batches += 1;
                    continue;
                }
                if b.glass_first == 0 {
                    continue; // glass only: drawn in the glass pass
                }
                prog.set1f(gl, U::EdgeMask, b.edge_mask);
                if !water_pass {
                    let bob = if self.water_animation {
                        b.bob
                    } else {
                        [0.0; 4]
                    };
                    prog.set4f(gl, U::Bob, bob);
                }
                if let Some(nodes) = &b.nodes {
                    prog.set4fv(gl, U::Nodes, nodes);
                }
                let tris = b.glass_first as u32 / 3;
                if !b.ranges.is_empty() {
                    // chunk-sorted batch (PERF-R-002): only the chunk ranges in view
                    let mut runs = [(0usize, 0usize); MAX_CHUNK_RUNS];
                    let n = plan_chunk_runs(
                        b.ranges.len(),
                        |i| b.range_visible(&cull, i),
                        |i| b.ranges[i].count,
                        tris,
                        &mut runs,
                    );
                    let mut drawn = 0u32;
                    for &(s, e) in &runs[..n] {
                        let (r0, r1) = (&b.ranges[s], &b.ranges[e]);
                        let count = r1.first + r1.count - r0.first;
                        prog.set2ui(gl, U::LightMask, self.ranges_light_mask(b, s, e));
                        gl.bind_vertex_array(Some(r0.vao.as_ref().unwrap_or(&b.vao)));
                        gl.draw_elements_instanced_with_i32(
                            Gl::TRIANGLES,
                            b.glass_first,
                            Gl::UNSIGNED_INT,
                            0,
                            count as i32,
                        );
                        stats.draw_calls += 1;
                        drawn += count;
                    }
                    stats.instances += drawn;
                    stats.triangles += tris * drawn;
                    if let Some(d) = &mut self.debug_draws {
                        d.push(format!(
                            "{}@{} x{} {}t ({} of {} in {} draws)",
                            b.name,
                            b.region,
                            drawn,
                            tris * drawn,
                            drawn,
                            b.instances.len(),
                            n
                        ));
                    }
                } else {
                    prog.set2ui(gl, U::LightMask, self.batch_light_mask(b));
                    if let Some(d) = &mut self.debug_draws {
                        d.push(format!(
                            "{}@{} x{} {}t",
                            b.name,
                            b.region,
                            b.instances.len(),
                            tris as usize * b.instances.len()
                        ));
                    }
                    gl.bind_vertex_array(Some(&b.vao));
                    gl.draw_elements_instanced_with_i32(
                        Gl::TRIANGLES,
                        b.glass_first,
                        Gl::UNSIGNED_INT,
                        0,
                        b.instances.len() as i32,
                    );
                    stats.draw_calls += 1;
                    stats.instances += b.instances.len() as u32;
                    stats.triangles += tris * b.instances.len() as u32;
                }
                if b.glass_first < b.index_count {
                    glass_batches += 1;
                }
            }
        }

        // Skinned characters.
        if !characters.is_empty() {
            self.gl.use_program(Some(&self.skinned_prog.program));
        }
        let eye = camera.eye();
        let char_half = Vec3::new(CHARACTER_HALF_M, 0.0, CHARACTER_HALF_M)
            + Vec3::splat(CHARACTER_LIGHT_MARGIN_M);
        for (name, draw) in characters {
            if !character_visible(&cull, draw.pos) {
                continue; // beyond the close views' far plane or off screen
            }
            let mask = self.box_light_mask(
                draw.pos - char_half - Vec3::Y * 0.5,
                draw.pos + char_half + Vec3::Y * CHARACTER_HEIGHT_M,
            );
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
            p.set3f(gl, U::Eye, eye);
            if draw.under_water {
                p.set4f(gl, U::Tint, UNDER_WATER_TINT);
                p.set1f(gl, U::DepthBias, UNDER_WATER_DEPTH_BIAS_M);
            } else {
                p.set4f(gl, U::Tint, draw.tint);
                p.set1f(gl, U::DepthBias, 0.0);
            }
            p.set2ui(gl, U::LightMask, mask);
            p.set_mat4(gl, U::Model, &draw.model());
            gl.bind_vertex_array(Some(&sm.vao));
            gl.active_texture(Gl::TEXTURE0);
            for part in &sm.parts {
                gl.bind_texture(Gl::TEXTURE_2D, Some(&part.texture));
                let c = if part.textured {
                    [1.0, 1.0, 1.0, 0.0]
                } else {
                    [part.color[0], part.color[1], part.color[2], 1.0]
                };
                p.set4f(gl, U::Color, c);
                // eye_glow (NIGHT-006): unlit eye colour × texel luminance inside the lantern
                let e = match part.emit {
                    Some(e) if draw.eye_glow => [e[0], e[1], e[2], EMIT_EYE],
                    _ => [0.0; 4],
                };
                p.set4f(gl, U::Emit, e);
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
            self.gl.use_program(Some(&self.crowd_prog.program));
        }
        for k in 0..self.crowd_names.len() {
            let name = self.crowd_names[k];
            let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
            let Some(sm) = self.skinned.get_mut(name) else {
                continue;
            };
            let n_joints = sm.joints.len();
            let mut n = 0usize;
            for (_, draw) in crowd
                .iter()
                .filter(|(m, d)| *m == name && character_visible(&cull, d.pos))
                .take(MAX_CROWD)
            {
                pose_character(sm, draw);
                let model = draw.model();
                for (j, m) in sm.joints.iter().enumerate() {
                    let o = (n * n_joints + j) * 16;
                    sm.crowd_data[o..o + 16].copy_from_slice(&(model * *m).to_cols_array());
                }
                lo = lo.min(draw.pos - char_half - Vec3::Y * 0.5);
                hi = hi.max(draw.pos + char_half + Vec3::Y * CHARACTER_HEIGHT_M);
                n += 1;
            }
            if n == 0 {
                continue;
            }
            let Some(sm) = self.skinned.get(name) else {
                continue;
            };
            let mask = self.box_light_mask(lo, hi);
            let gl = &self.gl;
            let p = &self.crowd_prog;
            p.set2ui(gl, U::LightMask, mask);
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
                p.set4f(gl, U::Color, c);
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

        // Glass panes (`glass` slot, alpha 0.35): after everything opaque, blended into the
        // colour, the depth and the edge mask untouched (ARCH-007).
        if glass_batches > 0 {
            let gl = &self.gl;
            let prog = &self.rich_prog;
            gl.use_program(Some(&prog.program));
            prog.set4f(gl, U::Bob, [0.0; 4]);
            gl.active_texture(Gl::TEXTURE0);
            gl.bind_texture(Gl::TEXTURE_2D, Some(&self.palette));
            gl.enable(Gl::BLEND);
            gl.blend_func_separate(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA, Gl::ZERO, Gl::ONE);
            gl.depth_mask(false);
            for b in &self.batches {
                if b.live == 0
                    || b.glass_first >= b.index_count
                    || !visible.get(b.region as usize).copied().unwrap_or(true)
                    || (b.region != REGION_ALWAYS && !b.any_chunk_visible(&cull))
                {
                    continue;
                }
                prog.set1f(gl, U::EdgeMask, b.edge_mask);
                if let Some(nodes) = &b.nodes {
                    prog.set4fv(gl, U::Nodes, nodes);
                }
                prog.set2ui(gl, U::LightMask, self.batch_light_mask(b));
                gl.bind_vertex_array(Some(&b.vao));
                gl.draw_elements_instanced_with_i32(
                    Gl::TRIANGLES,
                    b.index_count - b.glass_first,
                    Gl::UNSIGNED_INT,
                    b.glass_first * 4,
                    b.instances.len() as i32,
                );
                stats.draw_calls += 1;
                stats.triangles +=
                    ((b.index_count - b.glass_first) as u32 / 3) * b.instances.len() as u32;
            }
            gl.depth_mask(true);
            gl.disable(Gl::BLEND);
        }

        // Decals over their faces (sign silhouettes and texts, ART-ENVIRONMENT 6/7).
        if self.decals.uploaded {
            let gl = &self.gl;
            let p = &self.decal_prog;
            gl.use_program(Some(&p.program));
            gl.active_texture(Gl::TEXTURE0);
            gl.enable(Gl::BLEND);
            // colour and normal rgb blend by the decal alpha; the edge mask (dst alpha) stays
            gl.blend_func_separate(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA, Gl::ZERO, Gl::ONE);
            gl.depth_mask(false);
            gl.disable(Gl::CULL_FACE);
            gl.enable(Gl::POLYGON_OFFSET_FILL);
            gl.polygon_offset(-1.0, -4.0);
            gl.bind_vertex_array(self.decals.vao.as_ref());
            // draws of consecutive batched quads with the same texture and normal merge
            let mut run: Option<(usize, i32, &DecalDraw)> = None; // (first quad, quads, decal)
            let flush = |run: &mut Option<(usize, i32, &DecalDraw)>, stats: &mut FrameStats| {
                let Some((first, n, d)) = run.take() else {
                    return;
                };
                let Some(t) = self.decal_textures.get(&d.texture) else {
                    return;
                };
                gl.bind_texture(Gl::TEXTURE_2D, Some(t));
                p.set3f(gl, U::Normal, d.normal);
                gl.draw_elements_with_i32(
                    Gl::TRIANGLES,
                    6 * n,
                    Gl::UNSIGNED_SHORT,
                    (first * 6 * 2) as i32,
                );
                stats.draw_calls += 1;
                stats.triangles += 2 * n as u32;
                stats.decals += n as u32;
            };
            for d in &self.decals.draws {
                if !cull.visible(d.min, d.max)
                    || (d.region != REGION_ALWAYS
                        && self
                            .regions
                            .get(d.region as usize)
                            .is_some_and(|r| r.hidden))
                {
                    flush(&mut run, &mut stats);
                    continue;
                }
                let quad = d.first_vertex / 4;
                if let Some((first, n, prev)) = &mut run {
                    if d.batch
                        && prev.batch
                        && prev.texture == d.texture
                        && prev.normal == d.normal
                        && *first + *n as usize == quad
                    {
                        *n += 1;
                        continue;
                    }
                }
                flush(&mut run, &mut stats);
                run = Some((quad, 1, d));
            }
            flush(&mut run, &mut stats);
            gl.disable(Gl::POLYGON_OFFSET_FILL);
            gl.depth_mask(true);
            gl.disable(Gl::BLEND);
        }

        // Outline pass to the canvas.
        let gl = &self.gl;
        let Some(g) = &self.gbuf else { return };
        gl.bind_vertex_array(None);
        gl.bind_framebuffer(Gl::FRAMEBUFFER, None);
        gl.viewport(0, 0, w, h);
        gl.disable(Gl::DEPTH_TEST);
        gl.disable(Gl::CULL_FACE);
        let p = &self.post_prog;
        gl.use_program(Some(&p.program));
        for (unit, tex) in [(0, &g.color), (1, &g.normal), (2, &g.depth)] {
            gl.active_texture(Gl::TEXTURE0 + unit);
            gl.bind_texture(Gl::TEXTURE_2D, Some(tex));
        }
        gl.active_texture(Gl::TEXTURE0);
        p.set2f(gl, U::Texel, 1.0 / w as f32, 1.0 / h as f32);
        p.set1f(gl, U::Px, self.outline_px());
        p.set2f(gl, U::NearFar, camera.near(), camera.far());
        // sky + distance haze of the close views (GAME-CAMERA-VIEWS 5, 7; sky.rs)
        p.set_mat4(gl, U::InvViewProj, &view_proj.inverse());
        p.set3f(gl, U::Eye, eye);
        let fog = camera.fog();
        p.set4f(
            gl,
            U::Fog,
            [fog.start, fog.end, fog.amount, camera.sky_amount()],
        );
        let (top, horizon) = crate::sky::sky_colors(self.light.night);
        p.set3f(gl, U::SkyTop, top);
        p.set3f(gl, U::SkyHorizon, horizon);
        p.set1f(gl, U::SkyNight, self.light.night);
        p.set1f(gl, U::SkyClouds, f32::from(u8::from(self.clouds)));
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

/// Frame culling (PERF-R-015, PERF-R-018): the view frustum and, in the full close views,
/// the haze — a box whose nearest point is farther from the eye than the fog end (+ margin)
/// is fully hidden by the haze and not drawn. `all` (debug `no_culling`): no frustum test.
struct Cull {
    planes: [Vec4; 6],
    all: bool,
    haze: Option<(Vec3, f32)>,
}

impl Cull {
    fn visible(&self, min: Vec3, max: Vec3) -> bool {
        min.x <= max.x
            && (self.all || aabb_visible(&self.planes, min, max))
            && self
                .haze
                .is_none_or(|(eye, end)| eye.clamp(min, max).distance_squared(eye) <= end * end)
    }
}

/// Room beyond the fog end (m) before the haze culls a box.
const HAZE_CULL_MARGIN_M: f32 = 0.05;

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

    fn load(rel: &str) -> Model {
        let bytes = std::fs::read(format!("{}/../../{rel}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        Model::from_glb(&bytes).unwrap()
    }

    /// ARCH-007: static vertices carry the glow slots (sRGB emission, mode 1), glass tint
    /// (mode 3, its triangles sorted after the opaque ones) and each vertex's part pivot and
    /// code; the part behaviours come from the node names.
    #[test]
    fn arch_007_static_vertices_carry_slots_glass_and_parts() {
        let nh = load("assets/models/buildings/night_house.glb");
        let v = static_vertices(&nh.mesh, &nh.materials, &nh.nodes);
        assert_eq!(
            v.vertices.len(),
            nh.mesh.positions.len() * STATIC_VERTEX_FLOATS
        );
        assert_eq!(v.indices.len(), nh.mesh.indices.len());
        let glass_tris = (v.indices.len() as u32 - v.glass_first) / 3;
        assert_eq!(glass_tris, 72, "night house glass panes");
        // every index after glass_first is a glass vertex, none before
        let mode = |i: u32| v.vertices[i as usize * STATIC_VERTEX_FLOATS + 11];
        assert!(v.indices[v.glass_first as usize..]
            .iter()
            .all(|&i| mode(i) == 3.0));
        assert!(v.indices[..v.glass_first as usize]
            .iter()
            .all(|&i| mode(i) != 3.0));
        // glow slots: mode 1 with the sRGB emission
        assert!(v.indices.iter().any(|&i| mode(i) == 1.0));
        // roof / walls_upper parts: hide bits
        let roof = nh.node_index("roof").unwrap();
        let walls = nh.node_index("walls_upper").unwrap();
        assert!(v.has_nodes);
        assert_eq!(v.nodes[roof * 4 + 2], HIDE_ROOF as f32);
        assert_eq!(v.nodes[walls * 4 + 2], HIDE_WALLS_UPPER as f32);
        // a roof vertex carries its part code and pivot
        let k = nh
            .mesh
            .node
            .iter()
            .position(|&n| n as usize == roof)
            .unwrap();
        let o = k * STATIC_VERTEX_FLOATS;
        assert_eq!(v.vertices[o + 15], roof as f32);
        assert!((v.vertices[o + 13] - nh.nodes[roof].pivot.y).abs() < 1e-5);

        // moon door leaves turn ±90° about +Y (open to the back)
        let door = load("assets/models/props/moon_door.glb");
        let v = static_vertices(&door.mesh, &door.materials, &door.nodes);
        let l = door.node_index("leaf_l").unwrap();
        let r = door.node_index("leaf_r").unwrap();
        assert_eq!(v.nodes[l * 4], 2.0);
        assert!((v.nodes[l * 4 + 1] - 90f32.to_radians()).abs() < 1e-6);
        assert!((v.nodes[r * 4 + 1] + 90f32.to_radians()).abs() < 1e-6);
        assert_eq!(v.glass_first as usize, v.indices.len(), "no glass");
        // a model without parts: no node uniform needed
        assert!(v.is_rich(), "glow slots and leaves: the rich format");
        let bed = load("assets/models/props/bed.glb");
        let b = static_vertices(&bed.mesh, &bed.materials, &bed.nodes);
        assert!(!b.has_nodes);
        assert!(!b.is_rich(), "palette only: the lean 8-float format");
        let tile = load("assets/models/props/grass_tile.glb");
        assert!(!static_vertices(&tile.mesh, &tile.materials, &tile.nodes).is_rich());
        // behaviours by name
        assert_eq!(NodeBehaviour::of("sails").mode, 1.0);
        assert_eq!(NodeBehaviour::of("rotor").mode, 1.0);
        assert_eq!(NodeBehaviour::of("rotor").axis, 2.0);
        assert_eq!(NodeBehaviour::of("night_sky").mode, 2.0);
        assert_eq!(NodeBehaviour::of("glass"), NodeBehaviour::STATIC);
    }

    /// ARCH-008: static batching merges instances exactly as the instanced shader places
    /// them; glass, hiding, spinning and night-only parts are never baked.
    #[test]
    fn arch_008_baked_groups_match_their_instances() {
        let bed = load("assets/models/buildings/../props/garden_bed.glb");
        let b = static_vertices(&bed.mesh, &bed.materials, &bed.nodes);
        assert!(b.is_bakeable());
        let items = [
            (&b, Vec3::new(1.0, 0.0, -2.0), 0.0, Vec3::ONE),
            (
                &b,
                Vec3::new(4.0, 0.0, -2.0),
                std::f32::consts::FRAC_PI_2,
                Vec3::ONE,
            ),
        ];
        let origin = Vec3::new(2.5, 0.0, -2.0);
        let baked = bake_vertices(&items, origin);
        let n = b.vertices.len() / STATIC_VERTEX_FLOATS;
        assert_eq!(baked.vertices.len(), 2 * b.vertices.len());
        assert_eq!(baked.indices.len(), 2 * b.indices.len());
        assert_eq!(baked.indices[b.indices.len()], b.indices[0] + n as u32);
        // the second copy's vertex k = yaw 90° (x' = z, z' = −x) + position − origin
        for k in [0usize, 5, n - 1] {
            let v = &b.vertices[k * STATIC_VERTEX_FLOATS..];
            let w = &baked.vertices[(n + k) * STATIC_VERTEX_FLOATS..];
            let want = Vec3::new(v[2], v[1], -v[0]) + Vec3::new(4.0, 0.0, -2.0) - origin;
            assert!(Vec3::new(w[0], w[1], w[2]).abs_diff_eq(want, 1e-5), "{k}");
        }
        // what may be baked
        let can = |rel: &str| {
            let m = load(rel);
            static_vertices(&m.mesh, &m.materials, &m.nodes).is_bakeable()
        };
        assert!(
            can("assets/models/props/toy_chest.glb"),
            "the lid rests open"
        );
        assert!(
            can("assets/models/props/bedside_lamp.glb"),
            "glow slots stay"
        );
        assert!(!can("assets/models/buildings/night_house.glb"), "glass");
        assert!(
            !can("assets/models/buildings/zookeeper_house.glb"),
            "roof hides"
        );
        assert!(!can("assets/models/props/window_moon.glb"), "night sky");
        assert!(!can("assets/models/props/windmill.glb"), "sails spin");
        assert!(!can("assets/models/props/carousel.glb"), "rotor spins");
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

    // PERF-016 (PERF-R-003): the CPU frame block follows the std140 rules of
    // `shaders::frame_block()` (vec3 + float share 16 bytes, arrays start on 16).
    #[test]
    fn perf_016_frame_block_is_std140() {
        use std::mem::{offset_of, size_of};
        assert_eq!(offset_of!(FrameBlock, view), 0);
        assert_eq!(offset_of!(FrameBlock, view_proj), 64);
        assert_eq!(offset_of!(FrameBlock, sun_dir), 128);
        assert_eq!(offset_of!(FrameBlock, dither), 140);
        assert_eq!(offset_of!(FrameBlock, shadow_tint), 144);
        assert_eq!(offset_of!(FrameBlock, glow_on), 156);
        assert_eq!(offset_of!(FrameBlock, fade), 160);
        assert_eq!(offset_of!(FrameBlock, night), 176);
        assert_eq!(offset_of!(FrameBlock, time), 192);
        assert_eq!(offset_of!(FrameBlock, lights), 208);
        assert_eq!(
            offset_of!(FrameBlock, light_colors),
            208 + 16 * MAX_POINT_LIGHTS
        );
        assert_eq!(offset_of!(FrameBlock, pools), 208 + 32 * MAX_POINT_LIGHTS);
        assert_eq!(
            size_of::<FrameBlock>(),
            208 + 32 * MAX_POINT_LIGHTS + 16 * MAX_LIGHT_POOLS
        );
        // the members named in the driver check exist in the GLSL block, in this order
        let glsl = shaders::frame_block();
        let mut at = 0;
        for (name, _) in FRAME_OFFSETS {
            let n = name.trim_end_matches("[0]");
            let k = glsl[at..]
                .find(&format!(" {n}"))
                .unwrap_or_else(|| panic!("{n}"));
            at += k;
        }
    }

    // PERF-016: every uniform set per draw / pass has a name; no name of the frame block.
    #[test]
    fn perf_016_per_draw_uniforms_are_not_in_the_frame_block() {
        let glsl = shaders::frame_block();
        for u in U::ALL {
            assert!(!glsl.contains(&format!(" {};", u.name())), "{}", u.name());
            assert!(!glsl.contains(&format!(" {}[", u.name())), "{}", u.name());
        }
    }

    // PERF-023 (PERF-R-002): instances are uploaded sorted by chunk row-major; each chunk is
    // one contiguous range; instances of a chunk keep their order.
    #[test]
    fn perf_023_chunk_order_is_row_major_and_contiguous() {
        let keys = [
            IVec2::new(1, 0),
            IVec2::new(0, 1),
            IVec2::new(0, 0),
            IVec2::new(-1, 1),
        ];
        // instance → chunk index
        let inst = [0u16, 1, 2, 0, 3, 2, 1];
        let (order, spans) = chunk_order(&keys, &inst);
        // row z = 1 (south) first: (-1,1) then (0,1); then row z = 0: (0,0) then (1,0)
        let want_chunks: Vec<usize> = spans.iter().map(|s| s.0).collect();
        assert_eq!(want_chunks, vec![3, 1, 2, 0]);
        assert_eq!(order, vec![4, 1, 6, 2, 5, 0, 3]);
        let mut next = 0;
        for &(c, first, count) in &spans {
            assert_eq!(first, next);
            for slot in first..first + count {
                assert_eq!(inst[order[slot as usize] as usize] as usize, c);
            }
            next += count;
        }
        assert_eq!(next as usize, inst.len());
    }

    // PERF-023: one draw per run of visible chunks; cheap off-screen gaps are drawn along;
    // never more than MAX_CHUNK_RUNS draws; nothing visible → no draw.
    #[test]
    fn perf_023_chunk_runs_merge_cheap_gaps_and_are_capped() {
        let mut out = [(0, 0); MAX_CHUNK_RUNS];
        let vis = |v: &'static [u8]| move |i: usize| v[i] == 1;
        // two visible runs split by an expensive gap (3 chunks × 64 × 158 triangles)
        let v: &[u8] = &[0, 1, 1, 0, 0, 0, 1, 0];
        let n = plan_chunk_runs(v.len(), vis(v), |_| 64, 158, &mut out);
        assert_eq!(&out[..n], &[(1, 2), (6, 6)]);
        // the same gap of tiny grass tiles (28 triangles, 2 per chunk) is drawn along
        let n = plan_chunk_runs(v.len(), vis(v), |_| 2, 28, &mut out);
        assert_eq!(&out[..n], &[(1, 6)]);
        // nothing visible
        let v: &[u8] = &[0, 0, 0];
        assert_eq!(plan_chunk_runs(3, vis(v), |_| 1, 1, &mut out), 0);
        // every other chunk visible, expensive gaps: capped, the last run takes the rest
        let v: &[u8] = &[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let n = plan_chunk_runs(v.len(), vis(v), |_| 100, 200, &mut out);
        assert_eq!(n, MAX_CHUNK_RUNS);
        assert_eq!(out[0], (0, 0));
        assert_eq!(out[MAX_CHUNK_RUNS - 1], (2 * (MAX_CHUNK_RUNS - 1), 14));
    }

    // PERF-025 (PERF-R-015): the culling box of an instance holds every corner of its mesh
    // at any yaw (±radius square for quarter turns, √2 otherwise), scaled, and the parts
    // below the origin — so frustum culling never drops a visible instance.
    #[test]
    fn perf_025_instance_extent_holds_the_mesh_at_any_yaw() {
        let (radius, lo_y, height) = (1.5, -0.4, 2.0);
        for k in 0..64 {
            let yaw = k as f32 * std::f32::consts::TAU / 64.0;
            for scale in [0.5, 1.0, 2.0] {
                // the square's corner is the farthest vertex: radius_xz = radius × √2
                let rxz = radius * std::f32::consts::SQRT_2;
                let (lo, hi) = instance_extent(radius, rxz, lo_y, height, yaw, scale);
                let (s, c) = yaw.sin_cos();
                for (x, z) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
                    let p = Vec3::new(x * radius, 0.0, z * radius) * scale;
                    let w = Vec3::new(c * p.x + s * p.z, 0.0, -s * p.x + c * p.z);
                    assert!(w.x >= lo.x - 1e-4 && w.x <= hi.x + 1e-4, "{yaw} {scale}");
                    assert!(w.z >= lo.z - 1e-4 && w.z <= hi.z + 1e-4, "{yaw} {scale}");
                }
                assert!(lo.y <= lo_y * scale && hi.y >= height * scale);
            }
        }
        // quarter turns keep the tight square (ground tiles: no extra chunk overlap)
        let (lo, hi) = instance_extent(0.5, 0.7, 0.0, 0.05, std::f32::consts::FRAC_PI_2 * 3.0, 1.0);
        assert!((hi.x - 0.5).abs() < 1e-6 && (lo.z + 0.5).abs() < 1e-6);
        // a round mesh (tree canopy) keeps its circle at any yaw
        let (lo, hi) = instance_extent(2.0, 2.05, 0.0, 5.0, 0.3, 1.0);
        assert!((hi.x - 2.05).abs() < 1e-6 && (lo.z + 2.05).abs() < 1e-6);
    }

    // PERF-025: the swept space of turning parts is inside the culling extent: a door leaf
    // hinged at x = 1 (1 m wide, closed towards the centre) reaches √2 m when it opens.
    #[test]
    fn perf_025_turning_parts_are_inside_the_culling_extent() {
        let mut nodes = [0.0; MAX_NODE_PARTS * 4];
        nodes[4..8].copy_from_slice(&[2.0, std::f32::consts::FRAC_PI_2, 0.0, 0.0]); // part 1 turns about Y
        let mut vertices = Vec::new();
        for (x, y, code) in [(1.0f32, 0.0f32, 1.0f32), (0.0, 2.0, 1.0), (0.3, 0.0, 0.0)] {
            vertices.extend_from_slice(&[x, y, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
            vertices.extend_from_slice(&[0.0; 4]);
            vertices.extend_from_slice(&[1.0, 0.0, 0.0, code]); // pivot (1, 0, 0)
        }
        let p = StaticVertices {
            vertices,
            indices: vec![0, 1, 2],
            glass_first: 3,
            nodes,
            has_nodes: true,
        };
        let (rxz, rot, top, bottom) = static_reach(&p);
        // the leaf's far edge (x = 0) is 1 m from the hinge axis (Y): it can swing out to
        // 1 + 1 = 2 m from the origin (a sphere bound would say 1 + √5)
        assert!((rxz - 2.0).abs() < 1e-5 && (rot - 2.0).abs() < 1e-5);
        assert!(top >= 2.0 && bottom <= 0.0);
        // every angle of the swing stays inside
        for k in 0..32 {
            let a = k as f32 * std::f32::consts::TAU / 32.0;
            let (sn, c) = a.sin_cos();
            let q = Vec3::new(1.0 - c, 2.0, sn); // (0, 2, 0) turned about the hinge (1, _, 0)
            assert!(q.x.hypot(q.z) <= rxz + 1e-5);
        }
    }

    // PERF-017: the light-mask margin covers a yawed mesh's corners and parts below the origin.
    #[test]
    fn perf_017_light_margin_covers_rotated_corners() {
        let r = 2.0;
        // a corner (r, r) of a mesh with the extent r reaches r·√2 when yawed by 45°
        assert!(r + light_margin(r, 0.0) >= r * std::f32::consts::SQRT_2 + 1.0 - 1e-5);
        assert!(light_margin(r, -0.8) >= light_margin(r, 0.0) + 0.8 - 1e-5);
    }
}
