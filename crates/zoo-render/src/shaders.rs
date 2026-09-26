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
in vec3 v_normal;
in vec2 v_uv;
in vec4 v_color;
in float v_view_depth;
in float v_fadeable;
layout(location = 0) out vec4 o_color;
layout(location = 1) out vec4 o_normal;
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
    vec4 tex = texture(u_palette, v_uv);
    if (v_color.a < 0.5 && tex.a < 0.5) discard; // alpha-tested decals (faces, ART-RIG §6)
    vec3 albedo = v_color.a > 0.5 ? v_color.rgb : tex.rgb;
    vec3 n = normalize(v_normal);
    float lit = step(0.12, dot(n, u_sun_dir));
    vec3 c = albedo * mix(u_shadow_tint, vec3(1.0), lit);
    o_color = vec4(c, 1.0);
    o_normal = vec4(n * 0.5 + 0.5, u_edge_mask);
}
"#;

pub fn static_vs() -> String {
    r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec4 a_pos_yaw;     // instance: origin (world) + yaw
layout(location = 4) in vec4 a_scale_fade;  // instance: scale xyz + fadeable flag
layout(location = 5) in vec4 a_color;       // instance: flat colour (a = 1) or palette (a = 0)
uniform mat4 u_view;
uniform mat4 u_view_proj;
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;
out float v_view_depth;
out float v_fadeable;
void main() {
    float c = cos(a_pos_yaw.w);
    float s = sin(a_pos_yaw.w);
    vec3 p = a_pos * a_scale_fade.xyz;
    vec3 w = vec3(c * p.x + s * p.z, p.y, -s * p.x + c * p.z) + a_pos_yaw.xyz;
    vec3 n = a_normal / a_scale_fade.xyz;
    v_normal = vec3(c * n.x + s * n.z, n.y, -s * n.x + c * n.z);
    v_uv = a_uv;
    v_color = a_color;
    v_fadeable = a_scale_fade.w;
    v_view_depth = -(u_view * vec4(w, 1.0)).z;
    gl_Position = u_view_proj * vec4(w, 1.0);
}
"#
    .to_owned()
}

pub fn static_fs() -> String {
    format!("#version 300 es\n{CEL_FRAGMENT}")
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
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;
out float v_view_depth;
out float v_fadeable;
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
    v_fadeable = 0.0;
    v_view_depth = -(u_view * w).z;
    gl_Position = u_view_proj * w;
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
    o_color = vec4(mix(col, u_line_color, edge), 1.0);
}
"#
    .to_owned()
}
