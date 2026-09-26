//! LAYOUT-018 (GAME-LAYOUT "Collision footprints", Q-087): every prop footprint of
//! `zoo_core::collision::footprint` covers the exported mesh's cross-section between 0.05 m
//! and 1.4 m height (within 0.02 m) and extends at most 0.1 m beyond it. Measured like the
//! spec table: on the extents (bounding box) of the cross-section; round footprints may
//! leave the corners of box-shaped meshes uncovered by at most 0.1 m.

use std::path::PathBuf;

use glam::{Vec2, Vec3};
use zoo_assets::Model;
use zoo_core::collision::{footprint, LocalShape};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Clips a triangle to the slab `lo ≤ y ≤ hi` (Sutherland–Hodgman) and returns the
/// polygon's outline sampled every 5 cm, projected to (x, z).
fn slab_outline(tri: [Vec3; 3], lo: f32, hi: f32) -> Vec<Vec2> {
    let mut poly: Vec<Vec3> = tri.to_vec();
    for (plane, keep_above) in [(lo, true), (hi, false)] {
        let inside = |p: &Vec3| {
            if keep_above {
                p.y >= plane
            } else {
                p.y <= plane
            }
        };
        let mut out = Vec::new();
        for i in 0..poly.len() {
            let a = poly[i];
            let b = poly[(i + 1) % poly.len()];
            let (ia, ib) = (inside(&a), inside(&b));
            if ia {
                out.push(a);
            }
            if ia != ib {
                let t = (plane - a.y) / (b.y - a.y);
                out.push(a + (b - a) * t);
            }
        }
        poly = out;
        if poly.is_empty() {
            return Vec::new();
        }
    }
    let mut pts = Vec::new();
    for i in 0..poly.len() {
        let a = Vec2::new(poly[i].x, poly[i].z);
        let b = Vec2::new(poly[(i + 1) % poly.len()].x, poly[(i + 1) % poly.len()].z);
        let n = ((b - a).length() / 0.05).ceil().max(1.0) as usize;
        for k in 0..n {
            pts.push(a + (b - a) * (k as f32 / n as f32));
        }
    }
    pts
}

/// Distance of `p` outside the union of the shapes (≤ 0 = covered).
fn outside(shapes: &[LocalShape], p: Vec2) -> f32 {
    shapes
        .iter()
        .map(|s| match *s {
            LocalShape::Circle { x, z, r } => p.distance(Vec2::new(x, z)) - r,
            LocalShape::Box { x, z, hx, hz } => {
                let d = (p - Vec2::new(x, z)).abs() - Vec2::new(hx, hz);
                d.max(Vec2::ZERO).length() + d.x.max(d.y).min(0.0)
            }
        })
        .fold(f32::INFINITY, f32::min)
}

fn aabb(shapes: &[LocalShape]) -> (Vec2, Vec2) {
    let mut lo = Vec2::splat(f32::MAX);
    let mut hi = Vec2::splat(f32::MIN);
    for s in shapes {
        let (a, b) = match *s {
            LocalShape::Circle { x, z, r } => (Vec2::new(x - r, z - r), Vec2::new(x + r, z + r)),
            LocalShape::Box { x, z, hx, hz } => {
                (Vec2::new(x - hx, z - hz), Vec2::new(x + hx, z + hz))
            }
        };
        lo = lo.min(a);
        hi = hi.max(b);
    }
    (lo, hi)
}

// LAYOUT-018
#[test]
fn layout_018_footprints_match_the_meshes() {
    // (model, lower slab height): `tree_round` is measured on the trunk (0.4–1.4 m, spec
    // table); its root flare is walkable over by design ("the feet may overlap the flat
    // roots"). Not checked: `enclosure_sign` (panel between the posts, Q-086 open),
    // `tree_grove` (kept at the trunk, dense areas only), `bridge_wood` (rails only — the
    // deck is walkable), `tree_eucalyptus` (not placed yet).
    let models = [
        ("info_board", 0.05),
        ("map_board", 0.05),
        ("food_box", 0.05),
        ("food_box_stack", 0.05),
        ("tree_round", 0.4),
        ("bush", 0.05),
        ("rock", 0.05),
        ("bamboo", 0.05),
        ("road_block", 0.05),
        ("repair_sign", 0.05),
        ("zookeeper_cart", 0.05),
        ("traffic_cone", 0.05),
        ("fallen_tree", 0.05),
        ("gate_zoo_closed", 0.05),
    ];
    let mut problems = Vec::new();
    for (name, lo) in models {
        let path = root().join(format!("assets/models/props/{name}.glb"));
        let Ok(bytes) = std::fs::read(&path) else {
            problems.push(format!("{name}: no .glb"));
            continue;
        };
        let m = Model::from_glb(&bytes).unwrap();
        let shapes = footprint(name);
        assert!(!shapes.is_empty(), "{name}: no footprint");
        let pos = &m.mesh.positions;
        let mut pts = Vec::new();
        for t in m.mesh.indices.chunks(3) {
            let tri = [0, 1, 2].map(|k| Vec3::from(pos[t[k] as usize]));
            pts.extend(slab_outline(tri, lo, 1.4));
        }
        assert!(!pts.is_empty(), "{name}: empty cross-section");
        let worst = pts
            .iter()
            .map(|p| outside(shapes, *p))
            .fold(f32::MIN, f32::max);
        let (flo, fhi) = aabb(shapes);
        let (mut plo, mut phi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
        for p in &pts {
            plo = plo.min(*p);
            phi = phi.max(*p);
        }
        let beyond = (plo - flo).max(fhi - phi).max_element();
        let uncovered = (flo - plo).max(phi - fhi).max_element();
        println!(
            "{name}: extents uncovered {uncovered:.3} m, beyond {beyond:.3} m, worst point \
             {worst:.3} m (mesh {plo:.2}..{phi:.2})"
        );
        if uncovered > 0.02 {
            problems.push(format!(
                "{name}: mesh extents {uncovered:.3} m outside the footprint"
            ));
        }
        if worst > 0.1 {
            problems.push(format!(
                "{name}: mesh point {worst:.3} m outside the footprint"
            ));
        }
        if beyond > 0.1 {
            problems.push(format!("{name}: footprint {beyond:.3} m beyond the mesh"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}
