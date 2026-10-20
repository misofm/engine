# Read every graph-group test once and cut the defended ones (test audit, read pass, group graph)

Owner decision, 2026-10-20, relayed by root: read every remaining test once, crate by crate, and
delete each test that a named remaining test already defends. The rule is the AGENTS.md "Test
value" question: *which plausible defect turns this red that no remaining test catches?* This spec
is the graph group: the crates `graph`, `graph-compiler`, `builtins-compiler` and
`effect-compiler`. Branch `codex/batch-test-read-graph`, from `main` at `64fe78c46`. Number: TBD.

## Method

- Four readers (r4a to r4d) each read a declared file list and gave every test one verdict:
  KEEP, DELETE or MERGE. A DELETE names, by full path, a remaining test that asserts the same
  claim, and the reader read that defender. "Siblings cover it", "the randomized test covers it"
  or a file name alone is not an answer. A MERGE turns near-duplicates into one table-driven test,
  one row per old test, and keeps every claim.
- Never deleted: realtime gates, ABI and wire-format gates, corpus and fuzz gates, the sole red
  test of a `MUTATIONS.md` row, tests in a known-defect list, seeded or randomized differentials,
  allocation, count and ceiling claims, regression reproducers, acked-batch tests, and tests that
  a workflow or script runs by name.
- The integrator (i4) checked the defenders across all four readers, built, cleaned helpers
  (the compiler reported none unused), fixed references, ran a mutation spot-check and the gates.

## Per-crate counts

| crate | tests read | kept | deleted (lines) | merged (old lines -> new test, lines) | net diff |
|---|---|---|---|---|---|
| `graph` | 136 | 132 | 3 (311) | 1 (10) into an existing test; 1 test tail trimmed (25) | +8 / -353 |
| `graph-compiler` | 153 | 146 | 0 | 7 (2,187) -> 1 new test (880 with row helpers) | +2,405 / -3,718 |
| `effect-compiler` | 39 | 35 | 1 (21) | 3 (92) -> 1 new test (86) | +70 / -101 |
| `builtins-compiler` | 73 | 73 | 0 | 0 | 0 |

## What changed, by test

`graph`:
- DELETE `runtime::tests::all_active_folded_bank_chain_dispatches_the_real_graph_cohort`.
  Defender: `runtime::tests::a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit`
  (its staged arm panics in `fold_plane`, counts `[cohorts, 0, cohorts]` and matches the fused
  arm bit for bit, at both widths).
- DELETE `tests::executor_applies_exact_pdc_then_fixed_pairwise_reduction`. Defenders:
  `tests::enabled_and_bypass_pdc_align_at_launch_rates_and_quanta` renders an inserted
  compensation delay on a `RouteDestination` edge into a two-route output sum, exact sample, both
  channels, at every launch rate and five quanta; a wrong delay or a dropped input splits or
  halves the sample it pins. `runtime::tests::reduction_is_left_to_right_bit_identical_to_scalar_reference`
  pins the reduction order. The integrator read both and checked the delay with targeted mutants
  (below).
- DELETE `program::tests::fan_out_blocks_in_place_and_fan_in_keeps_its_reduction`. Defender:
  `program::tests::lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs`. Its
  generator builds tracks with one or two routes (the deleted test's exact shape, `plain_track`
  with two routes, occurs in about 12 of its 300 graphs). Its symbolic interpreter compares every
  op's value with the naive graph evaluation, and `Expr::Sum` keeps operand order, so a route
  written in place over a two-reader buffer corrupts the other route's input, and a lost or
  reordered reduction changes the output expression. Two targeted mutants of the in-place reader
  check (below) are red in it.
- MERGE `runtime::tests::a_single_in_place_input_is_left_untouched` into
  `runtime::tests::reduction_preserves_repeated_silence_self_and_unrelated_buffers` (the
  self-alias case was already there; the other-buffer assertion was added).
- Trim: the removed-VCA tail of
  `runtime::follow_curve_tests::a_moved_vca_mute_table_lands_over_its_own_length_at_the_first_sample`
  (25 lines) repeated the fixture and assertions of
  `runtime::follow_curve_tests::a_removed_follow_curve_lands_the_column_on_its_static_value`
  case 1.

`graph-compiler`: the seven launch fixture tests
(`launch_parametric_eq_fixture_retains_banks_and_matches_scalar_across_blocks`,
`launch_compressor_fixture_retains_bank_tail_and_connected_scalar_with_zero_pdc`,
`launch_gate_expander_fixture_retains_width_correct_banks_and_scalar_fallbacks`,
`launch_true_peak_limiter_fixture_retains_banks_tails_latency_and_transactional_caps`,
`launch_multiband_compressor_fixture_closes_bank_graph_and_transactional_caps`,
`launch_soft_clip_fixture_closes_banks_tails_pdc_support_and_transactional_caps`,
`launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps`) are now the rows
of `tests::launch_effect_fixtures_bank_render_and_bypass_as_frozen`. Each row keeps its bank
count and membership, scalar-tail order, prepared latency, tail and ports, the bank-free
reference arm (same schedule, route timings, inserted delays and PCM), its estimate pins, its
output check, its bypass graph and, where the old test had them, the canonical evidence, the
output latency and tail, the observer order and the one-byte-below plan cap. Some rows now check
more than their old test did, because the table runs a check for every row that sets it. The
merged test compiles and passes.

`effect-compiler` (`tests/native_session.rs`):
- DELETE `launch_registry_prepares_the_accepted_nine_track_parametric_eq_fixture`. Defenders:
  `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells` (exactly eight registry
  descriptors, each prepares), `a_session_bypass_lowers_to_an_enabled_effect_and_a_bypassed_lane`
  (names all eight IDs from the registry), and `src/prepare.rs`
  `metadata_mismatch_tests::a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused`
  (prepares the nine-track fixture through the EQ factory to nine entries).
- MERGE the three retired parameter ID tests (gate ID 8, compressor ID 8, multiband ID 2 with its
  positive control) into `retired_parameter_ids_reject_before_native_publication`.

`builtins-compiler`: no change. Reader r4b proposed a 40-line trim of
`tests::pair_factory_declines_wrong_order_policy_and_shape_with_original_owners` (its order,
policy and width legs repeat `late_factory_guards` and `declined_factory_owners`; its WrongOwner
leg is unique). That edit is **proposed, not made**: the permission system refused it, and it was
not made another way.

## Never-delete keeps, by class

From the readers' class column (one class per test, first match in this order: acked-batch,
realtime gate, allocation, wire/ABI, corpus, seeded or randomized, regression, `MUTATIONS.md`
row, count/ceiling/budget):

| class | graph | graph-compiler | builtins-compiler | effect-compiler |
|---|---|---|---|---|
| acked-batch question | 0 | 3 | 3 | 0 |
| realtime gate | 1 | 5 | 5 | 0 |
| allocation count | 5 | 8 | 5 | 0 |
| wire or ABI | 2 | 0 | 0 | 1 |
| corpus | 0 | 2 | 0 | 0 |
| seeded or randomized differential | 7 | 6 | 2 | 0 |
| regression reproducer | 4 | 29 | 1 | 1 |
| sole red test of a `MUTATIONS.md` row | 35 | 17 | 10 | 0 |
| count, ceiling or budget claim | 12 | 17 | 21 | 4 |
| no never-delete class (kept for a unique claim) | 66 | 59 | 26 | 29 |

## Defender cross-check

No DELETE defender is itself deleted or merged away in any reader's file. Every defender named
above exists in the tree at the branch head.

## References fixed

- `crates/graph/tests/MUTATIONS.md`: the #936/#957 gates 2 and 3 named their tests by old names;
  they now name `runtime::tests::an_observed_source_input_stays_dispatched_and_meters_the_full_dispatch_values`
  and `runtime::tests::a_delayed_claim_stays_dispatched_and_renders_the_full_dispatch_bits`.
- `crates/rack/tests/MUTATIONS.md`: the `bind_rack_banks` outcome note names the parametric-EQ row
  of `tests::launch_effect_fixtures_bank_render_and_bypass_as_frozen`.
- `.github/ISSUE_SPECS/1379-*.md`: the #1375 note names the true-peak-limiter row of that test.
- No `MUTATIONS.md` row named a deleted test. No script or workflow names one. Dated records
  (`docs/handoffs`, `docs/audits`) are not edited.

## Mutation spot-check

Seed `random.Random(20261020)`. Pool: the `cargo mutants --list --json` mutants (cargo-mutants
27.1.0) of `graph`, `graph-compiler` and `effect-compiler` whose function (joined by file and
function name to the older coverage map at `81a3074fc`) a deleted or merged test covered: 1,615
mutants. Draw: 15 from the mutants whose function at most 60 remaining tests cover, 15 from the
rest. Four targeted mutants were added for the two deletes that root asked to check: the
in-place reader test of `lower_with` (`program.rs:741`, `&&` to `||` and `==` to `!=`) and the
compensation delay read of `CompensationDelay::process` (`runtime.rs:2458` and `:2459`). Each
mutant was applied with `git apply` to a clean tree, the mutant's package and `graph-compiler`
were tested (fail-fast; targeted ones: the whole `graph` lib, no fail-fast), and the tree was
reverted (on exit and on SIGTERM, SIGINT and SIGHUP).

| result | random (30) | targeted (4) |
|---|---|---|
| caught (one by timeout: `walk_columns` `==` to `!=` hangs) | 22 | 4 |
| unviable (does not compile) | 6 | 0 |
| not caught | 2 | 0 |

- The two in-place mutants are red in
  `program::tests::lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs` (and in
  other `program::tests`), so the fan-out delete stands. The two delay mutants are red, so the
  executor PDC delete stands.
- Not caught, `runtime.rs` `ArenaMembers::distinct_planes_mut`, delete the four-lane arm: the
  caller (`rack` `scatter_tiled`) then takes the staged scatter, which writes the same bits; on
  this AVX2 host the launch banks are eight lanes wide. The deleted launch tests at `64fe78c46`
  are also green with this mutant. Not a gap of this cut.
- Not caught, `runtime.rs` `BorrowedPlanningMetadata::has_effect` returns `false`: its one caller,
  `plain_route_gains`, then reads `route(node)`, which is `None` for an effect node, so the
  result is the same (an equivalent mutant). The deleted launch tests at `64fe78c46` are also
  green with it. Not a gap of this cut.

No deleted test was restored: every viable mutant that the cut could expose is red in a remaining
test.

## Gaps

- No new gap from this cut. The two surviving mutants above are equivalent in output (one may be
  a performance-only change on four-lane targets: a dropped direct scatter falls back to the staged
  scatter). A successor may decide whether a counter test for the four-lane direct scatter is
  wanted on x86; this spec does not add it.
- Reader r4b's proposed trim of
  `tests::pair_factory_declines_wrong_order_policy_and_shape_with_original_owners` is left for a
  successor (proposed, not made).

## Gates

- Every `run:` step of the qualification `lint` job, in order, on this branch (the audit's
  `policy-job.sh`, pointed at this worktree): 31 steps, all PASS, including workspace clippy
  with `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc` and the lane-guard probes.
- `cargo test --locked --all-features -p graph -p graph-compiler -p effect-compiler
  -p builtins-compiler -p rack`: 458 passed, 0 failed.
- `cargo clippy --locked --all-targets --all-features` on the touched packages reported no unused
  helper, so no helper was deleted.
