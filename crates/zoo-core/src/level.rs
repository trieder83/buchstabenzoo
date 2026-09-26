//! Level layout data (GAME-LAYOUT, GAME-LEVEL-1) and the walkable grid derived from it.
//!
//! Grid: 1 m cells, `+X` east, `+Z` north. A cell `(x, z)` covers `[x, x+1) × [z, z+1)`;
//! positions are level coordinates `Vec2(x, z)` in metres (GAME-LAYOUT "Coordinate spaces";
//! convert to render/world space only with `crate::coords::level_to_world`).

use std::collections::BTreeSet;

use glam::{IVec2, Vec2};
use serde::Deserialize;

/// Grid rectangle `[x, z, w, d]` (south-west corner + size), covering cells
/// `x .. x+w-1`, `z .. z+d-1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(from = "[i32; 4]")]
pub struct Rect {
    pub x: i32,
    pub z: i32,
    pub w: i32,
    pub d: i32,
}

impl From<[i32; 4]> for Rect {
    fn from(v: [i32; 4]) -> Self {
        Self {
            x: v[0],
            z: v[1],
            w: v[2],
            d: v[3],
        }
    }
}

impl Rect {
    pub const fn new(x: i32, z: i32, w: i32, d: i32) -> Self {
        Self { x, z, w, d }
    }

    pub fn contains(&self, c: IVec2) -> bool {
        c.x >= self.x && c.x < self.x + self.w && c.y >= self.z && c.y < self.z + self.d
    }

    /// All cells of the rectangle.
    pub fn cells(self) -> impl Iterator<Item = IVec2> {
        (self.z..self.z + self.d)
            .flat_map(move |z| (self.x..self.x + self.w).map(move |x| IVec2::new(x, z)))
    }

    /// Distance in metres from a level position to the nearest point of the rectangle's area.
    pub fn distance_to(&self, p: Vec2) -> f32 {
        let min = Vec2::new(self.x as f32, self.z as f32);
        let max = min + Vec2::new(self.w as f32, self.d as f32);
        p.distance(p.clamp(min, max))
    }

    /// Whether the cell touches the rectangle (8-neighbourhood) without being inside it.
    pub fn is_adjacent(&self, c: IVec2) -> bool {
        !self.contains(c)
            && c.x >= self.x - 1
            && c.x <= self.x + self.w
            && c.y >= self.z - 1
            && c.y <= self.z + self.d
    }
}

/// Centre of a grid cell in level coordinates (metres).
pub fn cell_center(c: IVec2) -> Vec2 {
    Vec2::new(c.x as f32 + 0.5, c.y as f32 + 0.5)
}

/// Grid cell containing a level position.
pub fn cell_of(p: Vec2) -> IVec2 {
    IVec2::new(p.x.floor() as i32, p.y.floor() as i32)
}

/// Element type (GAME-LAYOUT "Element types").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementType {
    Path,
    Enclosure,
    Building,
    Landmark,
    Barrier,
    Boundary,
    Decoration,
    /// Proposal Q-044: overlay rectangle, not solid.
    HidingPlace,
}

impl ElementType {
    /// Solid = every type except `path` and `hiding_place` (GAME-LEVEL-1 "Elements").
    pub fn is_solid(self) -> bool {
        !matches!(self, ElementType::Path | ElementType::HidingPlace)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ElementType::Path => "path",
            ElementType::Enclosure => "enclosure",
            ElementType::Building => "building",
            ElementType::Landmark => "landmark",
            ElementType::Barrier => "barrier",
            ElementType::Boundary => "boundary",
            ElementType::Decoration => "decoration",
            ElementType::HidingPlace => "hiding_place",
        }
    }
}

/// One element of the layout data.
#[derive(Debug, Clone, Deserialize)]
pub struct Element {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: ElementType,
    pub kind: Option<String>,
    pub rect: Rect,
    /// Enclosures and hiding places: the animal id.
    pub animal: Option<String>,
    /// Enclosures: gate cells (part of the enclosure rectangle).
    pub gate: Option<Rect>,
    /// Buildings: door cell.
    pub door: Option<[i32; 2]>,
    /// Hiding places: the cell where the animal waits.
    pub animal_spot: Option<[i32; 2]>,
    /// Hiding places: riddle details the place must show.
    #[serde(default)]
    pub features: Vec<String>,
    /// Info boards: the enclosure they belong to.
    pub enclosure: Option<String>,
    /// Barriers: level transition, e.g. `level_1->level_2`.
    pub transition: Option<String>,
    #[serde(default)]
    pub blocks_view: bool,
    pub height_m: Option<f32>,
}

impl Element {
    pub fn door_cell(&self) -> Option<IVec2> {
        self.door.map(|d| IVec2::new(d[0], d[1]))
    }

    pub fn animal_spot_cell(&self) -> Option<IVec2> {
        self.animal_spot.map(|d| IVec2::new(d[0], d[1]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    Path,
    Grass,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelHeader {
    pub id: String,
    pub spec: Option<String>,
    pub cell_size_m: f32,
    pub bounds: Rect,
    pub ground_walkable: bool,
    pub ground_surface: Surface,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spawn {
    pub cell: [i32; 2],
    pub facing: String,
}

impl Spawn {
    pub fn cell(&self) -> IVec2 {
        IVec2::new(self.cell[0], self.cell[1])
    }
}

/// Level direction from a facing string of the layout data (`+z` north, `-z` south, `+x`
/// east, `-x` west); unknown strings face north.
pub fn facing_vec(s: &str) -> Vec2 {
    match s {
        "-z" => Vec2::NEG_Y,
        "+x" => Vec2::X,
        "-x" => Vec2::NEG_X,
        _ => Vec2::Y,
    }
}

fn default_south() -> String {
    "-z".to_owned()
}

/// A food box prop (GAME-FEED §7): which food, where its centre stands (level `(x, z)`) and
/// which way its label faces.
#[derive(Debug, Clone, Deserialize)]
pub struct FoodBoxData {
    pub food: String,
    pub pos: [f32; 2],
    #[serde(default = "default_south")]
    pub facing: String,
}

impl FoodBoxData {
    pub fn pos(&self) -> Vec2 {
        Vec2::new(self.pos[0], self.pos[1])
    }

    pub fn facing(&self) -> Vec2 {
        facing_vec(&self.facing)
    }
}

/// Parsed `assets/levels/level-<N>.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub level: LevelHeader,
    pub spawn: Spawn,
    #[serde(rename = "element")]
    pub elements: Vec<Element>,
    /// Food boxes (GAME-FEED §7); optional.
    #[serde(default, rename = "food_box")]
    pub food_boxes: Vec<FoodBoxData>,
}

#[derive(Debug)]
pub enum LevelError {
    Toml(toml::de::Error),
    Invalid(String),
}

impl std::fmt::Display for LevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelError::Toml(e) => write!(f, "level toml: {e}"),
            LevelError::Invalid(s) => write!(f, "invalid level: {s}"),
        }
    }
}

impl std::error::Error for LevelError {}

impl LevelData {
    pub fn from_toml_str(s: &str) -> Result<Self, LevelError> {
        let data: LevelData = toml::from_str(s).map_err(LevelError::Toml)?;
        let mut ids = BTreeSet::new();
        for e in &data.elements {
            if !ids.insert(e.id.as_str()) {
                return Err(LevelError::Invalid(format!(
                    "duplicate element id {}",
                    e.id
                )));
            }
            if e.rect.w <= 0 || e.rect.d <= 0 {
                return Err(LevelError::Invalid(format!("empty rect for {}", e.id)));
            }
        }
        for b in &data.food_boxes {
            if crate::food::Food::from_id(&b.food).is_none() {
                return Err(LevelError::Invalid(format!("unknown food {}", b.food)));
            }
        }
        if !data.level.ground_walkable {
            return Err(LevelError::Invalid(
                "only ground_walkable = true is supported (Q-046)".into(),
            ));
        }
        Ok(data)
    }

    pub fn element(&self, id: &str) -> Option<&Element> {
        self.elements.iter().find(|e| e.id == id)
    }

    pub fn elements_of(&self, ty: ElementType) -> impl Iterator<Item = &Element> {
        self.elements.iter().filter(move |e| e.ty == ty)
    }

    /// Pairs of solid elements sharing a cell, with the first shared cell (LAYOUT-003).
    pub fn solid_overlaps(&self) -> Vec<(String, String, IVec2)> {
        let solids: Vec<&Element> = self.elements.iter().filter(|e| e.ty.is_solid()).collect();
        let mut out = Vec::new();
        for (i, a) in solids.iter().enumerate() {
            for b in &solids[i + 1..] {
                if let Some(c) = a.rect.cells().find(|c| b.rect.contains(*c)) {
                    out.push((a.id.clone(), b.id.clone(), c));
                }
            }
        }
        out
    }
}

/// What a grid cell is, for movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    OutOfBounds,
    Solid,
    /// Walkable ground with its surface (GAME-LAYOUT, Q-046).
    Walkable(Surface),
    /// Enclosure gate cell; index into `LevelData::elements` of the enclosure. Passable only
    /// while the player leads an animal (GAME-LAYOUT "enclosure").
    Gate(usize),
}

#[derive(Debug, Clone, Copy, Default)]
struct CellInfo {
    solid: Option<u16>,
    path: bool,
    gate: Option<u16>,
    /// The player circle cannot stand on the cell centre because of a prop (GAME-PLAYER §7).
    prop_blocked: bool,
}

/// Walkable grid of a level.
#[derive(Debug, Clone)]
pub struct Grid {
    bounds: Rect,
    cells: Vec<CellInfo>,
    ground: Surface,
}

impl Grid {
    /// Builds the grid; barriers listed in `open_barriers` are treated as removed.
    pub fn build(data: &LevelData, open_barriers: &BTreeSet<String>) -> Self {
        let b = data.level.bounds;
        let mut cells = vec![CellInfo::default(); (b.w * b.d) as usize];
        let index = |c: IVec2| -> Option<usize> {
            b.contains(c)
                .then(|| ((c.y - b.z) * b.w + (c.x - b.x)) as usize)
        };
        for (i, e) in data.elements.iter().enumerate() {
            let open = e.ty == ElementType::Barrier && open_barriers.contains(&e.id);
            for c in e.rect.cells() {
                let Some(k) = index(c) else { continue };
                if e.ty == ElementType::Path {
                    cells[k].path = true;
                } else if e.ty.is_solid() && !open && cells[k].solid.is_none() {
                    cells[k].solid = Some(i as u16);
                }
            }
            if let (ElementType::Enclosure, Some(g)) = (e.ty, e.gate) {
                for c in g.cells() {
                    if let Some(k) = index(c) {
                        cells[k].gate = Some(i as u16);
                    }
                }
            }
        }
        Self {
            bounds: b,
            cells,
            ground: data.level.ground_surface,
        }
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn index(&self, c: IVec2) -> Option<usize> {
        self.bounds
            .contains(c)
            .then(|| ((c.y - self.bounds.z) * self.bounds.w + (c.x - self.bounds.x)) as usize)
    }

    pub fn cell_at(&self, idx: usize) -> IVec2 {
        let w = self.bounds.w as usize;
        IVec2::new(
            self.bounds.x + (idx % w) as i32,
            self.bounds.z + (idx / w) as i32,
        )
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn kind(&self, c: IVec2) -> CellKind {
        let Some(k) = self.index(c) else {
            return CellKind::OutOfBounds;
        };
        let info = self.cells[k];
        match (info.gate, info.solid) {
            (Some(g), _) => CellKind::Gate(g as usize),
            (None, Some(_)) => CellKind::Solid,
            (None, None) if info.path => CellKind::Walkable(Surface::Path),
            (None, None) => CellKind::Walkable(self.ground),
        }
    }

    /// Index of the solid element covering the cell, if any.
    pub fn solid_element(&self, c: IVec2) -> Option<usize> {
        self.index(c)
            .and_then(|k| self.cells[k].solid.map(usize::from))
    }

    /// Whether a `path` element covers the cell (regardless of solids).
    pub fn has_path(&self, c: IVec2) -> bool {
        self.index(c).is_some_and(|k| self.cells[k].path)
    }

    /// Surface of a walkable cell (gates count as `grass`/ground).
    pub fn surface(&self, c: IVec2) -> Option<Surface> {
        match self.kind(c) {
            CellKind::Walkable(s) => Some(s),
            CellKind::Gate(_) => Some(self.ground),
            _ => None,
        }
    }

    /// Marks cells whose centre is blocked by props (used by grid paths, not by the cell
    /// collision itself).
    pub fn set_prop_blocked(&mut self, colliders: &crate::collision::Colliders) {
        for k in 0..self.cells.len() {
            let c = self.cell_at(k);
            self.cells[k].prop_blocked =
                colliders.overlaps(cell_center(c), crate::collision::PLAYER_RADIUS_M);
        }
    }

    /// Whether a prop blocks the cell centre.
    pub fn is_prop_blocked(&self, c: IVec2) -> bool {
        self.index(c).is_some_and(|k| self.cells[k].prop_blocked)
    }

    /// Walkable and not blocked by a prop at the cell centre (grid paths, GAME-PLAYER §7).
    pub fn is_passable(&self, c: IVec2, allow_gates: bool) -> bool {
        self.is_walkable(c, allow_gates) && !self.is_prop_blocked(c)
    }

    pub fn is_walkable(&self, c: IVec2, allow_gates: bool) -> bool {
        match self.kind(c) {
            CellKind::Walkable(_) => true,
            CellKind::Gate(_) => allow_gates,
            _ => false,
        }
    }
}

/// A level at runtime: data plus which barriers are open.
#[derive(Debug, Clone)]
pub struct Level {
    pub data: LevelData,
    open_barriers: BTreeSet<String>,
    grid: Grid,
    colliders: crate::collision::Colliders,
}

impl Level {
    pub fn new(data: LevelData) -> Self {
        let open_barriers = BTreeSet::new();
        let mut grid = Grid::build(&data, &open_barriers);
        let scene = crate::scene::LevelScene::build(&data);
        let colliders =
            crate::collision::Colliders::from_placements(&scene.placements, data.level.bounds);
        grid.set_prop_blocked(&colliders);
        Self {
            data,
            open_barriers,
            grid,
            colliders,
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Prop collision shapes (GAME-PLAYER §7).
    pub fn colliders(&self) -> &crate::collision::Colliders {
        &self.colliders
    }

    /// Ids of the opened barriers (GAME-SAVE).
    pub fn open_barrier_ids(&self) -> Vec<String> {
        self.open_barriers.iter().cloned().collect()
    }

    pub fn is_barrier_open(&self, id: &str) -> bool {
        self.open_barriers.contains(id)
    }

    /// Removes a barrier; its cells become walkable (GAME-LAYOUT §3). Returns false if the id
    /// is not a barrier or already open.
    pub fn open_barrier(&mut self, id: &str) -> bool {
        let is_barrier = self
            .data
            .element(id)
            .is_some_and(|e| e.ty == ElementType::Barrier);
        if !is_barrier || !self.open_barriers.insert(id.to_owned()) {
            return false;
        }
        self.grid = Grid::build(&self.data, &self.open_barriers);
        self.grid.set_prop_blocked(&self.colliders);
        true
    }

    /// Barriers whose transition leaves this level (`<level_id>-><next>`), proposal Q-022.
    pub fn exit_barriers(&self) -> Vec<String> {
        let prefix = format!("{}->", self.data.level.id);
        self.data
            .elements_of(ElementType::Barrier)
            .filter(|e| {
                e.transition
                    .as_deref()
                    .is_some_and(|t| t.starts_with(&prefix))
            })
            .map(|e| e.id.clone())
            .collect()
    }
}

// ------------------------------------------------------------------ modular edges (Q-057)

/// Axis of a straight run in level coordinates; runs always go towards +x (east) or +z
/// (north) from their start (GAME-LAYOUT "Modular edges").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunAxis {
    X,
    Z,
}

impl RunAxis {
    /// Unit direction in level coordinates.
    pub fn dir(self) -> Vec2 {
        match self {
            RunAxis::X => Vec2::X,
            RunAxis::Z => Vec2::Y,
        }
    }
}

/// One straight modular piece of a run: `length_m` is 2 (`fence_wood`, `hedge`, `zoo_wall`)
/// or 1 (`*_1m`), starting `offset_m` metres from the run start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub offset_m: u32,
    pub length_m: u32,
}

/// Fill rule of GAME-LAYOUT "Modular edges" (Q-057): 2 m segments from the start, plus one
/// 1 m segment at the end when the length is odd.
pub fn segment_run(length_m: u32) -> Vec<Segment> {
    let mut out = Vec::with_capacity(length_m.div_ceil(2) as usize);
    let mut offset_m = 0;
    while offset_m + 2 <= length_m {
        out.push(Segment {
            offset_m,
            length_m: 2,
        });
        offset_m += 2;
    }
    if offset_m < length_m {
        out.push(Segment {
            offset_m,
            length_m: 1,
        });
    }
    out
}

/// A straight run of modular pieces: `start` is on the run's centre line (level metres),
/// the run extends `length_m` along `axis`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Run {
    pub start: Vec2,
    pub axis: RunAxis,
    pub length_m: u32,
}

impl Run {
    pub fn end(&self) -> Vec2 {
        self.start + self.axis.dir() * self.length_m as f32
    }

    pub fn segments(&self) -> Vec<Segment> {
        segment_run(self.length_m)
    }

    /// Centre of a segment in level coordinates (where the origin of a straight piece goes).
    pub fn segment_center(&self, s: Segment) -> Vec2 {
        self.start + self.axis.dir() * (s.offset_m as f32 + s.length_m as f32 / 2.0)
    }
}

/// Hedge/wall band → one run on the band's centre line (GAME-LAYOUT "Modular edges",
/// proposal). The run is parallel to the level border the band touches; a band touching no
/// border or two borders runs along its longer side. `None` if the direction is ambiguous
/// (square band off the border) or the band is deeper than 2 cells.
pub fn band_run(rect: Rect, bounds: Rect) -> Option<Run> {
    let x_border = rect.x == bounds.x || rect.x + rect.w == bounds.x + bounds.w;
    let z_border = rect.z == bounds.z || rect.z + rect.d == bounds.z + bounds.d;
    let axis = match (x_border, z_border) {
        (true, false) => RunAxis::Z,
        (false, true) => RunAxis::X,
        _ if rect.w > rect.d => RunAxis::X,
        _ if rect.d > rect.w => RunAxis::Z,
        _ => return None,
    };
    let (start, length, depth) = match axis {
        RunAxis::X => (
            Vec2::new(rect.x as f32, rect.z as f32 + rect.d as f32 / 2.0),
            rect.w,
            rect.d,
        ),
        RunAxis::Z => (
            Vec2::new(rect.x as f32 + rect.w as f32 / 2.0, rect.z as f32),
            rect.d,
            rect.w,
        ),
    };
    (depth <= 2 && length > 0).then_some(Run {
        start,
        axis,
        length_m: length as u32,
    })
}

/// Fence of an enclosure (GAME-LAYOUT "Modular edges", proposal): corner pieces at the four
/// corners of the rectangle outline, a 2 m gate opening on one side and the straight runs
/// in between.
#[derive(Debug, Clone, PartialEq)]
pub struct FenceLayout {
    /// SW, SE, NE, NW corner points (level metres).
    pub corners: [Vec2; 4],
    /// The 2 m gate opening, as a run along its side.
    pub gate: Option<Run>,
    /// Straight runs between corner arms and the gate.
    pub runs: Vec<Run>,
}

/// Lays out the fence along the outline of `rect`; `gate` = the gate cells (inside `rect`,
/// one cell deep, on exactly one side). Errors if the gate is not 2 m wide, not on a side,
/// or overlaps a corner arm.
pub fn enclosure_fence(rect: Rect, gate: Option<Rect>) -> Result<FenceLayout, String> {
    if rect.w < 2 || rect.d < 2 {
        return Err(format!("enclosure {rect:?} too small for corner pieces"));
    }
    let (x0, z0, x1, z1) = (rect.x, rect.z, rect.x + rect.w, rect.z + rect.d);
    // sides: (fixed coordinate, from, to, axis); runs go towards +x / +z
    let sides = [
        (z0, x0, x1, RunAxis::X), // south
        (z1, x0, x1, RunAxis::X), // north
        (x0, z0, z1, RunAxis::Z), // west
        (x1, z0, z1, RunAxis::Z), // east
    ];
    let gate_side = match gate {
        None => None,
        Some(g) => {
            let inside = g.x >= x0 && g.z >= z0 && g.x + g.w <= x1 && g.z + g.d <= z1;
            let on = [
                g.d == 1 && g.z == z0,
                g.d == 1 && g.z + 1 == z1,
                g.w == 1 && g.x == x0,
                g.w == 1 && g.x + 1 == x1,
            ];
            let hits: Vec<usize> = (0..4).filter(|&i| on[i]).collect();
            if !inside || hits.len() != 1 {
                return Err(format!("gate {g:?} is not on exactly one side of {rect:?}"));
            }
            let i = hits[0];
            let (g0, g1) = match sides[i].3 {
                RunAxis::X => (g.x, g.x + g.w),
                RunAxis::Z => (g.z, g.z + g.d),
            };
            if g1 - g0 != 2 {
                return Err(format!("gate {g:?} is {} m wide, expected 2 m", g1 - g0));
            }
            if g0 < sides[i].1 + 1 || g1 > sides[i].2 - 1 {
                return Err(format!("gate {g:?} overlaps a corner arm"));
            }
            Some((i, g0, g1))
        }
    };
    let point = |axis: RunAxis, fixed: i32, along: i32| match axis {
        RunAxis::X => Vec2::new(along as f32, fixed as f32),
        RunAxis::Z => Vec2::new(fixed as f32, along as f32),
    };
    let mut runs = Vec::new();
    let mut gate_run = None;
    for (i, &(fixed, from, to, axis)) in sides.iter().enumerate() {
        let (a, b) = (from + 1, to - 1);
        let mut spans = vec![(a, b)];
        if let Some((gi, g0, g1)) = gate_side {
            if gi == i {
                spans = vec![(a, g0), (g1, b)];
                gate_run = Some(Run {
                    start: point(axis, fixed, g0),
                    axis,
                    length_m: 2,
                });
            }
        }
        for (s, e) in spans {
            if e > s {
                runs.push(Run {
                    start: point(axis, fixed, s),
                    axis,
                    length_m: (e - s) as u32,
                });
            }
        }
    }
    Ok(FenceLayout {
        corners: [
            Vec2::new(x0 as f32, z0 as f32),
            Vec2::new(x1 as f32, z0 as f32),
            Vec2::new(x1 as f32, z1 as f32),
            Vec2::new(x0 as f32, z1 as f32),
        ],
        gate: gate_run,
        runs,
    })
}

#[cfg(test)]
mod modular_tests {
    use super::*;

    // LAYOUT-012
    #[test]
    fn layout_012_segment_run_small_cases() {
        assert!(segment_run(0).is_empty());
        let one = segment_run(1);
        assert_eq!(
            one,
            vec![Segment {
                offset_m: 0,
                length_m: 1
            }]
        );
        let three: Vec<u32> = segment_run(3).iter().map(|s| s.length_m).collect();
        assert_eq!(three, vec![2, 1]);
        let four: Vec<u32> = segment_run(4).iter().map(|s| s.length_m).collect();
        assert_eq!(four, vec![2, 2]);
    }

    // LAYOUT-013 (fence rules on a synthetic enclosure)
    #[test]
    fn layout_013_fence_rejects_bad_gates() {
        let r = Rect::new(0, 0, 10, 8);
        assert!(enclosure_fence(r, Some(Rect::new(0, 3, 1, 3))).is_err()); // 3 m wide
        assert!(enclosure_fence(r, Some(Rect::new(0, 0, 1, 2))).is_err()); // corner arm
        assert!(enclosure_fence(r, Some(Rect::new(4, 3, 1, 2))).is_err()); // not on a side
        let f = enclosure_fence(r, Some(Rect::new(4, 0, 2, 1))).unwrap();
        let total: u32 = f.runs.iter().map(|r| r.length_m).sum();
        // perimeter 36 m = 4 corners x 2 arms + 2 m gate + runs
        assert_eq!(total, 36 - 8 - 2);
    }
}
