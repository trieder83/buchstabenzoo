//! Coordinate spaces (GAME-LAYOUT "Coordinate spaces", Q-056).
//!
//! - **Level coordinates** `Vec2(x, z)`: x = east, z = north, metres. Used by the layout data,
//!   the ASCII maps (north up) and all game logic in `zoo-core`.
//! - **World space** `Vec3(X, Y, Z)`: right-handed, Y-up (glTF). Level north = world −Z,
//!   level east = world +X.
//!
//! This module is the **only** place where the axis flip happens: `world = (x, 0, −z)`.
//! Models are never mirrored; they are oriented by rotations about +Y only
//! (positive angle = counter-clockwise seen from above, north → west).

use glam::{Vec2, Vec3};

/// Level east as a world direction.
pub const WORLD_EAST: Vec3 = Vec3::X;
/// Level north as a world direction.
pub const WORLD_NORTH: Vec3 = Vec3::NEG_Z;
/// World up.
pub const WORLD_UP: Vec3 = Vec3::Y;

/// Level point or direction `(x, z)` on the ground → world space `(x, 0, −z)`.
pub fn level_to_world(p: Vec2) -> Vec3 {
    Vec3::new(p.x, 0.0, -p.y)
}

/// Level point `(x, z)` at `height` metres above the ground → world `(x, height, −z)`.
pub fn level_to_world_at(p: Vec2, height: f32) -> Vec3 {
    Vec3::new(p.x, height, -p.y)
}

/// World point or direction → level `(x, z)` (the height is dropped).
pub fn world_to_level(w: Vec3) -> Vec2 {
    Vec2::new(w.x, -w.z)
}

/// Rotation angle about world +Y (radians) for `k` clockwise quarter turns seen from above
/// (north → east → south → west), as used for tile and piece rotations.
pub fn quarter_turns_cw_to_yaw(k: i32) -> f32 {
    -(k.rem_euclid(4) as f32) * std::f32::consts::FRAC_PI_2
}

#[cfg(test)]
mod tests {
    use super::*;

    // LAYOUT-007
    #[test]
    fn layout_007_flip_is_single_and_exact() {
        assert_eq!(
            level_to_world(Vec2::new(3.0, 5.0)),
            Vec3::new(3.0, 0.0, -5.0)
        );
        assert_eq!(
            world_to_level(Vec3::new(3.0, 7.0, -5.0)),
            Vec2::new(3.0, 5.0)
        );
        assert_eq!(level_to_world(Vec2::X), WORLD_EAST);
        assert_eq!(level_to_world(Vec2::Y), WORLD_NORTH);
    }
}
