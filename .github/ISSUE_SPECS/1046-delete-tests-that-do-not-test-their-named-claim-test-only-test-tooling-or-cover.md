# Delete tests that do not test their named claim, test only test tooling, or cover removed surface

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 6). Base `a9414c0c`. Paths starting `../`
are relative to the audit's handoff folder. The rows in part C
need owner rulings R6 and R7. Parts A and B need none.

## Problem

The audit counts about 150 tests that do not test the claim in their name. This issue lists the
about 80 whose rows were checked one by one. The source-text scrapes are issue 07, and the dominated
slow tests are issue 09. Some can still fail when the product breaks
badly, as a side effect of running it, but where the audit measured, nothing they catch is uniquely
theirs. They fall into four kinds:
- a tautology, such as `QuantumFrames(128).0 == 128`, `render() == render()` of a pure function, or
  `(B+C)-B == C`;
- IEEE arithmetic only;
- std-library behaviour only;
- a test-local constant or helper.

A few assert nothing at all, and some print and return. Others test benchmark and fixture tooling
rather than the engine, or test surface the owner removed. They run in about 1 s in total, so the
cost is maintenance: re-pins, reading time, and false confidence in the counts.

Each row below was read and spot-checked by the audit's classifiers against the body, not the name,
and each names its surviving guard. The item numbers refer to
[`../data/candidates-dsp-graph.md`](../data/candidates-dsp-graph.md) (**D**) and
[`../data/candidates-host-tools.md`](../data/candidates-host-tools.md) (**H**), which hold the file:line,
the reason and the guard for every row.

## Outcome: delete these

**A. The named claim holds by construction, or nothing is asserted (no ruling).**
- **D2:** `crates/parametric-eq/src/lib.rs:4471` (seven `println!`, runs in CI, asserts nothing).
- **D3:** `crates/gate-expander/tests/identity.rs:223` (returns unless `Simd4`; asserts nothing even
  then).
- **D6:** the `print_pins`/`print_digests` regeneration printers in parametric-eq, effect-runtime and
  delay.
- **D8-D16, D17 except the `g6_ftz_inert.rs:123` stub, D18-D21:**
  - the evidence-only reduction helper tests in `graph/src/lib.rs:4532`, `:4545`;
  - the repeat of a compile-time `const` assert (`graph/src/runtime.rs:8238`);
  - constant restatements (`parametric-eq/src/lib.rs:4639`, `effect-runtime/tests/state_payload.rs:25`,
    `lane/tests/fp_env.rs:207`, `lane/tests/f64_lane.rs:191`);
  - the signature tautologies `a_resident_read_is_repeatable_to_the_bit` (compressor and limiter);
  - IEEE-only `compressor/tests/mono_collapse.rs:163`;
  - equivalence-blind `compressor/tests/ramps.rs:587`;
  - `rack/tests/console_bank.rs:405`;
  - the Annex-2 one-hot tautology in `dsp-reference`;
  - a test of test instrumentation, `builtins-compiler/src/lib.rs:9759`;
  - `transient-shaper/tests/allocation.rs:112,117`, **only after** that file's zero-allocation gate
    (`:202`) moves to `bench_support::alloc`. Until then they are its positive controls;
  - four copies of `shared_hex_adapter_matches_literal_bytes`.
- **H1-H17:**
  - std-only (`protocol/tests/delivery_ownership.rs:435`, `host-core/tests/builtin_batch_endpoint.rs:862`);
  - IEEE-only (`host-core/src/builtin_batch_endpoint.rs:2127`);
  - test-local sums already stale (`session/tests/invalid_matrix.rs:1166`, `:1206`);
  - `engine/src/lib.rs:135`, `:140`;
  - `host-web/src/tests.rs:6379`;
  - `bench-support/src/stats.rs:140`;
  - `tools/audit/src/source.rs:497`;
  - the determinism-of-a-pure-function pair (`parameter-metadata/tests/abi_layout.rs:1348`,
    `session-validator/tests/validate.rs:432`);
  - `tools/bench/src/floor.rs:608`;
  - `source/src/native_source.rs:3949`;
  - `effect-package/tests/package_v1_qualification.rs:371`;
  - the ignored print-only `session/tests/descriptive_scale.rs:54`;
  - `source/src/native_source.rs:4435`.
  - Not in this list: `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`, which moves to issue 12.

**B. Tools re-testing shared bench-support helpers through an import alias (no ruling).**
- **H37-H39:**
  - five `shared_sha256_alias_matches…` copies in `tools/audit`;
  - three SHA "abc" re-tests in `tools/bench`;
  - four percentile and escape re-tests.
- The owner, `tools/bench-support`'s own tests, stays.

**C. Removed or out-of-scope surface (rulings R6, R7).**
- **R6, isolated kernel benchmarks that are not real host paths:**
  - D1 and D4-D5: the MQ-1/MQ-2 benchmark tests and markers, and the ignored ns-per-frame
    printouts in builtins, parametric-eq, multiband, true-peak-limiter, soft-clip and lane;
  - the `tools/bench/src/gate_active.rs` and `multiband_active.rs` test modules;
  - done together with issue 10's runner scripts.
- **R7:**
  - `builtins/tests/contract.rs:297`, `conformance/tests/effect_contract.rs:240` and
    `conformance/tests/fixtures.rs:34` (extended research-rate tiers);
  - H55 (retired pre-causal descriptors,
    `effect-package/tests/descriptor_v1_qualification.rs:876,967,1043`).
- **Not here, because they guard live product code:**
  - `session/tests/render_mode_tiers.rs:59`, `:72` guard the refusal of a token the grammar still
    knows (`session/src/model.rs:97`). They go with the token, in a product issue, if R7 says so.
  - H56, `engine/src/realtime/disjoint.rs:1016`, is the data-race probe for
    `unsafe impl Sync for DisjointArena` (`:86`). It goes only with the multi-lease code and that
    impl.

**Keep, and do not delete** the `cfg(not(x86))` stubs (`lane/tests/g6_ftz_inert.rs:123`,
`tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`) and `target-smoke`'s per-target half. With
AArch64 back in scope they are the only tests that would run there; issue 12 makes them real.

## Scope

Authorized paths: the test files named in the rows, `nightly.yml` if a removed ignored test was
listed there (none is), and this issue's spec. No product code.

## Gates

1. **Cannot-fail rows.** For each row in part A, the PR states the one-line reason it cannot fail.
   Before deleting, the implementer applies a mutation the row's name claims to guard, in a scratch
   branch, and the row stays green. For example: reorder the reduction for D8; change
   `EFFECTIVE_CASCADE_DEPTH`'s *use* rather than its value for D11.
2. **Mutation equivalence** in the audited crates the rows touch, before and after, same arguments:
   - `../tools/run-mutants.sh compressor 5 10`;
   - `graph-compiler 5 10`;
   - `host-core 5 10 --features control-provider,test-support --shard 0/4 --sharding round-robin`;
   - `parametric-eq 6 12 --shard 0/4 --sharding round-robin`. The caught set is identical before and after. In the
   audit's baseline:
   - D14, D20 and the MQ-2 marker and spike tests catch nothing;
   - D5's preflight (108 mutants), D13 (54) and D15 (36) catch mutants, but none uniquely: every
     one is also caught by a kept test.
3. **Historical bugs.** `../tools/revert.py` for #966, #970, #994 and #1015. The red set is
   unchanged, and no row here is a reproducer.
4. **Counts.** The PR reports the before and after `#[test]` count and the number of test binaries.

## Saving and risk

- **Saving:** about 1 s of runtime and a few binaries to link. The real saving is roughly 150 fewer
  tests to read and re-pin.
- **Risk:** nil for parts A and B, whose surviving guards are named per row. For part C it is the
  owner's rulings.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Part C is settled by filed work. Drop it.**
   - R6 is decided by footprint R9. #1025-#1028 delete the MQ-1/MQ-2, `gate_active` and
     `multiband_active` code and tests. The ignored ns-per-frame printouts (D4) fall under the same
     ruling: delete them in part A.
   - R7's extended-rate rows are deleted by #1036 (footprint R5).
   - H55 and `effect-package/tests/package_v1_qualification.rs:371` are deleted with the crate by
     #1037.
2. **Drop rows in code the filed issues delete:**
   - `source/src/native_source.rs:3949`, `:4435` (#1035);
   - any `effect-package` row (#1037);
   - `tools/bench/src/floor.rs:608`, only if #1039 or #1025 deletes it.

   Re-list the remaining rows against the tree at implementation time.
3. **Spot check: every row sampled holds.** I read D8, D11, D14 and D16, and ran D16 empirically.
   - D16 `rack/tests/console_bank.rs:405`: with the shunt allocated unconditionally
     (`rack/src/lib.rs:1011-1014`), **every** rack, capi, graph-compiler and host-core test stays
     green.
   - So D16 cannot fail on its claim, and its named guard (`:557`, an observation test) does not
     guard it either. Delete it, and correct the row: the claim has no guard. That is harmless,
     because console-free banks use `EffectBankStage`.
4. **Gate 2.** Mutation equivalence applies only in crates the audit measured. For every other row,
   gate 1's cannot-fail demonstration is the gate, and the PR records it per row.

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1046-tests-without-claims` from `codex/batch-slim-4` at `351ca593`
(`main` `a8955ad4` plus the specs of #1069-#1073). Commits: `cb489843` (the deletions), `a55c835b`
(one row restored, below) and this record. Every mutation, bug re-injection and mutant run below ran
on a scratch copy (`git archive` of `351ca593` or of the change), never the checkout.

**Size.** 37 files, +51 / −1,977 (`git diff --numstat 351ca593 a55c835b`). No product code: every
Rust hunk is a `tests/` file or a `#[cfg(test)]` module (`multiband-compressor/src/split.rs` is
`#[cfg(test)] mod split`). Outside the named test files: `crates/soft-clip/tests/MUTATIONS.md` loses
the bullet that named the deleted `descriptive_bench.rs`, and `compressor/tests/mono_collapse.rs`'s
module doc no longer names the test deleted from it.

### The recount on this base

Rows that were already gone at `351ca593`, so nothing was done here:
- D1, D5 (MQ-1/MQ-2 tests and markers) and the `gate_active.rs`/`multiband_active.rs` modules: #1027
  (`b5aa74fb`). D6 (`print_pins`, `print_digests`): #1048 (`a7f6fd65`).
- H1-H3: #1056 (`c46d5b36`). H9 (`tools/audit/src/source.rs`), H13, H17: #1035 (`fecafc00`). H14, H55
  and `effect_interchange.rs`'s SHA re-test: #1037 (`c59f9345`). H37's `source_fixture.rs` copy: #1033
  (`5e15bc96`). H38/H39's `graph.rs`, `builtins.rs`, `rack.rs`: #1026 (`560f2119`);
  `conformance.rs`, `session.rs`: #1039 (`9530ee3d`).

Part C (R6, R7) leaves nothing to delete here:
- The R7 extended-rate rows (`builtins/tests/contract.rs:297`, `conformance/tests/effect_contract.rs:316`,
  `conformance/tests/fixtures.rs:34`) were rewritten by #1036: each now asserts that 176.4-384 kHz is
  *refused*, which is live product code, not removed surface.
- `session/tests/render_mode_tiers.rs:59`, `:72` were replaced by #1063 with
  `retired_dependency_waves_is_an_unknown_enum_value`, which guards the parser's refusal. Kept.
- H56 (`engine/src/realtime/disjoint.rs:989`) stays, as the spec says: the multi-lease builder
  (`:506`) and `unsafe impl Sync for DisjointArena` (`:86`) still exist.

Kept after checking (the brief: a test with a unique catch stays):
- **D3** `gate-expander/tests/identity.rs:229`. #1017 turned it into an AArch64 check: it is
  `#[ignore]`d on x86 with its reason and, on a four-lane build, asserts that the W4 bank binds
  (`expect("a four-lane build binds W4")`). It no longer asserts nothing.
- **D12** `effect-runtime/tests/state_payload.rs:25` `expected_sizes_account_for_the_header`. The
  mutation `StatePayloadSizes::total()` → `common + left` (m12b) is red on this test **and on no
  other**, over effect-runtime, all eight effect crates, conformance, effect-compiler, host-core and
  capi. `total()` has no caller outside tests once D2's `println!` goes. Its other two mutations are
  not unique: `data_words()` dropping a lane (m12a) is red on 11 kept tests; the header left out of
  the common size (m12c, the named claim) is red on 88.
- **H11** `session-validator/tests/validate.rs:432` `the_report_is_deterministic` (restored in
  `a55c835b`). A render that varies per call (a static counter folded into `ValidationReport::render`,
  mh11) is red on this test **only**: no other test compares rendered report text, and the guard the
  row named (`:401`) reads the canonical JSON, not the report.

### Deleted: 41 tests, 7 test binaries

Each row: why it cannot fail on its named claim, the gate-1 mutation of that claim (applied alone to
the base) and its result, and where a side-effect catch goes instead. "Audit" means
`data/mutation-summary.md`'s pass at `a9414c0c`.

**Assert nothing, or run nowhere**
- D2 `parametric-eq/src/lib.rs:4457` `resident_sizes_are_measured_separately_from_serialized_state`:
  seven `println!`s of `size_of`, no assertion; `size_of` is resolved at compile time. Audit: no catch.
- D4, the eight ignored kernel timings (owner R9: only real host paths are benchmarked). None is named
  by any workflow or script (`git grep` of `.github/` and `scripts/` at the base), so none can run in
  CI. `b1_speed` asserts only `block_best > 0.0`; the others print.
  - `builtins/tests/speed.rs:65`, `multiband-compressor/tests/descriptive_frame_cost.rs:27`,
    `true-peak-limiter/tests/bench.rs:76`, `soft-clip/tests/descriptive_bench.rs:82`,
    `lane/tests/b1_speed.rs:101`, `lane/tests/b2_interleave.rs:175`: the whole file.
  - `parametric-eq/tests/bank.rs:582` `descriptive_bank_throughput`.
  - `multiband-compressor/src/split.rs:849` `the_split_costs_what_it_costs`, with the measurement
    harness only it used (`OBSERVATIONS`, `Arm`, `percentile`, `traffic`, `Subject`, `measure`).
- H16 `session/tests/descriptive_scale.rs:54` (whole file): ignored timing printout; its runner
  script is gone and nothing else names it.

**Tautologies and restatements**
- D8 `graph/src/lib.rs:4528` `reduction_is_left_to_right` (`[1,2,3]` sums to 6 in any order) and D9
  `:4541` (the error bound admits every order; the shuffle is sorted back before reducing).
  - m8a, `reduce_left_to_right` reduces in reverse: both GREEN. Red: graph-compiler's
    `graph_fixture` bin `checked_in_fixtures_are_the_generated_bytes` (the D9 residual fixture).
  - m8b, the last contribution dropped: both red, and the same fixture test red. No unique catch.
- D10 `graph/src/runtime.rs:8294`: repeats the `const` assertion at `runtime.rs:2135`. m10, a field
  added to `UnitIdentityWithoutFlags`: compile error E0080 "a UnitIdentity flag no longer fits the
  row's padding". The runtime asserts can never execute red.
- D11 `parametric-eq/src/lib.rs:4625`: two constant asserts. m11, the stereo interleave run at depth 1
  instead of `EFFECTIVE_CASCADE_DEPTH` (the use, not the value): GREEN. Red: `boundary_fold::the_folded_verdict_is_the_boundary_scan`,
  `elision::an_admitted_plan_runs_every_pair_select_free`,
  `elision::an_odd_tail_without_dry_lanes_skips_the_select`. Divisibility is also a
  `debug_assert_eq!` on every render (`lib.rs:1931`, `:2492`). Audit: no catch.
- D13 `compressor/tests/observation.rs:125` and `true-peak-limiter/tests/observation.rs:148`
  `a_resident_read_is_repeatable_to_the_bit`. Their own documented red mutation, a read that
  "freshens" the smoother, cannot be written: m13 and m13b are compile errors E0594 (`&self`) in both
  crates. Side effects: the limiter's left reading forced to `0.0` (m13c) is red on its test and on
  `the_limiter_reads_the_reduction_word_the_envelope_persists` plus three pinned scenarios; the
  compressor's 54 catches are all caught elsewhere (gate 2).
- D14 `compressor/tests/mono_collapse.rs:163`: IEEE arithmetic on a literal; runs no compressor code.
  985-M1, the Average collapsed link simplified to the `abs` arm, is recorded GREEN everywhere
  including this test (`compressor/tests/MUTATIONS.md:263`). Audit and gate 2: no catch.
- D15 `compressor/tests/ramps.rs:587`: m15, the stationary hoist removed from
  `LinearRamp::set_target`: GREEN (a no-op ramp renders the same bits; row 24). Red:
  effect-runtime `stationary_hoist.rs:71`, `:182`, `:213`, compressor
  `native_points.rs:524`, `:739`, `scenario_1006_ramping_prefix_is_pinned`,
  `the_ramping_prefix_renders_the_pinned_bank_scenario`. Its 36 catches are caught elsewhere (gate 2).
- D16 `rack/tests/console_bank.rs:405`: m16, the shunt allocated unconditionally: all 55 rack tests
  GREEN, this one included (amendment 3 reproduced; the claim has no guard, as it said).
- D17 `lane/tests/fp_env.rs:341`: m17a, `FP_ENV_CONTROLLED` without `aarch64`: GREEN on x86 (it
  compares the constant to the `cfg!` that defines it; its `size_of` branch runs only on a target
  with no control word, which no leg tests natively). `lane/tests/f64_lane.rs:191`: m17b, `f64x8`
  implemented at width 4: compile errors E0308 and E0080, from the `const` asserts at
  `f64_lane.rs:214-216`.
- D18 `dsp-reference/src/true_peak_limiter.rs:465`: m18a, taps read oldest first: red here and on the
  limiter's E2 `bs1770_annex2_conformance_is_unchanged` (`lib.rs:5237`). m18b, one table coefficient
  changed: GREEN here (the one-hot returns whatever the table holds), red on E2.
- D19 `builtins-compiler/src/lib.rs:9862`: counts only the test module's `AliasObserver`
  thread-locals; no product code runs. No mutation applies.
- H4 `session/tests/invalid_matrix.rs:1166`: sums a test-local array. H5 `:1206`: `matches!` over
  both variants of two closed two-variant enums (`model.rs:447`, `:464`). Neither can fail.
- H6 `engine/src/lib.rs:113`: `QuantumFrames(128).0 == 128`, a tuple field. `:118`: mh6, the version
  set to 0.1.1, is red on this test only. It pins a number the spec records no requirement for; it is
  a change detector, not a defect catch, and cargo-mutants makes no mutant of a `const`.
- H7 `host-web/src/tests.rs:6443`: literal arithmetic plus `std` sizes; its one engine term,
  `WebMeterHeader`, is pinned at `tests.rs:390-391`. It cannot see the struct it names: at the base
  `ReadyOwnership` (`lib.rs:2080`) already carries two observation fields its sum leaves out
  (`observation_arm_samples`, `meter_frame`), and it is green.
- H8 `bench-support/src/stats.rs:140`: `per_mille` is `nearest_rank(.., 1000)` by definition. mh8b,
  `nearest_rank` rounds down: GREEN. Red: `small_samples_round_up_to_the_next_rank`,
  `percentiles_sort_an_unsorted_short_sample`.
- H10 `parameter-metadata/tests/abi_layout.rs:1348`: mh10, a render that varies per call: red here
  and on `the_checked_in_self_test_fixture_is_current` (`:1354`).
- H12 `tools/bench/src/floor.rs:608`: mh12, one op added to the compressor-only row: red here, on
  `the_current_effect_recount_keeps_fractional_link_work_and_composes_the_strip` and on
  `rust_and_jq_floor_tables_have_exact_key_value_parity`.

**Test tooling**
- D20, the four `shared_hex_adapter_matches_literal_bytes` (compressor `cross_target.rs:29`, limiter
  `determinism.rs:45`, parametric-eq `determinism.rs:28`, transient-shaper `cross_target.rs:63`):
  m20, `bench_support::digest::hex` upper-cases: red on all four and on bench-support's
  `hex_encodes_empty_leading_zero_and_each_nibble` and three more digest tests. The three `hex`
  adapters that only the deleted test called go with it; parametric-eq's stays (its corpus test uses
  it).
- D21 `transient-shaper/tests/allocation.rs:112`, `:117`. The file's zero-allocation gate now reads
  `bench_support::alloc`'s per-thread counters (after `assert_installed`), so its private allocator
  and both positive controls go. They duplicate bench-support's
  `current_thread_counts_every_allocator_operation` (`alloc.rs:267`) and
  `foreign_thread_counts_are_global_but_not_current_thread` (`:304`). Row 11
  (`black_box(Vec::<u8>::with_capacity(1))` in `Shaper::process_block`) on the migrated gate: RED in
  dev and release, `left: 2000, right: 0`, as recorded.
- H37 (part B), the four remaining `shared_sha256_alias_matches_published_literals` in `tools/audit`
  (`fixture_builtins.rs:5168`, `fixture_builtins_listening.rs:587`, `vectorization.rs:502`,
  `builtins_fixture_check.rs:508`): mh37, `sha256_hex` upper-cases: red on all four and on
  bench-support's `matches_the_published_vectors`.

### `cargo test -- --list` diff

Base `351ca593` against `a55c835b`, the 21 changed packages with CI's features (debug-a's for
graph, rack, builtins-compiler, session, engine, host-web, bench-support, parameter-metadata and
session-validator; debug-b's for the DSP crates and dsp-reference; release for audit and bench), each
test executable listed by package and target: **1,403 → 1,362 tests, 158 → 151 test binaries**, 41
names removed and none added.

```text
- audit bin:audit: builtins_fixture_check::tests::shared_sha256_alias_matches_published_literals
- audit bin:audit: fixture_builtins::tests::shared_sha256_alias_matches_published_literals
- audit bin:audit: fixture_builtins_listening::tests::shared_sha256_alias_matches_published_literals
- audit bin:audit: vectorization::tests::shared_sha256_alias_matches_published_literals
- bench bin:bench: floor::tests::the_compressor_isolate_is_the_compressor_inventory
- bench-support lib:bench_support: stats::tests::the_numerator_denominator_form_matches_the_per_mille_form
- builtins test:speed: bench_input_stage_ns_per_track_frame
- builtins-compiler lib:builtins_compiler: tests::alias_observer_counts_and_capture_are_thread_local
- compressor test:cross_target: shared_hex_adapter_matches_literal_bytes
- compressor test:mono_collapse: a_halved_subnormal_does_not_come_back
- compressor test:observation: a_resident_read_is_repeatable_to_the_bit
- compressor test:ramps: a_restated_point_leaves_every_lane_on_the_idle_body
- dsp-reference lib:dsp_reference: true_peak_limiter::tests::a_unit_impulse_reproduces_the_annex2_table_rows
- engine lib:engine: tests::quantum_is_a_lossless_carrier
- engine lib:engine: tests::version_is_stable_for_bootstrap
- graph lib:graph: runtime::tests::rt9_identity_metadata_has_no_retained_or_peak_layout_delta
- graph lib:graph: tests::left_to_right_reduction_meets_analytic_bound_and_ignores_completion_order
- graph lib:graph: tests::reduction_is_left_to_right
- host-web rlib:host_web: tests::the_observation_fields_account_for_the_moved_bridge_rows
- lane test:b1_speed: b1_block_kernel_against_a_per_sample_path
- lane test:b2_interleave: b2_interleave_sweep
- lane test:f64_lane: gate1_widen_widths_match
- lane test:fp_env: the_target_declares_whether_it_pins
- multiband-compressor lib:multiband_compressor: split::the_split_costs_what_it_costs
- multiband-compressor test:descriptive_frame_cost: descriptive_frame_cost
- parameter-metadata test:abi_layout: rendering_is_deterministic
- parametric-eq lib:parametric_eq: interleave_identity::resident_sizes_are_measured_separately_from_serialized_state
- parametric-eq lib:parametric_eq: interleave_identity::the_tuned_depth_is_per_backend_and_divides_the_cascade
- parametric-eq test:bank: descriptive_bank_throughput
- parametric-eq test:determinism: shared_hex_adapter_matches_literal_bytes
- rack test:console_bank: a_stage_with_no_controlled_lane_allocates_no_shunt
- session test:descriptive_scale: canonical_compile_parse_timings_at_32768_and_65536_tracks
- session test:invalid_matrix: corpus_distribution_totals_124_cases
- session test:invalid_matrix: tagged_route_roles_are_structurally_closed
- soft-clip test:descriptive_bench: descriptive_bank_throughput
- transient-shaper test:allocation: foreign_thread_allocations_do_not_enter_the_callers_counts
- transient-shaper test:allocation: the_counter_observes_same_thread_allocation_and_free
- transient-shaper test:cross_target: shared_hex_adapter_matches_literal_bytes
- true-peak-limiter test:bench: bench_w8_ns_per_lane_sample
- true-peak-limiter test:determinism: shared_hex_adapter_matches_literal_bytes
- true-peak-limiter test:observation: a_resident_read_is_repeatable_to_the_bit
```

### Gate 2: mutation equivalence

- **compressor** (the audited crate the rows touch that has catches). Two phases, `run-mutants.sh`'s
  settings (cargo-mutants 27.1.0, opt-level 1 with debug assertions, `--no-fail-fast`), 525 product
  mutants; `src/corpus.rs` excluded because none of the four deleted tests reaches `compressor::corpus`.
  - Phase 1, the base, only the four deleted compressor tests: 66 caught, 417 missed, 42 unviable, 0
    timeout. By test: `a_resident_read_is_repeatable_to_the_bit` 54,
    `a_restated_point_leaves_every_lane_on_the_idle_body` 36, the other two 0. The audit's 54 and 36.
  - Phase 2, the change, the whole compressor suite (21 binaries, 89 tests in the baseline), those 66
    mutants only: **66 caught**, 0 missed.
  - Deleting a test cannot change another test's outcome, so the caught set after equals the caught
    set before exactly when every phase-1 catch is still caught. It is.
- **parametric-eq**: the deleted tests are D2 (no assertion), D11 (constants; cargo-mutants mutates no
  `const` item), D20 (calls only bench-support) and an ignored benchmark. None can catch a
  parametric-eq mutant. The audit's 1-in-4 shard lists all four under "no catch".
- **graph-compiler, host-core**: no row touches them on this base (H2 and H3 went with #1056).

### Gate 3: historical bugs

`../tools/revert.py` on scratch copies of the base and the change, each bug alone, `cargo test
--no-fail-fast` over the recorded reds' packages plus every changed package that depends on the bug's
crate: the whole DSP and graph packages, and for host-web, parameter-metadata, session-validator,
bench (release) and audit (release) the binaries that held deleted tests.

| bug | base: red | change: red | passed, base → change (the difference is the deleted tests in the run) |
|---|---|---|---|
| #966 | 9: the `bank_levels.rs` tests, the randomized probe included | the same 9 | 68 → 61 |
| #970 | 8: the 7 `collapse_arming.rs` reproducers and the `bank_levels.rs` randomized probe; graph lib 0 | the same 8; graph lib 0 | 127 → 119, graph lib 107 → 104 |
| #994 | 10: the knee reproducers (compressor, effect-runtime, multiband `knee_tests::a_band_level_at_the_threshold_never_takes_a_nan_target`) and the three `randomized_differential_*` | the same 10 | 452 → 433 |
| #1015 | 3: the `stationary_subnormal` tests | the same 3 | 182 → 172 |

No deleted test is red on any of the four bugs.

### Gate 4 and the other gates

- **Counts.** `#[test]` attributes under `crates/`, `hosts/`, `tools/`: 2,221 → 2,180 (the 41). Test
  binaries in the changed packages: 158 → 151. Top-level `tests/*.rs` files: 200 → 193.
- `cargo fmt --all -- --check`; `cargo check` and `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`: pass.
- The changed packages' tests, CI features:
  - dev: 643 + 660 + 53 passed, 0 failed (4 + 3 ignored);
  - release: 660 + 53 passed, and 642 of 643 in the debug-a set. The failure is
    `bench-support` `alloc::tests::current_thread_counts_every_allocator_operation`, **pre-existing**:
    it fails identically on the base in release (LLVM removes the unused `alloc_zeroed`/`dealloc`
    pair; `left: allocations 2, right: 3`). CI runs it only in debug. Not in this issue's scope.
- Console digests (`gain_pan_profile digests`, release): 17 rows, byte-identical on the base and the
  change.
- Policy: 52 of 52 steps pass: every hermetic policy script of `qualification.yml`'s route and lint
  jobs, plus the router-gated self-tests (`test-env-vocabulary.sh`, `test-conformance-boundaries.sh`)
  and `aarch64-known-defects.py --self-test`, Python under `python3 -B`. This includes #1052's
  source-scrape lint in `check-workspace-policy.sh` and its planted case in
  `test-workspace-policy.sh`; no deleted or edited test reads source text.
- AArch64 debug leg, its exact package and feature set (`product-crates.sh`: 25 crates, plus
  `dsp-reference`, `conformance`, `target-smoke`), resolved on x86: `cargo test --no-run` builds 211
  executables; the `-- --list` (1,759 tests) passes `aarch64-known-defects.py judge-skips debug`, and
  the release leg's `lane`/`math` listing passes `judge-skips release`. Each of the four known-defect
  rows names one existing test; none is a deleted test. The leg's no-silent-skip scan finds nothing.

### Left as found, for the reviewer

- `crates/lane/src/lib.rs:202` and `docs/rulings/cross-bank-interleave.md:33` cite
  `tests/b2_interleave.rs` as where `SVF_CASCADE_DEPTH` was measured, and
  `crates/soft-clip/src/lib.rs:22` cites `tests/descriptive_bench.rs` for its 246 → 3.0 ns. They record
  where a past measurement was made (the files stay in history at `351ca593`). A product file and a
  ruling are outside this issue's paths, so they are unchanged.
- effect-runtime's `StatePayloadSizes::total()` has no production caller; D12 is its only test.
- The release-only bench-support failure above.
