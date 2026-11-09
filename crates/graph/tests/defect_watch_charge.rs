//! Issue #2427: a plan's lane-swap defect watch charges exactly the heap it holds.
//!
//! The watch's `heap_bytes` is `graph::lane_swap_defect_watch_bytes`, the one formula both the
//! preparation's projection and the plan's inventory row (and so `capi_retained_bytes`) read, so a
//! projection test cannot see it drift: both sides move together. This holds the formula to the
//! audited allocator instead.
#![cfg(feature = "test-support")]

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};

/// A watch with no strip (a plan with no pack group's, which still makes its table for the plan's
/// totals) and one over two strips: each charges the bytes the allocator saw it make, plus the
/// names and their table it was given.
///
/// Test value: a charge that leaves out the table of a watch with no strip (every plan without a
/// pack group then under-charges its inventory and `capi_retained_bytes`), or a strip's flag or
/// name.
#[test]
fn a_defect_watch_charges_exactly_the_heap_it_holds() {
    assert_installed();
    for names in [Vec::new(), vec!["eq0", "eq12"]] {
        let strips: Box<[Box<str>]> = names.iter().map(|name| Box::from(*name)).collect();
        // The names and their table exist before the watch takes them.
        let given = core::mem::size_of_val::<[Box<str>]>(&strips)
            + names.iter().map(|name| name.len()).sum::<usize>();
        let mark = current_thread_counters();
        let watch = graph::GraphLaneSwapDefectWatch::test_only_new(strips);
        let used = current_thread_delta_since(mark);
        let made = usize::try_from(used.requested_bytes - used.released_bytes)
            .expect("a small allocation");
        assert!(made > 0, "{names:?}: the watch makes its table");
        assert_eq!(watch.heap_bytes(), made + given, "{names:?}");
    }
}
