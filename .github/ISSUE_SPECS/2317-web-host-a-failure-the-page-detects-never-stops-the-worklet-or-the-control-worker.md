# Web host: a failure the page detects never stops the worklet or the control Worker

Child of #2258 (implicit state machines), item WEB-2: the page-side follow-up that #2267's spec names, with its reverse direction. Re-verified at base `5d403acd8`.

## Problem
The browser host is three parties: the main-realm host (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`, class `MisoAudioWorkletHost`), the worklet (`miso-engine-v1-audio-worklet.js`) and, in `worker` mode, the control Worker (`miso-engine-v1-control-worker.js`). #2279 made the host's lifecycle one `#phase` with one failure transition, `#fail(error, source)` (line 988). Its first entry into `failed` (`prior === null`, line 991) is where #2279's spec says this follow-up attaches.

Verified on base, by reading and by the rows below:

1. **A failure the page finds reaches no peer.** `#fail` stops the single-mode tick and rejects requests (lines 999-1012). It posts nothing. The worklet's `process()` is gated only by its own phase (`miso-engine-v1-audio-worklet.js` line 1769), and in `worker` mode also by the session's Rust lifecycle word, which the Worker owns (#2267). So the session keeps playing after the page has failed, until the app disposes the host. The failures that only the page finds:
   - a malformed meter, telemetry, control-event or watermark frame (lines 709, 727, 744, 765);
   - a malformed reply, a malformed `miso.error.v1` or teardown, or a reply that matches no request (lines 791, 814, 820, 925);
   - a fence that finds no request ID or cannot be posted (lines 963, 974);
   - a port's `messageerror` (lines 569, 585);
   - in `worker` mode, the control Worker's `error` (line 597). The Worker is terminated, so nobody is left to fail the lifecycle word, and the worklet keeps rendering the render half it holds.
2. **The reverse direction.** In `worker` mode the worklet's JavaScript sticky error (`sticky`, line 1082, for example a malformed lease request or an unexpected message from the Worker) fails only the worklet's phase. The worklet holds no control half, so it cannot call `session_fail`. It posts `miso.error.v1` to the page, and the page fails with it, typed. The Worker never hears of it: it keeps its service tick, its feed drain and its render checks running (`tick`, control-worker line 505) for a session that no longer renders.

What is already right, and stays:
- A typed `miso.error.v1` from the peer that found the failure needs no post to that peer: the peer is sticky already. In `worker` mode the Worker's own sticky error also fails the lifecycle word (#2267), so the worklet silences.
- A `processorerror` means the processor threw out of its constructor or `process()`: it renders no more, so it needs no post.

## Decision
- **One new internal message, `miso.fail.v1 { tag, result }`**, sent page to peer. `result` is the host's sticky error's code. It is not a request: it carries no request ID, takes no place in the request sequence and gets no reply, like `miso.service.v1`.
  - **The worklet** handles it first on its page port, in both modes, before the mode's dispatch. A `live` phase fails sticky with `result` (`fail(result, false)`), and `failurePosted` is set, since the page holds the error. Any other phase stays as it is: a failure already recorded keeps its code, and a final phase (`released`, `disposed`) is never left. Nothing is posted back. A malformed `miso.fail.v1` falls through to the existing unknown-message path, which is sticky.
  - **The control Worker** handles it in `onPageMessage`, after the `disposed` check and before `receive`. A session that is neither failed nor disposing fails exactly as for the Worker's own first sticky error: `session_fail(handle, result)`, so the render half silences at its next block (#2267). Then `ready` is false, `stickyResult` is `result`, and the service tick stops. Nothing is posted back. A session already failed keeps its first code. A dispose already begun is left alone, because its release stops the worklet and its dispose frees the host. The shared part of `sticky` becomes `failSession(result)`, so the two paths cannot drift.
- **The host posts from `#fail` only.** `#fail(error, source, aware)` gains a third argument, `aware`: the peer that needs no post because it reported this failure itself, typed, or because its processor is gone (`processorerror`). `#receive(message, peer)` learns which port a message came from: the Worker's port is `PEER_WORKER`, and the worklet's port is `PEER_WORKLET` (in `single` mode, the only port). The two typed-error sites (lines 803 and 812) pass that peer. A malformed `miso.error.v1` is a schema failure that the page found, so it passes none. The failed record gains `workletKnows`. It is set once the worklet reported a failure, was posted one, or its processor failed.
  - **The Worker** is posted `miso.fail.v1` on the first entry into `failed` (`prior === null`), unless the failure is the Worker's own (`aware === PEER_WORKER`) or the Worker is gone (`FAIL_WORKER_GONE`). It owns the lifecycle word while it lives (#2267's Decision), so it is the one peer told while it lives. After the first entry it always knows: it was told, it reported the failure, or it is gone.
  - **The worklet** is posted `miso.fail.v1` when no live Worker will tell it, and it does not know yet. That is: in `single` mode, on the first entry; in `worker` mode, when the failure first marks the Worker gone, whether that is the first entry or a later one. A Worker can die before it handles the page's post, so the page cannot rely on that post.
  - Every post is in `try/catch`. A channel that cannot carry the post is a peer that cannot be told, and the host's failure stands.
- **Routing, by failure:**

  | Failure | `single` mode | `worker` mode |
  |---|---|---|
  | found by the page (schema, stray, fence, `messageerror`) | worklet | Worker |
  | typed error from the Worker | (none) | none |
  | typed error from the worklet | none | Worker (the reverse direction) |
  | `processorerror` | none | Worker |
  | the Worker's `error` | (none) | worklet, unless it knows |
- **Idempotent, and nothing resurrects.** The host posts at most once per peer. Each receiver ignores the message unless its session is live, so a repeated or late post changes nothing. A post after a dispose is queued behind the dispose on the same port, so it reaches a peer that is disposing or disposed, which ignores it. If a single-mode dispose is refused as retryable, the worklet does fail, and so does the host, which returns to `failed`.
- **No registry change.** `miso.fail.v1` is internal to the three shipped files, like `miso.service.v1`, `miso.fence.v1`, `miso.release.v1` and `miso.render-check.v1`. There is no tag registry. `scripts/check-web-audioworklet.sh` freezes the module's exports and the render-callback policy body, not message tags. The SDK's typed surface (`shipped-host.d.ts`, `live-controls.ts`, `default-host.ts`) names only replies the page receives. No export is added: the Worker already calls `session_fail`, and the worklet calls none.

## Non-goals
- **WEB-9** (#2298): the lifecycle word's monotone stores. `session_fail` keeps its refusal of a session that is not ready.
- **The factory's boot failures.** They tear the node and the Worker down themselves (`cleanupNode`, `disposeBootedWorker`).
- **Unsolicited frames delivered on a failed host.** On base, a well-formed meter, telemetry, control-event or watermark frame reaches its listeners after the host failed. This is unchanged and is listed as a follow-up.
- **Single mode's Rust lifecycle word.** In `single` mode the worklet's phase gates `process()` alone, as #2267 recorded.
- **WEB-5, WEB-7 and `web/stem-store/`.**

## Gates
- `scripts/test-web-audioworklet.mjs` rows, red on base:
  - main realm: the routing table above, case by case, asserting exactly which `miso.fail.v1` each peer was posted (at most one, with the sticky code), plus a second failure that posts nothing more and a Worker `error` after a failure that the worklet did or did not know of;
  - worklet: `miso.fail.v1` fails a live phase sticky in both modes (silent `process()`, a later request refused with that code, no post back, no render-check report), leaves a failed phase's code standing and a released phase final, and a malformed one is sticky `INVALID_ARGUMENT`;
  - control Worker: `miso.fail.v1` calls `session_fail` once with its code, posts nothing, stops the tick and refuses later requests with that code; a second one, or one after the Worker's own sticky error, calls nothing; one during a dispose calls nothing and the dispose completes.
- The mechanical mutant generator (#2279's `gen.py` and `run2.py`) is run over the changed lines. Every green mutant gets a row or a named equivalence.
- `bash scripts/test-web-audioworklet.sh`, `bash scripts/check-workspace-policy.sh`, `cargo run --release -p realtime-policy`. No Rust changes, so no cargo gates apply beyond `cargo fmt --check`.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `5d403acd8`, which includes #2304 (#2291's worklet trap containment), so no merge was needed.

**Red on base.** The new rows were run against base copies of the three shipped files through the suite's module overrides (`MISO_ENGINE_WEB_HOST_TEST_MODULE`, `..._WORKLET_...`, `..._CONTROL_WORKER_...`), one file at a time. Each run stops at its first new row:

```
host:   AssertionError [ERR_ASSERTION]: single: a malformed meter frame: who is told
        + worklet: []   - worklet: [ 255 ]
worklet: AssertionError [ERR_ASSERTION]: single: nothing is posted back   2 !== 1
worker: AssertionError [ERR_ASSERTION]: the lifecycle word is failed with the page's code
        [ [ 1, + 1  - 9 ] ]
```

On base the worklet and the Worker read `miso.fail.v1` as an unknown request: each refuses it sticky `INVALID_ARGUMENT` on the page's port, and the Worker fails the word with 1, not the page's code.

**Rows** (`scripts/test-web-audioworklet.mjs`, all "Issue #2317"):
- **Main realm: the routing table.** It has 22 cases, seven in `single` mode and fifteen in `worker` mode. They cover:
  - page-found failures: a malformed frame, a stray reply, a malformed reply or typed error, a `messageerror` on either port, a fence that cannot be posted;
  - typed errors from each peer: request ID 0, and a request (a control call to the Worker, a lease to the worklet);
  - `processorerror`;
  - the Worker's `error`, alone and after each other kind of failure.

  Each case asserts exactly which codes each peer was posted. A second failure must post nothing more, and a post must carry `tag` and `result` only.
- **Main realm: the early-failure loop.** It now also asserts who is told after an early `error`, an early `messageerror`, or both.
- **Main realm: an early typed error.** A typed error the Worker posts before the host takes it over is the Worker's own.
- **Main realm: the status read.** A status call on a host that failed while a read was in flight, with a later request sent after the read, settles with the sticky error.
- **Worklet.** `miso.fail.v1` is tested in both modes:
  - the output is silent, and nothing is posted back;
  - the published view says failed;
  - a later request is refused with the first code, and the dispose still runs;
  - a render check reports nothing;
  - a release still hands the half back;
  - a released or disposed instance stays final;
  - four malformed shapes are each sticky 1.
- **Control Worker:**
  - `session_fail` is called once with the page's code, nothing is posted back, and the tick stops;
  - a second post fails nothing;
  - a later source request is refused with the first code, and its planes come back;
  - a post after the Worker's own failure fails nothing;
  - a post during a dispose, or after it, fails nothing, and the dispose completes;
  - three malformed shapes take the unknown-message path.

**Mutations.** I ran #2279's generator three ways, plus a hand list:
- **The reviewer's `gen.py`/`run2.py`, over the whole host.**
  - Base, lines 466-1590: 794 mutants. 633 are RED, 9 are RED only in the MAX_SAFE run, and 152 are GREEN. These are #2279's attempt-4 counts exactly.
  - This branch, lines 466-1640: 823 mutants. 662 are RED, 9 are RED only in the MAX_SAFE run, and 152 are GREEN.
  - Compared by line text and mutation, no mutant that was red on base is green here. The only GREEN entries not on base are two. One is `early?.messages ?? []` with `||`, the same mutant as base's, on a changed line. The other is a factory-input clause past base's range cutoff (#2279's follow-up).
- **`gen2.py`/`run3.py`.** This is the generator widened to the three files, run over every line this diff adds plus the whole of `#fail` and `#tell`. To #2279's mutators it adds:
  - `aware` swaps, peer-token swaps and `#tell` target swaps;
  - the code argument;
  - deletion of each one-line statement and of each later `&&` conjunct.

  It made 150 mutants: 141 RED and 9 GREEN. The GREEN ones are below.
- **Hand mutants**, for the first conjuncts and disjuncts the generator does not drop. There are 13, and 12 are RED:
  - `tellWorker` without `prior === null`, or without `!workerGone`;
  - `tellWorklet` without `workletKnows`, without `aware`, without the mode, or without `workerGone`;
  - `workletKnows` without each of its three terms;
  - `workerGone` without `prior`;
  - the Worker's guard without `disposing`;
  - the worklet's guard widened to any phase that is not final.

Each GREEN mutant, with its equivalence:

| Line | Mutant | Equivalence |
|---|---|---|
| 593 | `early?.messages ?? []` to `\|\|` | `messages` is an array, or there is no early record (`undefined`); `\|\|` and `??` agree. Also GREEN on base. |
| 816 | `#fail` source `null` to `pending` | This site requires `requestId === 0`, and no request has ID 0 (IDs start at 1), so `pending` is `null` there. Also GREEN on base (its line 803). |
| 1012 | `if (phase.kind === "disposed") return;` deleted, or set `false` | Unreachable. `#teardown` nulls every handler that calls `#fail`: the ports, `onprocessorerror` and `worker.onerror`. The fence and `#failWorker` run only from those handlers or the constructor. This is #2279's equivalence, upheld. |
| 1016 | `tellWorker` without `this.#worker !== undefined` | In `single` mode, `#tell(undefined, ...)` throws a `TypeError` inside `#tell`'s `try`, and nothing is posted. The guard states the intent, and the catch makes its removal invisible. |
| 1022 | `prior?.error ?? error` to `\|\|` | `prior.error` is a frozen error record, never falsy. Pre-existing. |
| 1028 | the Worker's post: `failure.error.result` to `error.result` | The Worker is posted only on the first entry, where `failure.error` is `error`. The worklet's post (line 1029) can come after a first failure, and its swap is RED ("the Worker's typed error, then its error"). |
| 1034 | `this.#inFlight.clear()` deleted | #2279's equivalence: once the host has failed, no bounded request is sent again. |
| 1037 | `this.#statusFollow = null` deleted | Once the host has failed, `#statusRead` is `null` and no read is kept again (`#request` refuses one). So `#statusFollow` is not read before the teardown, which rejects an already-rejected deferred, a no-op. (The neighbouring `#statusRead = null` was GREEN too, and is now RED at the status-read row.) |
| Worker `onPageMessage` | `miso.fail.v1` handled before the `disposed` check (hand) | `disposing` stays set once a dispose has begun, so a disposed Worker is also `disposing`, and `receiveHostFailure` returns. A boot that failed never gets a host, and so never gets a post. |

**Render path.** No render code changed. `process()`, `render_next` and every `REALTIME_POLICY` region are untouched. The worklet's new handler runs on the message port, outside `process()`. So no bit-for-bit comparison applies. No cross-thread protocol was added: the Worker stores into the lifecycle word through the existing `session_fail`, as #2267 does, so no loom model is needed.

**Gates (all green):**
- `bash scripts/test-web-audioworklet.sh`, exit 0. This runs the hermetic suite, both MAX_SAFE copies, the source policy, the opcode, module-shape, timer and capability self-tests, the call-graph self-test, and the qualification, live-control, metadata, session-map and UTF-8 mutation gates.
- `bash scripts/check-workspace-policy.sh`, which reported `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`, which reported `realtime policy: ok (294 marked regions in 46 files)`.
- `cargo fmt --check`.

No Rust file changed, so no other cargo gate applies.

**Not run here:** the built-artifact legs (the SDK package checks and the Playwright browser legs). They need the pinned browser nightly. The SDK's shipped files are the three files themselves, and no export or SDK type changed.

**Test value.**
- Routing table: red if a failure the page found (or, in `worker` mode, one the worklet posted) reaches no live peer, so the session keeps playing or the Worker keeps ticking. It is also red if a peer is told twice, or told of a failure it posted itself; if a gone Worker is posted; if the worklet is left playing when the Worker dies before it could fail the word; or if a later post carries a code other than the sticky error's. No existing row reads what the host posts on a failure.
- Early typed error: red if the early messages are replayed as if the worklet had sent them, or are not replayed at all.
- Status read: red if a failure leaves the shared read behind, so a status call on the failed host never settles.
- Worklet: red if the worklet keeps rendering after the page's post, answers or refuses the post on the page's port, reports it again on a render check, lets a later post replace the first code, revives a final phase, or accepts a malformed post.
- Control Worker: red if the Worker leaves the lifecycle word ready or keeps ticking after the post, reads it as a request, calls `session_fail` again for a later post or for one during a dispose, or drops a sticky refusal's planes.

**Follow-ups (not filed):**
- Flagged by review; the coordinator tracks it. A well-formed meter, telemetry, control-event or watermark frame still reaches its listeners after the host has failed (`#receive` dispatches unsolicited frames before any failure check). A failed host should drop such a frame, or count it. This stays out of #2317 (see Non-goals).

**Pending before review: the merge with #2306 (WEB-7).** #2306 replaces the Worker's `disposing`/`disposed` flags with one `this.phase` union: `unbooted | refused | live | disposing{…} | disposed{…}`. During a dispose, the Worker's tick keeps the release deadline. On the merged tree, `receiveHostFailure`'s `this.disposing` guard would be `undefined`. A `miso.fail.v1` during a dispose would then fail the session and stop the deadline tick. The row "a post during a dispose fails nothing" goes red on the three-way merge, so the conflict is caught. Once #2306 is on `main`:
- merge `origin/main` in with a normal merge commit;
- guard on `this.phase.kind !== "live"` plus the sticky latch;
- never stop the tick outside `live`;
- re-run the generators and update the tables above.
- #2279's follow-ups stand: the factory-input, returned-plane, boot-reply and frame-schema clause swaps have no rows.
