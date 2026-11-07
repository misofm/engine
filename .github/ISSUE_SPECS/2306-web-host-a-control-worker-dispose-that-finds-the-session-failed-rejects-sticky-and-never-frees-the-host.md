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
  - `{ kind: "booting" }`: while `boot()` runs. It stays only when `boot()` throws; the page then gets the Worker's `error` event and terminates it;
  - `{ kind: "refused" }`: the boot was refused; nothing is held;
  - `{ kind: "live" }`: booted, with the render half released to the worklet;
  - `{ kind: "disposing", requestIds, deadline }`: the release was asked for. `requestIds` are the page's dispose requests, answered together. `deadline` is the release deadline;
  - `{ kind: "disposed", outcome, reason }`: the host is freed. `outcome` is the dispose's result and `reason` the typed teardown reason (`release-timeout`), if any.

  It replaces `booted`, `ready`, `disposing`, `disposed`, `disposeOutcome`, `disposeReason`, `disposeRequestIds` and `releaseDeadline`.
- **`stickyResult` stays a separate field.** A sticky error is the session's failure, not a lifecycle phase. A failed session is still `live`: it answers status reads and the session map, and it must still be disposed. Folding the failure into the union would repeat it in `live` and `disposing`. `ready` was always `live && stickyResult === RESULT_OK`, and is now derived as `controlLive()`.
- **`controlLive()`** is the one guard of control work: the feed drain, the service step, the watermark, the event drain, the seek preparation and the tick's work. It is false in every phase but `live`, and after a sticky error. This fixes claim 2: no control work runs once a dispose began. The tick already behaved this way.
- **A boot ends `live` or `refused`.** `onPageMessage` enters `booting` and calls `boot()`. If the phase is still `booting` afterwards, the boot is refused with its result, or with 255 when it returned `RESULT_OK` without a host.
- **Only `live` and `disposing` hold a host.** In every other phase a page message is dropped, except a dispose, which is answered at once: with the dispose's outcome once `disposed`, and 255 otherwise (as before). A worklet message is dropped too, except a late `miso.released.v1` once `disposed`, whose orphan is reclaimed. This fixes claim 3.
- **Unchanged:** the request-ID check, the order of the dispose and sticky checks in `receive`, `refuseDisposing`, and `fatal`'s posts in `live` and `disposing`.

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
- A guard table: every guard and every phase write in the Worker's lifecycle code is mutated, and each mutant is red on a named test or has a stated equivalence.
- `bash scripts/test-web-audioworklet.sh`.
- `bash scripts/check-workspace-policy.sh`.
- `bash scripts/build-web-audioworklet.sh`, then `bash scripts/check-sdk-headless.sh` on that build, so the real-Wasm `worker`-mode evals run the new Worker.
- No Rust changes. The realtime policy tool, the `cargo` gates and the no-bit-moved comparison do not apply: the Worker is the control thread, and no render code or PCM path changes. No cross-thread state is added, so no loom model applies.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `d441cd408`.
