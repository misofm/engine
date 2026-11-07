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
### Attempt 1 (Terra)
(filled in below)
