# Make the boot contract red when the worklet skips spectrum collection staging

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-09 by root from the #1477 attempt-1 verdict, NIT 2
(`/home/bl/misofm/submix-verdicts/1477-attempt1.md`), and the "Open for root" item of #1477's
Follow-ups record.

Root's ruling (2026-10-09), verbatim:

> (2) #1477: a hermetic test that is red when a worklet skips collection staging, and correct
> #1477's Hazards "real module" line (stream H).

The Hazards correction is made in #1477's spec in the same commit that files this issue. This issue
is the test.

No product change. No rendered bit moves.

## Problem (verified on `codex/d15-batch-misc` at `e9798393e`, which carries #1477)

- **The boot contract runs the real worklet class on fake exports.** `testQualificationBoot`
  (`scripts/test-web-audioworklet.mjs:2500`) constructs the real worklet processor with
  `withSpectrumCollectionStaging(makeFake(...))` (`:2618`), not the real module.
  `withSpectrumCollectionStaging` (`:2573-2603`) adds the seven pre-boot collection staging exports
  with the bridge's sizing rules, but nothing reads what was staged.
- **The worklet's staging step.** At boot the worklet stages either a single spectrum request or
  a collection (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:313-322`;
  `stageSpectrumCollectionRequest`, `:476-554`), then calls `miso_engine_web_v1_boot` or
  `miso_engine_web_v1_boot_with_spectrum_hop` (`:328-330`). The real bridge reads the staged
  collection at boot (`hosts/host-web/src/ffi.rs`, `staged_spectrum_request`, `:936` ff.); with
  nothing staged it boots with no spectrum capture and no error.
- **The gap.** A worklet that skips collection staging completely (verifier probe p7: the
  collection branch at `:319-321` replaced by `this.stageSpectrumRequest(init.options.spectrum)`)
  is green on the suite with and without #1477
  (`/home/bl/misofm/submix-verdicts/evidence/1477-attempt1/mut/p7-worklet-skip-staging.*.log`:
  `qualification boot contract passed: callers=6 ...`). Only a browser leg shows it. The same holds
  for a staged collection whose bytes differ from the options in a way the fake's sizing does not
  read (a channel code, the capture budget).

## Decisions

- **D1. The fake's boot reads the staged collection.** In `withSpectrumCollectionStaging`, the fake
  wraps whichever boot exports the fake carries (`miso_engine_web_v1_boot` and
  `miso_engine_web_v1_boot_with_spectrum_hop`). Before delegating, the wrapper decodes the staged
  collection from the fake's memory the way the bridge lays it out: the request header (struct
  size, ABI version, entry count, `maximumCaptureBytes`), each 24-byte entry (target code, channel
  code, identity byte length) and the concatenated identity bytes. It records the decoded
  collection, or `null` when the request header's entry count is 0 or the request pointer was
  never asked for, as the boot's staging witness. It does not change the fake's boot result.
- **D2. The staged collection equals the options.** For `runStagingReadRun`, the suite asserts
  the staging witness equals the collection in the caller's own witnessed boot options: the same
  entry count and order, each entry's target code and channel code (the worklet's constants for
  `trackPostPan`, `output` and `both`), each identity string, and `maximumCaptureBytes`. The
  expected value is derived from the witnessed options, not hard-coded, so D2 tests the staging,
  not the caller's values (those stay #1477 D2's).
- **D3. Callers with no collection stage none.** For each of the five other callers, the staging
  witness is `null`.
- **D4. Placement.** All changes stay inside `testQualificationBoot` (the fake wrapper and the
  assertions after the boots).

## Authorized paths

- `scripts/test-web-audioworklet.mjs` (`testQualificationBoot` only)
- `hosts/host-web/MUTATIONS.md` (the new rows)
- this spec

## Non-goals

- Any change to the worklet, the host, the bridge or `qualification.js`.
- Booting the real module in the hermetic suite.
- Holding the single-request staging path (`stageSpectrumRequest`) beyond D3's `null`.

## Hazards

- The fake's memory addresses (`:2577-2579`) are fixed; decode through a fresh `DataView` on each
  boot, as the existing staging exports do.
- `TextDecoder` is the test realm's; the worklet scope still has no `TextEncoder` (#1477's
  attempt record, finding 1).

## Objective gates

1. `bash scripts/test-web-audioworklet.sh` passes with a private `TMPDIR` and leaves nothing in
   it; the log shows the boot contract line.
2. Mutations, each applied alone to the worklet (PR evidence; each red on this suite, green on the
   suite at the base commit, then reverted):
   - p7: the collection branch at `miso-engine-v1-audio-worklet.js:319-321` replaced by
     `this.stageSpectrumRequest(init.options.spectrum)`: D2 red (witness `null`);
   - the collection channel map writes `SPECTRUM_CHANNEL_LEFT` for `both`: D2 red;
   - `request.setBigUint64(16, BigInt(options.maximumCaptureBytes / 2), true)`: D2 red.
3. `bash scripts/check-workspace-policy.sh` passes.

*Test value.* D1-D2 are red when the worklet skips collection staging (p7) or stages a channel
code or capture budget that differs from its options; each of these is green on today's suite,
because the fake sizes the staging but nothing reads it before boot (gate 2 records both suites).

## Evidence

- Gate 1 log; gate 2's three mutations, each with its red run, its base-suite green run and the
  green run after revert.

## Dependencies

- After (same stream): H #1477 (its `withSpectrumCollectionStaging` and `runStagingReadRun`
  witness).
- After (other streams): none.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and #1477's verdict NIT 2 first.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under three hours.

## Attempt record
