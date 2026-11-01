# Bring the swap block under the 70 % callback margin at 96 kHz

Filed by root from the #1286 record (PR #2228, `artifacts/steps/swap-carry-base/`). Not launch-blocking for #1360: a warm adoption costs no more than the move it replaces.

## Problem
On the 64-track app console at 96 kHz, quantum 128, native 8-lane AVX2 (AMD EPYC 7313P), the block that applies a swap is above the release gate's callback margin (#26: P99.99 below 70 % of the quantum; `m · D` = 933,333 ns):

| block | p50 ns | p99 ns | p50 / quantum |
|---|---|---|---|
| ordinary | 73,630 | 177,157 | 5.5 % |
| move (carrying swap, A ⇄ B) | 1,033,337 | 1,070,848 | 77 % |
| warm adoption, first growth | 981,639 | 1,109,782 | 74 % |
| warm adoption, ΣP = 4,096 | 508,558 | 625,831 | 38 % |

So every structural edit during playback at 96 kHz / 128 takes a block near or over the margin. At 48 kHz the quantum is twice as long and the same block is about 39 %. #1269 deferred a whole-bank move; the record says it earns a brief. The first warm adoption costs about twice the last, with the smallest prime, so its cost is not the prime bytes D3 charges.

## Scope
1. Profile one move block and one first-growth adoption block on the record's subject (no `perf` on the bench host; use the bench's own phase timers or a scoped counter) and name where the time goes: lane copies, carry walk, state moves, fixed costs.
2. Brief the smallest change that brings both blocks under 70 % of the quantum at 96 kHz / 128, for example the whole-bank move of #1269 or moving fixed work to the control thread before the swap. Keep render allocation-free and syscall-free.
3. Re-record `swap-carry-base` once after the change (one warmup, two rounds).

## Gates
- Move and adoption p99 below 933,333 ns on the record's subject.
- No rendered bit moves (one-time comparison, PR evidence only).
