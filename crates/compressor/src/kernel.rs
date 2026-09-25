//! Generic causal compressor kernel.
//!
//! The scalar and native bank paths share this one lane-generic implementation. A frame reads
//! the current main/detector sample, updates the gain envelope, and writes the current output.
//! There is no audio or detector history: the compressor adds exactly zero samples of latency.

use effect_contract::LinkMode;
use effect_runtime::bank;
use effect_runtime::dynamics::{GainComputerCoef, gain_delta_db};
use effect_runtime::envelope::rms_follow;
use effect_runtime::ramp::LinearRamp;
use lane::kernels::gain_mix_step;
use lane::{Lane, flush};
use math::fast_db::{fast_gain_from_db, fast_level_db};

use crate::design::{
    ALL_PARAMETERS, COEF_ATTACK, COEF_HALF_KNEE, COEF_INV_RATIO_MINUS_ONE, COEF_INV_TWO_KNEE,
    COEF_MAKEUP, COEF_MIX, COEF_RELEASE, COEF_THRESHOLD, CoefWords, MAX_WIDTH, PARAMETER_COUNT,
    RAMP_COUNT, SMOOTHING_SAMPLES, design_lane, rate_coefficient,
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
    /// Attack and release coefficient ramps, indexed attack then release.
    ///
    /// Their current values are kept bit-identical to `words[COEF_ATTACK/RELEASE]`. The parameter
    /// ramps remain the public current/target state; these ramps interpolate the exact coefficient
    /// endpoints without evaluating an exponential in the sample loop.
    pub(crate) rate_ramps: [[LinearRamp; MAX_WIDTH]; 2],
    /// The recursive gain-reduction envelope, in dB.
    pub(crate) gain_reduction_db: L,
}

impl<L: Lane> Channel<L> {
    /// Creates a channel and designs all coefficient words on the control side.
    pub(crate) fn new(defaults: &[[f32; PARAMETER_COUNT]; MAX_WIDTH], sample_rate: u32) -> Self {
        let mut channel = Self {
            defaults: *defaults,
            words: [[0.0; MAX_WIDTH]; crate::design::COEF_COUNT],
            ramps: [[LinearRamp::fixed(0.0); MAX_WIDTH]; RAMP_COUNT],
            rate_ramps: [[LinearRamp::fixed(0.0); MAX_WIDTH]; 2],
            gain_reduction_db: L::zero(),
        };
        channel.seed_from_defaults(sample_rate);
        channel
    }

    /// Copies all mutable processing state to the other channel at a mono-collapse seam.
    pub(crate) fn copy_state_from(&mut self, source: &Self) {
        self.words = source.words;
        self.ramps = source.ramps;
        self.rate_ramps = source.rate_ramps;
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
            self.seed_rate_ramps(lane);
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
            self.seed_rate_ramps(lane);
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
        self.seed_rate_ramps(lane);
    }

    fn seed_rate_ramps(&mut self, lane: usize) {
        self.rate_ramps[0][lane] = LinearRamp::fixed(self.words[COEF_ATTACK][lane]);
        self.rate_ramps[1][lane] = LinearRamp::fixed(self.words[COEF_RELEASE][lane]);
    }

    /// Reconstructs the active coefficient trajectories after a version-1 payload restore.
    ///
    /// The payload intentionally keeps its frozen 22-word shape and does not serialize auxiliary
    /// coefficient state. As with the parameter ramp step, which is also reconstructed from the
    /// payload's current/target/remaining triple, the resumed coefficient path starts from the
    /// exact design of the serialized current time and reaches the exact target design after the
    /// serialized number of samples.
    pub(crate) fn restore_rate_ramps(&mut self, lane: usize, sample_rate: u32) {
        for (slot, (parameter, coefficient)) in [(3, COEF_ATTACK), (4, COEF_RELEASE)]
            .into_iter()
            .enumerate()
        {
            let parameter_ramp = self.ramps[parameter][lane];
            let start = self.words[coefficient][lane];
            let target = rate_coefficient(parameter_ramp.target, sample_rate);
            let mut ramp = LinearRamp::fixed(start);
            if parameter_ramp.remaining != 0 {
                ramp.set_target(target, parameter_ramp.remaining);
            }
            self.rate_ramps[slot][lane] = ramp;
        }
    }

    /// Retargets a smoothed parameter and, for attack/release, its coefficient ramp.
    pub(crate) fn set_parameter_target(
        &mut self,
        parameter: usize,
        lane: usize,
        value: f32,
        sample_rate: u32,
    ) {
        self.ramps[parameter][lane].set_target(value, SMOOTHING_SAMPLES);

        let slot = match parameter {
            3 => 0,
            4 => 1,
            _ => return,
        };
        let target = rate_coefficient(value, sample_rate);
        self.rate_ramps[slot][lane].set_target(target, SMOOTHING_SAMPLES);

        // A Point that cancels a time ramp at its current value still has to return a coefficient
        // that was mid-interpolation to the exact design for that value. Keep the exposed time
        // current/target fixed and use its existing remaining field to carry that bounded return.
        if !self.ramps[parameter][lane].is_ramping() && self.rate_ramps[slot][lane].is_ramping() {
            self.ramps[parameter][lane] = LinearRamp {
                current: value,
                target: value,
                step: 0.0,
                remaining: SMOOTHING_SAMPLES,
            };
        }
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

    /// Advances each in-flight parameter ramp and its dependent coefficient ramp.
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
            let rate_bits = (1 << 3) | (1 << 4);
            let designed = changed & !rate_bits;
            if designed != 0 {
                design_lane(&values, sample_rate, designed, &mut self.words, lane);
            }
            if changed & (1 << 3) != 0 {
                self.words[COEF_ATTACK][lane] = self.rate_ramps[0][lane].next_value();
            }
            if changed & (1 << 4) != 0 {
                self.words[COEF_RELEASE][lane] = self.rate_ramps[1][lane].next_value();
            }
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
            linked: if matches!(link, LinkMode::DualMono) {
                none
            } else {
                all
            },
            averaged: if matches!(link, LinkMode::Average) {
                all
            } else {
                none
            },
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
// FAST-DB-CROSSING X1: detector level conversion is a dynamics reading, never a pinned
// coefficient word; the sealed fast approximation is required by the compressor contract.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X1: dynamics detector reading, never a pinned coefficient"
)]
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
// FAST-DB-CROSSING X2: applied gain conversion is a dynamics result, never a pinned coefficient.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X2: applied gain from a smoothed reduction, never a pinned coefficient"
)]
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

#[cfg(test)]
mod coefficient_ramp_tests {
    use super::Channel;
    use crate::design::{
        COEF_ATTACK, COEF_RELEASE, MAX_WIDTH, PARAMETER_COUNT, PARAMETER_SPECS, rate_coefficient,
    };
    use crate::state::{STATE_HEADER_WORDS, commit_channel, validate_channel, write_channel};
    use effect_runtime::ramp::LinearRamp;

    const SAMPLE_RATE: u32 = 48_000;

    fn defaults() -> [[f32; PARAMETER_COUNT]; MAX_WIDTH] {
        let values = core::array::from_fn(|index| PARAMETER_SPECS[index].default);
        [values; MAX_WIDTH]
    }

    fn assert_ramp_matches(channel: &Channel<f32>, slot: usize, coefficient: usize) {
        let ramp = channel.rate_ramps[slot][0];
        assert_eq!(
            ramp.current.to_bits(),
            channel.words[coefficient][0].to_bits(),
            "the current coefficient word and its ramp state must agree"
        );
    }

    #[test]
    fn attack_and_release_coefficient_retargets_are_continuous_and_snap_exactly() {
        let mut channel = Channel::<f32>::new(&defaults(), SAMPLE_RATE);
        for (parameter, slot, coefficient, first_target, second_target) in [
            (3, 0, COEF_ATTACK, 180.0, 100.0),
            (4, 1, COEF_RELEASE, 800.0, 3_200.0),
        ] {
            let initial = channel.words[coefficient][0];
            channel.set_parameter_target(parameter, 0, first_target, SAMPLE_RATE);
            let first_coefficient_target = rate_coefficient(first_target, SAMPLE_RATE);
            let mut expected = LinearRamp::fixed(initial);
            expected.set_target(first_coefficient_target, 64);

            for _ in 0..17 {
                let expected_word = expected.next_value();
                channel.advance_ramps(SAMPLE_RATE);
                assert_eq!(
                    channel.words[coefficient][0].to_bits(),
                    expected_word.to_bits()
                );
                assert_ramp_matches(&channel, slot, coefficient);
            }

            let live = channel.words[coefficient][0];
            channel.set_parameter_target(parameter, 0, second_target, SAMPLE_RATE);
            assert_eq!(
                channel.words[coefficient][0].to_bits(),
                live.to_bits(),
                "retargeting must not jump from the live coefficient"
            );
            let second_coefficient_target = rate_coefficient(second_target, SAMPLE_RATE);
            expected = LinearRamp::fixed(live);
            expected.set_target(second_coefficient_target, 64);
            for update in 0..64 {
                let expected_word = expected.next_value();
                channel.advance_ramps(SAMPLE_RATE);
                assert_eq!(
                    channel.words[coefficient][0].to_bits(),
                    expected_word.to_bits(),
                    "parameter {parameter}, update {update}"
                );
            }
            assert_eq!(
                channel.words[coefficient][0].to_bits(),
                second_coefficient_target.to_bits(),
                "the 64th sample snaps to the exact f64-designed endpoint"
            );
            assert_eq!(channel.rate_ramps[slot][0].remaining, 0);
        }
    }

    #[test]
    fn cancel_to_current_returns_the_live_coefficient_to_its_exact_design() {
        let mut channel = Channel::<f32>::new(&defaults(), SAMPLE_RATE);
        channel.set_parameter_target(3, 0, 180.0, SAMPLE_RATE);
        for _ in 0..19 {
            channel.advance_ramps(SAMPLE_RATE);
        }
        let current_time = channel.ramps[3][0].current;
        let live_coefficient = channel.words[COEF_ATTACK][0];
        let exact_target = rate_coefficient(current_time, SAMPLE_RATE);
        assert_ne!(
            live_coefficient.to_bits(),
            exact_target.to_bits(),
            "fixture must cancel while the coefficient is between its endpoints"
        );

        channel.set_parameter_target(3, 0, current_time, SAMPLE_RATE);
        let parameter_ramp = channel.ramps[3][0];
        assert_eq!(parameter_ramp.current.to_bits(), current_time.to_bits());
        assert_eq!(parameter_ramp.target.to_bits(), current_time.to_bits());
        assert_eq!(parameter_ramp.step.to_bits(), 0.0_f32.to_bits());
        assert_eq!(parameter_ramp.remaining, 64);
        assert_eq!(
            channel.words[COEF_ATTACK][0].to_bits(),
            live_coefficient.to_bits(),
            "the cancel event itself must not jump the coefficient"
        );

        let mut expected = LinearRamp::fixed(live_coefficient);
        expected.set_target(exact_target, 64);
        for update in 0..64 {
            let expected_word = expected.next_value();
            channel.advance_ramps(SAMPLE_RATE);
            assert_eq!(
                channel.words[COEF_ATTACK][0].to_bits(),
                expected_word.to_bits(),
                "cancel return update {update}"
            );
            assert_eq!(
                channel.ramps[3][0].current.to_bits(),
                current_time.to_bits()
            );
        }
        assert_eq!(
            channel.words[COEF_ATTACK][0].to_bits(),
            exact_target.to_bits()
        );
        assert_eq!(channel.ramps[3][0].remaining, 0);
    }

    #[test]
    fn payload_restore_reconstructs_an_active_coefficient_ramp_from_remaining() {
        let mut source = Channel::<f32>::new(&defaults(), SAMPLE_RATE);
        source.set_parameter_target(4, 0, 2_800.0, SAMPLE_RATE);
        for _ in 0..23 {
            source.advance_ramps(SAMPLE_RATE);
        }
        let mut bytes = vec![0_u8; STATE_HEADER_WORDS * 4];
        write_channel(&mut bytes, &source, 0);
        validate_channel(&bytes).expect("valid source payload");

        let mut restored = Channel::<f32>::new(&defaults(), SAMPLE_RATE);
        commit_channel(&bytes, &mut restored, 0, SAMPLE_RATE);
        let parameter = restored.ramps[4][0];
        let expected_start = rate_coefficient(parameter.current, SAMPLE_RATE);
        let expected_target = rate_coefficient(parameter.target, SAMPLE_RATE);
        let mut expected = LinearRamp::fixed(expected_start);
        expected.set_target(expected_target, parameter.remaining);
        assert_eq!(
            restored.words[COEF_RELEASE][0].to_bits(),
            expected_start.to_bits()
        );
        assert_eq!(restored.rate_ramps[1][0].remaining, parameter.remaining);

        for update in 0..parameter.remaining {
            let expected_word = expected.next_value();
            restored.advance_ramps(SAMPLE_RATE);
            assert_eq!(
                restored.words[COEF_RELEASE][0].to_bits(),
                expected_word.to_bits(),
                "restored coefficient update {update}"
            );
        }
        assert_eq!(
            restored.words[COEF_RELEASE][0].to_bits(),
            expected_target.to_bits()
        );
        assert_eq!(restored.ramps[4][0].remaining, 0);
    }
}
