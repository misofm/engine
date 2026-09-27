# Port graph-compiler's tests to the with-builtins compile

Split from #959 (option (b), owner ruling 2026-09-27). Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` section 3.2 and `VERIFY.md`.

## Smallest closable slice

While `GraphCompiler::compile` still exists, port every graph-compiler test that uses it to `compile_with_builtins`: the 38 functions in `crates/graph-compiler/src/lib.rs` (the B, P and G groups), `tests/track_delay.rs` and `tests/scale.rs`. Rules for a faithful port:
- **Never weaken a test.** Re-pin a SHA, tail or chain count only with a one-line reason; a test whose subject was the builtins-less path itself is deleted and listed with its reason.
- `tests/scale.rs` keeps 65,537 tracks and moves to `Backend::current()` (needs the linear-compile fix).
- The Output-tail test gives its tracks input sections with no filters, so the builtins tail stays `FiniteZero` (`crates/builtins/src/lib.rs:3360-3371`) and is not re-pinned to `Infinite`.
- Hand-built test plans in `crates/graph` (all its tests, rt1, rt9, rt10), `crates/source` (`lib.rs:3334`, `native_source.rs:3261`, `:3657`) and `crates/builtins-compiler` (`lib.rs:5138`, `:6384`) are **out of scope**: they construct graphs directly rather than compiling, and `graph` cannot dev-depend on graph-compiler without a crate cycle.

## Objective gates

- After the port, no test in `crates/graph-compiler` calls the builtins-less entry.
- The ported suite passes in dev and release; the list of re-pins and deletions is in the evidence.
- Every console workload digest unchanged; fmt, clippy `-D warnings`, doc `-D warnings`, graph policy and determinism scripts.

## Dependencies

The linear-compile fix; #957.


## Attempt 1 evidence

Implementer: attempt 1, branch `codex/964-port-graph-compiler-tests` (on #962's linear compile
merged with the optimisation batch, `53807191`). Commits: `bb3d0e69` (lib tests), `fdb411fa`
(`track_delay`, `scale`), and the evidence commit that carries this section and the mutation log.

### Result

- No test in `crates/graph-compiler` calls the builtins-less entry. `rg -nP
  'GraphCompiler::compile(?!_with_builtins)|GraphCompileRequest|PreparedGraphArtifact\b'` over the
  crate's test module (`src/lib.rs` from `mod tests`) and `tests/` finds nothing, so #959's gate-1
  pattern has nothing left to find in these tests. The only remaining hit in the crate outside the
  compiler itself is the `graph_fixture` bin, a tool (#963).
- 47 test functions ported: the 38 lib tests (B, P and G), the 8 in `tests/track_delay.rs` and the
  one in `tests/scale.rs`. **0 tests deleted.** 4 re-pinned. 4 builtins-less arms or assertion
  blocks deleted inside the B and P tests, each listed below. 1 test renamed.
- One helper, `compile_with_session_builtins`, does every ported lib compile: it prepares the
  session's own builtins (no meters, no live controls, unbounded builtin caps) and calls
  `compile_with_builtins`. `bind_session_builtins` binds `external_binding_nodes` -- the same
  `[Input.., Output]` set a builtins-less plan required since #925 -- and `into_bound`s.

### How the fixtures were strengthened

The canonical fixture (`canonical.json`) and `parametric-eq-nine-track.json` carry builtins that
are not identities: a 20 Hz high-pass and 20 kHz low-pass on every lane (tail `Infinite`), and
`pan: {left: 1, right: 1}`. Pan values are constant-power *positions*, so that is both lanes panned
hard right: the right output is `L + R`, the left output `6.1e-17 * (L + R)`, and an impulse of
`1.0 / -1.0` renders as silence. With those builtins attached, three kinds of check would have gone
weak or wrong:

- an exact pin (a SHA, a `Finite(0)` tail, an impulse rendered as `1.0`, a soft clip's finite
  support, a delay's bits) moves to whatever the filters and the pan law produce;
- a non-vacuity check that the second block differs from the first would pass on the filters'
  state alone;
- a bank-against-per-node comparison could no longer see a crossed lane, because both lanes sum into
  one output (mutation 964-9's control shows this).

So a test whose subject is the graph layer or an effect compiles `identity_builtins` (new): no
polarity inversion, 0 dB trim, both filters off (cutoff `0`, #808), 0 dB unmuted fader, and the
explicit 2x2 identity matrix. Each stage multiplies by exactly `1.0` or adds exactly `0.0`. Applied
in `compile_fixture`, `compile_reverse_route_submix_fixture`, `accepted_compressor_graph_fixture`
(the root of every `accepted_*` fixture), `twelve_track_bank_fixture`, a new
`identity_cross_index_effect_fixture` (the shared `cross_index_effect_fixture` is unchanged for the
out-of-scope `borrowed_session_…` test), `launch_parametric_eq_…`'s model and
`route_transform_bits_…`'s changed model. The builtin banks still bind and run in every one of
them (3 to 8 banks per plan at eight lanes).

Measured against the builtins-less compile, which still exists on this branch (a scratch probe,
not committed): for the canonical direct route and the accepted compressor, soft-clip, delay,
limiter and nine-track EQ fixtures, the identity-builtins plan has the **same canonical SHA, the same
output tail, and renders the same PCM bit for bit over 24 blocks**.

Tests whose checks are relative and cannot be satisfied by the builtins alone keep the fixture's
production builtins: the console fixtures (per-track filters, trims, faders, asymmetric pans),
`add_a_track_…` (it observes at `PostSimd1`, before the matrix), the queued-EQ fixture (filters
already off), `rack_chain_fixture` (no render), and the `track_delay` and `scale` sessions.

### Port table

"Id." = identity builtins; "Fixture" = the fixture's own builtins. Unless a row says otherwise,
every assertion is unchanged and passes at its original value.

| # | test | grp | builtins | what it still proves; why it is at least as strong |
|---|---|---|---|---|
| 1 | `issue122_reverse_route_ids_emit_sorted_levels_and_bind` | G | Id. | sorted levels, the level-major schedule, SHA `14d73…`, corrupted schedules and bytes refused, repeatable compiles, and a render of `2.0 / -2.0` with stable observer order. Same SHA and PCM pins; the `PostMatrix` observers now watch a builtin bank member's op, which was an alias before (964-2) |
| 2 | `direct_graph_report_exposes_zero_output_latency_and_tail_without_identity_change` | G | Id. | zero output latency and a `Finite(0)` output tail with filter-free input sections, and identical evidence across compiles; not re-pinned to `Infinite` (964-1) |
| 3 | `scalar_dispatch_compiles_without_banks_on_any_host` | G | Id. | every semantic output is dispatch-independent; **added**: the scalar dispatch forms no builtin bank and the host dispatch forms `3 x ceil(12 / lanes)` (964-4) |
| 4 | `effect_control_resource_uses_independent_queue_and_owner_arithmetic` | G | Fixture | the owner deltas are the independently derived queue, target and lane bytes; now asserted on both the report and the whole-plan estimate, and the exact / one-below caps use the whole-plan estimate, which is the one the compile caps (964-6) |
| 5 | `runtime_bank_slot_reservation_is_published_and_capped_transactionally` | P | Fixture | the published estimate is the prior owners plus one literal slot reservation over effect and builtin banks; priors re-derived (see Deleted), every equality kept (964-5) |
| 6 | `compiled_plans_always_lower_to_a_smaller_executable_program` | G | Id. / Fixture | every plan lowers, ops level-major, arena below the executor model, ops fewer than schedule items, bank storage never rewritten; the `PostInputBuiltins` bank-member clause now fires (the stage was an alias on builtins-less plans) |
| 7 | `multi_slot_rack_chains_form_one_cohort_and_bind_every_slot` | G | Fixture | one cohort, a bank per slot, session order |
| 8 | `bank_membership_is_independent_of_entry_order` | G | Id. (cross-index) / Fixture | membership, SHA, metadata, tails, PCM and control ownership survive entry reordering; crossed processors and controls change PCM. Identity builtins keep the output tail finite and the lanes separate |
| 9 | `chains_of_different_depths_share_a_cohort_through_identity_slots` | G | Fixture | one cohort through subsequence masks; slot 0 binds, slot 1 stays scalar |
| 10 | `mixed_twelve_track_plan_binds_renders_full_banks_and_scalar_tails_without_graph_changes` | G | Id. | full banks plus scalar tail without graph change; bank against scalar within tolerance; post-bank observer order |
| 11 | `add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain` | G | Fixture | G3: a ninth track moves no bit of the other eight (pre-matrix observation, filters banked too); G5 law unchanged; chain shape re-pinned exactly (964-10) |
| 12 | `launch_parametric_eq_fixture_retains_banks_and_matches_scalar_across_blocks` | G | Id. | banks and scalar tails, bank equals scalar bit for bit, EQ state crosses blocks (only the EQ can make them differ), bypass pins `1.40625 / -0.703125` |
| 13 | `prepared_eq_target_queue_collapses_then_matches_always_dual_and_scalar` | G | Fixture | queued targets collapse and match the always-dual and scalar arms |
| 14 | `prepared_eq_target_queue_fifo_left_both_left_matches_final_dual_batch` | G | Fixture | FIFO target order matches the final dual batch |
| 15 | `launch_compressor_fixture_retains_bank_tail_and_connected_scalar_with_zero_pdc` | G | Id. | banks, scalar tails, bank equals scalar, first block live, bypass keeps the schedule |
| 16 | `mixed_causal_compressor_and_fixed_latency_limiter_keep_parallel_pdc_aligned` | G | Id. | 486-sample PDC on every parallel route, preserved by bypass |
| 17 | `mixed_causal_multiband_and_fixed_latency_limiter_keep_parallel_pdc_aligned` | G | Id. | as 16 |
| 18 | `mixed_causal_gate_and_fixed_latency_limiter_keep_parallel_pdc_aligned` | G | Id. | as 16 |
| 19 | `dynamic_rack_compressors_bank_and_render_bit_identically_to_the_per_node_path` | G | Id. | the dynamic rack banks; bank equals per node bit for bit; detector state crosses blocks. Identity matrix keeps crossed lanes visible (964-9 and its control) |
| 20 | `rack_placement_changes_the_bank_but_never_the_samples` | G | Id. | placement moves the bank, not the samples, PDC or bank state |
| 21 | `a_dynamic_slot_that_differs_from_its_bank_mates_falls_back_per_node` | G | Id. | the two odd tracks fall back per node |
| 22 | `a_sidechain_lifted_chain_slot_falls_back_instead_of_failing_the_compile` | G | Id. | the compile succeeds and the lifted chain renders per node |
| 23 | `the_merged_span_hold_costs_the_input_slots` (was `…_with_and_without_builtins`) | P | Fixture | the merged-span hold costs 256 banked against 193 per node on the plan every host renders; builtins-less arm deleted |
| 24 | `console_sixty_four_track_fixture_banks_its_dynamic_compressor_bit_identically` | G | Fixture | both racks bank fully; banked equals per node bit for bit through the production strip; G5 per chain; chain counts re-pinned exactly for both arms (964-10) |
| 25 | `intended_placement_merges_two_chains_into_one_bit_identically` | G | Fixture | the two placements render the same bits; one chain per cohort on both; slot count re-pinned |
| 26 | `launch_gate_expander_fixture_retains_width_correct_banks_and_scalar_fallbacks` | G | Id. | width-correct banks and scalar fallbacks, bank equals scalar |
| 27 | `launch_true_peak_limiter_fixture_retains_banks_tails_latency_and_transactional_caps` | G | Id. | banks, tails, 486-sample latency, transactional caps |
| 28 | `launch_multiband_compressor_fixture_closes_bank_graph_and_transactional_caps` | G | Id. | banks, release probe, transactional caps |
| 29 | `launch_soft_clip_fixture_closes_banks_tails_pdc_support_and_transactional_caps` | G | Id. | banks, tails, PDC and the finite 61-sample support (needs filter-free inputs) |
| 30 | `launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps` | G | Id. | banks, tails, PDC, caps |
| 31 | `launch_delay_fixture_closes_scalar_state_tail_pdc_and_transactional_caps` | G | Id. | the delay's exact bits across blocks, tail, PDC, caps |
| 32 | `builtins_replace_only_the_three_internal_track_bindings` | P | Fixture | the three builtin stages are compiler-owned bindings with their own ops, exact op list, `Infinite` input-builtins tail; builtins-less arm deleted |
| 33 | `post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs` | P | Fixture | a cap only the attached builtin banks exceed rejects and hands both inputs back; pre-attachment figure from the report (see Deleted) |
| 34 | `level_major_compiler_coloring_matches_independent_live_intervals` | G | Id. | the colouring matches an independent live-interval oracle |
| 35 | `accepted_session_compiles_binds_and_renders_direct_route` | B | Id. | estimates, identity-boundary colouring into 2 buffers, bind, and a render of `1.0 / -1.0`; the #925 shape assertions replaced by the exact with-builtins bind sets (964-7) |
| 36 | `canonical_artifacts_are_complete_and_repeatable_100_times` | G | Id. | complete sections, streaming hash equals materialised, 100 identical compiles |
| 37 | `route_transform_bits_participate_in_semantic_hash` | G | Id. (both models) | a route gain changes the hash; now the gain is the only difference between the two plans (964-8 and its control) |
| 38 | `route_transform_uses_the_canonical_db_to_gain_conversion` | G | Fixture | the route gain is `math::db_to_gain_f32`, `0x3de5_ca16` at -19 dB |
| 39 | `track_delay::a_zero_delay_session_lowers_no_delay_node` | G | Fixture, Scalar | no delay entry and no bytes for a zero-delay session |
| 40 | `track_delay::the_zero_delay_plan_digest_is_the_current_semantic_plan` | G | Fixture, Scalar | the zero-delay plan's canonical digest; re-pinned (964-3) |
| 41 | `track_delay::a_delayed_session_is_a_different_plan` | G | Fixture, Scalar | a delayed session hashes differently from the pinned digest |
| 42 | `track_delay::a_track_delay_moves_no_pdc_row` | G | Fixture, Scalar | inserted delays, route timings, output latency and node latencies unmoved by a delay |
| 43 | `track_delay::the_estimate_charges_each_lane_its_own_ring` | G | Fixture, Scalar | `delay_bytes` grows by 4 per sample per lane; read from the whole-plan estimate, equal at Scalar to the report's and to the builtins-less one (measured) |
| 44 | `track_delay::the_pdc_counts_are_untouched` | G | Fixture, Scalar | PDC counts unmoved while the bytes move |
| 45 | `track_delay::a_ring_is_a_named_allocation` | G | Fixture, Scalar | the largest allocation covers the largest ring |
| 46 | `track_delay::an_oversized_delay_is_rejected_by_the_caps` | G | Fixture, Scalar | aggregate rings over the plan cap are refused with `graph.resource.limit` |
| 47 | `scale::compiles_65_537_tracks_or_rejects_only_a_configured_resource` | G | Fixture, `current()` | 65,537 tracks compile with the same node, edge, route, effect and schedule counts, and only a configured cap refuses; **added**: the refusal hands every track's builtins back (964-11) |

### Re-pins

| test | before | after | reason |
|---|---|---|---|
| `track_delay::the_zero_delay_plan_digest_is_the_current_semantic_plan` | `eb3ca776…cb18e0ea10` | `957e97ca86f8af87ff8c0ea6adc35ec26b903046c73541613f6abb8ce7640e42` | the fixture's input filters make the nine `post-input-builtins` tails `infinite`; the new canonical text is the old one with exactly those 18 tokens changed (9 `node`, 9 `tail` rows), derived independently by editing the old text and hashing it, and mutation 964-3 reverts to exactly the old digest |
| `add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain` | `chains == slots` | `(effect, builtin banks) == (full cohorts, 3 x cohorts)`, `chains == 2 x cohorts`, `chains < slots` (8 tracks: 4 slots, 2 chains; 9 tracks: 7 slots, 4 chains) | each cohort also binds post-input, fader and matrix; post-input fuses into the EQ's chain and the `PostSimd1` observers decline EQ -> fader. Mutation 964-10 shows the old pin could not see a runtime that never merges |
| `console_sixty_four_track_fixture_banks_its_dynamic_compressor_bit_identically` | chains `= slots - cohorts`; per-node arm transposes `0` | chains `= slots - 4 x cohorts` (= cohorts, 40 slots -> 8 chains); per-node arm `(slots, chains) == (3 x cohorts, 2 x cohorts)` and transposes `= BLOCKS x chains`; builtin banks `= 3 x cohorts` in both arms | the whole five-slot strip fuses into one chain per cohort; the per-node arm refuses effect banks only, so it still binds its builtin banks, split by the per-node EQ and compressor |
| `intended_placement_merges_two_chains_into_one_bit_identically` | slots `2 x cohorts` (both layouts) | `(2 + 3) x cohorts` | the three builtin slots per cohort join the same chain; the chain counts (one per cohort) are unchanged |

No tail was re-pinned: the Output-tail test compiles filter-free input sections and stays
`Finite(0)`, as the brief directs.

### Deleted (builtins-less subjects only; no whole test deleted)

| test | deleted | reason |
|---|---|---|
| `accepted_session_compiles_binds_and_renders_direct_route` (B) | `required_bindings.len() == 2` and "the three builtin stages are not bindable" | #925's builtins-less shape. Replaced by the with-builtins statement of the same host contract: `required_bindings` is exactly `[Input, PostInputBuiltins, PostFader, PostMatrix, Output]` and `external_binding_nodes` exactly `[Input, Output]` (mutation 964-7) |
| `builtins_replace_only_the_three_internal_track_bindings` (P) | the builtins-less arm: `compile_fixture`'s ops are `[Input, Route, Output]` | the #925 alias elision of a builtins-less plan, which #958 reverts; the with-builtins arm still pins the exact op list |
| `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins` (P), renamed `the_merged_span_hold_costs_the_input_slots` | the builtins-less arm: 192 banked, 129 per node, both `<= 193` | measured only a builtins-less plan; the with-builtins arm (256 / 193) is kept. `crates/graph/src/program.rs`'s doc, which named the test and quoted both arms, is updated |
| `post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs` (P) | the builtins-less `base` compile | it only supplied the pre-attachment audio samples; the with-builtins report estimate publishes the same figure (18,696 on both, measured) |

`runtime_bank_slot_reservation_is_published_and_capped_transactionally` (P) also compiled
builtins-less priors. They are re-derived rather than deleted: the prior owners are the same
session compiled with its builtins at `Backend::Scalar`, where no bank of either kind forms, and for
the mixed fixture the effect banks are charged by their own `effect_bank_resource` term.
Measured: the builtins-less host estimate equals the scalar with-builtins estimate, and the
builtins-less effect-only estimate equals the scalar estimate plus the effect-bank fold and the
effect-only slot reservation. The helper `compile_console_model` (effect-only) is deleted; its one
caller uses `compile_console_model_with_builtins`.

### Mutations

Eleven rows in `crates/graph-compiler/tests/MUTATIONS.md` (`## Issue #964`), all red: 964-1 to
964-11 cover the builtin tail mapping (Output tail, SHA `14d73…`, the re-pinned delay digest), the
builtin banks' dispatch, the combined slot reservation, the control-owner estimate, the host's bind
set, the route-transform hash, a crossed lane against per node, chain merging, and a compiled track
ceiling at 65,537 tracks. Two controls stay green against the naive port, which is the evidence for
the fixture strengthening: 964-8 (the changed model with its own filters hashes differently for the
wrong reason) and 964-9 (the fixture's hard-right pan hides a crossed lane).

### Scale

`compiles_65_537_tracks_or_rejects_only_a_configured_resource` keeps 65,537 tracks and moves to
`Backend::current()` with the session's builtins (both compiles). It also asserts that the refused
compile hands all 65,537 tracks' builtins back. Debug, alone: 55.0 s, 1.34 GB peak RSS (was 29.9 s,
0.89 GB at Scalar without builtins). The whole `scale` binary in debug: 60.3 s, 2.47 GB peak (was
62.1 s, 1.87 GB): the two tests run in parallel, so the job's wall time is unchanged.
`track_delay` stays at `Backend::Scalar`, as it was.

### Gates

All with `CARGO_INCREMENTAL=0`, the worktree's own `target/`, output piped under `set -o pipefail`.

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph-compiler` (dev) | pass: lib 80, `graph_fixture` 1, `route_gain` 3, `scale` 2 (60.3 s), `track_delay` 8, doc 6 |
| `cargo test --locked --release -p graph-compiler --config 'profile.release.panic="unwind"'` | pass, same counts (`scale` 17.2 s). The override is #962's: without it the `graph_fixture` bin's `panic = "abort"` collides with the harness's cdylib |
| `cargo test --locked -p graph-compiler --features graph/test-support,builtins-compiler/test-support` (CI's debug feature set) | pass, same counts |
| `cargo test --locked -p graph` | pass: lib 109, rt10 1, rt1 1, rt9 1 |
| `cargo test --locked -p builtins-compiler --features test-support` | pass: 58, 9, 3, 6, 1, 2 |
| `cargo test --locked -p host-core --all-features` | pass (18 binaries) |
| `cargo test --locked -p console-workload`, and `--release` (CI's mode) | pass: 8, 4, 23, 3 both ways. No product code changed, so no console digest or unit census can move; the digest-pinning tests pass unchanged |
| `bash scripts/check-graph-policy.sh .`; `bash scripts/test-graph-policy.sh` | PASS; ok |
| `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| mutations 964-1 to 964-11 | all RED as recorded; both controls GREEN as expected |

The timed benchmark was not run.

### Deviations and notes for the verifier

- **Outside the crate:** one doc comment in `crates/graph/src/program.rs` (it named the renamed
  test and quoted its deleted arm). No product code changed: the branch diff is test code, the
  mutation log, that doc comment and this spec, so no console digest can move.
- **Scope:** `graph_fixture` (`src/bin`) still compiles builtins-less; it is a tool (#963) and moves
  `fixtures/graph/v1` bytes.
- **`Backend::Scalar`:** `track_delay`, `effect_control_resource_…`'s scalar case and the scalar arms
  of the P tests compile bankless with-builtins plans, as #959 flags. Out of scope here.
- **Finding:** the canonical fixture's `pan: {left: 1.0, right: 1.0}` pans both lanes hard right. It
  reads like unit gains; with builtins it renders an impulse of `1 / -1` as silence. Every test
  that relied on it now says what it wants explicitly (`identity_builtins`). Whether the fixture
  itself should change is for the owner (it would move `fixtures/graph/v1` and host bytes).
