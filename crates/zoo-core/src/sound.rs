//! Sound decisions (ART-SOUND "Playback"): which cue plays, where and how loud. Pure and
//! seeded; the host (Web Audio) only loads and plays the named cue. No audio data here.

use std::collections::HashMap;

use glam::Vec2;

use crate::game::{Game, GameEvent};
use crate::level::{cell_of, Level, Surface};
use crate::rng::Pcg32;
use crate::scene::OpeningKind;
use crate::AnimalState;

/// Master gain of all sound effects. The files are normalised to −16 LUFS, which is loud on
/// phones, so everything is attenuated (user decision 2026-09-30, ASND-010).
pub const MASTER_GAIN: f32 = 0.35;
/// Peak gain of any cue (distance ≤ [`FULL_RANGE_M`]) must stay at or below this (ASND-010).
pub const MAX_PEAK_GAIN: f32 = 0.5;
/// Within this distance of the player a sound is at full volume (m).
pub const FULL_RANGE_M: f32 = 2.0;
/// From this distance on a sound is silent (m).
pub const SILENT_RANGE_M: f32 = 15.0;
/// Clip time between two footfalls of the `walk` clip (`footstep_l` at 0, `footstep_r` at 0.4 s).
pub const STEP_HALF_CYCLE_S: f32 = 0.4;
/// An escaped animal notices the player within this distance (m).
pub const NOTICE_RANGE_M: f32 = 8.0;
/// Same animal calls at most this often (s).
pub const NOTICE_COOLDOWN_S: f32 = 20.0;
/// Ground raised at least this much under the player = wooden (bridge, jetty, floor).
pub const WOOD_MIN_HEIGHT_M: f32 = 0.08;
/// Pitch variation of a played cue (±5 %).
pub const PITCH_VARIATION: f32 = 0.05;

/// Final gain of the night cricket bed (ASND-022): no master / group factor on top, the file is
/// −22 LUFS so the bed plays at about −40 LUFS; "not too loud" (user request 2026-10-01).
pub const AMBIENT_GAIN: f32 = 0.05;
/// The bed fades in and out over this long (s).
pub const AMBIENT_FADE_S: f32 = 3.0;

/// Target gain of the ambient cricket loop: quiet crickets at dusk and night (day or night
/// zoo), none by day, while sleeping and in the morning (ASND-022, Q-222).
pub fn ambient_target(phase: crate::daytime::Phase) -> f32 {
    use crate::daytime::Phase;
    match phase {
        Phase::Dusk | Phase::Night => AMBIENT_GAIN,
        Phase::Day | Phase::Sleeping | Phase::Morning => 0.0,
    }
}

/// The non-animal cue ids (ASND list); animal cues are `animal_<species>_call|happy|refuse`.
pub const CUES: &[&str] = &[
    "step_path",
    "step_grass",
    "step_sand",
    "step_wood",
    "step_water",
    "door_wood_open",
    "door_wood_close",
    "gate_open",
    "gate_close",
    "glass_door",
    "moon_door",
    "pickup_food",
    "drop_food",
    "pickup_item",
    "drop_item",
    "harvest_plant",
    "basket_add",
    "ui_tap",
    "ui_refuse",
    "ui_success",
];

/// Loudness group of a cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Ui,
    Steps,
    Animals,
    Doors,
    Pickups,
}

impl Group {
    /// Group gain (ui ×0.6, steps ×0.5, animals ×0.8; doors ×0.8, pickups ×0.7).
    pub fn gain(self) -> f32 {
        match self {
            Group::Ui => 0.6,
            Group::Steps => 0.5,
            Group::Animals => 0.8,
            Group::Doors => 0.8,
            Group::Pickups => 0.7,
        }
    }
}

/// Loudness group of a cue id.
pub fn group_of(cue: &str) -> Group {
    if cue.starts_with("animal_") {
        Group::Animals
    } else if cue.starts_with("step_") {
        Group::Steps
    } else if cue.starts_with("ui_") {
        Group::Ui
    } else if cue.starts_with("door_")
        || cue.starts_with("gate_")
        || cue.starts_with("glass_")
        || cue.starts_with("moon_")
    {
        Group::Doors
    } else {
        Group::Pickups
    }
}

/// Gain of a cue heard at full volume (master × group).
pub fn peak_gain(cue: &str) -> f32 {
    MASTER_GAIN * group_of(cue).gain()
}

/// Distance law: 1 within 2 m of the player, 0 from 15 m, linear in between.
pub fn distance_gain(d: f32) -> f32 {
    ((SILENT_RANGE_M - d) / (SILENT_RANGE_M - FULL_RANGE_M)).clamp(0.0, 1.0)
}

/// Final gain of a cue heard at `distance` metres from the player.
pub fn cue_gain(cue: &str, distance: f32) -> f32 {
    peak_gain(cue) * distance_gain(distance)
}

/// Seeded variation of one played cue: a number the host maps to a variation file
/// (`v mod n`) and a pitch rate in 0.95…1.05.
pub fn variation(rng: &mut Pcg32) -> (u32, f32) {
    let v = rng.next_u32();
    let k = rng.below(1001) as f32 / 1000.0; // 0…1
    (v, 1.0 - PITCH_VARIATION + 2.0 * PITCH_VARIATION * k)
}

/// One footfall per half cycle of the `walk` clip, only while moving (ASND-011).
#[derive(Debug, Clone, Default)]
pub struct FootfallClock {
    /// Clip seconds until the next footfall (≤ 0: fires now).
    until: f32,
}

impl FootfallClock {
    /// Advances by `clip_dt` clip seconds (frame time × clip rate); true on a footfall. Standing
    /// resets, so the next walk starts with a step at once.
    pub fn advance(&mut self, clip_dt: f32, moving: bool) -> bool {
        if !moving {
            self.until = 0.0;
            return false;
        }
        let fire = self.until <= 0.0;
        if fire {
            self.until += STEP_HALF_CYCLE_S;
        }
        self.until -= clip_dt;
        fire
    }
}

/// Ground the player's feet are on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepSurface {
    Path,
    Grass,
    Sand,
    Wood,
    Water,
}

impl StepSurface {
    pub fn cue(self) -> &'static str {
        match self {
            StepSurface::Path => "step_path",
            StepSurface::Grass => "step_grass",
            StepSurface::Sand => "step_sand",
            StepSurface::Wood => "step_wood",
            StepSurface::Water => "step_water",
        }
    }
}

/// Surface under a level point: pool shallows → water, raised ground (bridge, jetty, floor)
/// → wood, a `sand` landmark → sand, else the grid's path / grass (ASND-012).
pub fn step_surface(level: &Level, p: Vec2) -> StepSurface {
    if crate::wander::water_depth(level, p) > 0.05 {
        return StepSurface::Water;
    }
    if level.ground_height(p) >= WOOD_MIN_HEIGHT_M {
        return StepSurface::Wood;
    }
    let c = cell_of(p);
    let sandy = |features: &[String]| features.iter().any(|f| f == "sand");
    if level
        .data
        .elements
        .iter()
        .any(|e| sandy(&e.features) && e.rect.contains(c))
        || level
            .data
            .hiding_places
            .iter()
            .any(|h| sandy(&h.features) && h.rect.contains(c))
    {
        return StepSurface::Sand;
    }
    match level.grid().surface(c) {
        Some(Surface::Path) => StepSurface::Path,
        _ => StepSurface::Grass,
    }
}

/// A cue to play; `animal` names the animal whose position is the source (else the event's
/// place: the player).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CueRequest {
    pub cue: String,
    pub animal: Option<String>,
}

fn plain(cue: &str) -> CueRequest {
    CueRequest {
        cue: cue.to_owned(),
        animal: None,
    }
}

fn animal(animal: &str, what: &str) -> CueRequest {
    CueRequest {
        cue: format!("animal_{animal}_{what}"),
        animal: Some(animal.to_owned()),
    }
}

/// Cues of a game event (ASND-013); most events have none.
pub fn cues_for_event(e: &GameEvent) -> Vec<CueRequest> {
    match e {
        GameEvent::FoodTaken { .. } => vec![plain("pickup_food")],
        GameEvent::FoodPutBack { .. } => vec![plain("drop_food")],
        GameEvent::ItemTaken { .. } | GameEvent::BambooCut { .. } => vec![plain("pickup_item")],
        GameEvent::ItemPutDown { id } => {
            vec![plain(if id.starts_with("food:") {
                "drop_food"
            } else {
                "drop_item"
            })]
        }
        GameEvent::Harvested { .. } => vec![plain("harvest_plant"), plain("basket_add")],
        GameEvent::NotInterested { animal: a } | GameEvent::Refuse { animal: a, .. } => {
            vec![plain("ui_refuse"), animal(a, "refuse")]
        }
        GameEvent::TreatRefused { animal: a, .. } | GameEvent::FoodRefused { animal: a, .. } => {
            vec![plain("ui_refuse"), animal(a, "refuse")]
        }
        GameEvent::PutDownRefused | GameEvent::BasketFull => vec![plain("ui_refuse")],
        GameEvent::MissionComplete { .. } | GameEvent::LevelComplete { .. } => {
            vec![plain("ui_success")]
        }
        GameEvent::StartedFollowing { animal: a }
        | GameEvent::TreatEaten { animal: a, .. }
        | GameEvent::FoodEaten { animal: a, .. }
        | GameEvent::BabyBorn { animal: a }
        | GameEvent::InEnclosure { animal: a } => vec![animal(a, "happy")],
        _ => vec![],
    }
}

/// Cue of a gate / door whose visible state flips to `open` (ASND-014).
pub fn door_cue(kind: &OpeningKind, open: bool) -> &'static str {
    match kind {
        OpeningKind::BuildingDoor { .. } => {
            if open {
                "door_wood_open"
            } else {
                "door_wood_close"
            }
        }
        OpeningKind::EnclosureGate { .. }
        | OpeningKind::GardenGate { .. }
        | OpeningKind::LevelGate { .. } => {
            if open {
                "gate_open"
            } else {
                "gate_close"
            }
        }
        OpeningKind::GlassDoor { .. } => "glass_door",
        OpeningKind::MoonDoor { .. } => "moon_door",
    }
}

/// An escaped animal notices the player: one call when it comes within
/// [`NOTICE_RANGE_M`], at most every [`NOTICE_COOLDOWN_S`] per animal (ASND-015).
#[derive(Debug, Clone, Default)]
pub struct NoticeTracker {
    cooldown: HashMap<&'static str, f32>,
    inside: HashMap<&'static str, bool>,
}

impl NoticeTracker {
    /// Ids of the animals that noticed the player in this step.
    pub fn update(&mut self, game: &Game, dt: f32) -> Vec<&'static str> {
        for c in self.cooldown.values_mut() {
            *c = (*c - dt).max(0.0);
        }
        let mut out = Vec::new();
        for a in &game.animals {
            let id = a.id();
            let near = a.state == AnimalState::Escaped
                && game.in_scope(a)
                && a.pos.distance(game.player.pos) <= NOTICE_RANGE_M;
            let was = self.inside.insert(id, near).unwrap_or(false);
            let ready = self.cooldown.get(id).copied().unwrap_or(0.0) <= 0.0;
            if near && !was && ready {
                self.cooldown.insert(id, NOTICE_COOLDOWN_S);
                out.push(id);
            }
        }
        out
    }
}
