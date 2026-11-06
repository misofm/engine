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
