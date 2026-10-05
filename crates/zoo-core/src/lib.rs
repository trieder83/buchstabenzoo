//! Buchstabenzoo game core: pure, deterministic game logic without web dependencies
//! (TECH-ARCH, ARCH-001). Time is passed in as `dt`; randomness comes from a seed.

pub mod ads;
pub mod ambient;
pub mod animals;
pub mod baby;
pub mod carrying;
pub mod collision;
pub mod content;
pub mod coords;
pub mod daytime;
pub mod food;
pub mod game;
pub mod garden;
pub mod ground;
pub mod hints;
pub mod house;
pub mod level;
pub mod nav;
pub mod night;
pub mod night_scene;
pub mod overview;
pub mod player;
pub mod quality;
pub mod rng;
pub mod save;
pub mod scene;
pub mod sound;
pub mod telescope;
pub mod view;
pub mod wander;
pub mod water;

pub use animals::{AnimalInfo, AnimalState, ANIMALS};
pub use content::{Content, Language, ReadingLevel};
pub use coords::{level_to_world, world_to_level};
pub use food::{Carry, Food, FoodBox, FoodStorage};
pub use game::{Game, GameEvent, InfoBoard, Interactable, Interaction, Target};
pub use level::{
    segment_run, CellKind, ElementType, Grid, Level, LevelData, Rect, Run, RunAxis, Segment,
    Surface,
};
pub use player::{MoveParams, Player};
