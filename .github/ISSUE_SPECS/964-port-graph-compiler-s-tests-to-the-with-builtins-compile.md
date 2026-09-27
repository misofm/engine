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

## Sol attempt 1 verdict: FAIL

Reviewed `git diff 53807191..HEAD` (`bb3d0e69`, `fdb411fa`, `741b5a5b`) side by side, test by test.
Host x86-64-v3 (eight-lane banks), `CARGO_INCREMENTAL=0`, the worktree's own `target/`. Every
mutation and probe below was applied, run and reverted; nothing of it is committed.

One blocking finding: five ported tests got weaker. The rest of the port is faithful.

### Findings, by severity

1. **Medium, blocking (the brief's "never weaken a test").** The five launch-effect
   transactional-cap tests read their one-byte-below plan cap from the report's estimate, which is
   taken before the builtin banks attach. The compile caps the whole-plan figure
   (`capped_estimate`, `compile.rs:695-786`), which `graph_resource_estimate()` publishes.
   - Affected: `crates/graph-compiler/src/lib.rs:11691` (limiter), `:12059` (multiband), `:12416`
     (soft clip), `:12776` (transient shaper) and `:13057` (delay). Each is
     `let minimum_plan_bytes = artifact.report().estimate.incremental_plan_bytes;`.
   - Builtins-less, the report's estimate was the capped figure, so the cap sat exactly one byte
     below it. Now it sits 64,043 bytes below: the builtin payload, measured on all five fixtures.
   - Evidence: mutation M-b drops the bank-slot reservation from `capped_estimate` only
     (`compile.rs:749-751`, `slot_resource` -> `Default::default()`). The base versions of the
     limiter, multiband, soft-clip and transient-shaper tests go RED ("one-byte-below ... graph cap
     must reject before publication"). The ported versions stay GREEN.
   - The fix is to read `artifact.graph_resource_estimate().incremental_plan_bytes` on those five
     lines. That was verified: all five pass unmutated and all five go RED under M-b, the delay
     test included.
   - This is the correction 964-6 already made in `effect_control_resource_…`. Port-table rows
     27-31 claim these assertions are unchanged. The suite still catches M-b, through tests 4 and 5,
     so there is no net hole, but each of these tests is weaker than it was.
2. **Low.** `Backend::Scalar` is not essential to `tests/track_delay.rs`.
   - It has no lane oracle, and its digest is dispatch-independent: at `Backend::current()` it is
     the same `957e97ca…`, with the same zero-delay `delay_bytes` and largest allocation.
   - All 8 tests pass unchanged with `dispatch: Backend::current()` at `track_delay.rs:174`
     (verified).
   - Under the owner's real-paths rule it should move. That is one line plus the comment at
     `:189-191`. The brief did not require it, so either take it in attempt 2 or hand it to #959
     explicitly.
3. **Info.** The `graph_fixture` bin's unit test (`src/bin/graph_fixture.rs:366`, through `:88`)
   still reaches the builtins-less entry. #963's brief owns it, so the gate holds for the 47 scoped
   tests but not literally for "no test in `crates/graph-compiler`". Say so in the evidence.
4. **Info.** `runtime_bank_slot_reservation_…` now takes its no-bank priors from with-builtins
   compiles at `Backend::Scalar` (`lib.rs:3269-3284`, `:3308-3322`).
   - They equal the old builtins-less priors by derivation, not by coincidence:
     - With no live controls, the builtin scalar-owner term and the effect-control term are both
       zero (`graph_scalar_owner_resource` counts only strips with `control.is_some()`).
     - The semantic estimate and the runtime metadata do not depend on builtins.
     - `Scalar` forms no bank and reserves no slot.
   - If #959 closes the Scalar back door, this test needs another no-bank prior.
   - The historical #925 rows in `crates/graph/tests/MUTATIONS.md:361-364` still name the deleted
     merged-span arm; #958 retires them.

### Checked and sound

- **Identity builtins are identity.**
  - An independent probe compiled 12 configurations both builtins-less and with
    `identity_builtins` at `Backend::current()`: direct route, nine-track EQ, compressor,
    dynamic-rack compressor, gate, limiter, multiband, soft clip, transient shaper, delay,
    identity cross-index and twelve-track.
  - Every one has the same SHA, output tail and output latency, and renders PCM bit-identical over
    24 blocks. The matrix kernel's identity select passes the lanes through untouched.
  - So these tests cannot hide a fault that the builtins-less versions exposed.
  - The queued-EQ fixture keeps its own filter-free hard-left/hard-right pan, and its lanes stay
    separate.
  - The console tests keep the production strip and still see a crossed lane: mutation MX (the
    per-node compressor crosses its lanes) turns both `dynamic_rack_compressors_…` and
    `console_sixty_four_track_…` RED.
- **The re-pins are derived.**
  - `track_delay` digest: the builtins-less and with-builtins canonical texts differ in exactly 18
    lines, the nine `node` rows and nine `tail` rows of `post-input-builtins`
    (`finite:0` -> `infinite`). Editing only those rows of the old text hashes to `957e97ca…`, and
    the old text hashes to `eb3ca776…`.
  - Chain and slot counts: the derivations hold. S7 turns `add_a_track` RED at 1 chain against 2,
    which confirms the stated reason (the `PostSimd1` observers decline the EQ -> fader merge).
- **The four deleted arms tested only the builtins-less path:**
  - #925's two-node bind shape;
  - #925's alias op list;
  - the 192/129 builtins-less arena;
  - the base compile as a pre-attachment figure, which equals the report's
    `audio_buffer_samples` by derivation (no effects, and a slot adds no samples).
- **The rename lost nothing.** Test counts are 80/8/2 before and after, and only one name changed.
  S2c turns the renamed test RED at 193 against 256.
- **No test in the scope calls the builtins-less compile.**
  `rg -P 'GraphCompiler::compile(?!_with_builtins)|GraphCompileRequest|PreparedGraphArtifact\b'`
  finds nothing in the `lib.rs` test module or in `tests/`.
- **The `program.rs` edit is doc-only and correct:** 4 comment lines, no code.
- **Scale**, reproduced: 55.00 s and 1.34 GB peak RSS alone in debug. The binary takes 60.2 s in
  debug and 17.1 s in release.

### Sol mutations (all RED as expected, all reverted)

| # | group | mutation | result |
|---|---|---|---|
| S1b | B | `compile.rs` lists `PostSimd2PreFader` as bindable with builtins | `accepted_session_…` RED at the exact `required_bindings` |
| S2 | P | the cap check reads the pre-attachment `audio_buffer_samples` | `post_bank_graph_cap_…` RED, "post-bank cap must reject" |
| S2b | P | `PostSimd2PreFader` is no longer an alias candidate | `builtins_replace_only_…` RED at the exact op list |
| S2c | P | the bank-window hold is disabled | `the_merged_span_hold_costs_the_input_slots` RED, 193 vs 256 |
| S3 | G | PDC compensation one sample short | all three `mixed_causal_…` RED |
| S4 | G/B | the identity-matrix arm swaps L and R | `accepted_session_…` (-1.0), `issue122_…`, EQ bypass pins and delay bits RED; bank-vs-per-node tests GREEN, as expected |
| MX | G | the per-node compressor crosses its lanes | `dynamic_rack_compressors_…` and `console_sixty_four_track_…` RED |
| S5 | track_delay | the left ring is charged 2 B/sample | `the_estimate_charges_each_lane_its_own_ring` RED, 2 vs 4 |
| S6 | scale | the builtin bank planner refuses more than 65,536 tracks | `compiles_65_537_…` RED; the old Scalar builtins-less gate never reached the planner |
| S7 | G | an observed alias no longer declines a merge | `add_a_track_…` RED, 1 vs 2 chains |
| M-b | G | the slot reservation is dropped from `capped_estimate` only | finding 1: ported launch cap tests GREEN, base versions RED |

### Gates (all pass)

- `cargo fmt --all --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `cargo test -p graph-compiler`:
  - dev: 80/1/3/2/8/6;
  - release with `panic="unwind"`: the same counts;
  - CI's `graph/test-support,builtins-compiler/test-support` feature set: the same counts.
- `-p graph` (109/1/1/1), `-p builtins-compiler --features test-support`,
  `-p host-core --all-features`, and `-p console-workload` in dev and release, so the console
  digests are unchanged.
- `check-graph-policy.sh` PASS, `test-graph-policy.sh` ok, `check-graph-determinism.sh` PASS
  (100/100).

### For attempt 2

- Fix finding 1 on the five lines and re-run M-b.
- Take finding 2, or hand it to #959 explicitly.
- Record finding 3 in the evidence.
- No other change is needed.

## Attempt 2 evidence

Implementer: attempt 2, on the attempt-1 tree plus Sol's verdict (`c4e178f6`). Commits:
`10172c71` (tests) and the evidence commit carrying this section and the `MUTATIONS.md` rows.

### Finding 1 (blocking): fixed

- The five launch-effect cap tests read `artifact.graph_resource_estimate().incremental_plan_bytes`:
  the limiter, multiband, soft-clip, transient-shaper and delay `launch_*` tests (formerly
  `lib.rs:11691`, `:12059`, `:12416`, `:12776`, `:13057`). Their one-byte-below cap again sits
  exactly one byte below the figure the compile caps.
- Sol's M-b (964-12 in `MUTATIONS.md`: the slot reservation is left out of `capped_estimate`
  only) turns all five RED. Attempt 1's versions stayed GREEN under it.
- The limiter, multiband, soft-clip and transient-shaper tests also pin bank-against-scalar byte
  deltas, which they state on the report's estimate. Builtins-less, that was the published and
  capped figure too. They now also call `assert_builtin_attachment_matches`: the builtin
  attachment (whole-plan minus report, per field) is the same in both arms and non-zero at a SIMD
  dispatch. With the report deltas pinned, that pins the whole-plan deltas to the same values.
  964-13 (the attachment charges the effect-bank metadata a second time) turns all four RED.

### Finding 2 (low): taken

`tests/track_delay.rs` compiles at `Backend::current()`; the comment that explained `Scalar` is
rewritten. All 8 tests pass unchanged, and the digest is still `957e97ca…` (dispatch-independent).
964-3 and Sol's S5 were re-run at `current()`: both RED.

### Finding 3 (info): recorded

The objective gate holds for the 47 scoped tests, not literally for every test target in the
crate: the `graph_fixture` bin's own unit test (`src/bin/graph_fixture.rs:366`, through its
`compile_fixture` at `:88`) still reaches the builtins-less entry. That bin is a tool, and #963's
brief owns its port (it moves `fixtures/graph/v1` bytes).

### Sweep: every pre-builtins estimate read in the ported tests

A cap is checked against the whole-plan estimate (`capped_estimate`, published as
`graph_resource_estimate()`, which is `graph().estimate`). The builtin attachment
(`checked_add_builtin_banks`) changes only `audio_buffer_samples`, `graph_metadata_bytes`,
`incremental_plan_bytes`, `session_plus_plan_bytes`, `largest_allocation_bytes` and the three
`builtin_bank_*` fields. Every read of `report().estimate` (or `.report.estimate`) in the 47 ported
tests was checked:

| test | read | verdict |
|---|---|---|
| limiter, multiband, soft clip, transient shaper, delay `launch_*` | one-byte-below plan cap | **was the finding; fixed** |
| limiter, multiband, soft clip, transient shaper `launch_*` | bank-against-scalar deltas of samples, graph, plan and session bytes | budget statements on the report; **strengthened** with the whole-plan check above |
| the same four, and `launch_delay_…` | `effect_bank_*` counts and bytes, `effects`, `declared_effect_bytes` | fields the attachment does not touch; report and whole-plan agree by construction (the new check asserts a zero delta for the `effect_bank_*` fields) |
| `effect_control_resource_uses_…` | owner deltas and exact / one-below caps | fixed in attempt 1 (964-6): deltas on both estimates, caps on the whole-plan one |
| `runtime_bank_slot_reservation_…` | fold arithmetic on a copied estimate (`below_largest`, `above_largest`, `zero`) | a template struct for `checked_add_bank_slot_owners`; its values are overwritten, no cap |
| `runtime_bank_slot_reservation_…` | "published estimate carries slots" (`artifact.report().estimate == expected`) | the report is the figure under test there, as before the port (that arm compiled with builtins before #964 too); the mixed arm pins the whole-plan estimate exactly, and the exact / one-below caps use it |
| `post_bank_graph_cap_…` | pre-attachment `audio_buffer_samples` | used only as the figure the cap must exceed; the cap is set from the whole-plan figure (Sol's S2 is RED) |
| `rack_placement_changes_the_bank_but_never_the_samples` | `effect_bank_*`, `declared_effect_bytes` equal across placements | not caps, and fields the attachment does not touch |
| `accepted_session_compiles_…` | `routes`, `effects`, `reductions`, `logical_nodes`, three `> 0` | counts and sanity, no cap |
| `scale::compiles_65_537_…` | `logical_nodes`, `edges`, `routes`, `effects`; the `maximum_nodes = 1` cap | counts the attachment does not touch; the cap is on `materialized_nodes`, which it does not touch either |
| `track_delay` (8 tests) | `graph_resource_estimate()` and `plan().estimate` | already the whole-plan figure (`plan()` is `graph()`), including the oversized-delay cap |

Not ported, so not swept: `live_scalar_owner_bytes_are_published_and_capped_before_binding`
already compiled with builtins before #964 (its caps read `graph_resource_estimate()`).

### Correction to the attempt-1 port table

Rows 27 to 31 said the launch cap assertions were unchanged. They were not: attempt 1 left the
cap reading the pre-attachment figure. As of attempt 2 they cap on the whole-plan figure, and rows
27, 28, 29 and 30 also carry the whole-plan delta check.

### Gates

Re-run on `10172c71` with `CARGO_INCREMENTAL=0`, the worktree's own `target/`, sequentially.

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph-compiler` (dev) | pass: 80/1/3/2/8/6 (`scale` 60.4 s) |
| the same, `--release --config 'profile.release.panic="unwind"'` | pass: 80/1/3/2/8/6 (`scale` 17.3 s) |
| the same, `--features graph/test-support,builtins-compiler/test-support` | pass: 80/1/3/2/8/6 |
| `cargo test --locked -p graph` | pass: 109/1/1/1 |
| `cargo test --locked -p builtins-compiler --features test-support` | pass: 58/9/3/6/1/2 |
| `cargo test --locked -p host-core --all-features` | pass (18 binaries) |
| `cargo test --locked -p console-workload`, dev and `--release` | pass: 8/4/23/3 both. Still no product code in the branch diff, so no console digest can move |
| `check-graph-policy.sh .`, `test-graph-policy.sh`, `check-graph-determinism.sh` | PASS, ok, PASS (100/100) |
| 964-12, 964-13, 964-3 and S5 rechecks | RED as recorded |

The timed benchmark was not run. `target/` is deleted.

## Sol attempt 2 verdict: PASS

Reviewed `git diff c4e178f6..HEAD` (`10172c71`, `aaded427`). Same host and rules as attempt 1:
x86-64-v3, eight lanes, `CARGO_INCREMENTAL=0`, the worktree's own `target/`. Every mutation was
applied, run and reverted; none is committed.

### The attempt-1 findings are resolved

- **Finding 1 is fixed.** All five launch cap tests (limiter, multiband, soft clip, transient
  shaper, delay) now read `graph_resource_estimate().incremental_plan_bytes`. My mutations, each
  run against the five tests plus `effect_control_resource_…` and `runtime_bank_slot_reservation_…`:

  | mutation | what it changes in `compile.rs` | the five launch tests | the other two |
  |---|---|---|---|
  | M-b | the slot reservation is dropped from `capped_estimate` | RED, all five | RED |
  | MB2 | the builtin payload is dropped from `capped_estimate` | RED, all five | RED |
  | OB1 | lenient off-by-one on the plan-bytes cap (`> cap + 1`) | RED, all five | RED |
  | OB2 | strict off-by-one (`>=`) | GREEN | RED |

  OB2 is not a weakening. It needs an exact-cap accept arm, and the builtins-less originals never
  had one; tests 4 and 5 carry it.
- **The new `assert_builtin_attachment_matches` is non-vacuous** (`lib.rs:1057-1109`). My own
  mutation turns all four callers RED with "the builtin attachment adds the same bytes to both
  arms": the attachment also charges `effect_bank_scratch_bytes / 8` to `session_plus_plan_bytes`
  in `PreparedGraphPlan::with_builtin_banks`.
- **Finding 2 is taken.** `track_delay` compiles at `Backend::current()`. All 8 tests pass, and
  the digest is unchanged at `957e97ca…`. 964-3 (the digest falls back to `eb3ca776…`) and S5 (2
  vs 4) are both RED at `current()`. `a_ring_is_a_named_allocation` stays non-vacuous: the zero-delay
  largest allocation is 48,915 bytes, well under the 192,000-byte ring.
- **Finding 3 is recorded** in the evidence.

### Sweep spot-check

I listed every `estimate` read in the test module independently. It matches the implementer's
table:

- The remaining `report().estimate` reads in the delay test, `rack_placement_…` and
  `accepted_session_…` are `effect_bank_*`, `effects`, `declared_effect_bytes`, counts or `> 0`
  checks.
- `checked_add_builtin_banks` (`graph/src/lib.rs:686-716`) never touches any of those fields.
- None of them sets a cap.

### Gates (all pass on `aaded427`)

- `cargo fmt --all --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `cargo test -p graph-compiler`:
  - dev: 80/1/3/2/8/6;
  - release with `panic="unwind"`: the same counts;
  - CI's `graph/test-support,builtins-compiler/test-support` feature set: the same counts.
- `-p graph` (109/1/1/1), `-p builtins-compiler --features test-support`,
  `-p host-core --all-features`, and `-p console-workload` in dev and release, so the console
  digests are unchanged. The branch still has no product code.
- `check-graph-policy.sh` PASS, `test-graph-policy.sh` ok, `check-graph-determinism.sh` PASS
  (100/100).

No new findings. The timed benchmark was not run.
