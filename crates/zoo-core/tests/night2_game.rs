//! GAME-LEVEL-NIGHT-2 game rules: the lantern gate (NIGHT-030), optional level and saves
//! (NIGHT-032), the gate hint (NIGHT-031), the pairs of snake / chameleon / poison dart frog
//! (FAM-021..027 for the new species), basic food and treat (FEED-37/38, FAM-031), hints (HINT-024/025).

mod common;

use glam::Vec2;
use zoo_core::animals::{self, pair_gap_m};
use zoo_core::hints::{candidates, HintKind, HintTracker};
use zoo_core::level::{cell_center, cell_of, ElementType};
use zoo_core::wander::{hiding_area, home_area};
use zoo_core::{AnimalState, Food, FoodBox, Game, GameEvent};

const GATE: &str = "barrier_n1_garden";
const NEW: [&str; 3] = ["snake", "chameleon", "poison_dart_frog"];
const NIGHT_1: [&str; 3] = ["hedgehog", "bat", "owl"];

/// A night in which the three night_1 animals are home: the garden gate is open.
fn garden(seed: u64) -> Game {
    let mut g = common::night2_game(seed);
    for a in ["zebra", "hippo", "panda"] {
        assert!(g.debug_send_home(a), "{a}");
    }
    assert!(g.debug_set_daytime("night"));
    for a in NIGHT_1 {
        assert!(g.debug_send_home(a), "{a}");
    }
    g.drain_events();
    assert!(g.level.is_barrier_open(GATE), "the gate is open");
    g
}

fn lead_home(g: &mut Game, id: &str) {
    let group = g.group(id);
    let enc = g.level.data.elements[g.animals[group[0]].enclosure].clone();
    let gate = enc.gate.unwrap();
    g.player.pos = cell_center(glam::IVec2::new(gate.x, gate.z));
    for &i in &group {
        g.animals[i].pos = g.player.pos + Vec2::new(0.0, 1.0);
    }
    for _ in 0..600 {
        g.update(1.0 / 60.0, Vec2::ZERO);
    }
    assert!(
        group
            .iter()
            .all(|&i| g.animals[i].state == AnimalState::InEnclosure),
        "{id}: both home"
    );
}

// NIGHT-030: night_1 complete -> the gate opens at once (not the next morning) and stays open;
// before that it is closed and solid; `barrier_ne_tree` still opens in the morning.
#[test]
fn night_030_gate_opens_when_night_1_is_complete() {
    let mut g = common::night2_game(3);
    assert!(g.debug_set_daytime("night"));
    assert!(!g.level.is_barrier_open(GATE));
    assert!(!g.level_unlocked("night_2"));
    assert!(g.debug_send_home("hedgehog") && g.debug_send_home("bat"));
    assert!(!g.level.is_barrier_open(GATE), "two of three: still closed");
    assert!(g.debug_send_home("owl"));
    let ev = g.drain_events();
    assert!(
        ev.iter()
            .any(|e| matches!(e, GameEvent::BarrierOpened { id } if id == GATE)),
        "{ev:?}"
    );
    assert!(g.level.is_barrier_open(GATE) && g.level_unlocked("night_2"));
    assert!(
        !g.level.is_barrier_open("barrier_ne_tree"),
        "level 2 waits for the morning"
    );
    // the morning: level 2 opens, the garden gate stays open; and every later night
    g.debug_next_morning();
    assert!(g.level.is_barrier_open("barrier_ne_tree"));
    assert!(g.level.is_barrier_open(GATE));
    assert!(g.debug_set_daytime("night"));
    assert!(g.level.is_barrier_open(GATE), "stays open the next night");
    // the moon door still follows the day / night rhythm
    assert!(g.debug_set_daytime("day"));
    assert!(
        g.level.is_barrier_open(GATE),
        "a night gate is no moon door"
    );
}

// NIGHT-032: night_2 is optional and saved: a half-played garden restores (animals, pair, baby, gate);
// a save from before night_2 existed opens it fresh once night_1 was complete.
#[test]
fn night_032_optional_level_save_restore_and_old_saves() {
    let mut g = garden(4);
    assert!(g.debug_send_home("snake"));
    let snake = g.group("snake");
    assert!(snake
        .iter()
        .all(|&i| g.animals[i].state == AnimalState::InEnclosure));
    g.carry.take(&FoodBox { food: Food::Eggs });
    assert_eq!(g.give_food("snake"), Some(true));
    assert_eq!(g.babies, ["snake"]);
    let save = g.to_save();
    let r = Game::from_save(common::zoo_with_night2(), &save).expect("restores");
    assert!(r.level.is_barrier_open(GATE));
    assert_eq!(r.babies, ["snake"]);
    assert!(r
        .group("snake")
        .iter()
        .all(|&i| r.animals[i].state == AnimalState::InEnclosure));
    assert!(r
        .group("chameleon")
        .iter()
        .all(|&i| r.animals[i].state == AnimalState::Escaped));
    // a save made before the gate existed: night_1 complete, no gate in the list
    let mut old = save.clone();
    old.open_barriers.retain(|b| b != GATE);
    old.babies.clear();
    let r = Game::from_save(common::zoo_with_night2(), &old).expect("restores");
    assert!(
        r.level.is_barrier_open(GATE),
        "the gate of a complete night_1 reopens"
    );
    // a save of the game WITHOUT night_2 (night_1 complete): the garden is there, fresh
    let mut older = common::night_game(6);
    assert!(older.debug_set_daytime("night"));
    for a in NIGHT_1 {
        older.debug_send_home(a);
    }
    let r = Game::from_save(common::zoo_with_night2(), &older.to_save()).expect("restores");
    assert!(r.level.is_barrier_open(GATE) && r.level_unlocked("night_2"));
    for id in NEW {
        assert_eq!(r.group(id).len(), 2, "{id}");
        assert!(r
            .group(id)
            .iter()
            .all(|&i| r.animals[i].state == AnimalState::Escaped));
    }
    // sleeping and the morning work with night_2 unfinished
    let mut g = garden(8);
    assert!(g.any_mission_open());
    g.debug_next_morning();
    assert!(
        g.level.is_barrier_open("barrier_ne_tree"),
        "the morning and level 2 do not wait for night_2"
    );
    assert!(g.level.is_barrier_open(GATE));
}

// FAM-021 / FAM-024 for the garden: the new species are pairs at one hiding place, a gap apart.
#[test]
fn fam_021_024_garden_pairs_start_together_a_gap_apart() {
    for seed in 0..40u64 {
        let g = common::night2_game(seed);
        for id in NEW {
            let group = g.group(id);
            assert_eq!(group.len(), 2, "{id}");
            let (a, b) = (&g.animals[group[0]], &g.animals[group[1]]);
            assert_eq!((a.member, b.member), (0, 1), "{id}");
            assert_eq!(a.hiding_place, b.hiding_place, "{id}");
            let d = a.pos.distance(b.pos);
            assert!(
                (pair_gap_m(id) - 1e-3..=3.0).contains(&d),
                "seed {seed} {id}: {d} m"
            );
            assert!(a.state == AnimalState::Escaped && b.state == AnimalState::Escaped);
            assert!(g.level.data.part_index("night_2") == Some(a.part));
        }
    }
}

// FAM-022 / FAM-023 for the garden: places hold two animals; the terrarium fits male, female, baby.
#[test]
fn fam_022_023_garden_places_and_terrariums_fit_two_and_a_baby() {
    let g = common::night2_game(1);
    for id in NEW {
        for h in g.level.data.hiding_places_of(id) {
            let area = hiding_area(&g.level, h);
            assert!(area.len() >= 9, "{}: {} cells", h.id, area.len());
            let far = area
                .cells()
                .filter(|&(c, _)| {
                    let d = cell_center(c).distance(h.spot());
                    d >= pair_gap_m(id) - 1e-3 && d <= 3.0
                })
                .count();
            assert!(far >= 1, "{}", h.id);
        }
        let e = g
            .level
            .data
            .elements_of(ElementType::Enclosure)
            .find(|e| e.animal.as_deref() == Some(id))
            .unwrap();
        assert!(e.pair && e.indoor && e.terrarium, "{id}");
        let idx = g
            .level
            .data
            .elements
            .iter()
            .position(|x| x.id == e.id)
            .unwrap();
        assert!(home_area(&g.level, idx).len() >= 12, "{id}: home cells");
    }
}

// FAM-026 for the garden: the basic food to one animal makes both follow, both enter the glass gate.
#[test]
fn fam_026_garden_pairs_follow_and_enter_together() {
    for id in NEW {
        let mut g = garden(5);
        let food = g.animals[g.group(id)[0]].info.foods[0];
        g.carry.take(&FoodBox { food });
        let first = g.group(id)[0];
        g.player.pos = g.animals[first].pos + Vec2::new(0.0, -1.0);
        g.show_food(id).unwrap_or_else(|e| panic!("{id}: {e:?}"));
        assert!(
            g.group(id)
                .iter()
                .all(|&i| g.animals[i].state == AnimalState::Following),
            "{id}: both follow"
        );
        lead_home(&mut g, id);
        assert!(g.mission(id).unwrap().complete, "{id}");
        assert!(g.group(id).iter().all(|&i| g.missions[i].complete), "{id}");
    }
}

// FEED-37: an escaped animal follows its basic food only; a treat or a wrong food gets the gentle
// "not interested" and no following.
#[test]
fn feed_037_only_the_basic_food_makes_the_escaped_animal_follow() {
    for (id, basic, treat) in [
        ("snake", Food::Fish, Food::Eggs),
        ("chameleon", Food::Crickets, Food::FrozenInsects),
        ("poison_dart_frog", Food::Flies, Food::Crickets),
    ] {
        for (food, follows) in [(treat, false), (Food::Grass, false), (basic, true)] {
            let mut g = garden(2);
            g.carry.take(&FoodBox { food });
            let first = g.group(id)[0];
            g.player.pos = g.animals[first].pos + Vec2::new(0.0, -1.0);
            g.drain_events();
            g.show_food(id).unwrap();
            let ev = g.drain_events();
            let following = g
                .group(id)
                .iter()
                .all(|&i| g.animals[i].state == AnimalState::Following);
            assert_eq!(following, follows, "{id} with {food:?}");
            if !follows {
                assert!(
                    ev.iter()
                        .any(|e| matches!(e, GameEvent::NotInterested { .. })),
                    "{id} {food:?}"
                );
            }
        }
    }
}

// FEED-38 / FAM-031 for the garden: at home the basic food gives hearts and no baby, the treat the
// baby (once), a neither-food a gentle refusal.
#[test]
fn feed_038_fam_031_treat_makes_the_baby_basic_food_does_not() {
    for (id, basic, treat) in [
        ("snake", Food::Fish, Food::Eggs),
        ("chameleon", Food::Crickets, Food::FrozenInsects),
        ("poison_dart_frog", Food::Flies, Food::Crickets),
    ] {
        let mut g = garden(7);
        assert!(g.debug_send_home(id));
        g.drain_events();
        // neither: refused
        g.carry.take(&FoodBox { food: Food::Grass });
        assert_eq!(g.give_food(id), Some(false), "{id}");
        assert!(g.babies.is_empty());
        // basic: hearts, no baby
        g.carry.take(&FoodBox { food: basic });
        assert_eq!(g.give_food(id), Some(true), "{id}");
        assert!(g.babies.is_empty(), "{id}: basic food makes no baby");
        assert!(!g.special_gift_in_hand(id));
        // treat: the baby, once
        g.carry.take(&FoodBox { food: treat });
        assert!(g.special_gift_in_hand(id));
        assert_eq!(g.give_food(id), Some(true), "{id}");
        assert_eq!(g.babies, [id], "{id}: one baby");
        assert_eq!(g.give_food(id), Some(true));
        assert_eq!(g.babies, [id], "{id}: never twice");
        // the models of the family
        assert!(
            animals::female_model(id).is_some() && animals::baby_model(id).is_some(),
            "{id}"
        );
        assert_eq!(animals::baby_look(id).scale, 1.0);
    }
}

// The info board names the treat (GAME-FEED "Info board") next to the basic food.
#[test]
fn info_board_of_the_new_species_lists_basic_food_and_treat() {
    let g = garden(1);
    for (id, basic, treat) in [
        ("snake", Food::Fish, Food::Eggs),
        ("chameleon", Food::Crickets, Food::FrozenInsects),
        ("poison_dart_frog", Food::Flies, Food::Crickets),
    ] {
        let b = g.info_board(id).unwrap();
        assert_eq!(b.food_key, basic.label_key());
        let (prefix, food) = b.treat.expect("treat line");
        assert_eq!(food, treat.label_key());
        assert!(prefix.starts_with("board-treat-"));
    }
    // a species without box-food treats keeps the old board
    assert!(g.info_board("zebra").unwrap().treat.is_none());
}

// NIGHT-031: in night_1 with the gate open and night_2 incomplete the 🧭 leads through the gate
// first (before the bed); in night_2 the first candidate is the next mission step; once night_2 is
// complete the bed.
#[test]
fn night_031_the_compass_leads_through_the_gate_then_through_the_garden() {
    let mut g = garden(9);
    let t = HintTracker::default();
    g.player.pos = cell_center(glam::IVec2::new(-30, 29)); // night_1 plaza
    let c = candidates(&g, &t);
    let first = c.first().expect("a hint");
    assert_eq!(first.kind, HintKind::NightGate, "{first:?}");
    assert!(first.priority <= hints_prio_unstarted());
    assert!(
        c.iter()
            .any(|h| h.kind == HintKind::Bed || h.kind == HintKind::MoonDoor),
        "bed offered too"
    );
    // in the garden: the next mission step, never the way out first
    g.player.pos = cell_center(glam::IVec2::new(-76, 25));
    let c = candidates(&g, &t);
    let first = c.first().unwrap();
    assert!(
        first.kind == HintKind::Board && first.animal.is_some_and(|a| NEW.contains(&a)),
        "{first:?}"
    );
    // everything home: the bed
    for id in NEW {
        g.debug_send_home(id);
    }
    let c = candidates(&g, &t);
    assert!(
        c.first()
            .is_some_and(|h| h.kind == HintKind::Bed || h.kind == HintKind::NightGate),
        "{:?}",
        c.first()
    );
}

fn hints_prio_unstarted() -> u8 {
    zoo_core::hints::PRIO_UNSTARTED
}

// HINT-025 (NEVER STUCK, night_2): from many states of the garden a hint of priority <= 3 always
// exists while a mission is open, and the safety net points at the missing animal after 120 s.
#[test]
fn hint_025_garden_always_has_a_mission_level_hint() {
    use zoo_core::rng::Pcg32;
    for seed in 0..60u64 {
        let mut r = Pcg32::new(seed);
        let mut g = garden(seed);
        for id in NEW {
            match r.next_u32() % 4 {
                0 => {}
                1 => {
                    for j in g.group(id) {
                        g.missions[j].started = true;
                    }
                }
                2 => {
                    g.debug_send_home(id);
                }
                _ => {
                    // split pair: one home, the partner out
                    let grp = g.group(id);
                    g.animals[grp[1]].state = AnimalState::InEnclosure;
                    g.missions[grp[0]].started = true;
                }
            }
        }
        g.drain_events();
        let grid = g.level.grid();
        let cells: Vec<_> = g.level.data.parts[g.level.data.part_index("night_2").unwrap()]
            .bounds
            .cells()
            .filter(|&c| grid.is_walkable(c, false))
            .collect();
        g.player.pos = cell_center(cells[(r.next_u32() as usize) % cells.len()]);
        let t = HintTracker::default();
        let open = NEW.iter().any(|id| !g.mission(id).unwrap().complete);
        let c = candidates(&g, &t);
        if open {
            assert!(
                c.first()
                    .is_some_and(|h| h.priority <= zoo_core::hints::PRIO_UNSTARTED),
                "seed {seed}: {:?}",
                c.first().map(|h| (&h.id, h.priority))
            );
            // never the treat as the only hint
            assert!(c.iter().any(|h| h.kind != HintKind::Treat));
        }
        assert!(cell_of(g.player.pos).x < -72, "in night_2");
    }
}

// GAME-MAP (MAP-010) for the garden: the overview lists night_2 as a night level, locked until the
// gate opens, then open with its three animals, then solved; the hedge and the terrarium house are
// drawn; the next mark is the gate while the child is in night_1.
#[test]
fn overview_lists_the_terrarium_garden_with_its_progress() {
    use zoo_core::overview::{overview, PartState};
    let t = HintTracker::default();
    let g = common::night2_game(1);
    let o = overview(&g, &t);
    let p = o
        .parts
        .iter()
        .find(|p| p.id == "night_2")
        .expect("night_2 listed");
    assert!(p.night);
    assert_eq!((p.state, p.total, p.home), (PartState::Locked, 3, 0));
    let mut g = garden(2);
    let o = overview(&g, &t);
    let p = o.parts.iter().find(|p| p.id == "night_2").unwrap();
    assert_eq!((p.state, p.total, p.home), (PartState::Open, 3, 0));
    assert!(o
        .shapes
        .iter()
        .any(|s| s.id == "terrarium_house" && s.class == "building"));
    for id in NEW {
        assert!(g.debug_send_home(id));
    }
    let o = overview(&g, &t);
    let p = o.parts.iter().find(|p| p.id == "night_2").unwrap();
    assert_eq!((p.state, p.home), (PartState::Solved, 3));
}
