//! Gate 7.3: the rendered gain agrees with an independent `f64` model inside a derived tolerance.
//!
//! The pin is `miso_engine_dsp_reference::reference_gate_expander_process`, an `f64` transcription
//! of brief 014 that shares no type and no line of code with the engine. It is never the old
//! production bits: those are what the audit found wrong (master plan #83 §8).
//!
//! # Where the tolerance comes from
//!
//! * `log2_lane` is qualified at 2 ulp (gate M1). At the top of the level range one ulp of a
//!   `log2` result near `2^5` is `2^-19`, so `|dX| <= 2 * 2^-19 * 6.0206 = 2.3e-5 dB`; taking the
//!   whole clamped range into account and rounding up, `|dX| <= 4.8e-5 dB`.
//! * That error enters the one-pole as a disturbance: `e' = a * e + delta` with
//!   `|delta| <= ulp(48) + (1 - a) * 4.8e-5`, whose steady state is `|delta| / (1 - a)`. The
//!   slowest release in this case is 100 ms at 48 kHz, `1 - a = 2.08e-4`, and `ulp(48) = 3.8e-6`,
//!   so the accumulated bound is `3.8e-6 / 2.08e-4 + 4.8e-5 = 0.0184 dB`.
//! * `exp2_lane` is qualified at 2 ulp, worth about `2e-6 dB` on the applied gain.
//!
//! Total under `0.02 dB`, which is the number asserted. It is not loosened: if the assertion
//! fails, the cause is an operation order or a coefficient, not the polynomials.

mod support;

use miso_engine_dsp_reference::{
    ReferenceGateExpanderParameters, ReferenceGateLink, ReferenceGatePhase, ReferenceGateTiming,
    reference_gate_expander_process,
};
use miso_engine_effect_contract::LinkMode;
use support::{
    Values, initial_values, prepare, prepare_bank_w8, render_scalar, request, set_parameter,
    track_of,
};

const FRAMES: usize = 48_000;
const THRESHOLD: f32 = -40.0;
const RATIO: f32 = 4.0;
const RANGE: f32 = 48.0;
const HYSTERESIS: f32 = 6.0;
const ATTACK_MS: f32 = 1.0;
const HOLD_MS: f32 = 5.0;
const RELEASE_MS: f32 = 100.0;
const LOOKAHEAD_MS: f32 = 2.0;

/// The tolerance derived in the module documentation. Never loosened.
const TOLERANCE_DB: f64 = 0.02;

fn corpus_values() -> Values {
    let mut values = initial_values();
    set_parameter(&mut values, 0, THRESHOLD, THRESHOLD);
    set_parameter(&mut values, 1, RATIO, RATIO);
    set_parameter(&mut values, 2, RANGE, RANGE);
    set_parameter(&mut values, 3, HYSTERESIS, HYSTERESIS);
    set_parameter(&mut values, 4, ATTACK_MS, ATTACK_MS);
    set_parameter(&mut values, 5, HOLD_MS, HOLD_MS);
    set_parameter(&mut values, 6, RELEASE_MS, RELEASE_MS);
    set_parameter(&mut values, 7, LOOKAHEAD_MS, LOOKAHEAD_MS);
    values
}

/// DC-free square-wave bursts: the sign flips every eight samples, and the amplitude alternates
/// between -30 and -50 dBFS every 4 800 samples, in antiphase between the two channels so that
/// the three link modes produce three different detector levels.
///
/// The quiet level is -50 dBFS rather than something deeper on purpose. At `rho = 4` and
/// `T = -40` the curve gives `3 * (-50 + 40) = -30 dB`, which is inside the 48 dB range: a level
/// that saturated the range clamp would make the comparison insensitive to the whole dB
/// conversion, since every closed sample would read exactly `-R` however the level was computed.
fn corpus_signals() -> (Vec<f32>, Vec<f32>) {
    let loud = 10.0_f32.powf(-30.0 / 20.0);
    let quiet = 10.0_f32.powf(-50.0 / 20.0);
    let mut left = vec![0.0; FRAMES];
    let mut right = vec![0.0; FRAMES];
    for frame in 0..FRAMES {
        let burst = (frame / 4_800) % 2 == 0;
        let sign = if (frame / 8) % 2 == 0 { 1.0 } else { -1.0 };
        left[frame] = sign * if burst { loud } else { quiet };
        right[frame] = sign * if burst { quiet } else { loud };
    }
    (left, right)
}

fn reference(
    link: LinkMode,
    left: &[f32],
    right: &[f32],
) -> miso_engine_dsp_reference::ReferenceGateTrace {
    let parameters = ReferenceGateExpanderParameters {
        threshold_db: f64::from(THRESHOLD),
        ratio: f64::from(RATIO),
        range_db: f64::from(RANGE),
    };
    let timing = ReferenceGateTiming {
        sample_rate: 48_000,
        attack_ms: f64::from(ATTACK_MS),
        hold_ms: f64::from(HOLD_MS),
        release_ms: f64::from(RELEASE_MS),
        lookahead_ms: f64::from(LOOKAHEAD_MS),
    };
    let link = match link {
        LinkMode::DualMono => ReferenceGateLink::DualMono,
        LinkMode::Maximum => ReferenceGateLink::Maximum,
        LinkMode::Average => ReferenceGateLink::Average,
    };
    let left: Vec<f64> = left.iter().map(|&x| f64::from(x)).collect();
    let right: Vec<f64> = right.iter().map(|&x| f64::from(x)).collect();
    reference_gate_expander_process(
        parameters,
        parameters,
        (f64::from(HYSTERESIS), f64::from(HYSTERESIS)),
        (timing, timing),
        link,
        (&left, &right),
        None,
    )
    .expect("reference render")
}

/// Compares one rendered channel against the model's `G` and returns the worst deviation.
fn compare(
    rendered: &[f32],
    gain_db: &[f64],
    dry: &[f64],
    phase: &[ReferenceGatePhase],
    context: &str,
) -> f64 {
    let mut worst = 0.0_f64;
    for frame in 0..rendered.len() {
        let z = dry[frame];
        let y = f64::from(rendered[frame]);
        if phase[frame] == ReferenceGatePhase::Open && gain_db[frame] == 0.0 {
            assert_eq!(
                rendered[frame].to_bits(),
                (z as f32).to_bits(),
                "{context}: an open gate is the exact identity at frame {frame}"
            );
        }
        if z.abs() < 1.0e-3 {
            continue;
        }
        let applied = 20.0 * (y.abs() / z.abs()).log10();
        let deviation = (applied - gain_db[frame]).abs();
        assert!(
            deviation <= TOLERANCE_DB,
            "{context}: frame {frame} deviates {deviation} dB from the f64 model (G = {}, limit {TOLERANCE_DB})",
            gain_db[frame]
        );
        worst = worst.max(deviation);
    }
    worst
}

#[test]
fn every_decision_is_at_least_one_decibel_clear_of_a_threshold() {
    // Assertion (a): the corpus must not sit on a decision boundary, or a single-ulp difference
    // in the level would flip a transition and the tolerance below would be measuring the wrong
    // thing entirely.
    let (left, right) = corpus_signals();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let trace = reference(link, &left, &right);
        for level in trace.level_db_left.iter().chain(&trace.level_db_right) {
            let open = (level - f64::from(THRESHOLD)).abs();
            let rearm = (level - f64::from(THRESHOLD - HYSTERESIS)).abs();
            assert!(
                open >= 1.0 && rearm >= 1.0,
                "{link:?}: level {level} dB is within 1 dB of a decision threshold"
            );
        }
    }
}

#[test]
fn oracle_pcm_within_derived_tolerance_scalar() {
    let (source_left, source_right) = corpus_signals();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let values = corpus_values();
        let mut request = request(&values);
        request.link_mode = link;
        let mut effect = prepare(request);
        let (mut left, mut right) = (source_left.clone(), source_right.clone());
        render_scalar(effect.as_mut(), &mut left, &mut right, 128);
        let trace = reference(link, &source_left, &source_right);
        let worst_left = compare(
            &left,
            &trace.gain_db_left,
            &trace.dry_left,
            &trace.phase_left,
            &format!("{link:?} left"),
        );
        let worst_right = compare(
            &right,
            &trace.gain_db_right,
            &trace.dry_right,
            &trace.phase_right,
            &format!("{link:?} right"),
        );
        assert!(
            worst_left.max(worst_right) > 0.0,
            "{link:?}: the corpus never attenuated, so the comparison is vacuous"
        );
        // And the attenuation must not be sitting on the range clamp, where the dB conversion
        // stops being observable at all.
        assert!(
            trace
                .gain_db_left
                .iter()
                .any(|gain| *gain < -1.0 && *gain > -f64::from(RANGE) + 1.0),
            "{link:?}: every attenuated sample is pinned to the range clamp"
        );
        eprintln!(
            "{link:?}: worst deviation from the f64 model {:.3e} dB (limit {TOLERANCE_DB})",
            worst_left.max(worst_right)
        );
    }
}

#[test]
fn oracle_pcm_within_derived_tolerance_w8() {
    let (source_left, source_right) = corpus_signals();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let values = [corpus_values(); 8];
        let Some(mut bank) = prepare_bank_w8(&values, link) else {
            eprintln!("no eight-lane backend on this build; scalar leg still gates");
            return;
        };
        let mut left = support::packed_w8(&vec![source_left.clone(); 8]);
        let mut right = support::packed_w8(&vec![source_right.clone(); 8]);
        let offsets = [0_u32; 9];
        let mut start = 0;
        while start < FRAMES {
            let end = (start + 128).min(FRAMES);
            let frames = (end - start) as u32;
            bank.process_bank(
                miso_engine_effect_contract::EffectBankProcessBlock::new(
                    &mut left[start * 8..end * 8],
                    &mut right[start * 8..end * 8],
                    None,
                    frames,
                    miso_engine_effect_contract::BankWidth::Eight,
                    start as u64,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            start = end;
        }
        let trace = reference(link, &source_left, &source_right);
        for track in 0..8 {
            compare(
                &track_of(&left, track, 8),
                &trace.gain_db_left,
                &trace.dry_left,
                &trace.phase_left,
                &format!("{link:?} W8 track {track} left"),
            );
            compare(
                &track_of(&right, track, 8),
                &trace.gain_db_right,
                &trace.dry_right,
                &trace.phase_right,
                &format!("{link:?} W8 track {track} right"),
            );
        }
    }
}
