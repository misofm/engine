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
/// Master plan #83 D7 replaced the per-sample classification with one mechanism: each recursive
/// state word is flushed once per sample inside the kernel, and nothing else looks at it. There is
/// no per-sample recovery any more -- non-finite output is caught once per block, by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceTptRetainedAction {
    /// Both retained words were at or above [`FLUSH_EPS`] in magnitude, or non-finite.
    FiniteNormal,
    /// At least one retained word was below [`FLUSH_EPS`] and became positive zero.
    Flushed,
}

/// Magnitude below which a retained word is flushed to `+0.0`.
///
/// The same constant as `miso_engine_lane::FLUSH_EPS`, written out here because this twin is
/// deliberately independent of the lane crate.
pub const FLUSH_EPS: f32 = 1.0e-20;

/// One conditioned TPT recurrence step evaluated by the twin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceTptRetainedStep {
    /// Retained state bits before the step.
    pub pre_state_bits: [u32; 2],
    /// Retained state bits after the step's in-kernel flush.
    pub next_state_bits: [u32; 2],
    /// Output sample bits, straight out of the `(m0, m1, m2)` mix.
    pub output_bits: u32,
    /// Whether either retained word was flushed on this step.
    pub action: ReferenceTptRetainedAction,
}

/// Hand-written `f32` twin of the master-plan §4.2 `svf_block` recurrence (issues 007 and 085).
///
/// It is **not** an independent oracle for the *response*: the coefficient design and the frozen
/// operation order are the production ones, transcribed from the equations rather than from the
/// production source, and it never calls `miso-engine-lane`. What it proves is that the shared
/// block kernel, at any width, computes the recurrence the master plan writes down -- including
/// its frozen unfused operation order and its D7 flush. The independent oracle for the response is
/// [`ReferenceSvfStateSpace`](crate::ReferenceSvfStateSpace), and for the time domain it is
/// [`ReferenceBiquad`](crate::ReferenceBiquad).
///
/// Storage is the A1 form: `c1 = t / (1 + t)` with `t = g * (g + k)`, and the output selection is
/// the `(m0, m1, m2)` mix -- high-pass `(1, -k, -1)`, low-pass `(0, 0, 1)` -- so there is no
/// per-sample branch on the filter kind.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceRetainedTptF32 {
    c1: f32,
    a2: f32,
    a3: f32,
    k: f32,
    m0: f32,
    m1: f32,
    m2: f32,
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
        let (m0, m1, m2) = match output {
            ReferenceTptOutput::HighPass => (1.0, -k, -1.0),
            ReferenceTptOutput::LowPass => (0.0, 0.0, 1.0),
        };
        Some(Self {
            c1,
            a2,
            a3,
            k,
            m0,
            m1,
            m2,
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

    /// Returns the seven stored words `[c1, a2, a3, k, m0, m1, m2]`.
    #[must_use]
    pub const fn section_words(self) -> [u32; 7] {
        [
            self.c1.to_bits(),
            self.a2.to_bits(),
            self.a3.to_bits(),
            self.k.to_bits(),
            self.m0.to_bits(),
            self.m1.to_bits(),
            self.m2.to_bits(),
        ]
    }

    /// The output selection this section was designed for.
    #[must_use]
    pub const fn output(self) -> ReferenceTptOutput {
        self.output
    }

    /// Overwrites the two retained words.
    pub fn set_state_bits(&mut self, bits: [u32; 2]) {
        self.s1 = f32::from_bits(bits[0]);
        self.s2 = f32::from_bits(bits[1]);
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

    /// Applies one frozen recurrence step (master plan §4.2, operation order verbatim).
    ///
    /// ```text
    /// v3 = v0 - ic2
    /// d1 = (-c1 * ic1) + (a2 * v3)
    /// v1 = ic1 + d1
    /// d2 = (a3 * v3) + (a2 * ic1)
    /// v2 = ic2 + d2
    /// ic1 = flush(ic1 + (d1 + d1))
    /// ic2 = flush(ic2 + (d2 + d2))
    /// y  = (m2 * v2) + ((m1 * v1) + (m0 * v0))
    /// ```
    ///
    /// `-c1` is a sign-bit flip and `d + d` is exact. Every multiply-add is **unfused**: the
    /// multiply rounds, then the add rounds (issue #163 phase 2). This twin is written in the same
    /// two IEEE basic operations the production kernel uses, so it stays a bit-exact restatement
    /// of the frozen order rather than a more accurate one -- which is the whole of its job.
    ///
    /// Being a bit-identity twin is also its limit: a pin regenerated from this function proves
    /// the order is reproducible, not that it is correct. Correctness comes from the `f64` oracles
    /// ([`crate::ReferenceSvf`] and friends), which use no multiply-add primitive at all and are
    /// therefore unaffected by the contract.
    pub fn process(&mut self, input: f32) -> ReferenceTptRetainedStep {
        let pre_state_bits = self.state_bits();

        let v0 = input;
        let v3 = v0 - self.s2;
        let d1 = ((-self.c1) * self.s1) + (self.a2 * v3);
        let v1 = self.s1 + d1;
        let d2 = (self.a3 * v3) + (self.a2 * self.s1);
        let v2 = self.s2 + d2;
        let n1 = self.s1 + (d1 + d1);
        let n2 = self.s2 + (d2 + d2);
        let flushed = below_flush_epsilon(n1) | below_flush_epsilon(n2);
        self.s1 = flush(n1);
        self.s2 = flush(n2);
        let y = (self.m2 * v2) + ((self.m1 * v1) + (self.m0 * v0));

        ReferenceTptRetainedStep {
            pre_state_bits,
            next_state_bits: self.state_bits(),
            output_bits: y.to_bits(),
            action: if flushed {
                ReferenceTptRetainedAction::Flushed
            } else {
                ReferenceTptRetainedAction::FiniteNormal
            },
        }
    }
}

/// Whether `value` is inside the flush band; `-0.0` is (its magnitude is zero).
fn below_flush_epsilon(value: f32) -> bool {
    value.abs() < FLUSH_EPS
}

/// `flush(x)`: exactly `+0.0` inside the band, unchanged outside it, NaN untouched.
fn flush(value: f32) -> f32 {
    if below_flush_epsilon(value) {
        0.0
    } else {
        value
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
