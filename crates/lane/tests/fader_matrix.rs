#![allow(missing_docs)]

use lane::{
    CanonicalFpEnv, Lane,
    kernels::builtins::{
        Matrix2x2Coef, fader_matrix_block, gain_mute_block, mask_from_flags, matrix2x2_block,
        matrix2x2_block_without_identity, no_lanes,
    },
};

const WIDTH: usize = 8;

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// The four hostile input families: finite, signed zero, subnormal, and non-finite with a NaN
/// payload.
fn family_source(family: usize) -> [f32; WIDTH] {
    let finite = [-3.5, 2.25, -0.0, 0.0, 1.0, -1.0, 0.375, -0.625];
    let signed_zero = [-0.0, 0.0, -0.0, 0.0, -0.0, 0.0, -0.0, 0.0];
    let subnormal = [
        f32::from_bits(1),
        f32::from_bits(2),
        -f32::from_bits(1),
        -f32::from_bits(3),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        f32::from_bits(7),
        -f32::from_bits(9),
    ];
    let nonfinite = [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        1.0,
        -2.0,
        0.0,
        -0.0,
        f32::from_bits(0x7fc0_1234),
    ];
    [finite, signed_zero, subnormal, nonfinite][family]
}

/// Both planes of `samples` words from one family, with one guard word at each end.
fn hostile_planes(samples: usize, family: usize) -> (Vec<f32>, Vec<f32>) {
    let source = family_source(family);
    let mut left = vec![f32::from_bits(0x42f6_e979); samples + 2];
    let mut right = vec![f32::from_bits(0xc2f6_e979); samples + 2];
    for index in 0..samples {
        left[index + 1] = source[index % WIDTH];
        right[index + 1] = source[(index * 3 + 1) % WIDTH];
    }
    (left, right)
}

/// Signed, asymmetric per-lane coefficients (`lr != rl` on every lane that is not zero in both).
fn hostile_matrix<L: Lane>(identity: [f32; WIDTH]) -> Matrix2x2Coef<L> {
    Matrix2x2Coef {
        ll: L::load(&[0.75, 1.0, -0.5, 0.25, 1.0, -0.75, 0.5, 1.0][..L::WIDTH]),
        lr: L::load(&[-0.25, 0.0, 0.75, -0.5, 0.0, 0.125, -1.0, 0.0][..L::WIDTH]),
        rl: L::load(&[0.5, 0.0, -0.25, 1.0, 0.0, 0.625, 0.25, 0.0][..L::WIDTH]),
        rr: L::load(&[0.25, 1.0, 1.0, -0.75, 1.0, 0.5, 0.875, 1.0][..L::WIDTH]),
        identity: mask_from_flags::<L>(&identity[..L::WIDTH]),
    }
}

fn compare_case<L: Lane>(frames: usize, identity: [f32; WIDTH], family: usize) {
    let samples = frames * L::WIDTH;
    let (mut left, mut right) = hostile_planes(samples, family);
    let mut old_left = left.clone();
    let mut old_right = right.clone();
    let gains_l = [1.25, 0.75, 1.0, 0.5, 1.1, 0.9, 0.8, 1.2];
    let gains_r = [0.625, 1.5, 0.25, 1.0, 0.7, 1.3, 0.5, 0.875];
    let mute_l = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
    let mute_r = [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0];
    let gain_left = L::load(&gains_l[..L::WIDTH]);
    let gain_right = L::load(&gains_r[..L::WIDTH]);
    let ml = mask_from_flags::<L>(&mute_l[..L::WIDTH]);
    let mr = mask_from_flags::<L>(&mute_r[..L::WIDTH]);
    let matrix = hostile_matrix::<L>(identity);
    gain_mute_block(&mut old_left[1..=samples], frames, gain_left, ml);
    gain_mute_block(&mut old_right[1..=samples], frames, gain_right, mr);
    matrix2x2_block(
        &mut old_left[1..=samples],
        &mut old_right[1..=samples],
        frames,
        &matrix,
    );
    fader_matrix_block(
        &mut left[1..=samples],
        &mut right[1..=samples],
        frames,
        gain_left,
        ml,
        gain_right,
        mr,
        &matrix,
    );
    assert_eq!(
        bits(&left),
        bits(&old_left),
        "left width={} frames={frames} family={family}",
        L::WIDTH
    );
    assert_eq!(
        bits(&right),
        bits(&old_right),
        "right width={} frames={frames} family={family}",
        L::WIDTH
    );
}

fn compare_width<L: Lane>() {
    for frames in [1, 3, 8, 9, 128] {
        for family in 0..4 {
            compare_case::<L>(frames, [0.0; WIDTH], family);
            compare_case::<L>(frames, [1.0; WIDTH], family);
            compare_case::<L>(frames, [1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0], family);
        }
    }
}

#[test]
fn settled_fader_matrix_matches_the_two_primitive_oracle() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| compare_width::<L>());
}

/// Class A as issue #944 states it and #1065 rules it (`dsp_reference::class_a`): equal bits,
/// except that a NaN word only has to stay a NaN.
///
/// The release build CI runs commutes the commutative `fadd` differently in the two kernels
/// (`rr * r + rl * l` against `rl * l + rr * r`), and x86 keeps the first operand's payload when both
/// are NaN. Every other word, signed zeros and subnormals included, must match bit for bit.
fn same_word(new: f32, old: f32) -> bool {
    dsp_reference::class_a::same(new, old)
}

/// Runs both settled kernels over one family and returns the first differing word, if any.
///
/// `matrix2x2_block` sees `identity`; the select-free kernel sees the same coefficients with no
/// identity lane, which is its precondition. Guard words at both ends are compared too.
fn select_free_mismatch<L: Lane>(
    frames: usize,
    identity: [f32; WIDTH],
    family: usize,
) -> Option<String> {
    let samples = frames * L::WIDTH;
    let (mut left, mut right) = hostile_planes(samples, family);
    let mut old_left = left.clone();
    let mut old_right = right.clone();
    let select_form = hostile_matrix::<L>(identity);
    let select_free = Matrix2x2Coef {
        identity: no_lanes::<L>(),
        ..select_form
    };
    matrix2x2_block(
        &mut old_left[1..=samples],
        &mut old_right[1..=samples],
        frames,
        &select_form,
    );
    matrix2x2_block_without_identity(
        &mut left[1..=samples],
        &mut right[1..=samples],
        frames,
        &select_free,
    );
    for (plane, new, old) in [("L", &left, &old_left), ("R", &right, &old_right)] {
        for (index, (new, old)) in new.iter().zip(old.iter()).enumerate() {
            if !same_word(*new, *old) {
                return Some(format!(
                    "{plane}[{index}] new={:08x} old={:08x}",
                    new.to_bits(),
                    old.to_bits()
                ));
            }
        }
    }
    None
}

fn select_free_width<L: Lane>() {
    const FRAMES: [usize; 5] = [1, 3, 8, 9, 128];
    for frames in FRAMES {
        for family in 0..4 {
            if let Some(word) = select_free_mismatch::<L>(frames, [0.0; WIDTH], family) {
                panic!("width={} frames={frames} family={family}: {word}", L::WIDTH);
            }
        }
    }
    // Negative control: with identity lanes the two kernels are different functions, which is why
    // the arm is keyed on the mask. Signed zeros show it: an identity lane passes `-0.0` through,
    // while `ll * -0.0 + lr * -0.0` is `+0.0`. Asserted over the union of frame counts, because at
    // `f32` with one frame the mixed mask is "all identity" on an input the arithmetic reproduces.
    let mixed = [1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0];
    assert!(
        FRAMES
            .iter()
            .any(|&frames| select_free_mismatch::<L>(frames, mixed, 1).is_some()),
        "width={}: the select-free kernel must differ from the select form on identity lanes",
        L::WIDTH
    );
}

/// Issue #944 gate 1: with no identity lane, the select-free kernel is `matrix2x2_block`.
///
/// Every family, every frame count, every width, guard words included; NaN words compare as "both
/// NaN" (see [`same_word`]). Runs in dev and in release, which is the build CI tests `lane` in.
///
/// Red mutation (issue #944 M3, `crates/lane/tests/MUTATIONS.md`): swap `c.lr` and `c.rl` in
/// `matrix2x2_block_without_identity`; the finite family differs on the first frame.
#[test]
fn select_free_matrix_matches_the_select_form_when_no_lane_is_identity() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| select_free_width::<L>());
}
