//! Decision D7: one flush per recursive state word, one boundary check per block per bank.
//!
//! The pre-audit crate classified seven intermediates per lane-sample through `Option`, sanitised
//! every input sample and counted per-sample "recoveries". All of that is gone. What is left is the
//! `miso_engine_lane::flush` inside the follower and the
//! `miso_engine_effect_runtime::bank::finish_block` scan of the output.

mod common;

use common::*;
use miso_engine_effect_contract::{EffectBankProcessBlock, EffectProcessBlock, LinkMode};

/// A non-finite output zeroes the block and resets the envelopes.
///
/// This is the master plan §4.4 policy, applied by the shared driver: the whole block is zeroed and
/// the state reset, rather than the pre-audit per-sample "recover this lane and carry on".
///
/// Red mutation: skip `finish_block` after the frame loop — the NaN reaches the output.
#[test]
fn a_nonfinite_block_is_zeroed_and_the_envelopes_are_reset() {
    let mut effect = prepare(&values_of(1.0, -1.0, 1.0));
    let mut left = [0.5_f32; 8];
    let mut right = [0.25_f32; 8];
    effect
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("warm"));
    let warm = snapshot(effect.as_ref());
    assert!(state_f32(&warm.0, 0) > 0.0);

    let mut left = [0.5_f32; 8];
    let mut right = [0.25_f32; 8];
    left[3] = f32::NAN;
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 8, &[], 128).expect("nan block"),
    );
    for (index, sample) in left.iter().chain(right.iter()).enumerate() {
        assert_eq!(
            sample.to_bits(),
            0.0_f32.to_bits(),
            "every sample of a rejected block is +0.0 (index {index})"
        );
    }
    let after = snapshot(effect.as_ref());
    assert_eq!(state_f32(&after.0, 0).to_bits(), 0.0_f32.to_bits());
    assert_eq!(state_f32(&after.0, 1).to_bits(), 0.0_f32.to_bits());
    assert_eq!(state_f32(&after.1, 0).to_bits(), 0.0_f32.to_bits());
    assert_eq!(state_f32(&after.1, 1).to_bits(), 0.0_f32.to_bits());
    // A reset clears history, not automation: the parameter words survive.
    assert_eq!(&after.0[8..], &warm.0[8..]);

    // An out-of-range but finite output is rejected on the same threshold.
    let mut effect = prepare(&values_of(1.0, 0.0, 1.0));
    // 2e29 is inside the limit; the +18 dB attack boost takes the output to 1.6e30, past it.
    let mut left = [2.0e29_f32; 4];
    let mut right = [0.0_f32; 4];
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("huge block"),
    );
    assert!(
        left.iter().all(|x| x.to_bits() == 0.0_f32.to_bits()),
        "a block that reaches 1e30 is rejected: {left:?}"
    );
}

/// A bank rejects the whole bank block, which is the landed driver's semantics.
///
/// Master plan §4.4 says a failing block "zeroes its output, resets the failing effect's state";
/// 83c's `finish_block` applies that to the bank as a unit — both channels, every lane — because a
/// bank's two channels share their coefficients and their reset. This gate pins what the driver
/// actually does rather than what a per-lane reading of §4.4 would give; the per-lane isolation
/// variant is 83c's to decide, not this crate's.
#[test]
fn a_nonfinite_bank_block_is_rejected_as_a_unit() {
    let Some((_, width)) = native_bank() else {
        println!("no bank width on this build; skipping");
        return;
    };
    let lanes = width.lanes() as usize;
    let values = vec![values_of(1.0, -1.0, 1.0); lanes];
    let mut bank = bind_native_bank(&values, LinkMode::DualMono).expect("bank");
    let frames = 4;
    let mut left = vec![0.5_f32; frames * lanes];
    let mut right = vec![0.25_f32; frames * lanes];
    left[lanes + 2] = f32::NAN;
    let offsets = vec![0_u32; lanes + 1];
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            frames as u32,
            width,
            0,
            &[],
            &offsets,
            128,
        )
        .expect("bank block"),
    );
    assert!(
        left.iter()
            .chain(right.iter())
            .all(|x| x.to_bits() == 0.0_f32.to_bits())
    );
    let sizes =
        miso_engine_transient_shaper::TRANSIENT_SHAPER_DESCRIPTOR.qualities[1].maximum_state;
    for track in 0..lanes {
        let state = bank_snapshot(bank.as_ref(), track as u32, sizes);
        assert_eq!(state_f32(&state.0, 0).to_bits(), 0.0_f32.to_bits());
        assert_eq!(state_f32(&state.1, 1).to_bits(), 0.0_f32.to_bits());
    }
}

/// A subnormal input is no longer sanitised: it renders, and the envelope word it produces is
/// flushed to exactly `+0.0` by `miso_engine_lane::flush`.
///
/// The flush band (`|x| < 1e-20`) strictly contains the subnormal range, so a subnormal detector
/// can never enter the recurrence — and the shaper is still the identity on it, because two floored
/// envelopes divide to exactly `1` and `log2_lane(1)` is exactly `0`.
///
/// Red mutation: drop `flush` from `ar_one_pole_step` — the state word becomes subnormal.
#[test]
fn a_subnormal_input_renders_and_flushes_the_envelope() {
    let mut effect = prepare(&values_of(1.0, -1.0, 1.0));
    let mut left = [f32::from_bits(1); 16];
    let mut right = [-f32::from_bits(3); 16];
    let original_left = left.map(f32::to_bits);
    let original_right = right.map(f32::to_bits);
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("subnormal block"),
    );
    // Both envelopes floor, contrast is exactly zero, so the shape-zero identity select returns the
    // input bits unchanged -- including the sign of the negative subnormal.
    assert_eq!(left.map(f32::to_bits), original_left);
    assert_eq!(right.map(f32::to_bits), original_right);
    let state = snapshot(effect.as_ref());
    for (section, word) in [(&state.0, 0), (&state.0, 1), (&state.1, 0), (&state.1, 1)] {
        assert_eq!(
            state_f32(section, word).to_bits(),
            0.0_f32.to_bits(),
            "envelope word {word} must flush to +0.0, not carry a subnormal"
        );
    }
}
