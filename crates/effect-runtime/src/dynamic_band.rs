//! The dynamic band step shared by the de-esser (#15) and the dynamic EQ (#17).
//!
//! Both effects are one TPT SVF band whose detector drives a gain change in dB:
//!
//! 1. [`band_step`]: one `lane::kernels::svf_step` (the workspace's only copy of the recurrence)
//!    and the unity-peak band-pass tap `b = k v1` (Simper's band output `v1` has peak gain `Q`, so
//!    `k = 1/Q` normalises it to 0 dB at the centre frequency).
//! 2. [`detector_level_db`]: `|d|` of the detector tap, linked across the channels (dual mono,
//!    the maximum or the average of the two magnitudes), floored at `1e-8` and converted with the
//!    fast dB tier, clamped to `[-160, 24]` dB.
//! 3. [`dynamic_band_gain`]: the gain computer of `crate::dynamics` (Giannoulis, Massberg and
//!    Reiss, JAES 2012, equation 4) as a delta, negated for an upward band, clamped to
//!    `[-range, range]`, then smoothed **in the dB domain** by a branching one-pole in the rate
//!    form `g' = flush(g + c (t - g))`: the attack rate while the target is further from 0 dB than
//!    the word (the change is growing), the release rate otherwise. Every word is a lane word, so
//!    a ramped threshold, slope, range or rate is read as it stands this sample.
//!
//! Each effect then applies its own output law to `(x, v1, g)` (the de-esser's split, wideband
//! and listen arms; the dynamic EQ's modulated bell mix). The `f64` oracle is
//! `dsp_reference::dynamic_band`, which shares no code with this module.

use crate::dynamics::{GainComputerCoef, gain_delta_db};
use lane::kernels::{SvfState, svf_step};
use lane::{Lane, flush};
use math::fast_db::{fast_gain_from_db, fast_level_db};

/// Detector level floor, below which the logarithm is not taken.
pub const LEVEL_FLOOR: f32 = 1.0e-8;

/// Lower detector level clamp in dB.
pub const LEVEL_MIN_DB: f32 = -160.0;

/// Upper detector level clamp in dB.
pub const LEVEL_MAX_DB: f32 = 24.0;

/// The four SVF words a band step reads: the pole words `c1, a2, a3` (designed in `f64` and
/// rounded once by the effect) and the damping `k = 1/Q` that scales the band-pass tap.
#[derive(Clone, Copy, Debug)]
pub struct BandWords<L: Lane> {
    /// `t / (1 + t)`, `t = g (g + k)`.
    pub c1: L,
    /// `g (1 - c1)`.
    pub a2: L,
    /// `g a2`.
    pub a3: L,
    /// `1/Q`.
    pub k: L,
}

/// One SVF step of `x`: `(v1, v2, k v1)`, the band and low integrator outputs and the unity-peak
/// band-pass tap. `rest` is the joint-flush threshold of the **effect input's** silence clock
/// (`lane::silence_step`, issue #1328).
#[inline(always)]
pub fn band_step<L: Lane>(x: L, w: &BandWords<L>, rest: L, s: &mut SvfState<L>) -> (L, L, L) {
    let (v1, v2) = svf_step(x, w.c1.neg(), w.a2, w.a3, rest, s);
    (v1, v2, w.k.mul(v1))
}

/// The detector link of one channel: `1.0` words select the mode, both `+0.0` is dual mono.
#[derive(Clone, Copy, Debug)]
pub struct LinkWords<L: Lane> {
    /// `1.0` for the maximum of the two channels' magnitudes.
    pub maximum: L,
    /// `1.0` for their average, `0.5 |own| + 0.5 |partner|`.
    pub average: L,
}

/// The linked detector level in dB of this channel's tap `own` and the other channel's `partner`.
#[inline(always)]
// FAST-DB-CROSSING X9: the dynamic band's detector level, a dynamics reading.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X9: dynamic band detector level, never a pinned coefficient"
)]
pub fn detector_level_db<L: Lane>(own: L, partner: L, link: &LinkWords<L>) -> L {
    let zero = L::zero();
    let (own, partner) = (own.abs(), partner.abs());
    let half = L::splat(0.5);
    let mut level = L::select(link.maximum.gt(zero), own.max(partner), own);
    level = L::select(
        link.average.gt(zero),
        half.mul(own).add(half.mul(partner)),
        level,
    );
    fast_level_db(level.max(L::splat(LEVEL_FLOOR)))
        .min(L::splat(LEVEL_MAX_DB))
        .max(L::splat(LEVEL_MIN_DB))
}

/// The gain law's words for one sample.
#[derive(Clone, Copy)]
pub struct DynamicBandCoef<L: Lane> {
    /// Threshold, slope `1/R - 1` and knee words of the static curve.
    pub curve: GainComputerCoef<L>,
    /// The largest change in dB, `>= 0`.
    pub range_db: L,
    /// `1.0` for an upward band (the change is `-gain_delta_db`, a boost), `+0.0` for downward.
    pub upward: L,
    /// Attack rate coefficient, `1 - exp(-1000 / (t fs))`.
    pub attack: L,
    /// Release rate coefficient.
    pub release: L,
}

/// One step of the smoothed change in dB: the curve's target at `level_db`, clamped to
/// `[-range, range]`, and the branching one-pole from `gain_db`, which it updates and returns.
#[inline(always)]
pub fn dynamic_band_gain<L: Lane>(level_db: L, c: &DynamicBandCoef<L>, gain_db: &mut L) -> L {
    let delta = gain_delta_db(level_db, &c.curve);
    let delta = L::select(c.upward.gt(L::zero()), delta.neg(), delta);
    let target = delta.max(c.range_db.neg()).min(c.range_db);
    let growing = target.abs().gt(gain_db.abs());
    let rate = L::select(growing, c.attack, c.release);
    *gain_db = flush(rate.fma(target.sub(*gain_db), *gain_db));
    *gain_db
}

/// The linear gain of a smoothed change `delta_db`.
#[inline(always)]
// FAST-DB-CROSSING X10: the dynamic band's applied gain, a dynamics result.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X10: dynamic band applied gain, never a pinned coefficient"
)]
pub fn applied_gain<L: Lane>(delta_db: L) -> L {
    fast_gain_from_db(delta_db)
}
