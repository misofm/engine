//! Bit-level constants shared by every [`crate::Lane`] implementation.
//!
//! `exp2_int` and `frexp` are written as bit-field constructions rather than as float↔int
//! conversions on purpose: `f32 as i32` saturates in Rust, `i32x4.trunc_sat` saturates in wasm and
//! `vcvttps2dq` produces `0x8000_0000` out of range, so a conversion-based form would have three
//! different out-of-range behaviours. The forms below use only add, `to_bits`, shift and
//! `from_bits`, which are identical on every backend by construction; the D8 clamp in
//! `Lane::exp2_int` keeps the input in range so the add is exact for integer-valued inputs.

/// `2^23 + 127`, added to an integer-valued exponent so that the low 23 mantissa bits of the sum
/// are exactly the biased exponent `n + 127`.
///
/// For `n` in `[-126, 127]` the sum is exact: `2^23 + m` with `m = n + 127` in `[1, 254]` has the
/// bit pattern `0x4B00_0000 | m`, and shifting that pattern left by 23 drops the constant (its low
/// nine bits are zero) and leaves `m` in the exponent field.
pub(crate) const EXP2_INT_MAGIC: f32 = 8_388_608.0 + 127.0;

/// Bits of `2^23`, or-ed onto a biased exponent to read it back as a float without a conversion
/// instruction: `from_bits(0x4B00_0000 | e)` is `2^23 + e` exactly, so subtracting
/// [`EXP2_INT_MAGIC`] yields the unbiased exponent as an exactly representable `f32`.
pub(crate) const EXPONENT_MAGIC_BITS: u32 = 0x4B00_0000;

/// Mantissa field of an `f32`.
pub(crate) const MANTISSA_MASK: u32 = 0x007F_FFFF;

/// Exponent field of `1.0`, or-ed onto a mantissa to build the `[1, 2)` significand of `frexp`.
pub(crate) const ONE_EXPONENT_BITS: u32 = 0x3F80_0000;

/// Number of mantissa bits: the shift between the exponent field and bit 0.
pub(crate) const MANTISSA_BITS: u32 = 23;

/// Lowest exponent [`crate::Lane::exp2_int`] accepts.
pub(crate) const EXP2_INT_MIN: f32 = -126.0;

/// Highest exponent [`crate::Lane::exp2_int`] accepts.
pub(crate) const EXP2_INT_MAX: f32 = 127.0;
