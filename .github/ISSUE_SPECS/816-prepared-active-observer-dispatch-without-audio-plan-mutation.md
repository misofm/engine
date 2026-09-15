# Prepared active observer dispatch without audio-plan mutation

Engine issue #816, child of #763. Root approved and synchronized the brief before implementation. Baseline engine `8e524194cdb945c5337bf81b3f209c5d4ae15984`. Design author: Astra XHIGH. Implementation: bounded Luna MAX tranches. One independent adversarial verdict per coherent attempt; maximum five attempts. This issue is a native product capability, not a new generic telemetry framework.

## Smallest closable result

A host prepares a finite catalog of graph observer bindings, then selects an admitted subset at block boundaries without changing graph topology, run units, audio buffers, bank cohorts, or DSP state. With the subset empty, graph observation dispatch visits no catalog entries and acquires no observation audio input. Existing unconditionally active observer constructors retain their semantics. Selective meters and spectrum adopt this seam in the dependent issue; this issue must demonstrate the contract with real graph observers and PCM, not just transport unit tests.

## Frozen architecture

1. Keep `GraphNodeObserverBinding` and every `RuntimeOp.observers` allocation structurally immutable. During lowering build a control-side catalog mapping stable observer handle to `(unit_index, member_index, observer_index, ordinal)`. `member_index` is absent for `RuntimeUnit::Op`. `ordinal` is the existing unit/member/direct/alias observer order. Opaque handles are already stable `u64`, not positional public identity.
2. Add an opt-in controlled binding constructor. Existing `GraphNodeObserverBinding::new` makes an always-active binding; add `GraphNodeObserverBinding::controlled(node, handle, observer)`. Controlled bindings start inactive. Duplicate controlled handles are preparation errors. Permanent observers remain in the dispatch base; they are accounted separately and may not be used by host-owned meters/spectrum after their migration. A permanent user observer deliberately continues its independently requested work.
3. Runtime owns a prepared, fixed-capacity active dispatch snapshot: boxed storage, a live length, revision and catalog-owner identity. Dispatch entries contain only validated `Copy` indices, never trait objects, `Arc`s or resources. Storage capacity is a caller-configured maximum active controlled bindings plus permanent bindings. All active entries are sorted off render by ordinal. The graph executor's existing audio-unit loop advances a cursor through this snapshot and calls only entries whose unit is the just-completed unit. Empty snapshot means one block-level gate; do not invoke `observe_unit` or walk bank members merely to find no observers. Preserve resident bank output handling and existing direct/alias order, including errors that accept resident input exactly once.
4. Snapshot switching is explicitly observation activation state. No mutation to `units`, op observer bindings, execution order, graph schedule, bank membership, alias analysis or arena allocation is permitted. Prepared observer-induced layout/optimization costs remain part of baseline preparation and are reported honestly.
5. Use the existing `bounded_spsc_move` ownership machinery, following `plan_exchange`'s ownership rules; no new unsafe transport. Prepare exactly three equal-capacity snapshot backing arrays: active, ordinary replacement, reserved removal replacement. Ordinary publication queue capacity is one, removal publication queue capacity is one, retirement queue capacity is two. A single control-side owner owns both producers, recycled arrays and monotonic revisions. At most one ordinary and one removal publication can be outstanding. A removal candidate must be a subset of the last accepted controlled set and must retain all permanent entries. While a removal publication is outstanding, ordinary publication refuses with `Backpressure`. A second removal returns `Backpressure` and is reconciled ahead of new starts by the host owner; this does not consume the ordinary credit. Closing all consumers uses the same empty-controlled-set removal.
6. At each block entry consume at most the two already-admitted snapshots, in monotonic revision order. An ordinary update admitted before a removal is applied first and the removal second at the same sample boundary; BOTH revisions receive an applied receipt with that exact sample. This is explicit successive state at one boundary, not cancellation of an accepted operation. No measurement runs between those two changes. Never poll/drain an unbounded queue. Switching swaps the active box; the displaced box is moved to retirement carrying the applied candidate's receipt. Queue capacity plus the three-buffer ownership invariant guarantees retirement space for each admitted publication. If defensive checks find no retirement space, retain the candidate in a prepared pending slot and defer; never drop/free a box on render, never overwrite a reader's storage, and never claim application before it happened.
7. All validation, handle lookup, sorting, duplicate rejection and desired-set construction happen control-side before publication. At activation, merge old/new sorted active entries to call the new optional observer hook `activation_changed(active: bool, generation: u64, first_sample: u64)` exactly for changed activation identities. This hook defaults to no-op; implementors must perform O(1) scalar reset/generation invalidation, no buffer clearing, queue draining, allocation or audio-state reset. Same live identity does not reset. One ordinary update may replace at most `maximum_active_observers` entries; two transitions cost no more than `4 * (maximum_active_observers + permanent_count)` old/new entry visits (checked arithmetic) plus fixed queue operations. This explicit configurable transition ceiling is part of preparation acceptance, not a performance claim.
8. Render failure invalidates only the currently dispatched observers, once each, preserving existing completed-before-failure semantics. Do not fall back to scanning all dormant prepared bindings on this error path. Transport-owner/catalog lifetime must outlive pending snapshot use. Teardown/drop is control-side after the render owner stops.

## Frozen public Rust surface

Place graph-specific implementation in `crates/graph/src/observation_activation.rs`, expose through `graph` (do not put graph identities in `engine`). Exact names below are the intended contract; ordinary derives/internal helper spellings are implementor choices.

```rust
pub struct GraphObservationActivationConfig {
    pub maximum_active_observers: usize,
    pub maximum_retained_bytes: u64,
}
pub struct GraphObservationActivationResources {
    pub retained_bytes: u64,
    pub largest_allocation_bytes: u64,
    pub maximum_active_observers: usize,
    pub maximum_transition_entry_visits_per_block: u64,
}
pub enum GraphObservationAdmissionError {
    UnknownHandle, DuplicateHandle, ActiveCapacity, RetainedBytes,
    InvalidRemoval, Backpressure, RevisionExhausted, OwnerClosed,
}
pub struct GraphObservationAccepted { pub revision: u64 }
pub struct GraphObservationApplied { pub revision: u64, pub first_sample: u64 }
pub struct GraphObservationController { /* unique control owner */ }
impl GraphObservationController {
    pub fn replace(&mut self, handles: &[u64])
        -> Result<GraphObservationAccepted, GraphObservationAdmissionError>;
    pub fn remove_to(&mut self, remaining_handles: &[u64])
        -> Result<GraphObservationAccepted, GraphObservationAdmissionError>;
    pub fn try_applied(&mut self) -> Option<GraphObservationApplied>;
    pub fn resources(&self) -> GraphObservationActivationResources;
}
```

`replace`/`remove_to` are complete sets of controlled handles; permanent observers are included automatically. Refusal preserves the complete last admitted set. `try_applied` recycles one retired backing array and returns one receipt; callers may invoke it in a bounded control-side loop. Application receipts never borrow native snapshot memory. Accepted means reserved and pending; it does not require a render call and must not claim an applied sample. Configure and create the controller as an additive optional field/result of the existing graph preparation path, keeping old preparation calls source-compatible through a new additive method rather than adding required fields to existing public struct literals. Implementor must locate the smallest existing graph builder seam and add `with_observation_activation(config)` plus a preparation result accessor `take_observation_controller()`; ownership is established before `PreparedRenderPlan` seals. Coordinator can approve a spelling-only adaptation if current builder ownership requires it; no architectural choice is delegated.

For this primitive, cap admission by active count and exact retained bytes. Per-family sample/copy/worker work admission is REQUIRED in the next issue before hosts expose activation. No claim that this count alone qualifies audio deadline headroom.

## Exact implementation tranches

A. `crates/graph/src/observation_activation.rs`, `crates/graph/src/lib.rs`: types, three-buffer ownership, preparation/resource projection, controlled binding constructor and optional default activation hook; bounded tests for ownership and refusal. No runtime dispatch rewrite in this tranche. It must compile; notify root for exact-path checkpoint before B.

B. `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, focused graph tests in the existing module or `crates/graph/tests/observation_activation.rs`: lower static catalog, active-only dispatch at the existing post-unit seam, boundary application/hooks/failure invalidation. Preserve resident planar-fallback contracts; update structural tests to discriminate the new seam rather than deleting their protection. Notify root for checkpoint.

No host, SDK, app, effect DSP, audio scheduler or ABI mutation in this issue. Changes to `engine` SPSC are not authorized: its existing move queues suffice; a demonstrated obstacle returns to coordinator before widening files.

## Discriminating acceptance

- Real active-audio graph with direct observer, alias observer, resident bank output and non-multiple SIMD tail: selected observations match old always-active outputs and PCM is bit-identical across empty/single/max/churn sets.
- Operation-site test counters: no-active dispatch visits zero observers, zero bank members for observation and zero planar/resident observation acquisitions. One active observer among a large prepared catalog visits only that observer. Existing per-audio-unit loop comparisons are reported as fixed integration overhead, not measurements.
- Acceptance/reservation race tests: ordinary full; reserved removal while ordinary outstanding; second removal refusal; no new starts ahead of pending removal; both admitted revisions applied in order at the same boundary; reader stalled for multiple blocks; delayed recycle; close during pending admission; old state survives all refusals. No accepted revision disappears.
- Count transition entry visits and enforce the declared bound; no arrays are cleared on start/stop. No allocations/frees/locks/syscalls across render and boundary application, including failure paths. No plan preparation/swap or DSP reset under churn.
- Resource projection exactly includes three snapshot arrays, catalog, two publication queues, retirement queue, permanent-base data and endpoint/shared state. Inclusive and one-below limits discriminate admission. Zero controlled preparation has no activation pool; existing no-observer graph path stays valid.
- Focused native graph suite and existing realtime policy/call-graph/allocation gates, plus Wasm graph compilation. Root owns broad exact-asset qualification at a coherent delivery boundary. No descriptive benchmark is needed for this primitive.

## Closure limit

Close only for the native prepared-dispatch capability. Parent #763 and selected host feed issue remain open. This does not yet make current browser meters or legacy spectrum idle; dependent migration and browser ingress qualification are mandatory before claiming the full product behavior.

## Coordinator freeze before attempt 1

Three buffers have one active role and two distinct free-credit roles: ordinary
and reserved removal. Ordinary admission never borrows the removal credit. A
publication carries its kind; retirement returns the displaced allocation tagged
with the applied candidate's kind, replenishing that credit regardless of the
physical buffer's previous role. Each retired record carries the newly applied
candidate revision/sample, never the displaced revision. Transition bounds include
permanent entries: checked 4*(maximum_active_observers+permanent_count).

The implementation worktree is /tmp/miso-observation-engine on
codex/observation-feeds. Tranche A is a focused native primitive checkpoint only;
tranche B must integrate it into real graph execution before issue closure.
Root owns all commits/pushes and source/remote evidence synchronization.
