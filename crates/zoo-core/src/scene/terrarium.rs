//! The real models of the terrarium house of `night_2` (AENV-018, `tools/blender/props/
//! README_terrarium.md`): the house shell, the glass fronts, frames and lamps and the case
//! dressing (sand and stone, water and waterfall, branches and a tree). Pure data: the host
//! draws and animates it. The placeholder boxes of `round_house.rs` are the fallback of the
//! shell only; every prop here is non-solid decoration (the animals' wander cells, the gate
//! and the stand cells of the hall stay as the grid has them).

use super::*;

/// Top of the case floors (m): every prop stands on it (`terrarium_floor_*` models).
pub const CASE_FLOOR_TOP_M: f32 = 0.08;
/// Mount height of the case lamps (m, origin = lowest point of the wall piece).
pub const CASE_LAMP_Y_M: f32 = 1.95;
/// The cases sit this far (m) off the glass line towards the back (the thin outer wall).
pub const CASE_WALL_M: f32 = 0.3;
/// `socket_emblem` of `terrarium_house` in house space: front of the cream plaque (x, h, z).
pub const HOUSE_EMBLEM: Vec3 = Vec3::new(0.0, 4.0, 0.63);
/// Diameter of the round emblem plaque (m); the snake pictogram decal sits on it.
pub const EMBLEM_PLAQUE_M: f32 = 1.9;
/// Mount height of the wall info boards (m), their model scale and `socket_pictogram` /
/// `socket_lamp` in the model's own frame (x, up, out).
pub const WALL_BOARD_MOUNT_M: f32 = 1.0;
pub const WALL_BOARD_SCALE: f32 = 1.6;
pub const WALL_BOARD_PICTOGRAM: Vec3 = Vec3::new(0.0, 0.995, 0.125);
/// Radius of the cream pictogram plate of `info_board_wall` (m, unscaled).
pub const WALL_BOARD_PLATE_R: f32 = 0.19;

/// A prop of a case in its viewer frame: model, `right` and `into` the case (m, from the
/// floor centre), height and the extra yaw (degrees, added to the case yaw).
type Piece = (&'static str, f32, f32, f32, f32);

const F: f32 = CASE_FLOOR_TOP_M;

const SNAKE: &[Piece] = &[
    ("terrarium_floor_snake", 0.0, 0.0, 0.0, 0.0),
    ("terrarium_rock_warm", 1.0, 1.4, F, 20.0),
    ("terrarium_rock_flat", 0.7, -0.2, F, 10.0),
    ("terrarium_branch_low", 0.1, 0.55, F, -8.0),
    ("terrarium_dish", 1.4, -1.5, F, 0.0),
    ("terrarium_lamp", 0.6, 2.35, CASE_LAMP_Y_M, 0.0),
];
const CHAMELEON: &[Piece] = &[
    ("terrarium_floor_chameleon", 0.0, 0.0, 0.0, 0.0),
    ("terrarium_branch", -1.8, 0.4, F, 0.0),
    ("terrarium_branch", 0.4, 0.9, F, 180.0),
    ("terrarium_tree", 2.1, 0.3, F, 0.0),
    ("terrarium_leaf_big", -2.5, -0.4, F, 30.0),
    ("terrarium_leaf_big", 2.6, -0.6, F, 200.0),
    ("terrarium_lamp_uv", 0.0, 1.85, CASE_LAMP_Y_M, 0.0),
];
const FROG: &[Piece] = &[
    ("terrarium_floor_frog", 0.0, 0.0, 0.0, 0.0),
    ("terrarium_waterfall", 0.6, 1.7, F, 0.0),
    ("mist_puff", 0.6, 0.95, F + 0.1, 0.0),
    ("terrarium_leaf_big", -1.4, 1.6, F, 0.0),
    ("terrarium_leaf_big", 1.3, -1.2, F, 90.0),
    ("terrarium_moss_log", -1.0, -0.9, F, -12.0),
    ("terrarium_fern", -0.3, 1.9, F, 0.0),
    ("terrarium_lamp_teal", -0.8, 2.35, CASE_LAMP_Y_M, 0.0),
];

/// What one case gets: its props, the `right` offsets of the 1 m glass fronts (the 2 m gate
/// stays free), how far in front of the floor centre the glass line is (`into`, negative)
/// and the frame model.
struct CaseKit {
    animal: &'static str,
    pieces: &'static [Piece],
    fronts: &'static [f32],
    glass_into: f32,
    frame: &'static str,
}

const KITS: [CaseKit; 3] = [
    CaseKit {
        animal: "snake",
        pieces: SNAKE,
        fronts: &[-1.5, 1.5],
        glass_into: -2.40,
        frame: "terrarium_frame",
    },
    CaseKit {
        animal: "chameleon",
        pieces: CHAMELEON,
        fronts: &[-3.0, -2.0, 1.0, 2.0, 3.0],
        glass_into: -1.85,
        frame: "terrarium_frame_wide",
    },
    CaseKit {
        animal: "poison_dart_frog",
        pieces: FROG,
        fronts: &[-1.5, 1.5],
        glass_into: -2.40,
        frame: "terrarium_frame",
    },
];

/// Frame of a case: the floor centre, the direction the glass faces (towards the hall) and
/// the yaw (degrees) that turns case-frame models (front = +Z = the glass side).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaseFrame {
    pub floor: Vec2,
    pub out: Vec2,
    pub yaw_deg: f32,
}

impl CaseFrame {
    /// Level position of a case-frame point: `right` of the viewer looking into the case,
    /// `into` the case (m from the floor centre).
    pub fn point(&self, right: f32, into: f32) -> Vec2 {
        let inward = -self.out;
        let r = Vec2::new(inward.y, -inward.x);
        self.floor + r * right + inward * into
    }
}

/// The frame of an indoor case: its glass side is the side its gate is on (the hall side).
pub fn case_frame(case: &Element) -> Option<CaseFrame> {
    let (g, r) = (case.gate?, case.rect);
    let out = if g.x + g.w == r.x + r.w {
        Vec2::X
    } else if g.x == r.x {
        Vec2::NEG_X
    } else if g.z == r.z {
        Vec2::NEG_Y
    } else {
        Vec2::Y
    };
    let centre = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0);
    // the floor runs from the glass line to the thin back wall: its centre is half a wall
    // thickness off the rect centre towards the glass
    let floor = centre + out * (CASE_WALL_M / 2.0);
    Some(CaseFrame {
        floor,
        out,
        yaw_deg: out.x.atan2(-out.y).to_degrees(),
    })
}

impl LevelScene {
    /// Places the case dressing of every indoor case of the round house `e` (floors, props,
    /// glass fronts, frames, lamps). Everything but the glass fronts is merged into one
    /// static mesh by the host (`bake_into`).
    pub(super) fn terrarium_cases(&mut self, e: &Element, data: &LevelData) {
        let part = e.part as u8;
        let model = e.model_rect.unwrap_or(e.rect);
        let cases: Vec<&Element> = data
            .elements_of(ElementType::Enclosure)
            .filter(|c| c.indoor && model.contains(IVec2::new(c.rect.x, c.rect.z)))
            .collect();
        for case in cases {
            let Some(kit) = KITS
                .iter()
                .find(|k| Some(k.animal) == case.animal.as_deref())
            else {
                continue;
            };
            let Some(frame) = case_frame(case) else {
                continue;
            };
            // the floor top walks like the tile under it; the animals stand on it
            self.ground_patches.push(crate::ground::GroundPatch {
                min: Vec2::new(case.rect.x as f32, case.rect.z as f32),
                max: Vec2::new(
                    (case.rect.x + case.rect.w) as f32,
                    (case.rect.z + case.rect.d) as f32,
                ),
                top: CASE_FLOOR_TOP_M,
            });
            for &(m, right, into, y, own) in kit.pieces {
                let k = self.model_at_y(
                    m,
                    frame.point(right, into),
                    y,
                    (frame.yaw_deg + own).to_radians(),
                );
                self.placements[k].part = part;
                // the moving pieces (breathing waterfall sheet, rising mist) stay apart
                if !matches!(m, "mist_puff" | "terrarium_waterfall") {
                    self.bake_into("terrarium_cases", part, k..k + 1);
                }
            }
            for &right in kit.fronts {
                let k = self.model_at_y(
                    "terrarium_front",
                    frame.point(right, kit.glass_into),
                    0.0,
                    frame.yaw_deg.to_radians(),
                );
                self.placements[k].part = part;
            }
            let k = self.model_at_y(
                kit.frame,
                frame.point(0.0, kit.glass_into),
                0.0,
                frame.yaw_deg.to_radians(),
            );
            self.placements[k].part = part;
            self.bake_into("terrarium_cases", part, k..k + 1);
        }
    }

    /// The house emblem: the animal pictogram on the cream plaque over the door (the plaque
    /// is part of the house model).
    pub(super) fn terrarium_emblem(&mut self, id: &str, house: Vec2, animal: &str) {
        let face = house + Vec2::new(HOUSE_EMBLEM.x, -HOUSE_EMBLEM.z);
        self.pictogram_decal(
            format!("{id}:emblem"),
            animal,
            face,
            Vec2::new(0.0, -1.0),
            HOUSE_EMBLEM.y,
            EMBLEM_PLAQUE_M * 0.26,
        );
    }
}
