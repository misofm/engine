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

## Evidence (attempt 1)

- **Profile (slice 1).** `bench swap` now records, per timed call, the render thread's minor page
  faults (`/proc/thread-self/stat`) and its allocator events, read on both sides of the call and
  off the clock (`move.ordinary_cost`, `move.move_cost`, `prime.growths[].adoption_cost`). Render
  holds no probe. At base the move and every adoption block take 0 faults at p50 (at most 3) and 0
  allocator events. So first-touch faults do not explain the dear first adoption, and no
  pre-touch helper was added (root ruling Q3).
- **Where the time goes.** A/B on the shape (untimed diagnostics on a loaded host, never a
  record): skipping the whole effect-lane payload carry saved only about 100 us of the
  1,015 us move. The dominant cost is `restore_home_lanes` (#1782a): a predecessor whose lanes
  lane packing (#1981) has displaced is put home lane swap by lane swap before the carry, and each
  effect lane swap was two payload snapshots and two validating, redesigning restores. A, B and a
  fresh A in the prime phase are packed after 32 blocks; `C_1..C_4` are not, which is why the
  first adoption costs twice the others. (These figures come from temporary timing probes in a
  scratch build, removed before any commit; the committed code and the records measure from the
  bench side only.)
- **Fix (slice 2).** `PreparedNativeEffectBank` gains `copy_lane_from` and `swap_lanes_direct`
  (with `as_any` and `as_any_mut`): a bank of the same type copies or exchanges lane words in
  place, with no payload, no validation and no redesign, to the bits the payload path leaves. The
  default declines and the payload path runs, so every other effect and every Node <-> Lane carry
  is unchanged. The parametric EQ and the compressor implement both; `rack`'s carry and
  `effect_contract::swap_bank_lanes` try the direct path first.
- **Tests.** `conformance::swap_probe::check_copy` (a copied lane, ramps in flight, holds, answers
  and renders exactly what the payload round trip leaves; zero allocator calls), run by
  `a_directly_copied_eq_lane_is_the_payload_round_trip` and
  `a_directly_copied_lane_is_the_payload_round_trip` (compressor). The #1747 swap gates
  (`swapped_eq_lanes_render_exactly_as_unswapped_ones`, `a_swapped_lane_renders_as_its_unswapped_twin`)
  now run through the direct swap. `host-core/tests/swap_block_realtime.rs`: both 64-track move
  blocks, after 32 blocks of packing each, leave the allocator and the render audit at 0.
  Red: EQ copy without `remaining` (copy test), EQ swap without the `initial_words` trade (swap
  test), compressor copy with the right channel left as it was (copy test), compressor swap
  without the link-mode trade (swap test), and a `Box` in the EQ's direct swap or copy (realtime
  test). A compressor copy without the not-carried `rest_link` stayed green: the probe never glides
  a link on the destination, so that mutant is equivalent on the probe's states.
- **No bit moved.** 300 blocks of every preparable `fixtures/session/v1` session, plain and live:
  the 34 digests are equal at base `d201e3082` and at head. The swap subject at the frozen shape
  rendered untimed (9,281 blocks, 32 moves, 128 adoptions, 0 refusals) has one digest,
  `bf3b72b3c736e51b`, at both. Both harnesses are scratch only.
- **Gate: passes.** One runner invocation each (one warmup, two rounds, controlled). p99 of the
  32 observations, round 1 / round 2, against 933,333 ns. Base
  (`artifacts/steps/swap-carry-2229-profile`, 82b899157): move 1,048,156 / 1,041,043; first
  adoption 1,016,576 / 998,532. Head (`artifacts/steps/swap-carry-2229`): move 723,738 / 729,729;
  adoption, growths 1-4, at most 817,397. At both, every swap block has at most 2 minor faults
  and 0 allocator events. One invocation before each record was refused at the load precondition,
  with no workload launched.
- **Review MINORs (folded).** The direct paths rest on two invariants that a restore checked on
  every call. Debug builds now assert both after every direct copy and swap: an EQ lane's target
  words are its targets' design (`Channel::targets_are_designed`), and a compressor lane's words
  are its current values' design (`state::words_are_designed`). Both checks run on stack
  arrays. `realtime_policy` passes, and the debug tests of parametric-eq, compressor and
  host-core pass with the copy, swap and swap-block allocation counts still 0.
  `swap_block_realtime.rs` now states what only it reaches. A `Box` in the EQ's direct swap
  leaves `successor_swap` gate 5 green and turns this test red. #1269 (closed) has a comment: its
  "whole-bank move" condition was met, and #2229 solves it with direct lane copies and swaps
  instead.
