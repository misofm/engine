# Pack strips by activity pattern: the pattern sort, dwell, pins and the global budget

Draft slice 5 (#2135) of the banking design (`docs/BANKING_DESIGN.md`, 2026-10-26). Widens #1981's packable class; amends #1782 D5 item 3.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Class

A, under #2131's per-lane un-bypass law: a move is a lane permutation and never changes a lane's
words (#1782a gate 3's shape, now over a bypass-toggling schedule).

## Problem

#2129 skips a bank-slot only when every lane of the bank has that slot bypassed. Under the
compile-time id-order placement the chance that a bank of `W` strips skips a slot enabled on a
share `p` of strips is `(1-p)^W`: at W=4 and p=0.5 it is 6 %, at W=8 0.4 %. A mix in progress
keeps most strip-slots bypassed, so most of the saving the owner's bypass decision buys is lost
to placement. #1981 moves lanes exactly on an activity change; its class is silence only.

## Smallest closable slice

- **Pattern.** Per strip: `silent_at_rest` (#1981's idle count at `R`, with #2129 D6's rest rule)
  and `bypassed[k]` settled per slot, read at the drain into a home-indexed `u16` bit set in
  `LaneMap`. Slots are ranked once at bind by the effect's measured isolated cycles per
  lane-sample (`docs/rulings/effect-floor-accounting.md`), tie by chain order; the pattern is an
  integer over the costliest `min(S, 8)` slots, slot 1 most significant.
- **Target: the full pattern sort.** On a dirty group, a counting sort over at most `2^8 = 256`
  patterns (preallocated `u16` counters and a `target: u16` per position): one pass counts the
  patterns of the movable positions, prefix sums give each pattern's first target position in
  descending pattern order (strips of one pattern in home order, so an all-one-pattern group's
  target is the identity map and the fused epilogue returns), a second pass assigns targets.
  Realised by a cycle walk upward through positions; a refused swap (automation fit, collapse
  arming, a staged cell the check will not carry) marks the strip `Fixed` for the episode and the
  target is recomputed next block.
- **Fixed strips.** Padded lanes; strips pinned at bind because a stored automation lane drives
  any slot's bypass (bind-time bit beside `automated`); strips inside their dwell; strips refused
  this episode.
- **Triggers.** `PackGroup::dirty` on: a bypass switch settling on any lane of the group; an idle
  count crossing `R`; **a strip's dwell expiring** (per-strip `dwell: u16` set to `R_b` at the
  settle, decremented at the drain, dirtying the group at zero); a move; bind. A clean group
  costs one load.
- **Residency.** Silent class `R = 16`; bypass class `R_b = 16` blocks after the last settle, during
  which the strip is `Fixed` and demanding (a toggle at the ramp period never moves).
- **Budget.** `B_total = W` swaps and `A_total = 4W` attempts per block across all groups,
  served round-robin over dirty groups by a cursor that survives blocks; a group stays dirty until
  its walk completes with nothing pending. A group converges in at most `ceil((span - 1)/W)`
  blocks of its share. Between blocks the cost may move by at most `2 x W x S` per block (two
  banks per swap); it is bounded, not monotone, because the sorted order is not always the layout
  of least cost; it is chosen because its cost is provably bounded (the finish-line bound).
- **Stale lanes** need no term: #2131 resets a lane at its own un-bypass wherever it is.

## Gates (one test per claim, red first on a named mutant)

1. `crates/graph-compiler/tests/silent_bank_skip.rs`, `a_pattern_packed_plan_renders_the_bits_
   of_an_unpacked_one`: the #1981 harness with a seeded bypass schedule on every slot of a
   20-strip C1-shaped session (groups, auxes, master), automation on two strips, one strip with
   automated bypass (pinned), one collapse-armed bank, a plan carry during a hold; every host
   word, post-matrix word, observed value and the master equal by `to_bits` against the twin with
   packing off; `bypass_skips` strictly greater than the twin's. Red on a fold out of home order,
   a target that counts a padded lane, a pinned strip that moved. Test value: red when a pattern
   pack moves a strip's or the master's bits, or packs without raising the skip count.
2. `crates/graph` unit test: a lane with a staged span (a live cell drained this block) moved by a
   forced swap before the unit loop renders the same words as unmoved. Red on a swap that leaves
   the cells behind. Test value: red when a move loses a drained-but-unapplied value (D15-2).
3. `crates/graph` unit test on the lane map, the bound and the dwell: two banks of four with
   patterns `[A,B,A,B | A,B,A,B]` (A: eq on, B: all off) reach `[A,A,A,A | B,B,B,B]` within
   `ceil(7/4) = 2` blocks under `B_total = 4`, the second bank's demand is 0 and the clean group
   costs no attempt; a strip toggled every 8 blocks never moves; **a strip toggled once is packed
   on the block its dwell expires, `R_b` blocks after the settle, with no other event** (N1);
   the review's W=4, k=2 layout (`11` at 0, `10` at 1-6, `01` at 7-8) measures 3 bank-slot runs
   on slot 2 against the bound `ceil(3/4) + 2^2 - 2 = 3`. Red on a missing expiry trigger (the
   toggled strip never packs), on a step that stops 1-swap-optimal short of the sort, on a move
   inside the dwell. Test value: red when the packer cannot reach the sorted layout the counts
   allow, thrashes on a ride, or forgets a strip after its dwell.
4. `crates/graph` unit test: four dirty groups and `B_total = W`, `A_total = 4W`: the moves and
   attempts per block never exceed the caps in total and every group converges within
   `4 x ceil((span - 1)/W)` blocks. Red on a per-group budget only, or on attempts uncapped under
   refusals. Test value: red when a scene recall's packing cost scales with the plan.
5. Allocation: #1981's carried-pair gate extended with a bypass toggle burst; zero.
6. CPU, descriptive (slice 0 (#2132) rows): C1 `Typical` and `Toggle` at 16/32/64 tracks, both widths,
   against the slice-0 baseline; `AllActive` within 2 %.

## Paths

`crates/graph/src/runtime.rs` (pattern bits, counting sort, targets, cycle walk, dwell, pin,
global cursor and caps), `crates/graph/src/lib.rs`, `crates/rack/src/lib.rs` (the stage exposes
the settled-bypass mask and the settle event at the drain; `lane_swap_check` on staged cells),
`crates/graph-compiler/src/` (the automated-bypass bind-time bit),
`crates/graph-compiler/tests/silent_bank_skip.rs`, `tools/console-workload/src/lib.rs` (the
`Toggle` row if slice 0 (#2132) has not landed), this spec.

## Dependencies

#1981 (merged first), #2131 (`reset_lane` and the per-lane un-bypass law), #2129 (the skip with
per-lane staleness and the travelling stale and hold bits); slice 4 (#2133) recommended first so the V8
baseline is the propagated one. Unblocks the dynamic finish line.
