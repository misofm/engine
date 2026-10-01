#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Gate 7.3: the rendered gain agrees with an independent `f64` model inside a derived tolerance.
//!
//! The reference is `dsp_reference::reference_gate_expander_process`, an `f64` transcription
//! of the current causal gate law that shares no type or arithmetic helper with this effect.
//!
//! # Where the tolerance comes from
//!
//! * The sealed `fast_level_db` tier is qualified at at most `2.810e-5 dB` on the detector
//!   domain (`math::fast_db`'s exhaustive F1 bounds). Keep the conservative `4.8e-5 dB` allowance.
//! * The rate recurrence subtracts, multiplies and adds with separate rounding. At the slowest
//!   release here (100 ms at 48 kHz), `b = 2.08e-4`; the state-addition error is bounded by
//!   `ulp(48) = 3.8e-6`, giving `3.8e-6 / b + 4.8e-5 = 0.0184 dB`. The product is smaller than
//!   `b * 48 < 0.01`, so its rounding contributes less than `5e-6 dB` after accumulation. The
//!   preceding subtraction contributes at most another `ulp(48)` to the accumulated error.
//!   1 ms attack has a larger `b`, and therefore a smaller accumulated bound.
//! * `fast_gain_from_db` is qualified below `7.431e-6 dB` on negative gain values, again by F1.
//!
//! Total under the unchanged `0.02 dB` limit asserted below.

mod support;

use dsp_reference::{
    ReferenceGateExpanderParameters, ReferenceGateLink, ReferenceGatePhase, ReferenceGateTiming,
    reference_gate_expander_process,
};
use effect_contract::LinkMode;
use support::{
    NATIVE_LANES, Values, initial_values, native_width, prepare, prepare_bank_native,
    render_scalar, request, set_parameter, track_of,
};

const FRAMES: usize = 48_000;
const THRESHOLD: f32 = -40.0;
const RATIO: f32 = 4.0;
const RANGE: f32 = 48.0;
const HYSTERESIS: f32 = 6.0;
const ATTACK_MS: f32 = 1.0;
const HOLD_MS: f32 = 5.0;
const RELEASE_MS: f32 = 100.0;

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
    values
}

/// DC-free square-wave bursts: the sign flips every eight samples, with a shared active interval
/// and a shared quiet interval before an antiphase tail. The shared intervals force every link mode
/// to exercise both attenuation and quiet-state decisions; the tail still gives the link modes
/// distinct detector levels.
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
        let shared = frame < 9_600;
        let amplitude = if burst { loud } else { quiet };
        left[frame] = sign
            * if shared {
                amplitude
            } else if burst {
                loud
            } else {
                quiet
            };
        right[frame] = sign
            * if shared {
                amplitude
            } else if burst {
                quiet
            } else {
                loud
            };
    }
    (left, right)
}

fn reference(link: LinkMode, left: &[f32], right: &[f32]) -> dsp_reference::ReferenceGateTrace {
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
        for (channel, gains) in [
            ("left", &trace.gain_db_left),
            ("right", &trace.gain_db_right),
        ] {
            let active_nonclamped = gains
                .iter()
                .any(|gain| *gain < -1.0 && *gain > -f64::from(RANGE) + 1.0);
            assert!(
                active_nonclamped,
                "{link:?} {channel}: active attenuation is absent or range-clamped"
            );
            let quiet_nonclamped = gains[4_800..9_600]
                .iter()
                .any(|gain| *gain < -1.0 && *gain > -f64::from(RANGE) + 1.0);
            assert!(
                quiet_nonclamped,
                "{link:?} {channel}: quiet interval is absent or range-clamped"
            );
        }
        eprintln!(
            "{link:?}: worst deviation from the f64 model {:.3e} dB (limit {TOLERANCE_DB})",
            worst_left.max(worst_right)
        );
    }
}

/// At the build's own width: W8 in the 8-lane (AVX2) build, W4 in a 4-lane (NEON/simd128) build
/// (#1112).
#[test]
fn oracle_pcm_within_derived_tolerance_native_bank() {
    let (source_left, source_right) = corpus_signals();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let values = [corpus_values(); 8];
        let mut bank = prepare_bank_native(&values, link).expect("the build's own width binds");
        let mut left = support::packed(&[source_left.as_slice(); NATIVE_LANES]);
        let mut right = support::packed(&[source_right.as_slice(); NATIVE_LANES]);
        let offsets = [0_u32; NATIVE_LANES + 1];
        let mut start = 0;
        while start < FRAMES {
            let end = (start + 128).min(FRAMES);
            let frames = (end - start) as u32;
            bank.process_bank(
                effect_contract::EffectBankProcessBlock::new(
                    &mut left[start * NATIVE_LANES..end * NATIVE_LANES],
                    &mut right[start * NATIVE_LANES..end * NATIVE_LANES],
                    None,
                    frames,
                    native_width(),
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
        for track in 0..NATIVE_LANES {
            compare(
                &track_of(&left, track, NATIVE_LANES),
                &trace.gain_db_left,
                &trace.dry_left,
                &trace.phase_left,
                &format!("{link:?} W{NATIVE_LANES} track {track} left"),
            );
            compare(
                &track_of(&right, track, NATIVE_LANES),
                &trace.gain_db_right,
                &trace.dry_right,
                &trace.phase_right,
                &format!("{link:?} W{NATIVE_LANES} track {track} right"),
            );
        }
    }
}
