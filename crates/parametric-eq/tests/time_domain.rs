//! Time-domain acceptance gates: what the kernel actually renders, over long windows.
//!
//! These are the gates issue #42 stopped on and issues #44/#45 could not satisfy. They are not
//! satisfiable by *any* correct filter as they were written, because the recovery predicate they
//! used counted subnormal state and subnormal output as a fault: a decaying impulse response passes
//! through `1e-38 .. 1e-45` on its way to zero, and the `f64` oracle's own `f32` cast contains 2,379
//! subnormal samples on the 44.1 kHz / 20 kHz / +24 dB / Q = 18 bell. Decision D7 replaces that with
//! one flush of the two integrator words inside the kernel and one finiteness check per block; a
//! subnormal sample is a signal value, and only a non-finite one is a fault.

mod support;

use dsp_reference::{ReferenceParametricEqCoefficients, ReferenceParametricEqSection};
use effect_contract::{EffectProcessBlock, NativeEffectFactory};
use engine::SampleRateHz;
use parametric_eq::ParametricEqFactory;
use support::{
    FROZEN_EDGES, FROZEN_KINDS, LAUNCH_RATES, SECTIONS, WORDS_PER_BAND, band_word, dft_db,
    impulse_dft_db, one_second_impulse, reference_kind, request_at_rate, single_section_values,
    snapshot,
};

/// The frozen one-second DFT tolerance of issues #42/#44.
const ONE_SECOND_DFT_TOLERANCE_DB: f64 = 0.05;
/// The frozen exclusion floor.
const RESPONSE_FLOOR_DB: f64 = -120.0;
/// The D7 flush band: a retained word is exactly `+0.0` or at least this large.
const FLUSH_EPS: f32 = 1.0e-20;

/// The finite-window `f64` reference impulse of one row, in the same window the gate measures.
///
/// Comparing against the *analytic* `|H(f0)|` is what issue #44 corrected in the reference crate:
/// a one-second window of a 10 Hz response is truncated, and the `f64` oracle itself differs from
/// the analytic value by 0.0085 dB (bell) to 0.0154 dB (low pass) — thirty per cent of the frozen
/// 0.05 dB budget spent before the production path is even involved.
fn reference_window(
    kind: parametric_eq::EqBandKind,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
    rate: u32,
) -> Vec<f64> {
    let coefficients = ReferenceParametricEqCoefficients::design(
        reference_kind(kind),
        f64::from(rate),
        f64::from(frequency),
        f64::from(gain),
        f64::from(q),
        f64::from(slope),
    )
    .expect("independent frozen edge design");
    let mut section = ReferenceParametricEqSection::new(coefficients);
    (0..rate as usize)
        .map(|index| section.process(if index == 0 { 1.0 } else { 0.0 }))
        .collect()
}

/// E3: forty-eight one-second impulses agree with the finite-window `f64` reference.
#[test]
fn one_second_impulse_dfts_match_the_independent_oracle_at_all_frozen_edges() {
    let mut cases = 0_u32;
    let mut worst = 0.0_f64;
    for rate in LAUNCH_RATES {
        for kind in FROZEN_KINDS {
            for (frequency, gain, q, slope) in FROZEN_EDGES {
                let (impulse, recovered_left, recovered_right) =
                    one_second_impulse(kind, frequency, gain, q, slope, rate);
                assert_eq!(recovered_left, 0, "left recovery Fs={rate} {kind:?}");
                assert_eq!(recovered_right, 0, "right recovery Fs={rate} {kind:?}");
                assert!(
                    impulse.iter().copied().all(f32::is_finite),
                    "one-second output stayed finite Fs={rate} {kind:?} f={frequency}"
                );
                let reference = reference_window(kind, frequency, gain, q, slope, rate);
                let expected = dft_db(reference.into_iter(), rate, f64::from(frequency));
                let actual = impulse_dft_db(&impulse, rate, f64::from(frequency));
                if expected >= RESPONSE_FLOOR_DB {
                    let error = (actual - expected).abs();
                    worst = worst.max(error);
                    assert!(
                        error <= ONE_SECOND_DFT_TOLERANCE_DB,
                        "one-second DFT Fs={rate} {kind:?} f={frequency} gain={gain} Q={q} \
                         S={slope}: actual={actual} expected={expected} error={error}"
                    );
                } else {
                    assert!(actual.is_finite());
                }
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 48);
    eprintln!("issue-087 E3 cases=48 worst_dft_db={worst:.6e}");
}

/// E4: the exact row that "recovered" at sample 39,223 keeps its state out of the subnormal range.
///
/// This is issue #87 F3 as a gate. The flush is `andnot(|x| < 1e-20, x)` inside the kernel, so a
/// retained word is either exactly `+0.0` or at least `1e-20` in magnitude — never a subnormal, and
/// never a fault. `1e-20` is about `2^-66`, so the flush band strictly contains the band hardware
/// FTZ acts on and a browser's forced FTZ cannot change these bits.
#[test]
fn flush_keeps_decaying_state_out_of_the_subnormal_range() {
    let configured =
        single_section_values(parametric_eq::EqBandKind::Bell, 20_000.0, 24.0, 18.0, 1.0);
    let rate = 44_100;
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, rate))
        .expect("the row that used to recover must prepare");
    let mut left = vec![0.0_f32; rate as usize];
    let mut right = vec![0.0_f32; rate as usize];
    left[0] = 1.0;
    right[0] = 1.0;
    let mut recovered = 0_u64;
    let mut blocks = 0_u32;
    for first in (0..left.len()).step_by(128) {
        let end = (first + 128).min(left.len());
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[first..end],
                &mut right[first..end],
                None,
                first as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        recovered += report.nonfinite_left_blocks + report.nonfinite_right_blocks;
        let (_, left_state, right_state) = snapshot(effect.as_ref());
        for payload in [&left_state[..], &right_state[..]] {
            for band in 0..SECTIONS {
                for word in 0..2 {
                    let value = f32::from_bits(band_word(payload, band, word));
                    assert!(
                        value.to_bits() == 0.0_f32.to_bits() || value.abs() >= FLUSH_EPS,
                        "block {blocks} band {band} word {word}: retained {value:e} is subnormal"
                    );
                }
            }
        }
        blocks += 1;
    }
    assert_eq!(recovered, 0, "a decaying tail is not a fault");
    assert!(
        left.iter().copied().all(f32::is_finite),
        "output stayed finite"
    );
    assert_eq!(blocks, rate.div_ceil(128));
}

/// E4: forty-eight frozen million-sample sequences stay bounded, with no recovery and no flush hole.
#[test]
fn forty_eight_frozen_million_sample_sequences_remain_valid_without_recovery() {
    const STABILITY_SAMPLES: usize = 1_000_000;
    const SEED: u64 = 0x0000_0000_0012_e911;
    let mut sequences = 0_u32;
    for rate in LAUNCH_RATES {
        for kind in FROZEN_KINDS {
            for (frequency, gain, q, slope) in FROZEN_EDGES {
                let configured = single_section_values(kind, frequency, gain, q, slope);
                let mut effect = ParametricEqFactory
                    .prepare(request_at_rate(&configured, false, rate))
                    .expect("frozen stability design must prepare");
                let mut noise_state = SEED
                    ^ u64::from(rate)
                    ^ (u64::from(kind as u32) << 32)
                    ^ u64::from(frequency.to_bits());
                let mut left = [0.0_f32; 128];
                let mut right = [0.0_f32; 128];
                let mut first_sample = 0_usize;
                let mut recovered = 0_u64;
                let mut sanitized = 0_u64;
                while first_sample < STABILITY_SAMPLES {
                    let frames = (STABILITY_SAMPLES - first_sample).min(left.len());
                    for index in 0..frames {
                        if first_sample + index == 0 {
                            left[index] = 0.99;
                            right[index] = -0.99;
                        } else {
                            left[index] = support::deterministic_noise(&mut noise_state);
                            right[index] = support::deterministic_noise(&mut noise_state);
                        }
                    }
                    let report = effect.process(
                        EffectProcessBlock::new(
                            &mut left[..frames],
                            &mut right[..frames],
                            None,
                            first_sample as u64,
                            &[],
                            128,
                        )
                        .expect("million-sample block"),
                    );
                    recovered += report.nonfinite_left_blocks + report.nonfinite_right_blocks;
                    sanitized += report.sanitized_main_samples;
                    assert!(
                        left[..frames].iter().copied().all(f32::is_finite)
                            && right[..frames].iter().copied().all(f32::is_finite),
                        "finite output Fs={rate} {kind:?} f={frequency}"
                    );
                    first_sample += frames;
                }
                assert_eq!(recovered, 0, "recovery Fs={rate} {kind:?} f={frequency}");
                assert_eq!(sanitized, 0, "the EQ no longer sanitises its input");
                let (_, left_state, right_state) = snapshot(effect.as_ref());
                for payload in [&left_state[..], &right_state[..]] {
                    for band in 0..SECTIONS {
                        for word in 0..WORDS_PER_BAND {
                            let value = f32::from_bits(band_word(payload, band, word));
                            if word == 14 {
                                continue;
                            }
                            assert!(
                                value.is_finite(),
                                "finite retained state Fs={rate} {kind:?} band={band} word={word}"
                            );
                            if word < 2 {
                                assert!(
                                    value.to_bits() == 0.0_f32.to_bits()
                                        || value.abs() >= FLUSH_EPS,
                                    "subnormal retained state Fs={rate} {kind:?} band={band}"
                                );
                            }
                        }
                    }
                }
                sequences += 1;
            }
        }
    }
    assert_eq!(sequences, 48);
}

/// A stable design never diverges, so the boundary check must be provoked to be observed.
///
/// Feeding a non-finite sample is the only way in: the input stage sanitises the render path (D7),
/// this effect no longer does, and the check is what stops a bad block from propagating. It zeroes
/// the block, clears the integrators and counts **one block**, not one sample per lane.
#[test]
fn a_non_finite_input_block_is_zeroed_counted_once_and_leaves_the_next_block_clean() {
    let configured =
        single_section_values(parametric_eq::EqBandKind::HighPass, 1_000.0, 0.0, 1.0, 1.0);
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, 48_000))
        .expect("prepare");
    let mut left = [0.5_f32; 8];
    let mut right = [0.25_f32; 8];
    left[3] = f32::NAN;
    let report = effect
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"));
    assert_eq!(report.nonfinite_left_blocks, 1);
    assert_eq!(report.nonfinite_right_blocks, 0);
    assert!(left.iter().all(|sample| sample.to_bits() == 0));
    assert!(right.iter().any(|sample| sample.to_bits() != 0));
    let (_, left_state, _) = snapshot(effect.as_ref());
    for band in 0..SECTIONS {
        for word in 0..2 {
            assert_eq!(band_word(&left_state, band, word), 0);
        }
    }
    let mut next_left = [0.5_f32; 8];
    let mut next_right = [0.25_f32; 8];
    let report = effect.process(
        EffectProcessBlock::new(&mut next_left, &mut next_right, None, 8, &[], 128).expect("block"),
    );
    assert_eq!(report.nonfinite_left_blocks, 0);
    assert!(next_left.iter().all(|sample| sample.is_finite()));
}

/// Sanity: `SampleRateHz` is the design domain, and a non-launch rate is refused at prepare.
#[test]
fn a_non_launch_sample_rate_is_not_a_legal_design() {
    assert!(
        parametric_eq::design_svf(
            parametric_eq::EqBandKind::Bell,
            1_000.0,
            6.0,
            1.0,
            1.0,
            SampleRateHz(44_101),
        )
        .is_err()
    );
}
