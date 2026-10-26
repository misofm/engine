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
