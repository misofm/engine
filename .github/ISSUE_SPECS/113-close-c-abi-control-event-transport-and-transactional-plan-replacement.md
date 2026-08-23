# 113 Close C ABI control/event transport and transactional plan replacement

## Outcome and readiness

Complete the launch C ABI control product omitted from immutable Issue 022: expose the accepted
Issue-005 command/response/event contract and publish structurally edited sessions as complete
replacement plans at render block boundaries with bounded off-render retirement.

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1 AFTER REMOTE SYNC.** Sol High implements;
Sol XHigh briefs and verifies. One implementation pass plus one bounded HOLD correction is the
complete budget. A second HOLD stops. Benchmark, timing and real workload counts are zero and must
remain zero.

Read-only boundary inspection found remote Issue 113 unallocated. Root must create/synchronize it
under this exact title after the docs checkpoint is upstream. This record makes no GitHub change.

## Dependencies by exact accepted title

- **Transport-neutral binary control protocol** (Issue 005)
- **Real-time memory, buffers, queues, and plan lifetime** (Issue 003)
- **Stable C ABI and host-fed planar PCM render** (Issue 022)

Issue 025, **Optional binary WebSocket sidecar**, follows this issue. The reference runner in Issue
073 is independent and does not gate this product.

## Frozen product slice

Preserve ABI V1 version and every existing symbol/struct field. Add only an event-egress function
whose hand-written C signature uses the existing opaque session handle and
`miso_engine_v2_bytes_out`, plus a fixed `uint32_t` lane selector for reliable or lossy Issue-005
events. Invalid selector is invalid argument; empty is OK with `required_bytes = 0`; too-small
output reports the exact required length and consumes nothing. Reliable events are never silently
dropped; lossy meter/counter events retain the accepted coalescing/drop-counter semantics. No Rust
layout, callback or allocator crosses C.

Replace the capability-only C provider with the accepted bounded `ProtocolController` and one
C-specific provider over canonical typed session state. `submit_command` retains exact request-ID,
revision, replay, response and diagnostic bytes from Issue 005. It must reserve response, reliable
event, replacement and retirement capacity before visible mutation. Malformed/unsupported/stale/
backpressured commands leave revision, canonical session, source routing and live plan unchanged.

### Transaction and plan lifecycle

Nonstructural transport/parameter/automation commands update only their accepted bounded control/
event queues. A committed structural transaction performs this exact off-render order:

1. decode and validate the complete Issue-005 transaction against the current canonical model;
2. create the prospective strict typed session and canonical TOML in scratch;
3. prepare every required host-source producer/consumer endpoint and compile a complete replacement
   `PreparedRenderPlan` under the compile-time limits;
4. reserve the response/reliable-event/replay rows and a publication plus retirement slot;
5. publish the complete plan for the next block boundary; and only on accepted publication commit
   the new revision, canonical model, source-provider epoch and response/event/replay state; and
6. reclaim the displaced plan and its matching retired source-provider epoch only through the
   control-side retirer, never on render.

Publication/retirement backpressure returns the accepted typed backpressure without partial
commit. Host submission after commit addresses the new provider epoch; the old provider remains
alive until its matching displaced plan is reclaimed. No render call allocates/frees, locks,
decodes, logs, performs I/O or waits. Destroy is off-render and drains/reclaims only after caller
quiescence. Resource reports include every new fixed controller/event/replay/publication/provider-
epoch byte and one-byte-below limits reject before either original child publishes.

## Acceptance gates

- All accepted Issue-005 command families and six event families traverse caller-owned C buffers
  with byte-exact Rust/C parity, exact replay and revision behavior, reliable/lossy ordering and
  stable result/diagnostic classification.
- Representative source-preserving and source-changing structural transactions prove pre-boundary
  old-plan output, next-boundary new-plan output, canonical snapshot equality and exact source epoch
  routing. Full retirement defers without mutation; later reclamation drops plan/providers only off
  render. Serial replacements preserve order.
- Every failure phase—decode, semantic validation, session encode, source preparation, compile,
  response/event/replay reservation, publication and retirement capacity—proves atomic session,
  source, revision and plan state plus by-value disposal of provisional allocations.
- Allocation/syscall/drop audits arm render across steady state and replacement boundaries; exact
  limit/resource rows, long counts without a compiled track ceiling, C11 layout/symbol smoke and
  Wasm exclusion pass. No benchmark, timer or listening gate is permitted.

## Allowed paths

Product edits are limited to `crates/miso-engine-capi/**` and the minimum sealed orchestration seam
inside `crates/miso-engine-core/src/realtime/**` if the accepted exchange cannot carry the matched
source-provider retirement epoch. Focused tests may use `crates/miso-engine-protocol` only without
changing its accepted wire/semantics. Root manifests/lock, C ABI docs, exact C ABI checker/policy
mutations and this issue's routing/evidence docs are permitted.

No source decoder behavior, graph/DSP/session schema, protocol bytes/opcode/diagnostic registry,
runner, host application, browser/mobile adapter, benchmark or timing change. A need to edit those
contracts is STOP/rescope.

## Checkpoint and evidence policy

Checkpoint 1 freezes the additive C event API, controller/provider and exhaustive byte/diagnostic/
resource tests. Checkpoint 2 freezes transactional replacement, provider-epoch retirement and
realtime/atomicity evidence. Sol High stops after each coherent focused-green tranche for Sol XHigh
review; the one-pass budget covers both planned checkpoints, not two attempts. Record exact paths,
hashes, counts and zero prohibited invocations. Overall PASS requires the complete focused product
and a clean proportional nonbenchmark seal; it does not wait for Issue 114.
