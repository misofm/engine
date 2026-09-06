# #443 Sol attempt 2 final evidence inventory

Reviewed pushed source head `85108496f235c9e76c1937e308c36cc3bdeaa96f`.

## New evidence satisfied in Sol2

- `scalar_factory_checks_both_exact_owners_and_policies_without_consuming_state`: both exact concrete owners, both immutable delivery policies, malicious wrong owners, no factory invocation, and returned owner state/queues.
- `scalar_invalid_envelope_leaves_the_later_matrix_queue_untouched`: fader drain and envelope validation precede matrix queue consumption.
- `scalar_owner_resource_is_independent_two_boxes_plus_two_pointer_outer`: retained fader/matrix boxes, exact two-pointer outer, serialized/Concurrent totals, no-console zero, and bank-selected zero.
- `synthetic_distinct_matrix_destination_is_the_scalar_pair_identity_decline`: exact same-output/single-undelayed-input/in-place guard plus labeled synthetic distinct-destination original source/destination/error/call/remaining-record effects.
- `live_scalar_owner_bytes_are_published_and_capped_before_binding`: actual scalar controlled/no-console resource delta across graph metadata, incremental plan and session-plus-plan; zero builtin banks; exact and one-below graph/plan/largest caps; published report equality; transactional ownership return.

## Existing regression evidence only

These unchanged fixtures do not fulfill the required new scalar proofs: `serialized_live_fader_matrix_is_selected_by_the_bound_render_path`, `a_post_fader_meter_declines_its_cohort_while_the_tail_pair_still_fuses`, `serialized_send_reader_declines_crossing_fader_while_pcm_matches_separate`, `serialized_alias_observer_is_the_decline_boundary`, `actual_queued_graph_phases_allocate_and_free_nothing`, and `acknowledged_pair_render_records_the_same_live_dispatch`. `crates/builtins-compiler/tests/allocation_tracker.rs` and `hosts/host-web/src/tests.rs` were unchanged in Sol2. `composite_live_sequence_matches_original_owners_and_discriminates_both_branches`, `composite_raw_record_errors_preserve_the_frozen_arithmetic_order`, and `queued_three_sample_ramp_crosses_call_boundary_before_fusing` exercise `FaderMatrixBankProcessor`, not the new `ScalarPairProcessor`.

## Missing original gates after Sol2

1. Actual prepared `ScalarPairProcessor` compact FIFO/state/PCM sequence over both real queues: same sample, immediate controls, positive ramp, mid-ramp retarget, end-inside-call fallback then next-call fusion, gain, mute, remembered gain, unmute and reset with private per-call witness.
2. The same positive mechanism assertion made to fail by an actual selection-to-separate mutation while PCM remains equal.
3. Actual scheduled same-output scalar graph later-matrix failure proving first error, completed fader PCM/state and remaining queue records.
4. Actual graph defensive distinct-output decline/reference beyond the private lowered-program predicate plus synthetic `execute_op` proof.
5. New actual scalar host acknowledgement linkage.
6. New observed-plus-eligible staggered schedule with meter data/window, nonunity send/crossfeed, explicit schedule and unretired/unredirected proof.
7. Complete direct/aliased/extra-reader/sidechain/delay/stage/order/type/policy/post-matrix observation decline matrix tied to the scalar witness.
8. Scalar checked overflow and true pre/post-bind resource equality, plus the independent CAPI scalar mirror if still required.
9. New authorized allocation fixture proving actual queued `ScalarPairProcessor` eligible/ramp/observation fallback zero allocations and frees with independent liveness.

Sol2 therefore remained incomplete and should not receive source PASS from broad regression counts.

## Release evidence and baseline attribution

- Post-checkpoint release gates passed: `/tmp/443-sol2-post-checkpoint-release.command`; `/tmp/443-sol2-post-release-builtins-compiler.{log,status}` status 0; `/tmp/443-sol2-post-release-allocation.{log,status}` status 0; `/tmp/443-sol2-post-release-graph.{log,status}` status 0; `/tmp/443-sol2-post-release-host.{log,status}` status 0. Unchanged allocation/host fixtures remain regression evidence only.
- #443 standalone graph-compiler release command: `/tmp/443-sol2-final-caps-release-2.command`; `/tmp/443-sol2-final-caps-release-2.{log,status}` status 101.
- Exact clean baseline head `660fce8f` detached reproduction: `/tmp/443-sol2-exact-baseline-release.command`; `/tmp/443-sol2-exact-baseline-release.{log,status}` status 101. It reports the same `effect-package` rlib/cdylib output filename collision and then graph-compiler `E0463` for `effect_compiler`. The temporary detached worktree was removed.
- Clean descendant worktree reproduction: `/tmp/443-sol2-baseline-release.command`; `/tmp/443-sol2-baseline-release.{log,status}` status 101.
- Feature-unified workspace attempt: `/tmp/443-sol2-feature-unified-release.command`; `/tmp/443-sol2-feature-unified-release.{log,status}` status 101 with duplicate `effect_contract` type identities.

The release graph-compiler failure is confirmed baseline infrastructure, not a #443 regression, but the scalar cap release test remained unexecuted.
