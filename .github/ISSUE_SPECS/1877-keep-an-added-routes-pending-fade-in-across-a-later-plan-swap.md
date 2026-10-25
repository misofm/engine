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

## Decision record (attempt 1, 2026-10-25)

- Scope note `scope/1877.md` and root decisions bind: the successor's re-arm wins over the carried
  arm; #1363 D3a's fire at the swap stays the fallback for a route with no successor arm; the
  inventory row is kinded (`FadeArmInventoryRow.owner: FadeArmOwner::{Strip, Route}`), not a
  parallel table.
- D1, D2 (`prepare.rs`): every route arm takes a row; `added_route_fades` re-arms a route of an
  unfired route row that the successor's model holds with the same source, tap and destination,
  carrying its entry through `install_fade_arm_carry`. A re-pointed, muted or following route is
  not re-armed.
- D6 (`runtime.rs`): `carry_route` fires the displaced arm only when the successor holds no arm;
  `LiveRoute::adopt` leaves the successor's own arm in place.
- Found in implementation: a successor prepared before the predecessor's fire and adopted after it
  re-arms a route whose pair then copies `Done`. Left alone, the route would wait for a fire its table
  no longer holds (a transition route stays silent; a laned route swallows every later live
  record). `carry_fade_arms` (`crates/graph/src/lib.rs`, a code change, not only a comment) therefore
  fires a route entry that takes `Done`, and a re-armed transition route with unchanged values joins
  the send carry (`RouteJoin { retarget: None }`), so its op takes the displaced op's mid-ramp state.
- Scope note's defect 2 as worded (a lane-less output route with a source-edge line, displaced before
  its fire) cannot occur: a source-edge line comes only from a raised floor on the route node, and an
  added route's node has none, so a lane-less added route has `D = 0` and fires at the first block.
  Gate 4 (d) tests the reachable form instead (prepared before the fire, adopted mid-ramp).
- Gate 4's allocation claim is asserted inside the new test (`fade_run`'s allocator counters over
  both hand-overs and every later block, as #1392 did), not by extending
  `the_swap_block_allocates_and_frees_nothing`.

## Decision record (attempt 2, 2026-10-25)

- Verifier MAJOR fixed: in the `armed_restarts` path a route already in `route_fades` now takes
  `carried = None`, so a restarted strip's send is armed only afresh with its fader (#1392 D2).
  Gate 4 (e) checks a restart at block 8, the successor prepared then or a block earlier, against
  the run whose second transaction adds the send. A restart after the fire is not observable in the
  12-block harness: the restarted limiter line holds the strip silent past the run's end.
- The injected `RouteJoin { retarget: None }` is limited to lane-less routes
  (`!controls::route_rows_live`).
- A lane-less transition route that has fired and is in flight across a later swap (the successor
  binds no op for it) existed before this issue; root moves it to #1953b's scope.
- Stated limit (root, attempt 2 PASS): row (e) covers only a restart before the fire; the clear is one unconditional line, so the order after the fire holds by construction; a later change that carries only `Done` entries would stay green.
