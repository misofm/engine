# Web host: the control Worker counts a ring's seek wait as underruns when the wait ends

Child of #2258 (implicit state machines), item WEB-5. Re-verified at base `1aee7e20d`.

## Mission
In `worker` mode the control Worker drains the SDK's MSB1 rings (`hosts/host-web/web/miso-engine-v1-control-worker.js`). In `single` mode the worklet prelude drains them (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`). Both must count a ring that waits for its seek the same way: as nothing. The Worker's seek state is also a loose bag of fields. It becomes one explicit model.

## What the audit claimed, and what holds
**The claimed wedge does not hold.** The claim was that an anchored seek whose `seek-at` is refused or never sent gets BACKPRESSURE on every tick. That wait is the designed contract, and it is the same in both copies:
- The prelude documents it at `miso-engine-v1-pcm-feed-worklet.js:630-637`: "a refused `seek-at` leaves it unseen until a newer ring seek supersedes it".
- The SDK's public contract documents it at `sdk/src/browser/pcm-feed.ts:133-135`: "Until it did, the drain reads none of that ring's chunks".
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
