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
