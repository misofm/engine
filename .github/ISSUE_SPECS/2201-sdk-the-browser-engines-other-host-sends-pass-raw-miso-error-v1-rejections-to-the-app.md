# SDK: the browser engine's other host sends pass raw miso.error.v1 rejections to the app

GitHub #2201. Successor of #2193 (found by the #2193/#2189 attempt-1 verify, 2026-10-29).

## Root cause

#2190 mapped the protocol client's send and #2193 the live queue's send through
`hostRecordFailure`, but each send in `sdk/src/browser/engine.ts` called `host.control` on its own.
The spectrum frames (`sendSpectrumFrame`), the boot's capability probe (`confirmBootSpectrum`), the
observation endpoint, the loudness frame and the track-response requester did not map a host
rejection, so the app received the frozen record `{ tag: "miso.error.v1", requestId, result }`,
which is not an `Error`.

## Scope

Route every host send through one mapped path, so that an app receives only `MisoEngineError` (or
the SDK's typed refusals) from the browser engine. Keep each call's retry behaviour. Do not change
the PCM feed files (another branch owns them). Root ruling at the attempt-1 verify: the scope is
"an app receives only `MisoEngineError` from the browser engine", so the host calls that are not
`control()` and can reject with the raw record are in it too.

## Decision record

- D1 (attempts 1 and 2): one mapping for every host call of the browser engine, in
  `sdk/src/browser/live-controls.ts`. `hostCall(host, call)` maps a rejection through
  `hostControlFailure`, which is the #2193 mapping with its sticky result-6 rule: a mapped
  `backpressure` asks `status()`, and if the host has failed, it is raised as `wrongState`.
  `hostControl(host, frame, document?)` is `hostCall` around `host.control`. The calls that use
  it:
  - `control()` frames: the live queue and control-list send, the queue's revision probe
    (`openBrowserControlQueue`), the protocol client, the spectrum frames, the boot's capability
    probe, the observation endpoint, the loudness frame and the track-response requester;
  - `status()`: `engine.position()` and the observation render wait (its `mark` and its poll),
    which a host-local result 6 now reaches because it is retryable;
  - `sessionMap()`: `engine.liveControls()`;
  - `dispose()`: `engine.close()`;
  - `status()` in the PCM feed's anchored start (`pcm-feed.ts`, one `readAnchor` read for the
    `frameAt` and `alignTo` starts; the residual of the feed branch, after #2207 merged).
- D1b (folded after the attempt-2 PASS, root ruling on verify MINOR 1): `createEngine()` does not
  reject with the host's raw record on a boot refusal. In `default-host.ts`, the catch after
  `createMisoAudioWorkletHost` already raised `miso.worker-failed.v1` as `BrowserBootError`. It now
  also raises a raw `miso.error.v1` (the worklet's error ready reply, `webError(255)`, a context
  mismatch `webError(1)`, a module fetch failure `webError(RESULT_UNSUPPORTED)`) as
  `MisoEngineError`, phase `boot`. The code is named as for a boot (`resultName(result, "boot")`,
  so result 1 is `refusedDocument`), the result is the host's, and the raw record is the error's
  `cause`. `MisoEngineErrorInit` gains an optional `cause`.

  `hostRecordFailure` is no longer exported. The callers keep their own reply handling (the
  revision note and `controlCallFailure`).
- D1a (checked and left unchanged):
  - `seekSession()`, `seekReport()`, `planLatency()` and the meter and telemetry leases (`meters()`,
    `telemetry()`, `measurement.ts`) already turn the record's result into their own typed
    refusal.
  - The track-response watch maps a `status()` rejection to its lifecycle error.
  - The `sessionMap()` of source retirement after a replace (`retireAbsent`) and the boot path's
    `dispose()` after a refused spectrum boot swallow their rejection, so it never reaches the app.
  - `pcm-feed.ts`'s `seekSourceAt()` and `seekReport()` already turn the record's result into
    the feed's typed refusal.
  - An app's own `createHost` is the app's code: `createEngine()` passes its rejection through.
- D2 (retries): each caller's retry rule is unchanged and reads the error's code, as #2193 D2
  decided for the writer. The one input that changes is a host-local refusal, which was a raw
  record before and is now typed. The observation owner resends an `Observe` refused with
  `backpressure` after a render. A host-local result 6 (64 frames in flight, which drain) is now
  that retryable `backpressure`, and a failed host's sticky result 6 is `wrongState`, which is
  thrown. Before, both were the raw record and were thrown at once. Spectrum, loudness and track
  response do not retry on a code, so they reject with the typed error where they rejected with
  the record before. The protocol client gains the sticky rule: a failed host's sticky result 6
  was `backpressure` there and is now `wrongState`.
- D3: no module, worklet or protocol change.

## Gate (one test per claim)

`sdk/test/browser-evals.mjs`, `#2201: no browser send hands the app the host's raw miso.error.v1
record`. This is one table test, and each row fails at its own host call:

- The host double refuses every frame (except the control-list reads and the capability probes)
  with the raw record of result 1. A protocol call (`counters`), a live-control `submit`,
  `configureSpectrum`, `readObservations`, `subscribeLoudness` and `queryTrackResponse` must each
  reject with `MisoEngineError` code `invalidArgument`.
- An `Observe` that the host refuses locally with result 6 enters the render wait. The render mark's
  `status()` rejects with the raw record in one row, and the wait's poll does in the other. The
  `subscribeObservations` call must reject with `MisoEngineError`.
- On a failed host, whose `status()`, `sessionMap()` and `dispose()` reject with the sticky raw
  record of result 6, `liveControls()`, `position()` and `close()` must reject with
  `MisoEngineError` code `wrongState`.
- A host that refuses the first `CAPABILITIES_GET` (a new queue's revision probe) makes
  `counters()` reject with `invalidArgument` (attempt 2, MAJOR 1).
- A boot with a spectrum tap, whose second `CAPABILITIES_GET` the host refuses, rejects in the same
  way.
- On the failed host, the PCM feed's anchored start (`feed.ready()`) rejects with `wrongState`
  (fold).

`sdk/test/browser-defaults-evals.mjs`, `default host failures close accepted context and map the
factory's raw refusal` (D1b): this test pinned the old behaviour (the raw record) and now asserts
the new contract. It is the one test of that claim.

Folded: the #2190 browser test (a protocol frame, now the `protocol` row) is deleted, and so is the
#2193 test's `submit()` result-1 step (now the `liveControls` row). The #2193 test keeps the
writer's retry and sticky-failure claims.

## Evidence (2026-10-29, base `888bbc901`, artifact `e65c1595`)

- Attempt 2: each site reverted to the raw host call, one at a time. Every row turns red and names
  itself:

  | reverted site | row that turns red |
  | --- | --- |
  | live queue / control-list send | `liveControls` |
  | queue revision probe (`openBrowserControlQueue`) | `the queue's revision probe` |
  | `sessionMap()` in `createBrowserLiveControls` | `liveControls() on a failed host` |
  | spectrum send | `spectrum` |
  | boot capability probe | `the boot's capability probe` |
  | observation send | `observation` |
  | loudness send | `loudness` |
  | track-response send | `trackResponse` |
  | protocol send | `protocol` |
  | `status()` in `position()` | `position() on a failed host` |
  | render wait `mark` | `the observation render mark` |
  | render wait poll | `the observation render wait` |
  | `dispose()` in `close()` | `close() on a failed host` |
  | `status()` in the PCM feed's `readAnchor` (fold) | `the feed's anchored start on a failed host` |
  | the `miso.error.v1` mapping in `default-host.ts` (fold) | `default host failures ... map the factory's raw refusal` |

  The test passed 10 of 10 runs on the tree.
- Attempt 1 verify (of `2cabe4121`): the queue-probe revert left every test green (MAJOR 1, now a
  row). Chromium and WebKit passed on both legs, with one #2198 `sdk-protocol-lossy` flake on WebKit
  worker that passed on rerun.
- Fold (rebased onto `e858c2503`, artifact rebuilt from that tree; the wasm module is unchanged and
  the control worker is #2207's): every row above red on its own revert and green on the tree. The
  SDK headless evals pass (709 of 709), and so do `browser-pcm-evals.mjs` (the PCM case table, 51
  of 51), `check-sdk-types.sh`, `check-sdk-deletions.py`, `check-sdk-generated.sh`,
  `sdk-package.sh check`, `check-workspace-policy.sh` and the Chromium browser qualification
  (`--sdk-root --check-matrix`), single and worker legs.
- README: the SDK README has no section for the next release, so there is no upgrade note. The
  "Observe the rendered mix" paragraph states the boot refusal's new type.
- Attempt 2 green: the SDK headless evals (705 of 705); `check-sdk-types.sh`,
  `check-sdk-generated.sh`, `check-sdk-deletions.py`, `sdk-package.sh check`,
  `check-workspace-policy.sh`; the Chromium browser qualification (`--sdk-root --check-matrix`),
  single and worker legs, with the SDK rows.
- README: a paragraph at the end of "Observe the rendered mix" states the one path and the retry
  rule.
