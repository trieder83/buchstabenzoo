//! Camera views (GAME-CAMERA-VIEWS): the high-angle **zoo view** (default, GAME-PLAYER §2),
//! the **look-around view** (held) and the **first-person view** (toggled). Pure data and
//! rules — the camera math lives in `zoo-render::camera`, the host only forwards input.
//!
//! Distances are in metres; directions use level coordinates `(x east, z north)`.

use glam::Vec2;

/// Which camera view is active (GAME-CAMERA-VIEWS 1–3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    /// High-angle follow camera (GAME-PLAYER §2), the default.
    #[default]
    Zoo,
    /// Low camera behind the player while a button is held (behaviour 2).
    LookAround,
    /// Eye-height camera, toggled (behaviour 3).
    FirstPerson,
}

impl ViewMode {
    /// Stable id (settings, debug getters).
    pub fn id(self) -> &'static str {
        match self {
            ViewMode::Zoo => "zoo",
            ViewMode::LookAround => "look_around",
            ViewMode::FirstPerson => "first_person",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "zoo" => Some(ViewMode::Zoo),
            "look_around" => Some(ViewMode::LookAround),
            "first_person" => Some(ViewMode::FirstPerson),
            _ => None,
        }
    }

    /// A close view: sky, fog and the short draw distance apply (behaviours 5–7).
    pub fn is_close(self) -> bool {
        self != ViewMode::Zoo
    }

    /// The player's body is not drawn (first person, behaviour 3).
    pub fn hides_player(self) -> bool {
        self == ViewMode::FirstPerson
    }

    /// Objects between camera and player fade (GAME-PLAYER §2) — not in first person.
    pub fn occluder_fade(self) -> bool {
        self != ViewMode::FirstPerson
    }

    /// The view that is stored in the settings: look-around is only held, never saved
    /// (behaviour 9).
    pub fn saved(self) -> ViewMode {
        match self {
            ViewMode::LookAround => ViewMode::Zoo,
            m => m,
        }
    }
}

/// Glide time between two views (behaviour 1, 2, 3).
pub const TRANSITION_S: f32 = 0.4;

/// First person: eye height above the feet (behaviour 3).
pub const FP_EYE_HEIGHT_M: f32 = 1.1;
/// First person: pitch limit up and down (behaviour 3).
pub const FP_PITCH_LIMIT_DEG: f32 = 20.0;

/// Look-around: camera distance behind the player (behaviour 2).
pub const LOOK_BACK_M: f32 = 3.5;
/// Look-around: camera height above the player's feet (behaviour 2).
pub const LOOK_UP_M: f32 = 1.6;
/// Look-around: downward pitch (behaviour 2).
pub const LOOK_PITCH_DEG: f32 = 12.0;
/// Look-around: the view turns at most this far from where it started (behaviour 2).
pub const LOOK_YAW_LIMIT_DEG: f32 = 180.0;

/// Vertical field of view of both close views (behaviour 2, 3).
pub const CLOSE_FOV_DEG: f32 = 50.0;
/// Near plane of the close views: walls right in front of the eye are not cut open
/// (behaviour 8).
pub const CLOSE_NEAR_M: f32 = 0.05;

/// Distance fog of the close views (behaviour 5): starts here …
pub const FOG_START_M: f32 = 9.0;
/// … and fully hides everything from here on (the visibility distance).
pub const FOG_END_M: f32 = 16.0;
/// Far plane / draw distance of the close views (behaviour 6): just past the fog end.
pub const CLOSE_FAR_M: f32 = FOG_END_M + 2.0;

/// Turn speed of a drag (mouse or right thumb), degrees per CSS pixel (behaviour 4, slow).
pub const TURN_DEG_PER_PX: f32 = 0.25;
/// Turn speed of the arrow keys in first person, degrees per second (behaviour 4).
pub const KEY_TURN_DEG_S: f32 = 90.0;
/// Easing rate (1/s) of the view turn towards the dragged target (no jerks, behaviour 4).
pub const TURN_EASE_RATE: f32 = 12.0;

/// Fog amount (0 = clear, 1 = fully hidden) at a distance from the eye (behaviour 5): a
/// smoothstep from [`FOG_START_M`] to [`FOG_END_M`].
pub fn fog_amount(distance_m: f32) -> f32 {
    let t = ((distance_m - FOG_START_M) / (FOG_END_M - FOG_START_M)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smallest distance from a close-view eye of a player standing at `stand` to a point at
/// level position `p` and height `h` **that the eye can see** (behaviour 5, CAMV-008).
///
/// The first-person eye is above `stand`. The look-around eye is [`LOOK_BACK_M`] *behind*
/// the player along its look direction, so every point inside its view (horizontal half
/// field of view < 50° on every screen shape) more than ≈ 2.6 m away is farther from that
/// eye than from the player (CAMV-009 checks this on the real camera). Hence the horizontal
/// distance from `stand` is the bound for both views, combined with the smaller height
/// difference to the two eye heights.
pub fn min_eye_distance(stand: Vec2, p: Vec2, h: f32) -> f32 {
    let d = stand.distance(p);
    let dy = (h - FP_EYE_HEIGHT_M).abs().min((h - LOOK_UP_M).abs());
    Vec2::new(d, dy).length()
}

/// Whether a point is fully hidden by the fog for every close-view eye of a player at
/// `stand` (behaviour 5).
pub fn hidden_by_fog(stand: Vec2, p: Vec2, h: f32) -> bool {
    min_eye_distance(stand, p, h) >= FOG_END_M
}

/// Horizontal level direction of a view yaw (same convention as the camera: `yaw = 0` looks
/// north, positive turns counter-clockwise seen from above).
pub fn yaw_to_level(yaw: f32) -> Vec2 {
    Vec2::new(-yaw.sin(), yaw.cos())
}

/// Inverse of [`yaw_to_level`].
pub fn level_to_yaw(dir: Vec2) -> f32 {
    (-dir.x).atan2(dir.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_round_trip() {
        for m in [ViewMode::Zoo, ViewMode::LookAround, ViewMode::FirstPerson] {
            assert_eq!(ViewMode::from_id(m.id()), Some(m));
        }
        assert_eq!(ViewMode::from_id("x"), None);
        assert_eq!(ViewMode::LookAround.saved(), ViewMode::Zoo);
        assert_eq!(ViewMode::FirstPerson.saved(), ViewMode::FirstPerson);
    }

    #[test]
    fn fog_is_clear_near_and_full_at_the_end() {
        assert_eq!(fog_amount(0.0), 0.0);
        assert_eq!(fog_amount(FOG_START_M), 0.0);
        assert!(fog_amount(12.0) > 0.0 && fog_amount(12.0) < 1.0);
        assert_eq!(fog_amount(FOG_END_M), 1.0);
        assert_eq!(fog_amount(100.0), 1.0);
    }

    #[test]
    fn yaw_and_level_direction_agree() {
        for k in 0..8 {
            let yaw = k as f32 * 0.7 - 2.0;
            let d = yaw_to_level(yaw);
            let back = yaw_to_level(level_to_yaw(d));
            assert!(back.abs_diff_eq(d, 1e-5));
        }
        assert!(yaw_to_level(0.0).abs_diff_eq(Vec2::Y, 1e-6));
    }

    #[test]
    fn eye_distance_is_horizontal_plus_height_difference() {
        let d = min_eye_distance(Vec2::ZERO, Vec2::new(20.0, 0.0), LOOK_UP_M);
        assert!((d - 20.0).abs() < 1e-4);
        let d = min_eye_distance(Vec2::ZERO, Vec2::new(12.0, 0.0), LOOK_UP_M + 9.0);
        assert!((d - 15.0).abs() < 1e-4);
        assert!(!hidden_by_fog(Vec2::ZERO, Vec2::new(12.0, 0.0), 10.6));
        assert!(hidden_by_fog(Vec2::ZERO, Vec2::new(16.0, 0.0), 1.1));
    }
}
