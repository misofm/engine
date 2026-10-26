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

## Evidence and decision record (attempt 1)

**Harness** (scratchpad only, not committed): Chromium 151.0.7922.34 headless (Playwright 1.62.1), main thread, cross-origin isolated. 64 stems of 24-bit stereo, 960,000 frames (20 s at 48 kHz) each, as in-memory Blobs and as disk-backed Files (`setInputFiles`, page-cache warm). The console's shape: rings of 512 chunks of 128 frames (`createFixtureMsb1Ring` + the pump's `Msb1RingWriter`, the MSB1 V1 layout the SDK writer shares), `windowFrames` 32,768, so a cold pass reads two windows per stem (128 reads). Steady state: a Worker drains every ring at 48 kHz (one 128-frame block per 2.667 ms); the pump runs `pumpUntilFull()` then sleeps 10 ms, as the console does; 10 s, underruns counted after 1 s of warm-up. One invocation per pump, descriptive.

| | before (sequential) | after (concurrent) |
|---|---|---|
| Cold pass, memory Blobs | 251 ms | 202 ms |
| Cold pass, disk Files | 260 ms | 188 ms |
| Reads in flight (max) | 1 | 8 |
| Steady state, worst pass (Files) | 119 ms | 8.8 ms |
| Steady state, underrun blocks after warm-up (Files, 64 stems, 10 s) | 0 | 0 |

**Where the time went (before, memory Blobs):** 130 ms of the 251 ms pass waited on window reads (128 reads, about 1 ms each, one at a time); deinterleaving the same 4.19 M frames costs 53 ms in isolation; the remaining about 68 ms is per-chunk work (32,768 reserve/commit pairs, each behind an `await`). After the change the reads overlap with deinterleave, so the cold pass is bound by main-thread work (deinterleave plus commits), not by reads. The steady-state worst pass was a 64-read window wave (all stems cross a window boundary together); read-ahead removes it.

**Not reproduced:** on this machine the sequential pump also kept 64 stems fed (0 underrun blocks): the 1.37 s ring depth absorbs a 119 ms wave. The console's 2,800 underrun blocks in 6 s therefore are not explained by read serialization alone on a fast local disk; slower reads (the user's Mac, a cold disk) or a busy console main thread lengthen the wave, and this change removes it. See the open question in the hand-back.

**Decisions:**
- D1. Reads run concurrently over all sources, at most `MAX_READS_IN_FLIGHT = 8`, the rest queued FIFO. Why 8: a 64-stem window wave costs ceil(64/8) = 8 read latencies instead of 64; the bound keeps the browser's blob/file I/O queue small and caps read buffers being filled at once at 8 windows (1.5 MiB at the console's window). The bound was set by this reasoning, not tuned on the harness.
- D2. Read-ahead: when a window is installed, the pump reads the next one, starting at the first chunk the resident window cannot hold whole (chunks step by the ring's frame capacity, so the prediction is exact). Memory per source: at most two windows (resident and ahead); a read a seek superseded may still finish, adding at most 8 windows briefly.
- D3. Each ring write (reserve, deinterleave, commit) is synchronous, with no await between reserve and commit, so each ring's order is exact even when two passes overlap. A read is tagged with its start frame and generation; a seek clears the read ahead, and a resolved read is installed only if its generation is the pump's and it is still the source's read ahead. The window's own start frame (not the cursor) positions it. The sequential pump positioned a late window at the cursor, so a seek during its read committed pre-seek PCM at the post-seek frame (red below); the console avoided it only with its own lock.
- D4. A read ahead that fails while no pass awaits it is caught; the pass that needs the window rethrows, stops the pump and calls `onError`, as before. API, types and SDK declaration unchanged (doc comments only where they said "one window").

**Tests** (`hosts/host-web/tests/stem-pump-v1.mjs`): `windowReadsOverlapAcrossSources` (24 sources, 40 ms reads: the cold pass takes about ceil(24/8) x 40 ms, at most 8 reads in flight, and the next pass after a drain writes read-ahead windows without waiting); `seekDuringReadCommitsNoStaleWindow` (a seek lands while the first window read is in flight; every chunk is generation 2 at its own frame with its own samples).

**Red evidence:** on the sequential pump (`HEAD~1`) the concurrency test fails "reads overlap up to the bound" (actual 1, expected 8) and the seek test fails "a seek during a window read committed the pre-seek window" (frames 0-3 at frame 4). Mutants of the new pump: install a superseded read at the cursor without the guard: seek test red; `MAX_READS_IN_FLIGHT = 1`: concurrency test red; no read-ahead: "a pass over read-ahead windows waited 120 ms" red; drop only the generation/ahead guard: green, because the start-frame tag alone keeps content exact (the guard is a resource guard: it drops a superseded window instead of holding it).
