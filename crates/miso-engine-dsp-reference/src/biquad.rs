//! Independent `f64` RBJ biquad oracle for conformance fixtures.

/// RBJ filter family selected by the independent oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceFilterKind {
    /// Second-order low-pass response.
    LowPass,
    /// Second-order high-pass response.
    HighPass,
}

/// Invalid independent-reference design input or unstable result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceBiquadError {
    /// The rate was nonfinite or not positive.
    InvalidRate,
    /// The cutoff was nonfinite or outside the open Nyquist interval.
    InvalidCutoff,
    /// Strict second-order Jury checks failed.
    Unstable,
}

/// A separate offline transposed-DF-II oracle. It intentionally shares no production code.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceBiquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    z1: f64,
    z2: f64,
}

impl ReferenceBiquad {
    /// Designs a normalized RBJ Butterworth section from independent `f64` equations.
    pub fn rbj_butterworth(
        rate_hz: f64,
        cutoff_hz: f64,
        kind: ReferenceFilterKind,
    ) -> Result<Self, ReferenceBiquadError> {
        if !rate_hz.is_finite() || rate_hz <= 0.0 {
            return Err(ReferenceBiquadError::InvalidRate);
        }
        if !cutoff_hz.is_finite() || cutoff_hz <= 0.0 || cutoff_hz >= rate_hz * 0.5 {
            return Err(ReferenceBiquadError::InvalidCutoff);
        }
        let omega = core::f64::consts::TAU * cutoff_hz / rate_hz;
        let cosine = omega.cos();
        let alpha = omega.sin() / (2.0 * core::f64::consts::FRAC_1_SQRT_2);
        let (b0, b1, b2) = match kind {
            ReferenceFilterKind::LowPass => {
                ((1.0 - cosine) * 0.5, 1.0 - cosine, (1.0 - cosine) * 0.5)
            }
            ReferenceFilterKind::HighPass => {
                ((1.0 + cosine) * 0.5, -(1.0 + cosine), (1.0 + cosine) * 0.5)
            }
        };
        let a0 = 1.0 + alpha;
        let (b0, b1, b2, a1, a2) = (
            b0 / a0,
            b1 / a0,
            b2 / a0,
            -2.0 * cosine / a0,
            (1.0 - alpha) / a0,
        );
        if a2.abs() >= 1.0 || 1.0 + a1 + a2 <= 0.0 || 1.0 - a1 + a2 <= 0.0 {
            return Err(ReferenceBiquadError::Unstable);
        }
        Ok(Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            z1: 0.0,
            z2: 0.0,
        })
    }
    /// Processes one `f64` sample using the independent transposed DF-II state.
    pub fn process(&mut self, input: f64) -> f64 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }
    /// Returns normalized `(b0, b1, b2, a1, a2)` coefficients for fixture inspection.
    pub fn coefficients(self) -> (f64, f64, f64, f64, f64) {
        (self.b0, self.b1, self.b2, self.a1, self.a2)
    }
}

/// Independently derives the RBJ Butterworth magnitude in dB at one frequency.
///
/// This is the analytic response of [`ReferenceBiquad`] itself: it evaluates the designed
/// normalized numerator and denominator as polynomials in `z` at `frequency_hz`.
pub fn rbj_butterworth_magnitude_db(
    rate_hz: f64,
    cutoff_hz: f64,
    kind: ReferenceFilterKind,
    frequency_hz: f64,
) -> Option<f64> {
    let filter = ReferenceBiquad::rbj_butterworth(rate_hz, cutoff_hz, kind).ok()?;
    let (b0, b1, b2, a1, a2) = filter.coefficients();
    let phase = core::f64::consts::TAU * frequency_hz / rate_hz;
    let (cosine, sine) = (phase.cos(), phase.sin());
    let (cosine2, sine2) = ((2.0 * phase).cos(), (2.0 * phase).sin());
    let numerator = (b0 + b1 * cosine + b2 * cosine2).hypot(-b1 * sine - b2 * sine2);
    let denominator = (1.0 + a1 * cosine + a2 * cosine2).hypot(a1 * sine + a2 * sine2);
    if numerator == 0.0 {
        Some(f64::NEG_INFINITY)
    } else if denominator == 0.0 {
        None
    } else {
        Some(20.0 * (numerator / denominator).log10())
    }
}
