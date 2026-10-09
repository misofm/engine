PASS

# #1477 attempt 1 -- adversarial verdict

Commit `870a61436` (parent `e403684d9`), worktree `/home/bl/misofm/wt-d15-misc`. I reviewed exported trees
(`git archive 870a61436` and `git archive e403684d9`) and did not build in, edit or check out the worktree.
`git diff e403684d9 870a61436` touches four files: the spec (attempt record appended, no other spec text changed),
`hosts/host-web/qualification/qualification.js` (one line, the `qualificationBootContract` export),
`hosts/host-web/MUTATIONS.md` (seven new rows) and `scripts/test-web-audioworklet.mjs`. In the test file, the
changes are in `testMainRealm` (the `renderAllocationsMutation` hook, the fake port's
`miso.renderallocations.v1` reply, the new reply-check cases beside the `status` schema case), in
`testQualificationBoot`, and one argument at that function's single call site in `testProcessor` (NIT 1). There is
no engine, host, worklet or SDK change. The rebuilt module is `6175e70c63e5...`, the same as the implementer's.

Evidence (logs, the mutation runner, the browser runner): `/home/bl/misofm/submix-verdicts/evidence/1477-attempt1/`.

## Decisions

- **D1 holds.** `qualificationBootContract` exports `runStagingReadRun` (`qualification.js:893`). The boot contract
  calls it once through `forwardingCreateHost("runStagingReadRun")` with its real arguments
  (`test-web-audioworklet.mjs:2756-2758`). One call is enough: `runStagingReadRun` (`qualification.js:623-645`)
  builds its options before the `streaming` branch, so both forms boot the same object. The caller list grows to
  six, the ready and dispose witnesses to six, the sentinel stops from four to five. Gate 1's log shows
  `callers=6 real-ready=6 real-disposed=6 diagnose-ready=1`.
- **D2 holds.** `:2791-2805` holds `spectrum === null`, the two collection entries in order, and the four
  live-control words, the same values the observation caller holds. A one-entry collection (m2) and swapped entries
  (p10) are each red. `maximumCaptureBytes` and `sourceRingFrames` are not held. The spec does not ask for them,
  but the spec's Test-value sentence says they are covered (MINOR 1).
- **D3 holds.** The fake port answers `{ tag, requestId, result: 0, count: 0 }` through `renderAllocationsMutation`
  (`:167-171`). Each of the nine malformed replies runs on a new host and must fail it with 255
  (`:1376-1394`). Then `count` 0 and `count` 4294967295 must resolve with their count (`:1395-1410`).
- **D4 holds.** The cases are directly after the `status` schema case and use `errorResult` with the same host
  factory arguments.

## The harness changes (asked: inside Authorized paths and Hazards?)

Both changes are inside `testQualificationBoot`, which the spec authorizes. They make the harness model the real
environment more accurately, and they do not weaken a sentinel or a witness. I accept them. The hazard's "stop and
report" applies when the shape can boot only with a weaker check. Here, the real host and worklet accept the shape
when the harness supplies what a browser main realm and the real bridge supply.

- **`mainRealmTextEncoder` / `createInMainRealm` (`:2658-2670`).** In a browser, the host always runs in a realm
  that has `TextEncoder`. `testProcessor` deletes it to model the worklet scope, and the boot contract shares that
  global. The swap covers only the synchronous part of `createMisoAudioWorkletHost`. That part ends at the first
  `await` (`fetchModule`, or `addModule` if a prepared module is given). The processor is constructed later, in
  `new AudioWorkletNode`, after `finally` has put the processor-scope value back. Probes:
  - h1 (the swap removed): red, `runStagingReadRun real host guard rejected (miso.error.v1 result=1)`. The swap is
    necessary.
  - p8 (the worklet's collection staging calls `new TextEncoder()` in place of `boundedUtf8Length`): red on the
    new suite, `result=255`. The worklet still runs with no `TextEncoder`, so the processor-scope sentinel is kept.
  - A host that used `TextEncoder` after its first await would go red here but not in a browser. That is a
    false-red risk, not a false-green risk.
- **`withSpectrumCollectionStaging` (`:2561-2595`).** I compared the fake with `hosts/host-web/src/ffi.rs:2509-2645`
  and `hosts/host-web/src/lib.rs:287-305`. The fake follows the real sizing.
  - The request header is 32 bytes (`WebSpectrumCollectionRequest`: four `u32` fields, a `u64` and `[u32; 2]`).
    `entry_count` is at offset 8.
  - An entry is 24 bytes, with `target_id_bytes` at offset 8.
  - The entry capacity is the header's `entry_count`. The identity capacity is the sum of the staged
    `target_id_bytes`. Calling `request_ptr` again sets both capacities to zero, as the real
    `release_collection_staging` does.
  - The worklet constants agree (`miso-engine-v1-audio-worklet.js:60-61`: 32 and 24).
  - The only differences are on paths that the worklet already refuses, or are out of scope. The ceiling is a
    20-entry fake-memory budget in place of `DEFAULT_MAXIMUM_MEMORY_BYTES`. The fake does not write
    `struct_size`/`abi_version`, which the worklet writes itself. `entry_ptr` does not set the identity
    capacity to zero.
  - The fake is wrapped around every boot-contract node, but only a collection boot calls the seven exports. The
    real module has these exports.
  - h2 (wrapper removed): red, `result=255`. p6 (the worklet writes the identity length at entry offset 12): red on
    the new suite, so the sizing finds real staging errors.

## Asked: are the implementer's two reports correct?

1. **Mutation 1 fails at the call, not at the caller-witness assertion. Confirmed.** Without the export,
   `hooks.runStagingReadRun(...)` throws `TypeError: hooks.runStagingReadRun is not a function` inside `expectStop`
   (stack `test-web-audioworklet.mjs:2756`, `expectStop :2720`). `expectStop` throws it again, so the run stops
   before the witness list assertion. No honest test can make the witness list the failing assertion. The gate's
   wording is wrong, not the test (MINOR 2).
2. **"The host's guards refuse that shape" is partly false. Partly confirmed.** The implementer's correction is
   also incomplete.
   - Confirmed: guards that are not specific to this shape are already caught. p1 (the host collection guard
     requires exactly one entry) and p2 (the collection capture is limited to 1 MiB) are red on both suites. The
     main-realm "frozen sparse exact entries" case boots three entries, and "collection capture above single
     limit" boots 1,048,577 bytes.
   - Not confirmed: a host guard that is specific to the shape's distinguishing entry is red only in the new suite.
     p3 (the host collection guard accepts only `output` targets, so it refuses the `trackPostPan` entry) is red on
     the new suite (`real host guard rejected (miso.error.v1 result=1)`) and green on the base suite. In the base
     run, every collection entry is `output`. The only `trackPostPan` collection entry is in `--boot-data-nested`,
     and no gate runs that mode. So this half of the claim is true for a `trackPostPan` collection entry.
   - Missed by both the spec and the implementer: D1's largest unique catch is the worklet's pre-boot collection
     staging. Before this commit, no hermetic test staged a collection in the real worklet: the base fake has no
     collection exports. p4 (header entry count written as 1), p5 (the worklet's collection target map loses
     `trackPostPan`), p6 (identity length at the wrong offset) and p8 (`TextEncoder` in the worklet) are each red on
     the new suite and green on the base suite.
   - The other half of the sentence is too broad. "Red when `runStagingReadRun`'s boot options change" is false
     for the options that D2 does not hold. p9 (`maximumCaptureBytes` changed from 2 MiB to 1 MiB) is green on both
     suites.
   - The implementer's proposed fix ("narrowed to the caller's options") would delete true catches (p3, p4-p6, p8)
     and keep the false one (p9).

## Findings

### MINOR

1. **The spec's Test-value sentence is not true as written. The attempt record's proposed fix is also incorrect.**
   Spec lines 91-93 and 178-180. Root's rule is that every test-value claim in a spec must be true. Probes p3-p9
   above. Corrected sentence for root:

   > *Test value.* D1 and D2 are red when `runStagingReadRun` boots different collection entries or entry order,
   > a non-null `spectrum` or different live-control words; when the host's option guards refuse a `trackPostPan`
   > collection entry; or when the worklet's pre-boot collection staging refuses or mis-stages the two-entry
   > collection (the entry count or an identity length written at the wrong offset, a lost `trackPostPan`
   > mapping, a `TextEncoder` call in the worklet). Today only a browser run shows these. A host guard that refuses
   > more than one entry or a capture budget above 1 MiB is already red in the main-realm spectrum option tests.
   > The collection's `maximumCaptureBytes` and `sourceRingFrames` are not held. D3 is red when any conjunct of the
   > reply check is lost (the tag, the exact field set, `result` 0, a `u32` count) or when the check refuses the
   > `u32` maximum. No test catches these today.

2. **Gate 2, row 1 names the wrong failing assertion.** Spec line 79. Corrected row: "`runStagingReadRun` removed
   from `qualificationBootContract`: the call throws `hooks.runStagingReadRun is not a function` before the caller
   witness assertion." The attempt record (`:163`) and `MUTATIONS.md:580` already give the true failure.

### NIT

1. **One changed line is outside the named functions.** The `mainRealmTextEncoder: originalTextEncoder` argument
   (`test-web-audioworklet.mjs:3487`) is in `testProcessor`, the single call site of `testQualificationBoot`. It is
   necessary, because `TextEncoder` is already removed when `testQualificationBoot` runs. The attempt record
   reports it.
2. **The hazard's statement "runs the real worklet class with the real module" is not correct.** The boot contract
   gives the real worklet class `createFakeExports`, not the module. This is why the fake needed the seven exports.
   The fake also does not check that the boot uses the staged collection. p7 (the worklet skips collection staging
   completely) is green on both suites. Only a browser leg would show it. This is out of this issue's scope, and
   the corrected sentence above does not claim it.
3. **"Accepted" can be the wrong label.** At `:1390` and `:1393-1394`, a refusal with a code other than 255 is put
   in `acceptedAllocationReplies`, and the run reports "the host accepted a malformed render allocation reply".
   The run is still red. Only the message is wrong.

No BLOCKER and no MAJOR.

## Test value (one sentence per new test)

- **D1, the staging-read caller in the boot contract:** red when the worklet's pre-boot staging of the two-entry
  collection refuses it or stages it wrong (p4, p5, p6, p8), or when the host's guards refuse a `trackPostPan`
  collection entry (p3). No existing hermetic test catches these: each is green on the base suite.
- **D2, the staging-read shape assertions:** red when `runStagingReadRun` boots a different collection, with one
  entry (m2) or swapped entries (p10), or a non-null `spectrum` or different live-control words. No existing test
  holds this caller's options.
- **D3, the nine refusal cases:** red when any renderAllocations-specific conjunct of the reply check is lost:
  `result === RESULT_OK` (m3), `validU32` weakened (m4) or removed (m5), the exact field list (m6), the expected tag
  (m7). The base file never names `miso.renderallocations.v1`.
- **D3, the two acceptance cases:** red when the check refuses a valid reply, for example a check that refuses the
  `u32` maximum (p11, `count < 0xffffffff`: "the host refused a render allocation reply with count 4294967295").
  The browser legs send only `count` 0.

## Gate-2 mutations (each applied alone in a copy of the export; new suite and base suite each run against the same mutated product)

| Mutation | New suite | Base suite |
| --- | --- | --- |
| m1 export removed | red: `TypeError: hooks.runStagingReadRun is not a function` (at the call) | green (callers=5) |
| m2 one-entry collection | red: `staging-read spectrum collection changed` | green |
| m3 `result === RESULT_OK` dropped | red: accepted `['result 6']` | green |
| m4 `validU32` -> `Number.isInteger` | red: accepted `['count -1', 'count 4294967296']` | green |
| m5 `validU32(count)` dropped | red: accepted `-1`, `4294967296`, `1.5`, `"0"`, `0n` | green |
| m6 field list + `"extra"` | red: accepted `['an extra field']` | green |
| m7 expected tag -> `miso.status.v1` | red: accepted `['a wrong tag']` | green |

Extra probes: p1 and p2 red on both. p3, p4, p5, p6, p8, p10 and p11 red on the new suite and green on the base
suite. p7 and p9 green on both. h1 and h2 red. All logs are in `evidence/1477-attempt1/mut/`.

## Gates run

- Worklet chain, the exact CI commands, `CARGO_INCREMENTAL=0` with a private target. All passed:
  - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`:
    module `6175e70c63e58df50398bbb3f8e1ccf70da211c57b618e20cf2b654b4646e6b5`.
  - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration ...`: `web AudioWorklet static/object
    checks passed`.
  - `python3 -B scripts/check-browser-expected-resources.py --artifacts ...`: digests agree, self-test passed with
    32 red mutations.
- Gate 1: `bash scripts/test-web-audioworklet.sh` in the CI form (a new private `TMPDIR`, then a check for
  leftovers). Exit 0, no leftovers. Both node passes log `qualification boot contract passed: callers=6
  real-ready=6 real-disposed=6 diagnose-ready=1`. Node is present, so the script did not run bun. For information,
  I ran `bun scripts/test-web-audioworklet.mjs` separately: it passed with callers=6, and the base file passed with
  callers=5.
- Browser leg: `npm run qualify -- --artifacts ... --sdk-root sdk --browser <b> --check-matrix --self-test-mutations`,
  with a private pulseaudio null sink. It logs `sdk bundle: the source at .../sdk/src (CI's mode)`, and `sdk/dist`
  is absent. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 each logged `all qualification gates passed`,
  with `staging-reads-one-shot=0 staging-reads-stream=0` render allocations.
- Gate 3: `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`. The export has no `.git`, so the script
  used its documented `find` fallback.
- Standing rulings:
  - No render-owned memory, queue, parallel API, retry or deadline change.
  - The acked-batch question does not apply: this commit adds no queue, and a malformed reply fails the whole host
    with 255.
  - No test greps source.
  - New names are unversioned. `miso.renderallocations.v1` is the existing wire tag.
