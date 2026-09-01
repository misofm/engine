//! E5 — the rendered signal tracks the independent `f64` oracle.
//!
//! `dsp_reference::ReferencePeakCompressor` is an independent transcription: its own
//! rings, its own `f64` curve, its own two-rounding one-pole, and the platform libm for `log10`,
//! `exp` and `powf`. It is deliberately *not* what the engine computes any more — this is the test
//! that says the class-B changes of #88 (one-rounding ballistic, `f64` coefficient design,
//! `log2`/`exp2` conversions, precomputed ramp step) did not change what the effect *is*.
//!
//! # The bound, derived
//!
//! The dominant term is the ballistic. `G` is a recursion `G += c (C - G)`; a per-sample
//! difference `d` between the two evaluations accumulates to at most `d / c` in the steady state.
//! Half an ulp of `G` at `|G| ~ 20` dB is `1e-6` dB, and the slowest coefficient used here is the
//! 100 ms release at 48 kHz, `c ~ 2.1e-4`, giving a bound of about `5e-3` dB on `G`. `5e-3` dB is
//! a relative gain error of `5.8e-4`, and the wet signal is at most `0.9` in magnitude here, so
//! the output bound is about `5.2e-4`... which is looser than the pre-audit `2e-5` this test
//! keeps. The `2e-5` holds because that worst case requires the two evaluations to differ by half
//! an ulp *in the same direction on every sample*, which a rounding difference does not do; the
//! measured worst deviations are printed by the test: 4.7e-7 for (a) and 1.2e-7 for (b), forty times
//! and better inside the gate.

mod support;

use dsp_reference::{ReferenceCompressorParameters, ReferencePeakCompressor};

use support::{prepare, render_scalar, request, values_with};

/// Renders one configuration through the effect and through the oracle and returns the worst
/// absolute deviation.
fn deviation(parameters: ReferenceCompressorParameters, input: &[f32]) -> f64 {
    let values = values_with(&[
        (0, parameters.threshold_db as f32),
        (1, parameters.ratio as f32),
        (2, parameters.knee_db as f32),
        (3, parameters.attack_ms as f32),
        (4, parameters.release_ms as f32),
        (5, parameters.makeup_db as f32),
        (6, parameters.mix as f32),
        (7, parameters.lookahead_ms as f32),
    ]);
    let mut effect = prepare(request(&values));
    let mut reference = ReferencePeakCompressor::new(48_000.0, parameters).expect("oracle");
    let expected: Vec<f32> = input
        .iter()
        .map(|sample| reference.process_sample(f64::from(*sample), f64::from(*sample)) as f32)
        .collect();

    let mut left = input.to_vec();
    let mut right = input.to_vec();
    render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);

    // The run must actually compress, or the comparison is vacuous.
    assert!(
        left[960..].iter().any(|sample| *sample != 0.0),
        "nothing was rendered past the latency"
    );
    assert!(
        expected[1_200].abs() < input[1_200].abs() * 0.99,
        "the configuration must exercise gain reduction"
    );

    let mut worst = 0.0_f64;
    for (actual, reference) in left.iter().zip(&expected) {
        let deviation = f64::from(*actual - *reference).abs();
        if deviation > worst {
            worst = deviation;
        }
    }
    worst
}

/// Both configurations stay within `2e-5` of the `f64` oracle.
///
/// Red mutations (MUTATIONS.md rows 5, 6, 10), all proven: use `c_release` for both ballistic
/// arms, drop `coef.makeup` from the gain step, and design the ballistic coefficient as the
/// retention `exp(-1/tau)` instead of the rate `1 - exp(-1/tau)`.
#[test]
fn scalar_output_matches_the_independent_f64_oracle() {
    let step: Vec<f32> = (0..2_048)
        .map(|index| if index < 1_280 { 0.9 } else { 0.1 })
        .collect();

    let plain = ReferenceCompressorParameters {
        threshold_db: -24.0,
        ratio: 8.0,
        knee_db: 0.0,
        attack_ms: 1.0,
        release_ms: 20.0,
        makeup_db: 0.0,
        mix: 1.0,
        lookahead_ms: 0.0,
    };
    let worst_plain = deviation(plain, &step);
    println!("E5 (a) hard knee, mix 1, no makeup: worst {worst_plain:.3e}");
    assert!(worst_plain <= 2.0e-5, "configuration (a): {worst_plain:e}");

    let full = ReferenceCompressorParameters {
        threshold_db: -30.0,
        ratio: 4.0,
        knee_db: 12.0,
        attack_ms: 5.0,
        release_ms: 100.0,
        makeup_db: 3.0,
        mix: 0.7,
        lookahead_ms: 5.0,
    };
    let worst_full = deviation(full, &step);
    println!("E5 (b) soft knee, mix 0.7, makeup +3, lookahead 5 ms: worst {worst_full:.3e}");
    assert!(worst_full <= 2.0e-5, "configuration (b): {worst_full:e}");
}
