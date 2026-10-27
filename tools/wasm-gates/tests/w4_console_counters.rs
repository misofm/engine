//! Issue #2143: the W=4 bank counters of the C1 console rows.
//!
//! The wasm leg (`wasm_gates <guest.wasm>`, run by `scripts/run-wasm-gates.sh`) reads every row's
//! counters out of the `simd128` guest under wasmtime and fails on any that differs from
//! [`W4_CONSOLE_COUNTERS`]. What runs here is the native half of that comparison, which only a
//! four-lane build can run, and the table's tie to the banking design's W=4 plan, which runs
//! everywhere.

use console_workload::{BANK_COUNTER_NAMES, W4_COUNTER_BLOCKS, W4_COUNTER_WORKLOADS};
use wasm_gates::{W4_CONSOLE_COUNTERS, native_console_counters};

/// Index of a counter in [`BANK_COUNTER_NAMES`].
fn counter(name: &str) -> usize {
    BANK_COUNTER_NAMES
        .iter()
        .position(|candidate| *candidate == name)
        .expect("a bank counter")
}

/// The native `Simd4` leg: every counter of every row, read in this process, equals the pin the
/// wasm leg is held to, so the two legs agree integer for integer.
///
/// Claim: the `simd128` module and native `Simd4` make the same bank decisions on the C1 rows. An
/// eight-lane build cannot produce that plan (`console.slot.foreign_width`, #1694), so it runs on a
/// four-lane build only: CI's `aarch64-release` job, `cargo test -p wasm-gates`.
#[test]
#[cfg_attr(
    not(any(target_feature = "neon", target_feature = "simd128")),
    ignore = "needs a four-lane build: an eight-lane one cannot bank C1's console at four lanes"
)]
fn w4_counters_match_between_wasm_and_native_simd4() {
    let rows = native_console_counters().expect("a four-lane build");
    assert_eq!(rows.len(), W4_CONSOLE_COUNTERS.len(), "every row is read");
    for (row, pins) in rows.iter().zip(&W4_CONSOLE_COUNTERS) {
        assert_eq!(
            &row.counters, pins,
            "{}: native Simd4 counters {BANK_COUNTER_NAMES:?} differ from the wasm leg's pins",
            row.row
        );
    }
}

/// The pinned table describes the plan the banking design's W=4 finish line is stated for
/// (`docs/BANKING_DESIGN.md`, section 7: 16 tracks, W=4; 28 bank-slot runs per block since the
/// windows planner, #2136).
///
/// Claim: re-pinning the table from a counter summed over the wrong units, or from a plan that
/// stopped matching the design, turns this red. The track banks are `ceil(16 / 4)` per console
/// rack (`pre_insert`: gate, EQ, compressor; `post_insert`: drive). The submix banks are six at
/// W=4: `pre_insert` over `{aux A, aux B}` (one bank: the auxes take post-fader sends, a level
/// before the groups, and aux A's delay keeps its window to that level) and over the four groups
/// (one), the six submix drives as one cohort over level windows (two), and the master's
/// `pre_insert` and drive (one each, split by its limiter).
#[test]
fn w4_counters_describe_the_designs_plan() {
    const W: u64 = 4;
    const TRACKS: u64 = 16;
    let track_banks = 2 * TRACKS.div_ceil(W);
    let track_bsr = (3 + 1) * TRACKS.div_ceil(W);
    let submix_banks = 2 + 6_u64.div_ceil(W) + 2;
    let submix_bsr = 3 * 2 + 6_u64.div_ceil(W) + (3 + 1);
    assert_eq!(track_bsr + submix_bsr, 28, "the design's W=4 table");

    let row = |kind: &str| {
        W4_COUNTER_WORKLOADS
            .iter()
            .position(|workload| workload.kind() == kind)
            .map(|index| W4_CONSOLE_COUNTERS[index])
            .unwrap_or_else(|| panic!("row {kind}"))
    };
    for (workload, pins) in W4_COUNTER_WORKLOADS.iter().zip(&W4_CONSOLE_COUNTERS) {
        let kind = workload.kind();
        assert_eq!(
            pins[counter("banks")],
            track_banks + submix_banks,
            "{kind}: banks"
        );
        assert_eq!(pins[counter("mergeable_partial_pairs")], 0, "{kind}: pairs");
    }
    for kind in ["c1_console_16_all_active", "c1_console_16_sparse"] {
        // No bypassed lane and no whole silent track bank: every track bank runs every slot.
        assert_eq!(
            row(kind)[counter("bank_slot_runs_track_group")],
            track_bsr * W4_COUNTER_BLOCKS,
            "{kind}: track bank-slot runs"
        );
        assert_eq!(
            row(kind)[counter("bypass_skips")],
            0,
            "{kind}: nothing bypassed"
        );
    }
    // #2129: the load state bypasses every console slot, so no bank runs its wet path.
    let load = row("c1_console_16_load");
    assert_eq!(
        load[counter("bank_slot_runs_total")],
        0,
        "load: no bank runs"
    );
    assert!(load[counter("bypass_skips")] > 0, "load: the banks skip");
}
