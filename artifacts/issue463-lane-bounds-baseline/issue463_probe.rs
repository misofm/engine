//! Retained opaque wrappers for issue #463's pre-edit bounds inspection.
//!
//! This file is compiled directly with `rustc`; it is deliberately outside Cargo target
//! discovery. Every slice has an independent runtime length, coefficients arrive through the ABI,
//! and each output is made observable through an opaque checksum returned to the caller.

#![allow(missing_docs, unsafe_code)]
#![crate_type = "lib"]

use lane::Lane;
use lane::kernels::{mix2x2_block, sum_into_block, sum2_block};

#[inline(never)]
fn observe(values: &[f32]) -> u32 {
    values
        .iter()
        .fold(0_u32, |bits, value| bits.rotate_left(5) ^ value.to_bits())
}

macro_rules! wrappers {
    ($sum2:ident, $sum_into:ident, $mix2x2:ident, $lane:ty) => {
        #[unsafe(no_mangle)]
        #[inline(never)]
        pub unsafe extern "C" fn $sum2(
            out: *mut f32,
            out_len: usize,
            a: *const f32,
            a_len: usize,
            b: *const f32,
            b_len: usize,
        ) -> u32 {
            // SAFETY: this inspection ABI requires each pointer to address its independently
            // declared span and requires the mutable output span not to overlap either input.
            let (out, a, b) = unsafe {
                (
                    core::slice::from_raw_parts_mut(out, out_len),
                    core::slice::from_raw_parts(a, a_len),
                    core::slice::from_raw_parts(b, b_len),
                )
            };
            sum2_block::<$lane>(out, a, b);
            observe(out)
        }

        #[unsafe(no_mangle)]
        #[inline(never)]
        pub unsafe extern "C" fn $sum_into(
            acc: *mut f32,
            acc_len: usize,
            x: *const f32,
            x_len: usize,
        ) -> u32 {
            // SAFETY: this inspection ABI requires each pointer to address its independently
            // declared span and requires the mutable accumulator not to overlap the input.
            let (acc, x) = unsafe {
                (
                    core::slice::from_raw_parts_mut(acc, acc_len),
                    core::slice::from_raw_parts(x, x_len),
                )
            };
            sum_into_block::<$lane>(acc, x);
            observe(acc)
        }

        #[unsafe(no_mangle)]
        #[inline(never)]
        pub unsafe extern "C" fn $mix2x2(
            left: *mut f32,
            left_len: usize,
            right: *mut f32,
            right_len: usize,
            ll: f32,
            lr: f32,
            rl: f32,
            rr: f32,
        ) -> u32 {
            // SAFETY: this inspection ABI requires each pointer to address its independently
            // declared span and requires the two mutable spans not to overlap.
            let (left, right) = unsafe {
                (
                    core::slice::from_raw_parts_mut(left, left_len),
                    core::slice::from_raw_parts_mut(right, right_len),
                )
            };
            mix2x2_block::<$lane>(left, right, [ll, lr, rl, rr]);
            observe(left).rotate_left(11) ^ observe(right)
        }
    };
}

#[cfg(target_arch = "x86_64")]
wrappers!(
    issue463_sum2_simd8,
    issue463_sum_into_simd8,
    issue463_mix2x2_simd8,
    lane::Simd8
);

#[cfg(target_arch = "wasm32")]
wrappers!(
    issue463_sum2_simd4,
    issue463_sum_into_simd4,
    issue463_mix2x2_simd4,
    lane::Simd4
);

wrappers!(
    issue463_sum2_scalar,
    issue463_sum_into_scalar,
    issue463_mix2x2_scalar,
    f32
);

const _: usize = <f32 as Lane>::WIDTH;
