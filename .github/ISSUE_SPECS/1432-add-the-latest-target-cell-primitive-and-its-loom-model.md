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
    - `RevisionGate::pair(initial) -> (RevisionGate, GatePin)` requires `initial <= GATE_REVISION_MAX`
      and builds the gate and its one pin together (follow-ups after attempt 1, m1). A gate belongs to
      one plan for the plan's life (#1502), so it has no re-arm operation.
    - `snapshot(&self) -> LiveSnapshot` (render, once per block): `fetch_or(ANNOUNCED, AcqRel)`.
      It returns the revision bits of the old word.
    - **Every write to the gate word is a read-modify-write**, and every read of it masks
      `ANNOUNCED`. A plain store would end the release sequences the proof uses.
  - **The control pin.** `GatePin` is control-private, one per gate, and has no public
    constructor. It holds `pinned: Option<u64>`, `last_published: u64` (at first the gate's initial
    revision) and `open: Option<u64>` (the revision stamped and not yet published).
    - `GatePin::publish(&mut self, gate, revision)`: `swap(revision, AcqRel)`. If the old word had
      `ANNOUNCED`, `pinned` becomes `Some(old revision)`; otherwise `pinned` is unchanged. Then
      `last_published := revision`.
    - `GatePin::stamp(&mut self, revision) -> LiveStamp { revision, pinned, published }`, where
      `published` is `last_published` at the stamp (follow-ups, m2). `A` is the newest value at or
      below `published`.
    - `publish` and `stamp` debug-assert that `revision` is above `last_published` and at most
      `GATE_REVISION_MAX`. A stamp at or below a revision already published would make the
      in-place rule below rewrite a slot render may be reading. They also debug-assert the rule
      "publish every stamped revision before a higher one is stamped": `stamp` refuses a second
      revision while `open` is set, and `publish` refuses a revision below `open`. `publish` also
      debug-asserts that the gate's old revision is `last_published`, which catches a pin used with
      another gate.
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

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b2` from `c4ca6f2dd`)

**What landed.**

- `crates/engine/src/realtime/latest_cell.rs` (new, safe Rust, atomics only): `RevisionGate`,
  `GatePin`, `LiveStamp`, `LiveSnapshot` (`ALL`), `GATE_REVISION_MAX`, `LatestCell<N>`,
  `CellWriter<N>`, `CellReader`, `CellRead<N>`, `DirtyWord`, `DirtyReader`. It follows D1-D3 and
  the design's state machine. `mod.rs` exports them. The loom shim is local to the module
  (`cfg(loom)` atomics); `spsc.rs` is untouched.
- Three render-side regions carry `REALTIME_POLICY` markers (the gate snapshot, the dirty take with
  `DirtyReader`, and `CellReader`). `scripts/check-realtime-policy.sh` floors rose to 27 files and
  96 regions. `docs/REALTIME_DEPENDENCY_POLICY.md` gained D4's one sentence.
- 12 unit tests (`latest_cell::tests`) and 5 loom models (`spsc_loom_cells_*`: L1, L2, L3, R2, L4).

**API choices the spec left open (for review).**

1. `CellReader::read(&mut self, cell, snapshot, stage: &mut DirtyReader, bit) -> CellRead<N>`.
   The read itself sets the deferred bit when a written slot is above `S`, so a stage cannot forget
   to defer. `DirtyReader::pending(&mut self, &DirtyWord) -> u64` is the deferred mask plus the
   take; it must follow the block's `snapshot()` in program order, which the caller owns (it is a
   documented rule, not a type rule).
2. `RevisionGate::new(initial)` and `GatePin::new(initial)` are separate (the spec: "one `GatePin`
   per gate"); `GatePin::new` must get the gate's initial revision. `LiveStamp` has private fields
   and accessors `revision()` and `pinned()`, so only `GatePin::stamp` makes one (the review's M2
   concern about a free `unpinned` constructor). `RevisionGate::revision()` is a masked read.
3. `GatePin::new` and `RevisionGate::new` use a hard `assert!` on `initial <= GATE_REVISION_MAX`
   (control-thread construction). `publish`, `stamp` and `write` debug-assert as the spec says.
4. `DirtyWord::mark` and `take` are private to the module; only `CellWriter::write` and
   `DirtyReader::pending` reach them.
5. `scripts/test-realtime-policy.sh` is outside the authorized path list, but its fixture sits
   exactly on the floors, so raising the floors made it red until the fixture gained one pad file
   (14 pad files, 27 files and 96 regions) and its three expected-failure strings changed.

**How the loom proofs avoid loom's store/read-modify-write imprecision.** Every atomic location
in every model is in exactly one of two classes. (a) Written only by read-modify-writes, after its
construction: the gate word (`swap`, `fetch_or`), the dirty word (`fetch_or`, `swap`) and L4's
mailbox word (`compare_exchange`). Loom is exact there. (b) Written only by plain `Relaxed`
stores: a slot's words, revision and sequence, which no model touches with a read-modify-write. No
model races a plain store against a read-modify-write on one location. Gate and dirty-word
initial values come from `AtomicU64::new`, not from a store. The module docs state the rule.
The loom leg is the real code on loom's atomics (the local shim), not a hand model. I cite no
mutation that depends on a plain-store gate.

**Gates.**

| Gate | Result |
|---|---|
| `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` | 13 passed (5 new, 8 existing), 24.5 s of model time |
| `cargo test --locked -p engine --lib latest_cell` | 12 passed (3 of them `should_panic` in debug) |
| `bash scripts/check-realtime-policy.sh` | ok (96 marked regions in 27 files) |
| `bash scripts/test-realtime-policy.sh` | ok |
| `cargo test --locked -p engine --features realtime-audit` | all pass (70 lib tests, 4, 1) |
| `bash scripts/check-cross-targets.sh` | PASS |
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `bash scripts/check-workspace-policy.sh` | ok |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) | all pass; shipped module `7cbdcff0...37e44` (the module is unreferenced from the browser host, so no pin is expected to move) |

**Mutation evidence.** Each row applies one defect to the committed file (red = at least that test
fails) and reverts it. "Unit" is `latest_cell::tests`; the loom column is `spsc_loom_cells_*`.

| Mutation | Loom red | Unit red |
|---|---|---|
| reader ignores `S` (the triple buffer) | L1, L2, L3, R2, L4 | `a_read_applies_only_what_the_snapshot_covers_and_defers_the_rest` |
| snapshot is a `load`, not a `fetch_or` | L2, R2, L4 | `the_pin_follows_the_announcement_and_survives_unannounced_publications` |
| control protects only `A` | L2, R2, L4 | none |
| control drops the pin on an unannounced swap | L2, L4 | `the_pin_follows_the_announcement_and_survives_unannounced_publications` |
| control picks the first slot that is not `A` (ignores `B` and the free-slot rule) | L2, R2, L4 | none |
| dirty bit set before the slot stores | L1, L2, L3, L4 | none |
| dirty `swap(0)` is `Relaxed` | L1, L2, L3, R2, L4 | none |
| reader never defers | L1, L2, L3, R2, L4 | `a_read_applies_only_what_the_snapshot_covers_and_defers_the_rest` |
| gate `fetch_or` and `swap` are `Relaxed` | L1, L3, R2, L4 | none |
| model takes the dirty bits before the snapshot | L1, L3, R2, L4 (corrected in the follow-ups: L4 was missed) | none |
| two writes of one revision are forced into two slots (no in-place rule; dropping the lookup alone is a benign variant, because slot choice is deterministic and every test stays green) | R2 only | `two_writes_in_one_revision_apply_the_second` |
| `peek_unread` consumes (sets the last applied sequence) | none | `peek_changes_nothing_and_a_read_counts_the_replaced_writes` |
| superseded count off by one | L1, L2, L3, R2, L4 | five tests |
| snapshot does not mask `ANNOUNCED` | (the run aborts) | `the_announcement_never_leaks_into_a_revision` and one more |
| `stamp`'s debug assert removed | none | `a_stamp_at_the_last_published_revision_panics` |
| `publish`'s debug assert removed | none | `a_publication_at_the_last_published_revision_panics` |
| `write`'s non-decreasing debug assert removed | none | `a_write_below_a_written_revision_panics` |
| ceiling constant `2^63 - 2` | (the run aborts) | `the_ceiling_is_the_largest_revision_a_gate_holds` and five more |
| `RevisionGate::new` accepts a revision above the ceiling | none | `a_gate_above_the_ceiling_is_refused` |
| slot tag stored before the words (the design's "store order is free" claim) | none (green, as claimed) | none |

L1 stays green under `protect_only_a`, `writer_forgets_pin` and the first-slot mutation: its
control thread writes a cell at most twice, so it never needs `B`. L2 and L4 (three and four
commits) are the models that reach it, which matches the design's table.

**Test value, one line each.** L1: the block applies one cell of a transaction and not the other
(red under the triple buffer, a take before the snapshot, `Relaxed` gate operations). L2/L4: control
overwrites the slot a pinned or just-claimed reader needs (red under the no-announcement, `A`-only
and forgot-the-pin defects). L3: a committed value lost between a take and its deferral. R2: two
writes of one revision in two slots (the only model that catches it; the unit test catches it
sequentially). The unit tests defend the counts, the peek's purity, the deferral order, the
masking, the pin's survival across unannounced publications and the debug panics, each shown red
above.

**Open items.** None blocking. Not done by design: the type does not force `pending` after
`snapshot` (a documented rule; model mutation "take before the snapshot" shows the loom catch).

### Follow-ups after attempt 1 verdict

Verdict: PASS with MINORs m1, m2 and NITs n2, n4 (`/home/bl/misofm/submix-verdicts/1432-attempt1.md`).
One commit on `codex/d15-stream-b2`. Every change is in `latest_cell.rs` and this spec.

**m1: a pin cannot be built apart from its gate.**

- `RevisionGate::new` and `GatePin::new` are gone. `RevisionGate::pair(initial) -> (RevisionGate,
  GatePin)` builds both; the pin starts at the gate's own revision, and nothing else makes a pin.
  A pin below its gate's initial revision is now unrepresentable.
- A pin used with a different gate stays representable (a pair can be split), so `publish`
  debug-asserts that the gate's old revision is `last_published`. It reads the `swap`'s result, so
  it costs no atomic operation. A gate has exactly one publisher, so the check holds for every
  correct caller.
- The loom models take the pin from `Shared::new` (L1, L2, L3, R2) and from the plan builder (L4).
- Note for #1502 and #1277: a gate built at revision `r` cannot take a stamp of `r`. A candidate
  whose retargets are stamped at `r` needs a gate built below `r` (L4 builds gate 0, then writes
  and publishes 2).

**m2: the publish-before-a-higher-stamp rule.**

- The rule is on `GatePin::stamp` and `CellWriter::write`, and in the module docs.
- `GatePin` tracks `open`, the revision stamped and not yet published. In a debug build `stamp`
  refuses a second revision while `open` is set (the same revision may stamp again), and `publish`
  refuses a revision below `open`.
- The type carries `published` in `LiveStamp`, and `A` is now the newest value at or below it
  (before: below the stamp's revision, "hence published"). Under the rule the two are equal. When
  a caller breaks the rule, even in a release build, `A` never follows an unpublished value, so an
  abandoned revision cannot shield a slot render holds. The probe in the verdict now reads the
  held value. An abandoned value's leak stays the caller's contract, as the verdict says.
- #1312's brief must carry the rule (the verdict's note).

**n2: L4 counts P.** It asserts that P's resolved writes equal 1 if P ran a block and 0 if C was
claimed first.

**Mutation evidence for the new and changed tests.** Each row applies one defect to the committed
file and reverts it (revert: all 17 unit tests and the 13 loom tests green, run last).

| Mutation | Red |
|---|---|
| `pair` starts the pin at 0 | `a_pair_starts_the_pin_at_the_gates_revision` |
| `publish`'s gate-identity assert removed | `a_pin_publishes_only_to_its_own_gate` |
| `stamp`'s open-revision assert removed | `a_second_revision_is_not_stamped_before_the_first_is_published` |
| `publish`'s open-revision assert removed | `a_publication_below_the_stamped_revision_panics` |
| `stamp` does not record `open` | the two tests above |
| `A` back to "below the stamp's revision" | `an_abandoned_revision_never_shields_the_slot_render_holds` (no loom model reaches it: the models obey the rule) |
| P's resolved count over by one (model only) | L4, on the new P assertion (line 1299) |
| the dirty take before the snapshot (L4 model) | L4 (the missed row of the attempt 1 table) |
| two writes of one revision forced into two slots | R2 and `two_writes_in_one_revision_apply_the_second` |
| control protects only `A`, rerun on the new `A` | L2, R2, L4 |
| control drops the pin on an unannounced swap, rerun | L2, L4 |

Test value, one line each. `a_pair_starts_the_pin_at_the_gates_revision`: a pair whose pin disagrees
with its gate (the verdict's m1 probe). `a_pin_publishes_only_to_its_own_gate`: a pin used with
another gate, which corrupts `pinned`. `a_second_revision_is_not_stamped_before_the_first_is_published`
and `a_publication_below_the_stamped_revision_panics`: an abandoned or reordered revision, the
verdict's m2. `an_abandoned_revision_never_shields_the_slot_render_holds`: the m2 probe in a release
build, where the asserts are off. `a_write_below_a_written_revision_panics` was rewritten: its old
body now trips `stamp`'s assert first, so it holds an older stamp across a later publication, the
only way left to reach `write`'s own assert. `the_all_snapshot_applies_the_newest_write` now
publishes revision 1 before it writes 5, as the rule requires.

**Gates (follow-ups commit).** All green: the CI loom command (13 passed), `cargo test --locked -p
engine --features realtime-audit`, `check-realtime-policy.sh` (96 regions in 27 files),
`test-realtime-policy.sh`, `cargo fmt --check`, workspace clippy `-D warnings`, `cargo doc` with
`-D warnings`, `check-workspace-policy.sh` and `check-cross-targets.sh` (PASS). The worklet chain was
not re-run: the module is not referenced from the browser host.
