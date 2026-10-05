//! Animal houses with door-only entry (GAME-HOUSE, HOUSE-001…020): a building inside an
//! enclosure with solid walls and one open doorway. Wall cells are not part of the home
//! wander area; interior and door cells are, so the interior is reachable only over the door.

use glam::{IVec2, Vec2};

use crate::level::{cell_center, EnclosureFeature, LevelData, Rect};

/// Of 10 new home wander targets this many are an interior cell of the house (HOUSE rule 1).
pub const HOUSE_TARGETS_OF_10: u32 = 2;
/// Rest inside the house, seconds (8–20, seeded).
pub const REST_RANGE_S: (f32, f32) = (8.0, 20.0);
/// The roof is cut away while an animal rests inside and the player is this close to the door.
pub const CUTAWAY_M: f32 = 8.0;
/// Rest length used for "until morning" (dusk): effectively endless.
pub const REST_UNTIL_MORNING_S: f32 = 1.0e6;
/// Models of animal houses that exist as `.glb` (the placeholder box is drawn for all others).
pub const BUILT_MODELS: &[&str] = &[];
/// Thickness of the placeholder roof slab, metres.
pub const ROOF_SLAB_M: f32 = 0.4;

/// Direction a door faces (towards the enclosure interior; `+z` = north).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorSide {
    NegX,
    PosX,
    NegZ,
    PosZ,
}

impl DoorSide {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "-x" => Self::NegX,
            "+x" => Self::PosX,
            "-z" => Self::NegZ,
            "+z" => Self::PosZ,
            _ => return None,
        })
    }

    /// Unit cell step from the house outwards through the door.
    pub fn out(self) -> IVec2 {
        match self {
            Self::NegX => IVec2::NEG_X,
            Self::PosX => IVec2::X,
            Self::NegZ => IVec2::NEG_Y,
            Self::PosZ => IVec2::Y,
        }
    }
}

/// A parsed animal house (`[[enclosure_feature]] kind = "animal_house"`).
#[derive(Debug, Clone, PartialEq)]
pub struct House {
    pub id: String,
    pub enclosure: String,
    pub model: String,
    pub footprint: Rect,
    pub interior: Rect,
    pub door: Rect,
    pub door_side: DoorSide,
    pub door_height_m: f32,
    pub roof_height_m: f32,
    pub rest: bool,
}

impl House {
    /// Builds the house of a feature (None for other kinds or incomplete data).
    pub fn from_feature(f: &EnclosureFeature) -> Option<House> {
        if f.kind != "animal_house" {
            return None;
        }
        Some(House {
            id: f.id.clone(),
            enclosure: f.enclosure.clone(),
            model: f.model.clone().unwrap_or_default(),
            footprint: f.rect,
            interior: f.interior?,
            door: f.door?,
            door_side: DoorSide::parse(f.door_side.as_deref()?)?,
            door_height_m: f.door_height_m?,
            roof_height_m: f.roof_height_m?,
            rest: f.rest.unwrap_or(true),
        })
    }

    pub fn is_interior(&self, c: IVec2) -> bool {
        self.interior.contains(c)
    }

    pub fn is_door(&self, c: IVec2) -> bool {
        self.door.contains(c)
    }

    /// A solid wall cell: in the footprint, neither interior nor door.
    pub fn is_wall(&self, c: IVec2) -> bool {
        self.footprint.contains(c) && !self.interior.contains(c) && !self.door.contains(c)
    }

    /// The cell lies on the footprint or on the ring of cells around it (walls, interior, door
    /// and door front): the pair gap is waived there, a pair squeezes through the door.
    pub fn near(&self, c: IVec2) -> bool {
        let f = self.footprint;
        c.x >= f.x - 1 && c.x <= f.x + f.w && c.y >= f.z - 1 && c.y <= f.z + f.d
    }

    /// Whether an animal may stand on the cell (interior or door; false for walls).
    pub fn is_open(&self, c: IVec2) -> bool {
        self.rest && (self.is_interior(c) || self.is_door(c))
    }

    /// The cells just outside the door (the door front).
    pub fn door_front(&self) -> Vec<IVec2> {
        let o = self.door_side.out();
        self.door.cells().map(|c| c + o).collect()
    }

    /// Centre of the door front in level metres.
    pub fn door_front_center(&self) -> Vec2 {
        let cells = self.door_front();
        cells.iter().map(|&c| cell_center(c)).sum::<Vec2>() / cells.len().max(1) as f32
    }

    /// Roof and front wall are cut away: an animal rests inside and the player is within
    /// [`CUTAWAY_M`] of the door front (HOUSE-014).
    pub fn cutaway(&self, player: Vec2, animal_inside: bool) -> bool {
        animal_inside && player.distance(self.door_front_center()) <= CUTAWAY_M
    }

    /// Interior cells in a fixed order.
    pub fn interior_cells(&self) -> Vec<IVec2> {
        self.interior.cells().collect()
    }
}

/// The house of an enclosure element id, if any.
pub fn house_of(data: &LevelData, enclosure: &str) -> Option<House> {
    data.features_of(enclosure).find_map(House::from_feature)
}

/// All animal houses of a level.
pub fn houses(data: &LevelData) -> Vec<House> {
    data.enclosure_features
        .iter()
        .filter_map(House::from_feature)
        .collect()
}

/// Random rest length inside the house (seeded).
pub fn draw_rest(rng: &mut crate::rng::Pcg32) -> f32 {
    let u = rng.next_u32() as f32 / u32::MAX as f32;
    REST_RANGE_S.0 + (REST_RANGE_S.1 - REST_RANGE_S.0) * u
}
