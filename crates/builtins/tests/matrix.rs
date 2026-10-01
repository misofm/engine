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

/// One settled-matrix shape of issue #944's gate 3.
#[derive(Clone, Copy, Debug)]
enum SettledShape {
    /// (a) Every member lane non-identity.
    NonIdentity,
    /// (b) One member exactly [`Matrix2x2::IDENTITY`], the rest non-identity.
    OneIdentityMember,
    /// (c) Every member non-identity; member 0 retargets at block 3 over 200 samples to another
    /// non-identity matrix, so its ramp ends 72 frames into block 4 and the tail is settled.
    RetargetToPan,
    /// (d) As (c), but member 0 retargets to [`Matrix2x2::IDENTITY`].
    RetargetToIdentity,
}

/// A non-identity matrix per lane: distinct, signed, and asymmetric (`lr != rl`), so a swapped
/// coefficient moves a word. Even lanes take the equal-power pan law, whose `cos(pi / 2)` is
/// `6.1e-17` and never `0.0`.
fn non_identity(lane: usize) -> Matrix2x2 {
    if lane.is_multiple_of(2) {
        let position = -0.875 + 0.25 * lane as f32;
        pan_matrix(position, -position * 0.5).expect("pan law")
    } else {
        let k = lane as f32 * 0.0625;
        Matrix2x2 {
            ll: 0.75 - k,
            lr: -0.3125 + k,
            rl: 0.5 - 0.5 * k,
            rr: -0.875 + k,
        }
    }
}

/// SplitMix64: a deterministic word stream with no dependency on the platform.
fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// One hostile input pair for `frame` of a block.
///
/// Frames `0, 1, 2` of every 16 put a signed zero on **every** lane of both planes, including the
/// `l = -0.0, r = +0.0` and `l = +0.0, r = -0.0` pairs that only an identity lane passes through
/// unchanged (`1 * -0.0 + 0 * +0.0` is `+0.0`). Frames `3` and `4` are subnormal; every other frame
/// draws a sign, a mantissa and an exponent in `2^-24 ..= 2^25`. No word is non-finite: the input
/// stage sanitises before the matrix in every plan, and a NaN's payload order is not part of the
/// class-A statement (issue #944, amendment 1).
fn hostile_pair(frame: usize, state: &mut u64) -> (f32, f32) {
    match frame % 16 {
        0 => (-0.0, 0.0),
        1 => (0.0, -0.0),
        2 => (-0.0, -0.0),
        3 => {
            let bits = splitmix(state);
            (
                f32::from_bits(1 + (bits as u32 & 0x007f_fffe)),
                -f32::from_bits(1 + ((bits >> 32) as u32 & 0x007f_fffe)),
            )
        }
        4 => (-f32::from_bits(1), f32::MIN_POSITIVE * 0.5),
        _ => {
            let mut word = || {
                let bits = splitmix(state);
                let sign = (bits as u32) & 0x8000_0000;
                let mantissa = (bits >> 8) as u32 & 0x007f_ffff;
                let exponent = (-24 + ((bits >> 40) % 50) as i32 + 127) as u32;
                f32::from_bits(sign | (exponent << 23) | mantissa)
            };
            (word(), word())
        }
    }
}

/// Every bank width this build has: both in the 8-lane (AVX2) build, and four alone in a 4-lane
/// (NEON/simd128) build, which has no eight-lane type (issue #1112). The two scenario gates below
/// fold every width into one digest, so each pins one digest per build.
const BANK_WIDTHS: &[(lane::Backend, effect_contract::BankWidth)] = &[
    (lane::Backend::Simd4, effect_contract::BankWidth::Four),
    #[cfg(target_feature = "avx2")]
    (lane::Backend::Simd8, effect_contract::BankWidth::Eight),
];

/// Issue #944 gate 3: every settled-matrix shape renders the bits the base commit rendered.
///
/// One deterministic scenario, 24 blocks of 128 frames, through [`BuiltinMatrixBank`] at both bank
/// widths with `{1, W - 1, W}` members, over the four [`SettledShape`]s. Every output word and
/// every block's per-lane retained words (padding lanes included) are folded into one SHA-256,
/// pinned at the value the unmodified base (`21a0dfcf`, #936 and #945 on `14f2917b`) printed, in
/// both dev and release, before the select-free arm existed. Only a full non-identity bank takes that arm; partial banks carry
/// identity padding lanes and keep the select, and so do shapes (b) and (d).
///
/// It also pins the post-ramp settle directly: after the block in which member 0's ramp ends, its
/// current words equal its target words bitwise and its `u32` countdown is `0`, so the settled tail
/// that follows runs on the exact target.
///
/// Red mutations (issue #944, `MUTATIONS.md`): M1, the settled arm always select-free, moves the
/// digest through the identity lanes' `-0.0`; M3, `lr` and `rl` swapped in the select-free kernel,
/// moves it through shape (a).
#[test]
fn settled_matrix_shapes_render_the_base_bits() {
    // One digest per build (see `BANK_WIDTHS`): the 4-lane pin is this scenario at four lanes alone.
    const BASE_DIGEST: &str = if cfg!(target_feature = "avx2") {
        "0e1c5af8e3aa66b66b2bb127149a8eaf10529fed838d69a4912c73bf345640c3"
    } else {
        "1c6d3055ea57737e00dce442398ddf208ba449e3bb9a3f783ff2a70ad43d15e8"
    };
    const BLOCKS: usize = 24;
    const FRAMES: usize = 128;
    const RETARGET_BLOCK: usize = 3;
    const WINDOW: u32 = 200;
    /// The block in which a `WINDOW`-sample ramp that starts at `RETARGET_BLOCK` ends.
    const RAMP_END_BLOCK: usize = RETARGET_BLOCK + (WINDOW as usize - 1) / FRAMES;
    let _canonical = lane::CanonicalFpEnv::enter();
    let mut sink = bench_support::digest::Sha256Sink::new();
    for &(backend, width) in BANK_WIDTHS {
        let lanes = width.lanes() as usize;
        for members in [1, lanes - 1, lanes] {
            for shape in [
                SettledShape::NonIdentity,
                SettledShape::OneIdentityMember,
                SettledShape::RetargetToPan,
                SettledShape::RetargetToIdentity,
            ] {
                let prepared = (0..members)
                    .map(|lane| {
                        let matrix = match shape {
                            SettledShape::OneIdentityMember if lane == members / 2 => {
                                Matrix2x2::IDENTITY
                            }
                            _ => non_identity(lane),
                        };
                        (matrix, 0)
                    })
                    .collect();
                let mut bank =
                    BuiltinMatrixBank::new(backend, width, prepared).expect("prepared bank");
                let mut state = (lanes * 1_000 + members * 10) as u64;
                let mut left = vec![0.0_f32; FRAMES * lanes];
                let mut right = vec![0.0_f32; FRAMES * lanes];
                for block in 0..BLOCKS {
                    if block == RETARGET_BLOCK {
                        let target = match shape {
                            SettledShape::RetargetToPan => Some(non_identity(lanes - 1)),
                            SettledShape::RetargetToIdentity => Some(Matrix2x2::IDENTITY),
                            _ => None,
                        };
                        if let Some(target) = target {
                            bank.set_target_smoothed(0, target, WINDOW)
                                .expect("in-domain retarget");
                        }
                    }
                    for frame in 0..FRAMES {
                        for lane in 0..lanes {
                            let (l, r) = hostile_pair(frame + lane, &mut state);
                            left[frame * lanes + lane] = l;
                            right[frame * lanes + lane] = r;
                        }
                    }
                    bank.process(&mut left, &mut right, FRAMES as u32);
                    for word in left.iter().chain(&right) {
                        sink.update(word.to_bits().to_le_bytes());
                    }
                    for lane in 0..lanes {
                        for word in test_support::matrix_bank_lane_words(&bank, lane) {
                            sink.update(word.to_le_bytes());
                        }
                    }
                    let retargeted = matches!(
                        shape,
                        SettledShape::RetargetToPan | SettledShape::RetargetToIdentity
                    );
                    if retargeted && block == RAMP_END_BLOCK {
                        let words = test_support::matrix_bank_lane_words(&bank, 0);
                        assert_eq!(
                            words[0..4],
                            words[4..8],
                            "{shape:?} width={lanes} members={members}: the ramp's last frame \
                             assigns the target"
                        );
                        assert_eq!(
                            words[13], 0,
                            "{shape:?} width={lanes} members={members}: the countdown is spent"
                        );
                    }
                }
            }
        }
    }
    let digest = sink.finish_hex();
    println!("settled_matrix_shapes_render_the_base_bits digest: {digest}");
    assert_eq!(digest, BASE_DIGEST, "a settled matrix shape moved a bit");
}

/// One fused fader/matrix shape of issue #954's scenario gate.
#[derive(Clone, Copy, Debug)]
enum FusedShape {
    /// Every member non-identity: with a full bank, the one shape with no identity lane at all.
    NonIdentity,
    /// One member exactly [`Matrix2x2::IDENTITY`], the rest non-identity.
    OneIdentityMember,
    /// Every member non-identity; member 0 retargets **instantly** (a 0-sample window) to
    /// [`Matrix2x2::IDENTITY`] at [`FUSED_TO_IDENTITY_BLOCK`] and instantly back to a pan at
    /// [`FUSED_FROM_IDENTITY_BLOCK`]. Both retargets settle before the next render, so every block
    /// stays on the fused path while the identity mask changes under it.
    InstantToIdentity,
    /// Every member non-identity; member 0's matrix ramps from [`FUSED_TO_IDENTITY_BLOCK`] over
    /// 200 samples and member 1's fader ramps from [`FUSED_FADER_RAMP_BLOCK`]: those blocks fall
    /// back to the split stages, and the fused path resumes once both settle.
    SmoothedRetargets,
}

/// The block at which [`FusedShape::InstantToIdentity`] makes member 0 the identity.
const FUSED_TO_IDENTITY_BLOCK: usize = 3;
/// The block at which [`FusedShape::InstantToIdentity`] makes member 0 a pan again.
const FUSED_FROM_IDENTITY_BLOCK: usize = 7;
/// The block at which [`FusedShape::SmoothedRetargets`] starts member 1's fader ramp.
const FUSED_FADER_RAMP_BLOCK: usize = 10;

/// Prepared fader parameters for one member: unity gain on the members that meet the signed-zero
/// frames on an identity lane, mixed gains and per-side mutes on the rest.
fn fused_fader(lane: usize) -> BuiltinParameters {
    let channel = |fader_db: f32, muted: bool| ChannelParameters {
        fader_db,
        muted,
        ..ChannelParameters::default()
    };
    let (left, right) = match lane % 4 {
        0 => (channel(0.0, false), channel(0.0, false)),
        1 => (channel(-6.0, true), channel(3.5, false)),
        2 => (channel(0.0, false), channel(-144.0, true)),
        _ => (channel(12.0, false), channel(-0.5, false)),
    };
    BuiltinParameters {
        left,
        right,
        ..BuiltinParameters::default()
    }
}

/// Issue #954 scenario gate: the fused fader/matrix pair renders the bits the base rendered.
///
/// One deterministic scenario, 16 blocks of 128 frames, through
/// [`BuiltinFaderBank::try_process_settled_with_matrix`] at both bank widths with `{1, W - 1, W}`
/// members, over the four [`FusedShape`]s. A block the fused call declines (a ramp in flight) runs
/// the split stages, exactly as the product's bank processor does. Every output word, whether the
/// block was fused, and every block's per-lane retained fader and matrix words (padding lanes
/// included) are folded into one SHA-256, pinned at the value the unmodified base (`e0f25bb6`)
/// printed in both dev and release before the select-free fused arm existed.
///
/// The input is [`hostile_pair`]'s, so every lane meets `l = -0.0, r = +0.0` at unity gain,
/// unmuted, on lanes `0 mod 4`: an identity lane passes that `-0.0` through, and
/// `1 * -0.0 + 0 * +0.0` would be `+0.0`. Only a full non-identity bank -- shape (a), and shapes
/// (c) and (d) outside their identity or ramp blocks -- has no identity lane; partial banks carry
/// identity padding lanes and keep the select.
///
/// Red mutations (issue #954, `MUTATIONS.md`): M1, the fused dispatch always select-free, moves the
/// digest through the identity lanes' `-0.0` (in dev the kernel's `debug_assert!` fires first); M3,
/// `lr` and `rl` swapped in the select-free fused kernel, moves it through shape (a).
#[test]
fn fused_fader_matrix_shapes_render_the_base_bits() {
    // One digest per build (see `BANK_WIDTHS`): the 4-lane pin is this scenario at four lanes alone.
    const BASE_DIGEST: &str = if cfg!(target_feature = "avx2") {
        "46cc00962fac4916a0d5dfed9b197fb12c85082acd0cae806774876c3744abce"
    } else {
        "2703b0f2653aca3d1d617e8d94deea260549c0df9475392e9d3c3f1b0d380b6e"
    };
    /// 384 blocks, less the 16 a matrix or fader ramp holds on the split stages, in the 8-lane
    /// (AVX2) build; the four-lane width's half of them in a 4-lane (NEON/simd128) build.
    const FUSED_BLOCKS: usize = if cfg!(target_feature = "avx2") {
        368
    } else {
        184
    };
    const BLOCKS: usize = 16;
    const FRAMES: usize = 128;
    const WINDOW: u32 = 200;
    let _canonical = lane::CanonicalFpEnv::enter();
    let mut sink = bench_support::digest::Sha256Sink::new();
    let mut fused_blocks = 0_usize;
    for &(backend, width) in BANK_WIDTHS {
        let lanes = width.lanes() as usize;
        for members in [1, lanes - 1, lanes] {
            for shape in [
                FusedShape::NonIdentity,
                FusedShape::OneIdentityMember,
                FusedShape::InstantToIdentity,
                FusedShape::SmoothedRetargets,
            ] {
                let identity_member =
                    matches!(shape, FusedShape::OneIdentityMember).then_some(members / 2);
                let prepared = (0..members)
                    .map(|lane| {
                        let matrix = if identity_member == Some(lane) {
                            Matrix2x2::IDENTITY
                        } else {
                            non_identity(lane)
                        };
                        (matrix, 0)
                    })
                    .collect();
                let mut matrix =
                    BuiltinMatrixBank::new(backend, width, prepared).expect("prepared matrix");
                let mut fader =
                    BuiltinFaderBank::new(backend, width, (0..members).map(fused_fader).collect())
                        .expect("prepared fader");
                let mut state = (lanes * 1_000 + members * 10 + 7) as u64;
                let mut left = vec![0.0_f32; FRAMES * lanes];
                let mut right = vec![0.0_f32; FRAMES * lanes];
                for block in 0..BLOCKS {
                    match (shape, block) {
                        (FusedShape::InstantToIdentity, FUSED_TO_IDENTITY_BLOCK) => matrix
                            .set_target_smoothed(0, Matrix2x2::IDENTITY, 0)
                            .expect("instant retarget"),
                        (FusedShape::InstantToIdentity, FUSED_FROM_IDENTITY_BLOCK) => matrix
                            .set_target_smoothed(0, non_identity(lanes - 2), 0)
                            .expect("instant retarget"),
                        (FusedShape::SmoothedRetargets, FUSED_TO_IDENTITY_BLOCK) => matrix
                            .set_target_smoothed(0, non_identity(lanes - 1), WINDOW)
                            .expect("smoothed retarget"),
                        (FusedShape::SmoothedRetargets, FUSED_FADER_RAMP_BLOCK) if members > 1 => {
                            fader
                                .set_fader_db(1, BuiltinLaneSelector::Both, -9.5, 100)
                                .expect("smoothed fader");
                        }
                        _ => {}
                    }
                    for frame in 0..FRAMES {
                        for lane in 0..lanes {
                            let (l, r) = hostile_pair(frame + lane, &mut state);
                            left[frame * lanes + lane] = l;
                            right[frame * lanes + lane] = r;
                        }
                    }
                    let fused = fader.try_process_settled_with_matrix(
                        &mut matrix,
                        &mut left,
                        &mut right,
                        FRAMES as u32,
                    );
                    if fused {
                        fused_blocks += 1;
                    } else {
                        fader.process(&mut left, &mut right, FRAMES as u32);
                        matrix.process(&mut left, &mut right, FRAMES as u32);
                    }
                    sink.update([u8::from(fused)]);
                    for word in left.iter().chain(&right) {
                        sink.update(word.to_bits().to_le_bytes());
                    }
                    for lane in 0..lanes {
                        for word in test_support::fader_bank_lane_words(&fader, lane) {
                            sink.update(word.to_le_bytes());
                        }
                        for word in test_support::matrix_bank_lane_words(&matrix, lane) {
                            sink.update(word.to_le_bytes());
                        }
                    }
                }
            }
        }
    }
    let digest = sink.finish_hex();
    println!(
        "fused_fader_matrix_shapes_render_the_base_bits digest: {digest} fused={fused_blocks}"
    );
    assert_eq!(
        fused_blocks, FUSED_BLOCKS,
        "the fused path's coverage moved"
    );
    assert_eq!(
        digest, BASE_DIGEST,
        "a fused fader/matrix shape moved a bit"
    );
}

/// Issue #954 scenario gate, scalar sites: the per-track fused fader/matrix renders the base bits.
///
/// The two `f32` call sites of the fused kernel -- [`BuiltinChain::process_dual_mono`] and
/// [`FaderMuteRampBuiltins::process_fader_matrix`] -- over four tracks: an exact identity matrix at
/// unity gain, a pan with a muted left side, a pan that retargets instantly to the identity at
/// [`FUSED_TO_IDENTITY_BLOCK`] and back at [`FUSED_FROM_IDENTITY_BLOCK`], and a pan whose matrix
/// ramps over 200 samples (the per-track fused call declines those blocks and the split stages
/// run). A scalar track's identity mask is its own flag, so the identity tracks keep the select and
/// the pan tracks take the select-free arm. Every output word and whether the block was fused are
/// folded into one SHA-256, pinned at the unmodified base (`e0f25bb6`) in dev and release.
///
/// Red mutation (issue #954 M1, `MUTATIONS.md`): the fused dispatch always select-free moves the
/// digest through the identity track's `-0.0` at the `process_fader_matrix` site. At the
/// `BuiltinChain` site the chain's input stage has already turned `-0.0` into `+0.0`, so this test
/// cannot see M1 there; that site is tools-only, and the dispatch witness (M5) guards it.
#[test]
fn scalar_fused_fader_matrix_renders_the_base_bits() {
    const BASE_DIGEST: &str = "c51310190d189b0432ae56f97e331ee79a9e2c1dd222b5cccc086b4abbcaaafe";
    /// 48 blocks, less the 4 the 200-sample matrix ramp holds on the split stages.
    const FUSED_BLOCKS: usize = 44;
    const BLOCKS: usize = 12;
    const FRAMES: usize = 64;
    const WINDOW: u32 = 200;
    let _canonical = lane::CanonicalFpEnv::enter();
    let mut sink = bench_support::digest::Sha256Sink::new();
    let mut fused_blocks = 0_usize;
    for track in 0..4_usize {
        let matrix = if track == 0 {
            Matrix2x2::IDENTITY
        } else {
            non_identity(track + 1)
        };
        let parameters = BuiltinParameters {
            matrix,
            ..fused_fader(track)
        };
        let mut chain = BuiltinChain::new(48_000, parameters).expect("prepared chain");
        let (_, _, mut split_matrix) = BuiltinChain::new(48_000, parameters)
            .expect("prepared sections")
            .into_sections();
        let mut ramp_fader = FaderMuteRampBuiltins::new(parameters).expect("prepared fader");
        let mut state = (track * 100 + 3) as u64;
        for block in 0..BLOCKS {
            let retarget = match (track, block) {
                (2, FUSED_TO_IDENTITY_BLOCK) => Some((Matrix2x2::IDENTITY, 0)),
                (2, FUSED_FROM_IDENTITY_BLOCK) => Some((non_identity(6), 0)),
                (3, FUSED_TO_IDENTITY_BLOCK) => Some((non_identity(7), WINDOW)),
                _ => None,
            };
            if let Some((target, window)) = retarget {
                test_support::chain_matrix_mut(&mut chain)
                    .set_target_smoothed(target, window)
                    .expect("chain retarget");
                split_matrix
                    .set_target_smoothed(target, window)
                    .expect("section retarget");
            }
            let mut chain_left = [0.0_f32; FRAMES];
            let mut chain_right = [0.0_f32; FRAMES];
            for frame in 0..FRAMES {
                (chain_left[frame], chain_right[frame]) = hostile_pair(frame + track, &mut state);
            }
            let mut ramp_left = chain_left;
            let mut ramp_right = chain_right;
            chain.process_dual_mono(
                DualMonoBlock::new(&mut chain_left, &mut chain_right, 0).expect("chain block"),
            );
            let mut ramp_block =
                DualMonoBlock::new(&mut ramp_left, &mut ramp_right, 0).expect("fader block");
            let fused = ramp_fader.process_fader_matrix(&mut split_matrix, &mut ramp_block);
            if fused {
                fused_blocks += 1;
            } else {
                ramp_fader.process(
                    DualMonoBlock::new(&mut ramp_left, &mut ramp_right, 0).expect("split block"),
                );
                split_matrix.process(
                    DualMonoBlock::new(&mut ramp_left, &mut ramp_right, 0).expect("split block"),
                );
            }
            sink.update([u8::from(fused)]);
            for word in chain_left
                .iter()
                .chain(&chain_right)
                .chain(&ramp_left)
                .chain(&ramp_right)
            {
                sink.update(word.to_bits().to_le_bytes());
            }
        }
    }
    let digest = sink.finish_hex();
    println!(
        "scalar_fused_fader_matrix_renders_the_base_bits digest: {digest} fused={fused_blocks}"
    );
    assert_eq!(
        fused_blocks, FUSED_BLOCKS,
        "the scalar fused path's coverage moved"
    );
    assert_eq!(
        digest, BASE_DIGEST,
        "a scalar fused fader/matrix track moved a bit"
    );
}
