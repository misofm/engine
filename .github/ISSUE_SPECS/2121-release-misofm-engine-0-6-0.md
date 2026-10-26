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

## Evidence and decision record (attempt 2, 2026-10-26)
- Why attempt 1 failed: qualify run 40282768798 refused, correctly. Main 346c662 builds `455e771cd32a3faef113ddfe51cba433996b9a6f190b0d7faf1e8108ff216816` (5,763,203 B), not the pinned `049a07f3`: #2123 (#2115 loudness S1) merged after the release PR and edited crates/true-peak-limiter/src/lib.rs.
- Cause (measured): named twins of e33f418 and 346c662 (both 6,526,337 B, same sections) differ in 158 bytes, all in the Data section; the Code section is byte-identical. Every differing word is a line number of a panic `Location` whose file is `crates/true-peak-limiter/src/lib.rs` (35 B): old lines 533-1695 move +11 and 1929-5959 move +12, exactly the lines #2123 added (the `annex2` module and doc comments). So the `pub` keywords themselves change no byte; the added source lines move the embedded panic line numbers. A code change, not a nondeterminism: each commit's digest reproduces (049a07f3 at e33f418 here and in attempt 1; 455e771c here and in CI).
- Re-pinned the module to `455e771c...` (pin file, `EXPECTED_WORKLET_SHA256`, test-npm-publish-modes.py's accepted pin); `--check-pin` passes. Built from 775f10c94 (main 346c662 plus the spec and the re-pin).
- Qualification (`--browser all --record-matrix --candidate-commit 775f10c94 --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink): Chromium 151.0.7922.34 pass, WebKit 26.5 pass.
- BLAKE3 module unchanged: `ee373d2f...` (44861 B); still no own pin (#2117).
