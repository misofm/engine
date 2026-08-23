//! Comparison against the independent `f64` oracle in `miso-engine-dsp-reference`.
//!
//! The oracle derives its four coefficients from the frozen time constants with `f64` `exp`, runs
//! the follower in `f64`, takes the contrast as two `f64` `log10`s and the gain as `f64` `powf`.
//! Nothing in `miso-engine-math`, `miso-engine-lane` or `miso-engine-effect-runtime` is reachable
//! from it, which is what makes it independent of the chain it is checking.
//!
//! # Why these tolerances did not move
//!
//! Replacing `20 log10(fast) - 20 log10(slow)` and `10^(shape / 20)` with `log2_lane` of the ratio
//! and `exp2_lane` is a class-B change (master plan §1.8): the bits move, so the bound has to be
//! derived rather than measured. With gate M1's `<= 2 ulp` on both lane polynomials:
//!
//! | step | bound |
//! |---|---|
//! | `q = max(f, FLOOR) / max(s, FLOOR)` | one rounding, relative `2^-24` |
//! | `l = log2_lane(q)`, surviving `|l| <= 24 / 6.0206 = 3.99` | `2 * 2^-22 + 2^-24 / ln 2 = 5.6e-7` |
//! | `c = l * DB_PER_OCTAVE`, clamp is 1-Lipschitz | `5.8e-6` dB |
//! | `shape = A * max(c, 0) + S * max(-c, 0)`, three roundings | `8.7e-6` dB |
//! | `a = shape * OCTAVES_PER_DB`, `|a| <= 2.99` | `1.75e-6` |
//! | `g = exp2_lane(a)` | `1.45e-6` relative = `1.26e-5` dB |
//! | `y = fma(mix, x * g - x, x)` | three roundings of `|x| * max(1, g)` |
//!
//! so `|y_new - y_exact| <= 1.7e-6 * |x| * max(1, g)`, about `1.5e-5` dB and about 15 ulp of the
//! output. The pre-audit `log10f`/`powf` path sat inside a similar envelope around the same exact
//! value, so the old and the new bits differ by at most about `3e-5` dB — measured at `4.7e-6` dB (8 ulp) over a corpus of four launch rates, three link modes, twelve parameter points, impulse, step and decay
//! on the corpora of these gates. The `2.0e-5` row tolerance and the `0.01` dB gate below are the
//! pre-audit ones, unchanged.

mod common;

use common::*;
use miso_engine_dsp_reference::{ReferenceTransientShaper, ReferenceTransientShaperParameters};
use miso_engine_effect_contract::EffectProcessBlock;

/// Red mutation: `DB_PER_OCTAVE = 20.0` (the `log10`/`log2` confusion) — the error leaves 0.5 dB.
#[test]
fn scalar_matches_the_independent_f64_oracle() {
    let mut effect = prepare(&values_of(0.75, -0.5, 1.0));
    let mut reference = ReferenceTransientShaper::new(
        48_000.0,
        ReferenceTransientShaperParameters {
            attack_amount: 0.75,
            sustain_amount: -0.5,
            mix: 1.0,
        },
    )
    .expect("reference");
    let mut left = (0..96)
        .map(|index| ((index as f32 * 0.071).sin() * 0.7).max(-0.7))
        .collect::<Vec<_>>();
    let mut right = left.clone();
    let input = left.clone();
    effect
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"));
    let mut worst = 0.0_f32;
    for (sample, original) in left.iter().zip(input) {
        let expected =
            reference.process_sample(f64::from(original), f64::from(original.abs())) as f32;
        let error = (sample - expected).abs();
        worst = worst.max(error);
        assert!(error < 2.0e-5, "sample={sample} expected={expected}");
    }
    println!("worst |production - oracle| on the 96-sample sine row: {worst:e}");
}

/// Issue-020 gate 2: the shaped gain tracks the oracle to 0.01 dB on an impulse, a step and a
/// decay, covering both signs of both amounts.
///
/// This row is follower-dominated — the followers are class A, unchanged — so the transcendental
/// swap moves it by the `1.5e-5` dB of the header table, four orders of magnitude inside the gate.
///
/// Red mutation: `OCTAVES_PER_DB = 0.05` (the `exp2`/`pow10` confusion) — the gain law is wrong by
/// a factor of `log2(10)` in the exponent and every row leaves the gate.
#[test]
fn impulse_step_and_decay_cover_both_attack_and_sustain_signs() {
    let mut impulse = vec![0.0_f32; 32];
    impulse[0] = 1.0;
    let (attack_boost_error, _, attack_boost_maximum) = render_reference_row(1.0, 0.0, &impulse, 0);
    assert!(attack_boost_error <= 0.01, "error={attack_boost_error}");
    assert!(attack_boost_maximum > 0.25);

    let step = vec![1.0_f32; 64];
    let (attack_cut_error, attack_cut_minimum, _) = render_reference_row(-1.0, 0.0, &step, 0);
    assert!(attack_cut_error <= 0.01, "error={attack_cut_error}");
    assert!(attack_cut_minimum < -0.25);

    let mut decay = vec![1.0_f32; 4_800];
    decay.extend((0..512).map(|index| 0.9_f32 * 0.995_f32.powi(index)));
    let (sustain_boost_error, _, sustain_boost_maximum) =
        render_reference_row(0.0, 1.0, &decay, 4_800);
    assert!(sustain_boost_error <= 0.01, "error={sustain_boost_error}");
    assert!(sustain_boost_maximum > 0.25);
    let (sustain_cut_error, sustain_cut_minimum, _) =
        render_reference_row(0.0, -1.0, &decay, 4_800);
    assert!(sustain_cut_error <= 0.01, "error={sustain_cut_error}");
    assert!(sustain_cut_minimum < -0.25);
}

/// Renders one signal through the production path and the oracle, returning the worst gain error in
/// dB and the range of reference gains the row actually exercised.
fn render_reference_row(
    attack_amount: f32,
    sustain_amount: f32,
    signal: &[f32],
    measured_from: usize,
) -> (f64, f64, f64) {
    let mut effect = prepare(&values_of(attack_amount, sustain_amount, 1.0));
    let mut reference = ReferenceTransientShaper::new(
        48_000.0,
        ReferenceTransientShaperParameters {
            attack_amount: f64::from(attack_amount),
            sustain_amount: f64::from(sustain_amount),
            mix: 1.0,
        },
    )
    .expect("reference");
    let mut maximum_error_db = 0.0_f64;
    let mut minimum_reference_gain_db = f64::INFINITY;
    let mut maximum_reference_gain_db = f64::NEG_INFINITY;
    for (index, input) in signal.iter().copied().enumerate() {
        let mut left = [input];
        let mut right = [input];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, index as u64, &[], 128)
                .expect("row sample"),
        );
        let expected = reference.process_sample(f64::from(input), f64::from(input.abs()));
        if index >= measured_from && input.abs() >= 1.0e-4 {
            let production_gain_db =
                20.0_f64 * (f64::from(left[0].abs()) / f64::from(input.abs())).log10();
            let reference_gain_db = 20.0_f64 * (expected.abs() / f64::from(input.abs())).log10();
            maximum_error_db = maximum_error_db.max((production_gain_db - reference_gain_db).abs());
            minimum_reference_gain_db = minimum_reference_gain_db.min(reference_gain_db);
            maximum_reference_gain_db = maximum_reference_gain_db.max(reference_gain_db);
        }
    }
    (
        maximum_error_db,
        minimum_reference_gain_db,
        maximum_reference_gain_db,
    )
}
