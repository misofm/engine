//! Render allocation owners: a banked held meter (#951), and issue #1765 gate 2, where
//! a render-time swap of two strips' builtin lanes allocates and frees
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

/// The held seed/pass/accept branch has one full-render allocation owner, using the same
/// isolated audited allocator as the lane-swap owner. Arithmetic and publication bits belong to
/// the meter differential and compiler unit parity test.
#[test]
fn a_banked_held_meter_render_allocates_nothing() {
    use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
    assert_installed();
    let mut bound = builtins_compiler::test_only_prepared_unpaired_graph_with_held_meter();
    let mut output = [0.0; 128];
    let render =
        |bound: &mut builtins_compiler::PreparedBuiltinsGraphBound, output: &mut [f32], at| {
            bound.plan.render(
                RenderIo {
                    output: PlanarBufferMut::try_new(output, 2, 64, 64).unwrap(),
                },
                RenderTime {
                    absolute_sample: at,
                },
            )
        };
    // Warm process statics and the real positive-hold/decay path before measuring.
    render(&mut bound, &mut output, 0).unwrap();
    bound.meter_consumers[0].consumer.try_pop().unwrap();
    for at in [64, 128, 192] {
        builtins::test_only_reset_banked_meter_commits();
        let mark = current_thread_counters();
        let result = render(&mut bound, &mut output, at);
        let delta = current_thread_delta_since(mark);
        result.unwrap();
        assert_eq!(
            (delta.allocations, delta.deallocations),
            (0, 0),
            "positive hold/decay render allocated or freed"
        );
        assert_eq!(
            builtins::test_only_banked_meter_commits(),
            1,
            "the held bank path must run"
        );
        // Snapshot queue access and checks are outside the render measurement.
        bound.meter_consumers[0].consumer.try_pop().unwrap();
    }
}
