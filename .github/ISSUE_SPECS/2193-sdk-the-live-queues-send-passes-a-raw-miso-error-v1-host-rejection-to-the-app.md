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

## Gate (one test per claim)

`sdk/test/browser-evals.mjs`, `#2193: a host's raw miso.error.v1 rejection of a live-control frame
is a MisoEngineError`: a host that refuses one `SET_CONTROL` frame with result 6 leaves the
writer's flush refused for backpressure with the edit pending, and the next flush admits it; a
refusal with result 1 rejects `submit()` with `MisoEngineError` code `invalidArgument`.

## Evidence (2026-10-29, base `0f97cc7ef`)

- Red with the mapping removed: the writer's flush throws `{ tag: 'miso.error.v1', requestId: 0,
  result: 6 }`.
- Green: the SDK headless evals (704 of 704) on the artifact built from this tree.
- README: the writer paragraph of "Change a mix and read the core's answer" states the mapping
  and the retry decision.
