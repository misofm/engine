# Add the latest-target cell primitive and its loom model

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Split out of *Hold live values in latest-target cells on both hosts* (#1312) by root on 2026-10-05
under AGENTS.md's half-day rule. Rewritten by Amendment 1 (root, 2026-10-05) to the
revision-bounded cell. Code anchors verified on `codex/d15-stream-b` at `13f335d79`.

## Product outcome

The engine has a revision-bounded latest-target cell and a revision gate. They are safe-Rust,
allocation-free primitives: one control thread writes them, and render reads them.

- Control writes a transaction's values into cells, then publishes the transaction's revision
  through the gate.
- Render takes one snapshot `S` of the gate per block. Every read in that block applies, per cell,
  the newest value whose revision is `<= S`. It never applies a newer value, a torn value or a
  value twice, and it never skips that value. So every value of one revision takes effect in one
  block.
- A write cannot fail and never waits, however far control laps render.
- The reader reports how many writes a later write replaced before a snapshot covered them.
- A carry can peek an unread value without consuming it.

*Give each plan its own revision gate and take each block's live snapshot from it* (#1502)
puts one gate in every plan. #1312 puts the strip fader, mute and matrix lanes on these
primitives. #1345, #1346, #1347, #1371 and #1390 reuse them.

## Context

- **Root's binding requirement (2026-10-05).** Today each strip drains its live lanes at its own
  node inside `render_inner`. So a transaction ("mute A, unmute B") committed while render is
  mid-block can apply to A in block `k` and to B in block `k+1`, and the watermark's
  `first_sample` can name a later block than the one that applied a value (#1314's attempt
  record). The contract is now: every live value of one committed revision takes effect in the
  same block, and the watermark's `first_sample` is exact.
- **Design and review.** The design, its proofs and the measured loom mutations are in
  `docs/handoffs/decision-15-2026-10-05/revision-bounded-cells/design.md`. The adversarial review
  (verdict SOUND, with findings) is `review.md` beside it, with the prototype and the review's
  loom models.
  - The design rejects four alternatives, each with a concrete interleaving: the triple buffer
    (this issue's old D1), a two-entry cell, per-cell revision tags without a reader
    announcement, and a separate announcement word. Moving every drain to block start shrinks the
    race window but does not close it.
  - Root ruled (2026-10-05) that the gate lives in the plan, one gate per plan (#1502).
- **Realtime root.** `scripts/check-realtime-policy.sh` admits no `unsafe` in a new realtime
  file. `crates/engine/src/realtime/observe.rs` shows the safe-Rust, atomics-only style.
- **Loom.** The loom shim and the `spsc_loom` filter live in `crates/engine/src/realtime/spsc.rs`.
  CI runs `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo
  test --locked --release -p engine --lib spsc_loom`.
  - Loom 0.7.2 handles a plain store that races a read-modify-write too permissively. In a probe,
    the final `fetch_or` read the first `fetch_or`'s write after both plain stores. C++20 forbids
    that. On a location written only by read-modify-writes, loom is exact.
- **Users.** #1502 (the plan gate), #1312 (strip cells, the stage dirty words, the session
  counter), #1277 (writes retarget values), #1280 (peeks predecessor cells), #1345, #1346, #1347,
  #1371, #1390.

## Decisions frozen for this slice

- **D1. The gate, the cell and their orderings.** New module
  `crates/engine/src/realtime/latest_cell.rs`, safe Rust (atomics only, no `unsafe`).
  - **The gate.** `RevisionGate` is one `AtomicU64`, embeddable (no `Arc` of its own).
    - Bit 63 is `ANNOUNCED`. Bits 0-62 are the revision. `GATE_REVISION_MAX = 2^63 - 1`.
    - `RevisionGate::new(initial)` requires `initial <= GATE_REVISION_MAX`. A gate belongs to one
      plan for the plan's life (#1502), so it has no re-arm operation.
    - `snapshot(&self) -> LiveSnapshot` (render, once per block): `fetch_or(ANNOUNCED, AcqRel)`.
      It returns the revision bits of the old word.
    - **Every write to the gate word is a read-modify-write**, and every read of it masks
      `ANNOUNCED`. A plain store would end the release sequences the proof uses.
  - **The control pin.** `GatePin` is control-private, one per gate. It holds `pinned:
    Option<u64>` and `last_published: u64` (at first the gate's initial revision).
    - `GatePin::publish(&mut self, gate, revision)`: `swap(revision, AcqRel)`. If the old word had
      `ANNOUNCED`, `pinned` becomes `Some(old revision)`; otherwise `pinned` is unchanged. Then
      `last_published := revision`.
    - `GatePin::stamp(&self, revision) -> LiveStamp { revision, pinned }`.
    - `publish` and `stamp` debug-assert that `revision` is above `last_published` and at most
      `GATE_REVISION_MAX`. A stamp at or below a revision already published would make the
      in-place rule below rewrite a slot render may be reading.
  - **`LiveSnapshot`** wraps the revision a block reads. `LiveSnapshot::ALL` is
    `GATE_REVISION_MAX`. Its only user is a drain of a quiescent predecessor plan (#1312 D6); no
    production render path takes it.
  - **The cell.** `LatestCell<const N: usize>` has three slots. Each slot holds `N` `AtomicU32`
    words (the target and its ramp, one unit), an `AtomicU64` revision (`u64::MAX` until first
    written) and an `AtomicU64` sequence.
    - `CellWriter<N>` is control-private. It holds a mirror of the three slot revisions and the
      next sequence (starting at 1).
    - `CellReader` is render-private. It holds the last applied sequence (0 for none).
    - `DirtyWord` is one `AtomicU64` that a stage shares among its cells, one bit per cell.
      `DirtyReader` is render-private and holds a deferred mask.
  - **Write** `(stamp, words)` on cell `c` with dirty bit `b`. It cannot fail and never waits.
    1. Choose the target slot.
       - If a slot's mirrored revision equals `stamp.revision`, rewrite that slot in place, so
         one revision occupies at most one slot.
       - Otherwise let `A` be the slot with the greatest mirrored revision `< stamp.revision`
         (the newest published value). Let `B` be the slot with the greatest mirrored revision
         `<= stamp.pinned`, if `pinned` is `Some`. Choose a slot that is neither `A` nor `B`;
         three slots guarantee one.
    2. Store the words, the next sequence and `stamp.revision` into it (`Relaxed`, any order).
       Update the mirror.
    3. `dirty.fetch_or(1 << b, Release)`, after step 2.

    Stamp revisions per cell are non-decreasing (debug-asserted).
  - **Read** (render, after the block's snapshot, in the stage's canonical order).
    - The pending set is the deferred mask, plus `dirty.swap(0, Acquire)` when
      `dirty.load(Relaxed) != 0`. An idle stage pays one `Relaxed` load. Every take of a block
      follows the block's snapshot in program order.
    - Per pending cell, load the three slot revisions (`Relaxed`) and choose the greatest one
      `<= S`, skipping unwritten slots.
      - If its sequence is above the last applied sequence, load its words (`Relaxed`) and return
        `Applied { words, sequence, superseded }`; the last applied sequence becomes `sequence`.
      - Otherwise return `Unchanged`.
    - A slot revision `> S` keeps the cell in the deferred mask.
    - Render never loads the words of a slot it did not choose.
  - **Peek** (carry only). `peek_unread(&self) -> Option<(words, sequence)>` returns the written
    slot with the greatest sequence when that sequence is above the last applied one, and `None`
    otherwise.
    - It changes nothing: no read-modify-write, and the last applied sequence, the dirty word and
      the deferred mask stay as they were.
    - It is valid only once the writer is quiescent (no write to this cell can start before the
      peek ends). Its only caller is the plan-swap carry (#1280 D2).
    - `last_applied(&self) -> u64` stays.
- **D2. The supersession count.**
  - A *write* is one `CellWriter::write` call. A block *covers* a write when the block's snapshot
    is `>=` the write's revision.
  - At the first block of a plan that covers it, a write is either applied (it is the cell's
    newest write `<= S`) or superseded. When render applies sequence `s` after `p`, the read
    reports `s - p - 1` (saturating) for that cell.
  - A write that no block covers is not counted here. #1312 D2 states the counter's unit.
  - The caller adds the count to its counter; this module holds no session counter.
- **D3. Placement and exports.**
  - `crates/engine/src/realtime/mod.rs` exports the gate, `GatePin`, `LiveStamp`, `LiveSnapshot`,
    `GATE_REVISION_MAX`, the cell, its writer and reader, the dirty word and the dirty reader.
  - Construction allocates (control thread). `snapshot`, `publish`, `stamp`, write, read and peek
    never allocate, lock or make a syscall.
  - Putting a gate in every plan is #1502's.
- **D4. Cost (the slice's measurable claim).**
  - Render per block: one read-modify-write per plan for the snapshot, one `Relaxed` load per
    idle stage, and for each pending cell three revision loads, one sequence load and `N` word
    loads. There is no retry and no spin.
  - On AArch64 without LSE (ARMv8.0) a read-modify-write compiles to a load-linked or
    store-conditional loop. That is the same property the mailbox claim and the dirty swap
    already have. `docs/REALTIME_DEPENDENCY_POLICY.md` states it.

## Deliverables

1. `latest_cell.rs` with D1-D3.
2. Its loom models and unit tests (gate 1).

## Authorized paths

- `crates/engine/src/realtime/latest_cell.rs` (new), `crates/engine/src/realtime/mod.rs`
  (exports).
- `crates/engine/src/realtime/spsc.rs`, only if the loom shim needs a type it lacks. A local
  `cfg(loom)` shim in `latest_cell.rs` is allowed.
- `scripts/check-realtime-policy.sh` (its file and region floors only, raised to the measured
  counts; the script's own rule requires the raise in the change that adds a marker).
- `docs/REALTIME_DEPENDENCY_POLICY.md` (D4's sentence only).

## Non-goals

- A gate in each plan, the block snapshot in render and the watermark (#1502).
- Any lane on cells, admission change, counter registry entry, header or docs text (#1312).
- The carry (#1277, #1280).

## Hazards

- A reader that takes `S` with a plain load, or a gate written with a plain store, loses the
  announcement or the release sequence. Control can then overwrite the slot render's snapshot
  needs. The design's mutation rows `reader_load_not_rmw`, `protect_only_a` and
  `writer_forgets_pin` turn L2 red. A plain-store mutation is also red, but partly through loom's
  store imprecision (Context), so it is not cited as evidence.
- A reader that chooses the newest slot regardless of `S` (the triple buffer) tears a transaction
  across blocks (L1, L2 and L3 red).
- Setting the dirty bit before the slot stores, a `Relaxed` take, or a reader that does not defer
  a cell whose newest revision is `> S` loses a committed value (L1, L2 and L3 red).
- Taking the dirty bits before the snapshot delays a covered value by one block (L1 and L3 red).
- Two writes of one revision in two slots let render apply the older one (R2 red).
- A peek that swaps or clears anything consumes the value. #1280's carry and its count rely on a
  peek that changes nothing.
- No model races a plain store against a gate (Context).

## Objective gates

1. **Revision-bounded, never torn, never lost (loom, `spsc_loom_cells_*` in
   `latest_cell.rs`).** In each model, after a final quiescent block,
   `superseded + applications == writes`.
   - **L1** `spsc_loom_cells_one_block_never_tears`: two cells, two transactions that each write
     both, racing one block. The block applies both cells at `S`, or neither, never one.
   - **L2** `spsc_loom_cells_a_pinned_reader_survives_a_lapping_writer`: render pinned at
     revision 1 while control commits 2, 3 and 4 applies exactly revision `S`'s words.
   - **L3** `spsc_loom_cells_a_dirty_bit_is_never_lost`: a write committed while a block drains is
     applied by a later block.
   - **R2** `spsc_loom_cells_an_in_place_rewrite_races_reads`: two writes per revision (2 and 3)
     race two blocks. Each block applies the second write of revision `S` (or revision 1's only
     write).
   - **L4** `spsc_loom_cells_a_claim_races_stamped_writes` (the review's R1).
     - Plan P runs on gate 0. Candidate C's cell holds a write at revision 2, published on C's
       gate before C is published by a `Release` compare-and-swap of a mailbox word.
     - Control then commits 3, 4 and 5 to C, each stamped from C's pin and published on C's gate.
       At least two commits after the publication exercise C's `B` protection.
     - Render claims C with an `AcqRel` compare-and-swap, takes C's snapshot and drains. While it
       runs C, every block applies exactly revision `S`.
   - Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)'
     cargo test --locked --release -p engine --lib spsc_loom`.
   - Unit tests in the same module:
     - three writes and no read: `peek_unread` returns the third write's words and sequence,
       twice. A read at `S = 3` then applies it with 2 superseded, and the next peek is `None`;
     - two writes in one revision: the read applies the second, with 1 superseded;
     - 9,999 commits after one read: one read applies the last, with 9,998 superseded, and no
       write failed;
     - in debug builds, a stamp at or below the last published revision panics, and so does a
       publish of one.
2. **Realtime.** `bash scripts/check-realtime-policy.sh` (no `unsafe` in the new module; raise its
   floors by the regions this adds) and `bash scripts/test-realtime-policy.sh`.
3. **Workspace.** `cargo test --locked -p engine --features realtime-audit`;
   `bash scripts/check-cross-targets.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

## Test value

- L1: a reader that ignores `S` (the triple buffer), a dirty take before the snapshot, or gate
  orderings weakened to `Relaxed`.
- L2: a reader without the announcement, or a writer that protects only the newest published slot
  or forgets the pin.
- L3: a dirty bit set before the slot stores, a `Relaxed` take, or a reader that does not defer.
- R2: a writer that puts two writes of one revision in two slots. L1-L3 stay green under it in
  loom; only R2 and the sequential unit test catch it.
- L4: a candidate's pin not honoured across its claim (red under `protect_only_a`,
  `reader_load_not_rmw` and `reader_never_defers`).
- The peek test: a peek that consumes.
- The debug-panic tests: a stamp or publish at or below the last published revision, which would
  rewrite a published slot in place.

## Amendment 1 (root, 2026-10-05)

Root's binding requirement (Context) replaced this issue's triple buffer before implementation.
No attempt had started. Under it, the triple buffer must either apply a value newer than `S` or
lose the value `<= S`. Root accepted the revision-bounded design, and the adversarial review found
it SOUND. Root's rulings (2026-10-05):

1. **One gate per plan**, not per mailbox cell (#1502). The gate needs no re-arm, and no
   production render path needs `LiveSnapshot::ALL`.
2. **The revision ceiling is `GATE_REVISION_MAX = 2^63 - 1`.**
   *Bound committed revisions at the plan gate's ceiling* (#1503) enforces it in the protocol
   store; this primitive debug-asserts it.
3. **An executor error after some drains** is a counted defect path in the render diagnostics
   (#1502), not an atomic block.
4. **The counter's unit** is writes to a plan that render adopts (#1312 D2).
5. **#1345 D4's span window** holds every live cell, so no deferral splits a revision.

Folded review findings:

- m1: the lemma covers only writes whose revision is `> S`.
- m2: the stamp and publish debug checks (D1).
- m5: T5's scope (D2).
- n1: bits 0-62.
- n2: the hazard citations.
- n3: the cost model (D4).
- n5: the masking rule (D1).
- n6: L4 is based on R1, with at least two commits after publication.

The design's L1-L3 and the review's R1 and R2 pass in the prototypes. The prototype and the
review's models are in the handoff folder.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309) (stream order).

Dependents: #1502, then #1503, *Hand each block's live snapshot to every live drain*
(#1504) and #1312; then #1277, #1280, #1345, #1346, #1347.
