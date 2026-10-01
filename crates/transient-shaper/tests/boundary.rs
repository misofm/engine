//! Decision D7: one flush per recursive state word, one boundary check per block per bank.
//!
//! The pre-audit crate classified seven intermediates per lane-sample through `Option`, sanitised
//! every input sample and counted per-sample "recoveries". All of that is gone. What is left is the
//! `lane::flush` inside the follower and one `effect_runtime::bank::check_block` scan of the output
//! per block, whose failing lanes are recovered one lane at a time (issue #1092).

mod common;

use common::*;
use effect_contract::EffectProcessBlock;

/// A non-finite output zeroes the block and resets the envelopes.
///
/// This is the master plan §4.4 policy: the scalar instance's one lane fails, so its whole block is
/// zeroed and its state reset, rather than the pre-audit per-sample "recover this lane and carry
/// on".
///
/// Red mutation: skip the boundary check after the frame loop — the NaN reaches the output.
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

/// A subnormal input is no longer sanitised: it renders, and the envelope word it produces is
/// flushed to exactly `+0.0` by `lane::flush`.
///
/// The flush band (`|x| < 1e-20`) strictly contains the subnormal range, so a subnormal detector
/// can never enter the recurrence — and the shaper is still the identity on it, because two floored
/// envelopes divide to exactly `1` and `fast_level_db(1)` is exactly `0`.
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
