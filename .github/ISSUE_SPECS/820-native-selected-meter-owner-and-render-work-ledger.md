# Native selected-meter owner and render-work ledger

Status: CLOSED. Fresh Astra XHIGH attempt 1 PASS; PR #821 merged as e907b6323e396cd7f48079014360c9bbab1f1202 after required qualification run 34933354053 passed. GitHub closure verified. Depends on closed #816 and #818.

## Product slice

Depends on closed #816 and #818. Smallest closable result: host-core prepares an explicit meter catalog and one private owner that admits/replaces/removes selected meter production within declared work/storage limits; the returned host starts idle. There is no spectrum support or resident/response protection claim yet. Scope: new `crates/host-core/src/observation_demand.rs`, existing `prepare.rs`, `lib.rs`, focused `tests/observation_demand.rs` plus existing preparation fixtures.

### B1: exact owner, records and accounting

Use a private-constructor `ObservationOwnerId(NonZeroU64)`, generated once off render with checked monotonic allocation from a process-local atomic counter. Exhaustion is preparation refusal. The value is scoped to the current host process/module lifetime; future wire adapters also retain their existing engine/owner epoch. It is never reused within that lifetime. No per-block owner allocation or global lookup map.

Freeze these public types:

```rust
pub struct HostMeterId { pub owner: ObservationOwnerId, pub handle: MeterHandle }
pub struct PreparedHostMeter {
    pub id: HostMeterId, pub track_id: Box<str>, pub tap: MeterTap,
    pub metrics: MeterMetricSet, pub period_frames: NonZeroU32,
}
pub struct ObservationWorkLimits {
    pub maximum_active_meter_channels: u64,
    pub maximum_meter_samples_per_block: u64,
    pub maximum_meter_publications_per_block: u64,
    pub maximum_meter_publication_bytes_per_block: u64,
    pub maximum_active_spectrum_captures: u64,
    pub maximum_capture_input_samples_per_block: u64,
    pub maximum_capture_copy_samples_per_block: u64,
    pub maximum_capture_publications_per_block: u64,
    pub maximum_capture_bytes_per_second: u64,
    pub maximum_transition_entry_visits_per_block: u64,
    pub maximum_retained_bytes: u64,
}
pub struct ObservationWorkCost {
    pub active_meter_channels: u64,
    pub meter_samples_per_block: u64,
    pub meter_publications_per_block: u64,
    pub meter_publication_bytes_per_block: u64,
    pub active_spectrum_captures: u64,
    pub capture_input_samples_per_block: u64,
    pub capture_copy_samples_per_block: u64,
    pub capture_publications_per_block: u64,
    pub capture_bytes_per_second: u64,
    pub transition_entry_visits_per_block: u64,
    pub retained_bytes: u64,
}
pub enum ObservationRefusalReason {
    NotPrepared, WrongOwner, Capacity, WorkBudget, Backpressure,
    Conflict, Closed, InvalidRequest, ArithmeticOverflow, RevisionExhausted,
}
pub struct ObservationRefusal {
    pub reason: ObservationRefusalReason,
    pub limit: Option<&'static str>, pub requested: Option<u64>,
    pub maximum: Option<u64>,
}
pub struct ObservationAccepted {
    pub owner: ObservationOwnerId, pub revision: u64, pub work: ObservationWorkCost,
}
pub struct ObservationApplied {
    pub owner: ObservationOwnerId, pub revision: u64, pub first_sample: u64,
}
pub enum ObservationReadError {
    WrongOwner, NotPrepared, Inactive, PendingApplication, Closed,
}
pub struct ObservedMeterSnapshot { pub meter: HostMeterId, pub snapshot: MeterSnapshot }
pub enum ObservationStop { Quiescent, Pending(ObservationAccepted) }
```

The meter publication/input fields are deliberate additions to the earlier draft: a sample count alone omitted per-window snapshot copy bursts. Spectrum input count is separate from copy count because `continuous_capture` scans selected input for nonfinite samples even on a waiting block. Resident and response budget fields are NOT inert placeholders in this first API; draft03 will add real admission fields when it supplies their producer seams. Do not advertise or accept limits that do nothing.

For M accepted dual-mono meters and Q quantum frames, checked worst-case costs are `active_meter_channels=2*M`, `meter_samples_per_block=2*M*Q`, `meter_publications_per_block=M`, `meter_publication_bytes_per_block=M*size_of::<MeterSnapshot>()`. Count a publication attempt even if the telemetry queue is full. Fixed meter windows are integral quanta, so every active meter can publish at most once per block; simultaneous alignment is the charged case. This counts payload bytes and sample work, not instruction counts or a CPU guarantee. Empty selection has zero measurement/publication cost.

Set `transition_entry_visits_per_block` to #816's FULL prepared bound, not the current set's smaller transition. Retained cost is a fixed conservative preparation reservation for the prepared lifetime: exact graph activation resources plus the existing builtin meter payload reservation plus new host catalog/identity strings/controller arrays. The builtin reservation already includes transferred readers and their identity strings; do not recharge them. It also includes compiler seal/request storage discarded before final host binding, so this total is not an exact live-heap claim. Exclude unrelated audio/control/source/effect storage; the existing HostPrepareCaps still bound those. A meter/capture removal never frees that retained reservation. No double counting: charge meter producer/queue/observer via builtin resources, readers/control metadata via host projection, graph snapshot/catalog via graph resources exactly once.

Controller owns preallocated complete-set arrays for accepted state, applied state, candidate scratch and TWO pending application records. Each array has capacity equal to the configured active controlled observer limit (or the exact lower possible count), and live length. Elements are Copy prepared-meter indices plus observation generations. No speculative spectrum descriptor is allocated; the later capture issue may extend private layouts. Pending records have no unbounded Vec/queue. Lookups may scan the prepared catalog off render. Exact layouts and identity-string allocations enter a host observation resources report. Allocate candidate host storage before any publication. Limits are inclusive; overflow yields ArithmeticOverflow, not saturation.

One private `ObservationBudget` owns these limits and accepted per-family cost. Use `preflight_graph_demand(&self, meters, spectrum_cost) -> Result<ObservationWorkCost, ObservationRefusal>` and an infallible `commit_graph_demand(cost)` after publication; this is the future shared owner for03, not another SDK policy map. Draft03 adds its existing-queue resident reservation records here and its response request budget here. It does not create another HostObservationController or expose its graph controller.

B1 assignment is records, checked pure projection/cost/refusal functions and tests only; no render binding. Prove inclusive/one-below for every implemented work field, overflow and empty costs. Root checkpoints before integration.

### B2: preparation and selected-meter methods

Freeze:

```rust
pub struct HostObservationPreparation<'a> {
    pub meters: &'a [HostMeterRequest],
    pub spectrum: Option<&'a SpectrumCaptureCollectionRequest>,
    pub work_limits: ObservationWorkLimits,
    pub activation: GraphObservationActivationConfig,
}
pub fn prepare_host_runtime_with_observation_demand(
    compiled: &CompiledSession, caps: &HostPrepareCaps,
    console: &HostConsoleRequest, observations: &HostObservationPreparation<'_>,
) -> Result<(PreparedHost, HostConsoleHandles, HostObservationController), PrepareDiagnostics>;
pub fn prepare_host_runtime_with_observation_demand_between_render_calls(
    compiled: &CompiledSession, caps: &HostPrepareCaps,
    console: &HostConsoleRequest, observations: &HostObservationPreparation<'_>,
) -> Result<(PreparedHost, HostConsoleHandles, HostObservationController), PrepareDiagnostics>;

impl HostObservationController {
    pub fn owner(&self) -> ObservationOwnerId;
    pub fn meters(&self) -> &[PreparedHostMeter];
    pub fn replace_meters(&mut self, meters: &[HostMeterId])
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn remove_meters_to(&mut self, remaining: &[HostMeterId])
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn stop_all(&mut self) -> Result<ObservationStop, ObservationRefusal>;
    pub fn try_applied(&mut self) -> Option<ObservationApplied>;
    pub fn try_read_meter(&mut self, meter: HostMeterId)
        -> Result<Option<ObservedMeterSnapshot>, ObservationReadError>;
    pub fn work(&self) -> ObservationWorkCost;
    pub fn is_closed(&self) -> bool;
}
```

B rejects `spectrum=Some(nonempty)` with `host.observation.spectrum_not_supported`. Empty spectrum collection is normalized to absent. C enables the field without replacing constructors. With both catalogs empty, use ordinary graph bind and `graph=None` inside the live host owner; zero feed storage/activation pool, zero work, `stop_all=Quiescent`, named target requests=NotPrepared. The owner record itself and any requested audio controls are not feed storage. Do not call #816 configured bind with an empty controlled catalog; #816 correctly refuses it. In this inert case owner lifetime is controlled by its host owner and no nonexistent graph pool is claimed as liveness evidence. `replace_meters` and `remove_meters_to` return NotPrepared even for an empty request; `stop_all` is the explicit no-op/quiescence API and creates no synthetic graph revision. `try_applied` returns None, all reads are NotPrepared, and `is_closed` does not infer renderer death from an absent pool.

For nonempty catalog, preserve preparation order in public meter catalog, allocate existing meter handles `index+1`, and require unique `(track,tap)`/handle identities. Existing canonical track order still controls audio queues. Require `Some(console.meter_period_frames)` for nonempty meter catalog and the controlled constraints from A. The explicit meter list is authoritative: never expand to all tracks from `console.meter_tap` or `master_track`. A designated master is only observed if named in the catalog and demanded.

Extend the shared PRIVATE `prepare_host_runtime_with_console_policy_and_spectrum` path with optional demand preparation. It selects A's correct controlled queue-policy wrapper, compiles the same artifact, prepares all control-side metadata, and calls A3's additive source-set binding. Existing wrappers pass None and keep their behavior. Resource/transition limits, graph/model cap and maximum single allocation must be checked before returning/publishing the prepared host. Include graph activation and host owner resources in the existing aggregate cap calculations as well as the narrower observation retained cap; expose additive report facts instead of hiding those allocations in an old unrelated field. If exact graph resources become available only after the transactional bind, reject and drop the entire LOCAL result off render before returning it; no partially prepared host is published. No plan is rendered during prepare.

Move `bound.meter_consumers` into the private HostObservationController. The returned HostConsoleHandles retains tracks/audio producers/master designation but has `meters=[]`; it has no raw spectrum or graph activation handle. Raw consumers cannot produce extra measurements, but ownership here also prevents unfiltered stale reads. `HostObservationController` is unique and non-Clone. `HostConsoleHandles.effect_observations` is empty because resident preparation is refused until03.

`replace_meters` validates owner, prepared handles, duplicates, per-family cost and pending-record room, then forms the complete controlled union retaining spectrum (absent in B). Call graph.replace last. New members receive the returned revision as generation; unchanged members retain their prior generation. Copy the resulting complete set into its already reserved host pending record and accepted view without further fallibility. `remove_meters_to` also requires an exact subset of the last ACCEPTED meter set and uses graph.remove_to; it retains sibling spectrum. Replacements that happen to be subsets may be explicitly routed to remove_to, but callers must never need ordinary room to stop.

`try_applied` pops/recycles at most ONE graph receipt per call, matches exactly one bounded pending record, updates applied state and returns that receipt. A caller may make at most two calls per reconciliation. An ordinary publication followed by removal can produce two real receipts at the same sample. Retain both even if the first set never observes samples. No receipt is overwritten/coalesced after admission. `stop_all` uses reserved removal; already admitted identical all-stop returns its pending receipt; already fully applied empty returns Quiescent without publishing another snapshot. On Closed it terminally invalidates any pending request; it does not claim application.

Reading a wrong-owner/unknown/inactive/pending-new-generation meter produces the explicit state without draining any measurement queue. For an applied live generation, snapshot `available_at_entry()` once, bounded by configured queue capacity; pop no more than that many entries, discard only mismatching observation generations, and return the newest matching record encountered (or None). No unbounded drain while a producer replenishes. Dropped/stale control-delivery records are counted in owner diagnostics separately from engine producer drop counters. During receipt reconciliation, before replacing the old applied array, use its live prefix as the finite cleanup list: for each removed meter freeze available_at_entry once and pop at most that count/capacity OFF RENDER. Cleanup is synchronous and bounded; no separate persistent cleanup queue is needed. Final Quiescent follows applied empty state with no pending receipts. Do not inspect every prepared inactive meter on polling; iterate only applied selections or explicitly removed entries.

B closure gates: real host empty/single/all-within-budget/churn renders produce bit-identical PCM; active among many prepared targets performs only selected sample work; pending paused host, exact refusal preserving old stream, duplicate/wrong owner/not prepared, removal during ordinary pending, first fresh window, stale queued generations and stalled reader all discriminate. No reprepare/plan swap/DSP reset. Existing native host/compiler realtime tests and Wasm build. Close B without waiting for resident/browser work; document the native meter-only scope and typed resident/spectrum refusals.


## Native ownership invariants

1. #816 supplies the immutable controlled catalog, two publication credits, checked monotonic graph revisions and exact application-boundary receipts. The host owns its one graph controller privately. No new graph activation protocol, scheduler, queue primitive, worker, FFT mode, hop configuration or concurrent spectrum-job capability belongs here.
2. Existing Rust constructors keep their fixed-demand behavior and numerical output. A controlled constructor is additive, starts with no active meters/captures, and never treats preparation as demand. Compatibility callers on a later protected browser/headless host must translate old calls into ledger requests. They cannot obtain raw activation/capture controls from that host.
3. These slices protect graph meter/capture activation and its retained/work budget. Until draft03 installs resident admission, the new demand constructor rejects `console.observation_taps != 0` before allocating observer storage (`host.observation.resident_not_supported`). Old fixed-demand constructors are unchanged. Existing `PreparedHost::copy_response_snapshot` remains an exclusive-boundary operation; response-copy rates and worklet message handlers are explicitly OUTSIDE these slices. Draft03 must mediate that method at the existing endpoint ownership seam using the same host budget owner, before advertising all-entry-point protection.
4. Native identity is `(observation owner, prepared feed identity, observation generation)`. An owner is one successful demand preparation/lifetime; a feed handle is never valid on another owner even if its numeric meter handle matches. Graph revisions are the observation generations of NEW activations. An unchanged active meter keeps its existing generation through sibling changes. Removal has an application revision but does not create a new stream generation.
5. Check all limits, identity arithmetic, allocation/projection arithmetic and pending-record room before graph publication. There is no allocation, checked arithmetic that can refuse, lookup that can fail, or fallible pool operation after successful publication. Commit the reserved host metadata with bounded infallible assignments. A refusal preserves the accepted set, applied set, current producer state and stored valid result.
6. Applied means #816 applied its activation state at that boundary. It is not a guarantee that the ensuing audio block succeeds; existing invalidation rules fence failed-block observations. A renderer closed before application produces terminal `Closed`, never a fabricated sample. Do not add a second render-completion system here.
7. Host admission serializes family changes into a complete union: accepted meters plus at most one accepted spectrum slot. Each family replacement is atomic only for that family. Pending publications remain obligations. A removal uses #816 `remove_to`, retains all sibling demand and can pass one ordinary publication. No ordinary admission while a removal is outstanding. At most two pending host application records match #816's two credits.
8. Render receives only prevalidated indices and activation hooks. Host desired-set construction, stale-queue cleanup, target lookup, results, receipts and all ledger changes happen control-side. Final quiescence requires removal applied and bounded control-side cleanup, not merely a close request.


## Accounting followthrough

Activation retained bytes include controller inline C and graph runtime R. A host aggregate already charging baseline graph R must subtract that overlap once; embedding controller C in the host owner must not charge it again through both containing-owner and activation rows. Derive the actual containing-owner boundary from the implementation, preserving exact-inclusive/one-below tests.

## Frozen coordinator implementation decisions

The following Astra XHIGH decisions are approved and supersede broader assignment boundaries or private placeholders above. B1 and B2.1–B2.4 are five sequential checkpoints inside this issue, each assigned to a fresh Luna MAX. They are not separate review attempts.

## Frozen private implementation choices

### One owner; five complete sets and one handle scratch

Use one `HostObservationController` containing the owner ID, `Option<GraphObservationController>`, public catalog `Box<[PreparedHostMeter]>`, transferred `Vec<MeterConsumer>`, one budget, selection storage, small private delivery counters, and terminal-closed flag. No public graph getter, cloneable owner, shared map, per-meter generation table, or meter-result cache.

Let N be catalog length and K=min(N, configured maximum active observers). Use:

```rust
#[derive(Clone, Copy, Default)]
struct MeterSelectionEntry { prepared_index: usize, generation: u64 }
struct MeterSelection { entries: Box<[MeterSelectionEntry]>, len: usize }
struct PendingApplication {
    accepted: Option<ObservationAccepted>,
    selection: MeterSelection,
}
```

Allocate five K-entry selection arrays: accepted, applied, candidate, pending ordinary, pending removal. Store pending records in a fixed two-element array, index 0 ordinary/index 1 removal. Also allocate one K-entry `Box<[u64]>` candidate-handle array: graph publication takes a contiguous handle slice, so allocating a Vec on each mutation is forbidden. All arrays have live lengths; empty preparation allocates none. Do not reserve a speculative spectrum descriptor; C can extend private layouts later.

Sort live selections by prepared index. Catalog handles are index+1, preserving request order independently of canonical audio-control order. Transfer readers in selected request order, never resort them by track. Candidate generation zero means unpublished new member; unchanged members retain the generation from the **accepted** set. After graph acceptance, fill zeros with its revision using only infallible assignments.

Owner allocation uses a process/module-local AtomicU64 last-issued counter, initially zero, with checked `fetch_update(Relaxed, Relaxed, |last| last.checked_add(1))`. Tests use their own near-exhausted atomic, never reset the production counter. Failed local preparation may abandon an ID; it cannot reuse one.

### Resource rows and exact overlaps

Add `HostPrepareReport.observation_demand_resources` with this small additive record (the draft did not fix report spelling):

```rust
pub struct HostObservationResources {
    pub graph_activation: Option<GraphObservationActivationResources>,
    pub owner_inline_bytes: u64,
    pub metadata_heap_bytes: u64,
    pub metadata_largest_allocation_bytes: u64,
    pub reserved_bytes: u64,
}
```

Old constructors report zero/None. Let H=size_of HostObservationController, C=size_of GraphObservationController, A=activation report, R=A.runtime_state_bytes, B=existing builtin meter reservation. All arithmetic/conversions are checked.

* Nonempty owner inline = H-C: A already charges C. Keep Option discriminator/padding and containing-owner padding in H-C; do not subtract size_of the whole Option.
* Metadata heap = new catalog array + its actual string allocations + five K-entry arrays + K-entry handle scratch. Derive Layout::array from actual types. Transferred readers/strings are excluded here.
* Metadata largest = largest individual new heap allocation, including strings; inline H/C is not a named allocation.
* Nonempty reserved = B + A.retained_bytes + owner inline + metadata heap. This is the narrow fixed observation reservation, retained after removal.
* Empty catalog: graph None, metadata heap 0, reserved/work 0. Owner inline H still exists, but is not feed storage or a heap allocation.

Add `owner inline + metadata heap + (A.retained_bytes-R)` to the common path's existing graph/model admission expression (A term zero when absent). The old builtin cap remains separate; do not move/recharge the builtin reservation into that expression. Fold new heap maximum and A.largest_allocation_bytes into the existing named-allocation maximum. Do not repurpose `observation_retained_bytes`, which is the existing resident-effect row.

Use A's **full configured** transition bound, never the smaller chosen set or K. For B, no permanent observers are added, so its pre-bind checked projection is 4*activation.maximum_active_observers; the returned graph report remains authority. Test exact/one-below from actual layouts, not hardcoded native bytes.

### Preparation

Extend only `prepare_host_runtime_with_console_policy_and_spectrum` with optional demand input and optional returned owner. A four-element private result tuple is sufficient. Old callers pass None. Demand wrappers pass the explicit meter slice, no legacy spectrum request, and their respective queue-policy flag.

Reject resident taps and nonempty spectrum with the frozen diagnostics before observer storage allocation; normalize empty spectrum to absent. Explicit empty meters must remain `Some(empty)`, so console period/tap/master never expands the catalog. Nonempty meters require a period and #818's controlled validation.

The demand builtin branch explicitly selects the controlled concurrent or between-render-calls wrapper. The current legacy selected branch always selects serialized delivery: do not copy that behavior or broaden this issue to fix legacy callers.

Use A3 source-set+activation bind only for a nonempty catalog; empty uses ordinary bind. Prepare all control metadata and finish exact cap checks before returning the host. Exact post-bind refusal drops the entire local result off render; no preparation render or partly published host. Return raw meter consumers only inside the owner; console tracks/audio producers/master remain available, console meters/resident readers are empty.

Empty-owner is_closed remains false for that live inert owner; absent graph resources cannot prove renderer death. No new liveness channel is needed.

### Admission

Use a common bounded candidate builder/publisher. `replace_meters` always uses ordinary publication, even for equal/subset requests; `remove_meters_to` always uses removal. This keeps the explicit reserved-stop route simple.

Before publication: absent graph→NotPrepared; terminal graph closure→Closed; validate owner/known handles/duplicates; check length<=K before fixed writes; validate removal against accepted set; project work; require ordinary pending[0] and pending[1] empty, or removal pending[1] empty; build candidate entries and handles. Lookups may scan the catalog off render. Scratch changes on refusal are harmless; accepted/applied/pending records, queues, and budget cannot change.

Graph call is last. On success fill candidate zero generations, copy into the reserved pending and accepted arrays, store ObservationAccepted, commit the prevalidated work. No fallible lookup, allocation, checked increment, or refused operation remains. Do not poll receipts to gain room.

Refusals: foreign owner→WrongOwner; unknown handle→NotPrepared; duplicate→InvalidRequest; non-subset removal→Conflict; selection capacity→Capacity; work field→WorkBudget with the actual public maximum_* field name. Graph mapping: UnknownHandle/ DuplicateHandle/ ActiveCapacity or RetainedBytes/ InvalidRemoval/ Backpressure/ RevisionExhausted/ OwnerClosed map to NotPrepared/ InvalidRequest/ Capacity/ Conflict/ Backpressure/ RevisionExhausted/ Closed. Keep existing graph/builtin preparation diagnostics.

### Receipts, cleanup, stop, reads

Only `try_applied` consumes graph receipts. Pop at most one, match its revision to exactly one of the two records, reconcile that complete applied set, recycle only that record, and return the real receipt. Never coalesce the two receipts from ordinary+removal at one sample. A missing local record is an invariant defect, not permission to synthesize a receipt.

Cleanup is synchronous control-side reconciliation: before replacing the old applied array, visit its live prefix, select entries absent from the incoming set, freeze each removed reader's available_at_entry once, and pop at most that count/capacity. The old applied array is the finite cleanup worklist. No persistent cleanup queue or scan over N inactive prepared meters is needed. A consumed removal receipt proves that old producer cannot run again before a later admitted activation.

`stop_all` does not poll receipts. Inert→Quiescent; closed→Closed. If latest accepted set is empty and an empty pending record exists, return that stored receipt (including an ordinary empty replacement). Applied/accepted empty with no pending records→Quiescent. Otherwise call remove_meters_to(empty). Repeating stop spends no new revision. Synchronous cleanup makes no-pending/applied-empty sufficient for final Quiescent. Closure invalidates pending obligations and produces no fabricated sample or Quiescent claim.

Read precedence: absent graph→NotPrepared; closed→Closed; foreign owner→WrongOwner; unknown→NotPrepared; absent accepted member→Inactive; absent applied member/different generation→PendingApplication. Equal accepted/applied generation remains readable during sibling changes. None of those error paths pops a queue or receipt.

Then freeze available_at_entry exactly once and return newest matching-generation snapshot from at most that many pops. Keep small private counters for stale generations, superseded matching records, and retirement discards, separately from engine producer drops. Informational counters may saturate; identity/cost/allocation arithmetic cannot. No extra public telemetry method or cache is required.

## Five sequential compiling assignments

### B1: records, owner ID, pure budget

Paths: new observation_demand.rs and lib.rs exports. Preserve frozen public records; implement checked projection/refusal/commit and ID allocation, without binding. Pure gates: last valid/exhausted owner ID, unique IDs, empty and fixed reservation costs, exact/one-below for implemented work fields, overflow, failed projection preserves accepted cost. Spectrum fields stay zero. Compile with/without control-provider; root checkpoints.

### B2.1: real preparation and storage/accounting

Paths: observation_demand.rs, prepare.rs, lib.rs, focused observation/preparation tests. Define actual owner/array layouts and resource report; implement both real constructors and owner/meters/work/is_closed getters. No fake mutation methods.

Gates: correct controlled queue-policy selection, request-order catalog versus canonical controls, no returned raw readers, owner identity, unchanged audio controls/master, typed unsupported-family refusals, empty catalog normalization, zero preparation sample visits. Derive exact new layout rows and reader overlap; test narrow retained/transition, graph/model, and largest-allocation exact/one-below caps. Preserve old preparation tests. A post-bind cap refusal returns no host. Root checkpoints before adding mutations.

### B2.2: admission and two pending obligations

Paths: observation_demand.rs and focused tests. Implement candidate/publish, replace_meters, remove_meters_to. Paused real-host gates: accepted without application/sample visits, ordinary backpressure, reserved removal passing ordinary, second removal refusal, all identity/subset/cap/work refusals preserve accepted state and both obligations. Private unit assertions verify unchanged/new generations without exposing ledger internals. Scoped allocation check uses existing test machinery. No receipt-drain shortcut. Root checkpoints.

### B2.3: receipts, cleanup, stop

Paths: same. Implement try_applied, bounded retired-reader cleanup, terminal closure, stop_all. Real-render gates: no fabricated paused receipt; actual first sample; two same-boundary receipts; repeated pending stop identity; initial/final quiescence consumes no revision; subset removal preserves siblings; renderer drop→Closed; actual selected sample counts become zero after last removal. Cleanup visits only retired entries and their bounded queues. Root checkpoints.

### B2.4: fenced reads and final native closure

Paths: same, necessary docs/export corrections, issue evidence. Implement try_read_meter and private counters. Gates: state-error reads never drain; unchanged meter remains readable during sibling changes; refusal preserves old stream; first fresh window/generation; stalled bounded reader; bit-identical PCM across empty/single/all/churn; sample-site count exactly 2*selected*Q, using an existing nine-track bank-plus-tail fixture.

Test the defensive stale-generation helper with actual MeterAccumulator-produced old/new records in one queue, restarting the accumulator directly in that small unit test. Do not invent a legal host reactivation that bypasses the retirement cleanup established above. Reuse existing allocation/realtime machinery; render activation/deactivation remains allocation/free/lock/syscall-free.

At the coherent issue boundary, run existing host-core native/control-provider tests, proportional shared-preparation/compiler realtime checks, and the existing Wasm build once. No new benchmark infrastructure, timed run, broad fixture matrix, or future SDK scaffolding. Root checkpoints/delivers in the active delivery mode; final fresh adversarial review and upstream GitHub synchronization remain required before closure.


## B1 evidence

Fresh Luna MAX added the frozen public records, checked process-local owner IDs,
and private pure meter budget. Preflight takes a selected meter count, avoiding
request-object reconstruction during later admission. Only fixed transition and
retained reservations survive into empty-selection cost; spectrum work stays zero
and nonzero spectrum demand is refused. Tests cover unique/sticky-exhausted IDs,
exact/one-below limits, checked overflow, empty/fixed costs and failed projection
preserving accepted work. Nine focused tests PASS with and without control-provider;
formatting/diff PASS. Only observation_demand.rs and lib.rs exports changed.
Host preparation and admission remain the sequential B2 tasks.

## B2.1 evidence

Fresh Luna MAX implemented the real owner storage and both preparation APIs through
the existing shared transaction. It preserves explicit request order, keeps raw
readers private, chooses the controlled concurrent/serialized compiler paths, and
checks graph/owner overlaps and all caps before returning a host. Empty demand
retains only its charged inline owner, with no activation pool/heap/work reservation.
Allocation refusal is distinct from checked arithmetic overflow; reported largest
allocation includes owner metadata; initial work includes the fixed reservation.
Coordinator supplied four public preparation tests after a fresh test-agent spawn
hit the thread limit. They independently check concrete owner/catalog/five-array
storage terms, absence of reader double charging, both constructors, initial zero
sample work, explicit unsupported-family refusals, empty normalization, and five
inclusive/one-below cap boundaries. All four PASS with control-provider; Luna's
existing host test suite and checks with/without control-provider PASS. Formatting
and diff PASS. A scoped argument-count lint allowance retains the frozen shared
preparation seam instead of introducing another preparation architecture.
B2.2 admission and B2.3/B2.4 lifecycle/read methods remain pending.

### B2.2 admission checkpoint

Fresh Luna MAX implemented complete-set meter candidate validation, ordinary/reserved-removal publication, graph refusal mapping, and infallible generation/pending/accepted/budget commit after graph acceptance. Paused real-host tests cover zero sample work, occupied ordinary credit, reserved removal passing ordinary, second removal refusal, and typed identity/subset/capacity/work refusals. Root reviewed publication ordering and added the existing thread-local audited allocator assertion around actual admission; no new harness. Focused observation integration tests pass in default/native and control-provider modes (7 each), format and diff checks pass. Generation continuity/reactivation will be qualified against actual published snapshots in B2.4, rather than a redundant private-state oracle. Receipt reconciliation, cleanup, stop and reads remain the next bounded tasks; no browser/SDK capability claimed.

### B2.3 receipt/stop checkpoint

Fresh Luna MAX implemented sole-consumer one-receipt reconciliation, synchronous retired-reader cleanup bounded by each frozen queue count, terminal closure invalidation, and idempotent pending/all-stop quiescence. Root reviewed invariant handling: a consumed real receipt must match a retained pending record; impossible private-state mismatches cannot silently return no receipt. Real StartedRenderSession tests prove paused pending state, two distinct receipts at the same boundary, nonzero application samples 128/256, preserved sibling sample work, final zero selected sample work, repeated stop receipt identity, and renderer-drop Closed without fake application. Existing focused integration suite passes 10 tests in native and control-provider modes; format/diff pass. Private retirement-discard counter is included automatically by actual containing-owner layout accounting. Generation-filtered reads/native closure remain B2.4; no browser/SDK claim.

### B2.4 complete native source checkpoint

Fresh Luna MAX implemented generation-filtered bounded newest-record reads with private stale/superseded diagnostics and actual MeterAccumulator-produced old/new generation coverage. Real nine-track bank-and-tail tests compare PCM bits across idle/single/all/tail/empty demand, count actual selected sample sites, check queued-state refusals, unchanged sibling generation, fresh reactivation, bounded stalled reads, and monotonic producer sequence/drop counters. Focused integration tests pass 13 per mode; module tests pass 10 per mode. Root full host-core native and all-feature suites pass, as do all-target/all-feature clippy with denied warnings, format and diff checks. Root corrected the earlier B2.1 nested cap-check lint without changing behavior. Native source capability is complete; independent Astra verdict, Wasm/integration qualification and merged/GitHub delivery remain pending. No browser feature completion claim.

### Attempt 1 independent native verdict and artifact integration

Fresh Astra XHIGH native review: **PASS**, no blockers, source candidate `a45aee3fb7491ca5372b647c2e5580cb6c563e58` against `b568b09a`. Reviewer independently passed 13 host observation tests and the existing controlled-activation realtime allocation/PCM test; inspected transactional admission, separate actual receipts, bounded retirement/reads, generation fences, closure, and concrete H-C/A-R accounting. Root full native host suite passed 132 tests (2 existing ignored); all-feature host suite 182 (2 existing ignored); compiler allocation suite 9/9, host policy/mutation gates, clippy/fmt/diff and clean production Wasm build passed. Optional stale module documentation was corrected; no behavioral review change. Full review retained at `/tmp/observation-820-attempt1-review.md` in coordinator evidence.

New reproducible worklet digest: `325bc4787f7661e53122649a4d53182efeb0131f6b00d776fa218fde1e5cbb39`. The additive 80-byte HostObservationResources field in HostPrepareReport is contained in PreparedHost, ReadyOwnership and AudioWorkletEngineHost; the existing sizeof-host-shell projection therefore increases bridgeMetadataBytes `1146127 -> 1146207` and bridgeRetainedBytes `1166636 -> 1166716`. Actual native layout corroborates the 80-byte row, and actual Wasm raw-oracle output differs only in those two derived rows; every PCM digest and other field remains identical. The existing resource checker passes actual Wasm/native witnesses plus 26 rejecting mutations. Existing C-ABI resource/lifecycle gates also pass 4/4, including independent exact/one-below cap accounting. All existing shipped-artifact browser gates pass Chromium 151.0.7922.34, Firefox 153.0, WebKit 26.5 and recorded the source candidate/digest. No new benchmark or fixture corpus. Required qualification run 34933354053 passed; PR #821 merged as e907b6323e396cd7f48079014360c9bbab1f1202 and GitHub closure was verified. This delivered native meter-only issue does not claim the browser has adopted demand subscriptions.
