# Enumerate sends and drive them from the SDK

Slice 25 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

An app or agent using the TypeScript SDK addresses a send by its route ID:

```ts
controls.edit.route("kick-verb").gainDb(-12)
```

It rides the send live with `.gainDb(db)`, `.mute(on)` and `.matrix({ll, lr, rl, rr})`, each with an
optional smoothing. The engine publishes which routes are live (the routes into submixes) in
canonical order, so the SDK never guesses an index. A route into the output refuses in the SDK with
a message saying output routes are not live.

## Context (verified on `fe8ac679`)

Batch K2 (*Name submix strips in the browser session map and the SDK measurement*, #1210) added the
`_submix_count`/`_submix_id` exports, `SessionMap.submixes`, the `submixes` key in the
`miso.sessionmap.v1` reply, `HostSessionShape.longest_submix_id_bytes`, and grew every export list and
session-map stub listed below by its own entries; re-read the current lines and counts.

- **Enumeration today.**
  - Exports `miso_engine_web_v1_live_control_track_count` and `_track_id`
    (`hosts/host-web/src/ffi.rs:3960`, `:3971`, the latter copying through
    `copy_live_control_track_id` into the ID staging buffer).
  - The worklet reads them (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:596-611`) and
    answers `miso.sessionmap.v1` (`:945-958`).
  - The host JS checks the reply's exact field list (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:980`)
    and its field types (`:1050`); the typings declare it (`MisoSessionMap`,
    `miso-engine-v1-audio-worklet-host.d.ts:385`), mirrored byte for byte in `sdk/src/browser/shipped-host.d.ts`.
  - The SDK reads the exports in `shape()` (`sdk/src/core/boundary.ts:459-464`) and returns
    `SessionMap` (`:252-256`, built at `:533-541`); the browser path builds its own `SessionMap` from
    the reply (`sdk/src/browser/live-controls.ts:46-52`).
- **The ID staging buffer** is sized from the longest ID it may hold:
  `max(longest_source_id_bytes, longest_track_id_bytes)` on `fe8ac679`
  (`hosts/host-web/src/lib.rs:1918-1925`, from `HostSessionShape`, `crates/host-core/src/shape.rs:62-81`),
  and after K2 also `longest_submix_id_bytes`. A route ID longer than all of them would overrun
  `copy_id_into_staging` (`lib.rs:2810`) and trap the module (VERIFY-2 M7). In
  `hosts/host-web/tests/browser-v1/session.json` the route ID `track-main` (10 bytes) is shorter than
  the source ID `fixture-source` (14), so `expected.json`'s `idStagingBytes` does not move.
- **Every export-list spelling** (VERIFY-2 M8):
  - `scripts/check-web-audioworklet.sh` (`expected_exports`, `:203-320`), checked against the shipped
    layout JSON's `exports` and against `wasm-objdump -x` of the module (`:322-355`);
  - `scripts/check-abi-layout-v1.py` `EXPORTS` (`:115` onward; must equal the layout exactly,
    `:432-434`) and its self-test fixture `scripts/fixtures/abi-layout-v1-self-test.json`;
  - `tools/parameter-metadata/src/abi_layout.rs` `EXPORTS` (`:132`, a fixed-size `[&str; N]` that
    grows by two);
  - the stub module in `scripts/test-web-audioworklet.mjs` (its `miso.sessionmap.v1` reply at
    `:178-189`, the track export pair at `:2191-2197`).
- **Every session-map shape spelling** (VERIFY-2 M8): `scripts/check-session-map-shape.py:428-429`
  (a mutation on the exact reply list), `scripts/test-web-audioworklet.sh:326` (the same mutation in
  `sed` form), `hosts/host-web/qualification/qualification.js:357`, and the SDK stubs that construct
  a `SessionMap` or a reply: `sdk/test/console-evals.mjs:893`, `sdk/test/browser-defaults-evals.mjs:439`,
  `sdk/test/spectrum-browser-evals.mjs:349`, `sdk/test/measurement-evals.mjs:277`,
  `sdk/test/live-controls-evals.mjs:139`, `:260`, `:367`, and `sdk/test/live-controls-types.ts:64-68`.
- **SDK live edits.**
  - `LiveControlEdits` (`sdk/src/core/live-controls.ts:398-422`, constructor `(map, session?)` at
    `:402`) maps `SessionMap.tracks` to indices; `track(id)` throws `MisoUsageError` for an unknown ID.
  - `TrackEdits` (`:423-610`) builds 48-byte records. After K2, `LiveControlEdits.submix(id)` exists.
- **Real-wasm evals.** `bash scripts/check-sdk-headless.sh <A>` runs every `sdk/test/*-evals.mjs`
  against the shipped module. `sdk/test/live-controls-evals.mjs` boots the real wasm through
  `sdk/test/support.mjs`.
- **After *Admit live send commands in the browser* (#1222):** kinds 13 `routeGainDb`, 14 `routeMute` and
  15 `routeMatrix` exist; their index word is the live-route index (the position among routes into
  submixes, in canonical route-ID order, which is `HostLiveControlHandles.route_controls`' order);
  an out-of-range index refuses with `unknownRoute`; `rack` and `channel` are 255.
- **The app handoff** is `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`, created by *Drive
  submix strips from the SDK live controls* (#1214).

## Decisions frozen for this slice

- **D1. Exports.**
  - `miso_engine_web_v1_live_control_route_count(handle) -> u32` returns the live-route count.
  - `miso_engine_web_v1_live_control_route_id(handle, index) -> u32` copies the route ID into the ID
    staging buffer the track and submix ID exports use.
  - Both enumerate exactly `route_controls`, in order. A host without live controls reports 0.
  - Both are appended to every export-list spelling (Context).
- **D2. Staging** (VERIFY-2 M7). `HostSessionShape` gains `longest_route_id_bytes` (over every route
  of the session), and the ID staging buffer is sized from the maximum of the source, track, submix
  and route lengths.
- **D3. The session map.**
  - The worklet's `miso.sessionmap.v1` reply and the SDK's `SessionMap` gain
    `routes: readonly string[]`: the live routes, in canonical route-ID order.
  - The reply's exact field list grows by `routes`, and every shape check and stub in Context moves
    with it.
- **D4. SDK API.**
  - `LiveControlEdits.route(routeId)` returns `RouteEdits`, with `.gainDb(db, options?)`,
    `.mute(on, options?)` and `.matrix({ ll, lr, rl, rr }, options?)`, where `options` carries
    `smoothingSamples` as the strip edits' do. Each encodes its kind at the route's index in
    `SessionMap.routes`, with `rack` and `channel` 255.
  - An ID that is not in `SessionMap.routes` throws `MisoUsageError`. When the SDK was given the
    session (`withSession`) and the ID names a route into the output, the message says output routes
    are not live; otherwise it lists the live routes.
- **D5. Typings.** `RouteEdits`, `SessionMap.routes` and the two exports are added to the `.d.ts` and
  its SDK mirror, byte-identically.
- **D6. App handoff.** A "Live sends" section in `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`:
  sends are live in a live-controlled engine; how to address a send by route ID; output routes and
  `follows_mute` stay structural.

## Deliverables

1. host-core: D2 in `crates/host-core/src/shape.rs`.
2. host-web: D1, the staging size (D2), and the worklet's D3.
3. SDK: D3's readers (`boundary.ts`, `sdk/src/browser/live-controls.ts`), D4 and D5, and the
   regenerated `sdk/assets/**` and `sdk/src/generated/**` (`node codegen/assets.mjs && node codegen/generate.mjs`
   in `sdk/`).
4. The export and session-map spellings and stubs listed in Context.
5. Evals in `sdk/test/live-controls-evals.mjs` and type tests in `sdk/test/live-controls-types.ts`
   (gates below).
6. D6.

## Authorized paths

- `crates/host-core/src/shape.rs` and `crates/host-core/tests/` (one shape test)
- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`,
  `hosts/host-web/qualification/qualification.js`
- `scripts/check-web-audioworklet.sh` (the export list only), `scripts/check-abi-layout-v1.py`
  (`EXPORTS` only), `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/check-session-map-shape.py`, `scripts/test-web-audioworklet.sh`,
  `scripts/test-web-audioworklet.mjs`
- `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS` only) and its tests
- `sdk/src/core/{live-controls.ts,boundary.ts}`, `sdk/src/index.ts` (exports only),
  `sdk/src/browser/**`, `sdk/assets/**` and `sdk/src/generated/**` (regenerated only)
- `sdk/test/{live-controls-evals,console-evals,browser-defaults-evals,spectrum-browser-evals,measurement-evals}.mjs`,
  `sdk/test/live-controls-types.ts`
- `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`
- this spec

## Non-goals

- No change to admission, kinds or reasons (*Admit live send commands in the browser*).
- No follow-mute composition.
- No live output routes.
- No parameter-metadata `routes` family (owner question Q2, DESIGN O11).

## Hazards

- **Index authority.** The SDK takes route indices from the engine's enumeration, never from its own
  sort of the session. Gate 1 boots a session whose route IDs are declared out of canonical order.
- **The export lists** grow by exactly two names. Any other change fails `check-web-audioworklet.sh`
  and `check-abi-layout-v1.py`.
- **A long route ID.** Without D2, a route ID longer than every source, track and submix ID traps
  the module when enumerated. Gate 4 pins it.

## Objective gates

1. **Encoding** (`sdk/test/live-controls-evals.mjs`, run by `check-sdk-headless.sh <A>`).
   - The session's route IDs are declared out of canonical order, and the SDK boots it with live
     controls on.
   - `controls.edit.route("kick-verb").gainDb(-12)` encodes kind 13 at `kick-verb`'s index in
     `SessionMap.routes`; `.mute(true)` encodes kind 14; `.matrix(...)` encodes kind 15 with the four
     values in `ll, lr, rl, rr` order.
   *Test value: it turns red if the SDK indexes routes by its own order or writes the matrix words in
   another order.*
2. **Refusals.**
   - An unknown route ID throws `MisoUsageError`.
   - A route into the output throws with a message naming output routes as not live (session given).
   *Test value: it turns red if the SDK sends a record at an index the engine reads as another route.*
3. **A live send edit renders on the shipped module.** In the same eval, a send gain edit with
   smoothing 0, submitted at a block boundary, renders bit-identically from that boundary to an engine
   booted from the session with the edited gain. The destination bus has no console slots, no
   stateful inserts and an identity input section; the sources differ per track and per lane.
   *Test value: it turns red if the export, the session map or the encoding disagrees with the engine
   on the shipped wasm. The native tests cannot see the module.*
4. **A long route ID enumerates.** A native host-web test boots a session whose one live route ID is
   longer than every source, track and submix ID, enumerates it through the route-ID export, and reads
   it back intact; a host-core test asserts `longest_route_id_bytes`.
   *Test value: it turns red if the staging buffer ignores route IDs and the copy overruns it.*
5. **SDK and browser:**
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-generated.sh <A>`
   - `bash scripts/check-sdk-headless.sh <A>`
   - `bash scripts/sdk-package.sh check <A>`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (the export list grows by exactly two)
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `python3 -B scripts/check-session-map-shape.py --self-test` and
     `python3 -B scripts/check-session-map-shape.py`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
6. **Workspace and policy.**
   - the test-debug-a command (DESIGN section 7)
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

No allocation gate: the exports run on the control plane, and no render-thread state changes.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new eval's name, with its one-sentence test-value answer.
- The ARTIFACT CHANGED report for the shipped module.

### Attempt 1 record (Terra)

Base `466f0ab63` (#1222's head). Anchors re-found by symbol.

**Implementation.**

- D2: `HostSessionShape.longest_route_id_bytes` measures every route of the session. The ID
  staging buffer is sized from the maximum of the source, track, submix and route lengths
  (`lib.rs` boot, and the three test mirrors in `tests.rs`).
- D1: `miso_engine_web_v1_live_control_route_count` and `_route_id` (`ffi.rs`) read
  `ReadyOwnership::route_controls`, the producers the send kinds index, through
  `live_control_route_count()` and `copy_live_control_route_id()`. A host without live controls
  has no producers and reports 0. Both names are added, sorted, to all four export lists, so each
  list grows by exactly two (`abi_layout::EXPORTS` is now 120).
- D3: the worklet reads the route IDs under the track and submix rules and posts `routes`. The
  host JS exact field list and validator, `MisoSessionMap.routes` (in both `.d.ts` copies,
  byte-identical), `SessionMap.routes` (`boundary.ts` reads the exports; the browser map copies
  the reply), and every stub listed in Context move with it.
- D4: `LiveControlEdits.route(id)` returns `RouteEdits` (`gainDb`, `mute`, `matrix`;
  `SmoothingOptions` only). Records are kinds 13, 14 and 15 at `SessionMap.routes`' index, with
  `rack` and `channel` 255. An unknown ID throws `MisoUsageError` with `diagnosticCode`
  `unknownRoute`. With a session, an output route's message says output routes are not live
  (`layoutOf` now also collects the output route IDs); otherwise it lists the live routes.
- #1222 deviation 2 is undone: `routeGainDb`, `routeMute` and `routeMatrix` are in `kindNames`,
  and the `kindsAwaitingSdk` exception is gone. The test is renamed "every command kind is built
  by name, and the strip and effect kinds are admitted by live Wasm": its one-track session has
  no send, and the send kinds are admitted in the #1223 evals.
- D6: the "Live sends (#1223, batch K3)" section in `APP-LIVE.md`.
- `check-session-map-shape.py`: the two exact-list mutations carry `"routes"`, and three new
  mutations are added (host list without `routes`, worklet without `routes`, `.d.ts` without
  `routes`). The self-test now catches 20.

**Tests and test value.** Each mutation was applied, run red and reverted. The rows are in
`hosts/host-web/MUTATIONS.md` "Issue #1223" and `crates/host-core/tests/MUTATIONS.md` 1223-H1.

- `tests::live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them` (gate 4,
  D1). The session has a 63-byte live send, longer than every source, track and submix ID. A
  second send, `send-a`, sorts before it, and `bx-main` sorts before both.
  - Red if staging ignores route IDs. `id_staging_bytes` is 2, not 63; without that assertion,
    the copy trips `compiled ID exceeds its projected staging capacity`.
  - Red if the export enumerates anything but the live send producers. Reading all routes puts
    `bx-main` at index 0.
- `live_routes::the_session_shape_measures_every_route_id` (gate 4, host-core). Red if
  `longest_route_id_bytes` measures only the live sends (7, not 39) or another ID family.
- `live-controls-evals.mjs` "a send edit encodes its kind at the engine's live-route index, over
  both transports" (gate 1). Sends are declared `snare-drums`, `vox-verb`, `kick-verb`,
  `kick-drums`, so `kick-verb` is 1 live but 2 by declaration and 2 among all routes.
  - Red if the SDK indexes by its own order of the session's routes, the browser map loses the
    engine's list, `OfflineEngine.sessionMap()` reorders it, or the matrix words are written
    `ll, rl, lr, rr`.
- "an unknown or output route ID refuses before any record is built" (gate 2). Red if an ID the
  SDK cannot place falls back to an index, or if the output-route reason is lost.
- "a live send gain equals the session booted at that gain from the edit's block on" (gate 3,
  shipped module). Red if the engine's export, the map or the encoding disagree on which send an
  index names.
  - Its unique catch: E1, a module rebuilt so that admission pushes onto route `index ^ 1`, turns
    only this eval red. Gate 1 stays green there because admission is `ok`.
- `live-controls-types.ts` (D4 and D5): `route()` returns `RouteEdits`, whose keys are exactly
  `gainDb | mute | matrix`, and `MisoSessionMap.routes` is `readonly string[]`.
  `@ts-expect-error` covers a lane option, a partial matrix and `faderDb`. Red if a send gains a
  lane option (`LaneOptions`) or a strip method.
- `test-web-audioworklet.mjs` (D3). The fake engine enumerates `zz-send`, `aa-send`, deliberately
  unsorted. Red if the worklet sorts or reorders them, the host validator accepts a non-string
  route, or construction skips the empty/oversized ID check. The four host-reply mutations mirror
  #1210 gate 3.
- No test is superseded. No digest or prose is pinned.

**Deviations.**

1. D5 says "the two exports are added to the `.d.ts`". The host `.d.ts` declares no export list,
   so there was nothing to add there. The exports reach TypeScript through the regenerated
   `sdk/src/generated/abi.ts` (`ExportName`). `RouteEdits` is an SDK type
   (`sdk/src/core/live-controls.ts`, exported by `index.ts`'s existing `export *`), not a host
   type.
2. `hosts/host-web/qualification/qualification.js` is unchanged. It reads `sessionMap()` and
   reports chosen fields, but spells no exact field list. K2 did not touch it for `submixes`
   either.
3. `scripts/test-web-audioworklet.sh`'s sed mutation still matches unchanged: it is a substring
   of the grown list. A routes sed mutation would duplicate the Python self-test's, so none was
   added.

**Gates** (x86-64-v3 AVX2; A = `target/ci/k3-1223-artifacts`, B = `target/ci/k3-1223-named`;
logs in `target/ci/k3-1223-logs/`). Every gate returned rc 0.

- Gate 5:
  - `check-sdk-types.sh`;
  - `check-sdk-generated.sh A`;
  - `check-sdk-headless.sh A` (355 pass, 0 fail);
  - `sdk-package.sh check A`;
  - `build-web-audioworklet.sh --named-twin B A`;
  - `check-web-audioworklet.sh A B/…simd128.named.wasm` (the export list grew by exactly the two
    names);
  - `check-abi-layout-v1.py --self-test` (22 caught) and on `A/…abi-layout.json`;
  - `check-session-map-shape.py --self-test` (20 caught) and plain;
  - `check-browser-expected-resources.py --artifacts A` (digests and exact rows agree, 32 red
    mutations; `idStagingBytes` did not move);
  - `test-web-audioworklet.sh`.
- Gate 6:
  - test-debug-a (DESIGN 7, `--no-fail-fast`): 108 binaries, 1,222 passed, 0 failed, 9 ignored;
  - `check-host-core-policy.sh`, `test-host-core-policy.sh` and `check-workspace-policy.sh`;
  - `cargo fmt --all -- --check`;
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
- **ARTIFACT CHANGED** (expected: two new exports and the worklet reader): `2daf0f83…`
  (2,762,489 B, #1222's record) -> `97a758d8fa5d274a2c2e1603d768101632f55b5a24ddc99fec22067d88be7d23`
  (2,763,281 B, +792). The named twin is `9e75ffa1…` (3,153,509 B). The module digest is not a
  per-change pin (#1061).

## Dependencies

- *Admit live send commands in the browser* (#1222)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Prefer the SDK's existing patterns (`TrackEdits`, `MisoUsageError`, `knownKeys`); add no new
  framework.
- In-place V1 amendment: append exports and fields; never renumber or rename.
- A test that greps source or prose is refused.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
