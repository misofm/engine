//! #1594 gate 9 and #1595 (scope note R5): the C ABI advertises `SPECTRUM_CONFIGURE`,
//! `SPECTRUM_CAPTURE` and `SPECTRUM_STREAM` and carries them to the session core.

use crate::runtime::tests::{
    SESSION, boxed_c_children_with_limits, command_bytes_at_revision, command_c_capacity, limits,
};
use protocol::{
    DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame, ExpectedRevision,
    ParameterChannel, ProtocolCodec, SpectrumCaptureRequest, SpectrumCaptureStatus,
    SpectrumConfigureRequest, SpectrumEntry, SpectrumForm, SpectrumStreamRequest,
    SpectrumStreamStatus, SpectrumTargetKind,
};

/// The frame limit a ready read needs: the default fixture's 4,096 bytes carry no ready reply,
/// so a configure there is refused (`spectrum.frame`, #1595 R6).
const FRAME_BYTES: u64 = 65_536;

/// Turns red if a C ABI client cannot discover or reach the spectrum commands: `CAPABILITIES_GET`
/// lists `0x0010`, `0x0011` and `0x0012` with capability bit 16 (#1595 gate 9); one
/// `SPECTRUM_CONFIGURE` frame through `miso_engine_v1_submit_command` answers OK with `rebuild`
/// true, and after render blocks and one `miso_engine_v1_service` the same frame again reports
/// adopted epoch 1 (the session's spectrum monitor promoted the plan); then an arm, 2,048
/// rendered frames and a read answer a ready window of `eq0` with token 1, and a stream started
/// then reads `ready` with both planes within a few hops.
#[test]
fn spectrum_commands_cross_the_c_abi_to_the_session() {
    let (session, plan) = boxed_c_children_with_limits(
        SESSION,
        crate::CompileLimits {
            maximum_control_frame_bytes: FRAME_BYTES,
            maximum_replay_bytes: 4 * FRAME_BYTES,
            ..limits()
        },
    );
    let request = SpectrumConfigureRequest {
        form: SpectrumForm::Single,
        entries: vec![SpectrumEntry {
            target_kind: SpectrumTargetKind::TrackPostPan,
            target_id: "eq0".to_owned(),
            channels: ParameterChannel::Both,
        }],
        maximum_capture_bytes: 1 << 20,
        hop_frames: None,
    };
    fn send(
        session: *mut crate::Session,
        request_id: u64,
        payload: protocol::CommandPayload<'_>,
    ) -> Vec<u8> {
        let frame = command_bytes_at_revision(request_id, ExpectedRevision::Any, payload);
        let (result, _, response) = command_c_capacity(session, &frame, FRAME_BYTES as usize);
        assert_eq!(result, crate::RESULT_OK);
        response
    }
    let configured = |request_id| {
        let response = send(
            session,
            request_id,
            protocol::CommandPayload::SpectrumConfigure(&request),
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SpectrumConfigured(reply),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a response frame")
        else {
            panic!("a spectrum reply");
        };
        reply
    };
    let captured = |request_id, capture: &SpectrumCaptureRequest| {
        let response = send(
            session,
            request_id,
            protocol::CommandPayload::SpectrumCapture(capture),
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SpectrumCaptured(reply),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a response frame")
        else {
            panic!("a capture reply");
        };
        reply
    };
    let mut pcm = vec![0.0_f32; 256];
    let mut frame = 0_u64;
    let mut render = |blocks: u64| {
        for _ in 0..blocks {
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: 128,
                plane_stride_samples: 128,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(plan, frame, &output),
                crate::RESULT_OK
            );
            frame += 128;
        }
        assert_eq!(crate::ffi::test_service(session), crate::RESULT_OK);
    };
    let first = configured(1);
    assert!(first.rebuild);
    assert_eq!((first.configured_epoch, first.adopted_epoch), (1, 0));
    render(4);
    // The same configuration again changes nothing and reports the adoption.
    let again = configured(2);
    assert!(!again.rebuild);
    assert_eq!((again.configured_epoch, again.adopted_epoch), (1, 1));

    assert_eq!(
        captured(3, &SpectrumCaptureRequest::Arm).status,
        SpectrumCaptureStatus::Done
    );
    render(16);
    let ready = captured(
        4,
        &SpectrumCaptureRequest::Read {
            channels: ParameterChannel::Both,
            include_window: false,
        },
    );
    assert_eq!(ready.status, SpectrumCaptureStatus::Ready);
    let window = ready.window.expect("a ready window");
    assert_eq!(
        (window.target_id.as_ref(), window.snapshot_token),
        ("eq0", 1)
    );

    let streamed = |request_id, stream: &SpectrumStreamRequest| {
        let response = send(
            session,
            request_id,
            protocol::CommandPayload::SpectrumStream(stream),
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SpectrumStreamed(reply),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a response frame")
        else {
            panic!("a stream reply");
        };
        reply
    };
    assert_eq!(
        streamed(5, &SpectrumStreamRequest::Start { smoothing_ms: 0.0 }).status,
        SpectrumStreamStatus::Warming
    );
    let mut request_id = 6;
    let ready = loop {
        render(1);
        let reply = streamed(request_id, &SpectrumStreamRequest::Read);
        if reply.status == SpectrumStreamStatus::Ready {
            break reply;
        }
        request_id += 1;
        assert!(request_id < 70, "the stream reads ready");
    };
    let bins = ready.bins.expect("ready bins");
    assert!(bins.left_db.is_some() && bins.right_db.is_some());

    let response = send(session, 1_000, protocol::CommandPayload::CapabilitiesGet);
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::Capabilities(capabilities),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("a response frame")
    else {
        panic!("a capabilities reply");
    };
    let commands = capabilities
        .supported_commands
        .as_chunks::<2>()
        .0
        .iter()
        .map(|id| u16::from_le_bytes(*id))
        .collect::<Vec<_>>();
    assert!(
        [0x0010, 0x0011, 0x0012]
            .iter()
            .all(|id| commands.contains(id)),
        "{commands:x?}"
    );
    assert_ne!(
        capabilities.flags.0 & protocol::CapabilityFlags::SPECTRUM.0,
        0
    );
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);
}
