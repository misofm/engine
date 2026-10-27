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

## Evidence (attempt 1, 2026-10-27; branch `codex/batch-b13-patternpack`, main `7be08dd18` merged)

### What landed

- `crates/graph/src/runtime.rs`: `pack_step` / `pack_group_step` / `pack_sort` replace #1981's
  running-unit rule. Per dirty group: pattern 0 for a strip silent at rest, else `PACK_AUDIBLE`
  (bit 8) plus one running bit per ranked slot (costliest first); a counting sort over 512
  preallocated counters (`PACK_PATTERNS`, the audible bit over `min(S, 8)` slot bits) gives the
  movable strips the movable positions, one pattern in home order; a cycle walk realises it.
  Fixed: padded lanes, pinned strips, strips in their dwell, strips refused this episode.
  `B_total = W`, `A_total = 4W` per block across all groups, round-robin from a cursor that
  survives blocks. Dwell `R_b = PACK_DWELL_BLOCKS = 16`, started by `LaneMap::record_settles`
  (any row's unit, from `BankChain::settle_edges`), counted down by `LaneMap::tick_dwell` at the
  block top; an expiry dirties the group. Slot rank at bind (`bind_ranked_slots`) from
  `pack_slot_cost`: compressor 16.691, limiter 13.918, EQ 4.984 cycles/lane-sample (the floor
  table's per-effect isolates); unmeasured effects rank below, ties in chain order.
- **Refusal rule (decided here):** a refused trade fixes the *position's occupant* for the
  episode, not the target strip. Fixing the target strip made every running stereo strip whose
  sorted place is in a collapse-armed mono bank stay put (#1981's gate 1 then skipped no more than
  its twin); fixing the occupant marks the incompatible bank's positions one by one and the strip
  lands in the first compatible one. Each refusal fixes one strip, so an episode refuses at most
  `span` times. A fact change (`fresh`) starts a new episode and forgets the refusals.
- `crates/rack/src/lib.rs`: `BankStage::{settled_bypassed_lanes, settle_edges, effect_id}`;
  `LiveControlEffectBankStage` caches the settled mask at each cell read and reports the XOR
  against the last read (no edge on the first drain; before it, the mask is read from the lanes
  bind left, so the sort at bind sees the session's bypasses); a move trades both bits.
  `BankChain::{settle_edges, slot_settled_bypassed_lanes, slot_effect_id, slot_count}`; the
  chain ORs the drained slots' edges in its existing step-0 loop.
- **`lane_swap_fits` widened (needed, not in the spec):** a channel-less bypassed lane (a session
  bypass, #1087) now trades with an uncontrolled lane. Both hold no cell; the shunt term still
  decides whether the bypassed lane finds a line. Without it, a plan with no live effect channels
  (every bench row but Toggle) refused every trade between a bypassed and an enabled strip and
  the packer moved nothing on `Typical`.
- Pin: `PreparedGraphPlan::pin_bypass_automated_strips`, filled by graph-compiler from the
  session's stored automation on an effect bypass row (`bypass_automated_strips`), applied at bind
  (`Runtime::pin_strips`). Stored effect-bypass automation does not render yet
  (`controls::automation_row_renders` is fader, mute and VCA only), so today the pin keeps a
  strip whose bypass is static in place; it is ready for the slice that renders it.

### Gates (red first on the named mutant)

| gate | test | mutant | red |
|---|---|---|---|
| 1 | `silent_bank_skip.rs` `a_pattern_packed_plan_renders_the_bits_of_an_unpacked_one` | pinned bit ignored in `pack_sort` | "the pinned strip moved", block 0 |
| 1 | same | running bits forced to 0 | "the packing plan moved strips" |
| 1 | same | `swap_lanes` (other stage) leaves the control lane behind | host word 38 differs, block 4 |
| 2 | covered by `crates/rack/tests/lane_swap.rs` `a_swapped_strip_renders_and_publishes_what_its_twin_does` (a strip "fed and written as it moves"); no second test | - | - |
| 3 | `runtime.rs` `the_pattern_sort_reaches_its_layout_and_keeps_the_dwell` | no dirty at dwell expiry | case 3 |
| 3 | same | dwell not fixed | case 2, block 0 |
| 3 | same | padded lane movable | "a padded lane never moves" |
| 4 | `runtime.rs` `the_packing_budget_is_global` | budget per group | 7 swaps against 4 |
| 4 | same | attempts uncapped | 26 attempts against 16 |
| 5 | `a_skipping_block_allocates_nothing` + toggle burst | a `Vec` in `tick_dwell` | audited allocator abort |

Gate 1 deviates from the spec on two points: it runs C1-**32**, not C1-16 (at W=8, C1-16 has two
track banks, and a collapse-armed bank of eight refuses every trade with the other, so nothing
crosses banks), and the collapse-armed bank never collapses in `Typical` (a bypassed lane clears
`UNBYPASSED`), so it exercises the arming refusal, not the collapse. It adds per-track compressor
thresholds and drives, without which C1's default console is close to the identity and a lane left
behind moved no bit. Not in gate 1: the plan carry during a hold (the carry restores the identity
first, `restore_home_lanes`, which #1782a's gate 3 covers) and a displaced group's fold (C1's track
groups do not fold; #1981's gate 1 covers the fold). `a_refused_pack_is_not_retried` is rewritten
to the new rule (attempts settle; some refused); the old attempt counts and peak step are gone.

### Gates run locally (all pass)

Lint job (fmt, clippy `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc`, every policy script
including `check-lane-policy.sh`); the workspace debug test step with its excludes and features;
`scripts/run-wasm-gates.sh`; the AudioWorklet artifact gates and the SDK headless evals.

### W=4 re-pin (`tools/wasm-gates/src/lib.rs`, `c1_console_16_typical`)

`[15, 0, 3584, 2304, 260, 3580, 1]` -> `[15, 0, 3328, 2048, 260, 3836, 1]`: the packer sorts the
fixed draw from bind, so the track group runs 8 bank-slots per block instead of 9 (256 blocks) and
the freed slot-block is a settled-bypass skip. `w4_counters_describe_the_designs_plan` passes. The
four-lane native leg (arm64) was not run here.

### Bit identity against main `7be08dd18` (one-time, not committed)

All 16 `BANK_REPORT_WORKLOADS` rows, 600 blocks through `console_workload::SessionRuntime`, FNV-1a
of every host word: identical on all 16.

### BSR and CPU (descriptive; `artifacts/steps/bank-design-5b/`, taskset -c 31, AMD EPYC 7313P, one warmup, two rounds, lower round's p50)

| row | BSR/block main -> branch (key-class ideal) | p50 us main -> branch |
|---|---|---|
| C1-16 Load | 0 -> 0 | 21.26 -> 20.85 |
| C1-16 Typical | 10.00 -> 10.00 (9) | 63.33 -> 61.84 |
| C1-16 AllActive | 16.66 -> 16.66 | 97.04 -> 97.03 (0.0 %) |
| C1-16 Sparse | 13.60 -> 13.60 | 78.91 -> 78.61 |
| C1-32 Load | 0 -> 0 | 32.18 -> 32.21 |
| C1-32 Typical | 19.00 -> 17.00 (12) | 106.32 -> 94.05 (-11.5 %) |
| C1-32 AllActive | 24.66 -> 24.66 | 147.02 -> 145.26 (-1.2 %) |
| C1-32 Sparse | 18.54 -> 18.55 | 110.34 -> 108.99 |
| C1-64 Load | 0 -> 0 | 57.32 -> 56.30 |
| C1-64 Typical | 29.00 -> 24.00 (17) | 183.72 -> 149.90 (-18.4 %) |
| C1-64 AllActive | 40.66 -> 40.66 | 244.11 -> 242.67 (-0.6 %) |
| C1-64 Sparse | 28.43 -> 28.46 | 171.63 -> 173.16 (+0.9 %) |

The amendment's "20.00" was #2132's static count: main already executes 10/19/29 on `Typical`
(#2142's grouping and S1b's skip). The track group at 16 tracks is already at its ideal (5), so
16 does not move. 32 and 64 land at 17 and 24, just above the amendment's estimates (12-16,
17-23); the gap to the key-class ideal is the lexicographic sort itself (compressor-major, so the
EQ, gate and drive runs split across the compressor boundary), which is the bound the design
accepts (3.3), not an implementation gap.

### Attempt 1 verdict (bounded opus-high verify, 2026-10-27): PASS

- No BLOCKER or MAJOR finding. The widened rack swap rule (a channel-less bypassed lane trades with an uncontrolled lane) moves every per-lane part with the strip; no bit-moving case was found. An extra verifier mutant (the cross-stage swap leaves the shunt line and `zero_run` behind) turns gate 1's static arm red.
- The refusal rule terminates: each refusal fixes one new strip, so an episode has at most `span` refusals. Refusals reset only on a new fact.
- Realtime: the pack, dwell and settle paths allocate nothing and make no syscalls; all tables are boxed at bind.
- MINORs, recorded and not folded:
  1. Each visited dirty group is sorted in full, and each refusal re-sorts. The per-block sort work is bounded (at most 4W sorts) but grows with plan size, not only with `B_total`. This goes to the performance pass.
  2. Gate 1 feeds the same signal to every track; strip diversity comes from the parameters and the draws.
  3. The `settle_edges` swap in rack `swap_lanes` is dead, because the next drain rewrites it. It is harmless.
  4. A plan carry during a hold and a displaced group's fold are left to existing tests (#1782a gate 3, #1981 gate 1).
- Known scope limit: stored effect-bypass automation does not render yet (`controls::automation_row_renders` covers fader, mute and VCA only), so the automated-bypass pin holds only static and live-written bypass. That is conservative: it can cost a pack, never a bit.
