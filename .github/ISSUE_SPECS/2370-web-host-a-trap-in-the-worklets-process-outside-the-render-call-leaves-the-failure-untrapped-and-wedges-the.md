# Web host: a trap in the worklet's `process()` outside the render call leaves the failure untrapped and wedges the dispose

Successor of #2291, filed when the implicit-state-machine tracker #2258 closed. #2291 contained a trap in a worklet message-handler export and listed this case as a non-goal because the fix changes `process()`, the render path.

## Problem
Two paths in `process()` meet a trapped Wasm instance without recording it:
- A `miso_engine_web_v1_meter_poll` trap (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, around line 1729), or in `worker` mode a `miso_engine_web_v1_meter_header_ptr` trap (around line 1700), escapes `process()`. The user agent fires `processorerror` and the host fails, but `failureTrapped` (line 234, set only at line 1679) stays false. A `single`-mode dispose then calls the dispose export that the trap left refusing, gets 1, and the host retries it.
- The SDK prelude's drain (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`, around line 430) catches a `source_submit` or `source_seek` trap, counts it in `CONTROL_ERRORS`, and renders on. Nothing records the trap, so the next block and the dispose both call exports on the trapped instance.

## Contract
- Any trap that `process()` observes, whether it is caught or not, sets the one failure record exactly once. That record is the same one #2291's handler containment writes.
- After that, `process()` outputs silence and calls no export. The dispose settles without calling an export on the trapped instance, the same way #2291's trapped dispose does.
- A non-trap exception from a feed, such as a malformed ring, keeps today's behaviour: it is counted in `CONTROL_ERRORS` and the render continues. The prelude tells a trap from a fault by where the throw came from: a throw out of an export it calls is a trap, any other is a fault. The type cannot tell them apart, because V8 reports a Wasm stack overflow as a `RangeError`, the same type as an out-of-range copy in the drain's own code. (Amended at the attempt-1 review; the brief first asked for a `WebAssembly.RuntimeError` type test.)
- In `single` mode, a failure `process()` records (it posts nothing) is posted to the page at the next `miso.service.v1` tick as `miso.error.v1` with request ID 0, once, with no request pending. `worker` mode already has this through the control Worker's render check. (Added at the attempt-1 review: the contained traps no longer fire `processorerror`, which had been their only prompt report.)

## Non-goals
- `worker` mode's control handler calling `meter_header_ptr` or `control_allocation_count` on a trapped instance (#2291 already contains that throw; refusing the calls outright is separate hardening).
- No Rust or ABI change.

## Gates
- Red on base: one row per path (meter poll in `single` mode, meter header in `worker` mode, prelude submit trap). Each injects a trap and asserts that the dispose settles with no export call after the trap, using the existing worklet test harness.
- Realtime: `process()` allocates nothing in the new branch. Run the harness's allocation-free render check, or state why none applies.
- No bit moved: render output on a non-trapping session is identical before and after the change (PR evidence).
- Chromium and WebKit browser matrix green.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `9c391cb46`. The red rows are `90e95bb7f`; the fix is the commit after it, which also rewrites the prelude row into its final form (two rings, a `source_seek` row).

**Claims re-verified at base.** All three hold, none dropped.
- `process()` calls `postMeterFrame()` after the render call, outside its `try` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1826`). That reaches `meter_poll` (line 1729) and, in `worker` mode, `meter_header_ptr` (line 1700). `failureTrapped` is written only by `fail` (line 1679).
- The prelude's drain catch (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:432-437`) counts every throw in `CONTROL_ERRORS` and renders on. That includes a `source_submit` (line 506) or `source_seek` (line 652) throw. It does not stop at the first ring.

**Fix.**
- Worklet `process()`: `if (this.meterLease) this.postMeterFrame();` is wrapped in a `try`. Its catch does what the render trap does: `fail(RESULT_INTERNAL, true)`, `silence(outputs)`, `return true`. That is the one #2256 failure record, so the dispose (`receiveDispose`) and the release (`releaseRenderHalf`) run no export. The pinned spelling the source policy greps is kept, and the branch only assigns and fills.
- Prelude: each drain export call (`source_submit` in `drainSharedRing`, `source_seek`/`source_prepare_seek` in `applySharedSeek`) has its own `try`. Its catch throws `EXPORT_TRAP`, a frozen object made once at load. The drain catch in `process()` compares the throw with that object. On a match it calls the engine's `fail(RESULT_INTERNAL, true)` and stops the loop, and the engine's `process()` then renders silence with no export call. Any other throw is counted in `CONTROL_ERRORS`, and the block renders as before. The attach port's `prepare-seek` reaches the same `applySharedSeek`. It already takes any throw for a trap (#2291), so the object changes nothing there.
- **A deviation from the contract's first wording, approved at review and folded into the Contract.** The brief said the prelude must tell a `WebAssembly.RuntimeError` apart from other exceptions. The fix tells them apart by where the throw came from, not by its type. A `RuntimeError` comes only out of an export, so every one is a trap. Origin also catches the throws a type test would miss, the reason #2291 gave: V8 reports a Wasm stack overflow as a `RangeError`, the same type as the drain's own out-of-range copy. The test's own-fault row is exactly such a `RangeError`, and it stays counted.
- After the change, the user agent no longer fires `processorerror` for these traps, because they no longer escape. Attempt 1 left the `single`-mode page to learn of them only at its next request, as it did for a render trap since #2256. The review fold below adds the prompt report.

**Red on base** (`90e95bb7f` over the base worklet and prelude):
- `single`, meter poll (`node scripts/test-web-audioworklet.mjs`): `AssertionError [ERR_ASSERTION]: Got unwanted exception: a meter_poll trap stays inside process()` / `Actual message: "synthetic wasm trap from the meter_poll export"`.
- `worker`, meter header (the `single` row removed for the run): `Got unwanted exception: worker mode: a meter_header_ptr trap stays inside process()` / `"synthetic wasm trap from the meter_header_ptr export"`.
- A temporary probe (not committed) let each throw pass and logged what followed:
  - `single`: `trapped: false phase: live`. The dispose was answered `{"tag":"miso.ack.v1","requestId":2,"result":1}`, which is the retried wedge. After the trap, these exports ran: `dispose, service, watermark_read, control_event, control_response_bytes, control_event, control_response_bytes, meter_header_ptr`.
  - `worker`: `trapped: false phase: live`. The release answered token `4294967297` after calling `host_release` on the trapped half.
- Prelude (`cd sdk && node --test --test-name-pattern="issue 2370" test/browser-pcm-evals.mjs`, final row over the base prelude): `AssertionError [ERR_ASSERTION]: a source_submit trap: no export after it, on any ring`, actual `['source_submit', 'source_submit', 'render']`. The trap was counted, the second ring was submitted, and render ran on the trapped instance.

**Mutations of the fix.** The runner is a scratch script and is not committed. Worklet mutants run the hermetic suite over a mutated copy (`MISO_ENGINE_WEB_WORKLET_TEST_MODULE`). Prelude mutants rewrite the prelude in place, run the #2370 row, and restore it.

| Mutation | First failing assertion |
|---|---|
| W1 the `process()` meter-frame catch rethrows | `Got unwanted exception: a meter_poll trap stays inside process()` |
| W2 the catch records no trap (`fail(.., false)`) | `a meter_poll trap: the dispose is final` |
| W3 the catch does not silence the block | `a meter_poll trap silences its block` |
| W4 the catch does not `return` | survived attempt 1, as a mutant equivalent under the contract. The code falls through to `recordRenderTime`, which calls no export, but that folds a trapped block into the telemetry window and can post a telemetry frame from a failed instance, which the render-trap path never does. Red after the fold (below) |
| W5 only the `single`-mode path is contained (`worker` mode rethrows) | `Got unwanted exception: worker mode: a meter_header_ptr trap stays inside process()` |
| P1 the drain catch counts an export's trap | `a source_submit trap: no export after it, on any ring` |
| P2 the submit's catch rethrows the export's own error | `a source_submit trap: no export after it, on any ring` |
| P3 the seek's catch rethrows the export's own error | `a source_seek trap: no export after it, on any ring` |
| P4 every drain throw is a trap | `the drain's own fault is counted on each ring` |
| P5 the drain records no trap (`fail(.., false)`) | `a source_submit trap: the dispose is final` |
| P6 the drain goes on to the next ring (no `break`) | `a source_submit trap: no export after it, on any ring` |

**Realtime.**
- Each new branch runs only on a throw. The worklet's branch assigns, then `fill(0)`s the outputs, as the render trap's does, and it passes the source policy: the `PROCESS_POLICY` body still has no `new`, `BigInt` or `WebAssembly`, and its two pinned posts and the lease-guarded call are unchanged.
- The prelude's branch throws an object made at load, which captures no stack, and the drain catch compares it by identity. The only allocation is the `Error` the trapping export itself makes.
- The steady path keeps its typed-array allocation check (`moved prelude drains odd mono/stereo rings and allocation mutation turns red`, green), and the browser legs' `sdk-render-allocations` are all 0.

**No bit moved** (one-time PR evidence, not a committed test). The same shipped module (`284cbdeb...`, no Rust change) was run with the base worklet and prelude, then with this change's, in the Chromium `single` and `worker` legs. All four render digests the logs print are identical in both legs:
- `structural-replace-large` `383be91c...`
- `warm-growth` `25784310...` (twice)
- `structural-edit` `c51d28da...`

Each run also passed `native-corpus-digest`, where the browser PCM equals the native corpus pin.

**Gates.**
- `bash scripts/test-web-audioworklet.sh`: exit 0 (hermetic suite and its host variants, source, clock, timer and capability policies, call-graph self-test, metadata and session-map gates, UTF-8 mutation).
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- SDK evals without an artifact (`cd sdk && node --test 'test/*-evals.mjs'`): 717 tests, 347 pass, 22 fail. Every failure is the missing artifact (`MISO_ENGINE_SDK_ARTIFACTS_HEX ...`).
- The same evals with the artifact built here (`scripts/build-web-audioworklet.sh`, toolchain `nightly-2026-08-20`): 716 tests, 716 pass.
- `bash scripts/check-sdk-types.sh`: passed. `python3 -B scripts/check-sdk-deletions.py`: ok. `bash scripts/sdk-package.sh check <artifact>`: `SDK publishable-tarball gate passed`.
- Browser qualification (`npm run qualify -- --check-matrix --self-test-mutations`): Chromium `single` and Chromium `worker` both `all qualification gates passed (151.0.7922.34)`. The browser's missing system libraries were unpacked into a private directory, with no system install. Audio ran without pulseaudio.
- **WebKit not run here.** Its WPE build needs `libjxl.so.0.8` and `libbacktrace.so.0`, which this host's Ubuntu 24.04 archive does not carry, and installing them needs root. The WebKit legs are left to CI's browser matrix.
- No Rust, ABI or render-thread export change, so cargo test, clippy, fmt, rustdoc and the realtime policy tool do not apply. No committed artifact digest covers either JS file: the `.sha256` pins are the Wasm modules', and the SDK manifest is generated at packaging.

**Test value.**
- `single` meter-poll row (`scripts/test-web-audioworklet.mjs`, "Issue #2370, `single` mode"): red if a `meter_poll` trap escapes `process()` or goes unrecorded, after which the dispose calls the refusing export and the host retries it. Also red if the trapped block's output stands. No existing row throws from an export in `process()` other than render.
- `worker` meter-header row ("Issue #2370, `worker` mode"): red if the containment misses the `worker`-mode header read (W5), after which the release calls `host_release` on the trapped half.
- Prelude row (`sdk/test/browser-pcm-evals.mjs`, "issue 2370"):
  - red if a `source_submit` or `source_seek` trap is only counted, or if the drain goes on to the next ring;
  - red if every drain throw is taken for a trap. No existing test reaches the drain catch, so the counted-fault behaviour had no test.
- No existing test was made useless.

**Verdict on attempt 1 (Sol): PASS, minors only.** The origin-based classification is approved. Four minors to fold:
1. The `.d.ts` "Trap means processor death" text is stale.
2. The prompt report that `processorerror` gave was lost, and `single` mode should post the unposted failure on its service tick.
3. W4 should be called "equivalent under the contract", or better, killed with a telemetry case.
4. The Contract should state the origin rule.

The coordinator files the unguarded `receiveControlPlane` export calls on a trapped instance as a follow-up.

### Attempt-1 review fold (Terra, 2026-11-08)
**Changes.**
- **Worklet, `single` mode:** the page port's handler calls the existing `reportRenderFailure()` on a `miso.service.v1` tick, right after `receive`. That is message-handler work, never `process()`. It posts a failure `process()` recorded and nobody posted (`failurePosted` false), once, as `miso.error.v1` with request ID 0. That gives render traps, render refusals and meter faults a prompt report too. Request-carrying messages already answer with the sticky code, and the worklet's doc comment on `reportRenderFailure` names both triggers. `worker` mode is unchanged: the control Worker's render check already does this.
- **`.d.ts`:** "Trap means processor death" became "A trap ends the instance, never the page", in `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and its mirror `sdk/src/browser/shipped-host.d.ts`. The two stay byte-identical, which `check-sdk-generated.sh` pins. The text says that every export call is contained (render, the meter frame, the SDK drain, the message handlers), that no export runs after a trap and the dispose is answered without one, and how the page hears of it: request 0 at the next service tick (`single`) or the next render check (`worker`), or the in-flight request.
- **Test, `single` meter-poll row:**
  - It now holds the telemetry lease and traps on the window's 128th block, then asserts that the trapped block posts no telemetry frame. This kills W4.
  - It sends a service tick and asserts the one post `{ tag: "miso.error.v1", requestId: 0, result: 255 }`. A second tick posts nothing.
  - The row is red without the tick report: `AssertionError [ERR_ASSERTION]: a meter_poll trap is posted to the page at the next service tick`. That is mutant W6, and it was also run over the fold's tests before the fix line was added.
- **Contract:** amended with the origin rule and the tick report (see above).

**Mutations after the fold.** All 12 are red.
- W1-W3, W5 and P1-P6 are red as in the table above.
- W4 (no `return`) is red: `a meter_poll trap: the trapped block posts no telemetry frame`.
- W6 (the tick reports nothing) is red: `a meter_poll trap is posted to the page at the next service tick`.

**Gates after the fold.**
- `bash scripts/test-web-audioworklet.sh`: exit 0. No existing row changed: no existing row both fails `process()` and sends a tick expecting silence.
- `bash scripts/check-workspace-policy.sh`: ok.
- `bash scripts/check-sdk-generated.sh <artifact>`: current, which includes the shipped `.d.ts` mirror pin.
- `bash scripts/check-sdk-types.sh`: passed. `bash scripts/sdk-package.sh check <artifact>`: passed.
- SDK evals with the artifact: 716/716.
- Chromium `single` and `worker` (`--check-matrix --self-test-mutations`): all qualification gates passed. The four render digests match base again.
- The artifact for these runs is the attempt-1 build with this commit's `miso-engine-v1-audio-worklet.js` and `.d.ts` copied in. The builder only copies those files, and the Wasm module is unchanged.
- WebKit is still left to CI, for the reason above.

### Second review fold (Terra, 2026-11-08)
The coordinator ruled that the first fold's follow-up is this PR's to fix.

**The defect.** In `single` mode the main-realm host took a `miso.error.v1` with request ID 0 as a malformed reply (`validRequestId(0)` is false), and failed with `webError(255, oldest pending or 0)`. The tick report the first fold added therefore turned a non-trap render refusal (8, say) into 255 and lost its code.

One correction to the premise. `single` mode already posted request-0 errors before the fold: #2291's tick-step failures and its meter-view fault on a tick do (`trap(0)`, `sticky(.., 0)`). Those reached the same malformed path, so this fix gives them their typed code too.

**Fix.**
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`: the #1332 S8 branch that takes a request-0 `miso.error.v1` as an unsolicited, typed failure no longer requires `worker` mode (the `this.#worker !== undefined` term is gone). Its guards are unchanged:
  - exact fields `tag`, `requestId`, `result`;
  - a valid, non-OK result.
- The branch calls `#fail(message, null, peer)`, so in `single` mode the worklet, which sent the failure, is not told it back.
- Any other request-0 shape still falls to the malformed path and fails 255. There is no second path.
- The `.d.ts` section (both copies, still byte-identical) adds that a non-trap render refusal reaches the page the same way, with its own code.

**Row** (`scripts/test-web-audioworklet.mjs`, main-realm suite, "Issue #2370: in `single` mode the worklet posts a failure"):
- a `single`-mode host, with a request held in flight, receives `{ tag: "miso.error.v1", requestId: 0, result: 8 }` on the worklet port;
- the held request and a later request both reject with 8;
- no `miso.fail.v1` goes back to the worklet.

Red on `9d844ed99`: `AssertionError [ERR_ASSERTION]: Expected values to be strictly equal: 255 !== 8`, at the held request's `errorResult(held, 8)`.

**Host mutations** (the hermetic suite over a mutated host copy, `MISO_ENGINE_WEB_HOST_TEST_MODULE`):
- H1, request 0 accepted in `worker` mode only (the `9d844ed99` guard): RED, `255 !== 8` at the held request.
- H2, the request-0 failure names no peer (`#fail(.., null, null)`, so the worklet is told back): RED, `single mode: the worklet is not told its own failure`.

**Gates after the second fold.**
- `bash scripts/test-web-audioworklet.sh`: exit 0, including the safe-integer host copies and their red mutation.
- `bash scripts/check-workspace-policy.sh`: ok.
- `check-sdk-generated.sh <artifact>`: current.
- `check-sdk-types.sh`: passed (shipped-host mirror pin included).
- `sdk-package.sh check`: passed.
- SDK evals with the artifact: 716/716.
- Chromium `single` and `worker`: all qualification gates passed, and the four render digests match base.
- The artifact is the attempt-1 Wasm build with this commit's worklet JS, host JS and `.d.ts` copied in.
- WebKit is still left to CI.
