//! Issue #1563 (decision 17, C5): the byte fixture the SDK's `SET_CONTROL` encoder and reply decoder
//! are held to.
//!
//! The protocol codec writes a fixed list of `SESSION_TRANSACTION_APPLY` command frames of
//! `SET_CONTROL` edits, and the typed response encoders write the replies the SDK must decode. The
//! frames are checked in as `fixtures/set-control-frames.json`; the SDK eval
//! (`sdk/test/control-frame-evals.mjs`) encodes the same edits in TypeScript and compares bytes, and
//! decodes the same replies. This is a wire format, so a byte comparison is the right gate.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate codec change.

use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    Backpressure, BackpressureQueueKind, DecodeScratch, Diagnostic, DiagnosticSeverity, EditPath,
    ExpectedRevision, MessageId, NonOkResponse, PathSegment, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, SessionTransactionFrame, StatusCode, SuccessResponsePayload,
    TransactionApplied, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
    control_core::{ControlAddress, Owner, Section},
};
use session::{ParameterChannel, StableId};

const FIXTURE: &str = "tests/fixtures/set-control-frames.json";

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

fn lane_name(lane: ParameterChannel) -> &'static str {
    match lane {
        ParameterChannel::Left => "left",
        ParameterChannel::Right => "right",
        ParameterChannel::Both => "both",
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut text, byte| {
        write!(text, "{byte:02x}").expect("write to a string");
        text
    })
}

/// One edit as the fixture's JSON object: the SDK builds its edit from these fields alone.
fn edit_json(edit: &SessionEdit) -> String {
    let SessionEdit::SetControl {
        address,
        value,
        ramp_samples,
    } = edit
    else {
        panic!("the fixture holds SET_CONTROL edits only");
    };
    let owner = match &address.owner {
        Owner::Session => "{ \"kind\": \"session\" }".to_owned(),
        Owner::Strip(id) => format!("{{ \"kind\": \"strip\", \"id\": \"{}\" }}", id.as_str()),
        Owner::Route(id) => format!("{{ \"kind\": \"route\", \"id\": \"{}\" }}", id.as_str()),
        Owner::Vca(id) => format!("{{ \"kind\": \"vca\", \"id\": \"{}\" }}", id.as_str()),
    };
    let section = match &address.section {
        Section::Own => "{ \"kind\": \"own\" }".to_owned(),
        Section::Builtins => "{ \"kind\": \"builtins\" }".to_owned(),
        Section::Console(slot) => format!(
            "{{ \"kind\": \"console\", \"slot\": \"{}\" }}",
            slot.as_str()
        ),
        Section::Inserts(effect) => {
            format!(
                "{{ \"kind\": \"inserts\", \"effect\": \"{}\" }}",
                effect.as_str()
            )
        }
    };
    let ramp = ramp_samples.map_or_else(|| "null".to_owned(), |ramp| ramp.to_string());
    format!(
        "{{ \"owner\": {owner}, \"section\": {section}, \"control\": {}, \"lane\": \"{}\", \
         \"valueBits\": \"{:08x}\", \"rampSamples\": {ramp} }}",
        address.control,
        lane_name(address.lane),
        value.to_bits(),
    )
}

fn set_control(
    owner: Owner,
    section: Section,
    control: u32,
    lane: ParameterChannel,
    value: f32,
    ramp_samples: Option<u32>,
) -> SessionEdit {
    SessionEdit::SetControl {
        address: ControlAddress {
            owner,
            section,
            control,
            lane,
        },
        value,
        ramp_samples,
    }
}

/// Every owner kind and section shape a `SET_CONTROL` can name, with a row ID for each.
fn shapes() -> Vec<(Owner, Section, u32)> {
    vec![
        (Owner::Session, Section::Own, 1),
        (Owner::Strip(id("vocal")), Section::Builtins, 3),
        (Owner::Strip(id("vocal")), Section::Own, 2),
        (Owner::Strip(id("vocal")), Section::Inserts(id("eq")), 1),
        (
            Owner::Strip(id("vocal")),
            Section::Console(id("desk-eq")),
            0,
        ),
        (Owner::Route(id("to-main")), Section::Own, 1),
        (Owner::Vca(id("grp")), Section::Own, 1),
    ]
}

struct Command {
    name: &'static str,
    request_id: u64,
    expected_revision: u64,
    edits: Vec<SessionEdit>,
}

fn commands() -> Vec<Command> {
    let lanes = [
        ParameterChannel::Left,
        ParameterChannel::Right,
        ParameterChannel::Both,
    ];
    let ramps = [None, Some(0), Some(480)];
    // Every shape in every lane with every ramp, in one transaction.
    let mut cross_product = Vec::new();
    for ramp in ramps {
        for lane in lanes {
            for (index, (owner, section, control)) in shapes().into_iter().enumerate() {
                cross_product.push(set_control(
                    owner,
                    section,
                    control,
                    lane,
                    -1.5 + index as f32,
                    ramp,
                ));
            }
        }
    }
    let one = |owner, section, control, lane, value, ramp| {
        vec![set_control(owner, section, control, lane, value, ramp)]
    };
    // Stable IDs of lengths 1 to 9 cover every padding residue of the UTF-8 fields.
    let padded: Vec<SessionEdit> = [
        "a",
        "bb",
        "ccc",
        "dddd",
        "eeeee",
        "ffffff",
        "ggggggg",
        "hhhhhhhh",
        "iiiiiiiii",
    ]
    .into_iter()
    .map(|name| {
        set_control(
            Owner::Strip(id(name)),
            Section::Console(id(name)),
            7,
            ParameterChannel::Both,
            0.25,
            None,
        )
    })
    .collect();
    let odd_values: Vec<SessionEdit> = [
        0.0_f32,
        -0.0,
        0.1,
        -80.0,
        f32::MAX,
        f32::MIN_POSITIVE,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ]
    .into_iter()
    .map(|value| {
        set_control(
            Owner::Strip(id("vocal")),
            Section::Builtins,
            3,
            ParameterChannel::Left,
            value,
            None,
        )
    })
    .collect();
    vec![
        Command {
            name: "one_edit_no_ramp",
            request_id: 1,
            expected_revision: 0,
            edits: one(
                Owner::Strip(id("vocal")),
                Section::Builtins,
                3,
                ParameterChannel::Left,
                -6.0,
                None,
            ),
        },
        Command {
            name: "one_edit_ramp_zero",
            request_id: 2,
            expected_revision: 1,
            edits: one(
                Owner::Route(id("to-main")),
                Section::Own,
                1,
                ParameterChannel::Both,
                -3.0,
                Some(0),
            ),
        },
        Command {
            name: "one_edit_ramp_max",
            request_id: 3,
            expected_revision: 2,
            edits: one(
                Owner::Vca(id("grp")),
                Section::Own,
                1,
                ParameterChannel::Both,
                1.0,
                Some(u32::MAX),
            ),
        },
        Command {
            name: "wide_identifiers",
            request_id: (1 << 40) + 7,
            expected_revision: (1 << 40) + 9,
            edits: padded,
        },
        Command {
            name: "unusual_values",
            request_id: u64::MAX,
            expected_revision: u64::MAX - 1,
            edits: odd_values,
        },
        Command {
            name: "every_shape_lane_and_ramp",
            request_id: 100,
            expected_revision: 12,
            edits: cross_product,
        },
    ]
}

fn command_frame(command: &Command) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = SessionTransactionFrame {
        request_id: RequestId::new(command.request_id).expect("nonzero request ID"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(command.expected_revision)),
        edits: &command.edits,
    };
    let length = codec
        .encoded_session_transaction_len(&frame)
        .expect("the frame is sized");
    let mut bytes = vec![0_u8; length];
    assert_eq!(
        codec
            .encode_session_transaction(&frame, &mut bytes)
            .expect("the frame encodes"),
        length
    );
    // The frame is a valid transaction that decodes back to the edits it was built from.
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 4096]))
        .expect("the frame decodes");
    for (got, want) in decoded.edits.iter().zip(&command.edits) {
        match (got, want) {
            (
                SessionEdit::SetControl {
                    address: got_address,
                    value: got_value,
                    ramp_samples: got_ramp,
                },
                SessionEdit::SetControl {
                    address: want_address,
                    value: want_value,
                    ramp_samples: want_ramp,
                },
            ) => {
                assert_eq!(got_address, want_address);
                assert_eq!(got_value.to_bits(), want_value.to_bits());
                assert_eq!(got_ramp, want_ramp);
            }
            _ => panic!("a SET_CONTROL edit decodes as itself"),
        }
    }
    assert_eq!(decoded.edits.len(), command.edits.len());
    bytes
}

struct Reply {
    name: &'static str,
    bytes: Vec<u8>,
    /// The fields the SDK decoder must report, as JSON.
    expect: String,
}

fn diagnostic(code: &str, operation_index: Option<u32>) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        severity: DiagnosticSeverity::Error,
        path: vec![
            PathSegment::Field("edits".to_owned()),
            PathSegment::Index(2),
        ],
        detail: Some("a detail".to_owned()),
        operation_index,
        sample_time: None,
        provider_sequence: None,
    }
}

fn success_reply(
    name: &'static str,
    request_id: u64,
    revision: u64,
    path: EditPath,
    applied: u32,
) -> Reply {
    let codec = ProtocolCodec::default();
    let frame = TypedSuccessResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(revision),
        payload: SuccessResponsePayload::SessionTransactionApplied(TransactionApplied {
            applied_operations: applied,
            edit_path: path,
        }),
    };
    let mut bytes = vec![0_u8; 256];
    let length = codec
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("the reply encodes");
    bytes.truncate(length);
    let path_name = match path {
        EditPath::Live => "live",
        EditPath::ModelOnly => "model_only",
        EditPath::Rebuild => "rebuild",
        EditPath::Monitor => "monitor",
    };
    Reply {
        name,
        bytes,
        expect: format!(
            "{{ \"requestId\": \"{request_id}\", \"status\": \"ok\", \"revision\": \"{revision}\", \
             \"appliedOperations\": {applied}, \"editPath\": \"{path_name}\", \"editPathCode\": {}, \
             \"diagnostics\": [], \"omittedDiagnostics\": 0, \"backpressure\": false }}",
            path.code()
        ),
    }
}

fn refusal_reply(
    name: &'static str,
    request_id: u64,
    revision: u64,
    status: (StatusCode, &'static str),
    payload: &NonOkResponse,
    expect_diagnostics: &str,
) -> Reply {
    let codec = ProtocolCodec::default();
    let frame = TypedNonOkResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(revision),
        message_id: MessageId::SessionTransactionApply,
        status: status.0,
        payload,
    };
    let mut bytes = vec![0_u8; 1024];
    let length = codec
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("the refusal encodes");
    bytes.truncate(length);
    Reply {
        name,
        bytes,
        expect: format!(
            "{{ \"requestId\": \"{request_id}\", \"status\": \"{}\", \"revision\": \"{revision}\", \
             \"diagnostics\": {expect_diagnostics}, \"omittedDiagnostics\": {}, \"backpressure\": {} }}",
            status.1,
            payload.omitted_diagnostics,
            payload.backpressure.is_some(),
        ),
    }
}

fn replies() -> Vec<Reply> {
    let validation = (StatusCode::ValidationFailed, "validation_failed");
    vec![
        success_reply("applied_live", 1, 5, EditPath::Live, 3),
        success_reply("applied_model_only", 2, 6, EditPath::ModelOnly, 1),
        success_reply("applied_rebuild", 3, (1 << 40) + 1, EditPath::Rebuild, 12),
        refusal_reply(
            "control_domain",
            4,
            5,
            validation,
            &NonOkResponse {
                diagnostics: vec![diagnostic("control.domain", Some(2))],
                omitted_diagnostics: 0,
                backpressure: None,
            },
            "[ { \"code\": \"control.domain\", \"severity\": \"error\", \"operationIndex\": 2 } ]",
        ),
        refusal_reply(
            "address_unknown_with_omitted",
            5,
            5,
            validation,
            &NonOkResponse {
                diagnostics: vec![
                    diagnostic("control.address.unknown", Some(0)),
                    diagnostic("control.domain", None),
                ],
                omitted_diagnostics: 4,
                backpressure: None,
            },
            "[ { \"code\": \"control.address.unknown\", \"severity\": \"error\", \"operationIndex\": 0 }, \
             { \"code\": \"control.domain\", \"severity\": \"error\", \"operationIndex\": null } ]",
        ),
        refusal_reply(
            "revision_conflict",
            6,
            9,
            (StatusCode::RevisionConflict, "revision_conflict"),
            &NonOkResponse {
                diagnostics: Vec::new(),
                omitted_diagnostics: 0,
                backpressure: None,
            },
            "[]",
        ),
        refusal_reply(
            "backpressure",
            7,
            9,
            (StatusCode::Backpressure, "backpressure"),
            &NonOkResponse {
                diagnostics: Vec::new(),
                omitted_diagnostics: 0,
                backpressure: Some(Backpressure {
                    queue_kind: BackpressureQueueKind::ControlCommand,
                    capacity: 8,
                    occupancy: 8,
                    requested_items: 1,
                    generation: Some(3),
                    retry_boundary: None,
                    requested_bytes: None,
                    available_bytes: None,
                }),
            },
            "[]",
        ),
    ]
}

fn render() -> String {
    let mut text = String::new();
    text.push_str("{\n  \"schema\": \"miso.protocol.set-control-frames\",\n  \"commands\": [\n");
    let commands = commands();
    for (position, command) in commands.iter().enumerate() {
        let edits: Vec<String> = command.edits.iter().map(edit_json).collect();
        write!(
            text,
            "    {{\n      \"name\": \"{}\",\n      \"requestId\": \"{}\",\n      \"expectedRevision\": \"{}\",\n      \"edits\": [\n        {}\n      ],\n      \"hex\": \"{}\"\n    }}{}\n",
            command.name,
            command.request_id,
            command.expected_revision,
            edits.join(",\n        "),
            hex(&command_frame(command)),
            if position + 1 == commands.len() { "" } else { "," },
        )
        .expect("write to a string");
    }
    text.push_str("  ],\n  \"replies\": [\n");
    let replies = replies();
    for (position, reply) in replies.iter().enumerate() {
        write!(
            text,
            "    {{\n      \"name\": \"{}\",\n      \"hex\": \"{}\",\n      \"expect\": {}\n    }}{}\n",
            reply.name,
            hex(&reply.bytes),
            reply.expect,
            if position + 1 == replies.len() { "" } else { "," },
        )
        .expect("write to a string");
    }
    text.push_str("  ]\n}\n");
    text
}

/// Gate 1 of #1563A: the checked-in fixture is exactly what the protocol codec writes.
///
/// Red if the codec's `SET_CONTROL` or transaction framing changes a byte without the fixture (and
/// with it the SDK encoder the SDK eval holds to this file) following.
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
