# Count and report every strip that a plan swap restarts

Owner decision (2026-10-23): add this counter in optimisation batch 4. Found by the design review of 2026-10-23.

## Problem

A structural edit that cannot carry a strip's stage state starts that strip at rest (`resolve_strip_carry` and the effect and strip-delay carries, `crates/host-core/src/prepare.rs` near 3734-3766, 5997-6044, 6252-6304). The fade-in that decision 15 D15-9 asks for is not implemented (#1288, #1324, #1325 are open), so a restart is a step in the output, and nothing counts or reports it: `PreparedHost::restarted_strips()` (`prepare.rs` near 729) is read only by tests, and the C ABI header says values on a lane that a swap restarts are not counted (`crates/capi/include/miso_engine_v1.h` near 70-71). Decision 15 requires an outcome flag or counter for every fallback (D15-17). An app cannot see that an edit restarted strips it did not touch.

## Smallest closable slice

- In the core: when render adopts a plan, add that plan's restarted-strip count to one monotone session counter, `strips_restarted`, next to `live_values_superseded`. The count is decided at prepare (the list exists); render only adds it at adoption, allocation-free. A withdrawn or never-adopted candidate adds nothing.
- Report it as one new protocol counter read with `COUNTERS_GET` on the C ABI, and through the same core counter frame in the browser and the SDK. Hosts stay thin: no host computes it.
- Correct the header sentence and the SDK doc.

## Authorized paths

- `crates/host-core/`, `crates/control-plane/`, `crates/protocol/` (the counter and its frame)
- `crates/capi/` (the counter id, the header doc, its parity row)
- `hosts/host-web/` and `sdk/` (the counter read and its parity row)
- this spec

## Objective gates

1. One core test: a structural edit that restarts N strips adds exactly N when render adopts the plan; a candidate that is withdrawn adds 0; an edit that restarts none adds 0.
2. One parity row per host: the C ABI and the browser report the same value for the same edit.
3. Render allocates nothing; `tools/realtime-policy` passes; fmt, the workspace clippy line, `cargo check --target aarch64-unknown-linux-gnu --tests` and `cargo doc -D warnings` on touched crates.

## Test value

The core test turns red when an adopted restart is not counted, or when a candidate that never plays is counted. No current test reads restarts outside host-core's carry tests.

## Dependencies

None. Edit order: #1767 (the fused-stage removal) edits `resolve_strip_carry` in the same file and merges first.
