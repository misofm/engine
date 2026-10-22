# Report one console lane's exact rest in the EQ, compressor and limiter banks

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 2. First of two predicate slices; #1746 is the second.

## Problem

#1107 skips a lane only when its input block is exact `+0.0` and every console slot reports that
lane at its exact fixed point. The parametric EQ, compressor and true-peak limiter already have a
silent fast path (#163), but each decides per bank (for example the limiter's `is_at_silent_rest`,
`crates/true-peak-limiter/src/lib.rs` near `:792`). No effect can answer the question per lane.

## Root ruling (2026-10-22, #1107 D3)

An exact fixed point does not need a `RestSamples` bound. A lane is at rest when a per-lane
predicate holds whose meaning is: from this lane's state, one block of exact `+0.0` input leaves
every state word of the lane bit-identical and outputs exact `+0.0`. The predicate reads state
only; it never renders a block to find out. `RestSamples` (#1372, #1375) bounds *when* the
predicate becomes true; it is not needed for correctness.

## Smallest closable slice

- Add a per-lane rest query to the bank contract (`effect-runtime` `BankKernel` or
  `effect-contract` `PreparedNativeEffectBank`; choose one and record why).
- Implement it for the parametric EQ, the compressor and the true-peak limiter. The bank-wide
  fast path becomes "every member lane is at rest", so the bank decision keeps today's bits.
- Ramps in flight, a latency or ring word that is not `+0.0`, or any non-zero envelope or filter
  state make the predicate false.

## Authorized paths

- `crates/effect-runtime/src/bank.rs`, `crates/effect-contract/src/lib.rs` (the query only)
- `crates/parametric-eq/`, `crates/compressor/`, `crates/true-peak-limiter/` (source and tests)
- this spec

## Objective gates

1. Per effect, at Simd4 and Simd8 (Simd4 where the host builds it, see #1694), a randomized test:
   drive random lanes with random signal then exact silence; at every block where the predicate
   holds for a lane, one more zero block leaves that lane's state bits equal and its output
   `+0.0`. The test also reaches states where the predicate is false.
2. The existing silent fast-path tests (for example `a_silent_padded_bank_takes_the_silent_fast_path`)
   pass unchanged. No rendered bit moves (the one-time base comparison is PR evidence).
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1's test turns red when a predicate reports rest while a state word is still non-zero, a
defect that today's bank-wide tests miss because they only check whole silent banks.

## Dependencies

None. #1372 and #1375 stay open and independent.
