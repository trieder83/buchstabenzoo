//! WebGL2 renderer (TECH-ARCH §7): comic look — 2-tone cel shading and a screen-space
//! outline pass (Q-050) — plus the high-angle follow camera (GAME-PLAYER §2) and the level
//! assembly re-exported from `zoo_core::scene` (the logic owns prop placements and their
//! collision footprints, GAME-PLAYER §7; the renderer only draws them).
//!
//! `camera` is pure (unit-tested natively); `renderer` talks to WebGL2 through `web-sys` and
//! only runs in the browser.

pub mod camera;
pub mod renderer;
pub mod shaders;

pub use camera::{CameraParams, FollowCamera};
pub use renderer::{CharacterDraw, FrameStats, Instance, Renderer};
pub use zoo_core::scene;
pub use zoo_core::scene::{BoxPlacement, LevelScene, Placement};
