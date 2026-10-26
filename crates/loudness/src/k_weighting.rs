//! The BS.1770-5 K-weighting filter, designed at the session rate.
//!
//! BS.1770-5 Annex 1 tabulates the two K-weighting biquads at 48 kHz only. This module derives
//! both at any launch rate from their analog prototypes, normalized to `w0 = 1`, by the bilinear
//! transform prewarped at each stage's `f0` (`K = tan(pi f0 / fs)`), in `f64`:
//!
//! ```text
//! stage 1 (high shelf)  H1(s) = (Vh s^2 + (Vb / Q1) s + 1) / (s^2 + s / Q1 + 1)
//!                       Vh = 10^(G / 20), Vb = Vh^E
//!                       a0 = 1 + K/Q1 + K^2
//!                       b0 = (Vh + Vb K/Q1 + K^2) / a0   b1 = 2 (K^2 - Vh) / a0
//!                       b2 = (Vh - Vb K/Q1 + K^2) / a0   a1 = 2 (K^2 - 1) / a0
//!                       a2 = (1 - K/Q1 + K^2) / a0
//! stage 2 (high-pass)   H2(s) = g48 s^2 / (s^2 + s / Q2 + 1)
//!                       a0 = 1 + K/Q2 + K^2, b = (g48 / a0) [1, -2, 1]
//!                       a1 = 2 (K^2 - 1) / a0, a2 = (1 - K/Q2 + K^2) / a0
//!                       g48 = a0 at 48 kHz
//! ```
//!
//! The constants are [`SHELF_HZ`], [`SHELF_GAIN_DB`], [`SHELF_Q`], [`SHELF_BAND_EXPONENT`],
//! [`HIGHPASS_HZ`] and [`HIGHPASS_Q`]; they reproduce the standard's 48 kHz table to about
//! `1e-15` per coefficient.
//!
//! # Stage 2's gain
//!
//! The standard's 48 kHz stage 2 has the numerator exactly `1, -2, 1`, which is a passband gain of
//! `g48 = 1.0049948987146884` (+0.0433 dB) over the prototype's unity. That gain is kept at every
//! rate, so one signal reads the same loudness at every rate; an unnormalized `1, -2, 1` at every
//! rate would read 96 kHz 0.022 dB below 48 kHz. At 48 kHz `g48 / a0` is exactly 1.
//!
//! Every transcendental is `crates/math`'s (D6), so the design is the same bits on every target.

use crate::{LoudnessError, supported_rate};

/// Stage 1 (high shelf) centre frequency, Hz.
pub const SHELF_HZ: f64 = 1_681.974_450_955_533;
/// Stage 1 high-frequency gain `G`, dB.
pub const SHELF_GAIN_DB: f64 = 3.999_843_853_973_347;
/// Stage 1 quality factor `Q1`.
pub const SHELF_Q: f64 = 0.707_175_236_955_419_6;
/// Stage 1 band-gain exponent `E`: `Vb = Vh^E`.
pub const SHELF_BAND_EXPONENT: f64 = 0.499_666_774_154_541_6;
/// Stage 2 (high-pass) corner frequency, Hz.
pub const HIGHPASS_HZ: f64 = 38.135_470_876_024_44;
/// Stage 2 quality factor `Q2`.
pub const HIGHPASS_Q: f64 = 0.500_327_037_323_877_3;
/// The rate whose stage-2 passband gain every rate keeps, Hz.
pub const HIGHPASS_GAIN_RATE: u32 = 48_000;

/// Both K-weighting stages at one launch rate, each `[b0, b1, b2, a1, a2]` with `a0 = 1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KWeighting {
    stage1: [f64; 5],
    stage2: [f64; 5],
}

/// `K = tan(pi f0 / fs)`, the bilinear prewarp of a stage's `f0`.
fn prewarp(f0: f64, rate: u32) -> f64 {
    math::tan(core::f64::consts::PI * f0 / f64::from(rate))
}

/// `1 + K/Q + K^2`, the prototype denominator `s^2 + s/Q + 1` transformed and multiplied by `K^2`.
fn leading(k: f64, q: f64) -> f64 {
    1.0 + k / q + k * k
}

impl KWeighting {
    /// Designs both stages at `rate`.
    ///
    /// # Errors
    ///
    /// [`LoudnessError::UnsupportedRate`] unless `rate` is 44.1, 48, 88.2 or 96 kHz.
    pub fn new(rate: u32) -> Result<Self, LoudnessError> {
        supported_rate(rate)?;

        let k = prewarp(SHELF_HZ, rate);
        let vh = math::pow(10.0, SHELF_GAIN_DB / 20.0);
        let vb = math::pow(vh, SHELF_BAND_EXPONENT);
        let a0 = leading(k, SHELF_Q);
        let stage1 = [
            (vh + vb * k / SHELF_Q + k * k) / a0,
            2.0 * (k * k - vh) / a0,
            (vh - vb * k / SHELF_Q + k * k) / a0,
            2.0 * (k * k - 1.0) / a0,
            (1.0 - k / SHELF_Q + k * k) / a0,
        ];

        let gain = leading(prewarp(HIGHPASS_HZ, HIGHPASS_GAIN_RATE), HIGHPASS_Q);
        let k = prewarp(HIGHPASS_HZ, rate);
        let a0 = leading(k, HIGHPASS_Q);
        let b = gain / a0;
        let stage2 = [
            b,
            -2.0 * b,
            b,
            2.0 * (k * k - 1.0) / a0,
            (1.0 - k / HIGHPASS_Q + k * k) / a0,
        ];
        Ok(Self { stage1, stage2 })
    }

    /// Stage 1, the high shelf: `[b0, b1, b2, a1, a2]`.
    #[must_use]
    pub fn stage1(&self) -> [f64; 5] {
        self.stage1
    }

    /// Stage 2, the high-pass: `[b0, b1, b2, a1, a2]`.
    #[must_use]
    pub fn stage2(&self) -> [f64; 5] {
        self.stage2
    }
}
