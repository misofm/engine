# Release @misofm/engine 0.8.1

Filed by root for the console boot spectrum fix on 2026-10-27. The owner approved this release on 2026-10-27, through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.8.1 on npm. It adds one change to 0.8.0 (merge 3374b21a0):
- **#2166 (#2168):** the boot spectrum taps are part of the first plan, so `createEngine({ spectrum })` and `createEngine({ spectrumCollection })` count no blocking rebuild at boot; the SDK confirms the taps after boot. The `createHost` request gains an optional `bootCommand`. A custom `createHost` that drops it now makes `createEngine` fail with `sdk.boot.command`. A boot spectrum configuration that the engine refuses now fails the boot itself, so the error's `code` is the boot refusal (`refusedDocument` or `refusedBudget`) and the diagnostic is the spectrum code.

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2164 did for 0.8.0. Check git history for `.github/ISSUE_SPECS/2164-release-misofm-engine-0-8-0.md` (706eb9be6).
- Re-pin the AudioWorklet module (it changed: the boot command frame). Use `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Check the BLAKE3 module against its pin and `EXPECTED_BLAKE3_SHA256` (docs/RELEASE.md, "BLAKE3 release identity"); re-pin only if its bytes changed.
- Record a fresh two-browser qualification, Chromium and WebKit, with `--browser all --record-matrix --candidate-commit <sha> --self-test-mutations`. It also records `results.json.blake3Qualification`. Firefox is out of scope (owner, 2026-10-25).
- Set the 0.8.1 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (install line and release-record link) and docs/RELEASE.md.
- sdk/README.md "Upgrading from 0.8.0" came with #2166; it must name every behaviour change.
- Run the identity report. This PR changes no Rust source.

## Release window

Between this PR's merge and the npm publish, root merges no engine PR that can change the browser module (see the memory rule "release window: hold module merges").

## Gates

- Every release-PR gate named in docs/RELEASE.md.
- The qualification record and the identity report: every check passes.
- After the merge: npm-publish.yml qualify (expected_sha = main head), then publish (qualification_run_id), then verify mode. `npm view @misofm/engine version` must read 0.8.1.

## Evidence and decision record (attempt 1, 2026-10-27)

- Worklet module: `18350855c70e5a9e75d92ef1932f227bf71e1768be77a38452ff62a45084b879` (6,122,897 B; named twin `9cd10889...`, 6,928,649 B), built from this tree with `nightly-2026-08-20`. It replaces 0.8.0's `7af24e63...`. Main 9eb357500's own CI run (40479371230) recorded the same digest, so this PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256` and test-npm-publish-modes.py's accepted pin carry it. `--check-pin` passes. A second build from a `git archive` export of this tree at another path gives the same bytes for every file of the closure (same `CARGO_HOME`; the identity report's twin input).
- BLAKE3 module: `miso-engine-v1-blake3.simd128.wasm`, `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B), unchanged since 0.6.0. It equals its pin (`hosts/host-web/web/miso-engine-v1-blake3-artifact.sha256`, #2117) and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --record-matrix --candidate-commit 55f4841c5 --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a `/tmp/qa.XXXXXX` socket): Chromium 151.0.7922.34 pass, WebKit 26.5 pass, each with the `simd128` BLAKE3 Worker row and the suspended-context feed row. Every render-allocation count is 0. `results.json` (with `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md` record it. The candidate's tree differs from the published head only in this spec and the qualification records; no module input.
- Release identity: npm-publish.yml's five `0.8.1` literals and the pin; `EXPECTED_BLAKE3_SHA256` unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20` (`rust-toolchain.toml` `[browser-artifact]`). test-npm-publish-modes.py follows (its "wrong release version" mutation moves 0.8.1 to 0.8.0) and passes. sdk/package.json and its lock read 0.8.1. docs/RELEASE.md names 0.8.1 (#2170) as the last release. Release record: this issue.
- Identity report (`--event push --before 9eb357500`): `ARTIFACT UNCHANGED` against the base's recorded digest, reproducible, the RELEASE line with every check passing, and the BLAKE3 RELEASE line (pin, `EXPECTED_BLAKE3_SHA256` and both browsers' Worker row agree with the built bytes).
- sdk/README.md: install line and release-record link at 0.8.1. "Upgrading from 0.8.0" (from #2166) names the fixed boot rebuild count, the added optional `bootCommand`, the custom-`createHost` failure (`sdk.boot.command`) and the boot refusal's new error `code`.
- Local gates, all pass: the route self-tests; docs-gates; the `artifact` build (with named twin) and producers check for both modules; the `sdk` job (check-sdk-generated, check-sdk-deletions, check-sdk-types, check-sdk-headless, sdk-package.sh check, with an empty private TMPDIR at the end; the source-only asset tests); `artifact-gates` (named-twin strip check, check-web-audioworklet.sh, check-browser-expected-resources, scalar-oracle-absent, test-web-audioworklet.sh with a private TMPDIR, render-lock probe, V8 spill on Node 22.23.2); the identity self-test and report; release-shape and its unwind workspace check; the sdk-deletions self-test; every `lint` step except the three x86 lane-guard probes (Rust only; this PR changes no Rust source: no `.rs` or Cargo file differs from 9eb357500).
