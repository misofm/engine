#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately.
//! The shared dynamic band step (issue #15 S1, reused by #17) against its independent `f64`
//! oracle, `dsp_reference::dynamic_band`: an 8 kHz burst at -6 dBFS through a 6.5 kHz, Q 2 band,
//! downward with a hard knee (the de-esser's law) and upward with a 6 dB knee (the dynamic EQ's).

use dsp_reference::{
    ReferenceBandTap, ReferenceDynamicBand, ReferenceDynamicBandLaw, ReferenceRbjBiquad,
    reference_level_db,
};
use effect_runtime::dynamic_band::{
    BandWords, DynamicBandCoef, LinkWords, band_step, detector_level_db, dynamic_band_gain,
};
use effect_runtime::dynamics::GainComputerCoef;
use effect_runtime::envelope::attack_release_coefficient;
use lane::kernels::SvfState;

const RATE: u32 = 48_000;
const FREQUENCY: f64 = 6_500.0;
const Q: f64 = 2.0;
/// One period of the fixture: the burst starts at `ON` and stops at `OFF`.
const PERIOD: usize = 24_000;
const ON: usize = 2_400;
const OFF: usize = 7_200;
const SAMPLES: usize = 1_000_000;

/// The SVF words of `(FREQUENCY, Q)` at `RATE`, designed in `f64` and rounded once (Simper).
fn words() -> BandWords<f32> {
    let g = (core::f64::consts::PI * FREQUENCY / f64::from(RATE)).tan();
    let k = 1.0 / Q;
    let t = g * (g + k);
    let c1 = t / (1.0 + t);
    let a2 = g * (1.0 - c1);
    BandWords {
        c1: c1 as f32,
        a2: a2 as f32,
        a3: (g * a2) as f32,
        k: k as f32,
    }
}

fn input(n: usize) -> f32 {
    let phase = n % PERIOD;
    if (ON..OFF).contains(&phase) {
        (0.5 * (core::f64::consts::TAU * 8_000.0 * n as f64 / f64::from(RATE)).sin()) as f32
    } else {
        0.0
    }
}

/// Samples from `start` until `trace` first crosses `level` (downward when `falling`).
fn crossing(trace: &[f64], start: usize, level: f64, falling: bool) -> usize {
    trace[start..]
        .iter()
        .position(|&value| {
            if falling {
                value <= level
            } else {
                value >= level
            }
        })
        .expect("the trace crosses the level")
}

/// Per sample the smoothed change is within 0.1 dB of the oracle's, the attack (to 90 % of the
/// burst's deepest change) and the release (back to 10 %) are within max(1 sample, 2 %) of the
/// oracle's, and a million samples stay finite.
#[test]
fn the_band_step_matches_the_f64_oracle_on_a_burst() {
    let words = words();
    let armed_after = lane::silence_frames(RATE) as f32;
    let link = LinkWords {
        maximum: 0.0,
        average: 0.0,
    };
    for (upward, knee) in [(false, 0.0_f32), (true, 6.0)] {
        let law = ReferenceDynamicBandLaw {
            threshold_db: -20.0,
            ratio: 4.0,
            knee_db: f64::from(knee),
            range_db: 20.0,
            attack_ms: 1.0,
            release_ms: 50.0,
            upward,
        };
        let (half_knee_db, inv_two_knee) = effect_runtime::dynamics::knee_coefficients(knee);
        let coef = DynamicBandCoef {
            curve: GainComputerCoef {
                threshold_db: -20.0,
                inv_ratio_minus_one: 1.0 / 4.0 - 1.0,
                half_knee_db,
                inv_two_knee,
            },
            range_db: 20.0,
            upward: f32::from(u8::from(upward)),
            attack: attack_release_coefficient(1.0, RATE),
            release: attack_release_coefficient(50.0, RATE),
        };
        let mut filter =
            ReferenceRbjBiquad::new(f64::from(RATE), FREQUENCY, Q, ReferenceBandTap::BandPass)
                .expect("filter");
        let mut oracle = ReferenceDynamicBand::new(f64::from(RATE), law);
        let (mut svf, mut run, mut gain_db) = (SvfState::<f32>::default(), 0.0_f32, 0.0_f32);
        let (mut ours, mut theirs) = (Vec::new(), Vec::new());
        for n in 0..SAMPLES {
            let x = input(n);
            let rest = lane::silence_step(x, &mut run, armed_after);
            let (v1, v2, band) = band_step(x, &words, rest, &mut svf);
            let level = detector_level_db(band, band, &link);
            let delta = dynamic_band_gain(level, &coef, &mut gain_db);
            assert!(
                [v1, v2, band, delta].iter().all(|word| word.is_finite()),
                "upward {upward}: sample {n} is not finite"
            );
            let reference = oracle.step(reference_level_db(filter.process(f64::from(x))));
            if n < PERIOD {
                ours.push(f64::from(delta));
                theirs.push(reference);
            }
        }
        let worst = ours
            .iter()
            .zip(&theirs)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        assert!(
            worst <= 0.1,
            "upward {upward}: change differs by {worst} dB"
        );
        let deepest = theirs
            .iter()
            .fold(0.0_f64, |m, &v| if v.abs() > m.abs() { v } else { m });
        assert!(
            deepest.abs() > 6.0,
            "upward {upward}: the burst engages the band ({deepest})"
        );
        let sign = deepest.signum();
        let flip = |trace: &[f64]| trace.iter().map(|v| v * sign).collect::<Vec<_>>();
        let (ours, theirs) = (flip(&ours), flip(&theirs));
        for (start, level, rising, name) in [
            (ON, 0.9 * deepest.abs(), true, "attack"),
            (OFF, 0.1 * deepest.abs(), false, "release"),
        ] {
            let ours = crossing(&ours, start, level, !rising);
            let theirs = crossing(&theirs, start, level, !rising);
            let allowed = (0.02 * theirs as f64).max(1.0);
            assert!(
                (ours as f64 - theirs as f64).abs() <= allowed,
                "upward {upward}: {name} takes {ours} samples, the oracle {theirs}"
            );
        }
    }
}
