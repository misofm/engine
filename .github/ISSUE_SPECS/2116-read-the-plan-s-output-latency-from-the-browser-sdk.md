# Read the plan's output latency from the browser SDK

Filed by root from console slice C1a (2026-10-26).

## Problem
The browser SDK gives no call to read the current plan's output latency, although the module exports `miso_engine_web_v1_plan_latency_read`. The console's fixed shape has 579 samples of latency at 48 kHz (soft clip on three stages plus the limiter). The app needs it to align meters, the playhead and any visual with the audio, and today it would have to hard-code it.

## Smallest slice
`engine.planLatency(): Promise<{ samples: number, ... }>` (or a field on the status), read from the host export, on both the browser and headless engines, with the same meaning as the C ABI's latency read. A README line.

## Gates
One eval: a session with a known latency (for example a limiter insert) reports exactly that number.

## Evidence and decision record
- Attempt 1 (2026-10-26), findings before any code. No Rust change is needed: `hosts/host-web/src/ffi.rs` already exports `miso_engine_web_v1_plan_latency_read`/`_ptr`, a 64-byte `WebPlanLatency` record (`latencySamples`, `tailKind`, `tailSamples`; layout in `sdk/src/generated/abi.ts` as `planLatency`). Its words are the control plane's `resources()` row for the plan that renders now, the same words as the C ABI's `miso_engine_v1_plan_resources` (`latency_samples`, `tail_kind`, `tail_samples`); a rebuild's successor shows from the first read after the block that adopts it, and `hosts/host-web/tests/control_parity.rs` holds both hosts alike across a pending swap. The record carries no revision or generation.
- Neither SDK engine can reach the read yet. The browser worklet (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`) has no message for it, and the host JS has no method. The headless engine reaches the module only through `WasmBoundary` (`sdk/src/core/boundary.ts`), which has no read. The slice therefore also needs two paths outside the brief's list: `sdk/src/core/boundary.ts` (a `readPlanLatency()` beside `readWatermark()`) and `tools/realtime-policy/render-thread-exports.json` (the worklet would call `plan_latency_read` in its message handler after ready, which `scripts/test-web-audioworklet.mjs` requires the table to list). Attempt 1 stopped here for authorization.
