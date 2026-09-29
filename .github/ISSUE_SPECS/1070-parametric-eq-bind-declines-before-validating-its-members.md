# Parametric EQ: bind declines before validating its members

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 2", seed 0).

## Problem

The EQ's bank bind returns "decline" (fall back to per-node rendering) before it validates the bank's members, so an invalid member can be reported as a decline instead of a refusal. The bind contract has three outcomes (bind, decline, refuse), and validation must decide refusal before the decline test runs.

## Smallest closable slice

Validate every member first, then decide bind or decline. Un-ignore the reproducer.

## Gates

1. The reproducer passes; an invalid member is refused with its typed diagnostic whether or not the bank would otherwise decline.
2. A planted reordering (decline first) turns it red.
3. Valid sessions bind or decline exactly as before; console digests unchanged.
