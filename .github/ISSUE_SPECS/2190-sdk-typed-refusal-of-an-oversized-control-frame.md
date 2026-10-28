# SDK: an oversized control frame is refused untyped in the browser (raw miso.error.v1 record)

GitHub #2190. Found by the studio-v2 console's bus add at 46 or more tracks (SDK 0.9.0, Chromium
worker mode, 2026-10-28). SDK only. The 16 KiB frame limit itself is #2191's, not this issue's.

## Problem

- The browser host's `control()` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`)
  refuses a frame over its frame limit before it is sent, with the frozen record
  `{tag: "miso.error.v1", requestId: 0, result: 1}`. The record is not an `Error`.
- `ProtocolClient` (`sdk/src/core/protocol-messages.ts`) never checks a frame against the core's
  reported `maximumFrameBytes`, so it sends the frame.
- The browser transport `send` (`sdk/src/browser/engine.ts`) converts only a non-zero
  `reply.result`, so the raw record reaches the app.
- On the headless engine the same frame fails as `MisoEngineError` `bufferTooSmall` (the core
  stages no buffer for it). The C ABI codec refuses it as `limit_exceeded`.
- The live-control queue already refuses an oversized submit before it stages it
  (`sdk/src/core/control-queue.ts`, `#requireFits`).

The host is not damaged: render continues and smaller edits still commit.

## Contract

1. `ProtocolClient` checks every frame it sends against the core's `maximumFrameBytes` before the
   transport sees it: `applyTransaction`, `replaceSession` (the frame; the document goes beside it
   and is not a frame) and every call through its one call path. The limit is read once with
   `CAPABILITIES_GET` before the first frame that needs it; that one request is a header with no
   fields and is not checked. A larger frame rejects with `MisoControlRefusal`, `status` and `code`
   `limit_exceeded` (the codec's status for the same frame), one diagnostic whose `detail` is
   `frame_bytes=<n>,maximum_frame_bytes=<m>`, a message that names both numbers, and `revision`
   the revision the frame expected. Nothing is sent and nothing is committed. Both hosts get the
   same refusal, because both build their calls on `ProtocolClient`.
2. The browser transport `send` converts a host rejection that carries a raw `miso.error.v1`
   record into `MisoEngineError` (`phase: "lifecycle"`, `code` the result's name, `result`), as
   the seek path does. Any other rejection passes unchanged.

## Non-goals

- The frame limit's value (#2191).
- Splitting an oversized transaction. The caller splits it; the refusal names the numbers it needs.

## Gates

1. `sdk/test/protocol-messages-evals.mjs`, "#2190: a frame over the core's frame limit is refused
   typed and never sent": a `ProtocolClient` over the headless core, with a transport that rejects
   a frame over the limit with the raw record, as the browser host does. Test value: red when
   `ProtocolClient` sends a frame over `maximumFrameBytes`, or refuses it untyped.
2. `sdk/test/browser-evals.mjs`, "#2190: a host's raw miso.error.v1 rejection of a protocol frame
   is a MisoEngineError": the stand-in host rejects `COUNTERS_GET` with the raw record. Test value:
   red when the browser transport passes the host's local refusal record through to the app.

## Paths

`sdk/src/core/protocol-messages.ts`, `sdk/src/core/errors.ts` (an optional message on
`MisoControlRefusal`), `sdk/src/browser/engine.ts`, the two eval files above, this spec.

## Evidence (2026-10-28, base `cf22f2a19`, module `b32a5429`)

- Red on the base code (the `sdk/src` changes stashed, the two tests kept):
  - gate 1: `not ok`, the rejection is the raw record (`error: '[object Object]'`), not a
    `MisoControlRefusal`.
  - gate 2: `not ok`, `The validation function is expected to return "true". Received false`
    (the raw record reached the caller).
- Probe: the oversized frame in gate 1 is 16,912 bytes (31 `upsertTrack` edits) against the
  headless core's 16,384-byte limit. Sent directly, the headless core fails it as
  `MisoEngineError` `bufferTooSmall`.
- Green with the change: both tests pass; `scripts/check-sdk-headless.sh` 700 of 700;
  `scripts/check-sdk-types.sh`, `scripts/check-sdk-generated.sh`,
  `scripts/check-sdk-deletions.py`, `scripts/sdk-package.sh check` and
  `scripts/check-workspace-policy.sh` pass.
