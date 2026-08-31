//! E6: allocation-call budgets for the complete 4,096-track control pipeline.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use core::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, System};
use toml::de::DeTable;

use session::{
    CompileCaps, RouteSource, SendTap, StableId, canonical_session_toml, compile_session,
    estimate_session_resources, parse_session_toml,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const N: usize = 4_096;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
thread_local! { static ARMED: Cell<bool> = const { Cell::new(false) }; }

struct CountingAllocator;
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

// SAFETY: all requests are forwarded unchanged to System; only a relaxed counter is added.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_one();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_one();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count_one();
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn count_one() {
    if ARMED.try_with(Cell::get).unwrap_or(false) {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }
}

fn measured<T>(body: impl FnOnce() -> T) -> (T, usize) {
    ARMED.with(|armed| armed.set(false));
    ALLOCATIONS.store(0, Ordering::Relaxed);
    ARMED.with(|armed| armed.set(true));
    let result = body();
    ARMED.with(|armed| armed.set(false));
    (result, ALLOCATIONS.load(Ordering::Relaxed))
}

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn large_model() -> session::SessionToml {
    let mut model = parse_session_toml(CANONICAL).expect("fixture parses");
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    model.tracks.reserve(N);
    model.routes.reserve(N);
    for index in 0..N {
        let track_id = StableId::parse(&format!("track-{index:04}")).expect("track ID");
        let mut next_track = track.clone();
        next_track.id = track_id.clone();
        let mut next_route = route.clone();
        next_route.id = StableId::parse(&format!("route-{index:04}")).expect("route ID");
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostMatrix,
        };
        model.tracks.push(next_track);
        model.routes.push(next_route);
    }
    model
}

#[test]
fn allocation_calls_stay_within_linear_phase_budgets() {
    let model = large_model();
    let text = canonical_session_toml(&model).expect("large model canonicalizes");
    let (raw, raw_parse_allocations) = measured(|| DeTable::parse(&text));
    assert!(raw.is_ok());
    let (parsed, parse_allocations) = measured(|| parse_session_toml(&text));
    assert!(parsed.is_ok());
    let (canonical, canonical_allocations) = measured(|| canonical_session_toml(&model));
    assert!(canonical.is_ok());
    let (compiled, compile_allocations) = measured(|| compile_session(&model, caps()));
    assert!(compiled.is_ok());
    let (estimate, estimate_allocations) = measured(|| estimate_session_resources(&model));
    assert!(estimate.is_ok());
    let owned_model_allocations = parse_allocations - raw_parse_allocations;
    println!(
        "allocation calls: raw_toml={raw_parse_allocations} ({:.3}/track), owned_model={owned_model_allocations} ({:.3}/track), parse={parse_allocations} ({:.3}/track), canonical={canonical_allocations}, compile={compile_allocations} ({:.3}/track), estimate={estimate_allocations}",
        raw_parse_allocations as f64 / N as f64,
        owned_model_allocations as f64 / N as f64,
        parse_allocations as f64 / N as f64,
        compile_allocations as f64 / N as f64
    );
    assert!(
        parse_allocations <= 32 * N + 512,
        "parse allocations: {parse_allocations}; intact effect-bearing tracks require the borrowed TOML tree plus owned IDs/effect/parameter vectors"
    );
    assert!(
        canonical_allocations <= 96,
        "canonical allocations: {canonical_allocations}"
    );
    assert!(
        compile_allocations <= 10 * N + 512,
        "compile allocations: {compile_allocations}"
    );
    assert_eq!(
        estimate_allocations, 0,
        "estimate must remain allocation-free"
    );
}
