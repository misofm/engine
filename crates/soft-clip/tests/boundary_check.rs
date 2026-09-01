#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E7 — D7: no per-value checking, one boundary check per block per bank.
//!
//! The audit found 258 per-operation store/`is_finite`/`is_subnormal`/reload wrappers per input
//! sample and per channel, costing about 95 % of the kernel's time (issue #91 F1). They are gone.
//! What replaces them is master plan §4.4: the block's output is scanned once with a vector
//! compare, a failing block is zeroed, the state is reset and a counter advances by one *block*.
//!
//! The tests below are the behaviour that replaced the old per-lane recovery: a non-finite input
//! is no longer swallowed sample by sample, it fails its block; and after a failure the effect is
//! exactly a fresh instance, so nothing of the bad state survives into the next block.

mod support;

use effect_contract::BankWidth;
use support::{bank_available, bits, initial_values, prepare, prepare_bank, process, process_bank};

const FRAMES: usize = 64;

fn signal(index: usize) -> f32 {
    (index as f32 * 0.037).sin() * 0.6
}

#[test]
fn a_non_finite_block_is_zeroed_reset_and_counted_once() {
    let values = initial_values();
    let mut effect = prepare(&values);

    // Warm the histories so a reset is observable.
    let mut left: Vec<f32> = (0..FRAMES).map(signal).collect();
    let mut right = left.clone();
    let report = process(effect.as_mut(), &mut left, &mut right, 0, &[]);
    assert_eq!(report, Default::default());

    // One NaN anywhere in the block fails the whole block.
    let mut left: Vec<f32> = (0..FRAMES).map(|index| signal(index + FRAMES)).collect();
    let mut right = left.clone();
    left[17] = f32::NAN;
    let report = process(effect.as_mut(), &mut left, &mut right, FRAMES as u64, &[]);
    assert!(
        left.iter().all(|sample| sample.to_bits() == 0),
        "a rejected block zeroes its left output"
    );
    assert!(
        right.iter().all(|sample| sample.to_bits() == 0),
        "and its right output, because the two share a reset"
    );
    assert_eq!(report.nonfinite_left_blocks, FRAMES as u64);
    assert_eq!(report.nonfinite_right_blocks, FRAMES as u64);
    assert_eq!(report.sanitized_main_samples, 0, "D7: nothing is sanitised");

    // After the reset the effect is a fresh instance, bit for bit.
    let mut fresh = prepare(&values);
    let mut recovered_left: Vec<f32> = (0..FRAMES).map(signal).collect();
    let mut recovered_right = recovered_left.clone();
    let mut fresh_left = recovered_left.clone();
    let mut fresh_right = recovered_left.clone();
    process(
        effect.as_mut(),
        &mut recovered_left,
        &mut recovered_right,
        2 * FRAMES as u64,
        &[],
    );
    process(fresh.as_mut(), &mut fresh_left, &mut fresh_right, 0, &[]);
    assert_eq!(bits(&recovered_left), bits(&fresh_left));
    assert_eq!(bits(&recovered_right), bits(&fresh_right));
    assert_eq!(
        support::snapshot(effect.as_ref()),
        support::snapshot(fresh.as_ref())
    );
}

/// A value that is finite but enormous fails the block too: the check is `|x| < 1e30`, not `x == x`.
#[test]
fn a_finite_but_out_of_range_block_also_fails() {
    // mix = 0 and output = 1 is the exact identity select, so the dry sample reaches the output
    // untouched 31 samples later -- the only way a *finite* out-of-range value can be observed,
    // since the cubic clamps the wet path to +-2/3.
    let values = support::values_from([(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)]);
    let mut effect = prepare(&values);
    let mut left = vec![0.0_f32; FRAMES];
    let mut right = vec![0.0_f32; FRAMES];
    left[0] = 1.0e35;
    let report = process(effect.as_mut(), &mut left, &mut right, 0, &[]);
    assert!(left.iter().all(|sample| sample.to_bits() == 0));
    assert_eq!(report.nonfinite_left_blocks, FRAMES as u64);
}

/// A bank counts the failure once per block for every lane, and comes back as a fresh bank.
#[test]
fn a_bank_block_fails_and_recovers_as_a_unit() {
    let width = if bank_available(BankWidth::Eight) {
        BankWidth::Eight
    } else if bank_available(BankWidth::Four) {
        BankWidth::Four
    } else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = initial_values();
    let per_lane: Vec<Vec<_>> = (0..lanes).map(|_| values.to_vec()).collect();
    let mut bank = prepare_bank(width, &per_lane).expect("bank binds");
    let offsets = vec![0_u32; lanes + 1];

    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut right = vec![0.0_f32; FRAMES * lanes];
    for frame in 0..FRAMES {
        for lane in 0..lanes {
            left[frame * lanes + lane] = signal(frame + lane);
            right[frame * lanes + lane] = signal(frame + lane + 5);
        }
    }
    process_bank(
        bank.as_mut(),
        width,
        &mut left,
        &mut right,
        FRAMES,
        0,
        &[],
        &offsets,
    );

    let mut left = vec![0.1_f32; FRAMES * lanes];
    let mut right = vec![0.1_f32; FRAMES * lanes];
    left[3 * lanes + 2] = f32::INFINITY;
    let report = process_bank(
        bank.as_mut(),
        width,
        &mut left,
        &mut right,
        FRAMES,
        FRAMES as u64,
        &[],
        &offsets,
    );
    assert!(left.iter().all(|sample| sample.to_bits() == 0));
    assert!(right.iter().all(|sample| sample.to_bits() == 0));
    for lane in 0..lanes {
        assert_eq!(report.reports[lane].nonfinite_left_blocks, FRAMES as u64);
        assert_eq!(report.reports[lane].nonfinite_right_blocks, FRAMES as u64);
    }

    let fresh = prepare_bank(width, &per_lane).expect("bank binds");
    for lane in 0..lanes {
        assert_eq!(
            support::snapshot_bank(bank.as_ref(), lane as u32),
            support::snapshot_bank(fresh.as_ref(), lane as u32),
            "lane {lane} is not a fresh bank after the reset"
        );
    }
}
