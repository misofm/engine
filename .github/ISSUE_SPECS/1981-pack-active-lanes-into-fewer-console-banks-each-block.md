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
- Correction (attempt 2): attempt 1's records moved to `artifacts/steps/b14-1981/attempt1/`. Its line above on "the other console rows" was wrong: it read only the 64-track console family. The verifier found idle +61.5 %, dispatch_only +18.8 %, gain_pan_live +15 %, gain_pan_ring +13 %, nine_track_baseline +6.5 % and builtins_only +6.4 % in the same records. Verdict: FAIL (cost and record).

## Decision record (attempt 2, 2026-10-25)

- Merged `origin/main` at `d47d78c88` (#1371's per-lane link cell); gates 1 and 3 re-run on the merged tree, green.
- **Facts after the run (verifier MAJOR 1).** The fact is read after `chain.run`. If the chain skipped (its `silent_skips` rose), every populated lane is quiet with no scan. That is exact: a skip proved every active input `+0.0` and every lane at rest. Otherwise one `lanes_at_rest` query, and the head test only on the lanes at rest.
- **No retry of a stable refusal (verifier MAJOR 2).** `PackGroup::dirty` is set when an idle count crosses `PACK_IDLE_BLOCKS` either way, by every move (`LaneMap::record_swap`), at bind, and when a step spends its whole budget; `pack_lanes` enters `pack_group` only while it is set. New test `a_refused_pack_is_not_retried`: the mono bank runs with packable lanes, `ch17` runs in the padded bank, no ride; 7 attempts by block 60 and none after, no move. Red on the mutant that ignores the bit (1288 attempts). Test value: red when the packer retries a refused move every block although nothing it decides on changed.
- **Packing gate.** It now seeks: the render clock jumps 10 blocks at block 545, with strips displaced (asserted). It gives `ch09` and `ch17` a `maximum` compressor link (#1369); both are among the moved strips (checked once with a print, not committed). Bits equal, 38 moves, 474 skips against 335, displaced 0 at the end.
- **Unverified:** a bound meter on a packed strip. This level binds observers (every strip's post-matrix words, compared), not meter streams.
- The test value reads "a pack that skips no more than the unpacked twin" in place of "packs a lane not at rest", and gate_one's doc says the Processed twin packs too. Re-run red: the rest-ignoring mutant (skips 335 against 335).

## Evidence (attempt 2): gate 3 FAILS

- **Native.** Candidate `2cf72c64f`; twin of main at the merge base `d47d78c88`; records moved to `artifacts/steps/b14-1981/attempt2/` in attempt 3. One invocation each under one exclusive lock, one warmup and two rounds, uncontrolled (loadavg 6.8 and 6.7 at the starts). Validator PASS on both; every one of the 78 output digests equal.
- **Native rows above +2 %**, p50 change, rounds 1/2:
  - **all-active +3.1 %/+6.2 % (gate fails)**
  - dispatch_only +21.0/+19.7
  - nine, ten and thirteen-track ragged strips +16.8/+17.4, +14.9/+16.4, +14.7/+16.2
  - gain_pan_live +15.2/+13.2, gain_pan_ring +15.1/+13.5
  - idle +11.6/+2.1, nine_track_baseline +6.3/+5.4
  - eq_comp_simd1 +5.0/+4.8, eq_only +5.3/+3.6, legacy +4.5/+4.5, mono +4.6/+3.4, compressor_only +3.6/+4.3, app_shape +3.6/+4.0
  - toggle +3.1/+3.8, one_twenty_eight_track_stretch +2.8/+3.7, half_mono +2.5/+3.1, mono_dual +3.0/+3.1, metered +2.8/+3.0
  - builtins_only +3.4/+1.9, sends +1.6/+2.3
  - sixteen_track_strip -16.4/+3.2 (the twin's round 1 is an outlier)
- **Native wins:** sparse -38.3/-38.3, one_per_bank -73.3/-74.5.
- **Noise:** every plan with a pack group runs the facts, so no row is unaffected. The twin against itself: its two rounds agree within about 1 % on every row except sixteen_track_strip.
- **V8.** Candidate `0d37b7471`, twin at `d47d78c88`; loadavg 4.7. Every document digest and the quiet/automated digests equal.
- **V8 change**, p50, rounds 1/2:
  - console +14.7/+3.3 (236.7/238.0 to 271.4/245.8 us)
  - app_shape +18.8/+3.7, toggle +11.7/+3.6, sends +6.3/+0.5
  - the quiet, restated and automated arms +3.2 to +4.1 in both rounds
  - sparse -34/-39, one-per-bank -58/-62
  - The candidate's round 1 is noisy (console p95 289 against 243 us); round 2 shows about +3.5 % on every running row.
- **Diagnosis** (untimed against the record, not committed; one launch per variant of the bench binary on cpus 30-31 under the exclusive lock), p50 us for main / candidate / candidate with no facts block / and also no `pack_lanes`:
  - dispatch_only 8.84 / 11.14 / 9.00 / 9.21
  - nine_track_ragged_strip 24.80 / 28.95 / 24.80 / 24.68
  - gain_pan_ring 9.06 / 11.03 / 8.76 / 8.83
  - console 97.98 / 102.50 / 98.76 / 98.66
- **Reading:** the regression is the per-block fact on a running grouped unit (the rest query over every slot plus the head tests through `gather_planes`, a few hundred ns per unit). The packer itself costs nothing measurable.
- Diagnosis follow-up: the attempt-2 records are in `artifacts/steps/b14-1981/attempt2/`.

## Decision record (attempt 3, 2026-10-25)

- Root ruling: `crates/rack` is authorized. The exact per-lane silence fact comes from the chain's own gather; no sampled or rotated facts.
- `tile_gather`, `AoSoaScratch::gather_lane` and `gather_lane_left` return, per lane, the OR of the bits of every word they moved: one OR per stored row, a side result. The stored words are unchanged (class A). `BankChain::gather` and `gather_mono` (tiled and scalar) fold the tiles and the ragged tail into `BankChain::input_silent_lanes`: the active lanes whose every input word was `+0.0`.
- A skip reports every active lane. A resident acquisition gathers nothing and reports none. A collapsed gather reads the left plane alone, so its mask is the left plane's (the collapse is armed only where both channels read one source channel).
- The graph reads the mask after the run and asks `lanes_at_rest` only when some lane's input was silent; a skipped block still asks nothing. The packing gate, `a_refused_pack_is_not_retried` and the allocation gate are unchanged and green.
- New rack test `the_gather_reports_exactly_the_silent_lanes`. The setup: 19 frames (16 tiled and a tail of 3), both the tiled and the scalar gather, one non-zero word in a tile of a right plane, one in the tail of a left plane, a `-0.0` lane, an inactive silent lane, and a second block that clears a lane. Test value: red when a lane with one non-zero word is reported silent, or a stale mask survives a block. Red first: the tail OR dropped (37), the right plane's OR dropped (35), the mask accumulated across blocks (block 2).

## Evidence (attempt 3)

- **Native.** Candidate `5ed8a1da8`, twin at the merge base `d47d78c88` (`main/`). One invocation each under one exclusive lock, one warmup and two rounds, uncontrolled (loadavg 3.9 and 3.7 at the starts). Validator PASS on both; all 78 digests equal.
- **Native all-active +0.56 %/+0.52 %** (98.04/98.38 to 98.59/98.89): inside 2 %.
- **Native rows above +2 %**, rounds 1/2:
  - dispatch_only +7.5/+3.7 (8.97/8.97 to 9.64/9.30 us)
  - idle +6.0/0.0 (5.90/6.01 to 6.25/6.01)
  - nine_track_baseline +3.5/+2.2
  - gain_pan_live -0.6/+3.4, builtins_only +2.3/+2.8
  - nine_track_ragged_strip +1.4/+2.6, mono +2.1/+1.1
  - Every other row is within +1.4 %.
- **Native wins:** sparse -39.8/-41.1, one_per_bank -74.6/-74.3.
- **Noise** from the twin's own two rounds: idle 1.9 %, eq_comp_simd1 1.4 %, gain_pan_live 2.2 %, the rest under 1 %. So idle, gain_pan_live and the ragged/nine-track rows sit at about the noise. dispatch_only (+0.3 to +0.7 us) and builtins_only are a small residual: the gather OR runs on every bank, grouped or not.
- **V8.** Same protocol (loadavg 3.4 at the twin's start, 6.8 at the candidate's); every document digest and the quiet/automated digests equal.
- **V8 all-active +3.2 %/+3.0 % (237.7/237.7 to 245.2/244.8 us): misses 2 %.**
- **V8 other rows**, rounds 1/2:
  - app_shape +4.3/+3.7, sends +5.6/+5.7, toggle +4.2/+5.6
  - the quiet, restated and automated arms +2.0 to +3.0
  - sparse -39.1/-39.0, one-per-bank -61.3/-61.6
  - The twin's own rounds agree within 0.3 %.
- **Owner question (spec condition):** native holds all-active within 2 % and V8 does not (+3 %). Accept the measured V8 cost for the sparse and one-per-bank wins (-39 % and -61 % in V8, -40 % and -74 % native), or shelve packing.

## Decision record (owner decision, 2026-10-26)

- **Owner decision (2026-10-26), verbatim: "1981 - accept."** This answers the owner condition and the attempt-3 owner question: the owner accepts the measured V8 all-active cost (+3.2 %/+3.0 %) for the sparse and one-per-bank wins. Native holds all-active within 2 %. The gate text is unchanged; this ruling is the record. No new timing run: the attempt-3 measurement stands.

## Evidence (merge of origin/main, 2026-10-26)

- Merged `origin/main` at `80532ee4a` with a merge commit (no rebase). No file conflicted. Every line the branch changes in `crates/graph/src/{lib,runtime}.rs`, `crates/rack/src/lib.rs`, `crates/graph-compiler/tests/silent_bank_skip.rs` and `tools/console-workload/src/lib.rs` is the same in the merged tree, so the packing code did not change in the merge.
- Gates on the merged tree, all green: `cargo fmt --all -- --check`; `cargo test --locked -p rack -p graph -p graph-compiler -p console-workload --features graph/test-support` (gates 1 and 2 in `silent_bank_skip.rs`: `a_packing_plan_renders_the_bits_of_an_unpacked_one`, `a_skipping_block_allocates_nothing`, `a_refused_pack_is_not_retried`, and the other two); `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `scripts/check-workspace-policy.sh`, `check-graph-policy.sh`, `check-rack-policy.sh`, `check-realtime-audit-leak.sh`; `realtime-policy` (233 marked regions in 43 files).
- Bit identity: gate 1 (the committed packed-against-unpacked bit test) is green on the merged tree. The 78-digest comparison against a main twin is a benchmark-run check and was not run again.
