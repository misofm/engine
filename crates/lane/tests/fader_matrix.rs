#![allow(missing_docs)]

use lane::{
    Lane, Simd4, Simd8,
    kernels::builtins::{
        Matrix2x2Coef, fader_matrix_block, gain_mute_block, mask_from_flags, matrix2x2_block,
    },
};

const WIDTH: usize = 8;

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

fn compare_case<L: Lane>(frames: usize, identity: [f32; WIDTH], family: usize) {
    let samples = frames * L::WIDTH;
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
    let source = [finite, signed_zero, subnormal, nonfinite][family];
    let mut left = vec![f32::from_bits(0x42f6_e979); samples + 2];
    let mut right = vec![f32::from_bits(0xc2f6_e979); samples + 2];
    for index in 0..samples {
        left[index + 1] = source[index % WIDTH];
        right[index + 1] = source[(index * 3 + 1) % WIDTH];
    }
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
    let matrix = Matrix2x2Coef {
        ll: L::load(&[0.75, 1.0, -0.5, 0.25, 1.0, -0.75, 0.5, 1.0][..L::WIDTH]),
        lr: L::load(&[-0.25, 0.0, 0.75, -0.5, 0.0, 0.125, -1.0, 0.0][..L::WIDTH]),
        rl: L::load(&[0.5, 0.0, -0.25, 1.0, 0.0, 0.625, 0.25, 0.0][..L::WIDTH]),
        rr: L::load(&[0.25, 1.0, 1.0, -0.75, 1.0, 0.5, 0.875, 1.0][..L::WIDTH]),
        identity: mask_from_flags::<L>(&identity[..L::WIDTH]),
    };
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
    compare_width::<f32>();
    compare_width::<Simd4>();
    compare_width::<Simd8>();
}

fn run_scalar(
    left: f32,
    right: f32,
    gain_left: f32,
    mute_left: f32,
    matrix: Matrix2x2Coef<f32>,
) -> ([u32; 2], [u32; 2]) {
    let mut old_left = [left];
    let mut old_right = [right];
    gain_mute_block(
        &mut old_left,
        1,
        gain_left,
        mask_from_flags::<f32>(&[mute_left]),
    );
    gain_mute_block(&mut old_right, 1, 1.0, mask_from_flags::<f32>(&[0.0]));
    matrix2x2_block(&mut old_left, &mut old_right, 1, &matrix);
    let mut dut_left = [left];
    let mut dut_right = [right];
    fader_matrix_block(
        &mut dut_left,
        &mut dut_right,
        1,
        gain_left,
        mask_from_flags::<f32>(&[mute_left]),
        1.0,
        mask_from_flags::<f32>(&[0.0]),
        &matrix,
    );
    (
        [old_left[0].to_bits(), old_right[0].to_bits()],
        [dut_left[0].to_bits(), dut_right[0].to_bits()],
    )
}

#[test]
fn wrong_equations_are_distinguished_before_the_dut_is_checked() {
    let identity = Matrix2x2Coef {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        identity: mask_from_flags::<f32>(&[1.0]),
    };
    let (old, dut) = run_scalar(-2.0, 1.0, 0.5, 1.0, identity);
    let wrong_mute = ((-2.0_f32 * 0.5) * 0.0).to_bits();
    assert_ne!(
        wrong_mute, old[0],
        "zero multiplication must differ from mask clearing"
    );
    assert_eq!(dut, old);

    let cross = Matrix2x2Coef {
        ll: 0.75,
        lr: -0.25,
        rl: 0.5,
        rr: 0.25,
        identity: mask_from_flags::<f32>(&[0.0]),
    };
    let (old, dut) = run_scalar(2.0, 4.0, 1.0, 0.0, cross);
    let overwritten_left = cross.rl * f32::from_bits(old[0]) + cross.rr * 4.0;
    assert_ne!(
        overwritten_left.to_bits(),
        old[1],
        "right must use the loaded left input"
    );
    assert_eq!(dut, old);

    let zeros = Matrix2x2Coef {
        ll: -0.0,
        lr: -0.0,
        rl: 0.0,
        rr: 1.0,
        identity: mask_from_flags::<f32>(&[0.0]),
    };
    let (old, dut) = run_scalar(1.0, 1.0, 1.0, 0.0, zeros);
    let recombined = ((0.0_f32 + zeros.ll * 1.0) + zeros.lr * 1.0).to_bits();
    assert_ne!(
        recombined, old[0],
        "zero-seeded recombination must differ from the frozen two-product sum"
    );
    assert_eq!(dut, old);
}
