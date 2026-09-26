# Red-mutation record for the #98 graph-executor gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing here is a claim about code
that was not run.

Host: `x86_64` (AVX2 + FMA, `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`),
debug profile except where a row says `--release`. Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p <package> --lib <test>
# and revert
```

| # | mutation | file | test | result |
|---|---|---|---|---|
| M1 | reduce in reverse edge order instead of the D9 stable order | `src/runtime.rs` `reduce_plane` | `reduction_is_left_to_right_bit_identical_to_scalar_reference` | RED |
| M1c | the same reversal, seen on the production 100-layout corpus | `src/runtime.rs` `reduce_plane` | graph-compiler `frozen_issue_037_seeded_builtin_bank_layouts_have_exact_membership_and_counters` | RED |
| M2 | fan-in 1 zero-fills then accumulates (the `fold(0.0, +)` shape, which turns `-0.0` into `+0.0`) | `src/runtime.rs` `reduce_plane` | `reduction_is_left_to_right_bit_identical_to_scalar_reference` | RED |
| M15 | fan-in 0 leaves the output buffer untouched instead of zeroing it | `src/runtime.rs` `reduce_plane` | `reduction_is_left_to_right_bit_identical_to_scalar_reference` | RED |
| M3 | the slice PDC keeps the block's entry cursor instead of advancing it | `src/runtime.rs` `CompensationDelay::process` | `compensation_delay_is_partition_invariant_and_matches_per_sample_reference` | RED |
| M12 | the slice PDC exchanges its two segments in the wrong order | `lane` `kernels::pdc_delay_block` | `compensation_delay_is_partition_invariant_and_matches_per_sample_reference` | RED |
| M4 | the route unfolds the gain and drops the fusion (`ll*l + lr*r`) | `lane` `kernels::mix2x2_block` | `route_applies_folded_gain_with_frozen_op_order` | RED |
| M9 | `mix2x2_block`'s scalar tail uses a different coefficient order than its vector body | `lane` `kernels::mix2x2_block` | `route_applies_folded_gain_with_frozen_op_order` | RED |
| M5 | the native executor stages an edge from the wrong producer | `src/runtime.rs` `build_native` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` | RED |
| M6 | the sequential executor reduces before staging its delayed edges | `src/runtime.rs` `execute_op` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` | RED |
| M8 | a bank member's inputs are gathered without their reduction | `src/runtime.rs` `Runtime::execute` | `level_major_w4_builtin_bank_is_analytic_for_three_blocks_in_both_executors` | RED |
| M7 | a tap's observers fire on the next op instead of the one that wrote the buffer | `src/runtime.rs` `taps_by_op` | `aliased_identity_stages_do_not_change_audio` | RED |
| M11 | aliasing is dropped: the internal boundaries keep their ops | `src/program.rs` `lower` | `aliased_identity_stages_do_not_change_audio` | RED |
| M13 | the elided-binding guard is removed, so a processor bound to an alias is dropped silently | `src/lib.rs` `PreparedGraphPlan::lowered` | `aliased_identity_stages_do_not_change_audio` | RED |
| M14 | lowering no longer refuses an unsorted `spec.nodes` | `src/program.rs` `lower` | `malformed_inputs_are_rejected_rather_than_lowered` | RED |
| M10 | `lane` dropped from the graph crate's expected dependency list | `scripts/check-graph-policy.sh` | `bash scripts/check-graph-policy.sh` | RED |

## Disclosed equivalent mutants

* **Swapping only the first two inputs of a reduction** is equivalent: IEEE addition is commutative,
  so `a + b == b + a` bit for bit. Order sensitivity begins at the third input, which is what M1
  actually perturbs.
* **Copying a single in-place input through a scratch buffer** produces identical audio -- it is a
  performance property, not a numeric one. It is gated by `program.buffers <= 2` in
  `aliased_identity_stages_do_not_change_audio`, by the arena's `debug_assert_ne!` in `split2`, and
  by the descriptive 64-track measurement, not by a bit comparison.
* **Changing the reduction order in `reduce_plane` alone does not redden**
  `fifty_random_dag_sessions_render_bit_identically_in_both_executors`, and must not: both
  executors call the same function, so that gate proves *agreement*, while M1/M1c prove the order.

## Issue #100 additions -- the pull-model arena, the persistent pool and bounded recovery

Same protocol: each row below was applied to the working tree, the named test was run, the failure
was recorded, and the mutation was reverted in the same session. Every row was run on this branch
except where the "result" column says otherwise.

| # | mutation | file | test | result |
|---|---|---|---|---|
| N1 | accept a second writer for an arena buffer (delete the I1 check) | `engine` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::overlapping_writes_are_rejected` | RED |
| N2 | accept a read of a producer in the reader's own wave (delete the I2 check) | `engine` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::a_read_from_the_same_wave_is_rejected` | RED |
| N3 | read a muted buffer directly instead of the silence slot | `engine` `ArenaLease::effective` | `disjoint::tests::a_muted_read_is_silence_and_unmuting_restores_it` | RED |
| N4 | off-by-one in the arena's write address, so a lease writes its neighbour | `engine` `ArenaLease::write` | `disjoint::tests::concurrent_leases_never_write_a_foreign_word` (`--release`) | RED |
| N5 | never take the executor hand-over at the block-boundary swap | `engine` `RealtimePlanOwner::enter_block` | `realtime::tests::enter_block_moves_the_executor_handover_and_returns_a_refused_one` | RED |
| N6 | add `unsafe` to a second `realtime/` file | `scripts/check-realtime-policy.sh` fixture | `scripts/test-realtime-policy.sh` (`unsafe-outside-disjoint-arena`) | RED |
| N17 | forget the silence-slot offset in the sequential executor's output buffer | `graph` `GraphExecutor::new` | builtins-fixture `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected` | RED (observed as a real defect during this work, then fixed) |

N7-N16 covered the native dependency-wave scheduler and were retired with it: the scheduler crate,
the `bind_native` family and the cross-executor 50-DAG oracle no longer exist, so none of those
mutations can be expressed. N1-N5 are unaffected -- the disjoint arena and its lease API are what
the *sequential* executor renders through, so they remain live production code with live gates.

N4 is the one row whose mutation is not the check it guards: I1 makes a foreign write unexpressible
through the builder, so the stress is mutated at the address arithmetic instead, which is the
failure I1 exists to make impossible.

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-5 | the `console.control.stage(..)` drain in `execute_op`'s `ConsoleEffect` arm never runs, so an admitted parameter reaches the effect a block late | `graph/src/runtime.rs` | `tests::a_console_parameter_command_applies_at_the_next_block_boundary` | RED (`every sample of the block that drains the command carries it`) |
| 140-6 | the `console.shunt.capture(..)` call is deleted, so a bypassed block renders the shunt's initial zeros instead of the latency-matched input | `graph/src/runtime.rs` | `tests::live_bypass_is_latency_preserving_and_reversible` | RED (`a bypassed block is the input delayed by exactly the declared latency`) |

## Issue #218 — the route fold and the in-order scatter-accumulate mixdown

Every row below was applied to the working tree, the named suites were run, the failure (or the
absence of one) was recorded, and the mutation was reverted in the same session. Host: `x86_64`,
workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one
mutation at a time over `cargo test -p graph -p graph-compiler
-p console-workload`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 218-1 | the first contributor accumulates instead of storing (`copy_from_slice` becomes `sum_into_block`) | `graph/src/runtime.rs` `ArenaMembers::fold_plane` | `runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign` | RED (`+0.0` where `-0.0` is required; bits 0 against 0x8000_0000) |
| 218-2 | the fan-in-zero fill is skipped for *every* kind, not only a bound source | `graph/src/runtime.rs` `execute_op` | `runtime::tests::the_fan_in_zero_fill_is_dead_only_under_a_bound_source` | RED (an identity node with no contributors renders the previous block instead of silence) |
| 218-3 | the association proof keeps its length check and drops the element-wise comparison | `graph/src/runtime.rs` `route_fold` | `route_ids_ordered_against_the_cohorts_decline_the_route_fold` | RED, plus `a_leased_stage_meter_declines_the_merge_and_still_meters` and `a_pre_fader_meter_splits_its_cohorts_chain_and_reads_the_limiter` |
| 218-4 | the observer clause is dropped from `foldable_lane` | `graph/src/runtime.rs` | `runtime::tests::a_post_matrix_meter_on_every_track_of_a_full_bank_keeps_the_fold_armed` | RED (a meter at `PostFader` no longer declines the fold). Re-run for #885: the original catcher, `the_folded_master_is_the_reductions_own_bits`, relied on a post-matrix meter declining the fold, a premise #885 removed; it now stays GREEN under this mutation. |
| 218-5 | the opening chain's own ops are no longer excluded from the in-between master scan | `graph/src/runtime.rs` `route_fold` | `every_standing_workload_folds_one_route_per_track` | RED (nothing folds at all: the session output's colour is track zero's input slot) |
| 218-6 | sole readership of a chain's last slot is dropped (`len() != 1` becomes `is_empty()`) | `graph/src/runtime.rs` `foldable_lane` | — | **GREEN.** Reported rather than dressed up: the clause is real but shadowed. A second route from the same tap adds a summand the master's input list carries, so the association proof declines on length first; a sidechain from that tap is read by an op scheduled before the route, so `readers[producer][0]` is not a route and the plain-route clause declines instead. |
| 218-7 | the in-between master scan is dropped entirely | `graph/src/runtime.rs` `route_fold` | — | **GREEN.** No compiled session reaches the hazard: the master's colour is the first colour the lowering frees, which is track zero's input buffer, and track zero is always in the opening cohort — whose ops the scan excludes anyway. Expressible in a lowered program, not in a session, exactly as `scatter_target`'s compensation-delay clause is. |
| 218-8 | the "one master op" retain admits candidates reducing into different masters | `graph/src/runtime.rs` `route_fold` | — | **GREEN.** Shadowed by the association proof's length check. |

Rows 218-6 to 218-8 are the honest half of this ledger. Each clause is kept because it defends a
hazard that is real in the lowered program and unreachable from a session the compiler can build,
and the reason is written down beside the clause in `route_fold`'s doc comment rather than left to
be rediscovered.

## Mono-collapse M2 — the dispatch, the structural join and the transition

The collapse renders the bits a dual run renders, so the rows below are counters and cross-arm
comparisons; a digest gate on a single arm cannot see any of these failures. They live beside the
console fixtures (`tools/console-workload/tests/chain_shape.rs`) because the *production*
plan is what they are about and it is assembled there.

Driver: one mutation at a time, `cargo test -p console-workload --test chain_shape`,
tree restored (and `touch`ed) between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-G1 | `Runtime::arm_mono_collapse` arms every banked unit regardless of its lanes' tracks (`let armed = !tracks.is_empty();`), so the structural join is performed and ignored | `graph/src/runtime.rs` | `the_half_mono_cohort_banks_like_a_uniform_one` (and `the_collapse_fires_on_every_mono_cohort_and_no_other`) | RED — 2 failed. The half-mono row renders the *uniform mono* row's bits, because its odd tracks' right channels become the duplicated left ones. This is the failure that found the hole: the runtime witness is source-agnostic by construction and admits every lane of that row |

The join is where this row has to be cut, and an earlier draft cut it in the wrong place. Flipping
`BankChain::new`'s `collapse_source` default to `true` is **green** on this suite: every plan the
console builds is joined, and `arm_mono_collapse` writes the field on every chain, so the default is
overwritten before a block renders. That mutation is still a real gate -- it reds
`rack`'s `an_unarmed_chain_never_collapses`, where nothing performs the join -- but it is
a gate on the *default*, not on the join, and the two are separate claims. Both are listed, in the
crate whose test carries each.
| M2-G2 | the disengage copy is skipped (`slot.stage.desymmetrize()` becomes a no-op) | `rack/src/lib.rs` `disengage_collapse` | `a_run_that_stops_collapsing_renders_what_a_never_collapsed_run_renders` | RED |
| M2-G3 | the drain is moved back after the dispatch | `rack/src/lib.rs` `run` | `a_live_one_channel_retarget_disengages_on_the_block_it_lands` | RED — see the rack's M2-5 row for what the ordering protects |

## Mono-collapse M3 — the transition evidence

Same driver: `cargo test -p console-workload --test chain_shape`, one mutation at a
time, tree restored between rows.

M3's graph-layer contribution is one accessor, and the reason it needs a row is the reason the M2
block counters needed theirs. The collapse renders the bits a dual run renders, so nothing a digest
can see distinguishes a chain that collapsed for the whole session from one that collapsed, retired
and came back. `collapse_transitions` is the only statement of the difference, and an accessor that
reported zeros would leave every re-engage assertion in the tree passing vacuously.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M3-G1 | `RuntimeUnit::collapse_transitions` returns `[0; 3]` for a banked unit, so the cycle is invisible above the chain | `graph/src/runtime.rs` | `chain_shape::the_switch_coming_back_re_engages_and_renders_the_never_collapsed_bits` | RED — `every cohort disengaged once and re-engaged once`. `no_workload_transitions_unless_something_moves_the_switch` stays green under it, which is the shape that names the cause: an all-zero accessor is indistinguishable from an undisturbed session and only a session that *did* transition can tell them apart |

## Issue #371 — the root-agnostic marked-region scan (RT-16 / IO-14)

The gate's discovery set changed from one directory (`crates/engine/src/realtime`) to every file
carrying a `REALTIME_POLICY_BEGIN` marker under `crates hosts tools`, and the
`>= 4` region floor became a floor on both the marked-region count (42) and the marked-file count
(12). Rows 371-1 and 371-2 were applied to the working tree itself, the gate run, and the mutation
reverted in the same session; rows 371-3 to 371-6 live in `scripts/test-realtime-policy.sh` (each
asserting the failure class, not just a non-zero exit) against that fixture, which mirrors the
real twelve-file, forty-two-region layout, column-zero and indented markers included.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 371-1 | `let _ = vec![0u8; 1];` inserted inside `execute_op`'s marked region | `graph/src/runtime.rs` | `bash scripts/check-realtime-policy.sh` | RED (`marked realtime code contains an allocation, lock, I/O, log, wait, syscall or panic surface`, pointing at the inserted line); GREEN once removed — the RT-16 verification gate |
| 371-2 | `let _ = vec![0u8; 1];` inserted inside `EffectControlLane::stage`'s marked region | `effect-contract/src/live.rs` | `bash scripts/check-realtime-policy.sh` | RED (same class); GREEN once removed — the IO-14 verification gate, also standing as `marked-effect-control-lane-stage` in `scripts/test-realtime-policy.sh` |
| 371-3 | a marked region's forbidden surface outside the old root: `let _ = vec![0u8; 1];` inside `render_next`'s marked region | `hosts/host-web/src/lib.rs` (fixture) | `scripts/test-realtime-policy.sh` (`marked-outside-realtime-root`) | RED (allocation class) |
| 371-4 | delete every marker of one file to silence the gate, dropping it out of the discovered set | `crates/builtins/src/lib.rs` (fixture) | `scripts/test-realtime-policy.sh` (`marked-file-count-floor`) | RED (`expected at least twelve marked realtime files`) |
| 371-5 | delete one marked region of a multi-region file, every remaining marker matched | `crates/rack/src/lib.rs` (fixture) | `scripts/test-realtime-policy.sh` (`marked-region-count-floor`) | RED (`expected at least forty-two marked realtime regions`) |
| 371-6 | a newly marked `tools/` file carrying a `vec!` inside its region | `tools/audit/src/marker_probe.rs` (fixture) | `scripts/test-realtime-policy.sh` (`marked-tools-root-scanned`) | RED (allocation class) — the discovery walk reaches every root, not only `crates/` and `hosts/` |
| 371-7 | delete the `END` marker of a region outside the old root, keeping its `BEGIN` | `hosts/host-web/src/lib.rs` (fixture) | `scripts/test-realtime-policy.sh` (`unmatched-markers-outside-root`) | RED (`unmatched realtime policy markers`) |

## Issue #915 — the fused fold epilogue (`ArenaMembers::fold_resident`)

Every row below was applied to the working tree at `62c76b08`, the named suites were run, the
failure (or its absence) was recorded, and the tree was restored with `git checkout` before the
next row. Host: `x86_64`, workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`, debug
profile. Sweep driver: one mutation at a time over `cargo test -p graph --lib -- resident_fold
a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, `cargo test -p rack
--lib -- a_fully_folded_full_bank_offers resident_fold_cohort` and `cargo test -p console-workload
--test chain_shape -- the_folded_master_is_the_reductions_own_bits`. "Gate 1" is
`runtime::tests::a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit`; "the `-0.0`
test" is `runtime::tests::a_resident_fold_stores_its_first_contributor_so_a_negative_zero_master_keeps_its_sign`;
"metered" is `runtime::tests::a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`.
The rack rows for the same issue (the offer itself) are in `rack/tests/MUTATIONS.md`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 915-1 | the coefficient roles swap on the left output: `lr.fma(right, ll.mul(left))` becomes `ll.fma(right, lr.mul(left))` | `graph/src/runtime.rs` `route_word` | gate 1, metered, console-workload `the_folded_master_is_the_reductions_own_bits` | RED, RED, RED (`Four, 4 frames, 1 cohort(s), store true: left master`; `Four/4/resident meters`; `nine_track_baseline`) |
| 915-2 | lanes `1..W` accumulate in reverse (`.enumerate().skip(1)` becomes `.enumerate().skip(1).rev()`) | `graph/src/runtime.rs` `fold_words` | gate 1, metered, console-workload | RED, RED, RED (one-ulp differences, e.g. `1258671376` for `1258671375`; console `nine_track_baseline`) |
| 915-3 | a continuation is seeded from zero instead of from the live master (`L::load(master).add(routed)` becomes `L::zero().add(routed)`, both planes) | `graph/src/runtime.rs` `fold_words` | gate 1, the `-0.0` test, metered, console-workload | RED, RED, RED, RED (`store false: left master`; `store false, start -0: frame 0`; `Four/8`, the two-cohort shape; `sixty_four_track_console`) |
| 915-3b | the first contributor is seeded from zero and added instead of stored (`(routed_left, routed_right)` becomes `(L::zero().add(routed_left), L::zero().add(routed_right))`) | `graph/src/runtime.rs` `fold_words` | the `-0.0` test | RED (`Four, store true, start 0: frame 0`). **GREEN on gate 1, metered and console-workload**, disclosed: the two forms differ only on a frame whose every contributor is `-0.0`, which no hostile-random corpus produces. That is why the `-0.0` test is an absolute property and not another differential. |
| 915-4 | the ragged tail is skipped (`for frame in tiled..frames` becomes `for frame in frames..frames`) | `graph/src/runtime.rs` `fold_resident_tiles` | gate 1, the `-0.0` test, metered | RED, RED, RED (`Four, 13 frames`; `frame 12`; `Four/4` at 13 frames). **GREEN on console-workload**, disclosed: every standing console workload renders a 128-frame quantum, a whole number of tiles at both widths, so no console block has a tail. |
| 915-5 | `fold_resident` returns `true` having written nothing | `graph/src/runtime.rs` `ArenaMembers::fold_resident` | all three new graph tests, metered, console-workload, and `tests/rt1_direct_bank_alloc.rs` `direct_bank_graph_render_is_allocation_free_and_bit_exact` | RED on every one (`the control must write the master`; `left master`; `nine_track_baseline`; rt1's folded PCM check) |
| 915-6 | the "only lane 0 may store" premise is dropped | `graph/src/runtime.rs` `fold_resident_tiles` | `runtime::tests::a_resident_fold_declines_before_writing_on_a_broken_premise` | RED (`Four: a later lane stores must decline`). GREEN elsewhere: no compiled plan gives a later lane `store`, so the premise defends a lowered shape a session cannot produce. |
| 915-7 | the fma operand order is swapped on the left output: `lr.fma(right, ll.mul(left))` becomes `ll.fma(left, lr.mul(right))`, i.e. `(ll * l) + (lr * r)` for `(lr * r) + (ll * l)` | `graph/src/runtime.rs` `route_word` | every suite above | **GREEN, and not a catcher** (the brief says so in advance): `Lane::fma` is the unfused `(a * b) + c` on every backend and IEEE addition of two finite values is commutative. The corpus is finite by construction. The two forms can differ only in which NaN payload propagates when both products are NaN, and nothing here claims to test that. |

Rows 915-3b, 915-4 (console only) and 915-6 are the honest half of this ledger. Each mutation is
caught, but only by the gate built for it and not by the end-to-end digests. The end-to-end
digests cannot reach an all-`-0.0` frame, a tail, or a later lane that stores.

## Issue #916 — the master written straight into the host planes

Every row below was applied to the working tree at `608f0379`, the whole graph lib suite was run
(`cargo test -p graph --lib`, 98 tests), every red test was recorded, and the tree was restored
before the next row. Each mutation is an exact-text replacement, so an unmatched pattern would have
been reported rather than silently skipped. Host: `x86_64`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile. The new tests these rows name:

- "Gate 1" is `runtime::tests::the_host_planes_are_the_arena_oracles_master_bit_for_bit_at_every_stride`.
- "Gate 3" is `runtime::tests::a_failed_render_silences_the_host_planes_and_a_rejected_one_leaves_them_alone`.
- "The kernel test" is `runtime::tests::a_host_plane_reduction_is_the_arena_reduction_bit_for_bit`.
- "Metered" is `runtime::tests::a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 916-1 | the pre-issue arena path: `preflight_sequential` resolves no Output op (so there is no Output unit and every fold master is `FoldTarget::Arena`), and `render` copies the Output's arena buffer into the host planes after the last unit | `graph/src/runtime.rs` `preflight_sequential`, `graph/src/lib.rs` `render` | gate 1 | RED, on its structural `output_unit.expect("an Output unit")`. Every other test is GREEN, because this path renders the same bits. |
| 916-1b | 916-1, with gate 1's structural Output-unit assertion removed from the test | as 916-1, plus the test | gate 1 | RED on the behavioural check alone: `Folded(13, false), stride 13, block 0: the output slot was written`. The block's plane and padding assertions, which run first, had passed. This is the brief's "keep the end-of-block copy" row. Only the no-arena-write assertion sees it, and it does. |
| 916-2 | the Output op writes its arena buffer and not the host (`execute` passes the Output op no `HostMaster`) | `graph/src/runtime.rs` `Runtime::execute` | gate 1, metered, and 11 pre-existing lib tests | RED on all of them (gate 1: `Unfolded, stride 13, block 0: the host planes are the oracle's`, whose planes still hold the `0x7fc0_0916` pad). The Folded shapes pass this row's gate-1 checks, as they should: a folded Output op's reduction is neutralised, so it writes nothing either way. |
| 916-3 | every folded Output master is installed as an arena master (`FoldTarget::Output` is never chosen) | `graph/src/runtime.rs` `validate_fold_installation` | gate 1, metered, `route_fold_preflight_returns_original_owners_and_sources_for_retry` | RED, RED, RED (gate 1: `Folded(13, false) ... the host planes are the oracle's`; the host planes hold their pad) |
| 916-4 | the fan-in-one copy reads the other plane (`lease.read(1 - plane, ..)`) | `graph/src/runtime.rs` `reduce_plane_into` | the kernel test, gate 1, and 5 pre-existing lib tests | RED on all of them (`1 frames, fan-in 1, plane 0`; gate 1 at `Submix`, the fan-in-one submix) |
| 916-5 | the Output op is identified by buffer index, as the brief wrote it (`op.output == output slot` in `execute` for plain ops and bank members, and in `observe_unit`) | `graph/src/runtime.rs` `Runtime::execute`, `Runtime::observe_unit` | gate 1, and 5 pre-existing lib tests | RED on all of them. Gate 1 fails at `Submix ... the output slot was written`: the pad strip that shares the Output's slot rendered into the host. The lib tests include `fifty_random_dag_sessions_render_deterministic_nonsilent_pcm` (`seed 4: the corpus must not be silent`) and `level_major_w4_builtin_bank_is_analytic_for_three_blocks`. This row is why the runtime picks the Output op by node. |
| 916-6 | the scatter redirect into the Output op is no longer withheld | `graph/src/runtime.rs` `build_sequential` | gate 1 | RED (`BankIntoOutput: [route folds, scatter redirects]`, `[0, 1]`). Without the count assertion, the chain scatters into the Output's arena buffer, the Output op's reduction is neutralised, and the host planes are never written. |
| 916-7 | no `+0.0` fill when a unit fails | `graph/src/lib.rs` `render` | gate 3 | RED (`Unit: both planes are +0.0`; five folded cohorts' partial master stays) |
| 916-8 | no fill when an observer fails (`observe_unit`) | `graph/src/lib.rs` `render` | gate 3 | RED (`Observer(false): both planes are +0.0`) |
| 916-13 | no fill when an active observer fails (`observe_active_unit`) | `graph/src/lib.rs` `render` | gate 3 | RED (`Observer(true): both planes are +0.0`) |
| 916-9 | no fill when the source set's `begin_block` fails | `graph/src/lib.rs` `render` | gate 3 | RED (`Source(true): both planes are +0.0`; the previous block's master stays) |
| 916-14 | no fill when `copy_track_input` fails | `graph/src/lib.rs` `render` | gate 3 | RED (`Source(false): both planes are +0.0`) |
| 916-10 | the fill also runs on the success path | `graph/src/lib.rs` `render` | gate 1, gate 3, metered, and 15 other lib tests | RED on all of them |
| 916-11 | the session output is not dedicated storage (`is_dedicated` loses its `Output` arm) | `graph/src/program.rs` `is_dedicated` | gate 1, gate 3, `chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers`, and 6 other lib tests | RED on all of them. An in-place Output op's single input is its own slot, so its reduction is neutralised and the host planes are never written. Gate 1 fails at `Submix ... the host planes are the oracle's`, whose planes hold the pad. |
| 916-12 | `HostMaster::new` checks no length | `graph/src/runtime.rs` `HostMaster::new` | gate 3 | RED. The rejection arm's short planes reach the fold epilogue and panic (`runtime.rs` `fold_resident_tiles`, an index past the plane) instead of returning `InvalidEnvelope`. |
| 916-15 | the Output op's observers read its arena buffer, not the host planes (the `observe_unit` host arm never taken) | `graph/src/runtime.rs` `Runtime::observe_unit` | gate 1 | RED (`Folded(13, false), stride 13: every observer window is the oracle's`) |

### Issue #916 scope amendment (Sol): the slot comparisons with `program.output`

Rows 916-16 and 916-17 were applied to the working tree after the scope amendment. One mutation at a
time, the suites below were run and the tree was restored:

- `cargo test -p builtins-compiler --features test-support --lib`
- builtins-compiler `tests/allocation_tracker.rs`, with `graph/test-support` and
  `engine/realtime-audit`
- `cargo test -p graph --lib`

Each row puts back a clause that #916 removed. Neither clause identified the Output: each compared a
producer's physical slot with `program.output`. After #916 the dedicated Output takes a retired
slot. So such a clause fires by colouring coincidence, and declines a pair or a merge that is sound.
Rows 916-1 to 916-15 were recorded at `608f0379`, before this change. They mutate code this change
does not touch.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 916-16 | `chains_into` again declines `producer.output == program.output` | `graph/src/runtime.rs` `chains_into` | builtins-compiler lib: `actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners`, `actual_scalar_graph_preserves_scheduled_matrix_prefix_error_and_queue_tail`, `staggered_observed_scalar_track_stays_separate_while_eligible_peer_pairs`, `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated`, `actual_scalar_nonadjacent_failed_render_materializes_post_fader_before_error`, `actual_scalar_nonadjacent_ramp_retarget_and_failed_retry_stay_at_original_boundaries`, `actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate`; allocation_tracker: `actual_queued_scalar_graph_allocates_and_frees_nothing`, `actual_scalar_prepare_and_bind_retain_the_charged_owner_layouts`, `actual_scalar_split_table_and_failed_render_fit_the_resource_gate` | RED on all ten. The two-track harness's Output takes the slot the trailing track's fader and matrix retire from. So the trailing track stops pairing, and the counts of 2 fall to 1. **GREEN on the graph corpus**: `cohort_chain_merging_preserves_dataflow_on_random_graphs` interprets through `chains_into_model`, not the runtime predicate. Its merge pin (3862) guards the model's copy of the clause. Putting the model's clause back gives the pre-amendment 3752. |
| 916-17 | `scalar_split_interval_is_clear` again declines `program.output == buffer` | `graph/src/runtime.rs` `scalar_split_interval_is_clear` | builtins-compiler lib: `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated`, `actual_scalar_nonadjacent_failed_render_materializes_post_fader_before_error`, `actual_scalar_nonadjacent_ramp_retarget_and_failed_retry_stay_at_original_boundaries`, `actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate`; allocation_tracker: `actual_scalar_split_table_and_failed_render_fit_the_resource_gate` | RED on all five. The split pair whose fader slot the Output later takes is declined again. |

### Issue #916 Sol verdict, should-fix S1: nothing reads the session output's arena slot

Applied to the working tree after the verdict. `cargo test -p graph --lib` was run, and the tree was
restored.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 916-18 | `preflight_sequential` no longer refuses a plan that reads the session output (`output_value_is_read` not called) | `graph/src/runtime.rs` `preflight_sequential` | `runtime::tests::a_plan_that_reads_the_session_output_is_refused_at_bind` | RED (`a plan that reads the session output must not bind`): the plan binds, and its `t01 PostFader` reads the Output's never-written arena slot. The other 98 lib tests stay GREEN. |

## Issue #918 — banked source inputs gathered from the played transfer block

Each row was applied alone to a committed tree, the listed suites were run, and the file was
restored with `git checkout`. Rows 918-1 to 918-9 ran on `3857a7fb`; rows 918-10 to 918-12 ran on
`7f027946`, which adds gate 1's ninth (compensated) shape and the set's refusal test and changes
none of the code rows 918-1 to 918-9 mutate. Gate 1 is
`runtime::tests::a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy`; gate 2 is
host-core `source_in_place::a_ring_fed_banked_session_gathers_in_place_with_the_copy_bits`; gate 3
is `rt10_source_in_place_alloc::an_in_place_source_gather_renders_the_copy_bits_and_allocates_nothing`.
Gate 1 poisons every claim's arena slot (`0x7fc1_0918` / `0x7fc2_0918`) before each block, so a
read of a slot the copy no longer fills shows as those words. Where a row says "mode-table assertion
removed", the gate's `the mode table` assertion was deleted in the same edit so that the bit
comparison, not the mode table, is what the row proves red.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 918-1 | the gather returns the played planes swapped, `(right, left)` (brief: "read the wrong channel plane") | `graph/src/runtime.rs` `ArenaMembers::plane` | gate 1, gate 3, gate 2 | RED on all three (`W8 x 8 ... block 0 (Full): the master is the copy arm's`; rt10 `the in-place gather renders the copied plan's bits`; `bank console, block 0`) |
| 918-2 | an unplayed quantum reads the claim's arena buffer instead of the silence buffer (brief: "skip the silence fallback") | `graph/src/runtime.rs` `ArenaMembers::plane` | gate 1; gate 3; gate 2 | RED on gate 1 at the underrun (`W8 x 8 ... block 3 (Underrun)`, left word `2143422744` = the poison). GREEN on gates 2 and 3: there the claim's slot is never written in place mode, so it still holds the arena's initial `+0.0`, which is why gate 1 poisons the slots. |
| 918-3 | clause (a) admits a `NodeKind::TrackDelay` input (brief: "mark a TrackDelay claim InPlace") | `graph/src/runtime.rs` `source_plane_table` | gate 1; host-core `track_delay` | RED: gate 1 at the mode table (`[true, true, false, true, false, true]`), and 4 of 8 host-core `track_delay` tests (`a_declared_delay_is_exactly_a_pre_padded_source`, `the_delay_survives_the_banked_path`, `the_pre_delay_region_is_exactly_positive_zero`, `the_two_lanes_carry_independent_delays`: the gather reads the raw block, `-0.4769106 != 0.0` inside the delay). Gate 2 GREEN (no delayed track). |
| 918-3b | 918-3, mode-table assertion removed | as 918-3 | gate 1 | RED at the bits (`delayed: Some(1) ... block 0 (Full): the master is the copy arm's`): the delay line runs over the poisoned slot and the gather reads the undelayed block |
| 918-4 | the copy loop keeps every claim, in-place ones included (brief: "keep the copy loop for InPlace claims") | `graph/src/lib.rs` `GraphExecutor::new` | gate 1, gate 3, gate 2 | No bit goes red, as the brief predicts: every master, meter and digest assertion passes. RED only on the mode counters: gate 1 `the in-place arm's [copies, played gathers, silent gathers]` (left `[64, 56, 8]`), rt10 `lent: no claim copied` (`[4000, 4000]`), gate 2 `[claims copied, played gathers, silent gathers] in place` (`[96, 72, 24]`) |
| 918-5 | the table is consulted by buffer alone, ignoring `UnitIdentity::source_lanes` | `graph/src/runtime.rs` `SourceGather::claim` | gate 1; gate 2 | RED on gate 1 shape 7 (`scalar: Some(PostFader) ... block 0 (Full)`): the `PostMatrix` bank gathers each input's recoloured slot for the fader's value and is served the played block instead. GREEN on gate 2 (no recoloured slot is gathered in those sessions). |
| 918-6 | clause (c) dropped (an observed input is bound in place), mode-table assertion removed | `graph/src/runtime.rs` `source_plane_table` | gate 1 | RED (`delayed: Some(1), observed: Some(2) ...: every meter window is the copy arm's`): the input meter reads the poisoned slot |
| 918-7 | clause (b) dropped (a claim is bound in place whatever reads it), mode-table assertion removed | `graph/src/runtime.rs` `source_plane_table` | gate 1; gate 2 | RED on gate 1 (`routed: Some(4) ... block 0 (Full)`, left word `2143422744`): the route reads the poisoned slot in place. GREEN on gate 2 (every reader there is a bank gather). |
| 918-8 | the production driver lends the claim's channels swapped | `source/src/lib.rs` `SourceGraphSourceSetDriver::played_planes` | gate 2; `cargo test -p source --all-features --lib` | RED: gate 2 (`bank console, block 0`) and the #917 source tests `graph_driver_played_planes_map_claims_until_the_next_begin_or_seek`, `played_block_retention_keeps_the_pre_change_admission_sequence` |
| 918-9 | `provides_played_planes` defaults to `true` | `graph/src/lib.rs` `GraphPreparedSourceSetDriver` | `cargo test -p graph --lib` | RED: #916's `a_failed_render_silences_the_host_planes_and_a_rejected_one_leaves_them_alone` (`Source(true): the first block wrote a master`). Its `FailingSource` never overrode `played_planes`, so its bound-in-place claim reads the default `None` as an underrun on every block and renders silence: why lending is opt-in |
| 918-10 | `gathers_only` drops its `staged.is_empty()` clause | `graph/src/runtime.rs` `gathers_only` | gate 1 | **GREEN, equivalent**: a delayed input's effective read (`RuntimeOp::inputs`) is its staging slot, which is taken while the producer's slot is still live, so `inputs == [buffer]` already refuses the compensated claim |
| 918-10c | `gathers_only`'s `inputs == [buffer]` becomes `inputs.len() == 1` | `graph/src/runtime.rs` `gathers_only` | `cargo test -p graph --lib` | **GREEN, equivalent**: every reader of the input names its buffer (`op_dataflow`), so one undelayed input is the buffer. The two clauses guard the same fact twice. |
| 918-10b | both of the above: one input that is the buffer or a staged copy of it | `graph/src/runtime.rs` `gathers_only` | gate 1 | RED (`compensated: Some(3) ... block 0 (Full)`): the member stages the poisoned slot through its compensation delay |
| 918-11 | the set lends any plane of any index (no claim or length check) | `graph/src/lib.rs` `GraphSourcePlanes for GraphPreparedSourceSet` | `tests::a_source_set_lends_only_quantum_planes_of_its_own_claims` | RED (`a short plane is refused`) |
| 918-12 | the unit loop is handed no source planes (`sources = None`) | `graph/src/lib.rs` `GraphExecutor::render` | gate 1, gate 3, gate 2 | RED on all three: every in-place gather reads its poisoned or zero slot |

## Issue #926 — in-place routes fused into the Output reduction in pairs

Issue #920 designed this fold and never landed (its merge was reverted, `b04e044b`); #926 re-applied
its eligibility, retire path, table, seam and tests unchanged (`da4a3f44`) and replaced its kernel
(`67649092`). So every row below ran on the #926 tree, #920's rows included: rows 926-1 to 926-5
are the brief's five kernel rows re-applied to the pair kernel, 926-6 is the brief's new row,
926-7, 926-8 and 926-18 are the pair kernel's own, and 926-9 to 926-17 are #920's structural rows
920-6 to 920-14 re-run on the re-anchored clauses. Each row was applied alone to the committed
tree (`67649092` for 926-1 to 926-5 and 926-18, `737d6bf1`, which changes only comments, for the
rest), as exact-text replacements whose match counts were checked before writing, and the file was
restored with `git checkout` before the next row. Host: `x86_64` (`.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`), debug profile. The tests these rows name:

- "The kernel test" is `runtime::tests::a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit`.
- "Gate 1" is `runtime::tests::an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit`,
  which also carries gate 2's declining shapes.
- "The tail test" is `runtime::tests::a_route_tail_refuses_a_run_as_long_as_the_widest_lane`.
- "The console gate" is console-workload `chain_shape::the_plumbing_rows_output_fold_is_the_route_ops_own_bits`.
- "The graph suite" is `cargo test -p graph --features test-support` (lib 104, rt1 1, rt9 8, rt10 1).
- "The artifact gate" is gate 6: the simd128 AudioWorklet module built with
  `scripts/build-web-audioworklet.sh`'s own cargo invocation and flags (its digest is the script's
  when the tree is the same), then `scripts/check-web-audioworklet.sh` over the seven-file set.
  "Rule 3" is its `--kernel-shape` check (`scripts/check-web-audioworklet-callgraph.py`).

The structural rows 926-9 to 926-17 are first red on gate 1's fold-count or unit-count assertion.
Each was then run a second time with those three count assertions deleted from the test, and the
result column gives that second run's failure. For 926-12, the second run also deleted the
`op.staged.is_empty()` clause of the `Runtime` constructor's route-table `debug_assert`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 926-1 | reverse the accumulation order inside a pair (each pair's inputs and routes passed second-first) | `graph/src/runtime.rs` `route_reduce` | the kernel test, gate 1, the console gate | RED, RED, RED. Kernel: `width 1, 1 frames, fan-in 6, -0.0 false: left`, one ulp apart (`3307340368` against `3307340369`); a first pair alone is commutative, so fan-in two cannot see it. Gate 1: `Plain, fan-in 64, 1 frames, block 3: the host planes`. Console: `the fused Output reduction is not the route ops' and the reduction's bits`. |
| 926-2 | seed the first pair from `+0.0` instead of taking `mix(in0)` as the value (`L::splat(0.0).add(mix)`) | `graph/src/runtime.rs` `route_run` | the kernel test, gate 1, the console gate | RED, RED, GREEN. Kernel: `width 1, 1 frames, fan-in 2, -0.0 true: left`, `+0.0` against `-0.0`. Gate 1: `NegativeZero, fan-in 64, 1 frames, block 0: the host planes`. The console row never sums to an all-`-0.0` frame, the only frame a `+0.0` seed changes. |
| 926-3 | apply the 2x2 to the running sum instead of the input (`acc = mix(acc) + (l, r)`) | `graph/src/runtime.rs` `add_mixed_chunks` | the kernel test, gate 1, the console gate | RED, RED, GREEN. Kernel: `width 1, 1 frames, fan-in 2, -0.0 false: left`. Gate 1: `Plain, fan-in 64, 1 frames, block 0`. Every console route is the identity 2x2 at 0 dB, so mixing the running sum is the same arithmetic there. |
| 926-4 | swap the coefficient roles of the left plane (`ll.fma(r, lr.mul(l))`) | `graph/src/runtime.rs` `mix_chunk` | the kernel test, gate 1, the console gate | RED, RED, RED. Kernel: `width 1, 1 frames, fan-in 2, -0.0 false: left`. Gate 1: `Plain, fan-in 64, 1 frames, block 0`. |
| 926-5 | store in every pair (`let initial_store = true;` in `route_reduce`) | `graph/src/runtime.rs` `route_reduce` | the kernel test, gate 1, the console gate | RED, RED, RED. Kernel: `width 1, 1 frames, fan-in 3, -0.0 false: left`, the first fan-in with a second pair. Gate 1: `Plain, fan-in 64, 1 frames, block 0`. |
| 926-6 | hoist the whole table's coefficients before the pair loop (`route_reduce` splats every route into one `Vec<[L; 4]>` before walking the pairs, and each pair's vector run reads its splats from it) | `graph/src/runtime.rs` `route_reduce`, `route_pair`, `route_run` | the artifact gate; `scripts/check-realtime-policy.sh`; the console gate; the graph suite | **GREEN on the artifact gate** (exit 0): rule 3 still reads `route_reduce` 44 vector / 0 scalar, because a hoist changes where splats are formed, not which family the arithmetic is in; and the gate's allocation half cannot see the `Vec`, because the `miso_engine_web_v1_render` closure it walks stops at `PreparedRenderPlan::render_inner`'s `call_indirect` into the executor (8 members; no `graph` function is among them). RED on the realtime policy (`marked realtime forbidden-body predicate`, `runtime.rs:664 ... .collect()`). RED on the console gate: SIGABRT, `bench-support`'s audited allocator aborting on an allocation inside the render scope. GREEN on the graph suite (the bits do not move). The brief expected this row red by the callgraph gate; it is red by the realtime policy and the render allocation audit instead. |
| 926-7 | inline the tail into the `L`-generic kernel (`route_tail` marked `#[inline(always)]`), #920's failure shape | `graph/src/runtime.rs` `route_tail` | the artifact gate; the graph suite | RED on the artifact gate at rule 3: `FAIL kernel ...route_reduce...4wide6f32x4...: vector=44 scalar=176`. GREEN on the graph suite: native tests cannot see it, which is why gate 6 is in the issue. |
| 926-8 | drop the tail's widest-lane bound | `graph/src/runtime.rs` `route_tail` | the tail test; the artifact gate; the objdump pre-check | RED on the tail test (`8 frames, 2 inputs`, `true` against `false`). GREEN on the artifact gate (exit 0): rule 3 inspects only functions named for the four-lane type, and `route_tail` is non-generic. RED on gate 6's pre-check: `route_tail` carries 24 `f32x4` operations beside 44 scalar ones, LLVM having vectorised both of its accumulate forms. |
| 926-9 | drop the `observed` clause (920-6) | `graph/src/runtime.rs` `output_route_fold` | gate 1 | RED on the count (`ObservedRouteAlias, fan-in 64, 1 frames: folds`, 64 against 0). With the counts deleted: RED, `ObservedRouteAlias, fan-in 64, 1 frames: every observer saw every block` (0 windows against 8): the alias meter hangs off the retired route op, so it is never dispatched. |
| 926-10 | drop the in-place clause (`!route_op.in_place` and `route_op.output != route_input.buffer`) (920-7) | `graph/src/runtime.rs` `output_route_fold` | gate 1 | RED on the count (`SharedInput ... folds`, 65 against 0). With the counts deleted: RED, `SharedInput, fan-in 64, 1 frames, block 1: the host planes`: the two routes that copied are retired, and the Output reads their never-written buffers. |
| 926-11 | drop the sole-reader clause (`readers[route] == [master_op]`) (920-8) | `graph/src/runtime.rs` `output_route_fold` | gate 1 | RED on the count (`LateReader ... folds`, 64 against 0). With the counts deleted: RED, `LateReader, fan-in 64, 1 frames: every observer window is the oracle's`: the late `PostFader` meter reads the unmixed words. |
| 926-12 | drop the master's delayed-input clause (920-9) | `graph/src/runtime.rs` `output_route_fold` | gate 1 | RED on the constructor's `debug_assert` (`a folded route table belongs to the Output op's identity reduction`: the Output op stages). With the counts and that clause deleted: RED, `DelayedEdge, fan-in 64, 1 frames, block 0: the host planes`, `-0.0` (`2147483648`) where the oracle has `+0.0`: the line's `+0.0` warm-up words are mixed through the delayed route's negative 2x2. |
| 926-13 | a non-route producer folds with an identity 2x2 (`plain_route_gains(..).unwrap_or([1.0, 0.0, 0.0, 1.0])`) (920-10) | `graph/src/runtime.rs` `output_route_fold` | gate 1 | RED on the count (`SubmixContributor ... folds`, 65 against 0). With the counts deleted: RED, `SubmixContributor, fan-in 64, 1 frames, block 0: the host planes`, `+0.0` where the oracle keeps `-0.0`: the pass-through's `(-0.0, +0.0)` left plane mixes to `+0.0`. |
| 926-14 | drop the in-between `op_names_buffer` scan (920-11) | `graph/src/runtime.rs` `output_route_fold` | the graph suite; the console gate | GREEN (104 of 104, and the console gate). No test can make the scan fire: a write between the route and the Output would need the live buffer's slot, which the colouring owns through the Output op, and a read is a second reader, which 926-11's clause declines. The scan is the belt to that brace, as `route_fold`'s is. |
| 926-15 | drop the no-bank clause (920-12) | `graph/src/runtime.rs` `output_route_fold` | the graph suite; the console gate | RED on five graph tests at bind, `graph.route_fold.master`: `a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy`, `a_failed_render_silences_the_host_planes_and_a_rejected_one_leaves_them_alone`, `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, `a_post_matrix_meter_on_every_track_of_a_full_bank_keeps_the_fold_armed`, `the_host_planes_are_the_arena_oracles_master_bit_for_bit_at_every_stride` (#920 saw four; #918's gate 1 is the fifth). RED on the console gate's other-rows sweep, at bind (`nine_track_baseline: console graph bindings`). On a banked plan whose chain fold is admitted, both folds claim the same routes and `validate_fold_installation` refuses the bind. |
| 926-16 | the Output fold's routes are not retired: they keep their units and run, and the table still applies (920-13) | `graph/src/runtime.rs` `build_sequential`, `validate_fold_installation` | gate 1; the console gate | RED on the unit count (`Plain ... each folded route is one unit fewer`, 193 against 129) and on the console gate's census (`the plumbing row must retire one route op per track`, 0 against 64). With the counts deleted: RED, `Plain, fan-in 64, 1 frames, block 0: the host planes`: every route is mixed twice. |
| 926-17 | the routes are retired but the table is not installed (`output_routes` empty) (920-14) | `graph/src/runtime.rs` `build_sequential` | gate 1; the console gate | RED on the count (0 against 64). With the counts deleted: RED, `Plain, fan-in 64, 1 frames, block 0: the host planes`: the Output sums the unmixed inputs. GREEN on the console gate: its routes are the identity 2x2 at 0 dB, so the unmixed sum is the mixed one there and the census still drops by 64. |
| 926-18 | skip an odd fan-in's lone last input (the `G = 1` arm returns `true`) | `graph/src/runtime.rs` `route_reduce` | the kernel test, gate 1, the console gate | RED, RED, GREEN. Kernel: `width 1, 1 frames, fan-in 3, -0.0 false: left`. Gate 1: `Plain, fan-in 9, 1 frames, block 0: the host planes`, the gate's odd fan-in. The console row's fan-in is sixty-four, all pairs. |

## Issue #925 — identity-bound builtin stages of a builtins-less plan lower as aliases

Each row was applied alone to `0973b805`, the three suites below were run with `--no-fail-fast`,
and the file was restored with `git checkout`. Suites: `cargo test -p graph --lib` (103 tests),
`cargo test -p graph-compiler --lib` (73), `cargo test -p console-workload --test chain_shape` (23).
Gate 1 is `tests::identity_bound_builtin_stages_alias_without_moving_a_bit`; gate 2 is
`program::tests::unlisted_builtin_stages_lower_as_aliases` with graph-compiler
`builtins_replace_only_the_three_internal_track_bindings`; gate 3 is console-workload
`the_plumbing_row_is_input_route_output_and_renders_the_base_bits`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 925-1 | elide the three builtin stages whatever the bindable set says (`is_alias_candidate(id) \|\| is_builtin_stage(id)`) | `graph/src/program.rs` `lower_with` | the three suites | RED everywhere a stage is listed: graph 31 of 103 (gate 2 `PostInputBuiltins listed`: the listed stage lost its op; gate 1's listed arm; E9; every builtin-bank test, whose members `lowered()` now refuses as elided bindings), graph-compiler 23 of 73, chain_shape 22 of 23 (every with-builtins workload fails to bind: `sixty_four_track_console: console graph bindings`). Only the builtins-less plumbing test survives. |
| 925-2 | keep the post-input stage out of the alias predicate because it is dedicated (`is_builtin_stage(id) && !is_dedicated(id) && !listed[index]`): the dedicated stage keeps its op while the other two alias | `graph/src/program.rs` `lower_with` | the three suites | RED: gate 1 op count (`left: 10, right: 7`), gate 2 (`Input, PostInputBuiltins, Route, Output`), graph-compiler `builtins_replace_only_the_three_internal_track_bindings` and `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins` (builtins-less arm), gate 3 (unit count). Every other test GREEN. |
| 925-3 | alias only `PostMatrix` (`is_builtin_stage` names one stage) | `graph/src/program.rs` `is_builtin_stage` | the three suites | RED: gate 1 op count (`left: 13, right: 7`), gate 2 (`Input, PostInputBuiltins, PostFader, Route, Output`), the same two graph-compiler tests, gate 3. Every other test GREEN. |
| 925-4 | `lower_from_current_fields` hands `lower` an empty bindable set instead of `required_bindings` | `graph/src/lib.rs` `lower_from_current_fields` | the three suites | RED: graph 21 (E9, gate 1's listed arm, every builtin-bank plan: its members are listed and now elided, so bind refuses them), graph-compiler 23, chain_shape 22 of 23. The seam, not only the predicate, is what keeps a listed stage's op. |
| 925-5 | the builtins-less compile keeps listing the three stages (`=> true`) | `graph-compiler/src/compile.rs` `required_bindings` | the three suites | RED: graph-compiler `accepted_session_compiles_binds_and_renders_direct_route` (`required_bindings.len()`), `builtins_replace_only_the_three_internal_track_bindings` (the builtins-less op list), the merged-span test's builtins-less arm, and gate 3 (321 units). graph GREEN (no compiler there): the compile change is what moves the row. |

## Issue #927 — plain-strip sources read in place by the fused Output reduction

Each row was applied alone to `7896ac7e` as exact-text replacements whose match counts were checked
before writing, the suites were run with `--no-fail-fast`, and the files were restored with
`git checkout` before the next row. Host: `x86_64`
(`.cargo/config.toml` pin `-C target-feature=+avx2,+fma`), debug profile. The tests these rows name:

- "Gate 1" is `runtime::tests::a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits`,
  which carries gate 3's underrun (block 3, claim 9) too.
- "Gate 2" is `runtime::tests::a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits`. It
  visits the shapes in the order `Plain` (skipped), `SendTap`, `DelayedEdge`, `SubmixReader`,
  `BoundStage`, `TrackDelayed`, `ObservedInput`, `ObservedAlias`, `LateInput`, `DeadClaim`,
  `RouteEdgeDelayed`, `FoldDeclined`, each at 1, 7, 16 and 128 frames, so the first failure names
  the first shape a row reaches.
- "The kernel test" is `runtime::tests::a_route_reduction_reads_each_lent_input_as_the_copys_words`.
- "rt10" is `rt10_source_in_place_alloc::an_in_place_output_read_renders_the_copy_bits_and_allocates_nothing`
  (the bankless plan; #918's banked test shares the binary).

Gates 1 and 2 poison every claim's arena slot (`0x7fc1_0927` / `0x7fc2_0927`, `2143357223` /
`2143422759`) before each block, so a read of a slot the copy no longer fills shows as those words.
"Bits-only" means the row was run a second time with three assertions of `assert_ring_shape`
replaced by no-ops in the same edit -- the mode table, the Output's per-input claims, and the
in-place arm's per-block counts -- so that the host planes or a meter window, not a mode assertion,
is what goes red.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 927-1 | the Output reads every input from the arena, while the admitted claims' copy is still skipped (brief: "read `lease.read` for an admitted claim after skipping its copy") | `graph/src/runtime.rs` `OutputSources::input` | gate 1, gate 2, the kernel test, rt10 | RED on all four. Gate 1 `Plain, 1 frames, block 0 (Full): the host planes are the copy arm's` (left word `2143422759`, the poison); gate 2 `SendTap, 1 frames, block 0 (Full)`; kernel `width 1, 1 frames, fan-in 2: [copies, played reads, silent reads]` (`[0, 0, 0]` against `[0, 1, 0]`); rt10 `the in-place Output read renders the copied plan's bits` (the never-written slots hold the arena's initial `+0.0`). The brief expected block 2; the poisoned slots make it block 0. |
| 927-2 | (b') admits a claim with two readers: `[reader]` becomes "any reader that is a retired route" (brief) | `graph/src/runtime.rs` `source_plane_table` | gate 1, gate 2, the kernel test, rt10 | **GREEN, equivalent.** A retired route runs in place over the claim's buffer, and `program::lower` lowers a consumer in place only over a buffer whose owner has exactly one reader (`reads_of[owner] == 1`); so no claim has a retired route among two readers, and the pattern and the fold's in-place clause guard one fact. 927-2b is the brief's intent in a form that is not equivalent. |
| 927-2b | (b') keeps only its schedule clause (e): every plain, unobserved claim that (b) declines skips the copy, and the Output reads it at its route's input only when its one reader is a retired route | `graph/src/runtime.rs` `source_plane_table` | gate 2; bits-only | RED `SendTap, 1 frames: the mode table` (claim 5, with two readers, in place). Bits-only: RED `SendTap, 1 frames, block 0 (Full): the host planes are the copy arm's` (the poison): the bound fader and the send read the slot the copy no longer fills. |
| 927-2c | (b') keyed on the buffer, not the reader: a plain claim is read at the Output input whose retired route's buffer is the claim's buffer, whatever reads it | `graph/src/runtime.rs` `source_plane_table` | gate 2; bits-only | RED `SubmixReader, 1 frames: the mode table`. Bits-only: `SubmixReader` passes -- its submix runs in place over the claim's buffer and is a no-op identity, so that Output input holds the claim's own words -- and RED `BoundStage, 1 frames, block 0 (Full)`: the bound fader processes the claim's slot in place, and the kernel reads the raw played block instead of the fader's words. |
| 927-3 | an unplayed quantum reads the claim's arena buffer instead of the silence buffer (brief) | `graph/src/runtime.rs` `OutputSources::input` | gate 1, gate 2, the kernel test, rt10 | RED on gate 1 at the underrun, `Plain, 1 frames, block 3 (ClaimUnderrun): the host planes` (the poison), on gate 2 `SendTap, 1 frames, block 3 (ClaimUnderrun)`, and on the kernel test `width 1, 1 frames, fan-in 9: left` (the poison; fan-in 9 is the first with a silent input). GREEN on rt10: its claims' slots are never written in place, so they still hold the arena's initial `+0.0`, which is why gates 1 and 2 poison. |
| 927-4 | clause (e) dropped | `graph/src/runtime.rs` `source_plane_table` | gate 2; bits-only | RED `LateInput, 1 frames: the mode table`. Bits-only: RED `LateInput, 1 frames, block 0 (Full): the host planes`: on the copy arm the empty submix's `+0.0` overwrites track 5's copied words before the Output reads them, and the in-place arm reads the played block. |
| 927-5 | the Output selects each input's claim by **buffer** through `source_plane_of_buffer`, the brief's literal lookup (three edits: the Op arm hands the buffer table, `OutputSources::input` indexes it by buffer, `route_reduce` drops its length check) | `graph/src/runtime.rs` `Runtime::execute`, `OutputSources::input`, `route_reduce` | gate 1, gate 2, the kernel test, rt10 | RED on gate 2 `DeadClaim, 1 frames, block 0 (Full): the host planes are the copy arm's`: the dead claim's slot, which #918 tables because nothing reads it, is the fader buffer an Output input reads, and that input is served the dead claim's block. RED on the kernel test (its claims are per position; a buffer index finds none: `[0, 0, 0]`). GREEN on gate 1 and rt10, which have no recoloured slot. This row is deviation 1's evidence. |
| 927-6 | clause (a) admits a `NodeKind::TrackDelay` input (shared with #918's (b)) | `graph/src/runtime.rs` `source_plane_table` | `cargo test -p graph --lib`; gate 2 bits-only | RED: gate 2 `TrackDelayed, 1 frames: the mode table`, and #918's gate 1 at its mode table (`delayed: Some(1)`, 918-3's failure). Bits-only: RED `TrackDelayed, 1 frames, block 0 (Full): the host planes`: the delay line runs over the poisoned slot and the kernel reads the undelayed block. |
| 927-7 | clause (c) dropped (an observed input is admitted) | `graph/src/runtime.rs` `source_plane_table` | gate 2; bits-only | RED `ObservedInput, 1 frames: the mode table`. Bits-only: RED `ObservedInput, 1 frames: every meter window is the copy arm's`: the input meter reads the poison (`2143357223`), while the host planes stay equal. |
| 927-8 | the Output op is handed no source planes (`planes: sources.filter(\|_\| false)`) | `graph/src/runtime.rs` `Runtime::execute` | gate 1, gate 2, rt10 | RED on all three (gate 1 `Plain, 1 frames, block 0 (Full)`, the poison; rt10 silence). The kernel test hands its own planes and passes. |
| 927-9 | the copy loop keeps every claim, in-place ones included | `graph/src/lib.rs` `GraphExecutor::new` | `cargo test -p graph --lib`; rt10 | No bit goes red. RED only on the counters: gate 1 `Plain, 1 frames, block 0: in place [copies, played reads, silent reads]` (`[64, 64, 0]` against `[0, 64, 0]`), gate 2 `SendTap` (`[64, 63, 0]` against `[1, 63, 0]`), #918's gate 1 (918-4's failure), and both rt10 tests (`[4000, 4000]` against `[0, 4000]`). |
| 927-10 | the Output reads its input's **position** as the claim index | `graph/src/runtime.rs` `OutputSources::input` | gate 1, gate 2, the kernel test, rt10 | GREEN on gate 1 and rt10: in the plain shape the Output's inputs come in claim order, so position and claim coincide. RED on gate 2 `SendTap, 1 frames, block 0 (Full)` (the bus route's input comes first and shifts every position) and on the kernel test, which lends under reversed claim indices for this reason (`[0, 0, 1]` against `[0, 1, 0]`). |
| 927-11 | the lent planes read swapped, `(right, left)` | `graph/src/runtime.rs` `OutputSources::input` | gate 1, gate 2, the kernel test, rt10 | RED on all four (gate 1 and 2 at block 0, the kernel test at its first case). |
| 927-12 | the fold records its retired routes in reverse (`producers` reversed) | `graph/src/runtime.rs` `output_route_fold` | gate 1, gate 2, rt10 | RED on all three at block 0: each claim is read at another claim's Output input, under that input's 2x2. The kernel test is bind-free and passes. |
| 927-13 | each pair's planes are formed through a `Vec` (`.collect()` in `route_pair`) | `graph/src/runtime.rs` `route_pair` | rt10; `scripts/check-realtime-policy.sh`; gates 1 and 2, the kernel test | RED on rt10 `1000 in-place blocks allocate and free nothing` (`(2000, 2000)`: one per pair per block), and on the realtime policy (`runtime.rs:796: .collect();`, `marked realtime forbidden-body predicate`). GREEN on the bit gates: no bit moves. |
