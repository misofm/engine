# Web host: a control Worker dispose that finds the session failed rejects sticky and never frees the host

Child of #2258 (implicit state machines), item WEB-7. Re-verified at base `d441cd408`.

## Mission
In `worker` mode the control Worker (`hosts/host-web/web/miso-engine-v1-control-worker.js`) owns the host's control half and is the only place that frees it. Its lifecycle is spread over loose fields: `booted`, `ready`, `disposing`, `disposed`, `stickyResult`, `disposeOutcome`, `disposeReason`, `disposeRequestIds` and `releaseDeadline`. Each site that must know the phase reads its own subset of them, and one stage forgot `disposing`. The lifecycle becomes one `phase` tagged union, and every site asks it the same question.

## What the audit claimed, and what holds
Line numbers are at `d441cd408`.

**1. "The first message is consumed as the boot."** This is the designed protocol, not a defect. The page posts the boot in the same task that constructs the Worker (`miso-engine-v1-audio-worklet-host.js:1554`), before the host object exists, so nothing can be posted ahead of it. `boot()` checks the exact boot fields (`control-worker.js:254-266`), so a first message that is not a boot is refused as a boot (`miso.error.v1`, request ID 0, result 1), which the page reads as a refused boot (`audio-worklet-host.js:1543-1546`).

What does hold nearby: `onPageMessage` sets `booted` before it calls `boot()` (lines 212-216), and treats any `RESULT_OK` as success. A boot that returns `RESULT_OK` without reaching ready posts nothing at all, and the Worker is left booted, not ready and not disposed: the page's boot waits with no reply. `boot()` returns `miso_engine_web_v1_boot_result()` when `miso_engine_web_v1_boot` gives handle 0 (line 296) or `document_ptr` gives 0 (line 285). Rust's `boot_staged` leaves the staging's result at the `RESULT_OK` that `document_ptr` stored when its `try_borrow_mut` fails (`hosts/host-web/src/ffi.rs:712-715`, `773-776`). That is unreachable today: nothing re-enters the staging. So this is hardening: the phase model must say what a boot that ends without a host is, and the answer is a refused boot (255).

**2. "`disposing` and `disposed` can both be true."** True: `finishDispose` sets `disposed` (line 1473) and never clears `disposing` (set at line 1446). Every reader checks `disposed` first or both, so on its own this changes no behaviour. **But the split hides a real defect:** the sites disagree on which flags mean "control work may run".
- `drainFeeds` (line 1101), `prepareSharedSeeks` (line 1373), `postMeterView` (line 1424) and the service tick (line 511) stop once a dispose began.
- `serviceControl` (line 838), `pushWatermark` (line 844) and `drainControlEvents` (line 875) check only `disposed`, `ready` and `stickyResult`. `ready` stays true through a dispose.
- `onPageMessage` runs `afterControl` after every page message (line 232), including the dispose message itself and every request refused during the dispose.

So the service step, the watermark read and the event drain run in the dispose window. When the session has failed (the render half stored `STATE_FAILED`; `service` and `read_watermark` then return `WRONG_STATE`, and `control_event` returns `RESULT_REFUSED_LIFECYCLE`: `hosts/host-web/src/lib.rs:1166-1169`, `2010-2013`, `2036-2039`, `1919-1921`), the refusal goes to `fatal` (lines 840, 849, 881). That:
- posts a sticky `miso.error.v1` carrying the dispose's own request ID, or the ID of the request refused during it. `refuseDisposing` documents the rule this breaks (line 666): "No refusal during a dispose is sticky: a sticky error would reject the dispose itself";
- makes the page's `#fail` reject every pending request, the dispose included (`audio-worklet-host.js:959-975`). `#disposeWorkerMode` then tears down unclean and terminates the Worker (`audio-worklet-host.js:1409-1417`, `1484-1488`) before the worklet's `miso.released.v1` reaches it. The Worker never adopts the half back and never calls `miso_engine_web_v1_dispose`, so the host's memory is never freed. The shared memory stays referenced by the worklet;
- stops the service tick, which holds the release deadline (lines 577, 1450), so a worklet that never answers the release would leave the dispose unanswered by the Worker.

This is reachable: a render failure between the Worker's last tick and the page's dispose (one tick is about 3 ms) is enough, and so is a render failure during the dispose window followed by any page request. A dispose of a session that the Worker already knew had failed is not affected: `stickyResult` stops all three stages, and the dispose frees the host. The defect is that the outcome depends on which side of a tick the failure fell.

**3. "A late `render-status` falls through to `fatal`."** True and reachable on the release-timeout path. A status read in flight across a dispose is answered from the Worker's own status in `finishDispose` (line 1467). If the release timed out, the worklet's `miso.render-status.v1` reply can still arrive. `statusReads` is empty, so the reply matches nothing and `onWorkletMessage` calls `fatal` (line 1525). The Worker then posts a `miso.error.v1` after it answered the dispose, and overwrites `stickyResult` in a Worker whose host is gone. The page tears down on the teardown reply, so today the stray post finds no listener. It is still a message the protocol does not allow: once the host is gone, the only worklet message with a meaning is a late `miso.released.v1`, whose orphan half is reclaimed (line 1510).

## Decision
- **`this.phase`** is one tagged union, the Worker's lifecycle:
  - `{ kind: "unbooted" }`: before the first page message;
  - `{ kind: "refused" }`: the boot did not reach `live`. It was refused, or it threw; on a throw the page gets the Worker's `error` event and terminates it. Nothing is held;
  - `{ kind: "live" }`: booted, with the render half released to the worklet;
  - `{ kind: "disposing", requestIds, deadline }`: the release was asked for. `requestIds` are the page's dispose requests, answered together. `deadline` is the release deadline;
  - `{ kind: "disposed", outcome, reason }`: the host is freed. `outcome` is the dispose's result and `reason` the typed teardown reason (`release-timeout`), if any.

  It replaces `booted`, `ready`, `disposing`, `disposed`, `disposeOutcome`, `disposeReason`, `disposeRequestIds` and `releaseDeadline`.
- **`stickyResult` stays a separate field.** A sticky error is the session's failure, not a lifecycle phase. A failed session is still `live`: it answers status reads and the session map, and it must still be disposed. Folding the failure into the union would repeat it in `live` and `disposing`. `ready` was always `live && stickyResult === RESULT_OK`, and is now derived as `controlLive()`.
- **`controlLive()`** is the one guard of control work: the feed drain, the service step, the watermark, the event drain, the seek preparation and the tick's work. It is false in every phase but `live`, and after a sticky error. This fixes claim 2: no control work runs once a dispose began. The tick already behaved this way.
- **A boot ends `live` or `refused`.** `onPageMessage` enters `refused` before it calls `boot()`, because a boot holds nothing until it reaches `live`; only `boot()`'s success enters `live`. If the phase is not `live` afterwards, the page gets the boot's result, or 255 when `boot()` returned `RESULT_OK` without a host. A separate `booting` phase was tried and dropped: it behaved exactly as `refused` in every handler (its write was an equivalent mutant).
- **`fatal` is folded into `sticky`.** `fatal` was `sticky` plus an immediate stop of the service tick. In `live` the stop was redundant: the tick stops itself at its next run, because `controlLive()` is false after a sticky error. In `disposing` it was harmful: it stopped the tick that holds the release deadline. One function now posts every sticky error, and the tick alone decides when it stops.
- **Only `live` and `disposing` hold a host.** In every other phase a page message is dropped, except a dispose, which is answered at once: with the dispose's outcome once `disposed`, and 255 otherwise (as before). A worklet message is dropped too, except a late `miso.released.v1` once `disposed`, whose orphan is reclaimed. This fixes claim 3.
- **Unchanged:** the request-ID check, the order of the dispose and sticky checks in `receive`, `refuseDisposing`, and the posts of every sticky error in `live` and `disposing`.

## Non-goals
- Folding `stickyResult` into the union (see the decision).
- WEB-2 (routing every sticky error through one `fail()` across the page, the Worker, the worklet and Rust), WEB-6 (the page's lifecycle, `miso-engine-v1-audio-worklet-host.js`, #2279) and the worklet (`miso-engine-v1-audio-worklet.js`, #2291).
- The ring seek model (#2282).
- Changing what a sticky error during a dispose posts when the page itself breaks the protocol (an out-of-order request ID). It is the page's bug, and it stays sticky.

## Gates
- Regression rows in their own block of `scripts/test-web-audioworklet.mjs` (`testControlWorker`), each red on base:
  - claim 2, one row per stage: with the service step, the watermark read or the event drain refusing `WRONG_STATE`, a dispose posts no sticky error, keeps its tick, and frees the host when the half comes back;
  - claim 3: after a release timeout, a late `miso.render-status.v1` posts nothing;
  - claim 1's hardening: a boot that returns `RESULT_OK` without a host is refused 255, and a later dispose is answered 255.
- A guard table: every guard and every phase write in the Worker's lifecycle code is mutated, and each mutant is red on a named test or has a stated equivalence. Guard rows for the lifecycle paths no existing row reached go in the same block.
- `bash scripts/test-web-audioworklet.sh`.
- `bash scripts/check-workspace-policy.sh`.
- `bash scripts/build-web-audioworklet.sh`, then `bash scripts/check-sdk-headless.sh` on that build, so the real-Wasm `worker`-mode evals run the new Worker.
- No Rust changes. The realtime policy tool, the `cargo` gates and the no-bit-moved comparison do not apply: the Worker is the control thread, and no render code or PCM path changes. No cross-thread state is added, so no loom model applies.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `d441cd408`.

**Red on base.** The three regression rows (N1 to N3 below) were written first and run against the base Worker. A copy of the runner reported each row's first failure and went on (`node`, one run):
```
RED-ROW the service step: a session failed in the dispose window posts no sticky error | [{"tag":"miso.error.v1","requestId":1,"result":3},{"tag":"miso.error.v1","requestId":2,"result":3}]
RED-ROW the watermark read: a session failed in the dispose window posts no sticky error | [{"tag":"miso.error.v1","requestId":1,"result":3},{"tag":"miso.error.v1","requestId":2,"result":3}]
RED-ROW the event drain: a session failed in the dispose window posts no sticky error | [{"tag":"miso.error.v1","requestId":1,"result":3},{"tag":"miso.error.v1","requestId":2,"result":3}]
RED-ROW a late status reply posts nothing once the host is freed | 4
RED-ROW Expected values to be strictly deep-equal: | []
```
- In N1, the dispose message itself and the status read refused during it each post a sticky `WRONG_STATE`. The first one carries the dispose's own request ID, so the page would reject its dispose with it.
- In N2, base posts a fourth message, `miso.error.v1` 255, after the `release-timeout` teardown.
- In N3, base posts nothing at all: the page's boot would wait with no reply.

The guard rows N4 to N9 are green on base. They hold rules base already kept, and each one exists because a mutant of the new model survived without it (the table below).

**The rows** (`scripts/test-web-audioworklet.mjs`, `testControlWorker`, the `Issue #2306` block). Each names the defect that turns it red and that no other row catches:
- **N1, the dispose window** (one row per stage): red if the service step, the watermark read or the event drain runs after a dispose began. Then a session that failed in the window posts a sticky error that rejects the dispose, and the host is never freed. No row failed a stage after a dispose began.
- **N2, a late worklet message**: red if a status reply that outlived a release-timeout dispose posts a sticky error, or if a late release with no token adopts anything. No row sent a worklet message after a timed-out dispose.
- **N3, a boot without a host**: red if a boot that reports `RESULT_OK` with handle 0 is taken for a success, which leaves the page's boot unanswered. D6 covers only a boot refused with its own code.
- **N4, the stages in a live session**: red if a stage's guard or call stops it running in `live`, if a live refusal is not sticky (or is posted twice), if the tick does not stop after it, or if a dispose after that stop starts no tick for its deadline. No row read the Worker's watermark or control-event posts, or ran a dispose after a stopped tick.
- **N5, the sticky latch**: red if an out-of-order request runs on after its sticky error, if a later request runs (the latch dropped), if a refused chunk loses its planes, or if the tick keeps asking the worklet after a sticky error. The #2267 rows check only the first sticky reply.
- **N6, the worklet protocol in `live`**: red if a release nobody asked for runs a dispose or falls through, or if a worklet message outside the protocol posts no sticky error. No row sent either.
- **N7, refused boots**: red if any of the four refusal paths after the boot export falls through to the release and the ready reply, or loses its code. There are six rows: an unready status, no TLS size, no reservation, a reservation past the memory, no token, and another handle's token. D6 reaches only the boot export's own refusal.
- **N8, the replies in the dispose window**: red if a branch of `refuseDisposing` answers another request's shape, or none. The #1332 row covers only the control call and the session map.
- **N9, feeds across a dispose**: red if the feed drain or the seek preparation runs in the dispose window, if an attach binds a ring then, or if the finished dispose leaves a ring attached or its port open. No row attached a ring across a dispose.

No existing row is made useless: the #1332, #2267 and D6 rows hold claims these rows do not repeat.

**Guard table.** `gen.py` (adapted from the #2279 review's generator) mutated every line of the lifecycle code, at commit `4c88d0b78`:
- `onPageMessage`, `holdsHost`, `controlLive`, `afterControl` and the constructor;
- the end of `boot` (from `readReadyFields`), `disposeAfterFailedBoot`, `failBoot`, the tick and its timer, and `sticky`;
- `receive` and `refuseDisposing`;
- the three stages, the attach in `receiveFeed`, `drainFeeds`' guard, `prepareSharedSeeks`' guard and `postMeterView`;
- `beginDispose`, `finishDispose`, `answerDispose`, `reclaimOrphan` and `onWorkletMessage`.

The mutations were these:
- `===`/`!==`, `&&`/`||`, `??`, the comparisons and `!this.`;
- the phase strings swapped (`"live"`/`"disposing"`, `"disposed"`/`"disposing"`, `"unbooted"`/`"booting"`, `"refused"`/`"disposed"`);
- the boot's `RESULT_OK ? RESULT_INTERNAL` and the dispose answer's `RESULT_INTERNAL` fallback;
- the `released` branch's reclaim and finish swapped;
- every `if` forced true and false, every `return` deleted, and every one-line statement deleted.

Each mutant replaced the Worker for one full run of `node scripts/test-web-audioworklet.mjs`, through `MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`. Of the 508 mutants, 425 were red. Ten of those did not parse (a `return` deleted inside a one-line `if`), and they are left out of the table. One more never finished: `statusReads.length >= 0` loops for good, so the run timed out. The other 82 were green, and each is listed under the table.

The table has one row per lifecycle line: the line, the mutations made there, and the rows that went red.

| Line | Guard or write | Mutations, all red | Red in |
|---|---|---|---|
| 180 | `this.phase = { kind: "unbooted" };` | delete, "unbooted"->"booting" | ?; #1332 S7 dispose row |
| 181 | `this.stickyResult = RESULT_OK;` | delete | #1332 S7 dispose row |
| 211 | `if (this.phase.kind === "unbooted") {` | ===->!==, if→false, if→true, "unbooted"->"booting" | #1332 S7 dispose row |
| 214 | `this.phase = { kind: "refused" };` | delete, "refused"->"disposed" | #2306 N3 boot without host |
| 216 | `if (this.phase.kind !== "live") this.failBoot(result === RESULT_OK ? R` | ===->!==, if→false, if→true, "live"->"disposing", !==->===, RESULT_OK ? RESULT_INTERNAL->RESULT_OK ? RESULT_OK | D6 refused boot; #1332 S7 dispose row; #2306 N3 boot without host |
| 224 | `if (!this.holdsHost()) {` | if→false, if→true, !this.->this. | #1332 S7 dispose row |
| 227 | `this.answerDispose(message.requestId);` | delete | #1332 S7 dispose row |
| 233 | `this.afterControl(requestId);` | delete | #1332 S7 dispose row |
| 238 | `return this.phase.kind === "live" \|\| this.phase.kind === "disposing"` | "disposing"->"live", ===->!==, "live"->"disposing", delete, \|\|->&& | #1332 S7 dispose row |
| 246 | `return this.phase.kind === "live" && this.stickyResult === RESULT_OK;` | &&->\|\|, ===->!==, "live"->"disposing", delete | #2306 N1 dispose window; #1332 S7 dispose row |
| 250 | `this.drainFeeds();` | delete | #1387 feed drain |
| 251 | `this.serviceControl(requestId);` | delete | #1332 S7 dispose row |
| 252 | `this.pushWatermark(requestId);` | delete | #2306 N4 live stages |
| 253 | `this.drainControlEvents(requestId);` | delete | #2306 N4 live stages |
| 254 | `this.postMeterView();` | delete | #1332 S7 dispose row |
| 312 | `if (typeof ready === "number") return this.disposeAfterFailedBoot(read` | ===->!==, if→false, if→true | #1332 S7 dispose row; #2306 N7 refused boots |
| 316 | `if (!u32(tlsSize) \|\| !u32(tlsAlign)) return this.disposeAfterFailedB` | if→false, if→true, \|\|->&& | #2306 N7 refused boots; #1332 S7 dispose row |
| 321 | `return this.disposeAfterFailedBoot(RESULT_INTERNAL);` | delete | #2306 N7 refused boots |
| 327 | `return this.disposeAfterFailedBoot(RESULT_INTERNAL);` | delete | #2306 N7 refused boots |
| 329 | `this.phase = { kind: "live" };` | delete, "live"->"disposing" | #1332 S7 dispose row |
| 347 | `this.startServiceTick();` | delete | #1332 S7 dispose row |
| 520 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | #1332 S8 release timeout; #1332 S8 render check |
| 521 | `if (clock() >= this.phase.deadline) this.finishDispose(0n, "release-ti` | >=->>, if→false, if→true | #1332 S8 release timeout |
| 524 | `if (!this.controlLive()) {` | if→false, if→true, !this.->this. | #2306 N4 live stages; #1332 S8 render check |
| 525 | `this.stopServiceTick();` | delete | #2306 N4 live stages |
| 528 | `this.afterControl(0);` | delete | #1387 feed drain |
| 529 | `this.checkRender();` | delete | #1332 S8 render check |
| 542 | `if (this.serviceTick === null) return;` | ===->!==, if→true, delete | #1332 S7 dispose row |
| 543 | `clearInterval(this.serviceTick);` | delete | #1332 S7 dispose row |
| 544 | `this.serviceTick = null;` | delete | #2306 N4 live stages |
| 572 | `if (this.stickyResult === RESULT_OK && this.handle !== 0) {` | ===->!==, if→false, !==->=== | #2267 sticky table |
| 579 | `this.stickyResult = result;` | delete | #2306 N4 live stages |
| 589 | `this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returned` | delete | #2267 sticky table |
| 594 | `this.beginDispose(message.requestId);` | delete | #1332 S7 dispose row |
| 597 | `if (this.stickyResult !== RESULT_OK) {` | if→false, if→true, !==->=== | #2306 N5 sticky latch; #1332 S7 dispose row |
| 598 | `this.sticky(this.stickyResult, message.requestId, returnedPlanes);` | delete | #2306 N5 sticky latch |
| 601 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | #1332 S7 dispose row |
| 602 | `this.refuseDisposing(message);` | delete | #1332 S7 dispose row |
| 623 | `this.sticky(RESULT_INTERNAL, message.requestId);` | delete | #2267 sticky table |
| 638 | `this.sticky(RESULT_INVALID_ARGUMENT, message.requestId, returnedPlanes` | delete | #2267 sticky table |
| 842 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | #2306 N1 dispose window; #1332 S7 dispose row |
| 844 | `if (result !== RESULT_OK) this.sticky(result, requestId);` | if→false, if→true, !==->=== | #2306 N4 live stages; #1332 S7 dispose row |
| 848 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | #2306 N1 dispose window; #2306 N4 live stages |
| 853 | `this.sticky(result, requestId);` | delete | #2306 N4 live stages |
| 879 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | #2306 N1 dispose window; #2306 N4 live stages |
| 885 | `this.sticky(result, requestId);` | delete | #2306 N4 live stages |
| 930 | `if (this.phase.kind !== "live") return;` | if→false, if→true, "live"->"disposing", !==->===, delete | #2306 N9 feeds across a dispose; #1387 feed drain |
| 1105 | `if (this.rings.length === 0 \|\| !this.controlLive()) return;` | ===->!==, if→false, if→true, !this.->this., \|\|->&& | #1387 feed drain; #2306 N9 feeds across a dispose |
| 1374 | `if (!this.controlLive() \|\| !Array.isArray(seeks) \|\| seeks.length !` | if→false, if→true, !==->===, !this.->this., \|\|->&& | #2306 N9 feeds across a dispose; MSB1 shared cases |
| 1424 | `if (this.phase.kind !== "live") return;` | if→false, if→true, "live"->"disposing", !==->=== | #1332 S8 release timeout; #1332 S7 dispose row |
| 1445 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | ?; #1332 S7 dispose row |
| 1446 | `this.phase.requestIds.push(requestId);` | delete | #1332 S7 dispose row |
| 1449 | `this.phase = { kind: "disposing", requestIds: [requestId], deadline: c` | "disposing"->"live", delete | #1332 S7 dispose row |
| 1452 | `if (this.serviceTick === null) this.startServiceTick();` | ===->!==, if→false, if→true | #1332 S7 dispose row; #2306 N4 live stages |
| 1457 | `if (phase.kind !== "disposing") return;` | "disposing"->"live", if→false, if→true, !==->===, delete | #1332 S7 dispose row; ? |
| 1458 | `this.stopServiceTick();` | delete | #1332 S7 dispose row |
| 1476 | `this.phase = { kind: "disposed", outcome: result, reason };` | "disposed"->"disposing", delete | #1332 S7 dispose row |
| 1478 | `this.closeFeeds();` | delete | #2306 N9 feeds across a dispose |
| 1479 | `for (const requestId of phase.requestIds) this.answerDispose(requestId` | delete | #1332 S7 dispose row |
| 1486 | `if (phase.kind === "disposed" && phase.reason !== undefined) {` | &&->\|\|, "disposed"->"disposing", ===->!==, if→false, if→true, !==->=== | #1332 S7 dispose row; #1332 S8 release timeout |
| 1487 | `this.post({ tag: "miso.teardown.v1", requestId, result: RESULT_INTERNA` | delete | #1332 S8 release timeout |
| 1489 | `this.acknowledge(requestId, phase.kind === "disposed" ? phase.outcome ` | "disposed"->"disposing", delete, ===->!==, phase.outcome : RESULT_INTERNAL->phase.outcome : RESULT_OK | #1332 S7 dispose row; #2306 N3 boot without host |
| 1511 | `if (this.phase.kind === "disposed") this.reclaimOrphan(token);` | "disposed"->"disposing", ===->!==, if→false, if→true, reclaimOrphan(token);\n      else this.finishDispose(token);->finishDispose(token);\n      else this.reclaimOrphan(token); | #1332 S7 dispose row; #1332 S8 release timeout |
| 1512 | `else this.finishDispose(token);` | delete | #1332 S7 dispose row |
| 1517 | `if (!this.holdsHost()) return;` | if→false, if→true, delete, !this.->this. | #2306 N2 late worklet message; #1332 S7 dispose row; #1387 feed drain |
| 1529 | `this.sticky(RESULT_INTERNAL, this.lastRequestId);` | delete | #2306 N6 worklet protocol in live |

**Green, equivalent (17).** These lines are lifecycle code:
- **Constructor initialisers of `handle`, `serviceTick`, `bootToken` and `lastRenderCheck` (183, 184, 195, 196).** Each is read only after `boot()` assigns it: `bootToken` only in `disposing`, `lastRenderCheck` only on a live tick, and `serviceTick` only after the boot started the tick. `handle` is also read by a `sticky` before the boot (a `messageerror`). That `session_fail` call runs on exports that do not exist yet, and it throws inside its `try`.
- **`stopServiceTick`'s null return forced false (542).** The mutant calls `clearInterval(null)`, which is a no-op in HTML.
- **`sticky` calls `session_fail` every time (572, two mutants).** The engine refuses a second fail and a freed handle, and changes nothing (`ffi.rs:1241-1243`).
- **`boot`'s `return RESULT_OK` deleted (348).** On a success, `onPageMessage` reads the phase, not the result.
- **`handle = 0` deleted in `disposeAfterFailedBoot` and `finishDispose` (498, 1477).** With no host, nothing reads `handle` except `sticky`'s `session_fail`, which the engine refuses for a freed handle.
- **The boot-token adoption forced true (1468).** `bootToken` is never 0 in `disposing`, because a boot reaches `live` only with a non-zero token (line 326).
- **The `miso.service.v1` handling (232, and 586, three mutants).** The page never sends that message in `worker` mode (`audio-worklet-host.js:556`).
- **`?? 0` as `|| 0` for an out-of-order request's ID (589).** The two differ only for a falsy request ID that is not nullish (NaN, `""`, `false`). Such an ID matches no pending request on the page either way, so the page fails the same.
- **`refuseDisposing`'s first test with `seek || seek-at` as `seek && seek-at` (647).** A seek or seek-at then falls through to the last branch, which posts the identical ack.

**Green, outside the lifecycle code (65).** These are inherited checks, left unchanged by this issue. They survive because no Worker row reaches them:
- the constructor initialisers of the watermark, meter-view and feed state (185-190, 202, 203);
- the `miso.feed-port.v1` shape (219, 220);
- the reservation record's end bound, `>` against `>=` (319);
- `postedMemoryBytes` at the boot (332). This one is equivalent: `postedHeader` starts at -1, so the first view is posted anyway;
- `transferList` (558, 560);
- `receive`'s dispatch of a live request (605-618). No Worker row sends a live source chunk, seek report or plan-latency read, or a request with an extra field;
- the watermark record checks and dedupe (851-871), and the event drain's lane bound and zero-pointer check (882, 891-893);
- `receiveFeed`'s message shape, `detach` and `detach-source` (926-954);
- a ring's error counter (1111);
- the field checks of `released`, `render-clock` and `render-status`, and the status-read queue (1508-1525).

They are listed as a follow-up.

**Render path.** No render code changed. The Worker is the control thread. No Rust changed, so the realtime policy tool, the `cargo` gates and the no-bit-moved comparison do not apply. No cross-thread state was added: the phase is the Worker's own, and nothing outside it reads the phase. So no loom model applies.

**Gates run:**
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- `bash scripts/check-workspace-policy.sh`, `workspace policy: ok`;
- `bash scripts/check-sdk-headless.sh` with no argument, exit 0: 715 tests, 715 passed. It builds the artifact itself with `scripts/build-web-audioworklet.sh` from this tree, so its real-Wasm `worker`-mode evals ran the new Worker.

**Not run here:** the Playwright two-browser legs.

**Follow-up (not in this issue).** The 65 inherited checks above have no Worker row: the dispatch of a live request, the watermark record checks, the event drain's bounds, the feed messages' shapes, and the worklet messages' field checks. They are not lifecycle code. A separate issue could give each one a row.
