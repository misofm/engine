//! E1 — the static curve is Giannoulis, Massberg and Reiss equation 4.
//!
//! The oracle is written here, from the paper, in `f64`, with three `if` arms and `20 * log10`
//! from the platform libm. It shares no code with the implementation: the implementation is
//! `effect_runtime::dynamics::gain_delta_db` on the lane `log2` of `math`,
//! branchless, in `f32`.
//!
//! # The bound, derived
//!
//! `log2_lane` is accurate to 2 ulp (gate M1). At the domain edge `|x| <= 160` dB the level is
//! about `26.6` octaves, so 2 ulp of the `log2` result is about `2 * 2^-23 * 32 = 7.6e-6`
//! octaves, which is `4.6e-5` dB after the `6.0206` scale. The curve then applies at most four
//! `f32` operations to a quantity bounded by 184 dB, contributing at most `4 * 0.5 ulp(184)`,
//! about `2.2e-5` dB. Total `6.8e-5`; the gate is `1e-4` dB, and the measured worst case over the
//! grid is reported by the test itself.

use effect_runtime::dynamics::{GainComputerCoef, gain_delta_db};

/// Equation 4 in `f64`, transcribed from the paper, as the gain change it applies.
fn oracle(level_db: f64, threshold: f64, ratio: f64, knee: f64) -> f64 {
    let over = level_db - threshold;
    if 2.0 * over < -knee {
        0.0
    } else if 2.0 * over > knee {
        (1.0 / ratio - 1.0) * over
    } else {
        let v = over + knee / 2.0;
        (1.0 / ratio - 1.0) * v * v / (2.0 * knee)
    }
}

/// The implementation agrees with the paper to better than `1e-4` dB over the frozen grid.
///
/// Red mutation (MUTATIONS.md row 20): `inv_two_knee` designed as `1 / knee` instead of
/// `1 / (2 * knee)` in `effect-runtime` — RED. That crate is not modified by this
/// branch; the mutation was applied temporarily to prove this gate would catch a change to the
/// shared curve.
#[test]
fn eq4_static_curve_matches_f64_within_1e4_db() {
    let mut worst = 0.0_f64;
    let mut worst_at = (0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32);
    for threshold in [-80.0_f32, -18.0, 0.0] {
        for ratio in [1.0_f32, 2.0, 4.0, 20.0] {
            for knee in [0.0_f32, 6.0, 24.0] {
                let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                let mut level = -160.0_f32;
                while level <= 24.0 {
                    let actual = f64::from(gain_delta_db::<f32>(level, &coefficients));
                    // The knee arm below `2(x - T) = -W` is exactly the identity in both, but the
                    // paper's strict `<` and the implementation's `<=` differ at the edge only by
                    // `+0.0` versus `-0.0`, which this comparison cannot see.
                    let expected = oracle(
                        f64::from(level),
                        f64::from(threshold),
                        f64::from(ratio),
                        f64::from(knee),
                    );
                    let deviation = (actual - expected).abs();
                    if deviation > worst {
                        worst = deviation;
                        worst_at = (threshold, ratio, knee, level);
                    }
                    level += 0.25;
                }
            }
        }
    }
    println!("E1 worst deviation {worst:.3e} dB at {worst_at:?}");
    assert!(
        worst <= 1.0e-4,
        "worst deviation {worst:e} dB at (threshold, ratio, knee, level) = {worst_at:?}"
    );
}

/// The knee joins the two straight arms with no step at either edge, and is the exact identity at
/// the lower one.
///
/// Covered by the same row 20 mutation.
#[test]
fn the_knee_is_continuous_at_both_edges() {
    for threshold in [-40.0_f32, -12.0, 0.0] {
        for ratio in [2.0_f32, 8.0] {
            for knee in [6.0_f32, 24.0] {
                let coefficients = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                let lower = threshold - knee / 2.0;
                let upper = threshold + knee / 2.0;
                assert_eq!(
                    gain_delta_db::<f32>(lower, &coefficients).to_bits(),
                    0.0_f32.to_bits(),
                    "the lower knee edge is the exact identity"
                );
                // Continuity, not flatness: the curve has a slope of at most `|1/R - 1|` at
                // either edge, so the admissible change across `2 * epsilon` is that slope times
                // the interval, plus the `1e-4` dB the conversions may contribute. A genuine
                // discontinuity — which is what a mis-halved knee or a swapped select produces —
                // is a jump of whole dB and fails this by three orders of magnitude.
                let epsilon = 0.001_f32;
                let slope = 1.0 - 1.0 / ratio;
                let admissible = f64::from(slope) * 2.0 * f64::from(epsilon) + 1.0e-4;
                for edge in [lower, upper] {
                    let step = f64::from(
                        gain_delta_db::<f32>(edge + epsilon, &coefficients)
                            - gain_delta_db::<f32>(edge - epsilon, &coefficients),
                    )
                    .abs();
                    assert!(
                        step <= admissible,
                        "step {step} dB across {edge} (T {threshold}, R {ratio}, W {knee}), admissible {admissible}"
                    );
                }
            }
        }
    }
}

/// A hard knee is exactly the identity at and below the threshold, and the line above it.
#[test]
fn a_hard_knee_is_exact_at_the_threshold() {
    let coefficients = GainComputerCoef::<f32>::new(-18.0, 4.0, 0.0);
    assert_eq!(
        gain_delta_db::<f32>(-18.0, &coefficients).to_bits(),
        0.0_f32.to_bits()
    );
    assert_eq!(
        gain_delta_db::<f32>(-24.0, &coefficients).to_bits(),
        0.0_f32.to_bits()
    );
    let above = gain_delta_db::<f32>(-6.0, &coefficients);
    assert!((f64::from(above) - oracle(-6.0, -18.0, 4.0, 0.0)).abs() <= 1.0e-4);
}
