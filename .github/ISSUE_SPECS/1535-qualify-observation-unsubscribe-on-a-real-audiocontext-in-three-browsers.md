# Qualify observation unsubscribe on a real AudioContext in three browsers

Decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Successor S1 of *Hold effect parameter, bypass and EQ-target values in latest-target cells*
(#1345), from root's split ruling (#1345 Amendment 7, root ruling 6, 2026-10-13). Qualification
work only: no host, SDK or engine change. **Fresh five-attempt budget** (AGENTS.md): #1345's
attempts and fix rounds do not count against this issue.

Code anchors are verified at `806de000a` (the last commit that holds the gate; its files are saved
byte-for-byte in `/home/bl/misofm/submix-verdicts/b3-acceptance/split/before/`). The split commit
is `4858c4c14` on `codex/d15-stream-b3`. Verify every anchor again on `main` after #1345 merges,
before the first edit.

## Product outcome

An app that unsubscribes a gain-reduction tap stops seeing that tap's reduction in the browser
meter frames. The shipped AudioWorklet host proves this in the `browser` qualification legs of
Chromium, Firefox and WebKit, on a real `AudioContext` that renders on its own clock:

1. After the unsubscribe acknowledgement (`result === 0`, no bindings), every meter frame the page
   receives has `trackGrDb[0] === 0`. No frame is excluded.
2. The zero is not a zero of silence: at least 48 frames after the acknowledgement carry the fed
   signal above the peak floor. A frame after the acknowledgement at or below the floor is
   counted and reported (root ruling R1), never a failure.
3. A gate failure names its cause: a refused request names its step, its kind and the page's
   in-flight counts; too few signal frames is a *shortfall*; a render that does not move is a
   *stall*. No failure prints `[object Object]` or a bare `result 6`.

## Context

- **Why this is a new issue.** #1345 Amendment 7 removed the gate from #1345 and moved it here.
  The acceptance run of the gate at `806de000a`
  (`/home/bl/misofm/submix-verdicts/b3-acceptance/summary.md`, `analysis.txt`,
  `failures-branch.txt`, `rows.json`, per-run logs in `legs/<leg>/<browser>/NNN-<side>/log`) had
  zero failures unloaded in every browser (60 WebKit runs alone, 60 paired WebKit runs, 10 Firefox,
  10 Chromium) and no stall signature on either side, but on WebKit under load:
  - `loadi-webkit/webkit/012-b3` and `loadii-webkit/webkit/013-b3`: the gate failed because a
    frame after the acknowledgement peaked at 0.0011 and 0.0021, below the 0.05 floor (the fed
    signal dropped out; reduction after the acknowledgement was 0).
  - `loadii-webkit/webkit/001-b3`: page mutation (d) `skip-unsubscribe` was refused for the wrong
    reason: reduction after the acknowledgement was 18.1 dB, but one frame peaked at 0.049, so the
    "only that check" proof failed.
  - `loadi-webkit/webkit/032-b3`: page mutation (e) `feed-ends-at-boundary` died with the page
    error `{"message":"[object Object]"}`.
  - `loadii-webkit/webkit/018-b3`, `019-b3`, `038-b3`: `observation qualification failed:
    {"tag":"miso.error.v1","requestId":0,"result":6,...}` with no step name. WebKit leg ii all
    failures: branch 8 of 40, `main` 2 of 40 (Fisher one-sided p = 0.0436).
- **The saved gate.** `/home/bl/misofm/submix-verdicts/b3-acceptance/split/observation-unsubscribe-gate.patch`
  (`git diff HEAD HEAD^` over the removal commit `4858c4c14`; `README.txt` beside it). It re-adds
  `runObservationUnsubscribeRun`, `unsubscribePlanes`, the `UNSUBSCRIBE_*` constants,
  `unsubscribe-session.json` and its `qualificationSessionSources` row, `unsubscribeVerdict`, the
  page mutations and their `run.mjs` proofs, the four result-field mutations, the eleventh
  render-allocation row, the bounded waits, the trusted-click resume and the stall classifier.
  **It also reverts two things the split kept, which this issue must not revert:** the
  `observation-disarmed` gate and its mutation (`run.mjs` at `4858c4c14`: `:30`, `:233`, `:739`),
  the dated Amendment 7 note and the `observation-disarmed` row of `MUTATIONS.md`, and the
  `resume-clicks` count (`:241-243`, now `0`/`2`). The patch's `STREAMS.md` hunk is root's file:
  do not apply it (it is under `docs/handoffs/`, which D1's exclusion removes).
- **The gate as it was** (`hosts/host-web/qualification/qualification.js` at `806de000a`):
  - constants `:26-40` (`UNSUBSCRIBE_SOURCE_BLOCKS = 512`, `UNSUBSCRIBE_METER_BLOCKS = 1n`,
    `UNSUBSCRIBE_LEAD_BLOCKS = 12`, `UNSUBSCRIBE_ARMED_FRAMES = 4`, `UNSUBSCRIBE_MINIMUM_BLOCKS = 16`,
    `UNSUBSCRIBE_PEAK_FLOOR = 0.05`); `DEFAULT_RING_FRAMES = 5120` at `:6`, so the ring holds
    40 blocks of 128 frames;
  - `unsubscribePlanes` `:138-157` (a +-0.5 square wave);
  - `bounded` `:548-562` (rejects with an `Error` on timeout, but passes through whatever `start()`
    rejects with); `requireSuspended` `:569-578`; `resumeClickPlan` `:591-597`; `countResumes`
    `:608-619`; `resumeThroughClick` `:629-642`;
  - `classifyStallSignature` `:659-686`; `boundedSample` `:692-706` (gives up on a status read
    after 1 s and leaves it in flight); `resumeUntil` `:710-713`; `renderUntil` `:725-754` (polls
    `describe()` every 250 ms); `sampleContext` `:758-764`;
  - `runObservationUnsubscribeRun` `:800-963`: the feed `:829-871` (`submitBlocks` sends every
    block up to the target with no in-flight limit and no check against the render position;
    `requireFeed` `:862-864` throws `feedFailure`, which can be a frozen `miso.error.v1` record,
    not an `Error`); the frame-driven top-up `:866-877`; `renderedQuanta` `:879` (a new
    `host.status()` per call); the resume and `R1` wait `:894-907`; the unsubscribe `:914-918`; the
    post-acknowledgement wait `:927-937` (ends at 16 *blocks*, a multi-block frame counting all its
    blocks);
  - `unsubscribeVerdict` `:1365-1381`; page mutations `:1387-1400`; the observation catch in
    `runQualification` `:1279-1293` (spreads `...error` into the message, which is how a raw record
    became `{"tag":"miso.error.v1","requestId":0,"result":6}`).
- **The gate in `run.mjs` as it was** (`806de000a`): `MUTATIONS` `:26-52` (the unsubscribe entries
  `:34-37`); `PAGE_DEADLINE_MS` `:57`; the gate `:237-249`; `resume-clicks` `:255-262`; `mutate`
  `:753-761`; `stallSignatureProofs` `:815-844`; `mutationProofs` `:846-873` (it checks only a `<browser>:` prefix for these entries, `:854`); `unsubscribePageMutationProofs` `:957-1015` (the
  "only that check" proof `:1004-1011`).
- **Where `result 6` with `requestId 0` comes from.** It is a **local** refusal of the main-realm
  host, not the observation queue. `#request`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1218-1263`) calls `#saturated`
  (`:1174-1192`) before it allocates a request ID and rejects with `webError(RESULT_BACKPRESSURE)`
  (`:1228-1229`; `webError` `:522-526` defaults `requestId` to 0). Its bounds: one `status` in
  flight (`:1179`), at most `#ringBlocks` source chunks in flight per source (`:1175-1177`). The
  page breaks both: `boundedSample` abandons a status read that is still in flight, so the next
  `renderedQuanta()` is refused; and the feed has no in-flight limit. The engine's observation
  refusal (an `Observe` record for a tap whose slot is not drained) is a resolved acknowledgement
  with `reason` backpressure (#1345 D2 Amendment 1), never `requestId 0`.
- **The source ring accepts only contiguous blocks.** The submit check refuses a chunk whose
  `start_frame` is not the next write frame (`crates/source/src/lib.rs:954-959`,
  `HostChunkError::NonContiguous`), so the page cannot skip a block without a seek. A block that
  arrives after the render passed its frame is discarded by `settle_block` (`:1453-1474`) and the
  render plays zeros for it.
- **What stays from #1345.** The contract text (both `.d.ts` twins, the `WebMeterHeader` and
  `meter_frame` docs), the Rust test
  `a_withdrawn_tap_leaves_the_first_polled_frame_and_only_its_own_reading`
  (`hosts/host-web/src/tests.rs`), the `observation-disarmed` and `observation-identity` gates.
  #1345 Amendment 6's reopening condition stands: if control moves to a Worker (H #1332), the
  ordering that makes "received after the acknowledgement" mean "polled after the disarm" must be
  proved again.
- **Method of the acceptance run.** `/home/bl/misofm/submix-verdicts/b3-acceptance/scripts/`
  (`one.sh`, `leg.sh`, `master.sh`, `acc1.sh`, `analyze.py`): the CI browser command
  (`npm run qualify -- --artifacts ... --sdk-root sdk --browser <name> --check-matrix
  --self-test-mutations`), `sdk/dist` removed (source-bundle mode, as CI), a private PulseAudio
  null sink and a fresh process per run, leg i = 32 busy loops with the browser unpinned, leg ii =
  4 busy loops and the browser on `taskset -c 0-3`, strictly alternating A (`main`) and B
  (branch) runs.

## Decisions frozen for this slice

- **D1. Start from the patch, keep what the split kept.** Apply the saved patch on the slice's base
  (`main` with #1345 merged) with
  `git apply --exclude='docs/handoffs/*' /home/bl/misofm/submix-verdicts/b3-acceptance/split/observation-unsubscribe-gate.patch`
  (as one whole, `git apply --check` fails on the `STREAMS.md` hunk; the exclusion drops that hunk
  and the rest applies cleanly). The keep-list still applies after
  the patch: restore the `observation-disarmed` gate and its mutation as they
  are on `main` (D9 then adds its frame count), and set `resume-clicks` to `declared === (result.sdkResponse === null ? 1 : 3)`
  (the two SDK clicks and the gate's one click). The `observation-unsubscribe` gate does not check
  the disarmed run again (`observation-disarmed` owns that). Do not apply the patch's `STREAMS.md`
  hunk.
- **D2. Root ruling 5's conditions stand unchanged.** One `AudioContext`, suspended before boot (its
  first `suspend()` not awaited; `requireSuspended` polls), resumed once by one trusted click, and
  never suspended again before the final close. `R1` is the host's `renderedQuanta` read after the
  subscribe resolves; the unsubscribe goes out only after a status read exceeds `R1`.
  `trackGrDb[0] === 0` on every frame received after the acknowledgement, none excluded. A frame
  after the acknowledgement that peaks at or below `UNSUBSCRIBE_PEAK_FLOOR = 0.05` is reported, not
  failed, and does not count toward D4's 48 frames (root ruling R1). The source streams
  just-in-time from `unsubscribe-session.json` (512 blocks); no seek. The one-block meter window
  (`UNSUBSCRIBE_METER_BLOCKS = 1n`) stays. No retry, no looser bound, no second resume.
- **D3. Request discipline (reviewer finding A).**
  1. Every host request the run makes goes through one helper that records the kind
     (`source`, `status`, `observe`, `meters`, `renderAllocations`, `dispose`), keeps the page's own
     in-flight count per kind, and bounds the wait with `bounded`. Any rejection or timeout is
     rethrown as an `Error` whose message is
     `request refused at <step>: kind=<kind> inFlight=<JSON of the per-kind counts> cause=<cause>`,
     where `<cause>` is `diagnosticJson` of a `miso.error.v1` record's `tag`, `requestId` and
     `result`, or an `Error`'s `message`. A raw record is never thrown or stored as a failure.
  2. `requireFeed` throws the same `Error` form; a feed failure names the block index. `bounded`'s
     timeout message keeps its form. The observation catch in `runQualification` prints `message`
     and `stallSignature` only (no `...error` spread of a record).
  3. **A status read is never abandoned while in flight.** The run holds at most one status read.
     `renderedQuanta()`, `sampleContext` and the stall poll call one `statusRead()`, which returns
     the outstanding promise when a read is in flight and sends a new one only when none is. A poll
     that stops waiting after `STALL_POLL_TIMEOUT_MS` (`boundedSample`) leaves the read outstanding;
     the next poll reuses it. With this rule the host's one-status bound can never refuse the page.
  4. **The feed never has more source requests in flight than the ring holds in blocks**
     (`RING_BLOCKS = DEFAULT_RING_FRAMES / QUANTUM_FRAMES = 40`). The page counts its own unsettled
     submits and sends no block while that count is 40; the next frame or acknowledgement resumes
     the feed.
  5. **The feed never sends a block the render has already passed.** The page's render position is
     the larger of the newest frame's `endSample / 128` and the newest status `renderedQuanta`. When
     the next block to send has an index below that position, the run records
     `feedBehind = { block, renderBlock }`, stops feeding and goes on to its verdict (the source
     cannot skip a block without a seek, Context).
  6. `UNSUBSCRIBE_LEAD_BLOCKS` becomes 32 (85 ms at 48 kHz). The ring holds at most 32 unrendered
     blocks, below its 40, so a submit is never refused as full; a longer main-thread stall than
     the old 12-block lead (32 ms) is tolerated. The lead no longer has to be short for mutation
     (e): D4's 48 frames exceed the ring depth whatever the lead.
- **D4. After the acknowledgement (reviewer finding B).** The run waits, under **one** deadline of
  `AUDIO_STEP_TIMEOUT_MS` (10 s) from the acknowledgement, until at least
  `UNSUBSCRIBE_SIGNAL_FRAMES = 48` frames received after the acknowledgement peak above the floor
  (recorded as `signalFramesAfterAcknowledgement`). 48 is more than the ring's 40 blocks, so a feed that stopped at the acknowledgement cannot supply
  them from the ring alone. **A multi-block frame counts as one** (Firefox and WebKit can close one
  frame over several blocks). At the deadline the run reads the status once more:
  - if `renderedQuanta - boundary >= 48`, the render moved but the signal frames are short: the
    run returns its result with `signalShortfall: true` (the named failure `shortfall`, D5);
  - otherwise the render did not move enough: the run throws the existing named stall failure with
    its `stallSignature` (`classifyStallSignature`, unchanged).
  A shortfall is never reported as a stall, and a stall never as a shortfall. A recorded
  `feedBehind` (D3.5) ends the wait at once, and the run returns its result for the verdict.
- **D5. The verdict names its failures.** `unsubscribeFailures(run)` (exported from
  `qualification.js`) returns the list of failed checks, empty on a pass; `unsubscribeVerdict(run)`
  is `unsubscribeFailures(run).length === 0`. A dropout is not a failure (root ruling R1). Names and checks:
  - `context`: `contextStateBeforeBoot !== "suspended"`;
  - `armed`: meter lease or subscribe result not 0, `armedFrames < 4` or
    `armedMaximumTrackGrDb <= 0`;
  - `render-between`: `renderedBeforeUnsubscribe <= renderedAfterSubscribe` (decimal strings);
  - `unsubscribe-ack`: `unsubscribeResult !== 0` or `unsubscribeBindings !== 0`;
  - `reduction`: `maximumTrackGrDbAfterAcknowledgement !== 0` (over every frame after the
    acknowledgement), or `framesAfterAcknowledgement < 1` (a run that received no frame proves
    nothing);
  - `shortfall`: `signalShortfall === true` or `signalFramesAfterAcknowledgement < 48`;
  - `feed`: `feedBehind !== null` or `maximumSourcesInFlight > 40`.
  The `run.mjs` gate message lists the names first, then the result JSON.
  The result also records `reducingSignalFramesAfterAcknowledgement` (frames after the
  acknowledgement with `trackGrDb > 0` and a peak above the floor), `framesAfterAcknowledgement`,
  `lowPeakFramesAfterAcknowledgement` (frames at or below the floor) and
  `minimumPeakAfterAcknowledgement`; the last two are reported in the gate message and fail
  nothing (root ruling R1).
- **D6. Page mutations prove the cause, not "only one check".**
  - (d) `skip-unsubscribe`: the failures include `reduction`, and
    `reducingSignalFramesAfterAcknowledgement >= 1` (the GR check fails on frames **above** the
    floor). A dropout in the same run does not make the proof fail. This replaces the "accepted when
    only that field is repaired" proof, which run `001-b3` broke.
  - (e) `feed-ends-at-boundary`: the run returns a result (it does not throw) and the failures
    include `shortfall`.
- **D7. Stated limit (carried from #1345 Amendment 6).** A single-block dropout inside a multi-block
  frame can be masked by the other block's peak. The reported low-peak count shows a feed that
  stops or falls behind, but it fails nothing (root ruling R1); mutation (e) and `shortfall` prove the signal
  continued past the ring. The host publishes no source-underrun count today; when S3 (*Report source
  underruns in the browser status*) is on `main`, a follow-up adds "no underrun block after the
  acknowledgement" to this gate. Not in this slice.
- **D8. No product change.** A gate that is red on the unmutated tree because of the host, the SDK
  or the engine is a finding: stop, record it, report to root. Do not edit `hosts/host-web/web/`,
  `hosts/host-web/src/`, `sdk/` or `crates/`.
- **D9. The zero-frame hole (batch verdict 3, MINOR 2; main has it too).** The gate must not pass
  on a run that received no frame.
  1. `observation-disarmed` (`run.mjs:233`) requires `observation.disarmed.frames >= 1` as well as
     `maximumTrackGrDb === 0`; `runObservationRun` (`qualification.js`) returns `frames`
     (`frames.length`) in its result for it.
  2. The `observation-unsubscribe` checks after the acknowledgement require
     `framesAfterAcknowledgement >= 1` (D5 `reduction`); D4's 48 signal frames already bound the
     real run from below.
  3. New result mutation `observation-disarmed-empty`: `observation.disarmed.frames = 0` -> the
     `observation-disarmed` gate fails (red); the unmutated run stays green.

## Deliverables

1. The patch applied per D1; D3-D6 in `qualification.js` (`runObservationUnsubscribeRun`, the
   request helper, `statusRead`, the feed, `unsubscribeFailures`, `unsubscribeVerdict`,
   `runUnsubscribePageMutation`) and the `runQualification` catch.
2. In `run.mjs`: the `observation-unsubscribe` gate on `unsubscribeFailures`, the result-field
   mutations of gate 2 (with `mutationProofs` requiring each entry's named
   failure in the message), the page-mutation proofs of D6, `stallSignatureProofs()` back in
   `--self-test-mutations`, the new `requestFailureProofs()` of gate 4, the eleventh render-allocation
   row, `resume-clicks` per D1, and the `observation-disarmed` frame count with its
   `observation-disarmed-empty` mutation (D9).
3. `unsubscribe-session.json` re-added; its `qualificationSessionSources` row re-added.
4. `generate-matrix.mjs` and `BROWSER_DEPLOYMENT_MATRIX.md`: the observation sentence says the gate
   requires both a never-armed tap with no reduction and an unsubscribe that stops the reduction
   (regenerate the matrix with the generator; do not hand-edit the `.md`).
5. `hosts/host-web/MUTATIONS.md`: one new dated section with a row per result mutation, per page
   mutation and per source mutation (gate 6), in the format of the #1530 rows; the history rows the
   patch restores keep their notes.
6. This spec's Attempt record: the acceptance tables of gate 8 and the per-browser
   `render-allocations` lines.

## Authorized paths

- `hosts/host-web/qualification/qualification.js`
- `hosts/host-web/qualification/run.mjs`
- `hosts/host-web/qualification/unsubscribe-session.json` (re-added, as saved)
- `hosts/host-web/qualification/generate-matrix.mjs`, `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
  (the observation sentence only)
- `hosts/host-web/MUTATIONS.md` (the new section, and the patch's restored history rows)
- This spec.
- Nothing else (D8). `scripts/test-web-audioworklet.sh` must still pass on the edited page; if one
  of its scoped `qualification.js` transforms no longer matches exactly one line, stop and report:
  that script is not in this slice.

## Non-goals

- No change to the host, the worklet, the SDK or the engine. Sharing one status read inside the host
  is S2's (*Share one in-flight status read in the browser host*); this slice's `statusRead` is
  page-local and stays correct with or without S2.
- No source-underrun counter (S3), no seek, no second resume, no `OfflineAudioContext` variant.
- No fix for #1522's spectrum gates, #1531's WebKit clock-stall diagnosis or #1533's page-error
  rule. A run that fails only in a gate those issues own is recorded apart (gate 8).
- No proof of the Worker ordering (H #1332's reopening condition).

## Objective gates

Gate names are the `gate(...)` names in `run.mjs`. Each test-value sentence answers: which plausible
defect turns this red that no existing test catches?

1. **`observation-unsubscribe` on the real result**, in each browser: `unsubscribeFailures` is empty;
   `signalFramesAfterAcknowledgement >= 48`; `renderAllocations === 0` (the eleventh row).
   *Test value:* turns red if an unsubscribe that the shipped worklet acknowledged with `result 0`
   leaves its tap folded into any later meter frame on a real render clock (for example a fold that
   ages a withdrawn window out over `windowBlocks`, or an acknowledgement posted before the disarm);
   the Rust test reads `poll_meters` natively, not the shipped worklet's one FIFO port, and
   `observation-disarmed` never subscribes.
2. **Result-field mutations** (`--self-test-mutations`; each in `MUTATIONS`, made in `mutate`; each
   must make `validate` throw a message that starts `<browser>: observation-unsubscribe:` and
   contains the named failure):
   - `observation-unsubscribe`: `maximumTrackGrDbAfterAcknowledgement = 1` -> `reduction`;
   - `observation-unsubscribe-empty`: `framesAfterAcknowledgement = 0` (with the maximum
     reduction left at 0) -> `reduction`;
   - `observation-disarmed-empty`: `observation.disarmed.frames = 0` -> the `observation-disarmed`
     gate (D9; message prefix `<browser>: observation-disarmed:`);
   - `observation-unsubscribe-short`: `signalFramesAfterAcknowledgement = 47` -> `shortfall`;
   - `observation-unsubscribe-norender`: `renderedBeforeUnsubscribe = renderedAfterSubscribe` ->
     `render-between`;
   - `observation-unsubscribe-inflight`: `maximumSourcesInFlight = 41` -> `feed`;
   - `observation-unsubscribe-behind`: `feedBehind = { block: 20, renderBlock: 21 }` -> `feed`.
   There is no `-silent` mutation: a dropout is reported, not failed (root ruling R1). A node proof in
   `--self-test-mutations` (`unsubscribeFailures` on an otherwise green run with
   `lowPeakFramesAfterAcknowledgement = 3` and `minimumPeakAfterAcknowledgement = 0`) must return an
   empty list.
   *Test value:* turns red if a check of D5 is dropped, weakened (for example 48 back to 16) or
   attributed to the wrong name, if a dropout starts to fail the gate (root ruling R1), or if a run with no frame
   passes (D9); the real green result cannot show it.
   *Mutation:* delete the `shortfall` check from `unsubscribeFailures`: the `-short` entry escapes.
3. **Page mutations (d) and (e)**, in each browser, under `--self-test-mutations` (D6).
   *Test value:* (d) turns red if the GR check can be satisfied while a still-armed tap reduces
   frames that carry signal (the run `001-b3` shape); (e) turns red if a feed that stops at the
   acknowledgement can pass, that is, if the 48-frame count is lowered to the ring depth or counts a
   multi-block frame as several, or if the stopped feed ends in a page error instead of a result.
   *Mutation:* change `UNSUBSCRIBE_SIGNAL_FRAMES` to 16: (e) escapes in every browser (the ring
   holds up to 32 fed blocks).
4. **`requestFailureProofs()`** (node, in `--self-test-mutations`, no browser): the exported
   request-failure formatter, given a frozen `{ tag: "miso.error.v1", requestId: 0, result: 6 }`, the
   step `status`, the kind `status` and in-flight counts `{ status: 1, source: 3 }`, returns an
   `Error` whose message contains `at status`, `kind=status`, `"result":6`, `"requestId":0` and the
   counts, and does not contain `[object Object]`; given an `Error("x")`, the message contains `x`.
   *Test value:* turns red if a refusal record reaches a page error raw (`loadi-webkit 032-b3`:
   `[object Object]`) or without its step (`loadii-webkit 018-b3`: a bare `result 6`).
5. **`stallSignatureProofs()`** restored unchanged from the patch, in `--self-test-mutations`.
   *Test value:* as recorded in #1345 ("B2 second round", item 4): red if the classifier calls a
   record `full` that lacks one of its conditions.
6. **Source mutations (PR evidence, rows in `MUTATIONS.md`; not committed).** Apply one at a time,
   run the CI browser command in each browser, record the red gate and its message, revert:
   - M1: `statusRead()` always sends a new read, and `STALL_POLL_TIMEOUT_MS = 1` -> red with
     `request refused at status: kind=status` (in at least one browser; record which).
   - M2: no in-flight guard and `UNSUBSCRIBE_LEAD_BLOCKS = 60` -> red with
     `request refused at submitSource: kind=source` or a feed failure naming its block.
   - M3: the post-acknowledgement wait counts blocks (`(endSample - firstSample) / 128`) instead of
     frames -> record the outcome; if it stays green in every browser, say so (it is not hidden).
7. **`resume-clicks`** passes with `declared` 3 (SDK on) and 1 (SDK off), and its three mutations
   stay red. *Test value:* unchanged from #1345 (red if a real context is resumed outside a trusted
   click or the harness clicks other than the declared controls).
8. **Acceptance (root's bars).** On the slice's head, measured as in Context ("Method"):
   - **Unloaded:** WebKit 60 runs, Firefox 10, Chromium 10: zero failures of any gate.
   - **Loaded, paired A/B against `main`** (the merge base), strictly alternating: leg i and leg ii,
     WebKit and Firefox 40 pairs per leg, Chromium 10. For each leg and browser, report pass/fail
     counts by first failing gate and named failure, the Fisher one-sided p for "branch failures >=
     observed" over all failures together, and the load at start and end. The bar: no
     `observation-unsubscribe` failure other than a stall, and no all-failures rise with p < 0.05.
   - **Stop condition:** any `stall-signature` line with `"full":true` on the branch stops the
     measurement and goes to root. No retry, no exemption claimed by this slice.
   - A failure in a gate that #1522 or #1531 owns is counted and listed apart, on both sides.
9. **Workspace.** `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-workspace-policy.sh`;
   `node --check` on the two edited modules; `sdk-render-allocations` stays green with its frozen
   rows, all 0.

## Dependencies

- **#1345 on `main`** (with Amendment 7's split): the base this slice applies the patch to.
- **Before H #1332** (the reopening condition: the gate must exist on the single-instance shipped
  host before control moves to a Worker, so #1332 inherits a gate to keep green).
- **Not on S2 or S3.** D3.3 is page-local, so S2's host change does not change this page's polling.
  S3 adds a stronger check later (D7).
- **Same files as H #1522, H #1530, H #1533** (`run.mjs`, `qualification.js`, `MUTATIONS.md`): in any
  order; the later slice rebases.

## Placement (root edits `STREAMS.md`)

Recommended: **stream H**, a qualification slice after B #1345 reaches `main` and before H #1332.
- Every authorized path is stream H's (`hosts/host-web`), so no named exception is needed, unlike
  the gate's time in #1345.
- "Before H #1332" is then an order inside one stream.
- Stream B's queue after #1345 (#1523, #1524, #1525, #1534) changes no file of this slice.
- Hot-file rows (line numbers of `STREAMS.md` at `4858c4c14`): `qualification.js` row (`:110`): H S1 re-adds the gate from the saved
  patch, in any order with H #1522, #1530 and #1533. `run.mjs`/`sdk-response-entry.ts` row (`:109`):
  H S1 (`run.mjs` only). `MUTATIONS.md` row (`:111`): H S1 (its own section). Re-add the
  `unsubscribe-session.json` row that Amendment 7 removed, naming H S1.

Alternative: stream B, if root wants the proof to stay with the observation-queue owner; it then needs
the named exceptions of #1345 Amendments 5 and 6 again.

## Attempt record

- **Attempt 1 (2026-10-24, base `40e3b61e9`).** Written against today's API, not applied from the
  saved patch (it does not apply: `qualification.js` and `run.mjs` moved; its subscription used
  `trackIndex`/`effectIndex`, #1677 addresses taps by `stripId`/`rack`/`effectId`; its session
  lacked `control_smoothing`). Unmutated tree, one unloaded run each, Playwright 1.62.1,
  `--sdk-root --check-matrix --self-test-mutations`: green in Chromium 151.0.7922.34, Firefox 153.0
  and WebKit 26.5 (`resume-clicks` 3); without the SDK green in Firefox and WebKit
  (`resume-clicks` 1). Per browser (`observation-unsubscribe:` line): signal frames after the
  acknowledgement 48 / 49 / 49, frames after it 48 / 63 / 55, low-peak frames 0, minimum peak
  0.0815, `maximumSourcesInFlight` 32. `render-allocations`: all fourteen rows 0 in every browser,
  `observation-unsubscribe=0`; `sdk-render-allocations` rows all 0.
- **Attempt 1 reds on the unmutated tree (verifier MAJOR, recorded, not retried away).** Two Firefox
  153.0 runs of attempt 1 were red, both with the PulseAudio socket at a 113-character path (over
  the 108-byte `sun_path` limit, so no sink existed):
  - without the SDK (`n-firefox.log`): `observation-unsubscribe qualification failed: resume
    timeout at the armed tap to render (click) after 10000 ms: state=suspended currentTime=0`;
  - with the SDK (`r-firefox.log`): `spectrum collection audio resume timed out (gesture=true,
    state=suspended)`, before the new run.
  `main` (`40e3b61e9`) with the same socket path was red in Firefox with the second message
  (`base-firefox.log`), and green once the socket path was short (`base-firefox2.log`). Chromium
  and WebKit were green under both socket paths. Logs: `scratchpad/b1535/`.
- **Attempt 2 (verifier MINOR and NITs).** When the feed would pass the end of the source, the D4
  wait now ends and the run returns `sourceExhausted`, which the gate names `shortfall` (one more
  node table row). Before this it was a thrown `Error`, which the gate reported as
  `browser-execution`. The peak-floor comment gives the measured minimum, 0.0815.

## Decision record (root, 2026-10-24)

- **Scope reduction accepted** (owner rulings "tests overboard" and feature sprint). Kept: the
  run (D2, D3.1-2, D3.4-6, D4), `unsubscribeFailures` (D5), the gate and its fourteenth
  `render-allocations` row, `resume-clicks` `1 : 3`, D9, `unsubscribe-session.json` from today's
  observation grammar, the matrix sentence, one `MUTATIONS.md` section.
- **One node proof replaces** gate 2's seven result mutations and gate 4's
  `requestFailureProofs()`: a table over `unsubscribeFailures` (one row per D5 name at its bound,
  one R1 row expecting `[]`), the constant relation 48 > 40, and the `requestFailure` form.
  `classifyStallSignature` and `stallSignatureProofs()` are restored (D4 needs stall vs shortfall).
- **Moved to #1605** with gate 8: page mutations (d) and (e) (gate 3) and source mutations M1-M3
  (gate 6). `runUnsubscribePageMutation` and `unsubscribeVerdict` are not added: nothing calls them.
- **D3.3 dropped**: the host shares one in-flight status read (S2 landed), so the page needs no
  `statusRead`.
- The reduction is checked on every frame received after the acknowledgement up to the dispose;
  the signal and low-peak counts on the frames of the D4 wait, while the feed runs.

## Root rulings (2026-10-13)

These override any conflicting text above.

- **R1. A dropout is reported, not failed.** A frame after the acknowledgement that peaks at or
  below the floor does not fail the gate and does not count toward D4's 48 frames. The gate
  records the count of such frames and the minimum peak. `trackGrDb == 0` stays required on every
  frame. Non-vacuity comes from the 48 frames above the floor and from self-test (d).
- **R2. Feed lead 32 blocks** (85 ms), below the 40-block ring, as D-item 6 states.
- **R3. Loaded bar.** No `observation-unsubscribe` failure except a full-signature stall (which
  is listed and routed to #1531), and no all-failures rise against main at one-sided Fisher
  p < 0.05.
- **R4. Placement.** Stream H, after #1345 is on main, before H #1332.

## Amendment (decision 17, root 2026-10-13)

Gate 8 (loaded paired A/B acceptance) moves to #1605, *Post-sprint qualification of decision 15's heavy gates* (root, 2026-10-13, Q8). This slice closes on gates 1-7 and 9 with its CI rows green.
