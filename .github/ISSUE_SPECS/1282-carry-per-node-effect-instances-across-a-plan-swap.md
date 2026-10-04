# Carry per-node effect instances across a plan swap

Slice 12 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

An effect that renders per node in both plans and is unchanged keeps its exact instance through a
plan swap: a delay's echoes and feedback tail, a multiband compressor with its prepared bypass, any
insert the planner leaves unbanked in both plans. The move costs a few words per effect, whatever
the state size (a delay holds two seconds per channel). Its live state and pending records come
along, so no acknowledged edit is lost.

## Context

- A per-node effect is `NodeKind::Effect(GraphPreparedEffect)` or
  `NodeKind::LiveControlEffect(Box<LiveControlEffect>)` (`crates/graph/src/runtime.rs:1135-1143`).
  `GraphPreparedEffect` (`crates/graph/src/lib.rs:880`) carries `id: EffectNodeId`, `metadata` and
  `processor: Box<dyn PreparedNativeEffect>`. `LiveControlEffect` (`runtime.rs:1184`) adds its
  `EffectControlLane`, a staging window, a `BypassShunt` (`crates/effect-contract/src/live.rs:867`)
  and optional observation; its records are applied inside `execute_op` (`runtime.rs:3517-3560`).
- The delay never banks and keeps a **prepared** bypass (`NEVER_BANKED_EFFECTS`,
  `crates/effect-compiler/src/prepare.rs:244`); so does the multiband
  (`PREPARED_BYPASS_EFFECTS`, `:259`; `lowers_session_bypass`, `:264-268`). A session bypass on them
  is inside the processor; a live bypass is the lane's `bypass` flag and the shunt (decision 14, F4).
- The inherited control state (`EffectControlLane::inherit_from`) comes from *Carry live-controlled
  effect lanes across a plan swap* (slice 11); bank ↔ per-node flips from *Carry an insert lane that
  moves between a bank and a per-node instance* (#1281).

## Decisions frozen for this slice

- **D1. Rule.** As slice 10's D1. The inventory's prepared layout includes the processor's prepared
  bypass. For the delay and the multiband a committed bypass change becomes a prepared bypass in the
  successor (`PREPARED_BYPASS_EFFECTS`), so the owner is not carried; that is decision 14's open F4,
  not this slice's to change. For an effect whose bypass is lowered, a live bypass is lane state
  (D3) and carries.
- **D2. Move.** When both plans hold the owner per node, swap their `processor` boxes
  (`core::mem::swap`). For a `LiveControlEffect`, also swap the `BypassShunt` (same frames and
  latency by D1). No payload, no copy, no allocation.
- **D3. Control state.** The successor's lane inherits the predecessor's control state with
  `EffectControlLane::inherit_from` (queue consumer, live bypass, symmetry terms, retained targets),
  so pending records render in the successor.
- **D4. Mixed wrappers.** If one plan has live controls and the other not (no host does this today),
  the owner is not carried, and the join records it.

## Deliverables

1. D1-D4 in `crates/graph` (location table and carry) and the inventory rows in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.
3. A test-support counter of bytes the carry copies through the payload scratch.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No change to effect crates or to the payload API.
- No graph delay lines (slices 13-14).

## Objective gates

1. **Gap-free acceptance: a delay tail.** Session A: three tracks; one with a delay insert at 50 ms
   and high feedback, one with a multiband compressor insert. The source plays for 4 blocks, then
   drops to a deterministic noise floor near -80 dBFS (never an exact zero). Session B adds a muted
   track whose ID sorts first. Swap after block 6, while the tail rings. The swapped run and a fresh
   B fed the same PCM from frame 0 are bit-identical for 64 blocks. With this slice's section of the
   program disabled, the tail stops at block 7.
2. **Pending records and live bypass.** Prepared with live controls: a delay-time record admitted to
   A just before the swap (not yet rendered), and a live bypass of a per-node compressor insert (one
   track, so it does not bank) admitted a block earlier; B prepared from the committed model. Every
   block equals the reference fed the same records at the same blocks.
3. **Prepared bypass is not carried.** A committed bypass change on the delay: the join has no pair
   for it, and its successor instance starts at rest (decision 14 F4).
4. **Cost shape.** At 96 kHz the payload-bytes counter reads zero for the delay's owner (it moved),
   and the swap block makes zero allocations and frees.
5. Commands:
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a carry that leaves per-node effects at rest cuts the tail; it turns red.
- Gate 2: a move that keeps the successor's own lane state drops the acknowledged live bypass and the
  pending record; it turns red (the compressor's bypass is a lowered lane bypass, so the gate sees
  it).
- Gate 3: a rule that ignores the prepared bypass moves an unbypassed delay into a plan whose
  committed bypass is on; it turns red.
- Gate 4: a carry that copies a delay through the payload scratch (correct bits, about 1.5 MiB
  copied on the render thread) turns it red.

## Dependencies

- *Carry an insert lane that moves between a bank and a per-node instance* (#1281).
