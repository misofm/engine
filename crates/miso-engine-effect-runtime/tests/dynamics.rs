//! The gain computer against an independent `f64` transcription of Giannoulis, Massberg and Reiss
//! (JAES 2012) equation 4, and the knee's continuity at both of its edges.

use miso_engine_effect_runtime::dynamics::{
    GainComputerCoef, gain_computer_db, gain_delta_db, gain_from_db, level_db,
};

/// Equation 4, transcribed from the paper in `f64`. Independent of the implementation: it is
/// written in the paper's own variables and branches, with no shared helper.
fn oracle(x: f64, threshold: f64, ratio: f64, knee: f64) -> f64 {
    if knee <= 0.0 {
        return if x <= threshold {
            x
        } else {
            threshold + (x - threshold) / ratio
        };
    }
    let d = 2.0 * (x - threshold);
    if d < -knee {
        x
    } else if d > knee {
        threshold + (x - threshold) / ratio
    } else {
        let v = x - threshold + 0.5 * knee;
        x + (1.0 / ratio - 1.0) * v * v / (2.0 * knee)
    }
}

/// The frozen grid: every launch-relevant threshold, ratio and knee, swept over the whole dB range
/// a detector can produce.
const THRESHOLDS: [f32; 5] = [0.0, -6.0, -18.0, -40.0, -80.0];
const RATIOS: [f32; 6] = [1.0, 1.5, 2.0, 4.0, 10.0, 20.0];
const KNEES: [f32; 5] = [0.0, 1.0, 6.0, 12.0, 24.0];

/// The lane curve matches the paper to better than 0.01 dB across the frozen grid.
///
/// Red mutation: `half_knee_db = knee_db` instead of `0.5 * knee_db` (the knee-width halving).
/// A 6 dB knee then spans 12 dB and the error at the old knee edges is several tenths of a dB.
#[test]
fn the_curve_matches_the_paper() {
    let mut worst = 0.0f64;
    let mut worst_case = String::new();
    for threshold in THRESHOLDS {
        for ratio in RATIOS {
            for knee in KNEES {
                let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                let mut level = -160.0f32;
                while level <= 24.0 {
                    let actual = f64::from(gain_computer_db::<f32>(level, &coefficients));
                    let expected = oracle(
                        f64::from(level),
                        f64::from(threshold),
                        f64::from(ratio),
                        f64::from(knee),
                    );
                    let error = (actual - expected).abs();
                    if error > worst {
                        worst = error;
                        worst_case = format!(
                            "T {threshold} R {ratio} W {knee} x {level}: {actual} vs {expected}"
                        );
                    }
                    level += 0.25;
                }
            }
        }
    }
    assert!(
        worst <= 0.01,
        "worst deviation {worst} dB exceeds 0.01 dB at {worst_case}"
    );
}

/// The knee joins the two straight arms without a step, at both edges.
///
/// At `x = T - W/2` the curve must be `x` itself, exactly; at `x = T + W/2` it must agree with the
/// above-threshold line to within a rounding of the arithmetic.
#[test]
fn the_knee_is_continuous_at_both_edges() {
    for threshold in THRESHOLDS {
        for ratio in RATIOS {
            for knee in KNEES {
                if knee == 0.0 {
                    continue;
                }
                let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                let lower = threshold - 0.5 * knee;
                assert_eq!(
                    gain_delta_db::<f32>(lower, &coefficients).to_bits(),
                    0.0f32.to_bits(),
                    "T {threshold} R {ratio} W {knee}: the lower knee edge must be exactly the input"
                );
                let upper = threshold + 0.5 * knee;
                let from_knee = f64::from(gain_computer_db::<f32>(upper, &coefficients));
                let from_line =
                    f64::from(threshold) + f64::from(upper - threshold) / f64::from(ratio);
                assert!(
                    (from_knee - from_line).abs() <= 1e-4,
                    "T {threshold} R {ratio} W {knee}: upper edge {from_knee} vs line {from_line}"
                );
            }
        }
    }
}

/// A hard knee is exact at the threshold and produces no `0 * inf`.
///
/// Red mutation: leave `inv_two_knee` as `1 / (2 * 0)` for a zero knee — the value at exactly the
/// threshold becomes NaN.
#[test]
fn a_hard_knee_is_exact_at_the_threshold() {
    for threshold in THRESHOLDS {
        for ratio in RATIOS {
            let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, 0.0);
            let at = gain_computer_db::<f32>(threshold, &coefficients);
            assert!(at.is_finite(), "T {threshold} R {ratio}: {at}");
            assert_eq!(at.to_bits(), threshold.to_bits());
            assert_eq!(
                gain_delta_db::<f32>(threshold, &coefficients).to_bits(),
                0.0f32.to_bits()
            );
        }
    }
}

/// A knee width at or below zero is a hard knee, not a knee turned inside out.
///
/// Red mutation: drop the `knee_db > 0.0` guard in `GainComputerCoef::new` and design the
/// coefficients unconditionally. A negative width then gives a negative `half_knee_db`, the
/// `under` arm swallows the first `|W|/2` dB above the threshold, and a signal that should be
/// compressed is passed through instead.
#[test]
fn a_non_positive_knee_is_a_hard_knee() {
    for threshold in THRESHOLDS {
        for ratio in RATIOS {
            let hard = GainComputerCoef::<f32>::new(threshold, ratio, 0.0);
            for knee in [-0.0f32, -1.0, -6.0, -24.0] {
                let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                for offset in [-10.0f32, -2.0, -0.5, 0.0, 0.5, 2.0, 10.0] {
                    let level = threshold + offset;
                    assert_eq!(
                        gain_delta_db::<f32>(level, &coefficients).to_bits(),
                        gain_delta_db::<f32>(level, &hard).to_bits(),
                        "T {threshold} R {ratio} W {knee} at {level} dB"
                    );
                }
            }
        }
    }
}

/// Below the knee the curve is the identity, bit for bit — a quiet signal is not "compressed by
/// zero dB", it is untouched.
#[test]
fn below_the_knee_is_the_exact_identity() {
    let coefficients = GainComputerCoef::<f32>::new(-18.0, 4.0, 6.0);
    for level in [-160.0f32, -100.0, -50.0, -21.000_1, -30.0] {
        assert_eq!(
            gain_delta_db::<f32>(level, &coefficients).to_bits(),
            0.0f32.to_bits(),
            "{level} dB"
        );
        assert_eq!(
            gain_computer_db::<f32>(level, &coefficients).to_bits(),
            level.to_bits()
        );
    }
}

/// A ratio of 1 is a no-op at every level, and a ratio below 1 expands upward.
#[test]
fn the_ratio_sets_the_slope() {
    let unity = GainComputerCoef::<f32>::new(-18.0, 1.0, 6.0);
    for level in [-160.0f32, -18.0, 0.0, 24.0] {
        assert_eq!(
            gain_delta_db::<f32>(level, &unity).to_bits(),
            0.0f32.to_bits(),
            "ratio 1 must be an identity at {level} dB"
        );
    }
    let expander = GainComputerCoef::<f32>::new(-40.0, 0.5, 0.0);
    assert!(
        gain_delta_db::<f32>(-20.0, &expander) > 0.0,
        "a ratio below 1 expands upward"
    );
}

/// The two dB conversions are exact at unity and are inverses to within a rounding.
#[test]
fn the_db_conversions_agree() {
    assert_eq!(gain_from_db::<f32>(0.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(level_db::<f32>(1.0).to_bits(), 0.0f32.to_bits());
    for db in [-60.0f32, -24.0, -6.0, -1.0, 0.0, 1.0, 6.0, 24.0] {
        let gain = gain_from_db::<f32>(db);
        let expected = 10.0f64.powf(f64::from(db) / 20.0);
        assert!(
            (f64::from(gain) - expected).abs() <= expected * 1e-6,
            "{db} dB: {gain} vs {expected}"
        );
        let back = level_db::<f32>(gain);
        assert!((back - db).abs() <= 1e-3, "{db} dB round-tripped to {back}");
    }
}

/// Silence gives a finite floor, not `-inf`, so a detector at rest cannot poison the curve.
#[test]
fn silence_gives_a_finite_floor() {
    let floor = level_db::<f32>(0.0);
    assert!(floor.is_finite(), "{floor}");
    assert!(floor < -700.0, "{floor} is not a floor");
    let coefficients = GainComputerCoef::<f32>::new(-18.0, 4.0, 6.0);
    assert!(gain_computer_db::<f32>(floor, &coefficients).is_finite());
}

/// A NaN level stays a NaN — it is never quietly turned into a gain.
///
/// Both ordered compares are false, so a NaN takes the knee arm and propagates. The block boundary
/// check of `bank` is what turns that into a reported, zeroed block.
#[test]
fn a_nan_level_stays_a_nan() {
    let coefficients = GainComputerCoef::<f32>::new(-18.0, 4.0, 6.0);
    assert!(gain_delta_db::<f32>(f32::NAN, &coefficients).is_nan());
    assert!(gain_computer_db::<f32>(f32::NAN, &coefficients).is_nan());
}
