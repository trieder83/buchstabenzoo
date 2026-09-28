//! GAME-PLAYER §4: reading panels open and close by themselves (PLAY-023…027, unit level).

mod common;

use glam::Vec2;
use zoo_core::game::{GameEvent, Target, PANEL_CLOSE_S, PANEL_SETTLE_S};
use zoo_core::{Food, Game};

const DT: f32 = 1.0 / 60.0;

/// Idles for `seconds` (no joystick input) and returns the panel events.
fn idle(g: &mut Game, seconds: f32) -> Vec<GameEvent> {
    let mut out = Vec::new();
    for _ in 0..(seconds / DT).round() as usize {
        g.update(DT, Vec2::ZERO);
        out.extend(g.drain_events().into_iter().filter(|e| {
            matches!(
                e,
                GameEvent::PanelOpened { .. } | GameEvent::PanelClosed { .. }
            )
        }));
    }
    out
}

fn place(g: &mut Game, pos: Vec2, facing: Vec2) {
    g.player.pos = pos;
    g.player.facing = facing.normalize();
}

fn food_box(g: &Game, food: Food) -> Vec2 {
    g.food_boxes.iter().find(|b| b.0 == food).unwrap().1
}

const ZEBRA_BOARD: Vec2 = Vec2::new(-8.5, 10.5); // readable side faces east

// PLAY-023 (unit level; the e2e test runs in the browser)
#[test]
fn play_023_board_panel_opens_after_settle_and_closes_when_leaving() {
    let mut g = common::game(1);
    let board = Target::InfoBoard { animal: "zebra" };
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    // not before the settle time
    assert!(idle(&mut g, PANEL_SETTLE_S - 0.05).is_empty());
    assert_eq!(g.panel.open, None);
    let ev = idle(&mut g, 0.1);
    assert_eq!(
        ev,
        vec![GameEvent::PanelOpened {
            target: board.clone()
        }]
    );
    assert_eq!(g.panel.open, Some(board.clone()));
    // auto-open reads the board: the mission starts (RESC-012)
    assert!(g.mission("zebra").unwrap().started);

    // hysteresis: 2.3 m away (out of the 2 m range) keeps it open
    place(&mut g, ZEBRA_BOARD + Vec2::new(2.3, 0.0), Vec2::NEG_X);
    assert!(idle(&mut g, 1.0).is_empty());
    assert_eq!(g.panel.open, Some(board.clone()));
    // > 2.5 m: closes after 0.3 s (not at once)
    place(&mut g, ZEBRA_BOARD + Vec2::new(2.8, 0.0), Vec2::NEG_X);
    assert!(idle(&mut g, PANEL_CLOSE_S - 0.05).is_empty());
    let ev = idle(&mut g, 0.1);
    assert_eq!(
        ev,
        vec![GameEvent::PanelClosed {
            target: board.clone()
        }]
    );
    assert_eq!(g.panel.open, None);

    // turning away closes it as well
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    idle(&mut g, 0.5);
    assert_eq!(g.panel.open, Some(board.clone()));
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::X);
    let ev = idle(&mut g, 0.5);
    assert_eq!(ev, vec![GameEvent::PanelClosed { target: board }]);
}

// PLAY-024
#[test]
fn play_024_walking_past_does_not_flash_a_panel() {
    let mut g = common::game(1);
    let grass = food_box(&g, Food::Grass);
    // available for 0.2 s only (passing by, then turned away)
    place(&mut g, Vec2::new(grass.x, 9.6), Vec2::Y);
    assert_eq!(
        g.available_target(),
        Some(Target::FoodBox { food: Food::Grass })
    );
    assert!(idle(&mut g, 0.2).is_empty());
    place(&mut g, Vec2::new(grass.x + 0.3, 9.4), Vec2::NEG_Y); // turned away
    assert!(idle(&mut g, 1.0).is_empty());
    assert_eq!(g.panel.open, None);

    // walking east along the ring path in front of the storage (boxes > 2 m away): nothing
    // opens. (Hugging the box row at 1 m does open each box in turn — every box is ahead
    // within ±75° for ~0.45 s; see the open issue in the M4b report.)
    let mut g = common::game(1);
    let mut events = Vec::new();
    place(&mut g, Vec2::new(-4.5, 8.6), Vec2::X);
    for _ in 0..(5.0 / DT) as usize {
        g.update(DT, Vec2::X);
        events.extend(
            g.drain_events()
                .into_iter()
                .filter(|e| matches!(e, GameEvent::PanelOpened { .. })),
        );
    }
    assert!(
        g.player.pos.x > 2.0,
        "walked along the boxes: {}",
        g.player.pos
    );
    assert!(events.is_empty(), "{events:?}");
}

// PLAY-026
#[test]
fn play_026_closed_by_hand_stays_closed_until_leaving_range() {
    let mut g = common::game(1);
    let board = Target::InfoBoard { animal: "zebra" };
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    idle(&mut g, 0.5);
    assert_eq!(g.panel.open, Some(board.clone()));
    g.close_panel();
    assert_eq!(g.panel.open, None);
    // stays closed while in range and facing, also when moving a little
    assert!(idle(&mut g, 2.0).is_empty());
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.2, 0.3), Vec2::NEG_X);
    assert!(idle(&mut g, 1.0).is_empty());
    // just out of the 2 m range is not "left" yet (hysteresis 2.5 m)
    place(&mut g, ZEBRA_BOARD + Vec2::new(2.2, 0.0), Vec2::NEG_X);
    idle(&mut g, 0.5);
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    assert!(idle(&mut g, 1.0).is_empty());
    // left the range and came back: opens again
    place(&mut g, ZEBRA_BOARD + Vec2::new(3.5, 0.0), Vec2::NEG_X);
    idle(&mut g, 0.2);
    place(&mut g, ZEBRA_BOARD + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    assert_eq!(
        idle(&mut g, 0.5),
        vec![GameEvent::PanelOpened { target: board }]
    );
    // interact (the button) still opens a panel by hand even when suppressed
    g.close_panel();
    assert!(matches!(
        g.interact(),
        Some(zoo_core::Interaction::InfoBoard(_))
    ));
    assert!(g.panel.open.is_some());
}

// PLAY-027
#[test]
fn play_027_food_only_taken_by_the_take_button() {
    let mut g = common::game(1);
    let grass = food_box(&g, Food::Grass);
    place(&mut g, Vec2::new(grass.x, 9.6), Vec2::Y);
    let ev = idle(&mut g, 1.0);
    assert_eq!(
        ev,
        vec![GameEvent::PanelOpened {
            target: Target::FoodBox { food: Food::Grass }
        }]
    );
    idle(&mut g, 3.0);
    assert_eq!(g.carry.food(), None, "walking up never takes food");
    g.take_food(Food::Grass).unwrap();
    assert_eq!(g.carry.food(), Some(Food::Grass));
}

// GAME-PLAYER §4: another interactable becoming nearer closes the panel, then opens its own
#[test]
fn nearer_interactable_switches_the_panel() {
    let mut g = common::game(1);
    let grass = food_box(&g, Food::Grass);
    let bamboo = food_box(&g, Food::Bamboo);
    place(&mut g, Vec2::new(grass.x, 9.6), Vec2::Y);
    idle(&mut g, 0.5);
    assert_eq!(g.panel.open, Some(Target::FoodBox { food: Food::Grass }));
    place(&mut g, Vec2::new(bamboo.x, 9.6), Vec2::Y);
    let ev = idle(&mut g, 1.0);
    assert_eq!(
        ev,
        vec![
            GameEvent::PanelClosed {
                target: Target::FoodBox { food: Food::Grass }
            },
            GameEvent::PanelOpened {
                target: Target::FoodBox { food: Food::Bamboo }
            }
        ]
    );
}
