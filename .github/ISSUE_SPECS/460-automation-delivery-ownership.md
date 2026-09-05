# Retain accepted automation reservations through bounded handoff and cancellation

Recommended title: **Retain accepted automation reservations through bounded handoff and cancellation**. Parent #140; common prerequisite for the future #444 native builtin admission integration. Queued draft for root adoption/numbering, not implementation authority.

Inspected `/home/bl/misofm/engine-140-plan` HEAD `14d35f4b7fa794098b37a0160ab4d0cee0f2b8fb`. Its protocol and host control-provider source is identical to delivered #442 main `452a327881bfd883c6c569b6606009a40b981e22`. Read adopted D1–D5 and the canonical-model ruling. No mutable #430 implementation, builds, tests, timing or repository/remote mutation.

## Smallest product and deliberate integration boundary

Deliver an opt-in prepared Rust ownership service that actually moves admitted fixed automation batches to a separately owned consumer, retains admission reservations until terminal return, and supports a boundary-acknowledged cancellation barrier. It must use real bounded SPSC endpoints and real protocol admission/reservation logic in tests. This is an independently usable ownership capability; it does NOT apply PCM, resolve a parameter handle into DSP state, replace the production controller, or close IO-5/#140/#444.

Keep current ProtocolQueues::prepare, ProtocolController construction, AutomationEnqueue wire/response, queued-only cancellation and all host delivery behavior unchanged. New delivery is available only through a distinct prepared owner type that takes exclusive ownership of ProtocolQueues; it cannot also be installed in the existing ProtocolController. This makes 'production host delivery disabled' structural, not a flag a host can accidentally enable without lifecycle integration. A later integration child must explicitly adapt the controller to this owner before it can submit existing protocol frames to a renderer.

Do not add generic handoff methods directly to public ProtocolQueues alongside unrestricted queues_mut/try_dequeue_automation: that would leave the old reservation-releasing dequeue and synchronous controller cancellation able to invalidate handed-off work. The new owner must not expose mutable raw automation access. The service may forward unrelated response/telemetry operations only when actually needed; no wholesale controller façade in this child.

## Current source seam and exact proposed API division

`protocol/queue.rs:807–830` admits one AutomationBatchSlot atomically and then records density/interval/frontier. `try_dequeue_automation:835` removes those reservations immediately. `report:1091` reports the queue's resident occupancy, not outstanding delivery. `validate_automation_admission:1101` enforces global time, past-at-admission, overlap and per-block density. Reuse those same routines and grammar; no second automation validator or reservation table.

Add one module under crates/protocol/src (for example delivery.rs), export its prepared service from lib.rs, and make only the necessary queue.rs internal helpers crate-visible. Names below are the intended API shape, not permission to expand it:

- `PreparedAutomationDelivery::prepare(config: ProtocolQueueConfig) -> Result<(AutomationDeliveryControl, AutomationDeliveryRender), PrepareError>` allocates all fixed storage before splitting owners. Control exclusively owns the ProtocolQueues and its density/interval/frontier state. Render has fixed pending payload/state plus its SPSC ends, never a ProtocolQueues/controller reference.
- `AutomationDeliveryControl::try_admit(current_sample, batch) -> Result<(), AdmissionError>` uses existing validation and fixed queue admission, but gates total outstanding count at the configured B, not just resident queue length. Rejection retains the original caller batch and changes no reservations. Queue generation/revision/request identities remain distinct.
- `try_handoff_next(capability: &PreparedDeliveryCapabilities) -> Result<HandoffResult, DeliveryError>` examines the oldest queued batch without releasing any reservation. The capability table is immutable, prepared, and supports exact (handle,kind) membership only; there is no arbitrary callback or target-resolution algorithm here. If every record is supported, publish that complete batch with its ticket. Otherwise return PendingUnsupported and leave the head/batch owned unchanged. FIFO head blocking is the explicitly bounded initial policy; no hidden skip/drop. Report the blocked result so a future host cannot falsely advertise complete delivery.
- `AutomationDeliveryRender::begin_boundary(first_sample)` processes only a configured finite number of transport commands and applies any cancellation barrier before returning the pending view. `pending(ticket)` exposes immutable raw records and applied-prefix count; it does not mark them applied. `mark_applied(ticket, new_prefix)` accepts only a monotonic prefix within length and matching identity. `finish_applied(ticket, observed_sample)` requires prefix==length and sends exactly one terminal return. There is no general arbitrary payload mutator or automatic late-event application in this service.
- `AutomationDeliveryControl::collect_terminal()` consumes a terminal exactly once, verifies ticket generation/serial, and releases the ORIGINAL complete batch's density/interval reservations and outstanding capacity. Retain full reservations even after a prefix applies until whole-ticket terminal disposition; this is the explicitly allowed conservative accounting. No credit is reused merely because the ring item moved or render finished before control consumed its return.
- `begin_cancel(reason)` reserves control-side reliable-event capacity for every outstanding batch and one prepared barrier request/ack credit before committing a cancel state. Failure leaves admission, ownership and lifecycle unchanged. Success blocks new admission/handoff for that generation. `poll_cancel_boundary()` completes only after the render acknowledges the barrier; it returns terminal records identifying applied prefix and remaining cancellation count. The service exposes a typed pending result, never a successful synchronous cancellation before acknowledgement.

The exact Rust naming/error enum spelling may be conventional, but ownership and fallibility above are frozen. No allocation in admission/handoff/render/terminal polling after preparation; errors contain fixed data or original fixed batch, not heap strings. The legacy API remains available only on independently prepared legacy ProtocolQueues.

Internal change: split old dequeue into a crate-private 'remove resident batch WITHOUT releasing admission' operation plus the current explicit reservation-release helper. Legacy try_dequeue_automation keeps its current behavior by invoking both. The new owner retains each original batch in a control-side B-slot ledger through terminal, so it can release the same reservation rows without receiving mutable reconstructed records from render. An internal peek/staged-head slot for capability evaluation remains counted in B and is returned/canceled like queued work; it must not accidentally increase admission capacity.

## Ticket state and terminal ownership

Ticket identity is `(delivery_generation, slot_index, serial)`, with checked nonwrapping generation/serial allocation. Reject stale or duplicate operations without touching credits. Reuse a slot only after control consumes its terminal. Do not use RequestId alone as reusable slot identity, and do not repurpose the fixed queue generation as a per-render epoch.

Logical states: Queued -> HandedOff -> Pending(prefix0..len) -> TerminalQueued -> Reclaimed. Cancellation overlays CancelRequested until render's boundary barrier returns either Applied(len) or Canceled(applied_prefix, remaining). The control ledger owns reservations throughout; render owns application progress after handoff. Copies of fixed payload across the SPSC are transport/storage copies, not a second logical ticket or second consumer.

For queued-only batches cancellation can immediately classify all records canceled, but the aggregate barrier must still account every handed-off ticket before reporting lifecycle completion. Already completed tickets racing cancellation remain Applied; never relabel them canceled. A partially applied ticket returns exactly len-prefix canceled records. A canceled batch with zero remainder produces no false cancellation count; any unused reliable reservation is released on control. Exactly one terminal disposition retires a ticket.

After control has requested cancellation, no new old-generation handoff is published. The barrier must be ordered behind previously committed handoffs, or otherwise carry an exact committed sequence that the render verifies before acknowledging. A FIFO command channel with bounded Deliver messages and one reserved Cancel barrier is an acceptable concrete route; cancellation publication cannot be blocked by ordinary handoff saturation because its credit is reserved. Render must process the barrier before exposing old-generation pending records for that boundary. An ack certifies no further old-generation application through this endpoint.

This transport boundary is NOT #444's application-sample admission cutoff. The service does not promise which newly handed-off future Point applies in the current block; that scheduling belongs to #140 B/C and #444's separate typed block-target integration. Its finite command-processing bound retains unprocessed commands for the next boundary and cannot drop accepted work. No producer-length snapshot is advertised as an application guarantee.

## Minimal typed payload accommodation for #444 — freeze now, no builtin dependency

Factor ONLY the ticket/transport/progress/terminal core over `P: Copy + Send + 'static`, with a caller-supplied validated logical record count in1..=256. Use `PreparedDelivery<P>`/control/render endpoints internally or as a narrowly documented Rust service. It owns credits/slots/terminal lifecycle, not parameter validation, timestamps or overlap semantics. The automation adapter instantiates P=AutomationBatchSlot and owns the existing protocol reservations. Test one tiny independent Copy payload instantiation to show the core has no AutomationRecord-layout assumption.

Later #444 can instantiate the SAME core with a fixed typed builtin block-target payload defined by its compiler/host adapter, retaining actual fader/mute/matrix fields. Do not add a builtin variant to AutomationRecord, change its32-byte wire identity, convert matrix updates into fake Point handles, or add a builtins dependency to protocol. This is code-level reuse of the SAME ownership state machine, not permission to maintain two independent ledgers for the SAME accepted ticket. Separate endpoints may of course have separately configured capacity; shared-resource cross-endpoint arbitration is outside this child and must be explicit if a future host allows both to target the same state.

Freeze a single source of ticket truth: the common core owns ticket/credit lifecycle, the automation adapter attaches original batch/reservation metadata indexed by that ticket. It must not independently retire or reuse the slot. Payload-specific density/interval admission remains in the existing protocol adapter; a future builtin adapter must define its own already-numbered command admission semantics rather than claiming those Point/segment reservations apply automatically.

## Resources and reliable cancellation

B=config.automation_batch_slots. Fixed bound is B outstanding tickets, each with <=256 records; one terminal return credit per ticket and one cancellation-barrier request/ack credit. Use checked arithmetic and actual Layout/size_of plus existing bounded_spsc_retained_payload for every allocated queue header/backing and ledger/pending array. Transport may transiently duplicate a fixed payload in original ledger, queue slot and render pending storage: CHARGE EACH allocation; B*256 is the logical record bound, not an excuse to report only one physical copy. Account the extra capacity+1 SPSC slot, largest single allocation and preparation failure before exposing endpoints.

Keep legacy ProtocolQueueResourceReport unchanged for legacy prepare. The new preparation report adds the exact new service allocations to that existing report. CAPI/host default resources must not grow merely because this optional service exists. No CAPI resource mirror edit is needed until a host enables it; independent new protocol resource tests cover this service now.

ReliableEventReservation(s) contain Arc-backed control ownership and Drop behavior; never send them to render. Hold reservations on control throughout cancellation. Render returns fixed terminal/cancel records only. Control commits existing ReliableSlot::automation_canceled records with revision/request/generation and remaining count using the retained credit. Sequence-number allocation must be prevalidated before cancel commit; no half-published cancellation on overflow. Expose typed terminal data to the later controller adapter rather than inventing a wire response or a competing protocol sequence source here.

## Finite acceptance

Use actual prepared endpoints and existing protocol fixtures, no PCM mock represented as integration:

1. B/B+1 admission and1/256/257 record boundaries; reject invalid/overlap/density/past/global-order with original batch and unchanged state. Handoff changes resident occupancy but not outstanding report/reservations. Unsupported/mixed batch stays whole pending and blocks later handoff by the declared FIFO policy; cancel remains available.
2. Transfer an actual valid batch across the SPSC to a distinct owner, read exact payload/revision/request, advance partial prefix, finish and consume terminal. Capacity remains unavailable before control consumption. Stale serial/generation, duplicate completion, overlong/regressing prefix and premature finish cannot free or apply twice. Future scheduling/application is not claimed.
3. Cancel before handoff, after handoff before application, after a prefix and racing final completion. Real deterministic thread/barrier coordination on publication/barrier/return points; no sleep/timing test. Exactly counted remaining cancellation, reliable-full refusal, blocked publication/generation advance until acknowledgement, no old-generation pending access after ack. Partial terminals release original reservation rows once, and the next same-handle legal batch is accepted only after release.
4. Actual production reservation-release-on-handoff counter-mutation must fail the SAME overlap/capacity assertion. Actual premature-cancel-complete/credit-release mutation must fail the SAME boundary/ownership assertion. Do not accept generic setup/panic failure as these controls. Original state-machine tests pass on restored source.
5. Exact resource report/overflow/cap checks and positive allocation/free liveness plus repeated endpoint boundary/mark/terminal operations with zero allocation/free. Dropping endpoints and retained heaps occurs only off render. Retain existing protocol/controller enqueue/replay/cancel/session canonical snapshot tests unchanged; no persistent-model or wire change is credited.

## Decisions and delivery boundary

Root should explicitly adopt the opt-in standalone owner (no raw mutable automation escape), conservative outstanding occupancy/full-ticket reservations, FIFO unsupported head policy, generic Copy payload core, and asynchronous cancel barrier above when numbering this child. These choices resolve the former open API/payload alternatives; Luna must not choose a parallel ledger or attach the service to production hosts during implementation.

One precise integration decision remains OUTSIDE this child: the controller's reliable sequence/lifecycle publication adapter for handed-off work. This child returns validated terminal data and barrier completion; it does not claim current SessionTransactionApply/locate synchronously cancel handed-off work. Keeping the new owner structurally separate prevents that unsupported use. #140 C must freeze the adapter before enabling host delivery. Similarly #444 still owns block-target endpoint semantics and application cutoff, for BOTH bank and scalar native paths.

Allowed paths: protocol queue.rs, a narrow delivery module/lib exports and existing focused protocol tests (plus a focused integration test if separate thread ownership needs it), numbered evidence. No SPSC implementation change, new dependency, native endpoint, graph/builtins/host/CAPI production change, parser/framework, benchmark or artifact work. If this cannot remain a half-day ownership slice, narrow the public façade rather than weakening terminal gates.

After numbering, actual-base Astra approval -> Luna1 -> one consolidated Astra verdict -> Sol2/3 only after FAIL -> hard stop. Source PASS precedes immutable workspace/required target qualification and actual-head PR/CI delivery. D1 accepted unsupported batches, D4 exact Point scheduling, D5 canonical/transient distinction and the full #140/#444 outcomes remain open. This document is a draft for root review; no implementation authorization or parent closure.

# #140 A amendment — one cancellation publication owner

This replaces the draft's ambiguous split between returning terminal data and publishing cancellation events. Read current `ProtocolQueues::commit_reserved_reliable_event` and queued controller cancellation in the planning tree. No source changes, tests, builds, timing or Git/GitHub mutations.

## Exact recommended route for root adoption

The OPTIONAL standalone `AutomationDeliveryControl` owns BOTH its ProtocolQueues and that queue set's sole reliable-event sequence cursor. Preparation takes an owned sequence authority alongside the queue configuration; a newly created standalone endpoint starts at its declared initial sequence (use1, matching the existing controller convention). The cursor is private thereafter. No separate controller may own or publish events into this same queue set. This is not a second sequence source for an existing endpoint: the existing production controller and its queues remain unchanged and cannot attach this opt-in service in this child.

Represent this as one constructor input `initial_reliable_event_sequence: u64`, validated before allocation/publication, and one private `next_reliable_event_sequence` field. Do NOT accept caller-supplied ranges on each cancellation, arbitrary event callbacks or competing sequence generators. Future controller integration must MOVE the existing sequence authority into the integrated owner and route all reliable-event publication through that single authority; it may not retain a second controller cursor. That future adaptation remains outside this child, explicitly.

This is the finite additional root decision requested: **approve sole sequence ownership in the standalone opt-in service**, with future controller integration required to transfer/unify that ownership. It gives this child a complete useful publication contract without implementing a production controller adapter now.

## API and phases

`begin_cancel(reason, event_revision) -> Result<CancelToken, CancelBeginError>`:

1. Verify no cancellation already pending and freeze the current outstanding ticket population (including terminal returns not yet consumed). Let M be its upper bound on nonempty cancellation events. Capture each ticket's original request/batch/queue-generation identity. The explicit event_revision is the revision to report, mirroring the current controller's post-commit revision argument; it does not change batch ownership revision or commit a session.
2. Check `next_reliable_event_sequence.checked_add(M)` before committing anything. This reserves arithmetic HEADROOM, not emitted event numbers. Reserve M reliable-event slots through the existing `reserve_reliable_events`, plus the already-prepared cancel request/ack credit. Any failure leaves admission/tickets/cursor/lifecycle unchanged.
3. Store the actual ReliableEventReservations object in the control owner's private PendingCancellation. Never move it to render. Commit the barrier, disable old-generation admission/handoff, and return Pending token. The cursor has not advanced. During pending cancellation the standalone service exposes no other reliable-event publication path; existing events may still be dequeued.

`poll_cancel_boundary(token) -> Result<CancelProgress, CancelPollError>`:

- Before render's matched boundary ack, return Pending and retain all event reservations and ticket ownership. Do not emit a completed-cancellation response.
- Accumulate fixed terminal outcomes for the frozen ticket set. Already applied tickets stay Applied; partially applied tickets carry their prefix and exact remaining count. The acknowledged first_sample supplies the actual effective cancellation sample. No caller-supplied guessed effective sample may be reported as render acknowledgement.
- Once the barrier and every relevant disposition are reconciled, publish cancellation events in deterministic original admission order for tickets with remaining_count>0. For each, call EXISTING `ProtocolQueues::commit_reserved_reliable_event` with EXISTING `ReliableSlot::automation_canceled(event_revision, next_sequence, request_id, remaining_count, reason, original_queue_generation, Some(boundary_sample))`, then increment the sole cursor once. Prevalidated M headroom makes these increments infallible; K actual events consume exactly K consecutive sequence numbers, not M numbers with unexplained holes.
- Applied tickets produce no cancellation event. Release the unused M-K event reservations on CONTROL after dispositions are known. Release each ticket's original admission reservations exactly once in the same terminal reconciliation; no slot becomes reusable before its disposition is consumed. Clear the pending cancellation and reset the old ordering epoch only after this sequence is complete.
- Return `Complete { boundary_sample, canceled_records, published_events: K, ... }` ONLY AFTER all K reliable slots have been committed. Publication means enqueued in the reliable queue, not delivered to a network peer or consumed by the caller. Existing reliable backpressure discipline protects them thereafter. No 'complete but events still need a later controller callback' state is permitted.

The service exposes a narrow `try_dequeue_reliable_event()` to let its control-side caller consume already-published existing ReliableSlots. It must not expose arbitrary mutable ProtocolQueues/event producer access or permit the caller to reset the private sequence while tickets exist. No new wire event/schema is introduced.

`collect_terminal` during cancellation must route frozen tickets into PendingCancellation reconciliation instead of reclaiming them on a path that bypasses cancellation publication. Normal Applied completion outside cancellation still releases its ticket with no invented AutomationApplied wire event. A dropped CancelToken is only a dropped COPY identifier; the cancellation/reservations remain owned by the control service and can be polled again. No token Drop may silently release event credits or cancel accepted work.

## Failure, shutdown and scope limits

A reliable queue full BEFORE begin_cancel returns typed refusal and leaves all accepted work untouched. After successful reservation, committing its events must use the existing guaranteed-capacity method, not fallible unreserved try_enqueue_event. Unexpected identity/protocol faults leave the pending cancellation owned and return an explicit error; they do not discard reserved credits or report success. Render never touches Arc-backed ReliableEventReservations or publishes protocol messages.

Normal shutdown must complete/reconcile pending cancellation and drain or hand off its reliable queue on control before destroying the endpoint. This child must not add a method that reports successful graceful shutdown while pending events are dropped. Forced owner destruction remains off-render resource destruction with no claim of successful cancellation notification; it is not an alternative terminal success path. Production provider-loss/host-shutdown integration still belongs to #140 C.

This standalone cursor does not commit SessionModel, update persistent revision, or make a transport locate effective. Its event_revision is explicit context, and Complete certifies only this delivery owner's barrier and reliable publication. Future host/controller integration must coordinate structural publication with that result. D1/D4/D5 and all existing canonical/wire semantics remain unchanged.

## Finite amendment gates

Add to the original child gates: reliable-full begin refusal leaves cursor/reservations/tickets unchanged; sequence headroom overflow refuses before barrier; partial/application race publishes exactly K existing cancellation events with consecutive sequence values and correct remaining counts; applied-only cancellation publishes0 and does not advance cursor; unused reserved capacity is returned; repeated polling/token reuse cannot duplicate events; and Complete is impossible before both boundary ack and actual reliable publication. A counter-mutation that returns Complete before committing the reserved events must fail the SAME event-presence/sequence assertion.

This is a small control-owner field and publication contract, not a new callback framework, generic sequencer or production controller rewrite. Once root adopts this exact ownership choice, the child no longer has an unresolved cancellation API: control owns reservations, control prevalidates/owns sequence headroom, poll_cancel_boundary commits existing reliable slots, and only that method reports completion after publication.

## Root adoption and frozen boundary

Root approves the opt-in standalone owner with exclusive ProtocolQueues ownership and no mutable raw automation escape, B outstanding tickets with full-ticket reservations until terminal consumption, FIFO blocking for unsupported whole batches, the narrow Copy-payload ownership core, and the asynchronous cancellation barrier. Root also approves the amendment's sole reliable sequence authority: preparation takes the initial sequence (1 for a fresh endpoint), control retains all event reservations, and poll_cancel_boundary publishes the reserved existing cancellation events before Complete. The amendment supersedes conflicting preliminary draft statements; there is no separate controller cursor or deferred publication callback.

Planning source is delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`, whose protocol/control-provider source matches the inspected draft base. Root must number/synchronize this child and obtain actual-base Astra scope approval before implementation. This is queued behind the current live-pair delivery work.

Admission here certifies durable acceptance, never PCM application. Unsupported/mixed batches remain whole, owned and cancelable under the declared FIFO policy. Point/segment scheduling and actual DSP application are not implemented by this service. Existing transient AutomationEnqueue leaves canonical session base/revision unchanged; persistent SessionTransactionApply and stored automation mutations still update the same typed session transactionally and snapshot canonically. No shadow session or render JSON is introduced.

#140 retains controller sequence/lifecycle integration, actual Point application and host rollout, including cancellation during structural revision/locate. #444 retains its typed builtin endpoint, application cutoff/late semantics and both native bank/scalar pairing. Neither parent nor audit #349 IO-5/RT-4 can close from this child alone. No production host enablement, ABI/wire change or benchmark authority is granted.

## Astra numbered scope approval — queued

# Astra #460 numbered scope/base review — PASS, queued

Exact clean planning checkpoint `72d46bfd73d7d789e890ef425272f585f6886996`, `/home/bl/misofm/engine-140-ownership`, based on delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`.

PASS for the numbered ownership-service scope. No additional design amendment is required by this review. This is queued readiness, not implementation authorization or a delivery claim; root must wait for #459 runtime delivery and compare the actual resulting base before assignment.

Read the full #460 spec, adopted draft and cancellation amendment, and reciprocal #140/#444 dispositions. Both report bodies are retained intact. Root's final adoption expressly gives the amendment precedence over preliminary draft signatures/terminal-data-only wording. Thus the effective preparation API includes the initial reliable sequence, begin_cancel includes event_revision, and poll_cancel_boundary has the publication contract below. Historical 'before numbering' text is superseded by the actual numbered spec and reciprocal #460 dispositions; it does not leave the source/API decision open. Root reports matching remote number/title/body verified OPEN; no independent remote-state verification is claimed in this review.

The source premise is unchanged: no crates/hosts/scripts/Cargo delta from the declared planning main, and protocol/control-provider source matches the previously inspected delivered #442 source. Existing admission validation, atomic fixed-batch push, density/interval rows, reservation-releasing legacy dequeue and reserved reliable-event commit remain the concrete integration seams. There is no hidden implementation in this planning checkpoint.

The smallest product remains an actual separately owned SPSC handoff with retained reservations and acknowledged cancellation. A new exclusive opt-in control owner contains ProtocolQueues and cannot also be installed in the current controller; mutable raw automation access is forbidden. Legacy prepare/controller/wire/host paths remain unchanged. The crate-private dequeue split preserves legacy behavior while allowing the new owner to retain original reservation identity until terminal consumption. B outstanding tickets, not resident queue length, controls capacity; full-ticket reservations conservatively persist after partial progress. Unsupported/mixed batches remain whole, FIFO-blocking, owned and cancelable.

The cancellation API is now unambiguous:

- Control owns the sole sequence cursor and actual ReliableEventReservations. It validates M-event sequence headroom and reserves M event credits plus the barrier credit before committing cancellation. Failed begin leaves all state intact; no caller callback or second cursor is permitted.
- Render sees fixed transport/cancellation records, not Arc-backed reservations or protocol/event producers. A matched boundary acknowledgement and reconciled ticket dispositions distinguish already applied records from the remaining canceled population.
- Control's poll_cancel_boundary commits K existing ReliableSlot::automation_canceled records through the reserved API in admission order, advances the sole cursor by K, releases unused M-K reservations and reclaims each original admission reservation exactly once.
- Complete is returned only after boundary acknowledgement AND actual reliable-slot publication. It does not mean the caller consumed or transmitted those events. collect_terminal cannot bypass pending cancellation; a copied token's drop cannot release pending credits. Sequence overflow, reliable saturation, duplicate/stale tickets and premature completion retain the frozen discriminating gates.

The narrow generic `P: Copy + Send + 'static` core owns ticket/credit lifecycle. The automation adapter attaches existing admission metadata indexed by that same ticket; it cannot independently reuse or retire the slot. One tiny alternate payload instantiation proves layout independence. This permits future typed builtin block targets without changing AutomationRecord or its wire encoding, introducing a builtins dependency, or assigning two ledgers to one accepted batch. Separate future endpoint arbitration is explicitly deferred, not implicitly solved.

Actual storage charges cover every retained physical payload copy, control ledger, transport backing, render pending array and terminal/barrier queue, including SPSC capacity+1 and largest allocation. Legacy host/CAPI resources do not grow merely because an opt-in service exists. The finite gates preserve original validation/replay/canonical behavior, B/B+1 and record bounds, in-flight overlap/density, completion credit reuse, partial/racing cancellation, actual two-owner transport, allocation/free liveness and zero operations, and genuine SAME-assertion production counter-controls. No fake consumer is described as PCM integration.

No scope outcome was lost: #140 retains production controller sequence transfer/lifecycle publication, exact Point/segment scheduling, actual DSP/readback and host rollout. #444 retains its typed native endpoint, application-sample cutoff/late behavior, and BOTH native bank and concurrent scalar pairing. The service's bounded transport boundary is not a queue-snapshot substitute for that application contract. Canonical D5 still applies only to the existing transient command category; persistent typed session edits remain transactional and snapshot-able. Neither parent nor broad IO-5/RT-4 closes with #460.

Allowed implementation remains protocol queue internals, one narrow service/module/export surface and focused protocol tests/evidence. No host/controller enablement, SPSC implementation change, new dependency, native endpoint, wire change, runtime DSP or timing is authorized. Current #459 and independent tooling retain ownership until their delivery boundary. After the future base check, normal Luna1/Sol2–3 and hard-stop review rules apply.

No implementation, tests, builds, timing, repository/spec changes or Git/GitHub mutations were performed; only this /tmp review was written.


## Delivery-base preflight while PR #466 awaits CI

# Astra #460 delivery-base preflight — sufficient, no scope adjustment

Read-only comparison of queued planning head `7508478b507ad9b6acc0fc2cf4fbf9ff78adcbf9` in `/home/bl/misofm/engine-140-ownership` against accepted PR466 head `a2bc849514724c8c85e41afa48970a281818d088`. The planning tree is clean. This is readiness conditional on actual #430/#459 delivery and root freezing/integrating that delivered base; it does not authorize implementation before the runtime WIP boundary.

No relevant source drift was found. Protocol sources (including queue/controller/control types), engine SPSC, host-core control provider, workspace/package Cargo and configuration inputs are unchanged between these trees. PR466 changes live builtin/graph pairing, existing tests and host test-support/artifact consumers; it does not change #460's admission, reliable reservation, control ownership or transport seam. The queued #460 changes since its earlier numbered approval are confined to its own spec. No implementation is hidden in the planning tree.

The concrete source seams remain exactly those briefed: queue.rs807 admits/validates fixed batches;835 is the legacy reservation-releasing dequeue;920 reserves reliable-event capacity;944 commits reserved reliable slots;1101 supplies the existing admission grammar/density/interval validation. Controller owns ProtocolQueues and exposes its existing queues_mut at2538, which is why the new owner must be exclusive and cannot expose that raw escape or also be installed in the controller. Engine SPSC still allocates physical capacity+1 and provides actual separate producer/consumer endpoints. No graph/builtin API migration is needed for this service.

## Effective frozen contract for assignment

Read the whole numbered body with its final cancellation amendment and root adoption taking precedence over preliminary signatures:

- The opt-in control owner exclusively owns ProtocolQueues and the sole admission/reservation ledger. B bounds outstanding tickets until terminal consumption, not resident SPSC occupancy. Handed-off/full/partially applied batches retain original full-ticket density/interval reservations until their exact terminal disposition is consumed. Unsupported mixed batches remain whole, owned, FIFO-blocking and cancelable.
- The narrow generic `P: Copy + Send + 'static` core is the one ticket/credit lifecycle. The automation adapter indexes its existing metadata by that same ticket and cannot independently retire/reuse it. Use actual preallocated SPSC ownership and fixed render pending data; no mutable raw automation escape, callback-driven admission or second validation table.
- Effective preparation includes `initial_reliable_event_sequence`, validated before publication. This is the sole standalone reliable sequence authority, not a second cursor alongside an active controller. Future production integration must transfer/unify the existing authority explicitly; it is outside #460.
- Effective begin_cancel takes reason AND event_revision, validates M-event headroom, reserves M actual reliable credits plus barrier request/ack credit before committing anything, and leaves state unchanged on refusal. Control retains ReliableEventReservations; render never receives its Arc or a protocol/event producer.
- Pending cancellation freezes/reconciles the original ticket set, including terminal returns not yet collected. Complete follows the actual matching boundary ack AND control-side publication of K existing automation_canceled slots in admission order, with exactly K consecutive sequences. Applied-only tickets emit none. Release unused credits and each original admission reservation exactly once after reconciliation; collect_terminal and copied-token drop cannot bypass publication.
- Charge all real physical storage, copies, ledgers, pending arrays, SPSC capacity+1 backing and maximum single allocation. No default host/CAPI allocation increase merely from defining this opt-in service. Preserve legacy dequeue/controller/wire/admission behavior and production host enablement state.

The approved finite identity/capacity/overlap/density/cancellation/backpressure/sequence/actual two-owner/zero-allocation and SAME-assertion controls remain sufficient. No new tests, framework, target matrix or architecture decision is added by this preflight. Allowed work remains narrow protocol queue internals/service/export/tests/evidence; no SPSC implementation change, dependency expansion, native endpoint, builtins/DSP application, host/controller enablement or wire change.

#140 retains exact Point/segment DSP application, controller sequence/lifecycle integration, canonical/transient semantics and host rollout. #444 retains native endpoint/cutoff/late behavior and native bank plus concurrent scalar pairing. Neither parent nor IO-5/RT-4 closes from #460's ownership service. After PR466 actually merges, root should record the resulting exact integrated base and confirm no intervening relevant source delta before Luna attempt1. The standard Astra review, Sol2/3 fallback and hard stop remain unchanged.

No repository/spec/Git/GitHub mutations, integration, tests, builds or timing were performed; only this /tmp report was written.


## Delivered integration and Luna attempt 1 assignment

PR #466 merged as `6589c5185411d51bce7d0a0aafab4df63a5e47db` after exact-head Astra PASS and required CI success; #430/#459 are remotely closed. Root integrated delivered main at `1e58ecc365ff166a244fc338c2ea0a11d88b8f56`. All runtime/build inputs are identical to main; the only differences are the retained approved #140/#444 planning records and this #460 spec. An initial root assertion expecting only the #460 spec was too narrow and stopped before writing assignment; the explicit three-spec comparison passed. The approved source preflight applies without runtime drift.

Root assigns Luna attempt 1 on the complete frozen contract and final cancellation amendment. Only the narrow protocol service, necessary queue internals, exports, focused tests and this evidence are authorized. Do not enable production hosts/controllers, alter wire grammar, SPSC implementation or dependencies. Preserve the sole ownership/sequence authority and all finite acceptance gates. Pause at each coherent compiling focused-green tranche for root checkpoint before layering more changes; final attempt receives one Astra verdict, with Sol fallback only after FAIL.


## Luna attempt 1 recovery checkpoint: initial compiling service

Luna paused after adding the opt-in delivery module, exports and crate-private retained-admission dequeue/release helpers. Legacy dequeue still releases admission immediately. This is an incomplete compiling implementation tranche, not source acceptance: generic payload/core factoring, complete capability-kind semantics, cancellation/resource requirements and the frozen adversarial acceptance evidence remain to be completed.

Luna reported cargo check and protocol library tests (124 passed) without retained log files. Root independently ran `PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-460-luna-target cargo test --locked -p protocol --lib`, exit 0 with 124 passed, zero failed. Root log/status: `/tmp/460-luna1-root-tranche1-test.{log,status}`. These inherited tests prove this checkpoint compiles and preserves existing library tests; they do not prove the new service contract. The three source paths plus this record are checkpointed before further implementation.


## Luna attempt 1 checkpoint 2: handoff and cancellation behavior

The second tranche adds exact handle/kind capabilities, resource projection, retained unsupported FIFO heads, queued-batch cancellation reconciliation and render partial-prefix cancellation. Two focused tests now exercise retained admission through handoff/terminal consumption and unsupported-head cancellation followed by event publication. `cargo test -p protocol --lib delivery::tests -- --nocapture` with the existing isolated target exited 0 (2 passed, 124 filtered); `/tmp/460-luna1-tranche2-test.{log,status}` retains evidence. Luna also ran cargo fmt successfully; root diff check passed.

This remains an incomplete checkpoint within Luna attempt 1. The full frozen generic ownership, physical resource, cancellation/backpressure/sequence/identity and realtime proof contract remains required before final review; these two tests do not substitute for it. Root checkpoints the two changed source paths and this record before continuation.


## Luna attempt 1 checkpoint 3: generic transport prototype, integration incomplete

The third compiling tranche adds generic PreparedDelivery<P> control/render endpoints and an independent u32 payload transfer test. Focused delivery tests report 3 passed, 124 filtered, exit 0 in `/tmp/460-luna1-tranche3-test.{log,status}`; formatting and root diff checks pass.

Root inspection finds that the concrete automation service still owns a separate ledger and transport implementation rather than instantiating the new generic core. This is NOT the frozen same-core contract and cannot receive acceptance as generic reuse. The next pass must consolidate the concrete adapter onto the generic ticket/credit lifecycle, then complete the retained capacity/cancellation/resources/realtime and two SAME-assertion gates. This useful compiling prototype is preserved as a recovery checkpoint, not a delivered capability or final attempt verdict. No additional parallel ownership model is accepted.


## Luna attempt 1 verdict and Sol attempt 2 assignment

# Astra #460 Luna attempt 1 — FAIL

Exact reviewed clean checkpoint `da7167ba87e79c101358a6e000b0ce644404eb92`, `/home/bl/misofm/engine-140-ownership`. One consolidated first-attempt verdict against the complete numbered scope, final cancellation amendment and adopted same-core requirement. Luna explicitly stopped unable to consolidate; this is an incomplete useful prototype, not an accepted capability. Preserve history and assign ONE coherent Sol2 revision in the finite five groups below. No further Luna repair, parent qualification or host enablement is authorized from this checkpoint.

## Accepted foundation and limits

The crate-private retaining dequeue/release split preserves legacy try_dequeue_automation behavior and reuses the original density/interval helpers. Default controller/host/wire behavior and dependencies are unchanged. Actual bounded SPSC endpoints, Copy records, exact handle/kind capability matching and control-owned reliable reservations provide useful starting points. Three focused tests cover a simple retained-capacity handoff, an entirely unhanded-off unsupported cancellation, and a separate u32 transport. The retained inherited124 tests and focused3 tests prove only those paths; they do not establish the frozen cancellation, generic reuse, physical resource or realtime contract.

## 1. One ownership core; no destructive error paths or raw escape

`PreparedDelivery<P>`/DeliveryCoreControl/Render (delivery.rs152-251) are independent from AutomationDeliveryControl/Render (362 onward). The latter still owns its own transport, tickets, ledger/outstanding counter and terminal lifecycle. It never instantiates the generic core. Consolidate the automation adapter onto the SAME `P: Copy + Send + 'static` ticket/transport/progress/terminal lifecycle, with the frozen logical record count1..=256. Attach original automation reservation metadata by that core ticket; do not keep a second slot allocator or independent retirement decision. The tiny alternate payload test must exercise that same implementation. No builtin dependency or new wire variant.

Remove public AutomationDeliveryControl::queues_mut (578). It defeats exclusivity: callers can dequeue/release original reservations, admit outside the outstanding bound, run legacy cancellation or publish reliable events outside the sole sequence owner. Expose only the narrow operations needed by this contract, including dequeue of already-published reliable events and an honest outstanding/resident report. An immutable report is fine; a mutable raw queue escape is not. Legacy separately prepared ProtocolQueues/controller remains unchanged.

Preserve ownership on every rejected operation. Current examples:

- Generic finish takes pending BEFORE validating ticket, so a wrong ticket loses the only pending copy. Generic collect pops a terminal before matching the caller ticket and takes the ledger entry before checking its identity, so a wrong caller ticket consumes someone else's terminal/credit.
- Automation finish_applied takes pending BEFORE checking ticket/prefix/completeness, losing accepted work on premature finish. Queue push errors also drop that taken payload. The current generic public prepare accepts arbitrary usize capacity but casts slot to u16 without a representability check; concrete put_ledger does the same. Use a slot representation covering the configured accepted capacity (for example usize); reject arithmetic/layout impossibility before allocation, never silently truncate or add an arbitrary ticket-cap limit.
- try_handoff_next dequeues first, then discards put_ledger errors via .ok(); serial overflow can turn accepted work into apparent Empty. A failed transport push clears the ledger and decrements outstanding rather than retaining the batch. Even if a full condition is intended to be structurally unreachable, establish that credit invariant and preserve ownership on a checked error rather than silently discard.

Prevalidate identities, serial/generation headroom and required credit before destructive transitions. Ticket stale/duplicate/bounds/prefix errors must leave pending/terminal/ledger ownership and original admission rows intact. Allocate ticket truth at the appropriate core transition so every admitted outstanding batch, including queued/unsupported heads, is accounted and cancelable. Preserve full-ticket reservations until CONTROL terminal consumption; no capacity reuse merely from handoff/render completion. FIFO unsupported/mixed batches remain whole and observable as PendingUnsupported. Document the narrow public ownership/error APIs rather than leaving conflicting prototype signatures.

## 2. A real ordered cancellation barrier and reconciled dispositions

Current begin_boundary ignores first_sample and barrier generation. It handles a Cancel on a separate channel, acknowledges with untyped u8, then may immediately pop an OLD DeliveryMessage from the other channel and return it as pending. Thus after handoff-before-render, begin_cancel followed by begin_boundary can acknowledge cancellation while exposing the canceled batch for application. This violates the central acknowledged-boundary contract.

Use the already-approved bounded transport choice: order the reserved cancellation barrier behind all committed handoffs, or carry and verify the exact committed handoff sequence before acknowledging. Preserve a reserved barrier/ack credit so ordinary handoff saturation cannot block cancellation. Bound each boundary's work and retain unprocessed commands; do not drain an unbounded producer stream. A matched ack includes cancellation identity and the ACTUAL boundary sample. No old-generation pending view/application may remain reachable after that ack.

Current cancellation discards terminal/ack push errors with `let _`, and ignores generation when consuming the ack. It never reconciles the terminal channel before publishing cancellation: LedgerEntry.applied_prefix remains0 because render prefix changes never update it. A partially applied batch is therefore reported fully canceled; a fully applied terminal queued before control consumption is likewise canceled incorrectly. collect_terminal also releases slots during pending cancellation without routing outcomes into the cancellation snapshot. Fix these as one state machine: freeze the exact outstanding set, reconcile applied/partial/canceled dispositions, hold terminal/barrier credits, match identity/sample, and retain any protocol-fault state. Applied-only tickets must not become cancellation events; partial tickets retain their original reservation identity until exactly one reconciliation.

## 3. Transactional reliable publication, ordering and sole sequence

Effective API is the FINAL amendment, not the prototype: prepare owns initial_reliable_event_sequence; begin_cancel takes reason/event_revision and returns a copyable identity token; poll(token) reports completion with actual boundary sample and counts AFTER reliable publication. Remove caller-supplied guessed effective_sample as cancellation acknowledgement. There is no competing caller/controller cursor, arbitrary event callback or unreserved event producer.

Current begin_cancel destructively moves queued batches into ledger before sequence-headroom/reliable reservation checks, so refusal already changes admission/ownership state. Reserve/prevalidate M-event headroom and all barrier/event credit before committing cancellation; failure must leave queues, tickets, ordering and cursor unchanged. Hold actual ReliableEventReservations only on control. Current cancellation iterates physical ledger slots; reused slots do not imply original admission order. Publish K nonempty cancellation events in ORIGINAL admission order with exactly K consecutive sequence values, original request/queue-generation identity, explicit event_revision and actual ack sample. Applied-only cancellation publishes0 and advances0. Release unused M-K credits on control and each original reservation exactly once.

Current generation increment occurs AFTER events/ledger release and can return overflow after partial committed success; prevalidate the required identity advance before committing. Current completion never resets the old automation ordering frontier, despite the existing queue helper, so the next legal epoch can remain spuriously rejected. Reset the old epoch only once all frozen dispositions/publications are complete. Repeated/stale token polls cannot duplicate events; token drop cannot release pending ownership. Preserve typed Pending until ack AND all relevant dispositions AND actual reliable-slot commits. Do not substitute transport consumption for delivery to a peer or claim graceful shutdown while accepted notifications are discarded.

## 4. Exact physical allocation report and bounded preparation

The current automation report covers its current queue headers/backings and heap ledger, but it is not an independent proof of the final same-core implementation, and the separately exported generic prototype has no corresponding resource report or overflow identity coverage. Consolidate first, then derive actual physical allocation charges from existing bounded_spsc_retained_payload and checked Layout/size_of: legacy queues, core ledger/reservation metadata, every payload-containing SPSC capacity+1 backing, terminal/barrier request/ack queues and any allocated pending storage. Charge each actual allocated copy, not merely logical B*256; distinguish inline returned-owner storage from heap allocations rather than inventing an allocation or silently omitting a real one. Preserve largest single allocation and preparation failure before endpoint publication.

The current pending Option is a single slot; if that remains a valid bounded implementation it need not become a new broad pending-array framework. Any chosen final storage must support the frozen ticket/barrier/disposition obligations and appear accurately in the report. Legacy ProtocolQueueResourceReport and default host/CAPI resources stay unchanged. No new dependency, allocator, SPSC implementation or CAPI mirror scope.

## 5. Complete the existing finite discriminating proof

The final pass must exercise the actual corrected service, not the separate generic demonstration plus a different concrete implementation. Reuse current protocol fixtures and existing allocator/audit infrastructure. No PCM application/mock may be presented as host integration.

- Capacity/admission: B/B+1; logical1/256/257 records; original invalid/overlap/density/past/global-order rejection with batch/state preserved; in-flight and partial-ticket reservation overlap; resident versus outstanding report; credit reuse only after terminal collection. Exact capability handle/kind and mixed unsupported whole FIFO head with a later supported batch.
- Identity/progress: actual payload/revision/request across separately owned SPSC endpoints; monotonic partial prefix; wrong serial/generation/slot, duplicate completion, regressing/overlong prefix and premature finish leave ownership intact and cannot double apply/release. Exercise actual returned terminal consumption and alternate Copy payload on the same core.
- Cancellation: before handoff, handed off not yet consumed, after prefix, and racing final completion using deterministic thread/barrier coordination, no sleeps. Check full reliable queue and sequence headroom refusal before mutation; reserved barrier at ordinary saturation; matched actual ack sample; no old-generation pending access afterward; correct K/remaining counts/admission order/consecutive sequence; applied-only0, unused capacity returned, stale/repeated token and collect_terminal interaction. Next same-handle legal batch is admitted only after correct terminal release/reset.
- Resources/realtime: independent exact allocation/layout and largest-allocation expectations, overflow refusal, repeated admission/handoff/render/terminal/cancel operations after preparation with positive allocation AND free liveness outside audit and zero of both in the assigned realtime spans. Use existing infrastructure and finite endpoint ownership, no new framework/global allocator. Do not count teardown or producer setup as render.
- Exactly TWO actual production counter-controls, ORIGINAL/mutant/restored SAME assertions: release reservation on handoff must fail the real in-flight overlap/capacity assertion; premature cancellation Complete/credit release must fail the actual boundary/event-presence/sequence assertion. Require targeted failure mechanism, not arbitrary compile/panic/setup failure. No additional campaign.

These are the original frozen acceptance groups; no fourth future matrix or architecture outcome has been added. Retain exact focused commands/statuses and honest failed/restored evidence; root checkpoints coherent tranches and receives ONE consolidated Sol2 verdict. Full workspace/targets/actual PR/requiredCI follow source PASS, not a substitute for these ownership discriminators.

## Revision boundary

Stay in protocol queue internals, one narrow shared delivery service/exports, existing focused protocol tests (one small separate-owner integration fixture if needed), and numbered evidence. #140 retains production controller/lifecycle/Point-DSP/model/host delivery; #444 retains native endpoint/cutoff/concurrent bank+scalar integration. No production host enablement, new wire/event kind, graph/builtin/rack/CAPI code, parser, dependency, SPSC algorithm, timing or architectural waiver. If a precise implementation obstacle truly cannot fit those approved seams, return it BEFORE expansion; do not deliver another parallel ownership service.

Review was read-only source/Git inspection. No tests, builds, timing, repository or GitHub mutations were performed. This is the complete bounded first-attempt FAIL and Sol2 correction list.

Root assigns one coherent Sol attempt 2 against these five finite groups. Preserve the compiling prototype history; consolidate the adapter onto the same core and complete the original gates without host enablement or scope expansion. Pause at coherent compiling checkpoints for root commits; one final Astra verdict follows the complete pass.


## Sol attempt 2 checkpoint 1: generic ownership error preservation

The generic core now preserves pending/terminal ownership on wrong-ticket finish/collection, advances serial only after successful publication and uses a representable usize core slot. Focused tests add wrong-identity preservation and capacity above u16: `cargo test --locked -p protocol --lib delivery::tests -- --nocapture` passed 5 tests. Test/fmt/diff statuses are all 0 under `/tmp/460-sol2-tranche1-*`. Root checkpoints only delivery.rs and this evidence before continuation. The concrete automation adapter is not yet consolidated onto this core; full Sol2 scope and final review remain outstanding.


## Sol attempt 2 checkpoint 2: common core and ordered cancellation

The automation adapter now uses the generic core ticket/ledger/transport/terminal lifecycle; unsupported reserved FIFO heads are not published to render. The mutable raw-queue escape is removed. Cancellation uses a typed token, committed handoff frontier and actual boundary sample acknowledgement; covered handoffs terminalize before acknowledgement. Adapter reservation metadata follows exact tickets and admission order. A focused handed-off/partial case checks actual-boundary cancellation and repeated/stale behavior.

The complete protocol library run passed 130 tests, zero failed; fmt/diff checks also exited0. Retained evidence is `/tmp/460-sol2-tranche2-{test,fmt,diffcheck}.{log,status}`. Root checkpoints the two source paths and this record before the remaining full frozen cancellation/credit/resource/realtime and causal-control proof. This is progress within Sol2, not final source acceptance.
