//! Independent `f64` RBJ parametric-EQ equations and direct-form-I reference processing.

use core::f64::consts::PI;

/// The six frozen RBJ parametric-EQ families.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceParametricEqKind {
    /// Peaking equalizer.
    Bell,
    /// Low shelving equalizer.
    LowShelf,
    /// High shelving equalizer.
    HighShelf,
    /// Second-order low-pass filter.
    LowPass,
    /// Second-order high-pass filter.
    HighPass,
    /// Second-order notch filter.
    Notch,
}

/// Invalid reference-EQ input, coefficient calculation, or stability proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceParametricEqError {
    /// Rate, frequency, gain, Q, or shelf slope was nonfinite or outside its mathematical domain.
    InvalidInput,
    /// An intermediate, normalized coefficient, or analytic denominator was nonfinite.
    NonFinite,
    /// Normalization would divide by zero.
    ZeroNormalization,
    /// The normalized second-order denominator failed strict Jury stability.
    Unstable,
}

/// Normalized f64 RBJ coefficients and their design rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceParametricEqCoefficients {
    sample_rate_hz: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    identity: bool,
}

impl ReferenceParametricEqCoefficients {
    /// Designs one normalized f64 RBJ section from independent equations.
    pub fn design(
        kind: ReferenceParametricEqKind,
        sample_rate_hz: f64,
        frequency_hz: f64,
        gain_db: f64,
        q: f64,
        shelf_slope: f64,
    ) -> Result<Self, ReferenceParametricEqError> {
        if !sample_rate_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !frequency_hz.is_finite()
            || frequency_hz <= 0.0
            || frequency_hz >= sample_rate_hz * 0.5
            || !gain_db.is_finite()
            || !q.is_finite()
            || q <= 0.0
            || !shelf_slope.is_finite()
            || shelf_slope <= 0.0
        {
            return Err(ReferenceParametricEqError::InvalidInput);
        }
        if matches!(
            kind,
            ReferenceParametricEqKind::Bell
                | ReferenceParametricEqKind::LowShelf
                | ReferenceParametricEqKind::HighShelf
        ) && gain_db == 0.0
        {
            return Ok(Self::identity(sample_rate_hz));
        }
        let omega = 2.0 * PI * frequency_hz / sample_rate_hz;
        let cosine = omega.cos();
        let sine = omega.sin();
        let amplitude = 10.0_f64.powf(gain_db / 40.0);
        let alpha_q = sine / (2.0 * q);
        let alpha_s =
            sine * 0.5 * ((amplitude + 1.0 / amplitude) * (1.0 / shelf_slope - 1.0) + 2.0).sqrt();
        let beta = 2.0 * amplitude.sqrt() * alpha_s;
        if ![omega, cosine, sine, amplitude, alpha_q, alpha_s, beta]
            .into_iter()
            .all(f64::is_finite)
        {
            return Err(ReferenceParametricEqError::NonFinite);
        }
        let (b0, b1, b2, a0, a1, a2) = match kind {
            ReferenceParametricEqKind::LowPass => (
                (1.0 - cosine) * 0.5,
                1.0 - cosine,
                (1.0 - cosine) * 0.5,
                1.0 + alpha_q,
                -2.0 * cosine,
                1.0 - alpha_q,
            ),
            ReferenceParametricEqKind::HighPass => (
                (1.0 + cosine) * 0.5,
                -(1.0 + cosine),
                (1.0 + cosine) * 0.5,
                1.0 + alpha_q,
                -2.0 * cosine,
                1.0 - alpha_q,
            ),
            ReferenceParametricEqKind::Notch => (
                1.0,
                -2.0 * cosine,
                1.0,
                1.0 + alpha_q,
                -2.0 * cosine,
                1.0 - alpha_q,
            ),
            ReferenceParametricEqKind::Bell => (
                1.0 + alpha_q * amplitude,
                -2.0 * cosine,
                1.0 - alpha_q * amplitude,
                1.0 + alpha_q / amplitude,
                -2.0 * cosine,
                1.0 - alpha_q / amplitude,
            ),
            ReferenceParametricEqKind::LowShelf => (
                amplitude * ((amplitude + 1.0) - (amplitude - 1.0) * cosine + beta),
                2.0 * amplitude * ((amplitude - 1.0) - (amplitude + 1.0) * cosine),
                amplitude * ((amplitude + 1.0) - (amplitude - 1.0) * cosine - beta),
                (amplitude + 1.0) + (amplitude - 1.0) * cosine + beta,
                -2.0 * ((amplitude - 1.0) + (amplitude + 1.0) * cosine),
                (amplitude + 1.0) + (amplitude - 1.0) * cosine - beta,
            ),
            ReferenceParametricEqKind::HighShelf => (
                amplitude * ((amplitude + 1.0) + (amplitude - 1.0) * cosine + beta),
                -2.0 * amplitude * ((amplitude - 1.0) + (amplitude + 1.0) * cosine),
                amplitude * ((amplitude + 1.0) + (amplitude - 1.0) * cosine - beta),
                (amplitude + 1.0) - (amplitude - 1.0) * cosine + beta,
                2.0 * ((amplitude - 1.0) - (amplitude + 1.0) * cosine),
                (amplitude + 1.0) - (amplitude - 1.0) * cosine - beta,
            ),
        };
        if ![b0, b1, b2, a0, a1, a2].into_iter().all(f64::is_finite) {
            return Err(ReferenceParametricEqError::NonFinite);
        }
        if a0 == 0.0 {
            return Err(ReferenceParametricEqError::ZeroNormalization);
        }
        let normalized = Self {
            sample_rate_hz,
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            identity: false,
        };
        if ![
            normalized.b0,
            normalized.b1,
            normalized.b2,
            normalized.a1,
            normalized.a2,
        ]
        .into_iter()
        .all(f64::is_finite)
        {
            return Err(ReferenceParametricEqError::NonFinite);
        }
        if !normalized.is_strictly_jury_stable() {
            return Err(ReferenceParametricEqError::Unstable);
        }
        Ok(normalized)
    }

    /// Exact normalized dry section for gain-neutral bell/shelf and explicit disabled use.
    #[must_use]
    pub const fn identity(sample_rate_hz: f64) -> Self {
        Self {
            sample_rate_hz,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            identity: true,
        }
    }

    /// Returns normalized `(b0, b1, b2, a1, a2)` in that fixed order.
    #[must_use]
    pub const fn values(self) -> (f64, f64, f64, f64, f64) {
        (self.b0, self.b1, self.b2, self.a1, self.a2)
    }

    /// Returns whether this is the exact identity section.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.identity
    }

    /// Returns whether the normalized denominator meets all frozen strict Jury inequalities.
    #[must_use]
    pub fn is_strictly_jury_stable(self) -> bool {
        self.a2.abs() < 1.0 && 1.0 + self.a1 + self.a2 > 0.0 && 1.0 - self.a1 + self.a2 > 0.0
    }

    /// Evaluates the exact f64 transfer magnitude at a legal DC-through-Nyquist frequency.
    pub fn magnitude_at_hz(self, frequency_hz: f64) -> Result<f64, ReferenceParametricEqError> {
        if !frequency_hz.is_finite()
            || frequency_hz < 0.0
            || frequency_hz > self.sample_rate_hz * 0.5
        {
            return Err(ReferenceParametricEqError::InvalidInput);
        }
        let omega = 2.0 * PI * frequency_hz / self.sample_rate_hz;
        let cosine = omega.cos();
        let sine = omega.sin();
        let cosine2 = (2.0 * omega).cos();
        let sine2 = (2.0 * omega).sin();
        let numerator_re = self.b0 + self.b1 * cosine + self.b2 * cosine2;
        let numerator_im = -self.b1 * sine - self.b2 * sine2;
        let denominator_re = 1.0 + self.a1 * cosine + self.a2 * cosine2;
        let denominator_im = -self.a1 * sine - self.a2 * sine2;
        let denominator = denominator_re.hypot(denominator_im);
        let magnitude = numerator_re.hypot(numerator_im) / denominator;
        if !magnitude.is_finite() {
            return Err(ReferenceParametricEqError::NonFinite);
        }
        Ok(magnitude)
    }
}

/// Independent f64 direct-form-I state for one parametric-EQ section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceParametricEqSection {
    coefficients: ReferenceParametricEqCoefficients,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl ReferenceParametricEqSection {
    /// Starts an independent zero-history section with immutable normalized coefficients.
    #[must_use]
    pub const fn new(coefficients: ReferenceParametricEqCoefficients) -> Self {
        Self {
            coefficients,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes one f64 sample in the frozen direct-form-I operation order.
    pub fn process(&mut self, input: f64) -> f64 {
        if self.coefficients.identity {
            let prior_x1 = self.x1;
            self.x2 = prior_x1;
            self.x1 = input;
            self.y2 = prior_x1;
            self.y1 = input;
            return input;
        }
        let p0 = self.coefficients.b0 * input;
        let p1 = self.coefficients.b1 * self.x1;
        let s0 = p0 + p1;
        let p2 = self.coefficients.b2 * self.x2;
        let s1 = s0 + p2;
        let p3 = self.coefficients.a1 * self.y1;
        let s2 = s1 - p3;
        let p4 = self.coefficients.a2 * self.y2;
        let output = s2 - p4;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }

    /// Returns the immutable coefficient design used by this reference state.
    #[must_use]
    pub const fn coefficients(self) -> ReferenceParametricEqCoefficients {
        self.coefficients
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_families_design_stably_and_have_finite_analytic_response() {
        for kind in [
            ReferenceParametricEqKind::Bell,
            ReferenceParametricEqKind::LowShelf,
            ReferenceParametricEqKind::HighShelf,
            ReferenceParametricEqKind::LowPass,
            ReferenceParametricEqKind::HighPass,
            ReferenceParametricEqKind::Notch,
        ] {
            let design =
                ReferenceParametricEqCoefficients::design(kind, 48_000.0, 1_000.0, 6.0, 1.0, 1.0)
                    .expect("legal RBJ design");
            assert!(design.is_strictly_jury_stable());
            assert!(
                design
                    .magnitude_at_hz(1_000.0)
                    .expect("magnitude")
                    .is_finite()
            );
        }
    }

    #[test]
    fn identity_section_returns_input_and_warms_independent_history() {
        let mut section = ReferenceParametricEqSection::new(
            ReferenceParametricEqCoefficients::identity(48_000.0),
        );
        assert_eq!(section.process(-0.0).to_bits(), (-0.0_f64).to_bits());
        assert_eq!(section.process(0.25), 0.25);
    }
}
