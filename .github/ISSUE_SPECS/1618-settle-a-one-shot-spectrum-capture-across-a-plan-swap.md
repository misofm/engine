# Settle a one-shot spectrum capture across a plan swap

Successor of #1609 (found by the batch 8 review), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on the batch 8
integration branch `codex/controls-batch-8`. Issue number: assigned at filing (written here as
#1618). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.
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

## Ruling (root, 2026-10-14): rescoped after #1395

A carried spectrum observer keeps its one-shot state across a swap (#1395 gate 3). This issue is
rescoped to the captures that do not carry (no successor observer at the same tap, or a refused
hand-over): such a capture settles to `Invalid` with its generation (option A), and a new arm
completes on the successor. It lands after #1395.

## Decision record: #1618, the settle (batch 8)

**Shipped (option A, as ruled).** `SpectrumService::adopt_successor(successor, pairing, carried,
analysis)` settles what the caller had on a capture that did not carry; a carried capture is
untouched (#1395a gate 3 stays green).

- *One-shot.* `SpectrumCapture::settle_unpaired(&mut self, predecessor)` (`spectrum.rs`): when the
  predecessor's word is in any phase but `IDLE` (armed, capturing, held with or without a queued
  result, complete, invalid) and its mode is one-shot, the successor's fresh capture takes
  `(predecessor generation, INVALID)` by one atomic exchange. The reader's next `read` answers
  `CaptureInvalid` once (the existing `INVALID` arm of `try_read_record`, which also drains a queued
  stale record and parks the word `IDLE`), and a new arm is generation `g + 1`, so no record of the
  old arm can pass for it. The arm stays the caller's until that read (`caller_armed` is kept).
  The successor's observer moves the word only from `ARMED` or `CAPTURING`, so the exchange is safe
  while that plan renders; no render, loom-model or `latest_cell` edit (the #1609 loom model and
  race test are unchanged and green).
- *Stream.* A new private `end_stream_state` (shared with `stop_stream`) runs when the stream did
  not carry: `stream_active` false, cadence, smoothing and window gone, metadata `Stopped`, and the
  caller's `analysis` ends with it. This is #1395b's note: after a lost selection `select_stream`
  now answers `StreamInactive` instead of arming a one-shot, and the host selects and starts again.
- *Collection.* When the selected entry did not pair, the successor has no selection (#1395b), so
  the caller's arm is released (`caller_armed` false) and a read answers `NotArmed`; the selection
  epoch tells the caller to select again. A single capture and a collection never pair, so a kind
  change also drops the arm and the stream.
- *Departures.* (1) `adopt_successor` takes `analysis: Option<&mut SpectrumAnalysis>` (the owner of
  the stream's history ends with the stream); its only caller is the test harness, updated. (2) A
  lost collection selection reads `NotArmed`, not `CaptureInvalid`: no capture exists to hold a
  word. (3) The gate sentence "a later `select_stream` restarts the cadence" is met as
  `select` + `start_stream` restarting it: with the stream ended, `select_stream` is a typed
  `StreamInactive` (it can only restart a running stream).

**Gates -> tests.** `crates/host-core/tests/successor_swap.rs`, `mod spectrum_carry`: gate 1
`a_one_shot_that_does_not_carry_settles_invalid_and_a_new_arm_completes` (changed channel mask: one
`CaptureInvalid` at the promotion, none before or after; the new arm completes at a later block,
window equal to a fresh arm on the same plan bit for bit) and
`a_one_shot_after_a_refused_hand_over_settles_invalid`; no false settle
`an_idle_capture_that_does_not_carry_settles_nothing`; stream
`a_lost_selection_ends_the_stream_and_a_new_start_restarts_the_cadence`,
`a_refused_hand_over_installs_the_fresh_capture` (now also `!stream_active`),
`a_one_shot_on_a_lost_selection_is_released`. Gate 2 `spectrum::tests::
a_window_held_behind_a_stale_record_settles_invalid_when_the_plan_swaps` (held window behind a
stale record, through the service: `CaptureInvalid`, then `NotArmed`, then a new arm completes with
the new observer's window). Gate 3: `spectrum_handoff_loom`, the #1609 race test and gate 4 of
#1395 (`..._allocate_and_free_nothing`, zero calls in `adopt_successor`) stay green. With the
settle disabled the gate 1 tests and the gate 2 test go red.
