//! Buchstabenzoo game core: pure, deterministic game logic without web dependencies
//! (TECH-ARCH, ARCH-001). Time is passed in as `dt`; randomness comes from a seed.

pub mod animals;
pub mod content;
pub mod coords;
pub mod food;
pub mod game;
pub mod level;
pub mod nav;
pub mod player;
pub mod rng;

pub use animals::{AnimalInfo, AnimalState, ANIMALS};
pub use content::{Content, Language, ReadingLevel};
pub use coords::{level_to_world, world_to_level};
pub use food::{Carry, Food, FoodBox, FoodStorage};
pub use game::{Game, GameEvent, InfoBoard};
pub use level::{
    segment_run, CellKind, ElementType, Grid, Level, LevelData, Rect, Run, RunAxis, Segment,
    Surface,
};
pub use player::{MoveParams, Player};
