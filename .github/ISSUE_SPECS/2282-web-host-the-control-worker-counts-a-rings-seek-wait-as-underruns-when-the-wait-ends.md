# Web host: the control Worker counts a ring's seek wait as underruns when the wait ends

Child of #2258 (implicit state machines), item WEB-5. Re-verified at base `1aee7e20d`.

## Mission
In `worker` mode the control Worker drains the SDK's MSB1 rings (`hosts/host-web/web/miso-engine-v1-control-worker.js`). In `single` mode the worklet prelude drains them (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`). Both must count a ring that waits for its seek the same way: as nothing. The Worker's seek state is also a loose bag of fields. It becomes one explicit model.

## What the audit claimed, and what holds
**The claimed wedge does not hold.** The claim was that an anchored seek whose `seek-at` is refused or never sent gets BACKPRESSURE on every tick. That wait is the designed contract, and it is the same in both copies:
- The prelude documents it at `miso-engine-v1-pcm-feed-worklet.js:630-637`: "a refused `seek-at` leaves it unseen until a newer ring seek supersedes it".
- The SDK's public contract documents it at `sdk/src/browser/pcm-feed.ts:133-135`: "Until it did, the drain reads none of that ring's chunks".
- `Msb1RingWriter.seek` documents it at `sdk/src/browser/pcm-ring.ts:47-49`. This is the one place a host that writes a raw ring is told: "an anchored seek published without that message stalls the ring (the drain reads none of its chunks)".
- The shared consumer case table pins it for both copies (`sdk/test/msb1-consumer-cases.mjs`, "an anchored seek waits for the handler's OK for its exact tuple").
- The SDK never leaves such a ring open. A refused `seekSourceAt` throws (`pcm-feed.ts:602`). `ready()` catches the throw and closes the feed (`pcm-feed.ts:630`), and the close posts `detach`, so the Worker withdraws the ring.
- Reading a slot during the wait would submit the new generation's PCM to a source the engine never sought. So the wait is correct.

**The drift risk is already guarded.** The two copies share one case table (#1387 MAJOR-2). `browser-pcm-evals.mjs` runs it against the prelude, and `scripts/test-web-audioworklet.mjs` runs it against the Worker. The copies also differ on purpose:
- The prelude submits two chunks a block and counts each block as it renders it.
- The Worker submits until the engine pushes back, keeps the headroom free, and counts on the engine's own counts.

**The prelude has no defect.** A waiting pass returns before `countSharedBlock` (lines 454, 458 and 543), so its blocks count nothing.

**What holds: the Worker's accounting of the wait.** The Worker's comment at `control-worker.js:1125-1126` promises what the prelude does: "a ring waiting for its seek ... counts no drain block and no underrun". The code breaks that in two ways:
1. **Late underruns.** A waiting pass zeroes `pendingElapsed` and returns (lines 1129 and 1135). It never reads the source report, so `reportUnderrun` keeps its pre-wait baseline. If the engine runs dry during the wait, the first pass after the wait computes `report.underrun - reportUnderrun` (line 1229) and counts every frame the engine missed during the wait as this ring's underruns.
   - This is reachable in the SDK flow. A start's `prime` runs between the anchored publication and `seek-at`, and a live writer's source may run dry in that time.
   - It is red on base in the shared case table: the Worker counts 3 underruns where the prelude counts 0.
2. **Lost carried quanta.** A pass whose report or fill read is refused carries its rendered quanta in `pendingElapsed` (line 1218). Those quanta were rendered before the wait, and the prelude would have counted them. A waiting pass zeroes them, so `DRAIN_BLOCKS` loses them.

**The model.** The ring's seek is held in 15 loose fields:
- `seenEpoch`;
- eight snapshot fields: `seekEpoch`, `seekTag`, `seekWrite`, `publishedGeneration`, its `High` and `Low` halves, `seekFrame` and `seekAnchor`;
- three anchored fields with a `NO_ANCHOR` sentinel: `anchoredGeneration`, `anchoredFrame` and `anchoredSample`;
- `seekGeneration` and `seekAdmit`.

Nothing stops `seekAdmit` being true while no seek is landing. `excludedFrames` is written and never read.

## Decision
- **`ring.seek`** is one tagged union, the drain's applied seek:
  - `{ phase: "settled", epoch }`: `epoch` is the last epoch the drain applied (at attach, the ring's own epoch), and it is in effect;
  - `{ phase: "landing", epoch, generation, admit }`: `epoch` is applied, and render does not yet report its `generation`. `admit` (a plain seek's) lets the new generation fill the headroom (root ruling 1).

  It replaces `seenEpoch`, `seekGeneration` and `seekAdmit`. `consumeSharedSeek` enters `landing`. `accountBlocks` returns it to `settled` when render reports the generation.
- **Waiting is a pass outcome, not a stored phase.** A pass waits when the publication is in flight, or when it is newer than `seek.epoch` and cannot be applied. Both facts are read again every pass, from the ring and from `anchoredSeek`. A stored `waiting` phase would be a second copy of them (pattern 2 of #2258). A newer seek that waits makes an older landing moot: no slot of the old generation is submitted again, and the next applied seek starts its own landing.
- **`waitBlocks(ring)`** is the one place a waiting pass accounts, for both the torn-publication wait and the apply wait:
  - The quanta rendered since the last pass are dropped: they are not drain blocks.
  - `pendingElapsed` is kept, because it was rendered before the wait.
  - The underrun baseline is dropped (`reportUnderrun = null`). The next report read starts a new baseline, so the frames the engine missed during the wait are never this ring's underruns.
  - The carried quanta keep their drain blocks but lose their underruns. When the report read was refused, the engine's cumulative count cannot tell the frames missed in the carried interval from those missed in the first waiting pass's interval. Closing the baseline on the wait's first pass would count the second as underruns, and #1387's rule gives that interval to the wait. When only the fill read was refused, the report was available and the two could have been split there. `accountBlocks` reads neither on a refused pass: that is pre-existing #1387 behaviour, and it is noted rather than changed (attempt 2 review). A carry needs a refused read just before a wait, so it is rare, and the rule errs toward not counting. `waitBlocks`' comment records this.

  At the wait's edges this is accurate to one Worker tick, the granularity the Worker's accounting already has (#1387 attempt 3).
- **`ring.publication`** is one record for the seqlock snapshot (`epoch`, `tag`, `write`, `generation`, `generationHigh`, `generationLow`, `frame`, `anchor`). `readSharedSeek` updates it in place.
- **`ring.anchoredSeek`** is either `null` or `{ generation, frame, sample }`. It replaces the three fields and the sentinel.
- **`excludedFrames` is removed.** The finished-region and idle-writer rule stays: their missing frames are simply not counted.
- **The prelude is unchanged.** It has no defect. A union with an object allocated on every transition would allocate inside `process()`, which realtime render forbids. The shared case table is the drift guard, and the new case is in it.

## Non-goals
- One generated source for both consumers. The control Worker is a pinned engine artifact (`scripts/build-web-audioworklet.sh`, the SDK's `ARTIFACT_SET`), and the prelude is an SDK asset copied as it is (`sdk/codegen/stage-package.mjs`). Sharing a module across the two is a packaging change in both builds, and so a separate issue.
- Changing the designed wait itself (an anchored seek waits for its `seek-at` OK; a refused plain seek is asked again every pass).
- WEB-7 (the Worker's boot, ready and dispose phases), `miso-engine-v1-audio-worklet-host.js` and `web/stem-store/`.

## Gates
- A new shared case in `sdk/test/msb1-consumer-cases.mjs`: "a ring waiting for its seek counts no drain block and no underrun, though the engine runs dry meanwhile". It must be red on base for the Worker and green for the prelude on base.
- A second shared case, next to the #2184 one: "a ring whose seek publication is in flight counts no drain block and no underrun, though the engine runs dry meanwhile" (the torn arm of the wait; attempt 2). It must be red on base for the Worker and green for the prelude on base.
- The guard table under attempt 2: every guard in the touched functions names the test that goes red when the guard is deleted, or why it is equivalent.
- The Worker-only #1387 row in `scripts/test-web-audioworklet.mjs` checks that the quanta a refused report read carried into an anchored wait still count once the wait ends. It must be red on base. It replaces that row's "the quanta of the wait pass uncounted" assertion, which the shared case now holds for both copies.
- `bash scripts/test-web-audioworklet.sh`.
- `node --test test/browser-pcm-evals.mjs` in `sdk/`.
- `bash scripts/check-sdk-headless.sh` against a fresh `scripts/build-web-audioworklet.sh` build.
- `bash scripts/check-workspace-policy.sh`.
- No Rust changes. The realtime policy tool, `cargo` gates and the no-bit-moved comparison do not apply: the Worker is the control thread, and no render code or PCM path changes.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `1aee7e20d`.

**Red on base.** Both tests were added before the fix, and were run against the base Worker.

`node scripts/test-web-audioworklet.mjs` stops at the first failure, the Worker-only row:
```
AssertionError [ERR_ASSERTION]: the two carried quanta count, the wait's three do not

4 !== 6
```
With that one assertion disabled, the shared case fails next, on the Worker:
```
AssertionError [ERR_ASSERTION]: nor once it ends: the blocks the engine ran dry in were the wait's
+ actual - expected

  [
+   3,
-   0,
    2
  ]
    at run (sdk/test/msb1-consumer-cases.mjs:382:14)
```
The same shared case is green on the prelude on base: `node --test --test-name-pattern="MSB1 consumer" test/browser-pcm-evals.mjs` passes all 16 cases. Before any test was written, a probe case gave the same split: `UNDERRUNS, DRAIN_BLOCKS` after the wait was `[0, 2]` for the prelude and `[3, 2]` for the Worker.

**Mutations of the fix.** Each mutant of the Worker ran the hermetic runner through `MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`.

| Mutation | Result |
|---|---|
| `waitBlocks` keeps the underrun baseline (= base for late underruns) | Red, shared case: `[3, 2]` vs `[0, 2]` |
| `waitBlocks` also zeroes `pendingElapsed` (= base for carried quanta) | Red, Worker row: `4 !== 6` |
| `accountBlocks` never returns `landing` to `settled` | Red, existing headroom rig: "once render reports the seek in effect, the headroom is kept again" (`0 !== 38`) |
| `consumeSharedSeek` keeps the old `seek.epoch` (the epoch is never applied) | Red, existing #1387 Worker row: `SEEKS_APPLIED` is 2, not 1 (the same seek is applied again on the next pass) |
| `applySharedSeek` does not clear `anchoredSeek` on consumption | Green: an equivalent mutant. Base also kept the generation and frame words. Seek generations are reserved once each, so the same tuple is never published twice. |

**Render path.** No render code changed. The control Worker runs on the control thread, and the PCM bytes it submits and the order it submits them in are unchanged: every PCM case in the shared table and in the Worker rows passes unchanged. No Rust changed, so the realtime policy tool and the `cargo` gates do not apply. No new cross-thread protocol was added: the seqlock reads are the same `Atomics` loads in the same order, so no loom model applies.

**Test value.**
- Shared case: red when a consumer counts the engine running dry during a seek wait as the ring's underruns once the wait ends. No existing case reads `UNDERRUNS` across a wait. It holds both copies to one rule, so it is also the drift guard for this rule.
- Worker row: red when a wait also drops the quanta a refused report read carried into it. Only the Worker carries quanta, and no existing row refuses a report read before a wait. The row's old assertion ("the quanta of the wait pass uncounted") is deleted: the shared case now holds it for both copies.

**Gates run (all green):**
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- `node --test test/browser-pcm-evals.mjs` in `sdk/`: the prelude's 16 shared cases, the new one included;
- `bash scripts/build-web-audioworklet.sh` on the pinned `nightly-2026-08-20`, then `bash scripts/check-sdk-headless.sh` on that build: 711 tests, 711 passed. The build carries this branch's control Worker, so the real-Wasm worker-mode evals run the new model;
- `bash scripts/check-workspace-policy.sh`, which reported `workspace policy: ok`.

**Not run here:** the Playwright two-browser legs.

**Follow-ups (not in this issue).**
- One generated source for both MSB1 consumers. This is a packaging change across the engine artifact and the SDK asset; the header of `sdk/test/msb1-consumer-cases.mjs` already names it.
- If the source is ever shared, the prelude's `seenEpoch` and `anchored*` fields could take the same shape, using preallocated records so `process()` never allocates.

**Verdict on attempt 1 (Sol): FAIL, on one MAJOR.** The reviewer agreed with the rest:
- the wedge is a documented contract. The reviewer asked that `pcm-ring.ts:47-49` be cited, and it now is;
- behaviour is preserved;
- nothing allocates on the render path.

The findings:
1. **MAJOR.** The torn-publication arm of the wait had no test. Splitting the guard back to base (`if (!this.readSharedSeek(ring)) { ring.pendingElapsed = 0; return; }`) left the suite green. So did skipping `waitBlocks` on the torn wait.
2. **MINOR.** Carried quanta keep their drain blocks but lose their underruns. This was not documented.
3. **MINOR.** Several guards inherited into rewritten lines had no test:
   - `!ring.finished`;
   - the anchored tuple's `sample` and `generation` compares;
   - the `anchoredSeek = null` clear;
   - the `prepareSharedSeeks` `matches` conjuncts and its `publication.anchor`;
   - the epoch rechecks in `readSharedSeek`.

### Attempt 2 (Terra, 2026-11-07)
Branch merged with `origin/main` at `1ed042efc` by a merge commit. None of this issue's files changed upstream.

**MAJOR.** There is a new shared case next to the #2184 one: "a ring whose seek publication is in flight counts no drain block and no underrun, though the engine runs dry meanwhile". It makes the steps the reviewer gave:
- attach, write g1, one pass: `[UNDERRUNS, DRAIN_BLOCKS]` is `[0, 1]`;
- hand-publish an odd epoch, three passes: `[0, 1]`;
- make the epoch even, write g2, one pass: `[0, 2]`.

It is green on the prelude. Against the base Worker (`git show 1aee7e20d:…control-worker.js`, through `MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`, with the Worker row's carry assertion disabled so that the run reaches the case table) it is red:
```
AssertionError [ERR_ASSERTION]: nor once it ends: the blocks the engine ran dry in were the wait's
+   3,
-   0,
    at run (sdk/test/msb1-consumer-cases.mjs:192:14)
```
The optional refused-read-before-a-torn-wait row was not added. Both arms of the wait are one guard and one `waitBlocks` call, and the carry is already tested through the anchored arm. The table below shows that every way of splitting the arms back apart goes red.

**MINOR 2.** This is documented, both in `waitBlocks` and in the Decision above. The suggested close-out on the wait's first pass was not taken. The engine's cumulative count cannot be split between the carried interval and the first waiting pass's interval. Closing the baseline there would count the wait's own interval as underruns, which is the defect this issue removes.

**MINOR 3.** Each inherited guard now has a test, except the two seqlock rechecks. Those need an interleaving hook inside `readSharedSeek` and are a follow-up. The tests added:
- the Worker #1387 row: a finished region's missing frames are no underrun;
- the shared "a seek clears the finished mark" case: the region underruns again after its seek (6 blocks), in both copies;
- the anchored case: a mismatched generation and a mismatched anchor sample leave the ring waiting, beside the existing mismatched frame. Also, an OK is spent on the seek it applied, so the same tuple published again waits;
- the prepare-seek case:
  - an epoch-0 request is superseded;
  - an old epoch whose tuple was republished is superseded;
  - the right epoch with another generation, or with another frame, is superseded;
  - an anchored publication without its OK is refused with 6 and prepares nothing.

  Its "no seek" assertion now counts `source_prepare_seek` calls, not every seek. The Worker drains after each message, and so applies the republished plain seek.

**Guard table.** These are the guards of the touched functions (`drainSharedRing`'s wait and headroom, `accountBlocks`, `waitBlocks`, `readSharedSeek`, `applySharedSeek`, `consumeSharedSeek` and `prepareSharedSeeks`' `matches`). Each mutant replaced the control Worker for one full run of `node scripts/test-web-audioworklet.mjs`, through `MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`. The shared cases run there against the Worker.

| Guard (mutation) | Red test, or why equivalent |
|---|---|
| The wait's torn arm split back to base (`pendingElapsed = 0; return`) | Shared torn case: `[3, 2]` |
| The torn arm skips `waitBlocks` | Shared torn case |
| The apply arm skips `waitBlocks` | Shared anchored-wait case: `[3, 2]` |
| `waitBlocks` keeps the baseline | Shared torn case (the first to run) |
| `waitBlocks` zeroes `pendingElapsed` | Worker #1387 row: "the two carried quanta count, the wait's three do not" |
| `admit` never honoured (`false ?`) | Headroom rig: "the plain seek's new generation fills the headroom (its runway)…" |
| `admit` always honoured (`true ?`) | Headroom rig: "the drain keeps H = 38 of the 80 free" |
| A refused read carries no quanta | Worker #1387 row (the carry) |
| A landing settles on any report (generation compare dropped) | Headroom rig: "the plain seek's new generation fills the headroom…" |
| A landing never settles | Headroom rig: "once render reports the seek in effect, the headroom is kept again" |
| `reportUnderrun !== null` dropped | Worker #1387 row: "the tick submits every published chunk". The first accounting throws on the null baseline (a bigint minus null), and the pass submits nothing |
| `!ring.finished` dropped | Worker #1387 row: "a finished region's missing frames are no underrun" |
| `WRITER_STATE === 1` dropped | Shared "a ring whose writer is not live counts no underrun" |
| The odd-epoch refusal dropped | Shared #2184 case: "a publication in flight asks no seek and reads no slot" |
| The cached-epoch recheck dropped | Shared "a seek published between the epoch load and its recheck (cached path)…": "the g3 chunk is not submitted before its seek" (attempt 2 fold) |
| The tuple recheck dropped | Shared "…(full path) is applied once": "the newer seek, applied once" (attempt 2 fold) |
| `epoch === seek.epoch` dropped | Worker #1387 row (every pass applies the seek again) |
| `recorded === null` dropped (`recorded?.sample`) | **Green: equivalent.** `undefined !== anchor` holds for every anchor (an `i64` bigint), so the null case still waits. |
| `recorded.sample` compare dropped | Shared anchored case: "a mismatched tuple is not this ring's seek: anchorSample" |
| `recorded.generation` compare dropped | Shared anchored case: "…: generation" |
| `recorded.frame` compare dropped | Shared anchored case: "…: sourceFrame" |
| `anchoredSeek = null` dropped | Shared anchored case: "a spent OK applies nothing" |
| An anchored seek lands with `admit` | Headroom rig: "pass 0: the headroom stays free through the hold" |
| A plain seek lands without `admit` | Headroom rig: "the plain seek's new generation fills the headroom…" |
| `consumeSharedSeek` does not clear `ring.finished` | Shared finished-mark case: "the region runs dry again after its one chunk" |
| `matches`: `seek.epoch !== 0` dropped | Shared prepare case: the epoch-0 request |
| `matches`: the epoch compare dropped | Shared prepare case: the republished tuple at its old epoch |
| `matches`: the generation compare dropped | Shared prepare case: the right epoch, another generation |
| `matches`: the frame compare dropped | Shared prepare case: the right epoch, another frame |
| `prepareSharedSeeks` passes `NO_ANCHOR` for `publication.anchor` | Shared prepare case: the anchored publication is refused with 6 and prepares nothing |
| `drainSharedRing` reads the write index live, not `publication.write` | Shared "a ring's first seek published after a pass's snapshot keeps its first chunk" (attempt 2 fold) |
| The attach initialiser `seek.epoch = SEEK_EPOCH` replaced by 0 | **Green: equivalent for every SDK ring.** The SDK attaches only rings it has just created (`createMsb1Ring`, `pcm-feed.ts:467`), at epoch 0, and publishes a ring's first seek only after `ready()` saw it attached. Only a raw ring that was sought before its attach could tell the two apart. Raw rings are attached only through the SDK feed's port, so that is outside the contract. |
| `sharedSlotAge`'s high-half compare dropped | **Green: equivalent below 2^32 generations.** The SDK reserves one generation per seek from 1, so the high half is 0 on every slot and in every publication. |
| `accountBlocks` settles on `>=`, mutated to `===` | The Worker headroom-rig row "#2282: a landing settles when render reports a later generation". A `miso.seek.v1` at g3 moves the source past a plain ring seek landing at g2. The row asserts "the landing settled", and "admit is cleared: the headroom is kept again" (`16 !== 38` under the mutant, with the state assertion removed). |

**Gates run (all green):**
- `bash scripts/test-web-audioworklet.sh`;
- `node --test test/browser-pcm-evals.mjs` in `sdk/`: 52/52, with `MISO_ENGINE_SDK_ARTIFACTS_HEX` set to the attempt-1 build. The prelude runs all 17 shared cases. Attempt 2 changes only a comment in the Worker, so the build was not redone;
- `bash scripts/check-workspace-policy.sh`.

**Test value (attempt 2).**
- Shared torn case: red when a consumer counts the engine running dry while a publication is in flight. The anchored-wait case does not reach the torn arm.
- The added guard assertions: each is red only when its guard is deleted (the table), and no earlier test caught that deletion.

**Follow-up (withdrawn in the attempt 2 fold).** No hook is needed: both harnesses share the test realm's `Atomics`.

**Verdict on attempt 2 (Sol): PASS, with three MINORs.** They are folded in the next commit:
1. **The seqlock rechecks.** The follow-up rested on a false premise. Both harnesses share the test realm's `Atomics`, so a case can wrap `Atomics.load` and publish right after the consumer's first `SEEK_EPOCH` load. There are now two shared cases, one for the cached path and one for the full path, with a helper `publishAfterEpochLoad` that matches the ring by its buffer. Dropping either recheck turns its case red: in the Worker (`MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`) and in the prelude (each recheck dropped in turn, then restored). The follow-up is withdrawn.
2. **`const write = ring.publication.write;` had no red test.** There is a new shared case, "a ring's first seek published after a pass's snapshot keeps its first chunk". It is red in the Worker when the write index is read live.
3. **Three guards were missing from the table.** It now has rows for:
   - the attach initialiser: equivalent for every SDK ring;
   - `sharedSlotAge`'s high half: equivalent below 2^32 generations;
   - the settle `>=`: its reachability is given.

The carried-quanta wording now says that when only the fill read was refused, the report was available and the intervals could have been split. That is pre-existing #1387 behaviour, and it is noted rather than changed.

**Gates (fold):**
- `bash scripts/test-web-audioworklet.sh`;
- `sdk` `node --test --test-name-pattern="MSB1 consumer" test/browser-pcm-evals.mjs`: 20/20;
- `bash scripts/check-workspace-policy.sh`.

**Coordinator follow-up (before the PR).** The settle `>=` was reachable and not pinned by a test, so it now has one. A Worker headroom-rig row runs a plain ring seek, landing at g2, admitted into the headroom. A `miso.seek.v1` at g3 then moves the source past it before render reports g2. The row asserts:
- that render reports g3;
- that the ring's seek is `settled`;
- that the next pass keeps the headroom of 38.

With `>=` mutated to `===` it is red: "the landing settled", or "admit is cleared…" (`16 !== 38`) when the state assertion is removed. `bash scripts/test-web-audioworklet.sh` is green.
