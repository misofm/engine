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
