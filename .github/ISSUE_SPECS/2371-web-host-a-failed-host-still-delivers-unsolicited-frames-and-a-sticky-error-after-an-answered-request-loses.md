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
(empty)
