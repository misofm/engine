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
use support::{initial_values, prepare, render_scalar, request, request_at_rate, snapshot};

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
    let quality = GATE_EXPANDER_DESCRIPTOR.qualities[1];
    let mut exact = request(&values);
    exact.limits = PrepareEffectLimits {
        maximum_total_state_bytes: 184,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 16,
    };
    GateExpanderFactory.prepare(exact).expect("exact caps");
    let mut state_short = exact;
    state_short.limits.maximum_total_state_bytes = 183;
    assert_eq!(
        GateExpanderFactory
            .prepare(state_short)
            .err()
            .expect("state limit")
            .code,
        "effect.resource.limit"
    );
    let mut scratch_short = exact;
    scratch_short.limits.maximum_scratch_bytes = 63;
    assert_eq!(
        GateExpanderFactory
            .prepare(scratch_short)
            .err()
            .expect("scratch limit")
            .code,
        "effect.resource.limit"
    );
    assert_eq!(quality.maximum_state.total(), Some(184));
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
fn same_index_bypass_and_ratio_one_are_exact_identity_at_all_rates() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let mut values = initial_values();
        values[0].value = -40.0;
        values[1].value = -40.0;
        values[2].value = 1.0;
        values[3].value = 1.0;
        let mut request = request_at_rate(&values, rate);
        request.bypass = true;
        let mut effect = GateExpanderFactory.prepare(request).expect("prepare");
        let mut left = [f32::from_bits(0x8000_0000), 0.25, -0.5];
        let mut right = [-0.0, -0.25, 0.5];
        let expected = (left.clone(), right.clone());
        render_scalar(effect.as_mut(), &mut left, &mut right, 3);
        assert_eq!(left, expected.0);
        assert_eq!(right, expected.1);
        assert_eq!(effect.metadata().latency, LatencySamples(0));

        let mut enabled_values = initial_values();
        support::set_parameter(&mut enabled_values, 1, 1.0, 1.0);
        support::set_parameter(&mut enabled_values, 2, 0.0, 0.0);
        let mut enabled = prepare(request_at_rate(&enabled_values, rate));
        let mut enabled_left = [f32::from_bits(0x8000_0000), 0.25, -0.5];
        let mut enabled_right = [-0.0, -0.25, 0.5];
        let expected_enabled = (enabled_left.clone(), enabled_right.clone());
        render_scalar(enabled.as_mut(), &mut enabled_left, &mut enabled_right, 3);
        assert_eq!(enabled_left, expected_enabled.0);
        assert_eq!(enabled_right, expected_enabled.1);
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
    let mut left = vec![0.5, 1.0e-4, 1.0e-4, 1.0e-4, 0.5];
    let mut right = left.clone();
    render_scalar(effect.as_mut(), &mut left, &mut right, 5);
    assert_eq!(left[0].to_bits(), 0.5_f32.to_bits(), "trigger");
    assert_eq!(
        left[1].to_bits(),
        1.0e-4_f32.to_bits(),
        "first held sample remains open"
    );
    assert_eq!(
        left[2].to_bits(),
        1.0e-4_f32.to_bits(),
        "second held sample remains open"
    );
    assert!(
        left[3].abs() < 1.0e-4,
        "the sample after K held samples closes"
    );
    assert!(left[4] > left[3], "retrigger uses the current sample");
}
