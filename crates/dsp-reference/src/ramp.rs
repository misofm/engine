//! Hand-written twin of the master-plan D11 linear parameter ramp.
//!
//! D11: the per-sample increment is computed **once**, at the event, and the last ramping sample is
//! an exact assignment of the target rather than another addition. The law this crate reproduces is
//! deliberately not the pre-#83 one, which divided by the remaining count on every sample; the two
//! are not numerically equivalent for windows longer than two samples, and the trailing snap is
//! what used to hide that.
//!
//! Issue #1408: every intermediate update is held inside `[min(current, target),
//! max(current, target)]`, so a long ramp's accumulated rounding can never carry the word past its
//! target before the snap. The clamp is written out here with the same strict comparisons as
//! `lane::kernels::builtins::ramp_toward`, so the twin stays independent of `lane`.

/// One linearly ramped `f32` parameter, evaluated the way a kernel evaluates it.
///
/// The countdown is exact integer arithmetic. A render kernel carries it as an `f32` integer
/// clamped to `2^24`, which is invisible: a lane can only reach zero inside a block when its
/// remaining count is at most the block length, and the countdown is reloaded every block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceLinearRamp {
    /// The value applied to the next sample.
    current: f32,
    /// The value assigned exactly on the last ramping sample.
    target: f32,
    /// `(target - start) / samples`, computed once per event.
    step: f32,
    /// Samples left in the ramp.
    remaining: u32,
}

impl ReferenceLinearRamp {
    /// A settled ramp holding `value`.
    #[must_use]
    pub const fn settled(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            remaining: 0,
        }
    }

    /// Retargets the ramp over `samples` updates. One division, here and nowhere else.
    ///
    /// `samples == 0` snaps immediately.
    pub fn set_target(&mut self, target: f32, samples: u32) {
        self.target = target;
        if samples == 0 {
            self.current = target;
            self.step = 0.0;
            self.remaining = 0;
            return;
        }
        self.step = (target - self.current) / samples as f32;
        self.remaining = samples;
    }

    /// Advances one sample and returns the value that sample is rendered with.
    pub fn next_value(&mut self) -> f32 {
        if self.remaining <= 1 {
            self.remaining = 0;
            self.current = self.target;
        } else {
            self.remaining -= 1;
            self.current = ramp_toward(self.current, self.step, self.target);
        }
        self.current
    }

    /// The value the next sample will be rendered with, without advancing.
    #[must_use]
    pub const fn current(self) -> f32 {
        self.current
    }

    /// Samples left in the ramp.
    #[must_use]
    pub const fn remaining(self) -> u32 {
        self.remaining
    }
}

/// One D11 update that never passes its target: `current + step`, held strictly inside
/// `[min(current, target), max(current, target)]`.
///
/// `max(a, b)` is `if a > b { a } else { b }` and `min(a, b)` is `if a < b { a } else { b }`,
/// the `lane` crate's D8 forms: a word is replaced only when it is strictly beyond an endpoint, so
/// an in-range word (signed zeros included) keeps its bits and a NaN passes through.
#[must_use]
fn ramp_toward(current: f32, step: f32, target: f32) -> f32 {
    let next = current + step;
    let low = if current < target { current } else { target };
    let high = if current > target { current } else { target };
    let raised = if low > next { low } else { next };
    if high < raised { high } else { raised }
}

#[cfg(test)]
mod tests {
    use super::ReferenceLinearRamp;

    /// D11: one division at the event, iterated additions, an exact assignment at the end.
    #[test]
    fn the_step_is_computed_once_and_the_last_sample_is_an_assignment() {
        let mut ramp = ReferenceLinearRamp::settled(0.0);
        ramp.set_target(1.0, 3);
        let step = 1.0_f32 / 3.0;
        assert_eq!(ramp.next_value().to_bits(), step.to_bits());
        assert_eq!(ramp.next_value().to_bits(), (step + step).to_bits());
        assert_eq!(ramp.next_value().to_bits(), 1.0_f32.to_bits());
        assert_eq!(ramp.next_value().to_bits(), 1.0_f32.to_bits());
        assert_eq!(ramp.remaining(), 0);
    }

    /// Issue #1408: a one-second fade from unity to zero at 48 kHz carries the unclamped word to
    /// `-0.000624` before its snap; the twin holds it at the target instead.
    ///
    /// Red mutation: revert `next_value` to `self.current += self.step` -> a word below zero.
    #[test]
    fn a_long_ramp_never_passes_its_target() {
        let mut ramp = ReferenceLinearRamp::settled(1.0);
        ramp.set_target(0.0, 48_000);
        for frame in 0..48_001 {
            let value = ramp.next_value();
            assert!((0.0..=1.0).contains(&value), "frame {frame}: {value:e}");
        }
        assert_eq!(ramp.current().to_bits(), 0.0_f32.to_bits());
    }

    /// The clamp keeps an in-range word's sign: `-1 + 1` is `+0.0` next to a `-0.0` target.
    #[test]
    fn the_clamp_keeps_a_signed_zero_word() {
        let mut ramp = ReferenceLinearRamp::settled(-1.0);
        ramp.set_target(-0.0, 2);
        assert_eq!(ramp.next_value().to_bits(), (-0.5_f32).to_bits());
        let mut ramp = ReferenceLinearRamp::settled(-1.0);
        ramp.set_target(-0.0, 1);
        assert_eq!(ramp.next_value().to_bits(), (-0.0_f32).to_bits());
        assert_eq!(
            super::ramp_toward(-1.0, 1.0, -0.0).to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(
            super::ramp_toward(-0.0, -0.0, 0.0).to_bits(),
            (-0.0_f32).to_bits()
        );
        assert!(super::ramp_toward(0.5, f32::NAN, 1.0).is_nan());
    }

    /// A zero-length window is an immediate assignment, not a division by zero.
    #[test]
    fn a_zero_window_snaps() {
        let mut ramp = ReferenceLinearRamp::settled(-0.5);
        ramp.set_target(0.25, 0);
        assert_eq!(ramp.current().to_bits(), 0.25_f32.to_bits());
        assert_eq!(ramp.next_value().to_bits(), 0.25_f32.to_bits());
    }
}
