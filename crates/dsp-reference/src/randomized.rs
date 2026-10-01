//! Seeded, edge-biased generation for the committed randomized differentials (issue #1051).
//!
//! Every randomized differential draws from [`Draw`] and runs its seeds through [`run_seeds`], so
//! all of them share one seed discipline:
//!
//! * a pull request runs the fixed seeds `0..per_pr`;
//! * [`SCALE_VARIABLE`]`=<n>` runs `0..per_pr * n` instead (the nightly job sets 100);
//! * [`SEED_VARIABLE`]`=<seed>` runs exactly that one seed, which is how a red seed is replayed;
//! * a failing seed prints the one command that replays it.
//!
//! The generator leans on the domain edges on purpose -- the exact bounds and one ulp inside them,
//! both zeros, subnormals, `f32::from_bits(1..=3)`, the infinities, NaN payloads, `+-f32::MAX`
//! and either side of the `1e30` output bound -- because every real bug the audit re-injected
//! lived at one of them (`docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md` §4.2).
//!
//! Nothing here depends on an engine crate: the generator is as independent of the code under
//! test as the `f64` oracles beside it.

use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// Replays exactly one seed: `MISO_ENGINE_RANDOMIZED_SEED=<seed>`.
pub const SEED_VARIABLE: &str = "MISO_ENGINE_RANDOMIZED_SEED";

/// Multiplies the per-pull-request seed count: `MISO_ENGINE_RANDOMIZED_SCALE=<n>`.
pub const SCALE_VARIABLE: &str = "MISO_ENGINE_RANDOMIZED_SCALE";

fn variable(name: &str) -> Option<u64> {
    let value = std::env::var(name).ok()?;
    Some(
        value
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("{name}={value:?} is not an unsigned integer")),
    )
}

/// Whether [`SEED_VARIABLE`] is replaying one seed. A test skips its reach assertions then: one
/// seed is not expected to reach everything the whole range does.
#[must_use]
pub fn replaying() -> bool {
    variable(SEED_VARIABLE).is_some()
}

/// Whether either variable overrides the pull-request seeds. A test whose own count differs by
/// profile passes its debug count to [`run_seeds`] then, so the nightly scale means one thing.
#[must_use]
pub fn overridden() -> bool {
    replaying() || variable(SCALE_VARIABLE).is_some()
}

/// The seeds this run covers for a differential whose pull-request share is `0..per_pr`.
///
/// # Panics
///
/// If either variable is set to something that is not an unsigned integer, or the scaled range
/// overflows.
#[must_use]
pub fn seeds(per_pr: u64) -> core::ops::Range<u64> {
    if let Some(seed) = variable(SEED_VARIABLE) {
        return seed..seed.checked_add(1).expect("a replayable seed");
    }
    let scale = variable(SCALE_VARIABLE).unwrap_or(1);
    0..per_pr
        .checked_mul(scale)
        .expect("the scaled seed count fits")
}

/// Runs `case` once for every seed [`seeds`] returns and returns how many ran.
///
/// A seed that panics prints `test`, the seed and `replay` -- the command that reruns the test,
/// without the seed variable -- and then resumes the panic, so the failure is the case's own.
///
/// # Panics
///
/// When a case panics, or as [`seeds`] does.
pub fn run_seeds(test: &str, replay: &str, per_pr: u64, mut case: impl FnMut(u64)) -> u64 {
    let range = seeds(per_pr);
    let count = range.end - range.start;
    for seed in range {
        if let Err(panic) = catch_unwind(AssertUnwindSafe(|| case(seed))) {
            eprintln!(
                "{test}: seed {seed} failed. Replay it alone with:\n    \
                 {SEED_VARIABLE}={seed} {replay}"
            );
            resume_unwind(panic);
        }
    }
    count
}

/// Whether two words are the same value for a class-A comparison: the same bits, or both NaN.
///
/// Issue #1065 (owner decision 10): the payload and sign of a NaN that an operation *generates* are
/// the CPU's choice (x86 and AArch64 differ), so a differential that must pass on both compares
/// every NaN as one class. Everything else, `-0.0` against `+0.0` included, is compared by bits.
/// This is the shared rule, [`crate::class_a::same`].
#[must_use]
pub fn same_word(a: f32, b: f32) -> bool {
    crate::class_a::same(a, b)
}

/// The first index at which two equally long planes differ under [`same_word`].
#[must_use]
pub fn first_difference(a: &[f32], b: &[f32]) -> Option<usize> {
    if a.len() != b.len() {
        return Some(a.len().min(b.len()));
    }
    a.iter().zip(b).position(|(x, y)| !same_word(*x, *y))
}

/// The hostile words of the audit's list: both zeros, the smallest and largest subnormals, the
/// infinities, quiet and signalling NaN payloads of both signs, `+-f32::MAX`, and the words either
/// side of the `1e30` block bound.
pub const HOSTILE: [f32; 22] = [
    0.0,
    -0.0,
    f32::from_bits(1),
    f32::from_bits(2),
    f32::from_bits(3),
    f32::from_bits(0x8000_0001),
    f32::from_bits(0x007f_ffff),
    f32::MIN_POSITIVE,
    -f32::MIN_POSITIVE,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::from_bits(0x7fc0_0000),
    f32::from_bits(0xffc0_0000),
    f32::from_bits(0x7f80_0001),
    f32::from_bits(0x7fc0_1234),
    f32::from_bits(0xffa0_0001),
    f32::MAX,
    f32::MIN,
    1.0e30,
    -1.0e30,
    f32::from_bits(0x7149_f2c9), // the largest word below 1e30
    1.0,
];

/// What one block of input carries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Profile {
    /// Uniform noise at an ordinary level.
    Clean,
    /// Noise with [`HOSTILE`] words sprinkled in, NaNs and infinities included.
    Hostile,
    /// `+0.0` throughout.
    Silence,
    /// `+0.0` and `-0.0` only.
    SignedZeros,
    /// Subnormals of both signs only.
    Subnormal,
    /// Noise at `1e-6` down to `1e-20`, around every detector floor.
    Tiny,
    /// The exact levels `+-1`, `+-0.5`, `0.25`, `2` and friends.
    Exact,
    /// Noise near full scale, up to `+-4`.
    Loud,
}

impl Profile {
    /// Every profile.
    pub const ALL: [Self; 8] = [
        Self::Clean,
        Self::Hostile,
        Self::Silence,
        Self::SignedZeros,
        Self::Subnormal,
        Self::Tiny,
        Self::Exact,
        Self::Loud,
    ];

    /// Whether every word this profile draws is finite and at most `4.0` in magnitude: the legal
    /// input an effect must answer with finite output.
    #[must_use]
    pub const fn is_finite_legal(self) -> bool {
        !matches!(self, Self::Hostile)
    }
}

/// A SplitMix64 stream with edge-biased draws.
#[derive(Clone, Debug)]
pub struct Draw {
    state: u64,
}

impl Draw {
    /// The stream for `seed`. Every seed, zero included, gives a distinct stream.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x4d49_534f_5f31_3035,
        }
    }

    /// The next 64 raw bits.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `0..bound`.
    ///
    /// # Panics
    ///
    /// If `bound` is zero.
    pub fn below(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "an empty range has nothing to draw");
        (self.next_u64() % bound as u64) as usize
    }

    /// `true` with probability `numerator / denominator`.
    pub fn chance(&mut self, numerator: u64, denominator: u64) -> bool {
        self.next_u64() % denominator < numerator
    }

    /// Uniform in `[0, 1)`, from the top 24 bits.
    pub fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * (1.0 / 16_777_216.0)
    }

    /// Uniform in `[-amplitude, amplitude)`.
    pub fn noise(&mut self, amplitude: f32) -> f32 {
        (self.unit() * 2.0 - 1.0) * amplitude
    }

    /// One element of `values`.
    ///
    /// # Panics
    ///
    /// If `values` is empty.
    pub fn pick<T: Copy>(&mut self, values: &[T]) -> T {
        values[self.below(values.len())]
    }

    /// A subnormal of either sign, never zero; half the time one of the three smallest, where a
    /// product with any coefficient below `0.5` in magnitude rounds to a signed zero.
    pub fn subnormal(&mut self) -> f32 {
        let magnitude = if self.chance(1, 2) {
            1 + self.below(3) as u32
        } else {
            1 + (self.next_u64() as u32 & 0x007f_fffe)
        };
        let sign = if self.chance(1, 2) { 0x8000_0000 } else { 0 };
        f32::from_bits(magnitude | sign)
    }

    /// One of [`HOSTILE`].
    pub fn hostile(&mut self) -> f32 {
        self.pick(&HOSTILE)
    }

    /// A block length in `1..=quantum`, half the time one of the edges: 1, 2, 3, the SIMD chunk
    /// sizes either side of 4, 8, 16 and 32, and `quantum - 1` and `quantum`.
    ///
    /// # Panics
    ///
    /// If `quantum` is zero.
    pub fn frames(&mut self, quantum: usize) -> usize {
        assert!(quantum > 0, "a block has at least one frame");
        if self.chance(1, 2) {
            let edges = [
                1,
                2,
                3,
                4,
                5,
                7,
                8,
                9,
                15,
                16,
                17,
                31,
                32,
                33,
                quantum - 1,
                quantum,
            ];
            edges[self.below(edges.len())].clamp(1, quantum)
        } else {
            1 + self.below(quantum)
        }
    }

    /// A value in `[minimum, maximum]`, three times in five at an edge: either bound, one ulp
    /// inside either bound, the midpoint, and `0.0` or the smallest positive subnormal when the
    /// domain holds them. Never `-0.0`.
    pub fn in_domain(&mut self, minimum: f32, maximum: f32) -> f32 {
        let value = if self.chance(3, 5) {
            let tiny = f32::from_bits(1);
            match self.below(8) {
                0 => minimum,
                1 => maximum,
                2 => minimum.next_up().min(maximum),
                3 => maximum.next_down().max(minimum),
                4 => minimum + (maximum - minimum) * 0.5,
                5 if minimum <= 0.0 && maximum >= 0.0 => 0.0,
                6 if minimum <= tiny && maximum >= tiny => tiny,
                _ => minimum + (maximum - minimum) * self.unit(),
            }
        } else {
            minimum + (maximum - minimum) * self.unit()
        };
        let value = value.clamp(minimum, maximum);
        if value == 0.0 { 0.0 } else { value }
    }

    /// One input word of `profile`.
    pub fn sample(&mut self, profile: Profile) -> f32 {
        match profile {
            Profile::Clean => self.noise(0.7),
            Profile::Hostile => {
                if self.chance(1, 5) {
                    self.hostile()
                } else {
                    self.noise(0.7)
                }
            }
            Profile::Silence => 0.0,
            Profile::SignedZeros => {
                if self.chance(1, 2) {
                    0.0
                } else {
                    -0.0
                }
            }
            Profile::Subnormal => self.subnormal(),
            Profile::Tiny => {
                let amplitude = self.pick(&[1.0e-6_f32, 1.0e-8, 3.0e-9, 1.0e-20]);
                self.noise(amplitude)
            }
            Profile::Exact => self.pick(&[1.0_f32, -1.0, 0.5, -0.5, 0.25, 2.0, 0.125, -0.1]),
            Profile::Loud => self.noise(4.0),
        }
    }

    /// One of [`Profile::ALL`].
    pub fn profile(&mut self) -> Profile {
        self.pick(&Profile::ALL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_seeds_give_distinct_streams() {
        let (mut a, mut b) = (Draw::new(0), Draw::new(1));
        assert_ne!(a.next_u64(), b.next_u64());
        let (mut c, mut d) = (Draw::new(7), Draw::new(7));
        assert_eq!(c.next_u64(), d.next_u64());
    }

    #[test]
    fn domain_draws_stay_in_the_domain_and_reach_both_bounds() {
        let mut draw = Draw::new(3);
        let (mut low, mut high) = (false, false);
        for _ in 0..1_000 {
            let value = draw.in_domain(-24.0, 24.0);
            assert!((-24.0..=24.0).contains(&value) && value.to_bits() != 0x8000_0000);
            low |= value == -24.0;
            high |= value == 24.0;
        }
        assert!(low && high);
    }

    #[test]
    fn first_difference_folds_nans_and_distinguishes_signed_zeros() {
        assert_eq!(first_difference(&[1.0, 0.0], &[1.0, -0.0]), Some(1));
        assert_eq!(first_difference(&[1.0, f32::NAN], &[1.0, -f32::NAN]), None);
    }

    #[test]
    fn frames_stay_inside_the_quantum() {
        let mut draw = Draw::new(9);
        for quantum in [1, 2, 32, 128] {
            for _ in 0..200 {
                assert!((1..=quantum).contains(&draw.frames(quantum)));
            }
        }
    }
}
