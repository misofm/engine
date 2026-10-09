//! The render-locked allocation counter (issue #1333 D1, decision 15 D15-10).
//!
//! Decision 15 rules that the AudioWorklet instance never allocates or frees after boot. The
//! static call-graph gate cannot prove that for the render export, because the whole plan executor
//! sits behind `Box<dyn PreparedPlanExecutor>` and is reached by a `call_indirect`. This module is
//! the runtime proof: every export the engine worklet or the SDK's PCM-feed worklet calls on its
//! render thread after boot runs inside [`render_locked`] (the set and its two named exceptions,
//! `dispose` and `render_allocation_count`, are listed in the `ffi` module header), and the
//! module's global allocator counts each allocator call made while the flag is set. Browser
//! qualification reads the count through `miso_engine_web_v1_render_allocation_count` and asserts
//! it is exactly zero.
//!
//! "Boot" here and in the `ffi` header is the engine worklet's whole construction path through its
//! `initialize` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`), up to the `miso.ready.v1`
//! post, and not only the `boot` export: `initialize` calls exports before the `boot` export and
//! after it returns. "After boot" is after that post. Boot is outside the render-locked rule:
//!
//! - Before the `boot` export (or `boot_with_spectrum_hop`), it calls, as its options require,
//!   `abi_version`, `spectrum_hop_capability`, `boot_options_ptr`, the spectrum request and
//!   collection accessors (`spectrum_request_*`, `spectrum_collection_*`) and `document_ptr`.
//!   None of them is wrapped, and neither is the `boot` export: there the collection staging is
//!   sized and allocated by design.
//! - After the `boot` export returns a handle, the unwrapped exports it can call are these 28:
//!   `buffer_ptr`, `buffer_capacity`, `status_ptr`, `resource_ptr`, `command_report_ptr`,
//!   `prepared_companion_ptr`, `prepared_companion_capacity`, `live_control_track_count`,
//!   `live_control_track_id`, `live_control_submix_count`, `live_control_submix_id`,
//!   `live_control_route_count`, `live_control_route_id`, `live_control_vca_count`,
//!   `live_control_vca_id`, `source_count`, `source_id`, `source_channels`, `source_frames`,
//!   `observation_count`, `observation_track_index`, `observation_rack`,
//!   `observation_effect_index`, `observation_effect_slot_id`, `observation_native_effect_id`,
//!   `observation_tap_count`, `observation_tap_id` and `meter_header_ptr`.
//! - When `document_ptr` or the `boot` export returns 0, it calls `boot_result`, which is not
//!   wrapped either.
//! - It can also call render-locked exports, and those calls are counted: `spectrum_target_id_ptr`
//!   and `spectrum_target_id_capacity` before the `boot` export, and after it eleven staging
//!   accessors: `observation_id_ptr`, `observation_id_capacity`, `observation_selection_ptr`,
//!   `observation_selection_bytes`, `observation_selection_capacity`,
//!   `track_response_request_ptr`, `track_response_request_bytes`, `track_response_track_id_ptr`,
//!   `track_response_track_id_capacity`, `track_response_snapshot_ptr` and
//!   `track_response_snapshot_capacity`.
//!
//! The flag is a const-initialised `Cell<bool>` thread local. It has no destructor, so it
//! registers nothing with std: it is a plain static on today's non-atomic module and an
//! instance-local slot once the module is built with atomics. The counter is a process-wide
//! `AtomicU32`; one instance owns one copy of it.
//!
//! The cost when unlocked is one thread-local load and one branch per allocator call. There is no
//! nesting and no RAII guard: the release profile aborts on panic, so a panicking body never
//! returns to a scope that would need to clear the flag.

#![allow(unsafe_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};

thread_local! {
    static RENDER_LOCKED: Cell<bool> = const { Cell::new(false) };
}

static RENDER_ALLOCATIONS: AtomicU32 = AtomicU32::new(0);

/// A global allocator that counts every call made while the current thread is render-locked, then
/// forwards it unchanged to the wrapped allocator.
///
/// `alloc`, `alloc_zeroed`, `realloc` and `dealloc` each count one call, whatever their outcome:
/// the claim is "render calls no allocator", not "render obtains no memory".
pub struct RenderLockedAllocator<A: GlobalAlloc>(pub A);

/// Whether the current thread is inside a [`render_locked`] window.
///
/// `try_with` keeps the panic path out of the render closure; a const, destructor-free thread
/// local is never destroyed, so it never fails.
#[inline]
fn locked() -> bool {
    RENDER_LOCKED.try_with(Cell::get).unwrap_or(false)
}

#[inline]
fn count_if_locked() {
    if locked() {
        RENDER_ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }
}

// SAFETY: every operation is forwarded to the wrapped allocator with its arguments unchanged; the
// counter observes calls and never alters a pointer, layout, size or lifetime.
unsafe impl<A: GlobalAlloc> GlobalAlloc for RenderLockedAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_if_locked();
        // SAFETY: the caller's contract for `GlobalAlloc::alloc` is forwarded unchanged.
        unsafe { self.0.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_if_locked();
        // SAFETY: the caller's contract for `GlobalAlloc::alloc_zeroed` is forwarded unchanged.
        unsafe { self.0.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count_if_locked();
        // SAFETY: the caller's contract for `GlobalAlloc::realloc` is forwarded unchanged.
        unsafe { self.0.realloc(pointer, layout, new_size) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        count_if_locked();
        // SAFETY: the caller's contract for `GlobalAlloc::dealloc` is forwarded unchanged.
        unsafe { self.0.dealloc(pointer, layout) }
    }
}

/// The browser module's allocator. The wasm cfg is not a target fork: the native rlib links into
/// test binaries and tools that register their own `#[global_allocator]`, and a second one would
/// not link. An integration test binary registers [`RenderLockedAllocator`] itself.
#[cfg(all(target_family = "wasm", not(test)))]
#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<std::alloc::System> =
    RenderLockedAllocator(std::alloc::System);

/// Run `operation` with the current thread render-locked, then clear the lock.
///
/// Every allocator call `operation` makes is counted. Windows do not nest.
#[inline]
pub(crate) fn render_locked<R>(operation: impl FnOnce() -> R) -> R {
    debug_assert!(!locked(), "render-locked windows do not nest");
    let _ = RENDER_LOCKED.try_with(|flag| flag.set(true));
    let result = operation();
    let _ = RENDER_LOCKED.try_with(|flag| flag.set(false));
    result
}

/// The number of allocator calls made inside render-locked windows since this instance started.
pub(crate) fn render_allocation_count() -> u32 {
    RENDER_ALLOCATIONS.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::System;

    /// The wrapper under test is called directly, never registered, so the test binary's own
    /// counting allocator and the libtest harness never touch it.
    static WRAPPER: RenderLockedAllocator<System> = RenderLockedAllocator(System);

    /// Every allocator entry point, called once each through the wrapper, in one window or none.
    /// Returns how far the shared counter moved.
    fn exercise(lock: bool) -> u32 {
        let layout = Layout::from_size_align(64, 16).expect("valid layout");
        let run = || {
            let before = render_allocation_count();
            // SAFETY: each pointer comes from the wrapper with `layout` and goes back to it with
            // the matching layout or size; nothing reads uninitialised memory.
            unsafe {
                let first = WRAPPER.alloc(layout);
                assert!(!first.is_null());
                let zeroed = WRAPPER.alloc_zeroed(layout);
                assert!(!zeroed.is_null());
                assert_eq!(*zeroed, 0);
                let grown = WRAPPER.realloc(first, layout, 256);
                assert!(!grown.is_null());
                WRAPPER.dealloc(
                    grown,
                    Layout::from_size_align(256, 16).expect("valid layout"),
                );
                WRAPPER.dealloc(zeroed, layout);
            }
            render_allocation_count().wrapping_sub(before)
        };
        if lock { render_locked(run) } else { run() }
    }

    // The counter is process-wide and libtest runs tests on parallel threads, so every case is in
    // one test: a second test's window would move the same counter. Only the wrapper above moves
    // it; the binary's registered allocator is a different one.
    #[test]
    fn render_lock_counts_each_entry_point_only_inside_its_own_window() {
        assert!(!locked(), "a fresh thread starts unlocked");
        assert_eq!(exercise(false), 0, "an unlocked thread counts nothing");
        assert_eq!(
            exercise(true),
            5,
            "alloc, alloc_zeroed, realloc and two deallocs each count once in the window"
        );
        assert!(!locked(), "the window clears the flag when it returns");
        assert_eq!(exercise(false), 0, "nothing counts after the window");
        assert_eq!(
            render_locked(|| 7_u8),
            7,
            "the window returns its body's value"
        );
        // Another thread's window never counts this thread's calls: the flag is a thread local,
        // which is what makes it instance-local once the module has atomics.
        let moved = render_locked(|| {
            std::thread::scope(|scope| {
                scope
                    .spawn(|| {
                        assert!(!locked(), "a window on one thread does not lock another");
                        exercise(false)
                    })
                    .join()
                    .expect("thread joins")
            })
        });
        assert_eq!(moved, 0, "a window on another thread counts nothing here");
    }
}
