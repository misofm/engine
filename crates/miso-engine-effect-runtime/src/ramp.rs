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
    ///
    /// # The stationary hoist (issue #144 item 6)
    ///
    /// A retarget to the value the ramp already holds arms a ramp whose every step is `+0.0`. It
    /// produces `current` on every sample of the window and then assigns a target that is already
    /// `current` — `samples` samples of arithmetic with no observable effect. It is not a rare
    /// case: a console re-sends a parameter it did not move on every automation refresh, and
    /// because the bank kernels take their ramping decision across *all* lanes, one lane's no-op
    /// ramp drags a whole eight-track bank onto the ramping path for the length of the window.
    ///
    /// [`LinearRamp::stationary_at`] decides the case by **bit compare**, never by tolerance, and
    /// the hoist settles the ramp instead of arming it. The three exclusions in `stationary_at`
    /// are what make the skip bit-identical rather than merely close; see its documentation.
    ///
    /// The hoist deliberately does not require the ramp to be at rest. `set_target` re-derives
    /// `step` from `current`, so a retarget *to the value in force right now* is a no-op window
    /// whether or not an earlier ramp is still in flight, and cancelling that flight is exactly
    /// what the non-hoisted arm arrives at when the window ends.
    pub fn set_target(&mut self, target: f32, samples: u32) {
        self.target = target;
        if samples == 0 || Self::stationary_at(self.current, target) {
            self.current = target;
            self.step = 0.0;
            self.remaining = 0;
            return;
        }
        self.step = (target - self.current) / samples as f32;
        self.remaining = samples;
    }

    /// `true` when ramping `current` to `target` cannot change a single rendered bit.
    ///
    /// Bit-identity, not nearness, is the whole bar, so the test is `to_bits` equality plus the
    /// three exclusions where `x + 0.0 != x` bitwise or where the ramp's own snap would restore a
    /// bit pattern the additions had destroyed:
    ///
    /// * **`-0.0`.** `-0.0 + 0.0` is `+0.0`, so a no-op ramp from `-0.0` renders `+0.0` for every
    ///   sample but the last, which the D11 snap returns to `-0.0`. Skipping it would render
    ///   `-0.0` throughout. The two arms genuinely differ, so `-0.0` is never hoisted. (`+0.0` is
    ///   hoisted: `+0.0 + 0.0` is `+0.0`.)
    /// * **Non-finite values.** `NaN - NaN` is `NaN`, so the step is `NaN` rather than zero, and a
    ///   signalling payload is quieted by the addition. An infinite target ramps by `NaN` too.
    /// * Subnormals are *not* excluded: `d + 0.0 == d` exactly, for every subnormal `d`, in the
    ///   canonical floating-point environment issue #146 installs at every render entry. That
    ///   environment is load-bearing here — under a host's FTZ/DAZ the addition would flush and
    ///   the two arms would part company, which is precisely why the guard is not optional.
    #[must_use]
    #[inline]
    pub fn stationary_at(current: f32, target: f32) -> bool {
        const NEGATIVE_ZERO: u32 = 0x8000_0000;
        let bits = current.to_bits();
        bits == target.to_bits() && bits != NEGATIVE_ZERO && current.is_finite()
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
    /// Named `next_value` rather than `next`, matching the contract's `ParameterSmoother`, because
    /// this is not an iterator: it never ends, it returns `f32` rather than `Option<f32>`, and a
    /// caller that treats it as an iterator would silently get the resting value for ever.
    ///
    /// * `remaining == 0` — the ramp is at rest: returns `current` unchanged.
    /// * `remaining == 1` — the final sample: assigns `target` exactly (the D11 snap).
    /// * otherwise — `current += step`, `remaining -= 1`.
    ///
    /// A three-sample ramp from `0.0` to `1.0` therefore produces `1/3`, `1/3 + 1/3`, `1.0` — not
    /// `1/3`, `1/2`, `1.0`, which is what re-deriving the step from the remaining distance gives.
    pub fn next_value(&mut self) -> f32 {
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
    /// The returned segment reproduces [`LinearRamp::next_value`] exactly, sample for sample:
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
    /// The state is then advanced by `min(frames, remaining)` calls of [`LinearRamp::next_value`] —
    /// iterated additions, matching the kernel's iterated additions — so splitting a block
    /// anywhere leaves both the applied gains and the resulting state bit-identical (gate P1).
    #[inline(always)]
    #[must_use]
    pub fn advance_block<L: Lane>(&mut self, frames: usize) -> RampSegment<L> {
        let start = match self.remaining {
            0 => self.current,
            1 => self.target,
            _ => self.current + self.step,
        };
        let ramp_frames = core::cmp::min(self.remaining.saturating_sub(1) as usize, frames);
        // Captured before the advance: `next_value` zeroes `step` when it snaps, and the segment must
        // carry the increment that was in force during this block.
        let step = if ramp_frames == 0 { 0.0 } else { self.step };
        let target = self.target;
        let advance = core::cmp::min(frames, self.remaining as usize);
        for _ in 0..advance {
            let _ = self.next_value();
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
