# Release @misofm/engine 0.5.0

Filed by root for the console sprint (2026-10-26). The owner approved publishing 0.5.0 when its release PR is green, through the manual qualify-then-publish workflow.

## Product outcome
`@misofm/engine` 0.5.0 on npm matches today's engine: the module, the worklet and every SDK API since 0.4.3, including #2100 (pcmFeed, position, play/stop), #2101 (identity-matrix default) and #2102 (WAV reader, PCM pump, BLAKE3 identity). The console app depends on it from npm instead of a vendored tarball.

## Smallest slice (docs/RELEASE.md, "The release PR")
1. Build the module and re-pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`.
2. Record a fresh two-browser qualification (Chromium and WebKit) with `--record-matrix --candidate-commit`.
3. Set the release identity: npm-publish.yml (PACKAGE_VERSION and the five version literals, EXPECTED_WORKLET_SHA256, toolchain checks), scripts/test-npm-publish-modes.py, sdk/package.json, sdk/package-lock.json, the README install line and release-record link.
4. A migration note "Upgrading from 0.4.3" in sdk/README.md: the removed and renamed exports, console -> liveControls, submitCommands -> transactions, the observation helpers -> handles, the session-type renames, the retired boot words, the identity-matrix default, and the new APIs.
5. The PR changes no Rust source. Its artifact-identity summary shows a passing `RELEASE:` line, and both browser legs pass.
6. After the merge: run npm-publish.yml `qualify` with `expected_sha` = the merge commit, then `publish` with that run's ID. Verify the published package (version, integrity, provenance).

## Gates
The release checks in CI, the two-browser record, `scripts/test-npm-publish-modes.py`, `sdk-package.sh check`, and the post-publish `verify`.

## Evidence and decision record (attempt 1, 2026-10-26)
- Module: `049a07f3fef5938a20937df03c4c80f3e4506158d378dda3e5985f0a4f5040bd` (5763203 B; named twin `b2385209...` 6526337 B), built from c8f73409c (main e4421343c plus this spec) with `nightly-2026-08-20`. `--check-pin` passes. A second build from another checkout path gives the same bytes. Main's own CI record for e4421343c is the same digest (identity report: ARTIFACT UNCHANGED), so the release pins the module `main` already builds.
- Qualification (`--browser all --record-matrix --candidate-commit c8f73409c --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink): Chromium 151.0.7922.34 pass, WebKit 26.5 pass; every render-allocation row 0.
- Release identity: npm-publish.yml's five `0.5.0` literals and `EXPECTED_WORKLET_SHA256`; the toolchains already match (`RUSTUP_TOOLCHAIN` 1.99.1, `BROWSER_ARTIFACT_TOOLCHAIN` nightly-2026-08-20), no edit. test-npm-publish-modes.py follows; its "wrong release version" mutation now moves 0.5.0 to 0.4.3. Release record: this issue.
- `web-audioworklet-identity.py report --event push --before e4421343c`: RELEASE line, every check passes.
- sdk/README.md "Upgrading from 0.4.3" is built from the export diff of the four entry points (TypeScript checker) between 172964aed and HEAD.
- Open: docs/RELEASE.md "Between releases" still says the pin describes a pre-#1061 re-pin; after this PR merges that paragraph is stale (outside this PR's paths).
