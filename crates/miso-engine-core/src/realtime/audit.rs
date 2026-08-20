//! Test instrumentation for proving the realtime call graph stays free of forbidden operations.
//!
//! Production builds retain only a thread-local depth check and fixed counters. The allocator
//! wrapper lives in the issue-003 audit executable; the engine never replaces an embedding host's
//! allocator.

use core::cell::Cell;

/// A forbidden operation category observed while a render scope was armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForbiddenOperation {
    /// Heap allocation or reallocation.
    Allocation,
    /// Heap deallocation.
    Deallocation,
    /// Lock or blocking synchronization.
    Lock,
    /// Logging, tracing, printing, or formatting for output.
    Log,
    /// File input/output.
    FileIo,
    /// Network input/output.
    NetworkIo,
    /// Any other direct operating-system call.
    Syscall,
}

/// Fixed audit counters for the current thread.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AuditSnapshot {
    /// Allocation attempts inside render.
    pub allocations: u64,
    /// Deallocation attempts inside render.
    pub deallocations: u64,
    /// Lock attempts inside render.
    pub locks: u64,
    /// Log attempts inside render.
    pub logs: u64,
    /// File-I/O attempts inside render.
    pub file_io: u64,
    /// Network-I/O attempts inside render.
    pub network_io: u64,
    /// Other explicit syscall attempts inside render.
    pub syscalls: u64,
}

impl AuditSnapshot {
    /// Total forbidden operations observed.
    #[must_use]
    pub const fn total(self) -> u64 {
        self.allocations
            .saturating_add(self.deallocations)
            .saturating_add(self.locks)
            .saturating_add(self.logs)
            .saturating_add(self.file_io)
            .saturating_add(self.network_io)
            .saturating_add(self.syscalls)
    }

    fn increment(&mut self, operation: ForbiddenOperation) {
        let counter = match operation {
            ForbiddenOperation::Allocation => &mut self.allocations,
            ForbiddenOperation::Deallocation => &mut self.deallocations,
            ForbiddenOperation::Lock => &mut self.locks,
            ForbiddenOperation::Log => &mut self.logs,
            ForbiddenOperation::FileIo => &mut self.file_io,
            ForbiddenOperation::NetworkIo => &mut self.network_io,
            ForbiddenOperation::Syscall => &mut self.syscalls,
        };
        *counter = counter.saturating_add(1);
    }
}

#[derive(Clone, Copy, Default)]
struct AuditState {
    depth: u32,
    counters: AuditSnapshot,
}

std::thread_local! {
    static STATE: Cell<AuditState> = const { Cell::new(AuditState {
        depth: 0,
        counters: AuditSnapshot {
            allocations: 0,
            deallocations: 0,
            locks: 0,
            logs: 0,
            file_io: 0,
            network_io: 0,
            syscalls: 0,
        },
    }) };
}

/// Initialize this thread's audit state before entering a host callback.
pub fn warm_up() {
    STATE.with(|state| {
        let _ = state.get();
    });
}

/// Reset counters while no render scope is active.
pub fn reset() {
    STATE.with(|state| {
        assert_eq!(state.get().depth, 0, "cannot reset an armed render audit");
        state.set(AuditState::default());
    });
}

/// Return the current thread's audit counters.
#[must_use]
pub fn snapshot() -> AuditSnapshot {
    STATE.with(|state| state.get().counters)
}

/// Whether the current thread is executing inside an audited render scope.
#[must_use]
pub fn is_render_scope_active() -> bool {
    STATE.with(|state| state.get().depth != 0)
}

/// Record a forbidden operation and panic if render is armed.
///
/// The allocator wrapper must use [`record_allocator_violation`] instead: unwinding from a
/// `GlobalAlloc` method is not permitted by Rust.
pub fn forbidden(operation: ForbiddenOperation) {
    if record(operation) {
        panic!("forbidden operation in realtime render scope: {operation:?}");
    }
}

/// Record an allocator/deallocator violation without unwinding through `GlobalAlloc`.
///
/// Returns `true` when the audit executable must terminate immediately.
#[must_use]
pub fn record_allocator_violation(operation: ForbiddenOperation) -> bool {
    debug_assert!(matches!(
        operation,
        ForbiddenOperation::Allocation | ForbiddenOperation::Deallocation
    ));
    record(operation)
}

fn record(operation: ForbiddenOperation) -> bool {
    STATE.with(|state| {
        let mut current = state.get();
        if current.depth == 0 {
            return false;
        }
        current.counters.increment(operation);
        state.set(current);
        true
    })
}

/// Execute one operation under the audit guard.
///
/// Engine users call `PreparedRenderPlan::render` or `RealtimePlanOwner::render`; this function is
/// public solely so the standalone audit executable can run deliberate mutation probes.
#[cfg(feature = "realtime-audit")]
#[doc(hidden)]
pub fn in_render_scope<T>(render: impl FnOnce() -> T) -> T {
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            STATE.with(|state| {
                let mut current = state.get();
                current.depth = current.depth.saturating_sub(1);
                state.set(current);
            });
        }
    }

    STATE.with(|state| {
        let mut current = state.get();
        current.depth = current
            .depth
            .checked_add(1)
            .expect("render audit nesting overflow");
        state.set(current);
    });
    let guard = Guard;
    let result = render();
    drop(guard);
    result
}

/// Execute the render closure directly when instrumentation is not selected.
#[cfg(not(feature = "realtime-audit"))]
#[doc(hidden)]
#[inline(always)]
pub fn in_render_scope<T>(render: impl FnOnce() -> T) -> T {
    render()
}

#[cfg(all(test, feature = "realtime-audit"))]
mod tests {
    use super::{ForbiddenOperation, forbidden, in_render_scope, reset, snapshot};

    #[test]
    fn forbidden_hooks_are_armed_only_inside_render() {
        reset();
        forbidden(ForbiddenOperation::Log);
        for operation in [
            ForbiddenOperation::Allocation,
            ForbiddenOperation::Deallocation,
            ForbiddenOperation::Lock,
            ForbiddenOperation::Log,
            ForbiddenOperation::FileIo,
            ForbiddenOperation::NetworkIo,
            ForbiddenOperation::Syscall,
        ] {
            let result = std::panic::catch_unwind(|| {
                in_render_scope(|| forbidden(operation));
            });
            assert!(result.is_err(), "{operation:?} hook was not armed");
        }
        assert_eq!(snapshot().allocations, 1);
        assert_eq!(snapshot().deallocations, 1);
        assert_eq!(snapshot().locks, 1);
        assert_eq!(snapshot().logs, 1);
    }
}
