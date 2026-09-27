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
}

impl Food {
    pub const ALL: [Food; 14] = [
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
        }
    }

    /// Foods of the night zoo (the night food storage, GAME-NIGHT rule 5).
    pub const NIGHT: [Food; 4] = [Food::Beetles, Food::Fruit, Food::Worms, Food::Nectar];

    pub fn from_id(id: &str) -> Option<Food> {
        Food::ALL.into_iter().find(|f| f.id() == id)
    }

    /// Fluent key of the box label word (CONT-MISSIONS §1: `food-<food_id>`).
    pub fn label_key(self) -> String {
        format!("food-{}", self.id())
    }
}

/// How a food box label is shown (GAME-FEED §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoodLabel {
    /// Fluent key of the word.
    pub word_key: String,
    /// `kiga`: a picture of the food next to the word.
    pub picture: bool,
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
            picture: level == ReadingLevel::Kiga,
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
