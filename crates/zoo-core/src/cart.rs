//! Golf carts (GAME-CART): the three drivable carts of the zoo — kinematics, collision of the
//! oriented 1.3 x 2.3 m box, boarding, getting out (free exit cell, parking check) and the
//! state saved with the game. Pure and deterministic.
//!
//! Heading: `dir` is a unit vector in level coordinates; `yaw` (saved, presented) uses the
//! camera convention of [`crate::view::level_to_yaw`] (0 = north, counter-clockwise positive).

use glam::{IVec2, Vec2};

use crate::animals::AnimalState;
use crate::collision::{Shape, PLAYER_RADIUS_M};
use crate::game::{Game, GameEvent, Target};
use crate::level::{cell_center, cell_of, CartData, CellKind, Surface};
use crate::view::{level_to_yaw, yaw_to_level};

/// Half width / half length of the driving box (m): 1.3 x 2.3 m.
pub const HALF_WIDTH_M: f32 = 0.65;
pub const HALF_LENGTH_M: f32 = 1.15;
/// Speed on paths / grass (m/s).
pub const PATH_SPEED_MS: f32 = 4.5;
pub const GRASS_SPEED_MS: f32 = 2.0;
/// Steering rate (degrees per second), acceleration and braking (m/s²).
pub const TURN_RATE_DEG_S: f32 = 150.0;
pub const ACCEL_MS2: f32 = 6.0;
pub const BRAKE_MS2: f32 = 10.0;
/// Heading error (degrees) below which the cart drives; at or above it the cart brakes and
/// turns on the spot (it never reverses).
pub const DRIVE_ERROR_DEG: f32 = 30.0;
/// The player gets in within this distance of the box (m).
pub const BOARD_RANGE_M: f32 = 1.5;
/// The cart stops at least this far (m) before an animal, visitor or duck.
pub const STOP_MARGIN_M: f32 = 0.5;
/// Deceleration assumed by the look-ahead (below [`BRAKE_MS2`], so it always suffices).
const LOOK_BRAKE_MS2: f32 = 8.0;
/// Animals, ducks: collision radius (m) for the stop rule.
const ANIMAL_RADIUS_M: f32 = 0.45;
/// Penetration smaller than this is ignored (m).
const EPS: f32 = 1e-3;
/// Largest push-out per step (m).
const MAX_PUSH_M: f32 = 0.5;
/// Seconds of held stick without any progress after which the cart counts as wedged (get-out
/// then ignores the parking check, a last resort puts the cart home).
pub const WEDGE_S: f32 = 2.5;
/// Seconds without progress after which a backwards stick backs the cart out, and its speed.
pub const REVERSE_AFTER_S: f32 = 1.0;
pub const REVERSE_SPEED_MS: f32 = 1.5;
/// Seconds without progress after which the cart is lifted to a roomy pose.
pub const RESCUE_S: f32 = 3.0;
/// A tap on the horn within this long after the last horn is ignored (ASND-033).
pub const HORN_COOLDOWN_S: f32 = 0.6;
/// A bump needs at least this much speed lost to a collision in one step (m/s).
pub const BUMP_MIN_LOST_MS: f32 = 1.0;
/// Speed loss (m/s) that gives the full bump strength.
pub const BUMP_FULL_LOST_MS: f32 = 4.5;
/// At most one bump sound per this long (s).
pub const BUMP_COOLDOWN_S: f32 = 0.8;

/// Strength 0.3..=1 of a bump that lost `lost` m/s, or `None` when it was too soft (ASND-033).
pub fn bump_strength(lost: f32) -> Option<f32> {
    (lost >= BUMP_MIN_LOST_MS).then(|| (lost / BUMP_FULL_LOST_MS).clamp(0.3, 1.0))
}
/// Pose of the child's feet above the seat is the `socket_driver` empty itself (no offset).
pub const ENTER_RANGE_PLAYER_M: f32 = BOARD_RANGE_M;

/// Model yaw (about world +Y) of a level heading: the model's front is world +Z.
pub fn yaw_of_dir(dir: Vec2) -> f32 {
    dir.x.atan2(-dir.y)
}

/// The driving box of a cart at `pos` heading `dir`.
pub fn box_shape(pos: Vec2, dir: Vec2) -> Shape {
    Shape::Box {
        c: pos,
        u: Vec2::new(-dir.y, dir.x),
        half: Vec2::new(HALF_WIDTH_M, HALF_LENGTH_M),
    }
}

/// Why a cart is locked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartLock {
    /// No cart key yet (`cart-locked`).
    NoKey,
    /// Its level is not open yet (`cart-closed-level`).
    ClosedLevel,
}

/// Result of [`Game::leave_cart`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaveResult {
    Left,
    /// The parked box would block something: she stays seated (🅿️✖).
    NoPark,
    NotSeated,
}

/// A golf cart.
#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub id: String,
    pub model: String,
    pub part: usize,
    pub pos: Vec2,
    pub dir: Vec2,
    /// Height of the ground under it (m).
    pub y: f32,
    pub speed: f32,
    /// Accumulated wheel angle (rad, wrapped) and steering in -1..1 (+ = left turn).
    pub wheel_angle: f32,
    pub steer: f32,
    pub home_pos: Vec2,
    pub home_dir: Vec2,
    pub stand: IVec2,
    pub locked_until: String,
    /// Seconds the held stick made no progress.
    pub wedged_s: f32,
    /// Backing out of a wedge (the only reversing).
    pub reversing: bool,
}

impl Cart {
    pub fn from_data(d: &CartData) -> Self {
        Self {
            id: d.id.clone(),
            model: d.model.clone(),
            part: d.part,
            pos: d.pos(),
            dir: d.heading(),
            y: 0.0,
            speed: 0.0,
            wheel_angle: 0.0,
            steer: 0.0,
            home_pos: d.pos(),
            home_dir: d.heading(),
            stand: d.stand_cell(),
            locked_until: d.locked_until.clone(),
            wedged_s: 0.0,
            reversing: false,
        }
    }

    pub fn shape(&self) -> Shape {
        box_shape(self.pos, self.dir)
    }

    /// Saved / presented yaw (camera convention).
    pub fn yaw(&self) -> f32 {
        level_to_yaw(self.dir)
    }

    /// Whether the held stick has made no progress for [`WEDGE_S`].
    pub fn wedged(&self) -> bool {
        self.wedged_s >= WEDGE_S
    }
}

/// Translation to apply to a box (`c`, `u`, `half`) so that it no longer overlaps `other`
/// (zero when they overlap by less than [`EPS`]).
fn push_box_from(c: Vec2, u: Vec2, half: Vec2, other: &Shape) -> Vec2 {
    let v = Vec2::new(-u.y, u.x);
    match *other {
        Shape::Circle { c: cc, r } => {
            let d = cc - c;
            let q = Vec2::new(d.dot(u), d.dot(v));
            let closest = q.clamp(-half, half);
            let off = q - closest;
            let dist = off.length();
            if dist > 1e-6 {
                let pen = r - dist;
                if pen <= EPS {
                    return Vec2::ZERO;
                }
                let n = (u * off.x + v * off.y) / dist;
                return -n * pen;
            }
            let gap = half - q.abs();
            if gap.x < gap.y {
                -u * (gap.x + r) * q.x.signum()
            } else {
                -v * (gap.y + r) * q.y.signum()
            }
        }
        Shape::Box {
            c: c2,
            u: u2,
            half: h2,
        } => {
            let v2 = Vec2::new(-u2.y, u2.x);
            let d = c - c2;
            let mut best = f32::INFINITY;
            let mut push = Vec2::ZERO;
            for axis in [u, v, u2, v2] {
                let r1 = (u.dot(axis) * half.x).abs() + (v.dot(axis) * half.y).abs();
                let r2 = (u2.dot(axis) * h2.x).abs() + (v2.dot(axis) * h2.y).abs();
                let dist = d.dot(axis);
                let pen = r1 + r2 - dist.abs();
                if pen <= EPS {
                    return Vec2::ZERO;
                }
                if pen < best {
                    best = pen;
                    push = axis * if dist >= 0.0 { pen } else { -pen };
                }
            }
            push
        }
    }
}

/// Distance from a point to the surface of a box (0 inside).
pub fn distance_to_box(pos: Vec2, dir: Vec2, p: Vec2) -> f32 {
    let u = Vec2::new(-dir.y, dir.x);
    let d = p - pos;
    let q = Vec2::new(d.dot(u), d.dot(dir));
    let half = Vec2::new(HALF_WIDTH_M, HALF_LENGTH_M);
    (q.abs() - half).max(Vec2::ZERO).length()
}

fn wrap_angle(a: f32) -> f32 {
    let t = std::f32::consts::TAU;
    (a + std::f32::consts::PI).rem_euclid(t) - std::f32::consts::PI
}

fn rotate(v: Vec2, a: f32) -> Vec2 {
    let (s, c) = a.sin_cos();
    Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y)
}

/// Signed angle from `a` to `b` (counter-clockwise positive), radians.
fn angle_between(a: Vec2, b: Vec2) -> f32 {
    wrap_angle(b.y.atan2(b.x) - a.y.atan2(a.x))
}

impl Game {
    // ------------------------------------------------------------------ setup

    /// Builds the carts and the cart-forbidden cell mask from the level data (called by
    /// [`Game::new`]).
    pub(crate) fn init_carts(&mut self) {
        let data = &self.level.data;
        self.carts = data.carts.iter().map(Cart::from_data).collect();
        for c in &mut self.carts {
            c.y = self.level.ground_height(c.pos);
        }
        let grid = self.level.grid();
        let mut mask = vec![false; grid.len()];
        if !self.carts.is_empty() {
            let mut ban = |c: IVec2| {
                if let Some(k) = grid.index(c) {
                    mask[k] = true;
                }
            };
            for e in data.elements.iter().filter(|e| e.is_enterable()) {
                e.interior_cells().into_iter().for_each(&mut ban);
                if let Some(d) = e.door_cell() {
                    ban(d);
                }
            }
            for g in &data.gardens {
                g.rect.cells().for_each(&mut ban);
            }
        }
        self.cart_banned = mask;
        self.sync_cart_shapes();
    }

    /// Puts the boxes of the parked carts into the level's colliders.
    pub fn sync_cart_shapes(&mut self) {
        let shapes: Vec<Shape> = self
            .carts
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != self.seated)
            .map(|(_, c)| c.shape())
            .collect();
        self.level.set_cart_shapes(shapes);
    }

    // ------------------------------------------------------------------ queries

    pub fn cart(&self, id: &str) -> Option<&Cart> {
        self.carts.iter().find(|c| c.id == id)
    }

    /// The cart she sits in.
    pub fn seated_cart(&self) -> Option<&Cart> {
        self.seated.and_then(|i| self.carts.get(i))
    }

    /// Why cart `i` is locked (`None` = usable, GAME-CART rule 2).
    pub fn cart_lock(&self, i: usize) -> Option<CartLock> {
        let c = self.carts.get(i)?;
        if !self.has_cart_key {
            return Some(CartLock::NoKey);
        }
        let open = (c.locked_until.is_empty() || self.level_unlocked(&c.locked_until))
            && self.part_unlocked(c.part);
        (!open).then_some(CartLock::ClosedLevel)
    }

    /// Headlights of cart `i` are on: driven and dusk or night (rule 10).
    pub fn cart_lights_on(&self, i: usize) -> bool {
        self.seated == Some(i) && self.daytime.lamps_on()
    }

    /// Whether a cell is forbidden for the cart (building interior / door, garden) or not
    /// walkable at all (solid, gate, outside).
    pub fn cart_cell_blocked(&self, c: IVec2) -> bool {
        let grid = self.level.grid();
        if !matches!(grid.kind(c), CellKind::Walkable(_)) {
            return true;
        }
        if let Some(part) = self.level.data.part_at(c) {
            if !self.part_unlocked(part) {
                return true;
            }
        }
        grid.index(c)
            .is_some_and(|k| self.cart_banned.get(k).copied().unwrap_or(false))
    }

    /// Distance of the player from cart `i`'s box.
    pub fn cart_distance(&self, i: usize, p: Vec2) -> f32 {
        let c = &self.carts[i];
        distance_to_box(c.pos, c.dir, p)
    }

    /// Cells of the grid blocked for the player circle by a box (centre test as the grid's
    /// `prop_blocked`).
    fn box_blocks_cell(pos: Vec2, dir: Vec2, c: IVec2) -> bool {
        box_shape(pos, dir).overlaps(cell_center(c), PLAYER_RADIUS_M)
    }

    /// Whether a box pose overlaps a solid cell, a prop collider or a parked cart.
    pub fn cart_pose_overlaps(&self, pos: Vec2, dir: Vec2) -> bool {
        self.pose_push(pos, dir) != Vec2::ZERO
    }

    /// Sum-free single push: the largest push vector of any blocker (zero if free).
    fn pose_push(&self, pos: Vec2, dir: Vec2) -> Vec2 {
        let u = Vec2::new(-dir.y, dir.x);
        let half = Vec2::new(HALF_WIDTH_M, HALF_LENGTH_M);
        let shape = box_shape(pos, dir);
        let (lo, hi) = shape.aabb();
        let mut best = Vec2::ZERO;
        let mut consider = |p: Vec2| {
            if p.length_squared() > best.length_squared() {
                best = p;
            }
        };
        let (a, b) = (cell_of(lo), cell_of(hi));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if self.cart_cell_blocked(c) {
                    consider(push_box_from(pos, u, half, &Shape::cell(c)));
                }
            }
        }
        let mut cand = Vec::new();
        let centre = (lo + hi) * 0.5;
        let r = (hi - lo).max_element() * 0.5 + 0.1;
        self.level.colliders().candidates(centre, r, &mut cand);
        for i in cand {
            consider(push_box_from(pos, u, half, self.level.colliders().shape(i)));
        }
        best
    }

    /// Moves a pose out of every blocker (a few passes); `None` when it does not get free
    /// within [`MAX_PUSH_M`].
    fn resolve_pose(&self, pos: Vec2, dir: Vec2) -> Option<Vec2> {
        let mut p = pos;
        for _ in 0..10 {
            let push = self.pose_push(p, dir);
            if push == Vec2::ZERO {
                return Some(p);
            }
            p += push;
            if p.distance(pos) > MAX_PUSH_M {
                return None;
            }
        }
        (self.pose_push(p, dir) == Vec2::ZERO).then_some(p)
    }

    /// Animals, ducks and visitors the cart must keep away from: (centre, radius).
    fn cart_entities(&self, out: &mut Vec<(Vec2, f32)>) {
        out.clear();
        for a in &self.animals {
            if matches!(a.state, AnimalState::InBowl) || !self.part_unlocked(a.part) {
                continue;
            }
            out.push((a.pos, ANIMAL_RADIUS_M));
        }
        out.extend(self.cart_obstacles.iter().copied());
    }

    fn clearance(pos: Vec2, dir: Vec2, ents: &[(Vec2, f32)]) -> f32 {
        ents.iter()
            .map(|&(p, r)| distance_to_box(pos, dir, p) - r)
            .fold(f32::INFINITY, f32::min)
    }

    /// Distance from the box front to the nearest entity ahead in the driving corridor.
    fn gap_ahead(pos: Vec2, dir: Vec2, ents: &[(Vec2, f32)]) -> f32 {
        let u = Vec2::new(-dir.y, dir.x);
        let mut gap = f32::INFINITY;
        for &(p, r) in ents {
            let rel = p - pos;
            let (f, l) = (rel.dot(dir), rel.dot(u).abs());
            if f > HALF_LENGTH_M && l < HALF_WIDTH_M + r + 0.15 {
                gap = gap.min(f - HALF_LENGTH_M - r);
            }
        }
        gap
    }

    // ------------------------------------------------------------------ driving

    /// One driving step of the seated cart with the stick direction `input` (level
    /// coordinates, length <= 1).
    pub(crate) fn drive_cart(&mut self, dt: f32, input: Vec2) {
        let Some(k) = self.seated else { return };
        if dt <= 0.0 {
            return;
        }
        let mut cart = self.carts[k].clone();
        let mut ents = std::mem::take(&mut self.cart_entity_buf);
        self.cart_entities(&mut ents);
        let start = (cart.pos, cart.dir);

        // get free first if something moved into the box (a gate that closed, ...)
        if self.pose_push(cart.pos, cart.dir) != Vec2::ZERO {
            if let Some(p) = self.resolve_pose(cart.pos, cart.dir) {
                cart.pos = p;
            }
        }

        let len = input.length();
        let want = len > 0.05;
        let desired = if want { input / len } else { cart.dir };
        let err = angle_between(cart.dir, desired);

        // steering: towards the stick direction, up to 150°/s
        let mut turned = 0.0;
        if want {
            let max = TURN_RATE_DEG_S.to_radians() * dt;
            let mut turn = err.clamp(-max, max);
            for _ in 0..3 {
                let nd = rotate(cart.dir, turn).normalize();
                // turning never sweeps the box into a duck / visitor; animals are pushed out of
                // the box instead (keep_animals_out_of_carts); a turn that does not make it worse
                // is always allowed (never stuck)
                let obstacles = &ents[ents.len() - self.cart_obstacles.len()..];
                let (clear_old, clear_new) = (
                    Self::clearance(cart.pos, cart.dir, obstacles),
                    Self::clearance(cart.pos, nd, obstacles),
                );
                let free = self
                    .resolve_pose(cart.pos, nd)
                    .filter(|_| clear_new >= clear_old.min(0.0));
                if let Some(p) = free {
                    cart.pos = p;
                    cart.dir = nd;
                    turned = turn;
                    break;
                }
                turn *= 0.5;
            }
        }
        cart.steer += (((turned / dt) / TURN_RATE_DEG_S.to_radians()).clamp(-1.0, 1.0)
            - cart.steer)
            * (1.0 - (-10.0 * dt).exp());

        // speed: drive when the heading is close to the stick direction, else brake and turn
        let surface_max = match self.level.grid().surface(cell_of(cart.pos)) {
            Some(Surface::Path) => PATH_SPEED_MS,
            _ => GRASS_SPEED_MS,
        };
        let err_now = angle_between(cart.dir, desired).abs();
        // wedge escape (never stuck): when the held stick makes no progress for a second and
        // points backwards, the cart backs out slowly — the only case it ever reverses
        if !want || err_now < std::f32::consts::FRAC_PI_2 {
            cart.reversing = false;
        } else if cart.wedged_s >= REVERSE_AFTER_S {
            cart.reversing = true;
        }
        let mut target = if cart.reversing {
            -REVERSE_SPEED_MS
        } else if want && err_now < DRIVE_ERROR_DEG.to_radians() {
            surface_max * len.min(1.0) * err_now.cos()
        } else {
            0.0
        };
        let gap = Self::gap_ahead(cart.pos, cart.dir, &ents);
        if gap.is_finite() && target > 0.0 {
            target = target.min((2.0 * LOOK_BRAKE_MS2 * (gap - STOP_MARGIN_M).max(0.0)).sqrt());
        }
        if cart.speed < target {
            cart.speed = (cart.speed + ACCEL_MS2 * dt).min(target);
        } else {
            cart.speed = (cart.speed - BRAKE_MS2 * dt).max(target);
        }
        if !cart.reversing {
            cart.speed = cart.speed.max(0.0);
        }

        // move in sub-steps; slide along obstacles by pushing the box out
        let total = cart.speed * dt;
        let speed_before = cart.speed;
        let mut braked_by_things = false;
        let steps = (total.abs() / 0.2).ceil().max(1.0) as usize;
        let delta = cart.dir * (total / steps as f32);
        let mut moved = 0.0;
        for _ in 0..steps {
            if total == 0.0 {
                break;
            }
            let cand = cart.pos + delta;
            let Some(q) = self.resolve_pose(cand, cart.dir) else {
                break;
            };
            if q.distance(cart.pos) > delta.length() + EPS {
                break; // no push-through jumps
            }
            let old = Self::clearance(cart.pos, cart.dir, &ents);
            let new = Self::clearance(q, cart.dir, &ents);
            if new < STOP_MARGIN_M && new < old {
                cart.speed = 0.0;
                braked_by_things = true;
                break;
            }
            moved += q.distance(cart.pos) * total.signum();
            cart.pos = q;
        }
        // a bump: speed lost to a wall / fence / building slide (not to ducks, not backing out of
        // a wedge, never from the rescue or the animal push-out; ASND-033)
        if total > 0.0 && !cart.reversing && !braked_by_things && self.bump_cooldown <= 0.0 {
            if let Some(strength) = bump_strength(speed_before - moved / dt) {
                self.bump_cooldown = BUMP_COOLDOWN_S;
                self.events.push(GameEvent::CartBump { strength });
            }
        }
        if total != 0.0 && moved.abs() < total.abs() * 0.25 {
            cart.speed = cart.speed.signum() * cart.speed.abs().min(moved.abs() / dt);
        }
        cart.wheel_angle = wrap_angle(cart.wheel_angle + moved / 0.28);

        // never-stuck bookkeeping
        let progress = cart.pos.distance(start.0) > 0.02 || turned.abs() > 0.01;
        if want && len > 0.5 && !progress {
            cart.wedged_s += dt;
        } else {
            cart.wedged_s = 0.0;
        }

        // wedge rescue (never stuck): nothing moved for RESCUE_S although the stick is held ->
        // lifted to the nearest roomy pose, where it can turn freely
        if cart.wedged_s >= RESCUE_S {
            if let Some(p) = self.roomy_pose_near(cart.pos, cart.dir, &ents) {
                cart.pos = p;
                cart.speed = 0.0;
                cart.reversing = false;
                self.cart_rescues += 1;
            }
            cart.wedged_s = 0.0;
        }
        cart.y = self.player.y;
        self.cart_entity_buf = ents;
        self.carts[k] = cart;
        let c = &self.carts[k];
        self.player.pos = c.pos;
        self.player.facing = c.dir;
        self.player.last_speed = 0.0;
        self.cart_speed_now = c.speed;
    }

    /// Sounds the horn of the seated cart (sound only; a tap within [`HORN_COOLDOWN_S`] of the
    /// last one and any tap on foot are ignored). True when the horn sounded.
    pub fn honk(&mut self) -> bool {
        if self.seated.is_none() || self.horn_cooldown > 0.0 {
            return false;
        }
        self.horn_cooldown = HORN_COOLDOWN_S;
        self.events.push(GameEvent::CartHorn);
        true
    }

    /// Animals never stand inside a cart's box: a pushed-out copy keeps them beside it.
    pub(crate) fn keep_animals_out_of_carts(&mut self) {
        for c in &self.carts {
            let shape = Shape::Box {
                c: c.pos,
                u: Vec2::new(-c.dir.y, c.dir.x),
                half: Vec2::new(HALF_WIDTH_M, HALF_LENGTH_M),
            };
            for a in &mut self.animals {
                if matches!(a.state, AnimalState::InBowl) {
                    continue;
                }
                let push = shape.push_out(a.pos, 0.3);
                if push != Vec2::ZERO {
                    a.pos += push;
                }
            }
        }
    }

    /// The nearest pose (within 8 m) where the box can turn through all headings and no
    /// animal / duck is within 1 m.
    fn roomy_pose_near(&self, pos: Vec2, dir: Vec2, ents: &[(Vec2, f32)]) -> Option<Vec2> {
        let mut best: Option<(f32, Vec2)> = None;
        for dx in -16..=16 {
            for dz in -16..=16 {
                let p = pos + Vec2::new(dx as f32, dz as f32) * 0.5;
                let d = p.distance(pos);
                if d > 8.0 || best.is_some_and(|(b, _)| d >= b) {
                    continue;
                }
                if Self::clearance(p, dir, ents) < 1.0 {
                    continue;
                }
                let roomy = (0..12).all(|k| {
                    let a = k as f32 * std::f32::consts::TAU / 12.0;
                    !self.cart_pose_overlaps(p, rotate(Vec2::X, a))
                });
                if roomy {
                    best = Some((d, p));
                }
            }
        }
        best.map(|(_, p)| p)
    }

    // ------------------------------------------------------------------ boarding

    /// Gets into cart `i` (usable and within reach); animals that follow wait (rule 7).
    pub fn board_cart(&mut self, i: usize) -> bool {
        if self.seated.is_some() || self.cart_lock(i).is_some() {
            return false;
        }
        if self.cart_distance(i, self.player.pos) > BOARD_RANGE_M {
            return false;
        }
        self.seated = Some(i);
        self.panel.open = None;
        self.sync_cart_shapes();
        let c = &mut self.carts[i];
        c.speed = 0.0;
        c.wedged_s = 0.0;
        c.y = self.player.y;
        self.player.pos = c.pos;
        self.player.facing = c.dir;
        let followers = self
            .animals
            .iter()
            .any(|a| a.state == AnimalState::Following);
        self.events.push(GameEvent::CartBoarded {
            id: self.carts[i].id.clone(),
            followers,
        });
        true
    }

    /// Whether the player circle can stand at `p` (cells, props) — gates are closed for her.
    fn circle_free(&self, p: Vec2) -> bool {
        let r = PLAYER_RADIUS_M;
        let grid = self.level.grid();
        let (a, b) = (cell_of(p - Vec2::splat(r)), cell_of(p + Vec2::splat(r)));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if !grid.is_walkable(c, false) && Shape::cell(c).overlaps(p, r) {
                    return false;
                }
            }
        }
        !self.level.colliders().overlaps(p, r)
    }

    fn exit_ok(&self, from: Vec2, p: Vec2, shape: &Shape) -> bool {
        if !self.circle_free(p) || shape.overlaps(p, PLAYER_RADIUS_M + 0.1) {
            return false;
        }
        if self
            .level
            .data
            .part_at(cell_of(p))
            .is_none_or(|k| !self.part_unlocked(k))
        {
            return false;
        }
        if let Some(k) = self.level.grid().index(cell_of(p)) {
            if self.cart_banned.get(k).copied().unwrap_or(false) {
                return false;
            }
        }
        // a short walk from the cart to the point, so she never lands behind a wall
        let n = ((p.distance(from)) / 0.15).ceil().max(1.0) as usize;
        (1..n).all(|s| self.circle_free(from.lerp(p, s as f32 / n as f32)))
    }

    /// Candidate exit points in the search order of rule 3.
    fn exit_candidates(pos: Vec2, dir: Vec2) -> Vec<Vec2> {
        let left = Vec2::new(-dir.y, dir.x);
        let side = HALF_WIDTH_M + PLAYER_RADIUS_M + 0.2;
        let end = HALF_LENGTH_M + PLAYER_RADIUS_M + 0.25;
        let mut v = vec![
            pos + left * side - dir * 0.2,
            pos - left * side - dir * 0.2,
            pos - dir * end,
            pos + dir * end,
        ];
        for ring in [1.7_f32, 2.1, 2.5] {
            for k in 0..16 {
                let a = std::f32::consts::FRAC_PI_2 + k as f32 * std::f32::consts::TAU / 16.0;
                v.push(pos + rotate(dir, a) * ring);
            }
        }
        v
    }

    /// First free exit point for the cart at `pos` / `dir`.
    pub fn exit_point(&self, pos: Vec2, dir: Vec2) -> Option<Vec2> {
        let shape = box_shape(pos, dir);
        Self::exit_candidates(pos, dir)
            .into_iter()
            .find(|&p| self.exit_ok(pos, p, &shape))
    }

    /// The parking check (rule 4): the parked box covers no gate, door, barrier, interaction
    /// point, `stand` cell, food box or hiding place, strands no cell of the walkable grid and
    /// has no animal inside.
    pub fn parking_reason(&self, pos: Vec2, dir: Vec2) -> Option<&'static str> {
        let data = &self.level.data;
        let grid = self.level.grid();
        let shape = box_shape(pos, dir);
        let u = Vec2::new(-dir.y, dir.x);
        let wide = Shape::Box {
            c: pos,
            u,
            half: Vec2::new(HALF_WIDTH_M + 0.5, HALF_LENGTH_M + 0.5),
        };
        let (lo, hi) = wide.aabb();
        let (a, b) = (cell_of(lo), cell_of(hi));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if !wide.overlaps(cell_center(c), 0.5) {
                    continue;
                }
                let barrier = grid
                    .solid_element(c)
                    .and_then(|e| data.elements.get(e))
                    .is_some_and(|e| e.ty == crate::level::ElementType::Barrier);
                let door_or_gate = matches!(grid.kind(c), CellKind::Gate(_))
                    || grid
                        .index(c)
                        .is_some_and(|k| self.cart_banned.get(k).copied().unwrap_or(false));
                if barrier || door_or_gate {
                    return Some(if barrier { "barrier" } else { "door_or_gate" });
                }
                if data
                    .items
                    .iter()
                    .any(|it| it.stand.map(IVec2::from) == Some(c))
                    || data.hiding_places.iter().any(|h| h.rect.contains(c))
                {
                    return Some("stand_or_hiding_place");
                }
            }
        }
        let near = |p: Vec2| wide.overlaps(p, 0.0);
        if self
            .interactables()
            .iter()
            .any(|it| !matches!(it.target, Target::Cart { .. }) && near(it.point))
            || self.food_boxes.iter().any(|(_, p, _)| near(*p))
            || data
                .entries
                .iter()
                .any(|e| e.cells.cells().any(|c| near(cell_center(c))))
        {
            return Some("interaction_point");
        }
        let mut ents = Vec::new();
        self.cart_entities(&mut ents);
        if ents.iter().any(|&(p, r)| shape.overlaps(p, r + 0.3)) {
            return Some("animal");
        }
        (!self.keeps_grid_connected(pos, dir)).then_some("disconnects")
    }

    /// Whether a cart may be parked at this pose (see [`Game::parking_reason`]).
    pub fn parking_ok(&self, pos: Vec2, dir: Vec2) -> bool {
        self.parking_reason(pos, dir).is_none()
    }

    /// Flood fill (4-neighbour) of the passable grid from the level spawns: no cell that was
    /// reachable may become unreachable because of the parked box.
    fn keeps_grid_connected(&self, pos: Vec2, dir: Vec2) -> bool {
        let grid = self.level.grid();
        let n = grid.len();
        let seeds: Vec<IVec2> = self
            .level
            .data
            .parts
            .iter()
            .map(|p| p.spawn.cell())
            .collect();
        let blocked_by_box = |c: IVec2| Self::box_blocks_cell(pos, dir, c);
        let fill = |with_box: bool| -> Vec<bool> {
            let mut seen = vec![false; n];
            let mut stack: Vec<IVec2> = Vec::new();
            let ok = |c: IVec2| {
                grid.is_passable(c, false)
                    && !(with_box && !self.carts.is_empty() && blocked_by_box(c))
            };
            for &s in &seeds {
                if let Some(k) = grid.index(s) {
                    if ok(s) && !seen[k] {
                        seen[k] = true;
                        stack.push(s);
                    }
                }
            }
            while let Some(c) = stack.pop() {
                for d in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
                    let q = c + d;
                    if let Some(k) = grid.index(q) {
                        if !seen[k] && ok(q) && grid.step_open(c, q) {
                            seen[k] = true;
                            stack.push(q);
                        }
                    }
                }
            }
            seen
        };
        // the box's own cart is not a collider here (`pos` is the pose being tested), the
        // other parked carts are part of both fills
        let before = fill(false);
        let after = fill(true);
        (0..n).all(|k| !before[k] || after[k] || blocked_by_box(grid.cell_at(k)))
    }

    /// Gets out (rule 3): on the first free exit cell; the parking check refuses a spot where
    /// the parked cart would be harmful (🅿️✖), unless the cart is wedged.
    pub fn leave_cart(&mut self) -> LeaveResult {
        let Some(k) = self.seated else {
            return LeaveResult::NotSeated;
        };
        let (pos, dir, wedged) = {
            let c = &self.carts[k];
            (c.pos, c.dir, c.wedged())
        };
        // the parking check judges the box without the seated cart's own collider
        if !wedged && !self.parking_ok(pos, dir) {
            self.events.push(GameEvent::CartNoPark);
            return LeaveResult::NoPark;
        }
        let exit = self.exit_point(pos, dir);
        self.seated = None;
        let id = self.carts[k].id.clone();
        match exit {
            Some(p) => {
                self.carts[k].speed = 0.0;
                self.carts[k].steer = 0.0;
                self.carts[k].y = self.level.ground_height(self.carts[k].pos);
                self.player.pos = p;
                self.player.facing = (p - pos).normalize_or(dir);
            }
            None => {
                // last resort (never on valid level data): the cart goes home, she stands on
                // the boarding cell
                self.cart_resets += 1;
                let c = &mut self.carts[k];
                c.pos = c.home_pos;
                c.dir = c.home_dir;
                c.speed = 0.0;
                self.player.pos = cell_center(self.carts[k].stand);
            }
        }
        self.carts[k].wedged_s = 0.0;
        self.player.y = self.level.ground_height(self.player.pos);
        self.cart_speed_now = 0.0;
        self.sync_cart_shapes();
        self.events.push(GameEvent::CartLeft { id });
        LeaveResult::Left
    }

    // ------------------------------------------------------------------ save

    /// Restores the carts and the seat from a save (rule "Save"): a pose that is no longer
    /// free moves to the nearest free pose within 4 m, else the parking pose.
    pub fn restore_carts(&mut self, saved: &[crate::save::CartSave], seated: Option<&str>) {
        self.seated = None;
        for s in saved {
            let Some(i) = self.carts.iter().position(|c| c.id == s.id) else {
                continue;
            };
            let dir = yaw_to_level(if s.yaw.is_finite() { s.yaw } else { 0.0 });
            let pos = Vec2::new(s.x, s.z);
            if !pos.is_finite() {
                continue;
            }
            self.carts[i].dir = dir;
            self.carts[i].pos = pos;
            self.carts[i].speed = 0.0;
        }
        self.level.set_cart_shapes(Vec::new());
        for i in 0..self.carts.len() {
            let (pos, dir) = (self.carts[i].pos, self.carts[i].dir);
            let (np, nd) = self.free_pose_near(pos, dir, i);
            self.carts[i].pos = np;
            self.carts[i].dir = nd;
            self.carts[i].y = self.level.ground_height(np);
        }
        if let Some(id) = seated {
            if let Some(i) = self.carts.iter().position(|c| c.id == id) {
                self.seated = Some(i);
                let c = &mut self.carts[i];
                self.player.pos = c.pos;
                self.player.facing = c.dir;
                self.player.y = c.y;
            }
        }
        self.sync_cart_shapes();
    }

    /// The pose itself, else the nearest free one within 4 m, else the parking pose. Other
    /// carts' boxes count as obstacles (parked ones are colliders already while the shapes
    /// are cleared, so only the walkable test runs here).
    fn free_pose_near(&self, pos: Vec2, dir: Vec2, i: usize) -> (Vec2, Vec2) {
        if !self.cart_pose_overlaps(pos, dir) {
            return (pos, dir);
        }
        let mut best: Option<(f32, Vec2)> = None;
        for dx in -8..=8 {
            for dz in -8..=8 {
                let p = pos + Vec2::new(dx as f32, dz as f32) * 0.5;
                let d = p.distance(pos);
                if d <= 4.0 && !self.cart_pose_overlaps(p, dir) && best.is_none_or(|(b, _)| d < b) {
                    best = Some((d, p));
                }
            }
        }
        match best {
            Some((_, p)) => (p, dir),
            None => (self.carts[i].home_pos, self.carts[i].home_dir),
        }
    }
}
