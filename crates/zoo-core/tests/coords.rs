//! GAME-LAYOUT "Coordinate spaces" (Q-056): level (x east, z north) vs. world (right-handed,
//! Y-up, north = −Z).

mod common;

use glam::camera::rh::{proj::opengl, view::look_at_mat4};
use glam::{Mat4, Quat, Vec2, Vec3};
use zoo_core::coords::{
    level_to_world, level_to_world_at, quarter_turns_cw_to_yaw, world_to_level, WORLD_EAST,
    WORLD_NORTH, WORLD_UP,
};
use zoo_core::level::cell_center;
use zoo_core::Rect;

const EPS: f32 = 1e-5;

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).length() < EPS
}

/// Screen position (NDC x, y) of a world point.
fn ndc(view_proj: Mat4, w: Vec3) -> Vec2 {
    let c = view_proj * w.extend(1.0);
    assert!(c.w > 0.0, "point {w} behind the camera");
    Vec2::new(c.x / c.w, c.y / c.w)
}

/// Default follow camera of GAME-PLAYER §2 in world space: south of the target looking
/// north, pitch 55°, 14 m, 35° vertical FOV, portrait 1080×2340.
fn default_camera(target: Vec3) -> Mat4 {
    let pitch = 55f32.to_radians();
    let d = 14.0;
    let eye = target - WORLD_NORTH * d * pitch.cos() + WORLD_UP * d * pitch.sin();
    let view = look_at_mat4(eye, target, WORLD_UP);
    let proj = opengl::perspective(35f32.to_radians(), 1080.0 / 2340.0, 0.1, 200.0);
    proj * view
}

fn rect_center(r: Rect) -> Vec2 {
    Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0)
}

/// Shoelace sum: > 0 counter-clockwise, < 0 clockwise (x right, y up).
fn signed_area(pts: &[Vec2]) -> f32 {
    (0..pts.len())
        .map(|i| pts[i].perp_dot(pts[(i + 1) % pts.len()]))
        .sum::<f32>()
        / 2.0
}

// LAYOUT-007
#[test]
fn layout_007_round_trip_level_world_level() {
    for x in -30..=30 {
        for z in -30..=30 {
            let p = Vec2::new(x as f32 * 0.75, z as f32 * 1.25);
            let w = level_to_world(p);
            assert_eq!(w, Vec3::new(p.x, 0.0, -p.y));
            assert_eq!(world_to_level(w), p);
            assert_eq!(world_to_level(level_to_world_at(p, 2.5)), p);
        }
    }
}

// LAYOUT-008
#[test]
fn layout_008_camera_looking_north_has_east_on_the_right() {
    let forward = WORLD_NORTH;
    assert_eq!(forward, Vec3::NEG_Z);
    let right = forward.cross(WORLD_UP);
    assert!(close(right, level_to_world(Vec2::X)), "right = {right}");
    assert!(close(right, WORLD_EAST));
    assert!(close(forward, level_to_world(Vec2::Y)));
    // same through a real view matrix: view-space +X is screen right
    let view = look_at_mat4(Vec3::ZERO, forward, WORLD_UP);
    let east_in_view = view.transform_vector3(level_to_world(Vec2::X));
    assert!(
        close(east_in_view, Vec3::X),
        "east in view = {east_in_view}"
    );
}

// LAYOUT-009
#[test]
fn layout_009_east_of_spawn_is_right_on_screen() {
    let data = common::level1();
    assert_eq!(data.spawn.facing, "+z");
    let spawn = level_to_world(cell_center(data.spawn.cell()));
    let vp = default_camera(spawn);
    let s = ndc(vp, spawn);
    let at = |id: &str| {
        let e = data.element(id).unwrap_or_else(|| panic!("{id} missing"));
        ndc(vp, level_to_world(rect_center(e.rect)))
    };
    let bench = at("bench_plaza"); // east of the spawn
    let map = at("map_board"); // west of the spawn
    let storage = at("food_storage"); // north of the spawn
    assert!(bench.x > s.x, "bench {bench} not right of spawn {s}");
    assert!(map.x < s.x, "map board {map} not left of spawn {s}");
    assert!(
        storage.y > s.y,
        "food storage {storage} not above spawn {s}"
    );
}

// LAYOUT-010
#[test]
fn layout_010_clockwise_ring_stays_clockwise_from_above() {
    // clockwise in the map (north up, east right): N, NE, E, SE, S, SW, W, NW
    let ring: Vec<Vec2> = [
        (0.0, 5.0),
        (4.0, 4.0),
        (5.0, 0.0),
        (4.0, -4.0),
        (0.0, -5.0),
        (-4.0, -4.0),
        (-5.0, 0.0),
        (-4.0, 4.0),
    ]
    .iter()
    .map(|&(x, z)| Vec2::new(x, z) + Vec2::new(3.0, 7.0))
    .collect();
    assert!(
        signed_area(&ring) < 0.0,
        "test ring must be clockwise in the map"
    );

    // top-down camera over the ring: looking down −Y, screen up = north
    let centre = level_to_world(Vec2::new(3.0, 7.0));
    let view = look_at_mat4(centre + WORLD_UP * 30.0, centre, WORLD_NORTH);
    let proj = opengl::orthographic(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0);
    let screen: Vec<Vec2> = ring
        .iter()
        .map(|&p| ndc(proj * view, level_to_world(p)))
        .collect();
    assert!(
        signed_area(&screen) < 0.0,
        "ring became counter-clockwise (mirrored)"
    );
    // and the picture is the map itself: same positions up to scale
    for (p, q) in ring.iter().zip(&screen) {
        let expect = (*p - Vec2::new(3.0, 7.0)) / 10.0;
        assert!(
            (expect - *q).length() < 1e-4,
            "{p} -> {q}, expected {expect}"
        );
    }
}

// LAYOUT-011
#[test]
fn layout_011_negative_yaw_is_clockwise_from_above() {
    let north = level_to_world(Vec2::Y);
    let east = level_to_world(Vec2::X);
    let west = level_to_world(Vec2::NEG_X);
    let south = level_to_world(Vec2::NEG_Y);
    assert!(close(
        Quat::from_rotation_y(-90f32.to_radians()) * north,
        east
    ));
    assert!(close(
        Quat::from_rotation_y(90f32.to_radians()) * north,
        west
    ));
    let turns = [north, east, south, west];
    for (k, want) in turns.iter().enumerate() {
        let got = Quat::from_rotation_y(quarter_turns_cw_to_yaw(k as i32)) * north;
        assert!(close(got, *want), "k = {k}: {got} != {want}");
    }
}
