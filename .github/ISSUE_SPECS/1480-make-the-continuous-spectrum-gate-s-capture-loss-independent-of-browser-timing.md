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
