//! Level layout data (GAME-LAYOUT, GAME-LEVEL-1) and the walkable grid derived from it.
//!
//! Grid: 1 m cells, `+X` east, `+Z` north. A cell `(x, z)` covers `[x, x+1) × [z, z+1)`;
//! world positions are `Vec2(x, z)` in metres.

use std::collections::BTreeSet;

use glam::{IVec2, Vec2};
use serde::Deserialize;

/// Grid rectangle `[x, z, w, d]` (south-west corner + size), covering cells
/// `x .. x+w-1`, `z .. z+d-1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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

    /// Distance in metres from a world position to the nearest point of the rectangle's area.
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

/// Centre of a grid cell in world metres.
pub fn cell_center(c: IVec2) -> Vec2 {
    Vec2::new(c.x as f32 + 0.5, c.y as f32 + 0.5)
}

/// Grid cell containing a world position.
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

/// Parsed `assets/levels/level-<N>.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub level: LevelHeader,
    pub spawn: Spawn,
    #[serde(rename = "element")]
    pub elements: Vec<Element>,
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
}

impl Level {
    pub fn new(data: LevelData) -> Self {
        let open_barriers = BTreeSet::new();
        let grid = Grid::build(&data, &open_barriers);
        Self {
            data,
            open_barriers,
            grid,
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
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
