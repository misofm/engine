# Sol implementation brief — issue 122 sorted graph dependency levels

## Decision

**READY FOR SOL HIGH PASS 1.** Implement only the deterministic ordering correction required for a
valid compiled graph to satisfy the accepted native binder. Sol High implements and freezes one
checkpoint; Sol XHigh performs the read-only adversarial review. One bounded HOLD correction is
the entire remaining budget, and a second material HOLD is terminal STOP. Benchmark, timing and
workload counters remain zero.

Accepted Issue-006 authority is implementation checkpoint
`40f0a2f3f5057e725e80715da18afb0e5f4d6bb3` plus accepted product rescope
`e1211bba07d680a0a97dcfccc87ce0a167dbca50`. Accepted Issue-039 authority is sealed candidate
`290037ccebc64204a743cd13f93e240a84f93040` plus final PASS evidence
`157b3eae11d500a6d1bdc4cea37a36827461b8ac`. Open audit Issue 099 finding F1 at
`ae02d2abd9bd5e3e97b33152cfc943013325045e` is technical input only.

## Exact implementation

In `crates/miso-engine-graph-compiler/src/lib.rs`, keep the existing Kahn topological schedule and
node-level calculation. After levels are known, reconstruct each `DependencyLevel.nodes` list from
the already sorted graph node IDs or sort the accumulated members. Require strict ascending
`GraphNodeId` order, with the same level assigned to every node and no duplicate or omitted node.

Add one small production-path regression: a track feeds two parallel submixes, and their downstream
route IDs sort opposite to readiness. Compile without renaming IDs, then bind through native
`SingleThread` and `DependencyWaves`. Both must accept and return exact matching PCM, PDC and stable
observer order. Independently check increasing level numbers, nonempty strictly sorted members,
node-once membership and source-level-before-destination-level for every edge.

Add an effective mutation that restores ready-pop member order or reverses a level and prove the
strict evidence rejects it. Repeated fresh compiles must have one canonical identity.

## Frozen fence

Do not change the sequential schedule, buffer coloring, bank trigger point, executor, native
scheduler, reduction, PDC/latency/tail, resource estimates, diagnostics, canonical grammar or any
public interface. Issue 098's stale bank-lane correction and proposal to make the sequential
schedule level-major are separate work and must not enter this checkpoint.

Production edits are limited to graph-compiler `src/lib.rs`. Existing focused tests may change.
The graph fixture generator and exact affected `fixtures/graph/**` members may change only if a
current checked dependency-level row is genuinely reordered; every such byte must be independently
derived and the manifest updated mechanically. Graph runtime, scheduler, core, Cargo files,
accepted benchmark artifacts and all unrelated crates/fixtures are read-only.

## Required gates and handoff

Before handoff, run only focused/locked graph-compiler correctness tests, checked graph fixtures and
their corruption tests, package warning-denied Clippy/rustdoc, format, graph/realtime policies,
the ordering mutation and exact diff/static/artifact checks. Do not invoke a benchmark, timing
runner, long graph audit, browser/device row or other workload.

Handoff one clean exact-path candidate with hashes; the reverse-ID topology; ordered level
transcript; node/edge invariant results; before/after sequential-schedule and canonical identities;
native single-thread/dependency-wave PCM/PDC/observer identities; fixture changes if any; mutation
results; and exact zero benchmark/timing/workload counters. Sol XHigh returns PASS or the sole HOLD.

PASS gates **End-to-end release, performance, and listening qualification**. It does not close
Issue 099's other findings or Issue 098.
