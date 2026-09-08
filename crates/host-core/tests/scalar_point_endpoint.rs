//! Product gates for borrowed scalar compressor Point delivery.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;
use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ObservationSample, ParameterAccessError, ParameterChannel,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedParameterState, PreparedPorts, PreparedSidechainPort,
    ProcessReport, ResetKind, StatePayloadError, StatePayloadInput, StatePayloadOutput,
};
use host_core::{
    ControllerScalarPointPrepareError, PlanSampleSource, ScalarPointAdmissionError,
    ScalarPointCancelBoundaryError, ScalarPointFault, ScalarPointFaultCause,
    ScalarPointFaultProgress, ScalarPointNativeOperation, ScalarPointRenderError,
    SessionControlProvider, prepare_controller_scalar_point_endpoint,
    prepare_scalar_point_endpoint, publish_controller_scalar_point_snapshot,
};
use protocol::{
    AutomationBatchError, AutomationBatchSlot, AutomationCancellationReason, AutomationEnqueue,
    AutomationEnqueueError, AutomationKind, AutomationRecord, CommandPayload, ControlCommand,
    ControlProvider, ControllerAutomationDelivery, ControllerAutomationPrepareError,
    ControllerRequest, ControllerRetainedCapacity, CounterId, CountersRequest, DecodeScratch,
    DecodedEventPayload, DecodedSuccessResponsePayload, DecodedTypedEventFrame,
    DecodedTypedResponseFrame, DeliveryError, ExpectedRevision, HandoffResult,
    ParameterChannel as ProtocolParameterChannel, ParameterHandle, ParameterMetadataRequest,
    ParameterStatePage, ParameterStateRequest, PreparedAutomationDelivery,
    PreparedDeliveryCapabilities, ProtocolCodec, ProtocolControllerConfig, ProtocolQueueConfig,
    ProviderFeatures, ReliablePayload, ReplayCacheConfig, RequestId, SampleTime, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

const Q: usize = 16;
const REV: SessionRevision = SessionRevision(7);
const H: [ParameterHandle; 2] = [ParameterHandle(11), ParameterHandle(12)];
fn config() -> ProtocolQueueConfig {
    ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(2).unwrap(),
        control_command_bytes: NonZeroUsize::new(256).unwrap(),
        automation_batch_slots: NonZeroUsize::new(2).unwrap(),
        reliable_response_slots: NonZeroUsize::new(2).unwrap(),
        reliable_event_slots: NonZeroUsize::new(4).unwrap(),
        telemetry_slots: NonZeroUsize::new(2).unwrap(),
        per_block_automation_density: NonZeroUsize::new(256).unwrap(),
        quantum_frames: NonZeroUsize::new(Q).unwrap(),
    }
}

#[derive(Clone, Copy, Debug)]
struct PreparationAllocationDiagnostic {
    global: bench_support::alloc::Counters,
    current_thread: bench_support::alloc::Counters,
    thread_audit: engine::realtime::audit::AuditSnapshot,
}

fn measure_preparation<T>(prepare: impl FnOnce() -> T) -> (T, PreparationAllocationDiagnostic) {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;

    audit::warm_up();
    audit::reset();
    let global_mark = bench_alloc::counters();
    let current_thread_mark = bench_alloc::current_thread_counters();
    let (value, thread_audit) = audit::in_render_scope(|| {
        let value = prepare();
        (value, audit::snapshot())
    });
    (
        value,
        PreparationAllocationDiagnostic {
            global: bench_alloc::delta_since(global_mark),
            current_thread: bench_alloc::current_thread_delta_since(current_thread_mark),
            thread_audit,
        },
    )
}

fn foreign_thread_allocation_probe() -> (
    bench_support::alloc::Counters,
    bench_support::alloc::Counters,
    engine::realtime::audit::AuditSnapshot,
) {
    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;
    use std::hint::black_box;
    use std::sync::{Arc, Barrier};

    let ready = Arc::new(Barrier::new(2));
    let start = Arc::new(Barrier::new(2));
    let finished = Arc::new(Barrier::new(2));
    let worker_ready = Arc::clone(&ready);
    let worker_start = Arc::clone(&start);
    let worker_finished = Arc::clone(&finished);
    let worker = std::thread::spawn(move || {
        worker_ready.wait();
        worker_start.wait();
        let allocation = Box::new([0_u8; 4096]);
        black_box(&allocation);
        drop(allocation);
        worker_finished.wait();
    });

    // The worker is live and waiting before the measured interval begins. All synchronization
    // objects and the thread handle therefore exist outside the global and TLS marks.
    ready.wait();
    audit::warm_up();
    audit::reset();
    let global_mark = bench_alloc::counters();
    let current_thread_mark = bench_alloc::current_thread_counters();
    let thread_audit = audit::in_render_scope(|| {
        start.wait();
        finished.wait();
        audit::snapshot()
    });
    let global = bench_alloc::delta_since(global_mark);
    let current_thread = bench_alloc::current_thread_delta_since(current_thread_mark);
    worker.join().expect("foreign allocation probe worker");
    (global, current_thread, thread_audit)
}

fn assert_global_dominates_current_thread(diagnostic: PreparationAllocationDiagnostic) {
    assert!(diagnostic.global.allocations >= diagnostic.current_thread.allocations);
    assert!(diagnostic.global.deallocations >= diagnostic.current_thread.deallocations);
    assert!(diagnostic.global.reallocations >= diagnostic.current_thread.reallocations);
    assert!(diagnostic.global.requested_bytes >= diagnostic.current_thread.requested_bytes);
}

const REAL_SESSION: &str =
    include_str!("../../../fixtures/session/v1/compressor-dynamic-observation.json");

fn compile_caps() -> session::CompileCaps {
    session::CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn controller_config() -> ProtocolControllerConfig {
    ProtocolControllerConfig {
        maximum_transaction_edits: 0,
        maximum_response_diagnostics: 8,
        provider_features: ProviderFeatures::ALL,
    }
}

fn controller_queue_config() -> ProtocolQueueConfig {
    ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(2).unwrap(),
        control_command_bytes: NonZeroUsize::new(1_024).unwrap(),
        automation_batch_slots: NonZeroUsize::new(2).unwrap(),
        reliable_response_slots: NonZeroUsize::new(2).unwrap(),
        reliable_event_slots: NonZeroUsize::new(4).unwrap(),
        telemetry_slots: NonZeroUsize::new(2).unwrap(),
        per_block_automation_density: NonZeroUsize::new(256).unwrap(),
        quantum_frames: NonZeroUsize::new(128).unwrap(),
    }
}

fn controller_replay_config() -> ReplayCacheConfig {
    ReplayCacheConfig {
        entries: NonZeroUsize::new(16).unwrap(),
        bytes: NonZeroUsize::new(16 * 1024).unwrap(),
        max_response_bytes: 2_048,
    }
}

struct RealClock(AtomicU64);

impl PlanSampleSource for RealClock {
    fn next_absolute_sample(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }
}

struct RealControllerFixture {
    effects: effect_compiler::EffectPreparedSession,
    session: protocol::SessionStore,
    provider: SessionControlProvider,
    clock: Arc<RealClock>,
    handles: [ParameterHandle; 2],
}

fn real_controller_fixture() -> RealControllerFixture {
    let model = session::parse_session_json(REAL_SESSION).expect("real compressor fixture");
    let compiled = session::compile_session(&model, compile_caps()).expect("compiled fixture");
    let registry = effect_compiler::launch_native_effect_registry().expect("native registry");
    let effects = effect_compiler::prepare_native_session_effects(
        &compiled,
        &registry,
        effect_compiler::EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
    )
    .expect("prepared real compressor");
    assert_eq!(effects.entries.len(), 1);
    let catalog = SessionControlProvider::prepare_session(&effects.entries).expect("catalog");
    let clock = Arc::new(RealClock(AtomicU64::new(0)));
    let mut provider = SessionControlProvider::try_new(
        catalog,
        Arc::clone(&clock) as Arc<dyn PlanSampleSource>,
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
        2,
    )
    .expect("provider");
    let metadata = provider
        .parameter_metadata(ParameterMetadataRequest {
            after_handle: 0,
            limit: u16::MAX,
        })
        .expect("catalog metadata");
    let mut handles = [None, None];
    for descriptor in metadata.descriptors {
        if descriptor.track_id == "comp0"
            && descriptor.effect_id == "comp"
            && descriptor.parameter_id == 6
        {
            match descriptor.channel {
                ProtocolParameterChannel::Left => {
                    handles[0] = Some(ParameterHandle(descriptor.handle))
                }
                ProtocolParameterChannel::Right => {
                    handles[1] = Some(ParameterHandle(descriptor.handle))
                }
                ProtocolParameterChannel::Both => {}
            }
        }
    }
    let handles = [
        handles[0].expect("actual Left makeup handle"),
        handles[1].expect("actual Right makeup handle"),
    ];
    RealControllerFixture {
        effects,
        session: protocol::SessionStore::new(model, compile_caps()).expect("session store"),
        provider,
        clock,
        handles,
    }
}

fn encoded_controller_enqueue(
    revision: SessionRevision,
    request_id: u64,
    records: &[(ParameterHandle, u64, f32)],
) -> Vec<u8> {
    let records: Vec<_> = records
        .iter()
        .map(|&(handle, sample, value)| AutomationRecord {
            kind: AutomationKind::Point,
            handle,
            start: SampleTime(sample),
            end: SampleTime(sample),
            start_value: value,
            end_value: value,
        })
        .collect();
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; 4_096];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).unwrap(),
                expected_revision: ExpectedRevision::Exact(revision),
                payload: CommandPayload::AutomationEnqueue(AutomationEnqueue { records: &records }),
            },
            &mut bytes,
        )
        .expect("encoded BTLV automation enqueue");
    bytes.truncate(length);
    bytes
}

fn encoded_state_get(revision: SessionRevision, request_id: u64, handles: &[u32]) -> Vec<u8> {
    let request = ParameterStateRequest {
        handles: handles.to_vec(),
    };
    let mut bytes = vec![0_u8; 4_096];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).unwrap(),
                expected_revision: ExpectedRevision::Exact(revision),
                payload: CommandPayload::ParameterStateGet(&request),
            },
            &mut bytes,
        )
        .expect("encoded BTLV parameter state request");
    bytes.truncate(length);
    bytes
}

fn process_controller_frame<P: ControlProvider>(
    controller: &mut ControllerAutomationDelivery<P>,
    input: &[u8],
) -> Vec<u8> {
    let mut output = vec![0_u8; 2_048];
    let written = controller
        .process_command_frame_into(
            input,
            &mut DecodeScratch::new(&mut [0_u16; 64]),
            &mut output,
        )
        .expect("complete caller-buffer command frame");
    output.truncate(written);
    output
}

#[derive(Clone, Copy)]
enum ControllerIngress {
    CallerBuffer,
    B1b,
}

fn process_controller_ingress<P: ControlProvider>(
    controller: &mut ControllerAutomationDelivery<P>,
    input: &[u8],
    ingress: ControllerIngress,
) -> Vec<u8> {
    match ingress {
        ControllerIngress::CallerBuffer => process_controller_frame(controller, input),
        ControllerIngress::B1b => {
            controller
                .process_b1b_btlv(input, &mut DecodeScratch::new(&mut [0_u16; 64]))
                .expect("complete B1b command frame")
                .frame
        }
    }
}

fn response_status(frame: &[u8]) -> StatusCode {
    match ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("typed response frame")
    {
        DecodedTypedResponseFrame::Success { header, .. }
        | DecodedTypedResponseFrame::NonOk { header, .. } => header.status,
    }
}

fn decode_state_page(frame: &[u8]) -> ParameterStatePage {
    match ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("typed state response")
    {
        DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ParameterState(page),
            ..
        } => page,
        _ => panic!(
            "expected successful parameter state response: {:?}",
            response_status(frame)
        ),
    }
}

fn process_real(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
    spans: &[PreparedAutomationSpan],
) -> ProcessReport {
    let frames = left.len() as u32;
    effect.process(EffectProcessBlock::new(left, right, None, first, spans, frames).unwrap())
}

fn warm_real(effect: &mut dyn PreparedNativeEffect) {
    const FRAMES: usize = 1_152;
    let mut left = signal(FRAMES, 0x0532_1001, 0.73);
    let mut right = signal(FRAMES, 0x0532_1002, 0.61);
    for (block, (left, right)) in left.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
        process_real(effect, left, right, (block * 128) as u64, &[]);
    }
}
fn effect() -> Box<dyn PreparedNativeEffect> {
    let values: Vec<_> = compressor::COMPRESSOR_PARAMETERS
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            [ParameterChannel::Left, ParameterChannel::Right].map(move |channel| {
                InitialParameterValue {
                    parameter_index: i as u32,
                    channel,
                    value: p.default_value,
                }
            })
        })
        .collect();
    compressor::CompressorFactory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: Q as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: effect_contract::PortId::new("sidechain-in").unwrap(),
                    required: false,
                },
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 12,
                maximum_automation_spans_per_block: 256,
            },
        })
        .unwrap()
}
fn asymmetric_effect() -> Box<dyn PreparedNativeEffect> {
    let pairs = [
        (-31.0, -13.0),
        (3.0, 7.0),
        (4.0, 10.0),
        (8.0, 30.0),
        (80.0, 300.0),
        (-3.0, 4.0),
        (0.25, 0.75),
        (10.0, 15.0),
    ];
    let values: Vec<_> = pairs
        .iter()
        .enumerate()
        .flat_map(|(parameter, &(left, right))| {
            [
                (ParameterChannel::Left, left),
                (ParameterChannel::Right, right),
            ]
            .map(move |(channel, value)| InitialParameterValue {
                parameter_index: parameter as u32,
                channel,
                value,
            })
        })
        .collect();
    compressor::CompressorFactory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: Q as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: effect_contract::PortId::new("sidechain-in").unwrap(),
                    required: false,
                },
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 12,
                maximum_automation_spans_per_block: 256,
            },
        })
        .unwrap()
}
fn signal(frames: usize, seed: u32, scale: f32) -> Vec<f32> {
    let mut state = seed;
    (0..frames)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((state >> 8) as f32 / 8_388_608.0 - 1.0) * scale
        })
        .collect()
}
fn process(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
    spans: &[PreparedAutomationSpan],
) -> ProcessReport {
    effect.process(EffectProcessBlock::new(left, right, None, first, spans, Q as u32).unwrap())
}
fn add_report(total: &mut ProcessReport, report: ProcessReport) {
    total.sanitized_main_samples += report.sanitized_main_samples;
    total.sanitized_sidechain_samples += report.sanitized_sidechain_samples;
    total.invalid_spans += report.invalid_spans;
    total.nonfinite_left_blocks += report.nonfinite_left_blocks;
    total.nonfinite_right_blocks += report.nonfinite_right_blocks;
}
fn warm(effect: &mut dyn PreparedNativeEffect) {
    const FRAMES: usize = 1_152;
    let mut left = signal(FRAMES, 0x0528_1001, 0.73);
    let mut right = signal(FRAMES, 0x0528_1002, 0.61);
    for (block, (left, right)) in left.chunks_mut(Q).zip(right.chunks_mut(Q)).enumerate() {
        process(effect, left, right, (block * Q) as u64, &[]);
    }
}
fn point_span(at: u64, channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: 5,
        start_sample: at,
        end_sample: at,
        start_value: value,
        end_value: value,
    }
}
fn payload(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).unwrap(),
        )
        .unwrap();
    (left, right)
}
fn resident(effect: &dyn PreparedNativeEffect) -> ObservationSample {
    let mut result = ObservationSample::default();
    assert!(effect.observe_resident(0, &mut result));
    result
}
fn assert_pcm_bits(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (sample, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(actual.to_bits(), expected.to_bits(), "PCM sample {sample}");
    }
}
fn assert_state_bits(actual: PreparedParameterState, expected: PreparedParameterState) {
    assert_eq!(
        actual.current_value.to_bits(),
        expected.current_value.to_bits()
    );
    assert_eq!(
        actual.target_value.to_bits(),
        expected.target_value.to_bits()
    );
}
fn assert_resident_bits(actual: ObservationSample, expected: ObservationSample) {
    assert_eq!(actual.left.to_bits(), expected.left.to_bits());
    assert_eq!(actual.right.to_bits(), expected.right.to_bits());
}
struct FailingAccessEffect {
    inner: Box<dyn PreparedNativeEffect>,
    fail_apply_channel: Option<ParameterChannel>,
    fail_read_after_process: bool,
    processed_frames: u64,
    apply_calls: u64,
    read_calls: core::cell::Cell<u64>,
}
impl FailingAccessEffect {
    fn apply_failure() -> Self {
        Self {
            inner: asymmetric_effect(),
            fail_apply_channel: Some(ParameterChannel::Right),
            fail_read_after_process: false,
            processed_frames: 0,
            apply_calls: 0,
            read_calls: core::cell::Cell::new(0),
        }
    }
    fn read_failure() -> Self {
        Self {
            inner: asymmetric_effect(),
            fail_apply_channel: None,
            fail_read_after_process: true,
            processed_frames: 0,
            apply_calls: 0,
            read_calls: core::cell::Cell::new(0),
        }
    }
}
impl PreparedNativeEffect for FailingAccessEffect {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.inner.metadata()
    }
    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        self.processed_frames += block.frames() as u64;
        self.inner.process(block)
    }
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        self.inner.observe_resident(tap_index, out)
    }
    fn apply_parameter_point(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), ParameterAccessError> {
        self.apply_calls += 1;
        if self.processed_frames != 0 && self.fail_apply_channel == Some(channel) {
            Err(ParameterAccessError::Unsupported)
        } else {
            self.inner
                .apply_parameter_point(parameter_index, channel, value)
        }
    }
    fn parameter_state(
        &self,
        parameter_index: u32,
        channel: ParameterChannel,
    ) -> Result<PreparedParameterState, ParameterAccessError> {
        self.read_calls.set(self.read_calls.get() + 1);
        if self.processed_frames != 0 && self.fail_read_after_process {
            Err(ParameterAccessError::Unsupported)
        } else {
            self.inner.parameter_state(parameter_index, channel)
        }
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.snapshot_state_payload(output)
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.restore_state_payload(version, input)
    }
    fn channel_symmetry(&self) -> bool {
        self.inner.channel_symmetry()
    }
}
fn pcm_case(records: &[AutomationRecord], splits: &[(usize, &[PreparedAutomationSpan])]) {
    let _fp = lane::CanonicalFpEnv::enter();
    let (mut endpoint_effect, mut reference_effect, mut no_event_effect) = (
        asymmetric_effect(),
        asymmetric_effect(),
        asymmetric_effect(),
    );
    warm(&mut *endpoint_effect);
    warm(&mut *reference_effect);
    warm(&mut *no_event_effect);
    assert_eq!(payload(&*endpoint_effect), payload(&*reference_effect));
    assert_eq!(payload(&*endpoint_effect), payload(&*no_event_effect));

    let (mut control, render, _) =
        prepare_scalar_point_endpoint(&mut *endpoint_effect, REV, H, config(), 1).unwrap();
    control.try_admit(SampleTime(0), batch(1, records)).unwrap();
    let ticket = match control.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        _ => panic!(),
    };
    let mut render = render.start().unwrap_or_else(|_| panic!());
    let mut actual_left = signal(Q, 0x0528_2001, 0.69);
    let mut actual_right = signal(Q, 0x0528_2002, 0.57);
    let mut expected_left = actual_left.clone();
    let mut expected_right = actual_right.clone();
    let mut unchanged_left = actual_left.clone();
    let mut unchanged_right = actual_right.clone();
    let endpoint_report = render
        .render(&mut actual_left, &mut actual_right, SampleTime(0))
        .unwrap();
    let snapshot = render.snapshot().unwrap();
    assert_eq!(snapshot.applied, records.len() as u64);
    assert_eq!(snapshot.next_sample, SampleTime(Q as u64));
    assert_eq!(snapshot.observed_sample, SampleTime(Q as u64));
    for (index, handle) in H.into_iter().enumerate() {
        assert_eq!(
            snapshot.last_application[index],
            records
                .iter()
                .rev()
                .find(|record| record.handle == handle)
                .map(|record| record.start)
        );
    }

    let mut reference_report = ProcessReport::default();
    let mut begin = 0;
    for &(end, spans) in splits {
        add_report(
            &mut reference_report,
            process(
                &mut *reference_effect,
                &mut expected_left[begin..end],
                &mut expected_right[begin..end],
                begin as u64,
                spans,
            ),
        );
        begin = end;
    }
    assert_eq!(begin, Q);
    let no_event_report = process(
        &mut *no_event_effect,
        &mut unchanged_left,
        &mut unchanged_right,
        0,
        &[],
    );

    assert_eq!(endpoint_report.native, reference_report);
    assert_eq!(endpoint_report.process_invocations as usize, splits.len());
    assert_pcm_bits(&actual_left, &expected_left);
    assert_pcm_bits(&actual_right, &expected_right);
    assert!(
        actual_left
            .iter()
            .zip(&unchanged_left)
            .chain(actual_right.iter().zip(&unchanged_right))
            .any(|(&actual, &unchanged)| actual.to_bits() != unchanged.to_bits()),
        "warmed Point render matched the no-event PCM after compressor lookahead"
    );
    assert_eq!(no_event_report, ProcessReport::default());
    for (index, channel) in [ParameterChannel::Left, ParameterChannel::Right]
        .into_iter()
        .enumerate()
    {
        let expected = reference_effect.parameter_state(5, channel).unwrap();
        assert_state_bits(snapshot.state[index], expected);
    }
    assert_eq!(
        control.collect_terminal(ticket).unwrap().applied_prefix,
        records.len() as u16
    );
    drop(render.stop());
    assert_resident_bits(resident(&*endpoint_effect), resident(&*reference_effect));
    assert_eq!(payload(&*endpoint_effect), payload(&*reference_effect));
}
fn record(handle: ParameterHandle, time: u64, value: f32) -> AutomationRecord {
    AutomationRecord {
        kind: AutomationKind::Point,
        handle,
        start: SampleTime(time),
        end: SampleTime(time),
        start_value: value,
        end_value: value,
    }
}
fn batch(id: u64, records: &[AutomationRecord]) -> AutomationBatchSlot {
    AutomationBatchSlot::new(REV, RequestId::new(id).unwrap(), records).unwrap()
}
#[test]
fn points_slice_real_pcm_and_readback() {
    let left_3 = [point_span(3, ParameterChannel::Left, 6.0)];
    let right_7 = [point_span(7, ParameterChannel::Right, -6.0)];
    pcm_case(
        &[record(H[0], 3, 6.0), record(H[1], 7, -6.0)],
        &[(3, &[]), (7, &left_3), (Q, &right_7)],
    );

    let simultaneous = [
        point_span(7, ParameterChannel::Left, 9.0),
        point_span(7, ParameterChannel::Right, -9.0),
    ];
    pcm_case(
        &[record(H[0], 7, 9.0), record(H[1], 7, -9.0)],
        &[(7, &[]), (Q, &simultaneous)],
    );
}

#[test]
fn claim_state_is_truthful_until_terminal_collection() {
    let mut fx = effect();
    let (mut c, mut r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    let initial = r.snapshot().unwrap();
    c.try_admit(
        SampleTime(0),
        batch(
            2,
            &[
                record(H[0], 16, 4.0),
                record(H[1], 19, 5.0),
                record(H[0], 32, 6.0),
            ],
        ),
    )
    .unwrap();
    assert_eq!((c.resident_automation(), c.outstanding()), (1, 1));
    assert_eq!(r.snapshot().unwrap(), initial);
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(t) => t,
        _ => panic!(),
    };
    assert_eq!((c.resident_automation(), c.outstanding()), (0, 1));
    assert_eq!(r.snapshot().unwrap().pending, None);
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let s = r.snapshot().unwrap();
    assert_eq!(s.observed_sample, SampleTime(16));
    assert_eq!(s.pending.unwrap().1, 0);
    assert_eq!(s.pending.unwrap().3, Some(SampleTime(16)));
    assert_state_bits(s.state[0], initial.state[0]);
    assert_state_bits(s.state[1], initial.state[1]);
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::Empty));

    l.fill(0.2);
    x.fill(0.3);
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    let partial = r.snapshot().unwrap();
    assert_eq!(partial.pending.unwrap().1, 2);
    assert_eq!(partial.pending.unwrap().3, Some(SampleTime(32)));
    assert_eq!(partial.state[0].target_value.to_bits(), 4.0_f32.to_bits());
    assert_eq!(partial.state[1].target_value.to_bits(), 5.0_f32.to_bits());
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::Empty));

    l.fill(0.2);
    x.fill(0.3);
    r.render(&mut l, &mut x, SampleTime(32)).unwrap();
    let complete = r.snapshot().unwrap();
    assert_eq!(complete.pending, None);
    assert_eq!(complete.state[0].target_value.to_bits(), 6.0_f32.to_bits());
    assert_eq!(c.outstanding(), 1);
    assert_eq!(c.collect_terminal(ticket).unwrap().applied_prefix, 3);
    assert_eq!(c.outstanding(), 0);

    l.fill(0.2);
    x.fill(0.3);
    let report = r.render(&mut l, &mut x, SampleTime(48)).unwrap();
    assert_eq!(report.process_invocations, 1);
    let empty = r.snapshot().unwrap();
    assert_eq!(empty.pending, None);
    assert_eq!(empty.applied, 3);
}

#[test]
fn late_points_apply_in_order_and_second_ticket_waits() {
    let _fp = lane::CanonicalFpEnv::enter();
    let mut fx = effect();
    let mut reference = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(3, &[record(H[0], 1, 1.0), record(H[0], 2, 2.0)]),
    )
    .unwrap();
    c.try_admit(SampleTime(0), batch(4, &[record(H[1], 3, 3.0)]))
        .unwrap();
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    let (mut expected_l, mut expected_r) = (l, x);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    process(&mut *reference, &mut expected_l, &mut expected_r, 0, &[]);
    assert_pcm_bits(&l, &expected_l);
    assert_pcm_bits(&x, &expected_r);
    c.try_handoff_next().unwrap();
    c.try_handoff_next().unwrap();
    l.fill(0.21);
    x.fill(0.31);
    expected_l = l;
    expected_r = x;
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    reference
        .apply_parameter_point(5, ParameterChannel::Left, 1.0)
        .unwrap();
    reference
        .apply_parameter_point(5, ParameterChannel::Left, 2.0)
        .unwrap();
    process(&mut *reference, &mut expected_l, &mut expected_r, 16, &[]);
    assert_pcm_bits(&l, &expected_l);
    assert_pcm_bits(&x, &expected_r);
    let s = r.snapshot().unwrap();
    assert_eq!((s.applied, s.late), (2, 2));
    assert_eq!(s.last_application[0], Some(SampleTime(16)));
    l.fill(0.22);
    x.fill(0.32);
    expected_l = l;
    expected_r = x;
    r.render(&mut l, &mut x, SampleTime(32)).unwrap();
    reference
        .apply_parameter_point(5, ParameterChannel::Right, 3.0)
        .unwrap();
    process(&mut *reference, &mut expected_l, &mut expected_r, 32, &[]);
    assert_pcm_bits(&l, &expected_l);
    assert_pcm_bits(&x, &expected_r);
    let s = r.snapshot().unwrap();
    assert_eq!((s.applied, s.late), (3, 3));
    assert_eq!(s.last_application[1], Some(SampleTime(32)));
    l.fill(0.23);
    x.fill(0.33);
    expected_l = l;
    expected_r = x;
    r.render(&mut l, &mut x, SampleTime(48)).unwrap();
    process(&mut *reference, &mut expected_l, &mut expected_r, 48, &[]);
    assert_pcm_bits(&l, &expected_l);
    assert_pcm_bits(&x, &expected_r);
    assert_eq!(
        (r.snapshot().unwrap().applied, r.snapshot().unwrap().late),
        (3, 3)
    );
    let endpoint_state = r.snapshot().unwrap().state;
    for (index, channel) in [ParameterChannel::Left, ParameterChannel::Right]
        .into_iter()
        .enumerate()
    {
        assert_state_bits(
            endpoint_state[index],
            reference.parameter_state(5, channel).unwrap(),
        );
    }
    drop(r.stop());
    assert_eq!(payload(&*fx), payload(&*reference));
}

#[test]
fn real_cancellation_preserves_applied_prefix() {
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(5, &[record(H[0], 3, 2.0), record(H[1], 30, 3.0)]),
    )
    .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.2; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let partial = r.snapshot().unwrap();
    assert_eq!(partial.pending.unwrap().1, 1);
    assert_eq!(partial.pending.unwrap().3, Some(SampleTime(30)));
    let right_before_cancel = partial.state[1];
    let token = c
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(done.effective_sample, SampleTime(16));
    assert_eq!(
        (
            done.applied_records,
            done.canceled_records,
            done.canceled_events
        ),
        (1, 1, 1)
    );
    assert_eq!(c.outstanding(), 0);
    assert_eq!(r.snapshot().unwrap().pending, None);
    let event = c.try_dequeue_event().unwrap();
    assert!(matches!(
        event.payload,
        ReliablePayload::AutomationCanceled {
            origin_request_id,
            canceled_records: 1,
            effective_sample: Some(SampleTime(16)),
            ..
        } if origin_request_id == RequestId::new(5).unwrap()
    ));
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::StaleTicket));
    l.fill(0.2);
    x.fill(0.2);
    r.render(&mut l, &mut x, SampleTime(32)).unwrap();
    let after_barrier = r.snapshot().unwrap();
    assert_eq!(after_barrier.applied, 1);
    assert_state_bits(after_barrier.state[1], right_before_cancel);
    assert_eq!(
        r.cancel_boundary(SampleTime(48)),
        Err(ScalarPointCancelBoundaryError::NotFaulted)
    );
    drop(r.stop());

    let mut future_fx = effect();
    let (mut future_c, future_r, _) =
        prepare_scalar_point_endpoint(&mut *future_fx, REV, H, config(), 20).unwrap();
    future_c
        .try_admit(SampleTime(0), batch(20, &[record(H[0], 30, 8.0)]))
        .unwrap();
    future_c.try_handoff_next().unwrap();
    let token = future_c
        .begin_cancel(AutomationCancellationReason::TransportLocate)
        .unwrap();
    let mut future_r = future_r.start().unwrap_or_else(|_| panic!());
    let future_initial = future_r.snapshot().unwrap();
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    future_r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let done = future_c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(
        (
            done.effective_sample,
            done.applied_records,
            done.canceled_records
        ),
        (SampleTime(0), 0, 1)
    );
    assert_eq!(future_r.snapshot().unwrap().pending, None);
    let future_after = future_r.snapshot().unwrap();
    assert_eq!(future_after.applied, 0);
    assert_state_bits(future_after.state[0], future_initial.state[0]);
    assert_state_bits(future_after.state[1], future_initial.state[1]);
    assert!(future_c.try_dequeue_event().is_ok());
    assert_eq!(future_c.outstanding(), 0);
    drop(future_r.stop());

    let mut unsupported_fx = effect();
    let (mut unsupported_c, unsupported_r, _) =
        prepare_scalar_point_endpoint(&mut *unsupported_fx, REV, H, config(), 30).unwrap();
    let step = AutomationRecord {
        kind: AutomationKind::Step,
        handle: H[1],
        start: SampleTime(4),
        end: SampleTime(8),
        start_value: -2.0,
        end_value: -2.0,
    };
    let linear = AutomationRecord {
        kind: AutomationKind::Linear,
        handle: H[0],
        start: SampleTime(9),
        end: SampleTime(12),
        start_value: 2.0,
        end_value: 4.0,
    };
    unsupported_c
        .try_admit(
            SampleTime(0),
            batch(30, &[record(H[0], 3, 2.0), step, linear]),
        )
        .unwrap();
    unsupported_c
        .try_admit(SampleTime(0), batch(31, &[record(H[1], 13, 6.0)]))
        .unwrap();
    assert_eq!(
        unsupported_c.try_handoff_next().unwrap(),
        HandoffResult::PendingUnsupported
    );
    assert_eq!(
        unsupported_c.try_handoff_next().unwrap(),
        HandoffResult::PendingUnsupported
    );
    assert_eq!(unsupported_c.outstanding(), 2);
    let token = unsupported_c
        .begin_cancel(AutomationCancellationReason::ExplicitReconfiguration)
        .unwrap();
    let mut unsupported_r = unsupported_r.start().unwrap_or_else(|_| panic!());
    let initial = unsupported_r.snapshot().unwrap();
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    unsupported_r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let done = unsupported_c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(
        (
            done.effective_sample,
            done.applied_records,
            done.canceled_records
        ),
        (SampleTime(0), 0, 4)
    );
    assert_eq!(done.canceled_events, 2);
    let after = unsupported_r.snapshot().unwrap();
    assert_eq!(after.pending, None);
    assert_eq!(after.applied, 0);
    assert_state_bits(after.state[0], initial.state[0]);
    assert_state_bits(after.state[1], initial.state[1]);
    assert_eq!(unsupported_c.outstanding(), 0);
    for expected in [
        (RequestId::new(30).unwrap(), 3),
        (RequestId::new(31).unwrap(), 1),
    ] {
        let event = unsupported_c.try_dequeue_event().unwrap();
        assert!(matches!(
            event.payload,
            ReliablePayload::AutomationCanceled {
                origin_request_id,
                canceled_records,
                effective_sample: Some(SampleTime(0)),
                ..
            } if (origin_request_id, canceled_records) == expected
        ));
    }
    drop(unsupported_r.stop());

    let mut full_cfg = config();
    full_cfg.reliable_event_slots = NonZeroUsize::new(4).unwrap();
    let mut reliable_fx = effect();
    let (mut reliable_c, reliable_r, _) =
        prepare_scalar_point_endpoint(&mut *reliable_fx, REV, H, full_cfg, 40).unwrap();
    let mut reliable_r = reliable_r.start().unwrap_or_else(|_| panic!());
    for cycle in 0..2_u64 {
        let first = cycle * Q as u64;
        reliable_c
            .try_admit(
                SampleTime(first),
                batch(40 + cycle * 2, &[record(H[0], first + 30, 4.0)]),
            )
            .unwrap();
        reliable_c
            .try_admit(
                SampleTime(first),
                batch(41 + cycle * 2, &[record(H[1], first + 31, -4.0)]),
            )
            .unwrap();
        let token = reliable_c
            .begin_cancel(AutomationCancellationReason::EndpointShutdown)
            .unwrap();
        let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
        reliable_r
            .render(&mut l, &mut x, SampleTime(first))
            .unwrap();
        let done = reliable_c.poll_cancel_boundary(token).unwrap().unwrap();
        assert_eq!((done.canceled_events, done.canceled_records), (2, 2));
    }
    reliable_c
        .try_admit(SampleTime(32), batch(44, &[record(H[0], 62, 4.0)]))
        .unwrap();
    let before = (
        reliable_c.outstanding(),
        reliable_c.resident_automation(),
        reliable_c.automation_status(),
    );
    assert!(matches!(
        reliable_c.begin_cancel(AutomationCancellationReason::EndpointShutdown),
        Err(DeliveryError::ReliableFull(_))
    ));
    assert_eq!(
        (
            reliable_c.outstanding(),
            reliable_c.resident_automation(),
            reliable_c.automation_status(),
        ),
        before
    );
    for _ in 0..4 {
        reliable_c.try_dequeue_event().unwrap();
    }
    let token = reliable_c
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    reliable_r.render(&mut l, &mut x, SampleTime(32)).unwrap();
    assert_eq!(
        reliable_c
            .poll_cancel_boundary(token)
            .unwrap()
            .unwrap()
            .canceled_records,
        1
    );
    drop(reliable_r.stop());

    let mut retained_fx = effect();
    let (mut retained_c, retained_r, _) =
        prepare_scalar_point_endpoint(&mut *retained_fx, REV, H, config(), 60).unwrap();
    retained_c
        .try_admit(SampleTime(0), batch(60, &[record(H[0], 0, 2.0)]))
        .unwrap();
    retained_c
        .try_admit(SampleTime(0), batch(61, &[record(H[1], 16, 3.0)]))
        .unwrap();
    let first = match retained_c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    assert!(matches!(
        retained_c.try_admit(SampleTime(0), batch(62, &[record(H[0], 17, 4.0)])),
        Err(ScalarPointAdmissionError::Service(
            AutomationEnqueueError::Full { .. }
        ))
    ));
    let mut retained_r = retained_r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.3; Q]);
    retained_r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    assert!(matches!(
        retained_c.try_admit(SampleTime(0), batch(63, &[record(H[0], 18, 4.0)])),
        Err(ScalarPointAdmissionError::Service(
            AutomationEnqueueError::Full { .. }
        ))
    ));
    retained_c.collect_terminal(first).unwrap();
    retained_c
        .try_admit(SampleTime(0), batch(64, &[record(H[0], 19, 4.0)]))
        .unwrap();
    assert_eq!(retained_c.outstanding(), 2);
}

#[test]
fn malformed_admission_and_render_envelopes_are_noops() {
    let _fp = lane::CanonicalFpEnv::enter();
    let mut fx = effect();
    let (mut c, mut r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    let pristine = r.snapshot().unwrap();
    {
        let mut reject =
            |candidate: AutomationBatchSlot,
             expected: fn(Result<(), ScalarPointAdmissionError>) -> bool| {
                assert!(expected(c.try_admit(SampleTime(10), candidate)));
                assert_eq!(c.outstanding(), 0);
                assert_eq!(c.resident_automation(), 0);
                assert_eq!(r.snapshot().unwrap(), pristine);
            };

        let mut empty = batch(60, &[record(H[0], 10, 1.0)]);
        empty.len = 0;
        reject(empty, |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::InvalidBatch {
                    error: AutomationBatchError::EmptyBatch,
                    ..
                })
            )
        });
        let mut too_long = batch(61, &[record(H[0], 10, 1.0)]);
        too_long.len = 257;
        reject(too_long, |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::InvalidBatch {
                    error: AutomationBatchError::TooManyRecords,
                    ..
                })
            )
        });
        let mut unequal_point = batch(62, &[record(H[0], 10, 1.0)]);
        unequal_point.records[0].end_value = 2.0;
        reject(unequal_point, |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::InvalidBatch {
                    error: AutomationBatchError::InvalidPoint,
                    ..
                })
            )
        });
        let mut out_of_order = batch(63, &[record(H[0], 10, 1.0), record(H[1], 11, 2.0)]);
        out_of_order.records.swap(0, 1);
        reject(out_of_order, |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::InvalidBatch {
                    error: AutomationBatchError::OutOfOrder,
                    ..
                })
            )
        });
        let wrong = AutomationBatchSlot::new(
            SessionRevision(8),
            RequestId::new(6).unwrap(),
            &[record(H[0], 10, 1.0)],
        )
        .unwrap();
        reject(wrong, |result| {
            matches!(result, Err(ScalarPointAdmissionError::WrongRevision { .. }))
        });
        reject(
            batch(64, &[record(ParameterHandle(99), 10, 1.0)]),
            |result| {
                matches!(
                    result,
                    Err(ScalarPointAdmissionError::UnknownBinding { .. })
                )
            },
        );
        reject(batch(65, &[record(H[0], 10, 24.5)]), |result| {
            matches!(result, Err(ScalarPointAdmissionError::InvalidValue { .. }))
        });
        let mut invalid_last = batch(66, &[record(H[0], 10, 1.0), record(H[1], 11, 2.0)]);
        invalid_last.records[1].start_value = f32::NAN;
        invalid_last.records[1].end_value = f32::NAN;
        reject(invalid_last, |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::InvalidBatch {
                    error: AutomationBatchError::NonFiniteValue,
                    ..
                })
            )
        });
        reject(batch(67, &[record(H[0], 9, 1.0)]), |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::Service(
                    AutomationEnqueueError::Invalid {
                        error: AutomationBatchError::TimeInPast,
                        ..
                    }
                ))
            )
        });
    }

    let mut r = r.start().unwrap_or_else(|_| panic!());
    let before = r.snapshot().unwrap();
    let (mut short, mut full) = ([1.0; Q - 1], [1.0; Q]);
    let short_before = short;
    let full_before = full;
    assert_eq!(
        r.render(&mut short, &mut full, SampleTime(0)),
        Err(ScalarPointRenderError::InvalidShape)
    );
    assert_eq!(short, short_before);
    assert_eq!(full, full_before);
    let (mut left, mut right) = ([0.31; Q], [0.27; Q]);
    let input = (left, right);
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(1)),
        Err(ScalarPointRenderError::DiscontinuousTime)
    );
    assert_eq!((left, right), input);
    assert_eq!(r.snapshot().unwrap(), before);
    assert_eq!(
        r.cancel_boundary(SampleTime(0)),
        Err(ScalarPointCancelBoundaryError::NotFaulted)
    );
    drop(r.stop());
    let after_rejections = payload(&*fx);
    let fresh = effect();
    assert_eq!(after_rejections, payload(&*fresh));

    let mut failing = FailingAccessEffect::apply_failure();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut failing, REV, H, config(), 70).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(70, &[record(H[0], 3, 7.0), record(H[1], 7, -7.0)]),
    )
    .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut left, mut right) = ([0.42; Q], [0.38; Q]);
    let fault = ScalarPointFault {
        cause: ScalarPointFaultCause::Native {
            operation: ScalarPointNativeOperation::Apply,
            channel: ParameterChannel::Right,
            error: ParameterAccessError::Unsupported,
        },
        at_sample: SampleTime(7),
        native_processed_frames: 7,
        progress: Some(ScalarPointFaultProgress {
            ticket,
            record_count: 2,
            native_applied_prefix: 1,
            delivery_applied_prefix: 1,
        }),
    };
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(0)),
        Err(ScalarPointRenderError::Fault(fault))
    );
    assert!(
        left.iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0)
    );
    assert_eq!(r.fault(), Some(fault));
    assert_eq!(r.snapshot(), Err(fault));
    let (mut invalid_left, mut invalid_right) = ([0.6; Q - 1], [0.5; Q]);
    let invalid_before = (invalid_left, invalid_right);
    assert_eq!(
        r.render(&mut invalid_left, &mut invalid_right, SampleTime(16)),
        Err(ScalarPointRenderError::InvalidShape)
    );
    assert_eq!((invalid_left, invalid_right), invalid_before);
    let (mut silent_left, mut silent_right) = ([0.6; Q], [0.5; Q]);
    assert_eq!(
        r.render(&mut silent_left, &mut silent_right, SampleTime(16)),
        Err(ScalarPointRenderError::Fault(fault))
    );
    assert!(
        silent_left
            .iter()
            .chain(&silent_right)
            .all(|sample| sample.to_bits() == 0)
    );
    assert_eq!(
        r.cancel_boundary(SampleTime(17)),
        Err(ScalarPointCancelBoundaryError::DiscontinuousTime)
    );
    let token = c
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    assert!(c.poll_cancel_boundary(token).unwrap().is_none());
    r.cancel_boundary(SampleTime(16)).unwrap();
    assert_eq!(r.fault(), Some(fault));
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(done.effective_sample, SampleTime(16));
    assert_eq!(
        (
            done.applied_records,
            done.canceled_records,
            done.canceled_events
        ),
        (1, 1, 1)
    );
    assert_eq!(c.outstanding(), 0);
    let event = c.try_dequeue_event().unwrap();
    assert_eq!(event.revision, REV);
    assert!(matches!(
        event.payload,
        ReliablePayload::AutomationCanceled {
            origin_request_id,
            canceled_records: 1,
            reason: AutomationCancellationReason::EndpointShutdown,
            effective_sample: Some(SampleTime(16)),
            ..
        } if origin_request_id == RequestId::new(70).unwrap()
    ));
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::StaleTicket));
    assert_eq!(r.snapshot(), Err(fault));
    drop(r.stop());
    assert_eq!((failing.processed_frames, failing.apply_calls), (7, 2));

    let mut read_failing = FailingAccessEffect::read_failure();
    let (mut c, r, _) =
        prepare_scalar_point_endpoint(&mut read_failing, REV, H, config(), 80).unwrap();
    c.try_admit(SampleTime(0), batch(80, &[record(H[0], 30, 8.0)]))
        .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut left, mut right) = ([0.4; Q], [0.3; Q]);
    r.render(&mut left, &mut right, SampleTime(0)).unwrap();
    let read_fault = ScalarPointFault {
        cause: ScalarPointFaultCause::Native {
            operation: ScalarPointNativeOperation::Read,
            channel: ParameterChannel::Left,
            error: ParameterAccessError::Unsupported,
        },
        at_sample: SampleTime(16),
        native_processed_frames: 0,
        progress: Some(ScalarPointFaultProgress {
            ticket,
            record_count: 1,
            native_applied_prefix: 0,
            delivery_applied_prefix: 0,
        }),
    };
    assert_eq!(r.snapshot(), Err(read_fault));
    assert_eq!(r.snapshot(), Err(read_fault));
    let (mut left, mut right) = ([0.4; Q], [0.3; Q]);
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(16)),
        Err(ScalarPointRenderError::Fault(read_fault))
    );
    assert!(
        left.iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0)
    );
    let token = c
        .begin_cancel(AutomationCancellationReason::ProviderUnavailable)
        .unwrap();
    r.cancel_boundary(SampleTime(16)).unwrap();
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!((done.applied_records, done.canceled_records), (0, 1));
    assert_eq!(done.effective_sample, SampleTime(16));
    assert_eq!(r.fault(), Some(read_fault));
    drop(r.stop());
    assert_eq!(read_failing.processed_frames, Q as u64);
    assert_eq!(read_failing.apply_calls, 0);
    assert_eq!(read_failing.read_calls.get(), 3);
}

#[test]
fn preparation_resources_and_success_path_are_bounded() {
    const CHILD: &str = "MISO_ENGINE_SCALAR_POINT_GATE6_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg("preparation_resources_and_success_path_are_bounded")
            .arg("--test-threads=1")
            .arg("--nocapture")
            .env(CHILD, "1")
            .status()
            .expect("isolated scalar Point resource child");
        assert!(
            status.success(),
            "isolated scalar Point resource child failed"
        );
        return;
    }

    use bench_support::alloc as bench_alloc;
    use engine::realtime::audit;
    use std::hint::black_box;

    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let liveness_mark = bench_alloc::counters();
    let probe = black_box(vec![0_u8; 4_096]);
    drop(probe);
    let liveness = bench_alloc::delta_since(liveness_mark);
    assert!(liveness.allocations > 0, "allocation counter is not live");
    assert!(liveness.deallocations > 0, "free counter is not live");
    audit::warm_up();
    audit::reset();
    audit::in_render_scope(|| {
        let probe = black_box(vec![0_u8; 4_096]);
        black_box(&probe);
        drop(probe);
    });
    let audit_liveness = audit::snapshot();
    assert!(
        audit_liveness.allocations > 0,
        "render-audit allocation counter is not live"
    );
    assert!(
        audit_liveness.deallocations > 0,
        "render-audit deallocation counter is not live"
    );
    audit::reset();

    let cfg = config();
    let expected_delivery = PreparedAutomationDelivery::resource_report_for_config(cfg).unwrap();
    let (direct_owners, direct_diagnostic) =
        measure_preparation(|| PreparedAutomationDelivery::prepare(cfg, 1).unwrap());

    let mut fx = asymmetric_effect();
    warm(&mut *fx);
    let ((mut control, render, resources), endpoint_diagnostic) =
        measure_preparation(|| prepare_scalar_point_endpoint(&mut *fx, REV, H, cfg, 1).unwrap());
    let (foreign_global, foreign_current_thread, foreign_thread_audit) =
        foreign_thread_allocation_probe();
    eprintln!(
        "scalar Point preparation diagnostics: direct global={direct_global:?} current={direct_current:?} thread={direct_thread:?}; wrapped global={wrapped_global:?} current={wrapped_current:?} thread={wrapped_thread:?}; foreign global={foreign_global:?} current={foreign_current:?} thread={foreign_thread_audit:?}",
        direct_global = direct_diagnostic.global,
        direct_current = direct_diagnostic.current_thread,
        direct_thread = direct_diagnostic.thread_audit,
        wrapped_global = endpoint_diagnostic.global,
        wrapped_current = endpoint_diagnostic.current_thread,
        wrapped_thread = endpoint_diagnostic.thread_audit,
        foreign_current = foreign_current_thread,
    );
    assert!(
        foreign_global.allocations > 0,
        "foreign allocation did not move process-wide allocation counter"
    );
    assert!(
        foreign_global.deallocations > 0,
        "foreign allocation did not move process-wide free counter"
    );
    assert_eq!(
        foreign_thread_audit.allocations, 0,
        "foreign allocation moved measured-thread allocation audit"
    );
    assert_eq!(
        foreign_thread_audit.deallocations, 0,
        "foreign free moved measured-thread deallocation audit"
    );
    assert_eq!(
        foreign_current_thread,
        bench_support::alloc::Counters::default(),
        "foreign allocation moved measured-thread allocator counters"
    );
    assert_eq!(
        endpoint_diagnostic.current_thread,
        direct_diagnostic.current_thread
    );
    assert_eq!(direct_diagnostic.current_thread.reallocations, 0);
    assert_eq!(direct_diagnostic.current_thread.deallocations, 0);
    assert_global_dominates_current_thread(direct_diagnostic);
    assert_global_dominates_current_thread(endpoint_diagnostic);
    assert_eq!(resources.delivery, expected_delivery);
    assert!(resources.delivery.retained_payload_bytes > 0);
    assert!(resources.delivery.largest_allocation_bytes > 0);
    assert_eq!(resources.control_size, core::mem::size_of_val(&control));
    assert_eq!(resources.render_size, core::mem::size_of_val(&render));
    black_box(&direct_owners);

    let mut render = render.start().unwrap_or_else(|_| panic!());
    let mut left = [0.41_f32; Q];
    let mut right = [-0.37_f32; Q];
    let targets = [(H[0], 5.0), (H[1], -5.0), (H[0], 9.0), (H[1], -9.0)];
    let mut applied = [0_u64; 4];
    let mut nonzero = [false; 4];
    audit::warm_up();
    for (block, &(handle, target)) in targets.iter().enumerate() {
        let first = (block * Q) as u64;
        control
            .try_admit(
                SampleTime(first),
                batch(90 + block as u64, &[record(handle, first + 3, target)]),
            )
            .unwrap();
        let ticket = match control.try_handoff_next().unwrap() {
            HandoffResult::HandedOff(ticket) => ticket,
            other => panic!("{other:?}"),
        };
        left.fill(0.41);
        right.fill(-0.37);
        audit::reset();
        let (report, snapshot) = audit::in_render_scope(|| {
            let report = render
                .render(&mut left, &mut right, SampleTime(first))
                .unwrap();
            let snapshot = render.snapshot().unwrap();
            (report, snapshot)
        });
        let observed = audit::snapshot();
        assert_eq!(observed.allocations, 0, "Point render/read allocated");
        assert_eq!(observed.deallocations, 0, "Point render/read freed");
        assert_eq!(
            observed.total(),
            0,
            "Point render/read used realtime-forbidden work"
        );
        assert_eq!(report.process_invocations, 2);
        applied[block] = snapshot.applied;
        nonzero[block] = left
            .iter()
            .chain(&right)
            .any(|sample| sample.to_bits() & 0x7fff_ffff != 0);
        assert_eq!(control.collect_terminal(ticket).unwrap().applied_prefix, 1);
    }
    assert_eq!(applied, [1, 2, 3, 4]);
    assert!(
        nonzero.into_iter().all(|value| value),
        "real compressor PCM stayed silent"
    );
    drop(render.stop());
    drop(control);
    drop(fx);
    drop(direct_owners);

    let mut failing = FailingAccessEffect::apply_failure();
    warm(&mut *failing.inner);
    let (mut control, render, _) =
        prepare_scalar_point_endpoint(&mut failing, REV, H, cfg, 100).unwrap();
    control
        .try_admit(
            SampleTime(0),
            batch(100, &[record(H[0], 3, 6.0), record(H[1], 7, -6.0)]),
        )
        .unwrap();
    let ticket = match control.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut render = render.start().unwrap_or_else(|_| panic!());
    let (mut left, mut right) = ([0.43_f32; Q], [-0.39_f32; Q]);
    audit::reset();
    let fault =
        audit::in_render_scope(
            || match render.render(&mut left, &mut right, SampleTime(0)) {
                Err(ScalarPointRenderError::Fault(fault)) => fault,
                other => panic!("unexpected fault render {other:?}"),
            },
        );
    let fault_render_audit = audit::snapshot();
    assert_eq!(fault_render_audit.allocations, 0, "fault render allocated");
    assert_eq!(fault_render_audit.deallocations, 0, "fault render freed");
    assert_eq!(
        fault_render_audit.total(),
        0,
        "fault render used forbidden work"
    );
    assert_eq!(fault.native_processed_frames, 7);
    assert_eq!(
        fault.progress,
        Some(ScalarPointFaultProgress {
            ticket,
            record_count: 2,
            native_applied_prefix: 1,
            delivery_applied_prefix: 1,
        })
    );
    assert!(
        left.iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0)
    );
    let token = control
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    audit::reset();
    audit::in_render_scope(|| render.cancel_boundary(SampleTime(16)).unwrap());
    let cancel_audit = audit::snapshot();
    assert_eq!(
        cancel_audit.allocations, 0,
        "fault cancel boundary allocated"
    );
    assert_eq!(cancel_audit.deallocations, 0, "fault cancel boundary freed");
    assert_eq!(
        cancel_audit.total(),
        0,
        "fault cancel boundary used forbidden work"
    );
    let complete = control.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(
        (complete.applied_records, complete.canceled_records),
        (1, 1)
    );
    assert!(control.try_dequeue_event().is_ok());
    assert_eq!(control.outstanding(), 0);
    drop(render.stop());
    assert_eq!(failing.processed_frames, 7);
}

#[test]
fn controller_points_drive_real_asymmetric_pcm_replay_and_clock_refusal() {
    for ingress in [ControllerIngress::CallerBuffer, ControllerIngress::B1b] {
        run_controller_points_drive_real_asymmetric_pcm_replay_and_clock_refusal(ingress);
    }
}

fn run_controller_points_drive_real_asymmetric_pcm_replay_and_clock_refusal(
    ingress: ControllerIngress,
) {
    let mut fixture = real_controller_fixture();
    let revision = fixture.session.revision();
    let handles = fixture.handles;
    let preparation = fixture.effects.entries[0].bank_preparation.clone();
    let mut reference = fixture.effects.entries[0]
        .factory
        .prepare(PrepareEffectRequest {
            sample_rate: preparation.sample_rate,
            quantum: preparation.quantum,
            quality: preparation.quality,
            bypass: preparation.bypass,
            link_mode: preparation.link_mode,
            ports: preparation.ports,
            initial_values: &preparation.initial_values,
            limits: preparation.limits,
        })
        .expect("independent scalar reference");
    let mut no_event = fixture.effects.entries[0]
        .factory
        .prepare(PrepareEffectRequest {
            sample_rate: preparation.sample_rate,
            quantum: preparation.quantum,
            quality: preparation.quality,
            bypass: preparation.bypass,
            link_mode: preparation.link_mode,
            ports: preparation.ports,
            initial_values: &preparation.initial_values,
            limits: preparation.limits,
        })
        .expect("no-event scalar reference");
    warm_real(&mut *reference);
    warm_real(&mut *no_event);
    let processor = fixture.effects.entries[0].processor.as_mut();
    warm_real(processor);
    let (mut controller, render, resources) = prepare_controller_scalar_point_endpoint(
        processor,
        handles,
        fixture.session,
        controller_queue_config(),
        fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    )
    .expect("combined controller/scalar preparation");
    assert_eq!(
        resources.controller.queue_and_delivery,
        PreparedAutomationDelivery::resource_report_for_config(controller_queue_config()).unwrap()
    );
    assert_eq!(
        resources.scalar_render_inline_bytes,
        core::mem::size_of_val(&render)
    );

    let records = [(handles[0], 3, 6.0), (handles[1], 128, -6.0)];
    let encoded = encoded_controller_enqueue(revision, 1, &records);
    let response = process_controller_ingress(&mut controller, &encoded, ingress);
    assert_eq!(response_status(&response), StatusCode::Ok);
    let decoded = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("decode typed enqueue response");
    assert!(matches!(
        decoded,
        DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::AutomationEnqueued(accepted),
            ..
        } if accepted.accepted_records == 2
    ));
    let replay = process_controller_ingress(&mut controller, &encoded, ingress);
    assert_eq!(replay, response);
    let changed = encoded_controller_enqueue(
        revision,
        1,
        &[(handles[0], 3, 5.0), (handles[1], 128, -6.0)],
    );
    assert_eq!(
        response_status(&process_controller_ingress(
            &mut controller,
            &changed,
            ingress,
        )),
        StatusCode::RequestIdReuse
    );
    assert_eq!(controller.outstanding(), 1);
    let malformed_outer = encoded[..encoded.len() - 1].to_vec();
    let mut outer_output = vec![0_u8; 2_048];
    assert!(matches!(
        controller.process_command_frame_into(
            &malformed_outer,
            &mut DecodeScratch::new(&mut [0_u16; 64]),
            &mut outer_output,
        ),
        Err(protocol::CommandFrameProcessError::Uncorrelatable(
            protocol::DecodeError::BadPayloadLength
        ))
    ));
    let mut malformed_payload = encoded.clone();
    malformed_payload[24..32].copy_from_slice(&99_u64.to_le_bytes());
    malformed_payload[protocol::OUTER_HEADER_BYTES + 8] = 0;
    assert_eq!(
        response_status(&process_controller_frame(
            &mut controller,
            &malformed_payload
        )),
        StatusCode::MalformedFrame
    );
    assert_eq!(controller.outstanding(), 1);
    let ticket = match controller.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("expected Point handoff, got {other:?}"),
    };
    let mut render = render.start().unwrap_or_else(|_| panic!("render start"));
    let mut left = signal(128, 0x0532_0001, 0.69);
    let mut right = signal(128, 0x0532_0002, 0.57);
    let mut expected_left = left.clone();
    let mut expected_right = right.clone();
    let mut no_event_left = left.clone();
    let mut no_event_right = right.clone();
    let mut expected_report = ProcessReport::default();
    add_report(
        &mut expected_report,
        process_real(
            &mut *reference,
            &mut expected_left[..3],
            &mut expected_right[..3],
            0,
            &[],
        ),
    );
    add_report(
        &mut expected_report,
        process_real(
            &mut *reference,
            &mut expected_left[3..],
            &mut expected_right[3..],
            3,
            &[point_span(3, ParameterChannel::Left, 6.0)],
        ),
    );
    process_real(
        &mut *no_event,
        &mut no_event_left,
        &mut no_event_right,
        0,
        &[],
    );
    let report = render
        .render(&mut left, &mut right, SampleTime(0))
        .expect("real PCM render");
    assert_eq!(report.native, expected_report);
    assert_eq!(report.process_invocations, 2);
    assert_pcm_bits(&left, &expected_left);
    assert_pcm_bits(&right, &expected_right);
    assert!(
        left.iter()
            .zip(&no_event_left)
            .chain(right.iter().zip(&no_event_right))
            .any(|(actual, baseline)| actual.to_bits() != baseline.to_bits())
    );
    assert!(
        left.iter()
            .chain(&right)
            .any(|sample| sample.to_bits() & 0x7fff_ffff != 0)
    );
    let first_snapshot = render.snapshot().unwrap();
    assert_eq!(first_snapshot.applied, 1);
    assert_eq!(first_snapshot.pending.unwrap().1, 1);
    assert_eq!(first_snapshot.pending.unwrap().3, Some(SampleTime(128)));
    assert_state_bits(
        first_snapshot.state[0],
        reference
            .parameter_state(5, ParameterChannel::Left)
            .unwrap(),
    );
    assert_state_bits(
        first_snapshot.state[1],
        reference
            .parameter_state(5, ParameterChannel::Right)
            .unwrap(),
    );
    fixture
        .clock
        .0
        .store(first_snapshot.next_sample.0, Ordering::Release);

    let mut next_left = signal(128, 0x0532_0003, 0.69);
    let mut next_right = signal(128, 0x0532_0004, 0.57);
    let mut next_expected_left = next_left.clone();
    let mut next_expected_right = next_right.clone();
    let mut next_no_event_left = next_left.clone();
    let mut next_no_event_right = next_right.clone();
    let expected_second_report = process_real(
        &mut *reference,
        &mut next_expected_left,
        &mut next_expected_right,
        128,
        &[point_span(128, ParameterChannel::Right, -6.0)],
    );
    process_real(
        &mut *no_event,
        &mut next_no_event_left,
        &mut next_no_event_right,
        128,
        &[],
    );
    let second_report = render
        .render(&mut next_left, &mut next_right, SampleTime(128))
        .expect("future boundary PCM render");
    assert_eq!(second_report.native, expected_second_report);
    assert_eq!(second_report.process_invocations, 1);
    assert_pcm_bits(&next_left, &next_expected_left);
    assert_pcm_bits(&next_right, &next_expected_right);
    assert!(
        next_left
            .iter()
            .zip(&next_no_event_left)
            .chain(next_right.iter().zip(&next_no_event_right))
            .any(|(actual, baseline)| actual.to_bits() != baseline.to_bits())
    );
    let snapshot = render.snapshot().unwrap();
    assert_eq!(snapshot.applied, 2);
    assert_eq!(snapshot.pending, None);
    assert_eq!(snapshot.next_sample, SampleTime(256));
    assert_state_bits(
        snapshot.state[0],
        reference
            .parameter_state(5, ParameterChannel::Left)
            .unwrap(),
    );
    assert_state_bits(
        snapshot.state[1],
        reference
            .parameter_state(5, ParameterChannel::Right)
            .unwrap(),
    );
    assert_eq!(snapshot.state[0].target_value.to_bits(), 6.0_f32.to_bits());
    assert_eq!(
        snapshot.state[1].target_value.to_bits(),
        (-6.0_f32).to_bits()
    );
    assert_eq!(
        controller.collect_terminal(ticket).unwrap().applied_prefix,
        2
    );
    fixture
        .clock
        .0
        .store(snapshot.next_sample.0, Ordering::Release);
    let past = process_controller_ingress(
        &mut controller,
        &encoded_controller_enqueue(revision, 100, &[(handles[0], 200, 1.0)]),
        ingress,
    );
    assert_eq!(response_status(&past), StatusCode::TimeInPast);
    assert_eq!(controller.outstanding(), 0);
    let delivery_before_publication = (
        controller.outstanding(),
        controller.resident_automation(),
        controller.automation_status(),
    );
    let typed_unpublished = controller.process(ControllerRequest {
        request_id: RequestId::new(108).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"typed-unpublished",
        command: ControlCommand::ParameterStateGet {
            request: ParameterStateRequest {
                handles: vec![handles[0].0, handles[1].0],
            },
        },
    });
    assert_eq!(typed_unpublished.status, StatusCode::Unavailable);
    if matches!(ingress, ControllerIngress::CallerBuffer) {
        let short_request = encoded_state_get(revision, 109, &[handles[0].0, handles[1].0]);
        for capacity in [0, 2_047] {
            let mut output = vec![0_u8; capacity];
            assert!(matches!(
                controller.process_command_frame_into(
                    &short_request,
                    &mut DecodeScratch::new(&mut [0_u16; 64]),
                    &mut output,
                ),
                Err(protocol::CommandFrameProcessError::OutputReservationTooSmall { .. })
            ));
        }
        let mut exact = vec![0_u8; 2_048];
        let written = controller
            .process_command_frame_into(
                &short_request,
                &mut DecodeScratch::new(&mut [0_u16; 64]),
                &mut exact,
            )
            .unwrap();
        assert_eq!(response_status(&exact[..written]), StatusCode::Unavailable);
    }
    let unpublished_request = encoded_state_get(revision, 110, &[handles[0].0, handles[1].0]);
    let unpublished = process_controller_ingress(&mut controller, &unpublished_request, ingress);
    assert_eq!(response_status(&unpublished), StatusCode::Unavailable);
    let publication_allocations = bench_support::alloc::current_thread_counters();
    publish_controller_scalar_point_snapshot(&mut controller, handles, &snapshot)
        .expect("publish quiescent scalar snapshot");
    let publication_delta =
        bench_support::alloc::current_thread_delta_since(publication_allocations);
    assert_eq!(publication_delta.allocations, 0);
    assert_eq!(publication_delta.reallocations, 0);
    let state_request = encoded_state_get(revision, 111, &[handles[0].0, handles[1].0]);
    let state_frame = process_controller_ingress(&mut controller, &state_request, ingress);
    let page = decode_state_page(&state_frame);
    assert_eq!(page.observed_sample, snapshot.observed_sample.0);
    assert_eq!(page.records.len(), 2);
    assert_eq!(page.records[0].handle, handles[0].0);
    assert_eq!(page.records[1].handle, handles[1].0);
    assert_eq!(
        page.records[0].value.to_bits(),
        snapshot.state[0].current_value.to_bits()
    );
    assert_eq!(
        page.records[1].value.to_bits(),
        snapshot.state[1].current_value.to_bits()
    );
    assert_eq!(
        page.records[0].flags,
        1 | (u32::from(
            snapshot.state[0].current_value.to_bits() != snapshot.state[0].target_value.to_bits(),
        ) * 2)
    );
    let subset_frame = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 112, &[handles[1].0]),
        ingress,
    );
    let subset_page = decode_state_page(&subset_frame);
    assert_eq!(subset_page.records.len(), 1);
    assert_eq!(subset_page.records[0], page.records[1]);
    let typed = controller.process(ControllerRequest {
        request_id: RequestId::new(113).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"typed-state-reversed",
        command: ControlCommand::ParameterStateGet {
            request: ParameterStateRequest {
                handles: vec![handles[1].0, handles[0].0],
            },
        },
    });
    assert_eq!(typed.status, StatusCode::Ok);
    let typed_page = decode_state_page(&typed.frame);
    assert_eq!(
        typed_page
            .records
            .iter()
            .map(|record| record.handle)
            .collect::<Vec<_>>(),
        vec![handles[1].0, handles[0].0]
    );
    let unknown = controller.process(ControllerRequest {
        request_id: RequestId::new(114).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"typed-state-unknown",
        command: ControlCommand::ParameterStateGet {
            request: ParameterStateRequest { handles: vec![99] },
        },
    });
    assert_eq!(unknown.status, StatusCode::NotFound);
    let unknown_frame = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 115, &[99]),
        ingress,
    );
    assert_eq!(response_status(&unknown_frame), StatusCode::NotFound);
    assert_eq!(
        controller.publish_scalar_point_state(
            SampleTime(snapshot.observed_sample.0 + 1),
            [
                protocol::ParameterStateRecord {
                    handle: handles[1].0,
                    flags: 1,
                    value: 11.0,
                },
                protocol::ParameterStateRecord {
                    handle: handles[0].0,
                    flags: 1,
                    value: 12.0,
                },
            ],
        ),
        Err(ControllerAutomationPrepareError::InvalidScalarStatePublication)
    );
    let after_reversed = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 116, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_reversed), page);
    assert_eq!(
        controller.publish_scalar_point_state(
            snapshot.observed_sample,
            [
                protocol::ParameterStateRecord {
                    handle: 0,
                    flags: 1,
                    value: 1.0,
                },
                page.records[1],
            ],
        ),
        Err(ControllerAutomationPrepareError::InvalidScalarStatePublication)
    );
    let after_zero_handle = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 117, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_zero_handle), page);
    assert_eq!(
        controller.publish_scalar_point_state(
            snapshot.observed_sample,
            [page.records[0], page.records[0]],
        ),
        Err(ControllerAutomationPrepareError::InvalidScalarStatePublication)
    );
    let after_duplicate_handle = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 118, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_duplicate_handle), page);
    assert_eq!(
        controller.publish_scalar_point_state(
            snapshot.observed_sample,
            [
                protocol::ParameterStateRecord {
                    handle: handles[0].0,
                    flags: 4,
                    value: 1.0,
                },
                page.records[1],
            ],
        ),
        Err(ControllerAutomationPrepareError::InvalidScalarStatePublication)
    );
    let after_flags = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 119, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_flags), page);
    assert_eq!(
        controller.publish_scalar_point_state(
            snapshot.observed_sample,
            [
                protocol::ParameterStateRecord {
                    handle: handles[0].0,
                    flags: 1,
                    value: f32::NAN,
                },
                page.records[1],
            ],
        ),
        Err(ControllerAutomationPrepareError::InvalidScalarStatePublication)
    );
    let after_rejection = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 120, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_rejection), page);
    controller
        .publish_scalar_point_state(
            snapshot.observed_sample,
            [
                protocol::ParameterStateRecord {
                    handle: handles[0].0,
                    flags: page.records[0].flags,
                    value: page.records[0].value,
                },
                protocol::ParameterStateRecord {
                    handle: handles[1].0,
                    flags: page.records[1].flags,
                    value: page.records[1].value,
                },
            ],
        )
        .unwrap();
    let after_identical = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 121, &[handles[0].0, handles[1].0]),
        ingress,
    );
    assert_eq!(decode_state_page(&after_identical), page);
    let mut replacement = snapshot;
    replacement.observed_sample = SampleTime(snapshot.observed_sample.0 + 7);
    replacement.state[0].current_value = 2.0;
    replacement.state[0].target_value = 2.0;
    replacement.state[1].current_value = -3.0;
    replacement.state[1].target_value = -4.0;
    publish_controller_scalar_point_snapshot(&mut controller, handles, &replacement).unwrap();
    let replaced = process_controller_ingress(
        &mut controller,
        &encoded_state_get(revision, 122, &[handles[0].0, handles[1].0]),
        ingress,
    );
    let replaced_page = decode_state_page(&replaced);
    assert_eq!(replaced_page.observed_sample, replacement.observed_sample.0);
    assert_eq!(replaced_page.records[0].value.to_bits(), 2.0_f32.to_bits());
    assert_eq!(replaced_page.records[0].flags, 1);
    assert_eq!(
        replaced_page.records[1].value.to_bits(),
        (-3.0_f32).to_bits()
    );
    assert_eq!(replaced_page.records[1].flags, 3);
    assert_eq!(
        process_controller_ingress(&mut controller, &state_request, ingress),
        state_frame
    );
    assert_eq!(
        response_status(&process_controller_ingress(
            &mut controller,
            &encoded_state_get(revision, 111, &[handles[0].0]),
            ingress,
        )),
        StatusCode::RequestIdReuse
    );
    assert_eq!(
        (
            controller.outstanding(),
            controller.resident_automation(),
            controller.automation_status(),
        ),
        delivery_before_publication
    );
    assert!(
        controller
            .dequeue_reliable_event_frame_into(&mut [0_u8; 64])
            .unwrap()
            .is_none()
    );
    drop(render.stop());
}

#[test]
fn controller_publication_survives_real_sticky_snapshot_fault() {
    let fixture = real_controller_fixture();
    let handles = fixture.handles;
    let revision = fixture.session.revision();
    let mut failing = FailingAccessEffect::read_failure();
    let mut queues = controller_queue_config();
    queues.quantum_frames = NonZeroUsize::new(16).unwrap();
    let (mut controller, render, _) = prepare_controller_scalar_point_endpoint(
        &mut failing,
        handles,
        fixture.session,
        queues,
        fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    )
    .expect("combined sticky-fault preparation");
    let mut render = render.start().unwrap_or_else(|_| panic!("render start"));
    let published = render.snapshot().unwrap();
    publish_controller_scalar_point_snapshot(&mut controller, handles, &published).unwrap();
    let mut left = signal(16, 0x0532_3001, 0.69);
    let mut right = signal(16, 0x0532_3002, 0.57);
    render
        .render(&mut left, &mut right, SampleTime(0))
        .expect("fault fixture render");
    let fault = render.snapshot().unwrap_err();
    assert_eq!(render.snapshot(), Err(fault));
    let response = controller.process(ControllerRequest {
        request_id: RequestId::new(1).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"sticky-fault-state",
        command: ControlCommand::ParameterStateGet {
            request: ParameterStateRequest {
                handles: vec![handles[0].0, handles[1].0],
            },
        },
    });
    assert_eq!(response.status, StatusCode::Ok);
    let page = decode_state_page(&response.frame);
    assert_eq!(page.observed_sample, published.observed_sample.0);
    assert_eq!(
        page.records[0].value.to_bits(),
        published.state[0].current_value.to_bits()
    );
    assert_eq!(
        page.records[1].value.to_bits(),
        published.state[1].current_value.to_bits()
    );
    drop(render.stop());
}

#[test]
fn controller_cancellation_keeps_real_prefix_event_credit_and_native_state() {
    for ingress in [ControllerIngress::CallerBuffer, ControllerIngress::B1b] {
        run_controller_cancellation_keeps_real_prefix_event_credit_and_native_state(ingress);
    }
}

fn run_controller_cancellation_keeps_real_prefix_event_credit_and_native_state(
    ingress: ControllerIngress,
) {
    let mut fixture = real_controller_fixture();
    let revision = fixture.session.revision();
    let handles = fixture.handles;
    let processor = fixture.effects.entries[0].processor.as_mut();
    warm_real(processor);
    let (mut controller, render, _) = prepare_controller_scalar_point_endpoint(
        processor,
        handles,
        fixture.session,
        controller_queue_config(),
        fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    )
    .expect("combined cancellation preparation");
    let response = process_controller_ingress(
        &mut controller,
        &encoded_controller_enqueue(
            revision,
            10,
            &[(handles[0], 3, 6.0), (handles[1], 200, -6.0)],
        ),
        ingress,
    );
    assert_eq!(response_status(&response), StatusCode::Ok);
    let ticket = match controller.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("expected Point handoff, got {other:?}"),
    };
    let mut render = render.start().unwrap_or_else(|_| panic!("render start"));
    let initial = render.snapshot().unwrap();
    let mut left = signal(128, 0x0532_2001, 0.69);
    let mut right = signal(128, 0x0532_2002, 0.57);
    render
        .render(&mut left, &mut right, SampleTime(0))
        .expect("prefix render");
    let prefix = render.snapshot().unwrap();
    assert_eq!(prefix.applied, 1);
    assert_eq!(prefix.pending.unwrap().1, 1);
    assert_eq!(prefix.pending.unwrap().3, Some(SampleTime(200)));
    assert_eq!(prefix.next_sample, SampleTime(128));
    assert_eq!(prefix.state[0].target_value.to_bits(), 6.0_f32.to_bits());
    assert_state_bits(prefix.state[1], initial.state[1]);
    publish_controller_scalar_point_snapshot(&mut controller, handles, &prefix).unwrap();

    fixture
        .clock
        .0
        .store(prefix.next_sample.0, Ordering::Release);
    let token = controller
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    let mut next_left = signal(128, 0x0532_2003, 0.69);
    let mut next_right = signal(128, 0x0532_2004, 0.57);
    render
        .render(&mut next_left, &mut next_right, prefix.next_sample)
        .expect("cancellation boundary render");
    let canceled = render.snapshot().unwrap();
    assert_eq!(canceled.applied, 1);
    assert_eq!(canceled.pending, None);
    assert_eq!(canceled.next_sample, SampleTime(256));
    assert_state_bits(canceled.state[0], prefix.state[0]);
    assert_state_bits(canceled.state[1], prefix.state[1]);
    fixture
        .clock
        .0
        .store(canceled.next_sample.0, Ordering::Release);
    let complete = controller.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(
        (
            complete.applied_records,
            complete.canceled_records,
            complete.canceled_events,
            complete.effective_sample,
        ),
        (1, 1, 1, SampleTime(128))
    );
    assert_eq!(controller.outstanding(), 0);
    assert_eq!(controller.resident_automation(), 0);
    let after_cancel = controller.process(ControllerRequest {
        request_id: RequestId::new(13).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"published-after-cancel",
        command: ControlCommand::ParameterStateGet {
            request: ParameterStateRequest {
                handles: vec![handles[0].0, handles[1].0],
            },
        },
    });
    assert_eq!(after_cancel.status, StatusCode::Ok);
    let after_cancel_page = decode_state_page(&after_cancel.frame);
    assert_eq!(after_cancel_page.observed_sample, prefix.observed_sample.0);
    assert_eq!(
        after_cancel_page.records[0].value.to_bits(),
        prefix.state[0].current_value.to_bits()
    );
    assert_eq!(
        after_cancel_page.records[1].value.to_bits(),
        prefix.state[1].current_value.to_bits()
    );
    assert_eq!(
        after_cancel_page.records[0].flags,
        1 | (u32::from(
            prefix.state[0].current_value.to_bits() != prefix.state[0].target_value.to_bits(),
        ) * 2)
    );
    assert_eq!(
        after_cancel_page.records[1].flags,
        1 | (u32::from(
            prefix.state[1].current_value.to_bits() != prefix.state[1].target_value.to_bits(),
        ) * 2)
    );
    assert_eq!(
        controller.collect_terminal(ticket),
        Err(DeliveryError::StaleTicket)
    );

    let mut encoded = vec![0_u8; 4_096];
    let encoded_len = controller
        .dequeue_reliable_event_frame_into(&mut encoded)
        .unwrap()
        .expect("one encoded cancellation event");
    let event = ProtocolCodec::default()
        .decode_typed_event(
            &encoded[..encoded_len],
            &mut DecodeScratch::new(&mut [0_u16; 64]),
        )
        .expect("decode cancellation event");
    assert!(matches!(
        event,
        DecodedTypedEventFrame {
            payload: DecodedEventPayload::AutomationCanceled(value),
            ..
        } if value.origin_request_id == RequestId::new(10).unwrap()
            && value.canceled_records == 1
            && value.effective_sample == Some(SampleTime(128))
            && value.reason == AutomationCancellationReason::EndpointShutdown
    ));
    assert!(
        controller
            .dequeue_reliable_event_frame_into(&mut encoded)
            .unwrap()
            .is_none()
    );

    let counters = controller.process(ControllerRequest {
        request_id: RequestId::new(14).unwrap(),
        expected_revision: ExpectedRevision::Exact(revision),
        canonical_bytes: b"canceled-counter",
        command: ControlCommand::CountersGet {
            request: CountersRequest {
                all: true,
                ids: Vec::new(),
            },
        },
    });
    assert_eq!(counters.status, StatusCode::Ok);
    let decoded = ProtocolCodec::default()
        .decode_typed_response(&counters.frame, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("decode canceled counter");
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
        ..
    } = decoded
    else {
        panic!("expected counter snapshot");
    };
    assert_eq!(
        snapshot
            .values
            .iter()
            .find(|value| value.id == CounterId::CanceledAutomation)
            .map(|value| value.value),
        Some(1)
    );

    let replacement = process_controller_ingress(
        &mut controller,
        &encoded_controller_enqueue(revision, 15, &[(handles[0], 300, 2.0)]),
        ingress,
    );
    assert_eq!(response_status(&replacement), StatusCode::Ok);
    let replacement_ticket = match controller.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("expected replacement handoff, got {other:?}"),
    };
    let mut replacement_left = signal(128, 0x0532_2005, 0.69);
    let mut replacement_right = signal(128, 0x0532_2006, 0.57);
    render
        .render(
            &mut replacement_left,
            &mut replacement_right,
            SampleTime(256),
        )
        .expect("replacement render");
    fixture
        .clock
        .0
        .store(render.snapshot().unwrap().next_sample.0, Ordering::Release);
    assert_eq!(
        controller
            .collect_terminal(replacement_ticket)
            .unwrap()
            .applied_prefix,
        1
    );
    assert_eq!(controller.outstanding(), 0);
    drop(render.stop());
}

#[test]
fn controller_scalar_preflight_rejections_and_single_allocation_authority() {
    const CHILD: &str = "MISO_ENGINE_CONTROLLER_SCALAR_GROUP3_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg("controller_scalar_preflight_rejections_and_single_allocation_authority")
            .arg("--test-threads=1")
            .arg("--nocapture")
            .env(CHILD, "1")
            .status()
            .expect("isolated Group3 child");
        assert!(status.success(), "isolated Group3 child failed");
        return;
    }

    use bench_support::alloc as bench_alloc;
    use std::hint::black_box;

    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let direct_fixture = real_controller_fixture();
    let direct_handles = direct_fixture.handles;
    let direct_capabilities = PreparedDeliveryCapabilities::new_exact(&[
        (direct_handles[0], AutomationKind::Point),
        (direct_handles[1], AutomationKind::Point),
    ])
    .expect("direct Point capabilities");
    let ((direct_controller, direct_render, direct_resources), direct_diagnostic) =
        measure_preparation(|| {
            ControllerAutomationDelivery::prepare(
                direct_fixture.session,
                controller_queue_config(),
                direct_fixture.provider,
                controller_replay_config(),
                ProtocolCodec::default(),
                controller_config(),
                ControllerRetainedCapacity {
                    meter_handles: 0,
                    counter_ids: 0,
                },
                direct_capabilities,
            )
            .expect("direct #530 preparation")
        });
    black_box((&direct_controller, &direct_render, &direct_resources));

    let mut combined_fixture = real_controller_fixture();
    let combined_handles = combined_fixture.handles;
    let processor = combined_fixture.effects.entries[0].processor.as_mut();
    let ((combined_controller, combined_render, resources), combined_diagnostic) =
        measure_preparation(|| {
            prepare_controller_scalar_point_endpoint(
                processor,
                combined_handles,
                combined_fixture.session,
                controller_queue_config(),
                combined_fixture.provider,
                controller_replay_config(),
                ProtocolCodec::default(),
                controller_config(),
                ControllerRetainedCapacity {
                    meter_handles: 0,
                    counter_ids: 0,
                },
            )
            .expect("combined preparation")
        });
    eprintln!(
        "controller scalar Point preparation diagnostics: direct global={direct_global:?} current={direct_current:?} thread={direct_thread:?}; wrapped global={wrapped_global:?} current={wrapped_current:?} thread={wrapped_thread:?}",
        direct_global = direct_diagnostic.global,
        direct_current = direct_diagnostic.current_thread,
        direct_thread = direct_diagnostic.thread_audit,
        wrapped_global = combined_diagnostic.global,
        wrapped_current = combined_diagnostic.current_thread,
        wrapped_thread = combined_diagnostic.thread_audit,
    );
    let transient_bytes = ("comp0".len() + "comp".len()) as u64;
    assert_eq!(
        combined_diagnostic.current_thread.allocations,
        direct_diagnostic.current_thread.allocations + 2
    );
    assert_eq!(
        combined_diagnostic.current_thread.deallocations,
        direct_diagnostic.current_thread.deallocations + 2
    );
    assert_eq!(
        combined_diagnostic.current_thread.reallocations,
        direct_diagnostic.current_thread.reallocations
    );
    assert_eq!(
        combined_diagnostic.current_thread.requested_bytes,
        direct_diagnostic.current_thread.requested_bytes + transient_bytes
    );
    assert_global_dominates_current_thread(direct_diagnostic);
    assert_global_dominates_current_thread(combined_diagnostic);
    assert_eq!(
        resources.controller.queue_and_delivery,
        direct_resources.queue_and_delivery
    );
    assert_eq!(
        resources.controller.queue_and_delivery,
        PreparedAutomationDelivery::resource_report_for_config(controller_queue_config()).unwrap()
    );
    assert_eq!(
        resources.scalar_render_inline_bytes,
        core::mem::size_of_val(&combined_render)
    );
    black_box((&combined_controller, &combined_render, &resources));
    drop(combined_render);
    drop(combined_controller);
    drop(direct_render);
    drop(direct_controller);

    let bad_native_fixture = real_controller_fixture();
    let bad_native_handles = bad_native_fixture.handles;
    let mut bad_native = effect();
    let bad_native_result = prepare_controller_scalar_point_endpoint(
        &mut *bad_native,
        bad_native_handles,
        bad_native_fixture.session,
        controller_queue_config(),
        bad_native_fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    );
    assert!(matches!(
        bad_native_result,
        Err(ControllerScalarPointPrepareError::ScalarPoint(
            host_core::ScalarPointPrepareError::InvalidProcessor
        ))
    ));
    let mut bad_left = [0.1_f32; Q];
    let mut bad_right = [-0.1_f32; Q];
    warm(&mut *bad_native);
    process(&mut *bad_native, &mut bad_left, &mut bad_right, 0, &[]);
    assert!(
        bad_left
            .iter()
            .chain(&bad_right)
            .any(|sample| *sample != 0.0)
    );

    let mut bad_binding_fixture = real_controller_fixture();
    let bad_binding_handles = bad_binding_fixture.handles;
    let wrong_right = ParameterHandle(bad_binding_handles[1].0 + 1);
    let bad_binding = prepare_controller_scalar_point_endpoint(
        bad_binding_fixture.effects.entries[0].processor.as_mut(),
        [bad_binding_handles[0], wrong_right],
        bad_binding_fixture.session,
        controller_queue_config(),
        bad_binding_fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    );
    assert!(matches!(
        bad_binding,
        Err(ControllerScalarPointPrepareError::InvalidProviderBinding)
    ));

    let mut bad_clock_fixture = real_controller_fixture();
    let bad_clock_handles = bad_clock_fixture.handles;
    bad_clock_fixture.clock.0.store(1, Ordering::Release);
    let bad_clock = prepare_controller_scalar_point_endpoint(
        bad_clock_fixture.effects.entries[0].processor.as_mut(),
        bad_clock_handles,
        bad_clock_fixture.session,
        controller_queue_config(),
        bad_clock_fixture.provider,
        controller_replay_config(),
        ProtocolCodec::default(),
        controller_config(),
        ControllerRetainedCapacity {
            meter_handles: 0,
            counter_ids: 0,
        },
    );
    assert!(matches!(
        bad_clock,
        Err(ControllerScalarPointPrepareError::InitialSampleMismatch {
            observed: SampleTime(1)
        })
    ));
}
