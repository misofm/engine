# Commit one edge-biased randomized differential per effect and per compiler/host stage

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. **Target shape** for the suite. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.4). Base `a9414c0c`. No ruling needed. Split
per crate before implementation: this body is the template, and the first slice is **one crate**.
Paths starting `../` are relative to the audit's handoff folder.

## Problem

- **Probes found the real bugs.** None of the four recent real bugs (#966, #970, #994, #1015) was
  caught by a test that existed when it was introduced. Each was found by an uncommitted randomized
  probe or differential, by research, or by the implementer.
- **Randomized differentials still catch them.** With each bug re-injected into HEAD
  (`../tools/revert.py`), three of the four are caught by a randomized differential as well as by
  their own reproducers:
  - the #966 probe (`graph-compiler/tests/bank_levels.rs`
    `randomized_consoles_compile_bind_and_render_the_scalar_bits`) catches #966 **and #970**;
  - the compressor's `randomized_differential_{f32,simd4,simd8}` catch #994. Their generator "keeps
    the subnormal knee widths" (`crates/compressor/src/kernel.rs:2137-2160`).
- **They carry many mutant catches.** In the mutation pass:
  - the compressor's three `randomized_differential_*` tests catch 49 % of all caught product mutants
    in 3.9 s;
  - the graph-compiler probe catches 35 % in 14.9 s;
  - parametric-eq's nine ramping-elision differentials catch 35 %.
- **Several crates have no such test.**
  - host-core has **no randomized test at all**.
  - parametric-eq has differentials but no randomized restore path. #1015 needed a restored
    subnormal, and at HEAD only its reproducers catch it.
  - The compressor's `designed_channel_symmetry` (`crates/compressor/src/lib.rs:691-716`), which
    decides mono-collapse eligibility, survives 18 planted bugs. No generator draws
    channel-asymmetric parameter or state words, which is what would expose a wrong answer, the
    #970 class.
  - builtins, source/stem and protocol rely on fixed inputs.
  - Verification scratch fuzzers keep being written and then thrown away: the #962 probe, the
    VERIFY-COMPRESSOR harness, and the dual-mono prototypes in
    `docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch`.

## Outcome (per crate slice)

One committed test module per crate, `tests/randomized.rs`, with:

- **An edge-biased generator.** It draws parameters at the domain edges (0, max, one ulp inside,
  subnormals, `f32::from_bits(1..3)`), exact thresholds and identities, and the interior. It draws
  input from clean signal, hostile words (±0, subnormals, ±inf, NaN payloads, ±MAX, just below and
  above the block limit), silence and tiny levels. It also draws random automation, random block
  partitions, **random restores mid-stream, including subnormal and signed-zero state words**,
  **channel-asymmetric parameter and state words**, bypass toggles and link modes.
- **Oracles.**
  - Scalar against `Simd4` against `Simd8`.
  - Collapsed against forced dual.
  - Elided or fast path against the full path.
  - Restored against continued.
  - Chunked against whole-block.
- **Invariants,** which catch what a differential cannot see when both sides share a defect:
  - finite output for finite legal input;
  - gain within its law's bounds;
  - no allocation in render;
  - state words that `flush` can have written.
- **Seeds.** A fixed seed set sized to at most 5 s in the debug job per PR. `nightly.yml` runs 100×
  the seeds, with the seed printed on failure so it can be replayed.

Order of slices, where the evidence says the gap is widest:
1. host-core console render;
2. parametric-eq with restores, and the compressor's collapse-eligibility inputs;
3. builtins;
4. source/stem;
5. the remaining effects without one (gate-expander, delay, soft-clip, transient-shaper,
   multiband).

The compressor, the limiter (`randomized_scenarios_render_exactly_the_unmodified_kernel`) and
graph-compiler already have one; their slice only adds nightly seeds.

**After** a crate's slice lands, its fixed-point tests and pinned scenarios that the new test
provably dominates are removed under issue 09's gate. That is the payoff.

## Scope (first slice: host-core)

Authorized paths: `crates/host-core/tests/randomized.rs` (new), `.github/workflows/nightly.yml` (one
step), and this issue's spec. No product code. A red found by the new test becomes its own issue with
a reproducer.

## Gates

1. **Catches the historical bugs.** Re-injecting #970 with `../tools/revert.py` turns the host-core
   randomized test red within the per-PR seed set. For the parametric-eq slice the bug is #1015, and
   for the compressor slice a `designed_channel_symmetry -> true` mutation.
2. **Mutation yield.** `../tools/run-mutants.sh host-core 5 10 --features
   control-provider,test-support --shard 0/4 --sharding round-robin`, with and without the new test.
   It catches at least 10 % of the baseline's 84 surviving mutants, or the PR explains why they are
   equivalent. **Depends on issue 13**, so that the `host-core/test-support` feature this uses is
   also enabled in CI.
3. **Budget.** At most about 5 s in the debug job per PR, measured with `--report-time`. The
   graph-compiler probe's 15 s is today's one exception. The nightly run
   covers 100× the seeds.
4. **Replayable.** A failing seed printed by nightly reproduces locally with one command.

## Saving and risk

- **Saving:** none directly. It adds a test. It is what makes issue 09's removals safe and turns the
  one-off verification fuzzers into lasting coverage.
- **Risk:** flaky seeds. Seeds are fixed per PR, so a red is always a real divergence or a generator
  bug.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **The dependency on issue 13 is satisfied.** #1021 is merged; `host-core/test-support` runs in
   `test-debug-a`.
2. **Re-aim the compressor slice (finding F5).**
   - I ran the 18 in-crate `designed_channel_symmetry` survivors against compressor, effect-compiler,
     host-core and graph-compiler.
   - **12 are caught** by tests CI runs (`symmetry_designed_words.rs`, the graph-compiler mono-pool
     tests, host-core `twin_parameter_spans_keep_the_lane_and_a_lone_half_declines_it`).
   - **6 survive everything:** the `|| -> &&` mutants in the ramp and rate-ramp comparisons,
     `lib.rs:704-706` and `:714-716`.
   - So the generator must draw **partial-field asymmetric in-flight ramps**: exactly one of
     `current`, `target`, `step` or `remaining` differing between the channels, mid-ramp.
   - The slice's gate: those 6 mutants go red.
3. **A second eligibility gap, found here.** gate-expander has 8 surviving `bind_homogeneous_bank`
   mutants (bank eligibility). Add it to slice 5's gate.
4. **Gate 2.** Baseline the test set unmutated with the same environment first (F4). At opt-level 1
   two `effect-compiler` tests fail without any mutant; they "catch" every mutant otherwise.
