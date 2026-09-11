#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The product-level gates: descriptor and resources, the unity-gain path, bypass, band
//! isolation, automation and the transactional restore.

mod support;

use dsp_reference::ReferenceLr4Crossover;
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectPrepareError, EffectQuality, LinkMode,
    NativeEffectFactory, ObservationSample, ParameterChannel, PrepareEffectBankRequest, ResetKind,
    StatePayloadInput, StatePayloadSizes,
};
use multiband_compressor::{MULTIBAND_COMPRESSOR_DESCRIPTOR, MultibandCompressorFactory};
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

fn active_values() -> [effect_contract::InitialParameterValue; support::PARAMETER_COUNT * 2] {
    let mut values = values();
    for channel in 0..2 {
        values[2 + channel].value = -45.0;
        values[4 + channel].value = 20.0;
        values[6 + channel].value = 0.1;
        values[8 + channel].value = 5.0;
        values[12 + channel].value = -45.0;
        values[14 + channel].value = 20.0;
        values[16 + channel].value = 0.1;
        values[18 + channel].value = 5.0;
    }
    values
}

/// The current byte rows, zero latency and exact-and-one-below resource caps.
///
/// The compact causal payload has one crossover word, two gain words, ten four-word ramps and four
/// filter words per channel. The common section stays empty: wave-2 decision W2-D2 on #83 defers
/// the shared codec's versioned header to #95.
#[test]
fn descriptor_preparation_and_exact_four_rate_resources_are_frozen() {
    effect_contract::validate_descriptor(&MULTIBAND_COMPRESSOR_DESCRIPTOR).expect("descriptor");
    assert_eq!(MULTIBAND_COMPRESSOR_DESCRIPTOR.parameters.len(), 11);
    assert_eq!(
        MULTIBAND_COMPRESSOR_DESCRIPTOR
            .parameters
            .iter()
            .map(|parameter| parameter.id.0)
            .collect::<Vec<_>>(),
        [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    );
    assert_eq!(MULTIBAND_COMPRESSOR_DESCRIPTOR.state_layout_version, 1);
    for (rate, bytes) in [
        (44_100u32, 188u32),
        (48_000, 188),
        (88_200, 188),
        (96_000, 188),
    ] {
        let initial = values();
        let mut prepared = request(&initial);
        prepared.sample_rate = rate;
        prepared.limits.maximum_total_state_bytes = 376;
        // The contract requires a positive capacity limit even when the prepared effect needs no
        // scratch; one byte is the smallest admissible declaration for the exact zero-byte row.
        prepared.limits.maximum_scratch_bytes = 1;
        let mut effect = MultibandCompressorFactory
            .prepare(prepared)
            .expect("prepare");
        let metadata = effect.metadata();
        assert_eq!(metadata.latency, effect_contract::LatencySamples(0));
        assert_eq!(metadata.state_sizes.common_bytes, 0);
        assert_eq!(metadata.state_sizes.left_bytes, bytes);
        assert_eq!(metadata.state_sizes.right_bytes, bytes);
        assert_eq!(metadata.scratch_bytes, 0);
        let total = metadata.state_sizes.total().expect("total");
        assert_eq!(total, 2 * u64::from(bytes));
        let old_bytes = match rate {
            44_100 => 7_256,
            48_000 => 7_880,
            88_200 => 14_312,
            96_000 => 15_560,
            _ => unreachable!(),
        };
        let old_left = vec![0u8; old_bytes];
        let old_right = vec![0u8; old_bytes];
        assert!(
            StatePayloadInput::new(&[], &old_left, &old_right, metadata.state_sizes).is_err(),
            "retired rate-dependent payload must not enter the compact codec at {rate} Hz"
        );
        let saved = {
            let mut left = support::signal(128, u64::from(rate));
            let mut right = support::signal(128, u64::from(rate) + 1);
            process(effect.as_mut(), &mut left, &mut right, 0, &[], 128);
            snapshot(effect.as_ref())
        };
        let old_sizes = StatePayloadSizes {
            common_bytes: 0,
            left_bytes: old_bytes as u32,
            right_bytes: old_bytes as u32,
        };
        let old_payload = StatePayloadInput::new(&[], &old_left, &old_right, old_sizes)
            .expect("historical payload shape");
        assert!(
            effect.restore_state_payload(1, old_payload).is_err(),
            "historical payload must be rejected by the scalar restore hook at {rate} Hz"
        );
        assert_eq!(snapshot(effect.as_ref()), saved);

        let mut bypass_request = request_with(&initial, LinkMode::DualMono, 128, true);
        bypass_request.sample_rate = rate;
        bypass_request.limits.maximum_total_state_bytes = total;
        bypass_request.limits.maximum_scratch_bytes = 1;
        let mut bypass = MultibandCompressorFactory
            .prepare(bypass_request)
            .expect("bypass prepare");
        assert_eq!(
            bypass.metadata().latency,
            effect_contract::LatencySamples(0)
        );
        assert_eq!(bypass.metadata().state_sizes, metadata.state_sizes);
        let mut bypass_left = support::signal(128, u64::from(rate) + 2);
        let mut bypass_right = support::signal(128, u64::from(rate) + 3);
        let input_left = bypass_left.clone();
        let input_right = bypass_right.clone();
        process(
            bypass.as_mut(),
            &mut bypass_left,
            &mut bypass_right,
            0,
            &[],
            128,
        );
        assert_eq!(bypass_left, input_left);
        assert_eq!(bypass_right, input_right);

        for width in [BankWidth::Four, BankWidth::Eight] {
            let lanes = width.lanes() as usize;
            let bank_values = vec![initial; lanes];
            let mut bank_requests = bank_values
                .iter()
                .map(|values| request(values))
                .collect::<Vec<_>>();
            for request in &mut bank_requests {
                request.sample_rate = rate;
                request.limits.maximum_total_state_bytes = total;
                request.limits.maximum_scratch_bytes = 1;
            }
            let mut bank = MultibandCompressorFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: backend_for(width),
                    width,
                    requests: &bank_requests,
                })
                .expect("bank prepare")
                .expect("bank");
            assert_eq!(bank.metadata().width, width);
            assert_eq!(
                bank.metadata().program_key.state_sizes,
                metadata.state_sizes
            );
            assert_eq!(
                bank.metadata().program_key.latency,
                effect_contract::LatencySamples(0)
            );
            let mut bank_left = vec![0.25f32; 128 * lanes];
            let mut bank_right = vec![-0.25f32; 128 * lanes];
            let offsets = vec![0u32; lanes + 1];
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    128,
                    width,
                    0,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            assert!(bank_left.iter().all(|sample| sample.is_finite()));
            assert!(bank_right.iter().all(|sample| sample.is_finite()));

            let mut bypass_bank_requests = (0..lanes)
                .map(|_| request_with(&initial, LinkMode::DualMono, 128, true))
                .collect::<Vec<_>>();
            for request in &mut bypass_bank_requests {
                request.sample_rate = rate;
                request.limits.maximum_total_state_bytes = total;
                request.limits.maximum_scratch_bytes = 1;
            }
            let mut bypass_bank = MultibandCompressorFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: backend_for(width),
                    width,
                    requests: &bypass_bank_requests,
                })
                .expect("bypass bank prepare")
                .expect("bypass bank");
            assert_eq!(
                bypass_bank.metadata().program_key.latency,
                effect_contract::LatencySamples(0)
            );
            let mut bypass_bank_left = vec![0.0f32; 128 * lanes];
            let mut bypass_bank_right = vec![0.0f32; 128 * lanes];
            for frame in 0..128 {
                for lane in 0..lanes {
                    let index = frame * lanes + lane;
                    bypass_bank_left[index] = if (frame + lane) % 3 == 0 {
                        -0.0
                    } else {
                        0.125 * (lane as f32 + 1.0)
                    };
                    bypass_bank_right[index] = if (frame + lane) % 4 == 0 {
                        -0.0
                    } else {
                        -0.25 * (lane as f32 + 1.0)
                    };
                }
            }
            let bypass_bank_left_input = bypass_bank_left.clone();
            let bypass_bank_right_input = bypass_bank_right.clone();
            bypass_bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bypass_bank_left,
                    &mut bypass_bank_right,
                    None,
                    128,
                    width,
                    0,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bypass bank block"),
            );
            assert!(
                bypass_bank_left
                    .iter()
                    .zip(bypass_bank_left_input.iter())
                    .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
            );
            assert!(
                bypass_bank_right
                    .iter()
                    .zip(bypass_bank_right_input.iter())
                    .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
            );
        }
        let mut below = request(&initial);
        below.sample_rate = rate;
        below.limits.maximum_total_state_bytes = total - 1;
        below.limits.maximum_scratch_bytes = 1;
        assert_eq!(
            MultibandCompressorFactory.prepare(below).err(),
            Some(EffectPrepareError {
                code: "effect.resource.limit"
            })
        );
        let mut below_bank_requests = (0..BankWidth::Four.lanes() as usize)
            .map(|_| request(&initial))
            .collect::<Vec<_>>();
        for request in &mut below_bank_requests {
            request.sample_rate = rate;
            request.limits.maximum_total_state_bytes = total - 1;
            request.limits.maximum_scratch_bytes = 1;
        }
        assert_eq!(
            MultibandCompressorFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: backend_for(BankWidth::Four),
                    width: BankWidth::Four,
                    requests: &below_bank_requests,
                })
                .err(),
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
        initial[2 + lane].value = 0.0;
        initial[6 * 2 + lane].value = 0.0;
    }
    let mut effect = MultibandCompressorFactory
        .prepare(request(&initial))
        .expect("unity-gain effect");
    let mut left = (0..12_288)
        .map(|index| 0.5 * (core::f32::consts::TAU * 1_000.0 * index as f32 / 48_000.0).sin())
        .collect::<Vec<_>>();
    let mut right = left.clone();
    let up = [
        point(5, ParameterChannel::Left, 5_120, 0.01),
        point(5, ParameterChannel::Right, 5_120, 0.01),
        point(10, ParameterChannel::Left, 5_120, 0.01),
        point(10, ParameterChannel::Right, 5_120, 0.01),
    ];
    let down = [
        point(5, ParameterChannel::Left, 7_680, 0.0),
        point(5, ParameterChannel::Right, 7_680, 0.0),
        point(10, ParameterChannel::Left, 7_680, 0.0),
        point(10, ParameterChannel::Right, 7_680, 0.0),
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

/// E0b. At unity gain the causal output is the LR4 sum, against the independent `f64` oracle.
#[test]
fn unity_gain_output_is_the_causal_lr4_sum() {
    let mut initial = values();
    for lane in 0..2 {
        initial[2 + lane].value = 0.0;
        initial[6 * 2 + lane].value = 0.0;
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
    for index in 0..8_192 {
        let error = (left[index] - expected[index]).abs();
        worst = worst.max(error);
        assert!(
            error <= 2.0e-5,
            "index={index} error={error} actual={} expected={}",
            left[index],
            expected[index]
        );
    }
    eprintln!("E0b worst_error={worst:e}");

    for quantum in [1u32, 128] {
        let input = (0..256)
            .map(|index| {
                if index == 0 {
                    0.37
                } else if index % 5 == 0 {
                    -0.21
                } else if index % 3 == 0 {
                    0.13
                } else {
                    0.01 * (index % 17) as f32 - 0.08
                }
            })
            .collect::<Vec<_>>();
        let mut actual = input.clone();
        let mut actual_right = input.clone();
        let mut effect = MultibandCompressorFactory
            .prepare(request_with(&initial, LinkMode::DualMono, quantum, false))
            .expect("irregular unity effect");
        let mut first_sample = 0u64;
        while first_sample < input.len() as u64 {
            let frames = (input.len() as u64 - first_sample).min(u64::from(quantum)) as usize;
            let start = first_sample as usize;
            process(
                effect.as_mut(),
                &mut actual[start..start + frames],
                &mut actual_right[start..start + frames],
                first_sample,
                &[],
                quantum,
            );
            first_sample += frames as u64;
        }
        let mut reference = ReferenceLr4Crossover::new(48_000.0, 1_000.0).expect("reference");
        for (index, sample) in input.iter().enumerate() {
            let (low, high) = reference.process_sample(f64::from(*sample));
            let expected = (low + high) as f32;
            assert!(
                (actual[index] - expected).abs() <= 2.0e-5,
                "quantum={quantum} frame={index}"
            );
            assert!(
                (actual_right[index] - expected).abs() <= 2.0e-5,
                "quantum={quantum} right frame={index}"
            );
        }
    }
}

/// An independent LR4 plus mathematical dynamics oracle exercises both active envelopes at the
/// current sample. Its resident negative-dB reading is checked at each block boundary.
#[test]
fn active_causal_oracle_engages_releases_and_starts_on_current_sample() {
    const FRAMES: usize = 4_096;
    let active = active_values();
    let mut effect = MultibandCompressorFactory
        .prepare(request_with(&active, LinkMode::DualMono, 128, false))
        .expect("active effect");
    let mut oracle = support::oracle::ActiveBands::new(
        48_000.0, 1_000.0, -45.0, 20.0, 0.1, 5.0, -45.0, 20.0, 0.1, 5.0,
    );
    let input = (0..FRAMES)
        .map(|index| {
            let frequency = if index < 1_024 {
                120.0
            } else if (2_048..3_072).contains(&index) {
                4_000.0
            } else {
                0.0
            };
            if frequency == 0.0 {
                0.0
            } else {
                0.8 * (core::f32::consts::TAU * frequency * index as f32 / 48_000.0).sin()
            }
        })
        .collect::<Vec<_>>();
    let mut output = input.clone();
    let mut right = input.clone();
    let mut expected = Vec::with_capacity(FRAMES);
    let mut low_envelope = Vec::with_capacity(FRAMES);
    let mut high_envelope = Vec::with_capacity(FRAMES);
    let state_word = |payload: &[u8], word: usize| -> f64 {
        f64::from(f32::from_le_bytes(
            payload[word * 4..word * 4 + 4]
                .try_into()
                .expect("state word"),
        ))
    };
    for start in (0..FRAMES).step_by(128) {
        let end = start + 128;
        for sample in &input[start..end] {
            let (value, low, high) = oracle.process(*sample);
            expected.push(value);
            low_envelope.push(low);
            high_envelope.push(high);
        }
        process(
            effect.as_mut(),
            &mut output[start..end],
            &mut right[start..end],
            start as u64,
            &[],
            128,
        );
        let mut observed = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut observed));
        let oracle_reduction = low_envelope[end - 1].min(high_envelope[end - 1]);
        assert!(
            (f64::from(observed.left) - oracle_reduction).abs() <= 0.005,
            "block ending {end}: observed={} oracle={oracle_reduction}",
            observed.left
        );
        let saved = snapshot(effect.as_ref());
        for (channel, payload) in [("left", &saved.1), ("right", &saved.2)] {
            let low = state_word(payload, 1);
            let high = state_word(payload, 2);
            assert!(
                (low - low_envelope[end - 1]).abs() <= 0.005,
                "block ending {end} {channel} low gain: state={low} oracle={}",
                low_envelope[end - 1]
            );
            assert!(
                (high - high_envelope[end - 1]).abs() <= 0.005,
                "block ending {end} {channel} high gain: state={high} oracle={}",
                high_envelope[end - 1]
            );
        }
    }
    let mut worst = 0.0f32;
    for (index, (actual, wanted)) in output.iter().zip(expected.iter()).enumerate() {
        let error = (*actual - *wanted).abs();
        worst = worst.max(error);
        assert!(error <= 2.0e-5, "frame={index} error={error}");
    }
    let low_min = low_envelope[..1_024].iter().copied().fold(0.0, f64::min);
    let high_min = high_envelope[2_048..3_072]
        .iter()
        .copied()
        .fold(0.0, f64::min);
    assert!(low_min < -5.0, "low envelope never engaged: {low_min}");
    assert!(high_min < -5.0, "high envelope never engaged: {high_min}");
    assert!(
        low_envelope[2_047] > low_min + 1.0,
        "low envelope did not release"
    );
    assert!(
        high_envelope[4_095] > high_min + 1.0,
        "high envelope did not release"
    );
    let low_burst_rms = rms(&expected[..1_024]);
    let high_burst_rms = rms(&expected[2_048..3_072]);
    assert!(
        (low_burst_rms - high_burst_rms).abs() > 1.0e-4,
        "low/high burst outputs were indistinguishable: low={low_burst_rms} high={high_burst_rms}"
    );
    eprintln!("active oracle worst_error={worst:e}");

    let mut first_values = values();
    for channel in 0..2 {
        first_values[4 + channel].value = 1.0;
        first_values[12 + channel].value = -80.0;
        first_values[14 + channel].value = 20.0;
        first_values[16 + channel].value = 0.1;
        first_values[18 + channel].value = 5.0;
    }
    let mut first_effect = MultibandCompressorFactory
        .prepare(request_with(&first_values, LinkMode::DualMono, 1, false))
        .expect("first-sample effect");
    let mut first_oracle = support::oracle::ActiveBands::new(
        48_000.0, 1_000.0, -18.0, 1.0, 10.0, 100.0, -80.0, 20.0, 0.1, 5.0,
    );
    let mut first = [1.0f32];
    let mut first_right = [1.0f32];
    let (wanted, _, high_state) = first_oracle.process(1.0);
    process(
        first_effect.as_mut(),
        &mut first,
        &mut first_right,
        0,
        &[],
        1,
    );
    assert!(
        high_state < -1.0,
        "first sample high envelope did not engage"
    );
    assert!((first[0] - wanted).abs() <= 2.0e-5);
    assert!(first[0].abs() < 0.99, "first sample was not reduced");

    let probe = (0..128)
        .map(|index| match index % 5 {
            0 => 0.7,
            1 => -0.2,
            2 => 0.3,
            3 => 0.05,
            _ => -0.4,
        })
        .collect::<Vec<_>>();
    let mut reference_a = support::oracle::ActiveBands::new(
        48_000.0, 1_000.0, -18.0, 20.0, 0.1, 5.0, -45.0, 20.0, 0.1, 5.0,
    );
    let mut reference_b = support::oracle::ActiveBands::new(
        48_000.0, 1_000.0, -17.0, 20.0, 0.1, 5.0, -45.0, 20.0, 0.1, 5.0,
    );
    let sensitivity = probe
        .iter()
        .map(|sample| {
            let (a, _, _) = reference_a.process(*sample);
            let (b, _, _) = reference_b.process(*sample);
            (a - b).abs()
        })
        .fold(0.0f32, f32::max);
    assert!(
        sensitivity > 1.0e-6,
        "independent oracle threshold perturbation was insensitive"
    );
}

/// Equal current prefixes stay bit-identical when a future suffix changes, proving causality.
#[test]
fn active_prefix_has_no_future_anticipation() {
    const PREFIX: usize = 64;
    let active = active_values();
    let mut first = vec![0.0f32; 512];
    let mut second = first.clone();
    for index in 0..PREFIX {
        let sample = 0.8 * (core::f32::consts::TAU * 120.0 * index as f32 / 48_000.0).sin();
        first[index] = sample;
        second[index] = sample;
    }
    for (index, sample) in second.iter_mut().enumerate().skip(PREFIX) {
        *sample = 0.8 * (core::f32::consts::TAU * 4_000.0 * index as f32 / 48_000.0).sin();
    }
    let mut first_right = first.clone();
    let mut second_right = second.clone();
    let mut first_effect = MultibandCompressorFactory
        .prepare(request(&active))
        .expect("first prefix effect");
    let mut second_effect = MultibandCompressorFactory
        .prepare(request(&active))
        .expect("second prefix effect");
    for start in (0..first.len()).step_by(128) {
        process(
            first_effect.as_mut(),
            &mut first[start..start + 128],
            &mut first_right[start..start + 128],
            start as u64,
            &[],
            128,
        );
        process(
            second_effect.as_mut(),
            &mut second[start..start + 128],
            &mut second_right[start..start + 128],
            start as u64,
            &[],
            128,
        );
    }
    assert_eq!(&first[..PREFIX], &second[..PREFIX]);
    assert_eq!(&first_right[..PREFIX], &second_right[..PREFIX]);
    assert!(first[..PREFIX].iter().any(|sample| *sample != 0.0));
    assert_ne!(&first[PREFIX..128], &second[PREFIX..128]);
    assert!(second[PREFIX..128].iter().any(|sample| sample.abs() > 0.0));
}

/// E0c. A bypassed instance is a causal dry path, and signed zero survives it.
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
    let span = [point(1, ParameterChannel::Left, 0, -80.0)];
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
    assert_eq!(output[0].to_bits(), (-0.5f32).to_bits());
    assert_eq!(output[1].to_bits(), (-0.0f32).to_bits());

    let saved = snapshot(effect.as_ref());
    let mut malformed = saved.clone();
    malformed.1[..4].copy_from_slice(&2_000.0f32.to_bits().to_le_bytes());
    malformed.2[..4].fill(u8::MAX);
    assert!(restore(effect.as_mut(), 1, &malformed, sizes).is_err());
    assert_eq!(snapshot(effect.as_ref()), saved);
    // An invalid version is rejected on the out-of-band `state_layout_version` argument, which is
    // where the version lives until #95 adopts the shared codec's header (W2-D2).
    assert_eq!(
        restore(effect.as_mut(), 0, &saved, sizes)
            .expect_err("stale version")
            .code,
        "effect.state.version"
    );
    restore(effect.as_mut(), 1, &saved, sizes).expect("round trip");
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
        (1, "effect.state.gain"),           // low-band smoother
        (3, "effect.state.parameter"),      // low threshold, current
        (43, "effect.state.filter"),        // first stage ic1
    ] {
        let mut corrupted = saved.clone();
        corrupted.1[word * 4..word * 4 + 4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        assert_eq!(
            restore(effect.as_mut(), 1, &corrupted, sizes)
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
    live_step.1[5 * 4..5 * 4 + 4].copy_from_slice(&0.5f32.to_bits().to_le_bytes());
    assert_eq!(
        restore(effect.as_mut(), 1, &live_step, sizes)
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
            effect_contract::StatePayloadOutput::new(
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

/// A bank stages both channels before commit: malformed right state and retired payload lengths
/// leave the target lane and every untouched lane byte-identical.
#[test]
fn bank_rejected_restore_changes_nothing_at_each_launch_rate() {
    for (rate, old_bytes) in [
        (44_100u32, 7_256usize),
        (48_000, 7_880),
        (88_200, 14_312),
        (96_000, 15_560),
    ] {
        let initial = values();
        let mut requests = (0..4).map(|_| request(&initial)).collect::<Vec<_>>();
        for request in &mut requests {
            request.sample_rate = rate;
        }
        let mut bank = support::bank(BankWidth::Four, &requests);
        let sizes = bank.metadata().program_key.state_sizes;
        let mut left = vec![0.3f32; 128 * 4];
        let mut right = vec![-0.2f32; 128 * 4];
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                128,
                BankWidth::Four,
                0,
                &[],
                &[0u32; 5],
                128,
            )
            .expect("bank block"),
        );
        let saved = (0..4)
            .map(|track| support::snapshot_track(bank.as_ref(), track, sizes))
            .collect::<Vec<_>>();
        let mut malformed = saved[0].clone();
        malformed.1[..4].copy_from_slice(&2_000.0f32.to_bits().to_le_bytes());
        malformed.2[..4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        let payload = StatePayloadInput::new(&malformed.0, &malformed.1, &malformed.2, sizes)
            .expect("compact malformed payload");
        assert!(bank.restore_track_state_payload(0, 1, payload).is_err());
        for (track, state) in saved.iter().enumerate() {
            assert_eq!(
                support::snapshot_track(bank.as_ref(), track as u32, sizes),
                *state,
                "right-channel rollback rate={rate} track={track}"
            );
        }
        let old_left = vec![0u8; old_bytes];
        let old_right = vec![0u8; old_bytes];
        let old_sizes = StatePayloadSizes {
            common_bytes: 0,
            left_bytes: old_bytes as u32,
            right_bytes: old_bytes as u32,
        };
        let old_payload = StatePayloadInput::new(&[], &old_left, &old_right, old_sizes)
            .expect("historical payload shape");
        assert!(bank.restore_track_state_payload(0, 1, old_payload).is_err());
        assert_eq!(
            support::snapshot_track(bank.as_ref(), 0, sizes),
            saved[0],
            "historical bank payload rollback rate={rate}"
        );
    }
}

/// Compressing one band leaves the other alone.
#[test]
fn isolated_low_and_high_band_compression_reduce_only_the_selected_band() {
    for (frequency, base) in [(120.0f32, 0usize), (4_000.0, 5)] {
        let mut active_values = values();
        let mut identity_values = values();
        for lane in 0..2 {
            active_values[(base + 1) * 2 + lane].value = -45.0;
            active_values[(base + 2) * 2 + lane].value = 20.0;
            active_values[(base + 3) * 2 + lane].value = 0.1;
            active_values[(base + 4) * 2 + lane].value = 5.0;
            identity_values[(base + 2) * 2 + lane].value = 1.0;
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
    let wrong_count = sets[..3].iter().map(|set| request(set)).collect::<Vec<_>>();
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: backend_for(BankWidth::Four),
                width: BankWidth::Four,
                requests: &wrong_count,
            })
            .err(),
        Some(EffectPrepareError {
            code: "effect.bank.requests"
        })
    );
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

    let mut mixed_malformed_sets = sets.clone();
    mixed_malformed_sets[3][0].value = f32::NAN;
    let mut mixed_malformed = mixed_malformed_sets
        .iter()
        .map(|set| request(set))
        .collect::<Vec<_>>();
    mixed_malformed[2] = request_with(&mixed_malformed_sets[2], LinkMode::Maximum, 128, false);
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: backend_for(BankWidth::Four),
                width: BankWidth::Four,
                requests: &mixed_malformed,
            })
            .err(),
        Some(EffectPrepareError {
            code: "effect.parameter.initial"
        }),
        "malformed members are rejected before mixed-program fallback"
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

    let quality = MULTIBAND_COMPRESSOR_DESCRIPTOR
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
    assert_eq!(per_track, 376);
    assert_eq!(per_track * 4, 1_504);
    assert_eq!(per_track * 8, 3_008);
}
