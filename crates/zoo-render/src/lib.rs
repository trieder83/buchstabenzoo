//! WebGL2 renderer (TECH-ARCH §7) — stub until milestone M3.
//!
//! Holds only the camera parameters of GAME-PLAYER §2 for now; the WebGL2 code (via
//! `web-sys`) arrives with M3.

/// High-angle follow camera defaults (GAME-PLAYER §2, PLAY-008/009).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraParams {
    /// Downward pitch in degrees.
    pub pitch_deg: f32,
    /// Default distance to the player in metres.
    pub distance_m: f32,
    /// Zoom limits in metres.
    pub min_distance_m: f32,
    pub max_distance_m: f32,
    /// Vertical field of view in degrees (every orientation, Q-052).
    pub vertical_fov_deg: f32,
    /// Rotation step in degrees.
    pub yaw_step_deg: f32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            pitch_deg: 55.0,
            distance_m: 14.0,
            min_distance_m: 10.0,
            max_distance_m: 20.0,
            vertical_fov_deg: 35.0,
            yaw_step_deg: 45.0,
        }
    }
}
