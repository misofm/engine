# Delete digest pins, type-size recomputations and unrun ignored tests (test policy cut 1)

Owner approval: owner-approved policy cut, relayed by root on 2026-10-18. The deletions below need
no mutation proof: each one falls in a class that AGENTS.md ("Test value") already refuses, or it
never runs. Base: `main` at `81a3074fc`. Branch: `codex/batch-test-policy-1`. Phase B PR 1 of the
2026-10 test audit.

## Mission

Remove test code that defends no claim the repository accepts as a test claim:

- (a) a digest or exact byte count of something that is not a wire, ABI or on-disk format and not
  the single owner of a cross-target corpus (G5);
- (b) a test that re-derives `size_of`/`align_of` or a struct byte total of an internal type;
- (c) an ignored test that asserts nothing, or an ignored sweep that no workflow runs;
- (d) the builtins-compiler scalar-oracle pairing tests (#1059: the pairing is an optimisation
  inside a test-only oracle).

Helpers that become unused go in the same change. Docs and mutation records that name a deleted
gate say so.

## Scope

Delete or trim exactly the tests listed under "Evidence". In a test that mixes a refused class
with a real claim, only the refused assertions go; the real claim (an exact cap accepts, one byte
below refuses, ownership is returned, a fold rolls back on overflow, an allocator ceiling) stays.

## Non-goals

- No product-code change. The scalar oracle stays as the reference for vector banking.
- No MERGE work (per-host duplicates, parameter families) from the audit.
- No change to ceilings the product enforces, C ABI or browser layouts, wire goldens, the G5
  corpus and its owner, fuzz targets, randomized differentials, allocation counts, or defect
  reproducers of open issues.
- The ASK items below stay until the owner rules.

## Objective gates

1. `cargo fmt --all --check`.
2. `cargo clippy --locked -p <each touched package> --all-targets --all-features -- -D warnings`.
3. `cargo test --locked --all-features -p <each touched package>`.
4. `bash scripts/check-workspace-policy.sh`.

Touched packages: builtins-compiler, capi, host-core, host-web, math, parametric-eq,
console-workload, audit, graph-compiler, graph, effect-compiler.

## Test value

No new test. Two tests were rewritten as relations without `size_of`:
`effect_control_resources_charge_actual_tables_and_shared_owner_once` (red if two producers that
share one factory are charged it twice, or if the producer table is charged by length and not by
capacity) and `effect_control_resources_charge_no_registry_owned_factory` (red if the launch
registry's factory is charged to a plan). Both claims were in the deleted exact-byte forms.

Review fold 1 kept three relations the first cut lost, each recorded red on a mutant and restored:
- `live_scalar_owner_bytes_are_published_and_capped_before_binding` keeps "runtime metadata is
  admitted once" (`plain` estimate minus semantic estimate == `checked_for_with_response_bindings`
  total, from node IDs and string lengths, no `size_of`). Mutant: pass `0` for
  `response_binding_string_bytes` in `graph-compiler/src/compile.rs` -> RED, `left: 448`,
  `right: 492`.
- `the_charge_counts_each_kind_and_the_activity_table_once` asserts the #1652 MAJOR-1 activation
  offsets (`both.activity_bytes == alone.activity_bytes + ACTIVITY_ROUTES * 4`, one `u32` per
  prepared route). Mutant: `activity_bytes.saturating_add(offsets)` -> `saturating_add(0)` in
  `graph/src/lib.rs` -> RED, `left: 4096`, `right: 4124`.
- `runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow` asserts that the two #936
  tables and the #1609 block-end table hold one entry per emitted op (doubling the op count
  doubles each, and each is nonzero).

## Evidence

Line counts are lines removed from the file (`git diff --minimal --numstat`, net of the few added
lines), helpers and docs included.

### (a) Digest and byte-count pins

| file | test | change | lines |
|---|---|---|---|
| `crates/graph-compiler/tests/track_delay.rs` | `the_zero_delay_plan_digest_is_the_current_semantic_plan`, `a_delayed_session_is_a_different_plan`, `ZERO_DELAY_CANONICAL_SHA256`, the `sha256` field | deleted (plan SHA; `graph_fixture` owns canonical text) | 75 |
| `crates/graph-compiler/src/lib.rs` | `issue122_reverse_route_ids_emit_sorted_levels_and_bind` | the pinned plan SHA (`f5e310c7…`, four sites) removed; the schedule, levels, repeat and render stay | about 15 |
| `tools/audit/src/fixture_builtins.rs` | `issue064_checked_corpus_is_read_only_and_complete` | the `MANIFEST.tsv` SHA-256 assertion and its re-pin history removed; `--check` and the entry count stay | 104 |
| `tools/console-workload/tests/gain_pan_profile.rs` | `digests` (ignored, "no bit moved" against a base) | deleted; `probes_split_the_bank_units_and_move_no_bit` is the standing gate | 17 |
| `tools/console-workload/src/lib.rs` | `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables` | deleted (exact metadata byte count) | 121 |

### (b) Type-size recomputations (trimmed in place)

| file | test | what went | lines |
|---|---|---|---|
| `crates/graph-compiler/src/lib.rs` | `live_scalar_owner_bytes_are_published_and_capped_before_binding` | nothing after review fold 1: its runtime-metadata block uses no `size_of` and was restored | 0 |
| same | `effect_control_resource_uses_independent_queue_and_owner_arithmetic` | owner bytes from `size_of`; now: the deltas agree, are zero exactly when nothing is controlled, exact caps accept and one below refuses | about 100 |
| same | `runtime_bank_slot_reservation_is_published_and_capped_transactionally` | the `size_of` slot literal; now folds the product's `checked_for` reservation | about 45 |
| `crates/graph/src/lib.rs` | `runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow`, `split_table_resource_is_zero_when_declined_and_one_entry_when_eligible` | `size_of` totals; layout witnesses, one-entry-per-op scaling, fold and overflow stay | about 28 |
| same | `the_charge_counts_each_kind_its_own_bytes_and_the_activity_table_once` (renamed `the_charge_counts_each_kind_and_the_activity_table_once`) | per-kind `size_of` bytes; counts, IDs, joint sum, activation offsets and activity-once stay | about 23 |
| `crates/graph-compiler/tests/live_routes.rs` | `route_control_resources_cover_the_allocation` | the `LatestCell` layout pin and the field-by-field formula; the audited-allocator ceiling stays | 19 |
| `crates/effect-compiler/src/prepare.rs` | `effect_control_resources_charge_actual_tables_and_shared_owner_once`, `effect_control_resources_charge_no_registry_owned_factory` | rewritten as relations (shared factory once, table by capacity, registry factory not charged) | 58 |

### (c) Ignored tests that assert nothing, and unrun ignored sweeps

`--ignored` runs were checked in `.github/workflows/*.yml` and `scripts/` first.

| file | test | why | lines |
|---|---|---|---|
| `crates/host-core/tests/live_edit_classifier.rs` | `timing_line` | prints a time | 70 |
| `crates/capi/src/runtime/live_tests.rs` | `in_place_command_timing_line` | prints a time | 44 |
| `hosts/host-web/tests/control_command_timing.rs` | `control_command_timing_line` (and its `frame` helper) | prints a time | 99 |
| `crates/math/tests/m2_lane_identity.rs` | `m2_exp2_lane_identity_all_f32_bit_patterns`, `m2_log2_lane_identity_all_f32_bit_patterns` (and their two helpers) | no workflow runs them | 63 |
| `crates/math/tests/scalar_accuracy.rs` | `sqrtf_is_correctly_rounded_exhaustive`, `sqrt_is_correctly_rounded_large_oracle` | no workflow runs them | 42 |
| `crates/parametric-eq/src/lib.rs` | `every_word_gets_the_input_contracts_verdict` | no workflow runs it | 67 |

Kept: every ignored test that `nightly.yml` or `qualification.yml` runs (`f1_fast_db_bounds`,
`m1_exhaustive`, `designer_total`, the 65,537-track scale gates, the release time budgets
including `a_vca_move_at_the_bound_fits_a_quantum_in_release`, the nightly PEQ and conformance
sets, `exhaustive_f32_round_trip`); the open-defect reproducers (#391, #1069, #1070, #1072,
#1073); `automation`'s `print_pins` (the G5 pin generator).

### (d) Scalar-oracle pairing tests (#1059)

17 of the 20 deleted from `crates/builtins-compiler/src/lib.rs`:
`actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners`,
`actual_scalar_nonadjacent_schedule_selects_the_split_owner`,
`actual_scalar_overlapping_nonadjacent_candidates_select_one_and_keep_the_other_separate`,
`actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated`,
`actual_scalar_nonadjacent_invalid_fader_matrix_boundaries_preserve_retry_order`,
`actual_scalar_graph_preserves_scheduled_matrix_error_and_applies_the_next_write`,
`actual_scalar_extra_reader_declines_and_retains_separate_owner_pcm`,
`actual_scalar_effect_sidechain_declines_and_matches_separate_owner_data_and_state`,
`staggered_observed_scalar_track_stays_separate_while_eligible_peer_pairs`,
`actual_scalar_nonunity_send_reader_declines_with_reference_output`,
`scalar_factory_checks_both_exact_owners_and_policies_without_consuming_state`,
`scalar_split_owner_defers_settled_fader_and_completes_before_matrix_error`,
`scalar_split_invalid_begin_envelope_does_not_arm_pending_or_drain_matrix`,
`scalar_invalid_envelope_leaves_the_later_matrix_queue_untouched`,
`scalar_pair_reset_matches_reset_original_owners`,
`scalar_owner_resource_is_independent_two_boxes_plus_two_pointer_outer`,
`the_scalar_owner_row_charges_one_automated_fader_lane`. With them went the now-unused harness
helpers (`render_scalar_pair_and_compare`, `scalar_live_control_owners`, `write_fader_both`,
`write_matrix_both`, `ScalarLiveControlOwners`) and the harness's `Sidechain` variant with its
`SidechainSum` fixture effect. 2,328 lines.

The oracle keeps its callers: no oracle type or function became dead. One oracle-side witness did:
`initial_matrix_state` (crate level, `cfg(all(test, feature = "test-support"))`, the only reader
of `SCALAR_INITIAL_MATRIX_TRACE`). Its only readers are three of the 20 tests, which stay until the
owner rules (ASK 1):
`actual_scalar_nonadjacent_intervening_observer_error_completes_and_retries`,
`actual_scalar_nonadjacent_failed_render_materializes_post_fader_before_error`,
`actual_scalar_nonadjacent_ramp_retarget_and_failed_retry_stay_at_original_boundaries` (713 lines).

### ASK (not cut; owner ruling needed)

1. The three pairing tests above. Options: (A) delete them and the witness
   (`initial_matrix_state`, `record_initial_matrix_state`, `SCALAR_INITIAL_MATRIX_TRACE`, its trace
   type and its one call in the oracle's matrix owner): test-only instrumentation, not the oracle's
   render path; (B) keep them. `#[allow(dead_code)]` is not an option.
2. `crates/math/tests/m2_lane_identity.rs` `EXP2_DIGEST`/`LOG2_DIGEST`. A second pin of cases G5
   owns, so class (a) by form. But `scripts/lib/aarch64-known-defects.py` names
   `m2_exp2_lane_identity` as the `aarch64-release` expected failure of LANE-3 (#1019), judged by
   that digest's panic text. Without the pin the test passes on AArch64 release and the leg fails
   ("a pass fails the leg"), and #1019 loses its native witness. Cut together with that row only
   if the owner accepts losing the witness.
3. `crates/host-core/src/limiter_linked_session.rs` `PIN`: a rendered-output digest recorded on the
   pre-#990 kernel (the "no bit moved against a base" shape), but self-declared "the single
   cross-target owner of the 64-track linked-limiter session corpus" and cited by
   `true-peak-limiter` MUTATIONS row 996-K2.
4. `tools/console-workload/tests/gain_pan_profile.rs` `phase_profile`: an ignored measurement
   harness no workflow runs (R9 retired descriptive benchmarks). Deleting it leaves the
   `graph::test_only_phase_profile` probes with no consumer but their own test.
5. Exact move-table byte counts in `crates/host-core/tests/successor_swap.rs` (14 `size_of` sites) and
   single `size_of` lines in `host-core` `route_mute.rs`, `submix_strip.rs`, `prepare.rs` and `capi`
   `resource_lifecycle.rs`: each is one line inside a larger test and encodes a move or entry count.
   The launch-effect fixture tests in `graph-compiler/src/lib.rs` (a MERGE family) also use
   `literal_bank_slot_reservation_bytes`.

### Not cut, by class

- Wire, ABI and layout: `protocol` `message_wire` goldens; `host-web` `src/tests.rs` `size_of`
  layouts and its browser digest pins; `effect-contract` `live_control.rs` (`<= 64` is a ceiling).
- G5: every effect and `math` corpus digest table; `bench-support` SHA-256 known answers.
- Product code: `tools/audit` `builtins.rs` and `builtins_graph.rs` fixture identities are checks
  in the tool itself, not test assertions.
- Ceilings: `graph` `runtime.rs` executor-table bound, `bypass_resources.rs`, allocation trackers.
- `track_delay.rs` `the_estimate_charges_each_lane_its_own_ring` (deleted by TA4; KEEP catcher graph-compiler `track_delay::a_ring_is_a_named_allocation`): four bytes per `f32` sample is the
  product's accounting rule, not a struct total.

### Docs updated

`.github/ISSUE_SPECS/1379-define-how-node-tails-compose-through-gain-in-the-graph-extent.md` (lines
169 and 263) names `ZERO_DELAY_CANONICAL_SHA256` as a re-pin step. That pin is deleted here, so the
step no longer applies; #1379's spec is left as written.

`crates/graph/tests/MUTATIONS.md` and `crates/graph-compiler/tests/MUTATIONS.md` gain a TA1 note
naming the rows whose tests were deleted or trimmed; `docs/derivations/241-schema-repins.md`
notes the deleted companion assertion. Module docs of the touched test files were corrected.
`docs/handoffs/` and `docs/audits/` are dated records and were not changed.
