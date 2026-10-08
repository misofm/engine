# Cache design bounds across preparations within a stated preparation budget

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling R7 on *State a bounded tail and an
exact-rest bound for every node* (#1329, Amendment 4). Code anchors are those #1329 creates;
verify them on the branch where #1329 has landed before starting.

## Product outcome

Preparing a plan never spends more than a stated control-thread budget on tail and exact-rest
bounds, however many strips it has and however often it is rebuilt. #1329 computes each distinct
input-section design's bound once per preparation; every rebuild (a structural edit, a swap)
computes them all again. After this slice a design's bound is computed at most once per engine
(or per stated cache lifetime), and the worst-case preparation cost of the bounds is a stated,
gated figure.

## Context

- **What #1329 ships.** `builtins::input_section_bounds(rate, strips)` computes each distinct
  design's bound (`InputSectionBound`: `T_decay`, `T_rest`, `RestSamples`) once per call, keyed by
  the rate, both channels' section words and the trim magnitudes (`crates/builtins/src/tail.rs`,
  `InputBoundKey`). `builtins-compiler` calls it once per preparation, before the phase-two
  allocation account, keeps each strip's bound beside its tail in `PreparedBuiltinsSession`, and the
  seal check reuses those values (#1329 Amendment 4, R5 and R7). A live input lane's bound depends
  only on the rate and is computed once per preparation (`input_section_live_bound`).
- **The cost is `O(T)` per design** (the bound walks the decay frame by frame to its tail), so a
  design near the top of the cutoff domain costs far more than a typical one. Measured on #1329
  attempt 4 (release, x86-64-v3, one thread, best of three):
  - typical designs: 20 Hz HPF into 20 kHz LPF 0.19-0.34 ms, a 1 kHz LPF 0.01-0.02 ms; a session
    of 1,024 strips sharing one design 0.35-0.49 ms;
  - near the top of the domain: LPF at the maximum, +24 dB, 3.5 ms; HPF one `f32` below the
    maximum into the LPF at the maximum, +24 dB, 7.0 ms; 64 distinct near-top designs (HPF
    10-640 Hz into the LPF 1-64 `f32` below the maximum, +24 dB) 529-561 ms, 8.3-8.8 ms each;
  - the live bound 0.04 ms.
  So the worst case grows with the number of distinct designs, which only the track count bounds:
  at 8.8 ms per design, a 65,537-strip session of distinct near-top designs would spend about
  9.6 minutes (extrapolated, not run), on every rebuild.
- **Where preparation runs.** On the C ABI, on the caller's control thread. In the browser it runs
  today on the AudioWorklet thread at processor construction; stream H's *Run the browser control
  plane in a Worker and keep the AudioWorklet render-only* (#1332) moves browser preparation off the
  AudioWorklet thread.
  ~~*(Amended by #1329 Amendment 5, root ruling A.)* Preparation computes a design bound only for a
  strip without a live input lane: a live strip reports the live bound, computed once per
  preparation (0.04 ms). The browser prepares every strip with a live input lane
  (`HostLiveLanes::ALL`), so it computes no design bound at all; its boot cost is back to the
  pre-#1329 figures (#1329 attempt 5's `rebuild-round` record). The worst case above therefore
  lands on the C ABI's control thread, which prepares strips without a live input lane
  (`HostLiveLanes::FADER_AND_MATRIX` plus effect lanes); that is the platform this issue's budget
  binds.~~ *(Superseded by root ruling R2 below: its premise was false for an audio-only browser
  boot.)*
  *(Root ruling R2.)* Preparation computes a design bound only for a strip without a live input
  lane; a live strip reports the live bound, computed once per preparation (0.04 ms native).
  `HostLiveLanes::ALL` attaches an input lane only to a strip that has a control request
  (`crates/host-core/src/prepare.rs`), and the browser makes control requests only when its live
  control command queue is non-zero (`hosts/host-web/src/lib.rs`). So:
  - a browser boot **with live controls** bounds no design: every strip reports the live bound;
  - an **audio-only** browser boot, the SDK default (`liveControls?.commandQueueRecords ?? 0` in
    `sdk/src/core/abi.ts`), attaches no input lane and bounds every distinct design, on the
    AudioWorklet thread today and on a Worker after #1332;
  - the C ABI bounds every distinct design on its caller's control thread.
  This issue's budget binds **both hosts**.

## Decisions to make in this slice (root approves before implementation)

- **D1. The budget.** A stated worst-case control-thread cost per preparation (for example per
  distinct design and per session), measured on the CI runner, with the platform it binds.
  *(Root ruling R2:)* it binds both hosts: the C ABI's control thread and the browser's
  preparation thread (the AudioWorklet thread until #1332, a Worker after it), for an audio-only
  browser boot.
- **D2. The cache.** Where it lives (engine instance, control-plane crate), its key (#1329's design
  key plus the flush law, which depends only on the rate), its bound on entries and memory, and its
  eviction. The cache is control-side only; render never reads it (#1329 R5).
- **D3. What happens past the budget.** A session whose distinct designs exceed it either is
  refused with a typed diagnostic, or prepares with a cheaper, still certified, looser bound. No
  `Infinite` fallback for a design #1329 can bound.

## Objective gates (to be fixed with D1-D3)

1. A rebuild of an unchanged session computes no design bound (counted).
2. **The worst case, not only typical sessions** (root ruling R1 below). The preparation cost of
   the bounds is measured once on the CI runner for the worst case #1329 attempt 4 recorded, and
   it is within D1's budget: distinct near-top designs (HPF one `f32` below the maximum into the
   LPF at the maximum, +24 dB; and the 64-design family HPF 10-640 Hz into the LPF 1-64 `f32`
   below the maximum, +24 dB) at 7-8.8 ms each, up to the track-count limit (65,537 distinct
   near-top designs, about 9.6 minutes extrapolated on every rebuild today), on a first
   preparation and on a rebuild. A typical session's figure is recorded beside it but never
   stands in for it. Under D3, a session past the budget shows the D3 outcome (typed refusal or
   the cheaper certified bound) at the measured worst case.
3. Every reported bound is bit-identical with and without the cache (`tail_contract`'s gates stay
   green).
4. *(Root ruling R2.)* **The audio-only browser boot.** The repository's rebuild-cost proxy
   (`scripts/web-mixing-automation-benchmark.mjs rebuild-round`) run with no live controls
   (`COMMAND_QUEUE_RECORDS = 0`, the SDK default) is recorded on the 9-track and 64-track
   documents beside R2's baseline table, and its design-bound cost is within D1's budget; a
   rebuild of an unchanged session adds nothing over the pre-#1329 boot (gate 1).

## Root rulings

### R1 (2026-10-06): the worst case is gated

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation. The gates hold
the measured worst case (#1329 attempt 4: 7-8.8 ms per distinct near-top design; about 9.6 minutes
extrapolated for 65,537 distinct near-top designs per rebuild) against D1's stated budget, not only
typical sessions. Gate 2 is amended to say so.

### R2 (2026-10-06): the budget binds both hosts, including the audio-only browser boot

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), correcting #1329 Amendment 5's root ruling A after #1329
attempt 5's verdict (`/home/bl/misofm/submix-verdicts/1329-attempt5.md`, m1). Ruling A's premise
("in the browser every strip is live") was wrong. A browser boot with live controls attaches an
input lane to every strip and bounds no design; an audio-only browser boot (the SDK default)
bounds every distinct design on the AudioWorklet thread until #1332 moves preparation to a Worker;
the C ABI bounds every distinct design on its control thread. D1's budget binds both hosts, and
gate 4 adds the audio-only browser boot. "Where preparation runs" is restated above.

The verifier's measurement (one `rebuild-round` invocation, two rounds; Node v22.23.2,
`--no-liftoff`, `taskset -c 7`, 25 boots per document, every boot audible; the audio-only rows use
a copy of the script with `COMMAND_QUEUE_RECORDS = 0`). Boot p50 in ms, round 1 / round 2:

| boot | module | 9-track EQ | 64-track console | 64-track app shape | 64-track sends |
|---|---|---|---|---|---|
| live controls | pre-#1329 `10a3c816…` | 2.15 / 2.16 | 20.31 / 20.36 | 19.75 / 19.92 | 33.86 / 34.04 |
| live controls | #1329 attempt 5 `ec1d2f66…` | 2.21 / 2.25 | 20.20 / 20.48 | 19.69 / 19.88 | 33.60 / 34.18 |
| live controls | attempt 5 with the 48 kHz live bound as a constant (measurement only, `ad09489c…`) | 2.15 / 2.13 | 20.11 / 20.19 | 19.62 / 19.71 | 34.14 / 34.00 |
| **audio-only** | pre-#1329 `10a3c816…` | 2.01 / 2.00 | 19.13 / 19.13 | 18.63 / 18.57 | 32.60 / 31.26 |
| **audio-only** | #1329 attempt 5 `ec1d2f66…` | **2.24 / 2.24** | **34.66 / 34.68** | **34.17 / 34.23** | **46.61 / 46.61** |

So an audio-only 64-track browser boot costs about +15.5 ms (+81 %; console 19.13 -> 34.66 ms p50)
today. These design bounds are not waste: they are the bounds the non-live strips report and graph
lowering reads (#1329 D4, D7). On the SDK default path the browser can hold the AudioWorklet thread
for the full worst case above (about 0.7 s estimated for 64 distinct near-top designs) until #1332
lands.

**The live bound's own cost (verdict NIT).** The live bound costs about 0.07 ms per live-control
browser boot: the 9-track document boots in 2.21 / 2.25 ms against 2.15 / 2.16 ms pre-#1329, and a
variant that returns the 48 kHz live bound as a constant boots like pre-#1329 (2.15 / 2.13 ms;
module size 3,053,689 B against 3,053,802 B, so size is not the cause). Every live-control boot
pays it; at 64 tracks it is inside the noise. The work is not waste (every live strip reports that
bound), but it depends only on the rate, of which there are four launch rates, so this issue's
cache, or a per-rate table checked by a test, removes it.

### Context from #1433 (2026-10-06): the live bound now costs more

*Tighten the cascade exact-rest bound with a frequency-aware cascade analysis* (#1433) replaced
#1329's crude live cascade step, so the live bound's cost above (0.04 ms native, about 0.07 ms per
live-control browser boot) is out of date. From #1433's record and verdict
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-g/1433-attempt1.md`, "Cost and memory"):

- **Native.** `input_section_live_bound` costs about 42 us before #1433 and about 240-254 us
  after, per preparation with a live input lane (release, one pinned core, best of 50; the
  verifier measured 236.5 / 241.6 / 245.8 / 249.5 us at 44.1 / 48 / 88.2 / 96 kHz). About 85 us
  is the zones; the settled zone groups take the rest.
- **Browser boot with live controls** (`rebuild-round`, p50, two rounds): +0.25 to +0.4 ms per
  boot. The 9-track EQ document boots in 2.22 / 2.22 ms before and 2.46 / 2.48 ms after; the
  64-track console in 20.28 / 20.21 ms before and 20.55 / 20.62 ms after. An audio-only boot
  computes no live bound.
- **Memory.** The bound's peak allocation (one power table of at most 26 `4x4` entries per zone
  group, dropped after each group) is what the browser fixture's exact `memoryBytes` pin observes.

The live bound still depends only on the rate (four launch rates), so this issue's cache, or a
per-rate table checked by a test, removes its whole cost from every preparation after the first.
D1's budget states it beside the design bounds.

## Non-goals

- Tightening any bound (#1433). Any render-side change. Effects' bounds (#1372-#1376).

## Dependencies

- #1329. Coordinate with #1332 (stream H) for the browser's preparation thread.
- Successors: #1470 (browser cache), #1471 (C ABI caches), #1474 (the near-top walk's
  data-dependent cost, Amendment 4).

## Amendment 1 (root rulings, 2026-10-07): D1-D3, the split and restated gates

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), after a first worker stopped for D1-D3. These rulings replace
"Decisions to make in this slice" above.

- **D3 (accepted): past the budget, the live bound.** A design whose bound would exceed the
  preparation's remaining budget reports its rate's **live bound**
  (`builtins::input_section_live_bound`). That bound is certified for every history of trim,
  polarity and filter targets (#1329 D5), so it holds for any fixed design, and it is never
  `Infinite` at a launch rate.
  - The trigger is deterministic: a per-preparation budget in **frames walked**, enforced by a
    horizon cap in `math::tail::fixed_cascade`, charged in **strip order**.
  - A cache entry stores its charge (the frames its computation walked). A cache hit charges the
    same stored amount, so every reported bound, and every fallback decision, is a pure function of
    the session: gate 3 holds (identical bits with and without the cache, and with a cold or a warm
    cache).
- **D1: the budget.** The frames-walked total that the CI runner covers in **20 ms** of design-bound
  work per preparation. The spec states both numbers, the frames and the measured ms, once measured
  (attempt record; frozen workload, one invocation, one warmup, two measured rounds). Reason:
  preparation stays interactive for agent edits; typical sessions (about 145 us per design) bound
  every distinct design exactly; near-top designs (7-8.8 ms each) beyond the first few take the
  certified live bound. The budget binds both hosts (R2): it is enforced inside the shared
  preparation (`builtins-compiler`), so it caps the C ABI's control thread and the browser's
  preparation thread alike. #1464-#1468 rerun gates 2 and 8 against this stated budget
  *(corrected by Amendment 3: gate 4 moved to #1471 and #1470 in Amendment 2; the commands are
  Amendment 3's)*.
- **D2 (accepted): the cache.**
  - A control-side cache per C ABI engine handle (`Engine`), keyed by `InputBoundKey` (which already
    carries the rate), with an entry cap. Eviction is clear-on-full or LRU, whichever is simpler and
    still proven by the gates. Render never reads it, and render-owned memory carries none of it.
  - The live bound becomes a **table for the four launch rates**, checked by a test against
    `input_section_live_bound`. This removes its cost (about 0.25-0.4 ms per browser boot with live
    controls, about 240-254 us per native preparation, #1433) from every preparation.
- **Split (accepted).** The browser cache and the browser "a rebuild adds nothing" clause move to
  *Cache design bounds across browser preparations in the control Worker* (#1470), which depends
  on #1332 (its Worker owns the cache). The worklet creates a new `WebAssembly.Instance` per
  processor, so no browser object outlives a preparation today. The budget (D1, D3) and the
  live-bound table (D2) still ship here and apply to the browser.

### Gates restated by Amendment 1

Gates 1 and 4 are restated to the C ABI and native scope; their browser halves are #1470's gates 1
and 2. Gates 2 and 3 stand as written, with D1's budget and D3's outcome.

1. A rebuild of an unchanged session through the same C ABI engine computes no design bound
   (counted with `builtins`' `fixed_input_bounds_computed`).
4. The native rebuild cost: the C ABI preparation of the 9-track and 64-track documents (the C ABI
   attaches no input lane, so it bounds every distinct design) is recorded on a first preparation
   and on a rebuild of the unchanged session; its design-bound cost is within D1's budget, and the
   rebuild computes no design bound (gate 1). The browser's audio-only boot keeps the budget (D1)
   and is recorded once beside R2's table; its rebuild clause is #1470's.
5. *(New.)* The fallback is taken exactly when the budget is exhausted: a session whose distinct
   designs' charges, summed in strip order, cross the budget reports each design's exact bound up
   to the crossing and the live bound from the crossing on, with and without the cache. The gate is
   red when the charge order is not strip order or when a cache hit charges anything other than
   its stored amount.
6. *(New.)* The live-bound table equals `input_section_live_bound` at each of the four launch rates,
   bit for bit.

### Paths this slice needs (pending root: not yet named exceptions; superseded by Amendment 2)

Stream G owns none of the C ABI or host-core paths below, and neither this spec nor STREAMS names
them for #1457. The cache must reach both C ABI preparation sites: `compile_children` (from
`miso_engine_v1_compile_session`, which holds the engine) and the structural-transaction rebuild
(`prepare_runtime(.., Some(SuccessorBase))` in the session's controller), which runs on the
**session** handle, not the engine.

- `crates/capi/src/abi.rs` (`Engine`: the cache field; stream B's)
- `crates/capi/src/ffi.rs` (`miso_engine_v1_compile_session`: hand the engine's cache to
  `compile_children`; stream B's)
- `crates/capi/src/runtime/compile.rs` (`compile_children`, `prepare_runtime`: thread the cache;
  stream B's)
- `crates/capi/src/runtime/control.rs` (the successor rebuild's `prepare_runtime` call and the
  controller's handle on the cache; stream B's)
- `crates/capi/include/miso_engine_v1.h` (thread-ownership notes, if a session shares its engine's
  cache; stream B's hot file)
- a new `crates/capi/tests/` file for gates 1, 4 and 5 (stream F's column)
- `crates/host-core/src/prepare.rs` (`prepare_host_runtime_with_live_lanes{,_successor}` and the
  `prepare_session_builtins*` calls: pass the cache; stream A's hot file)
- `crates/builtins-compiler/src/lib.rs` (the design-bound call, the budget, the live-bound read;
  stream G's #1329 tail rule, but its #1379 exception is #1464's)
- `crates/builtins/src/lib.rs` (`input_section_bounds`: cache and budget) and
  `crates/builtins/src/tail.rs` (the live-bound table, the charged design bound) (stream A's; the
  hot-file row orders #1457 before #1464)
- `crates/math/src/tail.rs` (`fixed_cascade`'s horizon cap; hot-file row with G's #1464-#1468)
- `crates/builtins/tests/tail_contract.rs` (gates 3, 5, 6)

**Design question for root (D2).** The header lets the engine and each session be driven by
different control threads, and does not require a session to be destroyed before its engine. A
cache "per engine" that also serves the session's rebuilds is therefore shared across threads and
lifetimes (for example `Arc<Mutex<..>>` held by the engine and by each session it compiled), and
the header's thread notes change. The alternatives are one cache per engine for
`compile_session` plus one per session for its rebuilds (no sharing, no header change; gate 1's
"rebuild of an unchanged session" is then a session transaction), or a cache owned only by the
session. D3's stored-charge rule keeps every result a pure function of the session under any of
them, including concurrent use.

## Amendment 2 (root rulings, 2026-10-07): the cache's owners, the C ABI split, D1's figure

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation, after attempt
1 stopped for paths and the D2 owner question. These rulings replace Amendment 1's D1 and D2 where
they differ, and its "Paths this slice needs".

- **D2 owners (ruling (b)).** The engine's cache serves `miso_engine_v1_compile_session`. Each
  session owns its own cache, seeded at compile, for its rebuilds. No mutable state is shared
  between an engine and its sessions, and the header does not change.
- **Split: the C ABI wiring is #1471.** Stream B's batch 1 (#1309) moves
  `crates/capi/src/runtime/{control,compile}.rs` into a new `control-plane` crate, so this slice
  does not edit `crates/capi`. *Wire the engine and session design-bound caches into the C ABI*
  (#1471, after #1309 is on `main`, built on `control-plane`) owns the engine and session caches,
  gates 1 and 4 below, and stream B's and F's paths.
- **No parallel API (ruling (i)).** No `_and_bound_cache` public variant at any layer.
  `builtins_compiler::prepare_session_builtins_with_live_controls` itself gains
  `bound_cache: Option<&mut InputBoundCache>`. In `host-core` the parameter stays internal to
  `prepare.rs` (its policy function, `prepare_host_runtime_with_live_controls_policy_and_spectrum`);
  every public host-core entry point passes `None`, and #1471 adds the parameter once to the two
  entry points the C ABI uses. The native gate 3 calls the policy function directly.
- **D1 (choice (B)).** The budget is **1,510,000 frames walked**, 20 ms of near-top design work on
  the CI-class runner. Its purpose decides: typical sessions bound every distinct design exactly.
  Both facts are stated: about 20 ms for near-top-heavy work, and a stated worst case of about
  **32 ms** for a session of many cheap distinct designs, both measured on the CI-class runner in
  one invocation (attempt record). The O(strips) design-keying cost is reported beside it.
  *(Superseded by Amendment 3, MJ1: the 32 ms worst case was wrong by about 2x, because the
  frames-only charge left each design's fixed cost uncharged; the budget is now in
  frame-equivalents and the worst case is restated in attempt 2's record.)*
- **Named exceptions for this slice:** `crates/host-core/src/prepare.rs`,
  `crates/builtins/src/lib.rs`, `crates/builtins/src/tail.rs`,
  `crates/builtins-compiler/src/lib.rs`, `crates/math/src/tail.rs`,
  `crates/builtins/tests/tail_contract.rs`; and, only to add `None` at callers of
  `prepare_session_builtins_with_live_controls`: the tests in `crates/graph-compiler/src/lib.rs`,
  `crates/builtins-compiler/tests/allocation_tracker.rs`, `hosts/host-web/src/tests.rs` and
  `crates/host-core/tests/prepare.rs` (the last two name it only in comments; no edit was needed).

### Gates as of Amendment 2

1. *(Moved to #1471, gate 1.)* A rebuild of an unchanged session through the C ABI computes no
   design bound. This slice proves the mechanism below the C ABI (gate 7).
2. The worst case is measured once (release, CI-class runner, one invocation, one warmup, two
   rounds) at the four launch rates: the top pair, the 64-design near-top family, 65,537 distinct
   near-top designs, typical designs and cheap designs, on a first preparation and on a rebuild
   with a warm cache. Design-bound walking stays within the budget; the D3 outcome shows past it.
   *(Amendment 3: the workload is committed as `crates/builtins/examples/input_bound_budget.rs`,
   with the cheap two-section and two-cascade families added; its command is Amendment 3's.)*
3. Every reported bound is bit-identical with and without the cache, cold and warm, at the bound
   level (`tail_contract`) and at the host level (the policy function's report).
4. *(Moved: the native C ABI rebuild cost is #1471's gate 4; the browser's audio-only rebuild
   clause is #1470's gate 2.)*
5. The fallback is taken exactly when the budget is exhausted, red on a wrong charge order or a
   wrong cache-hit charge.
6. The live-bound table equals `input_section_live_bound` at the four launch rates, bit for bit.
7. *(New.)* A preparation walks at most the budget, and a second preparation with the same cache
   walks no frame and computes no bound, the stopped design included.
8. *(New, D1's purpose.)* Every 64-track console document (`console-sixty-four-track.json`,
   `console-sixty-four-track-app.json`, `console-sixty-four-track-intended.json`,
   `console-sixty-four-track-mono.json`, `console-sixty-four-track-sends.json`) prepares with no
   strip on the live bound at every launch rate, its walk inside the budget. Red when the budget or
   the walk changes so that the purpose breaks (proved with a smaller budget).

## Amendment 3 (root rulings, 2026-10-07): the charged budget, the stopped walk, the gate commands

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation, after attempt
1's verdict (`/home/bl/misofm/submix-verdicts/1457-attempt1.md`: FAIL on MJ1 and MJ2).

- **MJ1, option (b): charge each design's fixed cost.** Each design, and each section, is charged a
  calibrated constant in frame-equivalents on top of the frames walked, so the budget measures the
  real work and the stated ms is a true bound. The constants are calibrated from the measured fixed
  cost (about 14 us per design in the verdict), per design and per section separately. Gate 2 is
  re-measured across all design families, including the two-section families of 513- and 769-frame
  walks that ran 58.5-63.7 ms in attempt 1; the worst case is stated as the measured maximum across
  families, with its spread. Gate 8 must still show every 64-track console document exact at every
  launch rate; if the new charge breaks it, the slice stops and reports with numbers, and the budget
  constant (1,510,000) does not change without a ruling. A frame-equivalent is one near-top frame
  on the CI-class runner (1 / 75,500 ms, D1's reference); this workstation (x86-64-v3, release, one
  pinned core) stands in for that runner, and both the code and this spec say so.
  *(Superseded by Amendment 4: a frame-equivalent is one frame of the slowest frame class, 23.5 ns;
  the per-design charge is removed.)*
- **MJ2: a stopped walk reports the frames it walked.** `math::tail::fixed_cascade_within` reports
  the frames actually walked, finished or stopped, and the test-support counter adds those, not the
  horizon. A test asserts `walked <= budget`, and it is shown red on a majorant pass that counts
  frames but does not stop (the verifier's X3).
- **Gate-2 command (m4).** The gate-2 workload is committed and runnable, release, one invocation,
  one warmup and two measured rounds, descriptive:
  `cargo build --locked --release -p builtins --features test-support --example input_bound_budget`,
  then `taskset -c <core> target/release/examples/input_bound_budget` (gate 2) and
  `taskset -c <core> target/release/examples/input_bound_budget calibrate` (the charge constants).
  Gate 8's command is `cargo test --locked -p builtins-compiler --features test-support --lib
  every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`. Successors
  (#1465, #1466, #1467, #1468) rerun "#1457's gates 2 and 8" with these commands, and a successor
  that changes the walk's cost reruns the calibration and restates the constants.
- **m3.** The cache-cap doc says a rebuild is served entirely from the cache only while the cache
  has not been cleared. #1471's gate 1 covers an engine cache near its cap and records the memory of
  each per-session copy at 8,192 entries.
- **m5, ruling (i).** `input_section_bounds_within` (a caller-chosen budget) is reachable only
  behind `test-support` (`builtins::test_support::input_section_bounds_within`): no public, ungated
  entry point takes an arbitrary budget.
- **n5.** #1470's spec records that the host-core policy function drops the cache on the
  selected-meters and between-render-calls branches, which is the browser's path.
- Also folded in: m1 (the host-core test's test value is X1), m2 (the stale docs at
  `builtins-compiler`'s `tails` field and `builtins/src/tail.rs`'s module doc) and n1-n4.

### Gates as of Amendment 3

Gates 2, 3, 5, 6 and 8 stand as Amendment 2 states them; the budget is in frame-equivalents (each
design's frames walked plus `INPUT_BOUND_DESIGN_CHARGE` and `INPUT_BOUND_SECTION_CHARGE` per
section) and gate 8 holds the charge, not only the frames, inside the budget. Gate 7 is restated:

7. A preparation walks at most the budget, counted frame by frame by the walk itself, a stopped
   design included (red on X3: a pass that counts frames but does not stop at the horizon), and a
   second preparation with the same cache walks no frame and computes no bound, the stopped design
   included.

## Amendment 4 (root rulings, 2026-10-08): the frame-equivalent from the slowest frame class

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation, after attempt
2's verdict (`/home/bl/misofm/submix-verdicts/1457-attempt2.md`: FAIL on MJ1, the stated worst
case 22.7 ms against a measured 33.6-34.0 ms in a band of near-top designs).

- **D1, option (a): restate.** The frame-equivalent is defined from the slowest frame class: the
  near-top band where the HPF is at about 0.74-0.85 of its maximum into the LPF at its maximum,
  +24 dB, costing 21-23 ns a frame. The section charge is recalibrated in those units. The band
  family joins the committed gate-2 workload (`crates/builtins/examples/input_bound_budget.rs`).
  The worst case is stated as the measured maximum across all families (about 34 ms), with its
  spread and the per-frame cause. Gate 8 stays exact (every 64-track console document at every
  launch rate); if the new unit breaks it, the slice stops and reports with numbers. Option (b),
  lowering the budget, is refused; `INPUT_BOUND_BUDGET_FRAMES` stays 1,510,000.
- **D1, option (c): a successor.** *Remove the data-dependent cost of the near-top tail walk*
  (#1474) diagnoses the band's per-frame cost by measurement (subnormal state or error radius in
  the first section, by subnormal-operand counts or a flush-to-zero A/B), and if confirmed uses a
  certified flush in the analysis (a radius rounded up to a stated tiny bound, proven sound), then
  recalibrates and restates the worst case down. Its gates include the band family's bound
  identical or soundly larger, and the timing evidence. It is ordered in stream G after #1457 and
  before #1465, and #1465 depends on it.
- **m1: no per-design charge.** `INPUT_BOUND_DESIGN_CHARGE` and its reservation code are removed:
  the measured per-design term is negative at every rate. A future non-zero charge brings its own
  test.
- **m2.** One math-level test sweeps the horizon over a cheap design and catches the verifier's
  X3, X5, X7 and X9, each shown with a mutation run.
- **n1.** The 11-12 ms on the first preparation is allocator first-touch, not a cache cost (with
  heap trimming off, a cold cache costs the same as no cache, 39 ms). The record is corrected, and
  #1471's scope explains and measures the first-touch cost.
- **n2-n4.** The example's doc names the real constants; `tail.rs`'s long doc line is wrapped; the
  record states how many calibration runs happened and why, and that the constants come from the
  fixed-cost measurement, not from gate-2 timings.
- Root's earlier ruling stands: if a verifier rules a gate-2 figure tainted, the batch verdict's
  single invocation is the figure of record.

### Gates as of Amendment 4

Gates 2, 3, 5, 6, 7 and 8 stand as Amendment 3 states them, with these changes: the budget is in
frame-equivalents of the slowest frame class, each design charging its frames walked plus
`INPUT_BOUND_SECTION_CHARGE` per section (no per-design charge); gate 2's workload includes the
band families; and gate 7's walk is also checked at the math level, under every horizon of a cheap
design (red on X3, X5, X7 and X9).

## Attempt record

### Attempt 1 (2026-10-07): rulings recorded, successor filed, stopped for paths

- Recorded Amendment 1 (root's rulings D1-D3, the split, gates 1 and 4 restated, gates 5 and 6
  added).
- Filed the successor #1470 (*Cache design bounds across browser preparations in the control
  Worker*) with a matching local spec and a stream G row (order 22, after #1457, after H #1332).
- Stopped before code under the worker rules: the C ABI and host-core paths above are not named
  exceptions for #1457, and D2's owner of the session-side rebuild needs root's choice. No code,
  measurement or mutation run yet; D1's frames and ms are still to be measured.

### Attempt 1, continued (2026-10-07): implementation under Amendment 2

**What changed.**
- `math::tail::fixed_cascade_within(sections, gain, law, peaks, horizon) -> Option<CascadeWalk>`:
  the same walk as `fixed_cascade`, counting every frame-by-frame step (the majorant pass, the replay
  of its crossing block, the deviation walk) and stopping (`None`) when one more frame would pass
  `horizon`. `fixed_cascade` is it with `u64::MAX`, so its result is unchanged bit for bit.
- `builtins` (`tail.rs`, `lib.rs`): `ChargedInputBound { bound, frames }`;
  `INPUT_BOUND_BUDGET_FRAMES = 1_510_000`; `InputBoundCache` (keyed by `InputBoundKey`, entry cap
  `INPUT_BOUND_CACHE_ENTRIES = 8_192`, clear-on-full; an entry is a charged bound or, for a stopped
  walk, the horizon it passed, `ChargeAbove`); `input_section_bound_charged`;
  `input_section_bounds(rate, strips, cache)` = `input_section_bounds_within(rate, strips,
  INPUT_BOUND_BUDGET_FRAMES, cache)`; `input_section_live_bound_table(rate)` (the live bound at the
  four launch rates). The charge rule: each distinct design at its first strip, in strip order,
  charges its frames; a repeat charges nothing; a cache hit charges its stored frames; a stored
  `ChargeAbove(h)` under a remaining budget of at most `h` reports the live bound without a walk;
  past the budget every design with an enabled filter reports the live bound without a walk. A
  memoryless design (no filter enabled on either channel) reports `InputSectionBound::ZERO`, walks
  nothing and is neither counted nor cached. The test-support counter `fixed_input_bounds_computed`
  now counts finished computations only, and `fixed_input_frames_walked` counts frames walked.
- `builtins-compiler`: `prepare_session_builtins_with_live_controls(.., bound_cache:
  Option<&mut InputBoundCache>)`; the live strips read the table (no live-bound computation in any
  preparation).
- `host-core/src/prepare.rs`: the policy function's `bound_cache` parameter (every public entry
  passes `None`) and the native gate-3 unit test. `None` added at the callers in
  `crates/graph-compiler/src/lib.rs` (tests) and `crates/builtins-compiler/tests/allocation_tracker.rs`.
- No render path changed. The cache and the table are control-side; nothing render owns stores
  them. No allocation-counting window moved: the bounds are computed before the phase-two
  observation, as before, and the table is a `const fn` (no lazy state to warm).

**D1 measurement** (this box as the CI-class runner: x86-64-v3, release, `taskset -c 7`, one
invocation, one warmup and two measured rounds, scratch probe not committed; frames walked per ms):

| design (each rate) | frames | ms | frames/ms |
|---|---|---|---|
| top pair (HPF one `f32` below the maximum into the LPF at the maximum, +24 dB) | 525,520-528,640 | 6.42-6.83 | 77,400-81,900 |
| LPF at the maximum, +24 dB | 448,513-451,073 | 3.37-3.59 | 125,600-133,300 |
| 64-design near-top family (HPF 10-640 Hz into the LPF 1-64 `f32` below the maximum, +24 dB), all 64 | 26.78-26.89 M | 346-355 | 75,500-77,400 |
| typical (20 Hz HPF into 20 kHz LPF), 256 distinct | 2.69-5.63 M | 41.5-78.9 | 64,700-71,400 |
| cheap (1 kHz LPF), 256 distinct | 131,328-196,864 | 2.71-3.84 | 47,500-53,000 |

Budget: 1,510,000 frames = 20 ms of near-top work (75,500 frames/ms). Stated worst case for many
cheap distinct designs: about 32 ms (measured 28.6-33.4 ms below).

**Gate 2** (`input_section_bounds` with a fresh `InputBoundCache`, then the same cache warm, then no
cache; rounds 1 and 2, ms; "exact" counts strips not on the live bound):

| workload | rate | first | rebuild (warm) | no cache | exact |
|---|---|---|---|---|---|
| 65,537 distinct near-top designs | 44.1 / 48 / 88.2 / 96 kHz | 46.3-78.5 | 26.1-47.2 | 45.2-79.8 | 3 / 65,537 |
| 64-design near-top family | all four | 18.5-22.9 | 0.017-0.035 | 18.5-27.0 | 3 / 64 |
| 64 typical designs | 44.1 / 48 kHz | 10.4-11.2 | 0.019-0.021 | 10.4-11.2 | 64 / 64 |
| 64 typical designs | 88.2 / 96 kHz | 18.7-28.8 | 0.019-0.021 | 18.7-20.6 | 64 / 64 |
| 4,096 cheap designs (1 kHz LPF family) | 44.1 / 48 kHz | 32.7-33.4 | 1.64-1.74 | — | 2,943 / 4,096 |
| 4,096 cheap designs | 88.2 / 96 kHz | 28.6-29.2 | 1.80 | — | 1,963 / 4,096 |

Design-bound walking stays within the budget (gate 7 holds the frame count exactly). The rest of
the 65,537-strip figure is the O(strips) design keying, outside the walk: designing each strip's
input track and its key, about 26 ms per 65,537 strips (0.4 us per strip, the warm-rebuild column);
the first preparation adds the cache's own insertions. Before #1457 the same session cost about 9.6
minutes per preparation (#1329, extrapolated). The 48 kHz round-2 outlier (78 ms) is noise on a
shared box; the run was not repeated (benchmark rule).

The 64-track console documents walk 610,432 (44.1 kHz), 658,816 (48 kHz), 1,157,760 (88.2 kHz) and
1,254,016 (96 kHz) frames, the mono variant 316,480-650,816, all inside the budget (gate 8).

**Tests added** (test-value sentences):
- `tail_contract::the_live_bound_is_taken_exactly_when_the_budget_is_exhausted` (gates 5, 3):
  red when the charge order is not strip order, a cache hit charges other than its stored frames, a
  repeated design charges again, the budget comparison is off by one (on a hit or on the walk), or a
  stopped walk leaves budget for a later, cheaper design.
- `tail_contract::a_full_bound_cache_is_cleared_and_reports_the_same_values` (gate 3, D2): red when
  the entry cap is not enforced.
- `tail_contract::live_bound_table_is_the_computed_live_bound_at_every_launch_rate` (gate 6): red
  when a table entry differs from the computed live bound.
- `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`
  (gate 7): red when a stopped walk's horizon is not cached, or is honoured with the wrong
  comparison, so a rebuild walks again.
- `builtins_compiler::tests::every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`
  (gate 8): red when the budget or the walk's charge changes so that a typical session leaves exact
  bounds.
- `host_core::prepare::tests::a_design_bound_cache_changes_no_prepared_value` (gate 3, native): red
  when a cached design's charge differs from its computed one, so the host report with a warm cache
  differs from the one without.
- Changed: `design_bounds_are_computed_only_for_strips_without_a_live_input_lane` (6 -> 5 and
  4 -> 3: a memoryless design is no longer computed).

**Mutation runs** (each applied alone, the named test run, then reverted):

| id | mutation | test | result |
|---|---|---|---|
| M1 | a cache hit charges 0 frames | gate 5 | red (order [0,1,0,2], budget 512, cache warmed under `u64::MAX`) |
| M2 | a repeated design is charged again | gate 5 | red (budget 513, no cache) |
| M3 | a hit fits only when `frames < remaining` | gate 5 | red (budget 513, warm cache) |
| M4 | a stored horizon is honoured only when `remaining < above` | gate 5 | green; gate 7 red (warm cache walked) |
| M5 | `take_frame` stops one frame early | gate 5 | red (budget 513, no cache) |
| M6 | a stopped walk leaves the remaining budget | gate 5 | red (order [3,0,1], budget 22,784; after a fourth, cheaper design was added for it) |
| M7 | designs charged in reverse strip order | gate 5 | red (order [0,1,0,2], budget 513) |
| M8 | a stopped walk's horizon is not cached | gate 7 | red (warm cache) |
| M9 | no clear-on-full | cache-cap test | red (3 entries in a cache of 2) |
| M10 | 48 kHz table `any_sanitized_input` + 1 | gate 6 | red |
| M11 | a cache hit charges 0 frames | native gate 3 | red (warm-cache report differs) |
| M12 | budget 950,000 | gate 8 | red (88.2 kHz: 12 strips on the live bound) |
| M13 | every walked frame charged twice | gate 8 | red (88.2 kHz) |

M4 is caught by gate 7, not gate 5: under a cold or warm cache the boundary case gives the same
values, because a stopped walk charges the whole remainder either way; only the frames walked differ.

**Gates run** (on `30a355d5d`, x86-64, `CARGO_INCREMENTAL=0`): test-debug-a (the workspace step with
its exact `--exclude` and `--features` list) exit 0; test-debug-b's DSP step (lane ... conformance,
`math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`) exit 0; release
`-p lane -p math -p wasm-gates --features math/lane` exit 0; release `tail_contract` 12 passed;
`cargo clippy --workspace --all-targets --all-features -D warnings`, `cargo fmt --check`,
`cargo doc -D warnings --exclude gate-expander` clean; `check-workspace-policy.sh`,
`check-realtime-policy.sh`, `check-builtins-policy.sh`, `check-cross-targets.sh`,
`check-capi-abi.sh` ok; `target/release/audit capi`: 0 allocations, 0 syscalls, 0 violations,
`pcm_digest` `cb10fbface44a3a4` (unchanged); worklet chain (`build-web-audioworklet.sh
--named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`,
`test-web-audioworklet.sh`) ok; `run-wasm-gates.sh --without-v8-spill --without-native` ok.
AArch64 runs only in CI. `check-host-core-policy.sh` fails with "only host-core may declare and capi
may enable control-provider" on the parent commit too (not this slice's; reported to root).

### Attempt 2 (2026-10-07): the charged budget (Amendment 3)

**What changed.**
- `math::tail::fixed_cascade_within` now always returns a `CascadeWalk { result, frames }`:
  `result` is `None` for a stopped walk, and `frames` counts the frames actually walked, finished
  or stopped (MJ2). `fixed_cascade` is unchanged bit for bit. The 10-line function lost its
  needless `too_many_lines` allow (n3).
- `builtins` (`tail.rs`, `lib.rs`):
  - `ChargedInputBound` gains `charge`, in frame-equivalents: the frames walked plus
    `INPUT_BOUND_DESIGN_CHARGE` once plus `INPUT_BOUND_SECTION_CHARGE` per section walked (both
    channels' when they differ). A design reserves its design charge, and each channel cascade its
    section charges, before it walks; the walk's horizon is what is left. So the frames walked
    never pass the budget, a stopped design included, and the charge of a finished design is what
    the budget is debited, computed or from the cache.
  - The test-support counter `fixed_input_frames_walked` adds the frames really walked (MJ2); a new
    counter `fixed_input_charged` adds the charges of the bounds computed (gate 8 reads it).
  - `input_section_bounds_within` is private; the gates reach it as
    `test_support::input_section_bounds_within`, behind `test-support` (m5, ruling (i)).
  - Docs: the budget, the two charge constants (with their calibration), the cache cap (a rebuild
    is served entirely from the cache only while the cache has not cleared; the memory measured
    below), the cache, and the module doc (m2) are restated. The budget's doc says this
    workstation stood in for the CI-class runner (n2).
- `builtins-compiler`: the `tails` field doc and the bound call's comment say what a non-live strip
  carries past the budget and that the live bound is read from the table (m2).
- `host-core`: the native gate-3 test's comment names its own catch (m1) and "the third design"
  (n1); its premise reads charges.
- `crates/builtins/examples/input_bound_budget.rs` (with a `required-features = ["test-support"]`
  entry in `crates/builtins/Cargo.toml`): the committed gate-2 workload and the calibration
  (Amendment 3's commands).
- Specs: Amendment 3 here; #1465-#1468 cite "#1457's gates 2 and 8" with the commands (m4); #1470
  records n5 and the frame-equivalent budget; #1471 records m3 (gate 1 near the cap, the memory of
  a per-session copy) and the frame-equivalent budget.
- No render path changed. The charge constants, the counters and the example are control-side or
  test-only.

**Calibration** (`input_bound_budget calibrate`; this workstation as the CI-class runner,
x86-64-v3, release, `taskset -c 7`, one warmup and two measured rounds). Each design is a one-strip
preparation with a fresh cache and no budget limit (its keying, the walk and the cache insertion);
per class, the least-squares line of its time against its frames over the designs of a 48-step
cutoff grid (40 Hz to the maximum, trims 0, +12 and +24 dB) that walk at most 16,384 frames.
Intercepts in us, rounds 1 / 2 (slope in ns per frame):

| rate | one cascade, one section | one cascade, two sections | two cascades, one section each | two cascades, two sections each |
|---|---|---|---|---|
| 44.1 kHz | 4.88 / 4.72 (8.9-9.0) | 12.48 / 11.73 (15.1) | 9.15 / 9.13 (8.9-9.0) | 19.73 / 19.69 (15.5-15.9) |
| 48 kHz | 4.66 / 4.84 (8.8-9.4) | 11.83 / 11.80 (14.6) | 9.24 / 9.16 (8.9) | 20.02 / 19.57 (15.2-15.4) |
| 88.2 kHz | 5.47 / 5.47 (8.3) | 13.71 / 13.71 (13.8) | 10.31 / 10.93 (8.4) | 22.70 / 22.33 (14.5-14.6) |
| 96 kHz | 5.51 / 5.56 (8.3-8.4) | 13.57 / 13.69 (13.7) | 10.41 / 10.51 (8.4) | 22.28 / 22.24 (14.4) |

- **Per design and per section, separately.** One cascade of one section is `D + S`, of two
  sections `D + 2S`: `S` = 6.95-8.24 us and `D` = -2.8 to -2.1 us. The per-design term is negative
  in every round at every rate: a design has no fixed cost beyond its sections', and the second
  section of a cascade costs more than the first (the fixed cost is superlinear in a cascade's
  sections). Two one-section cascades cost twice one, so there is no per-cascade term either.
- **The constants.** `INPUT_BOUND_DESIGN_CHARGE = 0` (the measured per-design term, clamped at
  zero). `INPUT_BOUND_SECTION_CHARGE = 520` frame-equivalents: the smallest per-section charge that
  covers every class's measured fixed cost, `max(c11, c12 / 2, c2x1 / 2, c2x2 / 4)` = 13.71 / 2 =
  6.86 us = 518 frame-equivalents at 75,500 frames per ms, rounded up. Every other class is charged
  above its fixed cost (one section: 520 against at most 420 frame-equivalents).
- **What the charge does not model.** The slope: a short two-section walk costs 13.7-15.1 ns per
  frame, above the budget's 13.25 ns reference (75,500 frames per ms, measured on long near-top
  walks). A per-frame excess cannot be a fixed charge, so the worst case below is above 20 ms; it is
  stated, not hidden.

**Gate 2** (`input_bound_budget`, same box and conditions, one invocation, the final constants:
budget 1,510,000, design 0, section 520). "first" is `input_section_bounds` with a fresh cache,
"rebuild" the same cache warm, "no cache" without one; "design work" is no cache less rebuild (the
O(strips) keying taken out). Ranges over rounds 1 and 2 at the rates named; ms:

| family | rates | first | rebuild | no cache | design work | frames walked | exact |
|---|---|---|---|---|---|---|---|
| top pair (HPF one `f32` below the maximum into the LPF at it, +24 dB) | all | 6.36-8.11 | 0.001 | 6.38-7.09 | 6.38-7.09 | 525,520-528,640 | 1 / 1 |
| LPF at the maximum, +24 dB | all | 3.33-3.77 | 0.001 | 3.33-3.43 | 3.33-3.43 | 448,513-451,073 | 1 / 1 |
| 64-design near-top family | all | 18.38-19.15 | 0.018 | 18.35-19.18 | 18.33-19.16 | 1,505,840 | 3 / 64 |
| 65,537 distinct near-top designs | all | 52.93-53.35 | 22.56-23.26 | 40.92-41.67 | 18.03-18.68 | 1,506,880 | 2 / 65,537 |
| 64 typical (20 Hz HPF + steps into a 20 kHz LPF, 0 dB) | 44.1 / 48 | 9.85-10.77 | 0.020 | 9.95-10.57 | 9.93-10.55 | 664,896 / 718,656 | 64 / 64 |
| 64 typical | 88.2 / 96 | 17.60-19.27 | 0.020 | 17.62-19.03 | 17.60-19.01 | 1,278,528 / 1,388,352 | 64 / 64 |
| 256 typical | all | 19.83-20.53 | 0.070-0.079 | 19.77-20.56 | 19.70-20.48 | 1,362,320-1,440,320 | 141 / 130 / 72 / 66 of 256 |
| 4,096 one-section (1 kHz LPF + steps, 0 dB) | all | 15.21-15.81 | 1.45-1.49 | 14.83-15.53 | 13.36-14.08 | 749,760 / 900,560 | 1,461 / 1,171 of 4,096 |
| 4,096 two-section (1 kHz HPF into 1.28 kHz LPF + steps, +24 dB) | all | 21.78-25.58 | 1.63-1.69 | 21.54-24.36 | 19.88-**22.72** | 498,636-749,275 | 972 / 834 / 834 / 731 |
| 4,096 two-section (1.28 kHz into 5.12 kHz, +12 dB) | all | 20.79-27.67 | 1.65-2.65 | 20.28-22.63 | 18.61-20.98 | 498,636 / 641,600 | 972 / 834 |
| 4,096 two-section (2.56 kHz into 5.12 kHz, +24 dB) | all | 18.09-20.66 | 1.65-1.66 | 17.69-20.12 | 16.04-18.46 | 498,636 | 972 |
| 4,096 two-section (5.12 kHz into 10.24 kHz, 0 dB) | all | 16.38-18.16 | 1.55-1.63 | 16.07-17.89 | 14.46-16.34 | 498,636 | 972 |
| 4,096 two-cascade (left 1 kHz LPF + steps, right 1 kHz HPF, 0 dB) | all | 14.78-15.58 | 1.39-1.40 | 14.58-15.46 | 13.18-14.06 | 749,760 / 900,560 | 730 / 585 |

The example asserts, for every row, that first, rebuild and no cache report identical bounds,
that the rebuild walks no frame and computes no bound, and that a preparation walks at most the
budget.

- **The stated worst case.** Design-bound work under the budget: measured maximum **22.7 ms** (the
  1 kHz / 1.28 kHz two-section family at 88.2 kHz), spread **13.2-22.7 ms** across the families
  (near-top 18.0-19.2 ms, typical 9.9-20.5 ms, the cheap families 13.2-22.7 ms). A whole
  preparation of up to 4,096 strips, keying included: at most 25.6 ms (one 27.7 ms outlier, in a
  round whose keying-only rebuild also took 2.65 ms against 1.65 ms). Attempt 1's frames-only
  charge let the same two-section families take 48-64 ms (verdict MJ1).
- **Beside it, outside the budget: the O(strips) keying.** 0.35 us a strip: 22.6-23.3 ms for
  65,537 strips, 1.4-1.7 ms for 4,096. The 65,537-strip first preparation takes 11-12 ms more than
  the same preparation without a cache although both run the same code once the budget is spent
  (three cache insertions apart); not investigated (probably fresh page faults for the
  per-preparation design map, which is the first large allocation of each round).
  *(Corrected in attempt 3, verdict n1 and root ruling: this is allocator first-touch of the
  preparation's own memory, not a cache cost. With glibc heap trimming off, a cold cache costs the
  same as no cache, 39 ms, and only the process's first preparation pays the extra; the 22.6 ms
  keying figure holds on warm heap memory. #1471 explains and measures the first-touch cost.)*
- **Measurement history, candidly.** The calibration and gate 2 were each run more than once:
  a gate-2 trial at a provisional section charge of 640 (before the calibration was final); one
  invocation of both while another workload loaded the box (every workload about 1.7x slower, the
  top pair 11.1 ms against 6.4 ms; kept in the scratch evidence, not used); the clean calibration
  above with a gate-2 run at a section charge of 500; and, after the calibration showed that 500
  does not cover the two-section cascade at 88.2 kHz (13.71 us against 13.25 us), the gate-2 run
  above at 520. No number was tuned against a timing result: the constants come from the
  calibration's fixed costs only.

**Gate 8** (`every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`,
`--nocapture`): every document exact at every rate. Charged (frames walked), margin to 1,510,000:

| documents | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| `console-sixty-four-track`, `-app`, `-intended`, `-sends` | 743,552 (610,432), margin 766,448 | 791,936 (658,816), margin 718,064 | 1,290,880 (1,157,760), margin 219,120 | 1,387,136 (1,254,016), margin **122,864** (92 % used) |
| `console-sixty-four-track-mono` | 383,040 (316,480), margin 1,126,960 | 408,384 (341,824), margin 1,101,616 | 667,712 (601,152), margin 842,288 | 717,376 (650,816), margin 792,624 |

The new charge adds 133,120 frame-equivalents to a stereo document (64 designs of two two-section
cascades) and 66,560 to the mono one; gate 8 holds at every rate, the tightest at 96 kHz.

**Cache memory** (a full cache of 8,192 one-section designs at 48 kHz, `bench_support`'s thread
counters, a scratch example not committed): 2,486,520 bytes, about 304 bytes an entry (2.37 MiB).
The attempt-1 verifier measured 2,366,400 bytes (2.26 MiB) before each entry gained its charge.

**Tests** (test-value sentences: which plausible defect turns each red that no other test catches):
- New: `tail_contract::a_design_walks_at_most_the_budget_it_is_given` (gate 7 at the bound level,
  MJ2): red when a walk loop counts frames without stopping at its horizon (X3, the majorant pass;
  X5, the deviation walk) or when the right channel of a two-cascade design is given the whole
  budget instead of what the left channel's charge left (C4) or its charge is dropped (C8). X3, C4
  and C8 turn only this test red in `tail_contract`; X3 also turns gate 7 red.
- Changed: `tail_contract::the_live_bound_is_taken_exactly_when_the_budget_is_exhausted` (gate 5)
  now charges `charge`, is gated on `test-support` and asserts each design's charge exceeds its
  frames: red when a cache hit is admitted on its frames rather than its charge (C2, unique), as
  well as attempt 1's M3, M5 and M7.
- Changed: `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`
  (gate 7): a cold preparation walks exactly the budget less the three designs' fixed charges,
  counted for real; red on X3 (1,565,321 frames walked against 1,506,880), and uniquely on M8 (a
  stopped design not cached) and M4 (honoured with `<`).
- Changed: `builtins_compiler::tests::every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`
  (gate 8) holds the charge inside the budget: red, and only it, when the section charge grows to
  1,100 (C6) or the budget drops to 1,350,000 (C7).
- Changed: `host_core::prepare::tests::a_design_bound_cache_changes_no_prepared_value` (native
  gate 3): red, and only it, when the policy function hands `None` to `builtins-compiler` instead
  of its cache (X1). Its premise now reads charges.

**Mutation runs** (each applied alone in the worktree, the named suites run in debug with
`--no-fail-fast`, then reverted; `tc` = `tail_contract`'s #1457 tests, `bc` = the three
builtins-compiler tests above, `hc` = the host-core test):

| id | mutation | red |
|---|---|---|
| X3 | the majorant pass counts frames but does not stop | tc: `a_design_walks_at_most_the_budget_it_is_given` (`walked <= budget`: 516,608 walked under a budget of 1,040); bc: gate 7 |
| X5 | the deviation walk counts frames but does not stop | tc: the new test and gate 5 |
| C1 | the sections' charge is not reserved before the walk | tc: both; bc: gate 7 |
| C2 | a cache hit is admitted on its frames, not its charge | tc: gate 5 only |
| C3 | a design's charge omits its section charge | tc: both; bc: gate 7 |
| C4 | the right channel gets the whole budget, not what the left left | tc: the new test only |
| C8 | a two-cascade design's charge omits the right channel's | tc: the new test only |
| C9 | a stopped design leaves the remaining budget | tc: gate 5; bc: gate 7 |
| M8 | a stopped design is not cached | bc: gate 7 only |
| M4 | a cached stop is honoured only when `remaining < above` | bc: gate 7 only |
| C6 | `INPUT_BOUND_SECTION_CHARGE` 1,100 | bc: gate 8 only |
| C7 | `INPUT_BOUND_BUDGET_FRAMES` 1,350,000 | bc: gate 8 only |
| X1 | the host-core policy function passes `None` to builtins-compiler | hc only |

`INPUT_BOUND_DESIGN_CHARGE` is zero, so no test can see it reserved or not; its mechanism is the
section charge's, which C1 and C3 cover.

**Corrections to attempt 1's record** (verdict m1, n4): the host-core test's own catch is X1, not
M11 (= M1, which gates 5 and 7 also catch); M10 (the 48 kHz table entry) is also caught by
`live_input_lane_reports_the_live_bound_and_plain_input_its_own`, and gate 6's own catch is M10b (the
96 kHz entry); M12 and M13 are also caught by the host-core test's premise, and gate 8's own catch
was M12b (budget 1,200,000), now C6 and C7.

**Gates run** (on the attempt-2 tree, x86-64, `CARGO_INCREMENTAL=0`):

- `cargo fmt --all -- --check`; test-debug-a (`cargo test -p builtins-compiler --no-run`, the
  workspace step with its exact `--exclude` and `--features` list, and its doctests); test-debug-b
  (the DSP step with `math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`,
  its doctests, `conformance_fixtures -- --check`): all exit 0.
- Release: `-p lane -p math -p wasm-gates --features math/lane`, `filter_liveness`, `tail_contract`
  (all four rates): exit 0. `cargo test -p builtins --test tail_contract --no-run` without
  `test-support` compiles (the gated tests drop out).
- `cargo clippy --workspace --all-targets -D warnings`, with and without `--all-features`: clean.
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`, no exclusion: clean.
- `target/release/audit capi`: 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations,
  `pcm_digest` `cb10fbface44a3a4` (unchanged).
- `check-cross-targets.sh`, `check-capi-abi.sh` and `--self-test`, `run-wasm-gates.sh
  --without-v8-spill --without-native`: ok. The worklet chain (`build-web-audioworklet.sh
  --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `check-sdk-headless.sh`,
  `check-sdk-generated.sh`, `test-web-audioworklet.sh`) and the scalar-oracle and v8-spill
  checks: ok.
- Every `scripts/check-*.sh` with its CI arguments (artifact-evidence-leak, bench-policy,
  bench-preconditions, builtins-fixtures, builtins-listening, builtins-policy,
  conformance-boundaries, console-benchmark-fixture, console-fixtures, cross-targets, dsp-research,
  effect-contract, effect-runtime-fixtures, effect-runtime-policy, env-vocabulary,
  graph-determinism, graph-policy, host-core-policy, lane-policy, parametric-eq-render-contract,
  protocol-control-policy, protocol-wasm-parity, rack-policy, realtime-audit-leak, realtime-policy,
  sdk-types, session-policy, unfused-seal and `--self-test`, workspace-policy), every
  `scripts/test-*.sh` self-test (the list above plus console-benchmark, gate-lib,
  realtime-trace-validator, sdk-artifact-builder-output-contract and the three
  `test-realtime-audit-probes.sh` legs), and the Python checks and self-tests (ci-path-routing,
  release-shape, script-reachability, sdk-deletions, test-support-ci, npm-publish-modes,
  session-map-shape, command-kind and command-reason vocabularies, and `--self-test` of
  abi-layout-v1 and parameter-metadata-v1): all exit 0. (`check-host-core-policy.sh` now passes;
  attempt 1 saw it fail on its base.)
- AArch64 runs only in CI.

### Attempt 3 (2026-10-08): the frame-equivalent from the slowest frame class (Amendment 4)

**What changed.**
- `builtins` (`tail.rs`, `lib.rs`): `INPUT_BOUND_DESIGN_CHARGE` and its reservation and addition
  are removed (m1): a design charges its frames walked plus `INPUT_BOUND_SECTION_CHARGE` per
  section. A frame-equivalent is 23.5 ns, one frame of the slowest frame class measured, rounded
  up; `INPUT_BOUND_SECTION_CHARGE` is 290 in those units (was 520 in units of 13.25 ns). The
  budget stays 1,510,000 frame-equivalents, now 35.5 ms. The constants' docs say so; the cache
  cap's arithmetic is restated (a design charges at least 547, so at most 2,760 designs a
  preparation); the 122-column doc line is wrapped (n3).
- `math::tail`: one unit test sweeps the horizon over a cheap design (m2, below).
- `crates/builtins/examples/input_bound_budget.rs`: two band families (64 and 4,096 designs, the
  HPF at 0.778 of the maximum plus 1-N `f32` steps into the LPF at the maximum, +24 dB); a
  design-work column (no cache less rebuild) and its five slowest rows; the calibration measures
  the frame classes first (ns per frame of long near-top walks, the HPF at 0.50-0.99 of the
  maximum and one `f32` below it, at 0 and +24 dB) and states the fixed costs in frames of the
  slowest class it measured; the doc names the real constants (n2).
- Tests: the design charge's terms leave `tail_contract::a_design_walks_at_most_the_budget_it_is_given`
  and `builtins_compiler`'s gate-7 test (their assertions are otherwise unchanged).
- Specs: Amendment 4 here, the n1 correction in attempt 2's record; the successor #1474 (filed,
  local spec, STREAMS row 24 after #1457 and before #1465, the hot-file order); #1465's
  Dependencies and F5 name #1474 and the removed charge; #1471's Context drops the design charge
  and gate 4 gains the first-touch measurement (root ruling, n1).
- No render path changed.

**Calibration** (one invocation, `taskset -c 7 target/release/examples/input_bound_budget
calibrate`, release, x86-64-v3, this workstation as the CI-class runner; load average 0.78 before
and 0.84 after). The constants come from this fixed-cost measurement only, never from a gate-2
timing.
- Frame classes, measured rounds 1 and 2: the slowest class costs 22.58-23.15 ns a frame at every
  rate and both trims, with the HPF at 0.73-0.87 of the maximum (the fractions above 18 ns; 0.66
  once); the points outside the band cost from 11.96 ns to below 18 ns (attempt 2's verifier
  measured 12.0-14.7 ns there). The warmup rounds (not measurements) reached 23.85 ns once and
  28.06 ns once (96 kHz, 0 dB). Slowest measured: **23.15 ns**, so a frame-equivalent is 23.5 ns.
- Fixed costs per class (measured rounds; intercepts of the time-against-frames line over walks of
  at most 16,384 frames): one cascade of two sections 11.31-13.55 us; two cascades of two sections
  18.58-21.82 us; per design -2.52 to -1.56 us at every rate (negative: no per-design charge). The
  largest fixed cost per section is 13.55 / 2 = 6.78 us (88.2 kHz, round 2), 292.7 frames of the
  23.15 ns class and 288.3 frame-equivalents of 23.5 ns, so `INPUT_BOUND_SECTION_CHARGE` = 290.
- Every frame of every design measured costs at most one frame-equivalent (short two-section walks
  13.6-15.5 ns, long near-top walks 11.96-23.15 ns), and every section's fixed cost at most its charge,
  so the budget bounds the real design-bound work at 35.5 ms.

**Gate 2** (one invocation after the constants were fixed, `taskset -c 7
target/release/examples/input_bound_budget`, one warmup and two measured rounds, no retry; load
average 1.59 before and 1.54 after, a shared box). "design work" is no cache less rebuild (the
O(strips) keying taken out). Ranges over rounds 1 and 2; ms:

| family | rates | first | rebuild | no cache | design work | frames walked | exact |
|---|---|---|---|---|---|---|---|
| top pair (HPF one `f32` below the maximum into the LPF at it, +24 dB) | all | 6.43-6.73 | 0.001 | 6.42-6.67 | 6.42-6.67 | 525,520 / 528,640 | 1 / 1 |
| LPF at the maximum, +24 dB | all | 3.31-3.37 | 0.001 | 3.32-3.42 | 3.32-3.42 | 448,513 / 451,073 | 1 / 1 |
| 64-design near-top family | all | 18.56-19.58 | 0.018 | 18.60-19.36 | 18.58-19.34 | 1,507,680 | 3 / 64 |
| 65,537 distinct near-top designs | all | 52.99-63.18 | 22.67-24.84 | 41.00-42.73 | 17.05-19.85 | 1,508,260 | 2 / 65,537 |
| **64 band designs** | all | 33.79-34.50 | 0.019 | 33.73-34.92 | **33.71-34.90** | 1,507,680 | 3 / 64 |
| **4,096 band designs** | all | 35.10-40.49 | 1.37-1.39 | 35.36-35.78 | **33.97-34.40** | 1,507,680 | 3 / 4,096 |
| 64 typical (20 Hz HPF + steps into a 20 kHz LPF, 0 dB) | 44.1 / 48 | 9.88-10.73 | 0.020 | 9.85-10.58 | 9.83-10.56 | 664,896 / 718,656 | 64 / 64 |
| 64 typical | 88.2 / 96 | 17.69-19.22 | 0.020 | 17.70-19.13 | 17.68-19.11 | 1,278,528 / 1,388,352 | 64 / 64 |
| 256 typical | all | 20.71-21.62 | 0.070-0.081 | 20.39-21.52 | 20.32-21.45 | 1,423,580-1,469,980 | 148 / 137 / 74 / 68 of 256 |
| 4,096 one-section (1 kHz LPF + steps, 0 dB) | all | 18.82-19.20 | 1.45-1.50 | 18.46-20.33 | 17.00-18.87 | 964,510 / 1,096,460 | 1,880 / 1,425 of 4,096 |
| 4,096 two-section (1 kHz into 1.28 kHz, +24 dB) | all | 27.75-36.27 | 1.68-1.80 | 27.90-31.22 | 26.19-29.54 | 708,453-964,220 | 1,381 / 1,119 / 1,119 / 940 |
| 4,096 two-section (1.28 kHz into 5.12 kHz, +12 dB) | all | 26.23-27.93 | 1.67-1.73 | 26.52-28.29 | 24.79-26.61 | 708,453 / 860,511 | 1,381 / 1,119 |
| 4,096 two-section (2.56 kHz into 5.12 kHz, +24 dB) | all | 24.53-27.44 | 1.70-1.90 | 24.34-27.62 | 22.58-25.92 | 708,453 | 1,381 |
| 4,096 two-section (5.12 kHz into 10.24 kHz, 0 dB) | all | 22.74-25.88 | 1.59-1.71 | 22.29-25.36 | 20.63-23.75 | 708,453 | 1,381 |
| 4,096 two-cascade (left 1 kHz LPF + steps, right 1 kHz HPF, 0 dB) | all | 18.54-18.86 | 1.40-1.42 | 18.36-30.49 | 16.95-29.07 | 964,510 / 1,096,460 | 940 / 712 |

The example asserts, for every row, that first, rebuild and no cache report identical bounds,
that the rebuild walks no frame and computes no bound, and that a preparation walks at most the
budget.

- **The stated worst case.** Design-bound work under the budget: measured maximum **34.90 ms**
  (64 band designs, 44.1 kHz, round 2), inside the budget's 35.5 ms. Spread of the design work
  over rounds and rates: the band families 33.71-34.90 ms; the cheap two-section families
  20.63-29.54 ms; typical 9.83-21.45 ms; cheap one-section and two-cascade 16.95-18.87 ms (one
  two-cascade round at 44.1 kHz took 29.07 ms while its first preparation took 18.75 ms: noise on
  a shared box, not repeated); near-top 17.05-19.85 ms; the single top designs 3.32-6.67 ms. **The per-frame cause:** the band walks about the same frames as the 64-design
  near-top family (1,507,680), but each costs about 23 ns against about 12.8 ns; attempt 2's
  verifier found the first section's state or error radius subnormal on almost every band frame
  (likely cause, not proven; #1474 diagnoses it and, if confirmed, removes it).
- **A whole preparation of up to 4,096 strips**, keying included: at most 35.8 ms, apart from two
  single-round first preparations (40.49 ms, 4,096 band designs at 88.2 kHz; 36.27 ms, the 1 kHz /
  1.28 kHz family at 88.2 kHz) whose no-cache runs in the same round took 35.78 and 31.22 ms.
- **Beside it, outside the budget: the O(strips) keying**, 1.4-1.9 ms for 4,096 strips and
  22.7-24.8 ms for 65,537 on warm heap memory. The 65,537-strip first preparation's extra 11-21 ms
  over no cache is allocator first-touch (n1, above), not a cache cost; #1471 measures it.

**Gate 8** (`every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`,
`--nocapture`): every document exact at every rate. Charged (frames walked), margin to 1,510,000:

| documents | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| `console-sixty-four-track`, `-app`, `-intended`, `-sends` | 684,672 (610,432), margin 825,328 | 733,056 (658,816), margin 776,944 | 1,232,000 (1,157,760), margin 278,000 | 1,328,256 (1,254,016), margin **181,744** (88 % used) |
| `console-sixty-four-track-mono` | 353,600 (316,480), margin 1,156,400 | 378,944 (341,824), margin 1,131,056 | 638,272 (601,152), margin 871,728 | 687,936 (650,816), margin 822,064 |

The new unit lowers each section's charge, so gate 8 gains margin at every rate (96 kHz: 122,864
to 181,744).

**Measurement history (n4).**
- Attempt 2, from its record and its verifier: the calibration ran twice (once while another
  workload loaded the box, about 1.7x slower, discarded; once clean, the record's figures). Gate 2
  ran four times: a trial at a provisional section charge of 640 before the calibration was final,
  the loaded run (discarded), a run at 500, and, after the clean calibration showed that 500 does
  not cover the two-section cascade at 88.2 kHz (13.71 us against 13.25 us), the run at 520 that
  the record reports. The 500-to-520 change came from the calibration, not from a gate-2 timing.
- Attempt 3: the calibration ran once and gate 2 ran once, after the constants were fixed. The
  constants come from the calibration's fixed-cost and frame-class measurements only.

**Tests** (test-value sentences: which plausible defect turns each red that no other test catches):
- New: `math::tail::tests::a_walk_takes_at_most_its_horizon_and_finishes_exactly_when_the_horizon_covers_it`
  (m2): red when the replay of the crossing block counts frames without stopping at its horizon
  (X7), which no other test catches; also red on X3 (majorant pass), X5 (deviation walk) and X9
  (`take_frame` lets one frame past the horizon). Its design (a 300 Hz LPF at 44.1 kHz) walks
  three majorant blocks, the replay and one deviation frame (1,025 frames), and every horizon from
  0 to 1,026 is checked: a stopped walk takes exactly its horizon and returns `None`, a covered
  walk returns `fixed_cascade`'s result bit for bit.
- Changed (the design charge's zero terms removed; assertions otherwise unchanged):
  `tail_contract::a_design_walks_at_most_the_budget_it_is_given` and
  `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`
  keep attempt 2's catches (C4 and C8 the first's alone; M4 and M8 the second's alone).
- Gate 8 (`every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`,
  unchanged code) under the new charge: red, and only it, at a section charge of 1,100 or a budget
  of 1,300,000.

**Mutation runs** (each applied alone in the worktree, then the file restored; debug,
`--no-fail-fast`; `tc` = `tail_contract` with `test-support`, `bc` = the builtins-compiler lib,
`hc` = the host-core test `a_design_bound_cache_changes_no_prepared_value`):

| id | mutation | red |
|---|---|---|
| X3 | the majorant pass counts, does not stop (`take_frame(walked, u64::MAX)`) | the new math test (horizon 0 walked 768) |
| X5 | the deviation walk counts, does not stop | the new math test (horizon 1,024 finished at 1,025) |
| X7 | the replay counts, does not stop | the new math test (horizon 768 walked 1,024) |
| X9 | `take_frame` stops at `walked > horizon` | the new math test (horizon 0 walked 1) |
| C1 | the sections' charge not reserved before the walk | tc: `a_design_walks_at_most_the_budget_it_is_given`, gate 5; bc: gate 7; hc green |
| C3 | a design's charge omits its section charge | tc: the same two; bc: gate 7; hc green |
| C6 | `INPUT_BOUND_SECTION_CHARGE` 1,100 | bc: gate 8 only |
| C7 | `INPUT_BOUND_BUDGET_FRAMES` 1,300,000 | bc: gate 8 only |

All four X mutations restored: the math lib's three tests pass.

**Gates run** (on the attempt-3 tree, x86-64, `CARGO_INCREMENTAL=0`; all exit 0):
- `cargo fmt --all -- --check`; test-debug-a (`cargo test -p builtins-compiler --no-run`, the
  workspace step with its exact `--exclude` and `--features` list, its doctests); test-debug-b
  (the DSP step with `math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`,
  its doctests, `conformance_fixtures -- --check`).
- Release: `-p lane -p math -p wasm-gates --features math/lane`, `filter_liveness`,
  `tail_contract`; `tail_contract --no-run` without `test-support` compiles.
- `cargo clippy --workspace --all-targets -D warnings`, with and without `--all-features`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`.
- Release `audit`, `bench`, `capi`, `session-validator` build and the release audit tests;
  `target/release/audit capi`: 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations,
  `pcm_digest` `cb10fbface44a3a4` (unchanged); `check-builtins-fixtures.sh`,
  `check-console-fixtures.sh`, `check-effect-contract.sh`, the three
  `test-realtime-audit-probes.sh` legs, `run-protocol-allocation-audit.sh`; `check-capi-abi.sh`
  and `--self-test`, then `check-scalar-oracle-absent.py --native` and `--self-test`;
  `check-cross-targets.sh`, `check-graph-determinism.sh`, `check-protocol-wasm-parity.sh`,
  `run-wasm-gates.sh --without-v8-spill --without-native`.
- The worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent.py --wasm`, `check-web-audioworklet-v8-spill.py` and `--self-test`;
  SDK `npm ci`, `check-sdk-generated.sh`, `check-sdk-deletions.py` and `--self-test`,
  `check-sdk-types.sh`, `check-sdk-headless.sh`, `sdk-package.sh check`;
  `test-web-audioworklet.sh`.
- Every lint-job `scripts/check-*.sh` (workspace, session, bench, host-core, protocol-control,
  realtime, realtime-audit-leak, artifact-evidence-leak, lane, rack, builtins, graph,
  effect-runtime policies and fixtures, env-vocabulary, conformance-boundaries,
  console-benchmark-fixture, bench-preconditions, unfused-seal and `--self-test`,
  parametric-eq-render-contract, dsp-research, builtins-listening), every `scripts/test-*.sh`
  self-test that runs locally (the same list plus builtins-fixtures, console-benchmark,
  realtime-trace-validator, gate-lib, sdk-artifact-builder-output-contract), and the Python checks
  and self-tests (ci-path-routing, test-support-ci, script-reachability, release-shape and
  `--self-test`, npm-publish-modes, session-map-shape, the command-kind and command-reason
  vocabularies, `--self-test` of abi-layout-v1 and parameter-metadata-v1).
- AArch64 runs only in CI.

**Open items for root.** #1468's and #1470's local specs still name `INPUT_BOUND_DESIGN_CHARGE`
(and #1470 "about 20 ms of near-top design work"); they are outside this attempt's named paths.
The GitHub bodies of #1465 and #1471 need a sync with their local specs.
