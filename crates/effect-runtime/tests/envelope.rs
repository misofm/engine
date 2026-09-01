//! Followers, coefficients and the gate hysteresis.

use effect_runtime::envelope::{
    ArCoef, HysteresisCoef, HysteresisState, ar_one_pole_step, attack_release_coefficient,
    hysteresis_step, peak_follow, retention_coefficient, rms_follow,
};
use lane::Lane;
use lane::softfma::unfused_multiply_add_via_f64;

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

/// `peak_follow` is exactly the unfused form: identical to an `f64` restatement of the same
/// operation order, with each `f32` rounding taken separately.
///
/// Before issue #163 phase 2 this test asserted the *fused* form, computing the expectation as one
/// `f64` expression narrowed once. The contract is now two roundings, so the expectation is built
/// the same way the kernel builds it: round the product to `f32`, then round the sum. Both steps
/// go through `f64` so the expectation does not simply re-execute the `f32` expression under test
/// -- the product is exact in `f64` (24 + 24 <= 53) and the sum double-rounds innocuously
/// (53 >= 2p + 2), which is what makes this a bit-for-bit assertion rather than a tolerance.
///
/// Red mutation: restore the fused expectation (one `f64` expression, one narrowing) and the
/// equality fails on the first input where the two contracts disagree.
#[test]
fn peak_follow_matches_the_unfused_f64_restatement() {
    let mut state = 0.0f64;
    let mut y = 0.0f32;
    for step in 0..20_000u32 {
        let x = ((step as f32) * 0.000_37).sin();
        let c = 0.9995f32;
        let x_abs = x.abs();
        let d = y - x_abs;
        let exact = unfused_multiply_add_via_f64(c, d, x_abs);
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

/// `rms_follow` is likewise exact against the unfused `f64` restatement.
#[test]
fn rms_follow_matches_the_unfused_f64_restatement() {
    let c = attack_release_coefficient(10.0, 48_000);
    let mut y = 0.0f32;
    for step in 0..20_000u32 {
        let x = ((step as f32) * 0.001_9).cos();
        let x2 = x * x;
        let d = x2 - y;
        let expected = unfused_multiply_add_via_f64(c, d, y);
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
    type L = lane::Simd4;
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

/// `ArCoef` precomputes an **exact** complement for every coefficient a follower can be given.
///
/// Red mutation: `one_minus_attack: one.sub(attack).mul(L::splat(1.0))` is still exact, so the
/// mutation that matters is arithmetic in the step instead of the constructor — see
/// `ar_one_pole_step_is_the_two_product_form`.
#[test]
fn ar_coefficient_complements_are_exact() {
    for rate in [44_100u32, 48_000, 88_200, 96_000] {
        for time_ms in [0.5f32, 10.0, 20.0, 100.0] {
            let c = retention_coefficient(time_ms, rate);
            assert!(c >= 0.5, "{time_ms} ms at {rate} Hz retains {c}");
            let coefficients = ArCoef::<f32>::splat(c, c);
            assert_eq!(
                coefficients.one_minus_attack.to_bits(),
                (1.0f32 - c).to_bits()
            );
            // Sterbenz: `1 - c` for c in [0.5, 1] is representable, so the round trip is exact.
            assert_eq!(
                (1.0f32 - coefficients.one_minus_attack).to_bits(),
                c.to_bits()
            );
        }
    }
}

/// The direction switch is strict: a detector exactly at the envelope releases.
///
/// Red mutation: `u.ge(e)` instead of `u.gt(e)` in `ar_one_pole_step`.
#[test]
fn ar_one_pole_step_switches_strictly_on_rising() {
    let coefficients = ArCoef::<f32>::splat(0.75, 0.99);
    // Rising: the attack coefficient is the one that moves the envelope.
    let rising = ar_one_pole_step(0.25f32, 1.0, &coefficients);
    assert_eq!(rising.to_bits(), (0.75f32 * 0.25 + 0.25f32 * 1.0).to_bits());
    // Falling: the release coefficient.
    let falling = ar_one_pole_step(1.0f32, 0.25, &coefficients);
    assert_eq!(
        falling.to_bits(),
        (0.99f32 * 1.0 + 0.010000005f32 * 0.25).to_bits()
    );
    // Exactly equal: releases. A convex combination of two equal values is that value in exact
    // arithmetic, so the equal case only distinguishes the two coefficients where the two rounded
    // products sum differently -- this witness is such a point at the 0.5 ms / 20 ms 44.1 kHz pair.
    let fast = ArCoef::<f32>::splat(f32::from_bits(0x3f74_a63c), f32::from_bits(0x3f7f_b5bd));
    let witness = f32::from_bits(0x3c1b_4ffb);
    let equal = ar_one_pole_step(witness, witness, &fast);
    let released = fast.release * witness + fast.one_minus_release * witness;
    let attacked = fast.attack * witness + fast.one_minus_attack * witness;
    assert_ne!(
        released.to_bits(),
        attacked.to_bits(),
        "the witness must separate the two coefficients"
    );
    assert_eq!(equal.to_bits(), released.to_bits());
}

/// The two-product form does not stall where the one-rounding form does.
///
/// At the 100 ms / 96 kHz slow-follower coefficient `k = 1 - c` is about `1.04e-4`, so the
/// one-rounding release `e + k * (u - e)` has a deadband: the product underflows the addition and
/// the envelope freezes at `e`. The two-product form `c * e + k * u` rounds the two products
/// separately, and their sum leaves `e`. A stalled slow envelope is a permanent contrast offset,
/// which is why the transient shaper's follower is this form and not `peak_follow`'s.
///
/// Red mutation: `ar_one_pole_step` rewritten as the single-rounding fused form — the
/// witness pair below then returns `e` unchanged and this test goes red.
#[test]
fn ar_one_pole_step_is_the_two_product_form() {
    let c = retention_coefficient(100.0, 96_000);
    let coefficients = ArCoef::<f32>::splat(c, c);
    let e = 0.7f32;
    let u = e - f32::from_bits(0x3870_0000);
    let k = 1.0f32 - c;
    // The one-rounding release. `Lane::fma` is unfused since #163 phase 2, so the fused form has
    // to be written out in `f64` -- one exact product, one sum, one narrowing -- to keep the
    // contrast this witness is about. (The stall is a property of the single rounding, not of the
    // spelling: it is why the transient shaper's follower is the two-product form.)
    let stalled = (f64::from(k) * f64::from(u - e) + f64::from(e)) as f32;
    assert_eq!(
        stalled.to_bits(),
        e.to_bits(),
        "the one-rounding form must stall on this witness"
    );
    let moved = ar_one_pole_step(e, u, &coefficients);
    assert_ne!(
        moved.to_bits(),
        e.to_bits(),
        "the two-product form must move"
    );
    assert!(moved < e && moved > u);
}

/// A decaying envelope reaches exactly `+0.0`, not a subnormal (D7).
///
/// Red mutation: drop `lane::flush` from `ar_one_pole_step`.
#[test]
fn ar_one_pole_step_flushes_the_recurrence() {
    let coefficients = ArCoef::<f32>::splat(0.5, 0.5);
    let mut e = 1.0e-18f32;
    for _ in 0..8 {
        e = ar_one_pole_step(e, 0.0, &coefficients);
    }
    assert_eq!(e.to_bits(), 0.0f32.to_bits(), "envelope must flush to +0.0");
}

/// One body, three widths, identical bits.
///
/// A follower is a recurrence, so lane identity is stated the way a bank actually runs it: each
/// lane is an independent track with its own envelope, stepped once, and the packed result must
/// equal the scalar results lane for lane.
///
/// Red mutation: any width-dependent edit to `ar_one_pole_step`.
#[test]
fn ar_one_pole_step_is_width_independent() {
    let envelopes = [0.0f32, 0.25, 0.5, 0.7, 1.0, 1.0e-19, 0.125, 0.9];
    let detectors = [0.1f32, 0.9, 0.2, 0.8, 0.3, 0.0, 0.4, 0.6];
    let coefficients = ArCoef::<f32>::splat(0.9556615, 0.998_866_9);
    let scalar: Vec<u32> = envelopes
        .iter()
        .zip(detectors.iter())
        .map(|(e, u)| ar_one_pole_step(*e, *u, &coefficients).to_bits())
        .collect();

    fn packed<L: Lane>(envelopes: &[f32; 8], detectors: &[f32; 8]) -> Vec<u32> {
        let coefficients = ArCoef::<L>::splat(0.9556615, 0.998_866_9);
        let mut words = [0u32; 8];
        let mut result = Vec::new();
        for (e, u) in envelopes.chunks(L::WIDTH).zip(detectors.chunks(L::WIDTH)) {
            ar_one_pole_step(L::load(e), L::load(u), &coefficients).store_bits(&mut words);
            result.extend_from_slice(&words[..L::WIDTH]);
        }
        result
    }

    assert_eq!(packed::<f32>(&envelopes, &detectors), scalar);
    assert_eq!(packed::<lane::Simd4>(&envelopes, &detectors), scalar);
    assert_eq!(packed::<lane::Simd8>(&envelopes, &detectors), scalar);
}
