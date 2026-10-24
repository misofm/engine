//! The dynamic EQ's one generic per-sample kernel (issue #17 S1).
//!
//! Per channel and frame: the live words step one sample along their ramps, then the four bands
//! run in index order, each on the previous band's output. A band is the shared dynamic band step
//! (`effect_runtime::dynamic_band`, also the de-esser's): one TPT SVF whose pole words are the
//! parametric EQ's bell design at the band's **static** gain, its unity-peak band-pass tap
//! `k v1` as the detector (or a second SVF with the same words on a connected sidechain), the
//! gain law smoothed in dB, and this module's output law, the bell with a modulated band mix:
//!
//! ```text
//! m1[n] = m1 + k A^2 (g[n] - 1) = k (A^2 g[n] - 1),   y = x + m1[n] v1
//! ```
//!
//! With `g = 1` the mix word is the EQ's own `m1` word exactly, so an idle band is #87's bell.
//! `Lane` keeps the scalar, four-lane and eight-lane paths on this one body.

use effect_runtime::dynamic_band::{
    BandWords, DynamicBandCoef, LinkWords, applied_gain, band_step, detector_level_db,
    dynamic_band_gain,
};
use effect_runtime::dynamics::GainComputerCoef;
use lane::kernels::{SvfState, ramp_toward};
use lane::{Lane, silence_step};

/// The widest backend and the length of lane scratch arrays owned by this effect.
pub const MAX_WIDTH: usize = 8;

/// Bands per instance, cascaded in index order (the parametric EQ's general band count).
pub const BANDS: usize = 4;

/// Live words per band, each a 64-sample ramp ([`WordRamp`]), in this order.
pub const BAND_RAMPS: usize = 14;
/// Threshold in dB.
pub const THRESHOLD: usize = 0;
/// The gain computer's slope `1/R - 1`.
pub const SLOPE: usize = 1;
/// Knee width in dB; the curve's two knee words are designed from it ([`knee_words`]).
pub const KNEE: usize = 2;
/// Range in dB (the largest change either way).
pub const RANGE: usize = 3;
/// Attack one-pole rate coefficient.
pub const ATTACK: usize = 4;
/// Release one-pole rate coefficient.
pub const RELEASE: usize = 5;
/// SVF word `c1`; `a2` and `a3` follow it.
pub const C1: usize = 6;
/// The damping word `k = 1/(Q A)`, the detector tap's scale.
pub const K: usize = 9;
/// The static band mix word `m1 = k (A^2 - 1)`, the parametric EQ's bell word.
pub const M1: usize = 10;
/// `k A^2 = A / Q`, the mix word's slope in the linear dynamic gain.
pub const KA2: usize = 11;
/// `1.0` for an upward band (a boost above the threshold), `+0.0` for downward.
pub const UPWARD: usize = 12;
/// `1.0` for the external (sidechain) detector, `+0.0` for the band's own tap.
pub const EXTERNAL: usize = 13;
/// The six words a disabled band holds at `+0.0` (the parametric EQ's identity section).
pub const DESIGN_WORDS: [usize; 6] = [C1, C1 + 1, C1 + 2, K, M1, KA2];

/// Live words per channel.
pub const RAMPS: usize = BANDS * BAND_RAMPS;

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

/// One band's running state on one channel.
#[derive(Clone, Copy)]
pub struct Band<L: Lane> {
    /// The band's SVF integrators on the cascade signal.
    pub main: SvfState<L>,
    /// The sidechain detector SVF's integrators; `+0.0` while the sidechain is unconnected.
    pub side: SvfState<L>,
    /// Smoothed gain change in dB: `<= 0` downward, `>= 0` upward.
    pub gain_db: L,
    /// `W/2` of the current knee word, derived ([`knee_words`]), not carried in the payload.
    pub half_knee: L,
    /// `1/(2W)` of the current knee word, derived, `+0.0` for a hard knee.
    pub inv_two_knee: L,
}

/// Per-channel state carried between blocks.
#[derive(Clone, Copy)]
pub struct Chan<L: Lane> {
    /// The bands, in cascade order.
    pub bands: [Band<L>; BANDS],
    /// Exactly-zero effect-input frames, the clock that arms every main SVF's joint flush (#1328).
    pub run: L,
    /// The sidechain's silence clock, which arms the detector SVFs' flush.
    pub side_run: L,
    /// The live words, band `b`'s at `b * BAND_RAMPS` plus the constants above.
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
        let band = Band {
            main: SvfState::default(),
            side: SvfState::default(),
            gain_db: zero,
            half_knee: zero,
            inv_two_knee: zero,
        };
        Self {
            bands: [band; BANDS],
            run: zero,
            side_run: zero,
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
    /// Connected sidechain channels; `None` on a connected instance is a silent sidechain.
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    /// `N_SILENCE` at the prepared rate (`lane::silence_frames`).
    pub armed_after: L,
    /// Bands that may change a word this segment; a band that cannot is skipped (see
    /// `DynamicEq::live_bands`).
    pub live: [bool; BANDS],
    /// Left and right prepared words.
    pub coef: (&'a Coef<L>, &'a Coef<L>),
    /// Left and right state.
    pub state: (&'a mut Chan<L>, &'a mut Chan<L>),
}

/// The curve's knee words of a knee width `w` in dB: `(W/2, 1/(2W))` where `W > 0` and the
/// reciprocal is below `+inf`, else `(+0, +0)`, the same `f32` operations as
/// `effect_runtime::dynamics::knee_coefficients`, lane by lane. The control plane and the ramping
/// frames both use it, so a settled knee holds the bits its design gives.
#[inline(always)]
pub fn knee_words<L: Lane>(w: L) -> (L, L) {
    let zero = L::zero();
    let inv_two_knee = L::splat(1.0).div(L::splat(2.0).mul(w));
    let soft = L::mask_and(w.gt(zero), inv_two_knee.lt(L::splat(f32::INFINITY)));
    (
        L::select(soft, L::splat(0.5).mul(w), zero),
        L::select(soft, inv_two_knee, zero),
    )
}

/// Rewrites every band's derived knee words from its current knee word.
#[inline(always)]
pub fn refresh_knees<L: Lane>(s: &mut Chan<L>) {
    for (band, state) in s.bands.iter_mut().enumerate() {
        let (half, inv) = knee_words(s.ramps[band * BAND_RAMPS + KNEE].current);
        state.half_knee = half;
        state.inv_two_knee = inv;
    }
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

/// The ramps of a frame and the frame's rest thresholds: the effect input's for every main SVF
/// and the sidechain's for every detector SVF. An unarmable segment (`ARMABLE = false`) had its
/// clocks advanced before the frame loop (`lane::kernels::silence_skip_block`) and reads none.
#[inline(always)]
fn open<L: Lane, const RAMPING: bool, const CONNECTED: bool, const ARMABLE: bool>(
    s: &mut Chan<L>,
    x: L,
    side: L,
    armed_after: L,
) -> (L, L) {
    if RAMPING {
        advance(&mut s.ramps);
        refresh_knees(s);
    }
    if !ARMABLE {
        return (L::zero(), L::zero());
    }
    let rest = silence_step(x, &mut s.run, armed_after);
    let side_rest = if CONNECTED {
        silence_step(side, &mut s.side_run, armed_after)
    } else {
        L::zero()
    };
    (rest, side_rest)
}

/// One band's front half on one channel: its SVF on the cascade word `u` and its detector tap,
/// the band's own unity-peak band-pass `k v1` or, when connected, crossfaded by the detector word
/// toward the same filter on the sidechain word. Returns `(v1, detector)`.
#[inline(always)]
fn band_front<L: Lane, const CONNECTED: bool, const ARMABLE: bool>(
    band: usize,
    s: &mut Chan<L>,
    u: L,
    side: L,
    rest: (L, L),
) -> (L, L) {
    let r = &s.ramps[band * BAND_RAMPS..(band + 1) * BAND_RAMPS];
    let words = BandWords {
        c1: r[C1].current,
        a2: r[C1 + 1].current,
        a3: r[C1 + 2].current,
        k: r[K].current,
    };
    let external = r[EXTERNAL].current;
    let state = &mut s.bands[band];
    let (v1, _, tap) = band_step(ARMABLE, u, &words, rest.0, &mut state.main);
    let detector = if CONNECTED {
        let (_, _, side_tap) = band_step(ARMABLE, side, &words, rest.1, &mut state.side);
        blend(tap, side_tap, external)
    } else {
        tap
    };
    (v1, detector)
}

/// One band's back half on one channel: the linked level, the gain law, and the modulated bell
/// mix on the cascade word `u`. A zero mix word (an idle 0 dB band, a disabled band) selects `u`
/// itself, so such a band is the identity bit for bit.
#[inline(always)]
fn band_back<L: Lane>(
    band: usize,
    c: &Coef<L>,
    s: &mut Chan<L>,
    own: L,
    partner: L,
    u: L,
    v1: L,
) -> L {
    let zero = L::zero();
    let link = LinkWords {
        maximum: c.link_max,
        average: c.link_avg,
    };
    let level_db = detector_level_db(own, partner, &link);
    let r = &s.ramps[band * BAND_RAMPS..(band + 1) * BAND_RAMPS];
    let state = &mut s.bands[band];
    let law = DynamicBandCoef {
        curve: GainComputerCoef {
            threshold_db: r[THRESHOLD].current,
            inv_ratio_minus_one: r[SLOPE].current,
            half_knee_db: state.half_knee,
            inv_two_knee: state.inv_two_knee,
        },
        range_db: r[RANGE].current,
        // The direction switches at the midpoint of its ramp, either way.
        upward: r[UPWARD].current.sub(L::splat(0.5)),
        attack: r[ATTACK].current,
        release: r[RELEASE].current,
    };
    let gain_db = dynamic_band_gain(level_db, &law, &mut state.gain_db);
    let g = applied_gain(gain_db);
    let g_minus_one = L::select(gain_db.eq(zero), zero, g.sub(L::splat(1.0)));
    let mix = r[KA2].current.fma(g_minus_one, r[M1].current);
    L::select(mix.eq(zero), u, mix.fma(v1, u))
}

/// One render segment of `left.len() / L::WIDTH` frames.
///
/// Per frame, band by band, both channels' fronts run before either back, so a linked detector
/// reads both current detector taps of that band. `MONO` (issue #973) is the collapsed body: the
/// left plane holds both channels, the left word is its own partner, and only the left state
/// moves, so the left plane and state are bit-identical to a dual segment whose right plane equals
/// its left. A collapsed bank is never connected.
#[inline(always)]
pub fn segment<
    L: Lane,
    const CONNECTED: bool,
    const RAMPING: bool,
    const MONO: bool,
    const ARMABLE: bool,
>(
    args: Args<'_, L>,
) {
    let Args {
        left,
        right,
        sidechain,
        armed_after,
        live,
        coef: (coef_left, coef_right),
        state: (state_left, state_right),
    } = args;
    let width = L::WIDTH;
    debug_assert!(CONNECTED || sidechain.is_none());
    debug_assert!(!(MONO && CONNECTED));
    let mut l = *state_left;
    let mut r = *state_right;
    for frame in 0..left.len() / width {
        let span = frame * width..frame * width + width;
        let x_left = L::load(&left[span.clone()]);
        if MONO {
            let rest = open::<L, RAMPING, false, ARMABLE>(&mut l, x_left, x_left, armed_after);
            let mut u = x_left;
            for (band, _) in live.iter().enumerate().filter(|(_, live)| **live) {
                let (v1, det) = band_front::<L, false, ARMABLE>(band, &mut l, u, u, rest);
                u = band_back(band, coef_left, &mut l, det, det, u, v1);
            }
            L::select(coef_left.bypass.gt(L::zero()), x_left, u).store(&mut left[span]);
            continue;
        }
        let x_right = L::load(&right[span.clone()]);
        let (s_left, s_right) = match sidechain {
            Some((side_left, side_right)) if CONNECTED => (
                L::load(&side_left[span.clone()]),
                L::load(&side_right[span.clone()]),
            ),
            _ => (L::zero(), L::zero()),
        };
        let rest_l = open::<L, RAMPING, CONNECTED, ARMABLE>(&mut l, x_left, s_left, armed_after);
        let rest_r = open::<L, RAMPING, CONNECTED, ARMABLE>(&mut r, x_right, s_right, armed_after);
        let (mut u_left, mut u_right) = (x_left, x_right);
        for (band, _) in live.iter().enumerate().filter(|(_, live)| **live) {
            let (v1_l, det_l) =
                band_front::<L, CONNECTED, ARMABLE>(band, &mut l, u_left, s_left, rest_l);
            let (v1_r, det_r) =
                band_front::<L, CONNECTED, ARMABLE>(band, &mut r, u_right, s_right, rest_r);
            u_left = band_back(band, coef_left, &mut l, det_l, det_r, u_left, v1_l);
            u_right = band_back(band, coef_right, &mut r, det_r, det_l, u_right, v1_r);
        }
        L::select(coef_left.bypass.gt(L::zero()), x_left, u_left).store(&mut left[span.clone()]);
        L::select(coef_right.bypass.gt(L::zero()), x_right, u_right).store(&mut right[span]);
    }
    *state_left = l;
    if !MONO {
        *state_right = r;
    }
}
