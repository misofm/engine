# Name submix strips in the browser session map and the SDK measurement

Slice 13 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The browser mixer knows every bus by name. The module enumerates the submix IDs in canonical order
through two new exports, the worklet's `miso.sessionmap.v1` reply and the SDK's `SessionMap` carry
them, and the SDK's browser measurement reports each bus's peaks and gain reduction keyed by its ID
("drums"), beside the per-track meters. The SDK live controls (*Drive submix strips from the SDK live
controls*, #1214) and route enumeration later index into this list.

A session without submixes reports `submixes: []`, and every existing field is unchanged.

## Context (verified on `fe8ac679`)

After *Carry submix strips in the browser meter frame* (#1209): the frame is `3(T + S) + 3` words, the
header carries `submixCount`, and the `miso.meter.v1` message carries `submixCount`, `submixPeaks`
(`2S`) and `submixGrDb` (`S`) beside its unchanged fields. After *List every strip in the
live-control handles and file bus effects in the browser* (#1207): `ReadyOwnership.tracks` is the track
prefix and `ReadyOwnership.submixes` the submix IDs in canonical order.

- **Enumeration today is tracks only.**
  - Exports `miso_engine_web_v1_live_control_track_count` and `_track_id`
    (`hosts/host-web/src/ffi.rs:3958-3973`), backed by `copy_live_control_track_id`
    (`hosts/host-web/src/lib.rs:2788-2797`), which copies into the ID staging buffer through
    `copy_id_into_staging` (`:2810`).
  - The worklet reads the track IDs once at construction (`miso-engine-v1-audio-worklet.js:598-612`,
    refusing an ID longer than `sourceIdCapacity`) and answers `miso.sessionmap.v1` with
    `{ tag, requestId, result, tracks, sources, metersAttached }` (`:945-958`).
  - The host validates the reply's exact field list
    (`miso-engine-v1-audio-worklet-host.js:980`, field checks `:1040-1051`); the `.d.ts` declares
    `MisoSessionMap` (`miso-engine-v1-audio-worklet-host.d.ts:385-400`).
  - The SDK headless boundary reads the same exports in `shape()` (`sdk/src/core/boundary.ts:455-470`)
    into `SessionShape` (`:228-236`) and returns `SessionMap { tracks, sources, metersAttached }`
    (`:250-256`, built in `sessionMap()` at `:533-541`).
  - The browser path builds its own `SessionMap` from the host's reply
    (`sdk/src/browser/live-controls.ts:48-52`) and creates its measurement feeds from the scratch boot's
    `shape.tracks` (`sdk/src/browser/engine.ts:590-594`).
- **ID staging overflows on a long submix ID** (VERIFY-2 M7). The staging buffer is sized from
  `max(longest_source_id_bytes, longest_track_id_bytes)` (`hosts/host-web/src/lib.rs:1918-1926`), from
  `HostSessionShape` (`crates/host-core/src/shape.rs:14-39`, built at `:50-81`). The same max is
  spelled in tests at `hosts/host-web/src/tests.rs:182`, `:923` and `:3382`. A submix ID longer than
  every track and source ID would overrun `copy_id_into_staging` and trap the module.
- **The frozen export list** has 117 names (with `memory`) in
  `scripts/check-web-audioworklet.sh:203-320`, and 116 in each of `scripts/check-abi-layout-v1.py`
  `EXPORTS` (`:115` onward, required equal to the layout at `:430-434`) and
  `tools/parameter-metadata/src/abi_layout.rs:132` (`pub const EXPORTS: [&str; 116]`). The generated
  layout (`sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`) and the self-test
  copy `scripts/fixtures/abi-layout-v1-self-test.json` list them too.
- **The session-map shape gate.** `scripts/check-session-map-shape.py` holds the worklet's posted map,
  the host's expected list and the `.d.ts` interface to the same field set; its self-test mutates the
  host list literal (`:424-432`). `scripts/test-web-audioworklet.sh:322-330` mutates the same literal
  on disk.
- **Stubs that model a session map or a scratch-boot shape** (each must carry `submixes`):
  - `scripts/test-web-audioworklet.mjs`: the stub reply `:178-189` and the stub module's track
    exports `:2191-2197` (with its `trackIds`);
  - `sdk/test/live-controls-types.ts:64-68`;
  - `sdk/test/browser-defaults-evals.mjs:439` (`sessionMap()`), and its scratch shapes (`shape`,
    `:65`, `:96`, `:114`, `:154`, `:300`, `:429`);
  - `sdk/test/spectrum-browser-evals.mjs:349` (`sessionMap()`) and `SHAPE` (`:7`);
  - `sdk/test/measurement-evals.mjs:80-87` (scratch shape) and `:275-278` (`sessionMap()`);
  - `sdk/test/console-evals.mjs:893`;
  - `sdk/test/live-controls-evals.mjs:139`, `:253-261` (the stub reply
    `createBrowserLiveControls` reads) and `:367`.

  Re-run `git grep -n "metersAttached" -- sdk/test scripts` and `git grep -n "scratchBoot" -- sdk/test`
  before editing.
- **Measurement.** `meterProjection` (`sdk/src/browser/measurement.ts:203-238`) builds per-track
  meters from `trackIds` and the master from the peaks; `MeterUpdate` is at `:25`.
  `createMeasurementFeeds` is at `:262`.
- **The shipped frame path in the SDK evals** is `sdk/test/capability-evals.mjs:135-147`
  (`measurement-evals.mjs` uses a fake host only; VERIFY-2 M9). `MeterUpdate`, `meterProjection` and
  `createMeasurementFeeds` are browser-only (called only from `sdk/src/browser/engine.ts:590`), so a
  headless eval sees the raw frame, never a `MeterUpdate`.
- **Observation is enriched against tracks only** (VERIFY-3 B1). After *List every strip in the
  live-control handles and file bus effects in the browser*, a bus effect that declares a tap has a
  filed observation handle, and host-web's `observation_binding` (`hosts/host-web/src/lib.rs:2462-2482`)
  reports its **strip** index `T + j` as the binding's `track_index`.
  `enrichObservationMap(tracks, raw)` (`sdk/src/core/observation.ts:195-226`) throws
  `sdk.observation.map` when `binding.trackIndex >= tracks.length`, and
  `resolveObservationAddressesWithTracks` (`:229-249`) resolves a selection with `tracks.indexOf`.
  Their callers pass the track list: headless `sdk/src/core/boundary.ts:622` and `:641`, browser
  `sdk/src/browser/engine.ts:605` and `:612`. `readObservations` calls `observationMap()` first
  (`boundary.ts:639-641`, `engine.ts:611-612`), so without D5 one bus tap makes **every**
  observation read throw, reads of a track's own taps included.

## Decisions frozen for this slice

- **D1. Exports.** `miso_engine_web_v1_live_control_submix_count(handle) -> u32` and
  `miso_engine_web_v1_live_control_submix_id(handle, index) -> u32`, beside the track pair, enumerate
  `ReadyOwnership.submixes` in canonical order and copy into the same ID staging buffer. Both are
  appended to every export list (the shell gate, the Python gate, the generator, the regenerated
  layout and the self-test copy).
- **D2. Staging covers submix IDs.** `HostSessionShape` gains `longest_submix_id_bytes: u64`, and the
  staging capacity is the max of the source, track and submix lengths (`lib.rs:1918-1926` and the
  three test spellings).
- **D3. The session map.** The worklet reads the submix IDs at construction exactly as it reads the
  track IDs, and its `miso.sessionmap.v1` reply, the host's expected field list, the `.d.ts`
  `MisoSessionMap` and its SDK mirror gain `submixes: readonly string[]`. The SDK's `SessionMap` and
  `SessionShape` gain `submixes`, read from the new exports by `shape()`; the browser path's
  `SessionMap` (`sdk/src/browser/live-controls.ts:48-52`) copies it from the reply.
- **D4. Measurement.** `createMeasurementFeeds` takes the submix IDs (from `shape.submixes`), and
  `MeterUpdate` gains `submixes: ReadonlyMap<string, TrackMeter>`, built from the message's
  `submixPeaks` and `submixGrDb` in canonical order. `meterProjection` refuses a frame whose
  `submixCount` differs from the known submix list (`sdk.meter.track_count`'s sibling
  `sdk.meter.submix_count`).
- **D5. Observation on strips.** `enrichObservationMap` and `resolveObservationAddressesWithTracks`
  take the strip list `[...tracks, ...submixes]`, tracks first (the strip index order of
  `ReadyOwnership`). Every caller (the four sites in the Context) passes
  `[...shape.tracks, ...shape.submixes]`. `trackId` in an `ObservationMap` binding names a strip; the
  field keeps its spelling (P17). `sdk/test/observation-evals.mjs:86` already passes a plain string
  list and needs no change.

## Deliverables

- D1-D5 across the host-web Rust, the worklet, the host JS, the typings and their SDK mirror, the SDK
  boundary, browser engine, live-controls and measurement modules, the layout generator and gates,
  and every stub in the Context.
- Regenerate the SDK assets (`node codegen/assets.mjs && node codegen/generate.mjs` in `sdk/`).
- `hosts/host-web/MUTATIONS.md` rows for the new exports and the reply field.

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `crates/host-core/src/shape.rs`
- `tools/parameter-metadata/src/abi_layout.rs` and `tools/parameter-metadata/tests/`
- `scripts/check-web-audioworklet.sh` (the export list), `scripts/check-abi-layout-v1.py` (`EXPORTS`),
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-session-map-shape.py`,
  `scripts/test-web-audioworklet.sh` (the session-map mutation literal at `:322-330`),
  `scripts/test-web-audioworklet.mjs`
- `sdk/src/core/boundary.ts`, `sdk/src/core/observation.ts`,
  `sdk/src/browser/{live-controls.ts,engine.ts,measurement.ts,shipped-host.d.ts}`,
  `sdk/src/browser/index.ts` and `sdk/src/headless/{engine.ts,index.ts}` (re-exported types only, if
  needed)
- `sdk/test/{live-controls-types.ts,browser-defaults-evals.mjs,spectrum-browser-evals.mjs,measurement-evals.mjs,console-evals.mjs,capability-evals.mjs,live-controls-evals.mjs}`
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only)
- this spec

## Non-goals

- No live control of submix strips (*Address submix strips in browser live commands* (#1213), *Drive submix
  strips from the SDK live controls*).
- No route enumeration (*Enumerate sends and drive them from the SDK*, #1223).
- No change to the frame (the previous slice).

## Hazards

- **ID staging.** The worklet refuses an ID longer than `sourceIdCapacity`, and the Rust copy traps
  past it. D2 must land with D1.
- **The exact-field validator.** The host fails whole on an unexpected reply field, so the worklet,
  the host list, the `.d.ts` and every stub move together; `check-session-map-shape.py` holds them.
- **Order.** The submix list is canonical submix order, the same order as the frame's submix
  sections. A different order mislabels every bus meter.

## Objective gates

1. **Names in canonical order, with long IDs.** Native host-web test: a session whose one submix ID
   (`zz-` followed by 60 characters) is longer than every source and track ID, and a second submix
   whose ID sorts before every track ID, boots; `live_control_submix_count` returns 2 and
   `_submix_id` returns each ID in canonical order, byte for byte; a session without submixes returns
   0.

   *Test value: it turns red if the staging capacity ignores submix IDs (the copy traps), or if the
   export orders submixes differently from the frame.*
2. **The SDK session map and measurement.**
   - (a) In `sdk/test/capability-evals.mjs` (headless, shipped module, run by
     `bash scripts/check-sdk-headless.sh <A>`), for a `T = 3`, `S = 2` session with meters on and a
     distinct source level per bus: `sessionMap().submixes` lists both IDs in canonical order, and the
     frame's `submixPeaks` pairs are ordered as that list.
   - (b) In `sdk/test/measurement-evals.mjs` (fake host), a posted 15-field `miso.meter.v1` message
     projects `MeterUpdate.submixes` keyed by ID in order. A message whose `submixCount` differs from
     the known list refuses with `sdk.meter.submix_count`. Compare structurally; never compare against
     hard-coded rendered values.

   *Test value: it turns red if the SDK keys a bus's meter by the wrong index, or the shipped
   enumeration and the frame disagree on submix order.*
3. **The browser path's session map.**
   - In `scripts/test-web-audioworklet.mjs`, a worklet reply without `submixes` fails the host, and
     one with it is delivered.

   *Test value: it turns red if the host accepts a worklet reply that lacks `submixes`. (That the
   browser live controls keep `submixes` is observable only once *Drive submix strips from the SDK live
   controls* exposes the map; its gate 4 covers it.)*
4. **Observation on strips.** In `sdk/test/capability-evals.mjs` (the shipped module), run by
   `bash scripts/check-sdk-headless.sh <A>`: a session with a bus compressor insert, a track
   compressor insert and observation taps on (`observationTaps` at least 2). `observationMap()` lists
   a binding whose `trackId` is the bus ID, and a `readObservations` of the **track** tap succeeds.

   *Test value: it turns red if the SDK enriches or resolves observation bindings against tracks only,
   so one bus tap breaks every observation read.*
5. **Shape gates.**
   - `python3 -B scripts/check-session-map-shape.py --self-test`
   - `python3 -B scripts/check-session-map-shape.py`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test`
6. **Browser artifact and SDK.**
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (the export list grows by exactly two)
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>`
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-headless.sh <A>`
   - `bash scripts/sdk-package.sh check <A>`
7. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7).
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The ARTIFACT CHANGED report and the regenerated layout diff (two new exports).

### Attempt 1 record (Terra)

- **D1** (`ffi.rs`, `lib.rs`): `miso_engine_web_v1_live_control_submix_count`/`_submix_id`
  beside the track pair, backed by `live_control_submixes()` and `copy_live_control_submix_id`
  over `ReadyOwnership.submixes` (canonical order, through `copy_id_into_staging`). Added to the
  shell gate list, `check-abi-layout-v1.py` `EXPORTS` and `abi_layout.rs` (`[&str; 118]`) in
  alphabetical position (each list is sorted; the shell list is `sort`ed). **D2**:
  `HostSessionShape.longest_submix_id_bytes`; staging = max of the three (`lib.rs` boot and the
  three `tests.rs` spellings). **D3**: worklet reads the submix IDs at construction under the
  track IDs' rules and posts `submixes` last in `miso.sessionmap.v1`; it also refuses
  construction when the meter header's `submix_count` differs from the enumerated count (the
  analogue of its existing `track_count` check). Host: `"submixes"` appended to the expected
  fields, entries must be nonempty strings. `.d.ts` `MisoSessionMap.submixes: readonly string[]`
  (SDK mirror is a byte copy). SDK `SessionShape`/`SessionMap.submixes` read by `shape()`; the
  browser `createBrowserLiveControls` map copies it. **D4**: `createMeasurementFeeds(host,
  trackIds, submixIds, available)`; `MeterUpdate.submixes`; `meterProjection` (now exported from
  the module only, not the barrel) refuses `sdk.meter.submix_count`. **D5**: headless
  `observationMap`/`readObservations` and the browser engine pass `[...tracks, ...submixes]` to
  enrich, resolve and `decodeObservationRows` (parameters renamed `strips`); the browser engine
  builds the list lazily, because `live-response-evals.mjs` scratch shapes carry no `tracks`.
- **Stubs**: every site in the Context carries `submixes` (`browser-defaults-evals` `shape` also
  gained `tracks: []`); `measurement-evals` call sites pass `[]` and its `meterFrame()` carries
  the three S = 0 fields. The `test-web-audioworklet.sh` sed literal still matches (substring)
  and still goes red; unchanged.
- **Layout diff**: `exports` gains `miso_engine_web_v1_live_control_submix_count` and
  `_submix_id` (nothing else); the self-test fixture is again a byte copy of the asset.
- **Tests and test value** (each mutation applied, run red, reverted; rows in
  `hosts/host-web/MUTATIONS.md`):
  - `host-web tests::submix_ids_enumerate_in_canonical_order_through_staging_sized_for_them`
    (gate 1; `zz-` + 60 declared first, `a-bus` second): red if staging ignores submix IDs or the
    export reorders submixes. Mutations: drop the submix max -> `id_staging_bytes` 14 != 63;
    reversed lookup -> submix 0 is the long ID.
  - `capability-evals.mjs` "the session map names every submix in the frame's bus order" (gate
    2a; `zz-bus` -40 dB declared first, `aa-bus` unity): red if the enumeration and the frame
    disagree on order or `shape()` skips the exports. Mutations: reversed list -> red; empty
    list -> red. Structural (loud > 10x quiet), no rendered values.
  - `measurement-evals.mjs` "a bus frame projects each submix's meter keyed by ID in canonical
    order" and "a meter frame for another submix list is refused with sdk.meter.submix_count"
    (gate 2b): red if a bus is keyed by the wrong index or read from another section, or the
    count is unchecked. Mutations: reversed index -> red; bus read from `peaks` -> red; count
    clause dropped -> red (the `submixCount: 3` case).
  - `measurement-evals.mjs` "the browser engine meters and observes a bus by its submix ID"
    (browser path of D4/D5, fake host): red if `engine.ts` hands the tracks alone to the feeds,
    the map or the resolver. Mutations: each of the three -> red. (Passing tracks to
    `decodeObservationRows` stays green: it falls back to `selection.trackId`, so no defect.)
  - `test-web-audioworklet.mjs` (gate 3): a reply without `submixes`, with a non-array, a
    non-string or an empty entry fails the host with 255; the well-formed reply is delivered
    with its order; the worklet posts the enumerated list and refuses an empty/over-long submix
    ID and a header count mismatch. Mutations: host list without `submixes` -> red; element rule
    dropped -> red; worklet stops posting -> red; header check dropped -> red.
  - `capability-evals.mjs` "a bus effect's observation names its submix and a track tap still
    reads" (gate 4; `t` and `bus` each with a compressor insert, taps 2): red if the SDK enriches
    against tracks only. Mutation: enrich with `shape.tracks` -> red. Resolving against tracks
    only stays green here (the track read resolves either way); the browser test above covers
    the resolver.
  - `check-session-map-shape.py --self-test`: two new mutations (host list, worklet reply
    without `submixes`); 17 caught.
- **Gates** (x86_64 AVX2; A = `target/ci/k2-1210-artifacts`, B = `target/ci/k2-1210-named`):
  5: both shape-gate commands ok, `--self-test` 17 caught; `check-abi-layout-v1.py --self-test`
  ok (22). 6: `build-web-audioworklet.sh --named-twin` ok; `check-abi-layout-v1.py <A>/...` ok;
  `check-web-audioworklet.sh` ok (export list +2); `check-browser-expected-resources.py
  --artifacts` ok (32 red mutations); `test-web-audioworklet.sh` ok; `check-sdk-generated.sh` ok;
  `check-sdk-types.sh` ok; `check-sdk-headless.sh` ok (339 pass, 0 fail); `sdk-package.sh check`
  ok. 7: workspace test command rc 0 (102 binaries, 1158 passed, 0 failed; includes
  `parameter-metadata`); `check-/test-host-core-policy.sh` ok; `cargo fmt --check` ok; workspace
  clippy `-D warnings` clean.
- **ARTIFACT CHANGED**: shipped module `c43ee960...82af59` (2 694 139 B) against #1209's
  `f5d36ba0...00ad0`; the bytes move because two exports and the worklet/host JS changed. No
  re-pin; CI's `artifact-identity` line is the authority.
- **Open (outside this slice):** host-web `observation_selection_for_address` still looks the
  address up in `ready.tracks` only, so a selected read of a **bus** tap is refused
  (`invalidArgument`) after the SDK resolves it to its strip index. Gate 4 needs only the track
  read; a bus-tap read needs a host-web successor (or #1213).

## Dependencies

- *Carry submix strips in the browser meter frame* (#1209)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: append exports and fields; never renumber, reorder or rename.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
