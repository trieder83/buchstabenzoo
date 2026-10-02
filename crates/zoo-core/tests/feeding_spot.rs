//! GARD-010/013/014 (user report 2026-09-30): the feeding spot, the group called to it, the
//! baby as a real group member, the pair entering together, the gate after the last animal.

mod common;

use glam::Vec2;
use zoo_core::game::{GameEvent, Target};
use zoo_core::garden::Treat;
use zoo_core::level::{cell_center, cell_of};
use zoo_core::{AnimalState, Game};

const DT: f32 = 1.0 / 60.0;
const TREAT_EATERS: [&str; 6] = ["zebra", "elephant", "giraffe", "hippo", "monkey", "panda"];

fn run(g: &mut Game, seconds: f32) {
    for _ in 0..(seconds / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
}

// GARD-013: every enclosure of an animal that takes treats has a feeding spot on the gate
// side: its two cells belong to the animal's home area, the stand point is outside the fence,
// walkable, and at least 1 m (one empty cell) from the gate posts.
#[test]
fn gard_013_feed_spot_for_every_treat_eater() {
    let g = common::night_game(1);
    for id in TREAT_EATERS {
        let a = g.animal(id).unwrap();
        let e = a.enclosure;
        let enc = &g.level.data.elements[e];
        let s = g
            .feed_spot(e)
            .unwrap_or_else(|| panic!("{id}: no feeding spot"));
        let home = zoo_core::wander::home_area(&g.level, e);
        for c in s.cells {
            assert!(home.contains(c), "{id}: spot cell {c} is in the home area");
            assert!(
                enc.gate
                    .unwrap()
                    .cells()
                    .all(|gc| (gc - c).abs().max_element() >= 2),
                "{id}: spot cell {c} keeps a cell from the gate"
            );
        }
        let st = g.level.grid();
        assert!(
            st.is_walkable(cell_of(s.stand), false),
            "{id}: stand walkable"
        );
        let r = enc.rect;
        let inside = s.stand.x > r.x as f32
            && s.stand.x < (r.x + r.w) as f32
            && s.stand.y > r.z as f32
            && s.stand.y < (r.z + r.d) as f32;
        assert!(!inside, "{id}: the child stands outside the fence");
    }
}

// GARD-010: the child at the feeding spot with a liked treat calls the group: male and female
// walk there (1 m/s) and the treat can be given; the hippo comes out of its pool.
#[test]
fn gard_010_group_comes_to_the_feed_spot() {
    for id in ["zebra", "hippo", "panda", "giraffe", "elephant", "monkey"] {
        let mut g = common::night_game(3);
        let barriers: Vec<String> = g
            .level
            .data
            .elements
            .iter()
            .filter(|e| e.ty == zoo_core::level::ElementType::Barrier)
            .map(|e| e.id.clone())
            .collect();
        for b in &barriers {
            g.level.open_barrier(b);
        }
        assert!(g.debug_send_home(id), "{id}");
        g.drain_events();
        g.garden.basket.carrots = 2;
        let e = g.animal(id).unwrap().enclosure;
        let s = g.feed_spot(e).unwrap().clone();
        g.player.pos = s.stand;
        g.player.facing = (cell_center(s.cells[0]) - s.stand).normalize();
        run(&mut g, 45.0);
        for j in g.group(id) {
            let a = &g.animals[j];
            let c = cell_center(s.cell_for(usize::from(a.member)));
            assert!(
                a.pos.distance(c) < 0.6,
                "{id}: member {} at its spot cell {c} (at {}, route {:?}, cells {:?})",
                a.member,
                a.pos,
                a.wander.route,
                s.cells
            );
        }
        // the nearest member is in reach and in front of the child
        assert_eq!(
            g.available_target(),
            Some(Target::Treat {
                animal: g.animal(id).unwrap().id()
            }),
            "{id}: can be given (player {} facing {}, spot {:?}, animals {:?})",
            g.player.pos,
            g.player.facing,
            s.cells,
            g.group(id)
                .iter()
                .map(|&j| g.animals[j].pos)
                .collect::<Vec<_>>()
        );
    }
}

// GARD-013: the baby is a real member of the group: a cell next to its mother inside the
// fence, it comes to the spot (rank 2) and turns to the child; it is restored from a save.
#[test]
fn gard_013_baby_is_a_group_member() {
    let mut g = common::night_game(5);
    g.debug_send_home("zebra");
    g.drain_events();
    g.garden.basket.carrots = 4;
    assert_eq!(g.give_treat("zebra", Treat::Carrot), Some(true));
    let b = g.baby_states.get("zebra").expect("the baby exists").clone();
    let e = g.animal("zebra").unwrap().enclosure;
    let r = g.level.data.elements[e].rect;
    let inner = b.pos.x > r.x as f32 + 0.9
        && b.pos.x < (r.x + r.w) as f32 - 0.9
        && b.pos.y > r.z as f32 + 0.9
        && b.pos.y < (r.z + r.d) as f32 - 0.9;
    assert!(inner, "the baby never stands at the fence: {:?}", b.pos);
    // at the feeding spot
    let s = g.feed_spot(e).unwrap().clone();
    g.player.pos = s.stand;
    g.player.facing = (cell_center(s.cells[0]) - s.stand).normalize();
    run(&mut g, 45.0);
    let b = g.baby_states.get("zebra").unwrap();
    assert!(
        b.pos.distance(cell_center(s.cell_for(2))) < 0.6,
        "baby at rank 2"
    );
    assert!(
        b.pos.x < s.stand.x + 0.01 || s.inward.x >= 0,
        "inside the fence on the child's side"
    );
    // restore keeps it
    let save = g.to_save();
    let h = Game::from_save(common::zoo_with_night(), &save).expect("restores");
    assert!(h.baby_states.contains_key("zebra"), "baby after restore");
}

// GARD-014: a partner that waits far behind must catch up; the pair enters together.
#[test]
fn gard_014_pair_enters_together_and_gate_lets_the_child_out() {
    let mut g = common::night_game(7);
    let e = g.animal("zebra").unwrap().enclosure;
    let gate = g.level.data.elements[e].gate.unwrap();
    let gate_c = cell_center(glam::IVec2::new(gate.x, gate.z));
    for j in g.group("zebra") {
        g.animals[j].state = AnimalState::Following;
        g.animals[j].pos = gate_c + Vec2::new(1.5, 0.0);
    }
    let far = g.group("zebra")[1];
    g.animals[far].waiting = true;
    g.animals[far].pos = gate_c + Vec2::new(25.0, 0.0);
    g.player.pos = gate_c;
    assert_eq!(
        g.available_target(),
        Some(Target::Gate {
            enclosure: "enc_zebra".into()
        })
    );
    g.interact();
    assert!(
        g.group("zebra")
            .iter()
            .all(|&j| g.animals[j].state == AnimalState::Following),
        "nobody enters while a partner is far behind"
    );
    assert!(g.animals[far].waiting, "the partner still waits far behind");
    // the hint sends the child back to fetch it
    let hints = zoo_core::hints::candidates(&g, &zoo_core::hints::HintTracker::default());
    assert!(
        hints.iter().any(|h| h.id.starts_with("partner:zebra")),
        "partner hint"
    );
    // she walks back: it follows again, comes close; then both enter and the mission is complete
    g.player.pos = g.animals[far].pos - Vec2::new(3.0, 0.0);
    for _ in 0..(40.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        if g.animals[far].pos.distance(g.player.pos) < 2.5 {
            break;
        }
    }
    g.player.pos = gate_c;
    g.animals[far].pos = gate_c + Vec2::new(1.5, 0.0);
    g.update(DT, Vec2::ZERO);
    g.interact();
    assert!(g
        .group("zebra")
        .iter()
        .all(|&j| g.animals[j].state == AnimalState::InEnclosure));
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::MissionComplete { .. })));
    // the gate is solid again, but the child standing on it can walk out
    let start = g.player.pos;
    for _ in 0..180 {
        g.update(DT, Vec2::new(1.0, 0.0));
    }
    assert!(
        g.player.pos.x > start.x + 1.0,
        "moved out: {:?} -> {:?}",
        start,
        g.player.pos
    );
}
