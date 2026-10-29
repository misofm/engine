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
the PCM feed files (another branch owns them).

## Decision record (attempt 1)

- D1: `hostControl(host, frame, document?)` in `sdk/src/browser/live-controls.ts` is the browser
  engine's one send to the host's `control` message. A rejection goes through
  `hostControlFailure`, the #2193 mapping with its sticky result-6 rule. Every `host.control` call
  of the SDK's browser engine now goes through it: the live queue and control-list send, the queue's
  revision probe, the protocol client, the spectrum frames, the boot's capability probe, the
  observation endpoint, the loudness frame and the track-response requester.
  `hostRecordFailure` is no longer exported. The callers keep their own reply handling (the
  revision note and `controlCallFailure`).
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
record`. The host double refuses every frame (except the control-list reads and the capability
probes) with the raw record of result 1. Each browser call must then reject with `MisoEngineError`
code `invalidArgument`: a protocol call (`counters`), a live-control `submit`,
`configureSpectrum`, `readObservations`, `subscribeLoudness` and `queryTrackResponse`. A boot
with a spectrum tap, whose second `CAPABILITIES_GET` the host refuses, must reject in the same way.

Folded: the #2190 browser test (a protocol frame, now the `protocol` row) is deleted, and so is the
#2193 test's `submit()` result-1 step (now the `liveControls` row). The #2193 test keeps the
writer's retry and sticky-failure claims.

## Evidence (2026-10-29, base `888bbc901`, artifact `e65c1595`)

- Red on each send reverted to a raw `host.control`, one at a time. Each run fails with a message
  that names its row: `spectrum`, `the boot's capability probe`, `observation`, `loudness`,
  `trackResponse`, `protocol` (the #2190 mapping removed) and `liveControls` (the #2193 mapping
  removed), `... passed the host's raw record`.
- Green: the SDK headless evals (705 of 705) and `check-sdk-types.sh`.
- README: a paragraph at the end of "Observe the rendered mix" states the one path and the retry
  rule.
