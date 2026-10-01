#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The gain computer against an independent `f64` transcription of Giannoulis, Massberg and Reiss
//! (JAES 2012) equation 4, and the knee's continuity at both of its edges.

use effect_runtime::dynamics::{
    GainComputerCoef, MIN_SOFT_KNEE_DB, gain_computer_db, gain_delta_db, gain_from_db,
    knee_coefficients, level_db,
};
use lane::Lane;

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

// ---------------------------------------------------------------------------------------------
// Issue #994: a knee so narrow that `1 / (2 W)` overflows is a hard knee
// ---------------------------------------------------------------------------------------------

/// `2^-129`: the widest knee whose `f32` reciprocal `1 / (2 W)` overflows.
const WIDEST_OVERFLOWING_KNEE: f32 = f32::from_bits(0x0010_0000);

/// Positive knee widths whose reciprocal overflows: the smallest subnormals, the verification's
/// 2.8e-45 (`0x0000_0002`), and the two widths just under and exactly at the bound.
const OVERFLOWING_KNEES: [u32; 6] = [
    0x0000_0001,
    0x0000_0002,
    0x0000_0003,
    0x0008_0000,
    0x000F_FFFF,
    0x0010_0000,
];

/// The narrowest soft knees: the bound itself, the next widths, and a few decades above it.
const NARROWEST_SOFT_KNEES: [u32; 5] = [
    0x0010_0001,
    0x0010_0002,
    0x0020_0000,
    0x0080_0000,
    1.0e-30_f32.to_bits(),
];

fn bits(words: (f32, f32)) -> (u32, u32) {
    (words.0.to_bits(), words.1.to_bits())
}

/// The derived bound is exact: `1 / (2 W)` is finite exactly from [`MIN_SOFT_KNEE_DB`] up, and
/// positive widths below it use the hard-knee words; finite reciprocals use `(W/2, 1/(2W))`.
///
/// Exhaustive over every `f32` from `+0.0` to `f32::MIN_POSITIVE` (about 8.4 million widths, the
/// whole subnormal range around the bound), then the widths at or below zero, NaN and 24, and a
/// random million of the admissible `[0, 24]` domain drawn by bit pattern. The coefficient law
/// holds at every width, including the subnormal transition and nonpositive/NaN inputs.
///
/// Red mutations (MUTATIONS.md rows 994-R1 to 994-R3): drop the `is_finite` test, or replace it
/// with a constant bound one ulp too high (`knee_db >= f32::from_bits(0x0010_0002)`) or one ulp
/// too low (`0x0010_0000`).
#[test]
fn knee_words_follow_the_finite_reciprocal_rule() {
    assert_eq!(
        f64::from(WIDEST_OVERFLOWING_KNEE),
        2.0_f64.powi(-129),
        "0x0010_0000 is 2^-129"
    );
    assert_eq!(
        f64::from(MIN_SOFT_KNEE_DB),
        2.0_f64.powi(-129) + 2.0_f64.powi(-149),
        "the bound is 2^-129 + 2^-149"
    );
    let check = |knee: f32| {
        let reciprocal = 1.0 / (2.0 * core::hint::black_box(knee));
        let designed = knee_coefficients(knee);
        if knee > 0.0 && reciprocal.is_finite() {
            assert!(
                knee >= MIN_SOFT_KNEE_DB,
                "{knee:e} ({:#010x}) is finite below the derived bound",
                knee.to_bits()
            );
            assert_eq!(
                bits(designed),
                ((0.5 * knee).to_bits(), reciprocal.to_bits())
            );
        } else {
            assert!(
                knee.is_nan() || knee <= WIDEST_OVERFLOWING_KNEE,
                "{knee:e} ({:#010x}) overflows above the derived bound",
                knee.to_bits()
            );
            assert_eq!(bits(designed), (0, 0), "{knee:e} must be a hard knee");
        }
    };
    for word in 0..=0x0080_0000_u32 {
        check(f32::from_bits(word));
    }
    for knee in [-0.0_f32, -1.0e-45, -1.0, -24.0, f32::NAN, 24.0] {
        check(knee);
    }
    let mut generator = SplitMix64(0x994_0000_0001);
    let top = 24.0_f32.to_bits();
    for _ in 0..1_000_000 {
        check(f32::from_bits(
            (generator.next() % u64::from(top + 1)) as u32,
        ));
    }
}

/// At the verification's failing knee (2.8e-45) and at every width up to the bound, a level
/// exactly at the threshold — and every level around it — gets the hard knee's value, bit for
/// bit, and never a NaN.
///
/// Before #994 each of these widths designed `inv_two_knee = +inf`, and every level inside the
/// knee computed `(v * v = 0) * inf = NaN`. The thresholds include `0 dB` (where a full-scale
/// sample lands exactly on it) and a subnormal threshold, so that levels inside the tiny knee
/// exist at all.
///
/// Red mutations (MUTATIONS.md rows 994-R1 and 994-R3): drop the `is_finite` test in
/// `knee_coefficients`, or replace it with the constant bound one ulp too low.
#[test]
fn a_knee_whose_reciprocal_overflows_is_a_hard_knee() {
    for knee_bits in OVERFLOWING_KNEES {
        let knee = f32::from_bits(knee_bits);
        for threshold in [0.0_f32, -0.0, -1.0e-45, -18.0, -80.0] {
            for ratio in RATIOS {
                let hard = GainComputerCoef::<f32>::new(threshold, ratio, 0.0);
                let narrow = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                assert_eq!(narrow.half_knee_db.to_bits(), 0);
                assert_eq!(narrow.inv_two_knee.to_bits(), 0);
                for level in levels_around(threshold, knee) {
                    let delta = gain_delta_db::<f32>(level, &narrow);
                    assert!(
                        delta.is_finite(),
                        "W {knee:e} T {threshold} R {ratio} x {level:e}: {delta}"
                    );
                    assert_eq!(
                        delta.to_bits(),
                        gain_delta_db::<f32>(level, &hard).to_bits(),
                        "W {knee:e} T {threshold} R {ratio} x {level:e}"
                    );
                }
                let at = gain_delta_db::<f32>(threshold, &narrow);
                assert_eq!(
                    at.to_bits(),
                    0.0_f32.to_bits(),
                    "W {knee:e} T {threshold} R {ratio}: exactly at the threshold the hard knee \
                     changes nothing"
                );
            }
        }
    }
}

/// The narrowest soft knees keep their soft design and are finite everywhere around the
/// threshold, and at the threshold itself they differ from the hard knee by at most
/// `|1/R - 1| W / 8` — which, this close to the bound, is zero after rounding.
///
/// Red mutation (MUTATIONS.md row 994-R2): the constant bound one ulp too high, which designs
/// `2^-129 + 2^-149` as a hard knee although its reciprocal is finite.
#[test]
fn the_narrowest_soft_knees_are_finite_at_the_threshold() {
    for knee_bits in NARROWEST_SOFT_KNEES {
        let knee = f32::from_bits(knee_bits);
        for threshold in [0.0_f32, -0.0, -1.0e-45, -18.0, -80.0] {
            for ratio in RATIOS {
                let soft = GainComputerCoef::<f32>::new(threshold, ratio, knee);
                assert!(soft.inv_two_knee.is_finite() && soft.inv_two_knee > 0.0);
                for level in levels_around(threshold, knee) {
                    let delta = gain_delta_db::<f32>(level, &soft);
                    assert!(
                        delta.is_finite(),
                        "W {knee:e} T {threshold} R {ratio} x {level:e}: {delta}"
                    );
                }
                let at = gain_delta_db::<f32>(threshold, &soft);
                let bound = f64::from((1.0 / ratio - 1.0).abs()) * f64::from(knee) / 8.0;
                assert!(
                    f64::from(at).abs() <= bound + f64::from(f32::from_bits(1)),
                    "W {knee:e} T {threshold} R {ratio}: {at:e} at the threshold, bound {bound:e}"
                );
            }
        }
    }
}

/// A randomized sweep over admissible knees, thresholds, ratios and levels: the gain computer
/// never produces a NaN or an infinity, the scalar and both SIMD widths agree bit for bit, and the
/// result stays within `1e-4` dB of the paper's curve in `f64` (where the hard-knee substitution
/// for an overflowing width is off by at most `|1/R - 1| W / 8 < 2^-132` dB).
///
/// Knees are drawn uniformly by *bit pattern* over `[0, 24]` (so every binade, subnormals
/// included, is as likely as any other), by bit pattern over the neighbourhood of the bound, and
/// uniformly in value; thresholds likewise over `[-80, 0]`, with `0`, `-0` and the smallest
/// subnormal forced in; ratios over `[1, 20]`. Each case evaluates the threshold itself, a run of
/// levels a few subnormal steps and a few ulps either side of it, both knee edges, levels spread
/// across the knee, levels uniform over `[-800, 30]` dB (wider than any detector reaches; the
/// silent floor is about `-758.6` dB), and the silent floor itself.
///
/// Red mutations (MUTATIONS.md rows 994-R1 and 994-R3): drop the `is_finite` test in
/// `knee_coefficients`, or replace it with the constant bound one ulp too low.
#[test]
fn a_randomized_sweep_never_leaves_the_finite_curve() {
    const CASES: usize = 100_000;
    let mut generator = SplitMix64(0x994_0000_0002);
    let top_knee = 24.0_f32.to_bits();
    let bottom_threshold = (-80.0_f32).to_bits();
    let silent_floor = level_db::<f32>(0.0);
    let mut worst = 0.0_f64;
    let mut worst_case = String::new();
    let mut evaluated = 0_usize;
    for case in 0..CASES {
        let knee = match case % 4 {
            0 => f32::from_bits((generator.next() % u64::from(top_knee + 1)) as u32),
            1 => f32::from_bits((generator.next() % 0x0040_0000) as u32),
            2 => generator.unit() * 24.0,
            _ => f32::from_bits(
                [0, 0x0000_0002, 0x0010_0000, 0x0010_0001, 6.0_f32.to_bits()]
                    [(generator.next() % 5) as usize],
            ),
        };
        let threshold = match generator.next() % 4 {
            0 => [0.0_f32, -0.0, -1.0e-45, -80.0][(generator.next() % 4) as usize],
            1 => f32::from_bits(
                0x8000_0000
                    | (generator.next() % u64::from((bottom_threshold & 0x7FFF_FFFF) + 1)) as u32,
            ),
            _ => -80.0 * generator.unit(),
        };
        let ratio = match generator.next() % 4 {
            0 => [1.0_f32, 1.000_000_1, 20.0][(generator.next() % 3) as usize],
            1 => math_free_log_uniform(&mut generator, 1.0, 20.0),
            _ => 1.0 + 19.0 * generator.unit(),
        };
        let mut levels = levels_around(threshold, knee);
        levels.push(silent_floor);
        for _ in 0..4 {
            levels.push(-800.0 + 830.0 * generator.unit());
            levels.push(threshold + knee * (generator.unit() - 0.5));
        }
        let scalar = GainComputerCoef::<f32>::new(threshold, ratio, knee);
        for level in levels {
            evaluated += 1;
            let delta = gain_delta_db::<f32>(level, &scalar);
            let output = gain_computer_db::<f32>(level, &scalar);
            assert!(
                delta.is_finite() && output.is_finite(),
                "W {knee:e} ({:#010x}) T {threshold:e} R {ratio} x {level:e}: delta {delta}, \
                 output {output}",
                knee.to_bits()
            );
            lane::each_vector_lane!(|L| {
                let coef = GainComputerCoef::<L>::new(threshold, ratio, knee);
                assert_eq!(
                    lane_bits(gain_delta_db::<L>(L::splat(level), &coef)),
                    delta.to_bits(),
                    "{} W {knee:e} T {threshold:e} R {ratio} x {level:e}",
                    core::any::type_name::<L>()
                );
            });
            let expected = oracle(
                f64::from(level),
                f64::from(threshold),
                f64::from(ratio),
                f64::from(knee),
            ) - f64::from(level);
            let error = (f64::from(delta) - expected).abs();
            if error > worst {
                worst = error;
                worst_case = format!("W {knee:e} T {threshold:e} R {ratio} x {level:e}");
            }
        }
    }
    println!(
        "#994 sweep: {evaluated} levels, worst |delta - f64 eq. 4| {worst:.3e} dB at {worst_case}"
    );
    assert!(
        worst <= 1.0e-4,
        "worst deviation {worst} dB at {worst_case}"
    );
}

/// Levels at and around `threshold` that land inside a knee of width `knee`, if any can: the
/// threshold itself, a run of subnormal steps either side of it, a run of ulps either side, the
/// two knee edges and their neighbours.
fn levels_around(threshold: f32, knee: f32) -> Vec<f32> {
    let mut levels = vec![threshold];
    for step in 1..=4_u32 {
        let tiny = f32::from_bits(step);
        levels.push(threshold + tiny);
        levels.push(threshold - tiny);
        levels.push(next_up(threshold, step));
        levels.push(next_down(threshold, step));
    }
    let half = 0.5 * knee;
    for edge in [threshold - half, threshold + half] {
        levels.push(edge);
        levels.push(next_up(edge, 1));
        levels.push(next_down(edge, 1));
    }
    levels
}

fn next_up(x: f32, steps: u32) -> f32 {
    let mut value = x;
    for _ in 0..steps {
        value = value.next_up();
    }
    value
}

fn next_down(x: f32, steps: u32) -> f32 {
    let mut value = x;
    for _ in 0..steps {
        value = value.next_down();
    }
    value
}

/// Every lane's bits, asserted equal, returned once.
fn lane_bits<L: Lane>(value: L) -> u32 {
    let mut words = [0_u32; 8];
    value.store_bits(&mut words[..L::WIDTH]);
    for word in &words[1..L::WIDTH] {
        assert_eq!(*word, words[0], "lanes of a splatted input disagree");
    }
    words[0]
}

/// `lo * (hi / lo)^u` in `f64`, rounded once: a log-uniform ratio without the lane `log2`.
fn math_free_log_uniform(generator: &mut SplitMix64, lo: f64, hi: f64) -> f32 {
    (lo * (hi / lo).powf(f64::from(generator.unit()))) as f32
}

/// SplitMix64 (Steele, Lea and Flood 2014), for a reproducible sweep with no dependency.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)`, 24 bits.
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32
    }
}
