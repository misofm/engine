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
