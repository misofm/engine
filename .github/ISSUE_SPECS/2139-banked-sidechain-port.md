# A banked sidechain port (after the finish line)

Draft slice 9 (#2139) of the banking design. Reopens #96 F9 and decision 12's "a banked sidechain port
would be a future contract issue". Class A (additive contract; a keyed lane's arithmetic is the
per-node kernel's). Not needed for the finish line; recorded so the design's flexibility claim has
a path for the one shape that renders per node by contract rather than by cost.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

A connected sidechain blocks banking of the whole chain (`EffectProgramKey::blocks_banking`,
`rack/src/lib.rs:79`). A session with sidechained compressors on many tracks (a ducking bus) runs
every such strip per node.

## Smallest closable slice

- `PrepareEffectBankRequest` gains an optional key tile: when every lane of a cohort has the same
  connected port, the bank gathers the key signals into a second AoSoA tile and the kernel reads
  it per lane. The key stays in the program key (connected vs not), so cohorts stay homogeneous;
  a lane without a key never shares a bank with one that has (a key equal to the input is not
  proven bit-identical to the no-key path).
- The chain's window includes every lane's key-source tap edge (slice 6b (#2136)'s ASAP already does).
- Implemented for the compressor and the gate/expander first (the two keyed console-eligible
  effects); the others keep `blocks_banking`.

## Gates

1. Per effect: a two-lane bank with distinct keys equals two per-node renders word for word.
   Red on a kernel that reads lane 0's key for lane 1. Test value: red when a banked key is read
   from the wrong lane.
2. The planner banks a sidechained cohort and places it after every key source (one
   graph-compiler row).

## Paths

`crates/effect-contract`, `crates/rack` (the second tile), `crates/compressor`,
`crates/gate-expander`, `crates/graph-compiler/src/banks.rs`, this spec.

## Dependencies

Slice 6b (#2136). After the finish line.
