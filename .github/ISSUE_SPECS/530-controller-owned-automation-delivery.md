# Controller-owned typed automation admission and cancellation

Status: numbered #530 on delivered main `301a57c6fb9982025b6fb0a49f3d35843c3e6f60` after #528 / PR #529 delivery. The protocol source is unchanged from Astra's read-only `af22dfa4` basis. Root adopts the limited typed/fixed-revision facade described below. Astra current-base numbered scope PASS is recorded below; root authorizes Luna attempt 1 after this checkpoint is pushed. Parent #140/#444, audit #349 IO5, ordered by #518. Workflow: Astra briefs/reviews, Luna attempt1, Sol attempts2/3; root owns Git/GitHub.

## Smallest closable outcome and deliberate restrictions

Deliver one opt-in Rust controller facade whose existing typed AutomationEnqueue requests pass through real controller validation and exact-byte replay into one #460 delivery owner. The returned real render half retains accepted reservations through handoff, partial application, completion and boundary-acknowledged cancellation. Existing transport-state events and cancellation events share the controller's reliable queue and sequence authority.

This is a controller admission/ownership product. It does not execute DSP, invoke #528, publish live ParameterStateGet, support framed/CAPI ingress, create graph bindings or complete IO5. The new facade has one fixed session revision and privately fixed provider/capabilities. SessionTransactionApply, any transport locate and ParameterStateGet report existing `Unavailable`; these are explicit limitations of this additive API. Existing default ProtocolController/CAPI behavior is preserved.

## Frozen public Rust API

Add unversioned `ControllerAutomationDelivery<P: ControlProvider>` with an off-render constructor of this exact substantive shape:

```rust
pub fn prepare(
    session: SessionStore,
    queues: ProtocolQueueConfig,
    provider: P,
    replay: ReplayCacheConfig,
    codec: ProtocolCodec,
    config: ProtocolControllerConfig,
    retained: ControllerRetainedCapacity,
    capabilities: PreparedDeliveryCapabilities,
) -> Result<
    (Self, AutomationDeliveryRender, ControllerAutomationResources),
    ControllerAutomationPrepareError,
>;

pub fn process(&mut self, request: ControllerRequest<'_>) -> ControllerResponse;
pub fn try_handoff_next(&mut self) -> Result<HandoffResult, DeliveryError>;
pub fn collect_terminal(&mut self, ticket: DeliveryTicket)
    -> Result<TerminalAutomation, DeliveryError>;
pub fn begin_cancel(&mut self, reason: AutomationCancellationReason)
    -> Result<CancelToken, DeliveryError>;
pub fn poll_cancel_boundary(&mut self, token: CancelToken)
    -> Result<Option<CancelComplete>, DeliveryError>;
pub fn dequeue_reliable_event_frame_into(&mut self, output: &mut [u8])
    -> Result<Option<usize>, EventEgressError>;
pub fn outstanding(&self) -> usize;
pub fn resident_automation(&self) -> u64;
pub fn automation_status(&self) -> QueueReport;
pub fn session(&self) -> &SessionStore;
```

Existing type/module spellings are authoritative. Preparation errors are a compact enum preserving `ProtocolQueueError`, `ReplayCacheError` and `ControllerResourceAllocationError`; no string/boxed error framework. Use the existing fresh fallible constructors. The event sequence starts at1, exactly as the ordinary controller; there is no second initial sequence input/cursor. The provided SessionStore/provider are trusted prepared inputs as with the existing controller. Create fresh queues and fresh replay from their configurations, never accept an already populated queue/cache/controller. Do not migrate live admissions.

`ControllerAutomationResources` reports `queue_and_delivery: DeliveryResourceReport`, `control_inline_bytes: usize` and `render_inline_bytes: usize`. The first field covers exactly one ProtocolQueues plus the delivered #460 ownership allocations. Inline fields use actual `size_of` and are reported separately, without double-counting the embedded ordinary controller. This report deliberately excludes preexisting session/provider storage, replay backing and the ordinary controller's diagnostic/telemetry/structural-owner allocations, which retain their existing preparation/accounting authorities; it is not a whole-process or whole-controller peak-memory claim. Explain those exclusions in rustdoc. No DSP/caller PCM is owned by this facade.

Do not expose `queues_mut`, `provider_mut`, mutable inner-controller access, `Deref`/`DerefMut`, raw retaining dequeue, arbitrary mark progress, prepared structural tokens, or an unchecked unwrap into the controller. Report/event methods are bounded control operations. The returned actual AutomationDeliveryRender retains its delivered public API and render obligations; its caller owns exclusive use and quiescence. Both halves are reclaimed off render only after real cancellation acknowledgment/quiescence. The facade cannot certify DSP application merely because its trusted render consumer marks a prefix.

## One concrete ownership extraction

Keep `ProtocolController`'s existing queues, replay and `next_reliable_event_sequence` as the sole authority. Its default layout and ordinary public constructors need no new optional-owned-service field. The facade privately contains that ordinary controller, fixed capabilities and one crate-private `AutomationDeliveryState`.

Extract only #460's existing concrete automation core/owners/order/generation/barrier/ack/cancel/staged fields and methods into this state. It borrows `&mut ProtocolQueues` for queue operations and the owner's sequence reference for begin/poll cancellation. It has no queue, replay or sequence cursor of its own. `AutomationDeliveryControl` remains the delivered standalone convenience owner of its queues/cursor and delegates to this identical state. Keep standalone APIs, ticket population, reservation semantics, FIFO ordering, terminal capacity, cancellation counts/order and resource formulas unchanged. No generic trait, dispatcher, event bus, scheduler or second accepted-work ledger is permitted.

Factor a crate-private controller typed-processing path accepting an optional borrowed concrete delivery context. Ordinary `ProtocolController::process` passes no context; the facade passes its state/capabilities. Reuse the current replay and execute/response functions rather than copying them. Only new-mode policy and AutomationEnqueue's admission destination differ. Existing framed and prepared structural entry points remain ordinary/default paths and are not exposed by the facade.

Prepare ProtocolQueues once, then add exactly the existing delivery ownership rings/entries/owners. A narrow crate-private helper may separate full-service preparation/report composition into base queues plus ownership state; do not call the old complete service constructor and allocate a duplicate queue set. No changes to the generic Copy-payload core's algorithms are needed.

## Processing, refusal and replay precedence

The typed API retains ControllerRequest's existing trust contract: command and canonical bytes describe the same already decoded request. It adds no new decoder. The *new facade path* must still validate public AutomationBatchSlot length/record shape before any as_slice or provider-domain lookup, then apply fixed revision and provider-domain validation and actual service admission. Preserve existing StatusCode mappings for malformed/invalid/past/full outcomes and the actual AutomationEnqueued count/queue occupancy/capacity/generation. That existing acknowledgment certifies owned admission, never application time. Do not create a second ack format or infer native support from descriptor automatable alone.

Replay classification precedes policy execution: exact cached requests return exact prior bytes and never execute again, including during cancellation; request-ID reuse/expiry/backpressure retains existing behavior. For a new executable request, retain existing feature/expected-revision checks, then apply the facade's mode restrictions before any provider/session mutation or structural compilation:

- SessionTransactionApply, ParameterStateGet and TransportSet with position=Some always return Unavailable in this facade.
- Non-locate TransportSet is allowed when no cancellation is pending; during cancellation it returns Unavailable before transport mutation or reliable reservation/publication.
- Other existing nonstructural typed commands use their ordinary provider/controller behavior. TelemetryConfigure may retain existing bounded configuration behavior; it does not create a sequenced lifecycle event. Do not expose separate manual event producer methods that bypass this policy.

New Unavailable results complete through the ordinary replay path. Retrying later requires a new request ID; reusing the same identity returns its cached refusal. An already cached success takes precedence over the new pending-state policy. Refusal changes no canonical model, fixed revision, transport or delivery ownership; ordinary response/replay bookkeeping is expected and must not be mislabeled a global no-op.

## Cancellation and a single event sequence

Begin cancellation uses the facade's fixed session revision and the controller's one next-event sequence for the existing checked headroom calculation. It reserves reliable capacity before publishing the actual #460 barrier. It must retain the delivered transactional ReliableFull/sequence-overflow behavior. While cancellation is pending, the facade blocks the only newly exposed sequenced producer (non-locate TransportSet); the sequence cursor cannot move between begin and poll. Structural sequenced events are unavailable in this mode. Cached responses emit no new event.

Poll waits for the actual render boundary acknowledgment, then publishes existing AutomationCanceled events directly to that same queue, updating the same controller sequence. It records `record_canceled_automation` on the provider exactly once when the real completion returns, preserving service applied/canceled counts. A repeated/stale poll must not duplicate counters or events. Dequeue/encode uses the controller's existing retained-event short-output retry path; do not extract/copy/renumber events through a second queue. Earlier transport events remain ahead, cancellation events follow in the existing stable order, and later transport events follow them.

Unknown capabilities retain current service meaning: a well-formed admitted batch unsupported by the fixed PreparedDeliveryCapabilities stays whole at PendingUnsupported and blocks later handoff. No supported subset is extracted. Admission overlap/density/total credit stays protected after handoff and after terminal publication until collection/cancellation reconciliation. The standing review question is: can an ack ever precede a drop? There is no public dequeue escape in this facade.

## Finite acceptance gates

Use one focused protocol fixture file/module with existing typed request/codec/replay helpers and small B=2 capacities. Freeze these six claim groups; do not expand them into a new corpus:

1. **Real controller admission/replay:** process one actual AutomationEnqueue, inspect decoded response fields and real returned render claim; replay identical canonical bytes gives identical complete response and no second admission. Include mismatched request-ID bytes and one malformed public length/valid-first-invalid-last rejection without panic or partial admission. Exercise provider domain/past checks using the existing eager provider fixture.
2. **Durable reservations:** hand off an admitted batch, reject overlapping/density-invalid work and B+1 as existing service rules require; render completion alone does not release credit, actual controller-facade collection does. Test duplicate terminal collection does not release twice. Use delivered service's deeper matrix unchanged rather than duplicating it.
3. **Unsupported FIFO:** a mixed supported/unsupported batch remains whole and pending, blocks a supported follower, then real cancellation reports all unapplied records and releases exactly the retained owners.
4. **One queue/cursor:** emit an ordinary non-locate TransportState, begin real partial-ticket cancellation, prove acknowledgment unavailable before a render boundary, verify new TransportSet is Unavailable with unchanged transport/sequence, and replay the earlier successful request byte-identically without another event. Complete cancellation, then use a new request to emit a later transport event. Decode the one egress stream and assert exact monotonic event sequences, effective sample, applied/canceled counts, provider canceled counter and short-output retry identity. Include one actual reliable-full begin refusal that leaves all owners and sequence unchanged.
5. **Fixed revision policy:** representative persistent edit, locate and ParameterStateGet return Unavailable without changing canonical snapshot/revision/transport/ownership. Read-only metadata/snapshot stays usable. Existing default controller's persistent edit/snapshot and transport tests prove its behavior is unchanged; add no mock “second session document.”
6. **One allocation authority:** report composition equals unchanged #460 queue-plus-ownership formula; fresh facade preparation is compared with one ordinary controller plus directly prepared ownership-state allocation sites, using existing isolated allocator tooling if totals are asserted. Demonstrate no second ProtocolQueues allocation. Keep control-side request/replay allocations allowed; the actual unchanged render boundary/progress/completion path must retain installed positive/zero realtime audit evidence through the reused #460 foundation. No new allocator, maximum counter or framework.

Run focused debug/release, full protocol tests (including unchanged #460 delivery/ownership and controller replay/lifecycle/resource regressions), affected strict Clippy/rustdoc/fmt/policies and supported scalar/SIMD Wasm compilation. A compile-time visibility/ownership check may prove the facade cannot expose mutable queues; no bespoke compile-test harness. No PCM fixture, benchmark, browser development matrix or full host lifecycle gate belongs here. Proportional immutable delivery, exact PR/current-base review, required qualification, checked premerge base and remote evidence/closure remain mandatory after source PASS.

## Exact paths, half-day assessment and stop

Allowed production paths are `crates/protocol/src/delivery.rs`, `crates/protocol/src/controller.rs`, optional small `crates/protocol/src/controller_delivery.rs`, and `crates/protocol/src/lib.rs`; one focused protocol test file/module plus existing controller/delivery tests as needed. Queue.rs is not preauthorized for semantic edits; its existing crate-private retaining operations suffice. No CAPI, host-core, provider implementation, graph, effect, browser, resource fixture or dependency change is planned. Root owns numbered spec/evidence.

The slice is feasible within half a working day **only in this fixed-revision, typed-only shape**: one mostly mechanical concrete-state extraction, one private processing context and thin facade, and six finite reused-fixture claims. The extraction and facade are one useful ownership outcome and should be implemented as one coherent pass; do not publish an intermediate new abstraction as a separate “capability.” Pause at compiling/focused-green completion for root's exact-path checkpoint, then finish proportional gates and one consolidated review. Preserve source-attributed failures. Luna1/Sol2/Sol3 and the hard stop apply.

If implementation needs public framed/prepared-structural processing, mutable CAPI migration, automatic revision/locate barriers, provider live readback, a second ledger, generic event reservation framework or changed delivery arithmetic, stop before editing those paths. Those are independently useful successors, already excluded to keep this leaf closable; do not grow them mid-attempt. Numbering/current-base approval must verify the mechanical extraction still fits these seams rather than relying on this estimate alone.

After this child: qualified controller-owned render-half adoption by the accepted scalar endpoint, live provider overlay, scalar graph/native-host binding and actual framed command-to-PCM/readback with matched lifecycle publication; then bank/other-effect/parameter/segment/browser rollout. #444 reuses ownership but retains its builtin BlockTarget/cutoff/late rules. IO18's browser console service remains separate. Neither this facade nor #528 closes #140/#444/IO5; protocol carries typed control records and never PCM.

## Numbered current-base scope approval

# Astra numbered/current-base scope review — #530

PASS for one implementation attempt under the frozen numbered spec. This is scope approval, not implementation or delivery acceptance.

Reviewed clean pushed checkpoint `cc37f367680e959cca4669e369c5ac800f53e0df` against delivered main `301a57c6fb9982025b6fb0a49f3d35843c3e6f60`. GitHub #530 is OPEN with matching title “Controller-owned typed automation admission and cancellation.” Independently observed remote main at that identity, an empty protocol-tree diff from `af22dfa4` to this base, and only the numbered spec added above base. No source/test/Git mutation or gate execution was performed.

The concrete extraction is feasible within existing protocol boundaries. `AutomationDeliveryControl` currently owns queues, concrete core/owners/order/generation/barrier/ack/cancel/staged state and its cursor. Moving only the concrete state behind borrowed queue/cursor operations lets the existing standalone owner delegate unchanged and the opt-in facade use its private ordinary controller as the sole queue/replay/sequence owner. The existing retaining dequeue/release operations already have crate visibility; no queue semantic change, dependency, generic framework or new delivery algorithm is necessary. Default controller public layout/behavior need not acquire an optional service.

Constructor spellings are consistent with current source: `ProtocolQueues::prepare`, `ReplayCache::try_new`, and `ProtocolController::try_with_config_and_retained_capacity`; their errors are respectively `ProtocolQueueError`, `ReplayCacheError`, and `ControllerResourceAllocationError`. Existing delivery ticket, cancellation, terminal, report and event-egress types support the specified facade API. The extracted ownership preparation helper must use the existing allocations without preparing a second queue. Existing allocation failure behavior is inherited; the spec does not commission a new allocator or promise universal recoverable OOM.

The adopted fixed-revision, typed-only restrictions make the event-cursor borrowing sound: no structural/locate mutation, no live ParameterStateGet claim, no mutable inner/provider/queue escape, and newly executed non-locate TransportSet refused while cancellation is pending. Existing replay classification stays ahead of policy; previously cached success emits no new event, newly cached Unavailable remains cached. Existing feature and expected-revision validation retains precedence. New-mode batch shape validation precedes unsafe length-dependent slicing/domain iteration. Cancellation publishes directly into the original queue, and real completion increments the provider canceled counter once. Preparation starts fresh; no accepted work is migrated or silently discarded.

All six numbered claim groups remain the minimum focused acceptance scope. Reuse unchanged #460 ownership/render and ordinary controller regressions. Gate6 proves one queue allocation authority and explicitly limited report composition; it must not mislabel queue-plus-delivery payload as total controller memory or invent a maximum-request measurement. Existing realtime positive/zero evidence must still cover the extracted render path. Focused debug/release, full protocol regressions, affected strict Clippy/rustdoc/fmt/policies and scalar/SIMD compilation are proportional. No extra browser development matrix, DSP corpus, benchmark or generic harness is needed.

The half-day estimate remains credible for this concrete extraction plus restricted facade and six reused-fixture groups. It is not authorization to expand into framed ingress, CAPI migration, automatic lifecycle publication, live provider overlay or PCM binding. If those become necessary, stop and split before editing their paths. #140/#444/IO5 remain open obligations. Root may checkpoint this approval and start Luna attempt1; the existing one coherent pass/one consolidated verdict and three-attempt hard stop remain in force.

## Concrete Gate6 clarification (same gate, no public escape)

Use the existing TLS `CountingAllocator` and `measured` closure in `crates/protocol/tests/delivery_ownership.rs`; it already measures count/free/requested bytes/largest, so do not add bench-support, a dev dependency or a second allocator. Gate6 can live in this existing integration fixture alongside the focused functional file. No private-state public accessor is needed: separately measure fresh (A) facade preparation, (B) ordinary controller preparation including fresh queues/replay, (C) standalone PreparedAutomationDelivery preparation and (D) ProtocolQueues preparation, all with the same queue configuration. Build equivalent owned session/provider inputs outside measured closures, retain every result until measurement is inactive, and include equivalent replay/retained configuration allocation in A and B. Assert A allocation count and requested bytes equal B + C - D, with zero preparation frees for these fresh paths. This directly compares the existing public preparation authorities and detects a second queue set. It is the observable equivalent of adding only the extracted ownership allocations.

Largest allocation is not subtractable. Keep the existing exact standalone C bytes/largest versus resource-report assertion and verify the facade queue_and_delivery report equals that unchanged composition; inline sizes remain separate. Do not call A's whole-controller largest the queue-and-delivery maximum. Existing TLS isolation needs no subprocess or new framework. This clarification resolves implementation mechanics within the numbered Gate6 rather than adding a gate or changing its resource claim.
