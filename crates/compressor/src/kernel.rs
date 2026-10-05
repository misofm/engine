//! Generic causal compressor kernel.
//!
//! The scalar and native bank paths share this one lane-generic implementation. A frame reads
//! the current main/detector sample, updates the gain envelope, and writes the current output.
//! There is no audio or detector history: the compressor adds exactly zero samples of latency.

use effect_contract::LinkMode;
use effect_runtime::dynamics::{GainComputerCoef, gain_delta_db};
use effect_runtime::envelope::rms_follow;
use effect_runtime::ramp::LinearRamp;
use lane::kernels::{gain_mix_step, ramp_toward};
use lane::{Lane, flush};
use math::fast_db::{fast_gain_from_db, fast_level_db};

use crate::design::{
    ALL_PARAMETERS, COEF_ATTACK, COEF_COUNT, COEF_HALF_KNEE, COEF_INV_RATIO_MINUS_ONE,
    COEF_INV_TWO_KNEE, COEF_MAKEUP, COEF_MIX, COEF_RELEASE, COEF_THRESHOLD, CoefWords, MAX_WIDTH,
    PARAMETER_COUNT, RAMP_COUNT, SMOOTHING_SAMPLES, design_lane, rate_coefficient,
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
    /// Dual blocks whose ramping prefix ran the two-pass body (issue #1006), once per block.
    static RAMPING_DUAL_BLOCKS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// Collapsed blocks whose ramping prefix ran the two-pass body (issue #1006), once per block.
    static RAMPING_MONO_BLOCKS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// Ramping prefixes, dual or collapsed, that took the all-wet arm (issue #1006).
    static RAMPING_WET_BLOCKS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// Adds one to a `#[cfg(test)]` witness counter.
#[cfg(test)]
fn witness(counter: &'static std::thread::LocalKey<core::cell::Cell<usize>>) {
    counter.with(|blocks| blocks.set(blocks.get() + 1));
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
            design_lane(&values, sample_rate, ALL_PARAMETERS, &mut self.words, lane);
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

    fn seed_rate_ramps(&mut self, lane: usize) {
        self.rate_ramps[0][lane] = LinearRamp::fixed(self.words[COEF_ATTACK][lane]);
        self.rate_ramps[1][lane] = LinearRamp::fixed(self.words[COEF_RELEASE][lane]);
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
    ///
    /// Since #1006 only the one-pass prefix of a connected sidechain calls this; the main
    /// detector's prefix advances whole lane vectors (`ChannelRamps`). `#[inline(never)]` keeps it
    /// where it was, out of line: inlined into the collapsed kernel it would put scalar control
    /// arithmetic into a function the wasm roster requires to carry none.
    #[inline(never)]
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
        Self::new(
            GainComputerCoef {
                threshold_db: L::load(&words[COEF_THRESHOLD]),
                inv_ratio_minus_one: L::load(&words[COEF_INV_RATIO_MINUS_ONE]),
                half_knee_db: L::load(&words[COEF_HALF_KNEE]),
                inv_two_knee: L::load(&words[COEF_INV_TWO_KNEE]),
            },
            L::load(&words[COEF_ATTACK]),
            L::load(&words[COEF_RELEASE]),
            L::load(&words[COEF_MAKEUP]),
            L::load(&words[COEF_MIX]),
        )
    }

    /// The coefficient set of the given words, with its three identity masks.
    #[inline(always)]
    fn new(curve: GainComputerCoef<L>, attack: L, release: L, makeup: L, mix: L) -> Self {
        Self {
            curve,
            attack,
            release,
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
fn curve_target<L: Lane>(
    detected: L,
    curve: &GainComputerCoef<L>,
    invariants: &Invariants<L>,
) -> L {
    let floored = detected.max(invariants.level_floor);
    let level = fast_level_db(floored)
        .max(invariants.level_min)
        .min(invariants.level_max);
    gain_delta_db(level, curve)
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
/// a NaN fails the block-boundary check, which zeroes that lane's words of the channel and resets
/// its envelope (issue #1090: per lane, so a bank-mate is untouched), and the rejection mask
/// depends on NaN-ness, not on the payload. The relaxation therefore rests on every production
/// caller (`Instance::render` and `render_mono`) applying that check (`finish_lanes`) before the
/// output leaves the effect. A new caller that exposes kernel output unchecked must render
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
    let target = curve_target(detected, &coef.curve, invariants);
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
        if let Detector::Main = detector {
            if L::WIDTH == 1 {
                ramping_main_scalar::<L>(
                    left,
                    right,
                    ramping,
                    link,
                    bypass,
                    channel_left,
                    channel_right,
                );
            } else {
                ramping_main::<L>(
                    left,
                    right,
                    ramping,
                    link,
                    bypass,
                    channel_left,
                    channel_right,
                );
            }
        } else {
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
            // A connected sidechain, present or absent, takes the same two-pass body with its own
            // detector source (issue #995).
            Detector::Silent | Detector::Sidechain(..) => settled_sidechain::<L>(
                left,
                right,
                detector,
                ramping,
                frames,
                link,
                bypass,
                channel_left,
                channel_right,
            ),
        }
    }
}

/// The settled body of a `Detector::Main` block: frames `start..end`, after every ramp finished.
///
/// The frame law is the one-pass `frames_loop::<L, false>`'s, on the same values: each frame's
/// target, its recurrence step and its output are computed by the same operations from the same
/// inputs, and the recurrence visits the frames in the same order. What changes is the loop around
/// it:
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
            settled_frames::<L, true, true>(
                left,
                right,
                Detector::Main,
                coefs,
                &mut gains,
                &invariants,
            );
        }
        (true, false) => {
            settled_frames::<L, true, false>(
                left,
                right,
                Detector::Main,
                coefs,
                &mut gains,
                &invariants,
            );
        }
        (false, true) => {
            settled_frames::<L, false, true>(
                left,
                right,
                Detector::Main,
                coefs,
                &mut gains,
                &invariants,
            );
        }
        (false, false) => {
            settled_frames::<L, false, false>(
                left,
                right,
                Detector::Main,
                coefs,
                &mut gains,
                &invariants,
            );
        }
    }
    channel_left.gain_reduction_db = gains.0;
    channel_right.gain_reduction_db = gains.1;
}

/// The settled body of a block whose detector is a connected sidechain, present
/// (`Detector::Sidechain`) or absent (`Detector::Silent`): frames `start..end`, after every ramp
/// finished (issue #995).
///
/// It is `settled_main` with the detector read from the sidechain, and the one-pass
/// `frames_loop::<L, false>` it replaces is the oracle, bit for bit. A frame's target depends only
/// on that frame's detector words and on coefficients constant over the settled slice, so the
/// two-pass order is exact here for the reason it is for the main detector (see
/// `settled_frames`); and nothing a pass reads is written by the other, because the sidechain
/// planes are borrowed shared and the main planes exclusively. The sidechain planes are sliced to
/// the settled frames, which they cover: the contract sizes them to the block
/// (`EffectProcessBlock::new`). An absent sidechain detects `+0.0` on every frame, so its one
/// target is computed once (`settled_frames`).
///
/// Two of `settled_main`'s block-level choices carry over and one does not:
///
/// * the DualMono detector arm (#984): under DualMono `link_frame` returns `abs` of whichever
///   source it read, so `abs` of the sidechain word is its result bit for bit;
/// * the linked detector: `link_frame(Detector::Main, ..)` on the sidechain's words is
///   `link_frame(Detector::Sidechain, ..)` on the same words, since the detector only chooses which
///   words are read;
/// * **not** the all-wet arm (#982). Its one relaxation, a signalling NaN quieted by `x * 1.0` in a
///   block the boundary check then rejects, is confined to the main detector, where it was
///   reviewed; a sidechained block renders the general law, which is `frames_loop`'s word for word,
///   NaN payloads included.
///
/// Outlined, unlike `settled_main`, and for a measured reason: inlined beside the four
/// main-detector bodies it reshuffled their register allocation (natively, an extra spill in the
/// unbanked instance's vectorised first pass, +0.4 % on 64 unbanked main-detector instances;
/// under V8, an extra reload in a linked bank's first pass), while out of line it leaves them as
/// they were. One call per sidechained block costs nothing measurable.
///
/// `#[cold]` changes nothing in this body, which is instruction-identical without it. It weights
/// the call site, and so moves only the callers' register and stack-slot allocation: in the #995
/// build, without it the unbanked instance's vectorised first pass kept two more stack accesses
/// per iteration than the batch head's, and in the #1006 build one of that instance's settled
/// loops keeps one more instruction and one more stack access. It is kept for that, and it is the
/// kind of effect any change around `process_block` can move again.
///
/// No bank reaches it -- a connected sidechain never banks -- so the roster's
/// `process_block::<Simd4>` still carries every banked body, and this function's `Simd4`
/// instantiation is dead code the roster does not need to name.
#[allow(clippy::too_many_arguments)]
#[cold]
#[inline(never)]
fn settled_sidechain<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    detector: Detector<'_>,
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
    let settled = start * width..end * width;
    let source = match detector {
        Detector::Sidechain(sidechain_left, sidechain_right) => Detector::Sidechain(
            &sidechain_left[settled.clone()],
            &sidechain_right[settled.clone()],
        ),
        other => other,
    };
    let left = &mut left[settled.clone()];
    let right = &mut right[settled];
    let coefs = (&coef_left, &coef_right);
    let mut gains = (
        channel_left.gain_reduction_db,
        channel_right.gain_reduction_db,
    );
    if matches!(link, LinkMode::DualMono) {
        settled_frames::<L, true, false>(left, right, source, coefs, &mut gains, &invariants);
    } else {
        settled_frames::<L, false, false>(left, right, source, coefs, &mut gains, &invariants);
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
///
/// `source` is where pass 1 reads its detector words (issue #995): the chunk's own planes for
/// `Detector::Main`, the same frames of the sidechain planes (already sliced to the settled
/// frames) for `Detector::Sidechain`. `Detector::Silent` detects the same `+0.0` on every frame, so
/// every target is the one word `link_frame` and `curve_target` make of it: it is computed once,
/// before the chunks, and pass 1 is skipped. `settled_main` passes the literal `Detector::Main`, so
/// in its four instantiations the match below folds away and the body is #983's.
#[inline(always)]
fn settled_frames<L: Lane, const DUAL_MONO: bool, const WET: bool>(
    left: &mut [f32],
    right: &mut [f32],
    source: Detector<'_>,
    coefs: (&Coef<L>, &Coef<L>),
    gains: &mut (L, L),
    invariants: &Invariants<L>,
) {
    let width = L::WIDTH;
    let (coef_left, coef_right) = coefs;
    let (mut gain_left, mut gain_right) = *gains;
    let mut targets = [(L::zero(), L::zero()); SETTLED_CHUNK];
    if let Detector::Silent = source {
        let (detected_left, detected_right) =
            link_frame(source, 0, invariants.zero, invariants.zero, invariants);
        targets = [(
            curve_target(detected_left, &coef_left.curve, invariants),
            curve_target(detected_right, &coef_right.curve, invariants),
        ); SETTLED_CHUNK];
    }
    let mut offset = 0;
    for (chunk_left, chunk_right) in left
        .chunks_mut(SETTLED_CHUNK * width)
        .zip(right.chunks_mut(SETTLED_CHUNK * width))
    {
        let words = chunk_left.len();
        let detector_planes = match source {
            Detector::Main => Some((&*chunk_left, &*chunk_right)),
            Detector::Sidechain(sidechain_left, sidechain_right) => Some((
                &sidechain_left[offset..offset + words],
                &sidechain_right[offset..offset + words],
            )),
            Detector::Silent => None,
        };
        offset += words;
        // Pass 1: every frame's target. No frame depends on another here.
        if let Some((planes_left, planes_right)) = detector_planes {
            for ((frame_left, frame_right), target) in planes_left
                .chunks_exact(width)
                .zip(planes_right.chunks_exact(width))
                .zip(targets.iter_mut())
            {
                let (detected_left, detected_right) = settled_detect::<L, DUAL_MONO>(
                    L::load(frame_left),
                    L::load(frame_right),
                    invariants,
                );
                *target = (
                    curve_target(detected_left, &coef_left.curve, invariants),
                    curve_target(detected_right, &coef_right.curve, invariants),
                );
            }
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

/// The one-frame-at-a-time body: frames `start..end`, reading the detector through `link_frame`.
///
/// Production runs it only as the ramp prefix (`RAMPING`) of a connected sidechain, present or
/// absent, where every frame advances the ramps and reloads the coefficients. Since #995 every
/// settled slice, whatever its detector, runs the two-pass body, and since #1006 so does the
/// main detector's prefix (`ramping_main`), so nothing instantiates `RAMPING = false` here any
/// more; the one-pass forms those bodies must equal live on as the oracle in
/// `settled_body_tests::reference`. The sidechain's prefix is left exactly as it was, parameter
/// included, rather than rewritten around a constant.
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

/// Parameter bits of the static curve: threshold, ratio and knee.
const CURVE_BITS: u8 = 0b0000_0111;
/// The mix parameter's bit.
const MIX_BIT: u8 = 1 << 6;
/// The parameter each rate ramp follows, and the coefficient word it writes: attack, release.
const RATE_PARAMETER: [usize; 2] = [3, 4];
const RATE_WORD: [usize; 2] = [COEF_ATTACK, COEF_RELEASE];

/// One parameter's (or one rate coefficient's) `LinearRamp` on every lane, as lane vectors.
///
/// `remaining` is carried as `f32`. It is an integer no greater than [`SMOOTHING_SAMPLES`] (a
/// restored payload is validated to that bound), so the conversion both ways is exact and the
/// compares below are integer compares.
#[derive(Clone, Copy)]
struct RampVec<L: Lane> {
    current: L,
    target: L,
    step: L,
    remaining: L,
}

impl<L: Lane> RampVec<L> {
    #[inline(always)]
    fn gather(ramps: &[LinearRamp; MAX_WIDTH]) -> Self {
        let mut current = [0.0_f32; MAX_WIDTH];
        let mut target = [0.0_f32; MAX_WIDTH];
        let mut step = [0.0_f32; MAX_WIDTH];
        let mut remaining = [0.0_f32; MAX_WIDTH];
        for (lane, ramp) in ramps.iter().enumerate().take(L::WIDTH) {
            current[lane] = ramp.current;
            target[lane] = ramp.target;
            step[lane] = ramp.step;
            remaining[lane] = ramp.remaining as f32;
        }
        Self {
            current: L::load(&current),
            target: L::load(&target),
            step: L::load(&step),
            remaining: L::load(&remaining),
        }
    }

    /// Writes back what an advance can change: `current`, `step` and `remaining`.
    #[inline(always)]
    fn scatter(self, ramps: &mut [LinearRamp; MAX_WIDTH]) {
        let mut current = [0.0_f32; MAX_WIDTH];
        let mut step = [0.0_f32; MAX_WIDTH];
        let mut remaining = [0.0_f32; MAX_WIDTH];
        self.current.store(&mut current);
        self.step.store(&mut step);
        self.remaining.store(&mut remaining);
        for (lane, ramp) in ramps.iter_mut().enumerate().take(L::WIDTH) {
            ramp.current = current[lane];
            ramp.step = step[lane];
            ramp.remaining = remaining[lane] as u32;
        }
    }

    /// `LinearRamp::next_value` on the lanes of `gate`, as selects: `remaining == 0` leaves every
    /// word as it is (at rest, or a restored ramp whose `current` is not its `target`),
    /// `remaining == 1` assigns the target and clears the step, and otherwise the word advances
    /// once by `ramp_toward(current, step, target)` (the step added, held inside `[min(current,
    /// target), max(current, target)]` so it never passes its target, issue #1409) and `remaining`
    /// counts down. Lanes outside `gate` are untouched. Returns the lanes
    /// that were in flight, which are the lanes `next_value` would have advanced.
    #[inline(always)]
    fn advance_where(&mut self, gate: L::Mask) -> L::Mask {
        let zero = L::zero();
        let one = L::splat(1.0);
        let moving = L::mask_and(gate, self.remaining.gt(zero));
        let last = L::mask_and(moving, self.remaining.eq(one));
        let next = L::select(
            last,
            self.target,
            ramp_toward(self.current, self.step, self.target),
        );
        self.current = L::select(moving, next, self.current);
        self.step = L::select(last, zero, self.step);
        self.remaining = L::select(moving, self.remaining.sub(one), self.remaining);
        moving
    }
}

/// One channel's ramps and coefficient words as lane vectors, for one ramping prefix.
///
/// `Channel::advance_ramps` per frame, lane-wide: gathered once per block, advanced once per
/// frame in the pass that reads them, and scattered once. A parameter without a ramp in flight on
/// any lane at the start of the prefix cannot start one inside it (retargets land between blocks),
/// so `active` decides once which parameters advance at all.
struct ChannelRamps<L: Lane> {
    /// Parameters with a ramp in flight on some lane at the start of the prefix.
    active: u8,
    ramps: [RampVec<L>; RAMP_COUNT],
    /// The attack and release coefficient ramps.
    rates: [RampVec<L>; 2],
    words: [L; COEF_COUNT],
}

impl<L: Lane> ChannelRamps<L> {
    #[inline(always)]
    fn gather(channel: &Channel<L>) -> Self {
        let mut active = 0_u8;
        for (parameter, ramps) in channel.ramps.iter().enumerate() {
            if ramps.iter().take(L::WIDTH).any(LinearRamp::is_ramping) {
                active |= 1 << parameter;
            }
        }
        Self {
            active,
            ramps: core::array::from_fn(|parameter| RampVec::gather(&channel.ramps[parameter])),
            rates: core::array::from_fn(|slot| RampVec::gather(&channel.rate_ramps[slot])),
            words: core::array::from_fn(|word| L::load(&channel.words[word])),
        }
    }

    #[inline(always)]
    fn scatter(&self, channel: &mut Channel<L>) {
        for (parameter, ramp) in self.ramps.iter().enumerate() {
            if self.active & (1 << parameter) != 0 {
                ramp.scatter(&mut channel.ramps[parameter]);
            }
        }
        for (slot, ramp) in self.rates.iter().enumerate() {
            if self.active & (1 << RATE_PARAMETER[slot]) != 0 {
                ramp.scatter(&mut channel.rate_ramps[slot]);
            }
        }
        for (word, value) in self.words.iter().enumerate() {
            value.store(&mut channel.words[word]);
        }
    }

    /// One frame of `advance_ramps` for threshold, ratio and knee: advance them, then write the
    /// redesigned curve into the lanes where any of the three moved, which is `design_lane`'s
    /// `changed` rule.
    #[inline(always)]
    fn advance_curve(&mut self) {
        if self.active & CURVE_BITS == 0 {
            return;
        }
        let all = L::zero().eq(L::zero());
        let mut moved = L::mask_not(all);
        for parameter in 0..3 {
            if self.active & (1 << parameter) != 0 {
                moved = L::mask_or(moved, self.ramps[parameter].advance_where(all));
            }
        }
        let curve = design_curve::<L>(
            self.ramps[0].current,
            self.ramps[1].current,
            self.ramps[2].current,
        );
        for (word, value) in [
            (COEF_THRESHOLD, curve.threshold_db),
            (COEF_INV_RATIO_MINUS_ONE, curve.inv_ratio_minus_one),
            (COEF_HALF_KNEE, curve.half_knee_db),
            (COEF_INV_TWO_KNEE, curve.inv_two_knee),
        ] {
            self.words[word] = L::select(moved, value, self.words[word]);
        }
    }

    /// One frame of `advance_ramps` for attack, release, makeup and mix. A rate ramp advances on
    /// the lanes whose parameter ramp was in flight, as `advance_ramps` gates it, whatever its own
    /// `remaining` says.
    #[inline(always)]
    fn advance_output(&mut self) {
        let all = L::zero().eq(L::zero());
        for slot in 0..2 {
            let parameter = RATE_PARAMETER[slot];
            if self.active & (1 << parameter) != 0 {
                let moved = self.ramps[parameter].advance_where(all);
                self.rates[slot].advance_where(moved);
                let word = RATE_WORD[slot];
                self.words[word] = L::select(moved, self.rates[slot].current, self.words[word]);
            }
        }
        for (parameter, word) in [(5, COEF_MAKEUP), (6, COEF_MIX)] {
            if self.active & (1 << parameter) != 0 {
                let moved = self.ramps[parameter].advance_where(all);
                self.words[word] =
                    L::select(moved, self.ramps[parameter].current, self.words[word]);
            }
        }
    }

    /// The static curve's words, which is all pass 1 reads.
    #[inline(always)]
    fn curve(&self) -> GainComputerCoef<L> {
        GainComputerCoef {
            threshold_db: self.words[COEF_THRESHOLD],
            inv_ratio_minus_one: self.words[COEF_INV_RATIO_MINUS_ONE],
            half_knee_db: self.words[COEF_HALF_KNEE],
            inv_two_knee: self.words[COEF_INV_TWO_KNEE],
        }
    }

    /// `Coef::load` of the current words.
    #[inline(always)]
    fn coef(&self) -> Coef<L> {
        Coef::new(
            self.curve(),
            self.words[COEF_ATTACK],
            self.words[COEF_RELEASE],
            self.words[COEF_MAKEUP],
            self.words[COEF_MIX],
        )
    }

    /// `true` when no mix ramp is open and every lane's mix is exactly `1`.
    #[inline(always)]
    fn wet(&self) -> bool {
        self.active & MIX_BIT == 0 && every_lane::<L>(self.words[COEF_MIX].eq(L::splat(1.0)))
    }
}

/// `GainComputerCoef::new` on every lane: `1/R - 1` as `1 / R - 1`, and `knee_coefficients` with
/// its branch as a select, `(W / 2, 1 / (2 W))` where `W > 0` and the reciprocal is below
/// `+inf`, else `(+0, +0)`. The same `f32` operations on the same operands, lane by lane.
#[inline(always)]
fn design_curve<L: Lane>(threshold: L, ratio: L, knee: L) -> GainComputerCoef<L> {
    let zero = L::zero();
    let one = L::splat(1.0);
    let inv_two_knee = one.div(L::splat(2.0).mul(knee));
    let soft = L::mask_and(knee.gt(zero), inv_two_knee.lt(L::splat(f32::INFINITY)));
    GainComputerCoef {
        threshold_db: threshold,
        inv_ratio_minus_one: one.div(ratio).sub(one),
        half_knee_db: L::select(soft, L::splat(0.5).mul(knee), zero),
        inv_two_knee: L::select(soft, inv_two_knee, zero),
    }
}

/// The ramping prefix of a `Detector::Main` block, frames `0..end`: the settled body's two passes
/// with the ramps advanced lane-wide (issue #1006).
///
/// `frames_loop::<L, true>` is the oracle. Per frame it advances every ramp (`advance_ramps`),
/// reloads the words and runs the one-pass law. Here each ramp advances exactly once per frame,
/// in the pass that reads it:
///
/// * a frame's target depends on its input and on the curve words, which depend only on the
///   threshold, ratio and knee ramps, so pass 1 advances those three and computes the chunk's
///   targets;
/// * pass 2 advances attack, release, makeup and mix, then runs the recurrence and the output,
///   frame by frame.
///
/// The word each frame reads is therefore the one-pass loop's, and the order across independent
/// frames is #983's. The DualMono detector arm is #984's. The all-wet arm (#982) is taken when the
/// block is unbypassed, no mix ramp is open on either channel and every lane's mix is `1`: then
/// mix is `1` on every frame, and the arm's argument holds frame by frame whatever makeup does.
///
/// `#[inline(always)]`, so that `process_block::<Simd4>` carries the bank's prefix and the wasm
/// roster checks it, as it did the one-pass prefix; a scalar instance calls it through
/// `ramping_main_scalar`.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn ramping_main<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
    channel_right: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let invariants = Invariants::<L>::new(link, bypass);
    let mut ramps_left = ChannelRamps::gather(channel_left);
    let mut ramps_right = ChannelRamps::gather(channel_right);
    let dual_mono = matches!(link, LinkMode::DualMono);
    let wet = !bypass && ramps_left.wet() && ramps_right.wet();
    let left = &mut left[..end * width];
    let right = &mut right[..end * width];
    let mut gains = (
        channel_left.gain_reduction_db,
        channel_right.gain_reduction_db,
    );
    #[cfg(test)]
    {
        witness(&RAMPING_DUAL_BLOCKS);
        if wet {
            witness(&RAMPING_WET_BLOCKS);
        }
    }
    let ramps = (&mut ramps_left, &mut ramps_right);
    match (dual_mono, wet) {
        (true, true) => {
            ramping_frames::<L, true, true>(left, right, ramps, &mut gains, &invariants);
        }
        (true, false) => {
            ramping_frames::<L, true, false>(left, right, ramps, &mut gains, &invariants);
        }
        (false, true) => {
            ramping_frames::<L, false, true>(left, right, ramps, &mut gains, &invariants);
        }
        (false, false) => {
            ramping_frames::<L, false, false>(left, right, ramps, &mut gains, &invariants);
        }
    }
    channel_left.gain_reduction_db = gains.0;
    channel_right.gain_reduction_db = gains.1;
    ramps_left.scatter(channel_left);
    ramps_right.scatter(channel_right);
}

/// `ramping_main` out of line, for a scalar instance (`L::WIDTH == 1`) only.
///
/// Measured, not assumed: inlined into `process_block::<f32>`, the prefix made that function too
/// large to inline into the scalar instance's `process`, and the unbanked settled loops came out
/// with more instructions and stack traffic (natively, +1.0 % on 64 settled unbanked instances,
/// slower in five of six alternations). Out of line, those loops are the batch head's again. A
/// bank keeps the prefix inline, so the wasm roster's `process_block::<Simd4>` carries it; one
/// call per ramping block is all this costs a scalar instance.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn ramping_main_scalar<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
    channel_right: &mut Channel<L>,
) {
    ramping_main::<L>(left, right, end, link, bypass, channel_left, channel_right);
}

/// The ramping prefix's two passes over chunks of [`SETTLED_CHUNK`] frames, both channels. The
/// chunking, the scratch and the in-place argument are `settled_frames`'.
#[inline(always)]
fn ramping_frames<L: Lane, const DUAL_MONO: bool, const WET: bool>(
    left: &mut [f32],
    right: &mut [f32],
    ramps: (&mut ChannelRamps<L>, &mut ChannelRamps<L>),
    gains: &mut (L, L),
    invariants: &Invariants<L>,
) {
    let width = L::WIDTH;
    let (ramps_left, ramps_right) = ramps;
    let (mut gain_left, mut gain_right) = *gains;
    let mut targets = [(L::zero(), L::zero()); SETTLED_CHUNK];
    for (chunk_left, chunk_right) in left
        .chunks_mut(SETTLED_CHUNK * width)
        .zip(right.chunks_mut(SETTLED_CHUNK * width))
    {
        // Pass 1: the curve ramps, then every frame's target.
        for ((frame_left, frame_right), target) in chunk_left
            .chunks_exact(width)
            .zip(chunk_right.chunks_exact(width))
            .zip(targets.iter_mut())
        {
            ramps_left.advance_curve();
            ramps_right.advance_curve();
            let (detected_left, detected_right) = settled_detect::<L, DUAL_MONO>(
                L::load(frame_left),
                L::load(frame_right),
                invariants,
            );
            *target = (
                curve_target(detected_left, &ramps_left.curve(), invariants),
                curve_target(detected_right, &ramps_right.curve(), invariants),
            );
        }
        // Pass 2: the output ramps, then the recurrence and the output of the same frame.
        for ((frame_left, frame_right), target) in chunk_left
            .chunks_exact_mut(width)
            .zip(chunk_right.chunks_exact_mut(width))
            .zip(targets.iter())
        {
            ramps_left.advance_output();
            ramps_right.advance_output();
            let coef_left = ramps_left.coef();
            let coef_right = ramps_right.coef();
            let main_left = L::load(frame_left);
            let main_right = L::load(frame_right);
            let smoothed_left = ballistic(target.0, &mut gain_left, &coef_left);
            let smoothed_right = ballistic(target.1, &mut gain_right, &coef_right);
            settled_output::<L, WET>(main_left, smoothed_left, &coef_left, invariants)
                .store(frame_left);
            settled_output::<L, WET>(main_right, smoothed_right, &coef_right, invariants)
                .store(frame_right);
        }
    }
    *gains = (gain_left, gain_right);
}

/// `ramping_main` on the collapsed plane (issue #1006): the one channel's ramps, the detector
/// `link_frame(main, main).0` (under DualMono its `abs` arm), and `frames_loop_mono::<L, true>`
/// as the oracle. Mono stems run this prefix whenever one of their compressor's knobs moves.
#[inline(always)]
fn ramping_main_mono<L: Lane>(
    left: &mut [f32],
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let invariants = Invariants::<L>::new(link, bypass);
    let mut ramps = ChannelRamps::gather(channel_left);
    let dual_mono = matches!(link, LinkMode::DualMono);
    let wet = !bypass && ramps.wet();
    let left = &mut left[..end * width];
    let mut gain = channel_left.gain_reduction_db;
    #[cfg(test)]
    {
        witness(&RAMPING_MONO_BLOCKS);
        if wet {
            witness(&RAMPING_WET_BLOCKS);
        }
    }
    match (dual_mono, wet) {
        (true, true) => {
            ramping_frames_mono::<L, true, true>(left, &mut ramps, &mut gain, &invariants);
        }
        (true, false) => {
            ramping_frames_mono::<L, true, false>(left, &mut ramps, &mut gain, &invariants);
        }
        (false, true) => {
            ramping_frames_mono::<L, false, true>(left, &mut ramps, &mut gain, &invariants);
        }
        (false, false) => {
            ramping_frames_mono::<L, false, false>(left, &mut ramps, &mut gain, &invariants);
        }
    }
    channel_left.gain_reduction_db = gain;
    ramps.scatter(channel_left);
}

/// `ramping_frames` on the one plane.
#[inline(always)]
fn ramping_frames_mono<L: Lane, const DUAL_MONO: bool, const WET: bool>(
    left: &mut [f32],
    ramps: &mut ChannelRamps<L>,
    gain: &mut L,
    invariants: &Invariants<L>,
) {
    let width = L::WIDTH;
    let mut gain_left = *gain;
    let mut targets = [L::zero(); SETTLED_CHUNK];
    for chunk in left.chunks_mut(SETTLED_CHUNK * width) {
        // Pass 1: the curve ramps, then every frame's target.
        for (frame, target) in chunk.chunks_exact(width).zip(targets.iter_mut()) {
            ramps.advance_curve();
            let main = L::load(frame);
            let (detected, _) = settled_detect::<L, DUAL_MONO>(main, main, invariants);
            *target = curve_target(detected, &ramps.curve(), invariants);
        }
        // Pass 2: the output ramps, then the recurrence and the output.
        for (frame, target) in chunk.chunks_exact_mut(width).zip(targets.iter()) {
            ramps.advance_output();
            let coef = ramps.coef();
            let main = L::load(frame);
            let smoothed = ballistic(*target, &mut gain_left, &coef);
            settled_output::<L, WET>(main, smoothed, &coef, invariants).store(frame);
        }
    }
    *gain = gain_left;
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
        if let Detector::Main = detector {
            ramping_main_mono::<L>(left, ramping, link, bypass, channel_left);
        } else {
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
    }
    if ramping < frames {
        match detector {
            Detector::Main => {
                settled_main_mono::<L>(left, ramping, frames, link, bypass, channel_left);
            }
            Detector::Silent | Detector::Sidechain(..) => frames_loop_mono::<L, false>(
                left,
                detector,
                ramping,
                frames,
                link,
                bypass,
                sample_rate,
                channel_left,
            ),
        }
    }
}

/// The collapsed body's settled frames: `settled_main`'s rewrite on the one plane (issue #985).
///
/// All four of #981-#984 land here together -- the chunked slice with the recursive word in a
/// local, the output law chosen once per block, the two passes, and the DualMono detector arm --
/// because the loop hygiene alone made this body slower natively. The collapsed contract does not
/// change: the block reads and writes the left plane only, and its detector is
/// `link_frame(main, main).0`, which under DualMono is `abs(main)` bit for bit. Under Maximum and
/// Average it stays `link_frame`: Average's `0.5|m| + 0.5|m|` is not `|m|` for a subnormal `m`, and
/// the collapsed body must render the dual body's left plane exactly.
///
/// `#[inline(always)]`, for the roster's `process_block_mono::<Simd4>` row, as `settled_main`.
#[inline(always)]
fn settled_main_mono<L: Lane>(
    left: &mut [f32],
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
) {
    let width = L::WIDTH;
    let invariants = Invariants::<L>::new(link, bypass);
    let coef = Coef::load(&channel_left.words);
    let dual_mono = matches!(link, LinkMode::DualMono);
    let wet = !bypass && every_lane::<L>(coef.wet_identity);
    let left = &mut left[start * width..end * width];
    let mut gain = channel_left.gain_reduction_db;
    #[cfg(test)]
    if wet {
        SETTLED_WET_BLOCKS.with(|blocks| blocks.set(blocks.get() + 1));
    }
    match (dual_mono, wet) {
        (true, true) => {
            settled_frames_mono::<L, true, true>(left, &coef, &mut gain, &invariants);
        }
        (true, false) => {
            settled_frames_mono::<L, true, false>(left, &coef, &mut gain, &invariants);
        }
        (false, true) => {
            settled_frames_mono::<L, false, true>(left, &coef, &mut gain, &invariants);
        }
        (false, false) => {
            settled_frames_mono::<L, false, false>(left, &coef, &mut gain, &invariants);
        }
    }
    channel_left.gain_reduction_db = gain;
}

/// `settled_frames` on the one plane. Its scratch is `SETTLED_CHUNK` lane words, 1 KiB at
/// `Simd8`, under the same justification: targets, never audio, and no in-place form.
#[inline(always)]
fn settled_frames_mono<L: Lane, const DUAL_MONO: bool, const WET: bool>(
    left: &mut [f32],
    coef: &Coef<L>,
    gain: &mut L,
    invariants: &Invariants<L>,
) {
    let width = L::WIDTH;
    let mut gain_left = *gain;
    let mut targets = [L::zero(); SETTLED_CHUNK];
    for chunk in left.chunks_mut(SETTLED_CHUNK * width) {
        // Pass 1: every frame's target, from the plane alone.
        for (frame, target) in chunk.chunks_exact(width).zip(targets.iter_mut()) {
            let main = L::load(frame);
            let (detected, _) = settled_detect::<L, DUAL_MONO>(main, main, invariants);
            *target = curve_target(detected, &coef.curve, invariants);
        }
        // Pass 2: the recurrence, then the output of the same frame.
        for (frame, target) in chunk.chunks_exact_mut(width).zip(targets.iter()) {
            let main = L::load(frame);
            let smoothed = ballistic(*target, &mut gain_left, coef);
            settled_output::<L, WET>(main, smoothed, coef, invariants).store(frame);
        }
    }
    *gain = gain_left;
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
///
/// The kernel tests' whole-channel form, which their pinned digests fold. Production applies the
/// same check per lane (`finish_lanes` in `lib.rs`, issue #1090), which is this function at
/// `L = f32` and on a block where every lane fails.
#[cfg(test)]
pub(crate) fn finish_channel<L: Lane>(io: &mut [f32], channel: &mut Channel<L>) -> u32 {
    effect_runtime::bank::finish_channel::<L>(io, || channel.clear_state())
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

    /// #1278 D2a: a lane restored mid-ramp resumes both ramps -- the release time and its
    /// coefficient -- word for word as the lane it was taken from continues them.
    #[test]
    fn payload_restore_resumes_an_active_coefficient_ramp_exactly() {
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
        let remaining = source.ramps[4][0].remaining;
        assert!(remaining > 0, "the fixture restores mid-ramp");
        for update in 0..=remaining {
            assert_eq!(
                restored.words[COEF_RELEASE][0].to_bits(),
                source.words[COEF_RELEASE][0].to_bits(),
                "restored coefficient update {update}"
            );
            assert_eq!(restored.ramps[4][0], source.ramps[4][0], "update {update}");
            assert_eq!(
                restored.rate_ramps[1][0], source.rate_ramps[1][0],
                "update {update}"
            );
            source.advance_ramps(SAMPLE_RATE);
            restored.advance_ramps(SAMPLE_RATE);
        }
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
    //! Chunked bodies against the current one-frame law, without a ramping/settled block split.
    //!
    //! The reference advances ramps, reads coefficients and renders each frame before proceeding
    //! to the next. It shares the physical frame primitives; the independent f64 numeric oracles
    //! live in `tests/oracle.rs` and `tests/static_curve.rs`.
    //!
    //! The deterministic grid covers mixed and compressing track tables, every link and detector,
    //! bypass, ragged chunks, prefixes of 0, 1, 18 and 40 frames, and hostile words. The seeded
    //! differentials add extreme parameters, retargets, resets, restores and bypass toggles, dual
    //! and collapsed, at every supported width. Dispatch witnesses defend the all-wet cost path.
    //!
    //! Comparisons cover rendered words, recursive words, coefficients, parameter/rate ramp
    //! fields and boundary-check masks, before and after `finish_channel`. Only NaN payloads fold
    //! through `dsp_reference::class_a`; each NaN-producing channel must be rejected, so folding
    //! cannot hide a NaN in an accepted block.

    use super::{
        Channel, Detector, RAMPING_DUAL_BLOCKS, RAMPING_MONO_BLOCKS, RAMPING_WET_BLOCKS,
        SETTLED_WET_BLOCKS, finish_channel, process_block, process_block_mono,
    };
    use crate::design::{MAX_WIDTH, PARAMETER_COUNT};
    use crate::state::{STATE_HEADER_WORDS, commit_channel, validate_channel, write_channel};
    use core::cell::Cell;
    use dsp_reference::class_a;
    use effect_contract::LinkMode;
    use lane::{Lane, Simd4};

    type Defaults = [[f32; PARAMETER_COUNT]; MAX_WIDTH];

    const SAMPLE_RATE: u32 = 48_000;
    const QUANTUM: usize = 128;
    const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

    /// One-frame scheduling reference: settled ramps are holds, so advance them unconditionally.
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
            let width = L::WIDTH;
            let invariants = Invariants::<L>::new(link, bypass);
            for frame in 0..frames {
                channel_left.advance_ramps(sample_rate);
                channel_right.advance_ramps(sample_rate);
                let coef_left = Coef::load(&channel_left.words);
                let coef_right = Coef::load(&channel_right.words);
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
            let width = L::WIDTH;
            let invariants = Invariants::<L>::new(link, bypass);
            for frame in 0..frames {
                channel_left.advance_ramps(sample_rate);
                let coef_left = Coef::load(&channel_left.words);
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
                        // #1006: the smallest subnormal, and either side of `MIN_SOFT_KNEE_DB`,
                        // whose reciprocal decides `design_curve`'s `inv < +inf` select.
                        f32::from_bits(1),
                        f32::from_bits(0x0010_0000),
                        f32::from_bits(0x0010_0001),
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
        if let Some(index) = (0..oracle.len())
            .find(|&index| class_a::word(oracle[index]) != class_a::word(candidate[index]))
        {
            panic!(
                "{context}: state word {index} differs: oracle {:#010x}, candidate {:#010x}",
                oracle[index], candidate[index]
            );
        }
    }

    /// Asserts two rendered planes the same class-A words: equal bits, with every NaN one value
    /// (#1065). Returns how many NaN words differ in sign or payload, which a step allows only in
    /// a block the boundary check rejects.
    fn assert_words(context: &str, oracle: &[f32], candidate: &[f32]) -> usize {
        assert_eq!(oracle.len(), candidate.len());
        let mut payloads = 0;
        for (index, (a, b)) in oracle.iter().zip(candidate).enumerate() {
            assert!(
                class_a::same(*a, *b),
                "{context}: word {index} differs: oracle {:#010x}, candidate {:#010x}",
                a.to_bits(),
                b.to_bits()
            );
            payloads += usize::from(a.to_bits() != b.to_bits());
        }
        payloads
    }

    /// The all-wet arm's witness: how many settled blocks have taken it on this thread.
    fn wet_arm_blocks() -> usize {
        SETTLED_WET_BLOCKS.with(Cell::get)
    }

    /// The ramping prefix's witnesses (#1006): dual prefixes, collapsed prefixes, and prefixes of
    /// either kind that took the all-wet arm, on this thread.
    fn prefix_blocks() -> [usize; 3] {
        [
            RAMPING_DUAL_BLOCKS.with(Cell::get),
            RAMPING_MONO_BLOCKS.with(Cell::get),
            RAMPING_WET_BLOCKS.with(Cell::get),
        ]
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
        /// Ramping prefixes that ran the two-pass body (#1006), dual and collapsed, and those that
        /// took the all-wet arm.
        prefix: usize,
        prefix_mono: usize,
        prefix_wet: usize,
        /// Settled bodies of a `Silent` or `Sidechain` block (#995), and those that started
        /// mid-block.
        settled_sidechain: usize,
        settled_sidechain_mid_block: usize,
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
            self.prefix += other.prefix;
            self.prefix_mono += other.prefix_mono;
            self.prefix_wet += other.prefix_wet;
            self.settled_sidechain += other.settled_sidechain;
            self.settled_sidechain_mid_block += other.settled_sidechain_mid_block;
            self.nan_payload += other.nan_payload;
        }
    }

    fn all_wet<L: Lane>(channel: &Channel<L>) -> bool {
        (0..L::WIDTH).all(|lane| channel.words[crate::design::COEF_MIX][lane] == 1.0)
    }

    /// The ramping prefix's all-wet predicate (#1006), on the state before the block: every lane
    /// wet and no mix ramp open.
    fn wet_prefix<L: Lane>(channel: &Channel<L>) -> bool {
        all_wet(channel)
            && channel.ramps[6]
                .iter()
                .take(L::WIDTH)
                .all(|ramp| !ramp.is_ramping())
    }

    /// Asserts which ramping prefix ran, from the witnesses' movement over one block.
    fn assert_prefix(
        context: &dyn Fn() -> String,
        moved: [usize; 3],
        expected: [bool; 3],
    ) -> [usize; 3] {
        assert_eq!(
            moved,
            expected.map(usize::from),
            "{}: the two-pass prefix runs exactly on a `Main` block with an open ramp \
             [dual, collapsed, all-wet]",
            context()
        );
        moved
    }

    /// Restores one lane of `channel` through the payload codec (`validate_channel`, then
    /// `commit_channel`), with the ramp of `parameter` replaced by `(current, target, remaining)`
    /// and the step `set_target` would give it.
    fn restore_ramp<L: Lane>(
        channel: &mut Channel<L>,
        lane: usize,
        parameter: usize,
        (current, target, remaining): (f32, f32, u32),
        sample_rate: u32,
    ) {
        let mut bytes = [0_u8; STATE_HEADER_WORDS * 4];
        write_channel(&mut bytes, channel, lane);
        let word = (1 + parameter * 4) * 4;
        let step = if remaining == 0 {
            0.0_f32
        } else {
            (target - current) / remaining as f32
        };
        bytes[word..word + 4].copy_from_slice(&current.to_le_bytes());
        bytes[word + 4..word + 8].copy_from_slice(&target.to_le_bytes());
        bytes[word + 8..word + 12].copy_from_slice(&step.to_le_bytes());
        bytes[word + 12..word + 16].copy_from_slice(&remaining.to_le_bytes());
        validate_channel(&bytes).expect("a legal payload");
        commit_channel(&bytes, channel, lane, sample_rate);
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

        /// Restores one lane of one channel, both sides, with one ramp rewritten.
        fn restore(&mut self, right: bool, lane: usize, parameter: usize, ramp: (f32, f32, u32)) {
            let rate = self.sample_rate;
            for pair in [&mut self.oracle, &mut self.candidate] {
                let channel = if right { &mut pair.1 } else { &mut pair.0 };
                restore_ramp(channel, lane, parameter, ramp, rate);
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
            let prefix_expected = ramping > 0 && source == Source::Main;
            let prefix_wet_expected = prefix_expected
                && !bypass
                && wet_prefix(&self.oracle.0)
                && wet_prefix(&self.oracle.1);
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
            let prefix_before = prefix_blocks();
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
            let prefix_after = prefix_blocks();
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
            let prefix = assert_prefix(
                &context,
                core::array::from_fn(|kind| prefix_after[kind] - prefix_before[kind]),
                [prefix_expected, false, prefix_wet_expected],
            );
            let relaxed_left = assert_words(
                &format!("{} left kernel", context()),
                &oracle_left,
                &candidate_left,
            );
            let relaxed_right = assert_words(
                &format!("{} right kernel", context()),
                &oracle_right,
                &candidate_right,
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
                prefix: prefix[0],
                prefix_mono: 0,
                prefix_wet: prefix[2],
                settled_sidechain: usize::from(settled && source != Source::Main),
                settled_sidechain_mid_block: usize::from(
                    settled && ramping > 0 && source != Source::Main,
                ),
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

        fn restore(&mut self, lane: usize, parameter: usize, ramp: (f32, f32, u32)) {
            let rate = self.sample_rate;
            restore_ramp(&mut self.oracle, lane, parameter, ramp, rate);
            restore_ramp(&mut self.candidate, lane, parameter, ramp, rate);
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
            let prefix_expected = ramping > 0 && source == Source::Main;
            let prefix_wet_expected = prefix_expected && !bypass && wet_prefix(&self.oracle);
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
            let prefix_before = prefix_blocks();
            process_block_mono::<L>(
                &mut candidate,
                detector,
                frames,
                link,
                bypass,
                self.sample_rate,
                &mut self.candidate,
            );
            let wet_arm = wet_arm_blocks() - wet_before;
            let prefix_after = prefix_blocks();
            let settled = ramping < frames;
            let all_wet_settled =
                settled && source == Source::Main && !bypass && all_wet(&self.oracle);
            let context = || format!("{} (frames {frames}, ramping {ramping})", context());
            assert_eq!(
                wet_arm,
                usize::from(all_wet_settled),
                "{}: the collapsed all-wet arm runs exactly once on an all-wet, unbypassed settled block",
                context()
            );
            let prefix = assert_prefix(
                &context,
                core::array::from_fn(|kind| prefix_after[kind] - prefix_before[kind]),
                [false, prefix_expected, prefix_wet_expected],
            );
            let relaxed = assert_words(&format!("{} kernel", context()), &oracle, &candidate);
            assert_state(&context(), &self.oracle, &self.candidate);
            let oracle_mask = finish_channel::<L>(&mut oracle, &mut self.oracle);
            let candidate_mask = finish_channel::<L>(&mut candidate, &mut self.candidate);
            assert_eq!(oracle_mask, candidate_mask, "{}: finish mask", context());
            assert!(
                relaxed == 0 || oracle_mask != 0,
                "{}: a NaN payload differs in an accepted block",
                context()
            );
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
                prefix: 0,
                prefix_mono: prefix[1],
                prefix_wet: prefix[2],
                settled_sidechain: 0,
                settled_sidechain_mid_block: 0,
                nan_payload: relaxed,
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
        lane::each_vector_lane!(|L, N| coverage.add(&grid::<L>(&format!("Simd{N}"), table, wet)));
        println!("grid coverage {coverage:?}");
        assert!(
            coverage.settled_mid_block > 0,
            "the grid must start settled bodies mid-block"
        );
        assert!(
            coverage.settled_sidechain_mid_block > 0,
            "the grid must start sidechained settled bodies mid-block (#995)"
        );
        assert!(
            coverage.prefix > 0 && coverage.prefix_mono > 0,
            "the grid must run the two-pass prefix, dual and collapsed (#1006)"
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
    fn the_chunked_bodies_match_the_frame_law_on_the_corpus_table() {
        grid_all(&CORPUS_TRACKS, false);
    }

    /// A heavily compressing all-wet table: low thresholds, high ratios, fast attacks, makeup.
    const COMPRESSING_TRACKS: [[f32; PARAMETER_COUNT]; 8] = [
        [-40.0, 20.0, 0.0, 0.1, 5.0, 12.0, 1.0],
        [-36.0, 16.0, 6.0, 0.5, 20.0, 9.0, 1.0],
        [-32.0, 12.0, 12.0, 1.0, 50.0, 6.0, 1.0],
        [-44.0, 10.0, 3.0, 0.2, 10.0, 15.0, 1.0],
        [-50.0, 8.0, 24.0, 2.0, 80.0, 18.0, 1.0],
        [-30.0, 20.0, 1.0e-3, 0.3, 5.0, 24.0, 1.0],
        [-60.0, 6.0, 9.0, 5.0, 200.0, 20.0, 1.0],
        [-80.0, 4.0, 18.0, 10.0, 500.0, 24.0, 1.0],
    ];

    /// The all-wet compressing set exercises both dual and collapsed chunked bodies.
    #[test]
    fn the_chunked_bodies_match_the_frame_law_on_the_compressing_table() {
        let coverage = grid_all(&COMPRESSING_TRACKS, true);
        assert!(
            coverage.all_wet_settled > 0,
            "the compressing set must take the arm"
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
        lane::each_lane!(|L| wet_arm_witness::<L>());
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
                    // One channel, or both with the same value in the same block: the web host's
                    // `channel = 2` traffic (#1006).
                    match rng.below(3) {
                        0 => dual.retarget(true, parameter, lane, value),
                        1 => {
                            dual.retarget(false, parameter, lane, value);
                            mono.retarget(parameter, lane, value);
                        }
                        _ => {
                            dual.retarget(false, parameter, lane, value);
                            dual.retarget(true, parameter, lane, value);
                            mono.retarget(parameter, lane, value);
                        }
                    }
                }
            }
            if rng.chance(0.03) {
                // A payload restore that leaves a ramp at `remaining = 0` with `current !=
                // target` (or, one time in four, mid-window), through the restore path (#1006).
                let lane = rng.below(width);
                let parameter = rng.below(PARAMETER_COUNT);
                let current = self::parameter(&mut rng, parameter);
                let target = self::parameter(&mut rng, parameter);
                let remaining = if rng.chance(0.75) {
                    0
                } else {
                    1 + rng.below(64) as u32
                };
                dual.restore(
                    rng.chance(0.5),
                    lane,
                    parameter,
                    (current, target, remaining),
                );
                mono.restore(lane, parameter, (current, target, remaining));
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
        // Issue #1051's seed discipline: `MISO_ENGINE_RANDOMIZED_SCALE` multiplies the debug share
        // (the nightly job), `MISO_ENGINE_RANDOMIZED_SEED` replays one seed.
        let per_pr = if dsp_reference::randomized::overridden() {
            10
        } else {
            seeds()
        };
        dsp_reference::randomized::run_seeds(
            &format!("randomized_differential W{}", L::WIDTH),
            "cargo test -p compressor --lib -- settled_body_tests::randomized_differential",
            per_pr,
            |seed| coverage.add(&randomized::<L>(seed, 128)),
        );
        println!("W{} randomized coverage {coverage:?}", L::WIDTH);
        if dsp_reference::randomized::replaying() {
            return;
        }
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
        assert!(
            coverage.settled_sidechain_mid_block > 0,
            "sidechained settled bodies must start mid-block (#995)"
        );
        assert!(
            coverage.prefix > 0 && coverage.prefix_mono > 0 && coverage.prefix_wet > 0,
            "the two-pass prefix must run dual, collapsed and on the all-wet arm (#1006)"
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

    /// The 8-lane (AVX2) twin of `randomized_differential_simd4` (#1112).
    #[cfg(target_feature = "avx2")]
    #[test]
    fn randomized_differential_simd8() {
        randomized_width::<lane::Simd8>();
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
}
