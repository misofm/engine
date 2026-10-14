//! #1574: the session-less response entry, `serve_response_frame`.
//!
//! * **Refusals a session-less caller can reach.** An address target, a missing rate or quantum,
//!   a message that is not `RESPONSE_GET` and a header that does not decode are each answered or
//!   refused as the entry documents.
//! * **The caller's limits.** The request's own `maximum_reply_bytes` lowers the reply bound.
//!
//! The byte equality with a C ABI session is `crates/capi/tests/response_get.rs`.

#![cfg(feature = "control-provider")]

use host_core::{ResponseFrameError, serve_response_frame};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    ExpectedRevision, MessageId, ParameterChannel, ProtocolCodec, RequestId, ResponseConfiguration,
    ResponseGetRequest, ResponseGrid, ResponseResult, ResponseTarget, StatusCode,
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
