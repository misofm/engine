# Follow an automated mute on following sends in render

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.3 and A6, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as #1651
*Render stored mute automation on the strip* and #1653 *Write a following send's route cell from the
shared commit*.**

## Product outcome

Every send into a submix with `follows_mute` from a strip whose mute the session automates follows
that automation in render, on every host and whatever the live-control options. The send's gate
changes at the render sample where the send's tap carries the timeline sample the curve names, so
the send mutes exactly the audio the strip mutes, latency included, and a saved session with an
automated mute and pre-fader sends leaks nothing. A route into the output never follows. The edits
that write the send's cell are #1653's.

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
- **The strip side** is #1651: mute cells, the stage mute `curve || terms`, events at exact
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
  - The session mute length of a route ramp is the `mute` word of #1639's plan cell, read from
    the same block context as the strip's fader stage reads it, so the strip ramp and the send ramp
    have the same length.
  - The route op gains an in-block retarget at an offset on #1638's model: mix `[0, o)`, retarget,
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
- `crates/graph/Cargo.toml` (a normal `automation` dependency: the route op runs #1637's
  cursors in graph, and graph cannot reach them through builtins-compiler,
  `scripts/check-builtins-policy.sh:21`), `Cargo.lock`, `scripts/check-graph-policy.sh` (the
  pinned graph dependency list, `:19-22`) and `scripts/test-graph-policy.sh` (its fixture list,
  `:7-13`), each gaining `automation` only. This slice is the first to add the edge.

## Non-goals

- The edits that write the route cell, a live `follows_mute` toggle and the solo and VCA terms
  (#1653). The strip's mute events (#1651). Send automation (OQ2). Routes into the output,
  which never follow (decision 13 P11).

## Hazards

- **One push with #1651 and #1653.** Root merges the three together.
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

- #1651 *Render stored mute automation on the strip* (same push).
- #1639 *Hold the automation jump lengths in a plan cell* (the route ramp length is its `mute`
  word).
- *Carry strip delay lines and live send ramps across a plan swap* (#1284): the join D3 extends.
- *Hold route-lane values in latest-target cells* (#1347): the cell D1 lays out for these routes.

#1638's split model, #1637's per-cell events and #1285's arrivals arrive through #1651.
#1653 depends on this draft and lands in the same push.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Route lanes exist on every plan** (C6, #1557): every route into a submix has a live lane and a
  `ROUTE_CELL_WORDS = 6` cell (`crates/graph/src/lib.rs:1004-1047`). D1's "attach a route lane" is
  done; D1 keeps the seven-word layout for a route whose source mute is automated.
- **`terms` includes the source's static mute** (OQ1 = C, #1651 amended): for an automated
  source lane, `terms = static_mute || VCA mute || solo term`.
- **#1654** (OQ2 = yes) reuses this slice's seven-word cell and in-block route retarget for a
  route whose own rows are automated; build them so the transform words can come from curves too.
- **Moved files.** The follow gate is computed by `controls::effective` (C7); route records come
  from `route_records` (`crates/host-core/src/live_delta.rs:1416`) and `RouteControlProducer`
  (`crates/host-core/src/route_controls.rs:201`); `live_route_state.rs` is gone.
- **Dependencies.** #1284 is open; #1347 has landed.

## Amendment (controls coordinator, batch 10, 2026-10-14)

- **Automated VCA mutes** (#1655): a route into a submix whose source lane is reached by a VCA
  with an automated mute also uses this slice's seven-word cell, and its follow term ORs those VCA
  curves, evaluated in the tap's node time. #1655 adds it on this slice's route op; this slice
  needs no change for it.

## Amendment (controls coordinator, batch 18 spec check, 2026-10-17)

Checked against `ad03780ee`. The filed text above stays; where it disagrees, this section wins.

**Anchors now.**
- Route cell: `RouteControlRecord` `crates/graph/src/lib.rs:963`, `ROUTE_CELL_WORDS = 6` `:1007`,
  `RouteLane` `:1047`, `GraphRouteControlProducer` `:1085`. `RouteGate` `:770`,
  `gated_route_coefficients` `:801`. `LatestCell<N>` is const-generic
  (`crates/engine/src/realtime/latest_cell.rs:590`), so a seven-word cell needs no edit under
  `crates/engine/src/realtime/`.
- Route op: `LiveRoute` `crates/graph/src/runtime.rs:887` (`apply_pending` `:928`, `active`
  `:964`, `mix` `:970`, `adopt` `:981`); the drain and activity in `execute_op` `:3676-3688`; the
  mix arm `:3791`. `execute_op` already takes the block's `GraphBlockTime` (`:3660`).
- Ramp: `IndexedRamp` `crates/lane/src/kernels.rs:1589`, `coefficients_at` `:1645`,
  `route_mix_ramp_block` `:1698`. The kernel takes sub-slices and a start position, so the split
  needs no kernel change. `crates/lane/src/kernels.rs` leaves the paths.
- `route_values` `crates/graph-compiler/src/ids.rs:322-338` (`pub(crate)`), `route_coefficients`
  `:362-376`, re-exported at `crates/graph-compiler/src/lib.rs:323`.
- Node time: `GraphBlockTime` `crates/graph/src/lib.rs:1298` (`node_span` `:1350`,
  `jump_lengths` `:1341`), `GraphJumpLengths` `:1244`, `floored_input_arrival` `:1362`.
  `GraphCompileReport::input_arrivals` (`crates/graph-compiler/src/lib.rs:194`) holds every node's
  floored input arrival, route nodes included. A route op reads its one undelayed input before the
  consumer's compensation delay, so the route node's floored input arrival is `a(tap)`.
  `crates/graph-compiler/src/pdc.rs` leaves the paths.
- Route lanes attach at `crates/host-core/src/prepare.rs:2867-2888`. Since C6 (#1557) every route
  into a submix is a live row, so every plan on both hosts has its lane (D1's attach is done).
- Carry: `join_routes` `prepare.rs:5473`, `resolve_route_carry` `:5537`, the join's retarget
  writes `:3654`; render side `carry_live_routes` `runtime.rs:5261`.
- The one record function of a send is `route_record_between` (`prepare.rs:4154-4193`). The
  classifier (`route_records`, `crates/host-core/src/live_delta.rs:1467`) and the join call it.
- Graph's dependency pin is `scripts/check-graph-policy.sh:21`; the fixture is
  `scripts/test-graph-policy.sh:7-13`. Batch 17 did not add `automation` to graph (it put the
  fader program in builtins-compiler), so this slice is still the first to add the edge.
- `scripts/check-realtime-policy.sh` is deleted (c1aadc234). The gate is
  `cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`.
- `crates/control-plane/Cargo.toml` already has `bench-support` (`vca_automation_realtime.rs`).
  It leaves the paths.

**How the slice maps onto the landed code.**
- **Program hand-off.** The plan's `Arc<automation::Program>` is built in builtins-compiler
  (`prepare_fader_automation`, `crates/builtins-compiler/src/lib.rs:2087`; #1651 adds the mute
  cells). #1651 exports, per strip, the program and the two mute cell indices to host-core
  preparation, as a handle type that graph defines. Host-core passes it, with `a(tap)` from
  `floored_input_arrival(report.input_arrivals, route node)`, to the route attach. Host-core needs
  no `automation` dependency. The "automated" test is #1651's mute-lane predicate in `controls`
  (as `automated_fader_lanes`, `crates/controls/src/automation.rs:447`).
- **Layout.** A seven-word route has its own lane type (`LatestCell<7>`) and its own boxed render
  state beside the route op. Every six-word route keeps its bytes (`LIVE_ROUTE_OWNER_BYTES`,
  `route_control_resources`) and its bits. The seven-word lane and its render state are charged in
  `route_control_resources`; the program is charged once, by builtins-compiler.
- **Flags word.** Bit 0 route `mute`, bit 1 `follows_mute`, bits 2-3 `terms[L, R]`, bits 4-5
  `automated[L, R]`, other bits 0 (decode refuses them, as `mute > 1` today). After OQ1 = C,
  `terms[lane]` is always `controls::effective`'s source-lane mute (it holds no curve), for an
  automated lane and for one that is not. `automated` is the plan's: render takes it from its own
  cursors, and the commit refuses, before the first write, a record whose bits differ (#1653).
- **Node time (R17-3).** The route op takes the same per-block context the builtin banks get
  (`GraphBlockTime`, set once per block), calls `node_span(a(tap))` (signed, R17-5) and builds its
  `automation::NodeSpan`. It uses `CellEvents` with the mute cell's rule (#1651; `SeekMode::Exact`,
  so a seek gives a `Set`, length 0).
- **Ramp length.** `GraphJumpLengths` "knows no row", so graph must not spell the mute row. The
  attach passes a resolver from host-core:
  `LiveRamps::from_lengths(fader, mute, pan).for_row(LiveRampRow::Mute)`, the rule the strip's
  stage uses. Gate 2 holds that the two lengths agree.
- **Order in one block.** First the cell at block entry: compose with the curve's value at the
  block's first sample and retarget over the cell's ramp word on a bits change. Then the curve
  events in offset order: each composes and retargets over the session mute length (0 for a `Set`)
  on a bits change. #1651 uses the same order on the strip, so gate 2 holds.
- **Seed.** The bound target composes the curve's first `start_value` (as #1651's prepared stage
  mute) with the seeded words. A route of a fresh plan whose curve at its first block's `τ_tap`
  differs sets exactly there (length 0), as the strip's fresh lane does (#1647 D2, #1651).
- **Activity (fix).** Today the op decides `active` at block entry, before it mixes. A seven-word
  route is active in every block where it retargets, so an unmute inside a block is mixed. It finds
  its changes first (for example on a copy of its `CellState`, which is `Copy`, 32 bytes). A
  muted, undelayed route with no change in the block stays inactive and moves no bit.
- **FP (R17-7).** Render composes inside the render entry's `CanonicalFpEnv`; the seed and the
  commit compose inside R17-7's guards. One `const fn`, `gated_route_coefficients`, gives both.
- **Scalar oracle (R17-4).** The route op is not banked and has one form. The strip's bank and
  its oracle are #1651's.
- **No queue.** D4 holds: render generates the events from the plan.

**D3, made exact (fix).**
- The lane exists in both plans, so the move by route ID always applies. Ramp, position and mute
  carry as today (`LiveRoute::adopt`).
- The follow cursors do not carry. A mute cell has no grid and no hold, so the successor places
  each cursor fresh at its own `τ_tap` of the first block and seeds it with `value_at` there.
- At the successor's first block: the join's record (if any) applies first, through the cell. Then
  the route compares its composed target with the adopted ramp's target. On a bits change it
  retargets from `coefficients_at(position)` over the session mute length.
- A six-word successor of a seven-word predecessor (the curve removed) makes the same comparison
  with its own prepared target. The model diff cannot see the curve, so only render knows the
  carried state. A six-word pair keeps today's code exactly.
- The join's record for a seven-word successor route is the seven-word record from
  `route_record_between` (#1653's branch).
- No `TransitionFallback` and no new watermark rule. A carried ramp whose target does not change
  keeps its own length. A changed target is the new curve's own jump, which an uninterrupted plan
  makes the same way at a curve event (as #1648's "Note 1" for a cell's own jump). B17-R14's
  window does not read routes.

**Gates (fixes).**
- Gate 1: drop "With live controls off in the browser too". There is no such option now (C6).
  Both hosts run it at the native width: `crates/capi/src/runtime/live_tests.rs` and
  `hosts/host-web/tests/control_parity.rs`. Host-core runs the same script in
  `crates/host-core/tests/route_mute.rs` at `Backend::Simd8` (`cfg(target_feature = "avx2")`) and
  `Backend::Simd4`, with the `Scalar` oracle (`test_only_prepare_host_runtime_*_on`).
- Gates 2 and 4 also run at `Simd8` (avx2) and `Simd4`.
- Gate 3 is a unit test in `crates/graph/src/runtime.rs`'s test module (the route op is
  `pub(crate)`). Add one case: a route that unmutes at offset `o` while inactive is mixed on
  `[o, q)`.
- Gate 4, new case (b): a rebuild that removes `t`'s mute entry while the curve holds the send
  muted unmutes the send over the session mute length from the swap block; it allocates nothing.
  Red if the six-word successor keeps the adopted mute.
- Gate 6: replace `bash scripts/check-realtime-policy.sh` with the `realtime-policy` binary above;
  add `bash scripts/test-graph-policy.sh` and `bash scripts/check-builtins-policy.sh`.

**Authorized paths after the check.**
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (route lane, cell, record, producer
  write, the route op, its activity, its carry). Render code, but not under
  `crates/engine/src/realtime/`: no root notice is needed.
- `crates/graph/Cargo.toml`, `Cargo.lock`, `scripts/check-graph-policy.sh`,
  `scripts/test-graph-policy.sh` (each gains `automation` only).
- `crates/graph-compiler/src/ids.rs` (`route_values` public), `crates/graph-compiler/src/lib.rs`
  (its re-export only).
- `crates/builtins-compiler/src/lib.rs` (only if #1651's export of the program and a strip's mute
  cells needs a change; after #1651).
- `crates/host-core/src/prepare.rs` (attach, seed, resolver, join), `crates/host-core/src/lib.rs`
  (re-exports only), `crates/host-core/tests/route_mute.rs`.
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `hosts/host-web/src/tests.rs`, `crates/control-plane/tests/follow_automation_realtime.rs` (new).
- No path under `crates/engine/src/realtime/`.

**For #1654 (phase 2, no scope here).** It reuses the open transform words, the flags word's spare
bits (6 and up), the per-event composition and the in-block retarget. It needs the seven-word
predicate widened to routes with automated rows, and carried cursor state by `(route, row)` for
curves that move on the grid; this slice's mute cursors are not carried.
