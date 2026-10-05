# Add the latest-target cell primitive and its loom model

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Split out of *Hold live values in latest-target cells on both hosts* (#1312) by root on 2026-10-05
under AGENTS.md's half-day rule. Its decisions are #1312's D1 and D2's counting rule, unchanged.

## Product outcome

The engine has a latest-target cell: a safe-Rust, allocation-free triple buffer that one control
thread writes and render reads. A write cannot fail and never waits. A read takes the newest
completed write in one pass, never a torn or mixed value, applies each write at most once, and
never skips the newest write that completed before the read began. The reader reports how many
written values a later write replaced before it read them. A carry can peek an unread value
without consuming it. #1312 puts the strip fader, mute and matrix lanes on this primitive;
#1345, #1346 and #1347 reuse it.

## Context

- **Realtime root.** `scripts/check-realtime-policy.sh` admits no `unsafe` in a new realtime
  file; `crates/engine/src/realtime/observe.rs` shows the safe-Rust atomics-only style.
- **Loom.** The loom shim and the `spsc_loom` filter live in `crates/engine/src/realtime/spsc.rs`;
  CI runs `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
  --locked --release -p engine --lib spsc_loom`.
- **Users.** #1312 (strip fader, mute and matrix cells, the stage dirty words, the session
  counter), #1277 (writes carry-then-retarget values), #1280 (peeks predecessor cells), #1345,
  #1346, #1347.

## Decisions frozen for this slice

- **D1. The cell: a triple buffer that never tears and never skips** (#1312 D1). New module
  `crates/engine/src/realtime/latest_cell.rs`, safe Rust (atomics only, no `unsafe`).
  - A cell has three slots. Each slot holds the cell's words (`AtomicU32` each: the target and its
    ramp, one unit; the word count is fixed per cell when it is built) and an `AtomicU64` sequence
    number.
  - A `middle: AtomicU32` holds a slot index and a `FRESH` bit. The writer (control) owns a
    private back index and next sequence. The reader (render) owns a private front index and the
    last sequence it applied.
  - **Write:** store the words and the sequence into the back slot (`Relaxed`). Then
    `prev = middle.swap(back | FRESH, AcqRel)`; the back index becomes `prev`'s index. Then
    `fetch_or` the cell's bit into its stage's dirty word (`Release`). Writes cannot fail.
  - **Read (render, at the drain):** `dirty.swap(0, Acquire)`. For each set bit whose `middle`
    has `FRESH`: `prev = middle.swap(front, AcqRel)`; the front index becomes `prev`'s index; read
    the front slot. The three indices are always distinct, so render never reads a slot the
    writer is writing. Render reads the newest completed write in one pass: no retry, no spin, no
    skip.
  - A write that completes before a block's drain begins is applied in that block (D15-2
    condition 2).
  - **Peek (carry only):** `peek_unread(&self) -> Option<(words, sequence)>` returns the `middle`
    slot's words and sequence when `middle` has `FRESH`, and `None` otherwise. It changes nothing:
    `middle`, the front index, the last applied sequence and the dirty word stay as they were, so
    a later read still applies the value. It is valid only once the writer is quiescent (no write
    to this cell can start before the peek ends), because a write after the peek began may reuse
    that slot. The plan-swap carry (#1280 D2) is its only caller. The reader also exposes
    `last_applied(&self) -> u64`.
  - The dirty word is a small type of this module that a stage owns and shares among its cells
    (one bit per cell); the read API takes it. Which cells a stage has is #1312's (#1312 D3).
- **D2. The supersession count** (#1312 D2's unit). When the reader reads sequence `s` after
  `p`, the read reports `s - p - 1` replaced values for that cell. The caller adds it to its
  counter; this module holds no session counter.
- **D3. Placement and exports.** `crates/engine/src/realtime/mod.rs` exports the cell, its writer
  and reader halves and the dirty word. Construction allocates (control thread); write, read and
  peek never allocate, lock or make a syscall.

## Deliverables

1. `latest_cell.rs` with D1-D3, `peek_unread` and `last_applied`.
2. Its loom models and unit tests (gate 1).

## Authorized paths

- `crates/engine/src/realtime/latest_cell.rs` (new), `crates/engine/src/realtime/mod.rs`
  (exports), `crates/engine/src/realtime/spsc.rs` (only if the loom shim needs a type it lacks).

## Non-goals

- Any lane on cells, admission change, counter registry entry, header or docs text (#1312).
- The carry (#1277, #1280).

## Hazards

- A peek that swaps `middle` or clears `FRESH` or the dirty bit consumes the value: #1280's carry
  and its count rely on a peek that changes nothing.
- A read that loops until `FRESH` clears would be unbounded on render.

## Objective gates

1. **Never skip under a racing writer (loom, new `spsc_loom_cells_*` tests in `latest_cell.rs`).**
   A writer making two writes racing a reader that reads before and after: the reader never sees
   a mixed cell, applies each sequence at most once, and, once the writer's last write completed
   before a read began, that read returns it. The reported supersession count equals writes
   minus applies. Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom
   --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`. A plain unit
   test in the same module: after three writes and no read, `peek_unread` returns the third
   write's words and sequence, a second peek returns the same, and the next read then applies
   that value and reports 2 replaced.
2. **Realtime.** `bash scripts/check-realtime-policy.sh` (no `unsafe` in the new module; raise its
   floors by the regions this adds if the script's own rule requires it) and
   `bash scripts/test-realtime-policy.sh`.
3. **Workspace.** `cargo test --locked -p engine --features realtime-audit`;
   `bash scripts/check-cross-targets.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

## Test value

- Gate 1: a cell that tears, applies twice, or skips the latest completed write (the single
  sequence-word design this replaces would skip it); judged by the interleavings loom reaches.
- Gate 1's peek case: a `peek_unread` that consumes the value, so a second peek or the next read
  no longer sees it.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309) (stream order).

Dependents: #1312, then #1277, #1280, #1345, #1346, #1347.
