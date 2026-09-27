//! Ambient animals (GAME-AMBIENT): ducks and ducklings on the river, frogs at the pond.
//! Pure decoration with behaviour — no mission, no collision, no prompt, not saved.
//! Deterministic from the seed and the inputs (`dt`, player position; AMB-004).
//!
//! - **Ducks** live on `river` elements (never on `stream`s — the level-3 stream is a riddle
//!   guard without ducks — ponds, pools or the fountain). Per river: [`ADULT_DUCKS`] adults
//!   (the first is a mother with [`DUCKLINGS`] ducklings in a line) that swim in river
//!   coordinates `(s, c)` of the water field (TECH-WATER), so they follow the bend and stay on
//!   open water; they keep ≥ 0.6 m apart, pass the bridge piles and stones, play `dip` /
//!   `preen` / `flap` every 5–20 s, flee from the player and drift back near the bridge.
//! - **Frogs** live at `pond` elements, sitting on lily-pad spots; they croak, hop between the
//!   spots of their pad and hop into the water when the player comes close, swim away and
//!   climb back onto a free pad when calm.
//!
//! Positions are level coordinates; [`Ambient::poses`] returns world-space poses for drawing.

use glam::{Quat, Vec2, Vec3};

use crate::coords::level_to_world;
use crate::level::{ElementType, LevelData};
use crate::rng::Pcg32;
use crate::scene::LevelScene;
use crate::water::{bob, bob_params, bob_transform, water_time, RiverPath};

/// Adult ducks per river (GAME-AMBIENT 2: 2–4).
pub const ADULT_DUCKS: usize = 3;
/// Ducklings following the first duck (the mother) in a line.
pub const DUCKLINGS: usize = 3;
/// Swim speed range (m/s).
pub const DUCK_SPEED: (f32, f32) = (0.3, 0.6);
/// Flee speed (m/s).
pub const DUCK_FLEE_SPEED: f32 = 2.4;
/// A duck flees when the player comes this close (GAME-AMBIENT 3).
pub const DUCK_FLEE_TRIGGER_M: f32 = 2.5;
/// …until it is at least this far away.
pub const DUCK_FLEE_TO_M: f32 = 4.0;
/// Calm time after fleeing (s).
pub const DUCK_CALM_S: (f32, f32) = (5.0, 10.0);
/// Time between two `dip` / `preen` / `flap` actions (s).
pub const ACTION_EVERY_S: (f32, f32) = (5.0, 20.0);
/// Minimum distance between adult ducks (m).
pub const DUCK_MIN_GAP_M: f32 = 0.6;
/// Clearance of a duck's centre from an obstacle's foam radius (m).
pub const DUCK_CLEARANCE_M: f32 = 0.2;
/// Ducks stay within this many metres of their home along the river, except on rare
/// excursions and when fleeing (AMB-008: within 8 m of the bridge most of the time).
pub const DUCK_HOME_M: f32 = 5.0;
/// Home of the ducks: this far downstream of the bridge (in view of the default camera,
/// which looks north over the bridge).
pub const DUCK_HOME_BELOW_BRIDGE_M: f32 = 2.5;
/// Distance between ducklings in the line (m).
pub const DUCKLING_GAP_M: f32 = 0.42;
/// Frogs hop into the water when the player comes this close (GAME-AMBIENT 7).
pub const FROG_FLEE_TRIGGER_M: f32 = 2.0;
/// Frog swim speed (m/s, `frog.swim.speed`).
pub const FROG_SWIM_SPEED: f32 = 0.3;
/// Frogs per pond.
pub const FROGS_PER_POND: usize = 2;

/// Clip durations (s) from `animal_anims.toml` (frames / 30; checked by a test).
pub const DUCK_DIP_S: f32 = 2.0;
pub const DUCK_FLAP_S: f32 = 1.5;
pub const DUCK_PREEN_S: f32 = 2.5;
pub const DUCK_SWIM_SPEED: f32 = 0.45;
pub const FROG_CROAK_S: f32 = 1.5;
pub const FROG_HOP_S: f32 = 1.0;
/// Frog hop: the frog moves between these clip times (`takeoff = 8`, `land = 22` frames).
pub const FROG_TAKEOFF_S: f32 = 8.0 / 30.0;
pub const FROG_LAND_S: f32 = 22.0 / 30.0;
/// Crossfade of one-shot clips (s).
const FADE_S: f32 = 0.15;
/// Lily-pad spots a frog can sit on (Blender x east / y north of the `lily_pad` model,
/// `kit_water.py` `lily_pad`), and the pad top height.
pub const PAD_SPOTS: [(f32, f32); 3] = [(0.32, 0.12), (-0.3, 0.15), (-0.2, -0.26)];
pub const PAD_TOP_Y: f32 = 0.02;
/// Water margin: ducks keep `|c|` below half width − this (m).
const RIVER_MARGIN_M: f32 = 0.6;
/// Ends of the river the ducks stay away from (m of `s`).
const RIVER_END_MARGIN_M: f32 = 2.0;

/// Kind of ambient animal = the model it is drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbientKind {
    Duck,
    Duckling,
    Frog,
}

impl AmbientKind {
    pub fn model(self) -> &'static str {
        match self {
            AmbientKind::Duck => "duck",
            AmbientKind::Duckling => "duckling",
            AmbientKind::Frog => "frog",
        }
    }
}

/// A one-shot clip being played.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Action {
    pub clip: &'static str,
    pub t: f32,
    pub dur: f32,
}

impl Action {
    fn new(clip: &'static str, dur: f32) -> Self {
        Self { clip, t: 0.0, dur }
    }

    fn done(&self) -> bool {
        self.t >= self.dur
    }

    /// Crossfade weight (in and out).
    pub fn blend(&self) -> f32 {
        (self.t / FADE_S)
            .min((self.dur - self.t) / FADE_S)
            .clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DuckMode {
    /// Swim towards a target in river coordinates.
    Swim {
        s: f32,
        c: f32,
        speed: f32,
    },
    /// Float (idle) for a while.
    Rest {
        left: f32,
    },
    /// Flee to a target, then calm down.
    Flee {
        s: f32,
        c: f32,
    },
    Calm {
        left: f32,
    },
    /// Duckling: follows the animal `lead` in the line.
    Follow {
        lead: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FrogMode {
    Sit {
        spot: usize,
    },
    /// Hop from `from` to `to` (level), landing on `land_spot` or in the water.
    Hop {
        from: Vec2,
        to: Vec2,
        from_y: f32,
        land_spot: Option<usize>,
    },
    /// Swim in the pond towards `target`; climb onto a pad when calm.
    Swim {
        target: Vec2,
        calm: f32,
    },
}

/// One ambient animal.
#[derive(Debug, Clone, PartialEq)]
pub struct AmbientAnimal {
    pub kind: AmbientKind,
    /// Level position.
    pub pos: Vec2,
    /// Unit heading (level).
    pub heading: Vec2,
    /// River (index into the water scene) and river coordinates of ducks.
    river: usize,
    s: f32,
    c: f32,
    duck: DuckMode,
    frog: FrogMode,
    speed: f32,
    pub action: Option<Action>,
    next_action: f32,
    pub idle_time: f32,
    pub swim_time: f32,
    /// 0 = idle clip, 1 = swim clip.
    pub swim_blend: f32,
    /// Frogs: in the water (swim clip, origin at the water surface).
    pub in_water: bool,
    /// Height of the origin above the water surface (frog on a pad / in a hop).
    pub lift: f32,
    /// Frogs: the pad this frog sits on / came from (for the pad's bobbing).
    pad: Option<usize>,
    /// Counters for the tests (AMB-002).
    pub actions_played: u32,
    pub distance_swum: f32,
    /// Seconds since the last `dip` started (water rings), `f32::MAX` if none.
    pub dip_age: f32,
}

/// A lily pad the frogs use (level position, yaw).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pad {
    pub pos: Vec2,
    pub yaw: f32,
    pub pond: usize,
}

impl Pad {
    /// Level position of a spot on the pad (without bobbing).
    pub fn spot(&self, k: usize) -> Vec2 {
        let (x, y) = PAD_SPOTS[k % PAD_SPOTS.len()];
        let (s, c) = self.yaw.sin_cos();
        self.pos + Vec2::new(c * x - s * y, s * x + c * y)
    }
}

/// Draw data of one ambient animal (world space).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmbientPose {
    pub model: &'static str,
    pub pos: Vec3,
    pub yaw: f32,
    /// Extra rotation applied before the yaw (bobbing roll).
    pub tilt: Quat,
    pub idle_clip: &'static str,
    pub idle_time: f32,
    pub walk_clip: &'static str,
    pub walk_time: f32,
    pub walk_blend: f32,
    pub action: Option<(&'static str, f32)>,
    pub action_blend: f32,
}

/// Water ripple of a swimming / dipping duck for the water shader (GAME-AMBIENT 5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ripple {
    /// Level position and unit heading.
    pub pos: Vec2,
    pub heading: Vec2,
    /// Wake strength 0…1 (speed).
    pub wake: f32,
    /// Dip ring age 0…1, or < 0 for none.
    pub ring: f32,
}

/// Butterflies per scenery area that lists `butterfly` in its props (GAME-AMBIENT 8).
pub const BUTTERFLIES_PER_AREA: usize = 4;
/// Butterfly flight height range (m).
pub const BUTTERFLY_HEIGHT_M: (f32, f32) = (0.35, 1.2);

/// A butterfly fluttering in loops over its meadow (a pure function of time).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Butterfly {
    /// Level centre of its loops and their radius (m).
    pub center: Vec2,
    pub radius: f32,
    pub phase: f32,
    /// Flat wing colour (sRGB).
    pub color: [f32; 3],
}

/// Draw data of a butterfly: world position, yaw and wing opening (0 closed … 1 open).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButterflyPose {
    pub pos: Vec3,
    pub yaw: f32,
    pub open: f32,
    pub color: [f32; 3],
}

impl Butterfly {
    /// Level position, height and whether it rests on a flower at time `t` (s).
    pub fn at(&self, t: f64) -> (Vec2, f32, bool) {
        // 12 s cycle: 9.5 s fluttering loops, 2.5 s resting on a flower of the meadow
        let cycle = ((t + self.phase as f64 * 12.0) % 12.0) as f32;
        let fly = |t: f32| {
            let w = 0.9;
            let a = w * t + self.phase * std::f32::consts::TAU;
            let p = self.center
                + Vec2::new(
                    self.radius * a.cos() + 0.25 * (2.3 * a).sin(),
                    self.radius * 0.7 * (1.3 * a).sin(),
                );
            let h = 0.85 + 0.25 * (1.7 * a).sin() + 0.06 * (7.0 * a).sin();
            (p, h)
        };
        let land_t = (t as f32) - cycle + 9.5; // the moment of landing in this cycle
        if cycle < 9.5 {
            let (p, h) = fly(t as f32);
            // glide down to the flower during the last second of the flight
            let k = ((cycle - 8.5) / 1.0).clamp(0.0, 1.0);
            let (lp, _) = fly(land_t);
            (p.lerp(lp, k * k), h + (0.5 - h) * k, false)
        } else {
            let (lp, _) = fly(land_t);
            (lp, 0.5, true)
        }
    }
}

/// All ambient animals of a (joined) level.
#[derive(Debug, Clone)]
pub struct Ambient {
    pub animals: Vec<AmbientAnimal>,
    pub butterflies: Vec<Butterfly>,
    pub pads: Vec<Pad>,
    rivers: Vec<RiverPath>,
    /// Home `s` (bridge) per river.
    home: Vec<f32>,
    /// Obstacles per river in river coordinates (s, c, radius).
    obstacles: Vec<Vec<(f32, f32, f32)>>,
    /// Pond areas (level min, max) per pond.
    ponds: Vec<(Vec2, Vec2)>,
    water: crate::water::WaterScene,
    rng: Pcg32,
    /// Seconds since start (bobbing clock).
    pub time: f64,
    /// Night (GAME-NIGHT, GAME-AMBIENT): ducks sleep (head tucked, no swimming), butterflies
    /// are hidden, frogs croak more often.
    pub night: bool,
}

fn unit(rng: &mut Pcg32) -> f32 {
    rng.next_u32() as f32 / 4_294_967_296.0
}

fn range(rng: &mut Pcg32, r: (f32, f32)) -> f32 {
    r.0 + (r.1 - r.0) * unit(rng)
}

fn yaw_of(heading: Vec2) -> f32 {
    let f = level_to_world(heading);
    f.x.atan2(f.z)
}

impl Ambient {
    /// Places the ambient animals of a level (seeded).
    pub fn new(data: &LevelData, scene: &LevelScene, seed: u64) -> Self {
        let mut rng = Pcg32::new(seed ^ 0xA3B1_7E55_D00C_u64);
        let water = scene.water.clone();
        let rivers = water.rivers.clone();
        let mut animals = Vec::new();
        let mut home = Vec::new();
        let mut obstacles = Vec::new();
        for (ri, path) in rivers.iter().enumerate() {
            let s_home = path
                .ids
                .iter()
                .filter_map(|id| data.element(id))
                .find(|e| e.ty == ElementType::Path && e.kind.as_deref() == Some("bridge"))
                .map_or(path.length() / 2.0, |b| {
                    let r = b.rect;
                    path.coords(Vec2::new(
                        r.x as f32 + r.w as f32 / 2.0,
                        r.z as f32 + r.d as f32 / 2.0,
                    ))
                    .0 + DUCK_HOME_BELOW_BRIDGE_M
                });
            home.push(s_home);
            obstacles.push(
                water
                    .obstacles
                    .iter()
                    .filter(|o| o.river)
                    .filter(|o| path.contains(crate::level::cell_of(o.pos)))
                    .map(|o| {
                        let (s, c) = path.coords(o.pos);
                        (s, c, o.radius)
                    })
                    .collect(),
            );
            // ducks only on rivers (the stream is a riddle guard without ducks)
            let ducky = path
                .ids
                .iter()
                .filter_map(|id| data.element(id))
                .any(|e| e.ty == ElementType::Landmark && e.kind.as_deref() == Some("river"));
            if !ducky {
                continue;
            }
            let cmax = path.half_width - RIVER_MARGIN_M;
            let first = animals.len();
            for k in 0..ADULT_DUCKS {
                // the mother (k = 0) a little downstream of home, the others around her
                let s = s_home + [1.0, -1.8, 3.4][k % 3] + range(&mut rng, (-0.3, 0.3));
                let c = [-0.5, 0.3, 0.6][k % 3] * cmax;
                animals.push(AmbientAnimal::duck(
                    AmbientKind::Duck,
                    ri,
                    s,
                    c,
                    path,
                    &mut rng,
                ));
            }
            // mother (the first duck) and her ducklings in a line behind her
            for k in 0..DUCKLINGS {
                let lead = if k == 0 { first } else { animals.len() - 1 };
                let (ls, lc) = (animals[lead].s, animals[lead].c);
                let mut d = AmbientAnimal::duck(
                    AmbientKind::Duckling,
                    ri,
                    ls - DUCKLING_GAP_M,
                    lc,
                    path,
                    &mut rng,
                );
                d.duck = DuckMode::Follow { lead };
                animals.push(d);
            }
        }
        // frogs on the lily pads of the ponds
        let mut pads = Vec::new();
        let mut ponds = Vec::new();
        for e in data
            .elements
            .iter()
            .filter(|e| e.ty == ElementType::Landmark && e.kind.as_deref() == Some("pond"))
        {
            let r = e.rect;
            let lo = Vec2::new(r.x as f32, r.z as f32);
            let hi = lo + Vec2::new(r.w as f32, r.d as f32);
            let pond = ponds.len();
            ponds.push((lo, hi));
            for p in scene.placements.iter().filter(|p| p.model == "lily_pad") {
                let lp = crate::coords::world_to_level(p.pos);
                if lp.cmpge(lo).all() && lp.cmple(hi).all() {
                    pads.push(Pad {
                        pos: lp,
                        yaw: p.yaw,
                        pond,
                    });
                }
            }
            let pond_pads: Vec<usize> = (0..pads.len()).filter(|&i| pads[i].pond == pond).collect();
            for k in 0..FROGS_PER_POND.min(pond_pads.len()) {
                // the last pads of the pond are the frogs' start pads (scene::POND_LILY_PADS)
                let pad = pond_pads[pond_pads.len() - 1 - k];
                let spot = rng.below(PAD_SPOTS.len() as u32) as usize;
                let mut f = AmbientAnimal::blank(AmbientKind::Frog, pads[pad].spot(spot));
                f.frog = FrogMode::Sit {
                    spot: pad * PAD_SPOTS.len() + spot,
                };
                f.pad = Some(pad);
                f.lift = PAD_TOP_Y;
                f.in_water = false;
                f.heading = Vec2::from_angle(unit(&mut rng) * std::f32::consts::TAU);
                f.next_action = range(&mut rng, (2.0, 8.0));
                animals.push(f);
            }
        }
        // butterflies only where a scenery area lists them (the meadow riddle, loc_meadow);
        // never in the garden or at other places (riddle guard, GAME-AMBIENT 8)
        let colors = [
            [0.96, 0.81, 0.23], // flower_yellow
            [0.97, 0.96, 0.93], // white
            [0.96, 0.66, 0.23], // orange (flower_center)
            [0.56, 0.36, 0.82], // flower_purple
        ];
        let mut butterflies = Vec::new();
        for sc in data
            .scenery
            .iter()
            .filter(|sc| sc.props.iter().any(|p| p == "butterfly"))
        {
            let r = sc.rect;
            let lo = Vec2::new(r.x as f32, r.z as f32);
            let size = Vec2::new(r.w as f32, r.d as f32);
            for k in 0..BUTTERFLIES_PER_AREA {
                let radius = (0.5 + 0.5 * unit(&mut rng))
                    .min(size.min_element() / 2.0 - 0.3)
                    .max(0.3);
                let center = lo
                    + Vec2::new(
                        radius + 0.3 + unit(&mut rng) * (size.x - 2.0 * radius - 0.6).max(0.0),
                        radius + 0.3 + unit(&mut rng) * (size.y - 2.0 * radius - 0.6).max(0.0),
                    );
                butterflies.push(Butterfly {
                    center,
                    radius,
                    phase: unit(&mut rng),
                    color: colors[k % colors.len()],
                });
            }
        }
        Self {
            animals,
            butterflies,
            pads,
            rivers,
            home,
            obstacles,
            ponds,
            water,
            rng,
            time: 0.0,
            night: false,
        }
    }

    /// Home `s` (the bridge) of a river.
    pub fn home_s(&self, river: usize) -> f32 {
        self.home[river]
    }

    pub fn rivers(&self) -> &[RiverPath] {
        &self.rivers
    }

    /// Advances all ambient animals by `dt` seconds; `player` = level position.
    pub fn update(&mut self, dt: f32, player: Vec2) {
        self.time += dt as f64;
        for i in 0..self.animals.len() {
            match self.animals[i].kind {
                AmbientKind::Duck | AmbientKind::Duckling => self.update_duck(i, dt, player),
                AmbientKind::Frog => self.update_frog(i, dt, player),
            }
        }
        self.separate_ducks();
        for a in &mut self.animals {
            a.idle_time += dt;
            if let Some(act) = &mut a.action {
                act.t += dt;
                if act.done() {
                    a.action = None;
                }
            }
            if a.dip_age < f32::MAX {
                a.dip_age += dt;
            }
        }
    }

    fn duck_target(&mut self, i: usize) -> DuckMode {
        let a = &self.animals[i];
        let path = &self.rivers[a.river];
        let home = self.home[a.river];
        let cmax = path.half_width - RIVER_MARGIN_M;
        let (lo, hi) = (RIVER_END_MARGIN_M, path.length() - RIVER_END_MARGIN_M);
        let rng = &mut self.rng;
        let roll = unit(rng);
        let s = if roll < 0.08 {
            // rare excursion along the river
            a.s + range(rng, (-10.0, 10.0))
        } else if roll < 0.7 && a.s < home + DUCK_HOME_M - 2.0 {
            // mostly downstream with the flow
            a.s + range(rng, (2.0, 5.0))
        } else {
            // paddle back upstream towards / past the bridge
            (home + range(rng, (-DUCK_HOME_M, DUCK_HOME_M * 0.3))).min(a.s - 1.0)
        };
        let s = s.clamp(lo, hi);
        let s = if roll >= 0.08 {
            s.clamp(home - DUCK_HOME_M, home + DUCK_HOME_M)
        } else {
            s
        };
        let c = range(rng, (-cmax, cmax));
        let speed = range(rng, DUCK_SPEED);
        DuckMode::Swim { s, c, speed }
    }

    fn update_duck(&mut self, i: usize, dt: f32, player: Vec2) {
        if self.night {
            // asleep on the water, head tucked: no swimming, no actions, no fleeing
            let a = &mut self.animals[i];
            a.action = None;
            a.swim_blend = (a.swim_blend - dt * 2.0).max(0.0);
            return;
        }
        let adult = self.animals[i].kind == AmbientKind::Duck;
        // react to the player (ducklings follow their mother)
        if adult {
            let d = self.animals[i].pos.distance(player);
            let fleeing = matches!(self.animals[i].duck, DuckMode::Flee { .. });
            if d < DUCK_FLEE_TRIGGER_M && !fleeing {
                let (s, c) = self.flee_target(i, player);
                let a = &mut self.animals[i];
                a.duck = DuckMode::Flee { s, c };
                a.action = Some(Action::new("flap", DUCK_FLAP_S));
                a.actions_played += 1;
            }
        }
        // one-shot actions every 5–20 s
        let busy = self.animals[i].action.is_some();
        {
            let a = &mut self.animals[i];
            a.next_action -= dt;
        }
        if !busy && self.animals[i].next_action <= 0.0 {
            let pick = self.rng.below(3);
            let (clip, dur) = [
                ("dip", DUCK_DIP_S),
                ("preen", DUCK_PREEN_S),
                ("flap", DUCK_FLAP_S),
            ][pick as usize];
            let next = range(&mut self.rng, ACTION_EVERY_S);
            let a = &mut self.animals[i];
            a.action = Some(Action::new(clip, dur));
            a.actions_played += 1;
            a.next_action = next + dur;
            if clip == "dip" {
                a.dip_age = 0.0;
            }
            if adult && matches!(a.duck, DuckMode::Swim { .. }) {
                // stop while dipping / preening
                a.duck = DuckMode::Rest { left: dur };
            }
        }
        let path = &self.rivers[self.animals[i].river];
        let cmax = path.half_width - RIVER_MARGIN_M;
        let (goal, speed) = match self.animals[i].duck {
            DuckMode::Swim { s, c, speed } => ((s, c), speed),
            DuckMode::Flee { s, c } => ((s, c), DUCK_FLEE_SPEED),
            DuckMode::Rest { .. } | DuckMode::Calm { .. } => {
                let a = &self.animals[i];
                ((a.s, a.c), 0.0)
            }
            DuckMode::Follow { lead } => {
                let l = &self.animals[lead];
                let dir = if l.heading.dot(path.dir_at(l.s)) >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                let goal = (l.s - dir * DUCKLING_GAP_M, l.c);
                let a = &self.animals[i];
                let dist = Vec2::new(goal.0 - a.s, goal.1 - a.c).length();
                let lead_speed = l.speed.max(0.2);
                (
                    (goal.0, goal.1),
                    if dist > 0.05 {
                        lead_speed * (0.8 + dist)
                    } else {
                        0.0
                    },
                )
            }
        };
        let a = &self.animals[i];
        let delta = Vec2::new(goal.0 - a.s, goal.1 - a.c);
        let step = (speed * dt).min(delta.length());
        let mut ns = a.s;
        let mut nc = a.c;
        if step > 1e-6 {
            let d = delta.normalize();
            ns += d.x * step;
            nc += d.y * step;
        }
        nc = nc.clamp(-cmax, cmax);
        // pass around obstacles (bridge piles, stones): push sideways out of the clearance
        for &(os, oc, r) in &self.obstacles[a.river] {
            let need = r + DUCK_CLEARANCE_M;
            let d = Vec2::new(ns - os, nc - oc);
            if d.length() < need {
                let side = if nc >= oc { 1.0 } else { -1.0 };
                let dc = (need * need - (ns - os) * (ns - os)).max(0.0).sqrt();
                let cand = oc + side * dc;
                nc = if cand.abs() <= cmax {
                    cand
                } else {
                    oc - side * dc
                };
                if nc.abs() > cmax {
                    // no room beside it: stay behind the obstacle
                    nc = nc.clamp(-cmax, cmax);
                    ns = a.s;
                }
            }
        }
        let new_pos = path.point(ns, nc);
        let moved = new_pos - a.pos;
        let dist = moved.length();
        let a = &mut self.animals[i];
        a.s = ns;
        a.c = nc;
        a.distance_swum += dist;
        a.speed = if dt > 0.0 { dist / dt } else { 0.0 };
        if dist > 1e-5 {
            a.heading = a
                .heading
                .lerp(moved / dist, (dt * 6.0).min(1.0))
                .normalize_or(moved / dist);
        }
        a.pos = new_pos;
        let want = (a.speed / 0.15).clamp(0.0, 1.0);
        a.swim_blend += (want - a.swim_blend) * (1.0 - (-6.0 * dt).exp());
        a.swim_time += dt * (a.speed / DUCK_SWIM_SPEED).clamp(0.4, 2.2);
        // mode transitions
        let arrived = step >= delta.length() - 1e-4;
        let far = a.pos.distance(player) >= DUCK_FLEE_TO_M + 0.5;
        match a.duck {
            DuckMode::Swim { .. } if arrived => {
                a.duck = DuckMode::Rest { left: 0.0 };
            }
            DuckMode::Rest { left } => {
                let left = left - dt;
                a.duck = DuckMode::Rest { left };
                if left <= 0.0 && a.action.is_none() {
                    let m = self.duck_target(i);
                    let rest = unit(&mut self.rng) < 0.25;
                    let a = &mut self.animals[i];
                    a.duck = if rest {
                        DuckMode::Rest {
                            left: 1.0 + 3.0 * unit(&mut self.rng),
                        }
                    } else {
                        m
                    };
                }
            }
            DuckMode::Flee { .. } if far => {
                let calm = range(&mut self.rng, DUCK_CALM_S);
                self.animals[i].duck = DuckMode::Calm { left: calm };
            }
            DuckMode::Flee { .. } if arrived => {
                // still too close: the next leg (greedy, keeps the distance growing)
                let (s, c) = self.flee_target(i, player);
                let a = &mut self.animals[i];
                if (s - a.s).abs() + (c - a.c).abs() < 0.05 {
                    let calm = range(&mut self.rng, DUCK_CALM_S);
                    self.animals[i].duck = DuckMode::Calm { left: calm };
                } else {
                    a.duck = DuckMode::Flee { s, c };
                }
            }
            DuckMode::Calm { left } => {
                let near = a.pos.distance(player) < DUCK_FLEE_TRIGGER_M;
                a.duck = if near {
                    DuckMode::Calm { left }
                } else {
                    DuckMode::Calm { left: left - dt }
                };
                if left - dt <= 0.0 && !near {
                    self.animals[i].duck = DuckMode::Rest { left: 0.0 };
                }
            }
            _ => {}
        }
    }

    /// Flee target: the river point within one short leg (s ± 3 m, three lanes) farthest
    /// from the player; fleeing chains legs until the duck is far enough.
    fn flee_target(&self, i: usize, player: Vec2) -> (f32, f32) {
        let a = &self.animals[i];
        let path = &self.rivers[a.river];
        let cmax = path.half_width - RIVER_MARGIN_M;
        let (lo, hi) = (RIVER_END_MARGIN_M, path.length() - RIVER_END_MARGIN_M);
        let mut best = (a.s, a.c, a.pos.distance(player));
        for k in -6..=6 {
            let s = (a.s + k as f32 * 0.5).clamp(lo, hi);
            for c in [-cmax, 0.0, cmax] {
                let d = path.point(s, c).distance(player) - (s - a.s).abs() * 0.02;
                if d > best.2 {
                    best = (s, c, d);
                }
            }
        }
        (best.0, best.1)
    }

    /// Keeps adult ducks ≥ [`DUCK_MIN_GAP_M`] apart (pushes along the river).
    fn separate_ducks(&mut self) {
        let n = self.animals.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (a, b) = (&self.animals[i], &self.animals[j]);
                if a.kind != AmbientKind::Duck || b.kind != AmbientKind::Duck || a.river != b.river
                {
                    continue;
                }
                let d = a.pos.distance(b.pos);
                if d >= DUCK_MIN_GAP_M + 0.02 {
                    continue;
                }
                let push = (DUCK_MIN_GAP_M + 0.03 - d) * 0.5;
                let sign = if a.s <= b.s { -1.0 } else { 1.0 };
                let path = &self.rivers[a.river];
                let (si, sj) = (a.s + sign * push, b.s - sign * push);
                let (ci, cj) = (a.c, b.c);
                let (pi, pj) = (path.point(si, ci), path.point(sj, cj));
                self.animals[i].s = si;
                self.animals[i].pos = pi;
                self.animals[j].s = sj;
                self.animals[j].pos = pj;
            }
        }
    }

    fn spot_free(&self, spot: usize, except: usize) -> bool {
        !self.animals.iter().enumerate().any(|(k, a)| {
            k != except
                && a.kind == AmbientKind::Frog
                && match a.frog {
                    FrogMode::Sit { spot: s } => s == spot,
                    FrogMode::Hop { land_spot, .. } => land_spot == Some(spot),
                    FrogMode::Swim { .. } => false,
                }
        })
    }

    fn spot_pos(&self, spot: usize) -> Vec2 {
        self.pads[spot / PAD_SPOTS.len()].spot(spot % PAD_SPOTS.len())
    }

    fn update_frog(&mut self, i: usize, dt: f32, player: Vec2) {
        let near = self.animals[i].pos.distance(player) < FROG_FLEE_TRIGGER_M;
        match self.animals[i].frog {
            FrogMode::Sit { spot } => {
                let a = &mut self.animals[i];
                a.lift = PAD_TOP_Y;
                a.in_water = false;
                a.swim_blend = 0.0;
                if near && a.action.is_none() {
                    // hop into the water, away from the player
                    let away = (a.pos - player).normalize_or(Vec2::X);
                    let from = a.pos;
                    let pond = self.pads[spot / PAD_SPOTS.len()].pond;
                    let to = self.clamp_to_pond(pond, from + away * 0.6);
                    self.start_hop(i, to, None);
                    return;
                }
                a.next_action -= dt;
                if a.next_action <= 0.0 && a.action.is_none() {
                    let roll = unit(&mut self.rng);
                    // frogs croak more at night (gentle night sounds, GAME-NIGHT rule 2)
                    let every = if self.night { (1.0, 3.0) } else { (3.0, 9.0) };
                    let next = range(&mut self.rng, every);
                    let roll = if self.night { roll.max(0.3) } else { roll };
                    let pad = spot / PAD_SPOTS.len();
                    let other =
                        pad * PAD_SPOTS.len() + self.rng.below(PAD_SPOTS.len() as u32) as usize;
                    if roll < 0.3 && other != spot && self.spot_free(other, i) {
                        let to = self.spot_pos(other);
                        self.start_hop(i, to, Some(other));
                    } else {
                        let a = &mut self.animals[i];
                        a.action = Some(Action::new("croak", FROG_CROAK_S));
                        a.actions_played += 1;
                    }
                    self.animals[i].next_action = next;
                }
            }
            FrogMode::Hop {
                from,
                to,
                from_y,
                land_spot,
            } => {
                let t = self.animals[i].action.map_or(FROG_HOP_S, |a| a.t);
                let k = ((t - FROG_TAKEOFF_S) / (FROG_LAND_S - FROG_TAKEOFF_S)).clamp(0.0, 1.0);
                let to_y = if land_spot.is_some() { PAD_TOP_Y } else { 0.0 };
                let a = &mut self.animals[i];
                a.pos = from.lerp(to, k);
                a.lift = from_y + (to_y - from_y) * k;
                if a.action.is_none() || t >= FROG_HOP_S - dt {
                    a.action = None;
                    match land_spot {
                        Some(spot) => {
                            a.frog = FrogMode::Sit { spot };
                            a.pad = Some(spot / PAD_SPOTS.len());
                            a.in_water = false;
                        }
                        None => {
                            a.in_water = true;
                            a.pad = None;
                            let away = (a.pos - player).normalize_or(a.heading);
                            let target = a.pos + away * 1.5;
                            let calm = range(&mut self.rng, (6.0, 10.0));
                            let pond = self.pond_of(self.animals[i].pos);
                            let target = self.clamp_to_pond(pond, target);
                            self.animals[i].frog = FrogMode::Swim { target, calm };
                        }
                    }
                }
            }
            FrogMode::Swim { target, calm } => {
                let pond = self.pond_of(self.animals[i].pos);
                let calm = if near { calm.max(3.0) } else { calm - dt };
                let mut target = target;
                if near {
                    let a = &self.animals[i];
                    let away = (a.pos - player).normalize_or(a.heading);
                    target = self.clamp_to_pond(pond, a.pos + away * 1.5);
                }
                // calm again: swim to the nearest free pad spot and hop onto it
                let mut climb = None;
                if calm <= 0.0 {
                    let a = &self.animals[i];
                    let free = (0..self.pads.len() * PAD_SPOTS.len())
                        .filter(|&s| self.pads[s / PAD_SPOTS.len()].pond == pond)
                        .filter(|&s| self.spot_free(s, i))
                        .min_by(|&x, &y| {
                            a.pos
                                .distance(self.spot_pos(x))
                                .total_cmp(&a.pos.distance(self.spot_pos(y)))
                        });
                    if let Some(s) = free {
                        let p = self.spot_pos(s);
                        if a.pos.distance(p) <= 0.6 {
                            climb = Some((s, p));
                        } else {
                            target = p + (a.pos - p).normalize_or(Vec2::X) * 0.5;
                        }
                    }
                }
                if let Some((s, p)) = climb {
                    self.start_hop(i, p, Some(s));
                    return;
                }
                let a = &mut self.animals[i];
                let d = target - a.pos;
                let step = (FROG_SWIM_SPEED * dt).min(d.length());
                if step > 1e-6 {
                    let dir = d.normalize();
                    a.pos += dir * step;
                    a.heading = dir;
                    a.distance_swum += step;
                }
                a.speed = if dt > 0.0 { step / dt } else { 0.0 };
                a.swim_blend = 1.0;
                a.swim_time += dt;
                a.lift = 0.0;
                a.in_water = true;
                a.frog = FrogMode::Swim { target, calm };
            }
        }
    }

    fn start_hop(&mut self, i: usize, to: Vec2, land_spot: Option<usize>) {
        let a = &mut self.animals[i];
        let from = a.pos;
        if (to - from).length() > 1e-4 {
            a.heading = (to - from).normalize();
        }
        a.frog = FrogMode::Hop {
            from,
            to,
            from_y: a.lift,
            land_spot,
        };
        a.action = Some(Action::new("hop", FROG_HOP_S));
        a.actions_played += 1;
    }

    fn pond_of(&self, p: Vec2) -> usize {
        (0..self.ponds.len())
            .min_by(|&a, &b| {
                let da = p.distance(p.clamp(self.ponds[a].0, self.ponds[a].1));
                let db = p.distance(p.clamp(self.ponds[b].0, self.ponds[b].1));
                da.total_cmp(&db)
            })
            .unwrap_or(0)
    }

    /// Nearest point of open pond water (≥ 0.25 m from the waterline).
    fn clamp_to_pond(&self, pond: usize, p: Vec2) -> Vec2 {
        let Some(&(lo, hi)) = self.ponds.get(pond) else {
            return p;
        };
        let inset = crate::water::WATERLINE_INSET + 0.3;
        let mut q = p.clamp(lo + inset, hi - inset);
        // rounded corners of the pond: pull towards the centre until on open water
        let centre = (lo + hi) / 2.0;
        for _ in 0..20 {
            if self.water.is_water(q)
                && self.water.is_water(q + Vec2::new(0.25, 0.0))
                && self.water.is_water(q - Vec2::new(0.25, 0.0))
                && self.water.is_water(q + Vec2::new(0.0, 0.25))
                && self.water.is_water(q - Vec2::new(0.0, 0.25))
            {
                break;
            }
            q = q.lerp(centre, 0.15);
        }
        q
    }

    /// World-space draw poses (bobbing included) of all ambient animals.
    pub fn poses(&self, out: &mut Vec<AmbientPose>) {
        out.clear();
        let t = water_time(self.time);
        for a in &self.animals {
            let yaw = yaw_of(a.heading);
            let (pos, tilt) = match (a.kind, a.pad) {
                (AmbientKind::Frog, Some(pad)) if !a.in_water => {
                    // sits on its pad: moves exactly with the pad's bobbing (WATER-009)
                    let p = &self.pads[pad];
                    let origin = level_to_world(p.pos);
                    let b = bob(origin, bob_params("lily_pad"), t);
                    let rel = a.pos - p.pos;
                    // pad-local offset (inverse yaw), then the shader's pad transform
                    let (s, c) = p.yaw.sin_cos();
                    let local_level = Vec2::new(c * rel.x + s * rel.y, -s * rel.x + c * rel.y);
                    let local = Vec3::new(local_level.x, a.lift, -local_level.y);
                    let w = bob_transform(origin, p.yaw, local, b);
                    let axis = Quat::from_rotation_y(p.yaw) * Vec3::X;
                    (w, Quat::from_axis_angle(axis, b.roll))
                }
                _ => {
                    let origin = level_to_world(a.pos);
                    let mut params = bob_params(a.kind.model());
                    params.drift = 0.0; // they swim themselves
                    if a.kind == AmbientKind::Frog {
                        params.tilt = 0.0;
                    }
                    let b = bob(origin, params, t);
                    let axis = Quat::from_rotation_y(yaw) * Vec3::X;
                    (
                        origin + Vec3::Y * (a.lift + b.offset.y),
                        Quat::from_axis_angle(axis, b.roll),
                    )
                }
            };
            let (idle_clip, walk_clip) = match a.kind {
                AmbientKind::Frog => (if a.in_water { "swim" } else { "idle" }, "swim"),
                // asleep at night: `sleep` (head tucked) — the renderer falls back to `idle`
                _ if self.night => ("sleep", "swim"),
                _ => ("idle", "swim"),
            };
            out.push(AmbientPose {
                model: a.kind.model(),
                pos,
                yaw,
                tilt,
                idle_clip,
                idle_time: a.idle_time,
                walk_clip,
                walk_time: a.swim_time,
                walk_blend: if a.kind == AmbientKind::Frog {
                    0.0
                } else {
                    a.swim_blend
                },
                action: a.action.map(|x| (x.clip, x.t)),
                action_blend: a.action.map_or(0.0, |x| {
                    if x.clip == "hop" {
                        x.blend().max(if x.t > 0.05 { 1.0 } else { 0.0 })
                    } else {
                        x.blend()
                    }
                }),
            });
        }
    }

    /// World-space poses of the butterflies (wing flap 8 Hz in flight, slow on a flower).
    pub fn butterfly_poses(&self, out: &mut Vec<ButterflyPose>) {
        out.clear();
        if self.night {
            return; // hidden at night (GAME-AMBIENT, art plan)
        }
        for b in &self.butterflies {
            let (p, h, rest) = b.at(self.time);
            let (p2, _, _) = b.at(self.time + 0.05);
            let d = p2 - p;
            let heading = if d.length_squared() > 1e-8 {
                d.normalize()
            } else {
                Vec2::NEG_Y
            };
            let t = self.time as f32 + b.phase * 7.0;
            let open = if rest {
                0.55 + 0.45 * (std::f32::consts::TAU * 0.6 * t).cos().abs()
            } else {
                0.2 + 0.8 * (std::f32::consts::TAU * 8.0 * t).cos().abs()
            };
            out.push(ButterflyPose {
                pos: crate::coords::level_to_world_at(p, h),
                yaw: yaw_of(heading),
                open,
                color: b.color,
            });
        }
    }

    /// Ripples of the swimming and dipping ducks and swimming frogs (water shader).
    pub fn ripples(&self, out: &mut Vec<Ripple>) {
        out.clear();
        for a in &self.animals {
            let swimming = a.kind != AmbientKind::Frog || a.in_water;
            if !swimming {
                continue;
            }
            let ring = if a.dip_age < DUCK_DIP_S + 0.6 {
                (a.dip_age / (DUCK_DIP_S + 0.6)).clamp(0.0, 1.0)
            } else {
                -1.0
            };
            out.push(Ripple {
                pos: a.pos,
                heading: a.heading,
                wake: (a.speed / 0.6).clamp(0.0, 1.0),
                ring,
            });
        }
    }
}

impl AmbientAnimal {
    fn blank(kind: AmbientKind, pos: Vec2) -> Self {
        Self {
            kind,
            pos,
            heading: Vec2::NEG_Y,
            river: 0,
            s: 0.0,
            c: 0.0,
            duck: DuckMode::Rest { left: 0.0 },
            frog: FrogMode::Sit { spot: 0 },
            speed: 0.0,
            action: None,
            next_action: 0.0,
            idle_time: 0.0,
            swim_time: 0.0,
            swim_blend: 0.0,
            in_water: true,
            lift: 0.0,
            pad: None,
            actions_played: 0,
            distance_swum: 0.0,
            dip_age: f32::MAX,
        }
    }

    fn duck(
        kind: AmbientKind,
        river: usize,
        s: f32,
        c: f32,
        path: &RiverPath,
        rng: &mut Pcg32,
    ) -> Self {
        let mut a = Self::blank(kind, path.point(s, c));
        a.river = river;
        a.s = s;
        a.c = c;
        a.heading = path.dir_at(s);
        a.next_action = range(rng, ACTION_EVERY_S);
        a.idle_time = unit(rng) * 3.0;
        a.swim_time = unit(rng);
        a.duck = DuckMode::Rest {
            left: unit(rng) * 2.0,
        };
        a
    }

    /// River coordinates `(s, c)` of a duck.
    pub fn river_coords(&self) -> (f32, f32) {
        (self.s, self.c)
    }

    /// Whether a frog is sitting on a pad.
    pub fn on_pad(&self) -> bool {
        matches!(self.frog, FrogMode::Sit { .. }) && !self.in_water
    }

    /// Whether a duck is fleeing / calming down after the player came close.
    pub fn fleeing(&self) -> bool {
        matches!(self.duck, DuckMode::Flee { .. })
    }
}
