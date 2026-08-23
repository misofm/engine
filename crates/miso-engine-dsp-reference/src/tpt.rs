//! Bit-identity `f32` twin of the issue-007 TPT SVF, and its transfer-model adapter.

use crate::ReferenceSvfStateSpace;

/// Selects a low-pass or high-pass observation from the TPT state-space model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTptOutput {
    /// The low-pass state-variable output.
    LowPass,
    /// The high-pass state-variable output.
    HighPass,
}

/// The retained-state boundary action taken by the twin `f32` recurrence.
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

/// One conditioned TPT recurrence step evaluated by the twin.
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

/// Bit-identity twin of the `miso_engine_core` scalar `process_tpt_scalar` graph (issue 007).
///
/// It is **not** an independent oracle: the coefficient calculation and the frozen ascending
/// multiply/add graph are the production ones, transcribed word for word. It exists so fixture
/// bits can be regenerated from a scalar twin (master plan §8.3), and it commits only finite
/// normal or canonical positive-zero state. The independent oracle for this topology is
/// [`ReferenceSvfStateSpace`](crate::ReferenceSvfStateSpace).
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

/// Transfer of the conditioned `(c1, a2, a3, k)` TPT words.
///
/// This is a thin wrapper over [`ReferenceSvfStateSpace`], the single TPT/SVF transfer model in
/// this crate (`svf.rs`): the conditioned words are exactly the master-plan §4.2 A1 storage form,
/// so the low-pass mix is `(0, 0, 1)` and the high-pass mix is `(1, -k, -1)`. It survives only to
/// keep the `(real, imaginary)` tuple response its builtins call sites use; new code should
/// construct [`ReferenceSvfStateSpace`] directly.
///
/// The wrapper is bit-identical to the hand-written model it replaced: with `a1 = 1 - c1`, the
/// mix `(0, 0, 1)` gives `C = [a2, 1 - a3]`, `D = a3` and the mix `(1, -k, -1)` gives
/// `C = [-k*a1 - a2, k*a2 - (1 - a3)]`, `D = 1 - k*a2 - a3`, each term in the same operation
/// order as before.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceTptStateSpace(ReferenceSvfStateSpace);

impl ReferenceTptStateSpace {
    /// Derives the selected transfer function from exact cast `f32` TPT coefficients.
    #[must_use]
    pub fn from_cast_coefficients(
        c1: f32,
        a2: f32,
        a3: f32,
        k: f32,
        output: ReferenceTptOutput,
    ) -> Self {
        let k = f64::from(k);
        let mix = match output {
            ReferenceTptOutput::LowPass => [0.0, 0.0, 1.0],
            ReferenceTptOutput::HighPass => [1.0, -k, -1.0],
        };
        Self(ReferenceSvfStateSpace::new(
            f64::from(c1),
            f64::from(a2),
            f64::from(a3),
            mix,
        ))
    }

    /// Returns the underlying single-model state space.
    #[must_use]
    pub const fn state_space(self) -> ReferenceSvfStateSpace {
        self.0
    }

    /// Returns the complex transfer response as `(real, imaginary)`.
    #[must_use]
    pub fn response(self, rate_hz: f64, frequency_hz: f64) -> Option<(f64, f64)> {
        let response = self.0.response(rate_hz, frequency_hz)?;
        Some((response.re, response.im))
    }

    /// Returns response magnitude in dB, floored only by IEEE zero behavior.
    #[must_use]
    pub fn magnitude_db(self, rate_hz: f64, frequency_hz: f64) -> Option<f64> {
        self.0.magnitude_db(rate_hz, frequency_hz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// E5: collapsing the hand-written TPT state space onto the one SVF model moved no bits.
    ///
    /// The expected words are the deleted model's own expressions, recomputed inline here from
    /// the same cast coefficients; `to_bits` equality (not a tolerance) is the gate.
    #[test]
    fn cast_coefficient_transfer_is_bit_identical_to_the_replaced_model() {
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            for cutoff in [10.0_f32, 20.0, 100.0, 1_000.0, 10_000.0] {
                for output in [ReferenceTptOutput::LowPass, ReferenceTptOutput::HighPass] {
                    let Some(reference) =
                        ReferenceRetainedTptF32::conditioned_butterworth(rate, cutoff, output)
                    else {
                        continue;
                    };
                    let [c1, a2, a3, k] = reference.coefficient_bits().map(f32::from_bits);
                    let state =
                        ReferenceTptStateSpace::from_cast_coefficients(c1, a2, a3, k, output);
                    let (c1, a2, a3, k) =
                        (f64::from(c1), f64::from(a2), f64::from(a3), f64::from(k));
                    let (expected_c0, expected_c1, expected_d) = match output {
                        ReferenceTptOutput::LowPass => (a2, 1.0 - a3, a3),
                        ReferenceTptOutput::HighPass => {
                            (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3)
                        }
                    };
                    let expected = ReferenceSvfStateSpace::from_words(
                        1.0 - 2.0 * c1,
                        -2.0 * a2,
                        2.0 * a2,
                        1.0 - 2.0 * a3,
                        2.0 * a2,
                        2.0 * a3,
                        expected_c0,
                        expected_c1,
                        expected_d,
                    );
                    assert_eq!(
                        state.state_space(),
                        expected,
                        "rate={rate} cutoff={cutoff} output={output:?}"
                    );
                    for frequency in [0.0, 10.0, cutoff.into(), 0.49 * f64::from(rate)] {
                        let (actual_re, actual_im) = state
                            .response(f64::from(rate), frequency)
                            .expect("response");
                        let modelled = expected
                            .response(f64::from(rate), frequency)
                            .expect("model");
                        assert_eq!(actual_re.to_bits(), modelled.re.to_bits());
                        assert_eq!(actual_im.to_bits(), modelled.im.to_bits());
                    }
                }
            }
        }
    }
}
