//! Living water (TECH-WATER, ART-ENVIRONMENT behaviour 5): the river centrelines from the
//! level data (`flow` on river elements, Q-066), the **water field** baked once per (joined)
//! level for the water shader, the obstacle foam list (Q-068), the global animation clock and
//! pure Rust mirrors of the shader patterns and of the bobbing (used by the tests only).
//!
//! Field layout (behaviour 1–2): 4 texels per metre over the bounding box of all water
//! (+ margin), rows along world +Z. Per texel `[s, c, shore, flow]`:
//! - `s`: metres along the river's flow (continuous through bends and bridges), 0 on ponds;
//! - `c`: signed metres from the centreline, + = left of the flow (level coordinates);
//! - `shore`: metres from the visible waterline into the water (negative on land);
//! - `flow`: 1 = river / stream, 0 = pond, pool, fountain.
//!
//! Level coordinates (x east, z north) are used for the logic; the field itself is indexed by
//! world XZ like the fragment shader (`world = (x, 0, −z)`, `coords`).

use glam::{IVec2, Vec2, Vec3};

use crate::level::{cell_center, ElementType, LevelData, Rect};

/// Bank grass strip width of the water kit (`kit_water.py` `M`).
pub const BANK_M: f32 = 0.22;
/// Horizontal width of the bank slope (`kit_water.py` `SLOPE`).
pub const BANK_SLOPE: f32 = 0.12;
/// Radius of the rounded outer corner of the water polygon (`kit_water.py` `R = 1 − M`).
pub const CORNER_R: f32 = 1.0 - BANK_M;
/// The visible waterline (foot of the bank slope) lies this far inside a bank edge.
pub const WATERLINE_INSET: f32 = BANK_M + BANK_SLOPE;
/// Field resolution (texels per metre).
pub const TEXELS_PER_M: u32 = 4;
/// Every water animation is periodic in this many seconds (AENV-008, behaviour 9).
pub const WATER_LOOP_S: f64 = 16.0;
/// Shore distances are clamped to this range (m).
pub const SHORE_MIN: f32 = -1.0;
pub const SHORE_MAX: f32 = 3.0;
/// Margin of the field around the water (m).
const FIELD_MARGIN_M: f32 = 2.0;
/// River half width minus the waterline inset (m): lane speed normalisation of the shader
/// (`HALF_W`, 3 m rivers; Q-066 note: store per river once other widths exist).
pub const STREAK_HALF_W: f32 = 1.16;

/// Shader time: elapsed seconds wrapped to `[0, 16)` (computed in `f64`, behaviour 9).
pub fn water_time(elapsed_s: f64) -> f32 {
    elapsed_s.rem_euclid(WATER_LOOP_S) as f32
}

/// Periods (s) of every time-dependent water pattern (TECH-WATER parameter table); each
/// divides [`WATER_LOOP_S`] (WATER-005).
pub const WATER_PERIODS_S: [(&str, f32); 9] = [
    ("river streak lane loop", 2.0),
    ("river fleck loop", 2.0),
    ("shore foam wobble", 2.0),
    ("shore foam dashes", 1.0),
    ("pond ring cycle", 4.0),
    ("pond twinkle", 4.0),
    ("pond glint breathing", 8.0),
    ("bob vertical / tilt", 4.0),
    ("bob drift", 16.0),
];

// ---------------------------------------------------------------------------------------
// Tile shapes (analytic waterline of the kit_water tiles)

/// Shape of a water tile in its canonical orientation (`kit_water.py` "WATER TILES").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileShape {
    /// All four sides water.
    Full,
    /// Bank on the west side.
    Bank,
    /// Banks on north and west, rounded outer corner around the SE corner.
    Corner,
    /// Full cell with a dry NE diagonal: small grass notch in the NE corner.
    Inner,
}

impl TileShape {
    /// Shape and body (`true` = river) of a water tile model; `None` for other models.
    pub fn of_model(model: &str) -> Option<(TileShape, bool)> {
        Some(match model {
            "water_river_straight" => (TileShape::Full, true),
            "water_river_bank" => (TileShape::Bank, true),
            "water_river_curve" => (TileShape::Corner, true),
            "water_river_inner" => (TileShape::Inner, true),
            "water_pond" => (TileShape::Full, false),
            "water_pond_edge" => (TileShape::Bank, false),
            "water_pond_corner" => (TileShape::Corner, false),
            _ => return None,
        })
    }

    /// Whether a canonical local point (cell centre = origin, x east, y north, m) is open
    /// water (inside the visible waterline).
    pub fn in_water(self, q: Vec2) -> bool {
        let e = 0.5;
        match self {
            TileShape::Full => true,
            TileShape::Bank => q.x > -e + WATERLINE_INSET,
            TileShape::Corner => q.distance(Vec2::new(e, -e)) < CORNER_R - BANK_SLOPE,
            TileShape::Inner => q.distance(Vec2::new(e, e)) > WATERLINE_INSET,
        }
    }

    /// Distance from a canonical local point to this tile's piece of the visible waterline
    /// (`None` for a full tile).
    pub fn waterline_distance(self, q: Vec2) -> Option<f32> {
        let e = 0.5;
        let arc = |c: Vec2, r: f32, a: Vec2, b: Vec2| {
            // quarter arc around `c` between the unit directions a and b
            let d = q - c;
            let inside = d.dot(a) >= 0.0 && d.dot(b) >= 0.0;
            if inside {
                (d.length() - r).abs()
            } else {
                q.distance(c + a * r).min(q.distance(c + b * r))
            }
        };
        match self {
            TileShape::Full => None,
            TileShape::Bank => {
                let x = -e + WATERLINE_INSET;
                Some(Vec2::new(q.x - x, (q.y.abs() - e).max(0.0)).length())
            }
            TileShape::Corner => Some(arc(
                Vec2::new(e, -e),
                CORNER_R - BANK_SLOPE,
                Vec2::Y,
                Vec2::NEG_X,
            )),
            TileShape::Inner => Some(arc(
                Vec2::new(e, e),
                WATERLINE_INSET,
                Vec2::NEG_X,
                Vec2::NEG_Y,
            )),
        }
    }
}

/// Clockwise quarter turn of a level vector (north → east).
pub fn rotate_cw(v: Vec2) -> Vec2 {
    Vec2::new(v.y, -v.x)
}

/// Canonical tile coordinates of a level offset from the cell centre, for a tile turned `k`
/// clockwise quarter turns (inverse of the tile rotation).
pub fn to_canonical(local: Vec2, turns: i32) -> Vec2 {
    let mut v = local;
    for _ in 0..turns.rem_euclid(4) {
        v = Vec2::new(-v.y, v.x); // counter-clockwise quarter turn
    }
    v
}

/// One water tile of the level scene.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterCell {
    pub cell: IVec2,
    /// River / stream (`true`) or still water (pond, pool).
    pub river: bool,
    pub shape: TileShape,
    /// Clockwise quarter turns of the tile.
    pub turns: i32,
}

impl WaterCell {
    fn local(&self, p: Vec2) -> Vec2 {
        to_canonical(p - cell_center(self.cell), self.turns)
    }

    /// Whether a level point in this cell is open water.
    pub fn in_water(&self, p: Vec2) -> bool {
        self.shape.in_water(self.local(p))
    }

    /// Distance from a level point to this tile's waterline piece.
    pub fn waterline_distance(&self, p: Vec2) -> Option<f32> {
        self.shape.waterline_distance(self.local(p))
    }
}

/// Still water that is not made of ground tiles (the fountain basin, level coordinates).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StillWater {
    pub min: Vec2,
    pub max: Vec2,
}

impl StillWater {
    /// Signed distance from the rim into the water (negative outside).
    pub fn shore(&self, p: Vec2) -> f32 {
        let inside = p.cmpge(self.min).all() && p.cmple(self.max).all();
        if inside {
            (p - self.min).min(self.max - p).min_element()
        } else {
            -p.distance(p.clamp(self.min, self.max))
        }
    }
}

/// A foam obstacle standing in the water (Q-068: bridge piles, rocks in the river, the water
/// wheel, jetty posts, the fountain jet). Level coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obstacle {
    pub pos: Vec2,
    pub radius: f32,
    /// In flowing water (ring + V-wake) or still water (ring only).
    pub river: bool,
}

// ---------------------------------------------------------------------------------------
// River centrelines (behaviour 2, Q-066)

/// Axis direction of a `flow` key (level coordinates).
pub fn flow_dir(flow: &str) -> Option<Vec2> {
    Some(match flow {
        "N" => Vec2::Y,
        "E" => Vec2::X,
        "S" => Vec2::NEG_Y,
        "W" => Vec2::NEG_X,
        _ => return None,
    })
}

fn left(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}

/// A piece of a river centreline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathPiece {
    /// Straight piece from `a` along the unit direction `dir`.
    Line {
        a: Vec2,
        dir: Vec2,
        len: f32,
        s0: f32,
    },
    /// Quarter arc around `center` (radius `r`), starting at angle `a0`, sweeping `sweep`
    /// radians (+ = counter-clockwise = left turn).
    Arc {
        center: Vec2,
        r: f32,
        a0: f32,
        sweep: f32,
        s0: f32,
    },
}

impl PathPiece {
    fn s0(&self) -> f32 {
        match *self {
            PathPiece::Line { s0, .. } | PathPiece::Arc { s0, .. } => s0,
        }
    }

    fn len(&self) -> f32 {
        match *self {
            PathPiece::Line { len, .. } => len,
            PathPiece::Arc { r, sweep, .. } => r * sweep.abs(),
        }
    }

    /// `(s, c, distance to the piece)`; `s` and `c` extrapolate beyond straight ends.
    fn coords(&self, p: Vec2) -> (f32, f32, f32) {
        match *self {
            PathPiece::Line { a, dir, len, s0 } => {
                let d = p - a;
                let t = d.dot(dir);
                let c = d.dot(left(dir));
                let dist = (p - (a + dir * t.clamp(0.0, len))).length();
                (s0 + t, c, dist)
            }
            PathPiece::Arc {
                center,
                r,
                a0,
                sweep,
                s0,
            } => {
                let d = p - center;
                let rho = d.length();
                let phi = d.y.atan2(d.x);
                let rel = wrap_pi(phi - a0) * sweep.signum();
                let alpha = rel.clamp(0.0, sweep.abs());
                let on = center + Vec2::from_angle(a0 + alpha * sweep.signum()) * r;
                let dist = if rel == alpha {
                    (rho - r).abs()
                } else {
                    p.distance(on)
                };
                let c = if sweep > 0.0 { r - rho } else { rho - r };
                (s0 + r * alpha, c, dist)
            }
        }
    }

    fn point(&self, s: f32, c: f32) -> Vec2 {
        match *self {
            PathPiece::Line { a, dir, s0, .. } => a + dir * (s - s0) + left(dir) * c,
            PathPiece::Arc {
                center,
                r,
                a0,
                sweep,
                s0,
            } => {
                let ang = a0 + sweep.signum() * (s - s0) / r;
                let rho = if sweep > 0.0 { r - c } else { r + c };
                center + Vec2::from_angle(ang) * rho
            }
        }
    }

    fn dir(&self, s: f32) -> Vec2 {
        match *self {
            PathPiece::Line { dir, .. } => dir,
            PathPiece::Arc {
                r, a0, sweep, s0, ..
            } => {
                let ang = a0 + sweep.signum() * (s - s0) / r;
                let radial = Vec2::from_angle(ang);
                if sweep > 0.0 {
                    left(radial)
                } else {
                    -left(radial)
                }
            }
        }
    }
}

fn wrap_pi(a: f32) -> f32 {
    let t = std::f32::consts::TAU;
    let mut x = (a + std::f32::consts::PI).rem_euclid(t) - std::f32::consts::PI;
    if x <= -std::f32::consts::PI {
        x += t;
    }
    x
}

/// Centreline of one river / stream with the rectangles it covers.
#[derive(Debug, Clone, PartialEq)]
pub struct RiverPath {
    /// Element ids in flow order (rivers, streams and the bridges over them).
    pub ids: Vec<String>,
    pub rects: Vec<Rect>,
    pub pieces: Vec<PathPiece>,
    pub half_width: f32,
}

impl RiverPath {
    /// Total length of the centreline (m).
    pub fn length(&self) -> f32 {
        self.pieces.last().map_or(0.0, |p| p.s0() + p.len())
    }

    /// Whether a cell belongs to this river.
    pub fn contains(&self, c: IVec2) -> bool {
        self.rects.iter().any(|r| r.contains(c))
    }

    /// River coordinates `(s, c)` of a level point (nearest centreline piece).
    pub fn coords(&self, p: Vec2) -> (f32, f32) {
        let mut best = (0.0, 0.0, f32::MAX);
        for piece in &self.pieces {
            let (s, c, d) = piece.coords(p);
            if d < best.2 - 1e-5 {
                best = (s, c, d);
            }
        }
        (best.0, best.1)
    }

    fn piece_at(&self, s: f32) -> &PathPiece {
        self.pieces
            .iter()
            .find(|p| s < p.s0() + p.len())
            .or(self.pieces.last())
            .expect("river path has pieces")
    }

    /// Level point at river coordinates (inverse of [`RiverPath::coords`]).
    pub fn point(&self, s: f32, c: f32) -> Vec2 {
        self.piece_at(s).point(s, c)
    }

    /// Unit flow direction at `s`.
    pub fn dir_at(&self, s: f32) -> Vec2 {
        self.piece_at(s).dir(s)
    }
}

struct Seg {
    id: String,
    rect: Rect,
    dir: Vec2,
}

fn rect_min(r: Rect) -> Vec2 {
    Vec2::new(r.x as f32, r.z as f32)
}

fn rect_size(r: Rect) -> Vec2 {
    Vec2::new(r.w as f32, r.d as f32)
}

/// Width across the flow and length along it.
fn across_along(r: Rect, dir: Vec2) -> (i32, i32) {
    if dir.x != 0.0 {
        (r.d, r.w)
    } else {
        (r.w, r.d)
    }
}

/// Cells just beyond the downstream edge of a rect.
fn exit_cells(r: Rect, dir: Vec2) -> Vec<IVec2> {
    let d = IVec2::new(dir.x as i32, dir.y as i32);
    r.cells()
        .filter(|c| !r.contains(*c + d))
        .map(|c| c + d)
        .collect()
}

/// Builds the river centrelines from the level data (Q-066, GAME-LAYOUT "Water"): rivers and
/// streams with their `flow`, bridges over them inherit the flow of the river piece they
/// continue. Pieces are chained by adjacency along the flow; a change of direction is a 90°
/// bend whose square is the first `width` cells of the downstream piece. Errors describe
/// inconsistent data (width changes, reversed flow, gaps, several sources).
pub fn river_paths(data: &LevelData) -> Result<Vec<RiverPath>, String> {
    let mut segs: Vec<Seg> = Vec::new();
    for e in &data.elements {
        let flowing =
            e.ty == ElementType::Landmark && matches!(e.kind.as_deref(), Some("river" | "stream"));
        if flowing {
            let f = e.flow.as_deref().unwrap_or("");
            let dir = flow_dir(f).ok_or_else(|| format!("{}: flow {f:?} is not N/E/S/W", e.id))?;
            segs.push(Seg {
                id: e.id.clone(),
                rect: e.rect,
                dir,
            });
        }
    }
    // bridges continue the river piece they are aligned with
    for e in &data.elements {
        if e.ty != ElementType::Path || e.kind.as_deref() != Some("bridge") {
            continue;
        }
        let along = segs.iter().find(|s| {
            let r = s.rect;
            let b = e.rect;
            let aligned = if s.dir.x != 0.0 {
                b.z == r.z && b.d == r.d
            } else {
                b.x == r.x && b.w == r.w
            };
            let touching = exit_cells(r, s.dir).iter().any(|c| b.contains(*c))
                || exit_cells(b, s.dir).iter().any(|c| r.contains(*c));
            aligned && touching
        });
        if let Some(s) = along {
            let dir = s.dir;
            segs.push(Seg {
                id: e.id.clone(),
                rect: e.rect,
                dir,
            });
        }
    }
    // downstream links
    let n = segs.len();
    let mut next: Vec<Option<usize>> = vec![None; n];
    let mut prev_count = vec![0; n];
    for i in 0..n {
        let exit = exit_cells(segs[i].rect, segs[i].dir);
        let hit: Vec<usize> = (0..n)
            .filter(|&j| j != i && exit.iter().any(|c| segs[j].rect.contains(*c)))
            .collect();
        let Some(&j) = hit.first() else { continue };
        if hit.len() > 1 || !exit.iter().all(|c| segs[j].rect.contains(*c)) {
            return Err(format!(
                "{}: its downstream edge is not covered by exactly one river piece",
                segs[i].id
            ));
        }
        let (a, b) = (&segs[i], &segs[j]);
        let (wa, _) = across_along(a.rect, a.dir);
        let (wb, _) = across_along(b.rect, b.dir);
        if wa != wb {
            return Err(format!("{} → {}: river width changes", a.id, b.id));
        }
        if a.dir == -b.dir {
            return Err(format!("{} → {}: flow reverses", a.id, b.id));
        }
        if a.dir == b.dir {
            let same_line = if a.dir.x != 0.0 {
                a.rect.z == b.rect.z
            } else {
                a.rect.x == b.rect.x
            };
            if !same_line {
                return Err(format!("{} → {}: centrelines do not line up", a.id, b.id));
            }
        } else {
            // bend: the exit row must lie in the first `width` cells of b along its flow
            let r = b.rect;
            let bend_ok = exit.iter().all(|c| {
                let k = match (b.dir.x as i32, b.dir.y as i32) {
                    (1, _) => c.x - r.x,
                    (-1, _) => r.x + r.w - 1 - c.x,
                    (_, 1) => c.y - r.z,
                    _ => r.z + r.d - 1 - c.y,
                };
                (0..wb).contains(&k)
            });
            if !bend_ok {
                return Err(format!(
                    "{} → {}: bend is not at the upstream end of {}",
                    a.id, b.id, b.id
                ));
            }
        }
        next[i] = Some(j);
        prev_count[j] += 1;
    }
    if let Some(j) = (0..n).find(|&j| prev_count[j] > 1) {
        return Err(format!("{}: two rivers flow into it", segs[j].id));
    }
    let mut out = Vec::new();
    let mut used = vec![false; n];
    for start in (0..n).filter(|&j| prev_count[j] == 0) {
        let mut chain = vec![start];
        while let Some(j) = next[*chain.last().unwrap()] {
            if chain.contains(&j) {
                return Err(format!("{}: river loops", segs[j].id));
            }
            chain.push(j);
        }
        for &k in &chain {
            used[k] = true;
        }
        out.push(build_path(&segs, &chain));
    }
    if let Some(k) = (0..n).find(|&k| !used[k]) {
        return Err(format!("{}: river without a source (loop)", segs[k].id));
    }
    Ok(out)
}

fn build_path(segs: &[Seg], chain: &[usize]) -> RiverPath {
    let first = &segs[chain[0]];
    let (w, _) = across_along(first.rect, first.dir);
    let r = w as f32 / 2.0;
    let center = |s: &Seg| rect_min(s.rect) + rect_size(s.rect) / 2.0;
    let half_along = |s: &Seg| {
        let (_, l) = across_along(s.rect, s.dir);
        l as f32 / 2.0
    };
    let mut pieces: Vec<PathPiece> = Vec::new();
    let mut s_acc = 0.0f32;
    let mut cur = center(first) - first.dir * half_along(first);
    let mut dir = first.dir;
    let push_line = |pieces: &mut Vec<PathPiece>, s_acc: &mut f32, a: Vec2, b: Vec2| {
        let len = (b - a).length();
        if len < 1e-4 {
            return;
        }
        let d = (b - a) / len;
        if let Some(PathPiece::Line {
            dir: pd, len: pl, ..
        }) = pieces.last_mut()
        {
            if pd.abs_diff_eq(d, 1e-4) {
                *pl += len;
                *s_acc += len;
                return;
            }
        }
        pieces.push(PathPiece::Line {
            a,
            dir: d,
            len,
            s0: *s_acc,
        });
        *s_acc += len;
    };
    for &k in chain {
        let s = &segs[k];
        if s.dir != dir {
            // bend square = first w cells of this piece
            let corner = cur + dir * r;
            let arc_center = corner + s.dir * r - dir * r;
            let end = corner + s.dir * r;
            let turn_left = dir.perp_dot(s.dir) > 0.0;
            let a0 = (cur - arc_center).y.atan2((cur - arc_center).x);
            let sweep = if turn_left {
                std::f32::consts::FRAC_PI_2
            } else {
                -std::f32::consts::FRAC_PI_2
            };
            pieces.push(PathPiece::Arc {
                center: arc_center,
                r,
                a0,
                sweep,
                s0: s_acc,
            });
            s_acc += r * sweep.abs();
            cur = end;
            dir = s.dir;
        }
        let end = center(s) + s.dir * half_along(s);
        push_line(&mut pieces, &mut s_acc, cur, end);
        cur = end;
    }
    RiverPath {
        ids: chain.iter().map(|&k| segs[k].id.clone()).collect(),
        rects: chain.iter().map(|&k| segs[k].rect).collect(),
        pieces,
        half_width: r,
    }
}

// ---------------------------------------------------------------------------------------
// Water scene and field bake

/// Everything the water field and the ambient animals need from the level assembly.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WaterScene {
    pub cells: Vec<WaterCell>,
    pub stills: Vec<StillWater>,
    pub rivers: Vec<RiverPath>,
    pub obstacles: Vec<Obstacle>,
}

impl WaterScene {
    /// The river a cell belongs to.
    pub fn river_of(&self, c: IVec2) -> Option<usize> {
        self.rivers.iter().position(|r| r.contains(c))
    }

    /// Water cell at a grid cell.
    pub fn cell(&self, c: IVec2) -> Option<&WaterCell> {
        self.cells.iter().find(|w| w.cell == c)
    }

    /// Whether a level point is open water of a river.
    pub fn is_river_water(&self, p: Vec2) -> bool {
        let c = crate::level::cell_of(p);
        self.cell(c).is_some_and(|w| w.river && w.in_water(p))
    }

    /// Whether a level point is open water (any body).
    pub fn is_water(&self, p: Vec2) -> bool {
        let c = crate::level::cell_of(p);
        self.cell(c).is_some_and(|w| w.in_water(p)) || self.stills.iter().any(|s| s.shore(p) > 0.0)
    }
}

/// The baked water field (behaviour 1–2), RGBA float per texel.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WaterField {
    /// World XZ of the field's corner (texel (0, 0) starts here).
    pub origin: Vec2,
    pub texels_per_m: u32,
    pub width: u32,
    pub height: u32,
    /// Row-major, row `j` at world z = origin.y + (j + 0.5) / texels_per_m.
    pub data: Vec<[f32; 4]>,
}

/// Dense lookup of water cells over a bounding box.
struct CellGrid {
    min: IVec2,
    w: i32,
    h: i32,
    idx: Vec<i32>,
}

impl CellGrid {
    fn get(&self, c: IVec2) -> Option<usize> {
        let d = c - self.min;
        if d.x < 0 || d.y < 0 || d.x >= self.w || d.y >= self.h {
            return None;
        }
        let k = self.idx[(d.y * self.w + d.x) as usize];
        (k >= 0).then_some(k as usize)
    }
}

impl WaterField {
    /// World size of the field (m).
    pub fn size_m(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32) / self.texels_per_m as f32
    }

    /// Texel at (i, j).
    pub fn texel(&self, i: u32, j: u32) -> [f32; 4] {
        self.data[(j * self.width + i) as usize]
    }

    /// World XZ of the centre of texel (i, j).
    pub fn texel_world(&self, i: u32, j: u32) -> Vec2 {
        self.origin + (Vec2::new(i as f32, j as f32) + 0.5) / self.texels_per_m as f32
    }

    /// Bilinear sample at world XZ (like the GPU with LINEAR + CLAMP_TO_EDGE).
    pub fn sample(&self, world_xz: Vec2) -> [f32; 4] {
        if self.width == 0 || self.height == 0 {
            return [0.0, 0.0, SHORE_MIN, 0.0];
        }
        let f = (world_xz - self.origin) * self.texels_per_m as f32 - 0.5;
        let x0 = f.x.floor();
        let y0 = f.y.floor();
        let (tx, ty) = (f.x - x0, f.y - y0);
        let cl = |v: f32, n: u32| (v as i64).clamp(0, n as i64 - 1) as u32;
        let (i0, i1) = (cl(x0, self.width), cl(x0 + 1.0, self.width));
        let (j0, j1) = (cl(y0, self.height), cl(y0 + 1.0, self.height));
        let (a, b, c, d) = (
            self.texel(i0, j0),
            self.texel(i1, j0),
            self.texel(i0, j1),
            self.texel(i1, j1),
        );
        let mut out = [0.0; 4];
        for k in 0..4 {
            let top = a[k] + (b[k] - a[k]) * tx;
            let bot = c[k] + (d[k] - c[k]) * tx;
            out[k] = top + (bot - top) * ty;
        }
        out
    }

    /// Bakes the field for a water scene (behaviour 2).
    pub fn bake(scene: &WaterScene) -> WaterField {
        let mut lo = Vec2::splat(f32::MAX);
        let mut hi = Vec2::splat(f32::MIN);
        for w in &scene.cells {
            lo = lo.min(w.cell.as_vec2());
            hi = hi.max(w.cell.as_vec2() + 1.0);
        }
        for s in &scene.stills {
            lo = lo.min(s.min);
            hi = hi.max(s.max);
        }
        if lo.x > hi.x {
            return WaterField {
                origin: Vec2::ZERO,
                texels_per_m: TEXELS_PER_M,
                width: 1,
                height: 1,
                data: vec![[0.0, 0.0, SHORE_MIN, 0.0]],
            };
        }
        let lo = (lo - FIELD_MARGIN_M).floor();
        let hi = (hi + FIELD_MARGIN_M).ceil();
        let tpm = TEXELS_PER_M;
        // world z = −level z: the field's world z range is [−hi.y, −lo.y]
        let origin = Vec2::new(lo.x, -hi.y);
        let width = ((hi.x - lo.x) as u32) * tpm;
        let height = ((hi.y - lo.y) as u32) * tpm;

        let cmin = lo.as_ivec2();
        let cw = (hi.x - lo.x) as i32;
        let ch = (hi.y - lo.y) as i32;
        let mut idx = vec![-1i32; (cw * ch) as usize];
        for (k, w) in scene.cells.iter().enumerate() {
            let d = w.cell - cmin;
            if d.x >= 0 && d.y >= 0 && d.x < cw && d.y < ch {
                idx[(d.y * cw + d.x) as usize] = k as i32;
            }
        }
        let grid = CellGrid {
            min: cmin,
            w: cw,
            h: ch,
            idx,
        };
        // river of every water cell (cached)
        let river_of: Vec<Option<usize>> = scene
            .cells
            .iter()
            .map(|w| {
                if w.river {
                    scene.river_of(w.cell).or_else(|| {
                        // nearest river (cells outside every rect, e.g. beyond a level edge)
                        let p = cell_center(w.cell);
                        (0..scene.rivers.len()).min_by(|&a, &b| {
                            let da = dist_to_rects(&scene.rivers[a].rects, p);
                            let db = dist_to_rects(&scene.rivers[b].rects, p);
                            da.total_cmp(&db)
                        })
                    })
                } else {
                    None
                }
            })
            .collect();

        let mut data = vec![[0.0, 0.0, SHORE_MIN, 0.0]; (width * height) as usize];
        for j in 0..height {
            for i in 0..width {
                let wxz = origin + (Vec2::new(i as f32, j as f32) + 0.5) / tpm as f32;
                let p = Vec2::new(wxz.x, -wxz.y); // level point
                data[(j * width + i) as usize] = texel_value(scene, &grid, &river_of, p);
            }
        }
        WaterField {
            origin,
            texels_per_m: tpm,
            width,
            height,
            data,
        }
    }
}

fn dist_to_rects(rects: &[Rect], p: Vec2) -> f32 {
    rects
        .iter()
        .map(|r| r.distance_to(p))
        .fold(f32::MAX, f32::min)
}

fn texel_value(
    scene: &WaterScene,
    grid: &CellGrid,
    river_of: &[Option<usize>],
    p: Vec2,
) -> [f32; 4] {
    // still water outside the tile grid (fountain basin)
    for s in &scene.stills {
        let d = s.shore(p);
        if d > -1.0 {
            return [0.0, 0.0, d.clamp(SHORE_MIN, SHORE_MAX), 0.0];
        }
    }
    let c = crate::level::cell_of(p);
    let own = grid.get(c);
    // nearest water cell (own first), within 2 cells
    let mut body: Option<usize> = own;
    let mut waterline = f32::MAX;
    let mut nearest = f32::MAX;
    for dz in -2..=2 {
        for dx in -2..=2 {
            let q = c + IVec2::new(dx, dz);
            let Some(k) = grid.get(q) else { continue };
            let w = &scene.cells[k];
            if let Some(d) = w.waterline_distance(p) {
                waterline = waterline.min(d);
            }
            if own.is_none() {
                let d = Rect::new(q.x, q.y, 1, 1).distance_to(p);
                if d < nearest {
                    nearest = d;
                    body = Some(k);
                }
            }
        }
    }
    let Some(k) = body else {
        return [0.0, 0.0, SHORE_MIN, 0.0];
    };
    let in_water = own.is_some_and(|o| scene.cells[o].in_water(p));
    let dist = if waterline == f32::MAX {
        SHORE_MAX
    } else {
        waterline
    };
    let shore = if in_water { dist } else { -dist }.clamp(SHORE_MIN, SHORE_MAX);
    match river_of[k] {
        Some(r) => {
            let (s, cc) = scene.rivers[r].coords(p);
            [s, cc, shore, 1.0]
        }
        None => [0.0, 0.0, shore, 0.0],
    }
}

// ---------------------------------------------------------------------------------------
// Pure mirrors of the shader patterns (tests: WATER-004/005/009)

const TAU: f32 = std::f32::consts::TAU;

fn fract(x: f32) -> f32 {
    x - x.floor()
}

/// GLSL `hash1` of the water shader.
pub fn hash1(n: f32) -> f32 {
    fract((n * 12.9898).sin() * 43758.547)
}

/// GLSL `hash2` of the water shader.
pub fn hash2(v: Vec2) -> f32 {
    fract((v.dot(Vec2::new(12.9898, 78.233))).sin() * 43758.547)
}

/// Speed (m/s) and dash period (m) of the streak lane `li` (behaviour 3).
pub fn streak_lane(li: f32) -> (f32, f32) {
    let cn = (((li + 0.5) * 0.24).abs() / STREAK_HALF_W).clamp(0.0, 1.0);
    let v = 0.85 + (0.5 - 0.85) * cn * cn;
    (v, 2.0 * v)
}

/// River streak mask (behaviour 3, hard edges): 1 where a light dash is drawn.
pub fn river_streak_mask(s: f32, c: f32, shore: f32, t: f32) -> f32 {
    const LANE: f32 = 0.24;
    let c = c + 0.045 * (s * 2.1 + c * 1.7).sin();
    let li = (c / LANE).floor();
    let lc = (fract(c / LANE) - 0.5) * LANE;
    let (v, p) = streak_lane(li);
    let x = (s - v * t) / p + hash1(li + 3.1) * 8.0;
    let cell = x.floor();
    let u = fract(x);
    let h = hash2(Vec2::new(li, cell.rem_euclid(8.0)));
    let len = 0.28 + (0.55 - 0.28) * fract(h * 13.7);
    let tt = (u / len).clamp(0.0, 1.0);
    let present = u <= len && h >= 0.3;
    let hw = (0.02 + 0.03 * fract(h * 7.3)) * (std::f32::consts::PI * tt).sin();
    if present && hw > 0.0 && lc.abs() < hw && shore >= 0.16 {
        1.0
    } else {
        0.0
    }
}

/// Pond ring mask (behaviour 7 rings, hard edges) at a world XZ point.
pub fn pond_ring_mask(p: Vec2, shore: f32, t: f32) -> f32 {
    let g = p / 2.5;
    let gi = g.floor();
    let hr = hash2(gi + 7.0);
    let jitter = Vec2::new(hash2(gi + 1.3), hash2(gi + 5.1)) - 0.5;
    let ctr = (gi + 0.5 + jitter * 0.28) * 2.5;
    let age = fract(t / 4.0 + hr);
    let dd = p.distance(ctr);
    let th = 0.04 * (1.0 - age) + 0.006;
    let r1 = age * 0.95;
    let r2 = age * 0.95 - 0.24;
    let ring = (dd - r1).abs() < th || (r2 >= 0.0 && (dd - r2).abs() < th);
    if ring && hr >= 0.25 && shore >= 0.18 {
        1.0
    } else {
        0.0
    }
}

/// Bobbing parameters of a batch (behaviour 8): vertical amplitude (m), tilt (rad), drift
/// radius (m).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BobParams {
    pub amp_y: f32,
    pub tilt: f32,
    pub drift: f32,
}

impl BobParams {
    pub const NONE: BobParams = BobParams {
        amp_y: 0.0,
        tilt: 0.0,
        drift: 0.0,
    };

    /// As the shader uniform `u_bob`.
    pub fn uniform(self) -> [f32; 4] {
        [self.amp_y, self.tilt, self.drift, 0.0]
    }
}

/// Bobbing of a model on the water (TECH-WATER behaviour 8); every other model: none.
pub fn bob_params(model: &str) -> BobParams {
    match model {
        "duck" | "duckling" => BobParams {
            amp_y: 0.03,
            tilt: 0.07,
            drift: 0.12,
        },
        "lily_pad" | "frog" => BobParams {
            amp_y: 0.012,
            tilt: 0.04,
            drift: 0.03,
        },
        _ => BobParams::NONE,
    }
}

/// Integer hash of the instance origin (same as the GLSL `bob_hash`): phase in [0, 1).
pub fn bob_hash(origin_world: Vec3) -> f32 {
    let kx = (origin_world.x * 10.0).floor() as i32 as u32;
    let kz = (origin_world.z * 10.0).floor() as i32 as u32;
    let mut h = kx.wrapping_mul(0x9E37_79B1) ^ kz.wrapping_mul(0x85EB_CA77);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    (h & 0xFFFF) as f32 / 65536.0
}

/// Bobbing pose of an instance at shader time `t`: world offset and roll about the model's
/// local X axis (radians).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bob {
    pub offset: Vec3,
    pub roll: f32,
}

/// The bobbing offset of an instance (mirrors `static_vs`).
pub fn bob(origin_world: Vec3, params: BobParams, t: f32) -> Bob {
    let ph = bob_hash(origin_world) * TAU;
    let y = params.amp_y * (TAU * t / 2.0 + ph).sin();
    let roll = params.tilt * (TAU * t / 4.0 + ph + 1.3).sin();
    let a = TAU * t / 16.0 + ph;
    Bob {
        offset: Vec3::new(params.drift * a.cos(), y, params.drift * a.sin()),
        roll,
    }
}

/// World position of a point given in a bobbing model's local frame (pad spots for frogs):
/// roll about local X, yaw about +Y, then origin + offset — exactly the vertex transform of
/// `static_vs`.
pub fn bob_transform(origin_world: Vec3, yaw: f32, local: Vec3, b: Bob) -> Vec3 {
    let (sr, cr) = b.roll.sin_cos();
    let p = Vec3::new(
        local.x,
        cr * local.y - sr * local.z,
        sr * local.y + cr * local.z,
    );
    let (s, c) = yaw.sin_cos();
    Vec3::new(c * p.x + s * p.z, p.y, -s * p.x + c * p.z) + origin_world + b.offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_rotation_inverts_the_tile_turns() {
        // bank on W turned once clockwise → bank on N: a point near the north edge is land
        let north = Vec2::new(0.0, 0.45);
        assert!(!TileShape::Bank.in_water(to_canonical(north, 1)));
        assert!(TileShape::Bank.in_water(to_canonical(Vec2::new(0.0, -0.3), 1)));
        assert_eq!(rotate_cw(Vec2::Y), Vec2::X);
    }

    #[test]
    fn waterline_distance_is_zero_on_the_slope_foot() {
        let x = -0.5 + WATERLINE_INSET;
        assert!(
            TileShape::Bank
                .waterline_distance(Vec2::new(x, 0.2))
                .unwrap()
                < 1e-6
        );
        let r = CORNER_R - BANK_SLOPE;
        let q = Vec2::new(0.5, -0.5) + Vec2::from_angle(2.3) * r;
        assert!(TileShape::Corner.waterline_distance(q).unwrap() < 1e-5);
    }

    #[test]
    fn water_time_wraps() {
        assert!((water_time(17.5) - 1.5).abs() < 1e-6);
        assert!(water_time(-1.0) > 14.9);
    }
}
