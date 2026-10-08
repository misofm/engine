# Cache design bounds across browser preparations in the control Worker

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-07 by the decision-15 root coordinator's split of *Cache design bounds across
preparations within a stated preparation budget* (#1457, Amendment 1), and aligned on 2026-10-07
with #1457's Amendment 2 (root rulings: the cache's owners, one entry point, the budget figure,
and the C ABI successor #1471). Ordered after #1457 in stream G and after stream H's *Run the browser control plane in a Worker and keep the AudioWorklet
render-only* (#1332). Verify every code anchor on the branch where both have landed before starting.

## Product outcome

A browser rebuild of an unchanged session computes no input-section design bound: the browser's
long-lived control Worker keeps the same design-bound cache that #1457 gives each C ABI engine, so
an audio-only browser boot or rebuild costs no more than the pre-#1329 boot once its designs are
cached. The browser follows the same ownership as the C ABI (#1457 Amendment 2, ruling (b)).

## Context

- **What a design bound is.** `builtins::input_section_bounds(rate, strips, cache)` computes, once per
  distinct design, the certified tail and exact-rest bound of a strip's input section (trim,
  polarity, HPF, LPF), keyed by `InputBoundKey` (the rate, both channels' section words and the trim
  magnitudes, `crates/builtins/src/tail.rs`). A strip with a live input lane reports the rate's
  live bound instead. The cost is `O(T)` frames walked per design: about 0.2-0.3 ms for typical
  designs and 7-8.8 ms for a design near the top of the cutoff domain (x86-64-v3, release).
- **What #1457 ships** (`crates/builtins/src/tail.rs`, `crates/builtins/src/lib.rs`; both hosts
  unless noted).
  - The per-preparation budget `INPUT_BOUND_BUDGET_FRAMES` = **1,510,000 frames walked**, charged
    in strip order, in frame-equivalents of 17.0 ns (the smallest half nanosecond that bounds every
    calibration batch's and gate-2 design's median work, of five measured samples, by its charged
    frame-equivalents, root's second ruling and ruling (c) of 2026-10-08), so 25.67 ms (#1457
    Amendment 4, restated by #1474 and #1465). The stated worst case is **24.60 ms** of
    design-bound work (median of five) measured on the CI-class runner (4,096 cheap two-section
    designs, 1 kHz into 1.28 kHz, +24 dB, 88.2 kHz), 95.8 % of the 25.67 ms budget; medians over
    the families and rates: the cheap families 17.80-24.60 ms, typical 11.08-22.88 ms, near-top
    20.64-21.22 ms, the band families 20.20-20.45 ms, the single top designs 3.71-7.33 ms (#1465's
    attempt 1 follow-up 2 record, which also states the largest raw sample). Past the
    budget a design reports its rate's live bound, which is certified for every history and is
    never `Infinite`. A design served from a cache charges the same stored amount it charged when
    computed, so every reported bound is a pure function of the session, with or without a cache,
    cold or warm.
  - The live bound as a table for the four launch rates, checked by a test against
    `input_section_live_bound`, so no preparation computes it.
  - The cache type `InputBoundCache` (keyed by `InputBoundKey`, which carries the rate), with the
    entry cap `INPUT_BOUND_CACHE_ENTRIES` = 8,192 and clear-on-full: the cap holds a whole
    preparation's designs, so a rebuild of an unchanged session is served entirely from it while
    the cache has not been cleared since that session's designs were inserted (#1457 Amendment 3,
    m3: a cache that several sessions fill can clear in the middle of a preparation). A full cache
    holds about 2.4 MiB (2,486,520 bytes measured at 8,192 entries). Render never reads it.
  - *(#1457 Amendments 3 and 4.)* The budget is in frame-equivalents: each design computed
    charges the frames it walks plus `INPUT_BOUND_SECTION_CHARGE` (550 frame-equivalents) per
    section. Amendment 4 removed the per-design charge.
  - **One entry point, no parallel API** (Amendment 2, ruling (i)):
    `builtins_compiler::prepare_session_builtins_with_live_controls` takes
    `bound_cache: Option<&mut InputBoundCache>`. In `host-core` the parameter is internal to
    `prepare.rs`'s policy function (`prepare_host_runtime_with_live_controls_policy_and_spectrum`);
    every public host-core entry point passes `None`. There is no `_and_bound_cache` variant at
    any layer.
- **The C ABI is #1471** (*Wire the engine and session design-bound caches into the C ABI*, after
  #1457 and stream B's #1309). Ruling (b): the engine's cache serves
  `miso_engine_v1_compile_session`, and each session owns its own cache, seeded at compile with a
  copy of the engine's cache after that compile, for its rebuilds. No mutable state is shared
  between an engine and its sessions. #1471 adds the parameter once to the two host-core entry
  points the C ABI uses (`prepare_host_runtime_with_live_lanes` and
  `prepare_host_runtime_with_live_lanes_successor`).
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

## Decisions

Frozen by #1457 Amendment 2 (root, 2026-10-07):

- **D0. Ownership (ruling (b)).** The Worker's long-lived object that compiles browser sessions owns
  one `InputBoundCache`, and a compile reads and fills it. Each browser session owns its own
  `InputBoundCache`, seeded at compile with a copy of the compile cache after that compile, and
  every rebuild of that session reads and fills it. No mutable cache state is shared between the
  compiling object and its sessions, or between sessions. Both caches live on the control Worker
  only; render, the AudioWorklet and render-owned memory carry none of it.
- **D0a. One entry point (ruling (i)).** `bound_cache: Option<&mut InputBoundCache>` is added once to
  each host-core entry point the Worker's preparation calls (on today's tree the browser host calls
  `prepare_host_runtime_with_selected_meters_between_render_calls` and
  `prepare_host_runtime_with_live_controls_and_spectrum`; re-read them where #1332 has landed),
  passed through to the policy function; every other caller passes `None`. No `_and_bound_cache`
  variant. If #1471 has already added the parameter to an entry point this slice needs, it is used
  as is.
  *(#1457 Amendment 3, attempt-1 verdict n5.)* On #1457's tree the policy function passes
  `bound_cache` to `builtins-compiler` only on its default branch: it drops the cache on the
  selected-meters branch (`prepare_selected_session_builtins_between_render_calls`) and on the
  between-render-calls branch (`prepare_session_builtins_between_render_calls`). The browser
  prepares through `prepare_host_runtime_with_selected_meters_between_render_calls`, which takes
  one of those branches, so this slice must carry the cache through the branch the Worker's
  preparation takes (the two `builtins-compiler` functions of those branches gain the parameter,
  or route through `prepare_session_builtins_with_live_controls`), and gate 1 is red until it
  does.
- **D0b. Budget and cap.** The budget is #1457's `INPUT_BOUND_BUDGET_FRAMES` (1,510,000 frames) and
  the cap and eviction are `INPUT_BOUND_CACHE_ENTRIES` (8,192) with clear-on-full, unchanged.

To decide in this slice (root approves before implementation):

- **D1. Which Worker objects own the two caches** under D0, given #1332's Worker shape, and the
  compile cache's lifetime across session replacements.
- **D2. A browser-specific cap or eviction** only if a measured reason differs from D0b (for
  example the Worker's memory budget); otherwise none.

## Objective gates

1. A browser rebuild of an unchanged session in the Worker computes no design bound (counted with
   `builtins`' test-support design-bound counter).
2. The audio-only rebuild-cost proxy (`rebuild-round` with `COMMAND_QUEUE_RECORDS = 0`) is recorded
   on the 9-track and 64-track documents beside the baseline above; a rebuild of an unchanged
   session adds nothing over the pre-#1329 boot.
3. Every reported bound is bit-identical with and without the cache, cold and warm, and the budget
   fallback is taken exactly when `INPUT_BOUND_BUDGET_FRAMES` is exhausted, as below the C ABI
   (#1457's gates stay green).
4. Render never reads the cache; the worklet chain and the browser memory pins move only where this
   slice's retained cache is charged, each re-pin stated with its reason.

## Non-goals

- The budget, the live-bound table and the cache type (#1457). The C ABI caches (#1471). A
  parallel `_and_bound_cache` entry point. Tightening any bound. Any render-side change.

## Dependencies

- #1457 (stream G), #1332 (stream H). Sibling: #1471 (the C ABI caches); if both add the
  parameter to the same host-core entry point, the later slice rebases.

## Attempt record

- 2026-10-07 (stream G2 batch follow-ups, root order): this local spec was aligned with #1457
  Amendment 2 (ownership ruling (b), one entry point, the 1,510,000-frame budget constant, the
  cache cap, the C ABI successor #1471). **The GitHub issue body is a pending sync**: it still
  states the Amendment 1 text and was not edited here; root syncs it.
