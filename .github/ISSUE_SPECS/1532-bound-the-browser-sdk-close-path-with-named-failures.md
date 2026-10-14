# Bound the browser SDK close path with named failures

Stream H SDK slice, from the stream B batch-3 B2 diagnosis (#1345, "B2 diagnosis and fix",
finding (e)). Code anchors verified on `codex/d15-stream-b3` at `ebe706208`. Verify them again on
`main` after #1345 merges, before the first edit.

## Product outcome

`engine.close()` of the browser SDK always settles in bounded time. Each wait inside it that has
no bound today is bounded at 10 s. A wait that does not settle rejects `close()` with a typed
error that names the step, and the `AudioContext` close is still attempted. The same bound applies to the context closes on the
boot failure paths. Today one worklet that does not answer the dispose request, or one
`AudioContext.close()` that does not settle, holds the caller and the context for ever.

## Context

- **The close path.** `sdk/src/browser/engine.ts:1306-1325`. `close()` sets `closed`, closes the
  measurement feeds and invalidates the observation subscriptions (sync), then in a memoized
  `closePromise` (`:1310`): awaits the track-response close (`:1312-1314`) and the spectrum close
  (`:1315-1317`), each in a `try/catch` that drops its error ("host close remains
  authoritative"); awaits `host.dispose()` (`:1318`); and in `finally` awaits `context.close()`
  (`:1319-1322`). No await has a bound.
- **What the two sub-close waits wait for.** The track-response step waits for
  `trackResponsePromise` (`:1116-1124`): `BrowserTrackResponse.create`
  (`sdk/src/browser/response.ts:242-271`) awaits its Worker's init with a deadline of
  `responseLimits.requestDeadlineMs ?? 5_000` (`:247`), and `engine.ts` passes no
  `responseLimits`, so that wait always ends within 5 s; its `close()` is synchronous (`:303-305`).
  The spectrum step waits for `spectrumPromise` (`engine.ts:652-675`): `BrowserSpectrum.create`
  (`response.ts:451-480`) awaits its init with `spectrumLimits.requestDeadlineMs ?? 5_000`
  (`:456`), and the caller's query sets that value, up to 2,147,483,647 ms
  (`engine.ts:655-661`). So the spectrum step can hold `close()` for as long as the caller's
  deadline; a client that resolves after `closed` is set is closed at once (`:663-665`).
- **The boot failure paths.** `await candidate.close()` after a wrong sample rate (`:545`),
  `await context.close()` after a quantum refusal (`:563`) and after a worklet boot failure
  (`:1327-1329`). No bound.
- **The public doc.** `close()` in the `BrowserEngine` interface, `:223-224`: "Dispose the worklet
  host, then close its context. Safe to call more than once."
- **What dispose waits for.** `MisoAudioWorkletHost.dispose()`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1622-1625`) sends
  `miso.dispose.v1` and awaits the worklet's reply through `#request` (`:1218-1260`), which has no
  deadline. The reply is built on the audio thread.
- **Why now.** The B2 diagnosis saw, on WebKit 26.5 with every CPU busy, a running worklet
  `AudioContext` whose clock never moved, and a script `suspend()` then click `resume()` that
  never settled and stopped the page's own 10 s timer (`/tmp/claude-1002/b2diag/drv4.out`,
  `drv5.out`). In those runs the 17 SDK closes took 0 to 8 ms, so the close path was not the
  cause of B2, and the diagnosis did not bound it because `engine.ts` was outside its exception.
  The risk stays: a caller that awaits `close()` on such a context has no bound.
- **The bound already in use.** The qualification harness bounds its own `suspend()` before a
  read at 10 s (`hosts/host-web/qualification/sdk-response-entry.ts:122-123`, `:137-150`), and
  every audio step of the unsubscribe gate at 10 s
  (`hosts/host-web/qualification/qualification.js:507`, `bounded`, `:513`). The SDK has a pattern
  for a bounded await that still observes a late rejection: `awaitSpectrum`
  (`engine.ts:381-405`, `void promise.catch(() => undefined)` at `:390`).
- **Existing tests.** `sdk/test/browser-evals.mjs:405-428` (close disposes the worklet before its
  context and is idempotent) and `:430-453` (close still releases the context when dispose
  rejects; a rejected close is not replayed). Both use a scripted `createContext` and
  `createHost`. CI runs them through `scripts/check-sdk-headless.sh:91`
  (`node --test 'test/*-evals.mjs'`).
- **The error code set.** `MisoEngineError` (`sdk/src/core/errors.ts:72-117`) takes `phase`
  (`"lifecycle"` exists), `code: MisoErrorCode` and `result: number`. `MisoErrorCode`
  (`errors.ts:14`) is `ResultCodeName | BootResultAliasName`, and both names are generated from
  the ABI layout (`sdk/src/generated/abi.ts:2443`). The result codes are `ok`,
  `invalidArgument`, `abiMismatch`, `wrongState`, `bufferTooSmall`, `refusedBudget`,
  `backpressure`, `unsupported`, `renderRejected`, `reprepareRequired` and `internal`
  (`sdk/assets/miso-engine-v1-abi-layout.json`). None of them is a timeout, and none names a
  close step. So the set cannot carry the failure, and this slice adds SDK-side codes (D2). No
  SDK code switches over `MisoErrorCode` exhaustively: the only comparisons are
  `code === "wrongState"` in `sdk/src/core/observation-subscriptions.ts` (`:392`, `:1281`,
  `:1468`, `:1479`, `:1756`), so the new members break no switch.

## Decisions frozen for this slice

- **D1. One bound, per step.** A module constant `CLOSE_STEP_TIMEOUT_MS = 10_000`. The spectrum
  step, `host.dispose()`, `context.close()` and each boot-path context close wait at most that
  long. It is not an option and it is not `requestDeadlineMs`. The worst case of one `close()` is
  5 s (track response) + 3 x 10 s = 35 s.
- **D1a. The track-response step is not bounded here.** Its only wait is already bounded at 5 s
  by `BrowserTrackResponse.create` (Context), and its close is synchronous, so a 10 s bound there
  is one no test can reach. If a later change lets that deadline exceed 10 s, that change bounds
  the step with this helper.
- **D2. The named failure.** The existing `MisoEngineError` class, `phase: "lifecycle"`, with
  one timeout code for each step. `errors.ts` adds
  `export type SdkTimeoutCode = "spectrumCloseTimeout" | "hostDisposeTimeout" |
  "contextCloseTimeout"` and widens `MisoErrorCode` to
  `ResultCodeName | BootResultAliasName | SdkTimeoutCode`. The doc of these codes says that they
  come from the SDK, not from the engine, and that they carry no engine result. A step that does
  not settle in time gives
  `new MisoEngineError("close: <step> did not settle within 10000 ms", { phase: "lifecycle",
  code: <its code> })`, with no `result`. The steps are:
  - `spectrum close` with code `spectrumCloseTimeout`;
  - `worklet host dispose` with code `hostDisposeTimeout`;
  - `AudioContext close` with code `contextCloseTimeout` (this code also covers the boot-path
    closes).

  **`result` becomes optional (root ruling, 2026-10-12).** `MisoEngineError.result` and
  `MisoEngineErrorInit.result` change from `number` to `number | undefined`: the field holds an
  engine result code only when the engine returned one, and it is absent for an SDK-side code.
  A sentinel such as 255 (`internal`) is refused, because it would claim an engine result that
  never happened. This changes a public SDK type before launch; the slice updates every reader
  of `MisoEngineError.result` (for example `sdk/src/enginectl.ts:280`) and its `.d.ts`, and the
  type evals cover both shapes. Callers match on `code`. Export `SdkTimeoutCode` next to every existing type export of
  `MisoErrorCode`. Today the only such export is `sdk/src/headless/index.ts:75`; the browser
  barrel exports no error types.
- **D3. Later steps still run.** After a timeout, the next steps run, each with its own bound.
  `context.close()` is always attempted, as today after a dispose rejection.
- **D4. Which error wins.** The track-response and spectrum closes keep dropping their own
  rejections (today's rule), but a spectrum timeout is kept. `close()` rejects with the first
  failure in step order (a timeout of any step, or a rejection of `dispose` or `context.close`).
  Later failures are dropped.
- **D5. No retry.** `closePromise` stays memoized: every later `close()` returns the same
  settled promise and calls nothing again.
- **D6. No leak.** The timer of a step is cleared when the step settles. A step that settles
  after its timeout is observed (`.catch(() => undefined)`), so it never becomes an unhandled
  rejection. A spectrum client that resolves after its step timed out is closed by the existing
  `closed` check (`:663-665`); keep that path.
- **D7. Boot paths.** At `:545` a close timeout rejects `createEngine` with D2's error (step
  `AudioContext close`). At `:563` and `:1327-1329` the original boot error propagates unchanged
  after at most 10 s; the bound only ends the wait.
- **D9. Test clock.** The tests use `node:test`'s `mock.timers` (D1's 10 s is not waited in real
  time). The SDK's other deadlines also use `setTimeout`, so a test that ticks past them sees
  them fire; each test says which deadlines it crosses.
- **D8. The doc.** `:223-224` says: each step waits at most 10 s; a step that does not settle
  rejects `close()` with a `lifecycle` error whose code names it (`spectrumCloseTimeout`,
  `hostDisposeTimeout`, `contextCloseTimeout`); the context close is still
  attempted; a settled close is not run again.

## Deliverables

1. One private helper in `engine.ts`: a bounded await that takes a step name and its code. Use it
   at the three bounded steps of `close()` and at the three boot-path closes (D1-D7). Add the
   three codes to `errors.ts` and the type exports (D2).
2. The doc of D8.
3. New tests in `sdk/test/browser-evals.mjs` (gates 1, 2, 4, 5 and 6), next to the two close
   tests, and one in `sdk/test/spectrum-browser-evals.mjs` (gate 3).
4. This spec's Attempt record: the mutation runs of gate 7 and the browser runs of gate 9.

## Authorized paths

- `sdk/src/browser/engine.ts` (the close path, the three boot-path closes, the doc at `:223-224`)
- `sdk/src/core/errors.ts` (`SdkTimeoutCode` and the widened `MisoErrorCode` only)
- `sdk/src/headless/index.ts` (the type export line `:75` only)
- `sdk/test/browser-evals.mjs`
- `sdk/test/spectrum-browser-evals.mjs` (one new test)
- This spec.
- Nothing else. Not `hosts/host-web/web/` (no deadline in the host's `#request`), not the
  qualification harness.

## Non-goals

- No deadline on other host requests, and no change to `MisoAudioWorkletHost`.
- No change to the observation, track-response or spectrum subscription closes themselves.
- No change to the headless engine's close.
- No retry and no new public option. The three codes are the only new public names.
- No fix of the WebKit clock stall (its own issue).

## Objective gates

The tests use the scripted `createContext`/`createHost` of `browser-evals.mjs:405-453` and
`node:test`'s `mock.timers.enable({ apis: ["setTimeout"] })`, enabled after `createEngine`
resolves (gate 6: before it is called; its scripted `scratchBoot` resolves at once). Each new test has `{ timeout: 5_000 }`, so a wait with no bound is a named red, not a
hang.

1. **Dispose does not settle.** `dispose` returns a promise that never settles. After
   `tick(9_999)` `close()` has not settled; after `tick(1)` it rejects with a `MisoEngineError`,
   `phase === "lifecycle"`, `code === "hostDisposeTimeout"`, message containing
   `worklet host dispose`;
   the context's `close` was called exactly once, after the timeout.
   *Test value:* turns red if the SDK awaits host disposal with no bound (a worklet that never
   answers `miso.dispose.v1`, as a stalled audio thread would, holds `close()` and the
   `AudioContext` for ever), if the bound is not 10 s, or if a dispose timeout skips the context
   close; no test today has a dispose or close that does not settle.
2. **Context close does not settle.** `dispose` resolves; the context's `close` never settles.
   `close()` rejects at 10 s with `code === "contextCloseTimeout"` and `AudioContext close` in the
   message.
   *Test value:* turns red if `context.close()` is awaited with no bound or its timeout is
   dropped, so a caller cannot learn that the context was not released.
3. **The spectrum step does not settle.** With the harness of
   `sdk/test/spectrum-browser-evals.mjs` (`:174-181`), start a `querySpectrum` with
   `spectrumLimits.requestDeadlineMs: 60_000` and a response Worker that never answers
   `spectrum-init`, then `close()`: it rejects at 10 s with `code === "spectrumCloseTimeout"` and `spectrum close`
   in the message; `dispose` and the
   context close each ran once; after `tick(60_000)` the Worker is terminated and no
   `unhandledRejection` reached `process`.
   *Test value:* turns red if a caller's long spectrum deadline holds `close()` (and the worklet
   and context behind it) for that deadline, or if a spectrum timeout is dropped like a spectrum
   rejection (D4).
4. **No retry.** After gate 1's rejection, a second `close()` returns a promise that rejects with
   the same error object and calls neither `dispose` nor the context's `close` again.
   *Test value:* turns red if a timed-out close clears `closePromise` and runs the steps again,
   which would stack a second unbounded dispose on a dead worklet.
5. **No leak.** (a) Gate 1's `dispose` rejects after the timeout: no `unhandledRejection` event
   reaches `process` during the test. (b) With real timers and steps that resolve at once,
   `process.getActiveResourcesInfo()` has no more `Timeout` entries after `close()` than before.
   *Test value:* (a) turns red if the late settle is not observed, which ends a Node caller's
   process and makes a browser page log a page error; (b) turns red if the timer is not cleared,
   which holds a Node process open 10 s after every close.
6. **Boot paths.** (a) `createContext` returns a context of the wrong rate whose `close` never
   settles, with `contextAttempts: 2`: `createEngine` rejects at 10 s with `AudioContext close`
   and the second candidate is not constructed. (b) A context with `renderQuantumSize: 256` (a
   quantum refusal) whose `close` never settles: `createEngine` rejects after 10 s with the same
   quantum error object that `assertQuantumMatch` threw. (c) `createHost` rejects and the
   context's `close` never settles: `createEngine` rejects after 10 s with the `createHost`
   error object.
   *Test value:* turns red if a boot failure path awaits a context close with no bound (the boot
   error never reaches the caller) or replaces the boot error with the close timeout (D7).
7. **Source mutations (PR evidence, in the Attempt record; not committed tests).** Apply one at a
   time, run `node --test test/browser-evals.mjs test/spectrum-browser-evals.mjs` in `sdk/`,
   record the red test, revert:
   - M1: no bound on `host.dispose()` -> gate 1 red (test timeout).
   - M2: `CLOSE_STEP_TIMEOUT_MS = 10_001` -> gate 1 red (not settled after 10 s).
   - M3: `9_999` -> gate 1 red (settled before 10 s).
   - M4: no `context.close()` after a dispose timeout -> gate 1 red.
   - M5: drop the late-settle observer -> gate 5(a) red.
   - M6: reset `closePromise` on rejection -> gate 4 red.
   - M7: drop `clearTimeout` -> gate 5(b) red.
   - M8: the spectrum timeout dropped like its rejection -> gate 3 red.
   - M9: no bound on the context close at `:563` -> gate 6(b) red (test timeout).
   - M10: at `:1327-1329`, throw the close timeout instead of the boot error -> gate 6(c) red.
   - M11: one shared code for all three steps -> gates 1 and 3 red (the codes do not name the
     step).
   A mutation that stays green is reported, not hidden.
8. **Existing tests.** The two close tests (`:405-453`) and `bash scripts/check-sdk-headless.sh`
   pass.
9. **Real browsers.** The CI command of `.github/workflows/qualification.yml:445` (it bundles
   `sdk/src` through `--sdk-root`) passes 3 unloaded runs in each of Chromium, Firefox and WebKit;
   every SDK instance of the qualification closes through this path.
10. `bash scripts/check-workspace-policy.sh` passes.

## Dependencies

- **#1345 on `main`.** It edits `sdk/src/browser/engine.ts` (`renderWait`, `:999-1032`) by
  amendment.
- **Before H #1332**, which moves disposal into a Worker: the bound and its tests then carry over.
  If #1332 lands first, this slice rebases onto its close path and keeps D1-D8.

## Placement

**Stream H**. `sdk/` belongs to stream H (`STREAMS.md:472`). Put this slice early in stream H's
queue: after B #1345 reaches `main` and before H #1332.

**Hot-file row (root, 2026-10-12).** `STREAMS.md` adds a row for `sdk/src/browser/engine.ts`:
"B #1345 (`renderWait`, by amendment) -> H #NNNN (the close bound) -> H #1332". This file has no
row today.

## Attempt record

Gates 1-6 and 8: implemented and green locally (`node --test test/browser-evals.mjs
test/spectrum-browser-evals.mjs`: 39 pass; `check-sdk-headless.sh` on a fresh artifact build: 393
pass; `check-sdk-types.sh`, `check-sdk-generated.sh`, `check-sdk-deletions.py`,
`check-workspace-policy.sh` pass). Gate 7 (mutations) moved to #TBD5 and was not run. Gate 9 (real
browsers) runs in CI; no local browser run (sprint mode).

## Decision record

- **Result type readers.** Making `MisoEngineError.result` `number | undefined` also widened
  `ValidationResult.result` (`headless/engine.ts`), `ErrorExtra.result` (`enginectl.ts`) and
  `ResponseWorkerError.result` (`response-worker.ts`); `deserializeError` (`response.ts`) no
  longer requires a `result` to rebuild a `MisoEngineError`. No reader switches on the value.
- **Type eval.** D2 asks the type evals to cover both shapes; that needed one new file,
  `sdk/test/error-types.ts` (compile-only, run by `check-sdk-types.sh`), outside the authorized
  path list.
- **Boot-path closes at the quantum refusal and the worklet boot failure** swallow any failure of
  the bounded close (a timeout or a rejection), so the boot error always reaches the caller (D7).
  Before, a rejecting `context.close()` there replaced the boot error.
- **Gate 3 clock.** The spectrum test ticks past close()'s 10 s bound and then the query's 60 s
  deadline; it crosses no other timer. Gates 1, 2, 4, 5(a) and 6 cross only the 10 s bound.

## Amendment (decision 17, root 2026-10-13)

Gate 7 (source mutations M1-M9) moves to #1605, *Post-sprint qualification of decision 15's heavy gates* (root, 2026-10-13, Q8). This slice closes on gates 1-6 and 8-10.
