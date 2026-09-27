//! LAYOUT-L3-017: the mill hut's water wheel runs in the water (GAME-LEVEL-3
//! `loc_water_wheel`, user request 2026-09-27).

mod common;

use glam::Vec2;
use zoo_core::collision::PLAYER_RADIUS_M;
use zoo_core::ground::WATER_TOP_M;
use zoo_core::level::cell_of;
use zoo_core::scene::{LevelScene, WATER_WHEEL_PADDLES};

// LAYOUT-L3-017
#[test]
fn layout_l3_017_water_wheel_turns_in_the_stream() {
    let data = common::zoo();
    let scene = LevelScene::build(&data);
    assert_eq!(
        scene.water_wheels.len(),
        1,
        "the only water wheel in the zoo"
    );
    let w = scene.water_wheels[0];
    let stream = data.element("stream_l3").unwrap().rect;
    let flow =
        zoo_core::water::flow_dir(data.element("stream_l3").unwrap().flow.as_deref().unwrap())
            .expect("stream flow");

    // the lowest paddles dip ≥ 0.25 × radius below the water surface, inside stream cells
    let mut lowest = f32::MAX;
    for step in 0..120 {
        let t = step as f64 * 0.05;
        for k in 0..WATER_WHEEL_PADDLES {
            let (p, y) = w.paddle_tip(k, t);
            lowest = lowest.min(y);
            if y < WATER_TOP_M {
                assert!(
                    stream.contains(cell_of(p)),
                    "paddle under water outside the stream"
                );
                assert!(
                    scene.water.is_water(p),
                    "paddle under water at {p:?} is in water"
                );
            }
        }
    }
    assert!(
        WATER_TOP_M - lowest >= 0.25 * w.radius - 1e-4,
        "lowest paddle {lowest} m, radius {}",
        w.radius
    );
    for dx in [-w.width / 2.0, w.width / 2.0] {
        let p = w.center + Vec2::new(dx, 0.0);
        assert!(scene.water.is_water(p), "wheel edge {p:?} in the water");
    }

    // turns continuously (≈ 60°/s), with the flow at the bottom — by day and at night alike
    // (the angle depends on play time only)
    let a0 = w.angle_at(0.0);
    let a1 = w.angle_at(0.5);
    let deg_per_s = (a1 - a0).abs().to_degrees() / 0.5;
    assert!((50.0..=70.0).contains(&deg_per_s), "{deg_per_s}°/s");
    for t in [0.0, 13.3, 600.0, 3600.0] {
        // the paddle at the bottom moves with the flow
        let bottom = (0..WATER_WHEEL_PADDLES)
            .min_by(|&a, &b| w.paddle_tip(a, t).1.total_cmp(&w.paddle_tip(b, t).1))
            .unwrap();
        let (p0, _) = w.paddle_tip(bottom, t);
        let (p1, _) = w.paddle_tip(bottom, t + 0.05);
        assert!(
            (p1 - p0).dot(flow) > 0.0,
            "t {t}: bottom paddle against the flow"
        );
        assert!(
            (w.angle_at(t + 0.1) - w.angle_at(t)).abs() > 0.01,
            "t {t}: turning"
        );
    }

    // not solid for the player
    let g = common::zoo_game(1);
    for k in 0..WATER_WHEEL_PADDLES {
        let (p, _) = w.paddle_tip(k, 0.0);
        assert!(
            !g.level.colliders().overlaps(p, PLAYER_RADIUS_M),
            "no collider at the wheel"
        );
    }
    assert!(!g.level.colliders().overlaps(w.center, 0.3));

    // foam where the paddles meet the water
    for q in w.water_points() {
        assert!(
            scene
                .water
                .obstacles
                .iter()
                .any(|o| o.pos.distance(q) < 0.1),
            "foam obstacle at {q:?}"
        );
    }
}
