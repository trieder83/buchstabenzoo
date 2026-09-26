//! TECH-WATER asset tests on the exported `kit_water` tiles: the analytic waterline of the
//! water field bake matches the meshes (WATER-003) and river tiles carry no baked streaks
//! or foam (WATER-010, Q-067).

use std::path::PathBuf;

use glam::{Vec2, Vec3};
use zoo_assets::Model;
use zoo_core::water::{rotate_cw, TileShape, WaterCell};

const TILES: [&str; 7] = [
    "water_river_straight",
    "water_river_bank",
    "water_river_curve",
    "water_river_inner",
    "water_pond",
    "water_pond_edge",
    "water_pond_corner",
];
const SOIL: u32 = 2;
const WATER_RIVER: u32 = 176;
const WATER_RIVER_LIGHT: u32 = 177;
const WATER_FOAM: u32 = 178;
const WATER_POND: u32 = 179;

fn load(name: &str) -> Model {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/models/props")
        .join(format!("{name}.glb"));
    Model::from_glb(&std::fs::read(&p).unwrap()).unwrap()
}

/// Triangles as (vertices, palette cell).
fn triangles(m: &Model) -> Vec<([Vec3; 3], u32)> {
    let mesh = &m.mesh;
    mesh.indices
        .chunks(3)
        .map(|t| {
            let v = [0, 1, 2].map(|k| Vec3::from(mesh.positions[t[k] as usize]));
            let uv = t
                .iter()
                .map(|&i| Vec2::from(mesh.uvs[i as usize]))
                .fold(Vec2::ZERO, |a, b| a + b)
                / 3.0;
            let cell = (uv.y * 16.0).floor() as u32 * 16 + (uv.x * 16.0).floor() as u32;
            (v, cell)
        })
        .collect()
}

/// glTF tile vertex → canonical level offset (x east, y north; glTF −Z = north).
fn canon(v: Vec3) -> Vec2 {
    Vec2::new(v.x, -v.z)
}

fn inside(p: Vec2, t: &[Vec3; 3]) -> bool {
    let [a, b, c] = t.map(canon);
    let s = |p1: Vec2, p2: Vec2| (p - p2).perp_dot(p1 - p2);
    let (d1, d2, d3) = (s(a, b), s(b, c), s(c, a));
    let neg = d1 < -1e-7 || d2 < -1e-7 || d3 < -1e-7;
    let pos = d1 > 1e-7 || d2 > 1e-7 || d3 > 1e-7;
    !(neg && pos)
}

// WATER-003: the analytic shore used by the bake is 0 (± 1 cm) at every waterline vertex
// (foot of the bank slope) for all four quarter turns, positive over open water and
// negative over the bank.
#[test]
fn water_003_analytic_shore_matches_the_tiles() {
    for name in TILES {
        let (shape, _) = TileShape::of_model(name).unwrap();
        let m = load(name);
        let tris = triangles(&m);
        let slope_feet: Vec<Vec2> = tris
            .iter()
            .filter(|(v, cell)| {
                let n = (v[1] - v[0]).cross(v[2] - v[0]).normalize();
                *cell == SOIL && n.y > 0.5
            })
            .flat_map(|(v, _)| v.iter().filter(|p| p.y < 0.002).map(|p| canon(*p)))
            .collect();
        if shape != TileShape::Full {
            assert!(!slope_feet.is_empty(), "{name}: bank slope");
        }
        for turns in 0..4 {
            let cell = WaterCell {
                cell: glam::IVec2::ZERO,
                river: name.contains("river"),
                shape,
                turns,
            };
            let rot = |mut q: Vec2| {
                for _ in 0..turns {
                    q = rotate_cw(q);
                }
                q + Vec2::splat(0.5) // cell (0, 0) centre
            };
            for f in &slope_feet {
                let d = cell.waterline_distance(rot(*f)).unwrap();
                assert!(d < 0.01, "{name} k={turns}: foot {f:?} is {d} m off");
            }
        }
        // open water above water faces only, land above land faces only
        let water: Vec<&[Vec3; 3]> = tris
            .iter()
            .filter(|(_, c)| *c == WATER_RIVER || *c == WATER_POND)
            .map(|(v, _)| v)
            .collect();
        let land: Vec<&[Vec3; 3]> = tris
            .iter()
            .filter(|(v, c)| *c != WATER_RIVER && *c != WATER_POND && v.iter().any(|p| p.y > 0.001))
            .map(|(v, _)| v)
            .collect();
        for iz in 0..20 {
            for ix in 0..20 {
                let q = Vec2::new(-0.475 + ix as f32 * 0.05, -0.475 + iz as f32 * 0.05);
                let d = shape.waterline_distance(q).unwrap_or(1.0);
                if d < 0.02 {
                    continue;
                }
                let over_land = land.iter().any(|t| inside(q, t));
                if shape.in_water(q) {
                    assert!(water.iter().any(|t| inside(q, t)), "{name}: water at {q:?}");
                    assert!(!over_land, "{name}: bank over open water at {q:?}");
                } else {
                    assert!(over_land, "{name}: land expected at {q:?}");
                }
            }
        }
    }
}

// WATER-010 (Q-067): river tiles have no streak (177) or foam (178) faces.
#[test]
fn water_010_river_tiles_have_no_baked_streaks() {
    for name in TILES.iter().filter(|n| n.contains("river")) {
        let m = load(name);
        for (_, cell) in triangles(&m) {
            assert!(
                cell != WATER_RIVER_LIGHT && cell != WATER_FOAM,
                "{name}: face in cell {cell}"
            );
        }
    }
}
