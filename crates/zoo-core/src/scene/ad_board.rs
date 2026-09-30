//! Geometry of an ad board (GAME-ADS): frame on two posts, a solid footprint and the picture
//! decal that the host fills (placeholder text or a verified campaign image).

use super::{colors, BoxPlacement, Decal, DecalImage, LevelScene, DECAL_LIFT_M};
use crate::ads::{texture_id, AdBoardData, BOTTOM_M, DEPTH_M, FRAME_M, PICTURE_M, TEXTURE_PX};
use crate::coords::{level_to_world, level_to_world_at, quarter_turns_cw_to_yaw};
use glam::{Vec2, Vec3};

impl LevelScene {
    /// Board frame + posts as boxes, footprint collider, picture decal `sign:ad:<id>`… the
    /// decal id is `ad:<id>` (also the texture id).
    pub(super) fn ad_board(&mut self, b: &AdBoardData) {
        let out = b.facing();
        let c = b.pos();
        let sideways = out.x.abs() > 0.5; // faces ±x: the board plane runs along z
        let yaw = if sideways {
            quarter_turns_cw_to_yaw(1)
        } else {
            0.0
        };
        let frame = Vec3::new(
            PICTURE_M.x + 2.0 * FRAME_M,
            PICTURE_M.y + 2.0 * FRAME_M,
            DEPTH_M,
        );
        let source = format!("ad_board:{}", b.id);
        let mut push = |pos: Vec3, size: Vec3| {
            self.boxes.push(BoxPlacement {
                pos,
                size,
                yaw,
                color: colors::WOOD,
                fadeable: false,
                source: source.clone(),
                part: b.part as u8,
            });
        };
        push(level_to_world_at(c, BOTTOM_M), frame);
        let tangent = Vec2::new(-out.y, out.x);
        for s in [-1.0_f32, 1.0] {
            let post = c + tangent * (s * (frame.x / 2.0 - 0.18));
            push(
                level_to_world_at(post, 0.0),
                Vec3::new(0.14, BOTTOM_M, 0.14),
            );
        }
        let half = b.footprint_half();
        self.box_colliders.push(crate::collision::Shape::Box {
            c,
            u: Vec2::X,
            half,
        });
        let front = level_to_world_at(
            c + out * (DEPTH_M / 2.0 + DECAL_LIFT_M),
            BOTTOM_M + FRAME_M + PICTURE_M.y / 2.0,
        );
        let n = level_to_world(out).normalize();
        let right = Vec3::Y.cross(n);
        let id = texture_id(&b.id);
        self.decals.push(Decal {
            id: id.clone(),
            image: DecalImage::Ad {
                board: b.id.clone(),
                width_px: TEXTURE_PX.0,
                height_px: TEXTURE_PX.1,
            },
            center: front,
            right: right * (PICTURE_M.x / 2.0),
            up: Vec3::Y * (PICTURE_M.y / 2.0),
        });
    }
}
