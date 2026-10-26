# Release @misofm/engine 0.7.0

Filed by root for the console sprint on 2026-10-26. The owner approved publishing SDK releases when their release PR is green, through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.7.0 on npm. It adds these changes to 0.6.0:
- **#2115:** `subscribeLoudness()` on both engines. It reports momentary, short-term, integrated, range and true peak, using capability bit 17 and the LOUDNESS command.
- **#2114:** the engine reverb, `miso.reverb`, a 32-line FDN. The README has an "Aux reverb return" recipe.
- **#2129 S1b (#2150):** a bypassed bank skips its wet path. Every un-bypass starts from rest: the effect holds dry for its warm-up W, then fades in. The console at load uses about 79 % less effect CPU.
- **Banking:** #1981 (packing), #2131 (`reset_lane`), #2133 (silence as a propagated fact), and the counters from #2132 and #2143.

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2121 did for 0.6.0. Check git history for `.github/ISSUE_SPECS/2121-release-misofm-engine-0-6-0.md`.
- Re-pin the AudioWorklet module (it changed: loudness S2, the reverb and the bypass skip). Use `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Record a fresh two-browser qualification, Chromium and WebKit, with `--browser all --record-matrix --candidate-commit <sha> --self-test-mutations`. Firefox is out of scope (owner, 2026-10-25).
- Set the 0.7.0 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, and docs/RELEASE.md.
- Add sdk/README.md "Upgrading from 0.6.0". It must name the un-bypass-from-rest behaviour change.
- Run the identity report.

## Release window

Between this PR's merge and the npm publish, root merges no engine PR that can change the browser module (see the memory rule "release window: hold module merges").

## Gates

- Every release-PR gate named in docs/RELEASE.md.
- The qualification record and the identity report: every check passes.
- After the merge: npm-publish.yml qualify (expected_sha = main head), then publish (qualification_run_id), then verify mode. `npm view @misofm/engine version` must read 0.7.0.

## Evidence and decision record (attempt 1, 2026-10-26)

- Worklet module: `aa7578f99e0f2612203d2a40d362819fc52106593997a1a41b4eac838e181b67` (5,880,530 B; named twin `1a0ecc8d...`, 6,659,374 B), built from this tree with `nightly-2026-08-20`. It replaces 0.6.0's `455e771c...`. Main 16f548a92's own CI run (40369917549) recorded the same digest, so this PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256` and test-npm-publish-modes.py's accepted pin carry it. `--check-pin` passes. A second build from another checkout path and `CARGO_HOME` gives the same bytes.
- BLAKE3 module: `miso-engine-v1-blake3.simd128.wasm`, `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B), unchanged from 0.6.0. It still has no own release pin (#2117).
- Qualification (`--browser all --record-matrix --candidate-commit 00ee2899f --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink): Chromium 151.0.7922.34 pass, WebKit 26.5 pass. `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md` record it. Each browser leg with `--check-matrix --self-test-mutations` then passes on the CI-layout artifact.
- Release identity: npm-publish.yml's five `0.7.0` literals and the pin; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`. test-npm-publish-modes.py follows (its "wrong release version" mutation moves 0.7.0 to 0.6.0). sdk/package.json and its lock read 0.7.0. docs/RELEASE.md names 0.7.0 (#2152) as the last release. Release record: this issue.
- Identity report (`--event push --before 16f548a92`): `ARTIFACT UNCHANGED` against the base's recorded digest, reproducible, and the RELEASE line with every check passing.
- sdk/README.md: install line and release-record link at 0.7.0; "Upgrading from 0.6.0" names `subscribeLoudness()`, `miso.reverb` with its aux-return recipe, and the un-bypass behaviour change (from rest, dry for the warm-up, then fade in).
- Local gates, all pass: the route self-tests; docs-gates; the `artifact` build and producers check; the `sdk` job (check-sdk-generated, check-sdk-deletions, check-sdk-types, check-sdk-headless, sdk-package.sh check, with an empty private TMPDIR at the end; the source-only asset tests); `artifact-gates` (named-twin strip check, check-web-audioworklet.sh, check-browser-expected-resources, scalar-oracle-absent, test-web-audioworklet.sh, render-lock probe, V8 spill); the identity self-test and report; release-shape; the sdk-deletions self-test (the route's one self-test); every `lint` step except the three x86 lane-guard probes (Rust only; this PR changes no Rust source).
