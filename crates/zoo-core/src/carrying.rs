//! Putting items down, picking them up again, and cutting bamboo in a bamboo forest
//! (GAME-FEED §8–17).
//!
//! "Hands" and "pocket": with the fish bowl carried, the bowl fills both hands and the one
//! food ([`crate::food::Carry`]) is in the pocket (GAME-RESCUE goldfish bowl, Q-084); without
//! the bowl the food is in the hands. Lying foods are kept oldest first; the bowl lies where
//! [`crate::game::Bowl::pos`] says when it was put down (`dropped`).

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::food::{Food, FoodBox};
use crate::game::{Game, GameEvent};
use crate::level::cell_of;

/// A put-down item lies this far in front of the player (GAME-FEED §9).
pub const DROP_AHEAD_M: f32 = 0.8;
/// If the spot in front is not free, the nearest free spot within this distance is used.
pub const DROP_SEARCH_M: f32 = 1.5;
/// A food dropped this close to its own food box goes back into the box (§10).
pub const BACK_TO_BOX_M: f32 = 1.5;
/// At most this many items lie in the zoo (§10, user decision 2026-09-27).
pub const MAX_LYING: usize = 8;
/// Footprint radius of a lying item (for "another item" / props around it).
pub const ITEM_RADIUS_M: f32 = 0.25;
/// Two lying items keep at least this distance (centre to centre).
pub const ITEM_SPACING_M: f32 = 0.5;
/// Food boxes keep this much room around them (they have no collider, LAYOUT-032).
pub const BOX_CLEAR_M: f32 = 0.7;
/// Openings (doors, gates) and their walkways stay free: nothing lies within the opening's
/// half width plus this distance of its centre (LAYOUT-032).
pub const OPENING_CLEAR_M: f32 = 1.0;
/// Cut spots are used from this distance (GAME-FEED §14).
pub const CUT_RANGE_M: f32 = 1.5;
/// Regrowth time of a cut stalk in play time (GAME-FEED §15, Q-156 answered).
pub const REGROW_S: f32 = 180.0;

/// A food lying on the ground (GAME-FEED §10).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LyingFood {
    /// Stable id of this lying item (interaction target), unique per game.
    pub uid: u32,
    pub food: Food,
    /// Where it lies (level) and the surface height there (PLAY-035).
    pub pos: Vec2,
    pub y: f32,
}

/// Foods lying in the zoo, oldest first.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lying {
    pub foods: Vec<LyingFood>,
    pub(crate) next_uid: u32,
}

impl Lying {
    pub fn get(&self, uid: u32) -> Option<&LyingFood> {
        self.foods.iter().find(|f| f.uid == uid)
    }

    pub(crate) fn push(&mut self, food: Food, pos: Vec2, y: f32) -> u32 {
        let uid = self.next_uid;
        self.next_uid += 1;
        self.foods.push(LyingFood { uid, food, pos, y });
        uid
    }

    fn remove(&mut self, uid: u32) -> Option<LyingFood> {
        let i = self.foods.iter().position(|f| f.uid == uid)?;
        Some(self.foods.remove(i))
    }
}

/// What fills the hands (GAME-FEED §8): the bowl (both hands) or a food.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    Bowl,
    Food(Food),
}

/// Why nothing was put down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropError {
    /// Nothing droppable in the hands (FEED-016).
    NothingHeld,
    /// No free walkable spot within [`DROP_SEARCH_M`] (the button shakes, §9).
    NoFreeSpot,
}

/// What a put-down did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dropped {
    /// The food lies at `pos`.
    Food { food: Food, pos: Vec2 },
    /// The food went back into its own box (§10).
    IntoBox { food: Food },
    /// The bowl lies at `pos`.
    Bowl { pos: Vec2 },
}

/// Growth stage of a cut spot (GAME-FEED §15): full grown (cuttable) → cut: stump → young
/// shoot → full again after [`REGROW_S`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StalkStage {
    Stump,
    Young,
    Full,
}

impl StalkStage {
    pub fn id(self) -> &'static str {
        match self {
            StalkStage::Stump => "stump",
            StalkStage::Young => "young",
            StalkStage::Full => "full",
        }
    }

    /// Stage for the regrowth time left: stump for the first half, then the young shoot.
    pub fn of_regrow(left_s: f32) -> Self {
        if left_s <= 0.0 {
            StalkStage::Full
        } else if left_s > REGROW_S / 2.0 {
            StalkStage::Stump
        } else {
            StalkStage::Young
        }
    }
}

/// Regrowth state of the cut spots (same order as `LevelData::cut_spots`); saved.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BambooForests {
    /// Play time left until full grown, per cut spot (0 = full grown).
    pub regrow_s: Vec<f32>,
}

impl BambooForests {
    pub fn new(n: usize) -> Self {
        Self {
            regrow_s: vec![0.0; n],
        }
    }

    pub fn stage(&self, i: usize) -> Option<StalkStage> {
        self.regrow_s.get(i).map(|&t| StalkStage::of_regrow(t))
    }

    /// Advances regrowth by `dt` seconds of play time (paused time is never passed in).
    pub fn update(&mut self, dt: f32) {
        for t in &mut self.regrow_s {
            *t = (*t - dt).max(0.0);
        }
    }
}

impl Game {
    /// What fills the hands (the bowl, else the food), if anything.
    pub fn held(&self) -> Option<Held> {
        if self.bowl.as_ref().is_some_and(|b| b.carried) {
            Some(Held::Bowl)
        } else {
            self.carry.food().map(Held::Food)
        }
    }

    /// Whether something can be put down: the put-down button is shown and `G` works
    /// (GAME-FEED §8, FEED-016). The cart key and the pocket are never dropped.
    pub fn can_put_down(&self) -> bool {
        self.held().is_some()
    }

    /// Number of items lying in the zoo (foods and the put-down bowl).
    pub fn lying_count(&self) -> usize {
        self.lying.foods.len() + usize::from(self.bowl_lying())
    }

    /// Whether the bowl lies somewhere after the child put it down.
    pub fn bowl_lying(&self) -> bool {
        self.bowl.as_ref().is_some_and(|b| !b.carried && b.dropped)
    }

    /// Puts the item in the hands down ≈ 0.8 m in front of the player (GAME-FEED §8–10).
    pub fn put_down(&mut self) -> Result<Dropped, DropError> {
        let at = self.player.pos + self.player.facing.normalize_or(Vec2::NEG_Y) * DROP_AHEAD_M;
        self.put_down_at(at)
    }

    /// Puts the item in the hands down at `nominal` (or the nearest free spot within 1.5 m).
    pub(crate) fn put_down_at(&mut self, nominal: Vec2) -> Result<Dropped, DropError> {
        let held = self.held().ok_or(DropError::NothingHeld)?;
        if let Held::Food(food) = held {
            // next to its own box: back into the box (§10)
            if self.near_own_box(food, nominal) {
                self.carry.consume();
                self.events.push(GameEvent::FoodPutBack { food });
                return Ok(Dropped::IntoBox { food });
            }
        }
        let pos = self.free_spot(nominal).ok_or(DropError::NoFreeSpot)?;
        let y = self.level.ground_height(pos);
        self.make_room();
        match held {
            Held::Food(food) => {
                self.carry.consume();
                self.lying.push(food, pos, y);
                self.events.push(GameEvent::ItemPutDown {
                    id: format!("food:{}", food.id()),
                });
                Ok(Dropped::Food { food, pos })
            }
            Held::Bowl => {
                let group = self.bowl.as_ref().map(|b| b.animal.clone());
                let b = self.bowl.as_mut().expect("held bowl");
                b.carried = false;
                b.dropped = true;
                b.pos = pos;
                b.lift_m = y; // on the surface (GAME-PLAYER 8)
                let (id, fish) = (b.id.clone(), b.fish);
                if fish {
                    // the fish stays in the bowl (FEED-012)
                    for j in group.map(|a| self.group(&a)).unwrap_or_default() {
                        self.animals[j].pos = pos;
                    }
                }
                self.events.push(GameEvent::ItemPutDown { id });
                Ok(Dropped::Bowl { pos })
            }
        }
    }

    /// Keeps at most [`MAX_LYING`] items: before one more is put down, the oldest lying food
    /// goes back into its box (never the bowl, §10).
    fn make_room(&mut self) {
        while self.lying_count() >= MAX_LYING && !self.lying.foods.is_empty() {
            let f = self.lying.foods.remove(0);
            self.events.push(GameEvent::FoodPutBack { food: f.food });
        }
    }

    fn near_own_box(&self, food: Food, p: Vec2) -> bool {
        self.food_boxes
            .iter()
            .any(|&(f, c, _)| f == food && c.distance(p) <= BACK_TO_BOX_M)
    }

    /// Whether an item may lie at `p` (GAME-FEED §9): free walkable ground — not water, a
    /// fence, wall, gate or enclosure (the grid), not in a prop collider, not in an opening
    /// or its walkway, not on another item or at a food box.
    pub fn spot_free(&self, p: Vec2) -> bool {
        let level = &self.level;
        if !level.grid().is_walkable(cell_of(p), false) {
            return false;
        }
        if crate::wander::water_depth(level, p) > 0.0 {
            return false;
        }
        if level.colliders().overlaps(p, ITEM_RADIUS_M) {
            return false;
        }
        if level
            .openings()
            .iter()
            .any(|o| o.center.distance(p) < o.opening_m / 2.0 + OPENING_CLEAR_M)
        {
            return false;
        }
        if self
            .lying
            .foods
            .iter()
            .any(|f| f.pos.distance(p) < ITEM_SPACING_M)
        {
            return false;
        }
        if let Some(b) = self.bowl.as_ref().filter(|b| !b.carried) {
            if b.pos.distance(p) < ITEM_SPACING_M {
                return false;
            }
        }
        !self
            .food_boxes
            .iter()
            .any(|&(_, c, _)| c.distance(p) < BOX_CLEAR_M)
    }

    /// `nominal` if free, else the nearest free spot within [`DROP_SEARCH_M`] (rings of
    /// 0.1 m, 32 directions; deterministic).
    pub fn free_spot(&self, nominal: Vec2) -> Option<Vec2> {
        if self.spot_free(nominal) {
            return Some(nominal);
        }
        let steps = (DROP_SEARCH_M / 0.1).round() as i32;
        for k in 1..=steps {
            let r = k as f32 * 0.1;
            let mut best: Option<Vec2> = None;
            for j in 0..32 {
                let a = j as f32 * std::f32::consts::TAU / 32.0;
                let q = nominal + Vec2::new(a.cos(), a.sin()) * r;
                if self.spot_free(q) {
                    // (all candidates of a ring are equally near: the first one wins)
                    best = Some(q);
                    break;
                }
            }
            if best.is_some() {
                return best;
            }
        }
        None
    }

    /// Picks up a lying food (within 2 m, GAME-FEED §11). Full hands swap: the held food lies
    /// on its spot. With the bowl in the hands the food goes to the pocket (a pocket food
    /// swaps with it).
    pub fn pick_up_food(&mut self, uid: u32) -> bool {
        let Some(f) = self.lying.get(uid).copied() else {
            return false;
        };
        if !crate::player::in_interaction_range(f.pos.distance(self.player.pos)) {
            return false;
        }
        self.lying.remove(uid);
        if let Some(old) = self.carry.take(&FoodBox { food: f.food }) {
            // swap: the held one lies on the same spot, keeping its age order as the newest
            self.lying.push(old, f.pos, f.y);
            self.events.push(GameEvent::ItemPutDown {
                id: format!("food:{}", old.id()),
            });
        }
        self.events.push(GameEvent::ItemTaken {
            id: format!("food:{}", f.food.id()),
        });
        true
    }

    /// Cuts a full-grown bamboo stalk at a cut spot (GAME-FEED §14): the player carries
    /// `bamboo` (in the pocket while she holds the bowl); food already held is put down at
    /// her feet. False if the spot is not full grown or unknown.
    pub fn cut_bamboo(&mut self, spot: &str) -> bool {
        let Some(i) = self.level.data.cut_spots.iter().position(|c| c.id == spot) else {
            return false;
        };
        if self.bamboo.stage(i) != Some(StalkStage::Full) {
            return false;
        }
        if let Some(old) = self.carry.food() {
            // the held (or pocket) food goes down at her feet (§14, FEED-020)
            let at = self.player.pos;
            if self.near_own_box(old, at) {
                self.carry.consume();
                self.events.push(GameEvent::FoodPutBack { food: old });
            } else {
                let Some(pos) = self.free_spot(at) else {
                    return false;
                };
                let y = self.level.ground_height(pos);
                self.make_room();
                self.carry.consume();
                self.lying.push(old, pos, y);
                self.events.push(GameEvent::ItemPutDown {
                    id: format!("food:{}", old.id()),
                });
            }
        }
        self.carry.take(&FoodBox { food: Food::Bamboo });
        self.bamboo.regrow_s[i] = REGROW_S;
        self.events.push(GameEvent::BambooCut {
            spot: spot.to_owned(),
        });
        true
    }
}
