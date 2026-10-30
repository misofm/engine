//! #240 A9: the pinned parse/model-build multiplier is measured, not trusted by inspection.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use std::alloc::{GlobalAlloc, System};

use host_core::{compile_host_model, parse_host_session};
use host_web::{
    AudioWorkletEngineHost, PARSE_TRANSIENT_MULTIPLIER, RESULT_REFUSED_BUDGET, WebBootOptions,
};
use session::{
    CompileCaps, RouteSource, SendTap, StableId, canonical_session_json, parse_session_json,
};

thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<usize> = const { Cell::new(0) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

struct PeakAllocator;

#[global_allocator]
static ALLOCATOR: PeakAllocator = PeakAllocator;

fn allocated(bytes: usize) {
    if !ARMED.try_with(Cell::get).unwrap_or(false) {
        return;
    }
    ALLOCATIONS.with(|count| count.set(count.get() + 1));
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
    DEALLOCATIONS.with(|count| count.set(count.get() + 1));
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
    ALLOCATIONS.with(|count| count.set(0));
    DEALLOCATIONS.with(|count| count.set(0));
    ARMED.with(|armed| armed.set(true));
    let result = operation();
    ARMED.with(|armed| armed.set(false));
    let peak = PEAK.with(Cell::get);
    (result, peak)
}

#[test]
fn paused_seek_recycles_without_allocating_or_freeing() {
    use host_web::{RESULT_BACKPRESSURE, RESULT_OK};
    let mut model = parse_session_json(include_str!("browser-v1/session.json")).unwrap();
    model.sources[0].frames = 480_000;
    let document = canonical_session_json(&model).unwrap();
    let options = WebBootOptions {
        source_ring_frames: 512,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap();
    let samples = [0.25; 128];
    let planes: [&[f32]; 2] = [&samples, &samples];
    for generation in 1..=3 {
        let origin = (generation - 1) * 10_000;
        for block in 0..4 {
            assert_eq!(
                host.submit_source(
                    b"fixture-source",
                    generation,
                    origin + block * 128,
                    48_000,
                    &planes,
                    128,
                    false
                ),
                RESULT_OK
            );
        }
        assert_eq!(
            host.submit_source(
                b"fixture-source",
                generation,
                origin + 512,
                48_000,
                &planes,
                128,
                false
            ),
            RESULT_BACKPRESSURE
        );
        if generation == 2 {
            // Exercise recycled/retained storage after a real render, then refill it.
            assert_eq!(host.render_next(), RESULT_OK);
            assert_eq!(
                host.submit_source(
                    b"fixture-source",
                    generation,
                    origin + 512,
                    48_000,
                    &planes,
                    128,
                    false
                ),
                RESULT_OK
            );
        }
        let clock = host.status().next_absolute_sample;
        let rendered = host.status().rendered_quanta;
        let (result, peak) = measured_peak(|| {
            host.prepare_source_seek(b"fixture-source", generation + 1, generation * 10_000)
        });
        assert_eq!(result, RESULT_OK);
        assert_eq!(peak, 0);
        assert_eq!(ALLOCATIONS.with(Cell::get), 0);
        assert_eq!(DEALLOCATIONS.with(Cell::get), 0);
        assert_eq!(host.status().next_absolute_sample, clock);
        assert_eq!(host.status().rendered_quanta, rendered);
    }
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

/// A dense accepted JSON shape: `tracks` tracks, four effect declarations per track, each with a
/// post-pan route to the main output, followed by JSON whitespace to `pad_to` bytes when given
/// (1 MiB, the staging ceiling before issue #2191). `examples/dense_document.rs` emits the same
/// shape at 6,000 tracks for the wasm leg of the dense gate (`scripts/check-web-boot-budget.mjs`).
fn dense_document(tracks: usize, pad_to: Option<usize>) -> Vec<u8> {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("seed fixture parses");
    let mut track = model.tracks[1].clone();
    // Four full effect declarations per track, as inserts: the densest per-track declaration
    // the schema accepts (a console entry is smaller than the effect it configures).
    let effect = model.lower_track(&track).pre_insert[0].clone();
    model.console.pre_insert.clear();
    track.console.clear();
    track.inserts.effects.clear();
    for index in 0..4 {
        let mut effect = effect.clone();
        effect.id = StableId::parse(&format!("effect-{index}")).expect("effect ID");
        track.inserts.effects.push(effect);
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
            tap: SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    let mut document = canonical_session_json(&model)
        .expect("worst accepted shape canonicalizes")
        .into_bytes();
    if let Some(maximum) = pad_to {
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
        (1, false, dense_document(1, None)),
        (64, false, dense_document(64, None)),
        (192, false, dense_document(192, None)),
        (192, true, dense_document(192, Some(1 << 20))),
        // Issue #2191 gate 5: a dense document of about 4 MiB, four times the retired 1 MiB
        // ceiling, which the budget-derived limit now admits.
        (1536, false, dense_document(1536, None)),
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

    let document = dense_document(192, Some(1 << 20));
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

/// Issue #2191: a 1 MiB control frame under a budget below its control-plane rows (about 4.3 MB)
/// but far above the document's parse projection is refused before the parser or the control
/// plane allocates anything, with a diagnostic that names the frame and its rows.
///
/// Test value: red if the boot's pre-allocation admission leaves out the frame-scaled rows: the
/// boot then compiles, allocates those rows, and only the exact check refuses it, after the memory
/// (permanently, on wasm) grew past the budget.
#[test]
fn a_frame_the_budget_cannot_hold_is_refused_before_anything_is_allocated() {
    let document = include_bytes!("../../../fixtures/session/v1/canonical-minimal.json");
    let (failure, peak) = measured_peak(|| {
        AudioWorkletEngineHost::boot(
            document,
            WebBootOptions {
                maximum_memory_bytes: 4 << 20,
                maximum_control_frame_bytes: 1 << 20,
                ..WebBootOptions::default()
            },
        )
        .err()
        .expect("a frame over the budget refuses")
    });
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
    let diagnostic = String::from_utf8_lossy(failure.diagnostic()).into_owned();
    assert!(
        diagnostic.starts_with("host.budget.parse_projection\t")
            && diagnostic.contains(",control_frame_bytes=1048576,control_frame_rows_bytes="),
        "{diagnostic}"
    );
    assert!(
        peak < document.len() + 4096,
        "the refusal allocated {peak} bytes; nothing past the diagnostic may be allocated"
    );
}

#[path = "common/effect_state_document.rs"]
#[allow(dead_code)]
mod effect_state_document;

/// Issue #2194 gate C: a session whose declared effect state fits the effect cap alone but whose
/// retained projection does not (128 tracks with four delays each, about 400 MB of delay state,
/// under a budget of exactly that state) is refused by the retained projection, which names its
/// effect row, before the control plane compiles or prepares anything: the boot's peak stays
/// inside the document's parse transient.
///
/// Test value: red when the retained projection has no effect row. Host-core's effect cap admits
/// the state, so the boot then prepares every delay before the exact aggregate refuses it, peaking
/// at about the whole budget (on wasm, a linear memory that never shrinks).
#[test]
fn effect_state_the_budget_cannot_hold_is_refused_before_preparation() {
    let document = effect_state_document::document(128);
    let model = parse_host_session(core::str::from_utf8(&document).expect("UTF-8"))
        .expect("the document parses");
    let compiled = compile_host_model(&model, unlimited_caps()).expect("the document compiles");
    let declared = host_core::project_effects(&compiled)
        .expect("a projection")
        .state_bytes;
    drop((compiled, model));
    assert!(declared <= host_web::DEFAULT_MAXIMUM_MEMORY_BYTES);
    let (failure, peak) = measured_peak(|| {
        AudioWorkletEngineHost::boot(
            &document,
            WebBootOptions {
                maximum_memory_bytes: declared,
                ..WebBootOptions::default()
            },
        )
        .err()
        .expect("the effect state and the other rows over the budget refuse")
    });
    let transient = document.len() as u64 * PARSE_TRANSIENT_MULTIPLIER;
    assert!(
        peak as u64 <= transient,
        "the refusal peaked at {peak} bytes, over the {transient}-byte parse transient"
    );
    let diagnostic = String::from_utf8_lossy(failure.diagnostic()).into_owned();
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET, "{diagnostic}");
    let effect_state_bytes: u64 = diagnostic
        .strip_prefix("host.budget.retained_projection\t$.maximum_memory_bytes[projected_bytes=")
        .and_then(|rest| rest.split_once(&format!(",budget_bytes={declared},effect_state_bytes=")))
        .and_then(|(_, effect)| effect.split_once(",graph_bytes="))
        .and_then(|(effect, _)| effect.parse().ok())
        .unwrap_or_else(|| panic!("{diagnostic}"));
    assert!(effect_state_bytes > declared, "{diagnostic}");
}

/// Issue #2199: the dense 6,000-track shape of #2191 (about 16.5 MB of JSON, four EQ inserts per
/// track) at the default budget is refused by the boot's retained projection, whose graph,
/// builtin and control rows are projected before they are allocated: the boot's peak stays inside
/// the document's parse transient, under the budget.
///
/// Test value: red when the boot projection leaves out the graph, builtin or control rows. The
/// control plane then prepares every effect, binds every bank and compiles the plan (a peak of
/// about 638 MB, over the 512 MiB budget) before the exact aggregate refuses the session.
#[test]
fn the_dense_shape_is_refused_before_its_rows_are_allocated() {
    let document = dense_document(6_000, None);
    let (failure, peak) = measured_peak(|| {
        AudioWorkletEngineHost::boot(&document, WebBootOptions::default())
            .err()
            .expect("the dense shape exceeds the default budget")
    });
    let diagnostic = String::from_utf8_lossy(failure.diagnostic()).into_owned();
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET, "{diagnostic}");
    assert!(
        diagnostic.starts_with("host.budget.retained_projection\t"),
        "{diagnostic}"
    );
    let transient = document.len() as u64 * PARSE_TRANSIENT_MULTIPLIER;
    assert!(
        (peak as u64) <= transient && (peak as u64) < host_web::DEFAULT_MAXIMUM_MEMORY_BYTES,
        "the refusal peaked at {peak} bytes, over the {transient}-byte parse transient"
    );
}
