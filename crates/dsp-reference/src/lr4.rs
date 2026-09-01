//! Independent offline `f64` Linkwitz-Riley fourth-order crossover reference.
//!
//! Each band is two cascaded [`ReferenceSvf`] sections designed at the same crossover frequency
//! with `Q = 1/sqrt(2)`, so `H_lp4 = H_lp^2` and `H_hp4 = H_hp^2` with `H_lp`/`H_hp` the
//! Butterworth-Q low-pass and high-pass outputs of the one Simper mapping in [`crate::svf`].
//!
//! **Sign convention.** Neither band is polarity-inverted, and the recombination is the plain sum
//! `low + high`. With the analog prototypes `H_lp(s) = 1/D(s)`, `H_hp(s) = s^2/D(s)` and
//! `D(s) = s^2 + sqrt(2) s + 1`,
//!
//! ```text
//! H_lp^2 + H_hp^2 = (1 + s^4) / D(s)^2 = (s^2 - sqrt(2) s + 1) / (s^2 + sqrt(2) s + 1)
//! ```
//!
//! because `s^4 + 1 = (s^2 + sqrt(2) s + 1)(s^2 - sqrt(2) s + 1)`. The sum is therefore the
//! second-order Butterworth all-pass, of magnitude exactly one at every frequency; the bilinear
//! transform preserves that identity, so the digital sum is all-pass too. A polarity flip on one
//! band belongs to odd-order Linkwitz-Riley designs; applying one here would make the sum a
//! band-reject response instead, which is what the flatness test rejects.
//!
//! Zavalishin, *The Art of VA Filter Design* ch. 4 (Linkwitz-Riley from squared Butterworth
//! sections).

use crate::{Complex64, ReferenceSvf, ReferenceSvfCoefficients, ReferenceSvfKind};

/// Reference construction failed because the rate or frozen crossover domain was invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceMultibandError {
    /// Input was non-finite or outside the fixed two-band launch domain.
    InvalidInput,
}

/// Independently state-owning two-section low/high Linkwitz-Riley crossover.
#[derive(Clone, Debug)]
pub struct ReferenceLr4Crossover {
    sample_rate_hz: f64,
    low: [ReferenceSvf; 2],
    high: [ReferenceSvf; 2],
}

impl ReferenceLr4Crossover {
    /// Designs the fixed Butterworth-Q Simper sections in `f64`.
    ///
    /// # Errors
    ///
    /// Returns [`ReferenceMultibandError::InvalidInput`] outside the frozen two-band launch
    /// domain (80 Hz to 8 kHz, below Nyquist) or if the design is not strictly stable.
    pub fn new(sample_rate_hz: f64, crossover_hz: f64) -> Result<Self, ReferenceMultibandError> {
        if !sample_rate_hz.is_finite()
            || !crossover_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !(80.0..=8_000.0).contains(&crossover_hz)
            || crossover_hz >= 0.5 * sample_rate_hz
        {
            return Err(ReferenceMultibandError::InvalidInput);
        }
        let design = |kind| {
            ReferenceSvfCoefficients::design(
                kind,
                sample_rate_hz,
                crossover_hz,
                core::f64::consts::FRAC_1_SQRT_2,
                0.0,
            )
            .map_err(|_| ReferenceMultibandError::InvalidInput)
        };
        let low = design(ReferenceSvfKind::LowPass)?;
        let high = design(ReferenceSvfKind::HighPass)?;
        Ok(Self {
            sample_rate_hz,
            low: [ReferenceSvf::new(low), ReferenceSvf::new(low)],
            high: [ReferenceSvf::new(high), ReferenceSvf::new(high)],
        })
    }

    /// Returns the independent low/high LR4 sample pair.
    pub fn process_sample(&mut self, input: f64) -> (f64, f64) {
        let low_first = self.low[0].process(input);
        let low = self.low[1].process(low_first);
        let high_first = self.high[0].process(input);
        let high = self.high[1].process(high_first);
        (low, high)
    }

    /// Resets both cascades to zero state.
    pub fn reset(&mut self) {
        for section in self.low.iter_mut().chain(self.high.iter_mut()) {
            section.reset();
        }
    }

    /// Returns `(H_lp^2, H_hp^2)` at a finite frequency inside Nyquist.
    #[must_use]
    pub fn response(&self, frequency_hz: f64) -> Option<(Complex64, Complex64)> {
        let square = |value: Complex64| Complex64 {
            re: value.re * value.re - value.im * value.im,
            im: 2.0 * value.re * value.im,
        };
        let low = self.low[0]
            .coefficients()
            .state_space()
            .response(self.sample_rate_hz, frequency_hz)?;
        let high = self.high[0]
            .coefficients()
            .state_space()
            .response(self.sample_rate_hz, frequency_hz)?;
        Some((square(low), square(high)))
    }

    /// Returns the low and high fourth-order magnitudes in dB.
    #[must_use]
    pub fn magnitude_db(&self, frequency_hz: f64) -> Option<(f64, f64)> {
        let (low, high) = self.response(frequency_hz)?;
        let decibels = |value: Complex64| {
            let magnitude = value.re.hypot(value.im);
            if magnitude == 0.0 {
                f64::NEG_INFINITY
            } else {
                20.0 * magnitude.log10()
            }
        };
        Some((decibels(low), decibels(high)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReferenceSvfStateSpace;

    const RATES: [f64; 4] = [44_100.0, 48_000.0, 88_200.0, 96_000.0];
    const CROSSOVERS: [f64; 4] = [80.0, 500.0, 2_000.0, 8_000.0];
    const PROBES: usize = 256;
    const HALF_POWER_DB: f64 = 6.020_599_913_279_624;
    const IMPULSE_FRAMES: usize = 4_096;

    fn probes(rate: f64) -> Vec<f64> {
        let (low, high) = (20.0_f64, 20_000.0_f64.min(0.5 * rate));
        (0..PROBES)
            .map(|index| {
                let ratio = index as f64 / (PROBES - 1) as f64;
                low * (high / low).powf(ratio)
            })
            .collect()
    }

    /// E3: `LP4 + HP4` is the second-order Butterworth all-pass, and each band is at -6.02 dB at
    /// the crossover.
    ///
    /// The oracle is the analytic Linkwitz-Riley identity, a property of the topology rather than
    /// a transcription of the same equations, so agreement is evidence and not a tautology.
    #[test]
    fn lr4_sum_is_allpass_and_crossing_is_half() {
        let mut worst_flat = 0.0_f64;
        let mut worst_crossing = 0.0_f64;
        for rate in RATES {
            for crossover in CROSSOVERS {
                let filter = ReferenceLr4Crossover::new(rate, crossover).expect("legal crossover");
                for probe in probes(rate) {
                    let (low, high) = filter.response(probe).expect("legal probe");
                    let sum = Complex64 {
                        re: low.re + high.re,
                        im: low.im + high.im,
                    };
                    let flatness_db = 20.0 * sum.re.hypot(sum.im).log10();
                    assert!(
                        flatness_db.abs() <= 1e-9,
                        "rate={rate} crossover={crossover} probe={probe} sum={flatness_db:e} dB"
                    );
                    worst_flat = worst_flat.max(flatness_db.abs());
                }
                let (low_db, high_db) = filter.magnitude_db(crossover).expect("legal crossover");
                for band in [low_db, high_db] {
                    let error = (band + HALF_POWER_DB).abs();
                    assert!(
                        error <= 1e-9,
                        "rate={rate} crossover={crossover} band={band} error={error:e}"
                    );
                    worst_crossing = worst_crossing.max(error);
                }
            }
        }
        eprintln!("E3 worst_flatness_db={worst_flat:e} worst_crossing_db={worst_crossing:e}");
    }

    /// E4: the two cascaded recurrences realise the cascaded state space.
    #[test]
    fn lr4_impulse_matches_cascaded_state_space() {
        let mut worst = 0.0_f64;
        for rate in RATES {
            for crossover in CROSSOVERS {
                let mut filter = ReferenceLr4Crossover::new(rate, crossover).expect("legal");
                let cascade = |space: ReferenceSvfStateSpace| {
                    space.filter(&space.impulse_response(IMPULSE_FRAMES))
                };
                let low = cascade(filter.low[0].coefficients().state_space());
                let high = cascade(filter.high[0].coefficients().state_space());
                for frame in 0..IMPULSE_FRAMES {
                    let input = if frame == 0 { 1.0 } else { 0.0 };
                    let (actual_low, actual_high) = filter.process_sample(input);
                    worst = worst
                        .max((actual_low - low[frame]).abs())
                        .max((actual_high - high[frame]).abs());
                }
            }
        }
        eprintln!("E4 worst_abs={worst:e}");
        assert!(worst <= 1e-12, "cascade vs state space worst {worst:e}");
    }

    #[test]
    fn crossover_domain_is_enforced() {
        for crossover in [79.0, 8_001.0, f64::NAN] {
            assert_eq!(
                ReferenceLr4Crossover::new(48_000.0, crossover).err(),
                Some(ReferenceMultibandError::InvalidInput)
            );
        }
        assert_eq!(
            ReferenceLr4Crossover::new(f64::NAN, 1_000.0).err(),
            Some(ReferenceMultibandError::InvalidInput)
        );
    }
}
