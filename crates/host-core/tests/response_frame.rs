//! #1574: the session-less response entry, `serve_response_frame`.
//!
//! * **Refusals a session-less caller can reach.** An address target, a missing rate or quantum,
//!   a message that is not `RESPONSE_GET` and a header that does not decode are each answered or
//!   refused as the entry documents.
//! * **The caller's limits.** The request's own `maximum_reply_bytes` lowers the reply bound.
//! * **The core's refusals, not the client's (#1623).** A logarithmic axis from zero, a quality the
//!   effect does not publish, a lane given twice, a non-finite endpoint or override value and a
//!   bypass value between 0 and 1 are each refused with a status and code of the core. The browser SDK sends them as given and shows exactly these
//!   (`sdk/test/response-evals.mjs`); `crates/capi/tests/response_get.rs` holds the C ABI session
//!   to the same codes.
//!
//! The byte equality with a C ABI session is `crates/capi/tests/response_get.rs`.

#![cfg(feature = "control-provider")]

use host_core::{ResponseFrameError, serve_response_frame};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    ExpectedRevision, MessageId, ParameterChannel, ProtocolCodec, RequestId, ResponseConfiguration,
    ResponseGetRequest, ResponseGrid, ResponseOverride, ResponseResult, ResponseTarget, StatusCode,
    TypedCommandFrame,
};

fn encode(payload: CommandPayload<'_>) -> Vec<u8> {
    let mut bytes = vec![0_u8; 1 << 16];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(0x1574).expect("request ID"),
                expected_revision: ExpectedRevision::Any,
                payload,
            },
            &mut bytes,
        )
        .expect("command frame");
    bytes.truncate(length);
    bytes
}

fn eq_target() -> ResponseTarget {
    ResponseTarget::Configuration(ResponseConfiguration::Effect {
        effect: "miso.parametric-eq".to_owned(),
        quality: session::EffectQuality::Normal,
        link_mode: session::LinkMode::DualMono,
    })
}

fn request() -> ResponseGetRequest {
    ResponseGetRequest {
        target: eq_target(),
        grid: ResponseGrid::Logarithmic,
        points: 64,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: ParameterChannel::Both,
        sections: true,
        overrides: Vec::new(),
        configuration_id: None,
        sample_rate_hz: Some(48_000),
        quantum_frames: Some(128),
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    }
}

/// The status and diagnostic codes of a non-OK reply, or `None` for a success.
fn refusal(reply: &[u8]) -> Option<(StatusCode, MessageId, Vec<String>)> {
    let mut fields = [0_u16; 1024];
    match ProtocolCodec::default()
        .decode_typed_response(reply, &mut DecodeScratch::new(&mut fields))
        .expect("a typed response")
    {
        DecodedTypedResponseFrame::NonOk { header, payload } => Some((
            header.status,
            header.message_id,
            payload.diagnostics.into_iter().map(|d| d.code).collect(),
        )),
        DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::ResponseResult(ResponseResult::Curves(_)),
        } => {
            assert_eq!(header.request_id.get(), 0x1574);
            assert_eq!(header.revision.0, 0);
            None
        }
        DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
    }
}

fn serve(query: &ResponseGetRequest) -> Vec<u8> {
    let mut reply = Vec::new();
    serve_response_frame(&encode(CommandPayload::ResponseGet(query)), &mut reply)
        .expect("a command frame");
    reply
}

#[test]
fn a_request_with_a_rate_and_a_quantum_is_answered() {
    assert_eq!(refusal(&serve(&request())), None);
}

#[test]
fn a_request_the_entry_cannot_resolve_is_refused_with_its_code() {
    let mut strip = request();
    strip.target = ResponseTarget::Strip("track-1".to_owned());
    strip.sections = false;
    let mut no_rate = request();
    no_rate.sample_rate_hz = None;
    let mut no_quantum = request();
    no_quantum.quantum_frames = None;
    let mut bad_rate = request();
    bad_rate.sample_rate_hz = Some(192_000);
    let mut small_reply = request();
    small_reply.maximum_reply_bytes = Some(100);
    for (name, query, status, code) in [
        (
            "a strip target",
            strip,
            StatusCode::Unavailable,
            "response.target.unavailable",
        ),
        (
            "no rate",
            no_rate,
            StatusCode::InvalidField,
            "response.rate",
        ),
        (
            "no quantum",
            no_quantum,
            StatusCode::InvalidField,
            "response.rate",
        ),
        (
            "an unsupported rate",
            bad_rate,
            StatusCode::InvalidField,
            "response.rate",
        ),
        (
            "a reply over the caller's bound",
            small_reply,
            StatusCode::LimitExceeded,
            "response.reply_too_large",
        ),
    ] {
        let reply = serve(&query);
        let (got_status, message, codes) = refusal(&reply).unwrap_or_else(|| panic!("{name}"));
        assert_eq!(
            (got_status, message),
            (status, MessageId::ResponseGet),
            "{name}"
        );
        assert_eq!(codes, [code], "{name}");
    }
}

/// The refusals a client sends as given, with the status and code the core answers. The SDK's
/// `response-evals.mjs` lists the same rows; `crates/capi/tests/response_get.rs` lists them for a
/// C ABI session.
///
/// Red if a client-side check is put back in front of one of them and the core's answer drifts, or
/// if the core stops refusing one: nothing then stands between a client and the engine.
#[test]
fn the_core_refuses_what_a_thin_client_sends_as_given() {
    let mut zero_log = request();
    zero_log.minimum_hz = 0.0;
    let with_quality = |quality| {
        let mut query = request();
        query.target = ResponseTarget::Configuration(ResponseConfiguration::Effect {
            effect: "miso.parametric-eq".to_owned(),
            quality,
            link_mode: session::LinkMode::DualMono,
        });
        query
    };
    let mut twice = request();
    twice.overrides = vec![
        ResponseOverride {
            control: 4,
            lane: ParameterChannel::Both,
            value: 1.0,
        },
        ResponseOverride {
            control: 4,
            lane: ParameterChannel::Left,
            value: 2.0,
        },
    ];
    let mut one_point = request();
    one_point.points = 1;
    for (name, query, status, code) in [
        (
            "a logarithmic axis from zero",
            zero_log,
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "one point",
            one_point,
            StatusCode::InvalidField,
            "response.points",
        ),
        (
            "draft quality",
            with_quality(session::EffectQuality::Draft),
            StatusCode::ValidationFailed,
            "effect.quality.unsupported",
        ),
        (
            "high quality",
            with_quality(session::EffectQuality::High),
            StatusCode::ValidationFailed,
            "effect.quality.unsupported",
        ),
        (
            "a lane given twice",
            twice,
            StatusCode::InvalidField,
            "response.override.duplicate",
        ),
    ] {
        let reply = serve(&query);
        let (got_status, message, codes) = refusal(&reply).unwrap_or_else(|| panic!("{name}"));
        assert_eq!(
            (got_status, message),
            (status, MessageId::ResponseGet),
            "{name}"
        );
        assert_eq!(codes, [code], "{name}");
    }
}

/// `frame` with the one four-byte occurrence of `from` replaced by `to`.
fn patched(mut frame: Vec<u8>, from: f32, to: f32) -> Vec<u8> {
    let from = from.to_le_bytes();
    let at: Vec<usize> = frame
        .windows(4)
        .enumerate()
        .filter(|(_, window)| *window == from)
        .map(|(at, _)| at)
        .collect();
    assert_eq!(at.len(), 1, "the value is in the frame exactly once");
    frame[at[0]..at[0] + 4].copy_from_slice(&to.to_le_bytes());
    frame
}

/// #1623 (root ruling, thick core): a frame the Rust encoder refuses to write -- a NaN or infinite
/// endpoint, a NaN override value, a bypass value between 0 and 1 -- is still a frame a client can
/// send. The decoder reads it as sent and the core refuses it with its own typed code, never the
/// codec's generic `protocol.failure`.
///
/// Red if the decoder refuses the value again (each code becomes `protocol.failure`), or if the core
/// stops checking the value before the owner is prepared.
#[test]
fn a_non_finite_or_bad_value_sent_raw_is_refused_with_the_core_code() {
    let mut with_rows = request();
    with_rows.overrides = vec![
        ResponseOverride {
            control: protocol::RESPONSE_BYPASS_CONTROL,
            lane: ParameterChannel::Both,
            value: 1.0,
        },
        ResponseOverride {
            control: 4,
            lane: ParameterChannel::Both,
            value: 3.25,
        },
    ];
    let frame = encode(CommandPayload::ResponseGet(&with_rows));
    assert_eq!(
        refusal(&{
            let mut reply = Vec::new();
            serve_response_frame(&frame, &mut reply).expect("a command frame");
            reply
        }),
        None,
        "the unpatched frame is answered"
    );
    for (name, from, to, status, code) in [
        (
            "a NaN minimum",
            20.0,
            f32::NAN,
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "an infinite maximum",
            20_000.0,
            f32::INFINITY,
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "a NaN override value",
            3.25,
            f32::NAN,
            StatusCode::InvalidField,
            "response.override.value",
        ),
        (
            "an infinite override value",
            3.25,
            f32::NEG_INFINITY,
            StatusCode::InvalidField,
            "response.override.value",
        ),
        (
            "a bypass of one half",
            1.0,
            0.5,
            StatusCode::InvalidField,
            "response.override.bypass",
        ),
    ] {
        let mut reply = Vec::new();
        serve_response_frame(&patched(frame.clone(), from, to), &mut reply)
            .expect("a command frame");
        let (got_status, message, codes) = refusal(&reply).unwrap_or_else(|| panic!("{name}"));
        assert_eq!(
            (got_status, message, codes),
            (status, MessageId::ResponseGet, vec![code.to_owned()]),
            "{name}"
        );
    }
}

#[test]
fn another_message_is_unsupported_and_a_broken_header_has_no_reply() {
    let mut reply = Vec::new();
    serve_response_frame(&encode(CommandPayload::CapabilitiesGet), &mut reply).expect("header");
    let (status, message, _) = refusal(&reply).expect("a refusal");
    assert_eq!(
        (status, message),
        (StatusCode::UnsupportedMessage, MessageId::CapabilitiesGet)
    );
    reply.extend_from_slice(b"stale");
    assert_eq!(
        serve_response_frame(&[0_u8; 10], &mut reply),
        Err(ResponseFrameError::Malformed)
    );
    assert!(reply.is_empty(), "a malformed frame leaves no stale reply");
}
