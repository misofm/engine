# Release @misofm/engine 0.8.0

Filed by root for the engine banking finish on 2026-10-27. The owner approved this release on 2026-10-27, through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.8.0 on npm. It changes no SDK API (no `sdk/src` file changed since 0.7.0, merge 26d8f97c8). It adds these engine changes to 0.7.0:
- **#2129 S2 (#2157):** a settled-bypassed per-node effect (the delay and the reverb, which never bank) skips its wet path. Un-bypass from rest (S1b, 0.7.0) is unchanged; the warm-up of both effects is zero. No rendered sample moves.
- **Banking:** #2135 (pattern packing, #2160), #2134 (the executor order is the graph contracted by bank) and #2136 (ASAP/ALAP window placement, #2161), #2138 (the leftover insert pass, #2162), and #2142 (pack groups across split strips, #2154). No rendered sample moves.
- **#2147 (#2156):** the reverb bank-report row (tools only).
- **Easy-fix batches #2159 and #2163:** #2096 and #2158 (a table-only plan replacement during a stored automation ride renders the uninterrupted bits, off the fader grid and with a coincident VCA step); #1670 (AArch64 and seek-invariance automation qualification); #2103 (suspended-context PCM feed attach: a qualification row and README text, no SDK change); #2104 (tool tail caps); #2117 (the BLAKE3 module's own release pin, `hosts/host-web/web/miso-engine-v1-blake3-artifact.sha256`, mirrored by `EXPECTED_BLAKE3_SHA256`, and its browser Worker qualification).
- **#1306 and #1668 (#2148):** each effect's span window is sized from the producers its plan has; the C ABI's obsolete span word is reserved. No browser SDK change.

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2152 did for 0.7.0. Check git history for `.github/ISSUE_SPECS/2152-release-misofm-engine-0-7-0.md`.
- Re-pin the AudioWorklet module (it changed: the per-node skip, the planner changes and the automation fixes). Use `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Check the BLAKE3 module against its pin and `EXPECTED_BLAKE3_SHA256` (docs/RELEASE.md, "BLAKE3 release identity"); re-pin only if its bytes changed.
- Record a fresh two-browser qualification, Chromium and WebKit, with `--browser all --record-matrix --candidate-commit <sha> --self-test-mutations`. It also records `results.json.blake3Qualification`. Firefox is out of scope (owner, 2026-10-25).
- Set the 0.8.0 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, and docs/RELEASE.md.
- Add sdk/README.md "Upgrading from 0.7.0". It must name every behaviour change.
- Run the identity report. This PR changes no Rust source.

## Release window

Between this PR's merge and the npm publish, root merges no engine PR that can change the browser module (see the memory rule "release window: hold module merges").

## Gates

- Every release-PR gate named in docs/RELEASE.md.
- The qualification record and the identity report: every check passes.
- After the merge: npm-publish.yml qualify (expected_sha = main head), then publish (qualification_run_id), then verify mode. `npm view @misofm/engine version` must read 0.8.0.

## Evidence and decision record (attempt 1, 2026-10-27)

- Worklet module: `7af24e632f9bbedfb05d5c6b0e1fada17e703519e9e0b396d3682d088025caf7` (6,118,606 B; named twin `252840d8...`, 6,924,057 B), built from this tree with `nightly-2026-08-20`. It replaces 0.7.0's `aa7578f9...`. Main 0d326c5c8's own CI run (40462488924) recorded the same digest, so this PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256` and test-npm-publish-modes.py's accepted pin carry it. `--check-pin` passes. A second build from a `git archive` export of this tree at another path gives the same bytes for every file of the closure (same `CARGO_HOME`; the identity report's twin input).
- BLAKE3 module: `miso-engine-v1-blake3.simd128.wasm`, `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B), unchanged since 0.6.0. It equals its pin (`hosts/host-web/web/miso-engine-v1-blake3-artifact.sha256`, #2117) and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --record-matrix --candidate-commit 6ee9f757c --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a `/tmp/qa.XXXXXX` socket): Chromium 151.0.7922.34 pass, WebKit 26.5 pass, each with the `simd128` BLAKE3 Worker row and the suspended-context feed row. Every render-allocation count is 0. `results.json` (with `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md` record it. The candidate's tree differs from the published head only in this spec and the qualification records; no module input.
- Release identity: npm-publish.yml's five `0.8.0` literals and the pin; `EXPECTED_BLAKE3_SHA256` unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20` (`rust-toolchain.toml` `[browser-artifact]`). test-npm-publish-modes.py follows (its "wrong release version" mutation moves 0.8.0 to 0.7.0) and passes. sdk/package.json and its lock read 0.8.0. docs/RELEASE.md names 0.8.0 (#2164) as the last release. Release record: this issue.
- Identity report (`--event push --before 0d326c5c8`): `ARTIFACT UNCHANGED` against the base's recorded digest, reproducible, the RELEASE line with every check passing, and the BLAKE3 RELEASE line (pin, `EXPECTED_BLAKE3_SHA256` and both browsers' Worker row agree with the built bytes).
- sdk/README.md: install line and release-record link at 0.8.0; "Upgrading from 0.7.0" says the SDK API is unchanged and names the per-node bypass skip for `miso.delay` and `miso.reverb` (no sample moves; un-bypass from rest unchanged), the banking planner changes (no sample moves), the automation fix (#2096, #2158: a table-only plan replacement during a ride now renders the uninterrupted samples), and the suspended-context feed attach text (#2103, behaviour as in 0.7.0).
- Local gates, all pass: the route self-tests; docs-gates; the `artifact` build (with named twin) and producers check for both modules; the `sdk` job (check-sdk-generated, check-sdk-deletions, check-sdk-types, check-sdk-headless, sdk-package.sh check, with an empty private TMPDIR at the end; the source-only asset tests); `artifact-gates` (named-twin strip check, check-web-audioworklet.sh, check-browser-expected-resources, scalar-oracle-absent, test-web-audioworklet.sh with a private TMPDIR, render-lock probe, V8 spill on Node 22.23.2); the identity self-test and report; release-shape and its unwind workspace check; the sdk-deletions self-test; every `lint` step except the three x86 lane-guard probes (Rust only; this PR changes no Rust source: no `.rs` or Cargo file differs from 0d326c5c8).
