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
