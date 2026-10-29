# Release @misofm/engine 0.10.2

Filed by root on 2026-10-29. It ships main 2b0a64d88 through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.2 on npm. It adds to 0.10.1 (#2203):
- **#2201:** every browser host call goes through one mapped path, so the app receives only `MisoEngineError` (or the SDK's typed refusals) from the browser engine. `createEngine()` maps a raw boot refusal to `MisoEngineError` (the raw record is its `cause`); it was the raw record. A failed host's sticky result 6 is `wrongState` on every path.
- **#2206:** a rebuild-path `apply` follows the strip order with a headers-only control-list read (`control_list_request` field 6, `headers_only`, an in-place V1 amendment). A bus add's apply at 64 tracks drops from about 200 ms to about 40 ms in the SDK probe.
- **#2184:** the MSB1 seek is a parity seqlock on SEEK_EPOCH (it steps by 2 per seek; odd means a publication is in flight).
- **#2185 (single mode):** a live plain ring seek waits for its first new-generation chunks (bounded at 50 ms) instead of cutting to silence. Worker mode stays open in #2185.
- No export is removed. `MisoEngineErrorInit` gains an optional `cause`. This is a patch release.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2203 did for 0.10.1.
- Re-pin the AudioWorklet module (#2206 changed it).
- Check the BLAKE3 module against its pin; re-pin only if its bytes changed.
- Record a fresh two-browser qualification with both legs.
- Set the 0.10.2 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (with an "Upgrading from 0.10.1" note: `createEngine()` boot refusals are now `MisoEngineError`), and docs/RELEASE.md.
- Remove the closed specs of #2201 and #2206.

## Gate

The release PR's `artifact-identity` shows a `RELEASE:` line with every check passing, and all four browser legs pass. Then `qualify`, `publish`, `verify` on main.

## Evidence and decision record (attempt 1, 2026-10-29)

- Worklet module: `f613f2860e84d4c61b381abdd167876297847911a762c83b7dd0e63ad4d62e9c`
  (6,173,299 B; named twin `a685e9d0...`, 6,987,224 B), built from this tree (main 2b0a64d88's
  module inputs) with `nightly-2026-08-20`. It replaces 0.10.1's `e65c1595...`. Main 2b0a64d88's
  own CI run (40885379582) recorded the same digest (commit status `audioworklet-sha256`), so this
  PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and
  test-npm-publish-modes.py's accepted `release_pin` and its "wrong release pin" mutation carry it.
  `--check-pin` passes. A second `--module-only` build from a `git archive` export of the tree at
  another path (own target directory; same `CARGO_HOME` and `RUSTUP_HOME`) gives identical bytes.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 7ae5b25ce
  --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a 21 B socket path
  `/tmp/qa.XXXXXX/native`), all four legs passed on the first run (no #2198 rerun was needed):

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 16.2 s |
  | Chromium | 151.0.7922.34 | worker | pass | 16.6 s |
  | WebKit | 26.5 | single | pass | 41.1 s |
  | WebKit | 26.5 | worker | pass | 32.9 s |

  Every render-allocation count is 0 on every leg, including the SDK rows. `blockingRebuilds` is
  1 for the structural edit on the single legs and 0 on the worker legs. The worker-mode live seek
  row is reported, not gated (the open #2185 worker residual). `results.json` (four leg rows,
  `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md` record it. The
  candidate's tree differs from this PR's head only in the qualification records and this spec; no
  module input.
- Release identity: npm-publish.yml's five `0.10.2` literals and the pin; `EXPECTED_BLAKE3_SHA256`
  unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`, which is
  `rust-toolchain.toml`'s `[browser-artifact] channel`. test-npm-publish-modes.py follows (its
  "wrong release version" mutation moves 0.10.2 to 0.10.1) and passes. sdk/package.json and its
  lock read 0.10.2. docs/RELEASE.md names 0.10.2 (#2210) as the last release.
- sdk/README.md: install line and release-record link at 0.10.2. A new "Upgrading from 0.10.1"
  section: #2201 (`createEngine()` boot refusals are `MisoEngineError`, phase `boot`, with the raw
  record as `cause`, were the raw record; every host call is mapped; `MisoEngineErrorInit` gains
  `cause`), #2206 (`ControlListRequest.headersOnly`, `control_list_request` field 6
  `headers_only`; a bus add's `apply` at 64 tracks about 40 ms instead of about 200 ms), #2184
  (the seek epoch is a parity seqlock and steps by 2 per seek) and #2185 (in `single` mode a live
  plain ring seek waits up to 50 ms for the new generation instead of cutting to silence; worker
  mode stays open). No export is removed.
- Identity report (`--event push --before 2b0a64d88`): `ARTIFACT UNCHANGED` against the base's
  recorded digest, reproducible, the RELEASE line with every check passing (pin,
  `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate 7ae5b25ce), and the BLAKE3
  RELEASE line.
- Spec retirement: the specs of closed issues #2201 and #2206 leave `.github/ISSUE_SPECS/` at
  this batch (each `gh issue view` reads CLOSED).
- Local gates: `scripts/check-workspace-policy.sh`, `scripts/test-npm-publish-modes.py`,
  `scripts/sdk-package.sh check` (packs `@misofm/engine@0.10.2`), `scripts/check-sdk-generated.sh`,
  and the identity self-test and report all pass. This PR changes no Rust source.
