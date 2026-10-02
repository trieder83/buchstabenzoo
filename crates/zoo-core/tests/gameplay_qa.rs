//! Gameplay QA regression tests (qa/reports/2026-09-26-poc-m4-gameplay.md): whole-level
//! sweeps of collision (GAME-PLAYER §7), interaction availability (§5), following animals
//! (GAME-RESCUE §6) and text completeness of everything a child can interact with.

mod common;

use glam::{IVec2, Vec2};
use zoo_core::animals::AnimalState;
use zoo_core::collision::{Shape, PLAYER_RADIUS_M};
use zoo_core::content::{riddle_key, Language, ReadingLevel};
use zoo_core::food::FoodBox;
use zoo_core::game::Target;
use zoo_core::level::{cell_center, cell_of, Level};
use zoo_core::nav::Autopilot;
use zoo_core::player::{MoveParams, Player};
use zoo_core::Food;

const DT: f32 = 1.0 / 60.0;

/// Deepest overlap (m) of a circle of the player radius at `p` with blocked cells (gates
/// passable with `gates`) and prop shapes; 0 = free.
fn penetration(level: &Level, p: Vec2, gates: bool) -> f32 {
    let r = PLAYER_RADIUS_M;
    let mut worst: f32 = 0.0;
    for dz in -1..=1 {
        for dx in -1..=1 {
            let c = cell_of(p) + IVec2::new(dx, dz);
            if !level.grid().is_walkable(c, gates) {
                worst = worst.max(Shape::cell(c).push_out(p, r).length());
            }
        }
    }
    for s in level.colliders().shapes() {
        worst = worst.max(s.push_out(p, r).length());
    }
    worst
}

fn unit(deg: f32) -> Vec2 {
    let a = deg.to_radians();
    Vec2::new(a.cos(), a.sin())
}

// PLAY-031: from every free walkable cell centre, walking 3 s in each of 8 directions never
// overlaps a solid cell or prop (1 mm slack) and never leaves the player trapped.
#[test]
fn play_031_collision_sweep_whole_level() {
    let level = Level::new(common::level1());
    let params = MoveParams::default();
    let mut starts = 0;
    for c in level.data.level.bounds.cells() {
        if !level.grid().is_walkable(c, false) {
            continue;
        }
        let s = cell_center(c);
        if penetration(&level, s, false) > 0.0 {
            continue; // cell centre covered by a prop (e.g. under the food box row)
        }
        starts += 1;
        for k in 0..8 {
            let dir = unit(k as f32 * 45.0 + 10.0);
            let mut p = Player::new(s, dir, &params);
            for _ in 0..180 {
                p.step_with(level.grid(), level.colliders(), &params, dir, DT, false);
                let e = penetration(&level, p.pos, false);
                assert!(
                    e < 1e-3,
                    "overlap {e:.4} m at {} (from {s}, dir {dir})",
                    p.pos
                );
            }
            let end = p.pos;
            let free = (0..8).any(|j| {
                let d = unit(j as f32 * 45.0);
                let mut q = p.clone();
                for _ in 0..20 {
                    q.step_with(level.grid(), level.colliders(), &params, d, DT, false);
                }
                q.pos.distance(end) > 0.15
            });
            assert!(free, "trapped at {end} (from {s}, dir {dir})");
        }
    }
    assert!(starts > 1000, "only {starts} start cells");
}

// PLAY-031: pushing diagonally into walls, props and corners does not jitter (no visible
// back-and-forth of more than 2 cm between frames while held against an obstacle).
#[test]
fn play_031_no_jitter_against_obstacles() {
    let level = Level::new(common::level1());
    let params = MoveParams::default();
    // (start, direction): zebra board, food box row, bench, bridge rail, jetty end, fence corner
    let cases = [
        (Vec2::new(-6.5, 15.5), Vec2::new(-1.0, 0.3)),
        (Vec2::new(-0.4, 9.2), Vec2::new(0.4, 1.0)),
        (Vec2::new(7.0, 2.5), Vec2::new(-0.3, 1.0)),
        (Vec2::new(11.5, 29.5), Vec2::new(0.5, 1.0)),
        (Vec2::new(-9.5, 23.0), Vec2::new(-1.0, -0.2)),
        (Vec2::new(-7.0, 20.0), Vec2::new(-1.0, -1.0)),
    ];
    for (start, dir) in cases {
        let dir = dir.normalize();
        let mut p = Player::new(start, dir, &params);
        let mut hist = Vec::new();
        for f in 0..240 {
            p.step_with(level.grid(), level.colliders(), &params, dir, DT, false);
            if f >= 180 {
                hist.push(p.pos);
            }
        }
        for w in hist.windows(3) {
            let (a, b) = (w[1] - w[0], w[2] - w[1]);
            if a.dot(b) < 0.0 {
                assert!(
                    a.length().min(b.length()) < 0.02,
                    "jitter at {} from {start}",
                    w[1]
                );
            }
        }
    }
}

// PLAY-020 (whole level): for every info board and food box, at 0.5–2.5 m in 0.25 m steps,
// every 15° around the readable side and 16 facings, availability equals the rule of
// GAME-PLAYER §5 (≤ 2 m, readable side ±60°, facing ±75°); every one of them can be reached
// and used from a free standing position.
#[test]
fn play_020_availability_sweep_all_boards_and_boxes() {
    let mut g = common::game(1);
    let targets: Vec<_> = g
        .interactables()
        .into_iter()
        .filter(|i| i.readable.is_some())
        .collect();
    // (the 4 garden signs of `garden_veg` are read like boards, GAME-GARDEN 1, GARD-009;
    // 16 food boxes = 10 outside + 6 inside the storage, Q-194 answered 2026-09-29;
    // the entrance map board is read like a board too, RESC-028)
    assert_eq!(
        targets.len(),
        3 + 16 + 4 + 1,
        "3 info boards, 16 food boxes, 4 garden signs, 1 welcome board"
    );
    let ang = |u: Vec2, v: Vec2| {
        u.normalize()
            .dot(v.normalize())
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees()
    };
    for it in &targets {
        let f = it.readable.unwrap();
        let mut usable = 0;
        for di in 2..=10 {
            let d = di as f32 * 0.25;
            for ai in -12..=12 {
                let side = ai as f32 * 15.0;
                let dir = Vec2::from_angle(side.to_radians()).rotate(f);
                let pos = it.point + dir * d;
                for fi in 0..16 {
                    let facing = unit(fi as f32 * 22.5);
                    g.player.pos = pos;
                    g.player.facing = facing;
                    let face_angle = ang(facing, it.point - pos);
                    // skip samples exactly on a limit (float noise)
                    if (d - 2.0).abs() < 1e-3
                        || (side.abs() - 60.0).abs() < 1e-3
                        || (face_angle - 75.0).abs() < 0.01
                    {
                        continue;
                    }
                    let expected = d <= 2.0 && side.abs() <= 60.0 && face_angle <= 75.0;
                    assert_eq!(
                        g.is_available(it),
                        expected,
                        "{:?}: d {d} side {side}° facing {face_angle:.1}°",
                        it.target
                    );
                    if expected && penetration(&g.level, pos, false) == 0.0 {
                        usable += 1;
                    }
                }
            }
        }
        assert!(
            usable >= 100,
            "{:?} usable from only {usable} samples",
            it.target
        );
    }
}

// PLAY-020: the prompt is offered for the board the child faces in front of it — also when
// she walks up against the board (collision stop) from straight ahead or at 30°.
#[test]
fn play_020_walk_up_to_each_board_gives_the_prompt() {
    let mut g = common::game(1);
    let boards: Vec<_> = g
        .interactables()
        .into_iter()
        .filter(|i| matches!(i.target, Target::InfoBoard { .. }))
        .collect();
    for it in boards {
        let f = it.readable.unwrap();
        for side in [-30.0f32, 0.0, 30.0] {
            let start = it.point + Vec2::from_angle(side.to_radians()).rotate(f) * 3.0;
            g.player.pos = start;
            for _ in 0..240 {
                let to = (it.point - g.player.pos).normalize();
                g.update(DT, to);
            }
            assert!(
                g.player.pos.distance(it.point) < 1.5,
                "{:?}: stopped too far at {}",
                it.target,
                g.player.pos
            );
            assert_eq!(
                g.available_target(),
                Some(it.target.clone()),
                "{:?} from {side}°: stopped at {}",
                it.target,
                g.player.pos
            );
        }
    }
}

// RESC-006: a following zebra led on a tour around the ring (bridge, pond, food storage,
// zebra gate) never stands inside a solid cell or prop and never gets lost.
#[test]
fn resc_006_follower_walks_around_obstacles() {
    let mut g = common::game(1);
    g.carry.take(&FoodBox { food: Food::Grass });
    let z = g.animal("zebra").unwrap().pos;
    g.player.pos = z + Vec2::new(-1.3, 0.0);
    g.player.facing = Vec2::X;
    g.show_food("zebra").unwrap();
    assert_eq!(g.animal("zebra").unwrap().state, AnimalState::Following);
    for target in [
        Vec2::new(11.5, 29.5),
        Vec2::new(0.0, 28.5),
        Vec2::new(-6.5, 22.5),
        Vec2::new(-6.5, 9.5),
        Vec2::new(3.5, 9.5),
        Vec2::new(-8.4, 12.0),
    ] {
        let mut ap = Autopilot::new(target);
        for _ in 0..(60 * 60) {
            let Some(input) = ap.input(g.level.grid(), g.player.pos, g.is_leading()) else {
                break;
            };
            g.update(DT, input);
            let a = g.animal("zebra").unwrap();
            let e = penetration(&g.level, a.pos, true);
            assert!(
                e < 1e-3,
                "zebra overlaps a blocker by {e:.3} m at {}",
                a.pos
            );
        }
        // the player stops at the target; the zebra catches up (keeps 1.5 m)
        for _ in 0..90 {
            g.update(DT, Vec2::ZERO);
        }
        let a = g.animal("zebra").unwrap();
        assert_eq!(a.state, AnimalState::Following);
        assert!(!a.waiting, "zebra fell behind on the way to {target}");
        assert!(
            a.pos.distance(g.player.pos) < 3.0,
            "zebra lost at {}",
            a.pos
        );
    }
}

// RESC-017, RESC-025 (only missions in `[level] missions` are interactable): every animal and info board that level 1 makes interactable has its texts
// (location riddle for every reading level, home message, name, facts) in de and en, so no
// panel, bubble or celebration ever shows a raw Fluent key (Q-069 answered: all three
// level-1 missions are in scope). Checked for every candidate place (any seed).
#[test]
fn resc_017_every_interactable_has_texts() {
    let g = common::game(1);
    let c = common::content();
    assert_eq!(
        g.animals.len(),
        6,
        "zebra, hippo and panda pairs (FAM-001, Q-308) are in scope"
    );
    let mut animals: Vec<&str> = g
        .interactables()
        .iter()
        .filter_map(|i| match i.target {
            Target::InfoBoard { animal } | Target::Animal { animal } => Some(animal),
            _ => None,
        })
        .collect();
    animals.sort_unstable();
    animals.dedup();
    let mut missing = Vec::new();
    assert_eq!(animals, ["hippo", "panda", "zebra"]);
    for animal in animals {
        let places: Vec<String> = g
            .level
            .data
            .hiding_places_of(animal)
            .map(|h| h.id.clone())
            .collect();
        for lang in Language::ALL {
            let mut keys: Vec<String> = places
                .iter()
                .flat_map(|p| {
                    ReadingLevel::ALL
                        .iter()
                        .map(move |&l| riddle_key(animal, p, l))
                })
                .collect();
            keys.push(format!("mission-{animal}-home"));
            keys.push(format!("animal-{animal}"));
            keys.push(format!("animal-{animal}-more"));
            for l in ReadingLevel::ALL {
                keys.push(zoo_core::content::facts_key(animal, l));
            }
            for k in keys {
                if c.text(lang, &k).is_none_or(|t| t.trim().is_empty()) {
                    missing.push(format!("{}:{k}", lang.id()));
                }
            }
        }
    }
    assert!(missing.is_empty(), "missing texts: {missing:?}");
}
