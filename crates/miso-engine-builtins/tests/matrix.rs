//! The smoothed 2x2 channel matrix: the D11 ramp law, its reference twin, and partition safety.
//!
//! D11 replaced the pre-#83 law, which divided by the remaining count on *every sample*, with one
//! division at the event and iterated additions. The two are not the same arithmetic for windows
//! longer than two samples; the trailing snap is what used to hide that. This file pins the new
//! law against `miso_engine_dsp_reference::ReferenceLinearRamp`, which is hand-written from D11.

use miso_engine_builtins::*;
use miso_engine_dsp_reference::ReferenceLinearRamp;

/// Reads the coefficient the next sample will be rendered with.
fn current(chain: &BuiltinChain) -> Matrix2x2 {
    test_support::matrix_current(test_support::chain_matrix(chain))
}

/// Renders one frame through the matrix section and returns the coefficients it used.
fn step(chain: &mut BuiltinChain) -> Matrix2x2 {
    let mut left = [0.0_f32];
    let mut right = [0.0_f32];
    chain.process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    current(chain)
}

fn chain_with(smoothing_samples: u32) -> BuiltinChain {
    BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples,
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare")
}

/// T8: every ramped coefficient equals the D11 reference, bit for bit, at every window length.
#[test]
fn matrix_ramp_matches_reference_d11_law() {
    for samples in [1_u32, 2, 8, 127, 128, 257, u32::MAX] {
        let mut chain = chain_with(samples);
        let target = Matrix2x2 {
            ll: 0.25,
            lr: -0.5,
            rl: 0.75,
            rr: -0.125,
        };
        chain.set_matrix_target(target).expect("target");
        let mut reference = [
            ReferenceLinearRamp::settled(1.0),
            ReferenceLinearRamp::settled(0.0),
            ReferenceLinearRamp::settled(0.0),
            ReferenceLinearRamp::settled(1.0),
        ];
        let targets = [target.ll, target.lr, target.rl, target.rr];
        for (ramp, value) in reference.iter_mut().zip(targets) {
            ramp.set_target(value, samples);
        }
        let frames = (samples as usize).min(600) + 4;
        for frame in 0..frames {
            let applied = step(&mut chain);
            let expected = reference.each_mut().map(ReferenceLinearRamp::next_value);
            for (actual, expected, name) in [
                (applied.ll, expected[0], "ll"),
                (applied.lr, expected[1], "lr"),
                (applied.rl, expected[2], "rl"),
                (applied.rr, expected[3], "rr"),
            ] {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "samples={samples}, frame={frame}, {name}"
                );
            }
        }
    }
}

/// T8: retargeting mid-ramp restarts the division from the value actually in flight.
#[test]
fn matrix_retarget_mid_ramp_matches_the_reference() {
    let mut chain = chain_with(128);
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 0.0,
            rl: 0.0,
            rr: 0.0,
        })
        .expect("target");
    let mut reference = ReferenceLinearRamp::settled(1.0);
    reference.set_target(0.0, 128);
    for _ in 0..37 {
        assert_eq!(
            step(&mut chain).ll.to_bits(),
            reference.next_value().to_bits()
        );
    }
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.5,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        })
        .expect("retarget");
    reference.set_target(0.5, 128);
    for frame in 0..200 {
        assert_eq!(
            step(&mut chain).ll.to_bits(),
            reference.next_value().to_bits(),
            "frame={frame}"
        );
    }
}

/// T8: the ramp tracks the exact `f64` interpolation it approximates.
///
/// The bound is derived, not chosen: `n` iterated `f32` additions each round once, so the drift is
/// at most `n * 2^-24` for coefficients of magnitude at most one -- `1.53e-5` at `n = 257`. The
/// measured worst is `1.12e-6`, a factor of 13.7 inside it. A per-sample division (the pre-D11
/// law) would not drift at all, which is exactly why this is a bound and not an equality.
#[test]
fn matrix_ramp_tracks_the_f64_interpolation() {
    const SAMPLES: u32 = 257;
    let mut chain = chain_with(SAMPLES);
    chain
        .set_matrix_target(Matrix2x2 {
            ll: -0.75,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        })
        .expect("target");
    let start = 1.0_f64;
    let step64 = (-0.75_f64 - start) / f64::from(SAMPLES);
    let mut worst = 0.0_f64;
    for index in 0..SAMPLES as usize {
        let applied = f64::from(step(&mut chain).ll);
        let exact = if index + 1 == SAMPLES as usize {
            -0.75
        } else {
            start + (index + 1) as f64 * step64
        };
        worst = worst.max((applied - exact).abs());
    }
    /// One half-ulp of `1.0` in `f32`.
    const HALF_ULP: f64 = 5.960_464_477_539_063e-8;
    assert!(worst <= f64::from(SAMPLES) * HALF_ULP, "worst={worst}");
}

/// T8: a settled identity matrix is a per-lane pass-through, so it preserves `-0.0`.
#[test]
fn settled_identity_matrix_preserves_signed_zero() {
    let mut chain = chain_with(0);
    let mut left = [-0.0_f32, 0.25];
    let mut right = [0.0_f32, -0.5];
    chain.process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
    assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(left[1].to_bits(), 0.25_f32.to_bits());
    assert_eq!(right[1].to_bits(), (-0.5_f32).to_bits());
}

/// A zero-length smoothing window is an assignment; `reset` cancels a ramp in flight.
#[test]
fn zero_window_snaps_and_reset_cancels_a_ramp() {
    let mut chain = chain_with(0);
    let target = Matrix2x2 {
        ll: 0.5,
        lr: 0.25,
        rl: -0.25,
        rr: 0.5,
    };
    chain.set_matrix_target(target).expect("target");
    assert_eq!(current(&chain), target);

    let mut chain = chain_with(64);
    chain.set_matrix_target(target).expect("target");
    step(&mut chain);
    assert_ne!(current(&chain), target);
    chain.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    assert_eq!(current(&chain), target);
}

#[test]
fn matrix_ramp_reaches_target() {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples: 2,
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 0.0,
            rl: 0.0,
            rr: 0.0,
        })
        .expect("target");
    let mut left = [1.0, 1.0];
    let mut right = [0.0, 0.0];
    chain.process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left, [0.5, 0.0]);
}

/// Issue #137 D1: an explicit ramp window travels with the retarget, and becomes the lane's own.
///
/// `matrix_ll/lr/rl/rr` are the only builtin parameters whose declared update rate is
/// `BuiltinParameterUpdateRate::BlockTarget`, so this is the one live builtin setter the parameter
/// ABI admits. The window and the target are one event because a console moves both together.
///
/// Red mutation: drop `self.smoothing_samples[lane] = samples;` from `MatrixStage::set_target_over`
/// -> the second retarget below runs over the prepared window of 0 instead of the requested 4, so
/// it settles on the first frame and `after_one` equals the target instead of a quarter of the way.
#[test]
fn explicit_window_retarget_ramps_over_the_requested_window_and_is_adopted() {
    let target = Matrix2x2 {
        ll: 0.5,
        lr: 0.25,
        rl: -0.25,
        rr: 0.75,
    };
    let mut chain = chain_with(0);
    let matrix = test_support::chain_matrix_mut(&mut chain);
    matrix
        .set_target_smoothed(target, 4)
        .expect("in-domain target");
    let after_one = step(&mut chain);
    assert_ne!(
        after_one, target,
        "a four-sample window does not settle in one"
    );
    for _ in 0..3 {
        step(&mut chain);
    }
    assert_eq!(
        current(&chain),
        target,
        "the window settles exactly on its last update"
    );

    // The window was adopted: the plain prepared-window setter now uses four samples too.
    let second = Matrix2x2 {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };
    test_support::chain_matrix_mut(&mut chain)
        .set_target(second)
        .expect("in-domain target");
    assert_ne!(step(&mut chain), second, "the adopted window still ramps");

    // A zero window is an immediate, unsmoothed jump, which is what a console asks for when the
    // session declared no pan smoothing.
    test_support::chain_matrix_mut(&mut chain)
        .set_target_smoothed(target, 0)
        .expect("in-domain target");
    assert_eq!(
        current(&chain),
        target,
        "a zero window settles at the event"
    );

    // The domain check is the same one the prepared path runs.
    assert!(
        test_support::chain_matrix_mut(&mut chain)
            .set_target_smoothed(
                Matrix2x2 {
                    ll: 2.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
                0,
            )
            .is_err(),
        "an out-of-domain coefficient is refused by the live setter too"
    );
}
