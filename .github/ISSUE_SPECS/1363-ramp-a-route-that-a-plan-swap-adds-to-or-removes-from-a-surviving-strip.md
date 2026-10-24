# Ramp a route that a plan swap adds to or removes from a surviving strip

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `7414a5e18`.

**Narrowed to slice 1 (2026-10-24, root):** routes that fade **in**. Removed and re-pointed routes
and prepared send value changes (the old D1 (b), D4, D5 and gates 2, 4, 5, 7) moved to #1953
(slice 2), which follows #1325 (two-phase removal) so one design serves the fading route and the
removed strip's sends. Carrying an added route's pending fade-in across a later swap is #1877.

## Product outcome

When a structural edit adds a route (a send, or a strip's path to an output) between two strips
that both keep playing, the route's signal enters at its destination with a fade instead of a step.
When a strip fades in because the swap adds it (#1288), its sends from taps before the fader fade
in with it. Every other path stays bit-identical.

## Context

- A route: `Route { id, source, destination, channel_matrix, gain_db, mute, follows_mute }`
  (`crates/session/src/model.rs:883-901`); a destination is a submix input or an output input.
- Since #1391 every route into a submix and every route whose tap precedes its strip's fader has a
  live lane on every plan that attaches route lanes (`attach_route_controls_following`,
  `crates/graph/src/lib.rs`; `controls::route_rows_live`, `crates/controls/src/route.rs:226`; the
  C ABI through `controls::live_lanes`). Its op is a `LiveRoute` (`crates/graph/src/runtime.rs`),
  which ramps by an `IndexedRamp` and never folds. The only lane-less routes are `post_fader` and
  `post_pan` routes into an output (they keep their constant and their fold), and every route of a
  plan prepared with `HostLiveLanes::FADER_AND_MATRIX`.
- A route's compensation line sits on its destination edge, **after** its op: the consumer reads
  the op's output through the line (`LiveRoute::delayed`, `route_activity`; PDC puts the delay on
  the `RouteDestination` edge, `crates/graph-compiler/src/pdc.rs`). A fade the op writes enters the
  line and is heard whole, `compensation_delay` samples later. Only a line before the op (a raised
  floor on the `RouteSource` edge) and latent nodes upstream of the tap feed the op zeros that a
  fade must wait out.
- Before this issue no code armed a route: `step_fade_arms` fired only fader lanes, and host-core
  armed only added strips and carried strip rows (#1288, #1392). `crates/host-core/src/transition.rs`
  does not exist; the work is in `prepare.rs`.

## Decisions (slice 1)

- **D1. Which routes** (`added_route_fades`, `crates/host-core/src/prepare.rs`), on the control
  thread from the base model and the successor's model:
  - (a) **Added.** A route whose ID the committed model does not hold and whose source strip and
    destination it does hold. A route from a strip whose fader is armed (added, or holding an
    unfired predecessor entry, #1392) takes no entry when its tap follows the fader (the fader's
    fade shapes it) and waits for that strip when its tap precedes it.
  - (c) **Sends of an added strip.** Every route out of a strip the successor adds (#1288 D1) whose
    tap precedes the fader (`SendTap::precedes_fader`), with that strip's claim. Root: added
    strips only in this slice; duck-swapped and restored strips (#1324, #1325) join with #1953.
  - No entry: a route whose destination strip is added (its fader's fade shapes it), a route the
    session mutes (it arms nothing), and a following route with an automated source mute
    (`follows_automated_mute`: a `LiveFollowRoute`, which this slice does not arm, so it steps as
    before; left for #1953).
- **D2. Transition route.** `LiveRoute::control` and `GraphRouteControlBinding::control` are
  `Option`. `attach_route_controls_following` takes a `transition` set: each route of it that gets
  no lane binds as a `LiveRoute` with no lane (it never folds, and it takes a `GraphLiveRoute` row
  so #1877 can carry it). The new third return value, `GraphTransitionRoutes`, is charged through
  `RouteControlResources::with_transition_routes`. A transition route into the output makes the
  master's route fold decline for that plan (the fold needs every master input folded), as a
  pre-fader live route into the output already does. The fold is bit-neutral, so only cost moves,
  and only in that plan.
- **D3. Arm and fire.** `GraphFadeArm.fader` became `target: GraphFadeTarget { Fader(lane),
  Route(unit) }`; `install_fade_arms` refuses a route unit that is not a six-word `LiveRoute`.
  `LiveRoute::arm_fade_in(N)` keeps the prepared target, settles at `[+0.0; 4]` and mutes;
  `fire_fade_in()` runs `IndexedRamp::new(current, target, N)` and unmutes, the retarget a live
  unmute record runs. **Arm law (root):** a record read before the fire replaces only the stored
  target and mute; the route stays silent until the fire. **Delay:** for (a) from a strip that keeps
  playing, only a line on the route's `RouteSource` edge (normally none); for a route that waits
  for its strip, `arrival(Route node) - arrival(strip Input)` (plus a submix's largest route line
  into it, as `fade_arms_for` does), with the strip's claim. **Correction of the old D3:** the
  destination-edge `compensation_delay` is not waited for, because the line sits after the op
  (Context); waiting it out only delays a fade that is click-free already. Route arms take no
  inventory row (#1877 D1).
- **D3a. A swap before the fire, until #1877.** `carry_route` fires a displaced plan's armed route
  before the successor takes its state, so the successor ramps it in from the swap block. Without
  this the successor would take a silent route waiting for a fire its own table does not hold, and
  it would stay silent for good. #1877 replaces this by carrying the arm.
- **D6. Length.** `N = LiveRamps::for_session(next model).mute_samples` (D15-9's law, #1054's
  length). The shape is the route's own `IndexedRamp`, so after `N` the route mixes the
  reference's bits.
- **D7. Reporting.** No scheduled swap; the revision completes at adoption as `EXACT` (or
  `SUPERSEDED`), never a fallback flag.
- **D8. Realtime.** No render allocation: the arm table, the arms and the transition routes are
  sized at preparation. A plan with no added route is class A: it binds and renders what it did.

## Authorized paths

`crates/graph/src/{lib.rs,runtime.rs}`, `crates/builtins-compiler/src/lib.rs` (the one
pass-through), `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
`crates/capi/include/miso_engine_v1.h` (comments), `docs/C_ABI_V1_QUALIFICATION.md`, this spec.

## Non-goals

Removed and re-pointed routes and prepared send value changes (#1953); live value edits of a
surviving route (#1225); every route live on every plan; carrying a pending route arm (#1877); a
crossfade between plans.

## Objective gates

In `successor_swap.rs`: quantum 128, 48 kHz, the default mute ramp (480 samples), every backend of
`backends()`.

1. **Added route fades in** (`an_added_route_fades_in_after_its_line_fills_on_every_width`). The
   successor adds route `aux-bus` from the playing track `aux` into a submix whose input arrives
   486 samples late (a 486-sample destination-edge line at rest). Every block equals the successor
   session with the route muted, fresh, given a live unmute at the swap block; one block later is
   other bits. (b) inner: the same route `post_fader` into the output (a transition route) equals
   the lane-ful `pre_fader` reference (a unity fader moves no bit). (c) inner (old gate 3): with
   `aux` fed zeros, both swaps equal A's own run. The hand-over and every successor block make no
   allocator call (old gate 8, counted by the fade run as #1288's gate does:
   `the_swap_block_allocates_and_frees_nothing` prepares without live controls, so it arms
   nothing and cannot carry this claim).
2. **Pre-fader send of an added strip** (`an_added_strips_pre_fader_send_fades_with_it_on_every_width`,
   old gate 6): an added track with a `pre_fader` send into the bus, on a playing source (`D = 0`)
   and behind a 486-sample limiter on a source silent before the swap (`D = 486`). Every block
   equals the fresh session with track and send muted, given both live unmutes at the fader's fire
   block.
3. **Arm law** (graph, `a_record_before_an_armed_routes_fire_retargets_the_fire_only`) and **a
   swap before the fire** (`an_armed_route_a_swap_displaces_fires_at_the_swap`).
4. Commands: `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support,builtins-compiler/test-support`;
   `bash scripts/check-graph-policy.sh`, `bash scripts/check-host-core-policy.sh`,
   `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
   `bash scripts/check-graph-determinism.sh`, clippy, fmt, the AArch64 check.

## Test value

- Gate 1: an added route that enters at a step at the swap, or a lane-less added route that still
  folds and steps; (c): a transition plan whose arm or lost fold moves a neighbour's bits.
- Gate 2: a pre-fader send left unarmed beside an armed fader (a step into the bus at adoption), or
  armed without its strip's claim and delay (it fires before the limiter delivers).
- Gate 3: a live record that lifts the arm early; a carried armed route that stays silent.

## Evidence (attempt 1, 2026-10-24)

Mutants, each red and then restored: no route arms (gates 1 (a) and 2 red at block 9, where the
output hears the step 486 samples after the swap); arm only lane-ful routes (gate 1 (b) red);
re-arm every route of the committed model (gate 1 red); a send of an added strip without its
strip's claim and delay (gate 2 (c) red); a record lifts the arm (gate 3 red); no fire at the swap
(gate 3's carry test red).

## Dependencies

#1288, #1310, #1283, #1284, #1285, #1314, #1054, #1391 (closed). Followed by #1877 and #1953.
