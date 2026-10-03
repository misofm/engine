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

## Historical issue #100 additions -- the pull-model arena, persistent pool and bounded recovery

Same protocol: each row below was applied to the working tree, the named test was run, the failure
was recorded, and the mutation was reverted in the same session. Every row was run on this branch
except where the "result" column says otherwise.

| # | mutation | file | test | result |
|---|---|---|---|---|
| N1 | accept a second writer for an arena buffer (delete the I1 check) | `engine` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::overlapping_writes_are_rejected` | RED |
| N2 | accept a read of a producer in the reader's own wave (delete the I2 check) | `engine` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::a_read_from_the_same_wave_is_rejected` | RED |
| N3 | read a muted buffer directly instead of the silence slot | `engine` `ArenaLease::effective` | `disjoint::tests::a_muted_read_is_silence_and_unmuting_restores_it` | RED |
| N4 | off-by-one in the arena's write address, so a lease writes its neighbour | `engine` `ArenaLease::write` | `disjoint::tests::concurrent_leases_never_write_a_foreign_word` (`--release`) | RED |
| N6 | add `unsafe` to a second `realtime/` file | `scripts/check-realtime-policy.sh` fixture | `scripts/test-realtime-policy.sh` (`unsafe-outside-disjoint-arena`) | RED |
| N17 | forget the silence-slot offset in the sequential executor's output buffer | `graph` `GraphExecutor::new` | builtins-fixture `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected` | RED (observed as a real defect during this work, then fixed) |

These rows preserve the original executed mutations. #1154 retires the multi-lease builder/API and
N1-N4's former owners: one non-cloneable `DisjointArena` now owns ordinary planar storage, and
exclusive borrowing prevents foreign access. Current direct refusal, many-borrow address/word,
transactional bank-shape and graph scatter gates qualify that single-owner API; these historical
rows are not a claim that the retired lease tests still run.

N7-N16 covered the native dependency-wave scheduler and were retired with it: the scheduler crate,
the `bind_native` family and the cross-executor 50-DAG oracle no longer exist, so none of those
mutations can be expressed. N5 (never take the executor hand-over at the block-boundary swap) was
retired by #1024, which deleted the hand-over itself and the test that guarded it. N6 still defends
the unsafe-file boundary, and N17 records the historical silence-slot offset correction.

Historically, N4 perturbed address arithmetic rather than the I1 check: the old builder rejected
overlapping write declarations, while its concurrent stress caught a neighbour's foreign tag.

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-5 | the `live.control.stage(..)` drain in `execute_op`'s `LiveControlEffect` arm never runs, so an admitted parameter reaches the effect a block late | `graph/src/runtime.rs` | `tests::a_live_control_parameter_command_applies_at_the_next_block_boundary` | RED (`every sample of the block that drains the command carries it`) |
| 140-6 | the `live.shunt.capture(..)` call is deleted, so a bypassed block renders the shunt's initial zeros instead of the latency-matched input | `graph/src/runtime.rs` | `tests::live_bypass_is_latency_preserving_and_reversible` | RED (`a bypassed block is the input delayed by exactly the declared latency`) |

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

**Retired (issue #957).** The Output route fold, its kernels and its tests were deleted: only a plan
with no bank at all reached them, and no shipped host builds one. None of these mutations can be
expressed any more. The rows stay as the record of what the gates proved while the code existed.

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
- "The console gate" is console-workload `chain_shape::the_plumbing_rows_output_fold_is_the_route_ops_own_bits`
  (deleted with the plumbing rows by issue #956; retired).
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

**Retired (issue #958).** The alias arm -- `is_builtin_stage`, `program::lower`'s `bindable`
parameter and the `listed` clause of the elision predicate -- was reverted once issue #959 had
deleted the builtins-less compile entry, the only producer of a plan that left a builtin stage out
of `required_bindings`. Gate 1 and gate 2's graph half were deleted with it, and gate 2's
graph-compiler half pins only the with-builtins shape since issue #964. The code these rows mutate
no longer exists. The rows stay as the record of what the gates proved while it did.

Each row was applied alone to `0973b805`, the three suites below were run with `--no-fail-fast`,
and the file was restored with `git checkout`. Suites: `cargo test -p graph --lib` (103 tests),
`cargo test -p graph-compiler --lib` (73), `cargo test -p console-workload --test chain_shape` (23).
Gate 1 is `tests::identity_bound_builtin_stages_alias_without_moving_a_bit`; gate 2 is
`program::tests::unlisted_builtin_stages_lower_as_aliases` with graph-compiler
`builtins_replace_only_the_three_internal_track_bindings`; gate 3 is console-workload
`the_plumbing_row_is_input_route_output_and_renders_the_base_bits` (deleted with the plumbing rows
by issue #956; retired).

| # | mutation | file | test | result |
|---|---|---|---|---|
| 925-1 | elide the three builtin stages whatever the bindable set says (`is_alias_candidate(id) \|\| is_builtin_stage(id)`) | `graph/src/program.rs` `lower_with` | the three suites | RED everywhere a stage is listed: graph 31 of 103 (gate 2 `PostInputBuiltins listed`: the listed stage lost its op; gate 1's listed arm; E9; every builtin-bank test, whose members `lowered()` now refuses as elided bindings), graph-compiler 23 of 73, chain_shape 22 of 23 (every with-builtins workload fails to bind: `sixty_four_track_console: console graph bindings`). Only the builtins-less plumbing test survives. |
| 925-2 | keep the post-input stage out of the alias predicate because it is dedicated (`is_builtin_stage(id) && !is_dedicated(id) && !listed[index]`): the dedicated stage keeps its op while the other two alias | `graph/src/program.rs` `lower_with` | the three suites | RED: gate 1 op count (`left: 10, right: 7`), gate 2 (`Input, PostInputBuiltins, Route, Output`), graph-compiler `builtins_replace_only_the_three_internal_track_bindings` and `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins` (builtins-less arm), gate 3 (unit count). Every other test GREEN. |
| 925-3 | alias only `PostMatrix` (`is_builtin_stage` names one stage) | `graph/src/program.rs` `is_builtin_stage` | the three suites | RED: gate 1 op count (`left: 13, right: 7`), gate 2 (`Input, PostInputBuiltins, PostFader, Route, Output`), the same two graph-compiler tests, gate 3. Every other test GREEN. |
| 925-4 | `lower_from_current_fields` hands `lower` an empty bindable set instead of `required_bindings` | `graph/src/lib.rs` `lower_from_current_fields` | the three suites | RED: graph 21 (E9, gate 1's listed arm, every builtin-bank plan: its members are listed and now elided, so bind refuses them), graph-compiler 23, chain_shape 22 of 23. The seam, not only the predicate, is what keeps a listed stage's op. |
| 925-5 | the builtins-less compile keeps listing the three stages (`=> true`) | `graph-compiler/src/compile.rs` `required_bindings` | the three suites | RED: graph-compiler `accepted_session_compiles_binds_and_renders_direct_route` (`required_bindings.len()`), `builtins_replace_only_the_three_internal_track_bindings` (the builtins-less op list), the merged-span test's builtins-less arm, and gate 3 (321 units). graph GREEN (no compiler there): the compile change is what moves the row. |

## Issue #927 — plain-strip sources read in place by the fused Output reduction

**Retired (issue #957).** The fused Output's in-place read -- `OutputSources`, `source_plane_table`
clauses (b') and (e), and the Output op's claim table -- was deleted with the fold it read through,
and so were gates 1 and 2, the kernel test and rt10's bankless test. Rows 927-6, 927-7 and 927-9
mutate code that stays (clause (a), clause (c) and the copy loop, all issue #918's); #918's rows
918-3, 918-6 and 918-4 prove the same mutations red on #918's gate 1. Gate 2's `DeadClaim` shape is
ported by issue #957 (row 957-4).

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

## Issue #936 — skip inert source-input units at render dispatch

Each row was applied alone to `3172be42` as an exact-text replacement whose match count (one) was
checked before writing, the suites were run with `--no-fail-fast`, and the file was restored with
`git checkout` before the next row. Host: `x86_64` (`.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`), debug profile. The tests these rows name:

- graph "gate 1" is `runtime::tests::an_unobserved_source_input_is_not_dispatched_and_moves_no_bit`,
  "gate 2" `runtime::tests::an_observed_source_input_stays_dispatched_and_meters_the_base_values`,
  "gate 3" `runtime::tests::a_delayed_claim_stays_dispatched_and_renders_the_base_bits`, "gate 5"
  `runtime::tests::the_metadata_charge_covers_the_ring_plans_executor_tables`, and "the metadata
  test" `tests::runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow`. Gates 1-3 pin
  the digest this fixture recorded on the base tree (`INERT_PRE_CHANGE`, `64b155d0`). Issue #957
  deleted the bankless ring fixture these rows ran on and ported the four gates onto issue #918's
  banked fixture (gate 5 is now `the_metadata_charge_covers_the_banked_source_plans_executor_tables`);
  the #957 rows below re-prove 936-1, 936-2 and 936-4 on the ported gates.
- console-workload "gate 1" is `tests::the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs`
  (then `the_driver_fed_plumbing_row_dispatches_only_its_output_unit`, re-homed by issue #956),
  "gate 5" `tests::the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables`, and
  "gate 4" is chain_shape `the_plumbing_row_is_input_route_output_and_renders_the_base_bits`
  (`BASE_DIGEST`; deleted with the plumbing rows by issue #956, retired) with the lib's
  `the_driver_fed_gain_pan_row_renders_the_bound_rows_bits` (then
  `the_driver_fed_plumbing_row_renders_the_bound_rows_bits`, re-homed by issue #956).
- "Bits-only" means the row was run a second time with three structural assertions of the #936
  helpers made vacuous in the same edit -- `render_ring_blocks`'s skipped-unit check,
  `assert_inert_shape`'s table equality and its meter count -- so that only the pinned digest of
  every host word and meter frame can go red.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 936-1 | the predicate ignores observation: both of its observation clauses dropped (`!identity.observed` and `op.observers.is_empty()`), the brief's "a predicate that ignores `observed`" | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib`; gate 2 bits-only | RED, graph 5 of 113: gate 2 `ObservedInput, 1 frames: skipped unit 5 is a plain unobserved source input`, #927's `a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits`, #918's `a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy`, `controlled_legacy_bind_refusal_preserves_callers_for_plain_and_source_retry` and `route_fold_preflight_returns_original_owners_and_sources_for_retry`. Bits-only: RED `ObservedInput: every host word and meter frame is the pre-change executor's` (`0xce6b3a3d49968c67`, which is the Plain shape's digest because the input meter never publishes, against `0x1f149a39d035ceea`). |
| 936-1a | only `!identity.observed` dropped | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | **GREEN, equivalent** (113 of 113). See 936-1b. |
| 936-1b | only `op.observers.is_empty()` dropped | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | **GREEN, equivalent** (113 of 113). `Runtime::new_with_observation_activation`, which every runtime passes through, sets `identity.observed = unit.has_observers()`, and for a plain op that is `!op.observers.is_empty()`. The two clauses the brief lists name one fact, so dropping either alone moves nothing. 936-1 is the brief's mutation in the form that is not equivalent. |
| 936-2 | a delayed claim is inert (`NodeKind::SourceInput \| NodeKind::TrackDelay { .. }`) | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib`; gate 3 bits-only | RED, graph 3 of 113: gate 3 `TrackDelayed, 1 frames: skipped unit 5 is a plain unobserved source input`, #927's gate 2, #918's gate 1. Bits-only: RED `TrackDelayed: every host word and meter frame is the pre-change executor's` (`0xce6b3a3d49968c67`, the undelayed Plain digest: the copied words reach the Output without their delay line, against `0xe8b0890ea031f463`). |
| 936-3 | the loop iterates the table in reverse (`active_units.iter().rev()`) | `graph/src/lib.rs` `GraphExecutor::render` | console-workload lib and chain_shape; `cargo test -p graph --lib` | RED on gate 4: chain_shape `the_plumbing_row_is_input_route_output_and_renders_the_base_bits` (`eliding identity stages moved a bit`, `BASE_DIGEST`; retired by issue #956) and `the_folded_master_is_the_reductions_own_bits`, and the lib's `the_driver_fed_plumbing_row_renders_the_bound_rows_bits` (now `the_driver_fed_gain_pan_row_renders_the_bound_rows_bits`). The driver-fed row dispatches only its Output op, so reversing it changes nothing; the bound row's reversal is what the equality catches. graph: 27 of 113 red, gates 2 and 3 and the rt9 source pin among them. |
| 936-4 | the loop dispatches every unit again (`let _ = &active_units; for unit in 0..runtime.units.len() {`) | `graph/src/lib.rs` `GraphExecutor::render` | `cargo test -p graph --lib`; console-workload lib and chain_shape | RED only on what counts dispatches: graph gates 1-3 (`Plain, 1 frames: units dispatched per block`, `[65; 16]` against `[1; 16]`) and the rt9 source pin (the pre-#936 header is refused); console gate 1 (`sixty_four_track_plumbing_ring, block 0: units dispatched`, 65 against 1; the row is now `sixty_four_track_gain_pan_ring` and the test `the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs`, issue #956). GREEN: chain_shape 24 of 24 and every digest. The change is class A, so a count is the only thing that can tell a skipped unit from a dispatched one. |
| 936-5a | `GraphExecutorWithoutObservationActivation` omits `active_units` | `graph/src/lib.rs` | `cargo test -p graph --lib` | RED, the metadata test only: `the observation owner delta is the runtime state's alone` (272 against 256). |
| 936-5b | `GraphExecutorWithoutSplitPairTable` omits `active_units` | `graph/src/lib.rs` | `cargo test -p graph --lib` | RED, the metadata test only: `the split owner delta is the runtime field's alone` (32 against 16). |
| 936-6a | the copied-claim table is charged nothing (`source_input_entry_bytes.checked_mul(0)`) | `graph/src/lib.rs` `checked_for_with_response_bindings` | `cargo test -p graph --lib`; console-workload lib | RED: the metadata test, graph gate 5 (`declined true: the charge (516, 0) covers the tables (4, 1024)`), console gate 5. |
| 936-6b | neither table is added to `total_bytes` | `graph/src/lib.rs` `checked_for_with_response_bindings` | `cargo test -p graph --lib`; console-workload lib | RED: the metadata test; console gate 5 `the charge grows by exactly the two tables` (15840 against 26100). |
| 936-7 | the loop collects the table into a `Vec` every block (`active_units.iter().copied().collect::<Vec<u32>>().into_iter()`) | `graph/src/lib.rs` `GraphExecutor::render` | rt10, rt1 and rt9 alloc tests (`--features test-support`); `scripts/check-realtime-policy.sh` | RED: rt10 2 of 2 (`1000 in-place blocks allocate and free nothing`), rt1 1 of 1, rt9 3 of 8 failed and then the binary aborted (SIGABRT). **GREEN on `check-realtime-policy.sh`**: its forbidden-body pattern matches `\.collect\(`, not the turbofish `.collect::<`. The counting-allocator tests are what catch it. This gap in the script is disclosed here, not fixed: the script is outside this brief's paths. |

## Issue #945 — read fold tiles without a stack copy

Each row was applied alone to `b8df5b52` as an exact-text replacement of
`core::array::from_fn(|row| rows[row])` in `tile_rows` (match count one, checked before writing),
the suites were run with `--no-fail-fast`, and the file was restored before the next row. Host:
`x86_64` (`.cargo/config.toml` pin `-C target-feature=+avx2,+fma`). The graph and chain_shape runs
are the debug profile in this worktree. "Digests" is the `digests` harness of
[`gain-pan-diagnosis-harnesses.patch`](https://github.com/misofm/engine/blob/5379e46ca3b349b9d277d642c008bb7a9643fb76/docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch)
(removed by #1031; every standing console
workload's 64-block output digest), built `--release` in a scratch copy with the patch applied and
the same edit made; it was never committed. The witnesses the amended brief names are graph
`runtime::tests::a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit` ("the resident
witness") and chain_shape `the_folded_master_is_the_reductions_own_bits`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 945-1 | the tile's rows from the wrong offset, `rows[(row + 1) % W]` (the brief's "`rows[i + 1]` wrapped"), at both widths | `graph/src/runtime.rs` `tile_rows` | `cargo test -p graph --lib`; chain_shape; digests | RED. graph 4 of 113: the resident witness (`Four, 4 frames, 1 cohort(s), store true: left master`), `a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy`, `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, `a_redirect_after_a_retired_route_lands_on_its_own_chain`. chain_shape 1 of 24: `the_folded_master_is_the_reductions_own_bits` (`nine_track_baseline: the folded master is not the reduction's bits`). Digests: 13 of 16 rows move (`nine_track_baseline` `e7c6ef01` -> `f39cdeaa`, `sixty_four_track_gain_pan_only` `01e465a7` -> `26fcd980`, and the ragged strip, console, stretch, eq, compressor, builtins, dispatch, console_legacy, eq_comp_simd1, console_mono and console_mono_dual rows). The three that stay are the rows that take no resident fold: `sixty_four_track_idle`, `sixty_four_track_plumbing_only` (`BASE_DIGEST`, which has no bank) and `sixty_four_track_console_half_mono`. |
| 945-1b | the same offset at `W = 4` only: `rows[if W == 4 { (row + 1) % W } else { row }]` | `graph/src/runtime.rs` `tile_rows` | `cargo test -p graph --lib`; chain_shape; digests | RED on graph 4 of 113, the same four as 945-1, the resident witness at `Four, 4 frames, 1 cohort(s), store true: left master`. **GREEN on chain_shape (24 of 24) and on all 16 digests**: a native x86-64-v3 bank is eight lanes wide, so no console workload reaches the `W = 4` instantiation on this host. `W = 4` is what the browser build ships, and the resident witness, which runs `BankWidth::Four` explicitly, is the gate that sees it. |
| 945-1c | the same offset at `W = 8` only | `graph/src/runtime.rs` `tile_rows` | `cargo test -p graph --lib`; chain_shape; digests | RED. graph 3 of 113: the resident witness at `Eight, 8 frames, 1 cohort(s), store true: left master`, `a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy`, `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit` (`a_redirect_after_a_retired_route_lands_on_its_own_chain` stays green, so it folds a four-lane bank). chain_shape as 945-1. Digests: the same 13 rows move to the same values as 945-1. |

Disclosed equivalent mutants:

* **Restoring the copy** (the pre-#945 zeroed tile and per-row `copy_from_slice`) moves no bit:
  that is the class-A claim itself. It is a codegen property, witnessed by the release `objdump`
  of `fold_resident` in the #945 spec's evidence (4 `memcpy` calls against 0), not by a test.
* **Dropping the `[..W * W]` slice** (the brief's prototype, `block.as_chunks::<W>()`) moves no bit
  either, because the caller hands exactly `W * W` words. The length of `rows` is then not a
  constant, so the release `W = 8` tile loop keeps a chain of five row-bounds compares, where the
  slice leaves one (`cmp $0x40`) per tile. Also a codegen property, not a test.

## Issue #937 — Output inputs resolved in groups of eight, and the tighter pair kernel

**Retired (issue #957).** The grouped Output kernel, `GraphSourcePlanes::played_planes_group` and
their tests were deleted with issue #926's fold. None of these mutations can be expressed any more.

Each row was applied alone to `40c62101` as an exact-text replacement (match count one, checked
before writing), the suites were run with `--no-fail-fast`, and the file was restored with
`git checkout` before the next row. Host: `x86_64` (`.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`), debug profile. The tests these rows name:

- "Gate 1" is `runtime::tests::a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit`
  (#926's kernel oracle, whose sweep this issue extends to fan-in 257). It visits `f32`, then
  `Simd4`, then `Simd8`, each over frames {1, 3, 7, 8, 13, 16, 33, 64} and fan-in 2..=19, 64, 257,
  so its first failure names the narrowest width and the smallest case a row reaches.
- "The lent test" is `runtime::tests::a_route_reduction_reads_each_lent_input_as_the_copys_words`,
  and "the group test" is this issue's `runtime::tests::a_source_sets_group_call_is_its_per_claim_call`.
- "#927's gates 1 and 2" are `a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits`
  and `a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits`; "#936's gates" are
  `an_unobserved_source_input_is_not_dispatched_and_moves_no_bit`,
  `an_observed_source_input_stays_dispatched_and_meters_the_base_values` and
  `a_delayed_claim_stays_dispatched_and_renders_the_base_bits`.
- "The console counter gate" is console-workload `tests::the_driver_fed_gain_pan_row_renders_the_bound_rows_bits`
  (then `the_driver_fed_plumbing_row_renders_the_bound_rows_bits`, re-homed by issue #956)
  (`[0, claims * BLOCKS, 0]`); "the console digests" are chain_shape
  `the_plumbing_row_is_input_route_output_and_renders_the_base_bits` (`BASE_DIGEST`) and
  `the_plumbing_rows_output_fold_is_the_route_ops_own_bits`, both deleted with the plumbing rows by
  issue #956 (retired).
- "Rule 3" is `scripts/check-web-audioworklet-callgraph.py --kernel-shape --kernel-pattern
  '4wide6f32x[48]' --kernel-min 11` over `wasm-objdump -d` of the simd128 AudioWorklet module,
  built with `scripts/build-web-audioworklet.sh`'s own cargo line and flags.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 937-1 | reassociate a later pair as `load + (m_2k + m_(2k+1))` (both planes of `route_pair_vectors`' accumulate loop) | `graph/src/runtime.rs` `route_pair_vectors` | `cargo test -p graph --lib --features test-support`; `cargo test -p console-workload` | RED. graph 7 of 114: gate 1 `width 1, 1 frames, fan-in 7, -0.0 false: right` (`1118879272` against `1118879273`; fan-ins 4 to 6 are the first with a later pair, and their one-frame sums happen to round alike), #926's `an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit` (`Plain, fan-in 64, 13 frames, block 0: the host planes`), #927's gates 1 and 2 (`the declined arm is the pre-change executor`) and #936's three gates (`every host word and meter frame is the pre-change executor's`). Console: both console digests (`9b7337c3...` against `BASE_DIGEST` `57535244...`). GREEN: the lent test and the console counter gate, which compare the kernel with itself. |
| 937-2 | drop the played-read counter call (`Some(played) => played,` in the provided body) | `graph/src/lib.rs` `GraphSourcePlanes::played_planes_group` | `cargo test -p graph --features test-support`; console-workload `--lib`; host-core `--test source_in_place` | RED. graph 4 of 114: the group test (`[0, 0, 3]` against `[0, 2, 3]`), the lent test (`width 1, 1 frames, fan-in 2`, `[0, 0, 0]` against `[0, 1, 0]`), #927's gate 1 (`Plain, 1 frames, block 0: in place`, `[0, 0, 0]` against `[0, 64, 0]`) and gate 2 (`SendTap`, `[1, 0, 0]` against `[1, 63, 0]`). Console counter gate RED (`[0, 0, 0]` against `[0, 4096, 0]`). **GREEN on host-core `source_in_place`**: it pins #918's bank gathers, which count at `ArenaMembers::plane`, the other counter site, and which this issue does not touch; it is listed in gate 2 as a must-stay-green test, not as this row's witness. rt10, rt1 and rt9 green (no bit moves). |
| 937-2b | drop the silent-read counter call (`None => silence,`) | `graph/src/lib.rs` `GraphSourcePlanes::played_planes_group` | `cargo test -p graph --features test-support`; console-workload `--lib` | RED. graph 4 of 114: the lent test (`width 1, 1 frames, fan-in 9`, the first fan-in with a silent input: `[0, 5, 0]` against `[0, 5, 1]`), the group test (`[0, 2, 0]` against `[0, 2, 3]`), #927's gates 1 and 2 at the block-3 underrun (`[0, 63, 0]` against `[0, 63, 1]`). GREEN on the console counter gate: the ring row never underruns. |
| 937-3 | resolve a group of eight but reduce only its first seven (`&planes[..7]`, `&table[..7]` when the group is full) | `graph/src/runtime.rs` `route_reduce` | `cargo test -p graph --features test-support`; `cargo test -p console-workload` | RED. graph 7 of 114: gate 1 at `width 1, 1 frames, fan-in 8, -0.0 false: left` (`1254604654` against `1254604426`), the first fan-in with a full group; #926's fold gate, #927's gates 1 and 2 and #936's three gates as in 937-1. Console: both console digests (`133d291a...`). GREEN: the lent test and the console counter gate (kernel against itself; every lent claim is still resolved and counted). |
| 937-4 | inline the tail into the vector kernel (`route_tail` marked `#[inline(always)]`), #920's failure shape | `graph/src/runtime.rs` `route_tail` | rule 3 | RED: `FAIL kernel ...route_group...4wide6f32x4...: vector=38 scalar=122`. The committed tree reads `vector=38 scalar=0`, so rule 3 does inspect `route_group<f32x4>`. |

## Issue #943 — the banked sample-peak pass

The witnesses are graph-compiler gate G3,
`tests::post_matrix_peak_meters_merge_one_bank_pass_per_cohort_and_publish_the_scalar_frames`
(64-track intended fixture, a `SAMPLE_PEAK` `PostMatrix` meter per track, both selected deliveries,
periods 512 and 300, the pass on against `test_only_set_bank_sample_peak_declined(true)`; its
controls are ALL-metric meters and a plan bound with an observation activation), and this crate's
source scan `runtime::tests::resident_meter_entry_has_one_final_output_dispatch_and_admission_control`,
whose own control rows now include the pass forced on (`let peaks = if true {`), the peak withheld
from the member call, the lane index shifted, the planes swapped, and the whole-block borrow
replaced. Each row below was applied alone to `1975fc44` as an exact-text replacement (match count
one), run in dev, and restored.

| # | mutation | G3 | source scan |
| --- | --- | --- | --- |
| G-1 | `row.sample_peak = unit.accepts_sample_peak();` becomes `row.sample_peak = true;` (attempt 2's form: `UnitObservation::of(unit.has_observers(), true)`, same result) | RED at the ALL control: `ALL meters must not make a bank run the pass` (`192` against `0`). PCM and every frame stay equal: the counter is the only witness | GREEN (the flag's derivation is not pinned there) |
| G-2 | the final lane's hand-off reads lane `lane ^ 1` of both planes | RED: `between_render_calls false period 512: meter 1 window 0` (left and right peaks are lane 1's) | RED: `valid(source)` |
| G-3 | the right plane's pass reads the left plane (`meter_sample_peak_block::<L>(left, ..)` into `peaks[1]`) | RED: `meter 1 window 0`, right `sample_peak` bits `543241058` against `994846919` (needs the L-differs-from-R window) | GREEN (`bank_sample_peak`'s body is not pinned there) |

### Attempt 2: the packed identity byte and the four-lane dispatch

Attempt 1's `sample_peak: bool` was a fifth one-byte field in `UnitIdentity`, which fits the
padding on a 64-bit target but grew the row from 20 to 24 bytes on wasm32 (Sol, attempt 1). The
flag now shares `observed`'s byte as `UnitObservation { Unobserved, Observed, ObservedWithPeak }`,
and the layout pin is a `const` assertion in production code (`size_of` and `align_of` of
`UnitIdentity` equal those of `UnitIdentityWithoutFlags`), so every target build checks it. G3 now
also runs at `Backend::Simd4`, the browser's and NEON's bank width. Rows applied alone to the
attempt 2 tree and restored:

| # | mutation | result |
| --- | --- | --- |
| S-7 | `BankWidth::Four => planes::<lane::Simd8>(left, right)` (Sol's row) | RED on G3's `Simd4` arm: `Simd4 between_render_calls false period 512: meter 6 window 0` (left peak bits `992536920` against `994476435`). GREEN on G4 (host width) and the source scan, as before; it was GREEN on every committed gate in attempt 1 |
| O-1 | `UnitObservation::of`'s `(true, true)` arm returns `Observed` (the peak bit lost in the packing) | RED: G3 `Simd8 ... period 512: one pass per cohort` (`0` against `192`); G4 (`0` against `8000`) |
| O-2 | `UnitObservation::observed` becomes `matches!(self, Self::Observed)` (a peak unit's observers skipped) | RED: G3 `every meter publishes every whole window` (`0` against `384`). GREEN on `-p graph --features test-support`: no graph test binds a peak-accepting observer, so G3 is the witness |
| L-A | the attempt 1 layout restored: a separate `sample_peak_flag: bool` after `source_lanes` | `cargo check -p graph` GREEN natively (32 bytes either way); `cargo check --target wasm32-unknown-unknown -p graph` RED: `evaluation panicked: a UnitIdentity flag no longer fits the row's padding` |
| L-B | `#[repr(u16)]` on `UnitObservation` | the same: native check GREEN, wasm32 check RED with the same message |

## Issue #950 — the banked full meter pass

The witnesses are graph-compiler gate M3,
`tests::post_matrix_all_meters_run_one_full_bank_pass_per_cohort_and_publish_the_scalar_frames`
(64-track intended fixture, an `ALL` `PostMatrix` meter per track, both selected deliveries,
`Simd8` and `Simd4`, periods 512 and 300, plus the `console_meters` path through
`compile_console_model_with_builtins`, each against `test_only_set_bank_meter_declined(true)`); its
controls `tests::the_full_meter_pass_stays_off_where_no_meter_can_commit_it` (mixed `SAMPLE_PEAK`
and `ALL`, issue #943's `SAMPLE_PEAK` fixture, an eight-frame hold, an activation-bound plan); the
failure boundary `tests::an_observer_failing_mid_bank_leaves_every_later_meter_as_the_declined_arm_does`;
and this crate's source scan
`runtime::tests::resident_meter_entry_has_one_final_output_dispatch_and_admission_control`, whose
control rows now include the full pass forced on, #943's pass run beside it, the meter lane withheld
or shifted, the seeds not read, and the pass handed one plane twice. Each row was applied alone to
`326607ce` as an exact-text replacement (match count one), run in dev, and restored with `git
checkout`; G-5 was rerun on `9696f74d`.

| # | mutation | result |
| --- | --- | --- |
| G-1 | the final lane's hand-off reads lane `lane ^ 1` (`meters.lane(lane ^ 1, seeds)`) | RED: M3 `Simd8 between_render_calls false period 512: meter 1 window 0` (lane 1's energies and peaks); the source scan |
| G-2 | the right plane's pass reads the left plane (`plane::<L>(left, frames, right_seeds)`) | RED: M3 `meter 1 window 0`, the right channel's words are the left's (needs the L-differs-from-R window). GREEN on the source scan (`bank_meter_pass`'s body is not pinned there) |
| G-3 | issue #943's pass also runs where the full pass ran (`let peaks = if sample_peak && eligible {`) | RED: the mixed control, `mixed: no sample-peak pass beside it` (`192` against `0`) |
| G-4 | `MeterObserver::accepts_banked_meter` also accepts `SAMPLE_PEAK`-only meters | RED: the mixed control, `mixed: the peak meters merge its peak` (`0` against `768`): the peak meters commit through the banked path instead. GREEN on #943's `SAMPLE_PEAK` fixture control (checked with the mixed control skipped): such a meter has no energy, so it answers no seed and no pass runs (amendment 2). The brief's "M3 P1-fixture control" witness predates that amendment; the mixed control is the witness |
| G-5 | the member loop keeps observing after an observer fails and returns the first error at the end | RED: the failure boundary, `ch00 and ch01 commit the failing block and no later meter does` (`8` against `2`: `ch02` to `ch07` observed the failing block) |
| A-5 | the pass runs when no lane answered a seed (amendment 5: the `bank_meter_seeds` result ignored) | RED: M3 `Simd8 between_render_calls false period 300: one pass per cohort per block a window holds` (`192` against `112`); the source scan |
| K-1 | the kernel's energy is a zero-seeded partial plus the seed (`crates/lane`) | RED: M3 `Simd8 between_render_calls false period 512: meter 1 window 1`, the energy and `rms` a few ulps off |

K-6 (`c > 1.0` for the clipped count) is GREEN on M3: no post-matrix word of the fixture is exactly
`1.0`. M1 and M2 are its witnesses.

The failure boundary also pins the realtime claim #714's clause rests on: the pass only reads the
later meters (their seeds, through `&self`), so when `ch01`'s second observer fails on block 5,
`ch02`'s meter has neither observed nor committed that block and opens its next window at a
discontinuity, exactly as in the declined arm; both arms' frames are equal by bits.

### Issue #950 attempt 2: the `2^24` decline and a schedule-independent failure boundary

Two gates changed, with no production change. `runtime::tests::the_full_meter_pass_declines_a_block_whose_counts_would_not_be_exact`
hands `bank_meter_pass` `2^24 + 1` lazily zero-mapped frames at `Four` and `Eight` (5.6 MB peak RSS:
the pass refuses on the slice length before it reads a word) and checks a three-frame block is
computed. The failure boundary now reads the tracks' observation order from a probe plan with an
order recorder on every post-matrix node, and requires exactly the meters up to `ch01` in that
order to see and commit the failing block, at the host width and `Simd4`, instead of assuming bank
0 is observed first. Rows applied alone to `958c7066` and restored:

| # | mutation | result |
| --- | --- | --- |
| D-1 | the `frames > BANK_METER_MAX_FRAMES` term deleted from `bank_meter_pass` | RED: `Four: 2^24 + 1 frames decline the pass` (the kernel then reads the zero pages: 17.8 s in dev, 6 MB RSS) |
| G-5 | the member loop keeps observing after a failure and returns the first error at the end | RED on the rewritten gate: `Simd8 failure boundary: exactly the meters observed before the failing observer saw block 5`, handles `[1, 2, 3, 4, 5, 6, 7, 8]` against `[1, 2]` |
| S-2 | `commit_banked`'s window check deleted (`crates/builtins`) | RED on the new mixed-bank gate (`crates/builtins/tests/MUTATIONS.md`) |

## Issue #957 — #936's gates on #918's banked fixture, and the ported dead claim

Issue #957 deleted the bankless ring fixture of #927 and #936 and ported #936's gates onto #918's
banked source fixture (`W4 x 6`, track `K = 5`, frames `{10, 13, 16, 128}`, sixteen blocks). Each
row was applied alone to `bf3bacab` as exact-text replacements whose match counts (one) were
checked before writing, `cargo test --locked -p graph --lib --no-fail-fast` was run (108 tests),
and `crates/graph/src` was restored with `git checkout` before the next row. Host: `x86_64`
(`.cargo/config.toml` pin `-C target-feature=+avx2,+fma`), debug profile. The tests these rows name:

- "Gate 1" is `runtime::tests::an_unobserved_source_input_is_not_dispatched_and_moves_no_bit`,
  "gate 2" `…::an_observed_source_input_stays_dispatched_and_meters_the_base_values` (shapes
  `ObservedInput`, then `ObservedAlias`), "gate 3" `…::a_delayed_claim_stays_dispatched_and_renders_the_base_bits`,
  "gate 5" `…::the_metadata_charge_covers_the_banked_source_plans_executor_tables`, and "the dead
  claim test" `…::a_claim_nothing_reads_is_bound_in_place_and_its_recoloured_slot_moves_no_bit`.
  Gates 1-3 pin digests recorded on `64b155d0` (`INERT_PRE_CHANGE`); the dead claim test pins
  `DEAD_CLAIM_PRE_CHANGE`, recorded there too.
- "Bits-only" means the row was run a second time with the ported gates' structural assertions
  made vacuous in the same edit -- `render_inert_blocks`'s skipped-unit check, `assert_inert_shape`'s
  table equality, per-block dispatch count and meter count, gate 2's `K`-meter count and gate 3's
  dispatched-input check -- so that only a pinned digest can go red. For 957-4 it is the dead
  claim test's mode table and in-place counters instead.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 957-1 | dispatch every `SourceInput` unit: the inert predicate never names one (`… && op.observers.is_empty() && false`) | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib`; bits-only | RED, 4 of 108: gates 1, 2, 3 at their tables (`Plain, 10 frames: the dispatched-unit table`, all 15 units against the 9 non-input units; `ObservedInput` and `TrackDelayed` against 10) and gate 5 (`declined false`, 15 against 9). Bits-only: gates 1-3 GREEN, their digests unmoved (class A: a dispatched inert unit writes nothing), and only gate 5's table count is RED. The count is the only thing that can tell a skipped unit from a dispatched one. |
| 957-1b | 936-4 re-run: the loop dispatches every unit again (`let _ = &active_units; for unit in 0..runtime.units.len() {`) | `graph/src/lib.rs` `GraphExecutor::render` | `cargo test -p graph --lib` | RED, 4 of 108: gates 1-3 at `units dispatched per block` (`[15; 16]` against `[9; 16]`, and `[10; 16]` for `ObservedInput` and `TrackDelayed`), and the rt9 source pin (`sole resident call must remain behind graph admission`). |
| 957-2 | skip an observed `Input` unit: both observation clauses dropped (936-1's form) | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib`; bits-only | RED, 4 of 108: gate 2 (`observed: Some(5), 10 frames: skipped unit 5 is a plain unobserved source input`), #918's gate 1 (`the copy arm is the pre-change executor`: its observed claim's meter goes silent), `controlled_legacy_bind_refusal_preserves_callers_for_plain_and_source_retry` and `route_fold_preflight_returns_original_owners_and_sources_for_retry`. Bits-only: RED `ObservedInput: every host word and meter frame is the pre-change executor's` (`0x3adf_c3ee_a1b4_75ab`, the `Plain` digest, because `K`'s input meter never publishes, against `0x51c6_c3cf_4cdc_5509`). |
| 957-2c | 957-2 bits-only, with gate 2 visiting `ObservedAlias` alone (the `PostSimd1` alias meter, amendment 1) | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | RED `ObservedAlias: every host word and meter frame is the pre-change executor's` (`0x3adf_c3ee_a1b4_75ab` against `0x51c6_c3cf_4cdc_5509`): the alias's meter binds to `K`'s input op, and skipping that unit silences it. The other three failures are 957-2's. |
| 957-3 | skip a `TrackDelay` unit (`NodeKind::SourceInput \| NodeKind::TrackDelay { .. }`, 936-2's form) | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib`; bits-only | RED, 2 of 108: gate 3 (`delayed: Some(5), 10 frames: skipped unit 5 is a plain unobserved source input`) and #918's gate 1 (`the copy arm is the pre-change executor`). Bits-only: RED `TrackDelayed: every host word and meter frame is the pre-change executor's` (`0x3adf_c3ee_a1b4_75ab`, the undelayed `Plain` digest: the copied words reach the gather without their delay line, against `0xd1eb_c3d4_2404_cb33`). |
| 957-4 | clause (b) refuses a claim with no reader (`… && !readers[op].is_empty()`) | `graph/src/runtime.rs` `source_plane_table` | `cargo test -p graph --lib`; bits-only | RED, 1 of 108: the dead claim test (`dead: true, 10 frames: the mode table`, `[…, false]` against `[…, true]`). Bits-only: **GREEN** (108 of 108), as expected: the copy writes the dead claim's words into a slot nothing reads before a later value overwrites it, so copied or not, the bits are the same; the mode table and the counters are what see the zero-reader arm. (Corrected: this row first said no bank on the fixture gathers the recoloured slot. That was wrong; the follow-up below builds that gather and proves it red at the bits, 957-5.) |

### Follow-up: Sol's finding 1 (the dead claim's gather) and finding 2 (the one- and seven-frame quanta)

Sol's verdict found that the ported dead claim test could not see its own hazard: with one bank
stage nothing gathers the dead claim's recoloured slot, so a gather that ignores the lane marking
stayed green on it. The test now runs `W4 x 6` with two bank stages (`PostInputBuiltins` then
`PostMatrix`) and checks on the bound runtime, in both redirect arms, that a bank after the dead
input gathers the dead slot on an unmarked lane. The fixture also caps `PLAYED_SCRIPT`'s short block
one frame below the quantum and keeps a one-frame block's noise word, so the ported gates render
#936's own quanta `{1, 7, 16, 128}` again (#918's gate 1 renders 13 and 16, where neither change
moves a word). Every pin was re-recorded on `64b155d0`. Rows applied alone to the tree of the
commit that adds this section, as above, `crates/graph/src` restored after each; bits-only as above, and
for 957-5 and 957-5b the dead claim test's mode table and in-place counters made vacuous:

| # | mutation | file | test | result |
|---|---|---|---|---|
| 957-1 | as above | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | RED, 4 of 108: gates 1, 2, 3 and 5 at their tables (`TrackDelayed, 1 frames: the dispatched-unit table`, all 15 units against 10). |
| 957-2 | as above, bits-only | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | RED: gate 2 at `ObservedInput: every host word and meter frame is the pre-change executor's` (`0x59ac_7ce2_0913_f0e1`, the `Plain` digest, against `0x57be_32ec_07e5_88cd`), #918's gate 1 and the two bind-retry tests. |
| 957-3 | as above, bits-only | `graph/src/runtime.rs` `Runtime::unit_inert` | `cargo test -p graph --lib` | RED: gate 3 at `TrackDelayed: every host word and meter frame is the pre-change executor's` (`0x59ac_7ce2_0913_f0e1` against `0x5669_2e20_31f4_30b1`), and #918's gate 1. |
| 957-4 | as above | `graph/src/runtime.rs` `source_plane_table` | `cargo test -p graph --lib`; bits-only | RED, 1 of 108: the dead claim test (`dead: true, 1 frames: the mode table`). Bits-only: GREEN (108 of 108), for 957-4's corrected reason. |
| 957-5 | the gather ignores `source_lanes`: `SourceGather::claim` serves any lane the table's claim (`if lane >= 8 {`, 918-5's form) | `graph/src/runtime.rs` `SourceGather::claim` | `cargo test -p graph --lib`; bits-only | RED, 2 of 108: the dead claim test at the bits (`dead: true, 1 frames, redirects bound, block 0 (Full): the master is the copy arm's`), and #918's gate 1 (918-5's failure). Bits-only: the same two, the same block. Attempt 1's one-stage dead claim test stayed GREEN under this row (Sol's verdict). |
| 957-5b | the leak limited to the dead claim: an unmarked lane is served the table's claim only when that claim is 6, the dead claim's index | `graph/src/runtime.rs` `SourceGather::claim` | `cargo test -p graph --lib`; bits-only | RED, 2 of 108: the dead claim test at the bits (`1 frames, block 0 (Full)`), and #918's gate 1 at its `W8 x 8` scalar-fader shape, whose claim 6 is live. Bits-only: the same. |

## Issue #1201 -- a submix's delay runs on its summed input

Applied to the attempt-1 tree of *Delay a submix strip's summed input*, `crates/graph/src` restored
after the run. x86-64 AVX2 host, debug profile.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 1201-2 | the `SumDelay` line runs in `execute_op`'s early-return path, before the reduction, and returns (the `TrackDelay` position) | `graph/src/runtime.rs` `execute_op` | `cargo test -p graph --lib a_bus_delay_runs_on_the_sum_after_the_reduction` | RED: `lane 0 sample 5: 0 != 5398.029` -- the reduction never writes the bus buffer, so the delayed sum is never there. |

## Issue #1217 -- an inactive route is skipped in its destination's sum

Applied one at a time to the attempt-1 tree of *Skip an inactive route in its destination's sum*,
each source file restored after its run. x86-64 AVX2 host, debug profile. The render gates are
`host-core/tests/route_mute.rs` (`cargo test -p host-core --features
host-core/test-support,graph/test-support --test route_mute`); gate 4 is
`graph-compiler/tests/route_activity.rs`.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 1217-1 | a silenced undelayed route stays active (bind never clears its bit): it mixes `[+0.0; 4]` | `graph/src/runtime.rs` `route_activity` | gates 1 and 2 | RED: `an_inactive_route_is_neither_mixed_nor_read` (`seed 0, Bus, r0 muted, plane 0: sample 16: -0.0 != 0.0`), `the_first_route_in_id_order_owns_the_store` (`r0 muted, r1 open at -0.0: plane 0 frame 1: -0.0, expected 0.0`), and the in-place test (`frame 0: -0.0, expected 0.0`). |
| 1217-2 | an inactive route's op still runs (D4's early return removed; the destination still skips it) | `graph/src/runtime.rs` `execute_op` | gates 1 and 2 | RED: `seed 0, Bus, r0 muted: the muted route's op mixed`, `t-c mixed`, and gate 2's mix counts. |
| 1217-3 | the first *active* input owns the store (no `+0.0` store for an inactive first input) | `graph/src/runtime.rs` `route_segments` | gates 1 and 2 | RED: `seed 0, Bus, r0 muted, plane 0: sample 0: -0.0 != 0.0`; `r0 muted, r1 open at -0.0: plane 0 frame 0: -0.0, expected 0.0`; the in-place test (`frame 0: -0.15693776, expected 0.0`). |
| 1217-4 | an inactive sole input leaves the destination's in-place buffer alone (the `+0.0` store only when `count > 1`) | `graph/src/runtime.rs` `route_segments` | gate 1, in place | RED, 1 of 7: `the muted sole route's bus: plane 0 frame 0: -0.15693776, expected 0.0` -- the raw `post_pan` tap. |
| 1217-5 | an active opening run is fill-`+0.0`-then-add instead of today's reduction | `graph/src/runtime.rs` `reduce_gated` | gates 1 and 2 | RED: `r0 open at -0.0, r1 and r2 muted: plane 0 frame 0: 0.0, expected -0.0`; `seed 2, Bus, r1 muted, plane 0: sample 0: 0.0 != -0.0`. |
| 1217-6 | the host-master form's later run stores instead of adding | `graph/src/runtime.rs` `reduce_gated_into` | gate 1 | RED, 1 of 7: `seed 0, Output, r0 muted, plane 0: sample 0: -0.0 != 0.0`. |
| 1217-7 | a delayed muted route goes inactive (`&& input.delay.is_none()` dropped) | `graph/src/runtime.rs` `route_activity` | gate 3 | Attempt 1: RED, 1 of 7, by the counter only (`a_muted_delayed_route_stays_active`, 0 mixes against 8). Attempt 2: also RED **on audio**, `a_muted_delayed_routes_line_carries_its_zero_mix` (`plane 0: e's input, d-e's delayed zero mix alone: sample 486: 0.0 != -0.0`), and #1218's `a_fully_follow_muted_delayed_send_stays_active` by its counter. |
| 1217-8 | a route destination copies its route inputs to a `Vec` per block | `graph/src/runtime.rs` `execute_op` | gate 5 | RED: `route_activity_renders_without_allocating` aborts (`SIGABRT`) on the armed render audit. |
| 1217-9 | bind builds the table whatever the gates | `graph/src/runtime.rs` `build_sequential`, `route_activity` | gate 4 | RED: `a plan without a silencing route builds no route-activity table`. |
| 1217-10 | the estimate charges no table | `graph-compiler/src/estimate.rs` `resource_estimate` | gate 4 | RED: `graph_metadata_bytes grew by 0, the bound plan by 237`. |
| 1217-11 | the host-master form stores nothing for an inactive first input (`SumSegment::Zero => {}` in `reduce_gated_into`) | `graph/src/runtime.rs` `reduce_gated_into` | gate 1 (attempt 2, host planes pre-filled with `HOST_SENTINEL = 7.0`) | RED: `an_inactive_route_is_neither_mixed_nor_read` (`seed 0, Output, r0 muted, plane 0: sample 0: 7.0 != 0.0`) and `a_destination_whose_every_route_is_muted_renders_positive_zero` (`Output, 2 muted routes: plane 0 frame 0: 7.0, expected 0.0`). GREEN on the whole file with the sentinel set back to `0.0`, which is the attempt-1 hole. |
| 1217-12 | the host-master form skips its sum when every route input is inactive (an early `return` in `reduce_gated_into`) | `graph/src/runtime.rs` `reduce_gated_into` | gate 1, every contributor muted | RED, 1 of 12: `a_destination_whose_every_route_is_muted_renders_positive_zero` (`Output, 2 muted routes: plane 0 frame 0: 7.0, expected 0.0`). |
| 1217-13 | the estimate charges twice the table's bound (`route_activity_bytes * 2`) | `graph-compiler/src/estimate.rs` `resource_estimate` | #1216 gate 5, tightened (attempt 2, NIT-1) | RED: `a_muted_route_seals_one_route_mute_row_after_its_transform` (`the estimate moves by the route-activity charge (545 bytes) and nothing else (route:eq0-main)`); gate 4's lower-bound check stays green. |
