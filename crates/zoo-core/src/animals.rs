//! Animal data and states (GAME-ANIMALS).

use crate::food::Food;

/// Static animal data (GAME-ANIMALS table / CONT-MISSIONS overview; list pending Q-002).
/// `hiding_places` lists the candidate places (CONT-MISSIONS; levels 2–3 proposal Q-095);
/// the level data (`[[hiding_place]]`) is the source of truth for positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimalInfo {
    pub id: &'static str,
    pub foods: &'static [Food],
    pub hiding_places: &'static [&'static str],
}

pub const ANIMALS: [AnimalInfo; 13] = [
    AnimalInfo {
        id: "zebra",
        foods: &[Food::Grass],
        hiding_places: &["loc_river", "loc_meadow", "loc_sand"],
    },
    AnimalInfo {
        id: "hippo",
        foods: &[Food::Melons],
        hiding_places: &["loc_pond", "loc_mud", "loc_shade"],
    },
    AnimalInfo {
        id: "panda",
        foods: &[Food::Bamboo],
        hiding_places: &["loc_cave", "loc_bamboo", "loc_leaves"],
    },
    AnimalInfo {
        id: "koala",
        foods: &[Food::Eucalyptus],
        hiding_places: &["loc_treehouse", "loc_tallest_tree", "loc_blossom_tree"],
    },
    AnimalInfo {
        id: "elephant",
        foods: &[Food::Hay],
        hiding_places: &["loc_fountain", "loc_log_pile", "loc_big_ball"],
    },
    AnimalInfo {
        id: "goldfish",
        foods: &[Food::FishFood],
        hiding_places: &["loc_waterfall", "loc_water_wheel", "loc_willow"],
    },
    AnimalInfo {
        id: "monkey",
        foods: &[Food::Bananas],
        hiding_places: &["loc_pirate_ship", "loc_carousel", "loc_trampoline"],
    },
    AnimalInfo {
        id: "giraffe",
        foods: &[Food::Leaves],
        hiding_places: &["loc_lookout_tower", "loc_train", "loc_playground"],
    },
    AnimalInfo {
        id: "lion",
        foods: &[Food::Meat],
        hiding_places: &["loc_sun_rocks", "loc_stage", "loc_deckchairs"],
    },
    AnimalInfo {
        id: "snow_fox",
        foods: &[Food::Berries],
        hiding_places: &["loc_ice_cream_kiosk", "loc_sprinkler", "loc_laundry"],
    },
    // Night zoo `night_1` (GAME-NIGHT rule 6, Q-076/Q-077); candidate places from the
    // level data (`assets/levels/night-1.toml`).
    AnimalInfo {
        id: "hedgehog",
        foods: &[Food::Beetles],
        hiding_places: &["night_1"],
    },
    AnimalInfo {
        id: "bat",
        foods: &[Food::Fruit],
        hiding_places: &["night_1"],
    },
    AnimalInfo {
        id: "owl",
        foods: &[Food::Beetles],
        hiding_places: &["night_1"],
    },
];

/// How deep a swimming animal sinks in water (metres below its land pose), so only eyes,
/// ears and back show (GAME-LEVEL-1 "Hippo enclosure pool", ART-ANIMALS hippo `swim`: the
/// model's water line is ~0.9 m). 0 = the animal does not swim.
pub fn swim_sink_m(animal: &str) -> f32 {
    match animal {
        "hippo" => 0.9,
        // wades in its pool (no `swim` clip: walks / stands in the water, GAME-LEVEL-2)
        "elephant" => 0.8,
        // the goldfish's origin is the water surface (ART-ANIMALS, fish rig): no sinking
        _ => 0.0,
    }
}

/// Clip data of `assets/models/animals/animal_anims.toml` (ART-ANIMALS "Clips"): per animal
/// and clip the frame count, loop flag and authored locomotion speed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimTable {
    clips: std::collections::BTreeMap<(String, String), ClipInfo>,
    climb: std::collections::BTreeMap<String, f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClipInfo {
    pub frames: u32,
    pub looping: bool,
    /// Authored speed (m/s) of a locomotion clip.
    pub speed: Option<f32>,
}

impl AnimTable {
    /// Parses the TOML (unknown keys are ignored). Tables `[<animal>.<clip>]`.
    pub fn from_toml_str(s: &str) -> Result<Self, String> {
        let v: toml::Table = toml::from_str(s).map_err(|e| e.to_string())?;
        let mut clips = std::collections::BTreeMap::new();
        let mut climb = std::collections::BTreeMap::new();
        for (animal, t) in &v {
            let Some(t) = t.as_table() else { continue };
            for (clip, c) in t {
                let Some(c) = c.as_table() else { continue };
                let frames = c.get("frames").and_then(|f| f.as_integer()).unwrap_or(0) as u32;
                let looping = c.get("loop").and_then(|f| f.as_bool()).unwrap_or(false);
                let speed = c
                    .get("speed")
                    .and_then(|f| f.as_float().or_else(|| f.as_integer().map(|i| i as f64)));
                if let Some(cs) = c.get("climb_speed").and_then(|f| f.as_float()) {
                    climb.insert(animal.clone(), cs as f32);
                }
                clips.insert(
                    (animal.clone(), clip.clone()),
                    ClipInfo {
                        frames,
                        looping,
                        speed: speed.map(|x| x as f32),
                    },
                );
            }
        }
        Ok(Self { clips, climb })
    }

    pub fn clip(&self, animal: &str, clip: &str) -> Option<ClipInfo> {
        self.clips
            .get(&(animal.to_owned(), clip.to_owned()))
            .copied()
    }

    /// Authored climbing speed (`climb_speed`, m/s) of an animal's `climb` clip.
    pub fn climb_speed(&self, animal: &str) -> Option<f32> {
        self.climb.get(animal).copied()
    }

    /// Authored speed of an animal's `walk` (default 1.4 m/s, ART-ANIMALS §6).
    pub fn walk_speed(&self, animal: &str) -> f32 {
        self.clip(animal, "walk")
            .and_then(|c| c.speed)
            .filter(|s| *s > 0.0)
            .unwrap_or(crate::player::WALK_CLIP_AUTHORED_SPEED)
    }
}

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
    /// In a carried container (the goldfish in the fish bowl, GAME-RESCUE "goldfish bowl").
    InBowl,
}
