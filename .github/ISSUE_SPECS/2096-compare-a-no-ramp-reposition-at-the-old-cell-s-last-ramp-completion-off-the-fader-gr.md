# Compare a no-ramp reposition at the old cell's last ramp completion off the fader grid

Filed by root from the #1716 fix D verification (2026-10-25).

## Problem
#1716 fix D compares a no-ramp `Reposition`'s new curve with the old curve at `span.start - 1`. That is exact when the block starts on the 64-sample fader grid, because the last grid ramp has just completed there. Off the grid, the stage's stored target is the old curve at the last grid ramp's completion `g + 63` (with `g + 63 >= span.start`), not at `span.start - 1`. So a table-only rebuild with no move, adopted at an off-grid block, still opens with a J-length adoption jump on a moving ride. This happens in production: the C ABI accepts any quantum (96, 100, 240, 480 ...), and seeks land on any frame. It is not a regression of fix D: the code before D jumped there too.

## Smallest slice
Compare at the old cell's last ramp completion (`ramp_done`, which `reposition` clears today) for a stage with no ramp, held or free. Keep a step of the new table at `span.start` jumping (B17-R13).

## Gates
- The builtins-compiler composed-lane row at an off-grid block (for example 100 frames after a seek) and a no-move table-only rebuild: bit-equal to the uninterrupted engine. Red on main.
- Class B (bits move only in that case); fix D's owner approval of 2026-10-25 covers the same rule ("compare at the same node time"). State that in the PR.
