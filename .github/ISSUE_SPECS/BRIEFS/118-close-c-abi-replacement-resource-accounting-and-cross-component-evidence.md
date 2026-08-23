# Sol implementation brief — issue 118 C ABI replacement accounting and evidence

## Decision

**TERMINAL STOP AFTER SECOND SOL XHIGH HOLD.** Technical checkpoint
`f9ad53896c21d03135e1ccf77c9a5dbe76a532ac` (tree
`2a2adaa57cfd69eb2e2f76457337e1a13b1a7ddb`) is retained without overall PASS. It improved retained
aggregate rows, source reset/change PCM and command/event evidence, but the independent oracle still
called production projections, telemetry storage remained lazily growing, the frozen Issue-022
graph report meaning was redefined, and retirement/reclaim/credit plus construction/disposal/
allocation/drop and production-origin vector evidence remained incomplete.

The one-pass-plus-one-HOLD budget is exhausted. Benchmark/timing/real-workload/playback/listening
counters remained zero. No accepted capability or downstream gate follows from Issue 118.

Accepted dependencies are **Transport-neutral binary control protocol** (005), **Real-time memory,
buffers, queues, and plan lifetime** (003), and **Stable C ABI and host-fed planar PCM render**
(022). Stopped **Complete C ABI transactions with two-phase protocol and plan reservations** (117)
is technical input only. Remaining work is Issue 119, **Preallocate C ABI controller resources and
independently seal replacement semantics**. Accepted 119 gates **Optional binary WebSocket sidecar**
(025); accepted 116 + 119 gate **Qualify native C ABI and reference runner target matrix** (114),
then 026.

## Smallest closable product

Replace standalone-candidate CAPI accounting with an exact phase peak that includes simultaneously
live current/prospective canonical/session/source-provider payloads, protocol reservations, old/new
plans, provider/report epochs, publication/retirement credits and fixed scratch/replay/diagnostics.
Every byte has one owner. Exact limit succeeds, one-below rejects before mutation, arithmetic is
checked, active report meanings stay frozen, and cancellation/reclaim releases all provisional
state once off render.

Then prove through the real C boundary: exact parity for all 11 commands; exact bytes and lane policy
for all six event families; empty/query/one-short non-consumption; immediate/cached/conflict/expired/
full replay decisions; source-preserving/changing PCM before and at the boundary; exact provider
epochs and host submission; serial replacement/retirement/reclaim; plan-handle guards and both
destroy orders; and every cancellable phase plus ordered dual faults with canaries and exact drop/
allocation counters. ID-only dispatch tests or behind-controller event injection are insufficient.

## Frozen seams and fence

Do not change protocol wire/status/diagnostic/replay/event/canonical bytes, accepted controller token
or one-call semantics, core reservation/credit semantics, ABI layouts/symbols beyond the already
frozen event dequeue, session schema, source decode, graph/DSP/effects, render constraints, runner
Issue 116 or Issue-114 tooling. Do not copy protocol decoding/dispatch into CAPI.

Edit only bounded CAPI product/tests, the minimum accepted protocol/core test seam proved necessary,
the event header/smoke only if preserving its frozen symbol, exact policies/mutations, minimal
unavoidable manifests/lock rows and Issue-118 evidence/routing docs.

Run focused locked CAPI/protocol/core gates, C11/C++17 ABI smoke, strict Clippy/rustdoc/fmt, Wasm/
realtime/resource/no-copy policies and mutations, shell/static/diff/artifact scans. Sol High hands
off one immutable checkpoint. Sol XHigh returns PASS or the sole HOLD; the correction is final. No
benchmark, timing, workload, playback, listening, browser/device or runner invocation.
