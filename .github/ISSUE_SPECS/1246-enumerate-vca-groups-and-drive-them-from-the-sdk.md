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
  `ReadyOwnership` keeps the normalized model's VCA IDs in a list built at boot beside `submixes`;
  `_vca_count` is its length and `_vca_id` copies from it. Both names are inserted in sorted
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
