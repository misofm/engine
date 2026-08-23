# Sol implementation brief — issue 123 level-major sequential graph schedule

## Decision

**COMPLETE / SOL XHIGH PASS.** After the recorded terminal verdict, the user directed
execution of standing controller Issue 125. AGENTS.md and Issue 125 authorize a narrower,
synchronized in-place rescope rather than a disguised fourth retry or a new attempt issue. This
fresh workflow passed its first Sol High implementation attempt at `494f4fe` under Sol XHigh
adversarial review; no HOLD or correction was consumed.

The replacement gates are #98 wave-0 E1/E2 plus the Issue-037 100-layout transcript derived from
independently prepared native execution on every layout. The previous 100,000-render audit remains
consumed (`invocations=1`, `retries=0`) and is removed from acceptance. Do not rerun it, use its
candidate hash as authority, or restore the stopped periodic-output extrapolation. Benchmark and
timed-benchmark counts remain zero.

Implement only the exact flattened-level schedule and matching recoloring, the shared
transactional bank-input bind invariant, the analytic four-track sequential/native regression,
the compiler-selected-width three-block sequential/native bit-parity regression, and the native-
oracle Issue-037 transcript re-pin. Preserve the pinned fixture exception and frozen product
boundaries. The stopped `34d0e825` checkpoint is reference material only, not a cherry-pick target.

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

## Final evidence and verdict

Pass 1 correctly landed the bounded F1 mechanics: compiler schedule equals concatenated sorted
levels, coloring is recomputed, and one private validator guards scalar/native plus
source/no-source binds transactionally before construction. The reverse-route canonical identity
is `464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5`;
Issue-122 level bytes, both executor render loops, native waves/scheduler, fixtures and F2–F13 are
frozen. Focused checks, coloring evidence, ownership retries, one-block 12-track
scalar/bank/native hash `47633fd9831d49c3`, strict package gates, policies and mutations passed.

The sole HOLD identified two evidence defects. First, the changed 100,000-render sequential hash
`f8ee8fef8f423df4` lacked an independent same-corpus oracle; the native/scalar proof covered only
one block, and its comparison to stale 100,000-block hash `9f30db0220656d79` was cross-length.
Second, forced-W4 evidence had no observer bindings and no explicit PDC/latency assertions.

The sole correction closed W4 completely: exact PCM `[10,-15,20,-30]`, bank counters `[2,2]`,
native single-thread fallback, empty inserted delays, all-node zero latency, and exact observer
audio/order 0–7 with count 8 all match across independently prepared scalar, banked and native
plans.

The 100,000-block finding remains. A native plan renders 4,096 blocks, the test observes three
equal 28-block PCM-output periods, and then synthetically repeats those output bytes to hash the
remaining corpus. Output equality does not prove equality of hidden effect, builtin, delay and
accumulator state, so it cannot prove the future output period. Same-length stale and one-bit
mutations reject, but they validate only the extrapolated reference. The one authorized
environment-gated audit was not rerun, as required.

The frozen technical checkpoint is
`34d0e825d8d470ce499f423276a1e28c3e19f991`, tree
`acfad7a8ff12f88e32a9582450bb78f22a419a6a`, with exact product hashes:

- graph: `4dcd1f3fbba12be49b593548ac00494ced9a5a83fc8aa4840909112ba326d956`;
- graph compiler: `22e2e2a508ad31c4d9389f2ba90787ac2c346d19c1a130b830f485cf1b2a930a`.

Its exact binary diff SHA-256 against base `89274a17a441cf8d255058058a4e83e7cef82692`
is `8b49c123bfe38cd402abd50b127aed3965e75259a22de24c7ce53555bebff1a4`.
The fixture check retains exactly its pinned sole mismatch and no fixture byte changed.

Final counters are
`environment_gated_100000_render_audit_invocations=1`,
`environment_gated_100000_render_audit_retries=0`, `benchmark_invocations=0`, and
`timed_benchmark_invocations=0`.

Verdict: **terminal STOP, no overall PASS, no retry.** The technical checkpoint is preserved as
input only and does not unblock Issue 026.

## Fresh-rescope verdict

Sol High candidate `494f4fe91ed1b9d5acf25426dc05543d386a7d61` (tree
`ddb902be388453e55561b7f25d002d9ee1028004`) received strict Sol XHigh PASS on attempt 1. It closes
the replacement gates with exact three-block analytic W4 output, production-selected W8-plus-tail
sequential/native parity and independently prepared native PCM/counter equality on 100/100
Issue-037 layouts before re-pinning `0xc85b220980077824` to `0x4965aa764307e393`.

The stopped periodic extrapolation did not return, the historical long audit was not rerun, all
red mutations failed and were reverted, the fixture exception remains exact, and integration with
accepted Issue-94 commit `97e1a03` is clean. This fresh PASS supersedes only the earlier terminal
verdict for the explicitly rescoped proof; the stopped checkpoint remains technical input only.
