//! Independent state-space transfer oracle for the issue-007 TPT SVF.

use crate::{ReferenceBiquad, ReferenceFilterKind};

/// Selects a low-pass or high-pass observation from the TPT state-space model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTptOutput {
    /// The low-pass state-variable output.
    LowPass,
    /// The high-pass state-variable output.
    HighPass,
}

/// A separately derived `f64` state-space transfer model built from cast TPT coefficient bits.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceTptStateSpace {
    a00: f64,
    a01: f64,
    a10: f64,
    a11: f64,
    b0: f64,
    b1: f64,
    c0: f64,
    c1: f64,
    d: f64,
}

impl ReferenceTptStateSpace {
    /// Derives the selected transfer function from exact cast `f32` TPT coefficients.
    pub fn from_cast_coefficients(
        a1: f32,
        a2: f32,
        a3: f32,
        k: f32,
        output: ReferenceTptOutput,
    ) -> Self {
        let (a1, a2, a3, k) = (f64::from(a1), f64::from(a2), f64::from(a3), f64::from(k));
        let (c0, c1, d) = match output {
            ReferenceTptOutput::LowPass => (a2, 1.0 - a3, a3),
            ReferenceTptOutput::HighPass => (-k * a1 - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3),
        };
        Self {
            a00: 2.0 * a1 - 1.0,
            a01: -2.0 * a2,
            a10: 2.0 * a2,
            a11: 1.0 - 2.0 * a3,
            b0: 2.0 * a2,
            b1: 2.0 * a3,
            c0,
            c1,
            d,
        }
    }
    /// Returns the complex transfer response at a finite frequency inside Nyquist.
    pub fn response(self, rate_hz: f64, frequency_hz: f64) -> Option<(f64, f64)> {
        if !rate_hz.is_finite()
            || !frequency_hz.is_finite()
            || rate_hz <= 0.0
            || !(0.0..=rate_hz * 0.5).contains(&frequency_hz)
        {
            return None;
        }
        let phase = core::f64::consts::TAU * frequency_hz / rate_hz;
        let (zr, zi) = (phase.cos(), phase.sin());
        let m00r = zr - self.a00;
        let m00i = zi;
        let m01r = -self.a01;
        let m10r = -self.a10;
        let m11r = zr - self.a11;
        let m11i = zi;
        let detr = m00r * m11r - m00i * m11i - m01r * m10r;
        let deti = m00r * m11i + m00i * m11r;
        let denominator = detr * detr + deti * deti;
        if denominator == 0.0 || !denominator.is_finite() {
            return None;
        }
        let inv00r = (m11r * detr + m11i * deti) / denominator;
        let inv00i = (m11i * detr - m11r * deti) / denominator;
        let inv01r = (-m01r * detr) / denominator;
        let inv01i = (m01r * deti) / denominator;
        let inv10r = (-m10r * detr) / denominator;
        let inv10i = (m10r * deti) / denominator;
        let inv11r = (m00r * detr + m00i * deti) / denominator;
        let inv11i = (m00i * detr - m00r * deti) / denominator;
        let state0r = inv00r * self.b0 + inv01r * self.b1;
        let state0i = inv00i * self.b0 + inv01i * self.b1;
        let state1r = inv10r * self.b0 + inv11r * self.b1;
        let state1i = inv10i * self.b0 + inv11i * self.b1;
        Some((
            self.d + self.c0 * state0r + self.c1 * state1r,
            self.c0 * state0i + self.c1 * state1i,
        ))
    }
    /// Returns response magnitude in dB, floored only by IEEE zero behavior.
    pub fn magnitude_db(self, rate_hz: f64, frequency_hz: f64) -> Option<f64> {
        let (real, imaginary) = self.response(rate_hz, frequency_hz)?;
        let magnitude = real.hypot(imaginary);
        Some(if magnitude == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * magnitude.log10()
        })
    }
}

/// Independently derives the RBJ Butterworth magnitude in dB at one frequency.
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
