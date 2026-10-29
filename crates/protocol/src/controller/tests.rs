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
    assert!(report.retained_payload_bytes <= 2 * 1_024);
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

fn controller(replay_entries: usize) -> ProtocolController<MockProvider> {
    controller_over(
        parse_session_json(EXAMPLE).expect("fixture"),
        replay_entries,
        SampleTime(0),
    )
}

fn controller_over(
    model: session::SessionModel,
    replay_entries: usize,
    current_sample: SampleTime,
) -> ProtocolController<MockProvider> {
    controller_with_event_slots(model, replay_entries, current_sample, 1)
}

/// [`controller_over`] with `event_slots` slots in the reliable event queue.
fn controller_with_event_slots(
    model: session::SessionModel,
    replay_entries: usize,
    current_sample: SampleTime,
    event_slots: usize,
) -> ProtocolController<MockProvider> {
    let session = SessionStore::new(
        model,
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
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(event_slots).expect("event"),
        telemetry_slots: NonZeroUsize::new(1).expect("telemetry"),
        counter_telemetry_slots: NonZeroUsize::new(1).expect("counter telemetry"),
    })
    .expect("queues");
    ProtocolController::new(
        session,
        queues,
        MockProvider {
            current_sample,
            controls: mock_controls(),
            control_revision: 7,
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

        let mut controller = controller(8);
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
    let mut limited = controller(8);
    // One below the fixture's own edit count: #241 took the corpus from 42 edits to 39 by
    // deleting opcodes 0x0006/0x0102/0x0104, #1094 appended `SetConsole` and
    // `SetTrackConsole`, #1241 the VCA edits and #1553 `SetControl`, and decision 17 (#1561)
    // retired thirteen value ops, and #1678 appended a second `SetControl` (an insert row with an
    // expected effect), and #1584 appended `Observe`, so the boundary this row exists to probe is
    // now 36. A smaller number still refuses, but it stops being a boundary.
    assert_eq!(conformance::complete_all_opcode_fixture().len(), 37);
    limited.config.maximum_transaction_edits = 36;
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
        reliable_response_slots: NonZeroUsize::new(1).expect("response"),
        reliable_event_slots: NonZeroUsize::new(reliable_event_slots).expect("events"),
        telemetry_slots: NonZeroUsize::new(telemetry_slots).expect("telemetry"),
        counter_telemetry_slots: NonZeroUsize::new(telemetry_slots).expect("counter telemetry"),
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

/// Configure counters and diagnostics through `TELEMETRY_CONFIGURE`, then install a meter table of
/// `meters` entries (handles `1..=meters`) beside it. The table is written in place: its entries
/// name strips the fixture's model does not have, and the egress tests read only the handles.
fn configure_event_egress(
    controller: &mut ProtocolController<MockProvider>,
    request_id: u64,
    meters: usize,
    counter_ids: Vec<crate::CounterId>,
    diagnostics_enabled: bool,
) -> TelemetryConfiguration {
    let mut configuration = TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_period_blocks: if counter_ids.is_empty() { 0 } else { 1 },
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
    configuration.meter_window_blocks = Some(core::num::NonZeroU32::MIN);
    assert_eq!(controller.telemetry_configuration, configuration);
    configuration.meters = (0..meters)
        .map(|index| crate::MeterEntry {
            target: crate::MeterTarget::Strip(format!("meter-{index}").into()),
            metric: crate::MeterMetric::Peak,
        })
        .collect();
    configuration.meter_period_blocks = u32::from(meters != 0);
    controller.telemetry_configuration = configuration.clone();
    configuration
}

/// One closed meter window: `[first, end)`, the first window of its configuration.
fn window(first: u64, end: u64) -> crate::MeterWindowSpan {
    crate::MeterWindowSpan {
        first_sample: SampleTime(first),
        end_sample: SampleTime(end),
        window_sequence: 1,
    }
}

/// The handle of the one automatable control row of the fixture list.
const AUTOMATABLE_CONTROL: u32 = 3;

/// The fixture's control list: an owner header, an inserts section header and one automatable
/// continuous row, in ascending handles, so a page carries each kind of row.
fn mock_controls() -> Vec<crate::ControlEntry> {
    let address = |section, slot: Option<&str>, control: Option<u32>| crate::ControlAddress {
        owner_kind: crate::ControlOwnerKind::Strip,
        owner_id: Some("vocal".to_owned()),
        section,
        slot_id: slot.map(str::to_owned),
        control,
        lane: control.map(|_| crate::ParameterChannel::Left),
    };
    vec![
        crate::ControlEntry {
            handle: 1,
            address: address(crate::ControlSection::Own, None, None),
            body: crate::ControlBody::OwnerHeader(crate::ControlOwnerHeader {
                kind: crate::ControlOwnerHeaderKind::Track,
                name: "v".to_owned(),
                route: None,
            }),
        },
        crate::ControlEntry {
            handle: 2,
            address: address(crate::ControlSection::Inserts, Some("comp"), None),
            body: crate::ControlBody::SectionHeader(crate::ControlSectionHeader {
                name: "c".to_owned(),
                effect: None,
            }),
        },
        crate::ControlEntry {
            handle: AUTOMATABLE_CONTROL,
            address: address(crate::ControlSection::Inserts, Some("comp"), Some(1)),
            body: crate::ControlBody::Control(crate::ControlRow {
                name: "t".to_owned(),
                display_name: String::new(),
                display_unit: String::new(),
                unit: crate::ParameterUnit::Db,
                kind: crate::ControlKind::Continuous,
                minimum: -1.0,
                maximum: 1.0,
                disabled_value: None,
                default: 0.0,
                taper: crate::ControlTaper::Linear,
                step: 0.1,
                precision: 1,
                ladder: Vec::new(),
                choices: Vec::new(),
                ramp_row: None,
                default_ramp_samples: 0,
                class: crate::ControlClass::Live,
                reason: None,
                reason_issue: None,
                reopen: None,
                scope: crate::ControlScope::Model,
                automatable: true,
                family: crate::ControlFamily::Effect,
                active: true,
                value: 0.25,
                automated: false,
            }),
        },
    ]
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
    let control_request = ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 8,
        owner: None,
        headers_only: false,
    };
    let telemetry = TelemetryConfiguration {
        meters: vec![crate::MeterEntry {
            target: crate::MeterTarget::Output,
            metric: crate::MeterMetric::Peak,
        }],
        meter_period_blocks: 1,
        meter_window_blocks: None,
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
            MessageId::ControlListGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::ControlListGet(&control_request),
            ),
        ),
        (
            MessageId::ControlStateGet,
            full_command(
                1,
                ExpectedRevision::Any,
                crate::CommandPayload::ControlStateGet(&control_request),
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
        let response = process_full_command(&mut controller(8), &input);
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
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::TransportSet(TransportSetRequest {
            state: TransportState::Playing,
        }),
    );
    let mut endpoint = controller(8);
    let first = process_full_command(&mut endpoint, &input);
    let events = |endpoint: &mut ProtocolController<MockProvider>| {
        endpoint
            .queues_mut()
            .report(crate::QueueKind::ReliableEvent)
            .occupancy
    };
    assert_eq!(events(&mut endpoint), 1, "the set raised its state event");
    let replay = process_full_command(&mut endpoint, &input);
    assert_eq!(replay, first);
    assert_eq!(
        events(&mut endpoint),
        1,
        "replay must not apply the set a second time"
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
    let mut endpoint = controller(8);
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
    let mut endpoint = controller(1);
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
    let mut conflicted = controller(4);
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
    let mut invalid_endpoint = controller(4);
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

    let mut prepared_compat = controller(4);
    let prepared = prepared_compat
        .prepare_command_frame(
            &conflict,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("public owned fallback");
    let PreparedCommandFrame::Immediate(prepared) = prepared else {
        panic!("revision conflict must not produce a structural token");
    };
    let mut prepared_bytes = [0_u8; 2048];
    let prepared_len = prepared
        .write_into(&mut prepared_bytes)
        .expect("prepared compatibility bytes");
    assert_eq!(&prepared_bytes[..prepared_len], direct_conflict);

    let mut pressured = controller(4);
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
    let mut endpoint = controller(8);
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

    let mut accepted = controller(8);
    let mut accepted_output = [0_u8; 2048];
    let accepted_len = accepted
        .process_command_frame_into(
            &input,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut accepted_output,
        )
        .expect("accepted one-call path");
    let accepted_event = accepted.queues_mut().try_dequeue_event().expect("event");

    let mut endpoint = controller(8);
    let before_snapshot = endpoint.session().canonical_snapshot().to_owned();
    let before_event = endpoint
        .queues_mut()
        .report(crate::QueueKind::ReliableEvent);
    PROSPECTIVE_REPLAY_CLONES.with(|clones| clones.set(0));
    RESPONSE_STAGING_VECS.with(|allocations| allocations.set(0));
    let prepared = match endpoint
        .prepare_command_frame(
            &input,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("valid transaction was immediate")
        }
    };
    // The token holds a plan against the committed cache, not a clone of it (#1615).
    PROSPECTIVE_REPLAY_CLONES.with(|clones| assert_eq!(clones.get(), 0));
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
            .expect("a compiled transaction")
            .canonical_json()
            .contains("prepared-session")
    );

    let committed = endpoint
        .commit_prepared_structural(prepared, EditPath::ModelOnly)
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

fn ended_key(strip: &str, tap_id: u32) -> crate::ObservationTapKey {
    crate::ObservationTapKey {
        strip_id: strip.to_owned(),
        section: crate::ObservationSection::Inserts,
        effect_id: "comp".to_owned(),
        tap_id,
    }
}

/// The reliable events of `endpoint`, decoded.
fn drain_events(
    endpoint: &mut ProtocolController<MockProvider>,
) -> Vec<(u64, crate::DecodedEventPayload<'static>)> {
    let codec = ProtocolCodec::default();
    let mut events = Vec::new();
    loop {
        let mut frame = vec![0_u8; 1024];
        let Some(length) = endpoint
            .dequeue_reliable_event_frame_into(&mut frame)
            .expect("an event")
        else {
            return events;
        };
        let mut fields = [0_u16; 64];
        let decoded = codec
            .decode_typed_event(&frame[..length], &mut DecodeScratch::new(&mut fields))
            .expect("a decodable event");
        let payload = match decoded.payload {
            crate::DecodedEventPayload::SessionCommitted(value) => {
                crate::DecodedEventPayload::SessionCommitted(value)
            }
            crate::DecodedEventPayload::ObservationEnded(value) => {
                crate::DecodedEventPayload::ObservationEnded(value)
            }
            _ => panic!("an unexpected event"),
        };
        events.push((decoded.header.revision.0, payload));
    }
}

/// #1584 D6a fold (root ruling A): the keys a host attaches to a prepared transaction travel in
/// ONE event, which the commit emits right after `SESSION_COMMITTED`, at the committing revision,
/// with the next endpoint sequence, whatever the number of keys and the number of lane slots.
///
/// Red if the ends are split into several events (the second and later one fails to fit a
/// two-slot lane), precede the commit event, carry the previous revision, drop or reorder a key,
/// or skip or repeat a sequence.
#[test]
fn attached_observation_ends_follow_their_commit_in_one_event() {
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("ending-session").expect("ID"),
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );
    // Two slots: the commit's event and the one event of the ends, however many keys it holds.
    let mut endpoint = controller_with_event_slots(
        parse_session_json(EXAMPLE).expect("fixture"),
        8,
        SampleTime(0),
        2,
    );
    let mut prepared = match endpoint
        .prepare_command_frame(
            &input,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("valid transaction was immediate")
        }
    };
    let keys: Vec<_> = (0..20_u32).map(|tap| ended_key("a", tap)).collect();
    let frame_bytes = endpoint
        .attach_observation_ended(&mut prepared, keys.clone())
        .expect("room for the commit and one event of twenty keys");
    endpoint
        .commit_prepared_structural(prepared, EditPath::Rebuild)
        .expect("commit");
    let mut frame = vec![0_u8; 2048];
    let mut events = Vec::new();
    while let Some(length) = endpoint
        .dequeue_reliable_event_frame_into(&mut frame)
        .expect("an event")
    {
        events.push(frame[..length].to_vec());
    }
    assert_eq!(events.len(), 2, "the commit and one event");
    assert_eq!(events[1].len(), frame_bytes, "the size attach reported");
    let mut fields = [0_u16; 256];
    let codec = ProtocolCodec::default();
    let committed = codec
        .decode_typed_event(&events[0], &mut DecodeScratch::new(&mut fields))
        .expect("commit event");
    let crate::DecodedEventPayload::SessionCommitted(commit) = committed.payload else {
        panic!("the commit event comes first");
    };
    assert_eq!(committed.header.revision.0, 8);
    let ended = codec
        .decode_typed_event(&events[1], &mut DecodeScratch::new(&mut fields))
        .expect("end event");
    assert_eq!(ended.header.revision.0, 8, "the committing revision");
    let crate::DecodedEventPayload::ObservationEnded(ended) = ended.payload else {
        panic!("one end event follows the commit");
    };
    assert_eq!(ended.keys, keys);
    assert_eq!(ended.event_sequence, commit.event_sequence + 1);
    // The next commit's sequence continues after the end.
    let next = full_command(
        2,
        ExpectedRevision::Exact(SessionRevision(8)),
        crate::CommandPayload::SessionTransactionApply(&[SessionEdit::SetSessionId {
            session_id: session::StableId::parse("again-session").expect("ID"),
        }]),
    );
    let mut output = [0_u8; 2048];
    endpoint
        .process_command_frame_into(
            &next,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut output,
        )
        .expect("next commit");
    let events = drain_events(&mut endpoint);
    let crate::DecodedEventPayload::SessionCommitted(second) = &events[0].1 else {
        panic!("commit event");
    };
    assert_eq!(second.event_sequence, commit.event_sequence + 2);
}

/// #1584 D6a fold, the acked-batch question: the room for the one end event is checked together
/// with the commit's own event, and only unread events take it away. A lane holding an unread event
/// refuses the attachment and changes nothing; a token dropped after a successful attachment
/// releases the room it held; an event no frame can carry is refused for good, not as no room.
///
/// Red if an attachment succeeds that the lane cannot honour (the commit would drop the end after
/// the ack), if the number of keys changes the room needed, if a refusal leaves a half
/// reservation, if a dropped token keeps the room, or if an unencodable event reports no room (a
/// retry that can never succeed).
#[test]
fn the_end_needs_room_with_the_commit_and_a_dropped_token_releases_it() {
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("ending-session").expect("ID"),
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );
    // Two slots: the commit and the end fit; an unread event leaves too little.
    let mut endpoint = controller_with_event_slots(
        parse_session_json(EXAMPLE).expect("fixture"),
        8,
        SampleTime(0),
        2,
    );
    let prepare = |endpoint: &mut ProtocolController<MockProvider>| match endpoint
        .prepare_command_frame(
            &input,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("valid transaction was immediate")
        }
    };
    // Empty lane: any number of keys fits, in one event.
    let mut token = prepare(&mut endpoint);
    let many: Vec<_> = (0..100_u32).map(|tap| ended_key("a", tap)).collect();
    endpoint
        .attach_observation_ended(&mut token, many)
        .expect("a hundred keys need one slot");
    // A second attachment replaces the first, so its room counts as free.
    endpoint
        .attach_observation_ended(&mut token, vec![ended_key("c", 1), ended_key("d", 1)])
        .expect("a replacement fits too");
    drop(token);
    // The dropped token released everything: a fresh one can reserve the whole lane again.
    let mut token = prepare(&mut endpoint);
    endpoint
        .attach_observation_ended(&mut token, vec![ended_key("a", 1)])
        .expect("the room came back");
    endpoint
        .commit_prepared_structural(token, EditPath::Rebuild)
        .expect("commit");
    assert_eq!(drain_events(&mut endpoint).len(), 2);

    // One unread event in the lane: the commit takes the other slot and the end has none.
    let next = full_command(
        2,
        ExpectedRevision::Exact(SessionRevision(8)),
        crate::CommandPayload::SessionTransactionApply(&[SessionEdit::SetSessionId {
            session_id: session::StableId::parse("again-session").expect("ID"),
        }]),
    );
    let mut output = [0_u8; 2048];
    endpoint
        .process_command_frame_into(
            &next,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut output,
        )
        .expect("a commit that leaves an unread event");
    let again = full_command(
        3,
        ExpectedRevision::Exact(SessionRevision(9)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );
    let mut token = match endpoint
        .prepare_command_frame(
            &again,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("valid transaction was immediate")
        }
    };
    assert!(matches!(
        endpoint.attach_observation_ended(&mut token, vec![ended_key("a", 1)]),
        Err(crate::AttachObservationEndedError::NoRoom(_))
    ));
    // Out of order keys cannot be encoded: refused for good, with the lane untouched.
    drop(token);
    drain_events(&mut endpoint);
    let mut token = prepare_at(&mut endpoint, 9, &transaction);
    assert!(matches!(
        endpoint.attach_observation_ended(&mut token, vec![ended_key("b", 1), ended_key("a", 1)]),
        Err(crate::AttachObservationEndedError::Unencodable)
    ));
    // So is a list no frame can hold: the controller's frame limit is the codec's.
    let oversized: Vec<_> = (0..200_000_u32).map(|tap| ended_key("a", tap)).collect();
    assert!(matches!(
        endpoint.attach_observation_ended(&mut token, oversized),
        Err(crate::AttachObservationEndedError::Unencodable)
    ));
}

fn prepare_at(
    endpoint: &mut ProtocolController<MockProvider>,
    revision: u64,
    transaction: &[SessionEdit],
) -> PreparedStructuralCommand {
    let input = full_command(
        7,
        ExpectedRevision::Exact(SessionRevision(revision)),
        crate::CommandPayload::SessionTransactionApply(transaction),
    );
    match endpoint
        .prepare_command_frame(
            &input,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("valid transaction was immediate")
        }
    }
}

fn transaction_edit_path(frame: &[u8]) -> EditPath {
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut fields))
        .expect("decodable transaction response")
    {
        crate::DecodedTypedResponseFrame::Success {
            payload: crate::DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
            ..
        } => applied.edit_path,
        _ => panic!("expected a transaction success"),
    }
}

#[test]
fn edit_path_is_written_at_commit_and_replayed_byte_for_byte() {
    let transaction = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("path-session").expect("ID"),
    }];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&transaction),
    );

    // The protocol-only path owns no render plan: the commit changes only the model (#1313 D4).
    let mut direct = controller(8);
    let mut direct_output = [0_u8; 2048];
    let direct_len = direct
        .process_command_frame_into(
            &input,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut direct_output,
        )
        .expect("one-call path");
    assert_eq!(
        transaction_edit_path(&direct_output[..direct_len]),
        EditPath::ModelOnly
    );

    for path in [EditPath::Live, EditPath::ModelOnly, EditPath::Rebuild] {
        let mut endpoint = controller(8);
        let prepared = match endpoint
            .prepare_command_frame(
                &input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare")
        {
            PreparedCommandFrame::Structural(prepared) => *prepared,
            PreparedCommandFrame::Immediate(_)
            | PreparedCommandFrame::Session(_)
            | PreparedCommandFrame::ObservationRead(_) => {
                panic!("valid transaction was immediate")
            }
        };
        let prepared_len = prepared.response_len();
        let committed = endpoint
            .commit_prepared_structural(prepared, path)
            .expect("commit");
        let mut first = [0_u8; 2048];
        let first_len = committed.write_into(&mut first).expect("committed bytes");
        assert_eq!(first_len, prepared_len, "the field is fixed width");
        assert_eq!(transaction_edit_path(&first[..first_len]), path);

        // The replay entry holds the committed frame, not the prepared placeholder.
        let mut replay = [0_u8; 2048];
        let replay_len = endpoint
            .process_command_frame_into(
                &input,
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                &mut replay,
            )
            .expect("replay");
        assert_eq!(&replay[..replay_len], &first[..first_len]);
        assert_eq!(transaction_edit_path(&replay[..replay_len]), path);
        assert_eq!(endpoint.session().revision(), SessionRevision(8));
    }
}

/// #1615: the replay entry of a structural commit is written to the committed cache by the commit,
/// with the oldest entry evicted there, and by nothing before. The cache of one entry makes every
/// commit an eviction, and a token dropped before its commit leaves the cache as it was: an
/// exact replay of the entry it held returns the bytes it acknowledged, and the evicted request
/// is expired once, at the commit that evicted it.
#[test]
fn a_structural_commit_writes_its_replay_entry_and_evicts_at_the_commit() {
    fn transaction(request: u64, revision: u64, name: &str) -> Vec<u8> {
        let edits = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse(name).expect("ID"),
        }];
        full_command(
            request,
            ExpectedRevision::Exact(SessionRevision(revision)),
            crate::CommandPayload::SessionTransactionApply(&edits),
        )
    }
    fn structural(
        endpoint: &mut ProtocolController<MockProvider>,
        input: &[u8],
    ) -> PreparedStructuralCommand {
        match endpoint
            .prepare_command_frame(
                input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare")
        {
            PreparedCommandFrame::Structural(prepared) => *prepared,
            PreparedCommandFrame::Immediate(_)
            | PreparedCommandFrame::Session(_)
            | PreparedCommandFrame::ObservationRead(_) => {
                panic!("expected structural token")
            }
        }
    }
    fn replay(endpoint: &mut ProtocolController<MockProvider>, input: &[u8]) -> Vec<u8> {
        let mut output = [0_u8; 2048];
        match endpoint.process_command_frame_into(
            input,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut output,
        ) {
            Ok(length) => output[..length].to_vec(),
            Err(error) => panic!("replay refused: {error:?}"),
        }
    }
    fn status(frame: &[u8]) -> StatusCode {
        match ProtocolCodec::default()
            .decode_typed_response(frame, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("typed response")
        {
            crate::DecodedTypedResponseFrame::Success { header, .. }
            | crate::DecodedTypedResponseFrame::NonOk { header, .. } => header.status,
        }
    }

    let first = transaction(1, 7, "replay-first");
    let second = transaction(2, 8, "replay-second");
    let mut endpoint = controller(1);

    let token = structural(&mut endpoint, &first);
    assert!(
        endpoint.replay().is_empty(),
        "nothing is written before the commit"
    );
    let committed = endpoint
        .commit_prepared_structural(token, EditPath::Live)
        .expect("first commit");
    let mut first_reply = [0_u8; 2048];
    let first_length = committed.write_into(&mut first_reply).expect("reply");
    let first_reply = &first_reply[..first_length];
    assert_eq!(
        endpoint.replay().len(),
        1,
        "the reply exists with its entry"
    );
    assert_eq!(replay(&mut endpoint, &first), first_reply);
    let _ = endpoint.queues_mut().try_dequeue_event();

    // A token dropped before its commit changes nothing: entry 1 is still there and exact.
    let token = structural(&mut endpoint, &second);
    assert_eq!(endpoint.replay().len(), 1);
    drop(token);
    assert_eq!(endpoint.replay().len(), 1);
    assert_eq!(replay(&mut endpoint, &first), first_reply);
    assert_eq!(endpoint.session().revision(), SessionRevision(8));

    // The commit of the second evicts the first, once.
    let token = structural(&mut endpoint, &second);
    assert_eq!(
        endpoint.replay().len(),
        1,
        "the eviction waits for the commit"
    );
    // While the token is outstanding every entry point refuses, a replay of a cached entry too:
    // typed, with nothing queued and nothing acknowledged.
    assert!(matches!(
        endpoint.process_command_frame_into(
            &first,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut [0_u8; 2048],
        ),
        Err(CommandFrameProcessError::PreparedCommandOutstanding)
    ));
    let committed = endpoint
        .commit_prepared_structural(token, EditPath::ModelOnly)
        .expect("second commit");
    let mut second_reply = [0_u8; 2048];
    let second_length = committed.write_into(&mut second_reply).expect("reply");
    let second_reply = &second_reply[..second_length];
    assert_eq!(endpoint.replay().len(), 1);
    assert_eq!(replay(&mut endpoint, &second), second_reply);
    assert_eq!(
        status(&replay(&mut endpoint, &first)),
        StatusCode::ReplayExpired
    );
    assert_eq!(endpoint.session().revision(), SessionRevision(9));
}

fn response_revision(frame: &[u8]) -> SessionRevision {
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(frame, &mut DecodeScratch::new(&mut fields))
        .expect("decodable response")
    {
        crate::DecodedTypedResponseFrame::Success { header, .. } => header.revision,
        crate::DecodedTypedResponseFrame::NonOk { .. } => panic!("expected a success"),
    }
}

/// A `SET_CONTROL` of the builtin row `row` of `strip`, on `lane`.
fn builtin_edit(
    strip: &str,
    row: &str,
    lane: crate::control_core::Lane,
    value: f32,
    ramp: Option<u32>,
) -> SessionEdit {
    SessionEdit::set_control(
        crate::control_core::row_address(
            crate::control_core::Owner::Strip(session::StableId::parse(strip).expect("ID")),
            crate::control_core::Section::Builtins,
            row,
            lane,
        )
        .expect("a builtin row"),
        value,
        ramp,
    )
}

fn solo_edit(track: &session::StableId, on: bool, ramp: Option<u32>) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: crate::control_core::ControlAddress {
            owner: crate::control_core::Owner::Strip(track.clone()),
            section: crate::control_core::Section::Own,
            control: 2,
            lane: crate::control_core::Lane::Both,
        },
        value: f32::from(u8::from(on)),
        ramp_samples: ramp,
    }
}

/// A transaction of monitor writes (#1559 D3, D4): its token exposes the writes, commits no model
/// and no revision, emits no `SESSION_COMMITTED` and cancels no automation, answers `monitor` at
/// the committed revision, and an exact replay returns the same bytes. A solo spends no revision
/// number: the next model commit is the committed revision plus one (#1603). The one-call path has
/// no overlay and refuses the write.
///
/// Red if a monitor write advances or spends a revision or emits an event (a listener would reload
/// a session that did not change), if a replay is not byte-identical, or if the one-call path
/// acknowledges a write that nobody applies.
#[test]
fn a_transaction_of_monitor_writes_commits_no_model_revision_or_event() {
    let track = parse_session_json(EXAMPLE).expect("fixture").tracks[0]
        .id
        .clone();
    let solo = [solo_edit(&track, true, Some(96))];
    let input = full_command(
        1,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&solo),
    );
    let prepare = |endpoint: &mut ProtocolController<MockProvider>, input: &[u8]| match endpoint
        .prepare_command_frame(
            input,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        )
        .expect("prepare")
    {
        PreparedCommandFrame::Structural(prepared) => *prepared,
        PreparedCommandFrame::Immediate(_)
        | PreparedCommandFrame::Session(_)
        | PreparedCommandFrame::ObservationRead(_) => {
            panic!("expected a token")
        }
    };

    {
        let mut endpoint = controller(8);
        let snapshot = endpoint.session().canonical_snapshot().to_owned();
        let events = endpoint
            .queues_mut()
            .report(crate::QueueKind::ReliableEvent);
        let prepared = prepare(&mut endpoint, &input);
        assert!(prepared.is_monitor_only());
        assert_eq!(
            prepared.monitor_edits(),
            [crate::control_core::MonitorWrite {
                strip: track.clone(),
                row: crate::control_core::MonitorRow::Solo,
                on: true,
                ramp: Some(96),
            }]
        );
        assert_eq!(
            prepared.prospective_session().revision(),
            SessionRevision(7)
        );
        assert_eq!(
            prepared
                .prospective_session()
                .compiled()
                .expect("a compiled transaction")
                .canonical_json(),
            snapshot,
            "the model is the committed one"
        );
        let committed = endpoint
            .commit_prepared_structural(prepared, EditPath::ModelOnly)
            .expect("a monitor token takes the monitor path whatever path the host names");
        let mut first = [0_u8; 2048];
        let len = committed.write_into(&mut first).expect("bytes");
        assert_eq!(transaction_edit_path(&first[..len]), EditPath::Monitor);
        assert_eq!(response_revision(&first[..len]), SessionRevision(7));
        assert_eq!(endpoint.session().revision(), SessionRevision(7));
        assert_eq!(endpoint.session().canonical_snapshot(), snapshot);
        assert_eq!(
            endpoint.queues().report(crate::QueueKind::ReliableEvent),
            events,
            "no SESSION_COMMITTED"
        );

        // The replay returns the committed bytes and spends nothing.
        let mut replay = [0_u8; 2048];
        let replay_len = endpoint
            .process_command_frame_into(
                &input,
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                &mut replay,
            )
            .expect("replay");
        assert_eq!(&replay[..replay_len], &first[..len]);
        assert_eq!(endpoint.session().revision(), SessionRevision(7));

        // The next model commit is the committed revision plus one: a solo spends no number
        // (#1603).
        let model_edit = [SessionEdit::SetSessionId {
            session_id: session::StableId::parse("after-solo").expect("ID"),
        }];
        let next = full_command(
            2,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::SessionTransactionApply(&model_edit),
        );
        let prepared = prepare(&mut endpoint, &next);
        assert!(!prepared.is_monitor_only());
        assert!(prepared.monitor_edits().is_empty());
        assert_eq!(
            prepared.prospective_session().revision(),
            SessionRevision(8)
        );
        endpoint
            .commit_prepared_structural(prepared, EditPath::ModelOnly)
            .expect("commit");
        assert_eq!(endpoint.session().revision(), SessionRevision(8));
    }

    // A solo beside a model edit is a model transaction with a monitor write, in order, the later
    // write of a strip replacing the earlier.
    let mut endpoint = controller(8);
    let mixed = [
        solo_edit(&track, true, Some(10)),
        SessionEdit::SetSessionId {
            session_id: session::StableId::parse("mixed").expect("ID"),
        },
        solo_edit(&track, false, Some(20)),
    ];
    let input = full_command(
        3,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&mixed),
    );
    let prepared = prepare(&mut endpoint, &input);
    assert!(!prepared.is_monitor_only());
    assert_eq!(
        prepared.monitor_edits(),
        [crate::control_core::MonitorWrite {
            strip: track.clone(),
            row: crate::control_core::MonitorRow::Solo,
            on: false,
            ramp: Some(20),
        }]
    );
    assert_eq!(
        prepared.prospective_session().revision(),
        SessionRevision(8)
    );
    endpoint
        .commit_prepared_structural(prepared, EditPath::Live)
        .expect("commit");
    assert_eq!(endpoint.session().revision(), SessionRevision(8));

    // The one-call path has no overlay: it refuses the write, as it always did.
    let mut endpoint = controller(8);
    let input = full_command(
        4,
        ExpectedRevision::Exact(SessionRevision(7)),
        crate::CommandPayload::SessionTransactionApply(&solo),
    );
    let mut output = [0_u8; 2048];
    let len = endpoint
        .process_command_frame_into(
            &input,
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            &mut output,
        )
        .expect("a typed refusal");
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(&output[..len], &mut DecodeScratch::new(&mut fields))
        .expect("decodable")
    {
        crate::DecodedTypedResponseFrame::NonOk { header, payload } => {
            assert_eq!(header.status, StatusCode::ValidationFailed);
            assert_eq!(payload.diagnostics[0].code, "control.monitor");
        }
        crate::DecodedTypedResponseFrame::Success { .. } => panic!("acknowledged a lost write"),
    }
    assert_eq!(endpoint.session().revision(), SessionRevision(7));
}

#[test]
fn structural_token_cancel_owner_generation_and_serial_rules_are_affine() {
    fn structural(
        endpoint: &mut ProtocolController<MockProvider>,
        input: &[u8],
    ) -> PreparedStructuralCommand {
        match endpoint
            .prepare_command_frame(
                input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare")
        {
            PreparedCommandFrame::Structural(prepared) => *prepared,
            PreparedCommandFrame::Immediate(_)
            | PreparedCommandFrame::Session(_)
            | PreparedCommandFrame::ObservationRead(_) => {
                panic!("expected structural token")
            }
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
    let mut owner = controller(8);
    let mut other = controller(8);

    let canceled = structural(&mut owner, &first);
    assert!(matches!(
        owner.prepare_command_frame(
            &first,
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048,
        ),
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
        other.commit_prepared_structural(wrong_owner, EditPath::ModelOnly),
        Err(PreparedCommandCommitError::WrongController)
    ));
    assert_eq!(owner.session().revision(), SessionRevision(7));
    assert_eq!(other.session().revision(), SessionRevision(7));

    let stale = structural(&mut owner, &first);
    owner.structural_generation.fetch_add(2, Ordering::AcqRel);
    assert!(matches!(
        owner.commit_prepared_structural(stale, EditPath::ModelOnly),
        Err(PreparedCommandCommitError::StaleGeneration)
    ));
    owner.structural_generation.fetch_add(1, Ordering::AcqRel);

    let first_token = structural(&mut owner, &first);
    owner
        .commit_prepared_structural(first_token, EditPath::ModelOnly)
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
        .commit_prepared_structural(second_token, EditPath::ModelOnly)
        .expect("second serial commit");
    assert_eq!(owner.session().revision(), SessionRevision(9));
    assert_eq!(owner.replay().len(), 2);
}

#[test]
fn identical_replay_changed_reuse_eviction_and_expiry_are_deterministic() {
    let mut controller = controller(1);
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

    let mut controller = controller(2);
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
    let mut controller = controller(4);
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
    let mut controller = controller(4);
    controller.config.maximum_response_diagnostics = 1;
    // A structural upsert can carry a bad value to the final validation; `SET_CONTROL` refuses a
    // non-finite value at its own edit instead.
    let mut track = controller.session().compiled().normalized_model().tracks[0].clone();
    track.fader.left_db = f32::NAN;
    track.fader.right_db = f32::INFINITY;
    let request = ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"two-validation-diagnostics",
        command: ControlCommand::SessionTransactionApply {
            edits: &[SessionEdit::UpsertTrack { track }],
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
    let mut controller = controller(4);
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

/// #1394 D4: a per-edit ramp length above one second of the committed rate is refused on the wire
/// as `session.edit.ramp_out_of_range` at that edit's `edits[i]` path, and nothing commits.
///
/// Red if the controller maps `RampOutOfRange` to another code (the typed error is covered by the
/// store's own test, the wire spelling by this one only), reports the wrong operation index or
/// path, or commits the refused transaction.
#[test]
fn a_ramp_length_out_of_range_answers_its_own_code_at_the_edit_path() {
    let mut controller = controller(4);
    let response = controller.process(ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"ramp-out-of-range",
        command: ControlCommand::SessionTransactionApply {
            edits: &[
                SessionEdit::SetSessionId {
                    session_id: session::StableId::parse("renamed").expect("stable ID"),
                },
                builtin_edit(
                    "vocal",
                    "fader_db",
                    crate::control_core::Lane::Left,
                    -2.0,
                    Some(u32::MAX),
                ),
            ],
        },
    });
    assert_eq!(response.status, StatusCode::ValidationFailed);
    let decoded = ProtocolCodec::default()
        .decode_non_ok_payload(response.payload(), 2)
        .expect("typed edit error");
    assert_eq!(decoded.omitted_diagnostics, 0);
    assert_eq!(decoded.diagnostics.len(), 1);
    let diagnostic = &decoded.diagnostics[0];
    assert_eq!(diagnostic.code, "session.edit.ramp_out_of_range");
    assert_eq!(
        diagnostic.path,
        vec![
            crate::PathSegment::Field("edits".to_owned()),
            crate::PathSegment::Index(1),
        ]
    );
    assert_eq!(diagnostic.operation_index, Some(1));
    assert_eq!(controller.session().revision(), SessionRevision(7));
}

/// Decision 12's retired and refused codes, one conformance row each (#1094): every row's frame,
/// processed whole by an endpoint over `conformance::console_session_fixture()`, is answered with
/// the row's status and diagnostic, and a refusing row commits nothing and emits no event -- the
/// acked-batch question: no ack precedes a drop.
///
/// Red if a retired rack code is read as another rack (the edit applies: `OK`), if a retired track
/// field is skipped as an unknown optional field (`OK`), or if a console-refused edit is routed to
/// the inserts or the entries (`OK`) or answers `NotFound` (another diagnostic). The control row
/// must commit, so a broken splice cannot pass as a refusal.
#[test]
fn retired_and_console_refused_codes_meet_their_conformance_rows() {
    let rows = conformance::retired_code_rows();
    assert_eq!(rows.len(), 16 + 4 + 7 + 13 + 1 + 4 + 10 + 2 + 1);
    for row in rows {
        let mut endpoint =
            controller_over(conformance::console_session_fixture(), 8, SampleTime(0));
        let snapshot = endpoint.session().canonical_snapshot().to_owned();
        if row.status == StatusCode::UnsupportedMessage as u16 {
            // A message ID that decodes to no message leaves no safe request identity to
            // answer, so the refusal is the decoder's status and no response frame exists.
            let mut output = [0x5a_u8; 2048];
            let refusal = endpoint
                .process_command_frame_into(
                    &row.bytes,
                    &mut DecodeScratch::new(&mut [0_u16; 32]),
                    &mut output,
                )
                .expect_err(&row.name);
            let CommandFrameProcessError::Uncorrelatable(error) = refusal else {
                panic!("{}: refused for another reason: {refusal:?}", row.name);
            };
            assert_eq!(error.status() as u16, row.status, "{}", row.name);
            assert_eq!(output, [0x5a; 2048], "{}: nothing is written", row.name);
            assert_eq!(endpoint.session().canonical_snapshot(), snapshot);
            continue;
        }
        let response = process_full_command(&mut endpoint, &row.bytes);
        let mut fields = [0_u16; 64];
        let decoded = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .unwrap_or_else(|error| panic!("{}: correlatable response: {error:?}", row.name));
        match decoded {
            crate::DecodedTypedResponseFrame::NonOk { header, payload } => {
                assert_eq!(header.status as u16, row.status, "{}", row.name);
                assert_eq!(header.request_id, id(1), "{}", row.name);
                let codes = payload
                    .diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.code.as_str())
                    .collect::<Vec<_>>();
                assert_eq!(
                    codes,
                    row.diagnostic.into_iter().collect::<Vec<_>>(),
                    "{}",
                    row.name
                );
                assert_eq!(
                    endpoint.session().revision(),
                    SessionRevision(7),
                    "{}",
                    row.name
                );
                assert_eq!(
                    endpoint.session().canonical_snapshot(),
                    snapshot,
                    "{}",
                    row.name
                );
                assert_eq!(
                    endpoint
                        .queues()
                        .report(crate::QueueKind::ReliableEvent)
                        .occupancy,
                    0,
                    "{}: a refusal emits no session event",
                    row.name
                );
            }
            crate::DecodedTypedResponseFrame::Success { header, .. } => {
                assert_eq!(row.status, StatusCode::Ok as u16, "{} succeeded", row.name);
                assert_eq!(header.revision, SessionRevision(8), "{}", row.name);
                assert_eq!(
                    endpoint.session().revision(),
                    SessionRevision(8),
                    "{}",
                    row.name
                );
            }
        }
    }
}

/// #1635 gate 1. A `TRANSPORT_SET` that still carries the retired position (field 2), in either
/// flag form, is refused at decode: the transport state, the revision, the session and the
/// reliable lane stay as they were, and no response frame acknowledges it.
///
/// Red if a peer can still store a position, if the field is skipped as an unknown optional field
/// and the state set anyway (a playing readback), or if the refusal moves state before it refuses.
#[test]
fn a_transport_set_with_the_retired_position_changes_nothing() {
    let rows = conformance::retired_code_rows()
        .into_iter()
        .filter(|row| row.name.starts_with("retired.transport_set_field_2."))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 2, "one row per flag form");
    for row in rows {
        let mut endpoint = controller(8);
        let snapshot = endpoint.session().canonical_snapshot().to_owned();
        let response = process_full_command(&mut endpoint, &row.bytes);
        let mut fields = [0_u16; 64];
        let crate::DecodedTypedResponseFrame::NonOk { header, .. } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .unwrap_or_else(|error| panic!("{}: correlatable response: {error:?}", row.name))
        else {
            panic!("{}: a set with a position was acknowledged", row.name);
        };
        assert_eq!(header.status as u16, row.status, "{}", row.name);
        assert_eq!(endpoint.session().revision(), SessionRevision(7));
        assert_eq!(endpoint.session().canonical_snapshot(), snapshot);
        assert_eq!(
            endpoint
                .queues()
                .report(crate::QueueKind::ReliableEvent)
                .occupancy,
            0,
            "{}: no TRANSPORT_STATE event",
            row.name
        );
        let get = process_full_command(
            &mut endpoint,
            &full_command(
                2,
                ExpectedRevision::Any,
                crate::CommandPayload::TransportGet,
            ),
        );
        let crate::DecodedTypedResponseFrame::Success { header, payload } =
            ProtocolCodec::default()
                .decode_typed_response(&get, &mut DecodeScratch::new(&mut fields))
                .expect("transport get response")
        else {
            panic!("{}: TRANSPORT_GET failed", row.name);
        };
        assert_eq!(header.message_id, MessageId::TransportGet);
        let crate::DecodedSuccessResponsePayload::TransportGetSnapshot(snapshot) = payload else {
            panic!("{}: not a transport snapshot", row.name);
        };
        assert_eq!(snapshot.state, TransportState::Stopped, "{}", row.name);
    }
}

/// #1635 gate 2. `TRANSPORT_SET` still works without the position: it answers a snapshot with the
/// requested state and the effective sample and queues exactly one `TRANSPORT_STATE` event that
/// carries the same two values.
///
/// Red if removing the position breaks the state round trip, the effective sample, or the event.
#[test]
fn a_transport_set_answers_the_state_and_queues_one_event_with_it() {
    let mut endpoint = controller_over(
        parse_session_json(EXAMPLE).expect("fixture"),
        8,
        SampleTime(96),
    );
    let response = process_full_command(
        &mut endpoint,
        &full_command(
            1,
            ExpectedRevision::Exact(SessionRevision(7)),
            crate::CommandPayload::TransportSet(TransportSetRequest {
                state: TransportState::Playing,
            }),
        ),
    );
    let mut fields = [0_u16; 64];
    let crate::DecodedTypedResponseFrame::Success { header, payload } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("transport set response")
    else {
        panic!("TRANSPORT_SET failed");
    };
    assert_eq!(header.message_id, MessageId::TransportSet);
    let crate::DecodedSuccessResponsePayload::TransportSetSnapshot(snapshot) = payload else {
        panic!("not a transport set snapshot");
    };
    assert_eq!(
        snapshot,
        TransportSnapshot {
            state: TransportState::Playing,
            effective_sample: SampleTime(96),
        }
    );
    assert_eq!(
        endpoint
            .queues()
            .report(crate::QueueKind::ReliableEvent)
            .occupancy,
        1
    );
    let slot = endpoint.queues_mut().try_dequeue_event().expect("event");
    assert_eq!(slot.message_id, MessageId::TransportState);
    let mut event = [0_u8; 64];
    let length = endpoint
        .encode_transport_state_event(slot, &mut event)
        .expect("event payload");
    assert_eq!(
        ProtocolCodec::default().decode_transport_state_event(&event[..length], 4),
        Ok(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Playing,
            effective_sample: SampleTime(96),
            origin_request_id: Some(id(1)),
        })
    );
    assert!(endpoint.queues_mut().try_dequeue_event().is_err());
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
        let controller = controller(4);
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
    let mut controller = controller(4);
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

        let mut typed = controller(4);
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

        let mut btlv = controller(4);
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
fn decoded_invalid_automation_rolls_back_after_final_compilation() {
    let mut controller = controller(4);
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
                1,
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
    };
    let set_bytes = set_frame(2, ExpectedRevision::Exact(SessionRevision(7)), set);
    let mut controller = controller(8);
    let get_response = controller
        .process_b1b_btlv(&get_frame, &mut DecodeScratch::new(&mut [0_u16; 0]))
        .expect("typed get");
    assert_eq!(get_response.status, StatusCode::Ok);
    assert_eq!(
        codec
            .decode_transport_snapshot(get_response.payload(), 2)
            .expect("get snapshot"),
        TransportSnapshot {
            state: TransportState::Stopped,
            effective_sample: SampleTime(0),
        }
    );
    let set_response = controller
        .process_b1b_btlv(&set_bytes, &mut DecodeScratch::new(&mut [0_u16; 2]))
        .expect("typed set");
    assert_eq!(set_response.status, StatusCode::Ok);
    assert_eq!(
        codec
            .decode_transport_snapshot(set_response.payload(), 2)
            .expect("set snapshot"),
        TransportSnapshot {
            state: TransportState::Playing,
            effective_sample: SampleTime(0),
        }
    );
    let event = controller
        .queues_mut()
        .try_dequeue_event()
        .expect("transport event");
    let mut event_payload = [0_u8; 64];
    assert_eq!(
        controller.encode_transport_state_event(event, &mut event_payload),
        Ok(64)
    );
    assert_eq!(
        codec.decode_transport_state_event(&event_payload, 4),
        Ok(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Playing,
            effective_sample: SampleTime(0),
            origin_request_id: Some(id(2)),
        })
    );
    let mut event_frame = [0_u8; crate::OUTER_HEADER_BYTES + 64];
    codec
        .write_outer_header(
            &mut event_frame,
            crate::FrameKind::Event,
            MessageId::TransportState,
            StatusCode::Ok,
            0,
            7,
            0,
            64,
            4,
        )
        .expect("event header");
    event_frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(&event_payload);
    assert_eq!(
        hex(&event_frame),
        concat!(
            "4d49534f43544c0001000000300003001080000040000000",
            "000000000000000007000000000000000400000000000000",
            "01000401080000000100000000000000",
            "02000101010000000200000000000000",
            "04000401080000000000000000000000",
            "05000400080000000200000000000000"
        )
    );
    let stop = TransportSetRequest {
        state: TransportState::Stopped,
    };
    let retained = controller
        .process_b1b_btlv(
            &set_frame(3, ExpectedRevision::Exact(SessionRevision(7)), stop),
            &mut DecodeScratch::new(&mut [0_u16; 1]),
        )
        .expect("stop");
    assert_eq!(
        codec.decode_transport_snapshot(retained.payload(), 2),
        Ok(TransportSnapshot {
            state: TransportState::Stopped,
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
    let invalid_revision = set_frame(5, ExpectedRevision::Any, set);
    assert_eq!(
        controller
            .process_b1b_btlv(&invalid_revision, &mut DecodeScratch::new(&mut [0_u16; 2]))
            .expect("revision response")
            .status,
        StatusCode::InvalidField
    );
}

/// #1628 gate 3. Nothing cancels: a transaction that changes the revision reserves and emits
/// exactly its `SESSION_COMMITTED`, and a `TRANSPORT_SET` that locates exactly its
/// `TRANSPORT_STATE`. A reliable-event queue of two slots holds each pair of events only if
/// neither commit also reserves a cancellation.
///
/// Red if a revision change or a locate still reserves or emits an `AUTOMATION_CANCELED` event,
/// which would spend a reliable slot for nothing and put a retired ID on the wire.
#[test]
fn a_revision_change_and_a_locate_reserve_and_emit_only_their_own_events() {
    let mut controller = egress_controller(2, 1);
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("nothing-cancels").expect("ID"),
    }];
    let committed = controller.process(ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"revision-change",
        command: ControlCommand::SessionTransactionApply { edits: &edits },
    });
    assert_eq!(committed.status, StatusCode::Ok);
    assert_eq!(
        controller
            .queues()
            .report(crate::QueueKind::ReliableEvent)
            .occupancy,
        1
    );
    let event = controller.queues_mut().try_dequeue_event().expect("event");
    assert_eq!(event.message_id, MessageId::SessionCommitted);
    assert!(controller.queues_mut().try_dequeue_event().is_err());

    let locate = controller.process(ControllerRequest {
        request_id: id(2),
        expected_revision: ExpectedRevision::Exact(SessionRevision(8)),
        canonical_bytes: b"locate",
        command: ControlCommand::TransportSet {
            request: TransportSetRequest {
                state: TransportState::Playing,
            },
        },
    });
    assert_eq!(locate.status, StatusCode::Ok);
    let event = controller.queues_mut().try_dequeue_event().expect("event");
    assert_eq!(event.message_id, MessageId::TransportState);
    assert!(controller.queues_mut().try_dequeue_event().is_err());
}

/// #1628 gate 1. An event frame under the retired ID `0x8002` (`AUTOMATION_CANCELED`) fails typed
/// event decode as an unsupported message, as the retired command frame under the same ID does in
/// the conformance rows.
///
/// Red if the retired event ID decodes again, or a peer's old frame is read as another event.
#[test]
fn an_event_frame_under_the_retired_cancellation_id_is_refused() {
    let codec = ProtocolCodec::default();
    let frame = TypedEventFrame {
        revision: SessionRevision(7),
        payload: EventPayload::SessionCommitted(SessionCommitted {
            event_sequence: 1,
            origin_request_id: id(1),
            previous_revision: SessionRevision(6),
            applied_operations: 1,
        }),
    };
    let mut bytes = vec![0_u8; 256];
    let written = codec
        .encode_event_frame_into(&frame, &mut bytes)
        .expect("event frame");
    bytes.truncate(written);
    assert!(
        codec
            .decode_typed_event(&bytes, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .is_ok(),
        "the control frame decodes before its ID is patched"
    );
    bytes[16..18].copy_from_slice(&0x8002_u16.to_le_bytes());
    assert_eq!(
        codec
            .decode_typed_event(&bytes, &mut DecodeScratch::new(&mut [0_u16; 32]))
            .err(),
        Some(DecodeError::UnsupportedMessage)
    );
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
        meters: vec![crate::MeterEntry {
            target: crate::MeterTarget::Output,
            metric: crate::MeterMetric::Peak,
        }],
        meter_period_blocks: 4,
        meter_window_blocks: None,
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
    let mut controller = controller(8);
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
    // #1583 D4: the echo adds the provider's meter window.
    assert_eq!(
        codec.decode_telemetry_configuration_echo(first.payload(), 7),
        Ok(TelemetryConfiguration {
            meter_window_blocks: Some(core::num::NonZeroU32::MIN),
            ..configuration.clone()
        })
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
    let mut controller = controller(8);
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
                controls: true,
                ..ProviderFeatures::NONE
            },
            12,
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
        let mut endpoint = controller(8);
        endpoint.set_provider_features(features);
        let response = endpoint.process(capability(1, b"features"));
        let decoded = codec
            .decode_capabilities(response.payload(), 23)
            .expect("capabilities");
        if required_command != 0 {
            assert!(
                decoded
                    .supported_commands
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_command)
            );
        }
        if required_event != 0 {
            assert!(
                decoded
                    .supported_events
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .any(|id| u16::from_le_bytes([id[0], id[1]]) == required_event)
            );
        }
        assert_ne!(decoded.flags.0 & (1 << required_bit), 0);
    }
    let mut endpoint = controller(8);
    endpoint.set_provider_features(ProviderFeatures::NONE);
    let caps = endpoint.process(capability(1, b"none"));
    let decoded = codec
        .decode_capabilities(caps.payload(), 23)
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

    let mut endpoint = controller(8);
    endpoint.config.maximum_transaction_edits = 0;
    let caps = endpoint.process(capability(1, b"zero-edit-limit"));
    let decoded = codec
        .decode_capabilities(caps.payload(), 23)
        .expect("zero-edit capabilities");
    assert_eq!(decoded.maximum_transaction_edits, 0);
    assert!(
        !decoded
            .supported_commands
            .as_chunks::<2>()
            .0
            .iter()
            .any(|value| u16::from_le_bytes([value[0], value[1]]) == 3)
    );
    assert!(
        !decoded
            .supported_events
            .as_chunks::<2>()
            .0
            .iter()
            .any(|value| { matches!(u16::from_le_bytes([value[0], value[1]]), 0x8001) })
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
    let mut controller = controller(4);
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

/// An executed `BACKPRESSURE` outcome is cached like any other: a retry under the same request ID
/// replays the identical response and executes nothing, even after the queue that refused the
/// first attempt has room. Red if the cache stops storing `BACKPRESSURE` outcomes: the retry then
/// executes, commits revision 8 and enqueues an event.
#[test]
fn a_backpressure_outcome_is_cached_and_its_retry_executes_nothing() {
    let mut controller = controller(4);
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
        .expect("fill the reliable event queue");
    let edits = [SessionEdit::SetSessionId {
        session_id: session::StableId::parse("retried-commit").expect("ID"),
    }];
    let request = || ControllerRequest {
        request_id: id(1),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
        canonical_bytes: b"backpressure-then-retry",
        command: ControlCommand::SessionTransactionApply { edits: &edits },
    };
    let refused = controller.process(request());
    assert_eq!(refused.status, StatusCode::Backpressure);
    assert_eq!(controller.session().revision(), SessionRevision(7));

    // Free the queue: an execution of the retry would now succeed.
    let _ = controller
        .queues_mut()
        .try_dequeue_event()
        .expect("the filler event");
    let replayed = controller.process(request());
    assert_eq!(replayed, refused, "the identical response, replayed");
    assert_eq!(controller.session().revision(), SessionRevision(7));
    assert_eq!(controller.session().canonical_snapshot(), before_snapshot);
    assert!(
        controller.queues_mut().try_dequeue_event().is_err(),
        "the replay enqueued no event"
    );
}

#[test]
fn committed_transaction_emits_typed_zero_header_id_event() {
    let mut controller = controller(4);
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
    let mut controller = controller(4);
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

    let mut controller = controller(4);
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
        rack_name: session::RackName::Inserts,
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
fn c2b2_event_egress_emits_all_five_schema_closed_full_frames() {
    let codec = ProtocolCodec::default();
    let mut controller = egress_controller(4, 256);
    let revision = controller.session().revision();
    let counter_ids = vec![
        crate::CounterId::ControlCommandBackpressure,
        crate::CounterId::TelemetryCoalesced,
    ];
    configure_event_egress(&mut controller, 1, 256, counter_ids, true);

    let committed = ReliableSlot::session_committed(revision, 1, id(9), SessionRevision(6), 2);
    let transport = ReliableSlot::transport_state(
        revision,
        2,
        TransportState::Playing,
        SampleTime(12),
        Some(id(11)),
    );
    controller
        .queues_mut()
        .try_enqueue_event(committed)
        .expect("committed");
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
        .stage_meter_batch_event(revision, window(12, 24), &meter_records)
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
            payload: EventPayload::TransportState(TransportStateEvent {
                event_sequence: 2,
                state: TransportState::Playing,
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
            first_sample: SampleTime(12),
            window_sequence: 1,
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
    configure_event_egress(&mut controller, 1, 0, vec![low, high], false);
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
    let second =
        ReliableSlot::transport_state(revision, 2, TransportState::Stopped, SampleTime(0), None);
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

    configure_event_egress(&mut controller, 2, 0, Vec::new(), true);
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
    configure_event_egress(&mut lossy, 1, 1, Vec::new(), false);
    lossy
        .stage_meter_batch_event(
            lossy_revision,
            window(0, 4),
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
            window(0, 1),
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

/// Decode a full response frame that the controller wrote.
fn decode_response(bytes: &[u8]) -> crate::DecodedTypedResponseFrame<'_> {
    let mut fields = [0_u16; 1024];
    ProtocolCodec::default()
        .decode_typed_response(bytes, &mut DecodeScratch::new(&mut fields))
        .expect("a typed response")
}

fn control_request(
    structure_epoch: u64,
    after_handle: u32,
    limit: u16,
    owner: Option<crate::ControlOwnerFilter>,
) -> ControlListRequest {
    ControlListRequest {
        structure_epoch,
        after_handle,
        limit,
        owner,
        headers_only: false,
    }
}

/// Gate 3: both control messages dispatch through typed frames to the provider, a page follows its
/// cursor and limit, an owner filter narrows it, and the state page holds the values of the same
/// rows without the headers.
///
/// Red if a message is not reachable through the frame path, the controller answers one message
/// with the other's page, or the cursor, limit or owner filter is dropped between frame and
/// provider.
#[test]
fn control_messages_dispatch_through_typed_frames_to_the_provider() {
    let mut endpoint = controller(8);
    let mut list = |request: &ControlListRequest, request_id: u64| {
        let input = full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::ControlListGet(request),
        );
        let output = process_full_command(&mut endpoint, &input);
        match decode_response(&output) {
            crate::DecodedTypedResponseFrame::Success {
                header,
                payload: crate::DecodedSuccessResponsePayload::ControlListPage(page),
            } => {
                assert_eq!(header.message_id, MessageId::ControlListGet);
                page
            }
            _ => panic!("expected a control list page"),
        }
    };
    let whole = list(&control_request(0, 0, 8, None), 1);
    assert_eq!(whole.entries.len(), 3);
    assert_eq!((whole.last_handle, whole.eof), (3, true));
    assert_eq!((whole.revision, whole.structure_epoch), (7, 1));
    assert!(matches!(
        whole.entries[0].body,
        crate::ControlBody::OwnerHeader(_)
    ));
    assert!(matches!(
        whole.entries[1].body,
        crate::ControlBody::SectionHeader(_)
    ));
    assert!(matches!(
        whole.entries[2].body,
        crate::ControlBody::Control(_)
    ));
    // The cursor and the limit reach the provider: a page of one, then the rest.
    let first = list(&control_request(1, 0, 1, None), 2);
    assert_eq!(
        (first.entries.len(), first.last_handle, first.eof),
        (1, 1, false)
    );
    let rest = list(&control_request(1, first.last_handle, 8, None), 3);
    assert_eq!(
        rest.entries
            .iter()
            .map(|entry| entry.handle)
            .collect::<Vec<_>>(),
        [2, 3]
    );
    assert!(rest.eof);
    // The owner filter reaches the provider: another strip has no rows and ends the list.
    let other = list(
        &control_request(
            0,
            0,
            8,
            Some(crate::ControlOwnerFilter {
                kind: crate::ControlOwnerKind::Strip,
                id: Some("guitar".to_owned()),
            }),
        ),
        4,
    );
    assert_eq!(
        (other.entries.len(), other.last_handle, other.eof),
        (0, 0, true)
    );

    let input = full_command(
        5,
        ExpectedRevision::Any,
        crate::CommandPayload::ControlStateGet(&control_request(1, 0, 8, None)),
    );
    let output = process_full_command(&mut endpoint, &input);
    let crate::DecodedTypedResponseFrame::Success {
        header,
        payload: crate::DecodedSuccessResponsePayload::ControlStatePage(state),
    } = decode_response(&output)
    else {
        panic!("expected a control state page");
    };
    assert_eq!(header.message_id, MessageId::ControlStateGet);
    assert_eq!(
        state.records,
        [crate::ControlStateRecord {
            handle: AUTOMATABLE_CONTROL,
            value: 0.25
        }],
        "the control row only; the two header rows have no record"
    );
    assert_eq!((state.last_handle, state.eof), (3, true));
}

/// A page holds at most `limit` rows and fewer when its response would not fit the endpoint's
/// response bound: the list and the state still page through to the end, in pages that each fit,
/// and one row that cannot fit alone is `LIMIT_EXCEEDED`.
///
/// Red if a request for 256 rows on a small frame is answered as an internal failure, a trimmed
/// page is marked as the end of the list, or its cursor skips or repeats a row.
#[test]
fn a_page_is_trimmed_to_the_response_bound_and_the_list_still_pages_through() {
    let mut endpoint = controller(8);
    let row = mock_controls().pop().expect("a control row");
    let count = 200_u32;
    endpoint.provider_mut().controls = (1..=count)
        .map(|handle| crate::ControlEntry {
            handle,
            ..row.clone()
        })
        .collect();
    let mut request_id = 0;
    let mut handles = Vec::new();
    let mut after_handle = 0;
    loop {
        request_id += 1;
        let request = control_request(1, after_handle, 256, None);
        let input = full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::ControlListGet(&request),
        );
        let output = process_full_command(&mut endpoint, &input);
        assert!(output.len() <= 1024, "a page fits the response bound");
        let crate::DecodedTypedResponseFrame::Success {
            payload: crate::DecodedSuccessResponsePayload::ControlListPage(page),
            ..
        } = decode_response(&output)
        else {
            panic!("expected a control list page");
        };
        assert!(!page.entries.is_empty());
        assert_eq!(page.last_handle, page.entries.last().expect("a row").handle);
        handles.extend(page.entries.iter().map(|entry| entry.handle));
        after_handle = page.last_handle;
        if page.eof {
            break;
        }
        assert!(page.entries.len() < usize::from(256_u16), "a trimmed page");
    }
    assert_eq!(handles, (1..=count).collect::<Vec<_>>());
    assert!(request_id > 1, "the list took more than one page");

    let mut records = Vec::new();
    let mut after_handle = 0;
    loop {
        request_id += 1;
        let request = control_request(1, after_handle, 256, None);
        let input = full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::ControlStateGet(&request),
        );
        let output = process_full_command(&mut endpoint, &input);
        assert!(output.len() <= 1024, "a page fits the response bound");
        let crate::DecodedTypedResponseFrame::Success {
            payload: crate::DecodedSuccessResponsePayload::ControlStatePage(page),
            ..
        } = decode_response(&output)
        else {
            panic!("expected a control state page");
        };
        records.extend(page.records.iter().map(|record| record.handle));
        after_handle = page.last_handle;
        if page.eof {
            break;
        }
    }
    assert_eq!(records, (1..=count).collect::<Vec<_>>());

    // A row that does not fit a page alone cannot be served at all.
    let mut huge = row;
    if let crate::ControlBody::Control(control) = &mut huge.body {
        control.display_name = "x".repeat(2_000);
    }
    endpoint.provider_mut().controls = vec![huge];
    let request = control_request(0, 0, 1, None);
    let input = full_command(
        request_id + 1,
        ExpectedRevision::Any,
        crate::CommandPayload::ControlListGet(&request),
    );
    let output = process_full_command(&mut endpoint, &input);
    let crate::DecodedTypedResponseFrame::NonOk { header, .. } = decode_response(&output) else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::LimitExceeded);
}

/// Batch 2 review fold item 3: a provider page whose content breaks the message's rules (here
/// two rows with one handle) is the provider's defect at any size, so it is `INTERNAL` for both
/// messages and is never trimmed into a smaller page or reported as `LIMIT_EXCEEDED`.
///
/// Red if the codec reports a content failure as an excess, so the controller halves the page down
/// to one row (which is valid) and answers a corrupt list as if it were a short one.
#[test]
fn a_content_invalid_provider_page_is_internal_and_is_not_trimmed() {
    let mut endpoint = controller(8);
    let row = mock_controls().pop().expect("a control row");
    endpoint.provider_mut().controls = (0..3)
        .map(|_| crate::ControlEntry {
            handle: 2,
            ..row.clone()
        })
        .collect();
    for (request_id, state) in [(1_u64, false), (2, true)] {
        let request = control_request(0, 0, 8, None);
        let payload = if state {
            crate::CommandPayload::ControlStateGet(&request)
        } else {
            crate::CommandPayload::ControlListGet(&request)
        };
        let input = full_command(request_id, ExpectedRevision::Any, payload);
        let output = process_full_command(&mut endpoint, &input);
        let crate::DecodedTypedResponseFrame::NonOk { header, .. } = decode_response(&output)
        else {
            panic!("expected a refusal");
        };
        assert_eq!(header.status, StatusCode::Internal, "state {state}");
    }
}

/// Gate 3: a request for a structure that is not current is `REVISION_CONFLICT` with the typed
/// diagnostic `control.list.stale`, for both messages, and `0` always names the current one.
///
/// Red if a stale epoch is served as a page of handles from another structure, or the refusal is
/// an untyped status that a client cannot tell from an edit conflict.
#[test]
fn a_stale_structure_epoch_is_a_revision_conflict_that_names_itself() {
    let mut endpoint = controller(8);
    for (request_id, stale) in [(1_u64, false), (2, true), (3, true)] {
        let request = control_request(if stale { 2 } else { 0 }, 0, 8, None);
        let payload = if request_id == 3 {
            crate::CommandPayload::ControlStateGet(&request)
        } else {
            crate::CommandPayload::ControlListGet(&request)
        };
        let input = full_command(request_id, ExpectedRevision::Any, payload);
        let output = process_full_command(&mut endpoint, &input);
        match decode_response(&output) {
            crate::DecodedTypedResponseFrame::Success { .. } => {
                assert!(!stale, "request {request_id} is served");
            }
            crate::DecodedTypedResponseFrame::NonOk { header, payload } => {
                assert!(stale, "request {request_id} is refused");
                assert_eq!(header.status, StatusCode::RevisionConflict);
                let codes = payload
                    .diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.code.as_str())
                    .collect::<Vec<_>>();
                assert_eq!(codes, ["control.list.stale"]);
            }
        }
    }
}

/// Gate 2: the capability registry lists `0x000c` and `0x000d` and never the retired `0x0004` and
/// `0x0005`; without the controls feature neither new message is listed, bit 7 is clear and a
/// request for one is `UNAVAILABLE`. #1595 D8: with the spectrum feature the three spectrum
/// commands are listed with bit 16, and without it none is and bit 16 is clear. #1586 D6: the
/// observation feature switches `0x000f` and bit 15 together, and the session executes the read
/// exactly when it is advertised.
///
/// Red if a retired ID is advertised or served, bit 7 stops tracking the pair, a disabled
/// provider answers a control request, or the spectrum feature does not switch the three
/// commands and bit 16 together, or the observation feature `0x000f`, bit 15 and the read's
/// route.
#[test]
fn the_capability_registry_lists_the_control_messages_and_not_the_retired_ones() {
    let ids = |endpoint: &mut ProtocolController<MockProvider>, request_id: u64| {
        let response = endpoint.process(capability(request_id, b"registry"));
        let decoded = ProtocolCodec::default()
            .decode_capabilities(response.payload(), 23)
            .expect("capabilities");
        let commands = decoded
            .supported_commands
            .as_chunks::<2>()
            .0
            .iter()
            .map(|id| u16::from_le_bytes([id[0], id[1]]))
            .collect::<Vec<_>>();
        // Bit 14 tracks `RESPONSE_GET` and bit 16 the spectrum commands (#1595 D8) whatever else
        // is listed.
        assert_eq!(decoded.flags.0 & (1 << 14) != 0, commands.contains(&14));
        assert_eq!(decoded.flags.0 & (1 << 16) != 0, commands.contains(&0x0010));
        assert_eq!(decoded.flags.0 & (1 << 15) != 0, commands.contains(&0x000f));
        // #2115: bit 17 tracks `LOUDNESS`.
        assert_eq!(decoded.flags.0 & (1 << 17) != 0, commands.contains(&0x0014));
        (commands, decoded.flags.0 & (1 << 7) != 0)
    };
    let mut endpoint = controller(8);
    assert_eq!(
        ids(&mut endpoint, 1),
        (
            vec![
                1, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14, 0x000f, 0x0010, 0x0011, 0x0012, 0x0014
            ],
            true
        )
    );
    endpoint.set_provider_features(ProviderFeatures {
        controls: false,
        spectrum: false,
        loudness: false,
        observation: false,
        ..ProviderFeatures::ALL
    });
    assert_eq!(
        ids(&mut endpoint, 2),
        (vec![1, 2, 3, 7, 8, 9, 10, 11, 14], false)
    );
    for (request_id, payload) in [
        (
            3,
            crate::CommandPayload::ControlListGet(&control_request(0, 0, 1, None)),
        ),
        (
            4,
            crate::CommandPayload::ControlStateGet(&control_request(0, 0, 1, None)),
        ),
    ] {
        let input = full_command(request_id, ExpectedRevision::Any, payload);
        let output = process_full_command(&mut endpoint, &input);
        let crate::DecodedTypedResponseFrame::NonOk { header, .. } = decode_response(&output)
        else {
            panic!("a disabled control provider must refuse");
        };
        assert_eq!(header.status, StatusCode::Unavailable);
    }
    // #1586 D6: the session executes an advertised `OBSERVATION_READ`; an endpoint that does not
    // advertise it answers `UNAVAILABLE` itself.
    let read = crate::ObservationReadRequest {
        selections: vec![crate::ObservationReadSelection {
            strip_id: session::StableId::parse("t1").expect("strip"),
            section: controls::TapSection::Inserts(session::StableId::parse("comp").expect("id")),
            tap_id: 1,
            channels: crate::ParameterChannel::Both,
        }],
    };
    for (request_id, observation) in [(5, false), (6, true)] {
        endpoint.set_provider_features(ProviderFeatures {
            observation,
            ..ProviderFeatures::ALL
        });
        let input = full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::ObservationRead(&read),
        );
        let prepared = endpoint
            .prepare_command_frame(
                &input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare");
        match (observation, prepared) {
            (true, PreparedCommandFrame::ObservationRead(prepared)) => {
                assert_eq!(prepared.request_id().get(), request_id);
            }
            (false, PreparedCommandFrame::Immediate(response)) => {
                let mut output = [0_u8; 2048];
                let written = response.write_into(&mut output).expect("reply");
                let crate::DecodedTypedResponseFrame::NonOk { header, .. } =
                    decode_response(&output[..written])
                else {
                    panic!("an unadvertised read must refuse");
                };
                assert_eq!(header.status, StatusCode::Unavailable);
            }
            _ => panic!("observation {observation}: the read took the wrong path"),
        }
    }
}

/// #1627 gate 1. A complete command frame under the retired message ID `0x0006`
/// (`AUTOMATION_ENQUEUE`), carrying the payload the command had, is refused at decode as an
/// unsupported message: nothing is written, the revision, the replay cache and every queue stay as
/// they were, and the request ID is not consumed (the next frame with that ID is served). The
/// capabilities record lists neither the ID nor flag bit 5.
///
/// Red if the retired ID dispatches again, if the refusal spends state before it fails (an ack
/// before a drop), or if a registry advertises the command or its flag.
#[test]
fn a_frame_under_the_retired_automation_id_is_refused_before_any_state_changes() {
    let row = conformance::retired_code_rows()
        .into_iter()
        .find(|row| row.name == "retired.message_id_0x0006")
        .expect("the conformance rows carry the retired automation ID");
    assert!(
        row.bytes.len() > crate::OUTER_HEADER_BYTES,
        "a former payload"
    );
    let mut endpoint = controller(8);
    let reports = |endpoint: &mut ProtocolController<MockProvider>| {
        [
            crate::QueueKind::ControlCommand,
            crate::QueueKind::ReliableResponse,
            crate::QueueKind::ReliableEvent,
            crate::QueueKind::Telemetry,
        ]
        .map(|kind| endpoint.queues_mut().report(kind))
    };
    let before = reports(&mut endpoint);
    let snapshot = endpoint.session().canonical_snapshot().to_owned();
    let mut output = [0x5a_u8; 2048];
    let refusal = endpoint
        .process_command_frame_into(
            &row.bytes,
            &mut DecodeScratch::new(&mut [0_u16; 32]),
            &mut output,
        )
        .expect_err("the retired ID is refused");
    let CommandFrameProcessError::Uncorrelatable(error) = refusal else {
        panic!("refused for another reason: {refusal:?}");
    };
    assert_eq!(error.status(), StatusCode::UnsupportedMessage);
    assert_eq!(output, [0x5a; 2048], "nothing is written");
    assert_eq!(endpoint.session().revision(), SessionRevision(7));
    assert_eq!(endpoint.session().canonical_snapshot(), snapshot);
    assert!(endpoint.replay().is_empty(), "no replay entry is spent");
    assert_eq!(reports(&mut endpoint), before, "no queue moved");

    let response = endpoint.process(capability(1, b"after the refusal"));
    assert_eq!(response.status, StatusCode::Ok, "request ID 1 is unspent");
    let decoded = ProtocolCodec::default()
        .decode_capabilities(response.payload(), 23)
        .expect("a capabilities record that passes the wire validator");
    let commands = decoded
        .supported_commands
        .as_chunks::<2>()
        .0
        .iter()
        .map(|id| u16::from_le_bytes([id[0], id[1]]))
        .collect::<Vec<_>>();
    assert!(!commands.contains(&6), "{commands:?}");
    assert_eq!(decoded.flags.0 & (1 << 5), 0, "flag bit 5");
}

fn response_request(points: u32) -> crate::ResponseGetRequest {
    crate::ResponseGetRequest {
        target: crate::ResponseTarget::Configuration(crate::ResponseConfiguration::InputFilters),
        grid: crate::ResponseGrid::Linear,
        points,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: crate::ParameterChannel::Both,
        sections: true,
        overrides: Vec::new(),
        configuration_id: Some(5),
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    }
}

/// Gate 3, 4 and 5 at the protocol layer: a `RESPONSE_GET` reaches the provider through a typed
/// frame and answers its curves; a provider refusal is its status with one error diagnostic that
/// carries its code; a reply past the response bound is `LIMIT_EXCEEDED`
/// (`response.reply_too_large`); a disabled provider is `UNAVAILABLE` and unadvertised; the same
/// request ID and bytes replay byte for byte and never reach the provider again; and a read leaves
/// the revision where it was.
///
/// Red if the request is dropped between frame and provider, a refusal loses its code, an
/// oversized reply is cached or sent, a replayed read runs twice, or a read moves the revision.
#[test]
fn response_get_dispatches_refuses_and_replays_through_the_controller() {
    let mut endpoint = controller(8);
    let revision = endpoint.session().revision();
    let ask = |endpoint: &mut ProtocolController<MockProvider>,
               request_id: u64,
               request: &crate::ResponseGetRequest| {
        let input = full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::ResponseGet(request),
        );
        (input.clone(), process_full_command(endpoint, &input))
    };
    let (_, output) = ask(&mut endpoint, 1, &response_request(16));
    let crate::DecodedTypedResponseFrame::Success {
        header,
        payload:
            crate::DecodedSuccessResponsePayload::ResponseResult(crate::ResponseResult::Curves(curves)),
    } = decode_response(&output)
    else {
        panic!("expected curves");
    };
    assert_eq!(header.message_id, MessageId::ResponseGet);
    assert_eq!(
        (
            curves.frequencies_hz.len(),
            curves.configuration_id,
            curves.section_count
        ),
        (16, 5, 2)
    );
    assert_eq!(curves.sections_left_db.as_ref().map(Vec::len), Some(32));

    // A refusal keeps its status and its code.
    let mut address = response_request(16);
    address.target = crate::ResponseTarget::Strip("vocal".to_owned());
    address.sections = false;
    let (_, output) = ask(&mut endpoint, 2, &address);
    let crate::DecodedTypedResponseFrame::NonOk { header, payload } = decode_response(&output)
    else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::Unavailable);
    assert_eq!(payload.diagnostics.len(), 1);
    assert_eq!(payload.diagnostics[0].code, "response.target.unavailable");
    assert_eq!(
        payload.diagnostics[0].severity,
        crate::DiagnosticSeverity::Error
    );
    let (_, output) = ask(&mut endpoint, 3, &response_request(1));
    let crate::DecodedTypedResponseFrame::NonOk { header, payload } = decode_response(&output)
    else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::InvalidField);
    assert_eq!(payload.diagnostics[0].code, "response.points");

    // A reply past the response bound is refused whole, with its code.
    let (_, output) = ask(&mut endpoint, 4, &response_request(200));
    let crate::DecodedTypedResponseFrame::NonOk { header, payload } = decode_response(&output)
    else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::LimitExceeded);
    assert_eq!(payload.diagnostics[0].code, "response.reply_too_large");

    // The same request ID and bytes replay the same bytes and never reach the provider again.
    let calls = endpoint.provider().response_calls();
    let (input, first) = ask(&mut endpoint, 5, &response_request(8));
    assert_eq!(endpoint.provider().response_calls(), calls + 1);
    let replayed = process_full_command(&mut endpoint, &input);
    assert_eq!(replayed, first);
    assert_eq!(endpoint.provider().response_calls(), calls + 1);
    assert_eq!(endpoint.session().revision(), revision);

    // A disabled provider is unavailable and unadvertised.
    endpoint.set_provider_features(ProviderFeatures {
        response: false,
        ..ProviderFeatures::ALL
    });
    let (_, output) = ask(&mut endpoint, 6, &response_request(8));
    let crate::DecodedTypedResponseFrame::NonOk { header, .. } = decode_response(&output) else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::Unavailable);
}

fn response_ready_event(tag: u64) -> crate::ResponseReady {
    crate::ResponseReady {
        sequence: 0,
        request_tag: tag,
        strip_id: "vocal".to_owned(),
        outcome: crate::ResponseReadyOutcome::Refused {
            status: StatusCode::NotFound,
            diagnostic: "response.strip.removed".to_owned(),
        },
    }
}

/// #1578 D3 and D4 at the protocol layer: `enqueue_response_ready_event` hands its event back
/// unchanged, with no sequence consumed, when the reliable queue is full, a structural transaction
/// is outstanding or the one cell is held;
/// a queued event stays in the cell across a short output buffer and leaves it only when its
/// complete frame (with the endpoint's next sequence) is delivered; and while the cell is held a
/// strip `RESPONSE_GET` is `BACKPRESSURE` `response.capture.pending` with the capture-slot record
/// and never reaches the provider, while a configuration request is still served.
///
/// Red if a refused enqueue loses or alters the event (an ack that precedes a drop), the cell is
/// emptied before delivery or never emptied, a second event overwrites a held one, or a strip
/// request is armed while an event is unread.
#[test]
fn a_response_ready_event_is_held_until_delivered_and_holds_off_a_strip_request() {
    let mut endpoint = controller(8);
    let revision = endpoint.session().revision();
    let mut output = vec![0_u8; 4096];
    let strip_request = || {
        let mut request = response_request(8);
        request.target = crate::ResponseTarget::Strip("vocal".to_owned());
        request.sections = false;
        request
    };
    let ask = |endpoint: &mut ProtocolController<MockProvider>,
               request_id: u64,
               request: &crate::ResponseGetRequest| {
        process_full_command(
            endpoint,
            &full_command(
                request_id,
                ExpectedRevision::Any,
                crate::CommandPayload::ResponseGet(request),
            ),
        )
    };

    // The one reliable event slot is full: the event comes back unchanged.
    endpoint
        .queues
        .try_enqueue_event(ReliableSlot::transport_state(
            revision,
            1,
            crate::TransportState::Stopped,
            SampleTime(0),
            None,
        ))
        .expect("room for one event");
    let sequence = endpoint.next_reliable_event_sequence;
    let refused = endpoint
        .enqueue_response_ready_event(revision, response_ready_event(1))
        .expect_err("a full queue");
    assert!(matches!(
        refused.reason,
        EventEgressError::ReliableQueueFull(_)
    ));
    assert_eq!(refused.event, response_ready_event(1));
    assert!(!endpoint.response_ready_pending());
    assert_eq!(endpoint.next_reliable_event_sequence, sequence);
    assert!(
        endpoint
            .dequeue_reliable_event_frame_into(&mut output)
            .expect("the transport event")
            .is_some()
    );

    // A structural transaction outstanding (an odd generation) refuses it too and hands it back.
    endpoint.structural_generation.store(1, Ordering::Release);
    let refused = endpoint
        .enqueue_response_ready_event(revision, response_ready_event(1))
        .expect_err("a structural transaction is outstanding");
    assert!(matches!(
        refused.reason,
        EventEgressError::ReliableQueueFull(_)
    ));
    assert_eq!(refused.event, response_ready_event(1));
    assert!(!endpoint.response_ready_pending());
    endpoint.structural_generation.store(0, Ordering::Release);

    endpoint
        .enqueue_response_ready_event(revision, response_ready_event(1))
        .expect("queued");
    assert!(endpoint.response_ready_pending());
    assert_eq!(endpoint.next_reliable_event_sequence, sequence + 1);
    // A held cell refuses a second event and hands it back.
    let refused = endpoint
        .enqueue_response_ready_event(revision, response_ready_event(2))
        .expect_err("the cell is held");
    assert_eq!(refused.reason, EventEgressError::ResponseReadyOccupied);
    assert_eq!(refused.event, response_ready_event(2));

    // While the cell is held, a strip request is refused before the provider sees it.
    let calls = endpoint.provider().response_calls();
    let crate::DecodedTypedResponseFrame::NonOk { header, payload } =
        decode_response(&ask(&mut endpoint, 1, &strip_request()))
    else {
        panic!("expected a refusal");
    };
    assert_eq!(header.status, StatusCode::Backpressure);
    assert_eq!(payload.diagnostics[0].code, "response.capture.pending");
    assert_eq!(
        payload.backpressure,
        Some(super::RESPONSE_CAPTURE_BACKPRESSURE)
    );
    assert_eq!(endpoint.provider().response_calls(), calls);
    // A configuration request is no capture and is served.
    assert!(matches!(
        decode_response(&ask(&mut endpoint, 2, &response_request(8))),
        crate::DecodedTypedResponseFrame::Success { .. }
    ));

    // A short buffer keeps the event; a complete frame delivers it and empties the cell.
    assert!(matches!(
        endpoint.dequeue_reliable_event_frame_into(&mut [0_u8; 8]),
        Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
    ));
    assert!(endpoint.response_ready_pending());
    let length = endpoint
        .dequeue_reliable_event_frame_into(&mut output)
        .expect("encode")
        .expect("the event");
    let mut fields = [0_u16; 64];
    let decoded = ProtocolCodec::default()
        .decode_typed_event(&output[..length], &mut DecodeScratch::new(&mut fields))
        .expect("an event");
    assert_eq!(decoded.header.message_id, MessageId::ResponseReady);
    assert_eq!(decoded.header.revision, revision);
    let crate::DecodedEventPayload::ResponseReady(event) = decoded.payload else {
        panic!("expected RESPONSE_READY");
    };
    assert_eq!(
        event,
        crate::ResponseReady {
            sequence,
            ..response_ready_event(1)
        }
    );
    assert!(!endpoint.response_ready_pending());
    assert!(
        endpoint
            .dequeue_reliable_event_frame_into(&mut output)
            .expect("empty")
            .is_none()
    );
    // The cell is free: the strip request reaches the provider again.
    let crate::DecodedTypedResponseFrame::NonOk { header, .. } =
        decode_response(&ask(&mut endpoint, 3, &strip_request()))
    else {
        panic!("expected the provider's refusal");
    };
    assert_eq!(header.status, StatusCode::Unavailable);
    assert_eq!(endpoint.provider().response_calls(), calls + 2);
}

/// Gate 4 (#1573), and #1578 gate 4's capability half: bit 14 means `RESPONSE_GET` (`0x000e`) and
/// its `RESPONSE_READY` event (`0x8040`) together. The registry lists both with the bit, and a
/// peer's capability frame that lists one without the other, or either without the bit, or the bit
/// without them, is refused.
///
/// Red if the registry stops listing `0x8040` with `0x000e`, or the invariant check stops tying
/// the flag, the command and the event to each other.
#[test]
fn a_capability_frame_must_tie_bit_14_to_the_response_command_and_event() {
    let codec = ProtocolCodec::default();
    let mut endpoint = controller(8);
    let response = endpoint.process(capability(1, b"tie"));
    assert!(codec.decode_capabilities(response.payload(), 23).is_ok());
    // Re-encode the registry with one side of the tie removed.
    let decoded = codec
        .decode_capabilities(response.payload(), 23)
        .expect("registry");
    let commands = decoded
        .supported_commands
        .as_chunks::<2>()
        .0
        .iter()
        .map(|id| u16::from_le_bytes([id[0], id[1]]))
        .collect::<Vec<_>>();
    let events = decoded
        .supported_events
        .as_chunks::<2>()
        .0
        .iter()
        .map(|id| u16::from_le_bytes([id[0], id[1]]))
        .collect::<Vec<_>>();
    assert!(commands.contains(&0x000e) && events.contains(&0x8040));
    let rebuilt = |commands: &[u16], events: &[u16], flags: u64| {
        let value = crate::Capabilities {
            minimum_version: decoded.minimum_version,
            maximum_version: decoded.maximum_version,
            maximum_frame_bytes: decoded.maximum_frame_bytes,
            maximum_tlvs: decoded.maximum_tlvs,
            maximum_string_bytes: decoded.maximum_string_bytes,
            maximum_nesting: decoded.maximum_nesting,
            control_command_slots: decoded.control_command_slots,
            control_command_bytes: decoded.control_command_bytes,
            reliable_response_slots: decoded.reliable_response_slots,
            reliable_event_slots: decoded.reliable_event_slots,
            telemetry_slots: decoded.telemetry_slots,
            replay_entries: decoded.replay_entries,
            replay_bytes: decoded.replay_bytes,
            maximum_cached_response_bytes: decoded.maximum_cached_response_bytes,
            maximum_control_page_items: decoded.maximum_control_page_items,
            maximum_diagnostic_page_items: decoded.maximum_diagnostic_page_items,
            maximum_telemetry_handles: decoded.maximum_telemetry_handles,
            maximum_transaction_edits: decoded.maximum_transaction_edits,
            supported_commands: commands,
            supported_events: events,
            flags: crate::CapabilityFlags(flags),
        };
        let mut payload = vec![0_u8; 4096];
        match codec.encode_capabilities(&value, &mut payload) {
            Ok(length) => {
                payload.truncate(length);
                Ok(payload)
            }
            Err(error) => Err(error),
        }
    };
    let flags = decoded.flags.0;
    let cleared = flags & !(1 << 14);
    assert!(flags & (1 << 14) != 0);
    assert!(rebuilt(&commands, &events, flags).is_ok());
    let without_command = commands
        .iter()
        .copied()
        .filter(|id| *id != 0x000e)
        .collect::<Vec<_>>();
    let without_event = events
        .iter()
        .copied()
        .filter(|id| *id != 0x8040)
        .collect::<Vec<_>>();
    assert!(rebuilt(&without_command, &events, flags).is_err());
    assert!(rebuilt(&commands, &without_event, flags).is_err());
    assert!(rebuilt(&without_command, &events, cleared).is_err());
    assert!(rebuilt(&commands, &without_event, cleared).is_err());
    assert!(rebuilt(&commands, &events, cleared).is_err());
    assert!(rebuilt(&without_command, &without_event, flags).is_err());
    assert!(rebuilt(&without_command, &without_event, cleared).is_ok());
}

/// #1594 gate 2 (spec gate 7): a `SPECTRUM_CONFIGURE` frame is handed to the session as
/// `PreparedCommandFrame::Session`, its completion writes the reply and the replay entry in one
/// call, and an exact replay of the frame returns the same bytes as an immediate reply, so the
/// session never executes it again. A dropped token records nothing, so the same request is new
/// again; a refusal is recorded and replayed like a reply.
///
/// Red if a replayed configure reaches the session a second time, or a reply is written without its
/// replay entry.
#[test]
fn a_session_command_completes_once_and_replays_its_exact_bytes() {
    let request = crate::SpectrumConfigureRequest {
        form: crate::SpectrumForm::Single,
        entries: vec![crate::SpectrumEntry {
            target_kind: crate::SpectrumTargetKind::TrackPostPan,
            target_id: "t1".to_owned(),
            channels: crate::ParameterChannel::Both,
        }],
        maximum_capture_bytes: 1 << 16,
        hop_frames: None,
    };
    let frame = |request_id| {
        full_command(
            request_id,
            ExpectedRevision::Any,
            crate::CommandPayload::SpectrumConfigure(&request),
        )
    };
    let prepare = |endpoint: &mut ProtocolController<MockProvider>, input: &[u8]| {
        endpoint
            .prepare_command_frame(
                input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare")
    };
    let mut endpoint = controller(8);
    let input = frame(1);

    // A dropped token writes nothing: the request is still new.
    let PreparedCommandFrame::Session(dropped) = prepare(&mut endpoint, &input) else {
        panic!("a configure is executed by the session");
    };
    assert_eq!(
        dropped.command(),
        &crate::SessionCommand::SpectrumConfigure(request.clone())
    );
    drop(dropped);

    let PreparedCommandFrame::Session(prepared) = prepare(&mut endpoint, &input) else {
        panic!("a dropped token left the request new");
    };
    // While the token is outstanding, every other entry point refuses.
    assert!(matches!(
        endpoint.prepare_command_frame(
            &frame(2),
            &[],
            &mut DecodeScratch::new(&mut [0_u16; 1024]),
            2048
        ),
        Err(CommandFrameProcessError::PreparedCommandOutstanding)
    ));
    let reply = crate::SpectrumConfigured {
        revision: endpoint.session().revision().0,
        configured_epoch: 1,
        adopted_epoch: 0,
        rebuild: true,
        plan_epoch: Some(1),
    };
    let mut output = [0_u8; 2048];
    let written = endpoint
        .complete_session_command(
            *prepared,
            crate::SessionCommandReply::SpectrumConfigured(reply),
            &mut output,
        )
        .expect("complete");
    let first = output[..written].to_vec();
    let codec = ProtocolCodec::default();
    let decoded = codec
        .decode_typed_response(&first, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("reply");
    let crate::DecodedTypedResponseFrame::Success {
        header,
        payload: crate::DecodedSuccessResponsePayload::SpectrumConfigured(decoded),
    } = decoded
    else {
        panic!("a spectrum reply");
    };
    assert_eq!(header.message_id, MessageId::SpectrumConfigure);
    assert_eq!(decoded, reply);

    // The exact replay answers the same bytes and hands nothing to the session.
    let PreparedCommandFrame::Immediate(replayed) = prepare(&mut endpoint, &input) else {
        panic!("a replayed configure reached the session again");
    };
    let mut again = [0_u8; 2048];
    let length = replayed.write_into(&mut again).expect("replayed bytes");
    assert_eq!(&again[..length], first.as_slice());

    // A refusal is recorded like a reply.
    let refused_input = frame(3);
    let PreparedCommandFrame::Session(refused) = prepare(&mut endpoint, &refused_input) else {
        panic!("a new configure is executed by the session");
    };
    let written = endpoint
        .complete_session_command_refused(
            *refused,
            StatusCode::NotFound,
            "spectrum.target.unknown",
            &mut output,
        )
        .expect("refuse");
    let refusal = output[..written].to_vec();
    let crate::DecodedTypedResponseFrame::NonOk { header, payload } = codec
        .decode_typed_response(&refusal, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("refusal")
    else {
        panic!("a non-OK reply");
    };
    assert_eq!(header.status, StatusCode::NotFound);
    assert_eq!(payload.diagnostics[0].code, "spectrum.target.unknown");
    let PreparedCommandFrame::Immediate(replayed) = prepare(&mut endpoint, &refused_input) else {
        panic!("a replayed refusal reached the session again");
    };
    let length = replayed.write_into(&mut again).expect("replayed bytes");
    assert_eq!(&again[..length], refusal.as_slice());

    // The one-call path has no session to run it on.
    let one_call = process_full_command(&mut endpoint, &frame(4));
    let header = codec.decode_header(&one_call).expect("header");
    assert_eq!(
        header.header.response().expect("response").status,
        StatusCode::Unavailable
    );
}

/// #1586 D6: an `OBSERVATION_READ` completes only with the request it prepared. Completed with a
/// request of the same length and request ID but another selection, or with a longer request, it
/// answers `Internal` without panicking and leaves the replay cache empty; the prepared request
/// then completes once and is cached.
///
/// Red if the completion records a replay entry for bytes the read was not planned for (or panics
/// on a longer request's reservation).
#[test]
fn an_observation_read_completes_only_with_its_own_request() {
    let read = |strip: &str, tap_id| crate::ObservationReadRequest {
        selections: vec![crate::ObservationReadSelection {
            strip_id: session::StableId::parse(strip).expect("strip"),
            section: controls::TapSection::Inserts(session::StableId::parse("comp").expect("id")),
            tap_id,
            channels: crate::ParameterChannel::Both,
        }],
    };
    let frame = |request: &crate::ObservationReadRequest| {
        full_command(
            7,
            ExpectedRevision::Any,
            crate::CommandPayload::ObservationRead(request),
        )
    };
    let input = frame(&read("t1", 1));
    let same_length = frame(&read("t1", 2));
    let longer = frame(&read("a-much-longer-strip-id", 1));
    assert_eq!(input.len(), same_length.len());
    assert!(longer.len() > input.len());
    let page = crate::ObservationReadPage {
        observed_sample: 0,
        rows: std::borrow::Cow::Owned(vec![crate::ObservationReadRow {
            status: crate::ObservationRowStatus::Unarmed,
            armed: false,
            arm_sample: 0,
            arm_tag: 0,
            window: None,
            left: None,
            right: None,
            unit: crate::ParameterUnit::Db,
        }]),
    };
    let mut endpoint = controller(8);
    let prepare = |endpoint: &mut ProtocolController<MockProvider>| {
        let PreparedCommandFrame::ObservationRead(prepared) = endpoint
            .prepare_command_frame(
                &input,
                &[],
                &mut DecodeScratch::new(&mut [0_u16; 1024]),
                2048,
            )
            .expect("prepare")
        else {
            panic!("the session executes the read");
        };
        prepared
    };
    let mut output = [0_u8; 2048];
    for other in [&same_length, &longer] {
        let prepared = prepare(&mut endpoint);
        assert!(matches!(
            endpoint.complete_observation_read(prepared, other, page.clone(), &mut output),
            Err(CommandFrameProcessError::Internal)
        ));
        assert!(endpoint.replay().is_empty(), "no entry for another request");
    }
    let prepared = prepare(&mut endpoint);
    assert!(
        endpoint
            .complete_observation_read(prepared, &input, page, &mut output)
            .is_ok()
    );
    assert_eq!(endpoint.replay().len(), 1);
}
