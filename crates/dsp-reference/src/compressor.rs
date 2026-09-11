//! Independent offline `f64` reference for the causal launch compressor.
//!
//! This module owns its own curve, smoother, and state. It does not import production
//! coefficient or gain code, and exists only for numerical conformance tests.

/// Parameters for one independently processed reference lane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceCompressorParameters {
    /// Threshold in dB.
    pub threshold_db: f64,
    /// Ratio, constrained by the caller to the launch domain.
    pub ratio: f64,
    /// Soft-knee width in dB.
    pub knee_db: f64,
    /// Attack time constant in milliseconds.
    pub attack_ms: f64,
    /// Release time constant in milliseconds.
    pub release_ms: f64,
    /// Makeup gain in dB.
    pub makeup_db: f64,
    /// Wet/dry mix.
    pub mix: f64,
}

/// Reference construction input was not finite or outside the frozen launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceCompressorError {
    /// The rate or one parameter was invalid.
    InvalidInput,
}

/// State-owning, offline f64 reference lane.
#[derive(Clone, Debug)]
pub struct ReferencePeakCompressor {
    sample_rate_hz: f64,
    parameters: ReferenceCompressorParameters,
    gain_reduction_db: f64,
}

impl ReferencePeakCompressor {
    /// Constructs the causal zero-latency reference lane.
    pub fn new(
        sample_rate_hz: f64,
        parameters: ReferenceCompressorParameters,
    ) -> Result<Self, ReferenceCompressorError> {
        if !sample_rate_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !parameters.threshold_db.is_finite()
            || !(-80.0..=0.0).contains(&parameters.threshold_db)
            || !parameters.ratio.is_finite()
            || !(1.0..=20.0).contains(&parameters.ratio)
            || !parameters.knee_db.is_finite()
            || !(0.0..=24.0).contains(&parameters.knee_db)
            || !parameters.attack_ms.is_finite()
            || !(0.1..=200.0).contains(&parameters.attack_ms)
            || !parameters.release_ms.is_finite()
            || !(5.0..=5000.0).contains(&parameters.release_ms)
            || !parameters.makeup_db.is_finite()
            || !(-24.0..=24.0).contains(&parameters.makeup_db)
            || !parameters.mix.is_finite()
            || !(0.0..=1.0).contains(&parameters.mix)
        {
            return Err(ReferenceCompressorError::InvalidInput);
        }
        Ok(Self {
            sample_rate_hz,
            parameters,
            gain_reduction_db: 0.0,
        })
    }

    /// Processes one already-sanitized current main and detector sample.
    pub fn process_sample(&mut self, main: f64, detector: f64) -> f64 {
        let level = (20.0 * detector.abs().max(1.0e-8).log10()).clamp(-160.0, 24.0);
        let threshold = self.parameters.threshold_db;
        let knee = self.parameters.knee_db;
        let reciprocal_ratio = 1.0 / self.parameters.ratio;
        let target = if knee == 0.0 && level <= threshold {
            0.0
        } else if knee == 0.0 || level > threshold + 0.5 * knee {
            (reciprocal_ratio - 1.0) * (level - threshold)
        } else if level < threshold - 0.5 * knee {
            0.0
        } else {
            (reciprocal_ratio - 1.0) * (level - threshold + 0.5 * knee).powi(2) / (2.0 * knee)
        }
        .clamp(-100.0, 0.0);
        let attack = 1.0 - (-1.0 / (0.001 * self.parameters.attack_ms * self.sample_rate_hz)).exp();
        let release =
            1.0 - (-1.0 / (0.001 * self.parameters.release_ms * self.sample_rate_hz)).exp();
        let coefficient = if target < self.gain_reduction_db {
            attack
        } else {
            release
        };
        self.gain_reduction_db += coefficient * (target - self.gain_reduction_db);
        let gain = 10.0_f64.powf((self.gain_reduction_db + self.parameters.makeup_db) * 0.05);
        let wet = main * gain;
        main + self.parameters.mix * (wet - main)
    }

    /// The causal compressor has no added processing latency.
    #[must_use]
    pub const fn latency_samples(&self) -> usize {
        0
    }
}
