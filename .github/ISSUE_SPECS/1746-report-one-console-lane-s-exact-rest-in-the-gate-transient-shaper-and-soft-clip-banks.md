# Report one console lane's exact rest in the gate, transient shaper and soft clip banks

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 2. Second of two predicate slices; #1745 is the first.

## Problem

The gate/expander, transient shaper and soft clip have no silent fast path at all, so a console
slot that holds one of them can never skip a lane. #1107 needs every console-eligible effect to
answer "is this lane at its exact fixed point?".

## Root ruling

As in #1745 (#1107 D3): a state-only per-lane predicate whose meaning is that
one zero block leaves the lane's state bits unchanged and outputs exact `+0.0`. No `RestSamples`
bound is needed.

## Smallest closable slice

- Implement the per-lane rest query that #1745 adds to the bank contract, for
  the gate/expander, the transient shaper and the soft clip.
- Add each effect's bank-wide silent fast path as "every member lane is at rest", so whole silent
  banks skip as they do in the EQ, compressor and limiter. The soft clip's shared `pos` cursor
  still advances in a skipped block.

## Authorized paths

- `crates/gate-expander/`, `crates/transient-shaper/`, `crates/soft-clip/` (source and tests)
- this spec

## Objective gates

1. The same randomized predicate test as the first slice, per effect.
2. A whole silent bank renders bit-identically with the fast path and without it.
3. Render allocates nothing (`bench_support::alloc` thread counters after warm-up).
4. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a predicate claims rest while an envelope, detector or cursor-dependent
word would still change; gate 2 turns red when the new fast path skips work that changes bits.

## Dependencies

- #1745 (EQ, compressor, limiter), which adds the bank-contract query.
- #1376 stays open and independent.
