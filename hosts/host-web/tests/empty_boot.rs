//! #1682 (browser part): a session with no source and no track boots on the browser module, and a
//! structural transaction adds the first stereo stem and its track.
//!
//! Both gates run through the module's exports on a fresh thread, as the worklet makes the calls,
//! inside the render-locked window, so the instance's counters read what a render call and a meter
//! poll allocated and freed:
//!
//! * **Boot.** The empty mixer (one console slot stays declared) reports one consistent shape: the
//!   status and the watermark of the boot revision, a meter frame of the three master words with no
//!   track and no submix, no source in the session map, a control list that names the session's own
//!   rows and no strip, and silence at the output, with zero render-locked allocations.
//! * **Growth.** One structural transaction adds a source, a track and a route. The successor adopts
//!   at the next block boundary without a render-locked allocation, the meter frame grows to the
//!   strip's words, the session map lists the stem, and the stem fed through the export is heard.
//!
//! The bit-for-bit comparison with the C ABI is `control_parity.rs`.

#![cfg(feature = "test-support")]

use std::alloc::System;

use host_web::{RESULT_OK, RenderLockedAllocator, WebBootOptions, native_staging};
use protocol::{
    CommandPayload, ControlListRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, TypedCommandFrame,
};
use session::{RouteSource, SessionModel, StableId, canonical_session_json, parse_session_json};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM_FRAMES: u32 = 128;
const STEM: &[u8] = b"stem";

/// The fixture cut to an empty mixer: no source, no track, no route and no VCA. The console slot
/// stays declared, so the track added later carries it.
fn empty_model(rate: u32) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM_FRAMES;
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    model.vcas.clear();
    assert_eq!(model.console.pre_insert.len(), 1, "a declared console slot");
    model
}

/// The stem, its track and the route of that track into the output.
fn stem_edits() -> Vec<SessionEdit> {
    let fixture = parse_session_json(FIXTURE).expect("the fixture parses");
    let mut source = fixture.sources[0].clone();
    source.id = StableId::parse("stem").expect("source ID");
    source.frames = u64::from(QUANTUM_FRAMES) * 64;
    let mut track = fixture.tracks[0].clone();
    track.id = StableId::parse("t1").expect("track ID");
    track.source_id = source.id.clone();
    let mut route = fixture.routes[0].clone();
    route.id = StableId::parse("t1-main").expect("route ID");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("the fixture routes a track")
    };
    *track_id = track.id.clone();
    vec![
        SessionEdit::UpsertSource { source },
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn frame(payload: CommandPayload<'_>, request: u64, expected: ExpectedRevision) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: expected,
                payload,
            },
            &mut bytes,
        )
        .expect("a command frame");
    bytes.truncate(length);
    bytes
}

/// The rows of the control list, paged through the export, and the structure epoch.
fn control_list(handle: u32, request: &mut u64) -> (u64, Vec<String>) {
    let mut rows = Vec::new();
    let (mut epoch, mut after) = (0, 0);
    loop {
        *request += 1;
        let list = ControlListRequest {
            structure_epoch: epoch,
            after_handle: after,
            limit: 64,
            owner: None,
            headers_only: false,
        };
        let (result, bytes) = native_staging::control_command(
            handle,
            &frame(
                CommandPayload::ControlListGet(&list),
                *request,
                ExpectedRevision::Any,
            ),
        )
        .expect("a live handle");
        assert_eq!(result, RESULT_OK, "the control list");
        let mut fields = [0_u16; 4096];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ControlListPage(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("a control page")
        else {
            panic!("expected a control list page")
        };
        rows.extend(page.entries.iter().map(|entry| format!("{entry:?}")));
        if page.eof {
            return (page.structure_epoch, rows);
        }
        (epoch, after) = (page.structure_epoch, page.last_handle);
    }
}

/// One render call and the meter poll after it, with the render-locked allocations they made.
fn render_and_poll(handle: u32) -> (Vec<f32>, u32, u32) {
    let before = host_web::miso_engine_web_v1_render_allocation_count();
    let result = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
    assert_eq!(result, RESULT_OK, "the render");
    let windows = host_web::miso_engine_web_v1_meter_poll(handle);
    let allocations = host_web::miso_engine_web_v1_render_allocation_count() - before;
    (
        native_staging::output_pcm(handle).expect("output"),
        windows,
        allocations,
    )
}

/// The empty mixer boots at every launch rate, and the status, the watermark, the meter frame, the
/// session map and the control list describe one empty mixer. A structural transaction adds the
/// first stem: the render half allocates nothing across the swap, and the stem is heard.
///
/// Test value: red if a browser boot-time size fails at zero (a strip order, a meter frame or its
/// header, the source map, the observation tables or the staging), if the meter frame of an empty
/// mixer is not the master's three words or its header counts a strip, if render or a poll allocates
/// on an empty plan or across the first strip's swap, or if the first strip's frame, source map or
/// audio does not arrive through the successor path.
#[test]
fn an_empty_session_boots_and_grows_by_a_structural_transaction() {
    for rate in RATES {
        std::thread::spawn(move || {
            let model = empty_model(rate);
            let options = WebBootOptions {
                require_sample_rate_hz: rate,
                require_quantum_frames: QUANTUM_FRAMES,
                live_control_meter_blocks: 1,
                ..WebBootOptions::explicit_defaults()
            };
            let document = canonical_session_json(&model).expect("canonical session");
            let length = native_staging::boot(options, document.as_bytes()).expect("stages");
            let handle = host_web::miso_engine_web_v1_boot(length);
            assert_ne!(handle, 0, "{rate} Hz: the empty session boots");
            assert_eq!(
                host_web::miso_engine_web_v1_meter_lease(handle, 1),
                RESULT_OK
            );

            let status = native_staging::status(handle).expect("a status");
            assert_eq!(
                (status.state, status.sample_rate_hz, status.quantum_frames),
                (host_web::STATE_READY, rate, QUANTUM_FRAMES),
                "{rate} Hz: ready"
            );
            let mut request = 0;
            let (empty_epoch, empty_rows) = control_list(handle, &mut request);
            assert!(
                empty_rows
                    .first()
                    .is_some_and(|row| row.contains("owner_kind: Session"))
                    && empty_rows
                        .iter()
                        .all(|row| !row.contains("owner_kind: Strip")),
                "{rate} Hz: only the session owns rows: {empty_rows:?}"
            );

            let mut published = 0;
            for block in 0..4 {
                let (pcm, windows, allocations) = render_and_poll(handle);
                assert!(
                    pcm.iter().all(|sample| sample.to_bits() == 0),
                    "{rate} Hz: silence"
                );
                assert_eq!(allocations, 0, "{rate} Hz: block {block} allocates nothing");
                if windows > 0 {
                    published += 1;
                    let (header, words) = native_staging::meter_frame(handle).expect("a frame");
                    assert_eq!(
                        (header.track_count, header.submix_count, words.len()),
                        (0, 0, 3),
                        "{rate} Hz: the master's words alone"
                    );
                    assert!(
                        words.iter().all(|word| *word == 0.0),
                        "{rate} Hz: silent meters"
                    );
                }
            }
            assert!(
                published > 0,
                "{rate} Hz: the master's frames are published"
            );
            let status = native_staging::status(handle).expect("a status");
            assert_eq!(status.rendered_quanta, 4);
            assert_eq!(
                (status.watermark_revision, status.watermark_first_sample),
                (model.revision, 0),
                "{rate} Hz: the boot watermark"
            );

            // One structural transaction adds the first stem.
            request += 1;
            let revision = host_web::miso_engine_web_v1_control_revision(handle);
            let edits = stem_edits();
            let (result, reply) = native_staging::control_command(
                handle,
                &frame(
                    CommandPayload::SessionTransactionApply(&edits),
                    request,
                    ExpectedRevision::Exact(SessionRevision(revision)),
                ),
            )
            .expect("a live handle");
            assert_eq!(result, RESULT_OK, "{rate} Hz: the transaction publishes");
            let mut fields = [0_u16; 64];
            let DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
                .expect("a reply")
            else {
                panic!("{rate} Hz: the transaction is refused")
            };
            assert_eq!(applied.edit_path, EditPath::Rebuild);

            let mut heard = false;
            let mut grown = false;
            for block in 4..14 {
                if block == 5 {
                    // The swap block ran; a seek opens a fresh generation at the render clock.
                    assert!(native_staging::source_id(handle, STEM));
                    assert_eq!(
                        host_web::miso_engine_web_v1_source_seek(handle, STEM.len() as u32, 2, 0),
                        RESULT_OK
                    );
                }
                if block >= 5 {
                    assert!(native_staging::source_id(handle, STEM));
                    assert!(native_staging::source_pcm(handle, 0.25));
                    assert_eq!(
                        host_web::miso_engine_web_v1_source_submit(
                            handle,
                            STEM.len() as u32,
                            2,
                            (block - 5) * u64::from(QUANTUM_FRAMES),
                            2,
                            QUANTUM_FRAMES,
                            0,
                        ),
                        RESULT_OK,
                        "{rate} Hz: the stem takes PCM"
                    );
                }
                let (pcm, windows, allocations) = render_and_poll(handle);
                assert_eq!(allocations, 0, "{rate} Hz: block {block} allocates nothing");
                heard |= pcm.iter().any(|sample| *sample != 0.0);
                if windows > 0 {
                    let (header, words) = native_staging::meter_frame(handle).expect("a frame");
                    if header.track_count == 1 {
                        assert_eq!(words.len(), 3 + 3, "{rate} Hz: the strip's words");
                        grown = true;
                    }
                }
                if block == 4 {
                    assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
                }
            }
            assert!(heard, "{rate} Hz: the stem is heard");
            assert!(grown, "{rate} Hz: the meter frame grew to the strip");
            let (epoch, rows) = control_list(handle, &mut request);
            assert!(epoch > empty_epoch, "{rate} Hz: the structure epoch moved");
            assert!(
                rows.len() > empty_rows.len() && rows.iter().any(|row| row.contains("t1")),
                "{rate} Hz: the strip's rows are listed"
            );
            assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        })
        .join()
        .expect("the run completes");
    }
}
