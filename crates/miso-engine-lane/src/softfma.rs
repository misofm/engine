//! Exact software FMA, and the MXCSR helpers gate G6 needs.
//!
//! # Software FMA (master plan §3.5)
//!
//! For `f32` inputs the product `a * b` is exact in `f64` (24 + 24 <= 53 bits). The sum is then
//! computed in `f64`, forced to round-to-odd when it is inexact, and demoted once:
//! `RN32(RO64(x)) == RN32(x)` because 53 >= 24 + 2 (Boldo and Melquiond, *Emulation of a FMA and
//! correctly rounded sums: proved algorithms using rounding to odd*, IEEE TC 2008). The result is
//! the IEEE fused result by construction; gate G3 proves it against hardware FMA on x86.
//!
//! Round-to-odd is **direction aware**. Setting the low bit unconditionally (`bits | 1`) always
//! moves the magnitude up, but when `RN64` rounded the sum up past the exact value the odd
//! neighbour lies below it. Measured against hardware `fmaf`: the `| 1` form mismatches on 250,000
//! of 2,000,000 constructed midpoint triples, the form below on none.
//!
//! # Why this file is the crate's only `unsafe`
//!
//! The `x86` MXCSR helpers below are used by gate G6 to prove that hardware flush-to-zero is inert
//! under the D7 flush law. The workspace denies `unsafe` outside an enumerated allowlist and
//! forbids inline assembly, so the helpers live here, in the one lane file that file
//! `scripts/check-realtime-policy.sh` allows to carry `unsafe`, and use the deprecated
//! `_mm_getcsr`/`_mm_setcsr` intrinsics rather than the inline assembly their deprecation note
//! recommends.

#![allow(unsafe_code)]

/// `a * b + c` for `f32`, computed exactly in `f64` with round-to-odd before the demotion.
///
/// Bit-identical to `f32::mul_add` on a host with hardware FMA (gate G3), including on the
/// overflow, infinity and NaN edges: the `finite` guard keeps an infinite sum from being nudged
/// into a NaN, and a NaN sum stays a NaN whatever the adjustment does to its payload bits.
///
/// Frozen operation order (a mutation to any line is a G3 red-mutation):
/// 1. `p = (a as f64) * (b as f64)` — exact, no rounding.
/// 2. `s = p + c` — one `f64` round-to-nearest-even.
/// 3. `bb = s - p; e = (p - (s - bb)) + (c - bb)` — Knuth's TwoSum: `e` is the exact error of 2.
/// 4. adjust `s` by one unit in the last place, toward the exact value, when `s` is inexact,
///    finite and even.
/// 5. demote once.
#[inline(always)]
pub fn fma_f32_via_f64(a: f32, b: f32, c: f32) -> f32 {
    let p = f64::from(a) * f64::from(b);
    let c = f64::from(c);
    let s = p + c;
    let bb = s - p;
    let e = (p - (s - bb)) + (c - bb);
    let s_bits = s.to_bits();
    // A NaN error term makes `inexact` true, which is harmless: `finite` is false for an infinite
    // sum, and an adjusted NaN is still a NaN.
    let inexact = e != 0.0;
    let finite = f64::abs(s) < f64::INFINITY;
    let even = s_bits & 1 == 0;
    // `e` and `s` have opposite signs exactly when the exact value is closer to zero than `s`.
    let toward_zero = (e < 0.0) == (s > 0.0);
    let adjust = inexact & finite & even;
    // `s == 0` implies `e == 0` (the smallest nonzero product of two `f32` is 2^-298, deep inside
    // `f64`'s normal range), so the step never crosses zero; crossing a binade boundary is correct
    // because the `f64` neighbour across the boundary is the round-to-odd value.
    let bits = if adjust {
        if toward_zero { s_bits - 1 } else { s_bits + 1 }
    } else {
        s_bits
    };
    f64::from_bits(bits) as f32
}

/// Four-lane software FMA: `a * b + c` per lane, lane by lane identical to [`fma_f32_via_f64`].
///
/// On wasm with `simd128` this is the vector form of master plan §3.5: promote each half to
/// `f64x2`, run the round-to-odd algorithm with `f64x2` and `v128` bit operations, demote both
/// halves and recombine. Everywhere else it is four scalar calls, which the compiler is free to
/// vectorise and which is bit-identical by construction.
#[inline(always)]
pub fn fma_f32x4_soft(a: [f32; 4], b: [f32; 4], c: [f32; 4]) -> [f32; 4] {
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        wasm_simd128::fma_f32x4(a, b, c)
    }
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    {
        [
            fma_f32_via_f64(a[0], b[0], c[0]),
            fma_f32_via_f64(a[1], b[1], c[1]),
            fma_f32_via_f64(a[2], b[2], c[2]),
            fma_f32_via_f64(a[3], b[3], c[3]),
        ]
    }
}

/// Eight-lane software FMA: two calls to [`fma_f32x4_soft`].
#[inline(always)]
pub fn fma_f32x8_soft(a: [f32; 8], b: [f32; 8], c: [f32; 8]) -> [f32; 8] {
    /// Copies the four lanes starting at `offset` out of an eight-lane array.
    #[inline(always)]
    fn half(x: [f32; 8], offset: usize) -> [f32; 4] {
        [x[offset], x[offset + 1], x[offset + 2], x[offset + 3]]
    }
    let low = fma_f32x4_soft(half(a, 0), half(b, 0), half(c, 0));
    let high = fma_f32x4_soft(half(a, 4), half(b, 4), half(c, 4));
    [
        low[0], low[1], low[2], low[3], high[0], high[1], high[2], high[3],
    ]
}

/// The wasm `simd128` vector form of [`fma_f32_via_f64`].
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm_simd128 {
    use core::arch::wasm32::{
        f32x4, f32x4_demote_f64x2_zero, f32x4_extract_lane, f64x2_abs, f64x2_add, f64x2_gt,
        f64x2_lt, f64x2_mul, f64x2_ne, f64x2_promote_low_f32x4, f64x2_splat, f64x2_sub,
        i32x4_shuffle, i64x2_add, i64x2_eq, i64x2_splat, v128, v128_and, v128_bitselect, v128_not,
        v128_xor,
    };

    /// Round-to-odd FMA for the low two lanes of three `f32x4` vectors.
    ///
    /// The operation order is [`super::fma_f32_via_f64`]'s, with `select` in place of the branch.
    #[inline(always)]
    fn half(x: v128, y: v128, z: v128) -> v128 {
        let p = f64x2_mul(f64x2_promote_low_f32x4(x), f64x2_promote_low_f32x4(y));
        let cc = f64x2_promote_low_f32x4(z);
        let s = f64x2_add(p, cc);
        let bb = f64x2_sub(s, p);
        let e = f64x2_add(f64x2_sub(p, f64x2_sub(s, bb)), f64x2_sub(cc, bb));
        let inexact = f64x2_ne(e, f64x2_splat(0.0));
        let finite = f64x2_lt(f64x2_abs(s), f64x2_splat(f64::INFINITY));
        let even = i64x2_eq(v128_and(s, i64x2_splat(1)), i64x2_splat(0));
        let toward_zero = v128_not(v128_xor(
            f64x2_lt(e, f64x2_splat(0.0)),
            f64x2_gt(s, f64x2_splat(0.0)),
        ));
        let direction = v128_bitselect(i64x2_splat(-1), i64x2_splat(1), toward_zero);
        let adjust = v128_and(v128_and(inexact, finite), even);
        f32x4_demote_f64x2_zero(i64x2_add(s, v128_and(direction, adjust)))
    }

    /// Four-lane round-to-odd FMA.
    #[inline(always)]
    pub(super) fn fma_f32x4(a: [f32; 4], b: [f32; 4], c: [f32; 4]) -> [f32; 4] {
        /// Moves lanes 2 and 3 into lanes 0 and 1 so the high half can use the same body.
        #[inline(always)]
        fn upper(v: v128) -> v128 {
            i32x4_shuffle::<2, 3, 2, 3>(v, v)
        }
        let av = f32x4(a[0], a[1], a[2], a[3]);
        let bv = f32x4(b[0], b[1], b[2], b[3]);
        let cv = f32x4(c[0], c[1], c[2], c[3]);
        let low = half(av, bv, cv);
        let high = half(upper(av), upper(bv), upper(cv));
        // `f32x4_demote_f64x2_zero` writes lanes 0 and 1 and zeroes 2 and 3, so the recombine takes
        // lanes 0 and 1 of `low` and lanes 0 and 1 of `high` (indices 4 and 5).
        let all = i32x4_shuffle::<0, 1, 4, 5>(low, high);
        [
            f32x4_extract_lane::<0>(all),
            f32x4_extract_lane::<1>(all),
            f32x4_extract_lane::<2>(all),
            f32x4_extract_lane::<3>(all),
        ]
    }
}

/// MXCSR flush-to-zero bit (`FTZ`): denormal results become zero.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const MXCSR_FTZ: u32 = 0x8000;

/// MXCSR denormals-are-zero bit (`DAZ`): denormal operands are treated as zero.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const MXCSR_DAZ: u32 = 0x0040;

/// Reads the current thread's MXCSR control word.
///
/// Gate G6 support, never called from a render path.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(deprecated)]
#[must_use]
pub fn read_mxcsr() -> u32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_getcsr;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_getcsr;
    // SAFETY: `_mm_getcsr` reads a control register and is sound on any SSE host; SSE2 is baseline
    // on x86_64 and required by the crate's x86-64-v3 compile guard.
    unsafe { _mm_getcsr() }
}

/// Writes the current thread's MXCSR control word.
///
/// Gate G6 support, never called from a render path: FTZ and DAZ are *observed*, never relied on
/// (D7). The write affects only the calling thread, so a test must restore the previous value
/// before it returns.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(deprecated)]
pub fn write_mxcsr(value: u32) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_setcsr;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_setcsr;
    // SAFETY: `_mm_setcsr` writes a control register and is sound on any SSE host. The value is a
    // control word previously read by `read_mxcsr` with at most the FTZ and DAZ bits changed, so no
    // rounding mode or exception mask is disturbed.
    unsafe { _mm_setcsr(value) }
}
