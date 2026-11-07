# Web host: a trap in a worklet message-handler export escapes the handler, and the dispose after it never completes

Tracker: #2258, the follow-up #2256 recorded in its non-goals ("Containing a trap in an export the message handler calls (control command, seek, source submit). It still escapes the handler"). Base `9c1ef3ea7`.

## Problem
The release browser module is `panic = "abort"` (`Cargo.toml` `[profile.release]`), so a Rust panic in any export, including an allocation failure inside a control command's rebuild, traps the instance and surfaces in JavaScript as a throw. #2256 contained that throw at two call sites in `hosts/host-web/web/miso-engine-v1-audio-worklet.js`: the render export in `process()` (line 1735) and the dispose export in `receiveDispose` (line 1249). The release (`releaseRenderHalf`) and a refused construction (`failInitialization`) contain theirs too. Every other export call that a message handler reaches is uncontained. The worklet scope holds four message-handler entries that reach an engine export, across the engine worklet and the SDK's PCM feed prelude (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`), which shares its scope and wraps the engine class:

1. **The `single`-mode page port** (`port.onmessage`, lines 290-311). `receive` (line 1139) calls the exports for source submit, the seeks (the prelude's `receiveSeekAt` override runs inside it), the seek report, plan latency, the control command (`readControlCommand`, line 1345), the meter lease and the session map, status and allocation reads. After it, the handler runs the service step, the watermark read, the event drain and `syncMeterViews`, each of which calls an export too. The handler has a `try`/`finally` and no `catch`.
2. **The `worker`-mode page port** (`receiveWorkerMode`, line 560): `receive` for the meter lease and the render-allocation count, then `syncWorkerMeterViews` (`meter_header_ptr`). No `catch`.
3. **The control Worker's port** (`receiveControlPlane`, line 596): `meter_header_ptr` and `control_allocation_count`. Its release export is contained. No `catch`.
4. **The prelude's attach port** (`MisoSabFeedAttachProcessor`'s `port.onmessage`, prelude line 839): its `prepare-seek` calls `prepareSharedSeeks`, which reaches `source_prepare_seek` (inside `with_host_mut`). No `catch`. Its `attach`, `detach` and `detach-source` reach no export: `attachSharedRings` reads only `exports.memory.buffer`.

No other code path reaches an export from a message handler. The remaining export calls run in the constructor (contained by `failInitialization`) or in `process()` (see the non-goals).

What a throw leaves, in `single` mode (verified by reading `hosts/host-web/src/ffi.rs` and by the test below):
1. **The request is never answered.** The page's request waits without bound. The main-realm host bounds only the dispose (`DISPOSE_TIMEOUT_MS`).
2. **The phase stays `live`, and the trap is not recorded.** In the shipped module a trap inside `with_host_mut` (for example `miso_engine_web_v1_control_command`, ffi.rs line 1280) leaves `LIVE_HOST` mutably borrowed. The next `process()` therefore calls the render export on the trapped instance. Its `try_borrow_mut` fails and it returns `RESULT_INVALID_ARGUMENT`, so the worklet fails sticky with 1, not 255, and records no trap.
3. **The dispose never completes**, the #2256 wedge reached by another route. `receiveDispose` sees no trap and calls the dispose export. That export's `try_borrow_mut` fails on every call and returns 1 (ffi.rs ~1596). The single-mode host retries a dispose that fails with 1, so the node is never disconnected.

## Scope
- **One containment, at the handler boundary.** Each of the four entries catches a throw and hands it to one method, the engine's `trap`. `trap` records the failure as `failed{code: 255, trapped}`, through the existing #2256 `fail` writer, and answers with the typed `miso.error.v1` result 255:
  - **Not yet answered** (the throw came from inside `receive`): the answer goes to the in-flight request. It carries that request's planes when it has any, as `receive`'s refusals do, so a source's planes come back. One exception: `receiveControl` posts its reply and then re-cuts the views. A throw from that re-cut gives an answered request a second answer, and the page fails on that too.
  - **Already answered** (the throw came from the service step, the watermark read, the drain or the meter-view sync): the answer carries no planes, because a source's went back with its acknowledgement, and it uses the steps' request ID. That ID is the last request in `single` mode, 0 for a service tick, and 0 in `worker` mode. `syncMeterViews` now takes the same ID, so a fault it finds on a tick is request 0's, as the other steps' are. On base it reported to `lastRequestId`.
  - **From the control Worker's port or the prelude's attach port**: neither carries page requests, so the failure is posted to the page as request 0. The prelude also answers its `prepare-seek` with `{ kind: "refused", result: 255 }`, which the feed rejects as `prepareRefused`.
- **After a trap, no export runs, by the #2256 rules that already exist.** The phase is `failed`, so `receive` refuses every later request with the sticky 255 and the handler's steps are skipped (each requires `live`). `process()` renders silence without calling the render export. The dispose is answered 255 without calling the export, and the instance ends; the host tears down on that answer (#2256).
- **Every throw that escapes a handler counts as a trap.** The handler cannot tell a trap from other throws: V8 reports a Wasm stack overflow as a `RangeError`, and a JavaScript exception thrown through Wasm frames is not a `WebAssembly.RuntimeError`. The handler's own code throws only on a module fault (an out-of-range pointer an export returned, or a status record that fails its check), after which the worklet cannot vouch for the instance's memory either. Treating such a throw as a trap costs only the dispose export's free, and in `single` mode the memory goes with the node.
- No change to `process()`, no Rust change, no change to the main-realm host or the control Worker. The prelude changes only in its attach port's `prepare-seek` branch.

## Non-goals
- **A trap in `process()` outside the render call: a successor issue, to be filed in #2258.** It has the same dispose wedge as this issue:
  - A `meter_poll` trap (or, in `worker` mode, a `meter_header_ptr` trap) escapes `process()`. The user agent then fires `processorerror` and the host fails, but the worklet never records `failureTrapped`. A single-mode dispose then calls the dispose export the trap left refusing, and is answered 1, which the host retries.
  - The prelude's drain (`process()`, around prelude line 430) swallows a `source_submit` or `source_seek` trap into `CONTROL_ERRORS` and renders on. Nothing is recorded, and the next block and the dispose call exports on the trapped instance.

  Containing these changes `process()`, which is the render path, so it belongs to its own issue.
- `worker` mode's control-plane handler still calls `meter_header_ptr` (meter view) and `control_allocation_count` (render status) on an instance that is already trapped. With this change a throw there is contained. Refusing those calls outright is a separate hardening.
- The main-realm host (#2279) and the control Worker (#2282).

## Gates
- A processor test (`scripts/test-web-audioworklet.mjs`), with one row per catch and per request-ID branch (the guard table below). In `single` mode, each row throws from one export through the real `port.onmessage`: control command and source submit (inside `receive`), the service step after a source acknowledged OK, and the service step on a tick after an answered request. Each throw:
  - does not escape the handler;
  - is answered `miso.error.v1` result 255, to the right request with the right planes;
  - leaves the render export uncalled on the next block;
  - makes the dispose answer 255 without calling the dispose export, after which the instance ends.

  A meter-view fault on a tick is request 0's. In `worker` mode, three rows: a throw inside `receive` on the page's port, one after the answer, and one on the control Worker's port after a page request. After each, render is not called and the release answers token `0n` without calling `host_release`. Red on base.
- A prelude test (`sdk/test/browser-pcm-evals.mjs`, over the real engine class). A throw from `source_prepare_seek` under `prepare-seek` stays in the handler, answers `seek-prepared` refused 255 and posts request 0 to the page, even after a page request; the dispose after it is final and calls no export.
- `bash scripts/test-web-audioworklet.sh` passes, as does `bash scripts/check-workspace-policy.sh`.
- The SDK evals that need no artifact pass with the same set as base, the PCM evals (`sdk/test/browser-pcm-evals.mjs`) included.

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

**Verdict on attempt 1 (Sol): FAIL, on two MAJORs.** The reviewer accepted the containment model, the rule that every throw is a trap, the request IDs and the planes handling, and found a merged tree of main, #2291 and #2279 green.
1. A handler entry was missed. The prelude attach port's `prepare-seek` (prelude line 839) reaches `source_prepare_seek` under `with_host_mut` with no `try`, which is the same dispose wedge.
2. Four guards survived mutation: the worker `answered` branch (M7, M9), the control port's `trap(0)` (M12), and the single-mode after-answer branch passing planes (M16). M16 is a real defect: re-sent transferred planes make the post throw a `DataCloneError` out of the handler.

Minors: the `process()` non-goal understated the consequence; the `session_seek` row was redundant; the single-mode catch comment was inexact about `receiveControl`; and `syncMeterViews` reported a tick's fault to `lastRequestId`.

### Attempt 2 (Terra, 2026-11-07)
`origin/main` was merged at `165a39635`, and attempt 2 is the commit after it.

**Changes.**
- The prelude's `prepare-seek` catches a throw, calls `registry.engine.trap(0)` and replies `{ kind: "refused", result: 255 }`. The SDK ships the engine worklet with its prelude (`sdk/src/assets.ts`, provenance), so `trap` is always present.
- `syncMeterViews(requestId)` takes the steps' request ID.
- The single-mode catch comment and the `trap` doc now name all four entries and the `receiveControl` exception.
- Tests:
  - the `session_seek` row is deleted;
  - added the single-mode rows "service after a source" and "meter-view fault on a tick";
  - the worker rows are now three (page port inside `receive`, page port after the answer, Worker port after a page request);
  - added one prelude row in `sdk/test/browser-pcm-evals.mjs`.

**Handler entries that reach an engine export.** Every one is contained and has a row:

| Entry | Exports it reaches | Containment | Rows |
|---|---|---|---|
| `single` page port (`port.onmessage`) | every `receive` export (source, seeks, seek report, plan latency, control command, meter lease, session map, status, allocation counts), then service, watermark read, event drain, `meter_header_ptr`; dispose | `catch` -> `trap`; dispose: `receiveDispose`'s own catch (#2256) | the four `single` rows, the tick meter-view row; #2256's dispose rows |
| `worker` page port (`receiveWorkerMode`) | `meter_lease`, `render_allocation_count`, then `meter_header_ptr` | `catch` -> `trap` | "page port, inside receive", "page port, after the answer" |
| control Worker's port (`receiveControlPlane`) | `meter_header_ptr`, `control_allocation_count`; `host_release` | `catch` -> `trap(0)`; release: `releaseRenderHalf`'s own catch (#2256) | "Worker port" |
| prelude attach port, `prepare-seek` | `source_prepare_seek` | `catch` -> `registry.engine.trap(0)`, refused 255 | the prelude #2291 row |
| prelude attach port, `attach` / `detach` / `detach-source` | none (`exports.memory.buffer` is a property read) | n/a | n/a |

Export callers that are not message handlers: the constructor (`initialize`, contained by `failInitialization`) and `process()` (render contained; `meter_poll`, worker `meter_header_ptr` and the prelude's drain are the successor in the non-goals).

**Guard table.** Each catch and request-ID branch, the mutation that removes it, and the row that goes red. M1-M17 are the reviewer's mutations (`review-2291/mutate.py`), adapted to this worktree; M18 and P1-P4 are new. All were run against attempt 2. Worklet mutations run the hermetic suite over a mutated copy; prelude mutations rewrite the prelude in place, run its #2291 row, and restore it.

| Mutation | Guard | Result: first failing assertion |
|---|---|---|
| M1 single catch rethrows | single catch | RED: `a control_command trap stays inside the handler` |
| M2 always `trap(lastRequestId, planes)` | single after-answer branch | RED: `a service after a source trap stays inside the handler` (`DataCloneError`) |
| M3 always `trap(requestId)` | single unanswered branch | RED: `a control_command trap: the typed answer` |
| M4 a tick's ID is `lastRequestId` | tick -> request 0 | RED: `a service on a tick trap: the typed answer` |
| M5 `trap` records no trap | `trap`'s `fail(.., true)` | RED: `a control_command trap: the dispose is final` |
| M6 `trap` drops planes | `trap` passes planes | RED: `a source_submit trap: the typed answer` |
| M7 worker `trap(lastRequestId)` | worker after-answer -> 0 | RED: `page port, after the answer: the typed answer` |
| M8 worker `trap(0)` | worker unanswered -> its ID | RED: `page port, inside receive: the typed answer` |
| M9 worker `answered = true` deleted | worker `answered` | RED: `page port, after the answer: the typed answer` |
| M10 worker page catch rethrows | worker page catch | RED: `page port, inside receive: the trap stays inside the handler` |
| M11 control-port catch rethrows | control-port catch | RED: `Worker port: the trap stays inside the handler` |
| M12 control port `trap(lastRequestId)` | control port -> 0 | RED: `Worker port: the typed answer` |
| M13 control port: sticky, not trapped | control port's trap record | RED: `Worker port: the release answers no token` |
| M14 worker page: sticky, not trapped | worker page's trap record | RED: `page port, inside receive: the release answers no token` |
| M15 `syncMeterViews` also in the catch | (the old `finally`) | survives, as an equivalent mutant: `syncMeterViews` returns unless `live`, and a trap leaves the phase `failed` |
| M16 after-answer branch passes planes | after-answer: no planes | RED: `a service after a source trap stays inside the handler` (`DataCloneError`) |
| M17 `receive`'s refusals drop planes | (refactor of an existing line) | survives: the planes a malformed or failed-phase request gets back were untested on base too, and the refactor moved the expression unchanged; the trap path's planes are M6's. A follow-up, not this issue's claim |
| M18 `syncMeterViews()` without the steps' ID | meter-view fault on a tick -> 0 | RED: `a meter-view fault on a tick is request 0's` |
| P1 prelude catch rethrows | prelude catch | RED: `Got unwanted exception` |
| P2 prelude records no trap | prelude's `trap` call | RED: `the page hears of the trap` |
| P3 prelude `trap(lastRequestId)` | prelude -> 0 | RED: `the page hears of the trap` |
| P4 prelude refusal not 255 | prelude refusal | RED: the `seek-prepared` reply |

**Green at attempt 2:**
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cd sdk && node --test test/browser-pcm-evals.mjs`: 51 tests, 48 pass, 3 fail. The 3 are the "actual Wasm" rows, which fail on base too, each on the missing artifact.
- `cd sdk && node --test 'test/*-evals.mjs'`: 711 tests, 341 pass, 22 fail. The failing set is identical to attempt 1's base (diffed by name).
- Not run: the artifact-backed SDK evals and the browser legs (no artifact here). No Rust or render-path change.

**Test value (attempt 2's new and rewritten rows).**
- Service after a source: red if a throw after the answer re-sends transferred planes (a `DataCloneError` escapes the handler) or loses the request's ID.
- The tick meter-view row: red if the meter-view sync reports a tick's fault to the last answered request.
- The worker rows "after the answer" and "Worker port": red if the worker `answered` branch or the control port's request 0 is lost; the reviewer's M7, M9 and M12 survived attempt 1.
- The prelude row: red if `prepare-seek` lets an export's throw escape, records no trap on the real engine class (the dispose then calls the export the trap left refusing), reports it to a page request, or refuses with anything but 255.
- Deleted: the `session_seek` row. It defended the same unanswered branch as the control command row, so it caught nothing that row does not.
