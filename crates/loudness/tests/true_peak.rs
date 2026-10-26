//! EBU Tech 3341 (2023) Table 1 cases 15-23, true peak, at every launch rate.
//!
//! Each case's largest true-peak reading since reset, in dBTP, must lie in
//! `[expected - 0.4, expected + 0.2]`, and must agree with the `dsp-reference` `f64` Annex-2
//! estimate of the same `f32` samples to [`ORACLE_BOUND`] (linear): the detector runs in `f32`,
//! and its rounding is the only difference the bound admits.

#![allow(clippy::disallowed_methods)]
// Test file: dBTP through the platform's log10, deliberately (D6 exemption).

mod common;

use common::{planes, run};
use dsp_reference::loudness::{reference_true_peak_burst, reference_true_peak_sine};
use dsp_reference::reference_true_peak_estimate;
use loudness::{LoudnessAnalysis, SUPPORTED_RATES};

/// Largest admitted `|production - oracle|`, linear full scale; the measured worst is 9.6e-8.
const ORACLE_BOUND: f64 = 1e-6;

/// Seconds of each test tone.
const SECONDS: f64 = 1.0;

#[test]
fn tech_3341_true_peak_cases_at_every_launch_rate() {
    for rate in SUPPORTED_RATES {
        let mut cases: Vec<(u32, Vec<f64>, f64)> = vec![
            (
                15,
                reference_true_peak_sine(rate, 4, 0.5, 0.0, SECONDS),
                -6.0,
            ),
            (
                16,
                reference_true_peak_sine(rate, 4, 0.5, 45.0, SECONDS),
                -6.0,
            ),
            (
                17,
                reference_true_peak_sine(rate, 6, 0.5, 60.0, SECONDS),
                -6.0,
            ),
            (
                18,
                reference_true_peak_sine(rate, 8, 0.5, 67.5, SECONDS),
                -6.0,
            ),
            (
                19,
                reference_true_peak_sine(rate, 4, 1.41, 45.0, SECONDS),
                3.0,
            ),
        ];
        for offset in 0..4 {
            cases.push((
                20 + offset as u32,
                reference_true_peak_burst(rate, offset, SECONDS),
                0.0,
            ));
        }
        for (case, signal, expected) in cases {
            let (plane, widened) = planes(&signal);
            let mut analysis = LoudnessAnalysis::new();
            run(rate, &plane, &mut analysis, |_, _| {});
            let reference = reference_true_peak_estimate(&widened).expect("finite");
            for (channel, peak) in analysis.true_peak().into_iter().enumerate() {
                let peak = f64::from(peak);
                let dbtp = 20.0 * peak.log10();
                assert!(
                    (expected - 0.4..=expected + 0.2).contains(&dbtp),
                    "case {case} at {rate} Hz, channel {channel}: {dbtp:.3} dBTP, expected \
                     {expected} +0.2/-0.4"
                );
                assert!(
                    (peak - reference).abs() <= ORACLE_BOUND,
                    "case {case} at {rate} Hz, channel {channel}: {peak} against the oracle's \
                     {reference}"
                );
            }
        }
    }
}
