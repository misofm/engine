# Name the failed predicate and bound the waits of the browser continuous-spectrum gate by a deadline

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).

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

1. Build the shipped artifact as the `artifact` job does (`rm -rf target/ci/qualification-artifacts
   target/ci/qualification-named-twin && mkdir -p target/ci/qualification-artifacts
   target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin
   target/ci/qualification-named-twin target/ci/qualification-artifacts`), run
   `npm ci --no-audit --no-fund` in `sdk/` and `npm ci` in `hosts/host-web/qualification`, then
   from `hosts/host-web/qualification` run, for each of `chromium`, `firefox` and `webkit`,
   `npm run qualify -- --artifacts "$PWD/../../../target/ci/qualification-artifacts" --sdk-root
   "$PWD/../../../sdk" --browser <name> --check-matrix --self-test-mutations` (the `browser` job's
   invocation in `.github/workflows/qualification.yml`; `--sdk-root` selects the SDK source-bundle
   mode CI runs; a headless machine needs an audio sink, which CI provides with a PulseAudio null
   sink). All three pass.
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

## Attempt record

### Attempt 1 (implementer, 2026-10-05, base `0e3e21b68`)

Change:
- `run.mjs`: the lifecycle gate of `sdk-spectrum-continuous` keeps its pass condition (the
  conjunction of the eight predicates) but evaluates each predicate as a named entry. Its message
  keeps the old text and appends `; false: <names>`, which lists every false predicate (`windows >= 1`
  carries the observed `windows` count).
- `sdk-response-entry.ts` (`runContinuousSpectrumQualification`): one 10 s deadline starts once
  rendering returns. The automatic-delivery race now uses that deadline instead of 100 ms, and its
  timer is cleared once the race settles. The ready-publication loop runs until a ready, available
  publication is readable or the deadline passes. It yields 5 ms to the event loop between pumps.
  Every predicate, the typed `did not publish a window` failure and `windows >= 1` are unchanged.
  The loop no longer used the module constant `CONTINUOUS_BLOCKS`, so it is deleted (one line above
  the function, and its only use was the replaced loop).

No test is committed. The gate itself is the qualification. Gates 2-4 are planted runs and serve as PR
evidence, as the spec directs. Each plant was applied to a scratch copy, run, and reverted, and the
committed tree is byte-identical to the fix diff.

Gate 1. The artifact was rebuilt as the `artifact` job builds it (shipped module `a9a51862…`), then
`npm ci` was run in `sdk/` and the qualification directory. The `browser`-job invocation (`--sdk-root`,
`--check-matrix`, `--self-test-mutations`) was run against the local PulseAudio sink:
- chromium 151.0.7922.34: `all qualification gates passed`
- firefox 153.0: `all qualification gates passed`
- webkit 26.5: `all qualification gates passed`

Gate 2 (slow automatic delivery). The plant removed the `onUpdate` resolve and resolved `callbackSeen`
500 ms after `startRendering()` returned, so the first automatic delivery is seen 500 ms after the race starts:
- with the fix: chromium passes (`all qualification gates passed`).
- with the old 100 ms race (old `sdk-response-entry.ts`, new `run.mjs`): chromium fails with
  `sdk-spectrum-continuous: ... close lifecycle; false: automaticDelivery`. The race was the defect,
  and the new message names it.

A first plant delayed the resolve by 500 ms from the first callback. The old code passed it too,
because in this setup the first automatic callback fires during `startRendering()`, before the race
begins. A slow runner breaks the race only when delivery lands after rendering returns, so the plant
above models that case.

Gate 3 (named false predicate). The plant forced `staleReadRefused = false`. chromium fails with
`sdk-spectrum-continuous: continuous spectrum did not prove warmup, shared ownership, capture loss,
or close lifecycle; false: staleReadRefused`.

Gate 4 (never publishes). The plant replaced `subscription.pump` with `async () => undefined` and
`readLatest` with `() => undefined`. chromium fails with the typed error
`continuous spectrum did not publish a window` after 9994 ms of loop wait, so it is bounded by the
10 s deadline. The plant appended the wait to the message, and the whole browser run took 12 s.

Defect each gate change catches, which no existing check catches:
- Message split: a false lifecycle predicate in a run would be reported only under the fixed message.
  Gate 3 shows the predicate now named.
- Deadline: delivery or publication slower than 100 ms, or than 48 immediate pumps, on a slow runner
  would fail a correct engine (the #1238 Firefox flake). Gate 2 shows the old code red and the new
  code green under the same delay.

Not run: the CI runner itself (no push, per worker rules). Only chromium was used for the plants. The
predicates under test are browser-independent harness logic.

### Attempt 1 verdict follow-ups (batch follow-ups, 2026-10-05)

The verifier passed attempt 1 and ran a closer gate-2 plant (plant B), which is now the stronger
gate-2 evidence. Plant B held every `onUpdate`, automatic or pumped, until 500 ms after
`startRendering()` returned, then replayed them in order. This is the spec's "500 ms delay before
the first `onUpdate`" directly:
- with the fix: chromium, firefox and webkit all pass (race wait 499-500 ms, loop 0 ms).
- with the old code: chromium fails with
  `...; false: automaticDelivery, windows >= 1 (windows=0), gap`. The old 48-pump loop also gave up
  before any window was delivered.

Note for #1106 (no change here, outside this spec's authorized paths): the 10 s deadline literal is
duplicated, once in `runContinuousSpectrumQualification` and once in the hop probe (`10_000`). When
#1106 lands, it can hoist one shared spectrum-deadline constant.
