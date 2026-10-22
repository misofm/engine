# Skip silent lanes' work in console banks

## Problem

Console slots always bank (decision 12). A bank's silent fast path is bank-wide:
`block_is_positive_zero` runs over frames x lanes. One active lane therefore keeps its bank-mates
processing. S4 (#1099) measured this at `f7ba70a8` (p50 µs per block, the mean of two rounds):
- `sixty_four_track_console` (all active): 100.51;
- `sixty_four_track_console_sparse` (odd tracks exact zeros): 100.11;
- `sixty_four_track_idle`: 31.16.

With half of every bank silent, the fast path recovers 0.40 of the 69.35 µs that silence removes
when whole banks are silent. That is against the owner's standing priority that no stage processes
silence. Masking a lane inside a vector removes no arithmetic, because the lanes run in lockstep.
The work can fall only if silent lanes stop occupying bank vectors.

## Smallest closable slice

Decide, prototype and measure one mechanism by which a console slot does no vector work for a lane
whose input block is exact positive zero and whose state is at the kernel's exact fixed point.
For example: per block, pack a slot group's active lanes into as few banks as they fill, and
leave the silent lanes untouched.

- Bits: banking may couple lanes' cost, never their bits.
- Latent slots: a silent lane of a latent slot still feeds its latency line (amendment 11).
- Masking: D7 recovery and reports stay masked by active lanes.

Name what the mechanism does at W=4 and at W=8. If it needs a new benchmark row (for example one
active lane per bank), split that row into a qualification issue first.

## Objective gates

1. Class A: a committed randomized differential renders sessions with random per-lane silence
   runs, with the skip and without it, on Simd8 and Simd4. Every lane is bit-identical, with NaNs
   folded.
2. Render allocates nothing: `allocations == 0`, measured on the render thread after warm-up.
3. One native run and one V8 run of the console benchmark. The sparse row is reported against S4's.
   The all-active console row is not more than 2 % slower than S4's.

## Dependencies

- None open. *Measure the console strip against its baseline* (S4, #1099) is closed as completed.
- Note (root-authorized, 2026-10-06; #1329 Amendment 3 addendum, A2): a lane's state is at the
  kernel's exact fixed point no later than the node's exact-rest bound, `RestSamples` (#1329 D2),
  which holds for every input up to its stated peak. Silence skipping uses `RestSamples`, not
  #1329's `T_rest`: `T_rest = max(T_decay, R(P*))` reaches exact zero only for inputs below the
  flush floor `P*` (above it, it is the `-144 dB` decay bound). #1329 delivers both for the builtin
  input section; each effect's bounds follow in #1372-#1376.

## Decision record

### D1 Design (attempt 1, 2026-10-22)

Mechanism: **swap-to-pack** inside one console chain (`rack::BankChain`, which runs every
console slot of a run over one resident AoSoA block per bank).

- The chain keeps a lane map, strip to (bank, lane), preallocated at prepare (`banks * W`
  entries, never a track limit). It starts as the compiler's order.
- Per block, a member lane is *active* unless its input block is exact `+0.0` on both planes
  and every slot of the chain reports that lane at its exact fixed point. With `A` active
  lanes, the chain runs only the first `K = ceil(A / W)` banks. An active lane in a bank
  `>= K` swaps with a silent member lane in a bank `< K`: the two lanes exchange their full
  per-lane state, coefficients, ramps, control lanes and taps in every slot of the chain.
  Padded lanes never swap, so D7 recovery and reports stay masked by the moved active mask.
- Gather and scatter read the lane map as a pointer list, so the audio copies stay one
  gather and one scatter per run. A skipped bank is not gathered; a strip whose output plane
  is not its input plane gets `frames` words of `+0.0` per channel.
- Bits: lanes run in lockstep with no cross-lane arithmetic (the detector link is L/R inside
  one lane), and a swap moves a lane's whole state, so the bits follow the strip. Shared
  per-bank cursors (limiter `Cursors`, soft-clip `pos`) still advance in a skipped bank, so
  every bank of a slot keeps equal cursors and a swapped lane lands on the same cursor.
- Latent slots (amendment 11): a lane at rest holds `+0.0` in its latency words (the limiter's
  `is_at_silent_rest` already proves `main_ring`); feeding `+0.0` into them is the cursor
  advance alone, which a skipped bank still does. The rack `BypassShunt` line needs the
  same proof per lane, or it keeps its capture.
- W=4 (Wasm, NEON): 64 tracks are 16 banks; the sparse row (odd tracks silent) packs 32
  active lanes into 8 banks. W=8 (AVX2): 8 banks pack into 4. One active lane per bank packs
  `B` banks into `ceil(B / W)`.
- All-active cost: the activity test is the existing `block_is_positive_zero` head (32 words
  per plane, which returns on the first non-zero word) plus a count; with `A` = members there
  is no swap and no other change. A swap costs the lane's state size (the limiter's rings
  dominate) and happens only when a lane's activity changes.

### D2 Blockers found before implementation (attempt 1 stops here)

1. **No console effect has an exact-rest bound.** Every console-eligible effect states
   `RestBound::Unstated` (parametric-eq `src/lib.rs:728`, compressor `:323`, gate-expander
   `:288`, soft-clip `:226`, transient-shaper `:175`, true-peak-limiter `:278`); #1372, #1375
   and #1376 are open. Under the brief's rule (the fixed point uses `RestSamples`; an effect
   without it does not skip), the mechanism skips no lane, so gate 3 can show nothing.
   Open question for root or the owner: does an *observed* exact fixed point satisfy the
   condition? The shipped bank-wide fast paths (#163: parametric-eq, compressor,
   true-peak-limiter) already skip on it (state bits equal before and after a zero block,
   `+0.0` output, stationary ramps), with no `RestSamples`.
2. **No per-lane fixed-point predicate and no render-time lane swap exist.** The three fast
   paths decide per bank; gate-expander, soft-clip and transient-shaper have none. The lane
   carry that exists (`EffectBankStage::carry_lane_from`, rack `src/lib.rs:911`) is a plan-swap
   restore through a payload, not a swap. The slice needs both on six effects and on
   `LiveControlEffectBankStage` (controls, taps, bypass shunt): more than half a day, so it
   splits up front.
3. **No V8 sparse row.** The V8 harness documents are `nine_track_eq`,
   `sixty_four_track_console`, `_app_shape` and `_console_sends`
   (`scripts/web-mixing-automation-benchmark.mjs:144-149`). Gate 3's V8 leg needs a new row,
   which the brief splits to a qualification issue first.
4. **No Simd4 plan leg on this host.** On an AVX2 build, five of the six console factories
   bind only the build's width (#1694, open), so gate 1's Simd4 leg needs #1694 or a wasm32
   test run.

Proposed split, in order: (a) qualification: the V8 sparse row; (b) the ruling in item 1, or
#1372/#1375/#1376; (c) a per-lane fixed-point predicate and an allocation-free per-lane swap
on the bank contract, effect by effect; (d) the chain compaction with gates 1-3.
