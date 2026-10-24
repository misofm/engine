//! Issue #1529: the positive control of the render-allocation gates, native leg.
//!
//! Every other render-allocation gate asserts the count stays zero, so it also passes when nothing
//! is counted. This binary compiles the library without `cfg(test)`, registers the module's own
//! counting allocator, and proves that one allocation and one free inside a real render-locked
//! window move the count by exactly 2, and the same pair outside a window moves it by 0. It holds
//! exactly one test, so nothing else shares the process-wide counter.

use std::alloc::System;

use host_web::{RenderLockedAllocator, miso_engine_web_v1_render_lock_probe as probe};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

#[test]
fn an_allocation_inside_a_render_locked_window_is_counted() {
    let start = probe(0);
    assert_eq!(
        [probe(0), probe(1), probe(0)],
        [start, start + 2, start + 2],
        "outside a window the count must not move; inside, one alloc and one free move it by 2",
    );
}
