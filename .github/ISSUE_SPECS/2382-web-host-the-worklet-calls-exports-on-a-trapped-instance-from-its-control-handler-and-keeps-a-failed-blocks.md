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
(empty)
