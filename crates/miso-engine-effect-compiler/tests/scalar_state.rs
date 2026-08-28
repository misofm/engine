//! Transactional scalar current-layout state snapshot and restore coverage.

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use sha2::{Digest, Sha256};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

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
const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 1,
            left_bytes: 2,
            right_bytes: 2,
        },
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}
static QUALITIES: [QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];
static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("test.scalar-state"),
    display_name: "Scalar state",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};
static ALTERNATE_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    display_name: "Other scalar",
    ..DESCRIPTOR
};
static INITIAL: [InitialParameterValue; 2] = [
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

#[derive(Default)]
struct Controls {
    prepare_calls: AtomicUsize,
    snapshot_calls: AtomicUsize,
    restore_calls: AtomicUsize,
    fail_prepare: AtomicBool,
    fail_snapshot: AtomicBool,
    fail_restore: AtomicBool,
    wrong_metadata: AtomicBool,
}

struct MockFactory {
    controls: Arc<Controls>,
}
struct MockEffect {
    controls: Arc<Controls>,
    metadata: PreparedEffectMetadata,
    common: [u8; 1],
    left: [u8; 2],
    right: [u8; 2],
}

impl NativeEffectFactory for MockFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DESCRIPTOR
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        self.controls.prepare_calls.fetch_add(1, Ordering::SeqCst);
        if self.controls.fail_prepare.load(Ordering::SeqCst) {
            return Err(EffectPrepareError {
                code: "test.prepare",
            });
        }
        let mut metadata = expected_prepared_metadata(&DESCRIPTOR, request)?;
        if self.controls.wrong_metadata.load(Ordering::SeqCst) {
            metadata.descriptor = &ALTERNATE_DESCRIPTOR;
        }
        Ok(Box::new(MockEffect {
            controls: Arc::clone(&self.controls),
            metadata,
            common: [0x11],
            left: [0x21, 0x22],
            right: [0x31, 0x32],
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        _: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}

impl PreparedNativeEffect for MockEffect {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process(&mut self, _: EffectProcessBlock<'_>) -> ProcessReport {
        ProcessReport::default()
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.controls.snapshot_calls.fetch_add(1, Ordering::SeqCst);
        output.common[0] = self.common[0];
        if self.controls.fail_snapshot.load(Ordering::SeqCst) {
            return Err(StatePayloadError {
                code: "test.snapshot",
            });
        }
        output.left.copy_from_slice(&self.left);
        output.right.copy_from_slice(&self.right);
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.controls.restore_calls.fetch_add(1, Ordering::SeqCst);
        self.common.copy_from_slice(input.common);
        if self.controls.fail_restore.load(Ordering::SeqCst) {
            return Err(StatePayloadError {
                code: "test.restore",
            });
        }
        if version != 1 {
            return Err(StatePayloadError {
                code: "test.version",
            });
        }
        self.left.copy_from_slice(input.left);
        self.right.copy_from_slice(input.right);
        Ok(())
    }
}

fn preparation(sample_rate: u32) -> EffectBankPreparation {
    EffectBankPreparation {
        sample_rate,
        quantum: 32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: INITIAL.into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 5,
            maximum_scratch_bytes: 8,
            maximum_automation_spans_per_block: 7,
        },
    }
}

fn wire(descriptor: &'static EffectDescriptor) -> Vec<u8> {
    let required = effect_descriptor_wire_required_size(descriptor, 1 << 20).unwrap();
    let mut output = vec![0; required as usize];
    encode_effect_descriptor_wire(descriptor, 1 << 20, &mut output).unwrap();
    output
}

fn bound<'a>(controls: &Arc<Controls>, wire: &'a [u8]) -> WireBoundNativeEffectFactory<'a> {
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(MockFactory {
        controls: Arc::clone(controls),
    });
    bind_native_effect_factory_state(factory, wire, 1 << 20).unwrap()
}

fn admission(preparation: &EffectBankPreparation) -> EffectStateRestoreAdmission {
    EffectStateRestoreAdmission {
        sample_rate: preparation.sample_rate,
        quantum: preparation.quantum,
        maximum_total_state_bytes: preparation.limits.maximum_total_state_bytes,
        maximum_scratch_bytes: preparation.limits.maximum_scratch_bytes,
        maximum_automation_spans_per_block: preparation.limits.maximum_automation_spans_per_block,
    }
}

fn snapshot(
    controls: &Arc<Controls>,
    descriptor_wire: &[u8],
    preparation: &EffectBankPreparation,
) -> Vec<u8> {
    let capability = bound(controls, descriptor_wire);
    let processor = capability.factory().prepare(preparation.request()).unwrap();
    let requirements = scalar_effect_state_requirements(
        &capability,
        preparation,
        EffectStateLimits::default(),
    )
    .unwrap();
    let mut scratch = vec![0xa5; requirements.payload_snapshot_scratch_bytes as usize + 3];
    let mut output = vec![0x5a; requirements.envelope_bytes as usize + 5];
    snapshot_scalar_effect_state(
        &capability,
        preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap();
    assert_eq!(
        &scratch[requirements.payload_snapshot_scratch_bytes as usize..],
        &[0xa5; 3]
    );
    assert_eq!(&output[requirements.envelope_bytes as usize..], &[0x5a; 5]);
    output.truncate(requirements.envelope_bytes as usize);
    output
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

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

#[test]
fn scalar_restore_is_transactional_and_admission_precedes_prepare() {
    for sample_rate in [44_100, 48_000, 88_200, 96_000] {
        let controls = Arc::new(Controls::default());
        let descriptor_wire = wire(&DESCRIPTOR);
        let preparation = preparation(sample_rate);
        let envelope = snapshot(&controls, &descriptor_wire, &preparation);
        let calls_after_snapshot = controls.prepare_calls.load(Ordering::SeqCst);

        let capability = bound(&controls, &descriptor_wire);
        let mut initial_scratch = [
            INITIAL[0],
            INITIAL[1],
            InitialParameterValue {
                parameter_index: 99,
                channel: ParameterChannel::Both,
                value: 0.5,
            },
        ];
        let restored = restore_scalar_effect_state(
            capability,
            &envelope,
            EffectStateLimits::default(),
            admission(&preparation),
            &mut initial_scratch,
        )
        .unwrap();
        assert_eq!(restored.metadata().sample_rate, sample_rate);
        assert_eq!(initial_scratch[2].parameter_index, 99);
        assert_eq!(restored.replay().initial_values.as_ref(), &INITIAL);

        let capability = bound(&controls, &descriptor_wire);
        let mut scratch = [INITIAL[0]; 2];
        let mut too_small = admission(&preparation);
        too_small.maximum_total_state_bytes = 4;
        let before = controls.prepare_calls.load(Ordering::SeqCst);
        let error = restore_scalar_effect_state(
            capability,
            &envelope,
            EffectStateLimits::default(),
            too_small,
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset),
            (EffectStateDiagnosticCode::Limit, 192)
        );
        assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);

        let capability = bound(&controls, &descriptor_wire);
        let mut short = [InitialParameterValue {
            parameter_index: 77,
            channel: ParameterChannel::Both,
            value: 0.25,
        }];
        let before = controls.prepare_calls.load(Ordering::SeqCst);
        let error = restore_scalar_effect_state(
            capability,
            &envelope,
            EffectStateLimits::default(),
            admission(&preparation),
            &mut short,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.detail),
            (
                EffectStateDiagnosticCode::BufferTooSmall,
                EFFECT_STATE_BUFFER_INITIAL_VALUE_SCRATCH
            )
        );
        assert_eq!(short[0].parameter_index, 77);
        assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);

        let mut bad_digest = envelope.clone();
        bad_digest[56] ^= 1;
        let capability = bound(&controls, &descriptor_wire);
        let before = controls.prepare_calls.load(Ordering::SeqCst);
        assert_eq!(
            restore_scalar_effect_state(
                capability,
                &bad_digest,
                EffectStateLimits::default(),
                admission(&preparation),
                &mut scratch
            )
            .unwrap_err()
            .code,
            EffectStateDiagnosticCode::Digest
        );
        assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);
        assert!(controls.prepare_calls.load(Ordering::SeqCst) > calls_after_snapshot);
    }
}

#[test]
fn every_restore_admission_owner_rejects_before_prepare() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let preparation = preparation(48_000);
    let envelope = snapshot(&controls, &descriptor_wire, &preparation);
    let base = admission(&preparation);
    let mut cases: Vec<(Vec<u8>, EffectStateRestoreAdmission, u64, u64)> = Vec::new();

    let mut admission = base;
    admission.sample_rate = 44_100;
    cases.push((envelope.clone(), admission, 96, 48_000));
    let mut admission = base;
    admission.quantum = 31;
    cases.push((envelope.clone(), admission, 100, 32));
    let mut admission = base;
    admission.maximum_total_state_bytes = 4;
    cases.push((envelope.clone(), admission, 192, 5));
    let mut admission = base;
    admission.maximum_scratch_bytes = 7;
    cases.push((envelope.clone(), admission, 200, 8));
    let mut admission = base;
    admission.maximum_automation_spans_per_block = 6;
    cases.push((envelope.clone(), admission, 208, 7));

    let mut extreme_rate = envelope.clone();
    put_u32(&mut extreme_rate, 96, u32::MAX);
    refresh_digest(&mut extreme_rate);
    cases.push((extreme_rate, base, 96, u64::from(u32::MAX)));
    let mut extreme_quantum = envelope.clone();
    put_u32(&mut extreme_quantum, 100, u32::MAX);
    refresh_digest(&mut extreme_quantum);
    cases.push((extreme_quantum, base, 100, u64::from(u32::MAX)));
    let mut extreme_state_cap = envelope.clone();
    put_u64(&mut extreme_state_cap, 192, u64::MAX);
    refresh_digest(&mut extreme_state_cap);
    cases.push((extreme_state_cap, base, 192, u64::MAX));
    let mut extreme_scratch_cap = envelope.clone();
    put_u64(&mut extreme_scratch_cap, 200, u64::MAX);
    refresh_digest(&mut extreme_scratch_cap);
    cases.push((extreme_scratch_cap, base, 200, u64::MAX));
    let mut extreme_automation_cap = envelope.clone();
    put_u32(&mut extreme_automation_cap, 208, u32::MAX);
    refresh_digest(&mut extreme_automation_cap);
    cases.push((extreme_automation_cap, base, 208, u64::from(u32::MAX)));

    let mut derived_payload = envelope.clone();
    put_u64(&mut derived_payload, 192, 4);
    refresh_digest(&mut derived_payload);
    let mut admission = base;
    admission.maximum_total_state_bytes = 4;
    cases.push((derived_payload, admission, 216, 5));
    let mut derived_scratch = envelope.clone();
    put_u64(&mut derived_scratch, 200, 1);
    refresh_digest(&mut derived_scratch);
    let mut admission = base;
    admission.maximum_scratch_bytes = 1;
    cases.push((derived_scratch, admission, 176, 2));
    let mut derived_automation = envelope.clone();
    put_u32(&mut derived_automation, 208, 6);
    refresh_digest(&mut derived_automation);
    let mut admission = base;
    admission.maximum_automation_spans_per_block = 6;
    cases.push((derived_automation, admission, 184, 7));

    for (envelope, admission, offset, required) in cases {
        let capability = bound(&controls, &descriptor_wire);
        let mut scratch = [InitialParameterValue {
            parameter_index: 91,
            channel: ParameterChannel::Both,
            value: 0.5,
        }; 2];
        let before = controls.prepare_calls.load(Ordering::SeqCst);
        let error = restore_scalar_effect_state(
            capability,
            &envelope,
            EffectStateLimits::default(),
            admission,
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.required_bytes),
            (EffectStateDiagnosticCode::Limit, offset, required)
        );
        assert!(scratch.iter().all(|value| value.parameter_index == 91));
        assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);
    }
}

#[test]
fn snapshot_and_restore_failures_publish_nothing() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let preparation = preparation(48_000);
    let capability = bound(&controls, &descriptor_wire);
    let processor = capability.factory().prepare(preparation.request()).unwrap();
    let requirements = scalar_effect_state_requirements(
        &capability,
        &preparation,
        EffectStateLimits::default(),
    )
    .unwrap();
    let snapshot_calls = controls.snapshot_calls.load(Ordering::SeqCst);
    let mut exact_scratch = vec![0x22; requirements.payload_snapshot_scratch_bytes as usize];
    let mut short_output = vec![0x23; requirements.envelope_bytes as usize + 3];
    let short_output_baseline = short_output.clone();
    let short_output_len = requirements.envelope_bytes as usize - 1;
    let error = snapshot_scalar_effect_state(
        &capability,
        &preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut exact_scratch,
        &mut short_output[..short_output_len],
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail, error.required_bytes),
        (
            EffectStateDiagnosticCode::BufferTooSmall,
            EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT,
            requirements.envelope_bytes
        )
    );
    assert_eq!(short_output, short_output_baseline);
    assert_eq!(
        controls.snapshot_calls.load(Ordering::SeqCst),
        snapshot_calls
    );

    let mut short_scratch = vec![0x24; requirements.payload_snapshot_scratch_bytes as usize + 2];
    let short_scratch_baseline = short_scratch.clone();
    let short_scratch_len = requirements.payload_snapshot_scratch_bytes as usize - 1;
    let mut exact_output = vec![0x25; requirements.envelope_bytes as usize];
    let exact_output_baseline = exact_output.clone();
    let error = snapshot_scalar_effect_state(
        &capability,
        &preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut short_scratch[..short_scratch_len],
        &mut exact_output,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail, error.required_bytes),
        (
            EffectStateDiagnosticCode::BufferTooSmall,
            EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH,
            requirements.payload_snapshot_scratch_bytes
        )
    );
    assert_eq!(short_scratch, short_scratch_baseline);
    assert_eq!(exact_output, exact_output_baseline);
    assert_eq!(
        controls.snapshot_calls.load(Ordering::SeqCst),
        snapshot_calls
    );

    let sentinel_before = snapshot_processor(&capability, &preparation, processor.as_ref());
    controls.fail_snapshot.store(true, Ordering::SeqCst);
    let mut scratch = vec![0x44; requirements.payload_snapshot_scratch_bytes as usize + 2];
    let mut output = vec![0x66; requirements.envelope_bytes as usize + 2];
    let baseline = output.clone();
    let error = snapshot_scalar_effect_state(
        &capability,
        &preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Payload, 1)
    );
    assert_eq!(output, baseline);
    assert_eq!(
        &scratch[requirements.payload_snapshot_scratch_bytes as usize..],
        &[0x44; 2]
    );

    controls.fail_snapshot.store(false, Ordering::SeqCst);
    let envelope = snapshot(&controls, &descriptor_wire, &preparation);
    controls.fail_restore.store(true, Ordering::SeqCst);
    let capability = bound(&controls, &descriptor_wire);
    let mut initial = INITIAL;
    let error = restore_scalar_effect_state(
        capability,
        &envelope,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Payload, 3)
    );

    controls.fail_restore.store(false, Ordering::SeqCst);
    controls.fail_prepare.store(true, Ordering::SeqCst);
    let capability = bound(&controls, &descriptor_wire);
    let error = restore_scalar_effect_state(
        capability,
        &envelope,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Factory, 3)
    );

    controls.fail_prepare.store(false, Ordering::SeqCst);
    controls.wrong_metadata.store(true, Ordering::SeqCst);
    let capability = bound(&controls, &descriptor_wire);
    let error = restore_scalar_effect_state(
        capability,
        &envelope,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Factory, 4)
    );
    let sentinel_capability = bound(&controls, &descriptor_wire);
    let sentinel_after = snapshot_processor(&sentinel_capability, &preparation, processor.as_ref());
    assert_eq!(sentinel_after, sentinel_before);
}

#[test]
fn binding_and_snapshot_metadata_failures_are_exact_and_atomic() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let alternate_wire = wire(&ALTERNATE_DESCRIPTOR);
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(MockFactory {
        controls: Arc::clone(&controls),
    });
    let error = bind_native_effect_factory_state(Arc::clone(&factory), &alternate_wire, 1 << 20)
        .unwrap_err();
    assert_eq!(
        error.detail >> 16,
        EffectDescriptorBindingErrorKind::StaticDescriptorMismatch as u32
    );
    let error = bind_native_effect_factory_state(factory, &[], 1 << 20).unwrap_err();
    assert_eq!(error.detail >> 16, 1);
    assert_eq!(error.byte_offset, 0);

    controls.wrong_metadata.store(true, Ordering::SeqCst);
    let capability = bound(&controls, &descriptor_wire);
    let preparation = preparation(48_000);
    let processor = capability.factory().prepare(preparation.request()).unwrap();
    let requirements = scalar_effect_state_requirements(
        &capability,
        &preparation,
        EffectStateLimits::default(),
    )
    .unwrap();
    let mut scratch = vec![0x33; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0x55; requirements.envelope_bytes as usize];
    let baseline = output.clone();
    let calls = controls.snapshot_calls.load(Ordering::SeqCst);
    let error = snapshot_scalar_effect_state(
        &capability,
        &preparation,
        processor.as_ref(),
        EffectStateLimits::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Metadata, 1)
    );
    assert_eq!(output, baseline);
    assert_eq!(controls.snapshot_calls.load(Ordering::SeqCst), calls);
}

#[test]
fn malformed_replay_is_rejected_before_initial_scratch() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let preparation = preparation(48_000);
    let canonical = snapshot(&controls, &descriptor_wire, &preparation);
    let mut envelope = canonical.clone();
    envelope[224] = b'u';
    refresh_digest(&mut envelope);
    let capability = bound(&controls, &descriptor_wire);
    let mut initial = [InitialParameterValue {
        parameter_index: 88,
        channel: ParameterChannel::Both,
        value: 0.5,
    }; 2];
    let before = controls.prepare_calls.load(Ordering::SeqCst);
    let error = restore_scalar_effect_state(
        capability,
        &envelope,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Metadata, 1)
    );
    assert!(initial.iter().all(|value| value.parameter_index == 88));
    assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);

    let mut wrong_sizes = canonical;
    put_u32(&mut wrong_sizes, 160, 2);
    put_u32(&mut wrong_sizes, 168, 1);
    refresh_digest(&mut wrong_sizes);
    let capability = bound(&controls, &descriptor_wire);
    let before = controls.prepare_calls.load(Ordering::SeqCst);
    let error = restore_scalar_effect_state(
        capability,
        &wrong_sizes,
        EffectStateLimits::default(),
        admission(&preparation),
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCode::Metadata, 12)
    );
    assert!(initial.iter().all(|value| value.parameter_index == 88));
    assert_eq!(controls.prepare_calls.load(Ordering::SeqCst), before);
}

fn delay_preparation(sample_rate: u32) -> EffectBankPreparation {
    let descriptor = &miso_engine_delay::DELAY_DESCRIPTOR;
    let values: Vec<_> = descriptor
        .parameters
        .iter()
        .enumerate()
        .flat_map(|(parameter_index, parameter)| {
            let channels: &[ParameterChannel] = match parameter.channel_policy {
                ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
                ParameterChannelPolicy::PerLane => {
                    &[ParameterChannel::Left, ParameterChannel::Right]
                }
            };
            channels.iter().map(move |channel| InitialParameterValue {
                parameter_index: parameter_index as u32,
                channel: *channel,
                value: parameter.default_value,
            })
        })
        .collect();
    let mut values = values.into_boxed_slice();
    for value in &mut values {
        value.value = match (value.parameter_index, value.channel) {
            (0, ParameterChannel::Left) => 1.0,
            (0, ParameterChannel::Right) => 2.0,
            (1, ParameterChannel::Left) => 0.4,
            (1, ParameterChannel::Right) => -0.3,
            (2, ParameterChannel::Left) => 0.2,
            (2, ParameterChannel::Right) => 0.4,
            (3, ParameterChannel::Left) => 0.7,
            (3, ParameterChannel::Right) => 0.6,
            (4, ParameterChannel::Both) => 0.25,
            _ => value.value,
        };
    }
    let quality = descriptor
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == sample_rate)
        .unwrap();
    EffectBankPreparation {
        sample_rate,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().unwrap(),
            maximum_scratch_bytes: quality.scratch_fixed_bytes,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn render_partition(
    processor: &mut dyn PreparedNativeEffect,
    first_sample: u64,
    frames: usize,
) -> (Vec<u32>, Vec<u32>, ProcessReport) {
    let mut left: Vec<_> = (0..frames)
        .map(|index| ((first_sample as usize + index) % 17) as f32 * 0.01 - 0.08)
        .collect();
    let mut right: Vec<_> = (0..frames)
        .map(|index| ((first_sample as usize + index * 3) % 19) as f32 * -0.008 + 0.07)
        .collect();
    let report = processor.process(
        EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], 128).unwrap(),
    );
    (
        left.into_iter().map(f32::to_bits).collect(),
        right.into_iter().map(f32::to_bits).collect(),
        report,
    )
}

fn snapshot_processor(
    capability: &WireBoundNativeEffectFactory<'_>,
    replay: &EffectBankPreparation,
    processor: &dyn PreparedNativeEffect,
) -> Vec<u8> {
    let requirements =
        scalar_effect_state_requirements(capability, replay, EffectStateLimits::default())
            .unwrap();
    let mut scratch = vec![0; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; requirements.envelope_bytes as usize];
    snapshot_scalar_effect_state(
        capability,
        replay,
        processor,
        EffectStateLimits::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap();
    output
}

#[test]
fn production_delay_active_common_and_lane_state_continues_exactly() {
    let descriptor_wire = wire(&miso_engine_delay::DELAY_DESCRIPTOR);
    let replay = delay_preparation(48_000);
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(miso_engine_delay::DelayFactory);
    let capability =
        bind_native_effect_factory_state(Arc::clone(&factory), &descriptor_wire, 1 << 20)
            .unwrap();
    let mut original = factory.prepare(replay.request()).unwrap();
    let mut cursor = 0_u64;
    for frames in [73, 41, 89, 54] {
        let _ = render_partition(original.as_mut(), cursor, frames);
        cursor += frames as u64;
    }
    let envelope = snapshot_processor(&capability, &replay, original.as_ref());
    let restore_capability =
        bind_native_effect_factory_state(Arc::clone(&factory), &descriptor_wire, 1 << 20)
            .unwrap();
    let mut initial_scratch = vec![replay.initial_values[0]; replay.initial_values.len() + 2];
    initial_scratch[replay.initial_values.len()].parameter_index = 77;
    let mut restored = restore_scalar_effect_state(
        restore_capability,
        &envelope,
        EffectStateLimits::default(),
        admission(&replay),
        &mut initial_scratch,
    )
    .unwrap();
    assert_eq!(
        initial_scratch[replay.initial_values.len()].parameter_index,
        77
    );

    for frames in [17, 64, 3, 91, 29] {
        let expected = render_partition(original.as_mut(), cursor, frames);
        let actual = render_partition(restored.processor_mut(), cursor, frames);
        assert_eq!(actual, expected);
        cursor += frames as u64;
    }
    let original_next = snapshot_processor(&capability, &replay, original.as_ref());
    let restored_next = snapshot_processor(
        restored.bound_factory(),
        restored.replay(),
        restored.processor(),
    );
    assert_eq!(restored_next, original_next);
}

#[test]
fn production_delay_state_round_trips_at_every_launch_rate() {
    let descriptor_wire = wire(&miso_engine_delay::DELAY_DESCRIPTOR);
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(miso_engine_delay::DelayFactory);
    for sample_rate in [44_100, 48_000, 88_200, 96_000] {
        let replay = delay_preparation(sample_rate);
        let capability =
            bind_native_effect_factory_state(Arc::clone(&factory), &descriptor_wire, 1 << 20)
                .unwrap();
        let mut original = factory.prepare(replay.request()).unwrap();
        let _ = render_partition(original.as_mut(), 0, 97);
        let envelope = snapshot_processor(&capability, &replay, original.as_ref());
        let restore_capability =
            bind_native_effect_factory_state(Arc::clone(&factory), &descriptor_wire, 1 << 20)
                .unwrap();
        let mut initial_scratch = vec![replay.initial_values[0]; replay.initial_values.len()];
        let restored = restore_scalar_effect_state(
            restore_capability,
            &envelope,
            EffectStateLimits::default(),
            admission(&replay),
            &mut initial_scratch,
        )
        .unwrap();
        assert_eq!(restored.metadata().sample_rate, sample_rate);
        assert_eq!(restored.metadata().tail, TailSamples::Infinite);
        assert_eq!(
            snapshot_processor(
                restored.bound_factory(),
                restored.replay(),
                restored.processor()
            ),
            envelope
        );
    }
}
