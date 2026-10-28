# Release @misofm/engine 0.9.0

Filed by root for the browser worker mode on 2026-10-28. The owner approved shipping worker mode in one 0.9.0 release on 2026-10-27, through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.9.0 on npm. It adds worker mode to 0.8.1 (merge 0512957d5):
- **#1332 (S3-S9: #2174, #2176, #2178, #2179, #2180, #2181, #2182, #2183):** on a cross-origin-isolated page the control plane runs in a dedicated Worker on one shared `WebAssembly.Memory`, and the AudioWorklet only renders. Structural edits no longer block the audio thread (`blockingRebuilds` 0). New artifact `miso-engine-v1-control-worker.js` (the artifact set is eight files). New SDK surface: `instanceMode` on the host and the engine, the explicit `instanceMode` option, the status counter `meterFramesSkipped`, the `MisoTeardown` record of the bounded dispose (5 s page bound, 2 s release bound), `BrowserBootError` `worker-failed` (`MisoWorkerFailed`). Each worker-mode control reply waits for the control-reply fence (up to about one quantum). A page without isolation keeps `single` mode.
- **#1380 (#2177):** the module imports one shared memory (`env.memory`, at most 512 MiB) at every instantiation site; a boot budget above it is refused with `web.boot.memory_maximum`.
- **#1387 (#2186):** the PCM feed drains in the control Worker in worker mode (`PcmFeedError` `attachRefused` for a feed without its `engine`); the worker-mode default source ring is 65 536 frames.
- **#2175:** AGENTS.md text only.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`). iOS Safari is not qualified (#1331 closed).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2170 did for 0.8.1. Check git history for `.github/ISSUE_SPECS/2170-release-misofm-engine-0-8-1.md` (658174b9a).
- Re-pin the AudioWorklet module (it changed: atomics, the imported shared memory and the host split). Use `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Check the BLAKE3 module against its pin and `EXPECTED_BLAKE3_SHA256`; re-pin only if its bytes changed.
- Make sure the release identity covers the eighth artifact file.
- Record a fresh two-browser qualification with both legs: `--browser all --leg all --record-matrix --candidate-commit <sha> --self-test-mutations`. This is the first release that records per-leg rows: four rows. It also records `results.json.blake3Qualification`. Firefox is out of scope (owner, 2026-10-25).
- Set the 0.9.0 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (install line, release-record link, "Upgrading from 0.8.1"), docs/RELEASE.md and hosts/host-web/DEPLOYMENT.md (the modes).
- Run the identity report. This PR changes no Rust source.

## Release window

Between this PR's merge and the npm publish, root merges no engine PR that can change the browser module (see the memory rule "release window: hold module merges").

## Gates

- Every release-PR gate named in docs/RELEASE.md.
- The qualification record and the identity report: every check passes.
- After the merge: npm-publish.yml qualify (expected_sha = main head), then publish (qualification_run_id), then verify mode. `npm view @misofm/engine version` must read 0.9.0.

## Evidence and decision record (attempt 1, 2026-10-28)

- Worklet module: `b32a5429f80510e0957b9f6120d7ea0f0e9ad3f8e0fc4d38bb898b0b16553553` (6,158,159 B; named twin `f7c0f4b5...`, 6,968,595 B), built from this tree (main f2518a5a3's module inputs) with `nightly-2026-08-20`. It replaces 0.8.1's `df4a8800...`. Main f2518a5a3's own CI run (40653173600) recorded the same digest, so this PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and test-npm-publish-modes.py's accepted `release_pin` and its "wrong release pin" mutation carry it. `--check-pin` passes. A second build from a `git archive` export of the tree at another path gives identical bytes for all eight files of the closure.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B), unchanged since 0.6.0. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Artifact set: eight files (`miso-engine-v1-control-worker.js` added by #1332 S7). The slices already carried the eighth file through every consumer: `build-web-audioworklet.sh`, `check-web-audioworklet.sh` (exact eight frozen outputs), `sdk/codegen/generate.mjs` `ARTIFACT_SET` (the SDK manifest), `sdk/src/assets.ts` `controlWorkerModule`, `sdk/test/package-tarball-smoke.mjs` and the qualification server. npm-publish.yml names no file count: it builds through `build-web-audioworklet.sh` and stages and smokes through `sdk-package.sh` and the package smoke, so it needed no change; `web-audioworklet-identity.py` checks the module and BLAKE3 digests only, also no change. Prose that still said seven is fixed: sdk/README.md (the SDK manifest paragraph) and hosts/host-web/DEPLOYMENT.md.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 3bf0793e5 --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a short scratch socket path, 102 B): four rows, all pass. Chromium 151.0.7922.34 single (15.6 s) and worker (15.4 s); WebKit 26.5 single (36.1 s) and worker (27.7 s). The single legs carry the `simd128` BLAKE3 Worker row and the suspended-context feed row. Every render-allocation count is 0 on every leg; `blockingRebuilds` is 1 for the structural replace on the single legs and 0 on the worker legs. `results.json` (four leg rows, `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md` record it. The candidate's tree differs from this PR's head only in the qualification records and this spec; no module input.
- Release identity: npm-publish.yml's five `0.9.0` literals and the pin; `EXPECTED_BLAKE3_SHA256` unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20` (`rust-toolchain.toml` `[browser-artifact]`). test-npm-publish-modes.py follows (its "wrong release version" mutation moves 0.9.0 to 0.8.1) and passes. sdk/package.json and its lock read 0.9.0. docs/RELEASE.md names 0.9.0 (#2187) as the last release, and its step 3 now gives `--leg all` (S9 made `--record-matrix` require it) and the four rows. Release record: this issue.
- Identity report (`--event push --before f2518a5a3`): `ARTIFACT UNCHANGED` against the base's recorded digest, reproducible, the RELEASE line with every check passing (pin, `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate 3bf0793e5), and the BLAKE3 RELEASE line.
- sdk/README.md: install line and release-record link at 0.9.0. "Upgrading from 0.8.1" names worker mode and its isolation requirement (COOP `same-origin`, COEP `require-corp`), the control Worker artifact, `instanceMode` (engine, host and option; a custom `createHost` must forward it), `meterFramesSkipped`, `staleViewQuanta` always 0, `worker-failed`, `attachRefused`, the 65 536-frame worker-mode default ring, the bounded dispose (`MisoTeardown`, 5 s and 2 s), the control-reply fence (up to about one quantum), the imported shared memory and `web.boot.memory_maximum`; non-isolated pages keep single mode. "Play in the browser" and the rebuild paragraph name both modes. hosts/host-web/DEPLOYMENT.md describes both modes and that the matrix qualifies both.
- Local gates (`rel090-gates.log`), all 62 steps pass, the same set as #2170: the route self-tests; docs-gates; the `artifact` build (with named twin) and producers check for both modules; the `sdk` job with an empty private TMPDIR and the source-only asset tests; `artifact-gates` (named-twin strip check, check-web-audioworklet.sh, check-browser-expected-resources, scalar-oracle-absent, test-web-audioworklet.sh, render-lock probe, V8 spill on Node 22.23.2); the identity self-test and report; release-shape and its unwind workspace check; the sdk-deletions self-test; fmt, clippy, doc and every `lint` step except the three x86 lane-guard probes (Rust only; this PR changes no Rust source: no `.rs` or Cargo file differs from f2518a5a3).
