//! Small active causal `f64` oracle for the public LR4 product gate.
//!
//! The oracle composes the independent LR4 reference with the frozen mathematical curve and
//! smoother. It is test local so production arithmetic remains the `f32` lane contract, while the
//! causal sample order and recombination are independently readable here.

// D6 gate exemption: this fixture intentionally keeps its host-f64 math independent from the
// production approximation so the oracle cannot share the implementation's transcendental path.
#![allow(clippy::disallowed_methods)]

use dsp_reference::ReferenceLr4Crossover;

fn frozen_retention(time_ms: f64, sample_rate: u32) -> f64 {
    let time_ms = time_ms as f32;
    let tau_samples = time_ms * 0.001 * sample_rate as f32;
    math::expf(-1.0 / tau_samples) as f64
}

/// Independent mathematical two-band curve and log-domain smoother over [`ReferenceLr4Crossover`].
pub struct ActiveBands {
    crossover: ReferenceLr4Crossover,
    low_state: f64,
    high_state: f64,
    low_threshold: f64,
    high_threshold: f64,
    low_inv_ratio_minus_one: f64,
    high_inv_ratio_minus_one: f64,
    low_attack: f64,
    low_release: f64,
    high_attack: f64,
    high_release: f64,
}

impl ActiveBands {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sample_rate: f64,
        crossover_hz: f64,
        low_threshold: f64,
        low_ratio: f64,
        low_attack_ms: f64,
        low_release_ms: f64,
        high_threshold: f64,
        high_ratio: f64,
        high_attack_ms: f64,
        high_release_ms: f64,
    ) -> Self {
        let sample_rate_u32 = sample_rate as u32;
        Self {
            crossover: ReferenceLr4Crossover::new(sample_rate, crossover_hz).expect("reference"),
            low_state: 0.0,
            high_state: 0.0,
            low_threshold,
            high_threshold,
            low_inv_ratio_minus_one: 1.0 / low_ratio - 1.0,
            high_inv_ratio_minus_one: 1.0 / high_ratio - 1.0,
            low_attack: frozen_retention(low_attack_ms, sample_rate_u32),
            low_release: frozen_retention(low_release_ms, sample_rate_u32),
            high_attack: frozen_retention(high_attack_ms, sample_rate_u32),
            high_release: frozen_retention(high_release_ms, sample_rate_u32),
        }
    }

    fn band(
        detector: f64,
        state: &mut f64,
        threshold: f64,
        inv_ratio_minus_one: f64,
        attack: f64,
        release: f64,
    ) -> (f64, f64) {
        let level = (detector.abs().max(1.0e-8).log10() * 20.0).clamp(-160.0, 24.0);
        let delta = level - threshold;
        let target = if delta <= -3.0 {
            0.0
        } else if delta > 3.0 {
            delta * inv_ratio_minus_one
        } else {
            let knee = delta + 3.0;
            knee * knee / 12.0 * inv_ratio_minus_one
        }
        .clamp(-100.0, 0.0);
        let coefficient = if target < *state { attack } else { release };
        *state = coefficient * (*state - target) + target;
        (10.0_f64.powf(*state / 20.0), *state)
    }

    /// Returns `(output, low_gain_reduction_db, high_gain_reduction_db)` for one sample.
    pub fn process(&mut self, input: f32) -> (f32, f64, f64) {
        let (low, high) = self.crossover.process_sample(f64::from(input));
        let (low_gain, low_state) = Self::band(
            low,
            &mut self.low_state,
            self.low_threshold,
            self.low_inv_ratio_minus_one,
            self.low_attack,
            self.low_release,
        );
        let (high_gain, high_state) = Self::band(
            high,
            &mut self.high_state,
            self.high_threshold,
            self.high_inv_ratio_minus_one,
            self.high_attack,
            self.high_release,
        );
        (
            (low * low_gain + high * high_gain) as f32,
            low_state,
            high_state,
        )
    }
}
