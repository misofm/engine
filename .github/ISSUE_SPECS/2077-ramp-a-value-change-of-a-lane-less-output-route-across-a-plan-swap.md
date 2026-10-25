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

## Decision record (attempt 1, 2026-10-25)
Root rulings, as the scope note recommends:
- **R1**: the swap block is the route's `S`; nothing carries. A lane-less route's own arm has
  `D = 0` and waits for no source claim, so its `S` is the first block of the plan that armed it,
  and a later rebuild is adopted at or after it. The verifier-note case cannot arise: a lane-less
  route out of a strip that still waits never arms itself (the strip's pair-carried fader arm
  owns the fade). Rejected: carry `S` through a muted arm entry (no reachable case); refuse or
  defer (refuses a valid edit).
- **R2**: no copy and no arm while the source strip is added or waiting (an unfired strip entry of
  the predecessor), or ducked by the transaction. Under #2080 a strip muted at preparation also
  arms, so it waits too and R2 covers it the same way.
- **R3**: a change of `mute` alone is a value change: an unmute is the fade-in only (a muted old
  route makes no copy), a mute is the copy fade-out only (a muted kept route arms nothing).

Implementation (`crates/host-core/src/prepare.rs`): one predicate, `crossfades_values`, names a kept
lane-less route at other values (same ID, source and destination; R2 applied; also not when the
predecessor's own arm of the route has not fired, since its carried entry already fades it in at
the new values, #1877 D2). `route_copies` then copies it at the old values (fade-out, alias of the
old op and lines), and `added_route_fades` (a) arms it like an added route (`strip: None`,
`D = 0`). The copy takes the predecessor's destination line; the kept route's line starts cold. No
graph change (one doc comment on `GraphFadeTarget::RouteOut`).

Evidence:
- New row `a_lane_less_route_value_change_crossfades_at_the_swap_on_every_width` (Simd8, Simd4):
  (a) gain change, with and without the 486-sample line; (b) unmute and mute; (c) R2, a waiting
  added track's route unmuted, or at another gain, by a rebuild before its fire.
- Red on main (0b8036ec1's `prepare.rs`): (a), (b)-unmute and (b)-mute make no arm and differ at
  block 9 (line) / 6 (no line) on both widths; (c) green on main (it defends R2). R2 mutant (drop
  the waiting-strip check): (c) "another gain" red (an extra copy arm).
- No bit moves elsewhere: base-vs-branch digest dump of `successor_swap` (method of #2069), 185
  `fade_run` digests identical (PR evidence, not a committed test).
