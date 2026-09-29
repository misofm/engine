# Soft-clip refuses its own subnormal snapshot on restore

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 3", seed 0).

## Problem

Soft-clip can produce an in-memory state snapshot containing a subnormal value, then refuses that same snapshot when it is restored (for example across a plan replacement). That breaks the snapshot/restore round trip and the effect's documented denormal behaviour: a state the effect produced must restore.

## Smallest closable slice

Either flush the subnormal when the snapshot is taken, consistent with the effect's denormal rule, or accept it on restore; state which and why against the effect contract. Un-ignore the reproducer.

## Gates

1. The reproducer passes; a snapshot/restore round trip is identity for every state the effect can produce, subnormals included.
2. Hostile (not self-produced) snapshots are still validated as before.
3. Console digests unchanged; render allocation-free.
