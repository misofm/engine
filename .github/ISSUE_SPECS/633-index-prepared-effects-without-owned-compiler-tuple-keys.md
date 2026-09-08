# Index prepared effects without owned compiler tuple keys

GitHub: https://github.com/misofm/engine/issues/633

Parent: #560 CP1. Coordination: #559. Baseline: delivered main `d98646db47bc603c32431d999cd08f43a0168043`.

CP1 observes that the graph compiler repeatedly carries heap-owned string identity before the graph lowering boundary. The audit's historical “15–20k allocations per 64-track compile” is not current evidence and is not an acceptance claim. Read-only inspection at the baseline finds one bounded product slice in the prepared-effect handoff: `compile.rs` builds owned `(String, RackId, String)` prepared/declared maps and recreates those tuples for lookups; `ids.rs::into_effects` clones the same tuple strings again; `banks.rs` performs another owned tuple lookup after borrowed entry selection.

This issue owns the smallest useful compiler-private correction. It replaces those owned tuple handoffs with an index-aligned prepared-effect identity representation and borrowed validation lookups. It preserves the public `StableGraphId`/`GraphNodeId` model, canonical graph bytes, deterministic ordering and diagnostics. It does not claim the whole CP1 front half: schedule, cycle, PDC, reduction and buffer passes still use owned graph identities and require a separate architecture slice.

Sol HIGH coordinates the stateless record, GitHub, checkpoints and delivery. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, evidence, exact-head, CI and delivery review. Documentation-only lane-A #632 and this issue occupy the two active slots with disjoint ownership; a qualification successor waits until #632 releases its slot.

## Smallest closable product slice

Within `crates/graph-compiler/src/compile.rs`, `ids.rs` and `banks.rs` only:

- assign each deterministically sorted prepared effect a compiler-private typed/index identity;
- retain borrowed `(track, rack, effect)` views only where validation must resolve session declarations;
- carry effect-node identity in index-aligned storage into `into_effects` and bank attachment;
- remove owned tuple construction and string cloning from those handoffs;
- keep all public graph identities and the final owned graph plan unchanged.

Narrow unit/integration tests may change only inside `crates/graph-compiler/src/` and existing `crates/graph-compiler/tests/`. No `crates/graph/**`, session/schema, effect package/contract, runtime, host, SDK, artifact, pin, matrix/result, dependency, manifest, lock, workflow, generic interner, or public API edit belongs here. Do not combine CP2/CP3/CP5 or refactor scheduling/PDC/cycle/buffer logic.

Preserve lexicographic node/diagnostic order, session slot order, duplicate/missing/unexpected-effect precedence and paths, sidechain port identity, bank program order/membership, required bindings, graph semantic SHA/canonical bytes, and scalar/bank lowering behavior. Invalid input must still fail transactionally without leaking a partial plan.

## Objective gates and successor boundary

Before implementation, Astra LOW must verify current applicability, exact path scope and a direct wrong-result control that would detect index/key misassociation. One Luna implementation attempt must provide focused fixtures covering reordered declarations, repeated effect names on different tracks/racks, missing/unexpected entries, routed sidechains, heterogeneous and homogeneous banks, and deterministic repeated compilation. Existing graph-compiler deterministic DAG, reverse-route, multi-slot bank, lowering, scale, track-delay and route-gain coverage remains authoritative where applicable.

Run focused graph-compiler tests in debug and release, full graph-compiler tests, strict affected Clippy, rustfmt, diff hygiene, workspace and relevant graph/compiler policy gates. Compare accepted representative plans before/after at the semantic SHA/canonical graph and diagnostic sequence boundaries. No benchmark or timing is allowed.

There is no current graph-compiler transient allocation counter; `GraphResourceEstimate` measures retained storage and render allocator tests do not measure compilation. After source PASS, create a separately numbered qualification/tooling successor with a fixed representative compile corpus and scoped counting allocator. That successor must measure the removed handoff allocations without introducing a generic benchmark framework, and it must preserve raw count/status/source identities. This issue remains open until that qualification and ordinary exact-head/current-main review, required PR qualification, guarded merge, post-main qualification and GitHub synchronization are complete.

CP1 remains open after this bounded delivery for its schedule/PDC/front-half identity remainder. Record the exact residual rather than extending this issue.
