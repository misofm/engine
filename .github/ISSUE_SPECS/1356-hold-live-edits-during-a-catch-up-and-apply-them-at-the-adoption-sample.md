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
  lanes into latest-target cells (D1-D3). Validation precedes every write (D7). Its D10 leaves
  effect, input and route records FIFO; *Hold effect parameter, bypass and EQ-target values in
  latest-target cells* (#1345), *Hold strip input-lane values in latest-target cells* (#1346) and
  *Hold route-lane values in latest-target cells* (#1347) make those lanes cells too. After them
  every live value is a latest-target cell, so no live value is ever refused for room (D15-2).
  Observation subscriptions stay a FIFO (#1345 D2); they are not live values.
- *Carry fader, mute and pan ramps across a plan swap* (#1277) D5 keeps a copy-mode successor's
  retarget records for publication; *Carry live-controlled effect lanes across a plan swap* (#1280)
  D4 does the same for effect lanes. They come from the prepared model, so they are older than any
  live edit committed after the successor's commit.
- #1312 D1's cell writer is control-owned and keeps the next sequence; render, the only writer of
  `live_values_superseded`, counts `s - p - 1` when it reads (#1312 D2). #1355 D4 keeps a warm
  successor's cells undrained until adoption.
- Today `commit_live` (`crates/control-plane/src/control.rs:1201`) pushes to the newest plan's
  providers (`newest_providers`, `:1628`). After *Extract the C ABI control plane into a portable
  crate both hosts call* (#1309) this code lives in `crates/control-plane`.

## Decisions frozen for this slice

- **D1. Where a live edit goes: the warm candidate's own cells.**
  - From a warm candidate's commit (the warm submit's, or the commit of a newer warm candidate
    that *Supersede a running catch-up by a structural edit* (#1357) holds unpublished) until its
    adoption or abandon, `commit_live` writes the candidate's cells, never the predecessor's,
    after the same validation (#1312 D7). This holds wherever the candidate is: control-held,
    published, `Returned`, or with render copying into it.
  - Those cells are the hold. #1355 D4 keeps them undrained until the adoption block, so each
    held edit applies at S, and latest target wins as in any cell. There is no shadow copy, no
    flush and no second writer.
  - **Revision.** Every revision committed while the catch-up runs, live or model-only, belongs to
    the successor, never to the running plan (*Publish an applied-revision watermark and complete
    edits asynchronously*, #1314 D2). It goes to the successor's word by #1314 D2's routing: its
    cell's word while it is published or `Returned` (`PlanPublisher::set_revision`, #1311 D6), and
    while control holds it, the word `CatchUp::set_revision` keeps (#1355 D5) or, for #1357's
    held newer candidate, the word #1357 D1a writes. The `Active` cell's word is never written
    while a catch-up is pending.
  - A pending rebuild that is not catching up keeps today's rule (#1053 D7): the newest
    candidate's cells. Its retargets were written at preparation (#1277 D5), so later edits
    replace them as usual.
- **D2. Supersession, one writer.** The control thread stays the cells' only writer and render the
  counter's only writer. Each held edit is an ordinary cell write, so it takes the next sequence,
  and render counts every value replaced unread when it reads the cell at S (#1312 D2). Nothing is
  counted on the control thread. The hold has fixed size and is never full, so no live edit is
  refused with `BACKPRESSURE` during a catch-up (D15-2). Observation subscriptions are not held;
  they keep #1345 D2's rule against the newest plan.
- **D3. Publication writes, retargets exactly once.** #1312 D1's cell writer gains
  `has_written(&self) -> bool`: true once any write through it has completed (a read of its
  private sequence, no atomic). Before each publication of a warm successor that may adopt it,
  the exact one (#1355 D5) and the pre-roll (#1358 D4) alike, the control plane:
  1. at the first such publication only, writes the kept retargets (#1277 D5, #1280 D4) into
     every cell whose writer has not written. A cell a live edit already wrote holds a newer
     committed value, so its retarget is skipped, never written over it. Later publications write
     no retarget;
  2. then raises the successor's revision word to the highest revision routed to it (D1), if it is
     higher (#1314 D2's control-held candidate rule);
  3. then writes the epoch's outcome word (#1355 D8), then publishes.

  A returned candidate keeps what was written, undrained until adoption (#1355 D4). An edit
  committed after the first publication is written to the cells directly and is never replaced by
  a retarget.
- **D4. Dropping the hold.** The held edits are dropped only with the successor, on a path that
  prepares the committed model again: divergence (#1355 D2), supersession (#1357), the transition
  (*Duck-swap the strips a latency growth restarts, and fall back to the transition when no
  catch-up can finish*, #1397) and a stop (#1359). Every held edit is in the committed model that
  candidate is prepared from, and the carry's retargets (#1277 D5) ramp a carried strip to it. On
  every path that adopts this successor instead (exact adoption, a pre-roll), the held edits are
  already in its cells, so nothing is dropped.
- **D5. Completion.** A held revision completes with S, with the outcome of the catch-up that
  adopts it (#1314 D5). Because its revision is only ever in the successor's word (D1, D3), the
  watermark cannot report it on a block of the running plan before S.
- **D6. Acked-batch question.** Every fallible check runs before the commit, and a cell write
  cannot fail. A held edit is in the successor's cells before the commit returns, and it is never
  overwritten by an older value (D3), or it is in the committed model of a re-prepared candidate
  (D4). An ack can never precede a drop.

## Deliverables

1. D1-D5 in the control plane and host-core (`crates/host-core/src/catch_up.rs`), and
   `has_written` (D3) in `crates/engine/src/realtime/latest_cell.rs`.
2. Gates in `crates/host-core/tests/warm_successor.rs` and in the control-plane tests.

## Authorized paths

- `crates/control-plane/src/` (the live commit path), `crates/host-core/src/catch_up.rs`
- `crates/engine/src/realtime/latest_cell.rs` (the `has_written` accessor only)
- `crates/host-core/tests/warm_successor.rs`, `crates/control-plane/tests/` or its unit tests

## Non-goals

- Structural edits during the window (#1357), and the C ABI service wiring (#1360).

## Objective gates

1. **Fader during the catch-up.** In #1355's gate 1 setup, commit a ramped fader change on an
   unchanged track after the copy at B and before publication.
   - The predecessor's blocks before S are bit-identical to a run with no edit.
   - From S, the output equals a run where the same record is pushed to the successor just before
     its first block at S.
2. **Effect parameter, never refused.** The same with the left-channel gain of one EQ band (a
   `Left` target: one cell, #1345 D1). A burst of 10,000 such commits during the window all
   return OK. At S the last one applies, and `live_values_superseded` rises by exactly 9,999,
   counted by render at S (a `Both` record would write two cells and count 19,998).
3. **Latest wins.** Two fader commits during the window apply as the second alone at S.
   `live_values_superseded` rises by 1.
4. **Watermark.** The held revision's watermark advance reports `first_sample == S` and `EXACT`.
5. **No early watermark.** In gate 1's setup, commit a live fader edit (revision `r`) and then a
   model-only edit (`r + 1`) during the catch-up, before the first publication, and a live mute
   edit (`r + 2`) after it, then force one return (render skips `S`; #1355 gate 3). Render the
   predecessor until the adoption at `S'`: on every block before `S'` the watermark stays at the
   revision it showed before the catch-up began, and the adoption block reports
   `(r + 2, S', EXACT)`.
6. **A retarget never overwrites a user value.** A successor whose join retargets strip X's fader
   and mute. (a) Before the first publication, commit a live fader edit on X: from S, X's fader is
   the user's value and its mute the retarget's. (b) After the first publication, commit a live
   fader edit on X, force one return, and let the catch-up publish again at `S'`: from `S'`, X's
   fader is the user's value. In (a) `live_values_superseded` does not move (the retarget was
   skipped, never written); in (b) it rises by 1 (the retarget written at the first publication
   and replaced unread).
7. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: an edit pushed to the predecessor during the window changes its blocks before S. Red.
- Gate 2: a hold kept as a bounded list refuses part of the burst with `BACKPRESSURE`; a hold
  that counts on the control thread races render's counter or misses the replaced values. Red.
- Gate 3: a hold kept outside the cells as a queue of records applies the first value at S. Red.
- Gate 4: completion reported at publication, not at S, gives the wrong sample. Red.
- Gate 5: a revision written to the `Active` cell whenever no candidate sits in a cell (the
  successor is control-held before publication and after a return) reports `r` at the next block
  of the predecessor, before `S'`. Red.
- Gate 6: retargets written at every publication, or written over a cell a live edit wrote,
  replace the user's newer fader value with the join's older one at S or `S'`. Red.

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
