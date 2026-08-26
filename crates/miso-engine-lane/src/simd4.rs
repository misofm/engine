//! `Simd4`: `impl Lane for wide::f32x4`.
//!
//! Four `f32` lanes: NEON on AArch64, `v128` on wasm with `simd128`, and SSE on x86 (where `Simd8`
//! is the production width). The body is `wide_impl::impl_lane_for_wide`; the notes there say what
//! is deliberately not forwarded to `wide`.

use wide::{f32x4, u32x4};

use crate::wide_impl::impl_lane_for_wide;

impl_lane_for_wide!(f32x4, u32x4, 4, 2);
