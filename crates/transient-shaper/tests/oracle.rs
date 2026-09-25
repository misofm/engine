#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Comparison against the independent `f64` oracle in `dsp-reference`.
//!
//! The oracle derives its four coefficients from the frozen time constants with `f64` `exp`, runs
//! the follower in `f64`, takes the contrast as two `f64` `log10`s and the gain as `f64` `powf`.
//! Nothing in `math`, `lane` or `effect-runtime` is reachable
//! from it, which is what makes it independent of the chain it is checking.
//!
//! # Bound after approved crossings X7/X8
//!
//! MA-5 exhaustively checked every ratio in `[1e-8, 16]` and the four `attack/sustain ∈ {-1, +1}`
//! corners. X7's level conversion is followed by a clamp; X8 converts the clamped shape directly
//! from dB. These measurements qualify the ratio range and the four corner combinations, but do
//! not establish a uniform error bound for every interior attack/sustain value. The measured
//! results are:
//!
//! | quantity | exhaustive coverage | maximum |
//! |---|---|---|
//! | X7 fast contrast vs exact-tier contrast after the ±24 dB clamp | 257,176,458 ratios | `1.525879e-5` dB |
//! | X8 fast applied gain vs exact-tier applied gain | same ratios × four amount corners | `1.654115e-5` dB |
//! | fast applied gain vs independent f64 `20 log10`/`10^(dB/20)` oracle, unit input | same ratios × four corners | `1.287460e-5` absolute gain |
//! | exact-tier applied gain vs that oracle, unit input | same ratios × four corners | `2.199415e-6` absolute gain |
//!
//! The existing 96-sample oracle row uses interior amounts `attack=0.75`, `sustain=-0.5`. The
//! complete production-path measurement for this row is in `docs/issue880-mb2.md`; its maximum
//! exceeds the frozen `2.0e-5` absolute sample tolerance. That gate remains unchanged pending an
//! owner ruling. The impulse, step and decay rows remain within their existing `0.01` dB gates.

mod common;

use common::*;
use dsp_reference::{ReferenceTransientShaper, ReferenceTransientShaperParameters};
use effect_contract::EffectProcessBlock;

/// Red mutation: substituting a `20 dB/octave` scale for the fast tier's `20 log10(2)` scale.
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
/// Red mutation: applying the wrong dB-to-gain scale before the fast gain conversion leaves rows
/// outside the existing `0.01` dB gate.
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
