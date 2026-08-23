// `floor` in the non-generic musl shape.
//
// libm 0.2.16 carries `floor` only as `src/math/generic/floor.rs`, generic over its private
// `support::Float` trait; the algorithm below is that file specialised to `f64`/`f32`, with the
// FP-status plumbing (which only sets the inexact flag) removed. Pure integer work, so every
// target executes the same sequence.

/// Largest integer `<= x` (f64).
pub(crate) fn floor(x: f64) -> f64 {
    const SIG_BITS: i32 = 52;
    const SIG_MASK: u64 = 0x000f_ffff_ffff_ffff;

    let mut ix = x.to_bits();
    let e = (((ix >> SIG_BITS) & 0x7ff) as i32) - 0x3ff;

    // Infinities, NaNs and values with no fractional part pass through unchanged.
    if e >= SIG_BITS {
        return x;
    }

    if e >= 0 {
        // |x| >= 1.0
        let m = SIG_MASK >> e;
        if ix & m == 0 {
            return x;
        }
        if x.is_sign_negative() {
            ix += m;
        }
        f64::from_bits(ix & !m)
    } else if x.is_sign_positive() {
        // 0.0 <= x < 1.0
        0.0
    } else if ix << 1 != 0 {
        // -1.0 < x < 0.0
        -1.0
    } else {
        // -0.0
        x
    }
}

/// Largest integer `<= x` (f32).
pub(crate) fn floorf(x: f32) -> f32 {
    const SIG_BITS: i32 = 23;
    const SIG_MASK: u32 = 0x007f_ffff;

    let mut ix = x.to_bits();
    let e = (((ix >> SIG_BITS) & 0xff) as i32) - 0x7f;

    if e >= SIG_BITS {
        return x;
    }

    if e >= 0 {
        let m = SIG_MASK >> e;
        if ix & m == 0 {
            return x;
        }
        if x.is_sign_negative() {
            ix += m;
        }
        f32::from_bits(ix & !m)
    } else if x.is_sign_positive() {
        0.0
    } else if ix << 1 != 0 {
        -1.0
    } else {
        x
    }
}
