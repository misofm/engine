# Settle a one-shot spectrum capture across a plan swap

Successor of #1609 (found by the batch 8 review), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on the batch 8
integration branch `codex/controls-batch-8`. Issue number: assigned at filing (written here as
TBD14). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.
The defect predates batch 8.

## Product outcome

A one-shot spectrum capture that is in flight or held when the plan swaps ends in a defined state:
the caller reads its window, or reads a typed refusal and can arm again. It never waits forever.

## Context

- After #1609, render moves a one-shot capture `CAPTURING -> COMPLETE` by a compare-and-swap at
  block end, and holds a completed window when a stale record fills the queue
  (`crates/host-core/src/spectrum.rs`, the one-shot state word and `one_shot_block_end`).
- At a plan swap the successor plan's spectrum observer starts from its own prepared state. A
  window the predecessor held is lost, and the shared state word can stay `CAPTURING` with no
  observer that will complete it. The caller's read then answers pending until it cancels.
- The carry chain (#1280 and later) carries effect lanes across a swap, not the spectrum capture.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A (settle).** At adoption of the successor plan, off the render thread, the control side
  moves a predecessor capture that is `CAPTURING` or held to `INVALID` with its generation, so the
  reader gets the existing `Invalid` answer and can arm again.
- **Option B (carry).** The successor's observer adopts the predecessor's armed capture and its
  held window. This needs a carry slot and a loom case.
- Recommendation: A. A plan swap is a structural edit; a refused window is honest and cheap.

## Objective gates

1. A host-core test arms a one-shot capture, swaps the plan before the window completes, and
   shows the read answers `Invalid` (option A) and that a new arm completes on the successor.
2. The same with a window held behind a stale record at the swap.
3. The #1609 loom model and race test stay green; render stays allocation-, lock- and
   syscall-free.

## Test value

- Gates 1 and 2 turn red if a capture can stay pending forever across a swap.

## Authorized paths

- `crates/host-core/src/spectrum.rs`, `crates/host-core/src/spectrum_service.rs` and their tests;
  the plan-adoption call site in `crates/control-plane` if the settle needs one; this spec's
  decision record.

## Dependencies

- #1609. Before #1593 (spectrum taps on every plan) makes swaps with taps common.

## Size

Half a day, one sonnet-high implementer.
