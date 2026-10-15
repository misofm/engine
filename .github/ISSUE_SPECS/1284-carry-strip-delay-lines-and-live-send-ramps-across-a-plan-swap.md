# Carry strip delay lines and live send ramps across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8, D15-9).
Slice 14 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

Three kinds of state keep their exact values through a plan swap. The carry moves them at the swap
block, and the predecessor never renders again:
- a strip's input alignment delay (`delay_samples`, used for multi-mic alignment);
- a submix's input delay;
- a live send whose level is ramping at the swap.

After this slice every DSP state family of a plan carries. A structural edit that changes no
node's latency is then bit-continuous for every unchanged path. Meters follow in #1327.
A changed `delay_samples` restarts its line, and the strip is reported for the duck-swap transition
(D15-9). A re-pointed send never ducks its source strip: it is a route removed plus a route added,
which *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363) ramps at
route level (decision 15, D15-9). The retarget of a carried send whose live value changed is
*Deliver value-only send edits to the running C ABI plan* (#1225) D8.

## Context

- A strip's `delay_samples` is a `TrackDelayLine` (`crates/graph/src/runtime.rs:1021`). It is driven
  by `NodeKind::TrackDelay` on a track input or `NodeKind::SumDelay` on a submix input
  (`:1106-1132`), and held in `Runtime::track_delays` (`:2513`).
  - It is not latency (`crates/graph-compiler/src/pdc.rs:1-24`).
  - A strip whose delay is 0 on both lanes has no line.
- A live send is `NodeKind::LiveRoute(Box<LiveRoute>)` (`runtime.rs:861-874`): an `IndexedRamp`,
  its position, its mute and its control lane. At bind it is settled on the prepared coefficients.
  Its `drain` (`:896`) applies the records present at entry directly to the ramp.
- The send records the classifier emits arrive with #1225 and *Let C ABI sends follow their
  source strip's mute live* (#1226); decision 15 also makes `follows_mute` live in the browser
  (#1342). Until they land the classifier emits no route record, so every send value change is a
  prepared difference. #1225 D8 adds the carried send's retarget when it makes them live; this slice
  does not depend on it.
- The base, the retarget pattern and the restart set come from #1277 (D2-D6) and *Carry
  compensation lines across a plan swap* (#1283). The carry program and its all-or-nothing check
  are on `main` (#1270-#1276).

## Decisions frozen for this slice

- **D1. Strip delay lines.** The key is the strip ID and the lane. A line carries when the lane's
  `delay_samples` is equal in the base and in the successor's model. The lengths are then equal.
  - At the swap block the carry swaps the two rings and the cursor.
  - A changed `delay_samples` is a prepared value (decision 14; D15-9). Its line starts at rest, and
    the strip goes into the restart set, so #1324's duck-swap removes the click.
- **D2. Live sends.** The key is the route ID. The route's source, tap and destination are
  structural.
  - If any of them differs, the route is re-pointed (decision 15, D15-9): its
    ramp state does not carry, and its source strip does **not** go into the restart set. #1363
    D1(b) ramps it at route level, as the old route removed plus the new route added.
  - Gain, matrix, mute and `follows_mute` are live exactly when the classifier emits a record for
    their change (#1277 D3). A send whose only differences are live carries.
  - A send with any other value difference does not carry and starts at its prepared value. Its
    source strip does not go into the restart set; #1363 handles it like a re-pointed route. Once
    #1225, #1226 and #1342 have landed, every value of a send into a submix is live, so this case no
    longer arises.
  - At the swap block, drain the predecessor's route lane (the records present at entry), then copy
    the ramp, its position and its mute. The drain applies the records at the boundary where the
    predecessor's next block would apply them, so the successor continues exactly.
- **Moved out.** The former D3 (carry, then retarget for sends) and its gate are #1225 D8 and its
  gate 7. This slice writes no route record and edits no classifier code.

## Deliverables

1. D1-D2 in `crates/graph`, and the inventory rows and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No change to delay or ramp kernels. No floors (#1285).
- No duck-swap itself (#1324), and no route-level ramp (#1363).
- No send retarget (#1225 D8).

## Objective gates

1. **Gap-free acceptance.** Session A has:
   - one track with `delay_samples` on both lanes (different values);
   - two tracks routed into a submix with `delay_samples`;
   - a live send from a third track into that submix.

   Prepare with live controls. Admit a send-level record with a multi-block ramp between blocks 5
   and 6. Prepare B (A plus a muted track whose ID sorts first) from the base. Every block equals
   the reference fed the same record at the same block.
2. **Prepared changes.** A strip whose `delay_samples` changed starts its line at rest. A send
   re-pointed to another submix starts at its prepared value, and its ramp is not carried.
   `restarted_strips()` is exactly the delay-changed strip; the re-pointed send's source strip is
   not in it.
3. **Realtime.** The swap block makes zero allocations and frees.
4. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: any of three defects turns it red:
  - a strip or submix delay line left at rest drops the delayed strip for its delay length;
  - a send ramp restarted from its target steps the level;
  - a record is lost because the copy ran before the drain.
- Gate 2: carrying a line whose `delay_samples` changed plays the old alignment. A send carried into
  a new destination mixes the old ramp into the wrong bus. A re-pointed send that still ducks its
  source strip puts it in the restart set. Each turns it red.

## Dependencies

- *Carry compensation lines across a plan swap* (#1283).
- None on #1225: the send retarget lives there (D8) and #1225 depends on this slice.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** `LiveRoute` is `crates/graph/src/runtime.rs:861` (its `drain` `:902`), the
  `NodeKind::LiveRoute` variant `:1166`, `TrackDelayLine` `:1039`, `NodeKind::TrackDelay` `:1124`,
  `NodeKind::SumDelay` `:1144`, `Runtime::track_delays` `:2607`.
- **Send records are live now.** #1225, #1226 and #1557 are closed. For a route into a submix the
  registry's gain, matrix, mute and `follows_mute` rows are `Live` (`crates/controls/src/route.rs`)
  on both hosts, and the browser reaches them through the core since the flip (#1342 is still open
  but has nothing left to make live). A route into the output has no `LiveRoute` lane and stays
  prepared. So the Context bullet "Until they land the classifier emits no route record" and D2's
  case "a send with any other value difference" apply only to routes into the output.
- **#1225 D8 moves back here.** #1225 closed as superseded by #1557, and its decision 17 amendment
  moves its D8 (carry, then retarget for sends) to this issue. The bullet "Moved out", the non-goal
  "No send retarget (#1225 D8)" and the Dependencies line on #1225 are void. New:
  - **D3. Carry, then retarget (D15-7; #1225 D8).** The classifier's per-route derivation
    (`route_records`, `crates/host-core/src/live_delta.rs:1416`) becomes one per-route function that
    `classify_live_delta` and the successor join both call (the pattern of #1277 D5). It works on
    effective values: a route's effective gate follows its source strip's effective mute (#1558
    D1). For every carried send whose live values differ between `committed` and the successor's
    model, the successor entry writes the route's record into the successor's route cell
    (`RouteControlProducer::write`, `crates/host-core/src/route_controls.rs:237`) with
    `stamp_held_gate(base.revision)`, in the retarget block with the strips (`prepare.rs:2644`),
    with the ramp from `SuccessorBase::edit_ramps`. The write cannot fail (#1347).
  - **Gate 5 (new; #1225 gate 7).** On gate 1's session, B is A plus a muted track whose ID sorts
    first, and B also changes the carried send's gain. Run 1 is the structural swap with D3's
    retarget. Run 2 writes the same value to A just before the swap and prepares B from a base that
    holds it. Every block of the two runs is bit-identical. It turns red if a carried send keeps
    the predecessor's level (a missing retarget) or if the retarget applies a block early or late.
- **D2's drain with cells.** A route lane holds a latest-target cell since #1347. "Drain the
  predecessor's route lane (the records present at entry)" reads: apply the predecessor's pending
  cell value under `LiveSnapshot::ALL` with `CellReader::read_counted`, as the strip carry does
  (#1277 D8), then copy the ramp, its position and its mute. No engine edit.
- **Authorized paths** gain `crates/host-core/src/live_delta.rs` (D3's extraction only).
- **Dependencies.** #1283 (shared files). #1225 is removed.

## Decision record: #1284 (batch 8)

**Shipped.** D1, D2 and the amended D3 (carry, then retarget; #1225 D8), as frozen.

- `graph`: `TrackDelayLine::lanes_match` / `adopt` (equal rings and cursors swap, lane by lane),
  `LiveRoute::adopt` (ramp, position, mute), `Runtime::{can_carry,carry}_{strip_delays,live_routes}`,
  all in policy regions and bounded by the move table. The route carry first applies the
  predecessor's pending cell value under `LiveSnapshot::ALL`, then copies. New API:
  `GraphStripDelayLine` / `GraphStripDelayMove` (lane mask `STRIP_DELAY_LEFT|RIGHT`),
  `GraphLiveRoute` / `GraphRouteMove` (by runtime unit), `take_strip_delay_lines`, `take_live_routes`
  (read once after bind, as `take_compensation_lines`), `install_strip_delay_carry`,
  `install_route_carry`, the two `*_carry_retained_bytes`, six install errors; both moves join the
  "every section checks before any section moves" list. `build_sequential` now also returns the bind
  identities.
- `host-core`: `PlanStateInventory::{strip_delays, live_routes}`; `join_strip_delays` and
  `resolve_strip_delay_carry` (the key is strip and lane; a lane that carries has an equal
  `delay_samples` and a line in both plans; a strip with a changed lane, or whose line one plan lacks,
  goes into `restarted`); `join_routes` and `resolve_route_carry` (a route into a submix with the same
  ID, source (strip and tap) and destination, and a live route in both plans). D3: the per-route
  function is `route_record_between` (+ `RouteEditRamps`, moved from `live_delta.rs`); the classifier
  and the join both call it, on effective values (`EffectiveDiff::route_gates`, `gate_length`,
  `source_strips`, un-gated from `control-provider`). The retarget is written with
  `stamp_held_gate(base.revision)` in the strips' block. `join_strips` takes the `EffectiveDiff`, built
  once per preparation. The late carry program (names the predecessor, moves no source) now also
  covers a plan whose only carry is a strip delay or a send.

**Departures.**

- `LiveRoute::drain` is renamed `apply_pending`. The call rule of the realtime policy refused
  `from.drain(..)` in the move loop, because other functions named `drain` pop a queue. `apply_pending`
  holds no pop (it reads a latest-target cell); its one other caller is the block drain in `execute_op`.
- A route carries only if its source and destination are equal. The text says "source, tap and
  destination"; the tap is part of `RouteSource`.
- Test harness: `EffectValue::SendGain` and `CellRecord::Route` write a send record. The two
  `a_followed_mute_change_*` oracles (the "written to A" run) now also write the send's gate record, as
  a live commit does (#1558 D1): with sends carried, a strip mute written to A without the following
  send's record is not the state the committed model describes.

**Gates and tests** (`crates/host-core/tests/successor_swap.rs` unless noted):

| Gate | Test |
|---|---|
| 1 | `strip_delays_and_a_send_ramp_cross_the_swap_at_{eight,four}_lanes` (track lanes 37/53, a lane with no delay, submix 41/29, a ramp from block 4 and a record pending at the swap) |
| 2 | `a_changed_lane_delay_restarts_only_its_lane_at_*` (restart set `["eq0"]`, left lane carried, right plane silent), `a_repointed_send_starts_at_its_prepared_value_at_*` (no restart, equals B without records), `only_the_delay_changed_strip_is_restarted_at_*` |
| 3 | `assert_clean_swap` in every case: zero allocations and frees on the swap block |
| 5 | `a_carried_send_is_retargeted_to_the_successors_gain_at_*` (run 1 retarget equals run 2 live write; not vacuous against a stale level) |
| unit | `graph::runtime::tests::a_strip_delay_line_hands_over_the_lanes_of_its_mask`, `prepare::tests::strip_delay_lanes_join_by_strip_and_lane`, `prepare::tests::live_routes_join_by_route_id` |

Red under mutation (each run once, reverted): no strip delay move (gates 1, 2 red), no route move
(gates 1, 2b red), no predecessor drain (gates 1, 5 red), no retarget write (gate 5 red), route join
without the destination (gate 2b red), lanes keyed together (gate 2a red).

**Measured.** Browser boot high-water growth 20,447,232 bytes (before: 20,447,232). No pin moved.
`realtime-policy`, `check-bench-policy.sh`, `check-workspace-policy.sh`, `check-builtins-fixtures.sh`,
`check-graph-determinism.sh`, `check-cross-targets.sh`, `check-web-audioworklet.sh`, rustdoc
`-D warnings` and clippy `-D warnings` (host-core no features, default, all features; graph) pass.
`trace-graph-audit.sh` was not run.
