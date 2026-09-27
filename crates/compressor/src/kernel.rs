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

/// Frames per chunk of the settled body's two passes (issue #983). 16, 64 and 128 measured the
/// same to within 3 %; 32 keeps the stack scratch smallest.
const SETTLED_CHUNK: usize = 32;

#[cfg(test)]
thread_local! {
    /// Settled blocks that took the all-wet arm (issue #982), counted once per block.
    static SETTLED_WET_BLOCKS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

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

/// The applied gain, `fast_gain_from_db(smoothed + makeup)`: the one place the output law crosses
/// into the fast dB tier, shared by `gain_mix` and the all-wet arm (issue #982).
#[inline(always)]
// FAST-DB-CROSSING X2: applied gain conversion is a dynamics result, never a pinned coefficient.
#[expect(
    clippy::disallowed_methods,
    reason = "FAST-DB-CROSSING X2: applied gain from a smoothed reduction, never a pinned coefficient"
)]
fn applied_gain<L: Lane>(smoothed: L, coef: &Coef<L>) -> L {
    fast_gain_from_db(smoothed.add(coef.makeup))
}

#[inline(always)]
fn gain_mix<L: Lane>(input: L, smoothed: L, coef: &Coef<L>, invariants: &Invariants<L>) -> L {
    let gain = applied_gain(smoothed, coef);
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

/// The settled body's output law: `gain_mix`, or on an all-wet, unbypassed block its wet arm
/// alone, `input * gain` (issue #982).
///
/// When every lane has `mix == 1` and the block is unbypassed, `gain_mix` reduces to
/// `select(smoothed == 0 && makeup == 0, input, input * gain)`. Where that select picks `input`,
/// `smoothed` is `+0.0` (`flush` never leaves `-0.0`) and `makeup` is `±0`, so the gain is
/// `fast_gain_from_db(+0.0)`, exactly `1.0` by the form of the sealed tier's `exp2`, and
/// `input * 1.0` is `input` for every word but a signalling NaN, which the multiply quiets.
///
/// That one word is the arm's only difference from `gain_mix`, and it cannot leave the effect:
/// a NaN fails the block-boundary check, which zeroes the whole channel and resets its state, and
/// the rejection mask depends on NaN-ness, not on the payload. The relaxation therefore rests on
/// every production caller (`Instance::render` and `render_mono`) applying `finish_channel` before
/// the output leaves the effect. A new caller that exposes kernel output unchecked must render
/// the general law. The recursive word never reads the output, so it is never affected.
#[inline(always)]
fn settled_output<L: Lane, const WET: bool>(
    input: L,
    smoothed: L,
    coef: &Coef<L>,
    invariants: &Invariants<L>,
) -> L {
    if WET {
        input.mul(applied_gain(smoothed, coef))
    } else {
        gain_mix(input, smoothed, coef, invariants)
    }
}

/// `true` when every lane of `mask` is set. A once-per-block decision, like `Lane::mask_any`.
#[inline(always)]
fn every_lane<L: Lane>(mask: L::Mask) -> bool {
    !L::mask_any(L::mask_not(mask))
}

/// The settled body's detector for `Detector::Main`: `link_frame`, or on a DualMono instance its
/// magnitudes alone (issue #984).
///
/// Under DualMono, `Invariants::new` makes `linked` the all-zero mask and `select` is bitwise, so
/// `link_frame` returns `magnitude`, which is `abs` of the same main word: the arm's bits are
/// `link_frame`'s for every input, NaN included. The link mode is the prepared, whole-instance
/// `metadata.link_mode`, never data. The arm lives only in the two-pass body's first pass: in a
/// loop that also carries the recurrence it made V8 spill the recurrence and run slower.
#[inline(always)]
fn settled_detect<L: Lane, const DUAL_MONO: bool>(
    main_left: L,
    main_right: L,
    invariants: &Invariants<L>,
) -> (L, L) {
    if DUAL_MONO {
        (main_left.abs(), main_right.abs())
    } else {
        link_frame(Detector::Main, 0, main_left, main_right, invariants)
    }
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
        match detector {
            Detector::Main => settled_main::<L>(
                left,
                right,
                ramping,
                frames,
                link,
                bypass,
                channel_left,
                channel_right,
            ),
            // A connected sidechain, present or absent, keeps the one-pass body: its detector is
            // another buffer, so the settled rewrite below does not apply (issue #981).
            Detector::Silent | Detector::Sidechain(..) => frames_loop::<L, false>(
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
            ),
        }
    }
}

/// The settled body of a `Detector::Main` block: frames `start..end`, after every ramp finished.
///
/// The frame law is `frames_loop::<L, false>`'s, on the same values: each frame's target, its
/// recurrence step and its output are computed by the same operations from the same inputs, and
/// the recurrence visits the frames in the same order. What changes is the loop around it:
///
/// * the detector is matched once per block, by the caller, instead of once per frame (#981);
/// * the frames are visited as chunks of the settled slice, so no frame indexes a plane and there
///   is no per-frame bounds check (#981);
/// * both recursive words live in locals for the whole slice and are written back to their
///   channels once, after the loop (#981);
/// * the output law is chosen once per block (#982): when neither channel has a lane that needs
///   anything but the wet arm, and the block is unbypassed, `settled_output`'s `input * gain`
///   replaces `gain_mix`. The masks come from the `Coef` loaded here, after the ramp prefix, so an
///   automated `mix` is seen on the block its ramp finishes in;
/// * each chunk of [`SETTLED_CHUNK`] frames runs in two passes (#983): every frame's target
///   first, then the recurrence and the output frame by frame (see `settled_frames`);
/// * a DualMono instance's first pass detects with `abs` alone (#984, `settled_detect`).
///
/// `#[inline(always)]` is load-bearing: `process_block::<Simd4>` is the one arithmetic-carrying
/// function the wasm callgraph roster names for this kernel, and an outlined body would leave the
/// roster checking only the ramping prefix.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn settled_main<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
    channel_right: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let invariants = Invariants::<L>::new(link, bypass);
    let coef_left = Coef::load(&channel_left.words);
    let coef_right = Coef::load(&channel_right.words);
    let dual_mono = matches!(link, LinkMode::DualMono);
    let wet = !bypass
        && every_lane::<L>(coef_left.wet_identity)
        && every_lane::<L>(coef_right.wet_identity);
    let left = &mut left[start * width..end * width];
    let right = &mut right[start * width..end * width];
    let coefs = (&coef_left, &coef_right);
    let mut gains = (
        channel_left.gain_reduction_db,
        channel_right.gain_reduction_db,
    );
    #[cfg(test)]
    if wet {
        SETTLED_WET_BLOCKS.with(|blocks| blocks.set(blocks.get() + 1));
    }
    match (dual_mono, wet) {
        (true, true) => {
            settled_frames::<L, true, true>(left, right, coefs, &mut gains, &invariants);
        }
        (true, false) => {
            settled_frames::<L, true, false>(left, right, coefs, &mut gains, &invariants);
        }
        (false, true) => {
            settled_frames::<L, false, true>(left, right, coefs, &mut gains, &invariants);
        }
        (false, false) => {
            settled_frames::<L, false, false>(left, right, coefs, &mut gains, &invariants);
        }
    }
    channel_left.gain_reduction_db = gains.0;
    channel_right.gain_reduction_db = gains.1;
}

/// The settled frames of both channels, in chunks of [`SETTLED_CHUNK`] frames of the settled
/// slice (issue #983). The slice starts wherever the ramp prefix ended, so a chunk need not be
/// aligned to the block, and the last chunk may be short; nothing here depends on either.
///
/// The frame law is one dependent chain per channel -- detector, level, curve, recurrence, gain,
/// output -- and only the recurrence and what follows it depend on the previous frame. A frame's
/// target is a function of that frame's input and of coefficients that are constant over the
/// settled slice: the compressor is feed-forward, and no recursive word feeds the curve. So pass 1
/// computes every target of the chunk, both channels, with no frame depending on another, and pass
/// 2 runs the recurrence and the output frame by frame, in today's order. The same operations run
/// on the same values; only their order across independent frames changes, which lets the core
/// keep many target chains in flight where the one-pass body kept about two. Pass 1 reads a
/// chunk's inputs before pass 2 overwrites any of them, and pass 2 reloads each input before it
/// stores that frame's output, so the in-place planes are safe.
///
/// `targets` is stack scratch: `2 * SETTLED_CHUNK` lane words, 2 KiB at `Simd8`, zero-filled once
/// per call. It holds `curve_target` values, which the one-pass body kept in registers or spill
/// slots; it is not a block copy of audio. No in-place form exists: pass 2 needs both a frame's
/// input and its target, and writing the targets into the plane would destroy the input.
#[inline(always)]
fn settled_frames<L: Lane, const DUAL_MONO: bool, const WET: bool>(
    left: &mut [f32],
    right: &mut [f32],
    coefs: (&Coef<L>, &Coef<L>),
    gains: &mut (L, L),
    invariants: &Invariants<L>,
) {
    let width = L::WIDTH;
    let (coef_left, coef_right) = coefs;
    let (mut gain_left, mut gain_right) = *gains;
    let mut targets = [(L::zero(), L::zero()); SETTLED_CHUNK];
    for (chunk_left, chunk_right) in left
        .chunks_mut(SETTLED_CHUNK * width)
        .zip(right.chunks_mut(SETTLED_CHUNK * width))
    {
        // Pass 1: every frame's target. No frame depends on another here.
        for ((frame_left, frame_right), target) in chunk_left
            .chunks_exact(width)
            .zip(chunk_right.chunks_exact(width))
            .zip(targets.iter_mut())
        {
            let (detected_left, detected_right) = settled_detect::<L, DUAL_MONO>(
                L::load(frame_left),
                L::load(frame_right),
                invariants,
            );
            *target = (
                curve_target(detected_left, coef_left, invariants),
                curve_target(detected_right, coef_right, invariants),
            );
        }
        // Pass 2: the recurrence, then the output of the same frame. The two channels'
        // recurrences share nothing, so stepping both before either output is exact.
        for ((frame_left, frame_right), target) in chunk_left
            .chunks_exact_mut(width)
            .zip(chunk_right.chunks_exact_mut(width))
            .zip(targets.iter())
        {
            let main_left = L::load(frame_left);
            let main_right = L::load(frame_right);
            let smoothed_left = ballistic(target.0, &mut gain_left, coef_left);
            let smoothed_right = ballistic(target.1, &mut gain_right, coef_right);
            settled_output::<L, WET>(main_left, smoothed_left, coef_left, invariants)
                .store(frame_left);
            settled_output::<L, WET>(main_right, smoothed_right, coef_right, invariants)
                .store(frame_right);
        }
    }
    *gains = (gain_left, gain_right);
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

    #[test]
    fn both_resets_reseed_attack_and_release_coefficient_ramps() {
        let mut channel = Channel::<f32>::new(&defaults(), SAMPLE_RATE);
        channel.set_parameter_target(3, 0, 180.0, SAMPLE_RATE);
        channel.set_parameter_target(4, 0, 3_200.0, SAMPLE_RATE);
        for _ in 0..19 {
            channel.advance_ramps(SAMPLE_RATE);
        }

        channel.discontinuity_reset(SAMPLE_RATE);
        assert_reset_rate_state(&channel);
        assert_eq!(channel.ramps[3][0].current.to_bits(), 180.0_f32.to_bits());
        assert_eq!(channel.ramps[4][0].current.to_bits(), 3_200.0_f32.to_bits());

        channel.set_parameter_target(3, 0, 100.0, SAMPLE_RATE);
        channel.set_parameter_target(4, 0, 900.0, SAMPLE_RATE);
        for _ in 0..11 {
            channel.advance_ramps(SAMPLE_RATE);
        }
        channel.full_reset(SAMPLE_RATE);
        assert_reset_rate_state(&channel);
        assert_eq!(
            channel.ramps[3][0].current.to_bits(),
            PARAMETER_SPECS[3].default.to_bits()
        );
        assert_eq!(
            channel.ramps[4][0].current.to_bits(),
            PARAMETER_SPECS[4].default.to_bits()
        );
    }

    fn assert_reset_rate_state(channel: &Channel<f32>) {
        for (slot, parameter, coefficient) in [(0, 3, COEF_ATTACK), (1, 4, COEF_RELEASE)] {
            let rate_ramp = channel.rate_ramps[slot][0];
            assert_eq!(rate_ramp.remaining, 0);
            assert_eq!(rate_ramp.current.to_bits(), rate_ramp.target.to_bits());
            assert_eq!(
                rate_ramp.current.to_bits(),
                channel.words[coefficient][0].to_bits()
            );
            assert_eq!(
                rate_ramp.current.to_bits(),
                rate_coefficient(channel.ramps[parameter][0].current, SAMPLE_RATE).to_bits()
            );
        }
    }
}

#[cfg(test)]
mod settled_body_tests {
    //! The settled body against the base kernel (issues #981-#985).
    //!
    //! [`reference`] is `process_block` and `process_block_mono` exactly as they stood before
    //! #981, one-pass `frames_loop` bodies included. It lives here, in the test code, so that every
    //! change to the production bodies is measured against one fixed oracle. It is built only from
    //! the frame law -- `link_frame`, `one_frame`, `Coef`, `Invariants` and
    //! `Channel::advance_ramps` -- which none of #981-#985 edits.
    //!
    //! Three kinds of gate live here:
    //!
    //! * the deterministic grid (`the_settled_body_is_the_base_body_*`, #981 gate 1): the corpus
    //!   track table, every link mode, both `bypass` values, every detector kind, ramp prefixes of
    //!   0, 1, 18 and 40 frames, and the hostile words of the brief;
    //! * the randomized differential (`randomized_differential_*`): seeded blocks with hostile
    //!   input, parameter extremes, automation that ends mid-block, resets, bypass toggles and every
    //!   detector kind, dual and collapsed, at `f32`, `Simd4` and `Simd8`;
    //! * the scenario digests (`scenario_*`, gate 2 of each slice), pinned on the unmodified base
    //!   before the change they gate. A digest pinned after the change would only prove the change
    //!   deterministic (the #944 lesson).
    //!
    //! Every comparison is by bits: rendered words, the recursive words, every coefficient word and
    //! every ramp field, then the `finish_channel` masks, and the same words again after it.

    use super::{
        Channel, Detector, SETTLED_WET_BLOCKS, finish_channel, process_block, process_block_mono,
    };
    use crate::design::{MAX_WIDTH, PARAMETER_COUNT};
    use core::cell::Cell;
    use effect_contract::LinkMode;
    use lane::{Lane, Simd4, Simd8};
    use sha2::{Digest, Sha256};

    type Defaults = [[f32; PARAMETER_COUNT]; MAX_WIDTH];

    /// A scenario block: its frame count, and a `(parameter, lane, value)` retarget before it.
    type ScenarioBlock = (usize, Option<(usize, usize, f32)>);

    const SAMPLE_RATE: u32 = 48_000;
    const QUANTUM: usize = 128;
    const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

    /// The kernel before #981: its block split and its one-pass bodies, verbatim.
    mod reference {
        use super::super::{Channel, Coef, Detector, Invariants, link_frame, one_frame};
        use effect_contract::LinkMode;
        use lane::Lane;

        #[allow(clippy::too_many_arguments)]
        pub(super) fn process_block<L: Lane>(
            left: &mut [f32],
            right: &mut [f32],
            detector: Detector<'_>,
            frames: usize,
            link: LinkMode,
            bypass: bool,
            sample_rate: u32,
            channels: (&mut Channel<L>, &mut Channel<L>),
        ) {
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

        #[allow(clippy::too_many_arguments)]
        pub(super) fn process_block_mono<L: Lane>(
            left: &mut [f32],
            detector: Detector<'_>,
            frames: usize,
            link: LinkMode,
            bypass: bool,
            sample_rate: u32,
            channel_left: &mut Channel<L>,
        ) {
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
    }

    /// The corpus track table (`crate::corpus`, `TRACKS`), in table order: threshold, ratio, knee,
    /// attack, release, makeup, mix. Copied because the corpus keeps it private; it mixes `mix`
    /// values, so no `Simd4` or `Simd8` bank of it is all-wet.
    const CORPUS_TRACKS: [[f32; PARAMETER_COUNT]; 8] = [
        [-18.0, 4.0, 0.0, 10.0, 100.0, 0.0, 1.0],
        [-24.0, 8.0, 24.0, 1.0, 50.0, 3.0, 0.75],
        [-6.0, 1.0, 6.0, 5.0, 200.0, -6.0, 0.5],
        [-40.0, 20.0, 12.0, 0.1, 5.0, 12.0, 0.0],
        [0.0, 2.0, 6.0, 50.0, 1000.0, -24.0, 0.25],
        [-80.0, 1.5, 3.0, 20.0, 5000.0, 24.0, 0.9],
        [-12.0, 12.0, 18.0, 0.5, 20.0, -3.0, 0.6],
        [-30.0, 6.0, 0.0, 2.0, 300.0, 6.0, 1.0],
    ];

    /// The hostile words of #981 gate 1 and of the verification (VERIFY-COMPRESSOR section 1.1).
    fn hostile_words() -> [f32; 22] {
        [
            0.0,
            -0.0,
            f32::from_bits(0x0000_0001),
            f32::from_bits(0x8000_0001),
            f32::from_bits(0x007f_ffff),
            f32::MIN_POSITIVE,
            1.0e29,
            -3.0e30,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            -f32::MAX,
            // A quiet NaN with a payload, and two signalling NaNs of either sign.
            f32::from_bits(0x7fc0_1234),
            f32::from_bits(0xffa0_0001),
            f32::from_bits(0x7f80_0001),
            // Levels that land exactly on thresholds, and the detector floor.
            1.0,
            -1.0,
            0.5,
            -0.25,
            1.0e-8,
            0.1,
            f32::from_bits(0x0000_0003),
        ]
    }

    /// `xorshift64*`.
    struct Rng(u64);

    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1)
        }

        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }

        /// In `[0, 1)`, from the top 24 bits.
        fn unit(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1_u64 << 24) as f32
        }

        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }

        fn chance(&mut self, p: f32) -> bool {
            self.unit() < p
        }

        fn pick<T: Copy>(&mut self, values: &[T]) -> T {
            values[self.below(values.len())]
        }

        fn noise(&mut self, amplitude: f32) -> f32 {
            (self.unit() * 2.0 - 1.0) * amplitude
        }
    }

    const PROFILES: usize = 10;

    /// One input word of a profile.
    fn sample(rng: &mut Rng, profile: usize, lane: usize) -> f32 {
        let hostile = hostile_words();
        match profile {
            0 => rng.noise(0.2),
            1 => rng.noise(1.5),
            // Noise with the hostile words sprinkled in.
            2 => {
                if rng.chance(0.08) {
                    rng.pick(&hostile)
                } else {
                    rng.noise(0.7)
                }
            }
            // `-0.0` and `+0.0` on every lane.
            3 => {
                if (lane + rng.below(2)).is_multiple_of(2) {
                    0.0
                } else {
                    -0.0
                }
            }
            // Subnormals only.
            4 => f32::from_bits(
                (rng.next() as u32 & 0x007f_ffff) | (rng.next() as u32 & 0x8000_0000),
            ),
            // The hostile words without the NaNs.
            5 => {
                let word = rng.pick(&hostile);
                if word.is_nan() { rng.noise(0.3) } else { word }
            }
            // Exact levels.
            6 => rng.pick(&[1.0_f32, -1.0, 0.5, -0.5, 0.25, 2.0, 0.125, -0.1]),
            // Tiny levels around the detector floor.
            7 => {
                let amplitude = rng.pick(&[1.0e-6_f32, 1.0e-8, 3.0e-9, 1.0e-20]);
                rng.noise(amplitude)
            }
            // Signed zeros and a quiet level, with NaNs: the dry-identity case.
            8 => {
                if rng.chance(0.08) {
                    f32::from_bits(rng.pick(&[0x7f80_0001_u32, 0xffa0_0001, 0x7fc0_1234]))
                } else if rng.chance(0.5) {
                    -0.0
                } else {
                    1.0e-7
                }
            }
            // Half clean, half tiny.
            _ => {
                if rng.chance(0.5) {
                    rng.noise(0.9)
                } else {
                    rng.noise(1.0e-5)
                }
            }
        }
    }

    fn fill(rng: &mut Rng, profile: usize, width: usize, out: &mut [f32]) {
        for (index, word) in out.iter_mut().enumerate() {
            *word = sample(rng, profile, index % width);
        }
    }

    /// A legal value of parameter `index`, favouring the domain edges and the identities.
    fn parameter(rng: &mut Rng, index: usize) -> f32 {
        let value = match index {
            0 => {
                rng.pick(&[
                    -80.0_f32,
                    0.0,
                    -f32::from_bits(1),
                    -6.0,
                    -18.0,
                    -40.5,
                    -79.99,
                ]) * if rng.chance(0.3) { rng.unit() } else { 1.0 }
            }
            1 => {
                if rng.chance(0.5) {
                    rng.pick(&[1.0_f32, 20.0, 1.000_000_1, 1.5, 4.0, 19.99])
                } else {
                    1.0 + rng.unit() * 19.0
                }
            }
            // The knee keeps its subnormal widths: the reduction clamp acts on them (#994).
            2 => {
                if rng.chance(0.6) {
                    rng.pick(&[
                        0.0_f32,
                        24.0,
                        f32::from_bits(2),
                        1.0e-40,
                        1.0e-38,
                        6.0,
                        3.0,
                        1.0e-3,
                    ])
                } else {
                    rng.unit() * 24.0
                }
            }
            3 => {
                if rng.chance(0.5) {
                    rng.pick(&[0.1_f32, 200.0, 2.0, 10.0])
                } else {
                    0.1 + rng.unit() * 199.9
                }
            }
            4 => {
                if rng.chance(0.5) {
                    rng.pick(&[5.0_f32, 5000.0, 40.0, 100.0])
                } else {
                    5.0 + rng.unit() * 4995.0
                }
            }
            5 => {
                if rng.chance(0.5) {
                    rng.pick(&[-24.0_f32, 24.0, 0.0, 3.0, -12.0])
                } else {
                    -24.0 + rng.unit() * 48.0
                }
            }
            _ => {
                if rng.chance(0.6) {
                    rng.pick(&[0.0_f32, 1.0, 0.999, 0.5, 0.999_999_94])
                } else {
                    rng.unit()
                }
            }
        };
        // Parameters arrive normalised: never `-0.0`.
        if value == 0.0 { 0.0 } else { value }
    }

    fn random_defaults(rng: &mut Rng, all_wet: bool, makeup_zero: bool) -> Defaults {
        let mut defaults = [[0.0_f32; PARAMETER_COUNT]; MAX_WIDTH];
        for lane in &mut defaults {
            for (index, value) in lane.iter_mut().enumerate() {
                *value = parameter(rng, index);
            }
            if all_wet {
                lane[6] = 1.0;
            }
            if makeup_zero {
                lane[5] = 0.0;
            }
        }
        defaults
    }

    /// Lanes `first..first + W` of a track table, in lanes `0..W`.
    fn table_defaults(
        table: &[[f32; PARAMETER_COUNT]; 8],
        first: usize,
        rotate: usize,
    ) -> Defaults {
        core::array::from_fn(|lane| table[(first + lane + rotate) % 8])
    }

    /// Every word of a channel's processing state: the recursive words, every coefficient word,
    /// and every field of every parameter and rate ramp.
    fn state_words<L: Lane>(channel: &Channel<L>) -> Vec<u32> {
        let mut words = channel.recursive_bits()[..L::WIDTH].to_vec();
        for row in &channel.words {
            words.extend(row.iter().map(|value| value.to_bits()));
        }
        for row in channel.ramps.iter().chain(channel.rate_ramps.iter()) {
            for ramp in row {
                words.extend([
                    ramp.current.to_bits(),
                    ramp.target.to_bits(),
                    ramp.step.to_bits(),
                    ramp.remaining,
                ]);
            }
        }
        words
    }

    fn assert_state<L: Lane>(context: &str, oracle: &Channel<L>, candidate: &Channel<L>) {
        let (oracle, candidate) = (state_words(oracle), state_words(candidate));
        if let Some(index) = (0..oracle.len()).find(|&index| oracle[index] != candidate[index]) {
            panic!(
                "{context}: state word {index} differs: oracle {:#010x}, candidate {:#010x}",
                oracle[index], candidate[index]
            );
        }
    }

    /// Asserts two rendered planes equal by bits.
    fn assert_words(context: &str, oracle: &[f32], candidate: &[f32]) {
        assert_words_relaxed(context, oracle, candidate, false);
    }

    /// Asserts two rendered planes equal by bits, except that with `nan_relaxed` two NaN words may
    /// differ in payload: the all-wet arm's one relaxation (#982). Returns how many did.
    fn assert_words_relaxed(
        context: &str,
        oracle: &[f32],
        candidate: &[f32],
        nan_relaxed: bool,
    ) -> usize {
        assert_eq!(oracle.len(), candidate.len());
        let mut relaxed = 0;
        for (index, (a, b)) in oracle.iter().zip(candidate).enumerate() {
            if a.to_bits() == b.to_bits() {
                continue;
            }
            assert!(
                nan_relaxed && a.is_nan() && b.is_nan(),
                "{context}: word {index} differs: oracle {:#010x}, candidate {:#010x}",
                a.to_bits(),
                b.to_bits()
            );
            relaxed += 1;
        }
        relaxed
    }

    /// The all-wet arm's witness: how many settled blocks have taken it on this thread.
    fn wet_arm_blocks() -> usize {
        SETTLED_WET_BLOCKS.with(Cell::get)
    }

    /// Which detector a block runs with. A sidechain block carries its own planes.
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Source {
        Main,
        Silent,
        Sidechain,
    }

    struct Sidechain {
        left: Vec<f32>,
        right: Vec<f32>,
    }

    impl Sidechain {
        fn detector(&self, source: Source) -> Detector<'_> {
            match source {
                Source::Main => Detector::Main,
                Source::Silent => Detector::Silent,
                Source::Sidechain => Detector::Sidechain(&self.left, &self.right),
            }
        }
    }

    /// What one block did, for the coverage assertions.
    #[derive(Debug, Default)]
    struct Coverage {
        blocks: usize,
        settled: usize,
        settled_mid_block: usize,
        all_wet_settled: usize,
        rejected: usize,
        sidechain: usize,
        /// Words the all-wet arm rendered as a NaN of another payload, every one of them in a
        /// block the boundary check rejected.
        nan_payload: usize,
    }

    impl Coverage {
        fn add(&mut self, other: &Self) {
            self.blocks += other.blocks;
            self.settled += other.settled;
            self.settled_mid_block += other.settled_mid_block;
            self.all_wet_settled += other.all_wet_settled;
            self.rejected += other.rejected;
            self.sidechain += other.sidechain;
            self.nan_payload += other.nan_payload;
        }
    }

    fn all_wet<L: Lane>(channel: &Channel<L>) -> bool {
        (0..L::WIDTH).all(|lane| channel.words[crate::design::COEF_MIX][lane] == 1.0)
    }

    /// Two identically prepared channel pairs: the oracle's and the candidate's.
    struct Dual<L: Lane> {
        oracle: (Channel<L>, Channel<L>),
        candidate: (Channel<L>, Channel<L>),
        sample_rate: u32,
    }

    impl<L: Lane> Dual<L> {
        fn new(left: &Defaults, right: &Defaults, sample_rate: u32) -> Self {
            Self {
                oracle: (
                    Channel::new(left, sample_rate),
                    Channel::new(right, sample_rate),
                ),
                candidate: (
                    Channel::new(left, sample_rate),
                    Channel::new(right, sample_rate),
                ),
                sample_rate,
            }
        }

        fn retarget(&mut self, right: bool, parameter: usize, lane: usize, value: f32) {
            let rate = self.sample_rate;
            for pair in [&mut self.oracle, &mut self.candidate] {
                let channel = if right { &mut pair.1 } else { &mut pair.0 };
                channel.set_parameter_target(parameter, lane, value, rate);
            }
        }

        fn reset(&mut self, full: bool) {
            let rate = self.sample_rate;
            for channel in [
                &mut self.oracle.0,
                &mut self.oracle.1,
                &mut self.candidate.0,
                &mut self.candidate.1,
            ] {
                if full {
                    channel.full_reset(rate);
                } else {
                    channel.discontinuity_reset(rate);
                }
            }
        }

        /// Renders one block both ways and compares everything, before and after the boundary
        /// check. Returns what the block did.
        #[allow(clippy::too_many_arguments)]
        fn block(
            &mut self,
            context: &dyn Fn() -> String,
            input: (&[f32], &[f32]),
            sidechain: &Sidechain,
            source: Source,
            link: LinkMode,
            bypass: bool,
        ) -> Coverage {
            let frames = input.0.len() / L::WIDTH;
            let ramping = (self.oracle.0.max_remaining())
                .max(self.oracle.1.max_remaining())
                .min(frames as u32) as usize;
            let (mut oracle_left, mut oracle_right) = (input.0.to_vec(), input.1.to_vec());
            let (mut candidate_left, mut candidate_right) = (input.0.to_vec(), input.1.to_vec());
            let detector = sidechain.detector(source);
            reference::process_block::<L>(
                &mut oracle_left,
                &mut oracle_right,
                detector,
                frames,
                link,
                bypass,
                self.sample_rate,
                (&mut self.oracle.0, &mut self.oracle.1),
            );
            let wet_before = wet_arm_blocks();
            process_block::<L>(
                &mut candidate_left,
                &mut candidate_right,
                detector,
                frames,
                link,
                bypass,
                self.sample_rate,
                (&mut self.candidate.0, &mut self.candidate.1),
            );
            let wet_arm = wet_arm_blocks() - wet_before;
            let settled = ramping < frames;
            let all_wet_settled = settled
                && source == Source::Main
                && !bypass
                && all_wet(&self.oracle.0)
                && all_wet(&self.oracle.1);
            let context = || format!("{} (frames {frames}, ramping {ramping})", context());
            assert_eq!(
                wet_arm,
                usize::from(all_wet_settled),
                "{}: the all-wet arm runs exactly once on an all-wet, unbypassed settled block",
                context()
            );
            let relaxed_left = assert_words_relaxed(
                &format!("{} left kernel", context()),
                &oracle_left,
                &candidate_left,
                wet_arm == 1,
            );
            let relaxed_right = assert_words_relaxed(
                &format!("{} right kernel", context()),
                &oracle_right,
                &candidate_right,
                wet_arm == 1,
            );
            assert_state(
                &format!("{} left", context()),
                &self.oracle.0,
                &self.candidate.0,
            );
            assert_state(
                &format!("{} right", context()),
                &self.oracle.1,
                &self.candidate.1,
            );
            let oracle_masks = (
                finish_channel::<L>(&mut oracle_left, &mut self.oracle.0),
                finish_channel::<L>(&mut oracle_right, &mut self.oracle.1),
            );
            let candidate_masks = (
                finish_channel::<L>(&mut candidate_left, &mut self.candidate.0),
                finish_channel::<L>(&mut candidate_right, &mut self.candidate.1),
            );
            assert_eq!(oracle_masks, candidate_masks, "{}: finish masks", context());
            // A payload difference is admissible only where the boundary check rejects the block.
            assert!(
                relaxed_left == 0 || oracle_masks.0 != 0,
                "{}: a left NaN payload differs in an accepted block",
                context()
            );
            assert!(
                relaxed_right == 0 || oracle_masks.1 != 0,
                "{}: a right NaN payload differs in an accepted block",
                context()
            );
            assert_words(
                &format!("{} left finished", context()),
                &oracle_left,
                &candidate_left,
            );
            assert_words(
                &format!("{} right finished", context()),
                &oracle_right,
                &candidate_right,
            );
            assert_state(
                &format!("{} left finished", context()),
                &self.oracle.0,
                &self.candidate.0,
            );
            assert_state(
                &format!("{} right finished", context()),
                &self.oracle.1,
                &self.candidate.1,
            );
            Coverage {
                blocks: 1,
                settled: usize::from(settled),
                settled_mid_block: usize::from(settled && ramping > 0),
                all_wet_settled: usize::from(all_wet_settled),
                rejected: usize::from(oracle_masks != (0, 0)),
                sidechain: usize::from(source != Source::Main),
                nan_payload: relaxed_left + relaxed_right,
            }
        }
    }

    /// One channel, both ways, for the collapsed body.
    struct Mono<L: Lane> {
        oracle: Channel<L>,
        candidate: Channel<L>,
        sample_rate: u32,
    }

    impl<L: Lane> Mono<L> {
        fn new(defaults: &Defaults, sample_rate: u32) -> Self {
            Self {
                oracle: Channel::new(defaults, sample_rate),
                candidate: Channel::new(defaults, sample_rate),
                sample_rate,
            }
        }

        fn retarget(&mut self, parameter: usize, lane: usize, value: f32) {
            let rate = self.sample_rate;
            self.oracle
                .set_parameter_target(parameter, lane, value, rate);
            self.candidate
                .set_parameter_target(parameter, lane, value, rate);
        }

        fn reset(&mut self) {
            let rate = self.sample_rate;
            self.oracle.discontinuity_reset(rate);
            self.candidate.discontinuity_reset(rate);
        }

        fn block(
            &mut self,
            context: &dyn Fn() -> String,
            input: &[f32],
            sidechain: &Sidechain,
            source: Source,
            link: LinkMode,
            bypass: bool,
        ) -> Coverage {
            let frames = input.len() / L::WIDTH;
            let ramping = self.oracle.max_remaining().min(frames as u32) as usize;
            let mut oracle = input.to_vec();
            let mut candidate = input.to_vec();
            let detector = sidechain.detector(source);
            reference::process_block_mono::<L>(
                &mut oracle,
                detector,
                frames,
                link,
                bypass,
                self.sample_rate,
                &mut self.oracle,
            );
            let wet_before = wet_arm_blocks();
            process_block_mono::<L>(
                &mut candidate,
                detector,
                frames,
                link,
                bypass,
                self.sample_rate,
                &mut self.candidate,
            );
            assert_eq!(
                wet_arm_blocks(),
                wet_before,
                "the collapsed body has no all-wet arm"
            );
            let settled = ramping < frames;
            let all_wet_settled =
                settled && source == Source::Main && !bypass && all_wet(&self.oracle);
            let context = || format!("{} (frames {frames}, ramping {ramping})", context());
            assert_words(&format!("{} kernel", context()), &oracle, &candidate);
            assert_state(&context(), &self.oracle, &self.candidate);
            let oracle_mask = finish_channel::<L>(&mut oracle, &mut self.oracle);
            let candidate_mask = finish_channel::<L>(&mut candidate, &mut self.candidate);
            assert_eq!(oracle_mask, candidate_mask, "{}: finish mask", context());
            assert_words(&format!("{} finished", context()), &oracle, &candidate);
            assert_state(
                &format!("{} finished", context()),
                &self.oracle,
                &self.candidate,
            );
            Coverage {
                blocks: 1,
                settled: usize::from(settled),
                settled_mid_block: usize::from(settled && ramping > 0),
                all_wet_settled: usize::from(all_wet_settled),
                rejected: usize::from(oracle_mask != 0),
                sidechain: usize::from(source != Source::Main),
                nan_payload: 0,
            }
        }
    }

    /// Two retarget values per parameter; the grid picks the one the ramp is not already heading
    /// to, so every retarget starts a ramp.
    const RETARGETS: [[f32; 2]; PARAMETER_COUNT] = [
        [-33.0, -21.0],
        [5.5, 2.5],
        [7.0, 1.0],
        [15.0, 4.0],
        [250.0, 60.0],
        [1.25, -2.0],
        [0.3, 0.95],
    ];

    /// The grid's block schedule: `(frames, retarget before the block)`. Each counted block is
    /// preceded, in turn, by nothing and by a fully ramping block of `64 - p` frames, so its
    /// settled body starts at frame `p` for `p` in `{0, 1, 18, 40}`.
    fn grid_schedule(counts: &[usize]) -> Vec<(usize, bool)> {
        let mut schedule = Vec::new();
        for &count in counts {
            for prefix in [0_usize, 1, 18, 40] {
                if prefix > 0 {
                    schedule.push((64 - prefix, true));
                }
                schedule.push((count, false));
            }
        }
        schedule
    }

    const GRID_COUNTS: [usize; 7] = [1, 7, 31, 32, 33, 97, 128];

    /// #981 gate 1 at width `L`, over one parameter table.
    fn grid<L: Lane>(label: &str, table: &[[f32; PARAMETER_COUNT]; 8], wet: bool) -> Coverage {
        let width = L::WIDTH;
        let schedule = grid_schedule(&GRID_COUNTS);
        let mut coverage = Coverage::default();
        for link in LINKS {
            for bypass in [false, true] {
                for source in [Source::Main, Source::Silent, Source::Sidechain] {
                    for group in 0..8 / width {
                        let left = table_defaults(table, group * width, 0);
                        let right = table_defaults(table, group * width, 3);
                        let mut dual = Dual::<L>::new(&left, &right, SAMPLE_RATE);
                        let mut mono = Mono::<L>::new(&left, SAMPLE_RATE);
                        let mut rng = Rng::new(0x981 + group as u64);
                        let mut retargets = 0_usize;
                        for (block, &(frames, retarget)) in schedule.iter().enumerate() {
                            if retarget {
                                // A wet table keeps `mix == 1`: its retargets skip the mix row.
                                let parameter = retargets % if wet { 6 } else { 7 };
                                let lane = retargets % width;
                                let right = retargets % 2 == 1;
                                let heading = if right {
                                    &dual.oracle.1
                                } else {
                                    &dual.oracle.0
                                };
                                let [a, b] = RETARGETS[parameter];
                                let value = if heading.ramps[parameter][lane].target == a {
                                    b
                                } else {
                                    a
                                };
                                dual.retarget(right, parameter, lane, value);
                                mono.retarget(parameter, lane, value);
                                retargets += 1;
                            }
                            let words = frames * width;
                            let mut input_left = vec![0.0_f32; words];
                            let mut input_right = vec![0.0_f32; words];
                            fill(&mut rng, block % PROFILES, width, &mut input_left);
                            fill(&mut rng, (block + 3) % PROFILES, width, &mut input_right);
                            let mut sidechain = Sidechain {
                                left: vec![0.0; words],
                                right: vec![0.0; words],
                            };
                            fill(&mut rng, (block + 5) % PROFILES, width, &mut sidechain.left);
                            fill(
                                &mut rng,
                                (block + 7) % PROFILES,
                                width,
                                &mut sidechain.right,
                            );
                            if source == Source::Sidechain {
                                // A quiet main under a loud sidechain: reading the main input as the
                                // detector would render a different gain.
                                for word in input_left.iter_mut().chain(&mut input_right) {
                                    *word *= 1.0e-3;
                                }
                            }
                            let context = || {
                                format!(
                                    "{label} W{width} group {group} {link:?} bypass {bypass} {source:?} block {block}"
                                )
                            };
                            coverage.add(&dual.block(
                                &context,
                                (&input_left, &input_right),
                                &sidechain,
                                source,
                                link,
                                bypass,
                            ));
                            let context = || format!("mono {}", context());
                            coverage.add(&mono.block(
                                &context,
                                &input_left,
                                &sidechain,
                                source,
                                link,
                                bypass,
                            ));
                        }
                    }
                }
            }
        }
        coverage
    }

    fn grid_all(table: &[[f32; PARAMETER_COUNT]; 8], wet: bool) -> Coverage {
        let mut coverage = grid::<f32>("f32", table, wet);
        coverage.add(&grid::<Simd4>("Simd4", table, wet));
        coverage.add(&grid::<Simd8>("Simd8", table, wet));
        println!("grid coverage {coverage:?}");
        assert!(
            coverage.settled_mid_block > 0,
            "the grid must start settled bodies mid-block"
        );
        assert!(
            coverage.rejected > 0,
            "the hostile input must reach the boundary check"
        );
        assert!(
            coverage.rejected < coverage.blocks,
            "most blocks must be accepted"
        );
        coverage
    }

    #[test]
    fn the_settled_body_is_the_base_body_on_the_corpus_table() {
        grid_all(&CORPUS_TRACKS, false);
    }

    /// An all-wet table on which the arm is taken (#982 gate 1): every lane `mix == 1` and
    /// `makeup`, and the lanes cross ratio `{1, 4, 20}` with knee `{0, 6}`.
    fn wet_table(makeup: f32) -> [[f32; PARAMETER_COUNT]; 8] {
        core::array::from_fn(|lane| {
            let lane_f = lane as f32;
            [
                -6.0 - 3.0 * lane_f,
                [1.0, 4.0, 20.0][lane % 3],
                [0.0, 6.0][lane % 2],
                2.0 + 1.5 * lane_f,
                40.0 + 15.0 * lane_f,
                makeup,
                1.0,
            ]
        })
    }

    /// #982 gate 1. Makeup `0` is also what the parameter layer delivers for `-0`; the raw `-0.0`
    /// word, which that layer never delivers, is covered as well.
    #[test]
    fn the_all_wet_arm_is_the_base_body_on_all_wet_tables() {
        for makeup in [0.0, -0.0, 3.0, -12.0] {
            let coverage = grid_all(&wet_table(makeup), true);
            assert!(
                coverage.all_wet_settled > 0,
                "makeup {makeup}: the arm must be taken"
            );
        }
        let coverage = grid_all(&FIXTURE_TRACKS, true);
        assert!(
            coverage.all_wet_settled > 0,
            "the fixture tracks must take the arm"
        );
    }

    /// Runs one `Detector::Main` block and returns how many times it took the all-wet arm.
    fn wet_blocks<L: Lane>(
        channels: &mut (Channel<L>, Channel<L>),
        frames: usize,
        bypass: bool,
    ) -> usize {
        let before = wet_arm_blocks();
        let mut left = vec![0.25_f32; frames * L::WIDTH];
        let mut right = vec![-0.5_f32; frames * L::WIDTH];
        process_block::<L>(
            &mut left,
            &mut right,
            Detector::Main,
            frames,
            LinkMode::DualMono,
            bypass,
            SAMPLE_RATE,
            (&mut channels.0, &mut channels.1),
        );
        wet_arm_blocks() - before
    }

    /// #982 gate 3: the arm's dispatch, which no bit-exactness gate can see.
    fn wet_arm_witness<L: Lane>() {
        let wet = table_defaults(&FIXTURE_TRACKS, 0, 0);
        let pair = |left: &Defaults, right: &Defaults| {
            (
                Channel::<L>::new(left, SAMPLE_RATE),
                Channel::<L>::new(right, SAMPLE_RATE),
            )
        };
        let mut channels = pair(&wet, &wet);
        for frames in [1, 7, 32, 128] {
            assert_eq!(
                wet_blocks(&mut channels, frames, false),
                1,
                "once per all-wet settled block of {frames} frames"
            );
        }
        assert_eq!(wet_blocks(&mut pair(&wet, &wet), 128, true), 0, "bypass");
        let mut nearly = wet;
        nearly[L::WIDTH - 1][6] = 0.999;
        assert_eq!(
            wet_blocks(&mut pair(&nearly, &wet), 128, false),
            0,
            "one left lane at mix 0.999"
        );
        assert_eq!(
            wet_blocks(&mut pair(&wet, &nearly), 128, false),
            0,
            "only the right channel has a non-wet lane"
        );
        let mut channels = pair(&wet, &wet);
        channels.0.set_parameter_target(0, 0, -30.0, SAMPLE_RATE);
        assert_eq!(
            wet_blocks(&mut channels, 64, false),
            0,
            "a block that is all ramping prefix"
        );
        let mut start = wet;
        for lane in &mut start {
            lane[6] = 0.9;
        }
        let mut channels = pair(&start, &start);
        assert_eq!(wet_blocks(&mut channels, 128, false), 0, "mix 0.9");
        for lane in 0..L::WIDTH {
            channels.0.set_parameter_target(6, lane, 1.0, SAMPLE_RATE);
            channels.1.set_parameter_target(6, lane, 1.0, SAMPLE_RATE);
        }
        assert_eq!(
            wet_blocks(&mut channels, 40, false),
            0,
            "40 frames of the 64-frame ramp to mix 1"
        );
        assert_eq!(
            wet_blocks(&mut channels, 128, false),
            1,
            "the ramp to mix 1 ends at frame 24, and the tail takes the arm"
        );
    }

    #[test]
    fn the_all_wet_arm_is_taken_exactly_when_every_lane_is_wet() {
        wet_arm_witness::<f32>();
        wet_arm_witness::<Simd4>();
        wet_arm_witness::<Simd8>();
    }

    /// Randomized dual and collapsed blocks at width `L`, one seed.
    fn randomized<L: Lane>(seed: u64, blocks: usize) -> Coverage {
        let width = L::WIDTH;
        let mut rng = Rng::new(seed);
        let all_wet = rng.chance(0.6);
        let makeup_zero = all_wet && rng.chance(0.4);
        let sample_rate = rng.pick(&[44_100_u32, 48_000, 88_200, 96_000]);
        let left = random_defaults(&mut rng, all_wet, makeup_zero);
        let right = if rng.chance(0.5) {
            left
        } else {
            random_defaults(&mut rng, all_wet, makeup_zero)
        };
        let link = rng.pick(&LINKS);
        let mut bypass = rng.chance(0.2);
        let mut dual = Dual::<L>::new(&left, &right, sample_rate);
        let mut mono = Mono::<L>::new(&left, sample_rate);
        let mut coverage = Coverage::default();
        for block in 0..blocks {
            if rng.chance(0.25) {
                for _ in 0..1 + rng.below(4) {
                    let parameter = rng.below(PARAMETER_COUNT);
                    let lane = rng.below(width);
                    let value = if all_wet && parameter == 6 && rng.chance(0.7) {
                        1.0
                    } else {
                        self::parameter(&mut rng, parameter)
                    };
                    let right = rng.chance(0.5);
                    dual.retarget(right, parameter, lane, value);
                    if !right {
                        mono.retarget(parameter, lane, value);
                    }
                }
            }
            if rng.chance(0.02) {
                dual.reset(false);
                mono.reset();
            }
            if rng.chance(0.01) {
                dual.reset(true);
            }
            if rng.chance(0.03) {
                bypass = !bypass;
            }
            let frames = if rng.chance(0.5) {
                rng.pick(&[1_usize, 7, 31, 32, 33, 63, 64, 65, 97, 127, 128])
            } else {
                1 + rng.below(QUANTUM)
            };
            let source = match rng.below(10) {
                0 => Source::Silent,
                1 => Source::Sidechain,
                _ => Source::Main,
            };
            let words = frames * width;
            let profile = rng.below(PROFILES);
            let profile_right = if rng.chance(0.7) {
                profile
            } else {
                rng.below(PROFILES)
            };
            let mut input_left = vec![0.0_f32; words];
            let mut input_right = vec![0.0_f32; words];
            fill(&mut rng, profile, width, &mut input_left);
            fill(&mut rng, profile_right, width, &mut input_right);
            let mut sidechain = Sidechain {
                left: vec![0.0; words],
                right: vec![0.0; words],
            };
            let profile_sidechain = rng.below(PROFILES);
            fill(&mut rng, profile_sidechain, width, &mut sidechain.left);
            fill(&mut rng, profile_sidechain, width, &mut sidechain.right);
            let context = || {
                format!(
                    "W{width} seed {seed} block {block} {link:?} bypass {bypass} {source:?} profile {profile}"
                )
            };
            coverage.add(&dual.block(
                &context,
                (&input_left, &input_right),
                &sidechain,
                source,
                link,
                bypass,
            ));
            let context = || format!("mono {}", context());
            coverage.add(&mono.block(&context, &input_left, &sidechain, source, link, bypass));
        }
        coverage
    }

    fn seeds() -> u64 {
        if cfg!(debug_assertions) { 10 } else { 320 }
    }

    fn randomized_width<L: Lane>() {
        let mut coverage = Coverage::default();
        for seed in 0..seeds() {
            coverage.add(&randomized::<L>(seed, 128));
        }
        println!("W{} randomized coverage {coverage:?}", L::WIDTH);
        assert!(
            coverage.settled > coverage.blocks / 2,
            "most blocks must reach the settled body"
        );
        assert!(
            coverage.settled_mid_block > 0,
            "some settled bodies must start mid-block"
        );
        assert!(
            coverage.all_wet_settled > 0,
            "some settled blocks must be all-wet and unbypassed"
        );
        assert!(
            coverage.rejected > 0,
            "the hostile input must reach the boundary check"
        );
        assert!(
            coverage.sidechain > 0,
            "the sidechain detectors must be exercised"
        );
    }

    #[test]
    fn randomized_differential_f32() {
        randomized_width::<f32>();
    }

    #[test]
    fn randomized_differential_simd4() {
        randomized_width::<Simd4>();
    }

    #[test]
    fn randomized_differential_simd8() {
        randomized_width::<Simd8>();
    }

    /// Folds a plane into a digest. `canonical` folds every NaN as one word: the only relaxation
    /// the all-wet arm (#982) is allowed, and one the boundary check makes unobservable.
    fn fold(hasher: &mut Sha256, words: &[f32], canonical: bool) {
        for word in words {
            let bits = if canonical && word.is_nan() {
                0x7fc0_0000
            } else {
                word.to_bits()
            };
            hasher.update(bits.to_le_bytes());
        }
    }

    fn fold_state<L: Lane>(hasher: &mut Sha256, channel: &Channel<L>) {
        for bits in &channel.recursive_bits()[..L::WIDTH] {
            hasher.update(bits.to_le_bytes());
        }
    }

    /// Renders a fixed dual scenario through `process_block` and folds, per block, the kernel's
    /// output words and recursive words, then the boundary masks, the finished words and the
    /// recursive words again. Returns how many blocks started their settled body mid-block.
    fn scenario_dual<L: Lane>(
        hasher: &mut Sha256,
        table: &[[f32; PARAMETER_COUNT]; 8],
        link: LinkMode,
        schedule: &[ScenarioBlock],
        canonical: bool,
    ) -> usize {
        let width = L::WIDTH;
        let mut mid_block = 0;
        for group in 0..8 / width {
            let left = table_defaults(table, group * width, 0);
            let right = table_defaults(table, group * width, 3);
            let mut channels = (
                Channel::<L>::new(&left, SAMPLE_RATE),
                Channel::<L>::new(&right, SAMPLE_RATE),
            );
            let mut rng = Rng::new(0x5ce0 + group as u64);
            for (block, &(frames, retarget)) in schedule.iter().enumerate() {
                if let Some((parameter, lane, value)) = retarget {
                    channels
                        .0
                        .set_parameter_target(parameter, lane % width, value, SAMPLE_RATE);
                    channels
                        .1
                        .set_parameter_target(parameter, lane % width, value, SAMPLE_RATE);
                }
                let ramping = channels.0.max_remaining().max(channels.1.max_remaining()) as usize;
                mid_block += usize::from(ramping > 0 && ramping < frames);
                let mut left = vec![0.0_f32; frames * width];
                let mut right = vec![0.0_f32; frames * width];
                fill(&mut rng, block % PROFILES, width, &mut left);
                fill(&mut rng, (block + 3) % PROFILES, width, &mut right);
                process_block::<L>(
                    &mut left,
                    &mut right,
                    Detector::Main,
                    frames,
                    link,
                    false,
                    SAMPLE_RATE,
                    (&mut channels.0, &mut channels.1),
                );
                fold(hasher, &left, canonical);
                fold(hasher, &right, canonical);
                fold_state(hasher, &channels.0);
                fold_state(hasher, &channels.1);
                let masks = [
                    finish_channel::<L>(&mut left, &mut channels.0),
                    finish_channel::<L>(&mut right, &mut channels.1),
                ];
                for mask in masks {
                    hasher.update(mask.to_le_bytes());
                }
                fold(hasher, &left, false);
                fold(hasher, &right, false);
                fold_state(hasher, &channels.0);
                fold_state(hasher, &channels.1);
            }
        }
        mid_block
    }

    fn hex(hasher: Sha256) -> String {
        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }

    /// #981 gate 2: 24 heterogeneous blocks, `Simd4` and `Simd8`, DualMono and Maximum, hostile
    /// input, one automation point that leaves a 23-frame ramp prefix. Pinned on the unmodified
    /// base (`197db1c9`), in dev and release.
    const SCENARIO_981: &str = "57cfd7ce05050c68ab73e6585ccc72bd5403543565cd38471fce47bb263e62a0";

    #[test]
    fn scenario_981_heterogeneous_hostile_render_is_pinned() {
        let schedule: Vec<ScenarioBlock> = [
            128, 1, 7, 31, 32, 33, 128, 41, 128, 64, 97, 128, 127, 2, 128, 65, 63, 128, 16, 128,
            100, 128, 3, 128,
        ]
        .iter()
        .enumerate()
        .map(|(block, &frames)| (frames, (block == 7).then_some((0, 1, -30.0))))
        .collect();
        let mut hasher = Sha256::new();
        let mut mid_block = 0;
        for link in [LinkMode::DualMono, LinkMode::Maximum] {
            mid_block +=
                scenario_dual::<Simd4>(&mut hasher, &CORPUS_TRACKS, link, &schedule, false);
            mid_block +=
                scenario_dual::<Simd8>(&mut hasher, &CORPUS_TRACKS, link, &schedule, false);
        }
        assert!(
            mid_block > 0,
            "the automation point must leave a settled body mid-block"
        );
        let digest = hex(hasher);
        println!("scenario 981 digest {digest}");
        assert_eq!(digest, SCENARIO_981);
    }

    /// The standing console fixture's first eight compressors
    /// (`fixtures/session/v1/console-sixty-four-track-intended.json`, tracks 0-7): heterogeneous
    /// and all-wet, so every `Simd4` and `Simd8` bank of them takes the all-wet arm (#982).
    const FIXTURE_TRACKS: [[f32; PARAMETER_COUNT]; 8] = [
        [-6.0, 1.5, 3.0, 2.0, 40.0, 0.0, 1.0],
        [-7.5, 2.25, 4.5, 3.5, 55.0, 0.5, 1.0],
        [-9.0, 3.0, 6.0, 5.0, 70.0, 1.0, 1.0],
        [-10.5, 3.75, 7.5, 6.5, 85.0, 1.5, 1.0],
        [-12.0, 4.5, 9.0, 8.0, 100.0, 2.0, 1.0],
        [-13.5, 5.25, 3.0, 9.5, 115.0, 2.5, 1.0],
        [-15.0, 6.0, 4.5, 11.0, 130.0, 3.0, 1.0],
        [-16.5, 6.75, 6.0, 12.5, 145.0, 0.0, 1.0],
    ];

    /// #982 gate 2: 24 all-wet heterogeneous blocks, `Simd4` and `Simd8`, DualMono and Average,
    /// hostile input. NaN words fold canonically before the boundary check (the arm's one
    /// relaxation) and by bits after it. Pinned on the unmodified base (`197db1c9`) and on
    /// #981, in dev and release.
    const SCENARIO_982: &str = "cd2d5b11da315893f13e5585bf82fcbb71f9026046497626544a6573ed97c8cf";

    #[test]
    fn scenario_982_all_wet_render_is_pinned() {
        let schedule: Vec<ScenarioBlock> = [
            128, 128, 1, 7, 31, 32, 33, 128, 41, 128, 64, 97, 128, 127, 2, 128, 65, 63, 128, 16,
            128, 100, 3, 128,
        ]
        .iter()
        .enumerate()
        .map(|(block, &frames)| (frames, (block == 8).then_some((0, 2, -21.0))))
        .collect();
        let mut hasher = Sha256::new();
        for link in [LinkMode::DualMono, LinkMode::Average] {
            scenario_dual::<Simd4>(&mut hasher, &FIXTURE_TRACKS, link, &schedule, true);
            scenario_dual::<Simd8>(&mut hasher, &FIXTURE_TRACKS, link, &schedule, true);
        }
        let digest = hex(hasher);
        println!("scenario 982 digest {digest}");
        assert_eq!(digest, SCENARIO_982);
    }
    /// Blocks that straddle 32-frame chunks, and 24-frame fully ramping blocks whose ramps then end
    /// at frame 40 of the next block, so its settled body starts mid-chunk.
    fn straddling_schedule(parameter: usize, values: [f32; 6]) -> Vec<ScenarioBlock> {
        let mut schedule = Vec::new();
        for (round, &frames) in [31_usize, 32, 33, 64, 97, 128].iter().enumerate() {
            schedule.push((frames, None));
            schedule.push((24, Some((parameter, round, values[round]))));
            schedule.push((frames.max(41), None));
        }
        schedule.extend([(128, None), (97, None), (33, None)]);
        schedule
    }

    /// #983 gate 2: the chunk-straddling scenario, both tables, every link mode, `Simd4` and
    /// `Simd8`. Pinned on the unmodified base (`197db1c9`) and on #982, in dev and release.
    const SCENARIO_983: &str = "47ffff05a0f1b605a42acd320948d09b7137d92e2f3b7925381b6c6dc0aed424";

    #[test]
    fn scenario_983_chunk_straddling_render_is_pinned() {
        let schedule = straddling_schedule(0, [-27.0, -33.0, -21.0, -45.0, -9.0, -36.0]);
        let mut hasher = Sha256::new();
        let mut mid_block = 0;
        for link in LINKS {
            for (table, canonical) in [(&CORPUS_TRACKS, false), (&FIXTURE_TRACKS, true)] {
                mid_block += scenario_dual::<Simd4>(&mut hasher, table, link, &schedule, canonical);
                mid_block += scenario_dual::<Simd8>(&mut hasher, table, link, &schedule, canonical);
            }
        }
        // Six ramps end at frame 40, per group, table and link mode.
        assert_eq!(
            mid_block,
            6 * 3 * 3 * 2,
            "every retarget must leave a mid-chunk start"
        );
        let digest = hex(hasher);
        println!("scenario 983 digest {digest}");
        assert_eq!(digest, SCENARIO_983);
    }
}
