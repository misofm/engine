# 123 Emit a level-major sequential graph schedule for bank correctness

## Outcome and readiness

Make the compiler's canonical sequential graph schedule exactly the concatenation of its accepted,
sorted dependency levels so a retained homogeneous bank never gathers another member's input
before that input has been produced in the current block.

**READY FOR SOL HIGH PASS 1.** Sol High implements and freezes one coherent checkpoint; Sol XHigh
performs the read-only adversarial review. The complete budget is one implementation pass plus one
bounded HOLD correction. A second material HOLD is terminal STOP. No benchmark or timing run is
needed or authorized, and no tuning is in scope.

Remote Issue 123 was read-only confirmed unallocated on 2026-08-23. Root owns GitHub creation,
body synchronization and state changes after this docs checkpoint is committed and upstream. This
record authorizes no GitHub mutation.

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
