# Mute a route in the session

Slice 18b of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3, pushed once with slices 18a-26. It is the second half of the former slice 18
(VERIFY-3 MINOR 4.1) and builds on *Gate every route's coefficients through one function* (#1215).

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

Every route (send) has an on/off switch, as every surveyed console and DAW gives each send. A muted
route stays in the graph, with its edge, its PDC and its level kept, and contributes silence. A
later unmute can then be a value-only edit instead of a plan rebuild.

Today the only ways to silence a send are to remove the route, which is structural, or to abuse
`gain_db = -1000`, which happens to round to exactly zero gain (-800 dB is refused as subnormal).

This slice is the field, the wire, the edit, the SDK, the migration, the canonical `route-mute` row
and the fold decline. It sets gate values through the coefficient interface that *Gate every
route's coefficients through one function* froze (DESIGN P11, 5.7). In this slice a muted
route is still **mixed**, with coefficients `[+0.0; 4]`; *Skip an inactive route in its
destination's sum* (#1217) makes an undelayed muted route inactive (DESIGN P4).

## Context (verified on `fe8ac679`)

Batch K1 changed the route source (`RouteSource::Submix { submix_id, tap }`, wire tag 2 with a
required tap) and the submix model; none of the anchors below moved in substance. Re-read line
numbers before editing.

- **Session.**
  - `Route { id, source, destination, channel_matrix, gain_db }` (`crates/session/src/model.rs:591-602`).
  - Parse: `parse_route` (`crates/session/src/parse.rs:1297-1316`), exact key list at `:1300-1302`,
    literal at `:1308`.
  - Field keys: `key_module!(route, ...)` (`crates/session/src/visit.rs:106`, fields 1-5). The
    canonical walk's arm is `Route=>route ... [5]` (`visit.rs:227-229`).
  - Validation: `validate_routes` (`crates/session/src/validate.rs:465-492`), finiteness only.
  - Session `Route {` literals: `crates/session/src/canonical.rs:317` (the writer corpus),
    `crates/session/tests/canonical_schema.rs:194`, `crates/graph-compiler/tests/bank_levels.rs:653`,
    `crates/graph-compiler/tests/compile_shapes.rs:443`, `crates/host-core/tests/collapse_arming.rs:206`
    and `:216`, `crates/host-core/tests/randomized.rs:304-317`, and the decode at
    `crates/protocol/src/session_wire.rs:1681`. Re-run `git grep -n 'Route {' -- '*.rs'` (session
    `Route` literals) and `git grep -l '"gain_db"'` at the K3 base: K1 and K2 add routed sessions
    in host-core, host-web, capi and SDK tests.
- **Wire.**
  - `schema::session::route` (`crates/protocol/src/schema.rs:1038-1051`, fields 1-5, all required);
    `set_route_gain` (`:1437-1444`) is the edit-message template.
  - Codec: `tx_route` (`crates/protocol/src/session_wire.rs:1009`) and `parse_route` (`:1679`); edit
    encode at `:580`, edit decode at `:1323`.
  - Edits: `SessionEditOpcode` (`crates/protocol/src/model.rs:20-106`), `SetRouteGainDb = 0x0505`
    (`:97`), `from_raw` (`:117`, arm `:155`), the `SessionEdit` variant (`:380`), its opcode map
    (`:438`) and its apply arm (`:706`). The next free route opcode is `0506`.
- **Opcode-count pins (41 today):**
  - `crates/protocol/src/model.rs:1315` (`assert_eq!(allocated.len(), 41)`);
  - `crates/protocol/src/controller/tests.rs:326`, with the transaction-edit limit at `:327` pinned
    at one below the fixture's count (40);
  - `crates/conformance/src/protocol_corpus.rs:664` (doc comment);
  - `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`;
  - `docs/CONTROL_PROTOCOL_REGISTRY.md:74`;
  - `fuzz/corpus/complete-schema-manifest.md:4`, `:19`.
- **`COMPLETE_SCHEMA_HASH`** (`0xebf2_8262_1550_d44a` on `fe8ac679`; K1 re-pinned it) is spelled at
  `crates/conformance/src/protocol_corpus.rs:666`, `scripts/check-protocol-wasm-parity.sh:172-173`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and `fuzz/corpus/complete-schema-manifest.md:10`.
  `complete_all_opcode_fixture` (`protocol_corpus.rs:16`) holds one edit per allocated opcode, and
  the corpus encodes `UpsertRoute` and the route edits (`:194-204`), so a route wire change re-pins
  the hash. The frame count 46 (`crates/conformance/src/main.rs:31`,
  `crates/conformance/tests/conformance_corpus.rs:14`, the parity script `:174`) does not move: the
  new edit rides the existing transaction frame.
- **Graph, after *Gate every route's coefficients through one function*.**
  - `graph::RouteGate { mute, follow_zeroed }` with `RouteGate::OPEN` and `silences()`;
    `PreparedRoute { node, transform, gate }`; `graph::gated_route_coefficients(&transform, gate)`
    replaced `folded_route` in `node_kind` and in `plain_route_gains` (`crates/graph/src/runtime.rs`,
    `:6110-6122` on `fe8ac679`); `PlanningMetadata::route` returns the gate with the transform.
  - `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` and
    `RouteValueError`; the route lowering (`crates/graph-compiler/src/compile.rs:322-351` on
    `fe8ac679`) calls it with `mute = false` and stores `RouteGate::OPEN`. A refusal is
    `graph.gain.non_finite` at `$.routes[id=<id>].gain_db`.
  - That slice's graph-compiler unit test (its gate 2) compares `route_coefficients` with the
    compiled plan's bound constants.
  - The canonical text writes one `route-transform` row per route
    (`crates/graph-compiler/src/canonical.rs:262-273`).
  - The fold oracle knob is `graph::test_only_set_route_fold_declined`.
- **SDK.** `RouteSpec` (`sdk/src/core/types.ts:218-225`); route validation (`sdk/src/core/session.ts:764`)
  and normalisation (`:1450-1460`, `gain_db` at `:1458`); the writer's route key order
  (`sdk/src/internal/session-json.ts:83`); the enginectl route keys (`sdk/src/cli/session-request.ts:357`,
  `:363`). The SDK rebuild helper `sdk/test/console-evals.mjs:647-660` rebuilds routes without
  `mute`; with the default `false` it stays byte-identical.
- **Documents and inline sessions.**
  - 21 checked-in JSON documents declare `routes` (listed under Authorized paths); the writer
    corpus embeds one more. Every route `gain_db` is 0.0 and every route matrix is in `[-1, 1]`.
  - Three of the 21 are derived (`console-sixty-four-track-intended.json`, `-mono.json` and
    `-app.json`), and `scripts/check-console-fixtures.sh` compares each byte for byte with its
    script's stdout (`:33-37`). Migrate the parent `console-sixty-four-track.json`, then regenerate
    in this order (`mono` and `app` read the checked-in `intended`), one command each:
    - `python3 -I -B scripts/derive-intended-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-intended.json`
    - `python3 -I -B scripts/derive-mono-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-mono.json`
    - `python3 -I -B scripts/derive-app-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-app.json`
  - Inline sessions: `sdk/test/support.mjs:96-99` (an inline route; without `mute` every SDK headless
    eval that uses it goes red); `tools/parameter-metadata/tests/abi_layout.rs:168-174`;
    `tools/parameter-metadata/tests/round_trip.rs`; `crates/protocol/tests/console_session_edits.rs`.
  - `.claude/skills/author-session/SKILL.md:70` lists a route `channel_matrix`'s keys; the skill has
    no list of a route's keys.
  - `fixtures/session/v1/canonical.json`'s SHA-256 is pinned in a chain:
    `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml:12` and
    `tools/audit/src/fixture_builtins.rs:5102`, then the two `.toml` files in
    `fixtures/builtins/v1/MANIFEST.tsv:10-11`, then the MANIFEST digest at
    `tools/audit/src/builtins_graph.rs:52` and `fixture_builtins.rs:5275`. Only
    `cargo test --locked --release -p audit -p bench -p console-workload` runs those `tools/audit`
    tests (`qualification.yml:710`).
  - `fixtures/graph/v1/direct-route.*` are generated from `canonical.json`
    (`crates/graph-compiler/src/bin/graph_fixture.rs:31`) and pinned in `fixtures/graph/MANIFEST.tsv`;
    regenerate with `cargo run --locked -p graph-compiler --bin graph_fixture -- --write` only if a
    file moves.
  - The browser's staged document size is pinned: `hosts/host-web/tests/browser-v1/expected.json:57`
    (`"sessionDocumentBytes": "1905"`, the size of `session.json`), with a self-test row that prints
    the pinned value plus one at `scripts/check-browser-expected-resources.py:572`.

## Decisions frozen for this slice

- **D1. The field.**
  - `Route.mute: bool`, JSON key `mute` after `gain_db`, route field 6 (`Wire::Bool`), **required**
    (V1 has no optional fields). A missing key refuses with `schema.missing_field` at
    `$.routes[<i>].mute`.
  - New opcode `0506`, `SetRouteMute { route_id, mute }`, with a message `set_route_mute`
    (`ROUTE_ID` 1, `MUTE` 2). The opcode count becomes 42 at every pin; the controller's edit limit
    becomes 41.
- **D2. Mute through the frozen interface.** The interface below was frozen by *Gate every route's
  coefficients through one function* and is restated for reference only; this slice changes no
  signature. What this slice changes is the last bullet: the lowering passes `route.mute`.
  - In `crates/graph`:

    ```rust
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct RouteGate { pub mute: bool, pub follow_zeroed: [bool; 2] }
    impl RouteGate {
        pub const OPEN: Self = Self { mute: false, follow_zeroed: [false; 2] };
        /// Muted, or follow-zeroed on both source lanes: the route contributes nothing.
        pub const fn silences(self) -> bool;
    }
    pub struct PreparedRoute { pub node: GraphNodeId, pub transform: RouteTransform, pub gate: RouteGate }
    pub fn gated_route_coefficients(transform: &RouteTransform, gate: RouteGate) -> [f32; 4];
    ```

    `gated_route_coefficients` returns `[+0.0; 4]` when `gate.silences()`. Otherwise it returns
    exactly today's `folded_route` bits `[gain * ll, gain * lr, gain * rl, gain * rr]`, with `ll` and
    `rl` set to `+0.0` when `follow_zeroed[0]` and `lr` and `rr` set to `+0.0` when
    `follow_zeroed[1]`. It replaced `folded_route`: `node_kind` binds
    `gated_route_coefficients(&route.transform, route.gate)`, and the fold planner uses it with
    `RouteGate::OPEN` for the routes it may fold.
  - `follow_zeroed` stays `[false; 2]` until *Let a route into a submix follow its source strip's
    mute in the session* (#1218).
  - In `crates/graph-compiler`, exported from `lib.rs`:

    ```rust
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RouteValueError { Domain }
    pub fn route_coefficients(gain_db: f32, matrix: [f32; 4], mute: bool,
                              source_lane_muted: [bool; 2]) -> Result<[f32; 4], RouteValueError>;
    ```

    It runs today's `route_transform` checks, then `gated_route_coefficients` with
    `RouteGate { mute, follow_zeroed: source_lane_muted }`, and returns `Domain` if `route_transform`
    refuses **or** any coefficient of the **open** fold
    (`gated_route_coefficients(&transform, RouteGate::OPEN)`) is not finite, so domain validity never
    depends on the gate (VERIFY-2 MINOR 3: a finite gain times a finite coefficient can overflow, for
    example +700 dB with `ll = 1e10`; #1215 MINOR-1).
  - **This slice:** the compiler's route lowering calls `route_coefficients` with `route.mute` (and
    `source_lane_muted = [false; 2]`) for its domain check and keeps `route_transform`'s unfolded
    transform plus `RouteGate { mute: route.mute, follow_zeroed: [false; 2] }` in `PreparedRoute`. A `Domain` refusal
    is `graph.gain.non_finite` at `$.routes[id=<id>].gain_db`, as today. No checked-in session folds
    to a non-finite coefficient, so nothing that compiles today is refused.
- **D3. A muted route is mixed in this slice.** Its coefficients are `[+0.0; 4]` and its op runs as
  today's route op. The activity rule (an undelayed muted route is neither mixed nor loaded) is the
  next slice's; until then a muted route's contribution is the zero-coefficient mix, signed zeros
  included.
- **D4. Fold.** `plain_route_gains` declines a route whose gate is not `RouteGate::OPEN`, so a master
  with a muted contributor declines the single-master fold. (`PlanningMetadata::route` already
  returns the gate with the transform.) A fold that skips the lane is deferred item O8.
- **D5. Graph text.** For a route whose gate has `mute`, the canonical text adds one
  `route-mute\t<node>` row, written right after that route's `route-transform` row. An unmuted
  route's text is unchanged, so no existing graph digest moves.
- **D6. SDK.**
  - `RouteSpec.mute?: boolean`, defaulting to `false`, validated as a boolean.
  - The writer emits `mute` after `gain_db`.
  - The enginectl CLI accepts an optional `mute` on a route request.

## Deliverables

1. Session: model, parse, field keys, walk count (`[6]`) and canonical writer (D1).
2. Protocol: route field 6, the `set_route_mute` message, opcode `0506` (enum, `from_raw`, opcode map,
   variant, apply arm, encode, decode), every count pin, the edit limit, `COMPLETE_SCHEMA_HASH` at its
   four sites (reason: "route field `mute`, opcode `0506`"), and one `SetRouteMute` edit in
   `complete_all_opcode_fixture`.
3. Graph: D4 in `plain_route_gains`.
4. Graph compiler: the lowering passes `route.mute` (D2), and the canonical `route-mute` row (D5).
5. SDK: D6, plus a writer-parity eval in `sdk/test/console-evals.mjs` (against `engineCanonical()`,
   `:470-479`), a builder-default eval in `sdk/test/builder-evals.mjs`, and an `enginectl` case in
   `sdk/test/enginectl-cli.mjs`.
6. Migration:
   - add `"mute": false` after `gain_db` to every route in every JSON document listed under
     Authorized paths (the three derived fixtures by regeneration);
   - regenerate the writer corpus with
     `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session canonical_writer_corpus_is_rust_generated_and_current`;
   - add `mute: false` to every Rust session `Route {` literal in the Context (and every one the
     Context's re-run grep lists), and to the inline JSON and JS sessions;
   - add a bullet after `.claude/skills/author-session/SKILL.md:70` listing a route's keys (`id,
     source, destination, channel_matrix, gain_db, mute`; *Let a route into a submix follow its source
     strip's mute in the session* appends `follows_mute`);
   - re-pin `canonical.json`'s chain, `fixtures/graph/` (only files that move) and
     `hosts/host-web/tests/browser-v1/expected.json`'s `sessionDocumentBytes` with its self-test row
     (`scripts/check-browser-expected-resources.py:572` prints the new value plus one), each with the
     reason "route field `mute` added, value false".

   No render digest may move.
7. Docs:
   - `docs/SESSION_SCHEMA_V1.md`: the route paragraph (`mute`, and that a muted route stays in the
     graph);
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: opcode `0506`, route field 6, and the count 42;
   - `docs/CONTROL_PROTOCOL_CONFORMANCE.md` (count and hash);
   - `docs/session-v1.schema.json` (the route object).

## Authorized paths

- `crates/session/**`
- `crates/protocol/src/{schema.rs,session_wire.rs,session_wire/tests.rs,model.rs}`,
  `crates/protocol/src/controller/tests.rs`, `crates/protocol/tests/console_session_edits.rs`
- `crates/conformance/src/{protocol_corpus.rs,main.rs}`, `crates/conformance/tests/conformance_corpus.rs`
- `crates/graph/src/runtime.rs` (`plain_route_gains` only, D4)
- `crates/graph-compiler/**`
- `crates/host-core/tests/{randomized,collapse_arming}.rs` (the `Route` literals) and one new test file,
  for example `crates/host-core/tests/route_mute.rs`
- `sdk/src/core/{types.ts,session.ts}`, `sdk/src/internal/session-json.ts`,
  `sdk/src/cli/session-request.ts`, `sdk/test/{builder-evals,support,console-evals,enginectl-cli}.mjs`
- Every file the two greps in the Context list at the K3 base, for the added key only.
- The migrated documents:
  - `.claude/skills/author-session/worked-session.json`, `.claude/skills/author-session/SKILL.md`
  - `crates/graph-compiler/tests/data/reduced-nobus-from-970-verify.json`
  - `fixtures/session/v1/{builtins-automation,canonical-minimal,canonical,compressor-bank-observation,compressor-dynamic-bank-observation,compressor-dynamic-observation,console-sixty-four-track,console-sixty-four-track-app,console-sixty-four-track-intended,console-sixty-four-track-mono,observation-frame-shape,parametric-eq-bank-console,parametric-eq-nine-track}.json`
  - `hosts/host-web/qualification/{live-control,observation,stall}-session.json`
  - `hosts/host-web/tests/browser-v1/{command-session,observation-session,session}.json`
  - `fixtures/session-canonical/v1/canonical-writer-corpus.json` (regenerated)
- Inline sessions: `tools/parameter-metadata/tests/{abi_layout,round_trip}.rs`
- The pin chain: `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `tools/audit/src/{fixture_builtins.rs,builtins_graph.rs}`,
  `fixtures/graph/v1/direct-route.*`, `fixtures/graph/MANIFEST.tsv`,
  `hosts/host-web/tests/browser-v1/expected.json` (`sessionDocumentBytes` only),
  `scripts/check-browser-expected-resources.py` (the self-test row at `:572` only)
- `scripts/check-protocol-wasm-parity.sh` (the hash literal), `fuzz/corpus/complete-schema-manifest.md`
- `docs/SESSION_SCHEMA_V1.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`, `docs/session-v1.schema.json`
- this spec

## Non-goals

- No activity rule: a muted route is still mixed (*Skip an inactive route in its destination's
  sum*).
- No live mute, gain or matrix (*Ramp live send coefficients on the render plane*, #1220).
- No `follows_mute` (*Let a route into a submix follow its source strip's mute in the session*).
- No tightening of route value domains beyond refusing a non-finite folded coefficient (owner
  question Q2, deferred item O11).
- No fold of a master with a muted contributor (O8). No performance work.

## Hazards

- **One function.** The bits a plan binds and the bits a live producer will push are the same only
  because both go through `gated_route_coefficients`. Do not add a second mute path. Gate 2 pins it.
- **The fold.** The fold builds its epilogue constants from the plan's routes. Folding a muted route
  with its open coefficients would accumulate a contribution the route op would not. D4 declines it;
  gate 4 pins it.
- **Signed zero.** A zero-coefficient mix of a negative sample is `-0.0`, so a muted route's
  contribution in this slice is not "absent". Every oracle in this slice mixes `[+0.0; 4]`; it is not
  compared with "the session without the route".
- **Fixture churn is mechanical.** The adversarial review diffs every migrated document for anything
  other than the added key.

## Objective gates

1. **A muted route mixes zero coefficients.** New host-core test (`crates/host-core/tests/route_mute.rs`,
   on the harness pattern of `crates/host-core/tests/submix_strip.rs`): bus `b` has two contributors
   fed distinct non-constant signals per lane, one route muted. The bus output equals a scalar oracle
   that mixes the muted route with `[+0.0; 4]` (the D3 expression, two roundings) and sums in route-ID
   order, bit for bit.
   *Test value: it turns red if the compiler or the codec drops `mute`, or applies it as anything
   other than four `+0.0` coefficients (for example through the gain, which keeps a signed matrix
   product).*
2. **One coefficient function, muted.** Extend the graph-compiler unit test of gate 2 of *Gate every
   route's coefficients through one function* (do not add a second test): it now compiles sessions
   with random `mute` per route and asserts that `route_coefficients(gain_db, matrix, mute, [false; 2])`
   equals, bit for bit, the constant the runtime binds for the same route (`gated_route_coefficients`
   of the compiled `PreparedRoute`).
   *Test value: it turns red if the compiler stores a gate whose `mute` differs from the session's,
   so the prepared and the domain-checked paths disagree on a muted route.*
3. **Mute is not structural.** A session with a latent path (a true-peak limiter insert, 486 samples
   at 48 kHz) compiles to identical inserted delays and output latency whether a given route is muted
   or not.
   *Test value: it turns red if mute drops the route from the graph, which would make a later live
   unmute a latency change.*
4. **No folded muted lane.** Eight tracks feed one bus, and one route is muted. `bank_route_folds()`
   (a plan-wide count) is 0 (the bus is the plan's only fold candidate; the same session unmuted
   reports a nonzero count, recorded in the PR), and the output is bit-identical to the same plan
   bound with `graph::test_only_set_route_fold_declined`.
   *Test value: it turns red if the fold accumulates a muted contribution with the route's open
   coefficients.*
5. **The gate is in the sealed text.** A graph-compiler test compiles one session twice, with a route
   muted and unmuted: the canonical texts differ by exactly one `route-mute` row, after that route's
   `route-transform` row.
   *Test value: it turns red if the gate is invisible to the canonical text, so two plans binding
   different bits would share one digest.*
6. **Grammar, wire and edit.**
   - A route without `mute` refuses with `schema.missing_field` at `$.routes[<i>].mute` (in
     `crates/session/tests/invalid_matrix.rs`).
   - `0506` mutes a route in the committed snapshot; canonical and BTLV round trips are exact; the
     opcode-count test reads 42 and the conformance corpus carries `0506`.
   *Test value: it turns red if the field is optional or lost in the codec, or `0506` is
   unregistered.*
7. **SDK.** In `sdk/test/console-evals.mjs`, against `engineCanonical()`, an SDK-built session with one muted send writes the
   engine's canonical text byte for byte; in `sdk/test/builder-evals.mjs`, a route without `mute`
   writes `"mute": false`; `enginectl` accepts `mute` (`sdk/test/enginectl-cli.mjs`).
   *Test value: it turns red if the SDK omits or misplaces the key, which the engine now refuses.*
8. **The migration moved no render.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`, with `target/issue6/fresh-process-determinism.json`
     identical to its base except for re-pinned rows listed with reasons
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`, with only the listed re-pins
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - every `output_sha256` unchanged
9. **Workspace, wire, SDK and policy.**
   - the test-debug-a and test-debug-b commands (DESIGN section 7)
   - `bash scripts/check-protocol-wasm-parity.sh`
   - `python3 -B scripts/check-browser-expected-resources.py --self-test` (this slice edits its row at
     `:572`)
   - `bash scripts/check-sdk-types.sh`; at the K3 boundary, `bash scripts/check-sdk-headless.sh <A>`,
     `bash scripts/sdk-package.sh check <A>` and
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (after
     `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in session protocol-control graph builtins host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
10. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug`
    job at the K3 push. `graph`, `graph-compiler` and `host-core` are in it.

No allocation gate: this slice adds no render-thread state (a muted route is an ordinary route op
with other constants).

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The re-pin list, each with its reason.
- A diff summary confirming that every migrated document changed only by the added key.

### Attempt 1 record (Terra)

- **Implementation.** Session: `Route.mute`, parsed as a required boolean (`schema.missing_field` at
  `$.routes[<i>].mute`), route field key 6, walk count `[6]`. Protocol: `route::MUTE` (field 6,
  `Wire::Bool`, required), `set_route_mute` (`ROUTE_ID` 1, `MUTE` 2), `SetRouteMute = 0x0506` (enum,
  `from_raw`, variant, opcode map, apply arm, encode, decode). Graph: `plain_route_gains` returns
  `None` unless the gate is `RouteGate::OPEN` (D4). Graph compiler: the lowering builds
  `RouteGate { mute: route.mute, follow_zeroed: [false; 2] }`, passes it to `route_coefficients` and
  stores it in `PreparedRoute` (D2); the canonical text writes `route-mute\t<node>` right after a
  muted route's `route-transform` row (D5). SDK: `RouteSpec.mute?`, boolean-validated, written as
  `mute` after `gain_db` (default `false`); `enginectl` admits `mute`. The SDK rebuild helper in
  `console-evals.mjs` now passes `mute: route.mute`.
- **Anchor drift.** `COMPLETE_SCHEMA_HASH` was `0xa1dc_c56f_2e4a_48f9` at the K3 base (K1/K2
  re-pinned it after `fe8ac679`); the skill's `channel_matrix` bullet is at `SKILL.md:79`; the opcode
  count sentence is `CONTROL_PROTOCOL_REGISTRY.md:81`. K1/K2 added session `Route` literals in
  `host-core/tests/{strip_controls,strip_meters,submix_strip}.rs` and inline routes in
  `crates/protocol/tests/submix_strip_edits.rs`; all are migrated (both Context greps list them).
  `canonical-minimal.json` declares no route, so it needs no edit.
- **Deviation 1 (pin the spec missed).** `crates/capi/src/runtime/tests.rs`
  `ALL_COMMAND_RESPONSE_VECTORS[1]` pins the C ABI snapshot response, whose length field is the
  nine-track fixture's canonical snapshot size: re-pinned 13,729 to 13,918 (`0x365e`), reason "route
  field `mute` added, value false" (21 bytes x 9 routes). The file is in the `Route {` grep's list
  (its `RemoveRoute`); the change is the length bytes and one comment line.
- **Deviation 2 (gate 4 topology).** With the bus feeding the output from `post_pan`, a muted
  contributor makes the bus decline and the planner then folds the *output's* single contributor
  (`bus-main`, 1 lane): the bus was not the plan's only fold candidate. The gate-4 session feeds the
  output from the bus's `pre_fader` tap, so the output's contributor reads mid-strip and the bus is
  the one candidate. Measured: open bus 8 lanes folded, muted bus 0.
- **Deviation 3 (gate 1 taps).** Gate 1 routes the contributors from their `input` taps and the bus
  to the output from its `input` tap (the oracle track likewise): a strip's pan turns some signed
  zeros positive, which hid the muted contribution's sign (measured: a gain-path mute stayed green
  through `post_pan` taps). The bus input is still the D9 sum under test.
- **Re-pins, each "route field `mute` added, value false" unless stated.**
  `COMPLETE_SCHEMA_HASH` `0xa1dc_c56f_2e4a_48f9` -> `0x39e5_a2c1_d317_a9fe` at its four sites (reason
  "route field `mute`, opcode `0506`"); opcode count 41 -> 42 (model test, controller fixture count,
  `CONTROL_PROTOCOL_CONFORMANCE.md`, `CONTROL_PROTOCOL_REGISTRY.md`, `complete-schema-manifest.md`),
  controller edit limit 40 -> 41; `canonical.json` sha256 `5f887676...` -> `0be977ee...` in both
  `prepare_256_tracks-*.toml` and `fixture_builtins.rs`; their `MANIFEST.tsv` rows (`57b65ec0...` ->
  `d5d19ee7...`, `2f5f80f3...` -> `66824cd9...`); the MANIFEST digest `fced289f...` -> `6d466903...`
  in `builtins_graph.rs` and `fixture_builtins.rs`; `sessionDocumentBytes` 1905 -> 1926 and its
  self-test row 1906 -> 1927; the capi vector above. `fixtures/graph/` did not move (an unmuted
  route's text is unchanged). The writer corpus was regenerated by its command.
- **Migration diff.** Every changed JSON document (20, including the three regenerated derived
  fixtures) parses to its base exactly once each route's `mute` key is removed, and every route ends
  `gain_db, mute` with `mute: false` (checked with a script against `HEAD`). The inline Rust, JS and
  writer-corpus sessions changed by the key alone.
- **Tests and test value (each mutation applied, run red, reverted).**
  - `session/tests/invalid_matrix.rs::route_mute_is_a_required_boolean`: red if `mute` becomes
    optional or non-boolean, or a muted route loses its switch through the canonical writer.
    Mutation: parse ignores the key (`Some(false)`) -> RED.
  - `protocol model::tests::set_route_mute_switches_the_route_in_the_committed_snapshot`: red if the
    `0506` apply arm writes the wrong route or ignores the value. Mutation: always mute -> RED.
  - Extended `opcode_registry_appends_the_console_edits_and_reallocates_nothing` (42, `0x0506`):
    `0x0506` arm removed -> RED (41 != 42). Extended
    `every_route_and_automation_opcode_round_trips_canonically` (an upserted muted route and a
    `SetRouteMute`): route decode drops field 6 -> RED.
  - Gate 2, extended `every_route_coefficient_comes_from_the_one_gated_function` (random `mute` per
    route; a muted overflow is still refused): red if the compiler stores a gate whose `mute`
    differs from the session's. Mutation: lowering stores `RouteGate::OPEN` -> RED.
  - Gate 5, `graph-compiler/tests/route_coefficients.rs::a_muted_route_seals_one_route_mute_row_after_its_transform`
    (first, middle and last route): red if the gate is invisible to the canonical text. Mutations:
    no row -> RED; open gate stored -> RED.
  - Gate 1, `host-core/tests/route_mute.rs::a_muted_route_mixes_zero_coefficients` (16 seeds): red if
    the session grammar or compiler drops `mute` (the BTLV codec drop is the protocol round trip's
    catch, above) or mutes by anything other than four `+0.0`. Mutations: open gate
    stored -> RED; mute through the gain (`0.0 * coefficient`) -> RED (`-0.0 != 0.0` at sample 12).
  - Gate 3, `route_mute.rs::muting_the_delayed_route_moves_no_delay_or_latency`: red if a muted
    *delayed* route loses its PDC line, which the activity rule (#1217) must not drop. Mutation: the
    lowering skips a muted route -> RED (also red under gates 1 and 5; verdict NIT 3).
  - Gate 4, `route_mute.rs::a_bus_with_a_muted_contributor_folds_no_lane`: red if the fold folds a
    gated route. Mutations: no decline -> RED (8 folds); fold with `RouteGate::OPEN` coefficients ->
    RED (8 folds).
  - Gate 7: `builder-evals.mjs` "a route without mute writes "mute": false after gain_db; a
    non-boolean mute is refused", `console-evals.mjs` "a session with one muted send is the engine's
    canonical JSON, byte for byte", `enginectl-cli.mjs` "a route request's optional mute reaches the
    document, and a non-boolean mute is refused": red if the SDK omits, misplaces or mistypes the
    key. Mutations: writer omits `mute` -> RED (builder, console); `mute` before `gain_db` -> RED
    (builder, console); normalisation drops it -> RED (builder, console); no boolean check -> RED
    (builder); `enginectl` key not admitted -> RED. The existing "bounded Rust-authority corpus" eval
    now expects the `gain_db` line's comma.
- **Gates (worktree head before commit, x86-64-v3 host).**
  - 8: release build ok; `check-graph-determinism.sh` PASS (100/100), and
    `target/issue6/fresh-process-determinism.json` is byte-identical (`cmp`) to the parent's;
    `graph_fixture -- --check` exit 0 (no fixture moved); `check-console-fixtures.sh` ok;
    `check-builtins-fixtures.sh` ok (50 files) with only the listed re-pins;
    `cargo test --locked --release -p audit -p bench -p console-workload` 110 passed, 0 failed;
    `audit capi` output byte-identical (`cmp`) to the earlier run (`pcm_digest ff6cdcb96cdcdad5`).
    No `output_sha256` moved.
  - 9: test-debug-a exit 0 (1181 passed, 0 failed, 9 ignored; run with `--no-fail-fast`);
    test-debug-b exit 0 (787 passed, 0 failed, 24 ignored); `check-protocol-wasm-parity.sh` ok
    (simd128); browser `--self-test` ok (32 red mutations) and `--artifacts <A>` ok;
    `check-sdk-types.sh` ok; `check-sdk-headless.sh <A>` 349 passed; `sdk-package.sh check <A>` ok
    (16 enginectl tests); `cargo fmt --all -- --check` ok; clippy `-D warnings` ok; the seven
    check/test policy pairs ok.
  - 10: `run-aarch64-tests.sh debug` not runnable on this x86 host; at batch push (CI
    `aarch64-debug`).
- **No bit moved (unmuted sessions).** The determinism record, the graph fixtures, every builtins
  `output_sha256`, the console-workload counts and the `audit capi` PCM digest are unchanged.

### K3 follow-up record (after the attempt 1 PASS verdict)

Applied in the K3 follow-up commit (on `eff44271d`, branch `codex/batch-submix-k3`):

- **NIT-1.** D2's copy of the frozen interface says the domain check is of the open fold (#1215
  MINOR-1's text).
- **NIT-2.** The `3.2e34` comment in `route_coefficients.rs` is `1.0e35`.
- **NIT-3.** The record's gate-1 and gate-3 test-value sentences are reworded (above).
- **NIT-4.** No change: enginectl leaves the `mute` boolean check to the builder, as for `bypass`.
- **NIT-5.** The transform is derived once (`route_values`, #1215 NIT-2).
- **INFO.** A delayed muted route writes `-0.0` words, which keep its destination out of a
  `+0.0` silence latch: a performance note for the silence work (DESIGN O2), never a bit change.

## Verdict

- **Attempt 1** (`f48fe7c74`): Sol PASS. No BLOCKER, MAJOR or MINOR; five NITs, applied or
  answered above. `docs/handoffs/submix-sends-2026-10-02/verdicts/1216-attempt1.md`.

## Dependencies

- *Gate every route's coefficients through one function* (#1215)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free.
- In-place V1 amendment: append field 6 and opcode `0506`; renumber nothing.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
