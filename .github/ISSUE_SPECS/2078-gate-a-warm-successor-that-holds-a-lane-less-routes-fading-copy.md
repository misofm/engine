# Gate a warm successor that holds a lane-less route's fading copy

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #1953 (1953b),
MINOR-3. The cited closed spec is read
at `da15a2824` (it has left this folder).

## Problem
A warm successor (#1354) compiles its floors from the lead's floor map (`warm_floors`). 1953b's
fading copies then take their alias's floor from that map. No test prepares a warm successor that
also holds a lane-less route copy, so nobody knows whether it prepares and renders correctly or
fails on an internal invariant (most likely `host.successor.warm_invariant`).

## Evidence
- `crates/host-core/src/prepare.rs:4070-4099`: a warm successor's floors come from `warm_floors`;
  each copy then takes its alias's floor (`floors.get(&alias)`).
- `crates/host-core/src/prepare.rs:2069-2089` (`warm_floors`): `a(n) + lead` on a predecessor node
  outside `R`, `a(n)` in `R`, none on an added node.
- `crates/host-core/src/prepare.rs:3741-3744`: copies are made for any successor with live lanes
  and no declared discontinuity, warm or not.
- `crates/host-core/tests/warm_successor.rs`: no row names a route copy (no match for "copy").
- 1953b spec, "Stated limits": "Warm successor (#1354, MINOR-3): copies compile with the alias's
  floor from the lead's floor map, untested."

## Smallest slice
Add one gate: a warm successor that also removes a lane-less route. If it prepares, it must equal
the live-mute reference from its adoption. If it refuses, the refusal must be typed and named, and
the slice decides whether that refusal is the rule (record the reason) or a defect to fix in this
issue (the fix is the copy's floor under the lead).

## Gates
- One `warm_successor` row as above, on every width the build binds.
- If code changes: no rendered bit moves for a warm successor without a route copy (one PR
  comparison, not a committed test).
- `cargo test -p host-core` passes.

## Paths
`crates/host-core/tests/warm_successor.rs`, `crates/host-core/src/prepare.rs` (only if the gate is
red), this spec.

## Decision record (attempt 1, 2026-10-25)
- Outcome: **green**. The row `a_warm_successor_fades_a_removed_lane_less_route_out_through_its_copy`
  (`crates/host-core/tests/warm_successor.rs`) passes on both bank widths with no change to
  `prepare.rs`: W prepares (no `warm_invariant`), leads by `P = 512`, restarts nothing, floors the
  copy `t2-main-` at its alias's warm floor `a(old) + P`, and its block `j` equals block `j + k` of
  the fresh W' reference (copy as a `pre_fader` route, live-muted over the session mute ramp before
  block `BEFORE + k`). R1's fix was not needed; no refusal path.
- Shape change from the scope note, two points, both forced by the reference, not by the code:
  - The growth is gate 1's muted limiter track `t3`, not `limiter_on_t1`. A limiter on the audible
    `t1` restarts `t1` (`restart_whole` is empty, but `restarted_strips` holds `t1`), and a
    restarted audible strip starts at rest, so W never equals a fresh W' there, with or without a
    copy (probe: all 10 blocks after the swap differ with no route removed).
  - The removed route is `t2-main` from `post_fader` (lane-less into the output too, #1953b's own
    shape), not `post_pan`: the `pre_fader` twin moves no bit only across a unity fader; across the
    strip's pan it does (probe: red at every block).
