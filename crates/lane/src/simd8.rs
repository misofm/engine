//! `Simd8`: `impl Lane for wide::f32x8`.
//!
//! Eight `f32` lanes in one `__m256`: the 8-lane (AVX2) width, which the workspace pins at compile
//! time. This module is compiled only where `avx2` is enabled (issues #1110 and #1112), so no
//! build lowers `f32x8` to two four-lane values. The body is `wide_impl::impl_lane_for_wide`; the
//! notes there say what is deliberately not forwarded to `wide`.

use wide::{f32x8, u32x8};

use crate::wide_impl::impl_lane_for_wide;

impl_lane_for_wide!(f32x8, u32x8, 8, 2);
