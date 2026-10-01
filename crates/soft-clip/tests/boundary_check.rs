#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E7 — D7: a failing lane's two output planes are zeroed and its histories cleared.
//!
//! A failed output boundary check snaps ramps to their targets and reports the block's frames
//! on both channels (#1073 owns the report unit). With default parameters the recovered scalar
//! instance matches a fresh one. The padded-bank tests cover lane-local recovery at every width.

mod support;

use support::{bits, initial_values, prepare, process};

const FRAMES: usize = 64;

fn signal(index: usize) -> f32 {
    (index as f32 * 0.037).sin() * 0.6
}

#[test]
fn a_non_finite_block_is_zeroed_reset_and_reports_its_frames() {
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
