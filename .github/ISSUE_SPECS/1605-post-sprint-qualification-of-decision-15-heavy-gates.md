# Post-sprint qualification of decision 15's heavy gates

Successor of #1530, #1532 and #1535 (root, 2026-10-13, Q8). Stream H. Issue number: assigned at
filing (written here as #1605). Not feature sprint work: it runs in the testing phase after the
sprint (owner, 2026-10-13).

## Product outcome

The three heavy measurements that the feature sprint moved out of their slices are run once, on
one head, with their bars unchanged. Each one either passes or opens a named defect. The slices
kept their CI rows; this issue holds only the measurements that CI cannot run or cannot afford.

## Context

- #1535 gate 8 (loaded paired A/B against `main`, Fisher one-sided p, stop on a full stall
  signature) needs about 260 browser runs and a loaded host. Method:
  `/home/bl/misofm/submix-verdicts/b3-acceptance/scripts/`.
- #1530 gate 6 (soak): 5 consecutive full-leg passes per browser and 10 Firefox passes under 24
  CPU hogs (the #1522 method).
- #1532 gate 7: source mutations M1-M9 (PR evidence, never committed tests).
- Not here: #1331's iOS leg (#1331b) is an owner-scheduled human step in #1331 itself (root,
  2026-10-13).

## Decisions

- D1. Every bar is the one in the source spec, unchanged. No bar is relaxed to pass.
- D2. One head: the `main` commit after the decision 17 browser chain (#1565, #1571) has landed.
  #1535's and #1530's measurements run on the then-current gates. A gate the chain changed (#1589
  reworks the observation hooks) is measured as it stands on that head.
- D3. Benchmark rule: one invocation, one warmup, two measured rounds, no retry, where a source
  spec measures time.

## Objective gates

1. #1535 gate 8, as written there.
2. #1530 gate 6, as written there.
3. #1532 gate 7: each mutation turns its named test red, recorded in this spec.

## Test value

- Gate 1 turns red if unsubscribe fails under load at a rate `main` does not show.
- Gate 2 turns red if the observation wait flakes under repetition or load.
- Gate 3 turns red if a close-path bound in #1532 is untested.

## Dependencies

- #1530, #1532, #1535 (their CI rows on `main`); #1565 and #1571 (the head of D2).

## Decision record (root, 2026-10-24/25)

- **Split.** Slice A (now): #1532 gate 7 (M1-M11) and #1535 source mutations M1-M3, one unloaded
  run each per browser. Slice B (owner-scheduled, after #1589 and #1590 merge, quiet machine): #1535
  gate 8 (about 440 runs), #1530 gate 6 soak, #1535 page mutations (d) and (e).
- **D4.** On one head the paired A/B of #1535 gate 8 has no B side, so its Fisher test is undefined.
  The bar is its absolute half: no `observation-unsubscribe` failure except a full-signature stall
  (failures counted by first failing gate; #1522/#1531 gates listed apart). This is a consequence of
  the method, not a relaxed bar (D1 stands).
- **#1532 count.** The source spec lists M1-M11 (M10, M11 were added after the Context line "M1-M9").

## Slice A record (2026-10-25, head `de42aac5a`)

Mutations were applied one at a time in the worktree and restored with `git checkout` and `touch`;
no tracked change to `sdk/` or `hosts/host-web/`. The spec's line anchors are stale: `:563` is now
the quantum-refusal close (`engine.ts:696`), `:1327-1329` the worklet-boot-failure close (`:1578`).

**#1532 gate 7.** `node --test test/browser-evals.mjs test/spectrum-browser-evals.mjs` in `sdk/`
(artifact from `scripts/build-web-audioworklet.sh`): baseline 41/41 pass. Every mutation red.

| M | Mutation | Red test (gate) | Output |
|---|---|---|---|
| M1 | `await host.dispose()` unbounded | dispose never settles (1) | pending at event-loop drain (`cancelledByParent`) |
| M2 | `CLOSE_STEP_TIMEOUT_MS = 10_001` | (1); also (3) | pending at drain |
| M3 | `9_999` | (1) | `still waiting one millisecond before the bound` `true !== false` |
| M4 | dispose catch rethrows (no context close) | (1); also (4) and the rejected-dispose close test | `the context close is attempted after a dispose timeout` `0 !== 1` |
| M5 | rejection handler rethrows after the timeout | (5a); also (3) | `unhandledRejection` `late port failure` |
| M6 | `closePromise = undefined` on rejection | (4) alone: pending at drain; full run: rejected-dispose close test | `a rejected close is still not replayed` `2 !== 1` |
| M7 | no `clearTimeout` | (5b) | `timers after close 11 exceed 9 before it` |
| M8 | spectrum timeout dropped | (3) | `Missing expected rejection.` |
| M9 | quantum-refusal close unbounded | (6b) | pending at drain |
| M10 | boot-failure close timeout thrown | (6c) | caught `contextCloseTimeout ... AudioContext close` |
| M11 | one shared code `closeStepTimeout` (added to `SdkTimeoutCode`) | (1), (2), (3), (5a), (6a) | `'closeStepTimeout'` vs `'hostDisposeTimeout'` / `'spectrumCloseTimeout'` |

An unbounded wait shows as a pending promise when the mocked-timer event loop drains, not as the
5 s test timeout; it cancels the later tests of the suite, so M1, M2, M6 and M9 also cancel tests
they do not target.

**#1535 source mutations.** CI command of `qualification.yml:479` (`--check-matrix
--self-test-mutations`, `--sdk-root sdk`), private PulseAudio null sink (short socket path), one run
per browser under the exclusive heavy lock, no retry. Chromium 151.0.7922.34, Firefox 153.0, WebKit
26.5, Playwright 1.62.1. 1-minute load average at start in brackets.

| Run | Chromium | Firefox | WebKit |
|---|---|---|---|
| base (no mutation) | pass (2.3) | pass (6.2) | pass (6.1) |
| M1' | pass (11.6) | pass (11.8) | pass (16.2) |
| M2 | red (10.9) | red (8.9) | red (5.8) |
| M3 | pass (4.5) | pass (3.7) | pass (2.9) |

- **M1 re-expressed.** The page has no `statusRead` (D3.3 dropped; the host shares one read). M1'
  removes the host's sharing (`status()` always calls `#sendStatus()`, in a copy of the artifact's
  `miso-engine-v1-audio-worklet-host.js`) and sets `STALL_POLL_TIMEOUT_MS = 1`. Green in all three:
  the host no longer has a status in-flight bound (`#saturated` has no `status` case), so nothing
  can refuse a status read. The claim M1 defended has no bound left to trip: **no longer
  applicable** as a page gate. Finding (not hidden): the qualification does not detect a host that
  stops sharing its status read.
- **M2** (`while (fed < target)` and `UNSUBSCRIBE_LEAD_BLOCKS = 60`): red in all three with
  `observation-unsubscribe qualification failed: request refused at submitSource block 40:
  kind=source inFlight={"source":60,...} cause={"tag":"miso.error.v1","requestId":0,"result":6}`.
- **M3** (signal count sums `(endSample - firstSample) / 128` per frame): **survives** in all three
  unloaded runs (signal frames 48 / 49 / 48, as in the baseline). Unloaded, every frame after the
  acknowledgement is one block, so counting blocks equals counting frames. The multi-block case
  occurs only under load; slice B's loaded legs are where it can show. Recorded; the gate is unchanged.
