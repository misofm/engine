//! #1060: the browser identity fixture's retained-memory budgets, on the native build.
//!
//! Owner decision 4 (`docs/rulings/engine-footprint-2026-09-28.md`): memory tests assert budgets
//! (upper limits), not exact byte counts. `tests/browser-v1/expected.json` used to pin every
//! resource row of this fixture to the byte, and every harmless layout change re-pinned it. The
//! shipped `wasm32` module's rows are now budgeted in `expected.json`'s `resourceCeilings` and
//! checked by `scripts/check-browser-expected-resources.py`; this test budgets the same fixture,
//! booted through the same facade with the options `direct-oracle.mjs` writes, at the native widths,
//! and proves the budget binds through the host's own admission.
//!
//! The completeness half of decision 4 -- that the reports charge every byte -- is capi's
//! allocator oracle, `crates/capi/tests/resource_lifecycle.rs`.

use host_web::{
    AudioWorkletEngineHost, LIVE_RESPONSE_CAPTURE_BYTES, RESULT_REFUSED_BUDGET, WebBootOptions,
    WebResourceReport,
};

/// The fixture the browser harness compiles, verbatim.
const SESSION_DOCUMENT: &str = include_str!("browser-v1/session.json");

/// The options the direct oracle writes into the module's `WebBootOptions` block: the physical
/// rate and quantum, plus a one-quantum ring that makes backpressure deterministic.
fn options() -> WebBootOptions {
    WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        ..WebBootOptions::explicit_defaults()
    }
}

/// One retained-memory budget: a report row and its ceiling at each native lane width.
struct Budget {
    row: &'static str,
    value: fn(&WebResourceReport) -> u64,
    eight_lanes: u64,
    four_lanes: u64,
}

impl Budget {
    /// The native lane width is a compile-time function of the target: eight lanes on
    /// x86-64-v3, four on AArch64 NEON. Matched by width, not by variant: the scalar variant
    /// exists only under `lane/test-support` (#1059), and no native product build selects it.
    fn ceiling(&self) -> u64 {
        match lane::Backend::current().width() {
            8 => self.eight_lanes,
            4 => self.four_lanes,
            width => panic!("no retained budget is declared for a {width}-lane native build"),
        }
    }
}

/// The 1 MiB live-response capture is a fixed ABI capacity inside the two bridge rows, so the
/// growth budget applies to the rest of each row.
const CAPTURE: u64 = LIVE_RESPONSE_CAPTURE_BYTES as u64;

/// **Headroom.** Each ceiling is the row's value on 2026-09-28 (`a509b681`) at that width, plus
/// 10 %, rounded up to a 64-byte multiple; for the two bridge rows the 10 % applies to the part
/// beyond the fixed 1 MiB capture. 10 % is where the history puts the line: #1060's review sampled
/// four past moves of these retained rows, +1.2 %, +2.3 %, +11 % and +48 %, and a 10 % budget lets
/// the two routine moves through unseen and stops the two structural ones, which then raise the
/// budget with their reason in the same commit. A zero ceiling is a claim: this identity
/// fixture declares no delay, no scalar effect and no observation capacity. The measured baselines,
/// x86-64 / AArch64 (AArch64 under qemu-user): bridge metadata 1,150,215 and bridge retained
/// 1,170,724 at both; source total 3,442 and overhead 2,418 at both; builtin retained 1,957 at both;
/// graph session+plan and incremental 48,034 / 35,230; graph metadata 6,711 / 6,675.
///
/// **Re-pinned: `builtin_retained_bytes` (#1326, then #1312).** Every browser plan carries each
/// strip's fader/mute and matrix/pan lanes, whatever the app's live-control options say (the
/// engine ramps a strip's mute for its own transitions). #1326 added them as two 16-record rings
/// per strip, 1,234 bytes retained for this one-strip fixture (2,149 to 3,383), and re-pinned the
/// ceiling to 3,776 (3,383 * 1.1 = 3,721.3, rounded up to 64). #1312 replaced the rings with
/// latest-target cells and the row fell to 2,887 (x86-64; 2,723 measured on the wasm32 simd128
/// module, `expected.json`). The ceiling is the new value plus 10 %, rounded up to 64 bytes:
/// 2,887 * 1.1 = 3,175.7, so 3,200. A ceiling left at 3,776 would let 889 bytes of regression
/// through unseen. Both lane widths carry that ceiling, but only the eight-lane value is measured
/// (x86-64); the four-lane one is inferred, because AArch64 runs only in CI: the cells are words,
/// not lanes, so the figure should match, and the 10 % headroom covers a small difference.
const BUDGETS: [Budget; 12] = [
    Budget {
        row: "bridge_metadata_bytes",
        value: |report| report.bridge_metadata_bytes,
        eight_lanes: CAPTURE + 111_808,
        four_lanes: CAPTURE + 111_808,
    },
    Budget {
        row: "bridge_retained_bytes",
        value: |report| report.bridge_retained_bytes,
        eight_lanes: CAPTURE + 134_400,
        four_lanes: CAPTURE + 134_400,
    },
    Budget {
        row: "source_total_bytes",
        value: |report| report.source_total_bytes,
        eight_lanes: 3_840,
        four_lanes: 3_840,
    },
    Budget {
        row: "source_overhead_bytes",
        value: |report| report.source_overhead_bytes,
        eight_lanes: 2_688,
        four_lanes: 2_688,
    },
    Budget {
        row: "builtin_retained_bytes",
        value: |report| report.builtin_retained_bytes,
        eight_lanes: 3_200,
        four_lanes: 3_200,
    },
    Budget {
        row: "graph_session_plus_plan_bytes",
        value: |report| report.graph_session_plus_plan_bytes,
        eight_lanes: 52_864,
        four_lanes: 38_784,
    },
    Budget {
        row: "graph_incremental_plan_bytes",
        value: |report| report.graph_incremental_plan_bytes,
        eight_lanes: 52_864,
        four_lanes: 38_784,
    },
    Budget {
        row: "graph_metadata_bytes",
        value: |report| report.graph_metadata_bytes,
        eight_lanes: 7_424,
        four_lanes: 7_360,
    },
    Budget {
        row: "graph_delay_bytes",
        value: |report| report.graph_delay_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "effect_scalar_state_bytes",
        value: |report| report.effect_scalar_state_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "effect_scalar_scratch_bytes",
        value: |report| report.effect_scalar_scratch_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "observation_retained_bytes",
        value: |report| report.observation_retained_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
];

fn ceiling(row: &str) -> u64 {
    BUDGETS
        .iter()
        .find(|budget| budget.row == row)
        .unwrap_or_else(|| panic!("no budget row {row}"))
        .ceiling()
}

#[test]
fn browser_identity_fixture_retained_rows_stay_within_their_budgets() {
    let host = AudioWorkletEngineHost::boot(SESSION_DOCUMENT.as_bytes(), options()).unwrap_or_else(
        |failure| panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic())),
    );
    let report = *host.resources();
    drop(host);

    // Every row against its ceiling, reported whole before any assertion so one red run shows
    // every row's headroom.
    let mut over = Vec::new();
    for budget in &BUDGETS {
        let value = (budget.value)(&report);
        let ceiling = budget.ceiling();
        println!("{}: {value} of {ceiling}", budget.row);
        if value > ceiling {
            over.push(format!("{} {value} > {ceiling}", budget.row));
        }
    }
    assert!(
        over.is_empty(),
        "retained rows over their budget (raise the budget with its reason, or find the \
         regression): {over:?}"
    );

    // The budget binds through the host's own admission. The browser's one cap is its total boot
    // memory budget, checked against the exact retained aggregate of the bridge, graph and source
    // rows: a host configured at the budgets' sum boots the fixture, and one configured a byte
    // below the fixture's own aggregate refuses it.
    let exact = report.bridge_retained_bytes
        + report.graph_session_plus_plan_bytes
        + report.source_total_bytes;
    let budgeted = ceiling("bridge_retained_bytes")
        + ceiling("graph_session_plus_plan_bytes")
        + ceiling("source_total_bytes");
    let at_budget = AudioWorkletEngineHost::boot(
        SESSION_DOCUMENT.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: budgeted,
            ..options()
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "a host at the budgeted {budgeted} bytes refused the fixture: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    });
    assert_eq!(*at_budget.resources(), report, "the budget moves no row");
    drop(at_budget);
    let refused = AudioWorkletEngineHost::boot(
        SESSION_DOCUMENT.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: exact - 1,
            ..options()
        },
    )
    .err()
    .expect("one byte below the fixture's exact retained aggregate must refuse");
    assert_eq!(refused.result(), RESULT_REFUSED_BUDGET);
}

/// The browser's graph row charges the allocation the plan makes for itself and not the session's
/// (#1502 m1, #1312 m3, #1399). The browser copies host-core's graph row unchanged, and the graph
/// compiler's row is its incremental row plus the session bytes the facade does not pass it, so the
/// row less the incremental row is the plan's own: its revision gate. The session's
/// `live_values_superseded` count is one allocation every plan of the session shares, so the
/// browser charges it once, in its bridge rows (the exact-decomposition tests in `src/tests.rs`
/// add it there), and the plan's row must not charge it again. A row that leaves the gate out would
/// let the exact aggregate under-report a real allocation, and one that charges the count per plan
/// would charge it twice; no ceiling above would notice either.
#[test]
fn the_graph_row_charges_the_plans_gate_and_not_the_sessions_superseded_count() {
    let host = AudioWorkletEngineHost::boot(SESSION_DOCUMENT.as_bytes(), options()).unwrap_or_else(
        |failure| panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic())),
    );
    let report = *host.resources();
    assert_eq!(
        report.graph_session_plus_plan_bytes - report.graph_incremental_plan_bytes,
        engine::realtime::plan_revision_gate_retained_bytes(),
        "the browser's graph row: the graph's own bytes and the plan's revision gate"
    );
}
