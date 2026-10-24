#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately.
//! The de-esser's own output law, on the shared band step (whose detector and gain law
//! `effect-runtime`'s `tests/dynamic_band.rs` holds against the oracle): every arm against the
//! independent `f64` reference, and the spec's split-band acceptance gate.

mod support;

use dsp_reference::{
    ReferenceBandTap, ReferenceDeEsser, ReferenceDeEsserOutput, ReferenceDynamicBandLaw,
};
use effect_contract::LinkMode;
use support::{RATE, defaults, prepare, process, set, tone};

const FRAMES: usize = 24_000;

/// 8 kHz at -6 dBFS from 50 ms to 250 ms over a -30 dBFS 1 kHz tone.
fn input(n: usize) -> f32 {
    let burst = if (2_400..12_000).contains(&n) {
        tone(8_000.0, n, 0.5)
    } else {
        0.0
    };
    burst + tone(1_000.0, n, 0.0316)
}

/// Renders `FRAMES` of `signal` on both channels through a scalar de-esser with `values`.
fn render(values: &support::Values, signal: impl Fn(usize) -> f32) -> Vec<f32> {
    let mut effect = prepare(values, LinkMode::DualMono);
    let mut out: Vec<f32> = (0..FRAMES).map(&signal).collect();
    let mut right = out.clone();
    for (start, (left, right)) in out.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
        process(&mut effect, left, right, (start * 128) as u64, &[]);
    }
    out
}

/// Wideband `g x`, split `x - (1 - g) f` on the band and the high tap, and listen `f` each match
/// the reference sample for sample within `1e-5` (the input peaks near `0.53`; measured `2.9e-7`).
#[test]
fn every_output_arm_matches_the_reference() {
    use ReferenceDeEsserOutput::{Listen, Split, Wideband};
    for (mode, detector, listen, output, tap) in [
        (1.0, 1.0, 0.0, Wideband, ReferenceBandTap::BandPass),
        (2.0, 1.0, 0.0, Split, ReferenceBandTap::BandPass),
        (2.0, 2.0, 0.0, Split, ReferenceBandTap::HighPass),
        (2.0, 2.0, 1.0, Listen, ReferenceBandTap::HighPass),
    ] {
        let mut values = defaults();
        set(&mut values, 7, mode);
        set(&mut values, 8, detector);
        set(&mut values, 9, listen);
        let ours = render(&values, input);
        let law = ReferenceDynamicBandLaw {
            threshold_db: -20.0,
            ratio: 4.0,
            knee_db: 0.0,
            range_db: 20.0,
            attack_ms: 1.0,
            release_ms: 50.0,
            upward: false,
        };
        let mut reference =
            ReferenceDeEsser::new(f64::from(RATE), 6_500.0, 2.0, tap, law, output).expect("design");
        let mut deepest = 0.0_f64;
        for (n, &sample) in ours.iter().enumerate() {
            let (expected, change) = reference.process(f64::from(input(n)));
            deepest = deepest.min(change);
            assert!(
                (f64::from(sample) - expected).abs() <= 1.0e-5,
                "{output:?} {tap:?}: sample {n} is {sample}, the reference {expected}"
            );
        }
        assert!(
            deepest < -3.0,
            "{output:?} {tap:?}: the burst engages ({deepest} dB)"
        );
    }
}

/// `|X(bin)|` in dB of `samples` (a whole number of the bin's periods).
fn bin_db(samples: &[f32], frequency: f64) -> f64 {
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for (n, &x) in samples.iter().enumerate() {
        let phase = core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE);
        re += f64::from(x) * phase.cos();
        im -= f64::from(x) * phase.sin();
    }
    20.0 * re.hypot(im).log10()
}

/// In split mode at the defaults (6.5 kHz, Q 2, 20 dB range), a 1 kHz tone 2.7 octaves below the
/// centre, inside the documented nominal region (at least 1.83 octaves away at Q 2), moves by at
/// most 0.1 dB while the 7 kHz burst over it is cut by at least 3 dB.
#[test]
fn split_mode_cuts_the_band_and_leaves_an_out_of_band_tone() {
    let signal = |n: usize| {
        let burst = if (2_400..12_000).contains(&n) {
            tone(7_000.0, n, 0.5)
        } else {
            0.0
        };
        burst + tone(1_000.0, n, 0.0316)
    };
    let ours = render(&defaults(), signal);
    let dry: Vec<f32> = (0..FRAMES).map(signal).collect();
    // 100 ms inside the burst, after the attack: whole periods of both tones.
    let window = 7_200..12_000;
    let low = bin_db(&ours[window.clone()], 1_000.0) - bin_db(&dry[window.clone()], 1_000.0);
    let high = bin_db(&ours[window.clone()], 7_000.0) - bin_db(&dry[window], 7_000.0);
    assert!(low.abs() <= 0.1, "the 1 kHz tone moved {low} dB");
    assert!(high <= -3.0, "the 7 kHz burst moved only {high} dB");
}
