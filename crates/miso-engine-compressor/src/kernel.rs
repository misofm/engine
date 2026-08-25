//! The generic block kernel: one body, instantiated at `f32`, `Simd4` and `Simd8`.
//!
//! Master plan #83 D10. `L = f32` *is* the scalar path — there is no second implementation, which
//! is what makes cross-backend bit identity a property of the code rather than a test result
//! (gate E2). Every arithmetic operation is a [`Lane`] method, a `miso-engine-math` lane function
//! or an integer compare-select, so the same bits come out of every target (D5, gate E4).
//!
//! # What is not here any more
//!
//! * No libm. `log10`/`powf`/`exp` became `dynamics::level_db` / `dynamics::gain_from_db` on the
//!   lane `exp2`/`log2` of `miso-engine-math`, and the ballistic coefficients are designed off the
//!   render path in `crate::design` (D6).
//! * No `%`. The ring wrap is `if next == B { 0 }`, and the detector tap is
//!   `if w >= D { w - D } else { w + B - D }` — integer compare-select (F2).
//! * No per-value finiteness check, no `sanitize`, no `recover`. `flush` is applied to `g`, the
//!   one recursive word, and the block boundary is checked once by
//!   `miso_engine_effect_runtime::bank` (D7, master plan §4.4).
//! * No `PreparedCompressorGainMixKernelV1`. The gain/mix step is the `gain_mix_block` form
//!   `mix.fma(wet - dry, dry)` written inline, because the compressor's `wet` comes from its own
//!   delayed ring rather than from the block being multiplied in place (D10).
//! * No data-dependent branch per sample: the five-way gain computer is
//!   `dynamics::gain_delta_db`, branchless, and the identity paths are lane selects.
//!
//! # Ramping and idle bodies
//!
//! `Linear 64` smoothing means a parameter event is over in 64 samples. The block is therefore cut
//! into a ramping prefix and an idle remainder, and the idle body never touches a ramp, never
//! redesigns a coefficient and loads its lane vectors once. `frames_loop` is generic over a `const
//! RAMPING: bool` so the two bodies are the same source and the idle one is free of the ramp work
//! at compile time.

use miso_engine_effect_contract::LinkMode;
use miso_engine_effect_runtime::bank;
use miso_engine_effect_runtime::dynamics::{GainComputerCoef, gain_delta_db};
use miso_engine_effect_runtime::envelope::rms_follow;
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_lane::kernels::gain_mix_step;
use miso_engine_lane::{Lane, flush};
use miso_engine_math::fast_db::{fast_gain_from_db, fast_level_db};

use crate::design::{
    ALL_PARAMETERS, COEF_ATTACK, COEF_HALF_KNEE, COEF_INV_RATIO_MINUS_ONE, COEF_INV_TWO_KNEE,
    COEF_MAKEUP, COEF_MIX, COEF_RELEASE, COEF_THRESHOLD, CoefWords, MAX_WIDTH, PARAMETER_COUNT,
    RAMP_COUNT, design_lane, detector_delay,
};

/// Detector level floor, below which the level is treated as `1e-8` (-160 dB).
const LEVEL_FLOOR: f32 = 1.0e-8;
/// Lowest level in dB the static curve sees.
const LEVEL_MIN_DB: f32 = -160.0;
/// Highest level in dB the static curve sees.
const LEVEL_MAX_DB: f32 = 24.0;
/// Most gain reduction the smoother may be asked to track, in dB.
const GAIN_REDUCTION_MIN_DB: f32 = -100.0;

/// Where a lane's detector signal comes from this block.
///
/// One loop-invariant choice per block, not a per-sample branch: which of the three it is follows
/// from the prepared port configuration and the caller's buffers, both of which are fixed for the
/// whole block.
#[derive(Clone, Copy)]
pub(crate) enum Detector<'a> {
    /// Unconnected sidechain: the main input is its own detector.
    Main,
    /// A connected sidechain port with no buffer supplied — silence, as before the audit.
    Silent,
    /// A connected sidechain port with its planar buffers.
    Sidechain(&'a [f32], &'a [f32]),
}

/// One channel of a prepared instance or bank: `L::WIDTH` tracks processed as one vector.
///
/// Two allocations, both in [`Channel::new`]: the main ring and the detector ring, each
/// `ring_length * L::WIDTH` samples, row major so that ring index `r` of every lane is one
/// contiguous `Lane::load`. The pre-audit layout was one pair of rings per lane — `2 * W`
/// allocations and a scalar gather for every access.
pub(crate) struct Channel<L: Lane> {
    /// Preparation-time values, per lane, for `ResetKind::FullToDefaults`.
    pub(crate) defaults: [[f32; PARAMETER_COUNT]; MAX_WIDTH],
    /// The lane words the render path reads. Source of truth; `Coef` is a load of these.
    pub(crate) words: CoefWords,
    /// One ramp per smoothed parameter per lane.
    pub(crate) ramps: [[LinearRamp; MAX_WIDTH]; RAMP_COUNT],
    /// Lookahead in ms per lane, as it was last derived from.
    pub(crate) lookahead_ms: [f32; MAX_WIDTH],
    /// Detector read-back distance `D` per lane.
    pub(crate) delay: [u32; MAX_WIDTH],
    /// Gain reduction in dB: the one recursive word, and the only thing `flush` is applied to.
    pub(crate) gain_reduction_db: L,
    /// Shared write index. Every lane has the same `B` and the same reset, so one cursor serves.
    pub(crate) cursor: u32,
    /// `B = N + 1`, the ring length in frames.
    pub(crate) ring_length: u32,
    /// Main-signal ring, `ring_length * L::WIDTH`, row major.
    pub(crate) main: Box<[f32]>,
    /// Detector ring, same shape.
    pub(crate) detector: Box<[f32]>,
}

impl<L: Lane> Channel<L> {
    /// Allocates a channel and designs every lane from its preparation values.
    pub(crate) fn new(
        defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
        ring_length: usize,
        sample_rate: u32,
    ) -> Self {
        let mut channel = Self {
            defaults: *defaults,
            words: [[0.0; MAX_WIDTH]; crate::design::COEF_COUNT],
            ramps: [[LinearRamp::fixed(0.0); MAX_WIDTH]; RAMP_COUNT],
            lookahead_ms: [0.0; MAX_WIDTH],
            delay: [0; MAX_WIDTH],
            gain_reduction_db: L::zero(),
            cursor: 0,
            ring_length: ring_length as u32,
            main: vec![0.0; ring_length * L::WIDTH].into_boxed_slice(),
            detector: vec![0.0; ring_length * L::WIDTH].into_boxed_slice(),
        };
        channel.seed_from_defaults(sample_rate);
        channel
    }

    /// Points every ramp at its preparation value and redesigns, including the lookahead taps.
    fn seed_from_defaults(&mut self, sample_rate: u32) {
        let ring_length = self.ring_length as usize;
        for lane in 0..MAX_WIDTH {
            let values = self.defaults[lane];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                ramp[lane] = LinearRamp::fixed(values[parameter]);
            }
            self.lookahead_ms[lane] = values[7];
            self.delay[lane] = detector_delay(values[7], sample_rate, ring_length);
            let smoothed: [f32; RAMP_COUNT] = core::array::from_fn(|index| values[index]);
            design_lane(
                &smoothed,
                sample_rate,
                ALL_PARAMETERS,
                &mut self.words,
                lane,
            );
        }
    }

    /// `ResetKind::FullToDefaults`: state cleared, parameters back to their preparation values,
    /// lookahead taps re-derived.
    pub(crate) fn full_reset(&mut self, sample_rate: u32) {
        self.clear_state();
        self.seed_from_defaults(sample_rate);
    }

    /// `ResetKind::DiscontinuityKeepParameters`: state cleared, ramps snapped to their targets,
    /// lookahead kept, coefficients redesigned from the snapped values.
    pub(crate) fn discontinuity_reset(&mut self, sample_rate: u32) {
        self.clear_state();
        for lane in 0..MAX_WIDTH {
            let mut values = [0.0; RAMP_COUNT];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                ramp[lane].snap();
                values[parameter] = ramp[lane].current;
            }
            design_lane(&values, sample_rate, ALL_PARAMETERS, &mut self.words, lane);
        }
    }

    /// Zeroes the rings, the cursor and the recursive word, and leaves parameters alone.
    ///
    /// This is the reset the master plan §4.4 boundary check runs on a rejected block.
    pub(crate) fn clear_state(&mut self) {
        self.cursor = 0;
        self.gain_reduction_db = L::zero();
        self.main.fill(0.0);
        self.detector.fill(0.0);
    }

    /// Current smoothed values of one lane, in table order.
    pub(crate) fn current_values(&self, lane: usize) -> [f32; RAMP_COUNT] {
        core::array::from_fn(|parameter| self.ramps[parameter][lane].current)
    }

    /// Redesigns one lane from its current ramp values; used after a restore.
    pub(crate) fn redesign(&mut self, lane: usize, sample_rate: u32) {
        let values = self.current_values(lane);
        design_lane(&values, sample_rate, ALL_PARAMETERS, &mut self.words, lane);
    }

    /// Highest number of samples any ramp of this channel still has to produce.
    fn max_remaining(&self) -> u32 {
        let mut most = 0;
        for parameter in &self.ramps {
            for lane in parameter.iter().take(L::WIDTH) {
                if lane.remaining > most {
                    most = lane.remaining;
                }
            }
        }
        most
    }

    /// Advances every ramp of every lane by one sample and redesigns what moved.
    ///
    /// Only the coefficients whose parameter actually moved are recomputed, so a threshold ramp
    /// never re-enters the exponential. A lane with no ramp in flight is skipped entirely.
    fn advance_ramps(&mut self, sample_rate: u32) {
        for lane in 0..L::WIDTH {
            let mut changed = 0_u8;
            let mut values = [0.0_f32; RAMP_COUNT];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                if ramp[lane].is_ramping() {
                    changed |= 1 << parameter;
                }
                values[parameter] = ramp[lane].next_value();
            }
            if changed != 0 {
                design_lane(&values, sample_rate, changed, &mut self.words, lane);
            }
        }
    }
}

/// The lane vectors one frame of the kernel needs, loaded from [`Channel::words`].
struct Coef<L: Lane> {
    /// Static-curve coefficients, in the runtime's frozen form.
    curve: GainComputerCoef<L>,
    /// Attack rate coefficient.
    attack: L,
    /// Release rate coefficient.
    release: L,
    /// Makeup gain in dB.
    makeup: L,
    /// Dry/wet mix.
    mix: L,
}

impl<L: Lane> Coef<L> {
    #[inline(always)]
    fn load(words: &CoefWords) -> Self {
        Self {
            curve: GainComputerCoef {
                threshold_db: L::load(&words[COEF_THRESHOLD]),
                inv_ratio_minus_one: L::load(&words[COEF_INV_RATIO_MINUS_ONE]),
                half_knee_db: L::load(&words[COEF_HALF_KNEE]),
                inv_two_knee: L::load(&words[COEF_INV_TWO_KNEE]),
            },
            attack: L::load(&words[COEF_ATTACK]),
            release: L::load(&words[COEF_RELEASE]),
            makeup: L::load(&words[COEF_MAKEUP]),
            mix: L::load(&words[COEF_MIX]),
        }
    }
}

/// Renders `frames` frames of both channels in place.
///
/// `left` and `right` are `frames * L::WIDTH` samples, frame major (AoSoA), which is the layout
/// the bank contract already uses and which the scalar instantiation degenerates to at `W = 1`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    detector: Detector<'_>,
    frames: usize,
    link: LinkMode,
    bypass: bool,
    sample_rate: u32,
    channels: (&mut Channel<L>, &mut Channel<L>),
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let (channel_left, channel_right) = channels;
    let remaining = channel_left
        .max_remaining()
        .max(channel_right.max_remaining()) as usize;
    let ramping = if remaining < frames {
        remaining
    } else {
        frames
    };
    if ramping > 0 {
        frames_loop::<L, true>(
            left,
            right,
            detector,
            0,
            ramping,
            link,
            bypass,
            sample_rate,
            channel_left,
            channel_right,
        );
    }
    if ramping < frames {
        frames_loop::<L, false>(
            left,
            right,
            detector,
            ramping,
            frames,
            link,
            bypass,
            sample_rate,
            channel_left,
            channel_right,
        );
    }
}

/// The frame body. `RAMPING` is a compile-time switch, not a branch.
///
/// Frozen operation order, per frame — every line is one rounding site and moving one moves bits:
///
/// 1. load main, choose the detector source, take magnitudes;
/// 2. link: `mx = max(|l|, |r|)`, `avg = 0.5*|l| + 0.5*|r|` (two products and an add, **not** an
///    `fma`: the product order is a frozen product rule), selected per lane;
/// 3. ring: write at `w`, read the main output at `w + 1` wrapped, gather the detector at
///    `w - D` wrapped — integer compare-select on both wraps;
/// 4. level: `u0 = max(u, 1e-8)`, `x = level_db(u0)` clamped to `[-160, 24]`;
/// 5. curve: `c = gain_delta_db(x)` clamped to `[-100, 0]`;
/// 6. ballistic: `g = flush(branching_one_pole(c, g, c_attack, c_release))`;
/// 7. gain: `a = gain_from_db(g + makeup)`;
/// 8. mix: `wet = z * a`, `y = fma(mix, wet - z, z)`, then the two identity selects;
/// 9. advance the cursor.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn frames_loop<L: Lane, const RAMPING: bool>(
    left: &mut [f32],
    right: &mut [f32],
    detector: Detector<'_>,
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    sample_rate: u32,
    channel_left: &mut Channel<L>,
    channel_right: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let ring_length = channel_left.ring_length as usize;
    let zero = L::zero();
    let one = L::splat(1.0);
    let half = L::splat(0.5);
    let level_floor = L::splat(LEVEL_FLOOR);
    let level_min = L::splat(LEVEL_MIN_DB);
    let level_max = L::splat(LEVEL_MAX_DB);
    let reduction_min = L::splat(GAIN_REDUCTION_MIN_DB);
    let all = zero.eq(zero);
    let none = L::mask_not(all);
    let linked = if matches!(link, LinkMode::DualMono) {
        none
    } else {
        all
    };
    let averaged = if matches!(link, LinkMode::Average) {
        all
    } else {
        none
    };
    let bypassed = if bypass { all } else { none };

    let mut coef_left = Coef::load(&channel_left.words);
    let mut coef_right = Coef::load(&channel_right.words);
    let mut gather = [0.0_f32; MAX_WIDTH];

    for frame in start..end {
        if RAMPING {
            channel_left.advance_ramps(sample_rate);
            channel_right.advance_ramps(sample_rate);
            coef_left = Coef::load(&channel_left.words);
            coef_right = Coef::load(&channel_right.words);
        }
        let slot = frame * width;

        // 1. main input and detector source.
        let main_left = L::load(&left[slot..]);
        let main_right = L::load(&right[slot..]);
        let (source_left, source_right) = match detector {
            Detector::Main => (main_left, main_right),
            Detector::Silent => (zero, zero),
            Detector::Sidechain(sidechain_left, sidechain_right) => (
                L::load(&sidechain_left[slot..]),
                L::load(&sidechain_right[slot..]),
            ),
        };

        // 2. link.
        let magnitude_left = source_left.abs();
        let magnitude_right = source_right.abs();
        let maximum = magnitude_left.max(magnitude_right);
        let average = magnitude_left.mul(half).add(magnitude_right.mul(half));
        let combined = L::select(averaged, average, maximum);
        let level_left = L::select(linked, combined, magnitude_left);
        let level_right = L::select(linked, combined, magnitude_right);

        // 3. rings.
        let write = channel_left.cursor as usize;
        let next = write + 1;
        let next = if next == ring_length { 0 } else { next };
        main_left.store(&mut channel_left.main[write * width..]);
        level_left.store(&mut channel_left.detector[write * width..]);
        main_right.store(&mut channel_right.main[write * width..]);
        level_right.store(&mut channel_right.detector[write * width..]);
        let delayed_left = L::load(&channel_left.main[next * width..]);
        let delayed_right = L::load(&channel_right.main[next * width..]);
        let detected_left = gather_detector(channel_left, write, &mut gather);
        let detected_right = gather_detector(channel_right, write, &mut gather);

        // 4-8, one channel then the other; the two share only the linked detector above.
        let output_left = one_frame(
            delayed_left,
            detected_left,
            &coef_left,
            &mut channel_left.gain_reduction_db,
            bypassed,
            (zero, one, level_floor, level_min, level_max, reduction_min),
        );
        let output_right = one_frame(
            delayed_right,
            detected_right,
            &coef_right,
            &mut channel_right.gain_reduction_db,
            bypassed,
            (zero, one, level_floor, level_min, level_max, reduction_min),
        );
        output_left.store(&mut left[slot..]);
        output_right.store(&mut right[slot..]);

        // 9.
        channel_left.cursor = next as u32;
        channel_right.cursor = next as u32;
    }
}

/// Reads each lane's detector ring `D[lane]` frames behind the write index.
///
/// The tap is per lane because `lookahead` is a per-lane parameter and is deliberately not part of
/// the program key, so a bank's lanes legitimately disagree about it. `D <= N = B - 1`, so
/// `w + B - D` is in `[1, 2B)` and the compare-select lands it in `[0, B)` without a modulo.
#[inline(always)]
fn gather_detector<L: Lane>(
    channel: &Channel<L>,
    write: usize,
    gather: &mut [f32; MAX_WIDTH],
) -> L {
    let width = L::WIDTH;
    let ring_length = channel.ring_length as usize;
    for (lane, slot) in gather.iter_mut().take(width).enumerate() {
        let delay = channel.delay[lane] as usize;
        let tap = if write >= delay {
            write - delay
        } else {
            write + ring_length - delay
        };
        *slot = channel.detector[tap * width + lane];
    }
    L::load(gather)
}

/// Steps 4 to 8 for one channel of one frame.
#[inline(always)]
fn one_frame<L: Lane>(
    delayed: L,
    detected: L,
    coef: &Coef<L>,
    gain_reduction_db: &mut L,
    bypassed: L::Mask,
    constants: (L, L, L, L, L, L),
) -> L {
    let (zero, one, level_floor, level_min, level_max, reduction_min) = constants;

    // 4. amplitude to level, floored and clamped into the curve's domain.
    //
    // FAST-DB-CROSSING X1: the compressor's detector level. This is a dynamics gain path -- the
    // result is a detector reading that feeds the static curve and is never pinned as a
    // coefficient word -- so it takes the sealed fast tier. Bounded at 2.810e-5 dB, 1.83x the
    // exact tier, by gate F1 in `miso-engine-math`.
    let floored = detected.max(level_floor);
    let level = fast_level_db(floored).max(level_min).min(level_max);

    // 5. the static curve, as the reduction it applies.
    let target = gain_delta_db(level, &coef.curve)
        .max(reduction_min)
        .min(zero);

    // 6. the branching one-pole, and the only `flush` in the crate.
    //
    // The recurrence itself is `envelope::rms_follow` — the runtime's frozen one-rounding form
    // `fma(c, target - y, y)` on a *rate* coefficient, which is the general one-pole and not
    // specific to a mean square (its own documentation says the squaring and the square root
    // belong to the caller). Nothing is added to the runtime for this: what the compressor
    // contributes is the **branch**, and a branch is a `Lane::select`, not a new primitive.
    //
    // Frozen: the select is strict (`target < y`), so equality takes the release coefficient —
    // BRIEFS/013's rule, and the sign convention that makes falling gain reduction the attack.
    // `peak_follow` is the wrong sibling here: its attack is an unconditional `max`, which is a
    // limiter's ballistic, not a compressor's (GMR 2012 section 4.2, "smooth branching").
    let coefficient = L::select(target.lt(*gain_reduction_db), coef.attack, coef.release);
    let smoothed = flush(rms_follow(target, *gain_reduction_db, coefficient));
    *gain_reduction_db = smoothed;

    // 7. level to amplitude.
    //
    // FAST-DB-CROSSING X2: the compressor's applied gain. Bounded at 7.431e-6 dB, 1.06x the
    // exact tier, by gate F1. `fast_gain_from_db(+-0.0)` is exactly `1.0`, so the identity
    // selects below remain true identities.
    let gain = fast_gain_from_db(smoothed.add(coef.makeup));

    // 8. gain and mix, then the identities.
    //
    // `gain_mix_step` is the lane crate's frozen `w = x * g; d = w - x; y = fma(mix, d, x)` — the
    // body of `gain_mix_block`, factored out for exactly this case, where `g` comes out of a
    // detector rather than out of a prepared coefficient. Using it rather than writing the three
    // lines again is what makes a compressor slot and a static gain/mix slot the same law.
    let wet = delayed.mul(gain);
    let mixed = gain_mix_step(delayed, gain, coef.mix);
    let wet_identity = coef.mix.eq(one);
    let dry_identity = L::mask_or(
        bypassed,
        L::mask_or(
            coef.mix.eq(zero),
            L::mask_and(smoothed.eq(zero), coef.makeup.eq(zero)),
        ),
    );
    let output = L::select(wet_identity, wet, mixed);
    L::select(dry_identity, delayed, output)
}

/// The master plan section 4.4 boundary check for one channel of one block.
///
/// Returns the bitmask of lanes that were out of bounds, `0` when the block is clean, and on
/// rejection zeroes the channel's block and clears its recursive state. The policy itself is
/// `miso_engine_effect_runtime::bank::finish_channel`; what belongs to this crate is only *which*
/// state a rejected block resets.
pub(crate) fn finish_channel<L: Lane>(io: &mut [f32], channel: &mut Channel<L>) -> u32 {
    bank::finish_channel::<L>(io, || channel.clear_state())
}
