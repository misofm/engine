//! The de-esser's one generic per-sample kernel (issue #15 S1).
//!
//! Per channel and frame: the twelve live words step one sample along their ramps, the shared
//! dynamic band step (`effect_runtime::dynamic_band`, also the dynamic EQ's) filters the main
//! input into its band-pass tap and gives the high-pass tap `x - k v1 - v2` beside it, a
//! connected sidechain runs a second SVF for the detector only, and the band's hard-knee,
//! downward gain law smooths the reduction in dB. This module owns only the de-esser's output
//! law: the wideband, split and listen arms. `Lane` keeps the scalar, four-lane and eight-lane
//! paths on this one body.

use effect_runtime::dynamic_band::{
    BandWords, DynamicBandCoef, LinkWords, applied_gain, band_step, detector_level_db,
    dynamic_band_gain,
};
use effect_runtime::dynamics::GainComputerCoef;
use lane::kernels::{SvfState, ramp_toward};
use lane::{Lane, silence_step};

/// The widest backend and the length of lane scratch arrays owned by this effect.
pub const MAX_WIDTH: usize = 8;

/// Live words per channel, each a 64-sample ramp ([`WordRamp`]), in this order.
pub const RAMPS: usize = 12;
/// Threshold in dB.
pub const THRESHOLD: usize = 0;
/// The gain computer's slope `1/R - 1`.
pub const SLOPE: usize = 1;
/// Range in dB (the deepest reduction).
pub const RANGE: usize = 2;
/// Attack one-pole rate coefficient.
pub const ATTACK: usize = 3;
/// Release one-pole rate coefficient.
pub const RELEASE: usize = 4;
/// SVF word `c1`; `a2` and `a3` follow it.
pub const C1: usize = 5;
/// The damping word `k = 1/Q`, the taps' mix word.
pub const K: usize = 8;
/// `1.0` for split-band attenuation, `+0.0` for wideband.
pub const MODE: usize = 9;
/// `1.0` for the high-pass detector tap, `+0.0` for the band-pass tap.
pub const DETECTOR: usize = 10;
/// `1.0` to output the filter tap itself (listen).
pub const LISTEN: usize = 11;

/// One live word in the precomputed-step form of `effect_runtime::ramp` (D11).
#[derive(Clone, Copy, Debug)]
pub struct WordRamp<L: Lane> {
    /// Value used for the current sample.
    pub current: L,
    /// Target assigned exactly on the final ramp sample.
    pub target: L,
    /// Precomputed per-sample increment.
    pub step: L,
    /// Samples remaining, an exact integer-valued lane word.
    pub remaining: L,
}

/// Per-channel words written only by the control plane.
#[derive(Clone, Copy, Debug)]
pub struct Coef<L: Lane> {
    /// `1.0` for the prepared bypass, otherwise `+0.0`.
    pub bypass: L,
    /// `1.0` for Maximum detector linking.
    pub link_max: L,
    /// `1.0` for Average detector linking.
    pub link_avg: L,
}

/// Per-channel state carried between blocks.
#[derive(Clone, Copy)]
pub struct Chan<L: Lane> {
    /// The main input's SVF integrators.
    pub main: SvfState<L>,
    /// The sidechain's SVF integrators; `+0.0` while the sidechain is unconnected.
    pub side: SvfState<L>,
    /// Exactly-zero main input frames, the clock that arms the SVF's joint flush (#1328).
    pub run: L,
    /// The sidechain's silence clock.
    pub side_run: L,
    /// Smoothed gain change in dB, `<= 0`.
    pub gain_db: L,
    /// The live words, indexed by the constants above.
    pub ramps: [WordRamp<L>; RAMPS],
}

impl<L: Lane> Default for Chan<L> {
    fn default() -> Self {
        let zero = L::zero();
        let ramp = WordRamp {
            current: zero,
            target: zero,
            step: zero,
            remaining: zero,
        };
        Self {
            main: SvfState::default(),
            side: SvfState::default(),
            run: zero,
            side_run: zero,
            gain_db: zero,
            ramps: [ramp; RAMPS],
        }
    }
}

/// Arguments borrowed by one render segment.
pub struct Args<'a, L: Lane> {
    /// Main left channel, read and written in place.
    pub left: &'a mut [f32],
    /// Main right channel; empty and never touched in the collapsed body.
    pub right: &'a mut [f32],
    /// Connected sidechain channels, present exactly when `CONNECTED` is true.
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    /// `N_SILENCE` at the prepared rate (`lane::silence_frames`).
    pub armed_after: L,
    /// Left and right prepared words.
    pub coef: (&'a Coef<L>, &'a Coef<L>),
    /// Left and right state.
    pub state: (&'a mut Chan<L>, &'a mut Chan<L>),
}

/// `a` at `w == 0`, `b` at `w == 1`, `a + w (b - a)` between: a live switch's crossfade. The
/// endpoints are selected, so a settled switch renders its arm's own bits.
#[inline(always)]
fn blend<L: Lane>(a: L, b: L, w: L) -> L {
    let mixed = w.fma(b.sub(a), a);
    L::select(w.eq(L::zero()), a, L::select(w.eq(L::splat(1.0)), b, mixed))
}

/// Advances every ramp one sample: `ramp_toward` (never past the target, #1409), the target on
/// the final sample. A settled ramp selects every field back, an exact no-op.
#[inline(always)]
fn advance<L: Lane>(ramps: &mut [WordRamp<L>; RAMPS]) {
    let zero = L::zero();
    let one = L::splat(1.0);
    for ramp in ramps {
        let moving = ramp.remaining.gt(zero);
        let last = ramp.remaining.eq(one);
        let stepped = ramp_toward(ramp.current, ramp.step, ramp.target);
        ramp.current = L::select(moving, L::select(last, ramp.target, stepped), ramp.current);
        ramp.step = L::select(last, zero, ramp.step);
        ramp.remaining = L::select(moving, ramp.remaining.sub(one), ramp.remaining);
    }
}

/// One SVF step on `x` and the selected tap: band `k v1` (0 dB at the centre) or high
/// `x - k v1 - v2`, crossfaded by the detector word. The joint flush is armed by `x`'s own
/// silence clock.
#[inline(always)]
fn filter<L: Lane>(
    x: L,
    s: &mut SvfState<L>,
    run: &mut L,
    words: &[WordRamp<L>; RAMPS],
    armed_after: L,
) -> L {
    let rest = silence_step(x, run, armed_after);
    let band_words = BandWords {
        c1: words[C1].current,
        a2: words[C1 + 1].current,
        a3: words[C1 + 2].current,
        k: words[K].current,
    };
    let (_, v2, band) = band_step(x, &band_words, rest, s);
    let high = x.sub(band).sub(v2);
    blend(band, high, words[DETECTOR].current)
}

/// One channel's front half: the ramps, the main tap and the detector tap.
#[inline(always)]
fn front<L: Lane, const RAMPING: bool, const CONNECTED: bool>(
    s: &mut Chan<L>,
    x: L,
    side: L,
    armed_after: L,
) -> (L, L) {
    if RAMPING {
        advance(&mut s.ramps);
    }
    let tap = filter(x, &mut s.main, &mut s.run, &s.ramps, armed_after);
    let detector = if CONNECTED {
        filter(side, &mut s.side, &mut s.side_run, &s.ramps, armed_after)
    } else {
        tap
    };
    (tap, detector)
}

/// One channel's back half: the linked level, the downward hard-knee gain law and the output
/// arm. A zero gain word selects the dry word on the wideband and split arms, so the split arm at
/// rest is `x` exactly, not `x + 0 b`.
#[inline(always)]
fn back<L: Lane>(c: &Coef<L>, s: &mut Chan<L>, own: L, partner: L, x: L, tap: L) -> L {
    let zero = L::zero();
    let link = LinkWords {
        maximum: c.link_max,
        average: c.link_avg,
    };
    let level_db = detector_level_db(own, partner, &link);
    let r = &s.ramps;
    let law = DynamicBandCoef {
        curve: GainComputerCoef {
            threshold_db: r[THRESHOLD].current,
            inv_ratio_minus_one: r[SLOPE].current,
            half_knee_db: zero,
            inv_two_knee: zero,
        },
        range_db: r[RANGE].current,
        upward: zero,
        attack: r[ATTACK].current,
        release: r[RELEASE].current,
    };
    let gain_db = dynamic_band_gain(level_db, &law, &mut s.gain_db);
    let g = applied_gain(gain_db);
    let unity = gain_db.eq(zero);
    let wide = L::select(unity, x, x.mul(g));
    let split = L::select(unity, x, g.sub(L::splat(1.0)).fma(tap, x));
    let r = &s.ramps;
    let out = blend(blend(wide, split, r[MODE].current), tap, r[LISTEN].current);
    L::select(c.bypass.gt(zero), x, out)
}

/// One render segment of `left.len() / L::WIDTH` frames.
///
/// Both channels' fronts run before either back, so a linked detector reads both current
/// detector taps, and both main words are loaded before either output is written. `MONO`
/// (issue #973) is the collapsed body: the left plane holds both channels, the left word is its
/// own partner, and only the left state moves, so the left plane and state are bit-identical to a
/// dual segment whose right plane equals its left. A collapsed bank is never connected.
#[inline(always)]
pub fn segment<L: Lane, const CONNECTED: bool, const RAMPING: bool, const MONO: bool>(
    args: Args<'_, L>,
) {
    let Args {
        left,
        right,
        sidechain,
        armed_after,
        coef: (coef_left, coef_right),
        state: (state_left, state_right),
    } = args;
    let width = L::WIDTH;
    let empty: &[f32] = &[];
    let (side_left, side_right) = sidechain.unwrap_or((empty, empty));
    debug_assert_eq!(CONNECTED, sidechain.is_some());
    debug_assert!(!(MONO && CONNECTED));
    let mut l = *state_left;
    let mut r = *state_right;
    for frame in 0..left.len() / width {
        let span = frame * width..frame * width + width;
        let x_left = L::load(&left[span.clone()]);
        if MONO {
            let (tap, detector) = front::<L, RAMPING, false>(&mut l, x_left, x_left, armed_after);
            back(coef_left, &mut l, detector, detector, x_left, tap).store(&mut left[span]);
            continue;
        }
        let x_right = L::load(&right[span.clone()]);
        let (s_left, s_right) = if CONNECTED {
            (
                L::load(&side_left[span.clone()]),
                L::load(&side_right[span.clone()]),
            )
        } else {
            (x_left, x_right)
        };
        let (tap_l, det_l) = front::<L, RAMPING, CONNECTED>(&mut l, x_left, s_left, armed_after);
        let (tap_r, det_r) = front::<L, RAMPING, CONNECTED>(&mut r, x_right, s_right, armed_after);
        let out_left = back(coef_left, &mut l, det_l, det_r, x_left, tap_l);
        let out_right = back(coef_right, &mut r, det_r, det_l, x_right, tap_r);
        out_left.store(&mut left[span.clone()]);
        out_right.store(&mut right[span]);
    }
    *state_left = l;
    if !MONO {
        *state_right = r;
    }
}
