//! The generic block kernel: one body, instantiated at `f32`, `Simd4` and `Simd8`.
//!
//! Master plan #83 D10. `L = f32` *is* the scalar path — there is no second implementation, which
//! is what makes cross-backend bit identity a property of the code rather than a test result
//! (gate E2). Every arithmetic operation is a [`Lane`] method, a `math` lane function
//! or an integer compare-select, so the same bits come out of every target (D5, gate E4).
//!
//! # What is not here any more
//!
//! * No libm. `log10`/`powf`/`exp` became `dynamics::level_db` / `dynamics::gain_from_db` on the
//!   lane `exp2`/`log2` of `math`, and the ballistic coefficients are designed off the
//!   render path in `crate::design` (D6).
//! * No `%`. The ring wrap is `if next == B { 0 }`, and the detector tap is
//!   `if w >= D { w - D } else { w + B - D }` — integer compare-select (F2).
//! * No per-value finiteness check, no `sanitize`, no `recover`. `flush` is applied to `g`, the
//!   one recursive word, and the block boundary is checked once by
//!   `effect_runtime::bank` (D7, master plan §4.4).
//! * No `PreparedCompressorGainMixKernel`. The gain/mix step is the `gain_mix_block` form
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
//!
//! The idle remainder is then staged: `idle_frames_staged` visits it three times, once for the
//! frame-independent work before the ballistic recurrence, once for the recurrence, once for the
//! frame-independent work after it, with the detector taps of the whole segment pre-gathered.
//! That is legal only when no lane taps a row the segment writes first, so `frames_loop` remains
//! the fallback and the general body; both are exercised.

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

    /// Copies every word this channel carries **except** its preparation defaults from `source`.
    ///
    /// The mono collapse's disengage boundary (`PreparedNativeEffectBank::desymmetrize_channels`).
    /// A collapsed block runs `process_block_mono`, which advances the left channel's ramps,
    /// redesigns its coefficient words from them, writes its two rings and moves its cursor and
    /// its one recursive word -- and touches none of the right channel's. This restores the right
    /// channel to what the counterfactual dual run would hold, which by the witness' induction is
    /// exactly the left channel's.
    ///
    /// # The copy list, term by term
    ///
    /// | word | why it is here |
    /// |---|---|
    /// | `words` | the eight designed coefficients the kernel loads. A ramp in flight rewrites them every frame, and only the left channel's were rewritten. |
    /// | `ramps` | all four fields of every smoothed parameter. `max_remaining` sizes the ramping prefix and `advance_ramps` moves `current` toward `target` by `step`; a right channel left mid-ramp would resume from a stale position. |
    /// | `lookahead_ms`, `delay` | the detector read-back distance and what it was derived from. `redesign` reads the former, `gather_detector`/`fill_taps`/`min_delay` the latter. |
    /// | `gain_reduction_db` | the one recursive word -- the compressor's whole cross-frame dependency. |
    /// | `cursor` | the shared ring write index. It advances once per frame on the collapsed channel only. |
    /// | `main`, `detector` | the two delay rings, whole. A partial copy would read the collapsed run's samples at some taps and the pre-collapse ones at others. |
    ///
    /// `defaults` is deliberately **not** copied: it is the control-plane reset table, no kernel
    /// reads it, and the counterfactual dual run never wrote it either. `ring_length` is a
    /// preparation shape both channels share.
    /// # Which entries are individually gated, and which are here by the rule
    ///
    /// `crates/compressor/tests/mono_collapse.rs` fails if any of `words`, `ramps`,
    /// `gain_reduction_db`, `cursor`, `main` or `detector` is dropped. `lookahead_ms` and `delay`
    /// are **not** individually gated and are on the list anyway: no rendered block writes them --
    /// they move only at prepare, restore and a full reset, none of which is reachable on a bound
    /// bank -- so a test cannot make them diverge today. They are copied because the rule is
    /// *whole per-channel state*, and a rule that admits exceptions for "the fields nothing moves
    /// yet" is a rule that fails the first time something moves one.
    pub(crate) fn copy_state_from(&mut self, source: &Self) {
        debug_assert_eq!(self.ring_length, source.ring_length);
        debug_assert_eq!(self.main.len(), source.main.len());
        self.words = source.words;
        self.ramps = source.ramps;
        self.lookahead_ms = source.lookahead_ms;
        self.delay = source.delay;
        self.gain_reduction_db = source.gain_reduction_db;
        self.cursor = source.cursor;
        self.main.copy_from_slice(&source.main);
        self.detector.copy_from_slice(&source.detector);
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

    /// The bit pattern of the one recursive word, for an exact before/after comparison.
    ///
    /// Bits, not floats, for the reason #163 phase 4 item 1 needs everywhere: the fast path
    /// promises not to move a bit, and `-0.0 == 0.0` would let a word that crossed between the
    /// two zeros be called unchanged.
    pub(crate) fn recursive_bits(&self) -> [u32; MAX_WIDTH] {
        let mut words = [0_u32; MAX_WIDTH];
        self.gain_reduction_db.store_bits(&mut words[..L::WIDTH]);
        words
    }

    /// `true` when both delay rings are entirely `+0.0`.
    ///
    /// This is what makes skipping the ring writes sound: a ring of exact `+0.0` reads back the
    /// same silence from any cursor position, so a block that writes only zeros into it leaves it
    /// bit-identical whatever the cursor did.
    pub(crate) fn rings_are_positive_zero(&self) -> bool {
        effect_runtime::bank::block_is_positive_zero(&self.main)
            && effect_runtime::bank::block_is_positive_zero(&self.detector)
    }

    /// Advance the shared write cursor by a whole block, as `frames` per-sample steps would.
    pub(crate) fn advance_cursor(&mut self, frames: u32) {
        if self.ring_length == 0 {
            return;
        }
        self.cursor = (self.cursor + frames % self.ring_length) % self.ring_length;
    }

    /// Highest number of samples any ramp of this channel still has to produce.
    pub(crate) fn max_remaining(&self) -> u32 {
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
    ///
    /// # The idle-lane guard
    ///
    /// A ramping *block* is not a ramping *lane*: `process_block` cuts the prefix from the longest
    /// ramp anywhere in either channel, so one automated track drags every lane of its bank and
    /// every unmoved parameter of that track through the prefix. The guard is sound because
    /// [`LinearRamp::next_value`] on a finished ramp (`remaining == 0`) returns `current` and
    /// mutates nothing at all — calling it is the identity, so not calling it is too. The lane
    /// scan reads `remaining` seven times and, when nothing is in flight, does no more; a lane
    /// that is ramping still takes `current` for its resting parameters, which is the same `f32`
    /// bit pattern `next_value` would have handed back.
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
    /// `mix == 1`: step 8's wet-identity select mask.
    wet_identity: L::Mask,
    /// `mix == 0`: the half of step 8's dry identity that the smoother cannot influence.
    dry_mix_zero: L::Mask,
    /// `makeup == 0`: the other half of the unity-gain dry identity.
    makeup_zero: L::Mask,
}

impl<L: Lane> Coef<L> {
    /// Loads the lane vectors *and* the three step-8 identity masks that are functions of them.
    ///
    /// The masks belong here rather than in [`one_frame`] because they are functions of
    /// coefficient words alone, and a coefficient word cannot change inside the idle body — that
    /// body loads `Coef` once and never redesigns (see the module documentation). The ramping body
    /// reloads `Coef` every frame, after `advance_ramps`, so a mask is always as fresh there as the
    /// words it came from and the two bodies still agree bit for bit. Only `smoothed == 0`, which
    /// is a function of the recursive word, stays per frame.
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
    staged: &mut Staged<L>,
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
        // The idle remainder takes the staged body when its detector taps can be pre-gathered,
        // and the general per-frame body when they cannot. The two are bit-identical; which one
        // runs is a property of the lanes' lookahead alone (see `idle_frames_staged`).
        if segment_is_stageable(channel_left, channel_right, frames - ramping) {
            let Staged {
                taps_left,
                taps_right,
                target_left,
                target_right,
            } = staged;
            idle_frames_staged::<L>(
                left,
                right,
                detector,
                ramping,
                frames,
                link,
                bypass,
                channel_left,
                channel_right,
                taps_left,
                taps_right,
                target_left,
                target_right,
            );
        } else {
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
}

/// The frame body. `RAMPING` is a compile-time switch, not a branch.
///
/// Frozen operation order, per frame — moving any line moves bits:
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
///
/// This is the general body: it is the only one that may ramp, and it is the fallback whenever
/// [`segment_is_stageable`] says an idle segment cannot be pre-gathered. Steps 4 to 8 are the
/// shared [`curve_target`], [`ballistic`] and [`gain_mix`], so the staged body below is the same
/// arithmetic in a different order of visits, not a second transcription of it.
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
    let invariants = Invariants::<L>::new(link, bypass);

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

        // 1 and 2. main input, detector source, magnitudes and link.
        let main_left = L::load(&left[slot..]);
        let main_right = L::load(&right[slot..]);
        let (level_left, level_right) =
            link_frame(detector, slot, main_left, main_right, &invariants);

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
            &invariants,
        );
        let output_right = one_frame(
            delayed_right,
            detected_right,
            &coef_right,
            &mut channel_right.gain_reduction_db,
            &invariants,
        );
        output_left.store(&mut left[slot..]);
        output_right.store(&mut right[slot..]);

        // 9.
        channel_left.cursor = next as u32;
        channel_right.cursor = next as u32;
    }
}

/// Longest idle segment [`idle_frames_staged`] will take in one visit.
///
/// This is the bound on the staged body's scratch, which lives in [`Staged`] and is allocated once
/// at preparation: `2 * 128 * W` floats of taps plus `2 * 128` lane vectors of curve targets,
/// 16 KiB at `W = 8` and 2.5 KiB at `W = 1`. Nothing is allocated on the render path (AGENTS.md,
/// allocation-free render).
///
/// The contract sets no ceiling on the prepared quantum, but 128 frames is the render quantum
/// every launch host actually carries — the Web Audio render quantum, the console fixtures and the
/// benchmark subjects — so at launch this is the whole idle remainder of a block and not a strip
/// of one. A longer segment is not wrong, it simply takes [`frames_loop`]. Cutting a long segment
/// into strips to keep it staged is a real option and deliberately not taken: at 128 frames the
/// scratch already sits in L1, and strip-mining this body to 16, 32 or 64 frames measured slower
/// on the kernel example at every one of the three sizes.
const MAX_STAGED_FRAMES: usize = 128;

/// The staged idle body's scratch, allocated once at preparation and reused by every block.
///
/// # Why this is not on the stack
///
/// It was, as four `[_; MAX_STAGED_FRAMES * MAX_WIDTH]` locals of [`idle_frames_staged`]. Rust has
/// no safe way to name an uninitialised local, so those four arrays were zeroed on entry — four
/// `memset` calls of 4 KiB each, once per bank per block, and every byte of them dead: `fill_taps`
/// writes every slot of `taps[..len * W]` before the first read, and pass A writes `target[..len]`
/// before passes B and C read it. The workspace denies `unsafe_code`, so `MaybeUninit` is not on
/// the table; a buffer that is zeroed once at preparation and then only ever overwritten is the
/// safe way to spend the zeroing once instead of once per block.
///
/// # Why this is not state
///
/// Nothing here survives a call. Every read of `taps` and `target` in [`idle_frames_staged`] is
/// preceded, within the same call, by a write of the same slot; the buffers carry nothing from one
/// block to the next, and reading them at any other moment observes only whatever the last staged
/// segment happened to leave behind. So they are deliberately **not** part of [`Channel`]: the
/// V1 payload layout in `state.rs` is a contract fixture of `24 + 2B` words per lane, and
/// `snapshot_lane` and `commit_channel` name every one of `Channel`'s fields explicitly, so a
/// fifth buffer on `Channel` would be a field the payload silently does not carry. They hang off
/// `Instance` beside the two channels instead, where no snapshot, restore, `maximum_state` or
/// reset path can reach them — `clear_state` has nothing to clear here, because there is nothing
/// here a later block can observe.
///
/// # Size
///
/// `MAX_STAGED_FRAMES` is the kernel's own segment bound (see above), not a track count, and the
/// width comes from `L::WIDTH` at preparation — there is no compiled maximum track count here.
pub(crate) struct Staged<L: Lane> {
    /// Left detector taps of one segment, frame major, `MAX_STAGED_FRAMES * L::WIDTH` samples.
    taps_left: Box<[f32]>,
    /// Right detector taps, same shape.
    taps_right: Box<[f32]>,
    /// Left curve target per frame, then the smoothed reduction pass B overwrites it with.
    target_left: Box<[L]>,
    /// Right curve target, same shape.
    target_right: Box<[L]>,
}

impl<L: Lane> Staged<L> {
    /// Allocates and zeroes the four buffers. Preparation only; never called on the render path.
    pub(crate) fn new() -> Self {
        let samples = MAX_STAGED_FRAMES * L::WIDTH;
        Self {
            taps_left: vec![0.0; samples].into_boxed_slice(),
            taps_right: vec![0.0; samples].into_boxed_slice(),
            target_left: vec![L::zero(); MAX_STAGED_FRAMES].into_boxed_slice(),
            target_right: vec![L::zero(); MAX_STAGED_FRAMES].into_boxed_slice(),
        }
    }
}

impl<L: Lane> Default for Staged<L> {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether the staged idle body may take a segment of `len` frames.
///
/// The one legality condition, derived in [`idle_frames_staged`]: every live lane of both channels
/// must tap a detector row that this segment does not write first, which is `D >= len`.
fn segment_is_stageable<L: Lane>(
    channel_left: &Channel<L>,
    channel_right: &Channel<L>,
    len: usize,
) -> bool {
    len <= MAX_STAGED_FRAMES && min_delay(channel_left) >= len && min_delay(channel_right) >= len
}

/// The idle body as three passes over the segment, with the detector taps pre-gathered.
///
/// # Why the passes
///
/// Step 6 is the compressor's only cross-frame dependency: one lane vector, `g`, carried from
/// frame to frame through a select, an `fma` and a `flush`. Everything before it (steps 1 to 5,
/// which is where the work is — `fast_level_db` and `gain_delta_db` are both several-term
/// polynomial evaluations) and everything after it (steps 7 and 8) depend only on the frame they
/// are in. Visiting the segment once per stage lets consecutive frames' polynomial chains overlap
/// instead of each one waiting behind the previous frame's recurrence, and leaves a pass B whose
/// serial chain is the ~7 operations it actually is.
///
/// Per lane and per sample the operation order is untouched: pass A does steps 1 to 5 of frame `i`
/// in the same order the per-frame body does, pass B does step 6 of frame `i` after step 6 of
/// frame `i - 1`, and pass C does steps 7 and 8. Only the *interleaving across frames* moves, and
/// no lane's arithmetic sees another frame's. That is the same argument class as #163 phase 3's
/// SVF interleave: a reordering of independent frames is bit-exact, a reordering of a dependent
/// chain is not, and the recurrence is kept in its own strictly sequential pass precisely so that
/// it is never the thing being reordered.
///
/// # Why pre-gathering the taps is legal
///
/// Write `B` for the ring length, `w0` for the cursor at the head of the segment and `D_k` for
/// lane `k`'s read-back distance. Frame `i` of the segment writes detector row
/// `w_i = (w0 + i) mod B` and reads row `t_i(k) = (w0 + i - D_k) mod B`.
///
/// Gathering every `t_i(k)` before the segment's first write returns a different value from the
/// per-frame body exactly when some earlier frame `j < i` of this same segment overwrote that row,
/// i.e. when `t_i(k) == w_j`, i.e. when `D_k ≡ i - j (mod B)` for some `1 <= i - j <= len - 1`.
/// Because `0 <= D_k <= N = B - 1`, that congruence is plain equality, so the gather is
/// bit-identical iff no lane has `D_k` in `[1, len - 1]`. `D_k == 0` is not safe either — the tap
/// would then be the row the frame itself has just written — so the condition is `D_k >= len` for
/// every live lane of both channels, which is what [`segment_is_stageable`] tests.
///
/// `D = N - L` with `N = Fs/50` (the 20 ms fixed latency) and `L` the lookahead in samples, so the
/// guard fails only for a lane whose lookahead is within `len` samples of the 20 ms maximum — over
/// 17.3 ms at 48 kHz for a full 128-frame segment. Those blocks take [`frames_loop`], which is why
/// the per-frame body stays a live, exercised path and not dead code.
///
/// The delayed main output needs no such guard. Frame `i` reads main row `(w0 + i + 1) mod B`
/// after writing row `w_i`; that row is written by frame `i + 1`, never by an earlier one, so it
/// is staged forward during pass A rather than gathered up front. It is staged into `left`/`right`
/// themselves: pass A has already consumed `left[slot]` (it is the frame's main input) and pass C
/// is going to overwrite it, so the block buffer is exactly the right-sized scratch for it and
/// carries no stack.
///
/// Not `#[inline(always)]`: the segment is visited once per block, and a separate frame keeps the
/// body's bookkeeping off the stack of blocks that take the per-frame body.
///
/// # Why the scratch arrives as four parameters and not as one `&mut Staged`
///
/// The four buffers used to be four locals of this function, so the optimiser knew they were four
/// distinct allocas and could not alias. They now live in [`Staged`], where they are four `Box`
/// pointers *inside* one struct: `&mut Staged` is `noalias`, but that covers the struct, not the
/// four heap blocks it points at, so a single-parameter form leaves the three passes below
/// reloading `taps` after every `target` store. Passed one by one they are four `noalias`
/// reference parameters again. Measured on `examples/lane_sample_timing`, both forms otherwise
/// identical: 7.03 ns/lane-sample here against 8.01 for the single-parameter form on the scalar
/// instantiation, and no separation at all at `W = 8` (1.293 against 1.298), which is what a
/// per-frame dependency stall looks like when it is amortised over eight lanes.
#[allow(clippy::too_many_arguments)]
fn idle_frames_staged<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    detector: Detector<'_>,
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
    channel_right: &mut Channel<L>,
    taps_left: &mut [f32],
    taps_right: &mut [f32],
    target_left: &mut [L],
    target_right: &mut [L],
) {
    let width = L::WIDTH;
    let ring_length = channel_left.ring_length as usize;
    let len = end - start;
    debug_assert!(segment_is_stageable(channel_left, channel_right, len));
    let invariants = Invariants::<L>::new(link, bypass);
    let coef_left = Coef::load(&channel_left.words);
    let coef_right = Coef::load(&channel_right.words);

    // The taps of the whole segment, frame major, read before the segment's first ring write.
    //
    // The left channel's cursor is the shared write index for both channels, exactly as it is in
    // the per-frame body: a rejected block clears one channel's state and not necessarily the
    // other's, so the two cursors can legitimately disagree on entry, and the frozen behaviour is
    // that the left one wins and both are left equal on exit.
    //
    // The scratch is `staged`, allocated at preparation: `fill_taps` writes every slot of
    // `taps[..len * width]` it is about to be read at, so what the buffer held on entry — the
    // previous block's segment — is never observed.
    let head = channel_left.cursor as usize;
    fill_taps(channel_left, head, len, taps_left);
    fill_taps(channel_right, head, len, taps_right);

    // Pass A: steps 1 to 5, plus the cursor. `target` holds the static-curve target of every
    // frame, which pass B then overwrites in place with the smoothed reduction it produces.
    //
    // Walked as an iterator rather than indexed by `index`, for the reason the stack arrays did
    // not need: their length was a compile-time constant, so `taps[index * width..]` and
    // `target[index]` carried no bounds check. A prepared buffer's length is not, and four checks
    // per frame cost 12% of the scalar instantiation and 2% at `W = 8` on the kernel example.
    // `chunks_exact(width)` also hands `L::load` a slice whose length the optimiser knows is
    // exactly `L::WIDTH`, which is the one thing that check was ever for.
    let mut write = head;
    for (index, (((tap_left, tap_right), frame_left), frame_right)) in taps_left[..len * width]
        .chunks_exact(width)
        .zip(taps_right[..len * width].chunks_exact(width))
        .zip(target_left[..len].iter_mut())
        .zip(target_right[..len].iter_mut())
        .enumerate()
    {
        let slot = (start + index) * width;
        let main_left = L::load(&left[slot..]);
        let main_right = L::load(&right[slot..]);
        let (level_left, level_right) =
            link_frame(detector, slot, main_left, main_right, &invariants);
        let next = write + 1;
        let next = if next == ring_length { 0 } else { next };
        main_left.store(&mut channel_left.main[write * width..]);
        level_left.store(&mut channel_left.detector[write * width..]);
        main_right.store(&mut channel_right.main[write * width..]);
        level_right.store(&mut channel_right.detector[write * width..]);
        L::load(&channel_left.main[next * width..]).store(&mut left[slot..]);
        L::load(&channel_right.main[next * width..]).store(&mut right[slot..]);
        *frame_left = curve_target(L::load(tap_left), &coef_left, &invariants);
        *frame_right = curve_target(L::load(tap_right), &coef_right, &invariants);
        write = next;
    }
    channel_left.cursor = write as u32;
    channel_right.cursor = write as u32;

    // Pass B: step 6, the one serial recurrence, one per channel.
    let mut gain_left = channel_left.gain_reduction_db;
    let mut gain_right = channel_right.gain_reduction_db;
    for (frame_left, frame_right) in target_left[..len]
        .iter_mut()
        .zip(target_right[..len].iter_mut())
    {
        *frame_left = ballistic(*frame_left, &mut gain_left, &coef_left);
        *frame_right = ballistic(*frame_right, &mut gain_right, &coef_right);
    }
    channel_left.gain_reduction_db = gain_left;
    channel_right.gain_reduction_db = gain_right;

    // Pass C: steps 7 and 8, over the delayed output pass A left in the block buffers.
    for (index, (smoothed_left, smoothed_right)) in target_left[..len]
        .iter()
        .zip(&target_right[..len])
        .enumerate()
    {
        let slot = (start + index) * width;
        let delayed_left = L::load(&left[slot..]);
        gain_mix(delayed_left, *smoothed_left, &coef_left, &invariants).store(&mut left[slot..]);
        let delayed_right = L::load(&right[slot..]);
        gain_mix(delayed_right, *smoothed_right, &coef_right, &invariants)
            .store(&mut right[slot..]);
    }
}

/// The smallest detector read-back distance over a channel's live lanes.
#[inline(always)]
fn min_delay<L: Lane>(channel: &Channel<L>) -> usize {
    let mut least = u32::MAX;
    for lane in 0..L::WIDTH {
        if channel.delay[lane] < least {
            least = channel.delay[lane];
        }
    }
    least as usize
}

/// Copies `len` frames of every live lane's detector tap into `scratch`, frame major.
///
/// The tap row of lane `k` advances one row per frame from `(write - D_k) mod B`, so the whole
/// segment is two contiguous runs of the ring per lane — the part before the wrap and the part
/// after it. Written that way the bounds checks leave the inner loop, `delay[k]` is read once
/// instead of once per frame, and the ring is walked forwards, which is what the per-access
/// compare-select in [`gather_detector`] cannot offer.
#[inline]
fn fill_taps<L: Lane>(channel: &Channel<L>, write: usize, len: usize, scratch: &mut [f32]) {
    let width = L::WIDTH;
    let ring_length = channel.ring_length as usize;
    let scratch = &mut scratch[..len * width];
    for lane in 0..width {
        let delay = channel.delay[lane] as usize;
        let row = if write >= delay {
            write - delay
        } else {
            write + ring_length - delay
        };
        let first = (ring_length - row).min(len);
        copy_lane(
            &channel.detector[row * width..(row + first) * width],
            &mut scratch[..first * width],
            lane,
            width,
        );
        copy_lane(
            &channel.detector[..(len - first) * width],
            &mut scratch[first * width..],
            lane,
            width,
        );
    }
}

/// Copies lane `lane` of every frame of `source` into lane `lane` of every frame of `destination`.
#[inline(always)]
fn copy_lane(source: &[f32], destination: &mut [f32], lane: usize, width: usize) {
    for (to, from) in destination
        .chunks_exact_mut(width)
        .zip(source.chunks_exact(width))
    {
        to[lane] = from[lane];
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

/// The lane vectors and masks that are constant for a whole block body.
///
/// Both bodies need exactly these, and building them in one place is what keeps the staged body
/// and the per-frame body the same law rather than two transcriptions of it. Nothing here is a
/// function of a coefficient word, so unlike [`Coef`] this survives a ramp untouched.
struct Invariants<L: Lane> {
    /// `+0.0`, the step 5 ceiling and the step 8 identity comparand.
    zero: L,
    /// The `0.5` of the average link.
    half: L,
    /// Step 4's `1e-8` detector floor.
    level_floor: L,
    /// Lowest level in dB the static curve sees.
    level_min: L,
    /// Highest level in dB the static curve sees.
    level_max: L,
    /// Most gain reduction the smoother may be asked to track, in dB.
    reduction_min: L,
    /// The link mode is not `DualMono`: both channels see the combined detector.
    linked: L::Mask,
    /// The link mode is `Average`: the combined detector is the mean rather than the maximum.
    averaged: L::Mask,
    /// The block's bypass flag, as a mask.
    bypassed: L::Mask,
}

impl<L: Lane> Invariants<L> {
    /// Splats the constants and turns the block's two loop-invariant choices into masks.
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

/// Steps 1 and 2 for one frame: the detector source, its magnitudes, and the link.
///
/// Returns the two detector levels that go into the rings. `main_left` and `main_right` are passed
/// in rather than loaded here because the caller needs them for the ring write anyway, and
/// `Detector::Main` must see the very same vectors.
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
        Detector::Sidechain(sidechain_left, sidechain_right) => (
            L::load(&sidechain_left[slot..]),
            L::load(&sidechain_right[slot..]),
        ),
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

/// Steps 4 to 8 for one channel of one frame.
#[inline(always)]
fn one_frame<L: Lane>(
    delayed: L,
    detected: L,
    coef: &Coef<L>,
    gain_reduction_db: &mut L,
    invariants: &Invariants<L>,
) -> L {
    let target = curve_target(detected, coef, invariants);
    let smoothed = ballistic(target, gain_reduction_db, coef);
    gain_mix(delayed, smoothed, coef, invariants)
}

/// Steps 4 and 5: the detector amplitude through the level and the static curve.
///
/// Frame independent — this is the pass A half of the staged body, and the expensive half: two
/// polynomial evaluations whose chains are what stall behind step 6 when the body is per frame.
#[inline(always)]
fn curve_target<L: Lane>(detected: L, coef: &Coef<L>, invariants: &Invariants<L>) -> L {
    // 4. amplitude to level, floored and clamped into the curve's domain.
    //
    // FAST-DB-CROSSING X1: the compressor's detector level. This is a dynamics gain path -- the
    // result is a detector reading that feeds the static curve and is never pinned as a
    // coefficient word -- so it takes the sealed fast tier. Bounded at 2.810e-5 dB, 1.83x the
    // exact tier, by gate F1 in `math`.
    let floored = detected.max(invariants.level_floor);
    let level = fast_level_db(floored)
        .max(invariants.level_min)
        .min(invariants.level_max);

    // 5. the static curve, as the reduction it applies.
    gain_delta_db(level, &coef.curve)
        .max(invariants.reduction_min)
        .min(invariants.zero)
}

/// Step 6: the branching one-pole, and the only `flush` in the crate.
///
/// The recurrence itself is `envelope::rms_follow` — the runtime's frozen one-rounding form
/// `fma(c, target - y, y)` on a *rate* coefficient, which is the general one-pole and not
/// specific to a mean square (its own documentation says the squaring and the square root
/// belong to the caller). Nothing is added to the runtime for this: what the compressor
/// contributes is the **branch**, and a branch is a `Lane::select`, not a new primitive.
///
/// Frozen: the select is strict (`target < y`), so equality takes the release coefficient —
/// BRIEFS/013's rule, and the sign convention that makes falling gain reduction the attack.
/// `peak_follow` is the wrong sibling here: its attack is an unconditional `max`, which is a
/// limiter's ballistic, not a compressor's (GMR 2012 section 4.2, "smooth branching").
///
/// This is the compressor's whole cross-frame dependency, which is why the staged body gives it a
/// pass of its own instead of letting it sit in the middle of the frame's other twenty-odd
/// operations.
#[inline(always)]
fn ballistic<L: Lane>(target: L, gain_reduction_db: &mut L, coef: &Coef<L>) -> L {
    let coefficient = L::select(target.lt(*gain_reduction_db), coef.attack, coef.release);
    let smoothed = flush(rms_follow(target, *gain_reduction_db, coefficient));
    *gain_reduction_db = smoothed;
    smoothed
}

/// Steps 7 and 8: the smoothed reduction as an amplitude, applied and mixed.
///
/// Frame independent again — the pass C half of the staged body.
#[inline(always)]
fn gain_mix<L: Lane>(delayed: L, smoothed: L, coef: &Coef<L>, invariants: &Invariants<L>) -> L {
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
    //
    // Six of the nine mask words step 8 used to build per frame were functions of `mix` and
    // `makeup` only. They are now built once, in `Coef::load` — the same masks, from the same
    // words, in the same order, so this is a hoist and not a rewrite of the identity law.
    let wet = delayed.mul(gain);
    let mixed = gain_mix_step(delayed, gain, coef.mix);
    let dry_identity = L::mask_or(
        invariants.bypassed,
        L::mask_or(
            coef.dry_mix_zero,
            L::mask_and(smoothed.eq(invariants.zero), coef.makeup_zero),
        ),
    );
    let output = L::select(coef.wet_identity, wet, mixed);
    L::select(dry_identity, delayed, output)
}

/// The master plan section 4.4 boundary check for one channel of one block.
///
/// Returns the bitmask of lanes that were out of bounds, `0` when the block is clean, and on
/// rejection zeroes the channel's block and clears its recursive state. The policy itself is
/// `effect_runtime::bank::finish_channel`; what belongs to this crate is only *which*
/// state a rejected block resets.
pub(crate) fn finish_channel<L: Lane>(io: &mut [f32], channel: &mut Channel<L>) -> u32 {
    bank::finish_channel::<L>(io, || channel.clear_state())
}

// ---------------------------------------------------------------------------------------------
// The mono-collapse one-plane bodies.
//
// Each is the dual body with the right channel's arguments deleted and every remaining line left
// where it was. Two things are **not** deletions and are the whole reason these are separate
// bodies rather than a `right = left` alias:
//
// * the link is computed on the one plane read twice --
//   `link_frame(detector, slot, main_left, main_left, ..)` -- and in the original operation order.
//   That is not a simplification of the linked expression, it is the expression: `Maximum` gives
//   `max(p, p) = p` bit-exact, and `Average` gives `0.5*p + 0.5*p`, whose two products and one add
//   are what round -- `p` itself would be a different value for a subnormal `p`, where `0.5*p`
//   loses a bit and the sum of two halves does not come back to `p`. Writing `p` there would be a
//   rounding change on exactly the inputs nobody tests by ear;
// * `Detector::Sidechain` still reads *both* sidechain planes through `link_frame`, so a collapsed
//   bank must never be handed one. `process_bank` already refuses a sidechained block; the
//   per-instance path declines on `Detector::Sidechain` for the same reason.
//
// The right channel's state is untouched. It is restored by `Channel::copy_state_from` at the
// disengage boundary, before any dual block runs.
// ---------------------------------------------------------------------------------------------

/// [`segment_is_stageable`] for one channel.
#[inline(always)]
fn segment_is_stageable_mono<L: Lane>(channel: &Channel<L>, len: usize) -> bool {
    len <= MAX_STAGED_FRAMES && min_delay(channel) >= len
}

/// [`process_block`] over one plane.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_block_mono<L: Lane>(
    left: &mut [f32],
    detector: Detector<'_>,
    frames: usize,
    link: LinkMode,
    bypass: bool,
    sample_rate: u32,
    channel_left: &mut Channel<L>,
    staged: &mut Staged<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    let remaining = channel_left.max_remaining() as usize;
    let ramping = if remaining < frames {
        remaining
    } else {
        frames
    };
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
        if segment_is_stageable_mono(channel_left, frames - ramping) {
            let Staged {
                taps_left,
                target_left,
                ..
            } = staged;
            idle_frames_staged_mono::<L>(
                left,
                detector,
                ramping,
                frames,
                link,
                bypass,
                channel_left,
                taps_left,
                target_left,
            );
        } else {
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
}

/// [`frames_loop`] over one plane.
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
    let ring_length = channel_left.ring_length as usize;
    let invariants = Invariants::<L>::new(link, bypass);

    let mut coef_left = Coef::load(&channel_left.words);
    let mut gather = [0.0_f32; MAX_WIDTH];

    for frame in start..end {
        if RAMPING {
            channel_left.advance_ramps(sample_rate);
            coef_left = Coef::load(&channel_left.words);
        }
        let slot = frame * width;

        let main_left = L::load(&left[slot..]);
        let (level_left, _level_right) =
            link_frame(detector, slot, main_left, main_left, &invariants);

        let write = channel_left.cursor as usize;
        let next = write + 1;
        let next = if next == ring_length { 0 } else { next };
        main_left.store(&mut channel_left.main[write * width..]);
        level_left.store(&mut channel_left.detector[write * width..]);
        let delayed_left = L::load(&channel_left.main[next * width..]);
        let detected_left = gather_detector(channel_left, write, &mut gather);

        let output_left = one_frame(
            delayed_left,
            detected_left,
            &coef_left,
            &mut channel_left.gain_reduction_db,
            &invariants,
        );
        output_left.store(&mut left[slot..]);

        channel_left.cursor = next as u32;
    }
}

/// [`idle_frames_staged`] over one plane.
#[allow(clippy::too_many_arguments)]
fn idle_frames_staged_mono<L: Lane>(
    left: &mut [f32],
    detector: Detector<'_>,
    start: usize,
    end: usize,
    link: LinkMode,
    bypass: bool,
    channel_left: &mut Channel<L>,
    taps_left: &mut [f32],
    target_left: &mut [L],
) {
    let width = L::WIDTH;
    let ring_length = channel_left.ring_length as usize;
    let len = end - start;
    debug_assert!(segment_is_stageable_mono(channel_left, len));
    let invariants = Invariants::<L>::new(link, bypass);
    let coef_left = Coef::load(&channel_left.words);

    let head = channel_left.cursor as usize;
    fill_taps(channel_left, head, len, taps_left);

    let mut write = head;
    for (index, (tap_left, frame_left)) in taps_left[..len * width]
        .chunks_exact(width)
        .zip(target_left[..len].iter_mut())
        .enumerate()
    {
        let slot = (start + index) * width;
        let main_left = L::load(&left[slot..]);
        let (level_left, _level_right) =
            link_frame(detector, slot, main_left, main_left, &invariants);
        let next = write + 1;
        let next = if next == ring_length { 0 } else { next };
        main_left.store(&mut channel_left.main[write * width..]);
        level_left.store(&mut channel_left.detector[write * width..]);
        L::load(&channel_left.main[next * width..]).store(&mut left[slot..]);
        *frame_left = curve_target(L::load(tap_left), &coef_left, &invariants);
        write = next;
    }
    channel_left.cursor = write as u32;

    let mut gain_left = channel_left.gain_reduction_db;
    for frame_left in target_left[..len].iter_mut() {
        *frame_left = ballistic(*frame_left, &mut gain_left, &coef_left);
    }
    channel_left.gain_reduction_db = gain_left;

    for (index, smoothed_left) in target_left[..len].iter().enumerate() {
        let slot = (start + index) * width;
        let delayed_left = L::load(&left[slot..]);
        gain_mix(delayed_left, *smoothed_left, &coef_left, &invariants).store(&mut left[slot..]);
    }
}
