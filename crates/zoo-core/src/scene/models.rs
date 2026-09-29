//! Models of buildings, doors and gates, furniture and gardens (`tools/blender/props/
//! README_night.md`): which model stands where, which of its parts move (LAYOUT-031), which
//! text faces carry a Fluent text. Pure data — the host draws and animates it.

use super::*;
use crate::level::GardenData;

/// Building models (`assets/models/buildings/`, ART-PIPELINE kind `buildings`).
pub const BUILDING_MODELS: [&str; 5] = [
    "zookeeper_house",
    "food_storage",
    "food_hut",
    "entrance_arch",
    "night_house",
];

/// Asset path (relative to `assets/`) of a model placed by the scene.
pub fn model_path(model: &str) -> String {
    if BUILDING_MODELS.contains(&model) {
        format!("models/buildings/{model}.glb")
    } else {
        format!("models/props/{model}.glb")
    }
}

/// A building model and where its door goes (README_night "kit_buildings", "Door
/// placements"): model-space origin = centre of the level rect (night house: `model_rect`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuildingSpec {
    /// Element kind it replaces.
    pub kind: &'static str,
    pub model: &'static str,
    /// Footprint (level w, d) at yaw 0.
    pub size: (i32, i32),
    /// Door cell centre relative to the origin (level metres) at yaw 0.
    pub door: Option<Vec2>,
    /// `door_wood` hinge in the model's glTF space and its yaw (degrees).
    pub hinge: Vec3,
    pub door_yaw_deg: f32,
    /// Height of the eaves (m): the roof part starts here.
    pub eaves_m: f32,
}

pub const BUILDING_SPECS: [BuildingSpec; 5] = [
    BuildingSpec {
        kind: "zookeeper_house",
        model: "zookeeper_house",
        size: (6, 5),
        door: Some(Vec2::new(2.5, 0.0)),
        hinge: Vec3::new(2.85, 0.0, 0.47),
        door_yaw_deg: 90.0,
        eaves_m: 2.5,
    },
    BuildingSpec {
        kind: "food_storage",
        model: "food_storage",
        size: (8, 6),
        door: Some(Vec2::new(0.5, -2.5)),
        hinge: Vec3::new(0.03, 0.0, 2.90),
        door_yaw_deg: 0.0,
        eaves_m: 2.6,
    },
    BuildingSpec {
        kind: "food_hut",
        model: "food_hut",
        size: (5, 6),
        door: Some(Vec2::new(2.0, 0.5)),
        hinge: Vec3::new(2.40, 0.0, -0.03),
        door_yaw_deg: 90.0,
        eaves_m: 2.3,
    },
    BuildingSpec {
        kind: "entrance",
        model: "entrance_arch",
        size: (6, 2),
        door: None,
        hinge: Vec3::ZERO,
        door_yaw_deg: 0.0,
        eaves_m: 5.0,
    },
    BuildingSpec {
        kind: "night_house",
        model: "night_house",
        size: (17, 13),
        door: Some(Vec2::new(0.0, -6.0)),
        hinge: Vec3::new(-0.47, 0.0, 6.35),
        door_yaw_deg: 0.0,
        eaves_m: 2.0,
    },
];

/// Level-space rotation of a model yaw (world +Y, counter-clockwise from above): level
/// `(x, z)` → `(c x − s z, s x + c z)`.
pub fn rot_level(v: Vec2, yaw: f32) -> Vec2 {
    let (s, c) = yaw.sin_cos();
    Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y)
}

/// Model space (glTF: x right, z = level south) → level offset at `yaw`.
pub fn model_offset(p: Vec3, yaw: f32) -> Vec2 {
    rot_level(Vec2::new(p.x, -p.z), yaw)
}

/// The building model of an element, its origin (level) and yaw — only when the element's
/// footprint and door cell fit the model (at yaw 0, ±90° or 180°); else the procedural
/// placeholder stays (e.g. `zookeeper_house_3`, 7 × 6 m).
pub fn building_model(e: &Element) -> Option<(&'static BuildingSpec, Vec2, f32)> {
    if e.ty != ElementType::Building {
        return None;
    }
    let kind = e.kind.as_deref()?;
    let spec = BUILDING_SPECS.iter().find(|s| s.kind == kind)?;
    let r = e.model_rect.unwrap_or(e.rect);
    let c = rect_center(r);
    for deg in [0.0f32, 90.0, -90.0, 180.0] {
        let dims = if deg.abs() == 90.0 {
            (r.d, r.w)
        } else {
            (r.w, r.d)
        };
        if dims != spec.size {
            continue;
        }
        let yaw = deg.to_radians();
        match (spec.door, e.door_cell()) {
            (Some(d), Some(cell)) => {
                if rot_level(d, yaw).distance(cell_center(cell) - c) < 0.01 {
                    return Some((spec, c, yaw));
                }
            }
            (None, None) => return Some((spec, c, yaw)),
            _ => {}
        }
    }
    None
}

/// What an opening's model does (LAYOUT-031, GAME-LAYOUT "Gates and doors").
#[derive(Debug, Clone, PartialEq)]
pub enum OpeningKind {
    /// `door_wood` of a building: swings inwards while the player passes (enterable
    /// buildings only; the others stay closed).
    BuildingDoor { building: String, enterable: bool },
    /// `gate_wood` of an enclosure: swings into the enclosure while animals are led in.
    EnclosureGate { enclosure: String },
    /// `glass_door` of an indoor enclosure (night house): as an enclosure gate.
    GlassDoor { enclosure: String },
    /// `garden_gate`: opens by itself when the player is within 2 m (proposal Q-102).
    GardenGate { garden: String },
    /// `moon_door`: open while its barrier is open (at night).
    MoonDoor { barrier: String },
    /// `gate_zoo` between two levels (LAYOUT-036): closed and solid while its barrier
    /// stands, open (and staying open) once the barrier is cleared.
    LevelGate { barrier: String },
}

/// A gate or door model in an opening.
#[derive(Debug, Clone, PartialEq)]
pub struct Opening {
    /// Index into [`LevelScene::placements`].
    pub placement: usize,
    pub kind: OpeningKind,
    /// Centre of the opening (level).
    pub center: Vec2,
    /// Clear width of the opening and the width the model fills (m).
    pub opening_m: f32,
    pub model_m: f32,
    /// Yaw added to the placement when fully open (models that swing as a whole:
    /// `door_wood`, `gate_wood`); 0 for models with leaf parts (open amount instead).
    pub swing: f32,
}

impl Opening {
    /// Models whose parts open (leaves) rather than the whole placement.
    pub fn opens_parts(&self) -> bool {
        self.swing == 0.0
    }
}

/// A building drawn by its model: its roof / upper walls hide while the player is inside in
/// the zoo view (PLAY-028), never in first person (CAMV-022).
#[derive(Debug, Clone, PartialEq)]
pub struct BuildingModel {
    pub element: String,
    pub placement: usize,
}

/// A blank text face of a placed model (`sign_face`, `note_face`) with its Fluent text.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFace {
    /// Decal id (`sign:<id>`).
    pub id: String,
    pub placement: usize,
    pub slot: &'static str,
    pub key: String,
}

/// A plant spot of a garden; its model (`<kind>_plant_<stage>`) is chosen by the game state.
#[derive(Debug, Clone, PartialEq)]
pub struct PlantPlacement {
    pub spot: String,
    /// `carrot` | `potato`.
    pub kind: String,
    /// Plant base on the soil (world).
    pub pos: Vec3,
    pub part: u8,
}

/// Placements merged into one static mesh (static batching, ARCH-008).
#[derive(Debug, Clone, PartialEq)]
pub struct BakeGroup {
    /// Mesh name of the merged group (`__bake:<id>`).
    pub name: String,
    /// Indices into [`LevelScene::placements`].
    pub placements: Vec<usize>,
    pub part: u8,
}

impl LevelScene {
    /// Adds placements to the bake group `id` (created on first use).
    pub(super) fn bake_into(&mut self, id: &str, part: u8, placements: std::ops::Range<usize>) {
        let name = format!("__bake:{id}");
        match self.bake_groups.iter_mut().find(|g| g.name == name) {
            Some(g) => g.placements.extend(placements),
            None => self.bake_groups.push(BakeGroup {
                name,
                placements: placements.collect(),
                part,
            }),
        }
    }
}

/// `gate_zoo` (kit_gates): pillar centres (±x), pillar half extents at the walking height
/// (0.5 × 0.6 m) and the clear opening between the pillars (m).
pub const GATE_ZOO_PILLAR_X: f32 = 1.25;
pub const GATE_ZOO_PILLAR_HALF: Vec2 = Vec2::new(0.25, 0.30);
pub const GATE_ZOO_OPENING_M: f32 = 2.0;
/// Half depth of the closed `gate_zoo` leaves with straps, rings and padlock (glTF z).
pub const GATE_ZOO_LEAF_HZ: f32 = 0.15;

/// Whether a barrier is the barrier of a level entry (`[[entry]] barrier`).
pub fn is_level_gate_barrier(data: &LevelData, id: &str) -> bool {
    data.parts
        .iter()
        .any(|p| p.entries.iter().any(|en| en.barrier == id))
        && data
            .element(id)
            .is_some_and(|b| b.kind.as_deref() != Some("moon_door"))
}

/// Pose of the level gate of an entry (LAYOUT-036): centre (level), yaw (front towards the
/// old level, the leaves open to the back into the new level) and the direction from the
/// gate towards the old level. A gate barrier (`closed_gate`) is replaced by the gate on the
/// barrier's hedge line (its row next to the old level); any other story barrier stays in
/// front and the gate stands on the new level's hedge line (the row of its 2 m border band
/// away from the barrier). `None` if the barrier does not touch the entry cells.
pub fn level_gate_pose(en: &crate::level::EntryData, data: &LevelData) -> Option<(Vec2, f32, Dir)> {
    let b = data
        .element(&en.barrier)
        .filter(|b| b.kind.as_deref() != Some("moon_door"))?; // the moon door has its own model
    let e = en.cells;
    let dir = Dir::ALL.into_iter().find(|d| {
        let o = d.offset();
        e.cells().any(|c| b.rect.contains(c + o))
    })?;
    let to_old = dir.offset().as_vec2();
    let ec = rect_center(e);
    let bc = rect_center(b.rect);
    let along = Vec2::new(to_old.y.abs(), to_old.x.abs());
    let c = if b.kind.as_deref() == Some("closed_gate") {
        // the barrier's row next to the old level, centred on the entry
        let depth = (b.rect.w as f32 * to_old.x.abs()) + (b.rect.d as f32 * to_old.y.abs());
        let row = bc + to_old * (depth / 2.0 - 0.5);
        row * to_old.abs() + ec * along
    } else {
        // the new level's band row away from the barrier (entry cells + 1 m)
        ec - to_old * 1.0
    };
    Some((c, facing_yaw(dir), dir))
}

/// Top of the plank platform in the wall band of the food storage / food hut models
/// (`kit_buildings.py` `PLATFORM`): the unlabelled stock boxes inside stand on it (Q-194).
pub const STORAGE_PLATFORM_M: f32 = 0.15;
/// Soil top of `garden_bed` (m): plants stand on it.
pub const GARDEN_SOIL_M: f32 = 0.22;
/// `door_wood` leaf width (m, x 0.01…0.95).
pub const DOOR_WOOD_M: f32 = 0.94;
/// `gate_wood` leaf length (x 0.02…1.80 from its hinge, placed 0.09 m inside the post).
pub const GATE_WOOD_M: f32 = 1.78;
/// Moon door light empties `light_l` / `light_r` (glTF): lanterns on the pillar caps.
pub const MOON_DOOR_LIGHTS: [Vec3; 2] = [Vec3::new(-1.35, 3.62, 0.0), Vec3::new(1.35, 3.62, 0.0)];
/// Entrance turnstiles: unit x offsets and glTF z of the row (README_night "Turnstiles":
/// glTF z −0.5; moved 0.2 m towards the plaza so the closed row stands at the walkable
/// edge of the arch cells — no invisible wall, LAYOUT-019).
pub const TURNSTILE_X: [f32; 3] = [-1.2, 0.0, 1.2];
pub const TURNSTILE_Z: f32 = -0.7;
/// Board-lamp socket in the board's model space (README_night "Board-lamp sockets").
pub const BOARD_LAMP_INFO: Vec3 = Vec3::new(0.0, 1.405, -0.236);
pub const BOARD_LAMP_MAP: Vec3 = Vec3::new(0.0, 1.93, -0.09);
/// Board-lamp clip above a wall-mounted info board (`mount = "wall"`, Q-157): on the facade
/// just above the panel (its shade hangs over the panel).
pub const BOARD_LAMP_WALL: Vec3 = Vec3::new(0.0, 1.85, 0.02);
/// `lantern_post` light empty (glTF).
pub const LANTERN_LIGHT: Vec3 = Vec3::new(0.0, 1.74, 0.57);
/// `wall_lamp` light empty and its mount height.
pub const WALL_LAMP_LIGHT: Vec3 = Vec3::new(0.0, 0.17, 0.32);
pub const WALL_LAMP_MOUNT_M: f32 = 1.6;
/// `board_lamp` light empty.
pub const BOARD_LAMP_LIGHT: Vec3 = Vec3::new(0.0, 0.12, 0.43);
/// `string_lights` span (m) from its post to the hook point.
pub const STRING_SPAN_M: f32 = 6.0;
/// `desk.socket_note`, `night_table.socket_lamp`, `key_box.socket_key`, `cart_key.socket_ring`.
pub const DESK_NOTE: Vec3 = Vec3::new(-0.10, 0.72, 0.05);
pub const NIGHT_TABLE_LAMP: Vec3 = Vec3::new(0.08, 0.55, -0.08);
pub const KEY_BOX_KEY: Vec3 = Vec3::new(0.0, 0.32, 0.07);
pub const CART_KEY_RING: Vec3 = Vec3::new(0.0, 0.15, 0.0);
/// Wall-mounted pieces: origin height above the floor (README_night).
pub const WINDOW_MOON_MOUNT_M: f32 = 1.0;
pub const KEY_BOX_MOUNT_M: f32 = 1.0;
/// Fluent key of the entrance arch board (proposal Q-148).
pub const ENTRANCE_SIGN_KEY: &str = "sign-zoo-entrance";

/// Yaw of a level `facing` string (`-z` → 0, `+x` → +90°, `-x` → −90°, `+z` → 180°).
pub fn facing_str_yaw(f: &str) -> f32 {
    facing_yaw(Dir::from_vec(crate::level::facing_vec(f)))
}

impl LevelScene {
    /// Places a model at a level point with a height; returns its placement index.
    pub(super) fn model_at_y(&mut self, model: &'static str, p: Vec2, y: f32, yaw: f32) -> usize {
        self.placements
            .push(Placement::new(model, level_to_world_at(p, y), yaw));
        self.placements.len() - 1
    }

    /// A building by its model: placement, door, sockets; returns true when a model fits.
    pub(super) fn building_by_model(&mut self, e: &Element, data: &LevelData) -> bool {
        let Some((spec, c, yaw)) = building_model(e) else {
            return false;
        };
        let k = self.model_at_y(spec.model, c, 0.0, yaw);
        self.building_models.push(BuildingModel {
            element: e.id.clone(),
            placement: k,
        });
        if spec.model == "entrance_arch" {
            // closed turnstiles under the arch (QA F8) and the zoo's name on the board
            for x in TURNSTILE_X {
                let p = c + model_offset(Vec3::new(x, 0.0, TURNSTILE_Z), yaw);
                self.model_at_y("turnstile", p, 0.0, yaw);
            }
            self.text_faces.push(TextFace {
                id: format!("sign:{}", e.id),
                placement: k,
                slot: "sign_face",
                key: ENTRANCE_SIGN_KEY.to_owned(),
            });
        }
        if spec.door.is_some() {
            let hinge = c + model_offset(spec.hinge, yaw);
            let d = self.model_at_y(
                "door_wood",
                hinge,
                0.0,
                yaw + spec.door_yaw_deg.to_radians(),
            );
            let center = e.door_cell().map_or(hinge, cell_center);
            self.openings.push(Opening {
                placement: d,
                kind: OpeningKind::BuildingDoor {
                    building: e.id.clone(),
                    enterable: e.is_enterable(),
                },
                center,
                opening_m: 1.0,
                model_m: DOOR_WOOD_M,
                swing: 90f32.to_radians(),
            });
        }
        let _ = data;
        true
    }

    /// `door_wood` in the door gap of a procedural building (hinge on the facade's middle
    /// line, the leaf across the 1 m gap, swinging inwards).
    pub(super) fn procedural_door(&mut self, e: &Element, facade_mid: f32, side: char) {
        let Some(dc) = e.door_cell() else { return };
        let (hinge, yaw) = match side {
            'S' => (
                Vec2::new(dc.x as f32 + 0.03, facade_mid),
                facing_yaw(Dir::S),
            ),
            'N' => (
                Vec2::new(dc.x as f32 + 0.97, facade_mid),
                facing_yaw(Dir::N),
            ),
            'E' => (
                Vec2::new(facade_mid, dc.y as f32 + 0.03),
                facing_yaw(Dir::E),
            ),
            _ => (
                Vec2::new(facade_mid, dc.y as f32 + 0.97),
                facing_yaw(Dir::W),
            ),
        };
        let d = self.model_at_y("door_wood", hinge, 0.0, yaw);
        self.openings.push(Opening {
            placement: d,
            kind: OpeningKind::BuildingDoor {
                building: e.id.clone(),
                enterable: e.is_enterable(),
            },
            center: cell_center(dc),
            opening_m: 1.0,
            model_m: DOOR_WOOD_M,
            swing: 90f32.to_radians(),
        });
    }

    /// The moon door model (pillars, beam, sign, lanterns and the two leaves); front towards
    /// the day level it belongs to.
    pub(super) fn moon_door_model(&mut self, e: &Element, data: &LevelData) {
        let (c, yaw) = moon_door_pose(e, data);
        let k = self.model_at_y("moon_door", c, 0.0, yaw);
        self.placements[k].part = e.part as u8;
        self.openings.push(Opening {
            placement: k,
            kind: OpeningKind::MoonDoor {
                barrier: e.id.clone(),
            },
            center: c,
            opening_m: 2.0,
            // leaves 0.96 m each from the hinges at ±0.96 (inner pillar faces ±1.0)
            model_m: 1.92,
            swing: 0.0,
        });
    }

    /// The `gate_zoo` of a level entry (GAME-LAYOUT "Gates between the levels", LAYOUT-036):
    /// its pillars are always solid, its closed leaves only while the barrier stands.
    pub(super) fn level_gate(
        &mut self,
        en: &crate::level::EntryData,
        part: usize,
        data: &LevelData,
    ) {
        let Some((c, yaw, _)) = level_gate_pose(en, data) else {
            return;
        };
        let k = self.model_at_y("gate_zoo", c, 0.0, yaw);
        // the gate replacing a gate barrier belongs to the old level (its hedge line)
        let is_gate_barrier = data
            .element(&en.barrier)
            .is_some_and(|b| b.kind.as_deref() == Some("closed_gate"));
        self.placements[k].part = if is_gate_barrier {
            data.element(&en.barrier).map_or(part, |b| b.part) as u8
        } else {
            part as u8
        };
        let pos = self.placements[k].pos;
        use crate::collision::{LocalShape, Shape};
        let h = GATE_ZOO_PILLAR_HALF;
        for sx in [-1.0, 1.0] {
            self.box_colliders.push(Shape::place(
                LocalShape::Box {
                    x: sx * GATE_ZOO_PILLAR_X,
                    z: 0.0,
                    hx: h.x,
                    hz: h.y,
                },
                pos,
                yaw,
            ));
        }
        self.barrier_colliders.push((
            en.barrier.clone(),
            Shape::place(
                LocalShape::Box {
                    x: 0.0,
                    z: 0.0,
                    hx: GATE_ZOO_OPENING_M / 2.0,
                    hz: GATE_ZOO_LEAF_HZ,
                },
                pos,
                yaw,
            ),
        ));
        self.openings.push(Opening {
            placement: k,
            kind: OpeningKind::LevelGate {
                barrier: en.barrier.clone(),
            },
            center: c,
            opening_m: GATE_ZOO_OPENING_M,
            model_m: GATE_ZOO_OPENING_M,
            swing: 0.0,
        });
    }

    /// `gate_wood` in an enclosure gate, or a `glass_door` for an indoor enclosure of a
    /// modelled night house.
    pub(super) fn enclosure_gate(&mut self, e: &Element, gate: Run, glass: bool) {
        let dir = gate.axis.dir();
        let rc = rect_center(e.rect);
        if glass {
            let mid = gate.start + dir * 1.0;
            // front towards the hall (outside the enclosure)
            let out = dir_away(e.rect, mid + (mid - rc).normalize_or_zero() * 0.01);
            let k = self.model_at_y("glass_door", mid, 0.0, facing_yaw(out));
            self.openings.push(Opening {
                placement: k,
                kind: OpeningKind::GlassDoor {
                    enclosure: e.id.clone(),
                },
                center: mid,
                opening_m: 2.0,
                model_m: 1.95,
                swing: 0.0,
            });
            return;
        }
        let hinge = gate.start + dir * 0.09;
        let yaw = run_yaw(gate.axis);
        let k = self.model_at_y("gate_wood", hinge, 0.0, yaw);
        // swing into the enclosure: the sign whose open leaf lies nearer the rect centre
        let swing = [90f32, -90.0]
            .into_iter()
            .map(f32::to_radians)
            .min_by(|a, b| {
                let mid = |s: f32| hinge + rot_level(Vec2::X, yaw + s) * (GATE_WOOD_M / 2.0);
                mid(*a).distance(rc).total_cmp(&mid(*b).distance(rc))
            })
            .unwrap_or(0.0);
        self.openings.push(Opening {
            placement: k,
            kind: OpeningKind::EnclosureGate {
                enclosure: e.id.clone(),
            },
            center: gate.start + dir * 1.0,
            opening_m: 1.82,
            model_m: GATE_WOOD_M,
            swing,
        });
    }

    /// Furniture of a `[[prop]]` by its model (the placeholder boxes are the fallback).
    pub(super) fn prop_model(&mut self, p: &crate::level::PropData) {
        let Some(model) = furniture_model(&p.model) else {
            self.prop_placeholder(p);
            return;
        };
        let yaw = facing_str_yaw(&p.facing);
        let y = match model {
            "window_moon" => WINDOW_MOON_MOUNT_M,
            "rug_round" => crate::ground::PATH_TOP_M + 0.001,
            // stock boxes stand on the plank platform of the storage's wall band (Q-194)
            "food_box" | "food_box_stack" => STORAGE_PLATFORM_M,
            _ => 0.0,
        };
        let k = self.model_at_y(model, p.pos(), y, yaw);
        self.placements[k].part = p.part as u8;
        if let Some(b) = &p.building {
            // a room's furniture is one static mesh (the moon window stays apart: its
            // night sky is shown only at night)
            self.bake_into(b, p.part as u8, k..k + 1);
        }
        // placeholder boxes only if the model is missing; the collider stays
        let first = self.boxes.len();
        self.prop_placeholder(p);
        // (the colliders and the rug's walkable top stay, GAME-PLAYER 8/9)
        let mut boxes: Vec<BoxPlacement> = self.boxes.drain(first..).collect();
        for b in &mut boxes {
            b.part = p.part as u8;
        }
        self.fallbacks.push(Fallback {
            model,
            boxes,
            placements: Vec::new(),
        });
        if model == "night_table" {
            // the bedside lamp on its socket
            let q = p.pos() + model_offset(NIGHT_TABLE_LAMP, yaw);
            let l = self.model_at_y("bedside_lamp", q, NIGHT_TABLE_LAMP.y, yaw);
            self.placements[l].part = p.part as u8;
            if let Some(b) = &p.building {
                self.bake_into(b, p.part as u8, l..l + 1);
            }
        }
    }

    /// The zoo's bed, desk note and key box by their models (`[[item]]`).
    pub(super) fn item_model(&mut self, it: &crate::level::ItemData, data: &LevelData) {
        let yaw = it.facing.as_deref().map_or(0.0, facing_str_yaw);
        match it.kind.as_str() {
            "bed" => {
                let k = self.model_at_y("bed", it.pos(), 0.0, yaw);
                self.placements[k].part = it.part as u8;
                if let Some(b) = &it.building {
                    self.bake_into(b, it.part as u8, k..k + 1);
                }
                // solid: the child uses it from its side (GAME-PLAYER 9); fallback boxes
                let first = self.boxes.len();
                self.bed_box(&it.id, it.pos(), Vec2::new(2.0, 1.0));
                let boxes: Vec<BoxPlacement> = self.boxes.drain(first..).collect();
                self.fallbacks.push(Fallback {
                    model: "bed",
                    boxes,
                    placements: Vec::new(),
                });
            }
            "note_math_fighter" => {
                // on the desk's note socket (the desk prop at the same place)
                let desk = data
                    .props
                    .iter()
                    .find(|p| p.model == "desk" && p.pos().distance(it.pos()) < 0.8);
                let (base, dyaw) =
                    desk.map_or((it.pos(), yaw), |d| (d.pos(), facing_str_yaw(&d.facing)));
                let q = base + model_offset(DESK_NOTE, dyaw);
                let k = self.model_at_y("note_paper", q, DESK_NOTE.y, dyaw);
                self.placements[k].part = it.part as u8;
                if let Some(b) = &it.building {
                    self.bake_into(b, it.part as u8, k..k + 1);
                }
            }
            "key_box" => {
                // on the facade (origin on the wall face, 1 m above the ground)
                let wall = it.pos()
                    - crate::level::facing_vec(it.facing.as_deref().unwrap_or("-z")) * 0.05;
                let k = self.model_at_y("key_box", wall, KEY_BOX_MOUNT_M, yaw);
                self.placements[k].part = it.part as u8;
                let key = wall + model_offset(KEY_BOX_KEY - CART_KEY_RING, yaw);
                let y = KEY_BOX_MOUNT_M + KEY_BOX_KEY.y - CART_KEY_RING.y;
                let k = self.model_at_y("cart_key", key, y, yaw);
                self.placements[k].part = it.part as u8;
                if let Some(b) = &it.building {
                    // the box stays shut (the cart key inside): part of the house's mesh
                    self.bake_into(b, it.part as u8, k - 1..k + 1);
                }
            }
            _ => {}
        }
    }

    /// A vegetable garden (GAME-GARDEN, proposal Q-102): beds, signs, fence, the gate, tools;
    /// colliders for the solid parts; plant spots for the host.
    pub(super) fn garden(&mut self, g: &GardenData, data: &LevelData) {
        // (beds, signs, fence pieces and tools are solid by their model footprints,
        // `collision::footprint`, GAME-PLAYER 7)
        let part = g.part as u8;
        let first = self.placements.len();
        for b in data.garden_beds.iter().filter(|b| b.garden == g.id) {
            let c = rect_center(b.rect);
            // long axis = glTF Z (level z at yaw 0); a bed along x turns a quarter
            let yaw = if b.rect.w > b.rect.d {
                facing_yaw(Dir::E)
            } else {
                0.0
            };
            self.model_at_y("garden_bed", c, 0.0, yaw);
            let syaw = facing_str_yaw(&b.sign_facing);
            let sp = Vec2::from(b.sign_pos);
            let k = self.model_at_y("garden_sign", sp, 0.0, syaw);
            self.text_faces.push(TextFace {
                id: format!("sign:{}", b.id),
                placement: k,
                slot: "sign_face",
                key: b.sign_key.clone(),
            });
        }
        for s in data.plant_spots.iter().filter(|s| s.garden == g.id) {
            self.plants.push(PlantPlacement {
                spot: s.id.clone(),
                kind: s.kind.clone(),
                pos: level_to_world_at(s.pos(), GARDEN_SOIL_M),
                part,
            });
        }
        // fence pieces on the runs, `fence_inset_m` inside the outline (posts at the ends)
        let rc = rect_center(g.rect);
        for r in &g.fence_runs {
            let (a, b) = (Vec2::new(r[0], r[1]), Vec2::new(r[2], r[3]));
            let along = (b - a).normalize_or_zero();
            let len = a.distance(b).round() as u32;
            // inwards: towards the garden centre, perpendicular to the run
            let n = Vec2::new(-along.y, along.x);
            let n = if (rc - a).dot(n) >= 0.0 { n } else { -n };
            let start = a.min(b) + n * g.fence_inset_m;
            let axis = if along.x.abs() > 0.5 {
                RunAxis::X
            } else {
                RunAxis::Z
            };
            let run = Run {
                start,
                axis,
                length_m: len,
            };
            self.run_pieces(&run, "garden_fence", "garden_fence_1m");
        }
        let before_gate = self.placements.len();
        // the gate: two leaves, back towards the garden (they swing inwards)
        let gc = g.gate_center();
        let out = Dir::from_vec(g.gate_out());
        let k = self.model_at_y(
            "garden_gate",
            gc + g.gate_out() * -g.fence_inset_m,
            0.0,
            facing_yaw(out),
        );
        self.openings.push(Opening {
            placement: k,
            kind: OpeningKind::GardenGate {
                garden: g.id.clone(),
            },
            center: gc,
            opening_m: 2.0,
            model_m: 1.98,
            swing: 0.0,
        });
        for pr in &g.props {
            let yaw = facing_str_yaw(&pr.facing);
            let p = Vec2::from(pr.pos);
            if let Some(m) = garden_prop_model(&pr.model) {
                self.model_at_y(m, p, 0.0, yaw);
            }
        }
        for p in &mut self.placements[first..] {
            p.part = part;
        }
        // beds, signs, fence and tools never move: one static mesh (the gate animates)
        let end = self.placements.len();
        self.bake_into(&g.id, part, first..before_gate);
        self.bake_into(&g.id, part, before_gate + 1..end);
    }
}

/// Static model name of a furniture `[[prop]]` model id.
fn furniture_model(m: &str) -> Option<&'static str> {
    [
        "desk",
        "night_table",
        "window_moon",
        "rug_round",
        "toy_chest",
        "bed",
        "flower_pots",
        // unlabelled stock boxes inside a food storage (GAME-FEED §7, proposal Q-194)
        "food_box",
        "food_box_stack",
    ]
    .into_iter()
    .find(|x| *x == m)
}

fn garden_prop_model(m: &str) -> Option<&'static str> {
    ["wheelbarrow", "watering_can", "basket"]
        .into_iter()
        .find(|x| *x == m)
}

/// Centre (level) and yaw of a moon door model: across its wall line, the front towards the
/// day level it belongs to (level 1: yaw +90°, front east).
pub fn moon_door_pose(e: &Element, data: &LevelData) -> (Vec2, f32) {
    let (c, along_z) = moon_door_axis(e, data);
    let candidates: [f32; 2] = if along_z { [90.0, -90.0] } else { [0.0, 180.0] };
    let own = |p: Vec2| data.part_at(crate::level::cell_of(p)) == Some(e.part);
    let yaw = candidates
        .into_iter()
        .map(f32::to_radians)
        .find(|&y| {
            // model front = glTF +Z = level south at yaw 0
            let front = rot_level(Vec2::NEG_Y, y);
            own(c + front * 2.5)
        })
        .unwrap_or(candidates[0].to_radians());
    (c, yaw)
}
