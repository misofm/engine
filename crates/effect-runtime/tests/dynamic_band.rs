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
/// oracle's, and a million samples stay finite. The two linked rows feed the burst to the partner
/// channel only, against the oracle's level of `max(|a|, |b|)` and of the mean.
#[test]
fn the_band_step_matches_the_f64_oracle_on_a_burst() {
    let words = words();
    let armed_after = lane::silence_frames(RATE) as f32;
    // (upward, knee, link): 0 dual mono on the own channel, 1 maximum and 2 average, partner only.
    for (upward, knee, linked) in [
        (false, 0.0_f32, 0),
        (true, 6.0, 0),
        (false, 0.0, 1),
        (false, 0.0, 2),
    ] {
        let link = LinkWords {
            maximum: f32::from(u8::from(linked == 1)),
            average: f32::from(u8::from(linked == 2)),
        };
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
        let design =
            ReferenceRbjBiquad::new(f64::from(RATE), FREQUENCY, Q, ReferenceBandTap::BandPass)
                .expect("filter");
        let mut filters = [design; 2];
        let mut oracle = ReferenceDynamicBand::new(f64::from(RATE), law);
        let mut svf = [SvfState::<f32>::default(); 2];
        let (mut runs, mut gain_db) = ([0.0_f32; 2], 0.0_f32);
        let (mut ours, mut theirs) = (Vec::new(), Vec::new());
        for n in 0..SAMPLES {
            // Channel 0 is this band's own, channel 1 its partner.
            let xs = if linked == 0 {
                [input(n), 0.0]
            } else {
                [0.0, input(n)]
            };
            let mut bands = [0.0_f32; 2];
            for channel in 0..2 {
                let x = xs[channel];
                let rest = lane::silence_step(x, &mut runs[channel], armed_after);
                let (v1, v2, band) = band_step(true, x, &words, rest, &mut svf[channel]);
                assert!(
                    [v1, v2, band].iter().all(|word| word.is_finite()),
                    "sample {n}"
                );
                bands[channel] = band;
            }
            let level = detector_level_db(bands[0], bands[1], &link);
            let delta = dynamic_band_gain(level, &coef, &mut gain_db);
            assert!(
                delta.is_finite(),
                "upward {upward}: sample {n} is not finite"
            );
            let taps = [0, 1].map(|channel| filters[channel].process(f64::from(xs[channel])).abs());
            let detector = match linked {
                0 => taps[0],
                1 => taps[0].max(taps[1]),
                _ => 0.5 * (taps[0] + taps[1]),
            };
            let reference = oracle.step(reference_level_db(detector));
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
            "upward {upward}, link {linked}: change differs by {worst} dB"
        );
        let deepest = theirs
            .iter()
            .fold(0.0_f64, |m, &v| if v.abs() > m.abs() { v } else { m });
        assert!(
            deepest.abs() > 3.0,
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
