//! Current causal contract and resource gates for the gate/expander.

mod support;

use effect_contract::{
    EffectQuality, InitialParameterValue, LatencySamples, LinkMode, NativeEffectFactory,
    ParameterChannel, PreparedPorts, PreparedSidechainPort, ResetKind, StatePayloadInput,
    StatePayloadOutput, TailSamples, validate_descriptor,
};
use gate_expander::{
    GATE_EXPANDER_DESCRIPTOR, GATE_EXPANDER_PARAMETERS, GateExpanderFactory,
    STATE_LAYOUT_VERSION,
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
    for (quality, rate) in GATE_EXPANDER_DESCRIPTOR.qualities.iter().zip([
        44_100, 48_000, 88_200, 96_000,
    ]) {
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
    assert!(effect.restore_state_payload(STATE_LAYOUT_VERSION, input).is_err());
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
fn retired_id_eight_is_rejected_before_preparation() {
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
    assert_eq!(
        GateExpanderFactory
            .prepare(request_at_rate(&values, 48_000))
            .expect_err("ID 8 must be rejected")
            .code,
        "effect.parameter.initial"
    );
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
    }
}

#[test]
fn connected_sidechain_uses_current_detector_word() {
    let mut values = support::active_values();
    values[0].value = -20.0;
    values[1].value = 4.0;
    values[2].value = 48.0;
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
