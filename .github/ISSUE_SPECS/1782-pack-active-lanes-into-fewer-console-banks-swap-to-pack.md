# Pack active lanes into fewer console banks (swap-to-pack)

Split from #1107 (D5 item 3, 2026-10-23). The design sketch is #1107's D5 item 3; constraints are #1107's D4 items 1-6.

## Problem

#1107 skips a console bank only when every lane of it is silent and at rest. A session where silence is spread across banks (the sparse row: odd tracks silent) skips nothing: every bank keeps one active lane. Packing the active lanes into as few banks as they fill lets the rest skip.

## Split (root, 2026-10-24)

This issue is split in two (scope note R1): **#1782a**, here, is the lane map, the move and the fold on a displaced group; **#1981** (#1782b) is the per-block packing decision (`R`, `B`, the packable test, the differential with packing on and off, and the native and V8 console runs). Nothing in production moves a lane until #1981.

## Smallest closable slice (whole issue; #1782a's part is recorded below)

- Pack groups: maximal runs of consecutive peer Bank units (equal slot program, equal collapse arming), with direct gather sources only, no resident pairing, and all or none folded into one master.
- A graph-level lane map (`home[strip] -> (unit, lane)`, inverse, per-strip `automated` bit). A move exchanges the two chains' lanes and, in the same call, the member ops of every slot, `fold[lane]` and the `source_lanes` bits; response-snapshot bindings, the predecessor side of a plan carry and `unit_eligibility` translate through the map.
- Folded groups: the epilogue fold is deferred to one group-level fold after the group's last unit, in home order, so the master's summation order is exactly the order `route_fold` proves; skipped strips use #1107's signed-zero rule. Packing must never move a master bit.
- Per-block packing decision after the drains, with a hysteresis of `R` idle blocks and a budget of `B = W` moves per group per block (preallocated constants, recorded with their reasons); automated strips move only into fader banks with automation slots (a bind-time bit).

## Authorized paths

To be confirmed in this issue's own design step before code: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/graph-compiler/`, `crates/rack/` (tests), the console benchmark files. This spec.

## Objective gates

1. Class A differential on compiled plans with random per-lane silence runs, packing on and off: every strip and the master equal by `to_bits`; Simd8 and Simd4 (the arm64 CI legs).
2. One plan-level swap test (#1107 D4-5).
3. Allocation-free.
4. One native and one V8 console run after #1783's rows exist: the sparse row against S4; the all-active row within 2 %; the toggle row's cost recorded.

## Test value

Gate 1 turns red when a pack moves a strip's or the master's bits; gate 2 when a plan-level swap leaves a lane-keyed reader behind.

## Dependencies

#1107 and #1781 merged; #1783's rows before gate 4's timing.

## #1782a decision record (2026-10-24)

- **R4, bind:** pack groups are found at bind in `graph/src/runtime.rs` (`LaneMap::discover`, after `input_producer_table`): maximal runs of two or more adjacent bank units, each gathering every lane directly (`bank_gather_source`), with no resident pairing on either side, no `input_producers` row and in none, no auxiliary lane, a trailing active mask, at most eight lanes, and all lanes folded or none; two units are peers when the widths and slot counts match, both fold into one master or neither folds, and some lane pair passes every slot's `BankStage::can_swap_lanes` (one slot program). graph-compiler is untouched but for the test.
- **Collapse arming** is not a group term: it is joined after bind (`arm_mono_collapse`), and `BankChain::swap_lanes` refuses a move between differently armed chains. So does a fader bank with no automation slot for an automated strip (`automation_swap_fits`).
- **The automation-slot bit and the per-strip `automated` bit are not built here.** Graph sees a fader bank only through `GraphPreparedBuiltinBankProcessor`, which does not expose its slots; the bit needs a trait method there and its `FaderBankProcessor` impl in `builtins-compiler` (outside #1782a's paths). The move is safe without it (the refusal above); #1981, its only consumer, adds it.
- **The move** (`Runtime::swap_bank_lanes`, a test-support hook `graph::test_only_swap_bank_lanes`): the chains' swap first (every refusal before any write), then the member op of every slot, the `FoldLane` entry (constants and store bit), the `source_lanes` bit, the two units' observation rows, and the map. `fold[lane]` follows the strip, so the group fold reads it through the map.
- **Readers keyed by (unit, lane) translate:** response-snapshot bindings (`current_member`), `unit_eligibility` rows (`lane_eligibility_of`, rows keep their home lanes), fade arms (`current_location`). The **plan carry** takes both plans back to the identity first (`restore_home_lanes`, at most one swap per strip, none under the identity), because its move tables group a bank's lanes by home location; a move back that refused would carry nothing (`PredecessorMismatch`).
- **R2, the fold:** a displaced folded group's units defer every epilogue (`ArenaMembers::defer`), and after its last unit `fold_displaced_group` folds each home unit in schedule order: its home tile re-materialised from the holders' resident blocks (a unit none of whose strips moved uses its own in place), through `fold_resident` and, declined or padded, the staged `fold_cohort` -- the kernels the unit's own epilogue uses. The identity map keeps the fused epilogue; a plan with no group pays one bounds-checked load per bank unit. `block_end_units` widens to every unit of a group that holds a `block_end` observer. No rack change.

## #1782a evidence

- Gate 3 (`a_moved_strip_renders_the_bits_of_an_unmoved_one`, `crates/graph-compiler/tests/silent_bank_skip.rs`): two carried pairs of one 20-track session, one under eleven forced moves (store lane, cross-bank, padded bank, during skips, before and after the swap), every host word, post-matrix word and response snapshot equal by `to_bits`. Gate 2 extended: the moves and every displaced block allocate nothing.
- Red first (mutants, each restored): group fold ignoring the map, members or fold entries not exchanged (across units, and within one), snapshot binding untranslated, the carry not taking the predecessor home, no deferral: gate 3 red; an allocation in the move or the group fold: gate 2 red. Green, and why: a deferred epilogue that still folds, and a skipped deferred unit's silent fold, both write before the group fold's store overwrites the master, which every session in the file holds in the group; `source_lanes`, observation rows, eligibility rows and fade arms are not reached by these sessions (no source set, every unit observed, equal witnesses, no fade arm).
