# Arm a later mono chain across a per-node op between its chains

Filed by root as the successor that #987's decision record names (backlog sprint, 2026-10-24).

## Product outcome

#987 (PR #1898) arms the mono collapse of a later chain on a split strip when every lane's producer is a same-track bank unit that collapsed in the same block. When a per-node op sits between the two chains (a TrackDelay, a pan or matrix op, a route or a sum), that op becomes the colour's last writer, `input_producer_table` (`crates/graph/src/runtime.rs`) gives the later chain no row, and the chain renders dual. The #970 measurement put the cost of rendering dual on realistic mono sessions at +42 % to +48 %.

## Smallest closable slice

1. For each per-node op kind that can sit between two chains of one strip, state whether it maps an L == R input to an L == R output exactly (bit for bit), and under which parameters. A TrackDelay with equal left and right delays does; a pan or a 2x2 matrix in general does not; state the exact condition for each.
2. Carry the producer's per-block symmetric fact through each op that preserves it, and arm the later chain on that fact, as #987 does for a bank producer.
3. One red-first test in the crate that owns the table: a later mono chain behind a symmetry-preserving per-node op arms and renders the dual bits; behind a symmetry-breaking op it stays dual.

## Constraints

Class A: no rendered bit moves. Owner rule kept: arm only when the consumer's own tracks are eligible (mono source). The memory rule "dual mono by source file" holds.

## Paths

`crates/graph/src/runtime.rs`, `crates/graph-compiler` tests, `crates/host-core/tests/collapse_arming.rs`, this spec.
