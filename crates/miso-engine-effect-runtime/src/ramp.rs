//! Linear parameter ramps with a precomputed increment (decision D11).
//!
//! A ramp divides **once, at the moment its target changes**, and then only adds. That is the
//! whole of D11: `step = (target - current) / samples` at event time, `current += step` per
//! sample, and an exact assignment of `target` on the final sample so a ramp always arrives
//! exactly where it was sent rather than within a rounding error of it.
//!
//! # Why the snap is a separate concept
//!
//! `current + step` iterated `n` times is not `current + n * step` in `f32`. The iterated form is
//! the one that is partition-invariant — a block boundary must not be observable — so it is the
//! form used, and the accumulated error is removed by assigning the target on the last sample
//! instead of by re-deriving the value from a sample index.
//!
//! # Scalar state, lane segments
//!
//! One [`LinearRamp`] is the control-plane state of one parameter of one lane (one track). To
//! drive a whole bank, each lane's ramp produces a [`RampSegment`] for the block through
//! [`LinearRamp::advance_block`], the segments are combined per lane by the caller, and
//! `miso_engine_lane::kernels::ramp_block` applies them. `advance_block` advances the scalar state
//! by exactly the same iterated additions the kernel performs, which is what makes the two agree
//! bit for bit (`tests/ramp.rs`).

use miso_engine_lane::Lane;
use miso_engine_lane::kernels::RampSegment;

/// A linear ramp from `current` to `target` in `remaining` samples.
///
/// `step` is `(target - current) / samples`, computed once by [`LinearRamp::set_target`]. The
/// invariant that matters is `remaining == 0` implies `current == target`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearRamp {
    /// Value the ramp has reached. This is the value the *previous* sample used.
    pub current: f32,
    /// Value the ramp is heading for, assigned exactly on the final sample.
    pub target: f32,
    /// Per-sample increment, precomputed at event time. The only division in the ramp.
    pub step: f32,
    /// Samples still to be produced before the ramp is at its target.
    pub remaining: u32,
}

impl LinearRamp {
    /// A ramp that is already at `value` and is not moving.
    #[must_use]
    pub const fn fixed(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            remaining: 0,
        }
    }

    /// Points the ramp at `target`, to be reached in `samples` samples.
    ///
    /// This is the one division (D11). `samples == 0` snaps immediately, which is also what a
    /// discontinuity reset and a preparation-time initial value use.
    ///
    /// Operation order, frozen: `step = (target - current) / samples as f32`.
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

    /// Assigns the target immediately and stops the ramp.
    pub fn snap(&mut self) {
        self.current = self.target;
        self.step = 0.0;
        self.remaining = 0;
    }

    /// `true` while the ramp still has samples to produce.
    #[must_use]
    pub const fn is_ramping(&self) -> bool {
        self.remaining != 0
    }

    /// Produces the next sample's value and advances the state.
    ///
    /// * `remaining == 0` — the ramp is at rest: returns `current` unchanged.
    /// * `remaining == 1` — the final sample: assigns `target` exactly (the D11 snap).
    /// * otherwise — `current += step`, `remaining -= 1`.
    ///
    /// A three-sample ramp from `0.0` to `1.0` therefore produces `1/3`, `1/3 + 1/3`, `1.0` — not
    /// `1/3`, `1/2`, `1.0`, which is what re-deriving the step from the remaining distance gives.
    pub fn next(&mut self) -> f32 {
        match self.remaining {
            0 => self.current,
            1 => {
                self.current = self.target;
                self.step = 0.0;
                self.remaining = 0;
                self.current
            }
            _ => {
                self.current += self.step;
                self.remaining -= 1;
                self.current
            }
        }
    }

    /// Describes the next `frames` samples as a [`RampSegment`] and advances the state past them.
    ///
    /// The returned segment reproduces [`LinearRamp::next`] exactly, sample for sample:
    ///
    /// * `start` is the value of the first sample of the block — `current + step`, or `target`
    ///   when this block contains the final sample, or `current` when the ramp is at rest.
    /// * `step` is the precomputed increment, splatted.
    /// * `ramp_frames` is `min(remaining - 1, frames)`: the frames that step. The remaining frames
    ///   of the block take `target` exactly. **The `- 1` is the snap**: the last ramping sample is
    ///   an assignment, not an addition, so it belongs to the target run and not to the stepping
    ///   run.
    /// * `target` is the target, applied from `ramp_frames` onward.
    ///
    /// The state is then advanced by `min(frames, remaining)` calls of [`LinearRamp::next`] —
    /// iterated additions, matching the kernel's iterated additions — so splitting a block
    /// anywhere leaves both the applied gains and the resulting state bit-identical (gate P1).
    #[must_use]
    pub fn advance_block<L: Lane>(&mut self, frames: usize) -> RampSegment<L> {
        let start = match self.remaining {
            0 => self.current,
            1 => self.target,
            _ => self.current + self.step,
        };
        let ramp_frames = core::cmp::min(self.remaining.saturating_sub(1) as usize, frames);
        // Captured before the advance: `next` zeroes `step` when it snaps, and the segment must
        // carry the increment that was in force during this block.
        let step = if ramp_frames == 0 { 0.0 } else { self.step };
        let target = self.target;
        let advance = core::cmp::min(frames, self.remaining as usize);
        for _ in 0..advance {
            let _ = self.next();
        }
        RampSegment {
            start: L::splat(start),
            step: L::splat(step),
            target: L::splat(target),
            ramp_frames,
        }
    }
}

impl Default for LinearRamp {
    fn default() -> Self {
        Self::fixed(0.0)
    }
}
