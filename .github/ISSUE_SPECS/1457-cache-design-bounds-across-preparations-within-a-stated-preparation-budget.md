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
  preparation thread alike. #1464-#1468 rerun gates 2 and 4 against this stated budget.
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

### Paths this slice needs (pending root: not yet named exceptions)

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

## Attempt record

### Attempt 1 (2026-10-07): rulings recorded, successor filed, stopped for paths

- Recorded Amendment 1 (root's rulings D1-D3, the split, gates 1 and 4 restated, gates 5 and 6
  added).
- Filed the successor #1470 (*Cache design bounds across browser preparations in the control
  Worker*) with a matching local spec and a stream G row (order 22, after #1457, after H #1332).
- Stopped before code under the worker rules: the C ABI and host-core paths above are not named
  exceptions for #1457, and D2's owner of the session-side rebuild needs root's choice. No code,
  measurement or mutation run yet; D1's frames and ms are still to be measured.
