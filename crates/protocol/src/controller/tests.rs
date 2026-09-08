use super::*;
use session::{CompileCaps, canonical_session_json, parse_session_json};
use std::num::NonZeroUsize;

const EXAMPLE: &str = include_str!("../../../../fixtures/session/v1/canonical.json");

fn id(value: u64) -> RequestId {
    RequestId::new(value).expect("nonzero")
}

#[test]
fn replay_resource_projection_is_bounded_and_overflow_checked() {
    let config = ReplayCacheConfig {
        entries: NonZeroUsize::new(4).expect("entries"),
        bytes: NonZeroUsize::new(1_024).expect("bytes"),
        max_response_bytes: 256,
    };
    let report = ReplayCache::resource_report_for_config(config).expect("projection");
    #[cfg(target_pointer_width = "64")]
    assert_eq!(report.retained_payload_bytes, 1_248);
    assert_eq!(report.largest_allocation_bytes, 1_024);
    let overflow = ReplayCacheConfig {
        entries: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
        ..config
    };
    assert_eq!(
        ReplayCache::resource_report_for_config(overflow),
        Err(ReplayCacheError::ResourceOverflow)
    );
}

#[test]
#[cfg(target_pointer_width = "64")]
fn replay_layout_stays_within_the_capi_resource_oracle() {
    assert_eq!(core::mem::size_of::<ReplayEntry>(), 56);
    assert_eq!(core::mem::size_of::<ReplayCache>(), 88);
    // #241 re-pin (-24): deleting three source/limit edit variants narrows the embedded
    // prepared-command enum by 24 bytes; all twelve queue endpoints are otherwise unchanged.
    assert_eq!(
        core::mem::size_of::<ProtocolController<MockProvider>>(),
        6_064
    );
    assert_eq!(core::mem::size_of::<PreparedStructuralCommand>(), 752);
}

#[test]
fn replay_prefix_derives_length_retained_bytes_and_highest_pending_id() {
    let mut cache = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(2).expect("two entries"),
        bytes: NonZeroUsize::new(8).expect("eight bytes"),
        max_response_bytes: 3,
    });
    assert_eq!((cache.len(), cache.retained_bytes()), (0, 0));
    assert_eq!(cache.highest_new_id(), None);

    assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::Execute);
    assert_eq!((cache.len(), cache.retained_bytes()), (0, 0));
    assert_eq!(cache.highest_new_id(), Some(id(1)));
    cache.complete(id(1), b"aa", b"x").expect("first");
    assert_eq!((cache.len(), cache.retained_bytes()), (1, 3));
    assert_eq!(cache.highest_new_id(), Some(id(1)));

    assert_eq!(cache.preflight(id(2), b"c"), ReplayDecision::Execute);
    assert_eq!((cache.len(), cache.retained_bytes()), (1, 3));
    assert_eq!(cache.highest_new_id(), Some(id(2)));
    cache.complete(id(2), b"c", b"yy").expect("second");
    let ReplayDecision::Cached(second_hit) = cache.preflight(id(2), b"c") else {
        panic!("second request must hit");
    };
    assert_eq!((cache.len(), cache.retained_bytes()), (2, 6));

    assert_eq!(cache.preflight(id(3), b"d"), ReplayDecision::Execute);
    assert_eq!(
        (cache.len(), cache.retained_bytes(), cache.highest_new_id()),
        (1, 3, Some(id(3))),
        "prefix eviction and pending bytes are derived without retained counters"
    );
    assert_eq!(cache.cached(second_hit), b"yy");
    cache.complete(id(3), b"d", b"zzz").expect("third");
    assert_eq!(
        (cache.len(), cache.retained_bytes(), cache.highest_new_id()),
        (2, 7, Some(id(3)))
    );
    assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::ReplayExpired);
}

#[test]
fn fixed_replay_arena_returns_exact_hits_and_compacts_one_evicted_prefix() {
    let mut cache = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(2).expect("two entries"),
        bytes: NonZeroUsize::new(18).expect("arena bytes"),
        max_response_bytes: 4,
    });
    assert_eq!(cache.retained_storage_capacities(), (2, 18));

    assert_eq!(cache.preflight(id(1), b"one"), ReplayDecision::Execute);
    cache.complete(id(1), b"one", b"1111").expect("first");
    let ReplayDecision::Cached(first_hit) = cache.preflight(id(1), b"one") else {
        panic!("first request must hit");
    };
    assert_eq!(cache.preflight(id(2), b"two"), ReplayDecision::Execute);
    cache.complete(id(2), b"two", b"22").expect("second");

    let ReplayDecision::Cached(second_hit) = cache.preflight(id(2), b"two") else {
        panic!("second request must hit");
    };
    assert_eq!(cache.cached(second_hit), b"22");
    assert_eq!(
        cache.preflight(id(2), b"changed"),
        ReplayDecision::RequestIdReuse
    );

    assert_eq!(cache.preflight(id(3), b"three"), ReplayDecision::Execute);
    cache.complete(id(3), b"three", b"333").expect("third");
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.try_cached(first_hit), None, "evicted hit is stale");
    assert_eq!(cache.cached(first_hit), b"");
    assert_eq!(
        cache.cached(second_hit),
        b"22",
        "surviving hit resolves after compaction"
    );
    assert_eq!(
        cache.preflight(id(1), b"one"),
        ReplayDecision::ReplayExpired
    );
    let ReplayDecision::Cached(compacted_second_hit) = cache.preflight(id(2), b"two") else {
        panic!("second request must survive prefix compaction");
    };
    assert_eq!(cache.cached(compacted_second_hit), b"22");
    let ReplayDecision::Cached(third_hit) = cache.preflight(id(3), b"three") else {
        panic!("third request must hit");
    };
    assert_eq!(cache.cached(third_hit), b"333");
    assert_eq!(cache.retained_storage_capacities(), (2, 18));

    let mut other = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(2).expect("two entries"),
        bytes: NonZeroUsize::new(18).expect("arena bytes"),
        max_response_bytes: 4,
    });
    assert_eq!(other.preflight(id(2), b"two"), ReplayDecision::Execute);
    other
        .complete(id(2), b"two", b"xx")
        .expect("foreign same-ID entry");
    assert_eq!(
        other.try_cached(second_hit),
        None,
        "foreign hit is rejected"
    );
    assert_eq!(other.cached(second_hit), b"");
}

#[test]
fn replay_reservation_binds_exact_request_bytes() {
    let mut cache = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(1).expect("one entry"),
        bytes: NonZeroUsize::new(8).expect("arena bytes"),
        max_response_bytes: 4,
    });
    assert_eq!(cache.preflight(id(1), b"aa"), ReplayDecision::Execute);
    assert_eq!(
        cache.preflight(id(2), b"cc"),
        ReplayDecision::Backpressure,
        "pending reservation cannot be overwritten"
    );
    assert_eq!(
        cache.complete(id(1), b"bb", b"ok"),
        Err(ReplayCacheError::ReservationMissing)
    );
    cache
        .complete(id(1), b"aa", b"ok")
        .expect("exact pending request remains retryable");
    let ReplayDecision::Cached(hit) = cache.preflight(id(1), b"aa") else {
        panic!("exact completed request hits");
    };
    assert_eq!(cache.cached(hit), b"ok");
}

#[test]
fn fixed_replay_arena_reports_response_and_reservation_bounds() {
    let config = ReplayCacheConfig {
        entries: NonZeroUsize::new(1).expect("one entry"),
        bytes: NonZeroUsize::new(8).expect("arena bytes"),
        max_response_bytes: 4,
    };
    let mut cache = ReplayCache::new(config);
    assert_eq!(
        cache.complete(id(1), b"x", b"12345"),
        Err(ReplayCacheError::ResponseTooLarge)
    );
    assert_eq!(
        cache.complete(id(1), b"123456789", b""),
        Err(ReplayCacheError::ReservationMissing)
    );
    assert_eq!(
        cache.preflight(id(1), b"12345"),
        ReplayDecision::Backpressure
    );
    assert!(cache.is_empty());
    assert_eq!(cache.retained_bytes(), 0);
}

fn controller(replay_entries: usize, automation_slots: usize) -> ProtocolController<MockProvider> {
    controller_at_sample(replay_entries, automation_slots, SampleTime(0))
}

fn controller_at_sample(
    replay_entries: usize,
    automation_slots: usize,
    current_sample: SampleTime,
) -> ProtocolController<MockProvider> {
    let session = SessionStore::new(
        parse_session_json(EXAMPLE).expect("fixture"),
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session");
    let queues = ProtocolQueues::prepare(crate::ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(1).expect("one"),
        control_command_bytes: NonZeroUsize::new(64).expect("bytes"),
        automation_batch_slots: NonZeroUsize::new(automation_slots).expect("automation"),
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(1).expect("event"),
        telemetry_slots: NonZeroUsize::new(1).expect("telemetry"),
        per_block_automation_density: NonZeroUsize::new(256).expect("density"),
        quantum_frames: NonZeroUsize::new(1).expect("quantum"),
    })
    .expect("queues");
    ProtocolController::new(
        session,
        queues,
        MockProvider {
            current_sample,
            parameter_metadata: vec![automation_descriptor()],
            parameter_state: ParameterStatePage {
                observed_sample: current_sample.0,
                records: vec![crate::ParameterStateRecord {
                    handle: 1,
                    flags: 1,
                    value: 0.0,
                }],
            },
            counter_snapshot: CounterSnapshot {
                observed_sample: current_sample,
                values: vec![
                    crate::CounterValue {
                        id: crate::CounterId::ControlCommandBackpressure,
                        value: 9,
                    },
                    crate::CounterValue {
                        id: crate::CounterId::TelemetryCoalesced,
                        value: 3,
                    },
                ],
            },
            diagnostics: vec![
                retained_diagnostic(3, crate::DiagnosticSeverity::Warning),
                retained_diagnostic(4, crate::DiagnosticSeverity::Error),
            ],
            ..MockProvider::default()
        },
        ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(replay_entries).expect("entries"),
            bytes: NonZeroUsize::new(4096).expect("bytes"),
            max_response_bytes: 1024,
        }),
    )
}

#[test]
fn frozen_deep_transaction_reaches_public_b1b_process_path() {
    let corpus = conformance::complete_schema_corpus();
    let transaction = corpus
        .iter()
        .find(|frame| frame.name == "command.session_transaction_apply")
        .expect("frozen transaction frame");
    controller(8, 1)
        .process_b1b_btlv(
            &transaction.bytes,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
        )
        .expect("public B1b process path accepts the frozen deep transaction");
}

#[test]
fn public_b1b_uses_exactly_the_typed_reader_passes_and_replays_identical_bytes() {
    fn assert_single_typed_dispatch(frame_name: &str) {
        let corpus = conformance::complete_schema_corpus();
        let frame = corpus
            .iter()
            .find(|frame| frame.name == frame_name)
            .expect("frozen command frame");
        let codec = ProtocolCodec::default();

        crate::btlv::reset_reader_passes();
        codec
            .decode_typed_command(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
            .expect("typed baseline decode");
        let typed_reader_passes = crate::btlv::reader_passes();
        assert!(typed_reader_passes > 0);

        let mut controller = controller(8, 1);
        crate::btlv::reset_reader_passes();
        let first = controller
            .process_b1b_btlv(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
            .expect("single typed dispatch");
        assert_eq!(
            crate::btlv::reader_passes(),
            typed_reader_passes,
            "controller must add no generic structural walk for {frame_name}"
        );

        let replay = controller
            .process_b1b_btlv(&frame.bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
            .expect("exact canonical replay");
        assert_eq!(
            replay, first,
            "status and exact bytes replay for {frame_name}"
        );
    }

    assert_single_typed_dispatch("command.transport_get");
    assert_single_typed_dispatch("command.session_transaction_apply");

    let corpus = conformance::complete_schema_corpus();
    let transaction = corpus
        .iter()
        .find(|frame| frame.name == "command.session_transaction_apply")
        .expect("frozen transaction frame");
    let mut limited = controller(8, 1);
    // One below the fixture's own edit count: #241 took the corpus from 42 edits to 39 by
    // deleting opcodes 0x0006/0x0102/0x0104, so the boundary this row exists to probe moved
    // 41 -> 38. A smaller number still refuses, but it stops being a boundary.
    assert_eq!(conformance::complete_all_opcode_fixture().len(), 39);
    limited.config.maximum_transaction_edits = 38;
    assert_eq!(
        limited.process_b1b_btlv(
            &transaction.bytes,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
        ),
        Err(DecodeError::LimitExceeded)
    );
    assert_eq!(limited.session().revision(), SessionRevision(7));
    assert!(limited.replay.is_empty());
}

#[test]
fn public_b1b_automation_replay_uses_the_ordinary_queue() {
    let records = [crate::AutomationRecord {
        kind: crate::AutomationKind::Point,
        handle: ParameterHandle(1),
        start: SampleTime(0),
        end: SampleTime(0),
        start_value: 0.25,
        end_value: 0.25,
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue { records: &records }),
    );
    let mut endpoint = controller(8, 2);
    let first = endpoint
        .process_b1b_btlv(&input, &mut DecodeScratch::new(&mut [0_u16; 32]))
        .expect("ordinary B1b automation");
    assert_eq!(first.status, StatusCode::Ok);
    assert_eq!(
        endpoint
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1
    );

    let replay = endpoint
        .process_b1b_btlv(&input, &mut DecodeScratch::new(&mut [0_u16; 32]))
        .expect("ordinary B1b automation replay");
    assert_eq!(replay, first);
    assert_eq!(
        endpoint
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1
    );

    let changed_records = [crate::AutomationRecord {
        start_value: 0.5,
        end_value: 0.5,
        ..records[0]
    }];
    let changed = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue {
            records: &changed_records,
        }),
    );
    assert_eq!(
        endpoint
            .process_b1b_btlv(&changed, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .expect("ordinary B1b changed reuse")
            .status,
        StatusCode::RequestIdReuse
    );
    assert_eq!(
        endpoint
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1
    );
}

fn egress_controller(
    reliable_event_slots: usize,
    telemetry_slots: usize,
) -> ProtocolController<MockProvider> {
    let session = SessionStore::new(
        parse_session_json(EXAMPLE).expect("fixture"),
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session");
    let queues = ProtocolQueues::prepare(crate::ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(1).expect("control"),
        control_command_bytes: NonZeroUsize::new(64).expect("bytes"),
        automation_batch_slots: NonZeroUsize::new(1).expect("automation"),
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(reliable_event_slots).expect("events"),
        telemetry_slots: NonZeroUsize::new(telemetry_slots).expect("telemetry"),
        per_block_automation_density: NonZeroUsize::new(256).expect("density"),
        quantum_frames: NonZeroUsize::new(1).expect("quantum"),
    })
    .expect("queues");
    ProtocolController::new(
        session,
        queues,
        MockProvider::default(),
        ReplayCache::new(ReplayCacheConfig {
            entries: NonZeroUsize::new(8).expect("replay"),
            bytes: NonZeroUsize::new(32 * 1024).expect("bytes"),
            max_response_bytes: 4096,
        }),
    )
}

fn configure_event_egress(
    controller: &mut ProtocolController<MockProvider>,
    request_id: u64,
    meter_handles: Vec<u32>,
    counter_ids: Vec<crate::CounterId>,
    diagnostics_enabled: bool,
) -> TelemetryConfiguration {
    let configuration = TelemetryConfiguration {
        meter_period_blocks: if meter_handles.is_empty() { 0 } else { 1 },
        counter_period_blocks: if counter_ids.is_empty() { 0 } else { 1 },
        meter_handles,
        counter_ids,
        diagnostics_enabled,
        minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
    };
    let response = controller.process(ControllerRequest {
        request_id: id(request_id),
        expected_revision: ExpectedRevision::Exact(controller.session().revision()),
        canonical_bytes: b"typed-event-egress-config",
        command: ControlCommand::TelemetryConfigure {
            configuration: configuration.clone(),
        },
    });
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(controller.telemetry_configuration, configuration);
    configuration
}

fn automation_descriptor() -> crate::ParameterDescriptor {
    crate::ParameterDescriptor {
        handle: 1,
        track_id: "vocal".to_owned(),
        rack: crate::ParameterRack::Dynamic,
        effect_id: "comp".to_owned(),
        parameter_id: 1,
        channel: crate::ParameterChannel::Left,
        value_kind: crate::ParameterValueKind::F32,
        unit: crate::ParameterUnit::Db,
        domain: crate::ParameterDomain::Continuous,
        minimum: Some(-1.0),
        maximum: Some(1.0),
        default: 0.0,
        mapping: crate::ParameterMapping::Linear,
        automation_rate: crate::ParameterAutomationRate::Sample,
        smoothing_samples: 0,
        flags: 3,
        display_name: None,
        display_unit: None,
        enum_choices: Vec::new(),
    }
}

fn retained_diagnostic(sequence: u64, severity: crate::DiagnosticSeverity) -> Diagnostic {
    Diagnostic {
        code: "provider.retained".to_owned(),
        severity,
        path: Vec::new(),
        detail: None,
        operation_index: None,
        sample_time: None,
        provider_sequence: Some(sequence),
    }
}

fn capability<'a>(request_id: u64, bytes: &'a [u8]) -> ControllerRequest<'a> {
    ControllerRequest {
        request_id: id(request_id),
        expected_revision: ExpectedRevision::Any,
        canonical_bytes: bytes,
        command: ControlCommand::CapabilitiesGet,
    }
}

fn batch(request_id: u64, sample: u64) -> AutomationBatchSlot {
    AutomationBatchSlot::new(
        SessionRevision(7),
        id(request_id),
        &[crate::AutomationRecord {
            kind: crate::AutomationKind::Point,
            handle: crate::ParameterHandle(1),
            start: SampleTime(sample),
            end: SampleTime(sample),
            start_value: 1.0,
            end_value: 1.0,
        }],
    )
    .expect("batch")
}

fn full_command(
    request_id: u64,
    expected_revision: ExpectedRevision,
    payload: crate::CommandPayload<'_>,
) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; 16 * 1024];
    let length = codec
        .encode_command_frame_into(
            &crate::TypedCommandFrame {
                request_id: id(request_id),
                expected_revision,
                payload,
            },
            &mut bytes,
        )
        .expect("typed command frame");
    bytes.truncate(length);
    bytes
}

fn process_full_command(
    controller: &mut ProtocolController<MockProvider>,
    input: &[u8],
) -> Vec<u8> {
    let mut output = [0_u8; 2048];
    let mut fields = [0_u16; 1024];
    let length = controller
        .process_command_frame_into(input, &mut DecodeScratch::new(&mut fields), &mut output)
        .expect("full-frame processing");
    output[..length].to_vec()
}

#[test]
fn full_frame_ingress_dispatches_every_registered_command_through_typed_frames() {
    let revision = ExpectedRevision::Exact(SessionRevision(7));
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("framed-session").expect("ID"),
    }];
    let parameter_state = ParameterStateRequest { handles: vec![1] };
    let automation_records = [crate::AutomationRecord {
        kind: crate::AutomationKind::Point,
        handle: ParameterHandle(1),
        start: SampleTime(0),
        end: SampleTime(0),
        start_value: 0.5,
        end_value: 0.5,
    }];
    let telemetry = TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: vec![crate::CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: crate::DiagnosticSeverity::Info,
    };
    let counters = CountersRequest {
        all: true,
        ids: Vec::new(),
    };

    let cases = [
        (
            MessageId::CapabilitiesGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::CapabilitiesGet,
            ),
        ),
        (
            MessageId::SessionSnapshotGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::SessionSnapshotGet(crate::SessionSnapshotRequest {
                    offset: 0,
                    maximum_bytes: 32,
                }),
            ),
        ),
        (
            MessageId::SessionTransactionApply,
            full_command(
                1,
                revision,
                crate::CommandPayload::SessionTransactionApply(&transaction),
            ),
        ),
        (
            MessageId::ParameterMetadataGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::ParameterMetadataGet(ParameterMetadataRequest {
                    after_handle: 0,
                    limit: 1,
                }),
            ),
        ),
        (
            MessageId::ParameterStateGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::ParameterStateGet(&parameter_state),
            ),
        ),
        (
            MessageId::AutomationEnqueue,
            full_command(
                1,
                revision,
                crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue {
                    records: &automation_records,
                }),
            ),
        ),
        (
            MessageId::TransportGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::TransportGet,
            ),
        ),
        (
            MessageId::TransportSet,
            full_command(
                1,
                revision,
                crate::CommandPayload::TransportSet(TransportSetRequest {
                    state: TransportState::Playing,
                    position: Some(SampleTime(48)),
                }),
            ),
        ),
        (
            MessageId::TelemetryConfigure,
            full_command(
                1,
                revision,
                crate::CommandPayload::TelemetryConfigure(&telemetry),
            ),
        ),
        (
            MessageId::CountersGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::CountersGet(&counters),
            ),
        ),
        (
            MessageId::DiagnosticsGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::DiagnosticsGet(DiagnosticsRequest {
                    after_sequence: 0,
                    limit: 1,
                    minimum_severity: crate::DiagnosticSeverity::Info,
                }),
            ),
        ),
    ];

    let codec = ProtocolCodec::default();
    for (message_id, input) in cases {
        let response = process_full_command(&mut controller(8, 2), &input);
        let mut fields = [0_u16; 1024];
        match codec
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("typed full response")
        {
            crate::DecodedTypedResponseFrame::Success { header, .. } => {
                assert_eq!(header.message_id, message_id);
                assert_eq!(header.request_id, id(1));
            }
            crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
                panic!("{message_id:?} unexpectedly returned {:#?}", header.status);
            }
        }
    }
}

#[test]
fn full_frame_ingress_replays_exact_bytes_and_rejects_changed_request_reuse() {
    let records = [crate::AutomationRecord {
        kind: crate::AutomationKind::Point,
        handle: ParameterHandle(1),
        start: SampleTime(0),
        end: SampleTime(0),
        start_value: 0.25,
        end_value: 0.25,
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::AutomationEnqueue(crate::AutomationEnqueue { records: &records }),
    );
    let mut endpoint = controller(8, 2);
    let first = process_full_command(&mut endpoint, &input);
    assert_eq!(
        endpoint
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1
    );
    let replay = process_full_command(&mut endpoint, &input);
    assert_eq!(replay, first);
    assert_eq!(
        endpoint
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1,
        "replay must not enqueue a second batch"
    );

    let changed = full_command(
        1,
        ExpectedRevision::Any,
        crate::CommandPayload::TransportGet,
    );
    let response = process_full_command(&mut endpoint, &changed);
    let mut fields = [0_u16; 32];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("request-reuse response")
    {
        crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
            assert_eq!(header.status, StatusCode::RequestIdReuse);
            assert_eq!(header.message_id, MessageId::TransportGet);
        }
        crate::DecodedTypedResponseFrame::Success { .. } => panic!("changed ID was executed"),
    }
}

#[test]
fn public_immediate_frame_path_never_builds_a_prepared_vec() {
    PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(0));
    let mut endpoint = controller(8, 2);
    let input = full_command(
        1,
        ExpectedRevision::Any,
        crate::CommandPayload::CapabilitiesGet,
    );
    let first = process_full_command(&mut endpoint, &input);
    assert_eq!(
        process_full_command(&mut endpoint, &input),
        first,
        "replay bytes"
    );

    let changed = full_command(
        1,
        ExpectedRevision::Any,
        crate::CommandPayload::TransportGet,
    );
    let _reuse = process_full_command(&mut endpoint, &changed);

    let mut malformed = full_command(
        2,
        ExpectedRevision::Any,
        crate::CommandPayload::CapabilitiesGet,
    );
    malformed[20..24].copy_from_slice(&8_u32.to_le_bytes());
    malformed[40..44].copy_from_slice(&1_u32.to_le_bytes());
    malformed.extend_from_slice(&[1, 0, 1, 0, 1, 0, 0, 0]);
    let _non_ok = process_full_command(&mut endpoint, &malformed);

    PREPARED_IMMEDIATE_CALLS.with(|calls| {
        assert_eq!(
            calls.get(),
            0,
            "public immediate, replay, reuse, and correlatable-error paths write directly"
        );
    });
}

#[test]
fn one_call_transaction_success_and_fallbacks_avoid_owned_preparation() {
    fn status(bytes: &[u8]) -> StatusCode {
        match ProtocolCodec::default()
            .decode_typed_response(bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("typed response")
        {
            crate::DecodedTypedResponseFrame::Success { header, .. }
            | crate::DecodedTypedResponseFrame::NonOk { header, .. } => header.status,
        }
    }

    PREPARED_IMMEDIATE_CALLS.with(|calls| calls.set(0));
    PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
    RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
    let first_edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("direct-transaction").expect("ID"),
    }];
    let first = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&first_edits),
    );
    let mut endpoint = controller(1, 2);
    TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
    crate::typed_frame::reset_frame_writer_passes();
    let committed = process_full_command(&mut endpoint, &first);
    assert_eq!(status(&committed), StatusCode::Ok);
    TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
    assert_eq!(crate::typed_frame::frame_writer_passes(), (1, 1));
    PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 0));
    RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 0));
    let committed_snapshot = endpoint.session().canonical_snapshot().to_owned();
    let committed_events = endpoint.queues().report(crate::QueueKind::ReliableEvent);

    let replay = process_full_command(&mut endpoint, &first);
    assert_eq!(replay, committed, "transaction replay bytes");
    assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
    assert_eq!(
        endpoint.queues().report(crate::QueueKind::ReliableEvent),
        committed_events
    );
    let event = endpoint
        .queues_mut()
        .try_dequeue_event()
        .expect("one committed event");
    assert_eq!(event.message_id, MessageId::SessionCommitted);
    assert!(
        endpoint.queues_mut().try_dequeue_event().is_err(),
        "one-call commit and exact replay emit one event total"
    );

    let changed_edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("changed-reuse").expect("ID"),
    }];
    let changed = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(8)),
        crate::CommandPayload::SessionTransactionApply(&changed_edits),
    );
    assert_eq!(
        status(&process_full_command(&mut endpoint, &changed)),
        StatusCode::RequestIdReuse
    );
    assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| {
        assert_eq!(
            clones.get(),
            0,
            "cached and reuse IDs bypass prospective replay allocation"
        );
    });

    let evict = full_command(
        2,
        ExpectedRevision::Any,
        crate::CommandPayload::TransportGet,
    );
    assert_eq!(
        status(&process_full_command(&mut endpoint, &evict)),
        StatusCode::Ok
    );
    assert_eq!(
        status(&process_full_command(&mut endpoint, &first)),
        StatusCode::ReplayExpired
    );
    assert_eq!(endpoint.session().canonical_snapshot(), committed_snapshot);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| {
        assert_eq!(
            clones.get(),
            0,
            "expired IDs bypass prospective replay allocation"
        );
    });

    let conflict_edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("must-not-commit").expect("ID"),
    }];
    let conflict = full_command(
        3,
        ExpectedRevision::Exact(SessionRevision(6)),
        crate::CommandPayload::SessionTransactionApply(&conflict_edits),
    );
    let mut conflicted = controller(4, 2);
    let before_conflict = conflicted.session().canonical_snapshot().to_owned();
    let clones_before_conflict = PROSPECTIVE_REPLAY_CLONES.with(core::cell::Cell::get);
    let direct_conflict = process_full_command(&mut conflicted, &conflict);
    assert_eq!(status(&direct_conflict), StatusCode::RevisionConflict);
    assert_eq!(conflicted.session().canonical_snapshot(), before_conflict);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| {
        assert_eq!(
            clones.get(),
            clones_before_conflict,
            "header-known semantic fallback bypasses prospective replay allocation"
        );
    });

    let invalid_edits = [SessionEdit::SetSampleRateHz { sample_rate_hz: 0 }];
    let invalid = full_command(
        4,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&invalid_edits),
    );
    let mut invalid_endpoint = controller(4, 2);
    let invalid_snapshot = invalid_endpoint.session().canonical_snapshot().to_owned();
    TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
    assert_eq!(
        status(&process_full_command(&mut invalid_endpoint, &invalid)),
        StatusCode::ValidationFailed
    );
    TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
    assert_eq!(
        invalid_endpoint.session().canonical_snapshot(),
        invalid_snapshot
    );
    PROSPECTIVE_REPLAY_CLONES.with(|clones| {
        assert_eq!(
            clones.get(),
            clones_before_conflict,
            "session-prepare fallback bypasses prospective replay allocation"
        );
    });

    let mut prepared_compat = controller(4, 2);
    let prepared = prepared_compat
        .prepare_command_frame(&conflict, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
        .expect("public owned fallback");
    let PreparedCommandFrame::Immediate(prepared) = prepared else {
        panic!("revision conflict must not produce a structural token");
    };
    let mut prepared_bytes = [0_u8; 2048];
    let prepared_len = prepared
        .write_into(&mut prepared_bytes)
        .expect("prepared compatibility bytes");
    assert_eq!(&prepared_bytes[..prepared_len], direct_conflict);

    let mut pressured = controller(4, 2);
    pressured.replay = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(1).expect("entry"),
        bytes: NonZeroUsize::new(64).expect("arena"),
        max_response_bytes: 32,
    });
    let before_pressure = pressured.session().canonical_snapshot().to_owned();
    assert_eq!(
        status(&process_full_command(&mut pressured, &first)),
        StatusCode::Backpressure
    );
    assert_eq!(pressured.session().canonical_snapshot(), before_pressure);
    assert!(pressured.replay.is_empty());
    PROSPECTIVE_REPLAY_CLONES.with(|clones| {
        assert_eq!(
            clones.get(),
            clones_before_conflict,
            "replay-capacity backpressure bypasses prospective replay allocation"
        );
    });

    PREPARED_IMMEDIATE_CALLS.with(|calls| {
        assert_eq!(
            calls.get(),
            1,
            "only the explicit public prepare fallback may build owned immediate bytes"
        );
    });
}

#[test]
fn full_frame_ingress_preserves_output_ownership_and_correlates_payload_errors() {
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("not-committed").expect("ID"),
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );
    let mut endpoint = controller(8, 2);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
    RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
    let mut short = [0xa5_u8; 32];
    let mut fields = [0_u16; 16];
    assert_eq!(
        endpoint.process_command_frame_into(
            &input,
            &mut DecodeScratch::new(&mut fields),
            &mut short,
        ),
        Err(CommandFrameProcessError::OutputReservationTooSmall { required: 1024 })
    );
    assert_eq!(short, [0xa5; 32]);
    assert_eq!(endpoint.session().revision(), SessionRevision(7));
    assert!(endpoint.replay().is_empty());
    assert_eq!(
        endpoint
            .queues()
            .report(crate::QueueKind::ReliableEvent)
            .occupancy,
        0
    );
    PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 0));
    RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 0));

    let before_malformed_transaction = endpoint.session().canonical_snapshot().to_owned();
    let mut malformed_transaction = input.clone();
    malformed_transaction[crate::OUTER_HEADER_BYTES + 2] = 1;
    TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
    let transaction_response = process_full_command(&mut endpoint, &malformed_transaction);
    TYPED_COMMAND_DECODES.with(|decodes| {
        assert_eq!(
            decodes.get(),
            1,
            "a malformed transaction carries its typed-decode error into the direct outcome path"
        );
    });
    let mut transaction_response_fields = [0_u16; 32];
    match ProtocolCodec::default()
        .decode_typed_response(
            &transaction_response,
            &mut DecodeScratch::new(&mut transaction_response_fields),
        )
        .expect("correlatable malformed transaction response")
    {
        crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
            assert_eq!(header.request_id, id(1));
            assert_eq!(header.message_id, MessageId::SessionTransactionApply);
            assert_eq!(header.status, StatusCode::MalformedFrame);
        }
        crate::DecodedTypedResponseFrame::Success { .. } => {
            panic!("malformed transaction payload succeeded")
        }
    }
    assert_eq!(
        endpoint.session().canonical_snapshot(),
        before_malformed_transaction
    );
    assert_eq!(
        endpoint
            .queues()
            .report(crate::QueueKind::ReliableEvent)
            .occupancy,
        0
    );

    let mut malformed = full_command(
        2,
        ExpectedRevision::Any,
        crate::CommandPayload::CapabilitiesGet,
    );
    malformed[20..24].copy_from_slice(&8_u32.to_le_bytes());
    malformed[40..44].copy_from_slice(&1_u32.to_le_bytes());
    malformed.extend_from_slice(&[1, 0, 1, 0, 1, 0, 0, 0]);
    TYPED_COMMAND_DECODES.with(|decodes| decodes.set(0));
    let response = process_full_command(&mut endpoint, &malformed);
    TYPED_COMMAND_DECODES.with(|decodes| assert_eq!(decodes.get(), 1));
    let mut response_fields = [0_u16; 32];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut response_fields))
        .expect("correlatable error response")
    {
        crate::DecodedTypedResponseFrame::NonOk { header, .. } => {
            assert_eq!(header.request_id, id(2));
            assert_eq!(header.message_id, MessageId::CapabilitiesGet);
            assert_eq!(header.status, StatusCode::MalformedFrame);
        }
        crate::DecodedTypedResponseFrame::Success { .. } => {
            panic!("malformed payload succeeded")
        }
    }

    let mut uncorrelatable = malformed;
    uncorrelatable[0] ^= 1;
    let mut output = [0x5a_u8; 2048];
    assert_eq!(
        endpoint.process_command_frame_into(
            &uncorrelatable,
            &mut DecodeScratch::new(&mut [0_u16; 32]),
            &mut output,
        ),
        Err(CommandFrameProcessError::Uncorrelatable(
            DecodeError::BadMagic
        ))
    );
    assert_eq!(output, [0x5a; 2048]);

    let mut owned = [0x3c_u8; 2048];
    let mut owned_fields = [0_u16; 32];
    let length = endpoint
        .process_command_frame_into(
            &full_command(
                3,
                ExpectedRevision::Any,
                crate::CommandPayload::TransportGet,
            ),
            &mut DecodeScratch::new(&mut owned_fields),
            &mut owned,
        )
        .expect("owned response");
    assert!(owned[..length].iter().any(|byte| *byte != 0x3c));
    assert!(owned[length..].iter().all(|byte| *byte == 0x3c));
}

#[test]
fn structural_prepare_is_invisible_until_commit_and_byte_identical_to_one_call() {
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("prepared-session").expect("ID"),
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );

    let mut accepted = controller(8, 2);
    let mut accepted_output = [0_u8; 2048];
    let accepted_len = accepted
        .process_command_frame_into(
            &input,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut accepted_output,
        )
        .expect("accepted one-call path");
    let accepted_event = accepted.queues_mut().try_dequeue_event().expect("event");

    let mut endpoint = controller(8, 2);
    let before_snapshot = endpoint.session().canonical_snapshot().to_owned();
    let before_event = endpoint
        .queues_mut()
        .report(crate::QueueKind::ReliableEvent);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
    RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
    let prepared = match endpoint
        .prepare_command_frame(&input, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_) => panic!("valid transaction was immediate"),
    };
    PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 1));
    RESPONSE_STAGING_VECS.with(|allocations| assert_eq!(allocations.get(), 1));
    assert_eq!(endpoint.session().revision(), SessionRevision(7));
    assert_eq!(endpoint.session().canonical_snapshot(), before_snapshot);
    assert!(endpoint.replay().is_empty());
    assert_eq!(
        endpoint.queues().report(crate::QueueKind::ReliableEvent),
        before_event
    );
    assert_eq!(
        prepared.prospective_session().revision(),
        SessionRevision(8)
    );
    assert!(
        prepared
            .prospective_session()
            .compiled()
            .canonical_json()
            .contains("prepared-session")
    );

    let committed = endpoint
        .commit_prepared_structural(prepared)
        .expect("affine commit");
    let mut output = [0xa5_u8; 2048];
    let committed_len = committed.write_into(&mut output).expect("committed bytes");
    assert_eq!(committed_len, accepted_len);
    assert_eq!(&output[..committed_len], &accepted_output[..accepted_len]);
    assert!(output[committed_len..].iter().all(|byte| *byte == 0xa5));
    assert_eq!(endpoint.session().revision(), SessionRevision(8));
    assert_eq!(
        endpoint.queues_mut().try_dequeue_event().expect("event"),
        accepted_event
    );
    assert_eq!(endpoint.replay().len(), 1);
}

#[test]
fn structural_token_cancel_owner_generation_and_serial_rules_are_affine() {
    fn structural(
        endpoint: &mut ProtocolController<MockProvider>,
        input: &[u8],
    ) -> PreparedStructuralCommand {
        match endpoint
            .prepare_command_frame(input, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048)
            .expect("prepare")
        {
            PreparedCommandFrame::Structural(prepared) => *prepared,
            PreparedCommandFrame::Immediate(_) => panic!("expected structural token"),
        }
    }

    let edit = |name: &'static str| {
        [SessionEdit::SetSessionId {
            session_id: session::StableId::parse(name).expect("ID"),
        }]
    };
    let first_edits = edit("token-first");
    let first = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&first_edits),
    );
    let mut owner = controller(8, 2);
    let mut other = controller(8, 2);

    let canceled = structural(&mut owner, &first);
    assert!(matches!(
        owner.prepare_command_frame(&first, &mut DecodeScratch::new(&mut [0_u16; 1024]), 2048,),
        Err(CommandFrameProcessError::PreparedCommandOutstanding)
    ));
    let blocked = owner.process(capability(99, b"must-not-enter-replay"));
    assert_eq!(blocked.status, StatusCode::Backpressure);
    assert!(owner.replay().is_empty());
    assert_eq!(owner.session().revision(), SessionRevision(7));
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = owner.queues_mut();
        }))
        .is_err()
    );
    assert!(matches!(
        owner.enqueue_diagnostic_event(
            SessionRevision(7),
            DiagnosticEvent {
                diagnostic: retained_diagnostic(1, crate::DiagnosticSeverity::Error),
            },
        ),
        Err(EventEgressError::ReliableQueueFull(_))
    ));
    assert!(
        owner
            .cancel_pending_automation(
                AutomationCancellationReason::ExplicitReconfiguration,
                Some(SampleTime(0)),
            )
            .is_err()
    );
    drop(canceled);
    assert_eq!(owner.session().revision(), SessionRevision(7));
    assert!(owner.replay().is_empty());
    owner
        .queues_mut()
        .try_enqueue_event(ReliableSlot::session_committed(
            SessionRevision(7),
            77,
            id(77),
            SessionRevision(6),
            1,
        ))
        .expect("drop released exact reservation");
    let _ = owner
        .queues_mut()
        .try_dequeue_event()
        .expect("clear reservation probe");

    let wrong_owner = structural(&mut owner, &first);
    assert!(matches!(
        other.commit_prepared_structural(wrong_owner),
        Err(PreparedCommandCommitError::WrongController)
    ));
    assert_eq!(owner.session().revision(), SessionRevision(7));
    assert_eq!(other.session().revision(), SessionRevision(7));

    let stale = structural(&mut owner, &first);
    owner.structural_generation.fetch_add(2, Ordering::AcqRel);
    assert!(matches!(
        owner.commit_prepared_structural(stale),
        Err(PreparedCommandCommitError::StaleGeneration)
    ));
    owner.structural_generation.fetch_add(1, Ordering::AcqRel);

    let first_token = structural(&mut owner, &first);
    owner
        .commit_prepared_structural(first_token)
        .expect("first serial commit");
    let _ = owner
        .queues_mut()
        .try_dequeue_event()
        .expect("serial event consumer");
    let second_edits = edit("token-second");
    let second = full_command(
        2,
        ExpectedRevision::Exact(SessionRevision(8)),
        crate::CommandPayload::SessionTransactionApply(&second_edits),
    );
    let second_token = structural(&mut owner, &second);
    owner
        .commit_prepared_structural(second_token)
        .expect("second serial commit");
    assert_eq!(owner.session().revision(), SessionRevision(9));
    assert_eq!(owner.replay().len(), 2);
}

#[test]
fn identical_replay_changed_reuse_eviction_and_expiry_are_deterministic() {
    let mut controller = controller(1, 1);
    let first = controller.process(capability(1, b"first"));
    assert_eq!(first.status, StatusCode::Ok);
    assert_eq!(controller.process(capability(1, b"first")), first);
    assert_eq!(
        controller.process(capability(1, b"changed")).status,
        StatusCode::RequestIdReuse
    );
    assert_eq!(
        controller.process(capability(2, b"second")).status,
        StatusCode::Ok
    );
    assert_eq!(
        controller.process(capability(1, b"first")).status,
        StatusCode::ReplayExpired
    );
    assert_eq!(
        controller.process(capability(1, b"other")).status,
        StatusCode::ReplayExpired
    );
}

#[test]
fn compatibility_response_uses_one_frame_backing_and_cached_header_only_decode() {
    fn assert_shared_payload_backing(response: &ControllerResponse) {
        assert_eq!(
            response.payload().as_ptr(),
            response
                .frame
                .as_ptr()
                .wrapping_add(crate::OUTER_HEADER_BYTES),
            "payload must be a range into the sole complete-frame backing"
        );
        assert_eq!(
            response.frame.len(),
            crate::OUTER_HEADER_BYTES + response.payload_len()
        );
    }

    let mut controller = controller(2, 1);
    let first = controller.process(capability(1, b"single-frame-backing"));
    assert_shared_payload_backing(&first);
    let mut payload = vec![0xa5; first.payload_len()];
    assert_eq!(first.encode_payload(&mut payload), Ok(first.payload_len()));
    assert_eq!(payload, first.payload());

    crate::btlv::reset_reader_passes();
    let cached = controller.process(capability(1, b"single-frame-backing"));
    assert_eq!(
        crate::btlv::reader_passes(),
        0,
        "cached metadata reconstruction must not invoke the recursive TLV Reader"
    );
    assert_eq!(
        cached, first,
        "cache hit preserves exact metadata and bytes"
    );
    assert_shared_payload_backing(&cached);

    let cloned = cached.clone();
    assert_eq!(cloned, cached, "Clone/Eq preserve complete frame bytes");
    assert_shared_payload_backing(&cloned);

    let malformed = ControllerResponse::from_complete_frame(
        id(99),
        StatusCode::Internal,
        SessionRevision(7),
        vec![0xa5; crate::OUTER_HEADER_BYTES - 1],
    );
    assert_eq!(malformed.payload_len(), 0);
    assert_eq!(malformed.payload(), &[]);
    assert_eq!(malformed.encode_payload(&mut []), Ok(0));
}

#[test]
fn mutations_require_exact_revision_and_roll_back_on_validation() {
    let mut controller = controller(4, 1);
    let any = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Any,
        canonical_bytes: b"any",
        command: ControlCommand::SessionTransactionApply { edits: &[] },
    };
    assert_eq!(controller.process(any).status, StatusCode::InvalidField);
    let invalid = ControllerRequest {
        request_id: id(2),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"invalid",
        command: ControlCommand::SessionTransactionApply {
            edits: &[SessionEdit::RemoveSource {
                source_id: session::StableId::parse("voice").expect("id"),
            }],
        },
    };
    assert_eq!(
        controller.process(invalid).status,
        StatusCode::ValidationFailed
    );
    assert_eq!(controller.session().revision(), SessionRevision(7));
}

#[test]
fn session_diagnostic_path_segments_preserve_field_index_and_stable_id_variants() {
    assert_eq!(
        session_path_segment_to_protocol(&session::PathSegment::Field("tracks".to_owned(),)),
        crate::PathSegment::Field("tracks".to_owned())
    );
    assert_eq!(
        session_path_segment_to_protocol(&session::PathSegment::Index(3)),
        crate::PathSegment::Index(3)
    );
    assert_eq!(
        session_path_segment_to_protocol(&session::PathSegment::Id("vocal".to_owned(),)),
        crate::PathSegment::StableId("vocal".to_owned())
    );
}

#[test]
fn validation_diagnostics_preserve_operation_paths_and_exact_omission_count() {
    let mut controller = controller(4, 1);
    controller.config.maximum_response_diagnostics = 1;
    let mut fader = controller.session().compiled().normalized_model().tracks[0]
        .fader
        .clone();
    fader.left_db = f32::NAN;
    fader.right_db = f32::INFINITY;
    let request = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"two-validation-diagnostics",
        command: ControlCommand::SessionTransactionApply {
            edits: &[SessionEdit::SetTrackFader {
                track_id: session::StableId::parse("vocal").expect("stable ID"),
                fader,
            }],
        },
    };

    let response = controller.process(request);
    assert_eq!(response.status, StatusCode::ValidationFailed);
    let codec = ProtocolCodec::default();
    let decoded = codec
        .decode_non_ok_payload(response.payload(), 2)
        .expect("canonical validation error");
    assert_eq!(decoded.omitted_diagnostics, 1);
    assert_eq!(decoded.diagnostics.len(), 1);
    assert_eq!(
        decoded.diagnostics[0],
        Diagnostic {
            code: "numeric.non_finite".to_owned(),
            severity: crate::DiagnosticSeverity::Error,
            path: vec![
                crate::PathSegment::Field("tracks".to_owned()),
                crate::PathSegment::Index(0),
                crate::PathSegment::Field("fader".to_owned()),
                crate::PathSegment::Field("left_db".to_owned()),
            ],
            detail: Some("value must be finite".to_owned()),
            operation_index: Some(1),
            sample_time: None,
            provider_sequence: None,
        }
    );

    let mut encoded = vec![
        0_u8;
        codec
            .encoded_non_ok_payload_len(&decoded)
            .expect("round-trip length")
    ];
    codec
        .encode_non_ok_payload(&decoded, &mut encoded)
        .expect("round-trip encode");
    assert_eq!(
        encoded,
        response.payload(),
        "non-OK payload remains canonical"
    );
}

#[test]
fn edit_rejections_use_typed_operation_diagnostics_not_protocol_failure() {
    let mut controller = controller(4, 1);
    let request = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"missing-source-edit",
        command: ControlCommand::SessionTransactionApply {
            edits: &[SessionEdit::RemoveSource {
                source_id: session::StableId::parse("missing").expect("stable ID"),
            }],
        },
    };
    let response = controller.process(request);
    assert_eq!(response.status, StatusCode::ValidationFailed);
    let decoded = ProtocolCodec::default()
        .decode_non_ok_payload(response.payload(), 2)
        .expect("typed edit error");
    assert_eq!(decoded.omitted_diagnostics, 0);
    assert_eq!(decoded.diagnostics.len(), 1);
    assert_eq!(decoded.diagnostics[0].code, "session.edit.not_found");
    assert_eq!(decoded.diagnostics[0].operation_index, Some(0));
}

#[test]
fn every_non_ok_status_has_a_canonical_common_response_payload() {
    struct Case {
        name: &'static str,
        status: StatusCode,
        top_level_tlvs: u32,
        decoder_error: Option<DecodeError>,
    }

    let cases = [
        Case {
            name: "malformed frame",
            status: StatusCode::MalformedFrame,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::BadMagic),
        },
        Case {
            name: "unsupported version",
            status: StatusCode::UnsupportedVersion,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::UnsupportedVersion),
        },
        Case {
            name: "unsupported message",
            status: StatusCode::UnsupportedMessage,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::UnsupportedMessage),
        },
        Case {
            name: "unknown required field",
            status: StatusCode::UnknownRequiredField,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::UnknownRequiredField),
        },
        Case {
            name: "invalid field",
            status: StatusCode::InvalidField,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "limit exceeded",
            status: StatusCode::LimitExceeded,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::LimitExceeded),
        },
        Case {
            name: "revision conflict",
            status: StatusCode::RevisionConflict,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "revision exhausted",
            status: StatusCode::RevisionExhausted,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "request ID reuse",
            status: StatusCode::RequestIdReuse,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "replay expired",
            status: StatusCode::ReplayExpired,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "backpressure",
            status: StatusCode::Backpressure,
            top_level_tlvs: 3,
            decoder_error: None,
        },
        Case {
            name: "validation failed",
            status: StatusCode::ValidationFailed,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "not found",
            status: StatusCode::NotFound,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "unavailable",
            status: StatusCode::Unavailable,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "time in past",
            status: StatusCode::TimeInPast,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "automation order",
            status: StatusCode::AutomationOrder,
            top_level_tlvs: 2,
            decoder_error: None,
        },
        Case {
            name: "PCM forbidden",
            status: StatusCode::PcmForbidden,
            top_level_tlvs: 2,
            decoder_error: Some(DecodeError::PcmForbidden),
        },
        Case {
            name: "internal",
            status: StatusCode::Internal,
            top_level_tlvs: 2,
            decoder_error: None,
        },
    ];

    let codec = ProtocolCodec::default();
    for (index, case) in cases.iter().enumerate() {
        if let Some(error) = case.decoder_error {
            assert_eq!(error.status(), case.status, "{} decoder mapping", case.name);
        }
        let controller = controller(4, 1);
        let request_id = id(u64::try_from(index).expect("case index fits") + 1);
        let response = controller.compatibility_response(
            MessageId::CapabilitiesGet,
            request_id,
            controller.non_ok(case.status, None),
        );
        assert_eq!(response.status, case.status, "{} status", case.name);
        let decoded = codec
            .decode_non_ok_payload(response.payload(), case.top_level_tlvs)
            .unwrap_or_else(|error| panic!("{} common non-OK payload: {error:?}", case.name));
        assert_eq!(
            decoded.omitted_diagnostics, 0,
            "{} omitted count",
            case.name
        );
        assert_eq!(
            decoded.backpressure.is_some(),
            case.status == StatusCode::Backpressure,
            "{} typed backpressure presence",
            case.name
        );
        let mut canonical = vec![
            0_u8;
            codec
                .encoded_non_ok_payload_len(&decoded)
                .expect("canonical non-OK length")
        ];
        codec
            .encode_non_ok_payload(&decoded, &mut canonical)
            .expect("canonical non-OK encode");
        assert_eq!(
            canonical,
            response.payload(),
            "{} canonical bytes",
            case.name
        );
    }
}

#[test]
fn backpressure_is_cached_without_second_enqueue() {
    let mut controller = controller(4, 1);
    let first = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"first-automation",
        command: ControlCommand::AutomationEnqueue { batch: batch(1, 0) },
    };
    assert_eq!(controller.process(first).status, StatusCode::Ok);
    let second = ControllerRequest {
        request_id: id(2),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"second-automation",
        command: ControlCommand::AutomationEnqueue { batch: batch(2, 1) },
    };
    let rejected = controller.process(second);
    assert_eq!(rejected.status, StatusCode::Backpressure);
    let _ = controller
        .queues_mut()
        .try_dequeue_automation()
        .expect("first remains");
    let retry = ControllerRequest {
        request_id: id(2),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"second-automation",
        command: ControlCommand::AutomationEnqueue { batch: batch(2, 1) },
    };
    assert_eq!(controller.process(retry), rejected);
    assert!(controller.queues_mut().try_dequeue_automation().is_err());
}

#[test]
fn replay_admission_rejects_before_provider_execution() {
    let mut cache = ReplayCache::new(ReplayCacheConfig {
        entries: NonZeroUsize::new(1).expect("one"),
        bytes: NonZeroUsize::new(8).expect("eight"),
        max_response_bytes: 8,
    });
    assert_eq!(cache.preflight(id(1), b"x"), ReplayDecision::Backpressure);
    assert!(cache.is_empty());
    assert_eq!(cache.highest_new_id(), None);
    assert_eq!(cache.preflight(id(1), b"x"), ReplayDecision::Backpressure);
    assert_eq!(cache.highest_new_id(), None);
}

#[test]
fn decoded_btlv_transaction_reaches_same_atomic_session_store() {
    let mut controller = controller(4, 1);
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("renamed").expect("ID"),
    }];
    let frame = crate::SessionTransactionFrame {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&frame)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&frame, &mut bytes)
        .expect("encode");
    let response = controller
        .process_session_transaction_btlv(&codec, &bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
        .expect("decode/process");
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.revision, SessionRevision(8));
    assert_eq!(
        controller
            .session()
            .compiled()
            .normalized_model()
            .session_id
            .as_str(),
        "renamed"
    );
}

#[test]
fn rejected_engine_rates_have_identical_typed_and_btlv_rollback_diagnostics() {
    const DETAIL: &str = "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz";

    fn assert_rejection(
        controller: &mut ProtocolController<MockProvider>,
        response: &ControllerResponse,
        before_revision: SessionRevision,
        before_snapshot: &str,
        before_model: &session::SessionModel,
        before_events: QueueReport,
    ) {
        assert_eq!(response.status, StatusCode::ValidationFailed);
        assert_eq!(response.revision, before_revision);
        let decoded = ProtocolCodec::default()
            .decode_non_ok_payload(response.payload(), 2)
            .expect("typed launch-rate diagnostic");
        assert_eq!(decoded.omitted_diagnostics, 0);
        assert_eq!(decoded.diagnostics.len(), 1);
        let diagnostic = &decoded.diagnostics[0];
        assert_eq!(diagnostic.code, "sample_rate.unsupported_at_launch");
        assert_eq!(
            diagnostic.path,
            [crate::PathSegment::Field("sample_rate_hz".to_owned())]
        );
        assert_eq!(diagnostic.detail.as_deref(), Some(DETAIL));
        assert_eq!(diagnostic.operation_index, Some(1));
        assert_eq!(controller.session().revision(), before_revision);
        assert_eq!(controller.session().canonical_snapshot(), before_snapshot);
        assert_eq!(
            controller.session().compiled().normalized_model(),
            before_model
        );
        assert_eq!(
            controller
                .queues_mut()
                .report(crate::QueueKind::ReliableEvent),
            before_events
        );
    }

    for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001] {
        let edits = [SessionEdit::SetSampleRateHz {
            sample_rate_hz: rate,
        }];

        let mut typed = controller(4, 1);
        let before_revision = typed.session().revision();
        let before_snapshot = typed.session().canonical_snapshot().to_owned();
        let before_model = typed.session().compiled().normalized_model().clone();
        let before_events = typed.queues_mut().report(crate::QueueKind::ReliableEvent);
        let response = typed.process(ControllerRequest {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(before_revision),
            canonical_bytes: b"typed-rejected-engine-rate",
            command: ControlCommand::SessionTransactionApply { edits: &edits },
        });
        assert_rejection(
            &mut typed,
            &response,
            before_revision,
            &before_snapshot,
            &before_model,
            before_events,
        );

        let mut btlv = controller(4, 1);
        let before_revision = btlv.session().revision();
        let before_snapshot = btlv.session().canonical_snapshot().to_owned();
        let before_model = btlv.session().compiled().normalized_model().clone();
        let before_events = btlv.queues_mut().report(crate::QueueKind::ReliableEvent);
        let frame = crate::SessionTransactionFrame {
            request_id: id(1),
            expected_revision: ExpectedRevision::Exact(before_revision),
            edits: &edits,
        };
        let codec = ProtocolCodec::default();
        let mut bytes = vec![
            0;
            codec
                .encoded_session_transaction_len(&frame)
                .expect("transaction length")
        ];
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("transaction encode");
        let response = btlv
            .process_b1b_btlv(&bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
            .expect("BTLV transaction response");
        assert_rejection(
            &mut btlv,
            &response,
            before_revision,
            &before_snapshot,
            &before_model,
            before_events,
        );
    }
}

#[test]
fn decoded_track_edit_reaches_same_atomic_session_store() {
    let mut controller = controller(4, 1);
    let mut fader = controller.session().compiled().normalized_model().tracks[0]
        .fader
        .clone();
    fader.left_db = -2.0;
    let edits = [SessionEdit::SetTrackFader {
        track_id: session::StableId::parse("vocal").expect("ID"),
        fader,
    }];
    let frame = crate::SessionTransactionFrame {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&frame)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&frame, &mut bytes)
        .expect("encode");
    let response = controller
        .process_session_transaction_btlv(&codec, &bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
        .expect("decode/process");
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(
        controller.session().compiled().normalized_model().tracks[0]
            .fader
            .left_db,
        -2.0
    );
}

#[test]
fn decoded_route_edit_reaches_same_atomic_session_store() {
    let mut controller = controller(4, 1);
    let edits = [SessionEdit::SetRouteGainDb {
        route_id: session::StableId::parse("to-main").expect("ID"),
        gain_db: -1.0,
    }];
    let frame = crate::SessionTransactionFrame {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&frame)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&frame, &mut bytes)
        .expect("encode");
    let response = controller
        .process_session_transaction_btlv(&codec, &bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
        .expect("decode/process");
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(
        controller.session().compiled().normalized_model().routes[0].gain_db,
        -1.0
    );
}

#[test]
fn decoded_invalid_automation_rolls_back_after_final_compilation() {
    let mut controller = controller(4, 1);
    let before = controller.session().canonical_snapshot().to_owned();
    let edits = [SessionEdit::SetAutomationSegments {
        automation_id: session::StableId::parse("eq-gain").expect("ID"),
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Exponential,
            start_sample: 12,
            end_sample: 12,
            start_value: 0.0,
            end_value: 0.0,
            unit: session::ParameterUnit::Db,
        }],
    }];
    let frame = crate::SessionTransactionFrame {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&frame)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&frame, &mut bytes)
        .expect("encode");
    let response = controller
        .process_session_transaction_btlv(&codec, &bytes, &mut DecodeScratch::new(&mut [0_u16; 1]))
        .expect("decode/process");
    assert_eq!(response.status, StatusCode::ValidationFailed);
    assert_eq!(controller.session().revision(), SessionRevision(7));
    assert_eq!(controller.session().canonical_snapshot(), before);
}

#[test]
fn endpoint_owned_current_sample_rejects_past_automation_without_client_time() {
    let mut controller = controller_at_sample(4, 1, SampleTime(5));
    let request = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"endpoint-current-sample",
        command: ControlCommand::AutomationEnqueue { batch: batch(1, 4) },
    };
    assert_eq!(controller.process(request).status, StatusCode::TimeInPast);
    assert!(controller.queues_mut().try_dequeue_automation().is_err());
}

#[test]
fn b2b_btlv_automation_uses_header_identity_typed_domains_and_backpressure() {
    fn frame(request_id: u64, sample: u64, value: f32) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let records = [crate::AutomationRecord {
            kind: crate::AutomationKind::Point,
            handle: crate::ParameterHandle(1),
            start: SampleTime(sample),
            end: SampleTime(sample),
            start_value: value,
            end_value: value,
        }];
        let payload_len = codec
            .encoded_automation_enqueue_len(crate::AutomationEnqueue { records: &records })
            .expect("payload length");
        let mut result = vec![0_u8; crate::OUTER_HEADER_BYTES + payload_len];
        codec
            .write_outer_header(
                &mut result,
                crate::FrameKind::Command,
                MessageId::AutomationEnqueue,
                StatusCode::Ok,
                request_id,
                7,
                0,
                payload_len as u32,
                3,
            )
            .expect("header");
        codec
            .encode_automation_enqueue(
                crate::AutomationEnqueue { records: &records },
                &mut result[crate::OUTER_HEADER_BYTES..],
            )
            .expect("payload");
        result
    }

    let mut controller = controller(8, 1);
    let first = frame(1, 0, 1.0);
    let response = controller
        .process_b1b_btlv(&first, &mut DecodeScratch::new(&mut [0_u16; 3]))
        .expect("BTLV admission");
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(
        ProtocolCodec::default()
            .decode_automation_enqueued(response.payload(), 4)
            .expect("success payload"),
        AutomationEnqueued {
            accepted_records: 1,
            occupancy: 1,
            capacity: 1,
            generation: 2,
        }
    );
    let full = frame(2, 1, 1.0);
    let response = controller
        .process_b1b_btlv(&full, &mut DecodeScratch::new(&mut [0_u16; 3]))
        .expect("BTLV backpressure");
    assert_eq!(response.status, StatusCode::Backpressure);
    assert_eq!(
        ProtocolCodec::default()
            .decode_non_ok_payload(response.payload(), 2)
            .expect("typed backpressure")
            .backpressure
            .expect("backpressure"),
        crate::Backpressure {
            queue_kind: crate::BackpressureQueueKind::Automation,
            capacity: 1,
            occupancy: 1,
            requested_items: 1,
            generation: Some(2),
            retry_boundary: None,
            requested_bytes: None,
            available_bytes: None,
        }
    );
    let _ = controller
        .queues_mut()
        .try_dequeue_automation()
        .expect("first batch");
    let invalid_domain = frame(3, 2, 2.0);
    assert_eq!(
        controller
            .process_b1b_btlv(&invalid_domain, &mut DecodeScratch::new(&mut [0_u16; 3]))
            .expect("domain response")
            .status,
        StatusCode::InvalidField
    );
    let mut past = controller_at_sample(8, 1, SampleTime(5));
    assert_eq!(
        past.process_b1b_btlv(&frame(1, 4, 1.0), &mut DecodeScratch::new(&mut [0_u16; 3]))
            .expect("past response")
            .status,
        StatusCode::TimeInPast
    );
}

#[test]
fn b3a_transport_btlv_is_typed_idempotent_and_emits_reliable_state() {
    fn set_frame(
        request_id: u64,
        revision: ExpectedRevision,
        request: TransportSetRequest,
    ) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let payload_len = codec.encoded_transport_set_request_len(request);
        let mut frame = vec![0_u8; crate::OUTER_HEADER_BYTES + payload_len];
        let (wire_revision, flags) = match revision {
            ExpectedRevision::Exact(value) => (value.0, 0),
            ExpectedRevision::Any => (0, 1),
        };
        codec
            .write_outer_header(
                &mut frame,
                crate::FrameKind::Command,
                MessageId::TransportSet,
                StatusCode::Ok,
                request_id,
                wire_revision,
                flags,
                payload_len as u32,
                if request.position.is_some() { 2 } else { 1 },
            )
            .expect("header");
        codec
            .encode_transport_set_request(request, &mut frame[crate::OUTER_HEADER_BYTES..])
            .expect("payload");
        frame
    }
    fn hex(bytes: &[u8]) -> String {
        engine::hex_lower(bytes)
    }

    let codec = ProtocolCodec::default();
    let get = crate::Frame::Command(crate::CommandFrame {
        request_id: id(1),
        expected_revision: ExpectedRevision::Any,
        message_id: MessageId::TransportGet,
    });
    let mut get_frame = [0_u8; crate::OUTER_HEADER_BYTES];
    codec.encode(&get, &mut get_frame).expect("get frame");
    assert_eq!(
        hex(&get_frame),
        "4d49534f43544c0001000000300001010700000000000000010000000000000000000000000000000000000000000000"
    );
    let set = TransportSetRequest {
        state: TransportState::Playing,
        position: Some(SampleTime(9)),
    };
    let set_bytes = set_frame(2, ExpectedRevision::Exact(SessionRevision(7)), set);
    let mut controller = controller(8, 1);
    let get_response = controller
        .process_b1b_btlv(&get_frame, &mut DecodeScratch::new(&mut [0_u16; 0]))
        .expect("typed get");
    assert_eq!(get_response.status, StatusCode::Ok);
    assert_eq!(
        codec
            .decode_transport_snapshot(get_response.payload(), 3)
            .expect("get snapshot"),
        TransportSnapshot {
            state: TransportState::Stopped,
            position: SampleTime(0),
            effective_sample: SampleTime(0),
        }
    );
    let set_response = controller
        .process_b1b_btlv(&set_bytes, &mut DecodeScratch::new(&mut [0_u16; 2]))
        .expect("typed set");
    assert_eq!(set_response.status, StatusCode::Ok);
    assert_eq!(
        codec
            .decode_transport_snapshot(set_response.payload(), 3)
            .expect("set snapshot"),
        TransportSnapshot {
            state: TransportState::Playing,
            position: SampleTime(9),
            effective_sample: SampleTime(0),
        }
    );
    let event = controller
        .queues_mut()
        .try_dequeue_event()
        .expect("transport event");
    let mut event_payload = [0_u8; 80];
    assert_eq!(
        controller.encode_transport_state_event(event, &mut event_payload),
        Ok(80)
    );
    assert_eq!(
        codec.decode_transport_state_event(&event_payload, 5),
        Ok(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Playing,
            position: SampleTime(9),
            effective_sample: SampleTime(0),
            origin_request_id: Some(id(2)),
        })
    );
    let mut event_frame = [0_u8; crate::OUTER_HEADER_BYTES + 80];
    codec
        .write_outer_header(
            &mut event_frame,
            crate::FrameKind::Event,
            MessageId::TransportState,
            StatusCode::Ok,
            0,
            7,
            0,
            80,
            5,
        )
        .expect("event header");
    event_frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(&event_payload);
    assert_eq!(
        hex(&event_frame),
        concat!(
            "4d49534f43544c0001000000300003001080000050000000",
            "000000000000000007000000000000000500000000000000",
            "01000401080000000100000000000000",
            "02000101010000000200000000000000",
            "03000401080000000900000000000000",
            "04000401080000000000000000000000",
            "05000400080000000200000000000000"
        )
    );
    let retain = TransportSetRequest {
        state: TransportState::Stopped,
        position: None,
    };
    let retained = controller
        .process_b1b_btlv(
            &set_frame(3, ExpectedRevision::Exact(SessionRevision(7)), retain),
            &mut DecodeScratch::new(&mut [0_u16; 1]),
        )
        .expect("retain position");
    assert_eq!(
        codec.decode_transport_snapshot(retained.payload(), 3),
        Ok(TransportSnapshot {
            state: TransportState::Stopped,
            position: SampleTime(9),
            effective_sample: SampleTime(0),
        })
    );
    let blocked = controller
        .process_b1b_btlv(
            &set_frame(4, ExpectedRevision::Exact(SessionRevision(7)), set),
            &mut DecodeScratch::new(&mut [0_u16; 2]),
        )
        .expect("event capacity response");
    assert_eq!(blocked.status, StatusCode::Backpressure);
    assert_eq!(
        controller.provider().transport_state,
        TransportState::Stopped,
        "a full reliable-event queue must leave the typed provider unchanged"
    );
    assert_eq!(controller.provider().transport_position, SampleTime(9));
    let invalid_revision = set_frame(5, ExpectedRevision::Any, set);
    assert_eq!(
        controller
            .process_b1b_btlv(&invalid_revision, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("revision response")
            .status,
        StatusCode::InvalidField
    );
}

#[test]
fn transport_state_only_preserves_automation_and_locate_starts_new_ordering_epoch() {
    let mut controller = egress_controller(3, 1);
    controller
        .queues_mut()
        .try_enqueue_automation(SampleTime(0), batch(9, 100))
        .expect("queued automation");
    let state_only = controller.process(ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"state-only-preserves-automation",
        command: ControlCommand::TransportSet {
            request: TransportSetRequest {
                state: TransportState::Playing,
                position: None,
            },
        },
    });
    assert_eq!(state_only.status, StatusCode::Ok);
    assert_eq!(
        controller
            .queues_mut()
            .report(crate::QueueKind::Automation)
            .occupancy,
        1
    );
    let _ = controller
        .queues_mut()
        .try_dequeue_event()
        .expect("state event");

    let locate = controller.process(ControllerRequest {
        request_id: id(2),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"locate-cancels-automation",
        command: ControlCommand::TransportSet {
            request: TransportSetRequest {
                state: TransportState::Playing,
                position: Some(SampleTime(10)),
            },
        },
    });
    assert_eq!(locate.status, StatusCode::Ok);
    assert!(controller.queues_mut().try_dequeue_automation().is_err());
    controller
        .queues_mut()
        .try_enqueue_automation(SampleTime(0), batch(10, 50))
        .expect("locate reset permits an earlier absolute scheduling epoch");
}

#[test]
fn b3b1_btlv_telemetry_echo_and_counters_are_typed_and_nondestructive() {
    fn frame(
        message_id: MessageId,
        request_id: u64,
        revision: ExpectedRevision,
        payload: &[u8],
        count: u32,
    ) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + payload.len()];
        let (wire_revision, flags) = match revision {
            ExpectedRevision::Any => (0, 1),
            ExpectedRevision::Exact(revision) => (revision.0, 0),
        };
        codec
            .write_outer_header(
                &mut bytes,
                crate::FrameKind::Command,
                message_id,
                StatusCode::Ok,
                request_id,
                wire_revision,
                flags,
                payload.len() as u32,
                count,
            )
            .expect("header");
        bytes[crate::OUTER_HEADER_BYTES..].copy_from_slice(payload);
        bytes
    }
    let codec = ProtocolCodec::default();
    let configuration = TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 4,
        counter_ids: vec![crate::CounterId::ControlCommandBackpressure],
        counter_period_blocks: 8,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: crate::DiagnosticSeverity::Warning,
    };
    let mut config_payload = vec![
        0;
        codec
            .encoded_telemetry_configuration_len(&configuration)
            .expect("config length")
    ];
    codec
        .encode_telemetry_configuration(&configuration, &mut config_payload)
        .expect("config");
    let mut controller = controller(8, 1);
    let first = controller
        .process_b1b_btlv(
            &frame(
                MessageId::TelemetryConfigure,
                1,
                ExpectedRevision::Exact(SessionRevision(7)),
                &config_payload,
                6,
            ),
            &mut DecodeScratch::new(&mut [0_u16; 6]),
        )
        .expect("configure");
    assert_eq!(first.status, StatusCode::Ok);
    assert_eq!(
        codec.decode_telemetry_configuration(first.payload(), 6),
        Ok(configuration.clone())
    );
    assert_eq!(controller.provider().telemetry_configuration, configuration);
    let second = controller
        .process_b1b_btlv(
            &frame(
                MessageId::TelemetryConfigure,
                2,
                ExpectedRevision::Exact(SessionRevision(7)),
                &config_payload,
                6,
            ),
            &mut DecodeScratch::new(&mut [0_u16; 6]),
        )
        .expect("idempotent configure");
    assert_eq!(second.payload(), first.payload());

    let request = CountersRequest {
        all: false,
        ids: vec![1, 5],
    };
    let mut request_payload = [0_u8; 32];
    codec
        .encode_counters_request(&request, &mut request_payload)
        .expect("counter request");
    let first = controller
        .process_b1b_btlv(
            &frame(
                MessageId::CountersGet,
                3,
                ExpectedRevision::Any,
                &request_payload,
                2,
            ),
            &mut DecodeScratch::new(&mut [0_u16; 2]),
        )
        .expect("counter read");
    let second = controller
        .process_b1b_btlv(
            &frame(
                MessageId::CountersGet,
                4,
                ExpectedRevision::Any,
                &request_payload,
                2,
            ),
            &mut DecodeScratch::new(&mut [0_u16; 2]),
        )
        .expect("nonreset read");
    assert_eq!(first.payload(), second.payload());
    assert_eq!(
        codec
            .decode_counter_snapshot(first.payload(), 3)
            .expect("snapshot")
            .values
            .len(),
        2
    );
    let unknown = CountersRequest {
        all: false,
        ids: vec![99],
    };
    let mut unknown_payload = [0_u8; 32];
    codec
        .encode_counters_request(&unknown, &mut unknown_payload)
        .expect("unknown selector");
    assert_eq!(
        controller
            .process_b1b_btlv(
                &frame(
                    MessageId::CountersGet,
                    5,
                    ExpectedRevision::Any,
                    &unknown_payload,
                    2
                ),
                &mut DecodeScratch::new(&mut [0_u16; 2]),
            )
            .expect("unknown response")
            .status,
        StatusCode::NotFound
    );
}

#[test]
fn b3b2_diagnostics_btlv_pages_are_typed_nondestructive_and_expire_old_cursors() {
    fn frame(request_id: u64, request: DiagnosticsRequest) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 48];
        codec
            .write_outer_header(
                &mut bytes,
                crate::FrameKind::Command,
                MessageId::DiagnosticsGet,
                StatusCode::Ok,
                request_id,
                0,
                1,
                48,
                3,
            )
            .expect("header");
        codec
            .encode_diagnostics_request(request, &mut bytes[crate::OUTER_HEADER_BYTES..])
            .expect("payload");
        bytes
    }

    let codec = ProtocolCodec::default();
    let mut controller = controller(8, 1);
    let first_request = DiagnosticsRequest {
        after_sequence: 0,
        limit: 1,
        minimum_severity: crate::DiagnosticSeverity::Info,
    };
    let first = controller
        .process_b1b_btlv(
            &frame(1, first_request),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("first page");
    assert_eq!(first.status, StatusCode::Ok);
    assert_eq!(
        codec.decode_diagnostics_page(first.payload(), 3),
        Ok(DiagnosticsPage {
            last_sequence: 3,
            eof: false,
            diagnostics: vec![retained_diagnostic(3, crate::DiagnosticSeverity::Warning)],
        })
    );
    let repeat = controller
        .process_b1b_btlv(
            &frame(2, first_request),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("nondestructive first page");
    assert_eq!(repeat.status, StatusCode::Ok);
    assert_eq!(
        repeat.payload(),
        first.payload(),
        "reads must not drain retained history"
    );

    let final_page = controller
        .process_b1b_btlv(
            &frame(
                3,
                DiagnosticsRequest {
                    after_sequence: 3,
                    limit: 1,
                    minimum_severity: crate::DiagnosticSeverity::Info,
                },
            ),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("final page");
    assert_eq!(
        codec.decode_diagnostics_page(final_page.payload(), 3),
        Ok(DiagnosticsPage {
            last_sequence: 4,
            eof: true,
            diagnostics: vec![retained_diagnostic(4, crate::DiagnosticSeverity::Error)],
        })
    );
    let filtered = controller
        .process_b1b_btlv(
            &frame(
                4,
                DiagnosticsRequest {
                    after_sequence: 0,
                    limit: 1,
                    minimum_severity: crate::DiagnosticSeverity::Error,
                },
            ),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("severity filtered page");
    assert_eq!(filtered.status, StatusCode::Ok);
    assert_eq!(
        codec.decode_diagnostics_page(filtered.payload(), 3),
        Ok(DiagnosticsPage {
            last_sequence: 4,
            eof: true,
            diagnostics: vec![retained_diagnostic(4, crate::DiagnosticSeverity::Error)],
        })
    );

    let expired = controller
        .process_b1b_btlv(
            &frame(
                5,
                DiagnosticsRequest {
                    after_sequence: 1,
                    limit: 1,
                    minimum_severity: crate::DiagnosticSeverity::Info,
                },
            ),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("expired cursor response");
    assert_eq!(expired.status, StatusCode::ReplayExpired);
    assert_eq!(
        codec
            .decode_non_ok_payload(expired.payload(), 2)
            .expect("typed expiration diagnostic")
            .diagnostics,
        vec![Diagnostic {
            code: "diagnostics.cursor_expired".to_owned(),
            severity: crate::DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: None,
        }]
    );

    let empty = controller
        .process_b1b_btlv(
            &frame(
                6,
                DiagnosticsRequest {
                    after_sequence: 4,
                    limit: 1,
                    minimum_severity: crate::DiagnosticSeverity::Info,
                },
            ),
            &mut DecodeScratch::new(&mut [0_u16; 3]),
        )
        .expect("empty page");
    assert_eq!(
        codec.decode_diagnostics_page(empty.payload(), 2),
        Ok(DiagnosticsPage {
            last_sequence: 4,
            eof: true,
            diagnostics: Vec::new(),
        }),
        "an empty page preserves the input cursor"
    );
}

#[test]
fn provider_feature_matrix_derives_capabilities_and_refuses_disabled_dispatch() {
    let codec = ProtocolCodec::default();
    for (features, required_command, required_event, required_bit) in [
        (
            ProviderFeatures {
                parameters: true,
                ..ProviderFeatures::NONE
            },
            4,
            0,
            7,
        ),
        (
            ProviderFeatures {
                transport: true,
                transport_events: true,
                ..ProviderFeatures::NONE
            },
            7,
            0x8010,
            8,
        ),
        (
            ProviderFeatures {
                meters: true,
                ..ProviderFeatures::NONE
            },
            0,
            0x8020,
            9,
        ),
        (
            ProviderFeatures {
                counters: true,
                ..ProviderFeatures::NONE
            },
            10,
            0x8021,
            10,
        ),
        (
            ProviderFeatures {
                diagnostics: true,
                ..ProviderFeatures::NONE
            },
            11,
            0x8030,
            11,
        ),
        (
            ProviderFeatures {
                session_events: true,
                ..ProviderFeatures::NONE
            },
            0,
            0x8001,
            12,
        ),
    ] {
        let mut endpoint = controller(8, 1);
        endpoint.set_provider_features(features);
        let response = endpoint.process(capability(1, b"features"));
        let decoded = codec
            .decode_capabilities(response.payload(), 27)
            .expect("capabilities");
        if required_command != 0 {
            assert!(
                decoded
                    .supported_commands
                    .chunks_exact(2)
                    .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_command)
            );
        }
        if required_event != 0 {
            assert!(
                decoded
                    .supported_events
                    .chunks_exact(2)
                    .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_event)
            );
        }
        assert_ne!(decoded.flags.0 & (1 << required_bit), 0);
    }
    let mut endpoint = controller(8, 1);
    endpoint.set_provider_features(ProviderFeatures::NONE);
    let caps = endpoint.process(capability(1, b"none"));
    let decoded = codec
        .decode_capabilities(caps.payload(), 27)
        .expect("none caps");
    assert_eq!(decoded.flags.0 & !((1 << 7) - 1), 0);
    assert_eq!(
        endpoint
            .process(ControllerRequest {
                request_id: id(2),
                expected_revision: ExpectedRevision::Any,
                canonical_bytes: b"disabled-transport",
                command: ControlCommand::TransportGet
            })
            .status,
        StatusCode::Unavailable
    );
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("disabled-events").expect("id"),
    }];
    assert_eq!(
        endpoint
            .process(ControllerRequest {
                request_id: id(3),
                expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
                canonical_bytes: b"disabled-session-event",
                command: ControlCommand::SessionTransactionApply { edits: &edits }
            })
            .status,
        StatusCode::Unavailable
    );
    assert!(endpoint.queues_mut().try_dequeue_event().is_err());

    let mut endpoint = controller(8, 1);
    endpoint.config.maximum_transaction_edits = 0;
    let caps = endpoint.process(capability(1, b"zero-edit-limit"));
    let decoded = codec
        .decode_capabilities(caps.payload(), 27)
        .expect("zero-edit capabilities");
    assert_eq!(decoded.maximum_transaction_edits, 0);
    assert!(
        !decoded
            .supported_commands
            .chunks_exact(2)
            .any(|value| u16::from_le_bytes([value[0], value[1]]) == 3)
    );
    assert!(
        !decoded
            .supported_events
            .chunks_exact(2)
            .any(|value| { matches!(u16::from_le_bytes([value[0], value[1]]), 0x8001 | 0x8002) })
    );
    assert_eq!(decoded.flags.0 & ((1 << 3) | (1 << 12)), 0);
    assert_eq!(
        endpoint
            .process(ControllerRequest {
                request_id: id(2),
                expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
                canonical_bytes: b"zero-edit-disabled",
                command: ControlCommand::SessionTransactionApply { edits: &edits },
            })
            .status,
        StatusCode::Unavailable
    );
}

#[test]
fn session_commit_reserves_reliable_event_before_replacing_store() {
    let mut controller = controller(4, 1);
    let before_snapshot = controller.session().canonical_snapshot().to_owned();
    controller
        .queues_mut()
        .try_enqueue_event(ReliableSlot::session_committed(
            SessionRevision(7),
            99,
            id(99),
            SessionRevision(6),
            1,
        ))
        .expect("fill prepared event queue");
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("blocked-commit").expect("ID"),
    }];
    let response = controller.process(ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"event-capacity-before-commit",
        command: ControlCommand::SessionTransactionApply { edits: &edits },
    });
    assert_eq!(response.status, StatusCode::Backpressure);
    assert_eq!(controller.session().revision(), SessionRevision(7));
    assert_eq!(controller.session().canonical_snapshot(), before_snapshot);
}

#[test]
fn committed_transaction_emits_typed_zero_header_id_event() {
    let mut controller = controller(4, 1);
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("evented-commit").expect("ID"),
    }];
    let response = controller.process(ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"emit-session-committed",
        command: ControlCommand::SessionTransactionApply { edits: &edits },
    });
    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.revision, SessionRevision(8));
    assert_eq!(
        controller
            .queues_mut()
            .try_dequeue_event()
            .expect("reliable session event"),
        ReliableSlot::session_committed(SessionRevision(8), 1, id(1), SessionRevision(7), 1,)
    );
}

#[test]
fn snapshot_pagination_has_exact_revision_conflict_and_eof_boundaries() {
    let mut controller = controller(4, 1);
    let codec = ProtocolCodec::default();
    let request = |request_id, revision, offset, maximum_bytes| {
        let snapshot = crate::SessionSnapshotRequest {
            offset,
            maximum_bytes,
        };
        let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 32];
        codec
            .encode(
                &crate::Frame::Command(crate::CommandFrame {
                    request_id: id(request_id),
                    expected_revision: revision,
                    message_id: MessageId::SessionSnapshotGet,
                }),
                &mut bytes[..crate::OUTER_HEADER_BYTES],
            )
            .expect("header");
        bytes[20..24].copy_from_slice(&32_u32.to_le_bytes());
        bytes[40..44].copy_from_slice(&2_u32.to_le_bytes());
        codec
            .encode_snapshot_request(snapshot, &mut bytes[crate::OUTER_HEADER_BYTES..])
            .expect("payload");
        bytes
    };
    let first = request(1, ExpectedRevision::Any, 0, 1);
    let first_response = controller
        .process_b1b_btlv(&first, &mut DecodeScratch::new(&mut [0_u16; 2]))
        .expect("first page");
    assert_eq!(first_response.status, StatusCode::Ok);
    let first_page = codec
        .decode_snapshot(first_response.payload(), 4)
        .expect("page");
    assert_eq!(first_page.offset, 0);
    assert!(!first_page.eof);
    let revision = first_response.revision;

    let final_page = request(
        2,
        ExpectedRevision::Exact(revision),
        controller.session().canonical_snapshot().len() as u64,
        1,
    );
    let final_response = controller
        .process_b1b_btlv(&final_page, &mut DecodeScratch::new(&mut [0_u16; 2]))
        .expect("final page");
    let final_snapshot = codec
        .decode_snapshot(final_response.payload(), 4)
        .expect("final payload");
    assert!(final_snapshot.eof);
    assert!(final_snapshot.canonical_json_chunk.is_empty());

    let conflict = request(
        3,
        ExpectedRevision::Exact(SessionRevision(revision.0 - 1)),
        1,
        1,
    );
    assert_eq!(
        controller
            .process_b1b_btlv(&conflict, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("conflict response")
            .status,
        StatusCode::RevisionConflict
    );
}

#[test]
fn canonical_json_snapshots_reparse_before_and_after_commit_across_utf8_split_pages() {
    fn request(
        codec: &ProtocolCodec,
        request_id: u64,
        revision: ExpectedRevision,
        offset: u64,
        maximum_bytes: u32,
    ) -> Vec<u8> {
        let mut bytes = vec![0_u8; crate::OUTER_HEADER_BYTES + 32];
        codec
            .encode(
                &crate::Frame::Command(crate::CommandFrame {
                    request_id: id(request_id),
                    expected_revision: revision,
                    message_id: MessageId::SessionSnapshotGet,
                }),
                &mut bytes[..crate::OUTER_HEADER_BYTES],
            )
            .expect("header");
        bytes[20..24].copy_from_slice(&32_u32.to_le_bytes());
        bytes[40..44].copy_from_slice(&2_u32.to_le_bytes());
        codec
            .encode_snapshot_request(
                crate::SessionSnapshotRequest {
                    offset,
                    maximum_bytes,
                },
                &mut bytes[crate::OUTER_HEADER_BYTES..],
            )
            .expect("snapshot request");
        bytes
    }

    let mut controller = controller(4, 1);
    let codec = ProtocolCodec::default();
    let mut initial_bytes = Vec::new();
    let mut initial_offset = 0_u64;
    let mut request_id = 1_u64;
    loop {
        let encoded = request(
            &codec,
            request_id,
            ExpectedRevision::Exact(SessionRevision(7)),
            initial_offset,
            128,
        );
        let response = controller
            .process_b1b_btlv(&encoded, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("initial snapshot page");
        assert_eq!(response.status, StatusCode::Ok);
        let page = codec
            .decode_snapshot(response.payload(), 4)
            .expect("initial snapshot payload");
        initial_bytes.extend_from_slice(page.canonical_json_chunk);
        initial_offset += page.canonical_json_chunk.len() as u64;
        request_id += 1;
        if page.eof {
            break;
        }
    }
    let initial_text = core::str::from_utf8(&initial_bytes).expect("initial UTF-8");
    let initial_model = parse_session_json(initial_text).expect("initial snapshot reparses");
    assert_eq!(
        canonical_session_json(&initial_model).expect("initial canonical"),
        initial_text
    );

    let edits = [SessionEdit::SetEffectIdentity {
        track_id: session::StableId::parse("vocal").expect("track ID"),
        rack_name: session::RackName::Dynamic,
        effect_id: session::StableId::parse("eq").expect("effect ID"),
        identity: session::EffectIdentity::ThirdPartyCid {
            cid: "bafy-é-🙂".to_owned(),
        },
    }];
    let applied = controller.process(ControllerRequest {
        request_id: id(request_id),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"unicode-commit",
        command: ControlCommand::SessionTransactionApply { edits: &edits },
    });
    assert_eq!(applied.status, StatusCode::Ok);
    assert_eq!(applied.revision, SessionRevision(8));
    request_id += 1;

    let mut snapshot = Vec::new();
    let mut offset = 0_u64;
    let mut split_utf8 = false;
    loop {
        let encoded = request(
            &codec,
            request_id,
            ExpectedRevision::Exact(SessionRevision(8)),
            offset,
            1,
        );
        let response = controller
            .process_b1b_btlv(&encoded, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("one-byte snapshot page");
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.revision, SessionRevision(8));
        let page = codec
            .decode_snapshot(response.payload(), 4)
            .expect("snapshot page");
        split_utf8 |= !page.canonical_json_chunk.is_empty()
            && core::str::from_utf8(page.canonical_json_chunk).is_err();
        snapshot.extend_from_slice(page.canonical_json_chunk);
        offset += page.canonical_json_chunk.len() as u64;
        request_id += 1;
        if page.eof {
            break;
        }
    }
    assert!(split_utf8, "one-byte paging must split the multibyte CID");
    let snapshot = String::from_utf8(snapshot).expect("reassembled snapshot UTF-8");
    assert_eq!(snapshot, controller.session().canonical_snapshot());
    let model = parse_session_json(&snapshot).expect("committed snapshot reparses");
    assert_eq!(
        canonical_session_json(&model).expect("committed canonical"),
        snapshot
    );
    assert_eq!(model.revision, 8);
}

#[test]
fn c2b2_event_egress_emits_all_six_schema_closed_full_frames() {
    let codec = ProtocolCodec::default();
    let mut controller = egress_controller(4, 256);
    let revision = controller.session().revision();
    let meter_handles = (1_u32..=256).collect::<Vec<_>>();
    let counter_ids = vec![
        crate::CounterId::ControlCommandBackpressure,
        crate::CounterId::TelemetryCoalesced,
    ];
    configure_event_egress(&mut controller, 1, meter_handles, counter_ids, true);

    let committed = ReliableSlot::session_committed(revision, 1, id(9), SessionRevision(6), 2);
    let canceled = ReliableSlot::automation_canceled(
        revision,
        2,
        id(10),
        3,
        AutomationCancellationReason::ExplicitReconfiguration,
        2,
        Some(SampleTime(12)),
    );
    let transport = ReliableSlot::transport_state(
        revision,
        3,
        TransportState::Playing,
        SampleTime(8),
        SampleTime(12),
        Some(id(11)),
    );
    controller
        .queues_mut()
        .try_enqueue_event(committed)
        .expect("committed");
    controller
        .queues_mut()
        .try_enqueue_event(canceled)
        .expect("canceled");
    controller
        .queues_mut()
        .try_enqueue_event(transport)
        .expect("transport");
    let diagnostic = Diagnostic {
        code: "mock.egress".to_owned(),
        severity: crate::DiagnosticSeverity::Warning,
        path: Vec::new(),
        detail: Some("bounded storage".to_owned()),
        operation_index: None,
        sample_time: Some(12),
        provider_sequence: Some(1),
    };
    controller
        .enqueue_diagnostic_event(
            revision,
            DiagnosticEvent {
                diagnostic: diagnostic.clone(),
            },
        )
        .expect("diagnostic");

    let meter_records = (1_u32..=256)
        .map(|handle| MeterRecord {
            handle,
            component: crate::MeterComponent::Left,
            flags: u16::try_from((handle % 7) + 1).expect("registered meter flags"),
            value: handle as f32,
        })
        .collect::<Vec<_>>();
    controller
        .stage_meter_batch_event(revision, SampleTime(24), &meter_records)
        .expect("meters");
    let counters = CounterSnapshot {
        observed_sample: SampleTime(24),
        values: vec![
            CounterValue {
                id: crate::CounterId::ControlCommandBackpressure,
                value: 7,
            },
            CounterValue {
                id: crate::CounterId::TelemetryCoalesced,
                value: 8,
            },
        ],
    };
    controller
        .stage_counter_snapshot_event(revision, &counters)
        .expect("counters");

    let expected = [
        TypedEventFrame {
            revision,
            payload: EventPayload::SessionCommitted(SessionCommitted {
                event_sequence: 1,
                origin_request_id: id(9),
                previous_revision: SessionRevision(6),
                applied_operations: 2,
            }),
        },
        TypedEventFrame {
            revision,
            payload: EventPayload::AutomationCanceled(AutomationCanceled {
                event_sequence: 2,
                origin_request_id: id(10),
                canceled_records: 3,
                reason: AutomationCancellationReason::ExplicitReconfiguration,
                queue_generation: 2,
                effective_sample: Some(SampleTime(12)),
            }),
        },
        TypedEventFrame {
            revision,
            payload: EventPayload::TransportState(TransportStateEvent {
                event_sequence: 3,
                state: TransportState::Playing,
                position: SampleTime(8),
                effective_sample: SampleTime(12),
                origin_request_id: Some(id(11)),
            }),
        },
        TypedEventFrame {
            revision,
            payload: EventPayload::Diagnostic(&diagnostic),
        },
    ];
    for frame in &expected {
        let mut expected_bytes = vec![0_u8; 1024];
        let expected_len = codec
            .encode_event_frame_into(frame, &mut expected_bytes)
            .expect("expected reliable frame");
        let mut actual = vec![0_u8; expected_len];
        assert_eq!(
            controller.dequeue_reliable_event_frame_into(&mut actual),
            Ok(Some(expected_len))
        );
        assert_eq!(actual, expected_bytes[..expected_len]);
        assert!(actual[24..32].iter().all(|byte| *byte == 0));
        assert_eq!(
            codec
                .decode_typed_event(&actual, &mut DecodeScratch::new(&mut [0_u16; 32]))
                .expect("typed reliable event")
                .header
                .revision,
            revision
        );
    }

    let meter_frame = TypedEventFrame {
        revision,
        payload: EventPayload::MeterBatch(MeterBatch {
            observed_sample: SampleTime(24),
            records: &meter_records,
        }),
    };
    let counter_frame = TypedEventFrame {
        revision,
        payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
            observed_sample: SampleTime(24),
            values: &counters.values,
        }),
    };
    for frame in [&meter_frame, &counter_frame] {
        let mut expected_bytes = vec![0_u8; 8192];
        let expected_len = codec
            .encode_event_frame_into(frame, &mut expected_bytes)
            .expect("expected lossy frame");
        let mut actual = vec![0_u8; expected_len];
        assert_eq!(
            controller.dequeue_lossy_event_frame_into(&mut actual),
            Ok(Some(expected_len))
        );
        assert_eq!(actual, expected_bytes[..expected_len]);
        assert!(actual[24..32].iter().all(|byte| *byte == 0));
    }
    assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
}

#[test]
fn counter_egress_splits_nonascending_staged_snapshots_without_stalling() {
    let codec = ProtocolCodec::default();
    let mut controller = egress_controller(1, 4);
    let revision = controller.session().revision();
    let high = crate::CounterId::TelemetryCoalesced;
    let low = crate::CounterId::ControlCommandBackpressure;
    configure_event_egress(&mut controller, 1, Vec::new(), vec![low, high], false);
    for (id, value) in [(high, 5_u64), (low, 1_u64)] {
        controller
            .stage_counter_snapshot_event(
                revision,
                &CounterSnapshot {
                    observed_sample: SampleTime(24),
                    values: vec![CounterValue { id, value }],
                },
            )
            .expect("stage one valid counter snapshot");
    }
    for (id, value) in [(high, 5_u64), (low, 1_u64)] {
        let expected = TypedEventFrame {
            revision,
            payload: EventPayload::CounterSnapshot(CounterSnapshotRef {
                observed_sample: SampleTime(24),
                values: &[CounterValue { id, value }],
            }),
        };
        let mut expected_bytes = vec![0_u8; 256];
        let expected_len = codec
            .encode_event_frame_into(&expected, &mut expected_bytes)
            .expect("expected counter event");
        let mut actual = vec![0_u8; expected_len];
        assert_eq!(
            controller.dequeue_lossy_event_frame_into(&mut actual),
            Ok(Some(expected_len))
        );
        assert_eq!(actual, expected_bytes[..expected_len]);
    }
    assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
}

#[test]
fn c2b2_short_buffers_retain_pending_reliable_order_and_diagnostic_storage() {
    let mut controller = egress_controller(1, 1);
    let revision = controller.session().revision();
    let first = ReliableSlot::session_committed(revision, 1, id(1), SessionRevision(6), 1);
    controller
        .queues_mut()
        .try_enqueue_event(first)
        .expect("first reliable event");
    let mut short = [0xa5_u8; 1];
    assert!(matches!(
        controller.dequeue_reliable_event_frame_into(&mut short),
        Err(EventEgressError::Encode(EncodeError::OutputTooSmall { required })) if required > short.len()
    ));
    assert_eq!(short, [0xa5]);
    let second = ReliableSlot::transport_state(
        revision,
        2,
        TransportState::Stopped,
        SampleTime(0),
        SampleTime(0),
        None,
    );
    controller
        .queues_mut()
        .try_enqueue_event(second)
        .expect("queued after pending first");
    let mut output = [0_u8; 256];
    let first_len = controller
        .dequeue_reliable_event_frame_into(&mut output)
        .expect("first retry")
        .expect("first length");
    assert_eq!(
        ProtocolCodec::default()
            .decode_typed_event(
                &output[..first_len],
                &mut DecodeScratch::new(&mut [0_u16; 8])
            )
            .expect("first event")
            .header
            .message_id,
        MessageId::SessionCommitted
    );
    let second_len = controller
        .dequeue_reliable_event_frame_into(&mut output)
        .expect("second event")
        .expect("second length");
    assert_eq!(
        ProtocolCodec::default()
            .decode_typed_event(
                &output[..second_len],
                &mut DecodeScratch::new(&mut [0_u16; 8])
            )
            .expect("second event")
            .header
            .message_id,
        MessageId::TransportState
    );

    configure_event_egress(&mut controller, 2, Vec::new(), Vec::new(), true);
    let diagnostic = DiagnosticEvent {
        diagnostic: retained_diagnostic(99, crate::DiagnosticSeverity::Info),
    };
    controller
        .enqueue_diagnostic_event(revision, diagnostic.clone())
        .expect("diagnostic stored");
    assert!(matches!(
        controller.dequeue_reliable_event_frame_into(&mut short),
        Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
    ));
    assert_eq!(
        controller.enqueue_diagnostic_event(revision, diagnostic.clone()),
        Err(EventEgressError::DiagnosticStorageFull)
    );
    let _ = controller
        .dequeue_reliable_event_frame_into(&mut output)
        .expect("diagnostic retry");
    controller
        .enqueue_diagnostic_event(revision, diagnostic)
        .expect("storage released only after full frame");

    let mut lossy = egress_controller(1, 1);
    let lossy_revision = lossy.session().revision();
    configure_event_egress(&mut lossy, 1, vec![1], Vec::new(), false);
    lossy
        .stage_meter_batch_event(
            lossy_revision,
            SampleTime(4),
            &[MeterRecord {
                handle: 1,
                component: crate::MeterComponent::Left,
                flags: 3,
                value: 0.5,
            }],
        )
        .expect("meter staging");
    assert!(matches!(
        lossy.dequeue_lossy_event_frame_into(&mut short),
        Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
    ));
    assert_eq!(short, [0xa5]);
    let lossy_len = lossy
        .dequeue_lossy_event_frame_into(&mut output)
        .expect("lossy retry")
        .expect("lossy length");
    assert_eq!(
        ProtocolCodec::default()
            .decode_typed_event(
                &output[..lossy_len],
                &mut DecodeScratch::new(&mut [0_u16; 8])
            )
            .expect("meter event")
            .header
            .message_id,
        MessageId::MeterBatch
    );
}

#[test]
fn c2b2_disabled_configuration_emits_nothing_and_reliable_full_returns_original() {
    let mut controller = egress_controller(1, 1);
    let revision = controller.session().revision();
    assert_eq!(
        controller.stage_meter_batch_event(
            revision,
            SampleTime(0),
            &[MeterRecord {
                handle: 1,
                component: crate::MeterComponent::Left,
                flags: 1,
                value: 0.0,
            }],
        ),
        Err(EventEgressError::Disabled)
    );
    assert_eq!(
        controller.enqueue_diagnostic_event(
            revision,
            DiagnosticEvent {
                diagnostic: retained_diagnostic(1, crate::DiagnosticSeverity::Info),
            },
        ),
        Err(EventEgressError::Disabled)
    );
    assert_eq!(controller.dequeue_lossy_event_frame_into(&mut []), Ok(None));
    let event = ReliableSlot::session_committed(revision, 1, id(1), SessionRevision(6), 1);
    controller
        .queues_mut()
        .try_enqueue_event(event)
        .expect("first reliable");
    assert!(matches!(
        controller.queues_mut().try_enqueue_event(event),
        Err(crate::ReliableEnqueueError { value, .. }) if value == event
    ));
}
