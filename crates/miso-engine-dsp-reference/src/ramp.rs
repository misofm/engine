//! Hand-written twin of the master-plan D11 linear parameter ramp.
//!
//! D11: the per-sample increment is computed **once**, at the event, and the last ramping sample is
//! an exact assignment of the target rather than another addition. The law this crate reproduces is
//! deliberately not the pre-#83 one, which divided by the remaining count on every sample; the two
//! are not numerically equivalent for windows longer than two samples, and the trailing snap is
//! what used to hide that.

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
            self.current += self.step;
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

    /// A zero-length window is an immediate assignment, not a division by zero.
    #[test]
    fn a_zero_window_snaps() {
        let mut ramp = ReferenceLinearRamp::settled(-0.5);
        ramp.set_target(0.25, 0);
        assert_eq!(ramp.current().to_bits(), 0.25_f32.to_bits());
        assert_eq!(ramp.next_value().to_bits(), 0.25_f32.to_bits());
    }
}
