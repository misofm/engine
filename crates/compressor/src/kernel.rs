//! Generic causal compressor kernel.
//!
//! The scalar and native bank paths share this one lane-generic implementation. A frame reads
//! the current main/detector sample, updates the gain envelope, and writes the current output.
//! There is no audio or detector history: the compressor adds exactly zero samples of latency.

use effect_contract::LinkMode;
use effect_runtime::bank;
use effect_runtime::dynamics::{gain_delta_db, GainComputerCoef};
use effect_runtime::envelope::rms_follow;
use effect_runtime::ramp::LinearRamp;
use lane::kernels::gain_mix_step;
use lane::{flush, Lane};
use math::fast_db::{fast_gain_from_db, fast_level_db};

use crate::design::{
    design_lane, ALL_PARAMETERS, COEF_ATTACK, COEF_HALF_KNEE, COEF_INV_RATIO_MINUS_ONE,
    COEF_INV_TWO_KNEE, COEF_MAKEUP, COEF_MIX, COEF_RELEASE, COEF_THRESHOLD, CoefWords,
    MAX_WIDTH, PARAMETER_COUNT, RAMP_COUNT,
};

const LEVEL_FLOOR: f32 = 1.0e-8;
const LEVEL_MIN_DB: f32 = -160.0;
const LEVEL_MAX_DB: f32 = 24.0;
const GAIN_REDUCTION_MIN_DB: f32 = -100.0;

/// Detector source selected by the prepared port configuration and current block buffers.
#[derive(Clone, Copy)]
pub(crate) enum Detector<'a> {
    /// The current main input is the detector.
    Main,
    /// A connected sidechain has no buffer for this block, so it detects silence.
    Silent,
    /// Current sidechain samples, planar and frame-major at the active lane width.
    Sidechain(&'a [f32], &'a [f32]),
}

/// One channel of a prepared scalar instance or homogeneous bank.
pub(crate) struct Channel<L: Lane> {
    /// Preparation-time values used by a full reset.
    pub(crate) defaults: [[f32; PARAMETER_COUNT]; MAX_WIDTH],
    /// Coefficients loaded by the realtime path.
    pub(crate) words: CoefWords,
    /// One smoothing ramp per parameter and lane.
    pub(crate) ramps: [[LinearRamp; MAX_WIDTH]; RAMP_COUNT],
    /// The recursive gain-reduction envelope, in dB.
    pub(crate) gain_reduction_db: L,
}

impl<L: Lane> Channel<L> {
    /// Creates a channel and designs all coefficient words on the control side.
    pub(crate) fn new(
        defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH],
        sample_rate: u32,
    ) -> Self {
        let mut channel = Self {
            defaults: *defaults,
            words: [[0.0; MAX_WIDTH]; crate::design::COEF_COUNT],
            ramps: [[LinearRamp::fixed(0.0); MAX_WIDTH]; RAMP_COUNT],
            gain_reduction_db: L::zero(),
        };
        channel.seed_from_defaults(sample_rate);
        channel
    }

    /// Copies all mutable processing state to the other channel at a mono-collapse seam.
    pub(crate) fn copy_state_from(&mut self, source: &Self) {
        self.words = source.words;
        self.ramps = source.ramps;
        self.gain_reduction_db = source.gain_reduction_db;
    }

    fn seed_from_defaults(&mut self, sample_rate: u32) {
        for lane in 0..MAX_WIDTH {
            let values = self.defaults[lane];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                ramp[lane] = LinearRamp::fixed(values[parameter]);
            }
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

    /// Full reset: clear the envelope and restore the seven preparation values.
    pub(crate) fn full_reset(&mut self, sample_rate: u32) {
        self.clear_state();
        self.seed_from_defaults(sample_rate);
    }

    /// Discontinuity reset: clear the envelope and snap ramps to their targets.
    pub(crate) fn discontinuity_reset(&mut self, sample_rate: u32) {
        self.clear_state();
        for lane in 0..L::WIDTH {
            let mut values = [0.0; RAMP_COUNT];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                ramp[lane].snap();
                values[parameter] = ramp[lane].current;
            }
            design_lane(&values, sample_rate, ALL_PARAMETERS, &mut self.words, lane);
        }
    }

    /// Clears only cross-frame processing state.
    pub(crate) fn clear_state(&mut self) {
        self.gain_reduction_db = L::zero();
    }

    pub(crate) fn current_values(&self, lane: usize) -> [f32; RAMP_COUNT] {
        core::array::from_fn(|parameter| self.ramps[parameter][lane].current)
    }

    pub(crate) fn redesign(&mut self, lane: usize, sample_rate: u32) {
        let values = self.current_values(lane);
        design_lane(&values, sample_rate, ALL_PARAMETERS, &mut self.words, lane);
    }

    pub(crate) fn recursive_bits(&self) -> [u32; MAX_WIDTH] {
        let mut words = [0_u32; MAX_WIDTH];
        self.gain_reduction_db.store_bits(&mut words[..L::WIDTH]);
        words
    }

    pub(crate) fn max_remaining(&self) -> u32 {
        let mut most = 0;
        for parameter in &self.ramps {
            for ramp in parameter.iter().take(L::WIDTH) {
                most = most.max(ramp.remaining);
            }
        }
        most
    }

    /// Advances each in-flight ramp and redesigns only coefficients whose parameter moved.
    fn advance_ramps(&mut self, sample_rate: u32) {
        for lane in 0..L::WIDTH {
            let mut changed = 0_u8;
            for (parameter, ramp) in self.ramps.iter().enumerate() {
                if ramp[lane].is_ramping() {
                    changed |= 1 << parameter;
                }
            }
            if changed == 0 {
                continue;
            }
            let mut values = [0.0_f32; RAMP_COUNT];
            for (parameter, ramp) in self.ramps.iter_mut().enumerate() {
                values[parameter] = if changed & (1 << parameter) != 0 {
                    ramp[lane].next_value()
                } else {
                    ramp[lane].current
                };
            }
            design_lane(&values, sample_rate, changed, &mut self.words, lane);
        }
    }
}

struct Coef<L: Lane> {
    curve: GainComputerCoef<L>,
    attack: L,
    release: L,
    makeup: L,
    mix: L,
    wet_identity: L::Mask,
    dry_mix_zero: L::Mask,
    makeup_zero: L::Mask,
}

impl<L: Lane> Coef<L> {
    #[inline(always)]
    fn load(words: &CoefWords) -> Self {
        let makeup = L::load(&words[COEF_MAKEUP]);
        let mix = L::load(&words[COEF_MIX]);
        Self {
            curve: GainComputerCoef {
                threshold_db: L::load(&words[COEF_THRESHOLD]),
                inv_ratio_minus_one: L::load(&words[COEF_INV_RATIO_MINUS_ONE]),
                half_knee_db: L::load(&words[COEF_HALF_KNEE]),
                inv_two_knee: L::load(&words[COEF_INV_TWO_KNEE]),
            },
            attack: L::load(&words[COEF_ATTACK]),
            release: L::load(&words[COEF_RELEASE]),
            makeup,
            mix,
            wet_identity: mix.eq(L::splat(1.0)),
            dry_mix_zero: mix.eq(L::zero()),
            makeup_zero: makeup.eq(L::zero()),
        }
    }
}

struct Invariants<L: Lane> {
    zero: L,
    half: L,
    level_floor: L,
    level_min: L,
    level_max: L,
    reduction_min: L,
    linked: L::Mask,
    averaged: L::Mask,
    bypassed: L::Mask,
}

impl<L: Lane> Invariants<L> {
    #[inline(always)]
    fn new(link: LinkMode, bypass: bool) -> Self {
        let zero = L::zero();
        let all = zero.eq(zero);
        let none = L::mask_not(all);
        Self {
            zero,
            half: L::splat(0.5),
            level_floor: L::splat(LEVEL_FLOOR),
            level_min: L::splat(LEVEL_MIN_DB),
            level_max: L::splat(LEVEL_MAX_DB),
            reduction_min: L::splat(GAIN_REDUCTION_MIN_DB),
            linked: if matches!(link, LinkMode::DualMono) { none } else { all },
            averaged: if matches!(link, LinkMode::Average) { all } else { none },
            bypassed: if bypass { all } else { none },
        }
    }
}

#[inline(always)]
fn link_frame<L: Lane>(
    detector: Detector<'_>,
    slot: usize,
    main_left: L,
    main_right: L,
    invariants: &Invariants<L>,
) -> (L, L) {
    let (source_left, source_right) = match detector {
        Detector::Main => (main_left, main_right),
        Detector::Silent => (invariants.zero, invariants.zero),
        Detector::Sidechain(left, right) => (L::load(&left[slot..]), L::load(&right[slot..])),
    };
    let magnitude_left = source_left.abs();
    let magnitude_right = source_right.abs();
    let maximum = magnitude_left.max(magnitude_right);
    let average = magnitude_left
        .mul(invariants.half)
        .add(magnitude_right.mul(invariants.half));
    let combined = L::select(invariants.averaged, average, maximum);
    (
        L::select(invariants.linked, combined, magnitude_left),
        L::select(invariants.linked, combined, magnitude_right),
    )
}

#[inline(always)]
fn curve_target<L: Lane>(detected: L, coef: &Coef<L>, invariants: &Invariants<L>) -> L {
    let floored = detected.max(invariants.level_floor);
    let level = fast_level_db(floored)
        .max(invariants.level_min)
        .min(invariants.level_max);
    gain_delta_db(level, &coef.curve)
        .max(invariants.reduction_min)
        .min(invariants.zero)
}

#[inline(always)]
fn ballistic<L: Lane>(target: L, gain_reduction_db: &mut L, coef: &Coef<L>) -> L {
    let coefficient = L::select(target.lt(*gain_reduction_db), coef.attack, coef.release);
    let smoothed = flush(rms_follow(target, *gain_reduction_db, coefficient));
    *gain_reduction_db = smoothed;
    smoothed
}

#[inline(always)]
fn gain_mix<L: Lane>(input: L, smoothed: L, coef: &Coef<L>, invariants: &Invariants<L>) -> L {
    let gain = fast_gain_from_db(smoothed.add(coef.makeup));
    let wet = input.mul(gain);
    let mixed = gain_mix_step(input, gain, coef.mix);
    let dry_identity = L::mask_or(
        invariants.bypassed,
        L::mask_or(
            coef.dry_mix_zero,
            L::mask_and(smoothed.eq(invariants.zero), coef.makeup_zero),
        ),
    );
    let output = L::select(coef.wet_identity, wet, mixed);
    L::select(dry_identity, input, output)
}

#[inline(always)]
fn one_frame<L: Lane>(
    input: L,
    detected: L,
    coef: &Coef<L>,
    gain_reduction_db: &mut L,
    invariants: &Invariants<L>,
) -> L {
    let target = curve_target(detected, coef, invariants);
    let smoothed = ballistic(target, gain_reduction_db, coef);
    gain_mix(input, smoothed, coef, invariants)
}

/// Renders one dual-mono block. The bounded ramp-prefix/idle-body split remains, but both bodies
/// use the same direct causal frame law.
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
    let ramping = remaining.min(frames);
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
    let invariants = Invariants::<L>::new(link, bypass);
    let mut coef_left = Coef::load(&channel_left.words);
    let mut coef_right = Coef::load(&channel_right.words);
    for frame in start..end {
        if RAMPING {
            channel_left.advance_ramps(sample_rate);
            channel_right.advance_ramps(sample_rate);
            coef_left = Coef::load(&channel_left.words);
            coef_right = Coef::load(&channel_right.words);
        }
        let slot = frame * width;
        let main_left = L::load(&left[slot..]);
        let main_right = L::load(&right[slot..]);
        let (detected_left, detected_right) =
            link_frame(detector, slot, main_left, main_right, &invariants);
        one_frame(
            main_left,
            detected_left,
            &coef_left,
            &mut channel_left.gain_reduction_db,
            &invariants,
        )
        .store(&mut left[slot..]);
        one_frame(
            main_right,
            detected_right,
            &coef_right,
            &mut channel_right.gain_reduction_db,
            &invariants,
        )
        .store(&mut right[slot..]);
    }
}

/// Renders the collapsed one-plane bank path. The detector is evaluated from the left plane twice
/// to retain the exact linked Average arithmetic used by the dual path.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_block_mono<L: Lane>(
    left: &mut [f32],
    detector: Detector<'_>,
    frames: usize,
    link: LinkMode,
    bypass: bool,
    sample_rate: u32,
    channel_left: &mut Channel<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    let remaining = channel_left.max_remaining() as usize;
    let ramping = remaining.min(frames);
    if ramping > 0 {
        frames_loop_mono::<L, true>(
            left,
            detector,
            0,
            ramping,
            link,
            bypass,
            sample_rate,
            channel_left,
        );
    }
    if ramping < frames {
        frames_loop_mono::<L, false>(
            left,
            detector,
            ramping,
            frames,
            link,
            bypass,
            sample_rate,
            channel_left,
        );
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn frames_loop_mono<L: Lane, const RAMPING: bool>(
    left: &mut [f32],
    detector: Detector<'_>,
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    sample_rate: u32,
    channel_left: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let invariants = Invariants::<L>::new(link, bypass);
    let mut coef_left = Coef::load(&channel_left.words);
    for frame in start..end {
        if RAMPING {
            channel_left.advance_ramps(sample_rate);
            coef_left = Coef::load(&channel_left.words);
        }
        let slot = frame * width;
        let main = L::load(&left[slot..]);
        let (detected, _) = link_frame(detector, slot, main, main, &invariants);
        one_frame(
            main,
            detected,
            &coef_left,
            &mut channel_left.gain_reduction_db,
            &invariants,
        )
        .store(&mut left[slot..]);
    }
}

/// Applies the block-boundary nonfinite policy and clears the envelope on a rejected channel.
pub(crate) fn finish_channel<L: Lane>(io: &mut [f32], channel: &mut Channel<L>) -> u32 {
    bank::finish_channel::<L>(io, || channel.clear_state())
}
