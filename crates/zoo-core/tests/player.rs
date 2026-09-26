//! GAME-PLAYER movement and interaction tests.

mod common;

use glam::Vec2;
use zoo_core::level::{cell_of, Level, Surface};
use zoo_core::player::{in_interaction_range, MoveParams, Player};

const DT: f32 = 1.0 / 60.0;

fn setup(pos: Vec2) -> (Level, Player, MoveParams) {
    let level = Level::new(common::level1());
    let params = MoveParams::default();
    let mut p = Player::new(pos, Vec2::Y, &params);
    // settle the surface speed on the start cell
    for _ in 0..60 {
        p.step(level.grid(), &params, Vec2::ZERO, DT, false);
    }
    (level, p, params)
}

fn walk(level: &Level, p: &mut Player, params: &MoveParams, dir: Vec2, seconds: f32) {
    let n = (seconds / DT).round() as usize;
    for _ in 0..n {
        p.step(level.grid(), params, dir, DT, false);
    }
}

// PLAY-003
#[test]
fn play_003_interaction_range_2m() {
    assert!(in_interaction_range(1.9));
    assert!(!in_interaction_range(2.1));
    // against a rectangle target (info board cell)
    let board = zoo_core::Rect::new(-9, 14, 1, 1);
    assert!(in_interaction_range(
        board.distance_to(Vec2::new(-6.1, 14.5))
    ));
    assert!(!in_interaction_range(
        board.distance_to(Vec2::new(-5.9, 14.5))
    ));
}

// PLAY-005
#[test]
fn play_005_path_speed() {
    let start = Vec2::new(-7.5, 9.5); // path_ring_s
    let (level, mut p, params) = setup(start);
    assert_eq!(level.grid().surface(cell_of(start)), Some(Surface::Path));
    walk(&level, &mut p, &params, Vec2::X, 1.0);
    let d = p.pos.distance(start);
    assert!((d - 1.4).abs() <= 1.4 * 0.05, "moved {d} m");
    assert_eq!(level.grid().surface(cell_of(p.pos)), Some(Surface::Path));
}

// PLAY-006
#[test]
fn play_006_grass_speed() {
    let start = Vec2::new(-21.5, 3.5); // open grass west of the plaza
    let (level, mut p, params) = setup(start);
    assert_eq!(level.grid().surface(cell_of(start)), Some(Surface::Grass));
    walk(&level, &mut p, &params, Vec2::X, 1.0);
    let d = p.pos.distance(start);
    let expected = 1.4 * params.grass_speed_factor;
    assert!((d - expected).abs() <= expected * 0.05, "moved {d} m");
}

// PLAY-007
#[test]
fn play_007_speed_blends_within_0_2s() {
    let start = Vec2::new(-3.5, 3.5); // plaza; grass begins at x = -6
    let (level, mut p, params) = setup(start);
    let grass = 1.4 * params.grass_speed_factor;
    let mut t = 0.0;
    let mut entered_grass_at = None;
    let mut prev_speed = p.surface_speed();
    let mut max_jump: f32 = 0.0;
    while t < 4.0 {
        p.step(level.grid(), &params, Vec2::NEG_X, DT, false);
        t += DT;
        max_jump = max_jump.max((p.surface_speed() - prev_speed).abs());
        prev_speed = p.surface_speed();
        if entered_grass_at.is_none()
            && level.grid().surface(cell_of(p.pos)) == Some(Surface::Grass)
        {
            entered_grass_at = Some(t);
        }
        if let Some(t0) = entered_grass_at {
            if t - t0 >= 0.2 {
                break;
            }
        }
    }
    assert!(entered_grass_at.is_some());
    assert!(
        (p.surface_speed() - grass).abs() < 0.01,
        "speed {} after 0.2 s",
        p.surface_speed()
    );
    assert!(max_jump < (1.4 - grass) * 0.5, "instant jump {max_jump}");
}

// GAME-PLAYER §6 / GAME-LAYOUT: solid cells are not walkable
#[test]
fn cannot_enter_solid_cells() {
    let start = Vec2::new(0.5, 9.5); // in front of the food storage (solid from z = 11)
    let (level, mut p, params) = setup(start);
    walk(&level, &mut p, &params, Vec2::Y, 3.0);
    assert!(p.pos.y < 11.0, "entered the storage: {}", p.pos);
    // sliding along the wall
    let before = p.pos;
    walk(
        &level,
        &mut p,
        &params,
        Vec2::new(1.0, 1.0).normalize(),
        1.0,
    );
    assert!(p.pos.x > before.x + 0.5 && p.pos.y < 11.0);
}

// GAME-LAYOUT "enclosure": gates only while leading an animal
#[test]
fn gate_closed_without_animal() {
    let start = Vec2::new(-8.5, 12.5); // east of the zebra gate (-10, 12)
    let (level, mut p, params) = setup(start);
    walk(&level, &mut p, &params, Vec2::NEG_X, 3.0);
    assert!(p.pos.x >= -9.0, "entered the gate: {}", p.pos);
    let mut q = Player::new(start, Vec2::Y, &params);
    for _ in 0..180 {
        q.step(level.grid(), &params, Vec2::NEG_X, DT, true);
    }
    assert!(
        q.pos.x < -9.0 && q.pos.x >= -10.0,
        "gate passable while leading: {}",
        q.pos
    );
}

#[test]
fn deterministic_movement() {
    let run = || {
        let (level, mut p, params) = setup(Vec2::new(0.5, 2.5));
        walk(&level, &mut p, &params, Vec2::new(0.3, 1.0), 5.0);
        p.pos
    };
    assert_eq!(run(), run());
}
