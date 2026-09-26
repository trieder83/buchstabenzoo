//! Player movement and interaction range (GAME-PLAYER §3, §5, §6).

use glam::Vec2;

use crate::collision::{Blockers, Colliders, PLAYER_RADIUS_M};
use crate::level::{cell_of, Grid, Surface};

/// Movement tuning (GAME-PLAYER §6, user decisions 2026-09-26: path 1.93 m/s, grass 0.98 m/s).
#[derive(Debug, Clone, Copy)]
pub struct MoveParams {
    /// Walking speed on `path` cells in m/s (1.93).
    pub walk_speed: f32,
    /// Grass speed = walk speed × this factor (≈ 0.51 → 0.98 m/s).
    pub grass_speed_factor: f32,
    /// Time for the speed to blend from one surface speed to the other (PLAY-007: ≤ 0.2 s).
    pub surface_blend_s: f32,
}

impl Default for MoveParams {
    fn default() -> Self {
        Self {
            walk_speed: PATH_SPEED,
            grass_speed_factor: GRASS_SPEED / PATH_SPEED,
            surface_blend_s: 0.15,
        }
    }
}

/// Path speed (GAME-PLAYER §6): 1.75 m/s + 10 % (user decision 2026-09-26).
pub const PATH_SPEED: f32 = 1.93;
/// Grass speed stays 0.98 m/s (GAME-PLAYER §6).
pub const GRASS_SPEED: f32 = 0.98;

impl MoveParams {
    pub fn speed_on(&self, s: Surface) -> f32 {
        match s {
            Surface::Path => self.walk_speed,
            Surface::Grass => self.walk_speed * self.grass_speed_factor,
        }
    }
}

/// Speed the `walk` clip is authored for (ART-RIG §4.7, Q-024), m/s.
pub const WALK_CLIP_AUTHORED_SPEED: f32 = 1.4;
/// Playback rate clamp of locomotion clips (ART-RIG §4.7).
pub const WALK_CLIP_RATE_RANGE: (f32, f32) = (0.8, 1.4);

/// Playback rate of the `walk` clip at a ground speed (GAME-PLAYER §6, ART-RIG §4.7):
/// `speed ÷ 1.4`, clamped to [0.8, 1.4] — ≈ 1.38 on paths (1.93 m/s), 0.8 on grass.
pub fn walk_clip_rate(speed: f32) -> f32 {
    (speed / WALK_CLIP_AUTHORED_SPEED).clamp(WALK_CLIP_RATE_RANGE.0, WALK_CLIP_RATE_RANGE.1)
}

/// Interaction range in metres (GAME-PLAYER §5).
pub const INTERACTION_RANGE_M: f32 = 2.0;

/// Whether something at `distance` metres is within interaction range (PLAY-003).
pub fn in_interaction_range(distance: f32) -> bool {
    distance <= INTERACTION_RANGE_M
}

/// Maximum distance moved per sub-step, so no cell is skipped.
const MAX_SUBSTEP_M: f32 = 0.25;

#[derive(Debug, Clone)]
pub struct Player {
    /// World position `(x, z)` in metres.
    pub pos: Vec2,
    /// Unit facing direction `(x, z)`.
    pub facing: Vec2,
    /// Current ground speed limit in m/s (blends between surfaces, PLAY-007).
    surface_speed: f32,
    /// Speed actually moved in the last update, m/s.
    pub last_speed: f32,
    /// First person (GAME-CAMERA-VIEWS 3): the facing is the view direction and walking
    /// (also sideways) does not turn the player. `None` = facing follows the walk direction.
    pub facing_lock: Option<Vec2>,
}

impl Player {
    pub fn new(pos: Vec2, facing: Vec2, params: &MoveParams) -> Self {
        Self {
            pos,
            facing,
            surface_speed: params.walk_speed,
            last_speed: 0.0,
            facing_lock: None,
        }
    }

    /// Locks the facing to a view direction (first person, CAMV-006) or unlocks it (`None`).
    /// A locked facing is applied at once.
    pub fn lock_facing(&mut self, dir: Option<Vec2>) {
        self.facing_lock = dir.and_then(|d| d.try_normalize());
        if let Some(d) = self.facing_lock {
            self.facing = d;
        }
    }

    /// Current surface speed (m/s) at full joystick deflection.
    pub fn surface_speed(&self) -> f32 {
        self.surface_speed
    }

    /// Restores the blended surface speed (GAME-SAVE).
    pub fn set_surface_speed(&mut self, speed: f32) {
        self.surface_speed = speed;
    }

    /// Moves the player without prop collision (cells only). See [`Player::step_with`].
    pub fn step(
        &mut self,
        grid: &Grid,
        params: &MoveParams,
        input: Vec2,
        dt: f32,
        allow_gates: bool,
    ) {
        self.step_with(grid, &Colliders::default(), params, input, dt, allow_gates);
    }

    /// Moves the player. `input` is the direction in level coordinates `(x, z)` (x east,
    /// z north), length ≤ 1 (joystick deflection). The player is a circle of radius
    /// [`PLAYER_RADIUS_M`] that cannot overlap solid cells (gates only with `allow_gates`)
    /// or prop shapes (GAME-PLAYER §7); blocked moves slide along them. Blockers the circle
    /// already overlaps (e.g. a gate that closed under the player) are ignored.
    pub fn step_with(
        &mut self,
        grid: &Grid,
        colliders: &Colliders,
        params: &MoveParams,
        input: Vec2,
        dt: f32,
        allow_gates: bool,
    ) {
        // Blend the speed limit towards the surface under the player (PLAY-007).
        if let Some(surface) = grid.surface(cell_of(self.pos)) {
            let target = params.speed_on(surface);
            let rate = (params.walk_speed * (1.0 - params.grass_speed_factor)).abs()
                / params.surface_blend_s.max(1e-4);
            let max_delta = rate * dt;
            self.surface_speed += (target - self.surface_speed).clamp(-max_delta, max_delta);
        }

        let input = input.clamp_length_max(1.0);
        if input.length_squared() < 1e-8 || dt <= 0.0 {
            self.last_speed = 0.0;
            return;
        }
        self.facing = self.facing_lock.unwrap_or_else(|| input.normalize());
        let total = input * self.surface_speed * dt;
        let steps = (total.length() / MAX_SUBSTEP_M).ceil().max(1.0) as usize;
        let delta = total / steps as f32;
        let start = self.pos;
        let mut blockers = Blockers::at(grid, colliders, allow_gates, self.pos);
        for _ in 0..steps {
            let r = PLAYER_RADIUS_M;
            let tries = [
                self.pos + delta,
                Vec2::new(self.pos.x + delta.x, self.pos.y),
                Vec2::new(self.pos.x, self.pos.y + delta.y),
            ];
            let mut moved = false;
            for t in tries {
                if let Some(q) = blockers.resolve(t, r) {
                    // never move further than the requested step (no push-through jumps)
                    if q.distance(self.pos) <= delta.length() + 1e-4 {
                        self.pos = q;
                        moved = true;
                        break;
                    }
                }
            }
            if !moved {
                break;
            }
        }
        self.last_speed = self.pos.distance(start) / dt;
    }
}
