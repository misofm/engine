//! Bounded adjacent state-migration registry and resolution coverage.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use effect_compiler::*;
use effect_contract::*;
use effect_package::*;

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

static PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
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
    lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
}];

static PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
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

const fn quality(sample_rate: u32, maximum_state: StatePayloadSizes) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state,
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}

static QUALITIES_SOURCE: [QualityDescriptor; 4] = [
    quality(44_100, sizes(1, 2)),
    quality(48_000, sizes(1, 2)),
    quality(88_200, sizes(1, 2)),
    quality(96_000, sizes(1, 2)),
];
static QUALITIES_INTERMEDIATE: [QualityDescriptor; 4] = [
    quality(44_100, sizes(2, 3)),
    quality(48_000, sizes(2, 3)),
    quality(88_200, sizes(2, 3)),
    quality(96_000, sizes(2, 3)),
];
static QUALITIES_CURRENT: [QualityDescriptor; 4] = [
    quality(44_100, sizes(3, 4)),
    quality(48_000, sizes(3, 4)),
    quality(88_200, sizes(3, 4)),
    quality(96_000, sizes(3, 4)),
];
static QUALITIES_ALTERNATE_CURRENT: [QualityDescriptor; 4] = [
    quality(44_100, sizes(4, 5)),
    quality(48_000, sizes(4, 5)),
    quality(88_200, sizes(4, 5)),
    quality(96_000, sizes(4, 5)),
];

static DESCRIPTOR_SOURCE: EffectDescriptor = EffectDescriptor {
    id: effect_id("test.migration"),
    display_name: "Migration",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES_SOURCE,
    observations: &[],
};
static DESCRIPTOR_INTERMEDIATE: EffectDescriptor = EffectDescriptor {
    state_layout_version: 2,
    qualities: &QUALITIES_INTERMEDIATE,
    ..DESCRIPTOR_SOURCE
};
static DESCRIPTOR_CURRENT: EffectDescriptor = EffectDescriptor {
    state_layout_version: 3,
    qualities: &QUALITIES_CURRENT,
    ..DESCRIPTOR_SOURCE
};
static DESCRIPTOR_CURRENT_CLONE: EffectDescriptor = EffectDescriptor {
    state_layout_version: 3,
    qualities: &QUALITIES_CURRENT,
    ..DESCRIPTOR_SOURCE
};
static DESCRIPTOR_ALTERNATE_CURRENT: EffectDescriptor = EffectDescriptor {
    state_layout_version: 3,
    qualities: &QUALITIES_ALTERNATE_CURRENT,
    ..DESCRIPTOR_SOURCE
};

static INITIAL: [InitialParameterValue; 1] = [InitialParameterValue {
    parameter_index: 0,
    channel: ParameterChannel::Both,
    value: 0.25,
}];

fn replay() -> EffectStateReplayView<'static> {
    EffectStateReplayView {
        effect_id: DESCRIPTOR_SOURCE.id,
        request: PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 32,
            quality: EffectQuality::Normal,
            bypass: true,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
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

fn descriptor_wire(descriptor: &'static EffectDescriptor) -> &'static [u8] {
    let required = effect_descriptor_wire_required_size(descriptor, 1 << 20).unwrap();
    let mut wire = vec![0; required as usize];
    encode_effect_descriptor_wire(descriptor, 1 << 20, &mut wire).unwrap();
    Box::leak(wire.into_boxed_slice())
}

fn bound(descriptor: &'static EffectDescriptor) -> BoundEffectDescriptorWire<'static> {
    let wire = descriptor_wire(descriptor);
    bind_effect_descriptor_wire(descriptor, wire, 1 << 20).unwrap()
}

fn envelope(bound: BoundEffectDescriptorWire<'_>) -> Vec<u8> {
    let requirements =
        effect_state_requirements(bound, replay(), EffectStateLimits::default()).unwrap();
    let metadata = effect_state_expected_metadata(bound, replay()).unwrap();
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
    encode_effect_state(
        bound,
        replay(),
        &common,
        &left,
        &right,
        EffectStateLimits::default(),
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
    descriptor: &'static EffectDescriptor,
    calls: Arc<FactoryCalls>,
}

impl NativeEffectFactory for MockFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
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
    descriptor: &'static EffectDescriptor,
    calls: Arc<FactoryCalls>,
) -> WireBoundNativeEffectFactory<'static> {
    let wire = descriptor_wire(descriptor);
    bind_native_effect_factory_state(Arc::new(MockFactory { descriptor, calls }), wire, 1 << 20)
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

impl EffectStateMigrationStep for MockStep {
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
    ) -> Result<EffectStateMigrationStepReport, EffectStateMigrationStepFailure> {
        self.calls.migrate.fetch_add(1, Ordering::SeqCst);
        Err(EffectStateMigrationStepFailure::Rejected)
    }
}

fn registration(
    source: BoundEffectDescriptorWire<'static>,
    target: BoundEffectDescriptorWire<'static>,
    scratch_bytes: u64,
    calls: Arc<StepCalls>,
) -> EffectStateMigrationRegistration<'static> {
    bind_effect_state_migration_registration(
        source,
        target,
        Arc::new(MockStep {
            scratch_bytes,
            calls,
        }),
    )
}

const fn migration_admission() -> EffectStateMigrationAdmission {
    EffectStateMigrationAdmission {
        maximum_chain_steps: 8,
        maximum_intermediate_envelope_bytes: 1 << 20,
        maximum_migration_scratch_bytes: 1 << 20,
    }
}

const fn restore_admission() -> EffectStateRestoreAdmission {
    EffectStateRestoreAdmission {
        sample_rate: 48_000,
        quantum: 32,
        maximum_total_state_bytes: 64,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 8,
    }
}

#[test]
fn diagnostics_and_step_report_layouts_are_exact() {
    assert_eq!(core::mem::size_of::<EffectStateMigrationDiagnostic>(), 56);
    assert_eq!(core::mem::align_of::<EffectStateMigrationDiagnostic>(), 8);
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, code),
        0
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, detail),
        4
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, item_index),
        8
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, reserved),
        12
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, required_bytes),
        16
    );
    assert_eq!(
        core::mem::offset_of!(EffectStateMigrationDiagnostic, nested_state),
        24
    );
    assert_eq!(core::mem::size_of::<EffectStateMigrationStepReport>(), 16);
    let ok = EffectStateMigrationDiagnostic::ok();
    assert_eq!(ok.nested_state.code, EffectStateDiagnosticCode::Ok);
    assert_eq!(ok.nested_state.item_index, EFFECT_STATE_UNAVAILABLE_INDEX);
    assert_eq!(ok.nested_state.byte_offset, EFFECT_STATE_UNAVAILABLE_OFFSET);
    assert_eq!(ok.reserved, 0);
}

#[test]
fn zero_one_and_two_step_resolution_have_exact_workspace_and_zero_hooks() {
    let source = bound(&DESCRIPTOR_SOURCE);
    let intermediate = bound(&DESCRIPTOR_INTERMEDIATE);
    let current_descriptor = bound(&DESCRIPTOR_CURRENT);
    let current_envelope = envelope(current_descriptor);
    let intermediate_envelope = envelope(intermediate);
    let source_envelope = envelope(source);
    let factory_calls = Arc::new(FactoryCalls::default());
    let current_factory = factory_capability(&DESCRIPTOR_CURRENT, Arc::clone(&factory_calls));

    let empty = StateMigrationRegistry::new(0, Box::new([])).unwrap();
    let zero = resolve_effect_state_migration(
        &empty,
        &current_factory,
        &current_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_chain_steps: 0,
            maximum_intermediate_envelope_bytes: 0,
            maximum_migration_scratch_bytes: 0,
        },
        restore_admission(),
    )
    .unwrap();
    assert_eq!(
        zero.requirements(),
        EffectStateMigrationWorkspaceRequirements {
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
    let one_registry = StateMigrationRegistry::new(
        1,
        vec![registration(
            intermediate,
            current_descriptor,
            7,
            Arc::clone(&one_calls),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let one = resolve_effect_state_migration(
        &one_registry,
        &current_factory,
        &intermediate_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap();
    let current_requirements =
        effect_state_requirements(current_descriptor, replay(), EffectStateLimits::default())
            .unwrap();
    assert_eq!(one.chain_step_count(), 1);
    assert_eq!(
        one.requirements().first_envelope_bytes,
        current_requirements.envelope_bytes
    );
    assert_eq!(one.requirements().second_envelope_bytes, 0);
    assert_eq!(
        one.requirements().migration_scratch_bytes,
        current_requirements.payload_snapshot_scratch_bytes + 7
    );

    let first_calls = Arc::new(StepCalls::default());
    let second_calls = Arc::new(StepCalls::default());
    let two_registry = StateMigrationRegistry::new(
        2,
        vec![
            registration(source, intermediate, 5, Arc::clone(&first_calls)),
            registration(
                intermediate,
                current_descriptor,
                7,
                Arc::clone(&second_calls),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let two = resolve_effect_state_migration(
        &two_registry,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap();
    let intermediate_requirements =
        effect_state_requirements(intermediate, replay(), EffectStateLimits::default()).unwrap();
    assert_eq!(two.chain_step_count(), 2);
    assert_eq!(
        two.requirements().first_envelope_bytes,
        intermediate_requirements.envelope_bytes
    );
    assert_eq!(
        two.requirements().second_envelope_bytes,
        current_requirements.envelope_bytes
    );
    assert_eq!(
        two.requirements().migration_scratch_bytes,
        current_requirements.payload_snapshot_scratch_bytes + 7
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
    diagnostic: EffectStateMigrationDiagnostic,
    code: EffectStateMigrationDiagnosticCode,
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
        EffectStateMigrationDiagnosticCode::State | EffectStateMigrationDiagnosticCode::Restore
    ) {
        assert_eq!(
            diagnostic.nested_state,
            EffectStateMigrationDiagnostic::ok().nested_state
        );
    }
}

#[test]
fn registry_rejects_edges_duplicates_caps_and_host_overflow_in_order() {
    let source = bound(&DESCRIPTOR_SOURCE);
    let intermediate = bound(&DESCRIPTOR_INTERMEDIATE);
    let current_descriptor = bound(&DESCRIPTOR_CURRENT);
    let calls = Arc::new(StepCalls::default());
    let error = StateMigrationRegistry::new(
        1,
        vec![bind_effect_state_migration_registration(
            source,
            current_descriptor,
            Arc::new(MockStep {
                scratch_bytes: 0,
                calls: Arc::clone(&calls),
            }),
        )]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Registry, 1, 0, 0);
    assert_eq!(calls.scratch.load(Ordering::SeqCst), 0);

    let malformed_later = bind_effect_state_migration_registration(
        source,
        current_descriptor,
        Arc::new(MockStep {
            scratch_bytes: 0,
            calls: Arc::new(StepCalls::default()),
        }),
    );
    let error = StateMigrationRegistry::new(
        2,
        vec![
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
            malformed_later,
        ]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Registry, 1, 1, 0);

    let error = StateMigrationRegistry::new(
        1,
        vec![
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
            bind_effect_state_migration_registration(
                source,
                current_descriptor,
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
        EffectStateMigrationDiagnosticCode::Limit,
        1,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        2,
    );

    let entry_calls = Arc::new(StepCalls::default());
    let error = StateMigrationRegistry::new(
        0,
        vec![registration(
            source,
            intermediate,
            0,
            Arc::clone(&entry_calls),
        )]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Limit,
        1,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        1,
    );

    let error = StateMigrationRegistry::new(
        2,
        vec![
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
        ]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Registry, 5, 1, 0);

    let error = StateMigrationRegistry::new(
        1,
        vec![registration(
            intermediate,
            current_descriptor,
            u64::MAX,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Overflow,
        1,
        0,
        u64::MAX,
    );
}

#[test]
fn resolution_rejects_missing_downgrade_terminal_caps_and_overflow_exactly() {
    let source = bound(&DESCRIPTOR_SOURCE);
    let intermediate = bound(&DESCRIPTOR_INTERMEDIATE);
    let current_descriptor = bound(&DESCRIPTOR_CURRENT);
    let source_envelope = envelope(source);
    let intermediate_envelope = envelope(intermediate);
    let current_envelope = envelope(current_descriptor);
    let current_calls = Arc::new(FactoryCalls::default());
    let current_factory = factory_capability(&DESCRIPTOR_CURRENT, Arc::clone(&current_calls));
    let empty = StateMigrationRegistry::new(0, Box::new([])).unwrap();
    let error = resolve_effect_state_migration(
        &empty,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Chain, 1, 0, 0);

    let only_first = StateMigrationRegistry::new(
        1,
        vec![registration(
            source,
            intermediate,
            0,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration(
        &only_first,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Chain, 1, 1, 0);

    let error = resolve_effect_state_migration(
        &only_first,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_chain_steps: 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Limit, 2, 1, 2);

    let old_current = factory_capability(&DESCRIPTOR_SOURCE, Arc::new(FactoryCalls::default()));
    let error = resolve_effect_state_migration(
        &empty,
        &old_current,
        &current_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Chain,
        2,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        3,
    );

    let alternate_current_envelope = envelope(bound(&DESCRIPTOR_ALTERNATE_CURRENT));
    let error = resolve_effect_state_migration(
        &empty,
        &current_factory,
        &alternate_current_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Chain,
        2,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        3,
    );

    let alternate_current = bound(&DESCRIPTOR_ALTERNATE_CURRENT);
    let wrong_terminal = StateMigrationRegistry::new(
        1,
        vec![registration(
            intermediate,
            alternate_current,
            0,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration(
        &wrong_terminal,
        &current_factory,
        &intermediate_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Chain, 3, 0, 0);

    let current_clone = bound(&DESCRIPTOR_CURRENT_CLONE);
    let wrong_provenance = StateMigrationRegistry::new(
        1,
        vec![registration(
            intermediate,
            current_clone,
            0,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration(
        &wrong_provenance,
        &current_factory,
        &intermediate_envelope,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Chain, 3, 0, 0);

    let two = StateMigrationRegistry::new(
        2,
        vec![
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
            registration(
                intermediate,
                current_descriptor,
                0,
                Arc::new(StepCalls::default()),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration(
        &two,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_chain_steps: 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(error, EffectStateMigrationDiagnosticCode::Limit, 2, 1, 2);

    let intermediate_requirements =
        effect_state_requirements(intermediate, replay(), EffectStateLimits::default()).unwrap();
    let error = resolve_effect_state_migration(
        &two,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_intermediate_envelope_bytes: intermediate_requirements.envelope_bytes - 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Limit,
        3,
        0,
        intermediate_requirements.envelope_bytes,
    );

    let error = resolve_effect_state_migration(
        &two,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_intermediate_envelope_bytes: intermediate_requirements.envelope_bytes - 1,
            maximum_migration_scratch_bytes: 0,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Limit,
        3,
        0,
        intermediate_requirements.envelope_bytes,
    );

    let first_scratch = intermediate_requirements.payload_snapshot_scratch_bytes;
    let error = resolve_effect_state_migration(
        &two,
        &current_factory,
        &source_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_migration_scratch_bytes: first_scratch - 1,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Limit,
        4,
        0,
        first_scratch,
    );

    let huge = StateMigrationRegistry::new(
        1,
        vec![registration(
            intermediate,
            current_descriptor,
            isize::MAX as u64,
            Arc::new(StepCalls::default()),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let error = resolve_effect_state_migration(
        &huge,
        &current_factory,
        &intermediate_envelope,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_migration_scratch_bytes: isize::MAX as u64,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Overflow,
        2,
        0,
        isize::MAX as u64 + sizes(3, 4).total().unwrap(),
    );
    assert_eq!(current_calls.prepare.load(Ordering::SeqCst), 0);
    assert_eq!(current_calls.bind.load(Ordering::SeqCst), 0);
}

#[test]
fn nested_state_diagnostics_and_current_admission_are_preserved_without_hooks() {
    let source = bound(&DESCRIPTOR_SOURCE);
    let intermediate = bound(&DESCRIPTOR_INTERMEDIATE);
    let current_descriptor = bound(&DESCRIPTOR_CURRENT);
    let mut malformed = envelope(source);
    malformed[56] ^= 1;
    let current_calls = Arc::new(FactoryCalls::default());
    let current_factory = factory_capability(&DESCRIPTOR_CURRENT, Arc::clone(&current_calls));
    let registry = StateMigrationRegistry::new(
        2,
        vec![
            registration(source, intermediate, 0, Arc::new(StepCalls::default())),
            registration(
                intermediate,
                current_descriptor,
                0,
                Arc::new(StepCalls::default()),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let current_envelope = envelope(current_descriptor);
    let error = resolve_effect_state_migration(
        &registry,
        &current_factory,
        &current_envelope,
        EffectStateLimits {
            maximum_descriptor_bytes: descriptor_wire(&DESCRIPTOR_CURRENT).len() as u64 - 1,
            ..EffectStateLimits::default()
        },
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::State,
        1,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        0,
    );
    assert_eq!(error.nested_state.code, EffectStateDiagnosticCode::Limit);

    let error = resolve_effect_state_migration(
        &registry,
        &current_factory,
        &malformed,
        EffectStateLimits::default(),
        migration_admission(),
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::State,
        1,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        0,
    );
    assert_eq!(error.nested_state.code, EffectStateDiagnosticCode::Digest);
    assert_eq!(error.nested_state.byte_offset, 56);

    let error = resolve_effect_state_migration(
        &registry,
        &current_factory,
        &malformed,
        EffectStateLimits::default(),
        EffectStateMigrationAdmission {
            maximum_intermediate_envelope_bytes: u64::MAX,
            ..migration_admission()
        },
        restore_admission(),
    )
    .unwrap_err();
    assert_outer(
        error,
        EffectStateMigrationDiagnosticCode::Overflow,
        2,
        EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
        u64::MAX,
    );

    let source_envelope = envelope(source);
    for (admission, offset, required) in [
        (
            EffectStateRestoreAdmission {
                sample_rate: 44_100,
                ..restore_admission()
            },
            96,
            48_000,
        ),
        (
            EffectStateRestoreAdmission {
                quantum: 31,
                ..restore_admission()
            },
            100,
            32,
        ),
        (
            EffectStateRestoreAdmission {
                maximum_total_state_bytes: 63,
                ..restore_admission()
            },
            192,
            64,
        ),
        (
            EffectStateRestoreAdmission {
                maximum_scratch_bytes: 63,
                ..restore_admission()
            },
            200,
            64,
        ),
        (
            EffectStateRestoreAdmission {
                maximum_automation_spans_per_block: 7,
                ..restore_admission()
            },
            208,
            8,
        ),
    ] {
        let error = resolve_effect_state_migration(
            &registry,
            &current_factory,
            &source_envelope,
            EffectStateLimits::default(),
            migration_admission(),
            admission,
        )
        .unwrap_err();
        assert_outer(
            error,
            EffectStateMigrationDiagnosticCode::State,
            3,
            EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX,
            0,
        );
        assert_eq!(error.nested_state.code, EffectStateDiagnosticCode::Limit);
        assert_eq!(error.nested_state.byte_offset, offset);
        assert_eq!(error.nested_state.required_bytes, required);
    }

    // Once target requirements prove derived resources fit the saved caps and the five raw
    // admission rows above prove those caps fit the current policy, derived 216/176/184 rejects
    // are algebraically unreachable during resolution.
    let derived = effect_state_derived_resources(current_descriptor, replay().request).unwrap();
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
    let package_manifest = include_str!("../../effect-package/Cargo.toml");
    let compiler_manifest = include_str!("../Cargo.toml");
    let realtime_module = include_str!("../../engine/src/realtime/mod.rs");
    let realtime_plan = include_str!("../../engine/src/realtime/plan.rs");
    let migration_source = include_str!("../src/migration.rs");
    assert!(!package_manifest.contains("effect-compiler"));
    assert!(compiler_manifest.contains("effect-package.workspace = true"));
    for forbidden in [
        "validate_descriptor",
        "effect_descriptor_identity",
        "bind_effect_descriptor_wire",
    ] {
        assert!(!migration_source.contains(forbidden));
    }
    for source in [realtime_module, realtime_plan] {
        assert!(!source.contains("effect_state_migration"));
        assert!(!source.contains("effect_package"));
        assert!(!source.contains("effect_compiler"));
    }
}
