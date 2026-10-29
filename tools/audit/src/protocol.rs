//! Deterministic caller-buffer allocation audit for the issue-005 protocol surface.

use std::num::NonZeroUsize;

use bench_support::alloc as bench_alloc;
use protocol::{
    CommandPayload, ControlCommand, ControllerRequest, CounterId, CounterSnapshot, CounterValue,
    DecodeScratch, DiagnosticsRequest, EditPath, EventPayload, ExpectedRevision, MessageId,
    MeterComponent, MeterRecord, MockProvider, NonOkResponse, ProtocolCodec, ProtocolController,
    ProtocolQueueConfig, ProtocolQueues, ReplayCache, ReplayCacheConfig, RequestId, SampleTime,
    SessionEdit, SessionRevision, SessionStore, StatusCode, SuccessResponsePayload,
    TelemetryConfiguration, TransactionApplied, TypedCommandFrame, TypedEventFrame,
    TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};
use session::{CompileCaps, StableId, parse_session_json};

fn request_id(value: u64) -> RequestId {
    RequestId::new(value).expect("nonzero request ID")
}

fn assert_zero_allocations(operation: impl FnOnce()) {
    // #104 F4: the same audited allocator every other tool uses. The thread-local armed counter
    // this replaces counted the same events; it just counted them in a fourteenth copy of the
    // wrapper.
    let mark = bench_alloc::counters();
    operation();
    assert_eq!(
        bench_alloc::delta_since(mark).allocations,
        0,
        "caller-buffer path allocated"
    );
}

struct Corpus {
    codec: ProtocolCodec,
    full_frames: Vec<Vec<u8>>,
    transaction_edits: Vec<SessionEdit>,
    non_ok: NonOkResponse,
    frame_output: Vec<u8>,
    decode_fields: [u16; 1024],
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
        .map(|index| SessionEdit::SetSessionId {
            session_id: StableId::parse(&format!("audit-{index}")).expect("prepared stable ID"),
        })
        .collect::<Vec<_>>();
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
    };
    let telemetry = TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let counters = protocol::CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    let control_request = protocol::ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 1,
        owner: None,
        headers_only: false,
    };
    let response_request = protocol::ResponseGetRequest {
        target: protocol::ResponseTarget::Configuration(
            protocol::ResponseConfiguration::InputFilters,
        ),
        grid: protocol::ResponseGrid::Logarithmic,
        points: 64,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: protocol::ParameterChannel::Both,
        sections: false,
        overrides: Vec::new(),
        configuration_id: None,
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    };
    let revision = ExpectedRevision::Exact(SessionRevision(7));
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(1),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::CapabilitiesGet,
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(2),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
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
        payload: CommandPayload::ControlListGet(&control_request),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(5),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::ControlStateGet(&control_request),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(14),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::ResponseGet(&response_request),
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(7),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::TransportGet,
    });
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(8),
        expected_revision: revision,
        payload: CommandPayload::TransportSet(protocol::TransportSetRequest {
            state: protocol::TransportState::Stopped,
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
            minimum_severity: protocol::DiagnosticSeverity::Info,
        }),
    });
    // #1386: the frame names the document by length and digest; the document is not in it.
    corpus.push_command(TypedCommandFrame {
        request_id: request_id(15),
        expected_revision: revision,
        payload: CommandPayload::SessionReplace(protocol::SessionReplaceRequest {
            document_bytes: 1_048_576,
            document_sha256: [0xa5; 32],
        }),
    });
    corpus.push_success(TypedSuccessResponseFrame {
        request_id: request_id(12),
        revision: SessionRevision(7),
        payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied::new(
            64,
            EditPath::Live,
        )),
    });
    corpus.push_event(TypedEventFrame {
        revision: SessionRevision(7),
        payload: EventPayload::SessionCommitted(protocol::SessionCommitted {
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
        parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
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
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(1).expect("event"),
        telemetry_slots: NonZeroUsize::new(1).expect("telemetry"),
        counter_telemetry_slots: NonZeroUsize::new(1).expect("counter telemetry"),
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
        meters: vec![protocol::MeterEntry {
            target: protocol::MeterTarget::Output,
            metric: protocol::MeterMetric::Peak,
        }],
        meter_period_blocks: 1,
        meter_window_blocks: None,
        counter_ids: vec![CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
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
        .try_enqueue_event(protocol::ReliableSlot::session_committed(
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
        .stage_meter_batch_event(
            revision,
            protocol::MeterWindowSpan {
                first_sample: SampleTime(0),
                end_sample: SampleTime(1),
                window_sequence: 1,
            },
            &meters,
        )
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
            payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied::new(
                64,
                EditPath::Live,
            )),
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
            payload: EventPayload::SessionCommitted(protocol::SessionCommitted {
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

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let mut corpus = prepare_corpus();
    let mut egress = prepare_egress_corpus();
    run_audit(&mut corpus, &mut egress);
    println!(
        "protocol caller-buffer audit: ok (command, success, non-OK, event, reliable/meter/counter egress, 64 edits; diagnostic egress is control-plane typed storage)"
    );
}
