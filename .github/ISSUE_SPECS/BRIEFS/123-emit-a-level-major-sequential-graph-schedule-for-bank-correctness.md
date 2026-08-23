# Sol implementation brief — issue 123 level-major sequential graph schedule

## Decision

**READY FOR SOL HIGH PASS 1.** Implement only Issue-098 F1: make the compiler's sequential schedule
the exact concatenation of accepted sorted dependency levels, recolor buffers against that
schedule, and reject invalid prepared schedules transactionally in both graph binding families.
Sol High freezes one checkpoint and Sol XHigh reviews read-only. One bounded HOLD correction is the
entire remaining budget; a second material HOLD is terminal STOP. No benchmark, timing or tuning is
authorized.

Accepted authorities are Issue 006 product checkpoint/rescope
`40f0a2f3f5057e725e80715da18afb0e5f4d6bb3` /
`e1211bba07d680a0a97dcfccc87ce0a167dbca50`, Issue 039 sealed/final checkpoints
`290037ccebc64204a743cd13f93e240a84f93040` /
`157b3eae11d500a6d1bdc4cea37a36827461b8ac`, and Issue 122 technical/final checkpoints
`776e2cbbc7d68fd7ac3dc95825dfe99651df5be1` /
`f9945f07c61b446e43ac00f379497536147930f9`. Issue-098 F1 at
`ae02d2abd9bd5e3e97b33152cfc943013325045e` is technical input only.

## Exact implementation

In `miso-engine-graph-compiler`, preserve `topo`'s accepted level assignment and Issue-122 member
sorting. Build the one canonical `sequential_schedule` by flattening those levels in increasing
level order, then compute buffer assignments from that schedule. All reports, prepared plans and
canonical serialization consume the same values. Do not keep the old ready-pop schedule for any
consumer.

In `miso-engine-graph`, add one private shared validation path before scalar or native executor
construction. Require the schedule to equal flattened levels exactly; node-once, sorted-level and
forward-edge invariants; same-level bank membership; and every incoming predecessor of every bank
member before the first scheduled member. Apply it to `bind`/`bind_with_source_set` and
`bind_native`/`bind_native_with_source_set`, after existing binding/source/observer validation.
Reject with `graph.scheduler.layout` and return the original plan, bindings, optional source set
and native config exactly once and reusable. Do not expose a validator or duplicate scalar/native
rules.

Recompute coloring under the new schedule and prove intervals independently. Keep deterministic
smallest-free-index reuse and existing identity aliasing, while preventing early fan-out reuse and
retaining distinct simultaneous bank-member outputs.

## Focused evidence

Add a four-track W4 builtin bank with lane-identifying constant dual-mono inputs. Block zero uses
left `[1,2,3,4]`, right `[-1,-2,-4,-8]`, summing to `10/-15`; the continuation block doubles each
lane and sums to `20/-30`. Render independently prepared instances through ordinary sequential
bind and native `SingleThread`; compare exact PCM to an analytic sum and scalar no-bank reference.
Pin PDC, observer ordering, bank counters and native fallback selection. Mutations for zero/stale
lanes, first-only execution and last-member triggering must fail.

Keep the existing mixed 12-track graph compiler test production-driven. Prove selected-backend
full-bank plus scalar-tail membership, flattened schedule identity, and sequential bank PCM parity
with separately prepared scalar and native references. Preserve the existing off-render W4/W8
factory membership probes without representing them as runtime-selected execution. Re-derive
changed output hashes from the analytic/native oracle, not from the corrected sequential output
itself. Stale-hash and copied-self mutations must fail.

The graph fixture corpus is read-only. The singleton-level direct-route fixture cannot change from
this schedule correction. Its accepted pre-existing resource-zero mismatch remains exactly:

- canonical checked/generated: `3726` /
  `7ae045dceca0490f4607817a2a44739492cd4a3cf68718f11a865571871ea9bb` and `3734` /
  `40bd3d4c126bf3cc8aa1730ebdda12371ffca4ecba2d2b1c94da3f1e9b0579e3`;
- report checked/generated: `331` /
  `c45d1065a90ab157100b36458ec6393f4c1ea63d974683203519f5516e2448c0` and `331` /
  `d2546a263146537b5da0786e7c793977912a4eb70bc2c858785ad4d0776c948c`;
- only five inserted zero estimate fields, common non-estimate canonical SHA-256
  `2683c125be905b87170172715956589e998f374bc2d15f1c2b17b4f1181d50e5`, unchanged
  level/schedule and other payload bytes, and no new/missing paths.

The exact fixture check may reproduce only its sole nonzero `graph fixture manifest mismatch`.
Do not regenerate, bless or edit a fixture, generator or manifest.

## Frozen fence

Production edits are limited to:

- `crates/miso-engine-graph-compiler/src/lib.rs`;
- `crates/miso-engine-graph/src/lib.rs`.

Their focused existing tests may change. Freeze Issue-122 dependency-level bytes, graph semantics,
reductions, PDC, observers, public/resource meanings, both executor render loops, bank trigger
behavior, native waves/units/partitions, scheduler selection/protocol, fixtures, Cargo files and
all other crates. Do not absorb Issue-098 F2–F13.

## Required gates and handoff

Pass focused/locked graph and graph-compiler tests; exact schedule/level/node/edge invariants;
independent coloring liveness; scalar/native transactional rejection and reuse for source/no-source
forms; four-track block-zero/continuation bank audio; mixed 12-track W4/W8 scalar/native/hash
parity; effective schedule, predecessor, coloring, stale-audio and stale-hash mutations; the pinned
fixture-only exception; warning-denied Clippy/rustdoc; format; graph/realtime policies; and exact
diff/static/artifact scans.

The existing schedule-dependent 100,000-render mixed audit may run once, non-timed, only if needed
to replace its frozen hash after all focused gates pass. Record its exact invocation count and
old/new/native-oracle identities; no retry. Otherwise record zero. Always record
`benchmark_invocations=0` and `timed_benchmark_invocations=0`.

Handoff one immutable exact-path candidate/tree with all hashes, transcripts, PCM/counter evidence,
mutation outcomes, fixture exception and invocation counters. Sol XHigh returns strict PASS or the
sole bounded HOLD. PASS gates **End-to-end release, performance, and listening qualification**; it
does not close Issue 098 or authorize its remaining findings.
