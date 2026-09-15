# Controlled peak-meter compiler capability

Status: Astra XHIGH design approved by coordinator for bounded Luna MAX implementation. Depends on closed #816 (PR #817, merge a4e2d8ca).

## Product slice

Smallest closable result: a native caller can compile explicit dormant peak meters, bind them through #816, activate any admitted subset and get generation-fenced fresh windows without changing PCM. This is useful independently of host-core. Scope: `crates/builtins/src/lib.rs`, `crates/builtins/src/tests.rs`, `crates/builtins/tests/meter.rs`, `crates/builtins-compiler/src/lib.rs` and its existing tests; the two crates' Cargo manifests may receive only the test-support feature forwarding described below. No host-core or graph runtime edits.

### A1: meter observation-generation state

Actual source: `MeterConfig.reset_generation` initializes `QueueGeneration` and is copied unchanged into every `MeterSnapshot`; `MeterAccumulator::reset` does not change it. It is NOT an activation generation.

Freeze:

```rust
// Additive Rust fields; not a change to a frozen C/JS record.
pub struct MeterSnapshot {
    // existing fields unchanged
    pub observation_generation: u64,
}
impl MeterAccumulator {
    pub fn restart_observation(&mut self, generation: u64);
    pub const fn observation_generation(&self) -> u64;
}
```

Add private `observation_generation: u64` to the accumulator, initialized to zero by ALL existing preparation entry points. Existing snapshots always carry zero unless the new restart method was invoked. `restart_observation` assigns the supplied generation and resets `start=None`, `frames=0`, and both scalar `MeterLane` values via `meter_lane()`. It preserves `config`, queue/producer, decay, lifetime window sequence and cumulative clipped/sanitized/discontinuity/drop counters. Observation stop/start is not an audio/source discontinuity. No queues or arrays are cleared and no new generation is incremented on render. The graph controller already guarantees a checked nonzero fresh revision before an activation is admitted; the hook copies that revision, so a fallible render-time increment is unnecessary.

`MeterObserver::activation_changed(active, generation, first_sample)` invokes restart ONLY for `active=true`. On removal it does nothing: the immutable inactive observer is not visited, and the next activation discards its partial scalar window before observing. The first new window starts at its first actual observed sample, normally `first_sample`; it cannot span a stopped interval. This hook never calls any audio builtin reset. Same-live-identity graph replacements invoke no hook and preserve the partial window. `emit` copies the accumulator's observation generation separately from reset_generation.

`MeterSnapshot` is a Rust value, not repr(C) or an exported ABI struct. Add the field to existing Rust fixture literals with value zero; do not change exported web layouts or relabel their producer-reset fields. `bounded_spsc_retained_payload::<MeterSnapshot>` and `Layout::new::<MeterObserver>()` in existing builtin projection must automatically reflect the actual new sizes. No hardcoded byte increment.

A1 gates: old full-stat and selected meters retain all prior fields/bits, restart mid-window discards only the partial history, old queued snapshots keep their old generation, lifetime sequence/drop counters stay monotonic, reset_generation remains exactly configured, and restart has zero allocation/free. Add a sample-peak operation-site counter beside the existing test-only `meter_work_probe`; this counts actual lane samples processed in `observe_selected_segment`, not public call counts. The current builtins crate has no test-support feature, so add empty `builtins/test-support`, forward it from the existing `builtins-compiler/test-support`, and expose only hidden `test_only_reset_peak_samples()` / `test_only_peak_samples() -> u64` under test or that feature. Use a thread-local counter for the new probe so concurrent Rust tests do not contaminate it. Host-core already enables builtins-compiler/test-support as a dev dependency, so no third manifest feature is needed. Production has no probe operations. No new benchmark harness. Checkpoint before A2.

### A2: additive controlled preparation for both queue policies

Actual compiler has selected preparation ONLY in `prepare_selected_session_builtins_between_render_calls`. Host code currently takes that path whenever selected_meters is Some, regardless of its concurrent flag. Do not copy that error into demand preparation.

Freeze these additive signatures, with existing result/error types:

```rust
pub fn prepare_selected_session_builtins_with_console(
    session: &CompiledSession, requests: &[SelectedMeterRequest],
    controls: &[TrackControlRequest], caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet>;
pub fn prepare_controlled_session_builtins_with_console(
    session: &CompiledSession, requests: &[SelectedMeterRequest],
    controls: &[TrackControlRequest], caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet>;
pub fn prepare_controlled_session_builtins_between_render_calls(
    session: &CompiledSession, requests: &[SelectedMeterRequest],
    controls: &[TrackControlRequest], caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet>;
```

All use one existing `prepare_session_builtins_with_console_and_policy` implementation, adding a private meter-binding policy (`Permanent` / `Controlled`) separately from `BuiltinControlDelivery`. A shared selected-request helper extracts plain requests/metrics. Selected concurrent means `BuiltinControlDelivery::Concurrent`; controlled serialized means `BetweenRenderCalls`. Every old wrapper chooses `Permanent` and its old delivery policy.

Controlled requests in this first issue MUST have `metrics == SAMPLE_PEAK`, `tap == PostMatrix`, and `period_frames >= quantum` with `period_frames % quantum == 0`. Peak-hold and decay must be zero. Return explicit `builtin.meter.controlled_metrics`, `.controlled_tap`, `.controlled_period`, or `.controlled_ballistics` diagnostics on mismatch. General valid fixed-demand requests remain supported. Existing duplicate handle/track-tap, unknown track and caps diagnostics remain.

Only binding construction changes: controlled uses `GraphNodeObserverBinding::controlled`, permanent uses `new`. The observer kernel remains the same MeterObserver. Carry the binding-policy choice in the private prepared/sealed request data as needed so artifact validation cannot conflate permanently requested and controlled meters. Do not expose mutable observer parts or weaken provenance checks. Exact projection continues to use actual layouts, including any changed seal element layout.

A2 gates: both new controlled queue-policy paths preserve their distinct builtin scheduling contracts; selected concurrent path does not silently opt into between-render delivery. Compiler-only preparation and refusals compile before the wrapper bind assignment. Root checkpoints.

### A3: sealed artifact bind forwarding

Host-core does not possess `PreparedGraphPlan`: its artifact is `PreparedBuiltinsGraphArtifact<R>` with private graph/builtin fields. Root approved this missing additive wrapper seam:

```rust
impl<R> PreparedBuiltinsGraphArtifact<R> {
    pub fn into_bound_with_observation_activation(
        self, bindings: GraphRuntimeBindings,
        config: graph::GraphObservationActivationConfig,
    ) -> Result<(PreparedBuiltinsGraphBound, graph::GraphObservationController),
                PreparedBuiltinsGraphBindFailure<R>>;
    pub fn into_bound_with_source_set_and_observation_activation(
        self, bindings: GraphRuntimeBindings, source_set: GraphPreparedSourceSet,
        config: graph::GraphObservationActivationConfig,
    ) -> Result<(PreparedBuiltinsGraphBound, graph::GraphObservationController),
                PreparedBuiltinsGraphSourceBindFailure<R>>;
}
```

Reuse existing wrapper node/observer prevalidation and builtin/external ownership partitioning. Delegate to #816's corresponding graph bind exactly once. On failure reconstruct the opaque artifact, caller bindings and source set from the graph failure, including controlled flags and all producers/consumers. No parts extraction, cloning trait objects, late unreachable, or discard of caller-owned state. Generalize the existing source-set rollback family privately rather than write a second binding architecture.

The existing no-source `into_bound` currently treats a graph failure as unreachable. Once controlled meters exist, legacy bind with them is an expected `observation_activation_required` refusal. Make that path return its existing ownership-preserving failure type, like source-set bind does; do not panic. This requires no public graph getter or protocol change.

A3 closure gates: real graph PCM across empty/single/maximum/stop/reactivate subsets including banked lanes and tail; only selected meters visit sample sites; fresh first window and stale generation discrimination; legacy controlled bind rejection preserves all inputs; additive wrapper preflight failure (duplicate external handle, active capacity, byte limit, source mismatch) preserves all inputs. Use existing resource/allocation/realtime tests and native/Wasm compiler checks. Closing A establishes the native controlled compiler capability only.


## Invariants and scope

The prepared controlled catalog starts dormant. Render uses only prevalidated
indices and constant-size activation hooks; demand never recompiles or swaps the
audio plan, resets audio DSP, allocates/frees, blocks, or performs FFT work. New
activations copy their graph revision as observation generation; unchanged live
identities retain their existing partial window. Applied means the graph applied
the activation at that boundary, not that a later failing audio block succeeded.
Transactional refusal preserves caller-owned inputs and permits retry.

This issue delivers the compiler/bind capability only. Host feed identity,
shared work admission, browser ingress, SDK scopes and viewport wiring belong
to later issues under #763; no host controller or transport API is added here.

## Delivery

Three bounded Luna MAX assignments A1, A2, A3, each checkpointed when focused gates pass. Coordinator owns scope and exact-path commits. One independent adversarial verdict per coherent attempt, at most five attempts. Close only after PASS, upstream evidence and GitHub synchronization. Host ownership, browser transport, SDK adoption and UI changes belong to successors.

## Required integration at the coherent issue boundary

Adding MeterSnapshot/accumulator state changes actual retained layouts. After A3,
refresh only affected existing builtin resource fixture rows, their manifest
identity, and the resource-sensitive 10,000-case compiler transcript with a
layout-derived explanation. Preserve cases, outcomes and all audio fixtures.
Rebuild/repin the current Wasm artifact and run the existing browser matrix once
for its actual digest; published SDK release records remain historical until
the dedicated release slice. These are existing CI integration requirements,
not a new harness or permission to broaden DSP work.

Resource integration inventory from #816: the manifest refresh also updates
current audit/bench consumer constants and existing benchmark preflight/validator
synthetic-test pins (historical measured records stay unchanged). Run
test-builtins-benchmark.sh --check-manifest-consumers and the existing synthetic
validator suites, never timed workloads for this bookkeeping. Check the C API
primitive resource oracle and browser-v1 expected.json when their affected layout
terms change; preserve independent derivations and exact/one-below gates.

## A1 evidence

Fresh Luna MAX implemented observation_generation, scalar-window restart, the
active-only observer hook and test-support sample-site probe. The focused fixture
keeps an old snapshot queued through restart, preserves a nonzero drop count and
monotonic sequence, verifies fresh peak/energy and sample span, and checks zero
allocator operations during restart. Legacy configured reset generation remains
separate. Validation: builtins unit14/14, meter integration10/10, allocator tracker
9/9, compiler library check with and without test-support, formatting/diff PASS.
Pre-change native layouts were MeterSnapshot160/MeterAccumulator232; integration
resource/artifact pins are refreshed once at the completed issue boundary.
A2 preparation and A3 sealed bind forwarding remain pending.
