# Seek the timeline and every source from the browser SDK and the PCM feed

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A browser app moves the playhead with one call, `engine.seek(frame)`. The SDK sends one session
seek to the worklet, which calls the module export of draft 06a between blocks, and resolves with
the generation it used. Only then does it reposition every PCM producer it was given to that
generation and frame. The shared-ring PCM feed accepts those ring seeks without asking the engine
to seek the same sources again, so no ring stalls. No suspended-context requirement is added.

## Context

- **Two browser paths reach the per-source export.**
  - The shipped host message: `MisoAudioWorkletHost.seekSource` posts `miso.seek.v1`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1277-1292`; typed at
    `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:896-898`), and the worklet calls the
    export between blocks (`receiveSeek`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1715-1739`;
    dispatch at `:1011-1012`).
  - The shared-ring feed: a producer writes a ring's seek words and bumps its epoch
    (`Msb1RingWriter.seek`, `sdk/src/browser/pcm-ring.ts:32`); the feed processor, a subclass of the
    engine processor (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:243`), sees the
    new epoch at block start and calls the per-source export (`applySharedSeek`, `:440-470`). A
    result that is neither OK nor backpressure is counted and returned (`:456-460`), the epoch stays
    unseen, and the drain returns before it submits any PCM (`:341`), on every later block too.
- **Why the feed needs a rule.** After a session seek at generation `g` (draft 06a), every source
  is already at `g`. A producer that then writes `g` into its ring makes the feed call the
  per-source export with `g`, which is refused as `source.generation.stale`; by the code above that
  ring never drains again.
- **The ring writers belong to the app.** `BrowserEngine` (`sdk/src/browser/engine.ts:192-218`)
  has no seek and no ring writer; rings come from `attachEngineFeed`
  (`sdk/src/browser/pcm-feed.ts:285`), and the stem-store pump writes them and seeks every stem on
  its own generation (`CanonicalPcmPump.seek`, `hosts/host-web/web/stem-store/pcm-pump.js:210-220`;
  `hosts/host-web/web/stem-store/index.d.ts:206`).
- **`prepareSeek`** (`sdk/src/browser/pcm-feed.ts:76-83`) frees stale slots while the context is
  suspended. It stays as it is.

## Decisions frozen for this slice

- **D1. The shipped host message.** `MisoAudioWorkletHost.seekSession({ generation, frame })`
  posts `miso.session-seek.v1`; the worklet's `receiveSessionSeek` calls
  `miso_engine_web_v1_session_seek` (draft 06a D1) between blocks and acknowledges with its result.
  One session seek may be unsettled at a time; a second is refused locally with
  `RESULT_BACKPRESSURE`, as the host does for a second seek of one source.
- **D2. The shared-ring rule.** The engine processor records the generation of the last session
  seek the export accepted. In `applySharedSeek`, a ring epoch whose requested generation equals
  that generation is consumed without calling the per-source export: its seek is already applied.
  The epoch is marked seen and the ring's depth is reset, as for an applied seek. Any other epoch is
  handled as today.
- **D3. `BrowserEngine.seek`.** `seek(frame: bigint, options?: { generation?: bigint; producers?:
  readonly SessionSeekProducer[] }): Promise<{ generation: bigint }>`.
  - The generation is `options.generation`, else one above the largest generation this engine
    object has used for a session seek, starting above 1 (every source and the timeline start at
    1). A caller that also seeks sources by itself passes its own; a stale one is refused with
    `source.generation.stale` and nothing changes.
  - It calls `host.seekSession` once. Only after the acknowledgement is OK does it call
    `producer.seek(frame, generation)` on each producer, in order. A producer clamps the frame to
    its own region, as the pump does today. On a refusal it repositions nothing and rejects with
    the typed result.
  - `SessionSeekProducer` is `{ seek(frame: bigint, generation: bigint): void | Promise<void> }`.
    `CanonicalPcmPump.seek(frame, generation?)` implements it: with a generation it uses that one
    (it must be above the pump's own), without one it keeps today's increment.
- **D4. The acked-batch question.** The export is all or nothing (draft 05 D1). The SDK repositions
  producers only after an OK, so no producer feeds a generation the engine refused. D2 consumes an
  epoch only for a seek the engine already applied. No ack can precede a drop.

## Deliverables

1. D1 in the shipped host JS and its `.d.ts`, and in the worklet.
2. D2 in the feed worklet.
3. D3 in the SDK browser engine; the pump's optional generation.
4. Tests below.

## Authorized paths

- `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (`receiveSessionSeek` and its dispatch),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
- `hosts/host-web/web/stem-store/pcm-pump.js`, `hosts/host-web/web/stem-store/index.d.ts`
  (`seek`'s optional generation only)
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` (D2 only), `sdk/src/browser/`,
  `sdk/test/`
- `scripts/test-web-audioworklet.mjs` (one hermetic test)

## Non-goals

- The module export and the headless SDK (draft 06a).
- The anchored session seek's export (draft 06a).
- Any change to the routing of *Move browser source submission and seeks into the Worker* (#1387),
  which lands first (through #1293): the session seek takes the thread the source seek takes.
- Any change to `prepareSeek`, `waitForPcmRunway` or the per-source seek's semantics.

## Hazards

- **Stream H owns these files** (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`): the worklets,
  the shipped host and the SDK browser code. Root sequences the merge with #1293, #1332 and #1387.
- **Ordering in the app.** A producer that repositions its ring before the session seek is
  acknowledged makes the per-source path take generation `g` first, and the session seek is then
  refused as stale with nothing changed. D3 does the steps in the right order; the `seek` and
  `CanonicalPcmPump.seek` documentation states it for apps that drive producers themselves.

## Objective gates

1. **The ring rule** (`sdk/test/browser-pcm-evals.mjs`, new case on its vm harness). After an
   accepted session seek at generation 2, a ring epoch carrying generation 2 is consumed with no
   per-source export call and its later PCM is submitted; a ring epoch carrying generation 3 still
   calls the per-source export.
2. **Shipped host message** (`scripts/test-web-audioworklet.mjs`, new hermetic case).
   `seekSession` posts `miso.session-seek.v1`, the worklet calls the export once, and a second
   unsettled call is refused locally with `RESULT_BACKPRESSURE`.
3. **Browser engine order** (`sdk/test/browser-evals.mjs`, new case). With a fake host and two
   recording producers: the producers are called after the acknowledgement, with the returned
   generation; on a refused acknowledgement neither is called. A second `seek` without options uses
   the next generation.
4. **Pump generation** (`sdk/test/browser-pcm-evals.mjs`, new case). `CanonicalPcmPump.seek(frame,
   g)` writes `g` into every ring's seek words with each stem's clamped frame; a `g` not above its
   own generation is refused.
5. **No rendered bit moves** for an app that never calls `seek`: the browser legs of the `browser`
   job in `.github/workflows/qualification.yml` pass with unchanged digests.
6. **Commands:**
   - `bash scripts/test-web-audioworklet.sh`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 turns red if the feed calls the per-source export for an epoch the session seek already
  applied, which stalls that ring for good, or if it swallows an epoch it should apply.
- Gate 2 turns red if the message is not wired or two session seeks can be unsettled at once.
- Gate 3 turns red if the SDK repositions producers before or without an OK, or picks a stale
  generation.
- Gate 4 turns red if the pump ignores the session's generation and writes one of its own.

## Dependencies

- Draft 06a *Seek the timeline and every source from the browser module export and the headless
  SDK* (the export, which calls draft 05's D1 function).
- Batch: R1.
