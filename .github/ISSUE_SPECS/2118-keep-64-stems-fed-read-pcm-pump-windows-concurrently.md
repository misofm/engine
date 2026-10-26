# Keep 64 stems fed: read PCM pump windows concurrently

Filed by root from console slice C1a (2026-10-26) and its review.

## Problem
At 64 stems the console's PCM feed underruns (about 2,800 underrun blocks in 6 s). The app is correct: one pump, a 10 ms cadence and a lock. From reading the code, the cause is the SDK's `CanonicalPcmPump.pumpUntilFull()` (hosts/host-web/web/stem-store/pcm-pump.js, published by #2102). It fills the sources one after another and awaits each window's `blob.slice().arrayBuffer()` read inline. All 64 stems start at frame 0 and need a new 32,768-frame window in the same pass, so the last stem waits behind 63 file reads. A shorter cadence does not help (2 ms gave the same result). This is inferred, not measured.

## Smallest slice
1. Measure first: the time of one `pumpUntilFull()` pass at 64 stems in Chromium, and where it waits (read against deinterleave).
2. Read windows concurrently, with a bounded number of reads in flight, and prefetch the next window before the ring needs it. Keep the existing ordering and generation rules (seek, endOfRegion) exact.
3. Keep the published API.

## Gates
- The existing stem-pump tests stay green.
- A new test: with N sources whose reads resolve with a fixed delay d, one pass takes about ceil(N / concurrency) * d, not N * d. Red on the sequential pump.
- Descriptive: 64 stems in Chromium with 0 underruns after warm-up (record the figure).
