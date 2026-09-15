# Native protected spectrum demand through the observation owner

Status: CLOSED through PR #823, merge 1d223493e68ee3d1a6cf7cfc5ca9d34f379ef88e. C1–C5 delivered; fresh Astra XHIGH attempt 1 PASS; required qualification 34943119276 succeeded at 0750426519060d1bac169106aa6716c1a381b4d5. GitHub closure verified 2026-09-15. Historical checkpoint sections below record evidence as it accumulated. Protected one-shot is deferred #824; the next native browser integration is #825. Full SDK/frontend production delivery remains open.

## Smallest closable capability

The native observation owner supports the existing fixed 2048-frame **continuous** spectrum: exact prepared-target replacement, same-target restart, reserved stop, generation-fenced reads and zero capture dispatch after applied stop. One spectrum producer may be accepted at a time, sharing the existing owner, graph activation endpoint and work budget with selected meters. UI demand changes never rebuild/swap an audio plan or reset audio DSP.

Scope: existing `crates/host-core/src/spectrum.rs`, `observation_demand.rs`, preparation/export glue and existing host/spectrum tests. Preserve legacy permanently bound single/collection and one-shot behavior. No browser/SDK implementation, resident-effect demand, overlapping FFT, configurable hop/FFT, simultaneous public jobs, new worker/transport, benchmark framework or native continuous result cache in this issue.

Protected one-shot admission/read and automatic completion cleanup follow the working browser EQ milestone in a bounded successor. Do not introduce placeholder protected one-shot methods now. The next browser milestone uses one exact TrackPostMatrix dual-mono continuous feed and already refuses protected one-shot aliases.

## Public host contract

```rust
pub enum HostSpectrumMode { OneShot, Continuous }
pub struct HostSpectrumDemand {
    pub target: SpectrumTarget,
    pub channels: SpectrumChannels,
    pub mode: HostSpectrumMode,
}
pub struct ObservedContinuousSpectrumWindow {
    pub owner: ObservationOwnerId,
    pub observation_generation: u64,
    pub selection_epoch: u64,
    pub window: SpectrumContinuousWindow,
}
pub enum HostSpectrumReadError {
    Inactive,
    PendingApplication,
    Warming,
    Pending,
    Closed,
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
}
impl HostObservationController {
    pub fn replace_spectrum(&mut self, demand: &HostSpectrumDemand)
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn restart_spectrum(&mut self)
        -> Result<ObservationAccepted, ObservationRefusal>;
    pub fn stop_spectrum(&mut self) -> Result<ObservationStop, ObservationRefusal>;
    pub fn try_read_continuous_spectrum(&mut self)
        -> Result<ObservedContinuousSpectrumWindow, HostSpectrumReadError>;
}
```

After the terminal-owner check, `replace_spectrum(OneShot)` refuses with `ObservationRefusalReason::InvalidRequest` and all optional details None, before target lookup, staging or publication. It preserves accepted/applied state, queues, work and identity counters. No new native refusal code; browser aliases later retain their specified Unsupported result. Do not add `ObservedSpectrumWindow`, `try_read_spectrum`, Invalid, WrongMode or CleanupRefused here.

Every successful replacement/restart activates a new graph generation, including the same target. `selection_epoch` starts at zero, first selection is 1, and changes only when the exact public target/channels entry changes. Mode/restart/stop do not advance it; failed publication commits neither this checked counter nor the last selected entry. Each capture activation starts its local continuous `stream_epoch` at 1 and sequence at zero. Complete identity is owner + observation generation + stream epoch; selection epoch describes public target selection separately. No cross-slot epoch allocator.

## Paired storage and capture lifecycle

Two internal preallocated slots per exact public entry are required because #816 does not invoke activation hooks for unchanged handles. A restart must switch slots at the admitted boundary, not mutate an active capture in place. These slots do not represent two supported public jobs.

```rust
pub(crate) fn prepare_controlled_capture_collection(
    request: &SpectrumCaptureCollectionRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<(Vec<GraphNodeObserverBinding>, ControlledSpectrumCaptureCollection), SpectrumPrepareError>;
pub(crate) fn controlled_spectrum_capture_collection_resources(
    entries: &[SpectrumCaptureCollectionEntry],
) -> Result<SpectrumCaptureResources, SpectrumPrepareError>;
```

Use the shared singular preparation implementation with private Permanent/Controlled binding policy. Existing callers remain Permanent; controlled slots use `GraphNodeObserverBinding::controlled`. Reject duplicate exact entries. Slot i uses checked handle `u64::MAX-i`; meter handles rise from 1. Check `2*entry_count`, subtraction, nonzero handles and combined disjoint ranges before creating spectrum bindings. No compiled track limit.

The collection owns a boxed array of entries, each containing exactly two existing `SpectrumCapture` values plus slot handles and independent scalar fields: staged, admitted_generation, applied_generation, retiring_at_revision. Public metadata comes from slot zero's immutable capture; no third identity string. A fixed two-index touched cache plus last accepted slot identifies all pending/active/retiring slots. Do not introduce another receipt queue or scan the whole catalog for retirement.

An affine candidate owns only scalar entry/slot/handle/mode/cadence metadata. Its `observer_handle()` and Copy `descriptor()` getters allow the host to prepare complete-set metadata before publication. Freeze these private operations:

```rust
stage(target, channels, mode, cadence) -> Result<ControlledSpectrumCandidate, SpectrumCaptureCollectionSelectionError>
commit_candidate(candidate, revision)
cancel_candidate(candidate)
commit_removal(revision)
reconcile_applied(revision)
```

Stage validates first and modifies only a Free slot. It may perform bounded control-side cleanup/reset, but leaves one-shot IDLE and continuous active=false. Continuous staged history is epoch1. Never call raw arm/start/commit_continuous/cancel/stop on an active or pending slot. A private stage-by-prepared-entry helper may share this path for restart without cloning a target string.

Graph publication is the last fallible admission operation. On refusal, cancel only the staged Free candidate. On success, commit candidate/old-slot retirement and host ledger by infallible bounded metadata writes. Private invariant failures must not silently skip post-publication bookkeeping. A pending start followed by removal keeps both revision fields until their actual receipts arrive.

For each consumed receipt, reconcile only the at-most-two touched slot indices: apply matching admission first; clean matching retired slots with at most one off-render queue pop each, then mark Free and compact the same bounded cache. No reuse before the removal receipt. The collection's last accepted slot may remain until its retirement receipt; the host accepted complete set is the admission/stop authority. Closure is terminal; disposal/reclamation stays off render.

`activation_changed(true,generation,first_sample)` sets the private observation generation, resets scalar capture/history/fault/sequence state and enables the prestaged mode. False disables only that slot and invalidates partial scalar history. Neither hook clears arrays, pops queues, changes targets, allocates/frees, resets audio DSP or performs locks/syscalls. Retained preallocated memory and bounded graph activation checks remain explicitly allowed.

Stamp `SpectrumCapturedRecord.observation_generation` at production through the shared borrowed buffer/finish family for both planar and resident inputs. Legacy captures retain generation0. Private record-returning helpers preserve identity before projection: `try_read_record(maximum_pops: Option<usize>)` and `try_read_continuous_record(maximum_pops: Option<usize>)`. None preserves legacy behavior; Some enforces the frozen pop budget, including zero. Keep existing failure/drop behavior.

## Complete sets, preparation and stop

Extend #820's existing five complete selection sets with one optional scalar spectrum descriptor/generation/selection_epoch each. Meter arrays remain capacity `min(prepared_meter_count, maximum_active_observers)`. Handle scratch uses checked `min(maximum_active_observers, prepared_meter_count + bool(nonempty_spectrum_catalog))`. Every admission checks the complete union; two internal capture slots consume only one active selection credit.

Both existing demand constructors admit nonempty controlled spectrum through their shared preparation/source bind. Empty spectrum normalizes absent; both catalogs empty retains the inert owner/no activation transport. Spectrum-only is a real controlled catalog. Resident-effect taps remain refused. Derive `SpectrumCadence::new(prepared_rate,Q)` once. Returned console handles expose no raw spectrum or selected-meter bypass. Clone graph target IDs only when actual spectrum preparation needs them.

Meter replace/remove preserves the accepted spectrum descriptor/generation; spectrum operations preserve accepted meters. Ordinary publication requires both pending records free, reserved removal only its own record free. No mutation polls receipts for capacity and no accepted obligation is coalesced away. Only `try_applied` consumes actual receipts, matches exact revision, reconciles spectrum and recycles that record, clearing optional selection metadata.

`stop_all` removes both families in one reserved snapshot; `remove_meters_to([])` alone now retains spectrum and cannot implement it. `stop_spectrum` removes only spectrum. If accepted spectrum is absent, and applied spectrum plus both pending complete selections are also absent, it returns Quiescent despite unrelated meter-only receipts. Otherwise reuse the highest-revision pending complete selection excluding spectrum; this preserves pending start+reserved stop before either receipt and repeated-stop identity. Quiescence never precedes required spectrum retirement. Renderer closure returns Closed without synthesizing Applied.

## Reads and retained results

Check terminal closure, accepted selection, then matching applied descriptor/generation before accessing a queue. Freeze selected-slot availability once, capped at one, and invoke the existing private record helper. Reject a record's observation generation before projecting its public window; stale discard returns Pending within the same pop budget. Never chase a producer refill. State refusals consume no queue or receipt.

Wrap Failed/Gap with owner and applied observation generation while preserving native stream epoch/drop values. Native NotActive maps Inactive; Warming/Pending map directly. Reads publish no removal or other mutation.

There is no native replay/cache guarantee. Returned windows remain caller-owned. A refused replacement preserves active producer state and unread queued data; an accepted replacement leaves the old active slot untouched until application, while new-demand reads return PendingApplication. ABI committed output and SDK last publication remain their own later responsibilities.

## Checked work and resource accounting

For one active capture, selected channels C, quantum Q, rate R and N=2048, reserve:

- active_spectrum_captures = 1;
- capture_input_samples_per_block = C*Q, including continuous WAITING finite validation;
- capture_copy_samples_per_block = C*min(Q,N)+4*N, including two full dual-plane payload copies;
- capture_publications_per_block = 1, including full-queue attempts;
- capture_bytes_per_second = (ceil(R/(ceil(N/Q)*Q))+1)*sizeof(SpectrumCapturedRecord), including restart alignment.

Use checked arithmetic. Add only these five fields to existing meter work plus fixed transition/retained reservation; never copy fixed rows from the spectrum argument again. All measurement/copy/publication costs are zero without active demand. A zero maximum_active_spectrum_captures work limit permits dormant preparation, but the graph's existing nonempty-catalog/zero-activation-capacity refusal remains. A limit above one does not enable multiple jobs. Bounds describe source operations/payloads, not universal CPU/deadline guarantees.

Charge two actual observers, queues, shared states, bindings and identities per entry, plus the concrete boxed entry array containing both control captures and lifecycle metadata. Do not add the legacy independent capture-value array. Inline collection/selection/cadence state is in actual owner size; no result payload cache. Graph activation charges its expanded catalog once. Keep #820 H-C owner and A-R graph overlaps: narrow reservation includes spectrum capture storage, while common graph/model admission already includes the spectrum report row and must not add it through owner metadata again. Derive actual layouts and named-allocation maxima; no copied native byte constants.

The approved resident continuous correction narrows its second extraction loop to `0..count`, preserving the full preceding selected-input finite scan. Small test-only TLS counters measure actual finite-validation samples, selected storage writes, payload constructions and publication attempts; no counter framework or benchmark runner.

## Bounded execution and closure gates

Use fresh disposable Luna MAX agents sequentially; root reviews, commits exact paths, pushes and synchronizes GitHub before the next tranche. One implementation attempt contains these checkpoints; final native adversarial verdict comes from a fresh Astra XHIGH agent.

1. **C1 capture identity/hooks — pushed.** Generation stamping, scalar activation reset, bounded raw record reads, resident extraction correction and operation probes. Both mode hooks have actual allocation/reallocation/free guards. Old queued generation and legacy behavior are preserved.
2. **C2 paired preparation/lifecycle — pushed.** Duplicate/handle/resource caps, staging/cancel preservation, independent pending+removal revisions, blocked early reuse, alternate-slot reuse and at-most-two-index reconciliation.
3. **C3 host preparation/storage/cost — pushed.** Spectrum-only/mixed/empty catalogs, both constructors, exact targets, no raw controls, dormant render probes, inclusive/one-below limits, checked work composition and retained accounting.
4. **C4 admission/receipts/stop — checkpointed.** Real nonzero A→B and same-A restart, atomic OneShot refusal, pre-boundary/failed-publication state and queued-data preservation, ordinary+reserved receipts at one boundary, family-preserving removal, repeated stop, actual zero dispatch after final stop and terminal closure. No public reads yet.
5. **C5 fenced continuous reads.** Pending-generation nonconsumption, stale-record rejection before projection, independent observation/selection/history identities (including stream_epoch1 across activations), queue-full/failure fencing, actual nonzero PCM bit identity and capture-site counts using existing bank-plus-tail and representative planar fixtures. Finish resident final-partial and Q>2048 operation-count cases here; they were deferred from C1 because direct resident lane construction is private, not waived. No new rack API/framework merely for fixture setup.

At closure run proportional existing host-core native/control-provider, shared preparation/compiler realtime and Wasm gates. Reuse actual allocation/free/lock/syscall guards and deterministic fixtures. No timed benchmark, second fixture corpus or broad new matrix. Root handles shipped artifact/layout integration, fresh native review, required PR qualification, merge/evidence and GitHub closure. Native issue completion is not browser/app deployment.

## Checkpoint evidence

- #820 foundation: PR821 merged e907b632; fresh native review PASS and required qualification succeeded.
- C1: b290ed41 production identity/hooks; 958c1e2e actual hook allocation guards and resident validation probes. Focused spectrum unit and nine existing spectrum integration cases passed; full direct resident count proof remains C5 as above.
- Continuous-first scope: fresh Astra recommendation approved and pushed d45ce3f9; protected one-shot/cache removed from this critical path.
- C2: 37b107ca. Root reran both controlled unit cases PASS; implementer checked host-core, all53 lib tests and nine spectrum integration cases, fmt/diff PASS. Real graph/queued-result integration remains C4/C5.
- C3: 0d2812a4. Root test-support observation integration16 PASS, including serialized constructor invocation and actual dormant render probes. Implementer reported no-default integration15, observation unit12, spectrum unit31, spectrum integration9, no-default/control-provider checks and fmt/diff PASS. Resource composition test includes nonzero meter/fixed rows and deliberately nonzero spectrum fixed rows to prove they are not charged twice.

Reproduce focused gates with `CARGO_TARGET_DIR=/home/bl/misofm/engine/target cargo test -p host-core --features test-support --test observation_demand`, `cargo test -p host-core --lib`, and `cargo test -p host-core --test spectrum`; use the same target directory for each Cargo invocation. Final issue-wide evidence and verdict are recorded below.

## Successor and next visible milestone

At this issue boundary, number the stateless protected-one-shot successor: admission/read, automatic reserved removal, one retained undelivered result, retry/exhaustion/closure semantics, actual storage accounting and representative contention gates. Rebrief its delivery contract; do not import a general continuous cache. Schedule it after working browser EQ.

Next deliver an additive opt-in protected EQ through the existing packed browser fixture: empty meter catalog, zero resident taps, one exact TrackPostMatrix dual-mono spectrum entry, bounded live-response capture, atomically refused raw Observe/Unobserve batches, and actual native generation carried through the existing worker/SDK owner. Legacy boot remains explicitly unprotected during that intermediate milestone; protected boot never falls back. Complete browser/SDK/adapter/app migration, resident feeds, overlap and multiple-job support remain part of the broader production rollout.

### C4 checkpoint evidence and consolidated read gates

Implemented continuous replace/restart/stop, shared complete-union publication, checked selection epochs, staged-candidate cancellation on graph refusal, family-specific and whole-owner stops, and exact-revision reconciliation. Public window record is declared for C5; no public read method or native cache yet. Root reviewed publication/cancellation and corrected OneShot refusal ordering for inert owners and complete-family applied-empty checks. Root reran the test-support observation integration suite:20 PASS; implementer reports no-default19, library test-support55, locked check and fmt/diff PASS.

The new cases prove restart/stop admission, repeated stop identity, pending start+reserved stop receipt order, meter/spectrum work preservation, atomic OneShot refusal including an inert owner, and zero capture operations after applied stop. Actual cross-target window generations, unread-queue preservation after graph-publication refusal, sibling generation continuity, and spectrum-specific renderer closure are consolidated into C5's real read fixture; they are not claimed complete by C4's work/receipt assertions. Those gates remain mandatory before issue closure.

### C5 checkpoint evidence

Added the public bounded continuous reader and typed state/failure/gap errors. Terminal and accepted/applied identity checks precede queue access; availability is frozen once at at most one record and generation checked before public projection. Actual integration fixtures cover target replacement/restart identities, pending receipt nonconsumption, source-seek failure/gap metadata, renderer closure, graph-publication refusal preserving the queued window, nonzero PCM bit parity, and Q192/Q4096 capture operation counts. Existing native stale-record fixtures cover bounded record rejection; no synthetic public-owner mutation seam was added. The sibling-publication case exercises admission but does not yet assert a completed surviving sibling window; fresh review must assess that remaining identity evidence. Implementer reports 56 library tests, 23 test-support observation tests (22 without), and nine spectrum tests passed. Issue-wide gates and independent verdict remain outstanding.

### Native candidate and browser compatibility evidence

C5 is pushed as de0fe11e; lint closure is ecbb449f3901cc3a5203c7e2bbad2526ebdcde01. Root gates passed: full host-core all-features and no-default-features suites, all-target/all-feature Clippy with warnings denied, compiler allocation/resource tracker nine cases, C API resource lifecycle four cases, host-core policy with negative controls, formatting and diff checks.

The reproducible shipped Wasm digest is 87555e1e58c38ad49e6a08cc56680f6a64f965f815174ceaa79d970228e500bf. The raw browser oracle remains exactly equal to expected.json, including PCM and resource rows. The existing Wasm/native resource checker and its 26 negative controls passed. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 existing qualification gates and mutations all passed; matrix records the source candidate above. This proves compatibility of existing browser delivery, not protected browser adoption. Fresh native review and required remote qualification remain pending.

### Independent native verdict

Fresh Astra XHIGH attempt 1: PASS, no native blockers, reviewed source ecbb449f3901cc3a5203c7e2bbad2526ebdcde01. Full findings are retained in docs/audits/822-attempt1-review.md. Root corrected the sibling fixture comment to match its current assertions; completed surviving-sibling result assertions are an optional extension when that fixture next changes. Native implementation and shipped browser compatibility integration are complete. Required PR qualification, merge and remote issue closure remain pending; this is not whole-product completion.
