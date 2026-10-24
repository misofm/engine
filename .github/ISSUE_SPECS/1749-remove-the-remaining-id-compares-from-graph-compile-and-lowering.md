# Remove the remaining ID compares from graph compile and lowering

Successor of #1384, which resolved node IDs to dense indices in the compiler's per-edge passes. The #1384 verifier listed the ID compares that remain in the compile path (2026-10-22).

## Problem

After #1384, these still compare `GraphNodeId`, `GraphEdgeId` or `GraphPortId` values on every compile:

- lowering: `edge_delay`'s `BTreeMap<&GraphEdgeId, u64>` lookup, called per edge several times (`crates/graph/src/program.rs` near `:599-630`);
- lowering: `node_index` on schedule IDs in the level and schedule loops (`program.rs` near `:579`, `:595`); these also validate the `schedule` input;
- `cycle_witnesses`' per-node sort by `GraphEdgeId` (a no-op on compile input, which is already edge-ID sorted);
- `ports_for` sort and dedup of `GraphPortId`s (`crates/graph-compiler/src/ids.rs` near `:73`);
- the `spec.nodes.windows` sortedness check and `route_transforms`' sort by node.

## Smallest closable slice

First measure, then change only what the measurement names. Run the #1384 gate 6 profile (needs `perf`; see #1384's record) at the #1384 merge and report the inclusive share of each item above. Remove an item only if its share is at least 1 % of preparation; record the rest as measured and left.

- `node_index` in the level/schedule loops: keep the validation of a caller's `schedule`. Passing dense levels from the compiler would change the public `lower` signature, which needs a decision in this spec before it is done.

## Authorized paths

- `crates/graph/src/program.rs`, `crates/graph-compiler/src/` (the items above only)
- this spec

## Objective gates

1. The profile before and after, attached (descriptive).
2. No bit moved: #1384 gate 4's four oracle digests are equal (PR evidence).
3. `cargo test --locked -p graph-compiler -p graph --features graph/test-support`, fmt, clippy, `check-graph-policy.sh`.

## Dependencies

- #1384 merged.
- A machine where `perf record` runs (`kernel.perf_event_paranoid` at most 2).

## Decision and evidence record (attempt 1, 2026-10-24)

- **R0 (root): no `perf`.** `perf record` is refused here (`kernel.perf_event_paranoid` = 4) and
  root refused `sudo perf`. The shares below are **wall-clock shares, not a profile**: a
  scratch-only instrumented build (never committed) times each listed site with `Instant` and
  divides by whole-boot time. Workload frozen first: `AudioWorkletEngineHost::boot` of
  `fixtures/session/v1/console-sixty-four-track-sends.json`, 50 boots per round, release, pinned
  to CPU 31, under the exclusive heavy lock; one invocation, one warmup round, two measured rounds,
  no retry. Load average about 10 to 12 (other streams' builds wait on the lock, but the box is
  shared). `edge_delay` was timed by replaying its lookups once per lower (an upper bound).
- **Before (`068742aba`), mean of the two rounds, per boot 25.14 ms:**

  | site | calls/boot | us/boot | share |
  |---|---|---|---|
  | `lower` whole | 3 | 2547 | 10.13 % |
  | `node_index`, level and schedule loops | 3 | 761 | 3.03 % |
  | `node_index`, edge loop | 3 | 843 | 3.36 % |
  | `spec.nodes.windows` check | 3 | 24 | 0.10 % |
  | `edge_delay` map build + lookups | 3 | 10 | 0.04 % |
  | `cycle_witnesses` edge-ID sorts | 1 | 5 | 0.02 % |
  | `ports_for` (sort + dedup alone) | 1 | 87 (29) | 0.34 % (0.12 %) |
  | `route_transforms` sort | 1 | 2 | 0.01 % |
  | `NodeTable::new` (#1384 D1, by design) | 1 | 297 | 1.18 % |

- **Changed: `node_index` only** (6.4 %). Every other site is below 1 % and is measured and left:
  R2 (`edge_delay`), R3 (`cycle_witnesses` sorts), R4 (`ports_for`) and R5 are not done.
  `NodeTable::new` is #1384 D1's one sort and resolution per compile and stays.
- **R1 as built.** `program::lower` keeps its ID-taking signature and all its validation. The
  schedule loop's second `node_index` pass is folded into the level loop (each level node is the
  schedule node at the same cursor, so its index also gives the schedule position). Each schedule
  node and edge end is then resolved through a `program::LoweringHint` (the compiler's `NodeTable`
  positions of its dense schedule and edge ends): a hint is taken only after an equality check
  against the ID, otherwise the binary search runs, and `spec.nodes` is checked strictly sorted
  first, so an accepted hint is the index the search returns. Output and errors are therefore
  `lower`'s for any hint. The plan keeps the hint (new `PreparedGraphPlan::new_hinted`; `new` is
  `new_hinted` with an empty hint), so construction, `attach_builtin_banks` and bind-time
  re-derivation all use it; because the hint is checked, a caller who edits the public schedule or
  spec between binds gets the same lowering as before. Retained cost: one `u32` per node and two
  per edge per plan, control-side, not in `graph_metadata_bytes` (whose formula also leaves out the
  plan's `program`).
- **After (`e54b112dc`), same workload, per boot 23.48 ms (-6.6 %):** `lower` whole 1125 us
  (4.79 %); level and schedule loops 54 us (0.23 %); edge loop 134 us (0.57 %); the other sites
  unchanged within noise.
- **Gate 2 (no bit moved; PR evidence).** `sdk_render_oracle 64 < F`, base `068742aba` and branch
  `e54b112dc`, built in separate scratch target dirs: equal digests for all four:
  `parametric-eq-nine-track.json` `2938bb78...f1f5c8`, `console-sixty-four-track.json`
  `fa320199...03e9244c`, `console-sixty-four-track-app.json` `ac23bd04...888430cf`,
  `console-sixty-four-track-sends.json` `bcf300dd...5d94768`. (Two differ from #1384's record
  because `main` moved since; base equals branch.)
- **No new test**, as scoped: the change is a representation change under the existing `topo`,
  lowering and compile-shape tests, and the gate-2 digests.
