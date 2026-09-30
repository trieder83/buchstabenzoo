//! GAME-SAVE: saving and restoring the game state (SAVE-001/005/006/007/009, unit level).

mod common;

use glam::Vec2;
use zoo_core::level::{cell_center, cell_of};
use zoo_core::save::{SaveState, SAVE_VERSION};
use zoo_core::{AnimalState, Food, Game};

const DT: f32 = 1.0 / 60.0;

/// Deterministic scripted input: walks in a slowly turning direction (with stops).
fn input(k: usize) -> Vec2 {
    if k % 240 > 200 {
        return Vec2::ZERO;
    }
    let a = k as f32 * 0.004;
    Vec2::new(a.cos(), a.sin())
}

fn run(g: &mut Game, from: usize, to: usize) {
    for k in from..to {
        g.update(DT, input(k));
        g.drain_events();
    }
}

/// A game in progress: grass taken, zebra following, player a few steps further.
fn game_in_progress() -> Game {
    let mut g = common::game(7);
    let (_, pos, facing) = *g.food_boxes.iter().find(|b| b.0 == Food::Grass).unwrap();
    g.player.pos = cell_center(cell_of(pos + facing * 1.1));
    g.take_food(Food::Grass).unwrap();
    let zebra = g.animal("zebra").unwrap().pos;
    let near = [Vec2::X, Vec2::NEG_X, Vec2::Y, Vec2::NEG_Y]
        .into_iter()
        .map(|d| zebra + d * 1.5)
        .find(|p| g.level.grid().is_walkable(cell_of(*p), false))
        .unwrap();
    g.player.pos = near;
    g.player.facing = (zebra - near).normalize();
    g.show_food("zebra").unwrap();
    assert_eq!(g.animal("zebra").unwrap().state, AnimalState::Following);
    g.missions[0].started = true;
    for _ in 0..90 {
        g.update(DT, Vec2::new(0.3, -1.0).normalize());
    }
    g.drain_events();
    g
}

fn restore(g: &Game) -> Game {
    let json = g.to_save().to_json();
    Game::from_save_json(common::level1(), &json).expect("restore")
}

// SAVE-001
#[test]
fn save_001_round_trip_restores_the_same_state() {
    let g = game_in_progress();
    let s = g.to_save();
    assert_eq!(s.version, SAVE_VERSION);
    assert_eq!(s.player.carry.as_deref(), Some("grass"));
    let r = restore(&g);
    assert_eq!(r.to_save(), s);
    assert_eq!(r.player.pos, g.player.pos);
    assert_eq!(r.player.facing, g.player.facing);
    assert_eq!(r.carry.food(), Some(Food::Grass));
    assert_eq!(r.animal("zebra").unwrap().state, AnimalState::Following);
    assert_eq!(
        r.animal("zebra").unwrap().pos,
        g.animal("zebra").unwrap().pos
    );
    assert!(r.mission("zebra").unwrap().started);
    // JSON round trip is lossless and small
    let json = s.to_json();
    assert_eq!(SaveState::from_json(&json).unwrap(), s);
    assert!(json.len() < 64 * 1024, "{} bytes", json.len());
    // camera and yaw (presentation) survive the JSON as well
    let mut s2 = s.clone();
    s2.camera = Some(zoo_core::save::CameraSave {
        yaw_steps: -3,
        distance_m: 12.5,
    });
    s2.animals[0].yaw = Some(1.25);
    assert_eq!(SaveState::from_json(&s2.to_json()).unwrap(), s2);
}

// SAVE-001 (completed mission, opened barriers, no events on restore)
#[test]
fn save_001_completed_mission_restores_without_celebration() {
    let mut g = common::game(3);
    let enc = g.animal("zebra").unwrap().enclosure;
    let rect = g.level.data.elements[enc].rect;
    let i = g.animal_index("zebra").unwrap();
    g.animals[i].state = AnimalState::InEnclosure;
    g.animals[i].pos = cell_center(glam::IVec2::new(rect.x + 2, rect.z + 2));
    g.missions[i].started = true;
    g.missions[i].complete = true;
    let mut r = restore(&g);
    assert!(r.mission("zebra").unwrap().complete);
    assert_eq!(r.animal("zebra").unwrap().state, AnimalState::InEnclosure);
    assert!(rect.contains(cell_of(r.animal("zebra").unwrap().pos)));
    let ev = r.drain_events();
    assert!(
        ev.is_empty(),
        "no events (celebration) after restore: {ev:?}"
    );
    assert!(r.to_save().missions[i].celebrated);
}

// SAVE-005
#[test]
fn save_005_invalid_saves_start_a_new_game() {
    let data = common::level1;
    for bad in [
        "",
        "{",
        "not json",
        "{\"version\": 99}",
        "{\"version\": 1}",
        "[1,2,3]",
    ] {
        assert!(
            Game::from_save_json(data(), bad).is_err(),
            "accepted {bad:?}"
        );
    }
    let g = common::game(1);
    let mut s = g.to_save();
    s.level_id = "level-9".into();
    assert!(Game::from_save(data(), &s).is_err());
    let mut s = g.to_save();
    s.version = SAVE_VERSION + 1;
    assert!(Game::from_save_json(data(), &s.to_json()).is_err());
    let mut s = g.to_save();
    s.animals[0].state = "flying".into();
    assert!(Game::from_save(data(), &s).is_err());
    let mut json = g.to_save().to_json();
    json = json.replacen("\"pos\":[", "\"pos\":[1e999,", 1);
    assert!(Game::from_save_json(data(), &json).is_err());
}

// SAVE-006
#[test]
fn save_006_unwalkable_position_moves_to_nearest_walkable_cell() {
    let g = common::game(1);
    let mut s = g.to_save();
    // in the south wall of the food storage beside its door (solid wall band; the door and the
    // interior are walkable since the storage is enterable, LAYOUT-041)
    s.player.pos = [-1.5, 11.4];
    let r = Game::from_save(common::level1(), &s).unwrap();
    let c = cell_of(r.player.pos);
    assert!(r.level.grid().is_walkable(c, false), "{}", r.player.pos);
    assert!(
        r.player.pos.distance(Vec2::new(-1.5, 11.4)) <= 1.5,
        "nearest: {}",
        r.player.pos
    );
    // a following zebra that stood on a now solid cell appears behind the player
    let mut g = game_in_progress();
    let i = g.animal_index("zebra").unwrap();
    g.animals[i].pos = Vec2::new(-3.5, 13.5); // west wall band of the storage
    let r = restore(&g);
    let z = r.animal("zebra").unwrap().pos;
    assert!(r.level.grid().is_walkable(cell_of(z), true));
    assert!(z.distance(r.player.pos) < 3.0, "behind the player: {z}");
}

// SAVE-007
#[test]
fn save_007_restore_then_same_inputs_gives_the_same_state() {
    let mut a = game_in_progress();
    run(&mut a, 0, 300);
    let mut b = restore(&a);
    run(&mut a, 300, 900);
    run(&mut b, 300, 900);
    assert_eq!(a.to_save(), b.to_save());
    assert!(a.player.pos.distance(Vec2::ZERO) > 0.0);
}

// SAVE-009
#[test]
fn save_009_autosave_while_walking_and_on_progress() {
    let mut g = common::game(1);
    assert!(!g.save_due());
    // standing still: no autosave
    for _ in 0..(6.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
    g.drain_events();
    assert!(!g.save_due());
    // walking 6 s without progress events: due after 5 s of moving
    let mut due_at = None;
    for k in 0..(6.0 / DT) as usize {
        g.update(DT, input(k));
        g.drain_events();
        if due_at.is_none() && g.save_due() {
            due_at = Some(k as f32 * DT);
        }
    }
    let t = due_at.expect("autosave due while walking");
    // 5 s of moving (the scripted walk pauses briefly) — within the 6 s walk
    assert!((4.9..=6.0).contains(&t), "due after {t} s");
    g.mark_saved();
    assert!(!g.save_due());
    // a progress event (food taken) makes it due at once
    let (_, pos, facing) = *g.food_boxes.iter().find(|b| b.0 == Food::Hay).unwrap();
    g.player.pos = cell_center(cell_of(pos + facing * 1.1));
    g.take_food(Food::Hay).unwrap();
    g.drain_events();
    assert!(g.save_due());
    // saving takes ≤ 2 ms (serialisation of the state)
    let g = game_in_progress();
    let n = 50;
    let t0 = std::time::Instant::now();
    let mut bytes = 0;
    for _ in 0..n {
        bytes += g.to_save().to_json().len();
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    assert!(ms <= 2.0, "save took {ms:.3} ms");
    assert!(bytes > 0);
}

// RESC-029: the entrance intro is shown once — a new game has not seen it, the flag is saved,
// and a save from before the flag existed counts as seen (a loaded game never repeats it).
#[test]
fn resc_029_intro_seen_is_saved_and_old_saves_count_as_seen() {
    let mut g = common::game(3);
    assert!(!g.intro_seen, "a new game shows the intro");
    g.intro_seen = true;
    assert!(g.to_save().intro_seen);
    assert!(restore(&g).intro_seen);
    let fresh = common::game(3);
    assert!(!restore(&fresh).intro_seen, "unseen stays unseen");
    let mut v: serde_json::Value = serde_json::from_str(&g.to_save().to_json()).unwrap();
    v.as_object_mut().unwrap().remove("intro_seen");
    let old = SaveState::from_json(&v.to_string()).unwrap();
    assert!(old.intro_seen, "an old save has no flag: seen");
}
