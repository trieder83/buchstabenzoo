//! TECH-WATER unit tests: river centrelines, the baked water field, the loop clock, the
//! pattern mirrors and the bobbing (WATER-001, 002, 004, 005, 009; AENV-007/008).

mod common;

use glam::{Vec2, Vec3};
use zoo_core::level::cell_of;
use zoo_core::scene::LevelScene;
use zoo_core::water::{
    bob, bob_params, bob_transform, pond_ring_mask, river_streak_mask, streak_lane, water_time,
    BobParams, WaterField, WATER_LOOP_S, WATER_PERIODS_S,
};

fn level1_field() -> (LevelScene, WaterField) {
    let scene = LevelScene::build(&common::level1());
    let field = WaterField::bake(&scene.water);
    (scene, field)
}

/// Level point of a field texel.
fn level_of(field: &WaterField, i: u32, j: u32) -> Vec2 {
    let w = field.texel_world(i, j);
    Vec2::new(w.x, -w.y)
}

// WATER-001 (AENV-007 direction): s grows along the flow of each river element and is
// continuous everywhere on the river, through the bridge and the bend.
#[test]
fn water_001_s_follows_the_flow_and_is_continuous() {
    let (scene, field) = level1_field();
    let data = common::level1();
    let texel = 1.0 / field.texels_per_m as f32;
    let river = |i: u32, j: u32| {
        let p = level_of(&field, i, j);
        field.texel(i, j)[3] > 0.5 && scene.water.is_river_water(p)
    };
    // Inside a bend s is an arc length at the centreline: at radius ρ from the arc centre
    // (inner corner) one metre across the flow is r/ρ metres of s (the inner side is slower).
    let path = &scene.water.rivers[0];
    let arcs: Vec<(Vec2, f32)> = path
        .pieces
        .iter()
        .filter_map(|p| match *p {
            zoo_core::water::PathPiece::Arc { center, r, .. } => Some((center, r)),
            _ => None,
        })
        .collect();
    let metric = |p: Vec2| {
        arcs.iter()
            .filter(|(c, r)| p.distance(*c) < 2.0 * r)
            .map(|(c, r)| (r / p.distance(*c).max(0.05)).max(1.0))
            .fold(1.0f32, f32::max)
    };
    let mut checked = 0;
    for j in 0..field.height - 1 {
        for i in 0..field.width - 1 {
            if !river(i, j) {
                continue;
            }
            let s = field.texel(i, j)[0];
            for (di, dj) in [(1, 0), (0, 1)] {
                if river(i + di, j + dj) {
                    let s2 = field.texel(i + di, j + dj)[0];
                    let p = level_of(&field, i, j);
                    let q = level_of(&field, i + di, j + dj);
                    assert!(
                        (s2 - s).abs() <= 1.6 * texel * metric(p).max(metric(q)) + 1e-4,
                        "s jumps at {:?}: {s} → {s2}",
                        level_of(&field, i, j)
                    );
                    checked += 1;
                }
            }
            // gradient direction inside straight pieces
            let p = level_of(&field, i, j);
            let c = cell_of(p);
            for (id, want) in [("river_n", Vec2::NEG_Y), ("river_e", Vec2::X)] {
                let e = data.element(id).unwrap();
                let bend = id == "river_e" && c.x < e.rect.x + 3;
                if e.rect.contains(c) && !bend && river(i + 1, j) && river(i, j + 1) {
                    // world texel steps: +i = level east, +j = level south
                    let gx = field.texel(i + 1, j)[0] - s;
                    let gz = -(field.texel(i, j + 1)[0] - s);
                    let g = Vec2::new(gx, gz).normalize();
                    assert!(g.dot(want) > 0.99, "{id} at {p:?}: gradient {g:?}");
                }
            }
        }
    }
    assert!(checked > 1000, "river texels checked: {checked}");
    // monotonic through the bend: along the centreline s increases
    assert_eq!(
        path.ids,
        ["river_n", "bridge_river", "river_mid", "river_e"]
    );
    let mut last = f32::MIN;
    for k in 0..200 {
        let s = k as f32 * path.length() / 200.0;
        let p = path.point(s, 0.0);
        let (s2, c2) = path.coords(p);
        assert!(
            (s2 - s).abs() < 1e-3 && c2.abs() < 1e-3,
            "round trip at s {s}"
        );
        assert!(s2 > last);
        last = s2;
        // flow direction rotates from south to east through the bend
        let d = path.dir_at(s);
        assert!(d.dot(Vec2::NEG_Y) > -1e-4 && d.dot(Vec2::X) > -1e-4);
    }
}

// WATER-002: c is continuous, bounded by the river width, and the flow flag separates river
// and pond.
#[test]
fn water_002_c_continuous_and_flow_flags() {
    let (scene, field) = level1_field();
    let data = common::level1();
    let texel = 1.0 / field.texels_per_m as f32;
    let half = scene.water.rivers[0].half_width;
    let bend = data.element("river_e").unwrap().rect;
    let (mut river_n, mut pond_n) = (0, 0);
    for j in 0..field.height - 1 {
        for i in 0..field.width - 1 {
            let p = level_of(&field, i, j);
            let t = field.texel(i, j);
            let Some(cell) = scene.water.cell(cell_of(p)) else {
                continue;
            };
            if !cell.in_water(p) {
                continue;
            }
            if cell.river {
                river_n += 1;
                assert_eq!(t[3], 1.0, "river flag at {p:?}");
                let c = cell_of(p);
                let in_bend_square = c.x < bend.x + 3 && bend.contains(c);
                let bound = if in_bend_square {
                    // outer corner of the bend square (rounded tile): up to (√2 − 1)·w + w/2
                    (2f32.sqrt() - 1.0) * 2.0 * half + half
                } else {
                    half
                };
                assert!(t[1].abs() <= bound + 1e-3, "|c| {} at {p:?}", t[1]);
                for (di, dj) in [(1, 0), (0, 1)] {
                    let q = level_of(&field, i + di, j + dj);
                    if scene.water.is_river_water(q) {
                        let c2 = field.texel(i + di, j + dj)[1];
                        assert!((c2 - t[1]).abs() <= 1.6 * texel + 1e-4, "c jumps at {p:?}");
                    }
                }
            } else {
                pond_n += 1;
                assert_eq!(t[3], 0.0, "pond flag at {p:?}");
                assert_eq!((t[0], t[1]), (0.0, 0.0));
            }
            assert!(t[2] >= -1e-4, "shore ≥ 0 in the water at {p:?}");
        }
    }
    assert!(river_n > 500 && pond_n > 500, "{river_n} / {pond_n}");
}

// WATER-002 (joined zoo): the level-3 stream flows south, pools and the fountain are still.
#[test]
fn water_002_zoo_stream_pools_and_fountain() {
    let scene = LevelScene::build(&common::zoo());
    let field = WaterField::bake(&scene.water);
    assert_eq!(scene.water.rivers.len(), 2);
    let stream = scene
        .water
        .rivers
        .iter()
        .find(|r| r.ids == ["stream_l3"])
        .expect("stream");
    assert!(stream.dir_at(3.0).abs_diff_eq(Vec2::NEG_Y, 1e-5));
    let data = common::zoo();
    let f = data.element("fountain_sw").unwrap().rect;
    let c = Vec2::new(f.x as f32 + 1.5, f.z as f32 + 1.2);
    let t = field.sample(Vec2::new(c.x, -c.y));
    assert_eq!(t[3], 0.0);
    assert!(t[2] > 0.5, "fountain water shore {}", t[2]);
    // obstacles (Q-068): bridge piles, rapids stones, jetty posts, water wheel, fountain jet
    let obs = &scene.water.obstacles;
    assert!(obs.iter().filter(|o| o.river).count() >= 8, "{obs:?}");
    assert!(obs.iter().any(|o| !o.river && o.pos.distance(c) < 0.5));
}

// WATER-004 (AENV-008): patterns are functions of time only and loop in 16 s.
#[test]
fn water_004_patterns_loop_and_ignore_the_frame_rate() {
    for k in 0..400 {
        let s = k as f32 * 0.137;
        let c = (k as f32 * 0.071).sin() * 1.1;
        let p = Vec2::new(k as f32 * 0.31 - 20.0, (k as f32 * 0.17).cos() * 9.0);
        for t0 in [0.3f64, 5.25, 11.9, 100.7] {
            let a = water_time(t0);
            let b = water_time(t0 + WATER_LOOP_S);
            assert_eq!(
                river_streak_mask(s, c, 1.0, a),
                river_streak_mask(s, c, 1.0, b)
            );
            assert_eq!(pond_ring_mask(p, 1.0, a), pond_ring_mask(p, 1.0, b));
        }
    }
    // same image at the same time whether reached in 60 Hz or 23 Hz steps
    let (mut t60, mut t23) = (0.0f64, 0.0f64);
    for _ in 0..(60 * 23) {
        t60 += 1.0 / 60.0;
    }
    for _ in 0..(23 * 23) {
        t23 += 1.0 / 23.0;
    }
    let (a, b) = (water_time(t60), water_time(t23));
    assert!((a - b).abs() < 1e-4, "{a} vs {b}");
    // the masks do change over time (animated)
    let changed = (0..200).any(|k| {
        let s = k as f32 * 0.05;
        river_streak_mask(s, 0.1, 1.0, 1.0) != river_streak_mask(s, 0.1, 1.0, 1.5)
    });
    assert!(changed);
}

// WATER-005 (AENV-008 loop): every period divides 16 s; lane dash period P = 2 s · V.
#[test]
fn water_005_periods_divide_the_loop() {
    for (name, p) in WATER_PERIODS_S {
        let n = WATER_LOOP_S as f32 / p;
        assert!((n - n.round()).abs() < 1e-6, "{name}: {p} s");
    }
    for li in -6..6 {
        let (v, p) = streak_lane(li as f32);
        assert!((0.5..=0.85).contains(&v), "lane {li}: {v} m/s");
        assert!((p - 2.0 * v).abs() < 1e-6);
    }
}

// WATER-009: bobbing stays within its amplitudes, phases differ by position, a frog on its
// pad shares the pad's phase, other models do not bob, and it loops in 16 s.
#[test]
fn water_009_bobbing() {
    let duck = bob_params("duck");
    let pad = bob_params("lily_pad");
    assert_eq!(bob_params("bench"), BobParams::NONE);
    let a = Vec3::new(11.3, 0.0, -29.4);
    let b = Vec3::new(12.1, 0.0, -30.2);
    let mut differ = false;
    for k in 0..160 {
        let t = k as f32 * 0.1;
        let x = bob(a, duck, t);
        assert!(x.offset.y.abs() <= duck.amp_y + 1e-6);
        assert!(x.roll.abs() <= duck.tilt + 1e-6);
        assert!(Vec2::new(x.offset.x, x.offset.z).length() <= duck.drift + 1e-5);
        let y = bob(b, duck, t);
        differ |= (x.offset.y - y.offset.y).abs() > 1e-3;
        let l = bob(a, duck, t + 16.0);
        assert!((l.offset - x.offset).length() < 1e-4 && (l.roll - x.roll).abs() < 1e-4);
        // frog on a spot of its pad: moves exactly with the pad
        let spot = Vec3::new(0.32, 0.02, -0.12);
        let pb = bob(a, pad, t);
        let on_pad = bob_transform(a, 0.7, spot, pb);
        let expect = bob_transform(a, 0.7, spot, bob(a, bob_params("frog"), t));
        assert!(on_pad.distance(expect) < 1e-6);
        assert_eq!(bob(a, BobParams::NONE, t).offset, Vec3::ZERO);
    }
    assert!(differ, "different positions → different phases");
}
