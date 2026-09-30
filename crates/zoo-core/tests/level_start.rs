//! All animals of a level are active somewhere on it when the level starts
//! (GAME-LAYOUT "Joining levels" / "Level design rules", user request 2026-09-30):
//! LAYOUT-044, LAYOUT-045, ANIM-013 and the data checks of LAYOUT-046 that do not need a
//! street graph.

mod common;

use glam::Vec2;
use zoo_core::animals::AnimalState;
use zoo_core::level::{cell_center, cell_of};
use zoo_core::Game;

const DT: f32 = 1.0 / 60.0;

fn all_zoo() -> Game {
    common::night_game(7)
}

// LAYOUT-044 / ANIM-013: every animal of every level is placed at start, `escaped`, standing
// inside the wander area of exactly one chosen hiding place of its own.
#[test]
fn layout_044_anim_013_every_animal_active_at_level_start() {
    for seed in 0..25u64 {
        let g = common::night_game(seed);
        let data = &g.level.data;
        for (k, part) in data.parts.iter().enumerate() {
            for mission in &part.missions {
                let members: Vec<_> = g
                    .animals
                    .iter()
                    .filter(|a| a.id() == mission && a.part == k)
                    .collect();
                assert!(
                    !members.is_empty(),
                    "seed {seed}: {mission} missing in {}",
                    part.id
                );
                for a in members {
                    assert_eq!(a.state, AnimalState::Escaped, "{mission}");
                    assert!(!a.waiting && !a.refusing);
                    let place = data
                        .hiding_place(&a.hiding_place)
                        .expect("chosen place exists");
                    assert_eq!(place.animal, a.id(), "own place");
                    assert!(
                        a.wander_area().contains(cell_of(a.pos)),
                        "seed {seed}: {mission} stands outside its wander area at {:?}",
                        a.pos
                    );
                }
            }
        }
    }
}

// ANIM-013: unlocked animals are simulated from the first frame and stay in their area.
#[test]
fn anim_013_unlocked_animals_are_simulated() {
    let mut g = all_zoo();
    g.player.pos = cell_center(g.level.data.spawn.cell());
    let start: Vec<Vec2> = g.animals.iter().map(|a| a.pos).collect();
    for _ in 0..(60.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
    let mut moved = 0;
    for (a, p0) in g.animals.iter().zip(start) {
        if g.part_unlocked(a.part) {
            assert_eq!(a.state, AnimalState::Escaped);
            assert!(
                a.wander_area().contains(cell_of(a.pos)),
                "{} left its area",
                a.id()
            );
            if a.pos.distance(p0) > 0.05 {
                moved += 1;
            }
        } else {
            // locked levels stay asleep (Q-201 answered: active only when the level opens)
            assert!(a.pos.distance(p0) < 1e-4, "{} moved while locked", a.id());
        }
    }
    assert!(moved >= 1, "no unlocked animal moved in 60 s");
}

// LAYOUT-045: every enclosure animal of a level is a mission of that level (or the field is
// empty) and has at least 3 hiding-place candidates.
#[test]
fn layout_045_every_enclosure_animal_has_a_mission_and_three_places() {
    let g = all_zoo();
    let data = &g.level.data;
    for a in &g.animals {
        let part = &data.parts[a.part];
        assert!(
            part.missions.iter().any(|m| m == a.id()),
            "{} is not a mission of {}",
            a.id(),
            part.id
        );
        let n = data
            .hiding_places
            .iter()
            .filter(|h| h.animal == a.id())
            .count();
        assert!(n >= 3, "{} has {n} hiding places", a.id());
    }
}
