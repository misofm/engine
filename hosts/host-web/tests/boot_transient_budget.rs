//! #240 A9: the pinned parse/model-build multiplier is measured, not trusted by inspection.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use std::alloc::{GlobalAlloc, System};

use host_core::{compile_host_model, parse_host_session};
use host_web::{
    AudioWorkletEngineHost, MAXIMUM_DOCUMENT_BYTES, PARSE_TRANSIENT_MULTIPLIER,
    RESULT_REFUSED_BUDGET, WebBootOptions,
};
use session::{
    CompileCaps, RouteSource, SendTap, StableId, canonical_session_json, parse_session_json,
};

thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<usize> = const { Cell::new(0) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
}

struct PeakAllocator;

#[global_allocator]
static ALLOCATOR: PeakAllocator = PeakAllocator;

fn allocated(bytes: usize) {
    if !ARMED.try_with(Cell::get).unwrap_or(false) {
        return;
    }
    LIVE.with(|live| {
        let next = live.get().saturating_add(bytes);
        live.set(next);
        PEAK.with(|peak| peak.set(peak.get().max(next)));
    });
}

fn deallocated(bytes: usize) {
    if !ARMED.try_with(Cell::get).unwrap_or(false) {
        return;
    }
    LIVE.with(|live| live.set(live.get().saturating_sub(bytes)));
}

// SAFETY: every allocator request is forwarded unchanged to `System`; the thread-local counters
// observe successful operations but never alter their pointers, sizes, alignments or lifetimes.
unsafe impl GlobalAlloc for PeakAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwards the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            allocated(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwards the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            allocated(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        deallocated(layout.size());
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: forwards the original allocation arguments unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            if new_size >= layout.size() {
                allocated(new_size - layout.size());
            } else {
                deallocated(layout.size() - new_size);
            }
        }
        replacement
    }
}

fn measured_peak<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    ARMED.with(|armed| armed.set(false));
    LIVE.with(|live| live.set(0));
    PEAK.with(|peak| peak.set(0));
    ARMED.with(|armed| armed.set(true));
    let result = operation();
    ARMED.with(|armed| armed.set(false));
    let peak = PEAK.with(Cell::get);
    (result, peak)
}

fn unlimited_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

/// A dense accepted JSON shape: 192 tracks, four effect declarations per track, followed by JSON
/// whitespace to the exact 1 MiB staging ceiling.
fn dense_document(tracks: usize, pad_to_limit: bool) -> Vec<u8> {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    let effect = track.simd1.effects[0].clone();
    track.simd1.effects.clear();
    for index in 0..4 {
        let mut effect = effect.clone();
        effect.id = StableId::parse(&format!("effect-{index}")).expect("effect ID");
        track.simd1.effects.push(effect);
    }
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    model.tracks.reserve(tracks);
    model.routes.reserve(tracks);
    for index in 0..tracks {
        let track_id = StableId::parse(&format!("track-{index:03}")).expect("track ID");
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        model.tracks.push(next_track);

        let mut next_route = route.clone();
        next_route.id = StableId::parse(&format!("route-{index:03}")).expect("route ID");
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostMatrix,
        };
        model.routes.push(next_route);
    }
    let mut document = canonical_session_json(&model)
        .expect("worst accepted shape canonicalizes")
        .into_bytes();
    if pad_to_limit {
        let maximum = MAXIMUM_DOCUMENT_BYTES as usize;
        assert!(
            document.len() <= maximum,
            "{tracks}x4 fixture is {} bytes before padding",
            document.len()
        );
        document.resize(maximum, b' ');
    }
    document
}

#[test]
fn pinned_multiplier_bounds_the_worst_accepted_parse_and_model_build_peak() {
    let cases = [
        (
            0,
            false,
            include_bytes!("../../../fixtures/session/v1/canonical-minimal.json").to_vec(),
        ),
        (1, false, dense_document(1, false)),
        (64, false, dense_document(64, false)),
        (192, false, dense_document(192, false)),
        (192, true, dense_document(192, true)),
    ];
    for (tracks, pad_to_limit, document) in cases {
        let (_, peak) = measured_peak(|| {
            let model = parse_host_session(core::str::from_utf8(&document).expect("UTF-8"))
                .expect("dense shape parses");
            let compiled =
                compile_host_model(&model, unlimited_caps()).expect("dense shape compiles");
            assert_eq!(compiled.normalized_model().tracks.len(), tracks);
            drop(compiled);
            drop(model);
        });
        let pinned = document.len() as u64 * PARSE_TRANSIENT_MULTIPLIER;
        println!(
            "tracks={tracks} padded={pad_to_limit} json_document_bytes={} parse_model_compile_peak={} bytes_per_input={:.3}",
            document.len(),
            peak,
            peak as f64 / document.len() as f64
        );
        assert!(
            peak as u64 <= pinned,
            "measured peak {peak} exceeds {PARSE_TRANSIENT_MULTIPLIER} × {} = {pinned}",
            document.len()
        );
    }

    let document = dense_document(192, true);
    let pinned = document.len() as u64 * PARSE_TRANSIENT_MULTIPLIER;

    let (failure, refused_peak) = measured_peak(|| {
        AudioWorkletEngineHost::boot(
            &document,
            WebBootOptions {
                maximum_memory_bytes: pinned - 1,
                ..WebBootOptions::default()
            },
        )
        .err()
        .expect("one byte below the pre-parse projection refuses")
    });
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
    assert!(
        failure
            .diagnostic()
            .starts_with(b"host.budget.parse_projection\t")
    );
    assert!(
        refused_peak < document.len(),
        "pre-parse refusal allocated {refused_peak} bytes; the parser must not have run"
    );
}
