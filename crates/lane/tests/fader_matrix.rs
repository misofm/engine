#![allow(missing_docs)]

use lane::{
    Lane, Simd4, Simd8,
    kernels::builtins::{
        Matrix2x2Coef, fader_matrix_block, gain_mute_block, mask_from_flags, matrix2x2_block,
    },
};

fn compare_width<L: Lane>() {
    let frames = 9;
    let mut left = (0..frames * L::WIDTH)
        .map(|i| {
            if i % 5 == 0 {
                -0.0
            } else {
                (i as f32 - 7.0) * 0.125
            }
        })
        .collect::<Vec<_>>();
    let mut right = (0..frames * L::WIDTH)
        .map(|i| {
            if i % 7 == 0 {
                f32::from_bits(1)
            } else {
                (3.0 - i as f32) * 0.25
            }
        })
        .collect::<Vec<_>>();
    let mut expected_left = left.clone();
    let mut expected_right = right.clone();
    let gains = [1.25, 0.75, 1.0, 0.5, 1.1, 0.9, 0.8, 1.2];
    let mute_l = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
    let mute_r = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let gain_left = L::load(&gains[..L::WIDTH]);
    let gain_right = L::load(&gains[8 - L::WIDTH..]);
    let ml = mask_from_flags::<L>(&mute_l[..L::WIDTH]);
    let mr = mask_from_flags::<L>(&mute_r[..L::WIDTH]);
    let matrix = Matrix2x2Coef {
        ll: L::splat(0.75),
        lr: L::splat(-0.25),
        rl: L::splat(0.5),
        rr: L::splat(0.25),
        identity: mask_from_flags::<L>(&[0.0; 8][..L::WIDTH]),
    };
    gain_mute_block(&mut expected_left, frames, gain_left, ml);
    gain_mute_block(&mut expected_right, frames, gain_right, mr);
    matrix2x2_block(&mut expected_left, &mut expected_right, frames, &matrix);
    fader_matrix_block(
        &mut left, &mut right, frames, gain_left, ml, gain_right, mr, &matrix,
    );
    assert_eq!(
        left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        expected_left
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        right.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
        expected_right
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
    );
}

#[test]
fn settled_fader_matrix_matches_the_two_primitive_oracle() {
    compare_width::<f32>();
    compare_width::<Simd4>();
    compare_width::<Simd8>();
}
