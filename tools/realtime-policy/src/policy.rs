//! The tool's constants.

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
}

impl Policy {
    /// The real policy of this workspace.
    #[must_use]
    pub fn workspace() -> Self {
        // The unsafe-ownership files. `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership",
        // carries the justification of each.
        const UNSAFE_ALLOWLIST: [&str; 19] = [
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
            marked_files_floor: 26,
            marked_regions_floor: 94,
        }
    }
}
