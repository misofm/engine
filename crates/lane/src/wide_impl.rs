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
//!   `wide::max` and `wide::min` are forwarded on no backend, and the trait's D8 rule
//!   (`select(a > b, a, b)`) remains the whole of the semantics. What the D8 rule is *lowered to*
//!   is chosen per backend; see [Lowerings chosen per backend](#lowerings-chosen-per-backend).
//! * `mul_add` is fused only when the build enables `+fma` on x86 or targets NEON, and is silently
//!   `(a * b) + c` otherwise — a per-target semantics split, which is exactly what the numeric
//!   contract forbids. It is not forwarded at all: [`crate::Lane::fma`] is `(a * b) + c` written
//!   out, on every backend (issue #163 phase 2).
//! * `bitselect` is `(a & m) | (b & !m)`, three FP-logic instructions on x86. `Lane::select`
//!   forwards to `wide`'s `select` instead, which is the same bit-blend on every mask this engine
//!   can build and one instruction on x86; again see
//!   [Lowerings chosen per backend](#lowerings-chosen-per-backend).
//!
//! Everything else (`add`, `sub`, `mul`, `div`, `sqrt`, `floor`, `abs`, `neg`, the ordered
//! comparisons, and the bit operations) forwards directly.
//!
//! # Lowerings chosen per backend
//!
//! Three operations below are written as the instruction the target actually has rather than as
//! the trait's portable form. None of them is a semantics change: each is the *same function of
//! its inputs*, argued case by case here and checked against the scalar oracle by gate G1
//! (`tests/g1_op_identity.rs`), which sweeps every ordered pair of the directed edge pool — both
//! signed zeros, both NaN payloads in both operand positions, both infinities, both subnormal
//! boundaries — at `Simd4` and `Simd8`, plus the dedicated per-lowering truth-table test
//! `g1_max_and_min_lowerings_match_the_oracle`. Rows 1 and 2 of `tests/MUTATIONS.md` record that
//! same pool going red for exactly these substitutions made wrong.
//!
//! ## `select`: `blendv` instead of `bitselect`
//!
//! `wide` 1.6.1 lowers `select` to `blendv_ps` under `sse4.1`/`avx` (one instruction), to
//! `v128.bitselect` under `simd128`, and to `vbslq_f32` on AArch64 NEON. The last two are *the
//! identical call* `bitselect` makes on those targets — same intrinsic, same operand order — so
//! this line moves emitted code on x86 only and is literally a no-op for wasm and NEON codegen.
//!
//! `blendv` reads bit 31 of each mask lane and nothing else; `bitselect` reads all 32 bits. The
//! two therefore agree exactly when every mask lane is all-zero or all-one bits, and can disagree
//! only on a mask that is neither. [`crate::Lane`]'s contract is that a mask lane *is* one of
//! those two — "A [`crate::Lane::Mask`] lane is either all zero bits or all one bits — masks are
//! produced only by the comparison and mask operations of this trait" — and that closure is real
//! rather than aspirational: the only mask producers in the workspace are this trait's five
//! ordered comparisons, its three mask combinators (which map canonical masks to canonical masks),
//! and the four constructors in [`crate::kernels::builtins`]
//! (`no_lanes`, `all_lanes`, `lanes_below`, `mask_from_flags`), each of which is itself written as
//! an ordered comparison for precisely this reason. Every reachable mask is canonical, so the two
//! lowerings coincide bit for bit on every input the engine can construct.
//!
//! `docs/rulings/effect-floor-accounting.md` counts `Lane::select` as **one** lane-op because
//! "masks here come only from comparisons, so `vblendvps` is a legal lowering", records that the
//! emitted code was three, and lists "a compiler that forms `vblendvps` from `bitselect`" among
//! the conditions that reopen its floor-gap accounting. This line is that condition, arrived at by
//! writing the instruction rather than by waiting for a compiler: it closes gap term 1 (24 of the
//! compressor's 221 non-floor instructions per channel-frame) and moves no floor, because the
//! floor already assumed the one-instruction form.
//!
//! ## `max`/`min`: one instruction where the target has one
//!
//! D8 is `max(a, b) = select(a > b, a, b)` and `min(a, b) = select(a < b, a, b)`: a strictly
//! ordered compare, so `b` wins every tie and every unordered pair, and the winner's bits are
//! copied through verbatim. Two targets have a single instruction with exactly that rule.
//!
//! * **x86** (Intel SDM, `MAXPS`/`MINPS`): `MAX(SRC1, SRC2)` returns `SRC2` when both are zero,
//!   `SRC2` when either operand is a NaN, `SRC1` when `SRC1 > SRC2`, and `SRC2` otherwise — that
//!   is `SRC1 > SRC2 ? SRC1 : SRC2`, with the chosen operand written verbatim (a NaN is neither
//!   quieted nor canonicalised). `MINPS` is the mirror. So `maxps(a, b)` *is* D8 `max(a, b)` and
//!   `minps(a, b)` is D8 `min(a, b)`; `wide` exposes them unfixed-up as `fast_max`/`fast_min`.
//! * **wasm `simd128`**: `f32x4.pmax(z1, z2)` is `z1 < z2 ? z2 : z1` and `f32x4.pmin(z1, z2)` is
//!   `z2 < z1 ? z2 : z1`, both returning an operand unchanged. Substituting `z1 = b`, `z2 = a`:
//!   `pmax(b, a) = (b < a ? a : b) = (a > b ? a : b)` and `pmin(b, a) = (a < b ? a : b)`. The
//!   operands are therefore **swapped** relative to x86, which is the whole subtlety of this
//!   lowering. `wide`'s `fast_max`/`fast_min` emit `pmax`/`pmin` with `self` first, so the calls
//!   below take `b` as the receiver and `self` as the argument.
//! * **AArch64 NEON**: `vmaxq_f32` propagates NaN and answers `+0.0` for `max(+0.0, -0.0)`, and
//!   `vmaxnmq_f32` is IEEE `maxNum`, which swallows a right-hand NaN. Neither is D8, so NEON keeps
//!   the trait's portable form. This is a measured null, not an oversight.
//! * **Every other target**, wasm without `simd128` included, keeps the portable form too:
//!   `wide`'s scalar-array fallback for `fast_max` is `a < b ? b : a`, which answers `a` on a tie
//!   where D8 answers `b`.
//!
//! The pairs that separate these lowerings, with `N1 = 0x7FC0_0000` and `N2 = 0xFFC0_0001` two
//! distinct NaN bit patterns:
//!
//! | `a` | `b` | D8 `max(a, b)` | x86 `maxps(a, b)` | wasm `pmax(b, a)` | NEON `vmaxnmq(a, b)` |
//! |---|---|---|---|---|---|
//! | `+0.0` | `-0.0` | `-0.0` | `-0.0` | `-0.0` | `+0.0` ✗ |
//! | `-0.0` | `+0.0` | `+0.0` | `+0.0` | `+0.0` | `+0.0` |
//! | `1.0` | `1.0` | `1.0` (`b`) | `SRC2` = `b` | `b` | `1.0` |
//! | `N1` | `1.0` | `1.0` | `SRC2` = `1.0` | `1.0` | `1.0` |
//! | `1.0` | `N2` | `N2` | `SRC2` = `N2` | `N2` | `1.0` ✗ |
//! | `N1` | `N2` | `N2` | `SRC2` = `N2` | `N2` | canonical NaN ✗ |
//! | `2.0` | `1.0` | `2.0` (`a`) | `SRC1` = `a` | `a` | `2.0` |
//! | `2^-149` | `+0.0` | `2^-149` (`a`) | `SRC1` = `a` | `a` | `2^-149` |
//! | `-inf` | `-inf` | `-inf` (`b`) | `SRC2` = `b` | `b` | `-inf` |
//!
//! `min` is the same table with the comparison reversed, and `tests/g1_op_identity.rs` runs both
//! over every ordered pair of the pool rather than over these nine rows alone.
//!
//! The accounting consequence is recorded in `docs/rulings/effect-floor-accounting.md` (#368):
//! max/min are one lane-op on x86 and wasm, while NEON retains the two-operation portable shape.
//! The select floor already assumed the single-instruction form and therefore does not move.
//!
//! ## The one precondition: DAZ must be clear
//!
//! The x86 equality above holds in the architectural default floating-point environment and is not
//! unconditional. With `MXCSR.DAZ` set, a subnormal source operand is converted to a zero of its
//! own sign *before use*, so `maxps(2^-149, -1.0)` answers `+0.0` while the compare-and-blend
//! form answers `2^-149`: `cmpps` reads the same flushed value, but `blendv` is a bit operation and
//! copies the original lane. Every native render entry installs `CANONICAL_MXCSR` (`0x1F80`, DAZ
//! and FTZ clear) for the length of the block and restores the caller's word on the way out
//! ([`crate::fpenv`], issue #146), and
//! `tools/wasm-gates/tests/g6_full_corpus_ftz.rs` is the standing proof that a
//! caller's FTZ+DAZ never reaches a render. wasm has no such mode at all. A world in which the
//! `fpenv` guard is removed is a world in which this lowering must be revisited, which is why the
//! dependency is written down here rather than left implicit.

/// Implements [`crate::Lane`] for one `wide` vector type.
///
/// Arguments: the vector type, its unsigned companion, the lane count, and the tuned SVF cascade
/// depth ([`crate::Lane::SVF_CASCADE_DEPTH`], issue #163 phase 3).
macro_rules! impl_lane_for_wide {
    ($simd:ty, $uint:ty, $width:literal, $cascade_depth:literal) => {
        impl $crate::Lane for $simd {
            const WIDTH: usize = $width;
            const SVF_CASCADE_DEPTH: usize = $cascade_depth;
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
                // Two roundings, natively, on every backend (issue #163 phase 2). There is no
                // `cfg` here and that is the point: the multiply rounds, then the add rounds, and
                // both are IEEE basic operations every target implements identically. The
                // hardware fused instruction is deliberately *not* used on the targets that have
                // one, because using it there would make the contract per-backend.
                (self * b) + c
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
                // `wide`'s `select(self, if_true, if_false)` takes its operands in `bitselect`'s
                // order and lowers to `blendv` on x86, to `v128.bitselect` on wasm and to `vbsl`
                // on NEON -- the last two being the same call `bitselect` makes. Every mask this
                // trait can produce is canonical, so the sign-bit form and the bitwise form are
                // the same function here; see the module documentation.
                m.select(a, b)
            }

            #[inline(always)]
            fn andnot(self, m: Self::Mask) -> Self {
                self & !m
            }

            #[inline(always)]
            fn max(self, b: Self) -> Self {
                // x86: `maxps`/`vmaxps` is `SRC1 > SRC2 ? SRC1 : SRC2`, which is D8 exactly.
                // The crate refuses to compile on x86 without `+avx2,+fma`, so both widths reach
                // a single-instruction arm of `wide`'s `fast_max`.
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // LANE-OP-OK(fast_max): the unfixed-up `maxps`, equal to the D8 form for
                    // every ordered pair (module documentation); DAZ is pinned clear by `fpenv`.
                    self.fast_max(b)
                }
                // wasm: `f32x4.pmax(z1, z2)` is `z1 < z2 ? z2 : z1`, so the operands swap.
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    // LANE-OP-OK(fast_max): `pmax(b, a)`, the operand order D8 requires. Written
                    // `b.fast_max(self)` because `wide` passes `self` to `pmax` first.
                    b.fast_max(self)
                }
                // NEON has no instruction with this rule, and `wide`'s scalar-array fallback
                // answers `a` on a tie where D8 answers `b`. Both keep the portable form.
                #[cfg(not(any(
                    target_arch = "x86",
                    target_arch = "x86_64",
                    all(target_arch = "wasm32", target_feature = "simd128")
                )))]
                {
                    $crate::Lane::select($crate::Lane::gt(self, b), self, b)
                }
            }

            #[inline(always)]
            fn min(self, b: Self) -> Self {
                // x86: `minps`/`vminps` is `SRC1 < SRC2 ? SRC1 : SRC2`, the mirror of `maxps`.
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // LANE-OP-OK(fast_min): the unfixed-up `minps`; see `max` above.
                    self.fast_min(b)
                }
                // wasm: `f32x4.pmin(z1, z2)` is `z2 < z1 ? z2 : z1`, so the operands swap here too.
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    // LANE-OP-OK(fast_min): `pmin(b, a)` = `a < b ? a : b`, which is D8 `min`.
                    b.fast_min(self)
                }
                #[cfg(not(any(
                    target_arch = "x86",
                    target_arch = "x86_64",
                    all(target_arch = "wasm32", target_feature = "simd128")
                )))]
                {
                    $crate::Lane::select($crate::Lane::lt(self, b), self, b)
                }
            }

            #[inline(always)]
            fn exp2_int(n: Self) -> Self {
                let n = $crate::Lane::min(
                    $crate::Lane::max(n, <$simd>::splat($crate::bits::EXP2_INT_MIN)),
                    <$simd>::splat($crate::bits::EXP2_INT_MAX),
                );
                $crate::Lane::exp2_int_in_range(n)
            }

            #[inline(always)]
            fn exp2_int_in_range(n: Self) -> Self {
                debug_assert!(!<$simd as $crate::Lane>::mask_any(
                    <$simd as $crate::Lane>::mask_not(<$simd as $crate::Lane>::mask_and(
                        <$simd as $crate::Lane>::ge(n, <$simd>::splat($crate::bits::EXP2_INT_MIN),),
                        <$simd as $crate::Lane>::le(n, <$simd>::splat($crate::bits::EXP2_INT_MAX),),
                    )),
                ));
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
