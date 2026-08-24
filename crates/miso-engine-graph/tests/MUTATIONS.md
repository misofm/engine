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

## Issue #100 additions — the pull-model arena, the persistent pool and bounded recovery

Same protocol: each row was applied, the named test was run, the failure recorded, the mutation
reverted. Rows marked *(design)* are mutations that the design itself makes impossible to express
without deleting a check; they are recorded with the check they delete.

| # | mutation | file | test | result |
|---|---|---|---|---|
| N1 | give two leases the same reserved arena buffer | `miso-engine-core` `realtime/disjoint.rs` (test fixture) | `disjoint::tests::overlapping_writes_are_rejected` | RED |
| N2 | let a lease read a producer of its own wave | `miso-engine-core` `realtime/disjoint.rs` (test fixture) | `disjoint::tests::a_read_from_the_same_wave_is_rejected` | RED |
| N3 | drop the `muted` redirection so a trapped producer is read directly | `miso-engine-core` `ArenaLeaseV1::effective` | `disjoint::tests::a_muted_read_is_silence_and_unmuting_restores_it` | RED |
| N4 | widen one lease's write set past its own buffers (bypassing the builder) | `miso-engine-core` `realtime/disjoint.rs` | `disjoint::tests::concurrent_leases_never_write_a_foreign_word` | RED |
| N5 | delete the hand-over block in `enter_block` | `miso-engine-core` `realtime/plan_exchange.rs` | `realtime::tests::enter_block_moves_the_executor_handover_and_returns_a_refused_one` | RED |
| N6 | add `unsafe` to a second `realtime/` file | `scripts/check-realtime-policy.sh` fixture | `scripts/test-realtime-policy.sh` (`unsafe-outside-disjoint-arena`) | RED |
| N7 | restore the unit-count split in place of the cost-weighted one | `miso-engine-native-scheduler` `partition_weighted_units_v1` | `lpt_balances_a_heavy_bank_against_scalar_tails` | RED |
| N8 | leave `block_open` true in `end_block` so the workers never park | `miso-engine-native-scheduler` `NativeSchedulerV1::end_block` | `workers_park_between_blocks_and_one_wake_brings_them_back` | RED |
| N9 | make `recovery_iterations` unbounded (`u64::MAX`) | `miso-engine-graph` bind budget | `a_late_worker_is_bounded_marked_dead_and_reaped_later` (wall-clock guard) | RED |
| N10 | never send the endpoints back in `WorkerLeaseV1::drop` | `miso-engine-native-scheduler` `platform/native.rs` | `a_released_lease_returns_to_its_pool` | RED |
| N11 | issue commands in ascending worker order, so a worker can wake a child before its command exists | `miso-engine-native-scheduler` `render_wave` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` (7 lanes, under load) | RED |
| N12 | a second `unpark` inside a marked region | `scripts/check-scheduler-policy.sh` fixture | `scripts/test-scheduler-policy.sh` (`second-coordinator-unpark`) | RED |
| N13 | `thread::park` outside `worker_loop` | `scripts/check-scheduler-policy.sh` fixture | `scripts/test-scheduler-policy.sh` (`park-outside-worker-loop`) | RED |
| N14 | request `fault-injection` from `[dependencies]` (and from a host) | `scripts/check-scheduler-policy.sh` fixture | `scripts/test-scheduler-policy.sh` (`fault-injection-in-dependencies`, `fault-injection-in-a-host`) | RED |
| N15 | drop the `graph.scheduler.lease` worker-count check at bind | `miso-engine-graph` `bind_native_optional_source_set` | `a_mismatched_worker_lease_is_refused_and_every_bind_input_returns` | RED |
| N16 | resolve a native op's inputs from the coloured buffer instead of its producing op | `miso-engine-graph` `runtime::op_producers` | `fifty_random_dag_sessions_render_bit_identically_in_both_executors` | RED |
| N17 | forget the `ARENA_BASE` offset in the sequential executor's output buffer | `miso-engine-graph` `GraphExecutor::new` | `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected` (builtins fixture PCM) | RED |
