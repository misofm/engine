//! The de-esser's one generic per-sample kernel (issue #15 S1).
//!
//! Per channel and frame: the twelve live words step one sample along their ramps, the shared
//! dynamic band step (`effect_runtime::dynamic_band`, also the dynamic EQ's) filters the main
//! input into its band-pass tap and gives the high-pass tap `x - k v1 - v2` beside it, a
//! connected sidechain runs a second SVF for the detector only, and the band's hard-knee,
//! downward gain law smooths the reduction in dB. This module owns only the de-esser's output
//! law: the wideband, split and listen arms. `Lane` keeps the scalar, four-lane and eight-lane
//! paths on this one body.

use effect_runtime::dynamic_band::WordRamp;
use effect_runtime::dynamic_band::{
    BandWords, DynamicBandCoef, LinkWords, advance_word_ramps, applied_gain, band_step, blend,
    detector_level_db, dynamic_band_gain,
};
use effect_runtime::dynamics::GainComputerCoef;
use lane::kernels::SvfState;
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
    /// The detector link glide weights (issue #1969, #1370 D2), `w_link` then `w_avg`: at rest
    /// each is exactly `effect_contract::link_glide_weights` of the lane's mode, the mode
    /// [`Coef::link_max`] and [`Coef::link_avg`] select. Only a `GLIDE` segment reads them.
    pub links: [WordRamp<L>; LINK_COUNT],
}

impl<L: Lane> Default for Chan<L> {
    fn default() -> Self {
        let zero = L::zero();
        Self {
            main: SvfState::default(),
            side: SvfState::default(),
            run: zero,
            side_run: zero,
            gain_db: zero,
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
    /// Connected sidechain channels, present exactly when `CONNECTED` is true.
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    /// `N_SILENCE` at the prepared rate (`lane::silence_frames`).
    pub armed_after: L,
    /// Left and right prepared words.
    pub coef: (&'a Coef<L>, &'a Coef<L>),
    /// Left and right state.
    pub state: (&'a mut Chan<L>, &'a mut Chan<L>),
}

/// One SVF step on `x` and the selected tap: band `k v1` (0 dB at the centre) or high
/// `x - k v1 - v2`, crossfaded by the detector word. The joint flush is armed by `x`'s own
/// silence clock; an unarmable segment (`ARMABLE = false`) had its clock advanced before the frame
/// loop (`lane::kernels::silence_skip_block`) and reads no threshold.
#[inline(always)]
fn filter<L: Lane, const ARMABLE: bool>(
    x: L,
    s: &mut SvfState<L>,
    run: &mut L,
    words: &[WordRamp<L>; RAMPS],
    armed_after: L,
) -> L {
    let rest = if ARMABLE {
        silence_step(x, run, armed_after)
    } else {
        L::zero()
    };
    let band_words = BandWords {
        c1: words[C1].current,
        a2: words[C1 + 1].current,
        a3: words[C1 + 2].current,
        k: words[K].current,
    };
    let (_, v2, band) = band_step(ARMABLE, x, &band_words, rest, s);
    let high = x.sub(band).sub(v2);
    blend(band, high, words[DETECTOR].current)
}

/// One channel's front half: the ramps, the main tap and the detector tap.
#[inline(always)]
fn front<L: Lane, const RAMPING: bool, const CONNECTED: bool, const ARMABLE: bool>(
    s: &mut Chan<L>,
    x: L,
    side: L,
    armed_after: L,
) -> (L, L) {
    if RAMPING {
        advance_word_ramps(&mut s.ramps);
    }
    let tap = filter::<L, ARMABLE>(x, &mut s.main, &mut s.run, &s.ramps, armed_after);
    let detector = if CONNECTED {
        filter::<L, ARMABLE>(side, &mut s.side, &mut s.side_run, &s.ramps, armed_after)
    } else {
        tap
    };
    (tap, detector)
}

/// One frame's linked detector level in dB of one channel: the lane's prepared link law, as its
/// masks select, and under `GLIDE` (issue #1969, #1370 D2-D3) the link weights advanced one frame
/// and, on every lane still in flight after it, the blend `combined = max + w_avg * (avg - max)`,
/// `d = own + w_link * (combined - own)` of `|own|` and `|partner|`, each a separate `mul` and
/// `add` (base Wasm SIMD has no fused multiply-add). A gliding lane's blend enters the shared
/// level as a dual-mono word, so it takes the same dB conversion; a lane at rest, or arriving on
/// this frame, keeps the prepared law's own operands, which its masks (already the target mode's)
/// select, so its bits are the non-glide body's.
#[inline(always)]
pub(crate) fn detector_level<L: Lane, const GLIDE: bool>(
    c: &Coef<L>,
    links: &mut [WordRamp<L>; LINK_COUNT],
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
    advance_word_ramps(links);
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

/// One channel's back half: the linked level, the downward hard-knee gain law and the output
/// arm. A zero gain word selects the dry word on the wideband and split arms, so the split arm at
/// rest is `x` exactly, not `x + 0 b`.
#[inline(always)]
fn back<L: Lane, const GLIDE: bool>(
    c: &Coef<L>,
    s: &mut Chan<L>,
    own: L,
    partner: L,
    x: L,
    tap: L,
) -> L {
    let zero = L::zero();
    let level_db = detector_level::<L, GLIDE>(c, &mut s.links, own, partner);
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
            let (tap, detector) =
                front::<L, RAMPING, false, ARMABLE>(&mut l, x_left, x_left, armed_after);
            back::<L, GLIDE>(coef_left, &mut l, detector, detector, x_left, tap)
                .store(&mut left[span]);
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
        let (tap_l, det_l) =
            front::<L, RAMPING, CONNECTED, ARMABLE>(&mut l, x_left, s_left, armed_after);
        let (tap_r, det_r) =
            front::<L, RAMPING, CONNECTED, ARMABLE>(&mut r, x_right, s_right, armed_after);
        let out_left = back::<L, GLIDE>(coef_left, &mut l, det_l, det_r, x_left, tap_l);
        let out_right = back::<L, GLIDE>(coef_right, &mut r, det_r, det_l, x_right, tap_r);
        out_left.store(&mut left[span.clone()]);
        out_right.store(&mut right[span]);
    }
    *state_left = l;
    if !MONO {
        *state_right = r;
    }
}

#[cfg(test)]
mod glide_law_tests {
    //! Issue #1969 (the #1370 gate 1 shape), the law's exact half: from the frame a glide
    //! arrives on, the lane's detector level is its target mode's prepared law bit for bit, and
    //! a lane that rests beside a glide takes its own mode's level on every frame, non-finite
    //! partner words included. The blend's values against the `f64` reference are
    //! `glide_tests::a_glide_tracks_the_f64_blend_into_the_target_law`.

    use super::{Coef, LINK_COUNT, MAX_WIDTH, WordRamp, detector_level};
    use effect_contract::{LinkMode, link_glide_weights};
    use effect_runtime::ramp::LinearRamp;
    use lane::Lane;

    const MODES: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];
    const SAMPLES: u32 = 64;
    const FRAMES: u32 = 96;

    fn mask_words(modes: &[LinkMode; MAX_WIDTH], mode: LinkMode) -> [f32; MAX_WIDTH] {
        core::array::from_fn(|lane| f32::from(u8::from(modes[lane] == mode)))
    }

    fn check<L: Lane>() {
        for from in MODES {
            for to in MODES.into_iter().filter(|mode| *mode != from) {
                let mut modes: [LinkMode; MAX_WIDTH] =
                    core::array::from_fn(|lane| MODES[lane % MODES.len()]);
                modes[0] = from;
                // Each lane at rest at its mode, then lane 0 retargeted: its masks take the
                // target at once (`DeEsser::retarget_link`), its weights glide.
                let mut ramps: [[LinearRamp; MAX_WIDTH]; LINK_COUNT] = core::array::from_fn(|w| {
                    core::array::from_fn(|lane| {
                        LinearRamp::fixed(link_glide_weights(modes[lane])[w])
                    })
                });
                modes[0] = to;
                for (w, ramp) in ramps.iter_mut().enumerate() {
                    ramp[0].set_target(link_glide_weights(to)[w], SAMPLES);
                }
                let load = |field: fn(&LinearRamp) -> f32, w: usize| {
                    L::load(&core::array::from_fn::<f32, MAX_WIDTH, _>(|lane| {
                        field(&ramps[w][lane])
                    }))
                };
                let mut links: [WordRamp<L>; LINK_COUNT] = core::array::from_fn(|w| WordRamp {
                    current: load(|ramp| ramp.current, w),
                    target: load(|ramp| ramp.target, w),
                    step: load(|ramp| ramp.step, w),
                    remaining: load(|ramp| ramp.remaining as f32, w),
                });
                let coef = Coef {
                    bypass: L::zero(),
                    link_max: L::load(&mask_words(&modes, LinkMode::Maximum)),
                    link_avg: L::load(&mask_words(&modes, LinkMode::Average)),
                };
                for frame in 1..=FRAMES {
                    let words = |scale: f32, salt: u32| -> [f32; MAX_WIDTH] {
                        core::array::from_fn(|lane| {
                            let hash = (frame.wrapping_mul(2_654_435_761)
                                ^ (lane as u32 + salt).wrapping_mul(40_503))
                                % 2_001;
                            scale * (hash as f32 / 1_000.0 - 1.0)
                        })
                    };
                    let mut partner = words(0.09, 7);
                    // Hostile partner words on every third frame, the arrival frame included: a
                    // blend at rest weights equals the prepared law on finite words, but
                    // `0 * inf` and `inf - inf` are NaN, so only the `select` keeps a dual-mono
                    // lane clear of its partner's non-finite word.
                    if frame % 3 == SAMPLES % 3 {
                        for (lane, word) in partner.iter_mut().enumerate() {
                            *word = if (frame as usize + lane).is_multiple_of(2) {
                                f32::INFINITY
                            } else {
                                f32::NAN
                            };
                        }
                    }
                    let (own, partner) = (L::load(&words(0.9, 1)), L::load(&partner));
                    let mut rested = links;
                    let law = detector_level::<L, false>(&coef, &mut rested, own, partner);
                    let got = detector_level::<L, true>(&coef, &mut links, own, partner);
                    let bits = |value: L| {
                        let mut out = [0_u32; MAX_WIDTH];
                        value.store_bits(&mut out[..L::WIDTH]);
                        out
                    };
                    let (got, law) = (bits(got), bits(law));
                    for lane in 0..L::WIDTH {
                        if lane == 0 && frame < SAMPLES {
                            continue;
                        }
                        assert_eq!(
                            got[lane],
                            law[lane],
                            "W{} {from:?} -> {to:?} frame {frame} lane {lane}: not the prepared \
                             law's level",
                            L::WIDTH
                        );
                    }
                }
                let mut remaining = [0.0_f32; MAX_WIDTH];
                for link in &links {
                    link.remaining.store(&mut remaining[..L::WIDTH]);
                    assert!(
                        remaining.iter().all(|word| *word == 0.0),
                        "the glide arrived"
                    );
                }
            }
        }
    }

    #[test]
    fn a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own() {
        lane::each_lane!(|L| check::<L>());
    }
}
