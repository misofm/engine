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
