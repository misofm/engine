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
### Attempt 1 (Terra), branch `codex/issue-2424-defect-rebuild`, base `48ca47617`

**Claims re-verified at base `48ca47617`.** Every claim holds; none is dropped.
- A defect resets both lanes and pins both strips, and nothing else acts on it: `Runtime::swap_bank_lanes`'s defect arm (`crates/graph/src/runtime.rs` 11636) ends in `lane_map.pin_positions` and `return Err(outcome)`. The only reader of the total is `graph::lane_swap_defect_count` (`crates/graph/src/lib.rs` 5143), which no production code calls.
- The residue is real on the launch banks. The branch's end-to-end test strands a real trade (below): each strip's composed EQ response becomes the other strip's, which is the other's prepared defaults rendering in its lane.
- The carry's way home ignores pins: `restore_home_lanes` (runtime.rs 11665) trades through `swap_bank_lanes`, and `take_predecessor` (lib.rs 7908) refuses the whole carry (`CarryOutcome::PredecessorMismatch`) when it fails.

**Red on base.** On the base, nothing reads the defect, so the residue lasts for the plan's life. The branch's main test, `a_lane_swap_defect_rebuilds_the_plan_once`, needs this branch's rebuild to pass. Its base equivalent is mutant C1 below (the service step never rebuilds), which is red at "each strip renders its own defaults again".

**Decisions.**
1. **The engine requests the rebuild, not the host.**
   - Render raises a flag. The control thread prepares the replacement in the service step, which the host already calls (D15-17): `SessionState::service_step` calls `rebuild_lane_swap_residue` after the epochs and the warm deadline step.
   - Reason: the engine holds the plan, the pin and the bound. A host can read neither per-plan total (the #2425 ruling), and the engine owns no thread. Both hosts run this one service step: `miso_engine_v1_service` and `host-web`'s `service`.
2. **The render-side signal**, which #2427 reuses (the #2425 ruling's constraint).
   - `graph::GraphLaneSwapDefectWatch` holds, per plan, one `AtomicBool` per distinct track of the pack groups, plus an `AtomicU64` with the plan's recoveries total. It is shared through an `Arc` with the control side.
   - It is raised only in `swap_bank_lanes`'s defect arm, inside the existing `REALTIME_POLICY` region. There, `LaneMap::quarantine` replaces the arm's `pin_positions`: it pins as before, then stores both strips' flags and the total `lane_swap_defect_total(units)`, read after every trade-back. That is three release stores and one bounded fold over the units, with no allocation, lock or syscall.
   - A healthy trade runs no new instruction.
   - The plan's preparation takes the watch from the bound plan (`GraphBoundPlan::lane_swap_defect_watch`) into its inventory (`PlanStateInventory::lane_swap_defects`). #2427 can read `recoveries()` there per epoch: the value only grows, and it is final once the plan retires.
3. **What a successor does with the residue: pinned strips are left as prepared, never carried.**
   - Every successor's preparation restarts each strip of its base's residue. This is not only the rebuild: host-core adds the residue to the restart set where the transition fallback adds its forced restarts. The owners before the fader and the fader start at rest, the fader is armed, the strip is duck-swapped, and it is listed in `restarted_strips`.
   - Why not carry the lane state: none of it is known to be the strip's own. The payload's designed words came from the other strip's defaults. The control lane holds the other strip's cells. The graph cannot tell which slots of a row still hold traded state (#2419). The carry's defaults guarantee covers only the defaults.
   - Exceptions:
     - A declared discontinuity starts every owner at rest anyway. Arming a fade there would make two strips fade in where every other strip starts at once.
     - A warm successor's floors are derived from its lead's restart set. Its lead is taken from the ordinary successor, which restarts the residue, so the residue restarts there too. A defect render raises between the two preparations is not in the lead, and restarting it would break the warm invariant.
   - Each exception is handed on at promotion (decision 5).
4. **No rebuild storm: a session-wide quarantine.**
   - Each successor pins its base's quarantine plus its base's residue, at bind (`GraphBoundPlan::pin_lane_swap_defect_strips`, the #2135 pin by track). This set is filtered to the successor's model and recorded as `PlanStateInventory::lane_swap_quarantine`.
   - So the packer never trades a strip of a defect pair again in the session. A bank that strands at every plan strands each pair once.
   - A strip the session removes leaves the quarantine. Re-added, it is a new strip and movable again, which keeps the quarantine bounded by the model.
5. **A defect raised after a successor was prepared is inherited.**
   - At promotion, the control plane calls `PlanStateInventory::inherit_lane_swap_residue` with the displaced plan's inventory. That plan's flags are final by then. Every strip of its residue that the promoted plan has and did not restart joins the promoted plan's residue, and the next service step rebuilds.
   - Not at a declared discontinuity, which carried nothing but sources.
   - The inheritance cannot tell a carried strip from one whose whole carry was refused. In that rare case (below) it rebuilds once for nothing.
6. **The rebuild** is `Rebuild::LaneSwapResidue`.
   - It prepares the committed model as an ordinary successor of the running plan. It carries like a transaction with no edit, and commits like a monitor rebuild at the committed revision (0 included), with the session's spectrum configuration.
   - It is never classified warm, so its restarts always run.
   - It runs only while nothing is pending: a pending candidate's adoption hands over what it did not restart (decision 5).
   - It is attempted once per running epoch and residue count. A refused rebuild is counted and is retried only when the epoch changes or the residue grows. Any later successor, for example one an edit makes, restarts the residue anyway.
7. **The carry's way home (`restore_home_lanes`): unchanged; a refused carry is accepted and counted.**
   - The carry tables name a lane by its home location, so every strip must be home. A pinned strip that is still away from home holds another strip's home, so skipping it would leave that strip away too.
   - When the trade home hits the defect again, that is a recovered defect: it is counted in the predecessor's total and raised on its watch. The whole carry is then refused, counted in `RealtimePlanOwner::carry_mismatch_count`, and the successor starts as prepared, which holds no residue.
8. **Visible.** `SessionState::lane_swap_rebuilds()` returns `(published, refused)`, in-process and never reset. A host sees a rebuild as the plan epoch advancing, and from #2427 on as counter 23 rising (the #2425 ruling). No `CounterId` is added.

**The bound.**
- **Trades within one plan.** Each defect-hitting packer trade pins a pair of strips that were unpinned. So a group of `n` movable strips sees at most `floor(n/2)` such trades, plus at most one per pair on the carry's way home.
- **Across plans.** Each pair is quarantined for the rest of the session, so a session sees at most `floor(N/2)` defect-hitting trades over `N` distinct grouped strip IDs. A strip that is removed and re-added counts again.
- **Counter increments.** The counter counts recoveries, not trades: one trade can add `1 + 2·split rows`.
- **Rebuilds.** Each rebuild follows a growth of the residue (a new pair, or a late flag inherited at promotion). So there are at most as many rebuilds as defect pairs, plus at most one refused attempt per running epoch and residue count.

**Tests.**
- `graph` `runtime::tests`, on the #2419 rig, whose units now name their strips, with the rig arming the watch:
  - the split-strip test and the stranded-trade test also assert the watch: both strips flagged and the whole total, `1 + 2·split rows`;
  - `a_defect_pins_the_strips_at_its_positions_by_home` now calls `quarantine` and also asserts the flags at the two home positions of a group at `base` 8;
  - new: `only_a_defect_raises_the_defect_watch`. A healthy trade, and a refusal the check makes, raise nothing and pin nothing.
    - Test value: a raise on every refusal or on every trade, which would rebuild a healthy plan.
- `control-plane` `tests/lane_swap_residue.rs`, new. It runs on the nine-track EQ session, on real banks, through `SessionState::service` and `PlanState::render`. The strand comes from `effect_contract::test_only_strand_lane_swaps`: the next bank lane swap rotates the two lanes' defaults and returns `Stranded`, as a broken bank's would. The trade is `graph::test_only_swap_bank_lanes`, the packer's own move. The forced trade allocates nothing.
  - `a_lane_swap_defect_rebuilds_the_plan_once`, at revision 0: the residue is visible (each response is the other strip's), one rebuild is published, each strip is its own again, the successor pins the pair, the defect count is 0, and nothing more is rebuilt.
  - `a_refused_rebuild_is_not_retried_and_any_successor_restarts_the_residue`: one refusal, never retried, and an edit's successor restarts and pins the pair.
  - `a_successor_adopted_after_a_late_defect_is_rebuilt`: the inheritance at promotion.
  - `a_residue_rebuild_is_never_warm`: a forced `Warm` answer does not reach the rebuild.
  - `a_warm_edit_over_residue_commits_and_a_late_defect_is_rebuilt`: a defect raised between an edit's ordinary and warm preparations, through the test preparer and `GraphLaneSwapDefectWatch::test_only_raise`. The edit commits, and the late strip is inherited and rebuilt.
  - `a_declared_discontinuity_over_residue_needs_no_rebuild`: no fade armed, no inheritance and no rebuild, while the pins hold.
  - `the_quarantine_holds_across_successors_and_drops_a_removed_strip`.
  - `a_removed_residue_strip_rebuilds_nothing`: the storm guard on a strip the successor lacks.
- The gate asked for a host-core test. The trigger is control-plane's service step, the code both hosts run, so the test lives in the crate that owns the logic. It drives host-core's preparation end to end.
- No existing test is made useless: the extended assertions sit on the existing defect scenarios.

MUTATION_TABLE

**No bit moved.** NO_BIT_MOVED

**Gates.** GATES_RECORD

**Follow-ups (not filed).**
- #2427 (the counter) reads `PlanStateInventory::lane_swap_defects().recoveries()` per epoch and sums the retired ones: the signal this issue built.
- A whole-carry refusal on the way home makes the inheritance rebuild once for nothing (decision 5). It is bounded at once per pair. Telling the two apart would need the carry outcome on the control side.

## Notes from #2419's review
- **The carry's way home bypasses pins.** `Runtime::restore_home_lanes` (graph runtime.rs ~11665-11699) trades through `swap_bank_lanes` and does not read pins. It runs on the predecessor at carry (graph lib.rs ~7908). A pinned strip that is still off its home lane is traded again there, can hit the same defect, and then refuses the whole carry (`CarryOutcome::PredecessorMismatch`), so the replacement starts with nothing carried. This issue must decide how that is handled: restore skips or tolerates pinned strips, or a refused carry is accepted and counted. State the bound including this path.
- **The bound counts trades, not counter increments.** At most n/2 defect-hitting trades per group of n strips per plan, from the packer, plus the carry path above. One trade can add `1 + 2·split rows` to `lane_swap_defect_count`, so the counter can exceed n/2.
