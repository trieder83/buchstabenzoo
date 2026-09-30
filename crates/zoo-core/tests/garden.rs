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
