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
  words). The outcome flags fit one `u64` word. The flag bits are #1314's (`EXACT = 1`,
  `PREROLL_FALLBACK = 2`, `TRANSITION_FALLBACK = 4`, `SUPERSEDED = 8`). Boot writes the initial watermark
  (`(initial revision, 0, EXACT)`); `render_next` overwrites the three words when D1 is `Some`.
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

## Authorized paths

- `crates/engine/src/realtime/plan_exchange.rs`, `watermark.rs`
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (status words only; stream H owns
  this crate, so root orders this after #1381)
- `tools/parameter-metadata/src/abi_layout.rs`, `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/check-abi-layout-v1.py` and its fixture

## Non-goals

- The SDK API (`engine.apply`'s completion promise) and the Worker loop (stream H, #1381, #1382).
  Any producer of non-`EXACT` flags (#1310, #1357, #1358).

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
5. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh` (with the build command in qualification.yml's
     browser job) and `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if the browser computes its own application sample instead of copying render's
  watermark (it would report r1 or r2 before the swap).
- Gate 2: red if the report carries a value on every block or misses the advancing one.
- Gate 3: red if `render_next` writes the status words from a different advance than the one the
  record published (for example, a block late).
- Gate 4: red if a mirror keeps the old `reserved` words or overlaps #1399's word at 48.

## Dependencies

- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Swap and retire browser plans through the Worker's service loop* (#1381): the service and
  watermark-and-counters exports, and the `SessionState::watermark()` accessor D3 reuses.
- *Report live_values_superseded in the browser status and prove both hosts drain strip cells
  alike* (#1399): it takes offset 48 first.
