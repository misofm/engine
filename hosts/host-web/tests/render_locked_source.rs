//! Issue #1488 D3: the post-boot control exports the AudioWorklet calls on its render thread never
//! allocate inside their render-locked windows.
//!
//! The PCM-feed worklet calls `source_submit` and `source_seek` from `process()`; the engine
//! worklet's port handlers call them, the observation command admission, the meter lease and the
//! spectrum arm, cancel, stream start and stream stop on the same thread. Each export wraps its body in `render_locked`, so an allocation in it, or in
//! what it calls, moves the instance's render-allocation count. This binary drives every one of
//! them through its export on a fresh thread, accepted and refused, and reads the count after each
//! call.
//!
//! This binary registers the module's own counting allocator and holds exactly one test, so
//! nothing else shares the process-wide counter (decision 15: host-web's native allocation-count
//! gates live in an integration binary, never in `src/tests.rs`, which registers its own).

use std::alloc::System;

use host_web::{
    ABI_VERSION, COMMAND_OBSERVE_SUBSCRIBE, COMMAND_RECORD_BYTES, RACK_CONSOLE,
    RACK_NOT_APPLICABLE, RESULT_BACKPRESSURE, RESULT_INVALID_ARGUMENT, RESULT_OK,
    RESULT_WRONG_STATE, RenderLockedAllocator, SPECTRUM_CAPTURE_BYTES, SPECTRUM_CHANNEL_BOTH,
    SPECTRUM_REQUEST_BYTES, SPECTRUM_TARGET_TRACK_POST_PAN, WebBootOptions, WebSpectrumRequest,
    native_staging,
};
use protocol::{
    CommandPayload, CounterId, DiagnosticSeverity, ExpectedRevision, ProtocolCodec, RequestId,
    SessionRevision, TelemetryConfiguration, TypedCommandFrame,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The qualification's observation session: one stereo track over `live-control-source` (2048
/// frames), with a console EQ in its pre-insert slot.
const DOCUMENT: &[u8] = include_bytes!("../qualification/observation-session.json");
const SOURCE_ID: &[u8] = b"live-control-source";
const SOURCE_FRAMES: u64 = 2048;
const TRACK_ID: &[u8] = b"track";
const QUANTUM_FRAMES: u32 = 128;

fn boot() -> u32 {
    native_staging::spectrum_request(
        WebSpectrumRequest {
            struct_size: SPECTRUM_REQUEST_BYTES,
            abi_version: ABI_VERSION,
            target: SPECTRUM_TARGET_TRACK_POST_PAN,
            channels: SPECTRUM_CHANNEL_BOTH,
            target_id_bytes: TRACK_ID.len() as u32,
            maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
            ..WebSpectrumRequest::default()
        },
        TRACK_ID,
    );
    let options = WebBootOptions {
        live_control_meter_blocks: 2,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    };
    let length = native_staging::boot(options, DOCUMENT).expect("the fixture stages");
    host_web::miso_engine_web_v1_boot(length)
}

fn render(handle: u32, blocks: u32) {
    for _ in 0..blocks {
        assert_eq!(
            host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
            RESULT_OK
        );
    }
}

/// Call one export, check its result, and check that the render-allocation count did not move.
fn unchanged(before: u32, call: &str, result: u32, expected: u32) {
    assert_eq!(result, expected, "{call} returned an unexpected result");
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "{call} allocated inside its render-locked window"
    );
}

fn source_exports(handle: u32, before: u32) {
    let id = SOURCE_ID.len() as u32;
    assert!(native_staging::source_id(handle, SOURCE_ID));
    assert!(native_staging::source_pcm(handle, 0.25));
    let submit = host_web::miso_engine_web_v1_source_submit;
    unchanged(
        before,
        "source_submit (full quantum)",
        submit(handle, id, 1, 0, 2, QUANTUM_FRAMES, 0),
        RESULT_OK,
    );
    unchanged(
        before,
        "source_submit (refused: end_of_region 2)",
        submit(
            handle,
            id,
            1,
            u64::from(QUANTUM_FRAMES),
            2,
            QUANTUM_FRAMES,
            2,
        ),
        RESULT_INVALID_ARGUMENT,
    );
    let short = QUANTUM_FRAMES / 2;
    let tail = SOURCE_FRAMES - u64::from(short);
    let seek = host_web::miso_engine_web_v1_source_seek;
    unchanged(before, "source_seek", seek(handle, id, 2, tail), RESULT_OK);
    unchanged(
        before,
        "source_seek (refused: ID past its staging)",
        seek(handle, u32::MAX, 3, 0),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(
        before,
        "source_submit (end-of-region short chunk)",
        submit(handle, id, 2, tail, 2, short, 1),
        RESULT_OK,
    );
}

/// The observation command admission, accepted-empty and refused: the worklet's port handler calls
/// `command_submit` between render quanta, and its bodies never allocate.
fn command_exports(handle: u32, before: u32) {
    let submit = host_web::miso_engine_web_v1_command_submit;
    unchanged(
        before,
        "command_submit (empty)",
        submit(handle, 0),
        RESULT_OK,
    );
    // A deleted value kind (3 was `faderDb`) is malformed at its index and moves nothing.
    let mut record = [0_u8; COMMAND_RECORD_BYTES as usize];
    record[0] = 3;
    record[1] = RACK_NOT_APPLICABLE;
    assert!(native_staging::command_records(handle, &record));
    unchanged(
        before,
        "command_submit (refused: a deleted kind)",
        submit(handle, 1),
        RESULT_INVALID_ARGUMENT,
    );
    // An observe record naming a key the plan does not have is refused at the key.
    let mut record = [0_u8; COMMAND_RECORD_BYTES as usize];
    record[0] = COMMAND_OBSERVE_SUBSCRIBE as u8;
    record[1] = RACK_CONSOLE;
    record[2] = 5;
    record[3] = 4;
    record[4..8].copy_from_slice(&1_u32.to_le_bytes());
    record[16..21].copy_from_slice(b"track");
    record[144..148].copy_from_slice(b"slot");
    assert!(native_staging::command_records(handle, &record));
    unchanged(
        before,
        "command_submit (refused: unknown key)",
        submit(handle, 1),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(
        before,
        "command_submit (refused: past the staging)",
        submit(handle, u32::MAX),
        RESULT_INVALID_ARGUMENT,
    );
}

fn lease_and_spectrum_exports(handle: u32, before: u32) {
    let lease = host_web::miso_engine_web_v1_meter_lease;
    unchanged(before, "meter_lease (take)", lease(handle, 1), RESULT_OK);
    unchanged(
        before,
        "meter_lease (refused: 2)",
        lease(handle, 2),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(before, "meter_lease (release)", lease(handle, 0), RESULT_OK);

    unchanged(
        before,
        "spectrum_arm",
        host_web::miso_engine_web_v1_spectrum_arm(handle),
        RESULT_OK,
    );
    unchanged(
        before,
        "spectrum_cancel",
        host_web::miso_engine_web_v1_spectrum_cancel(handle),
        RESULT_OK,
    );
    let start = host_web::miso_engine_web_v1_spectrum_stream_start;
    unchanged(
        before,
        "spectrum_stream_start (refused: NaN smoothing)",
        start(handle, f64::NAN),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(
        before,
        "spectrum_stream_start",
        start(handle, 100.0),
        RESULT_OK,
    );
    unchanged(
        before,
        "spectrum_arm (refused: stream active)",
        host_web::miso_engine_web_v1_spectrum_arm(handle),
        RESULT_BACKPRESSURE,
    );
    unchanged(
        before,
        "spectrum_cancel (refused: stream active)",
        host_web::miso_engine_web_v1_spectrum_cancel(handle),
        RESULT_WRONG_STATE,
    );
    let stop = host_web::miso_engine_web_v1_spectrum_stream_stop;
    unchanged(before, "spectrum_stream_stop", stop(handle), RESULT_OK);
    unchanged(
        before,
        "spectrum_stream_stop (refused: no host)",
        stop(0),
        RESULT_INVALID_ARGUMENT,
    );
}

/// Batch 7 review B1: with counter telemetry configured, the first control call after a render
/// stages a counter snapshot (an allocation). The PCM feed calls the source exports from
/// `process()` right after a render, so they must not run the service step: the snapshot is the
/// message handler's.
fn source_exports_after_a_render_with_counter_telemetry_never_allocate(handle: u32) {
    let revision = session::parse_session_json(std::str::from_utf8(DOCUMENT).expect("UTF-8"))
        .expect("the session parses")
        .revision;
    let mut bytes = vec![0_u8; 4 << 10];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
                    meter_handles: Vec::new(),
                    meter_period_blocks: 0,
                    counter_ids: vec![CounterId::LiveValuesSuperseded],
                    counter_period_blocks: 1,
                    diagnostics_enabled: false,
                    minimum_diagnostic_severity: DiagnosticSeverity::Info,
                }),
            },
            &mut bytes,
        )
        .expect("the telemetry frame");
    let (result, _) =
        native_staging::control_command(handle, &bytes[..length]).expect("the frame stages");
    assert_eq!(result, RESULT_OK, "counter telemetry is configured");
    render(handle, 2);
    let before = host_web::miso_engine_web_v1_render_allocation_count();
    let id = SOURCE_ID.len() as u32;
    assert!(native_staging::source_id(handle, SOURCE_ID));
    assert!(native_staging::source_pcm(handle, 0.25));
    unchanged(
        before,
        "source_seek after a render with counter telemetry",
        host_web::miso_engine_web_v1_source_seek(handle, id, 8, 0),
        RESULT_OK,
    );
    render(handle, 1);
    unchanged(
        before,
        "source_submit after a render with counter telemetry",
        host_web::miso_engine_web_v1_source_submit(handle, id, 8, 0, 2, QUANTUM_FRAMES, 0),
        RESULT_OK,
    );
}

fn post_boot_control_exports_never_allocate_in_the_render_locked_window() {
    let handle = boot();
    assert_ne!(handle, 0, "the observation session boots");
    render(handle, 4);
    let before = host_web::miso_engine_web_v1_render_allocation_count();
    source_exports(handle, before);
    render(handle, 4);
    command_exports(handle, before);
    render(handle, 4);
    lease_and_spectrum_exports(handle, before);
    render(handle, 4);
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before
    );
    source_exports_after_a_render_with_counter_telemetry_never_allocate(handle);
    assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
}

#[test]
fn post_boot_control_exports_are_render_locked_and_allocation_free() {
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
    // A fresh thread is a fresh thread-local block: the stagings and the host exist only there.
    std::thread::spawn(post_boot_control_exports_never_allocate_in_the_render_locked_window)
        .join()
        .expect("the control-export phase completes");
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
}
