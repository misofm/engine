//! One `Lane` body for both `wide` widths.
//!
//! `simd4.rs` and `simd8.rs` differ only in the vector type, the lane count and the unsigned
//! companion type, so the implementation lives here once and is instantiated twice. A second copy
//! of an operation body is exactly the defect this crate exists to remove (master plan §1.4).
//!
//! # What is *not* forwarded to `wide`
//!
//! `wide` is a vocabulary, not a semantics authority (master plan §3.3, verified in 1.6.1 source):
//!
//! * `max`/`min` are `maxps` plus a NaN fix-up on SSE, `pmax` plus a fix-up on wasm and
//!   `vmaxnmq` (IEEE `maxNum`) on NEON — `max(+0.0, -0.0)` is `-0.0` on x86 and `+0.0` on NEON.
//!   The trait's D8 default (`select(a > b, a, b)`) is used on every backend instead.
//! * `mul_add` is fused only when the build enables `+fma` on x86 or targets NEON, and is silently
//!   `(a * b) + c` otherwise. It is forwarded only on those two targets — guarded by the crate's
//!   `compile_error!` on x86 — and every other target uses the exact software FMA.
//! * `select` uses `blendv`, which inspects only the sign bit. `bitselect` is the bitwise form the
//!   scalar oracle defines, so `Lane::select` forwards to `bitselect`.
//!
//! Everything else (`add`, `sub`, `mul`, `div`, `sqrt`, `floor`, `abs`, `neg`, the ordered
//! comparisons, and the bit operations) forwards directly.

/// Implements [`crate::Lane`] for one `wide` vector type.
///
/// Arguments: the vector type, its unsigned companion, the lane count, and the software FMA to use
/// on targets without a hardware fused multiply-add.
macro_rules! impl_lane_for_wide {
    ($simd:ty, $uint:ty, $width:literal, $soft_fma:path) => {
        impl $crate::Lane for $simd {
            const WIDTH: usize = $width;
            type Mask = $simd;

            #[inline(always)]
            fn splat(x: f32) -> Self {
                <$simd>::splat(x)
            }

            #[inline(always)]
            fn zero() -> Self {
                <$simd>::splat(0.0)
            }

            #[inline(always)]
            fn load(src: &[f32]) -> Self {
                let mut lanes = [0.0f32; $width];
                lanes.copy_from_slice(&src[..$width]);
                <$simd>::new(lanes)
            }

            #[inline(always)]
            fn store(self, dst: &mut [f32]) {
                dst[..$width].copy_from_slice(&self.to_array());
            }

            #[inline(always)]
            fn add(self, b: Self) -> Self {
                self + b
            }

            #[inline(always)]
            fn sub(self, b: Self) -> Self {
                self - b
            }

            #[inline(always)]
            fn mul(self, b: Self) -> Self {
                self * b
            }

            #[inline(always)]
            fn div(self, b: Self) -> Self {
                self / b
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                // LANE-OP-OK(sqrt): forwards directly, IEEE-exact on every target (§3.3).
                self.sqrt()
            }

            #[inline(always)]
            fn fma(self, b: Self, c: Self) -> Self {
                #[cfg(any(
                    target_arch = "x86",
                    target_arch = "x86_64",
                    all(target_arch = "aarch64", target_feature = "neon")
                ))]
                {
                    // LANE-OP-OK(mul_add): hardware FMA only. On x86 the crate's `compile_error!`
                    // guarantees `+fma`, so this is `vfmadd`; on aarch64 `wide` lowers it to
                    // `vfmaq_f32`. Both are the single-rounding IEEE operation D3 pins.
                    self.mul_add(b, c)
                }
                #[cfg(not(any(
                    target_arch = "x86",
                    target_arch = "x86_64",
                    all(target_arch = "aarch64", target_feature = "neon")
                )))]
                {
                    <$simd>::new($soft_fma(self.to_array(), b.to_array(), c.to_array()))
                }
            }

            #[inline(always)]
            fn neg(self) -> Self {
                -self
            }

            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn floor(self) -> Self {
                // LANE-OP-OK(floor): forwards directly, IEEE floor on every target (§3.3).
                self.floor()
            }

            #[inline(always)]
            fn lt(self, b: Self) -> Self::Mask {
                self.simd_lt(b)
            }

            #[inline(always)]
            fn le(self, b: Self) -> Self::Mask {
                self.simd_le(b)
            }

            #[inline(always)]
            fn gt(self, b: Self) -> Self::Mask {
                self.simd_gt(b)
            }

            #[inline(always)]
            fn ge(self, b: Self) -> Self::Mask {
                self.simd_ge(b)
            }

            #[inline(always)]
            fn eq(self, b: Self) -> Self::Mask {
                self.simd_eq(b)
            }

            #[inline(always)]
            fn mask_and(a: Self::Mask, b: Self::Mask) -> Self::Mask {
                a & b
            }

            #[inline(always)]
            fn mask_or(a: Self::Mask, b: Self::Mask) -> Self::Mask {
                a | b
            }

            #[inline(always)]
            fn mask_not(a: Self::Mask) -> Self::Mask {
                !a
            }

            #[inline(always)]
            fn mask_any(m: Self::Mask) -> bool {
                m.any()
            }

            #[inline(always)]
            fn select(m: Self::Mask, a: Self, b: Self) -> Self {
                m.bitselect(a, b)
            }

            #[inline(always)]
            fn andnot(self, m: Self::Mask) -> Self {
                self & !m
            }

            #[inline(always)]
            fn exp2_int(n: Self) -> Self {
                let n = $crate::Lane::min(
                    $crate::Lane::max(n, <$simd>::splat($crate::bits::EXP2_INT_MIN)),
                    <$simd>::splat($crate::bits::EXP2_INT_MAX),
                );
                let biased = n + <$simd>::splat($crate::bits::EXP2_INT_MAGIC);
                <$simd>::from_bits(biased.to_bits() << $crate::bits::MANTISSA_BITS)
            }

            #[inline(always)]
            fn frexp(self) -> (Self, Self) {
                let bits = self.to_bits();
                let significand = <$simd>::from_bits(
                    (bits & <$uint>::splat($crate::bits::MANTISSA_MASK))
                        | <$uint>::splat($crate::bits::ONE_EXPONENT_BITS),
                );
                let exponent = <$simd>::from_bits(
                    (bits >> $crate::bits::MANTISSA_BITS)
                        | <$uint>::splat($crate::bits::EXPONENT_MAGIC_BITS),
                ) - <$simd>::splat($crate::bits::EXP2_INT_MAGIC);
                (significand, exponent)
            }

            #[inline(always)]
            fn store_bits(self, dst: &mut [u32]) {
                dst[..$width].copy_from_slice(&self.to_bits().to_array());
            }
        }
    };
}

pub(crate) use impl_lane_for_wide;
