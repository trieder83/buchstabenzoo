//! GAME-PLAYER "Standing on surfaces" (rules 8–9): PLAY-035.

mod common;

use glam::Vec2;
use std::collections::BTreeMap;
use zoo_core::coords::world_to_level;
use zoo_core::level::{cell_center, Grid, Level};
use zoo_core::scene::{BoxPlacement, LevelScene};

/// Visible top (m) of a ground model at level point `p`, if the model covers it.
fn model_top(model: &str, origin: Vec2, p: Vec2) -> Option<(f32, f32)> {
    let d = p - origin;
    match model {
        "grass_tile" | "sand_tile" if d.abs().max_element() <= 0.5 => Some((0.05, 0.05)),
        m if m.starts_with("path_tile") && d.abs().max_element() <= 0.5 => Some((0.065, 0.09)),
        "plaza_tile" if d.abs().max_element() <= 0.5 => Some((0.05, 0.075)),
        "bridge_wood" if d.x.abs() <= 1.7 && d.y.abs() <= 1.2 => {
            let y = 0.07 + 0.43 * (1.0 - (d.x / 1.7).powi(2));
            Some((y, y))
        }
        "jetty_wood" if (-2.3..=1.5).contains(&d.x) && d.y.abs() <= 0.8 => Some((0.2, 0.2)),
        _ => None,
    }
}

fn box_contains(b: &BoxPlacement, p: Vec2) -> bool {
    let c = world_to_level(b.pos);
    // level yaw: world rotation about +Y; level z = -world z
    // model x / z axes in level coordinates (as `collision::Shape::place`)
    let (s, co) = b.yaw.sin_cos();
    let d = p - c;
    let local = Vec2::new(d.dot(Vec2::new(co, s)), d.dot(Vec2::new(s, -co)));
    local.x.abs() <= b.size.x / 2.0 && local.y.abs() <= b.size.z / 2.0
}

pub struct Found {
    lo: f32,
    hi: f32,
    source: String,
}

/// Top of the visible surface at `p` from the scene geometry: the ground model under it,
/// raised by every box that rests on that surface (bottom at most 3 cm above it), stacked.
fn visible_top(scene: &LevelScene, p: Vec2) -> Option<Found> {
    let mut best: Option<Found> = None;
    for pl in &scene.placements {
        if (world_to_level(pl.pos) - p).length_squared() > 9.0 {
            continue;
        }
        if let Some((lo, hi)) = model_top(pl.model, world_to_level(pl.pos), p) {
            let (lo, hi) = (lo + pl.pos.y, hi + pl.pos.y);
            if best.as_ref().is_none_or(|b| hi > b.hi) {
                best = Some(Found {
                    lo,
                    hi,
                    source: pl.model.to_string(),
                });
            }
        }
    }
    let mut best = best?;
    let boxes: Vec<&BoxPlacement> = scene
        .boxes
        .iter()
        .chain(scene.fallbacks.iter().flat_map(|f| f.boxes.iter()))
        .filter(|b| box_contains(b, p))
        .collect();
    loop {
        let surface = best.hi;
        let raise = boxes
            .iter()
            .filter(|b| b.pos.y <= surface + 0.03 && b.pos.y + b.size.y > best.hi + 1e-4)
            .max_by(|a, b| (a.pos.y + a.size.y).total_cmp(&(b.pos.y + b.size.y)));
        match raise {
            Some(b) => {
                let top = b.pos.y + b.size.y;
                best = Found {
                    lo: top,
                    hi: top,
                    source: format!("box {}", b.source),
                };
            }
            None => return Some(best),
        }
    }
}

fn walkable_centres(data: &zoo_core::LevelData) -> (Level, Vec<Vec2>) {
    let level = Level::new(data.clone());
    let g: &Grid = level.grid();
    let pts = (0..g.len())
        .map(|k| g.cell_at(k))
        .filter(|&c| g.is_walkable(c, true) && !g.is_prop_blocked(c))
        .map(cell_center)
        .collect();
    (level, pts)
}

/// Walkable cell centres the player can stand on, with what is visible there.
fn check_zoo(data: zoo_core::LevelData) -> Vec<String> {
    let scene = LevelScene::build(&data);
    let (level, pts) = walkable_centres(&data);
    let mut bad: BTreeMap<String, (usize, f32, Vec2)> = BTreeMap::new();
    for &p in &pts {
        let g = level.ground_height(p);
        let Some(f) = visible_top(&scene, p) else {
            panic!("no ground drawn at walkable ({:.1}, {:.1})", p.x, p.y);
        };
        let err = (f.lo - g).max(g - f.hi);
        if err > 0.02 {
            let e = bad.entry(f.source.clone()).or_insert((0, 0.0, p));
            e.0 += 1;
            if err > e.1 {
                e.1 = err;
                e.2 = p;
            }
        }
    }
    // walkable neighbours differ by at most a walkable step (higher edges are walls)
    let grid = level.grid();
    for &p in &pts {
        for d in [Vec2::X, Vec2::Y] {
            let q = p + d;
            if pts.contains(&q) {
                // largest jump between samples 5 cm apart (a slope is fine, an edge is a step)
                let step = (0..20)
                    .map(|k| {
                        let a = p + d * (k as f32 * 0.05);
                        let b = p + d * ((k + 1) as f32 * 0.05);
                        (level.ground_height(a) - level.ground_height(b)).abs()
                    })
                    .fold(0.0f32, f32::max);
                assert!(
                    step <= zoo_core::ground::MAX_STEP_M,
                    "step {step:.2} m between ({:.1}, {:.1}) and ({:.1}, {:.1})",
                    p.x,
                    p.y,
                    q.x,
                    q.y
                );
            }
        }
    }
    let _ = grid;
    bad.into_iter()
        .map(|(s, (n, err, p))| {
            format!(
                "{s}: {n} cells, off by up to {err:.3} m at ({:.1}, {:.1})",
                p.x, p.y
            )
        })
        .collect()
}

// PLAY-035: ground_height equals the visible surface top at every walkable cell centre of
// the joined zoo (day levels + night_1), ± 2 cm (user report 2026-09-27).
#[test]
fn play_035_ground_height_matches_visible_surface() {
    let bad = check_zoo(common::zoo_with_night());
    assert!(
        bad.is_empty(),
        "feet not on the surface:\n{}",
        bad.join("\n")
    );
}

// PLAY-035: a player placed on a walkable cell centre has her feet on that surface (± 2 cm).
#[test]
fn play_035_player_feet_on_surface() {
    let data = common::zoo_with_night();
    let scene = LevelScene::build(&data);
    let (_, pts) = walkable_centres(&data);
    let mut g = zoo_core::Game::new(data, 7).expect("game starts");
    for p in pts.iter().step_by(5) {
        g.player.pos = *p;
        g.update(0.0, Vec2::ZERO);
        let f = visible_top(&scene, *p).expect("ground drawn");
        assert!(
            g.player.y >= f.lo - 0.02 && g.player.y <= f.hi + 0.02,
            "feet at {:.3} on {} ({:.3}…{:.3}) at ({:.1}, {:.1})",
            g.player.y,
            f.source,
            f.lo,
            f.hi,
            p.x,
            p.y
        );
    }
}

// PLAY-035: bridge and jetty of level 1 — the deck, not the grass under it.
#[test]
fn play_035_bridge_and_jetty_level_1() {
    let data = common::level1();
    let level = Level::new(data.clone());
    for (kind, want) in [("bridge", 0.5), ("jetty", zoo_core::ground::JETTY_TOP_M)] {
        let e = data
            .elements
            .iter()
            .find(|e| e.kind.as_deref() == Some(kind))
            .expect(kind);
        let c = Vec2::new(
            e.rect.x as f32 + e.rect.w as f32 / 2.0,
            e.rect.z as f32 + e.rect.d as f32 / 2.0,
        );
        let h = level.ground_height(c);
        assert!((h - want).abs() < 0.01, "{kind}: {h}");
    }
}

// PLAY-035 / GAME-PLAYER 9: furniture and beds are solid — no walkable cell centre inside.
#[test]
fn play_035_furniture_is_solid() {
    let data = common::zoo_with_night();
    let level = Level::new(data.clone());
    for p in &data.props {
        if p.model == "rug_round" || p.model == "window_moon" {
            continue;
        }
        assert!(
            level.colliders().overlaps(p.pos(), 0.05),
            "{} is not solid",
            p.id
        );
    }
    for it in data.items.iter().filter(|it| it.kind == "bed") {
        assert!(level.colliders().overlaps(it.pos(), 0.05), "{}", it.id);
    }
}
