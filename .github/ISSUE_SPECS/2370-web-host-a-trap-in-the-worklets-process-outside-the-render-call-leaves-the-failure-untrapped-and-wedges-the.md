# Web host: a trap in the worklet's `process()` outside the render call leaves the failure untrapped and wedges the dispose

Successor of #2291, filed when the implicit-state-machine tracker #2258 closed. #2291 contained a trap in a worklet message-handler export and listed this case as a non-goal because the fix changes `process()`, the render path.

## Problem
Two paths in `process()` meet a trapped Wasm instance without recording it:
- A `miso_engine_web_v1_meter_poll` trap (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, around line 1729), or in `worker` mode a `miso_engine_web_v1_meter_header_ptr` trap (around line 1700), escapes `process()`. The user agent fires `processorerror` and the host fails, but `failureTrapped` (line 234, set only at line 1679) stays false. A `single`-mode dispose then calls the dispose export that the trap left refusing, gets 1, and the host retries it.
- The SDK prelude's drain (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`, around line 430) catches a `source_submit` or `source_seek` trap, counts it in `CONTROL_ERRORS`, and renders on. Nothing records the trap, so the next block and the dispose both call exports on the trapped instance.

## Contract
- Any trap that `process()` observes, whether it is caught or not, sets the one failure record exactly once. That record is the same one #2291's handler containment writes.
- After that, `process()` outputs silence and calls no export. The dispose settles without calling an export on the trapped instance, the same way #2291's trapped dispose does.
- A non-trap exception from a feed, such as a malformed ring, keeps today's behaviour: it is counted in `CONTROL_ERRORS` and the render continues. The prelude must be able to tell a `WebAssembly.RuntimeError` apart from other exceptions.

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
- **A deviation from the contract's wording.** The contract says the prelude must tell a `WebAssembly.RuntimeError` apart from other exceptions. The fix tells them apart by where the throw came from, not by its type. A `RuntimeError` comes only out of an export, so every one is a trap. Origin also catches the throws a type test would miss, the reason #2291 gave: V8 reports a Wasm stack overflow as a `RangeError`, the same type as the drain's own out-of-range copy. The test's own-fault row is exactly such a `RangeError`, and it stays counted.
- After the change, the user agent no longer fires `processorerror` for these traps, because they no longer escape. The page learns of the failure the way it does for a render trap since #2256: in `worker` mode from the control Worker's render check (`reportRenderFailure`), and in `single` mode from the sticky 255 on its next request.

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
| W4 the catch does not `return` | survives, as an equivalent mutant: it falls through to `recordRenderTime`, which calls no export, and then returns `true`. The `return` is kept so that a trapped block, like a render-trapped one, is not folded into telemetry |
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

**Follow-ups (not filed).**
- In `single` mode the page hears of a `process()` failure only at its next request. This is pre-existing for render traps since #2256, and the `processorerror` these two paths used to fire had been their only prompt report.
