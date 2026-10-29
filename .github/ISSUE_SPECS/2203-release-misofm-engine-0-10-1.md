# Release @misofm/engine 0.10.1

Filed by root on 2026-10-29. It ships main 723595ed5 through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.1 on npm. It adds to 0.10.0 (#2196):
- **#2194:** effect state is projected against the memory budget before allocation, on boot, replace and rebuild. A session whose effects cannot fit is refused (`host.effect.resource.limit` / `host.budget.retained_projection` / `control.resource.replacement_peak`) before memory grows. A replace whose parse cannot fit beside the live session is `refusedBudget` (`host.budget.parse_projection`, naming `live_retained_bytes`); it was `bufferTooSmall`.
- **#2189:** browser live controls an app holds follow a rebuild that adds or removes a strip. Index addressing stays valid on each strip whose racks the rebuild did not change, and is refused on a strip whose racks changed.
- **#2193:** the live queue's send maps a raw `miso.error.v1` host rejection to `MisoEngineError`; a sticky failed host is `wrongState` (terminal), not `backpressure`.
- No export is removed, so this is a patch release.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2196 did for 0.10.0.
- Re-pin the AudioWorklet module (#2194 changed it): `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Check the BLAKE3 module against its pin; re-pin only if its bytes changed.
- Record a fresh two-browser qualification with both legs (`--browser all --leg all --record-matrix --candidate-commit <sha> --self-test-mutations`).
- Set the 0.10.1 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, and sdk/README.md.
- Remove the closed specs of #2189 and #2193.

## Gate

The release PR's `artifact-identity` shows a `RELEASE:` line with every check passing, and all four browser legs pass. Then `qualify`, `publish`, `verify` on main.

## Evidence and decision record (attempt 1, 2026-10-29)

- Worklet module: `e65c15954016344bd4c36b7e4c2984ea8b96f6b79ea8a2799ec6fcc677c9413d`
  (6,172,395 B; named twin `35154710...`, 6,986,320 B), built from this tree (main 723595ed5's
  module inputs) with `nightly-2026-08-20`. It replaces 0.10.0's `11f93b33...`. Main 723595ed5's
  own CI run (40844354573) recorded the same digest (commit status `audioworklet-sha256`), so this
  PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and
  test-npm-publish-modes.py's accepted `release_pin` and its "wrong release pin" mutation carry it.
  `--check-pin` passes. A second `--module-only` build from a `git archive` export of the tree at
  another path (own target directory; same `CARGO_HOME` and `RUSTUP_HOME`) gives identical bytes.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 306d52e3a
  --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a 21 B socket path
  `/tmp/qa.XXXXXX/native`), all four legs passed on the first run:

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 17.2 s |
  | Chromium | 151.0.7922.34 | worker | pass | 17.0 s |
  | WebKit | 26.5 | single | pass | 41.3 s |
  | WebKit | 26.5 | worker | pass | 33.7 s |

  Every render-allocation count is 0 on every leg, including the SDK rows. `blockingRebuilds` is
  1 for the structural edit on the single legs and 0 on the worker legs. `results.json` (four leg
  rows, `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md` record it.
  The candidate's tree differs from this PR's head only in the qualification records and this
  spec; no module input.
- Release identity: npm-publish.yml's five `0.10.1` literals and the pin; `EXPECTED_BLAKE3_SHA256`
  unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`, which is
  `rust-toolchain.toml`'s `[browser-artifact] channel`. test-npm-publish-modes.py follows (its
  "wrong release version" mutation moves 0.10.1 to 0.10.0) and passes. sdk/package.json and its
  lock read 0.10.1. docs/RELEASE.md names 0.10.1 (#2203) as the last release.
- sdk/README.md: install line and release-record link at 0.10.1. A new "Upgrading from 0.10.0"
  section: #2194 (effect state projected before allocation; a replace whose parse cannot fit
  beside the live session is `refusedBudget`, `host.budget.parse_projection`, was
  `bufferTooSmall`), #2189 (browser live controls follow a rebuild that moves strips) and #2193
  (a host-refused live-control frame is `MisoEngineError`; a failed host is `wrongState`, not
  `backpressure`). No export is removed.
- Identity report (`--event push --before 723595ed5`): `ARTIFACT UNCHANGED` against the base's
  recorded digest, reproducible, the RELEASE line with every check passing (pin,
  `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate 306d52e3a), and the BLAKE3
  RELEASE line.
- Spec retirement: the specs of closed issues #2189 and #2193 leave `.github/ISSUE_SPECS/` at
  this batch (each `gh issue view` reads CLOSED).
- Local gates: `scripts/check-workspace-policy.sh`, `scripts/test-npm-publish-modes.py`,
  `scripts/sdk-package.sh check` (packs `@misofm/engine@0.10.1`), `scripts/check-sdk-generated.sh`,
  and the identity self-test and report all pass. This PR changes no Rust source.
