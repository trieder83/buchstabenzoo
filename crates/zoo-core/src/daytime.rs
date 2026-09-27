//! Day and night (GAME-NIGHT): the time of day as a small state machine, driven by `dt`.
//!
//! `day` → (a day level's last celebration, [`CELEBRATION_S`]) → `dusk` ([`DUSK_S`]: warm
//! orange, then deep blue) → `night` (moon door open, bed usable) → `sleeping` (the child
//! lies down, [`SLEEP_S`] fade) → `morning` ([`MORNING_S`]: the light comes back; the exit
//! barriers of the levels completed before the night open — replaces the temporary rule of
//! Q-091) → `day`.
//!
//! Pure bookkeeping: [`crate::game::Game`] reacts to the returned [`Change`]s (barriers,
//! events, autosave).

use serde::{Deserialize, Serialize};

/// The host's mission celebration (confetti) lasts this long; dusk starts after it
/// (NIGHT-001: "after the celebration").
pub const CELEBRATION_S: f32 = 7.0;
/// Dusk: from the end of the celebration to full night (NIGHT-001: within 10–12 s).
pub const DUSK_S: f32 = 10.5;
/// Sleeping: the child lies down, the screen fades to a soft dream and back (NIGHT-003).
pub const SLEEP_S: f32 = 2.5;
/// Morning: the night light fades back to day.
pub const MORNING_S: f32 = 3.0;

/// Time of day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    #[default]
    Day,
    Dusk,
    Night,
    Sleeping,
    Morning,
}

impl Phase {
    pub const ALL: [Phase; 5] = [
        Phase::Day,
        Phase::Dusk,
        Phase::Night,
        Phase::Sleeping,
        Phase::Morning,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Phase::Day => "day",
            Phase::Dusk => "dusk",
            Phase::Night => "night",
            Phase::Sleeping => "sleeping",
            Phase::Morning => "morning",
        }
    }

    pub fn from_id(id: &str) -> Option<Phase> {
        Phase::ALL.into_iter().find(|p| p.id() == id)
    }
}

/// Whether a level id is a night level (`night_1`, … — GAME-NIGHT rule 4).
pub fn is_night_level(level_id: &str) -> bool {
    level_id.starts_with("night")
}

/// What changed in one [`Daytime::update`] step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    DuskStarted,
    NightFell,
    /// Sleeping is over: the morning starts (open the pending barriers, autosave).
    Woke,
    DayStarted,
}

/// The time of day and what waits for the next night / morning (saved, GAME-SAVE).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Daytime {
    pub phase: Phase,
    /// Seconds spent in the current phase.
    pub phase_s: f32,
    /// Countdown to dusk (the celebration still plays), if nightfall is due.
    pub dusk_in: Option<f32>,
    /// Day levels whose nightfall has happened / is scheduled (once per level, rule 1).
    pub nightfalls: Vec<String>,
    /// Completed levels whose exit barriers open the next morning (Q-091).
    pub pending_exits: Vec<String>,
    /// Nights so far (statistics).
    pub nights: u32,
    /// Sleeping by day ("until the evening", proposal Q-140): the sleep ends in dusk.
    #[serde(default)]
    pub until_evening: bool,
}

/// Light of the time of day for the renderer: `night` 0 … 1 and the warm dusk glow.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Light {
    pub night: f32,
    pub warm: f32,
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Daytime {
    /// A level's missions are all complete: its exits open the next morning; a day level
    /// schedules nightfall after the celebration (once per level). Returns whether
    /// nightfall was scheduled.
    pub fn level_complete(&mut self, level_id: &str, night_level: bool) -> bool {
        if !self.pending_exits.iter().any(|l| l == level_id) {
            self.pending_exits.push(level_id.to_owned());
        }
        if night_level || self.nightfalls.iter().any(|l| l == level_id) {
            return false;
        }
        self.nightfalls.push(level_id.to_owned());
        if self.phase == Phase::Day && self.dusk_in.is_none() {
            self.dusk_in = Some(CELEBRATION_S);
        }
        true
    }

    /// Sleeps by day until the evening (proposal Q-140: an unfinished night zoo is waiting).
    pub fn sleep_until_evening(&mut self) -> bool {
        if self.phase != Phase::Day || self.dusk_in.is_some() {
            return false;
        }
        self.until_evening = true;
        self.set(Phase::Sleeping);
        true
    }

    /// Whether it is night (moon door open, bed usable, NIGHT-002).
    pub fn is_night(&self) -> bool {
        self.phase == Phase::Night
    }

    /// The night-only lamp props (lantern posts, string lights) are shown and their glow
    /// slots lit (dusk on, GAME-NIGHT rule 1); lantern posts are solid while shown
    /// (LAYOUT-035).
    pub fn lamps_on(&self) -> bool {
        self.light().night > 0.2
    }

    /// Night light is on (dusk, night, sleeping, morning): lamps, lantern, eyes.
    pub fn is_dark(&self) -> bool {
        self.light().night > 0.3
    }

    fn set(&mut self, phase: Phase) {
        self.phase = phase;
        self.phase_s = 0.0;
    }

    /// Advances the time of day.
    pub fn update(&mut self, dt: f32, out: &mut Vec<Change>) {
        if let Some(t) = &mut self.dusk_in {
            *t -= dt;
            if *t <= 0.0 {
                self.dusk_in = None;
                if self.phase == Phase::Day {
                    self.set(Phase::Dusk);
                    out.push(Change::DuskStarted);
                }
            }
            return;
        }
        self.phase_s += dt;
        match self.phase {
            Phase::Dusk if self.phase_s >= DUSK_S => {
                self.set(Phase::Night);
                self.nights += 1;
                out.push(Change::NightFell);
            }
            Phase::Sleeping if self.phase_s >= SLEEP_S && self.until_evening => {
                // woke up in the evening: the light fades to night (Q-140)
                self.until_evening = false;
                self.set(Phase::Dusk);
                self.phase_s = DUSK_S * 0.5;
                out.push(Change::DuskStarted);
            }
            Phase::Sleeping if self.phase_s >= SLEEP_S => {
                self.set(Phase::Morning);
                out.push(Change::Woke);
            }
            Phase::Morning if self.phase_s >= MORNING_S => {
                self.set(Phase::Day);
                out.push(Change::DayStarted);
            }
            _ => {}
        }
    }

    /// The child goes to bed (only at night, NIGHT-003). Returns whether sleep started.
    pub fn sleep(&mut self) -> bool {
        if self.phase != Phase::Night {
            return false;
        }
        self.set(Phase::Sleeping);
        true
    }

    /// Jumps to a phase (debug / e2e, restore). Night counts a night; the others clear a
    /// pending dusk.
    pub fn force(&mut self, phase: Phase) {
        self.dusk_in = None;
        self.until_evening = false;
        if phase == Phase::Night && self.phase != Phase::Night {
            self.nights += 1;
        }
        self.set(phase);
    }

    /// Exits to open this morning (taken).
    pub fn take_pending_exits(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_exits)
    }

    /// Light of the time of day: dusk turns warm orange first, then deep blue; the morning
    /// fades back with a little warm glow; sleeping stays at night light (the host fades the
    /// screen).
    pub fn light(&self) -> Light {
        match self.phase {
            Phase::Day => Light::default(),
            Phase::Dusk => {
                let t = (self.phase_s / DUSK_S).clamp(0.0, 1.0);
                Light {
                    night: smooth((t - 0.3) / 0.7),
                    warm: (std::f32::consts::PI * (t * 1.25).min(1.0)).sin().max(0.0),
                }
            }
            Phase::Sleeping if self.until_evening => Light::default(),
            Phase::Night | Phase::Sleeping => Light {
                night: 1.0,
                warm: 0.0,
            },
            Phase::Morning => {
                let t = (self.phase_s / MORNING_S).clamp(0.0, 1.0);
                Light {
                    night: 1.0 - smooth(t),
                    warm: 0.6 * (std::f32::consts::PI * t).sin(),
                }
            }
        }
    }

    /// Dream fade of the sleep (0 … 1 … 0) for the host overlay.
    pub fn sleep_fade(&self) -> f32 {
        match self.phase {
            Phase::Sleeping => {
                let t = (self.phase_s / SLEEP_S).clamp(0.0, 1.0);
                (t / 0.3).min(1.0)
            }
            Phase::Morning => 1.0 - (self.phase_s / 0.8).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }

    /// Seconds from the end of the celebration until night (NIGHT-001), for tests.
    pub fn dusk_duration() -> f32 {
        DUSK_S
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(d: &mut Daytime, seconds: f32) -> Vec<(f32, Change)> {
        let mut out = Vec::new();
        let mut t = 0.0;
        let dt = 1.0 / 60.0;
        let mut ch = Vec::new();
        while t < seconds {
            d.update(dt, &mut ch);
            t += dt;
            out.extend(ch.drain(..).map(|c| (t, c)));
        }
        out
    }

    #[test]
    fn night_001_dusk_after_the_celebration_then_night_within_10_to_12_s() {
        let mut d = Daytime::default();
        assert!(d.level_complete("level_1", false));
        let ch = run(&mut d, 40.0);
        let dusk = ch.iter().find(|c| c.1 == Change::DuskStarted).unwrap().0;
        let night = ch.iter().find(|c| c.1 == Change::NightFell).unwrap().0;
        assert!(
            (dusk - CELEBRATION_S).abs() < 0.05,
            "not earlier than the end of the celebration: {dusk}"
        );
        assert!((10.0..=12.0).contains(&(night - dusk)), "{}", night - dusk);
        assert_eq!(ch.iter().filter(|c| c.1 == Change::DuskStarted).count(), 1);
        assert!(d.is_night());
        // once per level
        assert!(!d.level_complete("level_1", false));
    }

    #[test]
    fn night_levels_do_not_bring_nightfall_but_open_next_morning() {
        let mut d = Daytime::default();
        assert!(!d.level_complete("night_1", true));
        assert_eq!(d.dusk_in, None);
        assert_eq!(d.pending_exits, ["night_1"]);
    }

    #[test]
    fn sleep_only_at_night_then_morning_then_day() {
        let mut d = Daytime::default();
        assert!(!d.sleep());
        d.force(Phase::Night);
        assert!(d.sleep());
        let ch = run(&mut d, SLEEP_S + MORNING_S + 0.5);
        assert_eq!(
            ch.iter().map(|c| c.1).collect::<Vec<_>>(),
            [Change::Woke, Change::DayStarted]
        );
        assert_eq!(d.phase, Phase::Day);
    }

    #[test]
    fn light_goes_warm_then_blue() {
        let mut d = Daytime::default();
        d.force(Phase::Dusk);
        d.phase_s = DUSK_S * 0.25;
        let early = d.light();
        assert!(early.warm > 0.6 && early.night < 0.1, "{early:?}");
        d.phase_s = DUSK_S * 0.95;
        let late = d.light();
        assert!(late.night > 0.9 && late.warm < 0.3, "{late:?}");
        d.force(Phase::Night);
        assert_eq!(d.light().night, 1.0);
        d.force(Phase::Day);
        assert_eq!(d.light(), Light::default());
    }
}
