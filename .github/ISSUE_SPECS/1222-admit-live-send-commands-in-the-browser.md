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
