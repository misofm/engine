# Report the plan latency on the browser

One of the one-host differences of decision 17 C13 (`docs/rulings/one-control-abstraction-2026-10-13.md`).
Code anchors verified on `main` at `e2f6cfb24`. Issue number: assigned at filing (written here
as #1688). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A browser app reads the running plan's output latency and output tail, as a C ABI app does. After
a rebuild is adopted, the app reads the new plan's values. Both hosts read them from one core
function, so they always agree. An app that aligns a meter, a playhead or a recording to the
output can do it on the browser.

## Context

- **The core row.** Host-core reports a plan's latency as `HostPrepareReport::latency_samples`
  (`crates/host-core/src/prepare.rs:273`, filled at `:3134` from the output arrival). The control
  plane copies it into `PlanResources::latency_samples` (`crates/control-plane/src/compile.rs:1003`;
  the field is `crates/control-plane/src/adapter.rs:104`, beside `output_tail: TailSamples` at
  `:106`). The control plane keeps one row per plan epoch.
- **The core read.** `PlanQueries::resources()` (`crates/control-plane/src/plan.rs:97`) copies the
  row of the active plan epoch (`active_resources`, `:49`). It runs on any thread, beside render.
- **The C ABI.** `miso_engine_v1_plan_resources` (`crates/capi/src/ffi.rs:930`) writes that row
  as `PlanResourceReport`: `latency_samples`, `tail_kind`, `tail_samples`
  (`crates/capi/include/miso_engine_v1.h:366-368`, `crates/capi/src/abi.rs:283`). The tail
  encoding is in the capi adapter: `From<PlanResources> for PlanResourceReport`
  (`crates/capi/src/runtime/mod.rs:217-222`), with `TAIL_FINITE` 0 and `TAIL_INFINITE` 1
  (`crates/capi/src/abi.rs:37-39`).
- **The browser today.** The browser runs the same control plane (#1381a,
  `hosts/host-web/src/lib.rs:6-16`), and its adapter's row is `PlanResources` itself
  (`WebAdapter`, `lib.rs:5243-5244`). It holds `queries: PlanQueries<WebAdapter>` (`lib.rs:1514`).
  But it exports no latency and no tail. Its `WebResourceReport` (`lib.rs:1216`, exported by
  `miso_engine_web_v1_resource_ptr`, `hosts/host-web/src/ffi.rs:3450`) is a byte projection made
  once at boot (`lib.rs:2220`) and is never refreshed. It has no latency field.
- **The pattern to follow.** The watermark is a core read the browser copies into a fixed
  record: `read_watermark` (`lib.rs:2522-2544`) fills `WebPlanWatermark` (`lib.rs:1189`) from
  `ready.queries.watermark()`. The exports are `miso_engine_web_v1_watermark_read` and
  `miso_engine_web_v1_watermark_ptr` (`ffi.rs:3434`, `:3442`; #1381a D5).
- **Parity.** `hosts/host-web/tests/control_parity.rs` plays one script through both hosts. The
  script already holds a rebuild that moves the plan's latency: a strip's `delay_samples`
  (`control_parity.rs:226`, `:727`).
- **Floors.** *Keep every node's latency from dropping during playback* (#1285) makes the
  reported latency the floored output arrival (its D4). Its batch-15 amendment notes that the
  browser reports no latency, so its gate 3 covers the C ABI only.

## Decisions frozen for this slice

- **D1. One core function.** Both hosts read the latency and tail through
  `PlanQueries::resources()`. The tail's two-word encoding moves into the control plane as one
  function, `control_plane::tail_words(TailSamples) -> (u64, u64)`, with the two kind constants
  beside it. The capi adapter's `From` impl calls it, and the C ABI's `TAIL_FINITE` and
  `TAIL_INFINITE` keep their values. No new latency logic.
- **D2. The browser form.** `miso_engine_web_v1_plan_latency_read(handle) -> u32` copies the
  active row's `latency_samples` and the D1 tail words into a fixed record, `WebPlanLatency`:
  `struct_size: u32`, `reserved0: u32`, `latency_samples: u64`, `tail_kind: u64`,
  `tail_samples: u64`, `reserved: [u64; 4]` (56 bytes). `miso_engine_web_v1_plan_latency_ptr(handle)
  -> u32` returns the record's stable address. A host that is not ready returns
  `RESULT_WRONG_STATE` and leaves the record alone, as `read_watermark` does. The read runs where
  the plan queries are (the worklet's message handler in `single` mode; #1332 moves it with the
  rest of the control half).
- **D3. Mirrors.** Both exports and the record's offsets join every export list and layout
  mirror, as the watermark's did.
- **D4. Parity row.** `control_parity.rs` compares, after every scripted edit and at the end, the
  browser record's three words with the C ABI report's `latency_samples`, `tail_kind` and
  `tail_samples` for the same plan.
- **D5. The acked-batch question.** This is a read. It acks nothing and drops nothing.

## Deliverables

1. D1 in `crates/control-plane/src/` and `crates/capi/src/runtime/mod.rs`.
2. D2 in `hosts/host-web/src/lib.rs` and `hosts/host-web/src/ffi.rs`, with native tests.
3. D3 in the checkers, the generator and the generated SDK files.
4. D4 in `hosts/host-web/tests/control_parity.rs`.

## Authorized paths

- `crates/control-plane/src/plan.rs` or `crates/control-plane/src/adapter.rs` (D1),
  `crates/control-plane/src/lib.rs` (re-export only), `crates/capi/src/runtime/mod.rs` (the `From`
  impl only), `crates/capi/src/abi.rs` (the constants' source only, values unchanged)
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/control_parity.rs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py` and
  `scripts/fixtures/abi-layout-v1-self-test.json`, `tools/parameter-metadata/src/abi_layout.rs`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`,
  `scripts/check-sdk-generated.sh`, `scripts/test-web-audioworklet.mjs` (the mock's export list
  only)

## Non-goals

- No SDK API. A later issue gives the SDK a `planLatency()` call on this export.
- No change to how latency is computed (#1285 owns the floors) or to the C ABI's report.
- No refresh of `WebResourceReport`. It stays a boot projection.
- The other one-host differences of C13.

## Objective gates

1. **Parity (D4).** In `control_parity.rs`, at each launch rate the test already runs, the
   browser record equals the C ABI report's three words after every scripted edit and at the
   end. The `delay_samples` edit changes `latency_samples` on both hosts; the test asserts that
   the value moved, so the row is not vacuous.
2. **The read (native, `hosts/host-web/src/tests.rs`).** Before boot completes, the read returns
   `RESULT_WRONG_STATE` and the record keeps `struct_size` 0. After boot, it returns OK and the
   record holds the boot plan's latency and tail. After a rebuild that adds a true-peak limiter
   insert, the record keeps the old values until the adoption block, and holds the new plan's
   latency from the first read after it.
3. **Mirrors.** `python3 -B scripts/check-abi-layout-v1.py` and its self-test pass, and fail if a
   list omits an export or a record offset.
4. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p capi`, `cargo test --locked -p control-plane --features control-plane/test-support`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/test-web-audioworklet.sh`, `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`
   - `bash scripts/check-capi-abi.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 turns red if the browser computes latency or the tail by logic of its own, reads a
  stale plan's row, or encodes the tail differently from the C ABI.
- Gate 2 turns red if the read reports the candidate's latency before render adopts it, or keeps
  the boot value after a rebuild (a boot-time copy like `WebResourceReport`).
- Gate 3 turns red if a mirror lets the SDK call a name the module lacks.

## Dependencies

- Decision 17 C13 lists "plan latency and tail" as a one-host difference for root to file.
  This is that issue.
- *Keep every node's latency from dropping during playback* (#1285), ordering: land this after
  #1285, so the parity row reads floored latencies. #1285 then needs no browser gate of its own;
  this row covers the browser.
- #1381a has landed (the browser runs the control plane). #1332 (open) moves the read with the
  control half; it is not a dependency.

## Size

Half a day.

## Amendment (controls coordinator, batch 15a fold, 2026-10-16)

- **D2's record size.** `WebPlanLatency` is 64 bytes, not 56: `struct_size` and `reserved0` (4 + 4),
  `latency_samples`, `tail_kind` and `tail_samples` (3 x 8), `reserved: [u64; 4]` (32), with no
  padding. D2's "(56 bytes)" reads "(64 bytes)". The ABI layout mirrors (D3) record 64.
- **Decision 17 C13.** The two exports are listed there (root, 2026-10-16) as a transport read of
  the core's `PlanQueries::resources()` row, so they are not a C13 exception.

## Decision record: #1688 (batch 15b)

**Shipped.**

- D1: `control_plane::tail_words(TailSamples) -> (u64, u64)` with `TAIL_FINITE` (0) and
  `TAIL_INFINITE` (1), in `crates/control-plane/src/adapter.rs`, re-exported from `lib.rs`. The
  capi adapter's `From<PlanResources>` calls it, and capi's `TAIL_FINITE` and `TAIL_INFINITE` are
  now defined as the control plane's constants (values unchanged).
- D2: `WebPlanLatency` (64 bytes, per the batch 15a amendment), `PLAN_LATENCY_BYTES`,
  `AudioWorkletEngineHost::read_plan_latency` and `plan_latency`, and the exports
  `miso_engine_web_v1_plan_latency_read` and `miso_engine_web_v1_plan_latency_ptr`. The read copies
  `PlanQueries::resources()` and the `tail_words` encoding; host-web computes nothing. The record
  is all zero until the first successful read.
- D3: mirrors in `tools/parameter-metadata/src/abi_layout.rs` (exports 102 -> 104, structure
  `planLatency`), `scripts/check-abi-layout-v1.py` (exports, structure, field names, two new
  self-test mutations), `scripts/check-web-audioworklet.sh` (export list),
  `scripts/fixtures/abi-layout-v1-self-test.json`, `sdk/assets/miso-engine-v1-abi-layout.json` and
  `sdk/src/generated/abi.ts` (regenerated by `sdk/codegen/assets.mjs` and `generate.mjs`). No
  change to `scripts/check-sdk-generated.sh` or the mock's export list was needed.
- D4: `control_parity.rs` compares the browser record with `miso_engine_v1_plan_resources` before
  and after every block of the scripted run, at each launch rate (this covers the boot plan, every
  plain successor of the script's rebuilds and the end).

**Departures.**

- The spec says a strip's `delay_samples` moves the plan latency. It does not: a track delay
  shifts the output and leaves the PDC report alone (`graph-compiler` `track_delay.rs` P2-3). The
  moving rebuild is four added true-peak limiter inserts on `t1` (the added latency makes `t1` the
  longest path); `a_rebuild_that_adds_latency_moves_the_plan_latency_alike_on_both_hosts` asserts
  the value moved.
- "Before boot completes" has no host object: a handle that names no booted host returns
  `RESULT_INVALID_ARGUMENT` from the export. The `WRONG_STATE` and zero-record case is a host
  whose state is not ready (the native test sets `STATE_FAILED`).
- Mechanical: `crates/capi/src/runtime/tests.rs` calls `tail_words` in two places and spells
  `crate::TAIL_INFINITE` in one.

**Gates.**

- Gate 1: `control_parity`: `both_hosts_give_the_same_result_for_the_same_control_edits` (the
  record equals the C ABI report after every block, every launch rate),
  `a_rebuild_that_adds_latency_moves_the_plan_latency_alike_on_both_hosts` (pending values stay
  until the adopting block, then both move, equal) and
  `removing_a_limited_track_keeps_the_plan_latency_alike_on_both_hosts` (#1285 floors: both hosts
  report the same, unchanged latency after the swap, and a fresh compile of the committed model
  reports less). Result: 17 of 17 pass.
- Gate 2: `tests::structural_apply::the_plan_latency_record_follows_the_active_plan` (host-web
  lib): not ready leaves the record zero, boot copy of the core row, old value while the limiter
  rebuild waits, equal to a fresh boot after adoption. Passes.
- Gate 3: `check-abi-layout-v1.py --self-test` (32 mutations caught) and the artifact check in
  `check-web-audioworklet.sh` pass against a built module, so the module exports both names.
- `check-browser-expected-resources.py` passes with no re-pin (bridge retained bytes 1549094 of
  1667648).
