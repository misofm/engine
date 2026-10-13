# Refuse half a fused strip stage at carry install, and remove the carry drain's owner filter

Stream A of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). Successor to the
known gap (1) that *Hold strip input-lane values in latest-target cells* (#1346) and *Hold effect
parameter, bypass and EQ-target values in latest-target cells* (#1345) recorded
(`.github/ISSUE_SPECS/1346-*.md:477`, `1345-*.md:1226`), from stream B batch 3 verdict 2 (NIT) and
verdict 3 (NIT). Code anchors verified on `codex/d15-stream-b3` at `711847ce3`. Re-verify every
anchor before you start.


## Product outcome

A plan swap carries a strip's fader, mute and pan state. In the fused form (the browser's plans,
prepared between render calls) one stage holds both halves of a strip: the fader and mute half and
the matrix and pan half. Issue #1277 D9 says that this stage carries both halves or neither. Today
only host-core keeps that rule. The graph crate's public install function accepts a move set that
carries one half alone, and the render thread then needs an owner filter to keep the count of
replaced values (`live_values_superseded`) correct for a state that nothing makes. No test reaches
that filter. After this issue, the install function refuses a move set that breaks the rule, before
the plan goes to the render thread, a test proves the refusal, and the render-thread filter is
removed because it cannot change a result.

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

- **D1. The rule.** A strip move whose successor location is in a `BuiltinStripStage::FaderMatrix`
  bank is admitted only if the same move set holds exactly one move of the other owner with the
  same successor location and the same predecessor location. A move into a `Fader` or `Matrix`
  (split) bank is not affected: in the split form each stage carries on its own (#1277 coordinator
  ruling, `1277-*.md:372-378`).
- **D2. Where.** The check runs in `install_builtin_strip_carry`, after the existing per-move
  checks and before the sort, off the render thread. It may allocate nothing new; its cost is at
  most the existing duplicate scan's (quadratic in the move count is accepted, as the duplicate scan at `:3151-3156` is).
  The bank stage of a successor location is read through a new `pub(crate)` runtime method next to
  `has_strip_lane` and outside every realtime region, for example
  `fn strip_stage_at(&mut self, at: GraphLaneLocation) -> Option<BuiltinStripStage>`, which reads
  `carried_strip_lanes()` of the slot's processor.
- **D3. The refusal.** A new variant `GraphCarryInstallError::FusedStripHalf`, documented as "a move
  into a fused fader and matrix lane has no move of the other owner between the same two lanes
  (#1277 D9: a fused stage carries both halves or neither)". The refused install leaves any earlier
  section in place, as every other refusal does. The doc of `install_builtin_strip_carry` names the
  new refusal.
- **D4. Remove the filter.** Delete `.filter(|step| step.owner == head.owner)` from
  `carried_strip_lane_mask`. Its doc says that every move from `first` on in the bank is read, and
  why that is the owner's lane set: in a fused bank both owners move the same lanes (D1), and the
  sort puts each lane's two moves next to each other. The `carry_strip_lanes` doc keeps its current
  meaning. Do not merge the two mask helpers (non-goal).
- **D5. No render-thread check.** Do not add a `debug_assert` or any other check of D1 to the swap
  block. The install is the only way moves enter a plan (`executor.carry_strips` takes a move set only at
  `lib.rs:3159`; elsewhere it is built empty, `:3539`).

## Deliverables

1. D2 and D3 in `crates/graph/src/lib.rs` and the D2 helper in `crates/graph/src/runtime.rs`.
2. D4 in `crates/graph/src/runtime.rs`.
3. One new test in `crates/host-core/tests/successor_swap.rs`, written first (gate 1), after
   `a_bad_input_move_is_refused_whole`.

## Authorized paths

- `crates/graph/src/lib.rs`: the new `GraphCarryInstallError` variant and its doc; the body and doc
  of `install_builtin_strip_carry`.
- `crates/graph/src/runtime.rs`: the new D2 helper (outside every realtime region); the filter line
  and doc of `carried_strip_lane_mask`.
- `crates/host-core/tests/successor_swap.rs`: the new test only; no helper changes.
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

1. **Red-first test** `a_fused_strip_move_without_its_other_half_is_refused`. First add the D3
   variant with no check, write the test, run it and record it red in the attempt record (the
   failing assertion: install returned `Ok(())`). Then add the D2 check and record it green.
   1. `backend = Backend::Simd4`; `a = Session::compile(strip_state_session())`;
      `b = Session::compile(with_muted_track(strip_state_session()))`.
   2. Fused leg: `predecessor = CellPlan::fresh(&a, backend, Form::Fused)`,
      `successor = CellPlan::fresh(&b, backend, Form::Fused)`. Install a carry program on the
      successor's plan naming `predecessor.live.prepared.inventory.plan_identity()` with no source
      (as `a_bad_input_move_is_refused_whole` does). From `graph::builtin_strip_lanes` of each plan,
      take `eq0`'s and `eq1`'s lanes. Assert that each is `BuiltinStripStage::FaderMatrix` and that
      a strip's Fader and Matrix lanes have one location. Name them `s0` (successor `eq0`), `p0`,
      `p1` (predecessor `eq0`, `eq1`).
   3. Assert, in this order, on the successor's plan:
      - `[Fader s0 <- p0]` alone: `Err(GraphCarryInstallError::FusedStripHalf)`;
      - `[Matrix s0 <- p0]` alone: `Err(FusedStripHalf)`;
      - `[Fader s0 <- p0, Matrix s0 <- p1]` (halves from two predecessor lanes): `Err(FusedStripHalf)`;
      - `[Fader s0 <- p0, Matrix s0 <- p0]`: `Ok(())`.
   4. Split leg: the same two sessions with `Form::Split`, a carry program as in step 2, and
      `eq0`'s Fader lane (stage `Fader`) in each plan. `[Fader s0 <- p0]` alone is `Ok(())`.
   - *Test value:* red if the graph's public strip install accepts a move set that carries one half
     of a fused fader and matrix stage, or the two halves from different lanes, which #1277 D9
     forbids and which made the render-thread owner filter necessary; or if the check also refuses
     a single-owner move into a split bank, which host-core makes for every followed-mute restart.
     No existing test checks a strip install refusal.
   - *Mutations (PR evidence; record each red with its failing step):* M1, the bug's revert (remove
     the D2 check): red at step 3, first assertion. M2, the check compares only the successor
     location, not the predecessor: red at step 3, third assertion. M3, the check applies to split
     banks too: red at step 4; also record which existing split-form `successor_swap` tests go red.
2. **Filter removal evidence (PR evidence, not a committed test).** With D4 applied, the full suites
   of gate 4 pass unchanged. Record in the attempt record the argument of "Context, When the filter
   changes the mask" for this tree, and that `executor.carry_strips` takes a move set only in
   `install_builtin_strip_carry`.
3. **Realtime.** `bash scripts/check-realtime-policy.sh` passes. The D2 helper is outside every
   marked region; the marked-region count does not change.
4. **Workspace.**
   - `cargo test --locked -p host-core --all-features --test successor_swap a_fused_strip_move_without_its_other_half_is_refused`
   - `cargo test --locked -p graph --all-features`
   - `cargo test --locked -p host-core --all-features`
   - `cargo test --locked -p capi --all-features`
   - `cargo test --locked -p host-web --all-features`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p graph -p host-core --all-targets --all-features -- -D warnings`
   - `cargo doc --locked -p graph --no-deps` with `RUSTDOCFLAGS="-D warnings"`
   - `bash scripts/check-workspace-policy.sh`

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

- Attempt 1: not started.

## Root rulings (2026-10-13)

- **R1. Refuse at install and remove the filter.** An invalid move set is refused at the public
  boundary, off the render thread, so the render-time filter has no state to handle. The name
  `GraphCarryInstallError::FusedStripHalf` is accepted.
- **R2. Stream A owns this slice** (it owns `crates/graph` and `successor_swap.rs`), so no named
  exception is needed. It lands after #1345 and #1346 are on main.
