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
    let board = zoo_core::Rect::new(-9, 15, 1, 1);
    assert!(in_interaction_range(
        board.distance_to(Vec2::new(-6.1, 15.5))
    ));
    assert!(!in_interaction_range(
        board.distance_to(Vec2::new(-5.9, 15.5))
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
    // GAME-PLAYER §6 (user decisions 2026-09-26): 1.93 m/s on paths (1.75 + 10 %)
    assert!((params.walk_speed - 1.93).abs() < 1e-6);
    assert!((d - 1.93).abs() <= 1.93 * 0.05, "moved {d} m");
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
    // GAME-PLAYER §6: grass 1.45 m/s (factor ≈ 0.75), faster than before and slower than paths
    let expected = params.walk_speed * params.grass_speed_factor;
    assert!((expected - 1.45).abs() < 1e-4, "grass speed {expected}");
    assert!((d - 1.45).abs() <= 1.45 * 0.05, "moved {d} m");
}

// GAME-PLAYER §6 / ART-RIG §4.7 (RIG-016): walk clip playback = speed ÷ 1.4, clamped to
// [0.8, 1.4] — 1.93 m/s on paths plays at ≈ 1.38
#[test]
fn play_005_006_walk_clip_rate_matches_speed() {
    use zoo_core::player::walk_clip_rate;
    let params = MoveParams::default();
    assert!((walk_clip_rate(params.speed_on(Surface::Path)) - 1.93 / 1.4).abs() < 1e-5);
    assert!((walk_clip_rate(1.93) - 1.38).abs() < 0.005);
    // grass 1.45 / 1.4 ≈ 1.04
    assert!((walk_clip_rate(params.speed_on(Surface::Grass)) - 1.45 / 1.4).abs() < 1e-5);
    assert!((walk_clip_rate(1.2) - 1.2 / 1.4).abs() < 1e-5);
    assert!((walk_clip_rate(2.2) - 1.4).abs() < 1e-5);
}

// PLAY-007
#[test]
fn play_007_speed_blends_within_0_2s() {
    let start = Vec2::new(-3.5, 3.5); // plaza; grass begins at x = -6
    let (level, mut p, params) = setup(start);
    let grass = params.walk_speed * params.grass_speed_factor;
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
    assert!(
        max_jump < (params.walk_speed - grass) * 0.5,
        "instant jump {max_jump}"
    );
}

// GAME-PLAYER §6 / GAME-LAYOUT: solid cells are not walkable
#[test]
fn cannot_enter_solid_cells() {
    // in front of the food storage's south facade beside the door (solid from z = 11; the
    // door cell (0, 11) is walkable since the storage is enterable, LAYOUT-041)
    let start = Vec2::new(-1.5, 9.5);
    let (level, mut p, params) = setup(start);
    walk(&level, &mut p, &params, Vec2::Y, 3.0);
    assert!(p.pos.y < 11.0, "walked into the storage wall: {}", p.pos);
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

// ------------------------------------------------------------------ collision and interaction

use glam::IVec2;
use zoo_core::collision::{footprint, Colliders, Shape, PLAYER_RADIUS_M};
use zoo_core::coords::level_to_world;
use zoo_core::game::Target;
use zoo_core::scene::Placement;
use zoo_core::Food;

/// No blocked cell and no prop shape overlaps the player circle at `p` (1 mm slack).
fn overlap_free(level: &Level, colliders: &Colliders, p: Vec2) -> bool {
    let r = PLAYER_RADIUS_M;
    let cells_ok = (-1..=1).all(|dz| {
        (-1..=1).all(|dx| {
            let c = cell_of(p) + IVec2::new(dx, dz);
            level.grid().is_walkable(c, false) || Shape::cell(c).push_out(p, r).length() < 1e-3
        })
    });
    cells_ok
        && colliders
            .shapes()
            .iter()
            .all(|s| s.push_out(p, r).length() < 1e-3)
}

/// Walks with collision and checks every frame that nothing overlaps.
fn walk_checked(
    level: &Level,
    colliders: &Colliders,
    p: &mut Player,
    dir: Vec2,
    seconds: f32,
) -> Vec2 {
    let params = MoveParams::default();
    for _ in 0..(seconds / DT).round() as usize {
        p.step_with(level.grid(), colliders, &params, dir, DT, false);
        assert!(
            overlap_free(level, colliders, p.pos),
            "overlap at {}",
            p.pos
        );
    }
    p.pos
}

fn player_at(pos: Vec2) -> Player {
    Player::new(pos, Vec2::Y, &MoveParams::default())
}

// PLAY-019
#[test]
fn play_019_props_are_solid_and_player_slides() {
    let level = Level::new(common::level1());
    let col = level.colliders();
    // Info board of the zebra enclosure (cell (-9, 10), readable side east): walk west into it.
    let mut p = player_at(Vec2::new(-6.5, 10.5));
    let end = walk_checked(&level, col, &mut p, Vec2::NEG_X, 3.0);
    assert!(end.x > -8.0 + PLAYER_RADIUS_M - 0.01, "{end}");
    // Slide along the board and fence north-west: keeps moving north.
    let end2 = walk_checked(&level, col, &mut p, Vec2::new(-1.0, 1.0).normalize(), 1.0);
    assert!(end2.y > end.y + 0.5, "no slide {end} -> {end2}");

    // Enclosure sign of the zebra gate: beside the gate, the whole sign solid (LAYOUT-033,
    // Q-086 (b)); readable side east. Walk west into it, then slide along it.
    let scene = zoo_core::scene::LevelScene::build(&level.data);
    let gate = Vec2::new(-9.5, 13.0);
    let sign = scene
        .placements
        .iter()
        .filter(|p| p.model == "enclosure_sign")
        .map(|p| zoo_core::coords::world_to_level(p.pos))
        .min_by(|a, b| a.distance(gate).total_cmp(&b.distance(gate)))
        .expect("zebra sign");
    assert!((sign.y - gate.y).abs() > 2.6, "beside the gate: {sign}");
    let mut p = player_at(sign + Vec2::new(1.5, 0.0));
    let end = walk_checked(&level, col, &mut p, Vec2::NEG_X, 2.0);
    assert!(
        end.x >= sign.x + 0.36 + PLAYER_RADIUS_M - 0.02,
        "walked into the sign: {end}"
    );
    let away = (sign.y - gate.y).signum();
    let end2 = walk_checked(
        &level,
        col,
        &mut p,
        Vec2::new(-1.0, 0.6 * away).normalize(),
        1.0,
    );
    assert!((end2 - end).length() > 0.3, "stuck at the sign: {end2}");

    // Plaza bench (cells (6..7, 4)): walk north into it from the plaza.
    let mut p = player_at(Vec2::new(7.0, 2.5));
    let end = walk_checked(&level, col, &mut p, Vec2::Y, 2.0);
    assert!(end.y <= 4.0 - PLAYER_RADIUS_M + 0.01, "{end}");
    let end2 = walk_checked(&level, col, &mut p, Vec2::new(1.0, 1.0).normalize(), 1.0);
    assert!(end2.x > end.x + 0.5, "no slide along the bench: {end2}");

    // A tree on open ground (trunk circle): the player is stopped and slides around it.
    let tree = Placement::new("tree_round", level_to_world(Vec2::new(-2.0, 5.0)), 0.0);
    let trees = Colliders::from_placements(&[tree], level.data.level.bounds);
    let mut p = player_at(Vec2::new(-2.0, 3.5));
    let end = walk_checked(&level, &trees, &mut p, Vec2::Y, 2.0);
    assert!(end.y < 5.0 - 0.45 - PLAYER_RADIUS_M + 0.01, "{end}");
    let mut p = player_at(Vec2::new(-2.1, 3.5));
    let end = walk_checked(&level, &trees, &mut p, Vec2::Y, 3.0);
    assert!(end.y > 5.5, "did not slide around the trunk: {end}");
}

// PLAY-020
#[test]
fn play_020_board_available_only_in_front_and_facing() {
    let mut g = common::game(1);
    let board = Vec2::new(-8.5, 10.5); // readable side faces east (away from enc_zebra)
    let at = |g: &mut zoo_core::Game, pos: Vec2, facing: Vec2| {
        g.player.pos = pos;
        g.player.facing = facing.normalize();
        g.available_target()
    };
    let zebra_board = Some(Target::InfoBoard { animal: "zebra" });
    assert_eq!(
        at(&mut g, board + Vec2::new(1.5, 0.0), Vec2::NEG_X),
        zebra_board
    );
    // slightly off-axis but within ±60° and facing within ±75°
    assert_eq!(
        at(&mut g, board + Vec2::new(1.2, 0.8), Vec2::new(-1.0, -0.3)),
        zebra_board
    );
    // behind (enclosure side)
    assert_eq!(at(&mut g, board + Vec2::new(-1.5, 0.0), Vec2::X), None);
    // beside (north, 90° off the facing)
    assert_eq!(at(&mut g, board + Vec2::new(0.0, 1.5), Vec2::NEG_Y), None);
    // in front but facing away
    assert_eq!(at(&mut g, board + Vec2::new(1.5, 0.0), Vec2::X), None);
    // in front, facing it, but out of range
    assert_eq!(at(&mut g, board + Vec2::new(2.1, 0.0), Vec2::NEG_X), None);
    // interacting there opens the riddle (PLAY-010 on unit level) and starts the mission
    at(&mut g, board + Vec2::new(1.5, 0.0), Vec2::NEG_X);
    assert!(matches!(
        g.interact(),
        Some(zoo_core::Interaction::InfoBoard(_))
    ));
    assert!(g.mission("zebra").unwrap().started);
}

// PLAY-021
#[test]
fn play_021_nearest_available_wins() {
    let mut g = common::game(1);
    let grass = g.food_boxes.iter().find(|b| b.0 == Food::Grass).unwrap().1;
    let bamboo = g.food_boxes.iter().find(|b| b.0 == Food::Bamboo).unwrap().1;
    g.player.facing = Vec2::Y;
    g.player.pos = Vec2::new(grass.x - 0.2, 9.4);
    let all: Vec<_> = g
        .interactables()
        .into_iter()
        .filter(|i| g.is_available(i))
        .collect();
    assert!(all.len() >= 2, "{all:?}");
    assert_eq!(
        g.available_target(),
        Some(Target::FoodBox { food: Food::Grass })
    );
    g.player.pos = Vec2::new(bamboo.x + 0.2, 9.4);
    assert_eq!(
        g.available_target(),
        Some(Target::FoodBox { food: Food::Bamboo })
    );
}

// PLAY-022
#[test]
fn play_022_ground_decoration_does_not_block() {
    for m in [
        "grass_tuft",
        "lily_pad",
        "flower_bed",
        "reed",
        "path_edge",
        "grass_tile",
    ] {
        assert!(footprint(m).is_empty(), "{m} must not be solid");
    }
    let level = Level::new(common::level1());
    let deco: Vec<Placement> = [(-2.0, 4.0), (-2.0, 5.0), (-2.2, 6.0)]
        .into_iter()
        .zip(["grass_tuft", "lily_pad", "flower_bed"])
        .map(|((x, z), model)| Placement::new(model, level_to_world(Vec2::new(x, z)), 0.0))
        .collect();
    let col = Colliders::from_placements(&deco, level.data.level.bounds);
    assert!(col.shapes().is_empty());
    let mut p = player_at(Vec2::new(-2.0, 3.0));
    let end = walk_checked(&level, &col, &mut p, Vec2::Y, 3.0);
    assert!(
        (end.y - (3.0 + 3.0 * MoveParams::default().walk_speed)).abs() < 0.1,
        "blocked by ground decoration: {end}"
    );
}
