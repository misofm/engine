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

## Decision record

### D1 Where the query lives (attempt 1, 2026-10-22)

`effect-contract` `PreparedNativeEffectBank::lanes_at_rest(&self) -> u32`, default `0`.

- `BankKernel` (`effect-runtime`) has no production implementor: only a test fixture in
  `effect-runtime/tests/bank.rs` uses it. Every console slot is a `Box<dyn PreparedNativeEffectBank>`,
  which is what `rack` (#1107's consumer) holds, and #1747's `swap_lanes` belongs on the same bank
  stage contract. #1746's three effects implement the same method.
- A mask, not `fn(lane) -> bool`: one call answers every lane, as `observe_resident_bank` does,
  because a bank's state is vector state. #1107 ANDs the masks of a chain's slots per bank.
- Clocks (words a `+0.0` block advances by its length alone: the EQ's silence counters, the
  limiter's per-bank cursors) are not state words for the claim; a skip advances them as the
  bank's own fast path does.
- Padded lanes answer by the same predicate on their own state. The bank-wide fast path asks for
  every lane, padded included, which is what today's whole-bank observation covered.
- A collapsed bank's frozen right channel keeps the answer sound (it can only clear a bit).

### D2 The bank-wide fast path

Each effect keeps its cached claim, earned at the end of a block whose input was quiet, with the
same withdrawals (target, reset, restore, automation). The earning test becomes "the rest mask
has every lane set" in place of "the observed block left the state bits unchanged". Rendered bits
cannot move: the fast path is still "write nothing" under a sound claim.

### D3 Shared gate helper

`effect_runtime::bank::rest_probe`: one randomized driver and check (`Probe`, `Schedule`,
`check`, `Reach::assert_reached`). Each effect's test binds its own bank at `Simd4` and `Simd8`
through crate internals (`lane::each_vector_lane!`), so the Simd4 leg runs on this AVX2 host
without #1694.
