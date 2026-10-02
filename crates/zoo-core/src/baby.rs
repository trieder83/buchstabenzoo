//! Playful following of the baby animal (GAME-FAMILY §5, FAM-014..018).
//!
//! The baby keeps a loose leash around its mother instead of walking a straight line behind
//! her: it roams in little loops around her, trots ahead and drops back, sniffs for a moment,
//! runs short bursts with a small hop, and circles her while she stands. Farther than
//! [`LEASH_SOFT_M`] it trots back, farther than [`LEASH_HARD_M`] it runs. Deterministic: all
//! randomness comes from the `Pcg32` handed in; only picking a target allocates a route.

use glam::Vec2;

use crate::game::Baby;
use crate::level::{cell_of, Grid, Surface};
use crate::nav;
use crate::rng::Pcg32;
use crate::wander::{self, WanderArea};

/// Beyond this distance to the mother the baby trots back (catch-up).
pub const LEASH_SOFT_M: f32 = 5.0;
/// Beyond this distance it runs back at burst speed.
pub const LEASH_HARD_M: f32 = 8.0;
/// A returning baby is back "in play range" within this distance.
pub const RETURN_DONE_M: f32 = 4.0;
/// Safety net: farther than this the baby is put beside its mother (game.rs).
pub const TELEPORT_M: f32 = 30.0;
/// Ring radius around the standing mother (min, max), metres.
pub const RING_M: (f32, f32) = (1.8, 4.0);
/// Angle the baby advances around a standing mother per leg (min, max), degrees.
pub const ORBIT_STEP_DEG: (f32, f32) = (40.0, 140.0);
/// Offset along the heading of a walking mother (min, max), metres (negative = behind).
pub const AHEAD_M: (f32, f32) = (-3.5, 3.5);
/// Sideways offset magnitude (min, max) from a walking mother, metres.
pub const LATERAL_M: (f32, f32) = (1.0, 2.5);
/// Speed factors, multiples of the follow speed on the surface under the baby.
pub const AMBLE_FACTOR: f32 = 0.7;
pub const TROT_FACTOR: f32 = 1.25;
pub const CATCHUP_FACTOR: f32 = 1.6;
pub const BURST_FACTOR: f32 = 2.2;
/// Leg length (s) while the mother stands / walks (min, max).
pub const LEG_STAND_S: (f32, f32) = (3.0, 5.0);
pub const LEG_WALK_S: (f32, f32) = (1.5, 3.0);
/// Length of a burst leg (s).
pub const BURST_LEG_S: (f32, f32) = (1.5, 2.5);
/// Sniff / graze duration (s) and the largest distance to the mother to start one.
pub const SNIFF_S: (f32, f32) = (1.0, 3.5);
pub const SNIFF_MAX_M: f32 = 3.0;
/// Looking at the mother (s).
pub const IDLE_S: (f32, f32) = (0.8, 2.0);
/// Hop (sine) while bursting: height (m) and period (s).
pub const HOP_HEIGHT_M: f32 = 0.12;
pub const HOP_PERIOD_S: f32 = 0.45;
/// The mother counts as walking above this speed (m/s).
const MOTHER_MOVING_MPS: f32 = 0.15;
/// Retarget interval of a returning baby (s).
const RETURN_RETARGET_S: f32 = 1.0;

/// What the baby is doing (drives the clip choice in the host).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Stands and looks at its mother (sometimes hopping on the spot).
    #[default]
    Idle,
    /// Walks to a point around her.
    Roam,
    /// Short run with hops.
    Burst,
    /// Stands and sniffs / grazes (`eat` action).
    Sniff,
    /// Catches up with her (trot, or run when > [`LEASH_HARD_M`]).
    Return,
}

/// Behaviour state of one baby (not saved).
#[derive(Debug, Clone, PartialEq)]
pub struct BabyPlay {
    pub mode: Mode,
    /// Hop height in metres to add to the draw height (0 when not hopping).
    pub hop: f32,
    /// The baby's own clip clock (advances faster while it runs).
    pub clip_t: f32,
    /// Clip playback rate of the last step.
    pub rate: f32,
    /// Seconds into the current sniff (for the `eat` action).
    pub sniff_t: f32,
    timer: f32,
    orbit: f32,
    last_mother: Vec2,
    seen_mother: bool,
    hop_t: f32,
    excited: bool,
}

impl Default for BabyPlay {
    fn default() -> Self {
        Self {
            mode: Mode::Idle,
            hop: 0.0,
            clip_t: 0.0,
            rate: 1.0,
            sniff_t: 0.0,
            timer: 0.0,
            orbit: 0.0,
            last_mother: Vec2::ZERO,
            seen_mother: false,
            hop_t: 0.0,
            excited: false,
        }
    }
}

/// Everything the baby needs to know about its surroundings for one step.
pub struct Ctx<'a> {
    pub mother: Vec2,
    pub mother_facing: Vec2,
    /// The mother's wander area while she is home: the baby stays inside it.
    pub home: Option<&'a WanderArea>,
    pub grid: &'a Grid,
    /// Follow speeds (m/s) on a path / on grass.
    pub path_speed: f32,
    pub grass_speed: f32,
    pub dt: f32,
}

fn range(rng: &mut Pcg32, r: (f32, f32)) -> f32 {
    r.0 + (r.1 - r.0) * (rng.below(1000) as f32 / 999.0)
}

/// A point to walk to around the mother, or `None` if no valid cell was found.
fn pick_point(b: &Baby, ctx: &Ctx, rng: &mut Pcg32, moving: bool, near: bool) -> Option<Vec2> {
    let heading = ctx.mother_facing.normalize_or(Vec2::Y);
    let side = heading.perp();
    for _ in 0..8 {
        let cand = if near {
            // straight back towards her: a point on a small ring on the baby's side
            let to_b = (b.pos - ctx.mother).normalize_or(Vec2::X);
            ctx.mother + to_b * range(rng, (1.5, 2.5))
        } else if moving {
            let sign = if rng.below(2) == 0 { -1.0 } else { 1.0 };
            ctx.mother + heading * range(rng, AHEAD_M) + side * sign * range(rng, LATERAL_M)
        } else {
            // circle her: advance mostly one way round, sometimes back
            let step = range(rng, ORBIT_STEP_DEG).to_radians();
            let a = b_orbit(b, rng, step);
            ctx.mother + Vec2::new(a.cos(), a.sin()) * range(rng, RING_M)
        };
        let cand = if cand.distance(ctx.mother) > LEASH_SOFT_M - 0.5 {
            ctx.mother + (cand - ctx.mother).normalize_or(Vec2::X) * (LEASH_SOFT_M - 0.5)
        } else {
            cand
        };
        let c = cell_of(cand);
        let ok = match ctx.home {
            Some(area) => area.contains(c),
            None => ctx.grid.is_passable(c, false),
        };
        if ok {
            return Some(cand);
        }
    }
    None
}

fn b_orbit(b: &Baby, rng: &mut Pcg32, step: f32) -> f32 {
    let dir = if rng.below(4) == 0 { -1.0 } else { 1.0 };
    b.play.orbit + dir * step
}

/// Builds the route to `target` (a point) into `b.route`; `false` if there is none.
fn route_to(b: &mut Baby, ctx: &Ctx, target: Vec2) -> bool {
    let here = cell_of(b.pos);
    let goal = cell_of(target);
    b.route.clear();
    if goal == here {
        return true;
    }
    match ctx.home {
        Some(area) => {
            if let Some(r) = area.route(here, goal) {
                b.route = r;
            }
        }
        None => {
            if let Some(mut r) = nav::find_path(ctx.grid, here, goal, true) {
                r.retain(|&c| c != here);
                b.route = r;
            }
        }
    }
    !b.route.is_empty()
}

fn start(b: &mut Baby, mode: Mode, timer: f32) {
    b.play.mode = mode;
    b.play.timer = timer;
    b.play.sniff_t = 0.0;
    b.play.excited = false;
    b.route.clear();
}

/// Picks the next thing to do.
fn pick_next(b: &mut Baby, ctx: &Ctx, rng: &mut Pcg32, moving: bool, dist: f32) {
    let r = rng.below(100);
    let can_sniff = dist <= SNIFF_MAX_M;
    let (mode, leg, near_pick) = if moving {
        if r < 15 && can_sniff {
            (Mode::Sniff, range(rng, (SNIFF_S.0, 2.0)), false)
        } else if r < 40 {
            (Mode::Burst, range(rng, BURST_LEG_S), false)
        } else {
            (Mode::Roam, range(rng, LEG_WALK_S), false)
        }
    } else if r < 25 && can_sniff {
        (Mode::Sniff, range(rng, SNIFF_S), false)
    } else if r < 50 {
        (Mode::Burst, range(rng, BURST_LEG_S), false)
    } else if r < 85 {
        (Mode::Roam, range(rng, LEG_STAND_S), false)
    } else {
        (Mode::Idle, range(rng, IDLE_S), false)
    };
    start(b, mode, leg);
    match mode {
        Mode::Sniff => {}
        Mode::Idle => b.play.excited = rng.below(10) < 4,
        _ => {
            let ok = pick_point(b, ctx, rng, moving, near_pick).is_some_and(|p| {
                if !moving {
                    b.play.orbit = (p - ctx.mother).to_angle();
                }
                route_to(b, ctx, p)
            });
            if !ok {
                start(b, Mode::Idle, 0.5);
            }
        }
    }
}

/// One step of the baby's behaviour: moves it, sets facing, hop and clip clock.
pub fn step(b: &mut Baby, ctx: &Ctx, rng: &mut Pcg32) {
    let dt = ctx.dt;
    let mother_moving = b.play.seen_mother
        && dt > 0.0
        && ctx.mother.distance(b.play.last_mother) / dt > MOTHER_MOVING_MPS;
    b.play.last_mother = ctx.mother;
    b.play.seen_mother = true;
    let dist = b.pos.distance(ctx.mother);

    // leash
    if dist > LEASH_SOFT_M && b.play.mode != Mode::Return {
        start(b, Mode::Return, 0.0);
    }
    let mut speed_factor = 0.0;
    match b.play.mode {
        Mode::Idle | Mode::Sniff => {
            b.play.timer -= dt;
            if b.play.mode == Mode::Sniff {
                b.play.sniff_t += dt;
            }
            if mother_moving && b.play.timer > 1.0 {
                b.play.timer = 1.0;
            }
            b.facing = (ctx.mother - b.pos).normalize_or(b.facing);
            if b.play.timer <= 0.0 {
                pick_next(b, ctx, rng, mother_moving, dist);
            }
        }
        Mode::Roam | Mode::Burst => {
            b.play.timer -= dt;
            speed_factor = if b.play.mode == Mode::Burst {
                BURST_FACTOR
            } else if mother_moving {
                TROT_FACTOR
            } else {
                AMBLE_FACTOR
            };
            if b.route.is_empty() || b.play.timer <= 0.0 {
                pick_next(b, ctx, rng, mother_moving, dist);
            }
        }
        Mode::Return => {
            b.play.timer -= dt;
            speed_factor = if dist > LEASH_HARD_M {
                BURST_FACTOR
            } else {
                CATCHUP_FACTOR
            };
            if dist < RETURN_DONE_M {
                start(b, Mode::Idle, 0.4);
                speed_factor = 0.0;
            } else if b.route.is_empty() || b.play.timer <= 0.0 {
                b.play.timer = RETURN_RETARGET_S;
                let mut ok = route_to(b, ctx, ctx.mother);
                if !ok && ctx.home.is_none() {
                    b.route = vec![cell_of(ctx.mother)];
                    ok = true;
                }
                if !ok {
                    speed_factor = 0.0;
                }
            }
        }
    }

    // move
    let mut moved_speed = 0.0;
    if speed_factor > 0.0 && !b.route.is_empty() {
        let base = match ctx.grid.surface(cell_of(b.pos)) {
            Some(Surface::Path) => ctx.path_speed,
            _ => ctx.grass_speed,
        }
        .max(wander::WANDER_SPEED);
        let (moved, dir) = wander::follow_route(&mut b.pos, &mut b.route, base * speed_factor * dt);
        if dir != Vec2::ZERO {
            b.facing = dir;
        }
        moved_speed = if dt > 0.0 { moved / dt } else { 0.0 };
    }

    // hop + clip clock
    let hopping = (b.play.mode == Mode::Burst && moved_speed > 0.0)
        || (b.play.mode == Mode::Idle && b.play.excited);
    if hopping {
        b.play.hop_t += dt;
        b.play.hop = HOP_HEIGHT_M
            * (std::f32::consts::PI * b.play.hop_t / HOP_PERIOD_S)
                .sin()
                .abs();
    } else {
        b.play.hop_t = 0.0;
        b.play.hop = 0.0;
    }
    b.play.rate = if moved_speed > 0.0 {
        (moved_speed / crate::player::WALK_CLIP_AUTHORED_SPEED).clamp(0.8, 2.0)
    } else {
        1.0
    };
    b.play.clip_t += dt * b.play.rate;
}
