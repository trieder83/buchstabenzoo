//! GAME-MAP overview map: level progress, enclosures, player, next mark (MAP-010…016).

mod common;

use zoo_core::hints::HintTracker;
use zoo_core::overview::{overview, PartState};

fn part<'a>(o: &'a zoo_core::overview::Overview, id: &str) -> &'a zoo_core::overview::OverviewPart {
    o.parts.iter().find(|p| p.id == id).expect(id)
}

// MAP-010
#[test]
fn map_010_new_game_part_states() {
    let g = common::night_game(3);
    let o = overview(&g, &HintTracker::default());
    let l1 = part(&o, "level_1");
    assert_eq!((l1.state, l1.total, l1.home), (PartState::Open, 3, 0));
    assert_eq!(part(&o, "level_2").state, PartState::Locked);
    assert_eq!(part(&o, "level_3").state, PartState::Locked);
    let n = part(&o, "night_1");
    assert!(n.night);
    assert_eq!(n.state, PartState::Locked);
    assert!(!l1.night);
}

// MAP-011, MAP-009
#[test]
fn map_011_solved_and_enclosure_home_flags() {
    let mut g = common::night_game(3);
    assert!(g.debug_send_home("zebra"));
    let o = overview(&g, &HintTracker::default());
    let l1 = part(&o, "level_1");
    assert_eq!((l1.state, l1.home), (PartState::Open, 1));
    let home = |o: &zoo_core::overview::Overview, a: &str| {
        o.enclosures.iter().find(|e| e.animal == a).expect(a).home
    };
    assert!(home(&o, "zebra"));
    assert!(!home(&o, "hippo"));
    assert!(g.debug_send_home("hippo") && g.debug_send_home("panda"));
    let o = overview(&g, &HintTracker::default());
    assert_eq!(part(&o, "level_1").state, PartState::Solved);
}

// MAP-012
#[test]
fn map_012_opening_the_gate_unlocks_the_level() {
    let mut g = common::night_game(3);
    let barrier = g.level.data.parts[1].entries[0].barrier.clone();
    assert!(g.level.open_barrier(&barrier));
    let o = overview(&g, &HintTracker::default());
    let l2 = part(&o, "level_2");
    assert_eq!(l2.state, PartState::Open);
    assert_eq!((l2.total, l2.home), (4, 0));
}

// MAP-013
#[test]
fn map_013_player_position_and_facing() {
    let g = common::night_game(3);
    let o = overview(&g, &HintTracker::default());
    assert_eq!(o.player, g.player.pos);
    assert_eq!(o.facing, g.player.facing);
    assert_eq!(o.player_part, Some(0));
    let v: serde_json::Value = serde_json::from_str(&o.to_json()).unwrap();
    assert_eq!(v["player"]["part"], 0);
    assert_eq!(v["parts"].as_array().unwrap().len(), 4);
}

// MAP-005, MAP-014
#[test]
fn map_005_014_no_escaped_animals_or_hiding_places() {
    let g = common::night_game(11);
    let o = overview(&g, &HintTracker::default());
    let json = o.to_json();
    for h in &g.level.data.hiding_places {
        assert!(
            !json.contains(&format!("\"{}\"", h.id)),
            "hiding place {} leaked",
            h.id
        );
    }
    // one icon per enclosure, at the enclosure: nothing positioned at an escaped animal
    assert!(!o.enclosures.is_empty());
    for e in &o.enclosures {
        let a = g.animals.iter().find(|a| a.id() == e.animal).unwrap();
        let c = zoo_core::level::cell_of(a.pos);
        assert!(
            !e.rect.contains(c) || !e.home,
            "{} escaped inside its enclosure rect?",
            e.animal
        );
    }
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.get("animals").is_none());
}

// MAP-015
#[test]
fn map_015_next_mark_names_a_place_never_the_hiding_area() {
    let mut g = common::night_game(11);
    let t = HintTracker::default();
    let first = overview(&g, &t).next.expect("a next target");
    assert!(first.part == 0);
    // pick up the food for a mission, then the target is the animal's area: part only
    for _ in 0..4000 {
        g.update(1.0 / 60.0, glam::Vec2::ZERO);
    }
    let o = overview(&g, &t);
    if let Some(n) = &o.next {
        if n.kind == "animal" || n.kind == "help" {
            assert!(n.pos.is_none());
        } else {
            assert!(n.pos.is_some());
        }
    }
}

// MAP-016
#[test]
fn map_016_shapes_cover_buildings_paths_water_barriers() {
    let g = common::night_game(3);
    let o = overview(&g, &HintTracker::default());
    for class in ["path", "water", "building", "barrier", "wall"] {
        assert!(o.shapes.iter().any(|s| s.class == class), "no {class}");
    }
    for kind in ["entrance", "food_storage"] {
        assert!(o.shapes.iter().any(|s| s.kind == kind), "no {kind}");
    }
    assert!(o.shapes.iter().all(|s| s.part < o.parts.len()));
}
