//! Transactional unpublished-bank current-layout state coverage.

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use miso_engine_lane::Backend;
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

static PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
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
    nudge_ladder: None,
    enum_choices: &[],
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
const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
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
static QUALITIES: [QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];
static DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.bank-state"),
    display_name: "Bank state",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
};
static ALTERNATE_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    display_name: "Alternate bank state",
    ..DESCRIPTOR
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Payload {
    common: [u8; 1],
    left: [u8; 2],
    right: [u8; 2],
}

impl Payload {
    fn from_request(request: PrepareEffectRequest<'_>) -> Self {
        let left = request.initial_values[0].value.to_bits().to_le_bytes();
        let right = request.initial_values[1].value.to_bits().to_le_bytes();
        Self {
            common: [0x10],
            left: [left[0], left[1]],
            right: [right[0], right[1]],
        }
    }

    fn process(&mut self, left: &mut f32, right: &mut f32) {
        *left += f32::from(self.common[0].wrapping_add(self.left[0])) / 256.0;
        *right += f32::from(self.common[0].wrapping_add(self.right[0])) / 256.0;
        self.common[0] = self.common[0].wrapping_add(1);
        self.left[0] = self.left[0].wrapping_add(3);
        self.left[1] = self.left[1].wrapping_add(5);
        self.right[0] = self.right[0].wrapping_add(7);
        self.right[1] = self.right[1].wrapping_add(11);
    }

    fn snapshot(self, output: StatePayloadOutput<'_>) {
        output.common.copy_from_slice(&self.common);
        output.left.copy_from_slice(&self.left);
        output.right.copy_from_slice(&self.right);
    }

    fn restore(&mut self, input: StatePayloadInput<'_>) {
        self.common.copy_from_slice(input.common);
        self.left.copy_from_slice(input.left);
        self.right.copy_from_slice(input.right);
    }
}

#[derive(Default)]
struct Controls {
    bank_bind_calls: AtomicUsize,
    bank_snapshot_calls: AtomicUsize,
    bank_restore_calls: AtomicUsize,
    bank_drops: AtomicUsize,
    return_none: AtomicBool,
    fail_restore: AtomicBool,
    wrong_width: AtomicBool,
    wrong_program_key: AtomicBool,
    alternate_descriptor: AtomicBool,
}

struct MockFactory {
    controls: Arc<Controls>,
}

impl MockFactory {
    fn current_descriptor(&self) -> &'static EffectDescriptorV1 {
        if self.controls.alternate_descriptor.load(Ordering::SeqCst) {
            &ALTERNATE_DESCRIPTOR
        } else {
            &DESCRIPTOR
        }
    }
}

struct MockScalar {
    metadata: PreparedEffectMetadata,
    payload: Payload,
}

struct MockBank {
    controls: Arc<Controls>,
    metadata: PreparedBankMetadata,
    payloads: Vec<Payload>,
}

impl Drop for MockBank {
    fn drop(&mut self) {
        self.controls.bank_drops.fetch_add(1, Ordering::SeqCst);
    }
}

impl NativeEffectFactory for MockFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        self.current_descriptor()
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.current_descriptor(), request)?;
        Ok(Box::new(MockScalar {
            metadata,
            payload: Payload::from_request(request),
        }))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        self.controls.bank_bind_calls.fetch_add(1, Ordering::SeqCst);
        if self.controls.return_none.load(Ordering::SeqCst) {
            return Ok(None);
        }
        let first = expected_prepared_metadata(self.current_descriptor(), request.requests[0])?;
        let payloads = request
            .requests
            .iter()
            .copied()
            .map(Payload::from_request)
            .collect();
        Ok(Some(Box::new(MockBank {
            controls: Arc::clone(&self.controls),
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: first.program_key(),
            },
            payloads,
        })))
    }
}

impl PreparedNativeEffect for MockScalar {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, _: ResetKind) {}

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            self.payload.process(left, right);
        }
        ProcessReport::default()
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.payload.snapshot(output);
        Ok(())
    }

    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != 1 {
            return Err(StatePayloadError { code: "version" });
        }
        self.payload.restore(input);
        Ok(())
    }
}

impl PreparedNativeEffectBank for MockBank {
    fn metadata(&self) -> PreparedBankMetadata {
        let mut metadata = self.metadata.clone();
        if self.controls.wrong_width.load(Ordering::SeqCst) {
            metadata.width = BankWidth::Eight;
        }
        if self.controls.wrong_program_key.load(Ordering::SeqCst) {
            metadata.program_key.quantum += 1;
        }
        metadata
    }

    fn reset(&mut self, _: ResetKind) {}

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let width = block.width.lanes() as usize;
        for frame in 0..block.frames as usize {
            for track in 0..width {
                let index = frame * width + track;
                self.payloads[track].process(&mut block.left[index], &mut block.right[index]);
            }
        }
        BankProcessReport::empty(block.width)
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.controls
            .bank_snapshot_calls
            .fetch_add(1, Ordering::SeqCst);
        self.payloads[track_index as usize].snapshot(output);
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.controls
            .bank_restore_calls
            .fetch_add(1, Ordering::SeqCst);
        self.payloads[track_index as usize]
            .common
            .copy_from_slice(input.common);
        if self.controls.fail_restore.load(Ordering::SeqCst) {
            return Err(StatePayloadError { code: "partial" });
        }
        if version != 1 {
            return Err(StatePayloadError { code: "version" });
        }
        self.payloads[track_index as usize]
            .left
            .copy_from_slice(input.left);
        self.payloads[track_index as usize]
            .right
            .copy_from_slice(input.right);
        Ok(())
    }
}

fn wire(descriptor: &'static EffectDescriptorV1) -> Vec<u8> {
    let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
    let mut output = vec![0; required as usize];
    encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut output).unwrap();
    output
}

fn replay(seed: u32) -> EffectBankPreparationV1 {
    EffectBankPreparationV1 {
        sample_rate: 48_000,
        quantum: 32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: [
            InitialParameterValue {
                parameter_index: 0,
                channel: ParameterChannel::Left,
                value: seed as f32 / 16.0,
            },
            InitialParameterValue {
                parameter_index: 0,
                channel: ParameterChannel::Right,
                value: -(seed as f32) / 16.0,
            },
        ]
        .into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 5,
            maximum_scratch_bytes: 2,
            maximum_automation_spans_per_block: 7,
        },
    }
}

fn admission(replay: &EffectBankPreparationV1) -> EffectStateRestoreAdmissionV1 {
    EffectStateRestoreAdmissionV1 {
        sample_rate: replay.sample_rate,
        quantum: replay.quantum,
        maximum_total_state_bytes: replay.limits.maximum_total_state_bytes,
        maximum_scratch_bytes: replay.limits.maximum_scratch_bytes,
        maximum_automation_spans_per_block: replay.limits.maximum_automation_spans_per_block,
    }
}

fn mock_bound<'a>(
    controls: &Arc<Controls>,
    descriptor_wire: &'a [u8],
) -> WireBoundNativeEffectFactoryV1<'a> {
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(MockFactory {
        controls: Arc::clone(controls),
    });
    bind_native_effect_factory_state_v1(factory, descriptor_wire, 1 << 20).unwrap()
}

fn mock_bank<'a>(
    controls: &Arc<Controls>,
    descriptor_wire: &'a [u8],
) -> UnpublishedEffectBankStateV1<'a> {
    prepare_unpublished_effect_bank_state_v1(
        mock_bound(controls, descriptor_wire),
        Backend::Simd4,
        BankWidth::Four,
        (1..=4).map(replay).collect::<Vec<_>>().into_boxed_slice(),
        admission(&replay(1)),
    )
    .unwrap()
}

fn scalar_snapshot(
    capability: &WireBoundNativeEffectFactoryV1<'_>,
    replay: &EffectBankPreparationV1,
    processor: &dyn PreparedNativeEffect,
) -> Vec<u8> {
    let requirements =
        scalar_effect_state_v1_requirements(capability, replay, EffectStateLimitsV1::default())
            .unwrap();
    let mut scratch = vec![0; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; requirements.envelope_bytes as usize];
    snapshot_scalar_effect_state_v1(
        capability,
        replay,
        processor,
        EffectStateLimitsV1::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap();
    output
}

fn bank_snapshot(capability: &UnpublishedEffectBankStateV1<'_>, track_index: u32) -> Vec<u8> {
    let replay = &capability.replays()[track_index as usize];
    let requirements = scalar_effect_state_v1_requirements(
        capability.bound_factory(),
        replay,
        EffectStateLimitsV1::default(),
    )
    .unwrap();
    let mut scratch = vec![0; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0; requirements.envelope_bytes as usize];
    snapshot_unpublished_effect_bank_track_state_v1(
        capability,
        track_index,
        EffectStateLimitsV1::default(),
        &mut scratch,
        &mut output,
    )
    .unwrap();
    output
}

fn process_scalar(processor: &mut dyn PreparedNativeEffect, frames: usize) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![0.25; frames];
    let mut right = vec![-0.125; frames];
    processor.process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 32).unwrap());
    (
        left.into_iter().map(f32::to_bits).collect(),
        right.into_iter().map(f32::to_bits).collect(),
    )
}

fn process_mock_bank(
    capability: &mut UnpublishedEffectBankStateV1<'_>,
    frames: usize,
) -> (Vec<u32>, Vec<u32>) {
    let width = capability.width().lanes() as usize;
    let mut left = vec![0.25; frames * width];
    let mut right = vec![-0.125; frames * width];
    let offsets = vec![0; width + 1];
    capability.bank_mut().process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            frames as u32,
            BankWidth::Four,
            0,
            &[],
            &offsets,
            32,
        )
        .unwrap(),
    );
    (
        left.into_iter().map(f32::to_bits).collect(),
        right.into_iter().map(f32::to_bits).collect(),
    )
}

#[test]
fn preparation_admits_every_sibling_before_one_bind_and_none_is_failure() {
    let descriptor_wire = wire(&DESCRIPTOR);
    for mutate in 0..5 {
        let controls = Arc::new(Controls::default());
        let mut replays: Vec<_> = (1..=4).map(replay).collect();
        let last = replays.last_mut().unwrap();
        match mutate {
            0 => last.sample_rate = 44_100,
            1 => last.quantum = 31,
            2 => last.limits.maximum_total_state_bytes = 6,
            3 => last.limits.maximum_scratch_bytes = 3,
            _ => last.limits.maximum_automation_spans_per_block = 8,
        }
        let error = prepare_unpublished_effect_bank_state_v1(
            mock_bound(&controls, &descriptor_wire),
            Backend::Simd4,
            BankWidth::Four,
            replays.into_boxed_slice(),
            admission(&replay(1)),
        )
        .unwrap_err();
        assert_eq!(error.code, EffectStateDiagnosticCodeV1::Limit);
        assert_eq!(error.item_index, 3);
        assert_eq!(controls.bank_bind_calls.load(Ordering::SeqCst), 0);
    }

    let controls = Arc::new(Controls::default());
    let mut replays: Vec<_> = (1..=4).map(replay).collect();
    replays[0].limits.maximum_total_state_bytes = 4;
    replays[0].initial_values[0].value = f32::NAN;
    let mut derived_admission = admission(&replay(1));
    derived_admission.maximum_total_state_bytes = 4;
    let error = prepare_unpublished_effect_bank_state_v1(
        mock_bound(&controls, &descriptor_wire),
        Backend::Simd4,
        BankWidth::Four,
        replays.into_boxed_slice(),
        derived_admission,
    )
    .unwrap_err();
    assert_eq!(
        (
            error.code,
            error.byte_offset,
            error.required_bytes,
            error.item_index
        ),
        (EffectStateDiagnosticCodeV1::Limit, 216, 5, 0)
    );
    assert_eq!(controls.bank_bind_calls.load(Ordering::SeqCst), 0);

    let controls = Arc::new(Controls::default());
    controls.return_none.store(true, Ordering::SeqCst);
    let error = prepare_unpublished_effect_bank_state_v1(
        mock_bound(&controls, &descriptor_wire),
        Backend::Simd4,
        BankWidth::Four,
        (1..=4).map(replay).collect::<Vec<_>>().into_boxed_slice(),
        admission(&replay(1)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Factory, 3)
    );
    assert_eq!(controls.bank_bind_calls.load(Ordering::SeqCst), 1);
}

#[test]
fn scalar_and_bank_payloads_interchange_and_continue_exactly() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let replay = replay(2);
    let scalar_capability = mock_bound(&controls, &descriptor_wire);
    let mut scalar = scalar_capability
        .factory()
        .prepare(replay.request())
        .unwrap();
    let _ = process_scalar(scalar.as_mut(), 9);
    let scalar_envelope = scalar_snapshot(&scalar_capability, &replay, scalar.as_ref());

    let bank = mock_bank(&controls, &descriptor_wire);
    let mut bank = restore_unpublished_effect_bank_track_state_v1(
        bank,
        1,
        &scalar_envelope,
        EffectStateLimitsV1::default(),
        admission(&replay),
    )
    .unwrap();
    let expected = process_scalar(scalar.as_mut(), 7);
    let actual = process_mock_bank(&mut bank, 7);
    let width = BankWidth::Four.lanes() as usize;
    assert_eq!(
        actual
            .0
            .iter()
            .skip(1)
            .step_by(width)
            .copied()
            .collect::<Vec<_>>(),
        expected.0
    );
    assert_eq!(
        actual
            .1
            .iter()
            .skip(1)
            .step_by(width)
            .copied()
            .collect::<Vec<_>>(),
        expected.1
    );

    let bank_envelope = bank_snapshot(&bank, 1);
    let scalar_restore_capability = mock_bound(&controls, &descriptor_wire);
    let mut initial_scratch = vec![replay.initial_values[0]; replay.initial_values.len()];
    let mut restored_scalar = restore_scalar_effect_state_v1(
        scalar_restore_capability,
        &bank_envelope,
        EffectStateLimitsV1::default(),
        admission(&replay),
        &mut initial_scratch,
    )
    .unwrap();
    let bank_expected = process_mock_bank(&mut bank, 5);
    let scalar_actual = process_scalar(restored_scalar.processor_mut(), 5);
    assert_eq!(
        bank_expected
            .0
            .iter()
            .skip(1)
            .step_by(width)
            .copied()
            .collect::<Vec<_>>(),
        scalar_actual.0
    );
    assert_eq!(
        bank_expected
            .1
            .iter()
            .skip(1)
            .step_by(width)
            .copied()
            .collect::<Vec<_>>(),
        scalar_actual.1
    );
}

#[test]
fn bank_restore_is_serial_and_isolates_siblings() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let mut source = mock_bank(&controls, &descriptor_wire);
    let _ = process_mock_bank(&mut source, 6);
    let first = bank_snapshot(&source, 1);
    let second = bank_snapshot(&source, 2);
    let destination = mock_bank(&controls, &descriptor_wire);
    let sibling_before = bank_snapshot(&destination, 0);
    let destination = restore_unpublished_effect_bank_track_state_v1(
        destination,
        1,
        &first,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap();
    let mut destination = restore_unpublished_effect_bank_track_state_v1(
        destination,
        2,
        &second,
        EffectStateLimitsV1::default(),
        admission(&replay(3)),
    )
    .unwrap();
    assert_eq!(bank_snapshot(&destination, 0), sibling_before);
    assert_eq!(bank_snapshot(&destination, 1), first);
    assert_eq!(bank_snapshot(&destination, 2), second);
    let source_output = process_mock_bank(&mut source, 5);
    let destination_output = process_mock_bank(&mut destination, 5);
    let width = BankWidth::Four.lanes() as usize;
    for track in [1, 2] {
        assert_eq!(
            source_output
                .0
                .iter()
                .skip(track)
                .step_by(width)
                .copied()
                .collect::<Vec<_>>(),
            destination_output
                .0
                .iter()
                .skip(track)
                .step_by(width)
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            source_output
                .1
                .iter()
                .skip(track)
                .step_by(width)
                .copied()
                .collect::<Vec<_>>(),
            destination_output
                .1
                .iter()
                .skip(track)
                .step_by(width)
                .copied()
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn bank_failures_drop_partial_ownership_and_preserve_first_error_order() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let source = mock_bank(&controls, &descriptor_wire);
    let envelope = bank_snapshot(&source, 1);
    drop(source);
    let baseline_drops = controls.bank_drops.load(Ordering::SeqCst);

    let bad_index = mock_bank(&controls, &descriptor_wire);
    controls.wrong_program_key.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        bad_index,
        9,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Restore, 1)
    );
    controls.wrong_program_key.store(false, Ordering::SeqCst);

    let bad_config = mock_bank(&controls, &descriptor_wire);
    controls.wrong_width.store(true, Ordering::SeqCst);
    controls.wrong_program_key.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        bad_config,
        1,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Restore, 2)
    );
    controls.wrong_width.store(false, Ordering::SeqCst);
    controls.wrong_program_key.store(false, Ordering::SeqCst);

    let bad_replay = mock_bank(&controls, &descriptor_wire);
    controls.wrong_program_key.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        bad_replay,
        2,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(3)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Restore, 2)
    );
    controls.wrong_program_key.store(false, Ordering::SeqCst);

    let bad_key = mock_bank(&controls, &descriptor_wire);
    controls.wrong_program_key.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        bad_key,
        1,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Restore, 3)
    );
    controls.wrong_program_key.store(false, Ordering::SeqCst);

    let bad_provenance = mock_bank(&controls, &descriptor_wire);
    controls.alternate_descriptor.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        bad_provenance,
        1,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Restore, 4)
    );
    controls.alternate_descriptor.store(false, Ordering::SeqCst);

    let partial = mock_bank(&controls, &descriptor_wire);
    controls.fail_restore.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_v1(
        partial,
        1,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay(2)),
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateDiagnosticCodeV1::Payload, 4)
    );
    assert!(controls.bank_restore_calls.load(Ordering::SeqCst) > 0);
    assert_eq!(
        controls.bank_drops.load(Ordering::SeqCst),
        baseline_drops + 6
    );
}

#[test]
fn bank_snapshot_one_short_never_calls_hook_or_publishes_output() {
    let controls = Arc::new(Controls::default());
    let descriptor_wire = wire(&DESCRIPTOR);
    let bank = mock_bank(&controls, &descriptor_wire);
    let replay = &bank.replays()[1];
    let requirements = scalar_effect_state_v1_requirements(
        bank.bound_factory(),
        replay,
        EffectStateLimitsV1::default(),
    )
    .unwrap();
    let calls = controls.bank_snapshot_calls.load(Ordering::SeqCst);
    let mut scratch = vec![0x31; requirements.payload_snapshot_scratch_bytes as usize];
    let mut output = vec![0x41; requirements.envelope_bytes as usize + 3];
    let baseline = output.clone();
    let output_len = requirements.envelope_bytes as usize - 1;
    let error = snapshot_unpublished_effect_bank_track_state_v1(
        &bank,
        1,
        EffectStateLimitsV1::default(),
        &mut scratch,
        &mut output[..output_len],
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (
            EffectStateDiagnosticCodeV1::BufferTooSmall,
            EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT
        )
    );
    assert_eq!(output, baseline);
    assert_eq!(controls.bank_snapshot_calls.load(Ordering::SeqCst), calls);

    let mut short_scratch = vec![0x51; requirements.payload_snapshot_scratch_bytes as usize + 2];
    let scratch_baseline = short_scratch.clone();
    let scratch_len = requirements.payload_snapshot_scratch_bytes as usize - 1;
    let mut exact_output = vec![0x61; requirements.envelope_bytes as usize];
    let output_baseline = exact_output.clone();
    let error = snapshot_unpublished_effect_bank_track_state_v1(
        &bank,
        1,
        EffectStateLimitsV1::default(),
        &mut short_scratch[..scratch_len],
        &mut exact_output,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (
            EffectStateDiagnosticCodeV1::BufferTooSmall,
            EFFECT_STATE_V1_BUFFER_PAYLOAD_SCRATCH
        )
    );
    assert_eq!(short_scratch, scratch_baseline);
    assert_eq!(exact_output, output_baseline);
    assert_eq!(controls.bank_snapshot_calls.load(Ordering::SeqCst), calls);
}

fn production_replay(descriptor: &'static EffectDescriptorV1) -> EffectBankPreparationV1 {
    let initial_values = descriptor
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
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let quality = descriptor
        .qualities
        .iter()
        .find(|quality| quality.quality == EffectQuality::Normal && quality.sample_rate == 48_000)
        .unwrap();
    EffectBankPreparationV1 {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().unwrap(),
            maximum_scratch_bytes: quality.scratch_fixed_bytes
                + quality.scratch_bytes_per_frame * 128,
            maximum_automation_spans_per_block: 16,
        },
    }
}

#[test]
fn production_soft_clip_w8_member_restores_and_isolates_siblings() {
    let backend = Backend::current();
    if backend != Backend::Simd8 {
        return;
    }
    let descriptor = &miso_engine_soft_clip::SOFT_CLIP_DESCRIPTOR_V1;
    let descriptor_wire = wire(descriptor);
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(miso_engine_soft_clip::SoftClipFactory);
    let replay = production_replay(descriptor);
    let prepare_bank = || {
        prepare_unpublished_effect_bank_state_v1(
            bind_native_effect_factory_state_v1(Arc::clone(&factory), &descriptor_wire, 1 << 20)
                .unwrap(),
            backend,
            BankWidth::Eight,
            vec![replay.clone(); 8].into_boxed_slice(),
            admission(&replay),
        )
        .unwrap()
    };
    let mut source = prepare_bank();
    let width = 8;
    let frames = 37;
    let mut left: Vec<_> = (0..frames * width)
        .map(|index| index as f32 * 0.001 - 0.1)
        .collect();
    let mut right: Vec<_> = (0..frames * width)
        .map(|index| index as f32 * -0.001 + 0.1)
        .collect();
    let offsets = [0; 9];
    source.bank_mut().process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            frames as u32,
            BankWidth::Eight,
            0,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    let envelope = bank_snapshot(&source, 3);
    let destination = prepare_bank();
    let sibling_before = bank_snapshot(&destination, 2);
    let mut destination = restore_unpublished_effect_bank_track_state_v1(
        destination,
        3,
        &envelope,
        EffectStateLimitsV1::default(),
        admission(&replay),
    )
    .unwrap();
    assert_eq!(bank_snapshot(&destination, 2), sibling_before);
    assert_eq!(bank_snapshot(&destination, 3), envelope);

    let continuation_frames = 19;
    let continuation_size = continuation_frames * width;
    let source_left: Vec<_> = (0..continuation_size)
        .map(|index| index as f32 * 0.002 - 0.2)
        .collect();
    let source_right: Vec<_> = (0..continuation_size)
        .map(|index| index as f32 * -0.0015 + 0.15)
        .collect();
    let mut expected_left = source_left.clone();
    let mut expected_right = source_right.clone();
    let mut actual_left = source_left;
    let mut actual_right = source_right;
    source.bank_mut().process_bank(
        EffectBankProcessBlock::new(
            &mut expected_left,
            &mut expected_right,
            None,
            continuation_frames as u32,
            BankWidth::Eight,
            frames as u64,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    destination.bank_mut().process_bank(
        EffectBankProcessBlock::new(
            &mut actual_left,
            &mut actual_right,
            None,
            continuation_frames as u32,
            BankWidth::Eight,
            frames as u64,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
    assert_eq!(
        expected_left
            .iter()
            .skip(3)
            .step_by(width)
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        actual_left
            .iter()
            .skip(3)
            .step_by(width)
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        expected_right
            .iter()
            .skip(3)
            .step_by(width)
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        actual_right
            .iter()
            .skip(3)
            .step_by(width)
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(bank_snapshot(&destination, 3), bank_snapshot(&source, 3));
}
