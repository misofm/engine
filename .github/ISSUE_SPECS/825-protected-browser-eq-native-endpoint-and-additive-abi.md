# Protected browser EQ native endpoint and additive ABI

Depends on #822. This is the smallest native browser capability needed for the first working protected EQ: one exact prepared stereo TrackPostMatrix target, continuous capture, bounded live-response capture, and truthful graph application receipts. It does not implement the JavaScript receiver, sender, SDK subscription adoption, frontend, resident feeds, protected one-shot, overlap, or multiple jobs. Those remain subsequent vertical-slice issues.

Coordinator brief: approved Astra XHIGH design, root source-grounded rulings, and sequential bounded fresh Luna MAX implementation. Fresh Astra XHIGH reviews the completed native/ABI candidate. Maximum five attempts; one compiling exact-path checkpoint at a time, root commit/push and GitHub sync before another tranche. Never rebuild an audio graph for demand, allocate/free/lock/wait/perform I/O in render, execute FFT in the worklet, or introduce another subscription owner/worker. Retained prepared storage and scalar activation/epoch checks are allowed and charged.

Scope: hosts/host-web native implementation/FFI, parameter-metadata ABI generators, existing focused fixtures and source-derived artifact/layout evidence. Existing native #822 source is a dependency, not a redesign target. A narrow additive read-only spectrum status projection in crates/host-core/src/observation_demand.rs and lib.rs is permitted for real accepted/applied generation and persisted selection epoch; it must read existing scalar state without polling, mutating, or creating another ledger. Keep old public ABI layouts/numbers intact.

Explicit ingress record (all limits inclusive; no permissive defaults):

```rust
pub struct ObservationIngressLimits {
    pub maximum_control_bytes: u32,
    pub maximum_observation_rows: u32,
    pub maximum_result_bytes: u32,
    pub ordinary_operations_per_boundary: u32,
    pub removal_operations_per_boundary: u32,
    pub maximum_admission_entry_visits: u64,
    pub maximum_response_binding_visits: u64,
    pub maximum_response_section_visits: u64,
    pub maximum_response_copy_bytes: u64,
    pub maximum_handler_copy_bytes_per_boundary: u64,
    pub maximum_cleanup_entry_visits_per_boundary: u64,
    pub maximum_retained_bytes: u64,
}
```

## Preparation and private ownership

```rust
pub enum WebObservationProfile { EqSpectrum }
pub struct WebObservationPreparation<'a> {
    pub profile: WebObservationProfile,
    pub demand: host_core::HostObservationPreparation<'a>,
    pub ingress: ObservationIngressLimits, // explicit fields below
}
impl AudioWorkletEngineHost {
    pub fn boot_with_observation_demand(
        document: &[u8], options: WebBootOptions,
        preparation: &WebObservationPreparation<'_>,
    ) -> Result<Self, BootFailure>;
}
```

Require empty meters, zero resident taps, `console_meter_blocks=0`, `console_master_track_plus_one=0`, and exactly one spectrum entry: TrackPostMatrix/Both for the prepared track (`track` in the fixture). Response requests address that same track. Require activation maximum_active_observers=1 and explicit #822 work/retained limits; no permissive defaults. W2 ordinary/removal counts are exactly 1; inclusive control/row/result ceilings are at most 8192/32/65536, with every supplied visit/copy/cleanup/storage limit verified against preparation-derived bounds. Preserve console audio queues and controls.

Reuse the boot transaction and `prepare_host_runtime_with_observation_demand_between_render_calls`. Replace `ReadyOwnership.spectrum_capture` with a private sum: Legacy(optional existing capture) or Protected(controller, ingress scalars, cached bounds, side records). Never retain a raw capture alongside the protected controller. Charge actual containing-layout deltas and staging capacities once; keep #820/#822 overlap corrections.

Old Rust/ABI boot stays explicitly legacy/unprotected. An explicit protected boot failure never falls back. Capability status identifies the path independently of old status/options layouts.

## Small admission and receipt seams

Private `ObservationIngressState` holds epoch (initially 1), ordinary/removal-used bits and cached limits/bounds. `begin_observation(class, lengths) -> ObservationPermit` consumes one attempt before semantics/lookup/capture; refusals keep it consumed. The affine scalar permit is crate-private, cannot be constructed by callers, and is passed through nested helpers. `on_successful_render` checked-increments the epoch and clears bits; exhaustion permanently refuses new attempts. A failed render grants nothing.

Freeze three additional private helpers: `reserve_receipt(class)->Result<usize,ObservationRefusal>`, `commit_receipt(slot,accepted,operation)` (infallible), and `copy_response_snapshot_admitted(permit,track_id,sink)` (never reacquires credit). Public generic capture and the ABI concrete sink enter through the same permit; ABI preflight precedes sink construction. A refused graph publication releases only its unused receipt reservation.

Existing spectrum start/restart/read/stop methods and supported continuous exports use this path. Start makes ONE `replace_spectrum(Continuous)` using the prepared entry; restart calls C's restart; stop uses reserved removal. One-shot arm/read/cancel and collection select/stream-select refuse Unsupported before capture/staging. Worker-only analysis/import remains separate. Meter activation/selected reads refuse Unsupported; cached empty catalogs and absent frames remain honest. No output-peak scan runs.

Before `admit_commands`/prepared-companion parsing or shadow staging, scan only bounded raw kind words. Any Observe/Unobserve refuses the ENTIRE batch with the first offending original index; no publication, revision or audio-shadow mutation. Consume the appropriate attempt once classified; audio-bearing batches are ordinary. Never lower an observation record. The existing staging-capacity-bounded classification scan remains necessary even with exhausted observation credit to preserve audio-only admission. Account/report that remaining command-ingress cost separately; **this path does not promise scalar-only refusal or a new per-boundary raw-command scheduler**.

Use four fixed receipt slots in the existing host side-record storage, not another transport or demand ledger. Reserve before graph publication: ordinary requires its slot plus one remaining removal slot; removal requires one free slot. Fill Pending only after accepted publication. Slot identity is owner/domain/sequence; repeated pending stop returns its original receipt without another attempt. Capacity refusal preserves every accepted receipt.

`reconcile_observation_applications` calls C's `try_applied` at most twice, updates matching slots, and performs C's at-most-two-slot retirement. Call from bounded application-take between render calls, including after failed render, **outside** the realtime `render_next` region. Never poll receipts inside admission. Taking completed rows releases their storage; pending rows persist. Actual graph receipts remain Applied even if subsequent audio failed. Status failure alone does not invent application or closure; explicit terminal disposal marks only remaining Pending rows Closed/Failed and exposes them before destroying ownership. Applied rows survive unchanged. Charge simultaneous reconciliation/copies/cleanup; retained rows cannot be overwritten.

## Exact additive records and exports

All records use `repr(C)`, leading `struct_size:u32, abi_version:u32`, ABI_VERSION and zero reserved fields. Wire constants/layouts come from Rust metadata generation, never handwritten JavaScript offsets.

* `WebObservationPreparationRecord`: `profile, meter_count, resident_taps, spectrum_count, maximum_active_observers, reserved0:u32`; `activation_maximum_retained_bytes:u64`; embedded `WebObservationWorkLimits` (all eleven #822 `ObservationWorkLimits` fields, each u64), `WebObservationIngressLimits` (all twelve W2 fields, same widths), existing `WebSpectrumRequest`, and `target_id:[u8;128]`. Profile=1 means EqSpectrum; counts must be 0/0/1; target length comes from the embedded request, at most127, with zero remaining bytes. Fixed staging needs no variable catalog allocation. Translate activation count to checked usize.
* `WebObservationDemand`: `operation,count:u32; owner:u64; reserved:[u32;2]`. Operations 1 ReplaceMeters, 2 RemoveMetersTo, 3 StopGraph; only 3/count0 is supported here. It calls the same reserved stop path. Unsupported operations consume their class attempt.
* `WebObservationReceipt`: W3's exact fields: `domain,state:u32; owner,sequence,application_sample:u64; result,reserved:u32`. Domains Graph=1/Resident=2; states Pending=1/Applied=2/Closed=3/Failed=4. Only Graph occurs. Pending graph sample is zero/invalid; Applied uses C's actual first_sample. No response receipt.
* `WebObservationAdmission`: `result,operation,flags,reason,limit_bytes,reserved:u32; ingress_epoch,requested,maximum:u64; limit:[u8;128]; receipt:WebObservationReceipt`. Flags bits0–3 mean receipt-present, requested-present, maximum-present, pendingBoundary. Reasons 0=None, then 1–10 match the named native refusal variants in declaration order; generated metadata owns the mapping. `limit` is the exact native/ingress field spelling, length-qualified and zero-padded. Refusal writes only this side record.
* `WebObservationStatus`: `profile,flags,pending_count,reserved:u32; owner,ingress_epoch,accepted_generation,applied_generation,selection_epoch:u64`. Profile0=LegacyUnprotected/1=EqSpectrum; flags bits0–3 ordinary-available/removal-available/terminal/render-failed. Generation0 means no selection; selection epoch remains C's counter. Pending count is cached.
* `WebObservationCaptureIdentity`: `kind,flags:u32; owner,observation_generation,selection_epoch,snapshot_token:u64`. Kind1=Response/2=Spectrum; flags bit0 validates graph generation. Response uses owner+existing checked snapshot token, flag0 clear, generation/selection0. Spectrum uses C's actual generation/selection plus existing token; old stream/history epochs remain unchanged. Publish this side record atomically with successful raw output, carry it beside worker payloads, and preserve it on admission refusal.

Add `miso_engine_web_v1_observation_preparation_{ptr,bytes}()` and `boot_with_observation_demand(len)->handle`. Add `observation_demand_{ptr(handle),bytes(),capacity(),apply(handle)}` (capacity=0 meter rows); `observation_admission_{ptr(handle),bytes()}`; `observation_status_{ptr(handle),bytes()}`; `observation_application_{take(handle),ptr(),bytes(),capacity()}` (take returns completed row count, capacity=4); and `observation_capture_identity_{ptr(handle),bytes()}`. Application staging is the persistent final handoff during disposal too. Read-only getters consume no capture permit. Admission operation tags continue demand tags1–3 with StartSpectrum=4, RestartSpectrum=5, ReadSpectrum=6, StopSpectrum=7, CaptureResponse=8, RawObservationBatch=9, OneShot=10, CollectionSelection=11, MeterLease=12, MeterRead=13, ResidentRead=14. No grant-credit/mark-applied export. Old records, exports and numeric results remain unchanged.

## Response bound and compiling handoffs

Acquire the common permit and check `live_token+1` BEFORE `LiveResponseCaptureSink::new` or any error/result write. Reserve full prepared binding traversal, including rejected/missing-target paths, selected owner/section maxima, identity comparisons/copies, real header/failure writes and outgoing copies. Existing packing reserves all256 owner records (16KiB) even for one track; retain it. Constructor currently performs no clearing: charge actual operations. Freeze conservative projection: B=all compiled tracks+effects bounds binding visits; U=1+selected-track effects bounds owners (require U<=256); N=RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS. Charge 8*U*N section visits for two-plane scratch initialization, extraction, validation and packing. Packed maximum is sizeof(result)+256*sizeof(owner)+U*(3*127+2*N*sizeof(section)); all arithmetic checked. Copy reservation additionally covers native scratch/word construction, headers, and outgoing copies, using actual Rust sizes. The fixed inner word bound is RESPONSE_SNAPSHOT_WORDS. Derive identities off-render; no new graph index. Admission refusal preserves committed bytes/length/token. The generic Rust sink method shares engine-owned traversal bounds but cannot qualify arbitrary caller code; the concrete existing ABI byte sink is the qualified path.

1. **Rust owner/permits:** preparation, bounded projection, raw refusal, response preflight, continuous mediation and private receipt reservation; focused real-host gates, then checkpoint.
2. **ABI staging/receipts:** records/exports, protected boot, alias mediation, output identities and failed-render receipt preservation; direct ABI pending/start/stop tests, then checkpoint.
3. **Metadata closure:** existing `abi_layout.rs`/metadata export tables and normal generator, large-u64 round trips, unchanged-old-layout checks, representative native/scalar/simd128 and realtime gates; checkpoint. Regenerate shipped assets once with the later coherent browser batch.

No unresolved prerequisite beyond completion of #822. Coordinator explicitly approved the raw-scan qualification limit, concrete-sink scope, packed-layout accounting and separate response identity. Sender/SDK/resident/FFT/multijob work remains outside this contract.

## Bounded delivery and evidence

Split Rust checkpoint1 further before handing to Luna: (a) private preparation/storage and checked projections with no externally callable protected boot yet; (b) protected operation mediation/permits/receipts and public boot only when all aliases are guarded. Each compiles and has focused gates before root checkpoints. ABI staging and metadata then follow sequentially. This is a checkpoint split, not a new architectural option for implementers. Root supplies exact allowed files and method list for each task.

Use representative actual host and ABI fixtures: dormant capture, accepted pending before render, actual nonzero generation-bearing window, reserved stop application, ordinary/refusal budget exhaustion without audio/source loss, whole mixed raw batch refusal preserving shadows and queues, response refusal preserving committed bytes/token, both receipts when one boundary applies start and stop, failed-audio graph receipt preservation, terminal disposal, exact/one-below checked storage and copy limits. Reuse existing allocation/static realtime gates and generated ABI checks; source-derived scalar/simd128 builds and the normal browser artifact/layout qualification are required. No new benchmark machinery or claim of universal browser deadline isolation.

Closure requires independent review PASS, required CI, merged source/evidence and synchronized GitHub state. It proves the native endpoint/ABI, not a working browser UI. The next issue connects existing receiver/sender/SDK and packed EQ fixture before resident qualification expands scope.

### Root source seam ruling

The current native owner exposes owner/work/closure but no accepted/applied spectrum state getter. Add `HostSpectrumState { accepted_generation: u64, applied_generation: u64, selection_epoch: u64 }` and `HostObservationController::spectrum_state(&self) -> HostSpectrumState` as the narrow read-only projection needed by the browser status. Missing accepted/applied selections project generation0; selection_epoch is the existing persisted counter. No receipt polling, graph query, queue read, new stored counter, or mutable status cache. Export the type through host-core. It may land with preparation checkpoint (a), and existing owner fixtures can assert the scalar projection at their existing boundaries.
