# Make the continuous-spectrum gate's capture loss independent of browser timing

Stream J successor of #1248 under decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-08 by root. #1248 named each failed predicate of the browser `sdk-spectrum-continuous`
gate and bounded its waits by a deadline. The Firefox leg still fails intermittently, and every
failure now names one predicate: `gap`. A flaky gate is a defect (owner rule: no shortcuts). Find
the root cause first, then remove it.

No engine or SDK source change is planned; if the root cause is in the engine or the SDK, stop and
report (see Decisions, D3).

## Problem (verified on `main` at `1e78d7820`)

- **The failure.** `firefox: sdk-spectrum-continuous: continuous spectrum did not prove warmup,
  shared ownership, capture loss, or close lifecycle; false: gap`. Seen on PR #1463, run
  37523765300, attempt 1 (the Firefox browser leg; attempt 2 passed), and in the #1414 era before
  #1248 named the predicate.
- **Its rate.** The stream G batch verifier ran the exact qualify command alternately on two trees,
  each with its own artifact and a private null audio sink: base `f5e377d0d` failed 3 of 25 runs,
  the stream G batch 2 of 25. Every failure named `gap` alone. Chromium and WebKit passed every
  run (the stream G batch verifier's finding 2, 2026-10-07; root keeps that record outside the
  tree, and this paragraph carries all of it that matters).
- **The predicate** (`hosts/host-web/qualification/sdk-response-entry.ts:486-667`,
  `runContinuousSpectrumQualification`):
  - an `OfflineAudioContext` of `CONTINUOUS_FRAMES` (6,144) frames at 48 kHz renders a tone with
    a hop of 256 frames (`:17`, `:348-396`);
  - a subscription with `cadenceMs: 1` reads automatically while the context renders (`:527-550`);
  - `gap` (`:603-607`) is true when the last notification has status `"gap"`, or when any
    notification reports `nativeMissedWindows > 0` or `skippedPublications > 0`
    (`callbackGap`, `:534`);
  - `run.mjs:510-528` requires `gap === true` ("capture loss").
- **Why it can be false.** The predicate needs a capture loss, and a loss happens only when the
  reader falls behind the render. Whether it falls behind depends on how fast the browser renders
  the offline context and how often its timers run the 1 ms reader. Nothing in the qualification
  forces the reader to fall behind. The SDK computes the two counters in
  `sdk/src/core/observation-subscriptions.ts:1679-1694`.

## Decisions

- **D1. Root cause first, with evidence.** Before any fix, instrument a local Firefox run (not
  committed) to record, per run, each native stream read's sample range and its wall-clock time
  against the render's progress, and each notification's counters. Reproduce at least one `gap`
  failure and one pass, and state in the attempt record what differs between them. If no failure
  reproduces in 50 runs, stop and report.
- **D2. The fix makes the loss certain by construction.** The qualification must create a capture
  loss that does not depend on render speed or timer cadence, for example by rendering more
  windows than the native stream can hold before any read can run, or by holding the reader until
  the render has finished. The chosen mechanism is stated with the arithmetic that makes the loss
  certain (window count, stream capacity, hop). The same run must still prove automatic delivery,
  the shared job, owned arrays and the close lifecycle; if one mechanism cannot prove all of them,
  split the probe into two subscriptions or two runs, each deterministic.
- **D3. Not a test-only patch for a product defect.** If D1 shows the SDK or the engine reports
  no loss when windows were in fact lost (the counters are wrong), this issue stops, and root files
  the product defect; the gate stays as it is.
- **D4. Forbidden fixes.** No retry loop, no rerun-on-failure, no wider deadline, no removal or
  weakening of the `gap` predicate, no browser-specific branch, no `skip` on Firefox.

## Authorized paths

- `hosts/host-web/qualification/sdk-response-entry.ts` (`runContinuousSpectrumQualification` and
  `createContinuousSpectrumBrowser` only; stream H's file, by named exception)
- `hosts/host-web/qualification/run.mjs` (the `sdk-spectrum-continuous` predicates only, and only
  where D2 adds a field; the pass condition for `gap` stays `true`)
- `hosts/host-web/MUTATIONS.md` (the rows of D2's mutations)
- this spec

## Non-goals

- The other continuous predicates' meaning; the configured-hop and collection probes.
- Reading the SDK instances' allocation counts (H #1476, same file).
- Any CI workflow change.

## Hazards

- `sdk-response-entry.ts` is also edited by H #1476 (the SDK allocation counts). Either order; the
  later slice rebases (STREAMS hot-file row).
- A mechanism that holds the reader must not hold it by a wall-clock sleep: a sleep is a timing
  assumption.
- Rendering more frames lengthens the browser leg; record the leg's time before and after.

## Objective gates

1. **Root cause recorded** (D1): the instrumented runs, one failing and one passing, and the stated
   difference.
2. **Deterministic.** 50 consecutive local Firefox runs of the qualify command (SDK leg on) pass,
   on the same machine class that reproduced the failure. 25 runs each in Chromium and WebKit pass.
   Record each count.
3. **Red when the loss is not detected (PR evidence, not committed; one mutation at a time):**
   - the SDK's `nativeMissedWindows` forced to `0n` and `skippedPublications` to `0n` in the
     notification (`observation-subscriptions.ts:1693-1694`): `gap` is false on every run, in all
     three browsers;
   - D2's mechanism removed (the probe returns to today's shape): the gate is red in at least one
     of 25 Firefox runs or the record shows why today's shape cannot lose a window on that
     machine.
4. `--self-test-mutations` passes in all three browsers; CI's three browser legs pass.
5. `bash scripts/check-workspace-policy.sh` passes.

*Test value.* After the fix, `gap` is red exactly when the SDK fails to report a real capture loss
(gate 3, first mutation), and never red because a browser rendered fast or slow (gate 2).

## Evidence

- Gate 1's instrumented logs, gate 2's counts, gate 3's runs, the browser leg's time before and
  after.

## Dependencies

- After: none open (#1248 is on `main`).
- Either order with H #1476 (hot file).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day. If D1 alone takes more, stop and report what was learned.

## Attempt record

### Attempt 1 (2026-10-08, implementer; base `9dd5ae182`, after #1476 and #1479)

All runs: the CI browser-leg command (`npm run qualify -- --artifacts target/ci/qualification-artifacts
--sdk-root sdk --browser B --check-matrix --self-test-mutations`), SDK source-bundle mode with no
`sdk/dist`, one private PulseAudio null sink per run as `qualification.yml` sets it up, runs strictly
sequential, on a 32-thread AMD EPYC 7313P. The module is the worktree's
`target/ci/qualification-artifacts` (built 20:36 UTC from the #1479 tree; no rebuild, disk at 22 GB
free).

**Gate 1, root cause (D1).** Instrumentation (not committed) logged each native
`readSpectrumStream` (send and reply time, status, `droppedCaptures`, sample span), the render's
start and end, and each notification's counters. 50 Firefox runs: 48 passed the continuous gate, 2
failed it with `false: gap` (runs 21 and 40); 2 other runs failed other probes (see open items).
The native stream holds one record (`SPECTRUM_RESULT_SLOTS = 1`); 6,144 frames at hop 256 publish
17 windows; a native read round trip in Firefox takes 8-17 ms; the offline render takes 6-24 ms.
- Continuous-gate pass (run 16; its continuous probe shows the pattern of the 48 continuous-gate
  passes, but the full qualify command of run 16 failed later, on `sdk-spectrum-hop`; see open
  items): the first read that runs during the render meets drops already
  counted (`status 3 (gap), droppedCaptures 6`); the SDK's recovery read pops window 0; the first
  ready notification carries `nativeMissedWindows 6, skippedPublications 1`; `gap` is true.
- Fail (run 21): the first read that runs during the render pops window 0 before window 1 exists
  (`status 6 (ready), droppedCaptures 0`). The first ready notification carries `0, 0`, sets
  `recoveryDelivery`, and the probe's wait loop exits at once and computes `gap`. The render then
  drops 15 windows; the next automatic read reports `status 3, droppedCaptures 15` and the SDK's next
  notification carries `nativeMissedWindows 15, skippedPublications 1`, but it arrives during the
  probe's `subscription.close()`, after `gap` was computed. Run 40 is the same, with the loss read
  after the probe returned.
- Difference: whether one native read pops the queue between window 0 and window 1 of the render.
  That depends on the browser's render speed and message latency only.

**D3.** The counters are right: every loss the native side counted reached the next SDK
notification exactly (run 21: 15 dropped, `nativeMissedWindows 15`). No product defect; the defect
is the probe's race.

**D2, the mechanism.** `runContinuousSpectrumQualification` holds the native reads during the
render: its `readSpectrumStream` wrapper awaits a promise (`readsHeld`, released in a `finally` once
`startRendering` settles; no sleep), and every read already dispatched has settled before
`startRendering` is called. So no pop runs during the render, and the arithmetic is exact: windows
end at frames 2,048, 2,304, ..., 6,144, that is (6,144 - 2,048) / 256 + 1 = 17 windows into a
one-record queue: 1 queued, 16 dropped. The first read after the render must report `status 3`
with `droppedCaptures 16`; the SDK's single-flight poll then does its recovery read before it
notifies, so the first ready notification carries `nativeMissedWindows 16, skippedPublications 1`.
The probe records that first read as `renderLoss`, and `run.mjs` adds the named predicate
`renderLoss` (`status === 3 && droppedCaptures === "16"`); `gap` stays required `true`, unchanged.
Automatic delivery, the shared job, owned arrays and the close lifecycle are proven by the same run
as before (the automatic reader runs; it only waits for the render).

**Gate 2, determinism.** After the change: Firefox 50 of 50 runs pass every gate; Chromium 25 of
25; WebKit 25 of 25. Leg time (mean wall time of the whole qualify command): Firefox 15.5 s
before (50 instrumented runs; one uninstrumented run 14.4 s) and 15.4 s after; Chromium 4.8 s,
WebKit 9.7 s after.

**Gate 3, mutations (not committed; recorded in `hosts/host-web/MUTATIONS.md`).**
- SDK `nativeMissedWindows: 0n`, `skippedPublications: 0n` in `#notifySpectrum`: red on 5 of 5 runs
  in each of Chromium, Firefox and WebKit, every one `false: gap` alone.
- D2's mechanism removed (the hold's two `await`s deleted): Firefox red on 23 of 25 runs: 22 name
  `renderLoss` (0-8 drops, a read popped mid-render) and 1 names `gap, renderLoss` (the D1 race).
  Today's full shape (no hold, no `renderLoss`) was red on 2 of the 50 D1 runs.

**Gate 4.** `--self-test-mutations` ran in every run above and passed in all three browsers. CI's
browser legs are not run (no push).

**Gate 5.** `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Open items.** In the 50 D1 Firefox runs two other probes failed once each, both non-goals here and
not seen in the 50 post-change runs: run 16 `sdk-spectrum-hop` ("actual browser H1024 did not
preserve Worklet metadata ...") and run 38 `sdk-spectrum-collection` ("spectrum collection did not return distinct owned A/B/A
known-signal spans while audio continued"); their causes were not investigated. Root to decide
whether each needs its own issue.

### Follow-ups (2026-10-08, after the attempt 1 PASS)

- **NIT2.** The D1 record called run 16 a "typical pass". Its continuous probe passed with the
  pattern of the 48 continuous-gate passes, and the facts quoted for it stay as recorded, but the
  full qualify command of run 16 failed (exit 1) on `sdk-spectrum-hop`. The record now says so.
- **Durable D1 evidence.** The 50 instrumented Firefox runs are kept at
  `/home/bl/misofm/submix-verdicts/1480-d1-traces/`: `run-N.log` is the qualify output of run N,
  `trace-N.json` is the continuous probe's read and notification trace of run N, and `summary.txt`
  lists each run's exit code (runs 16, 21, 38 and 40 exit 1).

### Root record (2026-10-09): the two pre-fix single failures

- Before the fix, `sdk-spectrum-hop` (D1 run 16) and `sdk-spectrum-collection` (D1 run 38) each
  failed once in the 50 instrumented Firefox runs. Neither failed in the verifier's 110 runs or in
  the batch run. No issue is filed unless either recurs (root, 2026-10-09).
