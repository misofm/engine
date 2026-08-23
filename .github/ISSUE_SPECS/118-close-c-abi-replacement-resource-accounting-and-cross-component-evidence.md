# 118 Close C ABI replacement resource accounting and cross-component evidence

## Outcome and readiness

Close the smallest remaining C ABI transaction product after stopped Issue 117: account exactly for
the current-plus-prospective double-live replacement state, correct any bounded CAPI integration
defects exposed by that accounting or the required matrix, and prove the complete cross-component
command/event/replay/source-provider/PCM-boundary contract.

**TERMINAL STOP AFTER SECOND SOL XHIGH HOLD.** The final technical checkpoint is
`f9ad53896c21d03135e1ccf77c9a5dbe76a532ac` (tree
`2a2adaa57cfd69eb2e2f76457337e1a13b1a7ddb`). It improves the evidence and implementation but has
no overall PASS. The bounded review budget is exhausted; remaining work moves statelessly to Issue
119. Benchmark, timing, real-workload, playback and listening counters remained zero.

Read-only remote inspection on 2026-08-23 found Issue 118 unallocated. Root must create and
synchronize it under this exact title after the docs checkpoint is upstream. This record makes no
GitHub mutation.

## Dependencies and routing by exact title

Accepted product dependencies:

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

Stopped **Complete C ABI transactions with two-phase protocol and plan reservations** (Issue 117)
is technical input only. Preserve its accepted checkpoint-1 commit
`c9bd936673bfe167d783ca6f2a62c495c0928f37` and checkpoint-2 technical commit
`e1115750fba8a54e16ec2a0e333b40ce4f187f1c`, but inherit no overall PASS or resource/evidence
claim.

Accepted **Preallocate C ABI controller resources and independently seal replacement semantics**
(Issue 119) gates **Optional binary WebSocket sidecar** (Issue 025). Accepted Issues 116 and 119
jointly gate **Qualify native C ABI and reference runner target matrix** (Issue 114), which then
gates **End-to-end release, performance, and listening qualification** (Issue 026).

## Terminal review decision

Checkpoint `f9ad53896c21d03135e1ccf77c9a5dbe76a532ac` separated aggregate retained totals from
maximum-single-allocation rows, pinned the source-reset policy, added source-changing nonzero PCM,
and materially expanded command, event query/one-short/retry and lossy-counter evidence. The second
review nevertheless could not return PASS:

- the claimed independent resource oracle still invoked production projection helpers before
  duplicating their formula, so it was not an external/manual oracle;
- controller and provider telemetry vectors began empty and grew lazily even though the projection
  charged hypothetical maximum retained arrays, defeating exact eager resource ownership;
- the patch redefined Issue-022's graph/session/plan report row by adding compiled-model bytes,
  instead of retaining that ABI meaning and admitting double-live compiled models under a separate
  owner; and
- the residual matrix did not prove retirement render/reclaim/credit retry, complete construction/
  disposal/allocation/drop counters or every ordered failure phase. Several event rows still used
  behind-C injection rather than production-origin command/provider vectors.

These are technical findings only. Issue 118 is not accepted and grants no downstream capability;
Issue 119 consumes the checkpoint and closes only this bounded remainder.

## Frozen accepted technical seams

Do not redesign or copy the accepted protocol or core semantics. Retain:

- the controller-bound affine structural prepare/commit/cancel token, exact generation/outstanding
  guards, zero live mutation before commit and byte-identical one-call behavior;
- queue-owned reliable-event/replay/cancellation reservations and direct non-fallible commit;
- the exchange-bound publication slot plus pre-admitted displaced-plan retirement credit, cancel/
  drop release and render-boundary consumption with off-render reclamation;
- direct CAPI dispatch through `ProtocolController`, without a second opcode/semantic registry;
- structural ordering of protocol prepare -> complete source/plan prepare -> all admissions ->
  private non-fallible protocol commit -> matched provider/report install -> non-fallible plan
  publication -> caller response; and
- ABI V1 plus the sole additive session event-dequeue symbol with fixed `uint32_t` reliable/lossy
  lane values 0/1.

Protocol wire, opcodes, statuses, diagnostics, event/replay bytes, revision/cache rules, canonical
session bytes, core plan-exchange semantics, render envelope, graph/DSP/effects, source decoder and
accepted Issue-116 runner are immutable. A required change to any frozen authority is STOP/rescope.

## Exact double-live resource accounting

Define and enforce the real maximum retained state at every structural phase, not the size of one
standalone candidate. The accounting must include simultaneously live current and prospective
canonical/session storage, source-ID bytes, `ControlSource` arrays and endpoint/provider payloads;
protocol prepared-token/snapshot/response/event/replay reservations; current, pending and retired
provider/report rows; complete old/new plans; publication slot and retirement credit; fixed CAPI
scratch/diagnostic/replay storage; and every temporary allocation retained across commit.

Each byte belongs to one named resource owner exactly once. `maximum_capi_retained_bytes`, graph/
session/plan limits and `maximum_named_allocation_bytes` must reject before visible mutation when the
true phase peak is over limit. Exact-limit succeeds; one-below fails with the existing stable owner
diagnostic; checked arithmetic/platform conversion failures are deterministic. Cancellation,
backpressure, compile rejection, plan destruction and successful reclamation dispose provisional
state exactly once off render and release all credits without leaking capacity.

Resource reports must describe the accepted active state with their existing ABI meanings while
admission separately proves the transaction peak. Do not silently redefine an existing report field
to mean a temporary peak unless Issue 022 already defines it that way.

## Cross-component product matrix

C and Rust tests must cross the actual ABI/controller boundary and prove:

1. all 11 accepted command families use `ProtocolController` and preserve exact accepted response
   bytes, statuses, revisions, provider effects, diagnostics and replay outcomes for immediate, new,
   cached, conflicting, expired and full cases;
2. all six event families leave the C dequeue symbol as the only new ABI surface and preserve exact
   frame bytes, reliable FIFO/no-drop behavior, lossy coalescing/drop-counter behavior, invalid lane,
   empty success, zero-capacity query, one-short canary/non-consumption and exact retry;
3. source-preserving and source-changing edits preserve the documented old output/provider before
   the boundary, activate the exact new plan/provider pair at the boundary, route serial host
   submissions to the specified committed epoch, preserve or reset buffered source state exactly as
   the accepted contract requires, and reclaim old pairs only later off render;
4. serial replacements preserve plan epoch/revision/replay order across publication full and
   retirement-credit pressure; session-first and plan-first destroy orders are quiescent, leak-free
   and guarded against structural publication after the plan handle is gone;
5. failure injection at every cancellable phase, plus ordered dual faults, proves unchanged live
   session/canonical bytes, revision, provider, plan, response/event/replay queues, caller canaries,
   credits and resource counters; and
6. successful and rejected paths have exact construction/drop/allocation accounting, while armed
   render proves zero allocation/free, lock, syscall, I/O, log or plan/provider destruction.

Tests that merely observe a message ID or inject only behind the C boundary do not satisfy exact-byte
or cross-component parity. The matrix may reuse accepted protocol/core fixtures as independent
oracles but may not restate their decoding, dispatch or state-transition logic in CAPI.

## Smallest allowed product corrections

Product edits are limited to bounded corrections in `crates/miso-engine-capi/**` and the minimum
accepted protocol/core seam needed to expose existing state or inject deterministic test faults.
Such a seam must preserve public bytes and accepted behavior and may not introduce a second control
implementation. The hand-written header and C smoke may change only if needed to preserve/verify the
already-added event symbol; no further ABI symbol or layout is authorized.

Allowed support edits are exact CAPI/protocol/realtime static checks, mutation suites and Issue-118
evidence/routing docs, plus unavoidable minimal manifests/lock rows. Issue-116 runner code, fixtures
and publication contract; Issue-114 qualification tools; host/browser/mobile code; session schema;
source decode; graph/DSP/effect algorithms; and benchmark inputs remain read-only.

## Focused gates and acceptance

Required gates are locked focused CAPI/protocol/core tests; C11 and C++17 header/layout/symbol smoke;
warning-denied Clippy/rustdoc; format; Wasm exclusion; no-copied-protocol and realtime/resource
policies plus effective mutations; shell syntax; exact allowed-path/static/diff/artifact scans; and
zero prohibited counters. Do not duplicate Issue-114 platform/runner qualification.

Sol High stops after one coherent focused-green implementation checkpoint. Sol XHigh returns strict
PASS or the sole bounded HOLD. After a HOLD, one exact correction and one final review must either
PASS or STOP. Overall PASS requires exact double-live limit/one-below proof and the complete matrix
above on one immutable candidate. No benchmark, timing, real workload, playback, listening,
browser/device execution or Issue-116 runner invocation is authorized.
