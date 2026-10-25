# Arm a pre-fader route that follows its source strip's automated mute across a duck-swap

Filed by root from slice 1324a (backlog sprint, 2026-10-25).

## Defect
At a duck-swap (#1324), a restarted strip's pre-fader routes are armed so they fade in after `S`. A pre-fader
route that follows its source strip's mute (`follows_mute`) while that mute is automated cannot be armed:
arming it needs graph work that 1324a does not have. Such a route restarts at its prepared value at `S`,
which is an audible step.

## Product outcome
A following pre-fader route of a restarted strip fades in after `S` like every other armed route, with its
followed mute applied, and no step.

## Smallest closable slice
The graph support to arm a following route whose source mute is automated, the preparation that arms it at a
duck-swap, and one gate in the 1324a gate-1 shape (bits against the reference runs, zero allocations).

## Paths
`crates/graph/src/{lib,runtime}.rs`, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/`, this spec.

## Decision record (attempt 1)
- Root decisions R1-R5 (scope note) apply: the fire composes each column with the curve heard at the fire
  block; the arm refuses only on the route's own gate (`flags.mute`); records and curve events during the
  wait move transform, flags, cursors and silences only, the ramps stay at `+0.0`; the compensating clause of
  `submix_input_is_quiet` is deleted; no inventory or pair change.
- Graph: `LiveFollowRoute` holds `armed: Option<FollowRouteArm>`, with `arm_fade_in`/`fire_fade_in`;
  `apply_cell`, `place_carried`, `begin_block` (the probe stays inactive while armed) and `mix` hold the ramps.
  `adopt` keeps the route's own arm. `carry_route` asks `CarriedRoute::armed()` for either kind, and a
  following predecessor fires its own unfired arm when the successor holds none. `has_six_word_route` is now
  `has_live_route`.
- Host-core: the following-route exclusions at the 1324a duck-swap site and in `added_route_fades` are
  deleted, and so is the `!row.following` filter in `route_fade_arms`. That filter is a fourth site the
  scope note did not list: without its removal, preparation would fail on a following route.
  `added_route_fades` loses its unused `view` parameter.
- The existing `Following` row of `a_strip_keyed_from_a_restarted_strip_restarts_with_it_on_every_width`
  changes with R4: the bus input is quiet again (`eq1` leaves the set), and `late`'s following send is armed.
- Fact: in the gate's session, a send's tap node time runs 486 samples behind the block (behind `aux`'s
  limiter), so the gate's curve events sit 486 samples ahead of the render samples where they are heard.

## Decision record (attempt 2, root rulings after verification)
- MAJOR-1: in `carry_route`, a re-armed six-word successor of a following predecessor keeps its own
  arm: no `adopt_state` and no comparison. Before this fix the route was dead after its fire
  (the comparison was left pending, muted, with target `[0; 4]`).
- MAJOR-2: a following route's fire that comes before its first block (a fresh or a carried route)
  waits for that block. A fresh route places its cursors at the span start with no `Set`; a carried
  route runs `place_carried` with the arm still held. Then the fire composes with the curve heard at
  the fire block and ramps over the armed length. `carry_route` fires a following predecessor's own
  arm at once (`fire_now`).
- The gate gains row (d), an added following send fired on its first block, and counts render
  allocations (`Mode::Count`). A graph unit case covers a delayed armed route.
- The broken rustdoc link to `grown_strips` (prepare.rs) is now plain text: `transition` is a
  `control-provider` module, so a link breaks the build without that feature.

## Decision record (attempt 3)
- MAJOR-A: a fire-pending following arm counts as fired at the carry. `LiveFollowRoute::adopt` drops
  it and takes the predecessor's ramps and mute, which `place_carried` then compares with the heard
  curve, and `CarriedRoute::armed()` is false for it. The case: P1 arms R, P2 re-arms R, P1's arm
  fires, then P2 swaps in, `carry_fade_arms` copies `Done` and the successor's fire is deferred.
  Before this fix R dropped to zero at the swap.
- MINOR-A: in a deferred fire block, the fire runs after the placement and before the block's
  cell, so the block's own record drains after the fire, as for a six-word route. A record in the
  fire block can happen (a live send write before the successor's first block), so a graph unit
  test now covers it.
- Trace of `carried` to the `fade_arm_carry` pairs: each route fade becomes one arm, in order. A pair
  is pushed only when `install_fade_arms` returns a table entry for that arm. A carried fade whose arm
  is refused gets no pair, and nothing reports it. A refused arm is an arm on a route whose gate is
  already muted at preparation, or one that already holds an arm. Reported to root; not fixed here.
