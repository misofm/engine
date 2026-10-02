# Enumerate VCA groups and drive them from the SDK

Slice V4 of *VCA groups*. It closes the VCA batch, which the root pushes once after this slice's
verdict. **Drafted; anchors re-verified at filing:** they were read on `fe8ac679`, before batches
K1-K3 of *Submix strips and live aux sends* moved them; the Context says which slice moved what.

## Product outcome

An app or agent using the TypeScript SDK addresses a VCA by its ID:

```ts
controls.edit.vca("drums").faderDb(-6)
controls.edit.vca("fx").mute(true)
```

The engine publishes the VCA order (so the SDK never guesses an index) and a `vcas` metadata family
(so an app builds a VCA control from the engine's own domain), and the shipped module renders a VCA
ride bit-identically to a fresh plan.

The engine side has admitted the two VCA command kinds since *Ride VCA groups live in the browser*;
this slice gives them a name and a surface.

## Context (verified on `fe8ac679`; re-verify at filing)

- **After *Ride VCA groups live in the browser*:** kinds 16 `vcaFaderDb` and 17 `vcaMute` (or the
  numbers that slice recorded) exist; their index word is a VCA index in canonical ID order; an
  out-of-range index refuses with reason 14 `unknownVca`.
- **Exports and enumeration.** After *Name submix strips in the browser session map and the SDK
  measurement* and *Enumerate sends and drive them from the SDK*:
  - the frozen export list (`scripts/check-web-audioworklet.sh:203-320`, 117 names at `fe8ac679`,
    121 after K2-K3) is also spelled in `scripts/check-abi-layout-v1.py` `EXPORTS` (`:115` onward,
    checked exactly at `:432-434`) and its self-test fixture
    `scripts/fixtures/abi-layout-v1-self-test.json`, and in
    `tools/parameter-metadata/src/abi_layout.rs` `EXPORTS` (`[&str; 116]` at `:132` on `fe8ac679`,
    120 after K2-K3);
  - the enumeration exports follow the track pair `miso_engine_web_v1_live_control_track_count` and
    `_track_id` (`hosts/host-web/src/ffi.rs:3960`, `:3971`), which copy an ID into the ID staging
    buffer;
  - the stub module and session map in `scripts/test-web-audioworklet.mjs` (`:178-189`,
    `:2191-2197` at `fe8ac679`);
  - the worklet's `miso.sessionmap.v1` reply (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:945-958`)
    and its exact field list in the host JS, held by `scripts/check-session-map-shape.py` (mutation
    literal at `:428`) and `scripts/test-web-audioworklet.sh:326`;
  - `SessionMap` (`sdk/src/core/boundary.ts:252-256`, with `submixes` and `routes` after K2-K3),
    read by `shape()` (`:459-464`) and built at `:533-541`; the browser path builds it at
    `sdk/src/browser/live-controls.ts:48-52`;
  - `LiveControlEdits` (`sdk/src/core/live-controls.ts:398-415`), with `track(id)`, `submix(id)` and
    `route(id)` after K2-K3;
  - SDK stubs that construct a session map: `sdk/test/live-controls-types.ts:64-68`,
    `browser-defaults-evals.mjs:439`, `spectrum-browser-evals.mjs:349`, `console-evals.mjs:893`,
    `measurement-evals.mjs:277`.
- **ID staging** is sized from the longest source and track ID (`hosts/host-web/src/lib.rs:1918-1925`,
  `crates/host-core/src/shape.rs:62-77`), and K2-K3 added the longest submix and route ID
  (VERIFY-2 M7). A longer VCA ID copied into it would overrun `copy_id_into_staging` and trap the
  module.
- **Metadata.** The schema gate fixes the top-level keys (`scripts/check-parameter-metadata-v1.py:194-197`).
  The generator is `tools/parameter-metadata` (`--write | --check DIRECTORY | --print |
  --print-abi-layout`, `src/lib.rs:129-135`); `check-web-audioworklet.sh` runs `--check` and then the
  schema gate on `<A>/miso-engine-v1-parameter-metadata.json` (`:546-547`). The fader's domain is the
  builtin descriptor row at `crates/builtins/src/lib.rs:533-541` (`BUILTIN_PARAMETER_DESCRIPTORS`,
  `:453`).
- **Real-wasm evals.** `bash scripts/check-sdk-headless.sh <A>` runs every `sdk/test/*-evals.mjs`
  against the shipped module; `sdk/test/live-controls-evals.mjs` boots it through
  `sdk/test/support.mjs`.

## Decisions frozen for this slice

- **D1. Exports.** `miso_engine_web_v1_live_control_vca_count(handle) -> u32` and
  `miso_engine_web_v1_live_control_vca_id(handle, index) -> u32`, beside the track, submix and route
  pairs, enumerate the VCAs in canonical ID order (the index word of the VCA kinds). A host without
  live controls reports 0. Both are appended to every export list in Context.
- **D2. ID staging.** `HostSessionShape` gains `longest_vca_id_bytes`, and the staging buffer is sized
  from the maximum of all five longest-ID terms.
- **D3. Session map.** The `miso.sessionmap.v1` reply and `SessionMap` gain `vcas: readonly string[]`,
  in canonical order; the reply's exact-field checks and every stub that builds a session map add it.
- **D4. SDK API.** `LiveControlEdits.vca(id)` returns `VcaEdits` with `.faderDb(db, options?)` (per
  lane, as `TrackEdits.faderDb`) and `.mute(on, options?)`, each encoding its kind at the VCA's index
  in `SessionMap.vcas`. An unknown ID throws `MisoUsageError` naming the known VCAs. The typings are
  exported from `sdk/src/index.ts`, and the `.d.ts` and its SDK mirror stay byte-identical.
- **D5. Metadata.** A top-level `vcas` family: `fader_db` per lane in dB `[-144, 24]`, block target,
  the fader's mapping; `mute` per lane, boolean. The schema gate's top-level key set widens by exactly
  `vcas`, and its row checks accept exactly these two rows.

## Deliverables

1. host-core D2; host-web D1 and the worklet's D3.
2. SDK: D3's reader, D4 and its typings.
3. D5: the generator, the schema gate and its self-test fixture, and the regenerated `sdk/assets/**`
   and `sdk/src/generated/**`.
4. Evals in `sdk/test/live-controls-evals.mjs` and type tests in `sdk/test/live-controls-types.ts`.
5. `hosts/host-web/MUTATIONS.md` rows for the two exports and the session-map field.

## Authorized paths

- `crates/host-core/src/shape.rs`
- `hosts/host-web/src/{lib.rs,ffi.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/**`
- `scripts/check-web-audioworklet.sh` (the export list only), `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-parameter-metadata-v1.py`,
  `scripts/fixtures/parameter-metadata-v1-self-test.json`, `scripts/check-session-map-shape.py`,
  `scripts/test-web-audioworklet.sh`, `scripts/test-web-audioworklet.mjs`
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only), `sdk/src/core/{live-controls.ts,boundary.ts}`,
  `sdk/src/browser/**`, `sdk/src/index.ts`
- `sdk/test/{live-controls-evals.mjs,live-controls-types.ts,browser-defaults-evals.mjs,spectrum-browser-evals.mjs,console-evals.mjs,measurement-evals.mjs}`
- `docs/handoffs/submix-strips-and-sends/APP-LIVE.md` (one "Live VCA groups" section)
- this spec

## Non-goals

- No change to admission, kinds or reasons (*Ride VCA groups live in the browser*).
- No C ABI path (*Deliver value-only VCA edits to the running C ABI plan*).
- No VCA solo, no send trim and no automation.

## Hazards

- **Index authority.** The SDK takes VCA indices from the engine's enumeration, never from its own sort
  of the session. Gate 1 boots a session whose VCA IDs are declared out of canonical order.
- **The frozen export lists** grow by exactly two names each; any other change fails the layout and
  export gates.
- **ID staging.** A VCA ID longer than every other ID must not overrun the staging buffer (gate 3).

## Objective gates

1. **Encoding and enumeration** (`sdk/test/live-controls-evals.mjs`, run by
   `bash scripts/check-sdk-headless.sh <A>`). A session declares VCAs out of canonical order and boots
   with live controls. `SessionMap.vcas` lists them in canonical order (empty without VCAs);
   `controls.edit.vca("drums").faderDb(-6)` encodes the VCA fader kind at `drums`' index, and `.mute(true)`
   the VCA mute kind; `edits.vca("nope")` throws `MisoUsageError`. A type test asserts the `VcaEdits`
   surface.
   *Test value: it turns red if the SDK indexes VCAs by its own order, or the export, the session map
   and the encoding disagree.*
2. **A VCA ride renders on the shipped module.** In the same eval, a VCA fader edit with smoothing 0,
   submitted at a block boundary, renders bit-identically from that boundary to an engine booted from
   the session with the edited VCA value; the members feed a destination with no stateful downstream.
   *Test value: it turns red if the SDK's record reaches the wrong VCA or band in the shipped module;
   the native tests cannot see the module.*
3. **Long IDs.** A native host-web test boots a session whose VCA ID is longer than every source,
   track, submix and route ID, and reads it back through `_vca_id`.
   *Test value: it turns red if the staging buffer is sized without VCA IDs and the copy overruns.*
4. **Metadata.** The `vcas` family has exactly the two rows, with the fader's domain.
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` and
     `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`
   - `cargo run --locked -p parameter-metadata -- --check <A>`

   *Test value (self-test row): it turns red if the schema gate accepts a `vcas` family with an extra,
   missing or out-of-domain row.*
5. **Layout, vocabulary, browser and SDK.**
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `python3 -B scripts/check-session-map-shape.py --self-test` and
     `python3 -B scripts/check-session-map-shape.py`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (the export list grows by exactly two)
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>`, `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh <A>`, `bash scripts/sdk-package.sh check <A>`
6. **Workspace and policy.**
   - the workspace test command (`DESIGN.md` section 7)
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The ARTIFACT CHANGED report and the regenerated layout and metadata diffs.

## Dependencies

- *Ride VCA groups live in the browser*

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Prefer the SDK's existing patterns (`TrackEdits`, `MisoUsageError`); add no new framework.
- In-place V1 amendment: append exports, fields and metadata keys; never renumber or rename.
- A test that greps source or prose is refused.
- Commit on the VCA batch branch. The root pushes the batch once after this slice's verdict.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
