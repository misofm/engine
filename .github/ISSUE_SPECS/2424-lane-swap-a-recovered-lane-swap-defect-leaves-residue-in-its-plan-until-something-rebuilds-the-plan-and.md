# Lane swap: a recovered lane-swap defect leaves residue in its plan until something rebuilds the plan, and nothing does

Successor of #2419. That issue made every render-time lane-swap defect a recovered, counted outcome. A slot whose swap strands, or that refuses its swap-back, has both lanes reset to the defaults they hold (`reset_lane(FullToDefaults)`, with the `RESTORED` symmetry term withheld). The graph pins both strips for the plan's life, and `graph::lane_swap_defect_count(plan)` reads the total. The recovery is containment, not repair. This issue owns the repair.

## Problem
A reset ends the other strip's effect history in a refusing slot, but not everything the slot holds. Until the plan is replaced, the slot keeps:
- the other strip's **prepared defaults**: its parameter values, which the reset re-seeds the lane from;
- the other strip's **non-effect per-lane parts**:
  - the control lane: cells, applied bypass and its crossfade, carried writes. The lane map is unchanged, so this strip's own live writes land in a control lane that holds the other strip's cells. Every cell it never writes keeps the other strip's value;
  - the observation taps;
  - the dry-shunt line, which holds up to a latency of the other strip's dry input;
- the whole lane of a stage with no lane reset, which is `BankStage::reset_swapped_lanes`'s default. A builtin bank's `BuiltinStage` is such a stage.

So for the rest of the plan's life, a strip whose swap hit a defect renders with some of another strip's parameters in the refusing slot. Its own live writes only partly replace them. The engine counts the defect, but nothing acts on the count. The correct end state is a fresh plan: preparation seeds every lane from its own strip's session values.

## Contract
- A nonzero lane-swap defect total leads to a replacement plan, prepared from the committed session, through the existing control-plane path.
  - The issue decides whether the engine requests the rebuild (for example, a flag the next service call reads) or the host does after reading the total.
  - It records the reason for the choice. Today the engine owns no thread, and the host drives deferred control work through bounded service calls (D15-17).
- Render stays allocation-, lock- and syscall-free. Render may at most raise a flag, inside a `REALTIME_POLICY` region. Preparing the replacement is control-plane work.
- The plan carry into the replacement must not carry the residue:
  - the defaults are the successor's own, as the carry already guarantees;
  - a pinned strip's lane state either carries as any lane does, or is left as prepared. The issue decides which, and says why.
- The rebuild is visible: an outcome flag or a counter shows that a defect-driven rebuild happened, so it cannot loop silently.
  - A bank that strands at every episode must not cause a rebuild storm.
  - The #2419 pin fixes only the two strips of a trade that hit the defect. Other strips of the same banks stay movable, and their trades can still hit it. Within one plan the bound is at most one defect per pair of unpinned strips in a group: each defect pins its pair, so a group of `n` strips sees at most `n/2` defects before every strip is pinned.
  - The issue states that bound and the bound across plans. A fresh plan starts with no pin, so the rebuild policy must not be re-triggered without limit by a bank that strands at every plan.
- No behaviour change on a healthy plan: no rendered bit moves.

## Non-goals
- Putting the total on the wire (the owner-ruling successor of #2419).
- Making a lane swap infallible by type.
- Third-party banks.

## Gates
- A host-core test drives a plan whose swap strands. A test bank or hook strands a chosen swap, built off the render thread. The test asserts:
  - a replacement plan is prepared and adopted;
  - the strip's prepared defaults and control lane are its own again;
  - the counter or flag records one defect-driven rebuild.
- A test that a bank that strands at every plan does not rebuild without bound.
- Allocation counters stay zero on render.
- A no-bit-moved comparison on a healthy plan.
- Mutation evidence for every new branch, in debug and release.
- Gates for every touched crate:
  - `cargo test`;
  - clippy `-D warnings`, `cargo fmt --check` and `cargo doc -D warnings`;
  - `scripts/check-workspace-policy.sh` and `realtime_policy`;
  - the web call-graph gate, if render dispatch changes.

## Evidence / attempt record
(empty)

## Notes from #2419's review
- **The carry's way home bypasses pins.** `Runtime::restore_home_lanes` (graph runtime.rs ~11665-11699) trades through `swap_bank_lanes` and does not read pins. It runs on the predecessor at carry (graph lib.rs ~7908). A pinned strip that is still off its home lane is traded again there, can hit the same defect, and then refuses the whole carry (`CarryOutcome::PredecessorMismatch`), so the replacement starts with nothing carried. This issue must decide how that is handled: restore skips or tolerates pinned strips, or a refused carry is accepted and counted. State the bound including this path.
- **The bound counts trades, not counter increments.** At most n/2 defect-hitting trades per group of n strips per plan, from the packer, plus the carry path above. One trade can add `1 + 2·split rows` to `lane_swap_defect_count`, so the counter can exceed n/2.

## Note from the #2425 ruling (2026-11-09)
`docs/rulings/bank-contract-defect-counter-2026-11-09.md` puts both totals on the wire as one accumulating `CounterId`, `BankContractDefects = 23`, in a separate successor issue. For this issue it rules:
- add no `CounterId` here; keep reading `graph::lane_swap_defect_count` in process, and rebuild on the lane-swap total only (a refused carry leaves no residue);
- build the render-side rebuild flag on the lane-swap defect path (the arms that already reset lanes, inside `REALTIME_POLICY`), so the counter issue can carry the running total through the same path rather than a parallel one; whichever lands second reuses the first's signal;
- "the rebuild is visible" is met in process here (a tested tally and bound); a host sees a defect-driven rebuild as counter 23 rising with the plan epoch, so this issue needs no wire surface of its own.
