# Fuse a ramping output block that carries a timed fader operation

Successor of #890 (2026-10-22).

## Problem

#890 fuses the ramping fader, mute and matrix pass, but `try_process_ramping_with_matrix_and_operations` (`crates/builtins/src/lib.rs`) declines when a timed fader operation falls inside the block, as the settled path does (#1647 D1.5). Such a block takes the split path: fader L, fader R and matrix passes. Fader, mute, send and VCA automation therefore keeps its banks on the split path for every block that holds an automation point.

## Smallest closable slice

Cut the block at each operation offset and run the fused ramping pass on each span, applying each operation at its offset exactly as the split path's `driver` does. Class A: no rendered bit moves, and no operation is dropped or reordered (the acked-batch question: every operation pulled is applied; `driver.finish()` consumes nothing the fused path skipped).

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs` (dispatch), their tests
- `crates/lane/src/kernels/builtins.rs` only if a span helper is needed
- this spec

## Objective gates

1. A builtins test: random operation schedules (fader, mute and VCA offsets, including offset 0, the last frame, and two operations at one offset) on a bank with ramps in flight; PCM bits and every lane word equal the split path.
2. The existing decline test (`the_fused_settled_path_declines_only_an_operation_inside_the_block`) is rewritten to the new rule in the same PR, not duplicated.
3. host-core's stored fader, mute and VCA automation tests pass unchanged.
4. Render allocates nothing (`bench_support::alloc` thread counters after warm-up); `tools/realtime-policy` passes.
5. One descriptive run of the fader-ride row (#1750) with automation, if that row exists by then.

## Dependencies

- #890 merged.
- #1754: the fused ramping pass runs on product plans only after #1754 pairs the fader and matrix banks under concurrent delivery (#890 correction, 2026-10-22).
