# Make the plan-swap race's withdrawn/taken guard hold on one CPU

Split from #1405 (root decision, 2026-10-24).

## Problem

The back-to-back caller of the C ABI plan-swap race (`race_plan_swaps(64, 0, true)`, `crates/capi/tests/plan_swap_race.rs`) asserts #1310's `withdrawn > 0 && taken > 0`. On one CPU this guard can be vacuous. A `Taken` outcome needs render to start a block while the control thread is inside a command (`withdraw_pending_candidate`, `crates/control-plane/src/control.rs`). Neither a longer race nor a sleeping control thread after the last admission makes that interleaving happen. #1405's attempts measured it under `taskset` with a busy loop: attempt 1 had 9 such failures in 150 runs; attempt 2 had 1 in 50, with "71 withdrawn, 0 taken" over 128 edits. Raw output: #1405's evidence record.

## Smallest closable slice

Make the guard hold on one CPU without tuning a sleep to chase a pass. For example, force the interleaving inside the command phase with a deterministic hook the test controls, or move the claim to a lower-level test where the interleaving can be driven directly. The scheduler-free form is preferred.

## Gates

1. 50 runs pinned to one CPU with a busy loop beside them: 0 failures of this guard.
2. A planted defect that never yields `Taken` turns it red.


## Decision record

- **Root R1 (2026-10-24, accepted): the slice's second form.** The back-to-back guard in `race_plan_swaps` (`crates/capi/tests/plan_swap_race.rs`) loses its `taken > 0` half; `withdrawn > 0` and `withdrawn == superseded` stay, and the run still prints both counts. `Taken` is counted only when render's claim (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs`) lands between the command's service step and its `withdraw()` (`withdraw_pending_candidate`, `crates/control-plane/src/control.rs`), a window of microseconds that on one CPU only a preemption opens. This is a test-design fix for a nondeterministic assertion, not a gate moved after its result: the claim "a taken candidate is found and promoted" stays held deterministically by `a_displaced_plan_not_yet_reclaimed_is_charged_after_a_taken_promotion` (`crates/capi/src/runtime/supersede_tests.rs`, `candidate_taken == 1` through `control_plane::test_hold_reclamation`) and, at the exchange level, `withdrawal_after_render_claimed_reports_taken` (`crates/engine/src/realtime/mod.rs`). The test's doc names both.
- **Weighed and not taken (form A):** a hook that holds reclamation and waits for `rendered` to advance inside the race would make `Taken` deterministic, but it removes the withdraw-versus-claim concurrency from the steps that have it and repeats the scripted interleaving `supersede_tests.rs` already owns (one test per claim).
- **Root R2:** no exact `withdrawn` count; the race does not control whether the previous block rendered before the previous command.

## Evidence (attempt 1)
