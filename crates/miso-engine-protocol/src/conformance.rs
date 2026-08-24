//! Deterministic complete schema corpus shared by native and Wasm conformance runners.

use crate::*;

/// A canonical complete BTLV frame and the exact typed decoder that owns it.
pub struct ConformanceFrame {
    /// Stable review label.
    pub name: &'static str,
    /// Canonical BTLV bytes.
    pub bytes: Vec<u8>,
    /// Decoder family required for this frame.
    pub decoder: ConformanceDecoder,
}

/// The schema-closed decoder selected by one corpus frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceDecoder {
    /// Full typed command decoder.
    Command,
    /// Full typed response decoder.
    Response,
    /// Full typed event decoder.
    Event,
    /// Full typed session-transaction decoder.
    Transaction,
}

/// Build every command, successful response, registered non-OK status, event, and all-opcode
/// session transaction using only public typed encoder entry points.
#[must_use]
pub fn complete_schema_corpus() -> Vec<ConformanceFrame> {
    let codec = ProtocolCodec::default();
    let request = RequestId::new(1).expect("literal request ID");
    let revision = SessionRevision(7);
    let mut frames = Vec::new();
    let transaction_edits = complete_all_opcode_fixture();
    let state = ParameterStateRequest { handles: vec![1] };
    let automation = [AutomationRecord {
        kind: AutomationKind::Point,
        handle: ParameterHandle(1),
        start: SampleTime(1),
        end: SampleTime(1),
        start_value: 0.0,
        end_value: 0.0,
    }];
    let telemetry = TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    };
    let counters = CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    macro_rules! command {
        ($name:literal, $payload:expr) => {
            push_command(
                &mut frames,
                &codec,
                $name,
                TypedCommandFrame {
                    request_id: request,
                    expected_revision: if matches!(
                        &$payload,
                        CommandPayload::CapabilitiesGet
                            | CommandPayload::SessionSnapshotGet(_)
                            | CommandPayload::ParameterMetadataGet(_)
                            | CommandPayload::ParameterStateGet(_)
                            | CommandPayload::TransportGet
                            | CommandPayload::CountersGet(_)
                            | CommandPayload::DiagnosticsGet(_)
                    ) {
                        ExpectedRevision::Any
                    } else {
                        ExpectedRevision::Exact(revision)
                    },
                    payload: $payload,
                },
            );
        };
    }
    command!("command.capabilities_get", CommandPayload::CapabilitiesGet);
    command!(
        "command.session_snapshot_get",
        CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
            offset: 0,
            maximum_bytes: 1
        })
    );
    command!(
        "command.session_transaction_apply",
        CommandPayload::SessionTransactionApply(&transaction_edits)
    );
    command!(
        "command.parameter_metadata_get",
        CommandPayload::ParameterMetadataGet(ParameterMetadataRequest {
            after_handle: 0,
            limit: 1
        })
    );
    command!(
        "command.parameter_state_get",
        CommandPayload::ParameterStateGet(&state)
    );
    command!(
        "command.automation_enqueue",
        CommandPayload::AutomationEnqueue(AutomationEnqueue {
            records: &automation
        })
    );
    command!("command.transport_get", CommandPayload::TransportGet);
    command!(
        "command.transport_set",
        CommandPayload::TransportSet(TransportSetRequest {
            state: TransportState::Playing,
            position: Some(SampleTime(9))
        })
    );
    command!(
        "command.telemetry_configure",
        CommandPayload::TelemetryConfigure(&telemetry)
    );
    command!(
        "command.counters_get",
        CommandPayload::CountersGet(&counters)
    );
    command!(
        "command.diagnostics_get",
        CommandPayload::DiagnosticsGet(DiagnosticsRequest {
            after_sequence: 0,
            limit: 1,
            minimum_severity: DiagnosticSeverity::Info
        })
    );
    command!(
        "command.nudge_effect_param",
        CommandPayload::NudgeEffectParam(NudgeEffectParam {
            parameter_handle: 1,
            size: NudgeSize::Md,
            count: -3,
        })
    );

    let capabilities = Capabilities {
        minimum_version: ProtocolVersion::V1,
        maximum_version: ProtocolVersion::CURRENT,
        maximum_frame_bytes: 4096,
        maximum_tlvs: 1024,
        maximum_string_bytes: 1024,
        maximum_nesting: 4,
        maximum_automation_records: 256,
        control_command_slots: 1,
        control_command_bytes: 64,
        automation_batch_slots: 1,
        reliable_response_slots: 1,
        reliable_event_slots: 1,
        telemetry_slots: 1,
        replay_entries: 1,
        replay_bytes: 64,
        maximum_cached_response_bytes: 64,
        per_block_automation_density: 1,
        admission_quantum_frames: 1,
        maximum_parameter_page_items: 256,
        maximum_diagnostic_page_items: 256,
        maximum_telemetry_handles: 256,
        maximum_transaction_edits: 64,
        supported_commands: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        supported_events: &[0x8001, 0x8002, 0x8010, 0x8020, 0x8021, 0x8030],
        flags: CapabilityFlags::B4_BASE,
    };
    macro_rules! success {
        ($name:literal, $payload:expr) => {
            push_success(
                &mut frames,
                &codec,
                $name,
                TypedSuccessResponseFrame {
                    request_id: request,
                    revision,
                    payload: $payload,
                },
            );
        };
    }
    success!(
        "response.capabilities",
        SuccessResponsePayload::Capabilities(capabilities)
    );
    success!(
        "response.session_snapshot",
        SuccessResponsePayload::SessionSnapshot(SessionSnapshot {
            total_bytes: 0,
            offset: 0,
            canonical_toml_chunk: &[],
            eof: true
        })
    );
    success!(
        "response.session_transaction",
        SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
            applied_operations: 42
        })
    );
    success!(
        "response.parameter_metadata",
        SuccessResponsePayload::ParameterMetadata(ParameterMetadataPage {
            last_handle: 0,
            eof: true,
            descriptors: Vec::new()
        })
    );
    success!(
        "response.parameter_state",
        SuccessResponsePayload::ParameterState(ParameterStatePage {
            observed_sample: 0,
            records: Vec::new()
        })
    );
    success!(
        "response.automation",
        SuccessResponsePayload::AutomationEnqueued(AutomationEnqueued {
            accepted_records: 1,
            occupancy: 0,
            capacity: 1,
            generation: 1
        })
    );
    success!(
        "response.transport_get",
        SuccessResponsePayload::TransportGetSnapshot(TransportSnapshot {
            state: TransportState::Stopped,
            position: SampleTime(0),
            effective_sample: SampleTime(0)
        })
    );
    success!(
        "response.transport_set",
        SuccessResponsePayload::TransportSetSnapshot(TransportSnapshot {
            state: TransportState::Playing,
            position: SampleTime(9),
            effective_sample: SampleTime(9)
        })
    );
    success!(
        "response.telemetry",
        SuccessResponsePayload::TelemetryConfiguration(telemetry.clone())
    );
    success!(
        "response.counters",
        SuccessResponsePayload::CounterSnapshot(CounterSnapshot {
            observed_sample: SampleTime(0),
            values: Vec::new()
        })
    );
    success!(
        "response.diagnostics",
        SuccessResponsePayload::DiagnosticsPage(DiagnosticsPage {
            last_sequence: 0,
            eof: true,
            diagnostics: Vec::new()
        })
    );
    success!(
        "response.nudge_effect_param",
        SuccessResponsePayload::EffectParamNudged(EffectParamNudged {
            parameter_handle: 1,
            resolved_value: 0.25,
        })
    );
    for status in [
        StatusCode::MalformedFrame,
        StatusCode::UnsupportedVersion,
        StatusCode::UnsupportedMessage,
        StatusCode::UnknownRequiredField,
        StatusCode::InvalidField,
        StatusCode::LimitExceeded,
        StatusCode::RevisionConflict,
        StatusCode::RevisionExhausted,
        StatusCode::RequestIdReuse,
        StatusCode::ReplayExpired,
        StatusCode::Backpressure,
        StatusCode::ValidationFailed,
        StatusCode::NotFound,
        StatusCode::Unavailable,
        StatusCode::TimeInPast,
        StatusCode::AutomationOrder,
        StatusCode::PcmForbidden,
        StatusCode::Internal,
    ] {
        let payload = NonOkResponse {
            diagnostics: Vec::new(),
            omitted_diagnostics: 0,
            backpressure: (status == StatusCode::Backpressure).then_some(Backpressure {
                queue_kind: BackpressureQueueKind::ReplayCache,
                capacity: 1,
                occupancy: 0,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        };
        push_non_ok(
            &mut frames,
            &codec,
            status,
            TypedNonOkResponseFrame {
                request_id: request,
                revision,
                message_id: MessageId::CapabilitiesGet,
                status,
                payload: &payload,
            },
        );
    }
    let meter = [MeterRecord {
        handle: 1,
        component: MeterComponent::Left,
        flags: 1,
        value: 0.0,
    }];
    let diagnostic = Diagnostic {
        code: "protocol.conformance".to_owned(),
        severity: DiagnosticSeverity::Error,
        path: Vec::new(),
        detail: None,
        operation_index: None,
        sample_time: None,
        provider_sequence: Some(1),
    };
    macro_rules! event {
        ($name:literal, $payload:expr) => {
            push_event(
                &mut frames,
                &codec,
                $name,
                TypedEventFrame {
                    revision,
                    payload: $payload,
                },
            );
        };
    }
    event!(
        "event.session_committed",
        EventPayload::SessionCommitted(SessionCommitted {
            event_sequence: 1,
            origin_request_id: request,
            previous_revision: SessionRevision(6),
            applied_operations: 1
        })
    );
    event!(
        "event.automation_canceled",
        EventPayload::AutomationCanceled(AutomationCanceled {
            event_sequence: 1,
            origin_request_id: request,
            canceled_records: 1,
            reason: AutomationCancellationReason::RevisionChanged,
            queue_generation: 1,
            effective_sample: None
        })
    );
    event!(
        "event.transport_state",
        EventPayload::TransportState(TransportStateEvent {
            event_sequence: 1,
            state: TransportState::Stopped,
            position: SampleTime(0),
            effective_sample: SampleTime(0),
            origin_request_id: None
        })
    );
    event!(
        "event.meter_batch",
        EventPayload::MeterBatch(MeterBatch {
            observed_sample: SampleTime(0),
            records: &meter
        })
    );
    event!(
        "event.counter_snapshot",
        EventPayload::CounterSnapshot(CounterSnapshotRef {
            observed_sample: SampleTime(0),
            values: &[]
        })
    );
    event!("event.diagnostic", EventPayload::Diagnostic(&diagnostic));
    frames
}

fn output() -> Vec<u8> {
    vec![0; 65_536]
}
fn push_command(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedCommandFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("conformance command encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Command,
    });
}
fn push_success(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedSuccessResponseFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("conformance response encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Response,
    });
}
fn push_non_ok(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    status: StatusCode,
    frame: TypedNonOkResponseFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("conformance error encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name: status_name(status),
        bytes,
        decoder: ConformanceDecoder::Response,
    });
}
fn push_event(
    out: &mut Vec<ConformanceFrame>,
    codec: &ProtocolCodec,
    name: &'static str,
    frame: TypedEventFrame<'_>,
) {
    let mut bytes = output();
    let length = codec
        .encode_event_frame_into(&frame, &mut bytes)
        .expect("conformance event encodes");
    bytes.truncate(length);
    out.push(ConformanceFrame {
        name,
        bytes,
        decoder: ConformanceDecoder::Event,
    });
}
fn status_name(status: StatusCode) -> &'static str {
    match status {
        StatusCode::Ok => "response.ok",
        StatusCode::MalformedFrame => "response.malformed_frame",
        StatusCode::UnsupportedVersion => "response.unsupported_version",
        StatusCode::UnsupportedMessage => "response.unsupported_message",
        StatusCode::UnknownRequiredField => "response.unknown_required_field",
        StatusCode::InvalidField => "response.invalid_field",
        StatusCode::LimitExceeded => "response.limit_exceeded",
        StatusCode::RevisionConflict => "response.revision_conflict",
        StatusCode::RevisionExhausted => "response.revision_exhausted",
        StatusCode::RequestIdReuse => "response.request_id_reuse",
        StatusCode::ReplayExpired => "response.replay_expired",
        StatusCode::Backpressure => "response.backpressure",
        StatusCode::ValidationFailed => "response.validation_failed",
        StatusCode::NotFound => "response.not_found",
        StatusCode::Unavailable => "response.unavailable",
        StatusCode::TimeInPast => "response.time_in_past",
        StatusCode::AutomationOrder => "response.automation_order",
        StatusCode::PcmForbidden => "response.pcm_forbidden",
        StatusCode::Internal => "response.internal",
    }
}
