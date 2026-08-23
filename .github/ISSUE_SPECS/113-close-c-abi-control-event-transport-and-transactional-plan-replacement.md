# 113 Close C ABI control/event transport and transactional plan replacement

## Terminal decision

**TERMINAL ARCHITECTURE STOP / NO IMPLEMENTATION / NO OVERALL PASS.** The stateless readiness audit
proved that this issue's exact atomicity gate is not implementable through its frozen CAPI plus
single core-seam boundary. Sol High made no implementation edit. Sol XHigh confirmed the STOP
before pass 1 rather than copying accepted protocol semantics or weakening the transaction.

The audited clean baseline was commit `b5be8148b7651024307eca17b664b09a07a13122`, tree
`cef7922aff699afb292e22fa13953356aa875753`. No Cargo/build, benchmark, timing, workload, render,
audio, fuzz, Git or GitHub mutation was run. Benchmark, timing and real-workload counters are
`0/0/0`.

## Dependencies and successor

The stopped shape followed these exact accepted dependencies:

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

The stateless successor is Issue 117, **Complete C ABI transactions with two-phase protocol and
plan reservations**. Issue 113 is readiness evidence only and is not an accepted dependency. Issue
025 consumes accepted Issue 117 directly. Issue 114 waits for accepted Issues 116 and 117.

## Confirmed architecture blocker

The accepted `ProtocolController::process_command_frame_into` is an irreversible one-shot path. It
performs replay preflight/reservation, dispatches the command, writes the response and completes
replay. For a structural transaction, `execute_decoded_command` reserves event/cancellation rows,
then `SessionStore::apply_transaction` commits the compiled session/model/revision before the
reliable event and replay completion are committed. There is no prospective transaction token,
prepare/commit split, rollback, snapshot or external publication hook.

The accepted `PlanPublisher` exposes only fallible publication. Retirement admission remains a
render-boundary operation in `RealtimePlanOwner::enter_block`; the control side has no publication
reservation/cancel token and cannot pre-reserve the displaced plan's bounded retirement capacity.
Therefore neither possible order satisfies the frozen gate:

- protocol first can commit session/revision/event/replay before plan publication or retirement
  admission fails; and
- plan first can publish a replacement before protocol response/replay/session commit fails.

Reimplementing or copying the controller transaction in CAPI would create a second protocol and
was expressly forbidden. Adding the missing protocol and core reservations exceeded Issue 113's
allowed product boundary. This is an architecture STOP, not evidence of a product regression in
accepted Issues 003, 005 or 022.

## Preserved contract and non-evidence

The original required outcome remains useful input for Issue 117: byte-exact Issue-005 C command,
response and event transport; prospective strict-session compilation; matched source-provider
epochs; next-boundary complete-plan replacement; bounded off-render retirement; exact typed
backpressure; and no visible mutation on any failed phase. Existing ABI V1 layouts and symbols,
protocol wire/event/replay bytes, render constraints, session schema, sources, graph/DSP/effects,
runner and host products remain frozen.

No focused implementation checkpoint, test result, qualification result or remote synchronization
is claimed for Issue 113. Its only accepted output is this candid STOP and the bounded Issue-117
route.
