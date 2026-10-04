//! GAME-GARDEN in the game (GARD-005, GARD-007, GARD-009) and the garden layout of level 1
//! (proposal Q-102: fence edges, self-opening gate, animals never enter).

mod common;

use glam::Vec2;
use zoo_core::game::{GameEvent, Interaction, Target};
use zoo_core::garden::{Stage, Treat};
use zoo_core::level::{cell_center, cell_of};
use zoo_core::scene::OpeningKind;
use zoo_core::{AnimalState, Content, Game, Language, ReadingLevel};

const DT: f32 = 1.0 / 60.0;

/// Puts the player on a plant's stand cell, facing the plant.
fn at_plant(g: &mut Game, spot: &str) {
    let s = g
        .level
        .data
        .plant_spots
        .iter()
        .find(|s| s.id == spot)
        .unwrap()
        .clone();
    let stand = cell_center(glam::IVec2::new(s.stand[0], s.stand[1]));
    g.player.pos = stand;
    g.player.facing = (s.pos() - stand).normalize();
}

#[test]
fn harvest_by_interacting_with_a_ripe_plant() {
    let mut g = common::game(1);
    at_plant(&mut g, "carrot_w2");
    assert_eq!(
        g.available_target(),
        Some(Target::Plant {
            spot: "carrot_w2".into()
        })
    );
    let r = g.interact().expect("harvest");
    assert!(matches!(
        r,
        Interaction::Harvest {
            count: 1,
            treat: Treat::Carrot,
            ..
        }
    ));
    assert_eq!(g.garden.basket.carrots, 1);
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::Harvested { .. })));
    // empty soil: no plant target any more
    assert_ne!(
        g.available_target(),
        Some(Target::Plant {
            spot: "carrot_w2".into()
        })
    );
    assert_eq!(g.garden.plant("carrot_w2").unwrap().stage(), Stage::Empty);
}

#[test]
fn gard_005_treat_at_the_fence_eaten_or_refused() {
    let mut g = common::game(1);
    assert!(g.debug_send_home("zebra"));
    g.drain_events();
    g.garden.basket.carrots = 1;
    g.garden.basket.potatoes = 1;
    // outside the zebra fence, next to the enclosure
    let r = g.level.data.element("enc_zebra").unwrap().rect;
    let fence = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 - 0.1);
    let stand = zoo_core::Rect::new(fence.x as i32 - 3, fence.y as i32 - 3, 7, 4)
        .cells()
        .filter(|&c| g.level.grid().is_walkable(c, false))
        .map(cell_center)
        .min_by(|a, b| a.distance(fence).total_cmp(&b.distance(fence)))
        .expect("a place outside the fence");
    g.player.pos = stand;
    g.player.facing = Vec2::Y;
    // the zebra is interested: it walks to the fence by itself and waits there (GARD-010)
    for _ in 0..(20.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        if g.available_target().is_some() {
            break;
        }
    }
    assert_eq!(
        g.available_target(),
        Some(Target::Treat { animal: "zebra" })
    );
    // a carrot: the zebra eats it
    assert_eq!(g.give_treat("zebra", Treat::Carrot), Some(true));
    assert_eq!(g.garden.basket.carrots, 0);
    // a potato: it refuses, the potato stays
    assert_eq!(g.give_treat("zebra", Treat::Potato), Some(false));
    assert_eq!(g.garden.basket.potatoes, 1);
    let ev = g.drain_events();
    assert!(ev.iter().any(|e| matches!(
        e,
        GameEvent::TreatEaten {
            treat: Treat::Carrot,
            ..
        }
    )));
    assert!(ev.iter().any(|e| matches!(
        e,
        GameEvent::TreatRefused {
            treat: Treat::Potato,
            ..
        }
    )));
    // treats never rescue: an escaped animal takes none
    assert_eq!(g.animal("hippo").unwrap().state, AnimalState::Escaped);
    assert_eq!(g.give_treat("hippo", Treat::Potato), None);
}

#[test]
fn gard_007_harvest_while_carrying_food() {
    let mut g = common::game(2);
    let food = g.food_boxes[0].0;
    g.carry.take(&zoo_core::FoodBox { food });
    at_plant(&mut g, "potato_e1");
    let r = g.interact().expect("harvest with full hands");
    assert!(matches!(
        r,
        Interaction::Harvest {
            treat: Treat::Potato,
            ..
        }
    ));
    assert!(g.garden.basket.potatoes >= 2);
    assert_eq!(g.carry.food(), Some(food), "the food stays in the hands");
}

#[test]
fn gard_009_garden_signs_read_from_fluent() {
    let c: Content = common::content();
    let g = common::game(1);
    assert_eq!(g.level.data.garden_beds.len(), 4);
    for b in &g.level.data.garden_beds {
        for lang in [Language::De, Language::En] {
            let word = c.text(lang, &b.sign_key).expect("sign word");
            assert!(!word.is_empty());
            for level in [
                ReadingLevel::Klasse1,
                ReadingLevel::Klasse2,
                ReadingLevel::Klasse3,
            ] {
                let key = format!("{}-{}", b.sign_key, level.id());
                assert!(c.text(lang, &key).is_some(), "{key} ({lang:?})");
            }
        }
    }
    assert_eq!(
        c.text(Language::De, "garden-carrot").as_deref(),
        Some("Karotten")
    );
    // the sign opens its panel like a board
    let mut g = common::game(1);
    let b = g.level.data.garden_beds[0].clone();
    let sp = Vec2::from(b.sign_pos);
    let f = zoo_core::level::facing_vec(&b.sign_facing);
    g.player.pos = sp + f * 1.0;
    g.player.facing = -f;
    assert!(matches!(g.interact(), Some(Interaction::GardenSign { .. })));
}

#[test]
fn q102_fence_blocks_edges_and_the_garden_is_entered_by_its_gate() {
    let g = common::game(1);
    let grid = g.level.grid();
    // fence along x = 10 (river bank): no step from the garden to (10, z)
    assert!(!grid.step_open(glam::IVec2::new(9, 38), glam::IVec2::new(10, 38)));
    // south fence west of the gate: (6, 35) → (6, 36) blocked, the gate cells are open
    assert!(!grid.step_open(glam::IVec2::new(6, 35), glam::IVec2::new(6, 36)));
    assert!(grid.step_open(glam::IVec2::new(7, 35), glam::IVec2::new(7, 36)));
    assert!(grid.step_open(glam::IVec2::new(8, 35), glam::IVec2::new(8, 36)));
    // diagonal past a fence corner is blocked too
    assert!(!grid.step_open(glam::IVec2::new(9, 35), glam::IVec2::new(10, 36)));
    // the garden gate opens by itself within 2 m (no button)
    let scene = zoo_core::scene::LevelScene::build(&g.level.data);
    let gate = scene
        .openings
        .iter()
        .find(|o| matches!(o.kind, OpeningKind::GardenGate { .. }))
        .expect("garden gate");
    let mut g = g;
    g.player.pos = gate.center + Vec2::new(0.0, -1.5);
    assert!(g.opening_open(gate));
    g.player.pos = gate.center + Vec2::new(0.0, -3.0);
    assert!(!g.opening_open(gate));
}

#[test]
fn q102_following_animals_wait_outside_the_garden() {
    let mut g = common::game(1);
    // the zebra follows the child into the garden
    let z = g.animal_index("zebra").unwrap();
    g.animals[z].state = AnimalState::Following;
    let garden = g.level.data.gardens[0].clone();
    let gate = garden.gate_center();
    g.animals[z].pos = gate + Vec2::new(0.0, -2.5);
    g.player.pos = gate + Vec2::new(0.0, -1.0);
    g.player.facing = Vec2::Y;
    // she walks through the gate up the garden path; the zebra keeps following her
    let mut max_follow = 0.0f32;
    for _ in 0..(4.0 / DT) as usize {
        g.update(DT, Vec2::Y);
        max_follow = max_follow.max(g.animal("zebra").unwrap().pos.y);
    }
    assert!(
        g.level.data.gardens[0].rect.contains(cell_of(g.player.pos)),
        "she is inside: {}",
        g.player.pos
    );
    for _ in 0..(10.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
    assert!(
        !g.animal("zebra").unwrap().waiting,
        "still following (not left behind)"
    );
    let a = g.animal("zebra").unwrap();
    assert!(
        !garden.rect.contains(cell_of(a.pos)),
        "the zebra stays out of the garden: {}",
        a.pos
    );
    assert!(
        a.pos.distance(gate) < 2.0,
        "it waits at the gate: {}",
        a.pos
    );
    assert!(max_follow > gate.y - 3.0, "it came along to the gate");
}

/// Puts the player inside the zebra enclosure 1.2 m from the first zebra, facing it.
fn beside_home_zebra(g: &mut Game) {
    let z = g.animal("zebra").unwrap().pos;
    let i = g.animal_index("zebra").unwrap();
    g.animals[i].wander = Default::default();
    g.player.pos = z + Vec2::new(0.0, -1.2);
    g.player.facing = Vec2::Y;
}

// GARD-010: a home animal can be given something right at the animal (inside the enclosure),
// not only at the fence; both of a pair look at the child.
#[test]
fn gard_010_give_directly_at_the_animal() {
    let mut g = common::game(1);
    assert!(g.debug_send_home("zebra"));
    g.drain_events();
    g.garden.basket.carrots = 2;
    beside_home_zebra(&mut g);
    assert_eq!(
        g.available_target(),
        Some(Target::Treat { animal: "zebra" })
    );
    let r = g.interact().expect("gives");
    assert!(matches!(
        r,
        Interaction::Treat {
            animal: "zebra",
            accepted: true,
            ..
        }
    ));
    assert_eq!(g.garden.basket.carrots, 1);
    for j in g.group("zebra") {
        let a = &g.animals[j];
        let to = (g.player.pos - a.pos).normalize();
        assert!(a.facing.dot(to) > 0.99, "every group member turns to her");
    }
    // nothing in the basket and nothing in the hands: nothing to give, no prompt
    g.garden.basket.carrots = 0;
    assert_eq!(g.available_target(), None);
}

// GARD-011: the carried food of the animal given at home: it eats, the food stays in the
// hands, no baby (Q-198); another food is refused gently (RESC-005).
#[test]
fn gard_011_carried_food_at_home() {
    use zoo_core::FoodBox;
    let mut g = common::game(1);
    assert!(g.debug_send_home("zebra"));
    g.drain_events();
    let own = g.animal("zebra").unwrap().info.foods[0];
    g.carry.take(&FoodBox { food: own });
    beside_home_zebra(&mut g);
    assert_eq!(
        g.available_target(),
        Some(Target::Treat { animal: "zebra" })
    );
    assert!(matches!(
        g.interact(),
        Some(Interaction::FoodGift { accepted: true, .. })
    ));
    assert_eq!(g.carry.food(), Some(own), "the food stays in the hands");
    let ev = g.drain_events();
    assert!(ev.iter().any(|e| matches!(e, GameEvent::FoodEaten { .. })));
    assert!(!ev.iter().any(|e| matches!(e, GameEvent::BabyBorn { .. })));
    // a food the zebra does not eat: refused, stays in the hands
    let other = zoo_core::ANIMALS
        .iter()
        .flat_map(|a| a.foods.iter().copied())
        .find(|f| !g.animal("zebra").unwrap().info.eats(*f))
        .unwrap();
    g.carry.take(&FoodBox { food: other });
    assert!(matches!(
        g.interact(),
        Some(Interaction::FoodGift {
            accepted: false,
            ..
        })
    ));
    assert_eq!(g.carry.food(), Some(other));
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::FoodRefused { .. })));
}

// ------------------------------------------------------------------ fruit garden of level 3
// (user request 2026-10-01, Q-320…Q-324): apples and oranges, GARD-015…GARD-023.

/// The joined zoo with the day levels open (the barriers open the next morning).
fn open_zoo(seed: u64) -> Game {
    let mut g = common::zoo_game(seed);
    for a in [
        "zebra", "hippo", "panda", "koala", "elephant", "giraffe", "lion",
    ] {
        assert!(g.debug_send_home(a), "{a}");
    }
    g.debug_next_morning();
    g.drain_events();
    assert!(g.level_unlocked("level_3"));
    g
}

fn rect_overlap(a: zoo_core::Rect, b: zoo_core::Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.z < b.z + b.d && b.z < a.z + a.d
}

// GARD-018 / LAYOUT-L3-030: the fruit garden of level 3, its data and its place in the zoo.
#[test]
fn gard_018_fruit_garden_layout_and_reachability() {
    use zoo_core::level::{ElementType, Level};
    use zoo_core::nav::flood_fill;
    let data = common::zoo();
    let garden = data
        .gardens
        .iter()
        .find(|g| g.id == "garden_fruit")
        .expect("garden_fruit");
    let spots: Vec<_> = data
        .plant_spots
        .iter()
        .filter(|s| s.garden == "garden_fruit")
        .collect();
    assert_eq!(spots.iter().filter(|s| s.kind == "apple").count(), 2);
    assert_eq!(spots.iter().filter(|s| s.kind == "orange").count(), 2);
    let beds: Vec<_> = data
        .garden_beds
        .iter()
        .filter(|b| b.garden == "garden_fruit")
        .collect();
    assert_eq!(beds.len(), 2);
    for (plant, key) in [("apple", "garden-apple"), ("orange", "garden-orange")] {
        let b = beds.iter().find(|b| b.plant == plant).expect(plant);
        assert_eq!(b.sign_key, key);
        // the sign stands in the garden, off its fence
        let p = Vec2::from(b.sign_pos);
        let r = garden.rect;
        assert!(
            p.x > r.x as f32 + 0.6
                && p.x < (r.x + r.w) as f32 - 0.6
                && p.y > r.z as f32 + 0.6
                && p.y < (r.z + r.d) as f32 - 0.6,
            "{key}: sign {p:?} in the garden, 0.6 m off the fence"
        );
    }
    // no overlap with solid elements, hiding places, scenery, barriers or ad boards
    let r = garden.rect;
    for e in &data.elements {
        if e.ty.is_solid() || e.ty == ElementType::Barrier {
            assert!(!rect_overlap(r, e.rect), "garden overlaps {}", e.id);
        }
    }
    for h in &data.hiding_places {
        assert!(!rect_overlap(r, h.rect), "garden overlaps {}", h.id);
    }
    for sc in &data.scenery {
        assert!(!rect_overlap(r, sc.rect), "garden overlaps {}", sc.id);
    }
    for ad in &data.ad_boards {
        let p = Vec2::from(ad.pos);
        let area =
            zoo_core::Rect::new((p.x - 1.5).floor() as i32, (p.y - 0.5).floor() as i32, 4, 2);
        assert!(!rect_overlap(r, area), "garden overlaps ad board {}", ad.id);
    }
    // the gate touches the street: a path cell (not a garden path) outside the gate (rule 8)
    let out = garden.gate_out();
    let gate = garden.gate_center() + out * 0.5;
    let street = zoo_core::level::cell_of(gate);
    assert!(
        data.elements.iter().any(|e| e.ty == ElementType::Path
            && e.kind.as_deref() != Some("garden")
            && e.rect.contains(street)),
        "street cell {street:?} outside the gate"
    );
    // reachable from the level-3 spawn; every stand cell walkable, close to its tree
    let mut level = Level::new(data.clone());
    for b in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        assert!(level.open_barrier(b), "{b}");
    }
    let k = data.part_index("level_3").unwrap();
    let reach = flood_fill(level.grid(), data.parts[k].spawn.cell(), false);
    let reached = |c: glam::IVec2| level.grid().index(c).is_some_and(|i| reach[i]);
    assert!(reached(street), "the gate's street cell is reachable");
    for s in &spots {
        let stand = glam::IVec2::new(s.stand[0], s.stand[1]);
        assert!(level.grid().is_walkable(stand, false), "{} stand", s.id);
        assert!(reached(stand), "{}: stand reachable from the spawn", s.id);
        let d = cell_center(stand).distance(s.pos());
        assert!(d <= 1.2, "{}: stand {d} m from the tree", s.id);
    }
}

// GARD-015 / GARD-016 in the game: harvest an apple and an orange by interacting.
#[test]
fn gard_016_harvest_fruit_by_interacting() {
    let mut g = open_zoo(3);
    for (spot, treat) in [("apple_1", Treat::Apple), ("orange_2", Treat::Orange)] {
        at_plant(&mut g, spot);
        assert_eq!(
            g.available_target(),
            Some(Target::Plant { spot: spot.into() })
        );
        let r = g.interact().expect("harvest");
        assert!(
            matches!(r, Interaction::Harvest { treat: t, count: 1, .. } if t == treat),
            "{spot}"
        );
        assert_eq!(g.garden.basket.count(treat), 1);
        assert_eq!(g.garden.plant(spot).unwrap().stage(), Stage::Empty);
        assert!(g.available_target() != Some(Target::Plant { spot: spot.into() }));
    }
    assert!(g.drain_events().iter().any(|e| matches!(
        e,
        GameEvent::Harvested {
            treat: Treat::Apple,
            count: 1,
            ..
        }
    )));
    // the full basket refuses (6 in total over all kinds) and the tree keeps its fruit
    g.garden.basket.carrots = 4;
    assert_eq!(g.garden.basket.total(), 6);
    at_plant(&mut g, "apple_2");
    g.interact();
    assert_eq!(g.garden.plant("apple_2").unwrap().stage(), Stage::Ripe);
    assert!(g.drain_events().contains(&GameEvent::BasketFull));
}

// GARD-017: the saved game keeps all four treats; a save from before the fruit garden loads.
#[test]
fn gard_017_save_round_trip_with_fruit_and_old_saves() {
    let mut g = open_zoo(3);
    g.garden.basket.carrots = 1;
    g.garden.basket.apples = 2;
    g.garden.basket.oranges = 1;
    g.garden.update(1.0);
    let json = serde_json::to_string(&g.to_save()).unwrap();
    let back: zoo_core::save::SaveState = serde_json::from_str(&json).unwrap();
    let fresh = Game::from_save(common::zoo(), &back).expect("save loads");
    assert_eq!(fresh.garden.basket, g.garden.basket);
    // an old save: the basket JSON only has carrots and potatoes
    let mut v: serde_json::Value = serde_json::from_str(&json).unwrap();
    v["garden"]["basket"] = serde_json::json!({"carrots": 2, "potatoes": 1});
    let old: zoo_core::save::SaveState = serde_json::from_value(v).unwrap();
    let fresh = Game::from_save(common::zoo(), &old).expect("old save loads");
    let b = fresh.garden.basket;
    assert_eq!((b.carrots, b.potatoes, b.apples, b.oranges), (2, 1, 0, 0));
}

// GARD-019: the fruit signs read from Fluent in every reading level and both languages.
#[test]
fn gard_019_fruit_signs_read_from_fluent() {
    let c: Content = common::content();
    for (key, de, en) in [
        ("garden-apple", "Äpfel", "Apples"),
        ("garden-orange", "Orangen", "Oranges"),
    ] {
        assert_eq!(c.text(Language::De, key).as_deref(), Some(de));
        assert_eq!(c.text(Language::En, key).as_deref(), Some(en));
        for lang in [Language::De, Language::En] {
            for level in [
                ReadingLevel::Klasse1,
                ReadingLevel::Klasse2,
                ReadingLevel::Klasse3,
            ] {
                let k = format!("{key}-{}", level.id());
                assert!(c.text(lang, &k).is_some_and(|t| !t.is_empty()), "{k}");
            }
        }
    }
    // the sign opens its panel like a board
    let mut g = open_zoo(1);
    let b = g
        .level
        .data
        .garden_beds
        .iter()
        .find(|b| b.id == "bed_apple")
        .unwrap()
        .clone();
    let sp = Vec2::from(b.sign_pos);
    let f = zoo_core::level::facing_vec(&b.sign_facing);
    g.player.pos = sp + f * 1.0;
    g.player.facing = -f;
    assert!(matches!(g.interact(), Some(Interaction::GardenSign { .. })));
}

// GARD-020: fruit to the monkey pair at home — they eat (hearts), one baby (FAM-008); a potato
// is refused and stays.
#[test]
fn gard_020_monkeys_eat_apples_and_oranges_and_refuse_potatoes() {
    let mut g = open_zoo(4);
    assert!(g.debug_send_home("monkey"));
    g.drain_events();
    assert_eq!(g.group("monkey").len(), 2, "the monkeys come as a pair");
    g.garden.basket.potatoes = 1;
    g.garden.basket.apples = 2;
    g.garden.basket.oranges = 1;
    assert_eq!(g.give_treat("monkey", Treat::Potato), Some(false));
    assert_eq!(g.garden.basket.potatoes, 1, "the potato stays");
    assert!(g.babies.is_empty());
    assert_eq!(g.give_treat("monkey", Treat::Apple), Some(true));
    assert_eq!(g.garden.basket.apples, 1, "the apple leaves the basket");
    let ev = g.drain_events();
    assert!(ev.iter().any(
        |e| matches!(e, GameEvent::TreatEaten { animal, treat: Treat::Apple } if animal == "monkey")
    ));
    assert_eq!(
        ev.iter()
            .filter(|e| matches!(e, GameEvent::BabyBorn { .. }))
            .count(),
        1,
        "one baby"
    );
    assert_eq!(g.give_treat("monkey", Treat::Orange), Some(true));
    assert_eq!(g.garden.basket.oranges, 0);
    assert!(!g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::BabyBorn { .. })));
}

// GARD-021: the treat hint and the gift at the animal work with the fruit.
#[test]
fn gard_021_treat_hint_and_gift_for_the_monkeys() {
    use zoo_core::game::Gift;
    use zoo_core::hints::{candidates, HintKind, HintTracker};
    let mut g = open_zoo(4);
    assert!(g.debug_send_home("monkey"));
    let has = |g: &Game| {
        candidates(g, &HintTracker::default())
            .iter()
            .any(|h| h.kind == HintKind::Treat && h.animal == Some("monkey"))
    };
    assert!(!has(&g), "nothing to give");
    g.garden.basket.potatoes = 1;
    assert!(!has(&g), "monkeys do not like potatoes");
    g.garden.basket.potatoes = 0;
    g.garden.basket.apples = 1;
    assert!(g.gift_liked("monkey"));
    assert!(matches!(
        g.gift_for("monkey"),
        Some(Gift::Treat(Treat::Apple))
    ));
    assert!(has(&g), "a liked apple");
    g.garden.basket.apples = 0;
    g.garden.basket.oranges = 1;
    assert!(has(&g), "a liked orange");
    // the treat the child chose is offered first
    g.garden.basket.apples = 1;
    g.treat_choice = Some(Treat::Orange);
    assert!(matches!(
        g.gift_for("monkey"),
        Some(Gift::Treat(Treat::Orange))
    ));
    // an animal that does not like fruit (lion) gets no treat hint for it
    assert!(!zoo_core::garden::likes("lion", Treat::Apple));
}

// GARD-022 (unit part): fruit treats of a spot are saved and the garden state survives a save
// in the middle of regrowth.
#[test]
fn gard_regrow_fruit_in_three_steps_and_save() {
    let mut g = open_zoo(5);
    at_plant(&mut g, "orange_1");
    g.interact().expect("harvest");
    g.garden.update(70.0);
    assert_eq!(g.garden.plant("orange_1").unwrap().stage(), Stage::Sprout);
    let s = g.to_save();
    let mut fresh = Game::from_save(common::zoo(), &s).expect("save loads");
    assert_eq!(
        fresh.garden.plant("orange_1").unwrap().stage(),
        Stage::Sprout
    );
    fresh.garden.update(120.0);
    assert_eq!(fresh.garden.plant("orange_1").unwrap().stage(), Stage::Ripe);
}

// GARD-024 (user report 2026-10-03: the target indicator always showed carrots): the garden
// hint shows WHAT grows at its target (apple / orange / potato / carrot icon kinds) and, in
// level 3, prefers the fruit garden of the level the child stands in.
#[test]
fn gard_024_garden_hint_shows_the_fruit_of_the_level_the_child_is_in() {
    use zoo_core::hints::{candidates, HintKind, HintTracker};
    let data = common::zoo();
    let mut g = Game::new(data.clone(), 3).unwrap();
    for b in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        g.level.open_barrier(b);
    }
    // stand in level 3 (where the apple and orange trees are)
    let k = data.part_index("level_3").unwrap();
    g.player.pos = zoo_core::level::cell_center(data.parts[k].spawn.cell());
    // a plant hint is a task only while a pair at home still wants the treat for its baby
    // (HINT-026): the monkeys like apples and oranges
    assert!(g.debug_send_home("monkey"));
    let c = candidates(&g, &HintTracker::default());
    let garden: Vec<_> = c.iter().filter(|h| h.id.starts_with("plant:")).collect();
    assert!(!garden.is_empty(), "plants are offered");
    let first = garden[0];
    assert!(
        matches!(first.kind, HintKind::Apple | HintKind::Orange),
        "level 3: the first garden target is a fruit tree, got {:?} {}",
        first.kind,
        first.id
    );
    // every plant hint carries the icon of what grows there
    for h in &garden {
        let want = match h.id.as_str() {
            s if s.contains("apple") => HintKind::Apple,
            s if s.contains("orange") => HintKind::Orange,
            s if s.contains("potato") => HintKind::Potato,
            _ => HintKind::Garden,
        };
        assert_eq!(h.kind, want, "{}", h.id);
    }
}

// FAM-030 (user report 2026-10-03: the snow fox did not accept food to make a baby): species
// that like no garden treat get their baby from their own favourite food at home (once).
#[test]
fn fam_030_species_without_a_garden_treat_get_a_baby_from_their_own_food() {
    use zoo_core::{Food, GameEvent};
    for species in ["snow_fox", "koala", "lion"] {
        let mut g = common::zoo_game(3);
        let group = g.group(species);
        assert_eq!(group.len(), 2, "{species}");
        assert!(g.debug_send_home(species));
        g.drain_events();
        let food = g.animals[group[0]].info.foods[0];
        g.carry.take(&zoo_core::FoodBox { food: food as Food });
        assert_eq!(g.give_food(species), Some(true), "{species}");
        let ev = g.drain_events();
        assert_eq!(
            ev.iter()
                .filter(|e| matches!(e, GameEvent::BabyBorn { .. }))
                .count(),
            1,
            "{species}: baby"
        );
        // once only
        assert_eq!(g.give_food(species), Some(true));
        assert!(!g
            .drain_events()
            .iter()
            .any(|e| matches!(e, GameEvent::BabyBorn { .. })));
    }
    // a species WITH a liked garden treat does not get a baby from its box food
    let mut g = common::zoo_game(3);
    assert!(g.debug_send_home("zebra"));
    g.drain_events();
    g.carry.take(&zoo_core::FoodBox { food: Food::Grass });
    assert_eq!(g.give_food("zebra"), Some(true));
    assert!(!g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::BabyBorn { .. })));
}

// GARD-025 (user report 2026-10-03): what the child already carries is not a task — with apples
// in the basket the garden hint no longer sends her to the apple trees (oranges still).
#[test]
fn gard_025_no_hint_to_fetch_what_is_already_in_the_basket() {
    use zoo_core::garden::Treat;
    use zoo_core::hints::{candidates, HintKind, HintTracker};
    let data = common::zoo();
    let mut g = Game::new(data.clone(), 3).unwrap();
    for b in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        g.level.open_barrier(b);
    }
    let k = data.part_index("level_3").unwrap();
    g.player.pos = zoo_core::level::cell_center(data.parts[k].spawn.cell());
    // a plant hint is a task only while a pair at home still wants the treat for its baby
    // (HINT-026): the monkeys like apples and oranges
    assert!(g.debug_send_home("monkey"));
    let kinds = |g: &Game| -> Vec<HintKind> {
        candidates(g, &HintTracker::default())
            .iter()
            .filter(|h| h.id.starts_with("plant:"))
            .map(|h| h.kind)
            .collect()
    };
    assert!(kinds(&g).contains(&HintKind::Apple));
    g.garden.basket.add(Treat::Apple, 1);
    let after = kinds(&g);
    assert!(!after.contains(&HintKind::Apple), "{after:?}");
    assert!(
        after.contains(&HintKind::Orange),
        "oranges are still offered: {after:?}"
    );
}
