//! The tool's constants.

use crate::table::{self, RenderThreadExport};

/// The table of the exports either AudioWorklet calls after construction, shared with the two
/// worklet harnesses.
const RENDER_THREAD_EXPORTS: &str = include_str!("../render-thread-exports.json");

/// The exports of that table that are not wrapped in `render_locked`, each with its reason in the
/// table: `docs/REALTIME_DEPENDENCY_POLICY.md`, "The browser render-locked allocator".
const UNWRAPPED_EXPORTS: [&str; 2] = ["dispose", "render_allocation_count"];

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
        const UNSAFE_ALLOWLIST: [&str; 20] = [
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
            "crates/capi/tests/design_bound_cache.rs",
            "crates/capi/tests/plan_swap_race.rs",
            "hosts/host-web/src/ffi.rs",
            "hosts/host-web/src/render_lock.rs",
            "hosts/host-web/tests/boot_transient_budget.rs",
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
            // #1345 D9: the awk gate's 118 regions in 27 files) and #1448's `poll_meters` region.
            // A change that adds a marker raises them in the same change.
            marked_files_floor: 27,
            marked_regions_floor: 119,
            render_thread_exports: table::parse(RENDER_THREAD_EXPORTS)
                .unwrap_or_else(|error| panic!("render-thread-exports.json: {error}")),
            unwrapped_exports: UNWRAPPED_EXPORTS
                .iter()
                .map(|name| format!("{}{name}", table::PREFIX))
                .collect(),
        }
    }
}
