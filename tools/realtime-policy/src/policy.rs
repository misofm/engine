//! The tool's constants.

use crate::table::{self, RenderThreadExport};

/// The table of the exports either AudioWorklet calls after construction, shared with the two
/// worklet harnesses.
const RENDER_THREAD_EXPORTS: &str = include_str!("../render-thread-exports.json");

/// The exports of that table that are not wrapped in `render_locked`, each with its reason in the
/// table: `docs/REALTIME_DEPENDENCY_POLICY.md`, "The browser render-locked allocator".
///
/// Issue #1381a adds the two control exports of the single-instance mode: the service step and the
/// control allocation counter. The worklet's message handler calls them outside `process()` and
/// outside every render-locked window, because their allocations and frees are the control
/// counter's (`render_lock.rs`).
///
/// Issue #1562a adds the five control exports of the generic command entry (D1): the request
/// staging address, the command, the response address and length, and the committed revision. They
/// run in the message handler too, and `SessionState::command` allocates and frees, so they are
/// never inside a render-locked window.
///
/// Issue #1570b adds `control_event` (D1), the event entry: the worklet's message handler dequeues
/// both event lanes through it after each message, and `SessionState::dequeue_event` runs the
/// service step and encodes the frame, so it is control work of the same kind.
///
/// Issue #1290d1 added `meter_header_ptr` here; #1332 S7 wraps it (the worker-mode `process()`
/// reads the current header through it, root ruling R-S7-meters), so it left this list.
///
/// Issue #1294a adds `watermark_read`: the worklet's message handler copies the applied-revision
/// watermark into its fixed record after each service step, outside `process()` and every
/// render-locked window.
///
/// Issue #1697 adds `replace_document_ptr`: the worklet's message handler stages the document a
/// `SESSION_REPLACE` frame names beside the frame, a control allocation, never inside `process()`
/// or a render-locked window.
///
/// Issue #1644 adds `session_seek` and `session_seek_at` (D1): the worklet's message handler seeks
/// the timeline and every source through them between blocks, so their allocations are control
/// allocations and they are never inside `process()` or a render-locked window.
///
/// Issue #1297a adds `source_seek_at` and `source_seek_report`: the worklet's message handler
/// applies an anchored source seek and reads a seek report through them between blocks; both run
/// the service step, so they are control work, never inside `process()` or a render-locked window.
///
/// Issue #2116 adds `plan_latency_read`: the worklet's message handler copies the running plan's
/// latency and tail row into its fixed record, control work as `source_seek_report` is, never
/// inside `process()` or a render-locked window.
const UNWRAPPED_EXPORTS: [&str; 17] = [
    "dispose",
    "render_allocation_count",
    "service",
    "control_allocation_count",
    "control_request_ptr",
    "control_command",
    "replace_document_ptr",
    "control_event",
    "control_response_ptr",
    "control_response_bytes",
    "control_revision",
    "watermark_read",
    "session_seek",
    "session_seek_at",
    "source_seek_at",
    "source_seek_report",
    // #2116: control work in the message handler, as `source_seek_report` is.
    "plan_latency_read",
];

/// One entry of the control-side allowlist (#1443 D3): a function whose `try_pop` calls run on a
/// control, teardown or harness thread, never inside a product render callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlSideEntry {
    /// The file, relative to the workspace root with `/` separators.
    pub path: String,
    /// The canonical function path inside the file, with generics removed: `<fn>` for a free
    /// function, `<Type>::<fn>` for an inherent method, `<Trait> for <Type>::<fn>` for a trait
    /// method, each with `<mod>::` prefixes for the enclosing inline modules.
    pub function: String,
    /// The exact number of non-test `try_pop` sites in that function's own body, macro tokens
    /// included.
    pub sites: usize,
    /// The thread that runs the function on every host that runs it.
    pub thread: String,
    /// Why the pop never runs inside a product render callback.
    pub reason: String,
}

/// The day-one control-side allowlist (#1443 D7): path, function, sites, thread, reason. A reason
/// says why the pop never runs inside a product render callback (the C ABI render call, the
/// worklet's `process()` or a worklet message handler).
const CONTROL_SIDE: [(&str, &str, usize, &str, &str); 14] = [
    (
        "crates/control-plane/src/loudness.rs",
        "LoudnessMonitor::drain",
        1,
        "the session's control thread (both hosts' service step and LOUDNESS commands)",
        "the loudness meter's control half drains its hop records off the render thread; the render half only pushes (#2115)",
    ),
    (
        "crates/protocol/src/queue.rs",
        "ProtocolQueues::try_dequeue_event",
        1,
        "C ABI control thread",
        "ProtocolController dequeues reliable events on the control side; the protocol crate is not linked into the browser artifact",
    ),
    (
        "crates/protocol/src/queue.rs",
        "ProtocolQueues::try_dequeue_telemetry",
        1,
        "C ABI control thread",
        "ProtocolController batches telemetry on the control side; the protocol crate is not linked into the browser artifact",
    ),
    (
        "crates/protocol/src/queue.rs",
        "ProtocolQueues::try_dequeue_counter_telemetry",
        1,
        "C ABI control thread",
        "ProtocolController batches counter telemetry on the control side; the protocol crate is not linked into the browser artifact",
    ),
    (
        "hosts/host-web/src/lib.rs",
        "ControlHalf::reclaim_shapes",
        1,
        "the browser host's control half (the service step)",
        "the hand-back ring's consumer is the control half, which drops the retired plans' tables off the render thread; render only pushes (#1332 S5)",
    ),
    (
        "crates/engine/src/realtime/plan_exchange.rs",
        "PlanRetirer::try_reclaim",
        1,
        "the control/retirement owner",
        "the retirement queue's consumer is the control side, which reclaims a displaced plan off the render thread (its doc comment)",
    ),
    (
        "crates/engine/src/realtime/plan_exchange.rs",
        "Drop for PlanRetirer::drop",
        1,
        "the retirement owner, at teardown",
        "destroys retired plans on the retirement owner's thread, not on the render thread (its doc comment)",
    ),
    (
        "tools/console-workload/src/lib.rs",
        "SessionRuntime::drain_meter_snapshots",
        1,
        "the benchmark driver's thread",
        "the driver drains between blocks on the thread that renders, outside every render call",
    ),
    (
        "tools/audit/src/builtins_graph.rs",
        "run_retirement_worker",
        1,
        "the audit's retirement worker",
        "the worker thread pops its own command queue; it never renders",
    ),
    (
        "tools/audit/src/builtins_graph.rs",
        "drain_values",
        2,
        "the audit harness thread",
        "the harness drains each meter between render calls, on the thread that renders",
    ),
    (
        "tools/audit/src/builtins_graph.rs",
        "drain_fixture",
        2,
        "the audit harness thread",
        "the harness drains each meter between render calls, on the thread that renders",
    ),
    (
        "tools/audit/src/fixture_builtins.rs",
        "graph_tap_fixtures",
        1,
        "the audit harness thread",
        "the harness reads each tap's window after the render call returns",
    ),
    (
        "tools/audit/src/fixture_builtins.rs",
        "meters",
        10,
        "the audit harness thread",
        "the harness drains each meter between render calls, on the thread that renders",
    ),
    (
        "crates/builtins-compiler/src/lib.rs",
        "tests::render_post_input_bits_with_variant",
        1,
        "the test harness (test-support builds)",
        "the harness renders and drains in turn on one thread; it is a test-support helper, not a product render callback",
    ),
];

/// The constants a check reads. `Policy::workspace` holds the real values; a library case builds
/// its own by replacing fields, so no case depends on the real tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Policy {
    /// The files, relative to the workspace root with `/` separators, that may hold unsafe code.
    pub unsafe_allowlist: Vec<String>,
    /// The fewest marked files a tree may hold.
    pub marked_files_floor: usize,
    /// The fewest marked regions a tree may hold.
    pub marked_regions_floor: usize,
    /// The exports either AudioWorklet calls after construction (`hosts/host-web/src/ffi.rs`).
    pub render_thread_exports: Vec<RenderThreadExport>,
    /// The full names of the table's entries that may be unwrapped.
    pub unwrapped_exports: Vec<String>,
    /// The functions whose `try_pop` calls run off every render thread (#1443 D3). A non-test pop
    /// outside every marked region must be in the body of one of these.
    pub control_side_allowlist: Vec<ControlSideEntry>,
}

impl Policy {
    /// The real policy of this workspace.
    ///
    /// # Panics
    ///
    /// If the embedded `render-thread-exports.json` is malformed; a test holds it well formed.
    #[must_use]
    pub fn workspace() -> Self {
        // The unsafe-ownership files. `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership",
        // carries the justification of each.
        const UNSAFE_ALLOWLIST: [&str; 29] = [
            "crates/engine/src/realtime/spsc.rs",
            "crates/engine/src/realtime/disjoint.rs",
            "crates/lane/src/softfma.rs",
            "crates/lane/src/fpenv.rs",
            "crates/lane/tests/fp_env.rs",
            "crates/lane/tests/g6_ftz_inert.rs",
            "crates/capi/src/runtime/tests.rs",
            "crates/host-core/tests/fp_environment.rs",
            "tools/wasm-gates/tests/g6_full_corpus_ftz.rs",
            "crates/builtins-compiler/tests/allocation_tracker.rs",
            "crates/session/tests/allocation_budget.rs",
            "crates/capi/src/ffi.rs",
            "crates/capi/tests/resource_lifecycle.rs",
            "crates/capi/tests/input_design_bounds.rs",
            "crates/capi/tests/plan_swap_race.rs",
            "crates/capi/tests/response_get.rs",
            "crates/capi/tests/response_strip.rs",
            "crates/capi/tests/session_replace.rs",
            "crates/capi/tests/empty_boot.rs",
            "crates/capi/tests/strip_transitions.rs",
            "hosts/host-web/src/ffi.rs",
            // #1332 S6: the handoff registry's address round trip and the stack reservation.
            "hosts/host-web/src/handoff.rs",
            "hosts/host-web/src/render_lock.rs",
            "hosts/host-web/tests/boot_transient_budget.rs",
            "hosts/host-web/tests/control_parity.rs",
            "hosts/host-web/tests/protocol_cross_host.rs",
            "hosts/blake3-web/src/lib.rs",
            "tools/bench-support/src/alloc.rs",
            "tools/audit/src/capi.rs",
        ];
        Self {
            unsafe_allowlist: UNSAFE_ALLOWLIST
                .iter()
                .map(|path| (*path).to_owned())
                .collect(),
            // The floors equal the tree's counts, so a deleted marker or file fails here. They
            // carry stream B's raises (#1432's `latest_cell.rs`, #1277, #1279, #1346, #1347 and
            // #1345 D9: the awk gate's 118 regions in 27 files), #1448's `poll_meters` region and
            // #1443's seven (three in `crates/source`, four in `spectrum.rs`) and the spectrum
            // service's two (`spectrum_service.rs`: the prepared captures' one-shot methods and the
            // service's arm, read, cancel and select; #1591 fold) and #1609's one (`spectrum.rs`:
            // the one-shot render functions and the held-window publication) and #1637's three
            // (`crates/automation`: `value_at`, the event generator in `events.rs`, and the render
            // reads of `CellProgram` and `Program::cell` in `program.rs`) and #1638's three
            // (`crates/builtins`: the fader operation sink, and the bank's and the scalar track's
            // `process_with`) and #1639's two (`crates/graph`: the jump-length cell's word decode
            // and its block-entry read) and #1630's one (`crates/builtins-compiler`: the cell
            // readers' carry marks, D15-2) and #1316's (`seqlock.rs`, the engine's one seqlock
            // writer, B15-R1). The floors had fallen behind the tree before #1637; they are set to
            // its counts again. A change that adds a marker raises them in the same change: #1640
            // adds two in `crates/source` (the shared seek state's observe and reads, and the
            // timeline consumer's block entry and history readers).
            // #2115 adds two: the loudness meter's render half in `crates/loudness` (S1) and its
            // session tap in `crates/control-plane/src/loudness.rs` (S2).
            marked_files_floor: 35,
            marked_regions_floor: 155,
            render_thread_exports: table::parse(RENDER_THREAD_EXPORTS)
                .unwrap_or_else(|error| panic!("render-thread-exports.json: {error}")),
            unwrapped_exports: UNWRAPPED_EXPORTS
                .iter()
                .map(|name| format!("{}{name}", table::PREFIX))
                .collect(),
            control_side_allowlist: CONTROL_SIDE
                .iter()
                .map(
                    |&(path, function, sites, thread, reason)| ControlSideEntry {
                        path: path.to_owned(),
                        function: function.to_owned(),
                        sites,
                        thread: thread.to_owned(),
                        reason: reason.to_owned(),
                    },
                )
                .collect(),
        }
    }
}
