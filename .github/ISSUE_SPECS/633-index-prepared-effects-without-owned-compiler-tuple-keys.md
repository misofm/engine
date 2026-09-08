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

## Astra LOW scope review — PASS

Astra LOW passed exact clean pushed brief
`f84469fab324f73034414d41a80f3fa5f14b46c4` against delivered main
`d98646db47bc603c32431d999cd08f43a0168043`. The repeated owned tuple
construction remains present in validation and node/sidechain lookup in
`compile.rs`, `ids.rs::into_effects`, and bank attachment in `banks.rs`. #632 is
documentation-only and disjoint; #632/#633 occupy the two active slots.

Luna HIGH/XHIGH attempt 1 may edit production only in
`src/{compile,ids,banks}.rs`. Any `lib.rs` edit must be test-only, and focused
tests must stay in existing graph-compiler modules/files. Prepared-entry order
must remain independent of session traversal and bank slot order; differently
ordered collections may not be silently zipped.

Wrong-result controls must deliberately cross indices between tracks, racks and
slots with distinguishable processor, metadata and control identities, then
assert association-sensitive results. Routed-sidechain destinations and
homogeneous/heterogeneous bank membership/program order are mandatory. Preserve
diagnostic order/paths, transactional failure ownership, canonical graph and
semantic identities, and lowering regressions. No public graph/session/runtime
API, dependency, artifact, scheduling/PDC, timing or allocation claim is allowed.
Allocation measurement remains a separately numbered successor after source PASS.

## Attempt 1 source review — FAIL

Astra LOW reviewed exact clean pushed source
`ff64e8ef4794d1a2bc1db2c20b234687ec3bb8c3`. The production index mapping is
coherent and preserves the prior borrowed-key diagnostic traversal; no product
defect was identified. The attempt fails its explicit causal-test gate.

The association fixture constructs both returned nodes and expected IDs from the
same entries, so it cannot reject a processor/metadata pair attached to the wrong
node. Its metadata/processor comparison has the same blind spot, and control node
IDs do not prove control-channel ownership. It lacks intentional cross-track,
cross-rack and cross-slot swaps plus routed-sidechain and bank association
witnesses through production compilation.

Attempt 2 is limited to those discriminating test controls and any strictly
necessary correction inside the already authorized four paths. Use identities
whose wrong association changes an asserted processor result, metadata/control
ownership, sidechain destination, or bank slot/program order. Preserve the
attempt-1 source and verdict. The reported release `track_delay` command stopped
before tests at a Cargo duplicate `effect-package` output collision; it receives
no credit and must be preserved candidly. Run its corrected focused release leg
with a fresh isolated Cargo target, then complete proportional full-suite, strict
Clippy and policy evidence. No manifest/dependency repair, allocation measurement,
artifact work, timing or public/scheduling change is authorized.

## Attempt 2 source review — FAIL

Astra LOW reviewed exact clean pushed checkpoint
`f9feadd0a7f91ba7af352f81108f8f8d0bfb77bc`. The production mapping remains
coherent and unchanged from attempt 1, and the review identified no production
defect or scope/lock drift. The corrected fixture covers more production paths,
but it still does not satisfy the issue's causal association gate.

Reversing prepared entries is not an intentional wrong-association control. The
PCM legs send the same control through the same implementation, so nonzero audio
does not independently prove which processor received the control. The routed
sidechain assertions prove scalar fallback/bank exclusion without proving the
edge's exact destination and port. The heterogeneous-bank assertions count banks
without proving their member/program association.

The first fresh-target release `track_delay` invocation stopped before tests at
the previously observed duplicate `effect-package` output collision and E0463.
Luna then ran a second manifest-scoped release retry despite the coordinator's
stop-on-first-failure instruction. Both results remain preserved; neither retry
receives acceptance credit. Astra classifies this as a process defect rather
than evidence of a production regression, but it prevents an unconditional
attempt PASS.

Only final attempt 3 may add the missing causal controls and finish the bounded
evidence. Freeze production unless a causal control exposes a defect. The test
must deliberately demonstrate sensitivity to a crossed metadata/processor/control
association, independently distinguish the intended control target, assert the
sidechain destination and port, and assert heterogeneous bank members and program
order. Preserve both failed attempts. Run each authorized gate at most once, stop
at its first failure, and do not perform allocation qualification, artifact work,
timing, manifest/dependency repair, or broader refactoring.
