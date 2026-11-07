# Web host: a trap in a worklet message-handler export escapes the handler, and the dispose after it never completes

Tracker: #2258, the follow-up #2256 recorded in its non-goals ("Containing a trap in an export the message handler calls (control command, seek, source submit). It still escapes the handler"). Base `9c1ef3ea7`.

## Problem
The release browser module is `panic = "abort"` (`Cargo.toml` `[profile.release]`), so a Rust panic in any export, including an allocation failure inside a control command's rebuild, traps the instance and surfaces in JavaScript as a throw. #2256 contained that throw at two call sites in `hosts/host-web/web/miso-engine-v1-audio-worklet.js`: the render export in `process()` (line 1735) and the dispose export in `receiveDispose` (line 1249). Every other export the worklet calls runs in a message handler, and none of those calls is contained.

- **`single` mode** (`port.onmessage`, lines 290-311): `receive` (line 1139) calls the exports for source submit, the seeks, the seek report, plan latency, the control command (`readControlCommand`, line 1345), the meter lease and the session map and status reads. After it the handler runs the service step, the watermark read, the event drain and `syncMeterViews`, each of which calls an export too. The handler has a `try`/`finally` and no `catch`, so a throw from any of these escapes it.
- **`worker` mode**: `receiveWorkerMode` (line 560) calls `receive` for the meter lease and the render-allocation count, then `syncWorkerMeterViews`. `receiveControlPlane` (line 596) calls `meter_header_ptr` and `control_allocation_count`. Neither has a `catch`.

What a throw leaves, in `single` mode (verified by reading `hosts/host-web/src/ffi.rs` and by the test below):
1. **The request is never answered.** The page's request waits without bound. The main-realm host bounds only the dispose (`DISPOSE_TIMEOUT_MS`).
2. **The phase stays `live`, and the trap is not recorded.** In the shipped module a trap inside `with_host_mut` (for example `miso_engine_web_v1_control_command`, ffi.rs line 1280) leaves `LIVE_HOST` mutably borrowed. The next `process()` therefore calls the render export on the trapped instance. Its `try_borrow_mut` fails and it returns `RESULT_INVALID_ARGUMENT`, so the worklet fails sticky with 1, not 255, and records no trap.
3. **The dispose never completes**, the #2256 wedge reached by another route. `receiveDispose` sees no trap and calls the dispose export. That export's `try_borrow_mut` fails on every call and returns 1 (ffi.rs ~1596). The single-mode host retries a dispose that fails with 1, so the node is never disconnected.

## Scope
- **One containment, at the handler boundary.** Every entry the worklet's ports call (`port.onmessage` in `single` mode, `receiveWorkerMode`, `receiveControlPlane`) catches a throw and hands it to one method, `trap`. `trap` records the failure as `failed{code: 255, trapped}`, through the existing #2256 `fail` writer, and answers with the typed `miso.error.v1` result 255:
  - **Not yet answered** (the throw came from inside `receive`): the answer goes to the in-flight request. It carries that request's planes when it has any, as `receive`'s refusals do, so a source's planes come back.
  - **Already answered** (the throw came from the service step, the watermark read, the drain or the meter-view sync): the answer uses the request ID the step's own errors already use. That is the last request in `single` mode, 0 for a service tick, and 0 in `worker` mode.
  - **From the control Worker's port**: the failure is posted to the page once, as `reportRenderFailure` already does for a render failure.
- **After a trap, no export runs, by the #2256 rules that already exist.** The phase is `failed`, so `receive` refuses every later request with the sticky 255 and the handler's steps are skipped (each requires `live`). `process()` renders silence without calling the render export. The dispose is answered 255 without calling the export, and the instance ends; the host tears down on that answer (#2256).
- **Every throw that escapes a handler counts as a trap.** The handler cannot tell a trap from other throws: V8 reports a Wasm stack overflow as a `RangeError`, and a JavaScript exception thrown through Wasm frames is not a `WebAssembly.RuntimeError`. The handler's own code throws only on a module fault (an out-of-range pointer an export returned, or a status record that fails its check), after which the worklet cannot vouch for the instance's memory either. Treating such a throw as a trap costs only the dispose export's free, and in `single` mode the memory goes with the node.
- No change to `process()`, no Rust change, no change to the main-realm host or the control Worker.

## Non-goals
- A throw in `process()` from `meter_poll` or (in `worker` mode) `meter_header_ptr`. It reaches the user agent's `processorerror`, which the host already turns into a failure.
- `worker` mode's control-plane handler still calls `meter_header_ptr` (meter view) and `control_allocation_count` (render status) on an instance that is already trapped. With this change a throw there is contained. Refusing those calls outright is a separate hardening.
- The SDK's PCM feed prelude, which calls `source_submit` from its own drain in `process()`.
- The main-realm host (#2279) and the control Worker (#2282).

## Gates
- A processor test (`scripts/test-web-audioworklet.mjs`). In `single` mode, a throw from the control command, source submit, session seek and service-step exports, each through the real `port.onmessage`:
  - does not escape the handler;
  - is answered `miso.error.v1` result 255, to the in-flight request (with its planes, for the source) or, for the service step on a tick, to request 0;
  - leaves the render export uncalled on the next block;
  - makes the dispose answer 255 without calling the dispose export, after which the instance ends.

  In `worker` mode, a throw from the meter lease export on the page's port is answered 255 to its request, and the release then answers token `0n` without calling `host_release`. Red on base.
- `bash scripts/test-web-audioworklet.sh` passes, as does `bash scripts/check-workspace-policy.sh`.
- The SDK evals that need no artifact pass with the same set as base.

## Evidence
(attempt record below)
