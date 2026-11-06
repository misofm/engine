# Web host: a dispose after a render trap never completes in single mode

Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The single-mode AudioWorklet's lifecycle is spread over about eleven fields (`ready`, `disposed`, `stickyResult`, `stickyPosted`, `initializationErrorPosted`, `handle`, `handleDisposed`, `releaseAnswered`, `releasedToken`, `workerMode`, `controlPort`; `hosts/host-web/web/miso-engine-v1-audio-worklet.js`). The release browser module is `panic = "abort"` (`Cargo.toml` `[profile.release]`), so a panic inside `miso_engine_web_v1_render` traps the instance without unwinding: the `RefMut` that `with_render_mut` holds on `LIVE_HOST` (`hosts/host-web/src/ffi.rs`) is never dropped and `RENDER_LOCKED` stays set. `process()` catches the throw as sticky `RESULT_INTERNAL`, the same as a returned 255, and records nothing else. The later `miso.dispose.v1` calls `miso_engine_web_v1_dispose`, whose `try_borrow_mut` on `LIVE_HOST` fails forever (result 1). The single-mode host (`miso-engine-v1-audio-worklet-host.js` `dispose()`) treats a non-OK, non-teardown result as retryable: it drops `#disposePromise` and restarts its service tick. Dispose never completes and the node is never disconnected. Worker mode ends in teardown in the same situation.

## Scope
- The worklet's lifecycle is one explicit phase, `booting | live | failed{code, posted, trapped} | released | disposed`, with one writer (`enter`). The names `ready`, `disposed` and `stickyResult` stay as the phase's published view: the SDK's PCM feed prelude subclasses the processor and reads them (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`).
- A render trap is recorded (`failed{trapped}`). A trapped instance runs no export again: its dispose is answered `RESULT_INTERNAL` without calling the export, and it ends `disposed`; a dispose export that traps is treated the same. In worker mode the release of a trapped instance answers with no token, as it already did when the release threw.
- The single-mode host treats a dispose that fails with 255 as final: it tears down (disconnects the node, restarts no tick) and rejects with the engine's error, as worker mode does on the control Worker's 255. Any other failed single-mode dispose may still be retried.
- No change to the render path's allocation behaviour, and no Rust change: the trap is visible to JavaScript as the throw, and nothing can be repaired inside a trapped instance.

## Non-goals
- Containing a trap in an export the message handler calls (control command, seek, source submit). It still escapes the handler; a separate issue if wanted.
- The `.d.ts`, the control Worker, the stem store and the SDK copy.

## Gates
- A processor test: after a render trap, and with a dispose export that traps, the dispose is answered 255, no export is called on the trapped instance, and the instance ends. Red on today's code.
- A host test, the single-mode parity row of worker mode's `dispose255`: a dispose answered 255 rejects with it and tears down; a later dispose returns the same rejection and sends nothing. Red on today's code (retried, node never disconnected).
- `bash scripts/test-web-audioworklet.sh` (the hermetic host and worklet suite, the source policies and their mutations) passes; `scripts/check-workspace-policy.sh` passes.
- The SDK evals that read the shipped worklet source and need no artifact pass unchanged.

## Evidence
### Attempt 1 (Terra, 2026-11-06)
Base `8f46ab035`; implementation `4bf57a9ea`.

**Mechanism, confirmed by reading.** `miso_engine_web_v1_render` runs `render_locked(|| with_render_mut(..))`; `with_render_mut` holds `LIVE_HOST.try_borrow_mut()` across the render, and `render_locked` clears its flag only on return (no guard, by design under `panic = "abort"`). A trap therefore leaves the `RefCell` borrowed and `RENDER_LOCKED` set; `miso_engine_web_v1_dispose` then refuses at `LIVE_HOST.try_borrow_mut()` with `RESULT_INVALID_ARGUMENT` on every call. Nothing on the JavaScript side told a trap from a returned 255: `process()` mapped the throw to `RESULT_INTERNAL` and kept no record, and the single-mode dispose called the export regardless. Worker mode already contained it: the worklet's release catches, the Worker disposes without the half, and the page tears down on any dispose outcome.

**Phase model and transitions** (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, `PHASE_*`, writer `enter`):
- `booting` -> `live`: `initialize` / `initializeWorker` posted ready.
- `booting` -> `disposed` (single) / `released` (worker): `failInitialization`, once.
- `booting | live | failed` -> `failed{code, posted, trapped}`: `fail` (from `process()` on a render refusal, a trap or a quantum mismatch; from a meter-set fault; from `sticky`). The latest code is kept; a trap is permanent.
- `live | failed` -> `disposed` (single): `receiveDispose` on OK, or at once when trapped (no export called; answered 255), or when the dispose export itself traps (answered 255). Any other refusal keeps the phase; the host may retry.
- `live | failed` -> `released` (worker): `releaseRenderHalf`; a trapped instance answers with token `0n` without calling `host_release`.
- `released` and `disposed` are final: `process()` returns `false`, messages are ignored.

Fields folded away: `handleDisposed` (equal to `handle === 0` at every site), `initializationErrorPosted` (`failInitialization` is now idempotent on the phase), `stickyPosted` (now `failurePosted`). `ready`, `disposed` and `stickyResult` remain only as the phase's published view, written by `enter` and read by nothing in the worklet: the SDK's PCM feed prelude subclasses the processor and reads them, and its eval sets them on an unconstructed instance, so they cannot become getters without an SDK change. Phases are interned string constants, so `process()` compares, never allocates; `enter`, `ended` and `fail` sit inside `PROCESS_POLICY_BEGIN/END` because `process()` calls them, and the source policy passes over them.

**Host decision** (`miso-engine-v1-audio-worklet-host.js` `#disposeSingleMode`): a single-mode dispose that rejects with result 255 tears down, as a teardown does. 255 reaches a single-mode dispose only from a trapped worklet (the export itself never returns it), from a port that cannot take the request, or from a host failure while the dispose is in flight (a malformed worklet message, `processorerror`); in each, no retry can be answered. Result 3 (and any other code) is still retried; the existing tick test holds that.

**Red on today's code** (`node scripts/test-web-audioworklet.mjs`, tests added before the fix):
- Host row: `AssertionError [ERR_ASSERTION]: a 255 dispose disconnects the node ... actual: 0, expected: 1` (the dispose was dropped for retry). Red again with only the host change reverted (`MISO_ENGINE_WEB_HOST_TEST_MODULE=<base host>`), same assertion.
- Processor row, render trap: `actual: { tag: 'miso.ack.v1', requestId: 1, result: 1 }, expected: { ... result: 255 }` (the export was called; its refusal is what the host retried).
- Processor row, dispose trap: `Error: synthetic wasm trap from the dispose export` escapes the message handler (no answer; the host would wait out its 5 s bound).

**Green** at `4bf57a9ea`:
- `bash scripts/test-web-audioworklet.sh`: exit 0 (hermetic suite, the safe-integer host variants, source/clock/timer/capability policies and their mutations, call-graph self-test, parameter-metadata and session-map gates, UTF-8 mutation).
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- SDK evals without an artifact (`cd sdk && node --test 'test/*-evals.mjs'`): 710 tests, 340 pass, 22 fail, the identical failing set on base and on this change, every failure the missing artifact (`MISO_ENGINE_SDK_ARTIFACTS_HEX must encode a directory built by scripts/build-web-audioworklet.sh`, and one `compileCount` TypeError in `boot-evals.mjs`'s after-hook, downstream of its failed before-hook). `the prelude over the real engine worklet class` passes.
- Not run here: the artifact-backed SDK evals and the Playwright browser legs (the pinned browser nightly is not installed, so no artifact can be built). No Rust changed, so `cargo test -p host-web`, clippy and fmt were not needed.

**Test value.**
- Processor test (`scripts/test-web-audioworklet.mjs`, "Issue #2256: a trap leaves the instance's memory..."): red if the worklet calls an export on a trapped instance or answers a dispose after a trap with anything but a final 255, or if a trap inside the dispose export escapes the handler unanswered; no existing test disposes after a trap.
- Host row (the S8 single-mode teardown block, "the single-mode parity row of worker mode's `dispose255`"): red if a single-mode dispose answered 255 is dropped for retry instead of tearing down; the existing rows cover only a retryable 3 and an unanswered dispose.
- The pre-existing `processor.ready = false` fixture now reads `processor.enter("booting")`: the same claim (pre-ready silence), stated through the phase the worklet reads.
