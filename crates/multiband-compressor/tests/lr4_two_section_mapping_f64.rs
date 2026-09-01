#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Phase 0 of audit #94 wave 2: the LR4 sign and phase convention, proved in `f64` before any
//! production line moves.
//!
//! The audit's finding F4 says the four-section crossover collapses to two: sections 0 and 2 carry
//! bit-identical state, and the band sum is exactly the first section's all-pass output. The
//! two-section form this test pins is
//!
//! ```text
//! (v1a, v2a) = svf(x)          lp1 = v2a          ap = x - 2k*v1a
//! (_,   v2b) = svf(lp1)        low = v2b          high = ap - low
//! ```
//!
//! with `k = sqrt(2)` (Butterworth `Q = 1/sqrt(2)`) and `g = tan(pi fc / fs)`. Nothing here is
//! imported from the crate under test: the recurrence is transcribed from Simper's equations and
//! is checked against two independent oracles.
//!
//! # Oracle 1 — the merged four-section `f64` reference
//!
//! [`ReferenceLr4Crossover`] (issue #105 phase 1, merged at `d019fcb`) runs four sections with its
//! own coefficient design and documents the same no-inversion convention: neither band is
//! polarity-inverted and the recombination is the plain sum `low + high`. Its own gates measure
//! the analytic sum flat to `4.985573e-13` dB and each band at `-6.0206` dB at the crossover to
//! `2.575717e-13` dB, and its recurrence agrees with the cascaded state space to `1.110223e-16`.
//!
//! # Oracle 2 — the closed-form all-pass
//!
//! Not a recurrence at all: the second-order Butterworth all-pass under the bilinear map,
//! `H(e^jw) = D(-jt) / D(jt)` with `t = tan(w/2) / g` and `D(s) = s^2 + sqrt(2) s + 1`. The sum's
//! complex gain is measured by demodulation over exactly `fs` samples at integer-Hz probes, so
//! every probe is a whole number of periods and the estimate carries no leakage term.
//!
//! # Why the sign is worth a gate of its own
//!
//! `high = low - ap` and `ap = x - k*v1` both leave the sum flat at DC and at Nyquist and fail
//! only near the crossover, which is exactly where a four-section implementation and a two-section
//! one have to agree. Both are run as red mutations; see `tests/MUTATIONS.md`.

use dsp_reference::{ReferenceLr4Crossover, deterministic_bipolar_noise};

/// Butterworth `k = 1 / Q = sqrt(2)`.
const K: f64 = core::f64::consts::SQRT_2;

/// Launch sample rates.
const RATES: [f64; 4] = [44_100.0, 48_000.0, 88_200.0, 96_000.0];

/// Crossover frequencies at the ends and the middle of the frozen 80 Hz .. 8 kHz domain.
const CROSSOVERS: [f64; 3] = [80.0, 1_000.0, 8_000.0];

/// One TPT state-variable section in the master plan §4.2 storage (`c1 = t / (1 + t)`).
#[derive(Clone, Copy, Debug)]
struct Section {
    c1: f64,
    a2: f64,
    a3: f64,
    ic1: f64,
    ic2: f64,
}

impl Section {
    fn new(sample_rate_hz: f64, crossover_hz: f64) -> Self {
        let g = (core::f64::consts::PI * crossover_hz / sample_rate_hz).tan();
        let t = g * (g + K);
        let c1 = t / (1.0 + t);
        let a1 = 1.0 - c1;
        Self {
            c1,
            a2: g * a1,
            a3: g * (g * a1),
            ic1: 0.0,
            ic2: 0.0,
        }
    }

    /// One frozen §4.2 step; returns `(v1, v2)` — the band-pass and low-pass taps.
    fn step(&mut self, v0: f64) -> (f64, f64) {
        let v3 = v0 - self.ic2;
        let d1 = self.a2 * v3 - self.c1 * self.ic1;
        let v1 = self.ic1 + d1;
        let d2 = self.a2 * self.ic1 + self.a3 * v3;
        let v2 = self.ic2 + d2;
        self.ic1 += d1 + d1;
        self.ic2 += d2 + d2;
        (v1, v2)
    }
}

/// The two-section LR4 split under test.
#[derive(Clone, Copy, Debug)]
struct TwoSection {
    a: Section,
    b: Section,
    nk2: f64,
}

impl TwoSection {
    fn new(sample_rate_hz: f64, crossover_hz: f64) -> Self {
        let section = Section::new(sample_rate_hz, crossover_hz);
        Self {
            a: section,
            b: section,
            nk2: -2.0 * K,
        }
    }

    /// Returns `(low, high, ap)`.
    fn step(&mut self, x: f64) -> (f64, f64, f64) {
        let (v1a, lp1) = self.a.step(x);
        let ap = self.nk2 * v1a + x;
        let (_, low) = self.b.step(lp1);
        (low, ap - low, ap)
    }
}

/// Complex gain of `output` relative to `input` at `frequency_hz`, by demodulation.
fn complex_gain(
    input: &[f64],
    output: &[f64],
    sample_rate_hz: f64,
    frequency_hz: f64,
) -> (f64, f64) {
    let step = core::f64::consts::TAU * frequency_hz / sample_rate_hz;
    let (mut xr, mut xi, mut yr, mut yi) = (0.0, 0.0, 0.0, 0.0);
    for (index, (x, y)) in input.iter().zip(output.iter()).enumerate() {
        let phase = step * index as f64;
        let (sin, cos) = phase.sin_cos();
        xr += x * cos;
        xi -= x * sin;
        yr += y * cos;
        yi -= y * sin;
    }
    let norm = xr * xr + xi * xi;
    ((yr * xr + yi * xi) / norm, (yi * xr - yr * xi) / norm)
}

/// Analytic all-pass response `D(-jt) / D(jt)` as `(magnitude_db, phase_degrees)`.
fn analytic_allpass(sample_rate_hz: f64, crossover_hz: f64, frequency_hz: f64) -> f64 {
    let g = (core::f64::consts::PI * crossover_hz / sample_rate_hz).tan();
    let t = (core::f64::consts::PI * frequency_hz / sample_rate_hz).tan() / g;
    // D(jt) = (1 - t^2) + j k t; the all-pass is its conjugate over itself, so the phase is
    // minus twice the argument of D(jt) and the magnitude is exactly one.
    -2.0 * (K * t).atan2(1.0 - t * t).to_degrees()
}

/// Wraps a degree difference into `(-180, 180]`.
fn wrap_degrees(mut degrees: f64) -> f64 {
    while degrees <= -180.0 {
        degrees += 360.0;
    }
    while degrees > 180.0 {
        degrees -= 360.0;
    }
    degrees
}

/// One-sixth-octave integer-Hz probes from 20 Hz up to `0.45 * fs`, plus the crossover itself.
fn probes(sample_rate_hz: f64, crossover_hz: f64) -> Vec<f64> {
    let mut list = Vec::new();
    for index in 0..=60 {
        let frequency = (20.0 * 2.0_f64.powf(index as f64 / 6.0)).round();
        if frequency <= 0.45 * sample_rate_hz && !list.contains(&frequency) {
            list.push(frequency);
        }
    }
    if !list.contains(&crossover_hz) {
        list.push(crossover_hz);
    }
    list
}

/// E1a: the two-section form reproduces the independent four-section `f64` oracle.
#[test]
fn two_section_bands_match_the_four_section_reference() {
    const FRAMES: usize = 200_000;
    let noise = deterministic_bipolar_noise(1, FRAMES, 0x5EED_1234_ABCD_0001).expect("noise");
    let samples = noise.channel(0).expect("channel");
    let (mut worst_low, mut worst_high, mut worst_sum) = (0.0_f64, 0.0_f64, 0.0_f64);
    for rate in RATES {
        for crossover in CROSSOVERS {
            let mut two = TwoSection::new(rate, crossover);
            let mut reference = ReferenceLr4Crossover::new(rate, crossover).expect("reference");
            for sample in samples.iter().copied() {
                let (low, high, ap) = two.step(sample);
                let (reference_low, reference_high) = reference.process_sample(sample);
                worst_low = worst_low.max((low - reference_low).abs());
                worst_high = worst_high.max((high - reference_high).abs());
                worst_sum = worst_sum.max(((low + high) - ap).abs());
            }
        }
    }
    eprintln!(
        "E1a worst_low={worst_low:e} worst_high={worst_high:e} worst_sum_minus_ap={worst_sum:e}"
    );
    assert!(worst_low <= 1.0e-12, "low band deviates by {worst_low:e}");
    assert!(
        worst_high <= 1.0e-12,
        "high band deviates by {worst_high:e}"
    );
    assert!(
        worst_sum <= 1.0e-15,
        "low + high is not the all-pass output: {worst_sum:e}"
    );
}

/// E1b: the sum is the second-order Butterworth all-pass, in magnitude and in phase.
#[test]
fn the_band_sum_is_the_butterworth_allpass() {
    let (mut worst_flatness, mut worst_phase) = (0.0_f64, 0.0_f64);
    for rate in RATES {
        let frames = rate as usize;
        for crossover in CROSSOVERS {
            for frequency in probes(rate, crossover) {
                let mut two = TwoSection::new(rate, crossover);
                let step = core::f64::consts::TAU * frequency / rate;
                // One second of warm-up so the transient is below the measurement floor, then one
                // second of measurement: an integer number of periods at an integer-Hz probe.
                for index in 0..frames {
                    two.step((step * index as f64).sin());
                }
                let mut input = Vec::with_capacity(frames);
                let mut sum = Vec::with_capacity(frames);
                for index in frames..2 * frames {
                    let x = (step * index as f64).sin();
                    let (low, high, _) = two.step(x);
                    input.push(x);
                    sum.push(low + high);
                }
                let (real, imaginary) = complex_gain(&input, &sum, rate, frequency);
                let flatness = 20.0 * real.hypot(imaginary).log10();
                let phase = wrap_degrees(
                    imaginary.atan2(real).to_degrees()
                        - analytic_allpass(rate, crossover, frequency),
                );
                worst_flatness = worst_flatness.max(flatness.abs());
                worst_phase = worst_phase.max(phase.abs());
                assert!(
                    flatness.abs() <= 0.001,
                    "rate={rate} crossover={crossover} probe={frequency} flatness={flatness:e} dB"
                );
                assert!(
                    phase.abs() <= 0.01,
                    "rate={rate} crossover={crossover} probe={frequency} phase={phase:e} deg"
                );
            }
        }
    }
    eprintln!("E1b worst_flatness_db={worst_flatness:e} worst_phase_deg={worst_phase:e}");
}
