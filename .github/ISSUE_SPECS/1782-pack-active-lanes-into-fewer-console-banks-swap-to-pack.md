# Pack active lanes into fewer console banks (swap-to-pack)

Split from #1107 (D5 item 3, 2026-10-23). The design sketch is #1107's D5 item 3; constraints are #1107's D4 items 1-6.

## Problem

#1107 skips a console bank only when every lane of it is silent and at rest. A session where silence is spread across banks (the sparse row: odd tracks silent) skips nothing: every bank keeps one active lane. Packing the active lanes into as few banks as they fill lets the rest skip.

## Smallest closable slice

- Pack groups: maximal runs of consecutive peer Bank units (equal slot program, equal collapse arming), with direct gather sources only, no resident pairing, and all or none folded into one master.
- A graph-level lane map (`home[strip] -> (unit, lane)`, inverse, per-strip `automated` bit). A move exchanges the two chains' lanes and, in the same call, the member ops of every slot, `fold[lane]` and the `source_lanes` bits; response-snapshot bindings, the predecessor side of a plan carry and `unit_eligibility` translate through the map.
- Folded groups: the epilogue fold is deferred to one group-level fold after the group's last unit, in home order, so the master's summation order is exactly the order `route_fold` proves; skipped strips use #1107's signed-zero rule. Packing must never move a master bit.
- Per-block packing decision after the drains, with a hysteresis of `R` idle blocks and a budget of `B = W` moves per group per block (preallocated constants, recorded with their reasons); automated strips move only into fader banks with automation slots (a bind-time bit).

## Authorized paths

To be confirmed in this issue's own design step before code: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/graph-compiler/`, `crates/rack/` (tests), the console benchmark files. This spec.

## Objective gates

1. Class A differential on compiled plans with random per-lane silence runs, packing on and off: every strip and the master equal by `to_bits`; Simd8 and Simd4 (the arm64 CI legs).
2. One plan-level swap test (#1107 D4-5).
3. Allocation-free.
4. One native and one V8 console run after #1783's rows exist: the sparse row against S4; the all-active row within 2 %; the toggle row's cost recorded.

## Test value

Gate 1 turns red when a pack moves a strip's or the master's bits; gate 2 when a plan-level swap leaves a lane-keyed reader behind.

## Dependencies

#1107 and #1781 merged; #1783's rows before gate 4's timing.
