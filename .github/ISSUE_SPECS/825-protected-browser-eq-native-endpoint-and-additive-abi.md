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

## Storage checkpoint and bounded rescope

The initial private-preparation assignment was reduced before it grew further. It delivered the public preparation/limit types, private mutually exclusive Legacy/Protected storage, existing legacy method forwarding, and the native read-only HostSpectrumState getter. Root completed the existing constructor adaptation. Protected preparation and public protected boot are not enabled. Narrow temporary dead-code annotations identify storage to be wired in subsequent checkpoints; the inline owner is intentional and the actual containing allocation is charged.

Root's eight existing spectrum ABI cases passed, including continuous and collection behavior; formatting/diff and warning-denying host-web library Clippy passed. Shipped artifact repinning waits for completed native/ABI implementation. No new native review attempt has begun.

Source-derived overlap ruling: project_buffers charges the full AudioWorkletEngineHost shell, now containing the full native owner H. Native graph_session_plus_plan_bytes already includes H through (H-C)+(A-R), where A contains C. Protected bridge metadata/retained accounting must deduct full sizeof(HostObservationController) exactly once, not just the native owner_inline_bytes H-C. Largest allocation retains full actual shell size; the browser target identity allocation is separate and charged once. Legacy has no owner deduction.

The remaining preparation work is split into exact checked ingress projection, private boot wiring, then protected operation mediation. Astra XHIGH is supplying the missing code-ready projection arithmetic so fresh Luna tasks do not have to discover architectural budget rules while implementing. This refines execution order; the frozen endpoint contract is unchanged.

## Frozen source-derived ingress projection


Scope: one prepared TrackPostMatrix/Stereo target, zero meters/resident taps/permanent
observers, one active observer, two internal slots, one ordinary plus one removal
attempt per ingress epoch, four retained browser receipt rows. This is a source-derived
logical work reservation, not a compiler instruction/memcpy count or a deadline claim.
All arithmetic below is checked u64 arithmetic; conversion, subtraction, multiplication,
addition, or Layout overflow refuses preparation. No saturating arithmetic or defaults.

## Inputs and literal helper arithmetic

Use `project_observation_ingress(shape, sizes, bridge, native_reserved)` returning
`ObservationIngressBounds` plus packed result maxima and the separate per-call facts.
`sizes` is an explicit private argument until the ABI types land, then fill it with
their real `size_of`/`Layout` values. Do not substitute assumed numeric ABI sizes.
Names below denote bytes: H=sizeof(HostObservationController), NR=sizeof(ResponseSnapshotSection),
AR=sizeof(WebLiveResponseSection), OR=sizeof(WebLiveResponseOwner), HR=sizeof(WebLiveResponseResult),
LR=sizeof(WebLiveResponseRequest), SR=sizeof(WebSpectrumRequest), SH=sizeof(WebSpectrumWindow),
SM=sizeof(WebSpectrumStreamMetadata), SW=sizeof(SpectrumWindow), CW=sizeof(SpectrumContinuousWindow),
Z=sizeof(ObservedContinuousSpectrumWindow), AC=sizeof(ObservationAccepted),
AP=sizeof(ObservationApplied), WC=sizeof(ObservationWorkCost), DR=sizeof(WebObservationDemand),
RR=sizeof(WebObservationReceipt), CI=sizeof(WebObservationCaptureIdentity),
DA=sizeof(WebObservationAdmission). The future ABI names are explicit size parameters.
Z safely bounds a private SpectrumCapturedRecord copy: the same planes/channel/underrun
fields plus seven u64 identity/timing words in Z versus five in the private queue record.

```text
E=1; S=2; A=1; P=2; R=4; Q=1; O=2; F=4; L=127; K=S+1;
B=all_compiled_tracks + all_compiled_effect_instances;
U=1 + selected_track_effect_instances; require U<=256;
N=RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS;
W=RESPONSE_SNAPSHOT_WORDS * sizeof(u32);
PCM=2 * SPECTRUM_WINDOW_FRAMES * sizeof(f32);
packed_response=HR + 256*OR + U*(3*L + 2*N*AR);
packed_spectrum=SH + PCM;

admission_entry_visits=O*(3*R + 4*P + E + 5*S + A*(8+2*K) + 2*Q); // 94
response_binding_visits=B;
response_section_visits=8*U*N;
response_copy_bytes=2*LR + 2*(B+2)*L + 8*U*N*(NR+W+AR)
                    + 2*U*OR + 3*U*L + 2*HR + 2*packed_response + 4*CI;

spectrum_copy=6*Z + 2*CW + 2*SW + 2*PCM + 2*SH + 4*SM
              + 2*packed_spectrum + 4*CI + 2*SR + 2*L;
control_copy=4*H + 8*AC + 8*WC + 4*RR + 2*(SR+DR+L) + 2*Z;
receipt_copy=F*(2*H + 4*AP + 4*RR) + 4*(R+F)*RR;
retirement_copy=(F*S*Q + S*Q)*Z;
handler_copy_bytes_per_boundary=response_copy_bytes + spectrum_copy
                                + O*control_copy + receipt_copy + retirement_copy;
cleanup_entry_visits_per_boundary=F*(1+P+2*S+S*Q+R) + 2*R*(R+F)
                                  + (P+R+2*S+S*Q) + 2*Q+S; // 132

corrected_bridge=bridge.projected_full_retained + bridge.additive_staging_bytes
                 + bridge.private_target_allocation_bytes - H;
retained_bytes=native_reserved + corrected_bridge;
```

The handler sum deliberately reserves response, spectrum read, both control attempts,
receipt handoff, and retirement together although ordinary credit excludes several
combinations. This avoids a max-of-mutually-exclusive-branches implementation choice.
Both packed maxima must fit the supplied maximum_result_bytes (itself <=65536).
Require explicit W2 counts 1/1, control/row ceilings <=8192/32, and compare every seven
derived bounds to its corresponding inclusive supplied limit before publishing ownership.
Validate fixed request byte lengths against maximum_control_bytes at operation entry.

## Actual-loop justification and required small helper behavior

Admission: allow at most two R-row receipt-reservation passes and one R-row pending-stop
identity pass per permitted operation. Four P passes over-reserve native terminal/pending
stop searches. E is the exact prepared-entry lookup. Five S passes cover staged-slot
search, free-slot search, touch membership, debug previous membership, and rollback
compaction (success and rollback are both reserved). Native handles have length <=A;
duplicate inner scans are empty, sorts of length <=1 have no comparisons, and each of
the two catalog binary searches has <=K comparisons, including its final comparison.
Eight A visits cover outer validation/writes/copies and these trivial sort boundaries.
Two Q pops cover stage reset and failed-publication rollback. Meter loops visit zero.
Unsupported meter/collection/one-shot requests refuse before any supplied-row traversal.

Response: graph runtime filters the entire prepared binding array, including missing
target/error paths. Identity work reserves two operands for every bounded comparison,
two additional target checks, and three copied IDs per emitted owner. Reject length>L
before UTF-8/identity work, after obtaining the common permit. The 8*U*N passes cover
two-plane initialization, extraction, validation and packing; every pass reserves a
native section, its fixed word construction, and a wire section, conservatively even
when that pass uses fewer bytes. Owner/header construction and writes are separate.
The sink constructor reserves 256 owner rows but clears none: their space is charged
in packed size/outgoing copies, not fictitious initialization. Reserve two full outgoing
payload copies and identity construction/publication/handoff. Generic caller sink work
is outside qualification; the existing concrete ABI sink is the qualified path.

Spectrum: Q=1 comes from continuous_available_at_entry().min(1), stage reset's one pop,
and retire_controlled_after_receipt's one pop. Six Z copies conservatively cover queue
extraction, native return wrappers, observed-window construction and host/ABI handoff;
CW/SW cover continuous/as_window projections; 2*PCM covers f32 byte construction and
packing. Header/metadata/identity construction and two outgoing payload copies are
reserved explicitly. Control's four H copies bound the graph publication entry/record
construction and transfer payloads without exposing private graph layouts; eight AC/WC
copies cover candidate/pending/accepted/work-return records. Four RR covers receipt
construction, storage and acknowledgements. This counts record payload, not scalar
register arithmetic. Two Z reserves reset plus rollback queue extraction.

Cleanup: F=4 allows two carried native receipts from the preceding successful boundary
plus two current-credit publications applied by a failed render before epoch advance.
No render success means no new operation credit; later successful render starts a new
epoch. Each successful reconciliation visits one graph receipt, P pending matches,
S touched and S compaction entries, at most S*Q retired queue items, and R browser
receipt matches. Receipt-copy's H bound covers native retirement record ownership moves.
At most R+F distinct handoff rows exist when carried rows and current obligations are
over-reserved together; each nonempty take may scan R rows and clear R staging rows.
Use cached pending/completed scalars: empty calls must not repeat R-row scans or clears.
Call C.try_applied at most twice per take; a missing receipt performs only fixed scalar
queue/closure work and no matching/retirement scan. Terminal processing is one-shot;
reserve P native pending clears, R browser closures, two S slot passes, and S*Q pops.
The final 2*Q+S additionally reserves admission reset/rollback/compaction in cleanup.
Never poll receipts in admission. Preserve graph Applied even when audio later failed.

## Ownership overlap and costs outside the payload reservation

`projected_full_retained` is project_buffers with the actual final host shell and
configured spectrum staging (collection-entry/ID arrays remain zero). `additive_staging_bytes`
is only actual new allocation payloads plus actual containing-layout deltas not already
included by that projection: preparation/demand/admission/status/identity, four receipt
slots and persistent application staging as actually placed. Inline shell records are
already in sizeof(host); do not add them again. Use real Box/Vec capacities and final
TLS/container deltas. Charge the private browser target allocation exactly once; it may
be owned by a retained HostSpectrumDemand whose target is also borrowed for response.
The broad ingress retained ceiling intentionally includes existing bridge/audio/document
staging. Native reserved already includes H through (H-C)+activation; deduct full H,
not H-C. Full aggregate host admission separately adds its native graph/runtime model
to corrected_bridge; do not additionally add native_reserved to that full host total.
Largest allocation stays max(actual full host shell, existing staging maxima, each new
actual allocation, private ID allocation); never deduct H from an allocation maximum.

Exhausted attempts still return typed refusals. Fixed scalar checks and fixed admission
diagnostic writes are outside per-boundary payload-copy limits: report DA bytes per
diagnostic record write, with <=2*DA for construction plus publication per call. Likewise
report raw-command classification separately: <=existing staging record capacity kind
visits and sizeof(kind)*record_count bytes per call. Scan only bounded kind words before
ordinary admission/prepared-companion parsing/shadow mutation, including when observation
credit is exhausted; preserve audio-only admission and first offending original index.
An observation-bearing refusal consumes its classified attempt once, then does no deep
work without credit. Scalar getters and repeated-stop receipt identity checks are also
fixed per-call work; repeated pending stop returns its existing receipt without credit.
Owned sender/receiver throttling bounds entry frequency; arbitrary caller floods have
no universal deadline qualification. No scheduler or suppression of typed refusals.

### Projection implementation order

To use actual ABI sizes immediately, the next bounded checkpoint declares only the four scalar ABI records referenced by the projection (WebObservationDemand, WebObservationReceipt, WebObservationAdmission and WebObservationCaptureIdentity) and implements the frozen checked projection helper. No exports, FFI staging, receipt mediation or protected boot are enabled in that checkpoint. These are the final canonical record types, not mirrors or guessed byte constants. Private boot wiring follows using this exact validated projection. This further splits implementation to keep Luna tasks bounded; public protected boot still waits for complete alias mediation.

## Checked projection checkpoint

The four canonical demand/receipt/admission/capture-identity records are declared with their final repr(C) fields. The private observation_ingress module computes the frozen checked bounds and separate per-call facts using actual Rust sizes internally; callers cannot provide a layout-size table. Limit validation is a separate helper that must run before publishing protected preparation. The minimum control capacity covers the largest supported fixed request plus a 127-byte identity, and the minimum row capacity is four receipts. Invalid profile ceilings/counts use REFUSED_OPTIONS; insufficient supplied budgets use REFUSED_BUDGET with the named field.

Root reviewed the formula translation and ran all four focused arithmetic cases: PASS, including exact/one-below limits, overflow and full-H deduplication. The implementer also reports locked host-web check, warning-denying library Clippy, formatting and diff checks passed. No FFI exports, staging allocations, protected boot or operation routing are enabled yet. Next is private boot wiring using the projection and validator.


## Frozen native mediation and bounded execution handoffs

# #825 bounded native continuous-spectrum mediation contract

Authority: canonical #825 preparation/ingress/receipt contract, #822 C4/C5 source, and root's reset-history ruling. Native-only checkpoint; no FFI/metadata/JS/SDK changes, new owner, ledger, allocator, scheduler, or worker. Public protected boot remains private until the following raw-command/response/meter alias guards land. Preserve legacy public signatures and behavior. Allowed implementation: host-web lib.rs, observation_ingress.rs, one narrowly factored native mediation module, existing native tests. No host-core change is necessary.

## Storage and exact ownership

ProtectedObservationStorage retains the unique controller, existing ingress/projection facts, and prepared SpectrumCadence. Move its existing target_track_id allocation into `spectrum_demand: HostSpectrumDemand { target: TrackPostMatrix { track_id }, channels: Stereo, mode: Continuous }`; response later borrows this exact target string. No per-start clone or second target allocation.
Add `history_epoch: Option<u64>` and `history_dropped_captures: u64` there. Initially None/0. These are metadata caches, not observation generations or a second history allocator.
Add one inline private `ObservationSideRecords` field to AudioWorkletEngineHost, OUTSIDE ReadyOwnership, containing:
`admission: WebObservationAdmission`, `capture_identity: WebObservationCaptureIdentity`, `receipts: [WebObservationReceipt; 4]`, `applications: [WebObservationReceipt; 4]`, `application_len: usize`, `pending_count: u8`, `completed_count: u8`, `reserved_mask: u8`, `pending_stop_slot: Option<usize>`, `terminal_finalized: bool`.
There are exactly four authoritative receipt slots; applications is the ordinary four-row outgoing staging, not another admission ledger. Row state0 means free internally and is never exported. The reservation mask distinguishes a reserved free row. Empty staging length0 invalidates old staging bytes; do not clear arrays on empty calls.
All these inline fields are charged by actual sizeof(AudioWorkletEngineHost), with the already-approved full-H owner overlap subtraction once. Do not add their sizes again as additive allocation payloads. Preserve admission94/cleanup132 and F=4 projections unchanged; no guessed ABI sizes or extra queues.

## Exact private seams

Use these names/types; structs/enums are crate-private, with private permit fields and no Clone/Copy implementation for the permit. C below means the existing controller, never a raw capture.
```rust
enum ObservationClass { Ordinary, Removal }
struct ObservationLengths { control_bytes: u64, rows: u64, result_bytes: u64 }
struct ObservationPermit { owner: ObservationOwnerId, epoch: u64, class: ObservationClass }
enum ProtectedSpectrumReadError { Refused(ObservationRefusal), Native(HostSpectrumReadError) }
// AudioWorkletEngineHost methods; existing public stream methods call these only for Protected.
fn begin_observation(&mut self, class: ObservationClass, lengths: ObservationLengths)
    -> Result<ObservationPermit, ObservationRefusal>;
fn start_spectrum_stream_admitted(&mut self, permit: ObservationPermit) -> Result<SpectrumCadence, u32>;
fn restart_spectrum_stream_admitted(&mut self, permit: ObservationPermit) -> u32;
fn stop_spectrum_stream_admitted(&mut self, permit: ObservationPermit) -> u32;
fn read_protected_spectrum_stream(&mut self)
    -> Result<ObservedContinuousSpectrumWindow, ProtectedSpectrumReadError>;
fn read_spectrum_stream_admitted(&mut self, permit: ObservationPermit)
    -> Result<ObservedContinuousSpectrumWindow, ProtectedSpectrumReadError>;
fn reserve_receipt(&mut self, class: ObservationClass) -> Result<usize, ObservationRefusal>;
fn release_receipt(&mut self, slot: usize);
fn commit_receipt(&mut self, slot: usize, accepted: ObservationAccepted, operation: u32);
fn reconcile_observation_applications(&mut self);
fn take_observation_applications(&mut self) -> &[WebObservationReceipt];
fn close_observation_receipts(&mut self, failed: bool);
fn record_observation_admission(&mut self, operation: u32, result: u32,
    refusal: Option<ObservationRefusal>, receipt: Option<WebObservationReceipt>, pending_boundary: bool);
fn spectrum_capture_identity(window: &ObservedContinuousSpectrumWindow)
    -> Result<WebObservationCaptureIdentity, u32>;
fn commit_observation_capture_identity(&mut self, identity: WebObservationCaptureIdentity);
// ObservationIngressState method, called only in render's successful branch.
fn on_successful_render(&mut self);
```
Expose immutable native getters `observation_admission(&self) -> &WebObservationAdmission` and `observation_capture_identity(&self) -> &WebObservationCaptureIdentity`; take remains a between-render-calls native seam. Future ABI wrappers reuse these helpers and the same permit, never reacquiring credit.

## Permit and operation behavior

begin consumes the selected attempt before readiness/length/target/native checks; a refused attempt stays spent. Check explicit inclusive control/row/result lengths after consumption. Already-used credit refuses Backpressure with exact ordinary_operations_per_boundary/removal_operations_per_boundary, requested2, maximum1. Add ingress exhausted:bool, initially false; successful render checked-increments epoch and clears both bits, overflow permanently sets exhausted and grants nothing (RevisionExhausted). Failed render grants nothing. Permits carry actual C.owner(), current epoch and class; admitted helpers verify these fixed scalars and never issue another permit. A mismatched owner refuses WrongOwner, other token mismatch InvalidRequest, before native work; this cannot refund its original attempt.
For native Rust start/restart reserve control_bytes=sizeof(WebSpectrumRequest)+prepared target bytes, rows1, result_bytes0; read uses control_bytes0, rows1, result_bytes=prepared packed_spectrum_bytes; stop uses control_bytes=sizeof(WebObservationDemand), rows0, result_bytes0. Later ABI wrappers provide their actual validated encoded lengths through the same begin seam before touching output staging.
Public start/restart use Ordinary and tags4/5. After permit/readiness, reserve receipt, then call C.replace_spectrum(&retained_demand) ONCE or C.restart_spectrum() ONCE. Native refusal releases only that reservation; it changes no cached configuration or committed capture identity. Accepted publication commits Pending receipt infallibly, caches history1/drops0, records RESULT_OK with receipt and pendingBoundary, and returns prepared cadence/current result. No stop/start pair, trial publication, receipt polling, or raw capture calls.
Public stop uses Removal/tag7. Before acquiring credit, a scalar pending_stop_slot check may return its existing Pending receipt unchanged, even with exhausted credit; no native call or scan. Otherwise acquire permit and reserve one row, then call C.stop_spectrum(). Pending commits the real acknowledgement; Quiescent releases the unused row and records RESULT_OK without receipt/pendingBoundary. After accepted stop, getters report inactive cadence/epoch via accepted_generation0; retained capture output/identity still keeps its old identity. StopGraph/tag3 later reuses this exact path because this profile has no other selected family.
The ordinary reservation requires two free rows (its own plus one left for removal); removal requires one. Cached counts refuse full capacity without scanning. At most one four-row search picks a free unreserved slot. Refuse Backpressure when no admissible row, with no graph call and without overwriting any receipt. commit sets Graph1/Pending1, owner=accepted.owner.get(), sequence=accepted.revision, application_sample0, result=RESULT_OK and valid record header; stop/tag3 caches its slot. No invented receipt for Quiescent.

## Read, compatibility, and identity

read_protected_spectrum_stream acquires one Ordinary permit/tag6 then calls read_spectrum_stream_admitted; the admitted helper alone calls C.try_read_continuous_spectrum exactly once. No receipt slot, graph mutation, receipt reconciliation, or second queue pop. Native READY gating remains; a retained STATE_FAILED host permits receipt taking, not new capture/admission. A native read outcome is an admitted availability outcome, not a preflight refusal.
Preserve the existing public `read_spectrum_stream() -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError>`. Its Protected branch calls the typed seam, commits identity only when returning a successful native window, then returns observed.window. Its Legacy branch stays unchanged. Future protected FFI must call the typed/admitted seam rather than this lossy compatibility facade.
Native outcome | admission result / pendingBoundary | old Rust error
Inactive | RESULT_WRONG_STATE / false | NotActive
PendingApplication | RESULT_BACKPRESSURE / true | Pending
Warming | RESULT_BACKPRESSURE / false | Warming
Pending | RESULT_BACKPRESSURE / false | Pending
Closed | RESULT_WRONG_STATE / false | NotActive
Failed { epoch, .. } | RESULT_RENDER_REJECTED / false | Failed { same epoch }
Gap { epoch, drops, .. } | RESULT_OK / false | Gap { same epoch, drops }
All these native outcomes set reason0, no receipt, and retain the full HostSpectrumReadError in the typed path. Refused admission carries its exact ObservationRefusal in the side record; the old Rust facade maps Closed/NotPrepared to NotActive and all other admission refusals to Pending. Never infer a generation from that compatibility error.
Refusal result mapping: NotPrepared→UNSUPPORTED; WrongOwner/InvalidRequest→INVALID_ARGUMENT; Capacity/WorkBudget/ArithmeticOverflow/RevisionExhausted→REFUSED_BUDGET; Backpressure→BACKPRESSURE; Conflict/Closed→WRONG_STATE. Preserve canonical refusal reasons1–10 and native limit/requested/maximum. All diagnostic writes initialize canonical headers/flags/reserved bytes; refusals write only admission diagnostics, not committed output bytes/length/token/identity or stream configuration.
On successful typed read, owner/generation/selection come directly from ObservedContinuousSpectrumWindow. On Failed/Gap, keep their native owner/generation in the typed error; the unchanged accepted/applied scalar projection supplies selection epoch if needed. These error paths publish no new successful capture identity. Never substitute accepted generation for applied, receipt revision for a window generation, or stream epoch for selection epoch.
spectrum_capture_identity constructs kind2/flags1 and those actual three identities; snapshot_token is checked window.sequence+1, EXACTLY matching old raw continuous packing. Never use host.spectrum_token (one-shot only). Check end_sample and token before output publication; impossible overflow returns REFUSED_BUDGET, consumes this admitted read, and preserves committed output/identity without wrapping. The old facade records ArithmeticOverflow and returns Pending on this post-read failure; the typed read itself still preserves the original native window. Do not mislabel such a post-read failure as a preflight refusal that never touched capture.
The identity commit is infallible scalar assignment. Direct Rust success commits it with the returned window; the following ABI tranche will commit it only with successful final raw packing after all fallible checks, using its already-held permit. Admission refusal never clears raw capture_len/result_len. This checkpoint does not edit today's unsafe-for-Protected FFI staging aliases; private boot prevents their public reachability.

## Cadence, history, and status truth

spectrum_target/channels return the one prepared TrackPostMatrix/Both profile, even dormant; spectrum_selection_epoch returns C.spectrum_state().selection_epoch. spectrum_stream_cadence returns prepared cadence only when accepted_generation!=0. spectrum_stream_epoch similarly returns the cache only while accepted_generation!=0. These are scalar getters and never consume credit or poll receipts.
On accepted start/restart cache1: #822 explicitly stages history epoch1 and resets it again at activation. This is a real deterministic native reset contract, not a synthetic graph generation. Return it immediately for old stream-start metadata; accepted receipt and PendingApplication still explicitly prohibit claiming Applied. Selection epoch comes only from C and does not increment on same-target restart/stop.
On Window replace cache epoch/drops with window values; on native Failed replace with its real epoch and drops0; on Gap replace with its real epoch/cumulative drops. Warming/Pending preserve these values. No additional readonly host-core history getter is needed. No new analyzer/history allocation or analysis-epoch mutation belongs in this native checkpoint. Later FFI resets its existing analysis history only after admitted successful start/restart or actual Failed/Gap, and preserves it on admission refusal.

## Reconciliation and final handoff

Never reconcile in admission or inside REALTIME_POLICY_BEGIN/END. take calls reconcile between render calls, including after sticky render failure, then copies completed rows in deterministic slot order to applications, releases only those source slots, sets application_len, and returns that slice. A repeated empty take sets length0 and does not scan/clear four rows. Pending rows are never released by taking.
Reconcile calls C.try_applied at most twice per take; stop immediately on None. If cached pending_count0, skip native polling entirely. Each Some matches owner/Graph/sequence in at most four rows, changes Pending→Applied with actual first_sample, adjusts cached counts, and clears matching pending_stop_slot. C performs its own at-most-two-slot retirement. Do not read graph state or imitate its retirement.
F=4 already reserves two carried receipts plus two current-credit receipts applied by a failed render before epoch advance; keep it. Missing receipts do fixed scalar queue/closure checks only, and empty takes repeat neither matching scans nor staging clears. No extra per-boundary cleanup credits, deep drain loops, receipt coalescing, or scans across historical generations.
Before explicit dispose destroys ReadyOwnership, reconcile once (at most two), then close_observation_receipts exactly once: preserve all Applied rows, mark only remaining Pending Closed/result WRONG_STATE or Failed/result RENDER_REJECTED according to sticky host failure, leave application_sample0, update counts, and clear pending_stop_slot. Status failure alone never calls this finalizer or invents closure/application.
Outer-host authoritative rows survive native dispose; take can expose them afterward. The later FFI disposal tranche must copy the final taken rows into its persistent application staging BEFORE dropping the live host, because current FFI dispose drops the whole host. No TLS allocation or FFI change now. Previous takes already transferred their rows; the next take may replace outgoing staging normally.

## Focused native checkpoint gates

Reuse private protected boot: dormant getters; start Pending before rendering; take actual receipt then obtain nonzero generation-bearing PCM; same-target restart keeps selection and resets history1; ordinary exhaustion cannot pop native capture or alter committed identity; stop retains removal credit and repeated-stop receipt identity; four-row pressure preserves every acknowledgement; start+stop yield both actual same-boundary receipts; failed audio preserves already-applied receipts; disposal closes only outstanding Pending and final take survives native disposal. Reuse native fault/gap fixtures where available without adding a harness or host-core test mutation API.
Exercise exact/one-below containing retained bytes after final inline fields, cached empty-take behavior, successful-render epoch reset and overflow. Run focused native tests/check/fmt/diff with CARGO_TARGET_DIR=/home/bl/misofm/engine/target; root commits the compiling checkpoint. No public protected boot, ABI/artifact claim, broad target matrix, or unrelated alias implementation in this task.

## Dependency-ordered Luna checkpoints (supersedes a combined implementation task)

Each task starts only after root commits its predecessor. Keep protected boot private throughout; permit only narrowly explained transitional dead-code annotations. Every task runs locked host-web native check, its focused tests, fmt/diff, and stops at a compiling checkpoint. No FFI, metadata, host-core, JS/SDK, artifact, GitHub, or delegated work.
1. **Side records and permit primitives.** Add the exact inline side-record storage/defaults/getters; permit/class/length types; begin, scalar on_successful_render integration, diagnostic mapping, pure spectrum_capture_identity and its scalar commit helper. Preserve the existing protected target/cadence storage. Do not implement receipt reservation/lifecycle or protected spectrum operation routing yet.
   Gates: real private boot still dormant; ordinary/removal credits independent; length refusal consumes its class; second attempt preserves seeded committed identity; successful render resets credit while failure does not; epoch overflow permanently refuses; pure identity uses actual owner/generation/selection and checked sequence+1; final shell retained accounting passes exact/one-below. No receipt polling anywhere in this task.
2. **Four-row receipt lifecycle.** Implement reserve/release/commit, cached pending-stop identity, reconcile, take, and one-shot close; integrate native dispose ordering. Do not route public spectrum operations yet. Existing module tests may submit actual C start/stop and commit their acknowledgements through private helpers to exercise the completed lifecycle without enabling a public alias.
   Gates: ordinary needs two free rows/removal one; rejected reservation preserves all rows; actual start+stop produce two distinct same-sample Applied receipts; pending rows survive take; completed rows release on take; repeated empty take performs no row scan/clear; failed audio preserves real Applied; dispose closes only remaining Pending and final rows remain takeable afterward. Verify no more than two C.try_applied calls per reconcile invocation and no receipt poll in reservation/commit.
3. **Protected spectrum controls and metadata.** Move the existing target allocation into retained HostSpectrumDemand; route only start/restart/stop through one permit, receipt reservation and the exact native methods. Add pending-stop fast return and fixed target/channel/selection/cadence/history getters. Start/restart cache epoch1 only after acceptance. Leave protected continuous read forwarding for task4.
   Gates: accepted start is Pending until task2 reconciliation; same-target restart preserves selection and resets history1; native publication refusal releases only its reservation and preserves previous cached history/identity; ordinary exhaustion leaves reserved stop usable; repeated Pending stop returns its original receipt without credit/native call; applied stop has zero capture dispatch. Legacy start/restart/stop tests remain green.
4. **Typed continuous read and compatibility facade.** Implement read_protected_spectrum_stream/read_spectrum_stream_admitted, native availability mapping/cache updates, existing public read compatibility mapping and successful direct-Rust identity publication. Use the already-frozen permit and receipt helpers; do not add another storage layer or enable public protected boot.
   Gates: pending generation cannot pop capture; actual nonzero PCM carries native owner/generation/selection and sequence+1 token; admission exhaustion preserves queued capture and committed identity; native Failed/Gap preserve supplied epoch/drop facts and publish no successful identity; Warming/Pending retain metadata; legacy continuous-read tests pass. Reuse existing fixtures; no new fault-injection framework.

Disposal call rule: dispose invokes reconcile at most once (therefore at most two C.try_applied calls) while ReadyOwnership still exists, then finalizes Pending rows once and removes ReadyOwnership. A subsequent take is handoff-only: if ReadyOwnership is absent or terminal_finalized is true, it must not call reconcile. While live, take calls reconcile at most once, only when cached pending_count is nonzero. Never compose dispose with an additional live take/reconcile pass merely to stage final rows. The later FFI copies final rows by taking AFTER native dispose, before dropping the whole host. Thus final reconciliation uses the existing F=4 receipt reservation and the one-shot terminal work in cleanup132; it adds no duplicate retirement/terminal scan.


## Private boot checkpoint evidence

Private protected boot now shares the legacy parse/compile transaction, prepares one dormant native owner, validates the exact target and ingress limits, and retains packed result sizes. Public protected boot remains unavailable until operation aliases are guarded. Root retained two focused tests: dormant render performs zero capture operations; accepted/applied generations start at zero; incompatible meter options, channel and target refuse; full controller overlap and one target allocation match actual bridge reports; retained limit is inclusive. Both pass with test-support. Locked native check, lib Clippy with warnings denied, formatting and diff checks pass. Implementer also reported the existing lib suite (105 passed, 2 ignored) and four ingress tests passing before root retained the focused fixtures. No Wasm/artifact or deployment claim at this checkpoint.
