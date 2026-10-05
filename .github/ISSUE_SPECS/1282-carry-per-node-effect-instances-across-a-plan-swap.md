# Carry per-node effect instances across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 12 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

An effect that renders per node in both plans, and whose prepared values are unchanged, keeps its
exact state through a plan swap. Examples are a delay's echoes and feedback tail, a multiband
compressor, and any insert the planner leaves unbanked in both plans. Its live state and unread live
values come along, so no acknowledged edit is lost. There are two modes:
- **Move mode** (an ordinary swap) costs a few words per effect, whatever the state size. A delay
  holds two seconds per channel.
- **Copy mode** (the warm successor of D15-8) copies the state once into the successor's
  preallocated instance while the predecessor keeps rendering. There is no payload scratch and no
  second pass.

## Context

- A per-node effect is `NodeKind::Effect(GraphPreparedEffect)` or
  `NodeKind::LiveControlEffect(Box<LiveControlEffect>)` (`crates/graph/src/runtime.rs:1136`,
  `:1143`).
  - `GraphPreparedEffect` (`crates/graph/src/lib.rs:881-890`) carries `id: EffectNodeId`,
    `metadata`, `processor: Box<dyn PreparedNativeEffect>` and `native_id`.
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
  payload, no copy, no allocation.
- **D3. Copy mode.** `PreparedNativeEffect` gains a required method, `copy_state_from(&mut self,
  source: &dyn PreparedNativeEffect) -> Result<(), StatePayloadError>`, delivered by #1362. It
  copies every state word from a same-type, same-layout instance into this one's existing
  storage in one pass, and refuses anything else without writing. This slice calls it for every
  carried per-node owner. It copies the shunt's dry, line and cursor words, and the lane's control
  state through `carry_from` (#1280 D2, D6), which never consumes a predecessor value. The bytes
  go into `carry_program_copy_bytes`.
- **D4. Mixed wrappers.** If one plan has live controls on the owner and the other does not (no
  host does this today), the owner does not carry, and the join records it in the restart set.

## Deliverables

1. D1-D4 in `crates/graph` (the location table and both modes), and the inventory rows and the join
   in `crates/host-core`.
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
4. **Copy mode.** Gate 1 with a copy after block 6, followed at once by the adoption. Every block
   equals the move run, and A, rendered on for 8 blocks after the copy, equals its uncopied twin.
5. **Cost shape.** At 96 kHz, in move mode, the payload-bytes counter reads zero for the delay's
   owner (it moved). In copy mode it also reads zero (`copy_state_from` uses no scratch), and the
   delay's state size is counted in `carry_program_copy_bytes`. Both the swap block and the copy
   call make zero allocations and frees.
6. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that leaves per-node effects at rest cuts the tail. It turns red.
- Gate 2: a move that keeps the successor's own lane state drops the acknowledged live bypass and
  the unread value. It turns red: the compressor's bypass is a lowered lane bypass, so the gate
  sees it.
- Gate 3: a rule that ignores the prepared bypass moves an unbypassed delay into a plan whose
  committed bypass is on. It turns red.
- Gate 4: a copy that swaps the boxes (right for move mode) leaves the still-rendering predecessor
  with the successor's at-rest instance. Its twin turns red.
- Gate 5: a carry that copies a delay through the payload scratch (correct bits, about 1.5 MiB
  copied twice on the render thread) turns it red.

## Dependencies

- *Carry an insert lane that moves between a bank and a per-node instance* (#1281).
- *Copy a per-node effect's state into a same-layout instance in one pass* (#1362): D3's
  `copy_state_from`. Gates 4 and 5 need it.
