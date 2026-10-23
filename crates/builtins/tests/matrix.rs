//! The smoothed 2x2 channel matrix: the D11 ramp law, its reference twin, and partition safety.
//!
//! D11 replaced the pre-#83 law, which divided by the remaining count on *every sample*, with one
//! division at the event and iterated additions. The two are not the same arithmetic for windows
//! longer than two samples; the trailing snap is what used to hide that. This file pins the new
//! law against `dsp_reference::ReferenceLinearRamp`, which is hand-written from D11.

use builtins::*;
use dsp_reference::ReferenceLinearRamp;

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

/// Issue #137 D1: an explicit ramp window travels with the retarget, and becomes the lane's own.
///
/// `matrix_ll/lr/rl/rr` are the only builtin parameters whose declared update rate is
/// `BuiltinParameterUpdateRate::BlockTarget`, so this is the one live builtin setter the parameter
/// ABI admits. The window and the target are one event because live controls move both together.
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

    // A zero window is an immediate, unsmoothed jump, which is what live controls ask for when the
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

/// Issue #1408 gate 3: no matrix word passes its target.
///
/// Found by a scalar simulation of the unclamped D11 law (see the issue's attempt record): a
/// one-second identity-to-swap move at 48 kHz carries the `1 -> 0` words to `-0.000624` before
/// their snap. Inputs `(1, 0)` and `(0, 1)` read the four applied words out directly
/// (`yl = ll`, `yr = rl`, then `yl = lr`, `yr = rr`).
///
/// Red mutation: revert `matrix2x2_ramp_block`'s step 3 to `current + step` -> `ll` and `rr`
/// leave `[0, 1]`.
#[test]
fn a_long_matrix_move_stays_inside_its_endpoints() {
    const WINDOW: u32 = 48_000;
    const BLOCK: usize = 4_096;
    let swap = Matrix2x2 {
        ll: 0.0,
        lr: 1.0,
        rl: 1.0,
        rr: 0.0,
    };
    let start = Matrix2x2::IDENTITY;
    let mut first = chain_with(WINDOW);
    let mut second = chain_with(WINDOW);
    first.set_matrix_target(swap).expect("target");
    second.set_matrix_target(swap).expect("target");
    let frames = WINDOW as usize + 64;
    let mut rendered = 0;
    while rendered < frames {
        let block = BLOCK.min(frames - rendered);
        let mut ll = vec![1.0_f32; block];
        let mut rl = vec![0.0_f32; block];
        first.process_matrix(DualMonoBlock::new(&mut ll, &mut rl, 0).expect("block"));
        let mut lr = vec![0.0_f32; block];
        let mut rr = vec![1.0_f32; block];
        second.process_matrix(DualMonoBlock::new(&mut lr, &mut rr, 0).expect("block"));
        for (name, words, from, to) in [
            ("ll", &ll, start.ll, swap.ll),
            ("lr", &lr, start.lr, swap.lr),
            ("rl", &rl, start.rl, swap.rl),
            ("rr", &rr, start.rr, swap.rr),
        ] {
            let (low, high) = (from.min(to), from.max(to));
            for (index, word) in words.iter().enumerate() {
                assert!(
                    (low..=high).contains(word),
                    "{name} at frame {}: {word:e} outside [{low:e}, {high:e}]",
                    rendered + index
                );
            }
            if rendered + block == frames {
                assert_eq!(words[block - 1], to, "{name} settles at its target");
            }
        }
        rendered += block;
    }
}

/// Issue #1072: a banked lane on the identity renders the bits it renders alone while another
/// lane's ramp keeps the bank on the ramp kernel: `-0.0` passes through on every frame after the
/// lane's own ramp ends, and the ramp's last frame is still the arithmetic `1 * -0.0 + 0 * +0.0`,
/// which is `+0.0`.
///
/// Lane 0 ramps onto the identity over two frames, lane 1 ramps for the whole block, and lane 2
/// sits on the identity throughout. Each lane must equal its own `MatrixBuiltins` word for word,
/// and the signed zeros are also spelled out, so a bank and a scalar wrong the same way stay red.
#[test]
fn a_banked_identity_lane_keeps_signed_zero_while_another_lane_ramps() {
    const FRAMES: usize = 8;
    let start = Matrix2x2 {
        ll: 0.5,
        lr: 0.5,
        rl: 0.5,
        rr: 0.5,
    };
    let other = Matrix2x2 {
        ll: -1.0,
        lr: 0.0,
        rl: 0.0,
        rr: -1.0,
    };
    let prepared = [start, start, Matrix2x2::IDENTITY];
    let retargets = [
        (Matrix2x2::IDENTITY, 2),
        (other, 64),
        (Matrix2x2::IDENTITY, 0),
    ];
    let (negative, positive) = ((-0.0_f32).to_bits(), 0.0_f32.to_bits());
    for &width in effect_contract::BankWidth::ALL {
        let count = width.lanes() as usize;
        let mut bank = BuiltinMatrixBank::new(
            width.backend(),
            width,
            prepared.iter().map(|matrix| (*matrix, 0)).collect(),
        )
        .expect("bank");
        for (lane, (target, window)) in retargets.iter().enumerate() {
            bank.set_target_smoothed(lane, *target, *window)
                .expect("retarget");
        }
        let mut left = vec![-0.0_f32; FRAMES * count];
        let mut right = vec![0.0_f32; FRAMES * count];
        bank.process(&mut left, &mut right, FRAMES as u32);
        for (lane, (matrix, (target, window))) in prepared.iter().zip(&retargets).enumerate() {
            let mut alone = BuiltinChain::new(
                48_000,
                BuiltinParameters {
                    matrix: *matrix,
                    ..BuiltinParameters::default()
                },
            )
            .expect("prepare")
            .into_sections()
            .2;
            alone
                .set_target_smoothed(*target, *window)
                .expect("retarget");
            let mut l = [-0.0_f32; FRAMES];
            let mut r = [0.0_f32; FRAMES];
            alone.process(DualMonoBlock::new(&mut l, &mut r, 0).expect("block"));
            for frame in 0..FRAMES {
                let word = frame * count + lane;
                let context = format!("W{count} lane {lane} frame {frame}");
                assert_eq!(left[word].to_bits(), l[frame].to_bits(), "{context} left");
                assert_eq!(right[word].to_bits(), r[frame].to_bits(), "{context} right");
                let pass = match lane {
                    0 => frame >= 2,
                    1 => false,
                    _ => true,
                };
                if pass {
                    assert_eq!(left[word].to_bits(), negative, "{context}: -0.0 passes");
                } else if lane == 0 && frame == 1 {
                    assert_eq!(
                        left[word].to_bits(),
                        positive,
                        "{context}: the last ramp frame"
                    );
                }
            }
        }
    }
}
