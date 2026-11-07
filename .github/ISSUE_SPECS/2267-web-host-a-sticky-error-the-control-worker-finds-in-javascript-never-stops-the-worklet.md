# Web host: a sticky error the control Worker finds in JavaScript never stops the worklet

Child of #2258 (implicit state machines), item WEB-2. Found by the audit of 2026-11-06 and re-verified at base `de0878e23`.

## Problem
In `worker` mode a session's failure is held in four places: the page host's `#stickyError` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` `#fail`, line 959), the control Worker's `ready`/`stickyResult` (`miso-engine-v1-control-worker.js` `sticky`, line 555), the worklet's phase (`miso-engine-v1-audio-worklet.js`, #2256), and the session's Rust lifecycle word, an `Arc<AtomicU32>` the control half and the render half share (`hosts/host-web/src/lib.rs` `type Lifecycle`, line 939). The render half reads that word before every block and silences once it is not ready (`RenderHalf::render_next`, line 3194). The Rust control half writes it on every failure the engine records (`AudioWorkletEngineHost::fail`, line 2874).

The control Worker also finds sticky errors in JavaScript, with no engine call failing:
- an out-of-order or non-integer request ID (`receive`, line 571);
- an unknown tag or a malformed message (line 620);
- a session map that `readSourceRows` cannot read (line 606);
- a bad record size (lines 766, 782);
- an unexpected message from the worklet (`onWorkletMessage`);
- a non-OK result from the service step, the watermark or the event drain (`fatal`).

`sticky` sets `ready = false` and `stickyResult` and posts `miso.error.v1` to the page. Nothing reaches the lifecycle word, so the render half in the worklet keeps rendering the session after the page has been told that the host failed. The Worker's own tick stops (line 515), so the render check never runs again either. The page's `#fail` only rejects requests; it does not touch the node. The result is audible: the transport keeps playing until the app disposes the host.

## Decision
While the control Worker is alive, the Rust lifecycle word is the one owner of "the session failed". It cannot be the owner once the Worker is gone (trapped or terminated): nothing is left to store into it, so the page-side follow-up must fail the worklet's phase by message instead. The Worker's `sticky` is already the single funnel for its sticky errors (`fatal` calls it, and nothing else writes `stickyResult`). The first time it runs, it now also fails the session in Rust through a new control-path export:

- `miso_engine_web_v1_session_fail(handle, result) -> u32` calls `AudioWorkletEngineHost::fail_session(code)`. That is the existing control-half `fail`: the lifecycle word goes to `STATE_FAILED`, the status's last result becomes `code`, and the diagnostic becomes `web.host.failed\t$\n`. It returns `RESULT_OK`. The refusals:
  - `RESULT_INVALID_ARGUMENT` for a wrong handle or a zero code;
  - `RESULT_WRONG_STATE` for a session that is not ready. That covers one already failed by either half, one disposed, or no control half here.

  A refusal changes nothing, the last result included. A failure recorded first keeps its code and diagnostic, and the export never overwrites `STATE_DISPOSED`. That ordering is the only part of WEB-9 this fix needs. The refusal is a check-then-store (`control_ready()`, then the store), not a compare-and-swap. That is benign: the only other writer on another thread is the render half, which stores only `STATE_FAILED`, and `STATE_DISPOSED` is stored on this same control thread. WEB-9 owns making that ordering structural, together with the render half's own `fail` and `reject_output_quantum`, which still store plainly.
- The released render half then silences from its next block and returns `RESULT_WRONG_STATE`. The worklet's `process()` takes its existing failure path (#2256: phase `failed`, silence). No render code changes.
- The call is wrapped in `try/catch`, so a trapped or mismatched instance still posts the sticky error to the page.
- The export is a V1 in-place amendment of the web ABI surface, added to every list that freezes it:
  - `scripts/check-web-audioworklet.sh`;
  - `scripts/check-abi-layout-v1.py` and its self-test fixture;
  - `tools/parameter-metadata/src/abi_layout.rs`;
  - `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`, regenerated.

  The ABI version is unchanged. The release digest pin is not re-pinned per change (#1061).

## Non-goals
- **Page-side failures.** A Worker `error`, a malformed reply, `processorerror` or `messageerror` fails only the page's `#fail`, and the Worker and the worklet do not hear of it. This is a separate product outcome, and a separate issue. With the Worker gone, it must fail the worklet's phase by message.
- **The reverse direction.** A worklet-side JavaScript sticky error in `worker` mode stops the worklet but not the Worker. This is a separate issue.
- **Single mode.** It needs nothing: its worklet's `sticky` already sets the phase that gates `process()`.
- **Neighbouring tracker items.** WEB-5 (the ring seek consumer), WEB-7 (the Worker's phases), WEB-9 beyond the refusal above, and `web/stem-store/`.

## Release note
If this merges before release #2253 (0.10.5), that release's "Upgrading from 0.10.4" section in `sdk/README.md` must list the added export `miso_engine_web_v1_session_fail`. The SDK's `exportsOf` refuses an engine asset that lacks any export the layout lists, so the SDK needs an engine artifact rebuilt from this change.

## Gates
- The hermetic control-Worker row (`scripts/test-web-audioworklet.mjs`, "Issue #2267") must be red on base. It covers an out-of-order request ID, an unknown tag, an unreadable session map, and a trapped `session_fail`. In each case the page gets the sticky error, and the Worker calls `session_fail` once with the same code.
- A Rust unit test (`hosts/host-web/src/tests.rs`, `a_host_found_failure_silences_the_released_render_half_and_overwrites_nothing`) checks four things:
  - `fail_session` silences a released render half at its next block, with positive-zero output;
  - the control diagnostic is `web.host.failed\t$\n`, and the adoption back keeps it;
  - later control calls are refused;
  - a second failure keeps the first code;
  - a disposed session stays disposed.
- `cargo test --locked -p host-web -p parameter-metadata --all-features`, clippy `-D warnings` on both, and `cargo fmt --check`.
- `bash scripts/check-workspace-policy.sh`, the realtime policy tool, `bash scripts/test-web-audioworklet.sh`, `bash scripts/check-sdk-generated.sh`, and `scripts/check-abi-layout-v1.py --self-test`.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `de0878e23`.

**Red on base.** The JavaScript row was added before the fix, with the fake export only, and failed: `node scripts/test-web-audioworklet.mjs` gave

```
AssertionError [ERR_ASSERTION]: an out-of-order request ID: the session's lifecycle word is failed with the same code
+ actual - expected
+ []
- [ [ 1, 1 ] ]
```

The earlier assertion in the same row passed: the page did get `{ tag: "miso.error.v1", requestId: 1, result: 1 }`. The Rust test does not compile on base, because `fail_session` does not exist there.

**Mutations of the fix** (each one red, then restored):

| Mutation | Result |
|---|---|
| Worker passes `RESULT_INTERNAL` instead of the sticky code | JS row red ("…failed with the same code") |
| Worker drops the `try/catch` | JS row red: the trap escapes the handler |
| `fail_session` writes only this host's status (`render.silence(); record(code)`) and not the lifecycle word | Rust test red: `(2, 1)` vs `(3, 1)` |
| `fail_session` drops its not-ready refusal | Rust test red: the second failure is accepted (`0` vs `3`), so it would overwrite the first code |
| The diagnostic string becomes `web.host.fail` (MINOR 3) | Rust test red at the diagnostic assertion |
| `adopt_render` clears the control diagnostic (MINOR 3) | Rust test red: "the adoption keeps the session's diagnostic" |
| Worker never calls the export (= base) | JS row red, as recorded above |

**Render path.** No render code changed: `render_next`, `process()` and every `REALTIME_POLICY` region are untouched, and the new export is control path (`with_host_mut`, never render-locked). So no bit-for-bit comparison applies. No new cross-thread protocol was added. The control thread stores into the existing lifecycle word with `Release`, the same store every engine-recorded control failure already makes, and the render thread's `Acquire` load is unchanged. So no loom model was added.

**Gates run (all green):**
- `cargo test --locked -p host-web -p parameter-metadata --all-features`;
- `cargo clippy --locked -p host-web -p parameter-metadata --all-targets --all-features -- -D warnings`;
- `cargo fmt --check`;
- `bash scripts/check-workspace-policy.sh`, which reported `workspace policy: ok`;
- `cargo run --release -p realtime-policy`, which reported `realtime policy: ok (292 marked regions in 46 files)`;
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- `bash scripts/check-sdk-generated.sh`;
- `check-abi-layout-v1.py --self-test`, 44 mutations caught.

The generated layout still equals the self-test fixture byte for byte.

**Not run here:** the wasm artifact build and its artifact-backed gates (the call graph, the export set against the built module, the SDK artifact evals and the Playwright legs). The pinned browser nightly is not installed. CI's artifact job checks the export set against the built module.

**Test value.**
- JS row: red if the Worker reports a sticky error found only in JavaScript to the page but leaves the session's lifecycle word ready, so the worklet keeps rendering. It is also red if the code differs or a trap escapes. No existing row reads the lifecycle after a Worker-side sticky error.
- Rust test: red if `fail_session` fails only the status shell a release leaves, and not the shared word the released half reads. It is also red if it overwrites an earlier failure's code or a disposed state, or if the promised `web.host.failed` diagnostic is not the one the session keeps across the adoption back. No existing test fails a session from outside the engine.

**Follow-ups (not in this issue).** The coordinator files the first two as stateless issues.
- Page-side failures should reach the Worker and the worklet: a Worker `error`, a malformed reply, `processorerror` and `messageerror`. With the Worker gone, the worklet's phase must be failed by message.
- A worklet-side JavaScript sticky error in `worker` mode should reach the Worker.
- WEB-9: monotone lifecycle transitions for the remaining plain stores, including `fail_session`'s check-then-store.

**Verdict on attempt 1 (Sol): PASS with four MINORs.** All four are folded in the commit after `0431623a7`.
1. The Decision no longer claims one owner when the Worker is gone; it names the page-side follow-up.
2. The follow-ups stay in prose; the coordinator files them.
3. The `web.host.failed` diagnostic was untested. The Rust test now reads it after `fail_session` and after `adopt_render`; both mutations are in the table above.
4. The release note above was added.

The reviewer's answers:
- The ABI export is the right model. Failing the session through `dispose` or through a worklet message would leave the two owners (the lifecycle word and the worklet's phase) disagreeing.
- The page-side cases are a separate product outcome.
- CI's artifact job builds fresh. A pinned-nightly `--module-only` build exports exactly the 61 layout calls, `session_fail` included.
- `fail_session` is a check-then-store rather than a compare-and-swap. This is benign today and is recorded under WEB-9.
