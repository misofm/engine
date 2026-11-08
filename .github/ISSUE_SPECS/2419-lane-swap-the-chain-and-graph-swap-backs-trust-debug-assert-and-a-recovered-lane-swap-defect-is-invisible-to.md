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
  - Every refusal after the check passed pins both strips for the plan's life through a new `LaneMap::pin_positions` (the #2135 `pinned` flag, by home position), so the packer never asks the broken banks again. `restore_home_lanes` is unchanged; it does not read pins.
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
- `graph::lane_swap_defect_count`'s downcast wrapper is reached by no nonzero test. No prepared plan can hold a faulting bank without a test hook in the bound-plan code, which another stream owns. `Runtime::lane_swap_defect_count`, which it returns, is tested.
- A split slot's residue (the other strip's prepared defaults and its control lane, taps and dry-shunt line) lasts until a plan rebuild. A host that reads a nonzero total could request one; nothing does yet.
