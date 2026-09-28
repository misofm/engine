# Delete the unused control-provider endpoints

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`, and the live-control research `docs/handoffs/live-control-2026-09-28/FINDINGS.md` (+ `VERIFY.md`). **The Amendments sections supersede the body.** **Owner ruling (2026-09-28):** approved, including step 4 (the protocol delivery files): mobile live control uses the core's console lanes instead (see the live-control issue filed alongside). #140 is closed as *descoped*, not superseded, once the stored-automation ruling is recorded.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 3, row A4. The
code is dead today whatever you rule on the C ABI and the protocol, because nothing calls it, not
even `capi`.

## Context

`crates/host-core` carries two native control endpoints behind the `control-provider` feature
(`src/lib.rs:92-93`, `:108-109`, `:225-228`):

| file | lines |
|---|---:|
| `src/builtin_batch_endpoint.rs` (#576, #579, #580, #587, #594) | 2,529 |
| `src/scalar_point_endpoint.rs` (#528, #536, #563, #575, #605, #608) | 1,060 |
| `tests/builtin_batch_endpoint.rs` | 1,727 |
| `tests/scalar_point_endpoint.rs` | 2,928 |
| **total** | **8,244** |

- **No callers.** `rg 'BuiltinBatchEndpoint|ScalarPointEndpoint|builtin_batch_endpoint|scalar_point_endpoint' crates hosts tools`
  finds only these four files and `host-core/src/lib.rs`. `capi` enables `control-provider` but
  never names either endpoint.
- **Compile proof.** A scratch copy with the modules, the re-exports and the two test files
  removed passed `cargo check --workspace --all-targets --all-features`.
- **Test cost.** The tests run in `test-debug-a` (feature unification through `capi`). There are
  30 integration tests (20 + 10), plus the in-source unit tests of `builtin_batch_endpoint.rs`,
  and all of them together take under a second. The cost is compile time.
- **Not in the browser.** `host-web` never enables `control-provider`, and
  `scripts/check-host-core-policy.sh` forbids it. So the shipped module cannot change.
- **History.** These were partial steps toward open #140 (deliver admitted **protocol** automation
  to the running plan). #140 is about the native protocol path. The browser's command admission is
  host-web's own (`admit_commands`).

## Smallest closable slice

1. Delete the four files, their `mod` and `pub use` lines, and any `control-provider`-only items
   that become unused. Prove the latter by compile, and list them.
2. Delete what exists only for these endpoints:
   - their `MUTATIONS.md` rows;
   - their entries in `scripts/check-host-core-policy.sh` and `scripts/test-host-core-policy.sh`,
     if any;
   - their mentions in `docs/`, outside `docs/handoffs/`.
3. Post a comment on #140 saying the partial endpoints were removed as unused, and pointing at this
   issue. If the protocol ruling (`R3-…`) keeps the protocol, #140's successor re-derives what it
   needs.
4. Optional, and only if the protocol is kept: protocol's `delivery.rs`, `controller_delivery.rs`
   and `tests/delivery_ownership.rs` (about 4,930 lines) have these endpoints as their only callers
   outside `protocol`. `controller.rs` also depends on them, so removing them needs surgery. File
   that separately. If `R3-…` removes the protocol, skip it.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core`
     passes.
2. **Console digests.** The `gain_pan_profile digests` output
   (`cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`)
   is byte-identical on base and change.
3. **Shipped artifact: byte-identical.**
   - Build base and change with `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR` on one machine.
   - `host-web` does not compile these modules, and `lib.rs` changes only inside `cfg`'d lines.
   - If the hash moves, stop and explain.
4. **Tests (no live claim lost).** `cargo test` over the CI feature sets (after `00-…`) passes.
   The `-- --list` diff against base contains only tests defined in the four deleted files. List
   them in the evidence.
   - Their subject is the deleted endpoints themselves.
   - `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
     (`builtin_batch_endpoint.rs:1497`) compares each backend's endpoint with its own reference
     (`:1729-1743`), not Scalar with banked. So no "banking moves no bit" claim is lost. That claim
     lives in `graph-compiler/tests/bank_levels.rs`.
   - Some deleted tests also assert allocation-free render and off-render reclamation for the
     endpoints (`tests/builtin_batch_endpoint.rs:202`, `:374`; `src/builtin_batch_endpoint.rs:2138`;
     `tests/scalar_point_endpoint.rs:1469`, `:2736`). Name the surviving equivalents for the live
     paths in the evidence:
     - host-core `tests/spectrum.rs:439`, `:649` and `tests/observation_demand.rs:2007`;
     - `scripts/check-web-audioworklet.sh:349-449`;
     - host-web `src/tests.rs:3537`.
   - After this change, the `#[cfg(test)]` seam `prepare_host_runtime_with_console_backend`
     (`prepare.rs:596`) keeps one user (`limiter_linked_session.rs:333`). Keep it.
5. **CI routing.** No workflow references these files. `check-ci-path-routing.py` and
   `test-ci-path-routing.py` pass. `check-host-core-policy.sh` and its mutation test pass.

## Dependencies

`00-…`, then `01-…`, which deletes three members of these files.

## Standing rules for the implementer

- No product behaviour change.
- Commit on `codex/<issue>-delete-unused-endpoints`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3. **Hold this draft for an owner ruling. "Dead whatever you
rule on the C ABI and the protocol" is wrong under the corrected scope.**

1. **The compile facts hold.** With both source files, both test files and their `mod`/`pub use`
   lines removed: native `--all-targets --all-features` passes with no new warning, `wasm32`
   `simd128` passes, iOS and Android `--lib` pass, and `-p capi -p host-core --all-targets` passes
   on iOS. No `control-provider` item becomes unused.
2. **But they are unwired, not dead.** Mobile playback is live scope and the C ABI stays (R2
   amendment). The open #140 spec targets delivery of admitted protocol automation "through the
   actual C ABI command and render calls"; per its own text (line 13) the only production consumer
   of that queue today cancels it. These endpoints (#528-#608) and protocol's `delivery.rs` /
   `controller_delivery.rs` are the partial implementation of exactly that. Deleting them is a
   #140 design decision.
3. **Failure scenario:** the draft lands as "no ruling needed"; the owner then schedules mobile live
   control (a fan's fader or mute during playback), and #140's successor must re-derive about 8,200
   lines of qualified, tested endpoint code that git history holds but no current test exercises.
4. **Revised recommendation:** move this draft to the "needs your ruling" set, decided together
   with #140: either (a) #140's plan uses these endpoints, so keep them and wire them; or (b) #140
   re-plans from scratch, so delete them as drafted. Step 4 (protocol delivery files) follows the
   same decision.

## Attempt 1 evidence

Terra, 2026-09-28. Branch `codex/1056-delete-unwired-control-stack` from `codex/batch-slim-1` at
`ed0556a9`; implementation commit `c46d5b36`. Host x86-64-v3 Linux, rustc 1.97.1, `CARGO_INCREMENTAL=0`.
Base figures come from a `git archive` export of `ed0556a9` built on the same host. Scope is draft 02
plus its step 4, as ruled in `docs/rulings/engine-footprint-2026-09-28.md`. #1053 (mobile live
control) and the `AutomationEnqueue` refusal are not implemented here. capi's command path is
unchanged: `AutomationEnqueue` is still admitted but not applied.

### What was deleted

| file | lines |
|---|---:|
| `crates/host-core/src/builtin_batch_endpoint.rs` | 2,528 |
| `crates/host-core/src/scalar_point_endpoint.rs` | 1,060 |
| `crates/host-core/tests/builtin_batch_endpoint.rs` | 1,727 |
| `crates/host-core/tests/scalar_point_endpoint.rs` | 2,928 |
| `crates/protocol/src/delivery.rs` | 2,135 |
| `crates/protocol/src/controller_delivery.rs` | 2,102 |
| `crates/protocol/tests/delivery_ownership.rs` | 693 |
| **seven files** | **13,173** |

Plumbing:
- **host-core `lib.rs` (-9).** Removed the two `cfg(feature = "control-provider")` `mod` lines and
  their two glob `pub use` lines. Nothing outside `cfg` changed.
- **protocol `lib.rs` (-12).** Removed the two `mod` lines and the `controller_delivery`/`delivery`
  re-exports.
- **protocol `controller.rs` (+37/-302).**
  - Removed `DeliveryContext` and `ScalarStateSlot`, and the threading of both through
    `process`, `process_b1b_btlv`, `process_command_frame_into`, the legacy path and
    `execute_decoded_command`. Six `*_with_delivery_context*` wrappers collapse into their public
    callers, and `execute_with_delivery_context` becomes `execute`.
  - Removed the six `delivery_*` methods and the delivery-only branches of `execute`: the
    `Unavailable` refusals, the scalar-slot `ParameterStateGet` page, and the delivery
    `validate_records`/`try_admit` arm of `AutomationEnqueue`.
  - Removed the two test-only accessors `reset_typed_command_decodes` and `typed_command_decodes`,
    which only `controller_delivery` tests used. The `TYPED_COMMAND_DECODES` counter stays, because
    `controller/tests.rs` uses it.
  - Removed the now-unused `ParameterStateRecord` import. Every non-delivery path was already
    called with `None`/`None`, so the remaining code is the `None` arm verbatim.
- **protocol `queue.rs` (+2/-17).** Removed `release_automation_admission` and
  `try_dequeue_automation_retaining_admission`, the latter inlined into `try_dequeue_automation`.
- **`scripts/check-realtime-policy.sh` (+1/-3).** Removed the stale `delivery_ownership.rs` `unsafe`
  exemption and its comment (VERIFY A4).

**Items that became unused (proof by compile).** After the deletion, rustc flagged exactly one item:
the `ParameterStateRecord` import. `cargo check`/`clippy --all-features` then report no
dead code. No `control-provider` item became unused, as Amendment 1 predicted. `record_canceled_automation`
and `AutomationBatchSlot::validate_records` still have callers. `MUTATIONS.md`, the host-core
policy scripts, the workflows, the fuzz targets and `ci-path-router.py` did not name the deleted files.

**Kept, deliberately.**
- **`prepare_host_runtime_between_render_calls_with_backend` (`prepare.rs`).** It was split out of
  `prepare_host_runtime_between_render_calls` by #587 for the endpoint's scalar-backend seam, and
  it now has one caller. Folding it back measurably moved the shipped module: 6 bytes of
  panic-location line numbers (`3f744b03…` to `0c1b74b7…`), so the fold was reverted. Only its doc
  comment was rewritten, line for line, to drop the endpoint description. The public
  `prepare_host_runtime_between_render_calls` (#450) now has no caller in the workspace. It
  predates the endpoints and is outside this slice, so it is left for a later dead-code pass.
- **The `#[cfg(test)]` seam `prepare_host_runtime_with_console_backend`.** Its one user is
  `limiter_linked_session.rs`, as gate 4 requires.
- **`docs/audits/5xx-*` records** that name the deleted files (#571, #572, #576, #579, #587, #594).
  These are dated evidence of closed issues, not current-state documentation, so they are left as
  history.

**Docs re-pointed.** Edits inside crates that host-web compiles are line-neutral.
- `docs/CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status", now states:
  - admitted `AUTOMATION_ENQUEUE` reaches no PCM on any host, the C ABI included;
  - #140 is descoped, and #1056 deleted its unwired partial implementation;
  - stored automation belongs to #1058, and C ABI live fader, mute and pan to #1053;
  - refusing the command is a separate follow-up (#370, IO-5).
- `docs/SESSION_SCHEMA_V1.md:97-101` and `docs/REALTIME_MEMORY.md:13` point at #1058 (and #1053).
- `docs/rulings/builtins-input-liveness-d2.md` "What would reopen" gains a dated re-point to #1058.
- The `session/src/validate.rs` doc comment is re-pointed at #1058, line-neutral.
- The `protocol/src/queue.rs` doc comment is updated.

### Lines

`git diff --shortstat ed0556a9 c46d5b36`: 18 files, **+59 / -13,531, net -13,472.** Code and scripts alone
(`crates/`, `scripts/`) are +50/-13,526. VERIFY's parallel pass measured +38/-13,511 for the deletion
alone. The difference is the line-neutral comment rewrites and the doc re-points.

### Gates

| gate | result |
|---|---|
| 1. `cargo check --locked --workspace --all-targets --all-features` | pass, no warnings |
| 1. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the same without `--all-features` | pass |
| `cargo fmt --all -- --check`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` (also `-p host-core -p protocol --all-features`) | pass |
| 1. `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core` | pass |
| `aarch64-apple-ios` and `aarch64-linux-android`: `cargo check --lib` of `lane capi host-core host-mobile target-smoke engine session protocol graph graph-compiler builtins builtins-compiler effect-compiler soft-clip`; `cargo check --all-targets --all-features -p capi -p host-core -p protocol`; `clippy -D warnings` of the product `--lib` set (with `host-core/control-provider`) and of `-p capi -p protocol --all-targets --all-features` | pass |
| aarch64 `clippy --all-targets -p host-core` | fails with 6 `dead_code` errors in `host-core/tests/fp_environment.rs`: x86-only helpers in the test file. **Pre-existing**: the base export produces byte-identical output for every aarch64 row. Not touched here; it belongs to #1017's CI leg. |
| 2. `gain_pan_profile digests` (release) | **byte-identical** to base (all 17 rows) |
| 3. `scripts/build-web-audioworklet.sh --module-only` | **byte-identical**: base and change both `3f744b03e22ed0ecb45ab8358eea73fb3bf4a834a17e1ec4ac348aaca64b25da`. The checked-in pin (`476e58ad…`) already differs on base, as the batch repins at its boundary. Not repinned here. |
| 4. test-debug-a set (CI's exact command and features) | 1,436 passed, 0 failed, 10 ignored; `cargo run -p host-native` ok |
| 4. test-debug-b set | 807 passed, 0 failed, 28 ignored |
| host-core (`control-provider`, `test-support`), protocol (`test-support`), capi: all targets, dev and release | 351 passed, 0 failed, 2 ignored, in each profile |
| release `-p bench -p console-workload`; `audit protocol` caller-buffer allocation audit; `cargo check --manifest-path fuzz/Cargo.toml --bins` | pass (127 passed); audit "ok"; pass |
| `scripts/run-wasm-gates.sh` | ok: native, wasm scalar and wasm `simd128` legs at 0 mismatches over 358 comparisons; V8 spill gate ok |
| 5. `check-ci-path-routing.py`, `test-ci-path-routing.py`; `check-`/`test-` for workspace, session, bench, host-core, protocol-control, realtime, lane, rack, builtins, graph and effect-runtime policy; env-vocabulary, test-support-ci, realtime-audit-leak, artifact-evidence-leak, unfused-seal, conformance-boundaries, step-vocabulary, effect-interchange-policy | all pass. No workflow names the deleted files. |

### `cargo test -- --list`, base against change

Both lists come from the test-debug-a command. A second pair, `--all-features -p protocol -p host-core -p capi`,
gives the same diff. Base has 1,534 tests and change 1,446. **88 removed, 0 added, and every removed
test is defined in a deleted file:**
- host-core `tests/builtin_batch_endpoint.rs` (20): `actual_endpoint_allocations_and_nonempty_cancellation_reuse_are_live`, `batch_applies_one_fifo_ticket_at_a_late_boundary_and_reconciles`, `cancellation_before_claim_is_render_only_and_releases_after_collection`, `cancellation_reconciles_applied_and_future_frontier_dispositions`, `empty_and_precollected_cancellation_finalize_once_and_reopen_publication`, `endpoint_drives_nonzero_pcm_through_the_prepared_bank_and_scalar_plan`, `future_batch_stays_pending_then_applies_once_and_invalid_batches_are_atomic`, `invalid_and_saturated_publication_are_atomic_and_resources_have_exact_caps`, `ordinary_cancel_remains_reusable_before_terminal_shutdown`, `outcome_is_staged_before_credit_release_and_fifo_late_pair_is_exact`, `prepared_and_control_halves_have_transferable_ownership`, `repeated_render_and_cancellation_boundaries_are_allocation_free`, `retained_endpoint_report_matches_independent_concrete_layout_oracle`, `scoped_render_receiver_exits_when_control_sender_drops_on_panic`, `separate_control_and_render_threads_preserve_single_claim`, `shutdown_cannot_overtake_cached_ordinary_ack_after_final_collection`, `shutdown_reconciles_applied_claimed_future_and_queued_ownership_exactly`, `shutdown_rejects_stale_token_before_and_after_completion`, `shutdown_render_is_allocation_free_and_stop_reclaims_lifecycle_off_render`, `terminal_shutdown_acknowledges_once_then_quiesces_and_reconciles`
- host-core `tests/scalar_point_endpoint.rs` (10): `claim_state_is_truthful_until_terminal_collection`, `controller_cancellation_keeps_real_prefix_event_credit_and_native_state`, `controller_points_drive_real_asymmetric_pcm_replay_and_clock_refusal`, `controller_publication_survives_real_sticky_snapshot_fault`, `controller_scalar_preflight_rejections_and_single_allocation_authority`, `late_points_apply_in_order_and_second_ticket_waits`, `malformed_admission_and_render_envelopes_are_noops`, `points_slice_real_pcm_and_readback`, `preparation_resources_and_success_path_are_bounded`, `real_cancellation_preserves_applied_prefix`
- protocol `tests/delivery_ownership.rs` (11): `controller_facade_preparation_has_one_queue_allocation_authority`, `distinct_render_owner_reconciles_all_four_cancellation_positions_without_allocation`, `generic_boundary_cancel_has_independent_request_capacity_and_exact_frontier`, `generic_boundary_cancel_reports_zero_partial_and_full_without_releasing_credits_early`, `generic_boundary_cancel_scope_drops_release_on_control_failure`, `generic_boundary_cancel_thread_schedule_covers_zero_partial_and_full_application`, `generic_boundary_cancel_uses_separate_threads_and_holds_credit_after_ack`, `generic_invalid_prefix_and_resource_overflow_preserve_cancellation_ownership`, `generic_preparation_report_matches_allocation_and_repeated_reuse_stays_zero_alloc`, `prepared_heaps_free_off_thread_and_realtime_owner_operations_are_zero_zero`, `staged_automation_remains_non_applicable_while_cancel_races_boundary`
- host-core lib `builtin_batch_endpoint::tests` (11): `dropped_prepublication_release_rolls_back_without_hanging`, `endpoint_queue_retention_matches_layout_and_reclaims_off_render`, `endpoint_selects_existing_pair_factories_without_observer_barriers`, `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`, `lifecycle_arc_survives_control_drop_and_stop_until_off_render_drop`, `ordinary_ack_before_classification_cannot_be_overtaken_by_shutdown`, `pcm_bitwise_discriminator_rejects_signed_zero`, `pending_lifecycle_intent_renders_normally_and_failed_begin_rolls_back`, `private_post_claim_hold_rejects_same_block_second_claim`, `private_post_graph_fault_is_sticky_before_terminal_publication`, `sticky_render_fault_refuses_shutdown_and_retains_ticket_ownership`
- protocol lib `controller_delivery::tests` (16): `caller_buffer_correlatable_payload_preserves_identity_and_precedence`, `caller_buffer_facade_refuses_structural_commands_before_planning`, `caller_buffer_frame_ingress_reserves_and_replays_through_facade`, `caller_buffer_frame_limits_remain_unavailable_without_model_effects`, `caller_buffer_frame_rejections_saturate_and_cancel_whole_unsupported_batches`, `caller_buffer_pending_cancel_refuses_non_locate_transport_without_effects`, `encoded_point_ingress_uses_facade_delivery_context_and_replays`, `fixed_revision_refuses_structural_locate_and_state_without_model_change`, `framed_facade_limits_remain_unavailable`, `framed_facade_rejections_are_atomic_and_saturate_at_bound`, `framed_unsupported_batch_stays_pending_and_cancels_whole_batch`, `one_controller_queue_and_sequence_cover_transport_cancel_and_short_retry`, `real_admission_replay_shape_and_domain_guards_are_owned_by_facade`, `reservations_survive_render_completion_until_terminal_collection`, `scalar_opt_in_rejects_invalid_bindings_and_reports_typed_publication_errors`, `unsupported_head_blocks_fifo_and_real_cancel_releases_all_owners`
- protocol lib `delivery::tests` (20): `applied_only_cancel_emits_zero_and_resets_generation_and_ordering`, `automation_progress_rejections_preserve_exact_pending_and_terminal_ownership`, `cancellation_identity_overflow_is_preflighted_before_dequeue_or_reservation`, `cancellation_keeps_staged_and_newly_owned_automation_unpublished`, `cancellation_publication_uses_admission_order_after_physical_slot_reuse`, `cancellation_refusals_are_transactional_and_events_keep_admission_order`, `generic_copy_core_transfers_layout_independent_payload`, `generic_core_rejected_identities_preserve_pending_and_terminal_owner`, `generic_core_slot_identity_covers_capacity_above_u16`, `generic_old_cancel_token_is_stale_after_reuse_generation`, `generic_physical_credit_and_terminal_identity_are_held_until_collection`, `generic_serial_and_generation_overflow_refusals_are_transactional`, `logical_record_bounds_and_resource_report_are_exactly_composed`, `occupied_reliable_capacity_refuses_next_cancel_without_mutation`, `ordered_cancel_reconciles_handed_off_and_partial_at_actual_boundary`, `rejected_admission_preserves_the_exact_batch_and_service_state`, `retained_admission_survives_handoff_until_terminal_consumption`, `total_outstanding_bound_and_terminal_collection_are_the_only_reuse_credit`, `unsupported_head_blocks_fifo_and_cancel_ack_publishes_event`, `wide_cancellation_accounting_has_no_u16_boundary_or_saturation`

`forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses` compared each backend's
endpoint with its own reference, not Scalar with banked. So no "banking moves no bit" claim is lost:
that claim lives in `graph-compiler/tests/bank_levels.rs`.

**Surviving equivalents for the deleted allocation and reclamation claims**, on the live paths:
- **host-core.**
  - `tests/spectrum.rs:439` `selected_capture_is_pcm_bit_exact_and_allocation_free_for_idle_and_active_renders`
  - `:649` `prepared_collection_switches_exact_taps_without_audio_or_render_allocation`
  - `tests/observation_demand.rs:2007`: spectrum render, zero allocations and zero deallocations
- **Browser artifact.** `scripts/check-web-audioworklet.sh:349-449`: the render export's call graph
  reaches no allocator, deallocator or drop glue. Also host-web `src/tests.rs:3506`
  `production_effect_delivery_refuses_prepared_target_without_queue_or_full_mutation` (the brief's
  `:3537`).
- **C ABI.** `crates/capi/tests/resource_lifecycle.rs`:
  - `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`: plan replacement
    and off-render retirement through the exported calls;
  - `render_diagnostic_egress_reuses_eager_capi_storage_without_allocation`.

### Step 3: comment for #140, to be posted by root

I did not post it, per the brief. Suggested text:

> The partial native delivery stack built toward this issue had no caller, not even the C ABI, and
> was deleted as unused in #1056. It comprised the #460 delivery service and its #530 controller
> facade (`protocol/src/delivery.rs`, `controller_delivery.rs`) and the host-core builtin-batch
> (#576-#594) and scalar-point (#528-#608) endpoints: 13,173 lines. Git history keeps it.
> Per the 2026-09-28 owner ruling (`docs/rulings/engine-footprint-2026-09-28.md`), this issue closes
> as descoped:
> - stored session automation rendering moves to #1058;
> - mobile live fader, mute and pan moves to #1053 on the core's console lanes;
> - sample-timed `AutomationEnqueue` delivery has no product consumer, and it stays admitted but not
>   applied, as documented.

### Not done / notes for review

- **`AutomationEnqueue` is still admitted and advertised.** Refusing it is the separate follow-up
  (#370, IO-5). The docs now say so.
- **#1034** (protocol items rustc proves unused) should rebase on this change. The two decode-counter
  accessors, `release_automation_admission` and its helper are gone here.
- **`docs/audits/` history is left as is** (see "Kept").

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-28, on `c46d5b36`/`d468b80c` merged onto the current batch head `7d0d4adf`
(scratch merge, not kept). Since `ed0556a9` the batch moved only docs and two console-benchmark
scripts, so the merged Rust tree equals the branch's. Host EPYC 7313P, rustc 1.97.1,
`CARGO_INCREMENTAL=0`, one fresh target shared by the merge and a base worktree at `7d0d4adf`.

- **C ABI and mobile path.** The controller diff is the `None` arm verbatim. Every removed branch
  was guarded by a `Some` delivery context or scalar slot, and only the deleted facade ever passed
  one. The public `ProtocolController` signatures and `capability_registry` are identical to base.
  `crates/capi` and `include/` are untouched. All 36 capi tests pass, among them
  `capi_controller_dispatches_every_advertised_command_family`,
  `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`,
  `structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` and
  `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`. `audit capi` ran
  100,000 calls with 0 violations. `check-capi-abi.sh` and its self-test pass. On iOS and Android,
  the `--lib` set, `--all-targets --all-features -p capi -p host-core -p protocol` and both clippy
  sets pass.
- **Removed tests.** I compared `--list` on base and change, qualified by binary, for test-debug-a
  and for `--all-features -p protocol -p host-core -p capi`: 88 removed, 0 added. test-debug-b is
  unchanged. The removed tests are `builtin_batch_endpoint::tests` (11),
  `controller_delivery::tests` (16), `delivery::tests` (20) and the three deleted integration
  binaries (20, 10, 11).
  - Every test body drives a deleted type or helper: the endpoints, the delivery service, or the
    `facade()` fixture. `pcm_case` calls `prepare_scalar_point_endpoint`.
  - Three tests guard no product code: two std-only thread-scope harness checks and the signed-zero
    `to_bits` tautology. All three are on #1046's H-list.
  - One test touched surviving code: `endpoint_selects_existing_pair_factories_without_observer_barriers`
    checked pair selection through the endpoint's `BetweenRenderCalls` lowering. That selection is
    still covered by builtins-compiler's `test_only_fader_matrix_witness` tests and by host-core's
    `observation_demand` between-render-calls variants.
  - `apply_parameter_point` survives. It keeps its tests in `compressor/tests/native_points.rs` and
    `conformance.rs`.
- **Protocol.**
  - The wire, message and typed-frame sources are untouched.
  - `try_dequeue_automation` inlines the old two-step path and releases density and intervals the
    same way.
  - `record_canceled_automation` and `validate_records` keep their callers.
  - These pass: the protocol tests, `conformance` (in test-debug-b), `conformance_fixtures --check`,
    `check-protocol-wasm-parity.sh` (scalar and simd128), the caller-buffer allocation audit, and
    the fuzz `--bins` check.
- **Merged-tree gates.**
  - Lint and docs: fmt passes. Check and clippy `-D warnings` pass with all features and with
    default features. `cargo doc -D warnings` passes for the workspace and for
    `-p host-core -p protocol --all-features`.
  - Tests:
    - test-debug-a: 1,436 passed, 10 ignored; `host-native` runs.
    - test-debug-b: 807 passed, 28 ignored.
    - All-features protocol, host-core and capi: 351 passed, 2 ignored.
    - Release audit, bench and console-workload: 176 passed, 3 ignored.
  - Targets: wasm32 simd128 checks and `check-cross-targets.sh` pass.
  - `run-wasm-gates.sh`: 0 mismatches over 358 comparisons in each of three legs; the V8 spill gate
    passes.
  - Byte-identical to base: all 17 `gain_pan_profile digests` rows, and the `--module-only`
    artifact `3f744b03…` from fresh builds of both trees.
  - Scripts: the routing check and its mutation test pass. Every lint-job policy script passes
    except the one below.
- **Failures that are not this change.**
  - aarch64 `clippy --all-targets -p host-core` shows the same six `fp_environment.rs` errors on
    base (#1017).
  - `check-step-vocabulary.py` fails on the batch head itself, at `1025-…md:238` (a retired step
    spelling), introduced by #1025. The branch alone passes. This lint-job step must be fixed
    before the batch is pushed.

Findings:

1. LOW `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` (and the evidence above) name "#370, audit row IO-5"
   as the owner of the `AUTOMATION_ENQUEUE` refusal follow-up. #370 is CLOSED (docs-only,
   2026-09-04). The open owner is #349's IO-5 row, as #1053 A5 says. Replace `#370` with `#349`
   before the batch boundary.
2. NIT, evidence "Kept": the public `prepare_host_runtime_between_render_calls` did not lose a
   caller here, because it had none on base either. It and `apply_parameter_point` are now unused
   in production. Leave both for the dead-code pass.
3. NOTE: some open specs still name deleted tests or files: #1021 (its baseline list), #1046 (three
   H-list rows), #1023, #1034, #957 and #997. Refresh them when each is next touched.
