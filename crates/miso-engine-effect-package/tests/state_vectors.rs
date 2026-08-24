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
        nudge_ladder: None,
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
        nudge_ladder: None,
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

const fn overflow_quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(9),
        tail: TailSamples::Finite(17),
        maximum_state: STATE_SIZES,
        scratch_fixed_bytes: 1,
        scratch_bytes_per_frame: u64::MAX,
    }
}

static OVERFLOW_QUALITIES: [QualityDescriptorV1; 4] = [
    overflow_quality(44_100),
    overflow_quality(48_000),
    overflow_quality(88_200),
    overflow_quality(96_000),
];

static OVERFLOW_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.state-overflow"),
    display_name: "State overflow test",
    qualities: &OVERFLOW_QUALITIES,
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

fn fixture_hex(value: &str) -> Vec<u8> {
    let digits: Vec<_> = value
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    assert_eq!(digits.len() % 2, 0);
    digits
        .chunks_exact(2)
        .map(|pair| {
            let high = (pair[0] as char).to_digit(16).unwrap() as u8;
            let low = (pair[1] as char).to_digit(16).unwrap() as u8;
            (high << 4) | low
        })
        .collect()
}

#[test]
fn independent_reference_vector_binds_verifies_and_reencodes_byte_identically() {
    let descriptor_fixture = fixture_hex(include_str!(
        "../../../fixtures/effect-state/v1/canonical.descriptor.wire.hex"
    ));
    let identity_fixture = fixture_hex(include_str!(
        "../../../fixtures/effect-state/v1/canonical.descriptor.identity.hex"
    ));
    let state_fixture = include_bytes!("../../../fixtures/effect-state/v1/canonical.state.bin");
    let state_hex_fixture = fixture_hex(include_str!(
        "../../../fixtures/effect-state/v1/canonical.state.hex"
    ));
    let digest_fixture = fixture_hex(include_str!(
        "../../../fixtures/effect-state/v1/canonical.state.digest.hex"
    ));
    let rust_wire = descriptor_wire(&DESCRIPTOR);
    assert_eq!(rust_wire, descriptor_fixture);
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &descriptor_fixture, 1 << 20).unwrap();
    assert_eq!(bound.identity().as_bytes(), identity_fixture.as_slice());
    assert_eq!(&state_fixture[24..56], identity_fixture.as_slice());
    assert_eq!(state_fixture.as_slice(), state_hex_fixture);
    assert_eq!(&state_fixture[56..88], digest_fixture);

    let verified =
        verify_effect_state_v1(bound, state_fixture, EffectStateLimitsV1::default()).unwrap();
    validate_effect_state_current_layout_v1(verified).unwrap();
    validate_effect_state_replay_v1(verified, replay()).unwrap();
    assert_eq!(
        verified.payloads(),
        (&b"com"[..], &b"left!"[..], &b"right"[..])
    );
    assert_eq!(verified.initial_values().collect::<Vec<_>>(), INITIAL);

    let requirements =
        effect_state_v1_requirements(bound, replay(), EffectStateLimitsV1::default()).unwrap();
    assert_eq!(requirements.envelope_bytes, state_fixture.len() as u64);
    let mut encoded = vec![0xa5; requirements.envelope_bytes as usize];
    encode_effect_state_v1(
        bound,
        replay(),
        b"com",
        b"left!",
        b"right",
        EffectStateLimitsV1::default(),
        &mut encoded,
    )
    .unwrap();
    assert_eq!(encoded, state_fixture);
}

#[test]
fn independent_reference_malformed_oracle_matches_exact_diagnostics() {
    let descriptor_fixture = fixture_hex(include_str!(
        "../../../fixtures/effect-state/v1/canonical.descriptor.wire.hex"
    ));
    let state = include_bytes!("../../../fixtures/effect-state/v1/canonical.state.bin");
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &descriptor_fixture, 1 << 20).unwrap();
    let initial_start = 248;
    let mut cases: Vec<(&str, Vec<u8>, EffectStateDiagnosticV1)> = Vec::new();
    cases.push((
        "truncated-header",
        state[..223].to_vec(),
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Header,
            0,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            223,
        ),
    ));
    for (name, mutate, expected) in [
        (
            "magic",
            (0_usize, 1_u8),
            (
                EffectStateDiagnosticCodeV1::Header,
                0,
                EFFECT_STATE_V1_UNAVAILABLE_INDEX,
                0_u64,
            ),
        ),
        (
            "reserved-flags",
            (12, 1),
            (
                EffectStateDiagnosticCodeV1::Reserved,
                0,
                EFFECT_STATE_V1_UNAVAILABLE_INDEX,
                12,
            ),
        ),
        (
            "initial-reserved",
            (initial_start + 12, 1),
            (
                EffectStateDiagnosticCodeV1::Reserved,
                0,
                0,
                (initial_start + 12) as u64,
            ),
        ),
        (
            "digest",
            (56, state[56] ^ 1),
            (
                EffectStateDiagnosticCodeV1::Digest,
                0,
                EFFECT_STATE_V1_UNAVAILABLE_INDEX,
                56,
            ),
        ),
    ] {
        let mut bytes = state.to_vec();
        bytes[mutate.0] = mutate.1;
        cases.push((
            name,
            bytes,
            EffectStateDiagnosticV1::new(expected.0, expected.1, expected.2, expected.3),
        ));
    }
    let mut length = state.to_vec();
    length[16..24].copy_from_slice(&((state.len() + 1) as u64).to_le_bytes());
    cases.push((
        "total-length",
        length,
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Length,
            0,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            16,
        ),
    ));
    let mut quality = state.to_vec();
    put_u32(&mut quality, 104, 99);
    cases.push((
        "quality-enum",
        quality,
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Enum,
            0,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            104,
        ),
    ));
    let mut text = state.to_vec();
    text[224] = b'T';
    cases.push((
        "effect-text",
        text,
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Text,
            0,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            124,
        ),
    ));
    let mut order = state.to_vec();
    put_u32(&mut order, initial_start + 16 + 4, 3);
    cases.push((
        "initial-order",
        order,
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Order,
            0,
            2,
            (initial_start + 32) as u64,
        ),
    ));
    let mut identity = state.to_vec();
    identity[24] ^= 1;
    refresh_digest(&mut identity);
    cases.push((
        "descriptor-identity",
        identity,
        EffectStateDiagnosticV1::new(
            EffectStateDiagnosticCodeV1::Descriptor,
            3 << 16,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
        ),
    ));

    for (name, bytes, expected) in cases {
        let actual =
            verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap_err();
        assert_eq!(actual, expected, "{name}");
    }
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
fn checked_scratch_arithmetic_reports_exact_overflow_before_layout() {
    let wire = descriptor_wire(&OVERFLOW_DESCRIPTOR);
    let bound = bind_effect_descriptor_wire_v1(&OVERFLOW_DESCRIPTOR, &wire, 1 << 20).unwrap();
    let replay = EffectStateReplayViewV1 {
        effect_id: OVERFLOW_DESCRIPTOR.id,
        request: replay().request,
    };
    let error =
        effect_state_v1_requirements(bound, replay, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (
            error.code,
            error.detail,
            error.item_index,
            error.byte_offset,
            error.required_bytes,
        ),
        (
            EffectStateDiagnosticCodeV1::Overflow,
            0,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            176,
            0,
        )
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
fn structural_selector_is_diagnostic_equivalent_until_descriptor_binding() {
    let (wire, original) = encoded_state();
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let limits = EffectStateLimitsV1::default();
    assert_eq!(
        inspect_effect_state_selector_v1(&original, limits).unwrap(),
        effect_state_bound_selector_v1(bound)
    );

    for length in 0..original.len() {
        let bytes = &original[..length];
        assert_eq!(
            inspect_effect_state_selector_v1(bytes, limits).unwrap_err(),
            verify_effect_state_v1(bound, bytes, limits).unwrap_err(),
            "truncation {length}"
        );
    }
    let mut trailing = original.clone();
    trailing.push(0);
    assert_eq!(
        inspect_effect_state_selector_v1(&trailing, limits).unwrap_err(),
        verify_effect_state_v1(bound, &trailing, limits).unwrap_err()
    );
    for offset in 0..original.len() {
        let mut mutated = original.clone();
        mutated[offset] ^= 1;
        assert_eq!(
            inspect_effect_state_selector_v1(&mutated, limits).unwrap_err(),
            verify_effect_state_v1(bound, &mutated, limits).unwrap_err(),
            "single-byte mutation at {offset}"
        );
    }

    let mut changed_identity = original.clone();
    changed_identity[24] ^= 1;
    refresh_digest(&mut changed_identity);
    let selector = inspect_effect_state_selector_v1(&changed_identity, limits).unwrap();
    assert_ne!(selector.descriptor_identity(), bound.identity());
    assert_eq!(selector.state_layout_version(), 3);
    assert_eq!(
        verify_effect_state_v1(bound, &changed_identity, limits)
            .unwrap_err()
            .code,
        EffectStateDiagnosticCodeV1::Descriptor
    );
}

fn migration_descriptor(layout: u32, sizes: StatePayloadSizes) -> &'static EffectDescriptorV1 {
    let qualities = QUALITIES
        .iter()
        .copied()
        .map(|quality| QualityDescriptorV1 {
            maximum_state: sizes,
            ..quality
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Box::leak(Box::new(EffectDescriptorV1 {
        state_layout_version: layout,
        qualities: Box::leak(qualities),
        ..DESCRIPTOR
    }))
}

fn descriptor_with_parameters(
    layout: u32,
    sizes: StatePayloadSizes,
    parameters: [ParameterDescriptorV1; 2],
) -> &'static EffectDescriptorV1 {
    let descriptor = migration_descriptor(layout, sizes);
    Box::leak(Box::new(EffectDescriptorV1 {
        parameters: Box::leak(Box::new(parameters)),
        ..*descriptor
    }))
}

fn assert_incompatible_edge(
    source_bound: BoundEffectDescriptorWireV1<'_>,
    descriptor: &'static EffectDescriptorV1,
) {
    let wire = descriptor_wire(descriptor);
    let bound = bind_effect_descriptor_wire_v1(descriptor, &wire, 1 << 20).unwrap();
    assert_eq!(
        bind_effect_state_migration_edge_v1(source_bound, bound).unwrap_err(),
        EffectStateMigrationEdgeErrorV1::IncompatibleReplayDescriptor
    );
}

#[test]
fn migration_edges_are_adjacent_compatible_and_preserve_exact_provenance() {
    let source = migration_descriptor(
        2,
        StatePayloadSizes {
            common_bytes: 1,
            left_bytes: 4,
            right_bytes: 4,
        },
    );
    let source_wire = descriptor_wire(source);
    let target_wire = descriptor_wire(&DESCRIPTOR);
    let source_bound = bind_effect_descriptor_wire_v1(source, &source_wire, 1 << 20).unwrap();
    let target_bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &target_wire, 1 << 20).unwrap();
    let edge = bind_effect_state_migration_edge_v1(source_bound, target_bound).unwrap();
    assert_eq!(
        edge.source_selector(),
        effect_state_bound_selector_v1(source_bound)
    );
    assert_eq!(
        edge.target_selector(),
        effect_state_bound_selector_v1(target_bound)
    );
    assert_ne!(edge.source_selector(), edge.target_selector());
    assert_eq!(edge.source_bound().wire(), source_wire);
    assert_eq!(edge.target_bound().wire(), target_wire);
    assert_eq!(
        effect_state_descriptor_provenance_v1(target_bound),
        effect_state_descriptor_provenance_v1(target_bound)
    );

    let identical_static = Box::leak(Box::new(DESCRIPTOR));
    let identical_bound =
        bind_effect_descriptor_wire_v1(identical_static, &target_wire, 1 << 20).unwrap();
    assert_eq!(identical_bound.identity(), target_bound.identity());
    assert_ne!(
        effect_state_descriptor_provenance_v1(identical_bound),
        effect_state_descriptor_provenance_v1(target_bound)
    );

    let nonadjacent = migration_descriptor(1, STATE_SIZES);
    let nonadjacent_wire = descriptor_wire(nonadjacent);
    let nonadjacent_bound =
        bind_effect_descriptor_wire_v1(nonadjacent, &nonadjacent_wire, 1 << 20).unwrap();
    assert_eq!(
        bind_effect_state_migration_edge_v1(nonadjacent_bound, target_bound).unwrap_err(),
        EffectStateMigrationEdgeErrorV1::NonAdjacentLayout
    );
    assert_eq!(
        bind_effect_state_migration_edge_v1(target_bound, target_bound).unwrap_err(),
        EffectStateMigrationEdgeErrorV1::NonAdjacentLayout
    );

    for descriptor in [
        Box::leak(Box::new(EffectDescriptorV1 {
            id: effect_id("test.other-state"),
            ..DESCRIPTOR
        })) as &'static EffectDescriptorV1,
        Box::leak(Box::new(EffectDescriptorV1 {
            contract_minor: 8,
            ..DESCRIPTOR
        })),
    ] {
        let wire = descriptor_wire(descriptor);
        let bound = bind_effect_descriptor_wire_v1(descriptor, &wire, 1 << 20).unwrap();
        assert_eq!(
            bind_effect_state_migration_edge_v1(source_bound, bound).unwrap_err(),
            EffectStateMigrationEdgeErrorV1::EffectOrContractMismatch
        );
    }

    let mut changed_parameter = PARAMETERS;
    changed_parameter[0].default_value = 0.5;
    let mut changed_port = PORTS;
    changed_port[2].required = true;
    let mut changed_quality = QUALITIES;
    changed_quality[0].latency = LatencySamples(10);
    for descriptor in [
        Box::leak(Box::new(EffectDescriptorV1 {
            display_name: "Changed",
            ..DESCRIPTOR
        })) as &'static EffectDescriptorV1,
        Box::leak(Box::new(EffectDescriptorV1 {
            supported_link_modes: LinkModeSet::DUAL_MONO,
            ..DESCRIPTOR
        })),
        Box::leak(Box::new(EffectDescriptorV1 {
            parameters: Box::leak(Box::new(changed_parameter)),
            ..DESCRIPTOR
        })),
        Box::leak(Box::new(EffectDescriptorV1 {
            ports: Box::leak(Box::new(changed_port)),
            ..DESCRIPTOR
        })),
        Box::leak(Box::new(EffectDescriptorV1 {
            qualities: Box::leak(Box::new(changed_quality)),
            ..DESCRIPTOR
        })),
    ] {
        let wire = descriptor_wire(descriptor);
        let bound = bind_effect_descriptor_wire_v1(descriptor, &wire, 1 << 20).unwrap();
        assert_eq!(
            bind_effect_state_migration_edge_v1(source_bound, bound).unwrap_err(),
            EffectStateMigrationEdgeErrorV1::IncompatibleReplayDescriptor
        );
    }

    // Every independently variable parameter field is forbidden. Fields coupled by descriptor
    // validity (domain/minimum/maximum/mapping and automation/smoothing) use a valid bundle while
    // the equality implementation compares every member separately with f32 `to_bits()`.
    let parameter_mutations = [
        ParameterDescriptorV1 {
            id: ParameterId(3),
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            display_name: "Renamed",
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            display_unit: "ratio",
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            unit: ParameterUnit::Ratio,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            minimum: Some(-3.0),
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            maximum: Some(3.0),
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            default_value: 0.25,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            mapping: ParameterMapping::Exponential,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            automation_rate: AutomationRate::Sample,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            channel_policy: ParameterChannelPolicy::Shared,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            smoothing: SmoothingRule::Linear,
            smoothing_samples: 1,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            readable: false,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            automation_rate: AutomationRate::None,
            automatable: false,
            ..PARAMETERS[1]
        },
        ParameterDescriptorV1 {
            domain: ParameterDomain::Boolean,
            minimum: None,
            maximum: None,
            default_value: 1.0,
            mapping: ParameterMapping::Stepped,
            ..PARAMETERS[1]
        },
    ];
    for mutation in parameter_mutations {
        assert_incompatible_edge(
            source_bound,
            descriptor_with_parameters(3, STATE_SIZES, [PARAMETERS[0], mutation]),
        );
    }

    static ENUM_CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            value: 0.0,
            label: "zero",
        },
        EnumChoiceV1 {
            value: 1.0,
            label: "one",
        },
    ];
    static CHANGED_ENUM_CHOICES: [EnumChoiceV1; 2] = [
        EnumChoiceV1 {
            value: 0.0,
            label: "none",
        },
        EnumChoiceV1 {
            value: 1.0,
            label: "one",
        },
    ];
    let enum_parameter = ParameterDescriptorV1 {
        domain: ParameterDomain::Enumeration,
        minimum: None,
        maximum: None,
        default_value: 0.0,
        mapping: ParameterMapping::Stepped,
        enum_choices: &ENUM_CHOICES,
        ..PARAMETERS[1]
    };
    let enum_source = descriptor_with_parameters(
        2,
        StatePayloadSizes {
            common_bytes: 1,
            left_bytes: 4,
            right_bytes: 4,
        },
        [PARAMETERS[0], enum_parameter],
    );
    let enum_source_wire = descriptor_wire(enum_source);
    let enum_source_bound =
        bind_effect_descriptor_wire_v1(enum_source, &enum_source_wire, 1 << 20).unwrap();
    assert_incompatible_edge(
        enum_source_bound,
        descriptor_with_parameters(
            3,
            STATE_SIZES,
            [
                PARAMETERS[0],
                ParameterDescriptorV1 {
                    enum_choices: &CHANGED_ENUM_CHOICES,
                    ..enum_parameter
                },
            ],
        ),
    );

    let mut changed_port_id = PORTS;
    changed_port_id[2].id = port_id("alternate-detector");
    let mut changed_port_required = PORTS;
    changed_port_required[2].required = true;
    for ports in [changed_port_id, changed_port_required] {
        assert_incompatible_edge(
            source_bound,
            Box::leak(Box::new(EffectDescriptorV1 {
                ports: Box::leak(Box::new(ports)),
                ..DESCRIPTOR
            })),
        );
    }

    for mutation in [
        QualityDescriptorV1 {
            latency: LatencySamples(10),
            ..QUALITIES[0]
        },
        QualityDescriptorV1 {
            tail: TailSamples::Infinite,
            ..QUALITIES[0]
        },
        QualityDescriptorV1 {
            scratch_fixed_bytes: 12,
            ..QUALITIES[0]
        },
        QualityDescriptorV1 {
            scratch_bytes_per_frame: 3,
            ..QUALITIES[0]
        },
    ] {
        let mut qualities = QUALITIES;
        qualities[0] = mutation;
        assert_incompatible_edge(
            source_bound,
            Box::leak(Box::new(EffectDescriptorV1 {
                qualities: Box::leak(Box::new(qualities)),
                ..DESCRIPTOR
            })),
        );
    }

    let source_sizes = StatePayloadSizes {
        common_bytes: 1,
        left_bytes: 4,
        right_bytes: 4,
    };
    let mut source_rates = QUALITIES
        .iter()
        .copied()
        .map(|row| QualityDescriptorV1 {
            maximum_state: source_sizes,
            ..row
        })
        .collect::<Vec<_>>();
    source_rates.push(QualityDescriptorV1 {
        sample_rate: 176_400,
        maximum_state: source_sizes,
        ..QUALITIES[0]
    });
    let mut target_rates = QUALITIES.to_vec();
    target_rates.push(QualityDescriptorV1 {
        sample_rate: 192_000,
        ..QUALITIES[0]
    });
    let rate_source = Box::leak(Box::new(EffectDescriptorV1 {
        state_layout_version: 2,
        qualities: Box::leak(source_rates.into_boxed_slice()),
        ..DESCRIPTOR
    }));
    let rate_source_wire = descriptor_wire(rate_source);
    let rate_source_bound =
        bind_effect_descriptor_wire_v1(rate_source, &rate_source_wire, 1 << 20).unwrap();
    assert_incompatible_edge(
        rate_source_bound,
        Box::leak(Box::new(EffectDescriptorV1 {
            qualities: Box::leak(target_rates.into_boxed_slice()),
            ..DESCRIPTOR
        })),
    );

    let draft_source = QUALITIES
        .iter()
        .copied()
        .map(|row| QualityDescriptorV1 {
            quality: EffectQuality::Draft,
            maximum_state: source_sizes,
            ..row
        })
        .chain(QUALITIES.iter().copied().map(|row| QualityDescriptorV1 {
            maximum_state: source_sizes,
            ..row
        }))
        .collect::<Vec<_>>();
    let normal_high_target = QUALITIES
        .iter()
        .copied()
        .chain(QUALITIES.iter().copied().map(|row| QualityDescriptorV1 {
            quality: EffectQuality::High,
            ..row
        }))
        .collect::<Vec<_>>();
    let quality_source = Box::leak(Box::new(EffectDescriptorV1 {
        state_layout_version: 2,
        qualities: Box::leak(draft_source.into_boxed_slice()),
        ..DESCRIPTOR
    }));
    let quality_source_wire = descriptor_wire(quality_source);
    let quality_source_bound =
        bind_effect_descriptor_wire_v1(quality_source, &quality_source_wire, 1 << 20).unwrap();
    assert_incompatible_edge(
        quality_source_bound,
        Box::leak(Box::new(EffectDescriptorV1 {
            qualities: Box::leak(normal_high_target.into_boxed_slice()),
            ..DESCRIPTOR
        })),
    );
}

#[test]
fn replay_configuration_is_independent_of_layout_and_prepared_resources() {
    let historical = migration_descriptor(
        2,
        StatePayloadSizes {
            common_bytes: 1,
            left_bytes: 4,
            right_bytes: 4,
        },
    );
    let wire = descriptor_wire(historical);
    let bound = bind_effect_descriptor_wire_v1(historical, &wire, 1 << 20).unwrap();
    let historical_replay = EffectStateReplayViewV1 {
        effect_id: historical.id,
        request: replay().request,
    };
    let requirements =
        effect_state_v1_requirements(bound, historical_replay, EffectStateLimitsV1::default())
            .unwrap();
    let mut bytes = vec![0; requirements.envelope_bytes as usize];
    encode_effect_state_v1(
        bound,
        historical_replay,
        b"c",
        b"left",
        b"rght",
        EffectStateLimitsV1::default(),
        &mut bytes,
    )
    .unwrap();
    let state = verify_effect_state_v1(bound, &bytes, EffectStateLimitsV1::default()).unwrap();
    validate_effect_state_current_layout_v1(state).unwrap();
    validate_effect_state_replay_configuration_v1(state, historical_replay).unwrap();
    assert_eq!(state.state_layout_version(), 2);
    assert_ne!(state.state_sizes(), DESCRIPTOR.qualities[1].maximum_state);

    for offset in [88_usize, 90] {
        let mut changed_contract = bytes.clone();
        changed_contract[offset] ^= 1;
        refresh_digest(&mut changed_contract);
        let changed_state =
            verify_effect_state_v1(bound, &changed_contract, EffectStateLimitsV1::default())
                .unwrap();
        let error = validate_effect_state_replay_configuration_v1(changed_state, historical_replay)
            .unwrap_err();
        assert_eq!(error.code, EffectStateDiagnosticCodeV1::Metadata);
        assert_eq!(error.detail, 2);
    }

    let mut changed = INITIAL;
    changed[2].value = 1.25;
    let changed_replay = EffectStateReplayViewV1 {
        effect_id: historical.id,
        request: PrepareEffectRequest {
            initial_values: &changed,
            ..historical_replay.request
        },
    };
    let error = validate_effect_state_replay_configuration_v1(state, changed_replay).unwrap_err();
    assert_eq!(error.code, EffectStateDiagnosticCodeV1::InitialValues);
    assert_eq!(error.item_index, 2);

    let assert_metadata = |candidate: EffectStateReplayViewV1<'_>, detail| {
        let error = validate_effect_state_replay_configuration_v1(state, candidate).unwrap_err();
        assert_eq!(error.code, EffectStateDiagnosticCodeV1::Metadata);
        assert_eq!(error.detail, detail);
    };
    assert_metadata(
        EffectStateReplayViewV1 {
            effect_id: effect_id("test.other-state"),
            ..historical_replay
        },
        1,
    );
    for (request, detail) in [
        (
            PrepareEffectRequest {
                sample_rate: 44_100,
                ..historical_replay.request
            },
            4,
        ),
        (
            PrepareEffectRequest {
                quantum: 9,
                ..historical_replay.request
            },
            5,
        ),
        (
            PrepareEffectRequest {
                quality: EffectQuality::High,
                ..historical_replay.request
            },
            6,
        ),
        (
            PrepareEffectRequest {
                bypass: false,
                ..historical_replay.request
            },
            7,
        ),
        (
            PrepareEffectRequest {
                link_mode: LinkMode::Average,
                ..historical_replay.request
            },
            8,
        ),
        (
            PrepareEffectRequest {
                ports: PreparedPortsV1 {
                    sidechain: PreparedSidechainPort::Unconnected {
                        id: port_id("detector"),
                        required: false,
                    },
                },
                ..historical_replay.request
            },
            9,
        ),
        (
            PrepareEffectRequest {
                ports: PreparedPortsV1 {
                    sidechain: PreparedSidechainPort::Connected {
                        id: port_id("other-detector"),
                        required: false,
                    },
                },
                ..historical_replay.request
            },
            9,
        ),
        (
            PrepareEffectRequest {
                ports: PreparedPortsV1 {
                    sidechain: PreparedSidechainPort::Connected {
                        id: port_id("detector"),
                        required: true,
                    },
                },
                ..historical_replay.request
            },
            9,
        ),
        (
            PrepareEffectRequest {
                limits: PrepareEffectLimits {
                    maximum_total_state_bytes: 63,
                    ..historical_replay.request.limits
                },
                ..historical_replay.request
            },
            15,
        ),
        (
            PrepareEffectRequest {
                limits: PrepareEffectLimits {
                    maximum_scratch_bytes: 127,
                    ..historical_replay.request.limits
                },
                ..historical_replay.request
            },
            15,
        ),
        (
            PrepareEffectRequest {
                limits: PrepareEffectLimits {
                    maximum_automation_spans_per_block: 22,
                    ..historical_replay.request.limits
                },
                ..historical_replay.request
            },
            15,
        ),
    ] {
        assert_metadata(
            EffectStateReplayViewV1 {
                request,
                ..historical_replay
            },
            detail,
        );
    }

    let mut wrong_index = INITIAL;
    wrong_index[2].parameter_index = 0;
    let mut wrong_channel = INITIAL;
    wrong_channel[2].channel = ParameterChannel::Left;
    let mut wrong_bits = INITIAL;
    wrong_bits[2].value = f32::from_bits(INITIAL[2].value.to_bits() ^ 1);
    let too_many = [INITIAL[0], INITIAL[1], INITIAL[2], INITIAL[2]];
    let initial_offset = 248_usize;
    for (values, item, offset) in [
        (&wrong_index[..], 2, (initial_offset + 32) as u64),
        (&wrong_channel[..], 2, (initial_offset + 36) as u64),
        (&wrong_bits[..], 2, (initial_offset + 40) as u64),
        (
            &INITIAL[..2],
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
        ),
        (&too_many[..], 3, (initial_offset + 48) as u64),
    ] {
        let error = validate_effect_state_replay_configuration_v1(
            state,
            EffectStateReplayViewV1 {
                request: PrepareEffectRequest {
                    initial_values: values,
                    ..historical_replay.request
                },
                ..historical_replay
            },
        )
        .unwrap_err();
        assert_eq!(error.code, EffectStateDiagnosticCodeV1::InitialValues);
        assert_eq!(error.item_index, item);
        assert_eq!(error.byte_offset, offset);
    }
}

#[test]
fn selector_and_registry_paths_perform_no_descriptor_validation_pass() {
    let state_source = include_str!("../src/state.rs");
    let selector = state_source
        .split("pub fn inspect_effect_state_selector_v1")
        .nth(1)
        .unwrap()
        .split("pub fn verify_effect_state_v1")
        .next()
        .unwrap();
    assert!(!selector.contains("validate_descriptor_v1"));
    assert!(!selector.contains("effect_descriptor_identity_v1"));
    assert!(!selector.contains("bind_effect_descriptor_wire_v1"));

    let wire_source = include_str!("../src/wire.rs");
    let binder = wire_source
        .split("pub fn bind_effect_descriptor_wire_v1")
        .nth(1)
        .unwrap()
        .split("#[cfg(test)]")
        .next()
        .unwrap();
    assert_eq!(binder.matches("validate_descriptor_v1").count(), 1);

    let compatibility = state_source
        .split("fn parameters_compatible")
        .nth(1)
        .unwrap()
        .split("pub fn effect_state_bound_selector_v1")
        .next()
        .unwrap();
    for field in [
        ".id",
        ".display_name",
        ".display_unit",
        ".unit",
        ".domain",
        ".minimum",
        ".maximum",
        ".default_value",
        ".mapping",
        ".automation_rate",
        ".channel_policy",
        ".smoothing",
        ".smoothing_samples",
        ".readable",
        ".automatable",
        ".enum_choices",
        ".quality",
        ".sample_rate",
        ".latency",
        ".tail",
        ".scratch_fixed_bytes",
        ".scratch_bytes_per_frame",
    ] {
        assert!(
            compatibility.contains(field),
            "missing compatibility field {field}"
        );
    }
    assert!(state_source.contains("source_descriptor.ports != target_descriptor.ports"));
}

#[allow(clippy::too_many_arguments)]
fn assert_verify_diagnostic(
    name: &str,
    bound: BoundEffectDescriptorWireV1<'_>,
    bytes: &[u8],
    code: EffectStateDiagnosticCodeV1,
    detail: u32,
    item_index: u32,
    byte_offset: u64,
    required_bytes: u64,
) {
    let actual = verify_effect_state_v1(bound, bytes, EffectStateLimitsV1::default()).unwrap_err();
    assert_eq!(
        (
            actual.code,
            actual.detail,
            actual.item_index,
            actual.byte_offset,
            actual.required_bytes,
        ),
        (code, detail, item_index, byte_offset, required_bytes),
        "{name}"
    );
}

fn assert_replay_metadata(
    name: &str,
    bound: BoundEffectDescriptorWireV1<'_>,
    bytes: &mut [u8],
    detail: u32,
) {
    refresh_digest(bytes);
    let verified = verify_effect_state_v1(bound, bytes, EffectStateLimitsV1::default()).unwrap();
    let actual = validate_effect_state_replay_v1(verified, replay()).unwrap_err();
    assert_eq!(
        (
            actual.code,
            actual.detail,
            actual.item_index,
            actual.byte_offset,
            actual.required_bytes,
        ),
        (
            EffectStateDiagnosticCodeV1::Metadata,
            detail,
            EFFECT_STATE_V1_UNAVAILABLE_INDEX,
            EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
            0,
        ),
        "{name}"
    );
}

#[test]
fn every_state_header_field_and_payload_class_has_an_exact_diagnostic() {
    let (wire, original) = encoded_state();
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();
    let unavailable = EFFECT_STATE_V1_UNAVAILABLE_INDEX;

    for (name, offset, size, value, code, diagnostic_offset) in [
        (
            "version",
            8,
            2,
            2_u64,
            EffectStateDiagnosticCodeV1::Header,
            8,
        ),
        (
            "header-bytes",
            10,
            2,
            223,
            EffectStateDiagnosticCodeV1::Header,
            10,
        ),
        ("flags", 12, 4, 1, EffectStateDiagnosticCodeV1::Reserved, 12),
        (
            "total",
            16,
            8,
            original.len() as u64 + 1,
            EffectStateDiagnosticCodeV1::Length,
            16,
        ),
        (
            "layout-zero",
            92,
            4,
            0,
            EffectStateDiagnosticCodeV1::Header,
            92,
        ),
        (
            "reserved-tail",
            148,
            4,
            1,
            EffectStateDiagnosticCodeV1::Reserved,
            148,
        ),
        (
            "reserved-state",
            172,
            4,
            1,
            EffectStateDiagnosticCodeV1::Reserved,
            172,
        ),
        (
            "reserved-request",
            212,
            4,
            1,
            EffectStateDiagnosticCodeV1::Reserved,
            212,
        ),
        (
            "initial-table-bytes",
            188,
            4,
            47,
            EffectStateDiagnosticCodeV1::Length,
            188,
        ),
        (
            "payload-total",
            216,
            8,
            12,
            EffectStateDiagnosticCodeV1::Length,
            216,
        ),
    ] {
        let mut bytes = original.clone();
        match size {
            2 => bytes[offset..offset + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            4 => bytes[offset..offset + 4].copy_from_slice(&(value as u32).to_le_bytes()),
            8 => bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes()),
            _ => unreachable!(),
        }
        assert_verify_diagnostic(
            name,
            bound,
            &bytes,
            code,
            0,
            unavailable,
            diagnostic_offset,
            0,
        );
    }

    for (name, offset, value, diagnostic_offset) in [
        ("quality-enum", 104, 0, 104),
        ("bypass-enum", 108, 2, 108),
        ("link-enum", 112, 0, 112),
        ("sidechain-kind-enum", 116, 3, 116),
        ("sidechain-required-enum", 120, 2, 120),
        ("tail-kind-enum", 144, 3, 144),
    ] {
        let mut bytes = original.clone();
        put_u32(&mut bytes, offset, value);
        assert_verify_diagnostic(
            name,
            bound,
            &bytes,
            EffectStateDiagnosticCodeV1::Enum,
            0,
            unavailable,
            diagnostic_offset,
            0,
        );
    }

    for (name, mutate, detail) in [
        ("contract-major", (88_usize, 2_u64, 2_usize), 2),
        ("contract-minor", (90, 8, 2), 2),
        ("layout", (92, 4, 4), 3),
        ("sample-rate", (96, 44_100, 4), 4),
        ("quantum", (100, 9, 4), 5),
        ("quality", (104, 1, 4), 6),
        ("bypass", (108, 0, 4), 7),
        ("link", (112, 3, 4), 8),
        ("sidechain-kind", (116, 1, 4), 9),
        ("sidechain-required", (120, 1, 4), 9),
        ("latency", (136, 10, 8), 10),
        ("finite-tail", (152, 18, 8), 11),
        ("scratch", (176, 28, 8), 13),
        ("automation", (184, 24, 4), 14),
        ("request-state", (192, 65, 8), 15),
        ("request-scratch", (200, 129, 8), 15),
        ("request-automation", (208, 24, 4), 15),
    ] {
        let mut bytes = original.clone();
        match mutate.2 {
            2 => bytes[mutate.0..mutate.0 + 2].copy_from_slice(&(mutate.1 as u16).to_le_bytes()),
            4 => put_u32(&mut bytes, mutate.0, mutate.1 as u32),
            8 => bytes[mutate.0..mutate.0 + 8].copy_from_slice(&mutate.1.to_le_bytes()),
            _ => unreachable!(),
        }
        assert_replay_metadata(name, bound, &mut bytes, detail);
    }

    let mut descriptor_identity = original.clone();
    descriptor_identity[24] ^= 1;
    refresh_digest(&mut descriptor_identity);
    assert_verify_diagnostic(
        "descriptor-identity",
        bound,
        &descriptor_identity,
        EffectStateDiagnosticCodeV1::Descriptor,
        3 << 16,
        unavailable,
        EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
        0,
    );
    let mut digest = original.clone();
    digest[56] ^= 1;
    assert_verify_diagnostic(
        "digest",
        bound,
        &digest,
        EffectStateDiagnosticCodeV1::Digest,
        0,
        unavailable,
        56,
        0,
    );

    let mut effect_length = original.clone();
    put_u32(&mut effect_length, 124, 11);
    assert_verify_diagnostic(
        "effect-id-bytes",
        bound,
        &effect_length,
        EffectStateDiagnosticCodeV1::Text,
        0,
        unavailable,
        128,
        0,
    );
    let mut sidechain_length = original.clone();
    put_u32(&mut sidechain_length, 128, 7);
    sidechain_length[241] = 0;
    assert_replay_metadata("sidechain-id-bytes", bound, &mut sidechain_length, 9);
    let mut initial_count = original.clone();
    put_u32(&mut initial_count, 132, 4);
    assert_verify_diagnostic(
        "initial-count",
        bound,
        &initial_count,
        EffectStateDiagnosticCodeV1::Length,
        0,
        unavailable,
        188,
        0,
    );

    for (name, offset) in [
        ("common-bytes", 160),
        ("left-bytes", 164),
        ("right-bytes", 168),
    ] {
        let mut bytes = original.clone();
        let value = get_u32(&bytes, offset) + 1;
        put_u32(&mut bytes, offset, value);
        assert_verify_diagnostic(
            name,
            bound,
            &bytes,
            EffectStateDiagnosticCodeV1::Length,
            0,
            unavailable,
            216,
            0,
        );
    }
    let mut partition_sizes = original.clone();
    put_u32(&mut partition_sizes, 160, 4);
    put_u32(&mut partition_sizes, 168, 4);
    assert_replay_metadata("state-size-partition", bound, &mut partition_sizes, 12);

    let mut padding = original.clone();
    padding[242] = 1;
    assert_verify_diagnostic(
        "padding",
        bound,
        &padding,
        EffectStateDiagnosticCodeV1::Length,
        0,
        unavailable,
        242,
        0,
    );
    for (name, byte, expected_offset) in [("effect-text", 224, 124), ("sidechain-text", 234, 128)] {
        let mut bytes = original.clone();
        bytes[byte] = b'A';
        assert_verify_diagnostic(
            name,
            bound,
            &bytes,
            EffectStateDiagnosticCodeV1::Text,
            0,
            unavailable,
            expected_offset,
            0,
        );
    }
    let mut order = original.clone();
    put_u32(&mut order, 248 + 16 + 4, ParameterChannel::Both as u32);
    assert_verify_diagnostic(
        "initial-order",
        bound,
        &order,
        EffectStateDiagnosticCodeV1::Order,
        0,
        2,
        280,
        0,
    );
    let mut initial_enum = original.clone();
    put_u32(&mut initial_enum, 248 + 4, 0);
    assert_verify_diagnostic(
        "initial-channel",
        bound,
        &initial_enum,
        EffectStateDiagnosticCodeV1::Enum,
        0,
        0,
        252,
        0,
    );
    let mut initial_value = original.clone();
    put_u32(&mut initial_value, 248 + 8, f32::NAN.to_bits());
    assert_verify_diagnostic(
        "initial-value",
        bound,
        &initial_value,
        EffectStateDiagnosticCodeV1::InitialValues,
        0,
        0,
        256,
        0,
    );
    let mut payload = original.clone();
    payload[296] ^= 1;
    assert_verify_diagnostic(
        "payload-digest",
        bound,
        &payload,
        EffectStateDiagnosticCodeV1::Digest,
        0,
        unavailable,
        56,
        0,
    );
    assert_verify_diagnostic(
        "truncation",
        bound,
        &original[..original.len() - 1],
        EffectStateDiagnosticCodeV1::Length,
        0,
        unavailable,
        16,
        0,
    );
    let mut trailing = original.clone();
    trailing.push(0);
    assert_verify_diagnostic(
        "trailing",
        bound,
        &trailing,
        EffectStateDiagnosticCodeV1::Length,
        0,
        unavailable,
        16,
        0,
    );
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

/// Audit #97 F3: the envelope must bind the effect identity it names, with no placeholder anywhere
/// on the read path. Issue 079's rewrite checks `descriptor_identity` against the bound token and
/// the effect-ID text against the bound descriptor; these are the negatives that keep both honest,
/// plus the phase order that reports a stale digest before an identity mismatch.
#[test]
fn the_state_envelope_binds_the_effect_identity_it_names() {
    let (wire, original) = encoded_state();
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &wire, 1 << 20).unwrap();

    let verified =
        verify_effect_state_v1(bound, &original, EffectStateLimitsV1::default()).unwrap();
    assert_eq!(verified.effect_id(), DESCRIPTOR.id.as_str());
    assert_eq!(verified.descriptor_identity(), bound.identity());
    assert_eq!(validate_effect_state_current_layout_v1(verified), Ok(()));

    // A different, individually valid effect ID of the same length, with the digest recomputed so
    // the envelope is internally consistent: verification accepts the bytes, and the identity
    // check is what rejects them.
    let effect_id_length = get_u32(&original, 124) as usize;
    assert_eq!(effect_id_length, DESCRIPTOR.id.as_str().len());
    let mut renamed = original.clone();
    renamed[224..224 + effect_id_length].copy_from_slice(b"test.stats");
    refresh_digest(&mut renamed);
    let renamed_view =
        verify_effect_state_v1(bound, &renamed, EffectStateLimitsV1::default()).unwrap();
    assert_eq!(renamed_view.effect_id(), "test.stats");
    for actual in [
        validate_effect_state_current_layout_v1(renamed_view).unwrap_err(),
        validate_effect_state_replay_v1(renamed_view, replay()).unwrap_err(),
    ] {
        assert_eq!(
            (
                actual.code,
                actual.detail,
                actual.item_index,
                actual.byte_offset,
                actual.required_bytes,
            ),
            (
                EffectStateDiagnosticCodeV1::Metadata,
                1,
                EFFECT_STATE_V1_UNAVAILABLE_INDEX,
                EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
                0,
            )
        );
    }

    // The same descriptor-identity flip is a `Digest` failure without the recomputed digest and a
    // `Descriptor` failure with it: the digest phase runs before the identity phase.
    let mut stale = original.clone();
    stale[24] ^= 1;
    assert_verify_diagnostic(
        "identity-flip-stale-digest",
        bound,
        &stale,
        EffectStateDiagnosticCodeV1::Digest,
        0,
        EFFECT_STATE_V1_UNAVAILABLE_INDEX,
        56,
        0,
    );
    refresh_digest(&mut stale);
    assert_verify_diagnostic(
        "identity-flip-fresh-digest",
        bound,
        &stale,
        EffectStateDiagnosticCodeV1::Descriptor,
        3 << 16,
        EFFECT_STATE_V1_UNAVAILABLE_INDEX,
        EFFECT_STATE_V1_UNAVAILABLE_OFFSET,
        0,
    );

    // No placeholder identity survives anywhere on the read path.
    let mut canonical = original.clone();
    refresh_digest(&mut canonical);
    assert_eq!(canonical, original);
}
