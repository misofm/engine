# Native protected spectrum demand through the observation owner

Status: Astra XHIGH design approved by coordinator for five bounded Luna MAX assignments. Depends on closed #820 (PR #821, merge e907b632).


## Authoritative continuous-first contract

Fresh Astra XHIGH reviewed this reduction against the approved browser EQ contract; root approved it on 2026-09-15. This section supersedes every conflicting one-shot delivery/cache/API/gate requirement in the detailed preparation notes below. C2 private paired-slot machinery stays unchanged.

**Coordinator-approved scope: close #822 for protected continuous spectrum; protected one-shot follows the working browser EQ milestone.** Replace every conflicting one-shot/last-result requirement in #822 with this amendment. The closable capability is exact prepared-target continuous replace, same-target restart, reserved stop, generation-fenced continuous reads, and zero graph dispatch after applied stop. Preserve paired slots, private ownership, complete meter/spectrum unions, capture identity, budget equations, bounded retirement and receipt authority. Legacy permanent one-shot remains unchanged. C1/C2 may proceed unchanged; private one-shot machinery already shared with legacy capture need not be removed.

## Exact API and refusal

Keep `HostSpectrumDemand`, `ObservedContinuousSpectrumWindow`, `replace_spectrum`, `restart_spectrum`, `stop_spectrum`, and `try_read_continuous_spectrum`. Defer protected `ObservedSpectrumWindow` and `try_read_spectrum`; do not implement placeholder methods. Retain `HostSpectrumMode::{OneShot, Continuous}` if C2 uses it. Following the existing terminal-owner check, `replace_spectrum(OneShot)` returns exactly:

```rust
ObservationRefusal {
    reason: ObservationRefusalReason::InvalidRequest,
    limit: None,
    requested: None,
    maximum: None,
}
```

Refuse before target lookup, slot staging or graph publication; preserve accepted/applied state, queues, work and identity counters. No new native refusal variant or numeric mapping is needed. The protected browser endpoint continues its already specified `Unsupported` result before invoking C; its alias policy does not depend on native one-shot support.

## Last-result contract

**No native continuous result cache is required.** Continuous reads transfer a single generation-stamped window to the caller; neither approved browser contract requests native replay or last-result access. Their preserved previous values are existing ABI committed output and SDK publication state, maintained independently when admission/read does not succeed.

Delete #822's native tagged last-result-cell guarantee explicitly, including retention through accepted replacement. A refused replacement preserves the active capture and any unread queued record. An accepted replacement never resets the old active slot before application; subsequent reads report PendingApplication until the accepted generation applies. Successfully returned old windows remain caller-owned and distinguishable by identity. Stop/application fences future delivery and performs bounded retired-queue cleanup. No duplicated payload storage, cache validity flag or extra native result copy is necessary.

## Replacement handoffs and gates

**C3:** Keep real preparation, complete-set selections/scratch and checked meter-plus-spectrum cost composition. Omit inline last-result storage and pending-one-shot-delivery metadata; derive retained bytes from actual reduced layouts. Keep all existing C3 preparation, union-capacity, exact/one-below, overflow, no-raw-control and dormant-work gates.

**C4:** Implement continuous replace/restart/stop, meter-union preservation, `stop_all`, and bounded `try_applied` reconciliation. Remove result-cell initialization/invalidation and mode-transition behavior. Replace the mode-change gate with atomic OneShot refusal. Preserve A→B, same-A restart, no pre-boundary mutation, failed-publication queue/state preservation, ordinary-plus-removal receipts, repeated-stop identity, terminal closure and applied-stop zero-dispatch gates. Reads consume no receipts; only `try_applied` reconciles application.

**C5:** Implement only fenced continuous reading, with one entry-bounded queue pop, independent selection/generation/stream identities and existing Failed/Gap metadata. Retain applicable state errors; omit one-shot delivery and `CleanupRefused`. Remove automatic-removal retries, cleanup-admission exhaustion and undelivered-one-shot gates. Preserve stale-record rejection before projection, pending-generation nonconsumption, stream-epoch collisions, failed render/full queue, PCM parity, resident/planar operation counts, Q>2048, realtime guards and the proportional native/Wasm closure gates. Continuous reads never publish removal; explicit stop remains mandatory.


The protected read error for this slice is `HostSpectrumReadError::{Inactive, PendingApplication, Warming, Pending, Closed, Failed { owner: ObservationOwnerId, observation_generation: u64, stream_epoch: u64 }, Gap { owner: ObservationOwnerId, observation_generation: u64, stream_epoch: u64, dropped_captures: u64 }}`. No protected one-shot API, Invalid, WrongMode, or CleanupRefused variant is introduced here.

Successor scope, to be numbered at this issue boundary: protected OneShot admission/read, automatic reserved removal, one retained undelivered result, retry/exhaustion/closure semantics, exact storage accounting and representative contention gates. It follows the working browser EQ milestone and remains part of the overall production rollout. No native continuous replay/cache is promised.

## Detailed preparation and implementation notes


Depends on #820. Smallest closable result: the native demand owner supports the existing fixed 2048 one-shot/continuous spectrum and exact prepared-target selection, including same-target restart, with one admitted spectrum producer. Last stop removes graph dispatch. No overlap, custom hop/FFT, extra workers or simultaneous jobs. Scope: existing `crates/host-core/src/spectrum.rs`, `observation_demand.rs`, preparation/export glue and existing spectrum/host tests.

## Capture identity, ownership and render hooks

Root approved two slots as the smallest solution compatible with #816. #816 does not call activation_changed for an unchanged handle, so mutating/restarting an active capture in place would race render and bypass the admitted boundary.

Add crate-private controlled preparation and ownership alongside legacy types:

```rust
pub(crate) fn prepare_controlled_capture_collection(
    request: &SpectrumCaptureCollectionRequest, graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<(Vec<GraphNodeObserverBinding>, ControlledSpectrumCaptureCollection), SpectrumPrepareError>;
pub(crate) fn controlled_spectrum_capture_collection_resources(
    entries: &[SpectrumCaptureCollectionEntry],
) -> Result<SpectrumCaptureResources, SpectrumPrepareError>;
```

`ControlledSpectrumCaptureCollection` privately retains each public exact `(target,channels)` entry and TWO existing SpectrumCapture storage/queue/observer instances. Reuse the existing one-slot `SpectrumCapturedRecord` queue and scalar/array capture state; this is storage duplication, not a new transport. Legacy single/collection constructors still prepare their old one-slot-per-entry permanent bindings and retain their API. Controlled preparation calls a shared private variant of `prepare_capture_with_handle` selecting `GraphNodeObserverBinding::controlled`.

Stable public entry identity is its exact target+channel mask plus owner, not its internal slot. Internal handle for flat slot i is `u64::MAX-i`; meter handles rise from 1. Preflight total `2*entry_count`, every subtract and disjoint meter/spectrum handle ranges before creating bindings. Duplicate exact public entries remain a refusal. Charge TWO full captures/queues/bindings per entry, all control-side slot/entry/target storage and the expanded graph catalog exactly. Do not multiply the active spectrum work by two: only one slot is accepted active. Do not expose slots as two supported jobs.

Freeze slot lifecycle: `Free` (prepared, never active, or safely retired), `Staged` (candidate descriptor only), `Pending(revision)` (published obligation), `Active(generation)`, `Retiring(removal_or_replacement_revision)`. A slot becomes Free only after the graph receipt that removed that handle has been consumed, and its bounded one-item old queue has been cleaned OFF RENDER. That receipt proves no later observer call can touch the old slot. The new active observer may be rendering concurrently; never clean/reconfigure it. Pending ordinary and reserved removal each retain their obligations, so a slot admitted then removed at one boundary is still reconciled through both receipts. A renderer closure terminals outstanding slots and cleanup follows owner disposal off render. Implement admission/application/retirement as independent optional scalar fields (`admitted_generation`, `applied_generation`, `retiring_at_revision`) plus a staged flag, not an exclusive enum that would overwrite Pending when a reserved removal follows it. Thus a candidate can be admitted at revision r and scheduled for removal at r+1 while both receipts remain represented.

Staging operates only on a Free slot. It may reset that slot's bounded control-side queue, shared mode/cadence/epoch/failure metadata and read cursors before publication; it must leave its ARMED flag/shared.active false. It NEVER invokes existing raw `arm`, `start_continuous`, `commit_continuous`, `cancel`, `stop_continuous` on the currently active or pending slot. After all checks, graph publication is the sole commit. If publication refuses, return the candidate to Free; old selected fields, active shared values and result remain unchanged.

`SpectrumCaptureObserver::activation_changed(true,generation,first_sample)` sets its private observation_generation, resets only scalar one-shot/continuous cursors and fault/sequence history, and enables the PRESTAGED mode (`state=ARMED` for one-shot or `shared.active=1` for continuous). It performs no array zeroing or queue pop. False disables only that slot's state and invalidates scalar partial history; it cannot alter sibling slot state. Existing sample extraction/finite checks remain. All hooks are O(1). Add a private observation_generation to `SpectrumCapturedRecord`; add it to the observer/buffer/finish seams so records are stamped where produced. Existing fixed-demand captures carry zero. Exact resource projection uses new actual layouts.

Freeze the private function family rather than asking Luna to invent a staging protocol:

```rust
pub(crate) struct ControlledSpectrumCandidate { /* scalar slot/entry indices, mode, cadence */ }
impl ControlledSpectrumCandidate {
    pub(crate) fn observer_handle(&self) -> u64;
}
impl ControlledSpectrumCaptureCollection {
    pub(crate) fn stage(
        &mut self, target: &SpectrumTarget, channels: SpectrumChannels,
        mode: HostSpectrumMode, cadence: SpectrumCadence,
    ) -> Result<ControlledSpectrumCandidate, SpectrumCaptureCollectionSelectionError>;
    pub(crate) fn commit_candidate(
        &mut self, candidate: ControlledSpectrumCandidate, revision: u64,
    );
    pub(crate) fn cancel_candidate(&mut self, candidate: ControlledSpectrumCandidate);
    pub(crate) fn commit_removal(&mut self, revision: u64);
    pub(crate) fn reconcile_applied(&mut self, revision: u64);
}
```

The candidate is affine (not Clone/Copy publicly) and stores only fixed scalar metadata; it owns no render resources. stage performs every capture-specific check before changing the free slot. commit_candidate records its new Pending revision and marks the previously accepted slot Retiring at that revision; commit_removal marks the last accepted spectrum slot Retiring. Both are metadata-only and infallible after graph publication. reconcile_applied resolves exactly that revision's slot changes, cleans only the retired one-item queues, and makes them Free. The host's two pending records remain the receipt authority; the spectrum owner does not introduce a second queue. The candidate mode/cadence is immutable once staged through application. Free/retired-slot lookup is control-side and the existing singular capture helpers are reused privately; none of these functions is called from render.

For protected reads factor the existing try_read/try_read_continuous state machines into private record-returning helpers so generation is checked before converting away SpectrumCapturedRecord. Preserve their established failure/drop handling for legacy callers. Do not call the old public read method then guess a discarded generation from current shared state.

## Host operations, revision identity and cost

The controller owns the ControlledSpectrumCaptureCollection; it returns no raw SpectrumCapture/Collection. Extend preparation to accept Some(nonempty) through controlled preparation and the same artifact source bind. Normalize a singular request to a one-entry collection at any later compatibility adapter boundary.

Freeze additive host operations:

```rust
pub enum HostSpectrumMode { OneShot, Continuous }
pub struct HostSpectrumDemand {
    pub target: SpectrumTarget, pub channels: SpectrumChannels,
    pub mode: HostSpectrumMode,
}
pub struct ObservedSpectrumWindow {
    pub owner: ObservationOwnerId, pub observation_generation: u64,
    pub selection_epoch: u64, pub window: SpectrumWindow,
}
pub struct ObservedContinuousSpectrumWindow {
    pub owner: ObservationOwnerId, pub observation_generation: u64,
    pub selection_epoch: u64, pub window: SpectrumContinuousWindow,
}
impl HostObservationController {
    pub fn replace_spectrum(&mut self, demand: &HostSpectrumDemand)
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn restart_spectrum(&mut self)
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn stop_spectrum(&mut self) -> Result<ObservationStop, ObservationRefusal>;
    pub fn try_read_spectrum(&mut self)
        -> Result<ObservedSpectrumWindow, HostSpectrumReadError>;
    pub fn try_read_continuous_spectrum(&mut self)
        -> Result<ObservedContinuousSpectrumWindow, HostSpectrumReadError>;
}
```

`HostSpectrumReadError` distinguishes `Inactive`, `PendingApplication`, `Warming`, `Pending`, `WrongMode`, `Closed`, and the existing Invalid/Failed/Gap metadata wrapped WITH owner and observation_generation. Do not collapse telemetry faults into budget errors. The prepared host rate/Q fixes `SpectrumCadence::new(rate,Q)`; requested unsupported rate/quantum is not a new mutator in this API.

Replace validates the exact catalog entry/mode, prepares the Free alternate slot, reserves its work and one complete union/pending record, then publishes graph.replace retaining all accepted meters and replacing only the spectrum handle. `restart_spectrum` uses the same admitted target/mode/cadence with the other internal slot and is a real new activation, even for the same target. Same-target replacement also starts a fresh capture; adapters that want idempotent selection compare their accepted target first, preserving existing collection.select no-op semantics. A one-shot-to-continuous change uses the alternate slot; it never calls legacy commit_continuous on the active slot. Current and pending application metadata remain separate.

`selection_epoch` is a checked host-control counter, incremented only when the public target/channels changes, reserved before publication and committed after success. Observation generation is graph revision for EVERY new capture/restart/mode change. Existing SpectrumContinuousWindow.stream_epoch remains that capture activation's DSP-history/fault epoch, initialized at 1 and advanced by existing failure behavior; sequence starts at zero per activation. Its full identity is `(owner, observation_generation, stream_epoch)`. Do not attempt a concurrent cross-slot global stream_epoch allocator or compare local epochs from distinct generations. Legacy APIs retain their old semantics. Later ABI/SDK migration must transport the separate generation and fence it; this native slice does not reinterpret an existing frozen wire epoch.

Reads first require the corresponding generation applied, then pop at most the one queue slot available at entry. A slot's read is accepted only for that observation generation. Off-render changes of accepted demand do not destructively clear the last valid old result; it stays available in the owner's bounded retained last-result cell marked with its old generation until replacement applies or explicit stop cleanup retires it. New-generation `try_read_*` reports pending/warming and never presents that cached old result as current. The same slot cannot be reused until its removal receipt/cleanup. Failure/refusal tests inspect preservation of this cell.

`stop_spectrum` publishes graph.remove_to retaining meters; it disables no active state before that boundary. It uses reserved removal even with an ordinary pending replacement, and repeats share an identical pending stop receipt. After receipt, clean only the affected at-most-two slots (one pop each), clear/fence retained result and return Quiescent when no pending cleanup remains. It never scans every prepared target. `stop_all` removes both families in one removal snapshot. A completed one-shot remains a bounded owned request until read/cancel reconciliation submits its removal: after successfully taking its window, queue that removal through the same reserved lane, retain the returned window, and report pending cleanup if necessary. No result-read path may silently bypass or drop required stop. Quiescence is promised only after removal applies; inactive one-shot checks before that are not claimed as zero graph dispatch.

Worst-case C work for the ONE active capture, Q quantum, R sample rate, C selected channels, N=2048:

- `active_spectrum_captures=1`.
- `capture_input_samples_per_block=C*Q`: includes existing finite-validation reads even on a continuous WAITING block. Capture copying reads are charged separately below.
- `capture_copy_samples_per_block=C*min(Q,N)+4*N`: selected input-to-capture copies plus the two full dual-plane 2048-value payload copies at the publication seam. Record metadata is accounted in publication bytes; do not claim inactive channel planes disappear from the current fixed record layout. Count full-queue attempts too. Use existing operation-site probes to verify this model; if actual source-level payload copies exceed it, correct the frozen projection before acceptance, not the gate.
- `capture_publications_per_block=1`: current capture functions finish at most one window per callback, even if Q>N. This slice adds no catch-up loop.
- For rate budgeting under restarts, let `F=ceil(N/Q)*Q` with checked arithmetic. Charge `capture_bytes_per_second=(ceil(R/F)+1)*size_of::<SpectrumCapturedRecord>()`. This bounds any one-second publication interval including adversarial alignment and captures restarted earlier than the steady continuous hop; using merely 30 Hz would undercharge churn. The one-byte/generation metadata change follows actual size. It is a payload production budget, not browser serialization;03 budgets handler copies separately.

Add these costs to the active meter cost, and check every field BEFORE staging publication. Maximum capture count zero refuses start but permits capacity preparation; >1 in limits does not enable >1 product capability. For absent demand all measurement/copy/publication costs are zero while exact retained bytes and fixed transition allowance remain reserved. Candidate/browser profile values from the production plan remain unqualified until parent hardware/deadline evidence; no universal CPU claim is made here.

Native acceptance gates: A active/B dormant, A→B and same-A restart with nonzero signal; before boundary A unchanged and after boundary fresh generation/warmup; graph publication refusal preserves A state/result; pending start+reserved close yields both receipts then zero graph dispatch; removing spectrum retains meters and vice versa; last close, failed render, queue-full reader and generation collision fixtures. PCM bit identity, operation-site counts and no allocation/free/lock/syscall at activation/deactivation/render. Existing native/Wasm suites, no new benchmark infrastructure. Close C for native controlled fixed-spectrum support only.


## Coordinator-approved bounded implementation contract

The following decisions define the exact private seams and sequential task boundaries; they supersede any conflicting draft detail above. Execute five sequential disposable Luna MAX tasks; checkpoint each before the next.

# Protected native spectrum: bounded implementation handoffs

Astra XHIGH design frozen 2026-09-15 and reconciled by the coordinator against #820 source candidate a45aee3f. Extend its existing HostObservationController, MeterSelection, PendingApplication and ObservationBudget; do not restart ownership design. These are five sequential compiling checkpoints inside one implementation attempt, each for a fresh disposable Luna MAX. Root commits each focused-green tranche before the next. Root owns numbering, issue evidence, final fresh review and delivery.

The closable product remains C from `02-frozen-native-feed-slices.md`: existing fixed 2048-frame one-shot/continuous spectrum, two preallocated internal slots per exact target/channel entry, at most one accepted spectrum slot, the same private host owner and budget, generation-fenced reads, and zero dispatch after applied stop. Keep the frozen public constructors, demand/result records and methods. No overlap, configurable FFT/hop, simultaneous jobs, SDK/browser migration, resident-effect subscription work, worker/queue redesign, benchmark run or new harness. The existing `observe_resident` capture path is the banked audio input path and remains in scope; it is not resident-effect subscription scope.

## Source findings and root rulings already obtained

* `spectrum.rs:1076` `prepare_capture_with_handle` always constructs a permanent binding; add a private binding-policy parameter and keep every legacy caller Permanent. `SpectrumCapture::commit_continuous`, `cancel` and `stop_continuous` mutate shared state and use draining loops; protected admission must not call them on an active/pending slot.
* `SpectrumCaptureObserver` currently has no activation hook/generation. Both planar and banked capture paths reach `one_shot_finish`/`continuous_finish`; generation must flow through both. Record reads currently erase the private record identity; continuous reading loops after invalidated records. Protected reads must have a frozen one-pop budget before projection.
* **Approved root correction:** `continuous_capture_resident` currently reloads selected inputs for `0..frames`, even when `count < frames`. Narrow its second extraction loop to `0..count` and remove the inner `frame < count` checks. Retain the entire preceding selected-input finiteness scan and its `C*Q` charge. Captured PCM/validation stay the same; this makes C's frozen extraction accounting true for `Q > 2048` and final partial windows.
* **Approved root correction:** there are no existing spectrum operation-site probes, despite the draft wording. Add small thread-local, test-only counters in this module using the repository's existing test-support pattern; no harness or benchmark. Count validation samples, actual selected storage writes, full-record payload construction, and publication attempts separately. The existing graph dispatch counters and allocation/realtime guards remain the graph/realtime evidence.
* **Approved root public clarification:** add `HostSpectrumReadError::CleanupRefused(ObservationRefusal)`. A completed one-shot can meet a reserved meter-removal obligation, so its own automatic removal may refuse after a valid window is popped. Cache once in the required bounded last-result cell; set `pending_one_shot_delivery`; return the window only after its removal is accepted. Backpressure returns the existing `Pending` and preserves the undelivered window. Other cleanup-admission failures return `CleanupRefused`; Closed returns Closed and terminally invalidates delivery. Never translate revision exhaustion to endless Pending. Only `try_applied` consumes receipts; there is no retry queue or hidden receipt polling. A later read retries the cached delivery/removal; `stop_spectrum` exposes the same stored pending stop receipt or refusal. An explicit applied stop cancels undelivered data.

These are actual draft/source gaps, not reasons to expand C. No further architectural contradiction was found in the inspected capture seams.

## Small private seams to freeze before Luna

### Capture storage and scalar lifecycle

Use a boxed array of private entries, each containing exactly `[ControlledSpectrumSlot; 2]`. A slot contains its existing `SpectrumCapture`, precomputed graph handle and scalar lifecycle fields `staged`, `admitted_generation`, `applied_generation`, `retiring_at_revision`. Public target/channels can be read from slot zero's immutable capture: do not allocate a third identity string merely to mirror it. The two existing captures each keep their immutable target/channel metadata and queue. No runtime collection growth.

Keep the frozen `stage`, `commit_candidate`, `cancel_candidate`, `commit_removal`, `reconcile_applied` function family. Add only `ControlledSpectrumCandidate::descriptor(&self) -> ControlledSpectrumDescriptor`, an infallible getter, so the host can prepare its complete-set metadata before publication. A descriptor is Copy scalar data: public entry index, internal slot index/handle, mode. Its host selection adds observation generation and selection epoch. The affine candidate is consumed exactly once by commit or cancellation and owns no heap/render object. `stage` checks the exact entry, chooses only one of its two Free slots, prepares shared mode/cadence/local stream epoch 1/read counters, and leaves one-shot state IDLE and continuous active false. It never reads or resets the current active slot's state.

Keep a fixed two-element array of touched slot indices in the collection, plus an optional accepted slot index. It is a lookup cache for current slot metadata, not another publication/receipt queue. At most an old applied slot and its pending replacement/removal exist. `reconcile_applied(revision)` visits only these at-most-two slots: apply matching admitted generation first; then, for matching retiring revision, freeze/pop at most the one available queue item and clear its lifecycle to Free. Compact the index cache. Never scan the entire prepared collection for reconciliation/stop. Pending admission and later removal remain separate fields until both real receipts are consumed. Staged candidate space is checked before mutating the Free slot; cancellation returns it to Free.

Pass `observation_generation` through `SpectrumCaptureBuffers` and use that same small borrowed buffer bundle for planar/resident capture and both finish helpers. This avoids a separate state path and repetitive extra arguments. Activation true copies the graph generation, resets scalar partial/history/completion state, and enables the prestaged mode. Activation false disables only that slot and clears scalar partial/completion state. Neither hook touches arrays, queues, targets, heap ownership or the host ledger. Zero-initialized inactive channel planes remain safe because an internal slot's channel mask never changes.

Factor private record-returning read helpers beneath existing public raw reads. Add a bounded policy to the continuous helper: legacy callers retain their existing invalidation/drop behavior; the protected caller freezes `available_at_entry()` once and permits at most that one pop, returning Pending/Warming after discarding a stale record instead of chasing a refill. Check observation generation on the record before projecting either public window. Do not infer it from shared atomics. Use crate-private `try_read_record(&mut self, maximum_pops: Option<usize>) -> Result<SpectrumCapturedRecord, SpectrumCaptureReadError>` and `try_read_continuous_record(&mut self, maximum_pops: Option<usize>) -> Result<SpectrumCapturedRecord, SpectrumContinuousReadError>`: None preserves the legacy path, Some(available) enforces the protected entry budget including zero. Expose the record only crate-wide with generation and window-projection getters; no public raw-record surface is needed.

### Extend #820's existing complete sets and budget

Keep #820's five meter selection arrays. Add one optional scalar spectrum selection to each complete set (accepted, applied, candidate, ordinary pending, removal pending); do not allocate per-target pending arrays or create a second controller. The contiguous handle scratch must accommodate the complete legal union: `min(maximum_active_observers, prepared_meter_count + usize::from(nonempty_spectrum_catalog))`, checked before allocation. Meter array capacity remains its existing `min(prepared_meter_count, maximum_active_observers)`. Every admission checks the complete union count; two internal slots do not consume two active-demand credits.

Prepared spectrum slot handles are `u64::MAX - flat_slot_index`, checked and disjoint from all prepared meter handles before bindings are created. Meter replacements/removals retain the exact accepted spectrum descriptor and its generation; spectrum changes retain all accepted meters. Ordinary publication still requires both pending records free; reserved removal requires only the removal record free. No mutation polls receipts for capacity. `try_applied` remains the sole receipt authority and also calls the bounded spectrum reconciler for that exact revision.

Keep the collection and last-result cell inline inside the existing host owner, and count their actual containing layout through #820's owner-inline formula. A single tagged last-result cell holds one spectrum payload plus owner/generation/selection-epoch/mode/history metadata; it is not a per-target cache. Add a scalar pending-one-shot-delivery flag. Do not allocate a Box on each read. Preserve the old valid cell while a replacement is only accepted, and clear/fence it when that replacement applies or explicit stop cleanup retires it. New-demand reads never return the old cell as current.

Use one checked selection-epoch counter with the last successfully selected public entry index. No selected spectrum/empty catalog reads return Inactive; terminal closure returns Closed; then require accepted mode and matching applied generation before accessing its queue. Start at zero; first selection is epoch 1. Change it only for a different exact public target/channels entry; mode changes/restarts and stopping do not increment it. Failed publication commits neither counter nor last entry. Every activation gets the graph revision separately, and its local continuous history starts at stream epoch 1, sequence 0. Do not allocate a cross-slot epoch service.

Preparation uses the existing demand constructor and shared preparation transaction, enabling its already declared nonempty spectrum request. Empty meter plus nonempty spectrum is a real controlled catalog, not #820's inert owner case. Empty both remains inert. Derive cadence once from the prepared rate/Q. Keep resident-effect taps refused. The returned console handles contain no raw spectrum capture/controller.

Use the frozen C equations exactly after the approved resident-loop correction: one capture; validation `C*Q`; copy `C*min(Q,2048)+4*2048`; one publication attempt/block; bytes/sec `(ceil(R/(ceil(2048/Q)*Q))+1)*size_of::<SpectrumCapturedRecord>()`, with checked arithmetic. Charge publication attempts even on full queues. The extra metadata's real size follows the record layout. These are conservative source-operation/payload bounds, not machine memcpy counts or deadline guarantees. `ObservationBudget::preflight_graph_demand` must now compose its meter projection with only the five spectrum work fields; fixed retained/transition cost is charged once, not copied from the argument again. No public count above one enables another job.

Controlled resources charge two actual observer/queue/shared-state/binding/identity allocations per entry plus the concrete boxed entry array (which already contains both control-side captures and slot metadata). Do not also add an independent capture-value array. The inline collection/cache are already in the containing owner. Graph activation charges the expanded immutable catalog exactly once. Keep #820's graph runtime/controller overlaps and existing host spectrum row straight: narrow reservation includes capture storage; common graph/model admission already includes that row, so do not add it a second time through owner metadata. Exact/one-below tests derive actual layouts; no copied native-byte constants.

## Five sequential handoffs

### C1 — Record identity and render hooks

Paths: `src/spectrum.rs`, its existing unit tests, minimal test-support exports if needed. Implement generation stamping through the common buffer/finish family, constant-size activation hooks, private record-returning read helpers, the approved resident extraction correction and local counters. Existing preparations still bind permanently and old records use generation zero; protected host methods do not exist yet.

Focused gates: both planar/resident finish records stamp the hook generation; scalar reset discards partial history without clearing arrays; old queued record retains its generation; inactive hook state performs no capture work; legacy one-shot/continuous/drop/failure tests retain their behavior. Actual controlled graph zero-dispatch proof follows with controlled preparation in C2/C4; C1 must not invent a temporary binding path. Resident final-partial and `Q>2048` sample/copy counts remain mandatory in C5 using the existing banked host fixture; C1 does not add a rack test constructor or dependency just for private lane construction. Existing allocation guard around hooks. Compile/test this family and checkpoint; do not run a target matrix.

### C2 — Two-slot preparation and lifecycle ownership

Paths: `src/spectrum.rs` and its unit tests; declare the frozen `HostSpectrumMode` at its final host-facing location if needed. Implement the shared permanent/controlled preparation policy, exact controlled resource helper, paired slot collection/candidate, staging/cancel/commit/reconcile, and descriptor getter. No public host start/stop methods.

Focused gates: duplicate target rejection; two handle ranges/resource counts; exact/one-below capture/allocation caps; stage A while B active leaves B shared metadata/queue untouched; cancellation changes no accepted state; pending start followed by reserved removal retains both revision fields; each retirement pop is bounded and reuse is impossible before receipt; same-target alternate-slot reuse after cleanup. Test several dormant entries but assert reconciliation touches at most two indices. Compile/checkpoint before preparation integration.

### C3 — Real host preparation, complete-set storage and cost

Paths: `observation_demand.rs`, `prepare.rs`, `lib.rs`, existing focused preparation/observation tests. Enable nonempty controlled spectrum catalogs in both demand constructors; extend actual owner/complete-set/scratch layouts and narrow/common resource reports; implement checked spectrum cost composition. Preserve #820 meter methods with their new complete-set representation while spectrum is still initially absent. No fake/stub public spectrum methods.

Focused gates: empty meters/nonempty spectrum, both catalogs, both empty, exactly prepared targets, initially zero sample work, no returned raw controls, resident refusal unchanged; exact/one-below spectrum work fields, complete union capacity, overflow and cap accounting. Use real layouts and the existing preparation fixtures. Constructor refusal returns no partially prepared host. Compile/checkpoint.

### C4 — Host admission, receipts and stop reconciliation

Paths: `observation_demand.rs` and focused host observation tests, only small spectrum accessor adjustments justified by the frozen descriptor. Implement replace/restart/stop_spectrum and extend meter publication, try_applied and stop_all to complete unions. Include the required tagged result cell and invalidation points; production reads follow in C5. Validate request, union capacity, work, identity arithmetic and pending room before staging; prepare all commit metadata; graph publish last; cancel the Free candidate on graph refusal. Post-success commits are infallible scalar/fixed-array writes.

Focused real-host gates: A active/B pending then fresh B; same-A restart and mode change; no shared-state mutation before boundary; publication refusal preserves producer state/generation/result; one ordinary plus reserved removal yields two receipts even at one sample; removing either family retains the other; repeated stops reuse their receipt; last stop eliminates actual dispatch; closure is terminal. No graph rebuild/plan swap/DSP reset. Compile/checkpoint.

### C5 — Fenced results and native closure

Paths: same owner/spectrum files, `tests/observation_demand.rs`, existing `tests/spectrum.rs`, exports/docs/evidence necessary for C. Implement frozen read APIs/error type, owner/generation wrapping for Invalid/Failed/Gap, bounded stale discard, and the ruled one-shot cached-delivery/automatic-removal path. State errors do not consume queues or receipts. Preserve continuous history/failure semantics and last-result lifetime; generation, selection epoch and DSP stream epoch stay separate.

Focused gates: stale raw record rejected before projection; wrong mode/pending generation leaves queues alone; two different activations can both have stream epoch 1 without collision; failed render/full queue metadata fenced; valid cached one-shot + occupied meter removal returns Pending without losing data, then read retries removal and returns data once; cleanup revision exhaustion is CleanupRefused, closure Closed, explicit stop cancels undelivered data; successful read's stop returns pending until its real receipt. Complete PCM bit-identity and operation-site checks on the existing native bank-plus-tail fixture plus a representative planar output, including idle/staged/active/waiting/full/retired states. A short unit case covers Q>2048; no new fixture corpus.

At this coherent issue boundary run the existing host-core native/control-provider suites, proportional shared-preparation/compiler realtime gates and existing Wasm build once. Reuse graph dispatch/TLS probes and allocation/free/lock/syscall guards. No timed benchmark, generic counter framework, broad target matrix, or fixture expansion successor folded into C. Root records the native-only capability, obtains a fresh adversarial verdict, pushes/synchronizes/closes the numbered issue under the active delivery mode. This design author is not that final reviewer.

## Exact host spectrum read error

Freeze this additive enum in `observation_demand.rs`, exported with the other host observation records. Fault identity is the owner and applied observation generation being read; preserve the existing native stream epoch and cumulative drop value verbatim. No additional fault wrapper or selection-epoch field is needed.

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostSpectrumReadError {
    Inactive,
    PendingApplication,
    Warming,
    Pending,
    WrongMode,
    Closed,
    Invalid {
        owner: ObservationOwnerId,
        observation_generation: u64,
    },
    Failed {
        owner: ObservationOwnerId,
        observation_generation: u64,
        stream_epoch: u64,
    },
    Gap {
        owner: ObservationOwnerId,
        observation_generation: u64,
        stream_epoch: u64,
        dropped_captures: u64,
    },
    CleanupRefused(ObservationRefusal),
}
```

`Invalid` preserves `SpectrumCaptureReadError::Invalid`; `Failed` and `Gap` preserve the corresponding `SpectrumContinuousReadError` payloads. State variants and cleanup handling keep the rules above: cleanup Backpressure is Pending, cleanup Closed is Closed, and other cleanup admission failures retain their exact refusal in CleanupRefused.

## Delivery boundary

This issue closes for native controlled fixed-spectrum support only, after fresh adversarial review, proportional gates, merged evidence, and GitHub synchronization. Browser ingress, resident observation protection, SDK scopes, overlap, and simultaneous spectrum jobs remain separate successor slices.

## Next visible product milestone

After this native slice, the approved next milestone is an additive opt-in protected browser EQ path in the existing packed browser fixture, before the full resident migration: empty meter catalog, zero resident taps, one exact dual-mono trackPostMatrix spectrum entry, bounded live-response capture, atomically refused raw Observe/Unobserve batches. Existing boot remains explicitly legacy/unprotected during that intermediate milestone; eventual complete browser/SDK/adapter/app migration still remains required. This ordering does not add browser implementation to the present native issue.

## Implementation evidence — C1 checkpoint

Generation-stamped capture records, scalar activation hooks, bounded private raw readers, local operation probes, and the resident `0..count` extraction correction are implemented. Root reran the focused spectrum unit filter: 26 passed (`/tmp/observation-822-c1-unit.log`); the implementer also reported the nine existing spectrum integration cases, both feature checks, and realtime policy passing. This checkpoint is not issue completion. Direct resident final-partial/large-quantum operation assertions and an actual allocation guard around activation hooks remain to be added before C2; controlled graph dispatch proof remains in the later integration slice. The inactive planar continuous path now returns before finite validation, matching the dormant no-work contract.

### C1 focused gate follow-up

Added the missing one-shot resident finite-validation probe sites and actual allocation/reallocation/free assertions around activation, deactivation and reactivation in both capture modes. Disposable Luna MAX reported 26 spectrum unit tests and nine spectrum integration tests passing, plus fmt/diff checks. Direct resident lane construction is unavailable through host-core's current dependencies, so root moves those two operation-count cases to C5's existing banked host fixture; the issue closure requirement is retained. No new rack API or test framework is introduced. C2 may proceed after this checkpoint.

### C2 paired ownership checkpoint

Implemented private permanent/controlled binding policy, two preallocated capture slots per exact entry, concrete paired resource projection, affine staged candidates and bounded retirement/reconciliation. Root reviewed the lifecycle, preserved legacy capture-before-allocation refusal order, required explicit private invariant assertions after publication, and retained continuous local epoch1. HostSpectrumMode is declared/exported; no host start/read methods exist yet. Focused unit gates cover duplicate/one-below resource refusal, high handles, staging/cancel preservation, two pending revision fields, blocked early reuse and alternate-slot reuse. Root reran both controlled spectrum unit cases successfully (/tmp/observation-822-c2-focused.log); implementer reports cargo check, all53 host-core lib tests, nine spectrum integration tests and fmt/diff checks passing. Actual graph dispatch/queued-result preservation and resident operation counts remain integration gates in C4/C5; this checkpoint does not claim browser delivery.
