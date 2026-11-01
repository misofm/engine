# Cost-weighted mono/stereo remainders for console cohorts

Draft slice 7 (#2137) of the banking design. Amends #1098 amendment 10 (the retired #971 demotion for
console slots) with a stated objective. Class A: a mono-symmetric strip rendered dual-mono emits
the same words on both planes as its collapsed render (the collapse's own premise).

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

Mono and stereo strips never share a bank, so a mixed session pads two remainders per key class
and window (audit section 2, "Mixed mono/stereo"): at W=8 with 10 mono and 6 stereo tracks, three
banks instead of two. The planner never weighs it for console strips (`banks.rs:479-504`).

## Smallest closable slice

- After `plan_bank_groups` (levels today; `placed` windows after #2136), per `(rack, program, level)`: if the mono pool's last group is
  partial with `m` members and the stereo pool's last group is partial with `s` members and
  `m + s <= W`, the mono members join the stereo group (as stereo-class lanes) and the mono
  group is dropped. Objective: planes x banks; a merged stereo bank costs 1.0 against 0.6 + 1.0.
  The rule never touches a full group and never moves a strip whose builtin stage would then
  pool differently: the class map is updated for the moved strips before the builtin planner
  runs (the two-planner obligation, `CohortPoolClass::of_prepare_witness`'s doc), as #971 did.
- Insert cohorts keep #971's rule as it stands.

## Gates (one test per claim, red first)

1. `crates/rack-compiler` unit test: 10 mono + 6 stereo at W=8 forms two banks (8 mono, 2 mono +
   6 stereo); 8 + 8 forms two homogeneous banks; 2 mono + 14 stereo forms two banks. Red on a
   merge of a full mono group. Test value: red when the planner pads two remainders that fit one
   bank.
2. `crates/graph-compiler` half-mono console fixture: the builtin planner and the rack planner
   agree on every moved strip's class (the existing two-planner agreement test, one new row).
   Red on a class map updated after the builtin planner ran.
3. Bit identity on the half-mono fixture before and after (PR evidence).

## Paths

`crates/rack-compiler/src/lib.rs`, `crates/graph-compiler/src/banks.rs` (`stranded_mono_tracks`
rewritten as the remainder rule), its tests, this spec.

## Dependencies

None in substance (#2131 is merged; the rule runs on levels or windows alike). **Deferred.**


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

- **Deferred; recommend closing as not worth it now.** The rule fires only when the mono pool and
  the stereo pool both leave a partial remainder in one `(rack, program, level)`. No fixture shows
  it: C1 is all-stereo (console stems), `sixty_four_track_console_half_mono` is 32 + 32 (no
  remainder at W=8 or W=4), the irregular row's 8 mono tracks of 24 fill one W=8 bank exactly. The
  saving is 0.6 of a bank's slots per occurrence. Reopening condition: a measured session shape
  with two partial remainders in one class; then this slice runs after #2136 with the amendment
  above and its three gates as written.


## Amendment (attempt 1, 2026-11-01; base main `7d89eecc5`)

Reopened by root with a plan (`issue-2137-plan.md`). The reopening condition holds: a mixed-source
session with two partial remainders per console slot is a production shape under the dual-mono-by-
source-file ruling, and S4 adds the measured row.

### Root rulings

- **Q1, the weight:** 0.61. Measured on the 64-track mono row-pair of
  `artifacts/steps/b13-2133/main/console-benchmark.accepted.jsonl` (p50 61.5 us collapsed against
  100.2 us dual). The code names that record (`banks::MONO_BANK_WEIGHT`).
- **Q2, builtin stages:** the rack-chain compare is the proxy. The builtin remainders follow the
  per-track class; the agreement test checks the class.
- **Q3, pack groups (#2135):** the moved strip joins the stereo pack group, with no change there.
- **Q4, order:** S4 first, then S1 to S3, on one branch.

### Objective (stated)

The cost of a plan's changed groups is the sum over their bound effect banks of `w(class)`, with
`w(mono) = 61` and `w(stereo) = 100` (integer hundredths, exact compare). A move set is kept when
the re-plan's changed groups cost less than the trial's **and** bank no fewer slot members (no
banked effect node falls to the per-node path, which the weight does not price). One decision for
the whole move set. Candidates: the console tracks every console group of which is a partial mono
group that fits a partial stereo group of its `(level, rack)` (`m + s <= W`, each stereo group taken
once). Then #971's stranded set runs on the result with its bind-count objective. The class map is
updated before the builtin planner reads it.

### Slices

- **S4** `sixty_four_track_console_mixed` (37 mono, then 27 stereo; remainders 5 + 3 at W=8, 1 + 3
  at W=4), emitted last (`MIXED_CLASS_WORKLOADS`), the half-mono row's floor. Validators: 82
  records, 54 session records. No timed run: root schedules one at base and head.
- **S1** `BankPlan::class_remainder_moves(width)` (rack-compiler, pure).
- **S2** `try_pool_move(candidates, objective)` in `banks.rs`, run for the console remainder set
  (class-weighted) and then #971's stranded set (bind count).
- **S3** `mergeable_partial_pairs` counts a mono + stereo pair in a console rack.

### Tests and their red claims

- `a_fitting_mono_remainder_names_its_members` (rack-compiler): red when the fit is ignored (13 + 5
  names `[8..12]`). A full group never fits a non-empty remainder, so the fit also keeps full groups.
- `the_console_remainder_rule_merges_a_fitting_pair` (graph-compiler, the 63 + 1 test rewritten):
  red when the move is removed (27 banks for 24 at W=8), when the remainder objective is bind count,
  or when the builtin planner reads the pre-move map. Bits equal the Scalar oracle.
- `the_remainder_rule_never_trades_banked_work_for_per_node_work` (new): red when the weighted
  objective compares bank cost alone.
- `a_console_track_is_never_pooled_as_stereo_to_fill_a_bank` (doc updated): red when the remainder
  objective is bind count.
- `the_two_planners_agree_on_every_track_class`: one new row, 10 + 6; red when the builtin planner
  reads the class map from before the move.
- `the_bank_report_counts_mergeable_pairs` (the "another pool class" assertion inverted for a
  console rack, kept for the insert rack): red without the S3 clause, red when the clause reaches
  the insert rack.
- Shape edits, claims kept: `a_mono_track_whose_witness_declines_pools_as_stereo` (now 1 + W tracks
  so no remainder fits) and `a_later_mono_chain_arms_on_its_producers_and_renders_the_dual_bits`
  (stereo on the third and fourth of every four tracks, so every pool is whole banks).

### Not tested

The track filter "every console group fits" (a track whose `pre_insert` remainder fits while its
`post_insert` group is full) has no test: the shape needs differing post-insert levels, and the
objective weighs such a move anyway.

### Evidence (attempt 1)

Bank counts are what the planner reports (`BankUtilisation::banks`: bound plan groups, both
console racks). The W=8 counts come from a native render. The W=4 counts come from the `simd128`
guest under wasmtime (`bank_counters`, 256 blocks).

| shape | W=8 base | W=8 head | W=4 base | W=4 head |
| --- | --- | --- | --- | --- |
| 10 + 6 | 6 | 4 | 10 | 8 |
| 37 + 27 (`sixty_four_track_console_mixed`) | 18 | 16 | 34 | 32 |

The W=4 C1 rows (`W4_CONSOLE_COUNTERS`) did not change.

**No bit moved (PR evidence; the harness is in scratch and is not committed).** Base is S4
(`263c27651`), head is S3 plus the clippy fix (`f64ab9203`). Each case renders 300 blocks, plain
(`PlanConfig::BASELINE`) and live (the product's live lanes), at W=8. The harness hashes the
session output with SHA-256 and every track's `PostMatrix` words (bound observers, FNV-64). The
cases are the 10 + 6 and 37 + 27 shapes, the half-mono row and every `fixtures/session/v1`
session. Across 1075 per-strip digests and every output digest, base and head are identical. Base
and head refuse the same cases: `canonical` (an effect not available at launch),
`json-invalid-duplicate-key` (invalid by design), and the live arm of the submix sessions (C1,
sends, irregular graph, builtins-automation), whose route lanes the harness does not attach. The
moved strips render dual in a stereo bank and emit the words of their collapsed render.

**Checks:** `cargo fmt --all --check`; workspace clippy with CI's flags; release tests of
rack-compiler, graph-compiler (`--all-features`), console-workload and bench, and of host-core,
capi, graph, builtins-compiler and wasm-gates; `check-workspace-policy.sh`; `realtime_policy`;
`check-bench-policy.sh`, `test-bench-policy.sh` and `test-console-benchmark.sh`; the console
benchmark preflight (no workload launched); `build-web-audioworklet.sh` and
`check-web-audioworklet.sh`; browser qualification `--check-matrix` on Chromium and WebKit, single
and worker legs: all pass. No timed benchmark was run, because the machine was not quiet.
