# Web host: a dispose racing another request's failure is settled by that failure, not by its own reply

Tracker: #2258, item WEB-6. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The main-realm host (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`, class `MisoAudioWorkletHost`) keeps its lifecycle in loose fields: `#stickyError`, `#disposed`, `#disposePromise`, `#workerGone` and `#serviceTick` (base `76ff20b54` lines 501-507), plus a second test for "a dispose is in flight", `#disposePending()` (a scan of `#pending`, line 1201). Its request bounds live in nine parallel structures (`#inFlightSources`, `#inFlightSeeks`, `#inFlightSessionSeeks`, `#inFlightSeekReports`, `#inFlightRenderAllocations`, `#inFlightPlanLatency`, `#inFlightControl`, `#inFlightLease`, and `#pending` itself; lines 471-487), each reserved, released and cleared by hand.

Verified on base, by reading and by the tests below:

1. **A dispose racing a failure is rejected by the failure.** `#fail` (line 959) rejects every unsettled request, the dispose included, with the sticky error. However, a failure that arrives as a message, such as a typed `miso.error.v1` for a request sent before the dispose, or a malformed frame, does not stop the peer: the worklet (and in `worker` mode the control Worker) handles messages in arrival order and still answers the dispose (`miso-engine-v1-audio-worklet.js` `receive`, line 1150, answers a dispose before the failed-phase refusal; the Worker does the same at `control-worker.js` line 576). The dispose's own reply then finds no request and is dropped (line 763).
   - `single` mode: the dispose rejects with the other request's error (result 9 in the test). That is not 255, so `dispose()` treats it as a retryable refusal (line 1399): it drops the promise, restarts the service tick, and leaves the node connected, though the worklet has already disposed. A retry is sent to an ended worklet, which answers nothing (`receive`, line 1139), so it waits out `DISPOSE_TIMEOUT_MS` and tears down with `dispose-timeout`.
   - `worker` mode: the dispose rejects with the render failure and the page terminates the Worker before it answers. The same failure one message earlier, before `dispose()` was called, gives a clean dispose (the existing "#1332 S8" row).
2. **The sticky error is not sticky.** `#fail` overwrites `#stickyError` on every call. A later failure (a `processorerror`, a Worker `error`, a malformed frame) replaces the typed error that requests have already been rejected with, so later calls reject with a different cause. The "#1332 S9" row's comment ("the typed error stands") states the intended contract.
3. **`#fail` forgets `#inFlightLease`** (the tracker's claim). It holds, but is unobservable today: after a failure every request except the dispose is refused with the sticky error before its bound is checked (line 1028). It shows that `#fail` and the bounds are kept in step by hand, which is what the explicit model removes.

"Dispose in progress" does have two representations (`#disposePromise !== null` and `#disposePending()`). On base they disagree only after `#fail` has removed the dispose from `#pending`, and every caller then reaches the same refusal (result 3) through `#request`, so the disagreement is not itself observable.

## Model
- **One `#phase`**, an immutable record that only the transitions write:
  - `live`: requests are accepted. In `single` mode the service tick runs, and only in this phase.
  - `failed { error, workerGone }`: sticky. Only `dispose()` is accepted.
  - `disposing { promise, failure }`: `dispose()` was called. `failure` is `null` or the `failed` record. Every other request is refused with result 3, and `dispose()` returns `promise`.
  - `disposed { promise }`: torn down. Final.

  `#stickyError`, `#disposed`, `#disposePromise`, `#workerGone` and `#disposePending()` are gone. Single-mode retry is a transition from `disposing` back to `failure ?? live`, and only the `live` arm restarts the tick.
- **`#fail(error, source)` is the one failure transition.** `source` says what failed:
  - `FAIL_CHANNEL`: a port's `messageerror` or the processor's `processorerror`.
  - `FAIL_WORKER_GONE`: the control Worker's `error`.
  - the pending request whose reply was the failing message.
  - `null`: an unsolicited frame, or a message that answers no request.

  The first failure's error stands; a later call adds only `workerGone`. A dispose already sent survives the failure unless the source is the channel, the Worker, or the dispose's own reply. It is then settled by its own reply, or by the `DISPOSE_TIMEOUT_MS` bound. It never waits unbounded.
- **One in-flight table.** Each bounded request carries a slot, such as `source:<id>`, `seek:<id>`, `lease:<kind>` or `control`, with its bound. One `#inFlight` map counts unsettled requests per slot. `#reserve`, `#release`, `#fail` and `#teardown` each touch only that map, so no structure can be forgotten.

**Where the page-side follow-up attaches.** This is not done here; see Non-goals. The branch of `#fail` that runs on the first entry into `failed` (`prior === null`) is the host's single failure transition. A follow-up that tells the engine about page-detected failures adds its post there. `source` already names a peer that is gone (`FAIL_WORKER_GONE`: post to the worklet only; `FAIL_CHANNEL` from `processorerror`: post to the Worker only). The follow-up still needs to tell an engine-reported typed `miso.error.v1` (which needs no post) from a page-detected schema failure. It does this by adding that distinction to `source` at the `#receive` call sites. No other site enters `failed`.

## Non-goals
- Page-detected failures reaching the worklet or the Worker (a new worklet message tag; the follow-up listed in #2267's spec). It is its own issue.
- `miso-engine-v1-control-worker.js` (WEB-5 and WEB-7) and `web/stem-store/`.
- `#statusRead`/`#statusFollow` (the shared status read) are unchanged; they are already one owner.
- No render-path, worklet or Rust change. Public behaviour changes only in the two races above, and in one record shape (attempt 4): a sticky error caused by a stray message whose `requestId` is not a request ID carries request ID 0 instead of echoing it.

## Gates
- `scripts/test-web-audioworklet.mjs` rows, red on base:
  - single mode: a dispose racing a control request's typed failure resolves with its own reply, disconnects the node and restarts no tick;
  - single mode: after a typed failure, a `processorerror` leaves the typed error standing;
  - worker mode: a dispose racing a worklet render failure resolves cleanly once the Worker answers.
- A failure-source table (attempt 2; it absorbs attempt 1's guard row). Each of these settles a dispose in flight at once:
  - its own reply in every shape: a typed error (rejects with it, retryable, no tick; red on base at the tick), a malformed `miso.error.v1`, a malformed ack, and a worker-mode teardown with another reason (each 255, torn down);
  - `messageerror` in both modes, and `processorerror`;
  - a fence that cannot be posted;
  - a `processorerror` after the dispose survived a typed failure (rejects with 255, not 9, and disconnects).

  A failed host whose Worker is gone disposes at once with the first error and sends no dispose, whichever of the two failures came first. `openFeedPort` is `null` while failed and while disposing.

  The table's other half (attempt 3): a dispose is *not* settled by a failure another message brought. That covers a malformed meter frame, telemetry frame, control event or watermark, and a reply that matches no request. Each fails the host but leaves the dispose pending until its own reply.
- A bounds table (attempt 2) covering every bounded kind: source, seek, seek-at, seek report, session seek, render allocations, plan latency, control, meters, telemetry and session map. For each kind:
  - the request past the bound is refused with `RESULT_BACKPRESSURE`, posts nothing and burns no ID;
  - another key of the kind, and every kind with a slot of its own, is accepted (seek-at shares seek's slot and is refused);
  - settling one request frees exactly one place.

  The scattered single-kind refusals it supersedes are deleted.
- An early-failure row covering every order: `error`, `messageerror`, `messageerror` then `error`, `error` then `messageerror` (attempt 3). The Worker is terminated exactly when it raised `error`, and the dispose then tears down at once and sends nothing.
- A MAX_SAFE row (attempt 3): in worker mode, a held control reply whose fence gets no request ID. It fails the host with the control request's ID and posts no fence. Its dispose, which gets no ID either, rejects with result 1 and still terminates the Worker.
- Attempt 4 rows (from the reviewer's repros and the 794-mutant generator):
  - in the "does not settle" half: another request's reply, malformed as a `miso.error.v1` or failing its shape check (single mode), or a teardown carrying another request's ID (worker mode);
  - a channel failure in worker mode (worklet `processorerror` or `messageerror`, Worker `messageerror`, a fence that cannot be posted, an early `messageerror`): a later dispose is sent once to the live Worker and settles on its reply;
  - a duplicate control reply, which frees one place of the control bound, not two;
  - an unclean worker-mode dispose, which refuses a held control reply with 3;
  - a status follow-up still waiting at teardown, which rejects with 3;
  - a late control reply on a failed host, which posts no fence;
  - a release-timeout teardown, which refuses an overtaken request with 3;
  - a worker-mode dispose whose own reply is a typed error, which settles at once;
  - a malformed worker-mode control reply, which posts no fence;
  - a fence answered with a refusal, which fails the host;
  - a malformed unsolicited frame, which reaches no listener;
  - a stray message's sticky error carries ID 0 unless it has a request ID;
  - a channel failure's sticky error carries the oldest unsettled request's ID.
- Three rows for guards that no row pinned (attempt 2): an early `messageerror` fails the host the factory returns; a status read that failed to post is not kept as the shared read; a reply that matches no request fails a live host.
- Every existing row of `bash scripts/test-web-audioworklet.sh` stays green, including the safe-integer allocator mutation it runs.
- Mutations of the fix, each red at its row: make `#fail` reject a dispose whose reply is still due; let a later `#fail` replace the first error; keep the dispose on its own reply's failure, on a channel failure, or on the Worker's `error`; restart the tick on a retry into `failed`. Every `if` guard and every `#fail` call between `inFlightSlot` and the end of `bootControlWorker` (lines 466-1581) was mutated mechanically (the condition set to `false`; the source swapped). Each mutant is either red at a named row or named as equivalent in the attempt-3 guard table. Attempt 4 re-ran the reviewer's broader generator over the same lines. Every green mutant in the lifecycle, failure, dispose, status-sharing and bound code is named there with its equivalence; the schema, argument and boot-validation clause swaps are listed for the follow-up.
- `bash scripts/check-workspace-policy.sh` passes.
- No Rust or render path is touched, so cargo, clippy, fmt, doc, the realtime-policy tool and a no-bit-moved comparison do not apply.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `76ff20b54`.

**Red on base** (rows added before the fix; each row was run with the earlier new rows neutralised so it reached its own assertion):
- single-mode race: the dispose rejected with `{"tag":"miso.error.v1","requestId":1,"result":9}` (the control request's error), node `disconnectCount` 0, one live service tick restarted.
- sticky stands: `AssertionError [ERR_ASSERTION]: 255 !== 9` (`errorResult` after the `processorerror`).
- worker-mode race: an uncaught rejection `{ tag: 'miso.error.v1', requestId: 0, result: 9 }` from `await disposal`.
- own-reply guard row: its `rejected` assertions pass on base (base rejects every dispose on any failure); it is red on base at `a failed host runs no tick` (1 live interval: base's retry restarts the tick of a failed host, which then stops itself at its first fire without posting).
- processorerror guard row: green on base, as intended (it pins the existing behaviour the new survival rule must keep).

**Fix.** As in Model: one `#phase` (`HOST_LIVE`, `failed`, `disposing`, `disposed`) replaces `#stickyError`, `#disposed`, `#disposePromise`, `#workerGone` and `#disposePending()`; `#fail(error, source)` is the one failure transition; one `#inFlight` slot map with `inFlightSlot()` replaces the eight per-kind bound structures (`#pending` stays the correlation map); `#request`'s unused `allowSticky` parameter is gone (the dispose is sent only from `disposing`). `dispose()` creates its promise before it starts, so the `disposing` and `disposed` records carry it.

**Mutations** (each a `sed` of the fixed host, run through `MISO_ENGINE_WEB_HOST_TEST_MODULE`):
- reject a dispose whose reply is due (`if (false) this.#pending.set(...)`): RED, the single-mode race rejects with `{ tag: 'miso.error.v1', requestId: 1, result: 9 }`.
- keep the dispose only in single mode (`replyDue && this.#worker === undefined`): RED, the worker-mode row rejects with `{ tag: 'miso.error.v1', requestId: 0, result: 9 }`.
- a later failure replaces the first (`error,` for `error: prior?.error ?? error,`): RED, `255 !== 9`.
- keep the dispose on its own reply's failure (drop `&& source?.response !== "dispose"`): RED, `its own failed reply settles the dispose`.
- keep the dispose on a channel failure (drop `source !== FAIL_CHANNEL`): RED, `a processor error settles the dispose`.
- keep the dispose on the Worker's `error` (drop `source !== FAIL_WORKER_GONE`): RED, the existing `disposeTrap` row never settles (unsettled top-level await).
- restart the tick on any retry (drop `if (this.#phase === HOST_LIVE)`): RED, `a failed host runs no tick` (`1 !== 0`).
- drop `this.#inFlight.clear()` in `#fail`: GREEN, as expected. After a failure no bounded request can be sent again, so the stale counts cannot be observed; the clear keeps `#inFlight` in step with `#pending`.

**Gates run:**
- `node scripts/test-web-audioworklet.mjs`: passed. `bash scripts/test-web-audioworklet.sh`: exit 0, including the safe-integer allocator mutation and every policy self-test.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- SDK evals over the real module: `bash scripts/build-web-audioworklet.sh <dir>` (nightly-2026-08-20), then `MISO_ENGINE_SDK_ARTIFACTS_HEX=<dir> node --test 'test/*-evals.mjs' ../hosts/host-web/tests/stem-store-hash-v1.mjs` from `sdk/` (the `check-sdk-headless.sh` invocation): 710 tests, 710 pass. These evals drive the shipped host through `engine.close()` and the dispose paths.
- No Rust or render path was changed, so cargo test, clippy, fmt, doc, the realtime-policy tool and a no-bit-moved comparison do not apply.

**Test value.**
- single-mode race: red if a dispose already sent is rejected by a failure another message brought (a typed error for an earlier request), leaving it retryable, its node connected and its tick restarted while the worklet has already disposed; no existing row fails the host while a dispose is in flight.
- sticky stands: red if a later failure (`processorerror`) replaces the typed sticky error a request already rejected with; the S9 row checks only failures that arrive before the first.
- worker-mode race: red if a worker-mode dispose is rejected, and the Worker terminated, by a render failure the worklet posted while the Worker was disposing; the S8 rows fail the host before `dispose()`, never during it.
- own-reply and `processorerror` guard row: red if the survival rule keeps a dispose whose own reply was the failure, or one whose channel broke, waiting out its 5 s bound; and red if a retryable refusal of a failed host restarts the service tick.

### Attempt 1 verdict: FAIL, two MAJORs and two MINORs (2026-11-07)
The reviewer confirmed:
- the survival rule is correct in both modes;
- a dispose never hangs past its bound and never resolves without an OK ack;
- the slot map preserves every bound exactly, by reading;
- the scope is right.

The code was found correct. The failures are missing tests. The reviewer ran 63 mutations; these stayed green:
1. **MAJOR: the `#fail` source sites and the `workerGone` merge were untested.**
   - `messageerror` with `FAIL_CHANNEL` changed to `null`.
   - The dispose's own reply as a worker-mode teardown with the wrong reason, as a malformed `miso.error.v1`, and as a malformed ack, each with `pending` changed to `null`.
   - The fence post failure with `FAIL_CHANNEL` changed to `null`.
   - `workerGone` not kept from an earlier failure, or not added by a later one.
   - Rejecting with `failure.error` instead of `error`.
2. **MAJOR: the rewritten bounds were unpinned.**
   - The control, plan-latency and render-allocations slots set to `null`.
   - The control bound set to 65.
   - The three leases sharing one slot.
   - A seek report sharing the seek slot.
   - `#release` deleting the whole count instead of decrementing it.
3. **MINOR:** name the guards that need no test, with reasons.
4. **MINOR:** sync the issue body with the spec, including the `FAIL_CHANNEL`/`FAIL_WORKER_GONE` names.

### Attempt 2 (Terra, 2026-11-07)
The branch merged `origin/main` (`83a83e262`). The host module is unchanged from attempt 1; attempt 2 is tests and the spec.

**MAJOR 1: the failure-source table** (`scripts/test-web-audioworklet.mjs`, worker block, "what settles a dispose in flight at once"). The cases are listed under Gates. It replaces attempt 1's own-reply/`processorerror` guard row, which it covers.

**MAJOR 2: the bounds table** (single-mode block, "every bounded request kind, one table"). The plan-latency fake now answers a valid refusal, so a read can be settled. The table makes these older checks redundant, and they are deleted:
- the pipeline row's seek and session-seek overflows;
- the seek row's seek-at and seek-report overflows;
- `assertBoundedNoBurn` and its three lease/session-map calls.

The pipeline source overflow stays, because it also pins plane ownership on a local refusal.

**Mutations.** All 63 of the reviewer's mutants were re-run against the attempt-2 suite, plus their two follow-ups (`later_no_workergone`, `reject_with_sticky`):
- Every mutant in both MAJOR lists is now RED. The bounds table catches `slot_control_null`, `slot_planlatency_null`, `slot_renderalloc_null`, `bound_control_plus1`, `slot_lease_shared`, `slot_seekreport_shared_seek` and `release_delete_all`. The failure-source table catches the six source mutants and both `workerGone` mutants. `feedport_live` is RED at `no feed port while a dispose is in flight`.
- 11 stay GREEN. Each is named in the guard table below.

**Guard table.** It covers every guard in the lifecycle, failure, dispose, status-sharing and bound code of `MisoAudioWorkletHost`, and the early-failure handoff in `bootControlWorker`. Line numbers are in `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` at this attempt's head. Message-schema checks in `#receive` and the factory's argument validation are outside it: they are earlier schema gates, with their own rows. "Red" means that deleting the guard (or the reviewer's narrowing of it) turns the named row red.

| Line | Guard | Deleted or narrowed | Evidence |
|---|---|---|---|
| 472-478 | `inFlightSlot` arms | any arm `null`, merged or shared | Red: bounds table |
| 580 | tick only in `single` mode | always start | Red: S7 "worker mode holds no main-realm service tick" |
| 583 | early Worker `error` replayed | dropped | Red: S7 MINOR 1 |
| 584 | early `messageerror` replayed | dropped | Red: S7 MINOR 1, the new `messageerror` case |
| 585 | early `messageerror` source | `FAIL_CHANNEL` to `null` | Equivalent: no dispose exists during construction, and the source only decides whether a dispose survives |
| 592 | `#failWorker` once | guard deleted | Unreachable: `#failWorker` nulls `worker.onerror` before `#fail`, and the constructor's early-`error` replay runs once |
| 603-604 | `#failure()` in `failed`/`disposing` | `disposing` arm `null` | Red: S8 "the dispose tears down at once" |
| 619 | `#postServiceTick` only when `live` | narrowed to `disposed` | Equivalent: every exit from `live` (`dispose()`, `#fail`, `#teardown`) clears the timer synchronously, and a cleared interval never fires. Deleted outright: red, "a tick after dispose posts nothing" |
| 631 | `#stopServiceTick` with no timer | deleted | Equivalent: `clearInterval(null)` is a no-op |
| 651 | slot arm before the status arm | always the slot arm | Red: the suite never settles (a status follow-up is never sent) |
| 653 | decrement, not delete | delete the slot / no-op | Red: bounds table "and only one"; the pipeline row |
| 656 | status pending is the shared read | always | Equivalent: a status request is pending only while it is `#statusRead`. A read is kept only once posted, a follow-up is sent only after the read settles, and `#fail`/`#teardown` clear both together |
| 782 | late reply on a failed host ignored | narrowed to `failed`; deleted | Equivalent since this issue: the reply falls through to `#fail(255, null)`, where the first error stands, nothing but a surviving dispose is pending, and a `null` source keeps that dispose |
| 785 | teardown tag only in `worker` mode | either mode | Unreachable: the single-mode worklet never posts `miso.teardown.v1`, and both arms tear down |
| 791, 812, 814, 924 | the dispose's own reply as `source` | `pending` to `null` | Red: failure-source table (teardown with another reason, own typed error, malformed error, malformed ack) |
| 818 | unmatched reply fails a live host | deleted | Red: the new stray-reply row |
| 929, 944 | fence answer; hold for the fence | deleted | Red: the S9 rows |
| 932 | the control is still pending at its fence | deleted | Unreachable: a fence and its control leave `#pending` together (`#fail` and `#teardown` clear both, and only this branch releases the control) |
| 961 | fence ID exhausted | deleted | Unreachable in any session: it needs 2^53-1 request IDs. The shared allocator boundary is pinned at `#request` (1050) by the script's MAX_SAFE run. That run ends before the worker-mode block, so this site has no row |
| 962 | fence-exhaustion `source` | `control` to `null` | Equivalent: both sources keep the dispose |
| 973 | fence post failure `source` | `FAIL_CHANNEL` to `null` | Red: failure-source table "a fence it cannot post" |
| 989 | `#fail` once `disposed` | deleted | Unreachable: `#teardown` nulls every handler that calls `#fail` (ports, `onprocessorerror`, `worker.onerror`) |
| 994-995 | first error stands; `workerGone` merge | replace; not kept; not added | Red: sticky row; the Worker-gone table, both orders |
| 997 | failure kept inside `disposing` | phase set to `failed` | Red: single-mode race |
| 999-1000 | `replyDue` | any term dropped | Red: failure-source table; S8 `disposeTrap` |
| 1003 | `#inFlight.clear()` in `#fail` | deleted | Equivalent: once failed, no bounded request is sent again, so the counts are never read |
| 1008 | keep only a due dispose | keep all; keep the dispose always | Red: the suite never settles; own typed error case |
| 1009 | reject with this failure's error | `failure.error` | Red: failure-source table "a processorerror after it survived a typed failure" |
| 1019 | `#saturated` for a `null` slot | deleted | Equivalent: a `null` slot is never reserved, so its count is 0, under every bound |
| 1027 | `#reserve` skips a `null` slot | deleted | Equivalent: a count under the `null` key is never read (`#saturated` returns first, and `null`-slot requests release through the status arm) |
| 1041 | `disposed`, or the dispose/`disposing` XOR | `disposed` dropped; XOR narrowed | `disposed` dropped: red, because the suite never settles. XOR narrowed: equivalent, because only `#awaitDispose` sends a dispose, from `disposing` |
| 1044 | `failed` refuses with the sticky error | deleted | Red: sticky rows |
| 1046 | bound | deleted | Red: bounds table |
| 1050 | request ID exhausted | deleted | Red: the MAX_SAFE run of `test-web-audioworklet.sh` |
| 1205, 1234 | `#disposeBegan` | narrowed to `disposed`; to `disposing` | To `disposed`: red, because the suite never settles. To `disposing`: equivalent, because in `disposed` `#request` refuses with 3 too, and `#teardown` clears the follow-up |
| 1207, 1208, 1232 | shared-read join; follow-up | deleted | Red: Gate 3 rows |
| 1224 | read kept only when posted | always kept | Red: the new failed-post status row |
| 1254 | feed port only when `live` | narrowed to `disposed` | Red: failure-source table; the Worker-gone table |
| 1409, 1412, 1420-1422 | dispose promise; failure carried; retry arms | each mutated | Red: S7 "a second dispose is the first one's promise" (and the suite never settles); S8 "the dispose tears down at once"; S8 "and not restarted"; own typed error "no service tick runs" |
| 1434 | Worker gone disposes at once | deleted | Red: S8 broken-Worker row; Worker-gone table |
| 1446, 1463 | single-mode teardown; timeout | deleted | Red: S8 single-mode rows |
| 1474 | `#teardown` once | deleted | Unreachable: only the dispose runners call it, once per dispose (worker mode: `finally`; single mode: `catch` or success), always from `disposing` |
| 1484 | `#inFlight.clear()` in `#teardown` | deleted | Equivalent: nothing bounded is sent once `disposed` |
| 1486 | a fence is not rejected | deleted | Equivalent: a fence's `reject` is a no-op |
| 1487, 1505 | held replies after a clean dispose; terminate | deleted | Red: S9 clean dispose; S7 "the Worker ends after its dispose reply" |
| 1533 | boot `finish` once | deleted | Unreachable: the first `finish` replaces (on resolve) or nulls (on reject) every boot handler, and the post-failure `finish` runs before any handler can |
| 1535 | boot reject arm | deleted | Red: the boot rows |

**Gates run (attempt 2):**
- `node scripts/test-web-audioworklet.mjs`: passed.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Test value (attempt 2).**
- failure-source table: red if a `#fail` call site names the wrong source, so a dispose waits out its 5 s bound for a reply that already came or that a broken channel cannot bring; or if a dispose that survived a typed failure rejects with that failure, not the one that ended it, and stays retryable.
- Worker-gone table: red if `workerGone` is lost by a later failure, or never recorded when the Worker's error comes second; the dispose then posts to a terminated Worker. Also red if a failed host opens a feed port.
- bounds table: red if a kind is unbounded, a bound is off by one, two kinds or two keys share a slot, a refusal posts or burns an ID, or settling a request frees more than one place. The deleted single-kind checks each tested one cell of it.
- early `messageerror` case: red if a `messageerror` between the boot reply and the host's construction is dropped, so the host starts live; MINOR 1 covered only `error`.
- failed-post status row: red if a status read that failed to post is kept as the shared read, so the next call returns its rejection and posts nothing.
- stray-reply row: red if a reply that matches no request is ignored on a live host, or crashes the handler.

### Attempt 2 verdict: FAIL, one MAJOR and two MINORs (2026-11-07)
The reviewer upheld all 11 equivalence claims of attempt 2's guard table and confirmed that the deleted checks are superseded, with no coverage lost. Findings:
1. **MAJOR: five `#fail` sites with a `null` source were unpinned and missing from the table.** The sites are the malformed meter frame (709), telemetry frame (727), control event (744), watermark (765), and the reply matching no request (819). Swapping any of them to `FAIL_CHANNEL` stayed green, yet it would let an unsolicited frame settle an in-flight dispose with 255, which is this issue's own defect.
2. **MINOR:** the early-failure precedence in `bootControlWorker` (1544 and 1546) was unpinned.
3. **MINOR:** line 961, fence request-ID exhaustion, should get its row now.

### Attempt 3 (Terra, 2026-11-07)
The host module is unchanged; attempt 3 is tests and the spec.
- **MAJOR:** the failure-source table gained its "does not settle" half (the reviewer's repro, folded in). For each of the five frames, a single-mode host with the dispose held gets the frame. The dispose stays `pending`, later requests are refused with 3, and releasing the held reply resolves the dispose and tears down.
- **MINOR 2:** the S7 MINOR 1 row loops over `error`, `messageerror`, and both orders of the two. It asserts that the Worker is terminated exactly when it raised `error`, and that the dispose then tears down at once and sends nothing (`workerGone`).
- **MINOR 3:** a MAX_SAFE row for the fence's missing request ID (see Gates).
- **Enumeration:** `if` deletions surfaced three more guards in the class that no row pinned, so each got a row:
  - a malformed telemetry frame (716) fails the host;
  - a malformed control event (738) fails the host;
  - `seekReport` with a non-string source ID (1145) is refused locally.

**Mutations.**
- All 11 reviewer mutants in `review-2279b/mut-out3` are RED: the five frame-source swaps, both early-precedence mutants, `render0_src_chan`, `fence_held_noset`, `post_fail_no_release` and `teardown_clean_false`.
- A mechanical enumeration of the whole host gave 139 mutants: every `if (` (condition set to `false`, matched across lines) and every `this.#fail(` (source swapped: `pending`/`control` to `null`, `null` to `FAIL_CHANNEL`, `FAIL_CHANNEL` to `null`, `FAIL_WORKER_GONE` to `FAIL_CHANNEL`).
- Each mutant ran through the suite, and then, if still green, through the MAX_SAFE run.
- Result: 101 RED, 3 RED only in the MAX_SAFE run, and 35 GREEN. Of the green, 12 are in the host's lifecycle and bound code, each equivalent or unreachable (below); the other 23 lie outside it (below that).
- Assignments and ternaries are not `if` guards. They are covered by the reviewer's earlier mutants, all RED:
  - `phase_drop_disposing`, `dispose_failure_null`, `retry_always_live`, `impl_later_replaces`, `workergone_not_sticky`, `later_no_workergone`, `reject_with_sticky`, and the bound and slot mutants;
  - the boot precedence assignments at 1544 and 1546: `early_err_keep_first` and `early_msgerr_overwrite`.

**Guard table (attempt 3, mechanical).** It lists every `if` and `#fail` site from `inFlightSlot` to the end of `bootControlWorker` (lines 466-1581 of the host at this head) with its mutant's verdict. It replaces attempt 2's hand-written table. "Red row" is the first failing assertion.

| Line | Mutant | Verdict | Red row, or why it is equivalent |
|---|---|---|---|
| 472 | condition `false` | RED | the suite never settles (a request is left pending) |
| 475 | condition `false` | RED | meters: refused past its bound of 1 |
| 476 | condition `false` | RED | session seek: refused past its bound of 1 |
| 571 | condition `false` | RED | the suite never settles (a request is left pending) |
| 580 | condition `false` | RED | the tick runs while the host does |
| 583 | condition `false` | RED | expected rejection |
| 584 | condition `false` | RED | expected rejection |
| 592 | condition `false` | GREEN | Unreachable: `#failWorker` nulls `worker.onerror` before it calls `#fail`, and the constructor replays an early `error` once. |
| 603 | condition `false` | RED | an equality assertion |
| 604 | condition `false` | RED | the dispose tears down at once |
| 619 | condition `false` | RED | a tick after dispose posts nothing |
| 631 | condition `false` | GREEN | Equivalent: `clearInterval(null)` is a no-op. |
| 644 | condition `false` | RED | MAX_SAFE run: an equality assertion |
| 651 | condition `false` | RED | an equality assertion |
| 653 | condition `false` | GREEN | Equivalent: a count of 0 is stored instead of deleted; every read is `?? 0`, and a slot is released only after it was reserved. |
| 655 | condition `false` | RED | the suite never settles (a request is left pending) |
| 656 | condition `false` | RED | the suite never settles (a request is left pending) |
| 668 | condition `false` | RED | an equality assertion |
| 677 | condition `false` | RED | expected rejection |
| 715 | condition `false` | RED | an equality assertion |
| 716 | condition `false` | RED | expected rejection |
| 737 | condition `false` | RED | each frame with no listener is counted |
| 738 | condition `false` | RED | expected rejection |
| 747 | condition `false` | RED | each frame with no listener is counted |
| 758 | condition `false` | RED | the listener gets the posted words |
| 762 | condition `false` | RED | expected rejection |
| 782 | condition `false` | GREEN | Equivalent since this issue: the reply falls through to `#fail(255, null)`. The first error stands, nothing but a surviving dispose is pending, and a `null` source keeps that dispose. |
| 785 | condition `false` | RED | an equality assertion |
| 786 | condition `false` | RED | an equality assertion |
| 800 | condition `false` | RED | an equality assertion |
| 806 | condition `false` | RED | an equality assertion |
| 810 | condition `false` | RED | an equality assertion |
| 818 | condition `false` | RED | TypeError: Cannot read properties of null (reading 'response') |
| 920 | condition `false` | RED | expected rejection |
| 929 | condition `false` | RED | the suite never settles (a request is left pending) |
| 932 | condition `false` | GREEN | Unreachable: a fence and its control leave `#pending` together (`#fail` and `#teardown` clear both, and only this branch releases the control). |
| 944 | condition `false` | RED | the Worker's reply waits for the fence |
| 949 | condition `false` | RED | expected rejection |
| 961 | condition `false` | RED | MAX_SAFE run: with the control request's ID |
| 989 | condition `false` | GREEN | Unreachable: `#teardown` nulls every handler that calls `#fail` (ports, `onprocessorerror`, `worker.onerror`). |
| 1008 | condition `false` | RED | { tag: 'miso.error.v1', requestId: 1, result: 9 } |
| 1019 | condition `false` | GREEN | Equivalent: a `null` slot is never reserved, so its count is 0, under every bound. |
| 1027 | condition `false` | RED | the suite never settles (a request is left pending) |
| 1041 | condition `false` | RED | the suite never settles (a request is left pending) |
| 1044 | condition `false` | RED | expected rejection |
| 1046 | condition `false` | RED | the suite never settles (a request is left pending) |
| 1050 | condition `false` | RED | MAX_SAFE run: an equality assertion |
| 1078 | condition `false` | RED | expected rejection |
| 1105 | condition `false` | RED | expected rejection |
| 1126 | condition `false` | RED | expected rejection |
| 1145 | condition `false` | RED | expected rejection |
| 1178 | condition `false` | RED | expected rejection |
| 1205 | condition `false` | RED | the suite never settles (a request is left pending) |
| 1207 | condition `false` | RED | three calls post one read |
| 1208 | condition `false` | RED | one shared reply |
| 1224 | condition `false` | RED | three calls post one read |
| 1232 | condition `false` | RED | TypeError: Cannot read properties of null (reading 'resolve') |
| 1254 | condition `false` | RED | no feed port once disposed |
| 1293 | condition `false` | RED | expected rejection |
| 1301 | condition `false` | RED | TypeError: Cannot read properties of undefined (reading 'byteLength') |
| 1328 | condition `false` | RED | expected rejection |
| 1345 | condition `false` | RED | expected rejection |
| 1367 | condition `false` | RED | Missing expected exception. |
| 1378 | condition `false` | RED | Missing expected exception. |
| 1409 | condition `false` | RED | the suite never settles (a request is left pending) |
| 1420 | condition `false` | RED | a failed dispose leaves exactly one live service tick |
| 1422 | condition `false` | RED | a failed dispose leaves exactly one live service tick |
| 1434 | condition `false` | RED | the dispose tears down at once |
| 1446 | condition `false` | RED | the node is disconnected |
| 1463 | condition `false` | RED | Missing expected rejection: no reply on a closed context tears down with its reason |
| 1474 | condition `false` | GREEN | Unreachable: only the dispose runners call `#teardown`, once per dispose (worker mode: `finally`; single mode: `catch` or the success path), always from `disposing`. |
| 1486 | condition `false` | GREEN | Equivalent: a fence's `reject` is a no-op. |
| 1487 | condition `false` | RED | { tag: 'miso.error.v1', requestId: 0, result: 3 } |
| 1505 | condition `false` | RED | the Worker ends after its dispose reply |
| 1533 | condition `false` | GREEN | Unreachable: the first `finish` replaces (on resolve) or nulls (on reject) every boot handler, and the post-failure `finish` runs before any handler can. |
| 1535 | condition `false` | RED | an equality assertion |
| 1552 | condition `false` | RED | { tag: 'miso.error.v1', requestId: 0, result: 255 } |
| 1560 | condition `false` | RED | The validation function is expected to return "true". Received false |
| 569 | source FAIL_CHANNEL->null | RED | single: a messageerror settles the dispose at once |
| 578 | source FAIL_CHANNEL->null | RED | single: a processorerror settles the dispose at once |
| 585 | source FAIL_CHANNEL->null | GREEN | Equivalent: no dispose exists during construction, and the source decides only whether a dispose survives. |
| 597 | source FAIL_WORKER_GONE->FAIL_CHANNEL | RED | the dispose tears down at once |
| 709 | source null->FAIL_CHANNEL | RED | a malformed meter frame leaves the dispose to its reply |
| 727 | source null->FAIL_CHANNEL | RED | a malformed telemetry frame leaves the dispose to its reply |
| 744 | source null->FAIL_CHANNEL | RED | a malformed control event leaves the dispose to its reply |
| 765 | source null->FAIL_CHANNEL | RED | a malformed watermark leaves the dispose to its reply |
| 791 | source pending->null | RED | worker: its own teardown with another reason settles the dispose at once |
| 803 | source null->FAIL_CHANNEL | RED | { tag: 'miso.error.v1', requestId: 0, result: 9 } |
| 812 | source pending->null | RED | single: its own typed error settles the dispose at once |
| 814 | source pending->null | RED | single: its own malformed error settles the dispose at once |
| 819 | source null->FAIL_CHANNEL | RED | a reply that matches no request leaves the dispose to its reply |
| 924 | source pending->null | RED | single: its own malformed ack settles the dispose at once |
| 962 | source control->null | GREEN | Equivalent: `control` and `null` both keep the dispose. |
| 973 | source FAIL_CHANNEL->null | RED | worker: a fence it cannot post settles the dispose at once |

**Outside this issue: guards with no row in the factory-input, plane and boot validation.** These are earlier, separate gates that this branch does not touch: the factory's record snapshot, the returned-plane check and the factory's boot. Their deletion stays green. They are listed here so the gap is on record, not fixed under this issue's one product outcome. Proposed follow-up: "Web host: factory-input, returned-plane and boot-reply validation guards have no rows".

| Line | Mutant | Note |
|---|---|---|
| 140 | condition `false` | `hasExactFields` on `null` or a primitive. Deleted, `Object.keys(null)` throws inside the caller instead of returning false; no row posts a non-object message or option. |
| 147 | condition `false` | `capturePlainRecord`: a non-object or array factory record. No row passes one. |
| 149 | condition `false` | `capturePlainRecord`: a record with a foreign prototype. No row passes one. |
| 158 | condition `false` | `capturePlainRecord`: an accessor property. No row passes one. |
| 168 | condition `false` | `hasEnumerableShape`: an extra enumerable boot-option field. No row passes one. |
| 175 | condition `false` | `hasRequiredEnumerableFields`: a missing or non-enumerable required field. No row passes one. |
| 181 | condition `false` | `snapshotDocument` on a non-object. No row passes one. |
| 183 | condition `false` | `snapshotDocument` on a non-`Uint8Array` typed array. No row passes one. |
| 189 | condition `false` | `snapshotDocument`: a view whose length disagrees with its backing. No row builds one. |
| 201 | condition `false` | `snapshotBootOptions`: a malformed options record. No row passes one. |
| 223 | condition `false` | `snapshotFactory`: a record missing a required field. No row passes one. |
| 230 | condition `false` | `snapshotFactory`: a non-enumerable optional field. No row passes one. |
| 257 | condition `false` | Equivalent: no caller of `webError` passes `planes`. |
| 308 | condition `false` | `planeShape` grouping: with every group `undefined`, `validReturnedPlanes` still compares each plane with group `undefined`. Aliasing is untested (see 329, 331). |
| 319 | condition `false` | `validReturnedPlanes`: a reply with a different plane count. No row returns one. |
| 329 | condition `false` | `validReturnedPlanes`: planes of one buffer returned split. No row returns them. |
| 331 | condition `false` | `validReturnedPlanes`: planes of two buffers returned aliased. No row returns them. |
| 348 | condition `false` | `fetchModule`: a non-OK fetch. No row answers one. |
| 1597 | condition `false` | The factory refuses a record its snapshot rejected. It is reached only through the snapshot guards above. |
| 1674 | condition `false` | The worker-mode boot refuses a Worker ready reply with the wrong backend, memory, ring, frame or resources. No row answers one. |
| 1699 | condition `false` | The worker-mode boot hands the worklet its control port. The fake worklet ignores the port, so only the browser qualification exercises it. |
| 1706 | condition `false` | Unreachable: the boot `finish` nulls its handlers on first entry. |
| 1749 | condition `false` | A single-mode typed boot refusal is kept typed instead of becoming 255. No single-mode row refuses a boot with a typed error (the worker-mode one does). |

**Gates run (attempt 3):**
- `node scripts/test-web-audioworklet.mjs`: passed.
- The MAX_SAFE run (as `test-web-audioworklet.sh` does it): passed.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Test value (attempt 3).**
- the "does not settle" half: red if a failure that an unsolicited frame or an unmatched reply brought names the channel as its source, so it rejects an in-flight dispose with 255 though the worklet disposed. That is this issue's defect, at the five sites the first half could not see.
- early-failure order row: red if an early `messageerror` overwrites an earlier `error`, or an `error` does not overwrite an earlier `messageerror`. The host then keeps a crashed Worker as live, never terminates it, and posts its dispose to it.
- MAX_SAFE fence row: red if a held control reply whose fence has no request ID posts a fence with no ID, or is left pending for ever.
- telemetry and control-event schema row: red if either schema check is dropped, so a broken frame reaches the app or is counted as discarded while the host keeps answering.
- `seekReport` argument check: red if a non-string source ID is posted to the worklet.

### Attempt 3 verdict: FAIL, two MAJORs and three MINORs (2026-11-07)
The host module was found correct; the gaps were in the tests. The reviewer's generator (794-796 mutants over lines 466-1581) swaps each `#fail` source to every alternative, not just one. It also covers:
- `if` set to true and to false;
- `===`/`!==` and `&&`/`||`;
- `??` to `||`;
- the boundary operators;
- `true` to `false`;
- `resolve` to `reject`;
- deleting a `return`;
- deleting a `.delete`, `.set`, `.clear` or `.add` call.

Findings:
1. **MAJOR: another request's malformed reply settles the in-flight dispose.** The sites are 791, 814 and 924, with `pending` swapped to `FAIL_CHANNEL` or to `FAIL_WORKER_GONE`.
2. **MAJOR: `FAIL_CHANNEL` to `FAIL_WORKER_GONE` was unpinned** at 569, 578, 585 and 973. A worker-mode dispose after a channel failure then skips the live Worker.
3. **MINOR: the claimed equivalence at 932 is refuted.** A duplicate control reply plus a second fence frees a place in another control's bound.
4. **MINOR:** `this.#pending.clear()` in `#fail` (1002) was unpinned.
5. **MINOR:** pre-existing gaps in this issue's dispose and teardown code:
   - `clean` (1430) and the follow-up's rejection at teardown (1493);
   - the `#oldestRequestId` returns (639/640);
   - the `return`s after a failing `#fail` (793, 804, 816, 925).

All other equivalences were upheld. The attempt-3 table's entries for 932 (now RED, not unreachable) and for 585's `FAIL_WORKER_GONE` swap (now RED) are superseded by the attempt-4 tables below.

### Attempt 4 (Terra, 2026-11-07)
The branch merged `origin/main` (`cbab8eb3f`).
- **MAJOR 1:** the "does not settle" half gained the reviewer's three repro cases.
- **MAJOR 2:** the channel-failure row runs four failures in worker mode (a worklet `processorerror`, a worklet `messageerror`, a Worker `messageerror`, a fence that cannot be posted). In each, `dispose()` posts exactly one `miso.dispose.v1` to the live Worker and resolves on its reply. The early-failure row asserts the same for an early `messageerror` alone, which corrects 585's row.
- **MINOR 3:** the duplicate-reply row is added and the 932 equivalence is withdrawn. Follow-up: fail the host on a duplicate reply (today the second reply is held for a second fence and absorbed).
- **MINOR 4:** the late-reply-on-a-failed-host row (no fence).
- **MINOR 5:** rows for the unclean dispose's held reply (1430) and the follow-up at teardown (1493). For the remaining sites:
  - 639: a row (the oldest unsettled ID).
  - 640: named equivalent (`webError` defaults the ID to 0).
  - 793: a row (the release-timeout teardown).
  - 925: a row (a malformed control reply posts no fence).
  - 804 and 816: named equivalent (below).
- The generator's re-run found more pinnable greens, now rows:
  - the dispose's own typed error in worker mode (800, `&&` to `||`);
  - the `return` after a malformed meter, telemetry or watermark frame (710, 728, 766), which handed the frame to its listener;
  - a refused fence (924, the `||` to `&&` swap that folds `!validFence` into the plan-latency check).
- **One host change.** At 820, the stray-message failure took `message?.requestId ?? 0`. Its `??` mutants were not equivalent: a missing ID could become 1, and an ID like `"x"` was echoed into the sticky record. The host now uses `validRequestId(message?.requestId) ? message.requestId : 0`, and a row pins ID 0 for both shapes.

**The generator** (the reviewer's `tools/gen.py` and `tools/run2.py`), summarised so it can be re-run:
- `gen.py <repo> <out>` reads the host and, for each code line in 466-1581, writes one mutant per occurrence of each swap: `===`/`!==`, `&&`/`||`, `??`/`||`, `>=`/`>`, `<=`/`<`, ` > `/` >= `, ` < `/` <= `, `true`/`false`, `.resolve`/`.reject`, `resolve,`/`reject,`, `outcome.reject`/`outcome.resolve`, ` + 1`/` + 0`, ` - 1`/` - 0`, `?? 1`/`?? 0`, `?? 0`/`?? 1`.
- It also writes:
  - one mutant per line that deletes its first `return ...;`;
  - one per line that blanks a statement calling `.delete(`, `.set(`, `.clear()` or `.add(`;
  - two per `if (`, with the condition, paren-matched across lines, set to `false` and to `true`;
  - one per alternative `#fail` source (`null`, `FAIL_CHANNEL`, `FAIL_WORKER_GONE`, `pending`, each other than the current one).
- `run2.py <repo> <index.json> 12` runs each mutant through `scripts/test-web-audioworklet.mjs` via `MISO_ENGINE_WEB_HOST_TEST_MODULE`, 12 at a time. A mutant still green is re-run in the MAX_SAFE mode (`#lastRequestId` starting at `MAX_SAFE_INTEGER - 1`, `MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST=1`).

**Final counts on this head** (794 mutants): 633 RED, 9 RED only in the MAX_SAFE run, 152 GREEN.
- 45 of the green mutants are in the lifecycle, failure, dispose, status-sharing and bound code. Each is named below with its equivalence.
- 107 are schema, argument or boot-validation clause swaps, listed by family below for the follow-up.

**Green mutants in the lifecycle, failure, dispose, status-sharing and bound code** (each equivalent or unreachable; line numbers at this head):

| Line | Mutant | Why it is equivalent |
|---|---|---|
| 566 | `L566_qq_0` ('??'->'||' #0) | `worker` is a Worker or `undefined` and cannot be another falsy value, so `||` and `??` agree. |
| 582 | `L582_qq_0` ('??'->'||' #0) | `early.messages` is an array (truthy even when empty) or `early` is `undefined`; it cannot be another falsy value, so `||` and `??` agree. |
| 585 | `L585_failsrc_null` (#fail source FAIL_CHANNEL->null) | No dispose exists during construction, and the source only decides whether a dispose survives. (Its `FAIL_WORKER_GONE` swap is RED in the early-failure row.) |
| 592 | `L592_iffalse` (if condition -> false) | Unreachable, as for the deleted `return`. |
| 592 | `L592_noreturn` (delete 'return;') | Unreachable: `#failWorker` nulls `worker.onerror` before it calls `#fail`, and the constructor replays an early `error` once. |
| 631 | `L631_iffalse` (if condition -> false) | `clearInterval(null)` is a no-op. |
| 640 | `L640_noreturn` (delete 'return 0;') | `webError`'s `requestId` parameter defaults to 0, so a returned `undefined` becomes 0. |
| 652 | `L652_qq1_0` ('?? 1'->'?? 0' #0) | The default is read only for a slot with no count, and a slot is released only after it was reserved. |
| 652 | `L652_qq_0` ('??'->'||' #0) | A reserved slot's count is a positive number, and it cannot be another falsy value, so `||` and `??` agree. |
| 653 | `L653_iffalse` (if condition -> false) | The same as `<=` to `<`: a count of 0 is stored instead of deleted. |
| 653 | `L653_lteq_0` ('<='->'<' #0) | A count of 0 is stored instead of deleted. Every read is `?? 0`, and the `?? 1` read happens only on a reserved slot. |
| 655 | `L655_iftrue` (if condition -> true) | Every pending with a `null` slot that is not a status read (a dispose or a fence) fails the inner `#statusRead` ID check, so nothing runs. |
| 656 | `L656_iftrue` (if condition -> true) | A status request is pending only while it is `#statusRead`: a read is kept only once posted, a follow-up is sent only after the read settles, and `#fail`/`#teardown` clear both together. |
| 749 | `L749_noreturn` (delete 'return;') | It falls into the listener loop over an empty set. |
| 779 | `L779_qq_0` ('??'->'||' #0) | `Map.get` returns a pending record or `undefined`; it cannot be another falsy value, so `||` and `??` agree. |
| 782 | `L782_iffalse` (if condition -> false) | The reply falls through to `#fail(255, null)`. The first error stands, nothing but a surviving dispose is pending, and a `null` source keeps it. |
| 791 | `L791_qq_0` ('??'->'||' #0) | Request IDs are positive integers; an ID cannot be another falsy value, so `||` and `??` agree. |
| 803 | `L803_failsrc_pending` (#fail source null->pending) | `pending` is `#pending.get(0)`, always `null`, because request IDs start at 1. |
| 804 | `L804_noreturn` (delete 'return;') | It falls to the error path, where request ID 0 fails `validRequestId` and calls `#fail(255, null)` again. The first error stands, and a `null` source keeps the same dispose. |
| 814 | `L814_qq_0` ('??'->'||' #0) | Request IDs are positive integers; an ID cannot be another falsy value, so `||` and `??` agree. |
| 816 | `L816_noreturn` (delete 'return;') | It falls to the `pending === null` or shape-check `#fail`, a second call with the same source as the first (`null` or the same pending). The first error stands, and the second call finds the `#pending` the first one left. |
| 820 | `L820_failsrc_pending` (#fail source null->pending) | It is inside `if (pending === null)`, so `pending` is `null`. |
| 936 | `L936_noreturn` (delete 'return;') | It falls through with the fence record. Its second `#release` is a no-op (a `null` slot, not a status read) and its `resolve` is a no-op. |
| 963 | `L963_failsrc_FAIL_CHANNEL` (#fail source control->FAIL_CHANNEL) | **Withdrawn after the attempt-4 review: not equivalent.** A dispose sent after the control and before its reply takes the last ID, so the fence finds none while the dispose is in flight. RED since the fold, in the `MAX_SAFE - 2` run ("the dispose waits for its own reply"). |
| 963 | `L963_failsrc_null` (#fail source control->null) | Equivalent: `#fail` treats a `null` source and a non-dispose pending source alike, since both leave `replyDue` true. |
| 990 | `L990_iffalse` (if condition -> false) | Unreachable: `#teardown` nulls every handler that calls `#fail` (ports, `onprocessorerror`, `worker.onerror`). |
| 995 | `L995_qq_0` ('??'->'||' #0) | `prior.error` is a frozen error record and cannot be another falsy value, so `||` and `??` agree. |
| 1004 | `L1004_dropstmtclear` (blank line 'this.#inFlight.clear();') | Once failed, no bounded request is sent again, so the counts are never read. |
| 1020 | `L1020_iffalse` (if condition -> false) | A `null` slot is never reserved, so its count is 0, under every bound. |
| 1024 | `L1024_qq_0` ('??'->'||' #0) | A count is a number of at least 0, or absent; `0 || 0` is 0. |
| 1028 | `L1028_iftrue` (if condition -> true) | A count under the `null` key is never read: `#saturated` returns first, and `null`-slot requests release through the status arm. |
| 1028 | `L1028_qq_0` ('??'->'||' #0) | A count is a number of at least 0, or absent; `0 || 0` is 0. |
| 1210 | `L1210_qq_0` ('??'->'||' #0) | `#statusFollow` is `null` or a deferred and cannot be another falsy value, so `||` and `??` agree. |
| 1422 | `L1422_qq_0` ('??'->'||' #0) | `failure` is `null` or a failed record and cannot be another falsy value, so `||` and `??` agree. |
| 1474 | `L1474_false_0` ('false'->'true' #0) | Only the single-mode runner calls `#teardown()` with no argument, and single mode never holds a control reply (fences are worker mode only), so `clean` decides nothing there. |
| 1475 | `L1475_iffalse` (if condition -> false) | Unreachable: only the dispose runners call `#teardown`, once per dispose, always from `disposing`. |
| 1484 | `L1484_dropstmtclear` (blank line 'this.#pending.clear();') | Once disposed, nothing reads `#pending`: every port handler is nulled, `#request` refuses before it stamps, and `#oldestRequestId` is reached only through `#fail`, which is unreachable. |
| 1485 | `L1485_dropstmtclear` (blank line 'this.#inFlight.clear();') | Nothing bounded is sent once disposed. |
| 1487 | `L1487_iffalse` (if condition -> false) | A fence's `reject` is a no-op. |
| 1497 | `L1497_dropstmtclear` (blank line 'this.#controlEventListeners.clear();') | Listeners are called only from `#receive`, which no port can reach once `#teardown` nulls the handlers. |
| 1498 | `L1498_dropstmtclear` (blank line 'this.#watermarkListeners.clear();') | Listeners are called only from `#receive`, which no port can reach once `#teardown` nulls the handlers. |
| 1534 | `L1534_iffalse` (if condition -> false) | Unreachable, as for the deleted `return`. |
| 1534 | `L1534_noreturn` (delete 'return;') | Unreachable: the first `finish` replaces (on resolve) or nulls (on reject) every boot handler, and the synchronous post-failure `finish` runs before any handler can. |
| 1535 | `L1535_true_0` ('true'->'false' #0) | Unreachable, as for the deleted `return`: `settled` is read only by a second `finish`, which never runs. |
| 1547 | `L1547_qq_0` ('??'->'||' #0) | `early.failed` is `null` or a non-empty string and cannot be another falsy value, so `||` and `??` agree. |

**Green clause swaps outside this issue** (pre-existing, unchanged by this branch). They belong to the follow-up proposed in attempt 3, "Web host: factory-input, returned-plane and boot-reply validation guards have no rows", widened to cover the frame and reply schemas and the request arguments:

| Lines | Clause family | Green mutants |
|---|---|---|
| 550-557 | the public host properties' `enumerable` flags (API shape) | 6 |
| 682-708 | meter-frame schema clauses | 18 |
| 720-726 | telemetry-frame schema clauses | 10 |
| 741-743 | control-event schema clauses | 4 |
| 764 | watermark schema clause | 1 |
| 874-883 | session-map and status reply schema clauses | 11 |
| 916-919 | plan-latency reply schema clauses | 7 |
| 1080-1091 | `submitSource` argument clauses | 19 |
| 1107-1109 | `seekSource` argument clauses | 5 |
| 1126-1128 | `seekSourceAt` argument clauses | 4 |
| 1179-1181 | `seekSession` argument clauses | 4 |
| 1331 | `meters` argument clause | 1 |
| 1348 | `telemetry` argument clause | 1 |
| 1525 | the Worker constructor's failure at boot | 2 |
| 1553-1563 | the control Worker's boot-reply checks | 14 |

**Gates run (attempt 4):**
- `node scripts/test-web-audioworklet.mjs`: passed.
- The MAX_SAFE run: passed.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Follow-ups (not filed):**
- Fail the host on a duplicate reply.
- Give rows to the frame/reply schema, request-argument, factory-input, returned-plane and boot-reply validation clauses listed above.

**Test value (attempt 4).**
- another request's malformed reply or teardown: red if a reply to an earlier request names the channel or the Worker as its source, so it rejects a dispose the peer still answers.
- worker-mode channel failures: red if a channel failure is taken for a gone Worker, so the dispose skips the Worker that holds the control half.
- duplicate reply: red if the second fence releases the control again, so one settled call frees two places of the bound.
- unclean dispose: red if held replies resolve after a teardown that did not prove their transactions committed.
- follow-up at teardown: red if a waiting status follow-up resolves with nothing, or is left pending.
- late reply on a failed host: red if `#fail` leaves its rejected requests in `#pending`, so a late reply is held for a fence.
- release-timeout: red if the teardown reply is also read as a malformed reply and fails the overtaken request with 255.
- worker-mode own typed error: red if a typed error to the dispose is read as a render failure, with a source that keeps the dispose waiting.
- malformed worker-mode control reply: red if it is still held for a fence on the failed host.
- refused fence: red if the fence's result check is dropped, so a refused fence releases the held reply.
- malformed frame and listener: red if the frame reaches its listener after the host failed on it.
- stray and oldest IDs: red if the sticky error echoes a non-ID or turns a missing ID into 1, or if the oldest-request lookup answers 0 with requests unsettled.

### Attempt 4 verdict: PASS, with two MINORs and a nit folded (2026-11-07)
The reviewer confirmed the generator counts exactly: 633 RED, 9 RED only in the MAX_SAFE run, 152 GREEN. They upheld 44 of the 45 equivalences and confirmed that all 107 deferred mutants are on lines that already exist on `main`. Folded on the same branch:
1. **The `L963_failsrc_FAIL_CHANNEL` equivalence is refuted.** A dispose sent after a control request and before its reply takes the last safe ID, so the control reply's fence finds none while the dispose is in flight. The mutant then rejects the dispose with 255 instead of letting it settle on its own ack.
   - The reviewer's row is added to the MAX_SAFE block, under `MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST=2`.
   - `scripts/test-web-audioworklet.sh` runs it on a second safe-start copy that starts at `MAX_SAFE_INTEGER - 2`.
   - Both 963 rows above are corrected. The `null` swap stays equivalent because a `null` source and a non-dispose pending source both leave `replyDue` true.
   - Check: the reviewer's `L963fc` mutant is RED in that run, and head and the `null` mutant stay GREEN.
2. **Line 820 was pinned only for a missing ID and a string ID.** The stray loop now also sends acks with `requestId: -1` and `requestId: 1.5`. The reviewer's six line-820 mutants, including the `Number.isSafeInteger` and `typeof === "number"` substitutions, are all RED.
3. **Nit:** the duplicate-reply row now asserts result 6 (backpressure) for the 65th call, not just a rejection.

Gates after the fold:
- `bash scripts/test-web-audioworklet.sh`: exit 0, including both MAX_SAFE runs.
- `bash scripts/check-workspace-policy.sh`: ok.
