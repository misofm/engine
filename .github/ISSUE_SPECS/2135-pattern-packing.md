# Pack strips by activity pattern: the pattern sort, dwell, pins and the global budget

Draft slice 5 (#2135) of the banking design (`docs/BANKING_DESIGN.md`, 2026-10-26). Widens #1981's packable class; amends #1782 D5 item 3; **replaces #1981's running-unit pack rule** (its decision record, "The pack rule") with the pattern sort. The tests that pin the old rule's shape (`a_refused_pack_is_not_retried`'s attempt counts and peak step, the move count 38 in `a_packing_plan_renders_the_bits_of_an_unpacked_one`) are rewritten to this slice's claims in the same PR; one test per claim.

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

- **Pattern.** Per strip: `silent_at_rest` (#1981's idle count at `R`, with #2129 D6's rest rule;
  stated by #2133's propagated fact on **first-row units only**, #2142 attempt 2) and `bypassed[k]`
  settled per slot, read at the drain into a home-indexed `u16` bit set in `LaneMap`. After #2142 a
  strip is a **lane chain**: its slots sit in the chain's rows (`LaneMap::unit_chain`, `unit_row`),
  so the bits are gathered over every row of the chain (C1: all four console slots are in row 0; a
  strip split at `insert_send` has slots in two rows). The settled bit is the stage lane's
  `EffectControlLane::settled_bypassed()` (already read by `lanes_at_rest`,
  `crates/rack/src/lib.rs:2529-2545` on the #2142 branch); the settle **edge** is one new per-lane
  accessor on the stage, read once per block at the drain where S1b flags `rest_pending`
  (`read_cells`); no scan. Slots are ranked once at bind by the effect's measured isolated cycles per
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
  this episode (a refusal is any row's `BankChain::lane_swap_check` saying no: automation fit,
  collapse arming, a staged cell the check will not carry; the check runs over every row of both
  chains before any write, as #2142's `swap_bank_lanes` does).
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
   of_an_unpacked_one`: the #1981 harness on the **C1-16 fixture** (#2132) with `C1State::Typical`'s
   seeded draw as the base plus a seeded toggle schedule on every slot (groups, auxes, master), automation on two strips, one strip with
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
6. CPU, descriptive: one `bench console --bank-report` run (native W=8, one warmup, two rounds)
   and one `scripts/run-wasm-gates.sh` run (W=4 counters, #2143): C1 `Typical` and `Toggle` at
   16/32/64 against the #2142 baseline (`artifacts/steps/bank-design-5a/`); `AllActive` within 2 %.
   **The W=4 `Typical` pins move** (`tools/wasm-gates/src/lib.rs:242-243`,
   `[15, 0, 3584, 2304, 260, 3580, 1]`): the packer sorts the fixed Typical draw from bind, so the
   total runs, the track runs and `bypass_skips` change; re-pin from the wasm leg, and #2143's
   `w4_counters_describe_the_designs_plan` holds the arithmetic. Bit identity: all 16
   `BANK_REPORT_WORKLOADS` rows, 600 blocks, FNV-1a of every host word against main (the #2142 method).

## Paths

`crates/graph/src/runtime.rs` (pattern bits, counting sort, targets, cycle walk, dwell, pin,
global cursor and caps), `crates/graph/src/lib.rs`, `crates/rack/src/lib.rs` (the stage exposes
the settled-bypass mask and the settle event at the drain; `lane_swap_check` on staged cells),
`crates/graph-compiler/src/` (the automated-bypass bind-time bit),
`crates/graph-compiler/tests/silent_bank_skip.rs`, `tools/wasm-gates/src/lib.rs` (the W=4
re-pin), this spec. Slot ranking: a constant rank table per effect id at bind, derived once from
`docs/rulings/effect-floor-accounting.md` (no I/O).

## Dependencies

All merged: #1981, #2131, #2129 S1b (PR #2150), #2133, #2142 (PR #2154). #2129 S2 lands first
(it edits the per-node arm of `runtime.rs`, a disjoint region; declared edit order S2 then #2135).
Unblocks the dynamic finish line.


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

- Why this is the main item: #2132's `Typical` row executes 20.00 BSR/block at W=8/16 against a
  key-class ideal of 9 (28 vs 12 at 32, 44 vs 17 at 64); S1b's whole-bank-slot skip fires at
  `(1-p)^W` under id-order placement; #2142 grouped the track banks but #1981's class is silence
  only. No other open item reaches the dynamic finish line on the state the console lives in.
- Rewritten for #2142: the strip is a lane chain; facts on first-row units only; the swap is
  `swap_bank_lanes` over every row. Dependencies all merged. W=4 Typical pins will move.
- Expected after: `Typical` executed BSR/block from 20.00 to 9-12 (16 tracks, W=8), 28 to 12-16,
  44 to 17-23; W=4 16 tracks from 29 to about 14-18. `Load` stays 0; `AllActive` within 2 %.
