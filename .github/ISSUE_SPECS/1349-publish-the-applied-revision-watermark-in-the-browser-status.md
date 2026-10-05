# Publish the applied-revision watermark in the browser status

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The browser reports the same applied-revision watermark as the C ABI, with the same meaning. The
worklet's status block carries `(revision, first sample fully in effect, outcome flags)` after
every render that advances it, and the Worker's control plane can read the full record, counters
included, through one engine-side call that the Worker's watermark-and-counters export (#1381)
returns. The SDK's `engine.apply` (stream H) then reports completion from these fields instead of
the browser's own `applied_at_sample` arithmetic.

## Context

- `WebStatus` (`hosts/host-web/src/lib.rs:1279-1302`) is 80 bytes with `reserved: [u64; 4]` at
  offset 48 (`hosts/host-web/src/tests.rs:286`, `:411-413`). `render_next` writes it after each
  successful render (`next_absolute_sample` and `rendered_quanta`, `lib.rs:3242-3243`). It is read
  through `miso_engine_web_v1_status_ptr` (`hosts/host-web/src/ffi.rs:4366`).
- Its layout is mirrored by `tools/parameter-metadata/src/abi_layout.rs` (`status_fields`, `:318`),
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`,
  `scripts/check-abi-layout-v1.py` and `scripts/check-sdk-generated.sh`.
- The browser acks with `applied_at_sample = next_absolute_sample` (`hosts/host-web/src/lib.rs:3337`,
  `:3388`). That is exact only while admission runs on the audio thread between two renders, which
  *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) ends.
- The C ABI watermark, its record module and its reader come from *Publish an applied-revision
  watermark and complete edits asynchronously* (#1314: `crates/engine/src/realtime/watermark.rs`,
  `PlanPublisher::watermark_reader()`, render publication inside `RealtimePlanOwner`).
- The browser does not render through `RealtimePlanOwner` today; *Swap and retire browser plans
  through the Worker's service loop* (#1381) makes the worklet render through it, with the Worker
  holding the publisher and retirer and calling the service step of *Add miso_engine_v1_service for
  bounded control work between edits* (#1348). #1381 also adds the Worker's wasm exports this
  slice is read through: `miso_engine_web_v1_service(handle)` and a watermark-and-counters export,
  with their layout mirrors.
- *Report live_values_superseded in the browser status and prove both hosts drain strip cells
  alike* (#1399, D1) takes offset 48 for `live_values_superseded`, leaving `reserved: [u64; 3]`
  (offsets 56, 64, 72).
- **The JS readers check the reserved words.** The worklet's `readStatus()`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:944-960`) throws unless offsets 48, 56, 64
  and 72 read zero (`:946-949`), and its result is spread into the `miso.status.v1` reply
  (`:1070`). The main-realm host accepts that reply only with an exact field list and validated
  values (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1013-1017`, `:1121-1128`). The
  reply type is `MisoStatus` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:846-857`,
  copied in `sdk/src/browser/shipped-host.d.ts:846-857`). The hermetic harness's fake status
  reply lists the exact fields (`scripts/test-web-audioworklet.mjs:159-164`).
- #1381 D5 adds `SessionState::watermark(&self) -> Result<PlanWatermark, WatermarkBusy>` in the
  control-plane crate: a read of #1314's `PlanWatermarkReader`, counters included.

## Decisions frozen for this slice

- **D1. Render report.** `RealtimeRenderReport` (`crates/engine/src/realtime/plan_exchange.rs`)
  gains `watermark: Option<PlanWatermark>`: `Some` exactly on a block that advanced it (#1314 D3),
  with the values render just published. The worklet copies from it; it never reads the seqlock it
  writes.
- **D2. Status words.** After #1399, the three `reserved` words become `watermark_revision`
  (offset 56), `watermark_first_sample` (64) and `watermark_outcome_flags` (72). The record then
  reads, from offset 48: `live_values_superseded` (#1399), the three watermark words, and no
  reserved word; size stays 80 and no expansion word remains (D15-3 asks for exactly these three
  words). The outcome flags fit one `u64` word. The flag bits are #1314's: `EXACT = 1`,
  `TRANSITION_FALLBACK = 2` and `SUPERSEDED = 4`, with no pre-roll flag (D15-8 (round-5
  amendment)). The browser copies the word and assigns no bit of its own. Boot writes the initial
  watermark (`(initial revision, 0, EXACT)`); `render_next` overwrites the three words when D1 is
  `Some`.
- **D2a. JS readers.** Because boot writes `EXACT` at offset 72, every JS reader changes in this
  slice, or the first status read would throw:
  - `readStatus()` checks no reserved word any more (after #1399 it checks only 56, 64 and 72;
    this slice removes that check) and returns `appliedRevision` (offset 56), `appliedSample` (64)
    and `appliedOutcome` (72), each a `bigint`.
  - The main-realm host's status field list gains the three names, and its validation requires
    each to be a valid `u64`.
  - `MisoStatus` in both `.d.ts` copies gains `readonly appliedRevision: bigint`,
    `readonly appliedSample: bigint` and `readonly appliedOutcome: bigint`, marked `@internal`
    (the SDK's `engine.apply` is the public surface, stream H).
  - The harness's fake status reply carries the three fields.
- **D3. Worker hook.** This slice reuses the `SessionState::watermark()` accessor that #1381 D5
  adds; it adds no accessor of its own. The Worker reads the full record through #1381's
  watermark-and-counters export; the SDK exposes it (stream H). This slice adds no wasm export:
  the status words ride the existing `miso_engine_web_v1_status_ptr`.
- **D4. Single-instance mode.** A non-isolated page (#1332 D1 `single`) uses the same status words
  and the same render report; nothing differs.
- **D5. Acked-batch question.** No queue; the status words are a level. The ack bytes do not
  change here; the SDK's completion contract moves to the watermark in stream H.

## Deliverables

1. D1 in the engine.
2. D2 in `WebStatus`, boot and `render_next`; every layout mirror regenerated with the repository's
   generators.
3. D2a in the worklet reader, the main-realm host's status validation, both `.d.ts` copies and
   the hermetic harness.

## Authorized paths

- `crates/engine/src/realtime/plan_exchange.rs`, `watermark.rs`
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (status words only; stream H owns
  this crate, so root orders this after #1381)
- `tools/parameter-metadata/src/abi_layout.rs`, `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/check-abi-layout-v1.py` and its fixture
- `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (`readStatus()` only),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the status reply's field list and
  validation only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and
  `sdk/src/browser/shipped-host.d.ts` (`MisoStatus` only)
- `scripts/test-web-audioworklet.mjs` (the fake status reply and D2a's case only)

## Non-goals

- The SDK API (`engine.apply`'s completion promise) and the Worker loop (stream H, #1381, #1382).
  Any producer of non-`EXACT` flags (#1310, #1358, #1397).

## Objective gates

1. **Same meaning on both hosts.** A native host-web test drives the script of #1314's gate 2
   (a rebuild, then a live edit before the swap) through the browser host: the status words stay
   at the initial revision until the swap block and then read `(r2, swap block * quantum, EXACT)`,
   equal to the C ABI's `miso_engine_v1_plan_watermark` for the same script.
2. **Report only on advance.** Engine unit test: `RealtimeRenderReport::watermark` is `None` on a
   block with no revision change and `Some` with the published values on the advancing block.
3. **Status equals the record.** After gate 1's script, the three status words equal the
   `revision`, `first_sample` and `outcome_flags` that #1381's `SessionState::watermark()` returns
   for the same session.
4. **Layout.** `hosts/host-web` layout tests pin the new offsets (`live_values_superseded` 48,
   the watermark words 56, 64, 72, size 80); `python3 -B scripts/check-abi-layout-v1.py`
   and its self-test, `bash scripts/check-sdk-generated.sh` and the parameter-metadata `--check`
   pass, and each fails if one mirror omits a word.
5. **The status reply carries the words.** In `scripts/test-web-audioworklet.mjs`, a worklet
   whose fake status block holds `(r, s, EXACT)` at 56, 64 and 72 answers a status request with
   `appliedRevision === r`, `appliedSample === s` and `appliedOutcome === 1n`, and the host
   accepts the reply. A reply missing one of the three fields is refused by the host.
6. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh` (with the build command in qualification.yml's
     browser job) and `bash scripts/check-cross-targets.sh`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts` (it requires the two
     `.d.ts` copies to be equal) and `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if the browser computes its own application sample instead of copying render's
  watermark (it would report r1 or r2 before the swap).
- Gate 2: red if the report carries a value on every block or misses the advancing one.
- Gate 3: red if `render_next` writes the status words from a different advance than the one the
  record published (for example, a block late).
- Gate 4: red if a mirror keeps the old `reserved` words or overlaps #1399's word at 48.
- Gate 5: red if the worklet still rejects a nonzero word at 56-72 (every status read would
  throw from boot on, since boot writes `EXACT`), or if the host's exact-field check drops or
  refuses the new words.

## Dependencies

- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Swap and retire browser plans through the Worker's service loop* (#1381): the service and
  watermark-and-counters exports, and the `SessionState::watermark()` accessor D3 reuses.
- *Report live_values_superseded in the browser status and prove both hosts drain strip cells
  alike* (#1399): it takes offset 48 first.
