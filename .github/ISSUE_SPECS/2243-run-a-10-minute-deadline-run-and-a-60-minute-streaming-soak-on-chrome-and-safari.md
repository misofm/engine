# Run a 10-minute deadline run and a 60-minute streaming soak on Chrome and Safari

Filed by root on 2026-11-04 with the owner's ruling on #26 (close the August umbrella; keep one bounded soak).

## Problem
The release process (`docs/RELEASE.md`, `qualification.yml`, npm releases) qualifies each release on Chromium and WebKit, but no gate runs the engine for long: every browser case lasts seconds. #26 carried a 10-minute deadline run and a 60-minute streaming soak, but it was written for targets that are gone (native runner #1033, sidecar #25, Firefox).

## Scope
One soak, run on demand (not per PR):
- a 10-minute deadline run and a 60-minute streaming soak of the 64-track console session, at 44.1, 48, 88.2 and 96 kHz, on Chrome and Safari (Playwright Chromium and WebKit), both instance modes;
- gates: 0 render allocations, 0 underrun frames after warm-up, no deadline miss beyond the reported counter, memory flat (no growth beyond one stated bound after warm-up);
- a short record under `artifacts/steps/` and a line in `docs/RELEASE.md` saying when to run it (before a minor release).

## Gate
One recorded run that passes every gate, and one mutation (an injected per-block allocation or a leak) that the soak reports red.
