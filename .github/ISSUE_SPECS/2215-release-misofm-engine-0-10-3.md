# Release @misofm/engine 0.10.3

Filed by root on 2026-10-30. It ships main 7ef7567bd through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.3 on npm. It adds to 0.10.2 (#2210):
- **#2205 / #2185 (owner decision 2026-10-29):** a live seek (transport `engine.seek` and per-source ring seeks, both modes) keeps the old position until the new generation has its runway in the engine ring, bounded by about 50 ms, then cuts once to the target from its first sample; a session seek lands every source and the timeline in the same block. A seek with no live audio to keep (headless, or a stopped or suspended browser engine) lands at once, and a stopped `engine.seek` prepares the linked feeds itself.
- New browser exports `miso_engine_web_v1_source_prepare_seek` and `miso_engine_web_v1_session_prepare_seek`; `source_seek` no longer prepares the consumer.
- New C ABI symbol `miso_engine_v1_session_seek_at_once` (feature bit 12) for a stopped mobile host.
- The JS seek deferral of 0.10.2 is removed; `SEEK_EPOCH` still steps by 2 (#2184).
- No export is removed. This is a patch release.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2210 did for 0.10.2.
- Re-pin the AudioWorklet module (#2214 changed it).
- Check the BLAKE3 module against its pin; re-pin only if its bytes changed.
- Record a fresh two-browser qualification with both legs.
- Set the 0.10.3 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (with an "Upgrading from 0.10.2" note), and docs/RELEASE.md.
- Remove the closed specs of #2185 and #2205.

## Gate

The release PR's `artifact-identity` shows a `RELEASE:` line with every check passing, and all four browser legs pass. Then `qualify`, `publish`, `verify` on main.

## Evidence and decision record (attempt 1, 2026-10-30)

- Worklet module: `9968eed6c9f4188cd12c2142e9426974151caf516dadd4ff16688a16f3fb1726`
  (6,178,281 B; named twin `d83f3749...`, 6,993,013 B), built from this tree (main 7ef7567bd's
  module inputs) with `nightly-2026-08-20`. It replaces 0.10.2's `f613f286...`. Main 7ef7567bd's
  own CI run (41017581109) recorded the same digest (commit status `audioworklet-sha256`), so this
  PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and
  test-npm-publish-modes.py's accepted `release_pin` and its "wrong release pin" mutation carry it.
  `--check-pin` passes. A second `--module-only` build from a `git archive` export of the tree at
  another path (own target directory; same `CARGO_HOME` and `RUSTUP_HOME`) gives identical bytes.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 0b5b42d73
  --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a short socket path
  `/tmp/qa.XXXXXX/native`), all four legs passed on the first run (no #2198 rerun was needed):

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 15.0 s |
  | Chromium | 151.0.7922.34 | worker | pass | 15.4 s |
  | WebKit | 26.5 | single | pass | 31.1 s |
  | WebKit | 26.5 | worker | pass | 27.0 s |

  Every render-allocation count is 0 on every leg, including the SDK rows. `blockingRebuilds` is
  1 for the structural edit on the single legs and 0 on the worker legs. Every live, transport and
  stopped seek row underruns 0 frames on both browsers and both modes. The transport seek's added
  latency is 48.0 ms (single) and 21.3-34.7 ms (worker) on Chromium, 40.0-48.0 ms (single) and
  18.7-24.0 ms (worker) on WebKit (WebKit's seek rows are reported, not gated). `results.json`
  (four leg rows, `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md`
  record it. The candidate's tree differs from this PR's head only in the qualification records
  and this spec; no module input.
- Release identity: npm-publish.yml's five `0.10.3` literals and the pin; `EXPECTED_BLAKE3_SHA256`
  unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`, which is
  `rust-toolchain.toml`'s `[browser-artifact] channel`. test-npm-publish-modes.py follows (its
  "wrong release version" mutation moves 0.10.3 to 0.10.2) and passes. sdk/package.json and its
  lock read 0.10.3. docs/RELEASE.md names 0.10.3 (#2215) as the last release.
- sdk/README.md: install line and release-record link at 0.10.3. A new "Upgrading from 0.10.2"
  section: #2205/#2185 (a live seek keeps the old audio up to about 50 ms, then cuts once, in both
  modes; a transport seek lands every source and the timeline in one block; stopped and headless
  seeks land at once; a stopped `engine.seek` runs the linked feeds' `prepareSeek()` itself, so an
  app can drop its own call before `play()`), the two new browser exports
  `miso_engine_web_v1_source_prepare_seek` and `miso_engine_web_v1_session_prepare_seek` (and
  `source_seek` no longer prepares), and the new C ABI symbol
  `miso_engine_v1_session_seek_at_once` (feature bit 12). No export is removed.
- Identity report (`--event push --before 7ef7567bd`): `ARTIFACT UNCHANGED` against the base's
  recorded digest, reproducible, the RELEASE line with every check passing (pin,
  `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate 0b5b42d73), and the BLAKE3
  RELEASE line.
- Spec retirement: the specs of closed issues #2185 and #2205 leave `.github/ISSUE_SPECS/` at
  this batch (each `gh issue view` reads CLOSED).
- Local gates: `scripts/check-workspace-policy.sh`, `scripts/test-npm-publish-modes.py`,
  `scripts/sdk-package.sh check` (packs `@misofm/engine@0.10.3`), `scripts/check-sdk-generated.sh`,
  and the identity self-test and report all pass. This PR changes no Rust source.
