//! Canonical current-layout state-envelope and one-pass descriptor-binding coverage.

use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use sha2::{Digest, Sha256};

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("test effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("test port id"),
    }
}

static PARAMETERS: [ParameterDescriptorV1; 2] = [
    ParameterDescriptorV1 {
        id: ParameterId(1),
        display_name: "Shared",
        display_unit: "linear",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(-1.0),
        maximum: Some(1.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(2),
        display_name: "Per lane",
        display_unit: "linear",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(-2.0),
        maximum: Some(2.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        enum_choices: &[],
    },
];

static PORTS: [PortDescriptorV1; 3] = [
    PortDescriptorV1 {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("detector"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

const STATE_SIZES: StatePayloadSizes = StatePayloadSizes {
    common_bytes: 3,
    left_bytes: 5,
    right_bytes: 5,
};

const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(9),
        tail: TailSamples::Finite(17),
        maximum_state: STATE_SIZES,
        scratch_fixed_bytes: 11,
        scratch_bytes_per_frame: 2,
    }
}

static QUALITIES: [QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

static DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.state"),
    display_name: "State test",
    contract_major: 1,
    contract_minor: 7,
    state_layout_version: 3,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
};

static ALTERNATE_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    display_name: "Other name",
    ..DESCRIPTOR
};

static INITIAL: [InitialParameterValue; 3] = [
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Both,
        value: 0.25,
    },
    InitialParameterValue {
        parameter_index: 1,
        channel: ParameterChannel::Left,
        value: -0.5,
    },
    InitialParameterValue {
        parameter_index: 1,
        channel: ParameterChannel::Right,
        value: 1.5,
    },
];

fn descriptor_wire(descriptor: &'static EffectDescriptorV1) -> Vec<u8> {
    let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
    let mut output = vec![0; required as usize];
    assert_eq!(
        encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut output),
        Ok(required)
    );
    output
}

fn replay() -> EffectStateReplayViewV1<'static> {
    let request = PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 8,
        quality: EffectQuality::Normal,
        bypass: true,
        link_mode: LinkMode::Maximum,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::Connected {
                id: port_id("detector"),
                required: false,
            },
        },
        initial_values: &INITIAL,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 64,
            maximum_scratch_bytes: 128,
            maximum_automation_spans_per_block: 23,
        },
    };
    EffectStateReplayViewV1 {
        effect_id: DESCRIPTOR.id,
        request,
    }
}

fn encoded_state() -> (Vec<u8>, Vec<u8>) {
    let wire = descriptor_wire(&DESCRIPTOR);
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let requirements =
        effect_state_v1_requirements(bound, replay(), EffectStateLimitsV1::default()).unwrap();
    let mut output = vec![0xa5; requirements.envelope_bytes as usize];
    assert_eq!(
        encode_effect_state_v1(
            bound,
            replay(),
            b"com",
            b"left!",
            b"right",
            EffectStateLimitsV1::default(),
            &mut output
        ),
        Ok(requirements.envelope_bytes)
    );
    (wire, output)
}

fn refresh_digest(bytes: &mut [u8]) {
    let mut hasher = Sha256::new();
    hasher.update(b"miso.engine.effect-state.current-layout.v1\0");
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(&bytes[..56]);
    hasher.update([0; 32]);
    hasher.update(&bytes[88..]);
    let digest: [u8; 32] = hasher.finalize().into();
    bytes[56..88].copy_from_slice(&digest);
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn get_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[test]
fn diagnostic_layout_and_default_limits_are_frozen() {
    assert_eq!(core::mem::size_of::<EffectStateDiagnosticV1>(), 32);
    assert_eq!(core::mem::align_of::<EffectStateDiagnosticV1>(), 8);
    assert_eq!(core::mem::offset_of!(EffectStateDiagnosticV1, code), 0);
    assert_eq!(core::mem::offset_of!(EffectStateDiagnosticV1, detail), 4);
    assert_eq!(
        core::mem::offset_of!(EffectStateDiagnosticV1, item_index),
        8
    );
    assert_eq!(core::mem::offset_of!(EffectStateDiagnosticV1, reserved), 12);
    assert_eq!(
        core::mem::offset_of!(EffectStateDiagnosticV1, byte_offset),
        16
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateDiagnosticV1, required_bytes),
        24
    );
    assert_eq!(
        EffectStateLimitsV1::default(),
        EffectStateLimitsV1 {
            maximum_descriptor_bytes: 4_194_304,
            maximum_envelope_bytes: 268_435_456,
            maximum_payload_bytes: 134_217_728,
            maximum_initial_values: 4_096,
        }
    );
}

#[test]
fn binding_distinguishes_external_wire_from_static_mismatch() {
    let mut malformed = descriptor_wire(&DESCRIPTOR);
    malformed[12] = 1;
    let error = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &malformed, 1 << 20).unwrap_err();
    assert_eq!(
        error.kind(),
        EffectDescriptorBindingErrorKindV1::ExternalWire
    );
    assert_eq!(
        (error.diagnostic().code, error.diagnostic().byte_offset),
        (EffectDescriptorWireDiagnosticCodeV1::Reserved, 12)
    );

    let alternate = descriptor_wire(&ALTERNATE_DESCRIPTOR);
    let error = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &alternate, 1 << 20).unwrap_err();
    assert_eq!(
        error.kind(),
        EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch
    );
    assert_eq!(
        (error.diagnostic().code, error.diagnostic().byte_offset),
        (EffectDescriptorWireDiagnosticCodeV1::Semantic, 40)
    );
}

#[test]
fn exact_wire_round_trip_preserves_independent_sections_and_suffixes() {
    let wire = descriptor_wire(&DESCRIPTOR);
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let requirements =
        effect_state_v1_requirements(bound, replay(), EffectStateLimitsV1::default()).unwrap();
    assert_eq!(requirements.payload_snapshot_scratch_bytes, 13);
    assert_eq!(requirements.initial_value_scratch_slots, 3);
    let mut output = vec![0x5a; requirements.envelope_bytes as usize + 19];
    assert_eq!(
        encode_effect_state_v1(
            bound,
            replay(),
            b"com",
            b"left!",
            b"right",
            EffectStateLimitsV1::default(),
            &mut output
        ),
        Ok(requirements.envelope_bytes)
    );
    assert_eq!(&output[requirements.envelope_bytes as usize..], &[0x5a; 19]);
    assert_eq!(u16::from_le_bytes(output[10..12].try_into().unwrap()), 224);
    assert_eq!(get_u32(&output, 188), 3 * 16);
    assert_eq!(
        (
            get_u32(&output, 160),
            get_u32(&output, 164),
            get_u32(&output, 168)
        ),
        (3, 5, 5)
    );
    let verified = verify_effect_state_v1(
        bound,
        &output[..requirements.envelope_bytes as usize],
        EffectStateLimitsV1::default(),
    )
    .unwrap();
    validate_effect_state_replay_v1(verified, replay()).unwrap();
    assert_eq!(verified.effect_id(), "test.state");
    assert_eq!(verified.contract_version(), (1, 7));
    assert_eq!(verified.state_layout_version(), 3);
    assert_eq!(verified.sidechain(), (2, "detector", false));
    assert_eq!(verified.initial_values().collect::<Vec<_>>(), INITIAL);
    assert_eq!(
        verified.payloads(),
        (&b"com"[..], &b"left!"[..], &b"right"[..])
    );
}

#[test]
fn every_authoring_cap_and_one_short_output_are_atomic() {
    let wire = descriptor_wire(&DESCRIPTOR);
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let requirements =
        effect_state_v1_requirements(bound, replay(), EffectStateLimitsV1::default()).unwrap();
    let baseline = vec![0x6d; requirements.envelope_bytes as usize + 8];
    let mut one_short = baseline.clone();
    let short_len = requirements.envelope_bytes as usize - 1;
    let error = encode_effect_state_v1(
        bound,
        replay(),
        b"com",
        b"left!",
        b"right",
        EffectStateLimitsV1::default(),
        &mut one_short[..short_len],
    )
    .unwrap_err();
    assert_eq!(error.code, EffectStateDiagnosticCodeV1::BufferTooSmall);
    assert_eq!(error.detail, EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT);
    assert_eq!(error.required_bytes, requirements.envelope_bytes);
    assert_eq!(one_short, baseline);

    for limits in [
        EffectStateLimitsV1 {
            maximum_descriptor_bytes: wire.len() as u64 - 1,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_envelope_bytes: requirements.envelope_bytes - 1,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_payload_bytes: requirements.payload_snapshot_scratch_bytes - 1,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_initial_values: requirements.initial_value_scratch_slots - 1,
            ..EffectStateLimitsV1::default()
        },
    ] {
        let mut output = baseline.clone();
        let error = encode_effect_state_v1(
            bound,
            replay(),
            b"com",
            b"left!",
            b"right",
            limits,
            &mut output,
        )
        .unwrap_err();
        assert_eq!(error.code, EffectStateDiagnosticCodeV1::Limit);
        assert_eq!(output, baseline);
    }

    let mut wrong_payload = baseline.clone();
    let error = encode_effect_state_v1(
        bound,
        replay(),
        b"co",
        b"left!",
        b"right",
        EffectStateLimitsV1::default(),
        &mut wrong_payload,
    )
    .unwrap_err();
    assert_eq!(error.code, EffectStateDiagnosticCodeV1::Payload);
    assert_eq!(wrong_payload, baseline);
}

#[test]
fn borrowed_verification_enforces_each_caller_cap() {
    let (wire, bytes) = encoded_state();
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    for limits in [
        EffectStateLimitsV1 {
            maximum_descriptor_bytes: wire.len() as u64 - 1,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_envelope_bytes: bytes.len() as u64 - 1,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_payload_bytes: 12,
            ..EffectStateLimitsV1::default()
        },
        EffectStateLimitsV1 {
            maximum_initial_values: 2,
            ..EffectStateLimitsV1::default()
        },
    ] {
        assert_eq!(
            verify_effect_state_v1(bound, &bytes, limits)
                .unwrap_err()
                .code,
            EffectStateDiagnosticCodeV1::Limit
        );
    }
}

#[test]
fn representative_mutations_have_exact_phase_order_and_diagnostics() {
    let (wire, original) = encoded_state();
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let mut bytes = original.clone();
    bytes[0] ^= 1;
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Header, 0)
    );

    let mut bytes = original.clone();
    bytes[12] = 1;
    bytes.truncate(bytes.len() - 1);
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Reserved, 12)
    );

    let mut bytes = original.clone();
    put_u32(&mut bytes, 104, 0);
    refresh_digest(&mut bytes);
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Enum, 104)
    );

    let mut bytes = original.clone();
    put_u32(&mut bytes, 160, 4);
    put_u32(&mut bytes, 188, 49);
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Length, 188)
    );

    let mut bytes = original.clone();
    bytes[224] = b'u';
    refresh_digest(&mut bytes);
    let verified = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap();
    let error = validate_effect_state_replay_v1(verified, replay()).unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Metadata, 1)
    );

    let mut bytes = original.clone();
    bytes[56] ^= 1;
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Digest, 56)
    );

    let mut bytes = original.clone();
    bytes[24] ^= 1;
    refresh_digest(&mut bytes);
    let error = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Descriptor, 3 << 16)
    );

    let mut bytes = original.clone();
    put_u32(&mut bytes, 248 + 4, ParameterChannel::Left as u32);
    refresh_digest(&mut bytes);
    let verified = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap();
    let error = validate_effect_state_replay_v1(verified, replay()).unwrap_err();
    assert_eq!(error.code, EffectStateDiagnosticCodeV1::InitialValues);

    let mut bytes = original.clone();
    put_u32(&mut bytes, 96, 47_999);
    refresh_digest(&mut bytes);
    let verified = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap();
    assert_eq!(verified.sample_rate(), 47_999);
    assert_eq!(
        validate_effect_state_replay_v1(verified, replay())
            .unwrap_err()
            .code,
        EffectStateDiagnosticCodeV1::Metadata
    );

    let mut bytes = original.clone();
    bytes[152..160].copy_from_slice(&18_u64.to_le_bytes());
    bytes[192..200].copy_from_slice(&63_u64.to_le_bytes());
    refresh_digest(&mut bytes);
    let verified = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap();
    let error = validate_effect_state_replay_v1(verified, replay()).unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Metadata, 11)
    );

    let error = verify_effect_state_v1(
        bound,
        &original[..original.len() - 1],
        EffectStateLimitsV1::default(),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Length, 16)
    );
    let mut trailing = original.clone();
    trailing.push(0);
    let error =
        verify_effect_state_v1(bound, &trailing, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectStateDiagnosticCodeV1::Length, 16)
    );
}
