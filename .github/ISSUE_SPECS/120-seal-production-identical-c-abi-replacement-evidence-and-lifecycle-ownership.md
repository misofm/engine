# 120 Seal production-identical C ABI replacement evidence and lifecycle ownership

## Outcome and readiness

Close the smallest remaining C ABI replacement boundary after stopped Issue 119: make the tested
resource and event semantics identical to the production library, derive the complete transaction
admission oracle from primitive external authority, and observe real construction/Drop ownership
rather than a procedural test ledger.

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1 AFTER REMOTE SYNC.** Sol High implements;
Sol XHigh briefs and verifies. One implementation pass plus one bounded HOLD correction is the full
budget; a second HOLD stops. Benchmark, timing, real-workload, playback and listening counters start
at zero and must remain zero.

Remote Issue 120 is unallocated. Root must create and synchronize it under this exact title after
the docs checkpoint is upstream. This record makes no GitHub mutation.

## Dependencies and routing by exact title

Accepted product dependencies:

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

Stopped **Preallocate C ABI controller resources and independently seal replacement semantics**
(Issue 119) is technical input only. Preserve technical checkpoint
`1a3dde27e78634243d4474c13959697e672a7b33` and tree
`3edd0b8db01672ea2a625fc6e2deef38734c5eb1`, including eager controller/provider/replay arenas,
the restored Issue-022 report row, separate compiled-model admission, byte-pinned command vectors
and accepted replacement order. Inherit no overall PASS, oracle, event-origin, layout or lifecycle
claim. Stopped Issues 113, 117 and 118 are transitive technical history only.

Accepted Issue 120 gates **Optional binary WebSocket sidecar** (Issue 025). Accepted Issues 116 and
120 jointly gate **Qualify native C ABI and reference runner target matrix** (Issue 114), which then
gates **End-to-end release, performance, and listening qualification** (Issue 026).

## Frozen product authority

Protocol wire/opcodes/statuses/diagnostics/event/replay/canonical bytes, controller affine
transaction semantics, queue reservations and replay/revision rules are immutable. Core plan
publication/retirement reservations, render-boundary ownership transfer, reclamation and realtime
behavior are immutable. Preserve C ABI V1 layouts and the exact accepted 14-symbol set. Preserve
Issue-022 public resource-row and limit meanings, including
`graph_session_plus_plan_bytes == GraphResourceEstimate::session_plus_plan_bytes`, while keeping
double-live compiled models under the separate internal admission owner. Issue 116, its runner,
fixtures and publication contract are frozen and must not be invoked.

The useful Issue-119 eager construction and transaction mechanics may be corrected but not
redesigned. Remove the production-visible conformance mutator; protocol edits are limited to the
resource-construction correction or genuinely read-only observation needed by CAPI. No copied
protocol registry, alternate dispatcher, new ABI symbol, core behavior change or report
reinterpretation is authorized.

## Production-identical resource semantics

All exact resource assertions must execute against the same layouts and code paths shipped in the
production library. No `cfg(test)` field, queue, counter, observer or synthetic producer may alter a
type whose size or retained payload is charged by `CapiResources`, `PlanResourceReport` or the
double-live admission calculation. A production and test build of the same target/configuration
must emit identical public resource rows and accept/reject identical exact and one-below caps.

Remove every lazy retained-growth path, including the Issue-119 provider metadata/state conformance
mutator. Controller/provider telemetry and replay state remain fully eager at construction. Any
required production event handoff must itself be bounded and preallocated, charged once to the
correct active owner and covered by the named-largest rule. Exact-cap success and one-below atomic
rejection cover the complete current-plus-prospective phase without changing Issue-022 public row
meanings.

## Truly primitive full-admission oracle

Create an external integration oracle that starts from frozen primitive type layouts, explicit
configuration capacities and independently enumerated fixture/session shapes. It must manually
derive both aggregate retained bytes and largest-single-allocation bytes for the full active and
replacement phase: both compiled models; current/prospective graph, source, effect and builtin
owners; controller/exchange/provider/replay/event arenas; canonical/source-ID storage; plan/report/
handle payloads; response/token reservations and every retained candidate.

The expected path must not call `CompiledSession::resource_estimate`, CAPI projection/admission
functions, graph/source/effect/builtin resource reports, protocol queue or replay projection,
plan-exchange projection, or another production helper that returns the value being proved. Frozen
primitive sizes/constants and independently parsed fixture facts are allowed. Production values are
read only for the final comparison. Mutations must prove that changing or omitting each owner and
confusing aggregate with maximum-single fails the oracle.

## Production-origin C event and command evidence

Retain the pinned exact response vectors for all 11 commands and independently pin their statuses,
revisions, state/provider effects, diagnostics and replay outcomes for new/cached/conflict/expired/
stale/event-full/publication-full cases. Do not derive expected bytes or state from a second
production `SessionState`.

All six event families must reach the existing C dequeue symbol from real shipped pathways:

- session-committed, automation-canceled and transport-state originate from their exported-C
  commands;
- diagnostic, meter-batch and counter-snapshot originate from the bounded production CAPI provider/
  render observation handoff, not a `cfg(test)` collector or direct queue/stage helper.

The production pathway must preserve render constraints: render may publish only fixed preallocated
observations through an accepted bounded handoff; encoding, controller mutation and diagnostics
remain off render. Tests configure and stimulate it only through existing exported C calls and
ordinary host PCM/render actions. No hidden mutable protocol/CAPI setup backdoor is allowed.

For every family pin exact bytes and prove reliable FIFO/no-drop, lossy 1/1 coalescing/drop counters,
invalid lane, empty success, zero-capacity query, one-short canary/non-consumption and exact retry.

## Real ownership and failure evidence

Observe actual constructor and destructor execution for current/candidate provider, current/
candidate plan, prepared token, current/candidate replay arena and publication/retirement
reservation. Counters or probes must fire at the real construction/Drop/commit/reclaim sites; a
counter increment immediately before a return is not disposal evidence. Observation must neither
change production resource-counted layouts nor manufacture a drop, and the production-disabled/no-
observer path must have the same owned resource shape.

Run all six cancellable structural phases and the complete ordered 6x6 dual-fault matrix through
exported C. Each rejection preserves response/output canaries, canonical/model/revision, provider/
plan epochs, events/replay, reservations/credits and production resource rows; actual constructor/
Drop counts balance exactly. The successful row publishes once, render consumes at the boundary,
control reclaims old plan/provider off render, retirement credit is released and an exact retry
succeeds. Both destroy orders and rejected initial construction dispose all owners once. Armed
render proves zero allocation/free, lock, syscall, I/O, log, controller mutation or destruction.

## Allowed paths and gates

Product edits are limited to `crates/miso-engine-capi/**` and the minimum protocol resource-
construction/read-only correction needed to remove Issue-119 divergence. Core, header, Issue-116,
Issue-114 tooling, session/graph/source/effect/builtin semantics and host/browser/mobile code are
read-only. Allowed support is an external production-layout CAPI resource/lifecycle test or fixture,
focused CAPI/protocol tests, exact static/policy/mutation checks, unavoidable minimal manifest/lock
rows and Issue-120 evidence/routing docs.

Required gates are locked focused CAPI/protocol tests with production-layout integration evidence;
C11/C++17 exact 14-symbol checks; warning-denied Clippy/rustdoc; format; Wasm exclusion; no-copy,
realtime, resource and production/test-parity policies plus effective mutations; shell syntax;
exact allowed-path/static/diff/artifact scans; and zero prohibited counters. Sol High stops at one
immutable focused-green checkpoint. Sol XHigh returns strict PASS or the sole bounded HOLD; the
correction review must PASS or STOP. No benchmark, timing, workload, playback, listening, browser/
device execution or Issue-116 runner invocation is authorized.
