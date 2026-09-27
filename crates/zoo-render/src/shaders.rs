//! GLSL ES 3.00 sources (TECH-ARCH §7): 2-tone cel shading into a G-buffer (colour +
//! normal/edge mask + depth), then a screen-space outline pass (Q-050).

/// Shared fragment code: hard 2-tone cel shading, MRT output, occluder cut-out.
const CEL_FRAGMENT: &str = r#"
precision highp float;
uniform sampler2D u_palette;
uniform vec3 u_sun_dir;       // towards the sun, world space, normalised
uniform vec3 u_shadow_tint;   // multiplier of the shadow tone
uniform float u_edge_mask;    // 1 = normal edges allowed (props), 0 = ground tiles
uniform vec4 u_fade;          // xy = player on screen (px), z = radius (px), w = player view depth
uniform float u_dither;       // screen-door cell size in px (half the outline sample offset)
uniform vec4 u_tint;          // rgb + amount: water tint of under-water characters
in vec3 v_normal;
in vec2 v_uv;
in vec4 v_color;
in float v_view_depth;
in float v_fadeable;
in vec3 v_world;
in vec4 v_glow;               // material slot: rgb + mode (2 emissive, 3 glass, 4 eye glow)
layout(location = 0) out vec4 o_color;
layout(location = 1) out vec4 o_normal;
// @night (GAME-NIGHT §10; night.rs)
void main() {
    if (v_fadeable > 0.5 && u_fade.z > 0.0) {
        vec2 d = gl_FragCoord.xy - u_fade.xy;
        if (dot(d, d) < u_fade.z * u_fade.z && v_view_depth < u_fade.w - 1.2) {
            // Screen-door fade (checker of u_dither px cells): half of the occluder's pixels
            // are dropped. The outline pass samples 2 cells apart, so it sees the same surface
            // and draws no lines inside the faded area.
            ivec2 p = ivec2(gl_FragCoord.xy / u_dither) & 1;
            if (p.x == p.y) discard;
        }
    }
    // glass (instance colour alpha 0.5, e.g. the fish bowl): screen-door half transparency
    if (v_color.a > 0.25 && v_color.a < 0.75) {
        ivec2 p = ivec2(gl_FragCoord.xy / u_dither) & 1;
        if (p.x == p.y) discard;
    }
    vec3 n = normalize(v_normal);
    if (v_color.a > 1.5 || (v_glow.w > 1.5 && v_glow.w < 2.5)) {
        // emissive (lamp glass, lit windows, eyeshine, fireflies, `*_glow` slots at night):
        // flat, unlit (night.rs)
        o_color = vec4(v_color.a > 1.5 ? v_color.rgb : v_glow.rgb, 1.0);
        o_normal = vec4(n * 0.5 + 0.5, u_edge_mask);
        return;
    }
    vec4 tex = texture(u_palette, v_uv);
    if (v_glow.w > 3.5 && v_glow.w < 4.5) {
        // eye_glow inside the lantern radius (NIGHT-006): highlights shine, the pupil stays dark
        float lum = dot(tex.rgb, vec3(0.2126, 0.7152, 0.0722));
        o_color = vec4(v_glow.rgb * (0.25 + 0.75 * lum), 1.0);
        o_normal = vec4(n * 0.5 + 0.5, u_edge_mask);
        return;
    }
    // slot colour: glass (3, alpha 0.35) and flat text faces (5) use their own colour; one
    // shade() call for every path (branches are predicated on software GPUs)
    bool slot = v_glow.w > 2.5;
    bool glass = slot && v_glow.w < 3.5;
    if (!slot && v_color.a < 0.25 && tex.a < 0.5) discard; // alpha-tested decals (faces, ART-RIG §6)
    vec3 albedo = slot ? v_glow.rgb : (v_color.a > 0.25 ? v_color.rgb : tex.rgb);
    albedo = mix(albedo, u_tint.rgb, u_tint.a);
    float lit = step(0.12, dot(n, u_sun_dir));
    float alpha = glass ? 0.35 : 1.0;
    o_color = vec4(shade(albedo, lit, n, v_world), alpha);
    // glass: the normal buffer keeps what is behind the pane (blend weight 0: no outline)
    o_normal = vec4(n * 0.5 + 0.5, glass ? 0.0 : u_edge_mask);
}
"#;

/// Bobbing of props on the water (TECH-WATER behaviour 8): per-batch `u_bob = (amp_y m,
/// tilt rad, drift radius m, 0)`, phase from an integer hash of the instance origin (the same
/// as `zoo_core::water::bob_hash`, so CPU-placed frogs move exactly with their pad).
const BOB_GLSL: &str = r#"
uniform float u_time;   // water clock, elapsed mod 16 s
uniform vec4 u_bob;
const float TAU = 6.2831853;
float bob_hash(vec3 o) {
    ivec2 k = ivec2(floor(o.xz * 10.0));
    uint h = uint(k.x) * 0x9E3779B1u ^ uint(k.y) * 0x85EBCA77u;
    h ^= h >> 15u;
    h *= 0x2C1B3C6Du;
    h ^= h >> 12u;
    return float(h & 0xFFFFu) / 65536.0;
}
"#;

/// Attributes, uniforms and the part / slot code of the *rich* static vertex shader: models
/// with material slots or moving parts (ARCH-007). Plain meshes (tiles, hedges, fences —
/// most of the scene) use the lean shader without them (half the vertex fetch).
const RICH_DECL: &str = r#"
layout(location = 6) in vec4 a_glow;        // slot: sRGB emission / colour + mode (1 glow, 3 glass, 5 flat)
layout(location = 7) in vec4 a_node;        // part pivot (model space) + part code (0 = root)
layout(location = 8) in vec4 a_inst_node;   // instance: open 0..1, hide mask
uniform vec4 u_nodes[8];    // per part: axis (1 X, 2 Y, 3 Z), angle at open 1, hide bit, mode
uniform float u_glow_on;    // night: glow slots emissive, night-only parts shown
vec3 turn(vec3 v, int axis, float a) {
    float c = cos(a), s = sin(a);
    if (axis == 1) return vec3(v.x, c * v.y - s * v.z, s * v.y + c * v.z);
    if (axis == 2) return vec3(c * v.x + s * v.z, v.y, -s * v.x + c * v.z);
    return vec3(c * v.x - s * v.y, s * v.x + c * v.y, v.z);
}
"#;

const RICH_MAIN: &str = r#"
    int code = int(a_node.w + 0.5);
    if (code > 0 && code < 8) {
        // moving / hideable part of a multi-node asset (ARCH-007)
        vec4 nd = u_nodes[code];
        int bit = int(nd.z + 0.5);
        if ((bit > 0 && (int(a_inst_node.y + 0.5) & bit) != 0) || (nd.w > 1.5 && u_glow_on < 0.5)) {
            gl_Position = vec4(0.0, 0.0, 2.0, 1.0);   // hidden: outside the clip volume
            v_normal = vec3(0.0, 1.0, 0.0); v_uv = a_uv; v_color = a_color; v_view_depth = 0.0;
            v_fadeable = 0.0; v_glow = vec4(0.0); v_world = vec3(0.0);
            return;
        }
        int axis = int(nd.x + 0.5);
        float ang = (nd.w > 0.5 && nd.w < 1.5) ? nd.y * u_time : nd.y * a_inst_node.x;
        if (axis > 0 && ang != 0.0) {
            mp = turn(mp - a_node.xyz, axis, ang) + a_node.xyz;
            mn = turn(mn, axis, ang);
        }
    }
    // glow slots emit only at night; glass (3) and flat slot colours (5) always apply
    v_glow = a_glow.w > 2.5 ? a_glow : (a_glow.w > 0.5 && u_glow_on > 0.5 ? vec4(a_glow.rgb, 2.0) : vec4(0.0));
"#;

fn static_vs_src(rich: bool) -> String {
    // every static batch outputs its world position (night point lights, GAME-NIGHT §10)
    let (rich_decl, rich_main) = if rich {
        (RICH_DECL, RICH_MAIN)
    } else {
        ("", "    v_glow = vec4(0.0);\n")
    };
    format!(
        r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec4 a_pos_yaw;     // instance: origin (world) + yaw
layout(location = 4) in vec4 a_scale_fade;  // instance: scale xyz + fadeable flag
layout(location = 5) in vec4 a_color;       // instance: flat colour (a = 1) or palette (a = 0)
uniform mat4 u_view;
uniform mat4 u_view_proj;
{BOB_GLSL}
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;
out float v_view_depth;
out float v_fadeable;
out vec4 v_glow;
out vec3 v_world;
{rich_decl}
void main() {{
    vec3 mp = a_pos;
    vec3 mn = a_normal;
{rich_main}
    float c = cos(a_pos_yaw.w);
    float s = sin(a_pos_yaw.w);
    vec3 p = mp * a_scale_fade.xyz;
    vec3 n = mn / a_scale_fade.xyz;
    vec3 off = vec3(0.0);
    if (u_bob.x + u_bob.y + u_bob.z > 0.0) {{
        float ph = bob_hash(a_pos_yaw.xyz) * TAU;
        float tilt = u_bob.y * sin(TAU * u_time / 4.0 + ph + 1.3);
        float ct = cos(tilt), st = sin(tilt);
        p = vec3(p.x, ct * p.y - st * p.z, st * p.y + ct * p.z);   // roll about local X
        n = vec3(n.x, ct * n.y - st * n.z, st * n.y + ct * n.z);
        float a = TAU * u_time / 16.0 + ph;
        off = vec3(u_bob.z * cos(a), u_bob.x * sin(TAU * u_time / 2.0 + ph), u_bob.z * sin(a));
    }}
    vec3 w = vec3(c * p.x + s * p.z, p.y, -s * p.x + c * p.z) + a_pos_yaw.xyz + off;
    v_normal = vec3(c * n.x + s * n.z, n.y, -s * n.x + c * n.z);
    v_uv = a_uv;
    v_color = a_color;
    v_fadeable = a_scale_fade.w;
    v_view_depth = -(u_view * vec4(w, 1.0)).z;
    v_world = w;
    gl_Position = u_view_proj * vec4(w, 1.0);
}}
"#
    )
}

pub fn static_vs() -> String {
    static_vs_src(false)
}

/// Static vertex shader of models with material slots / moving parts (ARCH-007).
pub fn rich_vs() -> String {
    static_vs_src(true)
}

/// Vertex shader of the water tile batches (the plain static one).
pub fn water_vs() -> String {
    static_vs_src(false)
}

pub fn static_fs() -> String {
    format!("#version 300 es\n{CEL_FRAGMENT}").replace(
        "// @night (GAME-NIGHT §10; night.rs)\n",
        &crate::night::night_glsl(),
    )
}

pub fn skinned_vs() -> String {
    r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec4 a_joints;
layout(location = 4) in vec4 a_weights;
uniform mat4 u_view;
uniform mat4 u_view_proj;
uniform mat4 u_model;
uniform highp sampler2D u_joint_tex;   // 4 RGBA32F texels (matrix columns) per joint
uniform vec4 u_color;
uniform vec3 u_eye;
uniform float u_depth_bias;   // m towards the camera for the depth test (under water)
uniform vec4 u_emit;          // eye_glow part that shines: rgb + mode 4 (else 0)
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;
out float v_view_depth;
out float v_fadeable;
out vec3 v_world;
out vec4 v_glow;
mat4 joint(float j) {
    int x = int(j) * 4;
    return mat4(texelFetch(u_joint_tex, ivec2(x, 0), 0),
                texelFetch(u_joint_tex, ivec2(x + 1, 0), 0),
                texelFetch(u_joint_tex, ivec2(x + 2, 0), 0),
                texelFetch(u_joint_tex, ivec2(x + 3, 0), 0));
}
void main() {
    mat4 skin = a_weights.x * joint(a_joints.x) + a_weights.y * joint(a_joints.y)
              + a_weights.z * joint(a_joints.z) + a_weights.w * joint(a_joints.w);
    mat4 m = u_model * skin;
    vec4 w = m * vec4(a_pos, 1.0);
    v_normal = mat3(m) * a_normal;
    v_uv = a_uv;
    v_color = u_color;
    v_glow = u_emit;
    v_fadeable = 0.0;
    v_view_depth = -(u_view * w).z;
    v_world = w.xyz;
    vec4 p = u_view_proj * w;
    if (u_depth_bias > 0.0) {
        vec4 q = u_view_proj * vec4(w.xyz + normalize(u_eye - w.xyz) * u_depth_bias, 1.0);
        p.z = q.z / q.w * p.w;
    }
    gl_Position = p;
}
"#
    .to_owned()
}

/// Decals (sign silhouettes, sign texts, ART-ENVIRONMENT 6/7): textured quads in world space,
/// alpha-blended into the G-buffer over their model face. Same 2-tone light as the face (its
/// normal), the normal buffer keeps the face normal and its edge mask (blend alpha: dst), so
/// no outline is drawn inside the panel.
pub fn decal_vs() -> String {
    r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;
uniform mat4 u_view_proj;
out vec2 v_uv;
void main() {
    v_uv = a_uv;
    gl_Position = u_view_proj * vec4(a_pos, 1.0);
}
"#
    .to_owned()
}

pub fn decal_fs() -> String {
    r#"#version 300 es
precision highp float;
uniform sampler2D u_tex;
uniform vec3 u_normal;
uniform vec3 u_sun_dir;
uniform vec3 u_shadow_tint;
uniform vec4 u_night;   // x night, y warm (night.rs)
in vec2 v_uv;
layout(location = 0) out vec4 o_color;
layout(location = 1) out vec4 o_normal;
void main() {
    vec4 t = texture(u_tex, v_uv);
    if (t.a < 0.02) discard;
    float lit = step(0.12, dot(u_normal, u_sun_dir));
    vec3 c = t.rgb * mix(u_shadow_tint, vec3(1.0), lit);
    // signs stay readable at night: lit by their lamp / the lantern post at every gate
    // (GAME-NIGHT rule 5, Q-118) — warm lamp light instead of the night grading
    c = mix(c, t.rgb * vec3(0.98, 0.90, 0.74), max(u_night.x, u_night.y * 0.5));
    o_color = vec4(c, t.a);
    o_normal = vec4(u_normal * 0.5 + 0.5, t.a);
}
"#
    .to_owned()
}

pub fn post_vs() -> String {
    r#"#version 300 es
out vec2 v_uv;
void main() {
    // Fullscreen triangle.
    vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
    v_uv = p;
    gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
"#
    .to_owned()
}

/// Screen-space outline pass (Q-050): depth edges from the Laplacian of inverse linear depth
/// (zero on planes, so 1 m tile seams and flat ground get no lines) and normal edges on
/// props (`edge mask`), drawn in the outline colour about `u_px` pixels wide.
pub fn post_fs() -> String {
    r#"#version 300 es
precision highp float;
uniform sampler2D u_color;
uniform sampler2D u_normal;
uniform highp sampler2D u_depth;
uniform vec2 u_texel;       // 1 / size
uniform float u_px;         // sample offset in pixels (≈ half the line width)
uniform vec2 u_near_far;
uniform vec3 u_line_color;
in vec2 v_uv;
out vec4 o_color;
// @atmosphere (sky + distance haze of the close views, GAME-CAMERA-VIEWS 5/7; sky.rs)
float inv_lin(vec2 uv) {
    float d = texture(u_depth, uv).r * 2.0 - 1.0;
    float n = u_near_far.x, f = u_near_far.y;
    float z = 2.0 * n * f / (f + n - d * (f - n));
    return 1.0 / z;
}
void main() {
    vec2 o = u_texel * u_px;
    float c = inv_lin(v_uv);
    float l = inv_lin(v_uv - vec2(o.x, 0.0));
    float r = inv_lin(v_uv + vec2(o.x, 0.0));
    float b = inv_lin(v_uv - vec2(0.0, o.y));
    float t = inv_lin(v_uv + vec2(0.0, o.y));
    // Signed: only the nearer side of a depth step gets the line (width = u_px).
    float lap = max(2.0 * c - l - r, 2.0 * c - b - t) / c;
    float depth_edge = smoothstep(0.012, 0.03, lap);

    vec4 nc = texture(u_normal, v_uv);
    vec4 nl = texture(u_normal, v_uv - vec2(o.x, 0.0));
    vec4 nb = texture(u_normal, v_uv - vec2(0.0, o.y));
    // One-sided (left/below neighbours only) so creases get a single line of u_px width.
    vec3 n0 = nc.xyz * 2.0 - 1.0;
    float nd = 1.0;
    nd = min(nd, mix(1.0, dot(n0, nl.xyz * 2.0 - 1.0), min(nc.a, nl.a)));
    nd = min(nd, mix(1.0, dot(n0, nb.xyz * 2.0 - 1.0), min(nc.a, nb.a)));
    float normal_edge = 1.0 - smoothstep(0.55, 0.75, nd);

    float edge = max(depth_edge, normal_edge);
    vec3 col = texture(u_color, v_uv).rgb;
    col = mix(col, u_line_color, edge);
    o_color = vec4(atmosphere(col, v_uv, texture(u_depth, v_uv).r), 1.0);
}
"#
    .replace(
        "// @atmosphere (sky + distance haze of the close views, GAME-CAMERA-VIEWS 5/7; sky.rs)\n",
        &crate::sky::atmosphere_glsl(),
    )
}

/// Water fragment shader (TECH-WATER §4): the water cells `water_river` (176) and
/// `water_pond` (179) of the water tiles are shaded procedurally from the baked water field
/// and the water clock (river streaks + flecks + shore foam + obstacle foam, pond shimmer +
/// rings + glints + lapping line, duck wakes and dip rings); every other cell (bank grass,
/// soil slope) takes the normal cel path. The surface stays flat: colour only, normal (0, 1,
/// 0) and the batch edge mask, so the outline pass sees no change (behaviour 11). Ported from
/// `web/prototypes/water.html`.
pub fn water_fs() -> String {
    r#"#version 300 es
precision highp float;
uniform sampler2D u_palette;
uniform vec3 u_sun_dir;
uniform vec3 u_shadow_tint;
uniform float u_edge_mask;
uniform float u_time;
uniform highp sampler2D u_field;   // RGBA16F: s, c, shore, flow
uniform vec4 u_field_xf;           // xy = world XZ of the field corner, zw = 1 / size (m)
uniform vec4 u_obstacles[4];       // world x, z, radius (0 = unused), s
uniform vec4 u_obstacles_b[4];     // c, river flag
uniform vec4 u_ripples[8];         // world x, z, heading x, heading z
uniform vec4 u_ripples_b[8];       // wake 0..1, dip ring age 0..1 (< 0: none)
uniform int u_ripple_count;
in vec3 v_normal;
in vec2 v_uv;
in vec4 v_color;
in float v_view_depth;
in float v_fadeable;
in vec3 v_world;
layout(location = 0) out vec4 o_color;
layout(location = 1) out vec4 o_normal;
// @night (GAME-NIGHT §10; night.rs)

const float TAU = 6.2831853;
const float PI = 3.14159265;
const float HALF_W = 1.16;   // river half width − waterline inset (3 m rivers, Q-066)
float g_px = 0.01;           // pixel footprint (m) of this fragment
vec3 RIVER, RIVER_L, FOAM, POND, POND_L;

vec3 pal(int cell) {
    return textureLod(u_palette, (vec2(float(cell % 16), float(cell / 16)) + 0.5) / 16.0, 0.0).rgb;
}
float hash1(float n) { return fract(sin(n * 12.9898) * 43758.5453); }
float hash2(vec2 v) { return fract(sin(dot(v, vec2(12.9898, 78.233))) * 43758.5453); }
float aastep(float e, float v) { float w = g_px * 0.6; return smoothstep(e - w, e + w, v); }

// River: lens-shaped light dashes on sinuous lanes scrolling along s (behaviour 3) and
// sparse white flecks (behaviour 4).
vec3 riverStreaks(float s, float c, float shore, float t, vec3 col) {
    const float LANE = 0.24;
    c += 0.045 * sin(s * 2.1 + c * 1.7);
    float li = floor(c / LANE);
    float lc = (fract(c / LANE) - 0.5) * LANE;
    float cn = clamp(abs((li + 0.5) * LANE) / HALF_W, 0.0, 1.0);
    float V = mix(0.85, 0.5, cn * cn);
    float P = V * 2.0;
    float x = (s - V * t) / P + hash1(li + 3.1) * 8.0;
    float u = fract(x);
    float h = hash2(vec2(li, mod(floor(x), 8.0)));
    float len = mix(0.28, 0.55, fract(h * 13.7));
    float hw = mix(0.02, 0.05, fract(h * 7.3)) * sin(PI * clamp(u / len, 0.0, 1.0))
             * step(u, len) * step(0.3, h);
    float m = 1.0 - aastep(hw, abs(lc) + (hw > 0.0 ? 0.0 : 1.0));
    col = mix(col, RIVER_L, m * step(0.16, shore));
    const float FL = 0.45;
    float fi = floor((c + 0.2) / FL);
    float fc = (fract((c + 0.2) / FL) - 0.5) * FL;
    float fx = (s - t) / 2.0 + hash1(fi + 9.7) * 8.0;
    float fu = fract(fx);
    float fh = hash2(vec2(fi + 40.0, mod(floor(fx), 8.0)));
    float fhw = 0.03 * sin(PI * clamp(fu / 0.09, 0.0, 1.0)) * step(fu, 0.09) * step(0.72, fh);
    float fm = 1.0 - aastep(fhw, abs(fc) + (fhw > 0.0 ? 0.0 : 1.0));
    return mix(col, FOAM, fm * step(0.4, shore));
}

// Shore foam: wobbling band travelling with the flow + a dashed second line (behaviour 5).
vec3 riverShore(float s, float shore, float t, vec3 col) {
    float wob = 0.5 + 0.35 * sin(TAU * (s / 1.4 - t / 2.0)) + 0.3 * sin(TAU * (s / 0.7 - t));
    float w = 0.035 + 0.05 * wob;
    col = mix(col, FOAM, 1.0 - aastep(w, shore));
    float dash = step(0.55, fract((s - 0.7 * t) / 0.7 + 0.3));
    return mix(col, FOAM, (1.0 - aastep(0.028, abs(shore - (w + 0.07)))) * dash);
}

// Pond: twinkling shimmer dashes, expanding rings, breathing glints, lapping line (7).
vec3 pondWater(vec2 p, float shore, float t) {
    vec3 col = POND;
    vec2 q = mat2(0.87, 0.5, -0.5, 0.87) * p;
    const float LANE = 0.42;
    float li = floor(q.y / LANE);
    float lc = (fract(q.y / LANE) - 0.5) * LANE;
    float x = q.x / 1.1 + hash1(li) * 5.0;
    float h = hash2(vec2(li, floor(x)));
    float u = fract(x);
    float pulse = max(0.0, sin(TAU * (t / 4.0 + h)));
    float hw = 0.045 * pulse * sin(PI * clamp(u / 0.4, 0.0, 1.0)) * step(u, 0.4) * step(0.45, h);
    col = mix(col, POND_L, (1.0 - aastep(hw, abs(lc) + (hw > 0.0 ? 0.0 : 1.0))) * step(0.3, shore));
    vec2 g = p / 2.5, gi = floor(g);
    float hr = hash2(gi + 7.0);
    vec2 ctr = (gi + 0.5 + (vec2(hash2(gi + 1.3), hash2(gi + 5.1)) - 0.5) * 0.28) * 2.5;
    float age = fract(t / 4.0 + hr);
    float dd = length(p - ctr);
    float th = 0.04 * (1.0 - age) + 0.006;
    float r1 = age * 0.95, r2 = age * 0.95 - 0.24;
    float ring = max(1.0 - aastep(th, abs(dd - r1)), (1.0 - aastep(th, abs(dd - r2))) * step(0.0, r2));
    col = mix(col, POND_L, ring * step(0.25, hr) * step(0.18, shore));
    vec2 hg = floor(p / 4.0);
    vec2 hc = (hg + 0.5 + (vec2(hash2(hg + 2.0), hash2(hg + 4.0)) - 0.5) * 0.4) * 4.0;
    vec2 hd = mat2(0.87, 0.5, -0.5, 0.87) * (p - hc);
    float grow = 0.8 + 0.2 * sin(TAU * t / 8.0 + hash2(hg) * TAU);
    float g1 = length(vec2(max(abs(hd.x) - 0.26 * grow, 0.0), hd.y)) - 0.045;
    vec2 hd2 = hd - vec2(0.12, -0.16);
    float g2 = length(vec2(max(abs(hd2.x) - 0.13 * grow, 0.0), hd2.y)) - 0.04;
    col = mix(col, POND_L, (1.0 - aastep(0.0, min(g1, g2))) * step(0.35, hash2(hg + 8.0)) * step(0.5, shore));
    float w = 0.04 + 0.025 * sin(TAU * (t / 4.0) + (p.x - p.y) * 1.3);
    return mix(col, POND_L, 1.0 - aastep(w, shore));
}

// Foam at obstacles (behaviour 6): ring, and in rivers a dashed V-wake downstream.
vec3 obstacleFoam(vec2 p, float s, float c, bool river, float t, vec3 col) {
    for (int i = 0; i < 4; i++) {
        vec4 o = u_obstacles[i];
        if (o.z <= 0.0) continue;
        vec2 d = p - o.xy;
        float dl = length(d);
        if (dl > o.z + 1.9) continue;
        float a = atan(d.y, d.x);
        float ring = o.z + 0.06 + 0.03 * sin(3.0 * a - TAU * t);
        col = mix(col, FOAM, 1.0 - aastep(ring, dl));
        if (river && u_obstacles_b[i].y > 0.5) {
            float ds = s - o.w, dc = abs(c - u_obstacles_b[i].x);
            float arm = o.z * 0.8 + ds * 0.22;
            float fade = 1.0 - clamp(ds / 1.1, 0.0, 1.0);
            float wake = (1.0 - aastep(0.006 + 0.014 * fade, abs(dc - arm))) * step(0.0, ds)
                       * step(0.5, fract((ds - 0.7 * t) / 0.35)) * step(0.001, fade);
            col = mix(col, FOAM, wake);
        }
    }
    return col;
}

// Ducks and swimming frogs: dashed V-wake behind, rings when they dip (GAME-AMBIENT 5).
vec3 ripples(vec2 p, float t, vec3 light, vec3 col) {
    for (int i = 0; i < 8; i++) {
        if (i >= u_ripple_count) break;
        vec4 r = u_ripples[i];
        vec2 d = p - r.xy;
        if (dot(d, d) > 2.6) continue;
        vec2 h = r.zw;
        float back = -dot(d, h);
        float lat = abs(dot(d, vec2(-h.y, h.x)));
        float wake = u_ripples_b[i].x;
        if (wake > 0.05 && back > 0.08 && back < 1.3) {
            float fade = 1.0 - back / 1.3;
            float arm = 0.1 + back * 0.4;
            float line = 1.0 - aastep(0.012 + 0.02 * fade * wake, abs(lat - arm));
            float dash = step(0.3, fract((back + t * 0.9) / 0.36));
            col = mix(col, light, line * dash * step(0.2, wake * (0.3 + fade)));
        }
        float age = u_ripples_b[i].y;
        if (age >= 0.0) {
            float dl = length(d);
            float th = 0.03 * (1.0 - age) + 0.006;
            float rr = 0.14 + age * 0.85;
            float ring = max(1.0 - aastep(th, abs(dl - rr)),
                             (1.0 - aastep(th, abs(dl - rr + 0.2))) * step(0.34, rr));
            col = mix(col, light, ring);
        }
    }
    return col;
}

void main() {
    vec2 p = v_world.xz;
    g_px = max(length(fwidth(p)) * 0.7071, 1e-4);
    int cell = int(floor(v_uv.y * 16.0)) * 16 + int(floor(v_uv.x * 16.0));
    vec3 n = normalize(v_normal);
    if (v_color.a > 0.25 || (cell != 176 && cell != 179)) {
        // bank grass / soil slope: normal 2-tone cel path
        vec3 albedo = v_color.a > 0.25 ? v_color.rgb : textureLod(u_palette, v_uv, 0.0).rgb;
        float lit = step(0.12, dot(n, u_sun_dir));
        o_color = vec4(shade(albedo, lit, n, v_world), 1.0);
        o_normal = vec4(n * 0.5 + 0.5, u_edge_mask);
        return;
    }
    RIVER = pal(176); RIVER_L = pal(177); FOAM = pal(178); POND = pal(179); POND_L = pal(180);
    vec4 F = textureLod(u_field, (p - u_field_xf.xy) * u_field_xf.zw, 0.0);
    float s = F.r, c = F.g, shore = F.b;
    float t = u_time;
    vec3 col;
    bool river = cell == 176;
    if (river) {
        col = riverStreaks(s, c, shore, t, RIVER);
        col = riverShore(s, shore, t, col);
        col = ripples(p, t, FOAM, col);
    } else {
        col = pondWater(p, shore, t);
        col = ripples(p, t, POND_L, col);
    }
    col = obstacleFoam(p, s, c, river, t, col);
    if (u_night.x > 0.0 || u_night.y > 0.0) {
        // night (GAME-NIGHT rule 1): blue-violet water, lantern pools as warm reflections,
        // reflected stars as small 4-point comic sparkles that twinkle
        col = shade(col, 1.0, vec3(0.0, 1.0, 0.0), v_world);
        vec2 sg = p / 1.3;
        vec2 si = floor(sg);
        float h = hash2(si + 17.0);
        vec2 sp = (si + 0.25 + 0.5 * vec2(hash2(si + 3.0), hash2(si + 9.0))) * 1.3;
        vec2 sd = abs(p - sp);
        float tw = 0.6 + 0.4 * sin(TAU * (t / 3.0 + h));
        float star = max(1.0 - aastep(0.018, sd.y + sd.x * 0.16), 1.0 - aastep(0.018, sd.x + sd.y * 0.16));
        star *= step(sd.x, 0.11 * tw) * step(sd.y, 0.11 * tw) * step(0.55, h) * step(0.2, shore);
        col = mix(col, vec3(1.0, 0.97, 0.80), star * u_night.x);
    }
    // flat, always lit surface: colour only, the outline pass sees a static plane
    o_color = vec4(col, 1.0);
    o_normal = vec4(0.5, 1.0, 0.5, u_edge_mask);
}
"#
    .replace(
        "// @night (GAME-NIGHT §10; night.rs)\n",
        &crate::night::night_glsl(),
    )
}

/// Vertex shader of instanced skinned characters (ambient animals, GAME-AMBIENT rule 10):
/// one draw call per model and material; row `gl_InstanceID` of the joint texture holds
/// the instance's joint matrices already multiplied by its model matrix.
pub fn crowd_vs() -> String {
    skinned_vs()
        .replace("ivec2(x, 0)", "ivec2(x, gl_InstanceID)")
        .replace("ivec2(x + 1, 0)", "ivec2(x + 1, gl_InstanceID)")
        .replace("ivec2(x + 2, 0)", "ivec2(x + 2, gl_InstanceID)")
        .replace("ivec2(x + 3, 0)", "ivec2(x + 3, gl_InstanceID)")
}
