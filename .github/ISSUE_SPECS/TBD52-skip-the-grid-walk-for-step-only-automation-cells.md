# Skip the grid walk for step-only automation cells

Placeholder from batch 18 (#1651, #1652; weekly performance pass, phase 2). Root files it and gives it
a number. Stateless: everything needed is below.

## Problem

A stored mute curve is `step` only, values 0 and 1 (#1645 D1), so its cell is constant between its
discontinuities. `controls::MUTE_CELL_RULE` (`crates/controls/src/automation.rs`) still has
`grid_ramp: 64`, so each render block walks the cell's 64-sample grid and finds no event. A following
route (#1652) walks its source's mute cells the same way in its own node time, once per route per
block. The work is bounded and allocation-free, but it is spent on a cell that cannot move between
its jumps.

## Decisions to make at filing

- D1. A step-only cell (every segment `step`) skips the grid walk: its events are its jumps and seek
  landings only. The rule is chosen at preparation from the cell's segments, not by the row, so a
  future row with step-only curves takes it too.
- D2. No rendered bit moves: the fader and mute gates of #1647, #1651 and #1652 stay green, and a
  one-time digest of a session with mute automation and following sends is equal at base and head
  (PR evidence).
- D3. Measure first in the weekly pass (the console record's cycles for a session with 64 automated
  mutes and their following sends). No change unless the measured cost warrants it.

## Dependencies

- #1651, #1652 (shipped in batch 18).
