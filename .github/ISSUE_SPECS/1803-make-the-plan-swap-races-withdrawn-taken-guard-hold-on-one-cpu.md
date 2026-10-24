# Make the plan-swap race's withdrawn/taken guard hold on one CPU

Split from #1405 (root decision, 2026-10-24).

## Problem

The back-to-back caller of the C ABI plan-swap race (`race_plan_swaps(64, 0, true)`, `crates/capi/tests/plan_swap_race.rs`) asserts #1310's `withdrawn > 0 && taken > 0`. On one CPU this guard can be vacuous. A `Taken` outcome needs render to start a block while the control thread is inside a command (`withdraw_pending_candidate`, `crates/control-plane/src/control.rs`). Neither a longer race nor a sleeping control thread after the last admission makes that interleaving happen. #1405's attempts measured it under `taskset` with a busy loop: attempt 1 had 9 such failures in 150 runs; attempt 2 had 1 in 50, with "71 withdrawn, 0 taken" over 128 edits. Raw output: #1405's evidence record.

## Smallest closable slice

Make the guard hold on one CPU without tuning a sleep to chase a pass. For example, force the interleaving inside the command phase with a deterministic hook the test controls, or move the claim to a lower-level test where the interleaving can be driven directly. The scheduler-free form is preferred.

## Gates

1. 50 runs pinned to one CPU with a busy loop beside them: 0 failures of this guard.
2. A planted defect that never yields `Taken` turns it red.

