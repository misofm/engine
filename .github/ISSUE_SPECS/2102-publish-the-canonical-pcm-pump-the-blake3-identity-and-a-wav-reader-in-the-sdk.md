# Publish the canonical PCM pump, the BLAKE3 identity and a WAV reader in the SDK

Filed by root from the console sprint (2026-10-26). Source: the scope note `scratchpad/opt/scope/sdk-console-gaps.md`, item 3.

## Product outcome
A browser app loads local WAV stems into the engine through the npm package: it reads the WAV header, computes the engine's `blake3:` content identity, and pumps canonical PCM into the `Msb1RingWriter` rings, with no hand-written decoder.

## Smallest slice
- Publish `CanonicalPcmPump` and `deinterleaveCanonicalPcm` (hosts/host-web/web/stem-store/pcm-pump.js, pure) and `IncrementalBlake3`/`blake3Stream` (identity.js and incremental-blake3.js, repo-owned, with their provenance json).
- Stage them into `dist/browser/stems` the way `stage-package.mjs` (~:66) stages the feed worklet. Hand-write the d.ts from stem-store/index.d.ts (~155-225).
- `pcm-pump.js` must stop importing `StemStoreError` from opfs-store.js, and gets its own error class.
- Add a WAV header reader. None exists in the repo; the console's `readWavInfo` is a reference. It handles PCM 16 and 24 bit, WAVE_FORMAT_EXTENSIBLE, odd-chunk padding, an over-long data size, and walks chunks with small reads. It refuses 32-bit float with a typed error that names the fix, and refuses more than 2 channels.
- Out of scope: `OpfsStemStore`, `StemSessionGate`, the resolvers and the pump worker (phase 1 is local files; that part is #394).

## Gates
- `CanonicalPcmPump` from the package pumps a 16-bit mono and a 24-bit stereo canonical blob into `Msb1RingWriter` rings with exact int-to-f32 values. This catches a wrong deinterleave or scale.
- `blake3Stream` of a fixture equals the engine's `blake3:` identity for it. This catches a staged copy that diverges from the engine's.
- One eval of the WAV reader over a RIFF with an extensible fmt and an odd chunk.
- `sdk-package.sh check` and the tarball smoke include the new entry.

## Paths
sdk/codegen/stage-package.mjs, sdk/src/browser/index.ts, new sdk/src/browser/stems.ts and its d.ts, hosts/host-web/web/stem-store/pcm-pump.js (the error import only), sdk/README.md, sdk/test/browser-pcm-evals.mjs, this spec.
