# Release @misofm/engine 0.6.0

Filed by root for the console sprint (2026-10-26). The owner approved publishing SDK releases when their release PR is green, through the manual qualify-then-publish workflow.

## Product outcome
`@misofm/engine` 0.6.0 on npm, adding to 0.5.0:
- #2111: the WebAssembly SIMD BLAKE3 (`prepareBlake3`), about 1.5 GB/s per worker, and the allocation-free JS fallback;
- #2116: `planLatency()` on both engines;
- #2118: concurrent PCM pump window reads with read-ahead, and the stale-window seek fix.

## Smallest slice (docs/RELEASE.md, "The release PR")
Re-pin the worklet module, record a fresh two-browser qualification (Chromium and WebKit), set the 0.6.0 release identity (npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and lock, the README install line and release record #2121), and add a short "Upgrading from 0.5.0" note (additive API; behaviour changes). No Rust source change. After the merge: qualify, publish, verify.

## Evidence and decision record (attempt 1, 2026-10-26)
- Worklet module: `049a07f3fef5938a20937df03c4c80f3e4506158d378dda3e5985f0a4f5040bd`, the same bytes as 0.5.0 (#2111, #2116 and #2118 do not move it). The pin and `EXPECTED_WORKLET_SHA256` stay; `--check-pin` passes, and a second build from another checkout path gives the same bytes.
- BLAKE3 module (seventh closure file): `miso-engine-v1-blake3.simd128.wasm`, `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44861 B) in this build. 0.6.0 ships it without its own release pin; that pin is open successor #2117. What covers it now: npm-publish.yml's `qualify` builds the whole closure with `build-web-audioworklet.sh` and stages it with `sdk-package.sh build`; the package manifest records every file's size and SHA-256; `package-tarball-smoke.mjs` (qualify, and the re-smoke before `publish`) requires the exact seven-file manifest, the module packed once in `dist/assets`, and `prepareBlake3()` loading it from its default URL to hash the stem-identity corpus. Only the worklet module's digest is pinned against a release value.
- Qualification (`--browser all --record-matrix --candidate-commit aed90d175 --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink): Chromium 151.0.7922.34 pass, WebKit 26.5 pass.
- Release identity: npm-publish.yml's five `0.6.0` literals; test-npm-publish-modes.py follows (its "wrong release version" mutation moves 0.6.0 to 0.5.0). Release record: this issue.
- Identity report (`--event push --before 2c77c8a44`): RELEASE line, every check passes. The base had no recorded digest yet (ARTIFACT CANNOT TELL); CI's run on the PR compares against it.
- sdk/README.md: "Upgrading from 0.5.0"; the pump paragraph (two windows per source, eight reads in flight) and the manifest's artifact count (seven) are corrected.
