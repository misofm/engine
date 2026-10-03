# Carry submix strips in the browser meter frame

Slice 12 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The browser mixer sees every bus's level. With meters on, host-web meters every strip, the meter
frame carries a peak pair and a gain-reduction word per submix strip, the header says how many
submixes the frame holds, and the `miso.meter.v1` message the app receives carries the bus values in
three new fields beside today's.

A session without submixes produces the same frame words and the same existing message fields as
today; only the header grows from 64 to 72 bytes. Names for the bus slots (the submix IDs) are the
next slice, *Name submix strips in the browser session map and the SDK measurement* (#1210). A bus's
gain-reduction word becomes nonzero once *Address submix strips in browser live commands* (#1213) lets an
observation command arm a bus effect.

## Context (verified on `fe8ac679`)

After *List every strip in the live-control handles and file bus effects in the browser* (#1207): host-web
files bus effects, `observation_tracks` holds strip indices, `observation_present` is sized `T + S`,
and the gain-reduction fold skips strip indices `>= T` (that slice's D4) until this slice. After
*Meter any boundary of a submix strip and designate a master strip in host-core* (#1208):
`HostMeterRequest.strip_id` may name a submix.

- **Frame layout today: `3T + 3` `f32` words.** Track peaks `[2i]`, `[2i + 1]`; master L and R at
  `2T`, `2T + 1`; one gain-reduction word per track from `2T + 2`; the master's gain reduction last.
  Rust spellings in `hosts/host-web/src/lib.rs`:
  - `:1023-1024` (the header field doc, "`3T + 3`"), `:1464-1466` (field doc), `:2068-2076`
    (`meter_frame()` doc);
  - the poll: `:3154` (`track_count = ready.meters.len()`), `:3289-3290` (`tracks = ready.tracks.len()`,
    `master = tracks * 2`), `:3369` (`gain_base = tracks * 2 + 2`), the fold and master writes
    `:3413-3446`;
  - `:6056-6064` (header construction: `track_count`, `master_track_plus_one`);
  - `:6212-6226` (`boxed_zero_meter_frame`).
- **Meters requested** (`lib.rs:5715-5726`): when meters are on, host-web requests one `PostMatrix`
  `SAMPLE_PEAK` meter per **track**, from `model.tracks`.
- **`WebMeterHeader`** (`lib.rs:1018-1040`) is 64 bytes: `METER_HEADER_BYTES =
  size_of::<WebMeterHeader>()` at `:1054`.

  | Offset | Field |
  |---|---|
  | 0 | `struct_size` |
  | 4 | `abi_version` |
  | 8 | `track_count` |
  | 12 | `windows` |
  | 16 | `first_sample` |
  | 24 | `end_sample` |
  | 32 | `sequence` |
  | 40 | `master_track_plus_one` |
  | 44 | `master_gr_present` |
  | 48 | `reserved[0]` |
  | 56 | `reserved[1]` |

  **Both reserved words are in use:** `reserved[0]` is the publication generation; `reserved[1]` holds
  validity bits 0..3 (`METER_VALID_*`, `:1042-1051`) and a saturating loss count in bits 32..63. So a
  new field must be appended.
- **The worklet** (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`): `:4`
  (`METER_HEADER_BYTES = 64`); the frame views and header checks at `:720-746` (`(trackCount*3+3)*4`
  at `:724`, the `2T + 2` peak view `:731-735`, the gain view `:736-740`, the master index
  `trackCount*3+2` at `:741`, the header checks `:742-746`); the message object `:748-765`; the post
  `postMeterFrame` `:1673-1694`.
- **The main-realm host** (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:887-925`)
  validates `miso.meter.v1` with `hasExactFields` over exactly `tag, sequence, generation, validity,
  lossCount, windows, trackCount, peaks, trackGrDb, masterGrDb, firstSample, endSample` (`:899-902`),
  and checks `peaks.length === 2T + 2` and `trackGrDb.length === T`. An unexpected field fails the
  whole host.
- **The typings** `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`: the cost note
  `:98-108` and `MisoMeterFrame` `:569-605`. `sdk/src/browser/shipped-host.d.ts` is a byte-identical
  mirror (`cmp` passes on `fe8ac679`).
- **The SDK headless reader** (`sdk/src/core/boundary.ts`): the `MeterFrame` type `:287-298` and
  `pollMeters` `:1060-1108`, which checks `structSize` against the generated layout and sizes the
  frame as `trackCount * 3 + 3`.
- **Layout pins:** `scripts/check-abi-layout-v1.py:242` (`"meterHeader": 64`);
  `tools/parameter-metadata/src/abi_layout.rs:478-503` (`meter_header_fields() -> [Field; 10]`) and
  `:1827-1833`; the generated layout `sdk/assets/miso-engine-v1-abi-layout.json:190-203`,
  `sdk/src/generated/abi.ts` (from `:417`) and the self-test copy
  `scripts/fixtures/abi-layout-v1-self-test.json:190-203`; host-web's own pins
  `hosts/host-web/src/tests.rs:390-391`.
- **Harnesses that size frames or post messages:**
  - `scripts/test-web-audioworklet.mjs`: the meter messages it posts to the host at `:1685`, `:1753`,
    `:1865`, `:1901`, `:1915`, `:2005`; the stub header and frame `:2059-2079`; the stub poll
    `:2180-2185`; the frame assertions `:2985-3013`;
  - `hosts/host-web/tests/browser-v1/direct-oracle.mjs:33` (`METER_HEADER_BYTES = 64`), `:480-490`,
    `:515-530`;
  - `hosts/host-web/qualification/qualification.js:366`, `:415-425` (reads existing fields only).
- **Readers that keep working unchanged**, because the existing message fields keep their shapes:
  `sdk/src/browser/measurement.ts:203-238` (`meterProjection`), `sdk/test/capability-evals.mjs:135-147`
  (the shipped frame path: `peaks.length === 2T + 2`, `trackGrDb.length === T`) and
  `sdk/test/measurement-evals.mjs` (a fake host).
- **Prose:** `hosts/host-web/MUTATIONS.md:61` and `docs/BUILTINS_AND_METERING_V1.md:100-108`. The
  latter claims that a gain-reduction value describes the same window as the peak beside it
  (`:100-104`), which contradicts `lib.rs:1012-1013` and the typings: gain reduction is aged
  independently.

## Decisions frozen for this slice

- **D1. The frame is `3(T + S) + 3` words.**
  - Peaks: tracks `[2i, 2i + 1]` for `i < T`, then submix strips `[2(T + j), 2(T + j) + 1]`.
  - Master L and R at `2(T + S)` and `2(T + S) + 1`.
  - Gain reduction: tracks, then submix strips (canonical submix order), then the master.
  - With `S = 0` every frame word is where it is today.
  - When `S > 0` the master is no longer adjacent to the tracks, so the message's `peaks`
    (`2T + 2`) is assembled from two fixed views built at construction (the `2T` track words and the
    two master words), not the single `2T + 2` view the worklet builds today
    (`miso-engine-v1-audio-worklet.js:731-735`); the SDK headless boundary (`boundary.ts:1100`,
    `peakWords = trackCount * 2 + 2`) likewise reads the master at `2(T + S)`.
- **D2. The header appends `submix_count: u32` at offset 64 and `reserved_pad: u32` at 68** (layout
  names `submixCount`, `reservedPad`). The header becomes 72 bytes and `struct_size` reports 72.
  `track_count` keeps its meaning (tracks only). In-place V1 amendment, no `ABI_VERSION` bump; the
  layout JSON is regenerated.
- **D3. Meter requests.** When meters are on, host-web requests one `PostMatrix` `SAMPLE_PEAK` meter
  per **strip** (tracks, then submixes).
- **D4. The `miso.meter.v1` message** keeps every existing field with its exact shape (`peaks` is the
  `2T + 2` track and master peaks, `trackGrDb` the `T` track words, `masterGrDb` as today) and
  appends:
  - `submixCount: number` (`S`);
  - `submixPeaks: Float32Array` of `2S` words, `[bus0 L, bus0 R, ..]`;
  - `submixGrDb: Float32Array` of `S` non-negative decibel magnitudes.

  The worklet fills them from the frame's submix sections with fixed views built at construction (no
  per-block view or allocation). The host's exact-field check accepts exactly the 15 fields and
  checks `submixPeaks.length === 2 * submixCount` and `submixGrDb.length === submixCount`, every
  value finite and non-negative. The SDK headless `MeterFrame` gains the same three fields.
- **D5. The fold guard goes.** The gain-reduction fold writes every observed strip's word at
  `gain_base + strip`, with `gain_base = 2(T + S) + 2`; slice 10's skip of strip indices `>= T` is
  removed.

## Deliverables

- D1-D5 across the host-web Rust, the worklet, the host JS, the typings and their SDK mirror, the SDK
  headless reader, the layout generator and its checks, and the harnesses that size frames or post
  meter messages.
- Regenerate the SDK assets (`node codegen/assets.mjs && node codegen/generate.mjs` in `sdk/`).
- `hosts/host-web/MUTATIONS.md` rows for the frame's submix section, the header field and the message
  fields.
- `docs/BUILTINS_AND_METERING_V1.md:100-108`: the frame shape, the message fields, and the
  correction that gain reduction is aged independently of the peak window.

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `hosts/host-web/tests/browser-v1/direct-oracle.mjs`
- `hosts/host-web/qualification/{run.mjs,qualification.js,sdk-response-entry.ts}`, only if a field
  set they check must follow D4
- `tools/parameter-metadata/src/abi_layout.rs` and `tools/parameter-metadata/tests/`
- `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/test-web-audioworklet.mjs`
- `sdk/src/core/boundary.ts`, `sdk/src/browser/shipped-host.d.ts`
- `sdk/test/capability-evals.mjs` (the new submix gate only)
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only)
- `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No submix IDs in the session map and no submix-keyed SDK measurement (the next slice).
- No live control of submix strips, and no bus gain-reduction reading: a bus effect cannot be armed
  until *Address submix strips in browser live commands*, which carries the bus gain-reduction gate.
- No spectrum target on submixes, no #882 lease work, no C ABI meter change.

## Hazards

- **A hard-coded `3T + 3` or 64.** Every listed site must take `S` and 72. A missed reader reads the
  master from a submix's slot. Gate 2 decodes a frame with `S > 0` through the shipped module.
- **The exact-field validator.** A message with a field the host does not expect fails the whole host
  with a sticky 255. Every harness that posts a `miso.meter.v1` message must carry the three new
  fields.
- **The artifact digest.** The shipped module changes. Report CHANGED and re-pin only as the release
  step requires (`docs/RELEASE.md`).

## Objective gates

1. **Frame contents.** Native host-web test in `hosts/host-web/src/tests.rs`:
   - `T = 3`, `S = 2`, meters on, each track fed a distinct constant level per lane;
   - every submix peak word equals that bus's `PostMatrix` meter snapshot, and every track peak word
     and both master words equal today's computation;
   - every submix gain-reduction word is `0.0` (nothing is armed);
   - `submix_count == 2` and `struct_size == 72`.

   *Test value: it turns red if any writer still assumes `3T + 3`, if submix peaks land in the
   master's slots, or if a bus is unmetered.*
2. **Readers through the shipped module.** In `sdk/test/capability-evals.mjs` (the shipped frame path,
   `:135-147`), run by `bash scripts/check-sdk-headless.sh <A>`, boot a `T = 3`, `S = 2` session with
   meters on and check structurally: `peaks.length === 2T + 2`, `trackGrDb.length === T`,
   `submixCount === 2`, `submixPeaks.length === 4`, `submixGrDb.length === 2`, every value finite and
   non-negative, the bus peaks nonzero for a nonzero source. Compare against the frame's own shape,
   never against hard-coded rendered values.

   *Test value: it turns red if the SDK headless reader keeps the old shape check and refuses a frame
   with buses, or drops the submix sections.*
3. **The host validator.** `scripts/test-web-audioworklet.mjs` posts a 15-field message with
   `submixCount: 1` to the host and it is delivered; a message missing `submixGrDb`, or with
   `submixPeaks.length !== 2 * submixCount`, fails the host.

   *Test value: it turns red if the main-realm validator accepts a malformed submix section or still
   refuses the new fields.*
4. **`S = 0` is unchanged.** The existing frame tests pass with only the header-size update; the frame
   words of a session without submixes are byte-identical (`the_meter_frame_carries_the_app_shaped_gain_reduction`
   and the parity tests).
5. **Render allocates nothing** with bus meters on: after warm-up, a `render_next` plus meter poll
   measured with `crate::ffi::live_response_ffi_tests::measured` (the pattern at
   `hosts/host-web/src/tests.rs:3638-3647`) reports `allocations == 0` and `deallocations == 0`.

   *Test value: it turns red if the widened poll or the worklet's submix views allocate per window.*
6. **Layout, vocabulary and artifact.**
   - `python3 -B scripts/check-abi-layout-v1.py --self-test`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>`
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-headless.sh <A>`
7. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7).
   - `cargo test --locked -p parameter-metadata`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The ARTIFACT CHANGED report.
- The regenerated layout diff (header 64 to 72, the two new header fields).

### Attempt 1 record (Terra)

- **D1/D5** (`host-web/src/lib.rs`, anchors moved by #1207/#1208, found by symbol):
  `boxed_zero_meter_frame(strip_count)`; `poll_meters` uses `meter_count = meters.len()`,
  `strips = T + S`, master at `2(T + S)`, `gain_base = 2(T + S) + 2`, master GR at
  `gain_base + strips`; #1207's `track >= tracks` skip removed (only `u32::MAX` skipped). Field
  and function docs rewritten to `3(T + S) + 3`. **D2**: `WebMeterHeader` appends
  `submix_count: u32` (64) and `reserved_pad: u32` (68), 72 bytes, set from the ready submixes;
  no `ABI_VERSION` bump. **D3**: meters requested for `model.tracks` then `model.submixes` (the
  order of host-core's strip list; selected meters keep caller order). `ffi.rs` needed no change.
- **D4**: worklet reads `submix_count` from the header before the frame-capacity check
  (`(3(T + S) + 3) * 4`), builds five fixed views at construction (track peaks, bus peaks, master
  pair, track GR, bus GR; master GR index `3(T + S) + 2`); `peaks` is assembled with two `set`s
  (offset form, no view per block); `submixCount`/`submixPeaks`/`submixGrDb` appended. Host
  validator: exactly 15 fields, length and finite-non-negative rules for both bus arrays. Typings
  and cost note updated; `sdk/src/browser/shipped-host.d.ts` is a byte copy. SDK `MeterFrame`
  gains the three fields; `pollMeters` sizes by the header's `submixCount` and reads the master at
  `2(T + S)`.
- **Layout**: `meter_header_fields() -> [Field; 12]`; `check-abi-layout-v1.py` `meterHeader: 72`;
  regenerated `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`
  (`node codegen/assets.mjs && node codegen/generate.mjs`); the self-test fixture is again a byte
  copy of the asset. Diff: `"bytes": 64` -> `72`, plus `submixCount` (64, u32) and `reservedPad`
  (68, u32). Host-web pins (`tests.rs`) 72/64/68.
- **Harnesses**: every `miso.meter.v1` post in `test-web-audioworklet.mjs` carries the three
  fields; its processor stub now carries one submix (header 72, `submix_count` 1, a distinct value
  per section: tracks 0.5, bus 0.75, master 0.625, track GR 6.5, bus GR 2.25, master GR 7.5), so
  the worklet's S > 0 decode is exercised; `direct-oracle.mjs` header 72 and `submix_count == 0`.
  `qualification/*` read existing fields only: unchanged. Docs: `BUILTINS_AND_METERING_V1.md`
  frame shape, message fields, and the correction (GR is aged independently of the peak window).
- **Tests and test value** (each mutation applied, run, reverted; rows in
  `hosts/host-web/MUTATIONS.md`):
  - `host-web tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (gate 1;
    T = 3, S = 2, effect-free tracks with distinct fader pairs over one `(0.5, 0.25)` source; an
    S = 0 twin gives today's words): track words bit-equal to the twin's, each bus pair equal to
    its feeders' sum, master equal to the twin's master and to the bus sum (rel. 1e-6), every GR
    word +0.0, `submix_count == 2`, `struct_size == 72`. Red if a writer still assumes `3T + 3`,
    bus peaks land in the master's slots, or a bus is unmetered. Mutations: master at `2T`, frame
    `3T + 3`, track-only meters, `gain_base = 2T + 2`, `submix_count` 0 -> all red.
  - `host-web tests::bus_meters_render_and_poll_without_allocating` (gate 5; also closes #1207
    verdict MINOR-2): on #1207's bus-effect session (buses with console slots and a compressor
    insert, a meter on every strip, `t0`'s compressor tap armed), a window-closing `render_next`
    plus `poll_meters` under `measured` reports 0/0, the poll publishes one window and sets the
    gain-reduction validity bit. Red if the widened poll, its GR fold or the per-strip meters
    allocate per window (#1207's gate 4 never measured the poll). Mutations: a
    `submixes.len()`-sized `vec!` per window (allocates only when S > 0) -> red; boxing each folded
    GR value -> red.
  - `capability-evals.mjs` "a frame with submixes keeps the track shape and carries every bus"
    (gate 2, via `check-sdk-headless.sh`): structural only. Red if the headless reader keeps the
    old shape or drops the bus sections. Mutations: `strips = trackCount` -> red; empty
    `submixPeaks` -> red.
  - `test-web-audioworklet.mjs` (gate 3): a 15-field one-bus frame is delivered with its bus
    values; 16 new broken variants (missing `submixGrDb`/`submixPeaks`/`submixCount`, length
    mismatches, non-integer/negative count, plain arrays, negative/infinite/NaN values, an extra
    field) each fail the host with 255. Red if the validator accepts a malformed bus section or
    refuses the new fields. Mutations: drop the length rule, 12-field list, drop the GR value
    rule -> red. Worklet decode (modified existing assertions): master view at `2T`, bus peaks not
    copied, master GR at `3T + 2` -> red.
- **Gate 4**: no S = 0 frame test changed except the `gain_reduction` helper, which now reads
  `submix_count` (S = 0 there); all existing frame, parity and digest tests pass unchanged, and
  `check-browser-expected-resources.py` reproduces every pinned digest.
- **Gates** (x86_64 AVX2 host; A = `target/ci/k2-1209-artifacts`, B = `target/ci/k2-1209-named`):
  `--self-test` ok (22 mutations); `build-web-audioworklet.sh --named-twin` ok;
  `check-abi-layout-v1.py <A>/...layout.json` ok; `check-web-audioworklet.sh <A> <B>/...named.wasm`
  ok; `check-browser-expected-resources.py --artifacts` ok (32 red mutations);
  `test-web-audioworklet.sh` ok; `check-sdk-generated.sh` ok; `check-sdk-types.sh` ok (needs
  `sdk/node_modules`, copied from the primary checkout, identical lockfile); `check-sdk-headless.sh`
  ok. Workspace test command rc 0 (102 binaries, 1157 passed, 0 failed);
  `cargo test -p parameter-metadata` ok; `check-/test-realtime-policy.sh` ok; `cargo fmt --check`
  ok; workspace clippy `-D warnings` clean; `cargo doc -p host-web` `-D warnings` clean.
- **ARTIFACT CHANGED**: shipped module `f5d36ba0...00ad0` (2 693 332 B) against #1207's recorded
  `0a6e44c9...`; the bytes move because the frame, header and poll changed. No re-pin (not a
  release change); CI's `artifact-identity` line is the authority.

## Dependencies

- *Meter any boundary of a submix strip and designate a master strip in host-core* (#1208)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: append header fields and message fields; never renumber, reorder or reuse.
  No `ABI_VERSION` bump.
- The render callback stays allocation-free: build every view at construction.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
