//! M5b game logic on the joined zoo (levels 1–3): missions in scope per unlocked level,
//! discovery per level, perches (Q-094), the goldfish bowl (RESC-018…023), pairs (FAM-001/002,
//! LAYOUT-L2-015), saves across levels and the content of all ten animals.

mod common;

use glam::{IVec2, Vec2};
use zoo_core::content::{riddle_key, Language, ReadingLevel};
use zoo_core::game::{GameEvent, Target};
use zoo_core::level::{cell_center, ElementType};
use zoo_core::{AnimalState, Food, FoodBox, Game};

/// A zoo game with levels 1 and 2 complete (all barriers of levels 1–3 open).
fn unlocked_game(seed: u64) -> Game {
    let mut g = common::zoo_game(seed);
    for a in [
        "zebra", "hippo", "panda", "koala", "elephant", "giraffe", "lion",
    ] {
        assert!(g.debug_send_home(a), "{a}");
    }
    // the barriers open the next morning (GAME-NIGHT, Q-091)
    g.debug_next_morning();
    g.drain_events();
    assert!(g.level_unlocked("level_3"));
    g
}

fn carry(g: &mut Game, food: Food) {
    g.carry.take(&FoodBox { food });
}

/// Puts the player next to the goldfish (on the bank within 2 m) facing it.
fn at_goldfish(g: &mut Game) {
    let fish = g.animal("goldfish").unwrap().pos;
    let grid = g.level.grid();
    let stand = zoo_core::Rect::new(fish.x as i32 - 3, fish.y as i32 - 3, 7, 7)
        .cells()
        .filter(|&c| grid.is_passable(c, false))
        .map(cell_center)
        .min_by(|a, b| a.distance(fish).total_cmp(&b.distance(fish)))
        .unwrap();
    g.player.pos = stand;
    g.player.facing = (fish - stand).normalize();
}

#[test]
fn joined_zoo_has_all_ten_animals_and_level_1_is_unchanged() {
    for seed in [1, 17, 99] {
        let g = common::zoo_game(seed);
        let mut ids: Vec<&str> = g.animals.iter().map(|a| a.id()).collect();
        ids.dedup(); // the zebras and koalas come as pairs (GAME-FAMILY)
        assert_eq!(ids.len(), 10, "{ids:?}");
        // level 1 keeps its picks and wander timing from the main RNG (determinism of M5a)
        let one = common::game(seed);
        for (a, b) in one.animals.iter().zip(&g.animals) {
            assert_eq!((a.id(), a.member), (b.id(), b.member));
            assert_eq!(a.hiding_place, b.hiding_place, "{} seed {seed}", a.id());
            assert_eq!(a.wander.pause_s, b.wander.pause_s);
        }
    }
}

// RESC-025, LAYOUT-025 / GAME-LAYOUT "Missions in scope": only unlocked levels' missions are
// interactable; the later levels' animals wait asleep.
#[test]
fn missions_in_scope_are_the_unlocked_levels() {
    let mut g = common::zoo_game(5);
    let scope = |g: &Game| {
        let mut v: Vec<&str> = g
            .interactables()
            .iter()
            .filter_map(|i| match i.target {
                Target::InfoBoard { animal } | Target::Animal { animal } => Some(animal),
                _ => None,
            })
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    assert_eq!(scope(&g), ["hippo", "panda", "zebra"]);
    let koala = g.animal("koala").unwrap().pos;
    g.player.pos = koala + Vec2::new(0.0, -1.0);
    carry(&mut g, Food::Eucalyptus);
    assert!(g.show_food("koala").is_err(), "locked level");
    // locked animals do not wander
    let before: Vec<Vec2> = g.animals.iter().map(|a| a.pos).collect();
    for _ in 0..(60 * 40) {
        g.update(1.0 / 60.0, Vec2::ZERO);
    }
    for (a, p) in g.animals.iter().zip(before) {
        if a.part > 0 {
            assert_eq!(a.pos, p, "{} moved while locked", a.id());
        }
    }
    for a in ["zebra", "hippo", "panda"] {
        g.debug_send_home(a);
    }
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::LevelComplete {
        level: "level_1".into()
    }));
    assert!(
        !ev.contains(&GameEvent::BarrierOpened {
            id: "barrier_ne_tree".into()
        }),
        "the barrier opens the next morning (Q-091)"
    );
    let ev = g.debug_next_morning();
    assert!(ev.contains(&GameEvent::BarrierOpened {
        id: "barrier_ne_tree".into()
    }));
    assert!(
        !ev.contains(&GameEvent::AllAnimalsHome),
        "7 animals are still out"
    );
    // level-2 boards and animals are now in scope (the level-1 boards stay readable)
    assert_eq!(
        scope(&g),
        ["elephant", "giraffe", "hippo", "koala", "lion", "panda", "zebra"]
    );
}

// RESC-014 / RESC-002 per level: every candidate is picked for some seed, picks of one
// level are ≥ 12 m apart, no shared place, same seed → same places.
#[test]
fn resc_014_discovery_per_level() {
    let zoo = common::zoo();
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..300u64 {
        let g = Game::new(zoo.clone(), seed).unwrap();
        for part in 0..3 {
            let mut members: Vec<_> = g.animals.iter().filter(|a| a.part == part).collect();
            members.dedup_by(|a, b| a.id() == b.id()); // a pair shares one hiding place
            let picks: Vec<Vec2> = members
                .iter()
                .map(|a| zoo.hiding_place(&a.hiding_place).unwrap().spot())
                .collect();
            for (i, a) in picks.iter().enumerate() {
                for b in &picks[i + 1..] {
                    assert!(a.distance(*b) >= 12.0 - 1e-4, "seed {seed}: {a} {b}");
                }
            }
        }
        for a in &g.animals {
            seen.insert(a.hiding_place.clone());
        }
        let again = Game::new(zoo.clone(), seed).unwrap();
        for (a, b) in g.animals.iter().zip(&again.animals) {
            assert_eq!(a.hiding_place, b.hiding_place);
        }
    }
    assert_eq!(
        seen.len(),
        zoo.hiding_places.len(),
        "every candidate picked"
    );
}

// RESC-026, Q-094 perches: a koala / monkey at a perch place sits up there, does not wander, is shown
// food from the ground and comes down when it follows.
#[test]
fn perch_animals_sit_up_and_come_down() {
    let zoo = common::zoo();
    let seed = (0..200)
        .find(|&s| {
            let g = Game::new(zoo.clone(), s).unwrap();
            g.animal("koala").unwrap().hiding_place == "loc_tallest_tree"
        })
        .unwrap();
    let mut g = unlocked_game(seed);
    // koala was sent home by unlocked_game: use a fresh game with only level 1 done
    g = {
        let mut h = Game::new(zoo.clone(), seed).unwrap();
        for a in ["zebra", "hippo", "panda"] {
            h.debug_send_home(a);
        }
        h.debug_next_morning();
        let _ = &g;
        h
    };
    let k = g.animal("koala").unwrap();
    let (point, height) = g.perch(k).expect("perched");
    assert!((height - 9.0).abs() < 1e-6);
    assert!(point.distance(k.pos) <= 0.3, "perch next to the spot");
    let start = k.pos;
    for _ in 0..(60 * 30) {
        g.update(1.0 / 60.0, Vec2::ZERO);
    }
    assert_eq!(
        g.animal("koala").unwrap().pos,
        start,
        "perched animals do not wander"
    );
    g.player.pos = start + Vec2::new(-1.2, 0.0);
    g.player.facing = Vec2::X;
    carry(&mut g, Food::Eucalyptus);
    assert!(matches!(
        g.available_target(),
        Some(Target::Animal { animal: "koala" })
    ));
    g.interact();
    let k = g.animal("koala").unwrap();
    assert_eq!(k.state, AnimalState::Following);
    assert!(g.perch(k).is_none(), "comes down to follow");
}

// RESC-018 … RESC-023: the goldfish bowl.
#[test]
fn resc_018_to_023_goldfish_bowl() {
    let mut g = unlocked_game(4);
    let bowl = g.bowl.clone().expect("the zoo has a fish bowl");
    assert_eq!(bowl.animal, "goldfish");
    assert!(!bowl.carried && !bowl.water && !bowl.fish);
    // RESC-018: fish food, no bowl → stays with the "needs a bowl" feedback
    carry(&mut g, Food::FishFood);
    at_goldfish(&mut g);
    g.show_food("goldfish").unwrap();
    assert!(g.drain_events().contains(&GameEvent::NeedsContainer {
        animal: "goldfish".into()
    }));
    assert_eq!(g.animal("goldfish").unwrap().state, AnimalState::Escaped);
    assert_eq!(g.carry.food(), Some(Food::FishFood), "food not used up");
    // take the bowl from the table
    g.player.pos = Vec2::new(-7.5, 64.5);
    g.player.facing = Vec2::NEG_X;
    assert_eq!(
        g.available_target(),
        Some(Target::Item {
            id: "fish_bowl".into()
        })
    );
    g.interact();
    assert!(g.bowl.as_ref().unwrap().carried);
    // RESC-020: bowl + one food (pocket)
    assert_eq!(g.carry.food(), Some(Food::FishFood));
    g.carry.take(&FoodBox { food: Food::Meat });
    assert_eq!(
        g.carry.food(),
        Some(Food::Meat),
        "the pocket holds one food"
    );
    assert!(
        g.bowl.as_ref().unwrap().carried,
        "the bowl stays in the hands"
    );
    carry(&mut g, Food::FishFood);
    // RESC-019: empty bowl → "no water"
    at_goldfish(&mut g);
    g.show_food("goldfish").unwrap();
    assert!(g.drain_events().contains(&GameEvent::ContainerEmpty {
        animal: "goldfish".into()
    }));
    assert_eq!(g.animal("goldfish").unwrap().state, AnimalState::Escaped);
    // fill at the tap
    g.player.pos = Vec2::new(-6.5, 59.8);
    g.player.facing = Vec2::Y;
    assert!(matches!(g.available_target(), Some(Target::Water { .. })));
    g.interact();
    assert!(g.bowl.as_ref().unwrap().water);
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::ContainerFilled { .. })));
    // fed again → jumps into the bowl, the food is used up
    at_goldfish(&mut g);
    g.show_food("goldfish").unwrap();
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::InContainer {
        animal: "goldfish".into()
    }));
    assert_eq!(g.animal("goldfish").unwrap().state, AnimalState::InBowl);
    assert!(g.bowl.as_ref().unwrap().fish);
    assert_eq!(g.carry.food(), None);
    // RESC-022: save/restore keeps bowl, water and fish
    let json = g.to_save().to_json();
    let r = Game::from_save_json(common::zoo(), &json).unwrap();
    assert_eq!(r.bowl, g.bowl);
    assert_eq!(r.animal("goldfish").unwrap().state, AnimalState::InBowl);
    // RESC-023: 0.9 × the surface speed while carrying the fish
    let speed = |g: &mut Game| {
        g.player.pos = Vec2::new(-12.5, 70.5); // path_l3_ring_w
        g.player.set_surface_speed(1.93);
        g.update(0.1, Vec2::Y);
        g.player.last_speed
    };
    let with_fish = speed(&mut g);
    let mut plain = unlocked_game(4);
    let without = speed(&mut plain);
    assert!(
        (with_fish / without - 0.9).abs() < 0.01,
        "{with_fish} vs {without}"
    );
    // RESC-021: put the bowl at the goldfish home (stone step) → home, mission complete
    let gate = g.level.data.element("enc_goldfish").unwrap().gate.unwrap();
    g.player.pos = cell_center(IVec2::new(gate.x - 1, gate.z));
    g.player.facing = Vec2::X;
    assert_eq!(
        g.available_target(),
        Some(Target::Gate {
            enclosure: "enc_goldfish".into()
        })
    );
    g.interact();
    let fish = g.animal("goldfish").unwrap();
    assert_eq!(fish.state, AnimalState::InEnclosure);
    assert!(g.mission("goldfish").unwrap().complete);
    let pond = g
        .level
        .data
        .enclosure_features
        .iter()
        .find(|f| f.id == "goldfish_pond")
        .unwrap()
        .rect;
    assert!(
        pond.contains(zoo_core::level::cell_of(fish.pos)),
        "in its pond"
    );
    let b = g.bowl.as_ref().unwrap();
    assert!(!b.carried && !b.fish);
}

// RESC-027: a wrong enclosure refuses the fish; putting the bowl down keeps it safe.
#[test]
fn bowl_put_down_and_wrong_enclosure() {
    let mut g = unlocked_game(8);
    let b = g.bowl.as_mut().unwrap();
    b.carried = true;
    b.water = true;
    carry(&mut g, Food::FishFood);
    at_goldfish(&mut g);
    g.show_food("goldfish").unwrap();
    assert!(g.carrying_animal());
    // wrong gate (snow fox) refuses
    let gate = g.level.data.element("enc_snow_fox").unwrap().gate.unwrap();
    g.player.pos = cell_center(IVec2::new(gate.x, gate.z + 1));
    g.player.facing = Vec2::NEG_Y;
    assert!(matches!(g.available_target(), Some(Target::Gate { .. })));
    g.interact();
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::Refuse { .. })));
    assert!(g.carrying_animal());
    // put down anywhere (nothing else around) and pick up again
    g.player.pos = Vec2::new(-12.5, 70.5);
    g.player.facing = Vec2::Y;
    assert_eq!(g.available_target(), Some(Target::PutDown));
    g.interact();
    let b = g.bowl.clone().unwrap();
    assert!(!b.carried && b.fish && b.water, "the fish stays safe in it");
    assert_eq!(
        g.available_target(),
        Some(Target::Item {
            id: "fish_bowl".into()
        })
    );
    g.interact();
    assert!(g.carrying_animal());
}

/// Level-2 data with the koala pair switched on (GAME-FAMILY data flag).
fn pair_zoo() -> zoo_core::LevelData {
    let mut zoo = common::zoo();
    let enc = zoo
        .elements
        .iter_mut()
        .find(|e| e.ty == ElementType::Enclosure && e.animal.as_deref() == Some("koala"))
        .unwrap();
    enc.pair = true;
    zoo
}

// FAM-001, LAYOUT-L2-015 (with `pair = true`; two koalas of the same model): both start
// escaped at one shared hiding place.
#[test]
fn fam_001_pair_starts_together() {
    for seed in 0..20 {
        let g = Game::new(pair_zoo(), seed).unwrap();
        let koalas: Vec<_> = g.animals.iter().filter(|a| a.id() == "koala").collect();
        assert_eq!(koalas.len(), 2);
        assert_eq!(koalas[0].hiding_place, koalas[1].hiding_place);
        assert!(koalas.iter().all(|a| a.state == AnimalState::Escaped));
        assert_eq!((koalas[0].member, koalas[1].member), (0, 1));
        assert!(koalas[0].pos.distance(koalas[1].pos) <= 3.0);
    }
    // the flag is on in the level data since the female models exist
    assert_eq!(common::zoo_game(1).group("koala").len(), 2);
    assert_eq!(common::zoo_game(1).group("zebra").len(), 2);
}

// FAM-002: food shown to one → both follow as one group; the mission completes only when
// both are home.
#[test]
fn fam_002_pair_follows_and_completes_together() {
    let mut g = Game::new(pair_zoo(), 2).unwrap();
    for a in ["zebra", "hippo", "panda"] {
        g.debug_send_home(a);
    }
    g.debug_next_morning();
    g.drain_events();
    let second = g.group("koala")[1];
    g.player.pos = g.animals[second].pos + Vec2::new(0.0, -1.0);
    carry(&mut g, Food::Eucalyptus);
    g.show_food("koala").unwrap();
    let states: Vec<_> = g
        .group("koala")
        .iter()
        .map(|&i| g.animals[i].state)
        .collect();
    assert_eq!(states, [AnimalState::Following, AnimalState::Following]);
    // one is left behind (waiting): the mission does not complete
    let first = g.group("koala")[0];
    g.animals[first].waiting = true;
    let enc = g.level.data.element("enc_koala").unwrap().gate.unwrap();
    g.player.pos = cell_center(IVec2::new(enc.x, enc.z));
    g.update(1.0 / 60.0, Vec2::ZERO);
    // GARD-014: the pair enters together — the partner is called, nobody enters yet
    assert_eq!(g.animals[second].state, AnimalState::Following);
    assert!(
        g.animals[first].waiting,
        "the late partner still waits far behind"
    );
    assert!(
        !g.mission("koala").unwrap().complete,
        "one koala is still out"
    );
    g.animals[first].waiting = true;
    // the waiting one is fetched and brought home
    g.player.pos = cell_center(IVec2::new(enc.x - 1, enc.z));
    g.animals[first].pos = g.player.pos + Vec2::new(-1.0, 0.0);
    g.update(1.0 / 60.0, Vec2::ZERO);
    assert!(
        !g.animals[first].waiting,
        "follows again when the player is back"
    );
    g.animals[second].pos = g.player.pos + Vec2::new(-1.5, 0.0);
    g.update(1.0 / 60.0, Vec2::ZERO);
    g.update(1.0 / 60.0, Vec2::ZERO);
    g.player.pos = cell_center(IVec2::new(enc.x, enc.z));
    g.update(1.0 / 60.0, Vec2::ZERO);
    assert_eq!(g.animals[first].state, AnimalState::InEnclosure);
    assert_eq!(g.animals[second].state, AnimalState::InEnclosure);
    assert!(g.mission("koala").unwrap().complete);
}

// SAVE-010, GAME-SAVE §8: one save for the joined zoo (v2) round-trips across levels; a v1 save of
// level 1 (M4b/M5a) is migrated (level-1 progress kept, later levels start fresh).
#[test]
fn save_v2_across_levels_and_v1_migration() {
    let mut g = unlocked_game(11);
    g.player.pos = Vec2::new(-12.5, 70.5);
    let json = g.to_save().to_json();
    assert!(json.contains("\"version\":2") && json.contains("\"level_id\":\"zoo\""));
    let r = Game::from_save_json(common::zoo(), &json).unwrap();
    assert_eq!(r.to_save(), g.to_save());
    assert!(r.level_unlocked("level_3"));
    // v1 (level 1 only)
    let mut one = common::game(11);
    one.debug_send_home("zebra");
    let mut v1 = one.to_save();
    v1.version = 1;
    v1.bowl = None;
    let v1_json = v1.to_json();
    let m = Game::from_save_json(common::zoo(), &v1_json).expect("v1 migrates");
    assert!(m.mission("zebra").unwrap().complete);
    assert!(!m.mission("koala").unwrap().complete);
    assert_eq!(
        m.animal("hippo").unwrap().hiding_place,
        one.animal("hippo").unwrap().hiding_place
    );
    assert!(!m.level_unlocked("level_2"));
    // a v2 zoo save does not load into a single level (other level → new game, §5)
    assert!(Game::from_save_json(common::level1(), &json).is_err());
}

// Content coverage (RESC-003, RESC-017 for levels 2–3): every animal of the zoo has its
// riddles for every candidate, reading level and language, name, facts, home message; the
// goldfish its bowl texts — no raw key anywhere.
#[test]
fn content_all_animals_all_places_levels_languages() {
    let g = common::zoo_game(1);
    let c = common::content();
    let mut missing = Vec::new();
    for a in &g.animals {
        let animal = a.id();
        let places: Vec<String> = g
            .level
            .data
            .hiding_places_of(animal)
            .map(|h| h.id.clone())
            .collect();
        assert!(places.len() >= 3, "{animal}");
        for lang in Language::ALL {
            let mut keys = vec![
                format!("mission-{animal}-home"),
                format!("animal-{animal}"),
                format!("animal-{animal}-more"),
            ];
            for l in ReadingLevel::ALL {
                keys.push(zoo_core::content::facts_key(animal, l));
                for p in &places {
                    keys.push(riddle_key(animal, p, l));
                }
                if g.needs_container(animal) {
                    keys.push(format!("mission-{animal}-bowl-hint-{}", l.id()));
                }
            }
            if g.needs_container(animal) {
                for k in ["needs-bowl", "bowl-empty", "bowl-filled", "in-bowl"] {
                    keys.push(format!("mission-{animal}-{k}"));
                }
            }
            for f in a.info.foods {
                keys.push(f.label_key());
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

/// Zoo data with the zebra pair switched on (GAME-FAMILY data flag).
fn zebra_pair_zoo() -> zoo_core::LevelData {
    let mut zoo = common::zoo();
    let enc = zoo
        .elements
        .iter_mut()
        .find(|e| e.ty == ElementType::Enclosure && e.animal.as_deref() == Some("zebra"))
        .unwrap();
    enc.pair = true;
    zoo
}

// FAM-008: a liked special food given to a male + female pair at home makes exactly one baby;
// a single animal is only happy.
// FAM-009: ordinary/disliked food changes nothing; the baby is saved and never appears twice.
#[test]
fn fam_008_009_special_food_makes_one_baby_for_a_pair() {
    use zoo_core::garden::Treat;
    use zoo_core::GameEvent;
    let mut g = Game::new(zebra_pair_zoo(), 4).unwrap();
    g.debug_send_home("zebra");
    g.drain_events();
    g.garden.basket.carrots = 3;
    g.garden.basket.potatoes = 1;
    // a disliked treat: no baby, no penalty
    assert_eq!(g.give_treat("zebra", Treat::Potato), Some(false));
    assert!(g.babies.is_empty());
    // a liked carrot: happy, and one baby
    assert_eq!(g.give_treat("zebra", Treat::Carrot), Some(true));
    let ev = g.drain_events();
    assert_eq!(
        ev.iter()
            .filter(|e| matches!(e, GameEvent::BabyBorn { .. }))
            .count(),
        1
    );
    assert_eq!(g.babies, vec!["zebra".to_string()]);
    // never twice
    assert_eq!(g.give_treat("zebra", Treat::Carrot), Some(true));
    assert!(!g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::BabyBorn { .. })));
    // saved and restored
    let s = g.to_save();
    assert_eq!(s.babies, vec!["zebra".to_string()]);
    // a single animal (no pair): happy, no baby
    let mut zoo = common::zoo();
    for e in &mut zoo.elements {
        e.pair = false;
    }
    let mut single = Game::new(zoo, 4).unwrap();
    single.debug_send_home("zebra");
    single.garden.basket.carrots = 1;
    assert_eq!(single.give_treat("zebra", Treat::Carrot), Some(true));
    assert!(single.babies.is_empty());
}

// RESC-030: a split pair — one zebra already at home, the other still outside — the outside
// one still takes the right food and follows (the interaction addressed only the first member,
// which was at home, so the outside zebra "refused" to eat and follow).
#[test]
fn resc_030_split_pair_outside_member_follows_grass() {
    use zoo_core::Food;
    let mut g = Game::new(zebra_pair_zoo(), 4).unwrap();
    let group = g.group("zebra");
    assert_eq!(group.len(), 2);
    g.animals[group[0]].state = AnimalState::InEnclosure; // member 0 is home
    let outside = group[1];
    // stand next to the outside zebra with grass in the hands, facing it
    let a = g.animals[outside].pos;
    g.player.pos = a + glam::Vec2::new(0.0, -1.2);
    g.player.facing = glam::Vec2::Y;
    g.carry.take(&zoo_core::food::FoodBox { food: Food::Grass });
    assert_eq!(
        g.available_target(),
        Some(zoo_core::game::Target::Animal { animal: "zebra" })
    );
    g.interact().expect("interaction");
    assert_eq!(g.animals[outside].state, AnimalState::Following);
    assert_eq!(g.animals[group[0]].state, AnimalState::InEnclosure);
}
