//! Issue #1589b (decision 17, C9 and C13): the byte fixture the SDK's `Observe` encoder and
//! `OBSERVATION_READ` codec are held to.
//!
//! The protocol codec writes a fixed list of `SESSION_TRANSACTION_APPLY` command frames of `Observe`
//! edits and `OBSERVATION_READ` command frames, and the typed response encoders write the read
//! replies the SDK must decode. The frames are checked in as `fixtures/observation-frames.json`; the
//! SDK eval (`sdk/test/control-frame-evals.mjs`) encodes the same requests in TypeScript and compares
//! bytes, and decodes the same replies. This is a wire format, so a byte comparison is the right
//! gate, on the model of `set_control_frames.rs` (#1563). The transaction reply with arm tags is
//! `set_control_frames.rs`'s `applied_with_arm_tags`.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate codec change.

use std::borrow::Cow;
use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    CommandPayload, Diagnostic, DiagnosticSeverity, ExpectedRevision, MessageId, NonOkResponse,
    ObservationReadPage, ObservationReadRequest, ObservationReadRow, ObservationReadSelection,
    ObservationRowStatus, ObservationRowWindow, ParameterChannel, ParameterUnit, ProtocolCodec,
    RequestId, SessionEdit, SessionRevision, SessionTransactionFrame, StatusCode,
    SuccessResponsePayload, TypedCommandFrame, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
    control_core::TapSection,
};
use session::StableId;

const FIXTURE: &str = "tests/fixtures/observation-frames.json";

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut text, byte| {
        write!(text, "{byte:02x}").expect("write to a string");
        text
    })
}

fn channel_name(channels: ParameterChannel) -> &'static str {
    match channels {
        ParameterChannel::Left => "left",
        ParameterChannel::Right => "right",
        ParameterChannel::Both => "both",
    }
}

/// The section as the SDK's `ObserveEdit`/`ObservationSelection` spells it: the rack and the ID.
fn section_json(section: &TapSection) -> String {
    match section {
        TapSection::Console(slot) => {
            format!("\"rack\": \"console\", \"effectId\": \"{}\"", slot.as_str())
        }
        TapSection::Inserts(effect) => {
            format!(
                "\"rack\": \"inserts\", \"effectId\": \"{}\"",
                effect.as_str()
            )
        }
    }
}

fn observe(
    strip: &str,
    section: TapSection,
    tap_id: u32,
    armed: bool,
    window_blocks: u32,
) -> SessionEdit {
    SessionEdit::Observe {
        strip_id: id(strip),
        section,
        tap_id,
        armed,
        window_blocks,
    }
}

fn observe_json(edit: &SessionEdit) -> String {
    let SessionEdit::Observe {
        strip_id,
        section,
        tap_id,
        armed,
        window_blocks,
    } = edit
    else {
        panic!("the fixture's transactions hold Observe edits only");
    };
    format!(
        "{{ \"stripId\": \"{}\", {}, \"tapId\": {tap_id}, \"armed\": {armed}, \"windowBlocks\": {window_blocks} }}",
        strip_id.as_str(),
        section_json(section),
    )
}

struct Command {
    name: &'static str,
    request_id: u64,
    /// The request as the SDK's encoder takes it, as JSON.
    request: String,
    bytes: Vec<u8>,
}

/// A `SESSION_TRANSACTION_APPLY` of `Observe` edits, checked to decode back to its edits.
fn transaction(
    name: &'static str,
    request_id: u64,
    expected_revision: u64,
    edits: &[SessionEdit],
) -> Command {
    let codec = ProtocolCodec::default();
    let frame = SessionTransactionFrame {
        request_id: RequestId::new(request_id).expect("nonzero request ID"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(expected_revision)),
        edits,
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
    let decoded = codec
        .decode_session_transaction(
            &bytes,
            &mut protocol::DecodeScratch::new(&mut [0_u16; 4096]),
        )
        .expect("the frame decodes");
    assert_eq!(decoded.edits, edits, "an Observe edit decodes as itself");
    let request = format!(
        "{{ \"op\": \"apply\", \"expectedRevision\": \"{expected_revision}\", \"edits\": [{}] }}",
        edits
            .iter()
            .map(observe_json)
            .collect::<Vec<_>>()
            .join(", "),
    );
    Command {
        name,
        request_id,
        request,
        bytes,
    }
}

fn selection(
    strip: &str,
    section: TapSection,
    tap_id: u32,
    channels: ParameterChannel,
) -> ObservationReadSelection {
    ObservationReadSelection {
        strip_id: id(strip),
        section,
        tap_id,
        channels,
    }
}

/// An `OBSERVATION_READ`, which takes any revision.
fn read(name: &'static str, request_id: u64, selections: Vec<ObservationReadSelection>) -> Command {
    let value = ObservationReadRequest { selections };
    let frame = TypedCommandFrame {
        request_id: RequestId::new(request_id).expect("nonzero request ID"),
        expected_revision: ExpectedRevision::Any,
        payload: CommandPayload::ObservationRead(&value),
    };
    let mut bytes = vec![0_u8; 8_192];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("the command encodes");
    bytes.truncate(length);
    let request = format!(
        "{{ \"op\": \"read\", \"selections\": [{}] }}",
        value
            .selections
            .iter()
            .map(|row| format!(
                "{{ \"stripId\": \"{}\", {}, \"tapId\": {}, \"channels\": \"{}\" }}",
                row.strip_id.as_str(),
                section_json(&row.section),
                row.tap_id,
                channel_name(row.channels),
            ))
            .collect::<Vec<_>>()
            .join(", "),
    );
    Command {
        name,
        request_id,
        request,
        bytes,
    }
}

/// IDs of lengths 1 to 9 cover every padding residue of the UTF-8 fields.
const PADDED: [&str; 9] = [
    "a",
    "bb",
    "ccc",
    "dddd",
    "eeeee",
    "ffffff",
    "ggggggg",
    "hhhhhhhh",
    "iiiiiiiii",
];

fn padded_section(index: usize, name: &str) -> TapSection {
    if index % 2 == 0 {
        TapSection::Inserts(id(name))
    } else {
        TapSection::Console(id(name))
    }
}

fn commands() -> Vec<Command> {
    let lanes = [
        ParameterChannel::Left,
        ParameterChannel::Right,
        ParameterChannel::Both,
    ];
    let padded_observes: Vec<SessionEdit> = PADDED
        .iter()
        .enumerate()
        .map(|(index, name)| {
            observe(
                name,
                padded_section(index, PADDED[PADDED.len() - 1 - index]),
                u32::try_from(index + 1).expect("small"),
                index % 3 != 0,
                [0, 1, 4_096][index % 3],
            )
        })
        .collect();
    let padded_reads: Vec<ObservationReadSelection> = PADDED
        .iter()
        .enumerate()
        .map(|(index, name)| {
            selection(
                name,
                padded_section(index + 1, PADDED[PADDED.len() - 1 - index]),
                if index == 0 {
                    u32::MAX
                } else {
                    u32::try_from(index).expect("small")
                },
                lanes[index % 3],
            )
        })
        .collect();
    vec![
        transaction(
            "observe_arm_insert",
            1,
            0,
            &[observe(
                "vocal",
                TapSection::Inserts(id("comp")),
                1,
                true,
                4,
            )],
        ),
        // A window change of a tap one subscriber holds: the disarm of the old window and the arm
        // of the new one, in one transaction (#1584 gate 1).
        transaction(
            "observe_window_change",
            2,
            7,
            &[
                observe("drums", TapSection::Console(id("desk-comp")), 1, false, 2),
                observe("drums", TapSection::Console(id("desk-comp")), 1, true, 8),
            ],
        ),
        transaction(
            "observe_padded",
            (1 << 40) + 3,
            (1 << 33) + 5,
            &padded_observes,
        ),
        read(
            "read_one",
            4,
            vec![selection(
                "vocal",
                TapSection::Inserts(id("comp")),
                1,
                ParameterChannel::Both,
            )],
        ),
        read("read_padded", 5, padded_reads),
    ]
}

struct Reply {
    name: &'static str,
    bytes: Vec<u8>,
    /// The fields the SDK decoder must report, as JSON.
    expect: String,
}

fn opt_json<T: std::fmt::Display>(value: Option<T>, quoted: bool) -> String {
    match value {
        None => "null".to_owned(),
        Some(value) if quoted => format!("\"{value}\""),
        Some(value) => value.to_string(),
    }
}

fn row_json(row: &ObservationReadRow) -> String {
    let status = match row.status {
        ObservationRowStatus::Pending => "pending",
        ObservationRowStatus::Unarmed => "unarmed",
        ObservationRowStatus::Ready => "ready",
    };
    let window = row.window.map_or_else(
        || "null".to_owned(),
        |window| {
            format!(
                "{{ \"firstSample\": \"{}\", \"endSample\": \"{}\", \"sequence\": \"{}\", \"blocks\": {} }}",
                window.first_sample, window.end_sample, window.sequence, window.blocks
            )
        },
    );
    format!(
        "{{ \"status\": \"{status}\", \"armed\": {}, \"armSample\": \"{}\", \"armTag\": \"{}\", \
         \"window\": {window}, \"leftBits\": {}, \"rightBits\": {}, \"unit\": {} }}",
        row.armed,
        row.arm_sample,
        row.arm_tag,
        opt_json(
            row.left.map(|value| format!("{:08x}", value.to_bits())),
            true
        ),
        opt_json(
            row.right.map(|value| format!("{:08x}", value.to_bits())),
            true
        ),
        row.unit as u8,
    )
}

fn page(
    name: &'static str,
    request_id: u64,
    observed_sample: u64,
    rows: Vec<ObservationReadRow>,
) -> Reply {
    let expect = format!(
        "{{ \"requestId\": \"{request_id}\", \"revision\": \"5\", \"ok\": true, \"observedSample\": \"{observed_sample}\", \
         \"rows\": [{}] }}",
        rows.iter().map(row_json).collect::<Vec<_>>().join(", "),
    );
    let frame = TypedSuccessResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(5),
        payload: SuccessResponsePayload::ObservationRead(ObservationReadPage {
            observed_sample,
            rows: Cow::Owned(rows),
        }),
    };
    let mut bytes = vec![0_u8; 8_192];
    let length = ProtocolCodec::default()
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("the reply encodes");
    bytes.truncate(length);
    Reply {
        name,
        bytes,
        expect,
    }
}

fn refusal(
    name: &'static str,
    request_id: u64,
    (status, status_name): (StatusCode, &'static str),
    code: &'static str,
) -> Reply {
    let payload = NonOkResponse {
        diagnostics: vec![Diagnostic {
            code: code.to_owned(),
            severity: DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: None,
        }],
        omitted_diagnostics: 0,
        backpressure: None,
    };
    let frame = TypedNonOkResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(5),
        message_id: MessageId::ObservationRead,
        status,
        payload: &payload,
    };
    let mut bytes = vec![0_u8; 1_024];
    let length = ProtocolCodec::default()
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("the refusal encodes");
    bytes.truncate(length);
    Reply {
        name,
        bytes,
        expect: format!(
            "{{ \"requestId\": \"{request_id}\", \"revision\": \"5\", \"ok\": false, \
             \"status\": \"{status_name}\", \"code\": \"{code}\" }}",
        ),
    }
}

const fn row(
    status: ObservationRowStatus,
    armed: bool,
    arm_sample: u64,
    arm_tag: u64,
) -> ObservationReadRow {
    ObservationReadRow {
        status,
        armed,
        arm_sample,
        arm_tag,
        window: None,
        left: None,
        right: None,
        unit: ParameterUnit::Db,
    }
}

fn ready(
    arm_sample: u64,
    arm_tag: u64,
    left: Option<f32>,
    right: Option<f32>,
    unit: ParameterUnit,
) -> ObservationReadRow {
    ObservationReadRow {
        window: Some(ObservationRowWindow {
            first_sample: arm_sample + 256,
            end_sample: arm_sample + 768,
            sequence: arm_tag + 40,
            blocks: 4,
        }),
        left,
        right,
        unit,
        ..row(ObservationRowStatus::Ready, true, arm_sample, arm_tag)
    }
}

fn replies() -> Vec<Reply> {
    vec![
        // An unarmed tap, an arm render has not applied, and an arm render applied with no window
        // closed yet (#1585's status contract).
        page(
            "page_not_ready",
            21,
            4_096,
            vec![
                row(ObservationRowStatus::Unarmed, false, 0, 0),
                row(ObservationRowStatus::Pending, false, 0, 0),
                row(ObservationRowStatus::Pending, true, 4_096, 3),
            ],
        ),
        page(
            "page_ready",
            22,
            (1 << 40) + 768,
            vec![
                ready(
                    1 << 40,
                    (1 << 53) + 1,
                    Some(-3.5),
                    Some(0.0),
                    ParameterUnit::Db,
                ),
                ready(128, 2, Some(0.25), None, ParameterUnit::Linear),
                ready(0, 1, None, Some(-0.0), ParameterUnit::Ratio),
            ],
        ),
        refusal(
            "read_address_unknown",
            23,
            (StatusCode::ValidationFailed, "validation_failed"),
            "observation.address.unknown",
        ),
        refusal(
            "read_unavailable",
            24,
            (StatusCode::Unavailable, "unavailable"),
            "observation.unavailable",
        ),
        refusal(
            "read_frame",
            25,
            (StatusCode::LimitExceeded, "limit_exceeded"),
            "observation.frame",
        ),
    ]
}

fn render() -> String {
    let mut text = String::new();
    text.push_str("{\n  \"schema\": \"miso.protocol.observation-frames\",\n  \"commands\": [\n");
    let commands = commands();
    for (position, command) in commands.iter().enumerate() {
        write!(
            text,
            "    {{\n      \"name\": \"{}\",\n      \"requestId\": \"{}\",\n      \"request\": {},\n      \"hex\": \"{}\"\n    }}{}\n",
            command.name,
            command.request_id,
            command.request,
            hex(&command.bytes),
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

/// #1589b: the checked-in fixture is exactly what the protocol codec writes.
///
/// Red if the codec changes a byte of an `Observe` transaction, an `OBSERVATION_READ` request or
/// its reply without the fixture (and with it the SDK codec the SDK eval holds to this file)
/// following.
#[test]
fn the_checked_in_fixture_is_what_the_codec_writes() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE);
    let rendered = render();
    if std::env::var_os("MISO_ENGINE_BLESS_FIXTURES").is_some() {
        std::fs::write(&path, &rendered).expect("write the fixture");
        return;
    }
    let checked_in = std::fs::read_to_string(&path).expect("the checked-in fixture exists");
    assert!(
        checked_in == rendered,
        "{FIXTURE} differs from the codec's frames; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
    );
}
