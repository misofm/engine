# Release @misofm/engine 0.10.4

Filed by root on 2026-10-31 with the owner's approval of release 0.10.4. It ships main 4079ef102 through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.4 on npm. It adds to 0.10.3 (#2215):
- **#2199, #2219:** the browser boot and the replacement pre-check admit the projected graph, builtin and control rows before any allocation. A session that cannot fit is refused with a typed refusal (`host.budget.retained_projection`) before its rows exist. Before, the wasm boot trapped. The boot projection charges no processor for a banked member, so it is under 0.1 % over the exact total.
- **#2213:** new C ABI symbol `miso_engine_v1_plan_prepare_seek` (feature bit 13). A stopped mobile host can free a full ring and refill it before restart. Both render-side calls refuse a concurrent call with `render.busy`.
- **#2198:** render diagnostics coalesced while both slots are full count as `TELEMETRY_COALESCED`.
- **#2173:** a pack group of more than 65,535 strips no longer overflows its sort counters.
- No export is removed. This is a patch release.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2215 did for 0.10.3.
- Re-pin the AudioWorklet module (#2198, #2199, #2219 changed it).
- Check the BLAKE3 module against its pin; re-pin only if its bytes changed.
- Record a fresh two-browser qualification with both legs.
- Set the 0.10.4 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (with an "Upgrading from 0.10.3" note), and docs/RELEASE.md.
- Remove the closed specs of #2173 (both files), #2198, #2199, #2212, #2213, #2215 and #2219.

## Gate

The release PR's `artifact-identity` shows a `RELEASE:` line with every check passing, and all four browser legs pass. Then `qualify`, `publish`, `verify` on main.



## Evidence and decision record (attempt 1, 2026-10-31)

- Worklet module: `b7b9789c902e6265f0bbb5f436d37017540083d76fac8b0bd4fba73a462cb1d0`
  (6,361,806 B; named twin `b3a71480...`, 7,206,813 B), built from this tree (main 4079ef102's
  module inputs) with `nightly-2026-08-20`. It replaces 0.10.3's `9968eed6...`. Main 4079ef102's
  own CI run (41133168098) recorded the same digest (commit status `audioworklet-sha256`), so this
  PR moves no byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and
  test-npm-publish-modes.py's accepted `release_pin` and its "wrong release pin" mutation carry it.
  `--check-pin` passes. A second `--module-only` build from a `git archive` export of the tree at
  another path (own target directory; same `CARGO_HOME` and `RUSTUP_HOME`) gives identical bytes.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit f32b17a00
  --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a short socket path
  `/tmp/qa.XXXXXX/native`), all four legs passed on the first run (no rerun):

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 16.7 s |
  | Chromium | 151.0.7922.34 | worker | pass | 16.6 s |
  | WebKit | 26.5 | single | pass | 43.5 s |
  | WebKit | 26.5 | worker | pass | 34.2 s |

  Every render-allocation count is 0 on every leg, including the SDK rows. `blockingRebuilds` is
  1 for the structural edit on the single legs and 0 on the worker legs. Every live, transport and
  stopped seek row underruns 0 frames on both browsers and both modes. The transport seek's added
  latency is 48.0-50.7 ms (single) and 21.3-34.7 ms (worker) on Chromium, 40.0-42.7 ms (single)
  and 18.7-26.7 ms (worker) on WebKit (WebKit's seek rows are reported, not gated). `results.json`
  (four leg rows, `blake3Qualification` at the same candidate) and `BROWSER_DEPLOYMENT_MATRIX.md`
  record it. The candidate's tree differs from this PR's head only in the qualification records
  and this spec; no module input.
- Release identity: npm-publish.yml's five `0.10.4` literals and the pin; `EXPECTED_BLAKE3_SHA256`
  unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`, which is
  `rust-toolchain.toml`'s `[browser-artifact] channel`. test-npm-publish-modes.py follows (its
  "wrong release version" mutation moves 0.10.4 to 0.10.3) and passes. sdk/package.json and its
  lock read 0.10.4. docs/RELEASE.md names 0.10.4 (#2225) as the last release.
- sdk/README.md: install line and release-record link at 0.10.4. A new "Upgrading from 0.10.3"
  section: #2199/#2219 (the browser boot and the replacement pre-check refuse a session that
  cannot fit with `host.budget.retained_projection` before its rows exist, where 0.10.3 could
  trap), the new C ABI symbol `miso_engine_v1_plan_prepare_seek` (feature bit 13; it and
  `miso_engine_v1_render_f32_planar` refuse a concurrent call with `render.busy`), #2198 (a render
  diagnostic that waits while both slots are full is no longer lost, and each one replaced counts
  in `telemetryCoalesced`) and #2173 (no sort-counter overflow above 65,535 strips in a pack
  group). No export is removed.
- Identity report (`--event push --before 4079ef102`): `ARTIFACT UNCHANGED` against the base's
  recorded digest, reproducible, the RELEASE line with every check passing (pin,
  `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate f32b17a00), and the BLAKE3
  RELEASE line.
- Spec retirement: the specs of closed issues #2173 (both files), #2198, #2199, #2212, #2213,
  #2215 and #2219 leave `.github/ISSUE_SPECS/` at this batch (each `gh issue view` reads CLOSED).
- Local gates: `scripts/check-workspace-policy.sh`, `scripts/test-npm-publish-modes.py`,
  `scripts/sdk-package.sh check` (packs `@misofm/engine@0.10.4`), `scripts/check-sdk-generated.sh`,
  and the identity self-test and report all pass. This PR changes no Rust source.
