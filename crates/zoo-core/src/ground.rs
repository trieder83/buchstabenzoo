//! Ground height (GAME-PLAYER rules 8–9, PLAY-035): the top of the visible walkable surface
//! at a level point, so characters stand on tiles, paths, decks and flat dressing instead of
//! being drawn at y = 0 (user report 2026-09-27: feet sank into the garden path, the bridge
//! deck and the jetty).
//!
//! The height comes from what the scene draws ([`crate::scene::LevelScene`]): the ground
//! tile of every cell (`kit_ground.py` tops), the bridge deck curve and the jetty deck
//! (`kit_water.py`), and the flat walkable dressing boxes the scene registers as
//! [`GroundPatch`]es (mulch, flat rocks, mud, petals, trampoline, rugs, …). Everything a
//! character must not stand on is solid instead (cells, prop colliders).
//!
//! Heights are cached per cell (tile top plus the indices of the features that overlap the
//! cell); features are sampled exactly (the bridge curve is evaluated along its axis).

use glam::{IVec2, Vec2};

use crate::coords::world_to_level;
use crate::level::{cell_center, cell_of, Rect};
use crate::scene::{LevelScene, Placement};
use crate::water::WaterCell;

/// Top of grass, sand and ground slab tiles (`kit_ground.py` `SLAB_TOP`).
pub const GRASS_TOP_M: f32 = 0.05;
/// Walk height on path tiles: between the bed (0.065) and the paving stones (≤ 0.09).
pub const PATH_TOP_M: f32 = 0.075;
/// Walk height on plaza tiles (slabs 0.05 … 0.075).
pub const PLAZA_TOP_M: f32 = 0.065;
/// Water surface (water tiles, pools): swimmers sink relative to it.
pub const WATER_TOP_M: f32 = 0.0;
/// Jetty deck top (`kit_water.py` `jetty_wood` `TOP`).
pub const JETTY_TOP_M: f32 = 0.2;
/// Jetty deck extent from the model origin (level x: water end −X; half width in z).
pub const JETTY_DECK_X: (f32, f32) = (-2.3, 1.5);
pub const JETTY_DECK_HALF_W: f32 = 0.8;
/// Bridge deck: half length along level x, half width, rise (`kit_water.py` `BR_L`, `BR_W`,
/// `RISE`).
pub const BRIDGE_HALF_L: f32 = 1.7;
pub const BRIDGE_HALF_W: f32 = 1.25;
pub const BRIDGE_RISE: f32 = 0.43;
/// Height steps up to this are walked up and down (higher edges are walls, GAME-PLAYER 8).
pub const MAX_STEP_M: f32 = 0.25;
/// How fast a drawn height follows the ground (1/s, exponential), GAME-PLAYER 8 "smooth".
pub const FOLLOW_RATE: f32 = 18.0;
/// Height changes up to this per update are taken at once (slopes).
pub const SNAP_M: f32 = 0.03;

/// Deck top of the arched bridge at `dx` metres from its centre along the deck.
pub fn bridge_deck_y(dx: f32) -> f32 {
    let t = (dx / BRIDGE_HALF_L).clamp(-1.0, 1.0);
    0.07 + BRIDGE_RISE * (1.0 - t * t)
}

/// A flat walkable surface above the tile (level coordinates, axis-aligned).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundPatch {
    pub min: Vec2,
    pub max: Vec2,
    /// Top of the surface (m above y = 0).
    pub top: f32,
}

impl GroundPatch {
    /// Patch of a box centred at `c` with footprint `size` (level x, z).
    pub fn centered(c: Vec2, size: Vec2, top: f32) -> Self {
        Self {
            min: c - size / 2.0,
            max: c + size / 2.0,
            top,
        }
    }

    fn contains(&self, p: Vec2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }
}

/// A raised surface feature (deck or patch).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Feature {
    Patch(GroundPatch),
    /// Arched bridge deck centred at `c`, running along level x.
    Bridge {
        c: Vec2,
    },
}

impl Feature {
    fn bounds(&self) -> (Vec2, Vec2) {
        match self {
            Feature::Patch(p) => (p.min, p.max),
            Feature::Bridge { c } => {
                let h = Vec2::new(BRIDGE_HALF_L, BRIDGE_HALF_W);
                (*c - h, *c + h)
            }
        }
    }

    fn height(&self, p: Vec2) -> Option<f32> {
        match self {
            Feature::Patch(g) => g.contains(p).then_some(g.top),
            Feature::Bridge { c } => {
                let d = p - *c;
                (d.x.abs() <= BRIDGE_HALF_L && d.y.abs() <= BRIDGE_HALF_W)
                    .then(|| bridge_deck_y(d.x))
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Cell {
    base: f32,
    water: Option<WaterCell>,
    features: Vec<u32>,
}

/// Ground heights of a level, cached per cell.
#[derive(Debug, Clone, Default)]
pub struct GroundMap {
    bounds: Rect,
    cells: Vec<Cell>,
    features: Vec<Feature>,
    stills: Vec<crate::water::StillWater>,
}

/// Walk height of a ground tile model, if it is one.
pub fn tile_top(model: &str) -> Option<f32> {
    match model {
        "grass_tile" | "sand_tile" => Some(GRASS_TOP_M),
        "plaza_tile" => Some(PLAZA_TOP_M),
        m if m.starts_with("path_tile") => Some(PATH_TOP_M),
        m if m.starts_with("water_") => Some(WATER_TOP_M),
        _ => None,
    }
}

impl GroundMap {
    /// Builds the map from the assembled scene of a level.
    pub fn build(scene: &LevelScene, bounds: Rect) -> Self {
        let n = (bounds.w.max(0) * bounds.d.max(0)) as usize;
        let mut map = GroundMap {
            bounds,
            cells: vec![
                Cell {
                    base: GRASS_TOP_M,
                    ..Cell::default()
                };
                n
            ],
            features: Vec::new(),
            stills: scene.water.stills.clone(),
        };
        for p in &scene.placements {
            map.placement(p);
        }
        for w in &scene.water.cells {
            if let Some(k) = map.index(w.cell) {
                map.cells[k].water = Some(*w);
            }
        }
        for g in &scene.ground_patches {
            map.add(Feature::Patch(*g));
        }
        map
    }

    fn placement(&mut self, p: &Placement) {
        let at = world_to_level(p.pos);
        if let Some(top) = tile_top(p.model) {
            // ground tiles sit at the cell centre (fountain water tiles are lifted: skip)
            if p.pos.y.abs() < 1e-3 && (at - cell_center(cell_of(at))).length() < 1e-3 {
                if let Some(k) = self.index(cell_of(at)) {
                    self.cells[k].base = top;
                }
            }
            return;
        }
        match p.model {
            "bridge_wood" => self.add(Feature::Bridge { c: at }),
            "jetty_wood" => self.add(Feature::Patch(GroundPatch {
                min: at + Vec2::new(JETTY_DECK_X.0, -JETTY_DECK_HALF_W),
                max: at + Vec2::new(JETTY_DECK_X.1, JETTY_DECK_HALF_W),
                top: JETTY_TOP_M,
            })),
            _ => {}
        }
    }

    fn add(&mut self, f: Feature) {
        let i = self.features.len() as u32;
        let (lo, hi) = f.bounds();
        let (a, b) = (cell_of(lo), cell_of(hi));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                if let Some(k) = self.index(IVec2::new(x, z)) {
                    self.cells[k].features.push(i);
                }
            }
        }
        self.features.push(f);
    }

    fn index(&self, c: IVec2) -> Option<usize> {
        let b = self.bounds;
        b.contains(c)
            .then(|| ((c.y - b.z) * b.w + (c.x - b.x)) as usize)
    }

    /// Height (m) of the visible walkable surface at level point `p`: the ground tile top,
    /// raised by decks and flat dressing; open water is at [`WATER_TOP_M`].
    pub fn height(&self, p: Vec2) -> f32 {
        let Some(k) = self.index(cell_of(p)) else {
            return GRASS_TOP_M;
        };
        let cell = &self.cells[k];
        let mut h = match cell.water {
            Some(w) if w.in_water(p) => WATER_TOP_M,
            Some(_) => GRASS_TOP_M, // the bank of a water tile
            None => cell.base,
        };
        if self.stills.iter().any(|s| s.shore(p) > 0.0) {
            return crate::scene::FOUNTAIN_WATER_Y;
        }
        for &i in &cell.features {
            if let Some(t) = self.features[i as usize].height(p) {
                h = h.max(t);
            }
        }
        h
    }

    /// Same as [`GroundMap::height`] with level `(x, z)`.
    pub fn ground_height(&self, x: f32, z: f32) -> f32 {
        self.height(Vec2::new(x, z))
    }
}

/// Moves a drawn height `y` towards the ground `target` smoothly (no pops, GAME-PLAYER 8).
/// Differences larger than a walkable step (teleports, respawns) snap; `dt <= 0` snaps too.
/// Small changes (continuous slopes such as the bridge deck) are followed at once, so the
/// feet never lag behind a slope; only edges (the jetty step, a heap) are eased.
pub fn follow(y: f32, target: f32, dt: f32) -> f32 {
    let d = (target - y).abs();
    if dt <= 0.0 || d <= SNAP_M || d > 2.0 * MAX_STEP_M {
        return target;
    }
    y + (target - y) * (1.0 - (-FOLLOW_RATE * dt).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    // PLAY-035 (unit): the bridge curve of kit_water.py
    #[test]
    fn play_035_bridge_curve() {
        assert!((bridge_deck_y(0.0) - 0.5).abs() < 1e-6);
        assert!((bridge_deck_y(1.7) - 0.07).abs() < 1e-6);
        assert!((bridge_deck_y(-1.7) - 0.07).abs() < 1e-6);
        let x: f32 = 0.85;
        let want = 0.07 + 0.43 * (1.0 - (x / 1.7).powi(2));
        assert!((bridge_deck_y(x) - want).abs() < 1e-6);
        // beyond the deck ends it stays at the end height (the ramps meet the path)
        assert!((bridge_deck_y(2.5) - 0.07).abs() < 1e-6);
    }

    fn scene_with(models: &[(&'static str, Vec2)]) -> LevelScene {
        let mut s = LevelScene::default();
        for x in -5..5 {
            for z in -5..5 {
                let c = cell_center(IVec2::new(x, z));
                s.placements.push(Placement::new(
                    "grass_tile",
                    crate::coords::level_to_world(c),
                    0.0,
                ));
            }
        }
        for (m, p) in models {
            s.placements
                .push(Placement::new(m, crate::coords::level_to_world(*p), 0.0));
        }
        s
    }

    // PLAY-035 (unit): ground_height on the bridge samples the arched deck
    #[test]
    fn play_035_ground_height_bridge() {
        let s = scene_with(&[("bridge_wood", Vec2::new(0.5, 0.5))]);
        let g = GroundMap::build(&s, Rect::new(-5, -5, 10, 10));
        assert!((g.ground_height(0.5, 0.5) - 0.5).abs() < 1e-5);
        for k in 0..=34 {
            let dx = -1.7 + k as f32 * 0.1;
            let h = g.ground_height(0.5 + dx, 0.0);
            assert!((h - bridge_deck_y(dx)).abs() < 1e-5, "dx {dx}: {h}");
        }
        // off the deck: grass
        assert!((g.ground_height(3.5, 0.5) - GRASS_TOP_M).abs() < 1e-6);
    }

    // PLAY-035 (unit): the jetty deck is at 0.2 m, its water end overhangs 0.8 m (−X)
    #[test]
    fn play_035_ground_height_jetty() {
        let s = scene_with(&[("jetty_wood", Vec2::new(0.5, 0.0))]);
        let g = GroundMap::build(&s, Rect::new(-5, -5, 10, 10));
        for (x, z) in [(0.5, 0.0), (-1.7, 0.5), (1.9, -0.7), (-0.5, -0.5)] {
            assert!(
                (g.ground_height(x, z) - JETTY_TOP_M).abs() < 1e-6,
                "({x}, {z})"
            );
        }
        assert!((g.ground_height(2.2, 0.0) - GRASS_TOP_M).abs() < 1e-6);
        assert!((g.ground_height(0.5, 0.95) - GRASS_TOP_M).abs() < 1e-6);
    }

    #[test]
    fn follow_is_smooth_and_snaps_teleports() {
        let mut y = 0.05;
        let mut last = y;
        for _ in 0..60 {
            y = follow(y, 0.2, 1.0 / 60.0);
            assert!((y - last).abs() <= MAX_STEP_M);
            assert!(y <= 0.2 + 1e-6);
            last = y;
        }
        assert!((y - 0.2).abs() < 1e-3);
        assert_eq!(follow(0.05, 3.0, 1.0 / 60.0), 3.0);
        assert_eq!(follow(0.05, 0.2, 0.0), 0.2);
    }
}
