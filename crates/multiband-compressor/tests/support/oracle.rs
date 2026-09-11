//! Small active causal `f64` oracle for the public LR4 product gate.
//!
//! The oracle keeps the same two-stage state and operation order as the shipped split. It is test
//! local so production arithmetic remains the `f32` lane contract, while the causal sample order
//! and recombination are independently readable here.

#[derive(Clone, Copy, Default)]
struct State {
    ic1: f64,
    ic2: f64,
}

pub struct ActiveLr4 {
    nc1: f64,
    a2: f64,
    a3: f64,
    nk2: f64,
    first: State,
    second: State,
}

impl ActiveLr4 {
    pub fn new(sample_rate: f64, crossover_hz: f64) -> Self {
        let g = (core::f64::consts::PI * crossover_hz / sample_rate).tan();
        let k = core::f64::consts::SQRT_2;
        let t = g * (g + k);
        let c1 = (t / (1.0 + t)) as f32 as f64;
        let a2 = (g * (1.0 - f64::from(c1))) as f32 as f64;
        let a3 = (g * (g * (1.0 - f64::from(c1)))) as f32 as f64;
        Self {
            nc1: -f64::from(c1),
            a2,
            a3,
            nk2: f64::from(-2.0 * (core::f64::consts::SQRT_2 as f32)),
            first: State::default(),
            second: State::default(),
        }
    }

    fn stage(input: f64, nc1: f64, a2: f64, a3: f64, state: &mut State) -> (f64, f64) {
        let v3 = input - state.ic2;
        let d1 = nc1 * state.ic1 + a2 * v3;
        let v1 = state.ic1 + d1;
        let d2 = a3 * v3 + a2 * state.ic1;
        let v2 = state.ic2 + d2;
        state.ic1 += d1 + d1;
        state.ic2 += d2 + d2;
        (v1, v2)
    }

    pub fn split(&mut self, input: f32) -> (f64, f64) {
        let (v1, low_first) = Self::stage(
            f64::from(input),
            self.nc1,
            self.a2,
            self.a3,
            &mut self.first,
        );
        let ap = self.nk2 * v1 + f64::from(input);
        let (_, low) = Self::stage(low_first, self.nc1, self.a2, self.a3, &mut self.second);
        (low, ap - low)
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let (low, high) = self.split(input);
        (low + high) as f32
    }
}

/// Independent mathematical two-band curve and log-domain smoother over [`ActiveLr4`].
pub struct ActiveBands {
    crossover: ActiveLr4,
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
        let retention = |time_ms: f64| (-1.0 / (time_ms * 0.001 * sample_rate)).exp();
        Self {
            crossover: ActiveLr4::new(sample_rate, crossover_hz),
            low_state: 0.0,
            high_state: 0.0,
            low_threshold,
            high_threshold,
            low_inv_ratio_minus_one: 1.0 / low_ratio - 1.0,
            high_inv_ratio_minus_one: 1.0 / high_ratio - 1.0,
            low_attack: retention(low_attack_ms),
            low_release: retention(low_release_ms),
            high_attack: retention(high_attack_ms),
            high_release: retention(high_release_ms),
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
        let (low, high) = self.crossover.split(input);
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
