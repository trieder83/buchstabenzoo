//! RESC-028: the big map board at the level entry shows the game description of its level
//! (GAME-RESCUE "Welcome board at the entrance").

mod common;

use glam::Vec2;
use zoo_core::content::{animal_name_key, sentence_word_counts, welcome_keys};
use zoo_core::game::{GameEvent, Target};
use zoo_core::{Game, Interaction, Language, ReadingLevel};

const DT: f32 = 1.0 / 60.0;

fn welcome_boards(g: &Game) -> Vec<zoo_core::Interactable> {
    g.interactables()
        .into_iter()
        .filter(|i| matches!(i.target, Target::WelcomeBoard { .. }))
        .collect()
}

fn stand_in_front(g: &mut Game, it: &zoo_core::Interactable) {
    let out = it.readable.unwrap();
    g.player.pos = it.point + out * 1.5;
    g.player.facing = -out;
}

// RESC-028 (unit level)
#[test]
fn resc_028_standing_in_front_opens_the_welcome_panel() {
    let mut g = common::game(1);
    let boards = welcome_boards(&g);
    assert_eq!(boards.len(), 1, "level 1 has one map board");
    assert_eq!(
        boards[0].target,
        Target::WelcomeBoard {
            level: "level_1".into()
        }
    );
    stand_in_front(&mut g, &boards[0]);
    let mut opened = false;
    for _ in 0..60 {
        g.update(DT, Vec2::ZERO);
        opened |= g.drain_events().iter().any(|e| {
            matches!(
                e,
                GameEvent::PanelOpened {
                    target: Target::WelcomeBoard { .. }
                }
            )
        });
    }
    assert!(opened, "the panel opens by itself");
    assert!(Target::WelcomeBoard { level: "x".into() }.is_reading());
    // reading it does not start any mission
    assert!(g.missions.iter().all(|m| !m.started));
    match g.interact() {
        Some(Interaction::WelcomeBoard { level, animals }) => {
            assert_eq!(level, "level_1");
            assert_eq!(animals, g.part_animal_ids(0));
            assert_eq!(animals.len(), 3, "zebra, hippo, panda (pairs count once)");
        }
        other => panic!("welcome board expected, got {other:?}"),
    }
}

// RESC-028: not an info board, not a mission board (HINT-001 keeps its list)
#[test]
fn resc_028_welcome_board_is_no_info_board() {
    let g = common::game(1);
    let info = g
        .interactables()
        .iter()
        .filter(|i| matches!(i.target, Target::InfoBoard { .. }))
        .count();
    assert_eq!(info, 3);
    assert_eq!(
        Target::WelcomeBoard {
            level: "level_1".into()
        }
        .kind(),
        "welcome_board"
    );
}

// RESC-028: every level with a map board has its own animals and texts; levels 2/3 are
// reachable through their gates and show their own animals.
#[test]
fn resc_028_levels_show_their_own_animals() {
    let mut g = common::zoo_game(1);
    assert_eq!(
        welcome_boards(&g).len(),
        1,
        "level 2/3 boards only when unlocked"
    );
    for b in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        assert!(g.level.open_barrier(b));
    }
    let boards = welcome_boards(&g);
    let levels: Vec<String> = boards
        .iter()
        .map(|b| match &b.target {
            Target::WelcomeBoard { level } => level.clone(),
            _ => unreachable!(),
        })
        .collect();
    assert!(levels.contains(&"level_1".to_owned()) && levels.contains(&"level_3".to_owned()));
    let k3 = g.level.data.part_index("level_3").unwrap();
    let mut a3 = g.part_animal_ids(k3);
    a3.sort_unstable();
    assert_eq!(a3, ["goldfish", "monkey", "snow_fox"]);
    let k1 = g.level.data.part_index("level_1").unwrap();
    assert_ne!(g.part_animal_ids(k1), g.part_animal_ids(k3));
}

// RESC-017 style: no raw keys, glossary and READ-002 for every level and language
#[test]
fn resc_028_texts_exist_for_every_level_and_language() {
    let c = common::content();
    let mut missing = Vec::new();
    for part in ["level_1", "level_2", "level_3", "night_1"] {
        for lang in Language::ALL {
            for rl in ReadingLevel::ALL {
                let k = welcome_keys(part, rl);
                let mut keys = vec![k.title, k.text, k.goal, k.start, k.level_text];
                keys.extend(k.steps);
                for key in keys {
                    match c.text(lang, &key) {
                        None => missing.push(format!("{}:{key}", lang.id())),
                        Some(t) => {
                            assert!(!t.trim().is_empty() && !t.contains("welcome-"), "{key}");
                            for bad in ["Käfig", "cage"] {
                                assert!(!t.contains(bad), "{key}: {bad}");
                            }
                            if rl == ReadingLevel::Klasse1 {
                                assert!(
                                    sentence_word_counts(&t).iter().all(|&n| n <= 5),
                                    "READ-002 {key}: {t}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    for a in [
        "zebra", "hippo", "panda", "koala", "goldfish", "monkey", "snow_fox",
    ] {
        for lang in Language::ALL {
            assert!(c.text(lang, &animal_name_key(a)).is_some(), "{a}");
        }
    }
    assert!(missing.is_empty(), "missing: {missing:?}");
}
