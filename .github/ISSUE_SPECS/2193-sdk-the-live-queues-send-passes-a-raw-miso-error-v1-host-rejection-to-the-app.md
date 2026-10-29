# SDK: the live queue's send passes a raw miso.error.v1 host rejection to the app

GitHub #2193. Left open by #2190 (PR #2192).

## Root cause

`hostControlSend` in `sdk/src/browser/live-controls.ts` called `host.control` without the
`hostRecordFailure` mapping that #2190 added in the same file, so a host failure during a
live-control submit (and a control-list or control-state read, which use the same send) reached
the app as the frozen record `{ tag: "miso.error.v1", requestId, result }`, not as an `Error`.
Confirmed on `0f97cc7ef`: the protocol client's send and the queue's revision probe mapped it; this
send did not.

## Decision record (attempt 1)

- D1: `hostControlSend` maps a host rejection through `hostRecordFailure`, like the other two sends.
- D2 (the writer's retries): `LiveControlWriter` chooses by error type, so the mapping decides it.
  The host rejects `control()` before the port with `RESULT_INVALID_ARGUMENT` (1, an empty frame
  or one over the frame limit), `RESULT_BACKPRESSURE` (6, more than 64 frames in flight), and
  `wrongState` or `internal` (255) for a host that failed. `backpressure` becomes a
  `MisoEngineError` with code `backpressure`, which the writer already retries as flow control (the
  plan's in-flight limit takes the same type): frames in flight drain, so it is flow control.
  Every other mapped code is terminal: the writer drops the batch and throws, because a frame over
  the limit or a failed host never succeeds on resend. An error that is not a host record passes
  unchanged, so its handling is today's (left staged and thrown).
- D3 (attempt 2, verify of `d41af1727`, MINOR): a host that failed rejects every later call with
  its sticky record, and that record can carry result 6 (the worklet makes a refused
  `control_event` dequeue sticky with its result). It has the same shape as the local in-flight
  refusal. So on a mapped `backpressure` the send asks `host.status()`: a failed host rejects it at
  once with its sticky record, and the SDK then raises `MisoEngineError` code `wrongState` (result
  6), which the writer drops and throws; a healthy host answers, and the refusal stays
  `backpressure`. No module or protocol change. A host without `status()` (outside the shipped
  declaration) keeps the plain mapping. The other unmapped sends are #2201.
- D4 (attempt 2 PASS, folded MINOR): the question waits for no reply. `status()` shares the port
  with the control frames and is answered behind them, so awaiting it made a flush wait until the
  frames drained, and forever on a host that stopped answering. A failed host refuses `status()`
  locally, before it posts anything, so the SDK waits one task turn (`setTimeout(0)`), by which a
  local refusal has settled; no tuning number is needed. A `status()` still unsettled then means a
  host that has not failed, and the refusal stays `backpressure` at once, as before attempt 2.

## Gate (one test per claim)

`sdk/test/browser-evals.mjs`, `#2193: a host's raw miso.error.v1 rejection of a live-control frame
is a MisoEngineError`: a host that refuses one `SET_CONTROL` frame with result 6 leaves the
writer's flush refused for backpressure with the edit pending, and the next flush admits it; a
refusal with result 1 rejects `submit()` with `MisoEngineError` code `invalidArgument`; a host
failed with a sticky result-6 record, whose `status()` rejects, makes the writer's flush throw
`wrongState` and drops the batch (attempt 2). The healthy double's `status()` never settles, so the
backpressure case also holds that the refusal does not wait for a reply (folded MINOR).

## Evidence (2026-10-29, base `0f97cc7ef`)

- Red with the mapping removed: the writer's flush throws `{ tag: 'miso.error.v1', requestId: 0,
  result: 6 }`.
- Attempt 2: red with the `status()` question removed (`Missing expected rejection.`: the flush
  resolves refused for backpressure against the failed host).
- Folded MINOR: red with the probe awaiting `status()` (`Promise resolution is still pending but
  the event loop has already resolved`: the flush never settles).
- Attempt 2 green: the SDK headless evals (705 of 705) and the Chromium browser qualification,
  both legs, with the SDK rows.
- Attempt 1 green: the SDK headless evals (704 of 704) on the artifact built from this tree.
- README: the writer paragraph of "Change a mix and read the core's answer" states the mapping
  and the retry decision.
