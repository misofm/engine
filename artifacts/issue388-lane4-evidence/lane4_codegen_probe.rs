//! Non-inlined wrappers used to capture LANE-4 caller disassembly.

use std::hint::black_box;

use lane::Simd8;

/// Instantiates the fast-gain caller at the shipped native lane width.
#[inline(never)]
pub fn issue388_fast_gain_from_db_simd8(input: Simd8) -> Simd8 {
    math::fast_db::fast_gain_from_db(input)
}

/// Instantiates the exact exp2 caller at the shipped native lane width.
#[inline(never)]
pub fn issue388_exp2_lane_simd8(input: Simd8) -> Simd8 {
    math::exp2_lane(input)
}

fn main() {
    let input = black_box(Simd8::splat(0.125));
    black_box(issue388_fast_gain_from_db_simd8(input));
    black_box(issue388_exp2_lane_simd8(input));
}
