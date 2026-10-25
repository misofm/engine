# Fade in an added route that was muted at preparation and is unmuted before its fire

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #2011
(its spec's evidence, "a refused arm is an arm on a route whose gate is already muted at preparation").

## Problem
A plan swap arms a fade-in on each route it adds (#1363 D3), so the route waits muted and opens at
`S` over the session mute ramp. A route the successor session mutes at preparation arms nothing. If
a live record unmutes that route between the swap block and `S`, the route opens at once over the
record's ramp, not at `S` over the fade. Nothing reports it: preparation keeps no inventory row for
the route, so no outcome flag or counter shows the missing fade. The defect is older than #2011 and
applies to every six-word route; a following route (#2011) has the same rule.

## Evidence
- `crates/graph/src/runtime.rs:1060-1065`: `arm_fade_in` returns `false` when `self.mute` is set
  ("A route the session mutes arms nothing").
- `crates/graph/src/runtime.rs:1706-1710`: the same rule for a following route (`flags.mute`).
- `crates/graph/src/lib.rs:4650-4666`: `install_fade_arms` returns `None` for an arm that armed nothing.
- `crates/host-core/src/prepare.rs:5121-5137`: the route-arm loop skips each `None` entry, so the
  route gets no `FadeArmInventoryRow` and no carry pair.

## Smallest slice
Decide and implement one rule for a route muted at preparation that a swap adds: either arm it
(muted, fire to its prepared mute so an unmute before `S` retargets the fire, as a record does on an
armed route), or keep it unarmed and report the case. Record the choice and its reason in this spec.
Cover six-word and following routes on every bank width.

## Gates
- One graph test: a route added muted, unmuted by a record before `S`, renders equal to the same
  route added unmuted (fade at `S`), on every width the build binds; red on the current code (PR evidence).
- No rendered bit moves for a route that is not muted at preparation (one PR comparison).
- `cargo test -p graph` and `-p host-core` pass.

## Paths
`crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/host-core/src/prepare.rs`, their
tests; this spec.

## Decision record (attempt 1, 2026-10-25)
- Rule (root): arm it. A route's mute is a live value, so a mute at preparation is its initial
  state. The six-word arm records the prepared target and mute (`[+0.0; 4]`, muted); a record
  before `S` retargets the fire (#1363 D3). A following route's fire composes with the flags it
  holds at `S` (#2011), so it needs only the refusal removed.
- A fire to a mute from a waiting route ramps nowhere: `LiveRoute::fire_fade_in` takes the arm
  and leaves the route settled at `[+0.0; 4]`, muted, inactive. Without this the fire ramps
  `0 -> 0` over the fade and mixes zeros (work on silence; a signed-zero bit could move). The same
  holds for an unmuted arm that a muting record reaches before `S`. A fading copy's fade-out is
  never muted while it waits, so it always ramps. A following route's fire to `[+0.0; 4]` moves
  no column bit, so it ramps nowhere already.
- Host (`added_route_fades`): a muted route arms only when its mute is live in the plan: a six-word
  lane (`six_word && route_rows_live`) or a following route. A lane-less muted route (a
  `post_fader`/`post_pan` send into the output, or no route lanes) cannot be unmuted before `S`,
  so arming it would only turn its op into a transition route for nothing; it still arms nothing.
- #1877 re-arm: a waiting kept route the successor mutes is re-armed and carries its entry, as any
  value change; only a re-point is excluded. This does not touch 1953b's copy logic (a lane-less
  in-flight route is not live, so the predicate skips it if muted).
- Third site, same rule: the duck-swap (restarted strip) send arm in `prepare.rs` (`armed_restarts`) also skipped
  `route.mute`; it now arms a muted send too (its `has_op` already means a lane).
- Width: `LiveRoute`/`LiveFollowRoute` are per-op code, not bank kernels; the graph claims are
  width-free unit tests, and the host assertion runs on every bound width.
- Successor (not fixed here): the strip fader arm (`crates/graph/src/lib.rs` `arm_fade_in`, "a strip
  the session mutes on both channels arms nothing") has the same defect shape.
