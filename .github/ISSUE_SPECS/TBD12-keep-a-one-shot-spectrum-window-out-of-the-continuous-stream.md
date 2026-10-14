# Keep a one-shot spectrum window out of the continuous stream

Successor of #1609, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 8 integration branch `codex/controls-batch-8`. Issue number:
assigned at filing (written here as TBD12). Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review.

## Product outcome

A continuous spectrum stream returns only windows of its own stream epoch. A one-shot window that
render completes while control switches the capture to continuous mode is never read as a
continuous window.

## Context

- `SpectrumCapture::commit_continuous` (`crates/host-core/src/spectrum.rs`, after #1609) drains the
  result queue, parks the one-shot word idle, stores the new epoch and the phase, and stores
  `CONTINUOUS_MODE` last.
- Render can publish a one-shot record (its `stream_epoch` is `0`) after that drain and before
  the mode store, if the window was in flight. `try_read_continuous_record` pops records with no
  epoch check except the invalidation rule, so it returns that record as a continuous window.
- #1609 found this and left it (its D7: continuous mode is not in that slice).
- Continuous epochs start at `1`, so a one-shot record is always distinguishable by epoch.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A (reader).** `try_read_continuous_record` discards a record whose `stream_epoch` is
  not the current `shared.epoch` and continues the bounded pop loop. No render change.
- **Option B (render).** The one-shot completion in `block_end` publishes nothing while the mode
  word reads continuous. This adds a load to render and needs a loom case.
- Recommendation: A. It keeps render unchanged and also covers any older-epoch record left by a
  restart.

## Objective gates

1. A test drives the race deterministically (a one-shot record pushed between the drain and the
   mode store, in the #1609 loom model or a unit test that splits the switch at that point) and shows the continuous read never returns
   it; the read answers `Warming` or the first window of the new epoch.
2. The #1609 loom model gains one mode-switch case, or the decision record states why option A
   needs none.
3. `cargo test --locked -p host-core --all-features`, the host-core loom step, and the host-web
   spectrum tests stay green.

## Test value

- Gate 1 turns red if a record of another epoch reaches a continuous reader.

## Authorized paths

- `crates/host-core/src/spectrum.rs` and its tests; this spec's decision record.

## Dependencies

- #1609. Land before #1593, #1595 and #1596 read continuous windows on new paths.

## Size

Half a day, one sonnet-high implementer.
