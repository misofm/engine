//! The causal gate/expander's single generic per-sample kernel.
//!
//! The detector reads the current main (or connected sidechain) words before either output is
//! written. There is no audio history in this module: a prepared gate has zero added latency.
//! `Lane` keeps the scalar, four-lane and eight-lane paths on one arithmetic body.

use effect_runtime::envelope::HysteresisState;
use lane::{flush, Lane};
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
/// `RAMPING` selects the block-constant parameter update prologue. At each frame both original
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
            let stepped = ramp.current.add(ramp.step);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn causal_kernel_reads_the_current_sample() {
        let coef = GateCoef {
            attack: 1.0_f32,
            release: 1.0,
            hold_samples: 0.0,
            bypass: 1.0,
            link_max: 0.0,
            link_avg: 0.0,
        };
        let mut left = [0.25_f32, -0.5];
        let mut right = [-0.75_f32, 0.125];
        let mut state_left = GateState::default();
        let mut state_right = GateState::default();
        gate_block::<f32, false, false>(GateArgs {
            left: &mut left,
            right: &mut right,
            sidechain: None,
            frames: 2,
            coef: (&coef, &coef),
            state: (&mut state_left, &mut state_right),
        });
        assert_eq!(left, [0.25, -0.5]);
        assert_eq!(right, [-0.75, 0.125]);
    }
}
