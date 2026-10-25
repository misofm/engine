# Read the browser track response through the same frames as the C ABI

Slice S21a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13), first half of the planner's S21: the SDK moves to the core's frames. #1580 deletes the
browser's own capture. Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs.

## Product outcome

In the browser and on the headless engine, `queryTrackResponse` and `subscribeTrackResponse` send
the same `RESPONSE_GET` frame as a C ABI host and read the same `RESPONSE_READY` event. The
capture runs in the core's render step and the composition in the core, so the browser, the
headless engine and a mobile app get the same curve and the same owner list. The SDK keeps its
public track-response types; it no longer parses a capture or starts a response Worker for it.

## Context

- **The SDK path today.** The browser engine's `readTrackResponse` (`sdk/src/browser/engine.ts:1067`)
  asks the worklet host to capture (`captureTrackResponse`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1453`; the worklet's
  `receiveTrackResponse`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1188`), parses the
  bytes for change detection (`parseTrackResponseState`, `sdk/src/core/live-response.ts:410`;
  `sameTrackResponseState`, `:483`) and sends them to the SDK's response Worker
  (`BrowserTrackResponse.query`, `sdk/src/browser/response.ts:273`; `TrackResponseModule`,
  `live-response.ts:592`, which parses owners in `#parseCapture`, `:783`). The headless engine
  calls the capture and the analysis synchronously (`queryTrackResponse`,
  `sdk/src/core/boundary.ts:811`; `sdk/src/headless/engine.ts:242`).
- **Public API.** `queryTrackResponse` (`sdk/src/browser/engine.ts:217`, a `Promise`;
  `sdk/src/headless/engine.ts:242`, synchronous) and `subscribeTrackResponse` (`engine.ts:215`;
  `headless/engine.ts:232`), with `TrackResponseQuery`, `TrackResponseResult` and
  `TrackResponseMember` (`live-response.ts:7-69`). Managed subscriptions read through the
  `responseRead` seam (`sdk/src/core/observation-subscriptions.ts:161-165`) and suppress an
  unchanged state (`:1083` onward).
- **The core side is done.** #1577 (capture in `PlanState::render`) and #1578 (`RESPONSE_GET`
  target 3, `RESPONSE_READY` `0x8040`). After *Swap and retire browser plans through the Worker's
  service loop* (#1381) the browser renders through `PlanState::render`, so the render step runs
  in the worklet's `process()`; on a page without cross-origin isolation, the same instance runs
  it.
- **Transport.** #1562 D1: the browser's one command entry (`control` port message and the
  `miso_engine_web_v1_control_*` exports, also on the headless engine). #1570: the browser and
  headless event lane. #1563: `sdk/src/core/control-frame.ts`; #1574: `sdk/src/core/response-frame.ts`.
- **Qualification rows.** `hosts/host-web/qualification/run.mjs:427-486`
  (`sdk-track-response-subscription`, including `workerInitializations === 1` and
  `unchangedBeforeRender`) and `hosts/host-web/qualification/sdk-response-entry.ts:319-396`.

## Decisions frozen for this slice

- **D1. One request path.** `queryTrackResponse` encodes `RESPONSE_GET` with `strip_id`, grid,
  points, endpoints and channels (#1573 D2), sends it through #1562's entry, and waits for the
  `RESPONSE_READY` whose `request_tag` equals the reply's, from #1570's event lane. An event with
  another tag is not consumed by this wait. A non-OK reply or a non-zero `outcome` throws
  `MisoEngineError` with the diagnostic code. The request deadline (`responseLimits.requestDeadlineMs`)
  applies from the send to the event.
- **D2. The headless engine is asynchronous too.** `OfflineEngine.queryTrackResponse`
  (`sdk/src/headless/engine.ts:242`) returns `Promise<TrackResponseResult>`; it settles after the
  next `render()` (`:324`) that the app calls and the event lane delivers the event. The capture needs a rendered block on every host; the synchronous form cannot
  exist on the C ABI, so it does not stay on one host.
- **D3. The result keeps its type.** From the event: `trackId` = `strip_id`; `mode` `"target"`;
  `meaning` `"eqFilterSubtotal"`; `sampleRateHz`, `floorDb`, `frequenciesHz`, `leftDb`, `rightDb`,
  `capturedSample`, `excludedMemberCount`; `snapshotToken` = `request_tag`; `resultBytes` = the
  event frame length. Each member: `trackId` = `strip_id`; `nativeId` = `effect`; `stableId` =
  `slot_id`, or `"input-filters"` for builtins; `rack` `"input"`, `"console"` or `"inserts"`;
  `rackValue` = the decision-17 section code (4, 5 or 2; its doc comment changes from the retired
  record codes 0, 4 and 2); `slot` = the member's index; `kind`, `kindValue`, `bypassed`;
  `available` = kind is not 3; `excludedReason` `"linearResponseUnavailable"` for kind 3;
  `enabledLeft` and `enabledRight` = the first `section_count` mask bits.
- **D4. Subscriptions.** The `responseRead` seam of each engine calls D1. The managed owner keeps
  the last published result and publishes a new one only when a member field or a curve bit
  differs. `TrackResponseObservedState`, `parseTrackResponseState` and `sameTrackResponseState`
  are deleted. One capture is in flight per engine (#1578); the managed owner already runs one
  response job at a time.
- **D5. SDK code deleted.** `TrackResponseModule` and the capture parser
  (`sdk/src/core/live-response.ts`, from `TrackResponseCapture` at `:71` to the end at `:1028`,
  keeping the public types, `:7-69`, and `normalizeTrackResponseQuery`, `:240`), `BrowserTrackResponse` and `createTrackResponse`
  (`sdk/src/browser/response.ts:205-397`), the `track-response-*` messages of the response Worker
  (`sdk/src/browser/response-worker.ts`), `TrackResponseHost` and `TrackResponseCapture`, and
  `trackResponseHostRequest` (`sdk/src/browser/engine.ts:236`). The worklet's capture and the
  `track_response_*` exports stay, unused, until #1580.
- **D6. Bounds without the capture record.** `responseBounds`
  (`sdk/src/core/observation-subscriptions.ts:521-531`) takes `maximumPoints` from #1574's generated
  `maximumResponsePoints`, and `maximumCaptureBytes` from `1 << 20` (the frame limit), so
  `TrackResponseSubscriptionBounds` keeps its shape and no SDK code reads a `liveResponse*`
  constant. `responseLimits.maximumResultBytes` is sent as `RESPONSE_GET` field 18
  (`maximum_reply_bytes`), which caps the `RESPONSE_READY` frame.

## Deliverables

1. D1-D4 in `sdk/src/browser/engine.ts`, `sdk/src/headless/engine.ts`, `sdk/src/core/boundary.ts`,
   `sdk/src/core/live-response.ts`, `sdk/src/core/observation-subscriptions.ts` and the event
   decoder in `sdk/src/core/response-frame.ts`.
2. D5.
3. The evals and the qualification rows below.

## Authorized paths

- `sdk/src/{browser,core,headless}/**` (the track-response paths named above),
  `sdk/test/{live-response-evals.mjs,response-subscription-evals.mjs}`.
- `hosts/host-web/qualification/{run.mjs,sdk-response-entry.ts}` (the track-response rows).

## Non-goals

- No change in `hosts/host-web/src` or the worklet scripts (#1580 deletes them).
- No committed-model track response. No change to the response preview (#1574).

## Objective gates

Commands: `bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`, and the browser
qualification (`npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix
--self-test-mutations` in `hosts/host-web/qualification`) for chromium, firefox and webkit.

1. **Same result on every host.** On the headless engine and in the browser (both legs), for the
   fixture session of `sdk/test/live-response-evals.mjs`: `queryTrackResponse` returns members
   and curves equal to the C ABI's `RESPONSE_READY` for the same document and request (decoded
   with the same codec in a Rust test fixture), and to the values the eval asserted before this
   slice for every field D3 does not redefine.
2. **Applied state.** After a live EQ band-gain edit, a query sent before the next render shows
   the new gain; the headless query does not settle until the app renders.
3. **One in flight.** Two concurrent `queryTrackResponse` calls on one engine: the managed path
   serializes them, and a raw second `RESPONSE_GET` while one is pending is refused with
   `response.capture.pending`.
4. **Subscriptions.** `sdk-track-response-subscription` in three browsers, updated: no response
   Worker is created for track responses (replaces `workerInitializations === 1`); with no edit,
   renders publish no new value; after an edit the next render publishes once; membership and
   values equal a one-shot query; grid updates, channel ownership and bounds keep their gates.
5. **Removed code.** `check-sdk-deletions.py` refuses `TrackResponseModule`, `BrowserTrackResponse`,
   `parseTrackResponseState` and `sameTrackResponseState`.

## Test value

- Gate 1 turns red if the SDK maps a member or curve field differently from the core's event.
- Gate 2 turns red if the SDK reads a stale or committed-model curve, or the headless engine
  answers without a render.
- Gate 3 turns red if the SDK can run two captures at once on one engine.
- Gate 4 turns red if the subscription republishes unchanged results or misses a change.
- Gate 5 turns red if the SDK keeps a second parser of engine bytes.

## Dependencies

- #1577 and #1578 (the core capture and its frames), #1574 (`response-frame.ts`), #1562 (the
  command entry), #1563 (the frame codec), #1570 (the browser and headless event lane).
- *Swap and retire browser plans through the Worker's service loop* (#1381) and *Run the browser
  control plane in a Worker and keep the AudioWorklet render-only* (#1332): the browser render half
  is `PlanState::render`.
- *Bound the browser SDK close path with named failures* (#1532) and *Share one in-flight status
  read in the browser host* (#1536) edit the same SDK functions; they land first and this slice
  rebases (`docs/handoffs/decision-15-2026-10-05/STREAMS.md` row 104 for `sdk/src/browser/engine.ts`).

## Supersedes and shrinks

- None. #1580 ends the browser's own capture.

## Decision record (#1579a, 2026-10-25)

Slice #1579a (scope note, root): D1, D2, D3, D6 and the parity fixture, on the headless engine.
#1579b keeps D4 for the browser, the browser's half of D5, the qualification rows and gate 5.

- **Split of D5.** The browser engine still reads through its response Worker until #1579b: its
  qualification rows render an `OfflineAudioContext` before they query, so a D1 query there needs
  the #1579b row changes in the same slice. #1579a deletes only what the headless engine used:
  `TrackResponseModule.queryActive`, `queryActiveIfChanged` and its capture-export path, and
  `WasmBoundary.queryTrackResponseIfChanged`. `TrackResponseModule.querySnapshot*`,
  `parseTrackResponseState`, `sameTrackResponseState`, `BrowserTrackResponse`,
  `trackResponseHostRequest` and the Worker's `track-response-*` messages stay for #1579b.
- **Seam.** The `responseRead` seam keeps its shape. The headless read is one D1 query; its state
  key is built from the decoded result (`trackResponseStateOf`: every member field and curve bit,
  not the sample, tag or frame length), so the owner publishes only on a change (D4's rule, on the
  headless engine now).
- **Event fields.** The generated frame table holds only the `RESPONSE_GET` messages, so the
  `RESPONSE_READY` field IDs are written in `response-frame.ts`; the fixture holds them to the core.
  `response-frame.ts` stays free of `ABI_LAYOUT` and `WebAssembly`; result codes map in the caller.
- **Fixture.** `crates/capi/tests/response_strip.rs` writes `sdk/test/fixtures/response-ready.json`
  (the document, the strip request frame and the `RESPONSE_READY` frame the C ABI answered, with the
  codec's values). `sdk/test/live-response-evals.mjs` encodes the same request (bytes), decodes the
  event through D3, and asks the headless engine the same question of the same document.
- **`onEvent`.** `decodeProtocolEvent` (not in this slice's paths) refuses `0x8040`, which made an
  `onEvent` listener report an error after every query. Both engines' `onEvent` now skip a
  `RESPONSE_READY`: the query reads it. A `ProtocolEvent` kind for it is left to a later issue.

SDK public behaviour changes (record them in the changelog with the PR):

1. `OfflineEngine.queryTrackResponse` and `WasmBoundary.queryTrackResponse` return
   `Promise<TrackResponseResult>`, settled after the next `render()` (R2), rejected with
   `MisoUsageError` "exceeded its deadline" when `requestDeadlineMs` (default 5 s) passes first.
   A disposed engine rejects with `MisoUsageError`.
2. Concurrent headless queries run one at a time in call order (no "already in flight" refusal).
3. A headless query no longer returns the state before the next render: it reads the state the
   next block applied (gate 2). A headless `subscribeTrackResponse` and `pump()` settle only when
   the app renders.
4. Headless; the browser follows in #1579b. `rackValue` is the decision-17 section code: input
   `4`, console `5`, inserts `2` (was 0, 4, 2). Rack names do not change. `snapshotToken` is the
   request tag; `resultBytes` the event frame's length; `enabledLeft`/`enabledRight` have one entry
   per section the core reports. The browser engine keeps the capture record's values until #1579b.
5. A refusal raises `MisoEngineError` whose `code` is the core's diagnostic (for example
   `response.reply_too_large`, was `refusedBudget`) and whose `result` is the mapped result class.
   `responseRefusalResult` now maps `backpressure` to `backpressure` (was `internal`).
6. `WasmBoundary.queryTrackResponseIfChanged` is removed.
7. `engine.onEvent` (both engines) never delivers a `RESPONSE_READY`.
8. `TrackResponseSubscriptionBounds` takes `maximumPoints` from `maximumResponsePoints` and
   `maximumCaptureBytes` from `1 << 20` (same values as before).
9. Headless: `responseLimits.maximumResultBytes` bounds the `RESPONSE_READY` frame: it is sent as
   `maximum_reply_bytes`, and a request whose event bound exceeds it is refused
   `response.reply_too_large` (was a bound on the capture record).
10. Headless: while a query runs, or a timed-out query's tag is held until its event is read, the
    requester is an event listener, so events read then are not counted in
    `discardedControlEvents`.
11. Headless: a query made while none is in flight sends its frame before `queryTrackResponse`
    returns, so `query(); render(); await` settles at that render. After a timeout the next query
    waits, under its own deadline, until the timed-out tag's event is read, instead of being refused
    `response.capture.pending`.

Root rulings on #1579a (2026-10-25):

- The split of D5 is accepted: the browser-path deletions listed above move to #1579b.
- `onEvent` skipping `RESPONSE_READY` stands for this slice; #1579b decides whether a
  `ProtocolEvent` kind for it is exposed.
- Gate 5 is not a source-grep gate (AGENTS.md refuses tests that grep source, and
  `check-sdk-deletions.py` checks only numeric offsets since #1050). #1579b proves the deletion by
  the code being gone and its tests deleted.
- Disposing an engine rejects every pending and queued track response query at once with the error
  of a call on a disposed engine (`MisoUsageError` "this engine was disposed; boot a new one"), not
  at the deadline. `loadSession` disposes first, so it does the same.
- Behaviour change 3 is D2's behaviour; the doc comments of the headless `subscribeTrackResponse`
  and of `TrackResponseSubscription.pump` say the read settles at the next `render()`.

Review fold-in after attempt 2 (root, 2026-10-25):

- A `RESPONSE_READY` that cannot be decoded while a timed-out tag is held ends the held tag, and
  the query waiting for it fails with the decode error (one capture is ever armed).
- A tag reply that arrives after its query's deadline is held: until that send settles, the
  requester keeps listening and the next query waits. A fake asynchronous transport in
  `live-response-evals.mjs` reaches the case; the headless send returns before a deadline passes.
- `decodeResponseReady` checks the frame header and every rule of `response_ready_is_valid` and the
  codec's default limits: at least two finite points, a finite floor, one side or two of the axis's
  length, masks within the section count (at most 32), 127-byte effect and diagnostic, the
  stable-ID grammar for the strip and slot IDs, 64 KiB strings, 1024 fields per level. The codec
  eval holds one corrupt frame per rule.
- A query made from an event callback runs a nested event drain (documented on
  `queryTrackResponse`).
