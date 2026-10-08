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
   - `runStagingReadRun` removed from `qualificationBootContract`: the call throws
     `hooks.runStagingReadRun is not a function` before the caller witness assertion (corrected
     (follow-ups, per verdict MINOR 2); was "the caller witness assertion", which the run never
     reaches);
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

*Test value* (corrected (follow-ups, per verdict MINOR 1)). D1 and D2 are red when
`runStagingReadRun` boots different collection entries or entry order, a non-null `spectrum` or
different live-control words; when the host's option guards refuse a `trackPostPan` collection
entry; or when the worklet's pre-boot collection staging refuses or mis-stages the two-entry
collection (the entry count or an identity length written at the wrong offset, a lost
`trackPostPan` mapping, a `TextEncoder` call in the worklet). Today only a browser run shows
these. A host guard that refuses more than one entry or a capture budget above 1 MiB is already
red in the main-realm spectrum option tests. The collection's `maximumCaptureBytes` and
`sourceRingFrames` are not held. D3 is red when any conjunct of the reply check is lost (the tag,
the exact field set, `result` 0, a `u32` count) or when the check refuses the `u32` maximum. No
test catches these today.

The original sentence, struck because it was not true as written (it claimed any boot-option
change, and the host's guards in general):

> ~~*Test value.* D1 and D2 are red when `runStagingReadRun`'s boot options change or the host's
> guards refuse that shape, which today only a browser run shows; D3 is red when any conjunct of
> the reply check is lost, which no test catches today.~~

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

## Attempt record

### Attempt 1 (2026-10-08, implementer)

**Change.** `qualificationBootContract` exports `runStagingReadRun`. `testQualificationBoot` calls it
once (`streaming` false: both forms boot the same options and the sentinel stops at boot), holds
the six-caller witness list, six ready and six dispose witnesses and five sentinel stops, and holds
D2's shape (`spectrum` `null`, the two collection entries in order, the observation caller's four
live-control words). The main-realm fake port answers `miso.renderallocations.v1` through
`renderAllocationsMutation`; beside the `status` schema case, the nine refusal cases each run on a
fresh host and must fail it with `255`, then the `count` `0` and `4294967295` replies must resolve
with their count. The refusal cases are all run before one verdict, so a lost conjunct names every
reply it lets through.

**Two hermetic-environment findings (D1 hazard).** The collection shape did not boot in the
boot-contract harness as it stood, for two reasons that are harness gaps, not product defects:

1. `testProcessor` sets `globalThis.TextEncoder = undefined` to model the worklet scope, and
   `testQualificationBoot` runs the main-realm host in the same global. The host's collection guard
   (`validSpectrumCollectionBoot`) measures each identity with `TextEncoder`, which a browser main
   thread has, so the host refused with `1` (`TextEncoder is not a constructor`). No earlier caller
   booted a spectrum identity, so none reached it. Fix: `testQualificationBoot` takes the main
   realm's encoder (`mainRealmTextEncoder`, passed at its one call site) and installs it only for
   the synchronous span of `createMisoAudioWorkletHost` before its first await, where the option
   guards run; the worklet still runs with no `TextEncoder`.
2. `createFakeExports` has no pre-boot spectrum collection staging exports, so the real worklet
   could not stage the collection. Fix: `testQualificationBoot` adds the seven exports to its fake
   with the bridge's sizing rules (`hosts/host-web/src/ffi.rs`: entry capacity is the header's entry
   count, identity capacity is the sum of the staged identity lengths).

Neither weakens the sentinel or a witness count. Both stay inside `testQualificationBoot` (plus the
one argument at its call site).

**Gates.**

- Worklet chain: `scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin
  target/ci/qualification-artifacts` passed (module `6175e70c...`, unchanged engine);
  `scripts/check-web-audioworklet.sh --without-metadata-regeneration ...` passed;
  `check-browser-expected-resources.py --artifacts` passed.
- Gate 1: `bash scripts/test-web-audioworklet.sh` with a private `TMPDIR` passed, nothing left in
  it; the log shows `qualification boot contract passed: callers=6 real-ready=6 real-disposed=6
  diagnose-ready=1`. (Node present, so the script runs node, not bun.)
- Browser leg, `npm run qualify -- --artifacts ... --sdk-root sdk --browser <b> --check-matrix
  --self-test-mutations` with a private pulseaudio sink: Chromium 151, Firefox 153 and WebKit 26.5
  each `all qualification gates passed`.
- Gate 3: `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Gate 2 mutations.** Each applied alone, run against this suite and against the suite at
`e403684d9` (the base, which has none of the new cases), then reverted:

| Mutation | This suite | Base suite |
| --- | --- | --- |
| `runStagingReadRun` removed from the contract | red: `TypeError: hooks.runStagingReadRun is not a function` at the call (before the witness list assertion is reached) | green |
| one-entry `spectrumCollection` | red: `staging-read spectrum collection changed` | green |
| `message.result === RESULT_OK` removed | red: accepted `['result 6']` | green |
| `validU32` -> `Number.isInteger` | red: accepted `['count -1', 'count 4294967296']` | green |
| `validU32(message.count)` removed | red: accepted `-1`, `4294967296`, `1.5`, `"0"`, `0n` | green |
| field list widened by `"extra"` | red: accepted `['an extra field']` | green |
| expected tag changed to `miso.status.v1` | red: accepted `['a wrong tag']` | green |

After revert the suite is green.

**Test-value correction.** The Test-value paragraph says D1 is also red when "the host's guards
refuse that shape, which today only a browser run shows". Two generic guard mutations tried for
that half -- the collection guard requiring exactly one entry, and the collection capture capped
at 1 MiB (the staging-read run asks for 2 MiB) -- turn this suite red but also turn the base suite
red: the main-realm spectrum option tests already boot two-entry and larger collections. D1's
unique catch is the caller's own boot options (rows 1-2), not the host's generic guards.

**Open.** None in scope. ~~Root may want the Test-value sentence narrowed to the caller's options.~~
(Corrected (follow-ups, per verdict MINOR 1): this narrowing was wrong. It would delete true unique
catches -- a host guard that refuses a `trackPostPan` collection entry and the worklet's pre-boot
collection staging -- and keep a false one, `maximumCaptureBytes`, which no assertion holds. See
the Follow-ups section.)

### Follow-ups (2026-10-08, from the attempt-1 verdict: PASS, 2 MINOR, 3 NIT)

**Changes.**

- MINOR 1: the Test-value sentence is replaced with the verifier's corrected claim. The original
  is kept struck. The attempt record's proposed narrowing is struck and corrected.
- MINOR 2: gate 2 row 1 now says the call throws `hooks.runStagingReadRun is not a function`
  before the caller witness assertion.
- NIT 3: in `scripts/test-web-audioworklet.mjs`, each refusal case still fails exactly when
  `errorResult(..., 255)` rejects (the verdict logic is the same). The failure entry now names what
  the host did: `resolved with count N`, `refused with result R, fields ...` or `rejected with ...`.
  The assertion message is now "the host did not refuse every malformed render allocation reply
  with 255".
- NIT 1 needs no change (the one argument at the call site is reported above).

**Mutation evidence for the corrected sentence.** Each applied alone to a copy of this tree, run
with the follow-up suite (new) and the suite at `e403684d9` (base), then reverted:

| Mutation | New suite | Base suite |
| --- | --- | --- |
| p3: host collection guard `SPECTRUM_TARGETS.has(entry.target)` -> `entry.target === "output"` (refuses `trackPostPan`) | red: `runStagingReadRun real host guard rejected (miso.error.v1 result=1)` | green (callers=5) |
| p5: worklet pre-boot staging target map loses `trackPostPan` | red: `runStagingReadRun real host guard rejected (miso.error.v1 result=1)` | green |
| p8: worklet pre-boot staging measures the identity with `new TextEncoder()` | red: `... rejected (miso.error.v1 result=255)` | green |
| p9: `runStagingReadRun`'s `maximumCaptureBytes` 2 MiB -> 1 MiB | green (callers=6) | green |

p9 confirms that `maximumCaptureBytes` is not held, as the corrected sentence says. The other
parts of the sentence (p4, p6, p10, p11, m2-m7, and p1/p2 red on both suites) rest on the
verifier's runs, `submix-verdicts/evidence/1477-attempt1/mut/`; this follow-up does not change the
product or those assertions.

**NIT 3 evidence.** q1: the host's failure path refuses a `renderAllocations` reply with `1` in
place of `255`. New suite: red, `the host did not refuse every malformed render allocation reply
with 255`, entries `'a wrong tag: refused with result 1, fields requestId,result,tag'`, ... .
The attempt-1 suite on the same mutation: red, `the host accepted a malformed render allocation
reply`, entries `'a wrong tag'`, ... (the wrong label). Base suite: green. m3 (`result ===
RESULT_OK` dropped) on the new suite: red, `'result 6: resolved with count 0'`.

**Gates.**

- Gate 1: `bash scripts/test-web-audioworklet.sh` with a private `TMPDIR`: exit 0, nothing left
  in the `TMPDIR`, both node passes log `callers=6 real-ready=6 real-disposed=6 diagnose-ready=1`.
- Gate 3: `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No browser-compiled code changed, so the worklet build chain and the browser legs were not
  re-run.

**Open for root.**

- NIT 2 (out of scope, not fixed): the boot contract gives the real worklet class
  `createFakeExports`, not the real module, and the fake does not check that the boot uses the
  staged collection. A worklet that skips collection staging completely (verifier probe p7) is
  green on both suites; only a browser leg shows it. The Hazards line "runs the real worklet class
  with the real module" is also not correct for the same reason. A successor issue is needed if
  root wants it caught hermetically.
