# Sol implementation brief — issue 119 eager C ABI resources and independent replacement seal

## Decision

**TERMINAL STOP AFTER SECOND SOL XHIGH HOLD.** Preserve technical checkpoint
`1a3dde27e78634243d4474c13959697e672a7b33` (tree
`3edd0b8db01672ea2a625fc6e2deef38734c5eb1`) without overall PASS. Eager resource arenas, frozen
Issue-022 report assignment, separate compiled-model admission, pinned command bytes and bounded
replacement mechanics are technical input. The oracle remained production-derived; event origins
and resource layouts diverged under `cfg(test)`; a mutable protocol backdoor grew uncharged provider
vectors; and procedural counters did not observe actual Drop ownership.

Accepted dependencies are **Transport-neutral binary control protocol** (005), **Real-time memory,
buffers, queues, and plan lifetime** (003), and **Stable C ABI and host-fed planar PCM render**
(022). Remaining work is Issue 120, **Seal production-identical C ABI replacement evidence and
lifecycle ownership**. Accepted 120 gates **Optional binary WebSocket sidecar** (025); accepted 116
+ 120 gate **Qualify native C ABI and reference runner target matrix** (114), then 026.

The one-pass-plus-one-HOLD budget is exhausted. Benchmark/timing/workload/playback/listening
counters remained zero. Issue 119 grants no accepted downstream capability.

## Smallest closable product

Preallocate at construction every declared controller/provider telemetry configuration vector and
fixed CAPI retained payload; accepted commands must not lazily grow retained state. Preserve the
Issue-022 graph report row exactly, and place simultaneous current/prospective compiled models under
a separate internal admission owner. Account graph/source/effect/builtin/controller/exchange/
provider/replay/event/fixed payloads once, with distinct aggregate and largest-single meanings,
checked arithmetic, exact-cap success and atomic one-below rejection.

Build an external/manual oracle from primitive layouts and configuration inputs. It may not call
production CAPI, protocol queue, replay-cache, plan-exchange or other production projection helpers.
Compare its aggregate and max-single results independently with production.

Through exported C, prove all 11 command cases and all six production-origin event frames; exact
status/response/state/provider/replay behavior; reliable/lossy query/one-short/retry; source-preserve
and source-change PCM/reset/epochs; serial publication plus render/reclaim/credit retry; every phase
and ordered dual fault; both destroy orders; and exact allocation/drop/disposal/reservation counters
and canaries. Event-only backdoor injection and same-model self-comparison are insufficient.

## Frozen fence and gates

Protocol bytes and behavior, core plan/exchange/render behavior, C ABI symbols/layouts and Issue-022
public report meanings are frozen. Issue 116 is frozen. Product edits are limited to CAPI and the
minimum protocol resource-construction/read-only accepted seams; core may supply read-only
projections only. Do not copy protocol semantics.

Run focused locked CAPI/protocol gates with minimum core projections, C11/C++17 exact-symbol smoke,
strict Clippy/rustdoc/fmt, Wasm/realtime/resource/no-copy policies and effective mutations, and
shell/static/diff/artifact scans. Hand off one immutable checkpoint for strict Sol XHigh PASS or the
sole HOLD; its correction is final. No benchmark, timing, workload, playback, listening, browser/
device or runner invocation.
