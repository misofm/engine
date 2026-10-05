# Carry live-controlled effect lanes across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 11 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A banked effect lane that a live control drives keeps its exact state through a plan swap. The
browser always attaches live controls, and the C ABI does since #1263. The lane keeps its live
state too: its live bypass, its dry shunt and its pending records. No acknowledged live edit is
lost: a record admitted but not yet rendered at the swap is rendered by the successor, at any
quantum, including a ramp in flight across the swap. A structural transaction that also changes a
carried lane's live parameter or bypass sounds exactly like "live edit, then structural edit"
(D15-7). The same holds in copy mode, where the predecessor keeps rendering.

## Context

- A live-controlled bank slot is `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:937`, impl
  `:1151`). Its `drain` (`:1257`, run from `begin_block`, `:1211`) only **stages** records. Each
  lane's `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:341`) packs automation
  spans or prepared targets, and `process_inner` (`rack/src/lib.rs:1294`) applies them in the same
  block. Draining at the carry would stage records into a plan that never renders again (#1269's
  blocker X3).
- `EffectControlLane` (`live.rs:125`) holds the queue consumer, the prepared-target FIFO, the live
  `bypass` flag and the live symmetry terms.
  - Its span window is exactly `automation_capacity` spans. It is overflow-safe only because
    preparation refuses a deeper queue (`live.rs:300-312`).
  - Its target FIFO holds exactly the queue's capacity (`:184-190`). A target that does not fit sets
    `target_error`, which fails the block (`:387-392`).
- The slot's dry shunt is one interleaved AoSoA line for the whole bank, with one shared cursor
  (`shunt`, `rack/src/lib.rs:956-958`; `BypassShunt`, `live.rs:868-890`).
- The console-lane carry through the payload scratch, the effect rule D1 and the refusal counter
  come from *Carry console effect lanes across a plan swap* (#1279). The base, the retarget pattern
  and the restart set come from #1277 (D2-D6), and the copy-mode rules from #1322.

## Decisions frozen for this slice

- **D1. Rule.** As #1279 D1, with live controls attached in both plans or in neither. Live
  parameter values and a lowered bypass are live, because the lane exists. Everything #1279 D1
  lists as prepared stays prepared.
- **D2. Inherited queue (move mode).** `EffectControlLane::inherit_from(&mut self, predecessor:
  &mut EffectControlLane)` does three things:
  - it moves the predecessor's queue consumer into a new `inherited` slot of the successor lane;
  - it copies the predecessor's live `bypass` and live symmetry terms;
  - it copies no prepared target, because none is retained across a block boundary.

  The successor's next `stage` drains the inherited consumer (the records present at entry) before
  its own consumer, into the same window. The inherited consumer is never dropped on the render
  thread: it stays until the successor is retired, and its queue's bytes are charged to the
  successor.
- **D3. Capacity.** The window holds one queue's capacity. While a successor is pending, a host may
  admit to a successor lane only `capacity - unconsumed(predecessor lane)` records, and it refuses
  the rest with typed backpressure before it commits. Host-core gives the admission side one query
  per lane pair: the room left on a successor lane, given its predecessor. The rule is documented on
  the producer type. This slice applies the query in the control plane's effect admission
  (`commit_live`'s effect arm, `crates/capi/src/runtime/control.rs:1111-1130` on `6fb211594`; in
  `crates/control-plane/src/` once *Extract the C ABI control plane into a portable crate both hosts
  call* (#1309) has moved it), so no acked record
  can overflow the inheriting window. *Hold live values in latest-target cells on both hosts*
  (#1312) later replaces these queues with cells and keeps the rule's guarantee.
- **D4. Carry, then retarget (D15-7).** The per-effect record derivation inside
  `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) is extracted into one per-owner
  function, as #1277 D5 did for strips. For every carried lane whose live values differ, the
  successor entry points push its records into the successor's producer before they return: the
  `Bypass` record first, then the `Parameter` records, or an EQ's prepared targets designed from
  the base values. The records count against D3's room. If they do not fit, the preparation fails
  with typed backpressure before any commit. In copy mode the records are kept for publication
  (#1277 D5).
- **D5. Shunt.** A carried lane's shunt words move with it, as a copy of that lane's words of the
  interleaved line, taken relative to the shared cursor of each bank. The same holds in both modes.
- **D6. Copy mode (#1322 D7).** The predecessor keeps its consumer, so nothing is inherited. The
  successor lane copies the live `bypass` and symmetry terms, the payload (#1279 D4) and the shunt
  words.
  - A predecessor lane that holds unconsumed records cannot be copied exactly: its records are
    staged only inside a block. In that case the whole copy refuses: nothing is written,
    `PredecessorMismatch` is reported, and a graph counter of copy refusals is incremented.
  - D15-17 holds live edits once a catch-up starts, so no record is pending at a copy and the
    refusal is a checked invariant.

## Deliverables

1. D2 and D6's lane copy in `crates/effect-contract/src/live.rs`. D3's query and D4 in
   `crates/host-core`. D5 in `crates/rack`. The location table and both modes in `crates/graph`.
   The inventory rows and the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/effect-contract/src/live.rs`
- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/host-core/src/live_delta.rs`: D4's extraction only. This is stream B's file, and root
  orders the merge with #1312.
- `crates/capi/src/runtime/control.rs` before #1309, or `crates/control-plane/src/` after it, and
  `crates/capi/src/runtime/live_tests.rs`:
  D3's admission check only. These are stream B's files.

## Non-goals

- No bank to per-node flips (#1281), and no per-node moves (#1282).
- No browser admission change: the browser admits through the same control plane once
  *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) lands.
- No latest-target cells (#1312, which rebases onto this slice).

## Objective gates

1. **No acked record lost.** Prepare with live controls and a console EQ. An EQ gain prepared
   target `v0 → v1` is rendered in A. A prepared target `v1 → v0` is admitted to A just before the
   swap and not yet rendered. B is prepared from the base (`v0`). Every block equals the reference
   A plus the muted track, fed the same targets at the same blocks. Repeat with a compressor
   threshold record (a span).
2. **Ramp across the swap at a small quantum.** At quantum 32, a compressor threshold record whose
   64-sample ramp is in flight at the swap is bit-identical to the reference.
3. **Capacity.** Fill the predecessor lane's queue to capacity minus 2 just before the swap. Then
   admit through D3's query to the pending successor. It admits exactly 2 and refuses the third
   with backpressure. The same case through the C ABI (`crates/capi/src/runtime/live_tests.rs`)
   returns `MISO_ENGINE_V1_BACKPRESSURE` for the third effect transaction and commits nothing. The successor's first block drops nothing (`dropped` stays 0) and raises no
   `target_error`. The output is bit-identical to the reference fed the admitted records.
4. **Live bypass and shunt.** A true-peak limiter insert on four tracks (a full bank at
   `Backend::Simd4`; eight tracks at `Backend::Simd8`), with one lane bypassed live before the swap
   and the cursor at a nonzero offset. The bypassed lane keeps its delayed dry signal across the
   swap.
5. **Carry, then retarget.** B also changes a carried lane's compressor threshold and an EQ band
   gain (live values). Run 1 is the structural swap with D4's retarget. Run 2 pushes the same edits
   to A just before the swap and prepares B from a base that holds them. The two runs are
   bit-identical in every block, and the strip is not in the restart set.
6. **Copy mode.** Gates 2 and 4 with a copy after the swap block's predecessor block, followed at
   once by the adoption, with no pending record. Every block equals the move run, and A equals its
   uncopied twin. A second case leaves one record pending at the copy: the copy refuses, the
   refusal counter is 1, and both plans are unchanged.
7. **Realtime.** Every swap block and copy call makes zero allocations and frees.
8. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core -p capi --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && bash scripts/trace-effect-contract-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that drains the predecessor's queue into staged spans, which are lost with the
  retired plan, renders `v1` forever. It turns red.
- Gate 2: a payload that re-derives a ramp's step moves bits when the ramp crosses the swap. It
  turns red at quantum 32; every other gate runs at 128.
- Gate 3: admission that ignores the predecessor's unconsumed records overflows the window, and
  drops an acked span or fails the block. It turns red.
- Gate 4: a shunt copy that ignores the shared cursor turns it red.
- Gate 5: retarget records pushed ahead of the inherited ones apply the old value last. A missing
  retarget keeps the old threshold. Either turns it red.
- Gate 6: a copy mode that silently drops a pending record (no refusal) turns it red. So does a
  copy that moves the consumer out of the still-rendering predecessor.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279).
