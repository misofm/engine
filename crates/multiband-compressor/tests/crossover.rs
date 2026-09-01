#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E2: the `f32` two-stage crossover against the independent `f64` oracle, and its flatness.
//!
//! The `f64` mapping is pinned separately, before any production line moved, in
//! `tests/lr4_two_section_mapping_f64.rs`. This gate is the `f32` production form: the same
//! `lr4_step` the render path runs, at `WIDTH = 1`.

use dsp_reference::ReferenceLr4Crossover;
use lane::Lane;
use multiband_compressor::{Lr4State, lr4_coefficients, lr4_step};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const CROSSOVERS: [f32; 3] = [80.0, 1_000.0, 8_000.0];

/// Complex gain of `output` relative to `input`, by demodulation over a whole number of periods.
fn complex_gain(input: &[f32], output: &[f32], sample_rate: f64, frequency: f64) -> (f64, f64) {
    let step = core::f64::consts::TAU * frequency / sample_rate;
    let (mut xr, mut xi, mut yr, mut yi) = (0.0, 0.0, 0.0, 0.0);
    for (index, (x, y)) in input.iter().zip(output.iter()).enumerate() {
        let (sin, cos) = (step * index as f64).sin_cos();
        xr += f64::from(*x) * cos;
        xi -= f64::from(*x) * sin;
        yr += f64::from(*y) * cos;
        yi -= f64::from(*y) * sin;
    }
    let norm = xr * xr + xi * xi;
    ((yr * xr + yi * xi) / norm, (yi * xr - yr * xi) / norm)
}

fn rms(values: &[f32]) -> f64 {
    (values
        .iter()
        .map(|value| f64::from(*value) * f64::from(*value))
        .sum::<f64>()
        / values.len() as f64)
        .sqrt()
}

/// E2. Per-sample agreement with the four-section `f64` oracle, the half-power crossing, and a
/// flat band sum on the frozen one-sixth-octave grid.
///
/// The per-sample bound is version 1's `2e-5`, kept and not loosened. The flatness bound is new
/// and an order of magnitude tighter than version 1's 0.05 dB RMS check, because `high = ap - low`
/// makes the sum the all-pass by construction rather than by cancellation.
///
/// Red mutation: swap `a2` and `a3` in the design; drop the `nk2` doubling.
#[test]
fn the_two_stage_split_matches_the_reference_and_recombines_flat() {
    let mut worst_band = 0.0f64;
    let mut worst_flatness = 0.0f64;
    for rate in RATES {
        for crossover in CROSSOVERS {
            let coefficients = lr4_coefficients::<f32>(rate, crossover).expect("design");
            let mut state = Lr4State::<f32>::default();
            let mut reference =
                ReferenceLr4Crossover::new(f64::from(rate), f64::from(crossover)).expect("oracle");
            let (mut input, mut low_band, mut sum) = (Vec::new(), Vec::new(), Vec::new());
            for index in 0..8_192 {
                let sample =
                    (core::f32::consts::TAU * crossover * index as f32 / rate as f32).sin();
                let (low, high) = lr4_step(sample, &coefficients, &mut state);
                let (expected_low, expected_high) = reference.process_sample(f64::from(sample));
                worst_band = worst_band
                    .max(f64::from((low - expected_low as f32).abs()))
                    .max(f64::from((high - expected_high as f32).abs()));
                assert!(
                    (low - expected_low as f32).abs() < 2.0e-5
                        && (high - expected_high as f32).abs() < 2.0e-5,
                    "rate={rate} crossover={crossover} index={index}"
                );
                if index >= 4_096 {
                    input.push(sample);
                    low_band.push(low);
                    sum.push(low + high);
                }
            }
            let crossing = 20.0 * (rms(&low_band) / rms(&input)).log10();
            assert!(
                (crossing + 6.020_599_913).abs() <= 0.02,
                "rate={rate} crossover={crossover} crossing={crossing}"
            );
            for index in 0..=60 {
                let probe = (20.0 * 2.0f64.powf(f64::from(index) / 6.0)).round();
                if probe > 0.45 * f64::from(rate) {
                    break;
                }
                let frames = rate as usize;
                let mut state = Lr4State::<f32>::default();
                let step = core::f64::consts::TAU * probe / f64::from(rate);
                for frame in 0..frames {
                    let _ = lr4_step(
                        (step * frame as f64).sin() as f32,
                        &coefficients,
                        &mut state,
                    );
                }
                let mut input = Vec::with_capacity(frames);
                let mut summed = Vec::with_capacity(frames);
                for frame in frames..2 * frames {
                    let sample = (step * frame as f64).sin() as f32;
                    let (low, high) = lr4_step(sample, &coefficients, &mut state);
                    input.push(sample);
                    summed.push(low + high);
                }
                let (real, imaginary) = complex_gain(&input, &summed, f64::from(rate), probe);
                let flatness = 20.0 * real.hypot(imaginary).log10();
                worst_flatness = worst_flatness.max(flatness.abs());
                assert!(
                    flatness.abs() <= 0.01,
                    "rate={rate} crossover={crossover} probe={probe} flatness={flatness:e} dB"
                );
            }
        }
    }
    eprintln!("E2 worst_band_abs={worst_band:e} worst_flatness_db={worst_flatness:e}");
}

/// The design guards its domain and its own half-power self-check.
#[test]
fn the_design_guards_its_domain() {
    assert!(lr4_coefficients::<f32>(0, 1_000.0).is_none());
    for crossover in [79.0f32, 8_001.0, f32::NAN, f32::INFINITY, -0.0] {
        assert!(
            lr4_coefficients::<f32>(48_000, crossover).is_none(),
            "{crossover}"
        );
    }
    for rate in RATES {
        for crossover in [80.0f32, 1_000.0, 8_000.0] {
            let coefficients = lr4_coefficients::<f32>(rate, crossover).expect("design");
            // `nk2` is exactly `-2 * sqrt(2)`, rounded once, on every design.
            assert_eq!(
                coefficients.nk2.to_bits(),
                (-2.0 * core::f32::consts::SQRT_2).to_bits()
            );
            assert!(coefficients.a2 > 0.0 && coefficients.a3 > 0.0);
        }
    }
}

/// A subnormal-seeded state reaches exactly zero, and stays there, at every width (D7).
#[test]
fn the_recursive_words_are_flushed() {
    fn check<L: Lane>() {
        let coefficients = lr4_coefficients::<L>(48_000, 1_000.0).expect("design");
        let mut state = Lr4State::<L> {
            a: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
            b: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
        };
        for _ in 0..4 {
            let _ = lr4_step(L::zero(), &coefficients, &mut state);
        }
        let mut words = [0u32; 8];
        for value in [state.a.ic1, state.a.ic2, state.b.ic1, state.b.ic2] {
            value.store_bits(&mut words[..L::WIDTH]);
            for word in &words[..L::WIDTH] {
                assert_eq!(*word, 0, "a flushed state word must be exactly +0.0");
            }
        }
    }
    check::<f32>();
    check::<lane::Simd4>();
    check::<lane::Simd8>();
}
