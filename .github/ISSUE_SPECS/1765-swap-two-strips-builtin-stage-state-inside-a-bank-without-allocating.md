# Swap two strips' builtin stage state inside a bank without allocating

Split from #1747 (verifier finding 4, 2026-10-23). Needed by #1107.

## Problem

#1747 adds a render-time lane swap for effect bank stages, the live control stage and the dry shunt line (`BankStage::swap_lanes`, `BankChain::swap_lanes`). Production strip chains also hold `BuiltinStage` slots (`crates/graph/src/runtime.rs` near 5760 and 7900-7935). These keep the default `can_swap_lanes = false`, so `BankChain::swap_lanes` refuses every chain that the graph builds. That fails safe, but #1107 cannot pack a production console until builtin stages swap too.

The builtin stages carry per-lane state: the input section (polarity, trim, HPF, LPF and their ramps), the fader and mute ramps, the matrix ramps, timed operations, and their live control cells.

## Smallest closable slice

- Implement `can_swap_lanes` / `swap_lanes` for each builtin bank stage that a production strip chain holds: input section, fader, matrix. Include the fused fader+matrix stage only if it still exists when this starts (#1754's open owner question).
- Swap every per-lane field: state, coefficients, ramps (current, target, step, remaining), pending timed operations, live control cells and their dirty bits. Record a field class table in this spec, as #1747 D2 does.
- Allocate nothing. Decide every refusal before the first write.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, `crates/graph/src/runtime.rs` (only the builtin stage's `BankStage` impl)
- the tests of those crates
- this spec

## Objective gates

1. One randomized chain test on a graph-built strip chain: render N blocks, swap two strips (in one bank and across banks, mid-ramp, with a pending timed operation and different live values per lane), render M more with the inputs swapped too; every strip's output is bit-identical to the no-swap twin. Simd8 and, where built, Simd4.
2. The swap allocates nothing (`bench_support::alloc`, folded into the gate 1 test).
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a builtin swap leaves one per-lane field behind, such as a ramp target or a pending timed operation. No test can see this today, because builtin stages refuse every swap.

## Dependencies

- #1747 (the swap contract).
- #1754 owner decision on the fused fader+matrix stage.
- Blocks #1107.
