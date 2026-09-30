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

- *Measure the console strip against its baseline* (S4, #1099).
