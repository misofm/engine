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

## Decisions (root rulings on the attempt-1 plan)
- D1 (Q1). The source before bind is the plan's own transient bind, which #2199 D1 already makes. `plan_group_banks` measures `bank_processor_bytes = size_of_val(&*processor) + processor.retained_bytes()` on the bank it binds, keeps it in `PlannedEffectBank::processor_bytes`, then drops the bank. No new factory API.
- D2. One rule on both sides. `bind_planned_banks` measures the compile's bank with the same function. A different value is the new code `graph.effect.bank_resource` (Q3), beside `graph.effect.bank_metadata`.
- D3. The charge is the bank row: `GraphResourceEstimate::effect_bank_processor_bytes`, added by `checked_add_effect_banks` to `incremental_plan_bytes` and `session_plus_plan_bytes`. It is in `largest_allocation_bytes` as a ceiling (box plus its heap). Both admissions and `host.graph.projection` read it through `session_plus_plan_bytes`, with no host change. The C ABI report and the canonical graph text do not change.
- D4 (Q2). The members' declared state and scratch rows stay beside the bank row. This is a ceiling: a bank holds its members' state inline, so the two overlap. On the dense shape the overlap is at most the effect row, 56,896,000 B (8.0 % of the exact total at head).

## Evidence (attempt 1, x86-64-v3, eight lanes)
- Contract test `graph-compiler/tests/projected_compile.rs::a_banks_bytes_are_within_its_row`: 11 tracks, the console EQ and eight banking inserts per track, one per-node delay. Every banking effect binds a padded bank. Allocator live bytes equal the charge for all nine effects (true-peak limiter 194,176, dynamic EQ 53,440, EQ 47,488, soft clip 27,456, multiband 13,440, de-esser 12,864, compressor 8,064, gate 6,336, transient shaper 4,160; row 367,424). So no launch bank under-reports its heap; no `retained_bytes()` fix was needed. The four-lane run is the AArch64 CI legs' (a factory binds no foreign width, `console.slot.foreign_width`); it was not run locally.
- Red: with `retained_bytes()` left out of the rule, the row is 135,232 against 367,424 live. With the true-peak limiter's `retained_bytes()` halved, the row is 272,768 and the limiter shows 194,176 live against 99,520 charged.
- Drift fold: the console representative has one per-node delay. Red when the per-node row also charges the delay's declared state (over 775,996 against a bound of 9,510). Bounds re-pinned: nine-track 205, console 1,407 per million.
- C ABI reference: both graph rows rise by +47,488 (the EQ banks). Ceilings re-pinned (eight lanes measured; four lanes raised by the same delta plus 10 %, unmeasured).
- Dense shape (7,000 tracks), boot projection over exact: base 238,000 B / 627,576,439 B = 0.038 %; head 238,000 B / 710,680,439 B = 0.033 %. Exact rises by 83,104,000 B of bank processors and is 32.4 % over 512 MiB.
- Dense gates refuse before allocation: native boot peak 343,361,120 (base) and 343,393,888 (head) of the 384,115,980-byte parse transient; replace peak 337,084,980 and 337,117,748 of the same transient; wasm refusal growth 299,565,056 of the 536,870,912-byte budget.
- No rendered bit moved: the 34 digests over `fixtures/session/v1` (300 blocks, plain and live) are identical at base 19ad6b8ff and at head.
