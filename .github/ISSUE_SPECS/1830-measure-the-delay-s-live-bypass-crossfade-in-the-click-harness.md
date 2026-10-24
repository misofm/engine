# Measure the delay's live bypass crossfade in the click harness

Split from #1339 (root decision, 2026-10-24): #1339's gate 8.

## Problem

#1339 makes the delay's session bypass a live shunt with the session's bypass crossfade. Its gate 8 asked for a click measurement of a live delay bypass toggle with the standalone harness under `docs/handoffs/control-smoothing-defaults/measure/`. One run of that harness takes about 14 minutes. #1341 already proved the crossfade law on the effects that had a live shunt before #1339, and the delay has no latency line. So #1339 merged without the delay row of the harness.

## Smallest closable slice

Add the delay's live bypass to the harness's live rows and run it once. Record the measured click margin beside the other effects' rows in `docs/handoffs/control-smoothing-defaults/FINDINGS.md`.

## Gates

1. The delay row's click margin is inside the bound that #1341's rows meet, or the issue records the margin and the reason it differs.

