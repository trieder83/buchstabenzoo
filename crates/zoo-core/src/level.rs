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
    /// Enclosures: the feeding spot (cells just inside the fence on the gate side, 1-2 m
    /// wide) where the animals come to be given a treat; derived next to the gate when
    /// absent (GAME-GARDEN §6).
    pub feed_spot: Option<Rect>,
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
    /// Barriers: the level whose completion unlocks it (moon door: its nightfall, Q-133).
    pub unlock_after: Option<String>,
    /// Barriers: when an unlocked barrier opens - `night` (night-level gates such as
    /// `barrier_n1_garden`: the moment `unlock_after` is complete, GAME-LEVEL-NIGHT-2, Q-331),
    /// `morning` (day barriers: the next morning). Absent = by the barrier's own rule.
    pub opens_at: Option<String>,
    /// Bamboo forests: bamboo can be cut at its `[[cut_spot]]`s (GAME-FEED §14).
    #[serde(default)]
    pub harvestable: bool,
    #[serde(default)]
    pub blocks_view: bool,
    pub height_m: Option<f32>,
    /// Tree areas (`trees`, `tree_grove`): `dense` (solid as a whole) or `sparse` (walkable
    /// between the listed trees) — GAME-LAYOUT "Forests", proposal Q-085.
    pub density: Option<String>,
    /// Dense tree areas: visible border on the walkable sides (`bushes`), Q-085.
    pub edge: Option<String>,
    /// Sparse tree areas: every tree/bush (trunk centre, level coordinates) and its model.
    #[serde(default)]
    pub trees: Vec<TreeSpot>,
    /// Enclosures: surfaces the animal wanders on when home (`grass`, `water`), default
    /// `["grass"]` (GAME-LAYOUT "Enclosure features and wandering at home", Q-085).
    #[serde(default)]
    pub home_wander_on: Vec<String>,
    /// Enterable buildings (proposal Q-092): walkable interior cells (surface `path`); with
    /// the `door` cell they are the only walkable cells of the building rect.
    pub interior: Option<Rect>,
    /// Enterable buildings with a round plan (the terrarium house, LAYOUT-N2-020): further
    /// walkable interior rects (a stepped arc is a union of rects).
    #[serde(default)]
    pub interior_extra: Vec<Rect>,
    /// Enterable buildings with a round plan: further solid footprint rects besides `rect`
    /// (the wall mass between the hall and the outer arc); disjoint from every enclosure.
    #[serde(default)]
    pub footprint_extra: Vec<Rect>,
    /// Enterable buildings: footprint of the whole model when it is larger than the walkable
    /// building rect (the night house with its indoor enclosure wing, proposal Q-134).
    pub model_rect: Option<Rect>,
    /// Enclosures inside a building (the night house, proposal Q-134).
    #[serde(default)]
    pub indoor: bool,
    /// Indoor enclosures: a terrarium (glass case with heat / UV lamp, GAME-LEVEL-NIGHT-2,
    /// Q-333).
    #[serde(default)]
    pub terrarium: bool,
    /// Enclosures: the species lives here as a pair (GAME-FAMILY; data flag, off until the
    /// female model exists).
    #[serde(default)]
    pub pair: bool,
    /// Rivers and streams: flow direction `N` / `E` / `S` / `W` (level coordinates;
    /// GAME-LAYOUT "Water", Q-066). Required on every `river` / `stream` element.
    pub flow: Option<String>,
    /// Info boards: `mount = "wall"` — a flat board on the building facade behind its cell
    /// instead of a standing board: not solid, no footprint, so no pocket forms between the
    /// board and the wall beside a door (Q-157, LAYOUT-038).
    #[serde(default)]
    pub mount: Option<String>,
    /// Index of the level part the element comes from ([`LevelData::parts`]).
    #[serde(skip)]
    pub part: usize,
}

/// One tree or bush of a `sparse` tree area.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TreeSpot {
    pub pos: [f32; 2],
    pub model: String,
}

impl TreeSpot {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }
}

impl Element {
    /// Solid for the player: every type except `path` and `hiding_place`, and except `sparse`
    /// tree areas (only their trunks/bushes are solid, as prop colliders — GAME-LAYOUT
    /// "Forests").
    pub fn is_solid(&self) -> bool {
        self.ty.is_solid() && !self.is_sparse() && !self.is_wall_board()
    }

    /// An info board mounted flat on a facade (`mount = "wall"`, Q-157): not solid.
    pub fn is_wall_board(&self) -> bool {
        self.kind.as_deref() == Some("info_board") && self.mount.as_deref() == Some("wall")
    }

    /// A `sparse` tree area.
    pub fn is_sparse(&self) -> bool {
        self.density.as_deref() == Some("sparse")
    }

    /// Surfaces of the home wander area (default `grass`).
    pub fn home_surfaces(&self) -> Vec<&str> {
        if self.home_wander_on.is_empty() {
            vec!["grass"]
        } else {
            self.home_wander_on.iter().map(String::as_str).collect()
        }
    }

    pub fn door_cell(&self) -> Option<IVec2> {
        self.door.map(|d| IVec2::new(d[0], d[1]))
    }

    /// An enterable building (Q-092): a `building` with an `interior` rect.
    pub fn is_enterable(&self) -> bool {
        self.ty == ElementType::Building && self.interior.is_some()
    }

    /// Whether a cell of an enterable building is walkable (interior or door cell).
    pub fn is_open_cell(&self, c: IVec2) -> bool {
        self.is_interior_cell(c) && self.ty == ElementType::Building
            || (self.is_enterable() && self.door_cell() == Some(c))
    }

    /// A cell of the `interior` rect or of one of the `interior_extra` rects.
    pub fn is_interior_cell(&self, c: IVec2) -> bool {
        self.interior.is_some_and(|r| r.contains(c))
            || self.interior_extra.iter().any(|r| r.contains(c))
    }

    /// All walkable interior cells (the `interior` rect plus the extra rects).
    pub fn interior_cells(&self) -> Vec<IVec2> {
        self.interior
            .iter()
            .chain(&self.interior_extra)
            .flat_map(|r| r.cells())
            .collect()
    }

    /// Every cell of the footprint: `rect` plus the `footprint_extra` rects.
    pub fn footprint_cells(&self) -> impl Iterator<Item = IVec2> + '_ {
        std::iter::once(self.rect)
            .chain(self.footprint_extra.iter().copied())
            .flat_map(|r| r.cells())
    }

    /// Whether a cell belongs to the footprint (`rect` or `footprint_extra`).
    pub fn footprint_contains(&self, c: IVec2) -> bool {
        self.rect.contains(c) || self.footprint_extra.iter().any(|r| r.contains(c))
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
    /// Missions in scope (interactable) in this level; empty = every enclosure's animal
    /// (Q-069: only in-scope missions are interactable).
    #[serde(default)]
    pub missions: Vec<String>,
    /// `night` for a night level (GAME-NIGHT rule 4, proposal Q-133): reachable only at
    /// night through the moon door.
    pub time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spawn {
    pub cell: [i32; 2],
    pub facing: String,
}

/// A level entry (`[[entry]]`, GAME-LAYOUT "Joining levels", proposal Q-088): cells of this
/// level edge-adjacent to a barrier of an earlier level; the only walkable border cells.
#[derive(Debug, Clone, Deserialize)]
pub struct EntryData {
    pub id: String,
    pub cells: Rect,
    pub from_level: String,
    pub barrier: String,
}

/// A carryable item (`[[item]]`, proposal Q-093), e.g. the fish bowl.
#[derive(Debug, Clone, Deserialize)]
pub struct ItemData {
    pub id: String,
    pub kind: String,
    pub pos: [f32; 2],
    /// The side the child uses / reads (`+x`, `-x`, `+z`, `-z`).
    #[serde(default)]
    pub facing: Option<String>,
    /// The walkable cell the child stands on to use it (tests, scripted player).
    #[serde(default)]
    pub stand: Option<[i32; 2]>,
    pub building: Option<String>,
    /// The animal that needs this item to be carried home (GAME-RESCUE "goldfish bowl").
    pub animal: Option<String>,
    #[serde(skip)]
    pub part: usize,
}

impl ItemData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }
}

/// A drivable golf cart with its parking spot (`[[cart]]`, GAME-CART "Data shape").
#[derive(Debug, Clone, Deserialize)]
pub struct CartData {
    pub id: String,
    /// Centre of the parked cart (level metres).
    pub pos: [f32; 2],
    /// Parking heading (`+x`, `-x`, `+z`, `-z`).
    pub facing: String,
    /// Parking footprint cells (x, z, w, d); the oriented box lies inside.
    pub rect: Rect,
    /// Walkable boarding cell.
    pub stand: [i32; 2],
    /// The parking sign.
    pub sign_pos: [f32; 2],
    /// Level id that must be open (`""` = always).
    #[serde(default)]
    pub locked_until: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub notes: String,
    #[serde(skip)]
    pub part: usize,
}

impl CartData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }

    pub fn heading(&self) -> Vec2 {
        facing_vec(&self.facing)
    }

    pub fn stand_cell(&self) -> IVec2 {
        IVec2::from(self.stand)
    }
}

/// Furniture inside a building (`[[prop]]`, proposal Q-137): a model at a position; drawn as
/// a placeholder until the model exists.
#[derive(Debug, Clone, Deserialize)]
pub struct PropData {
    pub id: String,
    pub model: String,
    pub pos: [f32; 2],
    #[serde(default = "default_south")]
    pub facing: String,
    /// Footprint (m): across and along `facing`.
    pub size_m: Option<[f32; 2]>,
    pub building: Option<String>,
    #[serde(skip)]
    pub part: usize,
}

impl PropData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }
}

/// A fenced vegetable garden (`[[garden]]`, GAME-GARDEN, data proposal Q-102): its cells
/// stay walkable; the fence runs (cell-edge lines) block the player and the grid crossings,
/// the gate opens by itself, animals never enter.
#[derive(Debug, Clone, Deserialize)]
pub struct GardenData {
    pub id: String,
    pub rect: Rect,
    /// The gate opening (cells just inside it, on the `gate_side` fence line).
    pub gate: Rect,
    #[serde(default = "default_south")]
    pub gate_side: String,
    #[serde(default)]
    pub gate_model: Option<String>,
    #[serde(default)]
    pub fence_model: Option<String>,
    /// Fence lines `[x0, z0, x1, z1]` on cell edges (level metres).
    #[serde(default)]
    pub fence_runs: Vec<[f32; 4]>,
    #[serde(default)]
    pub fence_inset_m: f32,
    #[serde(default)]
    pub edges_closed_by: Vec<String>,
    #[serde(default)]
    pub animals_enter: bool,
    /// Decoration with a collider (wheelbarrow, watering can).
    #[serde(default)]
    pub props: Vec<GardenProp>,
    #[serde(skip)]
    pub part: usize,
}

/// A decoration model of a garden (`props` of `[[garden]]`).
#[derive(Debug, Clone, Deserialize)]
pub struct GardenProp {
    pub model: String,
    pub pos: [f32; 2],
    #[serde(default = "default_south")]
    pub facing: String,
}

impl GardenData {
    /// Centre of the gate opening on the fence line (level metres).
    pub fn gate_center(&self) -> Vec2 {
        let g = self.gate;
        let c = Vec2::new(g.x as f32 + g.w as f32 / 2.0, g.z as f32 + g.d as f32 / 2.0);
        match self.gate_side.as_str() {
            "-z" => Vec2::new(c.x, g.z as f32),
            "+z" => Vec2::new(c.x, (g.z + g.d) as f32),
            "-x" => Vec2::new(g.x as f32, c.y),
            _ => Vec2::new((g.x + g.w) as f32, c.y),
        }
    }

    /// Unit direction out of the garden through the gate (level).
    pub fn gate_out(&self) -> Vec2 {
        facing_vec(&self.gate_side)
    }
}

/// A raised soil bed of a garden (`[[garden_bed]]`).
#[derive(Debug, Clone, Deserialize)]
pub struct GardenBedData {
    pub id: String,
    pub garden: String,
    pub rect: Rect,
    /// `carrot` | `potato` | `apple` | `orange` (a treat id).
    pub plant: String,
    pub sign_pos: [f32; 2],
    #[serde(default = "default_south")]
    pub sign_facing: String,
    pub sign_key: String,
    #[serde(skip)]
    pub part: usize,
}

/// One harvestable plant (`[[plant_spot]]`, GAME-GARDEN §3).
#[derive(Debug, Clone, Deserialize)]
pub struct PlantSpotData {
    pub id: String,
    pub garden: String,
    pub bed: String,
    /// `carrot` | `potato` | `apple` | `orange` (a treat id).
    pub kind: String,
    pub pos: [f32; 2],
    /// `empty` | `sprout` | `young` | `ripe` in a new game.
    #[serde(default = "default_ripe")]
    pub start_stage: String,
    /// The walkable cell in front of the plant.
    pub stand: [i32; 2],
    #[serde(skip)]
    pub part: usize,
}

impl PlantSpotData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }
}

/// A place at the edge of a harvestable bamboo forest where a stalk can be snapped off
/// (`[[cut_spot]]`, GAME-FEED §14–15, Q-156).
#[derive(Debug, Clone, Deserialize)]
pub struct CutSpotData {
    pub id: String,
    /// The bamboo element (`harvestable = true`).
    pub forest: String,
    /// Foot of the cut stalk (level metres), at the forest edge.
    pub pos: [f32; 2],
    /// Where the child stands to cut it (walkable, within 1.5 m).
    pub stand: [f32; 2],
    #[serde(skip)]
    pub part: usize,
}

impl CutSpotData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }

    pub fn stand(&self) -> Vec2 {
        Vec2::from(self.stand)
    }
}

fn default_ripe() -> String {
    "ripe".to_owned()
}

/// A place where the fish bowl is filled (`[[water_source]]`, proposal Q-093): `tap` = prop
/// at `pos`; `bank` = every walkable cell edge-adjacent to the water element `water`.
#[derive(Debug, Clone, Deserialize)]
pub struct WaterSourceData {
    pub id: String,
    pub kind: String,
    pub pos: Option<[f32; 2]>,
    pub facing: Option<String>,
    pub water: Option<String>,
    #[serde(skip)]
    pub part: usize,
}

/// A lamp of the night (`[[light]]`, GAME-NIGHT rule 1, Q-118): lantern posts along the
/// paths, string lights over plazas, wall and board lamps. Night-only props (hidden by day).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LightData {
    #[serde(default)]
    pub id: String,
    /// `lantern_post` (default), `string_lights`, `wall_lamp`, `board_lamp`, `ceiling`.
    #[serde(default)]
    pub kind: String,
    /// Where it stands (level metres).
    pub pos: Option<[f32; 2]>,
    /// Alternative to `pos`: a cell (its centre).
    pub cell: Option<[i32; 2]>,
    /// Direction of the lamp arm / the lit side (`+z`, `-z`, `+x`, `-x`).
    pub facing: Option<String>,
    pub height_m: Option<f32>,
    pub radius_m: Option<f32>,
    /// String lights: the posts / anchor points the string hangs between.
    #[serde(default)]
    pub points: Vec<[f32; 2]>,
    pub from: Option<[f32; 2]>,
    pub to: Option<[f32; 2]>,
    /// The element the lamp hangs on (a building for `wall_lamp`, a board for `board_lamp`).
    pub attach: Option<String>,
    /// Light colour `#RRGGBB` (indoor lights), default warm lamp light.
    pub color: Option<String>,
    #[serde(skip)]
    pub part: usize,
}

impl LightData {
    pub fn pos(&self) -> Option<Vec2> {
        self.pos
            .map(Vec2::from)
            .or_else(|| self.cell.map(|c| cell_center(IVec2::from(c))))
            .or_else(|| self.points.first().map(|p| Vec2::from(*p)))
            .or_else(|| self.from.map(Vec2::from))
    }

    /// String light anchor points.
    pub fn points(&self) -> Vec<Vec2> {
        if !self.points.is_empty() {
            return self.points.iter().map(|p| Vec2::from(*p)).collect();
        }
        match (self.from.or(self.pos), self.to) {
            (Some(a), Some(b)) => vec![Vec2::from(a), Vec2::from(b)],
            _ => self.pos().into_iter().collect(),
        }
    }

    /// `color` as sRGB floats.
    pub fn color_rgb(&self) -> Option<[f32; 3]> {
        let h = self.color.as_deref()?.trim_start_matches('#');
        let v = u32::from_str_radix(h, 16).ok()?;
        Some([
            ((v >> 16) & 0xFF) as f32 / 255.0,
            ((v >> 8) & 0xFF) as f32 / 255.0,
            (v & 0xFF) as f32 / 255.0,
        ])
    }
}

/// One level file inside a (joined) [`LevelData`] (GAME-LAYOUT "Joining levels").
#[derive(Debug, Clone)]
pub struct LevelPart {
    pub id: String,
    pub bounds: Rect,
    pub spawn: Spawn,
    /// Missions in scope of this level: `[level] missions`, else the animal of every
    /// enclosure of the level (Q-069).
    pub missions: Vec<String>,
    pub entries: Vec<EntryData>,
    /// A night level (`time = "night"`, or an id `night_*`; GAME-NIGHT).
    pub night: bool,
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
    #[serde(skip)]
    pub part: usize,
}

impl FoodBoxData {
    pub fn pos(&self) -> Vec2 {
        Vec2::new(self.pos[0], self.pos[1])
    }

    pub fn facing(&self) -> Vec2 {
        facing_vec(&self.facing)
    }
}

/// One candidate hiding place (`[[hiding_place]]`, GAME-LAYOUT "Hiding places", Q-080).
#[derive(Debug, Clone, Deserialize)]
pub struct HidingPlaceData {
    pub id: String,
    pub animal: String,
    /// Overlay area (not solid); contains the spot, every wander cell and the scenery.
    pub rect: Rect,
    /// Cell where the animal starts and is found.
    pub animal_spot: [i32; 2],
    /// Escaped animals wander to cells whose centre is within this distance of the spot
    /// centre (≤ 3 m, GAME-ANIMALS).
    #[serde(default = "default_wander_radius")]
    pub wander_radius_m: f32,
    /// Surface the animal wanders on: `grass`, `water`, `cave`.
    #[serde(default = "default_grass")]
    pub wander_on: String,
    /// Water kinds for `wander_on = "water"` (landmark kinds, default pond/river/stream).
    #[serde(default)]
    pub water_kinds: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub scenery: Vec<String>,
    /// Clip played at the place when not walking (proposal, Q-043).
    pub pose: Option<String>,
    /// The animal sits up in a tree / on the ship at this height above its spot and does not
    /// wander (proposal Q-094).
    pub perch_height_m: Option<f32>,
    #[serde(skip)]
    pub part: usize,
}

fn default_wander_radius() -> f32 {
    3.0
}

fn default_grass() -> String {
    "grass".to_owned()
}

impl HidingPlaceData {
    pub fn spot_cell(&self) -> IVec2 {
        IVec2::new(self.animal_spot[0], self.animal_spot[1])
    }

    pub fn spot(&self) -> Vec2 {
        cell_center(self.spot_cell())
    }
}

/// Non-solid ground dressing a riddle relies on (`[[scenery]]`, Q-080).
#[derive(Debug, Clone, Deserialize)]
pub struct SceneryData {
    pub id: String,
    pub kind: String,
    pub rect: Rect,
    pub hiding_place: Option<String>,
    #[serde(default)]
    pub props: Vec<String>,
}

/// Something inside an enclosure that changes how its animal moves at home
/// (`[[enclosure_feature]]`, proposal Q-085): `pool` (water with an entry ramp) or `hut`
/// (reserved building area).
#[derive(Debug, Clone, Deserialize)]
pub struct EnclosureFeature {
    pub id: String,
    pub enclosure: String,
    pub kind: String,
    pub rect: Rect,
    pub water: Option<String>,
    pub ramp: Option<Rect>,
    pub ramp_side: Option<String>,
    #[serde(default)]
    pub edge_stones: Vec<[f32; 2]>,
    pub model: Option<String>,
    /// Animal house (GAME-HOUSE): the room, the doorway and its facing, heights.
    pub interior: Option<Rect>,
    pub door: Option<Rect>,
    pub door_side: Option<String>,
    pub door_height_m: Option<f32>,
    pub roof_height_m: Option<f32>,
    /// Animals may rest inside (default true).
    pub rest: Option<bool>,
}

impl EnclosureFeature {
    pub fn is_pool(&self) -> bool {
        self.kind == "pool"
    }

    /// An animal house with a door (GAME-HOUSE).
    pub fn is_animal_house(&self) -> bool {
        self.kind == "animal_house"
    }

    /// Whether a cell is part of the pool's entry ramp.
    pub fn is_ramp(&self, c: IVec2) -> bool {
        self.ramp.is_some_and(|r| r.contains(c))
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
    /// Candidate hiding places (discovery, GAME-RESCUE §1).
    #[serde(default, rename = "hiding_place")]
    pub hiding_places: Vec<HidingPlaceData>,
    /// Non-solid riddle dressing.
    #[serde(default, rename = "scenery")]
    pub scenery: Vec<SceneryData>,
    /// Enclosure pools and reserved areas.
    #[serde(default, rename = "enclosure_feature")]
    pub enclosure_features: Vec<EnclosureFeature>,
    /// Level entries (proposal Q-088).
    #[serde(default, rename = "entry")]
    pub entries: Vec<EntryData>,
    /// Carryable items (proposal Q-093).
    #[serde(default, rename = "item")]
    pub items: Vec<ItemData>,
    /// Water sources for the fish bowl (proposal Q-093).
    #[serde(default, rename = "water_source")]
    pub water_sources: Vec<WaterSourceData>,
    /// Lamps of the night (GAME-NIGHT, Q-118).
    #[serde(default, rename = "light")]
    pub lights: Vec<LightData>,
    /// Furniture (proposal Q-137).
    #[serde(default, rename = "prop")]
    pub props: Vec<PropData>,
    /// Golf carts and their parking spots (GAME-CART).
    #[serde(default, rename = "cart")]
    pub carts: Vec<CartData>,
    /// Ad billboards (GAME-ADS rule 1).
    #[serde(default, rename = "ad_board")]
    pub ad_boards: Vec<crate::ads::AdBoardData>,
    /// Vegetable gardens (GAME-GARDEN, proposal Q-102).
    #[serde(default, rename = "garden")]
    pub gardens: Vec<GardenData>,
    #[serde(default, rename = "garden_bed")]
    pub garden_beds: Vec<GardenBedData>,
    #[serde(default, rename = "plant_spot")]
    pub plant_spots: Vec<PlantSpotData>,
    /// Bamboo cut spots of harvestable bamboo forests (GAME-FEED §14).
    #[serde(default, rename = "cut_spot")]
    pub cut_spots: Vec<CutSpotData>,
    /// The level files joined into this data (one for a single level file).
    #[serde(skip)]
    pub parts: Vec<LevelPart>,
}

/// Id of the joined zoo (all day levels in one grid, GAME-LAYOUT "Joining levels").
pub const ZOO_ID: &str = "zoo";

/// Water landmark kinds (not walkable; hiding places `wander_on = "water"`; their banks fill
/// the fish bowl, proposal Q-093).
pub const WATER_KINDS: [&str; 4] = ["pond", "river", "stream", "fountain"];

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
            let flowing = e.ty == ElementType::Landmark
                && matches!(e.kind.as_deref(), Some("river" | "stream"));
            match (flowing, e.flow.as_deref()) {
                (true, Some("N" | "E" | "S" | "W")) | (false, None) => {}
                (true, _) => {
                    return Err(LevelError::Invalid(format!(
                        "{}: rivers and streams need flow = \"N\"|\"E\"|\"S\"|\"W\"",
                        e.id
                    )))
                }
                (false, Some(_)) => {
                    return Err(LevelError::Invalid(format!(
                        "{}: only rivers and streams have a flow",
                        e.id
                    )))
                }
            }
        }
        let mut place_ids = BTreeSet::new();
        for h in &data.hiding_places {
            if !place_ids.insert(h.id.as_str()) {
                return Err(LevelError::Invalid(format!(
                    "duplicate hiding place {}",
                    h.id
                )));
            }
            if !h.rect.contains(h.spot_cell()) {
                return Err(LevelError::Invalid(format!("{}: spot outside rect", h.id)));
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
        let mut data = data;
        let missions = if data.level.missions.is_empty() {
            data.elements_of(ElementType::Enclosure)
                .filter_map(|e| e.animal.clone())
                .collect()
        } else {
            data.level.missions.clone()
        };
        data.parts = vec![LevelPart {
            id: data.level.id.clone(),
            bounds: data.level.bounds,
            spawn: data.spawn.clone(),
            missions,
            entries: data.entries.clone(),
            night: data.level.time.as_deref() == Some("night")
                || crate::daytime::is_night_level(&data.level.id),
        }];
        Ok(data)
    }

    /// Joins level files into one continuous zoo (GAME-LAYOUT "Joining levels", proposal
    /// Q-088): same coordinates, disjoint bounds, the union of all elements. The first level
    /// gives the spawn; ids must be unique over all levels. `parts` keeps each level's id,
    /// bounds, spawn, missions and entries (the order of `levels`).
    pub fn join(levels: Vec<LevelData>) -> Result<Self, LevelError> {
        let mut it = levels.into_iter();
        let Some(mut out) = it.next() else {
            return Err(LevelError::Invalid("no level to join".into()));
        };
        for next in it {
            let k = out.parts.len();
            for p in &next.parts {
                if out.parts.iter().any(|q| rects_overlap(q.bounds, p.bounds)) {
                    return Err(LevelError::Invalid(format!(
                        "bounds of {} overlap another level",
                        p.id
                    )));
                }
            }
            for e in &next.elements {
                if out.element(&e.id).is_some() {
                    return Err(LevelError::Invalid(format!(
                        "duplicate element id {}",
                        e.id
                    )));
                }
            }
            for h in &next.hiding_places {
                if out.hiding_place(&h.id).is_some() {
                    return Err(LevelError::Invalid(format!(
                        "duplicate hiding place {}",
                        h.id
                    )));
                }
            }
            let shift = |p: usize| p + k;
            out.elements.extend(next.elements.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.food_boxes
                .extend(next.food_boxes.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.hiding_places
                .extend(next.hiding_places.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.items.extend(next.items.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.water_sources
                .extend(next.water_sources.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.props.extend(next.props.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.carts.extend(next.carts.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.ad_boards
                .extend(next.ad_boards.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.lights.extend(next.lights.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.gardens.extend(next.gardens.into_iter().map(|mut e| {
                e.part = shift(e.part);
                e
            }));
            out.garden_beds
                .extend(next.garden_beds.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.plant_spots
                .extend(next.plant_spots.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.cut_spots
                .extend(next.cut_spots.into_iter().map(|mut e| {
                    e.part = shift(e.part);
                    e
                }));
            out.scenery.extend(next.scenery);
            out.enclosure_features.extend(next.enclosure_features);
            out.entries.extend(next.entries);
            out.parts.extend(next.parts);
        }
        if out.parts.len() > 1 {
            let mut b = out.parts[0].bounds;
            for p in &out.parts[1..] {
                b = rect_union(b, p.bounds);
            }
            out.level.id = ZOO_ID.to_owned();
            out.level.spec = None;
            out.level.bounds = b;
            out.level.missions = out.parts.iter().flat_map(|p| p.missions.clone()).collect();
        }
        Ok(out)
    }

    /// Index of the level part whose bounds contain the cell.
    pub fn part_at(&self, c: IVec2) -> Option<usize> {
        self.parts.iter().position(|p| p.bounds.contains(c))
    }

    /// Index of a level part by its id.
    pub fn part_index(&self, id: &str) -> Option<usize> {
        self.parts.iter().position(|p| p.id == id)
    }

    /// Whether level part `k` is a night level (GAME-NIGHT rule 4).
    pub fn is_night_part(&self, k: usize) -> bool {
        self.parts.get(k).is_some_and(|p| p.night)
    }

    /// Level part of the enclosure of an animal (by the enclosure's `animal`).
    pub fn part_of_animal(&self, animal: &str) -> Option<usize> {
        self.elements
            .iter()
            .find(|e| e.ty == ElementType::Enclosure && e.animal.as_deref() == Some(animal))
            .map(|e| e.part)
    }

    pub fn element(&self, id: &str) -> Option<&Element> {
        self.elements.iter().find(|e| e.id == id)
    }

    /// A candidate hiding place by id.
    pub fn hiding_place(&self, id: &str) -> Option<&HidingPlaceData> {
        self.hiding_places.iter().find(|h| h.id == id)
    }

    /// Candidate hiding places of an animal, in data order.
    pub fn hiding_places_of<'a>(
        &'a self,
        animal: &'a str,
    ) -> impl Iterator<Item = &'a HidingPlaceData> + 'a {
        self.hiding_places
            .iter()
            .filter(move |h| h.animal == animal)
    }

    /// Enclosure features of an enclosure element.
    pub fn features_of<'a>(
        &'a self,
        enclosure: &'a str,
    ) -> impl Iterator<Item = &'a EnclosureFeature> + 'a {
        self.enclosure_features
            .iter()
            .filter(move |f| f.enclosure == enclosure)
    }

    pub fn elements_of(&self, ty: ElementType) -> impl Iterator<Item = &Element> {
        self.elements.iter().filter(move |e| e.ty == ty)
    }

    /// Pairs of solid elements sharing a cell, with the first shared cell (LAYOUT-003).
    pub fn solid_overlaps(&self) -> Vec<(String, String, IVec2)> {
        let solids: Vec<&Element> = self.elements.iter().filter(|e| e.is_solid()).collect();
        let mut out = Vec::new();
        for (i, a) in solids.iter().enumerate() {
            for b in &solids[i + 1..] {
                if let Some(c) = a.footprint_cells().find(|c| b.footprint_contains(*c)) {
                    out.push((a.id.clone(), b.id.clone(), c));
                }
            }
        }
        out
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.z < b.z + b.d && b.z < a.z + a.d
}

fn rect_union(a: Rect, b: Rect) -> Rect {
    let x0 = a.x.min(b.x);
    let z0 = a.z.min(b.z);
    let x1 = (a.x + a.w).max(b.x + b.w);
    let z1 = (a.z + a.d).max(b.z + b.d);
    Rect::new(x0, z0, x1 - x0, z1 - z0)
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
    /// Inside the joined bounding box but in no level (GAME-LAYOUT "Joining levels").
    outside: bool,
}

/// Walkable grid of a level.
#[derive(Debug, Clone)]
pub struct Grid {
    bounds: Rect,
    cells: Vec<CellInfo>,
    ground: Surface,
    /// Cell edges no step may cross (garden fences on cell-edge lines, Q-102): the lower /
    /// western cell and the axis of the step (0 = +x, 1 = +z).
    blocked_edges: std::collections::BTreeSet<(i32, i32, u8)>,
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
        if data.parts.len() > 1 {
            for c in b.cells() {
                if data.part_at(c).is_none() {
                    cells[index(c).expect("in bounds")].outside = true;
                }
            }
        }
        for (i, e) in data.elements.iter().enumerate() {
            let open = e.ty == ElementType::Barrier && open_barriers.contains(&e.id);
            // (the interior of a round house is not part of its footprint rects)
            for c in e.footprint_cells().chain(e.interior_cells()) {
                let Some(k) = index(c) else { continue };
                if e.ty == ElementType::Path {
                    cells[k].path = true;
                } else if e.is_open_cell(c) {
                    // interior / door of an enterable building: floor (Q-092)
                    cells[k].path = true;
                } else if e.is_solid() && !open && cells[k].solid.is_none() {
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
        let mut blocked_edges = std::collections::BTreeSet::new();
        for g in &data.gardens {
            for r in &g.fence_runs {
                for key in fence_edges(*r) {
                    blocked_edges.insert(key);
                }
            }
        }
        Self {
            bounds: b,
            cells,
            ground: data.level.ground_surface,
            blocked_edges,
        }
    }

    /// Whether a step between two neighbouring cells crosses no blocked cell edge (garden
    /// fences); a diagonal step is blocked by any of the four edges at its corner.
    pub fn step_open(&self, a: IVec2, b: IVec2) -> bool {
        if self.blocked_edges.is_empty() {
            return true;
        }
        let d = b - a;
        let edge = |p: IVec2, q: IVec2| -> bool {
            let (lo, axis) = if q.x != p.x {
                (if q.x > p.x { p } else { q }, 0u8)
            } else {
                (if q.y > p.y { p } else { q }, 1u8)
            };
            !self.blocked_edges.contains(&(lo.x, lo.y, axis))
        };
        if d.x != 0 && d.y != 0 {
            let ax = a + IVec2::new(d.x, 0);
            let az = a + IVec2::new(0, d.y);
            edge(a, ax) && edge(a, az) && edge(ax, b) && edge(az, b)
        } else {
            edge(a, b)
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
        if info.outside {
            return CellKind::OutOfBounds;
        }
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

/// Cell edges a fence line `[x0, z0, x1, z1]` (level metres, on cell edges) blocks:
/// `(lower cell x, z, axis)` with axis 0 = the step +x, 1 = the step +z.
pub fn fence_edges(r: [f32; 4]) -> Vec<(i32, i32, u8)> {
    let [x0, z0, x1, z1] = r;
    let mut out = Vec::new();
    if (z0 - z1).abs() < 1e-3 {
        // along x on the line z = z0: blocks the +z steps (x, z0 - 1) → (x, z0)
        let z = z0.round() as i32;
        let (a, b) = (x0.min(x1).round() as i32, x0.max(x1).round() as i32);
        for x in a..b {
            out.push((x, z - 1, 1));
        }
    } else if (x0 - x1).abs() < 1e-3 {
        let x = x0.round() as i32;
        let (a, b) = (z0.min(z1).round() as i32, z0.max(z1).round() as i32);
        for z in a..b {
            out.push((x - 1, z, 0));
        }
    }
    out
}

/// A level at runtime: data plus which barriers are open.
#[derive(Debug, Clone)]
pub struct Level {
    pub data: LevelData,
    open_barriers: BTreeSet<String>,
    grid: Grid,
    colliders: crate::collision::Colliders,
    /// Scene placements (for colliders) and the placements owned by each barrier.
    placements: Vec<crate::scene::Placement>,
    barrier_parts: Vec<(String, std::ops::Range<usize>)>,
    /// Solid placeholder boxes on walkable cells (GAME-PLAYER 9).
    box_colliders: Vec<crate::collision::Shape>,
    /// Shapes solid only while their barrier is closed (level-gate leaves, LAYOUT-036).
    barrier_colliders: Vec<(String, crate::collision::Shape)>,
    /// Ground heights (GAME-PLAYER 8).
    ground: crate::ground::GroundMap,
    /// Gate and door models in the openings (LAYOUT-031).
    openings: Vec<crate::scene::Opening>,
    /// Lantern-post colliders C(0, 0, 0.12), solid only while the posts are visible (at
    /// night, LAYOUT-035); they never change the grid (navigation stays the same by day and
    /// night).
    night_shapes: Vec<crate::collision::Shape>,
    night_solid: bool,
    /// Boxes of the parked golf carts (solid for the player, animals and other carts).
    cart_shapes: Vec<crate::collision::Shape>,
}

/// Collider radius of a lantern post while it is visible (GAME-LAYOUT `[[light]]`, LAYOUT-035).
pub const LANTERN_POST_RADIUS_M: f32 = 0.12;

impl Level {
    pub fn new(data: LevelData) -> Self {
        let open_barriers = BTreeSet::new();
        let mut grid = Grid::build(&data, &open_barriers);
        let scene = crate::scene::LevelScene::build(&data);
        let mut extra = scene.box_colliders.clone();
        extra.extend(scene.barrier_colliders.iter().map(|(_, s)| *s));
        let colliders = crate::collision::Colliders::from_placements_and(
            &scene.placements,
            &extra,
            data.level.bounds,
        );
        grid.set_prop_blocked(&colliders);
        let ground = crate::ground::GroundMap::build(&scene, data.level.bounds);
        let night_shapes = data
            .lights
            .iter()
            .filter(|l| l.kind.is_empty() || l.kind == "lantern_post")
            .filter_map(|l| l.pos())
            .map(|c| crate::collision::Shape::Circle {
                c,
                r: LANTERN_POST_RADIUS_M,
            })
            .collect();
        Self {
            data,
            open_barriers,
            grid,
            colliders,
            placements: scene.placements,
            barrier_parts: scene.barrier_parts,
            box_colliders: scene.box_colliders,
            barrier_colliders: scene.barrier_colliders,
            ground,
            openings: scene.openings,
            night_shapes,
            night_solid: false,
            cart_shapes: Vec::new(),
        }
    }

    /// Sets the boxes of the parked golf carts (GAME-CART rule 4): they become prop colliders
    /// and block the grid cells whose centre a player circle could not stand on.
    pub fn set_cart_shapes(&mut self, shapes: Vec<crate::collision::Shape>) {
        if self.cart_shapes == shapes {
            return;
        }
        self.cart_shapes = shapes;
        self.rebuild_colliders();
        self.refresh_prop_blocked();
    }

    /// The boxes of the parked golf carts.
    pub fn cart_shapes(&self) -> &[crate::collision::Shape] {
        &self.cart_shapes
    }

    /// Makes the lantern posts solid (they are visible: dusk to morning) or not (by day),
    /// LAYOUT-035. Only the player's colliders change, not the grid.
    pub fn set_night_solid(&mut self, on: bool) {
        if self.night_solid != on {
            self.night_solid = on;
            self.rebuild_colliders();
        }
    }

    /// Whether the lantern posts are solid now.
    pub fn night_solid(&self) -> bool {
        self.night_solid
    }

    /// Gate and door models in the openings (LAYOUT-031); indices match
    /// [`crate::scene::LevelScene::openings`].
    pub fn openings(&self) -> &[crate::scene::Opening] {
        &self.openings
    }

    /// Height (m) of the visible walkable surface at level point `p` (GAME-PLAYER 8).
    pub fn ground_height(&self, p: Vec2) -> f32 {
        self.ground.height(p)
    }

    /// The ground height map (GAME-PLAYER 8).
    pub fn ground(&self) -> &crate::ground::GroundMap {
        &self.ground
    }

    /// Colliders of every placement except the parts of opened barriers.
    fn rebuild_colliders(&mut self) {
        let removed: Vec<std::ops::Range<usize>> = self
            .barrier_parts
            .iter()
            .filter(|(id, _)| self.open_barriers.contains(id))
            .map(|(_, r)| r.clone())
            .collect();
        let kept: Vec<crate::scene::Placement> = self
            .placements
            .iter()
            .enumerate()
            .filter(|(i, _)| !removed.iter().any(|r| r.contains(i)))
            .map(|(_, p)| p.clone())
            .collect();
        let mut extra = self.box_colliders.clone();
        extra.extend(
            self.barrier_colliders
                .iter()
                .filter(|(id, _)| !self.open_barriers.contains(id))
                .map(|(_, s)| *s),
        );
        if self.night_solid {
            extra.extend_from_slice(&self.night_shapes);
        }
        extra.extend_from_slice(&self.cart_shapes);
        self.colliders =
            crate::collision::Colliders::from_placements_and(&kept, &extra, self.data.level.bounds);
    }

    /// Grid cells blocked by props: from the colliders without the night-only posts.
    fn refresh_prop_blocked(&mut self) {
        let night = self.night_solid;
        if night {
            self.night_solid = false;
            self.rebuild_colliders();
        }
        self.grid.set_prop_blocked(&self.colliders);
        if night {
            self.night_solid = true;
            self.rebuild_colliders();
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
        self.rebuild_colliders();
        self.grid = Grid::build(&self.data, &self.open_barriers);
        self.refresh_prop_blocked();
        true
    }

    /// Closes an opened barrier again (only the moon door closes by day, GAME-NIGHT). Returns
    /// false if it was not open.
    pub fn close_barrier(&mut self, id: &str) -> bool {
        if !self.open_barriers.remove(id) {
            return false;
        }
        self.rebuild_colliders();
        self.grid = Grid::build(&self.data, &self.open_barriers);
        self.refresh_prop_blocked();
        true
    }

    /// Barriers whose transition leaves this level (`<level_id>-><next>`), proposal Q-022.
    pub fn exit_barriers(&self) -> Vec<String> {
        self.exit_barriers_of(&self.data.parts[0].id.clone())
    }

    /// Barriers (not moon doors) that are unlocked by completing `level_id`: `unlock_after =
    /// level_id` when that level is joined (e.g. `barrier_ne_tree` after `night_1`, Q-078,
    /// Q-133), else the transition's source level (`<level_id>-><next>`).
    pub fn barriers_unlocked_by(&self, level_id: &str) -> Vec<String> {
        self.data
            .elements_of(ElementType::Barrier)
            .filter(|e| e.kind.as_deref() != Some("moon_door"))
            // a night gate (`opens_at = "night"`) leading into a level that is not joined stays
            // shut (night_1 played without the terrarium garden)
            .filter(|e| {
                e.opens_at.as_deref() != Some("night")
                    || e.transition
                        .as_deref()
                        .and_then(|t| t.split("->").nth(1))
                        .is_none_or(|to| self.data.part_index(to).is_some())
            })
            .filter(|e| {
                let from = e.transition.as_deref().and_then(|t| t.split("->").next());
                let by = e
                    .unlock_after
                    .as_deref()
                    .filter(|l| self.data.part_index(l).is_some())
                    .or(from);
                by == Some(level_id)
            })
            .map(|e| e.id.clone())
            .collect()
    }

    /// Barriers whose transition leaves the level `level_id` (`<level_id>-><next>`).
    pub fn exit_barriers_of(&self, level_id: &str) -> Vec<String> {
        let prefix = format!("{level_id}->");
        self.data
            .elements_of(ElementType::Barrier)
            // a moon door is not an exit: it opens every night (GAME-NIGHT, Q-133)
            .filter(|e| e.kind.as_deref() != Some("moon_door"))
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
