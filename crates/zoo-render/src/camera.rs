//! High-angle follow camera (GAME-PLAYER §2). Pure math — no GL — so it is unit-tested
//! natively.
//!
//! World space is glTF space (right-handed, Y-up, level north = world −Z, GAME-LAYOUT
//! "Coordinate spaces"). `yaw = 0` looks north; a positive yaw turns the view
//! counter-clockwise seen from above (north → west), like model rotations.

use glam::{Mat4, Vec2, Vec3, Vec4};

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

/// Height above the player's feet the camera looks at (keeps the character centred).
pub const LOOK_AT_HEIGHT_M: f32 = 0.7;
pub const NEAR_M: f32 = 0.5;
pub const FAR_M: f32 = 120.0;
/// Easing rate of rotation and zoom (1/s): ≈ 95 % of a step in 0.35 s.
const EASE_RATE: f32 = 9.0;

#[derive(Debug, Clone)]
pub struct FollowCamera {
    pub params: CameraParams,
    yaw_steps: i32,
    yaw: f32,
    distance: f32,
    target_distance: f32,
    /// World point the camera looks at.
    pub target: Vec3,
}

impl FollowCamera {
    /// Camera looking north (`yaw_steps = 0`) at `distance` metres (clamped to the zoom range).
    pub fn new(params: CameraParams, distance: f32) -> Self {
        let d = distance.clamp(params.min_distance_m, params.max_distance_m);
        Self {
            params,
            yaw_steps: 0,
            yaw: 0.0,
            distance: d,
            target_distance: d,
            target: Vec3::ZERO,
        }
    }

    /// Rotates the target yaw by whole 45° steps (positive = counter-clockwise from above).
    pub fn rotate_steps(&mut self, steps: i32) {
        self.yaw_steps += steps;
    }

    /// Multiplies the target distance (`< 1` zooms in), clamped to the zoom range.
    pub fn zoom_by(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.target_distance = (self.target_distance * factor)
                .clamp(self.params.min_distance_m, self.params.max_distance_m);
        }
    }

    pub fn target_yaw(&self) -> f32 {
        self.yaw_steps as f32 * self.params.yaw_step_deg.to_radians()
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Eases yaw and zoom towards their targets and follows `player_world` (feet position).
    pub fn update(&mut self, dt: f32, player_world: Vec3) {
        let k = 1.0 - (-EASE_RATE * dt.max(0.0)).exp();
        let ty = self.target_yaw();
        self.yaw += (ty - self.yaw) * k;
        if (ty - self.yaw).abs() < 1e-4 {
            self.yaw = ty;
        }
        self.distance += (self.target_distance - self.distance) * k;
        if (self.target_distance - self.distance).abs() < 1e-4 {
            self.distance = self.target_distance;
        }
        self.target = player_world + Vec3::Y * LOOK_AT_HEIGHT_M;
    }

    /// Jumps to the targets (no easing), e.g. at level start.
    pub fn snap(&mut self, player_world: Vec3) {
        self.yaw = self.target_yaw();
        self.distance = self.target_distance;
        self.target = player_world + Vec3::Y * LOOK_AT_HEIGHT_M;
    }

    /// Horizontal viewing direction in world space.
    pub fn forward_world(&self) -> Vec3 {
        Vec3::new(-self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    /// Horizontal viewing direction in level coordinates `(x east, z north)`.
    pub fn forward_level(&self) -> Vec2 {
        Vec2::new(-self.yaw.sin(), self.yaw.cos())
    }

    /// Screen-right direction in level coordinates.
    pub fn right_level(&self) -> Vec2 {
        Vec2::new(self.yaw.cos(), self.yaw.sin())
    }

    /// Converts a screen-space stick `(right, up)` into a level direction.
    pub fn stick_to_level(&self, stick: Vec2) -> Vec2 {
        self.right_level() * stick.x + self.forward_level() * stick.y
    }

    pub fn eye(&self) -> Vec3 {
        let p = self.params.pitch_deg.to_radians();
        self.target - self.forward_world() * (p.cos() * self.distance)
            + Vec3::Y * (p.sin() * self.distance)
    }

    pub fn view(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.eye(), self.target, Vec3::Y)
    }

    pub fn projection(&self, aspect: f32) -> Mat4 {
        glam::camera::rh::proj::opengl::perspective(
            self.params.vertical_fov_deg.to_radians(),
            aspect.max(1e-3),
            NEAR_M,
            FAR_M,
        )
    }

    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        self.projection(aspect) * self.view()
    }
}

/// Projects a world point to normalised device coordinates (`None` behind the camera).
pub fn project(view_proj: Mat4, p: Vec3) -> Option<Vec3> {
    let c: Vec4 = view_proj * p.extend(1.0);
    (c.w > 1e-6).then(|| c.truncate() / c.w)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PORTRAIT: f32 = 1080.0 / 2340.0;

    // PLAY-008
    #[test]
    fn play_008_default_pitch_distance_fov() {
        let mut cam = FollowCamera::new(CameraParams::default(), 14.0);
        cam.snap(Vec3::ZERO);
        let d = cam.target - cam.eye();
        assert!((d.length() - 14.0).abs() < 0.5);
        let pitch = (-d.y / Vec2::new(d.x, d.z).length()).atan().to_degrees();
        assert!((pitch - 55.0).abs() < 2.0, "pitch {pitch}");
        for aspect in [PORTRAIT, 1.0 / PORTRAIT] {
            let p = cam.projection(aspect);
            let fov = 2.0 * (1.0 / p.y_axis.y).atan();
            assert!((fov.to_degrees() - 35.0).abs() < 1e-3);
        }
    }

    // PLAY-009
    #[test]
    fn play_009_zoom_limits_and_no_sky() {
        let mut cam = FollowCamera::new(CameraParams::default(), 14.0);
        cam.zoom_by(100.0);
        cam.snap(Vec3::ZERO);
        assert_eq!(cam.distance(), 20.0);
        cam.zoom_by(0.001);
        cam.snap(Vec3::ZERO);
        assert_eq!(cam.distance(), 10.0);
        for step in 0..8 {
            cam.rotate_steps(1);
            cam.snap(Vec3::ZERO);
            // The top screen edge ray points below the horizon.
            let inv = cam.view_proj(PORTRAIT).inverse();
            let near = inv.project_point3(Vec3::new(0.0, 1.0, -1.0));
            let far = inv.project_point3(Vec3::new(0.0, 1.0, 1.0));
            assert!(far.y < near.y, "step {step}: sky visible");
        }
    }

    // PLAY-011: yaw settles on a multiple of 45° without a jump.
    #[test]
    fn play_011_yaw_eases_to_step() {
        let mut cam = FollowCamera::new(CameraParams::default(), 14.0);
        cam.snap(Vec3::ZERO);
        cam.rotate_steps(1);
        let mut last = cam.yaw();
        for _ in 0..120 {
            cam.update(1.0 / 60.0, Vec3::ZERO);
            assert!(cam.yaw() - last < 20f32.to_radians(), "no snapping jump");
            last = cam.yaw();
        }
        assert!((cam.yaw() - 45f32.to_radians()).abs() < 1e-3);
    }

    #[test]
    fn stick_up_walks_away_from_the_camera() {
        let mut cam = FollowCamera::new(CameraParams::default(), 14.0);
        cam.snap(Vec3::ZERO);
        // Looking north: up = level north, right = level east.
        assert!(cam.stick_to_level(Vec2::Y).abs_diff_eq(Vec2::Y, 1e-6));
        assert!(cam.stick_to_level(Vec2::X).abs_diff_eq(Vec2::X, 1e-6));
        cam.rotate_steps(2); // looking west
        cam.snap(Vec3::ZERO);
        assert!(cam.stick_to_level(Vec2::Y).abs_diff_eq(-Vec2::X, 1e-5));
        assert!(cam.stick_to_level(Vec2::X).abs_diff_eq(Vec2::Y, 1e-5));
    }
}
