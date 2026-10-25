//! The dynamic EQ's one generic per-sample kernel (issue #17 S1).
//!
//! Per channel and frame: the live words step one sample along their ramps, then the four bands
//! run in index order, each on the previous band's output. A band is the shared dynamic band step
//! (`effect_runtime::dynamic_band`, also the de-esser's): one TPT SVF whose pole words are the
//! parametric EQ's bell design at the band's **static** gain, a detector SVF at the band's own
//! quality `Q` (its unity-peak band-pass, on the band's input or on a connected sidechain), the
//! gain law smoothed in dB, and this module's output law, the bell with a modulated band mix:
//!
//! ```text
//! m1[n] = m1 + k A^2 (g[n] - 1) = k (A^2 g[n] - 1),   y = x + m1[n] v1
//! ```
//!
//! With `g = 1` the mix word is the EQ's own `m1` word exactly, so an idle band is #87's bell.
//! `Lane` keeps the scalar, four-lane and eight-lane paths on this one body.

pub use effect_runtime::dynamic_band::WordRamp;
use effect_runtime::dynamic_band::{
    BandWords, DynamicBandCoef, LinkWords, advance_word_ramps, applied_gain, band_step, blend,
    detector_level_db, dynamic_band_gain,
};
use effect_runtime::dynamics::GainComputerCoef;
use lane::kernels::SvfState;
use lane::{Lane, silence_step};

/// The widest backend and the length of lane scratch arrays owned by this effect.
pub const MAX_WIDTH: usize = 8;

/// Bands per instance, cascaded in index order (the parametric EQ's general band count).
pub const BANDS: usize = 4;

/// Live words per band, each a 64-sample ramp ([`WordRamp`]), in this order.
pub const BAND_RAMPS: usize = 17;
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
/// The static band mix word `m1 = k (A^2 - 1)` (`k = 1/(Q A)`), the parametric EQ's bell word.
pub const M1: usize = 9;
/// `k A^2 = A / Q`, the mix word's slope in the linear dynamic gain.
pub const KA2: usize = 10;
/// `1.0` for an upward band (a boost above the threshold), `+0.0` for downward.
pub const UPWARD: usize = 11;
/// `1.0` for the external (sidechain) detector, `+0.0` for the band's own input.
pub const EXTERNAL: usize = 12;
/// The detector SVF's word `c1` at the band's quality `Q` (`k = 1/Q`, any static gain); `a2` and
/// `a3` follow it.
pub const DC1: usize = 13;
/// The detector SVF's damping `1/Q`, its band-pass tap's scale.
pub const DK: usize = 16;
/// The nine words a disabled band holds at `+0.0` (the parametric EQ's identity section, and a
/// detector that integrates nothing).
pub const DESIGN_WORDS: [usize; 9] = [C1, C1 + 1, C1 + 2, M1, KA2, DC1, DC1 + 1, DC1 + 2, DK];

/// Live words per channel.
pub const RAMPS: usize = BANDS * BAND_RAMPS;

/// The detector link glide weights per channel (issue #1969, #1370 D2): `w_link`, then `w_avg`.
pub const LINK_COUNT: usize = 2;

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
    /// The internal detector SVF's integrators, on the same cascade signal at quality `Q`.
    pub detector: SvfState<L>,
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
    /// The detector link glide weights (issue #1969, #1370 D2), `w_link` then `w_avg`, one pair
    /// for every band of the channel: at rest each is exactly `effect_contract::link_glide_weights`
    /// of the lane's mode, the mode [`Coef::link_max`] and [`Coef::link_avg`] select. Only a
    /// `GLIDE` segment reads them.
    pub links: [WordRamp<L>; LINK_COUNT],
}

impl<L: Lane> Default for Chan<L> {
    fn default() -> Self {
        let zero = L::zero();
        let band = Band {
            main: SvfState::default(),
            detector: SvfState::default(),
            side: SvfState::default(),
            gain_db: zero,
            half_knee: zero,
            inv_two_knee: zero,
        };
        Self {
            bands: [band; BANDS],
            run: zero,
            side_run: zero,
            ramps: [WordRamp::zero(); RAMPS],
            links: [WordRamp::zero(); LINK_COUNT],
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

/// The ramps of a frame and the frame's rest thresholds: the effect input's for every main SVF
/// and the sidechain's for every detector SVF. An unarmable segment (`ARMABLE = false`) had its
/// clocks advanced before the frame loop (`lane::kernels::silence_skip_block`) and reads none.
/// `GLIDE` (issue #1969) also advances the link weights one frame, once for all the bands.
#[inline(always)]
fn open<
    L: Lane,
    const RAMPING: bool,
    const CONNECTED: bool,
    const ARMABLE: bool,
    const GLIDE: bool,
>(
    s: &mut Chan<L>,
    x: L,
    side: L,
    armed_after: L,
) -> (L, L) {
    if RAMPING {
        advance_word_ramps(&mut s.ramps);
        refresh_knees(s);
    }
    if GLIDE {
        advance_word_ramps(&mut s.links);
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

/// One band's front half on one channel: its bell SVF on the cascade word `u`, and its detector
/// tap, the unity-peak band-pass at quality `Q` of `u` or, when connected, crossfaded by the
/// detector word toward the same filter on the sidechain word. Returns `(v1, detector)`.
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
        // The bell's own band-pass tap is not read: the detector has its own SVF.
        k: L::zero(),
    };
    let detector_words = BandWords {
        c1: r[DC1].current,
        a2: r[DC1 + 1].current,
        a3: r[DC1 + 2].current,
        k: r[DK].current,
    };
    let external = r[EXTERNAL].current;
    let state = &mut s.bands[band];
    let (v1, _, _) = band_step(ARMABLE, u, &words, rest.0, &mut state.main);
    let (_, _, tap) = band_step(ARMABLE, u, &detector_words, rest.0, &mut state.detector);
    let detector = if CONNECTED {
        let (_, _, side_tap) = band_step(ARMABLE, side, &detector_words, rest.1, &mut state.side);
        blend(tap, side_tap, external)
    } else {
        tap
    };
    (v1, detector)
}

/// One band's linked detector level in dB on one channel: the lane's prepared link law, as its
/// masks select, and under `GLIDE` (issue #1969, #1370 D2-D3), on every lane whose link weights
/// (already advanced this frame by `open`) are still in flight, the blend
/// `combined = max + w_avg * (avg - max)`, `d = own + w_link * (combined - own)` of `|own|` and
/// `|partner|`, each a separate `mul` and `add` (base Wasm SIMD has no fused multiply-add). A
/// gliding lane's blend enters the shared level as a dual-mono word, so it takes the same dB
/// conversion; a lane at rest, or arriving on this frame, keeps the prepared law's own operands,
/// which its masks (already the target mode's) select, so its bits are the non-glide body's.
#[inline(always)]
pub(crate) fn detector_level<L: Lane, const GLIDE: bool>(
    c: &Coef<L>,
    links: &[WordRamp<L>; LINK_COUNT],
    own: L,
    partner: L,
) -> L {
    let link = LinkWords {
        maximum: c.link_max,
        average: c.link_avg,
    };
    if !GLIDE {
        return detector_level_db(own, partner, &link);
    }
    let zero = L::zero();
    let gliding = L::mask_or(links[0].remaining.gt(zero), links[1].remaining.gt(zero));
    let (own_abs, partner_abs) = (own.abs(), partner.abs());
    let maximum = own_abs.max(partner_abs);
    let half = L::splat(0.5);
    let average = half.mul(own_abs).add(half.mul(partner_abs));
    let combined = maximum.add(links[1].current.mul(average.sub(maximum)));
    let blend = own_abs.add(links[0].current.mul(combined.sub(own_abs)));
    let link = LinkWords {
        maximum: L::select(gliding, zero, link.maximum),
        average: L::select(gliding, zero, link.average),
    };
    detector_level_db(L::select(gliding, blend, own), partner, &link)
}

/// One band's back half on one channel: the linked level, the gain law, and the modulated bell
/// mix on the cascade word `u`. A zero mix word (an idle 0 dB band, a disabled band) selects `u`
/// itself, so such a band is the identity bit for bit.
#[inline(always)]
fn band_back<L: Lane, const GLIDE: bool>(
    band: usize,
    c: &Coef<L>,
    s: &mut Chan<L>,
    own: L,
    partner: L,
    u: L,
    v1: L,
) -> L {
    let zero = L::zero();
    let level_db = detector_level::<L, GLIDE>(c, &s.links, own, partner);
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
///
/// `GLIDE` (issue #1969) runs a detector link glide in flight ([`detector_level`]): only a
/// `RAMPING` segment that has a glide to advance takes it; every other body is the one before.
#[inline(always)]
pub fn segment<
    L: Lane,
    const CONNECTED: bool,
    const RAMPING: bool,
    const MONO: bool,
    const ARMABLE: bool,
    const GLIDE: bool,
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
            let rest =
                open::<L, RAMPING, false, ARMABLE, GLIDE>(&mut l, x_left, x_left, armed_after);
            let mut u = x_left;
            for (band, _) in live.iter().enumerate().filter(|(_, live)| **live) {
                let (v1, det) = band_front::<L, false, ARMABLE>(band, &mut l, u, u, rest);
                u = band_back::<L, GLIDE>(band, coef_left, &mut l, det, det, u, v1);
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
        let rest_l =
            open::<L, RAMPING, CONNECTED, ARMABLE, GLIDE>(&mut l, x_left, s_left, armed_after);
        let rest_r =
            open::<L, RAMPING, CONNECTED, ARMABLE, GLIDE>(&mut r, x_right, s_right, armed_after);
        let (mut u_left, mut u_right) = (x_left, x_right);
        for (band, _) in live.iter().enumerate().filter(|(_, live)| **live) {
            let (v1_l, det_l) =
                band_front::<L, CONNECTED, ARMABLE>(band, &mut l, u_left, s_left, rest_l);
            let (v1_r, det_r) =
                band_front::<L, CONNECTED, ARMABLE>(band, &mut r, u_right, s_right, rest_r);
            u_left = band_back::<L, GLIDE>(band, coef_left, &mut l, det_l, det_r, u_left, v1_l);
            u_right = band_back::<L, GLIDE>(band, coef_right, &mut r, det_r, det_l, u_right, v1_r);
        }
        L::select(coef_left.bypass.gt(L::zero()), x_left, u_left).store(&mut left[span.clone()]);
        L::select(coef_right.bypass.gt(L::zero()), x_right, u_right).store(&mut right[span]);
    }
    *state_left = l;
    if !MONO {
        *state_right = r;
    }
}
