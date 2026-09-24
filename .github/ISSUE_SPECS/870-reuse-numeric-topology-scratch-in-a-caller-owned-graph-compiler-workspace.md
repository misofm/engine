# Reuse numeric topology scratch in a caller-owned graph compiler workspace

GitHub: https://github.com/misofm/engine/issues/870

## Outcome and dependencies

Deliver the first usable compiler workspace on Rust 1.97.1. No predecessor in this series. Starting source baseline: `e09302ad`; refresh against synchronized main before implementation and record any relevant changes. This follows the delivered CP1 borrowed-key work; it does not reopen those issues or inherit their exhausted attempts.

## Exact implementation slice

Own `crates/graph-compiler/src/lib.rs`, `compile.rs`, `schedule.rs`, and a new `workspace.rs` plus focused tests in this crate. Add unversioned `GraphCompileWorkspace`, constructed with explicit `max_retained_bytes`, and additive `GraphCompiler::compile_with_workspace` / `compile_with_builtins_and_workspace` entry points. Existing entry points remain compatibility wrappers using ephemeral zero-retention workspaces.

Reuse only topo's `ordered: Vec<usize>`, `degree: Vec<u64>`, and `node_levels: Vec<Option<u64>>`. Extract a private typed borrowed-slice view into the existing topo algorithm. Both fresh and retained owners call that same algorithm. Initialize every active element on every call. Leave borrowed ID maps, ready sets, adjacency vectors, levels, cycle traversal, prepared effects and final outputs under their existing ownership. Do not flatten adjacency, replace the ready-set algorithm, or change ordering.

The workspace holds no session references, graph IDs, plans, DSP state or queue endpoints. It belongs to a serialized control caller and is independent of returned artifacts. Expose checked retained capacity bytes and explicit `release()`; record capacity times element size, not vector length. After every returned success/error, clear logical contents and release all scratch backing if its combined capacity exceeds the configured retention cap. Zero releases all backing. This cap bounds storage kept between calls; it is not a new peak preparation limit or a guarantee of recoverable global OOM. Preserve existing request ownership and error precedence. Overflow in retained-byte accounting releases storage rather than wrapping. Keep canonical reports and resource estimates about the plan unchanged.

## Objective gates

- Use existing independent ordering/canonical oracles and retain one compact original graph-fixture manifest as a baseline, since both candidate API entry points will share the refactored algorithm. Keep that evidence with the final review packet; no artifact-promotion project. Require candidate semantics and returned owner identities to match; do not regenerate historical expected fixtures to hide differences.
- Test repeated equal-size, growth, large-to-small, empty/malformed topology, shuffled IDs, parallel edges, and failure followed by a valid request. Include an error after topo has actually run. Preserve deterministic level/schedule order, canonical graph identity and transactional owner return.
- Test retention at zero, below, exactly at, and above observed capacity; capacity reports include every owned scratch allocation. Returned artifacts remain valid after workspace release/drop and after a later compile.
- Run graph-compiler library tests, focused release tests, affected strict Clippy, formatting, graph/workspace policy, and scalar plus simd128 Wasm checks. Existing independent ordering tests must continue to pass. Add no benchmark or timing to this issue.

The measurable claim is reuse of three selected scratch arrays, not allocation-free compilation. Commit once the slice compiles and focused tests pass, before another implementation tranche begins.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
