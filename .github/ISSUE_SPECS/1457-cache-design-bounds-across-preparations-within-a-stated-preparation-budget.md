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
  AudioWorklet thread, after which the browser pays the cost on a Worker.

## Decisions to make in this slice (root approves before implementation)

- **D1. The budget.** A stated worst-case control-thread cost per preparation (for example per
  distinct design and per session), measured on the CI runner, with the platform it binds.
- **D2. The cache.** Where it lives (engine instance, control-plane crate), its key (#1329's design
  key plus the flush law, which depends only on the rate), its bound on entries and memory, and its
  eviction. The cache is control-side only; render never reads it (#1329 R5).
- **D3. What happens past the budget.** A session whose distinct designs exceed it either is
  refused with a typed diagnostic, or prepares with a cheaper, still certified, looser bound. No
  `Infinite` fallback for a design #1329 can bound.

## Objective gates (to be fixed with D1-D3)

1. A rebuild of an unchanged session computes no design bound (counted).
2. The worst-case preparation cost of the bounds, measured once on the CI runner, is within D1.
3. Every reported bound is bit-identical with and without the cache (`tail_contract`'s gates stay
   green).

## Non-goals

- Tightening any bound (#1433). Any render-side change. Effects' bounds (#1372-#1376).

## Dependencies

- #1329. Coordinate with #1332 (stream H) for the browser's preparation thread.
