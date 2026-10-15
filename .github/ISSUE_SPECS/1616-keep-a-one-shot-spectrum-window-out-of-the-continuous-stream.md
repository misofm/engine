# Keep a one-shot spectrum window out of the continuous stream

Successor of #1609, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 8 integration branch `codex/controls-batch-8`. Issue number:
assigned at filing (written here as #1616). Feature sprint mode (owner, 2026-10-13): one
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

## Decision record: one-shot window out of the continuous stream (batch 8)

**Decision: option A.** `try_read_continuous_record` (`crates/host-core/src/spectrum.rs`) loads
the stream epoch once before its bounded pop loop and discards every popped record whose
`stream_epoch` is not that epoch. Render is unchanged on the publish side. The pop loop stays
inside the existing `REALTIME_POLICY_BEGIN/END` region and keeps its bound (`available`, frozen at
entry and capped at `SPECTRUM_RESULT_SLOTS`); a discard spends one pop of it, so a read that
meets only stale records answers `Warming` or `Pending`, never a refill chase.

**What the check covers.** A one-shot record (epoch 0, below every continuous epoch) that render
pushes after the switch's drain and before its mode store; a record of an epoch a restart
(`restart_continuous`, control) ended while render was publishing; and a record of an epoch a
render failure ended. Only control changes the epoch outside a failure, and a failure moves it
forward before render publishes anything of the new epoch, so one load covers the loop.

**Departure from the text (code is newer than the spec).** #1609 left an invalidation rule in the
same loop: `invalidated`/`invalidated_epoch` discarded one record with an epoch at or below the
failed one. Every failure bumps the epoch, so the new epoch check subsumes that rule and discards
all such records, not one. Keeping both left a dead flag, so the two fields, their stores in
`continuous_fail` and `commit_continuous`, and the guard arm are removed. Render does two fewer
atomic stores on a failure; nothing else on the render side moved.

**Gates.**
1. `a_one_shot_record_that_lands_after_the_mode_switch_is_never_a_continuous_window` arms a
   capture, completes a one-shot window, switches through the collection's replacement path
   (`begin_continuous(.., true)`), pushes the epoch-0 record as render's block end would after the
   drain, and shows the read answers `Warming` and the next window is epoch 1, sequence 0.
   `a_record_of_an_older_epoch_is_discarded_by_the_continuous_read` does the same for an epoch-1
   record after a restart to epoch 2. Both are red with the epoch arm disabled (as are five
   failure tests that formerly relied on the invalidation rule).
2. The loom model gains `spectrum_mode_switch_loom` (same module, so the CI step filter
   `spectrum_handoff_loom` runs it): render completes and publishes an armed one-shot window while
   control replaces the capture with a continuous stream; every interleaving of the push against
   the drain and the mode store must return only windows of epoch 1 (a continuous window of the
   same samples is a correct result when render already saw the new mode). `mode` is a plain
   `std` atomic, but render loads it before its first modelled operation and control stores it
   after its last, so loom still places the push before, between and after the drain; the model is
   red with the epoch arm disabled.
3. `cargo test --locked -p host-core --all-features`, the loom step, `host-web` with
   `test-support`, `realtime-policy`, `check-bench-policy.sh`, clippy `-D warnings` (no features,
   all features) and rustdoc `-D warnings` are green.

No measured numbers: no benchmark was run (feature sprint mode).
