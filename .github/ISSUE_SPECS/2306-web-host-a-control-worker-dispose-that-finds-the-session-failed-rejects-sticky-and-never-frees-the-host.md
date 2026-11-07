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
- **`stickyResult` stays a separate field.** A sticky error is the session's failure, not a lifecycle phase. A failed session is still `live`: it keeps its host, which only a dispose frees. Every request but a dispose gets the sticky error, a status read and a session map read included (attempt 2: attempt 1 said a failed session answers status reads, which it never did). Folding the failure into the union would repeat it in `live` and `disposing`. `ready` was always `live && stickyResult === RESULT_OK`, and is now derived as `controlLive()`.
- **`controlLive()`** is the one guard of control work: the feed drain, the service step, the watermark, the event drain, the seek preparation and the tick's work. It is false in every phase but `live`, and after a sticky error. This fixes claim 2: no control work runs once a dispose began. The tick already behaved this way.
- **A boot ends `live` or `refused`.** `onPageMessage` enters `refused` before it calls `boot()`, because a boot holds nothing until it reaches `live`; only `boot()`'s success enters `live`. If the phase is not `live` afterwards, the page gets the boot's result, or 255 when `boot()` returned `RESULT_OK` without a host. A separate `booting` phase was tried and dropped: it behaved exactly as `refused` in every handler (its write was an equivalent mutant).
- **`fatal` is folded into `sticky`.** `fatal` was `sticky` plus an immediate stop of the service tick. In `live` the stop was redundant: the tick stops itself at its next run, because `controlLive()` is false after a sticky error. In `disposing` it was harmful: it stopped the tick that holds the release deadline. One function now posts every sticky error, and the tick alone decides when it stops.
- **Only `live` and `disposing` hold a host.** In every other phase a page message is dropped, except a dispose, which is answered at once: with the dispose's outcome once `disposed`, and 255 otherwise (as before). A worklet message is dropped too, except a late `miso.released.v1` once `disposed`, whose orphan is reclaimed. This fixes claim 3. A `miso.feed-port.v1` is not opened either (attempt 2): no ring could be bound, and no dispose would close its port.
- **An unreadable page message (`messageerror`) follows the phase** (attempt 2). Before the boot it is the boot, refused 255, and the Worker is `refused`; with a host it is a sticky error; with none it is dropped. Attempt 1 posted it in every phase, overwriting `stickyResult` after the host was gone.
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

Attempt 1's per-line table and its lists of green mutants are superseded by attempt 2's, below; git history keeps them. Its equivalence for line 232 was wrong (see the verdict).

**Render path.** No render code changed. The Worker is the control thread. No Rust changed, so the realtime policy tool, the `cargo` gates and the no-bit-moved comparison do not apply. No cross-thread state was added: the phase is the Worker's own, and nothing outside it reads the phase. So no loom model applies.

**Gates run:**
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- `bash scripts/check-workspace-policy.sh`, `workspace policy: ok`;
- `bash scripts/check-sdk-headless.sh` with no argument, exit 0: 715 tests, 715 passed. It builds the artifact itself with `scripts/build-web-audioworklet.sh` from this tree, so its real-Wasm `worker`-mode evals ran the new Worker.

**Not run here:** the Playwright two-browser legs.

Attempt 1 listed the 65 inherited checks no Worker row reached as a follow-up. Attempt 2 gives each a row or an equivalence, so that follow-up is withdrawn.

**Verdict on attempt 1 (Sol): FAIL, on two MAJORs and two MINORs.** The reviewer confirmed:
- the Worker fix is correct;
- the merge with `main` is clean;
- `check-sdk-headless` passes 716/716 on the merged tree.

The findings:
1. **MAJOR.** The dispose-window claim of the `fatal`-into-`sticky` fold had no test. Re-adding `this.stopServiceTick()` as `sticky`'s first line kept every row green. So did adding it only to the worklet fall-through, the out-of-order refusal or `onmessageerror`.
2. **MAJOR.** The equivalence claimed for line 232 was false. The mutation `message?.tag === "miso.service.v1" ? 0 : this.lastRequestId` → `!==` (or `requestId = 0`) changes the request ID of every refusal that follows a page request. Probe: with the service step answering 3, a session map read posts `[sessionmap 1 0], [error 1 3]`; the mutant posts `[error 0 3]`.
3. **MINOR.** Deleting `this.statusReads.shift()` was green, and a status read was then answered twice. `answerStatus`'s two sticky errors had no row.
4. **MINOR.** The phase rule was overstated:
   - a failed session does not answer status reads;
   - `onmessageerror` still posted, and overwrote `stickyResult`, after `disposed` or `refused`;
   - a `miso.feed-port.v1` opened a feed after a dispose.

### Attempt 2 (Terra, 2026-11-07)
`origin/main` (`d11fed9b4`) is merged into the branch by a merge commit. None of this issue's lines changed upstream. The test file moved by about 830 lines, so the line numbers below are at `0266249b6`.

**The Worker.**
- `onmessageerror` now runs `onMessageError()`. Before the boot, an unreadable message is the boot: it is refused 255, and the Worker is `refused`, so a later boot message runs nothing. With a host, it is a sticky error. With none, it is dropped.
- The `miso.feed-port.v1` branch now comes after the host check, so a refused or freed Worker opens no feed.
- The phase comment no longer says that a failed session answers status reads.

**The rows added** (the `Issue #2306` block). Each is red on the defect named, and no other row catches it:
- **The dispose window keeps its deadline** (the review's P1 and P1b): red when a sticky error stops the tick. There are three cases: a worklet message outside the protocol, an unreadable page message, and an out-of-order request in the dispose window. In each the deadline still answers `release-timeout`. All four re-added-stop mutants are red here.
- **A refusal carries its request's ID** (P3): red when the step after a page request posts its sticky error as request 0.
- **A status read is answered once**, and the answer takes the render half's state and last result: red when a matched read stays queued (it is then answered again by the dispose), or when the state and last-result merge is wrong.
- **`answerStatus`'s two faults:** a worklet reply that is no record, and a control status that cannot be read. Each is a sticky internal error, never a status.
- **`onmessageerror` in every phase:** an unreadable boot is refused 255 for good; a live Worker posts it; a refused or freed Worker posts nothing.
- **A feed port with no host** is not opened, after a refused boot or a finished dispose.
- **The field checks:** a feed port that cannot post, a feed port with an extra field, a release with an extra field, a release whose token is no bigint, a zero render clock, a second render clock, and a status reply no read waits for. Each is a sticky error.
- **The watermark:** the initial zero counts, the count flag, the dedupe of an unchanged mark, the outcome word, a backpressured read (no error), and a record of the wrong size (sticky).
- **The event drain:** at most 16 frames a lane per pass, and an event with no address is sticky.
- **The boot:** a reservation that ends exactly at the memory's end is accepted, and an unreadable boot answers a later dispose 255.
- **A refused chunk's planes** come back by transfer, and a plane on shared memory is cloned, never put in the transfer list.
- **`receive`'s dispatch:** each live request (source, seek, session seek, anchored seek, seek report, plan latency, control) runs its own export and answers in its own shape. The same tag with an extra field is a sticky error that runs nothing.
- **A request whose ID is NaN** gets its sticky error under that ID, not 0. The page reads 0 as the typed failure of no request.
- **The feed port's messages:**
  - a message that is no object;
  - an attach whose rings are no list;
  - a new attach that replaces the old list;
  - a detach-source with an index that is no integer;
  - an unknown op, which is not answered;
  - a drain that throws, which is counted;
  - a detach that leaves the other feeds to the dispose.

**Guard table (attempt 2).** The generator now covers the whole lifecycle range of the Worker at `4bf0e194b`, lines 167-280, 325-363, 510-688, 855-928, 939-973, 1118-1130, 1387-1390 and 1436-1580. That is:
- the constructor, `onMessageError`, `onPageMessage`, `holdsHost`, `controlLive` and `afterControl`;
- the end of `boot`, `disposeAfterFailedBoot`, `failBoot`, the tick and its timer, `transferList` and `sticky`;
- `receive` and `refuseDisposing`;
- the three stages in full, and `receiveFeed` in full;
- `drainFeeds`, `prepareSharedSeeks`' guard and `postMeterView`;
- the dispose functions, `onWorkletMessage` and `answerStatus`.

The operators are attempt 1's. Seven hand mutants were added:
- `stopServiceTick()` re-added at the start of `sticky`, in the worklet fall-through, in the out-of-order refusal, and in `onMessageError`;
- the base `onmessageerror`;
- `requestId = 0` after a page request;
- the feed-port branch moved back above the host check.

There were 535 mutants. 505 are red, and nine of those did not parse (a `return` deleted inside a one-line `if`), so they are left out of the table. One never finishes: `statusReads.length >= 0` loops for good. The other 29 are green, and each is a named equivalence below. Every hand mutant is red:
- `H_sticky_stop` and `H_worklet_stop` fail the dispose-window row's "a worklet fault in the dispose window keeps the deadline's tick";
- `H_order_stop` fails its "an out-of-order request in the dispose window keeps the deadline's tick";
- `H_msgerr_stop` fails "so does an unreadable page message";
- `H_requestid_zero` fails the request-ID row;
- `H_msgerr_base` fails "and a boot after it runs nothing" (it boots);
- `H_feedport_first` fails the feed-port-with-no-host row.

One row per mutated line follows, with the rows that went red. A row is named by the first words of the comment that opens it, at its line in `scripts/test-web-audioworklet.mjs` at `0266249b6`.

| Line | Code | Red mutations | Red in |
|---|---|---|---|
| 168 | `this.scope = scope;` | delete | the first Worker row (the boot helper throws) |
| 181 | `this.phase = { kind: "unbooted" };` | delete, "unbooted"->"booting" | the first Worker row (the boot helper throws) |
| 182 | `this.stickyResult = RESULT_OK;` | delete | the first Worker row (the boot helper throws) |
| 183 | `this.lastRequestId = 0;` | delete | L6397 «In a live session, a release nobody asked for frees…» |
| 188 | `this.watermarkCounts = [0n, 0n, 0n];` | delete | the first Worker row (the boot helper throws) |
| 193 | `this.statusReads = [];` | delete | the first Worker row (the boot helper throws) |
| 200 | `this.feeds = [];` | delete | the first Worker row (the boot helper throws) |
| 201 | `this.rings = [];` | delete | the first Worker row (the boot helper throws) |
| 202 | `this.renderStatusPointer = 0;` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 206 | `scope.onmessageerror = () => this.onMessageError();` | hand: H_msgerr_base | L6569 «A page message that cannot be read: before the boot…» |
| 213 | `if (this.phase.kind === "unbooted") {` | ===->!==, if→false, if→true, "unbooted"->"booting" | L6499 «#2306 attempt 2. A sticky error in the dispose windo…»; L6569 «A page message that cannot be read: before the boot…» |
| 214 | `this.phase = { kind: "refused" };` | delete, "refused"->"disposed" | L6569 «A page message that cannot be read: before the boot…» |
| 215 | `this.failBoot(RESULT_INTERNAL);` | delete | L6569 «A page message that cannot be read: before the boot…» |
| 218 | `if (this.holdsHost()) this.sticky(RESULT_INTERNAL, this.lastRequ` | hand: H_msgerr_stop, if→false, if→true | L6499 «#2306 attempt 2. A sticky error in the dispose windo…»; L6569 «A page message that cannot be read: before the boot…» |
| 224 | `if (this.phase.kind === "unbooted") {` | ===->!==, if→false, if→true, "unbooted"->"booting" | the first Worker row (the boot helper throws) |
| 227 | `this.phase = { kind: "refused" };` | delete, "refused"->"disposed" | L6324 «A boot ends live or refused. Red if a boot that repo…» |
| 229 | `if (this.phase.kind !== "live") this.failBoot(result === RESULT_` | ===->!==, if→false, if→true, "live"->"disposing", !==->===, RESULT_OK ? RESULT_INTERNAL->RESULT_OK ? RESULT_OK | L6267 «D6: a refused boot answers with the engine's result…»; the first Worker row (the boot helper throws); L6324 «A boot ends live or refused. Red if a boot that repo…» |
| 230 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 232 | `if (!this.holdsHost()) {` | hand: feed-port branch before the host check, if→false, if→true, !this.->this. | L6600 «A feed port is opened only while a host is held. Red…»; the first Worker row (the boot helper throws) |
| 235 | `if (message?.tag === "miso.dispose.v1" && Number.isSafeInteger(m` | &&->\|\|, ===->!==, if→false, if→true | L6324 «A boot ends live or refused. Red if a boot that repo…»; the first Worker row (the boot helper throws) |
| 236 | `this.answerDispose(message.requestId);` | delete | the first Worker row (the boot helper throws) |
| 238 | `return;` | delete return | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 240 | `if (message?.tag === "miso.feed-port.v1" && exactFields(message,` | &&->\|\|, ===->!==, if→false, if→true | L6613 «A message outside the protocol is never taken for a…»; L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; the first Worker row (the boot helper throws) |
| 241 | `&& typeof message.port?.postMessage === "function") {` | &&->\|\|, ===->!== | L6613 «A message outside the protocol is never taken for a…»; L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 242 | `this.openFeed(message.port);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 243 | `return;` | delete return | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 245 | `this.receive(message);` | delete | the first Worker row (the boot helper throws) |
| 246 | `const requestId = message?.tag === "miso.service.v1" ? 0 : this.` | hand: H_requestid_zero, ===->!== | L6531 «A control-work refusal that follows a page request c…» |
| 247 | `this.afterControl(requestId);` | delete | the first Worker row (the boot helper throws) |
| 252 | `return this.phase.kind === "live" \|\| this.phase.kind === "dispos` | "disposing"->"live", ===->!==, "live"->"disposing", delete return, \|\|->&& | the first Worker row (the boot helper throws) |
| 260 | `return this.phase.kind === "live" && this.stickyResult === RESUL` | &&->\|\|, ===->!==, "live"->"disposing", delete return | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…»; the first Worker row (the boot helper throws) |
| 264 | `this.drainFeeds();` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 265 | `this.serviceControl(requestId);` | delete | the first Worker row (the boot helper throws) |
| 266 | `this.pushWatermark(requestId);` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 267 | `this.drainControlEvents(requestId);` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 268 | `this.postMeterView();` | delete | the first Worker row (the boot helper throws) |
| 272 | `return this.memory.buffer;` | delete return | the first Worker row (the boot helper throws) |
| 276 | `return new Constructor(this.memory.buffer, pointer, length);` | delete return | the first Worker row (the boot helper throws) |
| 326 | `if (typeof ready === "number") return this.disposeAfterFailedBoo` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6407 «Every refused boot frees what it booted and stays re…» |
| 330 | `if (!u32(tlsSize) \|\| !u32(tlsAlign)) return this.disposeAfterFai` | if→false, if→true, \|\|->&& | L6407 «Every refused boot frees what it booted and stays re…»; the first Worker row (the boot helper throws) |
| 332 | `if (!u32(reservation) \|\| reservation === 0` | ===->!==, if→false, if→true, \|\|->&& | the first Worker row (the boot helper throws); L6407 «Every refused boot frees what it booted and stays re…» |
| 333 | `\|\| reservation + RESERVATION_BYTES > this.buffer.byteLength` |  > -> >= , \|\|->&& | L6685 «A reservation that ends exactly at the memory's end…»; L6407 «Every refused boot frees what it booted and stays re…» |
| 334 | `\|\| this.view(DataView, reservation, RESERVATION_BYTES).getUint32` | !==->===, \|\|->&&, true->false | the first Worker row (the boot helper throws) |
| 335 | `return this.disposeAfterFailedBoot(RESULT_INTERNAL);` | delete return | L6407 «Every refused boot frees what it booted and stays re…» |
| 339 | `const unsigned = typeof token === "bigint" ? BigInt.asUintN(64, ` | ===->!== | the first Worker row (the boot helper throws) |
| 340 | `if (unsigned === 0n \|\| Number(unsigned >> 32n) !== this.handle) ` | ===->!==, if→false, if→true, !==->===, \|\|->&& | the first Worker row (the boot helper throws); L6407 «Every refused boot frees what it booted and stays re…» |
| 341 | `return this.disposeAfterFailedBoot(RESULT_INTERNAL);` | delete return | L6407 «Every refused boot frees what it booted and stays re…» |
| 343 | `this.phase = { kind: "live" };` | delete, "live"->"disposing" | the first Worker row (the boot helper throws) |
| 344 | `this.bootToken = unsigned;` | delete | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 345 | `this.lastRenderCheck = clock();` | delete | L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 361 | `this.startServiceTick();` | delete | the first Worker row (the boot helper throws) |
| 513 | `return result;` | delete return | L6407 «Every refused boot frees what it booted and stays re…» |
| 519 | `this.scope.postMessage({ tag: "miso.error.v1", requestId: 0, res` | delete | L6267 «D6: a refused boot answers with the engine's result…» |
| 534 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | L5810 «#1332 S8: the release is bounded. With no release wi…»; L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 535 | `if (clock() >= this.phase.deadline) this.finishDispose(0n, "rele` | >=->>, if→false, if→true | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 536 | `return;` | delete return | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…» |
| 538 | `if (!this.controlLive()) {` | if→false, if→true, !this.->this. | L6339 «In a live session the stages run on the tick, and th…»; L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 539 | `this.stopServiceTick();` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 540 | `return;` | delete return | L6368 «A sticky error is latched. Red if an out-of-order re…» |
| 542 | `this.afterControl(0);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 543 | `this.checkRender();` | delete | L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 550 | `if (now - this.lastRenderCheck < RENDER_CHECK_MS) return;` | if→false, if→true,  < -> <= , delete return | the first Worker row (the boot helper throws); L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 551 | `this.lastRenderCheck = now;` | delete | L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 552 | `this.workletPort.postMessage({ tag: "miso.render-check.v1" });` | delete | L5847 «#1332 S8: the service tick asks the worklet whether…» |
| 556 | `if (this.serviceTick === null) return;` | ===->!==, if→true, delete return | the first Worker row (the boot helper throws) |
| 557 | `clearInterval(this.serviceTick);` | delete | the first Worker row (the boot helper throws) |
| 558 | `this.serviceTick = null;` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 562 | `this.scope.postMessage(message, transfer);` | delete | the first Worker row (the boot helper throws) |
| 567 | `if (planes !== undefined) message.planes = planes;` | if→false, if→true, !==->=== | L6432 «While a dispose runs, every request but a dispose is…»; the first Worker row (the boot helper throws) |
| 568 | `this.post(message, this.transferList(planes));` | delete | the first Worker row (the boot helper throws) |
| 572 | `if (!Array.isArray(planes)) return [];` | if→false, if→true, delete return | the first Worker row (the boot helper throws); L6695 «A refused chunk's planes go back by transfer, and a…» |
| 574 | `.filter((plane) => plane instanceof Float32Array && plane.buffer` | &&->\|\| | the first Worker row (the boot helper throws) |
| 585 | `sticky(result, requestId, planes) {` | hand: H_sticky_stop | L6499 «#2306 attempt 2. A sticky error in the dispose windo…» |
| 586 | `if (this.stickyResult === RESULT_OK && this.handle !== 0) {` | ===->!==, if→false, !==->=== | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 588 | `this.exports.miso_engine_web_v1_session_fail(this.handle, result` | delete | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 593 | `this.stickyResult = result;` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 595 | `if (planes !== undefined) message.planes = planes;` | if→false, if→true, !==->=== | L6368 «A sticky error is latched. Red if an out-of-order re…»; L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 596 | `this.post(message, this.transferList(planes));` | delete | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 600 | `if (message?.tag === "miso.service.v1" && exactFields(message, [` | if→true | the first Worker row (the boot helper throws) |
| 602 | `if (!Number.isSafeInteger(message?.requestId) \|\| message.request` | if→false, if→true, <=-><, \|\|->&& | L5860 «Issue #2267: a sticky error this Worker finds in Jav…»; the first Worker row (the boot helper throws) |
| 603 | `this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, re` | hand: H_order_stop, delete, ??->\|\| | L6499 «#2306 attempt 2. A sticky error in the dispose windo…»; L5860 «Issue #2267: a sticky error this Worker finds in Jav…»; L6742 «A request whose ID is not a safe integer gets its st…» |
| 604 | `return;` | delete return | L6368 «A sticky error is latched. Red if an out-of-order re…» |
| 606 | `this.lastRequestId = message.requestId;` | delete | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 607 | `if (message.tag === "miso.dispose.v1" && exactFields(message, ["` | &&->\|\|, ===->!==, if→false, if→true | the first Worker row (the boot helper throws) |
| 608 | `this.beginDispose(message.requestId);` | delete | the first Worker row (the boot helper throws) |
| 609 | `return;` | delete return | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…» |
| 611 | `if (this.stickyResult !== RESULT_OK) {` | if→false, if→true, !==->=== | L6368 «A sticky error is latched. Red if an out-of-order re…»; the first Worker row (the boot helper throws) |
| 612 | `this.sticky(this.stickyResult, message.requestId, returnedPlanes` | delete | L6368 «A sticky error is latched. Red if an out-of-order re…» |
| 613 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 615 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | the first Worker row (the boot helper throws) |
| 616 | `this.refuseDisposing(message);` | delete | the first Worker row (the boot helper throws) |
| 617 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 619 | `if (message.tag === "miso.source.v1" && exactFields(message, SOU` | &&->\|\|, ===->!==, if→false, if→true | L6709 «A live request reaches its own export, and the same…»; the first Worker row (the boot helper throws) |
| 620 | `this.receiveSource(message);` | delete | L6709 «A live request reaches its own export, and the same…» |
| 621 | `} else if (message.tag === "miso.seek.v1" && exactFields(message` | &&->\|\|, ===->!==, if→false, if→true | L6709 «A live request reaches its own export, and the same…»; L5998 «Issue #1387 (root ruling 1, attempts 2-3): the headr…»; the first Worker row (the boot helper throws) |
| 622 | `this.receiveSeek(message);` | delete | L5998 «Issue #1387 (root ruling 1, attempts 2-3): the headr…» |
| 623 | `} else if (message.tag === "miso.session-seek.v1" && exactFields` | &&->\|\|, ===->!==, if→false, if→true | L6709 «A live request reaches its own export, and the same…»; L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…»; the first Worker row (the boot helper throws) |
| 624 | `this.receiveSessionSeek(message);` | delete | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 625 | `} else if (message.tag === "miso.seek-at.v1" && exactFields(mess` | &&->\|\|, ===->!==, if→false, if→true | L6709 «A live request reaches its own export, and the same…»; L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; the first Worker row (the boot helper throws) |
| 626 | `this.receiveSeekAt(message);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 627 | `} else if (message.tag === "miso.seek-report.v1" && exactFields(` | &&->\|\|, ===->!==, if→false, if→true | L6709 «A live request reaches its own export, and the same…»; the first Worker row (the boot helper throws) |
| 628 | `this.receiveSeekReport(message);` | delete | L6709 «A live request reaches its own export, and the same…» |
| 629 | `} else if (message.tag === "miso.plan-latency.v1" && exactFields` | &&->\|\|, ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6709 «A live request reaches its own export, and the same…» |
| 630 | `this.receivePlanLatency(message);` | delete | L6709 «A live request reaches its own export, and the same…» |
| 631 | `} else if (message.tag === "miso.control.v1"` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws) |
| 632 | `&& (exactFields(message, CONTROL_FIELDS) \|\| exactFields(message,` | &&->\|\|, \|\|->&& | L6709 «A live request reaches its own export, and the same…»; the first Worker row (the boot helper throws) |
| 633 | `this.receiveControl(message);` | delete | the first Worker row (the boot helper throws) |
| 634 | `} else if (message.tag === "miso.sessionmap.v1" && exactFields(m` | &&->\|\|, ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 636 | `if (sources === null) {` | ===->!==, if→false, if→true | L5860 «Issue #2267: a sticky error this Worker finds in Jav…»; L6368 «A sticky error is latched. Red if an out-of-order re…» |
| 637 | `this.sticky(RESULT_INTERNAL, message.requestId);` | delete | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 638 | `return;` | delete return | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 645 | `metersAttached: this.metersAttached === true,` | ===->!==, true->false | L6368 «A sticky error is latched. Red if an out-of-order re…» |
| 647 | `} else if (message.tag === "miso.status.v1" && exactFields(messa` | &&->\|\|, ===->!==, if→false, if→true | L5860 «Issue #2267: a sticky error this Worker finds in Jav…»; the first Worker row (the boot helper throws) |
| 649 | `this.statusReads.push(message.requestId);` | delete | the first Worker row (the boot helper throws) |
| 650 | `this.workletPort.postMessage({ tag: "miso.render-status.v1", req` | delete | the first Worker row (the boot helper throws) |
| 652 | `this.sticky(RESULT_INVALID_ARGUMENT, message.requestId, returned` | delete | L5860 «Issue #2267: a sticky error this Worker finds in Jav…» |
| 661 | `if (tag === "miso.source.v1" \|\| tag === "miso.seek.v1" \|\| tag ==` | ===->!==, if→false, if→true, \|\|->&& | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 662 | `this.acknowledge(requestId, RESULT_WRONG_STATE, tag === "miso.so` | delete, ===->!== | L6432 «While a dispose runs, every request but a dispose is…» |
| 663 | `} else if (tag === "miso.session-seek.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 664 | `this.post({ tag, requestId, result: RESULT_WRONG_STATE, generati` | delete | L6432 «While a dispose runs, every request but a dispose is…» |
| 665 | `} else if (tag === "miso.seek-report.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 666 | `this.post({ tag, requestId, result: RESULT_WRONG_STATE, report: ` | delete | L6432 «While a dispose runs, every request but a dispose is…» |
| 667 | `} else if (tag === "miso.plan-latency.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 668 | `this.post({ tag, requestId, result: RESULT_WRONG_STATE, latency:` | delete | L6432 «While a dispose runs, every request but a dispose is…» |
| 669 | `} else if (tag === "miso.control.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws) |
| 675 | `} else if (tag === "miso.status.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 676 | `this.answerStatus(requestId, null);` | delete | L6432 «While a dispose runs, every request but a dispose is…» |
| 677 | `} else if (tag === "miso.sessionmap.v1") {` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6432 «While a dispose runs, every request but a dispose is…» |
| 680 | `this.post(sources === null` | ===->!== | the first Worker row (the boot helper throws) |
| 682 | `: { tag, requestId, result: RESULT_OK, sources, metersAttached: ` | ===->!==, true->false | L6432 «While a dispose runs, every request but a dispose is…» |
| 685 | `this.acknowledge(requestId, RESULT_WRONG_STATE);` | delete | L6432 «While a dispose runs, every request but a dispose is…» |
| 856 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…»; the first Worker row (the boot helper throws) |
| 858 | `if (result !== RESULT_OK) this.sticky(result, requestId);` | if→false, if→true, !==->=== | L6339 «In a live session the stages run on the tick, and th…»; the first Worker row (the boot helper throws) |
| 862 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…»; L6339 «In a live session the stages run on the tick, and th…» |
| 865 | `if (result === RESULT_BACKPRESSURE) return;` | ===->!==, if→false, if→true, delete return | L6339 «In a live session the stages run on the tick, and th…»; L6641 «The watermark the stages post. A boot whose watermar…» |
| 866 | `if (result !== RESULT_OK) {` | if→false, if→true, !==->=== | L6339 «In a live session the stages run on the tick, and th…»; the first Worker row (the boot helper throws) |
| 867 | `this.sticky(result, requestId);` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 871 | `if (view.getUint32(0, true) !== WATERMARK_BYTES \|\| view.getUint3` | if→false, if→true, !==->===, \|\|->&&, true->false | L6641 «The watermark the stages post. A boot whose watermar…»; the first Worker row (the boot helper throws) |
| 872 | `this.sticky(RESULT_INTERNAL, requestId);` | delete | L6641 «The watermark the stages post. A boot whose watermar…» |
| 873 | `return;` | delete return | L6641 «The watermark the stages post. A boot whose watermar…» |
| 875 | `const revision = view.getBigUint64(8, true);` | true->false | L6339 «In a live session the stages run on the tick, and th…» |
| 876 | `const sample = view.getBigUint64(16, true);` | true->false | L6339 «In a live session the stages run on the tick, and th…» |
| 877 | `if (revision === this.watermarkRevision && sample === this.water` | &&->\|\|, ===->!==, if→false, if→true | L6641 «The watermark the stages post. A boot whose watermar…»; L6368 «A sticky error is latched. Red if an out-of-order re…»; L6339 «In a live session the stages run on the tick, and th…» |
| 878 | `const counts = [32, 40, 48].map((offset) => view.getBigUint64(of` | true->false | L6641 «The watermark the stages post. A boot whose watermar…» |
| 879 | `let outcome = view.getBigUint64(24, true);` | true->false | L6641 «The watermark the stages post. A boot whose watermar…» |
| 881 | `if (count > this.watermarkCounts[index]) outcome \|= WATERMARK_CO` |  > -> >= , if→false, if→true | L6641 «The watermark the stages post. A boot whose watermar…» |
| 883 | `this.watermarkRevision = revision;` | delete | L6641 «The watermark the stages post. A boot whose watermar…» |
| 884 | `this.watermarkSample = sample;` | delete | L6641 «The watermark the stages post. A boot whose watermar…» |
| 885 | `this.watermarkCounts = counts;` | delete | L6641 «The watermark the stages post. A boot whose watermar…» |
| 893 | `if (!this.controlLive()) return;` | if→false, if→true, !this.->this. | L6273 «Issue #2306 (#2258 WEB-7): the Worker's lifecycle ph…»; L6339 «In a live session the stages run on the tick, and th…» |
| 896 | `for (let calls = 0; calls < MAXIMUM_CONTROL_EVENTS_PER_LANE; cal` |  < -> <=  | L6672 «The event drain takes at most 16 frames a lane per p…» |
| 898 | `if (result !== RESULT_OK) {` | if→false, if→true, !==->=== | L6339 «In a live session the stages run on the tick, and th…»; the first Worker row (the boot helper throws) |
| 899 | `this.sticky(result, requestId);` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 900 | `return;` | delete return | L6339 «In a live session the stages run on the tick, and th…» |
| 903 | `if (bytes === 0) break;` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6339 «In a live session the stages run on the tick, and th…» |
| 905 | `if (pointer === 0) {` | ===->!==, if→false, if→true | L6339 «In a live session the stages run on the tick, and th…»; L6672 «The event drain takes at most 16 frames a lane per p…» |
| 906 | `this.sticky(RESULT_INTERNAL, requestId);` | delete | L6672 «The event drain takes at most 16 frames a lane per p…» |
| 907 | `return;` | delete return | L6672 «The event drain takes at most 16 frames a lane per p…» |
| 910 | `frame.set(this.view(Uint8Array, pointer, bytes));` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 911 | `this.post({ tag: "miso.control-event.v1", lane, frame }, [frame.` | delete | L6339 «In a live session the stages run on the tick, and th…» |
| 940 | `if (data === null \|\| typeof data !== "object") return;` | ===->!==, if→false, if→true, !==->===, delete return, \|\|->&& | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; the first Worker row (the boot helper throws) |
| 941 | `if (data.op === "attach" && Array.isArray(data.rings)) {` | &&->\|\|, ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 942 | `this.withdrawRings(feed.delivered);` | delete | L6751 «The feed port's messages. Red if a message that is n…» |
| 943 | `feed.delivered = data.rings;` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 944 | `if (this.phase.kind !== "live") return;` | if→false, if→true, "live"->"disposing", !==->===, delete return | L6463 «Once a dispose began, no ring is drained, bound or p…»; L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 947 | `if (ring === null) continue;` | ===->!==, if→false, if→true | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; the first Worker row (the boot helper throws) |
| 948 | `Atomics.store(ring.control, RING.ATTACHED, 1);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 949 | `this.rings.push(ring);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 951 | `} else if (data.op === "detach") {` | ===->!==, if→false, if→true | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 953 | `this.withdrawRings(feed.delivered);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 955 | `this.feeds = this.feeds.filter((entry) => entry !== feed);` | !==->=== | L6751 «The feed port's messages. Red if a message that is n…» |
| 956 | `feed.port.onmessage = null;` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 958 | `} else if (data.op === "detach-source") {` | ===->!==, if→false, if→true | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 961 | `const shared = Number.isSafeInteger(index) && index >= 0 && inde` | &&->\|\|, >=->> | L6751 «The feed port's messages. Red if a message that is n…»; L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 962 | `? feed.delivered[index] : null;` | delete | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 963 | `if (shared != null) {` | if→false | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 964 | `this.withdrawRings([shared]);` | delete | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 965 | `feed.delivered = feed.delivered.map((entry, at) => (at === index` | ===->!== | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 967 | `feed.port.postMessage({ op: "source-detached", requestId: data.r` | delete | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 968 | `} else if (data.op === "prepare-seek") {` | ===->!==, if→false, if→true | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…»; L6751 «The feed port's messages. Red if a message that is n…» |
| 969 | `const delivered = feed.delivered.filter((entry) => entry !== nul` | !==->=== | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 970 | `feed.port.postMessage({ op: "seek-prepared", requestId: data.req` | delete | L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 1119 | `if (this.rings.length === 0 \|\| !this.controlLive()) return;` | ===->!==, if→false, if→true, !this.->this., \|\|->&& | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; L6463 «Once a dispose began, no ring is drained, bound or p…» |
| 1123 | `this.drainSharedRing(ring, rendered);` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 1125 | `ring.control[RING.ERRORS] += 1;` | delete | L6751 «The feed port's messages. Red if a message that is n…» |
| 1388 | `if (!this.controlLive() \|\| !Array.isArray(seeks) \|\| seeks.length` | if→false, if→true, !==->===, !this.->this., \|\|->&& | L6463 «Once a dispose began, no ring is drained, bound or p…»; L6159 «Issue #1387 (MAJOR-2): the shared MSB1 consumer case…» |
| 1389 | `return { kind: "refused", result: RESULT_WRONG_STATE };` | delete return, "refused"->"disposed" | L6463 «Once a dispose began, no ring is drained, bound or p…» |
| 1438 | `if (this.phase.kind !== "live") return;` | if→false, if→true, "live"->"disposing", !==->=== | L5810 «#1332 S8: the release is bounded. With no release wi…»; the first Worker row (the boot helper throws) |
| 1443 | `if (header === this.postedHeader && memoryBytes === this.postedM` | &&->\|\|, ===->!==, if→false, if→true, delete return | the first Worker row (the boot helper throws) |
| 1444 | `this.postedHeader = header;` | delete | the first Worker row (the boot helper throws) |
| 1445 | `this.postedMemoryBytes = memoryBytes;` | delete | the first Worker row (the boot helper throws) |
| 1446 | `this.workletPort.postMessage({ tag: "miso.meter-view.v1", header` | delete | the first Worker row (the boot helper throws) |
| 1459 | `if (this.phase.kind === "disposing") {` | "disposing"->"live", ===->!==, if→false, if→true | the first Worker row (the boot helper throws) |
| 1460 | `this.phase.requestIds.push(requestId);` | delete | the first Worker row (the boot helper throws) |
| 1461 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 1463 | `this.phase = { kind: "disposing", requestIds: [requestId], deadl` | "disposing"->"live", delete | the first Worker row (the boot helper throws) |
| 1464 | `this.workletPort.postMessage({ tag: "miso.release.v1" });` | delete | the first Worker row (the boot helper throws) |
| 1466 | `if (this.serviceTick === null) this.startServiceTick();` | ===->!==, if→false, if→true | the first Worker row (the boot helper throws); L6339 «In a live session the stages run on the tick, and th…» |
| 1471 | `if (phase.kind !== "disposing") return;` | "disposing"->"live", if→false, if→true, !==->===, delete return | the first Worker row (the boot helper throws) |
| 1472 | `this.stopServiceTick();` | delete | the first Worker row (the boot helper throws) |
| 1481 | `if (token !== 0n) e.miso_engine_web_v1_host_adopt(token);` | if→false, if→true, !==->=== | the first Worker row (the boot helper throws); L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1482 | `else if (this.bootToken !== 0n) e.miso_engine_web_v1_host_adopt(` | delete, if→false, !==->=== | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1484 | `while (this.statusReads.length > 0) this.answerStatus(this.statu` | delete,  > -> >=  | the first Worker row (the boot helper throws); (never finishes) |
| 1485 | `result = e.miso_engine_web_v1_dispose(this.handle);` | delete | the first Worker row (the boot helper throws) |
| 1488 | `result = RESULT_INTERNAL;` | delete | L5798 «A trap in the Worker's dispose is a final outcome: t…» |
| 1490 | `this.phase = { kind: "disposed", outcome: result, reason };` | "disposed"->"disposing", delete | the first Worker row (the boot helper throws) |
| 1492 | `this.closeFeeds();` | delete | L6463 «Once a dispose began, no ring is drained, bound or p…» |
| 1493 | `for (const requestId of phase.requestIds) this.answerDispose(req` | delete | the first Worker row (the boot helper throws) |
| 1500 | `if (phase.kind === "disposed" && phase.reason !== undefined) {` | &&->\|\|, "disposed"->"disposing", ===->!==, if→false, if→true, !==->=== | the first Worker row (the boot helper throws); L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1501 | `this.post({ tag: "miso.teardown.v1", requestId, result: RESULT_I` | delete | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1503 | `this.acknowledge(requestId, phase.kind === "disposed" ? phase.ou` | "disposed"->"disposing", delete, ===->!==, phase.outcome : RESULT_INTERNAL->phase.outcome : RESULT_OK | the first Worker row (the boot helper throws); L6324 «A boot ends live or refused. Red if a boot that repo…» |
| 1511 | `if (token === 0n) return;` | ===->!==, if→false, if→true, delete return | L5810 «#1332 S8: the release is bounded. With no release wi…»; L6302 «Once the host is freed, a worklet message other than…» |
| 1513 | `if (this.exports.miso_engine_web_v1_host_adopt(token) === RESULT` | ===->!==, if→false, if→true | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1514 | `this.exports.miso_engine_web_v1_dispose(Number(token >> 32n));` | delete | L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1522 | `if (message?.tag === "miso.released.v1" && exactFields(message, ` | &&->\|\|, ===->!==, if→false, if→true | L6613 «A message outside the protocol is never taken for a…»; the first Worker row (the boot helper throws) |
| 1523 | `&& typeof message.token === "bigint") {` | &&->\|\|, ===->!== | L6613 «A message outside the protocol is never taken for a…»; the first Worker row (the boot helper throws) |
| 1525 | `if (this.phase.kind === "disposed") this.reclaimOrphan(token);` | "disposed"->"disposing", ===->!==, if→false, if→true, reclaimOrphan(token);       else this.finishDispose(token);->finishDispose(token);       else this.reclaimOrphan(token); | the first Worker row (the boot helper throws); L5810 «#1332 S8: the release is bounded. With no release wi…» |
| 1526 | `else this.finishDispose(token);` | delete | the first Worker row (the boot helper throws) |
| 1527 | `return;` | delete return | L6397 «In a live session, a release nobody asked for frees…» |
| 1531 | `if (!this.holdsHost()) return;` | if→false, if→true, delete return, !this.->this. | L6302 «Once the host is freed, a worklet message other than…»; the first Worker row (the boot helper throws); L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 1532 | `if (message?.tag === "miso.render-clock.v1" && exactFields(messa` | &&->\|\|, ===->!==, if→false, if→true | L6613 «A message outside the protocol is never taken for a…»; L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…»; the first Worker row (the boot helper throws) |
| 1533 | `&& u32(message.status) && message.status !== 0 && this.renderSta` | &&->\|\|, ===->!==, !==->=== | L6556 «A worklet status reply that is no record, and a cont…»; the first Worker row (the boot helper throws); L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 1534 | `this.renderStatusPointer = message.status;` | delete | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 1535 | `return;` | delete return | L5889 «Issue #1387 (S11): the MSB1 feed drain of `worker` m…» |
| 1537 | `if (message?.tag === "miso.render-status.v1" && exactFields(mess` | &&->\|\|, ===->!==, if→false, if→true | L6613 «A message outside the protocol is never taken for a…»; the first Worker row (the boot helper throws); L6397 «In a live session, a release nobody asked for frees…» |
| 1538 | `&& this.statusReads[0] === message.requestId) {` | &&->\|\|, ===->!== | L6397 «In a live session, a release nobody asked for frees…»; the first Worker row (the boot helper throws) |
| 1539 | `this.statusReads.shift();` | delete | L6540 «A status read is answered once: by the worklet's rep…» |
| 1540 | `this.answerStatus(message.requestId, message.status);` | delete | the first Worker row (the boot helper throws) |
| 1541 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 1543 | `this.sticky(RESULT_INTERNAL, this.lastRequestId);` | hand: H_worklet_stop, delete | L6499 «#2306 attempt 2. A sticky error in the dispose windo…»; L6397 «In a live session, a release nobody asked for frees…» |
| 1556 | `control = this.readStatus();` | delete | the first Worker row (the boot helper throws) |
| 1558 | `this.sticky(RESULT_INTERNAL, requestId);` | delete | L6556 «A worklet status reply that is no record, and a cont…» |
| 1559 | `return;` | delete return | the first Worker row (the boot helper throws) |
| 1561 | `if (rendered !== null && typeof rendered !== "object") {` | &&->\|\|, if→false, if→true, !==->=== | the first Worker row (the boot helper throws); L6556 «A worklet status reply that is no record, and a cont…» |
| 1562 | `this.sticky(RESULT_INTERNAL, requestId);` | delete | L6556 «A worklet status reply that is no record, and a cont…» |
| 1563 | `return;` | delete return | L6556 «A worklet status reply that is no record, and a cont…» |
| 1573 | `state: control.state !== STATE_READY ? control.state : render.st` | !==->=== | L6540 «A status read is answered once: by the worklet's rep…» |
| 1574 | `lastResult: render.lastResult \|\| control.lastResult,` | \|\|->&& | L6540 «A status read is answered once: by the worklet's rep…» |

**Green, equivalent (29).**
- **Constructor initialisers with the same effect undeleted** (184 `handle`, 185 `serviceTick`, 196 `bootToken`, 197 `lastRenderCheck`, 191 `postedMemoryBytes`, 204 `lastSessionSeekGeneration`).
  - `handle`, `serviceTick`, `bootToken`, `lastRenderCheck` and `postedMemoryBytes` are each read only after `boot()` assigned them. `bootToken` is read only in `disposing`; `lastRenderCheck` only on a live tick; `serviceTick` only after the boot started the tick; `postedMemoryBytes` is set at line 346, before any meter view.
  - Before the boot nothing calls `sticky`, which reads `handle`: an unreadable message then goes to `failBoot`.
  - `lastSessionSeekGeneration = undefined` deleted leaves the same value.
- **Sentinels that match nothing either way** (186 `watermarkRevision`, 187 `watermarkSample`, 190 `postedHeader`, 203 `renderClockWords`).
  - `undefined`, like `-1n`, equals no `u64` word, so the first watermark is posted either way.
  - `undefined`, like `-1`, equals no `u32` header.
  - `undefined?.buffer`, like `null?.buffer`, is not the memory's buffer, so the view is made either way.
- **`postedMemoryBytes` at the boot deleted (346).** `postedHeader` starts at -1, so the first view is posted anyway.
- **`onMessageError`'s `return` after the refused boot deleted (216).** The next statement is guarded by `holdsHost()`, which is false in `refused`.
- **`boot`'s `return RESULT_OK` deleted (362).** On a success, `onPageMessage` reads the phase, not the result.
- **`handle = 0` deleted in `disposeAfterFailedBoot` and `finishDispose` (512, 1491).** With no host, nothing reads `handle` but `sticky`'s `session_fail`. That `sticky` is reachable only through `onMessageError`, which now drops a message with no host.
- **`stopServiceTick`'s null return forced false (556).** The mutant calls `clearInterval(null)`, a no-op in HTML.
- **`sticky` calls `session_fail` every time (586, two mutants).** The engine refuses a second fail and a freed handle, and changes nothing (`ffi.rs:1241-1243`).
- **`receive`'s `miso.service.v1` return (600, three mutants).** In `worker` mode the page sends no service message: it starts its tick only when there is no Worker (`audio-worklet-host.js:580`). This is the one rule attempt 1 got right; its line-232 claim, about the request ID that `afterControl` carries, was wrong, and the request-ID row now pins it.
- **`refuseDisposing`'s `seek || seek-at` as `seek && seek-at` (661).** A seek or seek-at then falls through to the last branch, which posts the identical ack.
- **`pushWatermark`'s `return` after a refused read's sticky error deleted (868).** A refused read writes no record (`lib.rs:2036-2045`), and every read that did write either posted its revision and sample or found them equal to the stored ones. So the dedupe then returns.
- **The reserved word read big-endian (871, the second `true`).** The word must be zero, and zero reads as zero in either byte order.
- **A detached feed's `delivered = []` deleted (954).** The feed is dropped from `feeds` and its port's handler is cleared, so nothing reads that list again.
- **`detach-source`'s `<` as `<=` (961), and its `shared != null` forced true (963).** An index the mutants admit names no element, so `shared` is undefined and nothing is withdrawn or replaced. The reply's `detached` is computed apart, from the same `shared`.
- **The boot-token adoption forced true (1482).** `bootToken` is never 0 in `disposing`, because a boot reaches `live` only with a non-zero token (line 340).
- **`answerStatus`'s `rendered ?? …` as `||` (1565).** `rendered` is `null` or an object there, since anything else was refused just above, and every object is truthy.

**Follow-up for WEB-2 (not fixed here).** The probe for finding 2 shows a cost on the page side. The sticky error that follows a page request carries that request's ID, and the request was already answered. The page then finds no pending request under that ID, so it fails with 255 and the oldest request's ID (`audio-worklet-host.js`, the `miso.error.v1` branch), and the typed code 3 is lost. One `fail()` across the page and the Worker (WEB-2) is where that belongs.

**Render path.** Unchanged from attempt 1: no render code and no Rust changed, so the realtime policy tool, the `cargo` gates, the no-bit-moved comparison and loom do not apply.

**Gates run (attempt 2):**
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- `bash scripts/check-workspace-policy.sh`, `workspace policy: ok`;
- `bash scripts/check-sdk-headless.sh` with no argument, exit 0: 716 tests, 716 passed. It self-builds the artifact from this merged tree, so its real-Wasm `worker`-mode evals ran the attempt-2 Worker.

**Not run here:** the Playwright two-browser legs.
