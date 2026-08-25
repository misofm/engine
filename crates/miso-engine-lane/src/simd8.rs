//! `Simd8`: `impl Lane for wide::f32x8`.
//!
//! Eight `f32` lanes in one `__m256` on `x86-64-v3`, which the workspace pins at compile time. On
//! any other target `wide` lowers `f32x8` to two four-lane values; that is correct but is not a
//! production width (D4). The body is `wide_impl::impl_lane_for_wide`; the notes there say what is
//! deliberately not forwarded to `wide`.

use wide::{f32x8, u32x8};

use crate::wide_impl::impl_lane_for_wide;

impl_lane_for_wide!(f32x8, u32x8, 8, crate::softfma::fma_f32x8_soft, 2);
