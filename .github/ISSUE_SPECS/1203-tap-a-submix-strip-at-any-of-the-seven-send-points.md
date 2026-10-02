# Tap a submix strip at any of the seven send points

Slice 06 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A route or a sidechain can leave a submix strip at any of the seven taps a track offers: `input`,
`post_input`, `insert_send`, `insert_return`, `pre_fader`, `post_fader` and `post_pan`. For example:

- a drum bus can feed a parallel-compression bus from `pre_fader`;
- a vocal bus's `pre_fader` can key a ducking compressor;
- a bus can send post-fader to a delay return.

## Context (verified on `fe8ac679`; slices 02-05 changed the sites named "after")

- **Model.** `RouteSource::{Track { track_id, tap }, SubmixOutput { submix_id }}`
  (`crates/session/src/model.rs:605-619`). A submix source has no tap. `Sidechain.source` is a
  `RouteSource` too (`model.rs:527-532`).
- **JSON parse.** `parse_route_source` (`crates/session/src/parse.rs:1317-1360`) accepts `track` or
  `submix_output`, and refuses track fields on `submix_output` (`:1336-1347`); any other kind is
  `schema.invalid_enum` (`:1349-1357`). Routes and sidechains both call it (`parse.rs:1304`, `:1184`).
  Validation is `validate_route_source` (`validate.rs:494-522`), at index paths such as
  `$.routes[<i>].source`.
- **Wire.** `schema::session::route_source` (`crates/protocol/src/schema.rs:826-843`) has `TAG` 1,
  `ID` 2 and `TAP` 3. `TRACK` is `[TAG, ID, TAP]`, `SUBMIX` is `[TAG, ID]`, and `KNOWN` is
  `[TAG, ID, TAP]`.
  - `KNOWN` is the field spec of the route's source (`:1041`), the sidechain source (`:856`) and
    `set_route_source::VALUE` (`:1412`); the decoder picks `TRACK` or `SUBMIX` by tag
    (`session_wire.rs:1522-1529`).
  - Decode: `parse_route_source` (`crates/protocol/src/session_wire.rs:1521-1543`), shared by route
    (`:1683`), sidechain (`:1568`) and `SetRouteSource` (`:1309`). Taps are 1-7 (`parse_tap`,
    `:1787`).
  - Encode: `tx_route_source` (`:754`), used at `:564` (`SetRouteSource`), `:798` (sidechain) and
    `:1013` (route).
  - JSON `submix_output` maps to tag 2 in the canonical walk (`crates/session/src/visit.rs:258`).
- **Graph mapping.** `route_source_node` (`crates/graph-compiler/src/ids.rs:176-183`). After
  *Render a submix strip on its summed input* (#1200) it maps a submix source to the strip's `PostMatrix`
  stage. `stage(SendTap)` (`ids.rs:199-209`) maps each tap to its `TrackStage`. Sidechain edges are
  built at `crates/graph-compiler/src/compile.rs:352-381`; cycles are detected by the graph
  compiler's topological check (`crates/graph-compiler/src/schedule.rs:9`, witnesses `:83-89`).
- **Who names `SubmixOutput`** (from `git grep`):
  - graph-compiler:
    - `crates/graph-compiler/src/ids.rs:179`;
    - `src/lib.rs:432`, `:2203`, `:2208`;
    - `tests/bank_levels.rs:732`, `tests/compile_shapes.rs:445`.
  - host-core: `crates/host-core/tests/collapse_arming.rs:218`, `randomized.rs:379`.
  - protocol: `crates/protocol/src/session_wire.rs:762`, `:1538`; `session_wire/tests.rs:764`, `:1283`.
  - session source:
    - `crates/session/src/canonical.rs:319`, `model.rs:615`, `parse.rs:1345`;
    - `validate.rs:508`, `visit.rs:258`.
  - session tests:
    - `crates/session/tests/canonical_schema.rs:196`, `:252`;
    - `diagnostic_parity.rs:397`, `:408`, `:489`, `:501`;
    - `invalid_matrix.rs:597`, `:609`, `:690`, `:697`, `:704`.
  - plus every site slices 03-05 added (`crates/host-core/tests/submix_strip.rs`, the K1 boot test in
    `hosts/host-web/src/tests.rs`, graph-compiler tests); re-run `git grep -n SubmixOutput` at the K1
    branch head before editing.
- **JSON that spells `submix_output`:**
  - `.claude/skills/author-session/worked-session.json:289-290`;
  - the writer corpus (`fixtures/session-canonical/v1/canonical-writer-corpus.json`, regenerated
    from `canonical.rs`);
  - `docs/session-v1.schema.json:97`;
  - the prose at `docs/SESSION_SCHEMA_V1.md:213`.
- **SDK spellings that stay red until** *Build submix strips and bus taps in the SDK and teach agents
  to author them* (#1205, inside the batch; not this slice's gates; VERIFY-2 MINOR 13):
  `sdk/src/core/types.ts:197`; `sdk/src/core/session.ts:567-571`, `:592`;
  `sdk/src/internal/session-json.ts:100-103`; `sdk/src/cli/session-request.ts:115-131`;
  `sdk/test/console-evals.mjs:653` (in `rebuild()`, `:586-662`); `sdk/test/builder-evals.mjs:959`;
  `sdk/test/enginectl-cli.mjs:66`, `:244`, `:466-468`.
- **Wire pins.** The all-opcode corpus is codec data (`crates/conformance/src/protocol_corpus.rs:23-24`).
  It encodes `UpsertRoute` and `SetRouteSource` from `canonical.json`'s first route
  (`protocol_corpus.rs:30`, `:193-202`), whose source is a **track**. So changing tag 2's field list
  alone would not move `COMPLETE_SCHEMA_HASH`, and the wasm parity gate would never see a tapped
  submix source (VERIFY-2 MINOR 13). The hash is spelled at `protocol_corpus.rs:666`,
  `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and
  `fuzz/corpus/complete-schema-manifest.md:10`.

## Decisions frozen for this slice

- **D1. JSON.** A submix source is `{ "kind": "submix", "submix_id": <id>, "tap": <tap> }`, for
  routes and sidechains alike.
  - `submix_output` becomes an unknown token. It refuses with `schema.invalid_enum` at its index
    path (`$.routes[<i>].source.kind`, or the sidechain's source path), as the decision-12 retired
    tap spellings do.
  - The model variant is `RouteSource::Submix { submix_id, tap }`.
- **D2. Wire.** Tag 2 is kept, and `TAP` (field 3) becomes required for it: `SUBMIX` is
  `[TAG, ID, TAP]`. A tag-2 source without a tap refuses as a missing required field, whether it is
  a route source, a sidechain source or a `SetRouteSource` value. Tap codes stay 1-7.
- **D3. Graph.** `route_source_node` maps `{ submix, tap }` to `TrackStage { submix_id, stage(tap) }`
  for routes and sidechain edges. An old `submix_output` source becomes
  `{ kind: "submix", tap: "post_pan" }`, which is the same node.
- **D4. A bus tap reads what a track tap reads.** Pre-fader taps are un-gated: the fader mute applies
  at the fader.
- **D5. The corpus covers the tap.** The all-opcode corpus's `SetRouteSource` value becomes
  `RouteSource::Submix { submix_id: "drums", tap: PreFader }` (a codec value; the corpus is not a
  session a store applies), so `COMPLETE_SCHEMA_HASH` and the wasm parity gate cover tag 2's tap.

## Deliverables

1. **Session:** model, parser, canonical writer, field keys, and validation. The role check in
   `validate_route_source` is unchanged in substance.
2. **Protocol codec:** D2 and D5. Re-pin `COMPLETE_SCHEMA_HASH` at its four sites, with one sentence
   of reason. Pin the refusals in the existing refusal suites (the all-opcode corpus holds one
   successful edit per opcode and takes no refusal):
   - the retired spelling, for a route and a sidechain source, in
     `crates/session/tests/invalid_matrix.rs`;
   - a tapless tag-2 route source, sidechain source and `SetRouteSource` value, in
     `crates/protocol/src/session_wire/tests.rs`.
3. **Graph compiler:** D3, for routes and sidechains.
4. **Migration.**
   - Every Rust site in the Context moves to `RouteSource::Submix { tap: PostPan }`, or another tap
     where a test means one.
   - Every JSON document moves to `submix` with `tap: "post_pan"`. Regenerate the writer corpus
     (`MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session canonical_writer_corpus_is_rust_generated_and_current`).
   - Re-pin anything this moves, with its reason. None is expected beyond the K1 batch's earlier
     re-pins, because `post_pan` is the old node.
5. **Docs.**
   - `docs/SESSION_SCHEMA_V1.md`, the routes paragraph (`:213-214`): the source shapes, the seven
     taps on every strip, and that sidechains share the shape.
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: the route source in the nested registry (`:76`) and the
     retired spelling.
   - `docs/session-v1.schema.json` (`:97`).

## Authorized paths

- `crates/session/**`
- `crates/protocol/src/{schema.rs,session_wire.rs,session_wire/tests.rs}`
- `crates/conformance/src/protocol_corpus.rs`
- `scripts/check-protocol-wasm-parity.sh`, `fuzz/corpus/complete-schema-manifest.md` and
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md` (the hash only)
- `crates/graph-compiler/**`
- `crates/host-core/tests/{collapse_arming,randomized,submix_strip}.rs`
- `hosts/host-web/src/tests.rs` (the K1 boot test's route source only)
- `fixtures/session-canonical/v1/canonical-writer-corpus.json` (regenerated)
- `.claude/skills/author-session/worked-session.json`
- `docs/SESSION_SCHEMA_V1.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/session-v1.schema.json`
- this spec

## Non-goals

- No SDK change (*Build submix strips and bus taps in the SDK and teach agents to author them*).
- No route mute.
- No new tap.

## Hazards

- **Sidechains share the spec.** Making `TAP` required on tag 2 changes the sidechain source and
  `SetRouteSource` too. A codec change that special-cases routes would leave a tapless sidechain
  decodable. Gate 4 pins all three.
- **Cycles through taps.** Bus A's `pre_fader` into B, and B into A's input, is a cycle even though
  no route leaves A's end. The generic `graph.cycle` check must see the tap edge. Gate 5 pins it.

## Objective gates

1. **Seven taps, bit-identical to a track** (`crates/host-core/tests/submix_strip.rs`).
   - For each tap T, session A routes bus tap T to the output.
   - It renders bit-identically to the same route from tap T of a track that has the bus's strip and
     is fed the bus's sum, built as in gate 1 of *Render a submix strip on its summed input*.
   - The strip has a trim, an insert compressor, a console EQ, a fader of -6 dB and a swap matrix,
     so that every tap differs.
   - The contributor tracks carry every console entry with `bypass: true` (the EQ slot is
     latency-free, so no shift is needed).

   *Test value: it turns red if any tap maps to the wrong stage of a submix strip. No existing test
   taps a processed bus.*
2. **A sidechain keyed by a bus.** A compressor insert on track X is keyed from bus B's `pre_fader`.
   Its output is bit-identical to the same session in which B is replaced by a track fed B's sum.
   *Test value: it turns red if a sidechain from a bus tap reads the wrong stage, or the bus's end as
   before.*
3. **PDC by tap.**
   - The bus has a `post_insert` limiter: 486 samples at 48 kHz.
   - One route leaves the bus at `insert_return`, before the limiter, and one at `post_pan`, both to
     the output.
   - The `insert_return` route's edge gets exactly 486 samples of compensation, and the impulse
     aligns.

   *Test value: it turns red if arrival times use the strip's end for every tap.*
4. **Grammar and wire.**
   - `submix_output` refuses with `schema.invalid_enum`, for a route source and for a sidechain
     source.
   - A `submix` source without `tap`, or with an out-of-range tap, refuses at its index path.
   - A wire tag-2 route source, sidechain source or `SetRouteSource` value without `TAP` refuses.
   - These cases live in `crates/session/tests/invalid_matrix.rs` and
     `crates/protocol/src/session_wire/tests.rs` (deliverable 2).

   *Test value: it turns red if the retired spelling is silently read as `post_pan`, or the wire
   accepts a tapless submix source on any of the three shapes that share it.*
5. **A cycle through a tap.** Bus A's `pre_fader` into B, and B's `post_pan` into A, refuses with
   `graph.cycle`, naming the route.
   *Test value: it turns red if tap edges are left out of cycle detection.*
6. **Render allocates nothing.** `allocations == 0` and `frees == 0` around every render call for
   gate 1's sessions after warm-up, measured with `bench_support::alloc`'s thread-scoped counters.
   *Test value: it turns red if a mid-strip bus tap gets a dedicated buffer or copy allocated on the
   render thread, which no existing allocation test reaches because none taps a bus mid-strip.*
7. **Wire, workspace, 4-lane and policy.**
   - the test-debug-b command and `bash scripts/check-protocol-wasm-parity.sh` (with the re-pinned
     hash, now covering a tapped submix source);
   - the test-debug-a workspace command;
   - `bash scripts/run-aarch64-tests.sh debug`, or CI's `aarch64-debug` at the batch push;
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `bash scripts/check-graph-determinism.sh`,
     `cargo run --locked -p graph-compiler --bin graph_fixture -- --check` and
     `bash scripts/check-console-fixtures.sh target/release/session_validator`;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `for x in session protocol-control graph workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`

## Evidence

- The migration list.
- The re-pinned hash, with its reason ("tag-2 route source carries a required tap; the corpus's
  `SetRouteSource` value is a tapped submix source").
- Any other re-pin, with its reason.

### Attempt 1 record (Terra)

- **Sites.** Session: `RouteSource::Submix { submix_id, tap }` (`model.rs`); `parse_route_source`
  accepts `submix` with a required `tap` (closed token, `schema.invalid_enum`) and refuses
  `track_id` on it; `submix_output` falls to the unknown-kind arm ("expected track or submix");
  the visit walk writes `kind, submix_id, tap` (wire tag 2, field keys unchanged: `TAP` is 3);
  `validate_route_source` unchanged in substance. Protocol: `route_source::SUBMIX = [TAG, ID, TAP]`;
  `tx_route_source` emits the tap; the decoder reads it with `one_spec!` (missing -> `InvalidTlv`).
  Graph compiler: `route_source_node` maps `{ submix, tap }` to `track_node(id, stage(tap))` for
  routes and sidechain edges alike (one call site each, unchanged). Docs: `SESSION_SCHEMA_V1.md`
  routes paragraph, the registry's nested route-source row, `session-v1.schema.json`.
- **Re-pin.** `COMPLETE_SCHEMA_HASH` `0xc0f6_eced_bf50_920a` -> `0xa1dc_c56f_2e4a_48f9`, reason "a
  tag-2 route source carries a required tap; the corpus's `SetRouteSource` value is a tapped
  submix source" (`RouteSource::Submix { drums, PreFader }`, D5), at its four sites
  (`protocol_corpus.rs` with one doc sentence, `check-protocol-wasm-parity.sh` x2,
  `CONTROL_PROTOCOL_CONFORMANCE.md:3`, the fuzz manifest). No other re-pin: the writer corpus
  regenerated to a one-line diff (the full-surface `mix-to-main` source becomes
  `{ kind: submix, submix_id: mix, tap: post_pan }`); `worked-session.json`'s `band-main` moved the
  same way, and `session_validator validate --canonical` reproduces it byte for byte.
- **Migration list** (all `tap: PostPan`): `graph-compiler/src/lib.rs` (2 routes in the
  level-major coloring test; the route-helper test now maps all seven submix taps),
  `graph-compiler/tests/{bank_levels.rs (3), compile_shapes.rs (2)}`,
  `host-core/tests/{collapse_arming,randomized,submix_strip}.rs`, `host-web/src/tests.rs` (the K1
  boot test's route source), `protocol/src/session_wire/tests.rs` (2), `session/src/canonical.rs`,
  `session/tests/{canonical_schema,diagnostic_parity (4),invalid_matrix (5)}.rs`.
  `submix_strip.rs`'s gate-1 contributor/sum code moved unchanged into `drawn_contributors`, shared
  with the new tap gates (same draw order).
- **Gates** (x86-64 AVX2 host, native 8-lane):
  - 1 `host-core/tests/submix_strip.rs::a_bus_tap_renders_the_bits_of_the_same_tap_on_a_track_fed_its_sum`:
    seeds `0..8` (`run_seeds`, replayable), rate drawn per seed, all seven taps bit-identical, and
    the oracle's seven renders pairwise different every seed.
  - 2 `a_sidechain_keyed_by_a_bus_tap_reads_that_tap` green; the oracle keyed from `post_pan`
    instead renders other bits (asserted).
  - 3 `a_tap_before_a_bus_limiter_is_compensated_by_the_limiters_latency`: output latency 972, one
    inserted delay, on `bus-return`, of 486; the impulse peaks at 972 on both planes, summed.
  - 4 `session/tests/invalid_matrix.rs::submix_source_spellings_refuse_at_their_index_paths`
    (8 cases + 2 controls) and `protocol/src/session_wire/tests.rs::a_tapless_submix_source_is_refused_in_every_shape_that_carries_one`.
  - 5 `graph-compiler/tests/compile_shapes.rs::a_loop_through_a_bus_tap_is_a_graph_cycle`.
  - 6 `a_tapped_bus_renders_without_allocating` (seven sessions, render audit and thread-scoped
    counters exact zero after block 0).
  - 7: test-debug-a command rc 0 (98 binaries, 1140 passed); test-debug-b rc 0 (787 passed);
    `conformance_fixtures --check` ok; `check-protocol-wasm-parity.sh` `ok (simd128)` with the new
    hash; release build of audit/bench/capi/session-validator ok; `check-graph-determinism.sh` PASS
    100/100; `graph_fixture --check` clean; `check-console-fixtures.sh` ok;
    `check-builtins-fixtures.sh` ok (50 files) -- so no digest of a session without a submix tap
    moved; `cargo fmt --check`; workspace clippy `-D warnings` clean; the four policy pairs ok;
    `check-realtime-policy.sh` ok. `run-aarch64-tests.sh debug`: at batch push (no arm64 host).
- **Mutations** (each reverted):
  - `route_source_node` maps every submix source to `PostMatrix` -> gates 1, 2, 3 red (gate 3:
    no inserted delay) and gate 5 red (the witness runs through `a`'s end).
  - Submix `insert_return` mapped to the pre-fader stage -> gate 1 red at `tap InsertReturn`, gate
    3 red.
  - Sidechain edges only map a submix source to `PostMatrix` -> gate 2 red alone.
  - Cycle witnesses computed without route edges that leave a mid-strip stage -> gate 5 red (the
    compile panics on the unscheduled cycle).
  - JSON `"submix" | "submix_output"` alias -> gate 4 (session) red at `$.routes[0].source.kind`;
    a submix source given `tap: PostPan` regardless -> red at `$.routes[0].source.tap`.
  - Wire decoder defaulting a missing tag-2 tap to `PostPan` -> gate 4 (wire) red on the route
    shape; also making `SUBMIX` `[TAG, ID]` -> red.
  - Encoder writing tap 7 for every submix source -> `every_send_tap_tag_is_typed_and_canonical`
    and gate 4 (wire) red.
  - A `Vec` allocated in the graph runtime's `Route` arm -> gate 6 aborts under the render audit.
    The mutation is not tap-specific (it also reddens the existing allocation tests); gate 6's
    uniqueness is coverage: no other allocation test routes from a bus stage before its end.
- **Test value.**
  - Gate 1: red if any submix tap maps to the wrong stage of the strip; no existing test taps a
    processed bus.
  - Gate 2: red if a sidechain from a bus tap reads another stage (the bus's end, as before).
  - Gate 3: red if arrival times use the strip's end for every tap.
  - Gate 4 (session): red if `submix_output` is read as an alias or a tapless submix source gets
    a default tap, for a route or a sidechain source.
  - Gate 4 (wire): red if any of the route, sidechain or `SetRouteSource` decoders accepts a
    tapless tag-2 source.
  - Gate 5: red if a tap edge is left out of cycle detection or a tap is wired from the strip's
    end.
  - Gate 6: red if a mid-strip bus tap allocates on the render thread.
  - `every_send_tap_tag_is_typed_and_canonical` (extended to submix sources): red if the codec
    drops or rewrites a submix source's tap code.
  - `route_helpers_map_every_typed_variant_to_its_graph_node` (rewritten submix half): red if a
    submix tap maps to a stage other than the track tap's.
- **Deviations.**
  - Gate 1's console declares two latency-free EQ slots, `desk-eq` (`pre_insert`) and `desk-tone`
    (`post_insert`), not one: with an empty `post_insert` section `insert_return` and `pre_fader`
    would be the same audio, and the gate requires every tap to differ.
  - Gate 1 uses fixed strip parameters (compressor defaults, EQ bells at fixed gains) with drawn
    contributors, sums and rate, so the "every tap differs" assertion cannot fail by a draw.
  - Gate 5's diagnostic `path` is the witness's first edge, a strip edge spelled `$.submixes`; the
    routes are named in `cycle_edge_paths` (`$.routes[id=ab].source` etc.), which the test pins.
    The graph compiler's choice of primary path is unchanged.
  - The gate-4 refusal cases live in a new `invalid_matrix.rs` test rather than an existing
    category test, whose names pin their case counts.

## Dependencies

- *Carry every console slot on every submix strip* (#1202)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- In-place V1 amendment: retired spellings refuse and are never reallocated; codes are never
  renumbered.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch; no push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each.
