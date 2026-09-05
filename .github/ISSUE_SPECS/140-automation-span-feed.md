# Automation-span feed: route live parameter and fader commands into the running plan

## Current parent disposition

This is the synchronized stateless mirror of existing OPEN #140 under audit #349 IO-5. Root adopts decisions D1–D5 below on delivered source `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`: preserve current admission with explicit pending-only unsupported batches; retain bounded ownership/reservations until reliable terminal disposition; cancellation is a boundary-acknowledged lifecycle barrier; prepared native-effect Point delivery uses exact local slices; transient automation leaves canonical base SessionModel/revision unchanged and publishes truthful applied readback. These are parent-level design decisions, not permission for speculative source work.

The candidate A/B/C outcomes below must be numbered, assigned exact source APIs/resources/gates and approved by Astra before implementation. #435 remains the sole runtime feature and #442 is queued ahead of this work. Historical console capabilities and #143 observation delivery remain credited, but pending protocol batches are not considered applied. #140/IO-5 cannot close after an ownership service or first scalar integration. IO-18 and other owners’ SDK work remain separate.

# Proposed amended #140 parent: deliver admitted automation to the running plan

**Planning proposal, not implementation authorization.** Restore a numbered local #140 mirror and synchronize this amended accounting scope before child implementation. Keep #140 OPEN. Root must explicitly adopt the decisions below and number/synchronize bounded children before assigning Luna. This parent does not expand active #435, queued #442 or #430's live fader/matrix integration; IO-18/browser SDK ownership remains separate.

Revalidated immutable main `4b352b36` source, without reading root's dirty runtime edits. The relevant current facts remain: protocol admission atomically pushes a fixed batch then commits interval/density/frontier accounting; normal dequeue removes those reservations, and the only production consumer cancels. Graph ConsoleEffect drains EffectControlLane into block-first Point spans, coalescing same-parameter records. That is NOT a consumer for protocol timestamps. Existing EffectProcessBlock::new permits nonempty slices of length <= prepared quantum and checks endpoint overflow: local effect sub-block processing is an available seam, unlike a speculative whole-graph quantum subdivision. This makes a bounded Point application child possible without changing every effect's automation grammar.

## Parent accounting and closure

Already delivered under the historical #140 console branch: live effect parameter/bypass spans, ramped fader/mute, console metadata and application-sample/PCM fixtures. The former GR remainder is delivered through #143 observation taps, not a new gain_reduction method. Preserve current tests and accepted later browser lineage; do not rewrite the old issue comment's evidence as if it had run later browsers. #370 delivered an explicit documentation/deferral decision only. Its retained queue must now receive a scoped implementation; it must not be called fixed because cancellation consumes it.

Remaining parent outcomes: admitted protocol automation reaches actual PCM at defined samples, no admitted work disappears through handoff/cancellation, applied-state readback is truthful, and all currently accepted target/kind combinations have an explicit delivery disposition. Native and browser/other-host rollout, bank placement, segments and qualification may have separate children. Point-only or a synthetic queue consumer cannot close #140 or #349 IO-5. IO-18's shared console-command transport service is a different obligation: no host-web command extraction, SDK request-ID changes or wire consolidation is authorized here.

## Recommended binding decisions for root adoption

### D1: Preserve existing admission; add delivery without pretending all kinds are implemented

Do not change the existing AutomationEnqueue wire or silently refuse previously accepted kinds in existing endpoints. Current success means durable admission, not an appliedAtSample ack: AutomationEnqueued carries count/occupancy/capacity/generation only. Points include requested timestamps; actual application is proved by PCM and existing observed-sample readback.

First rollout may deliver only a preparation-declared subset while retaining unsupported accepted batches in their existing cancellation-capable ownership. A batch is eligible for initial handoff only if EVERY record has a supported target/kind; mixed batches remain pending whole. Record that pending-only behavior explicitly, expose no false complete-delivery capability and do not let a blocked subset silently starve deliverable work without a documented bounded policy. Prefer a prepared delivery-capability table in Rust; no new wire field in the first child. If root instead wants typed refusal for undeliverable kinds, that is an explicit endpoint capability/admission amendment requiring separate compatibility/replay tests and documentation before code, not implementer discretion.

### D2: One bounded ownership ledger from ack through terminal disposition

Introduce a prepared delivery state machine inside the existing protocol queue boundary, using fixed batch slots and existing SPSC vocabulary. Each admitted batch retains one logical ownership ticket through queued, handed-off/pending, partially applied and terminal states. Handoff moves/copies a fixed record representation into render-owned preallocated storage; render never borrows ProtocolController, provider or SessionModel. Preparation resolves revision handles into stable plan-local target bindings off render.

The current dequeue helper is NOT a safe handoff: it releases interval/density reservations too soon. Keep those reservations until the corresponding outstanding records are applied or reliably canceled, or define a checked transfer into equivalent prepared reservations with no gap. Preserve existing global ordering/replay rules. Count capacity against all outstanding accepted work rather than giving unlimited fresh credit after handoff. If queue occupancy's externally visible meaning changes to include in-flight tickets, state that conservative accounting explicitly; capacity does not expand and saturation remains typed before admission. No compiled MAX_TRACKS or allocation based on stem duration.

Reserve pending storage and terminal-status capacity before making a ticket render-visible. A simple bound is at most B tickets, at most B*256 retained records (checked arithmetic), and at most one terminal return credit per ticket, all prepared from configured limits. Returned credit remains unavailable for reuse until control has consumed completion/cancellation and released its reservations. Partial application updates remaining-record accounting without dropping the ticket. No render-side error path may drop heap ownership, wait, allocate or emit a reliable protocol message.

### D3: Cancellation is a lifecycle barrier, not a fire-and-forget generation bump

Control reserves reliable cancellation capacity before committing a cancel operation, as today. A full reliable queue defers cancellation/reconfiguration and leaves ownership intact. For handed-off work, a bounded cancel request is observed at a defined render boundary; completion acknowledges which records actually applied before that boundary and how many remain. Control emits exactly one correctly counted terminal cancellation notification per affected batch for remaining records; already applied records must not be labeled canceled. No silent queue generation reset can invalidate render-pending ownership.

Define the linearization race: an old generation may run only until the acknowledged boundary; a new plan/locate/revision requiring its cancellation cannot claim effective application before that boundary. Reuse the existing plan publication/retirement discipline; if matching it needs a new multi-phase lifecycle API, freeze that exact adaptation in the integration child before coding. Queued-only cancellation remains synchronous with existing reservation behavior. Endpoint shutdown/provider loss must retain/reclaim pending storage off render and account cancellation; no new render destructor path.

### D4: Exact Point scheduling through local effect slices

For the first Point-capable native effect path, process the prefix before the earliest due timestamp, then invoke the same prepared processor on the next nonempty sub-slice with Point spans at that sub-invocation's first_sample. Two points at offsets 3 and 7 in a 16-frame quantum must both apply: [0,3), [3,7), [7,16), not a last-wins update at block start. Group simultaneous points in the existing canonical parameter/channel order. Never invoke a zero-frame block. Future events remain pending; sample=end belongs to the next block. Preserve the existing per-effect coefficient/smoothing arithmetic, fixed latency and full graph/source quantum.

Use a configured density bound to bound subdivisions and scans. No per-sample controller call, whole-graph rescheduling, new interpolation or altered FMA/association. Point catch-up applies an already admitted late point at the next available render boundary and increments a saturating per-record late counter once; already rejected past-at-admission batches stay rejected. That proposed catch-up detail must be explicitly recorded rather than invented in implementation. Distinct already-late same-parameter points need a deterministic ordered application rule, not silent coalescing; test their state transition semantics.

### D5: Existing transient enqueue versus persistent typed session mutations

D5 applies exclusively to the existing transient AutomationEnqueue operation. Its pending/applied runtime overlay does not edit the canonical base SessionModel or increment its revision; ParameterStatePage reports the coherent observed live value and sample. This does not change persistent protocol mutation semantics. SessionTransactionApply, including persistent parameter/fader/matrix edits and automation upsert/remove/target/segments, must update the same typed SessionModel transactionally and remain snapshot-able as canonical JSON. No host/controller shadow document may substitute for that model.

Preserve both sides in the integration proof: transient enqueue/application/cancellation leaves the canonical base snapshot and revision unchanged while readback changes at the specified observed sample; a representative persistent typed edit changes the same canonical model/snapshot and revision as specified. Pending values must not be reported as already applied. Any future conversion of transient automation into stored session automation must use the existing typed transaction semantics, not mutate JSON or session structure on render. Existing persistent-edit/snapshot tests may supply that proof with an exact link to their unchanged behavior; no new corpus is required.

## Smallest concrete children and serial dependency

These are candidate scopes ready for root's decisions and subsequent numbered exact-base briefing, not permission to launch three parallel feature edits. Each has an independently reviewable outcome; no single issue combines all ownership, DSP invocation and host lifecycle work.

### A — Retain accepted automation reservations across bounded delivery ownership

Boundary: protocol queue/controller internals and existing protocol tests; existing realtime SPSC API is reused without changing it. Implement D2 ticket transfer/terminal return and D3 queued-versus-handed-off cancellation accounting as an additive prepared Rust service. Leave the production host delivery endpoint disabled until integration; existing enqueue/replay/default cancellation behavior remains unchanged.

Acceptance: B and B+1 admissions, max256/oversized record handling, reservation unchanged after handoff, overlap/density rejection against in-flight work, terminal credit reuse only after consumption, double-completion/replay cannot free twice, partial application cancellation counts, generation race and reliable-full refusal preserve all ticket ownership. Tiny fixed-capacity fixtures and existing queue tests suffice; no general concurrency framework. This child earns the ownership capability, NOT PCM delivery or IO-5 closure. Freeze exact resource formula/API at its numbered brief.

### B — Apply two prepared Point events inside a native effect block

Boundary: effect-contract prepared helper and its focused conformance/native-effect test seam, with graph binding deferred to C. Implement D4 using existing <=quantum EffectProcessBlock slicing; reuse PreparedAutomationSpan and unchanged processor arithmetic. Choose one actual launch native effect and existing automatable parameter from its descriptor in the numbered brief (no invented ID). This child must process a real prepared effect and prove output, not just sort timestamps.

Acceptance: exact prefix/two offsets/next-block boundary/late event; independent old processor calls split manually at known offsets as oracle; canonical FP finite signed-zero representatives; retained state/reset and smoothing; no-event output unchanged; invalid shapes/prepared capacity rejected before writes; positive live allocator plus repeated actual process zero audit. Do not generalize every effect or bank in this child. If the selected effect's sub-block behavior is not already partition-consistent, stop and choose/rebrief explicitly rather than changing DSP algorithm to fit the helper.

### C — Connect admitted native protocol Points to one running plan

Depends on accepted A+B. Boundary: host-core/C ABI controller-plan preparation and the minimal graph per-node effect binding, using existing protocol frames and real SessionControlProvider; existing C ABI live render/transaction tests. First representative is one track with one supported native effect on the existing scalar per-node path, avoiding new bank pairing or #430 edits. No new public C ABI entry point is needed merely to submit existing AutomationEnqueue, but any necessary prepare configuration/public capability exposure must be frozen before assignment.

Admission of two points at offsets3/7 must produce correct PCM through the actual C ABI command and render calls, with requested timestamp/observed readback proof. Preserve ack/replay, full/invalid whole-batch rejection, application before/after handoff cancellation and reliable saturation, revision replacement/locate boundary, base canonical snapshot invariance and live readback, no-event identity and repeated real render zero audit. Require one actual mechanism witness that deleting handoff or collapsing both points makes the PCM assertion fail. Existing host lifecycle tests remain. No “mock consumer” acceptance or all-bank/sample-segment claim.

C may exceed half a day if lifecycle publication adaptation spans a second independently useful control boundary. In that event split the preparation/binding outcome from actual host lifecycle integration BEFORE source work; root must not silently compress missing cancellation semantics to fit a schedule. A and B already isolate the two largest independent risks.

### Retained completion after C

Parent explicitly retains additional currently admitted parameter/target coverage, bank/scalar placement equivalence, step/linear/exponential segment semantics and host rollout. Each needs its own bounded current-source brief; no placeholder is called complete. Browser delivery/shared console service is coordinated with IO-18 and SDK owners separately, with no inspection or overwriting of their worktrees. A new descriptive capture/target expansion is qualification scope after an actual product reaches PCM; mandatory safety/identity/current supported target and PR/required-CI gates still belong to delivered runtime children.

## Workflow / closure

Root adopts or amends D1–D5 in the mirrored parent, numbers children with reciprocal retained obligations, freezes an actual merged base, then Astra approves the exact child's APIs/allowed paths/gates before Luna1. Sol only after FAIL, at most two coherent revisions, then hard stop/rescope. #435 remains the sole active feature now; these are queued. Do not close #140/#349 IO-5 after A, B or the first scalar C alone. No benchmark, new protocol wire, SDK refactor, artifact promotion or implementation is authorized by this planning document.

## Adopted-decision precedence

The current disposition adopts the proposed D1–D5 recommendations, superseding proposal wording that asks root to choose those semantics. All exact-child API/base/numbered approvals remain pending. The original issue scope is preserved in its GitHub history; the historical empty-span and gain-reduction statements are superseded by the delivered-console and #143 accounting above, not revived as missing work. No original capability outcome is dropped.

## Canonical-model contract clarification

# #140 canonical-model ruling

**D5 is consistent only as the narrowly defined existing AutomationEnqueue transient-delivery contract. It is not an exception to the user's rule for canonical session mutations. Amend its wording to make that distinction explicit before child briefs; no new wire or runtime implementation is required by this clarification.** Root adoption alone would not authorize overriding the user architecture.

The existing protocol deliberately has two distinct typed operations:

- `docs/CONTROL_PROTOCOL_SEMANTICS.md:7` defines SessionTransactionApply as atomic replacement of SessionModel/CompiledSession/revision, with canonical snapshot of the committed model. `docs/CONTROL_PROTOCOL_REGISTRY.md:64–74` includes persistent parameter/fader/matrix edits AND automation upsert/remove/target/segments opcodes0600–0603. `crates/protocol/src/model.rs:356–369,667–684` implements those typed automation edits; snapshot at801 returns compiled canonical JSON. `crates/session/src/model.rs:512–521` identifies stored Automation by stable ID with segments preserved canonically. These operations remain governed without qualification by the user's same-model/snapshot rule.
- `docs/CONTROL_PROTOCOL_SEMANTICS.md:13` explicitly says AutomationEnqueue does not increment revision, and15/17 distinguish queued delivery/cancellation from structural transactions. `crates/protocol/src/queue.rs:31` expressly describes its kinds as transient automation. Controller dispatch at3107 validates/adopts fixed queue records; it does not call SessionEdit application. The wire registry at30 gives admission count/occupancy/capacity/generation, while parameter-state pages separately carry observed_sample and live records. A transient runtime overlay is therefore an existing distinct operation, not a second persistent session model invented by D5.

The no-revision fact alone would be insufficient justification; the explicit distinct queue operation plus existing canonical automation-edit route establishes the distinction. Conversely, a live readback cache is not proof that a persistent protocol mutation is snapshot-able. D5 cannot be applied to SessionTransactionApply, persistent parameter edits or stored session automation merely because they eventually affect render parameters.

## Exact bounded wording correction

Replace D5's open-ended “recommended ruling / if root instead requires” choice with this binding clarification:

> D5 applies exclusively to the existing transient AutomationEnqueue operation. Its pending/applied runtime overlay does not edit the canonical base SessionModel or increment its revision; ParameterStatePage reports the coherent observed live value and sample. This does not change persistent protocol mutation semantics. SessionTransactionApply, including persistent parameter/fader/matrix edits and automation upsert/remove/target/segments, must update the same typed SessionModel transactionally and remain snapshot-able as canonical JSON. No host/controller shadow document may substitute for that model.
>
> Preserve both sides in the integration proof: transient enqueue/application/cancellation leaves the canonical base snapshot and revision unchanged while readback changes at the specified observed sample; a representative persistent typed edit changes the same canonical model/snapshot and revision as specified. Pending values must not be reported as already applied. Any future conversion of transient automation into stored session automation must use the existing typed transaction semantics, not mutate JSON or session structure on render.

The existing persistent-edit/snapshot tests can supply the second proof if their exact unchanged behavior is linked; no new corpus or duplicated mutation mechanism is requested. D1–D4 pending ownership/cancellation, actual sample-offset PCM delivery and child queue remain unaffected. This is a narrow interpretation of the existing command categories, not permission to waive the user rule for a newly introduced “live” mutation.

Read-only adopted parent and current protocol/session source/document inspection. No edits, Git/GitHub operations, builds/tests or timing performed.
