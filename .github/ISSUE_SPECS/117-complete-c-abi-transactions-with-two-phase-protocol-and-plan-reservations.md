# 117 Complete C ABI transactions with two-phase protocol and plan reservations

## Outcome and readiness

Add the two missing control-plane capabilities that made stopped Issue 113 unimplementable, then
use them to complete byte-exact C command/event transport and atomic structural replacement through
the accepted render-plan lifecycle.

**STOPPED AT TERMINAL CHECKPOINT-2 REVIEW / TECHNICAL INPUT FOR ISSUE 118 ONLY.** Checkpoint 1
received Sol XHigh PASS at commit `c9bd936673bfe167d783ca6f2a62c495c0928f37`: the shared
controller-bound two-phase protocol token and plan publication/retirement reservation satisfy their
focused contracts. Checkpoint 2 preserved the correct C transaction order and additive event ABI,
but its CAPI accounting omits double-live current-plus-prospective payloads and its focused evidence
does not close the required cross-component matrix. The bounded HOLD budget is exhausted. Issue 117
has no overall PASS and gates nothing. Benchmark, timing and real-workload counters remained
`0/0/0`.

Read-only remote inspection on 2026-08-23 found Issue 117 unallocated. Root must create and
synchronize it under this exact title after the docs checkpoint is upstream. This record makes no
GitHub mutation.

## Dependencies and routing by exact title

Direct accepted dependencies are:

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

Stopped Issue 113, **Close C ABI control/event transport and transactional plan replacement**, is
readiness evidence only. It proved that the accepted controller's irreversible one-call dispatch
and the accepted plan exchange's missing reservation seam cannot satisfy the atomic C transaction;
it is not an accepted product dependency.

Successor **Close C ABI replacement resource accounting and cross-component evidence** (Issue 118)
consumes this stopped technical checkpoint plus accepted Issues 005, 003 and 022. Accepted Issue 118
gates **Optional binary WebSocket sidecar** (Issue 025). Accepted Issues 116 and 118 jointly gate
**Qualify native C ABI and reference runner target matrix** (Issue 114), which then gates Issue 026.

## Smallest closable product vertical

Complete exactly three connected seams; none is independently claimed complete:

1. add an opaque, bounded two-phase protocol transaction capability for structural session
   commands while preserving the existing one-call API and all accepted command behavior;
2. add a bounded plan-replacement reservation that accounts for publication and eventual displaced-
   plan retirement before any protocol state becomes visible; and
3. integrate those capabilities in CAPI with byte-exact event egress, matched source-provider
   epochs and the existing next-block-boundary plan exchange.

This issue is the minimum honest vertical because the post-prepare commit sequence must be
non-fallible. A protocol token without the plan reservation still permits partial commit, and a plan
reservation without a prospective protocol token still requires duplicating accepted controller
semantics in CAPI.

## Protocol two-phase contract

Add an affine, opaque and non-`Clone` prepared-command token owned by one exact controller instance
and controller generation. Preparation accepts the same command bytes and caller response capacity
as the accepted one-call API and performs the same decode, semantic, revision, replay preflight,
resource and diagnostic ordering. For a new structural command it computes and owns the
prospective strict compiled session/model/revision, canonical snapshot bytes, exact response bytes,
reliable event, automation cancellation effects and replay completion reservation without changing
any live controller state.

Preparation has three closed outcomes:

- an immediate accepted response for malformed, unsupported, stale, cached replay, replay conflict,
  insufficient output or typed backpressure decisions that require no external plan;
- one prepared structural token plus read-only prospective session data required for source binding
  and plan compilation; or
- the existing exact typed error/diagnostic result, with no token and no mutation.

The token exposes only bounded read-only prospective data plus `commit` and `cancel`/drop. For an
affinity-valid, current and unconsumed token, commit's state application must be non-fallible and
must apply the prospective session, revision, reliable event, cancellation and replay completion
exactly once with the same bytes and ordering as the accepted one-call path. Token-affinity/use
validation precedes that non-fallible application; wrong-controller, stale-generation,
already-consumed and mismatched-token use rejects without mutation. Cancel/drop releases all
reservations and changes nothing. A prepared structural outcome changes no live session, event,
automation or replay state before valid commit.

The existing `process_command_frame_into` remains source-compatible and byte-compatible by using
the shared preparation/commit machinery internally. Nonstructural commands retain their accepted
bounded one-call behavior. No opcode, frame, status, diagnostic, revision, event, replay key,
cache/expiry or canonical-session byte changes are permitted.

## Plan publication and retirement reservation

Add an affine `PlanReplacementReservation` or equivalently narrow capability that is created on the
control side before protocol commit. It binds the exact exchange identity/generation, render
envelope, next plan epoch, one publication slot and one retirement credit for the active plan that
the candidate will eventually displace. Reservation failure returns the candidate/resources with
typed full, incompatible or epoch-exhausted classification and changes no queue, epoch or plan.

Committing a valid reservation with its exact complete `PreparedRenderPlan` is non-fallible,
consumes the epoch exactly once and publishes for the next boundary. Cancel/drop releases both
credits. The render owner consumes the attached pre-admitted retirement credit when swapping; it
must not make a new fallible capacity decision after publication. The displaced plan and its matched
source-provider epoch enter bounded retirement and are reclaimed/dropped only off render.

The implementation may add fixed preallocated credit accounting around the existing SPSC queues,
but it may not move allocation, destruction, locking, logging, I/O, syscalls or unbounded work into
render. Serial reservations/publications preserve epoch order; a pending candidate cannot be
overwritten; credits cannot leak, duplicate or be forged. Existing callers of the one-shot
`PlanPublisher::publish` remain source-compatible or receive an explicitly documented wrapper over
the same reservation semantics. Existing render reports remain byte/meaning compatible; a
previously accepted unreserved publication may retain its documented retirement deferral, but the
Issue-117 C transaction path may use only fully reserved replacements.

## C ABI transaction and event contract

Preserve ABI V1 and every existing layout/symbol. Add only the previously frozen event-egress
symbol using the opaque session handle, existing `miso_engine_v2_bytes_out`, and a fixed `uint32_t`
reliable/lossy lane selector. Invalid lane is invalid argument. Empty is success with
`required_bytes = 0`; too-small output reports the exact length and consumes nothing. Reliable
events are never silently dropped; lossy events keep the accepted coalescing/drop-counter policy.

For a structural C command, perform this exact order off render:

1. prepare the protocol token and prospective strict typed session;
2. prepare every required source producer/consumer endpoint and compile one complete replacement
   plan under the accepted limits;
3. reserve the exact publication/retirement capability and verify response/event/replay capacities;
4. execute the non-fallible protocol-token commit into controller-private state without returning
   or exposing its response to the caller; then
5. install the matched new source-provider epoch in the call-private control state, non-fallibly
   publish the complete plan, and only then return the exact committed response/event/replay result.

Every phase before step 4 is cancellable and leaves the live revision, canonical session, source
provider, plan, response, event and replay state unchanged. Steps 4–5 contain no remaining fallible
operation; tests must prove that invariant rather than compensate with rollback. The sole serial
control owner exposes neither intermediate state nor the response between those steps, while the
publish-last order prevents a concurrent render boundary from observing the new plan before its
protocol commit. Before the next render boundary, the old plan/provider remains active. At the
boundary the exact new pair becomes active, and only later control-side reclamation drops the old
pair. Host submission addresses only the committed provider epoch. Destroy remains off render and
requires caller quiescence.

## Diagnostics, resources and atomicity gates

- Preserve the accepted protocol phase/diagnostic order. CAPI-only argument, lane, buffer and
  resource failures retain the existing stable C result/diagnostic vocabulary; no second wire
  registry is added.
- Admission precedes allocation/compilation/hooks wherever the required size is already known.
  Resource reports include protocol-token storage, prospective session/snapshot, response/event/
  replay reservations, plan/publication/retirement credits and source-provider epochs. Exact limit,
  one-below and arithmetic-overflow rows must identify their real owner.
- Caller output is atomic. Insufficient response/event buffers, partial source/compile failures and
  every reservation/cancel path preserve canaries and perform no visible mutation. All provisional
  allocations and by-value capabilities are disposed exactly once off render.
- Dual-fault rows freeze decode -> protocol semantics/replay -> prospective session/source -> compile
  -> response/event/replay admission -> publication/retirement admission -> commit ordering.

## Required focused evidence

1. Protocol parity covers every command family, immediate/cached/conflict/replay-expiry decision,
   exact response/event/revision/canonical bytes, prepared cancel/drop, wrong owner/generation,
   double use and serial tokens. Live state must remain byte-identical until commit.
2. Core tests cover publication full, retirement-credit exhaustion, incompatible envelope, epoch
   exhaustion, cancellation/reuse, serial order, pending ownership and render-boundary application.
   Armed render proves zero allocation/free, lock, syscall, I/O, log or plan/provider destruction.
3. C/Rust tests cover all accepted commands and six event families, empty/one-short buffers,
   reliable/lossy behavior, source-preserving and source-changing structural edits, old/new boundary
   output, exact provider epochs, serial replacements, retirement/reclamation and destroy order.
4. A failure matrix injects every phase through the last cancellable step and proves session,
   revision, provider, plan, response/event/replay and caller buffers unchanged, with exact drop and
   allocation accounting. A static check forbids copied protocol decoding/dispatch in CAPI.
5. C11 layout/symbol smoke, warning-denied focused Rust gates, Wasm exclusion, policy mutations,
   shell syntax and clean diff/artifact scans pass. No target matrix from Issue 114 is duplicated.

## Allowed paths and frozen boundaries

Allowed product changes are limited to:

- `crates/miso-engine-protocol/src/controller.rs`, the minimum prospective-session seam in
  `crates/miso-engine-protocol/src/model.rs`, exports and focused tests;
- `crates/miso-engine-core/src/realtime/plan_exchange.rs`, its exports and focused tests;
- `crates/miso-engine-capi/**`;
- minimal manifests/lock changes required by those crates;
- the hand-written C ABI documentation/header only for the additive event symbol; and
- exact CAPI/protocol/realtime policy checks, mutation tests and Issue-117 evidence/routing docs.

Frozen: protocol wire/opcodes/status/diagnostic/replay/event bytes, public session schema, source
decoder behavior, graph/DSP/effect algorithms, render envelope and constraints, accepted ABI V1
layouts/symbol behavior, runner/fixtures, host/browser/mobile products and Issue-114 qualification
tools. A need for a wire/schema/render algorithm change is STOP/rescope.

## Checkpoints and completion

Checkpoint 1 freezes the shared protocol token and plan/retirement reservation with exhaustive
focused state/resource/realtime tests. Checkpoint 2 freezes CAPI integration, event egress,
source-provider retirement and cross-component atomicity. Both are tranches within the one allowed
implementation pass; Sol High stops after each coherent focused-green tranche for Sol XHigh review
and root checkpointing.

Overall PASS required both focused checkpoints plus one clean proportional nonbenchmark seal on an
immutable candidate. Issue 117 stopped before that seal and does not unblock Issue 025 or Issue 114.
Issue 118 owns the bounded completion without weakening these gates.

## Terminal checkpoint evidence

Checkpoint 1 passed at commit `c9bd936673bfe167d783ca6f2a62c495c0928f37`. It established the
accepted-semantics-preserving affine protocol prepare/commit/cancel token, controller-generation and
outstanding-token guards, zero live mutation before commit, queue-owned event reservations, and the
core publication-slot plus pre-admitted retirement-credit reservation. Its focused reports were 91
protocol tests and 38 core tests, with strict checks/policies green and prohibited counters zero.

Checkpoint 2 was reviewed as a live seven-path implementation tranche over that commit and was
preserved as technical checkpoint `e1115750fba8a54e16ec2a0e333b40ce4f187f1c`, tree
`e256f9101cb211c8bbf459efc8b11d3334785150`:

- `crates/miso-engine-capi/include/miso_engine_v2.h`;
- `crates/miso-engine-capi/src/abi.rs`;
- `crates/miso-engine-capi/src/ffi.rs`;
- `crates/miso-engine-capi/src/runtime.rs`;
- `crates/miso-engine-capi/tests/c/abi_smoke.c`;
- `crates/miso-engine-protocol/src/lib.rs`; and
- `scripts/check-capi-abi.sh`.

Sol XHigh confirmed direct `ProtocolController` preparation, private protocol commit followed by
bounded provider/report installation, non-fallible reserved plan publication and response exposure;
the additive reliable/lossy event dequeue ABI also preserved empty/query/one-short non-consumption.
The terminal review nevertheless stopped because `capi_resources` counted canonical/source-ID/
`ControlSource` payloads only once while current and prospective epochs coexist, so an exact accepted
limit can be exceeded during replacement. The added tests dispatched 11 command IDs and decoded six
event-family IDs but did not prove exact cross-component bytes/state or the required PCM boundary,
source epoch, full/dual-fault, destroy, disposal and allocation matrix. The reported checkpoint-2
focused counts were 17 CAPI and 91 protocol tests with policy/mutation gates green and prohibited
counters zero; those results are technical evidence only and cannot override the missing gates.
