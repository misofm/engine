# Shrink the slow tests that a cheaper test dominates

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.1 and §5 item 9;
[`../data/mutation-summary.md`](../data/mutation-summary.md)). Base `a9414c0c`. Paths starting `../`
are relative to the audit's handoff folder. No ruling needed, except that trimming extended research
rates out of builtins `response.rs` follows R7.
Every claim stays; only its size or its per-PR share changes.

## Problem

121 tests of 1 s or more take 90 % of test execution time. The mutation pass shows that several of the
slowest catch nothing a much cheaper test does not also catch. "Dominated" means one other single
test catches every mutant this test catches.

| test | local s | evidence |
|---|---:|---|
| compressor `src/kernel.rs` `settled_body_tests::the_all_wet_arm_is_the_base_body_on_all_wet_tables` | 23.4 | identical 142-mutant catch set to `the_collapsed_settled_body_is_the_base_body_on_the_three_parameter_sets` (4.7 s); its recorded reds 982-M1 and 995-M5 are also red on the randomized differentials (`crates/compressor/tests/MUTATIONS.md:221`, `:293`) |
| parametric-eq `src/lib.rs` `elision::the_two_channels_are_judged_together` | 24.6 | dominated by `an_elided_cascade_is_the_full_cascade_bit_for_bit` (5.0 s) |
| parametric-eq `tests/time_domain.rs:155` `forty_eight_frozen_million_sample_sequences_remain_valid_without_recovery` | 13.2 | dominated by a 0.4 s test; no `MUTATIONS.md` row is red only here (`../data/candidates-dsp-graph.md` item 54) |
| parametric-eq `ramping_elision::*_simd4` / `*_simd8` (six randomized differentials) | 62 in total | the `Simd8` twins are dominated by the `Simd4` twins for the sampled mutants, **but** 1005-M5d is recorded red only at W8, so both widths stay |
| graph-compiler meter-pass tests: `the_full_meter_pass_renders_without_an_audited_event`, `the_banked_sample_peak_pass_renders_without_an_audited_event`, the two `post_matrix_*` tests, `a_bank_of_mixed_periods_and_metric_sets_publishes_the_declined_arms_frames`, `the_full_meter_pass_stays_off_where_no_meter_can_commit_it` | 22.6 + 21.2 + 13.1 + 12.7 + 6.3 + 5.9 | a targeted pass over `graph`'s bank meter pass: `an_observer_failing_mid_bank_leaves_every_later_meter_as_the_declined_arm_does` (2.6 s) catches every mutant they catch. They are the *only* real guards of that code (graph's own tests catch 1 of 22), so they are shrunk, not deleted. The allocation claim of the first two is a property no mutant exercises |
| compressor scenario pins 981/982/983/995/985 (`src/kernel.rs:3352`, `:3411`, `:3461`, `:3503`, `:3578`) | ≈ 1 | frozen "no bit moved versus the pre-slice base" digests whose slices have landed. Every recorded red is also red on gate 1 (`../data/candidates-dsp-graph.md` item 50) |
| builtins `tests/response.rs:356`, `:422`; `:880` | ≈ 32 | the same cascade measured six ways, no mutation separates them; extended research rates included; `:880` designs about 9.4M cutoffs (items 55-56) |
| graph-compiler `src/lib.rs:15681` `canonical_artifacts_are_complete_and_repeatable_100_times` | small | 100 in-process repeats cannot see what `check-graph-determinism.sh` (100 fresh processes) sees (item 59) |
| conformance `tests/mutation_million.rs:10` | ≈ 2 | 1M decodes; asserts `first == second` and the corpus size (`:18-26`) (item 58) |
| true-peak-limiter `tests/gain_law.rs:110` | 6.8 | 1,152 configurations; the rate is not a branch of the ceiling law (item 60) |

## Outcome

- **Delete** the compressor all-wet grid test. Keep its dispatch assertion, "the arm is taken
  exactly when every lane is wet", which gate 3 already asserts. **Delete** the five
  compressor scenario pins.
- **Shrink** `the_two_channels_are_judged_together` to the channel pairs that separate a judgement,
  and each graph-compiler meter test to the smallest console that still reaches its arm, for
  example 8 tracks instead of 64. Each keeps its claim. One full-size (64-track) run of
  `the_full_meter_pass_renders_without_an_audited_event` moves to nightly, because an allocation
  that appears only at size cannot be seen at 8 tracks.
- **Per-PR seeds ÷ 4** for the six ramping-elision differentials. `nightly.yml` runs the full seed
  count.
- **Move to nightly:** the 48 × 1M sequences, keeping a 4-row debug representative, and
  `mutation_million` at 1M, keeping 10,000 per PR.
- **Trim:**
  - builtins response to launch rates, one DFT per (rate, cutoff, kind);
  - `:880` to stride 4,096 plus the endpoints and the seam;
  - the 100 repeats to 2;
  - the limiter's E4 to 288 configurations: rates {44.1 k, 96 k} × ceilings {-1, -12} × lookaheads
    {0, 1, 10} × both releases × 6 corpora.

## Scope

Authorized paths:
- `crates/compressor/src/kernel.rs` (test module only);
- `crates/parametric-eq/src/lib.rs` (test modules only), `crates/parametric-eq/tests/time_domain.rs`;
- `crates/graph-compiler/src/lib.rs` (test module only);
- `crates/builtins/tests/response.rs`;
- `crates/conformance/tests/mutation_million.rs`;
- `crates/true-peak-limiter/tests/gain_law.rs`;
- `.github/workflows/nightly.yml`;
- this issue's spec.

No product code.

## Gates

1. **Mutation equivalence** with `../tools/run-mutants.sh`, before and after, same arguments. The
   caught sets must be identical:
   - `compressor 5 10`, all mutants;
   - `parametric-eq 6 12`, with `--shard 0/4` **and** `--shard 1/4`, round-robin;
   - `graph 3 8 --file crates/graph/src/runtime.rs -F meter --test-package graph --test-package
     graph-compiler --features graph/test-support`, the audit's targeted pass (22 caught);
   - `graph-compiler 5 10`, all mutants.
2. **Recorded catches still red per PR.** Re-apply these from the crates' `MUTATIONS.md`; the shrunk
   tests still turn them red *per PR*, not only nightly:
   - 1005-M5d (W8 only) and 1006-M1 (red only at `Simd4`/`Simd8`);
   - graph G-2, A-5, 936-4 and 957-1b, which are red on the `post_matrix_*` tests and are the
     evidence issue 07 relies on to delete the graph source scrapes.
3. **The realtime claim holds.** The shrunk `…_renders_without_an_audited_event` tests still fail on
   one allocation injected into the bank meter pass.
4. **Historical bugs.** `../tools/revert.py 994` and `1015`. The red set is unchanged, including the
   compressor's randomized differentials and `stationary_subnormal`.
5. **Cost.** On a full-route PR:
   - test-debug-b: the parametric-eq lib binary (28.8 s in CI) and the compressor lib binary
     (20.9 s) are each at least 40 % shorter;
   - test-debug-a: the graph-compiler lib binary (46.4 s) is at least 30 % shorter.

   Record them from the job logs' `finished in` lines.

## Saving and risk

- **Saving:** about 170 s of serial test time. Because CI runs tests on 4 vCPUs, expect about 40-60 s
  off test-debug-b and about 30 s off test-debug-a.
- **Risk:** low, bounded by gate 1. A width-only or arm-only defect is protected by gate 2's
  recorded mutations.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **A deletion needs full mutation equivalence, not a shard (finding F8).**
   - Deleting `the_two_channels_are_judged_together`, the compressor all-wet grid or the five
     compressor scenario pins requires equivalence over **all** mutants of the files those tests
     exercise, with `--file`.
   - 1005-M5d, a W8-only catch that sampling missed, shows why.
   - Shrinks and seed reductions may use shards.
2. **Reproduced.** The compressor all-wet grid (23.4 s) has exactly the 142-mutant catch set of
   `the_collapsed_settled_body_is_the_base_body_on_the_three_parameter_sets` (4.7 s). This was a
   full 654-mutant re-run.
3. **The builtins response trim to launch rates overlaps #1036.** Land after it, and keep only the
   one-DFT-per-(rate, cutoff, kind) and stride trims.
4. **Cost gates are HEAD-specific.** The graph-compiler lib binary grew from 11 s to 46 s, and the
   compressor lib from 0 s to 21 s, in the #1016 batch. Measure "before" on the base of this
   change, not on the audit's figures.
5. **Gate 1 with `--test-package graph-compiler`.** Baseline the graph-compiler tests unmutated
   first (F4).

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1049-shrink-dominated-slow-tests` from `codex/batch-slim-1`
(`92ef396f`). Commits: `3c79c99f` (the shrink), `284e4aff` (per-PR vector seeds at stride 4),
`86e989fa` (nightly comment placement), `2069e445` (what the audited meter tests count), and this
record. Test modules, test files and `nightly.yml` only: every hunk under `crates/` is inside a
`#[cfg(test)]` module or a `tests/` file, so no product line moved and the mutant keys of the
before and after passes are the same.

**Size.** 8 files, +423 / −556 lines before this record (`git diff --numstat 92ef396f 2069e445`).
The compressor test module alone is −364 net.

### What changed

| item | before | after | full size |
|---|---|---|---|
| compressor all-wet grid (#982 gate 1) | 5 grids | deleted | its 142-mutant catch set is the collapsed-body test's (gate 1) |
| compressor scenario pins 981/982/983/985/995 | 5 digests | deleted | `scenario_1006` and the randomized differentials catch every mutant they caught (gate 1) |
| `elision::the_two_channels_are_judged_together` | 4,096 mask pairs | 192: every live union, each section live on the left only, the right only or both, rotated so each section takes each state | — |
| six `ramping_elision` `Simd4`/`Simd8` differentials | 40 scenarios (dev), 300 (release) | 10 / 75: every fourth seed from seed 2 | `the_vector_differentials_render_their_bits_at_the_full_scenario_count`, `#[ignore]`d, nightly |
| `time_domain` 48 × 1M sequences | 48 rows | `four_frozen_million_sample_sequences_…`: one row per launch rate, both edges, four families, incl. the 44.1 kHz / 20 kHz / +24 dB / Q 18 bell | the 48-row test, `#[ignore]`d, nightly |
| graph-compiler bank meter-pass tests (6) | 64-track intended console | its first 8 tracks and their routes (`intended_console`, `METER_CONSOLE_TRACKS`): 1 cohort at `Simd8`, 2 at `Simd4`, the mixed-periods pattern once | `…_renders_without_an_audited_event_on_sixty_four_tracks`, `#[ignore]`d, nightly |
| `canonical_artifacts_are_complete_and_repeatable_100_times` | 100 recompiles | renamed `…_repeatable`, 2 | `check-graph-determinism.sh` (100 fresh processes, audit-native) |
| builtins `one_second_impulse_dfts_…` | a DFT per quantum (5) | one DFT per (rate, cutoff, kind); the other 4 partitions held to its bits | — |
| builtins `representable_cutoff_domain_…` | every f32 cutoff (~9.4M designs) | stride 4,096 + both endpoints + `maximum − 1` (the seam) | — |
| conformance `mutation_million` | 1,000,000 | `ten_thousand_…` | the 1M test, `#[ignore]`d, nightly |
| true-peak-limiter E4 | 1,152 configurations | 288: {44.1 k, 96 k} × both links × {−1, −12} × {0, 1, 10} ms × both releases × 6 corpora; asserts the count | — |
| `nightly.yml` | — | job `full-size-tests`, dev build, the four `--ignored --exact` commands, and a row in `failure-notice` | — |

Left alone on purpose: builtins `response.rs`'s rate list (#1036 owns it, amendment 3), and the
sustained-sine test (`:422`), whose only allowed trim was the rate list.

### The `-- --list` diff (base `92ef396f` against the change; no name changed after `3c79c99f`)

Per binary, run tests (ignored in brackets): compressor lib 18 → 12; parametric-eq lib 54 [1] →
54 [2]; `parametric-eq --test time_domain` 5 → 5 [1]; graph-compiler lib 89 → 89 [1];
`builtins --test response` 11 → 11; `conformance --test mutation_million` 1 → 1 [1];
`true-peak-limiter --test gain_law` 2 → 2. Every other name is unchanged.

| name | change | who catches its mutants now |
|---|---|---|
| `kernel::settled_body_tests::the_all_wet_arm_is_the_base_body_on_all_wet_tables` | deleted | `the_collapsed_settled_body_is_the_base_body_on_the_three_parameter_sets`: the identical 142-mutant set, re-measured on this base |
| `…::scenario_981_heterogeneous_hostile_render_is_pinned` | deleted | `scenario_1006_ramping_prefix_is_pinned` (all 81) |
| `…::scenario_982_all_wet_render_is_pinned` | deleted | `scenario_1006` (75 of 76), `randomized_differential_f32` (1) |
| `…::scenario_983_chunk_straddling_render_is_pinned` | deleted | `scenario_1006` (81 of 82), `randomized_differential_f32` (1) |
| `…::scenario_985_collapsed_render_is_pinned` | deleted | `scenario_1006` (71 of 72), `randomized_differential_f32` (1) |
| `…::scenario_995_sidechain_render_is_pinned` | deleted | `scenario_1006` (47 of 67), `randomized_differential_simd4` (20) |
| `tests::canonical_artifacts_are_complete_and_repeatable_100_times` | renamed `…_repeatable`, same assertions, 2 recompiles | itself |
| `forty_eight_frozen_million_sample_sequences_remain_valid_without_recovery` | `#[ignore]`, nightly | per PR: new `four_frozen_million_sample_sequences_remain_valid_without_recovery` |
| `one_million_deterministic_mutations_cover_complete_schema_closed_dispatch` | `#[ignore]`, nightly | per PR: new `ten_thousand_deterministic_mutations_cover_complete_schema_closed_dispatch` |
| `ramping_elision::the_vector_differentials_render_their_bits_at_the_full_scenario_count` | new, `#[ignore]`, nightly | — |
| `tests::the_full_meter_pass_renders_without_an_audited_event_on_sixty_four_tracks` | new, `#[ignore]`, nightly | — |

The cover column comes from the base compressor pass's catch matrix (`parse_mutants.py`, then a
greedy cover over the surviving tests). No deleted test held a mutant that only deleted tests
caught.

### Gate 1: mutation equivalence

`cargo-mutants` 27.1.0 through the audit's `tools/run-mutants.sh`, unchanged, on a clean archive
of `92ef396f` and of `284e4aff` (the later commits change comments and `nightly.yml` only), one
pass at a time, `nice`. Each pass builds in `cargo-mutants`' own copies, so no target directory
was shared between before and after. "Same" below means the same mutant keys with the same
outcome, key by key:

| pass (the spec's arguments) | before: caught / missed / unviable / timeout | after | caught set |
|---|---|---|---|
| `compressor 5 10`, all 654 | 513 / 96 / 43 / 2 | 513 / 96 / 43 / 2 | same |
| `graph 3 8 --file crates/graph/src/runtime.rs -F meter --test-package graph --test-package graph-compiler --features graph/test-support` | 22 / 4 / 3 / 0 | 22 / 4 / 3 / 0 | same, and every graph-compiler meter test's own catch set is unchanged |
| `parametric-eq 6 12 --shard 0/4 --sharding round-robin`, 348 | 270 / 31 / 37 / 10 | 271 / 30 / 37 / 10 | same, plus one gained |
| `parametric-eq 6 12 --shard 1/4 --sharding round-robin`, 348 | 267 / 38 / 31 / 12 | 267 / 38 / 31 / 12 | same |
| `graph-compiler 5 10`, all 408 (scale tests included) | 261 / 77 / 67 / 3 | 261 / 77 / 67 / 3 | same |

The deletions (amendment 1): the compressor pass covers every mutant of every compressor source
file, and the deleted tests exercise only the compressor's kernel against its own oracle, whose
other inputs (effect-runtime's dynamics, lane, math) feed both arms alike. On this base, as in the
verification, the all-wet grid's catch set is the collapsed-body test's, 142 mutants. The five
scenario pins held no mutant that a surviving test does not catch (the `--list` table above).
`passes_effect_contract_conformance` caught one mutant more after than before (100 → 101); that
mutant was caught either way.

The shrinks, by test (per-test catch sets, before → after):
- `the_two_channels_are_judged_together`: 40 → 40 (shard 0) and 44 → 44 (shard 1), the same
  mutants.
- `four_frozen_million_…` catches all 34 of the 48-row test's shard-0 mutants, and 33 of its 35 in
  shard 1. The other two (`coef_word_mut` match arm 0, `EqBandKind::from_value` `==` → `!=`) are
  caught after by 33 and 17 other tests.
- The six vector differentials each lost 0-3 mutants and gained 5-11 per shard: a different seed
  set. Every mutant one of them lost is caught after by 7 to 44 other tests, the scalar
  differentials among them. The shard-0 gain, `restore_track` `delete !` (missed → caught), is theirs: with 10
  scenarios under that mutant, their "a restore was refused / accepted" non-vacuity asserts fail.
- The graph-compiler meter tests on 8 tracks: in the graph pass every one catches exactly what it
  caught on 64 tracks, and in the graph-compiler pass the six keep their sets too (66, 66, 68, 68,
  66, 66 mutants, the same keys). The renamed 2-recompile test keeps the 100-recompile test's 59.

### Gate 2: recorded catches still red per PR

Each row applied alone to a scratch copy of the base and of the change, then the per-PR command
in the dev profile (CI's), `--no-fail-fast`, restored. Red sets:

| row | base, per PR | change, per PR |
|---|---|---|
| 1005-M5d (`ramping_sections`' leg (b) → `true`) | `a_ramping_block_renders_the_batch_head_bits_simd8`, `the_unsafe_ramp_rule_keeps_every_dead_section_after_it` | the same two; nightly's full-count test red too |
| 1006-M1 (`advance_where` without the `remaining == 0` hold) | `randomized_differential_simd4`, `_simd8`, `scenario_1006_ramping_prefix_is_pinned`, `ramping_prefix_scenario::the_ramping_prefix_renders_the_pinned_bank_scenario` | the same four |
| #943 G-2 (hand-off reads lane `lane ^ 1`) | gc `post_matrix_peak_meters_…`; graph `resident_meter_entry_has_one_final_output_dispatch_and_admission_control` | the same |
| #950 G-1 (`meters.lane(lane ^ 1, …)`) | gc: 9 meter tests, among them `post_matrix_all_meters_…`, `the_full_meter_pass_renders_…`, `…_stays_off_…`, `a_bank_of_mixed_periods_…`, `an_observer_failing_mid_bank_…`; graph: the source scan | the same |
| #950 G-2 (right plane reads the left) | gc: G-1's 9 without `the_full_meter_pass_renders_…`; graph lib green, as recorded | the same |
| #950 A-5 (`bank_meter_seeds` result ignored) | gc `post_matrix_all_meters_…`; graph source scan | the same |
| 936-4 / 957-1b (every unit dispatched) | graph lib: 3 dispatch-count tests + `rt9_resident_entry_has_one_guarded_production_caller_and_control`; gc green | the same |

The first quarter of the seeds lost 1005-M5d at `Simd8` (it went green there on `3c79c99f`): a
scan of seeds 0-119 with M5d applied is red at `Simd8` only on seeds 26, 46, 53 and 101, and at
`Simd4` on 101, 105 and 106, so the 40-scenario dev range holds one catching seed. `284e4aff`
draws every fourth seed from seed 2, the residue class of 26, and M5d is red per PR again.
936-4 and 957-1b are red on graph's own dispatch tests, not on the `post_matrix_*` tests, before
and after; the spec's "red on the `post_matrix_*` tests" holds for G-2 and A-5.

### Gate 3: cannot pass as written, on the base either; what guards the realtime claim

**Finding.** Gate 3 asks that the shrunk `…_renders_without_an_audited_event` tests fail on one
allocation injected into the bank meter pass. They cannot, and they could not before this change:
`std::hint::black_box(Box::new(0_u64))` at the top of `bank_meter_pass`, and separately of
`bank_sample_peak` (`crates/graph/src/runtime.rs`), leaves both green on the 64-track base and on
the 8-track change, and so does the new nightly 64-track run.

The cause: `audit::snapshot()` (`crates/engine/src/realtime/audit.rs`) counts only what is
reported to it. Allocations are reported only by `bench-support`'s counting `#[global_allocator]`
(`tools/bench-support/src/alloc.rs:173-200`), and the graph-compiler lib test binary does not link
`bench-support`. Its "allocation audit" therefore sees the render path's own hooks (locks, logs,
syscalls and so on) and never an allocation. graph's `rt1_direct_bank_alloc`,
`rt9_resident_bank_input_alloc` and `rt10_source_in_place_alloc` do link the allocator, but they
stay green under both injections: none of them reaches either pass.

**What guards it.** Two `bench` console tests, which run under the audited allocator in abort mode:

| injected into | test that aborts (`SIGABRT`) | green |
|---|---|---|
| `bank_meter_pass` (#950's full pass) | `console::tests::the_meters_record_carries_each_arms_fold_and_redirect_counters` | the other one |
| `bank_sample_peak` (#943's pass) | `console::tests::the_metered_console_row_prints_its_meters_and_the_validator_pins_them` | the other one |

Each was run alone with `--exact`, in dev and in release (`cargo test --locked --release -p bench
--bin bench`), injection by injection. Both pass unmutated. In required CI they run in
`qualification.yml`'s `audit-native` job ("release audit, trace, and fixture gates"), step "Audit
and console-workload unit tests in release": `cargo test --locked --release -p audit -p bench -p
console-workload`. `audit-native` runs on every full-route PR, and the `qualification` verdict
requires its result. This issue does not touch either test or the job.

`2069e445` says this in the two graph-compiler tests' docs. For the graph-compiler tests to see
allocations themselves, graph-compiler needs `bench-support` as a dev-dependency, a `Cargo.toml`
change outside this issue's paths. That is a possible follow-up, not a gap: the claim is already
guarded per PR.

### Gate 4: historical bugs

`revert.py` on scratch copies, each built without reusing another tree's artifacts, then
CI's `test-debug-a` and `test-debug-b` command lines with `--no-fail-fast`
(`CARGO_PROFILE_DEV_DEBUG=0` for disk):

| bug | shard | base `92ef396f` | change |
|---|---|---|---|
| 994 | a | 0 red | 0 red (`3c79c99f`) |
| 994 | b | 10 red: compressor `randomized_differential_{f32,simd4,simd8}`, 3 `knee_overflow`, 3 effect-runtime `dynamics`, 1 `knee_tests::a_band_level_at_the_threshold_never_takes_a_nan_target` | the same 10 (`3c79c99f`) |
| 1015 | a | 0 red | 0 red (`3c79c99f` and `86e989fa`) |
| 1015 | b | 3 red: `stationary_subnormal::*` | the same 3 (`3c79c99f` and `86e989fa`) |

`284e4aff` changed only the parametric-eq ramping seeds, so 994 was not re-run on it; 1015 was.

### Gate 5: cost (local)

The CI figure can only come from a full-route run. Locally: each lib test binary built from
`92ef396f` and from `86e989fa` in the dev profile with the CI features, run whole with
`--test-threads=4` (CI's 4 vCPUs), base and change interleaved, three rounds. The machine has 32
cores shared with other agents (load 26-50) and one niced mutation pass of this issue running.
libtest `finished in`:

| binary | base (3 rounds) | change (3 rounds) | median | gate |
|---|---|---|---|---|
| compressor lib (test-debug-b) | 36.18, 35.17, 33.74 s | 10.37, 8.46, 8.69 s | 35.2 → 8.7 s, −75 % | ≥ 40 % |
| parametric-eq lib (test-debug-b) | 44.69, 37.99, 37.30 s | 12.28, 11.00, 15.99 s | 38.0 → 12.3 s, −68 % | ≥ 40 % |
| graph-compiler lib (test-debug-a) | 39.85, 39.43, 44.59 s | 14.72, 14.92, 14.66 s | 39.9 → 14.7 s, −63 % | ≥ 30 % |

Serial cost of the tests this issue touches, each run alone (`--exact --test-threads=1`), base and
change timed back to back under the same load:

| tests | base s | change s |
|---|---:|---:|
| compressor: all-wet grid, 5 scenario pins, collapsed-body test | 39.6 | 6.8 |
| parametric-eq: two-channel judgement | 32.5 | 1.4 |
| parametric-eq: six vector ramping differentials | 87.1 | 21.1 |
| parametric-eq `time_domain`: 48 × 1M → 4 × 1M | 21.9 | 1.8 |
| graph-compiler: six meter tests, 100 → 2 recompiles | 107.6 | 13.8 |
| builtins `response.rs`: impulse DFTs, cutoff domain | 31.9 | 21.1 |
| conformance `mutation_million` | 1.1 | 0.0 |
| true-peak-limiter E4 | 10.8 | 2.5 |
| **total** | **332.3** | **68.5** |

So about 264 s of serial test time per PR, against the spec's estimate of 170 s. The nightly job
runs the full sizes in about 150 s of tests (95 s vector differentials, 25 s for the 48 sequences,
28 s for the 64-track audited run, 1 s for the million frames), plus its three dev builds; the
exact commands were run locally from the YAML and pass.

### Other gates

- `cargo check --locked --workspace --all-targets --all-features`, `cargo clippy --locked
  --workspace --all-targets -- -D warnings` (re-run for graph-compiler after `2069e445`) and
  `cargo fmt --check`: pass, no warning.
- The affected crates' tests, CI features, `--all-targets`: compressor, parametric-eq, builtins,
  conformance and true-peak-limiter (68 binaries) and graph-compiler (7 binaries), dev and
  release: all pass. graph-compiler's release `--all-targets` run needs
  `CARGO_PROFILE_RELEASE_PANIC=unwind`, as `scripts/run-release-workspace-tests.sh` explains
  (without it, the `effect-package` cdylib clobbers itself and the build fails with a spurious
  E0463). Nothing in this change touches that.
- The four `#[ignore]`d tests pass, run alone and through the nightly job's exact commands.
- Every `scripts/check-*.sh` passes on the final tree, except `check-sdk-types.sh`, which needs
  `npm ci` in `sdk/` (network; not run). That includes `check-workspace-policy.sh`, whose #1052
  source-scrape lint passes with its allowlist unchanged (no scrape was added or removed), and
  the artifact builders (`check-cross-targets.sh`, `check-web-audioworklet.sh`,
  `check-protocol-wasm-parity.sh`, `check-capi-abi.sh` and the rest). The argument-free
  `check-*.py` pass. `check-ci-path-routing.py` failed once on `3c79c99f`: it reads the
  `release-budgets` job up to the next job key, so a comment above `full-size-tests:` counted as
  budget-step content; `86e989fa` moved the comment inside the job. `test-ci-path-routing.py`
  passes.
- F4 (amendment 5): the graph targeted pass's exact test set (`--package=graph
  --package=graph-compiler --features graph/test-support`, `run-mutants.sh`'s environment,
  opt-level 1), unmutated: 239 passed, 0 failed, on the base and on the change.

### Integration commit (merge of `codex/batch-slim-3`, `a509b681`)

The merge was clean (`nightly.yml`, `response.rs`, `graph-compiler/src/lib.rs` and
`parametric-eq/src/lib.rs` auto-merged). On the merged tree:
- `bash -n scripts/run-aarch64-tests.sh`; `aarch64-known-defects.py --self-test`; `rows debug`
  lists the two survivors.
- The debug leg's exact package and feature set (from `scripts/lib/product-crates.sh`, as the
  script builds it), on x86: `cargo test --locked --all-targets … --no-run` resolves and builds
  (218 executables); its `-- --list` (1,792 tests) passes `judge-skips debug`, so each remaining
  row names exactly one test.
- `cargo check --locked --workspace --all-targets --all-features`: pass, no warning.
- compressor, parametric-eq, builtins, conformance, true-peak-limiter and graph-compiler tests,
  dev, CI features: pass.
- `check-ci-path-routing.py` and `test-ci-path-routing.py`; every argument-free `check-*.py`; and
  all 34 `check-*.sh` but `check-sdk-types.sh` (needs `npm ci`): pass, with `python3 -B`.

### For root

- **#1017's AArch64 rows.** Done in the integration commit, after merging `codex/batch-slim-3`
  (`a509b681`, which carries #1017): the rows for
  `kernel::settled_body_tests::scenario_{981,983,985,995}_*_is_pinned` are gone from
  `scripts/lib/aarch64-known-defects.py`, the table `scripts/run-aarch64-tests.sh` reads. The
  runner fails on a row whose test no longer exists. `scenario_1006_ramping_prefix_is_pinned` and
  the EQ `bank` row stay. `docs/TARGET_MATRIX.md`'s "AArch64 NaN encodings (#1065)" entry and
  #1065's spec now name only those two. `scenario_982_all_wet_render_is_pinned` was deleted too; it
  never had a row.
- **#1036** landed in the batch and trimmed `response.rs` to the launch rates; it merged cleanly
  with this change's DFT and stride trims.
- **Gate 3** cannot pass as written; the bench console tests in `audit-native` guard the claim
  (see above).
- **Gate 5** needs the full-route CI run; the local figures clear all three thresholds.
- **Reproducing these gates.** `cargo-mutants` 27.1.0 was installed into scratch. Cargo's
  fingerprints do not include the source path, and `git archive` stamps files with the commit
  time, so two archived trees built into one target directory reuse each other's artifacts; the
  first base run of gate 4 did exactly that, and was discarded. Touch the files of each copy, or
  give each tree its own target. The mutation passes were not affected: `cargo-mutants` builds in
  its own directories.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-28. I verified head `78eb0631` against the merged base `a509b681` in
scratch worktrees, one mutation pass at a time. No catch is lost. The recorded width-only catches
are still red per PR, and every gate I ran passes. Gate 3 cannot pass as written, even on the base,
but that is a defect in the spec: the realtime claim is guarded per PR elsewhere. Findings, most
severe first:

1. **Gate 3 is a spec defect, not a FAIL. The realtime claim holds.** I injected
   `std::hint::black_box(Box::new(0_u64))` at the top of `bank_meter_pass`. Under it,
   `the_full_meter_pass_renders_without_an_audited_event` stays green on the base (64 tracks) and
   on the head (8 tracks), and so does its nightly 64-track twin. Injected into
   `bank_sample_peak`, it leaves `the_banked_sample_peak_pass_…` green too. graph-compiler's test
   binary links no `bench-support`, so `audit::snapshot()` never sees an allocation. Under the same
   two injections, run in release on the 64-track console, each pass has one `bench` test that
   aborts with `SIGABRT`: `the_meters_record_carries_each_arms_fold_and_redirect_counters` for
   `bank_meter_pass`, and `the_metered_console_row_prints_its_meters_and_the_validator_pins_them`
   for `bank_sample_peak`. Both are in `audit-native` (`cargo test --locked --release -p audit -p
   bench -p console-workload`). `ci-path-router.py` routes `crates/graph/src/runtime.rs`,
   `tools/bench/src/console.rs` and this diff `full`, and on `full` the verdict requires
   `audit-native` to succeed. So the nightly 64-track run holds pass and commit counts at size, not
   allocations, which is what its doc comment says. Follow-up, outside this issue's paths: give
   graph-compiler `bench-support` as a dev-dependency, or stop calling these tests "audited" for
   allocations.
2. **Medium-low: the per-PR seed offset is tuned to 1005-M5d, and it is the only one that
   works.** With M5d applied, I ran the #1005 `Simd4`/`Simd8` differentials once for each per-PR
   start residue:
   - 0 and 3: green, so M5d is lost at the differential.
   - 1: red for the wrong reason. Its three `Simd8` differentials fail "no restore was refused"
     with no mutant applied.
   - 2: red on `W8 seed 26 block 26 mono false: state`.

   So the choice is overfit to one recorded mutant. It is documented in the code, though, and it is
   the only residue that keeps both the tests' own non-vacuity asserts and the recorded catch.

   For other width-only defects:
   - 1007-M2 (the snap mask without `remaining == 0`, red in gate 1 only at `Simd4`/`Simd8`) is red
     per PR at both widths on seed 2, the first per-PR seed, and also on the deterministic gate 1b.
   - 1005-M5d is also red per PR on the deterministic
     `the_unsafe_ramp_rule_keeps_every_dead_section_after_it`, whatever the seeds.
   - A defect as seed-rare as M5d (one catching seed in the dev range of 40) keeps about a quarter
     of its former per-PR chance, and the nightly full count backs it up.

   That is the trade the spec's "seeds ÷ 4" accepted, and I do not count it as a lost guard. The
   non-vacuity margin at 10 scenarios is thin (2-9 refused restores per arm). A change to the
   scenario generator may need a new offset, but it will fail loudly, not silently.
3. **Low: the compressor module doc misattributes the pins' coverage.** `settled_body_tests` says
   "the grid and the randomized differentials catch every mutant they caught". On the base matrix,
   22 of each deleted pin's mutants are caught by no grid and no differential. `design.rs:131`
   `rate_coefficient` is one: the reference and the production body share it, so no differential
   can see it. `scenario_1006_ramping_prefix_is_pinned`, the bank pin in
   `ramping_prefix_scenario.rs` and the `f64` oracle catch them. The evidence table above is right;
   the comment should name those tests.
4. **Low: paths outside the spec's scope.** The integration commit edits
   `scripts/lib/aarch64-known-defects.py`, `docs/TARGET_MATRIX.md` and #1065's spec. The edits are
   necessary: with the `scenario_981` row put back, `judge-skips debug` fails ("names 0 tests in the
   leg"). Record them as an amendment. `crates/compressor/tests/MUTATIONS.md` still names the
   deleted tests as current reds (981-M1, 982-M1, 982-M3, 995-M1, 1006-M6/M7/M9/M10 and others).
   It needs a one-line note, as #1048 added for `cross_target`.
5. **Low: nightly `--exact` passes silently on a rename.** With `--list`, each of the four
   commands selects exactly one test today. After a rename, a command would run 0 tests and pass.
   `release-budgets` has the same pattern. Hardening it is optional.

What I verified:

- **Gate 1, compressor.** I ran `run-mutants.sh compressor 5 10` (cargo-mutants 27.1.0) on
  `a509b681` and on `78eb0631`. Both passes found 654 mutants, with 457 caught, 152 missed, 43
  unviable and 2 timeouts. The keys are the same, and so is each key's outcome. Every surviving
  test's own catch set is unchanged; only the six deleted tests' sets went to 0. My totals differ
  from the attempt's 513/96 because this base carries #1048, which moved the compressor corpus
  digest to wasm-gates G5. `corpus.rs` mutants are therefore missed by compressor's own tests on
  both sides. The all-wet grid's catch set is exactly the collapsed-body test's 142. Each deleted
  pin's catches (81, 76, 82, 72 and 67) are all held by surviving tests.
- **Amendment 1: the shared inputs.** The pins are absolute digests, so they can see mutants in a
  shared dependency that a differential cannot. I ran `run-mutants.sh effect-runtime 5 10 -p math`
  over effect-runtime `dynamics.rs`, `envelope.rs`, `ramp.rs` and `bank.rs` and math `fast_db.rs`,
  with `--test-package compressor`, before and after. Both passes: 143 mutants, 67 caught, 54
  missed, 20 unviable, 2 timeouts, with the same outcome key by key. On the base, 59 of the 67 are
  caught by a deleted test, and none only by deleted tests.
- **Parametric-eq shard 0's gained mutant is real, not noise.** It is `lib.rs:2715:24` `delete !`
  in `Channel::restore_track` (`if !words_are_identity(cursor)`). On the base, all 54 lib tests
  pass under it. On the head, the three `Simd8` differentials fail deterministically on "a restore
  edit was never accepted". The smaller sample's coverage assert catches it, not a bit comparison,
  so the gain is incidental.
- **Gate 2, per PR, dev profile, head.**
  - 1005-M5d is red on `a_ramping_block_renders_the_batch_head_bits_simd8` (`W8 seed 26`) and on
    `the_unsafe_ramp_rule_keeps_every_dead_section_after_it`.
  - 1006-M1 is red on `randomized_differential_simd4`/`_simd8`,
    `scenario_1006_ramping_prefix_is_pinned` and
    `the_ramping_prefix_renders_the_pinned_bank_scenario`, as recorded.
  - Graph A-5 is red on `post_matrix_all_meters_run_one_full_bank_pass_per_cohort_…` with 8 tracks.
- **Nightly.**
  - actionlint 1.7.7 is clean, and `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
  - `failure-notice` needs `full-size-tests` and reports it.
  - I ran the four commands as the job runs them. All pass: 113 s, 29 s, 1 s and 36 s of tests.
- **AArch64.**
  - `--self-test` passes, and `rows debug` lists the two remaining rows.
  - I built the debug leg's package and feature set on x86, as the script derives it: the 25
    product crates plus `dsp-reference`, `conformance` and `target-smoke`. It has 218 executables
    and lists 1,792 tests. `judge-skips debug` passes, and no row names a deleted test.
- **Test lists.** `cargo test --workspace --all-features -- --list` goes from 2,244 to 2,242: six
  compressor tests deleted, one renamed (`…_repeatable`) and four added (two per PR, two
  `#[ignore]`d). That is exactly the attempt's table.
- **Other gates.**
  - `cargo fmt --check` and `cargo check --workspace --all-targets --all-features` pass.
  - The lint job's exact clippy command, `cargo clippy --locked --workspace --all-targets
    --all-features -- -D warnings`, passes. The attempt's record omits `--all-features`.
  - The affected crates' tests pass with CI's features in dev and in release: 418 passed and 7
    ignored for test-debug-b's five crates, 117 passed and 1 ignored for graph-compiler.
  - `cargo test --release -p console-workload` (the console digests) passes.
  - `check-workspace-policy.sh` passes, #1052's source-scrape lint included.
  - Every other `scripts/check-*.sh` passes except `check-sdk-types.sh`, which needs `npm ci`.
    That includes `check-web-audioworklet.sh`, so the shipped module still matches its pin.
  - Every argument-free `check-*.py` passes under `python3 -B`. The argument-taking ones pass
    `--self-test`, and the listening validators pass through `check-builtins-listening.sh`.
  - Of the lint job's `test-*` mutation companions, `test-workspace-policy.sh`,
    `test-session-policy.sh` and `test-env-vocabulary.sh` pass. I stopped the rest for time: they
    test scripts this change does not touch.
- **No product line moved.** Every hunk under `crates/` is in `tests/` or inside a `#[cfg(test)]`
  module that runs to the end of its file.
- **Not verified.** I did not re-run gate 4. Gate 5's CI figures need a full-route run.
