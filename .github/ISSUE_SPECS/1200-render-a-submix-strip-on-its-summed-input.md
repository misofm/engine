# Render a submix strip on its summed input

Slice 03 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A submix now renders its strip on the D9 sum of the routes that target it: input section (trim,
polarity, HPF, LPF), inserts, fader and pan or matrix, exactly as a track renders its strip on its
source. A drum bus can hold a glue-compressor insert and a fader, and its latency joins plugin-delay
compensation. A bus is never mono-collapsed.

A browser that boots a bus session with live controls keeps booting: until K2 files bus effects,
live controls and observation attach only to track-owned effects (DESIGN P16).

A submix's own `delay_samples` lands in the next slice, *Delay a submix strip's summed input* (#1201).

## Context (verified on `fe8ac679`; slices 01 and 02 changed the sites named "after")

- **After *Declare the submix strip in the session grammar and wire* (#1199):**
  - a submix carries `builtins`, `inserts`, `fader` and `pan|matrix`, validated and encoded;
  - `Submix::unity(id)` builds a transparent strip;
  - `strips()` still yields tracks only, so no compiler sees a bus strip yet;
  - the compiler still lowers each submix as a bare node (that slice's D7).
- **After *Iterate session strips, not tracks, wherever strip semantics apply* (#1198):** every
  strip-semantics site goes through `strips()`/`StripRef`, `StripKind` is exhaustive (no
  `#[non_exhaustive]`), compiler paths come from `path_prefix()` and the chain-edge paths from
  `collection_path()`, and `EffectPreparedEntry` carries `strip_path`.
- **How a submix lowers today.**
  - The graph compiler emits one zero-latency `GraphNodeId::Submix` node
    (`crates/graph-compiler/src/compile.rs:296-307`).
  - It runs as `NodeKind::Identity`, a D9 reduction (`crates/graph/src/runtime.rs:4025-4067`).
  - `route_source_node` maps `SubmixOutput`, and `route_destination_node` maps `SubmixInput`, to
    that node (`crates/graph-compiler/src/ids.rs:176-194`).
- **The runtime already reduces a source-less `Input` stage.** A `TrackStage::Input` node that is
  not a source input, not a bank member and not bound lowers to `NodeKind::Identity`, which reduces
  its graph inputs (`node_kind`, `runtime.rs:4025-4067`; `execute_op` `:2970-3060`).
- **A track's delay cannot serve a bus.** A track's `delay_samples` uses the `TrackDelay` arm, keyed
  by **source inputs** (`runtime.rs:4026-4041`) and returning **before** any reduction (`:2980-2989`).
  `track_delays` (`compile.rs:460-471`) stays track-only (slice 01 left it so, with a comment).
- **`required_bindings`** (`compile.rs:796-810`) requires a binding on every `TrackStage::Input`;
  that binding is the source.
- **Reductions in the canonical text.**
  - `reduction_records` (`ids.rs:77-105`) records only `Submix` and `Output` nodes with more than
    one main input.
  - They feed `GraphCompiler::reductions` (`compile.rs:65`) and the canonical `reduction` rows
    (`crates/graph-compiler/src/canonical.rs:252-260`).
  - The graph-compiler estimate counts reductions over `Submix | Output` too
    (`crates/graph-compiler/src/estimate.rs:42-51`).
  - Once `Submix` nodes stop being emitted, bus sums would **vanish** from the canonical text unless
    a submix strip's `Input` reduction is counted.
- **The mono collapse and its real guard.**
  - `arm_mono_collapse` (`runtime.rs:2326-2338`) arms a bank only when every lane's ID is in the
    host-supplied `eligible` set.
  - `gathers_track_input` (`:5208-5222`) is a pure node-shape test: slot 0 of every lane is a
    `PostInputBuiltins` stage. It **will** hold for a bus chain.
  - The `eligible` set is `session_structural_symmetry` over tracks only
    (`crates/builtins-compiler/src/lib.rs:3839-3862`), filtered at
    `crates/host-core/src/prepare.rs:1284-1289`. That set is the guard.
  - `SessionPoolClasses::from_session` (`builtins-compiler/src/lib.rs:3924`) seeds the pool classes
    from tracks.
- **Live controls attach to every prepared effect** (VERIFY-2 N2).
  - With any queue depth, host-core calls `attach_effect_live_controls`
    (`crates/effect-compiler/src/prepare.rs:1457-1534`, from `host-core/src/prepare.rs:887-892`),
    which gives every prepared entry a channel; with observation taps it calls
    `attach_effect_observation` (`effect-compiler/src/prepare.rs:1577-1641`, from
    `host-core/src/prepare.rs:900-910`).
  - host-web files every producer and observation handle by binary search over `handles.tracks` and
    refuses a miss (`web.live_controls.effects`, `hosts/host-web/src/lib.rs:5921-5927`;
    `web.live_controls.observation`, `:5990-5995`).
  - So once this slice prepares bus effects, any browser boot with live controls of a session whose
    bus carries an effect would fail until K2 widens host-web.
  - Lanes with and without a channel already share banks: entries carry `control: None` unless a
    session bypass is lowered (`effect-compiler/src/prepare.rs:605-606`), and an observation lane
    exists only beside a control lane (`runtime.rs:4051-4053`).
  - The producer's mixer boots with `live_control_command_queue_records = 64`; the SDK and
    `WebBootOptions::explicit_defaults()` default to 0, so no K1 SDK eval sees this.
- **A graph-level literal.** `fixtures/graph/v1/invalid-scc-diagnostics.json` is a hand-written
  graph-level fixture with `submix:` nodes, emitted by
  `crates/graph-compiler/src/bin/graph_fixture.rs:211-224` and pinned in
  `fixtures/graph/MANIFEST.tsv`. It is not compiled from a session. The graph crate keeps the
  `Submix` variant (`crates/graph/src/lib.rs:279`), so the literal stays valid.
- **Pinned shapes with a submix** (each verified; the re-pin list):
  - `compile_reverse_route_submix_fixture` (`crates/graph-compiler/src/lib.rs:2177-2223`): the
    expected schedule `:2416`, the identity hash `:2441`, `:2528`, `:2569`, `:2583`, the expected
    levels `:2455`, the legacy schedule swaps `:2490-2491`, the level mutations `:2531`, `:2541`,
    `:2551-2552`, the label `:4166`, and the old Kahn swaps `:15112-15113`;
  - `crates/graph-compiler/tests/compile_shapes.rs:493` (`3 * 256` builtin bank members, which
    becomes `3 * 288` once its 32 submixes are strips) and the other shapes built at `:389`, `:443`;
  - `crates/graph-compiler/tests/bank_levels.rs:727-735`;
  - `crates/host-core/tests/{collapse_arming,randomized}.rs`, if they pin a digest.
- **Today's probe** (`DESIGN.md` 3.2) rendered bare-submix sessions correctly on four paths, and
  PDC was exact through them.

## Decisions frozen for this slice

- **D0. Strips include submixes.** `StripKind::Submix(&Submix)` is added (`crates/session`).
  `strips()` yields tracks, then submixes, each in model order (canonical ID order on a normalized
  model), and `path_prefix()` returns `$.submixes[id=<id>]` and `collection_path()` `"$.submixes"`
  for a submix. Through slice 01's sites, the session estimate (`crates/session/src/estimate.rs:64-99`,
  `:131-160`) and host-core's `count_effects` (`host-core/src/prepare.rs:1313-1324`) now charge bus
  inserts and parameters, once (slice 02 adds no direct submix charge; inside K1 only, bus inserts
  are uncharged between 02 and 03). This lands with the lowering so that no compiler sees a bus strip it
  cannot lower.
- **D1. Lowering.** A submix lowers through `strips()` and `lower_strip` to the same `TrackStage`
  nodes and racks a track does, keyed by the submix's ID. Tracks, submixes and outputs share one ID
  namespace (`crates/session/src/validate.rs:69-100`), so the key is unambiguous.
  - The submix's `Input` stage has no source binding. Its inputs are the `RouteDestination` edges
    that name it, summed in D9 edge order (route-ID order). Fan-in 0 is a `+0.0` fill.
  - `route_destination_node` maps `SubmixInput` to that `Input` stage.
  - `route_source_node` maps `SubmixOutput` to the strip's `PostMatrix`, the end of the strip.
    *Tap a submix strip at any of the seven send points* (#1203) adds the other taps.
  - `GraphNodeId::Submix` is no longer emitted. The graph crate keeps the variant.
- **D2. `required_bindings`** requires the source binding only on a **track's** `Input`.
- **D3. Reduction records.** `reduction_records` and the graph-compiler estimate count a submix
  strip's `Input` reduction exactly where they counted the `Submix` node: one record when it has more
  than one main input.
- **D4. Never collapsed.**
  - `SessionPoolClasses::from_session` seeds every submix as `Stereo`, explicitly.
  - `session_structural_symmetry` stays tracks-only. That eligibility set is the guard, and a
    submix ID must never enter it.
  - A submix strip is the same dual-mono strip as a track: independent L/R state and parameters,
    with coupling only through a declared `link_mode` or the 2x2.
- **D5. Interim: no bus delay.** A submix's `delay_samples` is not lowered in this slice: no delay
  entry is emitted for it. *Delay a submix strip's summed input* adds its arm. This holds inside
  batch K1 only, which is never pushed before slice 08 (the slice-02 D7 precedent).
- **D6. The K1 live-control interim (DESIGN P16).** In `crates/effect-compiler/src/prepare.rs`, a
  private `fn track_owned(session: &CompiledSession, owner: &str) -> bool` answers whether `owner`
  is a track of `session.normalized_model()` (a binary search of its sorted `tracks` by ID).
  `attach_effect_live_controls` and `attach_effect_observation` skip every entry that is not
  track-owned: no channel, no producer, no observation lane, no handle. A skipped entry renders
  through its live-control-free path. Both functions' doc comments say: "K1 interim (decision 13,
  P16): bus effects get no live channel until *List every strip in the live-control handles and
  file bus effects in the browser* (#1207) files them in the browser and removes this rule."
- **D7. The graph-level literal stays.** Add one comment line in `graph_fixture.rs` beside the
  literal: it is a graph-level fixture, `Submix` remains a graph-crate node, and no session compiles
  to it.

## Deliverables

0. **Session:** D0.
1. **Graph compiler:** D1, D2, D3 and D5.
2. **Builtins compiler:** submix strips get prepared builtins, tails, seals and bank membership
   through the slice-01 sites; D4. The seal's strip list is tracks then submixes, not one sorted
   list: keep every comparison sequence-to-sequence, as today.
3. **Effect compiler:** submix inserts are prepared through the strip loop. The effect index key
   `(strip_id, rack, effect_id)` keeps its shape, and `declared_live_addresses` (`:1650`) covers bus
   inserts. D6.
4. **Re-pins.** Re-pin, each with its reason, every canonical graph text and identity hash of a
   session that contains a submix (the sites listed in Context). Regenerate the graph fixtures only
   if one contains a submix (`cargo run --locked -p graph-compiler --bin graph_fixture -- --write`;
   none is expected). The reason is: "submix lowers to a strip; `submix:` nodes become
   `track:<id>:<stage>` chains; audio moves only where the identity input section turns `-0.0` into
   `+0.0`, or where D7 sanitizes a non-finite or `>= 1e30` value to `+0.0`".
5. **D7.**
6. **Docs.** `docs/SESSION_SCHEMA_V1.md` states that a submix sums its routes in route-ID order and
   then runs its strip. The stale "submixes and outputs carry no effect racks" text is not this
   slice's: *Meter any boundary of a submix strip and designate a master strip in host-core* (#1208) owns
   the host-core doc (`crates/host-core/src/prepare.rs:305-309`), and *Address submix strips in
   browser live commands* (#1213) owns the browser docs.

## Authorized paths

- `crates/session/src/{model.rs,lib.rs,estimate.rs}` (D0 only)
- `crates/graph-compiler/src/**` and `crates/graph-compiler/tests/**`
- `crates/builtins-compiler/src/lib.rs`
- `crates/effect-compiler/src/prepare.rs`
- `crates/host-core/src/prepare.rs` (only if a `count_effects` or source-semantics comment must
  move)
- `crates/host-core/tests/` (the new `submix_strip.rs`, plus re-pins in `collapse_arming.rs` and
  `randomized.rs`)
- `hosts/host-web/src/tests.rs` (gate 6's test only)
- `fixtures/graph/v1/**` and `fixtures/graph/MANIFEST.tsv` (only if regenerated)
- `docs/SESSION_SCHEMA_V1.md`
- this spec

## Non-goals

- No submix `delay_samples` (next slice).
- No console entries on submixes (*Carry every console slot on every submix strip*, #1202).
- No submix tap other than the end of the strip.
- No host-core caps, live controls or meters for submix strips (batch K2). No host-web code change.
- No SDK change and no route field.

## Hazards

- **The collapse.** `gathers_track_input` holds for a bus chain, so the only guard is the eligibility
  set. Code that seeds a pool class or a witness from `strips()` instead of tracks would arm a bus's
  collapse and copy its left lane over its right. Gate 4 and its recorded mutation pin this.
- **`-0.0`.** The identity input section normalizes `-0.0` to `+0.0` (the disabled-SVF identity,
  `docs/BUILTINS_AND_METERING_V1.md`). A session whose bus was bare and is now a transparent strip
  differs only there, and where D7 sanitizes. State that in each re-pin's reason.
- **Order.** Bank lane order and every digest depend on `strips()` yielding submixes after tracks,
  sorted by ID within each segment. The concatenation is not sorted; never binary-search it.
- **Mixed banks.** A builtins bank may hold lanes with and without live-control consumers.
  `StripPreparation.control` is per strip (`builtins-compiler/src/lib.rs:346`). Keep that working.
- **D6 is a removal-scheduled rule.** It must not leak into the graph or host-core: the filter lives
  only in the two attach functions, so the slice that removes it touches only them.

## Objective gates

New test file `crates/host-core/tests/submix_strip.rs`. host-core is in the
`run-aarch64-tests.sh` package list, so these tests also run 4-lane on arm64.

1. **A bus equals a track fed its sum.**
   - Session A: three tracks with transparent strips and sources of random nonzero finite samples
     send through routes with random gains and matrices into a submix with strip S. S routes at unity
     to the output.
   - Session B: one track with the same strip S. Its stereo source is the `f32` sum of the three
     routed contributions, computed in the test by the D3 expression (`l' = (lr * r) + (ll * l)`, two
     roundings) and summed left to right in route-ID order.
   - Strip S is drawn at random: trim, polarity, HPF, LPF, a compressor insert with
     `link_mode: maximum`, an EQ insert, a fader and a pan. `delay_samples` is 0. The console section
     is empty.
   - Over 32 seeds, with the rate drawn per seed from the launch rates at quantum 128 as
     `crates/host-core/tests/randomized.rs` does (`:57`, `:328`), the rendered outputs are
     bit-identical, with NaNs folded.
   - For a one-bus session with one insert, the session estimate's insert charge equals a hand count
     that charges that insert once.

   *Test value: it turns red if a bus's builtins, inserts, fader or pan are bound to the wrong lane,
   run in the wrong order or are skipped, or if the bus sums in an order other than route ID, or if a bus insert is charged twice in the
   session estimate. No existing test renders a processed bus.*
2. **PDC through a bus.**
   - The bus has a true-peak limiter insert: 486 samples at 48 kHz.
   - One contributor is also routed directly to the output.
   - With an impulse, the compiled plan's inserted delay on the direct edge is exactly 486, and both
     planes peak at sample 486.

   *Test value: it turns red if bus-strip effect latency is left out of the arrival times. Today's
   buses have no latency, so no test can see it.*
3. **A bus is never collapsed.**
   - A mono-source track (both lanes mapped to source channel 0), panned hard left, feeds a bus with
     a `dual_mono` compressor insert.
   - The bus's right output is exactly `+0.0` while its left is not.
   - The PR records one mutation run: adding submix IDs to the host's eligible set turns this test
     red.

   *Test value: it turns red if a bus enters the collapse eligibility set and copies its left lane
   to its right. No test routes a mono track into a processed bus.*
4. **Bus sums stay in the canonical text.** In a graph-compiler test, a session with a three-input
   bus has exactly one `reduction` row for the bus's `Input` stage in its canonical text and in
   `GraphCompiler::reductions`.
   *Test value: it turns red if `reduction_records` still keys on `Submix` nodes, which silently
   drops every bus sum from the sealed text.*
5. **No bus effect gets a live channel in K1** (D6).
   - In `submix_strip.rs`, session A of gate 1 is prepared with
     `prepare_host_session_with_live_controls` at `control_queue_depth: Some(64)` and nonzero
     `observation_taps` (with a meter period, as the existing observation tests set it).
   - Every `handles.effect_controls[i].track_id` and `handles.effect_observations[i].track_id` is a
     track ID; the track-owned producers are all present.

   *Test value: it turns red if the attach functions give a bus effect a producer or an observation
   handle, which host-web would refuse at boot.*
6. **A live-controlled browser boot of a bus session renders** (VERIFY-2 N2). A native test in
   `hosts/host-web/src/tests.rs`:
   - the session is `fixtures/session/v1/observation-frame-shape.json` with one submix added whose
     strip carries a `miso.compressor` insert, one track routed into it and the submix routed at
     unity to the output (built through the session model and written as canonical JSON, as
     `same_track_observation_host` builds its model);
   - it boots with `live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS` (64),
     `live_control_observation_taps: 4` and a meter period, as `observation_host` does;
   - boot succeeds and 8 blocks render with `RESULT_OK`.

   *Test value: it turns red if the K1 push would break the producer's mixer on any bus session
   with an effect, because host-web refuses a producer or observation handle it cannot file. No K1
   SDK eval boots with live controls.*
7. **Unchanged where no submix exists.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `bash scripts/trace-graph-audit.sh target/release/audit`

   Every digest of a session without submixes is unchanged.
8. **Render allocates nothing.** After warm-up, `allocations == 0` and `frees == 0` around every
   render call for session A of gate 1, measured with `bench_support::alloc`'s thread-scoped
   counters.
   *Test value: it turns red if a bus strip's lowering leaves an allocating path on the render
   thread (for example a per-block buffer for the bus reduction or its rack), which no existing
   allocation test can reach because none renders a processed bus.*
9. **Workspace, 4-lane and policy.**
   - the test-debug-a workspace command (DESIGN section 7);
   - `cargo test --locked --release -p audit -p bench -p console-workload`;
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
     batch push (recorded as "at batch push" if no arm64 host is available);
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `for x in graph builtins realtime host-core workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`

## Evidence

- The re-pin list with reasons.
- The gate-3 mutation run's output.
- The gate-1 seeds, and the widths run: native 8-lane AVX2, plus 4-lane on arm64 or "at batch push".
- The `bank_route_folds()` counts of gate 1's and gate 2's sessions on this branch, and of the same
  bus shapes on the base commit (bare submixes), so a lost bus fold on a real path is visible.

## Dependencies

- *Declare the submix strip in the session grammar and wire* (#1199)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded to one value (decision 10).
- Render stays allocation-, lock- and syscall-free (`scripts/check-realtime-policy.sh`).
- One implementation shape for every target. Only the lane width differs, and it is keyed on target
  features, never on `target_arch`.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K1 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
