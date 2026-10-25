# Pack active lanes into fewer console banks each block

Filed by root as #1782's second slice (backlog sprint, 2026-10-24). #1782a builds the lane map, the move and the fold on a displaced group; this issue is the per-block packing decision that uses them.

## Smallest closable slice

- After the drains, per group: a lane is packable when its input is exact `+0.0` on both planes
  (the per-lane `block_is_positive_zero` head) and `lanes_at_rest` covers it and it has idled `R`
  blocks. With `A` active lanes run the first `K = ceil(A/W)` units; an active lane in a unit
  `>= K` is swapped with a packable lane in a unit `< K`, at most `B = W` moves per group per
  block; a unit that keeps an active lane simply runs (always exact). Automated strips move only
  into units with the automation-slot bit. `R` and `B` are preallocated constants with reasons;
  recommend `R = 16` blocks (43 ms at 48 k: above any smoother ramp, below every effect rest
  bound, so it only debounces chains whose rest is immediate) and `B = W` (one bank's state per
  block is the worst swap cost; D5).
- Gates: (1) class A differential on compiled plans, packing on and off, random per-lane silence
  runs including whole-bank windows, a fader ride, stored automation on some strips, a collapsed
  mono-armed bank (R7), two route-sign layouts (D10): every strip and the master equal by
  `to_bits`, NaNs folded; Simd8 here, Simd4 on the arm64 CI legs at the batch push (D6). Test
  value: red when a pack moves a strip's or the master's bits, or packs a lane not at rest. Fold
  it into `silent_bank_skip.rs` (one claim, one owner crate). (2) allocation-free with packing
  active. (3) one native and one V8 console run (#1783's rows): the sparse row and
  `one_per_bank` against S4/opt6-1107-fix; all-active within 2 % of opt6-1107-fix (the re-based
  reference, D10 F1); the toggle row's cost recorded.

## Order

After #1782a. Reads #1783's toggle and one-active-lane-per-bank console rows.

## Owner condition

If all-active cannot hold within 2 % of opt6-1107-fix under #1782a's re-materialised home-tile fold, stop and ask the owner whether to accept the cost or shelve packing. A data-dependent fold order is never an option (D6).

## Paths

`crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/graph-compiler/tests/silent_bank_skip.rs`, `artifacts/steps/b14-1782/`, this spec.

## Decision record (2026-10-25, attempt 1)

Root accepted the scope note's R1-R6 (`scratchpad/opt/scope/1981.md`); where it differs from the slice above, it wins:

- **R1, placement.** `Runtime::pack_lanes` runs once per block in `GraphExecutor::render`, after the fade arms and the route carry and before the unit loop, never between units (`observe_unit` interleaves with `execute`: a move there observes one strip twice and its partner never; the mutant that packs after each unit is red at block 51). It reads the previous block's facts: a grouped unit's `execute` records which lanes' inputs were exact `+0.0` (`BankMembers::input_is_positive_zero`, the skip's own head test, up to eight per grouped unit; ungrouped units pay one load) and, only when some lane was, the chain's `lanes_at_rest` after the run. Both fold into a home-indexed `u16` idle count per strip (`LaneMap::idle`) and a per-group packable count. A fact a block old is always exact: each chain decides its skip from the block's own input and rest, so a strip packed on a stale fact costs its bank one rendered block, never a bit.
- **R2, refusals.** The automation-slot and collapse-arming refusals are attempt-and-refuse through `swap_bank_lanes` (every check before any write); no trait method, no builtins-compiler change.
- **R3, homing.** With the budget the pack step leaves, a displaced strip trades back home with the strip there when both are of one class (both running or both packable), else sends that occupant home under the same rule (the way home of `restore_home_lanes`). Same-class trades leave every unit's set of classes, and so which units can skip, unchanged, so homing never undoes a pack, and an all-active episode returns the group to `displaced == 0` and the fused epilogue.
- **R4.** `R = PACK_IDLE_BLOCKS = 16` (reason in its doc), `B = W` moves and at most `span` attempts per group per block.
- **R5.** Production always packs; `graph::test_only_disable_lane_packing` is the test-support off switch, with `test_only_bank_lane_moves` and `test_only_displaced_strips`.
- **R6.** Records under `artifacts/steps/b14-1981/`, the main twin's under `artifacts/steps/b14-1981/main/`.
- **The pack rule (implementer's change to the slice's `K = ceil(A/W)`).** A running strip in a unit above the first moves only into a *lower unit that already runs* (lowest first), and a padded lane is a fixed filler, not an active one (it never stops its bank's skip). The `K` rule targets the first `K` units whatever they are: in the gate's session the first bank is the collapse-armed mono bank, which refuses every move from the stereo banks, so the `K` rule packed almost nothing there, and a move into a unit that does not run only trades one rendered bank for another. The running-unit rule ends with every running unit but the highest full, which is `ceil(running/W)` units up to the refusals and padded lanes. When the highest running strip finds no candidate, no lower one can (its candidates are a subset), so the step stops: on the all-active steady state it is one scan of the span per group, and a group with nothing packable and nothing displaced is passed at one load.
- **Paths.** One file outside the list: `tools/console-workload/src/lib.rs`, whose feed test (`every_row_is_fed_through_a_source_set_and_gathered_in_place`) counts one gather per track per block; packed sparse rows now skip banks, which gather nothing, so that test renders with packing off (its claim is the feed). The gate-1 extension is a second `#[test]` on `gate_one`'s harness (`Twin::Unpacked`), and #1782a's gate 3 now runs with packing off so its forced moves stay the only ones.

## Evidence (attempt 1)

- Gate 1, `a_packing_plan_renders_the_bits_of_an_unpacked_one` (`crates/graph-compiler/tests/silent_bank_skip.rs`), Simd8, both layouts: gate 1's inputs plus a 40-block all-active tail, the second bank given the 0 dB compressor threshold so its strips rest; every host and observed word equal by `to_bits`; lane moves 38 (twin 0), skips 478 against 335, displaced 0 after the tail. Simd4 runs on the arm64 legs at the batch push.
- Gate 2, `a_skipping_block_allocates_nothing`, now on a carried pair: forced moves at blocks 10 and 100, the packer from block 101, `ch17` playing again in 200..280 so it is packed into the second bank, and the carry at block 300 from the still-displaced predecessor (the adopt and the successor's first block measured): zero allocations, reallocations and frees.
- Red first (each mutant restored with `git checkout` and `touch`): packing after each unit instead of before the loop (gate 1, host word at block 51); no packing (gate 1, `lane_moves > 0`); no homing (gate 1, 10 strips displaced after the tail); the idle count ignoring rest (gate 1, skips 335 against 335); an allocation in `pack_group` and one in `record_facts` (gate 2, the audited allocator aborts).
- Gate 3, native (`artifacts/steps/b14-1981/`, 78 records, validator PASS, Simd8, candidate `4637ba810`; main twin built at the base `889da4db1` in `artifacts/steps/b14-1981/main/`), one invocation each under the exclusive lock, one warmup and two rounds, uncontrolled (loadavg 4.5/3.8 at the starts), p50 us/block, rounds 1/2, candidate against twin: all-active 99.71/99.26 against 98.43/98.49 (+1.3 %/+0.8 %, inside 2 %); sparse 62.16/61.47 against 99.91/100.35; one-per-bank 28.41/28.50 against 99.11/99.39; toggle 100.71/100.54 against 98.69/98.49 (+2.1 %: its lanes never rest, so this is the facts' and the scan's cost alone). The other console rows read +0.7 % to +3.1 % (legacy 61.22/60.80 against 59.57/59.68, mono 62.46/61.66 against 60.61/60.59, mono_dual +2.4 %/+1.5 %, metered +2.1 %/+1.6 %); no gate holds them, recorded. Every one of the 78 output digests equals the twin's.
- Gate 3, V8 (`web-mixing-automation.jsonl`, validator PASS, twin under `main/`), p50 us/block, rounds 1/2, candidate against twin: console 241.7/238.7 against 254.8/238.6; sparse 149.0/158.1 against 258.9/244.2; one-per-bank 97.5/107.4 against 257.9/243.8; toggle 244.5/243.5 against 257.4/240.9; every document digest and the quiet/automated digests equal.
- The carry from a displaced predecessor (verifier ask; a one-off release measurement on gate 2's carried pair, not committed, cpu 31, one warmup and two rounds): `adopt_predecessor_plan` 405/411 us with two strips displaced against 369/368 us from the identity, adopt plus the successor's first block 470/476 us against 442/433 us (the walk home is the difference).
