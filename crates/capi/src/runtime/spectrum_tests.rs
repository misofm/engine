//! #1594 gate 9 (scope note): the C ABI carries `SPECTRUM_CONFIGURE` to the session core.
//! #1595 grows this file with the spectrum reads.

use crate::runtime::tests::{SESSION, boxed_c_children, command_bytes_at_revision, command_c};
use protocol::{
    DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame, ExpectedRevision,
    ParameterChannel, ProtocolCodec, SpectrumConfigureRequest, SpectrumEntry, SpectrumForm,
    SpectrumTargetKind,
};

/// Turns red if the C ABI does not transport the command to the core: one `SPECTRUM_CONFIGURE`
/// frame through `miso_engine_v1_submit_command` answers OK with `rebuild` true, and after render
/// blocks and one `miso_engine_v1_service` the same frame again reports adopted epoch 1 (the
/// session's spectrum monitor promoted the plan).
#[test]
fn a_spectrum_configure_crosses_the_c_abi_to_the_session() {
    let (session, plan) = boxed_c_children(SESSION);
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
    let reply = |request_id| {
        let frame = command_bytes_at_revision(
            request_id,
            ExpectedRevision::Any,
            protocol::CommandPayload::SpectrumConfigure(&request),
        );
        let (result, response) = command_c(session, &frame);
        assert_eq!(result, crate::RESULT_OK);
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
    let first = reply(1);
    assert!(first.rebuild);
    assert_eq!((first.configured_epoch, first.adopted_epoch), (1, 0));
    let mut pcm = vec![0.0_f32; 256];
    for block in 0..4_u64 {
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
            crate::ffi::test_render(plan, block * 128, &output),
            crate::RESULT_OK
        );
    }
    assert_eq!(crate::ffi::test_service(session), crate::RESULT_OK);
    // The same configuration again changes nothing and reports the adoption.
    let again = reply(2);
    assert!(!again.rebuild);
    assert_eq!((again.configured_epoch, again.adopted_epoch), (1, 1));
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);
}
