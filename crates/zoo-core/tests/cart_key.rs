//! GAME-CART key part (P2): the note on the desk, the key box with the 3-digit lock, the
//! tries counter, the save v3 fields (CART-012/013/022/024/025/026/027, SAVE-012/013).

mod common;

use glam::Vec2;
use zoo_core::cart_key::CodeResult;
use zoo_core::game::{Interaction, Target};
use zoo_core::math::{pad3, MathLevel};
use zoo_core::save::{SaveState, SAVE_VERSION};
use zoo_core::Game;

const NOTE: &str = "note_math_fighter";
const BOX: &str = "key_box_l1";

fn item(g: &Game, id: &str) -> (Vec2, Vec2) {
    let it = g
        .level
        .data
        .items
        .iter()
        .find(|i| i.id == id)
        .unwrap_or_else(|| panic!("item {id}"));
    let stand = it.stand.expect("stand cell");
    (
        it.pos(),
        Vec2::new(stand[0] as f32 + 0.5, stand[1] as f32 + 0.5),
    )
}

/// Stands the player on the item's stand cell, facing it.
fn stand_at(g: &mut Game, id: &str) {
    let (pos, stand) = item(g, id);
    g.player.pos = stand;
    g.player.facing = (pos - stand).normalize();
}

fn game(seed: u64) -> Game {
    common::zoo_game(seed)
}

// CART-012: the note on the desk
#[test]
fn cart_012_note_is_readable_at_the_desk() {
    let mut g = game(5);
    assert!(!g.note_read);
    stand_at(&mut g, NOTE);
    assert_eq!(g.available_target(), Some(Target::Note { id: NOTE.into() }));
    assert_eq!(g.available_target().unwrap().kind(), "note");
    assert!(Target::Note { id: NOTE.into() }.is_reading());
    let r = g.interact();
    assert_eq!(r, Some(Interaction::Note { id: NOTE.into() }));
    assert!(g.note_read);
    // the task is the one of the seed and math level; its code has 3 digits
    assert_eq!(g.cart_code(), pad3(g.cart_task().answer));
    assert_eq!(g.cart_code().len(), 3);
}

// CART-024: the panel opens by itself and reading resets the tries
#[test]
fn cart_024_reading_resets_tries_and_the_panel_opens_by_itself() {
    let mut g = game(5);
    stand_at(&mut g, BOX);
    for _ in 0..3 {
        g.enter_code(g.cart_task().answer % 999 + 1);
    }
    assert_eq!(g.key_box_tries, 3);
    assert!(g.lock_help());
    stand_at(&mut g, NOTE);
    for _ in 0..30 {
        g.update(1.0 / 60.0, Vec2::ZERO);
    }
    assert_eq!(g.panel.open, Some(Target::Note { id: NOTE.into() }));
    assert!(g.note_read);
    assert_eq!(g.key_box_tries, 0);
    assert!(!g.lock_help());
}

// CART-013: right and wrong codes
#[test]
fn cart_013_right_code_opens_wrong_code_counts() {
    let mut g = game(9);
    let answer = g.cart_task().answer;
    // too far away
    g.player.pos = Vec2::new(0.5, 2.5);
    assert_eq!(g.enter_code(answer), CodeResult::OutOfReach);
    stand_at(&mut g, BOX);
    assert_eq!(
        g.available_target(),
        Some(Target::KeyBox { id: BOX.into() })
    );
    assert!(!Target::KeyBox { id: BOX.into() }.is_reading());
    assert_eq!(g.interact(), Some(Interaction::KeyBox { id: BOX.into() }));
    let wrong = answer % 999 + 1;
    assert_eq!(g.enter_code(wrong), CodeResult::Wrong { tries: 1 });
    assert_eq!(g.enter_code(wrong), CodeResult::Wrong { tries: 2 });
    assert_eq!(g.key_box_tries, 2);
    assert!(!g.has_cart_key && !g.key_box_open);
    assert_eq!(g.enter_code(answer), CodeResult::Right);
    assert!(g.has_cart_key && g.key_box_open);
    assert_eq!(g.key_box_tries, 0);
    let events = g.drain_events();
    assert!(events.contains(&zoo_core::GameEvent::KeyBoxOpened));
    assert!(events
        .iter()
        .any(|e| matches!(e, zoo_core::GameEvent::WrongCode { tries: 2 })));
    // the open box is no longer a target; a second code finds no box
    assert_eq!(g.available_target(), None);
    assert_eq!(g.enter_code(answer), CodeResult::NoBox);
}

// MATH-011 (game part): the 3rd wrong code switches the visual aid of the note on; reading the
// note resets the tries but the aid stays until the task changes
#[test]
fn math_011_aid_after_three_wrong_codes() {
    let mut g = game(2);
    stand_at(&mut g, BOX);
    let wrong = g.cart_task().answer % 999 + 1;
    for _ in 0..2 {
        g.enter_code(wrong);
        assert!(!g.note_aid());
    }
    g.enter_code(wrong);
    assert!(g.note_aid() && g.lock_help());
    g.interact(); // (the key box panel)
    stand_at(&mut g, NOTE);
    g.interact();
    assert_eq!(g.key_box_tries, 0);
    assert!(g.note_aid(), "the aid stays after reading the note");
    let r = Game::from_save(common::zoo(), &g.to_save()).unwrap();
    assert!(r.note_aid());
    g.set_math_level(MathLevel::Mathe2);
    assert!(!g.note_aid(), "a new task starts without the aid");
}

// CART-023 (game part): exactly one of the 1000 codes opens the box
#[test]
fn cart_023_exactly_one_code_opens() {
    for level in MathLevel::ALL {
        let mut g = game(3);
        g.set_math_level(level);
        stand_at(&mut g, BOX);
        let code = g.cart_code();
        let mut opened = Vec::new();
        for c in 0..1000u32 {
            if g.key_box_open {
                break;
            }
            if g.enter_code(c) == CodeResult::Right {
                opened.push(pad3(c));
            }
        }
        assert_eq!(opened, vec![code], "{level:?}");
    }
}

// CART-025: no lockout
#[test]
fn cart_025_no_lockout_after_1000_wrong_codes() {
    let mut g = game(4);
    stand_at(&mut g, BOX);
    let answer = g.cart_task().answer;
    let wrong = answer % 999 + 1;
    for i in 0..1000u32 {
        g.update(0.01, Vec2::ZERO);
        match g.enter_code(wrong) {
            CodeResult::Wrong { tries } => assert_eq!(u32::from(tries), (i + 1).min(255)),
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(g.key_box_tries, 255, "saturating counter");
    assert_eq!(g.enter_code(answer), CodeResult::Right);
}

// CART-026: changing the level keeps note_read and an open box
#[test]
fn cart_026_math_level_change_after_the_box_is_open() {
    let mut g = game(4);
    stand_at(&mut g, BOX);
    let a = g.cart_task().answer;
    assert_eq!(g.enter_code(a), CodeResult::Right);
    g.note_read = true;
    g.set_math_level(MathLevel::Mathe3);
    assert!(g.key_box_open && g.has_cart_key && g.note_read);
}

// CART-022 / SAVE-012: save v3 round trip
#[test]
fn save_012_v3_fields_round_trip() {
    let mut g = game(6);
    g.has_cart_key = true;
    g.key_box_open = true;
    g.note_read = true;
    g.key_box_tries = 2;
    g.set_math_level(MathLevel::Mathe3);
    g.key_box_tries = 2;
    let s = g.to_save();
    assert_eq!(s.version, SAVE_VERSION);
    assert_eq!(SAVE_VERSION, 3);
    assert_eq!(s.math_level, "mathe3");
    let r = Game::from_save_json(common::zoo(), &s.to_json()).expect("restores");
    assert!(r.has_cart_key && r.key_box_open && r.note_read);
    assert_eq!(r.key_box_tries, 2);
    assert_eq!(r.math_level(), MathLevel::Mathe3);
    assert_eq!(r.to_save(), s);
    // the key without an open box repairs the box
    let mut broken = s.clone();
    broken.key_box_open = false;
    let r = Game::from_save(common::zoo(), &broken).expect("restores");
    assert!(r.key_box_open && r.has_cart_key);
    // a closed box stays a target after restore; an open one does not
    let mut g = game(6);
    let s = g.to_save();
    let mut r = Game::from_save(common::zoo(), &s).unwrap();
    stand_at(&mut r, BOX);
    assert!(matches!(r.available_target(), Some(Target::KeyBox { .. })));
    g.key_box_open = true;
    let mut r = Game::from_save(common::zoo(), &g.to_save()).unwrap();
    stand_at(&mut r, BOX);
    assert_eq!(r.available_target(), None);
}

// CART-027 / SAVE-013: a version-2 save loads with defaults
#[test]
fn save_013_v2_save_loads_with_defaults() {
    let g = game(8);
    let mut v: serde_json::Value = serde_json::from_str(&g.to_save().to_json()).unwrap();
    let obj = v.as_object_mut().unwrap();
    for k in [
        "has_cart_key",
        "key_box_open",
        "note_read",
        "key_box_tries",
        "math_level",
    ] {
        obj.remove(k);
    }
    obj.insert("version".into(), 2.into());
    let json = v.to_string();
    let s = SaveState::from_json(&json).expect("v2 parses");
    assert_eq!(s.version, 2);
    let r = Game::from_save(common::zoo(), &s).expect("v2 restores");
    assert!(!r.has_cart_key && !r.key_box_open && !r.note_read);
    assert_eq!(r.key_box_tries, 0);
    assert_eq!(r.math_level(), MathLevel::Mathe1);
    // a version above 3 starts a new game (SAVE-005)
    obj_version(&mut v, 4);
    assert!(SaveState::from_json(&v.to_string()).is_err());
}

fn obj_version(v: &mut serde_json::Value, n: u32) {
    v.as_object_mut()
        .unwrap()
        .insert("version".into(), n.into());
}
