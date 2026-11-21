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

## Evidence and decision record
(empty)
