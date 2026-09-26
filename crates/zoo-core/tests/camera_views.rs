//! GAME-CAMERA-VIEWS: first person and look-around — facing by view direction (CAMV-006)
//! and the distance fog that keeps the riddles fair (CAMV-008) on the joined levels 1–3.

mod common;

use glam::{IVec2, Vec2};
use zoo_core::level::{cell_center, ElementType, Grid, Level};
use zoo_core::player::INTERACTION_RANGE_M;
use zoo_core::view::{hidden_by_fog, min_eye_distance, yaw_to_level, FOG_END_M};
use zoo_core::wander::hiding_area;
use zoo_core::Target;

/// The joined zoo with every barrier open (all hiding places reachable).
fn open_zoo() -> Level {
    let mut level = Level::new(common::zoo());
    for b in [
        "barrier_ne_tree",
        "barrier_l2_construction",
        "barrier_north_gate",
    ] {
        assert!(level.open_barrier(b), "{b} is a barrier");
    }
    level
}

fn walkable_adjacent(grid: &Grid, rect: zoo_core::Rect) -> Vec<IVec2> {
    zoo_core::Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
        .filter(|&c| !rect.contains(c) && grid.is_walkable(c, false))
        .collect()
}

/// Walkable cell centres within `r` metres of a point (where the player can read a board).
fn walkable_within(grid: &Grid, p: Vec2, r: f32) -> Vec<Vec2> {
    let n = r.ceil() as i32 + 1;
    let c = zoo_core::level::cell_of(p);
    zoo_core::Rect::new(c.x - n, c.y - n, 2 * n + 1, 2 * n + 1)
        .cells()
        .filter(|&k| grid.is_walkable(k, false))
        .map(cell_center)
        .filter(|q| q.distance(p) <= r)
        .collect()
}

/// Animal heights for the sight tests (GAME-LEVEL-1/2/3), m.
fn animal_height(animal: &str) -> f32 {
    match animal {
        "koala" => 0.9,
        "elephant" => 3.0,
        "giraffe" => 4.5,
        "lion" => 1.5,
        "monkey" => 1.1,
        "goldfish" => 0.3,
        "snow_fox" => 0.9,
        "panda" => 1.2,
        _ => 1.6,
    }
}

// CAMV-008: for every animal with an enclosure and every candidate hiding place, the animal
// spot and every wander cell (at 0.5 m, the animal's height and perch + 1 m) are fully hidden
// by the fog for every close-view eye of a player standing where she can read the own info
// board (walkable cell centres ≤ 2.5 m from it, the panel range) or next to the own
// enclosure gate — levels 1, 2 and 3 (same places and heights as LAYOUT-L1-006/L2-006/L3-006).
#[test]
fn camv_008_hiding_places_beyond_the_fog_from_their_board() {
    let level = open_zoo();
    let data = &level.data;
    let grid = level.grid();
    let mut problems = Vec::new();
    let mut nearest = (f32::MAX, String::new());
    let mut places = 0;
    for enc in data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Enclosure && e.animal.is_some())
    {
        let animal = enc.animal.as_deref().unwrap();
        let board = data
            .elements
            .iter()
            .find(|e| {
                e.kind.as_deref() == Some("info_board") && e.enclosure.as_deref() == Some(&enc.id)
            })
            .unwrap_or_else(|| panic!("{animal}: no info board"));
        let board_point = cell_center(IVec2::new(board.rect.x, board.rect.z));
        let mut stands = walkable_within(grid, board_point, INTERACTION_RANGE_M + 0.5);
        if let Some(gate) = enc.gate {
            stands.extend(walkable_adjacent(grid, gate).into_iter().map(cell_center));
        }
        assert!(!stands.is_empty(), "{animal}: nowhere to stand");
        let height = animal_height(animal);
        for h in data.hiding_places_of(animal) {
            places += 1;
            let mut pts = vec![
                (cell_center(h.spot_cell()), 0.5),
                (cell_center(h.spot_cell()), height),
            ];
            for (c, _) in hiding_area(&level, h).cells() {
                let p = cell_center(c);
                pts.push((p, 0.5));
                pts.push((p, height));
                if let Some(ph) = h.perch_height_m {
                    pts.push((p, ph + 1.0));
                }
            }
            for s in &stands {
                for &(p, y) in &pts {
                    let d = min_eye_distance(*s, p, y);
                    if d < nearest.0 {
                        nearest = (d, format!("{} ({animal}) {p} from {s}", h.id));
                    }
                    if !hidden_by_fog(*s, p, y) {
                        problems.push(format!(
                            "{} ({animal}): {p} at {y} m is {d:.1} m from {s}",
                            h.id
                        ));
                    }
                }
            }
        }
    }
    assert_eq!(places, 30, "10 animals × 3 candidates");
    assert!(
        problems.is_empty(),
        "hiding places inside the fog end {FOG_END_M} m: {problems:#?}"
    );
    eprintln!(
        "CAMV-008: nearest wander cell {:.2} m — {}",
        nearest.0, nearest.1
    );
}

// CAMV-006: in first person the facing is the view direction — walking sideways past an
// info board keeps looking at it (available), turning the view away makes it unavailable,
// and interaction works with the same GAME-PLAYER §5 rules.
#[test]
fn camv_006_first_person_interaction_by_view_direction() {
    let mut g = common::game(1);
    let board = Vec2::new(-8.5, 14.5); // zebra board, readable side east
    let zebra_board = Some(Target::InfoBoard { animal: "zebra" });
    g.player.pos = board + Vec2::new(1.6, -0.6);
    // view straight west (yaw 90°: counter-clockwise from north)
    let west = yaw_to_level(90f32.to_radians());
    assert!(west.abs_diff_eq(Vec2::NEG_X, 1e-5));
    g.player.lock_facing(Some(west));
    assert_eq!(g.available_target(), zebra_board);
    // strafe north (stick right while looking west = level north) for 0.5 s: facing stays
    for _ in 0..30 {
        g.update(1.0 / 60.0, Vec2::Y * 0.5);
    }
    assert!(
        g.player.pos.y > board.y - 0.6 + 0.3,
        "moved {}",
        g.player.pos
    );
    assert!(
        g.player.facing.abs_diff_eq(west, 1e-5),
        "facing {}",
        g.player.facing
    );
    assert_eq!(g.available_target(), zebra_board);
    // turning the view away (north) → not available any more
    g.player.lock_facing(Some(Vec2::Y));
    assert_eq!(g.available_target(), None);
    // looking at it again → available; interact reads the riddle (PLAY-010 on unit level)
    g.player.lock_facing(Some(west));
    assert!(matches!(
        g.interact(),
        Some(zoo_core::Interaction::InfoBoard(_))
    ));
    // leaving first person: facing follows the walk direction again
    g.player.lock_facing(None);
    g.update(1.0 / 60.0, Vec2::X);
    assert!(g.player.facing.abs_diff_eq(Vec2::X, 1e-5));
}
