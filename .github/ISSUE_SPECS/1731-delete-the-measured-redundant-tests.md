# Delete the tests that mutation measurement found redundant (test audit TA4)

Owner approval: option A, relayed by root on 2026-10-20: delete what batch 1 measured redundant now,
sample batch 2, and end the audit with one PR (TA5) from that sample. Branch:
`codex/batch-test-policy-3`, from `main` at `b9135008d` (TA1 #1722, TA2 #1724 and TA3 #1726 merged).
Phase B PR 4 of the 2026-10 test audit.

## Method (batch 1)

Batch 1 is the 2026-10-19 stage 3 run at base `81a3074fc` (6.0 h; method and data kept with the
audit's working notes, `m/README.md`). Its candidates were the 180 reading-agent DELETE/UNSURE rows
outside the policy classes and the never-delete classes. cargo-mutants 27.1.0 mutants were drawn
from functions each candidate covers (stage 1 coverage map), up to 4 catches or 12 tries per
candidate; each viable mutant ran every candidate that covers its function and then the covering
KEEP tests. 392 mutants, 292 viable, 21 survivors.

- MEASURED-REDUNDANT: every mutant the candidate caught was also red in a KEEP test.
- REDUNDANT-IF-PEER-KEPT: some catches were made only by candidates; a greedy cover over those
  catches keeps 4 tests and drops the rest.
- UNIQUE: at least one mutant only this test caught. THIN, WEAK, NO-COVERAGE: too few tries or
  catches to judge.

MEASURED-REDUNDANT cannot see allocation, count or ceiling claims; tests whose claim is one of these
are kept or restored. The verifier's review restored three such tests (below, "Restored"). The
deleted capi `set_control::automated_vca` tests (`a_vca_move_over_an_automated_member_is_live_and_renders_the_prepared_plan`,
`vca_reach_and_count_changes_over_an_automated_member_are_live`, through the deleted helper `case`),
`set_control::automated_hand_move::a_hand_move_on_an_automated_fader_is_live_and_renders_the_stored_offset`
(through the deleted helper `leg`) and `set_control::automated_carry::an_exact_unmute_that_frees_a_held_lane_discharges_the_owed_move`
also asserted that every render block after the live edit makes 0 allocations and 0 frees on the
C ABI (`bench_support::alloc` thread counters). The same render path still has its allocation
counts in host-core: `tests/stored_vca_automation.rs` (`rig.allocations == (0, 0)` on the render
thread in its VCA ride, mute and carry tests) and `tests/stored_fader_automation.rs`
(`rendering_fader_events_allocates_nothing`, and the carry runs' swap-block counts).

"Redundant" means: no mutant of the sample needs this test. It is not a proof over all mutants,
and the KEEP catchers below are the recorded catchers of the sampled mutants, not of every row
mutation that a `MUTATIONS.md` names.

## What this change does

1. Deletes 137 of the 142 MEASURED-REDUNDANT tests: 2 no longer exist as measured and 3 are
   restored after review (both below, "Skipped" and "Restored").
2. Deletes the 8 REDUNDANT-IF-PEER-KEPT tests that the greedy cover drops. The cover keeps 4:
   `audit` `vectorization::tests::rust_disambiguator_suffix_is_narrowly_supported` and
   `parametric-eq` `malformed_requests_preserve_output_sentinels_and_opaque_ids` stay;
   `conformance` `tolerance_boundary_tie_and_snr_cases_are_explicit` and `effect-runtime`
   `the_mapping_inverse_round_trips` are UNIQUE only on test-only code and go with it (item 4).
   Deviation from root's brief, which said "delete 6": the data has 10 IF-PEER-KEPT rows. The
   cover keeps 2 of them; its other 2 kept tests are UNIQUE rows, not IF-PEER-KEPT. So the cover
   drops 8, and all 8 are deleted. Evidence per test (its candidate-only mutants, and who still
   catches them):
   - `conformance` `scaled_sum_squares_handles_extreme_finite_values` (m9174, m9175, m9176,
     m9193) and `block_metrics_and_repeat_bits_cover_boundaries` (m9120, m9121, m9122): every
     mutant is in `compare.rs`, which item 4 deletes; `tolerance_boundary_tie_and_snr_cases_are_explicit`
     also caught each one.
   - `effect-runtime` `mapping_is_exact_at_the_ends_and_monotone`,
     `a_logarithmic_mapping_is_constant_ratio` (m17453, m17454) and
     `a_degenerate_range_has_no_position` (m17453, m17454, m17480): every mutant is in
     `params::map_normalized`/`inverse_map_normalized`, which item 4 deletes;
     `the_mapping_inverse_round_trips` also caught each one.
   - `audit` `near_name_impostor_does_not_satisfy_probe` and
     `exact_probe_path_and_body_boundaries_are_preserved` (m4430): also red in the kept
     `rust_disambiguator_suffix_is_narrowly_supported`.
   - `parametric-eq` `common_eq_adapter_preserves_input_ownership_capacity_and_refusal_sentinels`
     (m35416): also red in the kept `malformed_requests_preserve_output_sentinels_and_opaque_ids`.
3. Keeps every UNIQUE (except item 4), THIN, WEAK and NO-COVERAGE test.
4. Deletes code that only tests call, with its UNIQUE tests. Proof: `git grep` over `crates/`,
   `hosts/` and `tools/` outside test code finds no caller, and the workspace builds warning-free
   without it (`cargo clippy --workspace --all-targets --all-features -- -D warnings`).
   - `conformance` `PlanarBlock`/`BlockError` (`src/block.rs`, 68 lines), `compare_f32_to_f64` and
     its report types (`src/compare.rs`, 235), `verify_bit_exact_repeat`/`DeterminismError`
     (`src/determinism.rs`, 34): only the crate's own tests used them. Deleted with
     `tests/api_metrics.rs` (100; UNIQUE `tolerance_boundary_tie_and_snr_cases_are_explicit` and
     `invalid_rates_shapes_tolerances_and_nonfinite_values_are_typed`), the lib test
     `block_metrics_and_repeat_bits_cover_boundaries`, and the two `PlanarBlock` assertions of
     the KEEP test `fixtures.rs` `planar_blocks_and_fixtures_accept_only_launch_rates`, renamed
     `fixtures_accept_only_launch_rates` (its fixture assertions stay).
   - `effect-contract` `map_normalized`, `inverse_map_normalized`, `map_stepped_normalized`,
     `inverse_map_stepped_normalized` (`src/lib.rs:1220-1285`): no product caller. Deleted with
     UNIQUE `conformance` `normalized_mapping_endpoints_ties_and_round_trips_are_stable`.
   - `effect-runtime` `params::map_normalized`, `params::inverse_map_normalized`
     (`src/params.rs:182-229`): no product caller. Deleted with UNIQUE
     `the_mapping_inverse_round_trips` and its three IF-PEER-KEPT peers. The
     `check-effect-runtime-policy.sh` ratchet rows for the pair go to 0.
   - `lane` `CanonicalFpEnv::saved_word` (`src/fpenv.rs:533-538` and `:590-593`): no product
     caller. Deleted with UNIQUE `fp_env.rs` `the_canonical_word_is_the_word_inside_the_guard`
     (its install and restore assertions are the KEEP tests' catches).
   - Kept: `controls` `ControlDescriptor::range_contains` (UNIQUE `domain_boundaries`),
     `audit` `is_encoding_token` and `symbol_header` (UNIQUE `call_inside_kernel_body_is_red`,
     `malformed_prose_and_instruction_prefixes_do_not_own_probe_body`) and `builtins`
     `PreparedInputFilterResponse::retained_bytes` (UNIQUE
     `common_builtin_adapter_preserves_resource_and_refusal_contract`; host-core `response.rs`
     calls it through the trait): product code calls each of them.
5. Deletes helpers only after the compiler reported them unused (dead_code or unused imports under
   clippy `--all-targets --all-features`), looped until it reported nothing. No
   `#[allow(dead_code)]` was added. Test files left with no test are deleted whole.
6. Updates the references: each `MUTATIONS.md` row or doc line that names a deleted test now says
   "deleted by TA4". It names a KEEP catcher only where batch 1 recorded that catcher red on the
   deleted test's mutants in the same crate and it can plausibly see the row's mutation; otherwise
   it names none. Each such `MUTATIONS.md` ends with a "TA4 deletions" list. Open specs that name a deleted test or file
   (#1290, #1350, #1406, #1654, #1696, #1722) say so in place.

## Non-goals

- No product render, prepare or control path changes. Item 4 deletes only code no product path
  calls.
- No new tests. The 21 survivors are TA4's other half (agent T4S, merged in later).
- Dated records under `docs/handoffs`, `docs/audits` and `artifacts/` and the rename table in
  `docs/rulings/de-versioning-inventory.md` are history and are not edited.

## Objective gates

Every `run:` step of the qualification `lint` job (fmt, workspace clippy `-D warnings`,
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, the hermetic policy gates),
and `cargo test --locked --all-features -p <package>` for every touched package.

## Evidence

Line numbers are at `b9135008d`. Each test lists the number of sampled mutants it caught and the
KEEP tests that were also red on them (count per catcher; "other candidates only" are the cover's
peers, "timeout" is a 20-minute KEEP run counted as caught).

### Deleted tests (150, 6,905 lines)

Tests per package: audit 7; bench 1; builtins 7; builtins-compiler 2; capi 18; compressor 7; conformance 6; console-workload 7; delay 1; effect-contract 6; effect-runtime 12; engine 2; gate-expander 4; graph 3; graph-compiler 8; host-web 9; lane 8; math 2; multiband-compressor 1; parametric-eq 10; protocol 5; realtime-policy 4; session 2; session-validator 3; soft-clip 3; source 9; transient-shaper 1; true-peak-limiter 2.

The list of each deleted test, with its location at `b9135008d`, its caught mutants and the KEEP
tests that were also red on them, is too long for an issue body. It is in the two comments on this
issue ("Deleted tests, part 1/2" and "part 2/2"), and the PR diff is the list of what went.

### Deleted test files (whole)

- `crates/conformance/src/block.rs` (68), `src/compare.rs` (235), `src/determinism.rs` (34),
  `tests/api_metrics.rs` (100): item 4.
- `crates/soft-clip/tests/partition_invariance.rs` (118), `crates/source/tests/seek_schedule_model.rs`
  (533), `hosts/host-web/tests/retained_ceilings.rs` (294), `tools/session-validator/tests/skill.rs`
  (111): every test in them is listed above; their helpers go with them. `retained_ceilings.rs`
  held the native budgets of the browser identity fixture: the shipped wasm32 artifact's ceilings
  stay in `expected.json` (checked by `check-browser-expected-resources.py`); the deleted file
  ceilinged only the native 8- and 4-lane builds, which do not ship. host-web's `lane`
  dev-dependency served only it and the deleted `canonical_live` module, so it goes too
  (`Cargo.lock` loses that edge).

### Deleted helpers (compiler-reported, 71 items, 2,041 lines)

- `Command`, `crates/builtins/tests/stage.rs:860-883`
- `automation_script`, `crates/builtins/tests/stage.rs:885-960`
- `render_banked`, `crates/builtins/tests/stage.rs:962-1054`
- `render_per_track`, `crates/builtins/tests/stage.rs:1056-1117`
- `document`, `crates/capi/src/runtime/live_tests.rs:8409-8417`
- `vca_at`, `crates/capi/src/runtime/live_tests.rs:8419-8425`
- `writes`, `crates/capi/src/runtime/live_tests.rs:8427-8445`
- `case`, `crates/capi/src/runtime/live_tests.rs:8447-8516`
- `fader_length`, `crates/capi/src/runtime/live_tests.rs:8518-8525`
- `fader_rows`, `crates/capi/src/runtime/live_tests.rs:9867-9879`
- `Leg`, `crates/capi/src/runtime/live_tests.rs:9881-9888`
- `leg`, `crates/capi/src/runtime/live_tests.rs:9890-9931`
- `follows_mute_edit`, `crates/capi/src/runtime/live_tests.rs:11380-11394`
- `route_entry`, `crates/capi/src/runtime/live_tests.rs:11500-11519`
- `a_reverted_owner_carries_from_p0`, `crates/capi/src/runtime/supersede_tests.rs:818-842`
- `upsert_added_source`, `crates/capi/src/runtime/supersede_tests.rs:1267-1286`
- `remove_source_track`, `crates/capi/src/runtime/supersede_tests.rs:1288-1299`
- `FreshSourceCase`, `crates/capi/src/runtime/supersede_tests.rs:1301-1313`
- `fresh_source_cases`, `crates/capi/src/runtime/supersede_tests.rs:1315-1354`
- `FRAMES`, `crates/compressor/tests/lane_identity.rs:24-24`
- `QUANTUM`, `crates/compressor/tests/lane_identity.rs:25-25`
- `track_values`, `crates/compressor/tests/lane_identity.rs:27-40`
- `every_parameter`, `crates/compressor/tests/partition.rs:26-45`
- `OVERFLOWING_KNEES`, `crates/effect-runtime/tests/dynamics.rs:237-246`
- `NARROWEST_SOFT_KNEES`, `crates/effect-runtime/tests/dynamics.rs:248-255`
- `track_values`, `crates/gate-expander/tests/identity.rs:12-29`
- `render_bank_native`, `crates/gate-expander/tests/identity.rs:31-70`
- `compile_bank_and_per_node`, `crates/graph-compiler/src/lib.rs:1141-1196`
- `taps`, `crates/graph-compiler/tests/compile_shapes.rs:376-386`
- `representative_console`, `crates/graph-compiler/tests/compile_shapes.rs:388-469`
- `assert_estimate_moves_by_the_route_activity_charge`, `crates/graph-compiler/tests/route_coefficients.rs:544-578`
- `assert_width_matches_reference`, `crates/graph/src/runtime.rs:12313-12427`
- `compare_holey_population`, `crates/lane/tests/fader_matrix.rs:430-502`
- `run_scalar`, `crates/lane/tests/fader_matrix.rs:510-543`
- `pair_cases`, `crates/lane/tests/g4_flush.rs:262-350`
- `invalid_lane_at`, `crates/lane/tests/meter_peak.rs:178-220`
- `F1_SHAPER_LEVEL_MIN`, `crates/math/tests/f1_fast_db_bounds.rs:561-561`
- `worst_over_run`, `crates/math/tests/f1_fast_db_bounds.rs:866-901`
- `crossing_worst_db`, `crates/math/tests/f1_fast_db_bounds.rs:903-925`
- `original_four`, `crates/parametric-eq/src/lib.rs:4238-4248`
- `process_original_four`, `crates/parametric-eq/src/lib.rs:4250-4261`
- `process_original_four_ramped`, `crates/parametric-eq/src/lib.rs:4263-4303`
- `all_high_pass_channel`, `crates/parametric-eq/src/lib.rs:4305-4330`
- `original_state_bits`, `crates/parametric-eq/src/lib.rs:4332-4346`
- `compare_disabled_cuts_to_original_four`, `crates/parametric-eq/src/lib.rs:4348-4456`
- `compare_signed_zero_refusal`, `crates/parametric-eq/src/lib.rs:4458-4548`
- `disabled_cuts_general_ramp_parity`, `crates/parametric-eq/src/lib.rs:4828-4894`
- `tiny_disabled_cut_state`, `crates/parametric-eq/src/lib.rs:4906-4944`
- `TAIL`, `crates/parametric-eq/tests/carry.rs:34-35`
- `bank_tail`, `crates/parametric-eq/tests/carry.rs:130-188`
- `refusal_codes`, `crates/protocol/tests/vca_edits.rs:278-297`
- `Refusal`, `crates/protocol/tests/vca_edits.rs:299-304`
- `Rendered`, `crates/transient-shaper/tests/partition.rs:34-35`
- `FADER_RAMP`, `hosts/host-web/src/tests.rs:7063-7065`
- `live_fader`, `hosts/host-web/src/tests.rs:7067-7097`
- `seek_ride`, `hosts/host-web/src/tests.rs:10269-10297`
- `seek_ride_blocks`, `hosts/host-web/src/tests.rs:10299-10317`
- `MEMBER_DB`, `hosts/host-web/src/tests.rs:10545-10546`
- `OFFSET_DB`, `hosts/host-web/src/tests.rs:10547-10547`
- `MEMBER`, `hosts/host-web/src/tests.rs:10548-10549`
- `CURVE_DB`, `hosts/host-web/src/tests.rs:10550-10554`
- `VCA_DB`, `hosts/host-web/src/tests.rs:10555-10555`
- `HAND_DB`, `hosts/host-web/src/tests.rs:10556-10556`
- `PRIOR_HAND_DB`, `hosts/host-web/src/tests.rs:10557-10557`
- `model`, `hosts/host-web/src/tests.rs:10559-10567`
- `set`, `hosts/host-web/src/tests.rs:10569-10581`
- `frame`, `hosts/host-web/src/tests.rs:10583-10599`
- `applied`, `hosts/host-web/src/tests.rs:10601-10614`
- `live_move_under_hostile_word`, `hosts/host-web/src/tests.rs:10616-10683`
- `automated`, `hosts/host-web/src/tests.rs:10766-10792`
- `benchmark_text_mutation`, `tools/audit/src/fixture_builtins.rs:5752-5763`

Also: the `parametric-eq`
`carry.rs` `Ramp::name` field (read only by deleted tests; each ramp keeps its name as a comment);
the emptied `host-web` `tests::canonical_live` module.

Lead for a successor, not fixed here (the compiler does not report it): `builtins`
`MATRIX_SELECT_FREE_BLOCKS` and `FUSED_SELECT_FREE_BLOCKS` (`crates/builtins/src/lib.rs`) are
`#[cfg(test)]` counters that are now written and never read. Their readers were the deleted
`settled_matrix_takes_the_select_free_arm_only_without_an_identity_lane` and
`fused_fader_matrix_takes_the_select_free_arm_only_without_an_identity_lane`.

### Restored after review

The TA4 verifier (READY WITH FIXES) restored these exactly as at `b9135008d`:

- `graph` `program::tests::a_bank_window_never_recycles_a_physical_slot` (the #169 minimal
  reproduction; a regression reproducer is never-delete) and
  `no_slot_is_recycled_inside_a_merged_bank_window`, with their helpers `dynamic_track` and
  `dynamic_effect` (`crates/graph/src/program/tests.rs`), and the `program.rs` doc line that cites
  the first.
- `rack` `tests::prepared_slot_dispatch_uses_constant_activity_checks` (a count claim: the dispatch
  makes a constant number of activity checks, which mutation cannot see), with the `#[cfg(test)]`
  `TEST_PREPARED_ACTIVITY_QUERIES` counter, its increment in `PreparedSlot::has_active_lanes` and
  its readers `reset_prepared_activity_observation` and `prepared_activity_observation`.

### The browser hostile-FP live gate

TA4 deleted host-web `tests::canonical_live::a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits`
(#1709 D2). The capi test of the same name stays (`crates/capi/src/runtime/live_tests.rs:8158`),
so the browser side of that check is gone. `1729-test-the-fp-environment-refusal.md`
records it in case the owner wants the browser side back.

### Skipped

- `graph-compiler` `live_routes.rs` `route_control_resources_cover_the_allocation` and
  `gate-expander` `contract.rs` `bypass_is_exact_identity_at_all_rates`: TA1 trimmed the first and
  TA2 rewrote the second after the measurement, so the measured test no longer exists. Both stay.
- The cover's kept `audit` `rust_disambiguator_suffix_is_narrowly_supported` and `parametric-eq`
  `malformed_requests_preserve_output_sentinels_and_opaque_ids`.

## Survivor gaps

Batch 1 left 21 survivors: viable mutants that no candidate and no covering KEEP test turned red.
Agent T4S classified each one (branch `codex/batch-test-gaps-ta4`, merged here):

- EQUIVALENT 6: m38090 (rack `BankStage::begin_block` default body), m26403 (lane
  `scheduling_barrier`, an empty `asm!`), m26693 (lane scalar select: disjoint bits, so `|` is `^`),
  m4419 (audit `vectorization.rs:336`, the next clause catches it), m35318 and m6786
  (`Complex::is_finite` in parametric-eq and builtins responses: a later finite check catches
  every non-finite value).
- UNREACHABLE 3: m17240 and m17239 (effect-contract `ExactDecimal::parse`; its one product caller
  passes canonical digits only) and m26408 (lane `attest_fp_environment` refusals, which need a
  control word that does not read back). m26408 goes to
  `1729-test-the-fp-environment-refusal.md` (a pure decision seam and its test).
- DEAD CODE 7: m9108, m9109, m9111, m9118 (`conformance` `PlanarBlock`, `compare_f32_to_f64`) and
  m16329, m16330 (`effect-contract` `map_stepped_normalized`) are in code that item 4 deleted.
  m42034 (`source` `validate_output_shape`, reached only through the test-only `read_block`) goes
  to `1730-remove-test-only-public-functions.md`.
- REAL GAP 5, closed by 4 new tests. Each was red with its mutant applied and is green on the branch:
  - m18792, gate-expander `tests/state.rs::a_subnormal_gain_word_is_refused` (red at
    `state.rs:886`). A restore that stops refusing a subnormal gain word (`normal_or_zero`
    dropped or weakened) turns it red; no other test feeds a gain word outside the normal/zero set.
  - m41330 and m41333, session-validator
    `tests/validate.rs::the_report_labels_each_stage_and_prints_every_message_line` (red at the
    label assertion, `validate.rs:487`, and at the multi-line assertion, `:512`). A report that
    loses its stage labels or drops message lines turns it red; the other tests only compare two
    renders or search stderr for the result line.
  - m39691, realtime-policy `tests/policy/drain/forms.rs` row `count-in-an-expression-macro` (red in
    `a_count_that_the_rule_cannot_prove_is_refused`). A D7 check that stops seeing a count used
    inside a non-assert macro in expression position turns it red.
  - m39758, realtime-policy `tests/policy/drain/outer.rs` row
    `array-from-fn-after-a-field-named-std` (red in
    `a_constructor_that_loops_is_refused_anywhere_in_a_region`). An `array::from_fn` exemption that
    matches a path with a single `:` turns it red.

Successors: `1729-test-the-fp-environment-refusal.md` and
`1730-remove-test-only-public-functions.md` (now only the `source` `read_block` item).
