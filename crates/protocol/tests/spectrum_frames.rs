//! Issue #1596 gate 1 (#1596b): the byte fixture the SDK's spectrum codec is held to.
//!
//! The protocol codec writes a fixed list of `SPECTRUM_CONFIGURE`, `SPECTRUM_CAPTURE` and
//! `SPECTRUM_STREAM` command frames (each operation of both read messages, a single and a
//! collection configuration with a hop) and the typed response encoders write the replies the SDK
//! must decode. The frames are checked in as `fixtures/spectrum-frames.json`; the SDK eval
//! (`sdk/test/control-frame-evals.mjs`) encodes the same requests in TypeScript and compares bytes,
//! and decodes the same replies. This is a wire format, so a byte comparison is the right gate, on
//! the model of `set_control_frames.rs` (#1563).
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate codec change.

use std::borrow::Cow;
use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    Backpressure, BackpressureQueueKind, CommandPayload, Diagnostic, DiagnosticSeverity,
    ExpectedRevision, MessageId, NonOkResponse, ParameterChannel, ProtocolCodec, RequestId,
    SessionRevision, SpectrumCaptureReply, SpectrumCaptureRequest, SpectrumCaptureStatus,
    SpectrumCaptureWindow, SpectrumConfigureRequest, SpectrumConfigured, SpectrumEntry,
    SpectrumForm, SpectrumStreamBins, SpectrumStreamReply, SpectrumStreamRequest,
    SpectrumStreamStatus, SpectrumStreamTarget, SpectrumTargetKind, StatusCode,
    SuccessResponsePayload, TypedCommandFrame, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};

const FIXTURE: &str = "tests/fixtures/spectrum-frames.json";

/// The bins of one window (`SPECTRUM_CAPTURE_BIN_COUNT`).
const BINS: usize = 1_025;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut text, byte| {
        write!(text, "{byte:02x}").expect("write to a string");
        text
    })
}

fn kind_name(kind: SpectrumTargetKind) -> &'static str {
    match kind {
        SpectrumTargetKind::TrackPostInput => "trackPostInput",
        SpectrumTargetKind::TrackPostPan => "trackPostPan",
        SpectrumTargetKind::Output => "output",
    }
}

fn channel_name(channels: ParameterChannel) -> &'static str {
    match channels {
        ParameterChannel::Left => "left",
        ParameterChannel::Right => "right",
        ParameterChannel::Both => "both",
    }
}

/// A target as the SDK's `SpectrumTarget` spells it.
fn target_json(kind: SpectrumTargetKind, id: &str) -> String {
    match kind {
        SpectrumTargetKind::Output => {
            format!("{{ \"kind\": \"output\", \"outputId\": \"{id}\" }}")
        }
        _ => format!(
            "{{ \"kind\": \"{}\", \"trackId\": \"{id}\" }}",
            kind_name(kind)
        ),
    }
}

/// One plane of a ready reply: `offset + step * (index % modulo)`, every word exact in binary32,
/// so the SDK eval rebuilds the same words from the three numbers.
#[derive(Clone, Copy)]
struct Plane {
    offset: f32,
    step: f32,
    modulo: usize,
}

impl Plane {
    #[allow(
        clippy::cast_precision_loss,
        reason = "index % modulo is below 2^24, so the conversion is exact"
    )]
    fn words(self) -> Vec<f32> {
        (0..BINS)
            .map(|index| self.offset + self.step * (index % self.modulo) as f32)
            .collect()
    }

    fn json(self) -> String {
        format!(
            "{{ \"offset\": {}, \"step\": {}, \"modulo\": {} }}",
            self.offset, self.step, self.modulo
        )
    }
}

const FREQUENCIES: Plane = Plane {
    offset: 0.0,
    step: 23.4375,
    modulo: BINS,
};
const LEFT: Plane = Plane {
    offset: -120.0,
    step: 0.25,
    modulo: 97,
};
const RIGHT: Plane = Plane {
    offset: -96.5,
    step: 0.125,
    modulo: 61,
};

struct Command {
    name: &'static str,
    request_id: u64,
    /// The request as the SDK's encoder takes it, as JSON.
    request: String,
    bytes: Vec<u8>,
}

fn command(
    name: &'static str,
    request_id: u64,
    request: String,
    payload: CommandPayload<'_>,
) -> Command {
    let frame = TypedCommandFrame {
        request_id: RequestId::new(request_id).expect("nonzero request ID"),
        expected_revision: ExpectedRevision::Any,
        payload,
    };
    let mut bytes = vec![0_u8; 1024];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("the command encodes");
    bytes.truncate(length);
    Command {
        name,
        request_id,
        request,
        bytes,
    }
}

fn entry(kind: SpectrumTargetKind, id: &str, channels: ParameterChannel) -> SpectrumEntry {
    SpectrumEntry {
        target_kind: kind,
        target_id: id.to_owned(),
        channels,
    }
}

fn configure(name: &'static str, request_id: u64, value: &SpectrumConfigureRequest) -> Command {
    let entries: Vec<String> = value
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{{ \"target\": {}, \"channels\": \"{}\" }}",
                target_json(entry.target_kind, &entry.target_id),
                channel_name(entry.channels)
            )
        })
        .collect();
    let request = format!(
        "{{ \"op\": \"configure\", \"form\": \"{}\", \"entries\": [{}], \"maximumCaptureBytes\": \"{}\", \"hopFrames\": {} }}",
        match value.form {
            SpectrumForm::Single => "single",
            SpectrumForm::Collection => "collection",
        },
        entries.join(", "),
        value.maximum_capture_bytes,
        value
            .hop_frames
            .map_or_else(|| "null".to_owned(), |hop| hop.to_string()),
    );
    command(
        name,
        request_id,
        request,
        CommandPayload::SpectrumConfigure(value),
    )
}

fn capture(name: &'static str, request_id: u64, value: &SpectrumCaptureRequest) -> Command {
    let request = match value {
        SpectrumCaptureRequest::Arm => "{ \"op\": \"capture\", \"capture\": \"arm\" }".to_owned(),
        SpectrumCaptureRequest::Cancel => {
            "{ \"op\": \"capture\", \"capture\": \"cancel\" }".to_owned()
        }
        SpectrumCaptureRequest::Read { channels, .. } => format!(
            "{{ \"op\": \"capture\", \"capture\": \"read\", \"channels\": \"{}\" }}",
            channel_name(*channels)
        ),
        SpectrumCaptureRequest::Select {
            target_kind,
            target_id,
            channels,
        } => format!(
            "{{ \"op\": \"capture\", \"capture\": \"select\", \"target\": {}, \"channels\": \"{}\" }}",
            target_json(*target_kind, target_id),
            channel_name(*channels)
        ),
    };
    command(
        name,
        request_id,
        request,
        CommandPayload::SpectrumCapture(value),
    )
}

fn stream(name: &'static str, request_id: u64, value: &SpectrumStreamRequest) -> Command {
    let simple = |op: &str| format!("{{ \"op\": \"stream\", \"stream\": \"{op}\" }}");
    let request = match value {
        SpectrumStreamRequest::Start { smoothing_ms } => format!(
            "{{ \"op\": \"stream\", \"stream\": \"start\", \"smoothingMs\": {smoothing_ms} }}"
        ),
        SpectrumStreamRequest::Select {
            target_kind,
            target_id,
            channels,
            smoothing_ms,
        } => format!(
            "{{ \"op\": \"stream\", \"stream\": \"select\", \"target\": {}, \"channels\": \"{}\", \"smoothingMs\": {} }}",
            target_json(*target_kind, target_id),
            channel_name(*channels),
            smoothing_ms.map_or_else(|| "null".to_owned(), |value| value.to_string()),
        ),
        SpectrumStreamRequest::Read => simple("read"),
        SpectrumStreamRequest::Reset => simple("reset"),
        SpectrumStreamRequest::Stop => simple("stop"),
    };
    command(
        name,
        request_id,
        request,
        CommandPayload::SpectrumStream(value),
    )
}

fn commands() -> Vec<Command> {
    use SpectrumTargetKind::{Output, TrackPostInput, TrackPostPan};
    vec![
        configure(
            "configure_single",
            1,
            &SpectrumConfigureRequest {
                form: SpectrumForm::Single,
                entries: vec![entry(TrackPostPan, "vocal", ParameterChannel::Both)],
                maximum_capture_bytes: 1 << 20,
                hop_frames: None,
            },
        ),
        configure(
            "configure_collection_hop",
            2,
            &SpectrumConfigureRequest {
                form: SpectrumForm::Collection,
                entries: vec![
                    entry(TrackPostInput, "vocal", ParameterChannel::Left),
                    entry(Output, "out", ParameterChannel::Right),
                ],
                maximum_capture_bytes: (1 << 40) + 7,
                hop_frames: Some(512),
            },
        ),
        configure(
            "configure_empty",
            3,
            &SpectrumConfigureRequest {
                form: SpectrumForm::Collection,
                entries: Vec::new(),
                maximum_capture_bytes: 0,
                hop_frames: None,
            },
        ),
        capture("capture_arm", 4, &SpectrumCaptureRequest::Arm),
        capture(
            "capture_read",
            5,
            &SpectrumCaptureRequest::Read {
                channels: ParameterChannel::Left,
                include_window: false,
            },
        ),
        capture("capture_cancel", 6, &SpectrumCaptureRequest::Cancel),
        capture(
            "capture_select",
            7,
            &SpectrumCaptureRequest::Select {
                target_kind: Output,
                target_id: "out".to_owned(),
                channels: ParameterChannel::Right,
            },
        ),
        stream(
            "stream_start",
            8,
            &SpectrumStreamRequest::Start { smoothing_ms: 37.5 },
        ),
        stream(
            "stream_select",
            9,
            &SpectrumStreamRequest::Select {
                target_kind: TrackPostPan,
                target_id: "vocal".to_owned(),
                channels: ParameterChannel::Both,
                smoothing_ms: Some(100.0),
            },
        ),
        stream(
            "stream_select_keep_smoothing",
            10,
            &SpectrumStreamRequest::Select {
                target_kind: TrackPostInput,
                target_id: "vocal".to_owned(),
                channels: ParameterChannel::Left,
                smoothing_ms: None,
            },
        ),
        stream("stream_read", 11, &SpectrumStreamRequest::Read),
        stream("stream_reset", 12, &SpectrumStreamRequest::Reset),
        stream("stream_stop", (1 << 53) + 1, &SpectrumStreamRequest::Stop),
    ]
}

struct Reply {
    name: &'static str,
    bytes: Vec<u8>,
    /// What the SDK decoder must report, as JSON.
    expect: String,
}

fn success(
    name: &'static str,
    message: MessageId,
    request_id: u64,
    revision: u64,
    payload: SuccessResponsePayload<'_>,
    expect: String,
) -> Reply {
    let frame = TypedSuccessResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(revision),
        payload,
    };
    let mut bytes = vec![0_u8; 32 * 1024];
    let length = ProtocolCodec::default()
        .encode_success_response_frame_into(&frame, &mut bytes)
        .expect("the reply encodes");
    bytes.truncate(length);
    Reply {
        name,
        bytes,
        expect: format!(
            "{{ \"message\": \"{}\", \"requestId\": \"{request_id}\", \"revision\": \"{revision}\", \"ok\": true, {expect} }}",
            message_name(message),
        ),
    }
}

fn message_name(message: MessageId) -> &'static str {
    match message {
        MessageId::SpectrumConfigure => "configure",
        MessageId::SpectrumCapture => "capture",
        MessageId::SpectrumStream => "stream",
        _ => panic!("the fixture holds spectrum replies only"),
    }
}

fn configured(name: &'static str, request_id: u64, value: SpectrumConfigured) -> Reply {
    let plan = value
        .plan_epoch
        .map_or_else(|| "null".to_owned(), |epoch| format!("\"{epoch}\""));
    let expect = format!(
        "\"configuredEpoch\": \"{}\", \"adoptedEpoch\": \"{}\", \"rebuild\": {}, \"planEpoch\": {plan}",
        value.configured_epoch, value.adopted_epoch, value.rebuild,
    );
    success(
        name,
        MessageId::SpectrumConfigure,
        request_id,
        value.revision,
        SuccessResponsePayload::SpectrumConfigured(value),
        expect,
    )
}

fn plane_json(plane: Option<Plane>) -> String {
    plane.map_or_else(|| "null".to_owned(), Plane::json)
}

struct Ready {
    kind: SpectrumTargetKind,
    id: &'static str,
    channels: ParameterChannel,
    captured_sample: u64,
    snapshot_token: u64,
    source_underrun: bool,
}

fn planes(channels: ParameterChannel) -> (Option<Plane>, Option<Plane>) {
    match channels {
        ParameterChannel::Left => (Some(LEFT), None),
        ParameterChannel::Right => (None, Some(RIGHT)),
        ParameterChannel::Both => (Some(LEFT), Some(RIGHT)),
    }
}

fn capture_reply(
    name: &'static str,
    request_id: u64,
    (adopted_epoch, selection_epoch): (u64, u64),
    status: SpectrumCaptureStatus,
    ready: Option<&Ready>,
) -> Reply {
    let window = ready.map(|ready| {
        let (left, right) = planes(ready.channels);
        SpectrumCaptureWindow {
            target_kind: ready.kind,
            target_id: Cow::Borrowed(ready.id),
            channels: ready.channels,
            sample_rate_hz: 48_000,
            window_frames: 2_048,
            bin_count: 1_025,
            floor_db: -120.0,
            captured_sample: ready.captured_sample,
            end_sample: ready.captured_sample + 2_048,
            snapshot_token: ready.snapshot_token,
            source_underrun: ready.source_underrun,
            frequencies_hz: Cow::Owned(FREQUENCIES.words()),
            left_db: left.map(|plane| Cow::Owned(plane.words())),
            right_db: right.map(|plane| Cow::Owned(plane.words())),
            left_window: None,
            right_window: None,
        }
    });
    let status_name = match status {
        SpectrumCaptureStatus::Done => "done",
        SpectrumCaptureStatus::Pending => "pending",
        SpectrumCaptureStatus::Ready => "ready",
    };
    let window_json = ready.map_or_else(
        || "null".to_owned(),
        |ready| {
            let (left, right) = planes(ready.channels);
            format!(
                "{{ \"target\": {}, \"channels\": \"{}\", \"sampleRateHz\": 48000, \"windowFrames\": 2048, \
                 \"binCount\": 1025, \"floorDb\": -120, \"capturedSample\": \"{}\", \"endSample\": \"{}\", \
                 \"snapshotToken\": \"{}\", \"sourceUnderrun\": {}, \"frequenciesHz\": {}, \"leftDb\": {}, \"rightDb\": {} }}",
                target_json(ready.kind, ready.id),
                channel_name(ready.channels),
                ready.captured_sample,
                ready.captured_sample + 2_048,
                ready.snapshot_token,
                ready.source_underrun,
                FREQUENCIES.json(),
                plane_json(left),
                plane_json(right),
            )
        },
    );
    success(
        name,
        MessageId::SpectrumCapture,
        request_id,
        5,
        SuccessResponsePayload::SpectrumCaptured(SpectrumCaptureReply {
            adopted_epoch,
            selection_epoch,
            status,
            window,
        }),
        format!(
            "\"adoptedEpoch\": \"{adopted_epoch}\", \"selectionEpoch\": \"{selection_epoch}\", \
             \"status\": \"{status_name}\", \"window\": {window_json}"
        ),
    )
}

fn stream_status_name(status: SpectrumStreamStatus) -> &'static str {
    match status {
        SpectrumStreamStatus::Inactive => "inactive",
        SpectrumStreamStatus::Warming => "warming",
        SpectrumStreamStatus::Pending => "pending",
        SpectrumStreamStatus::Gap => "gap",
        SpectrumStreamStatus::Failed => "failed",
        SpectrumStreamStatus::Stopped => "stopped",
        SpectrumStreamStatus::Ready => "ready",
    }
}

/// One stream reply: the metadata words are distinct per field, so a decoder that reads one
/// field's tag for another reports a wrong value.
fn stream_reply(
    name: &'static str,
    request_id: u64,
    status: SpectrumStreamStatus,
    target: Option<(SpectrumTargetKind, &'static str, ParameterChannel)>,
    ready: bool,
) -> Reply {
    let base = if ready { 2_048 } else { 0 };
    let value = SpectrumStreamReply {
        adopted_epoch: 4,
        selection_epoch: 6,
        status,
        target: target.map(|(kind, id, channels)| SpectrumStreamTarget {
            target_kind: kind,
            target_id: Cow::Borrowed(id),
            channels,
        }),
        sample_rate_hz: 44_100,
        quantum_frames: 128,
        hop_frames: 512,
        capture_epoch: 3,
        sequence: 8,
        dropped_captures: 2,
        windows: 9,
        captured_sample: (1 << 33) + base,
        end_sample: (1 << 33) + base + 2_048,
        analysis_epoch: 11,
        history_start_sample: 1_024,
        smoothing_ms: 62.5,
        source_underrun: ready,
        bins: if ready {
            let (left, right) = planes(target.expect("a ready reply has a target").2);
            Some(SpectrumStreamBins {
                frequencies_hz: Cow::Owned(FREQUENCIES.words()),
                left_db: left.map(|plane| Cow::Owned(plane.words())),
                right_db: right.map(|plane| Cow::Owned(plane.words())),
            })
        } else {
            None
        },
    };
    let target_json =
        target.map_or_else(|| "null".to_owned(), |(kind, id, _)| target_json(kind, id));
    let channels = target.map_or_else(
        || "null".to_owned(),
        |(_, _, channels)| format!("\"{}\"", channel_name(channels)),
    );
    let bins = if ready {
        let (left, right) = planes(target.expect("a ready reply has a target").2);
        format!(
            "{{ \"frequenciesHz\": {}, \"leftDb\": {}, \"rightDb\": {} }}",
            FREQUENCIES.json(),
            plane_json(left),
            plane_json(right)
        )
    } else {
        "null".to_owned()
    };
    let expect = format!(
        "\"adoptedEpoch\": \"4\", \"selectionEpoch\": \"6\", \"status\": \"{}\", \"target\": {target_json}, \
         \"channels\": {channels}, \"sampleRateHz\": 44100, \"quantumFrames\": 128, \"hopFrames\": 512, \
         \"captureEpoch\": \"3\", \"sequence\": \"8\", \"droppedCaptures\": \"2\", \"windows\": \"9\", \
         \"capturedSample\": \"{}\", \"endSample\": \"{}\", \"analysisEpoch\": \"11\", \
         \"historyStartSample\": \"1024\", \"smoothingMs\": 62.5, \"sourceUnderrun\": {ready}, \"bins\": {bins}",
        stream_status_name(status),
        value.captured_sample,
        value.end_sample,
    );
    success(
        name,
        MessageId::SpectrumStream,
        request_id,
        5,
        SuccessResponsePayload::SpectrumStreamed(value),
        expect,
    )
}

fn refusal(
    name: &'static str,
    request_id: u64,
    message: MessageId,
    (status, status_name): (StatusCode, &'static str),
    code: &'static str,
    backpressure: Option<Backpressure>,
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
        backpressure,
    };
    let frame = TypedNonOkResponseFrame {
        request_id: RequestId::new(request_id).expect("nonzero"),
        revision: SessionRevision(5),
        message_id: message,
        status,
        payload: &payload,
    };
    let mut bytes = vec![0_u8; 1024];
    let length = ProtocolCodec::default()
        .encode_non_ok_response_frame_into(&frame, &mut bytes)
        .expect("the refusal encodes");
    bytes.truncate(length);
    Reply {
        name,
        bytes,
        expect: format!(
            "{{ \"message\": \"{}\", \"requestId\": \"{request_id}\", \"revision\": \"5\", \"ok\": false, \
             \"status\": \"{status_name}\", \"code\": \"{code}\" }}",
            message_name(message),
        ),
    }
}

fn replies() -> Vec<Reply> {
    use SpectrumTargetKind::{Output, TrackPostInput, TrackPostPan};
    vec![
        configured(
            "configured_unchanged",
            21,
            SpectrumConfigured {
                revision: 7,
                configured_epoch: 3,
                adopted_epoch: 3,
                rebuild: false,
                plan_epoch: None,
            },
        ),
        configured(
            "configured_rebuild",
            22,
            SpectrumConfigured {
                revision: (1 << 40) + 1,
                configured_epoch: 4,
                adopted_epoch: 3,
                rebuild: true,
                plan_epoch: Some(9),
            },
        ),
        capture_reply(
            "capture_done",
            23,
            (1, 0),
            SpectrumCaptureStatus::Done,
            None,
        ),
        capture_reply(
            "capture_pending",
            24,
            (2, 5),
            SpectrumCaptureStatus::Pending,
            None,
        ),
        capture_reply(
            "capture_ready_left",
            25,
            (3, 0),
            SpectrumCaptureStatus::Ready,
            Some(&Ready {
                kind: TrackPostPan,
                id: "vocal",
                channels: ParameterChannel::Left,
                captured_sample: 4_096,
                snapshot_token: 3,
                source_underrun: true,
            }),
        ),
        capture_reply(
            "capture_ready_both",
            26,
            (3, 7),
            SpectrumCaptureStatus::Ready,
            Some(&Ready {
                kind: Output,
                id: "out",
                channels: ParameterChannel::Both,
                captured_sample: (1 << 40) + 5,
                snapshot_token: (1 << 53) + 3,
                source_underrun: false,
            }),
        ),
        stream_reply(
            "stream_inactive",
            27,
            SpectrumStreamStatus::Inactive,
            None,
            false,
        ),
        stream_reply(
            "stream_gap",
            28,
            SpectrumStreamStatus::Gap,
            Some((TrackPostInput, "vocal", ParameterChannel::Both)),
            false,
        ),
        stream_reply(
            "stream_ready_right",
            29,
            SpectrumStreamStatus::Ready,
            Some((TrackPostInput, "vocal", ParameterChannel::Right)),
            true,
        ),
        refusal(
            "capture_busy",
            30,
            MessageId::SpectrumCapture,
            (StatusCode::Backpressure, "backpressure"),
            "spectrum.busy",
            Some(Backpressure {
                queue_kind: BackpressureQueueKind::SpectrumArm,
                capacity: 1,
                occupancy: 1,
                requested_items: 1,
                generation: None,
                retry_boundary: None,
                requested_bytes: None,
                available_bytes: None,
            }),
        ),
        refusal(
            "configure_target_unknown",
            31,
            MessageId::SpectrumConfigure,
            (StatusCode::NotFound, "not_found"),
            "spectrum.target.unknown",
            None,
        ),
        refusal(
            "stream_inactive_refused",
            32,
            MessageId::SpectrumStream,
            (StatusCode::ValidationFailed, "validation_failed"),
            "spectrum.stream.inactive",
            None,
        ),
    ]
}

fn render() -> String {
    let mut text = String::new();
    text.push_str("{\n  \"schema\": \"miso.protocol.spectrum-frames\",\n  \"commands\": [\n");
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

/// Gate 1 of #1596: the checked-in fixture is exactly what the protocol codec writes.
///
/// Red if the codec changes a byte of a spectrum command or reply without the fixture (and with it
/// the SDK codec the SDK eval holds to this file) following.
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
