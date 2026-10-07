# Cache design bounds across browser preparations in the control Worker

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-07 by the decision-15 root coordinator's split of *Cache design bounds across
preparations within a stated preparation budget* (#1457, Amendment 1). Ordered after #1457 in
stream G and after stream H's *Run the browser control plane in a Worker and keep the AudioWorklet
render-only* (#1332). Verify every code anchor on the branch where both have landed before starting.

## Product outcome

A browser rebuild of an unchanged session computes no input-section design bound: the browser's
long-lived control Worker keeps the same design-bound cache that #1457 gives each C ABI engine, so
an audio-only browser boot or rebuild costs no more than the pre-#1329 boot once its designs are
cached.

## Context

- **What a design bound is.** `builtins::input_section_bounds(rate, strips)` computes, once per
  distinct design, the certified tail and exact-rest bound of a strip's input section (trim,
  polarity, HPF, LPF), keyed by `InputBoundKey` (the rate, both channels' section words and the trim
  magnitudes, `crates/builtins/src/tail.rs`). A strip with a live input lane reports the rate's
  live bound instead. The cost is `O(T)` frames walked per design: about 0.2-0.3 ms for typical
  designs and 7-8.8 ms for a design near the top of the cutoff domain (x86-64-v3, release).
- **What #1457 ships (both hosts unless noted).**
  - A per-preparation budget in frames walked (D1 of #1457; its spec states the frames and the
    measured ms), charged in strip order. Past the budget a design reports its rate's live bound,
    which is certified for every history and is never `Infinite`. A design served from a cache
    charges the same stored amount it charged when computed, so every reported bound is a pure
    function of the session, with or without a cache.
  - The live bound as a table for the four launch rates, checked by a test against
    `input_section_live_bound`, so no preparation computes it.
  - On the C ABI only: a control-side cache per engine handle, keyed by `InputBoundKey` and rate,
    with an entry cap. Its gate 1 (an unchanged rebuild computes no design bound, counted) and gate 4
    (the rebuild-cost proxy) are stated for the C ABI and native scope.
- **Why the browser was split off.** The AudioWorklet creates a new `WebAssembly.Instance` per
  processor (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, instance creation in the
  processor constructor), so no browser object lives long enough to own a cache across
  preparations today. #1332 moves browser preparation into a long-lived control Worker; that Worker
  is the natural owner.
- **The baseline to beat** (#1457, root ruling R2; `scripts/web-mixing-automation-benchmark.mjs
  rebuild-round`, Node `--no-liftoff`, `taskset -c 7`, 25 boots per document, p50 ms, two rounds,
  audio-only rows with `COMMAND_QUEUE_RECORDS = 0`, the SDK default): pre-#1329 9-track EQ
  2.01 / 2.00, 64-track console 19.13 / 19.13; #1329 attempt 5 9-track 2.24 / 2.24, 64-track
  console 34.66 / 34.68. Re-measure the baseline on the branch where #1332 has landed, because
  the Worker changes the boot path.

## Decisions to make in this slice (root approves before implementation)

- **D1. Where the cache lives in the Worker** (the module instance the Worker keeps, or the Worker's
  control-plane object), and its lifetime across session replacements.
- **D2. Its entry cap and eviction**, the same as #1457's C ABI cache unless a measured reason
  differs.

## Objective gates

1. A browser rebuild of an unchanged session in the Worker computes no design bound (counted with
   `builtins`' test-support design-bound counter).
2. The audio-only rebuild-cost proxy (`rebuild-round` with `COMMAND_QUEUE_RECORDS = 0`) is recorded
   on the 9-track and 64-track documents beside the baseline above; a rebuild of an unchanged
   session adds nothing over the pre-#1329 boot.
3. Every reported bound is bit-identical with and without the cache, and the budget fallback is
   taken exactly when #1457's budget is exhausted, as on the C ABI (#1457's gates stay green).
4. Render never reads the cache; the worklet chain and the browser memory pins move only where this
   slice's retained cache is charged, each re-pin stated with its reason.

## Non-goals

- The budget, the live-bound table and the C ABI cache (#1457). Tightening any bound. Any render-side
  change.

## Dependencies

- #1457 (stream G), #1332 (stream H).

## Attempt record
