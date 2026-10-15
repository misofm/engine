# Carry per-node effect instances across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 12 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

An effect that renders per node in both plans, and whose prepared values are unchanged, keeps its
exact state through a plan swap. Examples are a delay's echoes and feedback tail, a multiband
compressor, and any insert the planner leaves unbanked in both plans. Its live state and unread live
values come along, so no acknowledged edit is lost. The carry moves the instance by pointer at the
swap block, and the predecessor never renders again. It costs a few words per effect, whatever the
state size; a delay holds two seconds per channel. There is no payload scratch and no copy. A
latency-growth swap (*Grow latency during playback by adopting a primed warm successor*, #1287)
adopts in move mode at its first ready block too, so it moves the instance the same way.

## Context

- A per-node effect is `NodeKind::Effect(GraphPreparedEffect)` or
  `NodeKind::LiveControlEffect(Box<LiveControlEffect>)` (`crates/graph/src/runtime.rs:1136`,
  `:1143`).
  - `GraphPreparedEffect` (`crates/graph/src/lib.rs:881-891`) carries `id: EffectNodeId`,
    `metadata`, `processor: Box<dyn PreparedNativeEffect>` (`:884`), `response_snapshot_declared`
    and `native_id`. The state lives behind the `processor` box, so swapping two boxes moves the
    whole state by pointer.
  - `LiveControlEffect` (`runtime.rs:1184-1197`) adds its `EffectControlLane`, a span window, a
    `BypassShunt` and optional observation. Its records are applied inside `execute_op` (`:3510`).
- The delay never banks (`NEVER_BANKED_EFFECTS`, `crates/effect-compiler/src/prepare.rs:244`), and
  the multiband keeps a prepared bypass (`PREPARED_BYPASS_EFFECTS`, `:260`; `lowers_session_bypass`,
  `:266`). Their session bypass sits inside the processor. A live bypass is the lane's `bypass` flag
  and the shunt.
- The per-node payload pair is render-safe and exact mid-ramp for every effect, the delay included
  since `e0e4e8a20` (#1278 D2a). A payload copy of a delay at 96 kHz would still pass about 1.5 MiB
  through a scratch buffer twice.
- The effect rule, the retarget and the carried control state come from *Carry live-controlled
  effect lanes across a plan swap* (#1280). The bank and per-node flips come from #1281.

## Decisions frozen for this slice

- **D1. Rule.** As #1279 D1. The inventory's prepared layout includes the processor's prepared
  bypass.
  - A committed bypass change on the delay or the multiband is a prepared change. The owner
    restarts and its strip goes into the restart set, so *Duck-swap a strip whose state cannot
    continue across a plan swap* (#1324) gives it a transition (D15-13 E4).
  - When *Give the delay a live bypass shunt* (#1339) and *Give the multiband compressor a live
    bypass shunt* (#1340) land, that bypass becomes live and retargets under #1280 D4. They rewrite
    gate 3.
  - For an effect whose bypass is lowered, a live bypass is lane state and carries.
- **D2. Move mode.** When both plans hold the owner per node, swap their `processor` boxes
  (`core::mem::swap`). For a `LiveControlEffect`, also swap the `BypassShunt` (same frames and
  latency by D1). The successor's lane takes the predecessor's control state with
  `EffectControlLane::carry_from` (#1280 D2), so unread live values render in the successor. No
  payload, no copy, no allocation. The predecessor's plan then holds the successor's at-rest box,
  and it retires and is reclaimed off the render thread with that plan, as today. No
  `PreparedNativeEffect` method is added: the move needs none.
- **D3. Mixed wrappers.** If one plan has live controls on the owner and the other does not (no
  host does this today), the owner does not carry, and the join records it in the restart set.

## Deliverables

1. D1-D3 in `crates/graph` (the location table and the program section), and the inventory rows
   and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.
3. A test-support counter of bytes the carry passes through the payload scratch.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`

## Non-goals

- No graph delay lines (#1283, #1284).
- No live bypass shunt for the delay or the multiband (#1339, #1340).

## Objective gates

1. **Gap-free acceptance: a delay tail.** Session A has three tracks: one with a delay insert at
   50 ms and high feedback, and one with a multiband compressor insert. The source plays for 4
   blocks, then drops to a deterministic noise floor near -80 dBFS (never an exact zero). Session B
   adds a muted track whose ID sorts first. Swap after block 6, while the tail rings. The swapped
   run and a fresh B fed the same PCM from frame 0 are bit-identical for 64 blocks. With this
   slice's section of the program disabled, the tail stops at block 7.
2. **Unread values and live bypass.** Prepare with live controls. Write a delay-time value to A's
   cell just before the swap (not yet rendered), and a live bypass of a per-node compressor insert a
   block earlier (one track, so it does not bank). Prepare B from the base. Every block equals the
   reference fed the same values at the same blocks.
3. **A prepared bypass change restarts.** A committed bypass change on the delay: the join has no
   pair for it, its successor instance starts at rest, and `restarted_strips()` is exactly that
   strip.
4. **Cost shape.** At 96 kHz, the payload-bytes counter reads zero for the delay's owner (it
   moved). The swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that leaves per-node effects at rest cuts the tail. It turns red.
- Gate 2: a move that keeps the successor's own lane state drops the acknowledged live bypass and
  the unread value. It turns red: the compressor's bypass is a lowered lane bypass, so the gate
  sees it.
- Gate 3: a rule that ignores the prepared bypass moves an unbypassed delay into a plan whose
  committed bypass is on. It turns red.
- Gate 4: a carry that copies a delay through the payload scratch (correct bits, about 1.5 MiB
  copied twice on the render thread) turns it red.

## Dependencies

- *Carry an insert lane that moves between a bank and a per-node instance* (#1281).

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** `NodeKind::LiveControlEffect(Box<LiveControlEffect>)` is
  `crates/graph/src/runtime.rs:1161`, struct `:1228` (fields `effect`, `control`, `spans`, `shunt`,
  `observation`); `GraphPreparedEffect` is `crates/graph/src/lib.rs:909`; `NEVER_BANKED_EFFECTS` is
  `crates/effect-compiler/src/prepare.rs:291` and `PREPARED_BYPASS_EFFECTS` `:307`. #1339 and #1340
  are open, and the registry keeps the delay and multiband bypass rebuild-only
  (`lowers_session_bypass`), so D1 stands.
- **D3 for decision 17.** Both hosts attach effect lanes from `controls::live_lanes`, which depends
  on the session's shape only, so a mixed pair arises only when a transaction adds the first or
  removes the last effect with a live row. D3 stands.
- **D2.** Swap `effect`'s processor box and the `shunt`; the `spans` window is not moved (each plan
  sized its own from the same metadata; #1621, open, sizes it by live cells, which this move does
  not read). The lane's control state goes through `EffectControlLane::carry_from` with its carried
  slots (#1280a). The `observation` lane is #1327b's.
- **Live retarget.** A live difference on a per-node owner retargets by #1280c's per-instance
  function.
- **Gate 2.** A live bypass now crossfades over `mute_ms` (#1341a1). The reference gets the same
  record at the same block, so the gate stands.
- **Edit order with #1621** (`runtime.rs`, the per-node window): whichever is scheduled first lands
  first; the other rebases. They change different lines.
- **Dependencies.** #1281, #1280c (for D1's retarget).

## Decision record: #1282 (batch 11)

**Shipped.** A live-controlled per-node effect instance that survives a swap with its prepared
layout unchanged moves by pointer: D1-D3 as frozen.

- `graph`: `LiveControlEffect::can_move_from` / `move_from` (`runtime.rs`, in a realtime-policy
  region). The check compares the prepared layout, the control kind and cell shape (the successor
  holds its carried slots, #1633) and the shunt shape. The move takes the control state through
  `EffectControlLane::carry_from` (unread cells, bypass crossfade, symmetry terms, #1280a), then
  exchanges the two processor boxes and the two `BypassShunt`s with `core::mem::swap`. The `spans`
  window and the `observation` lane stay (#1621 sizes the window; the tap state is #1327b's), and
  the pending `Observe` records of the predecessor are applied at the carry by the second marked
  loop, as for every other move. `Runtime::can_carry_effect_lanes` and `carry_effect_lanes` gain the
  node-to-node arm. `install_effect_carry` no longer refuses a node-to-node move and
  `GraphCarryInstallError::EffectMoveBetweenNodes` is deleted.
- `host-core`: `resolve_effect_carry` drops the node-to-node exclusion, and a node-to-node move
  adds nothing to the payload scratch (`carry.scratch_bytes` counts only moves that go through it),
  so the carry's retained charge for a session of delays is the move table and the carried slots.
  D3 needs no code: a per-node effect without live controls has no inventory row, so its owner
  restarts its strip.
- `effect-contract` (outside the authorized list, one mechanical helper): `BypassShunt::same_shape`.
- `graph` test-support: `test_only_carry_payload_bytes` / `test_only_reset_carry_payload_bytes`, a
  thread-local count of the layout payload each lane move passes through the scratch.
- Test harness: `effect_run_blocks` (a run of more than `BLOCKS` blocks), `Feed::with_blocks`,
  `EffectRun::payload_bytes`; `LivePlan::submit` submits at the plan's own sample rate.

**Departures.** None from the frozen decisions. D1's "prepared bypass" needs no new code: the
inventory layout already carries `bypass`, and `effect_instance_records` refuses a bypass change of
an effect that does not lower it.

**Gates and tests** (`crates/host-core/tests/successor_swap.rs`):

| Gate | Test |
|---|---|
| 1 | `a_delay_tail_rings_on_through_the_swap_at_every_width` (64 blocks; restart run loses the echo) |
| 2 | `unread_values_and_a_live_bypass_cross_the_swap_at_every_width` |
| 3 | `a_prepared_bypass_change_restarts_its_strip_at_every_width` (`restarted_strips() == ["eq0"]`) |
| 4 | `a_delay_moves_by_pointer_at_96_khz_at_every_width` (payload bytes 0, swap block clean); `the_payload_counter_counts_a_lane_move` (the counter reads nonzero for a bank move) |
| D3 | `an_owner_without_live_controls_on_one_side_restarts_at_every_width` |
| D1 live | `a_live_difference_on_a_moved_instance_is_retargeted_at_every_width` (delay time, compressor bypass) |

Red under mutation (each run once, reverted): no processor exchange (gates 1, 2, 4 red), no
`carry_from` (gate 2 red).

**Consequence outside the slice.** `capi`'s `a_prepared_bypass_change_rebuilds_and_renders_the_committed_model`
(#1266 gate 3) used a control that rebuilt by content edits, which restarted the per-node multiband
as a side effect. A content edit now moves the multiband, so the control's rebuilds also replace
the multiband by a fresh instance of the same declaration under a new ID (a prepared change of
that owner), which restarts it and its strip as the bypass rebuild does. The oracle is otherwise
unchanged.

**Measured.** Browser boot high-water growth 20,447,232 bytes (before: 20,447,232; the pin is
20,971,520). No resource estimate or pin moved: `cargo test -p host-core --all-features` (428),
`effect-contract`, `rack`, `graph`, `graph-compiler`, `capi`, `engine`, `host-web --features
host-web/test-support` green; clippy `-D warnings` on `host-core` (no features, default,
all features) and `host-core graph effect-contract rack capi --all-targets --all-features`;
`realtime-policy`, `check-bench-policy.sh`, `check-builtins-fixtures.sh`, `check-cross-targets.sh`,
`check-web-audioworklet.sh`, `trace-graph-audit.sh` (1,000,000 blocks) and rustdoc `-D warnings` pass.
