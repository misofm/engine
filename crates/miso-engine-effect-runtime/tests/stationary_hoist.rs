//! Issue #144 item 6: the stationary-smoother bitwise hoist, and the exact boundary of it.
//!
//! The hoist skips a smoothing window whose every increment is `+0.0`. The bar is bit-identity,
//! not nearness, so every test here compares `to_bits` and every case is chosen because it is a
//! place where "obviously the same" and "the same bits" could come apart.
//!
//! The control arm is written out by hand — `armed_sequence` is what `set_target` did before the
//! hoist — so each test compares the hoisted path against the arithmetic it replaced rather than
//! against itself.

use miso_engine_effect_contract::{ParameterSmoother, SmoothingRule};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_lane::kernels::RampSegment;

/// The pre-hoist `set_target`: always divide, always arm. This is the control arm.
fn arm_unhoisted(ramp: &mut LinearRamp, target: f32, samples: u32) {
    ramp.target = target;
    if samples == 0 {
        ramp.current = target;
        ramp.step = 0.0;
        ramp.remaining = 0;
        return;
    }
    ramp.step = (target - ramp.current) / samples as f32;
    ramp.remaining = samples;
}

fn sequence(mut ramp: LinearRamp, count: usize) -> Vec<u32> {
    (0..count).map(|_| ramp.next_value().to_bits()).collect()
}

/// Both arms must render the same bits for the whole window and land in the same resting state.
fn assert_arms_agree(start: f32, target: f32, samples: u32, count: usize, what: &str) {
    let mut hoisted = LinearRamp::fixed(start);
    hoisted.set_target(target, samples);

    let mut control = LinearRamp::fixed(start);
    arm_unhoisted(&mut control, target, samples);

    assert_eq!(
        sequence(hoisted, count),
        sequence(control, count),
        "{what}: rendered bits differ between the hoisted and armed arms"
    );

    let mut hoisted_state = hoisted;
    let mut control_state = control;
    for _ in 0..count {
        hoisted_state.next_value();
        control_state.next_value();
    }
    assert_eq!(
        hoisted_state.current.to_bits(),
        control_state.current.to_bits(),
        "{what}: resting value differs"
    );
    assert_eq!(
        hoisted_state.target.to_bits(),
        control_state.target.to_bits(),
        "{what}: resting target differs"
    );
    assert_eq!(
        hoisted_state.remaining, 0,
        "{what}: hoisted arm never rests"
    );
    assert_eq!(control_state.remaining, 0, "{what}: armed arm never rests");
}

/// The case the optimisation exists for: a retarget to the value already in force.
#[test]
fn a_redundant_retarget_renders_the_same_bits_as_the_window_it_skips() {
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
        assert_arms_agree(
            value,
            value,
            64,
            80,
            &format!("redundant retarget to {value}"),
        );
        let mut ramp = LinearRamp::fixed(value);
        ramp.set_target(value, 64);
        assert_eq!(
            ramp.remaining, 0,
            "the window must not be armed for {value}"
        );
        assert_eq!(ramp.step.to_bits(), 0.0_f32.to_bits());
    }
}

/// **Negative zero is not hoisted, and the test says why.**
///
/// `-0.0 + 0.0` is `+0.0`, so the armed window renders `+0.0` for every sample but the last, which
/// the D11 snap returns to `-0.0`. A skipped window would render `-0.0` throughout. The two arms
/// genuinely differ, so the predicate excludes it and this test pins that they *do* differ — a
/// hoist that swallowed `-0.0` would silently change rendered bits.
#[test]
fn negative_zero_is_excluded_because_the_two_arms_really_do_differ() {
    assert!(!LinearRamp::stationary_at(-0.0, -0.0));
    assert!(LinearRamp::stationary_at(0.0, 0.0));

    let mut ramp = LinearRamp::fixed(-0.0);
    ramp.set_target(-0.0, 64);
    assert_eq!(ramp.remaining, 64, "the -0.0 window must still be armed");

    let armed = sequence(ramp, 64);
    assert_eq!(armed[0], 0.0_f32.to_bits(), "-0.0 + 0.0 is +0.0");
    assert_eq!(
        armed[63],
        (-0.0_f32).to_bits(),
        "the D11 snap restores -0.0 on the final sample"
    );
    assert_ne!(
        armed[0], armed[63],
        "if these ever agree the exclusion can be revisited"
    );
}

/// `+0.0` and `-0.0` are different targets, and the bit compare is what tells them apart.
#[test]
fn signed_zeros_are_distinct_targets() {
    assert!(!LinearRamp::stationary_at(0.0, -0.0));
    assert!(!LinearRamp::stationary_at(-0.0, 0.0));

    let mut ramp = LinearRamp::fixed(0.0);
    ramp.set_target(-0.0, 32);
    assert_eq!(ramp.remaining, 32, "a sign flip is a real retarget");
}

/// Subnormal targets hoist, and the arms agree — the property that depends on #146's environment.
///
/// `d + 0.0 == d` exactly for every subnormal `d` under the canonical floating-point environment
/// every render entry installs. Under a host's FTZ/DAZ the addition would flush and the armed arm
/// would render `+0.0` where the hoisted arm renders `d`; that is why the guard is load-bearing
/// for this optimisation rather than incidental to it.
#[test]
fn subnormal_targets_are_hoisted_and_both_arms_agree() {
    let subnormals = [
        f32::from_bits(1),
        f32::from_bits(0x0000_0002),
        f32::from_bits(0x007f_ffff),
        -f32::from_bits(1),
    ];
    for value in subnormals {
        assert!(value.is_subnormal(), "{value:e} must be subnormal");
        assert!(LinearRamp::stationary_at(value, value));
        assert_eq!(
            (value + 0.0).to_bits(),
            value.to_bits(),
            "the canonical FP environment must not be flushing subnormals"
        );
        assert_arms_agree(value, value, 64, 80, &format!("subnormal {value:e}"));
    }
}

/// Non-finite values are excluded: the step is `NaN`, not zero, so there is no no-op to skip.
#[test]
fn non_finite_values_are_excluded() {
    assert!(!LinearRamp::stationary_at(f32::NAN, f32::NAN));
    assert!(!LinearRamp::stationary_at(f32::INFINITY, f32::INFINITY));
    assert!(!LinearRamp::stationary_at(
        f32::NEG_INFINITY,
        f32::NEG_INFINITY
    ));
    assert!((f32::INFINITY - f32::INFINITY).is_nan());
}

/// **The smoother is mid-ramp when the hoist check runs.**
///
/// A retarget to the value in force *right now* is a no-op window whether or not an earlier ramp
/// is still in flight, because `set_target` re-derives the step from `current`. Cancelling the
/// flight is exactly where the armed arm ends up when its window closes.
#[test]
fn a_retarget_to_the_value_in_force_mid_ramp_agrees_with_the_armed_arm() {
    for elapsed in [1_u32, 2, 17, 31, 63] {
        let mut hoisted = LinearRamp::fixed(0.0);
        hoisted.set_target(1.0, 64);
        for _ in 0..elapsed {
            hoisted.next_value();
        }
        let live = hoisted.current;

        let mut control = hoisted;
        hoisted.set_target(live, 64);
        arm_unhoisted(&mut control, live, 64);

        assert_eq!(
            hoisted.remaining, 0,
            "the in-flight window must be cancelled after {elapsed} samples"
        );
        assert_eq!(
            sequence(hoisted, 96),
            sequence(control, 96),
            "mid-ramp retarget to the live value diverged after {elapsed} samples"
        );
    }
}

/// **The target is reached exactly at a block boundary.**
///
/// A window of exactly `frames` samples closes on the last sample of the block. The next block
/// starts at rest, so a retarget arriving then is a redundant one and must hoist without moving a
/// bit relative to the armed arm.
#[test]
fn a_window_that_closes_on_a_block_boundary_hoists_on_the_next_block() {
    for frames in [1_usize, 2, 64, 128] {
        let samples = u32::try_from(frames).expect("frame count fits");
        let mut hoisted = LinearRamp::fixed(0.0);
        hoisted.set_target(0.25, samples);
        let _: RampSegment<f32> = hoisted.advance_block::<f32>(frames);

        assert_eq!(
            hoisted.remaining, 0,
            "a {frames}-sample window must close on the block boundary"
        );
        assert_eq!(hoisted.current.to_bits(), 0.25_f32.to_bits());

        let mut control = hoisted;
        hoisted.set_target(0.25, 64);
        arm_unhoisted(&mut control, 0.25, 64);
        assert_eq!(hoisted.remaining, 0, "the boundary retarget must hoist");

        let hoisted_segment: RampSegment<f32> = hoisted.advance_block::<f32>(128);
        let control_segment: RampSegment<f32> = control.advance_block::<f32>(128);
        assert_eq!(hoisted_segment.ramp_frames, 0);
        assert_eq!(
            hoisted_segment.start.to_bits(),
            control_segment.start.to_bits(),
            "segment start diverged at a {frames}-frame boundary"
        );
        assert_eq!(
            hoisted_segment.target.to_bits(),
            control_segment.target.to_bits()
        );
    }
}

/// **A parameter change arrives mid-block**, splitting the block around it.
///
/// Driving the ramp block by block with a redundant retarget landing between two blocks must
/// produce the same bits as the armed arm, at every split point.
#[test]
fn a_redundant_change_arriving_mid_block_is_partition_invariant() {
    for split in [1_usize, 7, 63, 64, 65, 127] {
        let mut hoisted = LinearRamp::fixed(0.0);
        hoisted.set_target(0.75, 64);
        let mut control = LinearRamp::fixed(0.0);
        arm_unhoisted(&mut control, 0.75, 64);

        let mut hoisted_bits = Vec::new();
        let mut control_bits = Vec::new();
        for index in 0..256_usize {
            if index == split {
                let live = hoisted.current;
                assert_eq!(live.to_bits(), control.current.to_bits(), "arms pre-split");
                hoisted.set_target(live, 64);
                arm_unhoisted(&mut control, live, 64);
            }
            hoisted_bits.push(hoisted.next_value().to_bits());
            control_bits.push(control.next_value().to_bits());
        }
        assert_eq!(
            hoisted_bits, control_bits,
            "a redundant change at sample {split} moved rendered bits"
        );
    }
}

/// The hoist must never fire when the value is genuinely moving, however small the move.
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

/// The contract-side twin moves in step for `Linear`, and deliberately does not for `OnePole99`.
#[test]
fn the_contract_smoother_hoists_linear_and_refuses_one_pole() {
    let mut linear =
        ParameterSmoother::new(0.25, SmoothingRule::Linear, 64).expect("linear smoother");
    assert!(linear.set_target(0.25));
    assert_eq!(
        linear.next_value().to_bits(),
        0.25_f32.to_bits(),
        "a hoisted linear smoother rests at its value"
    );
    assert_eq!(linear.step().to_bits(), 0.0_f32.to_bits());

    // `OnePole99` computes `a * current + k * target`: two products, two roundings, and no
    // guarantee of returning `current` unchanged. Hoisting it would be a numeric change, so the
    // window stays armed and the value is still produced by the one-pole law.
    let mut one_pole =
        ParameterSmoother::new(0.25, SmoothingRule::OnePole99, 64).expect("one-pole smoother");
    assert!(one_pole.set_target(0.25));
    let first = one_pole.next_value();
    assert!(
        first.is_finite(),
        "the one-pole arm must still run its own law"
    );
}

/// The rest invariant survives the hoist: `remaining == 0` still implies `current == target`.
#[test]
fn the_rest_invariant_survives_the_hoist() {
    let mut ramp = LinearRamp::fixed(0.0);
    for (target, samples) in [
        (0.0_f32, 64_u32),
        (1.0, 64),
        (1.0, 64),
        (1.0, 0),
        (-2.5, 5),
        (-2.5, 64),
    ] {
        ramp.set_target(target, samples);
        for _ in 0..samples + 2 {
            ramp.next_value();
            if ramp.remaining == 0 {
                assert_eq!(ramp.current.to_bits(), ramp.target.to_bits());
            }
        }
        assert_eq!(ramp.remaining, 0);
        assert_eq!(ramp.current.to_bits(), target.to_bits());
    }
}
