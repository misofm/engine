//! The segmented block driver reproduces `LinearRamp::next_value`, sample for sample.
//!
//! D11 is one law, and it lives in `effect-runtime`: divide once at the event, add per
//! sample, assign the target exactly on the final sample. The soft-clip kernel does not call
//! `next_value` — it adds a splatted increment inside the block — so the driver has to reproduce
//! the law rather than share the code, and this test is the join between the two.
//!
//! The observable is the drive gain, made visible by putting the effect in its identity-free path
//! and reading the state payload back after each block, plus a direct comparison of the first 64
//! rendered ramp values against an independently iterated `LinearRamp`.

mod support;

use effect_contract::ParameterChannel;
use effect_runtime::ramp::LinearRamp;
use math::db_to_gain_f32;
use support::{prepare, process, values_from, word, word_f32};

/// Payload words of the drive ramp under state layout 2.
const DRIVE_CURRENT: usize = 0;
const DRIVE_TARGET: usize = 1;
const DRIVE_STEP: usize = 2;
const DRIVE_REMAINING: usize = 3;

#[test]
fn the_driver_reproduces_the_runtime_ramp_law_bit_for_bit() {
    let values = values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut effect = prepare(&values);
    let target = db_to_gain_f32(18.0);

    let mut expected = LinearRamp::fixed(db_to_gain_f32(0.0));
    expected.set_target(target, 64);

    // One frame at a time, so every sample of the ramp is observed.
    let spans = [support::point(0, ParameterChannel::Left, 18.0, 0)];
    for sample in 0..70_u64 {
        let mut left = [0.0_f32; 1];
        let mut right = [0.0_f32; 1];
        let automation: &[_] = if sample == 0 { &spans } else { &[] };
        process(effect.as_mut(), &mut left, &mut right, sample, automation);
        let want = expected.next_value();
        let (_, payload, _) = support::snapshot(effect.as_ref());
        assert_eq!(
            word(&payload, DRIVE_CURRENT),
            want.to_bits(),
            "sample {sample}: driver {:e} vs LinearRamp {want:e}",
            word_f32(&payload, DRIVE_CURRENT)
        );
        assert_eq!(word(&payload, DRIVE_TARGET), target.to_bits());
        assert_eq!(word(&payload, DRIVE_STEP), expected.step.to_bits());
        assert_eq!(word(&payload, DRIVE_REMAINING), expected.remaining);
    }
    assert_eq!(expected.remaining, 0);
    assert_eq!(expected.current.to_bits(), target.to_bits());
}

/// The same law across arbitrary block sizes: only one division happens, at the event.
#[test]
fn the_ramp_divides_once_and_snaps_exactly_whatever_the_block_size() {
    for block in [1_usize, 5, 63, 64, 65, 128] {
        let values = values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]);
        let mut effect = prepare(&values);
        let target = db_to_gain_f32(-24.0);
        let spans = [support::point(0, ParameterChannel::Left, -24.0, 0)];
        let mut done = 0_usize;
        let mut expected = LinearRamp::fixed(db_to_gain_f32(0.0));
        expected.set_target(target, 64);
        while done < 128 {
            let span = block.min(128 - done);
            let mut left = vec![0.0_f32; span];
            let mut right = vec![0.0_f32; span];
            let automation: &[_] = if done == 0 { &spans } else { &[] };
            process(
                effect.as_mut(),
                &mut left,
                &mut right,
                done as u64,
                automation,
            );
            for _ in 0..span {
                let _ = expected.next_value();
            }
            let (_, payload, _) = support::snapshot(effect.as_ref());
            assert_eq!(
                word(&payload, DRIVE_CURRENT),
                expected.current.to_bits(),
                "block {block}, after {} frames",
                done + span
            );
            assert_eq!(word(&payload, DRIVE_REMAINING), expected.remaining);
            done += span;
        }
        assert_eq!(
            word_f32(&support::snapshot(effect.as_ref()).1, DRIVE_CURRENT).to_bits(),
            target.to_bits()
        );
    }
}

/// A new point restarts from wherever the ramp currently is (the brief's rule), and the step is
/// re-derived once from that value.
#[test]
fn a_new_point_restarts_from_the_current_value() {
    let values = values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut effect = prepare(&values);
    let mut left = vec![0.0_f32; 32];
    let mut right = vec![0.0_f32; 32];
    process(
        effect.as_mut(),
        &mut left,
        &mut right,
        0,
        &[support::point(0, ParameterChannel::Left, 18.0, 0)],
    );
    let (_, payload, _) = support::snapshot(effect.as_ref());
    let midpoint = word_f32(&payload, DRIVE_CURRENT);
    assert_eq!(word(&payload, DRIVE_REMAINING), 32);
    assert_ne!(midpoint.to_bits(), db_to_gain_f32(18.0).to_bits());

    let restart = db_to_gain_f32(-6.0);
    process(
        effect.as_mut(),
        &mut left,
        &mut right,
        32,
        &[support::point(0, ParameterChannel::Left, -6.0, 32)],
    );
    let (_, payload, _) = support::snapshot(effect.as_ref());
    let mut expected = LinearRamp::fixed(midpoint);
    expected.set_target(restart, 64);
    for _ in 0..32 {
        let _ = expected.next_value();
    }
    assert_eq!(word(&payload, DRIVE_STEP), expected.step.to_bits());
    assert_eq!(word(&payload, DRIVE_CURRENT), expected.current.to_bits());
    assert_eq!(word(&payload, DRIVE_REMAINING), 32);
}
