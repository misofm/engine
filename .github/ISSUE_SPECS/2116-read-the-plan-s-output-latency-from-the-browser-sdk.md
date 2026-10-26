# Read the plan's output latency from the browser SDK

Filed by root from console slice C1a (2026-10-26).

## Problem
The browser SDK gives no call to read the current plan's output latency, although the module exports `miso_engine_web_v1_plan_latency_read`. The console's fixed shape has 579 samples of latency at 48 kHz (soft clip on three stages plus the limiter). The app needs it to align meters, the playhead and any visual with the audio, and today it would have to hard-code it.

## Smallest slice
`engine.planLatency(): Promise<{ samples: number, ... }>` (or a field on the status), read from the host export, on both the browser and headless engines, with the same meaning as the C ABI's latency read. A README line.

## Gates
One eval: a session with a known latency (for example a limiter insert) reports exactly that number.
