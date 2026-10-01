//! The stationary-smoother predicate and its signed-zero, subnormal and retarget boundaries.
//!
//! Expected values follow the current ramp contract: a stationary finite target rests immediately
//! and every subsequent update returns its exact word. Negative zero remains an armed D11 ramp.

use effect_runtime::ramp::LinearRamp;
use lane::kernels::RampSegment;

/// A settled ramp returns the supplied value, without advancing any state.
fn assert_constant(mut ramp: LinearRamp, expected: f32, count: usize) {
    assert_eq!(ramp.remaining, 0, "the stationary window must not arm");
    assert_eq!(ramp.step.to_bits(), 0);
    assert_eq!(ramp.current.to_bits(), expected.to_bits());
    assert_eq!(ramp.target.to_bits(), expected.to_bits());
    for update in 0..count {
        assert_eq!(
            ramp.next_value().to_bits(),
            expected.to_bits(),
            "update {update}"
        );
    }
    assert_eq!(ramp, LinearRamp::fixed(expected));
}

/// Redundant finite retargets settle immediately and keep every rendered word.
#[test]
fn a_redundant_retarget_is_constant_and_settled() {
    for value in [
        0.0_f32,
        1.0,
        -1.0,
        0.5,
        -24.0,
        1.0e-7,
        f32::MIN_POSITIVE,
        f32::MAX,
    ] {
        let mut ramp = LinearRamp::fixed(value);
        ramp.set_target(value, 64);
        assert_constant(ramp, value, 80);
    }
}

/// Negative zero arms: intermediate additions produce +0.0 and the final snap restores -0.0.
#[test]
fn negative_zero_remains_armed_and_snaps_back_to_its_sign() {
    assert!(!LinearRamp::stationary_at(-0.0, -0.0));
    assert!(LinearRamp::stationary_at(0.0, 0.0));

    let mut ramp = LinearRamp::fixed(-0.0);
    ramp.set_target(-0.0, 64);
    assert_eq!(ramp.remaining, 64, "the -0.0 window must still be armed");
    for update in 0..63 {
        assert_eq!(ramp.next_value().to_bits(), 0, "update {update}");
    }
    assert_eq!(ramp.next_value().to_bits(), (-0.0_f32).to_bits());
    assert_constant(ramp, -0.0, 2);
}

/// +0.0 and -0.0 are different targets, even though float comparison treats them as equal.
#[test]
fn signed_zeros_are_distinct_targets() {
    assert!(!LinearRamp::stationary_at(0.0, -0.0));
    assert!(!LinearRamp::stationary_at(-0.0, 0.0));

    let mut ramp = LinearRamp::fixed(0.0);
    ramp.set_target(-0.0, 32);
    assert_eq!(ramp.remaining, 32, "a sign flip is a real retarget");
}

/// Subnormal targets settle without flushing their words in the canonical FP environment.
#[test]
fn subnormal_targets_are_hoisted_without_flushing() {
    for value in [
        f32::from_bits(1),
        f32::from_bits(0x0000_0002),
        f32::from_bits(0x007f_ffff),
        -f32::from_bits(1),
    ] {
        assert!(LinearRamp::stationary_at(value, value));
        assert_eq!(
            (value + 0.0).to_bits(),
            value.to_bits(),
            "the canonical FP environment must not flush subnormals"
        );
        let mut ramp = LinearRamp::fixed(value);
        ramp.set_target(value, 64);
        assert_constant(ramp, value, 80);
    }
}

/// Non-finite values cannot be stationary: their ramp step is NaN rather than zero.
#[test]
fn non_finite_values_are_excluded() {
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(!LinearRamp::stationary_at(value, value));
    }
}

/// Retargeting to the value in force cancels an active flight and keeps that exact value.
#[test]
fn a_retarget_to_the_value_in_force_mid_ramp_settles() {
    for elapsed in [1_u32, 2, 17, 31, 63] {
        let mut ramp = LinearRamp::fixed(0.0);
        ramp.set_target(1.0, 64);
        for _ in 0..elapsed {
            ramp.next_value();
        }
        let live = ramp.current;
        ramp.set_target(live, 64);
        assert_constant(ramp, live, 96);
    }
}

/// A ramp ending at a block boundary produces a settled segment after a redundant retarget.
#[test]
fn a_window_that_closes_on_a_block_boundary_hoists_on_the_next_block() {
    for frames in [1_usize, 2, 64, 128] {
        let samples = u32::try_from(frames).expect("frame count fits");
        let mut ramp = LinearRamp::fixed(0.0);
        ramp.set_target(0.25, samples);
        let _: RampSegment<f32> = ramp.advance_block::<f32>(frames);
        assert_eq!(ramp.remaining, 0, "the window closes at the block boundary");
        assert_eq!(ramp.current.to_bits(), 0.25_f32.to_bits());

        ramp.set_target(0.25, 64);
        let segment: RampSegment<f32> = ramp.advance_block::<f32>(128);
        assert_eq!(segment.ramp_frames, 0);
        assert_eq!(segment.start.to_bits(), 0.25_f32.to_bits());
        assert_eq!(segment.target.to_bits(), 0.25_f32.to_bits());
        assert_eq!(segment.step.to_bits(), 0);
        assert_constant(ramp, 0.25, 2);
    }
}

/// The predicate must reject a moving value, even when it differs by only one ULP.
#[test]
fn a_one_ulp_move_is_never_hoisted() {
    for value in [1.0_f32, 0.5, -24.0, 1.0e-7] {
        let moved = f32::from_bits(value.to_bits() + 1);
        assert!(!LinearRamp::stationary_at(value, moved));
        let mut ramp = LinearRamp::fixed(value);
        ramp.set_target(moved, 64);
        assert_eq!(ramp.remaining, 64, "a one-ULP move must arm the window");
    }
}
