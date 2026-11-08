//! Linear parameter ramps with a precomputed increment (decision D11).
//!
//! A ramp divides **once, at the moment its target changes**, and then only adds. That is the
//! whole of D11: `step = (target - current) / samples` at event time,
//! `current = ramp_toward(current, step, target)` per sample, and an exact assignment of `target`
//! on the final sample so a ramp always arrives exactly where it was sent rather than within a
//! rounding error of it.
//!
//! # The endpoint clamp (issue #1409)
//!
//! `lane::kernels::ramp_toward` is `current + step` held inside `[min(current, target),
//! max(current, target)]`. Without it the accumulated rounding of a 64-sample ramp can carry the
//! word up to about 30 ulps past its target before the snap (a ratio ramp from `1.0000039` to its
//! domain floor `1.0` reaches `0.99999994`). With it every word lies between the word at the event
//! (or at the restore) and the target, for any finite step: the interval each update allows lies
//! inside the previous one. An in-range word keeps the unclamped law's bits, signed zeros
//! included, and a NaN step propagates (#1408 D2-D3).
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
//! `lane::kernels::ramp_block` applies them. `advance_block` advances the scalar state
//! by exactly the same iterated additions the kernel performs, which is what makes the two agree
//! bit for bit (`tests/ramp.rs`).
//!
//! # Lane vectors (issue #2376)
//!
//! A bank whose per-frame loop reads a ramp's value every frame (a detector link weight, a
//! coefficient that is redesigned each frame) cannot describe the block with one segment. It
//! gathers the lanes' ramps into a [`RampLanes`], advances that by the same law lane-wide inside
//! the loop, and scatters it back once through [`LinearRamp::record_advance`]. This is the one
//! lane-vector walk of the law; the compressor, the transient shaper and the true-peak limiter
//! use it (`tests/ramp_lanes.rs`), and so do the de-esser's and the dynamic EQ's live words,
//! which stay lane words across blocks ([`crate::dynamic_band::advance_word_ramps`], issue
//! #2390).

use lane::Lane;
use lane::kernels::{RampSegment, ramp_toward};

/// A linear ramp from `current` to `target` in `remaining` samples.
///
/// `step` is `(target - current) / samples`, computed once by [`LinearRamp::set_target`].
///
/// # The phase invariant (issue #2368, tracker #2258 GR-7)
///
/// `remaining` is the phase: `0` is *settled*, anything else is *ramping*. A settled ramp is at
/// its target with a `+0.0` step, bit for bit: `current.to_bits() == target.to_bits()` and
/// `step.to_bits() == 0`. A ramping ramp may hold any words. The fields are private and every
/// constructor and method keeps the invariant ([`LinearRamp::fixed`], [`LinearRamp::holding`],
/// [`LinearRamp::set_target`], [`LinearRamp::snap`], [`LinearRamp::next_value`],
/// [`LinearRamp::advance_block`]), so a settled ramp off its target cannot be built; words from
/// outside the engine enter only through [`LinearRamp::from_words`], which refuses them.
///
/// The phase is not a `Settled | Ramping` enum: the countdown already is the discriminant, and an
/// enum would duplicate it and change the hot [`LinearRamp::next_value`] match and the layout the
/// effects gather lane vectors from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearRamp {
    /// Value the ramp has reached. This is the value the *previous* sample used.
    current: f32,
    /// Value the ramp is heading for, assigned exactly on the final sample.
    target: f32,
    /// Per-sample increment, precomputed at event time. The only division in the ramp.
    step: f32,
    /// Samples still to be produced before the ramp is at its target.
    remaining: u32,
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

    /// A ramp at `value` that stays in flight, unmoving, for `samples` samples: step `+0.0`, so
    /// it produces `value` on every sample and then settles there. `samples == 0` is
    /// [`LinearRamp::fixed`].
    ///
    /// The compressor uses it to keep a parameter in flight while the coefficient it designs
    /// returns to that parameter's design.
    #[must_use]
    pub const fn holding(value: f32, samples: u32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            remaining: samples,
        }
    }

    /// The ramp held in four words, or `None` when they break the phase invariant: a settled
    /// ramp (`remaining == 0`) whose `current` is not its `target` bit for bit, or whose step is
    /// not `+0.0`. A ramping ramp's words are accepted as they are.
    ///
    /// This is the only way to build a ramp from words, so a state restore cannot admit a
    /// settled ramp that would render a stale `current` for as long as it stays settled. It
    /// checks the phase only; a restore still checks domains, bounds and the path
    /// ([`crate::state_payload::ramp_path_inside`]). Allocation-free, so a restore may run it on
    /// the render thread.
    #[must_use]
    pub const fn from_words(current: f32, target: f32, step: f32, remaining: u32) -> Option<Self> {
        if remaining == 0 && (current.to_bits() != target.to_bits() || step.to_bits() != 0) {
            return None;
        }
        Some(Self {
            current,
            target,
            step,
            remaining,
        })
    }

    /// The value the ramp has reached: the value the previous sample used.
    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> f32 {
        self.current
    }

    /// The value the ramp is heading for, or holds when settled.
    #[must_use]
    #[inline(always)]
    pub const fn target(&self) -> f32 {
        self.target
    }

    /// The per-sample increment: `+0.0` when settled.
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> f32 {
        self.step
    }

    /// Samples still to be produced before the ramp is at its target: `0` when settled.
    #[must_use]
    #[inline(always)]
    pub const fn remaining(&self) -> u32 {
        self.remaining
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
    /// case: live controls re-send a parameter they did not move on every automation refresh, and
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

    /// Records an advance made outside the ramp: a lane-vector walk of the same law as
    /// [`LinearRamp::next_value`] (the effects' banked gather, advance and scatter), which leaves
    /// the ramp at `current` with `step` and `remaining` samples to go. `target` does not change.
    ///
    /// A walk of the law settles exactly on the target with a `+0.0` step, so for it this is a
    /// plain store. A settled `remaining == 0` stores the target and `+0.0` whatever `current` and
    /// `step` say, so no caller can leave a settled ramp off its target (issue #2368). Two scalar
    /// selects per lane per block (the compiler may branch), never per sample.
    ///
    /// A debug build asserts that a settled advance already landed on the target, so a lane law
    /// that walks off its target stays visible there instead of being masked by the store.
    #[inline(always)]
    pub fn record_advance(&mut self, current: f32, step: f32, remaining: u32) {
        debug_assert!(
            remaining != 0 || current.to_bits() == self.target.to_bits(),
            "a settled advance lands on its target"
        );
        let settled = remaining == 0;
        self.current = if settled { self.target } else { current };
        self.step = if settled { 0.0 } else { step };
        self.remaining = remaining;
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
    /// * otherwise — `current = ramp_toward(current, step, target)`, `remaining -= 1`: the word
    ///   never passes its target (issue #1409).
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
                self.current = ramp_toward(self.current, self.step, self.target);
                self.remaining -= 1;
                self.current
            }
        }
    }

    /// Describes the next `frames` samples as a [`RampSegment`] and advances the state past them.
    ///
    /// The returned segment reproduces [`LinearRamp::next_value`] exactly, sample for sample:
    ///
    /// * `start` is the value of the first sample of the block — `ramp_toward(current, step,
    ///   target)`, or `target` when this block contains the final sample, or `current` when the
    ///   ramp is at rest.
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
            _ => ramp_toward(self.current, self.step, self.target),
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

/// The widest lane vector a [`RampLanes`] gathers: AVX2's eight `f32` lanes.
const RAMP_LANES: usize = 8;

/// The longest countdown a lane vector carries exactly: `2^24`, the largest power of two whose
/// every smaller integer is an `f32`. It covers the longest link glide
/// (`effect_contract::LINK_GLIDE_MAX_SAMPLES`) and every parameter window.
const EXACT_COUNTDOWN: u32 = 1 << 24;

/// One [`LinearRamp`] per lane, as four lane vectors (issue #2376): a block loop's working copy
/// of a parameter (or a coefficient, or a link weight) across a bank.
///
/// [`RampLanes::gather`] reads the lanes' ramps at the start of a run of frames,
/// [`RampLanes::advance`] (or [`RampLanes::advance_where`]) walks [`LinearRamp::next_value`]'s
/// law once per frame lane-wide, and [`RampLanes::scatter`] writes the run's end back through
/// [`LinearRamp::record_advance`], so the scalar ramps keep their phase invariant and a run
/// boundary is not observable. The countdown is carried as an `f32` integer of at most `2^24`,
/// so it converts both ways exactly and its compares are integer compares.
///
/// A lane vector holds no enum and takes no branch per frame: the countdown is the phase, as it
/// is in [`LinearRamp`], and every update is a select.
#[derive(Clone, Copy, Debug)]
pub struct RampLanes<L: Lane> {
    current: L,
    target: L,
    step: L,
    remaining: L,
}

impl<L: Lane> RampLanes<L> {
    /// Every lane at rest at its word of `value`: target `value`, step `+0.0`, countdown `0`.
    ///
    /// For a loop that knows its ramps are at rest and reads [`RampLanes::current`] alone (the
    /// limiter's stationary dispatch), and for an unused glide weight (`resting(L::zero())`).
    #[must_use]
    #[inline(always)]
    pub fn resting(value: L) -> Self {
        let zero = L::zero();
        Self {
            current: value,
            target: value,
            step: zero,
            remaining: zero,
        }
    }

    /// Lane `l` is the `l`-th ramp of `ramps`. The first [`Lane::WIDTH`] are read; a lane past
    /// the end rests at `+0.0`.
    ///
    /// Panics in debug builds if a ramp's countdown exceeds `2^24`, which no engine ramp reaches.
    #[must_use]
    #[inline(always)]
    pub fn gather<'a, I>(ramps: I) -> Self
    where
        I: IntoIterator<Item = &'a LinearRamp>,
    {
        const { assert!(L::WIDTH <= RAMP_LANES) };
        let mut words = [[0.0_f32; RAMP_LANES]; 4];
        for (lane, ramp) in ramps.into_iter().take(L::WIDTH).enumerate() {
            debug_assert!(ramp.remaining <= EXACT_COUNTDOWN, "an exact countdown");
            words[0][lane] = ramp.current;
            words[1][lane] = ramp.target;
            words[2][lane] = ramp.step;
            words[3][lane] = ramp.remaining as f32;
        }
        Self {
            current: L::load(&words[0]),
            target: L::load(&words[1]),
            step: L::load(&words[2]),
            remaining: L::load(&words[3]),
        }
    }

    /// Writes lane `l` back into the `l`-th ramp of `ramps` (the first [`Lane::WIDTH`]) through
    /// [`LinearRamp::record_advance`]: what an advance can change, `current`, `step` and
    /// `remaining`. Each ramp keeps its own target.
    #[inline(always)]
    pub fn scatter<'a, I>(self, ramps: I)
    where
        I: IntoIterator<Item = &'a mut LinearRamp>,
    {
        let mut words = [[0.0_f32; RAMP_LANES]; 3];
        self.current.store(&mut words[0]);
        self.step.store(&mut words[1]);
        self.remaining.store(&mut words[2]);
        for (lane, ramp) in ramps.into_iter().take(L::WIDTH).enumerate() {
            ramp.record_advance(words[0][lane], words[1][lane], words[2][lane] as u32);
        }
    }

    /// Four lane words as they are, for a lane-word owner that keeps them across blocks rather
    /// than gathering them from [`LinearRamp`]s: the dynamic band's
    /// [`crate::dynamic_band::WordRamp`] (issue #2390). The caller vouches for the phase
    /// invariant its restores check with [`LinearRamp::from_words`]; the walk itself never
    /// reaches a settled lane's words.
    #[inline(always)]
    pub(crate) const fn from_lane_words(current: L, target: L, step: L, remaining: L) -> Self {
        Self {
            current,
            target,
            step,
            remaining,
        }
    }

    /// The four lane words, `[current, target, step, remaining]`: the inverse of
    /// [`RampLanes::from_lane_words`].
    #[inline(always)]
    pub(crate) const fn into_lane_words(self) -> [L; 4] {
        [self.current, self.target, self.step, self.remaining]
    }

    /// The value each lane has reached: the value the previous frame used.
    ///
    /// At rest it is also the value [`RampLanes::advance`] would return, and the advance would
    /// change no word (`tests/ramp_lanes.rs`), so a loop that knows its ramps are at rest may read
    /// this instead of advancing.
    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> L {
        self.current
    }

    /// The lanes with frames still to produce: [`LinearRamp::is_ramping`], lane-wide.
    #[must_use]
    #[inline(always)]
    pub fn in_flight(&self) -> L::Mask {
        self.remaining.gt(L::zero())
    }

    /// One frame of [`LinearRamp::next_value`] on every lane, returning the frame's value.
    ///
    /// `remaining = max(remaining - 1, 0)`, then `current = select(remaining > 0,
    /// ramp_toward(current, step, target), target)` and the step cleared where it is not: the step
    /// added and held inside `[min(current, target), max(current, target)]`, so no word passes
    /// its target (issue #1409), and the last ramping frame an assignment of the target, never an
    /// addition (D11). On a lane at rest every word is unchanged: its `current` is its target and
    /// its step `+0.0`, bit for bit (issue #2368).
    #[inline(always)]
    pub fn advance(&mut self) -> L {
        let zero = L::zero();
        self.remaining = self.remaining.sub(L::splat(1.0)).max(zero);
        let stepping = self.remaining.gt(zero);
        self.current = L::select(
            stepping,
            ramp_toward(self.current, self.step, self.target),
            self.target,
        );
        self.step = L::select(stepping, self.step, zero);
        self.current
    }

    /// [`RampLanes::advance`] on the lanes of `gate` only; every word of every other lane is
    /// untouched. Returns the lanes it advanced that were in flight, which are the lanes
    /// [`LinearRamp::next_value`] would have moved.
    ///
    /// As selects: an in-flight lane with one frame left takes its target and a `+0.0` step, any
    /// other in-flight lane takes `ramp_toward(current, step, target)`, and its countdown drops
    /// by one. A gated loop (the compressor's rate coefficients, which advance only where their
    /// parameter moved) needs it; [`RampLanes::advance`] is the same walk on every lane.
    #[inline(always)]
    pub fn advance_where(&mut self, gate: L::Mask) -> L::Mask {
        let zero = L::zero();
        let one = L::splat(1.0);
        let moving = L::mask_and(gate, self.remaining.gt(zero));
        let last = L::mask_and(moving, self.remaining.eq(one));
        let next = L::select(
            last,
            self.target,
            ramp_toward(self.current, self.step, self.target),
        );
        self.current = L::select(moving, next, self.current);
        self.step = L::select(last, zero, self.step);
        self.remaining = L::select(moving, self.remaining.sub(one), self.remaining);
        moving
    }
}
