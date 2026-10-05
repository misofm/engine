# Resolve graph node IDs to dense indices once per graph compilation

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-15).
Code anchors verified on `main` at `6fb211594`.

Second half of D15-15, split from *Validate each effect descriptor once per type, not once per prepared
instance* (#1330).

## Product outcome

Graph compilation on both hosts stops comparing `GraphNodeId` strings in its per-edge passes. Each
node ID is ordered once and each edge endpoint is resolved to a dense index once per pass family,
and every later pass works on integers. No rendered bit, schedule, diagnostic or canonical text
changes.

## Context

- **The evidence.** A phase profile of the browser module's preparation
  (`docs/handoffs/decision-15-2026-10-05/PLAN-2026-10-05-adversary-round1.md`, C4, scratch profiles, not
  committed) attributes 11-12 % of preparation, inclusive, to `GraphNodeId` compares in the compiler
  hot path. The frozen workload is #1289's four boot documents
  (`artifacts/steps/web-rebuild-base/report.md`).
- **Why a compare is a string compare.** `GraphNodeId` (`crates/graph/src/lib.rs:273-291`) is an
  enum whose variants hold `StableGraphId(String)` (`:228`) or a `Box<GraphEdgeId>`; its derived
  `Ord` compares those strings.
- **Per-compile passes that key on `GraphNodeId`** (`crates/graph-compiler/src/compile.rs:426-454`
  calls them in this order):
  - `cycle_witnesses` (`schedule.rs:119-199`): two `BTreeMap<GraphNodeId, Vec<&GraphEdge>>` with a
    clone per node, and a Kosaraju walk over `BTreeSet<GraphNodeId>`, on every compile, acyclic or
    not.
  - `topo` (`schedule.rs:9-81`): already dense inside, but resolves both ends of every edge through
    `BTreeMap<&GraphNodeId, usize>` (`:14`, `:25-26`).
  - `timings` (`pdc.rs:38-...`): `BTreeMap<&GraphNodeId, Vec<&GraphEdge>>` (`:45`) and two more maps
    (`:53-54`), looked up per edge; `latencies`/`tails` are `BTreeMap<GraphNodeId, _>` filled by
    `add_node` (`ids.rs:118-129`).
  - `buffer_assignments` (`schedule.rs:247-...`): `BTreeMap<&GraphNodeId, usize>` (`:251`).
  - `resource_estimate` (`estimate.rs:28-...`): `BTreeMap<&GraphNodeId, u64>` input counts (`:51-55`).
- **Lowering** (`crates/graph/src/program.rs:549`, `lower_with`) calls `node_index` (`:173-178`, a
  binary search with `GraphNodeId::cmp`) for both ends of every edge several times: `:572`, `:588`,
  `:602-604`, `:639-647`, `:668`, `:685`, `:706`, and in `first_main_producer` (`:255-257`).
- **Order is the contract.** Levels are emitted in ascending node-ID order (`schedule.rs:1-4`), and
  `GraphSpec::nodes` is sorted by `GraphNodeId` (`program.rs:41`). A dense index assigned in
  ascending ID order therefore iterates in exactly the order the maps iterate today.

## Decisions frozen for this slice

- **D1. One node table per compile.** In `compile.rs`, once `nodes` and `edges` are final, build a
  `NodeTable`: node positions sorted once by `GraphNodeId` (dense index = rank), and an
  `EdgeEnds { source: u32, destination: u32 }` per edge, resolved once by binary search. A
  duplicate ID or a dangling endpoint is detected here and keeps today's outcome (`topo` returns
  `None`; `cycle_witnesses` returns no witness), as the tests at
  `crates/graph-compiler/src/lib.rs:1970-2001` pin.
- **D2. Passes take the table.** `cycle_witnesses`, `topo`, `timings`, `buffer_assignments` and
  `resource_estimate` take `&NodeTable` and use `Vec`s indexed by dense index instead of
  `BTreeMap`/`BTreeSet` keyed by `GraphNodeId`. `latencies`/`tails` become `Vec`s indexed the same
  way. `GraphNodeId` values are cloned only into outputs (levels, schedule, witnesses, diagnostics).
- **D3. Lowering resolves once.** `lower_with` resolves every edge's ends once into a local
  `Vec<(usize, usize)>` and passes it to `first_main_producer` and the later loops; `node_index`
  stays public for other callers.
- **D4. Iteration order.** Every loop that iterated a map in key order iterates the dense range in
  ascending order; every per-node edge list keeps edge order. So levels, schedule, buffers, delays,
  witnesses and diagnostics are unchanged.
- **D5. Bits.** No arithmetic changes. The PR records the one-time "no bit moved" comparison of
  gate 4 as evidence.

## Deliverables

The D1-D3 edits; the measurement of gate 5 with its two step records; the profile evidence of
gate 6 in the PR.

## Authorized paths

- `crates/graph-compiler/src/compile.rs`, `schedule.rs`, `pdc.rs`, `ids.rs`, `estimate.rs`, and the
  tests in `crates/graph-compiler/src/lib.rs` that call the changed signatures
- `crates/graph/src/program.rs` (`lower_with` and `first_main_producer` only)
- `artifacts/steps/d15-15-node-ids-before/`, `artifacts/steps/d15-15-node-ids-after/` (new)

## Non-goals

- Interning `StableGraphId` upstream, or changing `GraphNodeId`, `GraphSpec` or any wire.
- The runtime binding maps in `crates/graph/src/runtime.rs` (bind/lower/executor cost).
- Any change to PDC arithmetic or floors (#1285 owns those).

## Hazards

- `pdc.rs` and `crates/graph-compiler/src/lib.rs` are also edited by *Keep every node's latency
  from dropping during playback* (#1285, stream A). This slice starts after #1285 has merged and is
  rebased on it; the floor code then runs on dense indices too.

## Objective gates

1. `cargo test --locked -p graph-compiler -p graph --features graph/test-support` passes,
   including `topo`'s and `cycle_witnesses`' existing tests and `tests/compile_shapes.rs`.
2. The qualification `test-debug-a` line (as in #1330 gate 2) passes.
3. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
   `bash scripts/check-graph-policy.sh` and `bash scripts/check-workspace-policy.sh` pass.
4. **No bit moved (PR evidence).** `cargo run --locked --release -q -p host-web --example sdk_render_oracle -- 64 < F`
   on base and branch for the four `fixtures/session/v1/` documents of #1330 gate 4: equal digests.
5. **Frozen-workload before/after (descriptive).** As #1330 gate 5, with steps
   `d15-15-node-ids-before` and `d15-15-node-ids-after`. No pass threshold.
6. **Profile (descriptive).** `perf record -g` of `target/release/examples/sdk_render_oracle 1 <
   fixtures/session/v1/console-sixty-four-track-sends.json` built with
   `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`, before and after: the PR reports the inclusive
   share of `GraphNodeId`/`StableGraphId` comparison, which after the change should come only from
   D1's one sort and one resolution per compile and D3's one resolution per lower.

## Test value

- No new test is required: the existing `topo` tests (dangling ends, duplicates, cycles, reversed
  insertion order), the cycle-witness tests and the compile-shape corpus pin the outputs this
  refactor must keep. If the implementer finds an output with no test (for example witness order
  across two disjoint cycles), it adds one and names the defect it catches: a dense index that
  iterates in insertion order rather than ID order.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285): this slice rebases on it.
