//! Food, food boxes and carrying food (GAME-FEED).

use crate::content::ReadingLevel;

/// A food. `id()` is the code identifier; the printed box word comes from Fluent
/// (`food-<id>`, CONT-MISSIONS §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Food {
    Grass,
    Melons,
    Bamboo,
    Eucalyptus,
    Hay,
    FishFood,
    Bananas,
    Leaves,
    Meat,
    Berries,
    /// Night zoo (GAME-NIGHT rule 6, Q-077: hedgehog and owl).
    Beetles,
    /// Night zoo (fruit bat).
    Fruit,
    /// Night zoo (later night levels: badger, kiwi — Q-135).
    Worms,
    /// Night zoo (later night levels: slow loris — Q-135).
    Nectar,
    /// Night zoo `night_2` (GAME-FEED "Basic food and treats"): basic food of the snake.
    Fish,
    /// Basic food of the chameleon, treat of the poison dart frog.
    Crickets,
    /// Basic food of the poison dart frog.
    Flies,
    /// Treat of the snake.
    Eggs,
    /// Treat of the chameleon.
    FrozenInsects,
    /// Treat of the lion (box inside the level-2 storage, not stocked yet).
    Bone,
}

impl Food {
    pub const ALL: [Food; 20] = [
        Food::Grass,
        Food::Melons,
        Food::Bamboo,
        Food::Eucalyptus,
        Food::Hay,
        Food::FishFood,
        Food::Bananas,
        Food::Leaves,
        Food::Meat,
        Food::Berries,
        Food::Beetles,
        Food::Fruit,
        Food::Worms,
        Food::Nectar,
        Food::Fish,
        Food::Crickets,
        Food::Flies,
        Food::Eggs,
        Food::FrozenInsects,
        Food::Bone,
    ];

    /// Foods of the day zoo (the day food storages, GAME-FEED).
    pub const DAY: [Food; 10] = [
        Food::Grass,
        Food::Melons,
        Food::Bamboo,
        Food::Eucalyptus,
        Food::Hay,
        Food::FishFood,
        Food::Bananas,
        Food::Leaves,
        Food::Meat,
        Food::Berries,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Food::Grass => "grass",
            Food::Melons => "melons",
            Food::Bamboo => "bamboo",
            Food::Eucalyptus => "eucalyptus",
            Food::Hay => "hay",
            Food::FishFood => "fish_food",
            Food::Bananas => "bananas",
            Food::Leaves => "leaves",
            Food::Meat => "meat",
            Food::Berries => "berries",
            Food::Beetles => "beetles",
            Food::Fruit => "fruit",
            Food::Worms => "worms",
            Food::Nectar => "nectar",
            Food::Fish => "fish",
            Food::Crickets => "crickets",
            Food::Flies => "flies",
            Food::Eggs => "eggs",
            Food::FrozenInsects => "frozen_insects",
            Food::Bone => "bone",
        }
    }

    /// Foods of the night zoo (the night food storage, GAME-NIGHT rule 5).
    pub const NIGHT: [Food; 4] = [Food::Beetles, Food::Fruit, Food::Worms, Food::Nectar];

    /// Foods of the terrarium garden's storage (night_2: basic foods and treats, GAME-FEED
    /// "Basic food and treats"); `beetles` stands there as a distractor.
    pub const NIGHT_2: [Food; 5] = [
        Food::Fish,
        Food::Crickets,
        Food::Flies,
        Food::Eggs,
        Food::FrozenInsects,
    ];

    pub fn from_id(id: &str) -> Option<Food> {
        Food::ALL.into_iter().find(|f| f.id() == id)
    }

    /// Fluent key of the box label word (CONT-MISSIONS §1: `food-<food_id>`).
    pub fn label_key(self) -> String {
        format!("food-{}", self.id())
    }
}

/// How a food box label is shown (GAME-FEED §1): a pictogram plus the word.
#[derive(Debug, Clone, PartialEq)]
pub struct FoodLabel {
    /// Fluent key of the word.
    pub word_key: String,
    /// Pictogram id (= food id), drawn by the host (`web/src/pictograms.ts`).
    pub pictogram: &'static str,
    /// Share of the label height the pictogram takes (pictogram above the word); the word
    /// gets the rest. Shrinks with the reading level: the pictogram only supports the word.
    pub pictogram_scale: f32,
}

/// Pictogram share of the label height per reading level (GAME-FEED §1).
pub fn pictogram_scale(level: ReadingLevel) -> f32 {
    match level {
        ReadingLevel::Kiga => 0.62,
        ReadingLevel::Klasse1 => 0.50,
        ReadingLevel::Klasse2 => 0.34,
        ReadingLevel::Klasse3 => 0.28,
    }
}

/// Texture id of the shared food label atlas (host-rendered, FEED-030).
pub const ATLAS_TEXTURE: &str = "text:food-atlas";
/// Size of the food label atlas in px: 20 lid cells (128 x 128, 8 per row, 3 rows) on top, 20
/// front cells (256 x 64, 4 per row, 5 rows) below. 1024 x 704 RGBA8 = 2.9 MB for ALL boxes.
pub const ATLAS_PX: (u32, u32) = (1024, 704);
/// Top of the front cells (the lid rows above take 3 x 128 px).
const ATLAS_FRONT_Y: u32 = 384;

/// Which face of the crate an atlas cell is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasPart {
    /// Pictogram + word, flat on the lid (the legible side from the high camera).
    Lid,
    /// Small pictogram + word on the crate front.
    Front,
}

impl Food {
    /// Cell `[x, y, w, h]` (px) of this food in the atlas.
    pub fn atlas_cell(self, part: AtlasPart) -> [u32; 4] {
        let i = Food::ALL.iter().position(|f| *f == self).unwrap_or(0) as u32;
        match part {
            AtlasPart::Lid => [(i % 8) * 128, (i / 8) * 128, 128, 128],
            AtlasPart::Front => [(i % 4) * 256, ATLAS_FRONT_Y + (i / 4) * 64, 256, 64],
        }
    }

    /// UV rectangle `[u0, v0, u1, v1]` of the cell, inset by half a texel (no bleeding).
    pub fn atlas_uv(self, part: AtlasPart) -> [f32; 4] {
        let [x, y, w, h] = self.atlas_cell(part);
        let (aw, ah) = (ATLAS_PX.0 as f32, ATLAS_PX.1 as f32);
        [
            (x as f32 + 0.5) / aw,
            (y as f32 + 0.5) / ah,
            ((x + w) as f32 - 0.5) / aw,
            ((y + h) as f32 - 0.5) / ah,
        ]
    }
}

/// A closed food box in the food storage (GAME-FEED §2). Never runs out (§4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoodBox {
    pub food: Food,
}

impl FoodBox {
    pub fn label(&self, level: ReadingLevel) -> FoodLabel {
        FoodLabel {
            word_key: self.food.label_key(),
            pictogram: self.food.id(),
            pictogram_scale: pictogram_scale(level),
        }
    }
}

/// The food storage: a box of every food (proposal Q-047; the props in the level data decide
/// which boxes stand in which storage, Q-089).
#[derive(Debug, Clone)]
pub struct FoodStorage {
    pub boxes: Vec<FoodBox>,
}

impl Default for FoodStorage {
    fn default() -> Self {
        Self {
            boxes: Food::ALL.into_iter().map(|food| FoodBox { food }).collect(),
        }
    }
}

impl FoodStorage {
    pub fn has_box(&self, food: Food) -> bool {
        self.boxes.iter().any(|b| b.food == food)
    }
}

/// What the player carries: at most one food (GAME-FEED §3).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Carry {
    food: Option<Food>,
}

impl Carry {
    pub fn food(&self) -> Option<Food> {
        self.food
    }

    /// Takes food from a box. The currently carried food (if any) is put back into its box and
    /// returned. Boxes never run out.
    pub fn take(&mut self, from: &FoodBox) -> Option<Food> {
        self.food.replace(from.food)
    }

    /// The animals ate the food (GAME-RESCUE §8).
    pub fn consume(&mut self) -> Option<Food> {
        self.food.take()
    }
}
