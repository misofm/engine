# Release @misofm/engine 0.10.5

Filed by root on 2026-11-21 with the owner's approval of a release ("Yes please get the new mixer up"). It ships main 0e8404ac4 through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.5 on npm. It adds to 0.10.4 (#2225):
- **#2431:** a boot budget (`policy.maximumMemoryBytes`) caps the shared `WebAssembly.Memory` of the scratch boot and the worklet boot, and `createResponsePreview({ maximumMemoryBytes })` caps the preview Worker's. Each declares the initial pages, the budget plus an eighth, and 35 pages, not 512 MiB. WebKit reserves a shared memory's whole maximum up front, so an iPhone page with two or three engine memories ran out of address space at `createEngine`.
- Everything merged since 0.10.4 (#2229 to #2427): web-host trap containment (#2256, #2291, #2370, #2371, #2382), stem-store deadlines (#2372, #2381, #2396, #2397, #2405), protocol counter `bankContractDefects` (23, #2427), and DSP and graph fixes.
- No export is removed. This is a patch release.
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2225 did for 0.10.4.
- Re-pin the AudioWorklet module (main moved it since 0.10.4).
- Check the BLAKE3 module against its pin; re-pin only if its bytes changed.
- Record a fresh two-browser qualification with both legs.
- Set the 0.10.5 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and its lock, sdk/README.md (with an "Upgrading from 0.10.4" note), and docs/RELEASE.md.
- Remove the specs of the issues closed since the last retirement.

## Gate

The release PR's `artifact-identity` shows a `RELEASE:` line with every check passing, and all four browser legs pass. Then `qualify`, `publish`, `verify` on main.

## Evidence and decision record (attempt 1, 2026-11-21)

- Worklet module: `234130dd00d9bfe1c20eaa2117ba6205d84dc07c6e3002b99095d29b894e2839`
  (6,330,255 B; named twin `675df2f9...`, 7,177,602 B), built from this tree (main 0e8404ac4's
  module inputs) with `nightly-2026-08-20`. It replaces 0.10.4's `b7b9789c...`. Main 0e8404ac4's
  own CI run recorded the same digest (commit status `audioworklet-sha256`), so this PR moves no
  byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and test-npm-publish-modes.py's
  accepted `release_pin` and its "wrong release pin" mutation carry it. `--check-pin` passes.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 1b9032161
  --self-test-mutations`, Playwright 1.62.1, a null PulseAudio sink on a short socket path
  `/tmp/qa.XXXXXX/native`), all four legs passed on the first run:

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 20.3 s |
  | Chromium | 151.0.7922.34 | worker | pass | 20.7 s |
  | WebKit | 26.5 | single | pass | 93.9 s |
  | WebKit | 26.5 | worker | pass | 71.9 s |

  Every render-allocation count is 0 on every leg, and every seek row underruns 0 frames.
- Local: `test-npm-publish-modes.py`, `check-workspace-policy.sh`, `check-sdk-generated.sh`,
  `sdk-package.sh check` (packs `@misofm/engine@0.10.5`), `web-audioworklet-identity.py
  --self-test`: pass.
- Retired: the specs of 55 closed issues (#1286 to #2431).
