# MSB1 feed drain: read the generation tag before the seek epoch

GitHub #2184. Found by the bounded verify of #1387 (2026-10-28), in both MSB1 consumers: the SDK
prelude `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` and the control Worker's copy.

## Root cause

The drain reads `SEEK_EPOCH` before `GENERATION_TAG`, while the writer stores the tag before it
bumps the epoch (`sdk/src/browser/pcm-ring.ts`). A drain pass can therefore observe the old epoch
with the new tag and submit new-generation chunks before it issues `source_seek`. The engine
probably refuses them, but the slot is still consumed (`REFUSED`). The window is tiny, and it
predates #1387.

## Smallest slice

Read the epoch, then the tag, then the epoch again, and retry on a change; or reorder the writer so
that the epoch bump publishes the tag. Both consumers take the same fix.

## Gate (one test per claim)

One case in the shared MSB1 consumer case table (#1387, `sdk/test/msb1-consumer-cases.mjs`) that
interleaves the writer's two stores with a drain pass: red on the current read order.
