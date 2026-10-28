//! The render-locked allocation counter (issue #1333 D1, decision 15 D15-10).
//!
//! Decision 15 rules that the AudioWorklet instance never allocates or frees after boot. The
//! static call-graph gate cannot prove that for the render export, because the whole plan executor
//! sits behind `Box<dyn PreparedPlanExecutor>` and is reached by a `call_indirect`. This module is
//! the runtime proof: every export the engine worklet or the SDK's PCM-feed worklet calls on its
//! render thread after boot runs inside [`render_locked`] (the set is the table
//! `tools/realtime-policy/render-thread-exports.json`; its named exceptions are the entries with
//! `wrapped: false` that the worklets call after boot: `dispose`, `render_allocation_count`,
//! `service`, `control_allocation_count`, the control-frame exports, `control_event` and `watermark_read`), and the module's global allocator counts each allocator call
//! made while the flag is set. Browser qualification reads the count through
//! `miso_engine_web_v1_render_allocation_count` and asserts it is exactly zero.
//!
//! "Boot" here and in the `ffi` header is the engine worklet's whole construction path through its
//! `initialize` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`), up to the `miso.ready.v1`
//! post, and not only the `boot` export: `initialize` calls exports before the `boot` export and
//! after it returns. "After boot" is after that post. Boot is outside the render-locked rule:
//!
//! - Before the `boot` export, it calls `abi_version`, `boot_options_ptr` and `document_ptr`.
//!   None of them is wrapped, and neither is the `boot` export, which allocates by design.
//! - After the `boot` export returns a handle, the unwrapped exports it can call are these 17:
//!   `buffer_ptr`, `buffer_capacity`, `status_ptr`, `resource_ptr`, `source_count`, `source_id`,
//!   `source_channels`, `source_frames`, `observation_count`, `observation_track_index`,
//!   `observation_rack`, `observation_effect_index`, `observation_effect_slot_id`,
//!   `observation_native_effect_id`, `observation_tap_count`, `observation_tap_id` and
//!   `meter_header_ptr`.
//! - When `document_ptr` or the `boot` export returns 0, it calls `boot_result`, which is not
//!   wrapped either.
//! - It can also call render-locked exports, and those calls are counted: after the `boot` export
//!   five staging accessors: `observation_id_ptr`, `observation_id_capacity`,
//!   `observation_selection_ptr`, `observation_selection_bytes` and
//!   `observation_selection_capacity`.
//!
//! # The control counter (issue #1381a, #1332 D1)
//!
//! Every allocator call made while the flag is *clear* counts in a second counter,
//! `CONTROL_ALLOCATIONS`: boot, the control messages and the service step. In `single` mode, the
//! browser's page without cross-origin isolation, that is the one place this engine allocates on
//! an audio thread (decision 15 D15-10): the instance's control work runs in the worklet's message
//! handler, outside `process()`. The host reports the count as `singleModeControlAllocations`
//! through `miso_engine_web_v1_control_allocation_count`, so the allocation is counted and reported
//! and not hidden. The render-locked count stays exactly zero.
//!
//! The flag is a const-initialised `Cell<bool>` thread local. It has no destructor, so it
//! registers nothing with std: on the atomic module it is an instance-local slot. Since #1332 S6
//! the render-locked counter is one too, so on one shared memory each instance (the Worker, the
//! worklet) reports its own count. The control counter is a process-wide `AtomicU64`: on one
//! shared memory both instances count into one copy (#1331 recipe rule 3), so a page in `worker`
//! mode reads the two instances' control allocations together.
//!
//! The cost when unlocked is one thread-local load and one branch per allocator call. There is no
//! nesting and no RAII guard: the release profile aborts on panic, so a panicking body never
//! returns to a scope that would need to clear the flag.

#![allow(unsafe_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::sync::atomic::{AtomicU64, Ordering};

thread_local! {
    static RENDER_LOCKED: Cell<bool> = const { Cell::new(false) };
    /// The allocator calls made inside render-locked windows (#1332 S6): per instance, a const,
    /// destructor-free thread local like the flag, so on one shared memory the Worker's count and
    /// the worklet's are their own (#1331 recipe rule 3). Saturating.
    static RENDER_ALLOCATIONS: Cell<u32> = const { Cell::new(0) };
}

/// The allocator calls made while no render-locked window was open (#1381a). Wide, so that a long
/// session cannot wrap it to a small number; the export that reports it is 32 bits and saturates
/// (batch 7 review).
static CONTROL_ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

/// A global allocator that counts every call made while the current thread is render-locked, and
/// every other call in the control counter, then forwards it unchanged to the wrapped allocator.
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
fn count_call() {
    if locked() {
        let _ = RENDER_ALLOCATIONS.try_with(|count| count.set(count.get().saturating_add(1)));
    } else {
        CONTROL_ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }
}

// SAFETY: every operation is forwarded to the wrapped allocator with its arguments unchanged; the
// counter observes calls and never alters a pointer, layout, size or lifetime.
unsafe impl<A: GlobalAlloc> GlobalAlloc for RenderLockedAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_call();
        // SAFETY: the caller's contract for `GlobalAlloc::alloc` is forwarded unchanged.
        unsafe { self.0.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_call();
        // SAFETY: the caller's contract for `GlobalAlloc::alloc_zeroed` is forwarded unchanged.
        unsafe { self.0.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count_call();
        // SAFETY: the caller's contract for `GlobalAlloc::realloc` is forwarded unchanged.
        unsafe { self.0.realloc(pointer, layout, new_size) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        count_call();
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

/// The number of allocator calls made inside render-locked windows since this instance started:
/// this instance's own (on a native host, this thread's), never another's on the same memory.
pub(crate) fn render_allocation_count() -> u32 {
    RENDER_ALLOCATIONS.try_with(Cell::get).unwrap_or(0)
}

/// Issue #1529: the positive control of the render-allocation gates, never in a shipped build.
#[cfg(feature = "render-lock-probe")]
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_web_v1_render_lock_probe(inside: u32) -> u32 {
    let allocate = || drop(std::hint::black_box(Box::new(inside)));
    if inside != 0 {
        render_locked(allocate);
    } else {
        allocate();
    }
    render_allocation_count()
}

/// The number of allocator calls made outside render-locked windows since this instance started
/// (#1381a): its boot, control messages and service steps. It saturates at `u32::MAX` and never
/// wraps.
pub(crate) fn control_allocation_count() -> u32 {
    u32::try_from(CONTROL_ALLOCATIONS.load(Ordering::Relaxed)).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::System;

    /// The wrapper under test is called directly, never registered, so the test binary's own
    /// counting allocator and the libtest harness never touch it.
    static WRAPPER: RenderLockedAllocator<System> = RenderLockedAllocator(System);

    /// Every allocator entry point, called once each through the wrapper, in one window or none.
    /// Returns how far the render counter and the control counter moved.
    fn exercise(lock: bool) -> (u32, u32) {
        let layout = Layout::from_size_align(64, 16).expect("valid layout");
        let run = || {
            let before = (render_allocation_count(), control_allocation_count());
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
            (
                render_allocation_count().wrapping_sub(before.0),
                control_allocation_count().wrapping_sub(before.1),
            )
        };
        if lock { render_locked(run) } else { run() }
    }

    // The counter is process-wide and libtest runs tests on parallel threads, so every case is in
    // one test: a second test's window would move the same counter. Only the wrapper above moves
    // it; the binary's registered allocator is a different one.
    #[test]
    fn render_lock_counts_each_entry_point_only_inside_its_own_window() {
        assert!(!locked(), "a fresh thread starts unlocked");
        assert_eq!(
            exercise(false),
            (0, 5),
            "an unlocked thread counts in the control counter only"
        );
        assert_eq!(
            exercise(true),
            (5, 0),
            "alloc, alloc_zeroed, realloc and two deallocs each count once in the window, and \
             the window counts nothing in the control counter"
        );
        assert!(!locked(), "the window clears the flag when it returns");
        assert_eq!(
            exercise(false),
            (0, 5),
            "nothing counts in the render counter after the window"
        );
        assert_eq!(
            render_locked(|| 7_u8),
            7,
            "the window returns its body's value"
        );
        // Another thread's window never counts this thread's calls: the flag and the render
        // counter are thread locals, which is what makes them instance-local on the atomic module.
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
        assert_eq!(
            moved.0, 0,
            "a window on another thread counts nothing in the render counter"
        );
        // #1332 S6: the render counter is per instance (per thread natively): another thread's
        // render-locked calls never move this thread's count.
        let mine = render_allocation_count();
        let theirs = std::thread::scope(|scope| {
            scope.spawn(|| exercise(true)).join().expect("thread joins")
        });
        assert_eq!(theirs.0, 5, "the other thread counts its own window");
        assert_eq!(
            render_allocation_count(),
            mine,
            "another thread's window moved this thread's render count"
        );
        // The control counter is wide and its reader saturates: past `u32::MAX` calls it reads
        // `u32::MAX` and never wraps to a small number.
        CONTROL_ALLOCATIONS.store(u64::from(u32::MAX) - 1, Ordering::Relaxed);
        exercise(false);
        assert_eq!(control_allocation_count(), u32::MAX);
        CONTROL_ALLOCATIONS.store(0, Ordering::Relaxed);
    }
}
