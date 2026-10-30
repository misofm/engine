# Charge a bank's processor bytes in the bank row

Filed by root from the #2219 verify (attempt 1 MINORs M1 to M3). Follow-up; not launch-blocking.

## Problem
After #2219 the projection and the exact rows charge a processor only for a per-node effect instance (`host-core/src/prepare.rs:4243`, `per_node_processor_bytes`). A bank's own processor, its box, any undeclared heap and its padded lanes are charged by no row and no contract test: `effect_bank_resource` (`graph-compiler/src/banks.rs:1786`) charges scratch and metadata only. Before #2219 the per-member processor charge covered these bytes by accident. Today the graph estimate's slack covers them (about 40 % on the dense shape; the #2219 verify measured 193 MB live against 328 MB charged at 7,000 tracks), but nothing holds it.

Failure scenario: a future banked effect with a large undeclared heap (for example a banking reverb) is admitted with that heap charged nowhere, and the boot or a replace allocates past the budget.

## Scope
1. Charge each bank's processor bytes (box, declared state, undeclared heap, padded lanes) in the bank row, in the projection and in the exact rows, with the same rule in both.
2. One contract test in the crate that owns the bank row: the bytes live after `compile_projected` are at most the charged bank and per-node rows, over the launch effects that bank, with padded banks.
3. `graph-compiler/tests/projected_compile.rs:229` asserts the peak against every instance's processor, the old charge. Bound it by the per-node and bank charge that the host now uses.
4. The drift test has no fixture with a per-node instance, so it never exercises `per_node_processor_bytes`. Add one per-node instance to an existing drift fixture rather than a new fixture.

## Gates
- The contract test is red when a bank's undeclared heap is left out of the bank row.
- The browser boot projection stays under 1 % over exact on the dense shape.
- No rendered bit moves (one-time comparison, PR evidence only).
