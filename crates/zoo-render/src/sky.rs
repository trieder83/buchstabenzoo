//! Comic sky and distance haze of the close camera views (GAME-CAMERA-VIEWS 5, 7).
//!
//! Both are drawn in the outline pass (no extra draw call): where the G-buffer has no
//! geometry the pass paints the sky; everything else is mixed towards the sky colour of the
//! same view direction by the distance fog, so objects melt into the horizon and anything
//! beyond the fog end is fully hidden. The zoo view passes `amount = 0` and is unchanged.
//!
//! Sky: a vertical gradient (pastel haze at the horizon → friendly blue at the top) with a
//! ring of flat, rounded comic clouds painted like a backdrop in (azimuth, elevation) space —
//! white, one flat shadow tone at the bottom, a dark-brown outline (ART-DIRECTION). No sun.

use glam::Vec3;

/// Haze / sky colour at the horizon (pastel cream-blue) — the fog colour below the horizon.
pub const SKY_HORIZON: Vec3 = Vec3::new(0.87, 0.93, 0.93);
/// Sky colour at the top.
pub const SKY_TOP: Vec3 = Vec3::new(0.42, 0.71, 0.93);
/// Cloud colour (lit) and its one flat shadow tone.
pub const CLOUD: Vec3 = Vec3::new(1.0, 0.99, 0.96);
pub const CLOUD_SHADE: Vec3 = Vec3::new(0.80, 0.82, 0.94);
/// Elevation (radians) where the gradient reaches [`SKY_TOP`].
pub const SKY_GRADIENT_TOP_RAD: f32 = 0.9;

/// Sky colours `(top, horizon)` for a night amount 0 (day) … 1 (night): the close views'
/// sky and haze turn dark blue at night (GAME-NIGHT, Q-126: dark-blue gradient, the haze in
/// the same blue as the horizon).
pub fn sky_colors(night: f32) -> (Vec3, Vec3) {
    let k = night.clamp(0.0, 1.0);
    (
        SKY_TOP.lerp(crate::night::NIGHT_SKY_TOP, k),
        SKY_HORIZON.lerp(crate::night::NIGHT_SKY_HORIZON, k),
    )
}

/// Sky gradient colour for a view direction (CPU reference of the shader's gradient).
pub fn sky_gradient(dir: Vec3) -> Vec3 {
    let d = dir.normalize_or(Vec3::Y);
    let el = d.y.clamp(-1.0, 1.0).asin().max(0.0);
    let t = (el / SKY_GRADIENT_TOP_RAD).clamp(0.0, 1.0).powf(0.8);
    SKY_HORIZON.lerp(SKY_TOP, t)
}

fn vec3_glsl(v: Vec3) -> String {
    format!("vec3({:.4}, {:.4}, {:.4})", v.x, v.y, v.z)
}

/// GLSL (fragment, ES 3.00) inserted into the outline pass: uniforms and
/// `vec3 atmosphere(vec3 col, vec2 uv, float depth)`. Needs `u_line_color`.
pub fn atmosphere_glsl() -> String {
    format!(
        r#"
uniform mat4 u_inv_view_proj;
uniform vec3 u_eye;
uniform vec4 u_fog;   // start m, end m, fog amount 0..1, sky amount 0..1
uniform vec3 u_sky_top;       // day {top} … night (GAME-NIGHT, Q-126)
uniform vec3 u_sky_horizon;   // day {horizon} … night; also the haze colour
uniform float u_sky_night;    // 0 day … 1 night: moon, stars, blue clouds
const vec3 CLOUD = {cloud};
const vec3 CLOUD_SHADE = {shade};
const vec3 NIGHT_CLOUD = vec3(0.30, 0.38, 0.66);
const vec3 NIGHT_CLOUD_SHADE = vec3(0.22, 0.28, 0.54);
const vec3 MOON = vec3(1.0, 0.957, 0.788);
const vec3 MOON_SPOT = vec3(0.918, 0.875, 0.686);
const float SKY_GRADIENT_TOP = {grad_top:.4};
float sky_hash(float n) {{
    return fract(sin(n * 12.9898 + 4.1414) * 43758.5453);
}}
// Signed distance (degrees) to one comic cloud: puffs on a flat bottom.
float cloud_sdf(vec2 p, float s) {{
    float d = length(p - vec2(0.0, 0.25 * s)) - 0.52 * s;
    d = min(d, length(p - vec2(-0.62 * s, 0.02 * s)) - 0.36 * s);
    d = min(d, length(p - vec2(0.64 * s, 0.0)) - 0.34 * s);
    d = min(d, length(p - vec2(0.28 * s, 0.52 * s)) - 0.34 * s);
    return max(d, -0.12 * s - p.y);   // flat bottom
}}
// Sky colour; `haze` = the gradient without clouds.
vec3 sky_color(vec3 dir, out vec3 haze) {{
    float el = asin(clamp(dir.y, -1.0, 1.0));
    float t = pow(clamp(max(el, 0.0) / SKY_GRADIENT_TOP, 0.0, 1.0), 0.8);
    vec3 c = mix(u_sky_horizon, u_sky_top, t);
    haze = c;
    // night (Q-126): a few 4-point stars and the flat outlined comic moon
    float eld0 = degrees(el);
    float az0 = degrees(atan(dir.x, -dir.z)) + 180.0;
    vec2 sg = vec2(az0, eld0) / 7.0;
    vec2 si = floor(sg);
    float sh = sky_hash(si.x * 37.0 + si.y * 101.0);
    vec2 sd = abs((fract(sg) - 0.5 - (vec2(sky_hash(sh * 91.0), sky_hash(sh * 53.0)) - 0.5) * 0.5) * 7.0);
    float star = step(sd.y + sd.x * 0.2, 0.28) + step(sd.x + sd.y * 0.2, 0.28);
    star *= step(0.8, sh) * step(12.0, eld0) * u_sky_night;
    c = mix(c, vec3(1.0, 0.95, 0.75), min(star, 1.0));
    vec2 md = vec2(az0 - 150.0, eld0 - 30.0);
    md.x -= 360.0 * floor(md.x / 360.0 + 0.5);
    float mr = length(md);
    float mw = max(fwidth(mr), 1e-4) * 1.6;
    if (u_sky_night > 0.5 && mr < 5.0 + mw) {{
        vec3 mc = MOON;
        if (length(md - vec2(1.4, 1.2)) < 1.1 || length(md - vec2(-1.6, -0.8)) < 0.8) mc = MOON_SPOT;
        c = mr < 5.0 - mw ? mc : u_line_color;
    }}
    // (no early return: fwidth below needs uniform control flow)
    // clouds on a ring of 8 cells of 45° azimuth (backdrop, degrees)
    float az = degrees(atan(dir.x, -dir.z)) + 180.0;   // 0..360
    float eld = degrees(el);
    float best = 1e3;
    float by = 0.0;
    float bs = 1.0;
    float cell = floor(az / 45.0);
    for (int k = -1; k <= 1; k++) {{
        float i = mod(cell + float(k), 8.0);
        if (sky_hash(i + 0.5) < 0.2) continue;   // a gap now and then
        float cx = i * 45.0 + 22.5 + (sky_hash(i + 1.7) - 0.5) * 18.0;
        float cy = 7.0 + sky_hash(i + 3.1) * 9.0;   // low: visible in look-around too
        float s = 5.5 + sky_hash(i + 5.3) * 3.5;
        float dx = az - cx;
        dx -= 360.0 * floor(dx / 360.0 + 0.5);
        vec2 p = vec2(dx, eld - cy);
        float d = cloud_sdf(p, s);
        if (d < best) {{ best = d; by = p.y; bs = s; }}
    }}
    float w = max(fwidth(best), 1e-4) * 1.6;   // outline ≈ 1.6 px
    if (best < -w) {{
        c = by < 0.1 * bs ? mix(CLOUD_SHADE, NIGHT_CLOUD_SHADE, u_sky_night)
                          : mix(CLOUD, NIGHT_CLOUD, u_sky_night);
    }} else if (best < w) {{
        c = u_line_color;
    }}
    // clouds sink into the haze near the horizon
    return mix(u_sky_horizon, c, smoothstep(0.02, 0.12, el));
}}
vec3 atmosphere(vec3 col, vec2 uv, float depth) {{
    if (u_fog.z <= 0.0 && u_fog.w <= 0.0) return col;
    vec2 ndc = uv * 2.0 - 1.0;
    vec4 f = u_inv_view_proj * vec4(ndc, 1.0, 1.0);
    vec3 dir = normalize(f.xyz / f.w - u_eye);
    vec3 haze;
    vec3 sky = sky_color(dir, haze);
    if (depth >= 0.99999) return mix(col, sky, u_fog.w);
    vec4 w = u_inv_view_proj * vec4(ndc, depth * 2.0 - 1.0, 1.0);
    float dist = length(w.xyz / w.w - u_eye);
    float fog = smoothstep(u_fog.x, u_fog.y, dist) * u_fog.z;
    // clouds only in the last bit of the haze: no ghost clouds over nearer houses, but a
    // fully hidden object is exactly the sky (no silhouette cut into a cloud)
    return mix(col, mix(haze, sky, smoothstep(0.85, 1.0, fog)), fog);
}}
"#,
        horizon = vec3_glsl(SKY_HORIZON),
        top = vec3_glsl(SKY_TOP),
        cloud = vec3_glsl(CLOUD),
        shade = vec3_glsl(CLOUD_SHADE),
        grad_top = SKY_GRADIENT_TOP_RAD,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // CAMV-005/007 (behaviour 7): the haze below and at the horizon is the horizon colour, the
    // gradient gets bluer upwards, clouds are white with a light shadow tone.
    #[test]
    fn sky_gradient_from_haze_to_blue() {
        assert!(sky_gradient(Vec3::new(1.0, -0.3, 0.0)).abs_diff_eq(SKY_HORIZON, 1e-6));
        assert!(sky_gradient(Vec3::X).abs_diff_eq(SKY_HORIZON, 1e-6));
        let mid = sky_gradient(Vec3::new(1.0, 0.4, 0.0));
        let top = sky_gradient(Vec3::Y);
        assert!(top.abs_diff_eq(SKY_TOP, 1e-6));
        assert!(mid.z >= SKY_TOP.z && mid.x < SKY_HORIZON.x && mid.x > SKY_TOP.x);
        assert!(CLOUD.min_element() > CLOUD_SHADE.min_element());
        const { assert!(CLOUD_SHADE.z > CLOUD_SHADE.x, "blue-violet shadow tone") };
    }

    // CAMV-021, Q-126: at night the sky and the haze are dark blue (the haze = the horizon colour).
    #[test]
    fn night_sky_is_dark_blue() {
        let (top, horizon) = sky_colors(1.0);
        assert!(top.abs_diff_eq(crate::night::NIGHT_SKY_TOP, 1e-6));
        assert!(horizon.z > horizon.x && horizon.z > horizon.y && horizon.z < 0.7);
        assert_eq!(sky_colors(0.0), (SKY_TOP, SKY_HORIZON));
    }

    #[test]
    fn glsl_declares_the_uniforms() {
        let s = atmosphere_glsl();
        for u in [
            "u_inv_view_proj",
            "u_eye",
            "u_fog",
            "u_sky_top",
            "u_sky_horizon",
            "u_sky_night",
            "vec3 atmosphere(",
        ] {
            assert!(s.contains(u), "{u}");
        }
    }
}
