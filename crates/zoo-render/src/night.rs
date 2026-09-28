//! Night mode of the renderer (GAME-NIGHT §10, `art/night/README.md` "Night colours"):
//! a global light blend day ↔ dusk (warm orange) ↔ night (deep friendly blue), point lights
//! with a hard cartoon falloff (the player's hand lantern + the nearest lamps), flat
//! light-pool decals on the ground for the other lamps (Q-114) and emissive surfaces
//! (lamp glass, lit windows, eyes, fireflies). Cel shading and outlines stay.
//!
//! The grading is a colour ramp, not a plain multiply: mid colours (grass, leaves) become a
//! saturated deep blue, light colours (paths, white fur) a pale lavender, and a part of the
//! original chroma stays so wood and clothes remain recognisable — like the approved night
//! style frame. The darkest tone never goes below [`NIGHT_FLOOR`] (`#2B3566`, never black).
//!
//! [`grade`] is the CPU reference of the GLSL in [`night_glsl`] (unit-tested).

use glam::Vec3;

/// Converts `#RRGGBB` to linear-free sRGB floats (the renderer works in sRGB like the palette).
pub const fn hex(rgb: u32) -> Vec3 {
    Vec3::new(
        ((rgb >> 16) & 0xFF) as f32 / 255.0,
        ((rgb >> 8) & 0xFF) as f32 / 255.0,
        (rgb & 0xFF) as f32 / 255.0,
    )
}

/// Darkest night tone (art plan: "darkest shadow tone not below `#2B3566`").
pub const NIGHT_FLOOR: Vec3 = hex(0x2B3566);
/// Night ramp: mid and dark colours (grass, leaves, wood) → deep friendly blue.
pub const NIGHT_DEEP: Vec3 = Vec3::new(0.20, 0.27, 0.66);
/// Night ramp: light colours (paths, white fur) → pale moonlit lavender (key `#AFC3FF` side).
pub const NIGHT_PALE: Vec3 = Vec3::new(0.70, 0.72, 0.86);
/// Luma range of the ramp (smoothstep edges).
pub const NIGHT_RAMP: (f32, f32) = (0.55, 1.0);
/// Share of the original chroma that stays at night.
pub const NIGHT_CHROMA: f32 = 0.5;
/// Shadow side at night (one hard darker blue tone).
pub const NIGHT_SHADOW: f32 = 0.74;
/// Dusk light on the lit side (warm orange) and in the shadow (violet).
pub const DUSK_LIT: Vec3 = Vec3::new(1.0, 0.80, 0.62);
pub const DUSK_SHADOW: Vec3 = Vec3::new(0.60, 0.50, 0.70);
/// Warm lamp light inside a light pool (`#FFC46E` pool, applied as a light on the albedo).
pub const LAMP_LIGHT: Vec3 = Vec3::new(1.0, 0.94, 0.80);
/// Lamp glow (emissive glass) `#FFD66B`.
pub const LAMP_GLOW: Vec3 = hex(0xFFD66B);
/// Lit window pane `#FFC857`.
pub const WINDOW_GLOW: Vec3 = hex(0xFFC857);
/// Eyeshine `#E6F7A0` (never red).
pub const EYE_GLOW: Vec3 = hex(0xE6F7A0);
/// Fireflies `#EFFF8A`.
pub const FIREFLY_GLOW: Vec3 = hex(0xEFFF8A);
/// Moon sign / moon disc `#FFF4C9`, moon door rim `#8FB8FF`.
pub const MOON_GLOW: Vec3 = hex(0xFFF4C9);
pub const MOON_RIM: Vec3 = hex(0x8FB8FF);
/// Night house indoor light: soft blue `#5B7FE0`, warm red-orange `#E8735A` (Q-116).
pub const NIGHT_HOUSE_BLUE: Vec3 = hex(0x5B7FE0);
pub const NIGHT_HOUSE_WARM: Vec3 = hex(0xE8735A);
/// Night sky of the close views (Q-126): top, band, horizon (= haze).
pub const NIGHT_SKY_TOP: Vec3 = hex(0x1E2A5A);
pub const NIGHT_SKY_BAND: Vec3 = hex(0x2E3E7A);
pub const NIGHT_SKY_HORIZON: Vec3 = hex(0x3B4C8C);
/// Clear colour outside the level at night (dark blue grass).
pub const NIGHT_CLEAR: Vec3 = Vec3::new(0.16, 0.22, 0.45);

/// Point lights per frame: the player's lantern + the nearest lamps (Q-114).
pub const MAX_POINT_LIGHTS: usize = 9;
/// Light-pool decals per frame (lamps beyond the point-light budget, nearest first).
pub const MAX_LIGHT_POOLS: usize = 24;
/// Instance colour alpha that marks an emissive (unlit) surface.
pub const EMISSIVE_ALPHA: f32 = 2.0;

/// Global light of a frame: `night` 0 (day) … 1 (night), `warm` 0 … 1 (dusk / morning glow).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DayLight {
    pub night: f32,
    pub warm: f32,
}

/// A point light: world position, radius (m, hard edge) and colour (rgb, strength in a).
/// `tinted`: a coloured light (night house blue / red-orange) keeps its colour; lamp light
/// is pulled towards a flat warm cream pool.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub pos: Vec3,
    pub radius: f32,
    pub color: Vec3,
    pub strength: f32,
    pub tinted: bool,
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Luma (Rec. 601) of an sRGB colour.
pub fn luma(c: Vec3) -> f32 {
    c.dot(Vec3::new(0.299, 0.587, 0.114))
}

/// The pure night colour of an albedo (`lit` = 1 on the moonlit side, 0 in the shadow).
pub fn night_color(albedo: Vec3, lit: f32) -> Vec3 {
    let l = luma(albedo);
    let base = NIGHT_DEEP.lerp(NIGHT_PALE, smoothstep(NIGHT_RAMP.0, NIGHT_RAMP.1, l));
    let c = base + (albedo - Vec3::splat(l)) * NIGHT_CHROMA;
    let c = c * (NIGHT_SHADOW + (1.0 - NIGHT_SHADOW) * lit);
    c.max(NIGHT_FLOOR)
}

/// CPU reference of the shader grading (no point lights): day cel shading (`shadow_tint` in
/// the shadow), blended towards dusk (`warm`) and night (`night`).
pub fn grade(albedo: Vec3, lit: f32, shadow_tint: Vec3, light: DayLight) -> Vec3 {
    let day = albedo * shadow_tint.lerp(Vec3::ONE, lit);
    let dusk = albedo * DUSK_SHADOW.lerp(DUSK_LIT, lit);
    let c = day.lerp(dusk, light.warm.clamp(0.0, 1.0));
    c.lerp(night_color(albedo, lit), light.night.clamp(0.0, 1.0))
}

/// Hard cartoon falloff of a point light at distance `d` (1 inside, 0 outside, a thin
/// anti-aliased edge `aa` m wide).
pub fn light_band(d: f32, radius: f32, aa: f32) -> f32 {
    1.0 - smoothstep(radius - aa, radius, d)
}

fn glsl3(v: Vec3) -> String {
    format!("vec3({:.4}, {:.4}, {:.4})", v.x, v.y, v.z)
}

/// GLSL (fragment) with the per-draw light mask and `vec3 shade(vec3 albedo, float lit,
/// vec3 n, vec3 world)`: day cel shading, dusk and night grading, point lights and light
/// pools. Needs the shared frame block (`u_shadow_tint`, `u_night`, `u_lights`,
/// `u_light_colors`, `u_pools`; `shaders::frame_block`).
///
/// PERF-R-001: a draw gets only the lights that can reach it (`u_light_mask`, bit `i` = light
/// `i`, [`light_mask`]; the mask loop is uniform control flow), and a fragment outside a
/// light's hard edge skips the band and the colour work. The edge width is still
/// `fwidth(distance)` taken per light before the skip, exactly as before, so the picture is
/// pixel-identical (moving the derivatives out of the loop changes rim pixels: PERF-R-014,
/// Q-180).
pub fn night_glsl() -> String {
    format!(
        r#"
uniform highp uvec2 u_light_mask;   // per draw: x bit i = point light i, y bit i = light pool i
const vec3 NIGHT_FLOOR = {floor};
const vec3 NIGHT_DEEP = {deep};
const vec3 NIGHT_PALE = {pale};
const vec3 DUSK_LIT = {dusk_lit};
const vec3 DUSK_SHADOW = {dusk_shadow};
const vec3 LAMP_LIGHT = {lamp};
vec3 night_color(vec3 albedo, float lit) {{
    float l = dot(albedo, vec3(0.299, 0.587, 0.114));
    vec3 base = mix(NIGHT_DEEP, NIGHT_PALE, smoothstep({r0:.3}, {r1:.3}, l));
    vec3 c = base + (albedo - vec3(l)) * {chroma:.3};
    c *= {shadow:.3} + (1.0 - {shadow:.3}) * lit;
    return max(c, NIGHT_FLOOR);
}}
// hard cartoon edge at r, anti-aliased over fw = fwidth(d) (at least 2 cm)
float light_band(float d, float fw, float r) {{
    float aa = max(fw, 0.02);
    return 1.0 - smoothstep(r - aa, r, d);
}}
vec3 shade(vec3 albedo, float lit, vec3 n, vec3 world) {{
    vec3 c = albedo * mix(u_shadow_tint, vec3(1.0), lit);
    if (u_night.x <= 0.0 && u_night.y <= 0.0) return c;
    c = mix(c, albedo * mix(DUSK_SHADOW, DUSK_LIT, lit), u_night.y);
    c = mix(c, night_color(albedo, lit), u_night.x);
    // lamp light: flat warm pools with a hard edge (lantern, nearest lamps)
    float k = 0.0;
    bool tinted = false;
    vec3 tint = vec3(0.0);
    for (int i = 0; i < {np}; i++) {{
        uint bits = u_light_mask.x >> uint(i);
        if (bits == 0u) break;
        if ((bits & 1u) == 0u) continue;
        vec4 L = u_lights[i];
        vec3 to = L.xyz - world;
        float d = length(to);
        float fw = fwidth(d);   // before the skip: every quad lane computes it
        if (d >= L.w) continue;   // outside the hard edge: no light (band 0)
        float a = u_light_colors[i].a;   // strength, + 2 for a tinted (coloured) light
        float b = light_band(d, fw, L.w) * (a > 1.5 ? a - 2.0 : a);
        if (b > k) {{ k = b; tint = u_light_colors[i].rgb; tinted = a > 1.5; }}
    }}
    // light-pool decals: flat pools on the ground under the other lamps
    if (n.y > 0.6 && world.y < 0.5) {{
        for (int i = 0; i < {npool}; i++) {{
            uint bits = u_light_mask.y >> uint(i);
            if (bits == 0u) break;
            if ((bits & 1u) == 0u) continue;
            vec4 P = u_pools[i];
            float d = length(world.xz - P.xy);
            float fw = fwidth(d);
            if (d >= P.z) continue;
            float b = light_band(d, fw, P.z) * P.w;
            if (b > k) {{ k = b; tint = LAMP_LIGHT; tinted = false; }}
        }}
    }}
    // warm cream pool: the lit colour pulled a little towards cream (flat light pool of the
    // style frame; grass does not turn day-green)
    float la = dot(albedo, vec3(0.299, 0.587, 0.114));
    // ground pools are flat discs of the light colour (warm cream for lamps, blue / red-orange
    // in the night house); walls, fences and characters keep most of their colour
    float cream = n.y > 0.6 ? 0.7 : 0.14;
    vec3 pool = tinted ? tint * 0.85 : vec3(1.0, 0.87, 0.66);
    vec3 lamp = mix(albedo * tint, pool * (0.62 + 0.38 * la), cream);
    lamp *= mix(0.84, 1.0, lit);
    return mix(c, lamp, 0.9 * k * u_night.x);
}}
"#,
        np = MAX_POINT_LIGHTS,
        npool = MAX_LIGHT_POOLS,
        floor = glsl3(NIGHT_FLOOR),
        deep = glsl3(NIGHT_DEEP),
        pale = glsl3(NIGHT_PALE),
        dusk_lit = glsl3(DUSK_LIT),
        dusk_shadow = glsl3(DUSK_SHADOW),
        lamp = glsl3(LAMP_LIGHT),
        r0 = NIGHT_RAMP.0,
        r1 = NIGHT_RAMP.1,
        chroma = NIGHT_CHROMA,
        shadow = NIGHT_SHADOW,
    )
}

/// Lamps farther than this from the camera target are not lit (off screen in every view:
/// the zoo view shows ≈ 24 × 14 m at the maximum zoom, the close views end at the 16 m haze).
pub const LAMP_CULL_M: f32 = 22.0;

/// Picks the point lights and light pools of a frame (Q-114): `lamps` (world position,
/// radius, colour) within `max_dist` of `center`, nearest first; the nearest
/// `MAX_POINT_LIGHTS - reserved` become point lights, the next [`MAX_LIGHT_POOLS`] light-pool
/// decals. Allocation-free once `order` has grown (scratch space).
pub fn pick_lamps(
    lamps: &[PointLight],
    center: Vec3,
    max_dist: f32,
    reserved: usize,
    order: &mut Vec<(f32, usize)>,
    lights: &mut Vec<PointLight>,
    pools: &mut Vec<PointLight>,
) {
    order.clear();
    let max2 = (max_dist * max_dist).max(0.0);
    order.extend(
        lamps
            .iter()
            .enumerate()
            .map(|(i, l)| (l.pos.distance_squared(center), i))
            .filter(|(d, _)| *d <= max2),
    );
    order.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    let n_lights = MAX_POINT_LIGHTS.saturating_sub(reserved);
    for (k, &(_, i)) in order.iter().enumerate() {
        if k < n_lights {
            lights.push(lamps[i]);
        } else if k < n_lights + MAX_LIGHT_POOLS {
            pools.push(lamps[i]);
        } else {
            break;
        }
    }
}

/// Height (m) below which a ground fragment can be lit by a light pool (`shade()` GLSL).
pub const POOL_MAX_Y: f32 = 0.5;

/// Which lights of a frame can reach a draw (PERF-R-001): bit `i` of `.0` is set when point
/// light `i` (`lights`: world x, y, z, radius) has its sphere intersect the box
/// `min`..`max`; bit `i` of `.1` when light pool `i` (`pools`: world x, z, radius, strength)
/// has its disc intersect the box's ground footprint and the box reaches below
/// [`POOL_MAX_Y`]. A light without its bit lights no fragment inside the box (the band is 0
/// at and beyond the radius), so the masked shading equals shading with every light.
pub fn light_mask(lights: &[[f32; 4]], pools: &[[f32; 4]], min: Vec3, max: Vec3) -> (u32, u32) {
    const EPS: f32 = 1e-3;
    let mut m = (0u32, 0u32);
    for (i, l) in lights.iter().take(32).enumerate() {
        let c = Vec3::new(l[0], l[1], l[2]);
        let r = l[3] + EPS;
        if c.clamp(min, max).distance_squared(c) < r * r {
            m.0 |= 1 << i;
        }
    }
    if min.y < POOL_MAX_Y + EPS {
        let (lo, hi) = (glam::Vec2::new(min.x, min.z), glam::Vec2::new(max.x, max.z));
        for (i, p) in pools.iter().take(32).enumerate() {
            let c = glam::Vec2::new(p[0], p[1]);
            let r = p[2] + EPS;
            if c.clamp(lo, hi).distance_squared(c) < r * r {
                m.1 |= 1 << i;
            }
        }
    }
    m
}

/// Mask with the first `n_lights` point lights and `n_pools` light pools (draws without
/// bounds: dynamic batches).
pub fn full_light_mask(n_lights: usize, n_pools: usize) -> (u32, u32) {
    let bits = |n: usize| {
        if n >= 32 {
            u32::MAX
        } else {
            (1u32 << n) - 1
        }
    };
    (bits(n_lights), bits(n_pools))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHADOW_TINT: Vec3 = Vec3::new(0.70, 0.68, 0.84);

    // NIGHT-012, NIGHT-005 / art plan: the darkest tone at night is never below #2B3566 (never black).
    #[test]
    fn night_never_darker_than_the_floor() {
        for albedo in [
            Vec3::ZERO,
            Vec3::new(0.23, 0.14, 0.08), // outline brown
            Vec3::new(0.16, 0.15, 0.18), // black fur
            Vec3::new(0.56, 0.75, 0.34), // grass
            Vec3::ONE,
        ] {
            for lit in [0.0, 1.0] {
                let c = night_color(albedo, lit);
                assert!(c.cmpge(NIGHT_FLOOR - 1e-6).all(), "{albedo} {lit}: {c}");
            }
        }
    }

    // NIGHT-012: style frame: grass turns deep blue, the path pale lavender, white stays the lightest.
    #[test]
    fn night_palette_matches_the_style_frame() {
        let grass = night_color(Vec3::new(0.56, 0.75, 0.34), 1.0);
        assert!(grass.z > grass.x && grass.z > grass.y, "blue grass {grass}");
        assert!(luma(grass) < 0.45, "deep {grass}");
        let path = night_color(Vec3::new(0.95, 0.89, 0.80), 1.0);
        assert!(
            luma(path) > 0.55 && path.z > path.x,
            "pale lavender path {path}"
        );
        let white = night_color(Vec3::ONE, 1.0);
        assert!(luma(white) >= luma(path));
        // the shadow side is darker, but still readable
        let shade = night_color(Vec3::new(0.56, 0.75, 0.34), 0.0);
        assert!(luma(shade) < luma(grass) && luma(shade) > 0.2);
    }

    #[test]
    fn grade_blends_day_dusk_night() {
        let a = Vec3::new(0.56, 0.75, 0.34);
        let day = grade(a, 1.0, SHADOW_TINT, DayLight::default());
        assert!(day.abs_diff_eq(a, 1e-6));
        let dusk = grade(
            a,
            1.0,
            SHADOW_TINT,
            DayLight {
                night: 0.0,
                warm: 1.0,
            },
        );
        assert!(dusk.x / dusk.z > a.x / a.z, "dusk is warmer {dusk}");
        let night = grade(
            a,
            1.0,
            SHADOW_TINT,
            DayLight {
                night: 1.0,
                warm: 0.0,
            },
        );
        assert!(night.abs_diff_eq(night_color(a, 1.0), 1e-6));
    }

    #[test]
    fn lamp_light_has_a_hard_edge() {
        assert_eq!(light_band(0.0, 3.0, 0.05), 1.0);
        assert_eq!(light_band(2.9, 3.0, 0.05), 1.0);
        assert_eq!(light_band(3.0, 3.0, 0.05), 0.0);
        assert_eq!(light_band(5.0, 3.0, 0.05), 0.0);
    }

    // NIGHT-012, Q-114: nearest lamps become point lights, the next ones light pools.
    #[test]
    fn nearest_lamps_get_point_lights() {
        let lamps: Vec<PointLight> = (0..40)
            .map(|i| PointLight {
                pos: Vec3::new(i as f32 * 5.0, 0.0, 0.0),
                radius: 3.0,
                color: LAMP_LIGHT,
                strength: 1.0,
                tinted: false,
            })
            .collect();
        let (mut order, mut lights, mut pools) = (Vec::new(), Vec::new(), Vec::new());
        pick_lamps(
            &lamps,
            Vec3::ZERO,
            1000.0,
            1,
            &mut order,
            &mut lights,
            &mut pools,
        );
        assert_eq!(lights.len(), MAX_POINT_LIGHTS - 1);
        assert_eq!(pools.len(), MAX_LIGHT_POOLS);
        assert!(lights.iter().all(|l| l.pos.x <= 35.0));
        assert!(pools.iter().all(|l| l.pos.x >= 40.0));
        // far lamps are not lit at all
        lights.clear();
        pools.clear();
        pick_lamps(
            &lamps,
            Vec3::ZERO,
            LAMP_CULL_M,
            1,
            &mut order,
            &mut lights,
            &mut pools,
        );
        assert_eq!(lights.len() + pools.len(), 5);
    }

    #[test]
    fn glsl_declares_the_night_uniforms() {
        let s = night_glsl();
        for u in [
            "u_night",
            "u_lights",
            "u_light_colors",
            "u_pools",
            "u_light_mask",
            "vec3 shade(",
        ] {
            assert!(s.contains(u), "{u}");
        }
    }

    // PERF-017 (PERF-R-001): the light loops walk the per-draw mask (uniform control flow)
    // and skip a light outside its radius before the band; the edge width is fwidth(d) taken
    // before the skip (the unchanged anti-aliasing of the old shader).
    #[test]
    fn perf_017_light_loops_skip_far_lights_after_the_derivative() {
        let s = night_glsl();
        let body = &s[s.find("vec3 shade(").unwrap()..];
        let loops = &body[body.find("for (int i").unwrap()..];
        assert_eq!(loops.matches("if (bits == 0u) break;").count(), 2);
        for skip in ["if (d >= L.w) continue;", "if (d >= P.z) continue;"] {
            let at = loops.find(skip).unwrap_or_else(|| panic!("{skip}"));
            let before = &loops[..at];
            let fw = before.rfind("fwidth(d)").expect("fwidth before the skip");
            assert!(!before[fw..].contains("light_band("), "{skip}");
        }
        assert!(!loops.contains("u_light_count") && !loops.contains("u_pool_count"));
    }

    fn lamp_floats(ls: &[(Vec3, f32)]) -> Vec<[f32; 4]> {
        ls.iter().map(|(p, r)| [p.x, p.y, p.z, *r]).collect()
    }

    /// Light pools as uniform floats: `(x, z, _)` and radius → x, z, radius, strength 1.
    fn pool_floats(ps: &[(Vec3, f32)]) -> Vec<[f32; 4]> {
        ps.iter().map(|(p, r)| [p.x, p.y, *r, 1.0]).collect()
    }

    // PERF-017 (PERF-R-001): a light whose bit is not set lights no point of the box: for
    // many boxes and lights, every sampled point with a non-zero band has its light's bit.
    #[test]
    fn perf_017_light_mask_is_conservative() {
        let mut seed = 17u32;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % 10_000) as f32 / 10_000.0
        };
        let mut set_bits = 0;
        for _ in 0..300 {
            let min = Vec3::new(rnd() * 40.0 - 20.0, rnd() * 2.0 - 1.0, rnd() * 40.0 - 20.0);
            let max = min + Vec3::new(rnd() * 8.0, rnd() * 5.0, rnd() * 8.0);
            let lights: Vec<(Vec3, f32)> = (0..MAX_POINT_LIGHTS)
                .map(|_| {
                    let p = Vec3::new(rnd() * 50.0 - 25.0, rnd() * 4.0, rnd() * 50.0 - 25.0);
                    (p, 1.0 + rnd() * 5.0)
                })
                .collect();
            let pools: Vec<(Vec3, f32)> = (0..MAX_LIGHT_POOLS)
                .map(|_| {
                    let p = Vec3::new(rnd() * 50.0 - 25.0, rnd() * 50.0 - 25.0, 0.0);
                    (p, 1.0 + rnd() * 4.0)
                })
                .collect();
            let (lm, pm) = light_mask(&lamp_floats(&lights), &pool_floats(&pools), min, max);
            set_bits += lm.count_ones() + pm.count_ones();
            for _ in 0..200 {
                let q = min + (max - min) * Vec3::new(rnd(), rnd(), rnd());
                for (i, (c, r)) in lights.iter().enumerate() {
                    if light_band(q.distance(*c), *r, 0.02) > 0.0 {
                        assert!(lm & (1 << i) != 0, "light {i} reaches {q} in {min}..{max}");
                    }
                }
                if q.y < POOL_MAX_Y {
                    for (i, (c, r)) in pools.iter().enumerate() {
                        let d = glam::Vec2::new(q.x, q.z).distance(c.truncate());
                        if light_band(d, *r, 0.02) > 0.0 {
                            assert!(pm & (1 << i) != 0, "pool {i} reaches {q}");
                        }
                    }
                }
            }
        }
        assert!(set_bits > 100, "the test boxes are lit at all");
    }

    // PERF-017: far lights are masked out; a box above the pool height gets no pools.
    #[test]
    fn perf_017_light_mask_drops_far_lights() {
        let lights = lamp_floats(&[
            (Vec3::new(0.0, 1.0, 0.0), 3.0),
            (Vec3::new(10.0, 1.0, 0.0), 3.0),
            (Vec3::new(3.5, 1.0, 0.0), 3.0),
        ]);
        let pools = pool_floats(&[
            (Vec3::new(2.5, 0.0, 0.0), 2.0),
            (Vec3::new(20.0, 0.0, 0.0), 2.0),
        ]);
        let (min, max) = (Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 2.0, 1.0));
        assert_eq!(light_mask(&lights, &pools, min, max), (0b101, 0b1));
        let up = Vec3::Y * 1.0;
        assert_eq!(light_mask(&lights, &pools, min + up, max + up), (0b101, 0));
        assert_eq!(full_light_mask(3, 24), (0b111, 0xFF_FFFF));
        assert_eq!(full_light_mask(0, 0), (0, 0));
    }

    #[test]
    fn eyeshine_is_never_red() {
        const { assert!(EYE_GLOW.y > EYE_GLOW.x * 0.9 && EYE_GLOW.y > EYE_GLOW.z) };
    }
}
