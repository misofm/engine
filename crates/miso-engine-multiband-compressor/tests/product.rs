//! The product-level gates: descriptor and resources, the unity-gain path, bypass, band
//! isolation, automation and the transactional restore.

mod support;

use miso_engine_dsp_reference::ReferenceLr4Crossover;
use miso_engine_effect_contract::{
    BankWidth, EffectPrepareError, EffectQuality, LatencySamples, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectBankRequest, ResetKind,
};
use miso_engine_multiband_compressor::{
    MULTIBAND_COMPRESSOR_DESCRIPTOR_V1, MultibandCompressorFactory,
};
use support::{
    backend_for, new_sections, point, process, request, request_with, restore, snapshot, values,
    varied_values,
};

fn rms(values: &[f32]) -> f64 {
    (values
        .iter()
        .map(|value| f64::from(*value) * f64::from(*value))
        .sum::<f64>()
        / values.len() as f64)
        .sqrt()
}

/// Version 2's byte rows, the latency and the exact-and-one-below resource caps.
///
/// The rows changed **by decision**, not by drift: F1 removed the dry ring, F4 halved the filter
/// state and D11 added a step word to every ramp. Version 1 was `lane 4 * (43 + 3 * ring)`;
/// version 2 is `lane 4 * (48 + 2 * ring)`. The common section stays empty: wave-2 decision W2-D2
/// on #83 defers the shared codec's versioned header to #95.
#[test]
fn descriptor_preparation_and_exact_four_rate_resources_are_frozen() {
    miso_engine_effect_contract::validate_descriptor_v1(&MULTIBAND_COMPRESSOR_DESCRIPTOR_V1)
        .expect("descriptor");
    assert_eq!(MULTIBAND_COMPRESSOR_DESCRIPTOR_V1.parameters.len(), 12);
    assert_eq!(MULTIBAND_COMPRESSOR_DESCRIPTOR_V1.state_layout_version, 2);
    for (rate, bytes) in [
        (44_100u32, 7_256u32),
        (48_000, 7_880),
        (88_200, 14_312),
        (96_000, 15_560),
    ] {
        let initial = values();
        let mut prepared = request(&initial);
        prepared.sample_rate = rate;
        let effect = MultibandCompressorFactory
            .prepare(prepared)
            .expect("prepare");
        let metadata = effect.metadata();
        assert_eq!(metadata.latency, LatencySamples(u64::from(rate / 50)));
        assert_eq!(metadata.state_sizes.common_bytes, 0);
        assert_eq!(metadata.state_sizes.left_bytes, bytes);
        assert_eq!(metadata.state_sizes.right_bytes, bytes);
        assert_eq!(metadata.scratch_bytes, 0);
        let total = metadata.state_sizes.total().expect("total");
        assert_eq!(total, 2 * u64::from(bytes));
        let mut below = request(&initial);
        below.sample_rate = rate;
        below.limits.maximum_total_state_bytes = total - 1;
        assert_eq!(
            MultibandCompressorFactory.prepare(below).err(),
            Some(EffectPrepareError {
                code: "effect.resource.limit"
            })
        );
    }
}

/// E0. At unity gain the output has no step when a makeup ramp leaves and returns to zero.
///
/// A settled LR4 all-pass sine at 1 kHz can change by at most `2 * 0.5 * 1.001152 * sin(pi/48)`,
/// which is `0.065479`, per sample at 0.5 amplitude and +0.01 dB makeup. The 64-sample makeup ramp
/// adds under `9e-6` and `f32` rounding under `1e-6`, so `0.0656` is a conservative bound that
/// still rejects the version-1 dry/all-pass switch (measured step `0.8296956` before #94 F1).
#[test]
fn unity_gain_transition_has_no_step_at_crossover() {
    let mut initial = values();
    for lane in 0..2 {
        initial[2 * 2 + lane].value = 0.0;
        initial[7 * 2 + lane].value = 0.0;
    }
    let mut effect = MultibandCompressorFactory
        .prepare(request(&initial))
        .expect("unity-gain effect");
    let mut left = (0..12_288)
        .map(|index| 0.5 * (core::f32::consts::TAU * 1_000.0 * index as f32 / 48_000.0).sin())
        .collect::<Vec<_>>();
    let mut right = left.clone();
    let up = [
        point(6, ParameterChannel::Left, 5_120, 0.01),
        point(6, ParameterChannel::Right, 5_120, 0.01),
        point(11, ParameterChannel::Left, 5_120, 0.01),
        point(11, ParameterChannel::Right, 5_120, 0.01),
    ];
    let down = [
        point(6, ParameterChannel::Left, 7_680, 0.0),
        point(6, ParameterChannel::Right, 7_680, 0.0),
        point(11, ParameterChannel::Left, 7_680, 0.0),
        point(11, ParameterChannel::Right, 7_680, 0.0),
    ];
    for block in 0..96 {
        let start = block * 128;
        let spans: &[_] = match block {
            40 => &up,
            60 => &down,
            _ => &[],
        };
        let (left_block, right_block) = (
            &mut left[start..start + 128],
            &mut right[start..start + 128],
        );
        process(
            effect.as_mut(),
            left_block,
            right_block,
            start as u64,
            spans,
            128,
        );
    }
    let mut worst = 0.0f32;
    for index in 4_800..12_288 {
        let delta = (left[index] - left[index - 1]).abs();
        worst = worst.max(delta);
        assert!(
            delta <= 0.0656,
            "index={index} delta={delta} previous={} output={}",
            left[index - 1],
            left[index]
        );
    }
    eprintln!("E0 worst_consecutive_delta={worst:e}");
}

/// E0b. At unity gain the output is the delayed LR4 sum, against the independent `f64` oracle.
#[test]
fn unity_gain_output_is_the_delayed_lr4_sum() {
    let mut initial = values();
    for lane in 0..2 {
        initial[2 * 2 + lane].value = 0.0;
        initial[7 * 2 + lane].value = 0.0;
    }
    let input = (0..8_192)
        .map(|index| 0.5 * (core::f32::consts::TAU * 1_000.0 * index as f32 / 48_000.0).sin())
        .collect::<Vec<_>>();
    let mut left = input.clone();
    let mut right = input.clone();
    let mut effect = MultibandCompressorFactory
        .prepare(request(&initial))
        .expect("unity-gain effect");
    for block in 0..64 {
        let start = block * 128;
        let (left_block, right_block) = (
            &mut left[start..start + 128],
            &mut right[start..start + 128],
        );
        process(
            effect.as_mut(),
            left_block,
            right_block,
            start as u64,
            &[],
            128,
        );
    }
    let mut reference = ReferenceLr4Crossover::new(48_000.0, 1_000.0).expect("reference");
    let expected = input
        .iter()
        .map(|sample| {
            let (low, high) = reference.process_sample(f64::from(*sample));
            (low + high) as f32
        })
        .collect::<Vec<_>>();
    let mut worst = 0.0f32;
    for index in 4_096..8_192 {
        let error = (left[index] - expected[index - 960]).abs();
        worst = worst.max(error);
        assert!(
            error <= 2.0e-5,
            "index={index} error={error} actual={} expected={}",
            left[index],
            expected[index - 960]
        );
    }
    eprintln!("E0b worst_error={worst:e}");
}

/// E0c. A bypassed instance is a pure `Fs/50` delay, and signed zero survives it.
///
/// Signed zero lives on the bypass path and nowhere else: on the enabled path
/// `(+0.0) + (-0.0)` is `+0.0`, so the sum cannot preserve it, which is a property of addition and
/// not something to special-case (#94 F1 hazard).
#[test]
fn bypass_latency_automation_and_restore_are_transactional() {
    let initial = values();
    let mut effect = MultibandCompressorFactory
        .prepare(request_with(&initial, LinkMode::DualMono, 128, true))
        .expect("bypass");
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0.0; 128];
    let mut right = vec![0.0; 128];
    left[0] = -0.5;
    left[1] = -0.0;
    right[0] = 0.25;
    let span = [point(2, ParameterChannel::Left, 0, -80.0)];
    let mut output = Vec::new();
    for block in 0..8u64 {
        let spans: &[_] = if block == 0 { &span } else { &[] };
        process(
            effect.as_mut(),
            &mut left,
            &mut right,
            block * 128,
            spans,
            128,
        );
        output.extend_from_slice(&left);
        left.fill(0.0);
        right.fill(0.0);
    }
    assert!(output[..960].iter().all(|sample| *sample == 0.0));
    assert_eq!(output[960].to_bits(), (-0.5f32).to_bits());
    assert_eq!(output[961].to_bits(), (-0.0f32).to_bits());

    let saved = snapshot(effect.as_ref());
    let mut malformed = saved.clone();
    malformed.2[..4].fill(u8::MAX);
    assert!(restore(effect.as_mut(), 2, &malformed, sizes).is_err());
    assert_eq!(snapshot(effect.as_ref()), saved);
    // A version-1 payload is rejected on the out-of-band `state_layout_version` argument, which is
    // where the version lives until #95 adopts the shared codec's header (W2-D2).
    assert_eq!(
        restore(effect.as_mut(), 1, &saved, sizes)
            .expect_err("stale version")
            .code,
        "effect.state.version"
    );
    restore(effect.as_mut(), 2, &saved, sizes).expect("round trip");
    assert_eq!(snapshot(effect.as_ref()), saved);
}

/// A corrupted state word and an out-of-range track are rejected before anything is written.
#[test]
fn a_rejected_restore_changes_nothing() {
    let initial = values();
    let mut effect = MultibandCompressorFactory
        .prepare(request(&initial))
        .expect("prepare");
    let sizes = effect.metadata().state_sizes;
    let mut left = support::signal(128, 0x1234_5678);
    let mut right = support::signal(128, 0x8765_4321);
    process(effect.as_mut(), &mut left, &mut right, 0, &[], 128);
    let saved = snapshot(effect.as_ref());

    for (word, code) in [
        (0usize, "effect.state.parameter"), // crossover frequency
        (2, "effect.state.gain"),           // low-band smoother
        (4, "effect.state.parameter"),      // low threshold, current
        (44, "effect.state.filter"),        // first stage ic1
        (48, "effect.state.ring"),          // oldest low-ring sample
    ] {
        let mut corrupted = saved.clone();
        corrupted.1[word * 4..word * 4 + 4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        assert_eq!(
            restore(effect.as_mut(), 2, &corrupted, sizes)
                .expect_err("corrupt word")
                .code,
            code,
            "word {word}"
        );
        assert_eq!(snapshot(effect.as_ref()), saved, "word {word} left a trace");
    }

    // A resting ramp with a live step would have the segment driver add it for ever. The
    // invariant is `LinearRamp`'s; the restore enforces it rather than assuming it.
    let mut live_step = saved.clone();
    live_step.1[6 * 4..6 * 4 + 4].copy_from_slice(&0.5f32.to_bits().to_le_bytes());
    assert_eq!(
        restore(effect.as_mut(), 2, &live_step, sizes)
            .expect_err("a resting ramp cannot carry a step")
            .code,
        "effect.state.parameter"
    );
    assert_eq!(snapshot(effect.as_ref()), saved);

    let sets = (0..4).map(varied_values).collect::<Vec<_>>();
    let requests = sets.iter().map(|set| request(set)).collect::<Vec<_>>();
    let bank = support::bank(BankWidth::Four, &requests);
    let mut sections = new_sections(sizes);
    assert_eq!(
        bank.snapshot_track_state_payload(
            4,
            miso_engine_effect_contract::StatePayloadOutput::new(
                &mut sections.0,
                &mut sections.1,
                &mut sections.2,
                sizes
            )
            .expect("payload")
        )
        .expect_err("out of range")
        .code,
        "effect.state.track"
    );
}

/// Compressing one band leaves the other alone.
#[test]
fn isolated_low_and_high_band_compression_reduce_only_the_selected_band() {
    for (frequency, base) in [(120.0f32, 0usize), (4_000.0, 5)] {
        let mut active_values = values();
        let mut identity_values = values();
        for lane in 0..2 {
            active_values[(base + 2) * 2 + lane].value = -45.0;
            active_values[(base + 3) * 2 + lane].value = 20.0;
            active_values[(base + 4) * 2 + lane].value = 0.1;
            active_values[(base + 5) * 2 + lane].value = 5.0;
            identity_values[(base + 3) * 2 + lane].value = 1.0;
        }
        let mut active = MultibandCompressorFactory
            .prepare(request(&active_values))
            .expect("active");
        let mut identity = MultibandCompressorFactory
            .prepare(request(&identity_values))
            .expect("identity");
        let mut active_pcm = (0..3_072)
            .map(|index| 0.8 * (core::f32::consts::TAU * frequency * index as f32 / 48_000.0).sin())
            .collect::<Vec<_>>();
        let mut identity_pcm = active_pcm.clone();
        let mut active_right = active_pcm.clone();
        let mut identity_right = identity_pcm.clone();
        for block in 0..24 {
            let start = block * 128;
            process(
                active.as_mut(),
                &mut active_pcm[start..start + 128],
                &mut active_right[start..start + 128],
                start as u64,
                &[],
                128,
            );
            process(
                identity.as_mut(),
                &mut identity_pcm[start..start + 128],
                &mut identity_right[start..start + 128],
                start as u64,
                &[],
                128,
            );
        }
        assert!(
            rms(&active_pcm[1_600..]) < rms(&identity_pcm[1_600..]) * 0.9,
            "frequency={frequency}"
        );
    }
}

/// Every bank request is validated before any fallback, and the widths bind the right lane type.
#[test]
fn bank_requests_are_validated_before_any_fallback() {
    let factory = MultibandCompressorFactory;
    let sets = vec![values(); 4];
    let requests = sets.iter().map(|set| request(set)).collect::<Vec<_>>();
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: backend_for(BankWidth::Four),
                width: BankWidth::Eight,
                requests: &requests,
            })
            .err(),
        Some(EffectPrepareError {
            code: "effect.bank.requests"
        })
    );

    let mut malformed_sets = sets.clone();
    malformed_sets[3][0].value = f32::NAN;
    let malformed = malformed_sets
        .iter()
        .map(|set| request(set))
        .collect::<Vec<_>>();
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: backend_for(BankWidth::Four),
                width: BankWidth::Four,
                requests: &malformed,
            })
            .err(),
        Some(EffectPrepareError {
            code: "effect.parameter.initial"
        }),
        "every request is validated before an unavailable-backend fallback"
    );

    // A track whose program key differs falls back to scalar rather than silently binding.
    let mut other = sets.clone();
    let mut mixed = other.iter().map(|set| request(set)).collect::<Vec<_>>();
    mixed[2] = request_with(&other[2], LinkMode::Maximum, 128, false);
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: backend_for(BankWidth::Four),
                width: BankWidth::Four,
                requests: &mixed,
            })
            .expect("legal request")
            .is_none()
    );
    other[0][0].value = 1_000.0;

    for width in [BankWidth::Four, BankWidth::Eight] {
        let lanes = width.lanes() as usize;
        let sets = (0..lanes).map(varied_values).collect::<Vec<_>>();
        let requests = sets.iter().map(|set| request(set)).collect::<Vec<_>>();
        let mut bank = support::bank(width, &requests);
        assert_eq!(bank.metadata().width, width);
        bank.reset(ResetKind::FullToDefaults);
        bank.reset(ResetKind::DiscontinuityKeepParameters);
    }

    let quality = MULTIBAND_COMPRESSOR_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("48 kHz quality");
    assert_eq!(quality.quality, EffectQuality::Normal);
    let per_track = quality
        .maximum_state
        .total()
        .expect("state bytes")
        .checked_add(quality.scratch_fixed_bytes)
        .expect("prepared bytes");
    assert_eq!(per_track, 15_760);
    assert_eq!(per_track * 4, 63_040);
    assert_eq!(per_track * 8, 126_080);
}
