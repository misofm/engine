//! E6: allocation-call budgets for the complete 4,096-track control pipeline.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use core::sync::atomic::{AtomicUsize, Ordering};
use jstrict::{Parse, Value};
use std::alloc::{GlobalAlloc, System};

use session::{
    CompileCaps, RouteSource, SendTap, StableId, canonical_session_json, compile_session,
    estimate_session_resources, parse_session_json,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");
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

fn model_at(n: usize) -> session::SessionModel {
    let mut model = parse_session_json(CANONICAL).expect("fixture parses");
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.automation.clear();
    model.tracks.reserve(n);
    model.routes.reserve(n);
    for index in 0..n {
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
    // Measurements on the pinned jstrict 0.14.0 frontend at 1/256/4096 tracks fit below
    // `263 * tracks + 192`; this is the smallest integer-slope conservative envelope covering
    // all three observations with at least 32 calls of fixed headroom. It replaces the TOML-era
    // projection and keeps the component counts visible so a parser/model regression is local.
    for n in [1, 256, 4_096] {
        let model = model_at(n);
        let text = canonical_session_json(&model).expect("model canonicalizes");
        let (raw, raw_parse_allocations) = measured(|| Value::parse_str(&text));
        assert!(raw.is_ok());
        let (parsed, parse_allocations) = measured(|| parse_session_json(&text));
        assert!(parsed.is_ok());
        let (canonical, canonical_allocations) = measured(|| canonical_session_json(&model));
        assert!(canonical.is_ok());
        let (compiled, compile_allocations) = measured(|| compile_session(&model, caps()));
        assert!(compiled.is_ok());
        let (estimate, estimate_allocations) = measured(|| estimate_session_resources(&model));
        assert!(estimate.is_ok());
        let preflight_and_owned_model_allocations = parse_allocations - raw_parse_allocations;
        println!(
            "tracks={n}: raw_json={raw_parse_allocations}, preflight_and_owned_model={preflight_and_owned_model_allocations}, parse={parse_allocations}, canonical={canonical_allocations}, compile={compile_allocations}, estimate={estimate_allocations}"
        );
        assert!(
            parse_allocations <= 263 * n + 192,
            "parse allocation envelope: tracks={n}, calls={parse_allocations}"
        );
        assert!(
            canonical_allocations <= 96,
            "canonical allocations: {canonical_allocations}"
        );
        assert!(
            compile_allocations <= 10 * n + 512,
            "compile allocations: {compile_allocations}"
        );
        assert_eq!(
            estimate_allocations, 0,
            "estimate must remain allocation-free"
        );
    }
}
