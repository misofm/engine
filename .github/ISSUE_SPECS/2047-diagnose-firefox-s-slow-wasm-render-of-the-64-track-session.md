# Diagnose Firefox's slow Wasm render of the 64-track session

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1331, its "#1331a-2 result" section
(`.github/ISSUE_SPECS/1331-prove-two-wasm-instances-on-one-shared-memory-in-three-browser-engines-and-on-io.md`),
evidence `/home/bl/misofm/engine-work-evidence/1331/a2/` (README there).

## Problem
In Firefox the 64-track app session renders far slower than in the other engines. #1331a-2 D7 headroom
(48 kHz, 128 frames, atomic module on a shared memory, Worker instance): Chromium 151 h = 16.1-16.4,
WebKit 26.5 h = 13.7, Firefox 153 h = 1.10 (about 0.91 of real time per block). During the spans 4381 of
10219 Firefox worklet calls (about 43 %) read over 2 ms; #1331a-1 saw 44 % in `two` and 47-48 % in both single
legs, with one instance and no Worker (the memory is still shared). So the slowdown is not specific to the
Worker or the second instance. Every leg that recorded a late-call share ran the atomic module on shared
memory; no leg measured late calls on the plain module. The cause is not investigated.

Firefox h = 1.10 gives m*h = 0.77 <= 1, which triggers #1286 D3's stop. **#1286 D3 stays stopped (it derives
no browser value) until this issue is resolved and an owner ruling is made.** The #26 ceiling also fails on
Firefox.

## Evidence
- #1331 spec "#1331a-2 result": the D7 table and the late-call shares.
- `engine-work-evidence/1331/a2/results/firefox-d7.json` (and chromium, webkit); `a2/workload/FROZEN.txt`;
  `a2/harness/` (`run.mjs`, `worker.js`, `processor.js`); `a2/build/{atomic,plain}/` (atomic
  `ff266dfc...`, plain `4a7dc7ee...`, both nightly-2026-08-20).

## Smallest slice
A descriptive diagnosis, no engine change. Separate three candidate causes with one run each (frozen
workload, one warmup, two measured rounds, exclusive bench lock):
1. The Playwright Firefox build: the same page in a release Firefox of the same major version.
2. Shared memory: the plain module on an unshared memory, with the #1331a-2 late-call counters.
3. The `+atomics` build: the atomic module versus the plain module on the same memory kind.
Report h and the late-call share per run and name the cause, or state which candidates remain. Then root
takes the result to the owner for the #1286 D3 ruling.

## Gates
- Each run's raw record kept under `engine-work-evidence/` with its frozen workload hash; the table of
  h and late-call share per (browser build, memory kind, module) recorded in this spec.
- No committed code change; if a cause is in engine code, a successor issue is filed.

## Paths
This spec only (evidence outside the repo).

## Decision record

### Slice 1 result (2026-10-25, worktree base `c4f7d530e`; modules at the #1331a-2 base `b0bc367af`)

Evidence: `/home/bl/misofm/engine-work-evidence/2047/` (README, `FROZEN.txt`, one raw JSON per cell). The
#1331a-2 workload and modules are reused unchanged (atomic `ff266dfc...`, plain `4a7dc7ee...`, workload
`a2785943...`). New module: U/A `de663a63...`, the atomic shape's features (`+atomics`), linked as the
plain shape. It has the same atomic instructions as S/A, on an exported unshared memory. The plain shape
rebuilt at the same base reproduced `4a7dc7ee...`. Hashes were recorded before the first timed run. Each
cell ran once under `flock -x`, with no retry. Worklet metric: process() calls in the measured window
(`two`: while the Worker runs a span; `single-iso`: 10000 calls after the 350-block digest window),
late = over 2 ms, calls of 1 ms or more, and max (`Date.now`, 1 ms resolution). `h` is given only where
the memory is shared (the Worker leg). Digest = the first 350 worklet blocks.

| Cell | Browser build | Memory / module / leg | h warmup / r1 / r2 | worklet calls, late > 2 ms, >= 1 ms, max | digest |
|---|---|---|---|---|---|
| 1 | Playwright Firefox 153.0 (rev 1538) | S/A, `two` | 1.12 / 1.12 / 1.13 | 9997, 4146 (41.5 %), 9997, 4 ms | `44ecef33` |
| 1b | Playwright Firefox 153.0 | S/A, `single-iso` | - | 10000, 4009 (40.1 %), 10000, 5 ms | `44ecef33` |
| 2 | stock Firefox **153.0** (Mozilla tarball, no Playwright) | S/A, `two` | 15.55 / 18.24 / 18.22 | 656, 0, 237, 1 ms | `44ecef33` |
| 2b | Playwright Firefox 153.0 binary, launched **without** Playwright | S/A, `two` | 1.15 / 1.15 / 1.15 | 9794, 3529 (36.0 %), 9794, 5 ms | `44ecef33` |
| 3 | Playwright Firefox 153.0 | U/A, `single-iso` | - | 10000, 3926 (39.3 %), 10000, 4 ms | `44ecef33` |
| 4 | Playwright Firefox 153.0 | U/P, `single-iso` | - | 10000, 3567 (35.7 %), 10000, 5 ms | `44ecef33` |
| 5 | Playwright Firefox 153.0, `wasm_optimizingjit=false` | S/A, `two` | 1.12 / 1.12 / 1.13 | 10013, 4141 (41.4 %), 10013, 5 ms | `44ecef33` |
| 5b | stock Firefox 153.0, `wasm_optimizingjit=false` | S/A, `two` | 8.51 / 8.42 / 8.45 | 1326, 0, 708, 2 ms | `44ecef33` |
| k1b | Chromium 151.0.7922.34 | S/A, `single-iso` | - | 10000, 0, 3349, 1 ms | `44ecef33` |
| k3 | Chromium 151.0.7922.34 | U/A, `single-iso` | - | 10000, 0, 3461, 1 ms | `44ecef33` |
| k4 | Chromium 151.0.7922.34 | U/P, `single-iso` | - | 10000, 0, 3966, 1 ms | `44ecef33` |

All cells: 0 render errors, 0 non-finite words, 0 missed quanta, 0 worklet render allocations, no trap.
The sum of the per-call `Date.now` deltas divided by the call count (in each raw record, `sumMs`) is
2.32-2.38 ms in every Playwright Firefox cell (2.32 is cell 2b). This agrees with 2.667 ms / h = 2.38 ms.

**Run deviation (stated candidly).** The first stock install downloaded and staged an update to 158.0.1
during its untimed preflight launch, and the update was applied when cell `c2` launched (`logs/auto-update/`). So the first "stock" pair ran Firefox 158.0, although
`--version` printed 153.0: `c2` gave h 15.30 / 18.14 / 18.20 and `c5b` (optimizing tier off) gave 8.51 / 8.52 / 8.44.
Those records are kept and labelled 158. Cells 2 and 5b in the table ran once each, on a fresh 153.0
extraction with `distribution/policies.json` `DisableAppUpdate`. The page user agent reads `rv:153.0` and
libxul is unchanged after the runs. Cells 1b, 2b, 5b and k1b are additions to the scope note's matrix
(1b and k1b bridge the leg change between cells 1 and 3).

**Conclusion per cause.**
1. **The Playwright Firefox build: the cause.** Stock Firefox 153.0 renders the same page and module at
   h = 18.2. This is more than Chromium's 16.1-16.4, but that Chromium value comes from the #1331a-2
   session, so the comparison gives the direction only. The Playwright 153.0 build gives 1.12. The Playwright binary
   launched without Playwright (cell 2b) is also slow (1.15), so the slowdown is in the build itself
   (binary, bundled `playwright.cfg` prefs, or its juggler component), not in a connected Playwright
   client. Juggler loads even without one: cell 2b's stderr shows `chrome://juggler/content/Helper.js`.
2. **Shared memory: not the cause.** S/A and U/A in Playwright Firefox `single-iso` (cells 1b, 3) have
   the same late share (40.1 %, 39.3 %) and mean (2.38 ms). Chromium shows no shared-memory cost (k1b, k3).
3. **The `+atomics` codegen: not the cause.** U/A against U/P (cells 3, 4): 39.3 % against 35.7 % late,
   mean 2.38 against 2.36 ms. The difference is small and the module is slow in both.
4. **Tier-up: in the Playwright build only.** With the optimizing tier off, Playwright Firefox does not
   move (cell 5 = cell 1). Stock Firefox drops from 18.2 to 8.4 (cell 5b). This is consistent with the
   Playwright build never tiering up to the optimizing tier. Nothing here reads the pref or the tier back. It is also about 7.5 times slower than stock Firefox's
   baseline-only render. "Baseline only" therefore does not explain all of the slowdown.

**Mechanism not identified.** No wasm or JIT pref is in `playwright.cfg`. Its only `javascript.options.*` pref is
`javascript.options.showInConsole` (line 296). Leading
candidate (not tested): juggler makes the worker and worklet globals debuggees. SpiderMonkey then compiles
Wasm in debug mode, which is baseline-only with debug instrumentation. This agrees with both facts in
cause 4. Other candidates: a build-configuration difference, or a `playwright.cfg` pref with an indirect
effect.

**Recommendations (root decides).**
- #1286 D3: the Firefox h = 1.10 that started the stop is an artefact of the test browser. Stock Firefox
  153.0 gives h = 18.2. D3's browser row takes the h of the slowest engine. With Firefox at 18.2, the
  slowest engine is WebKit at h = 13.7 (#1331a-2), so D3 binds at m·h of about 0.70 x 13.7 = 9.6 (m = 0.70
  is the m that gave m·h = 0.77 at h = 1.10). That WebKit value also comes from a Playwright-patched build,
  and this slice did not check it against stock Safari. Because 9.6 > 1, the recommendation holds: root
  asks the owner to lift the stop and to take the Firefox value from stock Firefox. Playwright Firefox stays valid for bit identity: the
  digest agrees in every cell. Do not use its timings as performance evidence. The #26 Firefox ceiling
  failure was also measured on Playwright Firefox, so it must be measured again on stock Firefox.
- Next slice (optional, owner's choice, untimed or one run each): find the mechanism in the Playwright
  build. Run a scratch copy of the Playwright install without `playwright.cfg`, then one without the
  juggler component, then stock 153.0 with that file's prefs added. A stock-Firefox runner (`harness/stock.mjs`: headless launch, autostart, POST,
  updates blocked by policy) is the measurement tool for every Firefox timing.
- No engine change follows from this slice.

**Stated limits.** One run per cell. `Date.now` has 1 ms resolution, so the per-call mean is coarse
for sub-millisecond calls (stock and Chromium cells). The k3/k4 means (0.35 vs 0.40 ms) are inside that
coarseness and are not a finding. Stock Firefox ran headless on Linux with the content sandbox reduced
(`unshare(CLONE_NEWPID): EPERM`). The Playwright binary prints the same message when it is launched
without Playwright (cell 2b). macOS and Windows
Firefox were not measured. `perf` is not available on this box, so no profile was made.
