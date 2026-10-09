//! Issue #1333 gate 8: a booted instance never allocates a staging inside a render-locked window,
//! and (Amendment 1, A1) a collection capture's spectrum reads never allocate there either.
//!
//! The three stagings (response, spectrum, observation) are allocated on the one success path
//! that both boot exports share (issue #1333 D5). Were one still lazily allocated on first touch, a booted worklet would
//! allocate it inside the render-locked window the first time it read observations, responses or
//! spectra. The browser gate sees that only if its workload happens to touch each staging first
//! inside the window; this binary touches each one first through a render-locked export, on a
//! fresh thread, and reads the instance's counter.
//!
//! A collection capture's spectrum and spectrum-stream reads once cloned the selected target's
//! `String` on every read. The browser gate sees that only when a qualification workload reads a
//! collection; the second phase reads one on a second fresh thread.
//!
//! Selecting a collection entry once built an owned target from the staged identity and cloned
//! the selected target into the returned entry, on the browser's audio thread (issue #1492). The
//! second phase therefore counts from before its first select, and selects through both select
//! exports: a change, a repeat, and a stream select that changes the entry and its smoothing.
//!
//! This binary registers the module's own counting allocator and holds exactly one test, so
//! nothing else shares the process-wide counter (decision 15: host-web's native allocation-count
//! gates live in an integration binary, never in `src/tests.rs`, which registers its own).

use std::alloc::System;

use host_web::{
    ABI_VERSION, LIVE_RESPONSE_CAPTURE_BYTES, LIVE_RESPONSE_REQUEST_BYTES,
    OBSERVATION_SELECTION_BYTES, RACK_INSERTS, RESPONSE_CHANNEL_BOTH, RESPONSE_GRID_LINEAR,
    RESULT_BACKPRESSURE, RESULT_OK, RenderLockedAllocator, SPECTRUM_CAPTURE_BYTES,
    SPECTRUM_CHANNEL_BOTH, SPECTRUM_COLLECTION_REQUEST_BYTES, SPECTRUM_REQUEST_BYTES,
    SPECTRUM_TARGET_OUTPUT, SPECTRUM_TARGET_TRACK_POST_PAN, WebBootOptions, WebLiveResponseRequest,
    WebObservationSelection, WebSpectrumCollectionEntry, WebSpectrumCollectionRequest,
    WebSpectrumRequest, native_staging,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The qualification's observation session: one track whose insert compressor declares tap 1.
const DOCUMENT: &[u8] = include_bytes!("../qualification/observation-session.json");
const TRACK_ID: &[u8] = b"track";
const OUTPUT_ID: &[u8] = b"main-out";
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
    let _ = host_web::miso_engine_web_v1_spectrum_target_id_ptr();
    let _ = host_web::miso_engine_web_v1_spectrum_target_id_capacity();
    let _ = host_web::miso_engine_web_v1_spectrum_capture_ptr();
    let _ = host_web::miso_engine_web_v1_spectrum_capture_capacity();
    let _ = host_web::miso_engine_web_v1_spectrum_capture_bytes();
    let _ = host_web::miso_engine_web_v1_spectrum_stream_metadata_ptr();
    let _ = host_web::miso_engine_web_v1_spectrum_stream_metadata_bytes();
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
        live_control_command_queue_records: 64,
        live_control_meter_blocks: 2,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    }
}

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
    let length = native_staging::boot(boot_options(), DOCUMENT).expect("the fixture stages");
    host_web::miso_engine_web_v1_boot(length)
}

/// Boot the same session with a two-entry spectrum collection: the track's post-pan boundary and
/// the main output, in that order.
fn boot_collection() -> u32 {
    let entry = |target, id: &[u8]| WebSpectrumCollectionEntry {
        target,
        channels: SPECTRUM_CHANNEL_BOTH,
        target_id_bytes: id.len() as u32,
        ..WebSpectrumCollectionEntry::default()
    };
    assert!(native_staging::spectrum_collection(
        WebSpectrumCollectionRequest {
            struct_size: SPECTRUM_COLLECTION_REQUEST_BYTES,
            abi_version: ABI_VERSION,
            entry_count: 2,
            maximum_capture_bytes: SPECTRUM_CAPTURE_BYTES as u64,
            ..WebSpectrumCollectionRequest::default()
        },
        &[
            entry(SPECTRUM_TARGET_TRACK_POST_PAN, TRACK_ID),
            entry(SPECTRUM_TARGET_OUTPUT, OUTPUT_ID),
        ],
        &[TRACK_ID, OUTPUT_ID].concat(),
    ));
    let length = native_staging::boot(boot_options(), DOCUMENT).expect("the fixture stages");
    host_web::miso_engine_web_v1_boot(length)
}

/// Phase 1 (gate 8): every staging accessor and the four staging reads, first touched after boot.
fn booted_stagings_never_allocate_in_the_render_locked_window() {
    let handle = boot();
    assert_ne!(
        handle, 0,
        "the observation session boots with a spectrum capture"
    );
    let before = host_web::miso_engine_web_v1_render_allocation_count();

    touch_every_staging_accessor();
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "a staging accessor allocated inside its render-locked window"
    );

    // The four staging reads, each over a real staged request.
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
    assert_eq!(host_web::miso_engine_web_v1_spectrum_arm(handle), RESULT_OK);
    render(handle, 64);
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_BOTH),
        RESULT_OK
    );
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_stream_start(handle, 100.0),
        RESULT_OK
    );
    render(handle, 64);
    let stream = host_web::miso_engine_web_v1_spectrum_stream_read(handle);
    assert!(stream == RESULT_OK || stream == RESULT_BACKPRESSURE);
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

/// Phase 2 (Amendment 1, A1, and issue #1492): a collection capture's selects, spectrum read and
/// spectrum-stream read.
fn collection_spectrum_reads_never_allocate_in_the_render_locked_window() {
    let handle = boot_collection();
    assert_ne!(handle, 0, "the observation session boots with a collection");
    let before = host_web::miso_engine_web_v1_render_allocation_count();
    // Select the second entry, so a read that resolved the wrong entry would not find it armed.
    // The first select changes the entry; the second is a repeat that changes nothing.
    native_staging::spectrum_target_id(OUTPUT_ID);
    for call in ["the first select", "the repeated select"] {
        assert_eq!(
            host_web::miso_engine_web_v1_spectrum_select(
                handle,
                SPECTRUM_TARGET_OUTPUT,
                SPECTRUM_CHANNEL_BOTH,
                OUTPUT_ID.len() as u32,
            ),
            RESULT_OK,
            "{call} was refused"
        );
        assert_eq!(
            host_web::miso_engine_web_v1_render_allocation_count(),
            before,
            "{call} allocated"
        );
    }
    render(handle, 64);
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_read(handle, SPECTRUM_CHANNEL_BOTH),
        RESULT_OK,
        "the selected collection entry completed a window"
    );
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_stream_start(handle, 100.0),
        RESULT_OK
    );
    render(handle, 64);
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_stream_read(handle),
        RESULT_OK,
        "the selected collection entry streamed a window"
    );
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "a collection capture's spectrum read allocated"
    );
    // A stream select to the other entry with new smoothing: the continuous cadence restarts on
    // the new entry and the analysis history resets.
    native_staging::spectrum_target_id(TRACK_ID);
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_stream_select(
            handle,
            SPECTRUM_TARGET_TRACK_POST_PAN,
            SPECTRUM_CHANNEL_BOTH,
            TRACK_ID.len() as u32,
            50.0,
        ),
        RESULT_OK
    );
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "the stream select allocated"
    );
    render(handle, 64);
    assert_eq!(
        host_web::miso_engine_web_v1_spectrum_stream_read(handle),
        RESULT_OK,
        "the newly selected entry streamed a window"
    );
    assert_eq!(
        host_web::miso_engine_web_v1_render_allocation_count(),
        before,
        "a collection capture's stream read allocated after the stream select"
    );
    assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
}

#[test]
fn render_locked_reads_never_allocate_after_boot() {
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
    // A fresh thread is a fresh thread-local block: no staging exists until that thread boots.
    std::thread::spawn(booted_stagings_never_allocate_in_the_render_locked_window)
        .join()
        .expect("the single-capture phase completes");
    std::thread::spawn(collection_spectrum_reads_never_allocate_in_the_render_locked_window)
        .join()
        .expect("the collection phase completes");
    assert_eq!(host_web::miso_engine_web_v1_render_allocation_count(), 0);
}
