//! The causal gate/expander's single generic per-sample kernel.
//!
//! The detector reads the current main (or connected sidechain) words before either output is
//! written. There is no audio history in this module: a prepared gate has zero added latency.
//! `Lane` keeps the scalar, four-lane and eight-lane paths on one arithmetic body.

use effect_runtime::bank::check_block;
use effect_runtime::envelope::HysteresisState;
use lane::kernels::ramp_toward;
use lane::{Lane, flush};
use math::fast_db::{fast_gain_from_db, fast_level_db};

/// Smoothed parameters: threshold, ratio, range and hysteresis, in descriptor order.
pub const RAMP_COUNT: usize = 4;

/// The widest backend and the length of lane scratch arrays owned by this effect.
pub const MAX_WIDTH: usize = 8;

/// Detector level floor, below which the logarithm is not taken.
pub const LEVEL_FLOOR: f32 = 1.0e-8;

/// Lower detector level clamp in dB.
pub const LEVEL_MIN_DB: f32 = -160.0;

/// Upper detector level clamp in dB.
pub const LEVEL_MAX_DB: f32 = 24.0;

/// One smoothed parameter in the precomputed-step form.
#[derive(Clone, Copy, Debug)]
pub struct GateRamp<L: Lane> {
    /// Value used for the current sample.
    pub current: L,
    /// Target assigned exactly on the final ramp sample.
    pub target: L,
    /// Precomputed per-sample increment.
    pub step: L,
    /// Samples remaining, represented as an exact integer-valued lane word.
    pub remaining: L,
}

impl<L: Lane> GateRamp<L> {
    /// A ramp resting at `value`.
    #[must_use]
    pub fn fixed(value: L) -> Self {
        Self {
            current: value,
            target: value,
            step: L::zero(),
            remaining: L::zero(),
        }
    }
}

/// Per-channel coefficients and link selection prepared off the render thread.
#[derive(Clone, Copy, Debug)]
pub struct GateCoef<L: Lane> {
    /// Attack coefficient.
    pub attack: L,
    /// Release coefficient.
    pub release: L,
    /// Rounded hold length in samples.
    pub hold_samples: L,
    /// `1.0` for bypass, otherwise `+0.0`.
    pub bypass: L,
    /// `1.0` for Maximum detector linking.
    pub link_max: L,
    /// `1.0` for Average detector linking.
    pub link_avg: L,
}

/// Per-channel recursive state carried between blocks.
#[derive(Clone, Copy)]
pub struct GateState<L: Lane> {
    /// Smoothed gain reduction in dB.
    pub gain_db: L,
    /// Gate-open flag and hold countdown.
    pub hysteresis: HysteresisState<L>,
    /// Threshold, ratio, range and hysteresis ramps.
    pub ramps: [GateRamp<L>; RAMP_COUNT],
}

impl<L: Lane> Default for GateState<L> {
    fn default() -> Self {
        Self {
            gain_db: L::zero(),
            hysteresis: HysteresisState::default(),
            ramps: [GateRamp::fixed(L::zero()); RAMP_COUNT],
        }
    }
}

/// The words of every lane of `value`, lane `l` at index `l`; past `L::WIDTH` the words are `+0.0`.
fn lane_words<L: Lane>(value: L) -> [f32; MAX_WIDTH] {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words
}

impl<L: Lane> GateCoef<L> {
    /// This channel's coefficients split into one scalar coefficient set per lane (issue #1746).
    /// Stores only: no arithmetic.
    #[must_use]
    pub fn lanes(&self) -> [GateCoef<f32>; MAX_WIDTH] {
        let attack = lane_words(self.attack);
        let release = lane_words(self.release);
        let hold_samples = lane_words(self.hold_samples);
        let bypass = lane_words(self.bypass);
        let link_max = lane_words(self.link_max);
        let link_avg = lane_words(self.link_avg);
        core::array::from_fn(|lane| GateCoef {
            attack: attack[lane],
            release: release[lane],
            hold_samples: hold_samples[lane],
            bypass: bypass[lane],
            link_max: link_max[lane],
            link_avg: link_avg[lane],
        })
    }
}

impl<L: Lane> GateState<L> {
    /// This channel's state split into one scalar state per lane (issue #1746). Stores only: no
    /// arithmetic.
    #[must_use]
    pub fn lanes(&self) -> [GateState<f32>; MAX_WIDTH] {
        let gain_db = lane_words(self.gain_db);
        let open = lane_words(self.hysteresis.open);
        let hold = lane_words(self.hysteresis.hold);
        let ramps = self.ramps.map(|ramp| {
            [
                lane_words(ramp.current),
                lane_words(ramp.target),
                lane_words(ramp.step),
                lane_words(ramp.remaining),
            ]
        });
        core::array::from_fn(|lane| GateState {
            gain_db: gain_db[lane],
            hysteresis: HysteresisState {
                open: open[lane],
                hold: hold[lane],
            },
            ramps: ramps.map(|[current, target, step, remaining]| GateRamp {
                current: current[lane],
                target: target[lane],
                step: step[lane],
                remaining: remaining[lane],
            }),
        })
    }
}

/// Whether one lane of one channel is at its exact rest (issue #1746): a block of exact `+0.0`
/// on every input of the lane leaves every one of its state words bit-identical and writes exact
/// `+0.0`.
///
/// # The words, and why the list is complete
///
/// A channel's state is [`GateState`]: `gain_db`, the hysteresis `open` flag and `hold` countdown,
/// and the four ramps' `current`, `target`, `step` and `remaining`. [`GateCoef`] is written only by
/// the control plane (preparation, restore, D7 recovery), never by [`gate_block`]. Nothing else
/// crosses a block: the gate keeps no audio history.
///
/// * **Every ramp settled** (`remaining == 0`). A settled ramp is an exact no-op under both
///   `RAMPING` variants (`moving` and `final_sample` are false, so every field is selected back),
///   so its four fields are pinned and the frame map does not depend on where `run_block` splits
///   the block. A ramp in flight refuses the lane.
/// * **`gain_db`, `open`, `hold`: a one-frame fixed point.** They are not a closed form: with a
///   `+0.0` detector the level is the clamped floor, the gate may stay closed (hold untouched) or
///   stay open with `hold` reloaded, and `gain_db` stalls at a bit-stable value that need not equal
///   the curve target. So the kernel's own [`channel_step`] runs one frame on a copy, with own,
///   partner and dry all `+0.0` (every link mode's detector is then `+0.0`), and the three words
///   must come out bit-identical and the output `+0.0` by bits. With settled ramps and constant
///   input a block is `frames` applications of that same frame map to the same state, so by
///   induction a one-frame fixed point is a whole-block fixed point, output included.
/// * **`gain_db` inside the D7 bound.** `finish_block` resets a lane whose gain word fails
///   `check_block` (non-finite, or at least `BLOCK_LIMIT`), which would move state; such a lane
///   is refused. The output word `+0.0` always passes it.
///
/// The output leg (`+0.0` by bits) is implied by the others in every reachable state: the dry
/// word is `+0.0` and, for a finite gain word, the applied gain is finite and non-negative. It is
/// kept so the claim's output clause is checked rather than argued.
///
/// The scalar `f32` lane is the bit oracle of every vector backend (one arithmetic body; `fma` is
/// two roundings everywhere), so the answer evaluated here is the vector kernel's.
///
/// # Why `#[inline(never)]` and not generic
///
/// The wasm kernel roster (`scripts/check-web-audioworklet-callgraph.py`) requires exactly one
/// `gate_expander` function carrying `f32x4` arithmetic, `process_bank`, under a scalar budget it
/// nearly fills. This function takes plain `f32` values, so its name carries no `f32x4`, and kept
/// out of line its scalar step is never folded into `process_bank`.
#[inline(never)]
#[must_use]
pub fn channel_at_rest(coef: &GateCoef<f32>, state: &GateState<f32>) -> bool {
    if state.ramps.iter().any(|ramp| ramp.remaining != 0.0) {
        return false;
    }
    let mut next = *state;
    let out = channel_step::<f32, false>(coef, &mut next, 0.0, 0.0, 0.0);
    out.to_bits() == 0
        && next.gain_db.to_bits() == state.gain_db.to_bits()
        && next.hysteresis.open.to_bits() == state.hysteresis.open.to_bits()
        && next.hysteresis.hold.to_bits() == state.hysteresis.hold.to_bits()
        && check_block::<f32>(&[next.gain_db])
}

/// Arguments borrowed by one causal render block.
pub struct GateArgs<'a, L: Lane> {
    /// Main left channel, read and written in place.
    pub left: &'a mut [f32],
    /// Main right channel, read and written in place.
    pub right: &'a mut [f32],
    /// Connected sidechain channels, present exactly when `CONNECTED` is true.
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    /// Number of frames.
    pub frames: usize,
    /// Left and right prepared coefficients.
    pub coef: (&'a GateCoef<L>, &'a GateCoef<L>),
    /// Left and right mutable state.
    pub state: (&'a mut GateState<L>, &'a mut GateState<L>),
}

/// Runs one causal gate block.
///
/// `RAMPING` selects the block-constant parameter update prologue, which advances each moving
/// parameter word by `ramp_toward(current, step, target)` (never past its target, issue #1409)
/// and assigns the target on its final sample. At each frame both original
/// main words are loaded before either channel writes its output, preserving current-sample
/// causality and signed-zero identity in bypass and zero-gain paths.
#[inline(always)]
pub fn gate_block<L: Lane, const CONNECTED: bool, const RAMPING: bool>(args: GateArgs<'_, L>) {
    let GateArgs {
        left,
        right,
        sidechain,
        frames,
        coef,
        state,
    } = args;
    let width = L::WIDTH;
    debug_assert_eq!(left.len(), frames * width);
    debug_assert_eq!(right.len(), frames * width);
    debug_assert_eq!(CONNECTED, sidechain.is_some());
    let empty: &[f32] = &[];
    let (side_left, side_right) = sidechain.unwrap_or((empty, empty));
    if CONNECTED {
        debug_assert_eq!(side_left.len(), frames * width);
        debug_assert_eq!(side_right.len(), frames * width);
    }
    let (coef_left, coef_right) = coef;
    let (state_left, state_right) = state;

    for frame in 0..frames {
        let span = frame * width;
        // Load all source words before either channel writes its output.
        let main_left = L::load(&left[span..span + width]);
        let main_right = L::load(&right[span..span + width]);
        let (detector_left, detector_right) = if CONNECTED {
            (
                L::load(&side_left[span..span + width]),
                L::load(&side_right[span..span + width]),
            )
        } else {
            (main_left, main_right)
        };
        let out_left = channel_step::<L, RAMPING>(
            coef_left,
            state_left,
            detector_left,
            detector_right,
            main_left,
        );
        let out_right = channel_step::<L, RAMPING>(
            coef_right,
            state_right,
            detector_right,
            detector_left,
            main_right,
        );
        out_left.store(&mut left[span..span + width]);
        out_right.store(&mut right[span..span + width]);
    }
}

/// One current-sample step of one channel.
#[inline(always)]
// FAST-DB-CROSSING X3/X4: dynamics level and applied gain, never pinned coefficients.
#[expect(
    clippy::disallowed_methods,
    reason = "gate detector level and applied gain are the sealed fast-db crossings"
)]
fn channel_step<L: Lane, const RAMPING: bool>(
    coef: &GateCoef<L>,
    state: &mut GateState<L>,
    own: L,
    partner: L,
    dry: L,
) -> L {
    let zero = L::zero();
    let one = L::splat(1.0);

    if RAMPING {
        for ramp in &mut state.ramps {
            let moving = ramp.remaining.gt(zero);
            let final_sample = ramp.remaining.eq(one);
            // Issue #1409: the step added, held inside `[min(current, target), max(current,
            // target)]`, so no parameter word passes its target before the snap.
            let stepped = ramp_toward(ramp.current, ramp.step, ramp.target);
            ramp.current = L::select(
                moving,
                L::select(final_sample, ramp.target, stepped),
                ramp.current,
            );
            ramp.step = L::select(final_sample, zero, ramp.step);
            ramp.remaining = L::select(moving, ramp.remaining.sub(one), ramp.remaining);
        }
    }

    let threshold = state.ramps[0].current;
    let ratio = state.ramps[1].current;
    let range = state.ramps[2].current;
    let hysteresis = state.ramps[3].current;
    let own_abs = own.abs();
    let partner_abs = partner.abs();
    let mut level = own_abs;
    level = L::select(coef.link_max.gt(zero), own_abs.max(partner_abs), level);
    level = L::select(
        coef.link_avg.gt(zero),
        L::splat(0.5)
            .mul(own_abs)
            .add(L::splat(0.5).mul(partner_abs)),
        level,
    );
    let level_db = fast_level_db(level.max(L::splat(LEVEL_FLOOR)))
        .min(L::splat(LEVEL_MAX_DB))
        .max(L::splat(LEVEL_MIN_DB));

    // Closed opens at >= T. Open re-arms at >= T-H; below the band it remains open while the
    // prior countdown is positive, decrementing once, and closes only when it was already zero.
    let was_open = state.hysteresis.open.gt(zero);
    let above_open = level_db.ge(threshold);
    let above_rearm = level_db.ge(threshold.sub(hysteresis));
    let opening = L::mask_and(L::mask_not(was_open), above_open);
    let rearm = L::mask_and(was_open, above_rearm);
    let reload = L::mask_or(opening, rearm);
    let holding = L::mask_and(
        was_open,
        L::mask_and(L::mask_not(above_rearm), state.hysteresis.hold.gt(zero)),
    );
    state.hysteresis.open = L::select(L::mask_or(reload, holding), one, zero);
    state.hysteresis.hold = L::select(
        reload,
        coef.hold_samples,
        L::select(
            holding,
            state.hysteresis.hold.sub(one),
            state.hysteresis.hold,
        ),
    );

    let curve = ratio
        .sub(one)
        .mul(level_db.sub(threshold))
        .max(range.neg())
        .min(zero);
    let target = L::select(state.hysteresis.open.gt(zero), zero, curve);
    let rate = L::select(target.gt(state.gain_db), coef.attack, coef.release);
    let gain_db = flush(rate.fma(target.sub(state.gain_db), state.gain_db));
    state.gain_db = gain_db;
    let gain = fast_gain_from_db(gain_db);
    let identity = L::mask_or(gain_db.eq(zero), coef.bypass.gt(zero));
    L::select(identity, dry, dry.mul(gain))
}
