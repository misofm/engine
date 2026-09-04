//! The scalar oracle: `impl Lane for f32`, `WIDTH = 1`.
//!
//! This file is the definition of the numeric contract. It is written operation by operation to
//! the master plan §3.2 table — to the packed instruction each operation stands for — and never to
//! a `std` convenience. Two `std` conveniences in particular are forbidden here and everywhere
//! else on a render path: `f32::max` and `f32::min`, whose NaN rule is not D8's.
//!
//! Every other backend is proven equal to this one, lane by lane and bit for bit, by gate G1.

use crate::Lane;
use crate::bits::{
    EXP2_INT_MAGIC, EXP2_INT_MAX, EXP2_INT_MIN, EXPONENT_MAGIC_BITS, MANTISSA_BITS, MANTISSA_MASK,
    ONE_EXPONENT_BITS,
};

/// A scalar mask: exactly `0` or `u32::MAX`.
type ScalarMask = u32;

/// All bits set: the `true` mask.
const TRUE: ScalarMask = u32::MAX;

/// No bits set: the `false` mask.
const FALSE: ScalarMask = 0;

/// Maps a Rust `bool` from an ordered comparison to the two legal mask patterns.
#[inline(always)]
fn mask(condition: bool) -> ScalarMask {
    if condition { TRUE } else { FALSE }
}

impl Lane for f32 {
    const WIDTH: usize = 1;
    // #163 phase 3: the whole four-section cascade. The scalar oracle keeps one `f32` per
    // integrator, so eight live recurrences do not exhaust the register file the way two `Simd8`
    // streams times four sections do. Measured 2.088x against 1.773x at depth 2.
    const SVF_CASCADE_DEPTH: usize = 4;
    type Mask = ScalarMask;

    #[inline(always)]
    fn splat(x: f32) -> Self {
        x
    }

    #[inline(always)]
    fn zero() -> Self {
        0.0
    }

    #[inline(always)]
    fn load(src: &[f32]) -> Self {
        src[0]
    }

    #[inline(always)]
    fn store(self, dst: &mut [f32]) {
        dst[0] = self;
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
        // LANE-OP-OK(sqrt): IEEE-exact on every target, the one std float call the contract allows.
        f32::sqrt(self)
    }

    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self {
        // Two roundings: the multiply, then the add (issue #163 phase 2). `f32::mul_add` is
        // deliberately not called -- it would be a single rounding here and on the vector
        // backends only where the hardware has one, which is the per-backend split the contract
        // forbids. The scalar Lane is the oracle every other backend is compared against, so it
        // is written in the same two IEEE basic operations they are.
        (self * b) + c
    }

    #[inline(always)]
    fn neg(self) -> Self {
        // Sign-bit XOR, not `0.0 - self`, which turns `+0.0` into `+0.0` instead of `-0.0`.
        f32::from_bits(self.to_bits() ^ 0x8000_0000)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        f32::from_bits(self.to_bits() & 0x7FFF_FFFF)
    }

    #[inline(always)]
    fn floor(self) -> Self {
        // LANE-OP-OK(floor): IEEE floor, exact on every target.
        f32::floor(self)
    }

    #[inline(always)]
    fn lt(self, b: Self) -> Self::Mask {
        mask(self < b)
    }

    #[inline(always)]
    fn le(self, b: Self) -> Self::Mask {
        mask(self <= b)
    }

    #[inline(always)]
    fn gt(self, b: Self) -> Self::Mask {
        mask(self > b)
    }

    #[inline(always)]
    fn ge(self, b: Self) -> Self::Mask {
        mask(self >= b)
    }

    #[inline(always)]
    fn eq(self, b: Self) -> Self::Mask {
        mask(self == b)
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
        m != 0
    }

    #[inline(always)]
    fn select(m: Self::Mask, a: Self, b: Self) -> Self {
        // Bitwise, so that a partially set mask behaves the same here as under `vblendvps`'
        // bitwise counterpart `bitselect` used by the vector backends.
        f32::from_bits((a.to_bits() & m) | (b.to_bits() & !m))
    }

    #[inline(always)]
    fn andnot(self, m: Self::Mask) -> Self {
        f32::from_bits(self.to_bits() & !m)
    }

    #[inline(always)]
    fn exp2_int(n: Self) -> Self {
        let n = Lane::min(Lane::max(n, EXP2_INT_MIN), EXP2_INT_MAX);
        Lane::exp2_int_in_range(n)
    }

    #[inline(always)]
    fn exp2_int_in_range(n: Self) -> Self {
        debug_assert!(n >= EXP2_INT_MIN && n <= EXP2_INT_MAX);
        let biased = n + EXP2_INT_MAGIC;
        f32::from_bits(biased.to_bits() << MANTISSA_BITS)
    }

    #[inline(always)]
    fn frexp(self) -> (Self, Self) {
        let bits = self.to_bits();
        let significand = f32::from_bits((bits & MANTISSA_MASK) | ONE_EXPONENT_BITS);
        let exponent =
            f32::from_bits((bits >> MANTISSA_BITS) | EXPONENT_MAGIC_BITS) - EXP2_INT_MAGIC;
        (significand, exponent)
    }

    #[inline(always)]
    fn store_bits(self, dst: &mut [u32]) {
        dst[0] = self.to_bits();
    }
}
