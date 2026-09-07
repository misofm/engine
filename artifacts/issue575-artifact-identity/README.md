# Issue #575 unchanged-artifact delivery verification

The single authorized probe found no drift from the delivered #570 AudioWorklet SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`. Root therefore ran one ordinary build at documentation-only head `ec992089d1fa1208ceb49ff6ca65e6e2c05bf2ed` into `/tmp/issue575-artifact-ec992089` and compared it with the retained #570 qualified artifact `/tmp/issue570-qualified-artifact-3871e137`.

All six ordinary output files are byte-identical: ABI layout, host declaration, host JavaScript, AudioWorklet JavaScript, simd128 Wasm and parameter metadata. The Wasm remains pinned at the hash above. Static/object/ABI checks, browser expected resources plus 26 red mutations, generated matrix consistency, SDK dependency installation and the SDK packaging/self-test gate all pass. Resource and PCM expectations are unchanged.

No browser was launched for #575. The identical bytes reuse #570's qualified Chromium `151.0.7922.34`, Firefox `153.0` and WebKit `26.5` execution with its original candidate attribution `3871e137b519815540c1b3abcd6cfcec7932efa7`; #575 does not relabel that run. No pin, result lineage, matrix, manifest, lockfile, source or test file changed.

`01-build-preflight-mistake.*` preserves a non-credit status-2 harness error: the first capture omitted the builder's required pre-existing output directory and stopped before compilation. The corrected `02-build.*` is the credited ordinary build. Every later credited gate exited 0. Raw streams are losslessly compressed. `sha256sums.txt` covers every retained payload except itself and is verified from the repository root.
