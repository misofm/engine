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
