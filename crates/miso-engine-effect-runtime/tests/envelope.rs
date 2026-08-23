//! Followers, coefficients and the gate hysteresis.

use miso_engine_effect_runtime::envelope::{
    HysteresisCoef, HysteresisState, attack_release_coefficient, hysteresis_step, peak_follow,
    retention_coefficient, rms_follow,
};
use miso_engine_lane::Lane;

/// `exp(-1 / (tau * fs))` in `f64`, the independent oracle for the coefficient design.
fn oracle_retention(time_ms: f64, sample_rate: f64) -> f64 {
    (-1.0 / (time_ms * 0.001 * sample_rate)).exp()
}

/// The sign of the exponent is the content of the coefficient.
///
/// Red mutation: `expf(1.0 / tau_samples)` instead of `expf(-1.0 / tau_samples)`, or
/// `attack_release_coefficient` returning `retention - 1.0`. Either makes a coefficient leave
/// `[0, 1]` — and a follower built on it diverge — which is what these bounds catch.
#[test]
fn coefficients_are_in_range_and_correctly_signed() {
    for rate in [44_100u32, 48_000, 88_200, 96_000] {
        for time_ms in [0.1f32, 1.0, 5.0, 20.0, 200.0, 1000.0, 5000.0] {
            let retention = retention_coefficient(time_ms, rate);
            let rate_coefficient = attack_release_coefficient(time_ms, rate);
            assert!(
                (0.0..1.0).contains(&retention),
                "{time_ms} ms at {rate} Hz: retention {retention} left [0, 1)"
            );
            assert!(
                (0.0..=1.0).contains(&rate_coefficient),
                "{time_ms} ms at {rate} Hz: rate {rate_coefficient} left [0, 1]"
            );
            assert_eq!(
                rate_coefficient.to_bits(),
                (1.0f32 - retention).to_bits(),
                "the two must be exact complements"
            );
            let expected = oracle_retention(f64::from(time_ms), f64::from(rate));
            assert!(
                (f64::from(retention) - expected).abs() <= 1e-6,
                "{time_ms} ms at {rate} Hz: {retention} vs oracle {expected}"
            );
            // A longer time constant retains more: monotone in the time.
            assert!(retention > 0.0);
        }
        for pair in [(1.0f32, 5.0f32), (5.0, 20.0), (20.0, 200.0)] {
            assert!(
                retention_coefficient(pair.0, rate) < retention_coefficient(pair.1, rate),
                "retention must increase with the time constant"
            );
        }
    }
}

/// The degenerate arguments give an instantaneous follower rather than a NaN or an infinity.
#[test]
fn degenerate_coefficient_arguments_are_instantaneous() {
    for (time_ms, rate) in [
        (0.0f32, 48_000u32),
        (-1.0, 48_000),
        (f32::NAN, 48_000),
        (10.0, 0),
    ] {
        assert_eq!(
            retention_coefficient(time_ms, rate).to_bits(),
            0.0f32.to_bits()
        );
        assert_eq!(
            attack_release_coefficient(time_ms, rate).to_bits(),
            1.0f32.to_bits()
        );
    }
}

/// `peak_follow` is the exact single-rounding form: identical to an `f64` evaluation of the same
/// operation order, rounded once.
///
/// `f32` fma is exactly representable in `f64` (24 + 24 bits of product against a 53-bit
/// significand), so this is a bit-for-bit assertion, not a tolerance.
///
/// Red mutation: write the release as `c * y + (1 - c) * x` — two roundings — and the equality
/// fails on the first input where the two disagree.
#[test]
fn peak_follow_rounds_once() {
    let mut state = 0.0f64;
    let mut y = 0.0f32;
    for step in 0..20_000u32 {
        let x = ((step as f32) * 0.000_37).sin();
        let c = 0.9995f32;
        let x_abs = x.abs();
        let d = y - x_abs;
        let exact = (f64::from(c) * f64::from(d) + f64::from(x_abs)) as f32;
        let expected = if x_abs > exact { x_abs } else { exact };
        y = peak_follow::<f32>(x_abs, y, c);
        assert_eq!(
            y.to_bits(),
            expected.to_bits(),
            "step {step}: {y} vs {expected}"
        );
        state += f64::from(y);
    }
    assert!(state.is_finite());
}

/// `rms_follow` is likewise exact against a single-rounding `f64` evaluation.
#[test]
fn rms_follow_rounds_once() {
    let c = attack_release_coefficient(10.0, 48_000);
    let mut y = 0.0f32;
    for step in 0..20_000u32 {
        let x = ((step as f32) * 0.001_9).cos();
        let x2 = x * x;
        let d = x2 - y;
        let expected = (f64::from(c) * f64::from(d) + f64::from(y)) as f32;
        y = rms_follow::<f32>(x2, y, c);
        assert_eq!(y.to_bits(), expected.to_bits(), "step {step}");
    }
}

/// The extremes of the coefficient are exact: `c = 0` holds the input, `c = 1` freezes the state.
#[test]
fn follower_extremes_are_exact() {
    for x in [0.0f32, -0.0, 1.0, 0.25, 1e-20, 1e20] {
        let x_abs = x.abs();
        assert_eq!(
            peak_follow::<f32>(x_abs, 7.0, 0.0).to_bits(),
            x_abs.max(x_abs).to_bits(),
            "c = 0 must return the input for {x}"
        );
        assert_eq!(
            rms_follow::<f32>(x_abs, 3.5, 0.0).to_bits(),
            3.5f32.to_bits(),
            "c = 0 must freeze the mean-square follower"
        );
        assert_eq!(
            rms_follow::<f32>(x_abs, 0.0, 1.0).to_bits(),
            x_abs.to_bits(),
            "c = 1 from rest must take the input"
        );
    }
}

/// `peak_follow`'s `max` follows D8: `max(a, b)` returns `b` on equal and on unordered lanes.
#[test]
fn peak_follow_max_follows_d8() {
    // A NaN in the decayed term is returned, not discarded.
    let out = peak_follow::<f32>(1.0, f32::NAN, 0.5);
    assert!(out.is_nan(), "a NaN state must propagate, got {out}");
    // Signed zeros: max(+0, -0) is the second operand under D8.
    assert_eq!(
        peak_follow::<f32>(0.0, 0.0, 1.0).to_bits(),
        0.0f32.to_bits(),
        "both zero"
    );
    // fma(1, -0 - 0, 0) = +0.0; max(+0.0, +0.0) = +0.0 (second operand).
    assert_eq!(
        peak_follow::<f32>(0.0, -0.0, 1.0).to_bits(),
        0.0f32.to_bits()
    );
}

/// The gate opens above the open threshold, holds, and only closes once the countdown expires and
/// the level is below the close threshold.
#[test]
fn hysteresis_opens_holds_and_closes() {
    let coefficients = HysteresisCoef::<f32> {
        open_db: -30.0,
        close_db: -36.0,
        hold_samples: 4.0,
    };
    let mut state = HysteresisState::<f32>::default();

    assert_eq!(
        hysteresis_step(-50.0, &coefficients, &mut state).to_bits(),
        0.0f32.to_bits(),
        "a quiet lane starts closed"
    );
    assert_eq!(
        hysteresis_step(-20.0, &coefficients, &mut state).to_bits(),
        1.0f32.to_bits(),
        "a loud lane opens"
    );
    assert_eq!(
        state.hold.to_bits(),
        4.0f32.to_bits(),
        "the hold is reloaded"
    );

    // Between the thresholds: neither opens nor closes, the hold counts down.
    for expected_hold in [3.0f32, 2.0, 1.0, 0.0] {
        assert_eq!(
            hysteresis_step(-33.0, &coefficients, &mut state).to_bits(),
            1.0f32.to_bits(),
            "the hysteresis band must hold the state"
        );
        assert_eq!(state.hold.to_bits(), expected_hold.to_bits());
    }
    // Still in the band with the hold expired: still open, because the level is above close_db.
    assert_eq!(
        hysteresis_step(-33.0, &coefficients, &mut state).to_bits(),
        1.0f32.to_bits()
    );

    // Below the close threshold, but the hold was reloaded — so it takes the countdown first.
    assert_eq!(
        hysteresis_step(-40.0, &coefficients, &mut state).to_bits(),
        0.0f32.to_bits(),
        "below close with an expired hold closes"
    );
}

/// The hold delays closing by exactly `hold_samples` samples.
///
/// Red mutation: drop the `expired` term from the close mask, and the gate closes on the first
/// quiet sample instead of after the hold.
#[test]
fn the_hold_delays_closing() {
    let coefficients = HysteresisCoef::<f32> {
        open_db: -30.0,
        close_db: -36.0,
        hold_samples: 3.0,
    };
    let mut state = HysteresisState::<f32>::default();
    assert_eq!(
        hysteresis_step(-10.0, &coefficients, &mut state).to_bits(),
        1.0f32.to_bits()
    );
    let mut open_samples = 0;
    for _ in 0..10 {
        if hysteresis_step(-60.0, &coefficients, &mut state) == 1.0 {
            open_samples += 1;
        }
    }
    assert_eq!(open_samples, 3, "the gate must stay open for the hold");
}

/// The countdown clamps at zero rather than running negative.
#[test]
fn the_countdown_clamps_at_zero() {
    let coefficients = HysteresisCoef::<f32> {
        open_db: -30.0,
        close_db: -36.0,
        hold_samples: 1.0,
    };
    let mut state = HysteresisState::<f32>::default();
    for _ in 0..1000 {
        hysteresis_step(-90.0, &coefficients, &mut state);
    }
    assert_eq!(state.hold.to_bits(), 0.0f32.to_bits());
    assert_eq!(state.open.to_bits(), 0.0f32.to_bits());
}

/// Lanes are independent: one open lane does not open its neighbours.
#[test]
fn hysteresis_lanes_are_independent() {
    type L = miso_engine_lane::Simd4;
    let coefficients = HysteresisCoef::<L> {
        open_db: L::splat(-30.0),
        close_db: L::splat(-36.0),
        hold_samples: L::splat(0.0),
    };
    let mut state = HysteresisState::<L>::default();
    let level = L::load(&[-10.0, -50.0, -33.0, -10.0]);
    let out = hysteresis_step(level, &coefficients, &mut state);
    let mut bits = [0u32; 4];
    out.store_bits(&mut bits);
    assert_eq!(
        bits,
        [
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0.0f32.to_bits(),
            1.0f32.to_bits()
        ]
    );
}
