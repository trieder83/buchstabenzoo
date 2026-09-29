//! Automatic quality tier for weak phones (PERF-BUDGETS rule 5, Q-170, PERF-R-005).
//!
//! The governor watches the real frame intervals. When the p95 frame time of a 3 s window
//! is above 33 ms (more than 5 % of its frames slower than 33.3 ms), the game steps down
//! one tier: first the pixel ratio 1.5 ([`QualityTier::LowPixels`]), and only if a later
//! window is still too slow the cheaper passes as well ([`QualityTier::Low`]: the lantern +
//! 4 lamps as point lights, the rest light pools, no clouds in the close-view sky). It never
//! steps back up within a session (no flicker between tiers), shows no menu, and ignores the
//! first seconds after the start (loading hitches), the seconds after its own switch (the
//! resize hitch) and single intervals longer than a second (pauses, background tabs).
//! A fixed tier (debug override `?quality=high|low1|low`) disables the governor.

/// The renderer settings of a tier (PERF-BUDGETS rule 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityTier {
    /// Everything as designed (desktop default): pixel ratio cap 2, lantern + 8 lamps.
    High,
    /// Low tier, step 1: pixel ratio cap 1.5 (outlines stay 2 CSS px).
    LowPixels,
    /// Low tier, step 2: as step 1 plus the lantern + 4 lamps as point lights (the rest
    /// light pools) and no clouds in the close-view sky.
    Low,
}

impl QualityTier {
    /// Id of the tier (debug override, `App::quality`).
    pub fn id(self) -> &'static str {
        match self {
            QualityTier::High => "high",
            QualityTier::LowPixels => "low1",
            QualityTier::Low => "low",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "high" => Some(QualityTier::High),
            "low1" => Some(QualityTier::LowPixels),
            "low" => Some(QualityTier::Low),
            _ => None,
        }
    }

    /// Cap of the device pixel ratio of the drawing buffer.
    pub fn max_pixel_ratio(self) -> f64 {
        match self {
            QualityTier::High => 2.0,
            QualityTier::LowPixels | QualityTier::Low => LOW_PIXEL_RATIO,
        }
    }

    /// Lamps lit as point lights besides the player's lantern (the next ones become light
    /// pools; GAME-NIGHT "Renderer", budget 6).
    pub fn lamp_lights(self) -> usize {
        match self {
            QualityTier::High | QualityTier::LowPixels => 8,
            QualityTier::Low => 4,
        }
    }

    /// Clouds in the close-view sky.
    pub fn clouds(self) -> bool {
        self != QualityTier::Low
    }

    fn lower(self) -> Self {
        match self {
            QualityTier::High => QualityTier::LowPixels,
            QualityTier::LowPixels | QualityTier::Low => QualityTier::Low,
        }
    }
}

/// Pixel ratio cap of the low tier (−44 % pixels against the cap 2).
pub const LOW_PIXEL_RATIO: f64 = 1.5;
/// A frame slower than this is "slow" (the 30 fps minimum of budget 1).
pub const SLOW_FRAME_S: f32 = 1.0 / 30.0;
/// Length of one measuring window (s).
pub const WINDOW_S: f32 = 3.0;
/// Share of slow frames above which the window's p95 frame time is above [`SLOW_FRAME_S`].
pub const SLOW_SHARE: f32 = 0.05;
/// Ignored after the start (asset uploads, first frames; s).
pub const WARMUP_S: f32 = 5.0;
/// Ignored after a switch (new drawing buffer; s).
pub const SETTLE_S: f32 = 2.0;
/// Intervals longer than this are pauses (background tab, debugger), not frames (s).
pub const MAX_INTERVAL_S: f32 = 1.0;

/// Automatic or fixed tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityMode {
    Auto,
    Fixed(QualityTier),
}

/// Chooses the tier from the frame intervals (pure; PERF-022).
#[derive(Debug, Clone)]
pub struct QualityGovernor {
    mode: QualityMode,
    tier: QualityTier,
    /// Seconds still ignored (warm-up / settle).
    hold_s: f32,
    window_s: f32,
    frames: u32,
    slow: u32,
}

impl Default for QualityGovernor {
    fn default() -> Self {
        Self::new(QualityMode::Auto)
    }
}

impl QualityGovernor {
    pub fn new(mode: QualityMode) -> Self {
        let tier = match mode {
            QualityMode::Auto => QualityTier::High,
            QualityMode::Fixed(t) => t,
        };
        Self {
            mode,
            tier,
            hold_s: WARMUP_S,
            window_s: 0.0,
            frames: 0,
            slow: 0,
        }
    }

    pub fn tier(&self) -> QualityTier {
        self.tier
    }

    pub fn mode(&self) -> QualityMode {
        self.mode
    }

    /// Sets the mode (debug override). A fixed tier applies at once; `Auto` starts again
    /// from [`QualityTier::High`] with a fresh warm-up.
    pub fn set_mode(&mut self, mode: QualityMode) {
        *self = Self::new(mode);
    }

    /// Feeds one real frame interval (seconds; 0, e.g. a debug redraw, is no frame); returns
    /// `true` when the tier changed.
    pub fn sample(&mut self, interval_s: f32) -> bool {
        if self.mode != QualityMode::Auto
            || self.tier == QualityTier::Low
            || !(interval_s > 0.0 && interval_s <= MAX_INTERVAL_S)
        {
            return false;
        }
        if self.hold_s > 0.0 {
            self.hold_s -= interval_s;
            return false;
        }
        self.window_s += interval_s;
        self.frames += 1;
        if interval_s > SLOW_FRAME_S {
            self.slow += 1;
        }
        if self.window_s < WINDOW_S {
            return false;
        }
        let too_slow = self.slow as f32 > SLOW_SHARE * self.frames as f32;
        self.window_s = 0.0;
        self.frames = 0;
        self.slow = 0;
        if too_slow {
            self.tier = self.tier.lower();
            self.hold_s = SETTLE_S;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(g: &mut QualityGovernor, interval_s: f32, seconds: f32) -> u32 {
        let mut changes = 0;
        let mut t = 0.0;
        while t < seconds {
            changes += u32::from(g.sample(interval_s));
            t += interval_s;
        }
        changes
    }

    // PERF-022: fast frames keep the high tier (desktop default unchanged), also with a few
    // hitches (< 5 % of the frames).
    #[test]
    fn perf_022_fast_frames_stay_high() {
        let mut g = QualityGovernor::default();
        assert_eq!(g.tier(), QualityTier::High);
        assert_eq!(run(&mut g, 1.0 / 60.0, 60.0), 0);
        // one 50 ms hitch per second (≈ 1.6 % of the frames)
        for _ in 0..30 {
            for _ in 0..59 {
                g.sample(1.0 / 60.0);
            }
            g.sample(0.05);
        }
        assert_eq!(g.tier(), QualityTier::High);
        // 31 fps is not slow
        assert_eq!(run(&mut g, 1.0 / 31.0, 30.0), 0);
        assert_eq!(g.tier(), QualityTier::High);
    }

    // PERF-022: p95 above 33 ms for 3 s → pixel ratio 1.5 first; still slow → the full low
    // tier; never back up (no flicker), never below Low.
    #[test]
    fn perf_022_slow_frames_step_down_once_per_window_and_never_up() {
        let mut g = QualityGovernor::default();
        // warm-up: the first 5 s never switch
        assert_eq!(run(&mut g, 0.05, WARMUP_S - 0.1), 0);
        assert_eq!(g.tier(), QualityTier::High);
        // 3 s of 20 fps → step 1
        assert_eq!(run(&mut g, 0.05, 0.1 + WINDOW_S + 0.05), 1);
        assert_eq!(g.tier(), QualityTier::LowPixels);
        assert_eq!(g.tier().max_pixel_ratio(), 1.5);
        assert_eq!(g.tier().lamp_lights(), 8);
        // still slow after the settle time and one more window → step 2
        assert_eq!(run(&mut g, 0.05, SETTLE_S + WINDOW_S + 0.1), 1);
        assert_eq!(g.tier(), QualityTier::Low);
        assert_eq!(g.tier().lamp_lights(), 4);
        assert!(!g.tier().clouds());
        // fast again: stays low (hysteresis: no way back within a session)
        assert_eq!(run(&mut g, 1.0 / 60.0, 60.0), 0);
        assert_eq!(g.tier(), QualityTier::Low);
    }

    // PERF-022: step 1 is enough → the frames are fast again → it stays at step 1.
    #[test]
    fn perf_022_step_one_holds_when_fast_enough() {
        let mut g = QualityGovernor::default();
        run(&mut g, 0.05, WARMUP_S + WINDOW_S + 0.2);
        assert_eq!(g.tier(), QualityTier::LowPixels);
        assert_eq!(run(&mut g, 1.0 / 45.0, 60.0), 0);
        assert_eq!(g.tier(), QualityTier::LowPixels);
    }

    // PERF-022: pauses (background tab: one huge interval) are not slow frames.
    #[test]
    fn perf_022_pauses_are_ignored() {
        let mut g = QualityGovernor::default();
        run(&mut g, 1.0 / 60.0, WARMUP_S + 1.0);
        for _ in 0..20 {
            assert!(!g.sample(5.0));
            run(&mut g, 1.0 / 60.0, 0.5);
        }
        assert_eq!(g.tier(), QualityTier::High);
    }

    // PERF-022: a fixed tier (debug override) never changes; ids round-trip.
    #[test]
    fn perf_022_fixed_tier_and_ids() {
        for t in [QualityTier::High, QualityTier::LowPixels, QualityTier::Low] {
            assert_eq!(QualityTier::from_id(t.id()), Some(t));
            let mut g = QualityGovernor::new(QualityMode::Fixed(t));
            assert_eq!(run(&mut g, 0.2, 60.0), 0);
            assert_eq!(g.tier(), t);
        }
        assert_eq!(QualityTier::from_id("ultra"), None);
        let mut g = QualityGovernor::new(QualityMode::Fixed(QualityTier::Low));
        g.set_mode(QualityMode::Auto);
        assert_eq!(g.tier(), QualityTier::High);
    }
}
