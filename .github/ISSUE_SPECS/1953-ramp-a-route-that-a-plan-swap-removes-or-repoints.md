# Ramp a route that a plan swap removes from or re-points on a surviving strip

Filed by root as #1363's slice 2 (backlog sprint, 2026-10-24). #1363 slice 1 covers routes that fade in.

## Split (scope note, 2026-10-25)

The original D4/D5 design (a fading copy in the successor, a line alias, retirement by the
watermark) predates #1325's two-phase machinery. The issue is split:

- **1953a (this slice): routes with a live lane** (every route into a submix, every route whose
  tap precedes the fader: `controls::route_rows_live`) ramp in phase 1 through #1325's machinery.
  No copy, no alias, no retirement, no inventory row.
- **1953b: lane-less output routes** (`post_fader`/`post_pan` into an output) keep the master fold
  by owner ruling (decision 14; D15-9); they need the successor-side fading copy (old D4/D5, with
  #1363's gates 2, 4, 5 and 7 for them). After an owner answer.

## 1953a decisions (root, binding)

- R1: a removed or re-pointed lane-ful route of a kept strip joins `plan_strip_transition`'s duck
  set: it is muted in phase 1 over `N`, its lines join `C`, and the successor waits for `S`.
  `StripTransition::ducks()` is strips or routes. A route of a strip ducked now or earlier, or into
  one, is left to that strip's duck; a route already muted, in the overlay, or following an
  automated mute is skipped.
- D15-9: a re-point (same ID, another source, tap or destination) is a removal plus an addition:
  `added_route_fades` case (a) arms it as added. #1877's re-arm keeps its own rule (same
  endpoints), so a re-pointed route is never re-armed with the displaced entry; it arms afresh.
- Attempt 2 ruling 1: a route whose old instance has no lane is still *known* in
  `added_route_fades` when re-pointed: its removal half cannot ramp in phase 1, so a re-point from
  a lane-less route keeps main's behaviour (a step on both sides, no fresh arm) until 1953b, which
  takes both halves. A half-state (step out, fade in) is refused. #1877's carried filter keeps the
  strict same-endpoints rule.
- `control.rs:5655` (a warm growth is refused only while `duck.strips()` is non-empty): a pending
  phase 1 that ducks only routes now exists. The verifier found it harmless: such an overlay ducks
  no strip, so no carried node of the growth's strips is ducked, and a warm candidate is still held
  `Primed` until the inherited `S` (`hold_until`). No control-plane edit.
- R2: an added following route with automated source mute steps (the #1953 comment) is #2011's;
  following routes stay excluded from the arm set and the duck set.
- Class A: no rendered bit moves on existing fixtures except
  `a_repointed_send_starts_at_its_prepared_value`, whose claim is rewritten to the ramp
  (`a_repointed_send_fades_in_at_its_new_destination`).

## Paths (1953a)

`crates/host-core/src/transition.rs`, `crates/host-core/src/prepare.rs` (`added_route_fades`),
`crates/host-core/tests/successor_swap.rs`, `crates/capi/tests/strip_transitions.rs`,
`crates/capi/include/miso_engine_v1.h` (comments), `docs/C_ABI_V1_QUALIFICATION.md`, this spec.

## Gates (1953a)

1. `transition::tests::a_transition_ducks_the_removed_strips_outside_the_overlay`: a removed and a
   re-pointed `pre_fader` send duck alone (`routes() == [b-send]`, no strip, `C` its line); a
   removed lane-less route ducks nothing.
2. `successor_swap`'s `a_repointed_send_fades_in_at_its_new_destination_on_every_width`: the
   re-pointed send arms (`D = 0`, `N`), restarts nothing, and equals a fresh plan with the send
   muted and live-unmuted at the swap block, (a) re-pointed to another bus and (b) to the same bus
   from another tap; an in-flight gain ramp does not carry; (c) a re-point from a lane-less route
   arms nothing.
   Row 1 also: a removed `pre_fader` route into the output ducks alone, `C` its line.
3. `capi`'s `a_removed_or_repointed_send_ramps_out_then_in_at_s`: (a) removal equals the live
   mute reference, adopted at `S` with `C = L`; (b) re-point equals a plan that live-mutes the old
   send and live-unmutes a copy into the new bus at `S`.

## Evidence (1953a, attempt 1)

- All existing `successor_swap` and `strip_transitions` cases pass unchanged except the rewritten
  test.
- Mutants, each red: no route duck; `C` without the route's line; `ducks()` on strips only; a
  re-point not ducked; a re-point not armed.
- Attempt 2 mutants, each red: `known` without the source check (gate 2 (b)); the duck set's
  `OutputInput => false` (gate 1); `known` without the lane-less rule (gate 2 (c)).
