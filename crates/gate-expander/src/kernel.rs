//! The whole per-sample graph of the gate/expander, once, generic over [`Lane`].
//!
//! One body — detector tap, link, `log2` level, hysteresis, gain computer, one-pole, `exp2`,
//! identity select — instantiated at `f32` (`WIDTH = 1`, the scalar oracle), [`Simd4`] and
//! [`Simd8`]. There is no separate scalar path: master plan #83 §4.1 makes lane identity a
//! property of the code, so the eight-lane bank and eight scalar instances agree by construction
//! rather than by a fixture.
//!
//! [`Simd4`]: lane::Simd4
//! [`Simd8`]: lane::Simd8
//!
//! # What is *not* here
//!
//! No `if` whose condition depends on sample data (D10), no `f32::max`/`min` (D8: the select forms
//! of [`Lane`] only), no platform transcendental (D6: `math`), no `%` or `/` on a ring
//! index (the slot count is a power of two and the index is a masked wrapping `u32`), and no
//! per-value finiteness check (D7: [`flush`] on the one recursive word, and one boundary check per
//! block, applied by the caller through `effect_runtime::bank`).
//!
//! # Why the hysteresis is written here and not taken from the runtime
//!
//! `effect_runtime::envelope::hysteresis_step` is the same shape — open and hold as
//! lane words, a countdown clamped at zero and tested before it is decremented — and this module
//! uses its [`HysteresisState`] rather than a second state type. Its *transition* differs from
//! brief 014 in two pinned respects, and both are audible: it opens on `level > open_db` where the
//! gate opens at `level >= threshold`, and it reloads the hold only on an opening trigger where
//! the gate also reloads it on every sample inside the hysteresis band. Adopting it would move
//! rendered bits and would break the hold semantics issue #48 froze, so the gate keeps its own
//! transition and shares the representation.

use effect_runtime::envelope::HysteresisState;
use lane::{Lane, flush};
use math::fast_db::{fast_gain_from_db, fast_level_db};

/// Smoothed parameters: threshold, ratio, range, hysteresis, in descriptor ID order.
pub const RAMP_COUNT: usize = 4;

/// The widest backend, and so the length of every per-lane scratch array.
pub const MAX_WIDTH: usize = 8;

/// `20 * log10(2)`, rounded once: the factor that turns a `log2` into decibels.
///
/// The exact value is `6.020_599_913_279_624`; the nearest `f32` is `6.020_599_842_071_533`, which
/// this literal names with the fewest digits that select it.
pub const DB_PER_OCTAVE: f32 = 6.020_6;

/// `log2(10) / 20`, rounded once: the factor that turns decibels into an `exp2` argument.
///
/// Exact value `0.166_096_404_744_368_1`, nearest `f32` `0.166_096_404_194_831_85`.
pub const OCTAVES_PER_DB: f32 = 0.166_096_4;

/// Detector level floor, below which the logarithm is not taken. `-160 dB` is the clamped result.
pub const LEVEL_FLOOR: f32 = 1.0e-8;

/// Lower clamp of the detector level in dB.
pub const LEVEL_MIN_DB: f32 = -160.0;

/// Upper clamp of the detector level in dB.
pub const LEVEL_MAX_DB: f32 = 24.0;

/// One smoothed parameter of every lane, in the D11 precomputed-step form.
///
/// The law is `effect_runtime::ramp::LinearRamp`'s, transposed into lane words: the
/// division happens once, on the control plane, when a target is set; the render path only adds,
/// and assigns the target exactly on the final sample.
#[derive(Clone, Copy, Debug)]
pub struct GateRamp<L: Lane> {
    /// Value this sample uses.
    pub current: L,
    /// Value the ramp is heading for, assigned exactly on the final sample.
    pub target: L,
    /// Per-sample increment, precomputed at event time.
    pub step: L,
    /// Samples still to be produced, an integer-valued float in `[0, 64]`.
    pub remaining: L,
}

impl<L: Lane> GateRamp<L> {
    /// A ramp resting at `value` on every lane.
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

/// Everything one channel of one bank holds that a parameter change can move.
#[derive(Clone, Copy, Debug)]
pub struct GateCoef<L: Lane> {
    /// `1 - exp(-1 / (attack * fs))`, the rate coefficient of the attack one-pole.
    pub attack: L,
    /// `1 - exp(-1 / (release * fs))`, the rate coefficient of the release one-pole.
    pub release: L,
    /// Hold length in samples, an integer-valued float (at most 96 000, so exact in `f32`).
    pub hold_samples: L,
    /// `1.0` on a bypassed lane, `+0.0` otherwise.
    pub bypass: L,
    /// `1.0` when the detector takes the larger of the two channels.
    pub link_max: L,
    /// `1.0` when the detector takes their mean.
    pub link_avg: L,
}

/// Everything one channel of one bank carries across a block boundary.
#[derive(Clone, Copy)]
pub struct GateState<L: Lane> {
    /// `G`, the smoothed gain reduction in dB. The only recursive word, and so the only one
    /// [`flush`] is applied to (D7).
    pub gain_db: L,
    /// Open flag and hold countdown, in the runtime's lane-word representation.
    pub hysteresis: HysteresisState<L>,
    /// The four smoothed parameters.
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

/// One channel's delay storage, shared by every lane of a bank.
///
/// `main` and `detector` are AoSoA: slot `s` of lane `l` is at `s * WIDTH + l`. The slot count is
/// a power of two at least `latency + 1`, so a ring index is a wrapping `u32` and a mask — never a
/// `%`. `detector` is empty unless a sidechain is connected, which is the whole of finding F4: in
/// the only bankable configuration the detector ring was a byte-for-byte duplicate of the main
/// one.
pub struct GateRing<'a> {
    /// The delayed dry signal.
    pub main: &'a mut [f32],
    /// The sidechain signal, or an empty slice when no sidechain is connected.
    pub detector: &'a mut [f32],
    /// Per-lane detector tap in samples, `latency - lookahead`. Only `[..WIDTH]` is read.
    pub tap: &'a [u32],
}

/// Everything one call of [`gate_block`] borrows.
pub struct GateArgs<'a, L: Lane> {
    /// Left channel block, `frames * WIDTH` samples, read and written in place.
    pub left: &'a mut [f32],
    /// Right channel block, the same length.
    pub right: &'a mut [f32],
    /// Sidechain blocks, present exactly when `CONNECTED`.
    pub sidechain: Option<(&'a [f32], &'a [f32])>,
    /// Frames in the block.
    pub frames: usize,
    /// Left and right coefficients.
    pub coef: (&'a GateCoef<L>, &'a GateCoef<L>),
    /// Left and right state.
    pub state: (&'a mut GateState<L>, &'a mut GateState<L>),
    /// Left and right rings.
    pub rings: (GateRing<'a>, GateRing<'a>),
    /// Shared write cursor, advanced by `frames`.
    pub cursor: &'a mut u32,
    /// `slots - 1`, the ring index mask.
    pub slot_mask: u32,
    /// The fixed latency `N` in samples.
    pub delay: u32,
}

/// Runs one block of the gate through one lane width.
///
/// `CONNECTED` selects the sidechain detector, `RAMPING` the parameter-smoothing prologue; both
/// are block-constant, so they are const generics rather than per-sample branches. `RAMPING =
/// false` is not an approximation: it omits updates that are exactly no-ops while every lane's
/// `remaining` is zero, which the caller guarantees by splitting a block at the point the last
/// ramp ends.
///
/// Frozen operation order, per frame — moving any line moves bits:
///
/// ```text
/// write   x_l, x_r into ring slot (cursor & mask); the sidechain too when CONNECTED
/// dry     z_c = ring_c.main[(cursor - N) & mask]
/// gather  s_c = src_c[(cursor - tap_c[lane]) & mask], s_p = src_p[(cursor - tap_c[lane]) & mask]
/// ramps   current += step, or = target on the final sample     (RAMPING only)
/// link    u = |s_c| | max(|s_c|, |s_p|) | 0.5|s_c| + 0.5|s_p|
/// level   X = clamp(log2(max(u, 1e-8)) * 20log10(2), -160, 24)
/// gate    open' / hold' from the brief 014 transition
/// curve   C = select(open', 0, clamp((rho - 1) * (X - T), -R, 0))
/// pole    b = select(C > G, attack, release);  G' = flush(fma(b, C - G, G))     unfused (#163)
/// gain    A = exp2(G' * log2(10)/20)
/// out     y = select(G' == 0 or bypass, z, z * A)
/// ```
///
/// # Why the dry path keeps a signed zero under D7
///
/// `y = select(identity, z, z * A)` and `z` is a *ring word* — the delayed input — which is never
/// flushed, so `-0.0` survives the ring and the select bit for bit. [`flush`] touches only `G`.
/// Its only effect on the select is through the mask `G' == 0`, and `±0 == 0` holds for both
/// signs, so a flushed `+0.0` and an unflushed `-0.0` or tiny `G` make the same decision. Inside
/// the widened identity set (`|G'| < 1e-20`) the wet product would have used an `A` that already
/// rounds to exactly `1.0`, and `z * 1.0 == z` bit for bit, so the flush changes no PCM at all —
/// only the canonical sign of the stored state word.
///
/// # Why the caller must still check `G`
///
/// `exp2_lane` clamps its argument with the D8 `max`/`min`, which *swallow* NaN
/// (`select(NaN > -126, NaN, -126)` is `-126`), so a NaN `G` yields a finite `A` and finite
/// outputs. Scanning the output block alone would never see it. The caller therefore runs the
/// §4.4 boundary check over the gain words as well as over the block.
///
/// # Panics
///
/// Panics in debug builds if the blocks are not `frames * WIDTH` long, or if `CONNECTED` is set
/// without a sidechain.
#[inline(always)]
pub fn gate_block<L: Lane, const CONNECTED: bool, const RAMPING: bool>(args: GateArgs<'_, L>) {
    let GateArgs {
        left,
        right,
        sidechain,
        frames,
        coef,
        state,
        rings,
        cursor,
        slot_mask,
        delay,
    } = args;
    let width = L::WIDTH;
    debug_assert_eq!(left.len(), frames * width);
    debug_assert_eq!(right.len(), frames * width);
    debug_assert_eq!(CONNECTED, sidechain.is_some());
    let (coef_left, coef_right) = coef;
    let (state_left, state_right) = state;
    let (ring_left, ring_right) = rings;
    let empty: &[f32] = &[];
    let (side_left, side_right) = sidechain.unwrap_or((empty, empty));
    let base = *cursor;
    // Hoisted: the gather scratch is written and read within one frame, but zeroing 128 bytes per
    // frame would cost more than the gather itself.
    let mut taps = [[0.0_f32; MAX_WIDTH]; 4];

    for frame in 0..frames {
        let span = frame * width;
        let now = base.wrapping_add(frame as u32);
        let write = ((now & slot_mask) as usize) * width;
        L::load(&left[span..span + width]).store(&mut ring_left.main[write..write + width]);
        L::load(&right[span..span + width]).store(&mut ring_right.main[write..write + width]);
        if CONNECTED {
            L::load(&side_left[span..span + width])
                .store(&mut ring_left.detector[write..write + width]);
            L::load(&side_right[span..span + width])
                .store(&mut ring_right.detector[write..write + width]);
        }
        let read = ((now.wrapping_sub(delay) & slot_mask) as usize) * width;
        let dry_left = L::load(&ring_left.main[read..read + width]);
        let dry_right = L::load(&ring_right.main[read..read + width]);

        // Per-lane lookahead is a `PerLane` parameter, so the detector tap differs per lane and
        // the read is a gather: `WIDTH` scalar loads per source, no arithmetic, so the rounding
        // sequence each lane sees is still the scalar one.
        let (source_left, source_right) = if CONNECTED {
            (&*ring_left.detector, &*ring_right.detector)
        } else {
            (&*ring_left.main, &*ring_right.main)
        };
        // Four disjoint destination arrays written at the same index, from two tap arrays read at
        // that index: an iterator chain over all six would hide what the index means.
        #[allow(clippy::needless_range_loop)]
        for lane in 0..width {
            let own = ((now.wrapping_sub(ring_left.tap[lane]) & slot_mask) as usize) * width + lane;
            let partner =
                ((now.wrapping_sub(ring_right.tap[lane]) & slot_mask) as usize) * width + lane;
            taps[0][lane] = source_left[own];
            taps[1][lane] = source_right[own];
            taps[2][lane] = source_right[partner];
            taps[3][lane] = source_left[partner];
        }

        let out_left = channel_step::<L, RAMPING>(
            coef_left,
            state_left,
            L::load(&taps[0][..width]),
            L::load(&taps[1][..width]),
            dry_left,
        );
        let out_right = channel_step::<L, RAMPING>(
            coef_right,
            state_right,
            L::load(&taps[2][..width]),
            L::load(&taps[3][..width]),
            dry_right,
        );
        out_left.store(&mut left[span..span + width]);
        out_right.store(&mut right[span..span + width]);
    }
    *cursor = base.wrapping_add(frames as u32);
}

/// One sample of one channel: the detector, the transition, the curve, the one-pole and the
/// identity select. Both channels run this body, so neither can drift from the other.
#[inline(always)]
// FAST-DB-CROSSING X3 and X4's `#[expect]`: the crossing registry entries this replaces lived in
// `scripts/check-fast-db-seal.sh`, retired in favour of `clippy.toml`'s `disallowed-methods`.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X3/X4: gate detector level and applied gain, never pinned coefficients"
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
            // `LinearRamp::next_value`, per lane: at rest hold `current`, on the final sample
            // assign `target` exactly, otherwise add the precomputed step.
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
    // FAST-DB-CROSSING X3: the gate's detector level. A dynamics gain path; the reading feeds
    // the hysteresis comparison and the expansion curve and is never pinned as a coefficient.
    // The `DB_PER_OCTAVE` multiply moves inside the sealed tier, which spells the same constant
    // (`0x40c0_a8c1`); the clamp order stays here, and stays `min` then `max`.
    let level_db = fast_level_db(level.max(L::splat(LEVEL_FLOOR)))
        .min(L::splat(LEVEL_MAX_DB))
        .max(L::splat(LEVEL_MIN_DB));

    // Brief 014's transition, branchless. A closed lane opens at or above the threshold and takes
    // a full hold; an open lane re-arms at or above `T - H`, which reloads the hold; otherwise it
    // spends one sample of the countdown, and closes only once the countdown is already zero.
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
    let open = L::select(L::mask_or(reload, holding), one, zero);
    state.hysteresis.hold = L::select(
        reload,
        coef.hold_samples,
        L::select(
            holding,
            state.hysteresis.hold.sub(one),
            state.hysteresis.hold,
        ),
    );
    state.hysteresis.open = open;

    // `clamp((rho - 1) * (X - T), -R, 0)` in the D8 select form. `R = 0` yields `+0.0` from the
    // `min`, which is what makes a zero range an exact identity rather than a signed zero.
    let curve = ratio
        .sub(one)
        .mul(level_db.sub(threshold))
        .max(range.neg())
        .min(zero);
    let target = L::select(open.gt(zero), zero, curve);
    let rate = L::select(target.gt(state.gain_db), coef.attack, coef.release);
    let gain_db = flush(rate.fma(target.sub(state.gain_db), state.gain_db));
    state.gain_db = gain_db;
    // FAST-DB-CROSSING X4: the gate's applied gain. The `OCTAVES_PER_DB` multiply moves inside
    // the sealed tier, which spells the same constant (`0x3e2a_152d`).
    let gain = fast_gain_from_db(gain_db);
    let identity = L::mask_or(gain_db.eq(zero), coef.bypass.gt(zero));
    L::select(identity, dry, dry.mul(gain))
}
