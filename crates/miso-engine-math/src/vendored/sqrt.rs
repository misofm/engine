// Correctly rounded `sqrt` from an exact integer square root.
//
// libm 0.2.16 carries `sqrt` only as `src/math/generic/sqrt.rs`, generic over its private
// `support::Float` trait and, on most targets, short-circuited to a hardware instruction. Neither
// form is usable here: the crate is `no_std` (so `f64::sqrt` does not exist) and the policy is
// that every target executes the same operation sequence.
//
// Derivation. Write a finite positive `x` as `g * 2^F` with `g` an integer and `F` even. Then
// `sqrt(x) = sqrt(g) * 2^(F/2)`. With `N = g << 52`, `isqrt(N) = floor(sqrt(g) * 2^26)`, which has
// exactly 53 significant bits because `g` is normalised into `[2^52, 2^54)`. Writing `q` for that
// integer and `r = N - q*q`, the exact root lies strictly between `q` and `q + 1`, and
// `sqrt(N) > q + 1/2  <=>  N > q^2 + q + 1/4  <=>  r > q` (both sides integral). Equality is
// impossible, so there is never a tie to break and rounding to nearest is `r > q`. The result of
// `sqrt` on a finite positive `f64` is always normal, so no subnormal rounding case arises.

/// Correctly rounded square root (f64).
pub(crate) fn sqrt(x: f64) -> f64 {
    let bits = x.to_bits();
    let sign = bits >> 63;
    let biased_exp = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & 0x000f_ffff_ffff_ffff;

    if biased_exp == 0x7ff {
        // sqrt(NaN) = NaN, sqrt(+inf) = +inf, sqrt(-inf) = NaN.
        if frac != 0 || sign == 0 {
            return x + x;
        }
        return f64::NAN;
    }

    if bits << 1 == 0 {
        // sqrt(+-0) = +-0
        return x;
    }

    if sign == 1 {
        return f64::NAN;
    }

    // Decompose x = f * 2^e with f an integer in [2^52, 2^53).
    let (f, e) = if biased_exp == 0 {
        let shift = frac.leading_zeros() - 11;
        (frac << shift, -1022 - 52 - (shift as i32))
    } else {
        (frac | (1u64 << 52), biased_exp - 1023 - 52)
    };

    // Force an even exponent: x = g * 2^even.
    let odd = (e & 1) as u32;
    let g = (f as u128) << odd;
    let even = e - (odd as i32);

    let n = g << 52;
    let mut q = n.isqrt();
    if n - q * q > q {
        q += 1;
    }

    // q is in [2^52, 2^53]; renormalise the carry case.
    let mut p = even / 2 - 26;
    if q == 1u128 << 53 {
        q >>= 1;
        p += 1;
    }

    let biased = (p + 52 + 1023) as u64;
    f64::from_bits((biased << 52) | ((q as u64) & 0x000f_ffff_ffff_ffff))
}

/// Correctly rounded square root (f32).
///
/// Computed through [`sqrt`]: `f64` carries 53 bits, more than the `2p + 2 = 50` a single rounding
/// of a `f32` square root needs, so rounding the exact `f64` root back to `f32` is the correctly
/// rounded `f32` root (no double-rounding error is possible for square roots under that bound).
pub(crate) fn sqrtf(x: f32) -> f32 {
    if x.is_nan() {
        return x + x;
    }
    sqrt(x as f64) as f32
}
