# Give every route whose tap precedes its strip's fader a live lane on every plan

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every route whose source tap precedes its strip's fader (`input`, `post_input`, `insert_send`,
`insert_return`, `pre_fader`) has a live lane on every plan, on the C ABI and in the browser,
whatever its destination. A fader mute does not reach such a route, so a strip that fades out or
ducks (*Remove a strip in two phases: ramp out, then a scheduled swap*, #1325; *Duck-swap a strip
whose state cannot continue across a plan swap*, #1324) needs to ramp these routes on the running
plan; today a pre-fader route into the output has no lane and would step. A host can also change
such a route's `gain_db`, `mute` and `channel_matrix` live, like a send. Routes from `post_fader`
and `post_pan` into the output keep their prepared constants and their master fold.

## Scope ruling (stated, accepted by root)

Decision 13 keeps routes into the output prepared so that the master fold survives in live plans
(`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, deferred item O9, "Live output routes",
`docs/handoffs/submix-sends-2026-10-02/DESIGN.md:1311-1312`; decision 14 row,
`docs/rulings/live-update-versus-rebuild-2026-10-04.md:158`). Decision 15, D15-9, narrows O9 for
routes whose tap precedes the fader only (record lines 216-221 and 442-444): every such route gets a
live lane; every other route into the output keeps the fold. The narrowing costs no fold: the fold
absorbs only a route whose producer is its chain's last slot, the post-matrix stage
(`foldable_lane`, `crates/graph/src/runtime.rs:7135-7176`; `served_by_the_resident_lane`, `:7075`),
and a master with any contributor that is not a folded lane declines its whole fold (association
proof, doc at `:7202-7203`). A pre-fader route into the output is such a contributor, so on `main`
its master already does not fold.

## Context

- Taps: `SendTap` (`crates/session/src/model.rs:856-871`). `graph_compiler::ids::stage`
  (`crates/graph-compiler/src/ids.rs:210-218`) maps the five pre-fader taps to `TrackStage::Input`,
  `PostInputBuiltins`, `PostSimd1`, `PostDynamic` and `PostSimd2PreFader`, all below
  `TrackStage::PostFader` (`crates/graph/src/lib.rs:257-265`, `Ord`). `route_source_node`
  (`ids.rs:187-193`) makes a route's `RouteSource` edge leave `TrackStage { stage }` of its strip,
  for a track and a submix alike.
- Lane selection today: `PreparedGraphPlan::attach_route_controls`
  (`crates/graph/src/lib.rs:1687-1771`) picks the routes whose `RouteDestination` edge lands on a
  submix input (`:1706-1720`), in canonical route-ID order. Wrapper:
  `attach_route_live_controls` (`crates/builtins-compiler/src/lib.rs:2972-2977`). host-core calls
  it only when `HostLiveLanes::routes` is set and a control channel exists
  (`crates/host-core/src/prepare.rs:1574-1590`; `HostLiveLanes`, `:370-379`).
  *Hold route-lane values in latest-target cells* (#1347) removes the `depth` argument and makes
  the lanes cells.
- The model's view of the same set: `LiveRouteState::live_routes`
  (`crates/host-core/src/live_route_state.rs:78-83`), routes into a submix. The browser checks by
  ID that it equals the producer list (`hosts/host-web/src/lib.rs:6932-6943`,
  `web.live_controls.routes`), and addresses a send command by its live-route index (`:3063`).
- Every plan has a control channel: the C ABI always (`crates/capi/src/runtime/compile.rs:572-600`);
  the browser after *Give every browser plan live strip fader and mute lanes* (#1326), which keeps
  `routes` off when the app's live-command option is 0 (#1326 D3).
- C ABI: *Deliver value-only send edits to the running C ABI plan* (#1225) sets `routes: true`,
  keeps the producers by route ID, and makes a send's values live while every field of a route into
  the output stays structural (its D1, citing O9).
- Existing guards: `live_controls_cost_the_standing_sessions_no_fold`
  (`crates/host-core/tests/live_routes.rs:698`; the console fixture's routes are all `post_pan`
  into the output) and `live_send_handles_are_in_canonical_route_order` (`:750`).

## Decisions frozen for this slice

- **D1. One rule for "has a live lane".** A route has a live lane when (a) its tap precedes the
  fader, on every plan with a control channel, or (b) its destination is a submix and the plan's
  `HostLiveLanes::routes` is set. Graph form: `attach_route_controls` takes
  `into_submix: bool` and selects routes whose `RouteSource` edge leaves a `TrackStage` below
  `PostFader`, plus, when `into_submix`, routes into a submix; canonical route-ID order, deduped.
  Model form: `LiveRouteState::live_routes(model, into_submix: bool)` with the same rule on
  `RouteSource`'s tap. host-core passes `lanes.routes` to both.
- **D2. Attach whenever a control channel exists.** host-core calls the attach whenever
  `control_queue_depth` is `Some` (always, on both hosts, after #1326), not only when
  `lanes.routes` is set. `HostLiveLanes::routes`'s doc says it selects the routes into a submix;
  pre-fader routes are always live.
- **D3. Classifier.** #1225 D1's live route set grows to D1's rule: a value-only change
  (`gain_db`, `mute`, `channel_matrix`) of a route whose tap precedes the fader is live whatever
  its destination. Records, ramps and commit order are #1225's (D3-D6), unchanged. A route into
  the output keeps `follows_mute = false` (session rule), so #1226 is unaffected. Every field of a
  `post_fader` or `post_pan` route into the output stays structural (O9 as narrowed).
- **D4. Browser.** host-web passes `lanes.routes` to `live_routes` so its ID check holds in both
  option modes. With option 0, commands stay refused (#1326 D4) and the lanes serve the engine's
  transitions. With a nonzero option, the existing send kinds address a pre-fader route into the
  output by its live-route index, like any send.
- **D5. Resources.** The new lanes are charged by the existing route-lane rows
  (`graph::route_control_resources`); the PR lists any re-pinned `expected.json` row with this
  reason.
- **D6. Realtime and the acked-batch question.** No render-path change beyond a route op bound in
  its live form. Writes are #1347's infallible cell writes after every check (#1225 D6): no ack
  precedes a drop.

## Deliverables

1. D1 in `crates/graph/src/lib.rs`, `crates/builtins-compiler/src/lib.rs` and
   `crates/host-core/src/live_route_state.rs`; D2 in `crates/host-core/src/prepare.rs`.
2. D3 in `crates/host-core/src/live_delta.rs`; D4 in `hosts/host-web/src/lib.rs`.
3. Tests below; `docs/C_ABI_V1_QUALIFICATION.md` states which routes are live.

## Authorized paths

- `crates/graph/src/lib.rs` (route-lane selection only), `crates/builtins-compiler/src/lib.rs`
  (the wrapper's signature)
- `crates/host-core/src/prepare.rs` (attach call, `HostLiveLanes` doc),
  `crates/host-core/src/live_route_state.rs`, `crates/host-core/src/live_delta.rs` (route set only)
- `crates/host-core/tests/live_routes.rs`, `crates/host-core/tests/live_delta.rs`,
  `crates/graph-compiler/tests/live_routes.rs` (call sites)
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/browser-v1/expected.json` (re-pins with reasons)
- `crates/capi/src/runtime/live_tests.rs` (or its successor location after #1309)
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No lane for `post_fader` or `post_pan` routes into the output; O9 stands for them.
- No change to the decision 13 or 14 records (decision 15's record carries the narrowing).
- No SDK change (stream H owns `sdk/`). No transition logic (#1324, #1325, #1363).
- No new C symbol, opcode, field or feature bit.

## Objective gates

1. **Exact lane set (host-core).** Extend `live_send_handles_are_in_canonical_route_order`: the
   `out_of_order` session gains `t1-direct`, an `input`-tap route into the output, and `t2-pf`, a
   `pre_fader` route into the output. With `lanes.routes` set the producer IDs are
   `["aa-send", "mm-send", "t1-direct", "t2-pf", "zz-send"]`; with `HostLiveLanes::FADER_AND_MATRIX`
   they are `["t1-direct", "t2-pf"]`; `t0-main` (`post_pan`) never has one. Each producer drives
   its own route's index, as today.
2. **Fold kept.** `live_controls_cost_the_standing_sessions_no_fold` stays green unchanged.
3. **Settled lane equals the constant.** Extend `the_live_target_is_the_prepared_constant`
   (`live_routes.rs:425`) with a `pre_fader` route into the output: a plan with the lane and no
   record renders bit-identically to the same route bound as a prepared constant.
4. **Live on the C ABI.** Beside #1225's gate 1: `0x0505`, `0x0506` and `0x0504` on a `pre_fader`
   route into the output keep the plan (no new epoch, no pending candidate). From `latency_samples`
   plus the ramp plus the route's compensation delay the output equals a fresh plan of the
   committed model, bit for bit. `0x0505` on a `post_pan` route into the output still rebuilds.
5. **Browser.** A host-web test with option 64: a send-gain command on the `pre_fader` route into
   the output is accepted and the settled output equals a fresh plan; with option 0 the producer
   exists and the command is refused with `COMMAND_REASON_UNSUPPORTED_KIND`.
6. **Realtime.** Extend `live_sends_render_without_allocating` (`live_routes.rs:932`) with the
   gate 1 routes: zero allocations and frees.
7. Commands:
   - `cargo test --locked -p graph -p graph-compiler -p host-core -p capi -p host-web --features graph/test-support,builtins-compiler/test-support,host-core/test-support,host-web/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   - `bash scripts/check-graph-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-capi-abi.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a selection keyed on destination only (no pre-fader output lane, so #1325's route duck
  has nothing to write), one that also gives `post_pan` output routes a lane (the fold lost), or a
  graph and model rule that disagree, turns it red.
- Gate 3: a live form whose settled arithmetic differs from the prepared op (a bit moves on every
  plan with such a route) turns it red.
- Gate 4: a classifier still keyed on "into a submix" (the edit rebuilds) turns it red, and one
  that admits every output route turns its last clause red.
- Gate 5: a browser ID check that ignores option 0 (`web.live_controls.routes` at boot) or a
  command gate keyed on the lanes turns it red.

## Dependencies

- *Hold route-lane values in latest-target cells* (#1347).
- *Deliver value-only send edits to the running C ABI plan* (#1225).
- *Give every browser plan live strip fader and mute lanes* (#1326).
- Followed by *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) and
  *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
