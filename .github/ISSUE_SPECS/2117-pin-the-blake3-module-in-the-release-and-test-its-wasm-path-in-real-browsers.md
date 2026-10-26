Filed by root from the #2111 verification (2026-10-26).

## Problem
#2111 ships `miso-engine-v1-blake3.simd128.wasm` in the delivery closure. Two gaps remain:
1. Its Wasm path is tested only in node V8. No committed test runs it in a real browser (Chromium or WebKit), in a Worker. The engine's web tree resolves the default URL to a path that does not exist, so the browser qualification legs take the JS fallback.
2. The release fingerprints only the AudioWorklet module. The BLAKE3 module has no release pin.

## Smallest slice
- One qualification row in Chromium and WebKit that loads the module in a Worker, resolves `"simd128"`, and hashes the corpus with the expected identities.
- A release pin for the BLAKE3 module beside the worklet pin, checked by the identity report and by npm-publish's qualify step, with a test-npm-publish-modes mutation.
