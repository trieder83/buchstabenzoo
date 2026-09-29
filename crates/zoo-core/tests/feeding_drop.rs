//! GAME-FEED §8–17: putting items down, picking them up again, the 8-item limit, giving
//! consumes, and cutting bamboo in the bamboo forest (FEED-009…023).

mod common;

use glam::{IVec2, Vec2};
use zoo_core::carrying::{
    DropError, Dropped, Held, StalkStage, BACK_TO_BOX_M, DROP_AHEAD_M, DROP_SEARCH_M, MAX_LYING,
    REGROW_S,
};
use zoo_core::collision::PLAYER_RADIUS_M;
use zoo_core::game::{GameEvent, Target};
use zoo_core::level::{cell_center, cell_of, ElementType};
use zoo_core::{AnimalState, Food, FoodBox, Game};

fn carry(g: &mut Game, food: Food) {
    g.carry.take(&FoodBox { food });
}

/// Open plaza ground in level 1 (entrance plaza, nothing around).
const PLAZA: Vec2 = Vec2::new(2.5, 4.5);

fn stand(g: &mut Game, p: Vec2, facing: Vec2) {
    g.player.pos = p;
    g.player.facing = facing.normalize();
}

/// Level 1 with every mission out of the way of the plaza.
fn plaza_game() -> Game {
    let mut g = common::game(3);
    stand(&mut g, PLAZA, Vec2::Y);
    g
}

/// The joined zoo with levels 1 and 2 complete (level 3 with the fish bowl unlocked).
fn unlocked_game(seed: u64) -> Game {
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

fn follow(g: &mut Game, animal: &str) {
    for j in g.group(animal) {
        g.animals[j].state = AnimalState::Following;
    }
}

// FEED-009: hay lies on free walkable ground ≈ 0.8 m in front of her, on the surface.
#[test]
fn feed_009_put_down_in_front() {
    let mut g = plaza_game();
    carry(&mut g, Food::Hay);
    assert!(g.can_put_down());
    assert!(g.spot_free(PLAZA + Vec2::Y * DROP_AHEAD_M));
    let r = g.put_down().unwrap();
    assert!(matches!(
        r,
        Dropped::Food {
            food: Food::Hay,
            ..
        }
    ));
    assert_eq!(g.carry.food(), None, "she carries nothing");
    assert!(!g.can_put_down());
    let f = g.lying.foods[0];
    assert_eq!(f.food, Food::Hay);
    assert!((f.pos.distance(PLAZA) - DROP_AHEAD_M).abs() < 1e-4);
    assert!(
        (f.y - g.level.ground_height(f.pos)).abs() < 1e-5,
        "stands on the surface"
    );
    assert!(f.y > 0.0, "on the plaza slabs, not at y = 0");
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::ItemPutDown { id } if id == "food:hay")));
    // nothing in the hands: nothing happens (FEED-016 logic)
    assert_eq!(g.put_down(), Err(DropError::NothingHeld));
}

/// A walkable cell centre one or two cells before the element `id` in direction `dir`.
fn beside(g: &Game, id: &str, dir: IVec2) -> Vec2 {
    let e = g.level.data.element(id).unwrap();
    // the free cell along that side farthest from any gate or door
    let cells: Vec<Vec2> = e
        .rect
        .cells()
        .flat_map(|c| [c - dir, c - dir * 2])
        .filter(|&c| g.level.grid().is_walkable(c, false) && !e.rect.contains(c))
        .map(cell_center)
        .filter(|&p| !g.level.colliders().overlaps(p, PLAYER_RADIUS_M))
        .filter(|&p| !g.spot_free(p + dir.as_vec2() * DROP_AHEAD_M))
        .collect();
    let gap = |p: &Vec2| {
        g.level
            .openings()
            .iter()
            .map(|o| o.center.distance(*p))
            .fold(f32::MAX, f32::min)
    };
    cells
        .into_iter()
        .max_by(|a, b| gap(a).total_cmp(&gap(b)))
        .unwrap_or_else(|| panic!("no walkable cell beside {id}"))
}

// FEED-010: water, fence, enclosure inside, gate walkway: the nearest free spot within 1.5 m;
// none: not dropped.
#[test]
fn feed_010_blocked_spots_move_or_refuse() {
    let mut g = plaza_game();
    // water: the player on the bank of the river, facing the water
    for (id, dir) in [
        ("river_e", IVec2::Y),
        ("pond_water", IVec2::X),
        ("enc_hippo", IVec2::X),
        ("enc_zebra", IVec2::NEG_X),
    ] {
        let p = beside(&g, id, dir);
        stand(&mut g, p, dir.as_vec2());
        let nominal = p + dir.as_vec2() * DROP_AHEAD_M;
        assert!(!g.spot_free(nominal), "{id}: the spot in front is not free");
        carry(&mut g, Food::Hay);
        let n = g.lying.foods.len();
        g.put_down().unwrap_or_else(|e| panic!("{id}: {e:?}"));
        let f = *g.lying.foods.last().unwrap();
        assert_eq!(g.lying.foods.len(), n + 1);
        assert!(f.pos.distance(nominal) <= DROP_SEARCH_M + 1e-4, "{id}");
        assert!(g.level.grid().is_walkable(cell_of(f.pos), false), "{id}");
        let e = g.level.data.element(id).unwrap();
        assert!(
            !e.rect.contains(cell_of(f.pos)),
            "{id}: not in the water / enclosure"
        );
    }
    // a gate walkway: facing the zebra gate from the path
    let o = g
        .level
        .openings()
        .iter()
        .find(|o| matches!(o.kind, zoo_core::scene::OpeningKind::EnclosureGate { .. }))
        .unwrap()
        .clone();
    let from = (1..30)
        .map(|k| o.center + Vec2::X * (k as f32 * 0.1))
        .chain((1..30).map(|k| o.center - Vec2::X * (k as f32 * 0.1)))
        .find(|&p| {
            g.level.grid().is_walkable(cell_of(p), false)
                && !g.level.colliders().overlaps(p, PLAYER_RADIUS_M)
                && p.distance(o.center) > 1.2
        })
        .expect("a stand in front of the gate");
    stand(&mut g, from, o.center - from);
    carry(&mut g, Food::Grass);
    g.put_down().unwrap();
    let f = *g.lying.foods.last().unwrap();
    assert!(
        f.pos.distance(o.center) >= o.opening_m / 2.0 + 1.0 - 1e-3,
        "not in the gate walkway"
    );
    // in the middle of the pond: nothing free within 1.5 m → not dropped, still carried
    let pond = g.level.data.element("pond_water").unwrap().rect;
    stand(
        &mut g,
        Vec2::new(pond.x as f32 + 4.0, pond.z as f32 + 4.0),
        Vec2::Y,
    );
    carry(&mut g, Food::Melons);
    assert_eq!(g.put_down(), Err(DropError::NoFreeSpot));
    assert_eq!(g.carry.food(), Some(Food::Melons));
    // (golf carts, GAME-CART, are not in the game yet: nothing to sit in)
}

// FEED-011: pick up within 2 m; full hands swap.
#[test]
fn feed_011_pick_up_and_swap() {
    let mut g = plaza_game();
    carry(&mut g, Food::Hay);
    g.put_down().unwrap();
    let hay = g.lying.foods[0];
    // step back 1.5 m, facing it
    stand(&mut g, PLAZA - Vec2::Y * 0.7, Vec2::Y);
    assert_eq!(
        g.available_target(),
        Some(Target::LyingFood {
            uid: hay.uid,
            food: Food::Hay
        })
    );
    g.interact().expect("picked up");
    assert_eq!(g.carry.food(), Some(Food::Hay));
    assert!(g.lying.foods.is_empty());
    // swap: carrying bamboo, the hay lying → bamboo lies on the hay's spot
    g.put_down().unwrap();
    let hay = g.lying.foods[0];
    carry(&mut g, Food::Bamboo);
    assert!(matches!(
        g.available_target(),
        Some(Target::LyingFood { .. })
    ));
    g.interact().expect("swapped");
    assert_eq!(g.carry.food(), Some(Food::Hay));
    assert_eq!(g.lying.foods.len(), 1);
    assert_eq!(g.lying.foods[0].food, Food::Bamboo);
    assert_eq!(g.lying.foods[0].pos, hay.pos);
    // out of range (> 2 m): nothing
    stand(&mut g, PLAZA - Vec2::Y * 2.0, Vec2::Y);
    let uid = g.lying.foods[0].uid;
    assert!(!g.pick_up_food(uid));
}

// FEED-012: bowl with water + fish and a food in the pocket: the bowl lies, the food moves
// to the hands; picking the bowl up again restores bowl + fish, food back to the pocket.
#[test]
fn feed_012_bowl_and_pocket() {
    let mut g = unlocked_game(8);
    let open = Vec2::new(-12.5, 70.5);
    {
        let b = g.bowl.as_mut().unwrap();
        b.carried = true;
        b.water = true;
        b.fish = true;
    }
    for j in g.group("goldfish") {
        g.animals[j].state = AnimalState::InBowl;
    }
    carry(&mut g, Food::FishFood);
    stand(&mut g, open, Vec2::Y);
    assert_eq!(g.held(), Some(Held::Bowl));
    let r = g.put_down().unwrap();
    let Dropped::Bowl { pos } = r else {
        panic!("{r:?}")
    };
    let b = g.bowl.clone().unwrap();
    assert!(!b.carried && b.water && b.fish && b.dropped);
    assert_eq!(b.pos, pos);
    assert!((b.lift_m - g.level.ground_height(pos)).abs() < 1e-5);
    assert_eq!(
        g.held(),
        Some(Held::Food(Food::FishFood)),
        "food to the hands"
    );
    assert_eq!(g.lying_count(), 1);
    // the fish stays in the bowl on the ground
    for j in g.group("goldfish") {
        assert_eq!(g.animals[j].state, AnimalState::InBowl);
        assert_eq!(g.animals[j].pos, pos);
    }
    // pick the bowl up again: bowl + fish, the food back into the pocket
    assert_eq!(
        g.available_target(),
        Some(Target::Item {
            id: "fish_bowl".into()
        })
    );
    g.interact().unwrap();
    assert!(g.carrying_animal());
    assert_eq!(g.held(), Some(Held::Bowl));
    assert_eq!(g.carry.food(), Some(Food::FishFood), "pocket");
    assert_eq!(g.lying_count(), 0);
}

// FEED-013: following animals keep following when their food is put down; animals never eat
// or take lying food.
#[test]
fn feed_013_animals_keep_following() {
    let mut g = plaza_game();
    follow(&mut g, "zebra");
    carry(&mut g, Food::Grass);
    g.put_down().unwrap();
    for _ in 0..600 {
        g.update(1.0 / 60.0, Vec2::ZERO);
    }
    for j in g.group("zebra") {
        assert_eq!(g.animals[j].state, AnimalState::Following);
    }
    assert_eq!(g.lying.foods.len(), 1, "the grass still lies there");
    // led home without the food in the hands: the lying grass stays
    assert!(g.debug_send_home("zebra"));
    assert_eq!(g.lying.foods.len(), 1);
    assert_eq!(g.lying.foods[0].food, Food::Grass);
}

// FEED-014: next to its own box → back into the box; the 9th item returns the oldest food,
// never the bowl; the basket (and the honey pot) are never put down (Q-172).
#[test]
fn feed_014_back_into_box_and_limit() {
    let mut g = plaza_game();
    let (_, hay_box, _) = *g
        .food_boxes
        .iter()
        .find(|b| b.0 == Food::Hay)
        .expect("hay box");
    // 1.3 m in front of the box, facing it: the drop spot is within 1.5 m of the box
    stand(&mut g, hay_box - Vec2::Y * 1.3, Vec2::Y);
    carry(&mut g, Food::Hay);
    assert!(hay_box.distance(g.player.pos + Vec2::Y * DROP_AHEAD_M) <= BACK_TO_BOX_M);
    assert_eq!(g.put_down(), Ok(Dropped::IntoBox { food: Food::Hay }));
    assert!(g.lying.foods.is_empty() && g.carry.food().is_none());
    assert!(g
        .drain_events()
        .contains(&GameEvent::FoodPutBack { food: Food::Hay }));
    // another food next to the hay box does not go in (leaves: its own box is at the far
    // end of the row, not within reach)
    carry(&mut g, Food::Leaves);
    assert!(matches!(g.put_down(), Ok(Dropped::Food { .. })));
    assert_eq!(g.lying.foods.len(), 1);

    // the limit (with the bowl among the lying items)
    let mut g = unlocked_game(8);
    g.bowl.as_mut().unwrap().carried = true;
    stand(&mut g, Vec2::new(-12.5, 70.5), Vec2::Y);
    g.put_down().unwrap();
    assert!(g.bowl_lying());
    let foods = [
        Food::Grass,
        Food::Hay,
        Food::Melons,
        Food::Meat,
        Food::Leaves,
        Food::Berries,
        Food::Bananas,
    ];
    let mut x = -14.5;
    for f in foods {
        stand(&mut g, Vec2::new(x, 68.5), Vec2::NEG_Y);
        carry(&mut g, f);
        g.put_down().unwrap_or_else(|e| panic!("{f:?} {e:?}"));
        x += 0.6;
    }
    assert_eq!(g.lying_count(), MAX_LYING);
    g.drain_events();
    stand(&mut g, Vec2::new(x + 0.6, 68.5), Vec2::NEG_Y);
    carry(&mut g, Food::Eucalyptus);
    g.put_down().unwrap();
    assert_eq!(g.lying_count(), MAX_LYING, "never more than 8");
    assert!(g.bowl_lying(), "the bowl is never removed");
    assert!(
        !g.lying.foods.iter().any(|f| f.food == Food::Grass),
        "the oldest food went back"
    );
    assert!(g
        .drain_events()
        .contains(&GameEvent::FoodPutBack { food: Food::Grass }));

    // the basket is its own slot and is never put down (Q-172 answered 2026-09-28): with
    // only treats in the basket nothing can be put down; with a food in the hands only the
    // food goes, the treats stay in the basket
    let mut g = plaza_game();
    g.garden.basket.carrots = 1;
    g.garden.basket.potatoes = 2;
    stand(&mut g, PLAZA, Vec2::Y);
    assert!(!g.can_put_down(), "the basket is no hand item");
    assert_eq!(g.put_down(), Err(DropError::NothingHeld));
    carry(&mut g, Food::Hay);
    assert!(matches!(
        g.put_down(),
        Ok(Dropped::Food {
            food: Food::Hay,
            ..
        })
    ));
    assert_eq!(g.garden.basket.total(), 3, "the treats stay in the basket");
    assert!(!g.can_put_down());
}

// FEED-015: saved and restored with the same place and content.
#[test]
fn feed_015_lying_items_saved() {
    let mut g = unlocked_game(5);
    g.bowl.as_mut().unwrap().carried = true;
    g.bowl.as_mut().unwrap().water = true;
    stand(&mut g, Vec2::new(-12.5, 70.5), Vec2::Y);
    g.put_down().unwrap();
    for (k, f) in [Food::Hay, Food::Bamboo].into_iter().enumerate() {
        stand(&mut g, Vec2::new(-14.0 + k as f32, 68.5), Vec2::NEG_Y);
        carry(&mut g, f);
        g.put_down().unwrap();
    }
    let json = g.to_save().to_json();
    let r = Game::from_save_json(common::zoo(), &json).unwrap();
    assert_eq!(r.lying.foods.len(), 2);
    for (a, b) in g.lying.foods.iter().zip(&r.lying.foods) {
        assert_eq!((a.food, a.pos, a.y), (b.food, b.pos, b.y));
    }
    assert_eq!(r.bowl, g.bowl);
    assert!(r.bowl_lying());
    // the restored items can be picked up
    let mut r = r;
    stand(&mut r, Vec2::new(-14.0, 69.3), Vec2::NEG_Y);
    let uid = r.lying.foods[0].uid;
    assert!(r.pick_up_food(uid));
}

/// Stands at a cut spot's stand point facing its stalk.
fn at_spot(g: &mut Game, id: &str) {
    let c = g
        .level
        .data
        .cut_spots
        .iter()
        .find(|c| c.id == id)
        .unwrap()
        .clone();
    stand(g, c.stand(), c.pos() - c.stand());
}

fn spot_index(g: &Game, id: &str) -> usize {
    g.level
        .data
        .cut_spots
        .iter()
        .position(|c| c.id == id)
        .unwrap()
}

// FEED-017: a full-grown cut spot → she carries bamboo, the spot becomes a stump.
#[test]
fn feed_017_cut_bamboo() {
    let mut g = common::game(2);
    let id = g.level.data.cut_spots[0].id.clone();
    at_spot(&mut g, &id);
    assert_eq!(
        g.available_target(),
        Some(Target::CutSpot { spot: id.clone() })
    );
    g.interact().expect("cut");
    assert_eq!(g.carry.food(), Some(Food::Bamboo));
    assert_eq!(g.bamboo.stage(spot_index(&g, &id)), Some(StalkStage::Stump));
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, GameEvent::BambooCut { spot } if *spot == id)));
    // a stump is no target
    assert_ne!(g.available_target(), Some(Target::CutSpot { spot: id }));
}

// FEED-018: bamboo from the forest makes the panda follow and is eaten at home.
#[test]
fn feed_018_forest_bamboo_works_for_the_panda() {
    let mut g = common::game(4);
    let id = g.level.data.cut_spots[1].id.clone();
    at_spot(&mut g, &id);
    assert!(g.cut_bamboo(&id));
    let panda = g.animal("panda").unwrap().pos;
    stand(&mut g, panda + Vec2::new(0.0, -1.0), Vec2::Y);
    g.show_food("panda").unwrap();
    assert_eq!(g.animal("panda").unwrap().state, AnimalState::Following);
    assert!(g.debug_send_home("panda"));
    assert_eq!(g.carry.food(), None, "eaten at home");
    assert!(g
        .drain_events()
        .contains(&GameEvent::FoodConsumed { food: Food::Bamboo }));
}

// FEED-019: 3 min of play time: stump → young → full; paused time does not count.
#[test]
fn feed_019_regrowth() {
    let mut g = common::game(2);
    let id = g.level.data.cut_spots[2].id.clone();
    let i = spot_index(&g, &id);
    at_spot(&mut g, &id);
    assert!(g.cut_bamboo(&id));
    g.carry.consume();
    // paused: the host passes dt = 0
    for _ in 0..1000 {
        g.update(0.0, Vec2::ZERO);
    }
    assert_eq!(g.bamboo.regrow_s[i], REGROW_S);
    let mut seen = vec![g.bamboo.stage(i).unwrap()];
    let mut t = 0.0;
    while t < REGROW_S - 1.0 {
        g.update(1.0, Vec2::ZERO);
        t += 1.0;
        assert!(!g.cut_bamboo(&id), "not cuttable before 3 min ({t} s)");
        let s = g.bamboo.stage(i).unwrap();
        if *seen.last().unwrap() != s {
            seen.push(s);
        }
    }
    g.update(1.0, Vec2::ZERO);
    let s = g.bamboo.stage(i).unwrap();
    if *seen.last().unwrap() != s {
        seen.push(s);
    }
    assert_eq!(
        seen,
        vec![StalkStage::Stump, StalkStage::Young, StalkStage::Full]
    );
    at_spot(&mut g, &id);
    assert!(g.cut_bamboo(&id), "cuttable again");
}

// FEED-020: carrying hay and cutting bamboo: hay lies at her feet, she carries bamboo.
#[test]
fn feed_020_cut_puts_held_food_down() {
    let mut g = common::game(2);
    let id = g.level.data.cut_spots[3].id.clone();
    at_spot(&mut g, &id);
    carry(&mut g, Food::Hay);
    g.interact().expect("cut");
    assert_eq!(g.carry.food(), Some(Food::Bamboo));
    let hay = g.lying.foods.iter().find(|f| f.food == Food::Hay).unwrap();
    assert!(hay.pos.distance(g.player.pos) <= 0.6, "at her feet");
}

// FEED-021: growth stages and remaining time survive save / restore.
#[test]
fn feed_021_regrowth_saved() {
    let mut g = common::game(2);
    let ids: Vec<String> = g
        .level
        .data
        .cut_spots
        .iter()
        .map(|c| c.id.clone())
        .collect();
    at_spot(&mut g, &ids[0]);
    assert!(g.cut_bamboo(&ids[0]));
    for _ in 0..100 {
        g.update(1.0, Vec2::ZERO);
    }
    at_spot(&mut g, &ids[1]);
    assert!(g.cut_bamboo(&ids[1]));
    g.update(10.0, Vec2::ZERO);
    let r = Game::from_save_json(common::level1(), &g.to_save().to_json()).unwrap();
    assert_eq!(r.bamboo, g.bamboo);
    assert_eq!(r.bamboo.stage(0), Some(StalkStage::Young));
    assert_eq!(r.bamboo.stage(1), Some(StalkStage::Stump));
    assert_eq!(r.bamboo.stage(2), Some(StalkStage::Full));
}

// FEED-022: data: harvestable forests, cut spots with a walkable stand ≤ 1.5 m, the forest
// stays solid and the hiding place valid.
#[test]
fn feed_022_forest_data() {
    for data in [common::level1(), common::zoo()] {
        let g = Game::new(data, 1).unwrap();
        let d = &g.level.data;
        assert_eq!(d.cut_spots.len(), 4, "level 1: 4 cut spots (Q-156)");
        for c in &d.cut_spots {
            let e = d
                .element(&c.forest)
                .unwrap_or_else(|| panic!("{}", c.forest));
            assert_eq!(e.kind.as_deref(), Some("bamboo"), "{}", c.id);
            assert!(e.harvestable, "{}", c.id);
            let (pos, st) = (c.pos(), c.stand());
            assert!(st.distance(pos) <= 1.5, "{}: stand within 1.5 m", c.id);
            assert!(
                g.level.grid().is_walkable(cell_of(st), false)
                    && !g.level.colliders().overlaps(st, PLAYER_RADIUS_M),
                "{}: walkable stand",
                c.id
            );
            // the stalk stands at the forest's edge
            let r = e.rect;
            let min = Vec2::new(r.x as f32, r.z as f32);
            let q = pos.clamp(min, min + Vec2::new(r.w as f32, r.d as f32));
            assert!(q.distance(pos) < 1e-4, "{}: stalk inside the forest", c.id);
            // standing there facing it, the cut spot is the target
            let mut g2 = Game::new(d.clone(), 1).unwrap();
            stand(&mut g2, st, pos - st);
            assert_eq!(
                g2.available_target(),
                Some(Target::CutSpot { spot: c.id.clone() }),
                "{}",
                c.id
            );
        }
        // every harvestable forest stays solid and keeps its hiding place
        for e in d.elements.iter().filter(|e| e.harvestable) {
            assert_eq!(e.ty, ElementType::Decoration);
            for c in e.rect.cells() {
                assert!(!g.level.grid().is_walkable(c, true), "{} solid", e.id);
            }
        }
        let h = d.hiding_place("loc_bamboo").unwrap();
        assert!(h.scenery.iter().any(|s| s == "bamboo_sw"));
        assert!(g.level.grid().is_walkable(cell_of(h.spot()), false));
    }
}

// FEED-023: giving consumes the food in the hands; lying food stays; showing never consumes.
#[test]
fn feed_023_giving_consumes() {
    let mut g = plaza_game();
    follow(&mut g, "zebra");
    carry(&mut g, Food::Grass);
    assert!(g.debug_send_home("zebra"));
    assert_eq!(g.carry.food(), None, "the grass is eaten");
    // the grass put down earlier stays
    let mut g = plaza_game();
    follow(&mut g, "zebra");
    carry(&mut g, Food::Grass);
    g.put_down().unwrap();
    assert!(g.debug_send_home("zebra"));
    assert_eq!(g.lying.foods.len(), 1);
    // showing alone never consumes (FEED-004)
    let mut g = common::game(6);
    carry(&mut g, Food::Grass);
    let z = g.animal("zebra").unwrap().pos;
    stand(&mut g, z + Vec2::new(0.0, -1.0), Vec2::Y);
    g.show_food("zebra").unwrap();
    assert_eq!(g.carry.food(), Some(Food::Grass));
}
