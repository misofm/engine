# 006 Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Outcome

Compile unlimited dual-mono tracks and buses into a typed acyclic render graph with deterministic routes and exact latency compensation.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement node/port types; the stable input, post-input-builtins, post-SIMD1, post-dynamic, post-SIMD2/pre-fader, post-fader and post-matrix send taps; submix buses; sidechain ports; deterministic topological ordering/reductions; integer-sample PDC; bypass-latency preservation; and latency/tail propagation.

## Required public interfaces/contracts

`GraphSpec` uses stable node/port IDs; `GraphCompiler::compile` returns a canonical sequential schedule plus dependency-level IR and nonnegative `LatencySamples(u64)` per route; every cycle rejects in this issue. A future graph version may legalize feedback only through an explicit positive-latency edge.

## Deliverables

Graph IR, validator/compiler, PDC delay nodes, deterministic ordering rules, graph visualization/debug output, and fixtures.

## Explicit non-goals

Feedback processing, implicit sends, fractional/implicit PDC, track count ceiling, or realtime graph mutation.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Native effect runtime contract and conformance

## Hazards/decisions

Track chain is exactly input→builtins→SIMD1→dynamic→SIMD2→fader/mute→matrix→routes. Sends must name their tap; graph stays acyclic. VST latency reference: https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html.

## Acceptance gates with objective measurements

Same graph compiles to identical semantic order/hash across 100 fresh-process runs; PDC impulses on direct, send, submix and sidechain paths align exactly to the sample, including latency-preserving bypass; every cycle reports its full stable-ID path; invalid taps/ports fail with diagnostics; generated validation/estimation graphs beyond 65,536 tracks fail only for configured resources, never a compiled track ceiling; the frozen deterministic production summation strategy publishes absolute/RMS residual against an independent `f64` reduction on representative and adversarial cancellation fixtures.

## Target matrix

All targets; native dependency waves feed issue 009, browser consumes deterministic sequential order.

## Required evidence

Graph fixtures, order/hash report, impulse/PDC plots, invalid-graph diagnostics, and compile benchmark.

## Terra attempt 1 evidence — 2026-08-20

**Status: FAIL / incomplete; do not advance to Sol review as an accepted implementation.**

Implemented a compiling initial slice only:

- Added `miso-engine-graph` and `miso-engine-graph-compiler` with typed stable graph IDs,
  seven typed track stages, prepared-effect reconciliation, deterministic Kahn schedule/dependency
  levels, scalar balanced summation and exact dual-mono delay primitive, bounded PDC/tail/resource
  accounting, canonical debug bytes/SHA-256/DOT, and a non-publishable binding seam.
- Added the narrow core `PreparedPlanExecutor: Send` ownership seam and
  `PreparedRenderPlan::prepare_with_executor`; existing publication/retirement APIs were not
  changed.
- Passed: `cargo fmt --all`; `cargo check -p miso-engine-graph -p
  miso-engine-graph-compiler`; `cargo test -p miso-engine-graph` (2/2 tests).

Required gates that remain unimplemented or unrun: complete scalar graph execution and real source/
builtin/rack/output bindings; valid/invalid fixture corpus and manifest; independent reference and
10,000-mutation tests; 100-process determinism; all-rate/all-quantum impulse/PDC tests; scale
fixtures; summation residual reports; realtime allocation/swap audit; workspace policies, warning-
denied Clippy/rustdoc, locked workspace, and target builds. The complete Issue #6 acceptance suite
therefore **fails**.

`scripts/run-graph-compiler-benchmark.sh` was **not invoked** because the prerequisite
nonbenchmark gates do not pass. Benchmark invocation count: **0**.

## Sol adversarial review and correction attempt 2 — 2026-08-21

**Status: FAIL / incomplete; a final bounded correction was required.**

Sol found that attempt 1's compiling seam did not execute a graph: bindings were reduced to an ID
set, the executor emitted silence, the requested plan ID was discarded, and reported PDC was not
materialized or processed. Tail propagation, route transforms, sidechain audio, complete binding
ownership, resource accounting, canonical debug semantics, and cycle handling also had ownership
or correctness gaps. Attempt 2 replaced that seam with a scalar graph executor and added exact
per-edge delay processing, fixed pairwise reductions, typed source/builtin/fader/matrix/effect/
output bindings, ownership-returning bind failures, plan-ID/time propagation, deterministic
canonical bytes/SHA/DOT, iterative per-SCC witnesses, indexed lowering, and checked estimates.

Attempt-2 evidence passed direct accepted-session PCM rendering; all eight rates at quanta 1, 127,
128, 255, and 1,024 for enabled/latency-preserving bypass PDC; 10,000 deterministic mutations; 100
fresh-process fingerprints; a 65,537-track release scale compile; summation analytic-bound and 100
completion-order checks; native/Android/iOS/Wasm target builds; and a one-million-block graph render
audit. It remained incomplete because checked fixture regeneration/corruption, typed faster/slower
sidechain audio evidence, liveness coloring, graph-backed swap/destruction evidence, numeric report
files, and the benchmark driver/validator were still missing. The benchmark remained forbidden.
Benchmark invocation count after attempt 2: **0**.

## Sol correction attempt 3 prebenchmark evidence — 2026-08-21

**Status: PASS for every frozen nonbenchmark gate; exactly one benchmark invocation is now
authorized after this evidence is committed. Overall issue acceptance remains pending that result.**

The final bounded correction added deterministic smallest-index liveness coloring used by the
executor and estimates, including identity-boundary aliasing and fan-out lifetime tests; typed
faster-main and faster-sidechain impulse fixtures; exact checked-in canonical/DOT/report/resource/
diagnostic/PDC/summation files with a sorted length/SHA-256 manifest, non-mutating `--check`, and
changed/missing/unlisted corruption tests; and graph-backed plan exchange evidence with two accepted
swaps, one forced retirement-full deferral, and two displaced executor-owning plans destroyed on the
dedicated retirement thread.

The benchmark harness is ready but has not run. `miso-engine-graph-bench` freezes three workloads:
256 tracks/1,024 routes/32 submixes with 64 mixed-latency effects and 32 sidechains; 65,537-track
sparse validation/estimation; and canonical debug/SHA/DOT. Its strict validator requires six JSONL
records, two rounds per workload, stable hashes/counts, ordered nearest-rank percentiles, phase and
memory measurements, environment/optimization metadata with explicit missing disclosure, zero
errors, and descriptive-only/no-threshold status. The exact runner refuses overwrite/retry and
preserves and hashes rejected raw output. Runner/validator mutation tests launched **0** workloads.

Final local nonbenchmark evidence passed:

- formatting plus locked workspace all-target/all-feature check, tests, warning-denied Clippy, and
  warning-denied rustdoc;
- naming/dependency, realtime, effect-runtime, and graph policies plus policy mutation tests;
- exact seven-file graph fixture `--check`, all-file/manifest corruption, 100/100 fresh-process
  determinism, 10,000 graph mutations, SCC witnesses, all-rate/all-quantum PDC, numeric summation,
  and direct bound PCM tests;
- debug and release 65,537-track gates (release test 6.94 seconds on this machine), and the traced
  one-million-block lifecycle/render audit with zero allocation/free/lock/log/I/O/network/syscall
  violations;
- Android ARM64 and iOS ARM64 checks; Wasm scalar and separate `+simd128` release builds with cfg
  assertions; native x86-64 scalar, AVX2-without-FMA, and AVX2-plus-FMA checks.

No V1/legacy source was inspected. No benchmark was run while a nonbenchmark gate was missing.
Benchmark invocation count remains **0**. The single authorized command is exactly:

```text
scripts/run-graph-compiler-benchmark.sh
```
