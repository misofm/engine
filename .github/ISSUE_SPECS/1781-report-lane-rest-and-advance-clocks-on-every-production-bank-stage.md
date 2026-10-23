# Report lane rest and advance clocks on every production bank stage

Split from #1107 (D5 item 1, root approval 2026-10-23). A building block, the same kind as #1745 and #1746.

## Problem

Only `PreparedNativeEffectBank` has `lanes_at_rest` and `advance_clocks` (#1745, #1746, #1747). The rack `BankStage` trait has neither, and the builtin input, fader and matrix banks have no rest predicate. Every production console chain holds those stages, so no chain can report a lane at rest, and #1107's whole-bank skip cannot start.

## Smallest closable slice

- `BankStage::{lanes_at_rest, advance_clocks}`, declining by default, on:
  - `EffectBankStage`: forwards to the effect;
  - `LiveControlEffectBankStage`: the effect's mask minus every lane with a staged span, a staged prepared target, a bypass, a running crossfade or an armed observation tap after this block's drain; its clock advance also feeds the shunt line a `+0.0` block through `feed_line`;
  - `BuiltinStage`: forwards through `GraphPreparedBuiltinBankProcessor`.
- Builtin input, fader and matrix banks: a state-only predicate under #1107 D3 (one `+0.0` block leaves every state word bit-equal and writes `+0.0`):
  - input: every integrator at `+0.0` bits, no trim or filter ramp in flight; a lane whose signed trim or polarity can turn `+0.0` into `-0.0` declines unless the kernel proves otherwise;
  - fader: no gain or mute ramp in flight;
  - matrix: not ramping, and no output row whose two coefficients both carry the sign bit;
  - processor level: no automation span this block.
  - Clocks: the input's silence counter (`lane::silence_advance`); the fader and matrix have none.
- `BankChain::lanes_at_rest()`: the AND over the slots; a slot that is an identity on a lane counts as at rest on it.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/effect-contract/src/live.rs` (the live stage's predicate and shunt feed)
- `crates/builtins/`, `crates/builtins-compiler/src/lib.rs`, `crates/graph/src/lib.rs` (the two declining trait methods), `crates/graph/src/runtime.rs` (`BuiltinStage` forwarding)
- their tests; this spec

## Objective gates

1. One rest-probe test per new predicate, in the crate that owns it (the #1745 driver form): it renders a `+0.0` block and fails when a reported lane is not an exact fixed point (state words and output).
2. Each `advance_clocks` equals a rendered `+0.0` block on its clocks.
3. Allocation-free; fmt, the workspace clippy line, `tools/realtime-policy`, the AArch64 test check, `cargo doc -D warnings`.

## Test value

Gate 1 turns red when a predicate claims rest while a word would still change; gate 2 when a skipped lane's clock drifts from a rendered one.

## Dependencies

#1745, #1746, #1747, #1763, #1765 (merged). Blocks #1107.
