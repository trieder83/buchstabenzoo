//! Level assembly (GAME-LAYOUT, GAME-LEVEL-1): turns the layout data into model placements
//! and placeholder boxes in world space. Pure — no GL — so it is unit-tested natively.
//!
//! - Ground: one 1 m tile per cell — path tiles autotiled from the 4 path neighbours
//!   (`tools/blender/props/kit_ground.py`), `plaza_tile` on the plaza, `grass_tile`
//!   elsewhere; water, bridge and jetty are placeholder boxes.
//! - Enclosure fences, hedge and wall bands use the modular pieces and the rules of
//!   GAME-LAYOUT "Modular edges" (`zoo_core::level::{enclosure_fence, band_run}`).
//! - Everything without a model yet (buildings, rocks, trees, barriers, benches, water) is a
//!   coloured placeholder box sized from its rectangle and `height_m` (PROD-POC
//!   "Placeholders").
//!
//! All level → world conversions go through `zoo_core::coords` (Q-056); models are only
//! rotated about +Y, never mirrored.

use crate::coords::{level_to_world, level_to_world_at, quarter_turns_cw_to_yaw};
use crate::level::{band_run, cell_center, enclosure_fence, Element, Grid, Run, RunAxis};
use crate::level::{ElementType, LevelData, Rect};
use glam::{IVec2, Quat, Vec2, Vec3};

/// Placeholder colours (sRGB, flat) per element kind.
pub mod colors {
    pub const WATER: [f32; 3] = [0.36, 0.66, 0.90];
    pub const WOOD: [f32; 3] = [0.72, 0.48, 0.28];
    pub const WOOD_LIGHT: [f32; 3] = [0.79, 0.55, 0.34];
    pub const STONE: [f32; 3] = [0.81, 0.79, 0.77];
    pub const ROCK: [f32; 3] = [0.62, 0.62, 0.66];
    pub const TREE_CROWN: [f32; 3] = [0.40, 0.64, 0.28];
    pub const TREE_TRUNK: [f32; 3] = [0.55, 0.36, 0.22];
    pub const BUILDING: [f32; 3] = [0.93, 0.80, 0.60];
    pub const ROOF: [f32; 3] = [0.80, 0.36, 0.26];
    pub const BARRIER: [f32; 3] = [0.96, 0.56, 0.20];
    pub const HEDGE: [f32; 3] = [0.43, 0.61, 0.27];
    pub const GRASS: [f32; 3] = [0.56, 0.75, 0.34];
    pub const PATH: [f32; 3] = [0.95, 0.89, 0.80];
    pub const DEFAULT: [f32; 3] = [0.85, 0.40, 0.85];
    pub const WATER_LIGHT: [f32; 3] = [0.70, 0.88, 0.98];
    pub const FOAM: [f32; 3] = [0.96, 0.98, 1.0];
    pub const GOLD: [f32; 3] = [0.98, 0.80, 0.25];
    pub const WHITE: [f32; 3] = [0.97, 0.96, 0.93];
    pub const BLACK: [f32; 3] = [0.17, 0.16, 0.18];
    pub const WILLOW: [f32; 3] = [0.55, 0.74, 0.36];
    pub const BLOSSOM: [f32; 3] = [0.97, 0.68, 0.80];
    pub const HORSE: [f32; 3] = [0.85, 0.62, 0.40];
    pub const SAWDUST: [f32; 3] = [0.90, 0.78, 0.52];
    pub const WAGON_BLUE: [f32; 3] = [0.30, 0.52, 0.85];
    pub const ROPE: [f32; 3] = [0.82, 0.70, 0.48];
    pub const DIGGER: [f32; 3] = [0.98, 0.78, 0.15];
    pub const WET_GRASS: [f32; 3] = [0.40, 0.66, 0.36];
    pub const BARK: [f32; 3] = [0.52, 0.36, 0.24];
}

/// Landmark / decoration kinds of levels 2–3 drawn as coloured placeholder boxes (PROD-POC
/// "Placeholders"; the real models come after concept approval).
pub fn placeholder_kind(kind: &str) -> Option<&'static str> {
    const KINDS: [&str; 19] = [
        "fountain",
        "waterfall",
        "stream",
        "mill_hut",
        "willow",
        "pirate_ship",
        "carousel",
        "washing_line",
        "treehouse",
        "giant_tree",
        "blossom_tree",
        "log_pile",
        "play_ball",
        "lookout_tower",
        "zoo_train",
        "slide",
        "swings",
        "stage",
        "deckchairs",
    ];
    KINDS.iter().copied().find(|k| *k == kind)
}

/// The element a perch belongs to (first scenery id of the place that is an element).
pub fn perch_scenery<'a>(
    h: &crate::level::HidingPlaceData,
    data: &'a LevelData,
) -> Option<&'a Element> {
    h.scenery.iter().find_map(|id| data.element(id))
}

/// Where an animal sits at a perch (proposal Q-094), level coordinates: 0.25 m from its spot
/// centre towards the tree / ship it sits in (the player stands at the spot below).
pub fn perch_point(h: &crate::level::HidingPlaceData, data: &LevelData) -> Vec2 {
    let spot = h.spot();
    let Some(e) = perch_scenery(h, data) else {
        return spot;
    };
    let r = e.rect;
    let min = Vec2::new(r.x as f32, r.z as f32);
    let q = spot.clamp(min, min + Vec2::new(r.w as f32, r.d as f32));
    spot + (q - spot).clamp_length_max(0.25)
}

/// A model instance: `pos` is the model origin in world space, `yaw` the rotation about +Y
/// (positive = counter-clockwise seen from above), `scale` a uniform scale (footprints scale
/// with it).
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub model: &'static str,
    pub pos: Vec3,
    pub yaw: f32,
    pub scale: f32,
    /// Level part it belongs to ([`LevelData::parts`]; render chunks, GAME-LAYOUT "Joining
    /// levels").
    pub part: u8,
}

impl Placement {
    pub fn new(model: &'static str, pos: Vec3, yaw: f32) -> Self {
        Self {
            model,
            pos,
            yaw,
            scale: 1.0,
            part: 0,
        }
    }
}

/// Placeholder geometry drawn only when a model is missing (e.g. the tiled rim of
/// `pool_tiled`, GAME-LEVEL-1 "Hippo enclosure pool").
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fallback {
    pub model: &'static str,
    pub boxes: Vec<BoxPlacement>,
    pub placements: Vec<Placement>,
}

/// A placeholder box: `pos` = centre of its bottom face in world space, `size` = extent
/// along its local X/Y/Z before the yaw rotation.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxPlacement {
    pub pos: Vec3,
    pub size: Vec3,
    pub yaw: f32,
    pub color: [f32; 3],
    /// Tall objects that may occlude the player (GAME-PLAYER §2 occluder fade).
    pub fadeable: bool,
    /// Element id (for logging), or the missing model id.
    pub source: String,
    /// Level part it belongs to (render chunks).
    pub part: u8,
}

/// What a decal shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecalImage {
    /// An image file (path relative to `assets/`, e.g. an enclosure sign silhouette).
    Texture(String),
    /// A Fluent text rendered by the host into a `width` × `height` px texture (dark bold
    /// lettering on a cream sign, ART-ENVIRONMENT behaviour 7); re-rendered when the
    /// language changes.
    Text {
        key: &'static str,
        width_px: u32,
        height_px: u32,
    },
}

/// A flat textured quad drawn on top of a model face (sign silhouettes, sign texts).
/// `right` / `up` are half extents in world space; the image's top row is at `center + up`,
/// its left column at `center - right`; the visible side faces `right × up`.
#[derive(Debug, Clone, PartialEq)]
pub struct Decal {
    /// Stable id, e.g. `sign:enc_zebra` or `sign:food_storage`.
    pub id: String,
    pub image: DecalImage,
    pub center: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Decal {
    /// Unit normal of the visible side.
    pub fn normal(&self) -> Vec3 {
        self.right.cross(self.up).normalize_or_zero()
    }

    /// Corners (top-left, top-right, bottom-right, bottom-left) in world space.
    pub fn corners(&self) -> [Vec3; 4] {
        let (c, r, u) = (self.center, self.right, self.up);
        [c - r + u, c + r + u, c + r - u, c - r - u]
    }
}

/// Fluent key of the food storage sign (ART-ENVIRONMENT behaviour 7).
pub const FOOD_STORAGE_SIGN_KEY: &str = "sign-food-storage";

/// Asset path of an enclosure sign silhouette (ART-ENVIRONMENT behaviour 6).
pub fn silhouette_path(animal: &str) -> String {
    format!("textures/signs/silhouette_{animal}.png")
}

/// `enclosure_sign` panel face at yaw 0 (`README_kits_3_6.md`, palette cell 128 `sign_panel`):
/// centre, normal, size ~1.30 × 0.72 m, tilted back 40°.
const SIGN_PANEL_CENTER: Vec3 = Vec3::new(0.29, 1.36, 0.05);
const SIGN_PANEL_NORMAL: Vec3 = Vec3::new(0.0, 0.643, 0.766);
/// Silhouette decal size on the panel (m): 3:2 like the image, inside the 1.30 × 0.72 face.
const SILHOUETTE_SIZE: Vec2 = Vec2::new(1.02, 0.68);
/// Decals float this far in front of their face (plus a polygon offset in the renderer).
pub const DECAL_LIFT_M: f32 = 0.004;

/// Food storage sign board (ART-ENVIRONMENT behaviour 7): wooden board on the south facade
/// above the food boxes; the text decal covers its front minus a wooden frame.
pub const STORAGE_SIGN_BOARD: Vec3 = Vec3::new(3.4, 1.2, 0.08);
pub const STORAGE_SIGN_BOTTOM_M: f32 = 1.75;
const STORAGE_SIGN_FRAME_M: f32 = 0.09;

/// The assembled static scene of a level.
#[derive(Debug, Clone, Default)]
pub struct LevelScene {
    pub placements: Vec<Placement>,
    pub boxes: Vec<BoxPlacement>,
    /// Silhouettes and sign texts drawn on top of models (ART-ENVIRONMENT 6, 7).
    pub decals: Vec<Decal>,
    /// Placeholders for models that may be missing (drawn by the host only then).
    pub fallbacks: Vec<Fallback>,
    /// Placements (index ranges) that belong to a barrier element and disappear with it
    /// (e.g. the fallen tree and its collider).
    pub barrier_parts: Vec<(String, std::ops::Range<usize>)>,
    /// Boxes (index ranges) of a barrier element (placeholder barriers, e.g. the
    /// construction fence), hidden when it opens.
    pub barrier_boxes: Vec<(String, std::ops::Range<usize>)>,
    /// Boxes (index ranges) of the roof and upper walls of an enterable building, hidden
    /// while the player is inside (GAME-PLAYER §2, PLAY-028).
    pub roof_boxes: Vec<(String, std::ops::Range<usize>)>,
    /// Water tiles, still basins, river centrelines and foam obstacles (TECH-WATER); the
    /// renderer bakes the water field from it, the ambient animals swim in it.
    pub water: crate::water::WaterScene,
}

/// Bridge piles standing in the water (`kit_water.py` `bridge_wood`, Q-068): offsets from
/// the bridge centre (level x, z) and radius.
pub const BRIDGE_PILES: [(f32, f32); 4] =
    [(-0.55, -1.18), (0.55, -1.18), (-0.55, 1.18), (0.55, 1.18)];
pub const BRIDGE_PILE_R: f32 = 0.075;
/// Jetty posts in the water (`jetty_wood`, water end −X): offsets from the jetty centre.
pub const JETTY_POSTS: [(f32, f32); 2] = [(-2.22, -0.82), (-2.22, 0.82)];
/// Height of the fountain basin's water surface (m).
pub const FOUNTAIN_WATER_Y: f32 = 0.61;

/// Direction names in level space and their clockwise order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub const ALL: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

    fn index(self) -> i32 {
        match self {
            Dir::N => 0,
            Dir::E => 1,
            Dir::S => 2,
            Dir::W => 3,
        }
    }

    /// `k` clockwise quarter turns.
    pub fn turn_cw(self, k: i32) -> Dir {
        Dir::ALL[(self.index() + k).rem_euclid(4) as usize]
    }

    /// Nearest axis direction of a level vector.
    pub fn from_vec(v: Vec2) -> Dir {
        if v.x.abs() >= v.y.abs() {
            if v.x >= 0.0 {
                Dir::E
            } else {
                Dir::W
            }
        } else if v.y >= 0.0 {
            Dir::N
        } else {
            Dir::S
        }
    }

    pub fn offset(self) -> IVec2 {
        match self {
            Dir::N => IVec2::new(0, 1),
            Dir::E => IVec2::new(1, 0),
            Dir::S => IVec2::new(0, -1),
            Dir::W => IVec2::new(-1, 0),
        }
    }

    fn mask(set: &[Dir]) -> u8 {
        set.iter().fold(0, |m, d| m | (1 << d.index()))
    }
}

/// Path tile for a 4-neighbour connection mask (`kit_ground.py` "PATH AUTOTILING"): the
/// model and the number of clockwise quarter turns from its canonical orientation.
pub fn path_tile(connections: &[Dir]) -> (&'static str, i32) {
    const CANONICAL: [(&str, &[Dir]); 5] = [
        ("path_tile_cross", &[Dir::N, Dir::E, Dir::S, Dir::W]),
        ("path_tile_t", &[Dir::N, Dir::E, Dir::S]),
        ("path_tile_straight", &[Dir::N, Dir::S]),
        ("path_tile_curve", &[Dir::E, Dir::S]),
        ("path_tile_end", &[Dir::S]),
    ];
    let want = Dir::mask(connections);
    for (model, set) in CANONICAL {
        for k in 0..4 {
            let rotated: Vec<Dir> = set.iter().map(|d| d.turn_cw(k)).collect();
            if Dir::mask(&rotated) == want {
                return (model, k);
            }
        }
    }
    // No connection at all: a dead end is the closest look.
    ("path_tile_end", 0)
}

/// Water tile (`kit_water.py` `tile_for`): model and clockwise quarter turns. `dry_diag`
/// lists dry diagonals of a full cell by their clockwise-first side (N = NE corner).
/// River streaks run N-S in the canonical tiles; `flow_x` turns full river tiles a quarter.
pub fn water_tile(
    river: bool,
    conn: &[Dir],
    dry_diag: &[Dir],
    flow_x: bool,
) -> (&'static str, i32) {
    let names = if river {
        [
            "water_river_straight",
            "water_river_bank",
            "water_river_curve",
        ]
    } else {
        ["water_pond", "water_pond_edge", "water_pond_corner"]
    };
    if conn.len() == 4 {
        if river {
            if let Some(d) = dry_diag.first() {
                return ("water_river_inner", d.index());
            }
        }
        return (names[0], i32::from(flow_x));
    }
    let (m, k) = path_tile(conn);
    match m {
        "path_tile_t" => (names[1], k),
        "path_tile_curve" => (names[2], k),
        _ => (names[0], i32::from(flow_x)),
    }
}

/// Stones of the small rapids in `river_n` (offset from its rect corner, yaw in degrees).
pub const RIVER_ROCKS: [(f32, f32, f32); 3] =
    [(0.9, 12.4, 20.0), (2.0, 13.3, 140.0), (1.3, 14.8, 260.0)];
/// Scale of the rapids stones (`rock` is 1.48 m wide).
pub const RIVER_ROCK_SCALE: f32 = 0.3;
/// Foam radius of a rapids stone (m).
pub const RIVER_ROCK_FOAM_R: f32 = 0.2;
/// Lily pads of the pond (offset from the pond rect corner, yaw in degrees); the last two are
/// the frogs' start pads (near the jetty and the south bank, so the child can get close).
pub const POND_LILY_PADS: [(f32, f32, f32); 6] = [
    (2.2, 2.5, 0.0),
    (5.4, 3.1, 70.0),
    (3.6, 5.6, 150.0),
    (6.0, 6.2, 220.0),
    (6.8, 1.5, 0.0),
    (3.0, 1.0, 30.0),
];

/// Deterministic pseudo-random value in [0, 1) from two integers (placement jitter).
fn hash01(a: i32, b: i32) -> f32 {
    let mut h = (a as u32).wrapping_mul(0x9E37_79B1) ^ (b as u32).wrapping_mul(0x85EB_CA77);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    (h & 0xFFFF) as f32 / 65536.0
}

/// Corner piece (arms east + north in its canonical orientation) turns for the four
/// enclosure corners SW, SE, NE, NW.
const CORNER_TURNS: [i32; 4] = [0, 3, 2, 1];

/// Yaw that aligns a piece's local +X with a run axis.
fn run_yaw(axis: RunAxis) -> f32 {
    match axis {
        RunAxis::X => 0.0,
        RunAxis::Z => quarter_turns_cw_to_yaw(3),
    }
}

/// Yaw that turns a model whose front faces world +Z (level south) to face `dir`.
pub fn facing_yaw(dir: Dir) -> f32 {
    // south → k = 0; west = 1 cw turn from south; north = 2; east = 3.
    let k = match dir {
        Dir::S => 0,
        Dir::W => 1,
        Dir::N => 2,
        Dir::E => 3,
    };
    quarter_turns_cw_to_yaw(k)
}

fn rect_center(r: Rect) -> Vec2 {
    Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0)
}

/// Axis-aligned direction from the nearest point of `from` towards `to`.
fn dir_away(from: Rect, to: Vec2) -> Dir {
    let min = Vec2::new(from.x as f32, from.z as f32);
    let max = min + Vec2::new(from.w as f32, from.d as f32);
    let d = to - to.clamp(min, max);
    let d = if d.length_squared() < 1e-6 {
        to - rect_center(from)
    } else {
        d
    };
    if d.x.abs() >= d.y.abs() {
        if d.x >= 0.0 {
            Dir::E
        } else {
            Dir::W
        }
    } else if d.y >= 0.0 {
        Dir::N
    } else {
        Dir::S
    }
}

/// Moves a band run (hedge, wall) onto the row of the band next to its walkable side, so the
/// visible pieces stand at the edge the player walks along (Q-087, LAYOUT-019). One-cell
/// bands and bands with walkable ground on both or no side keep the centre line.
pub fn walkable_side_row(run: Run, rect: Rect, grid: &Grid) -> Run {
    let count = |cells: &mut dyn Iterator<Item = IVec2>| {
        cells.filter(|&c| grid.is_walkable(c, true)).count()
    };
    let mut out = run;
    match run.axis {
        RunAxis::X if rect.d == 2 => {
            let south = count(&mut (rect.x..rect.x + rect.w).map(|x| IVec2::new(x, rect.z - 1)));
            let north =
                count(&mut (rect.x..rect.x + rect.w).map(|x| IVec2::new(x, rect.z + rect.d)));
            if north > south {
                out.start.y = (rect.z + rect.d) as f32 - 0.5;
            } else if south > north {
                out.start.y = rect.z as f32 + 0.5;
            }
        }
        RunAxis::Z if rect.w == 2 => {
            let west = count(&mut (rect.z..rect.z + rect.d).map(|z| IVec2::new(rect.x - 1, z)));
            let east =
                count(&mut (rect.z..rect.z + rect.d).map(|z| IVec2::new(rect.x + rect.w, z)));
            if east > west {
                out.start.x = (rect.x + rect.w) as f32 - 0.5;
            } else if west > east {
                out.start.x = rect.x as f32 + 0.5;
            }
        }
        _ => {}
    }
    out
}

/// Centre of the row of `rect` next to its most walkable side (barrier models, Q-087).
pub fn walkable_row_center(rect: Rect, grid: &Grid) -> Vec2 {
    let c = rect_center(rect);
    let walk = |cells: Vec<IVec2>| {
        cells
            .into_iter()
            .filter(|&x| grid.is_walkable(x, true))
            .count()
    };
    let xs: Vec<i32> = (rect.x..rect.x + rect.w).collect();
    let zs: Vec<i32> = (rect.z..rect.z + rect.d).collect();
    let sides = [
        (
            walk(xs.iter().map(|&x| IVec2::new(x, rect.z - 1)).collect()),
            Vec2::new(c.x, rect.z as f32 + 0.5),
        ),
        (
            walk(xs.iter().map(|&x| IVec2::new(x, rect.z + rect.d)).collect()),
            Vec2::new(c.x, (rect.z + rect.d) as f32 - 0.5),
        ),
        (
            walk(zs.iter().map(|&z| IVec2::new(rect.x - 1, z)).collect()),
            Vec2::new(rect.x as f32 + 0.5, c.y),
        ),
        (
            walk(zs.iter().map(|&z| IVec2::new(rect.x + rect.w, z)).collect()),
            Vec2::new((rect.x + rect.w) as f32 - 0.5, c.y),
        ),
    ];
    sides
        .iter()
        .filter(|(n, _)| *n > 0)
        .max_by_key(|(n, _)| *n)
        .map_or(c, |(_, p)| *p)
}

/// Position (level) and readable-side direction of an info board element: at the rect
/// centre, facing away from its enclosure (GAME-PLAYER §5 uses the same pose).
pub fn info_board_pose(e: &Element, data: &LevelData) -> (Vec2, Dir) {
    let dir = e
        .enclosure
        .as_deref()
        .and_then(|id| data.element(id))
        .map(|enc| dir_away(enc.rect, rect_center(e.rect)))
        .unwrap_or(Dir::S);
    (rect_center(e.rect), dir)
}

impl LevelScene {
    /// Assembles the static scene of a level (all barriers closed).
    pub fn build(data: &LevelData) -> Self {
        let mut s = LevelScene::default();
        let grid = Grid::build(data, &Default::default());
        let b = data.level.bounds;

        // Per-cell classification for the ground.
        let idx = |c: IVec2| ((c.y - b.z) * b.w + (c.x - b.x)) as usize;
        let n = (b.w * b.d) as usize;
        // Water body per cell: 0 = none, 1 = river (bridge cells count as river), 2 = pond.
        let mut water = vec![0u8; n];
        // River cells flowing east-west (streaks turned a quarter).
        let mut flow_x = vec![false; n];
        let mut plaza = vec![false; n];
        for e in &data.elements {
            let kind = e.kind.as_deref();
            let body = match (e.ty, kind) {
                (ElementType::Landmark, Some("river" | "stream"))
                | (ElementType::Path, Some("bridge")) => 1,
                (ElementType::Landmark, Some("pond")) => 2,
                _ => 0,
            };
            for c in e.rect.cells().filter(|c| b.contains(*c)) {
                if body > 0 {
                    water[idx(c)] = body;
                    flow_x[idx(c)] = e.rect.w > e.rect.d;
                }
                if e.ty == ElementType::Path && kind == Some("plaza") {
                    plaza[idx(c)] = true;
                }
            }
        }
        let joined = data.parts.len() > 1;
        for c in b.cells() {
            let part = if joined {
                match data.part_at(c) {
                    Some(k) => k as u8,
                    None => continue, // between the levels: nothing (no ground)
                }
            } else {
                0
            };
            let first = s.placements.len();
            s.ground_cell(data, &grid, c, part, &water, &flow_x, &plaza, idx(c));
            for p in &mut s.placements[first..] {
                p.part = part;
            }
        }

        for e in &data.elements {
            let first = s.placements.len();
            let first_box = s.boxes.len();
            s.add_element(e, data, &grid);
            for p in &mut s.placements[first..] {
                p.part = e.part as u8;
            }
            for bx in &mut s.boxes[first_box..] {
                bx.part = e.part as u8;
            }
            if e.ty == ElementType::Barrier {
                s.barrier_parts
                    .push((e.id.clone(), first..s.placements.len()));
                s.barrier_boxes
                    .push((e.id.clone(), first_box..s.boxes.len()));
            }
        }
        for f in &data.enclosure_features {
            let first = s.placements.len();
            let first_box = s.boxes.len();
            s.enclosure_feature(f);
            let part = data.element(&f.enclosure).map_or(0, |e| e.part as u8);
            for p in &mut s.placements[first..] {
                p.part = part;
            }
            for bx in &mut s.boxes[first_box..] {
                bx.part = part;
            }
            if let Some(fb) = s.fallbacks.last_mut() {
                for bx in &mut fb.boxes {
                    bx.part = part;
                }
            }
        }
        for sc in &data.scenery {
            let first = s.placements.len();
            let first_box = s.boxes.len();
            s.scenery(sc);
            let part = data.part_at(IVec2::new(sc.rect.x, sc.rect.z)).unwrap_or(0) as u8;
            for p in &mut s.placements[first..] {
                p.part = part;
            }
            for bx in &mut s.boxes[first_box..] {
                bx.part = part;
            }
        }
        // perches (proposal Q-094): a branch / platform under every perch point
        for h in &data.hiding_places {
            if let Some(height) = h.perch_height_m {
                let first_box = s.boxes.len();
                s.perch_branch(h, data, height);
                for bx in &mut s.boxes[first_box..] {
                    bx.part = h.part as u8;
                }
            }
        }
        // Food boxes (GAME-FEED §7): label plate (model front) towards the box facing.
        for b in &data.food_boxes {
            let first = s.placements.len();
            s.model_at("food_box", b.pos(), facing_yaw(Dir::from_vec(b.facing())));
            for p in &mut s.placements[first..] {
                p.part = b.part as u8;
            }
        }
        // Carryable items stand on a table / the ground; the item itself is drawn by the
        // presentation (it moves). Water taps are small props (proposal Q-093).
        for it in &data.items {
            if it.building.is_some() {
                let first = s.placements.len();
                s.model_at("table_wood", it.pos(), 0.0);
                for p in &mut s.placements[first..] {
                    p.part = it.part as u8;
                }
            }
        }
        // Rivers (flow order, Q-066) and foam obstacles standing in the water (Q-068).
        s.water.rivers = crate::water::river_paths(data).unwrap_or_default();
        s.water_obstacles(data);
        for w in &data.water_sources {
            if let (Some(pos), "tap") = (w.pos, w.kind.as_str()) {
                let first = s.placements.len();
                let dir = w
                    .facing
                    .as_deref()
                    .map_or(Dir::S, |f| Dir::from_vec(crate::level::facing_vec(f)));
                s.model_at("water_tap", Vec2::from(pos), facing_yaw(dir));
                for p in &mut s.placements[first..] {
                    p.part = w.part as u8;
                }
            }
        }
        s
    }

    /// Ground of one cell: water tiles, path tiles with edging, plaza, pool water, sand or
    /// grass.
    #[allow(clippy::too_many_arguments)]
    fn ground_cell(
        &mut self,
        data: &LevelData,
        grid: &Grid,
        c: IVec2,
        _part: u8,
        water: &[u8],
        flow_x: &[bool],
        plaza: &[bool],
        i: usize,
    ) {
        let s = self;
        let b = data.level.bounds;
        let idx = |c: IVec2| ((c.y - b.z) * b.w + (c.x - b.x)) as usize;
        let water_at = |c: IVec2, body: u8| {
            if b.contains(c) && data.part_at(c).is_some() {
                water[idx(c)] == body
            } else {
                body == 1
            }
        };
        let path_at = |c: IVec2| b.contains(c) && grid.has_path(c);
        let jetty = |c: IVec2| {
            data.elements
                .iter()
                .any(|e| e.kind.as_deref() == Some("jetty") && e.rect.contains(c))
        };
        let pool_water = |c: IVec2| {
            data.enclosure_features
                .iter()
                .any(|f| f.is_pool() && f.rect.contains(c) && !f.is_ramp(c))
        };
        {
            let pos = level_to_world(cell_center(c));
            if water[i] > 0 {
                let body = water[i];
                let conn: Vec<Dir> = Dir::ALL
                    .into_iter()
                    .filter(|d| water_at(c + d.offset(), body))
                    .collect();
                let dry_diag: Vec<Dir> = if conn.len() == 4 {
                    // Diagonal named by its clockwise-first side: N = NE, E = SE, S = SW, W = NW.
                    Dir::ALL
                        .into_iter()
                        .filter(|d| !water_at(c + d.offset() + d.turn_cw(1).offset(), body))
                        .collect()
                } else {
                    Vec::new()
                };
                let (model, k) = water_tile(body == 1, &conn, &dry_diag, flow_x[i]);
                s.placements
                    .push(Placement::new(model, pos, quarter_turns_cw_to_yaw(k)));
                s.push_water_cell(model, c, k);
                return;
            }
            if plaza[i] {
                s.placements.push(Placement::new("plaza_tile", pos, 0.0));
            } else if path_at(c) && !jetty(c) {
                let conn: Vec<Dir> = Dir::ALL
                    .into_iter()
                    .filter(|d| path_at(c + d.offset()))
                    .collect();
                let (model, k) = path_tile(&conn);
                s.placements
                    .push(Placement::new(model, pos, quarter_turns_cw_to_yaw(k)));
                // Edging stones on the grass margin of every unconnected side.
                for d in Dir::ALL.into_iter().filter(|d| !conn.contains(d)) {
                    let off = d.offset().as_vec2() * 0.43;
                    let yaw = if matches!(d, Dir::N | Dir::S) {
                        0.0
                    } else {
                        quarter_turns_cw_to_yaw(1)
                    };
                    s.placements.push(Placement::new(
                        "path_edge",
                        level_to_world(cell_center(c) + off),
                        yaw,
                    ));
                }
            } else if pool_water(c) {
                // pool water (pond look, TECH-WATER); the rim/ramp come from `pool_tiled`
                let conn: Vec<Dir> = Dir::ALL
                    .into_iter()
                    .filter(|d| pool_water(c + d.offset()))
                    .collect();
                let (model, k) = water_tile(false, &conn, &[], false);
                s.placements
                    .push(Placement::new(model, pos, quarter_turns_cw_to_yaw(k)));
                s.push_water_cell(model, c, k);
            } else {
                let sand = data
                    .scenery
                    .iter()
                    .any(|sc| sc.kind == "sand_patch" && sc.rect.contains(c));
                let tile = if sand { "sand_tile" } else { "grass_tile" };
                s.placements.push(Placement::new(tile, pos, 0.0));
            }
        }
    }

    fn push_water_cell(&mut self, model: &str, cell: IVec2, turns: i32) {
        if let Some((shape, river)) = crate::water::TileShape::of_model(model) {
            self.water.cells.push(crate::water::WaterCell {
                cell,
                river,
                shape,
                turns,
            });
        }
    }

    /// Foam obstacles (Q-068): bridge piles, the rapids stones, jetty posts in the pond, the
    /// water wheel of the mill and the fountain jet.
    fn water_obstacles(&mut self, data: &LevelData) {
        for e in &data.elements {
            let c = rect_center(e.rect);
            match (e.ty, e.kind.as_deref()) {
                (ElementType::Path, Some("bridge")) => {
                    for (dx, dz) in BRIDGE_PILES {
                        let p = c + Vec2::new(dx, dz);
                        if self.water.is_water(p) {
                            self.obstacle(p, BRIDGE_PILE_R);
                        }
                    }
                }
                (ElementType::Path, Some("jetty")) => {
                    for (dx, dz) in JETTY_POSTS {
                        let p = c + Vec2::new(dx, dz);
                        if self.water.is_water(p) {
                            self.obstacle(p, 0.08);
                        }
                    }
                }
                (ElementType::Landmark, Some("river")) if e.id == "river_n" => {
                    let o = Vec2::new(e.rect.x as f32, e.rect.z as f32);
                    for (dx, dz, _) in RIVER_ROCKS {
                        self.obstacle(o + Vec2::new(dx, dz), RIVER_ROCK_FOAM_R);
                    }
                }
                (_, Some("mill_hut")) => {
                    let p = Vec2::new(e.rect.x as f32 - 1.5, c.y);
                    if self.water.is_water(p) {
                        self.obstacle(p, 0.3);
                    }
                }
                (ElementType::Landmark, Some("fountain")) => self.obstacle(c, 0.13),
                _ => {}
            }
        }
    }

    fn obstacle(&mut self, pos: Vec2, radius: f32) {
        let river = self.water.is_river_water(pos);
        self.water
            .obstacles
            .push(crate::water::Obstacle { pos, radius, river });
    }

    /// Wooden "Futter" board on the south facade of the food storage, centred above the row
    /// of food boxes, with the `sign-food-storage` text decal (ART-ENVIRONMENT 7).
    fn food_storage_sign(&mut self, e: &Element, data: &LevelData, wall_height: f32) {
        let r = e.rect;
        // the facade the food boxes stand in front of (their label facing), default south
        let dir = data
            .food_boxes
            .iter()
            .find(|b| r.distance_to(b.pos()) < 2.0)
            .map_or(Dir::S, |b| Dir::from_vec(b.facing()));
        let out = dir.offset().as_vec2();
        let c = rect_center(r);
        // facade of the (inset) building box, 0.05 m inside the rect edge
        let half_ext = Vec2::new(r.w as f32, r.d as f32) / 2.0 - Vec2::splat(0.05);
        let facade = c + out * half_ext;
        let board = STORAGE_SIGN_BOARD;
        let bottom = STORAGE_SIGN_BOTTOM_M.min(wall_height - board.y - 0.15);
        let center = facade + out * (board.z / 2.0);
        let along_z = matches!(dir, Dir::E | Dir::W);
        self.boxes.push(BoxPlacement {
            pos: level_to_world_at(center, bottom),
            size: board,
            yaw: if along_z {
                quarter_turns_cw_to_yaw(1)
            } else {
                0.0
            },
            color: colors::WOOD,
            fadeable: false,
            source: format!("{}:sign", e.id),
            part: 0,
        });
        // text on the front, inside the frame
        let front = level_to_world_at(
            facade + out * (board.z + DECAL_LIFT_M),
            bottom + board.y / 2.0,
        );
        let n = level_to_world(out).normalize();
        let right = Vec3::Y.cross(n);
        let half = Vec2::new(board.x, board.y) / 2.0 - Vec2::splat(STORAGE_SIGN_FRAME_M);
        let (w, h) = (512, (512.0 * half.y / half.x).round() as u32);
        self.decals.push(Decal {
            id: format!("sign:{}", e.id),
            image: DecalImage::Text {
                key: FOOD_STORAGE_SIGN_KEY,
                width_px: w,
                height_px: h,
            },
            center: front,
            right: right * half.x,
            up: Vec3::Y * half.y,
        });
    }

    fn push_box(&mut self, source: &str, center: Vec2, y0: f32, size: Vec3, color: [f32; 3]) {
        self.boxes.push(BoxPlacement {
            pos: level_to_world_at(center, y0),
            size,
            yaw: 0.0,
            color,
            fadeable: y0 + size.y > 1.5,
            source: source.to_owned(),
            part: 0,
        });
    }

    /// Box over a whole element rectangle, inset a little so neighbours get outlines.
    fn rect_box(&mut self, e: &Element, y0: f32, height: f32, color: [f32; 3]) {
        let r = e.rect;
        let size = Vec3::new(r.w as f32 - 0.1, height, r.d as f32 - 0.1);
        self.push_box(&e.id, rect_center(r), y0, size, color);
    }

    fn run_pieces(&mut self, run: &Run, two: &'static str, one: &'static str) {
        let yaw = run_yaw(run.axis);
        for seg in run.segments() {
            self.placements.push(Placement::new(
                if seg.length_m == 2 { two } else { one },
                level_to_world(run.segment_center(seg)),
                yaw,
            ));
        }
    }

    fn model_at(&mut self, model: &'static str, p: Vec2, yaw: f32) {
        self.placements
            .push(Placement::new(model, level_to_world(p), yaw));
    }

    fn model_scaled(&mut self, model: &'static str, p: Vec2, yaw: f32, scale: f32) {
        self.placements.push(Placement {
            model,
            pos: level_to_world(p),
            yaw,
            scale,
            part: 0,
        });
    }

    /// Flat non-solid ground box (scenery dressing).
    fn flat(&mut self, source: &str, c: Vec2, size: Vec3, y0: f32, color: [f32; 3]) {
        self.boxes.push(BoxPlacement {
            pos: level_to_world_at(c, y0),
            size,
            yaw: 0.0,
            color,
            fadeable: false,
            source: source.to_owned(),
            part: 0,
        });
    }

    /// Sparse tree area (GAME-LAYOUT "Forests", Q-085): the listed trees and bushes, each
    /// with its own collider (the element itself is not solid).
    fn sparse_trees(&mut self, e: &Element) {
        for (i, t) in e.trees.iter().enumerate() {
            let model = match t.model.as_str() {
                "bush" => "bush",
                "tree_grove" => "tree_grove",
                "tree_eucalyptus" => "tree_eucalyptus",
                _ => "tree_round",
            };
            let yaw = hash01(i as i32 + e.rect.x * 7, e.rect.z) * std::f32::consts::TAU;
            self.model_at(model, t.pos(), yaw);
        }
    }

    /// Bush border on the walkable sides of a dense tree area (`edge = "bushes"`, Q-085):
    /// a bush about every 1.3 m, centred 0.6 m inside the edge, so the solid edge is never
    /// an invisible wall (LAYOUT-019).
    fn bush_border(&mut self, e: &Element, grid: &Grid) {
        let r = e.rect;
        let (x0, z0, x1, z1) = (r.x, r.z, r.x + r.w, r.z + r.d);
        let walk = |c: IVec2| grid.is_walkable(c, false);
        // per side: first outside cell, step along the side, bush line start, length
        let sides = [
            (
                IVec2::new(x0, z0 - 1),
                IVec2::X,
                Vec2::new(x0 as f32, z0 as f32 + 0.6),
                r.w,
            ),
            (
                IVec2::new(x0, z1),
                IVec2::X,
                Vec2::new(x0 as f32, z1 as f32 - 0.6),
                r.w,
            ),
            (
                IVec2::new(x0 - 1, z0),
                IVec2::Y,
                Vec2::new(x0 as f32 + 0.6, z0 as f32),
                r.d,
            ),
            (
                IVec2::new(x1, z0),
                IVec2::Y,
                Vec2::new(x1 as f32 - 0.6, z0 as f32),
                r.d,
            ),
        ];
        for (k, (out0, step_c, start, len)) in sides.into_iter().enumerate() {
            let len = len as f32;
            let n = (len / 1.3).round().max(1.0) as i32;
            let step = len / n as f32;
            for i in 0..n {
                let t = step * (i as f32 + 0.5);
                if !walk(out0 + step_c * t as i32) {
                    continue;
                }
                let yaw = hash01(i + k as i32 * 31, r.x + r.z) * std::f32::consts::TAU;
                self.model_at("bush", start + step_c.as_vec2() * t, yaw);
            }
        }
    }

    /// Hippo pool and other enclosure features (GAME-LEVEL-1 "Hippo enclosure pool"): the
    /// water tiles come from the ground pass; the `pool_tiled` model (tiled rim + ramp) has a
    /// placeholder fallback; edge stones are small rocks; a `hut` is a wooden hut box.
    fn enclosure_feature(&mut self, f: &crate::level::EnclosureFeature) {
        let r = f.rect;
        let c = rect_center(r);
        match f.kind.as_str() {
            "pool" => {
                let model: &'static str = match f.model.as_deref() {
                    Some("pond_stone_rim") => "pond_stone_rim",
                    _ => "pool_tiled",
                };
                let stone_rim = model == "pond_stone_rim";
                self.model_at(model, c, 0.0);
                let mut fb = Fallback {
                    model,
                    ..Fallback::default()
                };
                let tile: [f32; 3] = if stone_rim {
                    colors::STONE
                } else {
                    [0.86, 0.93, 0.97]
                };
                const TILE_DARK: [f32; 3] = [0.55, 0.78, 0.90];
                let rim = 0.18;
                let h = 0.4;
                let (x0, z0) = (r.x as f32, r.z as f32);
                let (x1, z1) = (x0 + r.w as f32, z0 + r.d as f32);
                let mut rim_box = |a: Vec2, b: Vec2| {
                    let size = Vec3::new((b.x - a.x).abs().max(rim), h, (b.y - a.y).abs().max(rim));
                    fb.boxes.push(BoxPlacement {
                        pos: level_to_world_at((a + b) / 2.0, 0.0),
                        size,
                        yaw: 0.0,
                        color: tile,
                        fadeable: false,
                        source: format!("{}:rim", f.id),
                        part: 0,
                    });
                };
                // rim along the four sides, open where the ramp meets the rim
                let ramp = f.ramp;
                let side = f.ramp_side.as_deref().unwrap_or("-x");
                let open = |axis_lo: f32, axis_hi: f32| -> Vec<(f32, f32)> {
                    match ramp {
                        Some(rp) => {
                            let (a, b) = if side == "-x" || side == "+x" {
                                (rp.z as f32, (rp.z + rp.d) as f32)
                            } else {
                                (rp.x as f32, (rp.x + rp.w) as f32)
                            };
                            vec![(axis_lo, a), (b, axis_hi)]
                        }
                        None => vec![(axis_lo, axis_hi)],
                    }
                };
                let south = if side == "-z" {
                    open(x0, x1)
                } else {
                    vec![(x0, x1)]
                };
                let north = if side == "+z" {
                    open(x0, x1)
                } else {
                    vec![(x0, x1)]
                };
                let west = if side == "-x" {
                    open(z0, z1)
                } else {
                    vec![(z0, z1)]
                };
                let east = if side == "+x" {
                    open(z0, z1)
                } else {
                    vec![(z0, z1)]
                };
                for (a, b) in south {
                    rim_box(Vec2::new(a, z0 + rim / 2.0), Vec2::new(b, z0 + rim / 2.0));
                }
                for (a, b) in north {
                    rim_box(Vec2::new(a, z1 - rim / 2.0), Vec2::new(b, z1 - rim / 2.0));
                }
                // west / east segments run between the south and north rims (no overlapping
                // corner blocks: coplanar tops and sides would z-fight, ARCH-005)
                let inner = |(a, b): (f32, f32)| {
                    let (a, b) = (a.max(z0 + rim), b.min(z1 - rim));
                    (b > a + 1e-3).then_some((a, b))
                };
                for (a, b) in west.into_iter().filter_map(inner) {
                    rim_box(Vec2::new(x0 + rim / 2.0, a), Vec2::new(x0 + rim / 2.0, b));
                }
                for (a, b) in east.into_iter().filter_map(inner) {
                    rim_box(Vec2::new(x1 - rim / 2.0, a), Vec2::new(x1 - rim / 2.0, b));
                }
                if let Some(rp) = ramp {
                    // shallow tiled ramp (flat placeholder) with darker stripes
                    fb.boxes.push(BoxPlacement {
                        pos: level_to_world_at(rect_center(rp), 0.0),
                        size: Vec3::new(rp.w as f32 - 0.04, 0.06, rp.d as f32 - 0.04),
                        yaw: 0.0,
                        color: tile,
                        fadeable: false,
                        source: format!("{}:ramp", f.id),
                        part: 0,
                    });
                    for k in 0..(rp.d.max(rp.w) * 2) {
                        let t = k as f32 * 0.5 + 0.25;
                        let (pos, size) = if side == "-x" || side == "+x" {
                            (
                                Vec2::new(rect_center(rp).x, rp.z as f32 + t),
                                Vec3::new(rp.w as f32 - 0.1, 0.07, 0.06),
                            )
                        } else {
                            (
                                Vec2::new(rp.x as f32 + t, rect_center(rp).y),
                                Vec3::new(0.06, 0.07, rp.d as f32 - 0.1),
                            )
                        };
                        fb.boxes.push(BoxPlacement {
                            pos: level_to_world_at(pos, 0.0),
                            size,
                            yaw: 0.0,
                            color: TILE_DARK,
                            fadeable: false,
                            source: format!("{}:ramp", f.id),
                            part: 0,
                        });
                    }
                }
                self.fallbacks.push(fb);
                for (i, st) in f.edge_stones.iter().enumerate() {
                    let yaw = hash01(i as i32, r.x) * std::f32::consts::TAU;
                    self.model_scaled("rock", Vec2::from(*st), yaw, 0.5);
                }
            }
            "hut" => {
                // wooden hut placeholder (area reserved in the layout, model to come)
                let size = Vec3::new(r.w as f32 - 0.6, 2.0, r.d as f32 - 0.8);
                self.push_box(&f.id, c, 0.0, size, colors::WOOD);
                let roof = Vec3::new(r.w as f32 - 0.3, 0.6, r.d as f32 - 0.5);
                self.push_box(&f.id, c, 2.0, roof, colors::ROOF);
            }
            _ => {}
        }
    }

    /// Non-solid riddle dressing (`[[scenery]]`, Q-080): tall grass with flowers, mud,
    /// tree shade, a leaf pile with a rake. Sand is a ground tile (ground pass).
    fn scenery(&mut self, sc: &crate::level::SceneryData) {
        let r = sc.rect;
        let id = sc.id.as_str();
        match sc.kind.as_str() {
            "tall_grass" => {
                const FLOWERS: [[f32; 3]; 3] =
                    [[0.93, 0.30, 0.30], [0.98, 0.85, 0.25], [0.97, 0.97, 0.95]];
                for c in r.cells() {
                    for k in 0..3 {
                        let h = hash01(c.x * 3 + k, c.y * 5 - k);
                        let p =
                            cell_center(c) + Vec2::new(h - 0.5, hash01(c.y, c.x + k) - 0.5) * 0.8;
                        self.model_scaled("grass_tuft", p, h * 6.0, 3.0);
                    }
                    let h = hash01(c.x, c.y * 7);
                    let p = cell_center(c) + Vec2::new(0.3 - h * 0.6, h * 0.5 - 0.25);
                    self.flat(
                        id,
                        p,
                        Vec3::new(0.16, 0.1, 0.16),
                        0.55,
                        FLOWERS[(c.x + c.y).rem_euclid(3) as usize],
                    );
                }
            }
            "mud_puddle" => {
                const MUD: [f32; 3] = [0.47, 0.33, 0.20];
                const WET: [f32; 3] = [0.60, 0.45, 0.30];
                self.flat(
                    id,
                    rect_center(r),
                    Vec3::new(r.w as f32 - 0.5, 0.16, r.d as f32 - 0.5),
                    0.0,
                    MUD,
                );
                for c in r.cells().filter(|c| (c.x + c.y) % 3 == 0) {
                    let h = hash01(c.x, c.y);
                    self.flat(
                        id,
                        cell_center(c) + Vec2::splat(h - 0.5) * 0.4,
                        Vec3::new(0.35, 0.17, 0.2),
                        0.0,
                        WET,
                    );
                }
            }
            "flat_rocks" => {
                // big flat light-grey slabs flush with the grass (walkable)
                for (k, c) in r.cells().filter(|c| (c.x * 3 + c.y) % 4 != 0).enumerate() {
                    let h = hash01(c.x, c.y);
                    self.flat(
                        id,
                        cell_center(c) + Vec2::splat(h - 0.5) * 0.2,
                        Vec3::new(0.9 - h * 0.2, 0.08, 0.85),
                        0.0,
                        if k % 3 == 0 {
                            colors::ROCK
                        } else {
                            colors::STONE
                        },
                    );
                }
            }
            "petal_carpet" => {
                for c in r.cells() {
                    for k in 0..2 {
                        let p = cell_center(c)
                            + Vec2::new(hash01(c.x + k, c.y) - 0.5, hash01(c.y, c.x - k) - 0.5)
                                * 0.8;
                        // tops 5 mm apart so overlapping petals never share a plane (ARCH-005)
                        let hgt = 0.06 + 0.005 * ((c.x + c.y * 3 + k) as f32).rem_euclid(5.0);
                        self.flat(id, p, Vec3::new(0.25, hgt, 0.2), 0.0, colors::BLOSSOM);
                    }
                }
            }
            "trampoline" => {
                let c = rect_center(r);
                self.flat(
                    id,
                    c,
                    Vec3::new(r.w as f32 - 0.2, 0.12, r.d as f32 - 0.2),
                    0.0,
                    colors::ROOF,
                );
                self.flat(
                    id,
                    c,
                    Vec3::new(r.w as f32 - 0.6, 0.14, r.d as f32 - 0.6),
                    0.0,
                    colors::WAGON_BLUE,
                );
            }
            "wet_lawn" => {
                let c = rect_center(r);
                self.flat(
                    id,
                    c,
                    Vec3::new(r.w as f32 - 0.1, 0.075, r.d as f32 - 0.1),
                    0.0,
                    colors::WET_GRASS,
                );
                self.flat(id, c, Vec3::new(0.12, 0.45, 0.12), 0.0, colors::ROCK);
                self.flat(id, c, Vec3::new(0.5, 0.06, 0.08), 0.45, colors::ROCK);
                for k in 0..10 {
                    let a = k as f32 * 0.63;
                    let p = c + Vec2::new(a.cos(), a.sin()) * (1.0 + (k % 3) as f32);
                    self.flat(
                        id,
                        p,
                        Vec3::new(0.08, 0.08, 0.08),
                        0.6 + (k % 4) as f32 * 0.25,
                        colors::WATER_LIGHT,
                    );
                }
                // small rainbow (three stacked arcs as bars)
                for (k, col) in [colors::ROOF, colors::GOLD, colors::WAGON_BLUE]
                    .into_iter()
                    .enumerate()
                {
                    self.flat(
                        id,
                        c + Vec2::X * 1.5,
                        Vec3::new(0.08, 0.08, 2.0 - k as f32 * 0.3),
                        1.8 - k as f32 * 0.1,
                        col,
                    );
                }
            }
            "bark_mulch" => {
                self.flat(
                    id,
                    rect_center(r),
                    Vec3::new(r.w as f32 - 0.1, 0.075, r.d as f32 - 0.1),
                    0.0,
                    colors::BARK,
                );
            }
            "tree_shade" => {
                const SHADE: [f32; 3] = [0.33, 0.50, 0.24];
                self.flat(
                    id,
                    rect_center(r),
                    Vec3::new(r.w as f32, 0.15, r.d as f32 - 0.2),
                    0.0,
                    SHADE,
                );
            }
            "leaf_pile" => {
                const LEAVES: [[f32; 3]; 3] =
                    [[0.88, 0.36, 0.18], [0.95, 0.72, 0.20], [0.66, 0.40, 0.20]];
                let c = rect_center(r);
                for k in 0..7 {
                    let h = hash01(k, r.x);
                    let p = c + Vec2::new(
                        (h - 0.5) * (r.w as f32 - 1.0),
                        (hash01(r.z, k) - 0.5) * (r.d as f32 - 0.8),
                    );
                    let s = 0.7 + h * 0.5;
                    self.flat(
                        id,
                        p,
                        Vec3::new(s, 0.28 + h * 0.2, s * 0.8),
                        0.0,
                        LEAVES[(k % 3) as usize],
                    );
                }
                // rake leaning on the nearest tree north-west of the pile (placeholder)
                self.flat(
                    id,
                    Vec2::new(r.x as f32 + 1.0, r.z as f32 - 1.3),
                    Vec3::new(0.06, 1.4, 0.06),
                    0.0,
                    colors::WOOD,
                );
            }
            _ => {}
        }
    }

    /// Trees on a grid of about `spacing` metres inside a rectangle, jittered and turned
    /// deterministically (seeded by the cell) so groves do not look like a grid.
    fn trees(&mut self, e: &Element, model: &'static str, spacing: f32) {
        let r = e.rect;
        let cols = ((r.w as f32) / spacing).round().max(1.0) as i32;
        let rows = ((r.d as f32) / spacing).round().max(1.0) as i32;
        let cw = r.w as f32 / cols as f32;
        let cd = r.d as f32 / rows as f32;
        for j in 0..rows {
            for i in 0..cols {
                let h = hash01(i + r.x * 31, j + r.z * 17);
                let c = Vec2::new(
                    r.x as f32 + cw * (i as f32 + 0.5) + (h - 0.5) * 0.5,
                    r.z as f32 + cd * (j as f32 + 0.5) + (0.5 - h) * 0.5,
                );
                self.model_at(model, c, h * std::f32::consts::TAU);
            }
        }
    }

    /// Dense bamboo thicket (`bamboo_sw`, `loc_bamboo`): clumps inside the solid rect,
    /// turned 0° or 180° only, inset 0.75 m (north/south) / 0.6 m (east/west) on walkable
    /// sides: the leaves stand at the edge (no invisible wall, LAYOUT-019) and the colliders
    /// reach at most 0.13 m past it, never onto a walkable cell centre.
    fn bamboo_thicket(&mut self, e: &Element, grid: &Grid) {
        let r = e.rect;
        let walk = |c: IVec2| grid.is_walkable(c, false);
        let side = |cells: &mut dyn Iterator<Item = IVec2>, open: f32| {
            if cells.filter(|&c| walk(c)).count() > 0 {
                open
            } else {
                0.4
            }
        };
        let w = side(&mut (r.z..r.z + r.d).map(|z| IVec2::new(r.x - 1, z)), 0.6);
        let east = side(&mut (r.z..r.z + r.d).map(|z| IVec2::new(r.x + r.w, z)), 0.6);
        let s = side(&mut (r.x..r.x + r.w).map(|x| IVec2::new(x, r.z - 1)), 0.75);
        let n = side(
            &mut (r.x..r.x + r.w).map(|x| IVec2::new(x, r.z + r.d)),
            0.75,
        );
        let (x0, x1) = (r.x as f32 + w, (r.x + r.w) as f32 - east);
        let (z0, z1) = (r.z as f32 + s, (r.z + r.d) as f32 - n);
        let cols = ((x1 - x0) / 0.9).round().max(0.0) as i32 + 1;
        let rows = ((z1 - z0) / 0.9).round().max(0.0) as i32 + 1;
        for j in 0..rows {
            for i in 0..cols {
                let fx = if cols > 1 {
                    i as f32 / (cols - 1) as f32
                } else {
                    0.5
                };
                let fz = if rows > 1 {
                    j as f32 / (rows - 1) as f32
                } else {
                    0.5
                };
                let p = Vec2::new(x0 + (x1 - x0) * fx, z0 + (z1 - z0) * fz);
                // the outer clumps keep yaw 0 (longest leaves north/east), the others alternate
                let edge = i == cols - 1 || j == rows - 1;
                let yaw = if edge || (i + j) % 2 == 0 {
                    0.0
                } else {
                    std::f32::consts::PI
                };
                self.model_at("bamboo", p, yaw);
            }
        }
    }

    /// Small rapids with stones in the upper river (`river_n` notes); the ducks near the
    /// bridge (riddle detail of `loc_river`) are ambient animals (GAME-AMBIENT), not props.
    fn river_dressing(&mut self, e: &Element) {
        if e.id != "river_n" {
            return;
        }
        let r = e.rect;
        for (dx, dz, yaw) in RIVER_ROCKS {
            let first = self.placements.len();
            self.model_scaled(
                "rock",
                Vec2::new(r.x as f32 + dx, r.z as f32 + dz),
                yaw.to_radians(),
                RIVER_ROCK_SCALE,
            );
            self.placements[first].pos.y = -0.07;
        }
    }

    /// Lily pads, frogs and reeds (riddle details of `loc_pond`).
    fn pond_dressing(&mut self, e: &Element) {
        let r = e.rect;
        let o = Vec2::new(r.x as f32, r.z as f32);
        // the frogs are ambient animals sitting on these pads (GAME-AMBIENT 7)
        for (dx, dz, yaw) in POND_LILY_PADS {
            self.model_at("lily_pad", o + Vec2::new(dx, dz), yaw.to_radians());
        }
        for (dx, dz) in [(0.6, 7.3), (1.4, 7.5), (7.3, 7.3), (0.5, 0.8)] {
            self.model_at(
                "reed",
                o + Vec2::new(dx, dz),
                hash01(dx as i32, dz as i32) * 6.0,
            );
        }
    }

    /// Enclosure dressing (placeholder layout): bamboo for the panda, bushes and a rock for
    /// the zebra, stones for the hippo.
    fn enclosure_dressing(&mut self, e: &Element) {
        let r = e.rect;
        let at = |fx: f32, fz: f32| {
            Vec2::new(r.x as f32 + fx * r.w as f32, r.z as f32 + fz * r.d as f32)
        };
        if e.animal.as_deref() == Some("panda") {
            // cut bamboo on a wooden feeding rack, no growing bamboo (Q-081)
            let rack = at(0.75, 0.75);
            self.push_box(&e.id, rack, 0.0, Vec3::new(2.0, 0.9, 0.7), colors::WOOD);
            for k in 0..5 {
                let dz = -0.24 + k as f32 * 0.12;
                self.push_box(
                    &e.id,
                    rack + Vec2::new(0.0, dz),
                    0.9,
                    Vec3::new(2.2, 0.08, 0.08),
                    [0.45, 0.70, 0.30],
                );
            }
        }
        let list: &[(&'static str, f32, f32)] = match e.animal.as_deref() {
            Some("panda") => &[("bush", 0.3, 0.3), ("bush", 0.2, 0.8)],
            Some("zebra") => &[
                ("bush", 0.25, 0.8),
                ("bush", 0.6, 0.25),
                ("rock", 0.3, 0.4),
                ("grass_tuft", 0.55, 0.6),
            ],
            // eucalyptus trees of normal height (no blossoms, no house: riddle guards)
            Some("koala") => &[
                ("tree_eucalyptus", 0.3, 0.3),
                ("tree_eucalyptus", 0.65, 0.7),
            ],
            Some("elephant") => &[("rock", 0.2, 0.85)],
            Some("lion") => &[("bush", 0.15, 0.3)],
            Some("snow_fox") => &[("rock", 0.25, 0.3), ("bush", 0.8, 0.35)],
            Some("monkey") => &[("bush", 0.15, 0.8)],
            // hippo: pool, hut and edge stones come from [[enclosure_feature]] (Q-085)
            _ => &[],
        };
        let id = e.id.as_str();
        match e.animal.as_deref() {
            Some("giraffe") => {
                // tall feeding rack with leafy branches (4 m), no tower (riddle guard)
                let p = at(0.7, 0.7);
                // the pole ends inside the leafy top (no shared top face, ARCH-005)
                self.push_box(id, p, 0.0, Vec3::new(0.2, 4.1, 0.2), colors::WOOD);
                self.push_box(id, p, 3.4, Vec3::new(1.4, 0.8, 0.8), colors::TREE_CROWN);
                self.push_box(
                    id,
                    at(0.25, 0.75),
                    0.0,
                    Vec3::new(2.6, 3.2, 2.4),
                    colors::WOOD_LIGHT,
                );
            }
            Some("lion") => {
                // wooden sun deck with a straw roof and a lying log (no rocks: riddle guard)
                let p = at(0.7, 0.5);
                self.push_box(id, p, 0.0, Vec3::new(2.6, 0.4, 2.0), colors::WOOD);
                self.push_box(id, p, 2.2, Vec3::new(2.8, 0.3, 2.2), colors::SAWDUST);
                self.push_box(
                    id,
                    at(0.3, 0.6),
                    0.0,
                    Vec3::new(2.4, 0.45, 0.45),
                    colors::TREE_TRUNK,
                );
            }
            Some("elephant") => {
                self.push_box(
                    id,
                    at(0.15, 0.2),
                    0.0,
                    Vec3::new(1.2, 1.5, 0.5),
                    colors::WOOD,
                );
            }
            Some("monkey") => {
                // climbing frame of logs and ropes, a monkey house
                let p = at(0.35, 0.6);
                for dx in [-1.2, 1.2] {
                    self.push_box(
                        id,
                        p + Vec2::X * dx,
                        0.0,
                        Vec3::new(0.25, 3.2, 0.25),
                        colors::TREE_TRUNK,
                    );
                }
                self.push_box(id, p, 3.0, Vec3::new(2.7, 0.22, 0.22), colors::TREE_TRUNK);
                self.push_box(id, p, 1.2, Vec3::new(0.05, 1.8, 0.05), colors::ROPE);
                self.push_box(
                    id,
                    at(0.8, 0.75),
                    0.0,
                    Vec3::new(2.0, 1.8, 1.6),
                    colors::WOOD_LIGHT,
                );
                self.push_box(
                    id,
                    at(0.8, 0.75),
                    1.8,
                    Vec3::new(2.3, 0.4, 1.9),
                    colors::ROOF,
                );
            }
            Some("snow_fox") => {
                self.push_box(
                    id,
                    at(0.75, 0.65),
                    0.0,
                    Vec3::new(1.6, 1.0, 1.2),
                    colors::WOOD,
                );
                self.push_box(
                    id,
                    at(0.75, 0.65),
                    1.0,
                    Vec3::new(1.8, 0.3, 1.4),
                    colors::SAWDUST,
                );
            }
            _ => {}
        }
        for (i, (m, fx, fz)) in list.iter().enumerate() {
            self.model_at(
                m,
                at(*fx, *fz),
                hash01(i as i32, r.x) * std::f32::consts::TAU,
            );
        }
    }

    fn add_element(&mut self, e: &Element, data: &LevelData, grid: &Grid) {
        let kind = e.kind.as_deref().unwrap_or("");
        let h = e.height_m;
        match (e.ty, kind) {
            // Bridge deck runs W-E over the N-S river; jetty water end is -X (pond west).
            (ElementType::Path, "bridge") => self.model_at("bridge_wood", rect_center(e.rect), 0.0),
            (ElementType::Path, "jetty") => self.model_at("jetty_wood", rect_center(e.rect), 0.0),
            (ElementType::Path, _) | (ElementType::HidingPlace, _) => {}
            (ElementType::Landmark, "river") => self.river_dressing(e),
            (ElementType::Landmark, "pond") => self.pond_dressing(e),
            (ElementType::Landmark, "rock_hill") => {
                self.rect_box(e, 0.0, h.unwrap_or(5.0), colors::ROCK)
            }
            (ElementType::Landmark, "map_board") => {
                let spawn = data
                    .parts
                    .get(e.part)
                    .map_or(data.spawn.cell(), |p| p.spawn.cell());
                let to_spawn = cell_center(spawn);
                self.model_at(
                    "map_board",
                    rect_center(e.rect),
                    facing_yaw(dir_away(e.rect, to_spawn)),
                );
            }
            (ElementType::Decoration | ElementType::Boundary, "hedge" | "zoo_wall") => {
                let (two, one) = if kind == "hedge" {
                    ("hedge", "hedge_1m")
                } else {
                    ("zoo_wall", "zoo_wall_1m")
                };
                let bounds = data
                    .parts
                    .get(e.part)
                    .map_or(data.level.bounds, |p| p.bounds);
                match band_run(e.rect, bounds) {
                    Some(run) => self.run_pieces(&walkable_side_row(run, e.rect, grid), two, one),
                    None => self.rect_box(e, 0.0, h.unwrap_or(3.0), colors::HEDGE),
                }
            }
            (ElementType::Decoration, "tree_grove" | "trees") if e.is_sparse() => {
                self.sparse_trees(e)
            }
            (ElementType::Decoration, "tree_grove") => {
                self.trees(e, "tree_grove", 2.5);
                if e.edge.as_deref() == Some("bushes") {
                    self.bush_border(e, grid);
                }
            }
            (ElementType::Decoration, "trees") => {
                self.trees(e, "tree_round", 3.0);
                if e.edge.as_deref() == Some("bushes") {
                    self.bush_border(e, grid);
                }
            }
            (ElementType::Decoration, "bamboo") => self.bamboo_thicket(e, grid),
            (ElementType::Decoration, "bench") => self.rect_box(e, 0.0, 0.5, colors::WOOD),
            (ElementType::Decoration, "info_board") => {
                let (pos, dir) = info_board_pose(e, data);
                self.model_at("info_board", pos, facing_yaw(dir));
            }
            (ElementType::Enclosure, _) => self.enclosure(e),
            (ElementType::Building, "entrance") => {
                // Arch: two pillars and a beam, so the player is visible through it. The
                // pillars end under the beam: no coplanar faces (ARCH-005 — they used to
                // share the top at `height` and flickered red/blue).
                let r = e.rect;
                let c = rect_center(r);
                let height = h.unwrap_or(5.0);
                let beam_h = 0.7;
                let pillar = Vec3::new(1.2, height - beam_h, r.d as f32 - 0.2);
                let dx = r.w as f32 / 2.0 - 0.6;
                self.push_box(&e.id, c - Vec2::X * dx, 0.0, pillar, colors::STONE);
                self.push_box(&e.id, c + Vec2::X * dx, 0.0, pillar, colors::STONE);
                let beam = Vec3::new(r.w as f32, beam_h, 0.8);
                self.push_box(&e.id, c, height - beam_h, beam, colors::ROOF);
                // closed turnstiles in the arch on the plaza side (QA F8: no invisible wall)
                let row = Vec2::new(c.x, r.z as f32 + r.d as f32 - 0.5);
                let open_w = r.w as f32 - 2.4;
                let n = (open_w / 1.2).round().max(1.0) as i32;
                for i in 0..n {
                    let x = r.x as f32 + 1.2 + open_w * (i as f32 + 0.5) / n as f32;
                    self.push_box(
                        &e.id,
                        Vec2::new(x, row.y),
                        0.0,
                        Vec3::new(0.12, 1.0, 0.12),
                        [0.55, 0.57, 0.62],
                    );
                    self.push_box(
                        &e.id,
                        Vec2::new(x, row.y),
                        0.55,
                        Vec3::new(open_w / n as f32 - 0.1, 0.08, 0.5),
                        [0.70, 0.72, 0.76],
                    );
                }
                // top rail slightly deeper than the 0.12 m posts so their faces never
                // share a plane (ARCH-005)
                self.push_box(
                    &e.id,
                    row,
                    0.95,
                    Vec3::new(open_w, 0.1, 0.16),
                    [0.70, 0.72, 0.76],
                );
            }
            (ElementType::Building, _) if e.is_enterable() => self.enterable_building(e),
            (ElementType::Building, "kiosk") => self.kiosk(e),
            (ElementType::Landmark | ElementType::Decoration, _)
                if placeholder_kind(kind).is_some() =>
            {
                self.landmark_placeholder(e, data)
            }
            (ElementType::Barrier, "construction_fence") => self.construction_fence(e, grid),
            (ElementType::Building, _) => {
                let height = h.unwrap_or(4.0);
                self.rect_box(e, 0.0, height * 0.7, colors::BUILDING);
                if kind == "food_storage" {
                    self.food_storage_sign(e, data, height * 0.7);
                }
                let r = e.rect;
                let roof = Vec3::new(r.w as f32 + 0.3, height * 0.3, r.d as f32 + 0.3);
                self.push_box(&e.id, rect_center(r), height * 0.7, roof, colors::ROOF);
            }
            (ElementType::Barrier, "fallen_tree") => {
                // 0.3 m east of the rect centre so the crown stays out of the walkable
                // column of path_ne (GAME-LEVEL-1 "Collision and billboards")
                self.model_at("fallen_tree", rect_center(e.rect) + Vec2::X * 0.3, 0.0)
            }
            (ElementType::Barrier, "closed_gate") => {
                // on the walkable side of the band (Q-087): the gate leaf 0.25 m behind the
                // edge (its pillars reach 0.11 m out, solid by the footprint; LAYOUT-019)
                let row = walkable_row_center(e.rect, grid);
                let c = rect_center(e.rect);
                let c = c + (row - c).normalize_or_zero() * ((row - c).length() + 0.25);
                self.model_at("gate_zoo_closed", c, 0.0)
            }
            (ElementType::Barrier, "road_block") => {
                // Road block across the path (faces west) on the walkable-side row: its bar
                // 0.3 m behind the edge (the feet reach 0.12 m onto the path; Q-087, LAYOUT-019);
                // sign, cart and cones behind it.
                let c = rect_center(e.rect);
                let r = e.rect;
                let west = facing_yaw(Dir::W);
                let block = Vec2::new(r.x as f32 + 0.3, c.y);
                self.model_at("road_block", block, west);
                self.model_at(
                    "repair_sign",
                    block + Vec2::new(0.85, -0.75),
                    (-35f32).to_radians(),
                );
                self.model_at("zookeeper_cart", block + Vec2::new(1.1, 0.95), west);
                self.model_at("traffic_cone", block + Vec2::new(0.7, 0.35), 0.0);
                self.model_at("traffic_cone", block + Vec2::new(1.25, -1.2), 0.0);
            }
            (ElementType::Barrier, _) => self.rect_box(e, 0.0, h.unwrap_or(1.2), colors::BARRIER),
            _ => self.rect_box(e, 0.0, h.unwrap_or(1.0), colors::DEFAULT),
        }
    }

    /// Coloured box with an explicit yaw (placeholder parts).
    fn part_box(&mut self, source: &str, center: Vec2, y0: f32, size: Vec3, color: [f32; 3]) {
        self.push_box(source, center, y0, size, color);
    }

    /// Placeholder of a landmark / decoration without a model (PROD-POC "Placeholders"):
    /// coloured boxes with the element's height inside its (solid) rectangle, so the cell
    /// collision stays the collider. Perch trees and the ship leave room for the perch point
    /// (proposal Q-094).
    fn landmark_placeholder(&mut self, e: &Element, data: &LevelData) {
        let r = e.rect;
        let c = rect_center(r);
        let id = e.id.as_str();
        let (w, d) = (r.w as f32, r.d as f32);
        let kind = e.kind.as_deref().unwrap_or("");
        match kind {
            "fountain" => {
                self.part_box(id, c, 0.0, Vec3::new(w - 0.2, 0.6, d - 0.2), colors::STONE);
                // basin water: still-water tiles drawn by the water shader (TECH-WATER), 2 × 2
                // full pond tiles scaled to the inner basin
                let (iw, id_) = (w - 0.7, d - 0.7);
                let scale = iw.max(id_) / 2.0;
                for (sx, sz) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0f32)] {
                    let p = c + Vec2::new(sx * iw / 4.0, sz * id_ / 4.0);
                    let first = self.placements.len();
                    self.model_scaled("water_pond", p, 0.0, scale);
                    self.placements[first].pos.y = FOUNTAIN_WATER_Y;
                }
                self.water.stills.push(crate::water::StillWater {
                    min: c - Vec2::new(iw, id_) / 2.0,
                    max: c + Vec2::new(iw, id_) / 2.0,
                });
                self.part_box(id, c, 0.62, Vec3::new(0.18, 1.8, 0.18), colors::WATER_LIGHT);
                self.part_box(id, c, 2.3, Vec3::new(0.7, 0.2, 0.7), colors::WATER_LIGHT);
                for k in 0..5 {
                    let h = hash01(k, r.x);
                    let p =
                        c + Vec2::new((h - 0.5) * (w - 1.2), (hash01(r.z, k) - 0.5) * (d - 1.2));
                    self.part_box(id, p, 0.62, Vec3::new(0.14, 0.02, 0.14), colors::GOLD);
                }
            }
            "waterfall" => {
                // rock ledge along the north wall, white falling water on its south face
                self.part_box(
                    id,
                    c + Vec2::Y * 0.4,
                    0.0,
                    Vec3::new(w - 0.1, 2.9, d - 0.9),
                    colors::ROCK,
                );
                for k in 0..4 {
                    let x = r.x as f32 + 0.6 + k as f32 * (w - 1.2) / 3.0;
                    self.model_scaled(
                        "rock",
                        Vec2::new(x, r.z as f32 + 0.6),
                        hash01(k, 3) * 6.0,
                        1.3,
                    );
                }
                let fall_x = r.x as f32 + 1.5;
                self.part_box(
                    id,
                    Vec2::new(fall_x, r.z as f32 + 0.35),
                    0.0,
                    Vec3::new(2.6, 2.6, 0.25),
                    colors::FOAM,
                );
                for k in 0..6 {
                    let p = Vec2::new(
                        fall_x - 1.2 + hash01(k, 1) * 2.4,
                        r.z as f32 - 0.4 - hash01(2, k) * 1.4,
                    );
                    let hgt = 0.1 + 0.006 * k as f32; // distinct tops (ARCH-005)
                    self.part_box(id, p, 0.0, Vec3::new(0.5, hgt, 0.4), colors::FOAM);
                }
            }
            "stream" => {
                // pebbles and reeds on the banks (no bridge, no ducks, no lilies: riddle guards)
                for k in 0..(r.d / 3) {
                    let z = r.z as f32 + 1.5 + k as f32 * 3.0;
                    let side = if k % 2 == 0 { 0.15 } else { w - 0.15 };
                    self.model_at(
                        "reed",
                        Vec2::new(r.x as f32 + side, z),
                        hash01(k, r.x) * 6.0,
                    );
                }
            }
            "mill_hut" => {
                self.part_box(id, c, 0.0, Vec3::new(w - 0.6, 2.2, d - 0.6), colors::WOOD);
                self.part_box(id, c, 2.2, Vec3::new(w - 0.2, 0.7, d - 0.2), colors::ROOF);
                // water wheel over the stream west of the hut (not solid): hub, spokes, paddles
                let wheel = Vec2::new(r.x as f32 - 1.5, c.y);
                let axle = 1.3;
                // hub thicker than the spokes, crossing spokes of different section, so no
                // two faces share a plane (ARCH-005)
                self.part_box(
                    id,
                    wheel,
                    axle - 0.17,
                    Vec3::new(1.8, 0.34, 0.34),
                    colors::WOOD,
                );
                self.part_box(
                    id,
                    wheel,
                    0.05,
                    Vec3::new(0.25, 2.5, 0.3),
                    colors::WOOD_LIGHT,
                );
                self.part_box(
                    id,
                    wheel,
                    axle - 0.13,
                    Vec3::new(0.21, 0.26, 2.5),
                    colors::WOOD_LIGHT,
                );
                for k in 0..8 {
                    let a = k as f32 * std::f32::consts::TAU / 8.0;
                    let p = wheel + Vec2::new(0.0, a.cos() * 1.2);
                    self.part_box(
                        id,
                        p,
                        axle + a.sin() * 1.2 - 0.2,
                        Vec3::new(0.5, 0.4, 0.2),
                        colors::WOOD,
                    );
                }
            }
            "willow" => {
                self.part_box(id, c, 0.0, Vec3::new(0.6, 3.0, 0.6), colors::TREE_TRUNK);
                let crown = Vec3::new(w + 1.6, 1.6, d + 1.0);
                self.part_box(id, c + Vec2::new(-0.5, 0.0), 2.6, crown, colors::WILLOW);
                // long hanging branches (curtain) reaching down to the water on the west side
                for k in 0..9 {
                    let z = r.z as f32 - 0.3 + k as f32 * (d + 0.6) / 8.0;
                    let x = r.x as f32 - 1.1 + hash01(k, 7) * 0.4;
                    self.part_box(
                        id,
                        Vec2::new(x, z),
                        0.15 + hash01(k, 2) * 0.3,
                        Vec3::new(0.12, 2.5, 0.12),
                        colors::WILLOW,
                    );
                }
            }
            "pirate_ship" => self.pirate_ship(e, data),
            "carousel" => {
                self.part_box(
                    id,
                    c,
                    0.0,
                    Vec3::new(w - 0.3, 0.35, d - 0.3),
                    colors::WOOD_LIGHT,
                );
                self.part_box(id, c, 0.35, Vec3::new(0.3, 2.4, 0.3), colors::GOLD);
                for k in 0..8 {
                    let a = k as f32 * std::f32::consts::TAU / 8.0;
                    let p = c + Vec2::new(a.cos(), a.sin()) * (w / 2.0 - 0.9);
                    self.part_box(id, p, 0.35, Vec3::new(0.08, 2.3, 0.08), colors::GOLD);
                    if k % 2 == 0 {
                        self.part_box(id, p, 0.6, Vec3::new(0.3, 0.55, 0.8), colors::HORSE);
                    }
                }
                for k in 0..4 {
                    let s = w - 0.2 - k as f32 * 0.9;
                    let col = if k % 2 == 0 {
                        colors::ROOF
                    } else {
                        colors::WHITE
                    };
                    self.part_box(id, c, 2.7 + k as f32 * 0.3, Vec3::new(s, 0.3, s), col);
                }
            }
            "washing_line" => {
                let (x0, x1) = (r.x as f32 + 0.2, (r.x + r.w) as f32 - 0.2);
                for x in [x0, x1] {
                    self.part_box(
                        id,
                        Vec2::new(x, c.y),
                        0.0,
                        Vec3::new(0.12, 2.0, 0.12),
                        colors::WOOD,
                    );
                }
                self.part_box(
                    id,
                    Vec2::new(c.x, c.y),
                    1.9,
                    Vec3::new(x1 - x0, 0.03, 0.03),
                    colors::WHITE,
                );
                let n = (r.w as f32 / 1.2).floor().max(1.0) as i32;
                for k in 0..n {
                    let x = x0 + 0.6 + k as f32 * (x1 - x0 - 1.0) / n as f32;
                    // neighbouring sheets overlap: alternate depth and length (ARCH-005)
                    let odd = (k % 2) as f32;
                    self.part_box(
                        id,
                        Vec2::new(x, c.y + 0.05 * odd),
                        0.8 + 0.04 * odd,
                        Vec3::new(0.95, 1.08 - 0.06 * odd, 0.04),
                        colors::WHITE,
                    );
                }
                self.part_box(
                    id,
                    Vec2::new(x0 + 0.4, c.y - 0.2),
                    0.0,
                    Vec3::new(0.5, 0.3, 0.35),
                    colors::WOOD_LIGHT,
                );
            }
            "treehouse" | "giant_tree" | "blossom_tree" => self.perch_tree(e, data),
            "log_pile" => {
                // stacked rows (each shorter than the one below, logs 1 cm apart) so no two
                // log faces share a plane (ARCH-005)
                let lz = ((d - 0.6) / 4.0).min(0.43);
                for (row, n) in [(0, 4), (1, 3), (2, 2)] {
                    for k in 0..n {
                        let z = r.z as f32 + 0.3 + lz / 2.0 + (k as f32 + row as f32 * 0.5) * lz;
                        self.part_box(
                            id,
                            Vec2::new(c.x, z),
                            row as f32 * (lz - 0.01),
                            Vec3::new(w - 0.3 - 0.12 * row as f32, lz - 0.01, lz - 0.01),
                            colors::TREE_TRUNK,
                        );
                    }
                }
                for k in 0..6 {
                    let p = Vec2::new(
                        r.x as f32 + hash01(k, 5) * w,
                        r.z as f32 - 0.5 + hash01(5, k) * 0.3,
                    );
                    let hgt = 0.06 + 0.005 * k as f32;
                    self.part_box(id, p, 0.0, Vec3::new(0.5, hgt, 0.4), colors::SAWDUST);
                }
            }
            "play_ball" => {
                for (k, (y, s)) in [(0.0, 1.2), (0.3, 1.7), (0.75, 1.8), (1.2, 1.5), (1.6, 0.9)]
                    .into_iter()
                    .enumerate()
                {
                    let col = if k % 2 == 0 {
                        colors::ROOF
                    } else {
                        colors::WHITE
                    };
                    self.part_box(
                        id,
                        c,
                        y,
                        Vec3::new(s, if k == 4 { 0.2 } else { 0.45 }, s),
                        col,
                    );
                }
            }
            "lookout_tower" => {
                for (dx, dz) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                    let p = c + Vec2::new(dx * (w / 2.0 - 0.3), dz * (d / 2.0 - 0.3));
                    self.part_box(id, p, 0.0, Vec3::new(0.25, 5.4, 0.25), colors::WOOD);
                }
                self.part_box(
                    id,
                    c,
                    3.8,
                    Vec3::new(w - 0.2, 0.2, d - 0.2),
                    colors::WOOD_LIGHT,
                );
                self.part_box(id, c, 4.0, Vec3::new(w - 0.2, 0.9, 0.08), colors::WOOD);
                self.part_box(id, c, 5.4, Vec3::new(w + 0.2, 0.5, d + 0.2), colors::ROOF);
                for k in 0..6 {
                    let x = r.x as f32 + 0.4 + k as f32 * 0.4;
                    self.part_box(
                        id,
                        Vec2::new(x, r.z as f32 + 0.5),
                        0.0,
                        Vec3::new(0.35, 0.6 * (k + 1) as f32, 0.7),
                        colors::WOOD_LIGHT,
                    );
                }
            }
            "zoo_train" => {
                self.part_box(id, c, 0.0, Vec3::new(w, 0.1, 1.2), colors::ROCK);
                let x0 = r.x as f32 + 1.4;
                self.part_box(
                    id,
                    Vec2::new(x0, c.y),
                    0.1,
                    Vec3::new(2.4, 1.3, 1.3),
                    colors::ROOF,
                );
                self.part_box(
                    id,
                    Vec2::new(x0 - 0.4, c.y),
                    1.4,
                    Vec3::new(1.2, 0.9, 1.2),
                    colors::ROOF,
                );
                self.part_box(
                    id,
                    Vec2::new(x0 + 0.8, c.y),
                    1.4,
                    Vec3::new(0.35, 0.9, 0.35),
                    colors::BLACK,
                );
                self.part_box(
                    id,
                    Vec2::new(x0 + 0.2, c.y),
                    1.4,
                    Vec3::new(0.25, 0.3, 0.25),
                    colors::GOLD,
                );
                for k in 0..2 {
                    let x = x0 + 2.6 + k as f32 * 2.3;
                    let col = if k == 0 {
                        colors::WAGON_BLUE
                    } else {
                        colors::HEDGE
                    };
                    self.part_box(id, Vec2::new(x, c.y), 0.2, Vec3::new(2.0, 0.8, 1.2), col);
                }
            }
            "slide" => {
                self.part_box(
                    id,
                    Vec2::new(c.x, r.z as f32 + 0.5),
                    0.0,
                    Vec3::new(1.2, 1.8, 0.8),
                    colors::WAGON_BLUE,
                );
                for k in 0..4 {
                    let z = r.z as f32 + 1.1 + k as f32 * 0.45;
                    self.part_box(
                        id,
                        Vec2::new(c.x, z),
                        0.0,
                        Vec3::new(0.7, 1.6 - k as f32 * 0.4, 0.45),
                        colors::ROOF,
                    );
                }
            }
            "swings" => {
                for x in [r.x as f32 + 0.3, (r.x + r.w) as f32 - 0.3] {
                    self.part_box(
                        id,
                        Vec2::new(x, c.y),
                        0.0,
                        Vec3::new(0.15, 2.3, 1.2),
                        colors::ROOF,
                    );
                }
                self.part_box(id, c, 2.2, Vec3::new(w - 0.4, 0.12, 0.12), colors::ROOF);
                for dx in [-0.8, 0.8] {
                    self.part_box(
                        id,
                        c + Vec2::X * dx,
                        0.5,
                        Vec3::new(0.5, 0.06, 0.25),
                        colors::GOLD,
                    );
                    self.part_box(
                        id,
                        c + Vec2::X * dx,
                        0.55,
                        Vec3::new(0.03, 1.65, 0.03),
                        colors::ROCK,
                    );
                }
            }
            "stage" => {
                self.part_box(
                    id,
                    c,
                    0.0,
                    Vec3::new(w - 0.2, 0.5, d - 0.2),
                    colors::WOOD_LIGHT,
                );
                for (dx, dz) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                    let p = c + Vec2::new(dx * (w / 2.0 - 0.4), dz * (d / 2.0 - 0.4));
                    self.part_box(id, p, 0.5, Vec3::new(0.15, 2.3, 0.15), colors::WOOD);
                }
                for k in 0..4 {
                    let s = w - k as f32 * 1.0;
                    self.part_box(
                        id,
                        c,
                        2.8 + k as f32 * 0.35,
                        Vec3::new(s.max(0.4), 0.35, s.max(0.4)),
                        colors::ROOF,
                    );
                }
                self.part_box(
                    id,
                    c + Vec2::new(-0.6, 0.3),
                    0.5,
                    Vec3::new(0.5, 0.5, 0.5),
                    colors::ROOF,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.1, 0.5),
                    0.5,
                    Vec3::new(0.4, 0.4, 0.4),
                    colors::WAGON_BLUE,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.5, -0.5),
                    0.5,
                    Vec3::new(1.0, 0.6, 0.35),
                    colors::GOLD,
                );
            }
            "deckchairs" => {
                for k in 0..3 {
                    let x = r.x as f32 + 0.6 + k as f32 * 1.1;
                    for j in 0..3 {
                        let col = if j % 2 == 0 {
                            colors::WAGON_BLUE
                        } else {
                            colors::WHITE
                        };
                        self.part_box(
                            id,
                            Vec2::new(x - 0.2 + j as f32 * 0.2, c.y - 0.1),
                            0.3,
                            Vec3::new(0.2, 0.08, 1.1),
                            col,
                        );
                    }
                    self.part_box(
                        id,
                        Vec2::new(x, c.y + 0.5),
                        0.3,
                        Vec3::new(0.6, 0.7, 0.08),
                        colors::WAGON_BLUE,
                    );
                }
                let sh = Vec2::new((r.x + r.w) as f32 - 0.5, c.y);
                self.part_box(id, sh, 0.0, Vec3::new(0.08, 2.3, 0.08), colors::WHITE);
                self.part_box(id, sh, 2.2, Vec3::new(2.6, 0.2, 2.2), colors::ROOF);
                self.part_box(id, sh, 2.4, Vec3::new(1.6, 0.15, 1.4), colors::WHITE);
            }
            _ => {}
        }
    }

    /// Tree house oak, giant tree and blossom tree (level 2): trunk, crown and — for the
    /// perch — a branch / porch under the perch point (proposal Q-094).
    fn perch_tree(&mut self, e: &Element, data: &LevelData) {
        let r = e.rect;
        let c = rect_center(r);
        let id = e.id.as_str();
        let (w, d) = (r.w as f32, r.d as f32);
        let perch = data
            .hiding_places
            .iter()
            .find(|h| h.scenery.iter().any(|s| s == &e.id) && h.perch_height_m.is_some());
        let perch_h = perch.and_then(|h| h.perch_height_m).unwrap_or(3.0);
        match e.kind.as_deref() {
            Some("treehouse") => {
                self.part_box(id, c, 0.0, Vec3::new(0.8, 6.0, 0.8), colors::TREE_TRUNK);
                self.part_box(
                    id,
                    c,
                    perch_h - 0.2,
                    Vec3::new(w - 0.3, 0.2, d - 0.3),
                    colors::WOOD,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.3, 0.3),
                    perch_h,
                    Vec3::new(1.7, 1.5, 1.7),
                    colors::WOOD_LIGHT,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.3, 0.3),
                    perch_h + 1.5,
                    Vec3::new(2.1, 0.6, 2.1),
                    colors::ROOF,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.3, -0.56),
                    perch_h + 0.5,
                    Vec3::new(0.5, 0.5, 0.06),
                    colors::BLACK,
                );
                // rope ladder on the south side
                self.part_box(
                    id,
                    Vec2::new(c.x - 0.6, r.z as f32 + 0.12),
                    0.0,
                    Vec3::new(0.5, perch_h, 0.05),
                    colors::ROPE,
                );
                self.part_box(
                    id,
                    c + Vec2::new(0.4, 0.5),
                    perch_h + 2.1,
                    Vec3::new(w + 1.4, 2.2, d + 1.0),
                    colors::TREE_CROWN,
                );
            }
            Some("giant_tree") => {
                // 12 m, twice as tall as any other tree (riddle guard); the crown parts above
                // and below the perch leave the koalas visible on their branch
                self.part_box(
                    id,
                    c,
                    0.0,
                    Vec3::new(w - 0.2, 12.0, d - 0.2),
                    colors::TREE_TRUNK,
                );
                self.part_box(
                    id,
                    c + Vec2::X * 0.6,
                    5.0,
                    Vec3::new(w + 1.8, 3.0, d + 2.6),
                    colors::TREE_CROWN,
                );
                self.part_box(
                    id,
                    c + Vec2::X * 0.4,
                    perch_h + 1.3,
                    Vec3::new(w + 1.2, 3.2, d + 1.6),
                    colors::TREE_CROWN,
                );
            }
            _ => {
                // blossom tree: pink crown, falling petals, bees
                self.part_box(
                    id,
                    c,
                    0.0,
                    Vec3::new(0.5, perch_h + 1.2, 0.5),
                    colors::TREE_TRUNK,
                );
                self.part_box(
                    id,
                    c + Vec2::X * 0.4,
                    perch_h + 0.9,
                    Vec3::new(w + 1.4, 2.3, d + 1.6),
                    colors::BLOSSOM,
                );
                for k in 0..4 {
                    let a = k as f32 * 1.7;
                    let p = c + Vec2::new(a.cos(), a.sin()) * 1.9;
                    self.part_box(
                        id,
                        p,
                        perch_h + 1.2 + hash01(k, 4),
                        Vec3::new(0.14, 0.1, 0.1),
                        colors::GOLD,
                    );
                }
            }
        }
    }

    /// Branch / platform under a perch point (proposal Q-094), reaching back to its tree.
    fn perch_branch(&mut self, h: &crate::level::HidingPlaceData, data: &LevelData, height: f32) {
        let p = perch_point(h, data);
        let Some(target) = perch_scenery(h, data) else {
            return;
        };
        let q = rect_center(target.rect);
        let mid = (p + q) / 2.0;
        let len = p.distance(q) + 0.37;
        let along_x = (q - p).x.abs() >= (q - p).y.abs();
        let size = if along_x {
            Vec3::new(len, 0.18, 0.5)
        } else {
            Vec3::new(0.5, 0.18, len)
        };
        let color = if target.kind.as_deref() == Some("pirate_ship") {
            colors::WOOD_LIGHT
        } else {
            colors::TREE_TRUNK
        };
        // 2 cm proud of the floor / deck it may lie on (no coplanar tops, ARCH-005)
        self.push_box(&format!("{}:perch", h.id), mid, height - 0.16, size, color);
    }

    /// Pirate ship climbing frame (Q-017 proposal): hull, deck, mast with the crow's nest at
    /// the perch point, sail, black flag with a white paw, treasure chest, rope ladder. No
    /// slide, no skull (riddle guards).
    fn pirate_ship(&mut self, e: &Element, data: &LevelData) {
        let r = e.rect;
        let c = rect_center(r);
        let id = e.id.as_str();
        let (w, d) = (r.w as f32, r.d as f32);
        self.part_box(
            id,
            c,
            0.0,
            Vec3::new(w - 0.2, 1.4, d - 0.6),
            colors::TREE_TRUNK,
        );
        self.part_box(
            id,
            c + Vec2::X * (w / 2.0 - 0.4),
            0.3,
            Vec3::new(0.8, 1.5, d - 1.2),
            colors::TREE_TRUNK,
        );
        self.part_box(
            id,
            c,
            1.4,
            Vec3::new(w - 0.4, 0.12, d - 0.8),
            colors::WOOD_LIGHT,
        );
        let place = data
            .hiding_places
            .iter()
            .find(|h| h.scenery.iter().any(|s| s == &e.id) && h.perch_height_m.is_some());
        let nest_h = place.and_then(|h| h.perch_height_m).unwrap_or(4.0);
        let nest = place.map_or(c, |h| perch_point(h, data));
        // the mast 0.3 m from the perch towards the ship: the monkey's `climb` pose grips a
        // pole 0.17 m in front of it (animal_anims.toml)
        let mast = nest + (c - nest).normalize_or_zero() * 0.3;
        self.part_box(
            id,
            mast,
            0.0,
            Vec3::new(0.22, nest_h + 2.2, 0.22),
            colors::WOOD,
        );
        self.part_box(
            id,
            nest,
            nest_h - 0.15,
            Vec3::new(1.1, 0.15, 1.1),
            colors::WOOD,
        );
        self.part_box(id, nest, nest_h, Vec3::new(1.1, 0.35, 0.06), colors::WOOD);
        self.part_box(
            id,
            Vec2::new(mast.x + 1.4, mast.y),
            1.6,
            Vec3::new(2.4, 2.2, 0.06),
            colors::WHITE,
        );
        self.part_box(
            id,
            Vec2::new(mast.x + 0.5, mast.y),
            nest_h + 1.5,
            Vec3::new(0.9, 0.6, 0.04),
            colors::BLACK,
        );
        self.part_box(
            id,
            Vec2::new(mast.x + 0.5, mast.y - 0.03),
            nest_h + 1.7,
            Vec3::new(0.25, 0.22, 0.02),
            colors::WHITE,
        );
        self.part_box(
            id,
            c + Vec2::new(1.5, 0.4),
            1.52,
            Vec3::new(0.7, 0.45, 0.45),
            colors::GOLD,
        );
        self.part_box(
            id,
            Vec2::new(r.x as f32 + 0.15, c.y - 0.9),
            0.0,
            Vec3::new(0.05, nest_h, 0.5),
            colors::ROPE,
        );
    }

    /// Ice cream kiosk: a smaller building box at the back, striped awning, cone icon, and
    /// the freezer chest with its cold mist in front (inside the element rect).
    fn kiosk(&mut self, e: &Element) {
        let r = e.rect;
        let c = rect_center(r);
        let id = e.id.as_str();
        let (w, d) = (r.w as f32, r.d as f32);
        let back = Vec2::new(c.x, c.y + 0.5);
        self.part_box(
            id,
            back,
            0.0,
            Vec3::new(w - 0.2, 2.6, d - 1.1),
            colors::BUILDING,
        );
        for k in 0..4 {
            let col = if k % 2 == 0 {
                colors::ROOF
            } else {
                colors::WHITE
            };
            let x = r.x as f32 + 0.5 + k as f32 * (w - 1.0) / 3.0;
            self.part_box(
                id,
                Vec2::new(x, r.z as f32 + 0.9),
                2.3,
                Vec3::new((w - 0.2) / 4.0, 0.12, 1.0),
                col,
            );
        }
        self.part_box(id, back, 2.6, Vec3::new(0.5, 0.9, 0.5), colors::GOLD);
        self.part_box(id, back, 3.5, Vec3::new(0.7, 0.5, 0.7), colors::BLOSSOM);
        let freezer = Vec2::new(c.x, r.z as f32 + 0.45);
        self.part_box(id, freezer, 0.0, Vec3::new(1.5, 0.85, 0.7), colors::WHITE);
        self.part_box(
            id,
            freezer,
            0.85,
            Vec3::new(1.4, 0.05, 0.6),
            colors::WATER_LIGHT,
        );
        self.part_box(id, freezer, 0.9, Vec3::new(1.0, 0.25, 0.4), colors::FOAM);
    }

    /// Enterable building (Q-092): walls around the interior with the door gap, the upper
    /// walls and the roof in [`LevelScene::roof_boxes`] (hidden while inside, PLAY-028),
    /// shelves and a bed inside.
    fn enterable_building(&mut self, e: &Element) {
        let r = e.rect;
        let id = e.id.as_str();
        let height = e.height_m.unwrap_or(4.0);
        let wall_h = height * 0.7;
        let low = 1.0;
        let inner = e.interior.expect("enterable");
        let door = e.door_cell();
        let (x0, z0) = (r.x as f32 + 0.05, r.z as f32 + 0.05);
        let (x1, z1) = ((r.x + r.w) as f32 - 0.05, (r.z + r.d) as f32 - 0.05);
        let (ix0, iz0) = (inner.x as f32, inner.z as f32);
        let (ix1, iz1) = ((inner.x + inner.w) as f32, (inner.z + inner.d) as f32);
        // wall slabs between the outer rect and the interior, split around the door
        let mut slabs: Vec<(Vec2, Vec2)> = vec![
            (Vec2::new(x0, iz1), Vec2::new(x1, z1)),  // north
            (Vec2::new(x0, z0), Vec2::new(ix0, iz1)), // west
            (Vec2::new(ix1, z0), Vec2::new(x1, iz1)), // east
        ];
        match door {
            Some(dc) if dc.y < inner.z => {
                slabs.push((Vec2::new(ix0, z0), Vec2::new(dc.x as f32, iz0)));
                slabs.push((Vec2::new(dc.x as f32 + 1.0, z0), Vec2::new(ix1, iz0)));
            }
            _ => slabs.push((Vec2::new(ix0, z0), Vec2::new(ix1, iz0))),
        }
        for (a, b) in &slabs {
            let size = Vec3::new(b.x - a.x, low, b.y - a.y);
            self.push_box(id, (*a + *b) / 2.0, 0.0, size, colors::BUILDING);
        }
        let first = self.boxes.len();
        for (a, b) in &slabs {
            let size = Vec3::new(b.x - a.x, wall_h - low, b.y - a.y);
            self.push_box(id, (*a + *b) / 2.0, low, size, colors::BUILDING);
        }
        if let Some(dc) = door.filter(|dc| dc.y < inner.z) {
            // lintel over the door
            let p = Vec2::new(dc.x as f32 + 0.5, (z0 + iz0) / 2.0);
            self.push_box(
                id,
                p,
                2.2,
                Vec3::new(1.0, wall_h - 2.2, iz0 - z0),
                colors::BUILDING,
            );
        }
        let roof = Vec3::new(r.w as f32 + 0.3, height * 0.3, r.d as f32 + 0.3);
        self.push_box(id, rect_center(r), wall_h, roof, colors::ROOF);
        for b in &mut self.boxes[first..] {
            b.fadeable = false;
        }
        self.roof_boxes
            .push((e.id.clone(), first..self.boxes.len()));
        // furniture along the walls (not on the walkable cells in front of the door)
        self.push_box(
            id,
            Vec2::new(ix0 + 0.25, (iz0 + iz1) / 2.0),
            0.0,
            Vec3::new(0.4, 1.6, inner.d as f32 - 0.6),
            colors::WOOD,
        );
        self.push_box(
            id,
            Vec2::new(ix1 - 0.55, iz1 - 0.5),
            0.0,
            Vec3::new(0.95, 0.5, 0.9),
            colors::WAGON_BLUE,
        );
    }

    /// Construction fence (level-2 exit): striped panels on the walkable side, a sign with
    /// a digger icon, a small yellow digger behind (all removed when it opens).
    fn construction_fence(&mut self, e: &Element, grid: &Grid) {
        let r = e.rect;
        let id = e.id.as_str();
        let row = walkable_row_center(r, grid);
        let c = rect_center(r);
        let dir = row - c;
        let along_z = dir.x.abs() > dir.y.abs() || (dir == Vec2::ZERO && r.d > r.w);
        let n = if along_z { r.d } else { r.w };
        for k in 0..n * 2 {
            let t = k as f32 * 0.5 + 0.25;
            let p = if along_z {
                Vec2::new(row.x + dir.x.signum() * 0.2, r.z as f32 + t)
            } else {
                Vec2::new(r.x as f32 + t, row.y + dir.y.signum() * 0.2)
            };
            let col = if k % 2 == 0 {
                colors::BARRIER
            } else {
                colors::WHITE
            };
            let size = if along_z {
                Vec3::new(0.08, 1.2, 0.5)
            } else {
                Vec3::new(0.5, 1.2, 0.08)
            };
            self.push_box(id, p, 0.0, size, col);
        }
        let back = c - dir.normalize_or_zero() * 0.3;
        self.push_box(id, back, 0.0, Vec3::new(0.9, 0.8, 1.2), colors::DIGGER);
        self.push_box(id, back, 0.8, Vec3::new(0.7, 0.7, 0.7), colors::DIGGER);
        self.push_box(
            id,
            back + Vec2::Y * 0.7,
            0.6,
            Vec3::new(0.15, 0.15, 1.0),
            colors::DIGGER,
        );
        self.push_box(
            id,
            row + Vec2::Y * 0.9,
            1.2,
            Vec3::new(0.08, 0.5, 0.6),
            colors::WHITE,
        );
    }

    fn enclosure(&mut self, e: &Element) {
        self.enclosure_dressing(e);
        let fence = match enclosure_fence(e.rect, e.gate) {
            Ok(f) => f,
            Err(_) => {
                self.rect_box(e, 0.0, 1.1, colors::WOOD);
                return;
            }
        };
        for (corner, k) in fence.corners.iter().zip(CORNER_TURNS) {
            self.placements.push(Placement::new(
                "fence_wood_corner",
                level_to_world(*corner),
                quarter_turns_cw_to_yaw(k),
            ));
        }
        for run in &fence.runs {
            self.run_pieces(run, "fence_wood", "fence_wood_1m");
        }
        if let Some(gate) = fence.gate {
            let dir = gate.axis.dir();
            self.placements.push(Placement::new(
                "gate_wood",
                level_to_world(gate.start + dir * 0.09),
                run_yaw(gate.axis),
            ));
            // Enclosure sign just outside the gate, reading outwards.
            let mid = gate.start + dir * 1.0;
            let out = dir_away(e.rect, mid + (mid - rect_center(e.rect)).normalize() * 0.01);
            let pos = mid + out.offset().as_vec2() * 0.35;
            let yaw = facing_yaw(out);
            self.placements
                .push(Placement::new("enclosure_sign", level_to_world(pos), yaw));
            // Silhouette of the enclosure's animal on the sign panel (ART-ENVIRONMENT 6).
            if let Some(animal) = &e.animal {
                self.decals
                    .push(sign_silhouette(&e.id, animal, level_to_world(pos), yaw));
            }
        }
    }
}

/// Silhouette decal on the `sign_panel` face of an `enclosure_sign` placed at `origin`
/// (world) with `yaw` (front = local +Z, Q-061).
pub fn sign_silhouette(enclosure: &str, animal: &str, origin: Vec3, yaw: f32) -> Decal {
    let rot = Quat::from_rotation_y(yaw);
    let n = SIGN_PANEL_NORMAL.normalize();
    let up_dir = Vec3::new(0.0, n.z, -n.y); // in the panel plane, towards its top edge
    Decal {
        id: format!("sign:{enclosure}"),
        image: DecalImage::Texture(silhouette_path(animal)),
        center: origin + rot * (SIGN_PANEL_CENTER + n * DECAL_LIFT_M),
        right: rot * (Vec3::X * SILHOUETTE_SIZE.x / 2.0),
        up: rot * (up_dir * SILHOUETTE_SIZE.y / 2.0),
    }
}

/// Nominal bounding box of a model, used for its placeholder when the `.glb` is missing:
/// `(centre offset of the bottom face in model space, size, colour)`.
pub fn model_placeholder(model: &str) -> (Vec3, Vec3, [f32; 3]) {
    let tile = (Vec3::ZERO, Vec3::new(1.0, 0.06, 1.0), colors::PATH);
    match model {
        "grass_tile" | "sand_tile" => (Vec3::ZERO, Vec3::new(1.0, 0.05, 1.0), colors::GRASS),
        m if m.starts_with("path_tile") || m == "plaza_tile" => tile,
        "path_edge" => (Vec3::ZERO, Vec3::new(1.0, 0.1, 0.12), colors::STONE),
        "fence_wood" => (Vec3::ZERO, Vec3::new(2.0, 1.1, 0.18), colors::WOOD),
        "fence_wood_1m" => (Vec3::ZERO, Vec3::new(1.0, 1.1, 0.18), colors::WOOD),
        "fence_wood_corner" => (Vec3::ZERO, Vec3::new(0.2, 1.1, 0.2), colors::WOOD),
        "gate_wood" => (
            Vec3::new(0.9, 0.0, 0.0),
            Vec3::new(1.8, 1.05, 0.1),
            colors::WOOD_LIGHT,
        ),
        "hedge" => (Vec3::ZERO, Vec3::new(2.0, 3.0, 1.0), colors::HEDGE),
        "hedge_1m" => (Vec3::ZERO, Vec3::new(1.0, 3.0, 1.0), colors::HEDGE),
        "hedge_corner" => (Vec3::ZERO, Vec3::new(1.0, 3.0, 1.0), colors::HEDGE),
        "zoo_wall" => (Vec3::ZERO, Vec3::new(2.0, 2.5, 0.7), colors::STONE),
        "zoo_wall_1m" => (Vec3::ZERO, Vec3::new(1.0, 2.5, 0.7), colors::STONE),
        "zoo_wall_corner" => (Vec3::ZERO, Vec3::new(0.8, 2.5, 0.8), colors::STONE),
        "info_board" => (Vec3::ZERO, Vec3::new(0.8, 1.4, 0.25), colors::WOOD_LIGHT),
        "map_board" => (Vec3::ZERO, Vec3::new(2.1, 2.2, 0.3), colors::WOOD_LIGHT),
        "enclosure_sign" => (Vec3::ZERO, Vec3::new(2.36, 2.2, 0.25), colors::WOOD_LIGHT),
        // placeholder props of levels 2-3 (no model yet)
        "table_wood" => (Vec3::ZERO, Vec3::new(0.9, 0.75, 0.7), colors::WOOD),
        "water_tap" => (Vec3::ZERO, Vec3::new(0.3, 1.0, 0.3), colors::ROCK),
        _ => (Vec3::ZERO, Vec3::ONE, colors::DEFAULT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::world_to_level;

    fn level1() -> LevelData {
        let s = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        LevelData::from_toml_str(&s).unwrap()
    }

    #[test]
    fn path_tile_mapping() {
        assert_eq!(
            path_tile(&[Dir::N, Dir::E, Dir::S, Dir::W]).0,
            "path_tile_cross"
        );
        assert_eq!(path_tile(&[Dir::N, Dir::E, Dir::S]), ("path_tile_t", 0));
        // Grass on the north side: canonical (grass W) turned 1 step clockwise.
        assert_eq!(path_tile(&[Dir::E, Dir::S, Dir::W]), ("path_tile_t", 1));
        assert_eq!(path_tile(&[Dir::E, Dir::W]), ("path_tile_straight", 1));
        assert_eq!(path_tile(&[Dir::S, Dir::W]), ("path_tile_curve", 1));
        assert_eq!(path_tile(&[Dir::N]), ("path_tile_end", 2));
    }

    #[test]
    fn water_tiles_cover_river_pond_and_bridge() {
        let data = level1();
        let s = LevelScene::build(&data);
        let water = s
            .placements
            .iter()
            .filter(|p| p.model.starts_with("water_"))
            .count();
        // river_n 3x17 + river_mid 3x1 + river_e 14x3 + bridge 3x3 + pond 8x8
        // + hippo_pool 8x7 without its 3 ramp cells (GAME-LEVEL-1 "Hippo enclosure pool")
        assert_eq!(water, 51 + 3 + 42 + 9 + 64 + 53);
        assert_eq!(
            water_tile(false, &[Dir::E, Dir::S], &[], false),
            ("water_pond_corner", 0)
        );
        let all = [Dir::N, Dir::E, Dir::S, Dir::W];
        assert_eq!(
            water_tile(true, &all, &[Dir::E], false),
            ("water_river_inner", 1)
        );
        assert_eq!(
            water_tile(true, &all, &[], true),
            ("water_river_straight", 1)
        );
    }

    #[test]
    fn one_ground_tile_per_dry_cell() {
        let data = level1();
        let s = LevelScene::build(&data);
        let tiles = s
            .placements
            .iter()
            .filter(|p| p.model.ends_with("_tile") || p.model.starts_with("path_tile"))
            .count();
        let b = data.level.bounds;
        assert!(tiles > 2000 && tiles < (b.w * b.d) as usize, "{tiles}");
        // The spawn cell is a plaza tile, not mirrored: world x = level x, world z = -level z.
        let spawn = cell_center(data.spawn.cell());
        let t = s
            .placements
            .iter()
            .find(|p| world_to_level(p.pos).distance(spawn) < 1e-4)
            .unwrap();
        assert_eq!(t.model, "plaza_tile");
        assert!((t.pos.z + 2.5).abs() < 1e-5);
    }

    #[test]
    fn east_is_on_the_right_when_looking_north() {
        // The bench (east of the spawn) is at larger world X; the food storage (north) at
        // smaller world Z (GAME-LAYOUT "Coordinate spaces").
        let s = LevelScene::build(&level1());
        let bench = s.boxes.iter().find(|b| b.source == "bench_plaza").unwrap();
        let storage = s.boxes.iter().find(|b| b.source == "food_storage").unwrap();
        assert!(bench.pos.x > 0.5);
        assert!(storage.pos.z < -10.0);
    }

    #[test]
    fn enclosures_have_fences_and_gates() {
        let s = LevelScene::build(&level1());
        let count = |m: &str| s.placements.iter().filter(|p| p.model == m).count();
        assert_eq!(count("gate_wood"), 3);
        assert_eq!(count("fence_wood_corner"), 12);
        assert!(count("fence_wood") > 20);
        assert!(count("hedge") > 20);
        assert!(count("zoo_wall") > 20);
    }

    // AENV-011 (scene part): the zebra enclosure sign carries the zebra silhouette on its
    // panel, facing out of the enclosure towards the path and up to the camera.
    #[test]
    fn aenv_011_enclosure_signs_get_their_silhouette() {
        let data = level1();
        let s = LevelScene::build(&data);
        let d = s.decals.iter().find(|d| d.id == "sign:enc_zebra").unwrap();
        assert_eq!(
            d.image,
            DecalImage::Texture("textures/signs/silhouette_zebra.png".into())
        );
        let sign = s
            .placements
            .iter()
            .filter(|p| p.model == "enclosure_sign")
            .min_by(|a, b| {
                a.pos
                    .distance(d.center)
                    .total_cmp(&b.pos.distance(d.center))
            })
            .unwrap();
        assert!(sign.pos.distance(d.center) < 1.6, "decal far from its sign");
        let n = d.normal();
        // zebra sign faces east (+X world) and is tilted back 40°
        assert!(n.x > 0.7 && n.y > 0.6 && n.z.abs() < 1e-4, "normal {n}");
        // the decal lies on the panel plane (lifted 4 mm) and stays inside the panel
        let front = Quat::from_rotation_y(sign.yaw) * SIGN_PANEL_NORMAL.normalize();
        let panel = sign.pos + Quat::from_rotation_y(sign.yaw) * SIGN_PANEL_CENTER;
        assert!(((d.center - panel).dot(front) - DECAL_LIFT_M).abs() < 1e-4);
        assert!(d.right.length() * 2.0 <= 1.30 && d.up.length() * 2.0 <= 0.72);
        // top of the image is the upper edge of the tilted panel
        assert!(d.up.y > 0.0);
        // one silhouette decal per enclosure with an animal
        let enclosures = data
            .elements
            .iter()
            .filter(|e| e.ty == ElementType::Enclosure && e.animal.is_some())
            .count();
        let signs = s
            .decals
            .iter()
            .filter(|d| matches!(d.image, DecalImage::Texture(_)))
            .count();
        assert_eq!(signs, enclosures);
    }

    // AENV-012 (scene part): the "Futter" sign hangs on the south facade of the food storage
    // above the row of food boxes and faces south (the path and the default camera).
    #[test]
    fn aenv_012_food_storage_sign_above_the_boxes() {
        let data = level1();
        let s = LevelScene::build(&data);
        let d = s
            .decals
            .iter()
            .find(|d| d.id == "sign:food_storage")
            .unwrap();
        assert!(matches!(
            d.image,
            DecalImage::Text {
                key: FOOD_STORAGE_SIGN_KEY,
                ..
            }
        ));
        let n = d.normal();
        assert!((n - Vec3::Z).length() < 1e-5, "faces south: {n}");
        let [tl, _, br, _] = d.corners();
        let boxes: Vec<Vec2> = data.food_boxes.iter().map(|b| b.pos()).collect();
        let (min_x, max_x) = boxes
            .iter()
            .fold((f32::MAX, f32::MIN), |(a, b), p| (a.min(p.x), b.max(p.x)));
        assert!(
            tl.x >= min_x - 0.5 && br.x <= max_x + 0.5,
            "over the box row"
        );
        assert!(br.y > 1.2, "above the food boxes: {br}");
        assert!(tl.y < 3.15, "below the roof: {tl}");
        // just in front of the facade (level z = 11.05 → world z = -11.05)
        let board = s
            .boxes
            .iter()
            .find(|b| b.source == "food_storage:sign")
            .unwrap();
        assert!((board.pos.z + board.size.z / 2.0 - (-11.05 + board.size.z)).abs() < 1e-4);
        assert!((d.center.z - (board.pos.z + board.size.z / 2.0 + DECAL_LIFT_M)).abs() < 1e-4);
        // texture aspect matches the quad
        if let DecalImage::Text {
            width_px,
            height_px,
            ..
        } = d.image
        {
            let q = d.right.length() / d.up.length();
            assert!((width_px as f32 / height_px as f32 - q).abs() < 0.02);
        }
    }
}
