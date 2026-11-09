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

**Red on base.** On the base, nothing reads the defect, so the residue lasts for the plan's life.
- The probe: the base's `graph`, `host-core` and `control-plane` sources, with this branch's `effect-contract` strand hook only, and the branch's test rig. It strands one trade, then renders and services 64 blocks.
- It fails with `assertion left == right failed: each strip renders its own defaults again`. `eq0`'s left curve is still `eq1`'s flat one (`-3.0103, -2.16, ...` dB, where its own is `-2.43, -1.45, ..., +8.97`).
- The probe was scratch only and is not committed.
- On the branch, the same scenario is `a_lane_swap_defect_rebuilds_the_plan_once`, which is green.

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
   - A declared discontinuity's successor lists every strip as restarted already, so it needs no case of its own.
   - The one exception is a warm successor, which adds no residue itself.
     - Its floors are derived from its lead's restart set, and the lead is taken from the ordinary successor, which restarts the residue. So the residue restarts there too.
     - A defect render raises between the two preparations is not in the lead, and restarting it would break the warm invariant. That strip is inherited at promotion (decision 5).
4. **No rebuild storm: a session-wide quarantine.**
   - Each successor pins its base's quarantine plus its base's residue, at bind (`GraphBoundPlan::pin_lane_swap_defect_strips`, the #2135 pin by track). This set is filtered to the successor's model and recorded as `PlanStateInventory::lane_swap_quarantine`.
   - So the packer never trades a strip of a defect pair again in the session. A bank that strands at every plan strands each pair once.
   - A strip the session removes leaves the quarantine. Re-added, it is a new strip and movable again, which keeps the quarantine bounded by the model.
5. **A defect raised after a successor was prepared is inherited.**
   - At promotion, the control plane calls `PlanStateInventory::inherit_lane_swap_residue` with the displaced plan's inventory. That plan's flags are final by then. Every strip of its residue that the promoted plan has and did not restart joins the promoted plan's residue, and the next service step rebuilds.
   - The inheritance cannot tell a carried strip from one whose whole carry was refused. In that rare case (below) it rebuilds once for nothing.
6. **The rebuild** is `Rebuild::LaneSwapResidue`.
   - It prepares the committed model as an ordinary successor of the running plan. It carries like a transaction with no edit, and commits like a monitor rebuild at the committed revision (0 included), with the session's spectrum configuration.
   - It is classified like any rebuild. If it comes out warm, its lead is derived from the ordinary successor, which restarts the residue, so it restarts the residue all the same.
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
  - `a_warm_edit_over_residue_commits_and_a_late_defect_is_rebuilt`: a defect raised between an edit's ordinary and warm preparations, through the test preparer and `GraphLaneSwapDefectWatch::test_only_raise`. The edit commits, and the late strip is inherited and rebuilt.
  - `the_quarantine_holds_across_successors_and_drops_a_removed_strip`.
  - `a_removed_residue_strip_rebuilds_nothing`: the storm guard on a strip the successor lacks.
  - `a_grown_residue_is_rebuilt_after_a_refusal`: the attempt record keys on the residue count as well as the epoch.
  - `a_session_with_no_route_slack_boots_with_its_defect_watch_charged`: the nine tracks with no route boot. Without the projection's watch row, the inventory exceeds its ceiling (`host.control.projection`), which the probe showed.
- The gate asked for a host-core test. The trigger is control-plane's service step, the code both hosts run, so the test lives in the crate that owns the logic. It drives host-core's preparation end to end.
- No existing test is made useless: the extended assertions sit on the existing defect scenarios.

**Mutation sweep** (`sweep.py`, `sweep2.py` and `sweep3.py` in scratch). Each mutant was applied alone. Its focused tests ran in debug and in release: graph's `runtime::tests` and/or `control-plane --test lane_swap_residue`. The file was then restored from git.

The final code has 39 mutants, and all 39 are red in both builds (the 34 below, then P1-P5, in `sweep4.py` and `sweep5.py`).

| Mutant | Where | Debug | Release | Red at (among others) |
| --- | --- | --- | --- | --- |
| G1: the defect path raises nothing | `LaneMap::quarantine` | red | red | graph defect tests; every e2e test |
| G2: only one strip raised | `quarantine` | red | red | the same |
| G3: the total stored is not the whole recoveries total | `swap_bank_lanes` | red | red | `a_row_that_refuses_its_trade_back_...` |
| G4: raise by position, not by home | `quarantine` | red | red | `a_defect_pins_the_strips_at_its_positions_by_home` |
| G5: raise drops the group's `base` | `quarantine` | red | red | the same |
| G6: bind arms no watch | `bind` | red | red | every e2e test |
| G7: the watch names the next lane's track | `arm_lane_swap_defect_watch` | red | red | graph rig tests; e2e |
| G8: every position names strip 0 | the same | red | red | the same |
| G9: `flagged_strips` filter inverted | the watch | red | red | `only_a_defect_raises_the_defect_watch`; e2e |
| G10: no total stored | `raise` | red | red | graph defect tests |
| G11: `pin_lane_swap_defect_strips` a no-op | bound plan | red | red | e2e (pins) |
| G12: the bound plan hands out an empty watch | bound plan | red | red | every e2e test |
| G13: a flag stored `false` | `raise` | red | red | graph defect tests; e2e |
| H1: the quarantine drops the residue | host-core prep | red | red | e2e (pins) |
| H2: the quarantine drops the base's quarantine | prep | red | red | `the_quarantine_holds_across_successors_...` |
| H3: the quarantine keeps strips outside the model | prep | red | red | the same (re-added `eq1`) |
| H4: the quarantine is not pinned | prep | red | red | e2e |
| H5: the residue is not restarted | prep | red | red | e2e (each strip its own again) |
| H7: the residue is restarted in a warm successor | prep | red | red | `a_warm_edit_over_residue_...` (the warm invariant refuses) |
| H8: the residue restart ignores the models | prep | red | red | `a_removed_residue_strip_rebuilds_nothing` (`StripsRestarted`) |
| H9: the residue drops the inherited part | inventory | red | red | `a_successor_adopted_after_a_late_defect_...` |
| H10: `inherit_lane_swap_residue` a no-op | inventory | red | red | the same |
| H11: inheritance keeps restarted strips | inventory | red | red | `a_lane_swap_defect_rebuilds_the_plan_once` (a second rebuild) |
| H12: inheritance keeps strips the plan lacks | inventory | red | red | `a_removed_residue_strip_rebuilds_nothing` (the storm) |
| H13: the residue merge drops the flagged part | inventory | red | red | e2e |
| C1: the service step never rebuilds | control plane | red | red | e2e (the base's behaviour) |
| C2: a rebuild while a candidate is pending | control plane | red | red | e2e |
| C3: a rebuild of a healthy plan | control plane | red | red | e2e |
| C4: a refusal retried at every call | control plane | red | red | `a_refused_rebuild_is_not_retried_...` |
| C5: the attempt keyed by epoch only | control plane | red | red | `a_grown_residue_is_rebuilt_after_a_refusal` |
| C6: a rebuild not counted | control plane | red | red | e2e |
| C7: a refusal not counted | control plane | red | red | `a_refused_rebuild_...` |
| C8: the promotion inherits nothing | control plane | red | red | `a_successor_adopted_after_a_late_defect_...` |
| C11: the residue rebuild commits as a transaction | `rebuild` | red | red | `a_lane_swap_defect_rebuilds_the_plan_once` (revision 0) |
| P1: the projection charges the watch for one strip | inventory ceiling | red | red | host-web `representative_retained_projection_...` (identity over-projection) |
| P2: the projection charges the quarantine rows for a boot | inventory ceiling | red | red | the same (nine-track) |
| P3: the projection leaves the watch out | inventory ceiling | red | red | `a_session_with_no_route_slack_boots_...` |
| P4: the plan walk leaves out the position table | `LaneMap::retained_bytes` | red | red | capi `capi_retained_bytes_charge_every_byte_...` |
| P5: the inventory leaves out the watch | `PlanStateInventory::retained_bytes` | red | red | the same |

Mutation numbers H6, C9 and C10 are missing from the table because the sweep showed their guards were redundant. Each was green in both builds, so the guard was removed rather than kept untested:
- **H6 and C9: discontinuity guards.** There was a residue restart guard and an inheritance guard at a declared discontinuity. A discontinuity's successor already lists every strip as restarted: the `StripsRestarted` probe read +9 for the nine-strip session. So neither guard changes anything, and both were removed, together with their test.
- **C10: never classify a residue rebuild as warm.** A warm lead's restart set starts from the ordinary successor's restarts, which include the residue (`warm_lead`, prepare.rs, "`R`'s join part: the ordinary successor's restart set"). So a warm residue rebuild restarts the residue too. The guard and its test were removed.

The code each green result names is gone, so no mutant is still green.


**No bit moved.** The #2392 instrumentation (`graph-compiler`'s `silent_bank_skip` gate 3: an FNV-1a digest over the moved plan's host PCM and both plans' observed words, `--release`, `digest2392.patch` in scratch, not committed).
- Base sources (`origin/main` `48ca47617` for `graph` and `effect-contract`): `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`.
- Branch: identical.

These are also the digests #2419 recorded. A healthy trade's only change is the defect arm's call, `quarantine`, in place of `pin_positions`, which a healthy trade never reaches.

**Gates.** Run one crate at a time with `-j 4` and one private target directory, deleted afterwards. Every run below passed, with none failing, on the final tree:
- `cargo test --locked`:
  - `effect-contract`, `graph`, `host-core` and `control-plane`, each with its `test-support` feature, in debug;
  - `graph --lib` and `control-plane --test lane_swap_residue` in release;
  - `graph-compiler`, `capi` and `host-web` (the hosts of the changed service step and inventory) in debug. No graph plan fixture moved.
- Clippy `-D warnings` on `graph`, `effect-contract`, `host-core` and `control-plane` (all targets), with and without `test-support` and in release: clean.
- `cargo doc -D warnings` on the same four crates: clean.
- `cargo fmt --all --check`: clean.
- Policy scripts, each ok:
  - `check-workspace-policy.sh` and `test-workspace-policy.sh`;
  - `check-graph-policy.sh` (PASS; `runtime.rs`'s test module stays plain) and `test-graph-policy.sh`;
  - `check-rack-policy.sh` (PASS) and `check-host-core-policy.sh`;
  - `check-effect-contract.sh`;
  - `check-test-support-ci.py` and `check-ci-path-routing.py`.
- `realtime_policy`: ok (315 marked regions in 48 files).
- `check-web-audioworklet.sh --without-metadata-regeneration`, on a rebuilt artifact and named twin with wabt: passed ("web AudioWorklet static/object checks passed").
  - `miso_engine_web_v1_render` has closure=38, as on #2419, with atomic_waits=0 and destructor_registration=0.
  - The strand hook is `test-support` only, so it is not in the artifact.

**Resource accounting.**
- The watch is charged where the inventory holds it, as the fade watch is: the inventory's clone keeps it alive past the plan. The plan's walk counts only its position table, so dropping the plan frees exactly what the walk names (capi's test).
- The inventory ceiling charges the watch at every strip, and only for a model of two strips or more: a pack group is two peer chains.
- It charges the quarantine and the inherited residue at every strip, and only for a successor: a boot plan pins none and is never promoted.
- The first gate run caught both rows. The walk counted the watch (capi), and the ceiling charged the quarantine rows on a boot (host-web's pinned over-projection).

**Follow-ups (not filed).**
- #2427 (the counter) reads `PlanStateInventory::lane_swap_defects().recoveries()` per epoch and sums the retired ones: the signal this issue built.
- A whole-carry refusal on the way home makes the inheritance rebuild once for nothing (decision 5). It is bounded at once per pair. Telling the two apart would need the carry outcome on the control side.

### Verdict on attempt 1 (Sol, `14eec47ef`): PASS, minors only
Seven minors, folded on the same branch after merging origin/main (#2425's ruling and its note on this spec are kept):
1. The preparation read the base's residue twice, for the quarantine and for the restarts, while render raises flags concurrently.
2. A retryable `Backpressure` refusal used up the epoch's one rebuild attempt.
3. Mutant H7s (`if successor` -> `if false` in the inventory ceiling, which never charges a successor's quarantine and inherited rows) was untested.
4. Mutant H6s (the order test swapped in `lane_swap_residue`'s merge) was untested.
5. The watch's doc did not name the happens-before edge that makes a displaced plan's flags final at promotion.
6. `publish_lane_swap_residue_rebuild` duplicated `monitor_rebuild`'s preamble; C7 (the `plan_alive` check dropped) must be red once it is shared.
7. `service_step`'s "Bounded (D3)" doc did not say that a leading service step may prepare the residue rebuild, that a following transaction supersedes it, and what that does to the next structural edit.

### Fold (Terra), same branch, with origin/main merged
1. **One snapshot.** `prepare_or_project` reads `base.inventory.lane_swap_residue()` once, after the warm lead is known, into a sorted, deduplicated `lane_swap_residue`. The quarantine and the restarts both use it, so they agree, and a strip flagged after it is inherited at promotion. No test can place a raise between the two old reads (both are inside one preparation call), so this is a construction fix with no mutant.
2. **Backpressure re-arms.**
   - The attempt is now keyed on `(running epoch, the watch's recoveries total, plans_reclaimed)`. `SessionState::plans_reclaimed` counts each displaced plan `synchronize_plan_epochs` reclaims.
   - So a refused rebuild is retried after a new plan runs (residue is inherited only at a promotion), a new defect is recovered, or a displaced plan is reclaimed, which frees what a `Backpressure` refusal waits for. It is never retried at every call.
   - The recoveries total replaces attempt 1's residue count, which needed the merge to be distinct (item 4).
   - New test `a_backpressured_rebuild_is_retried_after_a_reclaim`. It holds reclamation (`test_hold_reclamation`) across an edit's adoption, strands a trade, and injects an `AfterAdmission` fault (`Backpressure`). It expects one refusal, not retried over four blocks; then the hold is released and the reclaim retries and publishes.
3. **The successor's quarantine charge is tested.** `a_session_with_no_route_slack_boots_with_its_defect_watch_charged` becomes `a_session_with_no_route_slack_charges_its_defect_watch_and_quarantine`. After the routeless boot it strands four pairs (`Rig::strand_positions`, by group position, so it holds at any bank width) and requires the rebuild that quarantines eight strips to be admitted and adopted.
4. **No merge.** `lane_swap_residue` is now the flagged strips chained with the inherited ones. A strip in both is named twice, and every reader collects a sorted, deduplicated set or asks only whether it is empty. `lane_swap_residue_count` is gone, and the attempt key no longer counts strips, so nothing depends on order or distinctness.
5. **Happens-before, in `GraphLaneSwapDefectWatch`'s doc.** A flag a block raises (the carry's way home in the adoption block included) is stored before that block's render publishes its epoch: `control-plane` `PlanState::render`'s release store of `active_epoch` (plan.rs 409). `synchronize_plan_epochs` loads it with acquire (control.rs 4091) before it promotes, so the promotion's `inherit_lane_swap_residue` reads final flags.
6. **One preamble.** `SessionState::committed_rebuild_parts` (plan alive, revision in range, the committed model's copy, the unchanged overlay and subscription changes) is shared by `monitor_rebuild` and `publish_lane_swap_residue_rebuild`.
7. **`service_step`'s doc.**
   - The step prepares at most one plan: the deadline step's transition or the residue rebuild, only while nothing is pending.
   - Every session call runs the step first, so a call that finds residue publishes the residue rebuild before its own work.
   - A following structural transaction supersedes it: that submit prepares twice, and `lane_swap_rebuilds` counts the withdrawn rebuild as published.
   - While that rebuild is pending, its duck is in the transaction's base, so a growth that would be prepared warm takes the transition fallback (a duck).

FOLD_MUT

FOLD_GATES

## Notes from #2419's review
- **The carry's way home bypasses pins.** `Runtime::restore_home_lanes` (graph runtime.rs ~11665-11699) trades through `swap_bank_lanes` and does not read pins. It runs on the predecessor at carry (graph lib.rs ~7908). A pinned strip that is still off its home lane is traded again there, can hit the same defect, and then refuses the whole carry (`CarryOutcome::PredecessorMismatch`), so the replacement starts with nothing carried. This issue must decide how that is handled: restore skips or tolerates pinned strips, or a refused carry is accepted and counted. State the bound including this path.
- **The bound counts trades, not counter increments.** At most n/2 defect-hitting trades per group of n strips per plan, from the packer, plus the carry path above. One trade can add `1 + 2·split rows` to `lane_swap_defect_count`, so the counter can exceed n/2.

## Note from the #2425 ruling (2026-11-09)
`docs/rulings/bank-contract-defect-counter-2026-11-09.md` puts both totals on the wire as one accumulating `CounterId`, `BankContractDefects = 23`, in a separate successor issue. For this issue it rules:
- add no `CounterId` here; keep reading `graph::lane_swap_defect_count` in process, and rebuild on the lane-swap total only (a refused carry leaves no residue);
- build the render-side rebuild flag on the lane-swap defect path (the arms that already reset lanes, inside `REALTIME_POLICY`), so the counter issue can carry the running total through the same path rather than a parallel one; whichever lands second reuses the first's signal;
- "the rebuild is visible" is met in process here (a tested tally and bound); a host sees a defect-driven rebuild as counter 23 rising with the plan epoch, so this issue needs no wire surface of its own.
