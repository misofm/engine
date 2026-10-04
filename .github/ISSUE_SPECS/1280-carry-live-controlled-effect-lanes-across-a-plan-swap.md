# Carry live-controlled effect lanes across a plan swap

Slice 11 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A banked effect lane that a live control drives (the browser always; the C ABI after #1263) keeps its
exact state through a plan swap, together with its live state: its live bypass and dry shunt and its
pending records. No acknowledged live edit is lost: a record admitted but not yet rendered when the
swap happened is rendered by the successor, at any quantum, including a ramp in flight across the
swap.

## Context

- A live-controlled bank slot is `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:930`, impl
  `:1144`). Its `drain` (`:1247-1276`, run from `begin_block`) only **stages** records: each lane's
  `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:340`) packs automation spans or
  prepared targets, and `process_inner` (`:1286-1302`) applies them in the same block. Draining at the
  carry would stage records into a plan that never renders again (the umbrella's blocker X3).
- `EffectControlLane` (`live.rs:124`) holds the queue consumer, the prepared-target FIFO, the live
  `bypass` flag and the live symmetry terms. Its span window is exactly `automation_capacity` spans,
  overflow-safe only because preparation refuses a deeper queue (`live.rs:300-312`); its target FIFO
  holds exactly the queue's capacity (`:193-205`); a target that does not fit sets `target_error`,
  which fails the block (`:386-393`).
- The slot's dry shunt is one interleaved AoSoA line for the whole bank with one shared cursor
  (`rack/src/lib.rs:975-985`; `BypassShunt`, `live.rs:867-890`).
- The console-lane carry through the payload scratch, and exact mid-ramp payloads, come from *Carry
  console effect lanes across a plan swap* (#1279) and *Make every banked effect's state restore
  allocation-free* (slice 9).

## Decisions frozen for this slice

- **D1. Rule.** As slice 10's D1, with live controls attached in both plans or in neither.
- **D2. Inherited queue.** `EffectControlLane::inherit_from(&mut self, predecessor: &mut
  EffectControlLane)` moves the predecessor's queue consumer into a new `inherited` slot of the
  successor lane, and copies the predecessor's live `bypass` and live symmetry terms. The successor's
  next `stage` drains the inherited consumer (records present at entry) before its own consumer, into
  the same window. No prepared target is retained across a block boundary, so none is copied. The
  inherited consumer is never dropped on the render thread: it stays until the successor is retired,
  and its queue's bytes are charged to the successor.
- **D3. Capacity.** The window holds one queue's capacity. While a successor is pending, a host must
  admit to a successor lane only `capacity - unconsumed(predecessor lane)` records, refusing the rest
  with typed backpressure. Host-core gives the admission side one query per lane pair (the room left
  on a successor lane given its predecessor), and documents the rule on the producer type. The
  hosts apply it: the C ABI in its effect admission (#1264, #1265, #1266), the browser in B3b.
- **D4. Shunt.** A carried lane's shunt words move with it: a copy of that lane's words of the
  interleaved line, taken relative to the shared cursor of each bank.

## Deliverables

1. D2-D3 in `crates/effect-contract/src/live.rs` and `crates/host-core`; D4 in `crates/rack`; the
   location table and carry in `crates/graph`; the inventory rows and join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/effect-contract/src/live.rs`
- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/control_preparation.rs`,
  `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No bank ↔ per-node flips (slice 11b). No per-node moves (slice 12).
- No host admission change: the hosts adopt D3 in their own slices.

## Objective gates

1. **No acked record lost.** Prepared with live controls, a console EQ: an EQ gain prepared target
   `v0 → v1` is rendered in A; a prepared target `v1 → v0` is admitted to A just before the swap and
   not yet rendered; B is prepared from the committed model (`v0`). Every block equals the reference A
   plus the muted track fed the same targets at the same blocks. Repeat with a compressor threshold
   record (a span).
2. **Ramp across the swap at a small quantum.** At quantum 32, a compressor threshold record whose
   64-sample ramp is in flight at the swap: bit-identical to the reference.
3. **Capacity.** Fill the predecessor lane's queue to capacity minus 2 just before the swap, then
   admit through D3's query to the pending successor: it admits exactly 2 and refuses the third with
   backpressure; the successor's first block drops nothing (`dropped` stays 0) and raises no
   `target_error`; output bit-identical to the reference fed the admitted records.
4. **Live bypass and shunt.** A true-peak limiter insert on four tracks (a full bank at
   `Backend::Simd4`, eight at `Backend::Simd8`) with one lane bypassed live before the swap, the
   cursor at a nonzero offset: the bypassed lane keeps its delayed dry signal across the swap.
5. **Realtime.** Each swap block makes zero allocations and frees.
6. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a carry that drains the predecessor's queue into staged spans (lost with the retired plan)
  renders `v1` forever; it turns red.
- Gate 2: a payload that re-derives a ramp's step moves bits when the ramp crosses the swap; it turns
  red at quantum 32 (every other gate runs at 128).
- Gate 3: admission that ignores the predecessor's unconsumed records overflows the window and drops
  an acked span or fails the block; it turns red.
- Gate 4: a shunt copy that ignores the shared cursor turns it red.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279).
