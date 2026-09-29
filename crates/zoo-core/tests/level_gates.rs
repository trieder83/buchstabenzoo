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

// LAYOUT-040 (GAME-LAYOUT "Gates between the levels", user request 2026-09-28): the street
// continues under every level gate and the moon door (Q-182). For each entry cell of a level
// transition, the cells in a straight line from 2 m inside the new level (behind the gate)
// through the entry cell and every barrier cell to the first cell of the old level are path
// cells (the barrier stands ON the street), walkable path once the barrier is open, and the scene draws a path tile on
// each of them with no edging stone (curb) across the line — no grass gap, curb or seam.
#[test]
fn layout_040_street_continues_under_every_level_gate() {
    let data = common::zoo_with_night();
    // the level gates and the moon door (Q-182 answered 2026-09-28: the street also runs
    // under the moon door, from `path_moon` to night_1's entry path)
    let ts: Vec<(String, String)> = data
        .parts
        .iter()
        .flat_map(|p| p.entries.iter())
        .map(|en| (en.barrier.clone(), en.id.clone()))
        .collect();
    assert!(ts.len() >= 4, "{ts:?}");
    assert!(ts.iter().any(|(b, _)| b == "moon_door"), "{ts:?}");
    let s = LevelScene::build(&data);
    let closed = zoo_core::level::Grid::build(&data, &Default::default());
    let all: std::collections::BTreeSet<String> = ts.iter().map(|(b, _)| b.clone()).collect();
    let open = zoo_core::level::Grid::build(&data, &all);
    let tile_at = |c: glam::IVec2| -> Option<&'static str> {
        let w = zoo_core::coords::level_to_world(zoo_core::level::cell_center(c));
        // the ground tile of the cell (path, plaza or grass)
        s.placements
            .iter()
            .filter(|p| p.model.ends_with("_tile") || p.model.starts_with("path_tile"))
            .find(|p| (p.pos - w).length() < 0.01)
            .map(|p| p.model)
    };
    let curb_between = |a: glam::IVec2, b: glam::IVec2| -> bool {
        let mid = (zoo_core::level::cell_center(a) + zoo_core::level::cell_center(b)) / 2.0;
        s.placements.iter().any(|p| {
            p.model == "path_edge" && zoo_core::coords::world_to_level(p.pos).distance(mid) < 0.2
        })
    };
    let mut bad = Vec::new();
    for (barrier, entry) in &ts {
        let en = data
            .parts
            .iter()
            .flat_map(|p| p.entries.iter())
            .find(|e| &e.id == entry)
            .unwrap();
        let b = data.element(barrier).unwrap();
        // towards the old level: the side of the entry cells the barrier touches
        let to_old = level_gate_pose(en, &data)
            .map(|(_, _, dir)| dir.offset())
            .or_else(|| {
                zoo_core::scene::Dir::ALL
                    .into_iter()
                    .map(|d| d.offset())
                    .find(|&o| en.cells.cells().any(|c| b.rect.contains(c + o)))
            })
            .expect("barrier next to the entry");
        for c in en.cells.cells() {
            // 2 cells inside the new level (the gate row and behind it) … through the barrier …
            // the first cell of the old level
            let mut line = vec![c - to_old * 2, c - to_old, c];
            let mut k = c + to_old;
            while b.rect.contains(k) {
                line.push(k);
                k += to_old;
            }
            line.push(k);
            for (i, &q) in line.iter().enumerate() {
                if !closed.has_path(q) {
                    bad.push(format!("{barrier}: {q} not a path cell"));
                }
                if open.kind(q)
                    != zoo_core::level::CellKind::Walkable(zoo_core::level::Surface::Path)
                {
                    bad.push(format!("{barrier}: {q} not walkable path when open"));
                }
                match tile_at(q) {
                    Some(m) if m.starts_with("path_tile") || m == "plaza_tile" => {}
                    m => bad.push(format!("{barrier}: {q} drawn as {m:?}")),
                }
                if i > 0 && curb_between(line[i - 1], q) {
                    bad.push(format!("{barrier}: curb between {} and {q}", line[i - 1]));
                }
            }
        }
        // the barrier stands on the street: every barrier cell is a path cell
        for q in b.rect.cells() {
            if !closed.has_path(q) {
                bad.push(format!("{barrier}: barrier cell {q} off the street"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
