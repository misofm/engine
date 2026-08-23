// Vendored from rust-lang/libm 0.2.16 (MIT), src/math/tanhf.rs.
// Target-conditional code, `force_eval!` FP-flag statements, `no_panic` attributes and
// upstream unit tests are removed so every target executes an identical operation sequence.
// See LICENSE-libm.txt and VENDORED.md.
use super::expm1f;

/// The hyperbolic tangent of `x` (f32).
///
/// `x` is specified in radians.
pub(crate) fn tanhf(mut x: f32) -> f32 {
    /* x = |x| */
    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;
    x = f32::from_bits(ix);
    let w = ix;

    let tt = if w > 0x3f0c9f54 {
        /* |x| > log(3)/2 ~= 0.5493 or nan */
        if w > 0x41200000 {
            /* |x| > 10 */
            1. + 0. / x
        } else {
            let t = expm1f(2. * x);
            1. - 2. / (t + 2.)
        }
    } else if w > 0x3e82c578 {
        /* |x| > log(5/3)/2 ~= 0.2554 */
        let t = expm1f(2. * x);
        t / (t + 2.)
    } else if w >= 0x00800000 {
        /* |x| >= 0x1p-126 */
        let t = expm1f(-2. * x);
        -t / (t + 2.)
    } else {
        /* |x| is subnormal */
        x
    };
    if sign { -tt } else { tt }
}
