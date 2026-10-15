//! Issue #140 B: the live, ramped fader and mute section.
//!
//! Two properties matter and both are gated here.
//!
//! 1. **Bit identity with the prepared path when nothing is commanded.** A live-control-attached
//!    track that never receives a fader record must render exactly the bits `FaderMuteBuiltins`
//!    renders, including the exact `+0.0` a declared mute produces. That is what lets #140 add a
//!    ramped fader without moving a single builtins fixture digest.
//! 2. **Mute is a fader endpoint.** A mute with a window fades; a mute with no window is the
//!    instantaneous, exact `+0.0` the prepared path gives. Neither is a discontinuity the caller
//!    did not ask for.

use builtins::*;

fn parameters(left_db: f32, right_db: f32, left_mute: bool, right_mute: bool) -> BuiltinParameters {
    BuiltinParameters {
        left: ChannelParameters {
            fader_db: left_db,
            muted: left_mute,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            fader_db: right_db,
            muted: right_mute,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    }
}

fn prepared(parameters: BuiltinParameters) -> FaderMuteBuiltins {
    BuiltinChain::new(48_000, parameters)
        .expect("chain")
        .into_sections()
        .1
}

fn render(stage: &mut impl FnMut(&mut [f32], &mut [f32]), input: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut left = input.to_vec();
    let mut right: Vec<f32> = input.iter().map(|value| -*value).collect();
    stage(&mut left, &mut right);
    (left, right)
}

const INPUT: [f32; 8] = [1.0, -1.0, 0.5, -0.5, 0.25, -0.25, 0.125, -0.125];

/// Red mutation: make the settled tail of `FaderMuteRampBuiltins::process_lane` multiply by
/// `self.current[lane]` even when the lane is muted -> the muted right plane renders `-0.0` where
/// the prepared `andnot` renders `+0.0`, and the `to_bits` comparison below fails.
#[test]
fn an_uncommanded_live_fader_is_bit_identical_to_the_prepared_one() {
    for (left_db, right_db, left_mute, right_mute) in [
        (0.0_f32, 0.0_f32, false, false),
        (-6.0, 3.0, false, false),
        (0.0, 0.0, true, false),
        (-12.0, -12.0, true, true),
        (-144.0, 24.0, false, true),
    ] {
        let params = parameters(left_db, right_db, left_mute, right_mute);
        let mut prepared_stage = prepared(params);
        let mut live = FaderMuteRampBuiltins::new(params).expect("live fader");
        let (prepared_left, prepared_right) = render(
            &mut |left: &mut [f32], right: &mut [f32]| {
                prepared_stage.process(DualMonoBlock::new(left, right, 0).expect("block"));
            },
            &INPUT,
        );
        let (live_left, live_right) = render(
            &mut |left: &mut [f32], right: &mut [f32]| {
                live.process(DualMonoBlock::new(left, right, 0).expect("block"));
            },
            &INPUT,
        );
        assert_eq!(
            prepared_left
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            live_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "left plane at {left_db} dB (muted={left_mute}) must be bit-identical"
        );
        assert_eq!(
            prepared_right
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            live_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "right plane at {right_db} dB (muted={right_mute}) must be bit-identical"
        );
    }
}

/// A zero-window fader move is instantaneous and exact from the first sample of the block.
#[test]
fn a_zero_window_move_takes_effect_on_the_first_sample() {
    let mut live = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
    live.set_fader_db(BuiltinLaneSelector::Left, -6.020_6, 0)
        .expect("domain");
    let mut left = [1.0_f32; 4];
    let mut right = [1.0_f32; 4];
    live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    for sample in left {
        assert!(
            (sample - 0.5).abs() < 1e-5,
            "a zero window is immediate: {sample}"
        );
    }
    assert_eq!(right, [1.0; 4], "the right lane is untouched");
}

/// Red mutation: drop the `index + 1 == self.remaining[lane]` exact-assignment leg so the ramp
/// only accumulates -> the window's last sample is the accumulated approximation rather than the
/// target, and the exact assertion below fails.
///
/// The window/target pairs are chosen so that accumulation demonstrably *cannot* land on the
/// target: for each of them, `n` `f32` additions of `(target - 1) / n` differ from `target` in the
/// last place. A single convenient pair would not gate D11 at all -- eight steps toward `0.5`
/// happen to accumulate exactly, which is precisely how a weak version of this test passes with
/// the law deleted.
#[test]
fn a_windowed_move_is_monotone_and_lands_exactly_on_its_target() {
    for (db, window) in [
        (-6.020_6_f32, 3_u32),
        (-20.0, 2),
        (24.0, 8),
        (-3.0, 2),
        (-1.0, 2),
    ] {
        let mut live =
            FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
        live.set_fader_db(BuiltinLaneSelector::Both, db, window)
            .expect("domain");
        let target = live.target_gain(0);
        let frames = window as usize + 2;
        let mut left = vec![1.0_f32; frames];
        let mut right = vec![1.0_f32; frames];
        live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        let last = window as usize - 1;
        for index in 1..=last {
            let monotone = if db < 0.0 {
                left[index] < left[index - 1]
            } else {
                left[index] > left[index - 1]
            };
            assert!(
                monotone,
                "db={db} window={window}: the fade is monotone: {left:?}"
            );
        }
        assert_eq!(
            left[last].to_bits(),
            target.to_bits(),
            "db={db} window={window}: the last update assigns the target exactly (D11)"
        );
        assert_eq!(
            left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        );
        // The settled tail of the same block, and the next block, are the target itself.
        for (index, sample) in left.iter().enumerate().skip(window as usize) {
            assert_eq!(
                sample.to_bits(),
                target.to_bits(),
                "db={db} window={window}: sample {index} is settled"
            );
        }
        let mut left = [1.0_f32; 4];
        let mut right = [1.0_f32; 4];
        live.process(DualMonoBlock::new(&mut left, &mut right, frames as u64).expect("block"));
        assert_eq!(left.map(f32::to_bits), [target.to_bits(); 4]);
    }
}

/// Mute is a fader endpoint: with a window it fades to zero; the settled state is the exact
/// `+0.0` the prepared path produces, for a negative input too.
///
/// Red mutation: make `set_mute` snap (`self.current[lane] = target` unconditionally) -> the first
/// sample of the fade is already `0.0` and the "still audible partway through" assertion fails.
#[test]
fn mute_is_a_fader_endpoint_and_settles_to_exact_positive_zero() {
    const WINDOW: u32 = 4;
    let mut live = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
    live.set_mute(BuiltinLaneSelector::Both, true, WINDOW);
    assert!(live.is_muted(0) && live.is_muted(1));
    let mut left = [-1.0_f32; 8];
    let mut right = [-1.0_f32; 8];
    live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert!(
        left[0] < 0.0 && left[0] > -1.0,
        "the first sample of a mute fade is still audible: {}",
        left[0]
    );
    assert!(left[1] > left[0], "the fade is monotone toward zero");
    for (index, sample) in left.iter().enumerate().skip(4) {
        assert_eq!(
            sample.to_bits(),
            0.0_f32.to_bits(),
            "sample {index} is settled to exactly +0.0, not -0.0"
        );
    }

    // Unmuting returns to the lane's fader gain, again as a ramp.
    live.set_mute(BuiltinLaneSelector::Both, false, WINDOW);
    assert!(!live.is_muted(0));
    let mut left = [1.0_f32; 8];
    let mut right = [1.0_f32; 8];
    live.process(DualMonoBlock::new(&mut left, &mut right, 8).expect("block"));
    assert!(left[0] > 0.0 && left[0] < 1.0, "the unmute fades back up");
    assert_eq!(left[7].to_bits(), 1.0_f32.to_bits(), "back to unity");
}

/// A zero-window mute is the exact prepared mute from the first sample, including for a negative
/// input: that is the "mute button" case, and it must not carry `-0.0`.
#[test]
fn a_zero_window_mute_is_the_exact_prepared_mute() {
    let mut live = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
    live.set_mute(BuiltinLaneSelector::Right, true, 0);
    let mut left = [-1.0_f32; 4];
    let mut right = [-1.0_f32; 4];
    live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left.map(f32::to_bits), [(-1.0_f32).to_bits(); 4]);
    assert_eq!(
        right.map(f32::to_bits),
        [0.0_f32.to_bits(); 4],
        "an instantaneous mute is exactly +0.0"
    );
}

/// A fader move made while muted is remembered and applies on unmute, like a physical fader.
#[test]
fn a_move_made_while_muted_applies_on_unmute() {
    let mut live = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, true, true)).expect("fader");
    live.set_fader_db(BuiltinLaneSelector::Both, -6.020_6, 0)
        .expect("domain");
    let mut left = [1.0_f32; 4];
    let mut right = [1.0_f32; 4];
    live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(
        left.map(f32::to_bits),
        [0.0_f32.to_bits(); 4],
        "still muted"
    );

    live.set_mute(BuiltinLaneSelector::Both, false, 0);
    let mut left = [1.0_f32; 4];
    let mut right = [1.0_f32; 4];
    live.process(DualMonoBlock::new(&mut left, &mut right, 4).expect("block"));
    for sample in left {
        assert!(
            (sample - 0.5).abs() < 1e-5,
            "unmute restores the fader value set while muted: {sample}"
        );
    }
}

/// The declared domain is enforced at the setter, off the render thread.
#[test]
fn the_fader_domain_is_enforced_at_the_setter() {
    let mut live = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
    assert!(
        live.set_fader_db(BuiltinLaneSelector::Both, 24.1, 0)
            .is_err()
    );
    assert!(
        live.set_fader_db(BuiltinLaneSelector::Both, -144.1, 0)
            .is_err()
    );
    assert!(
        live.set_fader_db(BuiltinLaneSelector::Both, f32::NAN, 0)
            .is_err()
    );
    assert!(
        live.set_fader_db(BuiltinLaneSelector::Both, 24.0, 0)
            .is_ok()
    );
    assert!(
        live.set_fader_db(BuiltinLaneSelector::Both, -144.0, 0)
            .is_ok()
    );
}

/// A ramp that spans blocks is partition-invariant: eight frames as one block and as two blocks
/// of four produce the same bits.
#[test]
fn a_cross_block_ramp_is_partition_invariant() {
    let build = || {
        let mut live =
            FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
        live.set_fader_db(BuiltinLaneSelector::Both, -20.0, 6)
            .expect("domain");
        live
    };
    let whole = {
        let mut live = build();
        let mut left = [1.0_f32; 8];
        let mut right = [1.0_f32; 8];
        live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        left
    };
    let split = {
        let mut live = build();
        let mut first = [1.0_f32; 4];
        let mut first_right = [1.0_f32; 4];
        live.process(DualMonoBlock::new(&mut first, &mut first_right, 0).expect("block"));
        let mut second = [1.0_f32; 4];
        let mut second_right = [1.0_f32; 4];
        live.process(DualMonoBlock::new(&mut second, &mut second_right, 4).expect("block"));
        let mut joined = [0.0_f32; 8];
        joined[..4].copy_from_slice(&first);
        joined[4..].copy_from_slice(&second);
        joined
    };
    assert_eq!(
        whole.map(f32::to_bits),
        split.map(f32::to_bits),
        "a cross-block fader ramp is to_bits identical across the partition boundary"
    );
}

/// Issue #1408 gate 3: no fader or mute move applies a gain outside the two words it moves
/// between.
///
/// Found by a scalar simulation of the unclamped D11 law (see the issue's attempt record): a
/// one-second fade from unity to mute at 48 kHz accumulates to `-0.000624` before its snap, and
/// a +24 dB to -144 dB move over 65,536 samples reaches `-0.0108` -- in both cases a gain whose
/// sign the user never set. DC `1.0` in, so every output sample is the applied gain word.
///
/// Red mutation: revert `gain_mute_ramp_block`'s step 3 to `current + step` -> both moves leave
/// their interval.
#[test]
fn a_long_fader_or_mute_move_stays_inside_its_endpoints() {
    const BLOCK: usize = 4_096;
    for (from_db, mute, to_db, window) in [
        (0.0_f32, true, 0.0_f32, 48_000_u32),
        (24.0, false, -144.0, 65_536),
    ] {
        let mut live =
            FaderMuteRampBuiltins::new(parameters(from_db, from_db, false, false)).expect("fader");
        let start = live.target_gain(0);
        let target = if mute {
            live.set_mute(BuiltinLaneSelector::Both, true, window);
            0.0
        } else {
            live.set_fader_db(BuiltinLaneSelector::Both, to_db, window)
                .expect("domain");
            live.target_gain(0)
        };
        let (low, high) = (start.min(target), start.max(target));
        let frames = window as usize + 64;
        let mut rendered = 0;
        while rendered < frames {
            let block = BLOCK.min(frames - rendered);
            let mut left = vec![1.0_f32; block];
            let mut right = vec![1.0_f32; block];
            live.process(
                DualMonoBlock::new(&mut left, &mut right, rendered as u64).expect("block"),
            );
            for (index, gain) in left.iter().chain(&right).enumerate() {
                assert!(
                    (low..=high).contains(gain),
                    "{from_db} dB -> {} over {window}: frame {} applies {gain:e} outside \
                     [{low:e}, {high:e}]",
                    if mute {
                        "mute".to_owned()
                    } else {
                        format!("{to_db} dB")
                    },
                    rendered + index % block
                );
            }
            rendered += block;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Issue #1638: timed operations inside a block on the fader stage.
// ---------------------------------------------------------------------------------------------

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use effect_contract::BankWidth;
use lane::Backend;

/// One timed operation: frame offset in the block, addressed lane index, operation.
type Timed = (u32, usize, FaderOperation);

/// A pull source over a fixed slice of timed operations, sorted by offset. Allocation-free.
struct SliceSource<'a> {
    operations: &'a [Timed],
    index: usize,
}

impl<'a> SliceSource<'a> {
    fn new(operations: &'a [Timed]) -> Self {
        Self {
            operations,
            index: 0,
        }
    }
}

/// Lane index `i` is lane `i`, both channels, on a bank; and channel `i` of lane `0` on the
/// scalar track, whose two channels are its two independent lanes.
fn address(scalar: bool, index: usize) -> (usize, BuiltinLaneSelector) {
    match (scalar, index) {
        (true, 0) => (0, BuiltinLaneSelector::Left),
        (true, _) => (0, BuiltinLaneSelector::Right),
        (false, lane) => (lane, BuiltinLaneSelector::Both),
    }
}

/// Builds one fresh subject.
type Build = Box<dyn Fn() -> Subject>;

/// The operations of one block, by block index.
type OpsFor = Box<dyn Fn(usize) -> Vec<Timed>>;

/// The three users of the stage, behind one shape so every gate runs on each of them.
enum Subject {
    Bank(Box<BuiltinFaderBank>),
    Scalar(FaderMuteRampBuiltins),
}

struct Source<'a> {
    inner: SliceSource<'a>,
    scalar: bool,
}

impl FaderOperationSource for Source<'_> {
    fn next_offset(&mut self) -> Option<u32> {
        self.inner
            .operations
            .get(self.inner.index)
            .map(|(offset, ..)| *offset)
    }

    fn apply(&mut self, offset: u32, sink: &mut FaderOperationSink) {
        while let Some(&(at, index, operation)) = self.inner.operations.get(self.inner.index) {
            if at != offset {
                break;
            }
            let (lane, channels) = address(self.scalar, index);
            sink.push(lane, channels, operation)
                .expect("a valid operation");
            self.inner.index += 1;
        }
    }
}

impl Subject {
    fn subjects() -> Vec<(&'static str, Build)> {
        let mut subjects: Vec<(&'static str, Build)> =
            vec![("simd4", Box::new(|| Self::bank(Backend::Simd4)))];
        #[cfg(target_feature = "avx2")]
        subjects.push(("simd8", Box::new(|| Self::bank(Backend::Simd8))));
        subjects.push((
            "scalar",
            Box::new(|| {
                Self::Scalar(
                    FaderMuteRampBuiltins::new(parameters(-2.0, -5.0, false, false))
                        .expect("fader"),
                )
            }),
        ));
        subjects
    }

    /// A bank with one padding lane, so padding lanes ride along inert.
    fn bank(backend: Backend) -> Self {
        let width = BankWidth::for_backend(backend).expect("bank width");
        let members = width.lanes() as usize - 1;
        let faders = (0..members)
            .map(|lane| parameters(-2.0 - lane as f32, -5.0 + lane as f32, false, false))
            .collect();
        Self::Bank(Box::new(
            BuiltinFaderBank::new(backend, width, faders).expect("fader bank"),
        ))
    }

    const fn scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }

    fn width(&self) -> usize {
        match self {
            Self::Bank(bank) => bank.width().lanes() as usize,
            Self::Scalar(_) => 1,
        }
    }

    fn live_gain(&mut self, index: usize, db: f32, window: u32) {
        let (lane, channels) = address(self.scalar(), index);
        match self {
            Self::Bank(bank) => bank.set_fader_db(lane, channels, db, window),
            Self::Scalar(track) => track.set_fader_db(channels, db, window),
        }
        .expect("domain");
    }

    fn live_mute(&mut self, index: usize, muted: bool, window: u32) {
        let (lane, channels) = address(self.scalar(), index);
        match self {
            Self::Bank(bank) => bank.set_mute(lane, channels, muted, window).expect("lane"),
            Self::Scalar(track) => track.set_mute(channels, muted, window),
        }
    }

    fn apply(&mut self, index: usize, operation: FaderOperation) {
        let (lane, channels) = address(self.scalar(), index);
        match self {
            Self::Bank(bank) => bank.apply_fader_operation(lane, channels, operation),
            Self::Scalar(track) => track.apply_fader_operation(channels, operation),
        }
        .expect("a valid operation");
    }

    fn held(&self, index: usize) -> bool {
        match self {
            Self::Bank(bank) => bank.gain_held(index, 0).expect("member lane"),
            Self::Scalar(track) => track.gain_held(index).expect("a channel"),
        }
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32], start: u64) {
        match self {
            Self::Bank(bank) => {
                let frames = (left.len() / bank.width().lanes() as usize) as u32;
                bank.process(left, right, frames);
            }
            Self::Scalar(track) => {
                track.process(DualMonoBlock::new(left, right, start).expect("block"));
            }
        }
    }

    fn process_with(&mut self, left: &mut [f32], right: &mut [f32], start: u64, ops: &[Timed]) {
        let mut source = Source {
            inner: SliceSource::new(ops),
            scalar: self.scalar(),
        };
        match self {
            Self::Bank(bank) => {
                let frames = (left.len() / bank.width().lanes() as usize) as u32;
                bank.process_with(left, right, frames, &mut source);
            }
            Self::Scalar(track) => {
                track.process_with(
                    DualMonoBlock::new(left, right, start).expect("block"),
                    &mut source,
                );
            }
        }
        assert!(
            source.next_offset().is_none(),
            "every operation was applied"
        );
    }

    /// The reference: `process` up to each offset, the operations applied through the setter,
    /// then `process` over the rest.
    fn process_split(&mut self, left: &mut [f32], right: &mut [f32], start: u64, ops: &[Timed]) {
        let width = self.width();
        let frames = left.len() / width;
        let mut position = 0;
        for &(offset, index, operation) in ops {
            let at = offset as usize;
            if at > position {
                self.process(
                    &mut left[position * width..at * width],
                    &mut right[position * width..at * width],
                    start + position as u64,
                );
                position = at;
            }
            self.apply(index, operation);
        }
        if position < frames {
            self.process(
                &mut left[position * width..],
                &mut right[position * width..],
                start + position as u64,
            );
        }
    }
}

/// A deterministic block with both signs and no zero, distinct per frame and lane.
fn signal(frames: usize, width: usize, block: usize) -> (Vec<f32>, Vec<f32>) {
    let at = |index: usize, salt: usize| {
        let phase = (block * 7919 + index * 31 + salt) % 211;
        (phase as f32 - 105.5) / 97.0
    };
    let samples = frames * width;
    (
        (0..samples).map(|index| at(index, 3)).collect(),
        (0..samples).map(|index| at(index, 101)).collect(),
    )
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// The state a lane is in when the block with the operation starts.
#[derive(Clone, Copy, Debug)]
enum LaneState {
    Settled,
    Ramping,
    RampEndsBeforeOffset,
    Muted,
    MuteRamp,
    UnmuteRamp,
}

fn prime(subject: &mut Subject, index: usize, state: LaneState, offset: u32) {
    match state {
        LaneState::Settled => {}
        LaneState::Ramping => subject.live_gain(index, -14.0, 300),
        LaneState::RampEndsBeforeOffset => subject.live_gain(index, -14.0, (offset / 2).max(1)),
        LaneState::Muted => subject.live_mute(index, true, 0),
        LaneState::MuteRamp => subject.live_mute(index, true, 300),
        LaneState::UnmuteRamp => {
            subject.live_mute(index, true, 0);
            subject.live_mute(index, false, 300);
        }
    }
}

const KINDS: [FaderOperation; 6] = [
    FaderOperation::RetargetGain {
        gain: 0.3,
        samples: 40,
    },
    FaderOperation::RetargetGain {
        gain: 1.7,
        samples: 1_000,
    },
    FaderOperation::SetGain { gain: 0.7 },
    FaderOperation::SetMute {
        muted: true,
        samples: 24,
    },
    FaderOperation::SetMute {
        muted: false,
        samples: 24,
    },
    FaderOperation::RememberGain { gain: 0.4 },
];

/// Gate 1: one `process_with` call is bit for bit `process` split at the offsets with the
/// operations applied between, at every width, for every kind and lane state, and the block after
/// it agrees too (so the state left behind is the same).
///
/// Red if a piece reuses a stale countdown or an operation lands a sample early or late. Each width
/// is compared only with its own split, so a divergence between widths is not this test's claim.
#[test]
fn an_operation_inside_a_block_renders_the_split_blocks_bits() {
    let states = [
        LaneState::Settled,
        LaneState::Ramping,
        LaneState::RampEndsBeforeOffset,
        LaneState::Muted,
        LaneState::MuteRamp,
        LaneState::UnmuteRamp,
    ];
    let mut checked = 0;
    for (name, build) in Subject::subjects() {
        for (quantum, offsets) in [(128_usize, &[1_u32, 63, 64, 127][..]), (100, &[1, 50, 99])] {
            for &offset in offsets {
                for state in states {
                    for operation in KINDS {
                        // One lane, then two lanes with operations at different offsets.
                        let other = (offset + quantum as u32 / 3) % quantum as u32;
                        let other = if other == offset { offset - 1 } else { other };
                        let mut two = vec![(offset, 0, operation), (other, 1, operation)];
                        two.sort_by_key(|(at, ..)| *at);
                        for ops in [vec![(offset, 0, operation)], two] {
                            let mut whole = build();
                            let mut split = build();
                            for index in 0..2 {
                                prime(&mut whole, index, state, offset);
                                prime(&mut split, index, state, offset);
                            }
                            let width = whole.width();
                            for block in 0..2 {
                                let (mut left, mut right) = signal(quantum, width, block);
                                let (mut split_left, mut split_right) =
                                    (left.clone(), right.clone());
                                let start = (block * quantum) as u64;
                                let ops: &[Timed] = if block == 0 { &ops } else { &[] };
                                whole.process_with(&mut left, &mut right, start, ops);
                                split.process_split(&mut split_left, &mut split_right, start, ops);
                                let context = format!(
                                    "{name} q={quantum} o={offset} {state:?} {operation:?} \
                                     lanes={} block={block}",
                                    ops.len()
                                );
                                assert_eq!(bits(&left), bits(&split_left), "left: {context}");
                                assert_eq!(bits(&right), bits(&split_right), "right: {context}");
                            }
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(checked >= 2 * 7 * 6 * 6 * 2, "the matrix ran: {checked}");
}

/// Operations at offset `0` apply before the first frame, and an operation past the block stays
/// with the source.
#[test]
fn an_operation_at_offset_zero_is_block_entry_and_one_past_the_block_waits() {
    for (name, build) in Subject::subjects() {
        let mut whole = build();
        let mut split = build();
        let width = whole.width();
        let ops = [(0, 0, FaderOperation::SetGain { gain: 0.25 })];
        let (mut left, mut right) = signal(64, width, 0);
        let (mut split_left, mut split_right) = (left.clone(), right.clone());
        whole.process_with(&mut left, &mut right, 0, &ops);
        split.process_split(&mut split_left, &mut split_right, 0, &ops);
        assert_eq!(bits(&left), bits(&split_left), "{name}: offset 0");

        let late = [(64, 0, FaderOperation::SetGain { gain: 0.5 })];
        let mut source = Source {
            inner: SliceSource::new(&late),
            scalar: whole.scalar(),
        };
        let (mut left, mut right) = signal(64, width, 1);
        let (mut plain_left, mut plain_right) = (left.clone(), right.clone());
        match &mut whole {
            Subject::Bank(bank) => {
                bank.process_with(&mut left, &mut right, 64, &mut source);
            }
            Subject::Scalar(track) => {
                track.process_with(
                    DualMonoBlock::new(&mut left, &mut right, 64).expect("block"),
                    &mut source,
                );
            }
        }
        split.process(&mut plain_left, &mut plain_right, 64);
        assert_eq!(bits(&left), bits(&plain_left), "{name}: nothing applied");
        assert_eq!(
            source.next_offset(),
            Some(64),
            "{name}: the late operation waits"
        );
    }
}

/// Renders `blocks` blocks of 128 on lane 0 with `ops_for(block)` applied, returning lane 0's
/// left samples in frame order.
fn lane_zero_render(
    subject: &mut Subject,
    blocks: usize,
    ops_for: &dyn Fn(usize) -> Vec<Timed>,
    live_for: &dyn Fn(&mut Subject, usize),
) -> Vec<f32> {
    let width = subject.width();
    let mut out = Vec::new();
    for block in 0..blocks {
        live_for(subject, block);
        let (mut left, mut right) = signal(128, width, block);
        let ops = ops_for(block);
        subject.process_with(&mut left, &mut right, (block * 128) as u64, &ops);
        out.extend(left.iter().step_by(width));
    }
    out
}

/// Gate 2: gain moves during a 480-sample mute ramp are only remembered -- by a timed
/// `RetargetGain`, by `RememberGain` and by the live setter -- so the lane renders the bits of the
/// mute ramp alone, and an unmute then ramps to the last remembered gain.
///
/// Red if a gain move during a mute ramp restarts it toward `0` (README F12/F22's artefact) or
/// the remembered gain is lost.
#[test]
fn gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it() {
    const LAST_DB: f32 = -4.5;
    let last = checked_fader_gain(LAST_DB).expect("domain");
    let mute_at_entry = |subject: &mut Subject, block: usize| {
        if block == 0 {
            subject.live_mute(0, true, 480);
        }
    };
    for (name, build) in Subject::subjects() {
        let alone = lane_zero_render(&mut build(), 4, &|_| Vec::new(), &mute_at_entry);
        let every_64 = |make: fn(f32) -> FaderOperation| {
            move |block: usize| -> Vec<Timed> {
                (0..2)
                    .map(|half| {
                        let at = block * 128 + half * 64 + 1;
                        let gain = if at + 64 >= 480 {
                            last
                        } else {
                            0.1 + at as f32 / 1_000.0
                        };
                        ((half * 64 + 1) as u32, 0, make(gain))
                    })
                    .filter(|(offset, ..)| block * 128 + (*offset as usize) < 480)
                    .collect()
            }
        };
        let cases: [(&str, OpsFor, bool); 3] = [
            (
                "RetargetGain",
                Box::new(every_64(|gain| FaderOperation::RetargetGain {
                    gain,
                    samples: 32,
                })),
                false,
            ),
            (
                "RememberGain",
                Box::new(every_64(|gain| FaderOperation::RememberGain { gain })),
                false,
            ),
            ("live", Box::new(|_| Vec::new()), true),
        ];
        for (case, ops, live) in cases {
            let mut subject = build();
            let live_moves = |subject: &mut Subject, block: usize| {
                mute_at_entry(subject, block);
                if live && block * 128 < 480 {
                    let db = if block == 3 {
                        LAST_DB
                    } else {
                        -1.0 - block as f32
                    };
                    subject.live_gain(0, db, 32);
                }
            };
            let rendered = lane_zero_render(&mut subject, 4, &*ops, &live_moves);
            assert_eq!(
                bits(&rendered),
                bits(&alone),
                "{name} {case}: the mute ramp alone"
            );
            assert!(subject.held(0), "{name} {case}: still muted");

            // Unmute over 32 samples; the lane ramps to the last remembered gain and settles there.
            let unmute = |block: usize| -> Vec<Timed> {
                if block == 0 {
                    vec![(
                        0,
                        0,
                        FaderOperation::SetMute {
                            muted: false,
                            samples: 32,
                        },
                    )]
                } else {
                    Vec::new()
                }
            };
            let after = lane_zero_render(&mut subject, 2, &unmute, &|_, _| {});
            let width = subject.width();
            let (input, _) = signal(128, width, 1);
            // The live setter converts `LAST_DB` with the same function, so one coefficient.
            let expected: Vec<f32> = input.iter().step_by(width).map(|x| x * last).collect();
            assert_eq!(
                bits(&after[128..]),
                bits(&expected),
                "{name} {case}: the remembered gain"
            );
            assert!(!subject.held(0), "{name} {case}: the unmute ramp ended");
        }
    }
}

/// Gate 3: at one offset, gain operations apply before mute operations whatever order they are
/// pushed in. An unmute and a gain move end at the new gain; a mute and a gain move end muted.
///
/// Red if the stage applies them in push order: the unmute's ramp would then hold the gain, and
/// the lane would settle at its old gain.
#[test]
fn at_one_offset_the_gain_applies_before_the_mute() {
    for (name, build) in Subject::subjects() {
        let width = build().width();
        // Muted lane: unmute (pushed first) and a gain move at offset 10.
        let mut subject = build();
        subject.live_mute(0, true, 0);
        let ops = [
            (
                10,
                0,
                FaderOperation::SetMute {
                    muted: false,
                    samples: 32,
                },
            ),
            (
                10,
                0,
                FaderOperation::RetargetGain {
                    gain: 0.45,
                    samples: 32,
                },
            ),
        ];
        let mut source = Source {
            inner: SliceSource::new(&ops),
            scalar: subject.scalar(),
        };
        let (mut left, mut right) = signal(128, width, 0);
        match &mut subject {
            Subject::Bank(bank) => {
                bank.process_with(&mut left, &mut right, 128, &mut source);
            }
            Subject::Scalar(track) => {
                track.process_with(
                    DualMonoBlock::new(&mut left, &mut right, 0).expect("block"),
                    &mut source,
                );
            }
        }
        let after = lane_zero_render(&mut subject, 1, &|_| Vec::new(), &|_, _| {});
        let (input, _) = signal(128, width, 0);
        let expected: Vec<f32> = input.iter().step_by(width).map(|x| x * 0.45).collect();
        assert_eq!(
            bits(&after),
            bits(&expected),
            "{name}: unmuted to the new gain"
        );

        // Unmuted lane: mute (pushed first) and an exact gain set at offset 10.
        let mut subject = build();
        let ops = [
            (
                10,
                0,
                FaderOperation::SetMute {
                    muted: true,
                    samples: 16,
                },
            ),
            (10, 0, FaderOperation::SetGain { gain: 0.9 }),
        ];
        let mut source = Source {
            inner: SliceSource::new(&ops),
            scalar: subject.scalar(),
        };
        let (mut left, mut right) = signal(128, width, 0);
        match &mut subject {
            Subject::Bank(bank) => {
                bank.process_with(&mut left, &mut right, 128, &mut source);
            }
            Subject::Scalar(track) => {
                track.process_with(
                    DualMonoBlock::new(&mut left, &mut right, 0).expect("block"),
                    &mut source,
                );
            }
        }
        let lane: Vec<f32> = left.iter().step_by(width).copied().collect();
        assert!(
            lane[26..].iter().all(|x| x.to_bits() == 0),
            "{name}: ended muted"
        );
        let after = lane_zero_render(&mut subject, 1, &|_| Vec::new(), &|_, _| {});
        assert!(
            after.iter().all(|x| x.to_bits() == 0),
            "{name}: stays muted"
        );
        subject.live_mute(0, false, 0);
        let after = lane_zero_render(&mut subject, 1, &|_| Vec::new(), &|_, _| {});
        let expected: Vec<f32> = input.iter().step_by(width).map(|x| x * 0.9).collect();
        assert_eq!(
            bits(&after),
            bits(&expected),
            "{name}: the gain set before the mute"
        );
    }
}

/// D2: the held flag follows the mute ramp. It is set by a windowed mute change, cleared the
/// frame the countdown ends, cleared by a live retarget during an unmute ramp, and carried
/// across a plan swap's lane export and import.
#[test]
fn the_held_flag_follows_the_mute_ramp() {
    let mut bank = match Subject::bank(Backend::Simd4) {
        Subject::Bank(bank) => bank,
        Subject::Scalar(_) => unreachable!(),
    };
    let block = |bank: &mut BuiltinFaderBank, frames: u32| {
        let mut left = vec![0.5_f32; frames as usize * 4];
        let mut right = left.clone();
        bank.process(&mut left, &mut right, frames);
    };
    bank.set_mute(0, BuiltinLaneSelector::Left, false, 40)
        .expect("lane");
    assert_eq!(
        bank.gain_held(0, 0),
        Some(true),
        "an unmute ramp holds the gain"
    );
    assert_eq!(
        bank.gain_held(0, 1),
        Some(false),
        "the right channel is free"
    );
    block(&mut bank, 39);
    assert_eq!(bank.ramp_remaining(0, 0), Some(1));
    assert_eq!(bank.gain_held(0, 0), Some(true), "one frame left");

    // A plan swap carries the flag.
    let state = bank.export_lane(0).expect("member");
    let mut successor = match Subject::bank(Backend::Simd4) {
        Subject::Bank(bank) => bank,
        Subject::Scalar(_) => unreachable!(),
    };
    successor.import_lane(1, &state).expect("member");
    assert_eq!(successor.gain_held(1, 0), Some(true), "carried by the swap");

    block(&mut bank, 1);
    assert_eq!(
        bank.gain_held(0, 0),
        Some(false),
        "cleared as the countdown ends"
    );

    bank.set_mute(0, BuiltinLaneSelector::Left, false, 40)
        .expect("lane");
    bank.set_fader_db(0, BuiltinLaneSelector::Left, -6.0, 8)
        .expect("domain");
    assert_eq!(
        bank.gain_held(0, 0),
        Some(false),
        "a live retarget ends an unmute ramp"
    );
    bank.set_mute(0, BuiltinLaneSelector::Left, true, 0)
        .expect("lane");
    assert_eq!(bank.gain_held(0, 0), Some(true), "muted");
    assert_eq!(bank.gain_held(3, 0), None, "a padding lane");
    assert_eq!(bank.gain_held(0, 2), None, "no third channel");
}

/// The scalar fader's lane queries answer `None` for a lane above `1`, as the bank's do for a
/// channel above `1`.
///
/// Red if a lane out of range wraps onto another lane (lane 2 read as lane 0's muted state).
#[test]
fn the_scalar_fader_refuses_a_lane_out_of_range() {
    let mut track = FaderMuteRampBuiltins::new(parameters(0.0, 0.0, false, false)).expect("fader");
    track
        .apply_fader_operation(
            BuiltinLaneSelector::Left,
            FaderOperation::SetMute {
                muted: true,
                samples: 0,
            },
        )
        .expect("a valid operation");
    assert_eq!(track.gain_held(0), Some(true), "muted");
    assert_eq!(track.gain_held(1), Some(false), "the right lane is free");
    assert_eq!(track.gain_held(2), None, "no third lane");
    assert_eq!(track.ramp_remaining(0), Some(0), "a mute with no ramp");
    assert_eq!(track.ramp_remaining(2), None, "no third lane");
}

/// Gate 5: 1,000 blocks of `process_with` with operations allocate and free nothing.
#[test]
fn process_with_operations_never_allocates() {
    assert_installed();
    static OPS: [Timed; 6] = [
        (
            0,
            0,
            FaderOperation::RetargetGain {
                gain: 0.3,
                samples: 40,
            },
        ),
        (
            5,
            1,
            FaderOperation::SetMute {
                muted: true,
                samples: 30,
            },
        ),
        (17, 0, FaderOperation::RememberGain { gain: 0.8 }),
        (64, 1, FaderOperation::SetGain { gain: 0.5 }),
        (
            64,
            0,
            FaderOperation::SetMute {
                muted: false,
                samples: 10,
            },
        ),
        (
            127,
            1,
            FaderOperation::SetMute {
                muted: false,
                samples: 0,
            },
        ),
    ];
    for (name, build) in Subject::subjects() {
        let mut subject = build();
        let width = subject.width();
        let mut left = vec![0.25_f32; 128 * width];
        let mut right = vec![-0.25_f32; 128 * width];
        // Warm once so every process static is initialised outside the window.
        subject.process_with(&mut left, &mut right, 0, &OPS);
        let mark = current_thread_counters();
        for block in 1..=1_000_u64 {
            subject.process_with(&mut left, &mut right, block * 128, &OPS);
        }
        let delta = current_thread_delta_since(mark);
        assert_eq!(delta.allocations, 0, "{name}: allocations");
        assert_eq!(delta.deallocations, 0, "{name}: frees");
    }
}

/// One fused fader-and-matrix subject: a settled fader bank with one padding lane and a matrix
/// bank of the same shape, so the fused settled path is eligible.
fn fused_pair(width: BankWidth) -> (BuiltinFaderBank, BuiltinMatrixBank) {
    let backend = width.backend();
    let members = width.lanes() as usize - 1;
    let faders = (0..members)
        .map(|lane| parameters(-2.0 - lane as f32, -5.0 + lane as f32, false, false))
        .collect();
    let matrices = (0..members)
        .map(|lane| {
            let k = lane as f32 * 0.0625;
            let matrix = Matrix2x2 {
                ll: 0.75 - k,
                lr: -0.3125 + k,
                rl: 0.5 - 0.5 * k,
                rr: -0.875 + k,
            };
            (matrix, 0)
        })
        .collect();
    (
        BuiltinFaderBank::new(backend, width, faders).expect("fader bank"),
        BuiltinMatrixBank::new(backend, width, matrices).expect("matrix bank"),
    )
}

/// Every member lane's fader and matrix state, as comparable text.
fn fused_words(fader: &BuiltinFaderBank, matrix: &BuiltinMatrixBank) -> String {
    (0..fader.width().lanes() as usize)
        .map(|lane| {
            format!(
                "{:?} {:?}",
                fader.export_lane(lane),
                matrix.export_lane(lane)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// D5: `try_process_settled_with_matrix_and_operations` declines, touching nothing, when an
/// operation sits inside the block, and otherwise forwards to the fused settled path. Checked
/// against a twin pair driven by the paths it chooses between.
#[test]
fn the_fused_settled_path_declines_only_an_operation_inside_the_block() {
    const FRAMES: u32 = 128;
    let held = FaderOperation::RetargetGain {
        gain: 0.3,
        samples: 40,
    };
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for (case, offset, declines) in [
            ("inside", Some(10_u32), true),
            ("at the block end", Some(FRAMES), false),
            ("none", None, false),
        ] {
            let ops: Vec<Timed> = offset.map(|at| (at, 0, held)).into_iter().collect();
            let (mut fader, mut matrix) = fused_pair(width);
            let (mut twin_fader, mut twin_matrix) = fused_pair(width);
            let (mut left, mut right) = signal(FRAMES as usize, lanes, 0);
            let (mut twin_left, mut twin_right) = (left.clone(), right.clone());
            let input = (bits(&left), bits(&right));
            let words = fused_words(&fader, &matrix);
            let mut source = Source {
                inner: SliceSource::new(&ops),
                scalar: false,
            };
            let fused = fader.try_process_settled_with_matrix_and_operations(
                &mut matrix,
                &mut left,
                &mut right,
                FRAMES,
                &mut source,
            );
            assert_eq!(fused, !declines, "width={lanes} {case}");
            assert_eq!(
                source.next_offset(),
                offset,
                "width={lanes} {case}: the source keeps its operation"
            );
            if declines {
                assert_eq!(
                    (bits(&left), bits(&right)),
                    input,
                    "width={lanes} {case}: a declined block is untouched"
                );
                assert_eq!(
                    fused_words(&fader, &matrix),
                    words,
                    "width={lanes} {case}: a decline changes no stage word"
                );
                // The caller then renders the block with `process_with`; so does the twin.
                fader.process_with(&mut left, &mut right, FRAMES, &mut source);
                matrix.process(&mut left, &mut right, FRAMES);
                let mut twin_source = Source {
                    inner: SliceSource::new(&ops),
                    scalar: false,
                };
                twin_fader.process_with(&mut twin_left, &mut twin_right, FRAMES, &mut twin_source);
                twin_matrix.process(&mut twin_left, &mut twin_right, FRAMES);
            } else {
                assert!(
                    twin_fader.try_process_settled_with_matrix(
                        &mut twin_matrix,
                        &mut twin_left,
                        &mut twin_right,
                        FRAMES,
                    ),
                    "width={lanes} {case}: the twin is settled"
                );
            }
            assert_eq!(bits(&left), bits(&twin_left), "width={lanes} {case}: left");
            assert_eq!(
                bits(&right),
                bits(&twin_right),
                "width={lanes} {case}: right"
            );
            assert_eq!(
                fused_words(&fader, &matrix),
                fused_words(&twin_fader, &twin_matrix),
                "width={lanes} {case}: state"
            );
        }
    }
}
