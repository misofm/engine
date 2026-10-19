# Finish the test policy cut: the five owner answers to TA1's asks (test policy cut 2)

Owner answers: the five open questions of `.github/ISSUE_SPECS/1722-delete-digest-type-size-and-unrun-tests.md`
("ASK"), answered by the owner and relayed by root on 2026-10-19. Branch:
`codex/batch-test-policy-2`, from `main` at `ce714017c` (TA1 #1722 / PR #1723), with `origin/main`
`fdb1c44c2` (TA2 #1725) merged in before the gates. Phase B PR 2 (TA3) of the 2026-10 test audit.

## The owner answers

1. **ASK 1, the last three scalar-oracle pairing tests: option A.** Delete them and the test-only
   initial-matrix trace. The oracle's render path stays.
2. **ASK 2, `math` m2 `EXP2_DIGEST`/`LOG2_DIGEST`: keep.** It stays until #1019 is fixed; the PR that
   fixes #1019 deletes it together with its `scripts/lib/aarch64-known-defects.py` row.
3. **ASK 3, the host-core `limiter_linked_session` `PIN`:** if G5 already covers the case, delete
   the pin and cite the covering entry; if not, add the case to G5 in the corpus's own way, then
   delete the pin, and point true-peak-limiter `MUTATIONS.md` row 996-K2 at the new owner.
4. **ASK 4, `gain_pan_profile` `phase_profile`: delete.** Then delete the graph phase-profile probes
   too if only tests or test-support use them.
5. **ASK 5, the one-line `size_of` / byte-count re-derivations inside larger tests: trim (class (b),
   approved).** Keep real resource ceilings, allocation counts and ABI layouts. A line that guards a
   relation the product enforces becomes that relation without `size_of` of an internal type.

## Non-goals

- No change to product render paths, ceilings the product enforces, C ABI or browser layouts, wire
  goldens, fuzz targets, randomized differentials, allocation counts, or open-defect reproducers.
- `crates/math/tests/m2_lane_identity.rs` is not touched (answer 2).

## Objective gates

1. `cargo fmt --all --check`.
2. `cargo clippy --locked -p <each touched package> --all-targets --all-features -- -D warnings`.
3. `cargo test --locked --all-features -p <each touched package>`.
4. `bash scripts/check-workspace-policy.sh`.
5. `scripts/run-wasm-gates.sh --without-v8-spill` (the G5 wasm leg, for the new corpus case).

Touched packages: builtins-compiler, true-peak-limiter, wasm-gate-corpus, wasm-gates (its G5 test
reads the new case), host-core, graph, rack, console-workload, graph-compiler.

## Evidence

Line numbers are the base's (`ce714017c` for items 1-4, the merged tree `be08944b3` for item 5).

### Item 1: the scalar-oracle pairing tests (answer 1)

`crates/builtins-compiler/src/lib.rs` (1,100 lines removed, 12 added), commit "TA3 item 1":

- Deleted tests: `actual_scalar_nonadjacent_intervening_observer_error_completes_and_retries`,
  `actual_scalar_nonadjacent_failed_render_materializes_post_fader_before_error`,
  `actual_scalar_nonadjacent_ramp_retarget_and_failed_retry_stay_at_original_boundaries` (12466-13178).
- Deleted helpers they alone used: `render_failed_buffer_result`, `first_fader_state`,
  `last_fader_state`, `first_matrix_state`, `last_matrix_state`, `assert_owner_states_match`
  (12224-12338).
- Deleted instrumentation (5252-5528): `TestOnlyInitialMatrixTrace`, `EMPTY_INITIAL_MATRIX_TRACE`,
  `SCALAR_INITIAL_MATRIX_TRACE`, `record_initial_matrix_state`, `initial_matrix_state`, and with them
  the scalar state trace only those tests read: `TestOnlyScalarStateTrace`,
  `SCALAR_STATE_TRACE_CAPACITY`, `EMPTY_SCALAR_STATE_TRACE`, `SCALAR_STATE_TRACE`,
  `test_only_scalar_state_trace`, `SCALAR_FADER_STATE_WITNESS`, `SCALAR_MATRIX_STATE_WITNESS`, the
  witness's `scalar_fader_words`/`scalar_matrix_words` fields (unread since TA1), the writers
  `record_scalar_fader_state`, `record_scalar_matrix_state`, `record_scalar_state`, `scalar_owner_id`.
- Deleted plumbing in the oracle: the `test_only_owner` fields of `LiveControlFaderProcessor` and
  `LiveControlMatrixProcessor` and their initializers (7036-7097, 9418-9513), and the fifteen
  `record_scalar_*` calls of `ScalarPairProcessor` and `ScalarSplitPairProcessor` (9632-9910). Each
  `.inspect_err(|_| record…)?` became `?`. The render arithmetic and the remaining witnesses
  (`FADER_MATRIX_LIVE_WITNESS`, `FADER_MATRIX_OUTPUT_WITNESS`, owner drops, outer lifetime) are
  unchanged.
- `BoundaryVariant::Nonadjacent` and `NonadjacentOutputConflict` (constructed only by the deleted
  tests) and the output-track branch for the latter (11198-11295).
- `crates/graph/tests/MUTATIONS.md`: a TA3 note after the TA1 note (rows 916-16, 916-17).

### Item 2: the m2 digests (answer 2)

No change. The PR that fixes #1019 deletes `EXP2_DIGEST`/`LOG2_DIGEST` and the LANE-3 row of
`scripts/lib/aarch64-known-defects.py`.

### Item 3: the linked-limiter session pin (answer 3)

**Did G5 cover it? No.** G5's owner is `tools/wasm-gates/tests/g5_native_corpus.rs`
(`g5_native_digests_match_pins`) and the wasm leg of `scripts/run-wasm-gates.sh`, over
`tools/wasm-gate-corpus`. The limiter's delegated cases (`crates/true-peak-limiter/src/corpus.rs`,
`D90_DIGESTS`) were five, each one `limiter_block` call over 1,024 frames with fixed parameters:
`dual_mono/noise`, `maximum/noise`, `dual_mono/near_nyquist_w_min`,
`dual_mono/impulse_train_long_lookahead`, `dual_mono/subnormal_release_flush`. None retargets a
parameter, unlinks a pair or re-equals one; `limiter_block` passes `linked = false`, and the #990
decision and its `gain_linked` record live in `LimiterCore::process_block`, which no case reached.
So a relink on designed agreement (996-K2) could not move any G5 digest.

**Added, in the corpus's own way:** case 5, `maximum/link_unlink_relink`
(`crates/true-peak-limiter/src/corpus.rs`, appended; `CASE_COUNT` 5 -> 6; `run_linked`,
`LINKED_RETARGETS`, `fill_linked`). It builds the prepared `LimiterCore` at each width (eight
uniform lanes at a 5 ms lookahead, `LinkMode::Maximum`), renders eight 128-frame blocks of noise at
+6 to +18 dBFS, and retargets: both channels of every lane at block 3 (a linked ramp), lane 0 left
and lane 7 right at block 4 (unlinks), the other channels equal again at block 5. Pin, generated as
the corpus documents (the scalar-oracle digest `g5_native_digests_match_pins` prints): `31b09672…`.
It was identical at `Scalar`, `Simd4` and `Simd8` natively and in the wasm `simd128` leg
(`run-wasm-gates.sh`: 153 cases, no mismatch). AArch64 runs it in CI's `aarch64-release` G5 step;
it was not run on this box. Appending it moves the compressor, builtin-tail and automation blocks up
by one global index; their pins are indexed in their own crates, and the layout test passed.
`tools/wasm-gate-corpus/src/lib.rs` (`LIMITER_CASE_COUNT` doc) says so.

Mutants (true-peak-limiter `src/lib.rs`, one at a time, restored with `git checkout`), on
`g5_native_digests_match_pins`:

| mutant | Scalar | Simd4 | Simd8 |
|---|---|---|---|
| control: the decision always `false` (never links, the pre-#990 behaviour) | green | green | green |
| 996-K2 / 997-K2: the decision not written back to `gain_linked` | RED | RED | RED |
| K1: the decision drops `designed_gain_agree` | RED | RED | RED |
| #990 M1, M2, M6 (mirrored stores into the right rings) | RED | RED | RED |
| #990 M9: right ramps not copied at block end | green | green | green |
| E0: `designed_gain_agree` skips lane 0 | RED | RED | green |

The control shows the pin is the words of a kernel that never links. M9 and E0 at `Simd8` stay
owned by the session test's "equal" and "parts nothing" checks.

**Deleted:** `crates/host-core/src/limiter_linked_session.rs` `PIN`, the SHA-256 over master words
and readings (`Sha256Sink`, `digest`), the digest print and the `MISO_ENGINE_REPIN_LIMITER_LINKED_SESSION`
switch (lines 156, 193-194, 532-695, 763-771). The three tests keep every session check and are
renamed `the_hot_console_links_unlinks_and_limits_at_{simd8,simd4,scalar}`; the module note says
where the words are owned now. `crates/true-peak-limiter/tests/MUTATIONS.md`: row 996-K2 points at
the G5 case, and a TA3 section records the table above.

### Item 4: the phase profile and its probes (answer 4)

**Probe decision: delete.** No product code uses them: every probe was `cfg(feature =
"test-support")` (rack) or `cfg(any(test, feature = "test-support"))` (graph), and their only
consumers were `phase_profile`, the `bus_send_profile` example (an ignored-style measurement
instrument that gates nothing), and the probes' own tests.

- `tools/console-workload/tests/gain_pan_profile.rs`: deleted (504 lines; `phase_profile` and
  `probes_split_the_bank_units_and_move_no_bit`, the probes' own gate).
- `tools/console-workload/examples/bus_send_profile.rs`: deleted (397 lines).
- `crates/graph/src/lib.rs`: `test_only_phase_profile` (38-210) and the four probe sites in
  `GraphExecutor::render` (5879-6004). `crates/graph/src/runtime.rs`: `test_only_unit_phase`
  (4582-4610).
- `crates/rack/src/lib.rs`: `test_only_bank_phase_profile` (21-188), `BankStage::test_only_stage_name`
  (575-581), the probe sites in `BankChain::run` and the scatter paths (3215-3978), and
  `ProbedMembers` with `bank_phase_probes_split_every_scatter_path_and_move_no_bit` (7926-8081).
- `crates/rack/Cargo.toml`: the now empty `test-support` feature; `crates/graph/Cargo.toml`:
  `test-support = []`; `scripts/test-test-support-ci.py` line 161: `rack` leaves the expected set.

### Item 5: size_of re-derivations (answer 5)

| file | lines | change | relation kept, and its mutant |
|---|---|---|---|
| `crates/host-core/tests/successor_swap.rs` | 48-52, 214-218, 314-315, 967-970, 1680-1681, 1732-1756, 1805-1810, 3250-3254, 3293-3294, 4393-4395, 4696-4780, 4980-4982, 5175-5176, 5410-5411 | the fourteen `size_of::<GraphLaneMove/GraphStripMove/GraphEffectMove/(u32, u32)>` formulas (and the literal `8`s beside them) become exact table counts, `[sources, input lanes, strip stages, effect lanes]`, through a new test-support accessor `graph::test_only_carry_counts` (`crates/graph/src/lib.rs`, beside `carry_program_retained_bytes`) and a `carry_counts` field in `tests/support/successor.rs` and `PlainRun` | every exact move count. Mutant: drop `row.live != live_input` from the input-section join (`host-core/src/prepare.rs`) -> `a_changed_control_kind_carries_no_input_section` RED |
| `crates/host-core/tests/prepare.rs` | 407-475 | `effect_control_report_uses_actual_native_capacity_strings_and_owners` (a field-by-field `size_of` formula of the effect compiler's charge) becomes `effect_control_report_charges_the_producers_the_host_hands_out`: the report equals `effect_compiler::effect_control_resources` of the producers handed out; the owners share one factory; largest = max(table, owned). The charge's own relations stay in effect-compiler (TA1) | Mutant: host-core charges `&Vec::new()` -> RED |
| `crates/host-core/tests/submix_strip.rs` | 836-848 | `size_of::<Effect>() + params * size_of::<EffectParam>()` goes | "a bus insert is charged once": the model delta equals the same insert's on a track, the strings are retained once, and the delta exceeds the canonical part. Mutant: charge submix insert vectors twice (`session/src/estimate.rs`) -> RED |
| `crates/host-core/tests/route_mute.rs` | 2460, 2514-2517 | the `LatestCell<7>` + `DirtyWord` layout line goes | the audited-allocator ceiling (`total_bytes >= added`) stays; no product equality depended on the line |
| `crates/graph-compiler/src/lib.rs` | 385-391, 12414, 12824, 13181, 13549 | `literal_bank_slot_reservation_bytes` (a `size_of` of `rack::BankSlot` and `Box<dyn BankStage>`) becomes `GraphBankSlotResourceEstimate::checked_for(bank_count, width).total_bytes`, as TA1 did in `runtime_bank_slot_reservation_is_published_and_capped_transactionally` | the reports fold the reservation exactly once into four estimate rows. Mutant: skip the `checked_add_bank_slot_owners` fold on `estimate` (`graph-compiler/src/compile.rs`) -> six tests RED, the four launch-fixture tests among them |
| `crates/capi/tests/resource_lifecycle.rs` | 670, 693, 863 | **not changed** | each line feeds an exact-cap peak the product enforces (at the peak admits, one byte below refuses): the strips' slice (`size_of_val` of the real slice), the affine token's `PreparedStructuralCommand` layout, and the `u16` decode field. No ABI report exposes the prepared-protocol term, so a relation without `size_of` needs a product change; out of scope here |

## Test value

No test is new. The rewritten assertions and the defect each turns red on:

- `limiter` G5 case 5: a pair that relinks on designed agreement without reset, restore or
  `desymmetrize` (996-K2), at every width and on every target G5 runs.
- `successor_swap` counts: a carry clause that moves the wrong number of input, strip or effect
  entries (as the byte formulas did, without the layout).
- `effect_control_report_charges_the_producers_the_host_hands_out`: a host that admits a charge of
  anything but the producers it hands out.
- `a_bus_insert_is_charged_once_in_the_session_estimate`: a bus insert charged differently from
  the same track insert.
- the launch-fixture estimate tests: a slot reservation folded twice, or not at all, into the
  estimate rows.

## Docs updated

`crates/graph/tests/MUTATIONS.md` and `crates/true-peak-limiter/tests/MUTATIONS.md` (TA3 sections),
the module notes of `limiter_linked_session.rs` and `corpus.rs`, and the G5 limiter block doc in
`tools/wasm-gate-corpus/src/lib.rs`. `docs/handoffs/`, `docs/audits/` and `artifacts/` are dated
records and were not changed.
