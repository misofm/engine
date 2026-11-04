# Gate suspend-then-resume of a rendered context on WebKit and Chromium

Filed by root on 2026-11-04 from the verify of #1531's closure. #1531 closes with this issue as its successor.

## Problem
#1531's diagnosis found that a rendered AudioContext, once suspended and resumed, can stall on WebKit: the clock does not advance and nothing renders. It stalled in 19 of 60 runs with the engine and in 0 of 510 trivial runs, so the cause may be in the engine package. The studio console boots on exactly this path: it suspends a context that has rendered (`studio-v2 src/studio/engine-client.ts:382`) and resumes it on a click (`:654`). No product gate covers it; #1535 covers observation unsubscribe with one resume per context.

## Scope
- A browser qualification case on Chromium and WebKit, both instance modes: boot, render a short first segment (6 quanta or fewer, as the console does), suspend, resume, and require that `currentTime` and `renderedQuanta` advance and that audio renders, repeated enough times to catch a 1-in-3 stall.
- If the case shows the stall, find the cause (engine package or browser) and fix it in the engine if it is ours; if it is the browser, record the evidence and a work-around the SDK applies.

## Gate
The case passes 20 consecutive runs per leg with no rerun, and a mutation that skips the resume path turns it red.
