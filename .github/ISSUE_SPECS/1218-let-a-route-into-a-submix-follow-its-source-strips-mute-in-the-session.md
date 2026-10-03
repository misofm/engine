# Let a route into a submix follow its source strip's mute in the session

Slice 20 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A send can follow its source strip's mute, as SSL's "Follow Mute" (on by default), DiGiCo's
pre-mute tap and S6L's post-mute pickoffs do. With `follows_mute: true` on a route into a submix:

- a muted track's pre-fader reverb send goes quiet with the track;
- when both of the source's lanes are muted, the send does no work at all.

Today a pre-fader send leaks when its channel is muted or solo-muted: the probe measured
0.3155/0.3549 in the bus where 0 was expected.

`follows_mute` is a **send's** property. A route into the output is never live (DESIGN 5.7), so a
follow there could only be prepared, and a live unmute of its source would leave the strip's main
route silent until a plan replacement (VERIFY-2 N1). A route into the output therefore must have
`follows_mute: false`, and the session refuses `true` there (DESIGN P11).

This slice is the session and preparation half: a saved session with a muted source renders the
send silent on every host, including a fan's control-free playback. Live composition (solo, live
mute) is *Let a send follow its source strip's mute live in the browser* (#1224).

## Context (verified on `fe8ac679`)

- **After *Mute a route in the session* (#1216) and *Skip an inactive route in its destination's sum* (#1217):**
  - `Route.mute` is route field 6, opcode `0506` exists, and the opcode count is 42;
  - `PreparedRoute.gate: RouteGate { mute, follow_zeroed: [bool; 2] }`, with `follow_zeroed` always
    `[false; 2]` so far, and `RouteGate::silences()` true when muted **or** follow-zeroed on both
    lanes;
  - `graph::gated_route_coefficients` zeroes the left source column (`ll`, `rl`) for
    `follow_zeroed[0]` and the right column (`lr`, `rr`) for `follow_zeroed[1]`;
  - `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` already takes the
    fourth argument; the compiler passes `[false; 2]`;
  - a silenced undelayed route is inactive; a silenced delayed route is mixed with `[+0.0; 4]`; the
    canonical text has a `route-mute` row for a muted route;
  - the route key order and every pin are as that slice left them (`crates/session/src/visit.rs:106`,
    `crates/protocol/src/schema.rs:1038-1051`, `sdk/src/internal/session-json.ts:83`, the count pins
    and the migrated document list).
- **The source strip's mutes** are its session fader's `left_mute` and `right_mute`
  (`DualMonoFader`, `crates/session/src/model.rs:537-546`). After batch K1 a submix strip carries the
  same fader, and a route source names its strip by `track_id` or `submix_id`
  (`RouteSource::{Track, Submix}`, each with a `tap`).
- **Mute clears bits.** A settled-muted lane is exact `+0.0` at the fader, so a `post_fader` or
  `post_pan` send from a muted strip is already silent. `follows_mute` changes audio only for taps
  before the fader (`input` through `pre_fader`); for the later taps it changes no audible sample.
  A send whose two source lanes are muted is skipped, and its silent contribution becomes `+0.0`
  instead of a signed zero (verdict MINOR-2: with a negative matrix coefficient, `+0.0 * c` is
  `-0.0`, so the follow is digest-visible, never audible).
- **Contextual refusal precedent.** Session validation refuses a closed token that is legal in
  general but not in its context with `schema.invalid_enum` at the token's path
  (`crates/session/src/validate.rs:646-652`: a shared builtin parameter must be addressed as `both`).
  `validate_routes` is at `validate.rs:465-492`.
- **The SDK rebuild helper** `sdk/test/console-evals.mjs:647-660` rebuilds a document's routes
  without `mute` or `followsMute`. Its byte-for-byte rebuilds (`:681-692`) cover the app and intended fixtures, whose routes all go to the
  output, so the output default (`false`) keeps them green; but a rebuild of any document with a send
  would write the SDK default `true` where the document says `false`. The helper must carry both
  fields (VERIFY-2 M8).
- **#1053** (open, no code on `fe8ac679`) classifies a C ABI committed-model delta as live when
  `fader` and `matrix_or_pan` are the only fields that differ
  (`.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md:38-40`;
  the A2 D1 amendment `:145-147`). Its classifier will be host-core's `live_builtin_delta`, behind the
  `control-provider` feature (`crates/host-core/Cargo.toml:15`). *Record the submix, send and VCA
  ruling* annotated #1053's spec with the coordination rule (DESIGN P13).

## Decisions frozen for this slice

- **D1. The field.**
  - `Route.follows_mute: bool`, JSON key `follows_mute` after `mute`, route field 7 (`Wire::Bool`),
    **required**. A missing key refuses with `schema.missing_field` at `$.routes[<i>].follows_mute`.
  - **Only into a submix** (DESIGN P11, VERIFY-2 N1). `follows_mute: true` on a route whose
    destination is the output refuses with `schema.invalid_enum` at `$.routes[<i>].follows_mute`,
    message "follows_mute applies only to a route into a submix".
  - New opcode `0507`, `SetRouteFollowsMute { route_id, follows_mute }` (message
    `set_route_follows_mute`: `ROUTE_ID` 1, `FOLLOWS_MUTE` 2). It is validated on the final
    candidate like every edit, so `0507` setting `true` on a route into the output refuses the whole
    transaction. A `follows_mute` change is structural for liveness (DESIGN 5.7).
  - The opcode count becomes 43 at every pin, and the controller's edit limit 42.
- **D2. Lowering.** For a route with `follows_mute`, the compiler sets
  `RouteGate.follow_zeroed = [source.fader.left_mute, source.fader.right_mute]` of its source strip
  (a track or a submix), and its domain check calls `route_coefficients` with the same
  `source_lane_muted`. A route without `follows_mute` keeps `[false; 2]`. Zeroing is applied to the
  coefficients by `gated_route_coefficients`, **never** through the gain. With both lanes muted the
  gate silences the route, so it is inactive when undelayed and mixed with `[+0.0; 4]` when delayed.
- **D3. Graph text.** For a gate with any `follow_zeroed` lane, the canonical text adds one
  `route-follow-zeroed\t<node>\t<0|1>\t<0|1>` row right after that route's `route-transform` row (and
  its `route-mute` row, if any). Other routes' text is unchanged.
- **D4. Documents keep their sound.** The migration writes `"follows_mute": false` into every
  existing route, so no render moves.
- **D5. The SDK** (DESIGN P11, Q4).
  - `RouteSpec.followsMute?: boolean`. Omitted, it is `true` for a route into a submix and `false`
    for a route into the output.
  - `followsMute: true` on a route into the output throws `MisoUsageError` naming the rule.
  - The writer emits `follows_mute` after `mute`, and the enginectl CLI accepts `followsMute`.
- **D6. #1053 coordination (DESIGN P13).**
  - If #1053 has landed, extend its classifier: a committed-model delta that changes `left_mute` or
    `right_mute` of a strip that is, in the post-commit model, the source of any route with
    `follows_mute: true` is **structural**, until *Let C ABI sends follow their source strip's mute
    live* (#1226) composes the follow records.
  - If #1053 has not landed, change nothing in capi; check that #1053's spec carries the annotation.

## Deliverables

1. Session: model, parse, the D1 validation, canonical writer, field keys and walk count (`[7]`).
2. Protocol: route field 7, the `set_route_follows_mute` message, opcode `0507` (enum, `from_raw`,
   opcode map, variant, apply arm, encode, decode), every count pin, the edit limit,
   `COMPLETE_SCHEMA_HASH` at its four sites (reason: "route field `follows_mute`, opcode `0507`"),
   and one `SetRouteFollowsMute` edit in `complete_all_opcode_fixture`.
3. Graph compiler: D2 and D3.
4. SDK: D5, with a writer-parity eval, a builder-default eval and a builder-refusal eval.
5. Migration: add `"follows_mute": false` after `mute` in exactly the documents, inline sessions and
   writer corpus of *Mute a route in the session*, and append `follows_mute` to the route-key bullet
   that slice added after `.claude/skills/author-session/SKILL.md:70`. Update the `InvalidEnum` doc
   comment in `crates/session/src/diagnostic.rs` ("A string enum token is outside its closed schema
   set") to cover a closed boolean value illegal in its context. Fix `sdk/test/console-evals.mjs:647-660` to carry
   `mute` and `followsMute` into its rebuild. Re-pin the same chain (and `expected.json`'s
   `sessionDocumentBytes` with its self-test row), each with the reason "route field `follows_mute`
   added, value false".
6. D6, only if #1053 has landed: the classifier rule and one capi test.
7. Docs:
   - `docs/SESSION_SCHEMA_V1.md`: the route paragraph, D1 and D2;
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: opcode `0507`, route field 7, the output-route rule and the
     count 43;
   - `docs/CONTROL_PROTOCOL_CONFORMANCE.md`;
   - `docs/session-v1.schema.json`;
   - `docs/BUILTINS_AND_METERING_V1.md`, "Metering and observation while soloed" (`:168-180`): one
     sentence saying that a pre-fader send follows its strip's mute only with `follows_mute`, which
     only a route into a submix may set.

## Authorized paths

- Exactly the authorized paths of *Mute a route in the session*, for the same reasons, including
  `sdk/test/console-evals.mjs` (the rebuild helper)
- `sdk/test/` (one builder-refusal eval, in `builder-evals.mjs`)
- `crates/host-core/src/**` and `crates/capi/src/runtime/control.rs` plus one new capi test file,
  only for D6 and only if #1053 has landed
- `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No live follow composition (solo or live mute moving a send): *Let a send follow its source strip's
  mute live in the browser* and *Let C ABI sends follow their source strip's mute live*.
- No change to taps: pre-fader taps stay un-gated. `follows_mute` is per route.
- No follow on a route into the output, and no live output routes (DESIGN O9).
- No follow for VCA mutes: the VCA umbrella feeds them through the same lowering (DESIGN 5.10).

## Hazards

- **The column, not the gain.** Zeroing through the gain silences both columns; zeroing the wrong
  column swaps the lanes. Gate 1 pins both.
- **A delayed send with both lanes muted** must stay mixed with `[+0.0; 4]` (P4), exactly like a
  muted delayed route.
- **The output-route refusal is a validation, not a parse.** The parser accepts any boolean; the
  candidate model refuses it, so a transaction that sets it and then re-points the route into a
  submix commits.
- **#1053 ordering.** If #1053 lands after this slice, its own spec (annotated by slice 00) carries
  D6's rule. Check the annotation is present before closing.

## Objective gates

Render gates go in `crates/host-core/tests/route_mute.rs`, with distinct non-constant signals per
lane and an asymmetric send matrix.

1. **A left-only follow.** A track with `left_mute: true` has a `pre_fader` send with `follows_mute`
   into a bus. The send's left source column contributes nothing and its right column is unchanged:
   the bus input is bit-identical to the same session with that route's matrix at `ll = rl = 0` and
   `follows_mute: false`.
   *Test value: it turns red if the flag is ignored, zeroes the wrong column, or is applied through
   the gain.*
2. **Both lanes.** With both source lanes muted, an undelayed follow send is inactive (its route-mix
   counter does not advance) and the bus equals the P4 oracle. A delayed follow send (486 samples of
   compensation from a sibling's true-peak limiter, 48 kHz) is mixed with `[+0.0; 4]` and its counter
   advances every block.
   *Test value: it turns red if a fully follow-muted send is mixed when undelayed, or skipped when
   delayed.*
3. **Not following leaks as before.** PR evidence, not a committed test (`AGENTS.md`: a one-time
   "no bit moved" comparison against the base): with `follows_mute: false`, a session whose muted
   track has a pre-fader send renders bit-identically on this branch and on its base.
4. **A submix source.** A submix strip with `right_mute: true` and a `pre_fader` follow send into
   another submix zeroes the right column, bit-identically to gate 1's construction mirrored.
   *Test value: it turns red if source mutes are read from tracks only.*
5. **One function.** Extend the graph-compiler unit test of gate 2 of *Gate every route's
   coefficients through one function* (#1215, extended by gate 2 of *Mute a route in the session*); do not
   add a second test. It draws random gains, matrices, mutes and source mutes, and asserts that `route_coefficients(gain_db, matrix, mute, source_lane_muted)` equals, bit
   for bit, `gated_route_coefficients` of the `PreparedRoute` the compiler built for the same route.
   *Test value: it turns red if the compiler and a live producer would compute a follow-muted route's
   coefficients by different paths.*
6. **The output-route rule.**
   - A route into the output with `follows_mute: true` refuses with `schema.invalid_enum` at
     `$.routes[<i>].follows_mute` (`crates/session/tests/invalid_matrix.rs`).
   - A transaction whose `0507` sets `true` on a route into the output refuses whole: the model and
     revision do not move (`crates/protocol/tests/console_session_edits.rs`).
   - The SDK builder throws `MisoUsageError` for `followsMute: true` on a route into the output, and
     writes `follows_mute: false` there when the spec omits it (`sdk/test/builder-evals.mjs`).

   *Test value: it turns red if an output route can follow, which would leave a strip's main route
   silent after a live unmute (VERIFY-2 N1).*
7. **Grammar, wire and edit.**
   - A route without `follows_mute` refuses with `schema.missing_field` at
     `$.routes[<i>].follows_mute`.
   - `0507` sets it in the committed snapshot; round trips are exact; the count reads 43;
     conformance carries `0507`.
   - The SDK writes `follows_mute: true` for a route into a submix whose spec omits it, and the
     writer-parity eval matches the engine's canonical text byte for byte.

   *Test value: it turns red if the field is optional, lost in the codec, or defaulted the wrong way
   in the SDK.*
8. **The migration moved no render.** The commands of gate 8 of *Mute a route in the session*,
   including `cargo test --locked --release -p audit -p bench -p console-workload`.
9. **Conditional on #1053 having landed.** Through the C ABI, a `020f` that mutes a track which is
   the source of a `follows_mute` send produces a new epoch (structural). After the boundary the
   output is bit-identical to a plan compiled from the committed model.
   *Test value: it turns red if #1053's value-only path pushes the strip's mute and leaves the
   prepared follow-zeroed send open.*
10. **Workspace, wire, SDK, policy and 4-lane.** Gates 9 and 10 of *Mute a route in the session*,
    plus `bash scripts/check-capi-abi.sh`, `bash scripts/check-capi-abi.sh --self-test` and
    `./target/release/audit capi` if D6 applies.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The re-pin list with reasons.
- Whether #1053 had landed, and therefore whether D6 was implemented here.

### Attempt 1 record (Terra)

- **#1053 had not landed** (`git log origin/main` holds only its docs commit `bdafbb437`), so D6 is
  not implemented and capi's control path is untouched. #1053's spec carries the P13 annotation
  (`1053-...md:153-161`: a `left_mute`/`right_mute` delta of a strip that is the post-commit source
  of a `follows_mute: true` route is structural until #1226). Gate 9 does not apply.
- **Implementation.** Session: `Route.follows_mute`, a required boolean after `mute`, key field 7,
  walk count `[7]`; `validate_routes` refuses `true` on an `OutputInput` route with
  `schema.invalid_enum` at `$.routes[<i>].follows_mute`, message "follows_mute applies only to a
  route into a submix"; the `InvalidEnum` doc covers a value legal in general but not in context.
  Protocol: `route::FOLLOWS_MUTE` (field 7, `Wire::Bool`, required), `set_route_follows_mute`
  (`ROUTE_ID` 1, `FOLLOWS_MUTE` 2), `SetRouteFollowsMute = 0x0507` (enum, `from_raw`, variant, opcode
  map, apply arm, encode, decode); count 43, edit limit 42, one `SetRouteFollowsMute` (encoded
  `true`, a codec value) in `complete_all_opcode_fixture`. Graph compiler (D2): the lowering builds
  a strip-ID -> `[left_mute, right_mute]` map (only when some route follows; tracks and submixes)
  and sets `RouteGate.follow_zeroed` from the route's source strip, passing the same value to
  `route_coefficients`; zeroing stays in `gated_route_coefficients`. D3: a gate with a zeroed lane
  writes `route-follow-zeroed\t<node>\t<l>\t<r>` after the route's `route-transform` (and
  `route-mute`) row. SDK (D5): `RouteSpec.followsMute?`, default `destination.kind ===
  "submix_input"`, boolean-checked, `true` into the output throws `MisoUsageError`
  (`schema.invalid_enum`); the writer emits `follows_mute` after `mute`; enginectl admits
  `followsMute`; the `console-evals.mjs` rebuild helper carries `mute` and `followsMute`.
- **Anchor drift.** `validate_routes` is at `validate.rs:548` (the `InvalidEnum` precedent at
  `:743`); the skill's route-key bullet at `SKILL.md:80`; the rebuild helper's route call at
  `console-evals.mjs:657-668`; `COMPLETE_SCHEMA_HASH` was `0x39e5_a2c1_d317_a9fe` (#1216's pin).
- **Deviations.**
  1. Pin the spec missed, as #1216's deviation 1: `crates/capi/src/runtime/tests.rs`
     `ALL_COMMAND_RESPONSE_VECTORS[1]` (the nine-track snapshot length) 13,918 -> 14,179
     (`0x3763`), 29 bytes x 9 routes, reason "route field `follows_mute` added, value false".
  2. Existing tests edited only for the new key: #1216's
     `set_route_mute_switches_the_route_in_the_committed_snapshot` substring (`"mute": false,`) and
     builder eval regex; #1205's rebuild eval gains `followsMute: false` on `t-bus` so the helper fix
     has a red case (mutation: helper drops `followsMute` -> RED).
  3. Beyond the listed evals: one `enginectl-cli.mjs` case (D5's CLI clause), and one D3 test
     (`a_following_send_seals_one_route_follow_zeroed_row`); the spec gates D3 nowhere.
  4. #1217 gate 3's body became `assert_delayed_route_stays_active(model)` and
     `delayed_session_checked(model)` so #1218 gate 2's delayed case reuses it; #1217's test is
     otherwise unchanged and green.
  5. `docs/session-v1.schema.json` carries the output rule as an `if`/`then` on the route object.
- **Re-pins**, each "route field `follows_mute` added, value false" unless stated:
  `COMPLETE_SCHEMA_HASH` `0x39e5_a2c1_d317_a9fe` -> `0x95c1_ceb6_8e44_f6e2` at its four sites (reason
  "route field `follows_mute`, opcode `0507`"); opcode count 42 -> 43 (model test, controller
  fixture count, conformance doc, registry, `complete-schema-manifest.md`), edit limit 41 -> 42;
  `canonical.json` sha256 `0be977ee...` -> `d946f463...` (both `prepare_256_tracks-*.toml`,
  `fixture_builtins.rs`); MANIFEST rows `d5d19ee7...` -> `372831d2...`, `66824cd9...` ->
  `d35b894e...`; MANIFEST digest `6d466903...` -> `867d9004...` (`builtins_graph.rs`,
  `fixture_builtins.rs`); `sessionDocumentBytes` 1926 -> 1955, self-test row 1927 -> 1956; the
  capi vector (deviation 1). `fixtures/graph/` did not move. Writer corpus regenerated by its
  command.
- **Migration diff.** Every changed JSON document (20 plus the writer corpus) parses to its parent
  exactly once `follows_mute` is removed, and every route ends `mute, follows_mute` with
  `follows_mute: false` (script against `HEAD`; 125 routes in documents and inline sessions). The
  derived console fixtures were regenerated by their three commands.
- **Tests and test value** (each mutation applied, run red, reverted):
  - `session/tests/invalid_matrix.rs::route_follows_mute_is_required_and_legal_only_into_a_submix`
    (gates 6, 7): red if the key is optional or non-boolean, if an output route may follow, or if
    the writer drops a follow. Mutations: parse ignores the key -> RED; validation removed -> RED;
    writer writes `false` -> RED.
  - `protocol/tests/console_session_edits.rs::set_route_follows_mute_is_legal_only_on_a_route_into_a_submix`
    (gates 6, 7): red if `0507` can make an output route follow, if the rule were per-edit (the
    re-pointing transaction would refuse), or if the apply arm ignores its value. Mutations: apply
    no-op -> RED; apply always `true` -> RED; validation removed -> RED; edit decode drops the
    value -> RED. Extended `opcode_registry_...` (43, `0x0507`): `from_raw` arm removed -> RED;
    extended `every_route_and_automation_opcode_round_trips_canonically` (a following route, a
    `SetRouteFollowsMute`): route decode drops field 7 -> RED.
  - Gate 5, extended `every_route_coefficient_comes_from_the_one_gated_function` (the nine-track
    fixture plus a `pre_fader` send per track into `bus`, a `bus -> aux` send, random strip mutes
    and random `follows_mute` on submix routes; all four follow shapes drawn): red if the compiler
    and a live producer would compute a follow-muted route by different paths. Mutations: lowering
    ignores the follow -> RED; lanes swapped -> RED; source mutes read from tracks only -> RED.
  - `graph-compiler/tests/route_coefficients.rs::a_following_send_seals_one_route_follow_zeroed_row`
    (D3): red if a follow-zeroed plan shares a digest with an open one. Mutations: no row -> RED;
    lane bits swapped -> RED.
  - Gates 1 and 4, `host-core/tests/route_mute.rs::a_follow_send_zeroes_its_muted_source_lanes_column`
    (8 seeds; track/left, submix/right and the two mirrors; asymmetric drawn matrix, distinct
    noise per lane; bit-identical to the column-zeroed, non-following session; the non-following
    session leaks): red if the flag is ignored, zeroes the wrong column, goes through the gain, or
    reads source mutes from tracks only. Mutations: follow ignored -> RED; lanes swapped -> RED;
    any zeroed lane zeroes all four (the gain path) -> RED; tracks only -> RED (submix case).
  - Gate 2, `...::a_fully_follow_muted_undelayed_send_is_inactive` (16 seeds, #1217's D3 oracle
    and mix counters): red if a fully follow-muted undelayed send is mixed. Mutations: activity
    keyed on `mute` only -> RED (`r0` mixed 8); follow ignored -> RED.
  - Gate 2, `...::a_fully_follow_muted_delayed_send_stays_active` (486-sample compensation asserted
    from the plan): red if a fully follow-muted delayed send is skipped. Mutation: follow-silenced
    delayed route made inactive -> RED (0 mixes against 8); follow ignored -> RED.
  - SDK: `builder-evals.mjs` "an omitted followsMute is true into a submix and false into the
    output, written after mute" and "followsMute: true on a route into the output throws, at the
    route's own path"; `console-evals.mjs` "follow sends from a muted track and a muted bus are the
    engine's canonical JSON, byte for byte"; `enginectl-cli.mjs` "a route request's optional
    followsMute reaches the document, and an output follow is refused": red if the SDK defaults,
    orders, validates or forwards the key wrongly. Mutations: constant default `false` -> RED
    (builder, console); key dropped from the order -> RED; key before `mute` -> RED; destination
    check removed -> RED; boolean check removed -> RED; enginectl key not admitted -> RED; enginectl
    mapping dropped -> RED.
- **Gate 3 (PR evidence, not committed).** A scratch host-core test (removed) rendered 16 quanta of
  one session -- `t0` both lanes muted, `t1` left muted, `t2` open, each with a `post_pan` main
  route and a `pre_fader` send into `verb` (right lane muted), `verb -> echo` from `input`, every
  route `follows_mute: false` -- on this tree and, without the key, on base `071c6c14a` (a detached
  worktree): the two 16,384-byte PCM files are identical (`cmp`), with 4,096 of 4,096 samples
  nonzero.
- **Gates (x86-64-v3 host).**
  - 8: release build ok; `check-graph-determinism.sh` PASS (100/100), and
    `fresh-process-determinism.json` is byte-identical (`cmp`) to the base's; `graph_fixture --
    --check` exit 0; `check-console-fixtures.sh` ok; `check-builtins-fixtures.sh` ok (50 files);
    `cargo test --locked --release -p audit -p bench -p console-workload` 110 passed, 0 failed;
    `audit capi` `pcm_digest ff6cdcb96cdcdad5`, `allocations 0`, `total_violations 0` (unchanged).
  - 10: test-debug-a exit 0 (1,192 passed, 0 failed, 9 ignored; `--no-fail-fast`); test-debug-b
    exit 0 (787 passed, 0 failed, 24 ignored); `check-protocol-wasm-parity.sh` ok (simd128);
    browser `--self-test` ok (32 red mutations) and `--artifacts <A>` ok; `check-sdk-types.sh` ok;
    `check-sdk-headless.sh <A>` 352 passed; `sdk-package.sh check <A>` ok (17 enginectl tests);
    `cargo fmt --all -- --check` ok; clippy `-D warnings` ok; the seven check/test policy pairs ok.
    `run-aarch64-tests.sh debug`: not runnable on this x86 host; at batch push (CI `aarch64-debug`).
    D6's capi gates do not apply.
    `check-web-audioworklet.sh` (inherited row, added by verdict MINOR-3): FAIL on this head,
    identical on parent `071c6c14a` (#1217 BLOCKER-1: eight outlined `reduce_group_into<f32x4, N>`);
    PASS with #1217 attempt 2's `#[inline(always)]` applied over this head.
- **Worktree note.** During this attempt an uncommitted change to `crates/graph/src/runtime.rs`
  marked "issue #1217 attempt 2" (`reduce_group_into` `#[inline(always)]`, a tag-bit check in
  `route_activity`'s index) appeared in the shared worktree. It is not this slice's and is left
  uncommitted; the gates above ran with it present. The committed tree (without it) was rebuilt and
  its `route_mute` and `route_coefficients` tests rerun green in a separate worktree.

### K3 follow-up record (after the attempt 1 PASS verdict)

Applied in the K3 follow-up commit (on `eff44271d`, branch `codex/batch-submix-k3`):

- **MINOR-1.** `every_route_and_automation_opcode_round_trips_canonically`
  (`crates/protocol/src/session_wire/tests.rs`) upserts two routes with swapped booleans
  (`mute: false, follows_mute: true` and `mute: true, follows_mute: false`). Test value: red if the
  codec confuses route fields 6 and 7 in either direction. M10 (`parse_route` reads `follows_mute`
  from `route::MUTE`) and M10b (`tx_route` writes `value.mute` into field 7) are each RED, then
  reverted.
- **MINOR-2.** `docs/SESSION_SCHEMA_V1.md` and this spec's Context say that from a `post_fader` or
  `post_pan` tap the follow changes no audible sample, and that a send with both source lanes
  muted is skipped, so its silent contribution becomes `+0.0` instead of a signed zero.
- **MINOR-3.** The record's gate list carries the inherited `check-web-audioworklet.sh` row.
- **NIT-1.** The lowering `expect`s the source strip ("a validated route source names a strip")
  instead of defaulting to `[false; 2]`.
- **NIT-2.** *Research: render stored session automation* (#1058) gains a coordination note: a
  following send must follow stored mute automation.
- **NIT-3.** `a_following_send_seals_one_route_follow_zeroed_row` adds the case where only the
  follow silences the send (`(true, true, false)`) and asserts that the `estimate` row moves
  exactly then. Test value: red if the activity charge is keyed on the route's `mute` alone rather
  than the gate's `silences()`. That mutation (`estimate.rs`) is RED here and green on every other
  test of `graph`, `graph-compiler` and `host-core`.

## Verdict

- **Attempt 1** (`0da034dba`): Sol PASS. Three MINORs and three NITs, applied above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1218-attempt1.md`; probes `docs/handoffs/submix-sends-2026-10-02/verdicts/1218-attempt1-verifier-scratch.rs`.

## Dependencies

- *Skip an inactive route in its destination's sum* (#1217)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free.
- In-place V1 amendment: append field 7 and opcode `0507`; renumber nothing.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
