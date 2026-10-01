> **Status: tracker. No implementation authorized; per-repo slices to be briefed and approved separately.**
> Raised from the #379 slice analysis, which rated this above every API change in that proposal.

## The duplication, measured

The same stem-store concern — content identity, OPFS staging, incremental SHA-256, the verified lease, the bounded PCM pump, the worker client — is implemented three times:

| location | size | notes |
| --- | --- | --- |
| `engine/hosts/host-web/web/stem-store` | 120K | `identity.js`, `opfs-store.js`, `pcm-pump.js`, `pcm-pump-worker.js`, `resolver.js`, `session-gate.js`, `worker-client.js`, `incremental-sha256.js` |
| `misofm/app` `src/lib/mixer/stem-store` | 428K | plus its own FLAC packetizer, worker pool, ingest ledger, gate machine, capabilities |
| `misofm/app` `src/lib/mixer/engine` | 532K | hand-rolled boot, AudioWorklet host, SAB feed, console writer, request-id shadow counter |
| `@misofm/engine-web-adapter` `dist/stems` | — | the intended owner |

The filename overlap tells the story on its own: `identity`, `gate`/`session-gate`, `pump`, `worker-client`, `sha256` and `store` each appear in more than one copy.

## Why this is worth more than the API changes

Every defect the #379 arc surfaced was found *once per copy*. The request-id collision was independently worked around in three places (`app`'s shadow counter with its documented cascade failure, `misofm/website`'s retry-with-fresh-id, the qualification harness's `10001`/`20001` magic numbers). The int16 exactness trap was hit again by a third integration months after the first. Three copies means three chances to diverge and three places to fix anything.

Consolidating deletes code rather than adding abstraction, which is the standard #379 sets for itself and mostly fails to meet elsewhere.

## Ordering — this tracker is gated

The adapter must first be the thing worth depending on. In flight: engine-web-adapter#12 replaces WebCodecs with a bounded libFLAC WASM decoder, targeting `0.2.0` and Chromium + macOS/mobile Safari parity. Nothing here should start before that lands and is qualified.

## Proposed per-repo slices, in order

1. **Adapter is consumer of record.** #12 ships `0.2.0` with bit-exact integer PCM across WebKit and Chromium. Gate: a real third-party integration (`misofm/website`) drives the mixer on Safari and Chromium from the adapter's own FLAC path with no caller-side decode.
2. **`misofm/website` migrates.** Drop the hand-rolled `decodeAudioData`, the float32→int16 conversion, and the `MemoryStemResolver` wiring in favour of handing the adapter FLAC URLs. This is the cheapest real test of whether the new surface is right: **if the integration does not shrink substantially, the design is not done.**
3. **`misofm/app` migrates** off `src/lib/mixer/engine` and `src/lib/mixer/stem-store` onto the adapter. Largest slice; needs its own brief and its own evidence, and should not be attempted before 2 has proven the surface on a smaller consumer.
4. **Delete `engine/hosts/host-web/web/stem-store`.** Gate: no in-repo consumer remains; qualification harness sources the adapter or its own fixtures.

## Non-goals

Not an API redesign — that is #379 and its slices (#393 is slice 1). Not a change to the canonical PCM contract or to `pcmSha256` as object identity. Not a transport/URL policy change; that stays caller-owned.

Related: #379, #393, misofm/engine-web-adapter#8, #11, #12.
