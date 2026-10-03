# Name the failed predicate and bound the waits of the browser continuous-spectrum gate by a deadline

## Problem

The browser qualification's `sdk-spectrum-continuous` gate failed once in Firefox and passed when
the failed job was re-run:

```
Error: firefox: sdk-spectrum-continuous: continuous spectrum did not prove warmup, shared ownership, capture loss, or close lifecycle
```

Evidence:
- PR #1238 (batch K3), run 37115329466, attempt 1, job 111181516289, head `8c6268967`. The rerun
  of the failed jobs passed, and the same head passed every local three-browser run.
- The same attempt also failed Chromium's configured hop probe (`H1024 browser probe published 1
  windows`, job 111181516316). That one is *Bound the browser spectrum hop probe by its deadline,
  not a poll count* (#1106), whose spec records it.

Two defects make this failure both possible on a slow runner and impossible to diagnose. Every
anchor is verified on `8c6268967`.

1. **The gate does not say which predicate failed.** `hosts/host-web/qualification/run.mjs:484-489`
   ANDs eight predicates of `spectrum.continuous` (`pendingBeforeRender`, `sharedJob`,
   `automaticDelivery`, `windows >= 1`, `gap`, `ownedArrays`, `sharedAfterFirstClose`,
   `staleReadRefused`) into one gate with one fixed message. The log of the failed run therefore
   cannot say which one was false.
2. **Two of the predicates are decided by a wall-clock race or a poll count, not a deadline**
   (`runContinuousSpectrumQualification`, `hosts/host-web/qualification/sdk-response-entry.ts:487`):
   - `automaticDelivery` (`:566-569`) races the first automatic `onUpdate` against a fixed 100 ms
     `setTimeout`. A slow runner that delivers in 101 ms fails it.
   - The wait for a ready publication (`:573-578`) is bounded by `CONTINUOUS_BLOCKS` (48) pumps,
     not by a deadline. `pump()` can return at once, so the loop can end before a window is
     published, and `windows`, `gap` and the `ready` status then depend on scheduling. This is the
     pattern #1106 removes from the hop probe.

## Smallest closable slice

1. Split the gate at `run.mjs:484-489` so a failure names every false predicate (one gate per
   predicate, or one gate whose message lists the false ones). The pass condition is unchanged.
2. Replace the 100 ms race with a wait bounded by a deadline (10 s, as the hop probe's), and bound
   the ready-publication loop by that deadline, yielding to the event loop between pumps (for
   example `await new Promise((resolve) => setTimeout(resolve, 5))`). Keep every predicate, the
   typed failures and the `windows >= 1` requirement.

## Authorized paths

- `hosts/host-web/qualification/run.mjs` (the `sdk-spectrum-continuous` gates only).
- `hosts/host-web/qualification/sdk-response-entry.ts` (`runContinuousSpectrumQualification` only).
- This spec's own record sections.

## Non-goals

- The hop probe (#1106).
- Any change to the SDK, the worklet, the spectrum engine or the gate's pass condition.

## Objective gates

1. `npm run qualify -- --check-matrix --self-test-mutations` in `hosts/host-web/qualification`
   (SDK source-bundle mode, CI's) passes in Chromium, Firefox and WebKit.
2. A planted slow automatic delivery (a 500 ms delay before the first `onUpdate`) still passes,
   while the same delay against the old 100 ms race fails. Record both as PR evidence, not as a
   committed test.
3. A planted false predicate (for example `staleReadRefused` forced `false`) fails with a message
   that names that predicate. PR evidence.
4. A subscription that never publishes still fails with its typed message within the deadline.

## Evidence

- Each gate's command and result, and the planted runs of gates 2-4.

## Dependencies

- None. It may land with #1106 in one PR.
