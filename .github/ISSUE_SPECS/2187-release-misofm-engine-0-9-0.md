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

Pending.
