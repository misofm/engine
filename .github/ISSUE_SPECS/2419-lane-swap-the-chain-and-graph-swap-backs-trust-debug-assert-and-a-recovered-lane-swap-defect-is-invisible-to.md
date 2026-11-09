# Lane swap: the chain and graph swap-backs trust `debug_assert!`, and a recovered lane-swap defect is invisible to the host

Successor of #2414. That issue made `effect_contract::swap_bank_lanes` check every exchange and every undo. It also made `rack` recover a swap that strands: the stage resets both lanes, counts the event in `BankStage::lane_swap_stranded`, and the chain returns `ChainLaneSwapRefusal::Defect`. Sol's review of #2414 named four things one level up that #2414 did not own. This issue owns them. Every line cite below is at commit `754357225`.

## Problem
1. **The chain's and the graph's swap-backs are asserted, not checked.**
   - `BankChain::swap_lanes` (`crates/rack/src/lib.rs` line 5330 (in-chain) and line 5371 (cross-chain)) swaps back the slots it has already swapped after a slot refuses. Each swap-back's result goes only to `debug_assert!(undone, "a swapped slot swaps back")`.
   - The graph's `Runtime::swap_bank_lanes` (`crates/graph/src/runtime.rs` line 11577) does the same per row: `debug_assert!(undone.is_ok(), "a traded row trades back")`.
   - In a release build, a slot that refuses its swap-back leaves that slot holding the two strips' lanes traded while the earlier slots are swapped back. The chain still returns `Defect`, whose doc says "the slots already swapped were swapped back". The graph still returns a refusal, its map unchanged.
   - The result is a **split strip**: one strip's state is spread over two lanes, part in each. Nothing reports it.
2. **The graph still has a `debug_assert!(false)` on its defect path.** `crates/graph/src/runtime.rs` line 11564: `debug_assert!(false, "a checked row trades its lanes")`.
   - #2414 removed rack's twin of this assert and replaced it with a tripwire that fires only on a refusal no stage counted.
   - So a debug build of any graph test that strands a slot panics here, while the rack recovers and counts the same defect.
3. **A recovered defect is invisible to the host, and the packer keeps hitting it.**
   - `lane_swap_stranded` is per stage. Nothing sums it the way `graph::carry_refusal_count` (`crates/graph/src/lib.rs` line 5122) sums refused carries.
   - The packer marks a refused trade only for the current episode (`pack_group_step`). A persistently broken bank is therefore asked again, and its two lanes reset again, at every new episode.
4. **The exchange's trait doc contradicts the launch banks.** `LaneSwap::exchange_lane_defaults` (`crates/effect-contract/src/lib.rs` line 2526) says "It changes no state word and no payload word".
   - A bank whose prepared defaults carry a lane's detector link seats that lane's link weights at rest inside the exchange. For example, `rest_link` in `crates/gate-expander/src/lib.rs` and in the de-esser, dynamic EQ, limiter, compressor and transient shaper banks.
   - That ends a glide in flight, which changes a state word.
   - #2414 made the swap restore each lane's own payload after every undo, so the swap no longer depends on the claim. The doc is still false.

## Contract
- **A swap-back is checked.**
  - The chain and the graph treat a swap-back refusal as a distinct outcome, never as `Defect`.
  - That outcome names the slot or row left traded: a split strip.
  - The caller recovers the split strip's lanes in that slot or row, without assuming the earlier slots were swapped back. One option is the #2414 reset: `reset_lane(FullToDefaults)` on both lanes, with their symmetry terms withheld.
  - Which recovery is used is the implementer's choice, recorded with its reason.
  - A `debug_assert!` may stay only beside a check.
- **The graph's defect path is testable in debug.** The `debug_assert!(false)` at line 11564 becomes a tripwire on refusals nobody counted, as rack's is.
- **The host can see a recovered defect.**
  - A host-facing total sums `lane_swap_stranded` over every bank stage, beside `carry_refusal_count`.
  - The same applies to any split-strip count this issue adds.
  - On `Chain(Defect)`, the graph quarantines both strips for the plan's life with the existing #2135 pin (`LaneMap::pinned`), so the packer never asks the broken bank again. The plan-carry home walk (`restore_home_lanes`) keeps its own contract.
- **The trait doc is true.** `exchange_lane_defaults`'s doc says what an exchange may change: a state word that the lane's own payload restores. It then states the swap's obligation to restore the payloads. Alternatively, the banks stop touching state inside the exchange, and no rendered bit moves.
- **Realtime.**
  - No allocation, lock or syscall.
  - Every new branch sits inside a `REALTIME_POLICY` region.
  - No behaviour change on a healthy swap: no rendered bit moves.

## Non-goals
- Making an exchange or a swap infallible by type.
- The limiter's history/main-ring tie (#2393).
- Third-party banks.

## Gates
- **Red on base.** Each case runs on a rack test bank whose swap (exchange or restore) refuses on chosen calls, like #2414's `StripBank` `Fault`:
  - a chain slot that refuses its swap-back, in an in-chain and in a cross-chain swap;
  - a graph row that refuses its trade-back;
  - a graph trade that strands a slot. On base this panics in debug at line 11564, and in release the packer retries it at the next episode.

  Record the output of debug and release builds.
- **Host total.** A test that strands a swap reads a nonzero host-facing total.
- **Mutation evidence.** Each new check is reverted to its assertion, and each test turns red.
- **No bit moved.** Run `graph-compiler`'s `silent_bank_skip` gate-3 digest once against base (#2392's instrumentation: base `7d31fea2b6652670` / `8e4dfb8b8dc1d599`). This is PR evidence, not a committed digest.
- **Gates for every touched crate.**
  - `cargo test --locked`.
  - Clippy `-D warnings`, `cargo fmt --check` and `cargo doc -D warnings`.
  - `scripts/check-workspace-policy.sh` and `realtime_policy`.
  - The call-graph gate (`scripts/check-web-audioworklet.sh`) on a rebuilt artifact, if render dispatch changes.

## Evidence / attempt record
### Attempt 1 (Terra), branch `codex/issue-2419-lane-swap-split-strip`, base `0c9a85b10`

**Claims re-verified at base `0c9a85b10`.** Every claim holds; none is dropped. The line cites of `754357225` are unchanged at the base:
- (a) `BankChain::swap_lanes` (`crates/rack/src/lib.rs` 5333 in-chain, 5376 cross-chain) sends each swap-back's result only to `debug_assert!(undone, "a swapped slot swaps back")`; the graph's `Runtime::swap_bank_lanes` (`crates/graph/src/runtime.rs` 11577) sends each row's trade-back only to `debug_assert!(undone.is_ok(), "a traded row trades back")`.
- (b) `crates/graph/src/runtime.rs` 11564 is `debug_assert!(false, "a checked row trades its lanes")`.
- (c) No total sums `lane_swap_stranded` (`graph::carry_refusal_count`, `crates/graph/src/lib.rs` 5122, sums only refused carries), and `pack_group_step` marks a refused trade only in `refused`, which a fresh episode clears.
- (d) `LaneSwap::exchange_lane_defaults` (`crates/effect-contract/src/lib.rs` 2526) says "It changes no state word and no payload word", while `exchange_defaults` in `crates/gate-expander/src/lib.rs` 1946 calls `rest_link`, as the de-esser, dynamic EQ, limiter, compressor, multiband compressor and transient shaper banks do.

**Red on base, both builds.** A one-time probe on the base sources (the branch's test scenarios, adapted to the base API; scratch only, not committed):

| Case | Debug build | Release build |
| --- | --- | --- |
| Chain: slot 1 refuses its swap-back after slot 2 strands, in-chain `(0,1)/(0,3)` | panic "a swapped slot swaps back" (`lib.rs:5333`) | `Err(Defect)`; slot 1 left with the two strips' defaults traded; no lane of slot 1 reset |
| Chain: the same, cross-chain `(0,1)/(1,2)` | panic, same | the same |
| Chain: slot 0 refuses its swap-back after slot 1 strands, in-chain and cross-chain | panic, same | the same, for slot 0 |
| Graph: row 2 strands, then the trade-back of rows 1 and 0 strands, in-unit `(0,1)/(0,2)` | panic "a checked row trades its lanes" (`runtime.rs:11564`) | `Err(Chain(Defect))`; units 0 and 2 (rows 0 and 1) left with their member ops and source bit traded; nothing pinned |
| Graph: row 2 strands, then row 0's trade-back strands, cross-unit `(0,1)/(1,1)` | panic, same | `Err(Chain(Defect))`; units 0 and 1 (row 0) left with their readers traded; nothing pinned |
| Graph: a pack-group trade whose row 1 strands, then a new episode | panic, same | episode 1: 1 attempt, 0 moves; episode 2: the packer asks the same banks again, 5 attempts and 4 moves in all (row 1's bank swapped 5 times); nothing pinned |

The branch's tests (below) assert the documented outcome of each case in both builds.

**Fix.**
- **(a) Checked swap-backs, at the chain.** Each swap-back in both arms of `BankChain::swap_lanes` is a checked branch. A slot that refuses its swap-back has both lanes reset through a new `BankStage::reset_swapped_lanes`, is counted in a new chain counter, and the swap-back goes on with the slots below it. The call returns a new `ChainLaneSwapRefusal::Split { slot }`, naming the highest such slot, never `Defect`. Both `debug_assert!(undone, ..)`s are gone.
- **(a) Checked trade-backs, at the graph.** Each trade-back in `Runtime::swap_bank_lanes` is a checked branch. A row that refuses its trade-back is recovered by `recover_unit_lanes`: both lanes are reset in every slot of the row's chains (a new `BankChain::reset_swapped_lanes`, counted), and every reader that moved with the strip (member ops, fold entry, source bit, observation rows) is traded back. The trade-back goes on with the rows below it, and the call returns a new `BankLaneMoveRefusal::Split { row }`, naming the highest such row. The map is unchanged. `swap_unit_lanes` and `recover_unit_lanes` share one body, `trade_unit_lanes`, which takes a `RowChains` (`Trade` or `Reset`) for the chain step and always trades the readers. The `debug_assert!(undone.is_ok(), ..)` is gone.
- **The recovery, and why.** It is the #2414 strand recovery: `reset_lane(FullToDefaults)` on both lanes, with `RESTORED` withheld. `EffectBankStage::recover_lane` and `LiveControlEffectBankStage::recover_lane` are now the one body that both the strand branches (four arms) and `reset_swapped_lanes` call, so `swap_stage_bank_lanes` no longer resets. Why a reset:
  - a split slot holds the other strip's history (envelopes, delay lines), which would otherwise render into this strip's audio; the reset ends it at once;
  - the reset makes each lane's effect state a function of the defaults it holds, as #2414 argued for a strand;
  - the graph cannot tell which slots of a refusing row still hold the strips traded, so it resets every slot of that row.

  What a reset does not undo, documented on `Split`: a split slot may keep the other strip's prepared defaults and its non-effect per-lane parts (control lane, taps, dry-shunt line) until a plan rebuild prepares them fresh. The pin and the count bound and expose that residue.
- **(b)** The graph's `debug_assert!(false)` is now a tripwire beside the checks. It fires only on a refusal after the check passed that no chain recovered and counted, that is, anything but `Chain(Defect | Split)`. A recovered defect runs in debug, as rack's does.
- **(c) Host total and pin.**
  - `BankChain::lane_swap_defects()` sums every slot's `lane_swap_stranded` and the chain's split count.
  - `Runtime::lane_swap_defect_count` sums it over the bank units.
  - `graph::lane_swap_defect_count(plan)`, beside `carry_refusal_count` and the same shape, is the host-facing total. It is in-process only: `carry_refusal_count` is deliberately off the wire, and a wire `CounterId` is a protocol and C-ABI surface, which needs an owner ruling (follow-up below).
  - Every refusal after the check passed pins both strips for the plan's life through a new `LaneMap::pin_positions` (the #2135 `pinned` flag, by home position), so the packer never asks for these two strips again. Other strips of the same banks stay movable. `restore_home_lanes` is unchanged; it does not read pins.
- **(d)** `exchange_lane_defaults`'s doc now says what an exchange may change: a state word that seats the incoming defaults and that the lane's own payload holds (`rest_link`, a glide in flight ended), and nothing else but a collapsed bank's disengage copy (#2352), which moves no bit. It then states the caller's obligation, which `swap_bank_lanes` meets since #2414: restore the payloads afterwards.
- **Realtime.** No allocation, lock or syscall. Every new branch and method on the render path is inside an existing or new `REALTIME_POLICY` region: the chain's swap-backs and `reset_swapped_lanes`, both stages' `reset_swapped_lanes` and `recover_lane`, and the graph's row code and `pin_positions`. Nothing new runs on a healthy swap but one `RowChains` match per row.

**Tests.**
- `rack` `lane_swap::a_slot_that_refuses_its_swap_back_splits_the_strip_and_is_reset`. Six cases:
  - an effect slot (0) and the live slot (1) refuse their swap-back, each in an in-chain and a cross-chain swap;
  - both refuse at once, in-chain and cross-chain.

  Each case asserts `Split { slot }` naming the highest slot, the counts (`lane_swap_defects` = strand + splits), exactly the refusing slots' two lanes reset in their own banks, `RESTORED` withheld on each split lane (per stage), and every other slot swapped back while a split slot keeps the defaults traded.
  - Test value: a chain that takes a refused swap-back for a done one, stops at the first refusal, resets the wrong slot, lane or bank, keeps `RESTORED`, does not count, or names the lowest split slot.
- `graph` `runtime::tests` (a hand-built runtime, `SwapRig`: one pack group of two lane chains by three rows, each unit a real `BankChain` of one `FaultyStage`, whose swap strands, or whose check refuses, on chosen calls):
  - `a_row_that_refuses_its_trade_back_is_recovered_as_a_split_strip`: an in-unit trade (rows 1 and 0 both refuse) and a cross-unit trade (row 0 refuses). It asserts `Split { row }` naming the highest row, the host total, both strips and only they pinned, the reset count of every unit, every reader home, the rows that traded back holding their own state, and the map unchanged.
    - Test value: a trade that takes a refused trade-back for a done one, stops at the first refusal, leaves a recovered row's readers traded or its lanes unreset, names the lowest row, does not pin, or does not count.
  - `a_stranded_trade_pins_both_strips_for_the_plan`: the packer's own trade strands. It asserts `Chain(Defect)` with no panic in debug, a host total of 1, both strips pinned, and that a fresh episode makes no attempt and no swap.
    - Test value: the base's debug panic, a missing pin (a broken bank reset again at every episode), or a total that misses a stage's strand.
  - `an_unrecovered_row_refusal_trips_the_trade_in_debug` (debug only, `#[should_panic]`): a chain whose own check refuses where the trade's passed.
    - Test value: the graph tripwire dropped.
- The stage of the graph tests is a stand-in, not rack's `StripBank`: `StripBank` lives in rack's integration test, and the graph code under test sees only the `BankChain` outcomes, which here come from a real `BankChain`. The stage-level faults are rack's `StripBank` cases.
- No existing test is made useless. The #2414 tests now also cover the shared `recover_lane`.

**Mutation sweep** (`sweep.py` in scratch). Each mutation was applied alone; the focused tests (`rack --test lane_swap` and/or the four graph tests) ran in debug and in release; the tree was restored.

All 49 mutants are red in both builds, with two exceptions: the tripwire mutant (red in debug, green in release, because it is compiled out by design) and one equivalent mutant. "x2" means both arms were mutated at once; every x2 mutation was also run per arm, and each arm alone is red in both builds.

| Mutation | Where | Debug | Release | Red at |
| --- | --- | --- | --- | --- |
| C1: swap-back check dropped, in-chain arm and cross-chain arm (each alone) | chain | red | red | `a_slot_that_refuses_its_swap_back_...` |
| C2: split slot's reset dropped, each arm | chain | red | red | the same |
| C3: split slot's count dropped, each arm | chain | red | red | the same |
| C4: the lowest split slot named, not the highest; x2, and each arm alone | chain | red | red | the same (double-split cases) |
| C5: the swap-back stops at the first refusal; x2, and each arm alone | chain | red | red | the same |
| C6: in-chain reset of the calling lane twice | chain | red | red | the same |
| S1: the effect stage's `recover_lane` resets nothing, or keeps `RESTORED` | stage | red | red | the same and `a_stranded_slot_...` |
| S2: the live stage's `recover_lane` resets nothing, or sets no `unrestored` bit | stage | red | red | the same and `a_stranded_slot_...` |
| S3: `reset_swapped_lanes` drops its own lane, recovers the wrong in-chain lane, or drops the cross-chain peer; x2, and each stage alone | stage | red | red | `a_slot_that_refuses_its_swap_back_...` |
| S4: the strand branch recovers the peer's lane in its own bank, or one in-chain lane; x2, and each stage alone | stage | red | red | `a_stranded_slot_...` or `a_slot_that_refuses_...` |
| K1: `BankChain::reset_swapped_lanes` skips its in-chain arm, or its cross-chain arm | chain | red | red | `a_row_that_refuses_its_trade_back_...` |
| K2: `BankChain::reset_swapped_lanes` does not count | chain | red | red | the same |
| K3: `lane_swap_defects` drops the stranded term, or the split term | chain | red | red | rack and graph tests |
| G1: trade-back check dropped | graph | red | red | `a_row_that_refuses_its_trade_back_...` |
| G2: no row recovery | graph | red | red | the same |
| G3: the lowest split row named | graph | red | red | the same |
| G4: the trade-back stops at the first refusal | graph | red | red | the same |
| G5: no pin, or only one strip pinned | graph | red | red | both graph trade tests |
| G6: tripwire disabled | graph | red | green, compiled out by design | `an_unrecovered_row_refusal_trips_the_trade_in_debug` |
| G7: the row reset skips the in-unit chain, or the cross-unit chains | graph | red | red | `a_row_that_refuses_its_trade_back_...` |
| G8: the recovery does not trade the readers back | graph | red | red | the same |
| G9: `Runtime::lane_swap_defect_count` returns 0 | graph | red | red | both graph trade tests |

Equivalent mutants, green in both builds:
- S2-designed: the live stage's `designed` refresh in `recover_lane` dropped. This is #2414's N7. While `RESTORED` is withheld, every reader of the witness declines, and the next restore recomputes `designed`.

Branches with no reachable mutant:
- The `get_mut(lane)` guards in both `recover_lane`s and in `pin_positions`, and the `downcast` guard of the cross-chain peer in both stages' `reset_swapped_lanes`. These keep the methods panic-free. Every caller passes lanes and positions it has checked, and a peer the chain's check proved is of the same type.
- `recover_unit_lanes`'s ignored `Err`. It is only a unit that is not a bank, and this row's units have just traded forward.

**No bit moved.** The #2392 instrumentation (`graph-compiler`'s `silent_bank_skip` gate 3, an FNV-1a digest over the moved plan's host PCM and both plans' observed words, `--release`): base `0c9a85b10` `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`; branch identical. This is PR evidence only, not committed.

**Gates.** Run one crate at a time with `-j 4` and one private target dir, deleted afterwards:
- `cargo test --locked`: every run below passed, with none failing (113 result lines, all ok):
  - `effect-contract`, `rack`, `graph`, `graph-compiler` and `host-core`, all in debug. The last two cover the struct-size change, a `u64` on `BankChain`; no fixture moved.
  - `rack --release --test lane_swap`.
  - `graph --release --lib` over the four new tests.
- Clippy `-D warnings` on `effect-contract`, `rack` and `graph` (all targets), in debug and release: clean.
- `cargo doc -D warnings` on the same three crates: clean, after the docs stopped linking the private `recover_lane`s.
- `cargo fmt --all --check`: clean.
- `check-workspace-policy.sh`: ok.
- `check-rack-policy.sh`: PASS (no `graph::` in rack source).
- `realtime_policy`: ok (314 marked regions in 48 files).
- `check-web-audioworklet.sh --without-metadata-regeneration`, on a rebuilt artifact and named twin with wabt 1.0.34: passed ("web AudioWorklet static/object checks passed"). `miso_engine_web_v1_render` has closure=38, as on #2414; atomic_waits=0, destructor_registration=0.

**Follow-ups (not filed).**
- A wire `CounterId` for `lane_swap_defect_count` (and `carry_refusal_count`) needs an owner ruling: both are in-process reads today.
- `graph::lane_swap_defect_count`'s downcast wrapper is reached by no nonzero test. `Runtime::lane_swap_defect_count`, which it returns, is tested. *Corrected in attempt 3:* no production hook is needed. A graph `lib.rs` unit test moves the lane-swap rig's runtime into a bound plan.
- A split slot's residue (the other strip's prepared defaults and its control lane, taps and dry-shunt line) lasts until a plan rebuild. A host that reads a nonzero total could request one; nothing does yet.

**Sol's verdict on attempt 1 (`d4fd3f88e`): FAIL**, with two MAJOR and four MINOR findings, plus a request for successor specs (item 6).
1. MAJOR: the graph's row recovery left the healthy slots of a split row fully traded.
   - A row's trade-back was the chain's own `swap_lanes`. When a slot stranded, the chain "swapped back" the earlier slots, which traded them forward again.
   - `RowChains::Reset` then reset their effect state, but left their defaults, control lane, taps and shunt traded. A `BuiltinStage` (the no-op default) kept its whole lane traded.
   - Sol's probe: two `FaultyStage` slots per unit, `(0,1)/(0,2)`, row 2's slot 1 strands forward, row 1's slot 1 strands on its trade-back. Result: `Split { row: 1 }` with unit 2's healthy slot 0 at `[32,34,33,35]`.
2. MAJOR: untested checks, each green in debug and release:
   - N1: `pin_positions` by position, not by home;
   - N2: `base` dropped;
   - N11: group forced to 0;
   - N3: the tripwire's `Chain(Split)` arm removed;
   - N4: the cross-unit reset used `high_lane`;
   - N8: the total summed the even units only.
3. MINOR: rack's `Split` doc said "no slot holds the other strip's history". The dry-shunt line and the builtin no-op contradict it.
4. MINOR: no test reaches the pub `graph::lane_swap_defect_count` on a real plan.
5. MINOR: the docs should say the total counts recoveries, not strips.
6. Write the follow-ups as ready-to-file successor specs.

### Attempt 2 (Sol's findings folded), same branch, origin/main `cf108f5b1` (#2423) merged

**Item 1, the row trade-back.**
- `rack` gains `BankChain::swap_lanes_back(lane, peer, scratch)`, a checked best-effort swap-back.
  - It asks every slot, last first, and unwinds nothing.
  - A slot that refuses has both lanes reset (`BankStage::reset_swapped_lanes`) and is counted, and the slots below it are still swapped back.
  - It returns `Err(Split { slot })` naming the highest such slot.
  - It and `swap_lanes`'s own swap-back are one body, `swap_slots_back(end, ..)`.
- `BankChain::reset_swapped_lanes`, the attempt-1 whole-row reset, is removed.
- The graph trades each row back with `swap_unit_lanes_back`:
  - `RowChains::Back` calls `swap_lanes_back`;
  - the row's readers are traded back whatever the slots did;
  - `RowChains::Reset` and `recover_unit_lanes` are gone.
- So after a split, every slot that swapped back is home, readers included. Only a refusing slot keeps what a reset leaves.
- The `BankLaneMoveRefusal` and `Split` docs say this.
- Test: `a_row_that_refuses_its_trade_back_is_recovered_as_a_split_strip` now runs on a two-slot rig (`SwapRig::new(2)`): a healthy slot 0 beside a broken slot 1. It asserts, among the rest:
  - every unit's slot 0 tags are home, which is Sol's probe made a committed assertion;
  - slot 0 was never reset.
- Red on attempt 1's design: G7, the trade-back through the unwinding `swap_lanes` (below), turns it red.

**Item 2, the untested checks.**
- `a_defect_pins_the_strips_at_its_positions_by_home` runs on the `LaneMap` alone. It uses two groups, the second at `base` 8. A cycle of two moves displaces the strips, so neither of the trade's positions holds its own strip (the test asserts this first). `pin_positions` must pin exactly the two home positions. Red for N1, N2 and N11.
- `a_row_its_chain_splits_is_a_recovered_defect`: row 1's chain returns `Chain(Split { slot: 0 })` on the forward trade (slot 1 strands, then slot 0 strands its swap-back). It asserts:
  - no debug panic;
  - row 0 traded back;
  - both strips pinned;
  - a total of 3.

  Red for N3.
- The split-row test gains a cross-unit trade of different lanes, `(0,1)/(1,2)` (red for N4 and its `swap_lanes_back` twin), and an in-unit trade on chain 1, `(1,1)/(1,3)`, whose counts land on the odd units (red for N8).

**Item 3.** Rack's `Split` doc now says what a reset ends and what stays: the other strip's prepared defaults; its control lane, taps and dry-shunt line (up to a latency of its dry input); and the whole lane of a stage with no lane reset, such as a builtin bank's. `BankStage::reset_swapped_lanes`'s doc says the same.

**Item 4.** `graph-compiler`'s `silent_bank_skip` gate 3 now asserts `graph::lane_swap_defect_count(plan) == Some(0)` on both moved plans, after their healthy forced swaps. This covers the downcast. A `None` wrapper is red (N5b). `Some(0 * ..)` (N5) stays green: a healthy plan reads zero, and a nonzero count through the wrapper still needs the fault hook that is out of this issue's regions.

**Item 5.** The docs of `BankChain::lane_swap_defects`, `Runtime::lane_swap_defect_count` and `graph::lane_swap_defect_count` say the total counts recoveries, not strips: one slot that strands on its swap-back counts as a strand and as a split.

**Item 6.** Two successor specs, ready to file, are in the scratchpad:
- `succ/bb.md`, #2424: a plan rebuild on a nonzero lane-swap defect total, which is the residue's correct end state.
- `succ/cc.md`, #2425: an owner ruling on putting `lane_swap_defect_count` and `carry_refusal_count` on the wire.

The follow-ups below name them.

**Mutation sweep, attempt 2** (`sweep2.py` in scratch, rewritten for the new code). Each mutation was applied alone; the focused tests (rack `--test lane_swap`, the six graph tests, and for the wrapper `silent_bank_skip`'s gate 3) ran in debug and in release; the tree was restored. Sol's N1-N11 are included.

Of 54 mutants, 50 are red in debug and release. The four that are not are listed at the end. A mutation in `swap_slots_back` runs both the rack and graph tests, because the chain's own swap-back and `swap_lanes_back` share it.

| Mutation | Where | Debug | Release | Red at |
| --- | --- | --- | --- | --- |
| C1: the swap-back check dropped, each arm | `swap_slots_back` | red | red | `a_slot_that_refuses_...`, `a_row_that_refuses_...` |
| C2: a refusing slot's reset dropped, each arm | `swap_slots_back` | red | red | the same |
| C3: its count dropped, each arm | `swap_slots_back` | red | red | the same |
| C4: the lowest slot named, each arm | `swap_slots_back` | red | red | `a_slot_that_refuses_...` |
| C5: stop at the first refusal, each arm | `swap_slots_back` | red | red | both |
| C6: in-chain reset of the calling lane twice | `swap_slots_back` | red | red | both |
| N6: the forward slot named instead of the refusing one | `swap_slots_back` | red | red | `a_slot_that_refuses_...`, `a_row_its_chain_splits_...` |
| C7: `swap_lanes_back` skips the last slot | chain | red | red | `a_row_that_refuses_...`, `a_row_its_chain_splits_...` |
| C8: `swap_lanes_back` always `Ok` | chain | red | red | `a_row_that_refuses_...` |
| C9: the chain's own swap-back skipped, each arm | chain | red | red | rack and graph tests |
| S1: the effect stage's `recover_lane` resets nothing, or keeps `RESTORED` | stage | red | red | `a_slot_that_refuses_...`, `a_stranded_slot_...` |
| S2: the live stage's `recover_lane` resets nothing, or sets no `unrestored` bit | stage | red | red | the same |
| S3: `reset_swapped_lanes` drops its own lane, recovers the wrong in-chain lane, or drops the cross-chain peer; each stage alone | stage | red | red | `a_slot_that_refuses_...` |
| S4: the strand branch recovers the peer's lane in its own bank, or one in-chain lane; each stage alone | stage | red | red | `a_stranded_slot_...`, `a_slot_that_refuses_...` |
| K3: `lane_swap_defects` drops the stranded term, or the split term | chain | red | red | rack and graph tests |
| G1: the trade-back check dropped | graph | red | red | `a_row_that_refuses_...` |
| G3: the lowest split row named | graph | red | red | the same |
| G4: the trade-back stops at the first refusal | graph | red | red | the same |
| N7: the forward row named | graph | red | red | the same |
| G5: no pin, or one strip pinned | graph | red | red | four graph tests |
| N10: pin on `Split` only | graph | red | red | `a_stranded_trade_...`, `a_row_its_chain_splits_...` |
| N1: pin by position, not by home | graph | red | red | `a_defect_pins_the_strips_at_its_positions_by_home` |
| N2: pin without `base` | graph | red | red | the same |
| N11: pin in group 0 | graph | red | red | the same |
| G7: the trade-back through the unwinding `swap_lanes`, the attempt-1 shape; each arm | graph | red | red | `a_row_that_refuses_...` |
| N4: the cross-unit trade-back swaps `high_lane` | graph | red | red | the same |
| G8: the readers not traded back, each arm | graph | red | red | the same (and `a_row_its_chain_splits_...`) |
| G9: `Runtime::lane_swap_defect_count` returns 0 | graph | red | red | the same and two more |
| N8: the total sums the even units only | graph | red | red | `a_row_that_refuses_...` (the chain-1 case) |
| N5b: the pub wrapper returns `None` | graph `lib.rs` | red | red | `silent_bank_skip::a_moved_strip_renders_the_bits_of_an_unmoved_one` |
| G6: the tripwire disabled | graph | red | green, compiled out by design | `an_unrecovered_row_refusal_trips_the_trade_in_debug` |
| N3: the tripwire's `Chain(Split)` arm removed | graph | red | green, compiled out by design | `a_row_its_chain_splits_is_a_recovered_defect` (debug panic) |

The four that are not red in both builds:
- G6 and N3 are tripwire mutants. They are red in debug and compiled out of release by design.
- S2-designed, the live stage's `designed` refresh dropped, is green in both builds. It is equivalent: #2414's N7.
- N5, the pub wrapper returning `Some(0 * ..)`, is green in both builds. A healthy real plan reads zero, and a nonzero read through the wrapper needs the fault hook in the bound-plan code.

**No bit moved.** The #2392 gate-3 digest on the merged branch is `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`. This is identical to base `0c9a85b10` and to attempt 1. The healthy trade path changed only by `trade_unit_lanes`' `RowChains::Trade` arm returning `Ok` into the readers step. This is PR evidence, not committed.

**Gates, on the merged tree.** Run one crate at a time with `-j 4` and one private target dir, deleted afterwards:
- `cargo test --locked`: every run below passed, with none failing (113 result lines, all ok):
  - `effect-contract`, `rack`, `graph`, `graph-compiler` and `host-core`, all in debug;
  - `rack --release --test lane_swap`;
  - `graph --release --lib` over the new graph tests;
  - `graph-compiler --release --test silent_bank_skip` (14).
- Clippy `-D warnings` on `effect-contract`, `rack` and `graph` (all targets), in debug and release: clean.
- `cargo doc -D warnings`: clean.
- `cargo fmt --all --check`: clean.
- `check-workspace-policy.sh`: ok.
- `check-rack-policy.sh`: PASS.
- `realtime_policy`: ok (314 marked regions in 48 files).
- `check-web-audioworklet.sh --without-metadata-regeneration`, on a rebuilt artifact and named twin with wabt 1.0.34: passed. `miso_engine_web_v1_render` has closure=38, atomic_waits=0, destructor_registration=0.

**Follow-ups (not filed).**
- #2424: a plan rebuild on a nonzero lane-swap defect total (`succ/bb.md`).
- #2425: an owner ruling on a wire `CounterId` for `lane_swap_defect_count` and `carry_refusal_count` (`succ/cc.md`).
- *Corrected in attempt 3:* a nonzero test through the pub `graph::lane_swap_defect_count` wrapper needed no fault hook. It is `the_lane_swap_defect_total_reads_the_plans_runtime`, and N5 is now red.

**Sol's verdict on attempt 2 (`20f4b3a2a`): FAIL**, with one MAJOR and two MINOR findings. The code was judged correct, and all 12 of the reviewer's mutants were red.
1. MAJOR: kill N5 in this issue. No production hook is needed (#2417 has merged). A graph `lib.rs` test can move the lane-swap rig's runtime into a bound plan.
2. MINOR: the pin docs overclaim "never asks the broken banks again". Only the two strips are pinned.
3. MINOR: two corrections to `succ/bb.md`:
   - the within-plan bound;
   - which strip's live writes land in the residual control lane.

### Attempt 3 (Sol's attempt-2 findings folded), same branch, at origin/main `cf108f5b1`

- **Item 1.** `graph` `runtime::tests` is now `pub(crate)`, and so are `SwapRig` (`runtime`, `faults`, `new`), `SwapFault` and its `strand`.
  - A new graph `lib.rs` test, `the_lane_swap_defect_total_reads_the_plans_runtime`:
    - strands row 1 of a rig trade;
    - moves the rig's runtime into `sidechain_pdc_plan(false)` (`core::mem::swap` on `bound.executor().runtime`);
    - seals it and asserts `lane_swap_defect_count(&mut plan) == Some(1)`.
  - Test value: a wrapper that reads anything but the plan's runtime total.
  - Red on mutation, in debug and release: N5 (`Some(0 * ..)`) and N5b (`None`).
  - It supersedes the attempt-2 `Some(0)` assertion in `silent_bank_skip` gate 3, which only covered the downcast. That assertion is deleted, under the one-test-per-claim rule.
  - The attempt-1 and attempt-2 follow-up lines that called this impossible are corrected in place.
- **Item 2.** The `pin_positions` doc, the `swap_bank_lanes` doc, and the pin test's doc and message now say the packer never asks for **these two strips** again. `pin_positions` adds that other strips of the same banks stay movable and may still hit the defect, once per pair.
- **Item 3.** `succ/bb.md` now:
  - bounds the defects within one plan at one per pair of unpinned strips (at most `n/2` per group of `n`), plus the cross-plan bound the successor must state;
  - says the lane map is unchanged, so this strip's own live writes land in a control lane holding the other strip's cells, and the cells it never writes keep the other strip's values.

**Mutation evidence, attempt 3.** Production changed only in doc comments and in the `pub(crate)` visibility of test items, so the attempt-2 sweep stands. The wrapper mutants were rerun against the new test:
- N5: red in debug and release (`the_lane_swap_defect_total_reads_the_plans_runtime`);
- N5b: red in debug and release (the same test).

With N5 red, the only mutants not red in both builds are:
- the two tripwire mutants (G6, N3), compiled out of release by design;
- the equivalent `designed` refresh (S2-designed).

**No bit moved.** No render code changed since attempt 2's digest (`7d31fea2b6652670` / `8e4dfb8b8dc1d599`).

**Gates**, one crate at a time with `-j 4` and one private target dir, deleted afterwards. Every run passed (21 result lines, all ok, none failing):
- `cargo test --locked -p graph`, in debug and in release;
- `cargo test --locked -p graph-compiler --test silent_bank_skip`;
- clippy `-D warnings` on `rack`, `graph` and `graph-compiler` (all targets), in debug and release: clean;
- `cargo doc -D warnings` on `rack` and `graph`: clean;
- `cargo fmt --all --check`: clean;
- `check-workspace-policy.sh`: ok;
- `check-rack-policy.sh`: PASS;
- `realtime_policy`: ok (314 marked regions in 48 files).

No render dispatch changed, so the web gate was not rerun; it passed on attempt 2.

### Verdict on attempt 3 (Sol, `08d74a5b7`): PASS
No BLOCKER or MAJOR. N5 (`Some(0 * ..)`), N5b (`None`) and two sibling-counter mutants are red in debug and release. The test items made `pub(crate)` leak nothing into non-test builds. The deleted `Some(0)` assertion in `silent_bank_skip` was a duplicate: rack's exact-count lane-swap test owns that claim (mutant H is red there). The two MINORs on the rebuild successor's bound (the carry path bypasses pins; the bound counts trades, not counter increments) are folded into #2424.

### CI fix after the PASS: the graph dependency-boundary policy (PR #2426)

PR #2426 failed "Rack, builtins and graph dependency-boundary policy" with "publication, I/O, threading, synchronization, or logging leaked into graph path".
- **Cause.** `scripts/check-graph-policy.sh` strips a test module only when it matches `^#\[cfg(test)\]\nmod NAME {`. Attempt 3 made `runtime.rs`'s `mod tests` into `pub(crate) mod tests`, so the script read that module's `std::sync::Mutex` uses as graph-path code.
- **Fix.** The gate is unchanged.
  - `runtime.rs`'s test module is a plain `#[cfg(test)] mod tests` again, and `SwapRig`, `SwapFault` and their fields are private again.
  - `the_lane_swap_defect_total_reads_the_plans_runtime` moved into `runtime.rs`'s tests. It reaches `lib.rs`'s helper as `crate::tests::sidechain_pdc_plan`, which is now `pub(crate)` inside `lib.rs`'s unchanged `mod tests`. `lib.rs`'s module line is untouched.
- **Mutation.** Both wrapper mutants are red in debug and release at the moved test: N5 (`Some(0 * ..)`) and N5b (`None`).
- **Gates.** Every gate below passed, with one private target dir, deleted afterwards:
  - Policy scripts, each ok:
    - `check-graph-policy.sh`: PASS;
    - `test-graph-policy.sh`;
    - `check-rack-policy.sh` and `test-rack-policy.sh`;
    - `check-builtins-policy.sh` and `test-builtins-policy.sh`;
    - `test-workspace-policy.sh` and `check-workspace-policy.sh`.
  - `cargo test --locked -p graph`, in debug and release;
  - clippy `-D warnings` on `graph` (all targets), in debug and release;
  - `cargo doc -D warnings`, `cargo fmt --all --check`.
