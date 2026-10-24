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
