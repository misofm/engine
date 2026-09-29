# D7 recovery reports break the contract (frames vs blocks; missing counts)

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 5", fixed input).

## Problem

The D7 recovery's reports are inconsistent across effects: gate, multiband and soft-clip count frames where the contract counts blocks, and transient and limiter report no count at all. Hosts reading the report cannot compare effects or trust the number.

## Smallest closable slice

Make every effect's D7 report follow the contract's unit (blocks) and report its count; update the contract doc if the contract itself is ambiguous. Un-ignore the reproducer.

## Gates

1. The reproducer passes for every effect.
2. A planted frames-instead-of-blocks report turns it red.
3. Console digests unchanged.
