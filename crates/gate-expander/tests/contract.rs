//! Current causal contract and resource gates for the gate/expander.

mod support;

use effect_contract::{
    BankWidth, InitialParameterValue, LatencySamples, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits, PreparedPorts,
    PreparedSidechainPort, ResetKind, StatePayloadInput, StatePayloadOutput, TailSamples,
    validate_descriptor,
};
use gate_expander::{
    GATE_EXPANDER_DESCRIPTOR, GATE_EXPANDER_PARAMETERS, GateExpanderFactory, STATE_LAYOUT_VERSION,
};
use support::{
    initial_values, prepare, render_scalar, render_scalar_sidechain, request, request_at_rate,
    snapshot,
};

fn word(bytes: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("word"))
}

#[test]
fn descriptor_and_exact_zero_latency_resources_are_frozen() {
    validate_descriptor(&GATE_EXPANDER_DESCRIPTOR).expect("descriptor");
    assert_eq!(STATE_LAYOUT_VERSION, 1);
    assert_eq!(GATE_EXPANDER_PARAMETERS.len(), 7);
    for (index, parameter) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
        assert_eq!(parameter.id.0, index as u32 + 1);
    }
    for (quality, rate) in GATE_EXPANDER_DESCRIPTOR
        .qualities
        .iter()
        .zip([44_100, 48_000, 88_200, 96_000])
    {
        assert_eq!(quality.sample_rate, rate);
        assert_eq!(quality.latency, LatencySamples(0));
        assert_eq!(quality.tail, TailSamples::Finite(0));
        assert_eq!(quality.maximum_state.common_bytes, 8);
        assert_eq!(quality.maximum_state.left_bytes, 88);
        assert_eq!(quality.maximum_state.right_bytes, 88);
        assert_eq!(quality.maximum_state.total(), Some(184));
        assert_eq!(quality.scratch_fixed_bytes, 64);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
    }
}

#[test]
fn exact_state_and_scratch_caps_prepare_and_one_byte_below_rejects() {
    let values = initial_values();
    for quality in GATE_EXPANDER_DESCRIPTOR.qualities {
        let mut exact = request_at_rate(&values, quality.sample_rate);
        exact.limits = PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: quality.scratch_fixed_bytes,
            maximum_automation_spans_per_block: 16,
        };
        GateExpanderFactory
            .prepare(exact)
            .unwrap_or_else(|error| panic!("exact caps at {} Hz: {error:?}", quality.sample_rate));
        let mut state_short = exact;
        state_short.limits.maximum_total_state_bytes -= 1;
        assert_eq!(
            GateExpanderFactory
                .prepare(state_short)
                .err()
                .expect("state limit")
                .code,
            "effect.resource.limit",
            "state cap at {} Hz",
            quality.sample_rate
        );
        let mut scratch_short = exact;
        scratch_short.limits.maximum_scratch_bytes -= 1;
        assert_eq!(
            GateExpanderFactory
                .prepare(scratch_short)
                .err()
                .expect("scratch limit")
                .code,
            "effect.resource.limit",
            "scratch cap at {} Hz",
            quality.sample_rate
        );
        assert_eq!(quality.maximum_state.total(), Some(184));
    }
}

#[test]
fn payload_header_is_one_fourty_four_words_and_restore_is_transactional() {
    let values = initial_values();
    let mut effect = prepare(request(&values));
    let (common, left, right) = snapshot(effect.as_ref());
    assert_eq!(common.len(), 8);
    assert_eq!(left.len(), 88);
    assert_eq!(right.len(), 88);
    assert_eq!(word(&common, 0), 1);
    assert_eq!(word(&common, 1), 44);

    let before = (common.clone(), left.clone(), right.clone());
    let mut malformed = right.clone();
    malformed[87] = 0x7f;
    let sizes = effect.metadata().state_sizes;
    let input = StatePayloadInput::new(&common, &left, &malformed, sizes).expect("sizes");
    assert!(
        effect
            .restore_state_payload(STATE_LAYOUT_VERSION, input)
            .is_err()
    );
    assert_eq!(snapshot(effect.as_ref()), before);

    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let mut output_common = vec![0; sizes.common_bytes as usize];
    let mut output_left = vec![0; sizes.left_bytes as usize];
    let mut output_right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(
                &mut output_common,
                &mut output_left,
                &mut output_right,
                sizes,
            )
            .expect("sizes"),
        )
        .expect("snapshot");
}

#[test]
fn extra_initial_record_is_rejected_before_preparation() {
    let mut values = initial_values().to_vec();
    values.extend([
        InitialParameterValue {
            parameter_index: 7,
            channel: ParameterChannel::Left,
            value: 0.0,
        },
        InitialParameterValue {
            parameter_index: 7,
            channel: ParameterChannel::Right,
            value: 0.0,
        },
    ]);
    let result = GateExpanderFactory.prepare(request_at_rate(&values, 48_000));
    assert!(result.is_err(), "ID 8 must be rejected");
    assert_eq!(
        result.err().expect("error").code,
        "effect.parameter.initial"
    );
}

#[test]
fn malformed_bank_member_is_rejected_before_scalar_fallback() {
    let mut values = [initial_values(); 8];
    values[3][0].parameter_index = 99;
    let requests: Vec<_> = values.iter().map(|values| request(values)).collect();
    let result = GateExpanderFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
        backend: lane::Backend::Simd8,
        width: BankWidth::Eight,
        requests: &requests,
    });
    assert_eq!(
        result.err().expect("malformed member").code,
        "effect.parameter.unknown"
    );
}

#[test]
fn initial_values_require_interleaved_lane_records_and_normal_values() {
    let mut values = initial_values();
    values[0].channel = ParameterChannel::Right;
    assert!(GateExpanderFactory.prepare(request(&values)).is_err());
    let mut values = initial_values();
    values[0].value = f32::from_bits(1);
    assert!(GateExpanderFactory.prepare(request(&values)).is_err());
    let mut values = initial_values();
    values[0].value = -0.0;
    assert!(GateExpanderFactory.prepare(request(&values)).is_err());
    let mut values = initial_values();
    values[0].channel = ParameterChannel::Both;
    assert!(GateExpanderFactory.prepare(request(&values)).is_err());
}

#[test]
fn missing_and_misordered_initial_records_return_exact_errors() {
    let mut missing = initial_values().to_vec();
    missing.pop();
    let error = GateExpanderFactory
        .prepare(request(&missing))
        .err()
        .expect("missing initial record");
    assert_eq!(error.code, "effect.parameter.initial");

    let mut misordered = initial_values();
    misordered[2].parameter_index = 3;
    let error = GateExpanderFactory
        .prepare(request(&misordered))
        .err()
        .expect("misordered initial record");
    assert_eq!(error.code, "effect.parameter.initial");
}

#[test]
fn bypass_is_exact_identity_at_all_rates() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let values = initial_values();
        let mut request = request_at_rate(&values, rate);
        request.bypass = true;
        let mut effect = GateExpanderFactory.prepare(request).expect("prepare");
        let mut left = [f32::from_bits(0x8000_0000), 0.25, -0.5];
        let mut right = [-0.0, -0.25, 0.5];
        let expected = (left, right);
        render_scalar(effect.as_mut(), &mut left, &mut right, 3);
        assert_eq!(left, expected.0);
        assert_eq!(right, expected.1);
        assert_eq!(effect.metadata().latency, LatencySamples(0));
    }
}

#[test]
fn enabled_ratio_one_is_exact_identity_with_a_nonzero_sample_zero() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let mut values = support::active_values();
        support::set_parameter(&mut values, 1, 1.0, 1.0);
        let mut effect = prepare(request_at_rate(&values, rate));
        let mut left = [0.5, -0.25, 0.125];
        let mut right = [-0.5, 0.25, -0.125];
        let expected = (left, right);
        render_scalar(effect.as_mut(), &mut left, &mut right, 3);
        assert_eq!(left, expected.0, "ratio one at {rate} Hz");
        assert_eq!(right, expected.1, "ratio one at {rate} Hz");
    }
}

#[test]
fn enabled_range_zero_is_exact_identity_with_a_nonzero_sample_zero() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let mut values = support::active_values();
        support::set_parameter(&mut values, 2, 0.0, 0.0);
        let mut effect = prepare(request_at_rate(&values, rate));
        let mut left = [0.5, -0.25, 0.125];
        let mut right = [-0.5, 0.25, -0.125];
        let expected = (left, right);
        render_scalar(effect.as_mut(), &mut left, &mut right, 3);
        assert_eq!(left, expected.0, "range zero at {rate} Hz");
        assert_eq!(right, expected.1, "range zero at {rate} Hz");
    }
}

#[test]
fn opening_and_future_suffixes_are_current_sample_causal() {
    let mut values = support::active_values();
    support::set_parameter(&mut values, 0, -20.0, -20.0);
    support::set_parameter(&mut values, 1, 4.0, 4.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    support::set_parameter(&mut values, 5, 0.0, 0.0);
    let mut a = prepare(request(&values));
    let mut b = prepare(request(&values));
    let mut left_a = vec![0.0, 0.5, 0.0, 0.0];
    let mut right_a = left_a.clone();
    let mut left_b = left_a.clone();
    let mut right_b = right_a.clone();
    left_b[2] = 0.9;
    right_b[2] = 0.9;
    render_scalar(a.as_mut(), &mut left_a, &mut right_a, 4);
    render_scalar(b.as_mut(), &mut left_b, &mut right_b, 4);
    assert_eq!(
        left_a[1].to_bits(),
        left_b[1].to_bits(),
        "opening uses the current sample"
    );
    assert!(
        left_a[1].abs() < 0.5,
        "closed state is established before the trigger"
    );
}

#[allow(clippy::disallowed_methods)]
fn detector_level_db(amplitude: f32) -> f32 {
    use gate_expander::kernel::{LEVEL_FLOOR, LEVEL_MAX_DB, LEVEL_MIN_DB};
    let level = math::fast_db::fast_level_db::<f32>(amplitude.max(LEVEL_FLOOR));
    level.clamp(LEVEL_MIN_DB, LEVEL_MAX_DB)
}

fn transition_values(threshold: f32, hysteresis: f32) -> support::Values {
    let mut values = initial_values();
    support::set_parameter(&mut values, 0, threshold, threshold);
    support::set_parameter(&mut values, 1, 20.0, 20.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    support::set_parameter(&mut values, 3, hysteresis, hysteresis);
    support::set_parameter(&mut values, 4, 1.0, 1.0);
    support::set_parameter(&mut values, 5, 0.0, 0.0);
    support::set_parameter(&mut values, 6, 5.0, 5.0);
    values
}

#[test]
fn a_closed_gate_opens_on_the_current_sample_and_uses_first_attack() {
    let values = transition_values(-20.0, 6.0);
    let mut effect = prepare(request(&values));
    let mut quiet = [0.01_f32, 0.01];
    let mut quiet_right = quiet;
    render_scalar(effect.as_mut(), &mut quiet, &mut quiet_right, 2);
    let (_, closed, _) = snapshot(effect.as_ref());
    assert_eq!(
        word(&closed, 1),
        0,
        "the quiet prefix establishes closed state"
    );
    assert!(
        f32::from_bits(word(&closed, 0)) < 0.0,
        "the closed prefix establishes negative gain"
    );

    let mut trigger = [0.5_f32];
    let mut trigger_right = trigger;
    render_scalar(effect.as_mut(), &mut trigger, &mut trigger_right, 1);
    assert!(
        trigger[0] > 0.0 && trigger[0] < 0.5,
        "first attack is not unity"
    );
    let (_, opened, _) = snapshot(effect.as_ref());
    assert_eq!(
        word(&opened, 1),
        1.0_f32.to_bits(),
        "trigger opens the gate"
    );
}

#[test]
fn hold_zero_closes_on_the_first_sample_below_the_close_band() {
    let values = transition_values(-20.0, 6.0);
    let mut effect = prepare(request(&values));
    let mut left = [0.01_f32];
    let mut right = left;
    render_scalar(effect.as_mut(), &mut left, &mut right, 1);
    let (_, payload, _) = snapshot(effect.as_ref());
    assert_eq!(word(&payload, 1), 0, "hold zero closes immediately");
    assert!(
        left[0].abs() < 0.01,
        "the first closing sample is attenuated"
    );
}

#[test]
fn opening_and_rearm_are_inclusive_at_exact_thresholds() {
    let trigger = 0.1_f32;
    let level = detector_level_db(trigger);
    let mut values = transition_values(level, 6.0);
    let mut render = |threshold: f32, source: &[f32]| {
        support::set_parameter(&mut values, 0, threshold, threshold);
        let mut effect = prepare(request(&values));
        let mut left = source.to_vec();
        let mut right = left.clone();
        render_scalar(effect.as_mut(), &mut left, &mut right, 128);
        let (_, payload, _) = snapshot(effect.as_ref());
        (left, payload)
    };
    let below = level.next_down();
    let above = level.next_up();
    let source = [0.01_f32, trigger];
    let (_below_output, below_state) = render(below, &source);
    let (_equal_output, equal_state) = render(level, &source);
    let (_above_output, above_state) = render(above, &source);
    assert_eq!(word(&below_state, 1), 1.0_f32.to_bits());
    assert_eq!(word(&equal_state, 1), 1.0_f32.to_bits());
    assert_eq!(word(&above_state, 1), 0);

    let hold_level = 0.1_f32;
    let close_level = detector_level_db(hold_level);
    let hysteresis = 6.0_f32;
    let mut threshold = close_level + hysteresis;
    while (threshold - hysteresis).to_bits() != close_level.to_bits() {
        threshold = if threshold - hysteresis > close_level {
            threshold.next_down()
        } else {
            threshold.next_up()
        };
    }
    let mut threshold_below = threshold;
    while (threshold_below - hysteresis).to_bits() == close_level.to_bits() {
        threshold_below = threshold_below.next_down();
    }
    let mut threshold_above = threshold;
    while (threshold_above - hysteresis).to_bits() == close_level.to_bits() {
        threshold_above = threshold_above.next_up();
    }
    let mut values = transition_values(threshold, hysteresis);
    let source = [trigger, hold_level];
    let mut run_rearm = |threshold: f32| {
        support::set_parameter(&mut values, 0, threshold, threshold);
        let mut effect = prepare(request(&values));
        let mut left = source;
        let mut right = left;
        render_scalar(effect.as_mut(), &mut left, &mut right, 128);
        let (_, payload, _) = snapshot(effect.as_ref());
        (left[1], payload)
    };
    let (_below_rearm, below_state) = run_rearm(threshold_below);
    let (_equal_rearm, equal_state) = run_rearm(threshold);
    let (_above_rearm, above_state) = run_rearm(threshold_above);
    assert_eq!(word(&below_state, 1), 1.0_f32.to_bits());
    assert_eq!(word(&equal_state, 1), 1.0_f32.to_bits());
    assert_eq!(word(&above_state, 1), 0);
}

#[test]
fn connected_sidechain_uses_current_detector_word() {
    let mut values = support::active_values();
    support::set_parameter(&mut values, 0, -20.0, -20.0);
    support::set_parameter(&mut values, 1, 4.0, 4.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    let mut preparation = request(&values);
    preparation.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: support::sidechain_port(),
            required: false,
        },
    };
    preparation.link_mode = LinkMode::DualMono;
    let mut effect = GateExpanderFactory.prepare(preparation).expect("sidechain");
    let mut main_left = vec![0.5; 8];
    let mut main_right = vec![0.5; 8];
    let side_left = vec![0.0; 8];
    let side_right = vec![0.0; 8];
    let report = support::render_scalar_sidechain(
        effect.as_mut(),
        &mut main_left,
        &mut main_right,
        Some((&side_left, &side_right)),
        8,
        &[],
        0,
    );
    assert_eq!(report.nonfinite_left_blocks, 0);
    assert!(main_left.iter().all(|sample| sample.is_finite()));
}

#[test]
fn connected_sidechain_is_current_and_not_ignored_or_delayed() {
    let mut values = support::active_values();
    support::set_parameter(&mut values, 0, -20.0, -20.0);
    support::set_parameter(&mut values, 1, 4.0, 4.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    support::set_parameter(&mut values, 5, 0.0, 0.0);
    let mut preparation = request(&values);
    preparation.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: support::sidechain_port(),
            required: false,
        },
    };
    let mut effect = GateExpanderFactory.prepare(preparation).expect("sidechain");
    let mut main_left = [0.01_f32, 0.01];
    let mut main_right = main_left;
    let side_left = [0.5_f32, 0.0];
    let side_right = side_left;
    render_scalar_sidechain(
        effect.as_mut(),
        &mut main_left,
        &mut main_right,
        Some((&side_left, &side_right)),
        2,
        &[],
        0,
    );
    assert_eq!(main_left[0].to_bits(), 0.01_f32.to_bits());
    assert_eq!(main_right[0].to_bits(), 0.01_f32.to_bits());
    assert!(
        main_left[1].abs() < 0.01,
        "the next quiet sidechain word closes"
    );
}

#[test]
fn a_valid_connected_sidechain_uses_scalar_fallback_after_bank_rejection() {
    let values = [initial_values(); 4];
    let mut requests: Vec<_> = values.iter().map(|set| request(set)).collect();
    for item in &mut requests {
        item.ports = PreparedPorts {
            sidechain: PreparedSidechainPort::Connected {
                id: support::sidechain_port(),
                required: false,
            },
        };
    }
    assert!(
        GateExpanderFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: lane::Backend::Simd4,
                width: BankWidth::Four,
                requests: &requests,
            })
            .expect("valid connected requests")
            .is_none(),
        "connected sidechain must use scalar fallback"
    );
    let mut scalar = GateExpanderFactory
        .prepare(requests[0])
        .expect("connected scalar fallback");
    let mut main_left = [0.01_f32];
    let mut main_right = [0.01_f32];
    let side = [0.5_f32];
    let report = render_scalar_sidechain(
        scalar.as_mut(),
        &mut main_left,
        &mut main_right,
        Some((&side, &side)),
        1,
        &[],
        0,
    );
    assert_eq!(report, effect_contract::ProcessReport::default());
    assert_eq!(main_left[0].to_bits(), 0.01_f32.to_bits());
}

#[test]
fn connected_sidechain_future_suffix_does_not_change_current_output() {
    let mut values = support::active_values();
    support::set_parameter(&mut values, 0, -20.0, -20.0);
    support::set_parameter(&mut values, 1, 4.0, 4.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    support::set_parameter(&mut values, 5, 0.0, 0.0);
    let mut request_a = request(&values);
    request_a.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: support::sidechain_port(),
            required: false,
        },
    };
    let request_b = request_a;
    let mut a = GateExpanderFactory.prepare(request_a).unwrap();
    let mut b = GateExpanderFactory.prepare(request_b).unwrap();
    let mut main_a = vec![0.0, 0.5, 0.0, 0.0];
    let mut main_b = main_a.clone();
    let side_a = vec![0.0, 0.5, 0.0, 0.0];
    let side_b = vec![0.0, 0.5, 0.9, 0.9];
    let mut right_a = main_a.clone();
    let mut right_b = main_b.clone();
    support::render_scalar_sidechain(
        a.as_mut(),
        &mut main_a,
        &mut right_a,
        Some((&side_a, &side_a)),
        4,
        &[],
        0,
    );
    support::render_scalar_sidechain(
        b.as_mut(),
        &mut main_b,
        &mut right_b,
        Some((&side_b, &side_b)),
        4,
        &[],
        0,
    );
    assert_eq!(main_a[1].to_bits(), main_b[1].to_bits());
}

#[test]
fn connected_sidechain_nan_keeps_existing_detector_boundary_policy() {
    let values = support::active_values();
    let mut preparation = request(&values);
    preparation.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: support::sidechain_port(),
            required: false,
        },
    };
    let mut effect = GateExpanderFactory.prepare(preparation).unwrap();
    let mut main_left = vec![0.25; 4];
    let mut main_right = vec![0.25; 4];
    let side_left = vec![f32::NAN; 4];
    let side_right = vec![f32::NAN; 4];
    let report = support::render_scalar_sidechain(
        effect.as_mut(),
        &mut main_left,
        &mut main_right,
        Some((&side_left, &side_right)),
        4,
        &[],
        0,
    );
    assert_eq!(report.nonfinite_left_blocks, 0);
    assert_eq!(report.nonfinite_right_blocks, 0);
    assert!(main_left.iter().all(|sample| sample.is_finite()));
}

#[test]
fn production_hold_is_k_plus_one_and_retrigger_is_current_sample() {
    let mut values = initial_values();
    support::set_parameter(&mut values, 0, -40.0, -40.0);
    support::set_parameter(&mut values, 1, 4.0, 4.0);
    support::set_parameter(&mut values, 2, 48.0, 48.0);
    support::set_parameter(&mut values, 3, 6.0, 6.0);
    support::set_parameter(
        &mut values,
        5,
        2.0 * 1000.0 / 48_000.0,
        2.0 * 1000.0 / 48_000.0,
    );
    let mut effect = prepare(request(&values));
    let mut left = vec![0.5, 0.006, 1.0e-4, 1.0e-4, 1.0e-4, 0.5];
    let mut right = left.clone();
    render_scalar(effect.as_mut(), &mut left, &mut right, 6);
    assert_eq!(left[0].to_bits(), 0.5_f32.to_bits(), "trigger");
    assert_eq!(
        left[1].to_bits(),
        0.006_f32.to_bits(),
        "in-band sample rearms the nonzero hold"
    );
    assert_eq!(
        left[2].to_bits(),
        1.0e-4_f32.to_bits(),
        "first below-band sample remains open"
    );
    assert_eq!(
        left[3].to_bits(),
        1.0e-4_f32.to_bits(),
        "second below-band sample remains open"
    );
    assert!(left[4].abs() < 1.0e-4, "the K+1th below-band sample closes");
    assert!(left[5] > left[4], "retrigger uses the current sample");
}
