//! High-angle follow camera (GAME-PLAYER §2) and the close look-around / first-person views
//! it glides to (GAME-CAMERA-VIEWS). Pure math — no GL — so it is unit-tested natively.
//!
//! World space is glTF space (right-handed, Y-up, level north = world −Z, GAME-LAYOUT
//! "Coordinate spaces"). `yaw = 0` looks north; a positive yaw turns the view
//! counter-clockwise seen from above (north → west), like model rotations.

use glam::{Mat4, Vec2, Vec3, Vec4};
use zoo_core::view::{
    self, ViewMode, CLOSE_FAR_M, CLOSE_FOV_DEG, CLOSE_NEAR_M, FOG_END_M, FOG_START_M,
    FP_EYE_HEIGHT_M, FP_PITCH_LIMIT_DEG, LOOK_BACK_M, LOOK_PITCH_DEG, LOOK_UP_M,
    LOOK_YAW_LIMIT_DEG, TRANSITION_S, TURN_EASE_RATE,
};

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
    /// Requested view (GAME-CAMERA-VIEWS 1).
    mode: ViewMode,
    /// The close view being blended in or out (`LookAround` or `FirstPerson`).
    close: ViewMode,
    /// Glide progress: 0 = zoo pose, 1 = close pose (linear; eased when used).
    blend: f32,
    /// Close-view yaw (same convention as `yaw`) and its dragged target.
    look_yaw: f32,
    look_yaw_target: f32,
    /// Yaw where look-around started (turn limit ±180°, behaviour 2).
    look_origin: f32,
    /// First-person pitch (radians, positive = up) and its dragged target.
    look_pitch: f32,
    look_pitch_target: f32,
}

/// One camera pose (the zoo pose, a close pose or a blend of both).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    pub eye: Vec3,
    /// Yaw of the view direction (`0` = north, counter-clockwise from above).
    pub yaw: f32,
    /// Pitch of the view direction (radians, negative = down).
    pub pitch: f32,
    pub fov_deg: f32,
    pub near: f32,
    pub far: f32,
}

impl Pose {
    pub fn dir(&self) -> Vec3 {
        let (sp, cp) = self.pitch.sin_cos();
        Vec3::new(-self.yaw.sin() * cp, sp, -self.yaw.cos() * cp)
    }
}

/// Distance fog of a frame (GAME-CAMERA-VIEWS 5): `amount` 0 = off (zoo view), 1 = full.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fog {
    pub start: f32,
    pub end: f32,
    pub amount: f32,
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Shortest signed angle from `from` to `to` (radians).
fn angle_diff(to: f32, from: f32) -> f32 {
    let d = (to - from).rem_euclid(std::f32::consts::TAU);
    if d > std::f32::consts::PI {
        d - std::f32::consts::TAU
    } else {
        d
    }
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
            mode: ViewMode::Zoo,
            close: ViewMode::FirstPerson,
            blend: 0.0,
            look_yaw: 0.0,
            look_yaw_target: 0.0,
            look_origin: 0.0,
            look_pitch: 0.0,
            look_pitch_target: 0.0,
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

    /// Rotation in whole 45° steps (GAME-SAVE: saved with the zoom distance).
    pub fn yaw_steps(&self) -> i32 {
        self.yaw_steps
    }

    /// Target zoom distance in metres.
    pub fn target_distance(&self) -> f32 {
        self.target_distance
    }

    /// Restores a saved rotation and zoom at once (no easing; GAME-SAVE §4).
    pub fn set_state(&mut self, yaw_steps: i32, distance: f32) {
        self.yaw_steps = yaw_steps;
        if distance.is_finite() {
            self.target_distance =
                distance.clamp(self.params.min_distance_m, self.params.max_distance_m);
        }
        self.yaw = self.target_yaw();
        self.distance = self.target_distance;
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
        // GAME-CAMERA-VIEWS: glide between the zoo pose and the close pose, ease the turn
        let goal = if self.mode.is_close() { 1.0 } else { 0.0 };
        let step = dt.max(0.0) / TRANSITION_S;
        self.blend += (goal - self.blend).clamp(-step, step);
        let k = 1.0 - (-TURN_EASE_RATE * dt.max(0.0)).exp();
        self.look_yaw += (self.look_yaw_target - self.look_yaw) * k;
        self.look_pitch += (self.look_pitch_target - self.look_pitch) * k;
    }

    /// Jumps to the targets (no easing), e.g. at level start.
    pub fn snap(&mut self, player_world: Vec3) {
        self.yaw = self.target_yaw();
        self.distance = self.target_distance;
        self.target = player_world + Vec3::Y * LOOK_AT_HEIGHT_M;
        self.blend = if self.mode.is_close() { 1.0 } else { 0.0 };
        self.look_yaw = self.look_yaw_target;
        self.look_pitch = self.look_pitch_target;
    }

    // ------------------------------------------------------------ views (GAME-CAMERA-VIEWS)

    /// The requested view.
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// The view the glide currently shows most (the requested one once it is past halfway).
    pub fn shown(&self) -> ViewMode {
        if self.blend >= 0.5 {
            self.close
        } else {
            ViewMode::Zoo
        }
    }

    /// Glide progress 0 (zoo pose) … 1 (close pose), eased.
    pub fn blend(&self) -> f32 {
        smoothstep(self.blend)
    }

    /// Switches the view (behaviour 1–3). `facing_yaw` is the player's facing as a yaw (first
    /// person starts looking that way). Look-around is only entered from the zoo view;
    /// returns `false` when the switch is not allowed.
    pub fn set_view(&mut self, mode: ViewMode, facing_yaw: f32) -> bool {
        if mode == self.mode {
            return true;
        }
        match mode {
            ViewMode::Zoo => {}
            ViewMode::LookAround => {
                if self.mode != ViewMode::Zoo {
                    return false;
                }
                // start where the zoo view looks (no spin)
                self.set_look(self.yaw, -LOOK_PITCH_DEG.to_radians());
                self.look_origin = self.yaw;
            }
            ViewMode::FirstPerson => self.set_look(facing_yaw, 0.0),
        }
        if mode.is_close() {
            if self.blend > 0.0 && self.close != mode {
                self.blend = 0.0; // never blend two close poses into each other
            }
            self.close = mode;
        }
        self.mode = mode;
        true
    }

    fn set_look(&mut self, yaw: f32, pitch: f32) {
        self.look_yaw = yaw;
        self.look_yaw_target = yaw;
        self.look_pitch = pitch;
        self.look_pitch_target = pitch;
    }

    /// Turns a close view by `d_yaw` (positive = counter-clockwise from above) and — in first
    /// person — `d_pitch` (positive = up), eased (behaviour 2–4). Ignored in the zoo view.
    pub fn turn(&mut self, d_yaw: f32, d_pitch: f32) {
        if !d_yaw.is_finite() || !d_pitch.is_finite() {
            return;
        }
        match self.mode {
            ViewMode::Zoo => {}
            ViewMode::LookAround => {
                let lim = LOOK_YAW_LIMIT_DEG.to_radians();
                self.look_yaw_target = self.look_origin
                    + (self.look_yaw_target + d_yaw - self.look_origin).clamp(-lim, lim);
            }
            ViewMode::FirstPerson => {
                let lim = FP_PITCH_LIMIT_DEG.to_radians();
                self.look_yaw_target += d_yaw;
                self.look_pitch_target = (self.look_pitch_target + d_pitch).clamp(-lim, lim);
            }
        }
    }

    /// Close-view yaw (the direction the child looks in first person / look-around).
    pub fn look_yaw(&self) -> f32 {
        self.look_yaw
    }

    /// Close-view look direction in level coordinates (first person: the player's facing).
    pub fn look_level(&self) -> Vec2 {
        view::yaw_to_level(self.look_yaw)
    }

    /// The zoo pose (GAME-PLAYER §2).
    pub fn zoo_pose(&self) -> Pose {
        let p = self.params.pitch_deg.to_radians();
        let fwd = Vec3::new(-self.yaw.sin(), 0.0, -self.yaw.cos());
        Pose {
            eye: self.target - fwd * (p.cos() * self.distance)
                + Vec3::Y * (p.sin() * self.distance),
            yaw: self.yaw,
            pitch: -p,
            fov_deg: self.params.vertical_fov_deg,
            near: NEAR_M,
            far: FAR_M,
        }
    }

    /// The pose of a close view at the current look direction.
    pub fn close_pose(&self, mode: ViewMode) -> Pose {
        let feet = self.target - Vec3::Y * LOOK_AT_HEIGHT_M;
        let fwd = Vec3::new(-self.look_yaw.sin(), 0.0, -self.look_yaw.cos());
        let (eye, pitch) = match mode {
            ViewMode::LookAround => (
                feet + Vec3::Y * LOOK_UP_M - fwd * LOOK_BACK_M,
                -LOOK_PITCH_DEG.to_radians(),
            ),
            _ => (feet + Vec3::Y * FP_EYE_HEIGHT_M, self.look_pitch),
        };
        Pose {
            eye,
            yaw: self.look_yaw,
            pitch,
            fov_deg: CLOSE_FOV_DEG,
            near: CLOSE_NEAR_M,
            far: CLOSE_FAR_M,
        }
    }

    /// The pose drawn this frame: the zoo pose and the close pose blended (behaviour 1).
    pub fn pose(&self) -> Pose {
        let zoo = self.zoo_pose();
        let s = self.blend();
        if s <= 0.0 {
            return zoo;
        }
        let close = self.close_pose(self.close);
        Pose {
            eye: zoo.eye.lerp(close.eye, s),
            yaw: zoo.yaw + angle_diff(close.yaw, zoo.yaw) * s,
            pitch: zoo.pitch + (close.pitch - zoo.pitch) * s,
            fov_deg: zoo.fov_deg + (close.fov_deg - zoo.fov_deg) * s,
            near: (zoo.near.ln() + (close.near.ln() - zoo.near.ln()) * s).exp(),
            far: self.fog().end + (CLOSE_FAR_M - FOG_END_M),
        }
    }

    /// Distance fog of this frame (behaviour 5): off in the zoo view, 9 → 16 m in the close
    /// views; while gliding it moves in from the zoo view's far plane.
    pub fn fog(&self) -> Fog {
        let s = self.blend();
        let end = (FAR_M - 2.0) + (FOG_END_M - (FAR_M - 2.0)) * s;
        Fog {
            start: end * (FOG_START_M / FOG_END_M),
            end,
            amount: s,
        }
    }

    /// How much sky is drawn where there is no geometry (0 in the zoo view, behaviour 7).
    pub fn sky_amount(&self) -> f32 {
        self.blend()
    }

    /// Vertical field of view of this frame in degrees.
    pub fn fov_deg(&self) -> f32 {
        self.pose().fov_deg
    }

    pub fn near(&self) -> f32 {
        self.pose().near
    }

    pub fn far(&self) -> f32 {
        self.pose().far
    }

    /// The occluder fade applies (GAME-PLAYER §2) — not in first person (behaviour 8).
    pub fn occluder_fade(&self) -> bool {
        self.shown().occluder_fade()
    }

    /// The player's body is not drawn (first person, behaviour 3).
    pub fn hides_player(&self) -> bool {
        self.shown().hides_player()
    }

    /// Distance from the eye to the look-at point of the player (occluder fade radius).
    pub fn fade_distance(&self) -> f32 {
        self.eye().distance(self.target)
    }

    /// Yaw of the view drawn this frame (zoo yaw, close-view yaw or the glide between).
    pub fn view_yaw(&self) -> f32 {
        if self.blend <= 0.0 {
            return self.yaw;
        }
        self.pose().yaw
    }

    /// Horizontal viewing direction in world space.
    pub fn forward_world(&self) -> Vec3 {
        let y = self.view_yaw();
        Vec3::new(-y.sin(), 0.0, -y.cos())
    }

    /// Horizontal viewing direction in level coordinates `(x east, z north)`.
    pub fn forward_level(&self) -> Vec2 {
        let y = self.view_yaw();
        Vec2::new(-y.sin(), y.cos())
    }

    /// Screen-right direction in level coordinates.
    pub fn right_level(&self) -> Vec2 {
        let y = self.view_yaw();
        Vec2::new(y.cos(), y.sin())
    }

    /// Converts a screen-space stick `(right, up)` into a level direction.
    pub fn stick_to_level(&self, stick: Vec2) -> Vec2 {
        self.right_level() * stick.x + self.forward_level() * stick.y
    }

    pub fn eye(&self) -> Vec3 {
        self.pose().eye
    }

    pub fn view(&self) -> Mat4 {
        if self.blend <= 0.0 {
            return glam::camera::rh::view::look_at_mat4(self.eye(), self.target, Vec3::Y);
        }
        let p = self.pose();
        glam::camera::rh::view::look_to_mat4(p.eye, p.dir(), Vec3::Y)
    }

    pub fn projection(&self, aspect: f32) -> Mat4 {
        let p = self.pose();
        glam::camera::rh::proj::opengl::perspective(
            p.fov_deg.to_radians(),
            aspect.max(1e-3),
            p.near,
            p.far,
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
    use std::f32::consts::PI;

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

    // ------------------------------------------------------------ GAME-CAMERA-VIEWS

    const DT: f32 = 1.0 / 60.0;
    const LANDSCAPE: f32 = 2340.0 / 1080.0;

    fn zoo_cam() -> FollowCamera {
        let mut cam = FollowCamera::new(CameraParams::default(), 14.0);
        cam.rotate_steps(1);
        cam.snap(Vec3::new(3.0, 0.0, -5.0));
        cam
    }

    fn run(cam: &mut FollowCamera, seconds: f32, feet: Vec3) -> f32 {
        let mut max_jump = 0.0f32;
        let n = (seconds / DT).round() as usize;
        for _ in 0..n {
            let before = cam.eye();
            cam.update(DT, feet);
            max_jump = max_jump.max(cam.eye().distance(before));
        }
        max_jump
    }

    fn pitch_of(v: Vec3) -> f32 {
        (v.y / Vec2::new(v.x, v.z).length()).atan().to_degrees()
    }

    /// View direction of the drawn frame (from the view matrix).
    fn view_dir(cam: &FollowCamera) -> Vec3 {
        let inv = cam.view().inverse();
        (inv.transform_vector3(Vec3::NEG_Z)).normalize()
    }

    // CAMV-001
    #[test]
    fn camv_001_look_around_pose_and_glide() {
        let feet = Vec3::new(3.0, 0.0, -5.0);
        let mut cam = zoo_cam();
        let zoo = cam.zoo_pose();
        assert!(cam.set_view(ViewMode::LookAround, 0.0));
        // halfway: between both poses
        run(&mut cam, 0.2, feet);
        let mid = cam.eye();
        assert!(mid.y < zoo.eye.y && mid.y > LOOK_UP_M, "mid eye {mid}");
        let jump = run(&mut cam, 0.2, feet);
        assert!(jump < 1.0, "eye jumped {jump} m in one frame");
        assert!((cam.blend() - 1.0).abs() < 1e-6, "glide done after 0.4 s");
        let eye = cam.eye();
        let back = Vec2::new(eye.x - feet.x, eye.z - feet.z).length();
        assert!((back - LOOK_BACK_M).abs() < 0.05, "back {back}");
        assert!((eye.y - feet.y - LOOK_UP_M).abs() < 0.05);
        // behind the player = opposite of the view direction
        let d = view_dir(&cam);
        assert!(Vec2::new(d.x, d.z).dot(Vec2::new(feet.x - eye.x, feet.z - eye.z)) > 0.0);
        assert!(
            (pitch_of(d) + LOOK_PITCH_DEG).abs() < 0.5,
            "pitch {}",
            pitch_of(d)
        );
        let fov = 2.0 * (1.0 / cam.projection(PORTRAIT).y_axis.y).atan();
        assert!((fov.to_degrees() - 50.0).abs() < 1e-3);
        // it starts looking where the zoo view looked
        assert!((cam.view_yaw() - zoo.yaw).abs() < 1e-4);
        // release: back to the zoo pose (PLAY-008 values) within 0.4 s
        cam.set_view(ViewMode::Zoo, 0.0);
        let jump = run(&mut cam, 0.4, feet);
        assert!(jump < 1.0, "eye jumped {jump} m in one frame");
        assert!(cam.eye().abs_diff_eq(zoo.eye, 1e-3));
        let d = cam.target - cam.eye();
        assert!((d.length() - 14.0).abs() < 0.5);
        assert!((-pitch_of(d) - 55.0).abs() < 2.0);
    }

    // CAMV-002
    #[test]
    fn camv_002_look_around_turns_smoothly_within_180() {
        let feet = Vec3::ZERO;
        let mut cam = zoo_cam();
        let (steps, dist) = (cam.yaw_steps(), cam.target_distance());
        let start = cam.yaw();
        cam.set_view(ViewMode::LookAround, 0.0);
        run(&mut cam, 0.5, feet);
        // a small drag turns a little (no 45° step)
        cam.turn(3f32.to_radians(), 0.0);
        let mut last = cam.look_yaw();
        for _ in 0..60 {
            cam.update(DT, feet);
            assert!((cam.look_yaw() - last).abs() < 1f32.to_radians());
            last = cam.look_yaw();
        }
        assert!((cam.look_yaw() - start - 3f32.to_radians()).abs() < 1e-3);
        // never more than 180° from the start, either way
        for _ in 0..50 {
            cam.turn(20f32.to_radians(), 0.0);
        }
        run(&mut cam, 2.0, feet);
        assert!((cam.look_yaw() - start - PI).abs() < 1e-3);
        for _ in 0..100 {
            cam.turn(-20f32.to_radians(), 0.0);
        }
        run(&mut cam, 2.0, feet);
        assert!((cam.look_yaw() - start + PI).abs() < 1e-3);
        // look-around never pitches
        cam.turn(0.0, 0.5);
        run(&mut cam, 1.0, feet);
        assert!((pitch_of(view_dir(&cam)) + LOOK_PITCH_DEG).abs() < 0.5);
        // release: zoo view unchanged
        cam.set_view(ViewMode::Zoo, 0.0);
        run(&mut cam, 0.5, feet);
        assert_eq!((cam.yaw_steps(), cam.target_distance()), (steps, dist));
        assert!((cam.view_yaw() - start).abs() < 1e-5);
        // zoo view: turn is ignored; look-around not allowed from first person
        cam.turn(1.0, 0.0);
        assert!((cam.view_yaw() - start).abs() < 1e-5);
        cam.set_view(ViewMode::FirstPerson, 0.0);
        assert!(!cam.set_view(ViewMode::LookAround, 0.0));
        assert_eq!(cam.mode(), ViewMode::FirstPerson);
    }

    // CAMV-003
    #[test]
    fn camv_003_first_person_pose_pitch_limit_and_back() {
        let feet = Vec3::new(-2.0, 0.0, 4.0);
        let mut cam = zoo_cam();
        cam.snap(feet);
        let zoo = cam.zoo_pose();
        let facing = 2.0f32; // yaw of the player's facing
        cam.set_view(ViewMode::FirstPerson, facing);
        let jump = run(&mut cam, 0.4, feet);
        assert!(jump < 1.0);
        let eye = cam.eye();
        assert!(
            eye.abs_diff_eq(feet + Vec3::Y * FP_EYE_HEIGHT_M, 1e-4),
            "{eye}"
        );
        let d = view_dir(&cam);
        let want = view::yaw_to_level(facing);
        assert!(Vec2::new(d.x, -d.z).abs_diff_eq(want, 1e-4), "dir {d}");
        assert!(pitch_of(d).abs() < 1e-3);
        let p = cam.projection(LANDSCAPE);
        let fov = 2.0 * (1.0 / p.y_axis.y).atan();
        assert!((fov.to_degrees() - 50.0).abs() < 1e-3);
        assert!((cam.near() - 0.05).abs() < 1e-6);
        // pitch clamps at ±20°
        cam.turn(0.0, 2.0);
        run(&mut cam, 2.0, feet);
        assert!((pitch_of(view_dir(&cam)) - 20.0).abs() < 0.05);
        cam.turn(0.0, -5.0);
        run(&mut cam, 2.0, feet);
        assert!((pitch_of(view_dir(&cam)) + 20.0).abs() < 0.05);
        assert!(cam.hides_player() && !cam.occluder_fade());
        // back to the zoo pose
        cam.set_view(ViewMode::Zoo, 0.0);
        run(&mut cam, 0.4, feet);
        let zoo_now = FollowCamera {
            target: feet + Vec3::Y * LOOK_AT_HEIGHT_M,
            ..cam.clone()
        };
        assert!(cam.eye().abs_diff_eq(zoo_now.zoo_pose().eye, 1e-3));
        assert!((cam.view_yaw() - zoo.yaw).abs() < 1e-5);
        assert!(!cam.hides_player() && cam.occluder_fade());
    }

    // CAMV-004
    #[test]
    fn camv_004_close_views_show_the_sky() {
        for mode in [ViewMode::LookAround, ViewMode::FirstPerson] {
            for aspect in [PORTRAIT, LANDSCAPE] {
                let mut cam = zoo_cam();
                cam.set_view(mode, 0.3);
                cam.snap(Vec3::ZERO);
                let inv = cam.view_proj(aspect).inverse();
                let near = inv.project_point3(Vec3::new(0.0, 1.0, -1.0));
                let far = inv.project_point3(Vec3::new(0.0, 1.0, 1.0));
                assert!(
                    far.y > near.y,
                    "{mode:?}: top edge ray points up (sky visible)"
                );
                assert!(cam.sky_amount() > 0.99);
            }
        }
        let cam = zoo_cam();
        assert_eq!(cam.sky_amount(), 0.0);
    }

    // CAMV-005
    #[test]
    fn camv_005_fog_and_far_plane() {
        let mut cam = zoo_cam();
        let f = cam.fog();
        assert_eq!(f.amount, 0.0);
        assert!((cam.far() - FAR_M).abs() < 1e-4);
        assert!((cam.near() - NEAR_M).abs() < 1e-6);
        cam.set_view(ViewMode::FirstPerson, 0.0);
        let (mut last_end, mut last_far) = (f32::MAX, f32::MAX);
        for _ in 0..30 {
            cam.update(DT, Vec3::ZERO);
            let f = cam.fog();
            assert!(
                f.end <= last_end + 1e-4 && cam.far() <= last_far + 1e-4,
                "monotonic"
            );
            last_end = f.end;
            last_far = cam.far();
        }
        let f = cam.fog();
        assert!((f.start - 9.0).abs() < 1e-4 && (f.end - 16.0).abs() < 1e-4);
        assert!((f.amount - 1.0).abs() < 1e-6);
        assert!((cam.far() - 18.0).abs() < 1e-4);
        assert_eq!(view::fog_amount(16.0), 1.0);
        assert_eq!(view::fog_amount(9.0), 0.0);
    }

    // CAMV-007
    #[test]
    fn camv_007_first_person_walks_relative_to_the_view() {
        let mut cam = zoo_cam();
        let look = 1.2f32;
        cam.set_view(ViewMode::FirstPerson, look);
        cam.snap(Vec3::ZERO);
        let d = view::yaw_to_level(look);
        assert!(cam.stick_to_level(Vec2::Y).abs_diff_eq(d, 1e-5));
        // right = 90° clockwise of D seen from above: (x, z) → (z, −x)
        assert!(cam
            .stick_to_level(Vec2::X)
            .abs_diff_eq(Vec2::new(d.y, -d.x), 1e-5));
        assert!(cam.look_level().abs_diff_eq(d, 1e-5));
    }

    // CAMV-009: everything the look-around camera sees (≥ 3 m away) is at least as far from
    // its eye as from the player's feet (horizontally) — the player's position bounds CAMV-008.
    #[test]
    fn camv_009_look_around_sees_nothing_nearer_than_the_player_does() {
        for aspect in [PORTRAIT, LANDSCAPE, 21.0 / 9.0] {
            for k in 0..16 {
                let feet = Vec3::new(1.0, 0.0, 2.0);
                let mut cam = zoo_cam();
                cam.set_view(ViewMode::LookAround, 0.0);
                cam.turn(k as f32 * 0.4 - 3.0, 0.0);
                cam.snap(feet);
                let vp = cam.view_proj(aspect);
                let eye = cam.eye();
                let mut inside = 0;
                for ix in -40..=40 {
                    for iz in -40..=40 {
                        for y in [0.5, 1.6, 5.0] {
                            let p = feet + Vec3::new(ix as f32 * 0.5, y, iz as f32 * 0.5);
                            let Some(ndc) = project(vp, p) else { continue };
                            if ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || ndc.z.abs() > 1.0 {
                                continue;
                            }
                            let h_eye = Vec2::new(p.x - eye.x, p.z - eye.z).length();
                            if h_eye < 3.0 {
                                continue;
                            }
                            inside += 1;
                            let h_feet = Vec2::new(p.x - feet.x, p.z - feet.z).length();
                            assert!(h_eye >= h_feet - 1e-4, "{p}: eye {h_eye} < player {h_feet}");
                        }
                    }
                }
                assert!(inside > 100);
            }
        }
    }

    // CAMV-015: no head bob, eased turns.
    #[test]
    fn camv_015_no_head_bob_and_eased_turns() {
        let mut cam = zoo_cam();
        cam.set_view(ViewMode::FirstPerson, 0.0);
        cam.snap(Vec3::ZERO);
        cam.turn(1.5, 0.0);
        let mut last = cam.look_yaw();
        for i in 0..120 {
            let feet = Vec3::new(0.0, 0.0, -(i as f32) * 0.03);
            cam.update(DT, feet);
            assert!((cam.eye().y - FP_EYE_HEIGHT_M).abs() < 1e-3);
            let step = (cam.look_yaw() - last).abs();
            // eased: at most rate × dt of the remaining turn per frame
            assert!(step <= 1.5 * (1.0 - (-TURN_EASE_RATE * DT).exp()) + 1e-4);
            last = cam.look_yaw();
        }
    }
}
