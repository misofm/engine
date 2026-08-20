//! Frozen SplitMix64 fixture generator.

/// Default fixture seed: ASCII `MISOENG2`.
pub(crate) const DEFAULT_SEED: u64 = 0x4D49_534F_454E_4732;

/// Deterministic SplitMix64 state.
#[derive(Clone, Copy, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl Default for SplitMix64 {
    fn default() -> Self {
        Self::new(DEFAULT_SEED)
    }
}
impl SplitMix64 {
    /// Starts at an explicit seed.
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    /// Returns the next frozen SplitMix64 word.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Returns the high 24 bits scaled by exactly `2^-24`.
    pub fn next_unit_f32(&mut self) -> f32 {
        ((self.next_u64() >> 40) as f32) * (1.0 / 16_777_216.0)
    }
    /// Returns a value in `[-1, 1)`.
    pub fn next_bipolar_f32(&mut self) -> f32 {
        self.next_unit_f32() * 2.0 - 1.0
    }
}
