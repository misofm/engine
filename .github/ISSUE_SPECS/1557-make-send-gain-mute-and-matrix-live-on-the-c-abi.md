# Make send gain, mute and matrix live on the C ABI

Slice S6 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6,
the route family). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs.

## Product outcome

On iOS and Android, as in the browser, a send's gain, mute and channel matrix (a route into a
submix) change live, with the same ramps and bits as a plan prepared from the edited session. A
route into the output stays a rebuild for the reason its row states. The change is one lowering
function of the route family and six row flips per send.

## Context

- **Rows.** Route own rows 1-6 (`gain_db`, `mute`, `matrix_ll`..`matrix_rr`) of a route into a
  submix are `RebuildOnly(RuledLiveNotBuilt { issue: ROUTE_SLICE })`; of a route into the output,
  `RebuildOnly(Optimisation)`; `follows_mute` (row 7) waits for #1558 (#1546 D5). Ramps `SendGain`,
  `SendMute`, `SendMatrix`.
- **One coefficient function.** `graph_compiler::route_coefficients(gain_db, matrix, mute,
  source_lane_muted)` (`crates/graph-compiler/src/ids.rs:362`) is what preparation folds into a
  route's constants and what `RouteControlProducer::record` (`crates/host-core/src/route_controls.rs:178`)
  uses; `record` refuses a length above `ROUTE_RAMP_LENGTH_MAXIMUM` (`1 << 22`) and a value outside
  the route domain. A route cell holds one target and one ramp (#1347).
- **Lanes.** `attach_route_live_controls` (`crates/builtins-compiler/src/lib.rs:4045`) attaches one
  producer per route into a submix, in canonical route-ID order; routes into the output keep their
  constants and their fold. On the C ABI no route lane is attached today; #1555's
  `controls::live_lanes` sets `routes` as soon as a route row is `Live`.
- **Follow.** A route with `follows_mute` reads its source strip's effective lane mutes;
  `controls::effective` (#1547) gives them (`EffectiveRoute::gate`).
- **The issue this replaces.** #1225 (its D1 classification, D3 records, D4 ramps, D6 commit order
  and D7 resources are restated here for the family; its D8 carry belongs to #1284, open).
- **Resources.** The C ABI charges its producers in `capi_resources`
  (`crates/control-plane/src/compile.rs:175`); `graph::route_control_resources`
  (`crates/graph/src/lib.rs:1167`) sizes the route cells.

## Decisions frozen for this slice

- **D1. Flip the rows.** `ROUTE_SLICE` takes this issue's number. Rows 1-6 of a route into a submix
  become `Live`. Output routes and `follows_mute` keep their classes.
- **D2. The family's lowering.** `fn lower(Route, ..)` for one route whose live rows changed:
  `route_target(model, route) -> ([f32; 4], silenced)` computes the target exactly as preparation
  does, `route_coefficients(gain_db, matrix, mute, source_lane_muted)`, where
  `source_lane_muted` is the source strip's `controls::effective(model,
  &MonitorOverlay::default()).strips[source].mute` when `follows_mute` is set, else `[false; 2]`.
  It emits one record when the target
  bits or `silenced` differ between the two models (a gain edit on a muted route emits none; the
  later unmute carries the new gain). `LiveDelta` gains `routes`, in canonical route-ID order. A
  domain or length refusal is `LiveRebuild::Domain`.
- **D3. Ramps** (#1225 D4). One record per route. If the gate changes, the `SendMute` row governs
  (with the route's `RouteMute` edit entry). Otherwise the longest resolved length among the
  changed rows (`SendGain` with `RouteGain`, `SendMatrix` with `RouteMatrix`). Each through
  `LiveRamps::resolve`; an explicit `0` is a step.
- **D4. The commit.** `commit_live` keeps the plan's route producers beside its strip and effect
  producers, resolves each record's producer by route ID (binary search over the route-ID order),
  and writes the cells after every fallible step (classify, build records, live admission, resolve,
  commit predicate), then commits. A cell write cannot fail.
- **D5. Resources.** The route cells are charged in `capi_resources` by
  `graph::route_control_resources`; the resource oracles change by exactly those rows, re-derived
  from the function.
- **D6. Docs.** `docs/C_ABI_V1_QUALIFICATION.md` lists send gain, mute and matrix as live and
  states that a route into a submix carries a lane and does not fold.
- **D7. Acked-batch question.** As #1225 D6: no step after the first cell write can fail.

## Deliverables

1. D1; D2-D3 in host-core; D4-D5 in control-plane; D6.
2. The tests below.

## Authorized paths

- `crates/controls/**`.
- `crates/host-core/src/{live_delta.rs,prepare.rs,route_controls.rs}`,
  `crates/host-core/tests/live_delta.rs`.
- `crates/control-plane/src/{compile.rs,control.rs}`.
- `crates/capi/src/runtime/live_tests.rs`, `crates/capi/tests/resource_lifecycle.rs` (oracles).
- `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No `follows_mute`, VCA or solo liveness (#1558, #1559); the followed-mute guard stays.
- No lane for a route into the output, pre-fader or not (#1391).
- No carry of a send's ramp across a plan swap (#1284).

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Live and exact.** `SET_CONTROL` on a send's `gain_db`, `mute` and one matrix row commits with
   the plan epoch unchanged, and after the ramp every block is bit-identical to a plan prepared from
   the committed model; a gain edit on a muted send emits no record and the unmute carries it.
2. **Output routes rebuild.** A `post_fader` output route's `gain_db` and a `pre_fader` output
   route's `mute` each commit as a rebuild, and the classifier reports `RebuildOnly(Optimisation)`.
3. **Ramps.** Gain and matrix changed in one transaction with explicit lengths 240 and 480 give one
   record of 480; a mute toggle with a gain change takes the mute row's length.
4. **Atomic.** A transaction of a valid send gain and an out-of-domain matrix value is refused
   whole; no route cell is written.
5. **Realtime.** `audit capi` reports 0 allocations, frees, locks and syscalls. Any PCM digest that
   moves because a send lost its fold is listed in the PR with its cause.

## Test value

- Gate 1 turns red if a send edit rebuilds, lowers different coefficients than preparation, or
  re-ramps a muted send.
- Gate 2 turns red if an output route loses its fold through a live edit.
- Gate 3 turns red if a changed field moves faster than its own row's ramp.
- Gate 4 turns red if a cell is written before a later check refuses.
- Gate 5 turns red if the route lane adds render work that allocates.

## Dependencies

- #1554 (family dispatch), #1555 (lanes from rows), #1547 (`controls::effective` for the follow
  lanes), #1553 (`SET_CONTROL` for the tests).
- *Hold route-lane values in latest-target cells* (#1347, closed).

## Supersedes and shrinks

- Supersedes *Deliver value-only send edits to the running C ABI plan* (#1225).
- Shrinks *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391):
  its live-edit half becomes a class rule in the route rows; its render lane work stays.
- Shrinks *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the
  live console lanes* (#1053): G1 is lifted for sends.
