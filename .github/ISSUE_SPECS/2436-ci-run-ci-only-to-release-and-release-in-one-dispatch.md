# CI: run CI only to release, and release in one dispatch

Owner request (2026-11-21): one change costs about 1.5 hours of CI before it is on npm: the fix PR's qualification, the release PR's, the merge push's, then `npm-publish.yml`'s `qualify`, `publish` and `verify`. CI should run only when something is released, and a release should take minutes.

## Contract
- No workflow runs on `pull_request`, `push` or `schedule`. `qualification.yml`, `nightly.yml` and `fuzz.yml` keep every job and script and become `workflow_dispatch` only. `main` requires no status check (`qualification` is dropped from branch protection once this merges).
- `npm-publish.yml` (its filename stays: it is npm's trusted publisher) is one dispatch with inputs `version` (`patch`, `minor`, `major` or `X.Y.Z` above npm's `latest`) and `dry_run`. One run builds the shipped closure once; `package` (generated surface, types, headless evals, package build, one tarball carrying the built modules, smoke, dry-run publish), `artifact-gates` (export set, allocation-free render closure, native/simd128 parity, scalar oracle absent, hermetic host tests, V8 spill), the four browser legs (Chromium and WebKit, `single` and `worker`) and `core-tests` (`host-web`, `session`, `protocol`) run in parallel on those bytes; `publish` sends that tested tarball with provenance and verifies the registry and the provenance; `record` creates the GitHub Release `v<version>` with the digests and attaches the tarball and named twin.
- No release PR: `sdk/package.json` carries `0.0.0-development` and the release stamps its version; the tag names the source commit. Pins and `results.json` are no longer re-pinned by a release.
- Only `publish` holds `id-token: write`, and only publishes from `main`; only `record` holds `contents: write`, and it runs no repository code. No npm token is read.
- `docs/RELEASE.md` and `AGENTS.md` describe the new flow.

## Gates
- `scripts/test-npm-publish-modes.py` holds the new workflow's permission layout and its version, publish and provenance rules (rewritten for it).
- `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py`, `scripts/check-script-reachability.py`, `scripts/check-workspace-policy.sh`, `scripts/check-test-support-ci.py` pass.
- A `dry_run=true` dispatch on `main` passes every job; its wall time is recorded below.

## Non-goals
- Deleting a gate script or test suite.
- Changing npm's trusted-publisher configuration.

## Evidence / attempt record
(empty)
