# Ramp a value change of a lane-less output route across a plan swap

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #1953 (1953b),
old #1953 D1 (b)(2). The cited closed spec is read
at `da15a2824` (it has left this folder).

## Problem
A lane-less output route (a `post_fader` or `post_pan` route into the output: decision 14's master
fold, D15-9) has no live lane. A plan swap that changes one of its values (gain, matrix, mute) and
keeps its ID, source and destination steps the route to the new values at the swap block. 1953b
ramps only a removal and a re-point: the successor compiles a fading copy of the old route. A value
change makes no copy, so it is not smoothed.

## Evidence
- `crates/host-core/src/prepare.rs:7982-8004` (`route_copies`): the fresh-copy loop skips a route
  that `model` keeps with the same ID, source and destination (`kept`), so a value change gets no copy.
- `crates/host-core/src/prepare.rs:7983`: a route whose rows are live (`controls::route_rows_live`)
  or that the committed session mutes makes no copy either.
- `crates/host-core/src/prepare.rs:7887-7905` (`RouteCopy`): the copy machinery (copy ID, alias
  of the old op and lines, `GraphFadeTarget::RouteOut` fade-out, `Settling` retirement).
- 1953b spec decision record, "Successor issue (root files it)": a value change of a lane-less route
  still steps; the same copy machinery owns it.

## Verifier note (#2069), which the slice must decide
A fading copy made at the swap does not know the predecessor's `S`. So an unmute before `S` would
fade in at the swap, not at `S`. The slice decides
and records one rule for this case (for example, carry the predecessor's `S` into the copy pair, or
refuse or defer the case with a typed outcome). It may not leave the case silent.

## Smallest slice
For a lane-less route that the successor keeps (same ID, source, destination) with changed values,
the successor compiles a fading copy of the old route at the old values (fade-out, as 1953b) and
arms the kept route at the new values to fade in (a linear coefficient ramp over the session mute
ramp). Decide the verifier-note rule above. Record each choice and its reason in this spec.

## Gates
- One `successor_swap` row: a lane-less route whose gain changes across a swap renders equal to the
  live-ramp reference (old values fading out, new values fading in over the same ramp), on every
  width the build binds; red on the current code (PR evidence).
- One row for the verifier-note case, by the rule the slice chose.
- No rendered bit moves for a swap that does not change a lane-less route's values (one PR
  comparison against base, not a committed test).
- `cargo test -p host-core`, `-p graph` and `-p capi` pass.

## Paths
`crates/host-core/src/prepare.rs`, `crates/graph/src/{lib,runtime}.rs`,
`crates/host-core/tests/successor_swap.rs`, this spec.
