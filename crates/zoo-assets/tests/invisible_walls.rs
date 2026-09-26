//! LAYOUT-019 / LAYOUT-L1-025 (no invisible walls) and LAYOUT-017 (billboards are solid) on
//! the level-1 scene with the exported meshes (QA F7/F8/F13, Q-087).

use std::collections::HashMap;
use std::path::PathBuf;

use glam::{IVec2, Quat, Vec2, Vec3};
use zoo_assets::Model;
use zoo_core::coords::world_to_level;
use zoo_core::level::{cell_center, cell_of, CellKind, ElementType, Level, LevelData};
use zoo_core::scene::{model_placeholder, LevelScene};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const LO: f32 = 0.05;
const HI: f32 = 1.4;

/// Outline points (model space x, z) of a mesh's cross-section between `LO` and `HI`.
fn cross_section(m: &Model) -> Vec<Vec2> {
    let pos = &m.mesh.positions;
    let mut out = Vec::new();
    for t in m.mesh.indices.chunks(3) {
        let mut poly: Vec<Vec3> = [0, 1, 2]
            .iter()
            .map(|&k| Vec3::from(pos[t[k] as usize]))
            .collect();
        for (plane, above) in [(LO, true), (HI, false)] {
            let inside = |p: &Vec3| if above { p.y >= plane } else { p.y <= plane };
            let mut next = Vec::new();
            for i in 0..poly.len() {
                let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                if inside(&a) {
                    next.push(a);
                }
                if inside(&a) != inside(&b) {
                    next.push(a + (b - a) * ((plane - a.y) / (b.y - a.y)));
                }
            }
            poly = next;
            if poly.is_empty() {
                break;
            }
        }
        for i in 0..poly.len() {
            let a = Vec2::new(poly[i].x, poly[i].z);
            let b = Vec2::new(poly[(i + 1) % poly.len()].x, poly[(i + 1) % poly.len()].z);
            let n = ((b - a).length() / 0.05).ceil().max(1.0) as usize;
            for k in 0..n {
                out.push(a + (b - a) * (k as f32 / n as f32));
            }
        }
    }
    out
}

/// Models that are ground or decoration, not a wall (tiles, water, tufts, animals on water).
fn is_ground(model: &str) -> bool {
    model.ends_with("_tile")
        || model.starts_with("path_tile")
        || model.starts_with("water_")
        || matches!(
            model,
            "path_edge" | "grass_tuft" | "lily_pad" | "flower_bed" | "reed" | "duck" | "frog"
        )
}

/// Level-space points of visible geometry (0.05–1.4 m) of the scene, hashed per cell.
struct Geometry {
    cells: HashMap<(i32, i32), Vec<Vec2>>,
}

impl Geometry {
    fn add(&mut self, p: Vec2) {
        let c = cell_of(p);
        self.cells.entry((c.x, c.y)).or_default().push(p);
    }

    fn near(&self, p: Vec2, r: f32, filter: impl Fn(Vec2) -> bool) -> bool {
        let c = cell_of(p);
        (-1..=1).any(|dz| {
            (-1..=1).any(|dx| {
                self.cells
                    .get(&(c.x + dx, c.y + dz))
                    .is_some_and(|v| v.iter().any(|q| q.distance(p) <= r && filter(*q)))
            })
        })
    }
}

fn level1() -> LevelData {
    let s = std::fs::read_to_string(root().join("assets/levels/level-1.toml")).unwrap();
    LevelData::from_toml_str(&s).unwrap()
}

fn geometry(data: &LevelData) -> (Geometry, LevelScene, HashMap<&'static str, Vec<Vec2>>) {
    let scene = LevelScene::build(data);
    let mut sections: HashMap<&'static str, Vec<Vec2>> = HashMap::new();
    let mut g = Geometry {
        cells: HashMap::new(),
    };
    let mut boxes = scene.boxes.clone();
    for p in &scene.placements {
        if is_ground(p.model) {
            continue;
        }
        if !sections.contains_key(p.model) {
            let path = root().join(format!("assets/models/props/{}.glb", p.model));
            let sec = std::fs::read(&path)
                .ok()
                .and_then(|b| Model::from_glb(&b).ok())
                .map(|m| cross_section(&m));
            match sec {
                Some(s) => {
                    sections.insert(p.model, s);
                }
                None => {
                    // missing model: its fallback geometry or its placeholder box
                    if let Some(f) = scene.fallbacks.iter().find(|f| f.model == p.model) {
                        boxes.extend(f.boxes.iter().cloned());
                    } else {
                        let (off, size, color) = model_placeholder(p.model);
                        boxes.push(zoo_core::scene::BoxPlacement {
                            pos: p.pos + Quat::from_rotation_y(p.yaw) * off,
                            size: size * p.scale,
                            yaw: p.yaw,
                            color,
                            fadeable: false,
                            source: p.model.to_owned(),
                        });
                    }
                    sections.insert(p.model, Vec::new());
                }
            }
        }
        let rot = Quat::from_rotation_y(p.yaw);
        for q in &sections[p.model] {
            let w = p.pos + rot * (Vec3::new(q.x, 0.0, q.y) * p.scale);
            g.add(world_to_level(w));
        }
    }
    for b in &boxes {
        if b.pos.y > HI || b.pos.y + b.size.y < LO {
            continue;
        }
        let rot = Quat::from_rotation_y(b.yaw);
        let (hx, hz) = (b.size.x / 2.0, b.size.z / 2.0);
        let corners = [(-hx, -hz), (hx, -hz), (hx, hz), (-hx, hz)];
        for i in 0..4 {
            let a = Vec2::from(corners[i]);
            let c = Vec2::from(corners[(i + 1) % 4]);
            let n = ((c - a).length() / 0.05).ceil().max(1.0) as usize;
            for k in 0..n {
                let l = a + (c - a) * (k as f32 / n as f32);
                g.add(world_to_level(b.pos + rot * Vec3::new(l.x, 0.0, l.y)));
            }
        }
    }
    (g, scene, sections)
}

// LAYOUT-019, LAYOUT-L1-025 (invisible walls): every edge between a grid-solid cell and a
// walkable cell has visible geometry (0.05–1.4 m) within 0.3 m on the solid side. Water is
// visible by its surface (not checked). Accepted exceptions of GAME-LEVEL-1 "Collision and
// billboards": the back of `map_board` (its 0.36 m deep board stands in a 1 m cell) and the
// fallen tree `barrier_ne_tree` (irregular crown in a 2 m band; removed with level 2).
#[test]
fn layout_019_l1_025_no_invisible_walls() {
    let data = level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let (geo, _, _) = geometry(&data);
    let water = |c: IVec2| {
        data.elements.iter().any(|e| {
            e.ty == ElementType::Landmark
                && matches!(e.kind.as_deref(), Some("pond" | "river"))
                && e.rect.contains(c)
        })
    };
    let map_board = |c: IVec2| data.element("map_board").unwrap().rect.contains(c);
    let fallen_tree = |c: IVec2| data.element("barrier_ne_tree").unwrap().rect.contains(c);
    let mut bad = Vec::new();
    for c in data.level.bounds.cells() {
        if grid.kind(c) != CellKind::Solid || water(c) || fallen_tree(c) {
            continue;
        }
        for d in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
            let w = c + d;
            if !grid.is_walkable(w, false) {
                continue;
            }
            // points on the shared edge; the solid side is -d
            let mid = cell_center(c) + d.as_vec2() * 0.5;
            let along = Vec2::new(-d.y as f32, d.x as f32);
            let reach = if map_board(c) { 0.8 } else { 0.3 };
            for t in [-0.3, 0.0, 0.3] {
                let p = mid + along * t;
                let ok = geo.near(p, reach + 0.05, |q| (q - p).dot(-d.as_vec2()) >= -0.05);
                if !ok {
                    bad.push(format!("{c}→{w} at {p}"));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} invisible wall points: {:#?}",
        bad.len(),
        &bad[..bad.len().min(40)]
    );
}

// LAYOUT-017, LAYOUT-L1-025 (billboards): the player circle can reach no position that
// overlaps the cross-section below 1.4 m of an info board or the map board — every point of
// it lies in a solid cell or inside a collider. The enclosure signs are the known exception
// until Q-086 (gate arch) is decided: their panel between the posts is not solid.
#[test]
fn layout_017_l1_025_billboards_are_solid() {
    let data = level1();
    let level = Level::new(data.clone());
    let (_, scene, sections) = geometry(&data);
    let mut bad = Vec::new();
    let mut sign_points_open = 0;
    for p in &scene.placements {
        if !matches!(p.model, "info_board" | "map_board" | "enclosure_sign") {
            continue;
        }
        let rot = Quat::from_rotation_y(p.yaw);
        for q in &sections[p.model] {
            let l = world_to_level(p.pos + rot * Vec3::new(q.x, 0.0, q.y));
            let solid = !level.grid().is_walkable(cell_of(l), false);
            let inside = level
                .colliders()
                .shapes()
                .iter()
                .any(|s| s.push_out(l, 1e-3) != Vec2::ZERO);
            if !solid && !inside {
                if p.model == "enclosure_sign" {
                    sign_points_open += 1;
                } else {
                    bad.push(format!("{} at {l}", p.model));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "reachable billboard parts: {:#?}",
        &bad[..bad.len().min(20)]
    );
    // Q-086 still open: document that the sign panels are the remaining violation
    println!("enclosure_sign: {sign_points_open} open cross-section points (Q-086)");
}
