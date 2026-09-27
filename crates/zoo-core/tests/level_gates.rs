//! LAYOUT-036 (GAME-LAYOUT "Gates between the levels", user request 2026-09-27): every level
//! transition has a real `gate_zoo` in the boundary hedge/wall line across its entry path —
//! closed and solid while the level is locked (the story barrier in front of it on the old
//! level's side), open, walkable both ways and drawn open after unlocking.

mod common;

use glam::Vec2;
use zoo_core::collision::PLAYER_RADIUS_M;
use zoo_core::level::cell_of;
use zoo_core::scene::{level_gate_pose, LevelScene, OpeningKind};
use zoo_core::Game;

/// (barrier, entry id) of every level transition except the moon door.
fn transitions(data: &zoo_core::LevelData) -> Vec<(String, String)> {
    data.parts
        .iter()
        .flat_map(|p| p.entries.iter())
        .filter(|en| {
            data.element(&en.barrier)
                .is_some_and(|b| b.kind.as_deref() != Some("moon_door"))
        })
        .map(|en| (en.barrier.clone(), en.id.clone()))
        .collect()
}

/// Walks with a constant input for `secs` (level direction) and returns where she ends.
fn walk(g: &mut Game, from: Vec2, dir: Vec2, secs: f32) -> Vec2 {
    g.player.pos = from;
    g.player.facing = dir;
    for _ in 0..(secs * 60.0) as usize {
        g.update(1.0 / 60.0, dir);
    }
    g.player.pos
}

// LAYOUT-036
#[test]
fn layout_036_level_gates_closed_while_locked_open_after() {
    let data = common::zoo_with_night();
    let ts = transitions(&data);
    let barriers: Vec<&str> = ts.iter().map(|(b, _)| b.as_str()).collect();
    for want in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        assert!(barriers.contains(&want), "{want}: no level transition");
    }
    let s = LevelScene::build(&data);
    let gates: Vec<_> = s
        .openings
        .iter()
        .filter_map(|o| match &o.kind {
            OpeningKind::LevelGate { barrier } => Some((barrier.clone(), o)),
            _ => None,
        })
        .collect();
    assert_eq!(gates.len(), ts.len(), "one gate per transition");
    // the gate barrier itself is not drawn any more (the gate replaces it)
    assert!(!s.placements.iter().any(|p| p.model == "gate_zoo_closed"));

    for (barrier, entry) in &ts {
        let en = data
            .parts
            .iter()
            .flat_map(|p| p.entries.iter())
            .find(|e| &e.id == entry)
            .unwrap();
        let (_, o) = gates
            .iter()
            .find(|(b, _)| b == barrier)
            .unwrap_or_else(|| panic!("{barrier}: no level gate"));
        assert_eq!(s.placements[o.placement].model, "gate_zoo", "{barrier}");
        let (c, _, dir) = level_gate_pose(en, &data).unwrap();
        assert!(o.center.distance(c) < 1e-4);
        let to_old = dir.offset().as_vec2();
        let along = to_old.perp();

        let mut g = common::night_game(1);
        // posts touching the hedge / wall on both sides: just outside the pillars is solid
        for side in [-1.0f32, 1.0] {
            let p = c + along * side * 1.6;
            assert!(
                !g.level.grid().is_walkable(cell_of(p), true),
                "{barrier}: gap beside the gate post at {p}"
            );
        }
        // the gate stands on the new level's side of (or replaces) the story barrier
        let b = data.element(barrier).unwrap();
        let bc = Vec2::new(
            b.rect.x as f32 + b.rect.w as f32 / 2.0,
            b.rect.z as f32 + b.rect.d as f32 / 2.0,
        );
        // (`barrier_north_gate` *is* the gate: it stands on the barrier's old-level row)
        if b.kind.as_deref() != Some("closed_gate") {
            assert!(
                (bc - c).dot(to_old) > 0.5,
                "{barrier}: barrier behind the gate"
            );
        } else {
            assert!(
                b.rect.contains(cell_of(c)),
                "{barrier}: gate outside its band"
            );
        }

        // locked: closed and solid (she cannot walk from the new level side through it)
        let o_now = g.level.openings().iter().find(|x| x.center == c).unwrap();
        assert!(!g.opening_open(o_now), "{barrier}: open while locked");
        assert!(
            g.level.colliders().overlaps(c, PLAYER_RADIUS_M),
            "{barrier}: closed leaves not solid"
        );
        let inside = c - to_old * 1.2;
        let end = walk(&mut g, inside, to_old, 2.0);
        assert!(
            (end - c).dot(to_old) < 0.0,
            "{barrier}: walked through the closed gate to {end}"
        );

        // unlocked: the barrier is gone, the gate open and walkable both ways
        assert!(g.level.open_barrier(barrier));
        let o_now = g.level.openings().iter().find(|x| x.center == c).unwrap();
        assert!(g.opening_open(o_now), "{barrier}: not open after unlocking");
        assert!(!g.level.colliders().overlaps(c, PLAYER_RADIUS_M));
        let out = walk(&mut g, inside, to_old, 4.0);
        assert!(
            (out - c).dot(to_old) > 1.5,
            "{barrier}: stuck going to the old level at {out}"
        );
        let back = walk(&mut g, c + to_old * 2.0, -to_old, 4.0);
        assert!(
            (back - c).dot(-to_old) > 1.5,
            "{barrier}: stuck going to the new level at {back}"
        );
        // the open walkway in front of and behind the gate is free of colliders
        for d in [0.6f32, 1.0, 1.4] {
            for t in [-0.2f32, 0.0, 0.2] {
                for side in [1.0f32, -1.0] {
                    let p = c + to_old * side * d + along * t;
                    assert!(
                        !g.level.colliders().overlaps(p, PLAYER_RADIUS_M * 0.9)
                            && g.level.grid().is_walkable(cell_of(p), true),
                        "{barrier}: open walkway blocked at {p}"
                    );
                }
            }
        }
    }
}
