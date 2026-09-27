//! Game state and the rescue mission flow (GAME-RESCUE, GAME-ANIMALS, GAME-FEED).
//!
//! Deterministic: all randomness comes from the seed, time only from `dt`.

use std::collections::BTreeMap;

use glam::{IVec2, Vec2};

use crate::animals::{animal_info, AnimalInfo, AnimalState};
use crate::content::{
    animal_more_key, animal_name_key, facts_key, riddle_key, Language, ReadingLevel,
};
use crate::daytime::Daytime;
use crate::food::{Carry, Food, FoodBox, FoodLabel, FoodStorage};
use crate::level::{
    cell_center, cell_of, facing_vec, CellKind, ElementType, Level, LevelData, Surface,
};
use crate::nav;
use crate::player::{in_interaction_range, MoveParams, Player};
use crate::rng::Pcg32;
use crate::scene::info_board_pose;
use crate::wander::{self, Wander, WanderArea};

/// Chosen hiding places of one animal set are at least this far apart (RESC-014, Q-082).
pub const MIN_PICK_SPREAD_M: f32 = 12.0;
/// Maximum number of seeded draws of the whole set before the fallback (Q-082).
pub const MAX_PICK_DRAWS: u32 = 64;

/// Follow behaviour (GAME-RESCUE §6).
#[derive(Debug, Clone, Copy)]
pub struct FollowParams {
    /// Distance kept behind the player ("a short distance", value not in the spec).
    pub keep_distance_m: f32,
    /// Following animals wait when the player is farther away than this.
    pub wait_beyond_m: f32,
    /// Waiting animals follow again when the player is back within this distance.
    pub resume_within_m: f32,
}

impl Default for FollowParams {
    fn default() -> Self {
        Self {
            keep_distance_m: 1.5,
            wait_beyond_m: 15.0,
            resume_within_m: 5.0,
        }
    }
}

/// Events for presentation (animation, audio, UI). Drained with [`Game::drain_events`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    MissionStarted {
        animal: String,
    },
    /// Player took food; `returned` is the food put back into its box (FEED-003).
    FoodTaken {
        food: Food,
        returned: Option<Food>,
    },
    NotInterested {
        animal: String,
    },
    StartedFollowing {
        animal: String,
    },
    Waiting {
        animal: String,
    },
    FollowingAgain {
        animal: String,
    },
    /// Animals stop at a wrong enclosure's gate (RESC-007).
    Refuse {
        animal: String,
        enclosure: String,
    },
    FoodConsumed {
        food: Food,
    },
    /// Animals entered their own enclosure: play `eat`, then `happy`; gate closes.
    InEnclosure {
        animal: String,
    },
    MissionComplete {
        animal: String,
    },
    AllAnimalsHome,
    BarrierOpened {
        id: String,
    },
    /// Every mission of a level is complete (its exit barriers open the next morning, Q-091).
    LevelComplete {
        level: String,
    },
    /// Dusk starts after the celebration of a day level's last mission (NIGHT-001).
    DuskStarted,
    /// Full night: the moon door is open, the bed is usable (NIGHT-002).
    NightFell,
    /// The child went to bed (NIGHT-003); the game is saved.
    SleepStarted,
    /// The next morning: pending barriers opened (Q-091), autosave.
    Morning,
    /// Full daylight again.
    DayStarted,
    /// The player went through the open moon door (`into_night_zoo`: towards the night zoo).
    MoonDoor {
        into_night_zoo: bool,
    },
    /// The player picked up a carryable item (the fish bowl).
    ItemTaken {
        id: String,
    },
    /// The player put a carried item down.
    ItemPutDown {
        id: String,
    },
    /// The fish bowl was filled at a water source (RESC-019).
    ContainerFilled {
        id: String,
        animal: String,
    },
    /// Right food, but the animal needs a container the player does not carry (RESC-018).
    NeedsContainer {
        animal: String,
    },
    /// Right food and the container is carried, but it holds no water (RESC-019).
    ContainerEmpty {
        animal: String,
    },
    /// The animal jumped into the carried container (RESC-019).
    InContainer {
        animal: String,
    },
    /// The reading panel of an info board / food box opened by itself (GAME-PLAYER §4).
    PanelOpened {
        target: Target,
    },
    /// The reading panel closed by itself (target no longer available).
    PanelClosed {
        target: Target,
    },
}

/// Reading panels open this long after their target became available (GAME-PLAYER §4).
pub const PANEL_SETTLE_S: f32 = 0.25;
/// An open panel closes this long after its target stopped being available.
pub const PANEL_CLOSE_S: f32 = 0.3;
/// Range hysteresis: an open panel stays while the player is within this distance.
pub const PANEL_KEEP_RANGE_M: f32 = 2.5;

/// Automatic open/close of the reading panel (GAME-PLAYER §4, PLAY-023…027).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReadingPanel {
    /// Target whose panel is open.
    pub open: Option<Target>,
    /// Reading target that is available but still settling, and for how long.
    candidate: Option<(Target, f32)>,
    /// How long the open target has not been available.
    lost_s: f32,
    /// Closed by hand: not reopened until the player left its range (> 2.5 m).
    suppressed: Option<Target>,
}

impl ReadingPanel {
    /// Target closed by hand and not yet left.
    pub fn suppressed(&self) -> Option<&Target> {
        self.suppressed.as_ref()
    }
}

/// Readable side tolerance (GAME-PLAYER §5).
pub const READABLE_SIDE_DEG: f32 = 60.0;
/// Player facing tolerance (GAME-PLAYER §5).
pub const FACING_DEG: f32 = 75.0;

fn angle_deg(a: Vec2, b: Vec2) -> f32 {
    let (a, b) = (a.normalize_or_zero(), b.normalize_or_zero());
    a.dot(b).clamp(-1.0, 1.0).acos().to_degrees()
}

/// Something the player can interact with (GAME-PLAYER §4/§5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    InfoBoard {
        animal: &'static str,
    },
    FoodBox {
        food: Food,
    },
    Animal {
        animal: &'static str,
    },
    /// Enclosure gate (only while leading animals or carrying an animal in its container).
    Gate {
        enclosure: String,
    },
    /// A carryable item lying somewhere (the fish bowl), to pick up.
    Item {
        id: String,
    },
    /// A water source (tap, bank) to fill the carried container.
    Water {
        source: String,
    },
    /// Put the carried item down here (lowest priority, only when nothing else is available).
    PutDown,
    /// The bed in the zookeeper house (only at night, GAME-NIGHT rule 3).
    Bed,
    /// The open moon door (only at night): goes through it (GAME-NIGHT rule 3).
    MoonDoor {
        id: String,
    },
}

impl Target {
    /// Targets with a reading panel (info boards, food boxes) open it by themselves.
    pub fn is_reading(&self) -> bool {
        matches!(self, Target::InfoBoard { .. } | Target::FoodBox { .. })
    }

    /// Short id for the host UI (button icon).
    pub fn kind(&self) -> &'static str {
        match self {
            Target::InfoBoard { .. } => "info_board",
            Target::FoodBox { .. } => "food_box",
            Target::Animal { .. } => "animal",
            Target::Gate { .. } => "gate",
            Target::Item { .. } => "item",
            Target::Water { .. } => "water",
            Target::PutDown => "put_down",
            Target::Bed => "bed",
            Target::MoonDoor { .. } => "moon_door",
        }
    }
}

/// An interactable with its interaction point (level) and readable-side direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Interactable {
    pub target: Target,
    pub point: Vec2,
    pub readable: Option<Vec2>,
}

/// Result of [`Game::interact`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interaction {
    /// Open the text panel with the riddle (starts the mission, RESC-012).
    InfoBoard(InfoBoard),
    /// Open the text panel with the box label and a take button (GAME-FEED §6).
    FoodBox { food: Food, label: FoodLabel },
    /// Food shown to an animal; `accepted` = it follows now.
    ShowFood {
        animal: &'static str,
        accepted: bool,
        carried: Option<Food>,
    },
    /// Leading animals into an enclosure gate.
    Gate { enclosure: String, entered: bool },
    /// Picked up / put down / filled a carryable item.
    Item { id: String, action: &'static str },
    /// Went to bed (GAME-NIGHT rule 3, NIGHT-003).
    Sleep,
    /// Went through the moon door.
    MoonDoor { into_night_zoo: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractError {
    UnknownTarget,
    OutOfRange,
}

/// One animal group (herd size Q-030: modelled as one unit).
#[derive(Debug, Clone)]
pub struct Animal {
    pub info: &'static AnimalInfo,
    pub state: AnimalState,
    pub pos: Vec2,
    /// Facing (unit, level coordinates): where it walks, or the player it looks at (ANIM-009).
    pub facing: Vec2,
    /// Wandering at its hiding place or at home (ANIM-008…011).
    pub wander: Wander,
    /// Cells it may wander on in its current state (derived from the level, not saved).
    pub(crate) area: WanderArea,
    /// Id of the hiding place chosen for this playthrough.
    pub hiding_place: String,
    /// Index of its enclosure element.
    pub enclosure: usize,
    /// `following` but waiting for the player (> 15 m, RESC-006).
    pub waiting: bool,
    /// Stopped at a wrong enclosure's gate (RESC-007).
    pub refusing: bool,
    pub(crate) path: Vec<IVec2>,
    pub(crate) path_target: Option<IVec2>,
    /// 0, or 1 for the second animal of a pair (GAME-FAMILY).
    pub member: u8,
    /// Level part of its enclosure (missions in scope = unlocked levels).
    pub part: usize,
}

impl Animal {
    pub fn id(&self) -> &'static str {
        self.info.id
    }

    /// Whether the animal is walking (wandering) right now.
    pub fn is_wandering(&self) -> bool {
        !self.wander.route.is_empty()
    }

    /// Its wander area (hiding place while escaped, enclosure when home).
    pub fn wander_area(&self) -> &WanderArea {
        &self.area
    }
}

/// Picks one candidate hiding place per animal (GAME-RESCUE §1, Q-082): uniform seeded pick
/// per animal (in the given order), the whole set redrawn while two chosen spots are
/// < [`MIN_PICK_SPREAD_M`] apart (at most [`MAX_PICK_DRAWS`] draws, then the first valid
/// combination in data order). `avoid` maps an animal to the place of the previous game,
/// which is not picked again when the animal has another candidate.
pub fn pick_hiding_places(
    data: &LevelData,
    animals: &[&str],
    rng: &mut Pcg32,
    avoid: &BTreeMap<String, String>,
) -> Result<Vec<String>, GameError> {
    let mut cands: Vec<Vec<&crate::level::HidingPlaceData>> = Vec::new();
    for &a in animals {
        let all: Vec<_> = data.hiding_places_of(a).collect();
        if all.is_empty() {
            return Err(GameError::NoHidingPlace(a.to_owned()));
        }
        let fresh: Vec<_> = all
            .iter()
            .copied()
            .filter(|h| avoid.get(a) != Some(&h.id))
            .collect();
        cands.push(if fresh.is_empty() { all } else { fresh });
    }
    let valid = |set: &[&crate::level::HidingPlaceData]| {
        set.iter().enumerate().all(|(i, a)| {
            set[i + 1..]
                .iter()
                .all(|b| a.id != b.id && a.spot().distance(b.spot()) >= MIN_PICK_SPREAD_M - 1e-4)
        })
    };
    for _ in 0..MAX_PICK_DRAWS {
        let set: Vec<_> = cands
            .iter()
            .map(|c| c[rng.below(c.len() as u32) as usize])
            .collect();
        if valid(&set) {
            return Ok(set.iter().map(|h| h.id.clone()).collect());
        }
    }
    // fallback: first valid combination in data order (odometer over the candidate lists)
    let mut idx = vec![0usize; cands.len()];
    loop {
        let set: Vec<_> = cands.iter().zip(&idx).map(|(c, &i)| c[i]).collect();
        if valid(&set) {
            return Ok(set.iter().map(|h| h.id.clone()).collect());
        }
        let mut k = 0;
        loop {
            if k == idx.len() {
                // no valid combination at all: the first one
                return Ok(cands.iter().map(|c| c[0].id.clone()).collect());
            }
            idx[k] += 1;
            if idx[k] < cands[k].len() {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mission {
    pub started: bool,
    pub complete: bool,
}

/// Player settings relevant for content.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub reading_level: ReadingLevel,
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            reading_level: ReadingLevel::Klasse1,
            language: Language::De,
        }
    }
}

/// Content of an info board (GAME-ANIMALS "Info board") as Fluent keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoBoard {
    pub animal: &'static str,
    /// Heading: the animal's name (`animal-<animal>`).
    pub name_key: String,
    /// "More about the animal" heading above the facts (`animal-<animal>-more`).
    pub more_key: String,
    /// Facts about the animal (`mission-<animal>-facts-<level>`, ANIM-006); never names the
    /// hiding place (ANIM-007). Shown after the riddle and the food word.
    pub facts_key: String,
    pub riddle_key: String,
    /// `kiga`: a picture of the hiding place (its id) next to the one-word riddle.
    pub picture: Option<String>,
    /// The food word — the same key as the matching food box label (ANIM-003).
    pub food_key: String,
}

#[derive(Debug)]
pub enum GameError {
    UnknownAnimal(String),
    NoHidingPlace(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::UnknownAnimal(a) => write!(f, "unknown animal {a}"),
            GameError::NoHidingPlace(a) => write!(f, "no hiding place for {a} in the level"),
        }
    }
}

impl std::error::Error for GameError {}

pub struct Game {
    pub level: Level,
    pub player: Player,
    pub carry: Carry,
    pub storage: FoodStorage,
    /// Food box props from the level data (GAME-FEED §7): food, centre, label facing.
    pub food_boxes: Vec<(Food, Vec2, Vec2)>,
    pub animals: Vec<Animal>,
    pub missions: Vec<Mission>,
    pub settings: Settings,
    pub move_params: MoveParams,
    pub follow_params: FollowParams,
    pub(crate) events: Vec<GameEvent>,
    /// Gate cell the player stood on in the last update (for once-per-entry events).
    pub(crate) last_gate: Option<usize>,
    pub(crate) all_home: bool,
    /// Automatic reading panel (GAME-PLAYER §4).
    pub panel: ReadingPanel,
    /// Seed of this playthrough and the RNG after the setup (GAME-SAVE).
    pub(crate) seed: u64,
    pub(crate) rng: Pcg32,
    /// Play time in seconds (sum of `update` steps).
    pub time_s: f64,
    /// Autosave bookkeeping (GAME-SAVE §3).
    pub(crate) autosave: Autosave,
    /// The fish bowl (`[[item]]` of kind `fish_bowl`, GAME-RESCUE "goldfish bowl"), if the
    /// joined levels have one.
    pub bowl: Option<Bowl>,
    /// Time of day (GAME-NIGHT).
    pub daytime: Daytime,
}

/// Walking speed factor while carrying an animal in its container (proposal Q-084, RESC-023).
pub const CARRY_ANIMAL_SPEED_FACTOR: f32 = 0.9;
/// Height of a table top an item stands on inside a building (presentation).
pub const TABLE_HEIGHT_M: f32 = 0.78;

/// A carryable container for an animal that cannot walk (the fish bowl, GAME-RESCUE
/// "goldfish bowl", proposal Q-093/Q-084).
#[derive(Debug, Clone, PartialEq)]
pub struct Bowl {
    /// Item id (`fish_bowl`).
    pub id: String,
    /// The animal that travels in it.
    pub animal: String,
    /// Where it stands when not carried (level coordinates).
    pub pos: Vec2,
    /// Height of what it stands on (table / step), presentation only.
    pub lift_m: f32,
    pub carried: bool,
    /// Filled with water (RESC-019).
    pub water: bool,
    /// The animal is inside (RESC-019…022).
    pub fish: bool,
}

/// Seed of the discovery RNG of level part `k ≥ 1` (level 1 keeps the main RNG, so its picks
/// and wandering do not change when later levels are joined).
fn part_seed(seed: u64, k: usize) -> u64 {
    seed ^ (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

/// Autosave rules (GAME-SAVE §3): after every progress event and every 5 s of moving.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Autosave {
    /// A progress event happened since the last save.
    pub(crate) progress: bool,
    /// Seconds the player moved since the last save.
    pub(crate) moving_s: f32,
}

/// Autosave interval while the player moves (GAME-SAVE §3).
pub const AUTOSAVE_MOVING_S: f32 = 5.0;

impl GameEvent {
    /// Progress events trigger an autosave (GAME-SAVE §3).
    pub fn is_progress(&self) -> bool {
        matches!(
            self,
            GameEvent::MissionStarted { .. }
                | GameEvent::FoodTaken { .. }
                | GameEvent::StartedFollowing { .. }
                | GameEvent::InEnclosure { .. }
                | GameEvent::MissionComplete { .. }
                | GameEvent::AllAnimalsHome
                | GameEvent::BarrierOpened { .. }
                | GameEvent::LevelComplete { .. }
                | GameEvent::ItemTaken { .. }
                | GameEvent::ItemPutDown { .. }
                | GameEvent::ContainerFilled { .. }
                | GameEvent::InContainer { .. }
                | GameEvent::NightFell
                | GameEvent::SleepStarted
                | GameEvent::Morning
                | GameEvent::MoonDoor { .. }
        )
    }
}

impl Game {
    /// New game: every enclosure empty, every animal at a hiding place picked with the seeded
    /// RNG (GAME-RESCUE §1). One animal per enclosure in the level data (only the missions in
    /// scope, `[level] missions`, Q-069).
    pub fn new(data: LevelData, seed: u64) -> Result<Self, GameError> {
        Self::new_avoiding(data, seed, &BTreeMap::new())
    }

    /// [`Game::new`] that avoids each animal's hiding place of the previous game (`avoid`:
    /// animal id → place id, Q-082).
    pub fn new_avoiding(
        data: LevelData,
        seed: u64,
        avoid: &BTreeMap<String, String>,
    ) -> Result<Self, GameError> {
        let mut rng = Pcg32::new(seed);
        let mut part_rngs: Vec<Pcg32> = (0..data.parts.len())
            .map(|k| Pcg32::new(part_seed(seed, k)))
            .collect();
        let mut animals = Vec::new();
        for (k, part) in data.parts.iter().enumerate() {
            let mut enclosures = Vec::new();
            for (i, enc) in data.elements.iter().enumerate() {
                if enc.ty != ElementType::Enclosure || enc.part != k {
                    continue;
                }
                let id = enc.animal.clone().unwrap_or_default();
                if !part.missions.contains(&id) {
                    continue; // not in scope: scenery only (Q-069)
                }
                let info = animal_info(&id).ok_or_else(|| GameError::UnknownAnimal(id.clone()))?;
                enclosures.push((i, info, enc.pair));
            }
            let ids: Vec<&str> = enclosures.iter().map(|(_, info, _)| info.id).collect();
            let r = if k == 0 { &mut rng } else { &mut part_rngs[k] };
            let picks = pick_hiding_places(&data, &ids, r, avoid)?;
            for ((i, info, pair), place_id) in enclosures.into_iter().zip(picks) {
                let place = data.hiding_place(&place_id).expect("picked from the data");
                for member in 0..if pair { 2 } else { 1 } {
                    animals.push(Animal {
                        info,
                        state: AnimalState::Escaped,
                        pos: place.spot(),
                        facing: Vec2::NEG_Y,
                        wander: Wander::default(),
                        area: WanderArea::default(),
                        hiding_place: place_id.clone(),
                        enclosure: i,
                        waiting: false,
                        refusing: false,
                        path: Vec::new(),
                        path_target: None,
                        member,
                        part: k,
                    });
                }
            }
        }
        let move_params = MoveParams::default();
        let facing = facing_vec(&data.spawn.facing);
        let food_boxes = data
            .food_boxes
            .iter()
            .filter_map(|b| Food::from_id(&b.food).map(|f| (f, b.pos(), b.facing())))
            .collect();
        let player = Player::new(cell_center(data.spawn.cell()), facing, &move_params);
        let missions = vec![Mission::default(); animals.len()];
        let bowl = data
            .items
            .iter()
            .find(|it| it.kind == "fish_bowl")
            .map(|it| Bowl {
                id: it.id.clone(),
                animal: it.animal.clone().unwrap_or_default(),
                pos: it.pos(),
                lift_m: if it.building.is_some() {
                    TABLE_HEIGHT_M
                } else {
                    0.0
                },
                carried: false,
                water: false,
                fish: false,
            });
        let level = Level::new(data);
        for a in &mut animals {
            a.area = wander_area_of(&level, a);
            a.facing = rest_facing(&level, a);
            // the second animal of a pair waits one wander cell away (GAME-FAMILY §1)
            if a.member > 0 {
                if let Some(c) = a
                    .area
                    .cells()
                    .map(|(c, _)| c)
                    .filter(|&c| c != cell_of(a.pos))
                    .min_by(|x, y| {
                        cell_center(*x)
                            .distance(a.pos)
                            .total_cmp(&cell_center(*y).distance(a.pos))
                    })
                {
                    a.pos = cell_center(c);
                }
            }
            let r = if a.part == 0 {
                &mut rng
            } else {
                &mut part_rngs[a.part]
            };
            a.wander.pause_s = wander::draw_pause(r);
        }
        Ok(Self {
            level,
            player,
            carry: Carry::default(),
            storage: FoodStorage::default(),
            food_boxes,
            animals,
            missions,
            settings: Settings::default(),
            move_params,
            follow_params: FollowParams::default(),
            events: Vec::new(),
            last_gate: None,
            all_home: false,
            panel: ReadingPanel::default(),
            seed,
            rng,
            time_s: 0.0,
            autosave: Autosave::default(),
            bowl,
            daytime: Daytime::default(),
        })
    }

    /// Whether a level part is unlocked: the first level always, a later one when one of its
    /// entry barriers is open (GAME-LAYOUT "Joining levels"; missions in scope = union of the
    /// unlocked levels' missions).
    pub fn part_unlocked(&self, k: usize) -> bool {
        k == 0
            || self.level.data.parts.get(k).is_some_and(|p| {
                p.entries
                    .iter()
                    .any(|e| self.level.is_barrier_open(&e.barrier))
            })
    }

    /// Whether a level (by id) is unlocked.
    pub fn level_unlocked(&self, id: &str) -> bool {
        self.level
            .data
            .part_index(id)
            .is_some_and(|k| self.part_unlocked(k))
    }

    /// Whether the animal's mission is in scope (its level is unlocked).
    pub fn in_scope(&self, a: &Animal) -> bool {
        self.part_unlocked(a.part)
    }

    /// Indices of the animals of one species (both animals of a pair, GAME-FAMILY).
    pub fn group(&self, animal: &str) -> Vec<usize> {
        (0..self.animals.len())
            .filter(|&i| self.animals[i].id() == animal)
            .collect()
    }

    /// Whether the player carries the bowl with its animal inside (RESC-023).
    pub fn carrying_animal(&self) -> bool {
        self.bowl.as_ref().is_some_and(|b| b.carried && b.fish)
    }

    /// Whether an animal needs a container to be carried home (an `[[item]]` names it).
    pub fn needs_container(&self, animal: &str) -> bool {
        self.bowl.as_ref().is_some_and(|b| b.animal == animal)
    }

    /// Perch of an escaped animal (proposal Q-094): the point it sits at (level) and its
    /// height above the ground, if its hiding place has `perch_height_m`.
    pub fn perch(&self, a: &Animal) -> Option<(Vec2, f32)> {
        if a.state != AnimalState::Escaped {
            return None;
        }
        let place = self.level.data.hiding_place(&a.hiding_place)?;
        let h = place.perch_height_m?;
        Some((crate::scene::perch_point(place, &self.level.data), h))
    }

    /// Takes the events since the last call. Progress events among them make an autosave due
    /// (GAME-SAVE §3; the host drains after every update / interaction).
    pub fn drain_events(&mut self) -> Vec<GameEvent> {
        let events = std::mem::take(&mut self.events);
        if events.iter().any(GameEvent::is_progress) {
            self.autosave.progress = true;
        }
        events
    }

    /// Whether an autosave is due: a progress event happened, or the player moved for
    /// [`AUTOSAVE_MOVING_S`] since the last save (GAME-SAVE §3, SAVE-009).
    pub fn save_due(&self) -> bool {
        self.autosave.progress || self.autosave.moving_s >= AUTOSAVE_MOVING_S
    }

    /// Resets the autosave rules after the host stored a save.
    pub fn mark_saved(&mut self) {
        self.autosave = Autosave::default();
    }

    pub fn animal_index(&self, id: &str) -> Option<usize> {
        self.animals.iter().position(|a| a.id() == id)
    }

    pub fn animal(&self, id: &str) -> Option<&Animal> {
        self.animals.iter().find(|a| a.id() == id)
    }

    pub fn mission(&self, animal: &str) -> Option<Mission> {
        self.animal_index(animal).map(|i| self.missions[i])
    }

    /// Whether the player currently leads an animal group (gates passable).
    pub fn is_leading(&self) -> bool {
        self.animals
            .iter()
            .any(|a| a.state == AnimalState::Following)
    }

    /// Animals inside the given enclosure element.
    pub fn enclosure_occupants(&self, enclosure_id: &str) -> Vec<&'static str> {
        self.animals
            .iter()
            .filter(|a| {
                a.state == AnimalState::InEnclosure
                    && self.level.data.elements[a.enclosure].id == enclosure_id
            })
            .map(|a| a.id())
            .collect()
    }

    /// Info board of an animal's enclosure at the current reading level (ANIM-005).
    pub fn info_board(&self, animal: &str) -> Option<InfoBoard> {
        let a = self.animal(animal)?;
        let level = self.settings.reading_level;
        Some(InfoBoard {
            animal: a.id(),
            name_key: animal_name_key(a.id()),
            more_key: animal_more_key(a.id()),
            facts_key: facts_key(a.id(), level),
            riddle_key: riddle_key(a.id(), &a.hiding_place, level),
            picture: (level == ReadingLevel::Kiga).then(|| a.hiding_place.clone()),
            food_key: a.info.foods[0].label_key(),
        })
    }

    /// Distance from the player to the info board of an animal's enclosure.
    fn board_distance(&self, animal: &str) -> Option<f32> {
        let a = self.animal(animal)?;
        let enc_id = &self.level.data.elements[a.enclosure].id;
        self.level
            .data
            .elements
            .iter()
            .filter(|e| {
                e.kind.as_deref() == Some("info_board") && e.enclosure.as_deref() == Some(enc_id)
            })
            .map(|e| e.rect.distance_to(self.player.pos))
            .reduce(f32::min)
    }

    /// The player reads the info board of an animal's enclosure (must be within 2 m). Starts
    /// the mission on first read (RESC-012).
    pub fn read_info_board(&mut self, animal: &str) -> Result<InfoBoard, InteractError> {
        let d = self
            .board_distance(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !in_interaction_range(d) {
            return Err(InteractError::OutOfRange);
        }
        let i = self
            .animal_index(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !self.in_scope(&self.animals[i]) {
            return Err(InteractError::UnknownTarget);
        }
        if !self.missions[i].started {
            for j in self.group(animal) {
                self.missions[j].started = true;
            }
            self.events.push(GameEvent::MissionStarted {
                animal: animal.to_owned(),
            });
        }
        self.info_board(animal).ok_or(InteractError::UnknownTarget)
    }

    /// Distance from the player to the food storage door (boxes are taken at the door; the
    /// storage interior is not in the layout data yet).
    pub fn storage_distance(&self) -> Option<f32> {
        self.level
            .data
            .elements
            .iter()
            .filter(|e| e.kind.as_deref() == Some("food_storage"))
            .filter_map(|e| e.door_cell())
            .map(|d| crate::level::Rect::new(d.x, d.y, 1, 1).distance_to(self.player.pos))
            .reduce(f32::min)
    }

    /// Food box labels at the current reading level.
    pub fn food_labels(&self) -> Vec<(Food, FoodLabel)> {
        self.storage
            .boxes
            .iter()
            .map(|b| (b.food, b.label(self.settings.reading_level)))
            .collect()
    }

    /// Takes food from a box in the storage (GAME-FEED §3).
    pub fn take_food(&mut self, food: Food) -> Result<(), InteractError> {
        // With food box props the player must stand at that box; otherwise at the door.
        let d = if self.food_boxes.is_empty() {
            self.storage_distance()
        } else {
            // every level has its own storage with all boxes (Q-089): the nearest box
            self.food_boxes
                .iter()
                .filter(|b| b.0 == food)
                .map(|b| b.1.distance(self.player.pos))
                .reduce(f32::min)
        }
        .ok_or(InteractError::UnknownTarget)?;
        if !in_interaction_range(d) {
            return Err(InteractError::OutOfRange);
        }
        let b: FoodBox = *self
            .storage
            .boxes
            .iter()
            .find(|b| b.food == food)
            .ok_or(InteractError::UnknownTarget)?;
        let returned = self.carry.take(&b);
        self.events.push(GameEvent::FoodTaken { food, returned });
        Ok(())
    }

    /// Shows the carried food to an animal group within 2 m (GAME-RESCUE §5, GAME-FEED §4).
    /// Both animals of a pair follow (FAM-002). An animal that needs a container (the goldfish)
    /// jumps into the carried, filled bowl instead (RESC-018/019).
    pub fn show_food(&mut self, animal: &str) -> Result<(), InteractError> {
        let i = self
            .animal_index(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !self.in_scope(&self.animals[i]) {
            return Err(InteractError::UnknownTarget);
        }
        let near = self
            .group(animal)
            .into_iter()
            .map(|j| self.animals[j].pos.distance(self.player.pos))
            .fold(f32::MAX, f32::min);
        if !in_interaction_range(near) {
            return Err(InteractError::OutOfRange);
        }
        if self.animals[i].state != AnimalState::Escaped {
            return Ok(()); // following or home: nothing changes (ANIM-002)
        }
        // Only one group follows at a time (§5); a second group is not interested (Q-041 proposal).
        let other_following = self.is_leading() || self.carrying_animal();
        let right_food = self
            .carry
            .food()
            .is_some_and(|f| self.animals[i].info.eats(f));
        let id = animal.to_owned();
        if right_food && self.needs_container(animal) {
            let (carried, water) = self
                .bowl
                .as_ref()
                .map_or((false, false), |b| (b.carried, b.water));
            if !carried || other_following {
                self.events.push(GameEvent::NeedsContainer { animal: id });
            } else if !water {
                self.events.push(GameEvent::ContainerEmpty { animal: id });
            } else {
                // fed from the bank: it jumps into the bowl, the food is used up
                if let Some(food) = self.carry.consume() {
                    self.events.push(GameEvent::FoodConsumed { food });
                }
                for j in self.group(animal) {
                    let a = &mut self.animals[j];
                    a.state = AnimalState::InBowl;
                    a.wander = Wander::default();
                    a.area = WanderArea::default();
                    a.pos = self.player.pos;
                }
                if let Some(b) = &mut self.bowl {
                    b.fish = true;
                }
                self.events.push(GameEvent::InContainer { animal: id });
            }
            return Ok(());
        }
        if right_food && !other_following {
            for j in self.group(animal) {
                let a = &mut self.animals[j];
                if a.state != AnimalState::Escaped {
                    continue;
                }
                a.state = AnimalState::Following;
                a.waiting = false;
                a.path.clear();
                a.path_target = None;
                a.wander = Wander::default();
            }
            self.events.push(GameEvent::StartedFollowing { animal: id });
        } else {
            self.events.push(GameEvent::NotInterested { animal: id });
        }
        Ok(())
    }

    /// Picks up the bowl (within 2 m, not carried).
    pub fn take_item(&mut self, id: &str) -> Result<(), InteractError> {
        let p = self.player.pos;
        let b = self
            .bowl
            .as_mut()
            .filter(|b| b.id == id)
            .ok_or(InteractError::UnknownTarget)?;
        if b.carried {
            return Ok(());
        }
        if !in_interaction_range(b.pos.distance(p)) {
            return Err(InteractError::OutOfRange);
        }
        b.carried = true;
        b.lift_m = 0.0;
        self.events.push(GameEvent::ItemTaken { id: id.to_owned() });
        Ok(())
    }

    /// Puts the carried bowl down in front of the player (GAME-RESCUE "goldfish bowl" 7: the
    /// fish stays safe in it).
    pub fn put_down_item(&mut self) -> bool {
        let at = self.player.pos + self.player.facing * 0.5;
        let Some(b) = self.bowl.as_mut().filter(|b| b.carried) else {
            return false;
        };
        b.carried = false;
        b.pos = at;
        b.lift_m = 0.0;
        let id = b.id.clone();
        self.events.push(GameEvent::ItemPutDown { id });
        true
    }

    /// Fills the carried bowl at a water source within range (RESC-019).
    pub fn fill_container(&mut self) -> Result<(), InteractError> {
        let (source, point) = self.water_point().ok_or(InteractError::OutOfRange)?;
        let _ = source;
        if !in_interaction_range(point.distance(self.player.pos)) {
            return Err(InteractError::OutOfRange);
        }
        let b = self
            .bowl
            .as_mut()
            .filter(|b| b.carried)
            .ok_or(InteractError::UnknownTarget)?;
        if !b.water {
            b.water = true;
            self.events.push(GameEvent::ContainerFilled {
                id: b.id.clone(),
                animal: b.animal.clone(),
            });
        }
        Ok(())
    }

    /// The nearest point of a water source in the unlocked levels (proposal Q-093): a tap
    /// (`[[water_source]] kind = "tap"`) or the edge of a water landmark (`[[water_source]]
    /// kind = "bank"`, and every river, pond, stream and fountain of the unlocked map).
    pub fn water_point(&self) -> Option<(String, Vec2)> {
        let data = &self.level.data;
        let p = self.player.pos;
        let mut best: Option<(String, Vec2)> = None;
        let mut offer = |id: &str, q: Vec2| {
            if best
                .as_ref()
                .is_none_or(|(_, b)| q.distance(p) < b.distance(p))
            {
                best = Some((id.to_owned(), q));
            }
        };
        for w in &data.water_sources {
            if !self.part_unlocked(w.part) {
                continue;
            }
            if let (Some(pos), "tap") = (w.pos, w.kind.as_str()) {
                offer(&w.id, Vec2::from(pos));
            }
        }
        for e in &data.elements {
            let water = e.ty == ElementType::Landmark
                && e.kind
                    .as_deref()
                    .is_some_and(|k| crate::level::WATER_KINDS.contains(&k));
            if !water || !self.part_unlocked(e.part) {
                continue;
            }
            let r = e.rect;
            let min = Vec2::new(r.x as f32, r.z as f32);
            let q = p.clamp(min, min + Vec2::new(r.w as f32, r.d as f32));
            let id = data
                .water_sources
                .iter()
                .find(|w| w.water.as_deref() == Some(e.id.as_str()))
                .map_or(e.id.as_str(), |w| w.id.as_str());
            offer(id, q);
        }
        best
    }

    /// Everything the player can interact with right now, with interaction point and — for
    /// boards and boxes — the readable side (GAME-PLAYER §5).
    pub fn interactables(&self) -> Vec<Interactable> {
        let mut out = Vec::new();
        let data = &self.level.data;
        for e in &data.elements {
            if e.kind.as_deref() != Some("info_board") {
                continue;
            }
            let Some(enc) = e.enclosure.as_deref() else {
                continue;
            };
            let Some(a) = self
                .animals
                .iter()
                .find(|a| data.elements[a.enclosure].id == enc && self.in_scope(a))
            else {
                continue;
            };
            let (point, dir) = info_board_pose(e, data);
            out.push(Interactable {
                target: Target::InfoBoard { animal: a.id() },
                point,
                readable: Some(dir.offset().as_vec2()),
            });
        }
        for &(food, point, facing) in &self.food_boxes {
            out.push(Interactable {
                target: Target::FoodBox { food },
                point,
                readable: Some(facing),
            });
        }
        for a in &self.animals {
            if a.state == AnimalState::Escaped && self.in_scope(a) {
                out.push(Interactable {
                    target: Target::Animal { animal: a.id() },
                    point: a.pos,
                    readable: None,
                });
            }
        }
        if let Some(b) = &self.bowl {
            if !b.carried && self.part_unlocked(data.part_at(cell_of(b.pos)).unwrap_or(0)) {
                out.push(Interactable {
                    target: Target::Item { id: b.id.clone() },
                    point: b.pos,
                    readable: None,
                });
            }
            if b.carried && !b.water {
                if let Some((source, point)) = self.water_point() {
                    out.push(Interactable {
                        target: Target::Water { source },
                        point,
                        readable: None,
                    });
                }
            }
        }
        if self.bed_usable() {
            for point in self.beds() {
                out.push(Interactable {
                    target: Target::Bed,
                    point,
                    readable: None,
                });
            }
        }
        if self.daytime.is_night() {
            for id in self.moon_doors() {
                if let Some(e) = data
                    .element(&id)
                    .filter(|_| self.level.is_barrier_open(&id))
                {
                    let r = e.rect;
                    out.push(Interactable {
                        target: Target::MoonDoor { id: id.clone() },
                        point: Vec2::new(
                            r.x as f32 + r.w as f32 / 2.0,
                            r.z as f32 + r.d as f32 / 2.0,
                        ),
                        readable: None,
                    });
                }
            }
        }
        if self.is_leading() || self.carrying_animal() {
            for e in data.elements_of(ElementType::Enclosure) {
                if let Some(g) = e.gate {
                    let c = Vec2::new(g.x as f32 + g.w as f32 / 2.0, g.z as f32 + g.d as f32 / 2.0);
                    out.push(Interactable {
                        target: Target::Gate {
                            enclosure: e.id.clone(),
                        },
                        point: c,
                        readable: None,
                    });
                }
            }
        }
        out
    }

    /// GAME-PLAYER §5: within 2 m of the interaction point, on the readable side (±60°) if
    /// the target has one, and facing it (±75°).
    pub fn is_available(&self, it: &Interactable) -> bool {
        self.is_available_within(it, crate::player::INTERACTION_RANGE_M)
    }

    /// [`Game::is_available`] with another range (panel hysteresis, GAME-PLAYER §4).
    pub fn is_available_within(&self, it: &Interactable, range: f32) -> bool {
        let p = self.player.pos;
        let to_player = p - it.point;
        let dist = to_player.length();
        if dist > range {
            return false;
        }
        if dist < 1e-3 {
            return true;
        }
        if let Some(f) = it.readable {
            if angle_deg(f, to_player) > READABLE_SIDE_DEG {
                return false;
            }
        }
        angle_deg(self.player.facing, -to_player) <= FACING_DEG
    }

    /// The nearest available interactable (PLAY-021), if any. Putting a carried item down
    /// is offered only when nothing else is available.
    pub fn available_target(&self) -> Option<Target> {
        let p = self.player.pos;
        self.interactables()
            .into_iter()
            .filter(|it| self.is_available(it))
            .min_by(|a, b| a.point.distance(p).total_cmp(&b.point.distance(p)))
            .map(|it| it.target)
            .or_else(|| {
                self.bowl
                    .as_ref()
                    .is_some_and(|b| b.carried)
                    .then_some(Target::PutDown)
            })
    }

    /// Interacts with the available target (GAME-PLAYER §4/§5). Reading targets return what
    /// the text panel shows; food boxes are only taken with [`Game::take_food`] (GAME-FEED §6).
    pub fn interact(&mut self) -> Option<Interaction> {
        let target = self.available_target()?;
        if target.is_reading() {
            // opened by hand: the panel is open now (and closes by itself again)
            self.panel.open = Some(target.clone());
            self.panel.candidate = None;
            self.panel.lost_s = 0.0;
            self.panel.suppressed = None;
        }
        match target {
            Target::InfoBoard { animal } => self
                .read_info_board(animal)
                .ok()
                .map(Interaction::InfoBoard),
            Target::FoodBox { food } => Some(Interaction::FoodBox {
                food,
                label: FoodBox { food }.label(self.settings.reading_level),
            }),
            Target::Animal { animal } => {
                self.show_food(animal).ok()?;
                let accepted = self
                    .animal(animal)
                    .is_some_and(|a| a.state == AnimalState::Following);
                Some(Interaction::ShowFood {
                    animal,
                    accepted,
                    carried: self.carry.food(),
                })
            }
            Target::Gate { enclosure } => {
                let i = self
                    .level
                    .data
                    .elements
                    .iter()
                    .position(|e| e.id == enclosure)?;
                let entered = if self.carrying_animal() {
                    self.deliver_container(i)
                } else {
                    self.lead_into(i, true)
                };
                Some(Interaction::Gate { enclosure, entered })
            }
            Target::Item { id } => {
                self.take_item(&id).ok()?;
                Some(Interaction::Item { id, action: "take" })
            }
            Target::Water { .. } => {
                self.fill_container().ok()?;
                let id = self.bowl.as_ref()?.id.clone();
                Some(Interaction::Item { id, action: "fill" })
            }
            Target::PutDown => {
                let id = self.bowl.as_ref()?.id.clone();
                self.put_down_item().then_some(Interaction::Item {
                    id,
                    action: "put_down",
                })
            }
            Target::Bed => self.sleep().then_some(Interaction::Sleep),
            Target::MoonDoor { id } => self
                .go_through_moon_door(&id)
                .map(|into_night_zoo| Interaction::MoonDoor { into_night_zoo }),
        }
    }

    /// Puts the bowl with its animal at an enclosure's gate (the goldfish's stone step): its
    /// own home → it jumps in, mission complete (RESC-021); another enclosure → it refuses.
    fn deliver_container(&mut self, enc: usize) -> bool {
        let Some(animal) = self.bowl.as_ref().map(|b| b.animal.clone()) else {
            return false;
        };
        let group = self.group(&animal);
        let Some(&first) = group.first() else {
            return false;
        };
        if self.animals[first].enclosure != enc {
            self.events.push(GameEvent::Refuse {
                animal,
                enclosure: self.level.data.elements[enc].id.clone(),
            });
            return false;
        }
        let gate = self.level.data.elements[enc].gate;
        if let Some(b) = &mut self.bowl {
            b.carried = false;
            b.fish = false;
            b.pos = gate.map_or(self.player.pos, |g| {
                Vec2::new(g.x as f32 + g.w as f32 / 2.0, g.z as f32 + g.d as f32 / 2.0)
            });
            b.lift_m = 0.1;
        }
        for j in group.clone() {
            self.animals[j].pos = self.player.pos;
            self.enter_enclosure(j, false);
        }
        self.complete_group(&animal);
        true
    }

    /// The leading group enters enclosure element `enc` if it is theirs, else refuses
    /// (GAME-RESCUE §7/§8). Returns whether they entered.
    fn lead_into(&mut self, enc: usize, announce_refuse: bool) -> bool {
        let leader = self
            .animals
            .iter()
            .position(|a| a.state == AnimalState::Following && !a.waiting);
        let Some(i) = leader else { return false };
        if self.animals[i].enclosure == enc {
            let id = self.animals[i].id();
            for j in self.group(id) {
                let a = &self.animals[j];
                if a.state == AnimalState::Following && !a.waiting {
                    self.enter_enclosure(j, true);
                }
            }
            self.complete_group(id);
            true
        } else {
            self.animals[i].refusing = true;
            if announce_refuse {
                self.events.push(GameEvent::Refuse {
                    animal: self.animals[i].id().to_owned(),
                    enclosure: self.level.data.elements[enc].id.clone(),
                });
            }
            false
        }
    }

    /// Advances the simulation by `dt` seconds with the joystick `input` (level
    /// coordinates `(x, z)`).
    pub fn update(&mut self, dt: f32, input: Vec2) {
        let leading = self.is_leading();
        let mut params = self.move_params;
        if self.carrying_animal() {
            params.walk_speed *= CARRY_ANIMAL_SPEED_FACTOR; // careful (RESC-023)
        }
        self.player.step_with(
            self.level.grid(),
            self.level.colliders(),
            &params,
            input,
            dt,
            leading,
        );
        if let Some(b) = self.bowl.as_ref().filter(|b| b.carried && b.fish) {
            let animal = b.animal.clone();
            for j in self.group(&animal) {
                self.animals[j].pos = self.player.pos;
                self.animals[j].facing = self.player.facing;
            }
        }
        self.check_gate();
        self.update_followers(dt);
        self.update_wander(dt);
        self.update_daytime(dt);
        self.update_panel(dt);
        self.time_s += f64::from(dt);
        if self.player.last_speed > 0.01 {
            self.autosave.moving_s += dt;
        }
    }

    /// Closes the reading panel by hand (✖/Esc, or after taking food): it stays closed until
    /// the player has left the target's range and comes back (PLAY-026).
    pub fn close_panel(&mut self) {
        if let Some(t) = self.panel.open.take() {
            self.panel.suppressed = Some(t);
        }
        self.panel.candidate = None;
        self.panel.lost_s = 0.0;
    }

    /// Distance from the player to a target's interaction point.
    fn target_distance(&self, t: &Target) -> Option<f32> {
        self.interactables()
            .into_iter()
            .find(|it| &it.target == t)
            .map(|it| it.point.distance(self.player.pos))
    }

    /// Automatic reading panel (GAME-PLAYER §4): opens after the target was available for
    /// [`PANEL_SETTLE_S`], closes after it was not available for [`PANEL_CLOSE_S`] (range
    /// hysteresis [`PANEL_KEEP_RANGE_M`]; another nearer interactable also closes it).
    fn update_panel(&mut self, dt: f32) {
        let available = self.available_target();
        if let Some(s) = &self.panel.suppressed {
            let left = self
                .target_distance(s)
                .is_none_or(|d| d > PANEL_KEEP_RANGE_M);
            if left {
                self.panel.suppressed = None;
            }
        }
        if let Some(open) = self.panel.open.clone() {
            let keep = match &available {
                Some(t) => *t == open,
                None => self
                    .interactables()
                    .iter()
                    .find(|it| it.target == open)
                    .is_some_and(|it| self.is_available_within(it, PANEL_KEEP_RANGE_M)),
            };
            if keep {
                self.panel.lost_s = 0.0;
            } else {
                self.panel.lost_s += dt;
                if self.panel.lost_s >= PANEL_CLOSE_S - 1e-4 {
                    self.panel.open = None;
                    self.panel.lost_s = 0.0;
                    self.events.push(GameEvent::PanelClosed { target: open });
                }
            }
            if self.panel.open.is_some() {
                return;
            }
        }
        let candidate =
            available.filter(|t| t.is_reading() && self.panel.suppressed.as_ref() != Some(t));
        let Some(t) = candidate else {
            self.panel.candidate = None;
            return;
        };
        let settled = match &mut self.panel.candidate {
            Some((c, s)) if *c == t => {
                *s += dt;
                *s
            }
            _ => {
                self.panel.candidate = Some((t.clone(), 0.0));
                0.0
            }
        };
        if settled >= PANEL_SETTLE_S - 1e-4 {
            self.panel.candidate = None;
            self.panel.lost_s = 0.0;
            if let Target::InfoBoard { animal } = &t {
                // reading the board starts the mission (RESC-012)
                let _ = self.read_info_board(animal);
            }
            self.panel.open = Some(t.clone());
            self.events.push(GameEvent::PanelOpened { target: t });
        }
    }

    fn check_gate(&mut self) {
        let gate = match self.level.grid().kind(cell_of(self.player.pos)) {
            CellKind::Gate(enc) => Some(enc),
            _ => None,
        };
        let entered = gate.is_some() && gate != self.last_gate;
        self.last_gate = gate;
        for a in &mut self.animals {
            a.refusing = false;
        }
        let Some(enc) = gate else { return };
        self.lead_into(enc, entered);
    }

    /// GAME-RESCUE §8: one animal enters its enclosure (steps onto the cell inside the gate).
    /// `eat_food`: the carried food is used up (not for an animal carried in its container,
    /// which was fed at the bank).
    fn enter_enclosure(&mut self, i: usize, eat_food: bool) {
        let enc_index = self.animals[i].enclosure;
        let area = wander::home_area(&self.level, enc_index);
        let entry = wander::home_entry(&self.level, enc_index)
            .and_then(|c| {
                if area.contains(c) {
                    Some(c)
                } else {
                    area.nearest(cell_center(c))
                }
            })
            .unwrap_or_else(|| {
                let r = self.level.data.elements[enc_index].rect;
                IVec2::new(r.x + r.w / 2, r.z + r.d / 2)
            });
        let pause = wander::draw_pause(&mut self.rng);
        let a = &mut self.animals[i];
        a.state = AnimalState::InEnclosure;
        a.waiting = false;
        a.refusing = false;
        a.path.clear();
        a.path_target = None;
        let to_entry = cell_center(entry) - a.pos;
        a.facing = to_entry.try_normalize().unwrap_or(a.facing);
        a.pos = cell_center(entry);
        a.area = area;
        a.wander = Wander {
            pause_s: pause,
            route: Vec::new(),
        };
        let id = a.id().to_owned();
        if eat_food {
            if let Some(food) = self.carry.consume() {
                self.events.push(GameEvent::FoodConsumed { food });
            }
        }
        self.events.push(GameEvent::InEnclosure { animal: id });
    }

    /// GAME-RESCUE §8/§9, GAME-FAMILY §2: the mission completes when every animal of the
    /// species is home; a level whose missions are all complete opens its exit barrier and
    /// the entry barriers of the next level **the next morning** (GAME-NIGHT, Q-091); a day
    /// level brings nightfall after its celebration (NIGHT-001).
    fn complete_group(&mut self, animal: &str) {
        let group = self.group(animal);
        if group.is_empty()
            || !group
                .iter()
                .all(|&j| self.animals[j].state == AnimalState::InEnclosure)
            || self.missions[group[0]].complete
        {
            return;
        }
        for &j in &group {
            self.missions[j].complete = true;
        }
        self.events.push(GameEvent::MissionComplete {
            animal: animal.to_owned(),
        });
        let part = self.animals[group[0]].part;
        let part_done = (0..self.animals.len())
            .filter(|&j| self.animals[j].part == part)
            .all(|j| self.missions[j].complete);
        if !self.all_home && self.missions.iter().all(|m| m.complete) {
            self.all_home = true;
            self.events.push(GameEvent::AllAnimalsHome);
        }
        if part_done {
            let level_id = self.level.data.parts[part].id.clone();
            self.events.push(GameEvent::LevelComplete {
                level: level_id.clone(),
            });
            // the exits open the next morning; a day level brings nightfall (GAME-NIGHT, Q-091)
            let night = self.level.data.is_night_part(part);
            self.daytime.level_complete(&level_id, night);
        }
    }

    /// Opens the barriers a completed level unlocks (its exits `<level>-><next>`, proposal
    /// Q-022, or barriers with `unlock_after = <level>`, Q-133) and every entry barrier of
    /// the next level if it is joined (level 3 is also entered through the level-1 north
    /// gate, proposal Q-090).
    pub(crate) fn open_exits(&mut self, level_id: &str) {
        let mut ids = Vec::new();
        for b in self.level.barriers_unlocked_by(level_id) {
            let next = self
                .level
                .data
                .element(&b)
                .and_then(|e| e.transition.as_deref())
                .and_then(|t| t.split("->").nth(1))
                .map(str::to_owned);
            ids.push(b);
            if let Some(k) = next.and_then(|n| self.level.data.part_index(&n)) {
                ids.extend(
                    self.level.data.parts[k]
                        .entries
                        .iter()
                        .map(|e| e.barrier.clone()),
                );
            }
        }
        for b in ids {
            if self.level.open_barrier(&b) {
                self.events.push(GameEvent::BarrierOpened { id: b });
            }
        }
    }

    fn update_followers(&mut self, dt: f32) {
        let p = self.player.pos;
        let player_cell = cell_of(p);
        let fp = self.follow_params;
        for a in &mut self.animals {
            if a.state != AnimalState::Following {
                continue;
            }
            let dist = a.pos.distance(p);
            if !a.waiting && dist > fp.wait_beyond_m {
                a.waiting = true;
                self.events.push(GameEvent::Waiting {
                    animal: a.id().to_owned(),
                });
            } else if a.waiting && dist <= fp.resume_within_m {
                a.waiting = false;
                a.path_target = None;
                self.events.push(GameEvent::FollowingAgain {
                    animal: a.id().to_owned(),
                });
            }
            if a.waiting || a.refusing || dist <= fp.keep_distance_m {
                continue;
            }
            let grid = self.level.grid();
            let speed = match grid.surface(cell_of(a.pos)) {
                Some(Surface::Path) => self.move_params.speed_on(Surface::Path),
                _ => self.move_params.speed_on(Surface::Grass),
            };
            let here = cell_of(a.pos);
            if a.path_target != Some(player_cell) || !a.path.contains(&here) {
                a.path = nav::find_path(grid, here, player_cell, true).unwrap_or_default();
                a.path_target = Some(player_cell);
            }
            // Next waypoint: the cell after the current one, or the player when adjacent / no path.
            let waypoint = a
                .path
                .iter()
                .position(|&c| c == here)
                .and_then(|k| a.path.get(k + 1))
                .filter(|&&c| c != player_cell)
                .map(|&c| cell_center(c))
                .unwrap_or(p);
            let to = waypoint - a.pos;
            let step = (speed * dt).min(to.length()).min(dist - fp.keep_distance_m);
            if step > 0.0 {
                a.pos += to.normalize_or_zero() * step;
                a.facing = to.normalize_or(a.facing);
            }
        }
    }

    /// Wandering of escaped animals (inside their hiding place's wander area) and of animals
    /// at home (inside their enclosure), GAME-ANIMALS "Animal states", ANIM-008…012.
    fn update_wander(&mut self, dt: f32) {
        let p = self.player.pos;
        let night_part: Vec<bool> = self.level.data.parts.iter().map(|pt| pt.night).collect();
        let dark = self.daytime.is_dark();
        for i in 0..self.animals.len() {
            let state = self.animals[i].state;
            if matches!(state, AnimalState::Following | AnimalState::InBowl)
                || self.animals[i].area.is_empty()
                || !self.part_unlocked(self.animals[i].part)
            {
                continue; // locked levels are asleep (not simulated)
            }
            if state == AnimalState::InEnclosure && dark && !night_part[self.animals[i].part] {
                continue; // day animals lie down in their enclosures at night (rule 1)
            }
            if self.perch(&self.animals[i]).is_some() {
                // up in the tree / crow's nest: looks at the player when she is near (Q-094)
                let a = &mut self.animals[i];
                let to = p - a.pos;
                if to.length() <= wander::NOTICE_PLAYER_M {
                    a.facing = to.normalize_or(a.facing);
                }
                continue;
            }
            let escaped = state == AnimalState::Escaped;
            let water_bias = !escaped
                && self.level.data.elements[self.animals[i].enclosure]
                    .home_surfaces()
                    .contains(&"water");
            let a = &mut self.animals[i];
            let to_player = p - a.pos;
            let near = to_player.length();
            if escaped && near <= wander::NOTICE_PLAYER_M {
                // out of reach (e.g. far out in the pond): come to the cell nearest to her
                // (proposal Q-097); within reach: stop and look at her (ANIM-009)
                let reachable = near <= crate::player::INTERACTION_RANGE_M - 0.2;
                if !reachable {
                    if let Some(best) = a.area.nearest(p) {
                        let here = cell_of(a.pos);
                        let gain = a.pos.distance(p) - cell_center(best).distance(p);
                        if best != here && gain > 0.3 && a.wander.route.last() != Some(&best) {
                            a.wander.route = a.area.route(here, best).unwrap_or_default();
                        }
                    }
                }
                if reachable || near <= wander::STOP_NEAR_PLAYER_M && a.wander.route.is_empty() {
                    a.wander.route.clear();
                    a.facing = to_player.normalize_or(a.facing);
                    continue;
                }
            }
            if a.wander.route.is_empty() {
                // night animals are awake and wander more at night (GAME-NIGHT rule 5)
                a.wander.pause_s -= if night_part[a.part] { 2.0 * dt } else { dt };
                if a.wander.pause_s > 0.0 {
                    continue;
                }
                let here = cell_of(a.pos);
                let target = wander::draw_target(&a.area, here, water_bias, true, &mut self.rng);
                a.wander.pause_s = wander::draw_pause(&mut self.rng);
                if let Some(t) = target {
                    a.wander.route = a.area.route(here, t).unwrap_or_default();
                }
                continue;
            }
            let (_, dir) =
                wander::follow_route(&mut a.pos, &mut a.wander.route, wander::WANDER_SPEED * dt);
            if dir != Vec2::ZERO {
                a.facing = dir;
            }
        }
    }

    /// Clip an animal rests with at its place (GAME-RESCUE §11): the hiding place `pose`
    /// while escaped (e.g. `drink` at the river, `swim` in the pond), `swim` in water at home,
    /// else `idle`. The presentation falls back to `idle` if the model lacks the clip.
    pub fn rest_clip(&self, a: &Animal) -> &str {
        if a.state == AnimalState::InBowl {
            return "swim";
        }
        if a.state == AnimalState::InEnclosure && self.daytime.is_dark() && !self.is_night_animal(a)
        {
            // the animals in their enclosures lie down at night (GAME-NIGHT rule 1)
            return "sleep";
        }
        if self.water_depth(a) > 0.5 && a.state != AnimalState::Following {
            return "swim";
        }
        if a.state == AnimalState::Escaped {
            if let Some(pose) = self
                .level
                .data
                .hiding_place(&a.hiding_place)
                .and_then(|h| h.pose.as_deref())
            {
                return pose;
            }
        }
        "idle"
    }

    /// Water depth factor 0…1 under an animal (presentation sinks swimmers, GAME-LEVEL-1).
    pub fn water_depth(&self, a: &Animal) -> f32 {
        wander::water_depth(&self.level, a.pos)
    }

    /// Debug / e2e / tests: the animal walks into its own enclosure as if led there (same
    /// rules and events as GAME-RESCUE §8). Returns false for an unknown animal or one that
    /// is already home.
    pub fn debug_send_home(&mut self, animal: &str) -> bool {
        let Some(i) = self.animal_index(animal) else {
            return false;
        };
        if self.animals[i].state == AnimalState::InEnclosure {
            return false;
        }
        let id = self.animals[i].id();
        for j in self.group(id) {
            if self.animals[j].state != AnimalState::InEnclosure {
                self.enter_enclosure(j, true);
            }
        }
        if let Some(b) = self.bowl.as_mut().filter(|b| b.animal == id && b.fish) {
            b.fish = false;
        }
        self.complete_group(id);
        true
    }

    /// Recomputes the derived wander areas (after a restore).
    pub(crate) fn refresh_areas(&mut self) {
        for i in 0..self.animals.len() {
            let area = wander_area_of(&self.level, &self.animals[i]);
            self.animals[i].area = area;
        }
    }
}

/// Wander area for an animal's state: its hiding place while escaped, its enclosure when
/// home, none while following.
fn wander_area_of(level: &Level, a: &Animal) -> WanderArea {
    match a.state {
        AnimalState::Escaped => level
            .data
            .hiding_place(&a.hiding_place)
            .map(|h| wander::hiding_area(level, h))
            .unwrap_or_default(),
        AnimalState::InEnclosure => wander::home_area(level, a.enclosure),
        AnimalState::Following | AnimalState::InBowl => WanderArea::default(),
    }
}

/// Facing of an escaped animal at the start: towards the nearest water when it drinks
/// there (`pose = "drink"`), else level south (towards the default camera).
fn rest_facing(level: &Level, a: &Animal) -> Vec2 {
    let drinks = level
        .data
        .hiding_place(&a.hiding_place)
        .is_some_and(|h| h.pose.as_deref() == Some("drink"));
    if !drinks {
        return Vec2::NEG_Y;
    }
    level
        .data
        .elements
        .iter()
        .filter(|e| {
            e.ty == ElementType::Landmark && matches!(e.kind.as_deref(), Some("river" | "pond"))
        })
        .flat_map(|e| e.rect.cells())
        .map(|c| cell_center(c) - a.pos)
        .min_by(|x, y| x.length().total_cmp(&y.length()))
        .map_or(Vec2::NEG_Y, |d| d.normalize_or(Vec2::NEG_Y))
}
