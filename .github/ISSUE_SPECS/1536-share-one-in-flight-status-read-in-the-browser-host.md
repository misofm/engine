# Share one in-flight status read in the browser host

Decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Successor S2 of *Hold effect parameter, bypass and EQ-target values in latest-target cells*
(#1345), from the stream B batch-3 acceptance review (root ruling 6, 2026-10-13). A product defect
in the shipped browser host, found while diagnosing the qualification page's `result 6`.
Code anchors verified at `806de000a` (`codex/d15-stream-b3`; the files below are unchanged by the
split commit `4858c4c14`). Verify them again on `main` before the first edit.

## Product outcome

Any number of callers may ask the browser host for its status at the same time. Each call
resolves with a status reply that the worklet produced after every request the host had sent
before the call. No status call is refused locally with `RESULT_BACKPRESSURE`. Today a second
status call while one is in flight is refused, so the SDK's managed observation wait, its live
track-response read, its spectrum query and an app that calls `engine.host.status()` can make each
other fail.

## Context

- **The refusal.** `status()` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1309-1311`)
  calls `#request(..., "status")` (`:1218-1263`), which rejects with `webError(RESULT_BACKPRESSURE)`
  before it allocates a request ID when `#saturated("status")` is true (`:1228-1229`). That arm is
  `this.#inFlightStatus >= 1` (`:1179`); `#reserve` counts up (`:1201-1202`), `#release` counts
  down (`:873-874`), `#fail` clears it (`:1157-1170`). `webError` (`:522-526`) gives
  `{ tag: "miso.error.v1", requestId: 0, result: 6 }`.
- **The documented reason.** The host's streaming-model comment (`:699-713`) says "Status stays at
  one because a second answer would carry no new information". The `.d.ts` twins say "plus one
  unsettled seek per source and one unsettled status" (`miso-engine-v1-audio-worklet-host.d.ts:154-160`,
  byte-identical in `sdk/src/browser/shipped-host.d.ts`); `status(): Promise<MisoStatus>` is at
  `:948` with no doc.
- **Every request gets the next request ID.** `#allocateRequestId` (`:859-863`) is strictly
  monotonic and every message the host sends to the worklet goes through `#request` (prepared
  commands too: `#ensurePreparedControl` `:782-806` routes `configCopy` and submits through it; the host's only
  `postMessage` is in `#request`, `:1256`). The worklet handles messages in arrival order and answers on one FIFO port
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1076-1084`, the status reply built from
  `readStatus()` `:944-960`). So a status request with ID `n` is answered after every request with
  an ID below `n`.
- **The SDK has three independent status callers** (`sdk/src/browser/engine.ts`):
  1. the managed observation wait, `renderWait.next` (`:1008-1032`): `host.status()` at `:1022` for
     the start count, then a poll every 4 ms at `:1027`;
  2. the live track-response read, `readTrackResponse` (`:1067`), whose lifecycle check `status()`
     (`:1097-1110`, `host.status()` at `:1098`) runs at `:1112` and `:1132`;
  3. the spectrum query, `querySpectrum` (`:1149`), `awaitSpectrum(host.status(), ...)` at `:1263`;
     `awaitSpectrum` (`:381-406`) stops waiting at its deadline and leaves the request in flight.
  The engine also hands the raw host to the app (`readonly host: MisoAudioWorkletHost`, `:195`;
  returned at `:1292`), so an app's own `host.status()` is a fourth caller the SDK cannot see.
- **What the refusal does to a caller.** In `renderWait.next` the refusal rejects the inner loop,
  and `.catch((error) => finish(false, error))` (`:1030`) rejects `next` with the raw record.
  `#submit` in `sdk/src/core/observation-subscriptions.ts:881-916` awaits `wait.next(remaining)`
  (`:912`) with no catch, so a managed `subscribe`, `update` or `close` that overlaps a spectrum
  query, a track-response read or an app status read fails with a raw
  `{ tag: "miso.error.v1", requestId: 0, result: 6 }`, not with a `MisoEngineError`.
- **The test that pins the refusal today.** `scripts/test-web-audioworklet.mjs:1694-1709`:
  `assertBoundedNoBurn("status", ...)` holds one status read and requires a second to be refused
  locally with result 6 and no ID burn (`localErrorResult` `:96-100`). The harness's `FakePort`
  (`:137`) can hold every reply (`holdAll`, `heldAll`) and mutate a status reply
  (`statusMutation`, `:117`, `:160-167`).
- **The qualification page.** S1 (*Qualify observation unsubscribe on a real AudioContext in three
  browsers*) shares one status read inside the page itself, so it does not depend on this issue.

## Decisions frozen for this slice

- **D1. The host shares the read, not the SDK.** The refusal is a host-local policy, the host is the
  only place that sees every caller (the SDK's three and the app's), and the reason the host gives
  for its bound ("a second answer would carry no new information") is a reason to share, not to
  refuse. An SDK-side helper would leave the app's `engine.host.status()` able to break the SDK's
  managed wait. The SDK is not changed.
- **D2. The sharing rule.** The host keeps at most one status request on the port.
  - A `status()` call **joins** the in-flight status read when that read's request ID is the host's
    newest allocated request ID (no other request was sent after it). It gets the same promise.
  - Otherwise (a status read is in flight and a newer request was sent after it), the call joins the
    host's **follow-up read**: one pending status read that the host sends when the in-flight read
    settles. Every call that arrives before the follow-up is sent joins it. When the follow-up is
    sent it becomes the in-flight read and the rule repeats.
  - With no read in flight, the call sends a new one.
  So every reply a caller receives was produced by the worklet after every request the host had sent
  before that call (Context: IDs and FIFO order).
- **D3. Settling.** A joined read settles every joined caller with the same frozen reply, or the same
  rejection. A caller that stops waiting (a `Promise.race` timeout such as `awaitSpectrum`) does not
  cancel the read and does not stop later callers from joining it. When the in-flight read rejects,
  the follow-up is still attempted through `#request`, which rejects it at once if the host is
  disposed or has a sticky error, as for any request. `#fail` and `dispose()` settle a shared read
  exactly as they settle one pending status today.
- **D4. No local status refusal.** The `"status"` arm of `#saturated` and the `#inFlightStatus`
  count are removed (the follow-up state replaces them). `status()` never rejects with
  `RESULT_BACKPRESSURE`. Request IDs stay strictly monotonic; a joined call allocates none. The
  other arms of `#saturated` do not change.
- **D5. Documented contract.** The host comment (`:699-713`) and both `.d.ts` twins (the streaming
  comment at `:154-160` and a new doc on `status()` at `:948`) state D2 and D3 in one short
  paragraph: concurrent calls share one read; a reply reflects every request sent before the call;
  `status()` is never refused for being in flight. H #1332 (Worker control plane) must keep this
  contract wherever `status()` is answered.

## Deliverables

1. D2-D4 in `miso-engine-v1-audio-worklet-host.js` (`status()`, `#saturated`, `#reserve`,
   `#release`, `#fail` and the shared-read state; no other method).
2. D5 in the host comment and both `.d.ts` twins (byte-identical).
3. Gates 1-4 in `scripts/test-web-audioworklet.mjs`; the superseded `assertBoundedNoBurn("status", ...)`
   case deleted in the same change (AGENTS.md: a change that supersedes a test deletes it).
4. This spec's Attempt record: the red-first runs (gate 1 on the unchanged host) and the mutation
   runs.

## Authorized paths

- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (D1-D5 only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`
  (D5 text only)
- `scripts/test-web-audioworklet.mjs` (the status cases only)
- This spec.

## Non-goals

- No SDK change (`sdk/src/browser/engine.ts`, `sdk/src/core/observation-subscriptions.ts`). The raw
  record that `renderWait.next` passes through for any *other* host rejection is a separate defect
  (open question for root), not this slice.
- No change to the worklet (`miso-engine-v1-audio-worklet.js`), the Rust host or the status layout
  (S3 adds a status word; it does not touch sharing).
- No change to the other request kinds' bounds (sources, seeks, commands, observation reads,
  spectrum, leases). B #1524 owns the command arm; C #1406 the source arm.
- No qualification-page change (S1 shares its own read).

## Objective gates

Each test-value sentence answers: which plausible defect turns this red that no existing test
catches? All cases use the harness's real host on its `FakePort`.

1. **Concurrent callers share one read (red first).** With replies held (`holdAll`), call
   `status()` three times. Exactly one `miso.status.v1` request is posted. Release: all three
   resolve to one reply (same `requestId`); the next request of any kind gets that ID plus one.
   Run this case against the unchanged host first and record that the second call rejects with
   result 6 (the red).
   *Test value:* turns red if a second concurrent status call is refused (today's defect, which
   fails a managed observation `close()` that overlaps a spectrum query) or if each call posts its
   own read, which would let the in-flight count grow without bound.
   *Mutation:* restore the `"status"` arm of `#saturated`: red.
2. **A reply is never older than the call.** Hold replies. `a = status()`. Send one other request
   (for example `renderAllocationCount()`). `b = status()` and `c = status()`. Exactly one status
   request is on the port. Release one reply at a time: `a` resolves with the first read's ID; then
   exactly one new status request is posted, with an ID above the other request's; `b` and `c`
   both resolve with that reply.
   *Test value:* turns red if a call joins a read that was sent before a request the caller had
   already sent (a plain "share whatever is in flight" rule), which would let the managed wait read
   a render count from before its refused command, or a lifecycle check read a state from before its
   capture.
   *Mutation:* join any in-flight read regardless of later requests: `b` resolves with `a`'s ID:
   red.
3. **Failure and disposal settle every joined caller.**
   - Hold replies; three joined calls; the reply fails validation (`statusMutation` making it
     invalid, as `:1361-1362` does): all three reject with the same `result 255` error, and a later
     `status()` rejects with the sticky error without posting.
   - Hold replies; two joined calls and one follow-up caller; `dispose()`: all three reject as one
     pending status rejects on dispose today, and nothing more is posted.
   *Test value:* turns red if a joined caller is left pending for ever (a shared promise that only
   the first caller's settlement path resolves) or if a follow-up read is sent after disposal.
4. **A caller that stops waiting does not break the others.** Hold replies; race one `status()`
   against a 1 ms timer and let the timer win; call `status()` again (it joins, nothing new is
   posted); release: the second call resolves.
   *Test value:* turns red if the host drops or cancels a read when its first caller stops waiting
   (the `awaitSpectrum` deadline case), which would leave the next caller hanging or posting a
   second read.
5. **Twins and policies.** `bash scripts/check-sdk-generated.sh` (the `.d.ts` twins byte-identical);
   `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-web-audioworklet.sh` on the built
   artifact set (`bash scripts/build-web-audioworklet.sh --named-twin <T> <A>`, as
   `qualification.yml` runs it); `bash scripts/check-sdk-types.sh`;
   `bash scripts/check-workspace-policy.sh`.
6. **Browsers.** The CI browser command in Chromium, Firefox and WebKit, once each, unloaded: all
   gates pass. (No soak: the change is in the main realm and gates 1-4 hold its behaviour.)

## Dependencies

- None to start. The code is on `main` today.
- **Before H #1332**, which rewrites how `status()` is answered (D5).
- Hot file: `hosts/host-web/web/miso-engine-v1-audio-worklet-host.{js,d.ts}`,
  `sdk/src/browser/shipped-host.d.ts` (`STREAMS.md:109` at `806de000a`). B #1524 (the command arm of
  `#saturated`) and C #1406 (the source arm) are independent arms: any order, the later slice
  rebases. `scripts/test-web-audioworklet.mjs` (`:113`): any order with B #1399, B #1524 and J #1496.

## Placement (root edits `STREAMS.md`)

Recommended: **stream H** (product risk), early in its queue and before H #1332.
- Every authorized path is stream H's (`hosts/host-web`, `sdk/`), so no named exception is needed.
- It is a small, independent product fix with no dependency, so it can land in H's next batch.
- S1 does not wait for it (S1's page shares its own read).
- Hot-file rows: add "H S2 (the status arm and `status()` only)" to the worklet-host row (`:109`)
  before H #1332, in any order with B #1524 and C #1406; add "H S2 (the status cases only)" to the
  `scripts/test-web-audioworklet.mjs` row (`:113`), any order.

## Attempt record

- Attempt 1: not started.

## Root rulings (2026-10-13)

- **R1. The host shares the read (D1).** Only the host sees every caller.
- **R2. The raw `renderWait.next` record** for other host rejections stays out of this slice.
  It is recorded here as a known defect for the SDK error-typing work; it is not fixed here.
- **R3. #1332.** #1332's spec gets a one-line dependency note ("the shared status read of this
  issue must hold when control moves to a Worker"), not a copy of the rule.
- **R4. Placement.** Stream H, early, before #1332.
