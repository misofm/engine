# Publish the revision gate only after the observe records of its transaction are pushed

Filed by root from the #1585a verification (backlog sprint, 2026-10-24).

## Defect

`commit_live` in `crates/control-plane/src/control.rs` publishes the revision gate (`revision_gate.publish_key(key)`, about :4614) before it pushes the transaction's observe records (about :4632). Its own step-8 comment says every record push comes before the publication. With render on another thread, the applied-revision watermark can cover revision r one block before render drains r's observe record, so "covered" does not mean "the observation slot shows it".

## Smallest closable slice

1. Move the observe-record push before the revision publication, so that every record of revision r is in its queue before r can be applied.
2. One test that pins the order: with render on a second thread, a reader that sees the watermark cover r also sees r's arm state in the slot (or a deterministic test of the push/publish order through the controller seam).
3. If the current order is proved harmless instead, record the proof and pin it with the same test.

## Paths

`crates/control-plane/src/control.rs`, `crates/control-plane/tests/` (one test), this spec.

## Decision and evidence record (attempt 1, 2026-10-24)

- **Confirmed on main 962acf086.** `commit_live_once` published the gate (`revision_gate.publish_key(key)`) before `push_admitted_observation_batch`. The other commit paths do not have the defect: a rebuild (a transaction's, a discontinuity's, a monitor rebuild) queues its records on the successor's own queues after it publishes the revision on the successor's gate, but render cannot reach the successor until `reservation.commit(revision)` hands it over, after every push. The host's own `admit_and_push_observation_batch` publishes no revision.
- **Order chosen.** The protocol commit (the last fallible step), then the owner commits, the catalog, the report row and the kept base, then the observation pushes and the binding table, then the gate publication as the very last write. The pushes were already after the commit and cannot fail (admitted whole before the first write, on queues the control thread alone produces into), so moving them changes no refusal: a full queue is still refused before the commit with nothing written, and no acknowledgement can precede a drop. The pushes go on the newest epoch's queues, the same epoch whose gate publishes the revision; the gate is borrowed again after the pushes.
- **Why it holds.** Render takes its `Acquire` snapshot of the gate and then drains every observation queue in the same block (`BankStage::drain`). With the pushes before the `Release` publication, a block whose snapshot covers `r` sees `r`'s records and drains them, so a watermark that covers `r` means the tap's state is `r`'s. The queues are outside the gate, so a record may apply one block before `r` is covered; it is committed and nothing after it can fail, as with any write before the publication.
- **Scope note.** An `Observe`-only transaction publishes nothing (`monitor` path, no revision), so the order matters only when the arm rides in a transaction that publishes (a model edit or a monitor write that produced a record).
- **Test.** `a_live_commit_queues_its_observation_records_before_it_publishes_the_revision` in `crates/control-plane/tests/observation_bindings.rs`: a mute `SetControl` plus an arm commits `r`; a test-support seam (`SessionState::test_take_observe_push_key`, set where a record is queued) shows the gate still held `r - 1` when the arm was queued, then one block shows the watermark at `r` and the tap armed from that block. Red on the old order (mutant moving the publication back above the pushes: `left: 2, right: 1`); the 21 other tests in the file stay green on that mutant.
