//! Deterministic caller-buffer allocation audit for the issue-005 protocol surface.

#![allow(unsafe_code)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    num::NonZeroUsize,
};

use miso_engine_protocol::{
    AutomationEnqueue, AutomationRecord, CommandPayload, ControlCommand, ControllerRequest,
    CounterId, CounterSnapshot, CounterValue, DecodeScratch, DiagnosticsRequest, EventPayload,
    ExpectedRevision, MessageId, MeterComponent, MeterRecord, MockProvider, NonOkResponse,
    ProtocolCodec, ProtocolController, ProtocolQueueConfig, ProtocolQueues, ReplayCache,
    ReplayCacheConfig, RequestId, SampleTime, SessionEditV1, SessionRevision, SessionStore,
    StatusCode, SuccessResponsePayload, TelemetryConfiguration, TransactionApplied,
    TypedCommandFrame, TypedEventFrame, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};
use miso_engine_session::{CompileCaps, StableId, parse_session_toml};

struct AuditAllocator;

std::thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditAllocator = AuditAllocator;

fn count_allocation() {
    ARMED.with(|armed| {
        if armed.get() {
            ALLOCATIONS.with(|count| count.set(count.get().saturating_add(1)));
        }
    });
}

// SAFETY: this audit-only global allocator forwards every valid pointer/layout contract unchanged
// to `System`; while armed it only increments thread-local counters and never owns, retains, or
// alters allocator data. The audit is isolated in this non-production tool.
unsafe impl GlobalAlloc for AuditAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the caller's valid layout unchanged to the system allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the caller's valid layout unchanged to the system allocator.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwards the original allocator pointer/layout pair unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the original pointer/layout and requested size unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn request_id(value: u64) -> RequestId {
    RequestId::new(value).expect("nonzero request ID")
}

fn assert_zero_allocations(operation: impl FnOnce()) {
    ALLOCATIONS.with(|count| count.set(0));
    ARMED.with(|armed| armed.set(true));
    operation();
    ARMED.with(|armed| armed.set(false));
    assert_eq!(
        ALLOCATIONS.with(Cell::get),
        0,
        "caller-buffer path allocated"
    );
}

struct Corpus {
    codec: ProtocolCodec,
    full_frames: Vec<Vec<u8>>,
    transaction_edits: Vec<SessionEditV1>,
    non_ok: NonOkResponse,
    frame_output: Vec<u8>,
    decode_fields: [u16; 1024],
    automation_batches: Vec<[AutomationRecord; miso_engine_protocol::AUTOMATION_BATCH_RECORDS]>,
    automation_counts: [usize; 40],
    queues: ProtocolQueues,
}

/// Prepared egress state. Reliable, meter, and counter dequeue/encode paths are allocation-free
/// after this construction/configuration phase. Diagnostic egress is intentionally excluded: its
/// bounded typed `Diagnostic` owns control-plane strings and structured paths.
struct EgressCorpus {
    controller: ProtocolController<MockProvider>,
    output: Vec<u8>,
}

impl Corpus {
    fn push_command(&mut self, frame: TypedCommandFrame<'_>) {
        let mut bytes = vec![0_u8; 32 * 1024];
        let length = self
            .codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("prepared command");
        bytes.truncate(length);
        self.full_frames.push(bytes);
    }

    fn push_success(&mut self, frame: TypedSuccessResponseFrame<'_>) {
        let mut bytes = vec![0_u8; 32 * 1024];
        let length = self
            .codec
            .encode_success_response_frame_into(&frame, &mut bytes)
            .expect("prepared success response");
        bytes.truncate(length);
        self.full_frames.push(bytes);
    }

    fn push_event(&mut self, frame: TypedEventFrame<'_>) {
        let mut bytes = vec![0_u8; 32 * 1024];
        let length = self
            .codec
            .encode_event_frame_into(&frame, &mut bytes)
            .expect("prepared event");
        bytes.truncate(length);
        self.full_frames.push(bytes);
    }
}

fn prepare_corpus() -> Corpus {
    let codec = ProtocolCodec::default();
    let edits = (0..64)
        .map(|index| SessionEditV1::SetSessionId {
            session_id: StableId::parse(&format!("audit-{index}")).expect("prepared stable ID"),
        })
        .collect::<Vec<_>>();
    let automation_counts = core::array::from_fn(|index| if index == 39 { 16 } else { 256 });
    let mut next_sample = 0_u64;
    let mut automation_batches = Vec::with_capacity(automation_counts.len());
    for count in automation_counts {
        let mut records = [AutomationRecord::EMPTY; miso_engine_protocol::AUTOMATION_BATCH_RECORDS];
        for record in &mut records[..count] {
            *record = AutomationRecord {
                kind: miso_engine_protocol::AutomationKind::Point,
                handle: miso_engine_protocol::ParameterHandle(1),
                start: SampleTime(next_sample),
                end: SampleTime(next_sample),
                start_value: next_sample as f32,
                end_value: next_sample as f32,
            };
            next_sample += 1;
        }
        automation_batches.push(records);
    }
    assert_eq!(next_sample, 10_000, "prepared exact automation corpus");
    let queue_config = ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(1).expect("prepared control slots"),
        control_command_bytes: NonZeroUsize::new(1).expect("prepared control bytes"),
        automation_batch_slots: NonZeroUsize::new(1).expect("prepared automation slots"),
        reliable_response_slots: NonZeroUsize::new(1).expect("prepared response slots"),
        reliable_event_slots: NonZeroUsize::new(1).expect("prepared event slots"),
        telemetry_slots: NonZeroUsize::new(1).expect("prepared telemetry slots"),
        per_block_automation_density: NonZeroUsize::new(256).expect("prepared density"),
        quantum_frames: NonZeroUsize::new(1).expect("prepared quantum"),
    };
    let non_ok = NonOkResponse {
        diagnostics: Vec::new(),
        omitted_diagnostics: 0,
        backpressure: None,
    };
    let mut corpus = Corpus {
        codec,
        full_frames: Vec::new(),
        transaction_edits: Vec::new(),
        non_ok,
        frame_output: vec![0_u8; 32 * 1024],
        decode_fields: [0_u16; 1024],
        automation_batches,
        automation_counts,
        queues: ProtocolQueues::prepare(queue_config).expect("prepared protocol queues"),
    };
    let telemetry = TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let counters = miso_engine_protocol::CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    let parameter_state = miso_engine_protocol::ParameterStateRequest { handles: vec![1] };
    let automation_record = [AutomationRecord {
        kind: miso_engine_protocol::AutomationKind::Point,
        handle: miso_engine_protocol::ParameterHandle(1),
        start: SampleTime(0),
        end: SampleTime(0),
        start_value: 0.0,
        end_value: 0.0,
    }];
    let revision = ExpectedRevision::Exact(SessionRevision(7));
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(1),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::CapabilitiesGet,
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(2),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::SessionSnapshotGet(miso_engine_protocol::SessionSnapshotRequest {
            offset: 0,
            maximum_bytes: 1,
        }),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(3),
        expected_revision: revision,
        payload: CommandPayload::SessionTransactionApply(&edits),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(4),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::ParameterMetadataGet(
            miso_engine_protocol::ParameterMetadataRequest {
                after_handle: 0,
                limit: 1,
            },
        ),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(5),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::ParameterStateGet(&parameter_state),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(6),
        expected_revision: revision,
        payload: CommandPayload::AutomationEnqueue(AutomationEnqueue {
            records: &automation_record,
        }),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(7),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::TransportGet,
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(8),
        expected_revision: revision,
        payload: CommandPayload::TransportSet(miso_engine_protocol::TransportSetRequest {
            state: miso_engine_protocol::TransportState::Stopped,
            position: None,
        }),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(9),
        expected_revision: revision,
        payload: CommandPayload::TelemetryConfigure(&telemetry),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(10),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::CountersGet(&counters),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(11),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::DiagnosticsGet(DiagnosticsRequest {
            after_sequence: 0,
            limit: 1,
            minimum_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        }),
    });
    corpus.push_success(TypedSuccessResponseFrame {
        request_id: request_id(12),
        revision: SessionRevision(7),
        payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
            applied_operations: 64,
        }),
    });
    corpus.push_event(TypedEventFrame {
        revision: SessionRevision(7),
        payload: EventPayload::SessionCommitted(miso_engine_protocol::SessionCommitted {
            event_sequence: 1,
            origin_request_id: request_id(3),
            previous_revision: SessionRevision(6),
            applied_operations: 64,
        }),
    });
    let mut non_ok_bytes = vec![0_u8; 128];
    let length = corpus
        .codec
        .encode_non_ok_response_frame_into(
            &TypedNonOkResponseFrame {
                request_id: request_id(13),
                revision: SessionRevision(7),
                message_id: MessageId::CapabilitiesGet,
                status: StatusCode::InvalidField,
                payload: &corpus.non_ok,
            },
            &mut non_ok_bytes,
        )
        .expect("prepared non-OK response");
    non_ok_bytes.truncate(length);
    corpus.full_frames.push(non_ok_bytes);

    corpus.transaction_edits = edits;
    corpus
}

fn prepare_egress_corpus() -> EgressCorpus {
    let session = SessionStore::new(
        parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
            .expect("prepared session"),
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("prepared store");
    let queues = ProtocolQueues::prepare(ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(1).expect("control"),
        control_command_bytes: NonZeroUsize::new(64).expect("bytes"),
        automation_batch_slots: NonZeroUsize::new(1).expect("automation"),
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(1).expect("event"),
        telemetry_slots: NonZeroUsize::new(1).expect("telemetry"),
        per_block_automation_density: NonZeroUsize::new(256).expect("density"),
        quantum_frames: NonZeroUsize::new(1).expect("quantum"),
    })
    .expect("prepared queues");
    let mut controller = ProtocolController::new(
        session,
        queues,
        MockProvider::default(),
        ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(4).expect("replay"),
            bytes: NonZeroUsize::new(4096).expect("replay bytes"),
            max_response_bytes: 1024,
        }),
    );
    let telemetry = TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: vec![CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let response = controller.process(ControllerRequest {
        request_id: request_id(20_000),
        expected_revision: ExpectedRevision::Exact(controller.session().revision()),
        canonical_bytes: b"prepared-egress-config",
        command: ControlCommand::TelemetryConfigure {
            configuration: telemetry,
        },
    });
    assert_eq!(response.status, StatusCode::Ok, "prepared telemetry config");
    let revision = controller.session().revision();
    controller
        .queues_mut()
        .try_enqueue_event(miso_engine_protocol::ReliableSlot::session_committed(
            revision,
            1,
            request_id(20_001),
            SessionRevision(revision.0.saturating_sub(1)),
            1,
        ))
        .expect("prepared reliable event");
    let meters = [MeterRecord {
        handle: 1,
        component: MeterComponent::Left,
        flags: 1,
        value: 0.0,
    }];
    controller
        .stage_meter_batch_event(revision, SampleTime(0), &meters)
        .expect("prepared meter event");
    let counters = CounterSnapshot {
        observed_sample: SampleTime(0),
        values: vec![CounterValue {
            id: CounterId::ControlCommandBackpressure,
            value: 1,
        }],
    };
    controller
        .stage_counter_snapshot_event(revision, &counters)
        .expect("prepared counter event");
    EgressCorpus {
        controller,
        output: vec![0_u8; 4096],
    }
}

fn run_audit(corpus: &mut Corpus, egress: &mut EgressCorpus) {
    assert_zero_allocations(|| {
        for frame in &corpus.full_frames {
            corpus
                .codec
                .decode(frame, &mut DecodeScratch::new(&mut corpus.decode_fields))
                .expect("prepared full frame decodes");
        }
        let command = TypedCommandFrame {
            request_id: request_id(100),
            expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
            payload: CommandPayload::SessionTransactionApply(&corpus.transaction_edits),
        };
        corpus.frame_output.fill(0xa5);
        let command_len = corpus
            .codec
            .encode_command_frame_into(&command, &mut corpus.frame_output)
            .expect("caller output command encode");
        assert!(
            corpus.frame_output[command_len..]
                .iter()
                .all(|byte| *byte == 0xa5)
        );
        let response = TypedSuccessResponseFrame {
            request_id: request_id(101),
            revision: SessionRevision(7),
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
                applied_operations: 64,
            }),
        };
        corpus.frame_output.fill(0xa5);
        let response_len = corpus
            .codec
            .encode_success_response_frame_into(&response, &mut corpus.frame_output)
            .expect("caller output response encode");
        assert!(
            corpus.frame_output[response_len..]
                .iter()
                .all(|byte| *byte == 0xa5)
        );
        let event = TypedEventFrame {
            revision: SessionRevision(7),
            payload: EventPayload::SessionCommitted(miso_engine_protocol::SessionCommitted {
                event_sequence: 2,
                origin_request_id: request_id(100),
                previous_revision: SessionRevision(6),
                applied_operations: 64,
            }),
        };
        corpus.frame_output.fill(0xa5);
        let event_len = corpus
            .codec
            .encode_event_frame_into(&event, &mut corpus.frame_output)
            .expect("caller output event encode");
        assert!(
            corpus.frame_output[event_len..]
                .iter()
                .all(|byte| *byte == 0xa5)
        );
        let non_ok = TypedNonOkResponseFrame {
            request_id: request_id(102),
            revision: SessionRevision(7),
            message_id: MessageId::CapabilitiesGet,
            status: StatusCode::InvalidField,
            payload: &corpus.non_ok,
        };
        corpus.frame_output.fill(0xa5);
        let non_ok_len = corpus
            .codec
            .encode_non_ok_response_frame_into(&non_ok, &mut corpus.frame_output)
            .expect("caller output non-OK encode");
        assert!(
            corpus.frame_output[non_ok_len..]
                .iter()
                .all(|byte| *byte == 0xa5)
        );

        let mut audited_records = 0_usize;
        for (batch_index, (records, count)) in corpus
            .automation_batches
            .iter()
            .zip(corpus.automation_counts)
            .enumerate()
        {
            corpus.frame_output.fill(0xa5);
            let length = corpus
                .codec
                .encode_automation_enqueue(
                    AutomationEnqueue {
                        records: &records[..count],
                    },
                    &mut corpus.frame_output,
                )
                .expect("caller output automation encode");
            assert!(
                corpus.frame_output[length..]
                    .iter()
                    .all(|byte| *byte == 0xa5)
            );
            let decoded = corpus
                .codec
                .decode_automation_enqueue(&corpus.frame_output[..length], 3)
                .expect("caller input automation decode");
            let slot = decoded
                .into_batch(
                    SessionRevision(7),
                    request_id(u64::try_from(batch_index).expect("batch index") + 1_000),
                )
                .expect("decoded automation slot");
            corpus
                .queues
                .try_enqueue_automation(SampleTime(0), slot)
                .expect("prepared queue accepts batch");
            let dequeued = corpus
                .queues
                .try_dequeue_automation()
                .expect("prepared queue returns batch");
            assert_eq!(dequeued.as_slice(), &records[..count]);
            audited_records += dequeued.as_slice().len();
        }
        assert_eq!(audited_records, 10_000, "exact automation record audit");

        egress.output.fill(0xa5);
        let reliable = egress
            .controller
            .dequeue_reliable_event_frame_into(&mut egress.output)
            .expect("reliable egress")
            .expect("reliable event");
        assert!(egress.output[reliable..].iter().all(|byte| *byte == 0xa5));
        egress.output.fill(0xa5);
        let meter = egress
            .controller
            .dequeue_lossy_event_frame_into(&mut egress.output)
            .expect("meter egress")
            .expect("meter event");
        assert!(egress.output[meter..].iter().all(|byte| *byte == 0xa5));
        egress.output.fill(0xa5);
        let counter = egress
            .controller
            .dequeue_lossy_event_frame_into(&mut egress.output)
            .expect("counter egress")
            .expect("counter event");
        assert!(egress.output[counter..].iter().all(|byte| *byte == 0xa5));
    });
}

fn main() {
    let mut corpus = prepare_corpus();
    let mut egress = prepare_egress_corpus();
    run_audit(&mut corpus, &mut egress);
    println!(
        "protocol caller-buffer audit: ok (command, success, non-OK, event, reliable/meter/counter egress, 64 edits, 10,000 automation records in 40 batches; diagnostic egress is control-plane typed storage)"
    );
}
