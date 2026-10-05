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
  lanes into latest-target cells (D1-D3). Validation precedes every write (D6). Its D9 leaves
  effect, input and route records FIFO; *Hold effect parameter, bypass and EQ-target values in
  latest-target cells* (#1345), *Hold strip input-lane values in latest-target cells* (#1346) and
  *Hold route-lane values in latest-target cells* (#1347) make those lanes cells too. After them
  every live value is a latest-target cell, so no live value is ever refused for room (D15-2).
  Observation subscriptions stay a FIFO (#1345 D2); they are not live values.
- *Carry fader, mute and pan ramps across a plan swap* (#1277) D5 keeps a copy-mode successor's
  retarget records for publication. #1355 D5 writes them just before it publishes `ExactlyAt(S)`.
- Today `commit_live` (`crates/capi/src/runtime/control.rs:1065`) pushes to the newest plan's
  providers (`newest_providers`, `:1488`). After *Extract the C ABI control plane into a portable
  crate both hosts call* (#1309) this code lives in `crates/control-plane`.

## Decisions frozen for this slice

- **D1. Where a live edit goes.**
  - While a catch-up is running (from the `CopyAndReturn` publication until adoption or abandon),
    `commit_live` never writes to the predecessor.
  - Until the catch-up's first `ExactlyAt` publication (D3), it writes into the catch-up's
    **hold**, after the same validation. After that publication it writes into the successor's
    cells directly, wherever the successor is (published, `Returned`, or taken back by the
    catch-up); those writes too wait for adoption (#1355 D4).
  - **Revision.** Every revision committed while the catch-up runs, live or model-only, belongs to
    the successor, never to the running plan (*Publish an applied-revision watermark and complete
    edits asynchronously*, #1314 D2). Before the first publication it raises the hold's
    `revision`. After it, it goes to the successor's word by #1314 D2's routing: its cell's word
    while it is published or `Returned` (`PlanPublisher::set_revision`, #1311 D6), the word
    `CatchUp::set_revision` keeps while the catch-up holds it. The `Active` cell's word is never
    written while a catch-up is pending.
  - A pending rebuild that is not catching up keeps today's rule (#1053 D7): the newest
    candidate's cells.
- **D2. The hold.** It is allocated when the warm successor is prepared, from the successor's own
  lane shapes: for every live cell (#1312, #1345, #1346, #1347), one shadow of the cell's words
  with a written flag. Latest target wins, as in a cell, and supersession is counted as #1312 D2
  counts it. A hold has fixed size and is never full, so no live edit is refused with
  `BACKPRESSURE` during a catch-up (D15-2). Observation subscriptions are not held; they keep
  #1345 D2's rule against the newest plan.
- **D3. Write before every publication.** Before each publication of the warm successor, the
  exact one (#1355 D5) and the fallback pre-roll (#1358 D4) alike, the control plane writes in this
  order:
  1. the #1277 D5 retarget records;
  2. then the hold's written cells, in their canonical order;
  3. then the hold's `revision` into the successor's revision word, if it is higher than the word
     (#1314 D2's control-held candidate rule);
  4. then the epoch's outcome word (#1355 D8), then it publishes.

  A returned candidate keeps what was written, undrained until adoption (#1355 D4). Edits committed
  after that write go into the successor's cells directly; they too wait for adoption.
- **D4. Dropping the hold.** The hold is dropped only on a path that prepares the committed model
  again: divergence (#1355 D2), supersession (#1357), the transition (*Duck-swap the strips a latency
  growth restarts, and fall back to the transition when no catch-up can finish*, #1397) and a stop (#1359). #1358 gates the pre-roll case. Every held edit is in the committed model that candidate is prepared
  from, and the carry's retargets (#1277 D5) ramp a carried strip to it. On every path that adopts
  this successor instead (exact adoption, a pre-roll), the hold has been written into it (D3)
  before publication, so nothing is dropped.
- **D5. Completion.** A held revision completes with S, with the outcome of the catch-up that
  adopts it (#1314 D5). Because its revision is only ever in the successor's word (D1, D3), the
  watermark cannot report it on a block of the running plan before S.
- **D6. Acked-batch question.** Every fallible check runs before the commit, and the hold cannot be
  full. A held edit reaches the successor before any publication that may adopt it (D3), or the
  committed model of a re-prepared candidate (D4). An ack can never precede a drop.

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
2. **Effect parameter, never refused.** The same with an EQ gain. A burst of 10,000 EQ gain commits
   during the window all return OK. At S the last one applies, and `live_values_superseded` rises
   by 9,999.
3. **Latest wins.** Two fader commits during the window apply as the second alone at S.
   `live_values_superseded` rises by 1.
4. **Watermark.** The held revision's watermark advance reports `first_sample == S` and `EXACT`.
5. **No early watermark.** In gate 1's setup, commit a live fader edit (revision `r`) and then a
   model-only edit (`r + 1`) during the catch-up, before the first publication, and a live mute
   edit (`r + 2`) after it, then force one return (render skips `S`; #1355 gate 3). Render the
   predecessor until the adoption at `S'`: on every block before `S'` the watermark stays at the
   revision it showed before the catch-up began, and the adoption block reports
   `(r + 2, S', EXACT)`.
6. **Retarget order.** A successor whose join retargets a carried strip, with a held mute on the
   same strip, ends at the held value (the hold is written after the retargets).
7. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: an edit pushed to the predecessor during the window changes its blocks before S. Red.
- Gate 2: a hold kept as a bounded list refuses part of the burst with `BACKPRESSURE`. Red.
- Gate 3: a hold kept as a FIFO for cells applies the first value at S. Red.
- Gate 4: completion reported at publication, not at S, gives the wrong sample. Red.
- Gate 5: a revision written to the `Active` cell whenever no candidate sits in a cell (the
  successor is control-held before publication and after a return) reports `r` at the next block
  of the predecessor, before `S'`. Red.
- Gate 6: the hold written before the retargets lets the join's value win over the user's. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345).
- *Hold strip input-lane values in latest-target cells* (#1346).
- *Hold route-lane values in latest-target cells* (#1347).
- *Carry fader, mute and pan ramps across a plan swap* (#1277), D5.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311), D6:
  the revision of a `Returned` cell.
