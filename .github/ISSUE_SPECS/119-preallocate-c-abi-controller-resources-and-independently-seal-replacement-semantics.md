# 119 Preallocate C ABI controller resources and independently seal replacement semantics

## Outcome and readiness

Close the bounded remainder from stopped Issue 118: make every retained C ABI controller/provider
resource eager and exactly owned, preserve the accepted Issue-022 resource-report meanings while a
separate admission owner accounts for double-live compiled models, and seal replacement semantics
with a genuinely independent resource oracle and production-origin cross-component evidence.

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1 AFTER REMOTE SYNC.** Sol High implements;
Sol XHigh briefs and verifies. One implementation pass plus one bounded HOLD correction is the full
budget; a second HOLD stops. Benchmark, timing, real-workload, playback and listening counters start
at zero and must remain zero.

Read-only remote inspection on 2026-08-23 found Issue 119 unallocated. Root must create and
synchronize it under this exact title after the docs checkpoint is upstream. This record makes no
GitHub mutation.

## Dependencies and routing by exact title

Accepted product dependencies:

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

Stopped **Close C ABI replacement resource accounting and cross-component evidence** (Issue 118)
is technical input only. Preserve its final technical checkpoint
`f9ad53896c21d03135e1ccf77c9a5dbe76a532ac` and tree
`2a2adaa57cfd69eb2e2f76457337e1a13b1a7ddb`, including the correct transaction order and expanded
evidence, but inherit no overall PASS or resource-accounting claim. Earlier stopped Issues 113 and
117 are transitive technical history only.

Accepted Issue 119 gates **Optional binary WebSocket sidecar** (Issue 025). Accepted Issues 116 and
119 jointly gate **Qualify native C ABI and reference runner target matrix** (Issue 114), which then
gates **End-to-end release, performance, and listening qualification** (Issue 026).

## Frozen accepted semantics

Preserve protocol wire/opcodes/statuses/diagnostics/event/replay/canonical bytes, controller affine
prepare/commit/cancel and one-call parity, queue reservation ownership, and replay/revision rules.
Preserve core publication/retirement reservation, render-boundary consumption, reclamation order,
plan behavior and realtime envelope. Preserve C ABI V1 layouts and the exact accepted symbol set.
Issue 116 and its runner, fixtures and publication contract are frozen and must not be invoked.

In particular, keep `PlanResourceReport.graph_session_plus_plan_bytes` equal to the Issue-022
`GraphResourceEstimate::session_plus_plan_bytes` meaning. Do not hide double-live compiled-model
admission in that public row or redefine `maximum_named_allocation_bytes`. A needed wire, core
behavior, public report-meaning or runner change is STOP/rescope.

## Exact eager resource product

At session/controller construction, allocate the full declared retained capacity for controller and
provider telemetry configuration payloads and every other fixed CAPI-owned command/event/replay/
diagnostic scratch object. Accepted commands may mutate bounded contents but may not cause lazy
retained growth. Construction failure is deterministic, leaves no partial session and disposes every
provisional owner once off render.

Keep active Issue-022 report rows unchanged. Introduce one separate internal admission owner for
the simultaneous current-plus-prospective compiled session models, with checked aggregate and
max-single-allocation accounting. The complete admission projection must also own, once each, the
current/prospective graph, canonical/session/source/effect/builtin/controller/exchange/provider,
replay/event and fixed payload state. Exact configured caps succeed; every named one-below cap fails
before protocol/model/provider/plan visibility changes. Cancellation, full queues and all compile or
publish failures release reservations and retained candidates exactly once.

The test oracle must be external/manual and independently derived from frozen primitive layouts and
configuration inputs. It must not call the CAPI production projector or production projection
helpers such as protocol queue, replay-cache or plan-exchange resource-report functions. It must
distinguish aggregate retained bytes from the largest single allocation and compare both independently
against production results.

## Complete residual cross-component matrix

Cross the exported C surface and use production-origin commands, controller/provider activity and
render transitions to prove:

1. all 11 command families have exact accepted responses, statuses, revisions, state/provider
   effects, diagnostics and replay decisions, including immediate/new, cached, conflict, expired,
   stale, event-full and publication-full cases;
2. all six event families have exact frame bytes from their production origin, reliable FIFO/no
   drop, lossy 1/1 coalesce/drop behavior and counters, invalid lane, empty success, zero-capacity
   query, one-short canary/non-consumption and exact retry;
3. source-preserving and source-changing replacement retain old PCM/provider before the boundary,
   activate the exact new pair at the boundary, preserve/reset buffered state by the frozen policy,
   produce nonzero source-changing PCM and advance provider epochs exactly;
4. serial replacement under publication and retirement pressure performs publication, render
   consumption, off-render reclaim and credit-release retry in order, with exact revision/epoch/
   replay ordering and no leaked publication or retirement credit;
5. success, cancellation and every ordered phase/dual-fault row preserve canaries and unchanged
   state when rejected, and count candidate/current provider, plan, token, replay, reservation,
   allocation, drop and disposal ownership exactly; both destroy orders remain guarded; and
6. armed render has zero allocation/free, lock, syscall, I/O, log or plan/provider destruction.

Behind-C event injection or a C result compared only with the same production `SessionState` is not
an independent semantic oracle. Minimal read-only accepted projections may expose state for tests,
but may not manufacture the behavior being proved.

## Allowed paths and focused gates

Product fixes are limited to `crates/miso-engine-capi/**` and the minimum protocol resource-
construction or read-only accepted seam required by the gates. Core implementation behavior is
read-only. No copied protocol registry or alternate state machine is authorized. The header may be
inspected but no new symbol/layout is allowed.

Allowed support edits are focused CAPI/protocol resource and parity tests, exact static/policy/
mutation checks and Issue-119 evidence/routing docs, plus unavoidable minimal manifest/lock rows.
Graph/DSP/effects/source decoding, host/browser/mobile code, session schema, core plan behavior,
Issue-116 runner and Issue-114 tooling remain read-only.

Required gates are locked focused CAPI/protocol tests with only minimum read-only core projections;
C11/C++17 exact 14-symbol ABI checks; warning-denied Clippy/rustdoc; format; Wasm exclusion; no-copy,
realtime and resource policies plus effective mutations; shell syntax; exact allowed-path/static/
diff/artifact scans; and zero prohibited counters. Sol High stops at one immutable focused-green
checkpoint. Sol XHigh returns strict PASS or the sole bounded HOLD; the correction review must PASS
or STOP. No benchmark, timing, workload, playback, listening, browser/device execution or runner
invocation is authorized.
