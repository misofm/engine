//! Retained opaque wrappers for issue #463's pre-edit bounds inspection.
//!
//! The wrappers keep lengths, pointers, coefficients, and output observable at the ABI boundary.
//! They are deliberately non-inlined so the release object retains one named body per kernel.

#![allow(missing_docs, unsafe_code)]

use lane::kernels::{mix2x2_block, sum2_block, sum_into_block};

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn issue463_sum2(out: *mut f32, a: *const f32, b: *const f32, len: usize) {
    // SAFETY: the caller supplies valid spans for this inspection probe.
    let (out, a, b) = unsafe {
        (
            core::slice::from_raw_parts_mut(out, len),
            core::slice::from_raw_parts(a, len),
            core::slice::from_raw_parts(b, len),
        )
    };
    sum2_block::<lane::Simd8>(out, a, b);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn issue463_sum_into(acc: *mut f32, x: *const f32, len: usize) {
    // SAFETY: the caller supplies valid spans for this inspection probe.
    let (acc, x) = unsafe {
        (
            core::slice::from_raw_parts_mut(acc, len),
            core::slice::from_raw_parts(x, len),
        )
    };
    sum_into_block::<lane::Simd8>(acc, x);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn issue463_mix2x2(
    left: *mut f32,
    right: *mut f32,
    len: usize,
    ll: f32,
    lr: f32,
    rl: f32,
    rr: f32,
) {
    // SAFETY: the caller supplies valid spans for this inspection probe.
    let (left, right) = unsafe {
        (
            core::slice::from_raw_parts_mut(left, len),
            core::slice::from_raw_parts_mut(right, len),
        )
    };
    mix2x2_block::<lane::Simd8>(left, right, [ll, lr, rl, rr]);
}

fn main() {
    let len = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(17);
    let mut out = vec![0.25_f32; len];
    let mut left = vec![0.5_f32; len];
    let mut right = vec![-0.25_f32; len];
    let input = vec![0.125_f32; len];
    // SAFETY: all spans are allocated above with the same runtime length.
    unsafe {
        issue463_sum2(out.as_mut_ptr(), left.as_ptr(), right.as_ptr(), len);
        issue463_sum_into(out.as_mut_ptr(), input.as_ptr(), len);
        issue463_mix2x2(left.as_mut_ptr(), right.as_mut_ptr(), len, 0.9, -0.1, 0.2, 0.8);
    }
    std::hint::black_box((out, left, right));
}
