# WebKit worker: pcm-feed-stopped-seek reads the landed generation after a fixed 300 ms

Filed by root from the #1713 build (2026-11-01). Seen once; the rerun passed. Not caused by #1713 (route code), per its verify.

## Problem
The browser qualification gate `pcm-feed-stopped-seek` failed once on the WebKit `worker` leg with "did not land seek 5". The check runs inside the suspended probe (`sdk/.../pcm-feed-suspended-entry.ts:299`): after `play()` it waits a fixed `wait(300)` and then reads the landed generation. That is a wall-clock assumption on the #2205 seek path (60efcf8b4). On a slow WebKit run the generation can land after 300 ms, so the gate reads too early.

## Scope
Make the check wait for the condition, not for a fixed time: poll the landed generation (or await the SDK's own landing signal) with a bounded deadline that names the reason when it expires. Keep the gate's claim (the stopped seek lands generation 5 at the target, with 0 underrun frames). Chromium and WebKit only.

## Gate
- The check passes 20 consecutive WebKit worker runs on the CI shape, with no rerun.
- It is red when the seek never lands (the deadline expires with the named reason).
