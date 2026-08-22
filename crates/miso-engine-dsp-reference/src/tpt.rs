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

/// The retained-state boundary action taken by the independent `f32` recurrence.
///
/// Finite subnormal retained words are canonicalized to positive zero. Nonfinite retained words
/// reset the complete two-word state and are the only action reported as recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTptRetainedAction {
    /// Every retained state word was finite normal or zero.
    FiniteNormal,
    /// At least one finite subnormal retained word became positive zero.
    SubnormalCanonicalization,
    /// A nonfinite retained word reset the complete two-word state.
    InvalidRecovery,
}

/// One independently evaluated conditioned TPT recurrence step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceTptRetainedStep {
    /// Retained state bits before the step's state boundary.
    pub pre_state_bits: [u32; 2],
    /// Retained state bits after the step's state boundary.
    pub next_state_bits: [u32; 2],
    /// Selected and output-sanitized sample bits.
    pub output_bits: u32,
    /// The state-boundary classification for this step.
    pub action: ReferenceTptRetainedAction,
    /// Number of invalid-state recovery report increments for this step.
    pub recovery_delta: u64,
    /// Whether the selected raw output was nonfinite/subnormal and became positive zero.
    pub output_sanitized: bool,
}

/// Independent retained-`f32`, non-fused recurrence for the conditioned launch TPT section.
///
/// The coefficient calculation is intentionally repeated here from the stated conditioned
/// topology, rather than calling an engine production kernel. The recurrence uses the frozen
/// ascending multiply/add graph and commits only finite normal or canonical positive-zero state.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceRetainedTptF32 {
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    s1: f32,
    s2: f32,
    output: ReferenceTptOutput,
}

impl ReferenceRetainedTptF32 {
    /// Independently designs the conditioned Butterworth section at an enabled cutoff.
    pub fn conditioned_butterworth(
        sample_rate_hz: u32,
        cutoff_hz: f32,
        output: ReferenceTptOutput,
    ) -> Option<Self> {
        if sample_rate_hz == 0 || !cutoff_hz.is_finite() || cutoff_hz <= 0.0 {
            return None;
        }
        let g = (core::f64::consts::PI * f64::from(cutoff_hz) / f64::from(sample_rate_hz)).tan();
        let k64 = core::f64::consts::SQRT_2;
        let t0 = g + k64;
        let t1 = g * t0;
        let denominator = 1.0 + t1;
        let c1 = (t1 / denominator) as f32;
        let a2 = (g / denominator) as f32;
        let t2 = g * g;
        let a3 = (t2 / denominator) as f32;
        let k = k64 as f32;
        if ![c1, a2, a3, k]
            .into_iter()
            .all(|value| value.is_finite() && !value.is_subnormal())
        {
            return None;
        }
        Some(Self {
            c1,
            a2,
            a3,
            k,
            s1: 0.0,
            s2: 0.0,
            output,
        })
    }

    /// Returns the exact retained coefficient words.
    #[must_use]
    pub const fn coefficient_bits(self) -> [u32; 4] {
        [
            self.c1.to_bits(),
            self.a2.to_bits(),
            self.a3.to_bits(),
            self.k.to_bits(),
        ]
    }

    /// Returns the current two retained state words.
    #[must_use]
    pub const fn state_bits(self) -> [u32; 2] {
        [self.s1.to_bits(), self.s2.to_bits()]
    }

    /// Resets both retained words to canonical positive zero.
    pub fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    /// Applies one frozen non-fused recurrence step.
    pub fn process(&mut self, input: f32) -> ReferenceTptRetainedStep {
        let pre_state_bits = self.state_bits();
        let mut action = self.canonicalize_pre_state();
        let mut recovery_delta = u64::from(action == ReferenceTptRetainedAction::InvalidRecovery);

        let v3 = input - self.s2;
        let p1 = self.a2 * v3;
        let p2 = self.c1 * self.s1;
        let d1 = p1 - p2;
        let v1 = self.s1 + d1;
        let p3 = self.a2 * self.s1;
        let p4 = self.a3 * v3;
        let d2 = p3 + p4;
        let v2 = self.s2 + d2;
        let q1 = d1 + d1;
        let n1 = self.s1 + q1;
        let q2 = d2 + d2;
        let n2 = self.s2 + q2;
        let low = v2;
        let kh = self.k * v1;
        let th = input - kh;
        let high = th - v2;

        if !n1.is_finite() || !n2.is_finite() {
            self.reset();
            action = ReferenceTptRetainedAction::InvalidRecovery;
            recovery_delta = recovery_delta.saturating_add(1);
            return ReferenceTptRetainedStep {
                pre_state_bits,
                next_state_bits: self.state_bits(),
                output_bits: 0.0_f32.to_bits(),
                action,
                recovery_delta,
                output_sanitized: false,
            };
        }
        let post_action = canonicalize_retained_word(&mut self.s1, n1)
            | canonicalize_retained_word(&mut self.s2, n2);
        if post_action && action == ReferenceTptRetainedAction::FiniteNormal {
            action = ReferenceTptRetainedAction::SubnormalCanonicalization;
        }
        let output = match self.output {
            ReferenceTptOutput::LowPass => low,
            ReferenceTptOutput::HighPass => high,
        };
        let output_sanitized = !output.is_finite() || output.is_subnormal();
        ReferenceTptRetainedStep {
            pre_state_bits,
            next_state_bits: self.state_bits(),
            output_bits: canonical_output(output).to_bits(),
            action,
            recovery_delta,
            output_sanitized,
        }
    }

    fn canonicalize_pre_state(&mut self) -> ReferenceTptRetainedAction {
        if !self.s1.is_finite() || !self.s2.is_finite() {
            self.reset();
            return ReferenceTptRetainedAction::InvalidRecovery;
        }
        let s1 = self.s1;
        let s2 = self.s2;
        if canonicalize_retained_word(&mut self.s1, s1)
            | canonicalize_retained_word(&mut self.s2, s2)
        {
            ReferenceTptRetainedAction::SubnormalCanonicalization
        } else {
            ReferenceTptRetainedAction::FiniteNormal
        }
    }
}

fn canonicalize_retained_word(target: &mut f32, value: f32) -> bool {
    *target = if value.is_subnormal() { 0.0 } else { value };
    value.is_subnormal()
}

fn canonical_output(value: f32) -> f32 {
    if value.is_finite() && !value.is_subnormal() {
        value
    } else {
        0.0
    }
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
        c1: f32,
        a2: f32,
        a3: f32,
        k: f32,
        output: ReferenceTptOutput,
    ) -> Self {
        let (c1_coefficient, a2, a3, k) =
            (f64::from(c1), f64::from(a2), f64::from(a3), f64::from(k));
        let (c0, output_c1, d) = match output {
            ReferenceTptOutput::LowPass => (a2, 1.0 - a3, a3),
            ReferenceTptOutput::HighPass => (
                -k * (1.0 - c1_coefficient) - a2,
                k * a2 - (1.0 - a3),
                1.0 - k * a2 - a3,
            ),
        };
        Self {
            a00: 1.0 - 2.0 * c1_coefficient,
            a01: -2.0 * a2,
            a10: 2.0 * a2,
            a11: 1.0 - 2.0 * a3,
            b0: 2.0 * a2,
            b1: 2.0 * a3,
            c0,
            c1: output_c1,
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
