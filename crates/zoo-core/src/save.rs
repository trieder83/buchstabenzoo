//! Saving and restoring progress (GAME-SAVE): the whole game state as versioned JSON
//! (serde). The host only stores / loads the string (browser `localStorage`).
//!
//! Transient things (open text panel, one-shot animations, bubbles) are not saved. The camera
//! and the animals' facing live in the presentation layer (`zoo-web`), which fills
//! [`SaveState::camera`] and [`AnimalSave::yaw`].

use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

use crate::animals::AnimalState;
use crate::food::{Carry, Food, FoodBox};
use crate::game::{Game, GameError};
use crate::level::{cell_center, cell_of, LevelData};
use crate::rng::Pcg32;

/// Save format version (GAME-SAVE §2). Unknown versions start a new game (§5).
/// Version 2 (M5b): one save for the joined zoo (`level_id = "zoo"`), the fish bowl, the
/// second animal of a pair. Version-1 saves (level 1 only, M4b/M5a) are migrated: their
/// level-1 state is kept, the later levels start fresh (GAME-SAVE §5).
pub const SAVE_VERSION: u32 = 2;
/// Oldest version that is migrated.
pub const MIN_SAVE_VERSION: u32 = 1;
/// Upper size limit of a save (GAME-SAVE §2).
pub const MAX_SAVE_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveState {
    pub version: u32,
    pub level_id: String,
    pub seed: u64,
    /// PCG32 state and increment.
    pub rng: (u64, u64),
    pub play_time_s: f64,
    pub player: PlayerSave,
    /// Filled by the presentation layer (camera yaw step + zoom).
    #[serde(default)]
    pub camera: Option<CameraSave>,
    pub animals: Vec<AnimalSave>,
    pub missions: Vec<MissionSave>,
    pub open_barriers: Vec<String>,
    pub last_gate: Option<usize>,
    pub all_home: bool,
    /// The fish bowl (v2).
    #[serde(default)]
    pub bowl: Option<BowlSave>,
    /// Time of day, nightfalls done, barriers waiting for the morning (GAME-NIGHT §8,
    /// NIGHT-008). Missing in older saves = day.
    #[serde(default)]
    pub daytime: Option<crate::daytime::Daytime>,
    /// Garden plant states and the treat basket (GARD-008). Missing = a fresh garden.
    #[serde(default)]
    pub garden: Option<crate::garden::Garden>,
}

/// The fish bowl (GAME-RESCUE "goldfish bowl" 7, RESC-022).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BowlSave {
    pub id: String,
    pub pos: [f32; 2],
    pub lift_m: f32,
    pub carried: bool,
    pub water: bool,
    pub fish: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSave {
    pub pos: [f32; 2],
    pub facing: [f32; 2],
    pub carry: Option<String>,
    /// Blended ground speed (the surface the player stands on, PLAY-007).
    pub surface_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CameraSave {
    pub yaw_steps: i32,
    pub distance_m: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimalSave {
    pub id: String,
    /// Second animal of a pair = 1 (v2, GAME-FAMILY).
    #[serde(default)]
    pub member: u8,
    /// `escaped`, `following`, `in_enclosure`.
    pub state: String,
    pub pos: [f32; 2],
    pub hiding_place: String,
    pub waiting: bool,
    pub refusing: bool,
    /// Follow path cache (keeps restored games deterministic, SAVE-007).
    #[serde(default)]
    pub path: Vec<[i32; 2]>,
    #[serde(default)]
    pub path_target: Option<[i32; 2]>,
    /// Presentation yaw (radians), filled by the presentation layer.
    #[serde(default)]
    pub yaw: Option<f32>,
    /// Logic facing (ANIM-009).
    #[serde(default)]
    pub facing: Option<[f32; 2]>,
    /// Wandering (ANIM-011): pause left and remaining route cells.
    #[serde(default)]
    pub wander_pause_s: Option<f32>,
    #[serde(default)]
    pub wander_route: Vec<[i32; 2]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionSave {
    pub animal: String,
    /// Info board read (the mission started).
    pub started: bool,
    pub complete: bool,
    /// The celebration was shown (never replayed after a restore).
    pub celebrated: bool,
}

#[derive(Debug)]
pub enum SaveError {
    Json(String),
    Version(u32),
    OtherLevel(String),
    Game(GameError),
    Invalid(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Json(e) => write!(f, "save: broken JSON: {e}"),
            SaveError::Version(v) => write!(f, "save: unknown version {v}"),
            SaveError::OtherLevel(l) => write!(f, "save: other level {l}"),
            SaveError::Game(e) => write!(f, "save: {e}"),
            SaveError::Invalid(e) => write!(f, "save: {e}"),
        }
    }
}

impl std::error::Error for SaveError {}

fn state_id(s: AnimalState) -> &'static str {
    match s {
        AnimalState::Escaped => "escaped",
        AnimalState::Following => "following",
        AnimalState::InEnclosure => "in_enclosure",
        AnimalState::InBowl => "in_bowl",
    }
}

fn state_from_id(s: &str) -> Option<AnimalState> {
    match s {
        "escaped" => Some(AnimalState::Escaped),
        "following" => Some(AnimalState::Following),
        "in_enclosure" => Some(AnimalState::InEnclosure),
        "in_bowl" => Some(AnimalState::InBowl),
        _ => None,
    }
}

fn v2(a: [f32; 2]) -> Result<Vec2, SaveError> {
    let v = Vec2::from(a);
    if v.is_finite() {
        Ok(v)
    } else {
        Err(SaveError::Invalid("non-finite position".into()))
    }
}

impl SaveState {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("save state serialises")
    }

    pub fn from_json(json: &str) -> Result<Self, SaveError> {
        if json.len() > MAX_SAVE_BYTES {
            return Err(SaveError::Invalid("save too large".into()));
        }
        // check the version first, so a newer format with other fields reports the version
        #[derive(Deserialize)]
        struct Header {
            version: u32,
        }
        let h: Header = serde_json::from_str(json).map_err(|e| SaveError::Json(e.to_string()))?;
        if !(MIN_SAVE_VERSION..=SAVE_VERSION).contains(&h.version) {
            return Err(SaveError::Version(h.version));
        }
        serde_json::from_str(json).map_err(|e| SaveError::Json(e.to_string()))
    }
}

impl Game {
    /// Snapshot of the game state (GAME-SAVE "What is saved"); camera and animal yaw are
    /// left for the presentation layer.
    pub fn to_save(&self) -> SaveState {
        SaveState {
            version: SAVE_VERSION,
            level_id: self.level.data.level.id.clone(),
            seed: self.seed,
            rng: self.rng.to_parts(),
            play_time_s: self.time_s,
            player: PlayerSave {
                pos: self.player.pos.to_array(),
                facing: self.player.facing.to_array(),
                carry: self.carry.food().map(|f| f.id().to_owned()),
                surface_speed: self.player.surface_speed(),
            },
            camera: None,
            animals: self
                .animals
                .iter()
                .map(|a| AnimalSave {
                    id: a.id().to_owned(),
                    member: a.member,
                    state: state_id(a.state).to_owned(),
                    pos: a.pos.to_array(),
                    hiding_place: a.hiding_place.clone(),
                    waiting: a.waiting,
                    refusing: a.refusing,
                    path: a.path.iter().map(|c| c.to_array()).collect(),
                    path_target: a.path_target.map(|c| c.to_array()),
                    yaw: None,
                    facing: Some(a.facing.to_array()),
                    wander_pause_s: Some(a.wander.pause_s),
                    wander_route: a.wander.route.iter().map(|c| c.to_array()).collect(),
                })
                .collect(),
            missions: self
                .animals
                .iter()
                .zip(&self.missions)
                .map(|(a, m)| MissionSave {
                    animal: a.id().to_owned(),
                    started: m.started,
                    complete: m.complete,
                    celebrated: m.complete,
                })
                .collect(),
            open_barriers: self.level.open_barrier_ids(),
            last_gate: self.last_gate,
            all_home: self.all_home,
            bowl: self.bowl.as_ref().map(|b| BowlSave {
                id: b.id.clone(),
                pos: b.pos.to_array(),
                lift_m: b.lift_m,
                carried: b.carried,
                water: b.water,
                fish: b.fish,
            }),
            daytime: Some(self.daytime.clone()),
            garden: Some(self.garden.clone()),
        }
    }

    /// Restores a saved game on `data` (GAME-SAVE §4, §5). Unknown version, another level or
    /// broken data are errors (the host then starts a new game). Positions that are not
    /// walkable any more move to the nearest walkable cell; following animals that cannot
    /// stand where they were stand behind the player; animals at home stay in their
    /// enclosure. Restoring produces no events (the celebration is not replayed).
    pub fn from_save(data: LevelData, s: &SaveState) -> Result<Game, SaveError> {
        if !(MIN_SAVE_VERSION..=SAVE_VERSION).contains(&s.version) {
            return Err(SaveError::Version(s.version));
        }
        // v1 saves hold level 1 only: accepted by the joined zoo that contains level 1
        // (migration, GAME-SAVE §5); otherwise the ids must match
        let migrates =
            s.version < SAVE_VERSION && data.parts.first().is_some_and(|p| p.id == s.level_id);
        if s.level_id != data.level.id && !migrates {
            return Err(SaveError::OtherLevel(s.level_id.clone()));
        }
        let mut g = Game::new(data, s.seed).map_err(SaveError::Game)?;
        g.rng = Pcg32::from_parts(s.rng.0, s.rng.1);
        g.time_s = if s.play_time_s.is_finite() {
            s.play_time_s.max(0.0)
        } else {
            0.0
        };
        for b in &s.open_barriers {
            g.level.open_barrier(b);
        }
        g.all_home = s.all_home;
        g.last_gate = s.last_gate;

        // missions (by animal id)
        for m in &s.missions {
            if let Some(i) = g.animal_index(&m.animal) {
                g.missions[i].started = m.started;
                g.missions[i].complete = m.complete;
            }
        }
        // (a v1 save's `all_home` meant level 1 only)
        g.all_home = s.all_home && g.missions.iter().all(|m| m.complete);
        let leading = s.animals.iter().any(|a| a.state == "following");

        // player
        let pos = v2(s.player.pos)?;
        let facing = v2(s.player.facing)?;
        let facing = if (facing.length() - 1.0).abs() < 1e-3 {
            facing // keep the exact value (SAVE-001/007)
        } else {
            facing.normalize_or(Vec2::Y)
        };
        g.player.pos = nearest_walkable(&g, pos, leading);
        g.player.y = g.level.ground_height(g.player.pos);
        g.player.facing = facing;
        if s.player.surface_speed.is_finite() && s.player.surface_speed > 0.0 {
            g.player.set_surface_speed(s.player.surface_speed);
        }
        g.carry = Carry::default();
        if let Some(food) = s.player.carry.as_deref().and_then(Food::from_id) {
            g.carry.take(&FoodBox { food });
        }

        if let Some(garden) = &s.garden {
            g.garden.restore(garden);
        }
        if let (Some(bs), Some(b)) = (&s.bowl, &mut g.bowl) {
            if bs.id == b.id {
                b.pos = v2(bs.pos)?;
                b.lift_m = if bs.lift_m.is_finite() {
                    bs.lift_m
                } else {
                    0.0
                };
                b.carried = bs.carried;
                b.water = bs.water;
                b.fish = bs.fish;
            }
        }
        // animals (by id and pair member)
        for a in &s.animals {
            let Some(i) = g
                .animals
                .iter()
                .position(|x| x.id() == a.id && x.member == a.member)
            else {
                continue;
            };
            let state = state_from_id(&a.state)
                .ok_or_else(|| SaveError::Invalid(format!("animal state {}", a.state)))?;
            let pos = v2(a.pos)?;
            let enc = g.level.data.elements[g.animals[i].enclosure].rect;
            // the chosen place must still be a candidate of this animal (RESC-016)
            let place_ok = g
                .level
                .data
                .hiding_place(&a.hiding_place)
                .is_some_and(|h| h.animal == a.id);
            let pos = match state {
                AnimalState::InEnclosure => {
                    if enc.contains(cell_of(pos)) {
                        pos
                    } else {
                        cell_center(IVec2::new(enc.x + enc.w / 2, enc.z + enc.d / 2))
                    }
                }
                AnimalState::Following => {
                    if g.level.grid().is_walkable(cell_of(pos), true) {
                        pos
                    } else {
                        let behind =
                            g.player.pos - g.player.facing * g.follow_params.keep_distance_m;
                        nearest_walkable(&g, behind, true)
                    }
                }
                AnimalState::Escaped => pos,
                AnimalState::InBowl => g.player.pos,
            };
            let an = &mut g.animals[i];
            an.state = state;
            an.pos = pos;
            if place_ok {
                an.hiding_place = a.hiding_place.clone();
            }
            an.waiting = a.waiting;
            an.refusing = a.refusing;
            an.path = a.path.iter().map(|&c| IVec2::from(c)).collect();
            an.path_target = a.path_target.map(IVec2::from);
            if let Some(f) = a.facing.map(Vec2::from).filter(|f| f.is_finite()) {
                an.facing = f;
            }
            if let Some(p) = a.wander_pause_s.filter(|p| p.is_finite()) {
                an.wander.pause_s = p;
            }
            an.wander.route = a.wander_route.iter().map(|&c| IVec2::from(c)).collect();
        }
        // time of day (NIGHT-008): a save made while sleeping wakes up the next morning
        if let Some(d) = &s.daytime {
            let mut d = d.clone();
            d.phase_s = if d.phase_s.is_finite() {
                d.phase_s.max(0.0)
            } else {
                0.0
            };
            g.daytime = d;
            match g.daytime.phase {
                crate::daytime::Phase::Sleeping => {
                    g.daytime.force(crate::daytime::Phase::Morning);
                    g.wake(true);
                    g.daytime.force(crate::daytime::Phase::Day);
                }
                crate::daytime::Phase::Morning => g.daytime.force(crate::daytime::Phase::Day),
                crate::daytime::Phase::Night => {
                    for id in g.moon_doors() {
                        g.level.open_barrier(&id);
                    }
                }
                _ => {
                    for id in g.moon_doors() {
                        g.level.close_barrier(&id);
                    }
                }
            }
        }
        g.refresh_areas();
        // escaped animals stay inside their (possibly restored) place's wander area
        for an in &mut g.animals {
            if an.state == AnimalState::Escaped && !an.area.is_empty() {
                if !an.area.contains(cell_of(an.pos)) {
                    let spot = g
                        .level
                        .data
                        .hiding_place(&an.hiding_place)
                        .map_or(an.pos, |h| h.spot());
                    an.pos = spot;
                    an.wander.route.clear();
                }
                let area = an.area.clone();
                an.wander.route.retain(|c| area.contains(*c));
            }
        }
        g.drain_events();
        g.mark_saved();
        Ok(g)
    }

    /// [`Game::from_save`] from the JSON string of [`SaveState::to_json`].
    pub fn from_save_json(data: LevelData, json: &str) -> Result<Game, SaveError> {
        let s = SaveState::from_json(json)?;
        Game::from_save(data, &s)
    }
}

/// `pos` if its cell is walkable, else the centre of the nearest walkable cell (breadth-first
/// search over the grid, SAVE-006).
pub fn nearest_walkable(g: &Game, pos: Vec2, allow_gates: bool) -> Vec2 {
    let grid = g.level.grid();
    let start = cell_of(pos);
    if grid.is_walkable(start, allow_gates) {
        return pos;
    }
    let b = grid.bounds();
    let mut best: Option<(f32, IVec2)> = None;
    // search rings of growing radius; the first ring with a hit holds the nearest cell
    for r in 1..(b.w.max(b.d)) {
        for dz in -r..=r {
            for dx in -r..=r {
                if dx.abs() != r && dz.abs() != r {
                    continue;
                }
                let c = start + IVec2::new(dx, dz);
                if !b.contains(c) || !grid.is_walkable(c, allow_gates) {
                    continue;
                }
                let d = cell_center(c).distance(pos);
                if best.is_none_or(|(bd, _)| d < bd) {
                    best = Some((d, c));
                }
            }
        }
        if let Some((d, c)) = best {
            // a cell in the next ring could still be nearer only if d > r (cell size 1 m)
            if d <= r as f32 {
                return cell_center(c);
            }
        }
    }
    best.map_or(pos, |(_, c)| cell_center(c))
}
