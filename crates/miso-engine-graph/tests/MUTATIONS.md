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
