//! The one audited global allocator for every tool under `tools/`.
//!
//! Three behaviours existed before #104 and all three are preserved:
//!
//! * **Abort on an armed violation** (the default; twelve historical tools). An allocation while a
//!   render scope is armed is recorded by [`engine::realtime::audit`] and the process
//!   terminates. It terminates rather than panicking because unwinding out of a `GlobalAlloc`
//!   method is not permitted by Rust.
//! * **Count and continue** ([`Mode::Count`]; the historical `ABORT_ALLOCATOR_VIOLATION = false`
//!   switch in the builtins audit's probe path, and the protocol audit/bench thread-local counter).
//!   The violation is still recorded in the core counters; only the abort is suppressed, so a probe
//!   can observe its own deliberate violation and report it.
//! * **Process-wide totals** ([`counters`]; the historical effect-contract bench statics). Always
//!   on, `Relaxed`, and never read inside a timed interval.
//!
//! The mode is process-global and only a probe may set it. A benchmark that sets [`Mode::Count`]
//! has disabled its own allocation gate.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use engine::realtime::audit::{ForbiddenOperation, record_allocator_violation};
use std::alloc::{GlobalAlloc, Layout, System};

/// What happens when an allocation is attempted while a render scope is armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Record the violation and terminate without unwinding.
    Abort,
    /// Record the violation and continue, so a deliberate probe can report it.
    Count,
}

static MODE: AtomicU8 = AtomicU8::new(0);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static REALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static REQUESTED_BYTES: AtomicU64 = AtomicU64::new(0);

/// Set the process-wide violation mode. Only a deliberate probe may call this.
pub fn set_mode(mode: Mode) {
    MODE.store(mode as u8 + 1, Ordering::Relaxed);
}

/// The current process-wide violation mode.
#[must_use]
pub fn mode() -> Mode {
    match MODE.load(Ordering::Relaxed) {
        2 => Mode::Count,
        _ => Mode::Abort,
    }
}

/// Process-wide allocator totals since start.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Counters {
    /// `alloc` and `alloc_zeroed` calls.
    pub allocations: u64,
    /// `dealloc` calls.
    pub deallocations: u64,
    /// `realloc` calls.
    pub reallocations: u64,
    /// Bytes requested by `alloc`, `alloc_zeroed` and `realloc`.
    pub requested_bytes: u64,
}

/// Read the process-wide totals.
#[must_use]
pub fn counters() -> Counters {
    Counters {
        allocations: ALLOCATIONS.load(Ordering::Relaxed),
        deallocations: DEALLOCATIONS.load(Ordering::Relaxed),
        reallocations: REALLOCATIONS.load(Ordering::Relaxed),
        requested_bytes: REQUESTED_BYTES.load(Ordering::Relaxed),
    }
}

/// Totals accumulated since `mark`, field-wise and saturating.
#[must_use]
pub fn delta_since(mark: Counters) -> Counters {
    let now = counters();
    Counters {
        allocations: now.allocations.saturating_sub(mark.allocations),
        deallocations: now.deallocations.saturating_sub(mark.deallocations),
        reallocations: now.reallocations.saturating_sub(mark.reallocations),
        requested_bytes: now.requested_bytes.saturating_sub(mark.requested_bytes),
    }
}

/// The audited allocator. Installed as this process's `#[global_allocator]`.
pub struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

fn violated(operation: ForbiddenOperation) {
    if record_allocator_violation(operation) && mode() == Mode::Abort {
        std::process::abort();
    }
}

// SAFETY: every method forwards the caller's unchanged pointer/layout contract to `System`, which
// satisfies `GlobalAlloc`. The audit branch either terminates the process or returns; it never
// unwinds through a `GlobalAlloc` method, never retains a pointer and never alters a layout.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        REQUESTED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        violated(ForbiddenOperation::Allocation);
        // SAFETY: the caller's valid layout is forwarded unchanged to the system allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        REQUESTED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        violated(ForbiddenOperation::Allocation);
        // SAFETY: the caller's valid layout is forwarded unchanged to the system allocator.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        violated(ForbiddenOperation::Deallocation);
        // SAFETY: the pointer/layout pair came from this allocator and is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        REALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        REQUESTED_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        violated(ForbiddenOperation::Allocation);
        // SAFETY: the allocation came from this allocator; the original layout and the requested
        // new size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

/// Panic unless the audited allocator is the one serving this process.
///
/// A binary that depends on this crate but never names a symbol from it may not link it at all,
/// and a `#[global_allocator]` in an unlinked rlib installs nothing: the audit would be silently
/// off while every gate still reported success. Every tool calls this once, before arming
/// anything, so that failure is loud and immediate instead.
///
/// # Panics
///
/// Panics if a heap allocation does not move [`counters`].
pub fn assert_installed() {
    let mark = counters();
    let probe = Vec::<u8>::with_capacity(64);
    let moved = delta_since(mark);
    drop(probe);
    assert!(
        moved.allocations >= 1,
        "the audited global allocator is not installed in this process"
    );
}

#[cfg(test)]
mod tests {
    use super::{Counters, Mode, assert_installed, counters, delta_since, mode};

    #[test]
    fn default_mode_is_abort() {
        assert_eq!(mode(), Mode::Abort);
    }

    #[test]
    fn the_installed_allocator_is_the_audited_one() {
        let mark = counters();
        let held = Vec::<u8>::with_capacity(4096);
        let moved = delta_since(mark);
        assert!(moved.allocations >= 1, "no allocation was counted");
        assert!(moved.requested_bytes >= 4096, "no bytes were counted");
        drop(held);
    }

    #[test]
    fn the_installed_allocator_assertion_holds_here() {
        assert_installed();
    }

    #[test]
    fn delta_saturates_below_its_mark() {
        let future = Counters {
            allocations: u64::MAX,
            deallocations: u64::MAX,
            reallocations: u64::MAX,
            requested_bytes: u64::MAX,
        };
        assert_eq!(delta_since(future), Counters::default());
    }
}
