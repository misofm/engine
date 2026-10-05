# Let following sends follow an automated mute

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A6 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in one push with draft 13a
*Render stored mute automation on the strip*: 13a must not reach `main` without 13b.**

## Product outcome

Every send into a submix with `follows_mute` from a strip whose mute the session automates follows
that automation on every host. The strip ramp and the send ramp start on the same sample with the
same length, so a saved session with an automated mute and pre-fader sends leaks nothing. Solo and
VCA mutes compose with the curve for the send as for the strip, and a live `follows_mute` toggle on
a playing engine is followed. A route into the output never follows.

## Context

- **Route lanes.** Preparation attaches one lane per route into a submix only with a control channel
  and `lanes.routes` (`crates/host-core/src/prepare.rs:1574-1592`). At block entry the route op
  applies its lane's value, ramping from the coefficients it is at
  (`crates/graph/src/runtime.rs:892-914`). The route ramp is the indexed law, a pure function of the
  frame index (`crates/lane/src/kernels.rs:1073-1095`, `IndexedRamp` `:1097`,
  `coefficients_at` `:1153`).
- **The one route derivation.** `graph::gated_route_coefficients(transform, RouteGate { mute,
  follow_zeroed })` (`crates/graph/src/lib.rs:777-812`) is a `const fn` that allocates nothing;
  `RouteGate` is `:753-775`. `graph_compiler::route_values` gives the open transform
  (`crates/graph-compiler/src/ids.rs:309-322`), and `route_coefficients` composes it with the gate
  (`:341-355`).
- **The browser's follow pass** (`hosts/host-web/src/lib.rs:5274-5290`) recomputes following routes
  through `LiveRouteMuteFollow::delta` (`crates/host-core/src/live_route_state.rs:232-257`), which
  yields only changed routes; the mirror holds `follows_mute` and `source_lane_muted` (`:46-53`).
- **The C ABI.** After *Deliver value-only send edits to the running C ABI plan* (#1225) and *Let C
  ABI sends follow their source strip's mute live* (#1226), the classifier emits route records from
  `route_target(model, route)`, follow records included (#1226 D3). Route lanes hold latest-target
  cells (*Hold route-lane values in latest-target cells*, #1347 D1: four coefficients, mute, ramp).
- **The strip side** is draft 13a: mute cells, the stage mute `curve || terms`, events at exact
  samples over the session mute length.
- **Swaps.** *Carry strip delay lines and live send ramps across a plan swap* (#1284) D2 carries a
  live send's ramp by route ID.

## Decisions frozen for this slice

- **D1. A route lane for every following route of an automated mute, on every plan.** Preparation
  attaches a route lane, on both hosts and whatever the live options, to every route into a submix
  whose source strip has an automated mute lane. Its cell (#1347 D1 amended, README row) holds seven
  words: the open transform (`gain`, `ll`, `lr`, `rl`, `rr`), one flags word (route `mute`,
  `follows_mute`, `terms[2]`, `automated[2]`), and the ramp. `terms[lane]` is the source lane's
  other mute terms: VCA mute and solo mute for an automated lane; the whole effective mute for a lane
  that is not automated. `automated` is fixed by the plan. Preparation seeds the cell.
- **D2. The route op follows the curve.**
  - The route op evaluates its source strip's mute cells itself, with its own cursors over the same
    plan-immutable tables and the source fader node's arrival `a(n)`. So its events fall on exactly
    the strip's render samples, and no mutable state is shared between the two ops.
  - At each source mute event at `r`, and at the block entry after its cell changes:
    `follow_zeroed[lane] = follows_mute && ((automated[lane] && curve[lane](r)) || terms[lane])`;
    the target is `gated_route_coefficients(&transform, RouteGate { mute, follow_zeroed })`. Only a
    target whose bits change retargets: a new `IndexedRamp` from `coefficients_at(position)` at
    offset `r` over the session mute length (the cell's ramp word for a cell change). This is #1226
    D5's rule (strip and sends ramp together) at a sample offset. A seek sets the target exactly.
  - The session mute length of a route ramp is the `mute` word of draft 12's plan cell, read from
    the same block context as the strip's fader stage reads it. So after a `control_smoothing` edit
    the strip ramp and the send ramp still have the same length.
  - The route op gains an in-block retarget at an offset on draft 08's model: mix `[0, o)`, retarget,
    mix `[o, q)`. The indexed law makes the split class A.
- **D3. Edits.** For a route of D1, a route gain, matrix or mute edit, a `follows_mute` toggle, and
  a change of a source lane's static mute, VCA mute or solo write the route cell (`live`) and give no
  follow record. On the C ABI this is #1225's route record in D1's layout (#1225, #1226 and #1347
  amended). In the browser the follow pass writes D1's cell instead of a follow record, and only
  when its words change.
- **D4. The C ABI has no solo**, and any VCA session rebuilds until #1247, so there `terms` is the
  static mute of a lane that is not automated.
- **D5. Swaps.** A route lane present in both plans carries under #1284 D2, whether or not its
  source mute is automated in either plan: a lane gained or lost for D1 is not a change of the
  route's structure (README amendment row for #1284 D2).
- **D6. The acked-batch question: can an ack ever precede a drop? No.** Every check of a batch
  precedes the first cell write; writes cannot fail; render generates curve events from the plan.

## Deliverables

1. D1: route lanes for following routes and the route cell layout.
2. D2: the route op's evaluation and in-block retarget.
3. D3-D5: the classifier, the C ABI route record, the browser follow pass, the carry rule.

## Authorized paths

- `crates/host-core/src/{prepare.rs,live_delta.rs,live_route_state.rs}`,
  `crates/host-core/tests/{live_delta.rs,route_mute.rs,live_routes.rs}`
- `crates/graph-compiler/src/ids.rs` (only to make `route_values`, `:309-322`, public or to expose
  a public wrapper for it)
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the route lane, its cell and the route
  op only; stream A's files, sequenced by root), `crates/lane/src/kernels.rs` (a range entry only,
  if the split needs one)
- `crates/control-plane/src/` (the route record), `crates/capi/src/runtime/live_tests.rs`
- `hosts/host-web/src/lib.rs` (the follow pass only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/follow_automation_realtime.rs` (new)

## Non-goals

- The strip's mute events (draft 13a). Send automation (OQ2). Routes into the output, which never
  follow (decision 13 P11).
- Live VCA edits on the C ABI (#1247).

## Hazards

- **One push with 13a.** 13b depends on 13a's mute cells and events; root merges them together.
- **A redundant retarget moves bits** (README "Change only"). D2 compares the whole target.
- **Delayed sends.** A follow-muted delayed send stays active and mixes zeros (#1226 Hazards). Its
  comparison point includes its compensation delay.
- **Two readers of one table** are safe only because the tables are immutable after preparation and
  each op owns its cursor; gate 2 proves the samples agree.

## Objective gates

1. **No leak** (both hosts: `crates/capi/src/runtime/live_tests.rs` and `hosts/host-web/src/tests.rs`,
   new). Track `t` sends `pre_fader` with `follows_mute: true` into bus `b`; `t` has a mute entry on
   `both` with steps 0, 1, 0, at samples that are not grid samples. From the end of each mute ramp
   (plus the send's compensation delay), the output is bit-identical to a plan prepared with the same
   static mute, fed the same PCM. With live controls off in the browser too.
2. **Same sample, same length** (`crates/host-core/tests/route_mute.rs`, new). The strip's mute ramp
   and the send's route ramp start on the same render sample with the same length, at quanta 128 and
   100, and with a latent insert before the fader.
3. **`follows_mute` toggle** (both hosts, new). On a playing engine with `t` automated-muted, a
   transaction turns `follows_mute` off, then on: each is `live`; after the route ramp the output
   equals a fresh plan of the committed model.
4. **Composition** (browser, new). Solo on another track mutes `t` by solo while its curve is 0;
   the send follows; unsolo restores the curve's state. A route into the output never follows.
5. **Route split** (`crates/graph` test, new). A route retargeted at offset `o` in `{1, 63, 64, 127}`
   (`q = 128`) and in a block of 100 equals the same block split at `o` with the retarget at the
   second part's entry, bit for bit.
6. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A `follows_mute` toggle on a D1 route
   gives one route-cell write and no follow record; a static mute change on the source's
   non-automated lane gives that lane's `Mute` record and one route-cell write.
7. **Realtime.** `hosts/host-web/tests/follow_automation_realtime.rs` (new integration binary, links
   `bench_support::alloc`, calls `assert_installed()` first) runs gate 1's browser script:
   `allocations == 0 && frees == 0` around every render call after warm-up.
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
8. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p graph --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a`) in `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-graph-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
9. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a following send ignores the curve (the leak A6 exists to prevent), or if a plan
  without live controls attaches no lane for it.
- Gate 2: red if the route op computes its own node time from the route's arrival instead of the
  source fader node's, or if the two ramps differ in length.
- Gate 3: red if `follows_mute` is still read from the prepared route only, so a live toggle is lost
  on an automated route.
- Gate 4: red if solo overwrites the curve instead of composing with it for the send.
- Gate 5: red if the in-block retarget restarts the indexed ramp from the target or from rest.
- Gate 6: red if a toggle still emits a folded follow record that the curve's next event would
  contradict.
- Gate 7: red if the route op's evaluation or the cell read allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 13a *Render stored mute automation on the strip* (same push).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the route ramp length is its `mute`
  word).
- *Deliver value-only send edits to the running C ABI plan* (#1225).
- *Let C ABI sends follow their source strip's mute live* (#1226), amended.
- *Carry strip delay lines and live send ramps across a plan swap* (#1284), amended at D2.
- *Make a send's follows_mute live in the browser* (#1342), amended.
- *Hold route-lane values in latest-target cells* (#1347), amended at D1.

Draft 08's split model and draft 07's per-cell events arrive through draft 13a.
