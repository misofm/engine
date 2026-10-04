# Send a replacement session to the AudioWorklet

Slice B5 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A page can post a new session document to the running engine AudioWorklet and keep hearing audio:
the worklet runs the replacement between two `process()` calls through the exports of *Export
session replacement from the browser engine module* (#1293), refreshes everything it caches, and
answers with the result. A replacement that grows the module's memory is expected and not fatal. It
can also post an anchored seek.

## Context

- The worklet processor (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, class at `:242`) boots
  in `initialize` (`:268-428`), caches views on `this.exports.memory.buffer` and forces every lazily
  allocated staging buffer before caching them, because a memory growth detaches views (`:336-341`).
- Today any change of `memory.buffer` is a fatal sticky `RESULT_REPREPARE_REQUIRED`: in `receive`
  (`:1005-1008`) and in `process` (`:1823-1828`). A replacement may grow memory.
- Messages are dispatched in `receive` (`:983`) by `tag` with exact field sets.
- The SDK's shared-memory feed wraps this processor class (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:243`)
  and pre-cuts views on the attach path; after a memory growth its drains refuse until re-cut
  (`:283-290`, `:334-338`).
- The host wrapper and its declarations: `miso-engine-v1-audio-worklet-host.js` and
  `miso-engine-v1-audio-worklet-host.d.ts`; `scripts/check-sdk-generated.sh:70-77` requires
  `sdk/src/browser/shipped-host.d.ts` to equal that declaration file.
- The hermetic harness `scripts/test-web-audioworklet.mjs` runs the worklet against fake exports
  (`:2242`, `:2752-2760`), so it proves message and refresh logic, not rendered bits. Rendered bits are
  proven natively (B2-B4) and in real browsers (B6).

## Decisions frozen for this slice

- **D1. Messages.** `miso.replace.v1 { tag, requestId, document }` (a `Uint8Array`) and
  `miso.seek-at.v1 { tag, requestId, sourceId, generation, sourceFrame, anchorSample }`. Exact field
  sets. A successful replace replies with the new resources and session map.
- **D2. Refresh.** Inside the replace handler, after the export returns, re-read every pointer and
  view and every cached map (session map, live-control bindings, observation map, meter frame shape)
  as `initialize` does, and set `this.memoryBuffer` to the current buffer. A memory growth caused by
  the replace is therefore not a fault; one with no replace in flight still is.
- **D3. Feed re-cut.** After a refresh the processor calls an overridable `afterReplace()`. The SDK
  feed wrapper overrides it to re-cut every attached ring's `idTarget` view on the current buffer and
  staging pointer, so shared-ring drains resume on the next block. Without it, every stem fed
  through a shared ring starves after a replacement that grows memory or moves the source-ID
  staging.
- **D4. Failure.** A refused replace changes no cached map, refreshes views only if memory grew, and
  replies with the typed result and the diagnostic.
- **D5. Timing.** The handler's duration enters the telemetry lease when it is on, so B1's estimate
  can be compared with the real cost.

## Deliverables

1. D1-D5 in the worklet, the host wrapper and its declarations; copy the declarations to
   `sdk/src/browser/shipped-host.d.ts`. D3's override in the SDK feed worklet.
2. Hermetic tests in `scripts/test-web-audioworklet.mjs` (message validation, refresh, refusal,
   growth).

## Authorized paths

- `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, `miso-engine-v1-audio-worklet-host.js`,
  `miso-engine-v1-audio-worklet-host.d.ts`
- `sdk/src/browser/shipped-host.d.ts`
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` (D3's override only), `sdk/test/browser-pcm-evals.mjs`
- `scripts/test-web-audioworklet.mjs`, `scripts/test-web-audioworklet.sh`

## Non-goals

- No Rust change. No SDK API (B7-B8). No browser qualification leg (B6).

## Objective gates

1. **Protocol.** In the hermetic harness: a well-formed `miso.replace.v1` calls the staging and
   replace exports in order; malformed messages are refused without calling them.
2. **Refresh after growth.** A fake replace that grows memory leaves the processor rendering (no
   sticky fault), with every view on the new buffer; a growth with no replace in flight is still
   fatal.
3. **Refusal.** A refused replace changes no cached map and leaves the sticky result unchanged.
4. **Declarations mirrored.** `bash scripts/check-sdk-generated.sh` passes.
5. **Feed survives.** In `sdk/test/browser-pcm-evals.mjs`, after a replacement that grows memory,
   every attached ring drains again on the next block (its drain counter advances).
6. Commands:
   - `bash scripts/test-web-audioworklet.sh`
   - the artifact build and `scripts/check-web-audioworklet.sh` as in B4
   - the umbrella's inherited gates.

## Test value

- Gate 2: a worklet that treats the replace's own growth as fatal kills the engine on the first
  large replacement; one that refreshes on any growth hides a real fault. Both turn it red.
- Gate 3: a refused replace that still rebinds maps turns it red.
- Gate 5: a feed that keeps its pre-cut views refuses every drain after the growth; it turns red.

## Dependencies

- *Export session replacement from the browser engine module* (#1293).
