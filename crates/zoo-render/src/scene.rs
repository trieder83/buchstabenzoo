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

use glam::{IVec2, Vec2, Vec3};
use zoo_core::coords::{level_to_world, level_to_world_at, quarter_turns_cw_to_yaw};
use zoo_core::level::{band_run, cell_center, enclosure_fence, Element, Grid, Run, RunAxis};
use zoo_core::{ElementType, LevelData, Rect};

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
}

/// A model instance: `pos` is the model origin in world space, `yaw` the rotation about +Y
/// (positive = counter-clockwise seen from above).
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub model: &'static str,
    pub pos: Vec3,
    pub yaw: f32,
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
}

/// The assembled static scene of a level.
#[derive(Debug, Clone, Default)]
pub struct LevelScene {
    pub placements: Vec<Placement>,
    pub boxes: Vec<BoxPlacement>,
}

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
                (ElementType::Landmark, Some("river")) | (ElementType::Path, Some("bridge")) => 1,
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
        // Water mask: the river continues beyond the level edge (under the hedges).
        let water_at = |c: IVec2, body: u8| {
            if b.contains(c) {
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
        for c in b.cells() {
            let i = idx(c);
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
                s.placements.push(Placement {
                    model,
                    pos,
                    yaw: quarter_turns_cw_to_yaw(k),
                });
                continue;
            }
            if plaza[i] {
                s.placements.push(Placement {
                    model: "plaza_tile",
                    pos,
                    yaw: 0.0,
                });
            } else if path_at(c) && !jetty(c) {
                let conn: Vec<Dir> = Dir::ALL
                    .into_iter()
                    .filter(|d| path_at(c + d.offset()))
                    .collect();
                let (model, k) = path_tile(&conn);
                s.placements.push(Placement {
                    model,
                    pos,
                    yaw: quarter_turns_cw_to_yaw(k),
                });
                // Edging stones on the grass margin of every unconnected side.
                for d in Dir::ALL.into_iter().filter(|d| !conn.contains(d)) {
                    let off = d.offset().as_vec2() * 0.43;
                    let yaw = if matches!(d, Dir::N | Dir::S) {
                        0.0
                    } else {
                        quarter_turns_cw_to_yaw(1)
                    };
                    s.placements.push(Placement {
                        model: "path_edge",
                        pos: level_to_world(cell_center(c) + off),
                        yaw,
                    });
                }
            } else {
                s.placements.push(Placement {
                    model: "grass_tile",
                    pos,
                    yaw: 0.0,
                });
            }
        }

        for e in &data.elements {
            s.add_element(e, data);
        }
        s
    }

    fn push_box(&mut self, source: &str, center: Vec2, y0: f32, size: Vec3, color: [f32; 3]) {
        self.boxes.push(BoxPlacement {
            pos: level_to_world_at(center, y0),
            size,
            yaw: 0.0,
            color,
            fadeable: y0 + size.y > 1.5,
            source: source.to_owned(),
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
            self.placements.push(Placement {
                model: if seg.length_m == 2 { two } else { one },
                pos: level_to_world(run.segment_center(seg)),
                yaw,
            });
        }
    }

    fn model_at(&mut self, model: &'static str, p: Vec2, yaw: f32) {
        self.placements.push(Placement {
            model,
            pos: level_to_world(p),
            yaw,
        });
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

    /// Ducks near the bridge (riddle detail of `loc_river`, GAME-LEVEL-1).
    fn river_dressing(&mut self, e: &Element) {
        if e.id != "river_n" {
            return;
        }
        let r = e.rect;
        for (k, (dx, dz, yaw)) in [(0.8, 1.3, 20.0), (1.9, 2.1, -30.0), (1.2, 3.4, 160.0f32)]
            .into_iter()
            .enumerate()
        {
            let _ = k;
            self.model_at(
                "duck",
                Vec2::new(r.x as f32 + dx, r.z as f32 + dz),
                yaw.to_radians(),
            );
        }
    }

    /// Lily pads, frogs and reeds (riddle details of `loc_pond`).
    fn pond_dressing(&mut self, e: &Element) {
        let r = e.rect;
        let o = Vec2::new(r.x as f32, r.z as f32);
        for (dx, dz, yaw) in [
            (2.2, 2.5, 0.0),
            (5.4, 3.1, 70.0),
            (3.6, 5.6, 150.0),
            (6.0, 6.2, 220.0f32),
        ] {
            self.model_at("lily_pad", o + Vec2::new(dx, dz), yaw.to_radians());
        }
        for (dx, dz, yaw) in [(4.6, 2.0, 0.0), (2.4, 4.6, 30.0f32)] {
            self.model_at("frog", o + Vec2::new(dx, dz), yaw.to_radians());
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
        let list: &[(&'static str, f32, f32)] = match e.animal.as_deref() {
            Some("panda") => &[
                ("bamboo", 0.2, 0.75),
                ("bamboo", 0.75, 0.8),
                ("bamboo", 0.8, 0.35),
                ("bush", 0.3, 0.3),
            ],
            Some("zebra") => &[
                ("bush", 0.25, 0.8),
                ("bush", 0.6, 0.25),
                ("rock", 0.3, 0.4),
                ("grass_tuft", 0.55, 0.6),
            ],
            Some("hippo") => &[("rock", 0.7, 0.3), ("rock", 0.25, 0.75), ("bush", 0.8, 0.8)],
            _ => &[],
        };
        for (i, (m, fx, fz)) in list.iter().enumerate() {
            self.model_at(
                m,
                at(*fx, *fz),
                hash01(i as i32, r.x) * std::f32::consts::TAU,
            );
        }
    }

    fn add_element(&mut self, e: &Element, data: &LevelData) {
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
                let to_spawn = cell_center(data.spawn.cell());
                self.placements.push(Placement {
                    model: "map_board",
                    pos: level_to_world(rect_center(e.rect)),
                    yaw: facing_yaw(dir_away(e.rect, to_spawn)),
                });
            }
            (ElementType::Decoration | ElementType::Boundary, "hedge" | "zoo_wall") => {
                let (two, one) = if kind == "hedge" {
                    ("hedge", "hedge_1m")
                } else {
                    ("zoo_wall", "zoo_wall_1m")
                };
                match band_run(e.rect, data.level.bounds) {
                    Some(run) => self.run_pieces(&run, two, one),
                    None => self.rect_box(e, 0.0, h.unwrap_or(3.0), colors::HEDGE),
                }
            }
            (ElementType::Decoration, "tree_grove") => self.trees(e, "tree_grove", 2.5),
            (ElementType::Decoration, "trees") => self.trees(e, "tree_round", 3.0),
            (ElementType::Decoration, "bench") => self.rect_box(e, 0.0, 0.5, colors::WOOD),
            (ElementType::Decoration, "info_board") => {
                let enc = e
                    .enclosure
                    .as_deref()
                    .and_then(|id| data.element(id))
                    .map(|enc| dir_away(enc.rect, rect_center(e.rect)))
                    .unwrap_or(Dir::S);
                self.placements.push(Placement {
                    model: "info_board",
                    pos: level_to_world(rect_center(e.rect)),
                    yaw: facing_yaw(enc),
                });
            }
            (ElementType::Enclosure, _) => self.enclosure(e),
            (ElementType::Building, "entrance") => {
                // Arch: two pillars and a beam, so the player is visible through it.
                let r = e.rect;
                let c = rect_center(r);
                let height = h.unwrap_or(5.0);
                let pillar = Vec3::new(1.2, height, r.d as f32 - 0.2);
                let dx = r.w as f32 / 2.0 - 0.6;
                self.push_box(&e.id, c - Vec2::X * dx, 0.0, pillar, colors::STONE);
                self.push_box(&e.id, c + Vec2::X * dx, 0.0, pillar, colors::STONE);
                let beam = Vec3::new(r.w as f32, 0.7, 0.8);
                self.push_box(&e.id, c, height - 0.7, beam, colors::ROOF);
            }
            (ElementType::Building, _) => {
                let height = h.unwrap_or(4.0);
                self.rect_box(e, 0.0, height * 0.7, colors::BUILDING);
                let r = e.rect;
                let roof = Vec3::new(r.w as f32 + 0.3, height * 0.3, r.d as f32 + 0.3);
                self.push_box(&e.id, rect_center(r), height * 0.7, roof, colors::ROOF);
            }
            (ElementType::Barrier, "fallen_tree") => {
                self.model_at("fallen_tree", rect_center(e.rect), 0.0)
            }
            (ElementType::Barrier, "closed_gate") => {
                self.model_at("gate_zoo_closed", rect_center(e.rect), 0.0)
            }
            (ElementType::Barrier, "road_block") => {
                // Road block across the path (faces west), sign in front, cart and cones.
                let c = rect_center(e.rect);
                let west = facing_yaw(Dir::W);
                self.model_at("road_block", c, west);
                self.model_at(
                    "repair_sign",
                    c + Vec2::new(-0.7, -1.1),
                    (-35f32).to_radians(),
                );
                self.model_at("zookeeper_cart", c + Vec2::new(0.55, 0.9), west);
                self.model_at("traffic_cone", c + Vec2::new(-0.6, 1.2), 0.0);
                self.model_at("traffic_cone", c + Vec2::new(-0.4, -1.3), 0.0);
            }
            (ElementType::Barrier, _) => self.rect_box(e, 0.0, h.unwrap_or(1.2), colors::BARRIER),
            _ => self.rect_box(e, 0.0, h.unwrap_or(1.0), colors::DEFAULT),
        }
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
            self.placements.push(Placement {
                model: "fence_wood_corner",
                pos: level_to_world(*corner),
                yaw: quarter_turns_cw_to_yaw(k),
            });
        }
        for run in &fence.runs {
            self.run_pieces(run, "fence_wood", "fence_wood_1m");
        }
        if let Some(gate) = fence.gate {
            let dir = gate.axis.dir();
            self.placements.push(Placement {
                model: "gate_wood",
                pos: level_to_world(gate.start + dir * 0.09),
                yaw: run_yaw(gate.axis),
            });
            // Enclosure sign just outside the gate, reading outwards.
            let mid = gate.start + dir * 1.0;
            let out = dir_away(e.rect, mid + (mid - rect_center(e.rect)).normalize() * 0.01);
            let pos = mid + out.offset().as_vec2() * 0.35;
            self.placements.push(Placement {
                model: "enclosure_sign",
                pos: level_to_world(pos),
                yaw: facing_yaw(out),
            });
        }
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
        _ => (Vec3::ZERO, Vec3::ONE, colors::DEFAULT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zoo_core::coords::world_to_level;

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
        assert_eq!(water, 51 + 3 + 42 + 9 + 64);
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
}
