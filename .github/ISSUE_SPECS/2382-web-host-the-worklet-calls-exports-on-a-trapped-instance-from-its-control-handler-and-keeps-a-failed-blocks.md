# Web host: the worklet calls exports on a trapped instance from its control handler, and keeps a failed block's output

Non-goal shared by #2291 and #2370. In `worker` mode, `receiveControlPlane` in `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (around line 660) calls `miso_engine_web_v1_meter_header_ptr` for the meter view, and `control_allocation_count` for render status, without checking the failure record. After a trap, those calls run exports on a trapped instance. #2291 contains the resulting throw, but the call should not be made at all.

A second case in the same file: in `single` mode, when the meter header says "replaced" but no matching successor set exists (`postMeterFrame`, around line 1728), the worklet calls `fail(RESULT_INTERNAL, false)` and returns normally. `process()` then keeps the block's output and folds telemetry from a failed instance. Found by #2370's fold review.

## Contract
- After the failure record says trapped, the control-plane handler calls no export. A request that needs one is answered from the failure record, as the dispose already is (#2291).
- A block that fails inside `process()` for any reason, trap or not, outputs silence and folds no telemetry, the same as the trap path since #2370.

## Gates
- Red on base: a `worker`-mode row that traps the instance, then sends a control-plane message that reads the meter view and render status, and asserts that no export runs. A `single`-mode row with a replaced header and no successor set asserts silence and no telemetry frame.
- `bash scripts/test-web-audioworklet.sh` green, and the browser matrix in CI.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `29f238005`. The red rows are `9a396259d`; the fix is the commit after it.

**Claims re-verified at base.** Both hold, none dropped.
- `worker` mode, `receiveControlPlane` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`). The meter view is guarded only by `ended()` (line 657), and a trapped instance is `failed`, not ended, so it calls `meter_header_ptr` (line 665) and can cut a pending set. The status read calls `control_allocation_count` (line 681) whenever the phase is not final.
- `single` mode, `postMeterFrame`. A header replaced with no matching successor set calls `fail(RESULT_INTERNAL, false)` and returns (line 1728). `process()` had already copied the block's output, and it goes on to `recordRenderTime` (line 1844), which can post a telemetry frame from the failed instance.

**Fix** (worklet only, no Rust or ABI change).
- `process()`: after the meter frame, `if (this.phase !== PHASE_LIVE)` silences the block and returns. The meter-frame `catch` now only records the trap. So a failure the frame finds, thrown or not, ends the block the way a render failure does. The pinned `if (this.meterLease) this.postMeterFrame();` spelling is kept.
- Meter view: it returns unless the phase is `live`. A failed instance renders silence and posts no frame, so it needs no view, and a trapped one may call no export.
- Status read: a trapped instance is answered from the failure record. `readStatus()` reads the status record in shared memory, never an export, and it already takes `failureCode` as `lastResult` (255). `singleModeControlAllocations` is the count last read before the trap. The count is read at the end of `initializeWorker` and again at every status read while untrapped (new field `controlAllocations`). So the reply keeps its shape and its code, and the Worker's `answerStatus` is unchanged.
- I chose this over answering `status: null`, which would make the Worker answer from its own record. That reply carries the Worker's `lastResult`, not 255, so a page status read that raced the render check would report a healthy render half.

**Red on base** (`9a396259d` over the base worklet; each assertion run with the ones before it removed in a scratch copy):
- `single`, replaced header with no successor (the existing #1290d1 row, extended to hold the telemetry lease and fault on the window's 128th block):
  - `AssertionError: a failed block posts no frame` / `4 !== 3` (the telemetry frame);
  - then `a failed block is silenced`.
- `worker` (new row):
  - `worker mode: a trapped instance cuts no view` (a pending set was cut);
  - `the count is the last one read before the trap` / `7 !== 6`;
  - `no export runs on the Worker's port after a trap`, actual `['miso_engine_web_v1_meter_header_ptr', 'miso_engine_web_v1_control_allocation_count']`.

**Mutations of the fix** (the hermetic suite over a mutated copy, `MISO_ENGINE_WEB_WORKLET_TEST_MODULE`). All 7 are red.

| Mutation | First failing assertion |
|---|---|
| M1 the failed-block branch does not silence | `a meter_poll trap silences its block` (#2370's row, which shares the branch) |
| M2 the failed-block branch does not return | `a meter_poll trap: the trapped block posts no telemetry frame` |
| M3 only a thrown meter-frame failure fails the block (the base shape) | `a failed block posts no frame` |
| M4 the meter view is guarded by `ended()` only (base) | `worker mode: a trapped instance cuts no view` |
| M5 the status read calls the count export when trapped | `worker mode (read before: true): the count is the last one read before the trap` |
| M6 no boot read of the count | `worker mode (read before: false): the count is the last one read before the trap` |
| M7 no refresh at an untrapped status read | `worker mode: an untrapped status read reads the count` |

**Realtime.** The `process()` change only moves the silence and return out of the `catch` behind one phase compare. The healthy path gains one string compare and allocates nothing. The source policy (`check-web-audioworklet.sh --source-policy`) passes. All Chromium `render-allocations` and `sdk-render-allocations` rows are 0.

**No bit moved** (one-time PR evidence, not a committed test).
- One shipped module (`79fc2304...`, built here with `nightly-2026-08-20`) was run in the Chromium `single` and `worker` legs, once with the base worklet JS (`a07cff52...`) and once with this change's (`b97fe6dd...`).
- All four render digests the logs print are identical in both legs:
  - `structural-replace-large` `383be91c...`
  - `warm-growth` `25784310...` (twice)
  - `structural-edit` `c51d28da...`
- Every run passed all qualification gates, including the native corpus digest.

**Gates.**
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.
- `bash scripts/check-sdk-headless.sh <artifact>`: 717/717. `bash scripts/check-sdk-generated.sh <artifact>`: current.
- Chromium `single` and `worker` (`npm run qualify -- --check-matrix --self-test-mutations`): `all qualification gates passed (151.0.7922.34)`, base and head. The browser's system libraries came from #2370's private unpack.
- WebKit is not run here, for #2370's reason (the WPE build's `libjxl.so.0.8` and `libbacktrace.so.0` need root to install). It is left to CI's browser matrix.
- No Rust changed, so cargo test, clippy, fmt, rustdoc and the realtime policy tool do not apply.

**Test value.**
- `single` row (the #1290d1 "replaced with no set cut" row, extended): red if a non-trap meter-frame failure keeps the block's output or folds the block into telemetry. No existing row checked a non-throwing failure after the output copy.
- `worker` row (new): red if the control Worker's port calls an export on a trapped instance (meter view or status), or if the trapped status loses the last count read (boot or status). No existing row sends either message after a trap.
- No existing test was made useless; the #1290d1 row was extended rather than duplicated.
