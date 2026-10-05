# Seek the timeline and every source from the browser SDK and the PCM feed

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`; attempt 3 read them again on `c63f5f37d`, whose code
equals `6ee64f484`'s (`git diff --name-only 6ee64f484 c63f5f37d` lists only `docs/handoffs/` and
`.github/ISSUE_SPECS/`).

This slice is written for the browser as *Move browser source submission and seeks into the Worker*
(#1387) and *Swap and retire browser plans through the Worker's service loop* (#1381) leave it. Both
land before it (through draft 06a and #1293).

## Product outcome

A browser app moves the playhead with one call, `engine.seek(frame)`. The SDK sends one session
seek to the control half: the Worker on a cross-origin-isolated page (`worker` mode), the worklet's
control handler otherwise (`single` mode). The control half calls the module export of draft 06a and
replies with the result and the generation it used. Only after an OK does the SDK reposition every
PCM producer it was given to that generation and frame. The MSB1 drain, in the realm where #1387
runs it, accepts those ring seeks without asking the engine to seek the same sources again, so no
ring stalls. No suspended-context requirement is added.

## Context

- **Where the per-source seek runs after #1387.**
  - `worker` mode: the Worker holds the producer half (#1387 D1). `miso.seek.v1` travels on #1332's
    page-to-control-plane port to the Worker (#1387 D4), and the Worker calls
    `miso_engine_web_v1_source_seek` (#1387 D2), which after #1381 goes through `SessionState`'s
    source routing (#1381 D2). The worklet's post-boot export set has no source export (#1387 D2).
  - `single` mode: the worklet's own control handler calls the exports, outside the render-locked
    window (#1387 D2). Today's handler is `receiveSeek`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1715-1739`; dispatch at `:1011-1012`),
    reached from `MisoAudioWorkletHost.seekSource`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1277-1292`; typed at
    `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:896-898`).
  - The host routes control messages to the control half by mode (#1332 D7). The internal messages
    of #1294 (`miso.apply.v1`, `miso.seek-at.v1`) use the same routing; this slice adds one more.
- **Where the MSB1 drain runs after #1387.**
  - `worker` mode: the SDK's feed module is loaded into the control Worker as a module. A repeating
    timer of one quantum runs the drain on each tick and after each control message (#1387 D5). It
    keeps today's rules: apply a published seek through the source seek export, drop stale
    generations, stop at `RESULT_BACKPRESSURE`.
  - `single` mode: the worklet prelude stays as it is (#1387 D5). It drains at the top of
    `process()` and applies a published seek in `applySharedSeek`
    (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:441-470`). A result that is neither
    OK nor backpressure is counted and returned (`:456-460`), the epoch stays unseen, and the drain
    returns before it submits any PCM (`:341`), on every later block too.
  - A producer writes a ring's seek words and bumps its epoch (`Msb1RingWriter.seek`,
    `sdk/src/browser/pcm-ring.ts:32`), in either mode.
- **Why the drain needs a rule.** After a session seek at generation `g` (draft 06a), every source
  is already at `g`. A producer that then writes `g` into its ring makes the drain call the
  per-source export with `g`, which is refused as `source.generation.stale`. By the code above that
  ring never drains again, in both realms.
- **The ring writers belong to the app.** `BrowserEngine` (`sdk/src/browser/engine.ts:192-218`)
  has no seek and no ring writer. Rings come from `attachEngineFeed`
  (`sdk/src/browser/pcm-feed.ts:285`). The stem-store pump writes them and seeks every stem on its
  own generation (`CanonicalPcmPump.seek`, `hosts/host-web/web/stem-store/pcm-pump.js:210-220`;
  `hosts/host-web/web/stem-store/index.d.ts:206`).
- **`prepareSeek`** (`sdk/src/browser/pcm-feed.ts:76-83`) frees stale slots while the context is
  suspended. It stays as it is.

## Decisions frozen for this slice

- **D1. The session seek message.** `MisoAudioWorkletHost.seekSession({ generation, frame })` posts
  `miso.session-seek.v1 { tag, requestId, generation, frame }` (both `bigint`) with an exact field
  set. It takes #1387's routing for control messages (#1387 D4, #1332 D7):
  - `worker` mode: the message travels on the page-to-control-plane port to the Worker. The
    Worker's handler calls `miso_engine_web_v1_session_seek` (draft 06a D1).
  - `single` mode: the worklet's control handler calls the same export, outside the render-locked
    window, between blocks.
  - The handler replies `miso.session-seek.v1 { tag, requestId, result, generation }` with the
    export's result. A malformed message is refused without calling the export.
  - One session seek may be unsettled at a time. A second is refused locally with
    `RESULT_BACKPRESSURE`, as the host does for a second seek of one source.
  - The method is marked `@internal` in the declarations and copied to
    `sdk/src/browser/shipped-host.d.ts`, as #1294 D6 does for its methods. The SDK is the public
    surface.
- **D2. The drain rule.** The component that calls the export records the accepted generation, and
  the drain reads it in its own realm:
  - The handler of D1, after an OK and before its reply, stores the generation in one realm-local
    value, `lastSessionSeekGeneration` (a `bigint`, initially none). It is on the control Worker in
    `worker` mode, and on the engine processor in `single` mode.
  - The drain's seek step reads that value in the same realm: #1387's Worker drain module in
    `worker` mode, `applySharedSeek` in the worklet prelude in `single` mode. A ring epoch whose
    requested generation equals `lastSessionSeekGeneration` is consumed without calling the
    per-source export, because its seek is already applied. The epoch is marked seen, the applied
    counter is incremented and the ring's depth is reset, as for an applied seek. Any other epoch is
    handled as today.
  - The handler and the drain run on one thread in each mode: the Worker's event loop (the drain
    runs on a timer tick or after a control message, never during one), or the audio thread (the
    control handler runs between `process()` calls). So the drain never reads a value that a
    concurrent handler is writing.
- **D3. `BrowserEngine.seek`.** `seek(frame: bigint, options?: { generation?: bigint; producers?:
  readonly SessionSeekProducer[] }): Promise<{ generation: bigint }>`.
  - The generation is `options.generation`, else one above the largest generation this engine
    object has used for a session seek, starting above 1 (every source and the timeline start at
    1). A caller that also seeks sources by itself passes its own. A stale one is refused with
    `source.generation.stale` and nothing changes.
  - Before it posts anything, it refuses a `frame` above `Number.MAX_SAFE_INTEGER` with a typed
    `RangeError` (nothing changes). It calls `host.seekSession` once with the `bigint` frame. Only
    after the reply is OK does it call `producer.seek(Number(frame), generation)` on each producer,
    in order: the SDK converts the frame, because the pump takes a `number` frame
    (`hosts/host-web/web/stem-store/index.d.ts:206`; `pcm-pump.js:210-211` validates it) and a
    `bigint` generation (its own counter is a `bigint`, `pcm-pump.js:212`). A producer clamps the frame to
    its own region, as the pump does today. On a refusal it repositions nothing and rejects with
    the typed result.
  - `SessionSeekProducer` is `{ seek(frame: number, generation: bigint): void | Promise<void> }`.
    `CanonicalPcmPump.seek(frame, generation?)` implements it: with a generation it uses that one
    (it must be above the pump's own), without one it keeps today's increment.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** The export is all or
  nothing (draft 05 D1), and the reply carries its result. The SDK repositions producers only after
  an OK, so no producer feeds a generation the engine refused. D2 consumes an epoch only for a
  generation the engine already applied to every source.

## Deliverables

1. D1 in the shipped host JS and its `.d.ts`, in the control Worker script and in the worklet's
   control handler.
2. D2 in the control Worker script, in #1387's Worker drain module and in the feed worklet prelude.
3. D3 in the SDK browser engine; the pump's optional generation.
4. Tests below.

## Authorized paths

- `hosts/host-web/web/miso-engine-v1-control-worker.js` (#1332's script: the `miso.session-seek.v1`
  handler and `lastSessionSeekGeneration`)
- `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (the `single`-mode control handler for
  `miso.session-seek.v1` and `lastSessionSeekGeneration` only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`
  (`seekSession` only)
- `hosts/host-web/web/stem-store/pcm-pump.js`, `hosts/host-web/web/stem-store/index.d.ts`
  (`seek`'s optional generation only)
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` and the Worker drain module that
  #1387 adds beside it (D2's seek step only)
- `sdk/src/browser/engine.ts`, `sdk/test/`
- `scripts/test-web-audioworklet.mjs` (the hermetic cases of gate 2)

## Non-goals

- The module export, its anchored form and the headless SDK (draft 06a).
- Any change to #1387's routing or to its drain's other rules: this slice adds one message on that
  routing and one rule in that drain's seek step.
- Any change to `prepareSeek`, `waitForPcmRunway` or the per-source seek's semantics.

## Hazards

- **Stream H owns these files** (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`): the worklet,
  the control Worker, the shipped host and the SDK browser code. Root sequences the merge with
  #1294 and #1297, which edit the same handler files and the same drain.
- **#1297's anchored ring seek.** If *Feed and retire the sources a browser edit adds or removes*
  (#1297) has landed, the drain's seek step calls `miso_engine_web_v1_source_seek_at` for a ring
  whose anchor is not `-1` (#1297 D3). D2's rule runs before that choice, so it covers both exports.
- **Ordering in the app.** A producer that repositions its ring before the session seek is
  acknowledged makes the per-source path take generation `g` first. The session seek is then
  refused as stale with nothing changed. D3 does the steps in the right order; the `seek` and
  `CanonicalPcmPump.seek` documentation states it for apps that drive producers themselves.

## Objective gates

1. **The drain rule, in both realms** (`sdk/test/browser-pcm-evals.mjs`, new cases on its vm
   harness). For each drain, the Worker drain module and the worklet prelude: after an accepted
   session seek at generation 2, a ring epoch that carries generation 2 is consumed with no
   per-source export call, and its later PCM is submitted. A ring epoch that carries generation 3
   still calls the per-source export.
2. **The message, in both modes** (`scripts/test-web-audioworklet.mjs`, new hermetic cases).
   - `worker` mode: `seekSession` posts `miso.session-seek.v1` to the Worker. The Worker calls the
     export once and replies with its result. The worklet receives no message and calls no export.
   - `single` mode: the worklet's control handler calls the export once, between blocks.
   - In both modes a second unsettled call is refused locally with `RESULT_BACKPRESSURE`, and a
     malformed message calls no export.
3. **Browser engine order** (`sdk/test/browser-evals.mjs`, new case). With a fake host and two
   recording producers: the producers are called after the OK reply, with the returned generation.
   On a refused reply neither is called. A second `seek` without options uses the next generation.
   The producers receive the frame as a `number`. A frame of `2n ** 53n` rejects with a
   `RangeError` and the fake host receives no message.
4. **Pump generation** (`sdk/test/browser-pcm-evals.mjs`, new case). `CanonicalPcmPump.seek(frame,
   g)` writes `g` into every ring's seek words with each stem's clamped frame. A `g` not above its
   own generation is refused.
5. **No rendered bit moves** for an app that never calls `seek`: the browser legs of the `browser`
   job in `.github/workflows/qualification.yml` pass with unchanged digests, on the isolated and
   the non-isolated leg.
6. **Commands:**
   - `bash scripts/test-web-audioworklet.sh`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 turns red if either drain calls the per-source export for an epoch the session seek
  already applied, which stalls that ring for good, or if it swallows an epoch it should apply, or
  if the rule is put in a realm where the drain does not run.
- Gate 2 turns red if the message goes to the worklet in `worker` mode (which has no source
  export), is not wired in `single` mode, or two session seeks can be unsettled at once.
- Gate 3 turns red if the SDK repositions producers before or without an OK, or picks a stale
  generation.
- Gate 4 turns red if the pump ignores the session's generation and writes one of its own.

## Dependencies

- Draft 06a *Seek the timeline and every source from the browser module export and the headless
  SDK* (the export, which calls draft 05's D1 function).
- *Move browser source submission and seeks into the Worker* (#1387): the routing of D1 and the
  Worker drain of D2.
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332): the
  control Worker script and the ports.
- *Send a session transaction to the browser control plane* (#1294): its internal messages on the
  same routing and the `@internal` declaration convention (#1294 D6) that D1 follows.
- Batch: R1.
