//! Buchstabenzoo game core: pure, deterministic game logic without web dependencies
//! (TECH-ARCH, ARCH-001). Time is passed in as `dt`; randomness comes from a seed.

pub mod animals;
pub mod content;
pub mod food;
pub mod game;
pub mod level;
pub mod nav;
pub mod player;
pub mod rng;

pub use animals::{AnimalInfo, AnimalState, ANIMALS};
pub use content::{Content, Language, ReadingLevel};
pub use food::{Carry, Food, FoodBox, FoodStorage};
pub use game::{Game, GameEvent, InfoBoard};
pub use level::{CellKind, ElementType, Grid, Level, LevelData, Rect, Surface};
pub use player::{MoveParams, Player};
