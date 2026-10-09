//! Issue #1488 D3: the post-boot control exports the AudioWorklet calls on its render thread never
//! allocate inside their render-locked windows.
//!
//! The PCM-feed worklet calls `source_submit` and `source_seek` from `process()`; the engine
//! worklet's port handlers call them, the EQ and input-filter configuration copies, the prepared
//! command admission, the meter lease and the spectrum arm, cancel, stream start and stream stop
//! on the same thread. Each export wraps its body in `render_locked`, so an allocation in it, or in
//! what it calls, moves the instance's render-allocation count. This binary drives every one of
//! them through its export on a fresh thread, accepted and refused, and reads the count after each
//! call.
//!
//! This binary registers the module's own counting allocator and holds exactly one test, so
//! nothing else shares the process-wide counter (decision 15: host-web's native allocation-count
//! gates live in an integration binary, never in `src/tests.rs`, which registers its own).

use std::alloc::System;

use host_web::{
    ABI_VERSION, COMMAND_MATRIX, COMMAND_RECORD_BYTES, RACK_CONSOLE, RACK_NOT_APPLICABLE,
    RESULT_BACKPRESSURE, RESULT_INVALID_ARGUMENT, RESULT_OK, RESULT_UNSUPPORTED,
    RESULT_WRONG_STATE, RenderLockedAllocator, SPECTRUM_CAPTURE_BYTES, SPECTRUM_CHANNEL_BOTH,
    SPECTRUM_REQUEST_BYTES, SPECTRUM_TARGET_TRACK_POST_PAN, WebBootOptions, WebSpectrumRequest,
    native_staging,
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
/// The fixed prepared-companion header: size, ABI version, host generation, target count, reserved.
const COMPANION_HEADER_BYTES: usize = 24;

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
        live_control_command_queue_records: 64,
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

/// The configuration copies and the prepared admission, in the worklet's order: the SDK reads the
/// host generation from a copied configuration before it builds a companion.
fn configuration_and_command_exports(handle: u32, before: u32) {
    unchanged(
        before,
        "input_filters_config_copy",
        host_web::miso_engine_web_v1_input_filters_config_copy(handle, 0),
        RESULT_OK,
    );
    unchanged(
        before,
        "input_filters_config_copy (refused: no such strip)",
        host_web::miso_engine_web_v1_input_filters_config_copy(handle, 99),
        RESULT_UNSUPPORTED,
    );
    unchanged(
        before,
        "eq_target_config_copy (refused: retired rack)",
        host_web::miso_engine_web_v1_eq_target_config_copy(handle, 0, 0, 0),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(
        before,
        "eq_target_config_copy",
        host_web::miso_engine_web_v1_eq_target_config_copy(handle, 0, u32::from(RACK_CONSOLE), 0),
        RESULT_OK,
    );
    // A native address does not fit the export's `u32`, so both calls return zero here; the
    // claim is about the window, not the address.
    let pointer = host_web::miso_engine_web_v1_eq_target_config_ptr(handle);
    unchanged(before, "eq_target_config_ptr", pointer, pointer);
    unchanged(
        before,
        "eq_target_config_ptr (refused: no host)",
        host_web::miso_engine_web_v1_eq_target_config_ptr(0),
        0,
    );

    let mut config = [0_u8; 24];
    assert!(native_staging::eq_target_config(handle, &mut config));
    let generation = &config[16..24];
    let mut companion = [0_u8; COMPANION_HEADER_BYTES];
    companion[0..4].copy_from_slice(&(COMPANION_HEADER_BYTES as u32).to_le_bytes());
    companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    companion[8..16].copy_from_slice(generation);
    assert!(native_staging::prepared_companion(handle, &companion));
    // One ordinary matrix record for track 0, with no prepared target.
    let mut record = [0_u8; COMMAND_RECORD_BYTES as usize];
    record[0] = COMMAND_MATRIX as u8;
    record[1] = RACK_NOT_APPLICABLE;
    record[2] = 255;
    for (slot, value) in [0.5_f32, 0.0, 0.0, 1.0].iter().enumerate() {
        record[24 + slot * 4..28 + slot * 4].copy_from_slice(&value.to_le_bytes());
    }
    assert!(native_staging::command_records(handle, &record));
    let header = COMPANION_HEADER_BYTES as u32;
    unchanged(
        before,
        "prepared_command_submit (refused: truncated companion)",
        host_web::miso_engine_web_v1_prepared_command_submit(handle, 1, header - 1),
        RESULT_INVALID_ARGUMENT,
    );
    unchanged(
        before,
        "prepared_command_submit",
        host_web::miso_engine_web_v1_prepared_command_submit(handle, 1, header),
        RESULT_OK,
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

fn post_boot_control_exports_never_allocate_in_the_render_locked_window() {
    let handle = boot();
    assert_ne!(handle, 0, "the observation session boots");
    render(handle, 4);
    let before = host_web::miso_engine_web_v1_render_allocation_count();
    source_exports(handle, before);
    render(handle, 4);
    configuration_and_command_exports(handle, before);
    render(handle, 4);
    lease_and_spectrum_exports(handle, before);
    render(handle, 4);
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before
    );
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
