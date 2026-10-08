# Witness the staging-read boot caller and test the render allocation reply check hermetically

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-08 by root from the #1333 attempt-1 verdict, MINOR 4 (open item 2). #1333 added a boot
caller with a new option shape and a new host request; the hermetic suite covers neither.

No engine source changes. No rendered bit moves.

## Problem (verified on `main` at `1e78d7820`)

- **The boot contract.** `qualificationBootContract`
  (`hosts/host-web/qualification/qualification.js:887-894`) exports five boot callers and
  `diagnoseReady` so the hermetic suite runs the same boot objects the browser runs (#288).
  `testQualificationBoot` in `scripts/test-web-audioworklet.mjs` calls each caller through
  `forwardingCreateHost` up to its stop sentinel (`:2628-2648`), then holds the caller list
  (`:2650-2656`), five real ready and five real dispose witnesses (`:2658-2659`), four sentinel
  stops (`:2660`) and each caller's boot options (`:2661-2678`).
- **The new caller is not in it.** #1333 added `runStagingReadRun`
  (`qualification.js:623`, called at `:715-716`). It boots a shape no other caller boots:
  `spectrum: null` with a two-entry `spectrumCollection` (`:636-641`). It is not exported in the
  contract and not witnessed, so a change to its boot options, or a refusal of that shape by the
  host's option guards, is first seen in a real browser.
- **The reply check has no hermetic test.** `renderAllocationCount()`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1318-1320`) sends
  `miso.renderallocations.v1`. The host accepts a reply only with the tag `:1026-1027`, the exact
  fields `["tag", "requestId", "result", "count"]` (`:1021-1022`), `result === 0` and a `u32`
  `count` (`:1136-1137`); anything else fails the host with `255` (`:1138-1143`). The fake worklet
  port in `test-web-audioworklet.mjs` answers `miso.status.v1` and others (`:150-175`) but not
  `miso.renderallocations.v1`; the file never names it. Only the browser legs' happy path
  exercises the request.

## Decisions

- **D1. Export and witness the caller.** `qualificationBootContract` exports `runStagingReadRun`.
  `testQualificationBoot` calls it through `forwardingCreateHost("runStagingReadRun")` with its
  real arguments (one call with `streaming` false is enough if both forms boot the same options;
  if they differ, both are called and both are witnessed). The caller list, the ready and dispose
  witness counts and the sentinel-stop count grow by the calls added.
- **D2. Hold its boot shape.** The suite asserts the witnessed options of `runStagingReadRun`:
  `spectrum` is `null`, `spectrumCollection` has exactly the two entries of `:636-641` in order,
  and the live-control fields equal the observation caller's (`:2674-2678`), as the other callers'
  shapes are held.
- **D3. The reply check, case by case.** The fake port answers `miso.renderallocations.v1` with
  `{ tag, requestId, result: 0, count: 0 }`, through a mutation hook like `statusMutation`
  (`:1352-1355`). One test holds:
  - the well-formed reply resolves with `count` `0`, and one with `count` `4294967295` resolves;
  - each of these fails the host with `255`: a wrong tag, a missing `count`, an extra field,
    `result` `6`, `count` `-1`, `4294967296`, `1.5`, `"0"` and `0n`.
- **D4. Placement.** The new cases live beside the `status` schema case (`:1346-1356`) and reuse
  its host setup and `errorResult` helper.

## Authorized paths

- `hosts/host-web/qualification/qualification.js` (the `qualificationBootContract` export only)
- `scripts/test-web-audioworklet.mjs` (`testQualificationBoot` and the fake port's reply and the
  new reply-check cases only)
- `hosts/host-web/MUTATIONS.md` (the new rows)
- this spec

## Non-goals

- Any change to the host's reply check, to the worklet or to `runStagingReadRun` itself.
- Reading the count in the SDK instances (#1476).
- The `.sh` wrapper's temporary-directory rules (#1421, #1429).

## Hazards

- The boot contract runs the real worklet class with the real module. If the collection shape
  cannot boot hermetically, stop and report; do not weaken the sentinel or the witness counts.
- `qualification.js` is also read by the browser runner; exporting one more name changes no
  browser behaviour.

## Objective gates

1. `bash scripts/test-web-audioworklet.sh` passes (node and, where the script runs it, bun), and
   its log shows the new caller count.
2. Mutations, each applied alone (PR evidence; each must turn the named case red, then revert):
   - `runStagingReadRun` removed from `qualificationBootContract`: the caller witness assertion;
   - `runStagingReadRun` boots `spectrumCollection` with one entry: D2's shape assertion;
   - the host's `message.result === RESULT_OK` removed from `validRenderAllocations`: the
     `result` `6` case;
   - `validU32(message.count)` replaced with `Number.isInteger(message.count)`: the `-1` and
     `4294967296` cases;
   - `validU32(message.count)` removed: the `"0"` and `0n` cases;
   - the `renderAllocations` field list widened by one name: the missing-`count` or extra-field
     case;
   - the `renderAllocations` expected tag changed: the wrong-tag case.
3. `bash scripts/check-workspace-policy.sh` passes.

*Test value.* D1 and D2 are red when `runStagingReadRun`'s boot options change or the host's
guards refuse that shape, which today only a browser run shows; D3 is red when any conjunct of
the reply check is lost, which no test catches today.

## Evidence

- Gate 1 log, each gate-2 mutation's red output and the green run after revert.

## Dependencies

- After (same stream): none open (#1333 is on `main`).
- After (other streams): none.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.
