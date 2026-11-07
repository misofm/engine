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
(empty)
