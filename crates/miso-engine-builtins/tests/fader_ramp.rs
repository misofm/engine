//! Issue #140 B: the live, ramped fader and mute section.
//!
//! Two properties matter and both are gated here.
//!
//! 1. **Bit identity with the prepared path when nothing is commanded.** A console-attached track
//!    that never receives a fader record must render exactly the bits `FaderMuteBuiltins` renders,
//!    including the exact `+0.0` a declared mute produces. That is what lets #140 add a ramped
//!    fader without moving a single builtins fixture digest.
//! 2. **Mute is a fader endpoint.** A mute with a window fades; a mute with no window is the
//!    instantaneous, exact `+0.0` the prepared path gives. Neither is a discontinuity the caller
//!    did not ask for.

use miso_engine_builtins::*;

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
