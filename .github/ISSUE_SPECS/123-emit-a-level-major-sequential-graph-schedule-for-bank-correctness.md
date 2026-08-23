# 123 Emit a level-major sequential graph schedule for bank correctness

## Outcome and readiness

Make the compiler's canonical sequential graph schedule exactly the concatenation of its accepted,
sorted dependency levels so a retained homogeneous bank never gathers another member's input
before that input has been produced in the current block.

**COMPLETE / SOL XHIGH PASS.** The user directed execution of standing audit controller Issue 125
after the terminal verdict recorded below. AGENTS.md requires a failed shape to be rescoped and
rebriefed before work restarts, while Issue 125 requires Issue 123 to finish in place rather than
creating an attempt issue. Sol XHigh approved this synchronized narrower restart and returned
strict PASS on its first fresh Sol High implementation attempt on 2026-08-23; no correction was
consumed.

The old 100,000-render audit remains consumed: invocation count `1`, retry count `0`. It is not an
acceptance gate, may not be rerun, and neither its `0xf8ee_8fef_8f42_3df4` candidate nor the stopped
4,096-block periodic-output model is authority. Benchmark and timed-benchmark counts remain zero.
The stopped checkpoint `34d0e825d8d470ce499f423276a1e28c3e19f991` is technical input only and
must not be resurrected wholesale.

## Authoritative 2026-08-23 rescope

This section supersedes the earlier attempt shape and acceptance/evidence requirements wherever
they conflict. It applies the newer Issue-123 audit-link comment and Issue-125 Step-0 instruction
to the #98 plan's wave-0 E1/E2 gates and #99's native-oracle transcript procedure:

1. Emit `sequential_schedule` as the exact concatenation of the accepted, sorted dependency
   levels and recompute deterministic buffer coloring from that schedule. Preserve every accepted
   Issue-122 level byte.
2. Keep one private scalar/native bind-time invariant that rejects an ID-ordered or otherwise
   bank-predecessor-invalid schedule transactionally, after existing binding/source/observer
   validation and before executor/scheduler construction.
3. E1 is a hand-built four-track identity builtin bank with distinct constant inputs. Exact lane
   observers and analytic sums must hold from block zero across three blocks for sequential and
   native `SingleThread`; the ID-ordered shape must reject with returned ownership reusable. The
   effective red mutation removes the bank-input invariant so that the invalid shape binds and
   exposes the auditor's `1/11/21` failure.
4. E2 compiles the production selected-width builtin bank, renders three blocks through separately
   prepared sequential and native `SingleThread` plans, and requires bit-identical PCM with a
   nonzero block zero. Reverting compiler emission to Kahn/ID order must expose zero or stale later
   lanes.
5. The existing Issue-037 100-layout test must prepare an independent native plan for every layout.
   Sequential PCM and qualification counters must equal that native oracle before the transcript
   is folded. Re-pin `0xc85b_2209_8007_7824` only from those native-equal results and record the
   old/new literal, 100/100 equality, selected host dispatch and #98 F1 reason.
6. Preserve the exact pinned graph-fixture exception, allowed-path fence, canonical identity,
   coloring/liveness proof, transactional corruption/retry coverage, focused package tests,
   warning-denied Clippy/rustdoc, formatting, graph/realtime policies and CI.

The #98 plan originally assigned the Issue-037 transcript re-pin to #99. Issue 125 and the newer
Issue-123 audit-link comment specifically assign it to this closure, which is the authority for
this rescope. Issue 123 may close only after Sol XHigh PASS, an upstream evidence commit, and green
CI. Passing closes #98 F1 only; #98 and #99 wave-3 work remains open.

## Accepted authorities and technical input

Accepted graph product authority comes from **Deterministic graph compiler, sends, submixes,
sidechains, and PDC** (Issue 006): implementation checkpoint
`40f0a2f3f5057e725e80715da18afb0e5f4d6bb3` (tree
`d8898cde03e4a7d12314e707369f67551607ea3b`) and accepted product rescope
`e1211bba07d680a0a97dcfccc87ce0a167dbca50` (tree
`f31db1e03beed6c1b2b16f77a4c7093ae2338d18`). Preserve its graph meaning, stable node and edge
identities, reductions, exact integer PDC, resource accounting, transactional ownership and
no-track-ceiling contract. Its exhausted benchmark workflow is closed history and is not rerun.

Accepted native execution authority comes from **Native graph scheduler qualification and
benchmark** (Issue 039): sealed candidate
`290037ccebc64204a743cd13f93e240a84f93040` (tree
`cb732a6def7516e8dac71f7f745df76ba321b028`) and final evidence commit
`157b3eae11d500a6d1bdc4cea37a36827461b8ac` (tree
`1caa1873fbe38674dd751ed38dde35479a86ca40`). Preserve its immutable dependency waves,
indivisible-bank units, stable reduction/observation order, scheduler selection, ownership and
realtime contracts. No scheduler qualification or benchmark may be repeated.

Accepted dependency-level authority comes from **Emit sorted graph dependency levels for valid
native binding** (Issue 122): technical checkpoint
`776e2cbbc7d68fd7ac3dc95825dfe99651df5be1` (tree
`f032193af3ff11499003c6bc91e71dcb828acc07`) and final evidence commit
`f9945f07c61b446e43ac00f379497536147930f9` (tree
`55895589671c6213d38f5d2c58e7105070812163`). Its strictly increasing, nonempty dependency levels,
strictly sorted member lists, node-once membership and source-level-before-destination-level edge
law are frozen. Issue 123 changes the sequential schedule to consume that accepted ordering; it
must not reconstruct or reinterpret the levels.

Open **098 Audit: miso-engine-graph (executor correctness bug, reductions, PDC)** is technical
input only. Finding F1, recorded against
`ae02d2abd9bd5e3e97b33152cfc943013325045e` (tree
`8fa639d0212171570e790bc2626bb622370a3fca`), shows that the old Kahn ready-pop schedule can visit
one track's bank member before another member's input node. The sequential executor fires the bank
at its first member, so later lanes can read zero, stale or recolored-buffer data. The native
dependency-wave executor is already level-correct. Issue 098 supplies no implementation authority
or overall PASS; findings F2–F13 remain open and out of scope.

The clean briefing baseline is `main`
`f9945f07c61b446e43ac00f379497536147930f9`, tree
`55895589671c6213d38f5d2c58e7105070812163`.

## Smallest closable correction

In the graph compiler, keep the existing topological level calculation and the Issue-122 sorted
`DependencyLevel.nodes` lists. Set `sequential_schedule` to the exact, allocation-bounded
concatenation of those level-member lists in increasing level order. Every report, prepared graph,
canonical byte stream and downstream consumer must receive that one schedule; no second schedule
or executor-local reorder may exist.

Recompute deterministic liveness buffer coloring from the new schedule. Preserve the existing
smallest-free-index and identity-boundary alias rules, but prove every assigned buffer remains live
through its last consumer under the level-major order, bank-member outputs remain distinct while a
bank is active, and fan-out is not overwritten early. Resource estimates must derive from the
recomputed assignments without changing their public meaning.

Add one shared, non-public prepared-plan structural validator used before either scalar
`bind`/`bind_with_source_set` construction or native `bind_native`/
`bind_native_with_source_set` preparation. It must require:

- `sequential_schedule` is byte-for-byte equal to the flattened dependency-level members;
- the schedule and levels contain every graph node exactly once, with increasing levels, sorted
  nonempty members and every edge strictly crossing from a lower to a higher level; and
- for every effect or builtin bank, all members occupy one dependency level and every incoming
  graph-edge source for every member precedes the bank's first scheduled member.

A non-level-major schedule or bank-predecessor-invalid plan rejects with the existing
`graph.scheduler.layout` code after the current binding/source/observer checks but before
executor/scheduler construction, preserving existing error precedence for otherwise invalid
inputs. Failure is transactional: the scalar and native families return the original prepared
graph, every runtime binding, optional source set, and native configuration exactly once and
reusable. Do not add a public validator, change public result shapes or copy validation semantics
into separate scalar/native helpers.

## Required bank-correctness evidence

Add an exact four-track W4 builtin-bank regression with distinct constant dual-mono inputs. Its
production-shaped graph must put all four input predecessors before the four
`PostInputBuiltins` members in the new schedule. From block zero, the banked result must equal an
independent analytic sum and a scalar no-bank reference. Pin block-zero lanes as left
`[1,2,3,4]`, right `[-1,-2,-4,-8]`, with sums `10/-15`; pin the continuation lanes as left
`[2,4,6,8]`, right `[-2,-4,-8,-16]`, with sums `20/-30`. These exact representable values make
zero-first-block and one-block-stale mutations observable.

Exercise independently prepared ownership instances of the same canonical sealed bank shape
through the ordinary sequential graph bind and the native `SingleThread` bind. Native
`SingleThread` must report its accepted sequential fallback selection; both paths must produce
bit-identical PCM, PDC, counters and observer order. This is not authority to rewrite the native
executor: its execution remains dependency-wave based even when the scheduler selects its
single-thread fallback.

Retain compiler-driven bank evidence through the existing mixed 12-track production compilation.
Prove the compiler creates the expected full bank(s) plus scalar tail for the actual selected
backend, while the existing off-render factory probes retain exact W4 and W8 membership/tail
coverage without pretending either was runtime-selected. The compiled schedule equals flattened
levels, and banked sequential output equals both the scalar compiled reference and a separately
prepared native reference. Re-derive every changed mixed-output hash from the analytic/native
oracle on the corrected candidate; do not copy the new sequential result into an expected literal,
use the old defective result as authority, or weaken the exact hash assertion. The existing
100,000-render functional audit may be invoked at most once only if it is required to replace its
schedule-dependent frozen hash; it remains non-timed and is not a benchmark or optimization
workload.

## Pinned pre-existing graph-fixture exception

Issue 122 accepted a pre-existing, out-of-scope graph-fixture mismatch. The exact command

```sh
cargo run --quiet --locked -p miso-engine-graph-compiler --bin miso_engine_graph_fixture -- --check fixtures/graph
```

exits nonzero with sole output `graph fixture manifest mismatch`. Only these checked/generated
identities differ:

| Path | Checked bytes / SHA-256 | Generated bytes / SHA-256 |
| --- | --- | --- |
| `v1/direct-route.canonical.txt` | `3726` / `7ae045dceca0490f4607817a2a44739492cd4a3cf68718f11a865571871ea9bb` | `3734` / `40bd3d4c126bf3cc8aa1730ebdda12371ffca4ecba2d2b1c94da3f1e9b0579e3` |
| `v1/direct-route.report.json` | `331` / `c45d1065a90ab157100b36458ec6393f4c1ea63d974683203519f5516e2448c0` | `331` / `d2546a263146537b5da0786e7c793977912a4eb70bc2c858785ad4d0776c948c` |

The canonical delta is exactly five inserted zero resource-estimate fields, or eight bytes; the
report changes only the embedded canonical length/hash. Removing the `estimate` row yields common
SHA-256 `2683c125be905b87170172715956589e998f374bc2d15f1c2b17b4f1181d50e5`.
Every dependency-level and sequential-schedule row is byte-identical. The other five fixture
payloads remain byte-identical at their exact baseline identities:

- `direct-route.dot`: `1359` bytes,
  `a5febba237458a01737653eeb4221634a3b827e4e5661fee44a4c28e2abe0499`;
- `direct-route.resources.json`: `350` bytes,
  `2ed780ba9e3a90a4b38ba241ae6dcc287dd9566d7fe144d6a5101ec765dc2b80`;
- `invalid-scc-diagnostics.json`: `296` bytes,
  `9b43dbd8d62935e3eb1d96e39c0114b0dfd0bec91d84a6a786cdc0902a4b4600`;
- `main-sidechain-pdc.csv`: `358` bytes,
  `c08654d06220ecec7e0730298725392bba3afeb2b4779fc20c597d003bfead29`;
- `summation-residuals.json`: `859` bytes,
  `ee39298ed192f19af249c1e6b550618851dc3af31b2619413c5d9f5293348b2f`.

There are no new or missing fixture paths.

The direct-route fixture has only singleton dependency levels, so the Issue-123 schedule change
must not change any checked or generated fixture byte. Do not edit, regenerate or bless
`fixtures/graph/**`, its manifest or the generator. The broad check remains an expected baseline
observation and is acceptable only if it reproduces the exact mismatch above with no additional,
missing or changed difference.

## Frozen boundaries

Preserve Issue-122 dependency levels byte-for-byte for identical graphs. Preserve graph node,
port, edge, stage and bank membership meanings; stable IDs; reduction and observer order; exact
integer PDC and latency/tail laws; source ownership; canonical grammar; resource-report field
meanings; allocation-free render; and every public interface.

Do not alter `GraphExecutor` or `NativeGraphExecutor` render semantics, bank trigger behavior,
native wave/unit/partition construction, dependency-wave scheduling, worker protocol, scheduler
selection, core plan lifetime, builtins/effect kernels, source handling, protocol, C ABI, host or
runner behavior. The corrected schedule and buffer assignments are compiler outputs, and the
shared bind-time validation is only a preconstruction guard.

Issue 098 findings F2–F13 are deferred: no reduction rewrite, delay/PDC optimization, route
arithmetic change, sanitization change, AoSoA layout redesign, executor unification, native staging
or worker-lifetime change, allocation-arena redesign, telemetry cleanup or test-scaffold cleanup.

## Allowed tracked paths

- `crates/miso-engine-graph-compiler/src/lib.rs` and its focused existing tests;
- `crates/miso-engine-graph/src/lib.rs` and its focused existing tests;
- this issue spec and its tracked brief; and
- minimal Issue-123 routing in `.github/ISSUE_SPECS/README.md`,
  `docs/IMPLEMENTATION_PLAN.md` and the Issue-026 dependency list.

Any edit to graph fixtures/generator, `crates/miso-engine-native-scheduler/**`, Cargo manifests or
lockfile, accepted qualification/benchmark artifacts, core, builtins/effect production code,
another runtime crate or another issue's evidence is STOP and requires a new or amended issue.

## Dependencies by exact title

- **Deterministic graph compiler, sends, submixes, sidechains, and PDC**
- **Native graph scheduler qualification and benchmark**
- **Emit sorted graph dependency levels for valid native binding**

Issue 123 gates **End-to-end release, performance, and listening qualification** after its accepted
dependencies. It does not close Issue 098 or depend on any of its F2–F13 work.

## Acceptance gates

1. Every fresh compiler result satisfies
   `sequential_schedule == dependency_levels.flat_map(nodes)` exactly. Levels retain Issue-122
   order and membership, every node occurs exactly once in both forms, and every edge crosses
   forward by level and schedule position. Repeated fresh compiles emit one schedule, buffer map,
   canonical byte sequence and SHA-256.
2. Buffer coloring is recomputed from the level-major schedule and passes independent interval
   reconstruction, fan-out, identity-alias and simultaneous-bank-member liveness checks. Removing
   recoloring, reusing the old schedule's assignments or forcing an early reuse fails.
3. Scalar and native binding families reject schedule/level mismatch, duplicate/omitted nodes,
   reversed members, same/backward-level edges, mixed-level banks and a predecessor at/after the
   bank's first member before constructing an executor or scheduler. Each failure returns all
   plan/binding/source/config ownership and the corrected inputs bind successfully afterward.
4. The four-track distinct-constant W4 builtin bank produces the analytic left/right PCM from
   block zero and on a continuation block through sequential bind and native `SingleThread`.
   Banked/scalar/native PCM, PDC, observer order and exact bank counters match. Zero-lane,
   prior-block-lane, first-member-only and trigger-at-last-member mutations fail.
5. The production compiler's mixed 12-track selected-backend bank-plus-tail case has a level-major
   schedule and matches its scalar and native oracle; off-render W4/W8 membership probes retain
   their exact full-bank/tail counts. Every schedule-dependent hash is independently rederived and
   pinned; stale defective hashes and copied-self expected values fail.
6. Direct-route fixture bytes remain untouched and the exact pre-existing resource-zero mismatch
   reproduces with no new/missing/changed difference. Focused canonical corruption proves a
   schedule reorder changes identity and cannot be silently accepted.
7. Focused graph and graph-compiler tests, locked package tests, warning-denied Clippy/rustdoc,
   format, graph/realtime policies and applicable mutations pass. Exact diff/static scans prove the
   allowed fence, frozen executors/scheduler/fixtures and absence of generated artifacts.
8. Sol High freezes one exact-path checkpoint with command, hash, mutation and ownership evidence.
   Sol XHigh returns strict PASS or the sole bounded HOLD. After a HOLD, the correction is terminal
   PASS or STOP; gates may not be weakened.

## Target matrix and execution budget

Run focused Linux native correctness only. The compiler change is target-neutral and both runtime
paths are exercised on the native host; existing Wasm and device compile authority remains with
accepted Issues 006/039. No target matrix, browser/device run, benchmark or timing round is needed.

If the existing mixed 12-track schedule-dependent 100,000-render hash must be replaced, authorize
exactly one non-timed functional audit invocation after all focused gates are green. Record whether
it ran and its old/new/oracle hashes; retry and tuning are forbidden. Otherwise its invocation
count remains zero. In all cases record `benchmark_invocations=0` and
`timed_benchmark_invocations=0`.

## Required evidence

Record the clean candidate/tree and exact changed-path hashes; accepted authority hashes; old Kahn
schedule versus flattened level-major transcript; dependency-level equality and node/edge
invariants; old/new buffer assignments with independent liveness proof; transactional corruption
and reuse outcomes for scalar/native and source/no-source forms; exact four-track topology,
constants, block-zero/continuation PCM/PDC/counters/observers; native scheduler selection; mixed
12-track W4/W8 bank/tail counts and analytic/scalar/native hashes; effective mutation results;
pinned fixture exception reproduction; strict gates; exact functional-audit count; zero
benchmark/timing counters; and Sol High/Sol XHigh verdicts.

## Explicit non-goals

Issue-098 F2–F13; executor unification or render-loop edits; native dependency-wave changes;
buffer-layout optimization; new public APIs; new effects/builtins/source/control behavior; fixture
regeneration; cross-target qualification; benchmark, timing, tuning, listening or performance
claims; and V1/legacy inspection.

## Implementation and review evidence — 2026-08-23

### Sol High pass 1 and sole Sol XHigh HOLD

Pass 1 implemented the F1 product correction within the exact two-product-path fence. The compiler
emits `sequential_schedule` by concatenating Issue-122's sorted dependency levels and recomputes
buffer coloring from that schedule. The graph crate adds one private structural-layout predicate
shared by scalar/native and source/no-source binding families after existing binding validation and
before executor or scheduler construction. Corrupt schedule, level, node, edge and bank layouts
return `graph.scheduler.layout` transactionally; corrected returned ownership binds successfully.
No render loop, native wave/unit/partition, scheduler, fixture, Cargo or Issue-098 F2–F13 behavior
changed.

Focused evidence passed the level-major schedule/canonical identity, independent liveness interval,
old-color rejection, bank-color distinctness, all-four-bind ownership retry, selected-backend
12-track bank/scalar/native one-block hash `47633fd9831d49c3`, W4/W8 off-render membership, strict
package checks, policies and mutations. The reverse-route canonical SHA-256 became
`464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5`
solely from its authorized sequential-schedule change; dependency levels stayed unchanged.

The one authorized environment-gated 100,000-render functional audit ran once and was not rerun.
It observed corrected sequential hash `f8ee8fef8f423df4`, replacing stale Kahn-schedule hash
`9f30db0220656d79`, with its existing allocation and bank-call checks green. Sol XHigh nevertheless
issued the sole bounded HOLD because that changed long-corpus hash had no independent like-for-like
analytic/native oracle: the independent scalar/bank/native evidence covered only the one-block hash,
and comparing that one-block value to the old 100,000-block value was not an effective stale-hash
mutation. The forced-W4 row also lacked the required observer-order and explicit PDC/latency
assertions.

### Sole correction and terminal Sol XHigh STOP

The correction closed the W4 finding. Independently prepared scalar, banked sequential and native
`SingleThread` plans now prove the exact two-block PCM `[10,-15,20,-30]`, exact bank counters
`[2,2]`, native sequential-fallback selection, empty inserted-delay sets, zero latency on every
node, and eight exact observer calls per plan. Each observer checks lane audio and the complete
stable order 0–3 at block zero then 4–7 on the continuation block.

The correction did not close the 100,000-block oracle gate. A separately prepared native
`SingleThread` graph renders only a 4,096-block prefix, observes that the last 28 PCM-output blocks
repeat three times, then repeats those output bytes synthetically through block 100,000 to obtain
`f8ee8fef8f423df4`. It rejects same-length stale hash `9f30db0220656d79` and a one-bit mutation,
but it compares only output blocks. It does not snapshot or prove equality of all hidden effect,
builtin, delay and accumulator state at the proposed cycle boundaries. Three equal output periods
therefore do not prove that a deterministic stateful graph cannot diverge later. The extended hash
is a modeled extrapolation, not an independent exact same-corpus native or analytic oracle.

The consumed environment-gated audit was correctly not rerun, but its missing independent seal
cannot be repaired by inference or by weakening the gate. Because the sole HOLD correction was
consumed, Sol XHigh returned terminal STOP rather than a second HOLD.

### Frozen technical checkpoint and counters

The preserved technical checkpoint is commit
`34d0e825d8d470ce499f423276a1e28c3e19f991`, tree
`acfad7a8ff12f88e32a9582450bb78f22a419a6a`. Its exact two product paths and SHA-256 identities
are:

- `crates/miso-engine-graph/src/lib.rs`:
  `4dcd1f3fbba12be49b593548ac00494ced9a5a83fc8aa4840909112ba326d956`;
- `crates/miso-engine-graph-compiler/src/lib.rs`:
  `22e2e2a508ad31c4d9389f2ba90787ac2c346d19c1a130b830f485cf1b2a930a`.

The exact binary diff SHA-256 against briefing base
`89274a17a441cf8d255058058a4e83e7cef82692` is
`8b49c123bfe38cd402abd50b127aed3965e75259a22de24c7ce53555bebff1a4`.
The pinned graph-fixture command still exits 1 with sole output
`graph fixture manifest mismatch`, exactly the two accepted resource-zero-derived identities and
no new, missing or changed path; fixtures, generator and manifest remain untouched.

Final execution counters are exactly:

- `environment_gated_100000_render_audit_invocations=1`;
- `environment_gated_100000_render_audit_retries=0`;
- `benchmark_invocations=0`;
- `timed_benchmark_invocations=0`.

Issue 123 has **no overall PASS**. Commit
`34d0e825d8d470ce499f423276a1e28c3e19f991` is technical input only and does not unblock Issue 026
or authorize a retry, another correction, a second 100,000-render audit, timing, benchmark or
performance claim.

## Rescoped implementation and final Sol XHigh evidence — 2026-08-23

The authoritative rescope above restarted the workflow with a different proof shape. Sol High
implemented it at `494f4fe91ed1b9d5acf25426dc05543d386a7d61` (tree
`ddb902be388453e55561b7f25d002d9ee1028004`) and Sol XHigh returned strict PASS on attempt 1.
The stopped checkpoint remains historical technical input only; no stopped long-corpus inference
was reused.

The exact implementation paths and SHA-256 identities are:

- `crates/miso-engine-graph/src/lib.rs`:
  `8b3672c4ddbd733b66e1c7d47c3d11b0ff5c13ff3a093679239aa0e1c0c4aa89`;
- `crates/miso-engine-graph-compiler/src/lib.rs`:
  `0aeba99e09b841986108d6cb3fbec096946dc129d5aeeea60c79098b8e72be78`.

Their exact binary diff SHA-256 is
`86f4283f539127837bc569da0396080826e06f0669ea4a5372ff9ba7a83d6230`.
No fixture, manifest, Cargo file, native scheduler, render loop or unrelated crate changed.

The compiler now emits the sequential schedule by flattening the accepted sorted levels and
recolors from that exact schedule. The reverse-route tail changed from Kahn order
`submix:a -> route:z -> submix:z -> route:a` to level-major order
`submix:a -> submix:z -> route:a -> route:z`; accepted Issue-122 level bytes did not change. Its
canonical SHA-256 changed from
`3e5c3e43fc220ec91eb159d18749bec44fd96fba3f6ef908850c850d995582ce` to
`464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5` for the authorized schedule
identity change. Independent interval reconstruction, identity aliasing, fan-out liveness and
simultaneous bank-color distinction all passed.

One private structural validator is shared by scalar/native and source/no-source binding after
binding/source/observer validation and before executor or scheduler construction. It rejects
schedule, node, level, edge and bank corruption with `graph.scheduler.layout`, returns all
ownership transactionally and permits a corrected retry.

E1 produced exact analytic PCM `10/-15`, `20/-30`, `30/-45` across three blocks through scalar,
banked sequential and native `SingleThread` plans. It observed every lane in stable order 12 times
per plan, retained zero node latency and no inserted PDC, reported scalar counters `[0,0]` and
banked/native counters `[3,3]`, and selected `Sequential(SingleThread)` natively. Removing the guard
exposed the auditor's exact stale left transcript `[1,11,21]`.

E2 used the production-selected X86Avx2Fma W8 bank plus four-track scalar tail. Separately prepared
sequential/native plans produced bit-identical nonzero PCM for all three blocks with matching
counters and distinct simultaneous bank colors.

The Issue-037 test independently prepared and rendered native `SingleThread` execution for every
one of the 100 layouts. Sequential PCM and counters matched the native values before each fold.
Only then was the transcript re-pinned from `0xc85b220980077824` to
`0x4965aa764307e393`. Restoring the stale literal failed with those exact values; flipping one
native-oracle bit failed the pre-fold equality.

Other executed and reverted red mutations restored ready-pop/Kahn emission, reversed the coloring
input, processed only the first bank member and triggered at the last member. They respectively
failed schedule equality, independent assignment reconstruction, observer audio and block-zero
lane audio. No mutation remains.

PASS commands covered locked graph and graph-compiler checks/tests (15 graph tests, 26 compiler
tests, binaries, doctests and the 65,537-track scale test), all-target/all-feature warning-denied
Clippy, all-feature warning-denied rustdoc, formatting, graph/realtime/workspace policies and the
realtime mutation suite. Integration with disjoint accepted Issue-94 checkpoint `97e1a03` passed
the focused multiband graph-compiler gate.

The graph fixture command still exits 1 with sole output `graph fixture manifest mismatch`; all
seven pinned fixture paths/hashes and the accepted checked/generated exception remain exact. The
historical 100,000-render audit was not invoked in the fresh workflow: total invocation count stays
`1`, retry count `0`; benchmark and timed-benchmark counts stay zero.

This PASS closes Issue-098 F1 only. Issue-098 F2–F13 and Issue-099 wave 3 remain open. GitHub Issue
123 closes after this evidence is upstream on `main` and the resulting CI run is green.
