//! The bounded histograms against exact gating, on a programme whose blocks spread over many
//! bins and whose gates fall inside bins.
//!
//! Stepped noise: independent deterministic noise on each channel, its level stepped to a new
//! pseudo-random value in -60..-5 dBFS after a pseudo-random 0.3-2 s, for 150 s. The
//! `dsp-reference` oracle keeps every block level and gates exactly; the meter's integrated
//! loudness must agree to 0.05 LU and its loudness range to 0.1 LU.

#![allow(clippy::disallowed_methods)]
// Test file: the programme's gains through the platform's powf, deliberately (D6 exemption).

use dsp_reference::deterministic_bipolar_noise;
use dsp_reference::loudness::ReferenceLoudnessTrace;
use loudness::{LoudnessAnalysis, LoudnessObserver};

const SECONDS: f64 = 150.0;

/// The stepped-noise programme at `rate`, `[left, right]` as `f32`.
fn stepped_noise(rate: u32) -> [Vec<f32>; 2] {
    let frames = (SECONDS * f64::from(rate)) as usize;
    let noise = deterministic_bipolar_noise(2, frames, 0x2115).expect("noise");
    let mut state = 0x5eed_2115_u64;
    let mut next = move || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (state >> 11) as f64 / (1_u64 << 53) as f64
    };
    let mut gains = vec![0.0; frames];
    let mut at = 0;
    while at < frames {
        let length = ((0.3 + 1.7 * next()) * f64::from(rate)) as usize;
        let gain = 10.0_f64.powf((-60.0 + 55.0 * next()) / 20.0);
        for slot in gains.iter_mut().skip(at).take(length) {
            *slot = gain;
        }
        at += length;
    }
    [0, 1].map(|channel| {
        noise
            .channel(channel)
            .expect("channel")
            .iter()
            .zip(&gains)
            .map(|(x, g)| (x * g) as f32)
            .collect()
    })
}

#[test]
fn histogram_integrated_and_range_match_exact_gating() {
    for rate in [44_100, 96_000] {
        let [left, right] = stepped_noise(rate);
        let mut observer = LoudnessObserver::new(rate).expect("launch rate");
        let mut analysis = LoudnessAnalysis::new();
        let mut first = 0_u64;
        for (l, r) in left.chunks(1024).zip(right.chunks(1024)) {
            observer.observe(l, r, first, &mut |record| analysis.ingest(&record));
            first += l.len() as u64;
        }
        let widen = |plane: &[f32]| plane.iter().map(|x| f64::from(*x)).collect::<Vec<_>>();
        let oracle =
            ReferenceLoudnessTrace::measure(&widen(&left), &widen(&right), rate).expect("oracle");
        let end = oracle.segments();
        let integrated = analysis.integrated().expect("I") - oracle.integrated(0, end).expect("I");
        let range =
            analysis.loudness_range().expect("LRA") - oracle.loudness_range(0, end).expect("LRA");
        eprintln!("{rate} Hz: dI {integrated:+.5} LU, dLRA {range:+.5} LU");
        assert!(
            integrated.abs() <= 0.05,
            "{rate} Hz: I differs by {integrated} LU"
        );
        assert!(range.abs() <= 0.1, "{rate} Hz: LRA differs by {range} LU");
    }
}
