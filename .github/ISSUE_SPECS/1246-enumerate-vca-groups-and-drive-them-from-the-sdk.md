# Enumerate VCA groups and drive them from the SDK

Slice V7 of *VCA groups* (#1239). It closes the VCA batch (#1240-#1246), which the root pushes once
after this slice's verdict. It is the VCA twin of *Enumerate sends and drive them from the SDK*
(#1223): read its spec (`.github/ISSUE_SPECS/1223-*.md` or git history) for the export, session-map
and stub pattern this slice repeats.

The design record cited below (`DESIGN`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

An app or agent using the TypeScript SDK addresses a VCA by its ID:

```ts
controls.edit.vca("drums").faderDb(-6)
controls.edit.vca("fx").mute(true)
```

The engine publishes the VCA order, so the SDK never guesses an index, and the shipped module
renders a VCA ride bit-identically to a freshly booted engine. The engine has admitted the two VCA
kinds since *Ride VCA groups live in the browser* (#1245); this slice gives them a name and a
surface, and removes the decision-13 qualifier from `AGENTS.md`'s VCA sentence.

## Context (verified on `8c6268967`)

- **After #1245:** kinds 16 `vcaFaderDb` and 17 `vcaMute` (kind 3's and kind 4's record shape:
  `channel` a lane selector, `values[0]` the offset in dB or 0/1); their index word is a VCA index in
  canonical VCA-ID order; an out-of-range index refuses with reason 14 `unknownVca`. host-web's
  `ReadyOwnership.vcas: LiveVcaState` holds the VCAs in that order (`vca_count()`).
  `sdk/test/live-controls-evals.mjs` lists `vcaFaderDb` and `vcaMute` as awaiting this slice.
- **Exports.** The frozen export list is `expected_exports` in
  `scripts/check-web-audioworklet.sh:203-325` (`memory` plus 120 functions, **sorted**; the route
  pair at `:215-216`, the track pair at `:219-220`). It is also spelled in `scripts/check-abi-layout-v1.py`'s `EXPORTS` (the route pair at
  `:144-145`, checked exactly), its self-test fixture `scripts/fixtures/abi-layout-v1-self-test.json`
  (`:33-34`), and `tools/parameter-metadata/src/abi_layout.rs`'s `EXPORTS: [&str; 120]` (`:134`, the
  route pair at `:161-162`), documented "sorted" (`:128`); `check-abi-layout-v1.py` checks the exact
  sequence (`:440`) and its self-test refuses an unsorted set.
- **The enumeration exports** to copy are the route pair
  (`hosts/host-web/src/ffi.rs:3995-4014`): `_route_count(handle) -> u32` (zero before compilation or
  without live controls) and `_route_id(handle, index) -> u32`, which copies the ID into the ID
  staging buffer and returns its byte length, zero for no such index (`copy_live_control_route_id`,
  `hosts/host-web/src/lib.rs:2956-2964`; `live_control_route_count`, `:2162-2166`). The track and
  submix IDs are copied from lists `ReadyOwnership` keeps (`ready.tracks`, `ready.submixes`,
  `:2933-2952`); `LiveVcaState` keeps no IDs.
- **ID staging** is sized from the longest source, track, submix and route ID
  (`hosts/host-web/src/lib.rs:2046-2050`, from `host_core::HostSessionShape`,
  `crates/host-core/src/shape.rs:14-55`, built once at `:55-100`). A longer VCA ID copied into it
  would overrun it. The same `.max(..)` chain is repeated in `hosts/host-web/src/tests.rs:184`,
  `:931` and `:3392`; `crates/host-core/tests/live_routes.rs:778-795` is the route test template.
- **The worklet** reads the route IDs once at boot (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:632-648`)
  under the staging rules (non-empty, at most `sourceIdCapacity`, ASCII), and replies to
  `miso.sessionmap.v1` with `tracks, sources, metersAttached, submixes, routes`
  (`:1008-1023`). The host JS expects exactly those fields (`miso-engine-v1-audio-worklet-host.js:992`)
  and validates them (`:1065-1069`); the `.d.ts` declares `MisoSessionMap`
  (`miso-engine-v1-audio-worklet-host.d.ts:413-435`, mirrored byte for byte in
  `sdk/src/browser/shipped-host.d.ts`). `scripts/check-session-map-shape.py` holds the three in step
  (its mutation literal at `:428`), as does `scripts/test-web-audioworklet.sh`.
- **The hermetic stub** (`scripts/test-web-audioworklet.mjs`): its session map (`:185-192`), its
  enumeration exports (`:2309-2330`) and its staging-rule mutations (`:3302-3314`).
- **SDK.**
  - `SessionMap` (`sdk/src/core/boundary.ts:254-269`); the headless `sessionMap()` (`:559-572`) reads
    routes through `#liveRoutes` (`:574-584`); the browser path copies the worklet's reply
    (`sdk/src/browser/live-controls.ts:47-54`).
  - `LiveControlEdits` (`sdk/src/core/live-controls.ts:432-507`) resolves `track`, `submix`,
    `strip` and `route` IDs from the map; `RouteEdits` (`:738-777`) is the per-address template;
    `StripEdits.faderDb` and `.mute` (`:558-572`) build kinds 3 and 4 with `lane(options)`,
    `smoothing(options)` and `builtinNumber("fader_db", db)` (`:376-387`, the catalog's own domain).
    `sdk/src/index.ts:10` re-exports the module.
  - Session-map stubs: `git grep -n 'routes: \[' -- sdk/test` and `git grep -n 'sessionMap()' --
    sdk/test` (at `8c6268967`: `browser-defaults-evals.mjs:439`, `:478`, `console-evals.mjs:939`,
    `live-controls-evals.mjs:143`, `:257-266`, `:373`, `:609`, `live-controls-types.ts:64-70`,
    `measurement-evals.mjs:390-393`, `spectrum-browser-evals.mjs:348-349`, and
    `capability-evals.mjs`'s `engine.sessionMap()` reads).
  - `bash scripts/check-sdk-headless.sh <A>` runs every `sdk/test/*-evals.mjs` against the shipped
    module; `sdk/test/live-controls-evals.mjs` boots it through `sdk/test/support.mjs`.
- **`AGENTS.md`'s VCA sentence** (in "Approved audio architecture") ends with the qualifier
  "Approved by decision 13 (#1196) as owner-delegated answer (a), landing with the *VCA groups*
  umbrella, which is filed when batch K3 of #1196 closes: no VCA group exists yet." (#1197 D5 makes
  the batch's closing slice remove it.)
- **App handoff**: `docs/handoffs/submix-strips-and-sends/APP-LIVE.md` has "Live sends (#1223,
  batch K3)" (`:54-87`).

## Decisions frozen for this slice

- **D1. Exports.** `miso_engine_web_v1_live_control_vca_count(handle) -> u32` (zero before
  compilation or without live controls) and `miso_engine_web_v1_live_control_vca_id(handle, index)
  -> u32` (copies the ID into the ID staging buffer and returns its byte length; zero for no such
  VCA) enumerate the VCAs in canonical VCA-ID order, the index word of kinds 16 and 17.
  `_vca_count` is the live VCA state's count (`ready.vcas.vca_count()`, the bound admission refuses
  a VCA index against, so it is 0 without live controls), and `_vca_id(i)` copies
  `ready.session.normalized_model().vcas[i].id` from the retained normalized model that state was
  built from; no VCA-ID list is copied. (Amended after attempt 1, whose deviation 1 this is: the
  frozen text had `ReadyOwnership` keep a list built at boot.) Both names are inserted in sorted
  position, right after `miso_engine_web_v1_live_control_track_id`, in every export list in the
  Context (123 names in `check-web-audioworklet.sh`, `[&str; 122]` in `abi_layout.rs`).
- **D2. ID staging.** `HostSessionShape` gains `longest_vca_id_bytes`, and every staging size takes
  the maximum of all five longest-ID terms (`lib.rs` and the three test copies).
- **D3. Session map.** The worklet reads the VCA IDs at boot under the route IDs' rules; the
  `miso.sessionmap.v1` reply, the host JS's exact field list and validation, `MisoSessionMap` (and
  its mirror) and the SDK's `SessionMap` gain `vcas: readonly string[]` in canonical order; the
  headless `sessionMap()` reads it through the new exports. `vcas` is `[]` without VCAs and without
  live controls (the count export answers 0). Every stub that builds a session map adds `vcas`, and
  the hermetic stub in `scripts/test-web-audioworklet.mjs` enumerates at least two VCAs, out of
  canonical order (as its routes are, `["zz-send", "aa-send"]`), so `_vca_id` is called at boot.
- **D4. SDK API.** `LiveControlEdits.vca(id)` returns `VcaEdits`, indexed by the VCA's position in
  `SessionMap.vcas`:
  - `.faderDb(db, options?: LaneOptions)` builds `vcaFaderDb` exactly as `StripEdits.faderDb` builds
    `faderDb` (`channel` from the lane option, the value through `builtinNumber("fader_db", db)`);
  - `.mute(on, options?: LaneOptions)` builds `vcaMute` as `StripEdits.mute` builds `mute`;
  - an unknown ID throws `MisoUsageError` with `diagnosticCode` `unknownVca`, naming the known VCAs.

  `VcaEdits` is exported through `sdk/src/index.ts`; the `.d.ts` and its SDK mirror stay byte
  identical.
- **D5. No metadata family.** A VCA fader's domain is the builtins' `fader_db` row, which the
  parameter metadata already publishes; sends set the precedent (K3 added no `routes` family). The
  metadata's top-level keys do not change.
- **D6. Vocabulary.** `sdk/test/live-controls-evals.mjs` moves `vcaFaderDb` and `vcaMute` into
  `kindNames` and drops the exclusion #1245 added.
- **D7. `AGENTS.md`.** Remove exactly the qualifier sentence quoted in the Context, changing nothing
  else; the VCA batch has closed and VCAs apply on every host.

## Deliverables

1. host-core D2; host-web D1 and the worklet's D3.
2. SDK: D3's readers, D4 and its typings, D6.
3. Evals in `sdk/test/live-controls-evals.mjs`, type tests in `sdk/test/live-controls-types.ts`.
4. `hosts/host-web/MUTATIONS.md` rows for the two exports and the session-map field.
5. `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`: a "Live VCA groups (#1246)" section after
   "Live sends".
6. `AGENTS.md` (D7).

## Authorized paths

- `crates/host-core/src/shape.rs` and `crates/host-core/tests/` (one test)
- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS` only)
- `scripts/check-web-audioworklet.sh` (the export list only), `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-session-map-shape.py`,
  `scripts/test-web-audioworklet.sh`, `scripts/test-web-audioworklet.mjs`
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only), `sdk/src/core/{live-controls.ts,boundary.ts}`,
  `sdk/src/browser/**`, `sdk/src/index.ts`
- every `sdk/test/*` file the Context's two greps list (the session-map stubs only), and
  `sdk/test/{live-controls-evals.mjs,live-controls-types.ts}`
- `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`
- `AGENTS.md` (that qualifier only)
- this spec

## Non-goals

- No change to admission, kinds or reasons (#1245) or to composition (#1244).
- No metadata family (D5). No C ABI path (#1247). No VCA solo, no send trim and no automation.

## Hazards

- **Index authority.** The SDK takes VCA indices from the engine's enumeration, never from its own
  sort of the session; gate 1 declares the VCAs out of canonical order.
- **The frozen export lists** grow by exactly two names each; any other change fails the layout and
  export gates.
- **ID staging.** A VCA ID longer than every other ID must not overrun the staging buffer (gate 3).

## Objective gates

1. **Encoding and enumeration** (`sdk/test/live-controls-evals.mjs`, run by
   `bash scripts/check-sdk-headless.sh <A>`). A session declares VCAs out of canonical order and
   boots with live controls. `SessionMap.vcas` lists them in canonical order (and is `[]` without
   VCAs); `controls.edit.vca("drums").faderDb(-6)` encodes kind 16 at `drums`' index, a left-lane
   `.mute(true, { channel: "left" })` kind 17 with channel 0; `edits.vca("nope")` throws
   `MisoUsageError` with `unknownVca`. A type test asserts the `VcaEdits` surface.
   *Test value: it turns red if the SDK indexes VCAs by its own order, or the export, the session map
   and the encoding disagree.*
2. **A VCA ride renders on the shipped module.** In the same eval file, a VCA fader edit with
   smoothing 0, submitted at a block boundary, renders bit-identically from that boundary to an
   engine booted from the session with the edited VCA value; a VCA mute likewise; the members feed a
   destination with no stateful downstream.
   *Test value: it turns red if the SDK's record reaches the wrong VCA, kind or lane in the shipped
   module, which the native tests cannot see.*
3. **Long IDs.** A host-core test asserts `longest_vca_id_bytes`, and a native host-web test boots a
   session whose VCA ID is longer than every source, track, submix and route ID and reads it back
   through `_vca_id`.
   *Test value: it turns red if the staging buffer is sized without VCA IDs and the copy overruns.*
4. **Session-map shape and stub rules.**
   - `python3 -B scripts/check-session-map-shape.py --self-test` and
     `python3 -B scripts/check-session-map-shape.py`, with a new self-test mutation "the host's
     acknowledgement validator does not expect the VCA list";
   - `bash scripts/test-web-audioworklet.sh`, with "an empty VCA ID" and "a VCA ID longer than
     staging" stub mutations refused at boot (the stub enumerates VCAs, D3).

   *Test value: it turns red if the worklet, the host JS and the `.d.ts` disagree on the session
   map, or the worklet accepts an unreadable VCA ID.*
5. **Layout, browser and SDK** (`npm ci` in `sdk/` first; `<A>`, `<B>` fresh empty directories):
   - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (the export list grows by exactly two)
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/check-sdk-generated.sh <A>`, `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh <A>` (with D6's vocabulary eval green),
     `bash scripts/sdk-package.sh check <A>`
   - The browser legs: in `hosts/host-web/qualification`, `npm ci`,
     `npx playwright install <browser>`, then
     `npm run qualify -- --artifacts <abs A> --sdk-root <abs sdk> --browser <browser> --check-matrix --self-test-mutations`
     for `chromium`, `firefox` and `webkit`, under a private PulseAudio null sink as the `browser`
     job of `.github/workflows/qualification.yml` (`:362-430`) sets it up.
6. **Workspace and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh` (`host-core` is a product crate)
7. **Batch boundary** (this slice closes the batch; run once on the batch head before the push):
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`,
     `./target/release/audit capi`, `bash scripts/trace-graph-audit.sh target/release/audit`,
     `bash scripts/check-graph-determinism.sh`,
     `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`,
     `bash scripts/check-builtins-fixtures.sh . target/release/audit`,
     `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-protocol-wasm-parity.sh`, `bash scripts/check-capi-abi.sh` and
     `bash scripts/check-capi-abi.sh --self-test`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the
     batch push (recorded "at batch push").

## Evidence

- The output of every gate command above, from the PR's head commit (gate 7 from the batch head).
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- The ARTIFACT CHANGED report and the regenerated layout diff.

### Attempt 1 record (Terra)

Base `86c050075` (#1245's head). Anchors re-found by symbol.

**Implementation.**

- D2: `HostSessionShape.longest_vca_id_bytes`; the ID staging size takes the maximum of all five
  longest-ID terms (`lib.rs` boot and the three `tests.rs` mirrors).
- D1: `miso_engine_web_v1_live_control_vca_count` / `_vca_id` (`ffi.rs`), through
  `live_control_vca_count()` and `copy_live_control_vca_id()` (`lib.rs`). Both names are inserted
  right after `..._track_id` in all four export lists (123 names in `check-web-audioworklet.sh`,
  `EXPORTS: [&str; 122]`).
- D3: the worklet reads the VCA IDs under the route rules and posts `vcas`; the host JS exact
  field list and validator, `MisoSessionMap.vcas` (both `.d.ts` copies byte-identical), the SDK
  `SessionMap.vcas` (headless `#liveVcas()` reads the exports; the browser map copies the reply)
  and every Context stub carry it. The hermetic stub enumerates `["zz-vca", "aa-vca"]`.
- D4: `LiveControlEdits.vca(id)` returns `VcaEdits` (`faderDb`, `mute`, `LaneOptions`), kinds 16
  and 17 at `SessionMap.vcas`' index, `rack` 255, built as `StripEdits.faderDb`/`mute` build kinds
  3/4. An unknown ID throws `MisoUsageError` with `diagnosticCode` `unknownVca`, listing the VCAs
  (or `none`). Exported by `index.ts`'s existing `export *`.
- D5: no metadata change. D6: `vcaFaderDb`/`vcaMute` are in `kindNames`; `kindsAwaitingSdk` is
  gone. D7: the `AGENTS.md` qualifier sentence is removed, nothing else.
- `check-session-map-shape.py`: the exact-list mutation literals carry `"vcas"`; three new
  mutations (host list without `vcas` -- the spec's named one -- worklet without `vcas`, `.d.ts`
  without `vcas`). The self-test now catches 23.
- `APP-LIVE.md` "Live VCA groups (#1246)"; `MUTATIONS.md` rows in `hosts/host-web/` and
  `crates/host-core/tests/` (1246-H1, -H2).

**Tests and test value.** Every mutation was applied, run red and reverted (rows in
`hosts/host-web/MUTATIONS.md` "Issue #1246").

- `vca_caps::the_session_shape_measures_the_longest_vca_id` (gate 3, host-core). Red if the shape
  leaves VCA IDs unmeasured or measures only the first/last VCA (the 46-byte VCA sits between two
  short ones in canonical order). H1 (`.next()` for `.max()`): 1, not 46; H2 (tracks): red.
- `tests::live_vca_ids_enumerate_in_vca_index_order_through_staging_sized_for_them` (gate 3, D1).
  A 73-byte `zz-` VCA, longer than every other ID, lists `vocal` and the nested `aa`; VCAs are
  declared and nested out of canonical order; a kind 16 at each exported index must move exactly
  that VCA's members. Red if staging ignores VCA IDs, the export's order is not the admission
  index's, or the host answers without live controls. Mutations W1-W4 all red.
- `live-controls-evals.mjs` "a VCA edit encodes its kind at the engine's VCA index, over both
  transports" (gate 1). The document declares `drums`, `fx`, `band`; the engine's order is `band`,
  `drums`, `fx`. Red if the SDK indexes by any other order, either map loses or reorders the list,
  or a record word is wrong; also asserts `vcas` is `[]` without VCAs and without live controls.
- "an unknown VCA ID refuses with unknownVca before any record is built" (gate 1). Red if an ID
  the SDK cannot place (a typo, a member strip, a submix) reaches an index, or the reason is not
  `unknownVca`.
- "a live VCA fader and mute equal the session booted at those values from the edit's block on"
  (gate 2, shipped module; a both-lane fader on `drums` and a left-lane mute on `fx`; members feed
  a unity bus or the output). Unique catch: E1, a module rebuilt so admission rides VCA
  `index ^ 1`'s offset, turns only this eval red (gate 1 stays green: admission is `ok`).
- `live-controls-types.ts`: `vca()` returns `VcaEdits` with keys exactly `faderDb | mute`;
  `MisoSessionMap.vcas` and `SessionMap.vcas` are `readonly string[]` and required;
  `@ts-expect-error` covers a numeric mute, `solo`, `pan`, `effect` and a foreign option. T1-T4 red.
- `test-web-audioworklet.mjs` (D3, gate 4): the stub's "an empty VCA ID" and "a VCA ID longer than
  staging" mutations are refused at boot; the host refuses four malformed `vcas` replies. K1-K5
  (worklet without either check, worklet sorts, host validator without either check) all red.
- No test is superseded. No digest or prose is pinned.

**Deviations.**

1. D1 says `ReadyOwnership` keeps a VCA-ID list built at boot. It does not copy one: `_vca_count`
   is `ready.vcas.vca_count()` (the live VCA state, the exact bound admission refuses an index
   against, so it is 0 without live controls), and `_vca_id(i)` copies
   `ready.session.normalized_model().vcas[i].id` -- the retained model that state was built from,
   as `copy_session_source_id` reads sources -- for `i` below that count. Same order and authority,
   no duplicate retained allocation and no unaccounted bridge bytes (the #1245 exact retained
   budget test stays unchanged).
2. `scripts/test-web-audioworklet.sh` is unchanged: its sed mutation still matches a substring of
   the grown host list, and a `vcas` sed mutation would duplicate the Python self-test's.
3. `hosts/host-web/qualification/` is unchanged: its stubs build `SessionShape` (`scratchBoot`),
   which did not change, and `qualification.js` spells no exact session-map field list. All three
   browser legs pass in SDK source-bundle mode.

**Gates** (x86-64-v3 AVX2; A = `/tmp/claude-1002/kv-1246/A`, B = `.../B`; logs in
`/tmp/claude-1002/kv-1246/logs/`). Every gate returned rc 0.

- Gate 4: `check-session-map-shape.py --self-test` (23 caught) and plain;
  `test-web-audioworklet.sh`.
- Gate 5: `build-web-audioworklet.sh --named-twin B A`; `check-abi-layout-v1.py --self-test`
  (22 caught) and on `A/...abi-layout.json`; `check-web-audioworklet.sh A B/...named.wasm`
  (exports grew by exactly the two names); `check-browser-expected-resources.py --artifacts A`
  (digests and exact rows agree, 32 red mutations, no re-pin); `check-sdk-generated.sh A`;
  `check-sdk-types.sh`; `check-sdk-headless.sh A` (360 pass, 0 fail); `sdk-package.sh check A`;
  the browser legs, `npm run qualify -- ... --check-matrix --self-test-mutations` under a private
  PulseAudio null sink with stray `sdk/dist` deleted: chromium 151.0.7922.34, firefox 153.0 and
  webkit 26.5 "all qualification gates passed" (SDK source-bundle mode).
- Gate 6: workspace tests (`--no-fail-fast`): 115 binaries, 1,288 passed, 0 failed, 9 ignored;
  `cargo fmt --check`; clippy `-D warnings`; `cargo doc` `-D warnings`; host-core, realtime and
  workspace policy check + test; `check-cross-targets.sh`.
- Gate 7 (on the batch head with this slice): the DSP-crate tests (146 binaries, 791 passed, 0
  failed); release `audit`/`bench`/`console-workload` tests (110 passed); the release build;
  `audit capi`; `trace-graph-audit.sh`; `check-graph-determinism.sh`; `graph_fixture --check`;
  `check-builtins-fixtures.sh`; `check-console-fixtures.sh`; `check-protocol-wasm-parity.sh`;
  `check-capi-abi.sh` and `--self-test`. `run-aarch64-tests.sh debug`: no arm64 host here, so
  recorded "at batch push" (CI's `aarch64-debug`).
- **ARTIFACT CHANGED** (expected: two exports and the worklet reader): `731f65cb...f5e`
  (2,848,600 B, #1245's record) -> `5d21f73e9f6667f7f77ac19d216ea689ebcf46529ba134d7617e2539e3c92675`
  (2,849,413 B, +813); named twin `386b0651...4743` (3,252,917 B). Not a per-change pin (#1061).
- Regenerated layout diff: `sdk/assets/miso-engine-v1-abi-layout.json` and
  `sdk/src/generated/abi.ts` each gain exactly `miso_engine_web_v1_live_control_vca_count` and
  `miso_engine_web_v1_live_control_vca_id`.

### VCA follow-up record (after the attempt 1 PASS verdict)

Applied in the VCA batch follow-up commit (on `5248f94c4`, branch `codex/batch-vca`):

- **m1.** `scripts/test-web-audioworklet.mjs`'s stub enumerates three VCAs (`zz-vca`, `mm-vca`,
  `aa-vca`) beside its two routes. Test value: red if the worklet reads the VCA count from another
  export (the route count it was cloned beside), which every gate missed while the stub had as many
  VCAs as routes. Mutation (the worklet's `vcaCount` read from
  `miso_engine_web_v1_live_control_route_count`): RED at `issue #1246: the enumerated VCA order`;
  green on the unmutated worklet. A `hosts/host-web/MUTATIONS.md` row records it.
- **n1.** The native enumeration test's doc says the model lists the VCAs out of canonical order and
  the booted canonical document sorts them; declaration-order independence is the SDK eval's.
- **n2.** D1 is amended to the accepted deviation 1 (IDs read from the retained normalized model; no
  list copied).
- **n3.** Not applied: a non-ASCII stub mutation for every enumerated ID reader (tracks, submixes,
  routes and VCAs share the gap) is pre-existing and a candidate successor.

## Verdict

- **Attempt 1** (`5248f94c4`): Sol PASS. One MINOR and three NITs; m1, n1 and n2 are applied above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1246-attempt1.md`.

## Dependencies

- *Ride VCA groups live in the browser* (#1245)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Prefer the SDK's existing patterns (`RouteEdits`, `StripEdits`, `MisoUsageError`); add no new
  framework.
- In-place V1 amendment: append exports and fields; never renumber or rename.
- A test that greps source or prose is refused.
- Commit on the VCA batch branch. The root pushes the batch once after this slice's verdict.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
