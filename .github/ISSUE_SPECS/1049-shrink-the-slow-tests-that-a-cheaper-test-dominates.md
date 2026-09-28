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
