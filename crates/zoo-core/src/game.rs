//! Game state and the rescue mission flow (GAME-RESCUE, GAME-ANIMALS, GAME-FEED).
//!
//! Deterministic: all randomness comes from the seed, time only from `dt`.

use std::collections::BTreeMap;

use glam::{IVec2, Vec2};

use crate::animals::{animal_info, AnimalInfo, AnimalState};
use crate::content::{
    animal_more_key, animal_name_key, facts_key, pair_note_key, riddle_key, Language, ReadingLevel,
};
use crate::daytime::Daytime;
use crate::food::{Carry, Food, FoodBox, FoodLabel, FoodStorage};
use crate::level::{
    cell_center, cell_of, facing_vec, CellKind, ElementType, Level, LevelData, Surface,
};
use crate::nav;
use crate::player::{in_interaction_range, MoveParams, Player};
use crate::rng::Pcg32;
use crate::scene::{info_board_pose, map_board_pose};
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
    /// A plant was harvested into the basket (GAME-GARDEN §3).
    Harvested {
        spot: String,
        treat: crate::garden::Treat,
        count: u32,
    },
    /// The basket is full: the plant stays (GARD-003).
    BasketFull,
    /// An animal at home ate a garden treat (GAME-GARDEN §6).
    TreatEaten {
        animal: String,
        treat: crate::garden::Treat,
    },
    /// A pair at home was given a liked special food: one baby is born (GAME-FAMILY
    /// "Special food and babies", FAM-008); once per pair.
    BabyBorn {
        animal: String,
    },
    /// An animal at home ate the food the child carries (GAME-GARDEN §6, GARD-010): the food
    /// stays in the hands.
    FoodEaten {
        animal: String,
        food: Food,
    },
    /// It does not like the carried food: it turns away, the food stays in the hands.
    FoodRefused {
        animal: String,
        food: Food,
    },
    /// It does not like that treat: it sniffs and turns away, the treat stays.
    TreatRefused {
        animal: String,
        treat: crate::garden::Treat,
    },
    /// The player picked up a carryable item (the fish bowl).
    ItemTaken {
        id: String,
    },
    /// The player put a carried item down (`fish_bowl`, `food:<id>`; GAME-FEED §8).
    ItemPutDown {
        id: String,
    },
    /// A food went back into its box: dropped next to it, or the oldest lying food when a
    /// 9th item is put down (GAME-FEED §10).
    FoodPutBack {
        food: Food,
    },
    /// Nothing was put down: no free spot (the put-down button shakes, GAME-FEED §9).
    PutDownRefused,
    /// A bamboo stalk was snapped off at a cut spot (GAME-FEED §14).
    BambooCut {
        spot: String,
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
    /// A food lying on the ground (GAME-FEED §11), by its lying id.
    LyingFood {
        uid: u32,
        food: Food,
    },
    /// A full-grown stalk at a bamboo cut spot (GAME-FEED §14).
    CutSpot {
        spot: String,
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
    /// A ripe plant in a garden bed: pull it out (GAME-GARDEN §3).
    Plant {
        spot: String,
    },
    /// A garden sign: its word in the text panel (GARD-009).
    GardenSign {
        bed: String,
    },
    /// The big map board at a level entry: the game description of that level (RESC-028).
    /// `level` is the id of the level part the board belongs to (`level_1`, …).
    WelcomeBoard {
        level: String,
    },
    /// An animal at home, from its fence, with a treat in the basket (GAME-GARDEN §6).
    Treat {
        animal: &'static str,
    },
    /// The toy telescope (only at night, GAME-TELESCOPE): opens the planet view. `id` is the
    /// level element id (`telescope_n1`).
    Telescope {
        id: String,
    },
}

/// What the child gives to an animal at home.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gift {
    /// A garden treat from the basket (leaves the basket when eaten).
    Treat(crate::garden::Treat),
    /// The food in the hands (stays in the hands).
    Food(Food),
}

impl Target {
    /// Targets with a reading panel (info boards, food boxes) open it by themselves.
    pub fn is_reading(&self) -> bool {
        matches!(
            self,
            Target::InfoBoard { .. }
                | Target::FoodBox { .. }
                | Target::GardenSign { .. }
                | Target::WelcomeBoard { .. }
        )
    }

    /// Short id for the host UI (button icon).
    pub fn kind(&self) -> &'static str {
        match self {
            Target::InfoBoard { .. } => "info_board",
            Target::FoodBox { .. } => "food_box",
            Target::Animal { .. } => "animal",
            Target::Gate { .. } => "gate",
            Target::Item { .. } => "item",
            Target::LyingFood { .. } => "lying_food",
            Target::CutSpot { .. } => "bamboo",
            Target::Water { .. } => "water",
            Target::PutDown => "put_down",
            Target::Bed => "bed",
            Target::MoonDoor { .. } => "moon_door",
            Target::Plant { .. } => "plant",
            Target::GardenSign { .. } => "garden_sign",
            Target::WelcomeBoard { .. } => "welcome_board",
            Target::Treat { .. } => "treat",
            Target::Telescope { .. } => "telescope",
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
#[derive(Debug, Clone, PartialEq)]
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
    /// Harvested a plant (`count` treats into the basket; 0 = the basket is full).
    Harvest {
        spot: String,
        treat: crate::garden::Treat,
        count: u32,
    },
    /// A garden sign: the word (Fluent key) and the reading-level sentence key.
    GardenSign { bed: String, key: String },
    /// The welcome board of a level (RESC-028): the level part id and its animal species.
    WelcomeBoard {
        level: String,
        animals: Vec<&'static str>,
    },
    /// Looked through the telescope (GAME-TELESCOPE): the host opens the planet view.
    Telescope { id: String },
    /// Gave the carried food to an animal at home (it stays in the hands).
    FoodGift {
        animal: &'static str,
        food: Food,
        accepted: bool,
    },
    /// Gave a treat to an animal at home.
    Treat {
        animal: &'static str,
        treat: crate::garden::Treat,
        accepted: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractError {
    UnknownTarget,
    OutOfRange,
}

/// The baby of a pair at home (GAME-FAMILY §5, GARD-013): a real member of the group that
/// follows the female, is called to the feeding spot with the pair (rank 2) and reacts to
/// treats. Its position is not saved (it is placed next to the female on load).
#[derive(Debug, Clone, PartialEq)]
pub struct Baby {
    pub pos: Vec2,
    pub facing: Vec2,
    /// Remaining cells to walk through; empty = resting.
    pub route: Vec<IVec2>,
    /// Playful following state (mode, hop, clip clock), see [`crate::baby`].
    pub play: crate::baby::BabyPlay,
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

/// NEVER STUCK: seconds without mission progress until the hint points at the missing animal
/// itself, and until the animal walks to the player (Q-097 proposal).
pub const STALL_HELP_S: f32 = 120.0;
/// After an animal group accepted a treat or food there is no treat hint for this species for
/// this long (s of play, GAME-HINT "No hint loops", HINT-026).
pub const GIFT_COOLDOWN_S: f32 = 180.0;
pub const STALL_WALK_S: f32 = 180.0;
const STALL_COME_M: f32 = 5.0;
const STALL_SPEED: f32 = 1.6;

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
    /// A pair species (male + female, GAME-FAMILY): the generic pair note
    /// (`mission-pair-note-<level>`, Q-308) shown after the facts.
    pub pair_note_key: Option<String>,
    /// A species with a box-food treat (GAME-FEED "Basic food and treats"): the treat line
    /// prefix key (`board-treat-<level>`) and the food label key of the (first) treat; shown
    /// before the facts so the child learns what makes the baby.
    pub treat: Option<(String, String)>,
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
    /// The intro at the entrance gate was shown (GAME-RESCUE "Intro at the entrance gate",
    /// RESC-029); saved, so a loaded game never repeats it.
    pub intro_seen: bool,
    /// Species whose pair already has its baby (GAME-FAMILY, FAM-008/009); saved.
    pub babies: Vec<String>,
    /// Where the baby of each pair species is (a real member of the group at home, GARD-013).
    pub baby_states: BTreeMap<String, Baby>,
    /// Seconds left of the feeding cooldown per species (GAME-HINT "No hint loops", HINT-026):
    /// set when a treat / food was accepted, no treat hint meanwhile. Not saved.
    pub(crate) gift_cooldown: BTreeMap<String, f32>,
    /// Feeding spot per element index (enclosures with a gate), GAME-GARDEN §6.
    pub feed_spots: Vec<Option<wander::FeedSpot>>,
    /// Animal house per element index (enclosures with one, GAME-HOUSE).
    pub houses: Vec<Option<crate::house::House>>,
    /// Automatic reading panel (GAME-PLAYER §4).
    pub panel: ReadingPanel,
    /// Seed of this playthrough and the RNG after the setup (GAME-SAVE).
    pub(crate) seed: u64,
    pub(crate) rng: Pcg32,
    /// Own RNG of the babies' playful behaviour (not saved; keeps the main stream untouched).
    pub(crate) baby_rng: Pcg32,
    /// Play time in seconds (sum of `update` steps).
    pub time_s: f64,
    /// Autosave bookkeeping (GAME-SAVE §3).
    pub(crate) autosave: Autosave,
    /// The fish bowl (`[[item]]` of kind `fish_bowl`, GAME-RESCUE "goldfish bowl"), if the
    /// joined levels have one.
    pub bowl: Option<Bowl>,
    /// Time of day (GAME-NIGHT).
    pub daytime: Daytime,
    /// Vegetable gardens: plant stages and the treat basket (GAME-GARDEN).
    pub garden: crate::garden::Garden,
    /// The treat offered at a fence (the child's choice; default: the first one in the basket).
    pub treat_choice: Option<crate::garden::Treat>,
    /// Foods lying on the ground (GAME-FEED §10).
    pub lying: crate::carrying::Lying,
    /// Regrowth of the bamboo cut spots (GAME-FEED §15).
    pub bamboo: crate::carrying::BambooForests,
    /// NEVER STUCK (GAME-RESCUE "Stall"): seconds of play without mission progress while a
    /// mission is open, and the progress signature it is measured against.
    pub(crate) stall_s: f32,
    stall_sig: u64,
    /// Time until the mission flags are reconciled with the animals' states again (s).
    reconcile_s: f32,
    /// Route of the escaped animals that walk to the player once the stall lasts too long.
    stall_routes: Vec<Vec<IVec2>>,
}

/// A building door opens while the player is this close to the door cell (m).
pub const DOOR_OPEN_M: f32 = 1.6;
/// An enclosure gate opens while the player leads animals this close to it (m).
pub const GATE_OPEN_M: f32 = 3.0;
/// The garden gate opens by itself within this distance (m, proposal Q-102).
pub const GARDEN_GATE_OPEN_M: f32 = 2.0;

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
    /// Put down by the child (lies on the ground, counts as a lying item, GAME-FEED §10).
    pub dropped: bool,
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
                | GameEvent::Harvested { .. }
                | GameEvent::TreatEaten { .. }
                | GameEvent::FoodEaten { .. }
                | GameEvent::BabyBorn { .. }
                | GameEvent::FoodPutBack { .. }
                | GameEvent::BambooCut { .. }
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
        let mut player = Player::new(cell_center(data.spawn.cell()), facing, &move_params);
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
                dropped: false,
            });
        let garden = crate::garden::Garden::new(&data);
        let bamboo = crate::carrying::BambooForests::new(data.cut_spots.len());
        let level = Level::new(data);
        player.y = level.ground_height(player.pos);
        let feed_spots = (0..level.data.elements.len())
            .map(|e| {
                (level.data.elements[e].ty == ElementType::Enclosure)
                    .then(|| wander::feed_spot(&level, e, &wander::home_area(&level, e)))
                    .flatten()
            })
            .collect();
        let houses = level
            .data
            .elements
            .iter()
            .map(|e| {
                (e.ty == ElementType::Enclosure)
                    .then(|| crate::house::house_of(&level.data, &e.id))
                    .flatten()
            })
            .collect();
        for a in &mut animals {
            a.area = wander_area_of(&level, a);
            a.facing = rest_facing(&level, a);
            // the second animal of a pair waits one wander cell away (GAME-FAMILY §1)
            if a.member > 0 {
                // (the cell nearest to the first one that is at least the species' pair gap away,
                // else the farthest cell; Q-308)
                let gap = crate::animals::pair_gap_m(a.id());
                let d = |c: &IVec2| cell_center(*c).distance(a.pos);
                let cells: Vec<IVec2> = a
                    .area
                    .cells()
                    .map(|(c, _)| c)
                    .filter(|&c| c != cell_of(a.pos))
                    .collect();
                let far = cells
                    .iter()
                    .copied()
                    .filter(|c| d(c) >= gap - 1e-3 && d(c) <= 3.0)
                    .min_by(|x, y| d(x).total_cmp(&d(y)));
                if let Some(c) = far.or_else(|| {
                    cells
                        .iter()
                        .copied()
                        .filter(|c| d(c) <= 3.0)
                        .max_by(|x, y| d(x).total_cmp(&d(y)))
                }) {
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
            stall_s: 0.0,
            stall_sig: 0,
            reconcile_s: 0.0,
            stall_routes: Vec::new(),
            intro_seen: false,
            babies: Vec::new(),
            baby_states: BTreeMap::new(),
            gift_cooldown: BTreeMap::new(),
            feed_spots,
            houses,
            panel: ReadingPanel::default(),
            seed,
            rng,
            baby_rng: Pcg32::new(seed ^ 0xBABE_BABE_5EED),
            time_s: 0.0,
            autosave: Autosave::default(),
            bowl,
            daytime: Daytime::default(),
            garden,
            treat_choice: None,
            lying: crate::carrying::Lying::default(),
            bamboo,
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

    /// The animal species of one level part in level order (pairs count once, RESC-028).
    pub fn part_animal_ids(&self, k: usize) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = Vec::new();
        for a in self.animals.iter().filter(|a| a.part == k) {
            if !out.contains(&a.id()) {
                out.push(a.id());
            }
        }
        out
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
        Some((
            crate::scene::perch_point_of(place, &self.level.data, a.member),
            h,
        ))
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

    /// The mission of an animal's species: started when any member's is, complete only when
    /// every member's is (NEVER STUCK: members must never disagree).
    pub fn mission(&self, animal: &str) -> Option<Mission> {
        let group = self.group(animal);
        if group.is_empty() {
            return None;
        }
        Some(Mission {
            started: group.iter().any(|&i| self.missions[i].started),
            complete: group.iter().all(|&i| self.missions[i].complete),
        })
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
            pair_note_key: (self.group(a.id()).len() >= 2).then(|| pair_note_key(level)),
            treat: a
                .info
                .treats
                .first()
                .map(|f| (format!("board-treat-{}", level.id()), f.label_key())),
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
        if !self.mission(animal).is_some_and(|m| m.started) {
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
        // the member that is still out (a pair may be split: one at home, one outside), else
        // the first one
        let first = self
            .animal_index(animal)
            .ok_or(InteractError::UnknownTarget)?;
        let i = self
            .group(animal)
            .into_iter()
            .filter(|&j| self.animals[j].state == AnimalState::Escaped)
            .min_by(|&x, &y| {
                let p = self.player.pos;
                self.animals[x]
                    .pos
                    .distance(p)
                    .total_cmp(&self.animals[y].pos.distance(p))
            })
            .unwrap_or(first);
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
        b.dropped = false;
        b.lift_m = 0.0;
        self.events.push(GameEvent::ItemTaken { id: id.to_owned() });
        Ok(())
    }

    /// Puts the carried bowl down in front of the player (GAME-RESCUE "goldfish bowl" 7: the
    /// fish stays safe in it); the GAME-FEED §8–10 rules apply ([`Game::put_down`]).
    pub fn put_down_item(&mut self) -> bool {
        if !self.bowl.as_ref().is_some_and(|b| b.carried) {
            return false;
        }
        match self.put_down() {
            Ok(_) => true,
            Err(_) => {
                self.events.push(GameEvent::PutDownRefused);
                false
            }
        }
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
        // the big map boards at the level entries explain the game (RESC-028)
        for e in &data.elements {
            if e.kind.as_deref() != Some("map_board") || !self.part_unlocked(e.part) {
                continue;
            }
            let Some(part) = data.parts.get(e.part) else {
                continue;
            };
            let (point, dir) = map_board_pose(e, part.spawn.cell());
            out.push(Interactable {
                target: Target::WelcomeBoard {
                    level: part.id.clone(),
                },
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
        // lying foods (GAME-FEED §11) and full-grown bamboo stalks (§14)
        for f in &self.lying.foods {
            if self.part_unlocked(data.part_at(cell_of(f.pos)).unwrap_or(0)) {
                out.push(Interactable {
                    target: Target::LyingFood {
                        uid: f.uid,
                        food: f.food,
                    },
                    point: f.pos,
                    readable: None,
                });
            }
        }
        for (i, c) in data.cut_spots.iter().enumerate() {
            let full = self.bamboo.stage(i) == Some(crate::carrying::StalkStage::Full);
            if full && self.part_unlocked(c.part) {
                out.push(Interactable {
                    target: Target::CutSpot { spot: c.id.clone() },
                    point: c.pos(),
                    readable: None,
                });
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
        // the toy telescope shows the planets, only at night (GAME-TELESCOPE, Q-365)
        if self.daytime.is_night() {
            for e in &data.elements {
                if e.kind.as_deref() == Some("telescope") && self.part_unlocked(e.part) {
                    let r = e.rect;
                    out.push(Interactable {
                        target: Target::Telescope { id: e.id.clone() },
                        point: Vec2::new(
                            r.x as f32 + r.w as f32 / 2.0,
                            r.z as f32 + r.d as f32 / 2.0,
                        ),
                        readable: None,
                    });
                }
            }
        }
        // vegetable gardens (GAME-GARDEN): ripe plants, the signs
        for sp in &data.plant_spots {
            let ripe = self
                .garden
                .plant(&sp.id)
                .is_some_and(|p| p.stage() == crate::garden::Stage::Ripe);
            if ripe && self.part_unlocked(sp.part) {
                out.push(Interactable {
                    target: Target::Plant {
                        spot: sp.id.clone(),
                    },
                    point: sp.pos(),
                    readable: None,
                });
            }
        }
        for b in &data.garden_beds {
            if self.part_unlocked(b.part) {
                out.push(Interactable {
                    target: Target::GardenSign { bed: b.id.clone() },
                    point: Vec2::from(b.sign_pos),
                    readable: Some(facing_vec(&b.sign_facing)),
                });
            }
        }
        // gifts: at an animal at home, inside its enclosure or at its fence (GAME-GARDEN §6)
        if !self.is_leading() {
            let p = self.player.pos;
            let mut seen: Vec<&'static str> = Vec::new();
            for a in &self.animals {
                if a.state != AnimalState::InEnclosure
                    || !self.in_scope(a)
                    || seen.contains(&a.id())
                    || self.gift_for(a.id()).is_none()
                {
                    continue;
                }
                seen.push(a.id());
                // the member of the group nearest to the player
                let near = self
                    .group(a.id())
                    .into_iter()
                    .filter(|&j| self.animals[j].state == AnimalState::InEnclosure)
                    .map(|j| self.animals[j].pos)
                    .chain(self.baby_states.get(a.id()).map(|b| b.pos))
                    .min_by(|x, y| {
                        // the baby counts like the adults (FAM-013): the nearest member in
                        // reach and in front of the child wins, else the nearest one
                        let key = |q: &Vec2| {
                            let d = q.distance(p);
                            let ok = d <= crate::player::INTERACTION_RANGE_M
                                && (d < 1e-3
                                    || angle_deg(self.player.facing, *q - p) <= FACING_DEG);
                            (!ok, d)
                        };
                        let (kx, ky) = (key(x), key(y));
                        kx.0.cmp(&ky.0).then(kx.1.total_cmp(&ky.1))
                    })
                    .unwrap_or(a.pos);
                out.push(Interactable {
                    target: Target::Treat { animal: a.id() },
                    point: near,
                    readable: None,
                });
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
        let range = match it.target {
            Target::CutSpot { .. } => range.min(crate::carrying::CUT_RANGE_M),
            _ => range,
        };
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
                    .group(animal)
                    .into_iter()
                    .any(|j| self.animals[j].state == AnimalState::Following);
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
            Target::LyingFood { uid, food } => self.pick_up_food(uid).then(|| Interaction::Item {
                id: format!("food:{}", food.id()),
                action: "take",
            }),
            Target::CutSpot { spot } => self.cut_bamboo(&spot).then(|| Interaction::Item {
                id: format!("cut_spot:{spot}"),
                action: "cut",
            }),
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
            Target::Telescope { id } => Some(Interaction::Telescope { id }),
            Target::Plant { spot } => self.harvest(&spot),
            Target::GardenSign { bed } => {
                let b = self.level.data.garden_beds.iter().find(|b| b.id == bed)?;
                Some(Interaction::GardenSign {
                    bed: b.id.clone(),
                    key: b.sign_key.clone(),
                })
            }
            Target::WelcomeBoard { level } => {
                let k = self.level.data.part_index(&level)?;
                Some(Interaction::WelcomeBoard {
                    animals: self.part_animal_ids(k),
                    level,
                })
            }
            Target::Treat { animal } => match self.gift_for(animal)? {
                Gift::Treat(treat) => {
                    let accepted = self.give_treat(animal, treat)?;
                    Some(Interaction::Treat {
                        animal,
                        treat,
                        accepted,
                    })
                }
                Gift::Food(food) => {
                    let accepted = self.give_food(animal)?;
                    Some(Interaction::FoodGift {
                        animal,
                        food,
                        accepted,
                    })
                }
            },
        }
    }

    /// Harvests a ripe plant into the basket (GARD-001/002); a full basket refuses and the
    /// plant stays (GARD-003). The hands are not needed (GARD-007).
    pub fn harvest(&mut self, spot: &str) -> Option<Interaction> {
        use crate::garden::HarvestError;
        match self.garden.harvest(spot, &mut self.rng) {
            Ok((treat, count)) => {
                self.events.push(GameEvent::Harvested {
                    spot: spot.to_owned(),
                    treat,
                    count,
                });
                Some(Interaction::Harvest {
                    spot: spot.to_owned(),
                    treat,
                    count,
                })
            }
            Err(HarvestError::BasketFull) => {
                self.events.push(GameEvent::BasketFull);
                let treat = self.garden.plant(spot)?.treat;
                Some(Interaction::Harvest {
                    spot: spot.to_owned(),
                    treat,
                    count: 0,
                })
            }
            Err(_) => None,
        }
    }

    /// The treat offered at a fence: the child's choice if the basket has it, else the first
    /// one in the basket.
    pub fn offered_treat(&self) -> Option<crate::garden::Treat> {
        let b = &self.garden.basket;
        self.treat_choice.filter(|t| b.count(*t) > 0).or_else(|| {
            crate::garden::Treat::ALL
                .into_iter()
                .find(|t| b.count(*t) > 0)
        })
    }

    /// Gives a treat to an animal at home (GAME-GARDEN §6, GARD-005): it eats it if it likes
    /// it (the treat leaves the basket), else it refuses and the treat stays. `None` if the
    /// animal is not at home or the basket has no such treat.
    pub fn give_treat(&mut self, animal: &str, treat: crate::garden::Treat) -> Option<bool> {
        let i = self.animal_index(animal)?;
        if self.animals[i].state != AnimalState::InEnclosure || self.garden.basket.count(treat) == 0
        {
            return None;
        }
        let accepted = crate::garden::likes(animal, treat);
        let id = self.animals[i].id().to_owned();
        if accepted {
            self.garden.basket.take(treat);
            self.note_gift(&id);
            self.events.push(GameEvent::TreatEaten {
                animal: id.clone(),
                treat,
            });
            // a male + female pair at home that gets a liked special food makes a baby
            self.maybe_baby(&id);
        } else {
            self.events
                .push(GameEvent::TreatRefused { animal: id, treat });
        }
        self.group_faces_player(self.animals[i].id());
        Some(accepted)
    }

    /// A pair at home (both members in the enclosure) that was given a special food has its
    /// baby — once per species (GAME-FAMILY "Special food and babies").
    fn maybe_baby(&mut self, id: &str) {
        let group = self.group(id);
        if group.len() >= 2
            && group
                .iter()
                .all(|&j| self.animals[j].state == AnimalState::InEnclosure)
            && !self.babies.iter().any(|b| b == id)
        {
            self.babies.push(id.to_owned());
            self.spawn_baby(id);
            self.events.push(GameEvent::BabyBorn {
                animal: id.to_owned(),
            });
        }
    }

    /// A gift was accepted: the feeding cooldown of the species starts (HINT-026).
    fn note_gift(&mut self, id: &str) {
        self.gift_cooldown.insert(id.to_owned(), GIFT_COOLDOWN_S);
    }

    /// Seconds left of the feeding cooldown of a species (0 = none, GAME-HINT "No hint loops").
    pub fn gift_cooldown_s(&self, id: &str) -> f32 {
        self.gift_cooldown.get(id).copied().unwrap_or(0.0)
    }

    /// Whether a baby is still possible for a species: a pair, both members at home, no baby yet.
    pub fn baby_possible(&self, id: &str) -> bool {
        let group = self.group(id);
        group.len() >= 2
            && group
                .iter()
                .all(|&j| self.animals[j].state == AnimalState::InEnclosure)
            && !self.babies.iter().any(|b| b == id)
    }

    /// Whether the child holds the species' SPECIAL gift that makes its baby (FAM-008/009): a
    /// garden treat it likes, or — for a species that likes no garden treat — its own food.
    pub fn special_gift_in_hand(&self, id: &str) -> bool {
        use crate::garden::{likes, Treat};
        // a species with box-food treats (night_2: snake, chameleon, poison dart frog): its treat
        if let Some(info) = animal_info(id).filter(|i| !i.treats.is_empty()) {
            return self.carry.food().is_some_and(|f| info.is_treat(f));
        }
        if Treat::ALL.iter().any(|&t| likes(id, t)) {
            return Treat::ALL
                .iter()
                .any(|&t| likes(id, t) && self.garden.basket.count(t) > 0);
        }
        self.carry
            .food()
            .zip(animal_info(id))
            .is_some_and(|(f, i)| i.eats(f))
    }

    /// Whether a treat hint for the species is a real task now: a baby is possible, the special
    /// gift is in the hands and the feeding cooldown is over (HINT-026).
    pub fn treat_hint_due(&self, id: &str) -> bool {
        self.baby_possible(id) && self.gift_cooldown_s(id) <= 0.0 && self.special_gift_in_hand(id)
    }

    /// Whether a garden treat is still wanted by a species of the zoo whose baby is possible
    /// (so a plant hint for it is a real task, not a loop).
    pub fn treat_wanted(&self, t: crate::garden::Treat) -> bool {
        self.animals.iter().any(|a| {
            self.in_scope(a) && crate::garden::likes(a.id(), t) && self.baby_possible(a.id())
        })
    }

    /// All members of an animal's group at home turn to the child.
    fn group_faces_player(&mut self, animal: &str) {
        let p = self.player.pos;
        for j in self.group(animal) {
            let a = &mut self.animals[j];
            if a.state == AnimalState::InEnclosure {
                let to = p - a.pos;
                if to.length() > 1e-3 {
                    a.facing = to.normalize();
                }
            }
        }
        if let Some(b) = self.baby_states.get_mut(animal) {
            b.facing = (p - b.pos).normalize_or(b.facing);
        }
    }

    /// What the child can give to an animal at home right now: a liked treat, else the liked
    /// carried food, else (to be refused) any treat, else the carried food. `None` = nothing
    /// in the basket or the hands.
    pub fn gift_for(&self, animal: &str) -> Option<Gift> {
        use crate::garden::{likes, Treat};
        let b = &self.garden.basket;
        let liked = |t: &Treat| b.count(*t) > 0 && likes(animal, *t);
        let liked_treat = self
            .treat_choice
            .filter(liked)
            .or_else(|| Treat::ALL.into_iter().find(liked));
        let food = self.carry.food();
        let food_liked = food.filter(|f| animal_info(animal).is_some_and(|i| i.accepts(*f)));
        liked_treat
            .map(Gift::Treat)
            .or(food_liked.map(Gift::Food))
            .or_else(|| self.offered_treat().map(Gift::Treat))
            .or(food.map(Gift::Food))
    }

    /// Whether the child holds something this animal likes (it then comes to the fence).
    pub fn gift_liked(&self, animal: &str) -> bool {
        match self.gift_for(animal) {
            Some(Gift::Treat(t)) => crate::garden::likes(animal, t),
            Some(Gift::Food(f)) => animal_info(animal).is_some_and(|i| i.accepts(f)),
            None => false,
        }
    }

    /// Gives the carried food to an animal at home (GARD-010): it eats it if it is its food
    /// (both of a pair, hearts), else it refuses. The food stays in the hands (FEED-023); no
    /// baby (Q-198: only special treats). `None` if the animal is not at home or nothing is
    /// carried.
    pub fn give_food(&mut self, animal: &str) -> Option<bool> {
        let i = self.animal_index(animal)?;
        let food = self.carry.food()?;
        if self.animals[i].state != AnimalState::InEnclosure {
            return None;
        }
        let id = self.animals[i].id().to_owned();
        let accepted = self.animals[i].info.accepts(food);
        if accepted {
            self.note_gift(&id);
        }
        self.events.push(if accepted {
            GameEvent::FoodEaten {
                animal: id.clone(),
                food,
            }
        } else {
            GameEvent::FoodRefused {
                animal: id.clone(),
                food,
            }
        });
        // a species that likes no garden treat (koala, lion, snow fox, goldfish, night animals,
        // Q-281) gets its baby from its own favourite food — the special food of that species
        // (user report 2026-10-03: the snow fox did not accept food to make a baby)
        let info = self.animals[i].info;
        if accepted
            && if info.treats.is_empty() {
                !crate::garden::Treat::ALL
                    .iter()
                    .any(|&t| crate::garden::likes(&id, t))
            } else {
                // box-food treats (GAME-FEED "Basic food and treats"): only the treat makes the
                // baby; the basic food gives hearts only
                info.is_treat(food)
            }
        {
            self.maybe_baby(&id);
        }
        self.group_faces_player(&id);
        Some(accepted)
    }

    /// The feeding spot of an enclosure element (GAME-GARDEN §6).
    pub fn feed_spot(&self, enclosure: usize) -> Option<&wander::FeedSpot> {
        self.feed_spots.get(enclosure)?.as_ref()
    }

    /// Target cells (male, female, baby) of animal `i`'s group while the child calls it with
    /// something it likes: the feeding spot when she stands near it, else the cells nearest to
    /// her at the fence. `None` = not called.
    fn gift_call(&self, i: usize) -> Option<[IVec2; 3]> {
        let a = &self.animals[i];
        if a.state != AnimalState::InEnclosure
            || self.is_leading()
            || self.animals[i].area.is_empty()
            || !self.gift_liked(a.id())
        {
            return None;
        }
        let p = self.player.pos;
        let r = self.level.data.elements[a.enclosure].rect;
        let min = Vec2::new(r.x as f32, r.z as f32);
        let max = min + Vec2::new(r.w as f32, r.d as f32);
        if p.distance(p.clamp(min, max)) > wander::GIFT_CALL_M {
            return None;
        }
        if let Some(s) = self.feed_spot(a.enclosure) {
            if p.distance(s.stand) <= wander::FEED_SPOT_CALL_M {
                return Some([s.cell_for(0), s.cell_for(1), s.cell_for(2)]);
            }
        }
        let mut cells: Vec<IVec2> = a.area.cells().map(|(c, _)| c).collect();
        cells.sort_by(|x, y| {
            cell_center(*x)
                .distance_squared(p)
                .total_cmp(&cell_center(*y).distance_squared(p))
        });
        let first = *cells.first()?;
        Some([
            first,
            *cells.get(1).unwrap_or(&first),
            *cells.get(2).unwrap_or(&first),
        ])
    }

    /// Index of the female (second member) of a pair species at home.
    fn female_of(&self, species: &str) -> Option<usize> {
        self.group(species)
            .into_iter()
            .find(|&j| self.animals[j].member == 1)
    }

    /// The cell the baby waits in next to its mother: a neighbouring cell of her area, never
    /// the cell of a parent and never at the fence (≥ 1 m inside where possible).
    fn baby_home_cell(&self, female: usize) -> Option<IVec2> {
        let f = &self.animals[female];
        let r = self.level.data.elements[f.enclosure].rect;
        let inner = |c: IVec2| c.x > r.x && c.x < r.x + r.w - 1 && c.y > r.z && c.y < r.z + r.d - 1;
        let here = cell_of(f.pos);
        let taken: Vec<IVec2> = self
            .group(f.id())
            .into_iter()
            .map(|j| cell_of(self.animals[j].pos))
            .collect();
        f.area
            .cells()
            .map(|(c, _)| c)
            .filter(|c| !taken.contains(c))
            .min_by(|x, y| {
                let key = |c: &IVec2| {
                    let d = cell_center(*c).distance(cell_center(here));
                    (!inner(*c), (d * 100.0) as i32)
                };
                key(x).cmp(&key(y))
            })
    }

    /// The cell the baby stands in beside its mother in any state: at home inside her area
    /// (see [`Game::baby_home_cell`]), else the nearest walkable cell around her (hiding
    /// place, on the way), never a parent's cell.
    fn baby_cell_beside(&self, female: usize) -> Option<IVec2> {
        let f = &self.animals[female];
        if f.state == AnimalState::InEnclosure && !f.area.is_empty() {
            return self.baby_home_cell(female);
        }
        let here = cell_of(f.pos);
        let taken: Vec<IVec2> = self
            .group(f.id())
            .into_iter()
            .map(|j| cell_of(self.animals[j].pos))
            .collect();
        let grid = self.level.grid();
        let mut best: Option<(i32, IVec2)> = None;
        for dx in -4i32..=4 {
            for dy in -4i32..=4 {
                let c = here + IVec2::new(dx, dy);
                if taken.contains(&c) || !grid.is_walkable(c, false) || grid.is_prop_blocked(c) {
                    continue;
                }
                let d = (cell_center(c).distance(f.pos) * 100.0) as i32;
                if best.is_none_or(|(bd, _)| d < bd) {
                    best = Some((d, c));
                }
            }
        }
        Some(best.map_or(here, |(_, c)| c))
    }

    /// Places the baby next to its mother (birth, save restore, safety net): the baby always
    /// is with the female (FAM-011).
    pub(crate) fn spawn_baby(&mut self, species: &str) {
        let Some(f) = self.female_of(species) else {
            return;
        };
        let Some(c) = self.baby_cell_beside(f) else {
            return;
        };
        let facing = self.animals[f].facing;
        self.baby_states.insert(
            species.to_owned(),
            Baby {
                pos: cell_center(c),
                facing,
                route: Vec::new(),
                play: crate::baby::BabyPlay::default(),
            },
        );
    }

    /// The baby is always with the female (FAM-011..013): at home it keeps next to her inside
    /// the fence and comes to the feeding spot with the pair (rank 2); while she is out or
    /// follows the child it walks after her (nav path, faster than she); a baby that is
    /// outside her area while she is home, or far from her, is put beside her.
    fn update_babies(&mut self, dt: f32) {
        let species: Vec<String> = self.baby_states.keys().cloned().collect();
        let p = self.player.pos;
        let dark = self.daytime.is_dark();
        let night_part: Vec<bool> = self.level.data.parts.iter().map(|pt| pt.night).collect();
        for id in species {
            let Some(f) = self.female_of(&id) else {
                continue;
            };
            let state = self.animals[f].state;
            if state == AnimalState::InEnclosure {
                if dark && !night_part[self.animals[f].part] {
                    // the baby sleeps beside its mother at night (user request 2026-10-04,
                    // NIGHT-034): it stands still with her `sleep` clip, no walking, no hop
                    if let Some(b) = self.baby_states.get_mut(&id) {
                        b.route.clear();
                        b.play.mode = crate::baby::Mode::Idle;
                        b.play.hop = 0.0;
                    }
                    continue;
                }
                let inside = self.baby_states.get(&id).is_some_and(|b| {
                    self.animals[f].area.is_empty() || self.animals[f].area.contains(cell_of(b.pos))
                });
                if !inside {
                    self.spawn_baby(&id);
                    continue;
                }
            } else {
                self.baby_walks_after(&id, f, dt);
                continue;
            }
            let call = self.gift_call(f);
            if call.is_none() {
                self.baby_play(&id, f, dt, true);
                continue;
            }
            let target = call.as_ref().map(|c| c[2]);
            let area = &self.animals[f].area;
            let Some(b) = self.baby_states.get_mut(&id) else {
                continue;
            };
            if let Some(t) = target {
                let here = cell_of(b.pos);
                if t != here && b.route.last() != Some(&t) {
                    b.route = area.route(here, t).unwrap_or_default();
                }
            }
            b.play.mode = crate::baby::Mode::Idle;
            b.play.hop = 0.0;
            if b.route.is_empty() {
                b.facing = (p - b.pos).normalize_or(b.facing);
                continue;
            }
            let speed = wander::GIFT_SPEED;
            let (_, dir) = wander::follow_route(&mut b.pos, &mut b.route, speed * dt);
            if dir != Vec2::ZERO {
                b.facing = dir;
            }
        }
    }

    /// The baby of a female that is not at home walks after her (nav path), a little faster
    /// than she; if it is hopelessly far it is put beside her.
    fn baby_walks_after(&mut self, id: &str, f: usize, dt: f32) {
        let far = self
            .baby_states
            .get(id)
            .is_some_and(|b| b.pos.distance(self.animals[f].pos) > crate::baby::TELEPORT_M);
        if far {
            self.spawn_baby(id);
            return;
        }
        self.baby_play(id, f, dt, false);
    }

    /// One step of the playful baby behaviour ([`crate::baby`]) around female `f`; `home`:
    /// she is in her enclosure, so the baby stays inside her wander area.
    fn baby_play(&mut self, id: &str, f: usize, dt: f32, home: bool) {
        let m = &self.animals[f];
        let area = (home && !m.area.is_empty()).then_some(&m.area);
        let ctx = crate::baby::Ctx {
            mother: m.pos,
            mother_facing: m.facing,
            home: area,
            grid: self.level.grid(),
            path_speed: self.move_params.speed_on(Surface::Path),
            grass_speed: self.move_params.speed_on(Surface::Grass),
            dt,
        };
        if let Some(b) = self.baby_states.get_mut(id) {
            crate::baby::step(b, &ctx, &mut self.baby_rng);
        }
    }

    /// Whether a gate / door model should stand open now (LAYOUT-031, GAME-LAYOUT "Gates and
    /// doors"): building doors while the player passes (enterable buildings), enclosure gates
    /// while the player leads animals (or carries one) near them, the garden gate while the
    /// player is within 2 m, the moon door while its barrier is open.
    pub fn opening_open(&self, o: &crate::scene::Opening) -> bool {
        use crate::scene::OpeningKind as K;
        let d = self.player.pos.distance(o.center);
        match &o.kind {
            K::BuildingDoor { enterable, .. } => *enterable && d <= DOOR_OPEN_M,
            K::EnclosureGate { .. } | K::GlassDoor { .. } => {
                (self.is_leading() || self.carrying_animal()) && d <= GATE_OPEN_M
            }
            K::GardenGate { .. } => d <= GARDEN_GATE_OPEN_M,
            K::MoonDoor { barrier } | K::LevelGate { barrier } => {
                self.level.is_barrier_open(barrier)
            }
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
            b.dropped = false;
            b.fish = false;
            b.pos = gate.map_or(self.player.pos, |g| {
                Vec2::new(g.x as f32 + g.w as f32 / 2.0, g.z as f32 + g.d as f32 / 2.0)
            });
            b.lift_m = self.level.ground_height(b.pos).max(0.1);
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
            // a partner that still waits far behind (RESC-006) must catch up first: the pair
            // enters together, else the mission would stay open (GARD-014)
            if self
                .group(id)
                .into_iter()
                .any(|j| self.animals[j].state == AnimalState::Following && self.animals[j].waiting)
            {
                if announce_refuse {
                    self.events.push(GameEvent::Waiting {
                        animal: id.to_owned(),
                    });
                }
                return false;
            }
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
        // lantern posts are solid while they are shown (LAYOUT-035)
        self.level.set_night_solid(self.daytime.lamps_on());
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
        // a species whose members are ALL at home is done, whatever the mission flags say (an
        // old save, a restore, a missed completion): the compass and the hints must never offer
        // a target for it (RESC-033, user report 2026-10-03)
        self.reconcile_s -= dt;
        if self.reconcile_s <= 0.0 {
            self.reconcile_s = 1.0;
            self.reconcile_missions();
        }
        // feet on the surface under her, smoothly (GAME-PLAYER 8, PLAY-035/036)
        let ground = self.level.ground_height(self.player.pos);
        self.player.y = crate::ground::follow(self.player.y, ground, dt);
        if let Some(b) = self.bowl.as_ref().filter(|b| b.carried && b.fish) {
            let animal = b.animal.clone();
            for j in self.group(&animal) {
                self.animals[j].pos = self.player.pos;
                self.animals[j].facing = self.player.facing;
            }
        }
        self.check_gate();
        self.garden.update(dt);
        self.bamboo.update(dt);
        self.update_followers(dt);
        self.update_stall(dt);
        self.update_wander(dt);
        self.update_babies(dt);
        self.gift_cooldown.retain(|_, s| {
            *s -= dt;
            *s > 0.0
        });
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
        // the partner already stands there: take the nearest home cell a pair gap away (Q-308)
        let gap = crate::animals::pair_gap_m(self.animals[i].id());
        let mates: Vec<Vec2> = self
            .group(self.animals[i].id())
            .into_iter()
            .filter(|&j| j != i && self.animals[j].state == AnimalState::InEnclosure)
            .map(|j| self.animals[j].pos)
            .collect();
        let entry = if mates
            .iter()
            .any(|m| m.distance(cell_center(entry)) < gap - 1e-3)
        {
            area.cells()
                .map(|(c, _)| c)
                .filter(|&c| {
                    mates
                        .iter()
                        .all(|m| m.distance(cell_center(c)) >= gap - 1e-3)
                })
                .min_by(|x, y| {
                    cell_center(*x)
                        .distance(cell_center(entry))
                        .total_cmp(&cell_center(*y).distance(cell_center(entry)))
                })
                .unwrap_or(entry)
        } else {
            entry
        };
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
            rest_s: 0.0,
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
    /// Completes the mission of every species whose members are all in their enclosure but whose
    /// flags disagree (RESC-033). Runs about once a second, allocation-free unless it has work.
    fn reconcile_missions(&mut self) {
        let n = self.animals.len();
        for i in 0..n {
            let id = self.animals[i].id();
            let all_home = self
                .animals
                .iter()
                .filter(|a| a.id() == id)
                .all(|a| a.state == AnimalState::InEnclosure);
            if !all_home {
                continue;
            }
            let incomplete = self
                .animals
                .iter()
                .zip(&self.missions)
                .any(|(a, m)| a.id() == id && !m.complete);
            if incomplete {
                self.complete_group(id);
            }
        }
    }

    fn complete_group(&mut self, animal: &str) {
        let group = self.group(animal);
        if group.is_empty()
            || !group
                .iter()
                .all(|&j| self.animals[j].state == AnimalState::InEnclosure)
            || group.iter().all(|&j| self.missions[j].complete)
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
            if night {
                self.open_night_gates(&level_id);
            }
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
        // animals never enter a garden: while the child is inside, they come to its gate
        // and wait outside (proposal Q-102)
        let p = self
            .level
            .data
            .gardens
            .iter()
            .find(|g| !g.animals_enter && g.rect.contains(cell_of(self.player.pos)))
            .map_or(self.player.pos, |g| g.gate_center() + g.gate_out() * 0.9);
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
            if self.stall_walking(i) {
                continue; // on its way to the player (NEVER STUCK)
            }
            if matches!(state, AnimalState::Following | AnimalState::InBowl)
                || self.animals[i].area.is_empty()
                || !self.part_unlocked(self.animals[i].part)
            {
                continue; // locked levels are asleep (not simulated)
            }
            let day_dark =
                state == AnimalState::InEnclosure && dark && !night_part[self.animals[i].part];
            if self.house_step(i, dt, day_dark) {
                continue; // resting or sleeping in its animal house (GAME-HOUSE)
            }
            if day_dark
                && (self.animals[i].wander.route.is_empty()
                    || !self.houses[self.animals[i].enclosure]
                        .as_ref()
                        .is_some_and(|h| h.rest))
            {
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
            if let Some(cells) = self.gift_call(i) {
                // the child holds something it likes: it comes to the feeding spot (or the
                // fence nearest to her) and waits there (GARD-010)
                let a = &mut self.animals[i];
                let rank = usize::from(a.member).min(2);
                let b = cells[rank];
                let here = cell_of(a.pos);
                let near = a.pos.distance(p);
                // (a spot cell with a pair gap beyond the child's 1.4 m is still walked to)
                if (near > wander::GIFT_WAIT_M || cell_center(b).distance(p) >= wander::GIFT_WAIT_M)
                    && cell_center(b).distance(a.pos) > 0.05
                {
                    if a.wander.route.last() != Some(&b) {
                        a.wander.route = if b == here {
                            vec![b] // to the middle of its spot cell
                        } else {
                            a.area.route(here, b).unwrap_or_default()
                        };
                    }
                    a.wander.pause_s = 1.0;
                    let (_, dir) = wander::follow_route(
                        &mut a.pos,
                        &mut a.wander.route,
                        wander::GIFT_SPEED * dt,
                    );
                    if dir != Vec2::ZERO {
                        a.facing = dir;
                    }
                } else {
                    a.wander.route.clear();
                    a.facing = (p - a.pos).normalize_or(a.facing);
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
                // a pair keeps its distance: only cells a pair gap from the partner (Q-308)
                let mates = self.mate_points(i);
                let pair_gap = crate::animals::pair_gap_m(self.animals[i].id());
                let house = if escaped {
                    None
                } else {
                    self.houses[self.animals[i].enclosure]
                        .as_ref()
                        .filter(|h| h.rest)
                };
                // 2 of 10 targets are a free interior cell of the animal house (HOUSE rule 1);
                // with a pool they replace land targets, the water share stays 7 of 10
                let pool = water_bias
                    && self.animals[i]
                        .area
                        .cells()
                        .any(|(_, k)| k == wander::AreaCell::Water);
                let roll = if house.is_some() {
                    self.rng.below(10)
                } else {
                    10
                };
                let (want_house, class) = match house {
                    Some(_) if pool => (
                        (7..9).contains(&roll),
                        match roll {
                            0..=6 => Some(wander::AreaCell::Water),
                            7..=8 => None,
                            _ => Some(wander::AreaCell::Land),
                        },
                    ),
                    Some(_) => (roll < crate::house::HOUSE_TARGETS_OF_10, None),
                    None => (false, None),
                };
                let mut in_house = None;
                if let (Some(h), true) = (house, want_house) {
                    let free: Vec<IVec2> = h
                        .interior
                        .cells()
                        .filter(|&c| {
                            c != here
                                && !mates.iter().any(|m| cell_of(*m) == c)
                                && self.animals[i].area.contains(c)
                        })
                        .collect();
                    if !free.is_empty() {
                        in_house = Some(free[self.rng.below(free.len() as u32) as usize]);
                    }
                }
                let keep_bias = house.is_none();
                let target = in_house.or_else(|| {
                    let area = &self.animals[i].area;
                    wander::draw_target_where(
                        area,
                        here,
                        water_bias && keep_bias,
                        true,
                        &|c| {
                            !house.is_some_and(|h| h.is_interior(c) || h.is_door(c))
                                && class.is_none_or(|k| area.class(c) == Some(k))
                                && !mates
                                    .iter()
                                    .any(|m| m.distance(cell_center(c)) < pair_gap - 1e-3)
                        },
                        &mut self.rng,
                    )
                });
                let a = &mut self.animals[i];
                a.wander.pause_s = wander::draw_pause(&mut self.rng);
                if in_house.is_some() {
                    a.wander.rest_s = crate::house::draw_rest(&mut self.rng);
                    a.wander.pause_s = 0.0;
                }
                if let Some(t) = target {
                    a.wander.route = a.area.route(here, t).unwrap_or_default();
                }
                continue;
            }
            let before = a.pos;
            // (at dusk the animals hurry into their house)
            let speed = if day_dark {
                wander::GIFT_SPEED
            } else {
                wander::WANDER_SPEED
            };
            let (_, dir) = wander::follow_route(&mut a.pos, &mut a.wander.route, speed * dt);
            if dir != Vec2::ZERO {
                a.facing = dir;
            }
            // never walk into the partner: stop where the gap would be undercut (Q-308)
            let now = self.animals[i].pos;
            // (an animal that comes to the player may close up to 60 % of the gap)
            let relax = if escaped && p.distance(now) <= wander::NOTICE_PLAYER_M {
                0.6
            } else {
                1.0
            };
            // (coming to the player, only the partner's real position counts, not its goal)
            let by_pos = |q: Vec2| {
                self.animals
                    .iter()
                    .enumerate()
                    .filter(|&(j, m)| {
                        j != i
                            && m.id() == self.animals[i].id()
                            && m.state == self.animals[i].state
                            && matches!(m.state, AnimalState::Escaped | AnimalState::InEnclosure)
                    })
                    .map(|(_, m)| m.pos.distance(q))
                    .fold(f32::INFINITY, f32::min)
            };
            let (gap_now, gap_before) = if relax < 1.0 {
                (by_pos(now), by_pos(before))
            } else {
                (self.mate_gap(i, now), self.mate_gap(i, before))
            };
            // (at a house — footprint and door front — the pair gap is waived, GAME-HOUSE rule 4)
            let at_house = |q: Vec2| {
                self.houses[self.animals[i].enclosure]
                    .as_ref()
                    .is_some_and(|h| h.near(cell_of(q)))
            };
            if now != before
                && !(at_house(now) || at_house(before))
                && gap_now < crate::animals::pair_gap_m(self.animals[i].id()) * relax - 1e-3
                && gap_now < gap_before
            {
                let a = &mut self.animals[i];
                a.pos = before;
                a.wander.route.clear();
                a.wander.pause_s = 1.0;
            }
        }
    }

    /// Whether the roof of animal house `id` is cut away: an animal rests inside and the
    /// player is within 8 m of the door (GAME-HOUSE rule 6, HOUSE-014).
    pub fn house_cutaway(&self, id: &str) -> bool {
        let Some(h) = self.houses.iter().flatten().find(|h| h.id == id) else {
            return false;
        };
        let inside = self.animals.iter().any(|a| {
            a.state == AnimalState::InEnclosure
                && self.level.data.elements[a.enclosure].id == h.enclosure
                && h.is_interior(cell_of(a.pos))
        });
        h.cutaway(self.player.pos, inside)
    }

    /// Animal house behaviour of animal `i` at home (GAME-HOUSE rules 1–3): comes out when the
    /// child is near, goes in at dusk, rests inside. Returns true when the animal rests or
    /// sleeps and needs no further wander handling this tick.
    fn house_step(&mut self, i: usize, dt: f32, day_dark: bool) -> bool {
        let a = &self.animals[i];
        if a.state != AnimalState::InEnclosure || a.area.is_empty() {
            return false;
        }
        let Some(h) = self.houses[a.enclosure].as_ref().filter(|h| h.rest) else {
            return false;
        };
        let p = self.player.pos;
        let here = cell_of(a.pos);
        let inside = h.is_interior(here) || h.is_door(here);
        let gate_near = self.level.data.elements[a.enclosure]
            .gate
            .is_some_and(|g| g.distance_to(p) <= wander::NOTICE_PLAYER_M);
        let child_near = gate_near || p.distance(h.door_front_center()) <= wander::NOTICE_PLAYER_M;
        let a = &mut self.animals[i];
        if inside && child_near {
            // never hide an animal the child needs: walk out over the door front
            let leaving = a
                .wander
                .route
                .last()
                .is_some_and(|&c| !h.footprint.contains(c));
            if !leaving {
                let front = h
                    .door_front()
                    .into_iter()
                    .filter(|&c| a.area.contains(c))
                    .min_by(|x, y| {
                        cell_center(*x)
                            .distance(a.pos)
                            .total_cmp(&cell_center(*y).distance(a.pos))
                    });
                if let Some(f) = front {
                    a.wander.route = a.area.route(here, f).unwrap_or_default();
                }
                a.wander.rest_s = 0.0;
                a.wander.pause_s = 1.0;
            }
            return false;
        }
        if day_dark {
            if !a.wander.route.is_empty() || child_near {
                return false; // still walking (the caller keeps walking an unfinished route)
            }
            if inside {
                a.wander.rest_s = crate::house::REST_UNTIL_MORNING_S;
                return true;
            }
            // dusk: walk into the house, a pair onto different cells
            let mates = self.mate_points(i);
            let a = &self.animals[i];
            let free: Vec<IVec2> = h
                .interior
                .cells()
                .filter(|&c| a.area.contains(c) && !mates.iter().any(|m| cell_of(*m) == c))
                .collect();
            if free.is_empty() {
                return false;
            }
            let t = free[self.rng.below(free.len() as u32) as usize];
            let a = &mut self.animals[i];
            a.wander.route = a.area.route(here, t).unwrap_or_default();
            a.wander.rest_s = crate::house::REST_UNTIL_MORNING_S;
            a.wander.pause_s = 0.0;
            return false;
        }
        if a.wander.rest_s > 0.0 && a.wander.route.is_empty() && h.is_interior(here) {
            if a.wander.rest_s > crate::house::REST_RANGE_S.1 {
                // woke up in the morning: a short rest, then on with the day
                a.wander.rest_s = crate::house::draw_rest(&mut self.rng);
            }
            a.wander.rest_s -= dt;
            if a.wander.rest_s <= 0.0 {
                a.wander.rest_s = 0.0;
                return false;
            }
            return true;
        }
        false
    }

    /// Where the partner(s) of animal `i` are or are heading (same pair, same state; empty for
    /// a single animal or a partner that follows / is carried).
    fn mate_points(&self, i: usize) -> Vec<Vec2> {
        let a = &self.animals[i];
        let mut out = Vec::new();
        for (j, m) in self.animals.iter().enumerate() {
            if j != i
                && m.id() == a.id()
                && m.state == a.state
                && matches!(m.state, AnimalState::Escaped | AnimalState::InEnclosure)
            {
                out.push(m.pos);
                if let Some(&c) = m.wander.route.last() {
                    out.push(cell_center(c)); // a partner on its way counts with its destination
                }
            }
        }
        out
    }

    /// Distance to the nearest partner point (infinite for a single animal).
    fn mate_gap(&self, i: usize, p: Vec2) -> f32 {
        self.mate_points(i)
            .iter()
            .map(|m| m.distance(p))
            .fold(f32::INFINITY, f32::min)
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
        let id = self.animals[i].id();
        // the whole group (a pair may be split: one at home, the partner out)
        if self
            .group(id)
            .iter()
            .all(|&j| self.animals[j].state == AnimalState::InEnclosure)
        {
            return false;
        }
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

    /// Seconds of play without mission progress while a mission is open (NEVER STUCK).
    pub fn stall_s(&self) -> f32 {
        self.stall_s
    }

    /// Whether any mission of an unlocked level is still open.
    pub fn any_mission_open(&self) -> bool {
        self.animals
            .iter()
            .enumerate()
            .any(|(i, a)| self.in_scope(a) && !self.missions[i].complete)
    }

    /// Whether escaped animal `i` currently walks to the player (stall rescue).
    pub(crate) fn stall_walking(&self, i: usize) -> bool {
        self.stall_s >= STALL_WALK_S
            && self.animals[i].state == AnimalState::Escaped
            && self.in_scope(&self.animals[i])
            && !self.missions[i].complete
    }

    /// The hint signature (GAME-HINT "No hint loops"): the mission progress plus the basket
    /// and the time of day — the state a hint action must change (giving a gift that only
    /// brings hearts is NOT a change).
    pub fn hint_signature(&self) -> u64 {
        let mut h = self.progress_signature();
        let mut mix = |v: u64| h = (h ^ v).wrapping_mul(1099511628211);
        let b = &self.garden.basket;
        for t in crate::garden::Treat::ALL {
            mix(u64::from(b.count(t)));
        }
        mix(self.daytime.phase as u64 + 7);
        mix(u64::from(self.daytime.dusk_in.is_some()));
        mix(self.daytime.nightfalls.len() as u64);
        h
    }

    fn progress_signature(&self) -> u64 {
        let mut h: u64 = 1469598103934665603;
        let mut mix = |v: u64| h = (h ^ v).wrapping_mul(1099511628211);
        for (a, m) in self.animals.iter().zip(&self.missions) {
            mix(a.state as u64 + 1);
            mix(u64::from(a.waiting) + 2 * u64::from(m.started) + 4 * u64::from(m.complete));
        }
        mix(self.carry.food().map_or(0, |f| 1 + f as u64));
        mix(self.lying.foods.len() as u64);
        mix(self.babies.len() as u64);
        h
    }

    /// NEVER STUCK safety net: counts play time without mission progress while a mission is
    /// open; after [`STALL_WALK_S`] the escaped animals of open missions walk to a cell near
    /// the player (the hint points at them from [`STALL_HELP_S`]).
    fn update_stall(&mut self, dt: f32) {
        let sig = self.progress_signature();
        if sig != self.stall_sig || !self.any_mission_open() || self.daytime.is_dark() {
            if sig != self.stall_sig || !self.any_mission_open() {
                self.stall_s = 0.0;
            }
            self.stall_sig = sig;
            self.stall_routes.clear();
            return;
        }
        self.stall_s += dt;
        if self.stall_s < STALL_WALK_S {
            return;
        }
        self.stall_routes.resize(self.animals.len(), Vec::new());
        let p = self.player.pos;
        for i in 0..self.animals.len() {
            if !self.stall_walking(i) {
                continue;
            }
            let pos = self.animals[i].pos;
            if self.stall_routes[i].is_empty() && pos.distance(p) > STALL_COME_M {
                let from = cell_of(pos);
                let pc = cell_of(p);
                let mut cells: Vec<IVec2> = (-6..=6)
                    .flat_map(|dx| (-6..=6).map(move |dz| pc + IVec2::new(dx, dz)))
                    .filter(|&c| {
                        let d = cell_center(c).distance(p);
                        (3.0..=STALL_COME_M).contains(&d) && self.level.grid().is_walkable(c, false)
                    })
                    .collect();
                cells.sort_by(|a, b| {
                    cell_center(*a)
                        .distance(pos)
                        .total_cmp(&cell_center(*b).distance(pos))
                        .then(a.x.cmp(&b.x))
                        .then(a.y.cmp(&b.y))
                });
                let route = cells
                    .into_iter()
                    .take(12)
                    .find_map(|c| crate::nav::find_path(self.level.grid(), from, c, false));
                self.stall_routes[i] = route.unwrap_or_default();
            }
            let mut route = std::mem::take(&mut self.stall_routes[i]);
            let a = &mut self.animals[i];
            let (_, dir) = wander::follow_route(&mut a.pos, &mut route, STALL_SPEED * dt);
            if dir != Vec2::ZERO {
                a.facing = dir;
            }
            self.stall_routes[i] = route;
        }
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
