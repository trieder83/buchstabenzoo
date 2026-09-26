//! WebGL2 renderer (TECH-ARCH §7): comic look — 2-tone cel shading and a screen-space
//! outline pass (Q-050) — plus the high-angle follow camera (GAME-PLAYER §2) and the level
//! assembly that turns layout data into model placements (GAME-LAYOUT).
//!
//! `camera` and `scene` are pure (unit-tested natively); `renderer` talks to WebGL2 through
//! `web-sys` and only runs in the browser.

pub mod camera;
pub mod renderer;
pub mod scene;
pub mod shaders;

pub use camera::{CameraParams, FollowCamera};
pub use renderer::{CharacterDraw, FrameStats, Instance, Renderer};
pub use scene::{BoxPlacement, LevelScene, Placement};
