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
