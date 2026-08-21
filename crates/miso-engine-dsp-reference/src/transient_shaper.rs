//! Independent offline `f64` reference for the launch transient shaper.
//!
//! This reference derives the four follower coefficients from their time constants and owns no
//! production effect type. It is test-only numerical evidence, not render-path code.

/// Independent parameters for one transient-shaper lane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceTransientShaperParameters {
    /// Attack-contrast amount in the inclusive `-1..=1` domain.
    pub attack_amount: f64,
    /// Sustain-contrast amount in the inclusive `-1..=1` domain.
    pub sustain_amount: f64,
    /// Wet mix in the inclusive `0..=1` domain.
    pub mix: f64,
}

/// Reference construction input was not finite or outside the launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTransientShaperError {
    /// The rate or one control was invalid.
    InvalidInput,
}

/// Offline `f64` state for one independent fast/slow detector lane.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceTransientShaper {
    sample_rate_hz: f64,
    parameters: ReferenceTransientShaperParameters,
    fast: f64,
    slow: f64,
}

impl ReferenceTransientShaper {
    /// Constructs a lane whose coefficients are derived from the frozen time constants.
    pub fn new(
        sample_rate_hz: f64,
        parameters: ReferenceTransientShaperParameters,
    ) -> Result<Self, ReferenceTransientShaperError> {
        if !sample_rate_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !parameters.attack_amount.is_finite()
            || !(-1.0..=1.0).contains(&parameters.attack_amount)
            || !parameters.sustain_amount.is_finite()
            || !(-1.0..=1.0).contains(&parameters.sustain_amount)
            || !parameters.mix.is_finite()
            || !(0.0..=1.0).contains(&parameters.mix)
        {
            return Err(ReferenceTransientShaperError::InvalidInput);
        }
        Ok(Self {
            sample_rate_hz,
            parameters,
            fast: 0.0,
            slow: 0.0,
        })
    }

    /// Processes one already-sanitized lane sample and its linked detector magnitude.
    pub fn process_sample(&mut self, input: f64, detector: f64) -> f64 {
        let detector = detector.abs();
        let fast_coefficient = coefficient(
            if detector > self.fast { 0.5 } else { 20.0 },
            self.sample_rate_hz,
        );
        let slow_coefficient = coefficient(
            if detector > self.slow { 10.0 } else { 100.0 },
            self.sample_rate_hz,
        );
        self.fast = fast_coefficient * self.fast + (1.0 - fast_coefficient) * detector;
        self.slow = slow_coefficient * self.slow + (1.0 - slow_coefficient) * detector;
        let contrast = (20.0 * self.fast.max(1.0e-8).log10()
            - 20.0 * self.slow.max(1.0e-8).log10())
        .clamp(-24.0, 24.0);
        let shape_db = (self.parameters.attack_amount * contrast.max(0.0)
            + self.parameters.sustain_amount * (-contrast).max(0.0))
        .clamp(-18.0, 18.0);
        if self.parameters.mix == 0.0 || shape_db == 0.0 {
            return input;
        }
        let wet = input * 10.0_f64.powf(shape_db * 0.05);
        if self.parameters.mix == 1.0 {
            wet
        } else {
            input + self.parameters.mix * (wet - input)
        }
    }

    /// Returns the retained fast follower.
    #[must_use]
    pub const fn fast_envelope(self) -> f64 {
        self.fast
    }

    /// Returns the retained slow follower.
    #[must_use]
    pub const fn slow_envelope(self) -> f64 {
        self.slow
    }
}

fn coefficient(milliseconds: f64, sample_rate_hz: f64) -> f64 {
    (-1.0 / (0.001 * milliseconds * sample_rate_hz)).exp()
}
