//! Correctness of the vendored scalar layer.
//!
//! The vendoring edits (VENDORED.md) are mechanical, but "mechanical" is not evidence. These tests
//! check the vendored functions against the host platform's libm through `std`, which is an
//! independent implementation of the same specifications, and check the two functions written here
//! rather than vendored (`floor`, `sqrt`) exhaustively where that is possible.
//!
//! `std` transcendental calls are legal here: `scripts/check-math-policy.sh` scans `src`, not
//! `tests` — a test comparing against the platform libm is the point.

use math as m;

/// Error of `got` against `want`, in units in the last place of `want`.
fn ulp_error_f64(got: f64, want: f64) -> f64 {
    if got == want {
        return 0.0;
    }
    assert!(
        want.is_finite(),
        "oracle produced {want} for a case that must be finite"
    );
    let spacing = {
        let up = f64::from_bits(want.abs().to_bits() + 1);
        up - want.abs()
    };
    ((got - want) / spacing).abs()
}

/// Error of `got` against the `f64` oracle `want`, in units in the last place of the `f32` result.
fn ulp_error_f32(got: f32, want: f64) -> f64 {
    let want_f32 = want as f32;
    if got == want_f32 && (f64::from(got) - want) == 0.0 {
        return 0.0;
    }
    assert!(
        want_f32.is_finite(),
        "oracle produced {want_f32} for a case that must be finite"
    );
    let spacing = {
        let up = f32::from_bits(want_f32.abs().to_bits() + 1);
        f64::from(up) - f64::from(want_f32.abs())
    };
    ((f64::from(got) - want) / spacing).abs()
}

/// Deterministic sample points spread over the argument range `[lo, hi]`.
fn sweep(lo: f64, hi: f64, count: usize) -> impl Iterator<Item = f64> {
    (0..count).map(move |i| lo + (hi - lo) * (i as f64) / ((count - 1) as f64))
}

#[test]
fn exact_anchors() {
    assert_eq!(m::exp2(1.0), 2.0);
    assert_eq!(m::exp2(0.0), 1.0);
    assert_eq!(m::exp2(10.0), 1024.0);
    assert_eq!(m::log2(8.0), 3.0);
    assert_eq!(m::log2(1.0), 0.0);
    assert_eq!(m::log10(1000.0), 3.0);
    assert_eq!(m::log(1.0), 0.0);
    assert_eq!(m::pow(2.0, 10.0), 1024.0);
    assert_eq!(m::pow(9.0, 0.5), 3.0);
    assert_eq!(m::sin(0.0), 0.0);
    assert_eq!(m::cos(0.0), 1.0);
    assert_eq!(m::tanh(0.0), 0.0);

    assert_eq!(m::exp2f(1.0), 2.0);
    assert_eq!(m::exp2f(0.0), 1.0);
    assert_eq!(m::log2f(8.0), 3.0);
    assert_eq!(m::log2f(1.0), 0.0);
    assert_eq!(m::powf(2.0, 10.0), 1024.0);
    assert_eq!(m::powf(9.0, 0.5), 3.0);
    assert!(ulp_error_f32(m::tanf(core::f32::consts::FRAC_PI_4), 1.0) <= 1.0);

    assert_eq!(m::db_to_gain(0.0), 1.0);
    assert_eq!(m::db_to_gain_f32(0.0), 1.0);
    assert_eq!(m::gain_to_db(1.0), 0.0);
    assert_eq!(m::gain_to_db_f32(1.0), 0.0);
    assert!((m::db_to_gain(-6.0) - 0.5011872336272722).abs() < 1e-15);
    assert!((m::gain_to_db(0.5) + 6.020599913279624).abs() < 1e-13);
}

#[test]
fn f64_functions_match_platform_libm() {
    // (name, vendored, platform, lo, hi, tolerance in ulp)
    #[allow(clippy::type_complexity)]
    let cases: &[(&str, fn(f64) -> f64, fn(f64) -> f64, f64, f64, f64)] = &[
        ("exp", m::exp, f64::exp, -700.0, 700.0, 1.0),
        ("exp2", m::exp2, f64::exp2, -1000.0, 1000.0, 1.0),
        ("expm1", m::expm1, f64::exp_m1, -30.0, 30.0, 1.0),
        ("log", m::log, f64::ln, 1e-300, 1e300, 1.0),
        ("log2", m::log2, f64::log2, 1e-300, 1e300, 1.0),
        ("log10", m::log10, f64::log10, 1e-300, 1e300, 1.0),
        ("sin", m::sin, f64::sin, -1000.0, 1000.0, 1.0),
        ("cos", m::cos, f64::cos, -1000.0, 1000.0, 1.0),
        ("tan", m::tan, f64::tan, -1000.0, 1000.0, 2.0),
        ("tanh", m::tanh, f64::tanh, -30.0, 30.0, 2.0),
        ("atan", m::atan, f64::atan, -1e6, 1e6, 1.0),
        ("sqrt", m::sqrt, f64::sqrt, 0.0, 1e300, 0.0),
        ("floor", m::floor, f64::floor, -1e17, 1e17, 0.0),
    ];

    for &(name, ours, theirs, lo, hi, tol) in cases {
        let mut worst = 0.0_f64;
        let mut worst_at = 0.0_f64;
        for x in sweep(lo, hi, 1000) {
            let want = theirs(x);
            if !want.is_finite() {
                continue;
            }
            let err = ulp_error_f64(ours(x), want);
            if err > worst {
                worst = err;
                worst_at = x;
            }
        }
        assert!(
            worst <= tol,
            "{name}: {worst} ulp at x = {worst_at} exceeds {tol}"
        );
    }

    // Two-argument functions.
    let mut worst_pow = 0.0_f64;
    for x in sweep(0.01, 100.0, 100) {
        for y in sweep(-20.0, 20.0, 100) {
            let want = x.powf(y);
            if !want.is_finite() || want == 0.0 {
                continue;
            }
            worst_pow = worst_pow.max(ulp_error_f64(m::pow(x, y), want));
        }
    }
    assert!(worst_pow <= 1.0, "pow: {worst_pow} ulp");

    let mut worst_atan2 = 0.0_f64;
    for y in sweep(-100.0, 100.0, 100) {
        for x in sweep(-100.0, 100.0, 100) {
            let want = y.atan2(x);
            worst_atan2 = worst_atan2.max(ulp_error_f64(m::atan2(y, x), want));
        }
    }
    assert!(worst_atan2 <= 1.0, "atan2: {worst_atan2} ulp");
}

#[test]
fn f32_functions_match_platform_libm_in_f64() {
    #[allow(clippy::type_complexity)]
    let cases: &[(&str, fn(f32) -> f32, fn(f64) -> f64, f32, f32, f64)] = &[
        ("expf", m::expf, f64::exp, -80.0, 80.0, 1.0),
        ("exp2f", m::exp2f, f64::exp2, -120.0, 120.0, 1.0),
        ("expm1f", m::expm1f, f64::exp_m1, -20.0, 20.0, 1.5),
        ("logf", m::logf, f64::ln, 1e-30, 1e30, 1.0),
        ("log2f", m::log2f, f64::log2, 1e-30, 1e30, 1.0),
        ("log10f", m::log10f, f64::log10, 1e-30, 1e30, 1.0),
        ("sinf", m::sinf, f64::sin, -1000.0, 1000.0, 1.0),
        ("cosf", m::cosf, f64::cos, -1000.0, 1000.0, 1.0),
        ("tanf", m::tanf, f64::tan, -1000.0, 1000.0, 1.5),
        ("tanhf", m::tanhf, f64::tanh, -20.0, 20.0, 2.0),
        ("sqrtf", m::sqrtf, f64::sqrt, 0.0, 1e30, 0.5),
        ("floorf", m::floorf, f64::floor, -1e7, 1e7, 0.0),
    ];

    for &(name, ours, theirs, lo, hi, tol) in cases {
        let mut worst = 0.0_f64;
        let mut worst_at = 0.0_f32;
        for x in sweep(f64::from(lo), f64::from(hi), 1000) {
            let x = x as f32;
            let want = theirs(f64::from(x));
            if !want.is_finite() || (want as f32).is_infinite() {
                continue;
            }
            let err = ulp_error_f32(ours(x), want);
            if err > worst {
                worst = err;
                worst_at = x;
            }
        }
        assert!(
            worst <= tol,
            "{name}: {worst} ulp at x = {worst_at} exceeds {tol}"
        );
    }
}

/// `sqrtf` is correctly rounded across the whole `f32` range.
///
/// `sqrt` is the one function here that IEEE 754 specifies exactly, and it is re-derived rather
/// than vendored (VENDORED.md), so it gets the strongest check available. The default run strides
/// the bit pattern by 1021 (odd, and coprime with the 2^23 exponent stride, so every exponent and
/// a dense spread of significands are covered); `sqrtf_is_correctly_rounded_exhaustive` walks all
/// 2^31 non-negative patterns and is `#[ignore]`d because it costs about 40 s.
#[test]
fn sqrtf_is_correctly_rounded() {
    check_sqrtf(1021);
}

/// The exhaustive form of [`sqrtf_is_correctly_rounded`]. Run with
/// `cargo test --release -p math --test scalar_accuracy -- --ignored`.
///
/// Measured on the delivery host: all 2,139,095,040 non-negative `f32` bit patterns agree with the
/// platform `sqrtf`, zero mismatches.
#[test]
#[ignore = "2^31 sweep: run with --release -- --ignored"]
fn sqrtf_is_correctly_rounded_exhaustive() {
    check_sqrtf(1);
}

fn check_sqrtf(stride: u32) {
    let mut bits = 0u32;
    while bits < 0x7f80_0000 {
        let x = f32::from_bits(bits);
        assert_eq!(
            m::sqrtf(x).to_bits(),
            x.sqrt().to_bits(),
            "sqrtf({x}) [bits {bits:#010x}] disagrees with the platform sqrt"
        );
        bits += stride;
    }

    assert_eq!(m::sqrtf(0.0).to_bits(), 0.0_f32.to_bits());
    assert_eq!(m::sqrtf(-0.0).to_bits(), (-0.0_f32).to_bits());
    assert!(m::sqrtf(-1.0).is_nan());
    assert_eq!(m::sqrtf(f32::INFINITY), f32::INFINITY);
    assert!(m::sqrtf(f32::NEG_INFINITY).is_nan());
    assert!(m::sqrtf(f32::NAN).is_nan());
}

/// `sqrt` (f64) is correctly rounded over a large deterministic sample, including subnormals and
/// perfect squares.
#[test]
fn sqrt_is_correctly_rounded() {
    let mut state = 0x243f_6a88_85a3_08d3_u64;
    let mut next = move || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };

    for _ in 0..2_000_000 {
        let bits = next() & !(1u64 << 63);
        let x = f64::from_bits(bits);
        if !x.is_finite() {
            continue;
        }
        assert_eq!(
            m::sqrt(x).to_bits(),
            x.sqrt().to_bits(),
            "sqrt({x}) [bits {bits:#018x}]"
        );
    }

    for n in 0..100_000u64 {
        let x = (n * n) as f64;
        assert_eq!(m::sqrt(x), n as f64, "sqrt of the perfect square {x}");
    }

    // Subnormals get their own pass: they are 1 in 2048 of the random draw above, and they are
    // the only inputs that take the normalising shift in the decomposition.
    for shift in 0..52 {
        for delta in [0u64, 1, 2] {
            let bits = (1u64 << shift) + delta;
            let x = f64::from_bits(bits);
            assert_eq!(
                m::sqrt(x).to_bits(),
                x.sqrt().to_bits(),
                "sqrt(subnormal {bits:#018x})"
            );
        }
    }
    for n in 1..200_000u64 {
        let x = f64::from_bits(n.wrapping_mul(0x0000_0001_1234_5677) & 0x000f_ffff_ffff_ffff);
        assert_eq!(
            m::sqrt(x).to_bits(),
            x.sqrt().to_bits(),
            "sqrt(subnormal {x:e})"
        );
    }

    for &x in &[
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::from_bits(0x000f_ffff_ffff_ffff),
        f64::MAX,
        1.0,
        4.0,
        f64::INFINITY,
    ] {
        assert_eq!(m::sqrt(x).to_bits(), x.sqrt().to_bits(), "sqrt({x})");
    }
    assert!(m::sqrt(-1.0).is_nan());
    assert!(m::sqrt(f64::NEG_INFINITY).is_nan());
    assert!(m::sqrt(f64::NAN).is_nan());
}

/// `floor`/`floorf` agree with the platform on every exponent class, including the signed-zero and
/// infinity cases the bit-twiddling form has to special-case.
#[test]
fn floor_matches_platform() {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = move || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };

    for _ in 0..2_000_000 {
        let bits = next();
        let x = f64::from_bits(bits);
        if x.is_nan() {
            continue;
        }
        assert_eq!(
            m::floor(x).to_bits(),
            x.floor().to_bits(),
            "floor({x}) [bits {bits:#018x}]"
        );
        let xf = f32::from_bits((bits >> 32) as u32);
        if !xf.is_nan() {
            assert_eq!(
                m::floorf(xf).to_bits(),
                xf.floor().to_bits(),
                "floorf({xf})"
            );
        }
    }

    for &x in &[
        0.0_f64,
        -0.0,
        -0.5,
        -1.0,
        1.0,
        f64::MAX,
        f64::MIN,
        f64::INFINITY,
    ] {
        assert_eq!(m::floor(x).to_bits(), x.floor().to_bits(), "floor({x})");
    }
    assert!(m::floor(f64::NAN).is_nan());
    assert!(m::floorf(f32::NAN).is_nan());
}

/// `scalbn` is exercised indirectly by `exp`/`pow`, but its three-step prescale only runs at the
/// extremes, so it gets a direct check through the functions that use it.
#[test]
fn extreme_scaling_paths() {
    assert_eq!(m::exp2(-1074.0), f64::from_bits(1));
    assert_eq!(m::exp2(1023.0), f64::from_bits(0x7fe0_0000_0000_0000));
    assert_eq!(m::exp2(1024.0), f64::INFINITY);
    assert_eq!(m::exp2(-1075.0), 0.0);
    assert_eq!(m::exp2f(-149.0), f32::from_bits(1));
    assert_eq!(m::exp2f(127.0), f32::from_bits(0x7f00_0000));
    assert_eq!(m::exp2f(128.0), f32::INFINITY);
    assert_eq!(m::exp2f(-150.0), 0.0);
}

/// Huge trig arguments go through `rem_pio2_large` and its 690-entry table, which is the piece
/// most at risk from the pointer-width edit in VENDORED.md.
#[test]
fn huge_argument_reduction() {
    for &x in &[
        1e10_f64,
        1e20,
        1e100,
        1e300,
        f64::MAX,
        1.0e22,
        6.077_100_506_506_192e-11,
        1.234_567_890_123_456_7e250,
    ] {
        assert!(ulp_error_f64(m::sin(x), x.sin()) <= 1.0, "sin({x})");
        assert!(ulp_error_f64(m::cos(x), x.cos()) <= 1.0, "cos({x})");
    }
    for &x in &[1e10_f32, 1e20, 1e30, f32::MAX, 1.0e22] {
        assert!(
            ulp_error_f32(m::sinf(x), f64::from(x).sin()) <= 1.0,
            "sinf({x})"
        );
        assert!(
            ulp_error_f32(m::cosf(x), f64::from(x).cos()) <= 1.0,
            "cosf({x})"
        );
    }
}
