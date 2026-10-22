# Swap two lanes' state inside a console bank without allocating

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 2.

## Problem

#1107's mechanism (D1, "swap-to-pack") moves an active lane out of a bank that it would keep
running and into a silent lane's position in a bank that runs. That needs a render-time swap of
two lanes' full per-lane state in every slot of a console chain. None exists: `carry_lane_from`
(`crates/rack/src/lib.rs` near `:911` and `:1382`) restores a lane through a payload at a plan
swap, and it is not a render-time operation.

## Smallest closable slice

- Add `swap_lanes(a: (bank, lane), b: (bank, lane))` to the bank stage contract. It exchanges the
  two lanes' state, coefficients, ramps, control lanes and taps, and allocates nothing.
- Implement it for the six console-eligible effects, for `LiveControlEffectBankStage` (controls,
  taps) and for the rack `BypassShunt` line.
- Shared per-bank cursors (the limiter's `Cursors`, the soft clip's `pos`) are not per lane and
  do not swap; the slice records that every bank of one slot keeps equal cursors, so a swapped
  lane lands on the same cursor.
- Padded lanes never swap.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/effect-runtime/src/bank.rs`, `crates/effect-contract/src/live.rs`
- the six effect crates (parametric-eq, compressor, gate-expander, soft-clip, transient-shaper,
  true-peak-limiter): source and tests
- this spec

## Objective gates

1. A randomized test per effect and for a whole chain: render N blocks, swap two random lanes
   (in one bank and across banks), render M more blocks with the inputs swapped too; every
   strip's output is bit-identical to the run with no swap. Simd8 and, where built, Simd4 (#1694).
2. `swap_lanes` allocates nothing (`bench_support::alloc` thread counters after warm-up).
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a swap leaves one state field behind (for example a ramp target or a tap),
which no current test can see because no swap exists.

## Dependencies

None. It can run beside the predicate slices.
