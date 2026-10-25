//! Issue #1333 gate 8: a booted instance never allocates a staging inside a render-locked window.
//!
//! The two stagings (response, observation) are allocated on the boot's one success path (issue
//! #1333 D5). Were one still lazily allocated on first touch, a booted worklet would allocate it
//! inside the render-locked window the first time it read observations or responses. The browser
//! gate sees that only if its workload happens to touch each staging first inside the window;
//! this binary touches each one first through a render-locked export, on a fresh thread, and reads
//! the instance's counter. The browser reads spectrum through the session's frames on the control
//! side (#1597), so no spectrum staging remains.
//!
//! This binary registers the module's own counting allocator and holds exactly one test, so
//! nothing else shares the process-wide counter (decision 15: host-web's native allocation-count
//! gates live in an integration binary, never in `src/tests.rs`, which registers its own).

use std::alloc::System;

use host_web::{
    ABI_VERSION, LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_REQUEST_BYTES,
    OBSERVATION_SELECTION_BYTES, RACK_INSERTS, RESPONSE_CHANNEL_BOTH, RESPONSE_GRID_LINEAR,
    RESULT_OK, RenderLockedAllocator, WebBootOptions, WebLiveResponseRequest,
    WebObservationSelection, native_staging,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The qualification's observation session: one track whose insert compressor declares tap 1.
const DOCUMENT: &[u8] = include_bytes!("../qualification/observation-session.json");
const TRACK_ID: &[u8] = b"track";
const QUANTUM_FRAMES: u32 = 128;

/// Every staging accessor of issue #1333 D2, through its export, once each. Each one is the first
/// touch of its staging after boot, inside its render-locked window.
fn touch_every_staging_accessor() {
    let _ = host_web::miso_engine_web_v1_observation_id_ptr();
    let _ = host_web::miso_engine_web_v1_observation_id_capacity();
    let _ = host_web::miso_engine_web_v1_observation_selection_ptr();
    let _ = host_web::miso_engine_web_v1_observation_selection_bytes();
    let _ = host_web::miso_engine_web_v1_observation_selection_capacity();
    let _ = host_web::miso_engine_web_v1_observation_result_ptr();
    let _ = host_web::miso_engine_web_v1_observation_result_bytes();
    let _ = host_web::miso_engine_web_v1_track_response_request_ptr();
    let _ = host_web::miso_engine_web_v1_track_response_request_bytes();
    let _ = host_web::miso_engine_web_v1_track_response_track_id_ptr();
    let _ = host_web::miso_engine_web_v1_track_response_track_id_capacity();
    let _ = host_web::miso_engine_web_v1_track_response_snapshot_ptr();
    let _ = host_web::miso_engine_web_v1_track_response_snapshot_capacity();
    let _ = host_web::miso_engine_web_v1_track_response_result_ptr();
    let _ = host_web::miso_engine_web_v1_track_response_result_bytes();
}

fn render(handle: u32, blocks: u32) {
    for _ in 0..blocks {
        assert_eq!(
            host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
            RESULT_OK
        );
        let _ = host_web::miso_engine_web_v1_meter_poll(handle);
    }
}

fn boot_options() -> WebBootOptions {
    WebBootOptions {
        live_control_meter_blocks: 2,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    }
}

fn boot() -> u32 {
    let length = native_staging::boot(boot_options(), DOCUMENT).expect("the fixture stages");
    host_web::miso_engine_web_v1_boot(length)
}

/// Every staging accessor and the two staging reads, first touched after boot.
fn booted_stagings_never_allocate_in_the_render_locked_window() {
    let handle = boot();
    assert_ne!(handle, 0, "the observation session boots");
    let before = host_web::miso_engine_web_v1_render_allocation_count();

    touch_every_staging_accessor();
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "a staging accessor allocated inside its render-locked window"
    );

    // The two staging reads, each over a real staged request.
    native_staging::observation_selections(&[WebObservationSelection {
        struct_size: OBSERVATION_SELECTION_BYTES,
        abi_version: ABI_VERSION,
        track_index: 0,
        rack: u32::from(RACK_INSERTS),
        effect_index: 0,
        tap_id: 1,
        channels: 3,
        reserved: 0,
    }]);
    native_staging::track_response_request(
        WebLiveResponseRequest {
            struct_size: LIVE_RESPONSE_REQUEST_BYTES,
            abi_version: ABI_VERSION,
            track_id_bytes: TRACK_ID.len() as u32,
            grid: RESPONSE_GRID_LINEAR,
            channels: RESPONSE_CHANNEL_BOTH,
            points: 5,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
            maximum_result_bytes: LIVE_RESPONSE_CAPTURE_BYTES as u32,
            reserved: [0; 3],
        },
        TRACK_ID,
    );
    render(handle, 64);
    assert_eq!(
        host_web::miso_engine_web_v1_observation_read(handle, 1),
        RESULT_OK
    );
    assert_eq!(
        host_web::miso_engine_web_v1_track_response_capture(handle),
        RESULT_OK
    );
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "a render-locked export allocated"
    );
    assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
}

#[test]
fn render_locked_reads_never_allocate_after_boot() {
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
    // A fresh thread is a fresh thread-local block: no staging exists until that thread boots.
    std::thread::spawn(booted_stagings_never_allocate_in_the_render_locked_window)
        .join()
        .expect("the staging phase completes");
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
}
