//! Ad billboards (GAME-ADS): level data of the boards, the seeded campaign-slot assignment and
//! the "standing in front of a board" test. What a board shows (local placeholder or a verified
//! external campaign, ADS-007…) is decided by the host (`web/src/ads.ts`); the game only knows
//! the 3 slots. Pure logic, no web dependencies.

use glam::Vec2;
use serde::Deserialize;

use crate::level::facing_vec;
use crate::rng::Pcg32;

/// Campaign slots (GAME-ADS rule 2): at most 3 campaigns at the same time.
pub const SLOTS: usize = 3;
/// Every campaign slot shows on at least this many boards of the zoo (ADS-002).
pub const MIN_BOARDS_PER_SLOT: usize = 2;
/// Picture texture of a board in pixels (2 : 1, filled by the host).
pub const TEXTURE_PX: (u32, u32) = (1024, 512);
/// Picture size (m) and frame of a board; the board hangs on two posts.
pub const PICTURE_M: Vec2 = Vec2::new(2.4, 1.2);
pub const FRAME_M: f32 = 0.1;
pub const DEPTH_M: f32 = 0.12;
/// Height of the lower frame edge above the ground (m).
pub const BOTTOM_M: f32 = 1.0;
/// Footprint (across × along the facing, m) of the whole board, solid (GAME-PLAYER 9).
pub const FOOTPRINT_M: Vec2 = Vec2::new(2.6, 0.3);
/// The reading range in front of the board (m from its centre, along the facing).
pub const READ_RANGE_M: f32 = 3.5;
/// Closest the player can stand (the footprint ends 0.15 m in front of the centre).
pub const READ_MIN_M: f32 = 0.3;
/// How far beside the board's centre the player may stand (m).
pub const READ_SIDE_M: f32 = 3.0;

/// Poster variant (GAME-ADS rule 1a): a framed picture flat on a wall face. Picture size (m),
/// frame width, depth and the height of the lower frame edge above the ground.
pub const POSTER_PICTURE_M: Vec2 = Vec2::new(2.0, 1.0);
pub const POSTER_FRAME_M: f32 = 0.06;
pub const POSTER_DEPTH_M: f32 = 0.05;
pub const POSTER_BOTTOM_M: f32 = 0.9;

/// Flyer variant (GAME-ADS rule 1a): a paper flyer lying flat on the ground. Picture size (m),
/// paper thickness and the height of its top face above the ground surface (m).
pub const FLYER_PICTURE_M: Vec2 = Vec2::new(1.2, 0.6);
pub const FLYER_PAPER_M: f32 = 0.025;
pub const FLYER_BASE_M: f32 = 0.045;
/// A flyer is read from any side within this distance (m); it is not solid.
pub const FLYER_READ_M: f32 = 2.5;

/// Look of an ad board (`variant` in the level data, GAME-ADS rule 1a).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdVariant {
    /// Free-standing sign on two posts (default, day levels).
    #[default]
    Post,
    /// Framed poster mounted flat on a wall / hedge face (night levels).
    Poster,
    /// Paper flyer lying on the ground (not solid, read from any side; at most one per night
    /// level).
    Flyer,
}

/// One ad board (`[[ad_board]]` in a level file, GAME-ADS rule 1).
#[derive(Debug, Clone, Deserialize)]
pub struct AdBoardData {
    pub id: String,
    /// Centre of the board on the ground (level metres).
    pub pos: [f32; 2],
    /// The side the picture faces and is read from (`+x`, `-x`, `+z`, `-z`).
    #[serde(default = "south")]
    pub facing: String,
    /// `post` (default) or `poster` (on a wall face; `pos` is then on the wall plane).
    #[serde(default)]
    pub variant: AdVariant,
    #[serde(skip)]
    pub part: usize,
}

fn south() -> String {
    "-z".to_owned()
}

impl AdBoardData {
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.pos)
    }

    /// Unit vector the picture faces.
    pub fn facing(&self) -> Vec2 {
        facing_vec(&self.facing)
    }

    /// The grid cells under the footprint (for layout tests and the level designer). A poster
    /// hangs on a wall and has no footprint of its own (only its thin plate).
    pub fn footprint_half(&self) -> Vec2 {
        let f = self.facing();
        if self.variant == AdVariant::Flyer {
            return FLYER_PICTURE_M / 2.0;
        }
        if self.variant == AdVariant::Poster {
            let w = POSTER_PICTURE_M.x + 2.0 * POSTER_FRAME_M;
            return if f.x.abs() > 0.5 {
                Vec2::new(POSTER_DEPTH_M, w) / 2.0
            } else {
                Vec2::new(w, POSTER_DEPTH_M) / 2.0
            };
        }
        if f.x.abs() > 0.5 {
            Vec2::new(FOOTPRINT_M.y, FOOTPRINT_M.x) / 2.0
        } else {
            FOOTPRINT_M / 2.0
        }
    }
}

/// The campaign slot (0-based, `0..SLOTS`) of every board (same order as `ids`) for a session
/// seed (ADS-005): the boards are shuffled with the seed and dealt round-robin, so every slot
/// gets `n / 3` or `n / 3 + 1` boards — at least [`MIN_BOARDS_PER_SLOT`] from 6 boards on
/// (ADS-002). Same seed, same assignment.
pub fn assign_slots(ids: &[&str], seed: u64) -> Vec<u8> {
    let n = ids.len();
    let mut order: Vec<usize> = (0..n).collect();
    // the ids (not their position) seed the shuffle: re-ordering the level data changes nothing
    let mut h = seed ^ 0x9e37_79b9_7f4a_7c15;
    for id in ids {
        for b in id.bytes() {
            h = (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    let mut rng = Pcg32::new(h);
    for i in (1..n).rev() {
        let j = rng.below(i as u32 + 1) as usize;
        order.swap(i, j);
    }
    let mut out = vec![0u8; n];
    for (k, &board) in order.iter().enumerate() {
        out[board] = (k % SLOTS) as u8;
    }
    out
}

/// Like [`assign_slots`], but dealt **per level** (`parts[i]` = level index of board `i`): every
/// level gets all slots when it has ≥ 3 boards, a 3-board level exactly one each (ADS-002).
pub fn assign_slots_by_level(ids: &[&str], parts: &[usize], seed: u64) -> Vec<u8> {
    let mut out = vec![0u8; ids.len()];
    let mut levels: Vec<usize> = parts.to_vec();
    levels.sort_unstable();
    levels.dedup();
    for level in levels {
        let idx: Vec<usize> = (0..ids.len()).filter(|&i| parts[i] == level).collect();
        let sub: Vec<&str> = idx.iter().map(|&i| ids[i]).collect();
        for (&i, slot) in idx.iter().zip(assign_slots(&sub, seed)) {
            out[i] = slot;
        }
    }
    out
}

/// Index of the board the player at `p` stands in front of (inside the reading range, in front
/// of the picture), the nearest one; `None` when there is none (ADS-007).
pub fn near_board(boards: &[AdBoardData], p: Vec2) -> Option<usize> {
    let mut best: Option<(usize, f32)> = None;
    for (i, b) in boards.iter().enumerate() {
        let f = b.facing();
        let d = p - b.pos();
        if b.variant == AdVariant::Flyer {
            let dist = d.length();
            if dist <= FLYER_READ_M && best.is_none_or(|(_, bd)| dist < bd) {
                best = Some((i, dist));
            }
            continue;
        }
        let front = d.dot(f);
        let side = d.dot(Vec2::new(-f.y, f.x)).abs();
        let dist = d.length();
        let near = front >= READ_MIN_M && side <= READ_SIDE_M && dist <= READ_RANGE_M;
        if near && best.is_none_or(|(_, bd)| dist < bd) {
            best = Some((i, dist));
        }
    }
    best.map(|(i, _)| i)
}

/// Texture id the host fills for a board (decal `ad:<board id>`).
pub fn texture_id(board: &str) -> String {
    format!("ad:{board}")
}
