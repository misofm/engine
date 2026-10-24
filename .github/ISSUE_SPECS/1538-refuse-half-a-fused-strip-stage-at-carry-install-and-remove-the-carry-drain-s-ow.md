# Refuse half a fused strip stage at carry install, and remove the carry drain's owner filter

Stream A of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). Successor to the
known gap (1) that *Hold strip input-lane values in latest-target cells* (#1346) and *Hold effect
parameter, bypass and EQ-target values in latest-target cells* (#1345) recorded
(`.github/ISSUE_SPECS/1346-*.md:477`, `1345-*.md:1226`), from stream B batch 3 verdict 2 (NIT) and
verdict 3 (NIT). Code anchors verified on `codex/d15-stream-b3` at `711847ce3`. Re-verify every
anchor before you start.


## Product outcome

Narrowed on 2026-10-24 (scope note, root decision). #1767 (PR #1776, merge `2724b2f58`) removed
the fused `BuiltinStripStage::FaderMatrix`: today `BuiltinStripStage` is `Fader | Matrix` and each
stage holds one owner (`crates/graph/src/lib.rs`, `BuiltinStripStage::holds`). A move set that
carries "half a fused stage" can no longer be built, so the install-time refusal (D1-D3) and its
red-first test (gate 1) are superseded. What remains is D4: the render-thread owner filter in
`carried_strip_lane_mask` cannot change a result, so it is removed, and `carry_strip_lanes` drains
each predecessor bank once with one `Option` instead of one per owner.

The Context section below describes the pre-#1767 tree and is kept as history.

## Context

- **The filter.** `carried_strip_lane_mask` (`crates/graph/src/runtime.rs:4212-4226`) builds the
  lane mask that one predecessor bank's strip drain reads. It takes the moves from `first` on that
  share the bank's `(unit, slot)` (`:4220-4223`) and then keeps only the moves of `moves[first]`'s
  owner: `.filter(|step| step.owner == head.owner)` (`:4224`). Its input twin
  `carried_input_lane_mask` (`:4198-4210`) has no owner. Both helpers are inside the realtime region
  `:4170-4296`.
- **The caller.** `carry_strip_lanes` (`runtime.rs:4588-4630`) drains each predecessor bank once
  for each owner (`drained: [Option<(u32, u16)>; 2]`, `:4594`) and calls
  `from.drain_strip_for_carry(step.owner, carried_strip_lane_mask(moves, index))` (`:4612`). The
  fused processor's drain reads only `owner`'s half (`crates/builtins-compiler/src/lib.rs:2203-2211`).
  The split processors ignore the other owner (`:1532-1534`, `:1675-1677`). So the owner argument
  already decides which half is read. The filter decides only which lanes of that half are read.
- **The sort.** `install_builtin_strip_carry` (`crates/graph/src/lib.rs:3127-3161`) sorts the moves
  by `(predecessor, owner)` (`:3158`), with `Fader < Matrix` (`lib.rs:1366-1371`). In a fused bank
  both owners' lanes sit at one location (`GraphStripLane`, `lib.rs:3104-3113`;
  `builtin_strip_lanes`, `:3115-3125`).
- **When the filter changes the mask.** Only when the two owners of one fused bank carry different
  lane sets. Example: Fader moves for lane 0 only, Matrix moves for lanes 0 and 1. Sorted:
  `(0, Fader), (0, Matrix), (1, Matrix)`. From index 0 without the filter the Fader mask is `{0, 1}`;
  with the filter it is `{0}`. Without the filter, lane 1's fader cells, which the swap restarts,
  would be drained and their replaced writes counted. When both owners carry the same lanes, the
  sorted moves are `(l0, F), (l0, M), (l1, F), (l1, M), ...`, so from the first index of either
  owner the bank's remaining moves name the same lane set as that owner's own moves: the filter
  changes nothing.
- **Why nothing makes different lane sets.** host-core's `resolve_strip_carry`
  (`crates/host-core/src/prepare.rs:2741-2790`) forces `carries = [both, both]` for any strip with a
  `FaderMatrix` row (`:2757-2764`), as its doc says (`:2738-2739`) and as #1277 requires (the
  fused stage "carries both halves from a fused predecessor only (D9)",
  `.github/ISSUE_SPECS/1277-*.md:288-289`; the coordinator ruling, `:372-378`). host-core is the
  only caller of `install_builtin_strip_carry` (`prepare.rs:2201`). So the filter is unreachable
  from every host. The graph function itself does not refuse such a set: it checks only that each
  successor location holds the owner (`has_strip_lane`, `runtime.rs:4463-4469`, through
  `strip_lane_mut`, `:4246-4260`) and that no location repeats for one owner (`lib.rs:3147-3157`).
- **The refusal type.** `GraphCarryInstallError` (`lib.rs:2991-3022`). host-core maps every variant
  to `host.carry.install` (`prepare.rs:2201-2202`). No test checks a strip install refusal today.
- **Test pattern to copy.** `a_bad_input_move_is_refused_whole`
  (`crates/host-core/tests/successor_swap.rs:1918-2060`) calls `install_carry_program` and
  `install_builtin_input_carry` directly on a prepared plan and checks each refusal. The fused and
  split strip plans come from `CellPlan::fresh(&session, backend, Form::{Split, Fused})`
  (`successor_swap.rs:2456-2462`, `:2497-2526`); sessions from `strip_state_session()` (`:2371`) and
  `with_muted_track` (`:904`).

## Decisions frozen

- **D1-D3 (install-time `FusedStripHalf` refusal, `strip_stage_at` helper, the new error
  variant): superseded by #1767.** No fused strip stage exists, so no move set can carry one half.
- **D4. Remove the filter.** Delete `.filter(|step| step.owner == head.owner)` from
  `carried_strip_lane_mask`; its doc says every move from `first` on in the predecessor bank is
  read and why they share one owner. Do not merge the two mask helpers (non-goal).
- **D4a (root, 2026-10-24).** In `carry_strip_lanes`, fold `drained: [Option<(u32, u16)>; 2]` to
  one `Option<(u32, u16)>` and drop the owner-to-half `match`. Same argument as D4.
- **D5. No render-thread check.** No `debug_assert` or other check is added to the swap block.
- **No new test** (owner ruling "tests overboard"). One strip bank holding one owner is already
  enforced by `has_strip_lane` at install and by `can_carry_strip_lanes` / `can_adopt_strip_lane`
  at the swap, and the `successor_swap` strip-carry suites exercise the drain mask on every swap.

## Authorized paths (narrowed)

- `crates/graph/src/runtime.rs`: `carried_strip_lane_mask` (doc and body) and `carry_strip_lanes`
  (doc, body, comment) only. Both are in realtime regions; the edits only remove code.
- This issue's own spec file.

## Non-goals

- `crates/builtins-compiler` (`drain_strip_for_carry` keeps its signature and bodies) and host-core
  `prepare.rs` (its clamp stays; it already obeys D1).
- `carried_input_lane_mask`, the input carry, the effect carry, and any change to the swap block's
  order or its counts.
- Merging the two mask helpers.
- The later-bank mask test gap of batch verdict 3 MINOR 1 (`successor_swap.rs:1355`, `:1361`), which
  is a separate finding.

## Objective gates

1. **Single-owner argument (PR evidence, not a test).** Recorded in the attempt record.
2. **Realtime.** `cargo run --locked --release -q -p realtime-policy` passes; the marked-region
   count does not change.
3. **Workspace.** `cargo test --locked --all-features` for `-p graph`, `-p host-core` (full, with
   `successor_swap`), `-p capi`, `-p host-web`; `cargo fmt --all -- --check`; `cargo clippy --locked
   -p graph -p host-core --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS="-D warnings"
   cargo doc --locked -p graph --no-deps`; `bash scripts/check-workspace-policy.sh`; AArch64
   `cargo check --tests -p graph`.

## Possible later fold (not this issue)

`crates/builtins-compiler` `drain_strip_for_carry`'s `owner ==` guard is now equally dead; its
signature is the trait contract, so a fold is a separate issue.

## Dependencies

- #1346 and #1345 on `main` (stream B batch 3): the lane-masked strip drain and
  `carried_strip_lane_mask` exist only from that batch.
- A #1277 (the strip carry and `install_builtin_strip_carry`) is on `main`.

## Placement (root edits `STREAMS.md`)

- Stream B, next B batch, a new row after #1537: dependency #1346; no other-stream dependency.
- `crates/graph` is stream A's. This issue needs a B named exception in the Stream B "Owns" line:
  "`crates/graph/src/{lib,runtime}.rs` (#NNNN: the `FusedStripHalf` refusal in
  `install_builtin_strip_carry`, its stage helper, and the owner filter of
  `carried_strip_lane_mask`)" and "`crates/host-core/tests/successor_swap.rs` (#NNNN: one test)".
- `crates/graph/src/{lib,runtime}.rs` hot-file row (`STREAMS.md:94`): add "B #NNNN (the strip
  install check and the mask filter) in any order with the rest, the later slice rebases".
- `crates/host-core/tests/successor_swap.rs` has no hot-file row. A's open carry slices (#1280-#1285,
  #1324) also add tests there; in any order, the later slice rebases.

## Attempt record

- Attempt 1 (2026-10-24, branch `codex/batch-b9-1538` from `82f987296`): D4 and D4a in
  `crates/graph/src/runtime.rs`. Single-owner argument, verified on this tree:
  1. `Executor` swap calls `carry_strip_lanes` only after `can_carry_strip_lanes` returned true
     for every move (`lib.rs`, the swap block refuses the whole hand-over with
     `PredecessorMismatch` otherwise).
  2. `can_carry_strip_lanes` resolves each move's predecessor through
     `strip_lane_mut(&mut predecessor.units, step.owner, step.predecessor)`, which requires
     `stage.holds(owner)` for that slot's `carried_strip_lanes()` stage.
  3. A strip bank reports one stage (`Fader` in the fader processor, `Matrix` in the matrix
     processor, `crates/builtins-compiler/src/lib.rs`), and `holds` pairs each stage with one
     owner. So every move of one predecessor `(unit, slot)` has that bank's owner: the
     `take_while` run is single-owner and the filter kept every element.
  4. `install_builtin_strip_carry` sorts by `(predecessor, owner)` with `GraphLaneLocation`
     ordered `unit, slot, lane`, so a bank's moves are adjacent; one `drained` slot therefore
     drains each bank exactly once, as the two-slot array did.
  `executor.carry_strips` takes a move set only in `install_builtin_strip_carry`; elsewhere it is
  built empty. No test or other code calls `carried_strip_lane_mask`. Gates all pass:
  realtime-policy (192 marked regions, no marker moved), tests graph 129, host-core 546 (incl.
  `successor_swap`), capi 185, host-web 204, 0 failed; fmt, clippy, doc `-D warnings`, AArch64
  check, workspace policy.

## Root rulings (2026-10-13)

- **R1. Refuse at install and remove the filter.** An invalid move set is refused at the public
  boundary, off the render thread, so the render-time filter has no state to handle. The name
  `GraphCarryInstallError::FusedStripHalf` is accepted.
- **R2. Stream A owns this slice** (it owns `crates/graph` and `successor_swap.rs`), so no named
  exception is needed. It lands after #1345 and #1346 are on main.
