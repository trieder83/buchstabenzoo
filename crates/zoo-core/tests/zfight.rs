//! ARCH-005: no z-fighting between placeholder boxes of the assembled scene.
//!
//! Two faces that lie in the same plane, face the same way and overlap make the GPU flicker
//! between their colours while the camera moves (user report 2026-09-26: the entrance arch
//! top flickered red/blue because the pillar tops and the roof beam top were both at 5.0 m).
//! Boxes must either overlap by a clear margin or not share a face plane.

mod common;

use glam::Vec3;
use zoo_core::scene::{BoxPlacement, LevelScene};

/// Faces closer than this (m) count as coplanar.
const PLANE_EPS: f32 = 0.002;
/// Overlap area (m²) below which touching edges are ignored.
const AREA_EPS: f32 = 1e-4;

/// Axis-aligned bounds of a box (yaw must be a multiple of 90°).
fn aabb(b: &BoxPlacement) -> Option<(Vec3, Vec3)> {
    let quarter = (b.yaw / std::f32::consts::FRAC_PI_2).round();
    if (b.yaw - quarter * std::f32::consts::FRAC_PI_2).abs() > 1e-3 {
        return None; // rotated boxes are not checked here
    }
    let (sx, sz) = if (quarter as i32).rem_euclid(2) == 1 {
        (b.size.z, b.size.x)
    } else {
        (b.size.x, b.size.z)
    };
    let min = Vec3::new(b.pos.x - sx / 2.0, b.pos.y, b.pos.z - sz / 2.0);
    let max = Vec3::new(b.pos.x + sx / 2.0, b.pos.y + b.size.y, b.pos.z + sz / 2.0);
    Some((min, max))
}

fn overlap(a0: f32, a1: f32, b0: f32, b1: f32) -> f32 {
    (a1.min(b1) - a0.max(b0)).max(0.0)
}

/// Returns a description of every pair of boxes with coplanar, same-facing, overlapping faces.
fn z_fights(boxes: &[BoxPlacement]) -> Vec<String> {
    let bounds: Vec<_> = boxes.iter().map(aabb).collect();
    let mut found = Vec::new();
    for i in 0..boxes.len() {
        let Some((a0, a1)) = bounds[i] else { continue };
        for j in i + 1..boxes.len() {
            let Some((b0, b1)) = bounds[j] else { continue };
            for axis in 0..3 {
                let (u, v) = match axis {
                    0 => (1, 2),
                    1 => (0, 2),
                    _ => (0, 1),
                };
                let area =
                    overlap(a0[u], a1[u], b0[u], b1[u]) * overlap(a0[v], a1[v], b0[v], b1[v]);
                if area < AREA_EPS {
                    continue;
                }
                // same-facing faces in the same plane: min-min or max-max
                let plane = if (a0[axis] - b0[axis]).abs() < PLANE_EPS && axis != 1 {
                    Some(("min", a0[axis]))
                } else if (a1[axis] - b1[axis]).abs() < PLANE_EPS {
                    Some(("max", a1[axis]))
                } else {
                    None
                };
                // bottoms on the ground (y = 0) are never visible from the game camera
                if let Some((side, at)) = plane {
                    found.push(format!(
                        "{} / {}: {side} face on axis {} at {at:.3} (overlap {area:.3} m²)",
                        boxes[i].source,
                        boxes[j].source,
                        ["x", "y", "z"][axis]
                    ));
                }
            }
        }
    }
    found
}

#[test]
fn arch_005_no_z_fighting_in_the_joined_zoo() {
    let scene = LevelScene::build(&common::zoo());
    let mut all = scene.boxes.clone();
    for f in &scene.fallbacks {
        all.extend(f.boxes.iter().cloned());
    }
    let fights = z_fights(&all);
    assert!(
        fights.is_empty(),
        "z-fighting faces:\n{}",
        fights.join("\n")
    );
}

#[test]
fn arch_005_detector_finds_the_entrance_case() {
    // pillar 0..5 m and a beam 4.3..5 m spanning it: tops coplanar at 5.0 m and the outer
    // side faces coplanar at x = -3.0
    let b = |x: f32, y0: f32, size: Vec3| BoxPlacement {
        pos: Vec3::new(x, y0, 0.0),
        size,
        yaw: 0.0,
        color: [1.0; 3],
        fadeable: false,
        source: "test".into(),
        part: 0,
    };
    let boxes = [
        b(-2.4, 0.0, Vec3::new(1.2, 5.0, 1.8)),
        b(0.0, 4.3, Vec3::new(6.0, 0.7, 0.8)),
    ];
    assert_eq!(z_fights(&boxes).len(), 2);
    let fixed = [
        b(-2.4, 0.0, Vec3::new(1.2, 4.3, 1.8)),
        b(0.0, 4.3, Vec3::new(6.0, 0.7, 0.8)),
    ];
    assert!(z_fights(&fixed).is_empty());
}
