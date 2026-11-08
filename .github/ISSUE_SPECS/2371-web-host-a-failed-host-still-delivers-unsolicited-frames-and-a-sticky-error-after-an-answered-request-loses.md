# Web host: a failed host still delivers unsolicited frames, and a sticky error after an answered request loses its typed code

Successor of #2317 and #2306, filed when the implicit-state-machine tracker #2258 closed. Both are failure-path reporting gaps on the page side, in `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`.

## Problem
1. **Frames after a failure (#2317 non-goal).** After the host fails, a well-formed meter, telemetry, control-event or watermark frame that is already in flight still reaches its listeners. A consumer can therefore see engine progress after it was told the engine failed. Base behaves the same.
2. **A lost typed code (#2306 follow-up).** When a sticky error arrives after its request was already answered, the page reports the failure without the typed code 3 that the sticky carries. The failure a consumer sees is therefore less specific than the one the Worker reported.

## Contract
- Once the host's lifecycle is failed or disposed, no unsolicited frame is delivered to a listener. The frame is dropped and counted in one existing telemetry counter, or in a new one that is documented in the `.d.ts`.
- The failure record a consumer reads always carries the typed code of the first sticky error, whether or not that error's request was already answered.

## Non-goals
- The worklet and the control Worker (#2317 already routes page-detected failures to them).
- The factory's boot failures.

## Gates
- Red on base: one row per claim. (1) A failed host whose listener receives a frame posted after the failure. (2) A sticky error with code 3 that follows an answered request and loses its code.
- Chromium and WebKit matrix green, and the `.d.ts` updated if a counter is added.

## Evidence / attempt record

### Attempt 1 (Terra), base `44fed08d5`, branch `codex/issue-2371-failed-host-frames-and-answered-sticky`

Line numbers below are in `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` on the branch head unless they say "base".

**Re-verification.** Both claims hold at base.
1. On base, `#receive` dispatched the four unsolicited tags first (base lines 681-791). Each branch checked the frame's shape and then called its listener. Nothing read the lifecycle, so a well-formed frame on a failed host reached its listener.
2. #2370 changed only the request-0 branch (base line 814): it dropped the `this.#worker !== undefined` clause, so `single` mode also takes a request-0 error typed. Claim 2 is about an error whose ID is a request that was already answered. The worklet's message handler reports the steps it runs after `receive` (`serviceControl`, `pushWatermark`, `drainControlEvents`, `syncMeterViews`) to `this.lastRequestId`. #2370 sets that ID to 0 only for a service tick (`miso-engine-v1-audio-worklet.js` lines 309-311). The control Worker's `afterControl(this.lastRequestId)` (`miso-engine-v1-control-worker.js` line 251) does the same. In both modes the page has already settled that request. On base the error then went to the general `miso.error.v1` branch (base line 820), found `pending === null`, and failed with `webError(255, oldest)`. #2370 did not fix claim 2, so it stays in scope.

**Fix.**
1. A new `UNSOLICITED_TAGS` set (line 16) holds the four frame tags. At the top of `#receive` (line 687), a frame with one of these tags on a host whose `#failure()` is non-null is dropped and counted in a new `discardedFramesAfterFailure` (getter at line 1452, documented `@internal` in both `.d.ts` copies, which stay byte-identical). This happens before the schema check: on a failed host, a malformed frame could only call `#fail` again, and that keeps the first error and tells no peer a second time. `#failure()` also covers a failed host whose dispose is in flight. A disposed host clears its port handlers in `#teardown`, so no frame reaches it there.
2. The request-0 branch now also takes a typed `miso.error.v1` that matches no pending request and whose ID is one this host allocated (`validRequestId(id) && id <= #lastRequestId`, line 834). It must also have exactly `tag, requestId, result`, a valid result and no `RESULT_OK`. The host then fails with the error unchanged, `#fail(error, null, peer)`: requests in flight and every later request reject with that error's code and ID, and the peer that posted it is not told back. The error may not carry planes, because the reply already returned them. An ID the host never sent (999, -1, 1.5) is still a stray and fails with 255.

**Tests** (`scripts/test-web-audioworklet.mjs`, `testMainRealm`).
- Claim 1, the row at line 1376. A single-mode host subscribes all four listeners, and a well-formed frame of each kind is delivered while the host is live. A `messageerror` then fails the host. The same four frames reach no listener, and they count 4 in `discardedFramesAfterFailure` and 0 in `discardedControlEvents`. While a dispose of the failed host is held, four more frames are dropped, for a count of 8. Defect it catches: a frame dispatched before any lifecycle check, so a consumer sees engine progress after it was told the engine failed. No existing row catches this. #2279's row checks only that a *malformed* frame never reaches a listener.
- Claim 2, the row at line 2802, run in both modes. A request is held in flight. A later request is answered, and it is the host's last ID in `single` mode. The peer then posts `miso.error.v1 { requestId: <answered>, result: 3 }`. The held request rejects with 3 and the answered request's ID, a later request rejects with 3, and neither peer gets a `miso.fail.v1`. Defect it catches: the error taken for a reply that matches no request, so the page fails with 255 and the oldest ID and tells the peer about a failure that peer posted itself. A guard row follows it: a typed error to an ID this host never sent (999, -1, 1.5) still fails 255. That row kills the mutants that read any ID as answered.
- Changed, not added: the #2317 routing table checked "a second failure tells nobody again" by injecting a malformed meter frame. With claim 1's fix that frame is dropped before `#fail`, so the row would no longer reach a second failure. It now uses a port `messageerror` (line 3323). Without that change the assertion would have passed vacuously.

**Red on base.** Each row was run against base's host (`git show 44fed08d5:…host.js`, through `MISO_ENGINE_WEB_HOST_TEST_MODULE`). Where an earlier row would fail first, a probe copy of the test skipped that row's assertions.
- Claim 1: `AssertionError: a failed host delivers no frame`. Actual: `['meter','telemetry','control-event','watermark']` heard twice; expected: once. With the counter lines kept, base fails earlier, at `a live host drops none` (`undefined !== 0`).
- Claim 2, `single`: `errorResult(held, 3)`: `255 !== 3`.
- Claim 2, `worker` alone: `255 !== 3`.

**Mutation of the fix.** 12 hand mutants, each run through the whole hermetic suite. All 12 are red:

| Mutant | Red at |
| --- | --- |
| claim-1 guard removed | a failed host delivers no frame |
| claim-1 count removed | each frame after the failure is counted |
| `#failure() !== null` replaced by `#phase.kind === "failed"` | a failed host's dispose in flight delivers no frame |
| each of the four tags dropped from `UNSOLICITED_TAGS` (4 mutants) | a failed host delivers no frame |
| claim-2 answered clause removed | `255 !== 3` |
| `<= #lastRequestId` bound removed | the stray row, `3 !== 255` (ID 999) |
| `validRequestId` removed from the clause | the stray row, `3 !== 255` (ID -1) |
| `<=` replaced by `<` | `255 !== 3` (single mode, where the answered ID is the last one) |
| `pending === null` removed | `single: its own typed error settles the dispose at once` (a pending dispose's own typed error lost its source) |

**Render path.** Not touched: only the main-realm host, its `.d.ts` and the hermetic test changed, with no Rust and no worklet or Worker code. So the no-bit-moved comparison, loom, the realtime policy tool and the cargo gates do not apply.

**Gates.**
- `bash scripts/test-web-audioworklet.sh`: exit 0, including the hermetic suite, the safe-integer allocator copies and the policy mutations.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-sdk-generated.sh`: current, and the two `.d.ts` copies are identical.
- `bash scripts/check-sdk-types.sh`: the sdk typecheck and the shipped-host mirror pass.
- `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`: 717 tests, 717 pass. The artifact was built from this branch by `build-web-audioworklet.sh --named-twin`. Its shipped module is `284cbdeb…`, the same bytes as #2370's, since no Rust changed.
- Browser qualification, `npm run qualify -- --check-matrix --self-test-mutations`: Chromium `single` and Chromium `worker` both print `all qualification gates passed (151.0.7922.34)`. The browser's missing libraries came from a private unpacked root, with nothing installed on the system.
- **WebKit not run here.** Its build needs `libgtk-4.so.1`, `libWPEWebKit-2.0.so.1`, `libbacktrace.so.0` and other libraries that this host lacks, and installing them needs root. The WebKit legs are left to CI's browser matrix, as for #2370.

**Dropped claims.** None.

**Verdict on attempt 1 (Sol): PASS, with three MINORs.** They are folded on the same branch, after `origin/main` was merged in. That merge brought in #2372, which touches only the stem store, and it applied without conflict.

**The fold.**
1. **The bound was the shared allocator.** In `worker` mode both peers draw IDs from one allocator, so `<= #lastRequestId` also admitted an ID the posting peer never received, such as a fence posted by the Worker. The host now keeps `#lastSent`, the highest request ID posted on each peer's port. `#request` writes it only after the post succeeds, and the clause is bounded by the posting peer's value.
   - **Fences are not counted.** In `worker` mode the worklet answers a fence at once and reports nothing to it afterwards: a fault after an answer is request 0's there (`miso-engine-v1-audio-worklet.js`, `receiveWorkerMode`). So an error carrying a fence's ID is a stray from either peer. Counting fences would only admit IDs nobody reports to.
   - **What a per-peer maximum can and cannot do.** It excludes an ID above the posting peer's highest. It does not exclude a lower ID that was sent only to the other peer, for example the worklet posting a control ID that is lower than a render-port request it was sent. Excluding that would need the set of IDs each peer was sent, which is unbounded state. No real peer posts such an error, so the bound stops at the maximum.
2. **The trap section of both `.d.ts` copies** now covers a failure found after a reply. It is posted with that reply's request ID (by the worklet in `single` mode, by the Worker in `worker` mode), and the page fails with the error's own code. The two copies are still byte-identical.
3. **Follow-ups** are recorded below.

**New rows in the fold.** Each kills a fold mutant that no other row catches.
- **A post that threw.** In `single` mode a control request's post throws, so the request rejects locally and the host stays live. A typed error to that ID still fails with 255. Defect it catches: the ID recorded before the post, so an ID the peer never received passes for an answered one.
- **Crossed IDs in `worker` mode, three cases, each 255:**
  - the Worker's error carrying the control reply's fence ID;
  - the Worker's error carrying a render-port request's ID (`renderAllocationCount`, sent only to the worklet);
  - the worklet's error carrying the fence ID it answered.
  
  Defects they catch: the clause bounded by the shared allocator, a request counted for both peers, or a fence counted. The first case goes red by itself under the shared-allocator mutant: the post-threw row's assertion was relaxed in a probe copy, and the run fails at "the Worker, to the fence".

**Fold mutation.** 19 hand mutants: the 12 above, then 7 for the fold. All 19 are red. The fold's 7:
- the per-peer bound replaced by the shared allocator;
- the request post not recorded;
- the post recorded for the wrong peer;
- the post recorded for both peers;
- a fence counted for the worklet;
- a fence counted for the Worker;
- the ID recorded before the post.

Before the new rows, three of these were green: "both peers", "fence not counted" (the fence was counted then) and "recorded before the post". The rows above were added for them, and the fence record was removed for the reason in fold item 1.

**Fold gates.**
- `test-web-audioworklet.sh`: pass.
- `check-workspace-policy.sh`: ok.
- `check-sdk-generated.sh`: current, and the `.d.ts` copies are identical.
- `check-sdk-types.sh`: pass.
- `check-sdk-headless.sh`: 717 of 717 pass. The artifact was rebuilt from this tree, and its module is still `284cbdeb…`.
- Chromium `single` and `worker`: all qualification gates passed.
- WebKit: still left to CI.

**Follow-ups (not filed).**
- **The two kinds of stray report different IDs.** A typed error to a stray ID fails with 255 and the *oldest pending* ID. A stray `miso.ack.v1` fails with the stray's own ID (#2279). This is outside this issue's contract.
- **Early frames are not counted.** The constructor replays the Worker's early messages before it applies `early.failed`. So a frame among them is handled before the host knows of a Worker failure, and it is not counted in `discardedFramesAfterFailure`. With no listener subscribed yet, a control event among them is counted in `discardedControlEvents` instead. No listener can exist yet, and boot is a non-goal here.
