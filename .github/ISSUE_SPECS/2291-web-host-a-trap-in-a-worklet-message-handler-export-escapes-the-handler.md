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

  In `worker` mode, a throw from the meter lease export on the page's port is answered 255 to its request, and a throw from `meter_header_ptr` under the control Worker's meter view is posted to the page as request 0. After either, render is not called and the release answers token `0n` without calling `host_release`. Red on base.
- `bash scripts/test-web-audioworklet.sh` passes, as does `bash scripts/check-workspace-policy.sh`.
- The SDK evals that need no artifact pass with the same set as base.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `9c1ef3ea7`. The spec and the red rows are `f1687a364`; the fix is the commit after it.

**Mechanism, confirmed by reading.** `miso_engine_web_v1_control_command` runs inside `with_host_mut`, which holds `LIVE_HOST.try_borrow_mut()` across the call. Under `panic = "abort"` a trap leaves that borrow held. The render export's `with_render_mut` and the dispose export's own `try_borrow_mut` then refuse with `RESULT_INVALID_ARGUMENT` on every call. Every other handler export either borrows `LIVE_HOST` the same way or reads behind it. The worklet's handler entries had no `catch`: the `single`-mode `port.onmessage` had only a `finally`, and `receiveWorkerMode` and the control port's handler had neither.

**Model.** The phase model #2256 introduced is unchanged: `failed{code, posted, trapped}`, written only by `fail`. This change adds the one method that reaches it from a handler, `trap(requestId, planes)`, and the three handler entries that call it:
- `single` mode `port.onmessage`: before `receive` has answered, `trap(lastRequestId, the message's planes)`; after, `trap(the step's request ID)`.
- `receiveWorkerMode`: `trap(lastRequestId)` before the answer, `trap(0)` after.
- The control port's `onmessage`: `trap(0)`.

The steps after `receive` stay in their order. The only change there is that the last `rebindMemoryViews`/`syncMeterViews` moved out of the `finally`, so after a trap they no longer run (`syncMeterViews` calls an export). The `returnedPlanes` expression that `receive` already used became a module function, so both sites share one definition.

**Red on base** (`node scripts/test-web-audioworklet.mjs` with the rows of `f1687a364` over the base worklet):
- `single` mode: `AssertionError [ERR_ASSERTION]: Got unwanted exception: a control_command trap stays inside the handler` / `Actual message: "synthetic wasm trap from the control_command export"`.
- A probe run (temporary, uncommitted) let the throw pass and logged each row's state. The pattern was the same for control command, source submit, session seek and service:
  - the last post was still `miso.ready.v1` (nothing answered);
  - after the trap, phase `live` and sticky 0;
  - after one block, phase `failed` and sticky **1**, with the render export called once;
  - the dispose was answered `{ tag: "miso.ack.v1", requestId: 2, result: 1 }`, with the dispose export called once. That is the retryable answer, the #2256 wedge reached by another route.
- `worker` mode, page port (single-mode rows skipped for the run): `Got unwanted exception: a worker-mode trap stays inside the handler` / `"synthetic wasm trap from the meter lease export"`.
- The control-port row and the service row's prior request were added with the fix, so `f1687a364` does not hold them. On base they sit behind the first red row; their own red is the matching mutation below.

**Mutations of the fix.** Each is red, and the first failing assertion is named:

| Mutation | First failing assertion |
|---|---|
| The `single`-mode catch rethrows | `a control_command trap stays inside the handler` |
| `trap` records no trap (`fail(.., false)`) | `a control_command trap: the dispose is final` (the dispose export ran) |
| An unanswered trap drops the planes | `a source_submit trap: the typed answer` |
| An answered trap uses `lastRequestId` instead of the step's ID | `a service trap: the typed answer` |
| The `worker` page-port catch rethrows | `a page-port trap stays inside the handler` |
| The control-port catch rethrows | `a worker-port trap stays inside the handler` |

They were run with `MISO_ENGINE_WEB_WORKLET_TEST_MODULE` pointing at mutated copies. The base worklet itself was run the same way and is red at the first row.

**Green at the fix:**
- `bash scripts/test-web-audioworklet.sh`: exit 0. That covers the hermetic suite with its safe-integer host variants, the source, clock, timer and capability policies with their mutations, the call-graph self-test, the parameter-metadata and session-map gates, and the UTF-8 mutation.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- SDK evals without an artifact (`cd sdk && node --test 'test/*-evals.mjs'`): 710 tests, 340 pass, 22 fail. The failing set is identical on base and on this change (diffed by name), and every failure is the missing artifact (`MISO_ENGINE_SDK_ARTIFACTS_HEX ...`). `the prelude over the real engine worklet class` passes.
- Not run: the artifact-backed SDK evals and the Playwright legs (no pinned browser nightly here, so no artifact can be built). No Rust, `process()` or render-path code changed, so cargo test, clippy, fmt, rustdoc, the realtime policy tool and a no-bit-moved comparison do not apply.

**Test value.**
- The `single`-mode rows (`scripts/test-web-audioworklet.mjs`, "Issue #2291: a trap in an export a message handler calls"). Red if a handler export's throw escapes `port.onmessage`, goes unanswered, comes back without a source's planes or with the wrong request ID, or leaves the trap unrecorded so that render and the dispose export run on the trapped instance. No existing test throws from a handler export other than the dispose export.
- The `worker`-mode rows ("Issue #2291, the `worker`-mode rows"). Red if a throw escapes the page-port or the control-port handler, which are entries separate from the `single`-mode one, or if the release then calls `host_release` on the trapped half.
- No existing test was made useless. #2256's dispose-trap row still defends `receiveDispose`'s own containment, which answers `miso.ack.v1` 255 and ends the instance; without it the boundary would answer `miso.error.v1` and keep the instance `failed`.
