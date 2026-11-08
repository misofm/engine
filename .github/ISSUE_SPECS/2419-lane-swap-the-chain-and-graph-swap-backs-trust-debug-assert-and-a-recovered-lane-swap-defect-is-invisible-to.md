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
(empty)
