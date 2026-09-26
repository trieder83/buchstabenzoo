//! Animal data and states (GAME-ANIMALS).

use crate::food::Food;

/// Static animal data (GAME-ANIMALS table / CONT-MISSIONS overview; list pending Q-002).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimalInfo {
    pub id: &'static str,
    pub foods: &'static [Food],
    pub hiding_places: &'static [&'static str],
}

pub const ANIMALS: [AnimalInfo; 10] = [
    AnimalInfo {
        id: "zebra",
        foods: &[Food::Grass],
        hiding_places: &["loc_river"],
    },
    AnimalInfo {
        id: "hippo",
        foods: &[Food::Melons],
        hiding_places: &["loc_pond"],
    },
    AnimalInfo {
        id: "panda",
        foods: &[Food::Bamboo],
        hiding_places: &["loc_cave"],
    },
    AnimalInfo {
        id: "koala",
        foods: &[Food::Eucalyptus],
        hiding_places: &["loc_tallest_tree"],
    },
    AnimalInfo {
        id: "elephant",
        foods: &[Food::Hay],
        hiding_places: &["loc_mud_pool"],
    },
    AnimalInfo {
        id: "goldfish",
        foods: &[Food::FishFood],
        hiding_places: &["loc_fountain"],
    },
    AnimalInfo {
        id: "monkey",
        foods: &[Food::Bananas],
        hiding_places: &["loc_pirate_ship"],
    },
    AnimalInfo {
        id: "giraffe",
        foods: &[Food::Leaves],
        hiding_places: &["loc_playground"],
    },
    AnimalInfo {
        id: "lion",
        foods: &[Food::Meat],
        hiding_places: &["loc_sun_rocks"],
    },
    AnimalInfo {
        id: "snow_fox",
        foods: &[Food::Berries],
        hiding_places: &["loc_ice_cream_kiosk"],
    },
];

pub fn animal_info(id: &str) -> Option<&'static AnimalInfo> {
    ANIMALS.iter().find(|a| a.id == id)
}

impl AnimalInfo {
    pub fn eats(&self, food: Food) -> bool {
        self.foods.contains(&food)
    }
}

/// Animal state machine (GAME-ANIMALS "Animal states").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalState {
    /// At its hiding place.
    Escaped,
    /// Follows the player (GAME-RESCUE §6).
    Following,
    /// Home. Final state (ANIM-002).
    InEnclosure,
}
