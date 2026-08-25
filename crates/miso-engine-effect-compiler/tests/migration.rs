//! Bounded adjacent state-migration registry and resolution coverage.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;

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

static PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
    id: ParameterId(1),
    display_name: "Value",
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
    nudge: None,
}];

static PORTS: [PortDescriptorV1; 2] = [
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

const fn sizes(common: u32, lane: u32) -> StatePayloadSizes {
    StatePayloadSizes {
        common_bytes: common,
        left_bytes: lane,
        right_bytes: lane,
    }
}

const fn quality(sample_rate: u32, maximum_state: StatePayloadSizes) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state,
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}

static QUALITIES_V1: [QualityDescriptorV1; 4] = [
    quality(44_100, sizes(1, 2)),
    quality(48_000, sizes(1, 2)),
    quality(88_200, sizes(1, 2)),
    quality(96_000, sizes(1, 2)),
];
static QUALITIES_V2: [QualityDescriptorV1; 4] = [
    quality(44_100, sizes(2, 3)),
    quality(48_000, sizes(2, 3)),
    quality(88_200, sizes(2, 3)),
    quality(96_000, sizes(2, 3)),
];
static QUALITIES_V3: [QualityDescriptorV1; 4] = [
    quality(44_100, sizes(3, 4)),
    quality(48_000, sizes(3, 4)),
    quality(88_200, sizes(3, 4)),
    quality(96_000, sizes(3, 4)),
];
static QUALITIES_ALT_V3: [QualityDescriptorV1; 4] = [
    quality(44_100, sizes(4, 5)),
    quality(48_000, sizes(4, 5)),
    quality(88_200, sizes(4, 5)),
    quality(96_000, sizes(4, 5)),
];

static DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.migration"),
    display_name: "Migration",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES_V1,
    observations: &[],
};
static DESCRIPTOR_V2: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 2,
    qualities: &QUALITIES_V2,
    ..DESCRIPTOR_V1
};
static DESCRIPTOR_V3: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 3,
    qualities: &QUALITIES_V3,
    ..DESCRIPTOR_V1
};
static DESCRIPTOR_V3_CLONE: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 3,
    qualities: &QUALITIES_V3,
    ..DESCRIPTOR_V1
};
static DESCRIPTOR_ALT_V3: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 3,
    qualities: &QUALITIES_ALT_V3,
    ..DESCRIPTOR_V1
};

static INITIAL: [InitialParameterValue; 1] = [InitialParameterValue {
    parameter_index: 0,
    channel: ParameterChannel::Both,
    value: 0.25,
}];

fn replay() -> EffectStateReplayViewV1<'static> {
    EffectStateReplayViewV1 {
        effect_id: DESCRIPTOR_V1.id,
        request: PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 32,
            quality: EffectQuality::Normal,
            bypass: true,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &INITIAL,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 64,
                maximum_scratch_bytes: 64,
                maximum_automation_spans_per_block: 8,
            },
        },
    }
}

fn descriptor_wire(descriptor: &'static EffectDescriptorV1) -> &'static [u8] {
    let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
    let mut wire = vec![0; required as usize];
    encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut wire).unwrap();
    Box::leak(wire.into_boxed_slice())
}

fn bound(descriptor: &'static EffectDescriptorV1) -> BoundEffectDescriptorWireV1<'static> {
    let wire = descriptor_wire(descriptor);
    bind_effect_descriptor_wire_v1(descriptor, wire, 1 << 20).unwrap()
}

fn envelope(bound: BoundEffectDescriptorWireV1<'_>) -> Vec<u8> {
    let requirements =
        effect_state_v1_requirements(bound, replay(), EffectStateLimitsV1::default()).unwrap();
    let metadata = effect_state_expected_metadata_v1(bound, replay()).unwrap();
    let mut common = vec![0x11; metadata.state_sizes.common_bytes as usize];
    let mut left = vec![0x22; metadata.state_sizes.left_bytes as usize];
    let mut right = vec![0x33; metadata.state_sizes.right_bytes as usize];
    if let Some(first) = common.first_mut() {
        *first = 0x41;
    }
    if let Some(first) = left.first_mut() {
        *first = 0x51;
    }
    if let Some(first) = right.first_mut() {
        *first = 0x61;
    }
    let mut output = vec![0; requirements.envelope_bytes as usize];
    encode_effect_state_v1(
        bound,
        replay(),
        &common,
        &left,
        &right,
        EffectStateLimitsV1::default(),
        &mut output,
    )
    .unwrap();
    output
}

#[derive(Default)]
struct FactoryCalls {
    prepare: AtomicUsize,
    bind: AtomicUsize,
}

struct MockFactory {
    descriptor: &'static EffectDescriptorV1,
    calls: Arc<FactoryCalls>,
}

impl NativeEffectFactory for MockFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        self.descriptor
    }

    fn prepare(
        &self,
        _: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        self.calls.prepare.fetch_add(1, Ordering::SeqCst);
        Err(EffectPrepareError {
            code: "migration.prepare.unreachable",
        })
    }

    fn bind_homogeneous_bank(
        &self,
        _: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        self.calls.bind.fetch_add(1, Ordering::SeqCst);
        Err(EffectPrepareError {
            code: "migration.bind.unreachable",
        })
    }
}

fn factory_capability(
    descriptor: &'static EffectDescriptorV1,
    calls: Arc<FactoryCalls>,
) -> WireBoundNativeEffectFactoryV1<'static> {
    let wire = descriptor_wire(descriptor);
    bind_native_effect_factory_state_v1(Arc::new(MockFactory { descriptor, calls }), wire, 1 << 20)
        .unwrap()
}

#[derive(Default)]
struct StepCalls {
    scratch: AtomicUsize,
    migrate: AtomicUsize,
}

struct MockStep {
    scratch_bytes: u64,
    calls: Arc<StepCalls>,
}

impl EffectStateMigrationStepV1 for MockStep {
    fn scratch_bytes(&self) -> u64 {
        self.calls.scratch.fetch_add(1, Ordering::SeqCst);
        self.scratch_bytes
    }

    fn migrate(
        &self,
        _: u32,
        _: u32,
        _: StatePayloadInput<'_>,
        _: StatePayloadOutput<'_>,
        _: &mut [u8],
    ) -> Result<EffectStateMigrationStepReportV1, EffectStateMigrationStepFailureV1> {
        self.calls.migrate.fetch_add(1, Ordering::SeqCst);
        Err(EffectStateMigrationStepFailureV1::Rejected)
    }
}

fn registration(
    source: BoundEffectDescriptorWireV1<'static>,
    target: BoundEffectDescriptorWireV1<'static>,
    scratch_bytes: u64,
    calls: Arc<StepCalls>,
) -> EffectStateMigrationRegistrationV1<'static> {
    bind_effect_state_migration_registration_v1(
        source,
        target,
        Arc::new(MockStep {
            scratch_bytes,
            calls,
        }),
    )
}

const fn migration_admission() -> EffectStateMigrationAdmissionV1 {
    EffectStateMigrationAdmissionV1 {
        maximum_chain_steps: 8,
        maximum_intermediate_envelope_bytes: 1 << 20,
        maximum_migration_scratch_bytes: 1 << 20,
    }
}

const fn restore_admission() -> EffectStateRestoreAdmissionV1 {
    EffectStateRestoreAdmissionV1 {
        sample_rate: 48_000,
        quantum: 32,
        maximum_total_state_bytes: 64,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 8,
    }
}

#[test]
fn diagnostics_and_step_report_layouts_are_exact() {
    assert_eq!(core::mem::size_of::<EffectStateMigrationDiagnosticV1>(), 56);
    assert_eq!(core::mem::align_of::<EffectStateMigrationDiagnosticV1>(), 8);
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, code),
        0
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, detail),
        4
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, item_index),
        8
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, reserved),
        12
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, required_bytes),
        16
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnosticV1, nested_state),
        24
    );
    assert_eq!(core::mem::size_of::<EffectStateMigrationStepReportV1>(), 16);
    let ok = EffectStateMigrationDiagnosticV1::ok();
    assert_eq!(ok.nested_state.code, EffectStateDiagnosticCodeV1::Ok);
    assert_eq!(
        ok.nested_state.item_index,
        EFFECT_STATE_V1_UNAVAILABLE_INDEX
    );
    assert_eq!(
        ok.nested_state.byte_offset,
        EFFECT_STATE_V1_UNAVAILABLE_OFFSET
    );
    assert_eq!(ok.reserved, 0);
}

#[test]
fn zero_one_and_two_step_resolution_have_exact_workspace_and_zero_hooks() {
    let v1 = bound(&DESCRIPTOR_V1);
    let v2 = bound(&DESCRIPTOR_V2);
    let v3 = bound(&DESCRIPTOR_V3);
    let current_envelope = envelope(v3);
    let v2_envelope = envelope(v2);
    let v1_envelope = envelope(v1);
    let factory_calls = Arc::new(FactoryCalls::default());
    let current = factory_capability(&DESCRIPTOR_V3, Arc::clone(&factory_calls));

    let empty = StateMigrationRegistryV1::new(0, Box::new([])).unwrap();
    let zero = resolve_effect_state_migration_v1(
        &empty,
        &current,
        &current_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_chain_steps: 0,
            maximum_intermediate_envelope_bytes: 0,
            maximum_migration_scratch_bytes: 0,
        },
        restore_admission(),
    )
    .unwrap();
    assert_eq!(
        zero.requirements(),
        EffectStateMigrationWorkspaceRequirementsV1 {
            chain_step_count: 0,
            first_envelope_bytes: 0,
            second_envelope_bytes: 0,
            migration_scratch_bytes: 0,
            scalar_initial_value_scratch_slots: 1,
            scalar_initial_value_scratch_bytes: core::mem::size_of::<InitialParameterValue>()
                as u64,
        }
    );

    let one_calls = Arc::new(StepCalls::default());
    let one_registry = StateMigrationRegistryV1::new(
        1,
        vec![registration(v2, v3, 7, Arc::clone(&one_calls))].into_boxed_slice(),
    )
    .unwrap();
    let one = resolve_effect_state_migration_v1(
        &one_registry,
        &current,
        &v2_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap();
    let v3_requirements =
        effect_state_v1_requirements(v3, replay(), EffectStateLimitsV1::default()).unwrap();
    assert_eq!(one.chain_step_count(), 1);
    assert_eq!(
        one.requirements().first_envelope_bytes,
        v3_requirements.envelope_bytes
    );
    assert_eq!(one.requirements().second_envelope_bytes, 0);
    assert_eq!(
        one.requirements().migration_scratch_bytes,
        v3_requirements.payload_snapshot_scratch_bytes + 7
    );

    let first_calls = Arc::new(StepCalls::default());
    let second_calls = Arc::new(StepCalls::default());
    let two_registry = StateMigrationRegistryV1::new(
        2,
        vec![
            registration(v1, v2, 5, Arc::clone(&first_calls)),
            registration(v2, v3, 7, Arc::clone(&second_calls)),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let two = resolve_effect_state_migration_v1(
        &two_registry,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap();
    let v2_requirements =
        effect_state_v1_requirements(v2, replay(), EffectStateLimitsV1::default()).unwrap();
    assert_eq!(two.chain_step_count(), 2);
    assert_eq!(
        two.requirements().first_envelope_bytes,
        v2_requirements.envelope_bytes
    );
    assert_eq!(
        two.requirements().second_envelope_bytes,
        v3_requirements.envelope_bytes
    );
    assert_eq!(
        two.requirements().migration_scratch_bytes,
        v3_requirements.payload_snapshot_scratch_bytes + 7
    );
    assert_eq!(one_calls.scratch.load(Ordering::SeqCst), 1);
    assert_eq!(first_calls.scratch.load(Ordering::SeqCst), 1);
    assert_eq!(second_calls.scratch.load(Ordering::SeqCst), 1);
    assert_eq!(one_calls.migrate.load(Ordering::SeqCst), 0);
    assert_eq!(first_calls.migrate.load(Ordering::SeqCst), 0);
    assert_eq!(second_calls.migrate.load(Ordering::SeqCst), 0);
    assert_eq!(factory_calls.prepare.load(Ordering::SeqCst), 0);
    assert_eq!(factory_calls.bind.load(Ordering::SeqCst), 0);
}

fn assert_outer(
    diagnostic: EffectStateMigrationDiagnosticV1,
    code: EffectStateMigrationDiagnosticCodeV1,
    detail: u32,
    item_index: u32,
    required_bytes: u64,
) {
    assert_eq!(
        (
            diagnostic.code,
            diagnostic.detail,
            diagnostic.item_index,
            diagnostic.reserved,
            diagnostic.required_bytes,
        ),
        (code, detail, item_index, 0, required_bytes)
    );
    if !matches!(
        code,
        EffectStateMigrationDiagnosticCodeV1::State | EffectStateMigrationDiagnosticCodeV1::Restore
    ) {
        assert_eq!(
            diagnostic.nested_state,
            EffectStateMigrationDiagnosticV1::ok().nested_state
        );
    }
}

#[test]
fn registry_rejects_edges_duplicates_caps_and_host_overflow_in_order() {
    let v1 = bound(&DESCRIPTOR_V1);
    let v2 = bound(&DESCRIPTOR_V2);
    let v3 = bound(&DESCRIPTOR_V3);
    let calls = Arc::new(StepCalls::default());
    let error = StateMigrationRegistryV1::new(
        1,
        vec![bind_effect_state_migration_registration_v1(
            v1,
            v3,
            Arc::new(MockStep {
                scratch_bytes: 0,
                calls: Arc::clone(&calls),
            }),
        )]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Registry,
        1,
        0,
        0,
    );
    assert_eq!(calls.scratch.load(Ordering::SeqCst), 0);

    let malformed_later = bind_effect_state_migration_registration_v1(
        v1,
        v3,
        Arc::new(MockStep {
            scratch_bytes: 0,
            calls: Arc::new(StepCalls::default()),
        }),
    );
    let error = StateMigrationRegistryV1::new(
        2,
        vec![
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
            malformed_later,
        ]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Registry,
        1,
        1,
        0,
    );

    let error = StateMigrationRegistryV1::new(
        1,
        vec![
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
            bind_effect_state_migration_registration_v1(
                v1,
                v3,
                Arc::new(MockStep {
                    scratch_bytes: u64::MAX,
                    calls: Arc::new(StepCalls::default()),
                }),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Limit,
        1,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        2,
    );

    let entry_calls = Arc::new(StepCalls::default());
    let error = StateMigrationRegistryV1::new(
        0,
        vec![registration(v1, v2, 0, Arc::clone(&entry_calls))].into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Limit,
        1,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        1,
    );

    let error = StateMigrationRegistryV1::new(
        2,
        vec![
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
        ]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Registry,
        5,
        1,
        0,
    );

    let error = StateMigrationRegistryV1::new(
        1,
        vec![registration(
            v2,
            v3,
            u64::MAX,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Overflow,
        1,
        0,
        u64::MAX,
    );
}

#[test]
fn resolution_rejects_missing_downgrade_terminal_caps_and_overflow_exactly() {
    let v1 = bound(&DESCRIPTOR_V1);
    let v2 = bound(&DESCRIPTOR_V2);
    let v3 = bound(&DESCRIPTOR_V3);
    let v1_envelope = envelope(v1);
    let v2_envelope = envelope(v2);
    let v3_envelope = envelope(v3);
    let current_calls = Arc::new(FactoryCalls::default());
    let current = factory_capability(&DESCRIPTOR_V3, Arc::clone(&current_calls));
    let empty = StateMigrationRegistryV1::new(0, Box::new([])).unwrap();
    let error = resolve_effect_state_migration_v1(
        &empty,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Chain, 1, 0, 0);

    let only_first = StateMigrationRegistryV1::new(
        1,
        vec![registration(v1, v2, 0, Arc::new(StepCalls::default()))].into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration_v1(
        &only_first,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Chain, 1, 1, 0);

    let error = resolve_effect_state_migration_v1(
        &only_first,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_chain_steps: 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Limit, 2, 1, 2);

    let old_current = factory_capability(&DESCRIPTOR_V1, Arc::new(FactoryCalls::default()));
    let error = resolve_effect_state_migration_v1(
        &empty,
        &old_current,
        &v3_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Chain,
        2,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        3,
    );

    let alt_v3_envelope = envelope(bound(&DESCRIPTOR_ALT_V3));
    let error = resolve_effect_state_migration_v1(
        &empty,
        &current,
        &alt_v3_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Chain,
        2,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        3,
    );

    let alt_v3 = bound(&DESCRIPTOR_ALT_V3);
    let wrong_terminal = StateMigrationRegistryV1::new(
        1,
        vec![registration(v2, alt_v3, 0, Arc::new(StepCalls::default()))].into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration_v1(
        &wrong_terminal,
        &current,
        &v2_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Chain, 3, 0, 0);

    let clone_v3 = bound(&DESCRIPTOR_V3_CLONE);
    let wrong_provenance = StateMigrationRegistryV1::new(
        1,
        vec![registration(
            v2,
            clone_v3,
            0,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration_v1(
        &wrong_provenance,
        &current,
        &v2_envelope,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Chain, 3, 0, 0);

    let two = StateMigrationRegistryV1::new(
        2,
        vec![
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
            registration(v2, v3, 0, Arc::new(StepCalls::default())),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration_v1(
        &two,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_chain_steps: 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCodeV1::Limit, 2, 1, 2);

    let v2_requirements =
        effect_state_v1_requirements(v2, replay(), EffectStateLimitsV1::default()).unwrap();
    let error = resolve_effect_state_migration_v1(
        &two,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_intermediate_envelope_bytes: v2_requirements.envelope_bytes - 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Limit,
        3,
        0,
        v2_requirements.envelope_bytes,
    );

    let error = resolve_effect_state_migration_v1(
        &two,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_intermediate_envelope_bytes: v2_requirements.envelope_bytes - 1,
            maximum_migration_scratch_bytes: 0,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Limit,
        3,
        0,
        v2_requirements.envelope_bytes,
    );

    let first_scratch = v2_requirements.payload_snapshot_scratch_bytes;
    let error = resolve_effect_state_migration_v1(
        &two,
        &current,
        &v1_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_migration_scratch_bytes: first_scratch - 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Limit,
        4,
        0,
        first_scratch,
    );

    let huge = StateMigrationRegistryV1::new(
        1,
        vec![registration(
            v2,
            v3,
            isize::MAX as u64,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration_v1(
        &huge,
        &current,
        &v2_envelope,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_migration_scratch_bytes: isize::MAX as u64,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Overflow,
        2,
        0,
        isize::MAX as u64 + sizes(3, 4).total().unwrap(),
    );
    assert_eq!(current_calls.prepare.load(Ordering::SeqCst), 0);
    assert_eq!(current_calls.bind.load(Ordering::SeqCst), 0);
}

#[test]
fn nested_state_diagnostics_and_current_admission_are_preserved_without_hooks() {
    let v1 = bound(&DESCRIPTOR_V1);
    let v2 = bound(&DESCRIPTOR_V2);
    let v3 = bound(&DESCRIPTOR_V3);
    let mut malformed = envelope(v1);
    malformed[56] ^= 1;
    let current_calls = Arc::new(FactoryCalls::default());
    let current = factory_capability(&DESCRIPTOR_V3, Arc::clone(&current_calls));
    let registry = StateMigrationRegistryV1::new(
        2,
        vec![
            registration(v1, v2, 0, Arc::new(StepCalls::default())),
            registration(v2, v3, 0, Arc::new(StepCalls::default())),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let current_envelope = envelope(v3);
    let error = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &current_envelope,
        EffectStateLimitsV1 {
            maximum_descriptor_bytes: descriptor_wire(&DESCRIPTOR_V3).len() as u64 - 1,
            ..EffectStateLimitsV1::default()
        },
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::State,
        1,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        0,
    );
    assert_eq!(error.nested_state.code, EffectStateDiagnosticCodeV1::Limit);

    let error = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &malformed,
        EffectStateLimitsV1::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::State,
        1,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        0,
    );
    assert_eq!(error.nested_state.code, EffectStateDiagnosticCodeV1::Digest);
    assert_eq!(error.nested_state.byte_offset, 56);

    let error = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &malformed,
        EffectStateLimitsV1::default(),
        EffectStateMigrationAdmissionV1 {
            maximum_intermediate_envelope_bytes: u64::MAX,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCodeV1::Overflow,
        2,
        EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
        u64::MAX,
    );

    let v1_envelope = envelope(v1);
    for (admission, offset, required) in [
        (
            EffectStateRestoreAdmissionV1 {
                sample_rate: 44_100,
                ..restore_admission()
            },
            96,
            48_000,
        ),
        (
            EffectStateRestoreAdmissionV1 {
                quantum: 31,
                ..restore_admission()
            },
            100,
            32,
        ),
        (
            EffectStateRestoreAdmissionV1 {
                maximum_total_state_bytes: 63,
                ..restore_admission()
            },
            192,
            64,
        ),
        (
            EffectStateRestoreAdmissionV1 {
                maximum_scratch_bytes: 63,
                ..restore_admission()
            },
            200,
            64,
        ),
        (
            EffectStateRestoreAdmissionV1 {
                maximum_automation_spans_per_block: 7,
                ..restore_admission()
            },
            208,
            8,
        ),
    ] {
        let error = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &v1_envelope,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission,
        )
        .unwrap_err();
        assert_outer(
            error,
            EffectStateMigrationDiagnosticCodeV1::State,
            3,
            EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX,
            0,
        );
        assert_eq!(error.nested_state.code, EffectStateDiagnosticCodeV1::Limit);
        assert_eq!(error.nested_state.byte_offset, offset);
        assert_eq!(error.nested_state.required_bytes, required);
    }

    // Once target requirements prove derived resources fit the saved caps and the five raw
    // admission rows above prove those caps fit the current policy, derived 216/176/184 rejects
    // are algebraically unreachable during resolution.
    let derived = effect_state_derived_resources_v1(v3, replay().request).unwrap();
    assert!(
        derived.state_sizes.total().unwrap() <= replay().request.limits.maximum_total_state_bytes
    );
    assert!(derived.scratch_bytes <= replay().request.limits.maximum_scratch_bytes);
    assert_eq!(
        derived.automation_capacity,
        replay().request.limits.maximum_automation_spans_per_block
    );
    assert_eq!(current_calls.prepare.load(Ordering::SeqCst), 0);
    assert_eq!(current_calls.bind.load(Ordering::SeqCst), 0);
}

#[test]
fn dependency_and_render_boundaries_remain_one_way() {
    let package_manifest = include_str!("../../miso-engine-effect-package/Cargo.toml");
    let compiler_manifest = include_str!("../Cargo.toml");
    let realtime_module = include_str!("../../miso-engine-core/src/realtime/mod.rs");
    let realtime_plan = include_str!("../../miso-engine-core/src/realtime/plan.rs");
    let migration_source = include_str!("../src/migration.rs");
    assert!(!package_manifest.contains("miso-engine-effect-compiler"));
    assert!(compiler_manifest.contains("miso-engine-effect-package.workspace = true"));
    for forbidden in [
        "validate_descriptor_v1",
        "effect_descriptor_identity_v1",
        "bind_effect_descriptor_wire_v1",
    ] {
        assert!(!migration_source.contains(forbidden));
    }
    for source in [realtime_module, realtime_plan] {
        assert!(!source.contains("effect_state_migration"));
        assert!(!source.contains("miso_engine_effect_package"));
        assert!(!source.contains("miso_engine_effect_compiler"));
    }
}
