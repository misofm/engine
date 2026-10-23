//! Issue #1765 gate 2: a render-time swap of two strips' builtin lanes allocates and frees
//! nothing. Its own test binary, so that `bench_support`'s audited allocator serves it (AGENTS.md:
//! an exact allocation count is measured with its thread-scoped counters). The scenario and its
//! bit checks are [`builtins_compiler::test_only_builtin_lane_swap`]; the bit claim is gated in
//! `tests/allocation_tracker.rs`.

#![cfg(feature = "test-support")]

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};

/// Every swap of the scenario -- in one chain and across two, stereo and collapsed mono, at every
/// width this build has, after the scenario's own warm-up blocks -- makes exactly zero allocations
/// and zero frees on the render thread.
///
/// Red if a swap of the input, fader or matrix bank, or the chain's own swap, allocates or frees.
#[test]
fn a_builtin_lane_swap_allocates_nothing() {
    assert_installed();
    let mut measured = 0_usize;
    let swaps = builtins_compiler::test_only_builtin_lane_swap(&mut |swap| {
        let mark = current_thread_counters();
        swap();
        let delta = current_thread_delta_since(mark);
        assert_eq!(
            (delta.allocations, delta.deallocations),
            (0, 0),
            "a builtin lane swap allocated or freed"
        );
        measured += 1;
    });
    assert!(swaps >= 8, "two passes of four swaps at one width at least");
    assert_eq!(measured, swaps, "every swap ran under the measure");
}
