# Admit live send commands in the browser

Slice 24 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

In the browser, a producer can ride a send's level, toggle a send, or change its 2x2 live, with the
same declick, the same all-or-nothing admission and the same 48-byte command record as a fader. A
send is addressed by its **live-route index**: its position among the routes into submixes, in
canonical route-ID order.

This slice is the admission path and its mirror. Enumeration and the SDK surface are *Enumerate
sends and drive them from the SDK* (#1223); follow-mute composition is *Let a send follow its source strip's
mute live in the browser* (#1224).

## Context (verified on `fe8ac679`)

**Every `hosts/host-web/src/lib.rs` line number below is pre-K2.** Batch K2 (*Address submix strips
in browser live commands*, #1213) widened the bands to strips and moved the bounds check; re-read the
current lines before editing.

- **The record.** The frozen 48-byte `miso.command.v1` record (`hosts/host-web/src/lib.rs:3820-3852`):
  `kind` (u8, offset 0), `rack` (u8, 1; `255` not applicable), `channel` (u8, 2; `255` not
  applicable), a required zero byte (3), the index word `track_index` (u32, 4), `effect_index` (8),
  `parameter_id` (12), `smoothing_samples` (16), a required zero word (20), `values[4]` (24..40) and a
  required zero `u64` (40).
  - Kinds are 1-12 (`:797-862`); 12 is `COMMAND_INPUT_FILTERS` (`:862`), so 13 is the next free value.
  - Decode: `CommandRecord::decode` (`:3854`), kind whitelist `:3869-3884`.
- **Admission.**
  - `admit_commands` (`:4235`) commits or rolls back the solo mirror (`:4256-4262`).
  - `admit_commands_staged` (`:4276`) decodes each record and, on `fe8ac679`, refuses
    `track_index >= track_count` with `COMMAND_REASON_UNKNOWN_TRACK` before kind dispatch
    (`:4323-4326`); after K2 the generic bound is the strip count. It stages per-slot wants in
    `command_wanted` (`:1400`), checks room in every slot with `queue_available` (`:1605-1625`), and
    then pushes with `push` (`:1629`).
  - The staged variants are `AdmittedCommand::{Matrix, Fader, Input, Effect}` (`:1700-1709`).
- **Queue slots** after K2, with `S` the strip count (tracks, then submixes) and `E` the effect
  count: `[0, S)` matrix/pan, `[S, 2S)` fader/mute, `[2S, 3S)` input, `[3S, 3S + E)` effects.
  `queue_count` is at `:5970-5973` and `command_staging_count` at `:6253-6257`:
  `2 * MAXIMUM_COMMAND_RECORDS + 2 * (strip count)` after K2 (`MAXIMUM_COMMAND_RECORDS = 256`,
  `:570`). A record stages at most two entries, and the solo coalescing pass at most two per strip.
- **Reasons** are 0-12 after K2 (`:926-966`; K2 added 12 `notSoloable`): `COMMAND_REASON_DOMAIN` is
  6 (`:939`), `COMMAND_REASON_BACKPRESSURE` 8 (`:953`). K2 moved the `COMMAND_REASON_FUTURE_TAP` drift
  mutation to 13 (`scripts/check-command-reason-vocabulary.py:332`, `scripts/test-web-audioworklet.sh:245`,
  `hosts/host-web/MUTATIONS.md:132`) and re-pointed `scripts/check-parameter-metadata-v1.py:637`
  (the "reason value renumbered" mutation) at reason 12.
- **Every spelling of a kind** (`scripts/check-command-kind-vocabulary.py` checks the first seven):
  1. `COMMAND_*` (`lib.rs:797-862`);
  2. the decode whitelist (`:3869-3884`);
  3. host JS `COMMAND_KINDS` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:257`);
  4. the `.d.ts` `MisoCommandKind` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:188`)
     and its byte-identical mirror `sdk/src/browser/shipped-host.d.ts`;
  5. `tools/parameter-metadata/src/lib.rs:154-182` (`applied`, `plane` columns);
  6. `scripts/check-parameter-metadata-v1.py:34-37`;
  7. the shipped metadata JSON;
  - also `tools/parameter-metadata/src/abi_layout.rs` (`wire_command_kinds`, `:1743-1756`), the
    self-test fixtures `scripts/fixtures/abi-layout-v1-self-test.json:615` (`wireCommandKinds`) and
    `scripts/fixtures/parameter-metadata-v1-self-test.json:6-18` (`commandKinds`), and the applied-kind
    prose at `miso-engine-v1-audio-worklet-host.d.ts:20`.
- **Every spelling of a reason** (`scripts/check-command-reason-vocabulary.py`): the Rust constant,
  host JS `COMMAND_REASONS` (`worklet-host.js:223`), the `.d.ts` `MisoCommandReason` (`:283`) and its
  mirror, the metadata generator rows, the schema-gate list, `abi_layout.rs` (`command_reasons`,
  around `:1758-1770`), `scripts/fixtures/abi-layout-v1-self-test.json:616` and
  `scripts/fixtures/parameter-metadata-v1-self-test.json:20-33`.
- **Self-test anchors that a kind 13 breaks.**
  - `check-command-kind-vocabulary.py` anchors its mutations on the literal
    `new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])` (`:432`, `:440`, `:448`) and on `<= 12`
    (`:457`). So does `scripts/test-web-audioworklet.sh:277`.
  - Its "added and not threaded" mutation inserts `COMMAND_SOLO_MODE: u32 = 12`
    (`check-command-kind-vocabulary.py:389-396`; `test-web-audioworklet.sh:295`, inserted after the
    `COMMAND_POLARITY_INVERT = 11` line). That value already ships as `INPUT_FILTERS`, so today the
    mutation turns red only through the contiguity rule; its comments are stale (`:384-388`;
    `.sh:289-291`).
  - The "set gains a kind the wire does not decode" mutation uses 13 (`:446-450`).
- **The native harness** is `hosts/host-web/src/tests.rs`, with `render_pair_and_compare` (`:6681`).
  Its `feed_and_render` (`:2667-2684`) feeds one constant to identical L and R planes of one shared
  source, which hides column, lane, route-index and delay-alignment errors (VERIFY-2 M13).
  `scripts/test-web-audioworklet.sh` is hermetic: vocabulary, layout and drift gates over a stubbed
  module.
- **After *Produce live send records from host-core* (#1221):** `HostLiveControlHandles.route_controls` holds
  one `RouteControlProducer` per route into a submix, in canonical route-ID order, with `free()`;
  `record(gain_db, matrix, mute, source_lane_muted, length)` is pure (`Domain` or `Length`), and
  `push(record)` refuses `Full` with nothing pushed. host-core re-exports `RouteControlRecord`.
- **After batch K2:** the index word of strip kinds is a strip index; `LiveControlSoloState` is
  sized per strip, with solo-safe submixes, and `effective_mute(strip, lane)` is the one composition.
- **After *Let a route into a submix follow its source strip's mute in the session* (#1218):** routes carry
  `follows_mute` (only into a submix), and a route's prepared `follow_zeroed` is its source strip's
  session `[left_mute, right_mute]` when it follows.

## Decisions frozen for this slice

- **D1. Kinds** (in-place V1 amendment; append, never renumber):
  - 13 `routeGainDb`: `values[0]` is the gain in dB;
  - 14 `routeMute`: `values[0]` is exactly 0 or 1;
  - 15 `routeMatrix`: `values[0..4]` are `ll, lr, rl, rr`.

  For all three, `rack` and `channel` are `255`, `effect_index` and `parameter_id` are 0, and the
  `values` words beyond the kind's own must be `0.0`; anything else is `MALFORMED`, as the fader
  kinds require. `smoothing_samples` is the ramp length. If 13-15 are taken at implementation, take
  the next free numbers and say so in the PR.
- **D2. Addressing and bounds.**
  - For kinds 13-15 the index word is a **live-route index**.
  - The generic bounds check moves **after** kind dispatch: strip kinds are checked against the strip
    count, route kinds against `route_controls.len()`.
  - A route index out of range refuses with a new reason 13, `unknownRoute`. Reusing the track reason
    would misname the address for an agent.
- **D3. The mirror.** `host_core::LiveRouteState`, one entry per live route, in `route_controls`
  order:
  - `gain_db`, `matrix`, `mute`, initialised from the session;
  - `follows_mute`, fixed for the plan;
  - `source_strip: usize`, the strip index (tracks, then submixes) of the route's source, fixed for
    the plan (VERIFY-2 MINOR 9; *Let a send follow its source strip's mute live in the browser* needs
    it);
  - `source_lane_muted: [bool; 2]`, initialised at preparation from the strip-mute state's
    `effective_mute(source_strip, lane)` when `follows_mute` is set, else `[false; 2]`. Solo is never
    persisted, so at preparation this equals the session fader mutes the compiler used.
  - **The constructor is frozen** (VERIFY-3 MINOR 7): it takes the routes and
    `effective_mute: &dyn Fn(usize /* strip */, usize /* lane */) -> bool`, as
    `LiveRouteMuteFollow::delta` does, never host-web's solo state, so *Let C ABI sends follow their
    source strip's mute live* (#1226) can seed it from the committed model's mutes without changing it.

  It has a shadow, `commit` and `rollback` exactly like `LiveControlSoloState`
  (`crates/host-core/src/solo.rs:257-283`), and host-web commits or rolls it back where it commits or
  rolls back the solo mirror.
- **D4. Lowering.**
  - A route kind updates its mirror field, then calls `RouteControlProducer::record` with the full
    mirror and the record's `smoothing_samples` as `length`.
  - A `Domain` or `Length` error refuses with `COMMAND_REASON_DOMAIN` at that record's index, and
    nothing is staged.
  - The record is staged as `AdmittedCommand::Route(RouteControlRecord)` on slot `3S + E + route`.
    `queue_count` grows by the live-route count; `queue_available` and `push` gain the route band,
    using `free()` and `push`.
  - `command_staging_count` does **not** grow here: a route command stages exactly one entry, inside
    the existing `2 * MAXIMUM_COMMAND_RECORDS` term. *Let a send follow its source strip's mute live
    in the browser* grows it for follow records (VERIFY-2 MINOR 9).
  - Several route kinds on one route in one batch update one mirror and stage one record per kind,
    in wire order; the render plane applies them in order, so the last one wins.
- **D5. No route metadata family.** The parameter-metadata `routes` family and a tighter route
  domain wait on owner question Q2 (DESIGN deferred item O11). The live domain is exactly the
  prepared one, because both go through `route_coefficients`.

## Deliverables

1. host-core `LiveRouteState` (D3), in a new module exported from `lib.rs`, with unit tests for
   construction, shadow, commit and rollback.
2. host-web: D1, D2 and D4: the constants, decode whitelist, admission, staging, room check, push,
   queue sizing and the mirror's commit and rollback.
3. Every spelling of the three kinds and of reason 13 listed in Context, and the regenerated
   `sdk/assets/**` and `sdk/src/generated/**` (`node codegen/assets.mjs && node codegen/generate.mjs`
   in `sdk/`).
4. The self-tests, re-anchored:
   - the kind-set literal and `<= 12` become `[1 … 15]` and `<= 15`, at
     `check-command-kind-vocabulary.py:432`, `:440`, `:448`, `:457` and
     `scripts/test-web-audioworklet.sh:277`;
   - the "added and not threaded" mutation becomes `COMMAND_SOLO_MODE: u32 = 16`, inserted after the
     new last kind's line, at `check-command-kind-vocabulary.py:389-396` and
     `test-web-audioworklet.sh:295`, with the stale comments (`:384-388`, `.sh:289-291`) rewritten;
   - the "set gains a kind the wire does not decode" mutation uses 16;
   - the `COMMAND_REASON_FUTURE_TAP` drift mutation moves from 13 to 14, anchored on the new last
     reason, at `check-command-reason-vocabulary.py`, `test-web-audioworklet.sh` and
     `hosts/host-web/MUTATIONS.md`;
   - the "reason value renumbered" mutation at `check-parameter-metadata-v1.py:637` re-points at
     reason 13.
5. `hosts/host-web/MUTATIONS.md` rows: the moved bounds check; each new kind; the mirror committed
   before the room check (red).

## Authorized paths

- `crates/host-core/src/` (one new module, plus `lib.rs`) and `crates/host-core/tests/`
- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/**`
- `scripts/check-command-kind-vocabulary.py`, `scripts/check-command-reason-vocabulary.py`,
  `scripts/check-parameter-metadata-v1.py`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/fixtures/parameter-metadata-v1-self-test.json`,
  `scripts/test-web-audioworklet.sh`, `scripts/test-web-audioworklet.mjs` (only where it spells kinds
  or reasons)
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only), `sdk/src/browser/shipped-host.d.ts`
- this spec

## Non-goals

- No export, `SessionMap` entry or SDK `route(id)` API (*Enumerate sends and drive them from the
  SDK*).
- No follow-mute composition: a strip mute does not yet move a send.
- No live output routes. No C ABI path.
- No parameter-metadata `routes` family and no domain tightening (owner question Q2).

## Hazards

- **The acked-batch question.**
  - A batch mixing strip, effect and route records checks room in **every** destination queue
    before any push.
  - The mirror changes only when the batch commits.
  - A refused batch leaves mirrors, queues and replay unchanged.
- **The moved bounds check.** Until it moves, a valid route index at or past the strip count is
  refused as an unknown track, and a route index below it would pass the generic check against the
  wrong vector. Gate 3 covers both.
- **Mirror and fresh plan.** After a ramp, the render must equal a plan prepared from a session that
  holds the mirror's values. Gate 1 is that check.
- **Vocabulary churn.** A missed spelling fails the kind or reason gate, and a missed self-test
  anchor makes a mutation "match nothing", which fails the self-test. Both are intended; fix them,
  never silence them.

## Objective gates

1. **A live send edit lands on a fresh plan's bits** (native harness, `hosts/host-web/src/tests.rs`).
   - The session has four tracks reading distinct source channels, fed a per-sample pattern distinct
     per channel and per lane (add a feeding helper; `feed_and_render`'s one constant hides the
     defects this gate claims), and two buses with sends from three of the tracks, so the edited
     routes reach different buses. Send matrices are asymmetric (`ll != rr`, `lr != rl`, all
     nonzero). The buses have no console slots, no stateful inserts and an identity input section.
   - Kinds 13, 14 and 15 are sent at random live routes, with smoothing 0 and 480.
   - After the ramp, the output is bit-identical to a host booted from a session holding the mirrored
     values and fed the same sources (`render_pair_and_compare`'s pattern, with the new feeder).
   *Test value: it turns red if a spelling or the index addresses the wrong route, the matrix words
   are read in another order, or the mirror sends a stale field. No browser path has driven a route.*
2. **All or nothing.**
   - A batch of one valid `routeGainDb` and one `routeMatrix` with a non-finite coefficient stages
     nothing, refuses with `DOMAIN` at the second record's index, and leaves the mirror unchanged.
   - A batch that overfills one route queue is typed backpressure: no push, mirror unchanged.
   - A batch of a valid route record and a valid fader record with one full fader queue pushes
     neither.
   *Test value: it turns red if admission pushes, or commits the mirror, before every check passes.*
3. **Addressing.**
   - Every existing host-web live-control test passes unchanged.
   - A strip kind at or past the strip count refuses with `UNKNOWN_TRACK`.
   - A route kind at or past the live-route count refuses with `unknownRoute`, including an index
     below the strip count.
   *Test value: it turns red if the bounds check runs before kind dispatch or against the wrong
   count.*
4. **The mirror** (host-core unit tests): construction seeds `source_strip` and `source_lane_muted`
   from a session with a muted follow source; a refused batch's `rollback` restores every field; a
   committed batch's `commit` keeps them.
   *Test value: it turns red if the mirror starts from values other than the prepared plan's, or a
   refused batch leaks a field into the next one.*
5. **Vocabulary, metadata and layout** (each script with its argument, DESIGN section 7):
   - `python3 -B scripts/check-command-kind-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-kind-vocabulary.py`
   - `python3 -B scripts/check-command-reason-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-reason-vocabulary.py`
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` and
     `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/test-web-audioworklet.sh` (each moved mutation still red, through the rule it
     names)
   - `bash scripts/check-sdk-generated.sh <A>`
6. **Browser artifact and policy:**
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (the shipped module's
     direct oracle, unchanged)
   - `bash scripts/check-sdk-types.sh` and `bash scripts/check-sdk-headless.sh <A>`
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-realtime-policy.sh`
   - the test-debug-a command (DESIGN section 7)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
7. **Render allocates nothing** while route edits are submitted, measured as in the host-web
   live-control allocation tests: `crate::ffi::live_response_ffi_tests::measured` (the pattern at
   `hosts/host-web/src/tests.rs:3638-3647`; host-web has no `bench-support`), after warm-up.
   *Test value: it turns red if admitting a route record or committing the route mirror allocates on
   the render-call path.*

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name, with its one-sentence test-value answer.
- The re-anchored mutation list, with each mutation's observed red result.
- Any digest or artifact re-pin, with its reason.

### Attempt 1 record (Terra)

Anchors re-found by symbol on `1c5ce5d02` (#1221's head). `route_id()` is a method there, not a
field.

**Implementation.**

- `crates/host-core/src/live_route_state.rs` (new, exported): `LiveRoute`, `LiveRouteState`,
  `LiveRouteStateError`.
  - `LiveRouteState::live_routes(model)` is the one selection of live routes: routes into a submix,
    in the normalized model's order, which is canonical route-ID order.
  - `try_new(model, effective_mute: &dyn Fn(usize, usize) -> bool)` seeds `gain_db`, `matrix`,
    `mute`, `follows_mute`, `source_strip` (tracks, then `T + j`) and `source_lane_muted`.
  - The setters shadow once per transaction; `commit` and `rollback` work as in
    `LiveControlSoloState`. `empty()` serves a plan with no live route.
- `hosts/host-web/src/lib.rs`:
  - Kinds 13 `COMMAND_ROUTE_GAIN_DB`, 14 `COMMAND_ROUTE_MUTE` and 15 `COMMAND_ROUTE_MATRIX` are
    in the decode whitelist. Reason 13 is `COMMAND_REASON_UNKNOWN_ROUTE`.
  - D2: the generic strip bound runs only for non-send kinds. A send kind is bounded in its own
    arm by the send queues and the mirror (`unknownRoute`).
  - D4: `into_route_edit` checks the shape (`MALFORMED`), and `routeMute` takes exactly 0 or 1
    (`DOMAIN`).
    - The record is built from the mirror with the one field replaced, through
      `RouteControlProducer::record`. `Domain` and `Length` map to `DOMAIN`.
    - The mirror is updated only after the record is built. It is staged as
      `AdmittedCommand::Route` on slot `3S + E + route`.
    - `queue_count` grows by the live-route count. `queue_available` uses `free()`, and `push`
      uses `push`. `command_staging_count` is unchanged.
  - `ReadyOwnership` keeps `route_controls`, declared before the plan, and `routes`. At boot it
    checks the producers' IDs against `live_routes(model)` in order (`web.live_controls.routes`).
    The mirror is seeded from the solo state's `effective_mute`. `admit_commands` commits or rolls
    back the mirror where it does so for solo.
- Every spelling: the host JS set and table, both `.d.ts` copies (enum members, the applied-kind
  prose, now fifteen, and the `trackIndex`, `smoothingSamples` and `values` docs), both
  generators, both schema-gate lists, both self-test fixtures, and the regenerated `sdk/assets/**`
  and `sdk/src/generated/{abi,catalog}.ts`.
- Re-anchored self-tests (deliverable 4):
  - In `check-command-kind-vocabulary.py`, the set literal is `[1 … 15]` and `<= 15`. The
    "added and not threaded" mutation is `COMMAND_SOLO_MODE: u32 = 16`, after
    `COMMAND_ROUTE_MATRIX`, and its comment is rewritten. The undecoded-kind mutation uses 16. The
    two "added last" mutations now drop `routeMatrix`.
  - `test-web-audioworklet.sh` has the same set literal and the same 16 mutation, with its comment
    rewritten.
  - `FUTURE_TAP` moves to 14, after `UNKNOWN_ROUTE = 13`, in the script, the shell test and
    `MUTATIONS.md`. The reason self-test's renumber targets (`UNKNOWN_TAP`, worklet
    `UNSUPPORTED_KIND`) and its `<= 13` bound move to the next free value too.
  - The "reason value renumbered" mutation in `check-parameter-metadata-v1.py` is now
    `commandReasons[13] → 14`.
  - Observed reds (in-process trace): the 16 kind is caught by `.d.ts MisoCommandKind disagrees
    with the Rust host constants`, the threading rule, not contiguity. The undecoded 16 is caught
    by `host JS COMMAND_KINDS set disagrees`, and `FUTURE_TAP = 14` by `host JS table disagrees`.
    The kind self-test has 32 red mutations, the reason self-test 20, layout 22, and metadata
    passes.

**Deviations.**

1. **The `free` callgraph rule** (outside the authorized paths). `check-web-audioworklet.sh`
   failed. `miso_engine_web_v1_command_submit`'s allocation-only closure reached
   `RouteControlProducer::free` and `GraphRouteControlProducer::free`, and
   `check-web-audioworklet-callgraph.py` forbids any function whose name contains `free`, as
   dlmalloc's own `4free` does. Both one-line methods are now `#[inline(always)]`, with a comment
   saying why: `crates/graph/src/lib.rs` and `crates/host-core/src/route_controls.rs`. There is no
   API or behaviour change. Renaming `free` would ripple into #1223 to #1226's specs.
2. **The SDK vocabulary test** (`sdk/test/live-controls-evals.mjs`, outside the authorized paths).
   `check-sdk-headless.sh` failed. "all twelve command kinds are built by name" requires the
   semantic methods to cover the wire vocabulary, and the send methods are #1223's (non-goal
   here). The test now names `routeGainDb`, `routeMute` and `routeMatrix` as awaiting #1223, so
   any other kind without a method still fails it. **#1223 must move them into `kindNames`.**
3. **Spec amendment from the #1221 verdict, MINOR-1** (root-routed, `hosts/host-web/src/lib.rs`).
   The browser's exact retained rows now charge `engine.route_control_resources.total_bytes`, plus
   the mirror and its shadow, in `bridge_metadata_bytes` and `bridge_retained_bytes`.
   `largest_allocation_bytes` is folded into both largest rows. The producer `Vec` is kept as
   host-core built it, so its table bytes, already inside the total, are counted once.
4. **Gate 2's "non-finite coefficient".** `CommandRecord::decode` refuses any non-finite value
   word as `MALFORMED` before dispatch, so a non-finite coefficient can never reach `DOMAIN`. The
   gate uses a finite `ll = 3e38` on `send-b`, which is at +2 dB. The fold overflows, and
   `route_coefficients` refuses it as `Domain`.
5. **Constructor shape** (D3 says "takes the routes"). It takes the normalized `SessionModel`, so
   that the live-route selection and the source-strip resolution are written once, in host-core,
   and #1226 can seed it from capi's committed model. `effective_mute` is exactly the frozen
   `&dyn Fn(usize, usize) -> bool`.
6. With no live controls, a send kind would be `unsupportedKind`. That is the strip kinds' rule,
   but the arm is defensive and unreachable: a control-free host has no staging buffer and refuses
   every submission first.

**The acked-batch question, plan swap and cross-route fence** (root notes from #1220 and #1221).

- Admission decodes and validates every record, builds every send record, and updates the mirror
  under its shadow. It then room-checks every staged entry's queue (strip, input, effect and send
  bands) before the first push, and commits the mirrors only after every push. Any refusal rolls
  back the solo state, the send mirror and the input shadows. No ack precedes a drop.
- **Cross-route fence.** `miso_engine_web_v1_command_submit` runs in the worklet's
  `port.onmessage`, on the render thread between `process()` calls. So a batch is fully pushed
  before the next render starts, and nothing pushes during a render. Every live route drains
  `available_at_entry()` at its op's start, every block (#1220 D4), so all of one batch's records
  land in the next block: strip and send records alike, across routes. The one-block skew #1220
  noted needs a concurrent producer, and the browser has none.
- **Plan swap.** The browser host has no plan swap. One `ReadyOwnership` holds the plan for the
  host's life, and `route_controls` is a field of it, dropped with the plan. A queued, acked send
  record can never be stranded by a retirement.

**Tests** (rows in `hosts/host-web/MUTATIONS.md`, "Issue #1222", and
`crates/host-core/tests/MUTATIONS.md` 1222-H1 to H4; each was applied, run red and reverted).

- `live_route_state::tests::construction_seeds_every_field_from_the_session_and_the_effective_mute`
  (gate 4). Red if the mirror keeps an output route, resolves a submix source without the track
  offset, or seeds following lanes from the session instead of the effective mute (H1, H2, H4).
- `live_route_state::tests::a_rollback_restores_every_field_and_a_commit_keeps_them` (gate 4).
  Red if a refused batch leaks a field into the next one, through a shadow not retaken per
  transaction (H3).
- `tests::a_live_send_edit_lands_on_a_fresh_plans_bits` (gate 1).
  - The session has four tracks with distinct per-lane feeds (`strip_planes`), two unity buses
    and seven sends: three tracks into both buses, plus `bx` into `by`. The sends are declared out
    of ID order, the matrices are asymmetric and nonzero, and send index 6 is past the 6 strips.
  - It runs 4 trials × {0, 480}, with kinds 13, 14 and 15 at random sends in two batches. It
    compares bits against a host booted with the edited values.
  - Red if the record's matrix words are reordered (`M1b`), a record reaches another send's queue
    (`M2`), or the mirror is stale or decoded wrong (`M1`, `M3`).
- `tests::a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (gate 2). Red if the mirror is
  committed before the room check (the required row), the send band is not room-checked, or a send
  record is pushed in pass one (`M6`, `M7`, `M8`).
- `tests::every_kind_is_bounded_by_the_count_it_addresses` (gate 3). It covers 7 sends and 6
  strips, and 3 sends and 4 strips (index 3 and the bus's strip index). Red if the generic bound
  runs before dispatch, a send reuses `unknownTrack`, or the send arm is bounded by the strip
  count (`M4`, `M5`, `M12`).
- `tests::send_records_are_shape_checked` (D1). Red if a send accepts a lane, rack, effect or
  parameter word, a value past its own, a non-boolean mute, or a ramp past `1 << 22` (`M10`).
- `tests::send_edits_admit_and_render_without_allocating` (gate 7, `ffi::live_response_ffi_tests::
  measured` after a warm-up round). Red if send admission or the mirror allocates (`M9`).
- `tests::the_exact_retained_budget_charges_the_send_lanes` (amendment 3).
  - Between depths 8 and 64, `bridge_retained` and `bridge_metadata` move by exactly the
    independent host-core preparation's `route_control_resources.total_bytes` delta.
  - A send-free session does not move at all.
  - The exact total is admitted, and one byte below it is refused with
    `host.budget.retained_exact`.
  - Red if the lanes go uncharged (`M11`).
- A first-draft second gate-3 test had no unique catch: its mutation also reddened the test
  above. It was folded into that test.

**Gates** (this commit, on base `1c5ce5d02`; x86-64-v3 AVX2).

- Gate 5:
  - kind vocabulary: `--self-test` rc 0 (32 red); plain and `--artifacts` rc 0;
  - reason vocabulary: `--self-test` rc 0 (20 red); plain rc 0;
  - parameter metadata: `--self-test` rc 0; `<A>/…parameter-metadata.json` rc 0;
  - ABI layout: `--self-test` rc 0 (22 caught); `<A>/…abi-layout.json` rc 0;
  - `test-web-audioworklet.sh` rc 0, with each moved mutation red as above;
  - `check-sdk-generated.sh <A>` rc 0.
- Gate 6:
  - `build-web-audioworklet.sh --named-twin` rc 0. The shipped module is `2daf0f83…`, 2,762,489
    bytes, against 2,755,262 in #1221's record (+7,227).
  - `check-web-audioworklet.sh` rc 0. The kernel shape is 13 kernels, `f32x4_arith=9396`.
  - `check-browser-expected-resources.py --artifacts` rc 0, with the digests and exact rows
    agreeing and 32 red self-test mutations. `check-scalar-oracle-absent.py --wasm` rc 0.
  - `check-sdk-types.sh` rc 0. `check-sdk-headless.sh <A>` rc 0, after deviation 2.
  - `check-host-core-policy.sh`, `test-host-core-policy.sh`, `check-realtime-policy.sh` (54
    regions) and `check-workspace-policy.sh`: all rc 0.
  - test-debug-a (DESIGN 7, `--no-fail-fast`): rc 0. 1,221 passed, 0 failed and 9 ignored, over
    108 binaries. That run had a separate, later-folded gate-3 test; the folded tree re-ran its
    new host-web and host-core tests green.
  - `cargo fmt --all -- --check` rc 0.
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` rc 0.
- Gate 7: `send_edits_admit_and_render_without_allocating` passes, with 0 allocations and 0 frees.
- **No re-pin.** `check-browser-expected-resources` is unchanged (none of its sessions has a send
  under live controls). The module digest is not a per-change pin (#1061).

### K3 follow-up record (after the attempt 1 PASS verdict)

Applied in the K3 follow-up commit (on `eff44271d`, branch `codex/batch-submix-k3`), in `hosts/host-web/src/tests.rs`. Each mutation was applied alone to
`hosts/host-web/src/lib.rs`, the host-web lib suite run, and the file restored.

- **MINOR-1/2 (probes 1 and 2).** New `a_following_sends_seeded_source_lanes_land_on_a_fresh_plans_bits`:
  following pre-fader sends from sources with all four lane-mute combinations and a bus with one
  muted lane, run without effects and with an insert on every track (`E > 0`), edited live; each
  send's queue room drops by exactly its own records and the output settles on a fresh plan's
  bits. Test value: red if a send record is built without the mirror's `source_lane_muted`; no other
  test edits a send whose source lanes start muted.
- **MINOR-2/3 (probes 3 and 4)**, folded into gate 2's
  `a_refused_send_batch_pushes_nothing_and_keeps_the_mirror`: a send record plus an effect record
  whose queue is full push neither (refused at index 1); and an admitted gain, then a refused
  batch, keeps the admitted mirror, and a further edit lands on fresh-plan bits.
- **MINOR-4.** `the_exact_retained_budget_charges_the_send_lanes` adds an absolute check (fix
  (a)): the send document against a twin with one send removed, at one depth, differs in
  `bridge_retained_bytes` by exactly the document and ID-staging rows, one staged command, the
  session-model bytes, the route-control resources and `2 * size_of::<LiveRoute>()`.
- **MINOR-5.** The solo paragraph is back on `into_solo_request`; `into_route_edit` keeps its own.
- **Mutations.** V1 (mirror seeded with `|_, _| false`): RED in the new test and #1224's
  `a_one_lane_mute_follows_into_its_own_source_column`. V8 (record built with `[false; 2]`): RED in
  the new test only. V2 (send band without the effect offset): RED in the new test, gate 2 and
  #1224's `a_delayed_send_follows_like_an_explicit_send_mute`. V6 (mirror not committed): RED in
  gate 2 and #1224's `a_full_send_queue_refuses_the_strip_mutes_it_follows`. V3 (only queue bytes
  charged) and V4 (mirror and shadow dropped): RED in the budget test. Probes 2-4 have no unique
  catch over #1224's tests, so they are folded rather than added as separate tests.
- **Root ratification.** The out-of-path `#[inline(always)]` on the two `free()` accessors is
  ratified; *Anchor the worklet callgraph checker's C allocator names* (#1234) removes them.
- **NIT-1 to NIT-3.** Not applied (optional).

## Verdict

- **Attempt 1** (`466f0ab63`): Sol PASS. Five MINORs and three NITs; the MINORs are applied above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1222-attempt1.md`; probes `docs/handoffs/submix-sends-2026-10-02/verdicts/1222-attempt1-verifier-scratch.rs`.

## Dependencies

- *Produce live send records from host-core* (#1221)
- *Drive submix strips from the SDK live controls* (#1214), for the strip-index admission batch K2 left

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Admission is all or nothing. A refusal is decided before anything is pushed. No ack precedes a
  drop.
- In-place V1 amendment: append kinds and reasons; never renumber or reuse.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
