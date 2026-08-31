//! Independent offline `f64` reference for the fixed-4x true-peak safety limiter.
//!
//! This module owns its own copy of the BS.1770-5 Annex-2 detector table, typed from the decimal
//! column of the issue-016 brief rather than from the production `f32` constants, and its own
//! rings, minimum filter, box smoother and release. It brute-forces every step that production
//! streams: the sliding minimum is a loop over the window, the box sum is a loop over the ring.
//! Sharing an algorithm with the code under test is exactly what an oracle must not do.
//!
//! # The law (issue #90, wave 2)
//!
//! For lane ceiling `C` dB, release `tau` ms, rate `Fs`, with `N = Fs/100`, `T = N + 6`,
//! `R = N + 1` and ramp window `Wb = clamp(L + 1, 32, R)`:
//!
//! ```text
//! P[n]   = max(|h[6]|, |v0|, |v1|, |v2|, |v3|)         // Annex-2 four-phase estimate
//! limit  = 10^((C - 1) / 20)                            // 1 dB internal estimator guard
//! r[n]   = if P[n] > limit { limit / P[n] } else { 1 }
//! m[n]   = min(r[n-N ..= n-N+Wb-1])                     // sliding minimum, window Wb
//! m_q[n] = floor(m[n] * 16384) / 16384                  // 2^-14 quantisation, exact in f32
//! s[n]   = (m_q[n] + m_q[n-1] + ... + m_q[n-Wb+1]) / Wb // box ramp
//! d[n]   = max(1 - s[n], d[n-1] + c * ((1 - s[n]) - d[n-1]))
//! g[n]   = 1 - d[n]
//! y[n]   = x[n-T] * g[n]
//! c      = 1 - exp(-1 / (0.001 * tau * Fs))
//! ```
//!
//! `g[n] <= r[n-N]` holds by construction: every box term is a minimum over a window that contains
//! `n-N`, so the average of the terms is at most `r[n-N]`, and `d >= 1 - s` forces `g <= s`. The
//! reference asserts that invariant on itself in debug builds.
//!
//! `limit` and `c` are ramped **in the linear domain** over 64 updates, which is the law and not an
//! approximation of a dB-domain ramp: production ramps the same two linear coefficients, so no
//! transcendental is evaluated per sample on either side.

/// Quantisation grid of the box-ramp terms: `2^14`.
///
/// Every `m_q` is an integer multiple of `2^-14` in `[0, 1]`, so a sum of at most `R <= 961` of
/// them is an integer multiple of `2^-14` below `2^24` and is therefore exact in `f32`. The
/// reference applies the same quantisation so the two laws are the same law.
pub const REFERENCE_BOX_GRID: f64 = 16_384.0;

/// Shortest ramp window, in samples (`W_MIN`).
///
/// A ramp shorter than the 12-tap detector span re-creates the inter-sample overshoot the detector
/// has already measured; 32 samples is 0.33 ms at 96 kHz and 0.73 ms at 44.1 kHz, below any attack
/// audibility threshold. A lookahead of 0 ms therefore means "fastest ramp", never "step".
pub const REFERENCE_MINIMUM_RAMP_WINDOW: usize = 32;

/// Discrete alignment of the 23.5-high-rate-sample FIR group delay, in base-rate samples.
pub const REFERENCE_FIR_ALIGNMENT: usize = 6;

/// Detector history words.
const HISTORY: usize = 12;

/// The Annex-2 four-phase detector table, rows `k = 0..11`, columns `p = 0..3`.
///
/// Typed from the decimal column of `.github/ISSUE_SPECS/BRIEFS/016-true-peak-limiter.md`; every
/// value is dyadic and exactly representable in both `f32` and `f64`.
const REFERENCE_ANNEX2_FIR: [[f64; 4]; HISTORY] = [
    [
        0.001_708_984_375,
        -0.029_174_804_687_5,
        -0.018_920_898_437_5,
        -0.008_300_781_25,
    ],
    [
        0.010_986_328_125,
        0.029_296_875,
        0.033_081_054_687_5,
        0.014_892_578_125,
    ],
    [
        -0.019_653_320_312_5,
        -0.051_757_812_5,
        -0.058_227_539_062_5,
        -0.026_611_328_125,
    ],
    [
        0.033_203_125,
        0.089_111_328_125,
        0.101_562_5,
        0.047_607_421_875,
    ],
    [
        -0.059_448_242_187_5,
        -0.166_503_906_25,
        -0.200_317_382_812_5,
        -0.102_294_921_875,
    ],
    [
        0.137_329_101_562_5,
        0.465_087_890_625,
        0.779_785_156_25,
        0.972_167_968_75,
    ],
    [
        0.972_167_968_75,
        0.779_785_156_25,
        0.465_087_890_625,
        0.137_329_101_562_5,
    ],
    [
        -0.102_294_921_875,
        -0.200_317_382_812_5,
        -0.166_503_906_25,
        -0.059_448_242_187_5,
    ],
    [
        0.047_607_421_875,
        0.101_562_5,
        0.089_111_328_125,
        0.033_203_125,
    ],
    [
        -0.026_611_328_125,
        -0.058_227_539_062_5,
        -0.051_757_812_5,
        -0.019_653_320_312_5,
    ],
    [
        0.014_892_578_125,
        0.033_081_054_687_5,
        0.029_296_875,
        0.010_986_328_125,
    ],
    [
        -0.008_300_781_25,
        -0.018_920_898_437_5,
        -0.029_174_804_687_5,
        0.001_708_984_375,
    ],
];

/// Parameters of one independently processed reference limiter lane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceTruePeakLimiterParameters {
    /// Ceiling in dBTP-est, `-24..=0`.
    pub ceiling_db: f64,
    /// Release time constant in milliseconds, `10..=2000`.
    pub release_ms: f64,
    /// Lookahead in milliseconds, `0..=10`; preparation-time only.
    pub lookahead_ms: f64,
}

/// Reference construction input was not finite or outside the frozen launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTruePeakLimiterError {
    /// The rate or one parameter was invalid.
    InvalidInput,
}

/// A linear ramp of one *linear-domain* coefficient over a fixed number of updates.
#[derive(Clone, Copy, Debug)]
struct ReferenceRamp {
    current: f64,
    target: f64,
    step: f64,
    remaining: u32,
}

impl ReferenceRamp {
    const fn fixed(value: f64) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            remaining: 0,
        }
    }

    fn advance(&mut self) -> f64 {
        match self.remaining {
            0 => {}
            1 => {
                self.current = self.target;
                self.step = 0.0;
                self.remaining = 0;
            }
            _ => {
                self.current += self.step;
                self.remaining -= 1;
            }
        }
        self.current
    }
}

/// State-owning, offline `f64` reference limiter lane.
#[derive(Clone, Debug)]
pub struct ReferenceTruePeakLimiter {
    sample_rate_hz: f64,
    parameters: ReferenceTruePeakLimiterParameters,
    n: usize,
    latency: usize,
    ring_length: usize,
    ramp_window: usize,
    history: [f64; HISTORY],
    main_ring: Vec<f64>,
    required_ring: Vec<f64>,
    box_ring: Vec<f64>,
    main_cursor: usize,
    ring_cursor: usize,
    box_cursor: usize,
    reduction: f64,
    limit: ReferenceRamp,
    release: ReferenceRamp,
}

/// `10^((ceiling_db - 1) / 20)`: the guarded linear limit of a ceiling.
#[must_use]
pub fn reference_true_peak_limit(ceiling_db: f64) -> f64 {
    10.0_f64.powf((ceiling_db - 1.0) / 20.0)
}

/// `1 - exp(-1 / (0.001 * release_ms * rate))`: the one-pole release rate coefficient.
#[must_use]
pub fn reference_release_coefficient(release_ms: f64, sample_rate_hz: f64) -> f64 {
    1.0 - (-1.0 / (0.001 * release_ms * sample_rate_hz)).exp()
}

impl ReferenceTruePeakLimiter {
    /// Constructs an independent reference lane at `sample_rate_hz`.
    ///
    /// # Errors
    ///
    /// [`ReferenceTruePeakLimiterError::InvalidInput`] if the rate or any parameter is outside the
    /// frozen launch domain.
    pub fn new(
        sample_rate_hz: f64,
        parameters: ReferenceTruePeakLimiterParameters,
    ) -> Result<Self, ReferenceTruePeakLimiterError> {
        if !sample_rate_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !parameters.ceiling_db.is_finite()
            || !(-24.0..=0.0).contains(&parameters.ceiling_db)
            || !parameters.release_ms.is_finite()
            || !(10.0..=2000.0).contains(&parameters.release_ms)
            || !parameters.lookahead_ms.is_finite()
            || !(0.0..=10.0).contains(&parameters.lookahead_ms)
        {
            return Err(ReferenceTruePeakLimiterError::InvalidInput);
        }
        let n = (sample_rate_hz / 100.0) as usize;
        if n == 0 {
            return Err(ReferenceTruePeakLimiterError::InvalidInput);
        }
        let latency = n + REFERENCE_FIR_ALIGNMENT;
        let ring_length = n + 1;
        let lookahead =
            ((parameters.lookahead_ms * sample_rate_hz / 1000.0 + 0.5).floor() as usize).min(n);
        let ramp_window = (lookahead + 1).clamp(REFERENCE_MINIMUM_RAMP_WINDOW, ring_length);
        Ok(Self {
            sample_rate_hz,
            parameters,
            n,
            latency,
            ring_length,
            ramp_window,
            history: [0.0; HISTORY],
            main_ring: vec![0.0; latency],
            required_ring: vec![1.0; ring_length],
            box_ring: vec![1.0; ramp_window],
            main_cursor: 0,
            ring_cursor: 0,
            box_cursor: 0,
            reduction: 0.0,
            limit: ReferenceRamp::fixed(reference_true_peak_limit(parameters.ceiling_db)),
            release: ReferenceRamp::fixed(reference_release_coefficient(
                parameters.release_ms,
                sample_rate_hz,
            )),
        })
    }

    /// The immutable reported latency, `Fs/100 + 6` samples.
    #[must_use]
    pub const fn latency_samples(&self) -> usize {
        self.latency
    }

    /// `N = Fs/100`: the delay between a detector estimate and the sample it guards.
    #[must_use]
    pub const fn required_delay(&self) -> usize {
        self.n
    }

    /// The box-ramp window `Wb` this lane's lookahead produced.
    #[must_use]
    pub const fn ramp_window(&self) -> usize {
        self.ramp_window
    }

    /// The parameters this lane was constructed with.
    #[must_use]
    pub const fn parameters(&self) -> ReferenceTruePeakLimiterParameters {
        self.parameters
    }

    /// Pushes `x` through the detector and returns the Annex-2 estimate `P[n]` for this lane.
    ///
    /// Detector only: no gain is applied and no output is produced. `Maximum` linking is expressed
    /// by taking the larger of two lanes' returns and handing that one value to both [`Self::apply`]
    /// calls, which is what the production law does.
    pub fn detect(&mut self, x: f64) -> f64 {
        for index in (1..HISTORY).rev() {
            self.history[index] = self.history[index - 1];
        }
        self.history[0] = x;
        let phases = reference_annex2_phases(&self.history);
        let mut peak = self.history[REFERENCE_FIR_ALIGNMENT].abs();
        for phase in phases {
            let magnitude = phase.abs();
            if magnitude > peak {
                peak = magnitude;
            }
        }
        peak
    }

    /// Advances the gain law by one sample and returns `y[n]`.
    ///
    /// `x` is this lane's *input* sample (it enters the main delay); `peak` is the possibly linked
    /// detector estimate from [`Self::detect`].
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the construction invariant `g[n] <= r[n-N]` is violated, which
    /// would mean the window alignment is wrong.
    pub fn apply(&mut self, x: f64, peak: f64) -> f64 {
        let limit = self.limit.advance();
        let release = self.release.advance();

        let required = if peak > limit { limit / peak } else { 1.0 };
        self.required_ring[self.ring_cursor] = required;

        // Window `r[n-N ..= n-N+Wb-1]`: slot `c+1` is `r[n-N]` because the ring is `N+1` long.
        let mut minimum = f64::INFINITY;
        for offset in 0..self.ramp_window {
            let slot = (self.ring_cursor + 1 + offset) % self.ring_length;
            let value = self.required_ring[slot];
            if value < minimum {
                minimum = value;
            }
        }
        let window_start = self.required_ring[(self.ring_cursor + 1) % self.ring_length];
        let quantised = (minimum * REFERENCE_BOX_GRID).floor() / REFERENCE_BOX_GRID;

        self.box_ring[self.box_cursor] = quantised;
        let mut sum = 0.0;
        for value in &self.box_ring {
            sum += *value;
        }
        let smoothed = sum / self.ramp_window as f64;

        let reduction_target = 1.0 - smoothed;
        let released = self.reduction + release * (reduction_target - self.reduction);
        self.reduction = if reduction_target > released {
            reduction_target
        } else {
            released
        };
        let gain = 1.0 - self.reduction;
        debug_assert!(
            gain <= window_start + 1.0e-12,
            "reference gain {gain} exceeds the required gain {window_start} it must respect"
        );

        let delayed = self.main_ring[self.main_cursor];
        self.main_ring[self.main_cursor] = x;

        self.main_cursor += 1;
        if self.main_cursor == self.latency {
            self.main_cursor = 0;
        }
        self.ring_cursor += 1;
        if self.ring_cursor == self.ring_length {
            self.ring_cursor = 0;
        }
        self.box_cursor += 1;
        if self.box_cursor == self.ramp_window {
            self.box_cursor = 0;
        }

        delayed * gain
    }

    /// The gain this lane applied on the most recent [`Self::apply`].
    #[must_use]
    pub fn gain(&self) -> f64 {
        1.0 - self.reduction
    }

    /// Clears every runtime word to the `FullToDefaults` state.
    pub fn reset(&mut self) {
        self.history = [0.0; HISTORY];
        self.main_ring.fill(0.0);
        self.required_ring.fill(1.0);
        self.box_ring.fill(1.0);
        self.main_cursor = 0;
        self.ring_cursor = 0;
        self.box_cursor = 0;
        self.reduction = 0.0;
        self.limit = ReferenceRamp::fixed(reference_true_peak_limit(self.parameters.ceiling_db));
        self.release = ReferenceRamp::fixed(reference_release_coefficient(
            self.parameters.release_ms,
            self.sample_rate_hz,
        ));
    }
}

/// The four Annex-2 phase outputs of a 12-word history, newest first.
///
/// Evaluated in increasing tap order from a `+0.0` accumulator with separately rounded multiply and
/// add, which is the frozen order the production `f32` detector uses.
#[must_use]
pub fn reference_annex2_phases(history_newest_first: &[f64; HISTORY]) -> [f64; 4] {
    let mut phases = [0.0_f64; 4];
    for (phase, output) in phases.iter_mut().enumerate() {
        let mut accumulator = 0.0_f64;
        for (tap, sample) in history_newest_first.iter().enumerate() {
            accumulator += REFERENCE_ANNEX2_FIR[tap][phase] * *sample;
        }
        *output = accumulator;
    }
    phases
}

/// The Annex-2 4x true-peak estimate of a whole signal: `max` over `n` of `P[n]`.
///
/// This is the measurement the #90 ceiling gate applies to *production output*, computed by an
/// implementation that shares nothing with it.
///
/// The record is measured, never extended. Sample magnitudes are taken at every index; the four
/// interpolated phases are taken only where the whole twelve-tap window lies inside the record.
/// Padding a record with zeros at either end measures a step discontinuity the signal does not
/// contain, and a twelve-tap interpolator across such a step reads about a decibel above the
/// signal's real peak — which would make a ceiling gate fail on an artefact of its own framing.
///
/// # Errors
///
/// [`ReferenceTruePeakLimiterError::InvalidInput`] if any sample is not finite.
pub fn reference_true_peak_estimate(samples: &[f64]) -> Result<f64, ReferenceTruePeakLimiterError> {
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(ReferenceTruePeakLimiterError::InvalidInput);
    }
    let mut history = [0.0_f64; HISTORY];
    let mut peak = 0.0_f64;
    for (index, sample) in samples.iter().enumerate() {
        for tap in (1..HISTORY).rev() {
            history[tap] = history[tap - 1];
        }
        history[0] = *sample;
        let magnitude = sample.abs();
        if magnitude > peak {
            peak = magnitude;
        }
        if index + 1 < HISTORY {
            continue;
        }
        for phase in reference_annex2_phases(&history) {
            let magnitude = phase.abs();
            if magnitude > peak {
                peak = magnitude;
            }
        }
    }
    Ok(peak)
}

impl core::fmt::Display for ReferenceTruePeakLimiterError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("invalid reference true-peak limiter input")
    }
}

impl core::error::Error for ReferenceTruePeakLimiterError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn parameters(lookahead_ms: f64) -> ReferenceTruePeakLimiterParameters {
        ReferenceTruePeakLimiterParameters {
            ceiling_db: -1.0,
            release_ms: 100.0,
            lookahead_ms,
        }
    }

    #[test]
    fn impulse_emerges_at_the_declared_latency() {
        let mut lane = ReferenceTruePeakLimiter::new(48_000.0, parameters(5.0)).expect("lane");
        assert_eq!(lane.latency_samples(), 486);
        let mut output = vec![0.0; 600];
        for (index, slot) in output.iter_mut().enumerate() {
            let x = if index == 0 { 1.0 } else { 0.0 };
            let peak = lane.detect(x);
            *slot = lane.apply(x, peak);
        }
        assert!(output[..486].iter().all(|value| *value == 0.0));
        assert!(output[486].abs() > 0.0);
        assert!(output[486].abs() <= reference_true_peak_limit(-1.0));
    }

    #[test]
    fn a_unit_impulse_reproduces_the_annex2_table_rows() {
        for tap in 0..HISTORY {
            let mut history = [0.0_f64; HISTORY];
            history[tap] = 1.0;
            let phases = reference_annex2_phases(&history);
            for (phase, value) in phases.iter().enumerate() {
                assert_eq!(*value, REFERENCE_ANNEX2_FIR[tap][phase]);
            }
        }
    }

    #[test]
    fn ramp_windows_follow_the_w_min_floor() {
        for (lookahead_ms, expected) in [(0.0, 32_usize), (5.0, 241), (10.0, 481)] {
            let lane =
                ReferenceTruePeakLimiter::new(48_000.0, parameters(lookahead_ms)).expect("lane");
            assert_eq!(lane.ramp_window(), expected, "lookahead {lookahead_ms}");
        }
    }

    #[test]
    fn the_estimate_of_a_scaled_impulse_is_the_scale() {
        let estimate = reference_true_peak_estimate(&[0.0, 0.5, 0.0, 0.0]).expect("estimate");
        assert!((estimate - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn out_of_domain_construction_is_rejected() {
        assert!(matches!(
            ReferenceTruePeakLimiter::new(48_000.0, parameters(11.0)),
            Err(ReferenceTruePeakLimiterError::InvalidInput)
        ));
    }
}
