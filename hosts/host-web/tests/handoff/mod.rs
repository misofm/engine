//! The shared helpers of the worker handoff tests (#1332 S6, S8): each test binary is one process
//! with its own handoff registry and reservation list, and these are the helpers both binaries use.

#![allow(dead_code)]

use host_web::{RESULT_OK, WebBootOptions, native_staging};

/// A thread that panics while its partner waits on a barrier would hang the test: the guard
/// aborts the binary instead, so a failure is a failure.
pub(crate) struct AbortOnPanic;

impl Drop for AbortOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!("a handoff thread panicked; aborting so its partner does not wait forever");
            std::process::abort();
        }
    }
}

pub(crate) const FIXTURE: &str =
    include_str!("../../../../fixtures/session/v1/parametric-eq-nine-track.json");
pub(crate) const SOURCE_ID: &[u8] = b"fixture-source";
pub(crate) const QUANTUM_FRAMES: u32 = 128;
/// Blocks fed before the first render: the ring the options ask for holds them all.
pub(crate) const FED_BLOCKS: usize = 40;
/// A TLS block of the size and alignment a module reports (a native test has no `__tls_size`).
pub(crate) const TLS_SIZE: u32 = 256;
pub(crate) const TLS_ALIGN: u32 = 16;

pub(crate) fn options() -> WebBootOptions {
    WebBootOptions {
        live_control_meter_blocks: 2,
        source_ring_frames: 64 * QUANTUM_FRAMES,
        ..WebBootOptions::explicit_defaults()
    }
}

/// Boot the fixture on this thread's instance and feed `FED_BLOCKS` quanta.
pub(crate) fn boot() -> u32 {
    let length = native_staging::boot(options(), FIXTURE.as_bytes()).expect("the fixture stages");
    let handle = host_web::miso_engine_web_v1_boot(length);
    assert_ne!(handle, 0, "the fixture boots");
    for block in 0..FED_BLOCKS {
        assert!(native_staging::source_id(handle, SOURCE_ID));
        assert!(native_staging::source_pcm(
            handle,
            0.05 + 0.01 * block as f32
        ));
        let result = host_web::miso_engine_web_v1_source_submit(
            handle,
            SOURCE_ID.len() as u32,
            1,
            block as u64 * u64::from(QUANTUM_FRAMES),
            2,
            QUANTUM_FRAMES,
            0,
        );
        assert_eq!(result, RESULT_OK, "block {block}: the feed");
    }
    handle
}

/// One block's output planes as bits, and the render result.
pub(crate) fn render(handle: u32) -> (u32, Vec<u32>) {
    let result = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
    let bits = native_staging::render_output(handle)
        .map(|(pcm, _, _)| pcm.iter().map(|sample| sample.to_bits()).collect())
        .unwrap_or_default();
    (result, bits)
}

/// Render `blocks` blocks, polling the meters after each as the worklet does; the bits.
pub(crate) fn render_blocks(handle: u32, blocks: usize, bits: &mut Vec<Vec<u32>>) {
    for _ in 0..blocks {
        let (result, block) = render(handle);
        assert_eq!(result, RESULT_OK, "block {}: the render", bits.len());
        let _ = host_web::miso_engine_web_v1_meter_poll(handle);
        bits.push(block);
    }
}
