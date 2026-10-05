# Follow an automated mute on following sends in render

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.3 and A6, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as drafts 13a
*Render stored mute automation on the strip* and 13c *Write a following send's route cell from the
shared commit*.**

## Product outcome

Every send into a submix with `follows_mute` from a strip whose mute the session automates follows
that automation in render, on every host and whatever the live-control options. The send's gate
changes at the render sample where the send's tap carries the timeline sample the curve names, so
the send mutes exactly the audio the strip mutes, latency included, and a saved session with an
automated mute and pre-fader sends leaks nothing. A route into the output never follows. The edits
that write the send's cell are draft 13c's.

## Context

- **Route lanes.** Preparation attaches one lane per route into a submix only with a control channel
  and `lanes.routes` (`crates/host-core/src/prepare.rs:1574-1592`). At block entry the route op
  applies its lane's value, ramping from the coefficients it is at
  (`crates/graph/src/runtime.rs:892-914`). The route ramp is the indexed law, a pure function of the
  frame index (`crates/lane/src/kernels.rs:1073-1095`, `IndexedRamp` `:1097`,
  `coefficients_at` `:1153`). A delayed route keeps feeding its compensation line
  (`crates/graph/src/runtime.rs:916-920`): the route op runs before the send's compensation delay.
- **The one route derivation.** `graph::gated_route_coefficients(transform, RouteGate { mute,
  follow_zeroed })` (`crates/graph/src/lib.rs:777-812`) is a `const fn` that allocates nothing;
  `RouteGate` is `:753-775`. `graph_compiler::route_values` gives the open transform
  (`crates/graph-compiler/src/ids.rs:309-322`), and `route_coefficients` composes it with the gate
  (`:341-355`).
- **Route cells.** *Hold route-lane values in latest-target cells* (#1347) D1: four coefficients,
  mute, ramp.
- **Node time** (README A1.3). A point whose signal arrives at `a` on the compensated graph
  processes, at render sample `r`, the audio of timeline sample `timeline(r + ΣP - a)`. For a send
  tap, `a(tap)` is the arrival of the tap's signal: the floored input arrival of the stage the tap
  reads after (#1285 D2), plus that stage's latency (PDC's output time,
  `crates/graph-compiler/src/pdc.rs:53-104`). For an input, post-input or insert-send tap behind
  inserts of latency `L`, `a(tap) = a(fader) - L`; for a post-fader or post-pan tap,
  `a(tap) = a(fader)`.
- **The strip side** is draft 13a: mute cells, the stage mute `curve || terms`, events at exact
  samples over the session mute length.
- **Swaps.** *Carry strip delay lines and live send ramps across a plan swap* (#1284) D2 carries a
  live send's ramp by route ID.
- **#1226 D6** leaves the follow of an automated mute to #1058, through the same composition.

## Decisions frozen for this slice

- **D1. A route lane for every following route of an automated mute, on every plan.** Preparation
  attaches a route lane, on both hosts and whatever the live options, to every route into a submix
  whose source strip has an automated mute lane. For such a route, this slice gives the route cell
  seven words instead of #1347's folded coefficients: the open transform (`gain`, `ll`, `lr`, `rl`,
  `rr`), one flags word (route `mute`, `follows_mute`, `terms[2]`, `automated[2]`), and the ramp.
  `terms[lane]` is the source lane's other mute terms: VCA mute and solo mute for an automated
  lane; the whole effective mute for a lane that is not automated. `automated` is fixed by the
  plan. Preparation seeds the cell. Every other route keeps #1347's layout; #1347 is not amended.
- **D2. The route op follows the curve at the tap's arrival.**
  - The route op evaluates its source strip's mute cells itself, with its own cursors over the same
    plan-immutable tables, in the node time of its tap, `τ_tap(r) = timeline(r + ΣP - a(tap))`. A
    curve event at timeline sample `t` therefore falls at the render sample where the tap carries
    `t`. For a post-fader tap that is the strip's own event sample; for a tap behind inserts of
    latency `L` it is `L` samples earlier in render time, and after the send's compensation delay
    the send's gate change meets the strip's on the same timeline sample at the submix input.
  - **Why the tap's arrival, not the fader's.** A1.3 says automation lines up with the audio it acts
    on at every node, latency included. With the fader's arrival, a pre-insert send would gate `L`
    samples of timeline late, and pass `L` samples of audio that the strip mutes: the leak A6 exists
    to prevent. *Let C ABI sends follow their source strip's mute live* (#1226) D5 ramps a live
    follow together with the strip in render time, because a live mute has no timeline sample; a
    stored curve has one, and the timeline rule governs it.
  - At each source mute event at render sample `r`, and at the block entry after its cell changes:
    `follow_zeroed[lane] = follows_mute && ((automated[lane] && curve[lane](τ_tap(r))) || terms[lane])`;
    the target is `gated_route_coefficients(&transform, RouteGate { mute, follow_zeroed })`. Only a
    target whose bits change retargets: a new `IndexedRamp` from `coefficients_at(position)` at
    offset `r` over the session mute length (the cell's ramp word for a cell change). A seek sets
    the target exactly where it reaches the tap.
  - The session mute length of a route ramp is the `mute` word of draft 12's plan cell, read from
    the same block context as the strip's fader stage reads it, so the strip ramp and the send ramp
    have the same length.
  - The route op gains an in-block retarget at an offset on draft 08's model: mix `[0, o)`, retarget,
    mix `[o, q)`. The indexed law makes the split class A.
- **D3. Swaps.** A route lane present in both plans carries by route ID under #1284 D2, whether or
  not its source mute is automated in either plan: a lane gained or lost for D1 is not a change of
  the route's structure. This slice extends #1284's join for it; #1284 is not amended.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** Render generates curve
  events from the plan; nothing is queued.

## Deliverables

1. D1: route lanes for following routes, the seven-word cell and its seed.
2. D2: the route op's evaluation at the tap's arrival and the in-block retarget.
3. D3: the carry join.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the route lanes, the cell seed and the carry join),
  `crates/host-core/tests/route_mute.rs` (exists; new cases)
- `crates/graph-compiler/src/ids.rs` (only to make `route_values`, `:309-322`, public or to expose
  a public wrapper for it), `crates/graph-compiler/src/pdc.rs` (only to export `a(tap)`, if #1285's
  inventory does not already give it)
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the route lane, its cell and the route
  op only; stream A's files, sequenced by root), `crates/lane/src/kernels.rs` (a range entry only,
  if the split needs one)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `crates/control-plane/tests/follow_automation_realtime.rs` (new)
- `crates/control-plane/Cargo.toml` (a `bench-support` dev-dependency for that binary, only if an
  earlier slice has not added it)
- `crates/graph/Cargo.toml` (a normal `automation` dependency: the route op runs draft 07's
  cursors in graph, and graph cannot reach them through builtins-compiler,
  `scripts/check-builtins-policy.sh:21`), `Cargo.lock`, `scripts/check-graph-policy.sh` (the
  pinned graph dependency list, `:19-22`) and `scripts/test-graph-policy.sh` (its fixture list,
  `:7-13`), each gaining `automation` only. This slice is the first to add the edge.

## Non-goals

- The edits that write the route cell, a live `follows_mute` toggle and the solo and VCA terms
  (draft 13c). The strip's mute events (draft 13a). Send automation (OQ2). Routes into the output,
  which never follow (decision 13 P11).

## Hazards

- **One push with 13a and 13c.** Root merges the three together.
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
2. **Same timeline sample, same length** (`crates/host-core/tests/route_mute.rs`, new cases). With a
   latent insert of `L = 64` before the fader and an `input` tap: the send's route ramp starts `L`
   render samples before the strip's mute ramp, with the same length, so both act on the same
   timeline sample; with a `post_fader` tap both start on the same render sample. At quanta 128 and
   100.
3. **Route split** (`crates/graph` test, new). A route retargeted at offset `o` in `{1, 63, 64, 127}`
   (`q = 128`) and in a block of 100 equals the same block split at `o` with the retarget at the
   second part's entry, bit for bit.
4. **Carry** (`crates/host-core/tests/route_mute.rs`). A rebuild that adds an automated mute to `t`
   keeps the send's ramp state by route ID; the swap block allocates and frees nothing.
5. **Realtime.** `crates/control-plane/tests/follow_automation_realtime.rs` (new integration binary in control-plane, the crate that
   holds the shared commit, `control_plane::SessionState`, #1309 D1-D9. A host-core binary cannot
   reach the commit without a dev-dependency cycle, since control-plane depends on host-core.
   `scripts/check-bench-policy.sh:257-280` allows a `bench-support` dev-dependency only in a
   `crates/` manifest, so no host-web binary can link it. The binary links `bench_support::alloc`
   and calls `assert_installed()` first). It drives the script through control-plane's shared
   commit and host-core's render session, the code the browser Worker (#1382) and the C ABI both
   run. For gate 1's script:
   `allocations == 0 && frees == 0` around every render call after warm-up.
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
6. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p graph --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p control-plane --features test-support`
   - the workspace debug leg (`test-debug-a`) in `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-graph-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule), `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a following send ignores the curve (the leak A6 exists to prevent), or if a plan
  without live controls attaches no lane for it.
- Gate 2: red if the route op times its events at the fader's arrival (a pre-insert send would gate
  `L` samples late and leak) or at the route's own arrival, or if the two ramps differ in length.
- Gate 3: red if the in-block retarget restarts the indexed ramp from the target or from rest.
- Gate 4: red if gaining automation on the source re-points the send and drops its ramp.
- Gate 5: red if the route op's evaluation or the cell read allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 13a *Render stored mute automation on the strip* (same push).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the route ramp length is its `mute`
  word).
- *Carry strip delay lines and live send ramps across a plan swap* (#1284): the join D3 extends.
- *Hold route-lane values in latest-target cells* (#1347): the cell D1 lays out for these routes.

Draft 08's split model, draft 07's per-cell events and #1285's arrivals arrive through draft 13a.
Draft 13c depends on this draft and lands in the same push.
