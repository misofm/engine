# Hold live edits during a catch-up and apply them at the adoption sample

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 5, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W5 first bullet. Code anchors verified
on `main` at `6fb211594`.

## Product outcome

While a warm successor catches up, a host's live edits (fader, mute, pan, sends, input, effect
parameters) are acknowledged at once and none is lost. Each takes effect exactly at the adoption
sample S, on the successor, with its ramp starting at S. The watermark reports the revision as in
effect from S. The running plan's output is not changed by them, so the catch-up stays exact.

## Context

- During a catch-up the successor is control-owned and leads render in graph time (#1355 D3).
  There is no exact replay of an edit applied to the predecessor at an earlier sample. So D15-17
  holds the edit and applies it at S.
- *Hold live values in latest-target cells on both hosts* (#1312) makes fader, mute and matrix
  lanes into latest-target cells (D1-D3). Validation precedes every write (D6). Effect, input and
  route records stay FIFO (D9).
- *Carry fader, mute and pan ramps across a plan swap* (#1277) D5 keeps a copy-mode successor's
  retarget records for publication. #1355 D5 writes them just before it publishes `ExactlyAt(S)`.
- Today `commit_live` (`crates/capi/src/runtime/control.rs:1065`) pushes to the newest plan's
  providers (`newest_providers`, `:1488`). After *Extract the C ABI control plane into a portable
  crate both hosts call* (#1309) this code lives in `crates/control-plane`.

## Decisions frozen for this slice

- **D1. Where a live edit goes.**
  - While a catch-up is running (from the `CopyAndReturn` publication until adoption or abandon),
    `commit_live` writes neither to the predecessor nor to the successor. It writes into the
    catch-up's **hold**, after the same validation.
  - A pending rebuild that is not catching up keeps today's rule (#1053 D7): the newest
    candidate's cells.
- **D2. The hold.** It is allocated when the warm successor is prepared, from the successor's own
  lane shapes:
  - for every cell lane, one shadow of the cell's words with a written flag (latest target wins,
    as in a cell; supersession counted as #1312 D2 counts it);
  - for every FIFO lane, a list with the successor queue's capacity, minus the records #1277 D5
    already holds for it.

  A FIFO commit that would overflow its list is refused with typed `BACKPRESSURE` before commit.
- **D3. Write at publication.** #1355 D5 writes in this order:
  1. the #1277 D5 retarget records;
  2. then the hold, cells and FIFO lists in their canonical drain order;
  3. then it publishes `ExactlyAt(S)`.

  A returned candidate keeps what was written. Edits committed after that write go into the
  successor's cells or queues directly, because the successor is published and not yet rendering.
- **D4. Abandon.** When a catch-up is abandoned (divergence, supersession, fallback, stop), the
  hold is not lost. Every held edit is already in the committed model, which the next candidate is
  prepared from, so the hold is dropped with the catch-up.
- **D5. Completion.** A held revision completes with S, with the outcome of the catch-up that
  adopts it (#1314 D5).
- **D6. Acked-batch question.** Every fallible check, the FIFO room included, runs before the
  commit. The hold is written only after them. A held edit reaches the successor (D3) or the
  committed model a later candidate is built from (D4). An ack can never precede a drop.

## Deliverables

1. D1-D5 in the control plane and host-core (`crates/host-core/src/catch_up.rs`).
2. Gates in `crates/host-core/tests/warm_successor.rs` and in the control-plane tests.

## Authorized paths

- `crates/control-plane/src/` (the live commit path), `crates/host-core/src/catch_up.rs`,
  `crates/host-core/src/live_delta.rs` (hold allocation only)
- `crates/host-core/tests/warm_successor.rs`, `crates/control-plane/tests/` or its unit tests

## Non-goals

- Structural edits during the window (#1357), and the C ABI service wiring (#1360).

## Objective gates

1. **Fader during the catch-up.** In #1355's gate 1 setup, commit a ramped fader change on an
   unchanged track after the copy at B and before publication.
   - The predecessor's blocks before S are bit-identical to a run with no edit.
   - From S, the output equals a run where the same record is pushed to the successor just before
     its first block at S.
2. **Effect parameter (FIFO).** The same with an EQ gain record. A burst one record beyond D2's
   capacity is refused with `BACKPRESSURE`, and the model and revision are unchanged.
3. **Latest wins.** Two fader commits during the window apply as the second alone at S.
   `live_values_superseded` rises by 1.
4. **Watermark.** The held revision's watermark advance reports `first_sample == S` and `EXACT`.
5. **Retarget order.** A successor whose join retargets a carried strip, with a held mute on the
   same strip, ends at the held value (the hold is written after the retargets).
6. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: an edit pushed to the predecessor during the window changes its blocks before S. Red.
- Gate 2: a FIFO list that grows, or drops, past capacity passes a burst the queue cannot hold.
  Red.
- Gate 3: a hold kept as a FIFO for cells applies the first value at S. Red.
- Gate 4: completion reported at publication, not at S, gives the wrong sample. Red.
- Gate 5: the hold written before the retargets lets the join's value win over the user's. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Carry fader, mute and pan ramps across a plan swap* (#1277), D5.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
