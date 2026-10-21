# Read every host-web test once and cut the defended duplicates (TA read, host-web)

Owner decision 2026-10-20, relayed by root: read every remaining test once, crate by crate, and
delete each test that a named remaining test already defends (AGENTS.md "Test value"). The host
twin addendum applies (owner decision 2026-10-20, "host twins, then stop"): a claim is tested once,
in the crate that owns its logic, and each host keeps at most one short parity row per host
behaviour. This spec covers the `host-web` package. Branch `codex/batch-test-read-host-web`, from
`main` at `64fe78c46`, with `main` at `d85e38442` (the host-core and graph read cuts, #1738 and
#1736) merged in. No new scope: findings that need work are listed under "Gaps and successors".

## Method

- Two readers read all 227 `host-web` tests in 18 files: w1 read `src/tests.rs`, w2 read
  `src/ffi.rs`, `src/render_lock.rs` and the 15 files under `tests/`. Per test they answered
  "which plausible defect turns this red that no remaining test catches?" and wrote KEEP, DELETE
  or MERGE.
- An adapter-only claim (ABI layout, pointer and length refusals, error mapping, the wasm memory
  and boot path, the worklet message handler, browser resource ceilings) stays. A twin of a claim
  whose logic lives in another crate goes, and names its defender in the owning crate and the
  parity row that reaches that path through the browser. `tests/control_parity.rs` holds the
  preferred parity rows (it runs the same edits on the browser host and the C ABI and compares
  them).
- Never-delete classes were kept: realtime and allocation gates, counts and ceilings, ABI, wire
  and corpus pins, regression reproducers, the sole red test of a MUTATIONS.md row, the acked-batch
  question, and tests a workflow or script runs by name.
- The integrator checked every DELETE defender: no defender is deleted or merged away by either
  reader; every host-core defender exists in the host-core read cut and again on the merged tree
  at `d85e38442`; every other defender exists on `main`. Then it built, removed what clippy
  reported unused, fixed live references, ran a mutation spot-check, and ran the full lint-job
  gate list.

## Result per file

Tests and test lines as the readers counted them (lines of the test function bodies).

| file | tests before -> after | kept (lines) | deleted (lines) | merged rows (lines) -> new |
|---|---|---|---|---|
| `src/tests.rs` | 113 -> 103 | 102 (6340) | 9 (306) | 2 (77) -> 1 |
| `src/ffi.rs` | 32 -> 31 | 30 (1915) | 0 | 2 (55) -> 1 |
| `tests/control_admission.rs` | 8 -> 4 | 4 (235) | 4 (356) | 0 |
| `tests/control_parity.rs` | 47 -> 43 | 43 (4025) | 4 (190) | 0 |
| `tests/stored_mute_automation.rs` | 4 -> 2 | 2 (88) | 2 (124) | 0 |
| 13 other files | 23 -> 23 | 23 (1504) | 0 | 0 |
| **total** | **227 -> 206** | **204 (14107)** | **19 (976)** | **4 (132) -> 2** |

The two merges:

- `tests::ordinary_spectrum_hops_reach_the_browser_stream_cadence` (rows: single capture with an
  explicit hop, collection with an explicit hop, single capture with no hop and the default
  cadence) replaces `ordinary_spectrum_single_and_collection_use_explicit_prepared_hops` and
  `ordinary_spectrum_omitted_hop_keeps_default_cadence`.
- `ffi::spectrum_ffi_tests::imported_stream_analysis_accepts_supported_hops` (rows: 48 kHz H256,
  48 kHz H1024, 96 kHz derived 3200) replaces `imported_stream_analysis_accepts_explicit_hops` and
  `imported_stream_analysis_accepts_high_rate_derived_default_hop`.

## Deleted tests and their defenders

| deleted test | defender (and parity row) |
|---|---|
| `tests::malformed_config_and_atomic_compile_failure_are_sticky` | `tests::each_boot_option_rule_has_its_own_typed_refusal`; bad document in `tests::empty_object_document_boots_to_a_diagnostic_not_a_trap` |
| `tests::source_backpressure_seek_render_and_stable_output_are_bounded` | `tests::raw_ffi_uses_stable_staging_and_exact_output_quantum_without_growth` |
| `tests::paused_seek_recycles_full_internal_queue_before_first_target_quantum` | `crates/source/src/lib.rs` `tests::paused_seek_prepares_full_queues_without_consuming_target`; row `control_parity::the_seek_report_is_alike_on_both_hosts_for_plain_late_and_future_seeks`; its one adapter assertion (the plain seek export refuses an unknown source with `RESULT_INVALID_ARGUMENT`) moved into `tests::the_anchored_seek_exports_play_at_the_anchor_and_report_where_it_landed` |
| `tests::a_live_control_free_host_refuses_every_live_kind_as_unsupported` | `tests::the_boot_word_is_reserved_and_observation_staging_follows_the_taps` |
| `tests::a_subscription_without_capacity_is_unsupported` | `tests::the_boot_word_is_reserved_and_observation_staging_follows_the_taps` |
| `tests::structural_apply::a_rebuild_swaps_with_a_spectrum_capture_or_a_collection_configured` | `crates/host-core/tests/browser_successor.rs` `every_branch_swaps_in_without_a_gap`; row `tests::structural_apply::a_rebuild_swaps_without_a_gap` |
| `tests::structural_apply::a_collection_selection_and_its_stream_continue_across_a_rebuild` | `crates/host-core/tests/successor_swap.rs` `spectrum_carry::a_reordered_collection_pairs_by_key_and_its_reader_keeps_receiving`; row `tests::structural_apply::a_running_spectrum_stream_continues_across_a_rebuild` |
| `tests::structural_apply::an_armed_one_shot_completes_across_a_rebuild` | `crates/host-core/tests/successor_swap.rs` `spectrum_carry::an_armed_one_shot_completes_across_the_swap`; same row |
| `tests::session_seek::a_stale_generation_changes_nothing` | `crates/host-core/src/source.rs` `tests::a_refusal_pushes_nothing_and_publishes_nothing`; row `tests::session_seek::one_full_slot_refuses_every_consumer` |
| `control_admission::every_live_row_commits_to_the_model_and_renders_as_a_fresh_plan` | `crates/host-core/tests/live_delta.rs` `pushed_records_render_the_rebuilt_plan` (and the send and VCA forms); row `control_parity::both_hosts_give_the_same_result_for_the_same_control_edits` |
| `control_admission::a_delay_samples_edit_is_a_rebuild_and_renders_as_a_fresh_plan` | `live_delta.rs` `a_rebuild_only_row_is_a_rebuild_with_its_reason`; same row |
| `control_admission::solo_is_the_monitor_overlay_and_a_later_live_edit_keeps_it` | `crates/host-core/tests/monitor_tick.rs` `a_monitor_write_applies_in_the_next_block_and_the_watermark_stays_on_the_revision`; same row |
| `control_admission::ramps_carry_end_to_end` | `live_delta.rs` `a_records_length_is_its_edits_own_when_the_edit_carried_one`; same row |
| `control_parity::the_status_words_carry_the_applied_revision_the_c_abi_reports` | `crates/host-core/tests/live_transaction.rs` `one_transaction_is_applied_whole_in_the_block_its_revision_is_published_for`; row `control_parity::the_status_words_carry_a_rebuilds_watermark_the_c_abi_reports` |
| `control_parity::every_scripted_edit_is_heard` | `live_delta.rs` `pushed_records_render_the_rebuilt_plan`; row `control_parity::both_hosts_give_the_same_result_for_the_same_control_edits` |
| `control_parity::a_following_send_leaks_nothing_of_a_stored_mute_on_both_hosts` | `crates/host-core/tests/route_mute.rs` `a_following_send_leaks_nothing_of_a_stored_mute`; row `control_parity::solo_and_a_vca_mute_compose_with_the_curve_for_a_following_send_on_both_hosts` |
| `control_parity::a_flat_vca_ride_and_mute_render_the_static_vca_bits_on_both_hosts` | `crates/host-core/tests/stored_vca_automation.rs` `a_flat_vca_ride_and_mute_render_the_static_vca_bits`; row `control_parity::a_linear_vca_ride_renders_alike_on_both_hosts` |
| `stored_mute_automation::a_mute_curve_mutes_at_its_exact_samples` | `crates/host-core/tests/stored_mute_automation.rs` `a_mute_step_mutes_the_lane_at_its_exact_sample_over_the_mute_length`; row `control_parity::a_composed_stored_mute_renders_alike_on_both_hosts` |
| `stored_mute_automation::solo_vca_and_hand_mutes_compose_with_the_curve` | `stored_mute_automation.rs` `a_hand_mute_on_an_automated_lane_composes_with_the_curve` (hand term) and `live_delta.rs` `a_mute_curve_takes_its_terms_as_mute_records`; the solo and VCA terms stay asserted against an oracle by the C ABI twin `capi::runtime::mute_automation_tests::solo_vca_and_hand_mutes_compose_with_the_curve`, which the capi read cut keeps; row `control_parity::a_composed_stored_mute_renders_alike_on_both_hosts` |

## Restores

No DELETE defender is deleted or merged away in either reader file, in the host-core cut or on the
merged tree, and the spot-check found no mutant that only a deleted test catches. The verifier
found two partial claims (strict rule: a defender must assert the whole claim):

- `stored_mute_automation::a_seek_into_a_muted_span_mutes_exactly_from_the_seek`: restored exactly
  as at `64fe78c46`, with its render-script helpers, and now KEEP. It is the only test of the
  post-seek curve step (the curve's next step lands on the new timeline after the seek); the
  host-core partner does not check it.
- `tests::paused_seek_recycles_full_internal_queue_before_first_target_quantum` stays deleted, but
  it held the only assertion that the plain seek refuses an unknown source with
  `RESULT_INVALID_ARGUMENT` (an adapter claim). That assertion is now in the kept adapter test
  `tests::the_anchored_seek_exports_play_at_the_anchor_and_report_where_it_landed`, through
  `miso_engine_web_v1_source_seek`. Red check: mapping the unknown source to `RESULT_WRONG_STATE`
  in `AudioWorkletEngineHost::seek_source` turns that test red at the new assertion (left 3,
  right 1); the mutant was reverted.

## Never-delete keeps by class

Readers' class labels on KEEP rows (one class per test): parity rows 52; adapter claims 30;
MUTATIONS.md sole red tests 24; realtime or allocation gates 16; counts or ceilings 13; ABI, wire
or corpus pins 11; the acked-batch question 7; regression reproducers 3; run by a workflow 1. The
other 46 KEEP rows name a unique claim without a class, plus the 1 verifier restore.

## Helper cleanup

Clippy (`--all-targets --all-features -D warnings`) reported the helpers that only the deleted
tests used; each was removed, then clippy ran clean. `src/tests.rs`: `effect_live_control_host`,
`capture_collection`, `other_track_id`, `two_entry_collection`. `tests/control_parity.rs`:
`browser_blocks`, `replaces`. `tests/control_admission.rs`: `Row`, `row`, `fixed_row`, `live_rows`,
`insert_row`, `console_row`, `route_row`, `vca_row`, `fresh_blocks`, `document_of`, `RampedEdit`,
`plane`, four constants and three imports. `tests/stored_mute_automation.rs`: `Term` and
its `SOLO` and `VCA_MUTE` constants (its one remaining use is now `hand_mute`); the render script
stays for the restored seek test. `tests/common/mod.rs` carries `#![allow(dead_code)]` at the base; no item in it
lost its last user in this cut.

## References fixed

- `tests/control_admission.rs` and `tests/stored_mute_automation.rs` headers now name what the
  file holds and where the deleted gates are held.
- `tests/control_parity.rs`: the doc of `both_hosts_give_the_same_result_for_the_same_control_edits`
  no longer claims that dropping any one edit changes a block (the deleted
  `every_scripted_edit_is_heard` held that); it says what the test checks now.
- `src/tests.rs`: the `#1290d2` spectrum comment names the host-core owner of the one-shot and
  collection carry.
- `.github/ISSUE_SPECS/1290-...`: gate 5, gate 7 collection and the armed one-shot citations say
  "deleted by the host-web read cut" and name the host-core defender.
- `tools/parameter-metadata/tests/round_trip.rs`: the doc of the host half now names
  `crates/host-core/tests/live_delta.rs` and `tests/control_parity.rs`.
- `hosts/host-web/MUTATIONS.md` names no deleted test. `scripts/`, `.github/workflows/` and
  `tools/` name none other than the one above. Dated records under `docs/handoffs/` and
  `docs/audits/` stay as written.
- Not edited here (owned by the capi read cut): `crates/capi/src/runtime/mute_automation_tests.rs`
  says, in its header and its script banner, that `hosts/host-web/tests/stored_mute_automation.rs`
  runs the same script and that the two copies are kept equal. After this cut the host-web file
  holds only the seek test and the control-list test. The capi cut or a successor should
  reword that sentence.

## Mutation spot-check

Seed `20261020`; `random.sample` of 30 from 2,224 mutants (`cargo mutants --list --json --diff`,
cargo-mutants 27.1.0, for `host-web` (566) and `host-core` (1,658), the owner of most twin claims)
whose function the 20 first-pass deleted tests covered (coverage map `cov.tsv`/`fns.tsv` at `81a3074fc`,
joined by file and function name). Each mutant was applied with `git apply` at the cut commit;
`cargo test --locked --all-features --no-fail-fast` ran for `host-web` (and `host-core` for a
host-core mutant); the files were reverted (signal-safe; the tree was clean at the end).

- 26 of 30 ran before the run reached its background time limit; mutants 26 to 29 did not run.
- 17 caught by a red remaining test. 4 unviable (do not compile).
- 2 timed out at 1,500 s (24: `strip_position` `<` to `<=`; 25:
  `SpectrumCapture::reset_after_cancel` `==` to `!=`). After them a host-core test binary
  (`collapse_arming`) was still running, so at least one of them hangs a test. They are not
  counted as caught.
- 5 not caught. For each, the 20 deleted tests (and the 4 merged originals) ran at `64fe78c46`
  with the mutant in place: none is red. So the cut lost no catcher; these are older gaps.

| # | mutant (line at `64fe78c46`) |
|---|---|
| 2 | `crates/host-core/src/control_provider.rs:1987` `<impl Hash for Shape>::hash` to `()` (likely equivalent: a constant hash changes cost, not results) |
| 7 | `crates/host-core/src/control_preparation.rs:215` `<` to `>` in `InputFilterPreparer::prepare` |
| 9 | `crates/host-core/src/prepare.rs:3239` `\|\|` to `&&` in `prepare_host_runtime_with_live_controls_policy_and_spectrum` |
| 12 | `hosts/host-web/src/lib.rs:5333` `>` to `>=` in `validate_options` |
| 18 | `crates/host-core/src/control_provider.rs:2011` `==` to `!=` in `Shape::of` |

## Gaps and successors

- `tests::finite_source_boundary_requires_only_the_actual_final_marker` stays in host-web. It is
  the only test of the early end-of-region marker refusal; host-core's `prepare.rs` tests cover
  only the unmarked final chunk. Its logic owner (host-core) lacks the test. Successor (TBD): add
  the host-core test, then delete the host-web one.
- `tests::a_subnormal_route_renders_as_a_zero_coefficient` stays in host-web. It is the only
  render-level test of #1237 gate 4 (a subnormal route product renders as +0);
  `graph-compiler`'s `route_coefficients.rs` checks only the constants. Its logic owner (the
  engine route path) lacks the test. Successor (TBD): add the engine test, then delete the
  host-web one.
- The five spot-check survivors and the two timeouts above. Successor (TBD): for each, find the
  claim and add one test in the owning crate (input filter ordering, the spectrum-carry and
  live-control branch in preparation, the boot-option bound in `validate_options`, the
  control-shape identity), or record the mutant as equivalent; find which test hangs under 24
  or 25.
- The capi mute-automation header sentence above.
- `stored_mute_automation::a_seek_into_a_muted_span_mutes_exactly_from_the_seek` stays in host-web
  as the only test of the post-seek curve step. Its logic owner (host-core) lacks it. Successor
  (TBD): add the host-core test, then delete the host-web one.

## Gates

- All 31 `run:` steps of the qualification `lint` job (fmt, workspace clippy `-D warnings`,
  `cargo doc -D warnings`, every hermetic policy gate and the three x86 lane-guard probes), from
  `policy-job-read-host-web.sh` pointed at this worktree, on the merged tree: PASS (31 of 31).
- `cargo test --locked --all-features -p host-web -p parameter-metadata` on the merged tree: PASS
  (223 passed, 0 failed). After the verifier fixes: `cargo test --locked --all-features -p
  host-web` PASS (204 passed, 0 failed), host-web clippy clean, and the 31 lint-job steps PASS
  again.

The per-test verdict list (227 rows) is posted as an issue comment.
