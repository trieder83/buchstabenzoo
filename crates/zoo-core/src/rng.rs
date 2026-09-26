//! Tiny seedable PCG32 (XSH RR 64/32) — deterministic on every platform, no dependency.

/// Seeded pseudo random number generator used for all game randomness.
#[derive(Debug, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    const MUL: u64 = 6_364_136_223_846_793_005;

    /// Creates a generator from a seed (stream fixed).
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (0xda3e_39cb_94b9_5bdb << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();
        rng
    }

    /// Internal state `(state, increment)` (GAME-SAVE: the RNG state is saved).
    pub fn to_parts(&self) -> (u64, u64) {
        (self.state, self.inc)
    }

    /// Generator from a saved [`Pcg32::to_parts`].
    pub fn from_parts(state: u64, inc: u64) -> Self {
        Self {
            state,
            inc: inc | 1,
        }
    }

    /// Next uniformly distributed `u32`.
    pub fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(Self::MUL).wrapping_add(self.inc);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Uniform index in `0..n` (`n > 0`), without modulo bias.
    pub fn below(&mut self, n: u32) -> u32 {
        assert!(n > 0, "below(0)");
        let threshold = n.wrapping_neg() % n;
        loop {
            let r = self.next_u32();
            if r >= threshold {
                return r % n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let a: Vec<u32> = {
            let mut r = Pcg32::new(42);
            (0..8).map(|_| r.next_u32()).collect()
        };
        let b: Vec<u32> = {
            let mut r = Pcg32::new(42);
            (0..8).map(|_| r.next_u32()).collect()
        };
        assert_eq!(a, b);
        let mut c = Pcg32::new(43);
        assert_ne!(a[0], c.next_u32());
    }

    #[test]
    fn below_in_range() {
        let mut r = Pcg32::new(1);
        for _ in 0..1000 {
            assert!(r.below(3) < 3);
        }
    }
}
