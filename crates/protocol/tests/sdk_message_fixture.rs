//! Issue #1571 gate 1: the byte fixture the SDK's protocol message codec
//! (`sdk/src/core/protocol-messages.ts`) is held to.
//!
//! The protocol codec writes one command frame per request the SDK encodes (`CAPABILITIES_GET`,
//! `SESSION_SNAPSHOT_GET`, `TRANSPORT_GET`, `TRANSPORT_SET`, `TELEMETRY_CONFIGURE`, `COUNTERS_GET`,
//! `DIAGNOSTICS_GET`), one success or refusal reply per reply the SDK decodes, and one frame of
//! each of the six events, with every optional field present once and absent once. The frames and
//! the values the SDK must decode from them are checked in as
//! `sdk/test/fixtures/protocol-messages.json`; `sdk/test/protocol-messages-evals.mjs` encodes the
//! same requests and compares bytes, and decodes every reply and event and compares values. The
//! fixture also lists every `CounterId` the codec accepts, by its camel-case name, which is the
//! SDK's counter name table. This is a wire format, so a byte comparison is the right gate.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate codec change.

use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    Capabilities, CapabilityFlags, CommandPayload, CounterId, CounterSnapshot, CounterSnapshotRef,
    CounterValue, CountersRequest, DecodeScratch, DecodedCommandPayload, Diagnostic,
    DiagnosticSeverity, DiagnosticsPage, DiagnosticsRequest, EventPayload, ExpectedRevision,
    MessageId, MeterBatch, MeterComponent, MeterEntry, MeterMetric, MeterRecord, MeterTarget,
    NonOkResponse, ObservationEnded, ObservationSection, ObservationTapKey, PathSegment,
    ProtocolCodec, ProtocolVersion, RequestId, SampleTime, SessionCommitted, SessionRevision,
    SessionSnapshot, SessionSnapshotRequest, StatusCode, SuccessResponsePayload,
    TelemetryConfiguration, TransportSetRequest, TransportSnapshot, TransportState,
    TransportStateEvent, TypedCommandFrame, TypedEventFrame, TypedNonOkResponseFrame,
    TypedSuccessResponseFrame,
};

const FIXTURE: &str = "../../sdk/test/fixtures/protocol-messages.json";

/// A JSON value, written in one fixed layout so the checked-in file is byte-stable.
enum J {
    Str(String),
    Num(u64),
    Float(f32),
    Bool(bool),
    Arr(Vec<J>),
    Obj(Vec<(&'static str, J)>),
}

/// A `u64` the SDK reads as a `bigint`: a decimal string.
fn big(value: u64) -> J {
    J::Str(value.to_string())
}

fn text(value: &str) -> J {
    J::Str(value.to_owned())
}

fn obj(fields: Vec<(&'static str, J)>) -> J {
    J::Obj(fields)
}

fn render_json(value: &J, out: &mut String) {
    match value {
        J::Str(text) => {
            out.push('"');
            for character in text.chars() {
                match character {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    _ => out.push(character),
                }
            }
            out.push('"');
        }
        J::Num(number) => write!(out, "{number}").expect("write to a string"),
        J::Float(number) => {
            assert!(number.is_finite(), "the fixture holds finite floats only");
            write!(out, "{number:?}").expect("write to a string");
        }
        J::Bool(flag) => out.push_str(if *flag { "true" } else { "false" }),
        J::Arr(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                render_json(item, out);
            }
            out.push(']');
        }
        J::Obj(fields) => {
            out.push('{');
            for (index, (key, item)) in fields.iter().enumerate() {
                out.push_str(if index > 0 { ", " } else { " " });
                write!(out, "\"{key}\": ").expect("write to a string");
                render_json(item, out);
            }
            out.push_str(if fields.is_empty() { "}" } else { " }" });
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut text, byte| {
        write!(text, "{byte:02x}").expect("write to a string");
        text
    })
}

fn request_id(value: u64) -> RequestId {
    RequestId::new(value).expect("nonzero request ID")
}

/// The SDK's name of a counter: the Rust variant in camel case. The match has no wildcard, so a new
/// variant is a compile error here until it has a name.
fn counter_name(id: CounterId) -> &'static str {
    match id {
        CounterId::ControlCommandBackpressure => "controlCommandBackpressure",
        CounterId::ReliableResponseBackpressure => "reliableResponseBackpressure",
        CounterId::ReliableEventBackpressure => "reliableEventBackpressure",
        CounterId::TelemetryCoalesced => "telemetryCoalesced",
        CounterId::TelemetryDropped => "telemetryDropped",
        CounterId::MalformedFrames => "malformedFrames",
        CounterId::ReplayHits => "replayHits",
        CounterId::RequestIdReuse => "requestIdReuse",
        CounterId::ReplayExpired => "replayExpired",
        CounterId::ValidationFailures => "validationFailures",
        CounterId::LiveValuesSuperseded => "liveValuesSuperseded",
        CounterId::TransitionReprepareRefusals => "transitionReprepareRefusals",
        CounterId::StripsRestarted => "stripsRestarted",
        CounterId::LiveBlockErrors => "liveBlockErrors",
        CounterId::SourceUnderrunBlocks => "sourceUnderrunBlocks",
        CounterId::SourceFramesDiscarded => "sourceFramesDiscarded",
        CounterId::MeterWindowsLost => "meterWindowsLost",
    }
}

fn severity_name(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Error => "error",
    }
}

fn state_name(state: TransportState) -> &'static str {
    match state {
        TransportState::Stopped => "stopped",
        TransportState::Playing => "playing",
    }
}

fn command_bytes(request: u64, revision: ExpectedRevision, payload: CommandPayload<'_>) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = TypedCommandFrame {
        request_id: request_id(request),
        expected_revision: revision,
        payload,
    };
    let mut bytes = vec![0_u8; 4096];
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("the command encodes");
    bytes.truncate(length);
    bytes
}

/// Every counter ID the codec accepts, found by decoding a `TELEMETRY_CONFIGURE` frame whose one
/// counter ID is each `u32` code in turn: the codec, not this file, says which codes exist.
fn counter_ids() -> Vec<CounterId> {
    let configuration = TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids: vec![CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    };
    let template = command_bytes(
        1,
        ExpectedRevision::Exact(SessionRevision(0)),
        CommandPayload::TelemetryConfigure(&configuration),
    );
    // The counter field: ID 3, wire `packed u32` (13), then its length and the one value.
    let at = template
        .windows(3)
        .position(|window| window == [3, 0, 13])
        .expect("the counter field")
        + 8;
    let codec = ProtocolCodec::default();
    let mut found = Vec::new();
    for code in 0..=u32::from(u16::MAX) {
        let mut frame = template.clone();
        frame[at..at + 4].copy_from_slice(&code.to_le_bytes());
        let mut ids = [0_u16; 64];
        if let Ok(decoded) = codec.decode_typed_command(&frame, &mut DecodeScratch::new(&mut ids))
            && let DecodedCommandPayload::TelemetryConfigure(configuration) = decoded.payload
        {
            assert_eq!(configuration.counter_ids.len(), 1);
            let id = configuration.counter_ids[0];
            assert_eq!(id as u32, code);
            found.push(id);
        }
    }
    assert!(!found.is_empty(), "the codec accepts some counter ID");
    found
}

/// A meter table entry as the SDK spells it (#1583 D1): the target, the strip ID of a strip target,
/// and the metric.
fn meter_entry_json(entry: &MeterEntry) -> J {
    let mut fields = Vec::new();
    match &entry.target {
        MeterTarget::Output => fields.push(("target", text("output"))),
        MeterTarget::Strip(id) => {
            fields.push(("target", text("strip")));
            fields.push(("stripId", text(id)));
        }
    }
    fields.push((
        "metric",
        text(match entry.metric {
            MeterMetric::Peak => "peak",
            MeterMetric::GainReduction => "gainReduction",
        }),
    ));
    obj(fields)
}

fn telemetry_json(value: &TelemetryConfiguration) -> J {
    let mut fields = vec![
        (
            "meters",
            J::Arr(value.meters.iter().map(meter_entry_json).collect()),
        ),
        (
            "meterPeriodBlocks",
            J::Num(u64::from(value.meter_period_blocks)),
        ),
    ];
    if let Some(blocks) = value.meter_window_blocks {
        fields.push(("meterWindowBlocks", J::Num(u64::from(blocks.get()))));
    }
    fields.extend([
        (
            "counterIds",
            J::Arr(
                value
                    .counter_ids
                    .iter()
                    .map(|id| text(counter_name(*id)))
                    .collect(),
            ),
        ),
        (
            "counterPeriodBlocks",
            J::Num(u64::from(value.counter_period_blocks)),
        ),
        ("diagnosticsEnabled", J::Bool(value.diagnostics_enabled)),
        (
            "minimumSeverity",
            text(severity_name(value.minimum_diagnostic_severity)),
        ),
    ]);
    obj(fields)
}

fn full_telemetry() -> TelemetryConfiguration {
    TelemetryConfiguration {
        meters: vec![
            MeterEntry {
                target: MeterTarget::Output,
                metric: MeterMetric::Peak,
            },
            MeterEntry {
                target: MeterTarget::Strip("vocal".into()),
                metric: MeterMetric::Peak,
            },
            MeterEntry {
                target: MeterTarget::Strip("vocal".into()),
                metric: MeterMetric::GainReduction,
            },
        ],
        meter_period_blocks: 24,
        meter_window_blocks: None,
        counter_ids: vec![
            CounterId::ControlCommandBackpressure,
            CounterId::LiveValuesSuperseded,
            CounterId::SourceFramesDiscarded,
        ],
        counter_period_blocks: 2,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: DiagnosticSeverity::Warning,
    }
}

fn empty_telemetry() -> TelemetryConfiguration {
    TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    }
}

struct Row {
    name: &'static str,
    /// What the SDK reads it with (a command or decoder name), and its arguments or results.
    fields: Vec<(&'static str, J)>,
    bytes: Vec<u8>,
}

fn row_json(row: Row) -> J {
    let mut fields = vec![("name", text(row.name))];
    fields.extend(row.fields);
    fields.push(("hex", J::Str(hex(&row.bytes))));
    obj(fields)
}

fn command(
    name: &'static str,
    message: &'static str,
    request: Vec<(&'static str, J)>,
    bytes: Vec<u8>,
) -> Row {
    Row {
        name,
        fields: vec![("message", text(message)), ("request", obj(request))],
        bytes,
    }
}

fn commands() -> Vec<Row> {
    let any = ExpectedRevision::Any;
    let exact = |revision| ExpectedRevision::Exact(SessionRevision(revision));
    let full = full_telemetry();
    let empty = empty_telemetry();
    vec![
        command(
            "capabilities_get",
            "capabilities",
            vec![("requestId", big(1))],
            command_bytes(1, any, CommandPayload::CapabilitiesGet),
        ),
        command(
            "snapshot_first_page",
            "sessionSnapshot",
            vec![
                ("requestId", big(2)),
                ("offset", big(0)),
                ("maximumBytes", J::Num(65_536)),
            ],
            command_bytes(
                2,
                any,
                CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                    offset: 0,
                    maximum_bytes: 65_536,
                }),
            ),
        ),
        command(
            "snapshot_continuation",
            "sessionSnapshot",
            vec![
                ("requestId", big(3)),
                ("expectedRevision", big(7)),
                ("offset", big(16_272)),
                ("maximumBytes", J::Num(16_272)),
            ],
            command_bytes(
                3,
                exact(7),
                CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                    offset: 16_272,
                    maximum_bytes: 16_272,
                }),
            ),
        ),
        command(
            "transport_get",
            "transportGet",
            vec![("requestId", big(4))],
            command_bytes(4, any, CommandPayload::TransportGet),
        ),
        command(
            "transport_set_playing",
            "transportSet",
            vec![
                ("requestId", big(5)),
                ("expectedRevision", big(9)),
                ("state", text("playing")),
            ],
            command_bytes(
                5,
                exact(9),
                CommandPayload::TransportSet(TransportSetRequest {
                    state: TransportState::Playing,
                }),
            ),
        ),
        command(
            "transport_set_stopped",
            "transportSet",
            vec![
                ("requestId", big(u64::MAX)),
                ("expectedRevision", big(u64::MAX - 1)),
                ("state", text("stopped")),
            ],
            command_bytes(
                u64::MAX,
                exact(u64::MAX - 1),
                CommandPayload::TransportSet(TransportSetRequest {
                    state: TransportState::Stopped,
                }),
            ),
        ),
        command(
            "telemetry_configure_full",
            "telemetryConfigure",
            vec![
                ("requestId", big(6)),
                ("expectedRevision", big(9)),
                ("configuration", telemetry_json(&full)),
            ],
            command_bytes(6, exact(9), CommandPayload::TelemetryConfigure(&full)),
        ),
        command(
            "telemetry_configure_empty",
            "telemetryConfigure",
            vec![
                ("requestId", big(7)),
                ("expectedRevision", big(0)),
                ("configuration", telemetry_json(&empty)),
            ],
            command_bytes(7, exact(0), CommandPayload::TelemetryConfigure(&empty)),
        ),
        command(
            "counters_all",
            "counters",
            vec![("requestId", big(8))],
            command_bytes(
                8,
                any,
                CommandPayload::CountersGet(&CountersRequest {
                    all: true,
                    ids: Vec::new(),
                }),
            ),
        ),
        command(
            "counters_some",
            "counters",
            vec![
                ("requestId", big(9)),
                (
                    "ids",
                    J::Arr(vec![
                        text("controlCommandBackpressure"),
                        text("liveValuesSuperseded"),
                        text("sourceFramesDiscarded"),
                    ]),
                ),
            ],
            command_bytes(
                9,
                any,
                CommandPayload::CountersGet(&CountersRequest {
                    all: false,
                    ids: vec![
                        CounterId::ControlCommandBackpressure as u32,
                        CounterId::LiveValuesSuperseded as u32,
                        CounterId::SourceFramesDiscarded as u32,
                    ],
                }),
            ),
        ),
        command(
            "diagnostics_defaults",
            "diagnostics",
            vec![
                ("requestId", big(10)),
                ("afterSequence", big(0)),
                ("limit", J::Num(64)),
                ("minimumSeverity", text("info")),
            ],
            command_bytes(
                10,
                any,
                CommandPayload::DiagnosticsGet(DiagnosticsRequest {
                    after_sequence: 0,
                    limit: 64,
                    minimum_severity: DiagnosticSeverity::Info,
                }),
            ),
        ),
        command(
            "diagnostics_after",
            "diagnostics",
            vec![
                ("requestId", big(11)),
                ("afterSequence", big((1 << 40) + 3)),
                ("limit", J::Num(256)),
                ("minimumSeverity", text("error")),
            ],
            command_bytes(
                11,
                any,
                CommandPayload::DiagnosticsGet(DiagnosticsRequest {
                    after_sequence: (1 << 40) + 3,
                    limit: 256,
                    minimum_severity: DiagnosticSeverity::Error,
                }),
            ),
        ),
    ]
}

fn path_json(segment: &PathSegment) -> J {
    match segment {
        PathSegment::Field(field) => obj(vec![("kind", text("field")), ("field", text(field))]),
        PathSegment::Index(index) => obj(vec![("kind", text("index")), ("index", big(*index))]),
        PathSegment::StableId(id) => obj(vec![("kind", text("stableId")), ("stableId", text(id))]),
    }
}

/// A diagnostic as the SDK reports it: an absent optional field is an absent key.
fn diagnostic_json(value: &Diagnostic) -> J {
    let mut fields = vec![
        ("code", text(&value.code)),
        ("severity", text(severity_name(value.severity))),
        ("path", J::Arr(value.path.iter().map(path_json).collect())),
    ];
    if let Some(detail) = &value.detail {
        fields.push(("detail", text(detail)));
    }
    if let Some(index) = value.operation_index {
        fields.push(("operationIndex", J::Num(u64::from(index))));
    }
    if let Some(sample) = value.sample_time {
        fields.push(("sampleTime", big(sample)));
    }
    if let Some(sequence) = value.provider_sequence {
        fields.push(("providerSequence", big(sequence)));
    }
    obj(fields)
}

/// Every optional field present, and a path of each segment kind.
fn full_diagnostic(sequence: u64) -> Diagnostic {
    Diagnostic {
        code: "source.underrun".to_owned(),
        severity: DiagnosticSeverity::Warning,
        path: vec![
            PathSegment::Field("tracks".to_owned()),
            PathSegment::Index(2),
            PathSegment::StableId("vocal".to_owned()),
        ],
        detail: Some("a detail".to_owned()),
        operation_index: Some(4),
        sample_time: Some((1 << 33) + 5),
        provider_sequence: Some(sequence),
    }
}

/// No optional field but the provider sequence events and pages require, and an empty path.
fn bare_diagnostic(sequence: Option<u64>) -> Diagnostic {
    Diagnostic {
        code: "render.block".to_owned(),
        severity: DiagnosticSeverity::Info,
        path: Vec::new(),
        detail: None,
        operation_index: None,
        sample_time: None,
        provider_sequence: sequence,
    }
}

fn success(
    name: &'static str,
    decoder: &'static str,
    request: u64,
    revision: u64,
    payload: SuccessResponsePayload<'_>,
    value: J,
) -> Row {
    let codec = ProtocolCodec::default();
    let frame = TypedSuccessResponseFrame {
        request_id: request_id(request),
        revision: SessionRevision(revision),
        payload,
    };
    let mut bytes = vec![0_u8; 4096];
    let length = codec
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("the reply encodes");
    bytes.truncate(length);
    Row {
        name,
        fields: vec![
            ("decoder", text(decoder)),
            (
                "expect",
                obj(vec![
                    ("ok", J::Bool(true)),
                    ("requestId", big(request)),
                    ("revision", big(revision)),
                    ("value", value),
                ]),
            ),
        ],
        bytes,
    }
}

fn refusal(
    name: &'static str,
    decoder: &'static str,
    message: MessageId,
    status: (StatusCode, &'static str),
    payload: &NonOkResponse,
) -> Row {
    let codec = ProtocolCodec::default();
    let frame = TypedNonOkResponseFrame {
        request_id: request_id(40),
        revision: SessionRevision(12),
        message_id: message,
        status: status.0,
        payload,
    };
    let mut bytes = vec![0_u8; 4096];
    let length = codec
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("the refusal encodes");
    bytes.truncate(length);
    Row {
        name,
        fields: vec![
            ("decoder", text(decoder)),
            (
                "expect",
                obj(vec![
                    ("ok", J::Bool(false)),
                    ("requestId", big(40)),
                    ("revision", big(12)),
                    ("status", text(status.1)),
                    (
                        "diagnostics",
                        J::Arr(payload.diagnostics.iter().map(diagnostic_json).collect()),
                    ),
                    (
                        "omittedDiagnostics",
                        J::Num(u64::from(payload.omitted_diagnostics)),
                    ),
                ]),
            ),
        ],
        bytes,
    }
}

fn capabilities_json(value: &Capabilities<'_>) -> J {
    let version = |version: ProtocolVersion| {
        obj(vec![
            ("major", J::Num(u64::from(version.major))),
            ("minor", J::Num(u64::from(version.minor))),
        ])
    };
    let ids = |ids: &[u16]| J::Arr(ids.iter().map(|id| J::Num(u64::from(*id))).collect());
    obj(vec![
        ("minimumVersion", version(value.minimum_version)),
        ("maximumVersion", version(value.maximum_version)),
        ("maximumFrameBytes", big(value.maximum_frame_bytes)),
        ("maximumTlvs", J::Num(u64::from(value.maximum_tlvs))),
        ("maximumStringBytes", big(value.maximum_string_bytes)),
        ("maximumNesting", J::Num(u64::from(value.maximum_nesting))),
        ("controlCommandSlots", big(value.control_command_slots)),
        ("controlCommandBytes", big(value.control_command_bytes)),
        ("reliableResponseSlots", big(value.reliable_response_slots)),
        ("reliableEventSlots", big(value.reliable_event_slots)),
        ("telemetrySlots", big(value.telemetry_slots)),
        ("replayEntries", big(value.replay_entries)),
        ("replayBytes", big(value.replay_bytes)),
        (
            "maximumCachedResponseBytes",
            big(value.maximum_cached_response_bytes),
        ),
        (
            "maximumControlPageItems",
            J::Num(u64::from(value.maximum_control_page_items)),
        ),
        (
            "maximumDiagnosticPageItems",
            J::Num(u64::from(value.maximum_diagnostic_page_items)),
        ),
        (
            "maximumTelemetryHandles",
            J::Num(u64::from(value.maximum_telemetry_handles)),
        ),
        (
            "maximumTransactionEdits",
            J::Num(u64::from(value.maximum_transaction_edits)),
        ),
        ("supportedCommands", ids(value.supported_commands)),
        ("supportedEvents", ids(value.supported_events)),
        ("flags", big(value.flags.0)),
    ])
}

const ALL_COMMANDS: [u16; 12] = [
    0x0001, 0x0002, 0x0003, 0x0007, 0x0008, 0x0009, 0x000a, 0x000b, 0x000c, 0x000d, 0x000e, 0x0013,
];
const ALL_EVENTS: [u16; 6] = [0x8001, 0x8010, 0x8011, 0x8020, 0x8021, 0x8030];

fn full_capabilities() -> Capabilities<'static> {
    Capabilities {
        minimum_version: ProtocolVersion { major: 1, minor: 0 },
        maximum_version: ProtocolVersion { major: 1, minor: 3 },
        maximum_frame_bytes: 16 << 10,
        maximum_tlvs: 4096,
        maximum_string_bytes: (1 << 36) + 1,
        maximum_nesting: 8,
        control_command_slots: 2,
        control_command_bytes: 1 << 20,
        reliable_response_slots: 3,
        reliable_event_slots: 4,
        telemetry_slots: 5,
        replay_entries: 6,
        replay_bytes: 7,
        maximum_cached_response_bytes: u64::MAX,
        maximum_control_page_items: 256,
        maximum_diagnostic_page_items: 255,
        maximum_telemetry_handles: 254,
        maximum_transaction_edits: 1024,
        supported_commands: &ALL_COMMANDS,
        supported_events: &ALL_EVENTS,
        // Every known bit: bits 0 to 14 but the retired bit 5.
        flags: CapabilityFlags(((1 << 15) - 1) & !(1 << 5)),
    }
}

fn minimal_capabilities() -> Capabilities<'static> {
    Capabilities {
        minimum_version: ProtocolVersion { major: 1, minor: 0 },
        maximum_version: ProtocolVersion { major: 1, minor: 0 },
        maximum_frame_bytes: 1,
        maximum_tlvs: 23,
        maximum_string_bytes: 0,
        maximum_nesting: 1,
        control_command_slots: 0,
        control_command_bytes: 0,
        reliable_response_slots: 0,
        reliable_event_slots: 0,
        telemetry_slots: 0,
        replay_entries: 0,
        replay_bytes: 0,
        maximum_cached_response_bytes: 0,
        maximum_control_page_items: 0,
        maximum_diagnostic_page_items: 0,
        maximum_telemetry_handles: 0,
        maximum_transaction_edits: 0,
        supported_commands: &[0x0001],
        supported_events: &[],
        flags: CapabilityFlags(0b111),
    }
}

fn snapshot_json(value: SessionSnapshot<'_>) -> J {
    obj(vec![
        ("totalBytes", big(value.total_bytes)),
        ("offset", big(value.offset)),
        ("chunk", J::Str(hex(value.canonical_json_chunk))),
        ("eof", J::Bool(value.eof)),
    ])
}

fn transport_json(value: TransportSnapshot) -> J {
    obj(vec![
        ("state", text(state_name(value.state))),
        ("effectiveSample", big(value.effective_sample.0)),
    ])
}

fn counters_json(value: &CounterSnapshot) -> J {
    obj(vec![
        ("observedSample", big(value.observed_sample.0)),
        (
            "values",
            J::Arr(
                value
                    .values
                    .iter()
                    .map(|row| {
                        obj(vec![
                            ("id", text(counter_name(row.id))),
                            ("value", big(row.value)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

fn page_json(value: &DiagnosticsPage) -> J {
    obj(vec![
        ("lastSequence", big(value.last_sequence)),
        ("eof", J::Bool(value.eof)),
        (
            "diagnostics",
            J::Arr(value.diagnostics.iter().map(diagnostic_json).collect()),
        ),
    ])
}

fn some_counters() -> CounterSnapshot {
    CounterSnapshot {
        observed_sample: SampleTime((1 << 35) + 128),
        values: vec![
            CounterValue {
                id: CounterId::ControlCommandBackpressure,
                value: 0,
            },
            CounterValue {
                id: CounterId::LiveValuesSuperseded,
                value: 3,
            },
            CounterValue {
                id: CounterId::SourceFramesDiscarded,
                value: u64::MAX,
            },
        ],
    }
}

fn replies() -> Vec<Row> {
    let full = full_capabilities();
    let minimal = minimal_capabilities();
    // A document chunk that splits a three-byte code point at its end.
    let chunk = "{\"name\":\"caf\u{e9}\u{20ac}".as_bytes();
    let first = SessionSnapshot {
        total_bytes: (chunk.len() + 3) as u64,
        offset: 0,
        canonical_json_chunk: &chunk[..chunk.len() - 1],
        eof: false,
    };
    let last = SessionSnapshot {
        total_bytes: (chunk.len() + 3) as u64,
        offset: (chunk.len() - 1) as u64,
        canonical_json_chunk: &[0xac, b'"', b'}', b'\n'],
        eof: true,
    };
    let stopped = TransportSnapshot {
        state: TransportState::Stopped,
        effective_sample: SampleTime(0),
    };
    let playing = TransportSnapshot {
        state: TransportState::Playing,
        effective_sample: SampleTime((1 << 40) + 64),
    };
    // The full echo carries the provider's meter window (#1583 D4); the empty one has none.
    let telemetry = TelemetryConfiguration {
        meter_window_blocks: core::num::NonZeroU32::new(12),
        ..full_telemetry()
    };
    let no_telemetry = empty_telemetry();
    let counters = some_counters();
    let no_counters = CounterSnapshot {
        observed_sample: SampleTime(0),
        values: Vec::new(),
    };
    let page = DiagnosticsPage {
        last_sequence: 9,
        eof: false,
        diagnostics: vec![bare_diagnostic(Some(8)), full_diagnostic(9)],
    };
    let empty_page = DiagnosticsPage {
        last_sequence: 0,
        eof: true,
        diagnostics: Vec::new(),
    };
    let invalid = NonOkResponse {
        diagnostics: Vec::new(),
        omitted_diagnostics: 0,
        backpressure: None,
    };
    let refused = NonOkResponse {
        diagnostics: vec![full_diagnostic(3), bare_diagnostic(None)],
        omitted_diagnostics: 2,
        backpressure: None,
    };
    vec![
        success(
            "capabilities_full",
            "capabilities",
            1,
            5,
            SuccessResponsePayload::Capabilities(full),
            capabilities_json(&full),
        ),
        success(
            "capabilities_minimal",
            "capabilities",
            2,
            0,
            SuccessResponsePayload::Capabilities(minimal),
            capabilities_json(&minimal),
        ),
        success(
            "snapshot_first_page",
            "sessionSnapshot",
            3,
            7,
            SuccessResponsePayload::SessionSnapshot(first),
            snapshot_json(first),
        ),
        success(
            "snapshot_last_page",
            "sessionSnapshot",
            4,
            7,
            SuccessResponsePayload::SessionSnapshot(last),
            snapshot_json(last),
        ),
        success(
            "transport_get_stopped",
            "transportGet",
            5,
            7,
            SuccessResponsePayload::TransportGetSnapshot(stopped),
            transport_json(stopped),
        ),
        success(
            "transport_set_playing",
            "transportSet",
            6,
            7,
            SuccessResponsePayload::TransportSetSnapshot(playing),
            transport_json(playing),
        ),
        success(
            "telemetry_full",
            "telemetryConfigure",
            7,
            7,
            SuccessResponsePayload::TelemetryConfiguration(telemetry.clone()),
            telemetry_json(&telemetry),
        ),
        success(
            "telemetry_empty",
            "telemetryConfigure",
            8,
            7,
            SuccessResponsePayload::TelemetryConfiguration(no_telemetry.clone()),
            telemetry_json(&no_telemetry),
        ),
        success(
            "counters_some",
            "counters",
            9,
            7,
            SuccessResponsePayload::CounterSnapshot(counters.clone()),
            counters_json(&counters),
        ),
        success(
            "counters_none",
            "counters",
            10,
            7,
            SuccessResponsePayload::CounterSnapshot(no_counters.clone()),
            counters_json(&no_counters),
        ),
        success(
            "diagnostics_page",
            "diagnostics",
            11,
            7,
            SuccessResponsePayload::DiagnosticsPage(page.clone()),
            page_json(&page),
        ),
        success(
            "diagnostics_empty",
            "diagnostics",
            12,
            7,
            SuccessResponsePayload::DiagnosticsPage(empty_page.clone()),
            page_json(&empty_page),
        ),
        refusal(
            "diagnostics_invalid_field",
            "diagnostics",
            MessageId::DiagnosticsGet,
            (StatusCode::InvalidField, "invalid_field"),
            &invalid,
        ),
        refusal(
            "snapshot_revision_conflict",
            "sessionSnapshot",
            MessageId::SessionSnapshotGet,
            (StatusCode::RevisionConflict, "revision_conflict"),
            &invalid,
        ),
        refusal(
            "transport_set_refused",
            "transportSet",
            MessageId::TransportSet,
            (StatusCode::ValidationFailed, "validation_failed"),
            &refused,
        ),
    ]
}

fn event(name: &'static str, revision: u64, payload: EventPayload<'_>, value: J) -> Row {
    let codec = ProtocolCodec::default();
    let frame = TypedEventFrame {
        revision: SessionRevision(revision),
        payload,
    };
    let mut bytes = vec![0_u8; 4096];
    let length = codec
        .encode_event_frame_into(&frame, &mut bytes)
        .expect("the event encodes");
    bytes.truncate(length);
    Row {
        name,
        fields: vec![("expect", value)],
        bytes,
    }
}

fn with_kind(kind: &'static str, revision: u64, mut rest: Vec<(&'static str, J)>) -> J {
    let mut fields = vec![("kind", text(kind)), ("revision", big(revision))];
    fields.append(&mut rest);
    obj(fields)
}

fn transport_event(name: &'static str, value: TransportStateEvent) -> Row {
    let mut fields = vec![
        ("sequence", big(value.event_sequence)),
        ("state", text(state_name(value.state))),
        ("effectiveSample", big(value.effective_sample.0)),
    ];
    if let Some(origin) = value.origin_request_id {
        fields.push(("originRequestId", big(origin.get())));
    }
    event(
        name,
        14,
        EventPayload::TransportState(value),
        with_kind("transportState", 14, fields),
    )
}

fn events() -> Vec<Row> {
    let committed = SessionCommitted {
        event_sequence: 2,
        origin_request_id: request_id((1 << 50) + 1),
        previous_revision: SessionRevision(13),
        applied_operations: 3,
    };
    let ended = ObservationEnded {
        event_sequence: 5,
        keys: vec![
            ObservationTapKey {
                strip_id: "drums".to_owned(),
                section: ObservationSection::Console,
                effect_id: "desk-comp".to_owned(),
                tap_id: 1,
            },
            ObservationTapKey {
                strip_id: "drums".to_owned(),
                section: ObservationSection::Inserts,
                effect_id: "eq".to_owned(),
                tap_id: 7,
            },
        ],
    };
    let records = [
        MeterRecord {
            handle: 1,
            component: MeterComponent::Left,
            flags: 0b001,
            value: -12.5,
        },
        MeterRecord {
            handle: 1,
            component: MeterComponent::Right,
            flags: 0b011,
            value: 0.25,
        },
        MeterRecord {
            handle: 70_000,
            component: MeterComponent::Aggregate,
            flags: 0b100,
            value: 0.0,
        },
    ];
    let counters = some_counters();
    let full = full_diagnostic(17);
    let bare = bare_diagnostic(Some(18));
    let record_json = |record: &MeterRecord| {
        obj(vec![
            ("handle", J::Num(u64::from(record.handle))),
            (
                "component",
                text(match record.component {
                    MeterComponent::Left => "left",
                    MeterComponent::Right => "right",
                    MeterComponent::Aggregate => "aggregate",
                }),
            ),
            ("flags", J::Num(u64::from(record.flags))),
            ("value", J::Float(record.value)),
        ])
    };
    let key_json = |key: &ObservationTapKey| {
        obj(vec![
            ("stripId", text(&key.strip_id)),
            (
                "rack",
                text(match key.section {
                    ObservationSection::Console => "console",
                    ObservationSection::Inserts => "inserts",
                }),
            ),
            ("effectSlotId", text(&key.effect_id)),
            ("tapId", J::Num(u64::from(key.tap_id))),
        ])
    };
    vec![
        event(
            "session_committed",
            14,
            EventPayload::SessionCommitted(committed),
            with_kind(
                "sessionCommitted",
                14,
                vec![
                    ("sequence", big(2)),
                    ("originRequestId", big((1 << 50) + 1)),
                    ("previousRevision", big(13)),
                    ("appliedOperations", J::Num(3)),
                ],
            ),
        ),
        transport_event(
            "transport_state_from_request",
            TransportStateEvent {
                event_sequence: 3,
                state: TransportState::Playing,
                effective_sample: SampleTime(256),
                origin_request_id: Some(request_id(77)),
            },
        ),
        transport_event(
            "transport_state_unrequested",
            TransportStateEvent {
                event_sequence: 4,
                state: TransportState::Stopped,
                effective_sample: SampleTime(u64::MAX),
                origin_request_id: None,
            },
        ),
        event(
            "observation_ended",
            15,
            EventPayload::ObservationEnded(&ended),
            with_kind(
                "observationEnded",
                15,
                vec![
                    ("sequence", big(5)),
                    ("keys", J::Arr(ended.keys.iter().map(key_json).collect())),
                ],
            ),
        ),
        event(
            "meter_batch",
            15,
            EventPayload::MeterBatch(MeterBatch {
                observed_sample: SampleTime(4096),
                first_sample: SampleTime(2560),
                window_sequence: 7,
                records: &records,
            }),
            with_kind(
                "meterBatch",
                15,
                vec![
                    ("observedSample", big(4096)),
                    ("firstSample", big(2560)),
                    ("windowSequence", big(7)),
                    ("records", J::Arr(records.iter().map(record_json).collect())),
                ],
            ),
        ),
        event(
            "counter_snapshot",
            15,
            EventPayload::CounterSnapshot(CounterSnapshotRef {
                observed_sample: counters.observed_sample,
                values: &counters.values,
            }),
            with_kind(
                "counterSnapshot",
                15,
                match counters_json(&counters) {
                    J::Obj(fields) => fields,
                    _ => unreachable!("an object"),
                },
            ),
        ),
        event(
            "diagnostic_full",
            16,
            EventPayload::Diagnostic(&full),
            with_kind(
                "diagnostic",
                16,
                vec![("diagnostic", diagnostic_json(&full))],
            ),
        ),
        event(
            "diagnostic_bare",
            16,
            EventPayload::Diagnostic(&bare),
            with_kind(
                "diagnostic",
                16,
                vec![("diagnostic", diagnostic_json(&bare))],
            ),
        ),
    ]
}

fn section(text: &mut String, key: &str, rows: Vec<Row>, last: bool) {
    writeln!(text, "  \"{key}\": [").expect("write to a string");
    let count = rows.len();
    for (index, row) in rows.into_iter().enumerate() {
        text.push_str("    ");
        render_json(&row_json(row), text);
        text.push_str(if index + 1 == count { "\n" } else { ",\n" });
    }
    text.push_str(if last { "  ]\n" } else { "  ],\n" });
}

fn render() -> String {
    let mut text = String::new();
    text.push_str("{\n  \"schema\": \"miso.protocol.sdk-messages\",\n  \"counterIds\": ");
    let ids = J::Arr(
        counter_ids()
            .into_iter()
            .map(|id| {
                obj(vec![
                    ("name", J::Str(counter_name(id).to_owned())),
                    ("code", J::Num(u64::from(id as u32))),
                ])
            })
            .collect(),
    );
    render_json(&ids, &mut text);
    text.push_str(",\n");
    section(&mut text, "commands", commands(), false);
    section(&mut text, "replies", replies(), false);
    section(&mut text, "events", events(), true);
    text.push_str("}\n");
    text
}

/// Gate 1 of #1571: the checked-in fixture is exactly what the protocol codec writes.
///
/// Red if the codec changes a byte of any request, reply or event the SDK's message codec
/// encodes or decodes, or accepts another counter ID, without the fixture (and with it the SDK
/// codec the SDK eval holds to this file) following.
#[test]
fn the_checked_in_fixture_is_what_the_codec_writes() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE);
    let rendered = render();
    if std::env::var_os("MISO_ENGINE_BLESS_FIXTURES").is_some() {
        std::fs::create_dir_all(path.parent().expect("a parent directory"))
            .expect("fixture directory");
        std::fs::write(&path, &rendered).expect("write the fixture");
        return;
    }
    let checked_in = std::fs::read_to_string(&path).expect("the checked-in fixture exists");
    assert!(
        checked_in == rendered,
        "{FIXTURE} differs from the codec's frames; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
    );
}
