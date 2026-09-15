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

`replace`/`remove_to` are complete sets of controlled handles; permanent observers are included automatically. Refusal preserves the complete last admitted set. `try_applied` recycles one retired backing array and returns one receipt; callers may invoke it in a bounded control-side loop. Application receipts never borrow native snapshot memory. Accepted means reserved and pending; it does not require a render call and must not claim an applied sample. Use the existing `PreparedGraphPlan::bind_optional_source_set` seam. Add `PreparedGraphPlan::bind_with_observation_activation(self, bindings: GraphRuntimeBindings, config: GraphObservationActivationConfig) -> Result<(PreparedRenderPlan, GraphObservationController), GraphBindFailure>` and `bind_with_source_set_and_observation_activation(self, bindings, source_set, config) -> Result<(PreparedRenderPlan, GraphObservationController), GraphSourceBindFailure>`. Preserve old `bind` / `bind_with_source_set` signatures by delegating to the shared internal implementation with activation disabled. Existing public struct literals gain no required fields. Controller/pool creation occurs after preflight resolves immutable runtime indices and before executor sealing. Private transport API for tranche A is `prepare_activation(catalog: Box<[ActivationBinding]>, permanent: &[ActivationEntry], config) -> Result<(GraphObservationController, RealtimeObservationActivation), GraphObservationAdmissionError>`; `ActivationEntry { unit: usize, member: Option<usize>, observer: usize, ordinal: usize }` is Copy; `ActivationBinding { handle: u64, entry: ActivationEntry }` is immutable; `RealtimeObservationActivation::apply_boundary(first_sample, on_changed)` applies at most two snapshots and invokes `on_changed(entry, active, revision, first_sample)`, with `entries() -> &[ActivationEntry]` for the current immutable borrowed dispatch and `resources()` for accounting. Graph lowering constructs the catalog, never the SDK. All these transport types/helpers except the specified public controller records remain crate-private.

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

### Endpoint lifetime clarification (before attempt-1 verdict)

Astra/root require a shared `Arc<AtomicBool>` renderer-alive flag: initialize true,
Release-store false when the realtime endpoint is disposed OFF RENDER, and
Acquire-load before controller admission. Add `is_closed(&self) -> bool` to the
controller. `replace`/`remove_to` return OwnerClosed after disposal. Already
recorded applied receipts remain recyclable; accepted-but-unapplied revisions
become explicitly terminal owner closure, never fabricated application. A
concurrent disposal after admission preflight is classified by this terminal
state. Include the exact shared Arc allocation in retained/largest-byte reports.

Opt-in controlled preparation rejects `maximum_active_observers == 0` with
ActiveCapacity. Existing bind paths without controlled preparation retain zero
activation pool and no controller; zero configured capacity is not owner death.
The graph builder uses the additive bind methods frozen above.
# #816 tranche B: transactional activation preflight

Bounded design clarification only; append to the numbered brief before B. Do not change the audio lowering or return ownership after partial executor construction.

`SequentialPlan` already freezes emitted units and members before inputs move: `run_units`, `unit_of_run`, `op_slot`, and retired fold decisions. `build_sequential` consumes exactly that plan. Therefore final activation indices can and must be derived while borrowing inputs; there is no need for a fallible catalog creation after constructing `Runtime`.

Add the following crate-private helper in runtime:

```rust
pub(crate) struct PreparedObservationActivation {
    pub(crate) controller: GraphObservationController,
    pub(crate) realtime: RealtimeObservationActivation,
}

pub(crate) fn preflight_observation_activation(
    plan: &PreparedGraphPlan,
    program: &ExecutionProgram,
    bindings: &GraphRuntimeBindings,
    planning: &SequentialPlan,
    config: Option<GraphObservationActivationConfig>,
) -> Result<Option<PreparedObservationActivation>, &'static str>;
```

1. With `config=None`, reject any controlled observer from either `plan.observers` or `bindings.observers` with `graph.plan.observation_activation_required`; otherwise return `Ok(None)`. Old bind must not turn a controlled observer into an always-active one.
2. For configured activation, construct a temporary borrowed map of all observer bindings from both inputs. Use the existing `taps_by_op(program,spec)` function and the same direct-node-first, aliases-in-existing-order sequence used by `build_op`/`take_observers`. Sort each node's borrowed observers by handle. Consume these temporary map rows once, following retained `planning.run_units` and their original op order. Do not remove/move original observer objects.
3. `planning.op_slot[op]` supplies `(unit,member)`. Use `member=None` for a plain run (`membership.is_empty()`); use `Some(member)` for a bank run. Observer index is its index in that runtime op's concatenated direct/alias list. Assign ordinal monotonically in this exact emission order. This is the actual lowered catalog, not a guessed count or substitute layout. Check all arithmetic, referenced mappings and complete consumption; an unresolved observer or inconsistent mapping returns `graph.plan.observer`. Controlled handles must be globally unique, including different nodes; permanent and controlled dispatch ordinals are necessarily distinct.
4. Call existing `prepare_activation(catalog, permanent, config)` during this borrowed preflight. It allocates/validates the exact pool, queues, retained layout and liveness state BEFORE any caller-owned processor/observer/source moves. Map its typed admission failures to explicit graph activation diagnostics. This is the sole fallible activation preparation. Do not call it again in `GraphExecutor::new` or after `build_sequential`.
5. In `bind_optional_source_set`, run existing structural/source validation and `preflight_sequential` first, then the helper above. Every error still returns `(self, bindings, source_set, code)` untouched. On success split the prepared activation pair: retain the controller in the enclosing bind function and pass only `Option<RealtimeObservationActivation>` to an additive private argument of `GraphExecutor::new`/runtime construction. `GraphExecutor::new` remains infallible and returns `Self`; additive public bind returns `(PreparedRenderPlan, controller)` after sealing. Legacy bind discards no controller: its preflight returns None.
6. Materialization consumes the same frozen `SequentialPlan` and unchanged observer bindings. No second lowering, admission decision, handle lookup, resource check or new queue allocation happens after moving inputs. A test/debug assertion may compare emitted observer coordinates to the preflight catalog as an internal invariant; it cannot replace required borrowed validation or become a normal recoverable late error.

Use one shared helper for direct-node/alias enumeration if needed to prevent preflight and emission drifting. Do not introduce an alternate graph-planning algorithm. Temporary preflight maps are ordinary control-side memory, reclaimed before returning; retained pool/resource figures remain based on the actual final catalog.

Tests: controlled binding through each legacy bind rejects while returning every original input; duplicate controlled handles on different nodes; zero-active-capacity and one-below byte limit; unresolved observer; source-set bind failure preserving source ownership; successful direct+alias+bank-tail preflight coordinates match materialized dispatch exactly. A drop-witness observer/processor must remain undropped inside returned failure inputs, proving no late consume-and-drop path. Existing inclusive exact resource and PCM/ordering gates remain.

### Attempt 1 tranche A checkpoint

Luna MAX implements the graph-specific three-buffer controller, ordinary/removal
credits, applied receipts, renderer-lifetime flag, checked resource projection,
controlled-binding constructor and default activation hook. Focused activation
unit tests (3), `cargo check -p graph`, changed-file formatting and diff checks pass.
The crate-private runtime endpoint is intentionally not yet wired, producing
expected unused-code warnings until tranche B. This is a compiling transport
checkpoint, not a native graph capability verdict. Tranche B must complete real
dispatch, transactional bind, resource/PCM/realtime evidence and independent review.

### Bounded tranche B assignments

Split B without changing its contract: B1 implements only borrowed activation
preflight/catalog mapping and focused coordinate/refusal tests in runtime.rs plus
minimal lib.rs visibility glue; it does not expose new public bind methods or
change render dispatch. B2 then wires transactional public bind and active-only
runtime dispatch/activation/failure handling and runs the real PCM/realtime gates.
Each compiling slice is committed before the next fresh Luna MAX agent starts.

B1 edge-case ruling: opt-in activation bind requires at least one controlled
binding. A configured activation with an empty controlled catalog returns
`graph.plan.observation_activation_capacity` during borrowed preflight. Ordinary
bind with permanent observers only retains its existing dispatch and no activation
pool. Do not return an unusable empty controller or suppress permanent observers
by attaching an empty activation snapshot.

Root also ran the unchanged-dispatch graph library suite at tranche-A source
checkpoint d39d9cf5: 73/73 tests pass, including existing graph PCM/layout/resource
regressions. Evidence: /tmp/observation-816-a-graph-suite.log. This supplements the
primitive checkpoint; it does not establish the pending active-dispatch behavior.

### Attempt 1 tranche B1 checkpoint

Fresh Luna MAX implemented borrowed activation preparation from the frozen
SequentialPlan, shared direct/alias enumeration, and compact real-plan mapping
and refusal tests. Caller-owned processor/observer inputs remain borrowed during
all fallible activation preparation. Focused preflight and existing alias tests,
cargo check -p graph, formatting and diff checks pass. Public bind and render
dispatch integration remain B2; no native active-dispatch capability is claimed
from this checkpoint alone.

### Coordinator concurrency ruling during B2

The realtime endpoint must sample removal-queue availability before ordinary-queue
availability. The unique producer publishes an ordinary revision before its
subsequent removal revision; acquiring the removal publication first guarantees
the subsequent ordinary availability read includes that earlier publication.
Reading ordinary first could observe zero, race both publications, then observe
one removal and apply revisions out of order. Counts remain bounded and frozen
for this boundary. Include all newly render-reachable activation helper bodies
in existing realtime-policy coverage.

B2 consistency rulings: preflight and materialization must both use stable
per-node handle sorting, preserving equal-handle legacy permanent rows. Failure
invalidation covers the entire currently active snapshot, including active
observers not reached in the failed block; each observer decides which completed
window remains valid. It never scans dormant bindings. Existing realtime policy
continues to prohibit panic/expect/unwrap in marked activation helpers.

### Attempt 1 tranche B2 checkpoint

Fresh Luna MAX wired additive transactional bind APIs, block-entry activation,
active-only dispatch, selected failure invalidation, runtime layout witnesses and
the concurrency/ordering rulings above. The existing graph library suite plus
a public controlled-bind activation test passes: 75/75. cargo check -p graph and
diff checks pass. Root realtime-policy gate passes with 50 marked regions in
14 files (local log /tmp/observation-816-b2-realtime-policy.log).

This is a working native dispatch checkpoint. Before the issue verdict, a fresh
bounded evidence tranche must exercise allocator-guarded active audio, PCM across
empty/single/maximum/churn sets, bank/tail and failure behavior, input-operation
counters, and transactional source ownership. Root owns Wasm and independent
review. No host/SDK/browser adoption or deadline qualification is claimed yet.

### Coordinator gate cleanup

Wasm graph compilation passes (/tmp/observation-816-wasm.log). Root resolved the
new-code Clippy findings mechanically: derive the zero entry, simplify equivalent
branches/map, name the private bind result, restrict the fixture constructor to
tests and check endpoint resource agreement during control-side preflight. No
behavioral contract changed. Graph library Clippy with -D warnings now passes
(/tmp/observation-816-clippy-final.log); existing unrelated clippy.toml unreachable
path notices remain informational. Graph library tests remain 75/75 and realtime
policy passes. The separate allocator/PCM fixture assignment remains active.
