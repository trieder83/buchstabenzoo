//! Animal data and states (GAME-ANIMALS).

use crate::food::Food;

/// Static animal data (GAME-ANIMALS table / CONT-MISSIONS overview; list pending Q-002).
/// `hiding_places` lists the candidate places (CONT-MISSIONS; levels 2–3 proposal Q-095);
/// the level data (`[[hiding_place]]`) is the source of truth for positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimalInfo {
    pub id: &'static str,
    /// Basic foods: the box food that makes the escaped animal follow (GAME-FEED "Basic food
    /// and treats").
    pub foods: &'static [Food],
    pub hiding_places: &'static [&'static str],
    /// Box-food treats that give the pair at home its baby (GAME-FEED "Basic food and
    /// treats"). Empty = the species still uses the older rule: a garden treat it likes, else
    /// its own food (FAM-008/009).
    pub treats: &'static [Food],
}

/// Model id of the female (member 1) of a pair species (GAME-FAMILY §3); member 0 is the male
/// = the adult model of the species.
pub fn female_model(species: &str) -> Option<&'static str> {
    match species {
        "zebra" => Some("zebra_female"),
        "koala" => Some("koala_female"),
        "snake" => Some("snake_female"),
        "chameleon" => Some("chameleon_female"),
        "poison_dart_frog" => Some("poison_dart_frog_female"),
        _ => None,
    }
}

/// Model id of the baby of a pair species (GAME-FAMILY §5), if the species has one.
pub fn baby_model(species: &str) -> Option<&'static str> {
    match species {
        "zebra" => Some("zebra_foal"),
        "koala" => Some("koala_joey"),
        "snake" => Some("snake_hatchling"),
        "chameleon" => Some("chameleon_baby"),
        "poison_dart_frog" => Some("frog_froglet"),
        _ => None,
    }
}

/// How a member of a pair is drawn when its species has no dedicated female / baby model yet
/// (Q-308, ART-ANIMALS "Family models"): the adult model scaled, the female slightly tinted.
/// The male is the adult model at scale 1 (GAME-FAMILY §3: ~10 % larger than the female).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FamilyLook {
    /// Uniform scale of the model.
    pub scale: f32,
    /// Colour tint `[r, g, b, amount]` mixed over the model colours (`amount` 0 = none).
    pub tint: [f32; 4],
}

/// Female (member 1) without her own model: ~92 % size (the male is ~10 % larger) and a warm
/// rosy tint, so the pair is told apart without art.
pub const FEMALE_FALLBACK_LOOK: FamilyLook = FamilyLook {
    scale: 0.92,
    tint: [1.0, 0.72, 0.78, 0.16],
};
/// Baby without its own model: the adult model at ~45 % (GAME-FAMILY §8), no tint.
pub const BABY_FALLBACK_LOOK: FamilyLook = FamilyLook {
    scale: 0.45,
    tint: [0.0; 4],
};
const OWN_LOOK: FamilyLook = FamilyLook {
    scale: 1.0,
    tint: [0.0; 4],
};

/// Look of pair member `member` (0 male, 1 female): own model = as modelled, else the fallback.
pub fn member_look(species: &str, member: u8) -> FamilyLook {
    if member >= 1 && female_model(species).is_none() {
        FEMALE_FALLBACK_LOOK
    } else {
        OWN_LOOK
    }
}

/// Look of the baby of a pair species.
pub fn baby_look(species: &str) -> FamilyLook {
    if baby_model(species).is_none() {
        BABY_FALLBACK_LOOK
    } else {
        OWN_LOOK
    }
}

/// Minimum distance (m) the two animals of a pair keep from each other at their hiding place and
/// at home (centre to centre, Q-308): small animals one cell, big ones two, so bodies and
/// props never clip. Perched animals (koala) sit side by side on a wide branch instead.
pub fn pair_gap_m(species: &str) -> f32 {
    match species {
        "hippo" | "giraffe" | "elephant" => 2.0,
        "zebra" | "panda" | "lion" => 1.5,
        _ => 1.0,
    }
}

pub const ANIMALS: [AnimalInfo; 16] = [
    AnimalInfo {
        id: "zebra",
        foods: &[Food::Grass],
        hiding_places: &["loc_river", "loc_meadow", "loc_sand"],
        treats: &[],
    },
    AnimalInfo {
        id: "hippo",
        foods: &[Food::Melons],
        hiding_places: &["loc_pond", "loc_mud", "loc_shade"],
        treats: &[],
    },
    AnimalInfo {
        id: "panda",
        foods: &[Food::Bamboo],
        hiding_places: &["loc_cave", "loc_bamboo", "loc_leaves"],
        treats: &[],
    },
    AnimalInfo {
        id: "koala",
        foods: &[Food::Eucalyptus],
        hiding_places: &["loc_treehouse", "loc_tallest_tree", "loc_blossom_tree"],
        treats: &[],
    },
    AnimalInfo {
        id: "elephant",
        foods: &[Food::Hay],
        hiding_places: &["loc_fountain", "loc_log_pile", "loc_big_ball"],
        treats: &[],
    },
    AnimalInfo {
        id: "goldfish",
        foods: &[Food::FishFood],
        hiding_places: &["loc_waterfall", "loc_water_wheel", "loc_willow"],
        treats: &[],
    },
    AnimalInfo {
        id: "monkey",
        foods: &[Food::Bananas],
        hiding_places: &["loc_pirate_ship", "loc_carousel", "loc_trampoline"],
        treats: &[],
    },
    AnimalInfo {
        id: "giraffe",
        foods: &[Food::Leaves],
        hiding_places: &["loc_lookout_tower", "loc_train", "loc_playground"],
        treats: &[],
    },
    AnimalInfo {
        id: "lion",
        foods: &[Food::Meat],
        hiding_places: &["loc_sun_rocks", "loc_stage", "loc_deckchairs"],
        treats: &[],
    },
    AnimalInfo {
        id: "snow_fox",
        foods: &[Food::Berries],
        hiding_places: &["loc_ice_cream_kiosk", "loc_sprinkler", "loc_laundry"],
        treats: &[],
    },
    // Night zoo `night_1` (GAME-NIGHT rule 6, Q-076/Q-077); candidate places from the
    // level data (`assets/levels/night-1.toml`).
    AnimalInfo {
        id: "hedgehog",
        foods: &[Food::Beetles],
        hiding_places: &["night_1"],
        treats: &[],
    },
    AnimalInfo {
        id: "bat",
        foods: &[Food::Fruit],
        hiding_places: &["night_1"],
        treats: &[],
    },
    AnimalInfo {
        id: "owl",
        foods: &[Food::Beetles],
        hiding_places: &["night_1"],
        treats: &[],
    },
    // Night zoo `night_2`, the terrarium garden (GAME-LEVEL-NIGHT-2, Q-330..339 answered
    // 2026-10-04): basic food from the outside boxes, the treat (baby) from inside the storage.
    AnimalInfo {
        id: "snake",
        foods: &[Food::Fish],
        hiding_places: &["night_2"],
        treats: &[Food::Eggs],
    },
    AnimalInfo {
        id: "chameleon",
        foods: &[Food::Crickets],
        hiding_places: &["night_2"],
        treats: &[Food::FrozenInsects],
    },
    AnimalInfo {
        id: "poison_dart_frog",
        foods: &[Food::Flies],
        hiding_places: &["night_2"],
        treats: &[Food::Crickets],
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
    /// `fly_height` of a `fly` clip (bat, owl): origin raised this much while flying.
    fly: std::collections::BTreeMap<String, f32>,
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
        let mut fly = std::collections::BTreeMap::new();
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
                if let Some(h) = c
                    .get("fly_height")
                    .and_then(|f| f.as_float().or_else(|| f.as_integer().map(|i| i as f64)))
                {
                    fly.insert(animal.clone(), h as f32);
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
        Ok(Self { clips, climb, fly })
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

    /// `fly_height` (m) of an animal that flies (bat, owl), if any.
    pub fn fly_height(&self, animal: &str) -> Option<f32> {
        self.fly.get(animal).copied()
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

    /// Whether `food` is one of the species' box-food treats.
    pub fn is_treat(&self, food: Food) -> bool {
        self.treats.contains(&food)
    }

    /// Whether the animal at home takes `food` (basic food or treat).
    pub fn accepts(&self, food: Food) -> bool {
        self.eats(food) || self.is_treat(food)
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

/// Speed limit (m/s) of an animal's baby while it plays at home: chameleons move slowly (user
/// request 2026-10-07, FAM-035); everyone else is unlimited.
pub fn baby_home_max_speed(species: &str) -> f32 {
    match species {
        "chameleon" => 0.12,
        _ => f32::INFINITY,
    }
}

/// Multiplier of the home wander speed (GAME-ANIMALS ≈ 0.5 m/s): chameleons crawl at ≈ 0.2 m/s,
/// the authored `walk` speed of their model (FAM-035).
pub fn home_wander_scale(species: &str) -> f32 {
    match species {
        "chameleon" => 0.4,
        _ => 1.0,
    }
}
