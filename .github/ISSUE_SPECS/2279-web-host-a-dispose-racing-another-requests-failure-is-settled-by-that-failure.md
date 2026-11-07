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
- No render-path, worklet or Rust change. Public behaviour changes only in the two races above.

## Gates
- `scripts/test-web-audioworklet.mjs` rows, red on base:
  - single mode: a dispose racing a control request's typed failure resolves with its own reply, disconnects the node and restarts no tick;
  - single mode: after a typed failure, a `processorerror` leaves the typed error standing;
  - worker mode: a dispose racing a worklet render failure resolves cleanly once the Worker answers.
- A guard row for the survival rule's boundary (single mode): a dispose whose own reply is a `miso.error.v1` rejects at once with it, and a retryable refusal of a now-failed host restarts no tick (red on base at the tick); a `processorerror` while the dispose is in flight rejects it at once with 255 and tears down (green on base).
- Every existing row of `bash scripts/test-web-audioworklet.sh` stays green, including the safe-integer allocator mutation it runs.
- Mutations of the fix, each red at its row: make `#fail` reject a dispose whose reply is still due; let a later `#fail` replace the first error; keep the dispose on its own reply's failure, on a channel failure, or on the Worker's `error`; restart the tick on a retry into `failed`.
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
- No Rust or render path was changed, so cargo test, clippy, fmt, doc, the realtime-policy tool and a no-bit-moved comparison do not apply.

**Test value.**
- single-mode race: red if a dispose already sent is rejected by a failure another message brought (a typed error for an earlier request), leaving it retryable, its node connected and its tick restarted while the worklet has already disposed; no existing row fails the host while a dispose is in flight.
- sticky stands: red if a later failure (`processorerror`) replaces the typed sticky error a request already rejected with; the S9 row checks only failures that arrive before the first.
- worker-mode race: red if a worker-mode dispose is rejected, and the Worker terminated, by a render failure the worklet posted while the Worker was disposing; the S8 rows fail the host before `dispose()`, never during it.
- own-reply and `processorerror` guard row: red if the survival rule keeps a dispose whose own reply was the failure, or one whose channel broke, waiting out its 5 s bound; and red if a retryable refusal of a failed host restarts the service tick.
