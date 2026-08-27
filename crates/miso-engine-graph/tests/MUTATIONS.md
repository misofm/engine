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
| M12 | the slice PDC exchanges its two segments in the wrong order | `miso-engine-lane` `kernels::pdc_delay_block` | `compensation_delay_is_partition_invariant_and_matches_per_sample_reference` | RED |
| M4 | the route unfolds the gain and drops the fusion (`ll*l + lr*r`) | `miso-engine-lane` `kernels::mix2x2_block` | `route_applies_folded_gain_with_frozen_op_order` | RED |
| M9 | `mix2x2_block`'s scalar tail uses a different coefficient order than its vector body | `miso-engine-lane` `kernels::mix2x2_block` | `route_applies_folded_gain_with_frozen_op_order` | RED |
| M5 | the native executor stages an edge from the wrong producer | `src/runtime.rs` `build_native` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` | RED |
| M6 | the sequential executor reduces before staging its delayed edges | `src/runtime.rs` `execute_op` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` | RED |
| M8 | a bank member's inputs are gathered without their reduction | `src/runtime.rs` `Runtime::execute` | `level_major_w4_builtin_bank_is_analytic_for_three_blocks_in_both_executors` | RED |
| M7 | a tap's observers fire on the next op instead of the one that wrote the buffer | `src/runtime.rs` `taps_by_op` | `aliased_identity_stages_do_not_change_audio` | RED |
| M11 | aliasing is dropped: the internal boundaries keep their ops | `src/program.rs` `lower` | `aliased_identity_stages_do_not_change_audio` | RED |
| M13 | the elided-binding guard is removed, so a processor bound to an alias is dropped silently | `src/lib.rs` `PreparedGraphPlan::lowered` | `aliased_identity_stages_do_not_change_audio` | RED |
| M14 | lowering no longer refuses an unsorted `spec.nodes` | `src/program.rs` `lower` | `malformed_inputs_are_rejected_rather_than_lowered` | RED |
| M10 | `miso-engine-lane` dropped from the graph crate's expected dependency list | `scripts/check-graph-policy.sh` | `bash scripts/check-graph-policy.sh` | RED |

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
| N1 | accept a second writer for an arena buffer (delete the I1 check) | `miso-engine-core` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::overlapping_writes_are_rejected` | RED |
| N2 | accept a read of a producer in the reader's own wave (delete the I2 check) | `miso-engine-core` `ArenaLeaseSetBuilder::finish` | `disjoint::tests::a_read_from_the_same_wave_is_rejected` | RED |
| N3 | read a muted buffer directly instead of the silence slot | `miso-engine-core` `ArenaLeaseV1::effective` | `disjoint::tests::a_muted_read_is_silence_and_unmuting_restores_it` | RED |
| N4 | off-by-one in the arena's write address, so a lease writes its neighbour | `miso-engine-core` `ArenaLeaseV1::write` | `disjoint::tests::concurrent_leases_never_write_a_foreign_word` (`--release`) | RED |
| N5 | never take the executor hand-over at the block-boundary swap | `miso-engine-core` `RealtimePlanOwner::enter_block` | `realtime::tests::enter_block_moves_the_executor_handover_and_returns_a_refused_one` | RED |
| N6 | add `unsafe` to a second `realtime/` file | `scripts/check-realtime-policy.sh` fixture | `scripts/test-realtime-policy.sh` (`unsafe-outside-disjoint-arena`) | RED |
| N17 | forget the silence-slot offset in the sequential executor's output buffer | `miso-engine-graph` `GraphExecutor::new` | builtins-fixture `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected` | RED (observed as a real defect during this work, then fixed) |

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
mutation at a time over `cargo test -p miso-engine-graph -p miso-engine-graph-compiler
-p miso-engine-console-workload`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 218-1 | the first contributor accumulates instead of storing (`copy_from_slice` becomes `sum_into_block`) | `graph/src/runtime.rs` `ArenaMembers::fold_plane` | `runtime::tests::the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign` | RED (`+0.0` where `-0.0` is required; bits 0 against 0x8000_0000) |
| 218-2 | the fan-in-zero fill is skipped for *every* kind, not only a bound source | `graph/src/runtime.rs` `execute_op` | `runtime::tests::the_fan_in_zero_fill_is_dead_only_under_a_bound_source` | RED (an identity node with no contributors renders the previous block instead of silence) |
| 218-3 | the association proof keeps its length check and drops the element-wise comparison | `graph/src/runtime.rs` `route_fold` | `route_ids_ordered_against_the_cohorts_decline_the_route_fold` | RED, plus `a_leased_stage_meter_declines_the_merge_and_still_meters` and `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect` |
| 218-4 | the observer clause is dropped from `foldable_lane` | `graph/src/runtime.rs` | `the_folded_master_is_the_reductions_own_bits` | RED |
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
console fixtures (`tools/miso-engine-console-workload/tests/chain_shape.rs`) because the *production*
plan is what they are about and it is assembled there.

Driver: one mutation at a time, `cargo test -p miso-engine-console-workload --test chain_shape`,
tree restored (and `touch`ed) between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-G1 | `Runtime::arm_mono_collapse` arms every banked unit regardless of its lanes' tracks (`let armed = !tracks.is_empty();`), so the structural join is performed and ignored | `graph/src/runtime.rs` | `the_half_mono_cohort_banks_like_a_uniform_one` (and `the_collapse_fires_on_every_mono_cohort_and_no_other`) | RED — 2 failed. The half-mono row renders the *uniform mono* row's bits, because its odd tracks' right channels become the duplicated left ones. This is the failure that found the hole: the runtime witness is source-agnostic by construction and admits every lane of that row |

The join is where this row has to be cut, and an earlier draft cut it in the wrong place. Flipping
`BankChain::new`'s `collapse_source` default to `true` is **green** on this suite: every plan the
console builds is joined, and `arm_mono_collapse` writes the field on every chain, so the default is
overwritten before a block renders. That mutation is still a real gate -- it reds
`miso-engine-rack`'s `an_unarmed_chain_never_collapses`, where nothing performs the join -- but it is
a gate on the *default*, not on the join, and the two are separate claims. Both are listed, in the
crate whose test carries each.
| M2-G2 | the disengage copy is skipped (`slot.stage.desymmetrize()` becomes a no-op) | `rack/src/lib.rs` `disengage_collapse` | `a_run_that_stops_collapsing_renders_what_a_never_collapsed_run_renders` | RED |
| M2-G3 | the drain is moved back after the dispatch | `rack/src/lib.rs` `run` | `a_live_one_channel_retarget_disengages_on_the_block_it_lands` | RED — see the rack's M2-5 row for what the ordering protects |
