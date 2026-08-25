//! Descriptive benchmark for the accepted effect interchange boundary.
#![allow(missing_docs)]

use miso_engine_bench_support::json;
use miso_engine_bench_support::stats::nearest_rank;
use std::{
    env,
    hint::black_box,
    sync::{Arc, OnceLock},
    time::Instant,
};

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use miso_engine_lane::Backend;
use sha2::{Digest, Sha256};

const OBSERVATIONS: usize = 256;
const WORKLOADS: [&str; 4] = [
    "descriptor_verify_identity_a",
    "package_verify_cid_select_a",
    "state_verify_reencode_current",
    "migration_two_step_bank_restore",
];
const EXPECTED_OUTPUT_SHA256: [&str; 4] = [
    "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1",
    "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f",
    "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48",
    "5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777",
];
#[cfg(test)]
const ISSUE_081_UNREACHABLE_MIGRATION_SHA256: &str =
    "350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441";
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const EXPECTED_MIGRATION_PAYLOAD: [u8; 11] = [
    0x10, 0x82, 0x83, 0x11, 0x12, 0x82, 0x83, 0x13, 0x14, 0x82, 0x83,
];

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("port id"),
    }
}

fn hex_bytes(text: &str) -> Vec<u8> {
    let compact: Vec<_> = text
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    assert_eq!(compact.len() % 2, 0, "hex fixture length");
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("lowercase hex fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn digest_hex(bytes: &[u8]) -> String {
    digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

static STATE_PARAMETERS: [ParameterDescriptorV1; 2] = [
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
static STATE_PORTS: [PortDescriptorV1; 3] = [
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
const fn state_quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(9),
        tail: TailSamples::Finite(17),
        maximum_state: StatePayloadSizes {
            common_bytes: 3,
            left_bytes: 5,
            right_bytes: 5,
        },
        scratch_fixed_bytes: 11,
        scratch_bytes_per_frame: 2,
    }
}
static STATE_QUALITIES: [QualityDescriptorV1; 4] = [
    state_quality(44_100),
    state_quality(48_000),
    state_quality(88_200),
    state_quality(96_000),
];
static STATE_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.state"),
    display_name: "State test",
    contract_major: 1,
    contract_minor: 7,
    state_layout_version: 3,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &STATE_PARAMETERS,
    ports: &STATE_PORTS,
    qualities: &STATE_QUALITIES,
    observations: &[],
};
static STATE_INITIAL: [InitialParameterValue; 3] = [
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

fn state_replay() -> EffectStateReplayViewV1<'static> {
    EffectStateReplayViewV1 {
        effect_id: STATE_DESCRIPTOR.id,
        request: PrepareEffectRequest {
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
            initial_values: &STATE_INITIAL,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 64,
                maximum_scratch_bytes: 128,
                maximum_automation_spans_per_block: 23,
            },
        },
    }
}

static MIGRATION_PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
    id: ParameterId(1),
    display_name: "value",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(-1.0),
    maximum: Some(1.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::PerLane,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
}];
static MIGRATION_PORTS: [PortDescriptorV1; 2] = [
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
];
const fn migration_sizes(layout: u32) -> StatePayloadSizes {
    StatePayloadSizes {
        common_bytes: layout,
        left_bytes: layout + 1,
        right_bytes: layout + 1,
    }
}
const fn migration_quality(rate: u32, layout: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state: migration_sizes(layout),
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}
static MIGRATION_Q1: [QualityDescriptorV1; 4] = [
    migration_quality(44_100, 1),
    migration_quality(48_000, 1),
    migration_quality(88_200, 1),
    migration_quality(96_000, 1),
];
static MIGRATION_Q2: [QualityDescriptorV1; 4] = [
    migration_quality(44_100, 2),
    migration_quality(48_000, 2),
    migration_quality(88_200, 2),
    migration_quality(96_000, 2),
];
static MIGRATION_Q3: [QualityDescriptorV1; 4] = [
    migration_quality(44_100, 3),
    migration_quality(48_000, 3),
    migration_quality(88_200, 3),
    migration_quality(96_000, 3),
];
static MIGRATION_D1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("bench.migration"),
    display_name: "Benchmark migration",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &MIGRATION_PARAMETERS,
    ports: &MIGRATION_PORTS,
    qualities: &MIGRATION_Q1,
    observations: &[],
};
static MIGRATION_D2: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 2,
    qualities: &MIGRATION_Q2,
    ..MIGRATION_D1
};
static MIGRATION_D3: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 3,
    qualities: &MIGRATION_Q3,
    ..MIGRATION_D1
};
static MIGRATION_INITIAL: [InitialParameterValue; 2] = [
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Left,
        value: -0.25,
    },
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Right,
        value: 0.75,
    },
];

fn migration_payload(layout: u32, seed: u8) -> Vec<u8> {
    (0..migration_sizes(layout).total().expect("migration sizes") as u8)
        .map(|value| seed.wrapping_add(value))
        .collect()
}

fn expected_migrated_payload(mut payload: Vec<u8>) -> Vec<u8> {
    for target_layout in 2..=3 {
        let source_sizes = migration_sizes(target_layout - 1);
        let common = source_sizes.common_bytes as usize;
        let left = source_sizes.left_bytes as usize;
        let sections = [
            payload[..common].to_vec(),
            payload[common..common + left].to_vec(),
            payload[common + left..].to_vec(),
        ];
        payload.clear();
        for section in sections {
            payload.extend_from_slice(&section);
            payload.push(0x80 | target_layout as u8);
        }
    }
    payload
}

fn copy_payload(payload: &[u8], sizes: StatePayloadSizes, output: StatePayloadOutput<'_>) {
    let common = sizes.common_bytes as usize;
    let left = sizes.left_bytes as usize;
    output.common.copy_from_slice(&payload[..common]);
    output.left.copy_from_slice(&payload[common..common + left]);
    output.right.copy_from_slice(&payload[common + left..]);
}

struct MigrationFactory;
struct MigrationBank {
    metadata: PreparedBankMetadata,
    payloads: Vec<Vec<u8>>,
}

impl NativeEffectFactory for MigrationFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &MIGRATION_D3
    }

    fn prepare(
        &self,
        _: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        Err(EffectPrepareError {
            code: "benchmark.bank-only",
        })
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(&MIGRATION_D3, request.requests[0])?;
        Ok(Some(Box::new(MigrationBank {
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: metadata.program_key(),
            },
            payloads: (0..request.width.lanes())
                .map(|index| migration_payload(3, 0x20 + index as u8 * 0x10))
                .collect(),
        })))
    }
}

impl PreparedNativeEffectBank for MigrationBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, _: ResetKind) {}

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        BankProcessReport::empty(block.width)
    }

    fn snapshot_track_state_payload(
        &self,
        index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let payload = self
            .payloads
            .get(index as usize)
            .ok_or(StatePayloadError { code: "track" })?;
        copy_payload(payload, self.metadata.program_key.state_sizes, output);
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != 3 {
            return Err(StatePayloadError { code: "version" });
        }
        let payload = self
            .payloads
            .get_mut(index as usize)
            .ok_or(StatePayloadError { code: "track" })?;
        payload.clear();
        payload.extend_from_slice(input.common);
        payload.extend_from_slice(input.left);
        payload.extend_from_slice(input.right);
        Ok(())
    }
}

struct GrowStep;
impl EffectStateMigrationStepV1 for GrowStep {
    fn scratch_bytes(&self) -> u64 {
        3
    }

    fn migrate(
        &self,
        source_layout: u32,
        target_layout: u32,
        source: StatePayloadInput<'_>,
        target: StatePayloadOutput<'_>,
        scratch: &mut [u8],
    ) -> Result<EffectStateMigrationStepReportV1, EffectStateMigrationStepFailureV1> {
        assert_eq!(target_layout, source_layout + 1);
        assert_eq!(scratch.len(), 3);
        scratch.fill(target_layout as u8);
        for (from, to) in [
            (source.common, target.common),
            (source.left, target.left),
            (source.right, target.right),
        ] {
            to[..from.len()].copy_from_slice(from);
            to[from.len()..].fill(0x80 | target_layout as u8);
        }
        let sizes = migration_sizes(target_layout);
        Ok(EffectStateMigrationStepReportV1 {
            common_bytes: sizes.common_bytes,
            left_bytes: sizes.left_bytes,
            right_bytes: sizes.right_bytes,
            reserved: 0,
        })
    }
}

fn descriptor_wire(descriptor: &'static EffectDescriptorV1) -> &'static [u8] {
    static WIRES: OnceLock<[Vec<u8>; 3]> = OnceLock::new();
    let wires = WIRES.get_or_init(|| {
        [&MIGRATION_D1, &MIGRATION_D2, &MIGRATION_D3].map(|value| {
            let required = effect_descriptor_wire_v1_required_size(value, 1 << 20)
                .expect("descriptor requirements");
            let mut wire = vec![0; required as usize];
            encode_effect_descriptor_wire_v1(value, 1 << 20, &mut wire)
                .expect("descriptor encoding");
            wire
        })
    });
    match descriptor.state_layout_version {
        1 => &wires[0],
        2 => &wires[1],
        3 => &wires[2],
        _ => panic!("benchmark migration layout"),
    }
}

fn bound_descriptor(
    descriptor: &'static EffectDescriptorV1,
) -> BoundEffectDescriptorWireV1<'static> {
    bind_effect_descriptor_wire_v1(descriptor, descriptor_wire(descriptor), 1 << 20)
        .expect("bound descriptor")
}

fn migration_replay() -> EffectBankPreparationV1 {
    EffectBankPreparationV1 {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Average,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: MIGRATION_INITIAL.into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 64,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 8,
        },
    }
}

fn restore_admission() -> EffectStateRestoreAdmissionV1 {
    EffectStateRestoreAdmissionV1 {
        sample_rate: 48_000,
        quantum: 128,
        maximum_total_state_bytes: 64,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 8,
    }
}

fn migration_envelope() -> Vec<u8> {
    let replay = migration_replay();
    let bound = bound_descriptor(&MIGRATION_D1);
    let requirements = effect_state_v1_requirements(
        bound,
        replay.state_replay(MIGRATION_D1.id),
        EffectStateLimitsV1::default(),
    )
    .expect("source state requirements");
    let payload = migration_payload(1, 0x10);
    let sizes = migration_sizes(1);
    let common = sizes.common_bytes as usize;
    let left = sizes.left_bytes as usize;
    let mut envelope = vec![0; requirements.envelope_bytes as usize];
    encode_effect_state_v1(
        bound,
        replay.state_replay(MIGRATION_D1.id),
        &payload[..common],
        &payload[common..common + left],
        &payload[common + left..],
        EffectStateLimitsV1::default(),
        &mut envelope,
    )
    .expect("source state encode");
    envelope
}

fn validate_migration_descriptors() {
    for (descriptor, layout) in [(&MIGRATION_D1, 1), (&MIGRATION_D2, 2), (&MIGRATION_D3, 3)] {
        assert_eq!(descriptor.state_layout_version, layout);
        assert_eq!(descriptor.qualities.len(), LAUNCH_RATES.len());
        for (quality, rate) in descriptor.qualities.iter().zip(LAUNCH_RATES) {
            assert_eq!(quality.quality, EffectQuality::Normal);
            assert_eq!(quality.sample_rate, rate);
            assert_eq!(quality.latency, LatencySamples(0));
            assert_eq!(quality.tail, TailSamples::Finite(0));
            assert_eq!(quality.maximum_state, migration_sizes(layout));
            assert_eq!(quality.scratch_fixed_bytes, 2);
            assert_eq!(quality.scratch_bytes_per_frame, 0);
        }
        let wire = descriptor_wire(descriptor);
        verify_effect_descriptor_wire_v1(wire, 1 << 20).expect("migration descriptor wire");
        let bound = bound_descriptor(descriptor);
        assert_eq!(bound.wire(), wire);
    }
}

trait MigrationTimer {
    fn start(&mut self);
    fn finish(&mut self) -> u64;
}

struct WallMigrationTimer(Option<Instant>);

impl MigrationTimer for WallMigrationTimer {
    fn start(&mut self) {
        self.0 = Some(Instant::now());
    }

    fn finish(&mut self) -> u64 {
        u64::try_from(
            self.0
                .take()
                .expect("migration timer started")
                .elapsed()
                .as_nanos()
                .max(1),
        )
        .expect("nanoseconds fit u64")
    }
}

#[cfg(test)]
struct UntimedMigration;

#[cfg(test)]
impl MigrationTimer for UntimedMigration {
    fn start(&mut self) {}

    fn finish(&mut self) -> u64 {
        0
    }
}

fn snapshot_bank_member(bank: &UnpublishedEffectBankStateV1<'_>, index: u32) -> Vec<u8> {
    let requirements = scalar_effect_state_v1_requirements(
        bank.bound_factory(),
        &bank.replays()[index as usize],
        EffectStateLimitsV1::default(),
    )
    .expect("member snapshot requirements");
    let mut payload = vec![0; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; requirements.envelope_bytes as usize];
    snapshot_unpublished_effect_bank_track_state_v1(
        bank,
        index,
        EffectStateLimitsV1::default(),
        &mut payload,
        &mut output,
    )
    .expect("member snapshot");
    output
}

fn execute_migration<T: MigrationTimer>(mut timer: T) -> (u64, Vec<u8>) {
    validate_migration_descriptors();
    let registry = StateMigrationRegistryV1::new(
        2,
        vec![
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound_descriptor(&MIGRATION_D1),
                bound_descriptor(&MIGRATION_D2),
                Arc::new(GrowStep),
            ),
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound_descriptor(&MIGRATION_D2),
                bound_descriptor(&MIGRATION_D3),
                Arc::new(GrowStep),
            ),
        ]
        .into_boxed_slice(),
    )
    .expect("migration registry");
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(MigrationFactory);
    let resolve_factory = bind_native_effect_factory_state_v1(
        Arc::clone(&factory),
        descriptor_wire(&MIGRATION_D3),
        1 << 20,
    )
    .expect("resolve factory");
    let destination_factory =
        bind_native_effect_factory_state_v1(factory, descriptor_wire(&MIGRATION_D3), 1 << 20)
            .expect("destination factory");
    let replay = migration_replay();
    let source = migration_envelope();
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &resolve_factory,
        &source,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_chain_steps: 2,
            maximum_intermediate_envelope_bytes: 1 << 20,
            maximum_migration_scratch_bytes: 1 << 20,
        },
        restore_admission(),
    )
    .expect("migration resolution");
    assert_eq!(resolved.chain_step_count(), 2);
    let requirements = resolved.requirements();
    let mut first = vec![0; requirements.first_envelope_bytes as usize];
    let mut second = vec![0; requirements.second_envelope_bytes as usize];
    let mut scratch = vec![0; requirements.migration_scratch_bytes as usize];
    let bank = prepare_unpublished_effect_bank_state_v1(
        destination_factory,
        Backend::Simd4,
        BankWidth::Four,
        (0..4)
            .map(|_| replay.clone())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        restore_admission(),
    )
    .expect("destination bank");
    let snapshot_requirements = scalar_effect_state_v1_requirements(
        bank.bound_factory(),
        &bank.replays()[1],
        EffectStateLimitsV1::default(),
    )
    .expect("snapshot requirements");
    let mut payload = vec![0; snapshot_requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; snapshot_requirements.envelope_bytes as usize];
    timer.start();
    let bank = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        1,
        &mut first,
        &mut second,
        &mut scratch,
    )
    .expect("bank migration restore");
    snapshot_unpublished_effect_bank_track_state_v1(
        &bank,
        1,
        EffectStateLimitsV1::default(),
        &mut payload,
        &mut output,
    )
    .expect("final bank snapshot");
    let elapsed = timer.finish();
    let verified = verify_effect_state_v1(
        bound_descriptor(&MIGRATION_D3),
        &output,
        EffectStateLimitsV1::default(),
    )
    .expect("final migrated state");
    validate_effect_state_current_layout_v1(verified).expect("final current layout");
    validate_effect_state_replay_v1(verified, migration_replay().state_replay(MIGRATION_D3.id))
        .expect("final replay");
    let (common, left, right) = verified.payloads();
    let final_payload = [common, left, right].concat();
    assert_eq!(
        expected_migrated_payload(migration_payload(1, 0x10)),
        EXPECTED_MIGRATION_PAYLOAD
    );
    assert_eq!(final_payload, EXPECTED_MIGRATION_PAYLOAD);
    let sizes = migration_sizes(3);
    let common_bytes = sizes.common_bytes as usize;
    let left_bytes = sizes.left_bytes as usize;
    let mut expected_envelope = vec![0; output.len()];
    encode_effect_state_v1(
        bound_descriptor(&MIGRATION_D3),
        migration_replay().state_replay(MIGRATION_D3.id),
        &EXPECTED_MIGRATION_PAYLOAD[..common_bytes],
        &EXPECTED_MIGRATION_PAYLOAD[common_bytes..common_bytes + left_bytes],
        &EXPECTED_MIGRATION_PAYLOAD[common_bytes + left_bytes..],
        EffectStateLimitsV1::default(),
        &mut expected_envelope,
    )
    .expect("expected final envelope");
    assert_eq!(output.len(), 283);
    assert_eq!(output, expected_envelope);
    for (index, seed) in [(0, 0x20), (2, 0x40), (3, 0x50)] {
        let sibling = snapshot_bank_member(&bank, index);
        let sibling_state = verify_effect_state_v1(
            bound_descriptor(&MIGRATION_D3),
            &sibling,
            EffectStateLimitsV1::default(),
        )
        .expect("unaffected sibling state");
        validate_effect_state_current_layout_v1(sibling_state)
            .expect("unaffected sibling current layout");
        validate_effect_state_replay_v1(
            sibling_state,
            migration_replay().state_replay(MIGRATION_D3.id),
        )
        .expect("unaffected sibling replay");
        let (common, left, right) = sibling_state.payloads();
        assert_eq!([common, left, right].concat(), migration_payload(3, seed));
    }
    (elapsed, output)
}

fn migration_workload() -> (u64, Vec<u8>) {
    execute_migration(WallMigrationTimer(None))
}

fn descriptor_workload(wire: &[u8]) -> (u64, Vec<u8>) {
    let start = Instant::now();
    let verified =
        verify_effect_descriptor_wire_v1(wire, 1 << 22).expect("accepted descriptor fixture");
    let identity =
        effect_descriptor_identity_v1(wire, 1 << 22).expect("accepted descriptor identity");
    let elapsed = u64::try_from(start.elapsed().as_nanos().max(1)).expect("nanoseconds fit u64");
    assert_eq!(verified.as_bytes(), wire);
    let output = identity.as_bytes().to_vec();
    (elapsed, output)
}

fn package_workload(package: &[u8]) -> (u64, Vec<u8>) {
    let start = Instant::now();
    let verified = verify_effect_package_v1(package, EffectPackageLimitsV1::default())
        .expect("accepted package fixture");
    let cid = effect_package_cid_v1(package, EffectPackageLimitsV1::default())
        .expect("accepted package CID");
    let selected = select_effect_package_artifact_v1(
        &verified,
        ArtifactSelectionRequestV1 {
            kind: EffectArtifactKindV1::TargetNative,
            target: "x86_64-unknown-linux-gnu",
            capabilities: &["avx2", "fma"],
        },
    )
    .expect("accepted AVX2/FMA artifact");
    let elapsed = u64::try_from(start.elapsed().as_nanos().max(1)).expect("nanoseconds fit u64");
    assert_eq!(selected.path(), "native/x86-fma.so");
    let output = [cid.as_binary(), selected.content()].concat();
    (elapsed, output)
}

fn state_workload(wire: &[u8]) -> (u64, Vec<u8>) {
    let state = include_bytes!("../../../fixtures/effect-state/v1/canonical.state.bin");
    let mut output = vec![0; state.len()];
    let start = Instant::now();
    let bound = bind_effect_descriptor_wire_v1(&STATE_DESCRIPTOR, wire, 1 << 20)
        .expect("accepted state descriptor");
    let verified = verify_effect_state_v1(bound, state, EffectStateLimitsV1::default())
        .expect("accepted state fixture");
    validate_effect_state_current_layout_v1(verified).expect("current state layout");
    validate_effect_state_replay_v1(verified, state_replay()).expect("state replay");
    let (common, left, right) = verified.payloads();
    encode_effect_state_v1(
        bound,
        state_replay(),
        common,
        left,
        right,
        EffectStateLimitsV1::default(),
        &mut output,
    )
    .expect("state re-encode");
    let elapsed = u64::try_from(start.elapsed().as_nanos().max(1)).expect("nanoseconds fit u64");
    assert_eq!(output, state);
    (elapsed, output)
}

struct WorkloadInputs {
    descriptor_wire: Vec<u8>,
    package: Vec<u8>,
    state_wire: Vec<u8>,
}

impl WorkloadInputs {
    fn load(expected_manifest_sha256: &str) -> Self {
        let descriptor_text =
            include_str!("../../../fixtures/effect-descriptor/v1/comprehensive-a.wire.hex");
        let package_text =
            include_str!("../../../fixtures/effect-package/v1/comprehensive-a.package.hex");
        let state_wire_text =
            include_str!("../../../fixtures/effect-state/v1/canonical.descriptor.wire.hex");
        assert_eq!(
            digest_hex(include_bytes!(
                "../../../fixtures/effect-interchange/v1/ACCEPTED.sha256"
            )),
            expected_manifest_sha256
        );
        assert_eq!(
            digest_hex(descriptor_text.as_bytes()),
            "0af73b1703fcf2b67a5772f585f301893b38be9b72f77e11010fb72368e8fa0c"
        );
        assert_eq!(
            digest_hex(package_text.as_bytes()),
            "2ef09dc7333e080b37d4c62eeb68b10e876558b12f95e0ff00c542955c870624"
        );
        assert_eq!(
            digest_hex(state_wire_text.as_bytes()),
            "42fec8759497f696a6d4513d6c9614b929dd8c7e153bc5d73699e7883eefbf7c"
        );
        assert_eq!(
            digest_hex(include_bytes!(
                "../../../fixtures/effect-state/v1/canonical.state.bin"
            )),
            "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48"
        );
        Self {
            descriptor_wire: hex_bytes(descriptor_text),
            package: hex_bytes(package_text),
            state_wire: hex_bytes(state_wire_text),
        }
    }
}

fn run_workload(workload: &str, inputs: &WorkloadInputs) -> (u64, Vec<u8>) {
    match workload {
        "descriptor_verify_identity_a" => descriptor_workload(&inputs.descriptor_wire),
        "package_verify_cid_select_a" => package_workload(&inputs.package),
        "state_verify_reencode_current" => state_workload(&inputs.state_wire),
        "migration_two_step_bank_restore" => migration_workload(),
        _ => panic!("unknown workload"),
    }
}

fn required_env(name: &str) -> String {
    let value = miso_engine_bench_support::metadata::Metadata::gather()
        .var(name)
        .unwrap_or_else(|_| panic!("missing environment {name}"));
    assert!(!value.is_empty(), "empty environment {name}");
    value
}

fn optional_env(name: &str, missing: &mut Vec<String>) -> String {
    match miso_engine_bench_support::metadata::Metadata::gather().var(name) {
        Ok(value) if !value.is_empty() => value,
        _ => {
            missing.push(name.to_ascii_lowercase());
            String::new()
        }
    }
}

fn json_string(value: &str) -> String {
    // The control-character guard is this subject's own: metadata that carries one is a runner
    // defect, not something to escape past. The escaping itself is the shared one (#104 F4).
    assert!(
        !value.chars().any(char::is_control),
        "control character in metadata"
    );
    format!("\"{}\"", json::escape(value))
}

struct Metadata {
    candidate_commit: String,
    candidate_tree: String,
    binary_sha256: String,
    tool_manifest_sha256: String,
    tool_source_sha256: String,
    fixture_manifest_sha256: String,
    rust_version: String,
    llvm_version: String,
    target_triple: String,
    profile: String,
    cpu_model: String,
    logical_cores: String,
    physical_cores: String,
    os: String,
    kernel: String,
    power_mode: String,
    governor: String,
    background_load: String,
    missing_metadata: Vec<String>,
}

impl Metadata {
    fn from_env() -> Self {
        let mut missing_metadata = Vec::new();
        let mut value = Self {
            candidate_commit: required_env("MISO_ENGINE_BENCH_CANDIDATE_COMMIT"),
            candidate_tree: required_env("MISO_ENGINE_BENCH_CANDIDATE_TREE"),
            binary_sha256: required_env("MISO_ENGINE_BENCH_BINARY_SHA256"),
            tool_manifest_sha256: required_env("MISO_ENGINE_BENCH_TOOL_MANIFEST_SHA256"),
            tool_source_sha256: required_env("MISO_ENGINE_BENCH_TOOL_SOURCE_SHA256"),
            fixture_manifest_sha256: required_env("MISO_ENGINE_BENCH_FIXTURE_MANIFEST_SHA256"),
            rust_version: required_env("MISO_ENGINE_BENCH_RUST_VERSION"),
            llvm_version: required_env("MISO_ENGINE_BENCH_LLVM_VERSION"),
            target_triple: required_env("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
            profile: required_env("MISO_ENGINE_BENCH_PROFILE"),
            cpu_model: optional_env("CPU_MODEL", &mut missing_metadata),
            logical_cores: optional_env("LOGICAL_CORES", &mut missing_metadata),
            physical_cores: optional_env("PHYSICAL_CORES", &mut missing_metadata),
            os: optional_env("OS", &mut missing_metadata),
            kernel: optional_env("KERNEL", &mut missing_metadata),
            power_mode: optional_env("POWER_MODE", &mut missing_metadata),
            governor: optional_env("GOVERNOR", &mut missing_metadata),
            background_load: optional_env("BACKGROUND_LOAD", &mut missing_metadata),
            missing_metadata,
        };
        value.missing_metadata.sort();
        value.missing_metadata.dedup();
        value
    }
}

fn record(
    metadata: &Metadata,
    workload: &str,
    round: u32,
    expected_output_sha256: &str,
    mut observations: Vec<u64>,
) -> String {
    observations.sort_unstable();
    assert_eq!(observations.len(), OBSERVATIONS);
    assert!(observations.iter().all(|value| *value > 0));
    let total: u128 = observations.iter().map(|value| u128::from(*value)).sum();
    let missing = metadata
        .missing_metadata
        .iter()
        .map(|value| json_string(value))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{\"schema_version\":1,\"issue\":108,\"workload_id\":{},\"round\":{},",
            "\"observation_count\":256,\"unit\":\"ns_per_operation\",",
            "\"candidate_commit\":{},\"candidate_tree\":{},\"binary_sha256\":{},",
            "\"tool_manifest_sha256\":{},\"tool_source_sha256\":{},",
            "\"fixture_manifest_sha256\":{},\"output_sha256\":{},",
            "\"rust_version\":{},\"llvm_version\":{},\"target_triple\":{},\"profile\":{},",
            "\"cpu_model\":{},\"logical_cores\":{},\"physical_cores\":{},\"os\":{},",
            "\"kernel\":{},\"power_mode\":{},\"governor\":{},\"background_load\":{},",
            "\"timer_method\":\"std::time::Instant\",",
            "\"percentile_method\":\"nearest-rank\",\"total_ns\":{},",
            "\"min_ns_per_operation\":{},\"p50_ns_per_operation\":{},",
            "\"p95_ns_per_operation\":{},\"p99_ns_per_operation\":{},",
            "\"p99_9_ns_per_operation\":{},\"max_ns_per_operation\":{},",
            "\"descriptive_only\":true,\"metadata_incomplete\":{},\"missing_metadata\":[{}]}}"
        ),
        json_string(workload),
        round,
        json_string(&metadata.candidate_commit),
        json_string(&metadata.candidate_tree),
        json_string(&metadata.binary_sha256),
        json_string(&metadata.tool_manifest_sha256),
        json_string(&metadata.tool_source_sha256),
        json_string(&metadata.fixture_manifest_sha256),
        json_string(expected_output_sha256),
        json_string(&metadata.rust_version),
        json_string(&metadata.llvm_version),
        json_string(&metadata.target_triple),
        json_string(&metadata.profile),
        json_string(&metadata.cpu_model),
        json_string(&metadata.logical_cores),
        json_string(&metadata.physical_cores),
        json_string(&metadata.os),
        json_string(&metadata.kernel),
        json_string(&metadata.power_mode),
        json_string(&metadata.governor),
        json_string(&metadata.background_load),
        total,
        observations[0],
        nearest_rank(&observations, 50, 100),
        nearest_rank(&observations, 95, 100),
        nearest_rank(&observations, 99, 100),
        nearest_rank(&observations, 999, 1000),
        observations[OBSERVATIONS - 1],
        !metadata.missing_metadata.is_empty(),
        missing,
    )
}

pub(crate) fn main() {
    assert_eq!(env::args_os().count(), 1, "benchmark takes no arguments");
    let metadata = Metadata::from_env();
    let inputs = WorkloadInputs::load(&metadata.fixture_manifest_sha256);
    eprintln!("MISO_ENGINE_BENCH_PHASE workload_started");
    let mut frozen = Vec::new();
    for (index, workload) in WORKLOADS.into_iter().enumerate() {
        let (_, output) = run_workload(workload, &inputs);
        let output_sha256 = digest_hex(&output);
        assert_eq!(output_sha256, EXPECTED_OUTPUT_SHA256[index]);
        frozen.push((workload, output_sha256));
    }
    for (workload, expected) in &frozen {
        let (_, output) = run_workload(workload, &inputs);
        assert_eq!(digest_hex(&output), *expected);
    }
    eprintln!("MISO_ENGINE_BENCH_PHASE warmup_complete");
    eprintln!("MISO_ENGINE_BENCH_PHASE timed_started");
    let mut records = Vec::with_capacity(8);
    for round in 1..=2 {
        for (workload, expected) in &frozen {
            let mut observations = Vec::with_capacity(OBSERVATIONS);
            for _ in 0..OBSERVATIONS {
                let (elapsed, output) = black_box(run_workload(workload, &inputs));
                assert_eq!(digest_hex(black_box(&output)), *expected);
                observations.push(elapsed);
            }
            records.push(record(&metadata, workload, round, expected, observations));
        }
        eprintln!("MISO_ENGINE_BENCH_PHASE round_{round}_complete");
    }
    assert_eq!(records.len(), 8);
    for record in records {
        println!("{record}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_four_rate_migration_envelope_without_timing() {
        let (elapsed, envelope) = execute_migration(UntimedMigration);
        assert_eq!(elapsed, 0);
        assert_eq!(envelope.len(), 283);
        let actual = digest_hex(&envelope);
        assert_ne!(actual, ISSUE_081_UNREACHABLE_MIGRATION_SHA256);
        assert_eq!(actual, EXPECTED_OUTPUT_SHA256[3]);
    }
}
