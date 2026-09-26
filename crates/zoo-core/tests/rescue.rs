//! GAME-RESCUE, GAME-ANIMALS and GAME-FEED tests on level 1.

mod common;

use glam::{IVec2, Vec2};
use zoo_core::content::riddle_key;
use zoo_core::game::InteractError;
use zoo_core::level::{cell_center, cell_of, CellKind, ElementType};
use zoo_core::{nav, AnimalState, Food, Game, GameEvent, Language, ReadingLevel, ANIMALS};

const DT: f32 = 1.0 / 60.0;

/// Walks the player along the shortest grid path to `target` with joystick input, like a
/// scripted child. Panics after `timeout` seconds.
fn walk_to(g: &mut Game, target: IVec2, timeout: f32) {
    let mut t = 0.0;
    let mut path: Vec<IVec2> = Vec::new();
    while cell_of(g.player.pos) != target || g.player.pos.distance(cell_center(target)) > 0.05 {
        let here = cell_of(g.player.pos);
        let next = if here == target {
            target
        } else {
            if !path.contains(&here) {
                let leading = g.is_leading();
                let grid = g.level.grid();
                // standing on a gate that just closed: start from a walkable neighbour
                let start = if grid.is_walkable(here, leading) {
                    here
                } else {
                    nav::neighbours(grid, here, leading)
                        .next()
                        .map(|(c, _)| c)
                        .unwrap_or(here)
                };
                path = nav::find_path(grid, start, target, leading)
                    .unwrap_or_else(|| panic!("no path to {target} from {}", g.player.pos));
                if start != here {
                    path.insert(0, here);
                }
            }
            let k = path.iter().position(|&c| c == here).unwrap();
            path[(k + 1).min(path.len() - 1)]
        };
        let dir = (cell_center(next) - g.player.pos).normalize_or_zero();
        g.update(DT, dir);
        t += DT;
        assert!(
            t < timeout,
            "walk to {target} timed out at {}",
            g.player.pos
        );
    }
}

fn idle(g: &mut Game, seconds: f32) {
    for _ in 0..(seconds / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
}

fn spot(g: &Game, animal: &str) -> IVec2 {
    let a = g.animal(animal).unwrap();
    g.level
        .data
        .element(&a.hiding_place)
        .unwrap()
        .animal_spot_cell()
        .unwrap()
}

fn gate(g: &Game, enclosure: &str) -> Vec<IVec2> {
    g.level
        .data
        .element(enclosure)
        .unwrap()
        .gate
        .unwrap()
        .cells()
        .collect()
}

/// Walkable cell (outside gates) within 2 m of the animal's spot, closest to the spawn side.
fn cell_near_spot(g: &Game, animal: &str) -> IVec2 {
    let s = cell_center(spot(g, animal));
    g.level
        .data
        .level
        .bounds
        .cells()
        .filter(|&c| g.level.grid().is_walkable(c, false) && cell_center(c).distance(s) <= 2.0)
        .min_by(|a, b| {
            cell_center(*a)
                .distance(s)
                .total_cmp(&cell_center(*b).distance(s))
        })
        .unwrap()
}

/// Walkable cell outside the enclosure next to its first gate cell.
fn before_gate(g: &Game, enclosure: &str) -> IVec2 {
    let gc = gate(g, enclosure)[0];
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .into_iter()
        .map(|d| gc + d)
        .find(|&c| g.level.grid().is_walkable(c, false))
        .unwrap()
}

/// Walkable cell in front of a food box's label (GAME-FEED §7).
fn box_front(g: &Game, food: Food) -> IVec2 {
    let &(_, pos, facing) = g.food_boxes.iter().find(|b| b.0 == food).unwrap();
    cell_of(pos + facing * 1.1)
}

fn get_food(g: &mut Game, food: Food) {
    walk_to(g, box_front(g, food), 120.0);
    g.take_food(food).unwrap();
}

fn make_follow(g: &mut Game, animal: &str, food: Food) {
    get_food(g, food);
    walk_to(g, cell_near_spot(g, animal), 120.0);
    // step up to the animal as far as the ground allows
    for _ in 0..60 {
        let to = g.animal(animal).unwrap().pos - g.player.pos;
        if to.length() < 1.8 {
            break;
        }
        g.update(DT, to.normalize());
    }
    g.show_food(animal).unwrap();
    assert_eq!(g.animal(animal).unwrap().state, AnimalState::Following);
}

fn lead_home(g: &mut Game, animal: &str) {
    let enc = format!("enc_{animal}");
    walk_to(g, before_gate(g, &enc), 120.0);
    idle(g, 2.0);
    walk_to(g, gate(g, &enc)[0], 30.0);
}

fn has(events: &[GameEvent], e: &GameEvent) -> bool {
    events.contains(e)
}

fn zebra(e: fn(String) -> GameEvent) -> GameEvent {
    e("zebra".to_owned())
}

// RESC-001
#[test]
fn resc_001_new_game_enclosures_empty_animals_hidden() {
    let g = common::game(7);
    for enc in g.level.data.elements_of(ElementType::Enclosure) {
        assert!(
            g.enclosure_occupants(&enc.id).is_empty(),
            "{} occupied",
            enc.id
        );
    }
    assert_eq!(g.animals.len(), 3);
    for a in &g.animals {
        assert_eq!(a.state, AnimalState::Escaped);
        let h = g.level.data.element(&a.hiding_place).unwrap();
        assert_eq!(h.ty, ElementType::HidingPlace);
        assert_eq!(h.animal.as_deref(), Some(a.id()));
        assert_eq!(a.pos, cell_center(h.animal_spot_cell().unwrap()));
    }
}

/// Level 1 with a second (synthetic) zebra hiding place, so the seeded choice matters.
fn level_with_two_zebra_places() -> zoo_core::LevelData {
    let mut data = common::level1();
    let mut extra = data.element("loc_river").unwrap().clone();
    extra.id = "loc_test".into();
    extra.animal_spot = Some([-21, 3]);
    data.elements.push(extra);
    data
}

// RESC-002
#[test]
fn resc_002_same_seed_same_hiding_place() {
    let pick = |seed| {
        Game::new(level_with_two_zebra_places(), seed)
            .unwrap()
            .animal("zebra")
            .unwrap()
            .hiding_place
            .clone()
    };
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        assert_eq!(pick(seed), pick(seed));
        seen.insert(pick(seed));
    }
    assert_eq!(seen.len(), 2, "both places are chosen for some seed");
}

// RESC-004, FEED-004
#[test]
fn resc_004_feed_004_grass_makes_zebras_follow_and_is_kept() {
    let mut g = common::game(1);
    make_follow(&mut g, "zebra", Food::Grass);
    assert!(has(
        &g.drain_events(),
        &zebra(|animal| GameEvent::StartedFollowing { animal })
    ));
    assert_eq!(g.carry.food(), Some(Food::Grass));
}

// RESC-005
#[test]
fn resc_005_bamboo_not_interested() {
    let mut g = common::game(1);
    get_food(&mut g, Food::Bamboo);
    let target = cell_near_spot(&g, "zebra");
    walk_to(&mut g, target, 120.0);
    g.drain_events();
    g.show_food("zebra").unwrap();
    assert_eq!(g.animal("zebra").unwrap().state, AnimalState::Escaped);
    assert!(has(
        &g.drain_events(),
        &zebra(|animal| GameEvent::NotInterested { animal })
    ));
    // nothing carried: not interested either
    let mut g = common::game(1);
    g.player.pos = cell_center(cell_near_spot(&g, "zebra"));
    g.show_food("zebra").unwrap();
    assert_eq!(g.animal("zebra").unwrap().state, AnimalState::Escaped);
}

#[test]
fn show_food_needs_interaction_range() {
    let mut g = common::game(1);
    get_food(&mut g, Food::Grass);
    assert_eq!(g.show_food("zebra"), Err(InteractError::OutOfRange));
}

// RESC-006
#[test]
fn resc_006_wait_beyond_15m_follow_again_within_5m() {
    let mut g = common::game(1);
    make_follow(&mut g, "zebra", Food::Grass);
    // follows the player along the ring (walks around the grove)
    walk_to(&mut g, IVec2::new(0, 28), 60.0);
    idle(&mut g, 3.0);
    let z = g.animal("zebra").unwrap();
    assert!(
        !z.waiting && z.pos.distance(g.player.pos) < 2.5,
        "zebra at {}",
        z.pos
    );
    // player 20 m away → zebras wait
    let zpos = z.pos;
    g.player.pos = zpos + Vec2::new(-20.0, 0.0);
    g.update(DT, Vec2::ZERO);
    assert!(g.animal("zebra").unwrap().waiting);
    assert!(has(
        &g.drain_events(),
        &zebra(|animal| GameEvent::Waiting { animal })
    ));
    idle(&mut g, 2.0);
    assert_eq!(g.animal("zebra").unwrap().pos, zpos, "waiting zebras stay");
    // 8 m: still waiting (resume only within 5 m)
    g.player.pos = zpos + Vec2::new(-8.0, 0.0);
    idle(&mut g, 1.0);
    assert_eq!(g.animal("zebra").unwrap().pos, zpos);
    // back within 5 m → follow again
    g.player.pos = zpos + Vec2::new(-4.0, 0.0);
    idle(&mut g, 2.0);
    let z = g.animal("zebra").unwrap();
    assert!(!z.waiting && z.state == AnimalState::Following);
    assert!(z.pos.distance(g.player.pos) < 2.0);
    assert!(has(
        &g.drain_events(),
        &zebra(|animal| GameEvent::FollowingAgain { animal })
    ));
}

// RESC-007
#[test]
fn resc_007_wrong_enclosure_refuse() {
    let mut g = common::game(1);
    make_follow(&mut g, "zebra", Food::Grass);
    let panda_gate = gate(&g, "enc_panda");
    let target = before_gate(&g, "enc_panda");
    walk_to(&mut g, target, 60.0);
    idle(&mut g, 2.0);
    g.drain_events();
    walk_to(&mut g, panda_gate[0], 10.0);
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::Refuse {
        animal: "zebra".into(),
        enclosure: "enc_panda".into()
    }));
    let z = g.animal("zebra").unwrap().clone();
    assert_eq!(z.state, AnimalState::Following);
    assert!(z.refusing);
    idle(&mut g, 2.0);
    let z2 = g.animal("zebra").unwrap();
    assert_eq!(z2.pos, z.pos, "zebras stop at the gate");
    assert!(!matches!(
        g.level.grid().kind(cell_of(z2.pos)),
        CellKind::Gate(_)
    ));
    assert!(g.enclosure_occupants("enc_panda").is_empty());
    assert_eq!(g.carry.food(), Some(Food::Grass));
}

// RESC-008, FEED-004 (food only consumed at home), GAME-RESCUE end-to-end on unit level
#[test]
fn resc_008_zebra_mission_end_to_end() {
    let mut g = common::game(3);
    g.settings.reading_level = ReadingLevel::Klasse1;
    walk_to(&mut g, IVec2::new(-8, 14), 60.0); // next to board_zebra
    let board = g.read_info_board("zebra").unwrap();
    assert_eq!(board.riddle_key, "mission-zebra-riddle-klasse1");
    assert_eq!(board.food_key, "food-grass");
    make_follow(&mut g, "zebra", Food::Grass);
    g.drain_events();
    lead_home(&mut g, "zebra");
    let ev = g.drain_events();
    assert_eq!(g.animal("zebra").unwrap().state, AnimalState::InEnclosure);
    assert_eq!(g.carry.food(), None);
    assert!(ev.contains(&GameEvent::FoodConsumed { food: Food::Grass }));
    assert!(has(&ev, &zebra(|animal| GameEvent::InEnclosure { animal })));
    assert!(has(
        &ev,
        &zebra(|animal| GameEvent::MissionComplete { animal })
    ));
    assert!(g.mission("zebra").unwrap().complete);
    assert_eq!(g.enclosure_occupants("enc_zebra"), vec!["zebra"]);
    assert!(!ev.contains(&GameEvent::AllAnimalsHome));
    // the gate closes again for the player (no animal led)
    assert!(!g.is_leading());
}

// RESC-009, LAYOUT-L1-010
#[test]
fn resc_009_l1_010_all_home_opens_ne_barrier() {
    let mut g = common::game(5);
    let mut events = Vec::new();
    for (animal, food) in [
        ("zebra", Food::Grass),
        ("hippo", Food::Melons),
        ("panda", Food::Bamboo),
    ] {
        make_follow(&mut g, animal, food);
        lead_home(&mut g, animal);
        assert_eq!(
            g.animal(animal).unwrap().state,
            AnimalState::InEnclosure,
            "{animal}"
        );
        events.extend(g.drain_events());
    }
    assert_eq!(
        events
            .iter()
            .filter(|e| **e == GameEvent::AllAnimalsHome)
            .count(),
        1
    );
    assert!(events.contains(&GameEvent::BarrierOpened {
        id: "barrier_ne_tree".into()
    }));
    let grid = g.level.grid();
    let r = |id: &str| g.level.data.element(id).unwrap().rect;
    assert!(r("barrier_ne_tree")
        .cells()
        .all(|c| grid.is_walkable(c, false)));
    for id in ["barrier_north_gate", "barrier_east_repair"] {
        assert!(
            r(id).cells().all(|c| grid.kind(c) == CellKind::Solid),
            "{id}"
        );
    }
}

// RESC-012
#[test]
fn resc_012_reading_board_starts_mission() {
    let mut g = common::game(1);
    assert!(!g.mission("zebra").unwrap().started);
    assert_eq!(g.read_info_board("zebra"), Err(InteractError::OutOfRange));
    g.player.pos = cell_center(IVec2::new(-8, 14));
    g.read_info_board("zebra").unwrap();
    assert!(g.mission("zebra").unwrap().started);
    assert!(has(
        &g.drain_events(),
        &zebra(|animal| GameEvent::MissionStarted { animal })
    ));
    g.read_info_board("zebra").unwrap();
    assert!(g.drain_events().is_empty(), "started only once");
}

// RESC-013
#[test]
fn resc_013_follow_without_reading_board() {
    let mut g = common::game(1);
    make_follow(&mut g, "zebra", Food::Grass);
    assert!(!g.mission("zebra").unwrap().started);
}

// GAME-RESCUE §5 / Q-041 proposal: a second group is not interested while one follows
#[test]
fn only_one_group_follows() {
    let mut g = common::game(1);
    make_follow(&mut g, "zebra", Food::Grass);
    g.drain_events();
    get_food(&mut g, Food::Bamboo);
    let target = cell_near_spot(&g, "panda");
    walk_to(&mut g, target, 120.0);
    g.show_food("panda").unwrap();
    assert_eq!(g.animal("panda").unwrap().state, AnimalState::Escaped);
    assert!(g.drain_events().contains(&GameEvent::NotInterested {
        animal: "panda".into()
    }));
}

// ANIM-001
#[test]
fn anim_001_every_animal_has_food_and_hiding_place() {
    for a in ANIMALS {
        assert!(
            !a.foods.is_empty() && !a.hiding_places.is_empty(),
            "{}",
            a.id
        );
    }
}

// ANIM-002
#[test]
fn anim_002_in_enclosure_is_final() {
    let mut g = common::game(3);
    make_follow(&mut g, "zebra", Food::Grass);
    lead_home(&mut g, "zebra");
    let pos = g.animal("zebra").unwrap().pos;
    for f in Food::ALL {
        g.carry.take(&zoo_core::FoodBox { food: f });
        g.player.pos = pos + Vec2::new(0.5, 0.0);
        g.show_food("zebra").unwrap();
        assert_eq!(g.animal("zebra").unwrap().state, AnimalState::InEnclosure);
    }
    g.player.pos = cell_center(IVec2::new(-8, 14));
    idle(&mut g, 20.0);
    let z = g.animal("zebra").unwrap();
    assert_eq!((z.state, z.pos), (AnimalState::InEnclosure, pos));
}

// ANIM-003 (for the missions with texts: zebra)
#[test]
fn anim_003_board_food_word_equals_box_label() {
    let c = common::content();
    let mut g = common::game(1);
    for level in ReadingLevel::ALL {
        g.settings.reading_level = level;
        let board = g.info_board("zebra").unwrap();
        let labels = g.food_labels();
        let (_, label) = labels.iter().find(|(f, _)| *f == Food::Grass).unwrap();
        assert_eq!(board.food_key, label.word_key);
        for lang in Language::ALL {
            assert!(c.text(lang, &board.food_key).is_some());
        }
    }
}

// ANIM-004 (animals of level 1; the other 7 hiding places are not in any layout data yet)
#[test]
fn anim_004_hiding_places_exist_in_layout() {
    let data = common::level1();
    for enc in data.elements_of(ElementType::Enclosure) {
        let a = zoo_core::animals::animal_info(enc.animal.as_deref().unwrap()).unwrap();
        for h in a.hiding_places {
            let e = data.element(h).unwrap_or_else(|| panic!("{h} missing"));
            assert_eq!(e.ty, ElementType::HidingPlace);
            assert_eq!(e.animal.as_deref(), Some(a.id));
        }
    }
}

// ANIM-005
#[test]
fn anim_005_board_shows_riddle_for_chosen_place() {
    for seed in 0..10 {
        let mut g = Game::new(level_with_two_zebra_places(), seed).unwrap();
        let hp = g.animal("zebra").unwrap().hiding_place.clone();
        for level in ReadingLevel::ALL {
            g.settings.reading_level = level;
            let b = g.info_board("zebra").unwrap();
            assert_eq!(b.riddle_key, riddle_key("zebra", &hp, level));
            assert_eq!(b.picture.is_some(), level == ReadingLevel::Kiga);
            if let Some(p) = b.picture {
                assert_eq!(p, hp);
            }
        }
    }
}

// READ-003
#[test]
fn read_003_level_change_applies_to_next_label() {
    let mut g = common::game(1);
    g.settings.reading_level = ReadingLevel::Kiga;
    assert!(g.food_labels().iter().all(|(_, l)| l.picture));
    assert_eq!(
        g.info_board("zebra").unwrap().riddle_key,
        "mission-zebra-riddle-kiga"
    );
    g.settings.reading_level = ReadingLevel::Klasse2;
    assert!(g.food_labels().iter().all(|(_, l)| !l.picture));
    assert_eq!(
        g.info_board("zebra").unwrap().riddle_key,
        "mission-zebra-riddle-klasse2"
    );
}

// FEED-001, FEED-002
#[test]
fn feed_001_002_label_forms() {
    let mut g = common::game(1);
    for level in ReadingLevel::ALL {
        g.settings.reading_level = level;
        for (food, label) in g.food_labels() {
            assert_eq!(label.picture, level == ReadingLevel::Kiga, "{level:?}");
            assert_eq!(label.word_key, food.label_key());
        }
    }
    let c = common::content();
    for lang in Language::ALL {
        for f in Food::ALL {
            let w = c.text(lang, &f.label_key()).unwrap();
            // "one word" (en "fish food" is a two-word label — see report)
            assert!(!w.is_empty() && w.split_whitespace().count() <= 2, "{w}");
        }
    }
}

// FEED-003
#[test]
fn feed_003_taking_another_box_puts_food_back() {
    let mut g = common::game(1);
    get_food(&mut g, Food::Hay);
    g.drain_events();
    g.take_food(Food::Bamboo).unwrap();
    assert_eq!(g.carry.food(), Some(Food::Bamboo));
    assert!(g.drain_events().contains(&GameEvent::FoodTaken {
        food: Food::Bamboo,
        returned: Some(Food::Hay)
    }));
    assert!(g.storage.has_box(Food::Hay));
}

// FEED-005
#[test]
fn feed_005_every_correct_food_has_a_box() {
    let g = common::game(1);
    for a in ANIMALS {
        for f in a.foods {
            assert!(g.storage.has_box(*f), "{} {f:?}", a.id);
        }
    }
}

// FEED-006
#[test]
fn feed_006_boxes_never_run_out() {
    let mut g = common::game(1);
    get_food(&mut g, Food::Grass);
    for _ in 0..10 {
        g.take_food(Food::Grass).unwrap();
    }
    g.take_food(Food::Grass).unwrap();
    assert_eq!(g.carry.food(), Some(Food::Grass));
}

#[test]
fn take_food_needs_to_be_at_storage() {
    let mut g = common::game(1);
    assert_eq!(g.take_food(Food::Grass), Err(InteractError::OutOfRange));
}

#[test]
fn whole_mission_is_deterministic() {
    let run = || {
        let mut g = common::game(9);
        make_follow(&mut g, "zebra", Food::Grass);
        walk_to(&mut g, IVec2::new(-7, 20), 120.0);
        (g.player.pos, g.animal("zebra").unwrap().pos)
    };
    assert_eq!(run(), run());
}

// FEED-007
#[test]
fn feed_007_interact_with_box_shows_label_then_take() {
    let mut g = common::game(1);
    let front = box_front(&g, Food::Grass);
    walk_to(&mut g, front, 60.0);
    g.player.facing = Vec2::Y;
    let it = g.interact().expect("grass box available");
    assert_eq!(
        it,
        zoo_core::Interaction::FoodBox {
            food: Food::Grass,
            label: zoo_core::food::FoodLabel {
                word_key: "food-grass".into(),
                picture: false
            }
        }
    );
    assert_eq!(g.carry.food(), None, "interacting alone takes nothing");
    g.take_food(Food::Grass).unwrap();
    assert_eq!(g.carry.food(), Some(Food::Grass));
}

// FEED-008
#[test]
fn feed_008_one_box_per_food_next_to_storage() {
    let g = common::game(1);
    let storage = g.level.data.element("food_storage").unwrap().rect;
    for f in Food::ALL {
        assert_eq!(g.food_boxes.iter().filter(|b| b.0 == f).count(), 1, "{f:?}");
    }
    let grid = g.level.grid();
    for &(f, pos, facing) in &g.food_boxes {
        assert!(
            grid.is_walkable(cell_of(pos), false),
            "{f:?} not on walkable ground"
        );
        assert!(
            storage.distance_to(pos) <= 1.0,
            "{f:?} not next to the storage"
        );
        let stand = g.level.data.level.bounds.cells().find(|&c| {
            let to = cell_center(c) - pos;
            grid.is_passable(c, false)
                && to.length() <= 2.0
                && to.normalize().dot(facing) >= 60f32.to_radians().cos()
        });
        assert!(stand.is_some(), "{f:?}: no standing point in front");
    }
}

// Scripted player (PROD-POC M4 e2e helper) reaches the zebra board and the grass box.
#[test]
fn autopilot_reaches_targets_with_collision() {
    let mut g = common::game(1);
    for target in [
        Vec2::new(-7.5, 14.5),
        Vec2::new(-0.4, 9.5),
        Vec2::new(8.0, 30.0),
    ] {
        let mut ap = nav::Autopilot::new(target);
        let mut t = 0.0;
        while let Some(dir) = ap.input(g.level.grid(), g.player.pos, g.is_leading()) {
            g.update(DT, dir);
            t += DT;
            assert!(t < 60.0, "autopilot to {target} stuck at {}", g.player.pos);
        }
    }
}
