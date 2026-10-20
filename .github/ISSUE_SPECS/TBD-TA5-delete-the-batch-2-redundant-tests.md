# Delete the redundant tests of the batch 2 sample (test audit TA5)

Owner approval: option A, relayed by root on 2026-10-20: delete what batch 1 measured redundant
(TA4, #1731), sample batch 2, and end the audit with one PR (TA5) from that sample. This is that
PR, the last of the 2026-10 test audit. Branch `codex/batch-test-policy-4`, from `main` at
`ad5c7ae86` (TA1 to TA4 merged).

## Method (batch 2)

Batch 2 is agent B2's 2026-10-20 run at base `81a3074fc` (8.3 h; method and data kept with the
audit's working notes, `b2/README.md`). Pool: the 3,516 batch 1 KEEP tests minus the 9 TA3
deletions and the 239 never-delete tests (`candidates.tsv` flags plus a name rule: `alloc`, `abi`,
`layout`, `golden`, `wire`, `g5`, `regression`/`reproducer`, `fuzz`), 3,268 tests. Draw:
`random.Random(20261020).sample(sorted(ids), 200)`. Mutants: cargo-mutants 27.1.0, from functions
each sampled test covers, up to 4 catches or 12 viable tries per test (median 12); each mutant ran
every sampled test that covers its function, then the covering KEEP tests (fail-fast, first red
test recorded). 414 mutants, 321 viable, 12 survivors.

## Result

- MEASURED-REDUNDANT (every caught mutant also red in a KEEP test): **162 of 200, 81.0%, 95%
  Wilson 75.0% to 85.8%**. The other 38: 11 CLAIM-KEPT, 5 HELD-RANDOMIZED, 6 UNIQUE, 14 THIN, 2
  WEAK.
- The projection onto the pool (about 2,647 tests) is an upper bound per test, not a sum: deleting
  many pool tests at once removes catchers of each other. The 162 are safe together because no
  sampled test was a catcher of another.

## The blind spot, and how it was handled

Mutation cannot see allocation counts, exact count claims or ceilings, and a sample of about 12
random mutants per test can miss a claim. B2 read the 178 mutation-redundant bodies (regex triage
for allocation, counts, ceilings, bytes, reproducers, randomness) and held 11 as CLAIM-KEPT and 5 as
HELD-RANDOMIZED; those are not in the 162. This PR then read each of the 162 and answered the
AGENTS.md question, "which plausible defect turns this red that no remaining test catches?". A
test is kept when the answer names a real edge case that the measured mutants could not reach and
no remaining test asserts (the remaining tests checked are named per test); otherwise it is
deleted. Reading found what the triage missed: a randomized differential named `run_seeds`, an
allocation audit inside a macro, three tests that a `MUTATIONS.md` records as the only red test of
a row, and one test that `scripts/lib/aarch64-known-defects.py` lists as a known AArch64 failure.

## What this change does

1. Deletes 131 of the 162 tests and keeps 31 by reading (list below). None of the 162 had moved
   or gone since `81a3074fc`; nothing was skipped.
2. Deletes 48 helper items (984 lines) only after `cargo clippy --all-targets --all-features`
   reported them unused, looped until it reported nothing; no `#[allow(dead_code)]` was added.
   Deletes two test files left with no test: `crates/host-core/tests/strip_handles.rs` and
   `crates/transient-shaper/tests/partition.rs`.
3. Updates the references: each `MUTATIONS.md` row that names a deleted test names a remaining
   same-crate catcher that batch 2 recorded red on that test's sampled mutants, or says "(deleted
   by TA5; no catcher recorded)". No list of deleted tests is added to a tracked file. Code
   comments and open specs (#1339, #1340, #1586, #1713) note a deleted name only where it would
   otherwise be a stale reference. Recorded log output and dated records (`docs/handoffs`,
   `docs/audits`, `artifacts/`) are not edited.
4. Gives the `qualification` job `test-debug-a` ("workspace debug tests (minus DSP crates and
   evidence tools)") `timeout-minutes: 22` in place of 15: 1.5 times the slowest recent green run
   (14:41). This refers to #1728 and does not close it; #1728's gate (ten consecutive green runs
   with at least 25% margin) stays open. No other timeout moves.

## Per package

| package | deleted | kept by reading | deleted test lines |
|---|---|---|---|
| audit | 0 | 1 | 0 |
| automation | 1 | 0 | 30 |
| bench-support | 1 | 0 | 5 |
| builtins | 5 | 2 | 406 |
| builtins-compiler | 0 | 1 | 0 |
| capi | 9 | 1 | 603 |
| compressor | 3 | 0 | 252 |
| console-workload | 2 | 0 | 83 |
| control-plane | 0 | 1 | 0 |
| controls | 3 | 0 | 117 |
| delay | 1 | 0 | 12 |
| dsp-reference | 1 | 1 | 15 |
| effect-compiler | 2 | 1 | 38 |
| effect-contract | 2 | 1 | 46 |
| effect-runtime | 5 | 1 | 97 |
| engine | 3 | 0 | 139 |
| gate-expander | 0 | 2 | 0 |
| graph | 4 | 0 | 326 |
| graph-compiler | 7 | 0 | 590 |
| host-core | 30 | 3 | 1,270 |
| host-web | 5 | 3 | 328 |
| lane | 4 | 1 | 76 |
| math | 2 | 1 | 23 |
| multiband-compressor | 3 | 0 | 151 |
| parametric-eq | 5 | 2 | 464 |
| protocol | 5 | 2 | 343 |
| rack | 5 | 1 | 322 |
| realtime-policy | 11 | 2 | 177 |
| session | 2 | 1 | 34 |
| soft-clip | 2 | 0 | 126 |
| source | 3 | 0 | 115 |
| transient-shaper | 3 | 1 | 197 |
| true-peak-limiter | 2 | 2 | 63 |
| **total** | **131** | **31** | **6,448** |

Lines are the deleted test functions with their doc comments, at `ad5c7ae86`. Shortstat of the
deletion commit: 107 files changed, 129 insertions, 7,783 deletions (tests, helpers, imports, the
two files, and the reference updates).

## Kept by reading (31)

- audit `fixture_builtins::tests::check_rejects_owned_jsonl_tuple_mutations`: a field or record
  change in an owned JSONL after the manifest is regenerated; the format test only deletes,
  appends a byte or empties.
- builtins `a_long_matrix_move_stays_inside_its_endpoints`: the #1408 reproducer at the matrix
  call site; the D11 reference test never overshoots and the lane test checks the law only.
- builtins `a_windowed_move_is_monotone_and_lands_exactly_on_its_target`: the D11 exact-assignment
  leg; the endpoint and partition siblings pass without it.
- builtins-compiler `composite_drain_applies_the_other_stages_value_when_one_stage_is_refused`: the
  acked-batch question on the fused fader+matrix drain; the sibling covers the input drain only.
- capi `capi_retained_bytes_charge_every_byte_the_compile_retains`: a resource-charge claim over
  four sessions, and the only caller of `assert_host_owners_are_charged`.
- control-plane `no_room_for_the_end_refuses_the_removal_and_changes_nothing`: the acked-batch
  question in the control plane (commit and drop the end event); the only one there.
- dsp-reference `gate_expander::tests::a_level_inside_the_hysteresis_band_keeps_reloading_the_hold`:
  the only red test of gate-expander `MUTATIONS.md` row 3 (the in-band reload).
- effect-compiler `a_session_bypass_lowers_to_an_enabled_effect_and_a_bypassed_lane`: the only pin
  of the constants `NEVER_BANKED_EFFECTS`, `PREPARED_BYPASS_EFFECTS` and the lowered set; no
  mutant edits a constant.
- effect-contract `a_drain_emits_the_contract_canonical_span_order`: the only red test of
  effect-contract `MUTATIONS.md` 140-1; the block validator is not on the render path.
- effect-runtime `subnormal_targets_are_hoisted_without_flushing`: the denormal rule; the siblings
  have no subnormal word.
- gate-expander `passes_effect_contract_conformance`: the conformance macro's `process.allocation`
  audit, the crate's only render allocation check.
- gate-expander `a_moving_ramp_word_past_its_domain_is_refused`: the one-ulp domain edge with
  `effect.state.parameter`; no other gate-expander test refuses with that code.
- host-core `a_bus_renders_the_bits_of_a_track_fed_its_sum`: a randomized differential (32 seeds of
  drawn strips, effects and routes); the only bus-equals-track-fed-its-sum check.
- host-core `a_hand_move_on_an_automated_fader_writes_the_lanes_offsets_cell`: the D6 summation
  order (hand offset first, then the VCAs by ID); a reorder is not a mutant.
- host-core `the_session_shape_measures_the_longest_vca_id`: the only assertion of
  `longest_vca_id_bytes`, a staging-size ceiling.
- host-web `ffi::live_response_ffi_tests::live_response_owners_name_console_slots_inserts_and_input_filters`:
  the browser ABI's rack codes 0, 2 and 4 and the refusal of the retired 1 and 3.
- host-web `tests::each_boot_option_rule_has_its_own_typed_refusal`: the `struct_size` and
  `reserved0` ABI refusals; other tests cover `abi_version` and the sizes only.
- host-web `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix`: the meter
  frame's `3(T + S) + 3` word layout with submixes, which the worklet reads.
- lane `g4_silence_skip_block_saturates_a_long_trailing_count`: the 2^24 clamp, reachable only
  with a 2^24 + 3 frame block.
- math `m2_exp2_lane_identity`: a known AArch64 release failure (LANE-3, #1019) named in
  `scripts/lib/aarch64-known-defects.py`; x86 mutants cannot reach it.
- parametric-eq `a_non_finite_input_block_is_zeroed_counted_once_and_leaves_the_next_block_clean`:
  the count claim (one block on one channel, not one per sample).
- parametric-eq `svf_words_match_the_independent_oracle_on_the_complete_grid`: the only gate on the
  A1 storage (parametric-eq `MUTATIONS.md`, recorded failures 1 and 1b); its 10 Hz, Q = 18,
  88.2 kHz row is on no other grid.
- protocol `controller::tests::canonical_json_snapshots_reparse_before_and_after_commit_across_utf8_split_pages`:
  snapshot pages are byte ranges that may split a UTF-8 sequence; the only such test.
- protocol `controller::tests::the_end_needs_room_with_the_commit_and_a_dropped_token_releases_it`:
  the acked-batch question for the end event's reservation, release and `Unencodable`.
- rack `re_equal_words_after_a_desymmetrised_episode_do_not_re_engage`: the only negative of
  `collapse_channels_agree`; the siblings assert re-engagement.
- realtime-policy `drain::outer::an_item_that_declares_the_collection_root_refuses_the_drain`: the
  reproducers of verdict rows r01 and b06.
- realtime-policy `render_wrap::an_unwrapped_entry_outside_the_named_exceptions_is_a_finding`: the
  only test that produces `UnwrappedNotPermitted`.
- session `route_mute_is_a_required_boolean`: a schema rule (a missing `mute` must not parse as
  on); the only test of the key.
- transient-shaper `a_moving_ramp_word_past_its_domain_is_refused`: the one-ulp edge; the sibling
  refuses gross values only.
- true-peak-limiter `every_case_is_finite_and_not_vacuous`: the guard of the limiter's G5 corpus,
  which a digest pin cannot give (NaN payloads are canonicalised on wasm).
- true-peak-limiter `production_tracks_the_f64_oracle`: the only independent `f64` oracle of the
  limiter gain law (E5); E4 bounds the peak only.

## Deleted helpers (compiler-reported)

- `crates/capi/src/runtime/live_tests.rs`: `live_alone`
- `crates/controls/tests/monitor.rs`: `FADER_DB`
- `crates/dsp-reference/src/svf.rs`: `IMPULSE_FRAMES`
- `crates/engine/src/realtime/spsc.rs`: `fail_if_stalled`, `STALL_DEADLINE`, `EndedOnDrop` and its
  `Drop`
- `crates/graph/src/runtime.rs`: `HostShape` and its impl, `render_arena_oracle`,
  `fill_output_slot`, `output_slot`, `stereo_bits`, `OUTPUT_SLOT_FILL`, `direct_bank_parts`,
  `test_only_set_host_master_declined`
- `crates/host-core/src/prepare.rs`: `instance`; `tests/browser_successor.rs`:
  `session_without_eq1`; `tests/live_delta.rs`: `with_effect_bus`; `tests/spectrum.rs`:
  `render_window`; `tests/successor_swap.rs`: `a_repointed_route_does_not_carry_its_line`,
  `two_bus_session`
- `crates/lane/tests/f64_lane.rs`: `square_mismatches`, `square_pool`, `oracle_square_bits`;
  `tests/fader_matrix.rs`: `fused_select_free_width`, `FUSED_FRAMES`,
  `fused_select_free_differences`, `FusedDifferences` and its `record`, `FUSED_MUTES`,
  `FUSED_GAINS`, `fused_family_source`; `tests/input_chain_arming.rs`: `every_state_word_arms`
- `crates/math/tests/m2_lane_identity.rs`: `exp2_caller_edges`, `FastGain` and its `LaneFn`
- `crates/multiband-compressor/tests/crossover.rs`: `rms`, `complex_gain`, `CROSSOVERS`
- `crates/parametric-eq/tests/carry.rs`: `same_bits`; `tests/exact_rest.rs`: `at_rest`,
  `HELD_BLOCKS`, `REST_BOUND_SAMPLES`
- `crates/rack/src/lib.rs`: `WrongWidthPlanes` and its `BankMembers`
- `hosts/host-web/src/ffi.rs`: `test_seek_report_addresses`

Leads the compiler does not report are in the successor spec
`TBD-TA5-successor-audit-leads.md`.

## Objective gates

Every `run:` step of the qualification `lint` job (fmt, workspace clippy `-D warnings`,
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, the hermetic policy gates),
and `cargo test --locked --all-features -p <package>` for every touched package.

## Evidence

The per-test list (package, test, location, the one-line answer, and the KEEP catchers batch 2
recorded) is too long for an issue body: per-test list with catchers: issue comments ("TA5
per-test list, part 1/3" to "part 3/3"). The PR diff is the list of what went.

## Survivor gaps

Batch 2 left 12 mutants that no sampled and no covering KEEP test turned red. Agent T5S read each
at `ad5c7ae86`: 3 equivalent, 1 unreachable, 2 dead code, 6 real gaps. The six real gaps (one
small test each) and the decision on the dead contract are the successor spec
`TBD-TA5-gap-close-the-batch-2-mutation-survivor-gaps.md`.
