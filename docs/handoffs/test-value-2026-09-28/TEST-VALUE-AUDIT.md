# Test value audit, 2026-09-28

Research only. Base `origin/main` `a9414c0c` (PR #1016 merged). Nothing in the product, the tests
or CI was changed. The issue drafts under [`issues/`](issues/) are proposals; none is a GitHub
issue.

The owner's question was: "I have a suspicion that a lot of the tests we run don't really
contribute a whole lot of value. I could be wrong but I want to verify this."

## 1. The answer in plain language

**Partly right.** The waste is real, but it sits in a few places that are easy to name. Most of the
2,552 Rust tests are not the problem.

- **Most tests are cheap duplicates, not useless ones.** In the four crates put through mutation
  testing, the whole suite catches 78-89 % of the planted bugs. Each caught bug is usually caught by
  5 to 20 or more tests. A set of 16-54 tests, out of 100-229 per crate, catches everything the full
  suite catches. Only about 1 test in 8 (68 of 571) catches something no other test catches. So most
  tests repeat each other, but they cost almost nothing to run: 2,321 tests run in 23 seconds in
  total. Deleting them would save attention and review time, not CI time.
- **The time goes to a few things.** 121 tests (5 %) take 90 % of test execution time. Beyond them,
  CI time goes to building, not to testing: about 6 minutes of runner time per pull request is spent
  linking release test binaries with the shipping optimiser (fat LTO). It also goes to script gates
  that test themselves (about 150 s in the lint job), to three copies of the same metadata build
  (about 140 s), and to scalar-Wasm legs for an artifact that never ships (about 117 s).
- **Most CI reds catch nothing.** From 1 to 28 September, 132 jobs went red:
  - 78 were stale pins, where a number recorded in a file had to be updated after an intended
    change;
  - 31 were hygiene;
  - 19 were broken tests;
  - 3 were real defects, and those 3 reds were only 2 distinct defects. Both were caught by script
    gates, not by a Rust test.
- **Real bugs were found by probes and differentials, not by the fixed-point suite.** None of the
  four recent real bugs (#966, #970, #994, #1015) was caught by a test that existed when it was
  introduced. Two were found by randomized probes, one by a research prototype and one by the
  implementer. With each bug put back into today's code, the tests that go red are the reproducers
  written after the fix, plus, for three of the four, a *randomized differential*: a test that
  renders random sessions two ways and demands identical bits. The #966 probe also catches #970, a
  bug it was never written for.
- **There are gaps as well as waste.**
  - No CI job builds or runs anything on AArch64, yet fans will open sessions in mobile apps. Known
    AArch64 defects are on record, and gate G1 is red there.
  - The 4-lane path that browsers and phones run is only partly exercised by the native test run.
  - Some tests never execute in CI because a feature flag is missing.
  - Between 1 in 10 and 1 in 5 planted bugs survive every test.

**How much, in numbers.** About 18 % of per-PR CI runner time (≈ 570 s of 3,120 s) adds no
discrimination and can go with no ruling at all, without losing a recorded catch. With the owner
rulings in §12 it rises to about a third (≈ 1,050 s). The pin rulings (R2, R3) would also remove
the source of about 60 % of all red CI jobs and of 50-125 pin-only commits a month.

By count, 94 tests do not test the claim in their name: they restate a constant, assert nothing, or
cover removed surface. With the 30 source-text scrapes and about 25 re-tests of shared test helpers,
that is about 150 tests (6 %). 869 tests (34 %) sit in near-duplicate families, where a table or one
randomized test would carry the same claim.

**What to do instead of mass deletion.** Keep every reproducer of a real bug and every realtime,
wire/ABI and DSP-reference check. Give each effect and each compiler stage one committed randomized
differential with a generator biased to domain edges. Then remove the fixed-point clusters and
pinned scenarios that such a differential provably dominates, as shown by the mutation equivalence
gate in section 10. Fix the pins, the duplicate builds and the gaps first: that is where the time,
the red builds and the real risk are.

## 2. What was measured

| measurement | how | where the raw data is |
|---|---|---|
| inventory | every `#[test]` fn parsed from source (2,552; 41 `#[ignore]`); test LOC = `tests/` + `#[cfg(test)]` code | [`data/test-classes.tsv`](data/test-classes.tsv) |
| local runtime | every CI test binary, built with `CARGO_INCREMENTAL=0` outside the lock, run once under `flock` on the shared timing lock with libtest `--report-time` (per-test `exec_time`); 32-core host, load 3.5-11.7 | §3.2 |
| CI time | job and step timings of PR #1016's run 36382065641 and main push 36382785722; per-binary `finished in` lines from the job logs; a 30-day sum over all 611 `qualification.yml` runs | §3.3 |
| claim class | every test classified by the claim it defends (below), by two helpers reading names and bodies, reusing the 2026-09-04 ledgers where tests are unchanged; spot-checked | [`data/test-classes.tsv`](data/test-classes.tsv) |
| discrimination | `cargo mutants` 27.1.0 on `compressor` (all 654 mutants), `graph-compiler` (all 412), `host-core` (every 4th of 1,733) and `parametric-eq` (every 4th of 1,401), plus a targeted pass over the graph meter pass; own-crate tests, `--no-fail-fast`, so every failing test is recorded per mutant | §4.1 |
| real bugs | the recorded revert of each of #966, #970, #994, #1015 re-injected into a scratch copy of HEAD; the CI debug shards (plus `console-workload`) run with `--no-fail-fast` | §4.2 |
| CI red history | all 132 failed jobs of 73 failed `qualification.yml` runs, 2026-09-01..28, fetched with `gh api` and classified by what fixed them | [`data/ci-red-jobs.tsv`](data/ci-red-jobs.tsv) |
| script gates | the earlier helper's per-gate table re-verified against source, CI logs and local runs | [`data/script-gates-verify.md`](data/script-gates-verify.md) |

Claim classes: **A** bit-identity/differential, **R** realtime safety, **D** DSP correctness against a
reference, **P** protocol/ABI/wire contract, **S** semantic behaviour (compiler, session, host), **B**
resource and byte pins, **Y** policy (source-text scans), **X** prose pins, **T** tooling and
benchmark bookkeeping.

Limits, stated once:

- Mutants are synthetic. A test that catches no mutant is a *candidate*, never proven useless.
- Mutation used each crate's own tests only. A test that lives in one crate but guards another
  crate's code, such as the graph-compiler meter tests guarding `graph`'s render path, is
  under-credited. §4.1 says where this matters.
- CI red history sees only what reached CI. It cannot see bugs a test caught on a developer's
  machine before a push.

## 3. Inventory and cost

### 3.1 The Rust tests by claim

| class | tests | share | typical cost | discrimination evidence |
|---|---:|---:|---|---|
| S semantic behaviour | 1,004 | 39 % | cheap | most mutant catches; heavily overlapping |
| P protocol / ABI / wire | 488 | 19 % | cheap | unique catches in host-core and compressor (refusal and restore paths) |
| A bit-identity / differential | 387 | 15 % | **most of the slow tests** | caught 3 of the 4 real bugs (randomized ones) |
| D DSP vs reference | 277 | 11 % | medium | few unique mutant catches; guards what no differential can (both sides wrong) |
| T tooling / benchmark bookkeeping | 189 | 7 % | cheap, but release builds | none on product code |
| R realtime safety | 116 | 5 % | some slow | 7 false reds from process-global counters; no real catch in window |
| B byte / digest pins | 67 | 3 % | cheap | high mutant counts, but mostly of the fixture generator itself (§4.1); 78 stale-pin reds |
| Y+X source-text and prose pins | 24 | 1 % | cheap | none |

Flags that cut across the classes: 154 tests assert a pin; 30 scrape source or doc text; 52 are
tautologies; 15 assert nothing that can fail; 36 test removed or out-of-scope surface. 869 tests
(34 %) sit in 132 near-duplicate families, where 4 or more tests prove one claim.

Scale of the suite: the `#[test]` attribute count grew from 1,553 to 2,553 since the 2026-09-04
audit's base (+64 %; `git grep` at `0e248bb0` and HEAD; one attribute is not on a function, hence
2,552 tests). There are **214,371 lines of test code against 196,732 of product code.**

The per-crate table is in [`data/inventory.md`](data/inventory.md) and the per-test rows are in
[`data/test-classes.tsv`](data/test-classes.tsv).

### 3.2 Where local test time goes

Timing was re-measured under the lock. It matches the earlier helper's run within 3 %.

| CI shard | binaries | tests | wall (binaries in sequence) | sum of per-test time |
|---|---:|---:|---:|---:|
| test-debug-a (debug) | 110 | 1,518 | 245 s | 451 s |
| test-debug-b (debug, DSP crates) | 146 | 806 | 221 s | 380 s |
| test-release (lane, math, wasm-gates) | 28 | 113 | 19 s | 23 s |
| audit-native unit tests (release) | 9 | 176 | 15 s | 71 s |

- **A few tests take most of the time.** 121 tests of 1 s or more take 833 s of the 925 s total.
  The 2,321 tests under 0.1 s take 23 s. The top five are:
  - graph-compiler's two 65,537-track scale tests, 61 s and 55 s;
  - `builtins-compiler` `phase_two_allocator_layouts_match_the_checked_resource_report`, 37 s;
  - `session` `ten_million_deterministic_f32_patterns_round_trip`, 31 s;
  - `graph` `cohort_chain_merging_preserves_dataflow_on_random_graphs`, 28 s.
- **By crate:** graph-compiler 267 s serial, parametric-eq 119, builtins 59, audit 52,
  builtins-compiler 49, compressor 43. Everything else is under 40 s.

### 3.3 Where CI time goes

**One PR run (PR #1016).** It used 3,123 runner-seconds over 17 jobs. The slowest jobs, which
decide how long the PR waits, are:
- audit-native, 463 s;
- test-release, 446 s;
- test-debug-a, 408 s.

**Over 30 days.** There were 611 `qualification.yml` runs, 535 of them on the full route, using
320 runner-hours. Nightly used 26 h and fuzz 3.5 h.

| job | runner h / 30 d | what dominates, from the PR #1016 logs |
|---|---:|---|
| release audit, trace, fixture gates | 46.0 | release build 99 s; **the unit-test step is 241 s, of which about 210 s is fat-LTO compile and 30 s is running**; capi rebuild 36 s |
| wasm guests | 37.2 | wasmtime runner build 118 s; scalar-Wasm legs ≈ 117 s (§7) |
| lint (fmt, clippy, doc, policy) | 36.3 | fmt+clippy+doc 35 s; **script gates ≈ 230 s**, of which three steps whose cost is mostly self-test suites take 148 s |
| test-debug-a | 33.8 | 53 s compile + 323 s run; 6 binaries = 232 s (scale 68, graph-compiler lib 46, bank_levels 46, allocation_tracker 40, session lib 34, graph lib 34) |
| test-release | 33.5 | **182 s compile, of which ≈ 165 s is fat-LTO linking of about 30 test binaries**; about 20 s of tests; M1 53 s + F1 124 s on math-closure PRs |
| test-debug-b | 26.4 | 26 s compile + 189 s run |
| artifact gates | 25.0 | native fat-LTO `parameter-metadata --check` 75 s (tautological in CI); native witness build for the resource rows ≈ 60 s; callgraph ≈ 45 s |
| browser ×3 | 20.7 | mostly browser install |
| SDK | 17.0 | third fat-LTO `parameter-metadata` build ≈ 72 s; sdk-deletions self-test 20 s |
| shipped artifact | 16.0 | the build |
| cross-target | 13.1 | cargo checks, effect package/descriptor corpora |

Rust tests make up about 120 of the 320 runner-hours. **In the two release-mode unit-test steps,
compiling costs about seven times as much as running** (≈ 390 s of compile per PR against ≈ 50 s of
tests, not counting the M1/F1 exhaustive sweeps that run only on math-closure PRs).

## 4. Does each test discriminate?

### 4.1 Mutation pass

Each planted change ("mutant") is a small code edit, such as `>` → `>=` or `&&` → `||`, or a function
body replaced by a default. Each mutant is built and every test of the crate is run.

| crate | mutants tested | caught | survived | score | tests | tests with ≥1 catch | tests with a unique catch | tests that catch everything the suite catches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| compressor, product code¹ | 525 | 390 | 93 | 80.7 % | 100 | 92 | 11 | 16 |
| graph-compiler, product code² | 335 | 217 (+3 timeouts) | 49 | 81.8 % | 113 | 107 | 9 | 17 |
| host-core (1 in 4 sample) | 434 | 284 (+5) | 84 | 77.5 % | 229 | 213 | 34 | 54 |
| parametric-eq (1 in 4 sample), product code¹ | 323 | 246 (+12) | 31 | 89.3 % | 129 | 121 | 14 | 21 |

¹ `src/corpus.rs` excluded. It is the frozen cross-target corpus. All 58 unique catches of the
compressor's corpus pin, and all 16 of parametric-eq's, are mutants *of the corpus itself*.

² `src/bin/graph_fixture.rs` excluded, for the same reason: 34 of graph-compiler's 37 unique catches by
`checked_in_fixtures_are_the_generated_bytes` are mutants of the fixture generator. **A digest pin's
large mutant count mostly measures the pin guarding its own fixture code, not the product.**

How often each caught mutant is caught:

| crate | by 1 test | by 2-4 | by 5-19 | by 20+ |
|---|---:|---:|---:|---:|
| compressor | 40 | 35 | 206 | 109 |
| graph-compiler | 14 | 78 | 49 | 76 |
| host-core | 75 | 88 | 96 | 25 |
| parametric-eq | 17 | 37 | 112 | 80 |

What this shows:

- **Redundancy is the norm.** Measured by these crates' own mutants, many catching tests are *fully
  dominated*, meaning one other single test catches every mutant they catch: 40 of the compressor's
  93, 80 of graph-compiler's 107, 135 of host-core's 213 and 64 of parametric-eq's 121.
- **Some slow tests are dominated.**
  - Compressor `the_all_wet_arm_is_the_base_body_on_all_wet_tables`, 23.4 s, the crate's slowest
    test, has exactly the same 142-mutant catch set as
    `the_collapsed_settled_body_is_the_base_body_on_the_three_parameter_sets`, 4.7 s.
  - parametric-eq:
    - `the_two_channels_are_judged_together` (24.6 s) is dominated by
      `an_elided_cascade_is_the_full_cascade_bit_for_bit` (5.0 s).
    - `forty_eight_frozen_million_sample_sequences_remain_valid_without_recovery` (13.2 s) is
      dominated by a 0.4 s test.
    - The `Simd8` twins of the ramping-elision randomized differentials (12.5, 12.0 and 11.7 s) are
      dominated by their `Simd4` twins. However, `MUTATIONS.md` records a W8-only catch (1005-M5d),
      so trim their seeds rather than drop them.
  - graph-compiler's meter-pass tests (22.6 s, 21.2 s, 13.1 s, 12.7 s) are dominated for
    graph-compiler's mutants. They mostly exercise `graph`'s render path, so a targeted pass over
    `graph`'s bank meter pass was run against both crates' tests (29 mutants, 22 caught, 4 survived).
    - These graph-compiler tests are that code's **main guards**: `graph`'s own tests catch only 1
      of the 22.
    - Among them, `an_observer_failing_mid_bank_leaves_every_later_meter_as_the_declined_arm_does`
      (2.6 s) catches every mutant that the 22.6, 13.1, 12.7, 6.3 and 5.9 s tests catch.
    - So they are candidates to **shrink** (fewer tracks and blocks), not to delete. The 22.6 s test
      asserts an allocation claim that no mutant exercises.
- **The scale tests catch nothing in graph-compiler.** The two 65,537-track tests (116 s locally,
  68 s in CI) were run against every mutant the rest of the suite let through (80). They caught
  none: 79 survived and 1 timed out, exactly as it had without them.
- **Unique catches come mostly from fixed-input contract tests.**
  - In host-core, all 34 unique catchers are fixed-input tests, across the S, P, A, D and R classes.
    host-core has no randomized test at all.
  - In the compressor, the unique catchers are restore, automation-validation and malformed-block
    tests, plus one pinned ramping scenario (8 product mutants). That scenario is the one pin with
    real product discrimination here.
- **Randomized differentials carry a large share of catches.**
  - In the compressor, the 18 seeded-generator tests catch 290 of 390 caught mutants (74 %). The
    three `randomized_differential_*` tests alone take 3.9 s.
  - In graph-compiler, the #966 probe `randomized_consoles_compile_bind_and_render_the_scalar_bits`
    catches 77 of 217 (35 %) in 14.9 s.
  - In parametric-eq, the seeded differentials catch 167 of 246 (68 %).
  - But fixed-input tests are the *only* catchers of 100 compressor mutants, 113 graph-compiler
    mutants and 79 parametric-eq mutants. **Randomized tests do not replace the contract tests.**
- **Survivors show gaps.**
  - graph-compiler: the cap comparisons in `compile_graph` (`compile.rs:389`, `:408-409`,
    `:770-773`, where `>` → `>=` survives) and `effect_bank_resource` (`banks.rs:562-565`). No
    graph-compiler test checks those exact boundaries. They may be checked downstream; that is not
    measured.
  - Compressor: most survivors are in the silent fast path (`lib.rs:505-603`), where a mutant changes
    speed but not bits. These are equivalent for output and can only be seen by counters.

Scripts and full output: [`tools/`](tools/) and §10.

### 4.2 The four real bugs, re-injected into today's code

| bug | how it was found (history, verified in the specs) | tests red at HEAD with the bug put back | other tests that stayed green |
|---|---|---|---|
| #966 bank spans dependency levels | uncommitted randomized probe in the #962 review (`.github/ISSUE_SPECS/962-*.md:385-393`) | 9, all in `graph-compiler/tests/bank_levels.rs`: 8 fixed reproducers plus the randomized probe | all ~1,500 other shard-a tests and `console-workload` |
| #970 collapse armed on a later chain | dual-mono research with a prototype probe (never committed) | 8: the 7 `host-core/tests/collapse_arming.rs` reproducers, plus **the #966 randomized probe** | everything else |
| #994 knee reciprocal overflow | scratch differential fuzzer with an edge-biased generator (`docs/handoffs/effects-2026-09-27/VERIFY-COMPRESSOR.md:35-60`) | 10: 7 reproducers in effect-runtime, compressor and multiband, plus the compressor's **three committed `randomized_differential_*` tests**. Their generator keeps the subnormal knee widths, committed with #981 in the same verification cycle | everything else |
| #1015 restored subnormal EQ state | the #1005 implementer (`1015 spec:7`) | 3: only its `stationary_subnormal` reproducers in `parametric-eq/src/lib.rs`. **No randomized test catches it**, because no generator restores subnormal integrator state | everything else, including every EQ differential |

**No test that existed when the bug was introduced caught any of the four.** Each one is now caught
by its own reproducers. #966, #970 and #994 are also caught by a randomized differential, and in
#970's case that differential was written for a different bug. #1015 is caught by nothing else: its
trigger is a *restored* subnormal state, and no generator restores state.

### 4.3 CI red history, 2026-09-01..28

Source: [`data/ci-red-jobs.tsv`](data/ci-red-jobs.tsv). All 132 failed job logs were fetched with
`gh api`; none was refused.

| category | red jobs | notes |
|---|---:|---|
| stale pin, re-pinned | 78 | shipped-artifact sha256 20; browser lineage `wasmSha256` 21; `expected.json` resource rows 6; builtins benchmark manifest users 5; graph audit record hash 4; builtins fixture `resources.jsonl`/`MANIFEST.tsv` 4; capi byte totals 4; `layout_total_bytes` 2; conformance corpus hashes 2; five one-offs; 5 expectation migrations |
| hygiene | 31 | env-vocabulary doc sync 8; SDK retired-word bans 5, each firing on *new, legitimate* code; rustdoc 4; clippy/fmt 3; policy allow-lists |
| broken test or gate | 19 | **7 were allocation or owner counters not scoped to one thread, 4 of them on `main`**; flaky browser timing 8; gate-script bugs 3 |
| infra | 1 | |
| real defect | 3 jobs, 2 defects | #920 `route_reduce::<f32x4>` scalarised (the callgraph vector ratchet); SDK `.d.ts` out of step with the runtime (`check-sdk-generated.sh`) |

**No Rust test caught a real defect in CI in this window.** The 16 red `workspace debug tests` jobs
were 9 pins and 7 broken tests. No pin exposed an unintended change: every pin fix changed only pins,
tests or docs. The pins moved often:
- the artifact sha256 took **98 values**;
- the browser lineage moved 86 times;
- the `expected.json` rows moved 24 times;
- the capi totals moved up to 18 times.

### 4.4 The randomized-differential hypothesis

*Hypothesis: the engine's real bug catching comes from randomized differentials against an oracle,
and many fixed-point tests and pins catch little.*

**Supported, with one limit.**

*For:*
- Three of the four real bugs are caught at HEAD by a randomized differential. #966's probe
  generalised to #970.
- No fixed-point test that existed at the time caught any of the four.
- The randomized tests catch 74 % of what the compressor suite catches, and 68 % in parametric-eq.
- Pins produced 78 reds and no catch.
- The fourth bug, #1015, shows the gap rather than refuting this: only its reproducers catch it,
  because no generator restores state. Restored state is an input a randomized suite should draw
  (issue 14).

*Against, and why this is the limit:*
- A differential compares two implementations, so it cannot see a defect that both share or a
  defect in the oracle. #994 was only visible because the optimised body and the base body differed
  once the reciprocal overflowed. An invariant check (finite output, bounded gain) is what makes such
  a harness see it.
- Refusal, restore, resource-accounting and protocol semantics are caught *only* by fixed-input
  contract tests:
  - all 34 of host-core's unique catchers;
  - 100 compressor mutants and 79 parametric-eq mutants that no randomized test catches.

**Target shape.**
1. **Keep, permanently:**
   - every real-bug reproducer (list: [`data/bug-reproducers.md`](data/bug-reproducers.md));
   - wire and ABI vectors;
   - realtime audits;
   - one owner per cross-target digest corpus;
   - DSP reference checks;
   - contract tests that hold a unique catch.
2. **Add, per effect and per compiler/host stage:** one committed randomized differential and
   invariant suite with an edge-biased generator. It should cover:
   - domain edges and subnormals;
   - hostile input words;
   - random automation, partitions and restores;
   - collapsed against forced-dual rendering;
   - Scalar, Simd4 and Simd8 lane widths.

   Run a fixed seed set per PR, about 2-15 s debug per crate, and 100× the seeds nightly. Most of the
   work exists already as scratch fuzzers from verifications: the #962 probe, the
   VERIFY-COMPRESSOR harness and the dual-mono prototypes. They should be committed rather than
   discarded. The compressor, the limiter (`randomized_scenarios_render_exactly_the_unmodified_kernel`)
   and graph-compiler (`bank_levels` probe) already have one. host-core, parametric-eq's restore
   path, builtins and the source/stem path do not.
3. **Then remove** fixed scenario pins and fixed-point near-duplicates that such a suite dominates,
   under the mutation equivalence gate (§10).

**Cost and yield.**
- Cost: roughly 300-600 lines and 2-15 s of debug time per suite.
- Yield on this evidence: 3 of 4 real bugs, and 35-74 % of synthetic mutants per suite.
- Blind spot: protocol and accounting semantics.

## 5. Ranked cut, merge and shrink candidates

Ranked by saving and certainty against risk. "Guarded elsewhere by" names the surviving protection.
Savings are per full-route PR unless stated.

| # | candidate | evidence | claim it guards → guarded elsewhere by | local saving | CI saving | risk | issue |
|---:|---|---|---|---|---|---|---|
| 1 | **Replace exact resource-byte and record-hash pins with ceilings and live reports**: `hosts/host-web/tests/browser-v1/expected.json` resource rows plus the native witness build; `layout_total_bytes`/`layout_entries` in `qualification.yml:671` plus the Rust copies (`tools/audit/src/source_duration.rs:325-337`); the graph audit record hash (`trace-builtins-graph-audit.sh:62`); the capi literal totals and 2,100-line mirror (`crates/capi/tests/resource_lifecycle.rs:605-2732`) | 78 pin reds, 0 catches; rows moved 24×, capi 18×, graph hash 5-8×; the audit exits non-zero itself on duration dependence | memory stays within budget → `tests.rs:650,984,1079`, `boot_transient_budget.rs`, `check-web-boot-budget.mjs`, `resource_lifecycle.rs:2785`, `host-core/tests/prepare.rs:161` | ~0 | ≈ 60 s runner (native fat-LTO witness) + about 50 re-pin edits a month | growth of a few hundred bytes under the ceiling goes unseen (owner ruling R2) | [01](issues/01-replace-exact-resource-pins-with-ceilings.md) |
| 2 | **Stop self-testing script gates on every PR; make env-vocabulary one pass** | lint script steps 148 s; the gates themselves are 15.5 s + 0.35 s; the env gate spawns 12,128 greps (one `git grep` does it in 0.7 s); `test-console-benchmark.sh` runs 4,187 `jq`; 9 env reds were hygiene | the gate scripts' own plumbing → the gates still run; self-tests run when the gate script changes (path route) or nightly | ~150 s | ≈ 165 s runner in lint + 20 s in sdk | a broken gate script lands and is caught at its next edit or nightly | [02](issues/02-run-gate-self-tests-only-when-the-gate-changes.md) |
| 3 | **Build `parameter-metadata` once; compare downstream** | three fat-LTO builds per PR (61.6 s, 75.0 s, 72.3 s); the artifact-gates `--check` compares the generator's output with itself at the same commit | codegen drift → `sdk/assets` byte compare against the downloaded, pin-verified artifact | 0 | ≈ 140 s runner | none | [03](issues/03-build-parameter-metadata-once-per-pr.md) |
| 4 | **Move the 65,537-track scale tests to nightly; trim `allocation_tracker` to [1, 4] tracks and the ten-million f32 loop** | 0 catches: run against the 80 graph-compiler mutants the rest of the suite let through, they caught none; the 2026-09-04 ledger showed layout classes are count-invariant | no hidden track cap (AGENTS.md) → nightly keeps the 65,537 rows; `bench graph_validate_65537_tracks` | ≈ 130 s serial | ≈ 130 s off test-debug-a (408 s → ≈ 275 s) | a u16 overflow above 65,536 is caught nightly, not per PR | [04](issues/04-move-65537-track-scale-tests-to-nightly.md) |
| 5 | **Drop the scalar-Wasm legs together with the "every other target" arms** | about 117 s of legs; the shipped host refuses a non-simd128 module (`host.js:61-64`); the atomics check already runs on the shipped module | no atomics / scalar path correctness → `check-web-audioworklet.sh:309-341` on the shipped artifact | 0 | ≈ 117 s runner | none, if the arms go too (owner ruling R5) | [05](issues/05-retire-scalar-wasm-legs-and-arms.md) |
| 6 | **Delete tests that do not test their named claim, test only test tooling, or cover removed surface** (≈ 150 tests: 52 tautologies, 15 no-assert, 36 out of scope, bench-support re-tests, the ignored print-only benchmarks) | the classifier rows, each spot-checked; where measured, none holds a unique mutant catch | the named claim holds by construction; what they catch as a side effect is caught by kept tests | ≈ 1 s | ≈ 0 s; fewer binaries to link | nil without rulings; the out-of-scope rows need R6/R7 | [06](issues/06-delete-tests-that-cannot-fail.md) |
| 7 | **Rewrite or delete the 30 source-text and prose scrapes; add the lint** | the rubric's banned pattern; new ones keep being added (`graph/src/runtime.rs:7770` pins rustfmt indentation) | the behaviour they paraphrase → render-and-compare tests; one (`capi/src/ffi.rs:2516`) has no other guard and is rewritten, not deleted | ≈ 0 | ≈ 0 | low | [07](issues/07-replace-source-text-scrapes-and-lint-them.md) |
| 8 | **One owner per cross-target digest corpus** | 10 per-crate pin compares repeat G5; the G5 Rust test repeats `run-wasm-gates.sh`'s native leg | class A across widths and targets → `run-wasm-gates.sh` (native + wasm legs) | ≈ 5 s | small; about 10 fewer re-pin sites | none, if every width stays compared (gate in the issue) | [08](issues/08-one-owner-per-cross-target-digest-corpus.md) |
| 9 | **Shrink slow tests that a cheaper test dominates** | compressor `the_all_wet_arm…` (23.4 s) has the same catch set as a 4.7 s test; EQ `the_two_channels_are_judged_together` (24.6 s) is dominated by a 5.0 s test; the 48×1M EQ sequences (13.2 s) are dominated by a 0.4 s test; one 2.6 s graph-compiler meter test catches everything six others (82 s) catch; five compressor slice pins are red only where gate 1 is | each claim stays, at a smaller size or with the full seeds nightly | ≈ 170 s serial | ≈ 40-60 s off debug-b, ≈ 30 s off debug-a | low with the mutation equivalence gate; W8-only recorded catches keep both widths | [09](issues/09-collapse-dominated-slow-differentials.md) |
| 10 | **Delete unrun one-shot scripts and retired-feature gate rules** | 18 top-level scripts (3,958 lines) that no workflow reaches; retired FLAC/TOML/softfma/delta-bank/pre-boot-v1 bans; 20 env rows kept alive only by input-symmetry tooling | nothing live | 0 | ≈ 0; 4 SDK false reds a month | none | [10](issues/10-delete-unrun-scripts-and-retired-feature-gates.md) |
| 11 | **Test binaries in release without fat LTO** (ruling) | ≈ 165 s linking in test-release, ≈ 210 s in audit-native; ≈ 50 s of actual tests | the shipped-profile codegen → the shipped artifacts keep their own gates (release audits run the real binaries; the Wasm artifact gates) | 0 | ≈ 300 s runner; critical path about −3 min | an LTO-only miscompile in a test binary (owner ruling R4) | [11](issues/11-ruling-release-test-binaries-without-fat-lto.md) |
| 12 | **Merge near-duplicate families into table-driven tests** (132 families, 869 members; 10 host families where 1-3 representatives suffice) | the families lists | same claims | ≈ 0 | ≈ 0 | low; maintenance only | folded into [06](issues/06-delete-tests-that-cannot-fail.md) where members are subsets, otherwise not proposed now |
| 13 | Small CI items: cache a capi-only target for `check-capi-abi.sh` (36 s); drop the M3 "FMA" leg (the default x86 build already has `+fma`, 9 s + a separate target dir); drop the lint duplicate of `test-web-audioworklet.mjs` and the second `check-sdk-generated.sh` | CI logs; `.cargo/config.toml` | same claims | 0 | ≈ 50 s runner | none | [03](issues/03-build-parameter-metadata-once-per-pr.md) |

Runner time and wall time differ, so the columns do not add up exactly. **Without any ruling
(items 2-4, 6-10 and 13): ≈ 570 s of runner time per PR, and ≈ 160 s off test-debug-a. With rulings
R2, R4 and R5 (items 1, 5 and 11): ≈ 1,050 s**, and each of the three longest jobs (audit-native,
test-release, test-debug-a) drops by about 2-3 minutes, so a PR waits about 3 minutes less.

## 6. Gaps: under-tested claims

These matter as much as the waste, especially after the owner's correction: fans open sessions in
web and mobile apps, and the mobile apps embed the engine natively on AArch64 with NEON.

1. **No AArch64 anywhere in CI.**
   - `scripts/check-cross-targets.sh:3-6` removed the Android and iOS rows under owner ruling #378
     (2026-09-04, "unsupported, no claim").
   - `docs/TARGET_MATRIX.md:86-104` records open AArch64 defects:
     - LANE-3: `max`/`min` fold to `fmaxnm`/`fminnm`, so **gate G1 is red on AArch64**;
     - the SVF `flush()` calls `memset` per frame on Darwin, a realtime violation;
     - tests assert `Backend::current() == Simd8`, so they would fail there.
   - The repo is public, so GitHub's arm64 Linux runners are free. An AArch64 leg would run the lane
     G-gates, the class-A digest corpora and a debug test subset, and its first expected red is
     LANE-3. Issue [12](issues/12-add-an-aarch64-neon-ci-leg.md); owner ruling R1.
2. **The 4-lane production path is only partly exercised natively.**
   - Browsers (`simd128`) and phones (NEON) both run 4-lane banks. The x86-64-v3 CI host is `Simd8`,
     and its effect factories decline most 4-lane banks.
   - As a result, the `Simd4` bank-count pins in `graph-compiler/tests/bank_levels.rs` never run
     (`docs/handoffs/bug-966-2026-09-27/README.md:13-19`). `gate-expander/tests/identity.rs:223`
     returns unless `Simd4`, so it never runs either.
   - The `wasm-pins-harness.patch` from #966 shows how to run them under wasmtime. Folded into
     issue 12.
3. **Tests that never execute in CI because a feature is off.**
   - `builtins/test-support` and `parametric-eq/test-support` are not enabled in test-debug-b. The
     counter halves of `builtins/tests/meter.rs:924,1403,1511` and the counter legs of
     `parametric-eq/tests/bank.rs` do not run, so mutations K-2, S-1L/R and A-2 would pass CI.
   - `host-web/test-support` and `host-core/test-support` are not enabled in test-debug-a. This
     includes host-web's only zero-allocation check on prepared-EQ admission and render,
     `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`
     (`hosts/host-web/src/tests.rs:3535`), and `host-core/tests/observation_demand.rs:524`.
   - Issue [13](issues/13-run-the-tests-ci-currently-compiles-out.md).
4. **11-22 % of planted bugs survive** in each crate measured (§4.1). Of the effects, only the
   compressor and the limiter have a committed randomized differential, and host-core has none
   (§4.4, issue [14](issues/14-commit-a-randomized-differential-per-effect-and-stage.md)).
5. **Realtime counters raise false alarms.** 7 red jobs, 4 of them on `main`, came from allocation
   or owner counters that also counted other threads. The fixes were per test. A single
   thread-scoped counter helper, used everywhere, would stop the class. Noted in issue 13.

## 7. Script gates: the earlier helper's findings, verified or corrected

Full evidence: [`data/script-gates-verify.md`](data/script-gates-verify.md).

| earlier headline | status | corrected figure |
|---|---|---|
| Lint spends ~150 s of 294 s in env vocabulary (71), benchmark validators (47), conformance (30) | **verified** (148 s; 204 s on the main push) | in each, the **self-test suite** is the cost: env gate 15.5 s vs suite 53.7 s; conformance gate 0.35 s vs suite 29.3 s, with 19 s of it re-running the suite recursively twice; `test-console-benchmark.sh` spawns 4,187 `jq` processes, not "~130" |
| C ABI self-test rebuilds `capi` in release 8 times | **corrected** | exactly **one** rebuild (36.4 s of the 43.6 s step): `-p capi` alone resolves different features than the audit build. The self-test costs 1.2 s. Setting `SKIP_BUILD=1` would point the ABI check at an audit-feature build, so the fix is a cached capi-only target |
| Pins with no behaviour: research headings, preregistration prose, EQ test literals, "nudge", sha256 of Python sources, second artifact sha | **verified**, all six | `REAL_WASM_SHA256` is read only under `--real-wasm-receiver`, which nothing runs, and it moved in 11 commits; "nudge" is already refused as a wire key by `check-parameter-metadata-v1.py:733-735` |
| Gates for retired features | **verified**, with file:line | the sdk-deletions retired-word bans fired 4× in 73 runs, each time on new, legitimate code |
| ≥ 10 of 40 recent reds were stale pins | **verified, corrected upward** | 21 of 40 runs contain a pin red; 12 of 40 were red *only* because of pins. The graph audit record hash (`trace-builtins-graph-audit.sh:62`) is a pin the helper missed |
| 27 unrun scripts, 6,167 lines | **corrected** | **26 files, 6,119 lines** (a jq `include` reaches one): 8 operator tools (1,479), the live `run-console-benchmark.sh` (682) outside `operator/`, and 18 one-shot issue files (3,958). `scripts/operator/README.md` claims every top-level script is reachable; nothing enforces it |
| Scalar Wasm tree built though only simd128 ships | **verified** | ≈ 117 s of legs. The shipped host rejects a non-simd128 module. The "every other target" arms (`lane/src/backend.rs:59-67`, `wide_impl.rs:290-321`, `graph/src/runtime.rs:328-335`, `soft-clip/src/lib.rs:903-911`, `target-smoke`, `host-web/src/lib.rs:7260-7265`) are the only code those legs cover: rule on both together |
| Must stay | **challenged** | real catches in history: the lane policy, realtime `unsafe` policy, host-core policy, effect-contract completeness, the callgraph vector ratchet, the command-vocabulary gates and the native PCM runner dependency rule. Only noise: the builtins `unsafe` substring scan (it duplicates `unsafe_code = "deny"`), the sdk-deletions word bans, the env row count, the graph audit hash, the resource rows and the layout dict |

The browser-side helper report (`fork-web.md`) is folded into items 1, 3 and 5 and into issue 15.
Its headline numbers were re-checked by the script verifier:
- the parameter-metadata builds are 61.6, 75.0 and 72.3 s;
- the pin-only commits since 2026-08-28 number 50 to 125, depending on what counts.

## 8. The 2026-09-04 audit: why nothing happened, and what I confirm

`docs/audits/test-usefulness-2026-09-04/` is a per-file ledger of 1,550 tests. Its README says:
"Nothing in this ledger has been applied. It is a proposal. Each deletion requires Sol's per-item
acceptance."

That acceptance never happened. The follow-up helper found that about 95 of its ~106 DELETE rows are
still present, none of the 20 TRIM rows it sampled was applied, and about 129 of ~150 MERGE rows are
still separate. Meanwhile about 1,000 tests were added. Both classifiers re-checked every row that
still maps to a live test.

- **Confirmed:** most DELETE and TRIM verdicts still hold, and they appear in the candidate lists
  ([`data/candidates-*.md`](data/)). None of host-core's delete, merge or trim verdicts was applied,
  and neither were the `tools/audit` `generated()` memoisation, the session, protocol, source and
  engine deletes, or the capi mirror trim.
- **Reversed:**
  - Keep `crates/capi/src/ffi.rs:2516`. It is a scrape, but with no Miri leg it is the only guard;
    rewrite it rather than delete it.
  - Keep `observation_identity.rs:122`, as V1 naming policy.
  - `tools/audit/src/source_duration.rs:317` goes from keep to delete (duplicate pin).
  - Four `disjoint.rs` tests are now out of scope (production builds one lease).
  - `conformance/tests/fixtures.rs:7` is not the only fixture gate (`qualification.yml:520`
    regenerates them).
  - graph-compiler `scale.rs:160` needs the full 65,537. The duplicate is `:91`'s unconstrained
    compile, not a need to trim to 10,923.
  - Several MERGE targets now hold unique mutation catches, for example rack `lib.rs:6264` and
    graph-compiler `:2513`.
- **Refined:** the per-crate trims in `data/candidates-*.md` carry exact representatives.
- **The four "known conflicts", settled by evidence:**
  1. Digest-pin ownership: one owner per corpus, `run-wasm-gates.sh` native plus Wasm legs (issue
     08).
  2. `controller_response_api.rs` is compile-only with no in-repo consumer. It is a delete
     candidate and the owner decides.
  3. The 65,537-track tests go to nightly, keeping one (issue 04).
  4. `effect_interchange_mutation.rs:318` asserts nothing and is deleted (issue 06).

**Why it failed, and what is different here:** the ledger needed one person to accept about 400
items one by one, and no issue owned the work. Here, each cut is a bounded issue with its own
objective gate, and the rule that stops the pattern returning is mechanical (§9).

## 9. Enforcement: keeping it from growing back

The mechanism must itself pass the ceremony boundary: cheap, and discriminating a real claim.

1. **One review rule in `AGENTS.md`**, under the issue workflow, in three sentences:
   - A new test names the defect it catches that no existing test catches. The existing
     `MUTATIONS.md` row is the evidence, and a regression reproducer of a real bug is exempt and
     permanent.
   - A change that supersedes a test deletes it in the same PR.
   - Exact byte, count or digest equality is allowed only for a wire, ABI or on-disk format, or for
     the one owner of a cross-target corpus. Budgets are ceilings.

   Sol's adversarial verdict already reads every new test; this adds one question to it.
2. **One lint line in `scripts/check-workspace-policy.sh`**, which already runs in about 1 s. It
   fails on `include_str!`, `include_bytes!` or `read_to_string` of a `.rs`, `.toml`, `.md`, `.sh`,
   `.yml` or `.py` path under `crates/`, `hosts/` or `tools/`, excluding `fixtures/`.
   - Today it has 33 hits: all test scrapes, plus one read of `fixtures/`.
   - It has no mutation suite: it is one `git grep`.
   - It lands with issue 07, after the scrapes are gone, so it needs no allow-list.

   A second lint, "no hex literal of 16 or more digits in two files", was prototyped and **rejected**.
   It finds 106 duplicated literals today, most legitimate: vendored libm constants, PRNG seeds and
   pinned Action SHAs. It would need an allow-list, which is ceremony. Duplicate pins are left to the
   review rule.
3. **Bounded issues, not a ledger.** Every cut in this audit is an issue small enough to close in
   half a day, with the equivalence gate below. That is what the 2026-09-04 audit lacked.

Optional, and not a gate: a weekly `cargo mutants --in-diff` over the week's changes, whose report
lists new survivors and zero-catch tests. It costs about one runner-hour a week. Adopt it only if the
owner wants the trend, because it is descriptive.

## 10. The objective gate every cut must pass

A cut loses no discrimination when both of these hold.

1. **Mutation equivalence.**
   - Run `cargo mutants` on each affected crate before and after the cut, with the same settings:
     [`tools/run-mutants.sh`](tools/run-mutants.sh), `--no-fail-fast`, the crate's CI features,
     sharding recorded in the issue.
   - The set of caught mutants must not shrink.
   - [`tools/parse_mutants.py`](tools/parse_mutants.py) builds the per-test matrix, and
     [`tools/dominance.py`](tools/dominance.py) shows which test takes over each removed test's
     catches.
2. **Historical bugs.** [`tools/revert.py`](tools/revert.py) re-injects #966, #970, #994 and #1015
   into a scratch copy. Each must still turn at least one remaining test red, and every reproducer
   named in [`data/bug-reproducers.md`](data/bug-reproducers.md) must remain.

Script-gate cuts have their own gate instead: every seeded violation the removed self-test covered
must still fail the real gate. Each issue lists those violations.

## 11. What must stay, and why

- **Real-bug reproducers** (all of [`data/bug-reproducers.md`](data/bug-reproducers.md)). They are
  the only tests proven to catch real defects, and the randomized ones also caught bugs they were not
  written for.
- **The randomized differentials:**
  - compressor `randomized_differential_*`;
  - graph-compiler `bank_levels` probe;
  - limiter `randomized_scenarios_*`;
  - graph `*_on_random_graphs`.

  These can be *trimmed* in seed count per PR, with the full count nightly, but they stay.
- **Realtime gates on the shipped artifact and binaries:**
  - the Wasm callgraph allocation-free closure and the vector ratchet (the one real performance
    catch);
  - the release audits' zero allocation, lock and syscall runs;
  - loom;
  - the V8 spill gate.
- **Cross-language ABI gates:** layout, command vocabulary, session map and SDK generated surface.
  They exist because hand copies drifted, and the `.d.ts` catch in §4.3 was one.
- **Wire and ABI vectors, the C ABI symbol check and consumer smoke.** Native and mobile hosts link
  through them.
- **One cross-target digest owner per corpus.** It is the mechanism an AArch64 leg needs to prove
  class A on phones.
- **DSP reference tests** (class D). They catch what a differential cannot: both sides wrong.
- **Contract tests with a unique catch.** host-core has 34; the compressor's restore, automation and
  malformed-block tests are examples.
- **The three browser legs.** WebKit is the engine behind fans' iOS web app; Firefox and Chromium are
  cheap.
- **Fuzzing:** per PR bounded and non-blocking, nightly deep.
- **Policy gates with real catches:** lane numeric-boundary, realtime `unsafe`, host-core facade,
  effect-contract completeness, native PCM runner dependencies, and the realtime-audit leak checks.

## 12. Owner rulings needed

| # | question | why it needs the owner | recommendation |
|---|---|---|---|
| R1 | Does native AArch64 (iOS/Android) come back into scope now that fans use mobile apps, reversing #378's "unsupported, no claim"? | changes a support claim; brings back known class-A defects (LANE-3) and a realtime one (Darwin `memset`) | yes, with an AArch64 CI leg first (issue 12) |
| R2 | Are exact retained-byte counts of fixtures a product requirement, or are budgets ceilings? | decides issue 01 | ceilings plus live reports |
| R3 | Enforce the artifact sha256 pin on every PR, or only at publish (plus a two-build reproducibility check)? And the `results.json` lineage record? | release-process claim; the app's provenance file consumes the pin | publish-time only; the lineage record is dropped (issue 15) |
| R4 | May test binaries build without fat LTO, given that D12 says one release profile for every *artifact*? | D12 wording | yes for test binaries; shipped artifacts keep their gates (issue 11) |
| R5 | Remove the scalar-Wasm legs *and* the "every other target" arms together? | "no target-specific code" and "modes production never needs" pull the same way, but it is code removal | yes (issue 05) |
| R6 | Isolated kernel benchmarks (`tools/bench/src/gate_active.rs`, `multiband_active.rs`, MQ-1/MQ-2) are not real host paths: delete them and their tests? | benchmark scope ruling | yes (issue 06 / 10) |
| R7 | `dependency_waves` is kept as a known-but-refused session token (`session/tests/render_mode_tiers.rs:59,72`), and extended research rates keep compatibility tests: keep either? | session grammar and release scope | drop both tests with the token |
| R8 | host-web keeps a "legacy" and a "protected" observation and spectrum path (`hosts/host-web/src/ffi.rs:646-690`), which doubles the refusal tests: is the legacy path still needed? | app contract | owner to confirm what the app calls |

## 13. Issue drafts

| # | draft | kind |
|---:|---|---|
| 01 | [Replace exact resource pins with ceilings](issues/01-replace-exact-resource-pins-with-ceilings.md) | cut (R2) |
| 02 | [Run gate self-tests only when the gate changes](issues/02-run-gate-self-tests-only-when-the-gate-changes.md) | cut |
| 03 | [Build parameter-metadata once per PR, and small CI duplicates](issues/03-build-parameter-metadata-once-per-pr.md) | cut |
| 04 | [Move 65,537-track scale tests to nightly](issues/04-move-65537-track-scale-tests-to-nightly.md) | shrink |
| 05 | [Retire scalar-Wasm legs and arms](issues/05-retire-scalar-wasm-legs-and-arms.md) | cut (R5) |
| 06 | [Delete tests that cannot fail](issues/06-delete-tests-that-cannot-fail.md) | cut |
| 07 | [Replace source-text scrapes and lint them](issues/07-replace-source-text-scrapes-and-lint-them.md) | cut + enforcement |
| 08 | [One owner per cross-target digest corpus](issues/08-one-owner-per-cross-target-digest-corpus.md) | merge |
| 09 | [Collapse dominated slow differentials](issues/09-collapse-dominated-slow-differentials.md) | shrink |
| 10 | [Delete unrun scripts and retired-feature gates](issues/10-delete-unrun-scripts-and-retired-feature-gates.md) | cut |
| 11 | [Ruling: release test binaries without fat LTO](issues/11-ruling-release-test-binaries-without-fat-lto.md) | ruling (R4) |
| 12 | [Add an AArch64/NEON CI leg](issues/12-add-an-aarch64-neon-ci-leg.md) | gap (R1) |
| 13 | [Run the tests CI currently compiles out](issues/13-run-the-tests-ci-currently-compiles-out.md) | gap |
| 14 | [Commit a randomized differential per effect and stage](issues/14-commit-a-randomized-differential-per-effect-and-stage.md) | target shape |
| 15 | [Ruling: artifact pin at publish, not per PR](issues/15-ruling-artifact-pin-at-publish-not-per-pr.md) | ruling (R3) |
| 16 | [Test-value rule in AGENTS.md](issues/16-test-value-rule-in-agents-md.md) | enforcement |

Order: 13, 12 and 16 first (gaps and the rule). Then 02, 03, 04 and 06, the cheap, certain ones.
Then 01, 05, 11 and 15 after their rulings, and 07, 08, 09 and 14 as capacity allows.

## Appendix: data files and reproduction

- [`data/inventory.md`](data/inventory.md): per-crate counts, lines, local time, CI job and claim
  classes.
- [`data/test-classes.tsv`](data/test-classes.tsv): one row per test, with crate, file:line, name,
  class, flags, family and note.
- [`data/candidates-dsp-graph.md`](data/candidates-dsp-graph.md) and
  [`data/candidates-host-tools.md`](data/candidates-host-tools.md): 120 itemised cut, merge and trim
  candidates, each naming its surviving guard.
- [`data/families-dsp-graph.md`](data/families-dsp-graph.md) and
  [`data/families-host-tools.md`](data/families-host-tools.md): the near-duplicate families.
- [`data/bug-reproducers.md`](data/bug-reproducers.md): the tests that must stay for #966, #970,
  #994 and #1015.
- [`data/ci-red-jobs.tsv`](data/ci-red-jobs.tsv): the 132 red jobs with category, failing gate,
  log evidence and fix commit.
- [`data/mutation-summary.md`](data/mutation-summary.md): per crate, the scores, unique catchers,
  zero-catch tests, dominance and survivors.
- [`data/script-gates-verify.md`](data/script-gates-verify.md): the script-gate verification.
- [`tools/`](tools/): `run-mutants.sh`, `parse_mutants.py`, `analyze_matrix.py`, `dominance.py`
  and `revert.py`, with usage in [`tools/README.md`](tools/README.md).
