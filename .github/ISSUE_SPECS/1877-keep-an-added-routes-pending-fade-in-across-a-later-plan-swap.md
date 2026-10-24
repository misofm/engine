# Keep an added route's pending fade-in across a later plan swap

Filed by root from #1392's split (backlog sprint, 2026-10-24). Blocked by #1363 (route arms and fades).

## Product outcome

#1392 keeps an added *strip's* pending fade-in (Waiting or FireAt) across a later plan swap, and
never fades a strip twice. This issue does the same for an added *route*, once #1363 gives routes
their own arm, delay and fire.

## Smallest closable slice (the route half of #1392's D1, D2 and D6, and its gate 4)

- D1: the plan-state inventory holds route rows (route ID, fade-arm entry) beside the strip rows.
- D2: successor preparation re-arms, through #1363's route entry, every route in the base's route
  rows that the successor keeps, with the successor's own delay and ramp. A fired route leaves the
  inventory, as a fired strip does (#1392's watch).
- D6: the route ramp state that #1284 carries for a route op gains the op's armed flag (#1363 D2),
  restored on import.
- Gate 4 (in `crates/host-core/tests/successor_swap.rs`): #1363 gate 1's added route A -> B with a
  compensation line survives a second swap while waiting and while FireAt, bit-identical to the
  one-swap reference, and a fired route fade does not repeat.

## Paths

`crates/graph/src/lib.rs`, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
the route op files #1363 touches, this spec.
