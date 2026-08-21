//! Independent offline `f64` reference for the launch feed-forward peak compressor.
//!
//! This module deliberately owns its own curve, smoother, and rings.  It neither imports nor
//! reproduces production state types, and exists only for numerical conformance tests.

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
    /// Immutable preparation-time effective lookahead in milliseconds.
    pub lookahead_ms: f64,
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
    cursor: usize,
    detector_delay: usize,
    gain_reduction_db: f64,
    main_ring: Vec<f64>,
    detector_ring: Vec<f64>,
}

impl ReferencePeakCompressor {
    /// Constructs the independent fixed-20 ms-delay reference lane.
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
            || !parameters.lookahead_ms.is_finite()
            || !(0.0..=20.0).contains(&parameters.lookahead_ms)
        {
            return Err(ReferenceCompressorError::InvalidInput);
        }
        let latency = (sample_rate_hz / 50.0) as usize;
        let ring_length = latency.checked_add(1).ok_or(ReferenceCompressorError::InvalidInput)?;
        let lookahead = (parameters.lookahead_ms * sample_rate_hz / 1000.0 + 0.5).floor()
            as usize;
        Ok(Self {
            sample_rate_hz,
            parameters,
            cursor: 0,
            detector_delay: latency - lookahead.min(latency),
            gain_reduction_db: 0.0,
            main_ring: vec![0.0; ring_length],
            detector_ring: vec![0.0; ring_length],
        })
    }

    /// Processes one already-sanitized main and detector sample with the frozen f64 equations.
    pub fn process_sample(&mut self, main: f64, detector: f64) -> f64 {
        let ring_length = self.main_ring.len();
        let cursor = self.cursor;
        self.main_ring[cursor] = main;
        self.detector_ring[cursor] = detector.abs();
        let delayed = self.main_ring[(cursor + 1) % ring_length];
        let level = self.detector_ring[(cursor + ring_length - self.detector_delay) % ring_length];
        self.cursor = (cursor + 1) % ring_length;

        let input_db = (20.0 * level.max(1.0e-8).log10()).clamp(-160.0, 24.0);
        let threshold = self.parameters.threshold_db;
        let knee = self.parameters.knee_db;
        let reciprocal_ratio = 1.0 / self.parameters.ratio;
        let output_db = if knee == 0.0 && input_db <= threshold {
            input_db
        } else if knee == 0.0 || input_db > threshold + 0.5 * knee {
            threshold + (input_db - threshold) * reciprocal_ratio
        } else if input_db < threshold - 0.5 * knee {
            input_db
        } else {
            let v = (input_db - threshold) + 0.5 * knee;
            input_db + (reciprocal_ratio - 1.0) * (v * v) / (2.0 * knee)
        };
        let target = (output_db - input_db).clamp(-100.0, 0.0);
        let attack = (-1.0 / (0.001 * self.parameters.attack_ms * self.sample_rate_hz)).exp();
        let release = (-1.0 / (0.001 * self.parameters.release_ms * self.sample_rate_hz)).exp();
        let coefficient = if target < self.gain_reduction_db { attack } else { release };
        self.gain_reduction_db = coefficient * self.gain_reduction_db
            + (1.0 - coefficient) * target;
        let gain = 10.0_f64.powf((self.gain_reduction_db + self.parameters.makeup_db) * 0.05);
        let wet = delayed * gain;
        delayed + self.parameters.mix * (wet - delayed)
    }

    /// Returns the fixed 20 ms reported delay in samples.
    #[must_use]
    pub fn latency_samples(&self) -> usize {
        self.main_ring.len() - 1
    }
}
