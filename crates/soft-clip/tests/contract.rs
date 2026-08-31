//! The frozen product contract: descriptor, latency, tail, alias claim and the f64 oracle bound.
//!
//! These are the assertions the launch briefs (`019`, `053`) own. None of them was allowed to move
//! in the issue-#91 re-landing except the state-payload sizes, which the plan bumps to layout
//! version 2 through the contract's own mechanism.

mod support;

use dsp_reference::{ReferenceSoftClip, reference_cubic_soft_clip, reference_halfband_63};
use effect_contract::{
    BankWidth, EffectPrepareError, LatencySamples, LinkModeSet, NativeEffectFactory,
    ParameterChannel, PrepareEffectBankRequest, ProcessReport, TailSamples, validate_descriptor,
};
use lane::kernels::halfband::{HALFBAND63_CENTER, HALFBAND63_EVEN};
use soft_clip::{SOFT_CLIP_DESCRIPTOR, SoftClipFactory};
use support::{PARAMETERS, bank_available, initial_values, prepare, process, request, values_from};

/// E11 — resources, latency, tail and the tap table against the independent `f64` design.
#[test]
fn descriptor_resources_and_independent_fir_design_are_frozen() {
    validate_descriptor(&SOFT_CLIP_DESCRIPTOR).expect("descriptor");
    assert_eq!(
        SOFT_CLIP_DESCRIPTOR.supported_link_modes,
        LinkModeSet::DUAL_MONO
    );
    assert_eq!(SOFT_CLIP_DESCRIPTOR.parameters.len(), PARAMETERS);
    assert_eq!(SOFT_CLIP_DESCRIPTOR.state_layout_version, 2);
    for quality in SOFT_CLIP_DESCRIPTOR.qualities {
        assert_eq!(quality.latency, LatencySamples(31));
        assert_eq!(quality.tail, TailSamples::Finite(29));
        // Layout 2: 104 effect words per channel, plus the shared codec's two header words.
        assert_eq!(quality.maximum_state.common_bytes, 8);
        assert_eq!(quality.maximum_state.left_bytes, 416);
        assert_eq!(quality.maximum_state.right_bytes, 416);
        assert_eq!(quality.maximum_state.total(), Some(840));
        assert_eq!(quality.scratch_fixed_bytes, 24);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
    }

    // The only surviving copy of the tap values is the lane crate's polyphase table; it must be
    // the even taps of the independent f64 half-band design, bit for bit.
    let reference = reference_halfband_63();
    for (index, actual) in HALFBAND63_EVEN.into_iter().enumerate() {
        let expected = reference[2 * (index + 1)] as f32;
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "h[{}]",
            2 * (index + 1)
        );
    }
    assert_eq!(
        HALFBAND63_CENTER.to_bits(),
        (reference[31] as f32).to_bits()
    );

    let values = initial_values();
    let mut too_small = request(&values);
    too_small.limits.maximum_total_state_bytes = 839;
    assert!(matches!(
        SoftClipFactory.prepare(too_small),
        Err(EffectPrepareError {
            code: "effect.resource.limit"
        })
    ));
}

/// E6 — the rendered signal against the independent `f64` oracle, after the warmup.
///
/// The bound is the frozen `3.0e-6`. It is unchanged: this is a class-B check on the whole chain
/// including the D6 decibel conversion, whose last bits differ from `powf` by design.
#[test]
fn scalar_matches_independent_oracle_after_warmup() {
    let values = values_from([(18.0, 18.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut effect = prepare(&values);
    let mut oracle = ReferenceSoftClip::new(18.0, 0.0, 1.0).expect("oracle");
    let mut input = (0..128)
        .map(|index| (index as f32 * 0.073).sin() * 0.8)
        .collect::<Vec<_>>();
    let expected = input
        .iter()
        .map(|value| oracle.process(*value as f64) as f32)
        .collect::<Vec<_>>();
    let mut right = input.clone();
    process(effect.as_mut(), &mut input, &mut right, 0, &[]);
    let mut worst = 0.0_f32;
    for (actual, expected) in input.into_iter().zip(expected).skip(64) {
        worst = worst.max((actual - expected).abs());
        assert!(
            (actual - expected).abs() <= 3.0e-6,
            "actual={actual:?}, expected={expected:?}"
        );
    }
    println!("issue_091_oracle worst_deviation={worst:e}");
}

/// E5 — the frozen alias row. Text unchanged from the launch qualification.
#[test]
fn frozen_alias_claim_improves_over_independent_naive_cubic() {
    const LENGTH: usize = 16_384;
    const FUNDAMENTAL_BIN: usize = 3_001;
    const WARM_PERIODS: usize = 3;
    const BLOCK: usize = 128;

    let values = values_from([(18.0, 18.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut effect = prepare(&values);
    let mut fixed_2x = Vec::with_capacity(LENGTH);
    for block_start in (0..((WARM_PERIODS + 1) * LENGTH)).step_by(BLOCK) {
        let mut left = [0.0_f32; BLOCK];
        for (offset, sample) in left.iter_mut().enumerate() {
            let index = (block_start + offset) % LENGTH;
            let phase =
                core::f64::consts::TAU * FUNDAMENTAL_BIN as f64 * index as f64 / LENGTH as f64;
            *sample = phase.sin() as f32;
        }
        let mut right = left;
        let report = process(
            effect.as_mut(),
            &mut left,
            &mut right,
            block_start as u64,
            &[],
        );
        assert_eq!(report, ProcessReport::default());
        assert_eq!(left.map(f32::to_bits), right.map(f32::to_bits));
        if block_start >= WARM_PERIODS * LENGTH {
            fixed_2x.extend(left.into_iter().map(f64::from));
        }
    }
    assert_eq!(fixed_2x.len(), LENGTH);

    let drive = 10.0_f64.powf(18.0 * 0.05);
    let naive_1x = (0..LENGTH)
        .map(|index| {
            let phase =
                core::f64::consts::TAU * FUNDAMENTAL_BIN as f64 * index as f64 / LENGTH as f64;
            reference_cubic_soft_clip(drive * phase.sin())
        })
        .collect::<Vec<_>>();
    let fixed_2x_ratio_db = rectangular_nonfundamental_ratio_db(&fixed_2x, FUNDAMENTAL_BIN);
    let naive_1x_ratio_db = rectangular_nonfundamental_ratio_db(&naive_1x, FUNDAMENTAL_BIN);
    let improvement_db = naive_1x_ratio_db - fixed_2x_ratio_db;
    println!(
        "issue_053_alias fixed_2x_nonfundamental_ratio_db={fixed_2x_ratio_db:.12} \
         naive_1x_nonfundamental_ratio_db={naive_1x_ratio_db:.12} \
         improvement_db={improvement_db:.12}"
    );
    assert!(fixed_2x_ratio_db.is_finite());
    assert!(naive_1x_ratio_db.is_finite());
    assert!(
        improvement_db >= 2.0,
        "fixed-2x improvement {improvement_db:.12} dB is below 2.0 dB"
    );
}

/// Latency 31, support through sample 60, silence after it.
#[test]
fn wet_impulse_has_exact_group_delay_and_final_causal_support() {
    let values = initial_values();
    let mut effect = prepare(&values);
    let mut left = vec![0.0; 128];
    let mut right = vec![0.0; 128];
    left[0] = 0.001;
    right[0] = -0.001;
    process(effect.as_mut(), &mut left, &mut right, 0, &[]);
    let peak = |samples: &[f32]| {
        samples
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
            .map(|(index, _)| index)
            .expect("nonempty impulse")
    };
    assert_eq!(peak(&left), 31);
    assert_eq!(peak(&right), 31);
    assert_ne!(left[60].to_bits(), 0.0_f32.to_bits());
    assert_ne!(right[60].to_bits(), 0.0_f32.to_bits());
    assert!(left[61..].iter().all(|sample| *sample == 0.0));
    assert!(right[61..].iter().all(|sample| *sample == 0.0));
}

/// A bypassed or identity instance emits the dry signal delayed by exactly 31 samples, with the
/// input's signed zero preserved, and keeps the wet histories warm (brief 053).
#[test]
fn identity_and_bypass_emit_the_delayed_dry_signal_bit_for_bit() {
    for identity in [true, false] {
        let values = if identity {
            values_from([(12.0, 12.0), (0.0, 0.0), (0.0, 0.0)])
        } else {
            values_from([(12.0, 12.0), (0.0, 0.0), (1.0, 1.0)])
        };
        let mut request = request(&values);
        request.bypass = !identity;
        let mut effect = SoftClipFactory.prepare(request).expect("prepare");
        let mut left = vec![0.0_f32; 128];
        let mut right = vec![0.0_f32; 128];
        left[0] = 0.25;
        left[31] = -0.0;
        right[0] = -0.5;
        process(effect.as_mut(), &mut left, &mut right, 0, &[]);
        assert_eq!(left[31].to_bits(), 0.25_f32.to_bits());
        assert_eq!(right[31].to_bits(), (-0.5_f32).to_bits());
        assert_eq!(
            left[62].to_bits(),
            (-0.0_f32).to_bits(),
            "signed zero survives"
        );
        assert!(left[..31].iter().all(|sample| sample.to_bits() == 0));
    }
}

/// Bank binding validates before it declines, and declines rather than failing where the width is
/// not this artifact's.
#[test]
fn bank_binding_validates_before_declining_an_unavailable_width() {
    let values = initial_values();
    let requests = [request(&values); 8];
    // Wrong request count for the width.
    assert!(matches!(
        SoftClipFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: support::backend(BankWidth::Eight),
            width: BankWidth::Eight,
            requests: &requests[..7],
        }),
        Err(EffectPrepareError {
            code: "effect.bank.requests"
        })
    ));
    // Backend and width disagree.
    assert!(matches!(
        SoftClipFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: support::backend(BankWidth::Four),
            width: BankWidth::Eight,
            requests: &requests,
        }),
        Err(EffectPrepareError {
            code: "effect.bank.requests"
        })
    ));
    // A mixed cohort is declined, not rejected.
    let other = values_from([(6.0, 6.0), (0.0, 0.0), (1.0, 1.0)]);
    let mut mixed = [request(&values); 8];
    mixed[4] = request(&other);
    mixed[4].quantum = 64;
    assert!(
        SoftClipFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: support::backend(BankWidth::Eight),
                width: BankWidth::Eight,
                requests: &mixed,
            })
            .expect("mixed cohort declines")
            .is_none()
    );
    // The host's own width binds.
    let bound = SoftClipFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: support::backend(BankWidth::Eight),
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("bind");
    assert_eq!(bound.is_some(), bank_available(BankWidth::Eight));
}

/// Automation validation is unchanged: canonical ordered points only.
#[test]
fn automation_rejects_everything_but_canonical_ordered_points() {
    let values = initial_values();
    let mut effect = prepare(&values);
    let mut left = vec![0.0_f32; 8];
    let mut right = vec![0.0_f32; 8];
    let mut both = support::point(0, ParameterChannel::Both, 6.0, 0);
    let mut ranged = support::point(1, ParameterChannel::Left, 6.0, 0);
    ranged.end_sample = 4;
    let mut out_of_domain = support::point(1, ParameterChannel::Right, 96.0, 0);
    out_of_domain.end_value = 96.0;
    let disordered = [
        support::point(2, ParameterChannel::Left, 0.5, 0),
        support::point(0, ParameterChannel::Left, 6.0, 0),
    ];
    both.end_value = 6.0;
    ranged.end_value = 6.0;
    let spans = [both, ranged, out_of_domain, disordered[0], disordered[1]];
    let report = process(effect.as_mut(), &mut left, &mut right, 0, &spans);
    assert_eq!(report.invalid_spans, 4);
    assert_eq!(report.sanitized_main_samples, 0);
}

/// Rectangular-window non-fundamental to fundamental energy ratio, in decibels.
fn rectangular_nonfundamental_ratio_db(samples: &[f64], fundamental_bin: usize) -> f64 {
    assert!(fundamental_bin != 0 && fundamental_bin < samples.len() / 2);
    let length = samples.len() as f64;
    let mut time_energy = 0.0_f64;
    let mut dc = 0.0_f64;
    let mut fundamental_re = 0.0_f64;
    let mut fundamental_im = 0.0_f64;
    for (index, sample) in samples.iter().copied().enumerate() {
        let phase = -core::f64::consts::TAU * fundamental_bin as f64 * index as f64 / length;
        time_energy += sample * sample;
        dc += sample;
        fundamental_re += sample * phase.cos();
        fundamental_im += sample * phase.sin();
    }
    let total_dft_energy = length * time_energy;
    let dc_energy = dc * dc;
    let fundamental_energy =
        2.0 * (fundamental_re * fundamental_re + fundamental_im * fundamental_im);
    let nonfundamental_energy = (total_dft_energy - dc_energy - fundamental_energy).max(0.0);
    assert!(fundamental_energy.is_finite() && fundamental_energy > 0.0);
    assert!(nonfundamental_energy.is_finite() && nonfundamental_energy > 0.0);
    10.0 * (nonfundamental_energy / fundamental_energy).log10()
}
