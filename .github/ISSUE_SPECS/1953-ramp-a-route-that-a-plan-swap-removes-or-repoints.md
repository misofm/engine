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
   removed lane-less route ducks nothing; a removed `pre_fader` route into the output ducks alone,
   `C` its line.
2. `successor_swap`'s `a_repointed_send_fades_in_at_its_new_destination_on_every_width`: the
   re-pointed send arms (`D = 0`, `N`), restarts nothing, and equals a fresh plan with the send
   muted and live-unmuted at the swap block, (a) re-pointed to another bus and (b) to the same bus
   from another tap; an in-flight gain ramp does not carry; (c) a re-point from a lane-less route
   arms nothing.
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

## 1953b decisions (root rulings and implementer facts, 2026-10-25)

- Root: keep the decision-14 / D15-9 master fold; no live lane for output routes. Build the
  successor-side fading copy; it fires at adoption (`f = 0`). b1 (removal) then b2 (re-point), one
  checkpoint each. b2 ramps both halves; the 1953a `known` rule is lifted only with the copy.
- The copy (`prepare.rs` `route_copies`): for a lane-less route of `committed` (not muted) that
  `next` lacks or re-points, whose source strip `next` keeps as the same kind and that no duck of
  the transaction silences (duck overlay, transition fallback), the successor compiles `next` plus
  the old route under a copy ID as a transition route. Only the effects, builtins and graph see the
  added route; every join, the catalog, the counts and the report read `next`. The copy ID is the
  old ID plus the fewest `-` that no route of either model (or an earlier copy) holds, so it sorts
  where the old route sorted (no ID lies between `x` and `x-`).
- Alias: the copy's node takes the old route's input floor, and its source and destination lines
  carry from the old route's (same endpoints); its op carries the old route's state when the
  predecessor gave it a live op (an in-flight transition route, or a kept copy). A copy out of a
  strip the successor restarts takes no op state and mutes at once (ramp 0): phase 1 ducked the
  strip, so its tap is silent at the swap.
- The arm: new `GraphFadeTarget::RouteOut`, `LiveRoute::arm_fade_out` (fires to `[0; 4]`, muted,
  over the session mute ramp, `D = 0`), and `GraphFadeArm::tail_samples` (the copy's destination
  line).
- Retirement: deviation from the scope note's watermark rule. `SuccessorBase` holds no watermark
  (it is control-plane state, outside this slice's paths), so render sets a second flag on the
  existing fade watch: an entry *settles* at the first block at or after fire + ramp + tail
  (`FadeArmState::Settling`). The inventory keeps one row per copy (`RouteCopyRow`: the copy's
  route and its fade entry); a successor keeps each copy whose entry has not settled, with its fade
  entry carried by pair (#1392 D3). Acked-batch question: the watch is a level written by render
  alone, no queue, so nothing is dropped; a flag read clear while render settles the entry only
  keeps a silent copy for one more plan.
- #1877 verify gap (root): a lane-less transition route whose entry fired but has not settled, kept
  unchanged, is re-armed and carried like an unfired one (#1877 D2, D6), so a successor prepared
  after the fire and adopted mid-ramp continues the ramp instead of folding to its constant.
- Out of this slice: a value change of a lane-less route (old D1 (b)(2)) still steps; a warm
  successor (#1354) compiles copies with the lead's floors but has no gate.

## Paths (1953b)

`crates/graph/src/{lib,runtime}.rs`, `crates/host-core/src/prepare.rs`,
`crates/host-core/tests/successor_swap.rs`, `crates/capi/tests/strip_transitions.rs`, this spec.
Comment text only, outside the scope note's list: `crates/host-core/src/transition.rs`,
`crates/capi/include/miso_engine_v1.h`, `docs/C_ABI_V1_QUALIFICATION.md` (the 1953a paragraphs said
such a route still steps).

## Gates (1953b)

1. `successor_swap`'s
   `a_removed_or_repointed_lane_less_route_fades_out_through_its_line_on_every_width`, with and
   without a 486-sample destination line: (a) a removal equals the `pre_fader` reference live-muted
   at the swap; (b) a second swap prepared after the fire, adopted mid-ramp, keeps the copy; (c) a
   second swap at block 11 drops a settled copy and keeps one whose line still delivers; (d) a
   re-point into the bus equals a live mute of the old route and a live unmute of the new one.
2. `a_pending_route_fade_survives_a_second_swap_on_every_width` (d) gains the swap prepared after
   the fire (the #1877 gap).
3. `capi`'s `a_removed_output_route_fades_out_through_its_line`: the C ABI parity row.
4. `graph`'s `a_fade_arm_fires_at_the_first_block_after_its_first_played_block_plus_d` checks the
   settled flag after each block.

## Evidence (1953b, attempt 1)

- Tests whose claim changed on purpose: `an_added_submix_delays_its_fade_by_its_longest_route_line`
  (the two re-pointed lane-less routes now add two copy arms) and
  `a_repointed_send_fades_in_at_its_new_destination` (its (c), "a re-point from a lane-less route
  arms nothing", is deleted: gate 1 (d) owns the new claim). Every other existing test passes
  unchanged.
- Mutants, each red: no copies; no line alias; no fade pair for a kept copy; no op carry for a
  copy; tail ignored; settled ignored; no in-flight re-arm (gate 2); settle at fire (gate 4); b2
  copy only on removal; b2 `known` keeps the lane-less rule. The capi row is red on no copies and
  on no line alias.
