//! Allocation evidence for registry-memoized named nudge lookup.

#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use miso_engine_effect_compiler::launch_native_effect_registry_v1;

struct TrackingAllocator;

static TRACKING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

// SAFETY: every operation forwards the original pointer/layout unchanged to System; the atomics
// only observe successful allocations during this isolated integration-test binary.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the allocator-provided layout is forwarded unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() && TRACKING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the original pointer and layout are forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[test]
fn memoized_named_nudge_lookup_allocates_nothing_per_call() {
    let registry = launch_native_effect_registry_v1().expect("native registry");
    let descriptor = registry
        .get_ascii("miso.compressor")
        .expect("compressor")
        .descriptor();
    let parameter = descriptor.parameters[0];
    let expected = registry
        .nudge_ladder(descriptor.id, parameter.id)
        .expect("memoized ladder");

    ALLOCATIONS.store(0, Ordering::Relaxed);
    TRACKING.store(true, Ordering::Release);
    for _ in 0..10_000 {
        assert_eq!(
            registry.nudge_ladder(descriptor.id, parameter.id),
            Some(expected)
        );
    }
    TRACKING.store(false, Ordering::Release);
    assert_eq!(ALLOCATIONS.load(Ordering::Acquire), 0);
}
