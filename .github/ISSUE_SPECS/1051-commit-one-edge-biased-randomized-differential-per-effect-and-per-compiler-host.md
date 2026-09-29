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

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1051-randomized-differentials` from `a509b681` (the local batch
`codex/batch-slim-3`). No product code changed.

### What landed

- **One seed discipline and one edge-biased generator**, `dsp_reference::randomized`: a SplitMix64
  `Draw`, input profiles (clean, hostile, silence, signed zeros, subnormal, tiny, exact, loud),
  the hostile word set (both zeros, `from_bits(1..=3)`, subnormals, the flush edge, infinities,
  quiet and signalling NaN payloads, `+-MAX`, the words either side of the D7 bound), domain-edge
  parameter draws, random block lengths including one frame and the full quantum, and
  `run_seeds`, which prints the failing seed and the one command that replays it.
  `MISO_ENGINE_RANDOMIZED_SEED=<n>` replays one seed; `MISO_ENGINE_RANDOMIZED_SCALE=<k>`
  multiplies every per-PR seed set (nightly: 100).
- **One contract-level bank differential for every launch effect**,
  `conformance::run_effect_differential`, instantiated by `tests/randomized.rs` in compressor,
  delay, gate-expander, multiband-compressor, parametric-eq, soft-clip and transient-shaper (the
  limiter's was built, measured and dropped; see gate 2). Only the shipped contract calls, at every width the build binds. Oracles:
  bank vs scalar (bits and reports), chunked vs whole (scalar arm), collapsed
  (`process_bank_mono`) vs forced dual, with the witness modelled from `ChannelSymmetryWitness`,
  restored vs restored (self, cross-lane, hostile-word and effect-crafted payloads, accepted or
  refused alike), the snapshot's round trip, and the three-outcome bind rule. Invariants: the D7
  block bound on every unbypassed output, no allocation, lock or syscall in `process`, and on
  mono scenarios the witness must hold (`witness: true` for compressor, delay and EQ).
  Every scenario takes one planned reset, and one block in six is silence. Delay does not bank,
  so it runs the scalar-twin mode.
- **Stage differentials**: host-core (random consoles through host-core's own prepare entry
  points, armed vs forced-dual vs serialized lowering, with random live records; and response
  queries against the grid law across paths), builtins (input, fader and matrix banks at four and
  eight lanes vs one scalar section per track, with forced-dual blocks and random retargets),
  source (random ring schedules vs an independent model written from the documented rules),
  parametric-eq in-crate `randomized_restores` (stationary vs full cascade across random
  restores, including subnormal and signed-zero state), and compressor in-crate `witness_tests`
  (amendment 2: exactly one of `current`, `target`, `step`, `remaining` differing mid-ramp).
- The compressor's `randomized_width` and the limiter's
  `randomized_scenarios_render_exactly_the_unmodified_kernel` join the seed discipline.
- **A D7 report probe**, `conformance::d7_report_violations`, and one `#[ignore]`d reproducer per
  effect that breaks it (#1073). Every known-defect reproducer names its issue (#1069-#1073).
- `nightly.yml`: one step on the release link proof's own build, at 100x.

**Scope deviation, for Sol.** The body authorises only the host-core slice; the brief asked for
every slice in one attempt. The shared harness lives in `conformance` and `dsp-reference`, so eight
effect crates do not each copy it. Beyond new files: `compressor/src/kernel.rs` and
`true-peak-limiter/src/lib.rs` (seed plumbing in existing tests), `compressor/src/lib.rs` (an
appended `#[cfg(test)]` module), `parametric-eq/src/lib.rs` (one `mod` line), `source/Cargo.toml`
(a `dsp-reference` dev-dependency), and `docs/ENGINE_ENV_VOCABULARY.md` (the two variables).

**NaN as one class (#1065).** `same_word` treats every NaN as one value in: the effect
differential's output comparisons, the builtins differential's outputs, and host-core's rendered
outputs. Everything else is compared by bits, signed zeros included. Payloads and the source ring
are compared by raw bits: they carry words, they do not compute them.

### Gate 1: the historical bugs, re-injected at HEAD

`docs/handoffs/test-value-2026-09-28/tools/revert.py` into a copy of HEAD, the per-PR seeds:

| Bug | Differential | Result | Reverted |
|---|---|---|---|
| #970 (`arm_mono_collapse` without `gathers_track_input`) | host-core `randomized_consoles_render_the_same_bits_armed_dual_and_serialized` | red at seed 3 of 12 (armed and forced-dual consoles part at block 2) | green |
| #1015 (the pre-#1015 stationary predicate) | parametric-eq `randomized_restores::the_stationary_cascade_renders_the_full_cascade_after_random_restores` | red at seed 1 of 8 (`0x80000000` where the full cascade writes `0x00000000`) | green |
| compressor `designed_channel_symmetry` (amendment 2) | compressor `witness_tests` + `tests/randomized.rs` | all six `\|\| -> &&` at `lib.rs:704-706` and `:714-716` red; 18/18 of the function's survivors red | -- |
| gate-expander `bind_homogeneous_bank` (amendment 3) | gate-expander `tests/randomized.rs` | at `a509b681` three bind mutants survive the crate's suite, not eight: `1041` and `1061` go red; `1075` is in the `Four` arm, which x86 never binds (expected red on the AArch64 leg's native `Simd4` bind, not run here) | -- |

### Gate 2: mutation yield

cargo-mutants 27.1.0, opt-level 1 (the audit's environment). Every run began with an unmutated
baseline that passed (F4); the cross-crate test set was baselined separately, unmutated, first.
The middle column counts mutants that survive the crate's own CI suite at `a509b681` and that a
new test turns red. The last column is what is left of them after a second pass against the other
CI suites that render the crate: effect-compiler, graph-compiler, host-core, host-web and capi
(`test-debug-a`'s features). Only that column is "caught by nothing else".

| Effect or stage | Mutants; survivors of the crate's suite | Caught by the new tests, missed by the crate's suite | Still unique across CI |
|---|---|---|---|
| host-core (the spec's `--shard 0/4 --sharding round-robin`) | 359; 63 survive | **9 (14.3 %, gate >= 10 %)**, all in `response.rs` (`apply_overrides`, `index_of_parameter`, `generate_response_grid` x4, `query_response_snapshot_into` x3), by the response differential; the console differential catches #970 | **9**: host-web and capi miss all nine |
| compressor | 93 survive (the #1049 baseline) | **26**: `designed_channel_symmetry` x18, `channel_symmetry` to true and to false, `lane_channel_symmetry` to true and to false, and four `render` mutants on the silent path (`510:43`, `536:29`, `537:21`, `537:37`), which the audit called output-equivalent and are not | **11**: the six amendment-2 `\|\| -> &&` (`704-706`, `714-716`), the four silent-path `render` mutants, `lane_channel_symmetry -> true` |
| gate-expander | 229; 41 survive | **10**: `apply_automation` validity x8 (`544-551`), `bind_homogeneous_bank` `1041` and `1061`. `commit_lane` `796` (a restore that does not extend the ramp window) is caught at nightly scale only (seed 128) | **10** |
| transient-shaper | 169; 19 survive | **7**: `apply_automation` validity (`724-734`) | **7** |
| delay | 74 on the contract surface; 28 survive | **13**: `bind_homogeneous_bank` `598`, `designed_channel_symmetry` x11, `channel_symmetry` to false | **1**: `bind_homogeneous_bank` `598` (the witness mutants are caught elsewhere) |
| soft-clip | 274; 28 survive | **2**: `SoftClipState::field_mut` arm 1, the bank's `reset` emptied | **2** |
| true-peak-limiter | 91 on the contract surface; the new test catches 65 | **7** of those survive the crate's suite: `designed_channel_symmetry` x5, `channel_symmetry` to false, `lane_channel_symmetry` to false | **0**: all seven caught elsewhere; the differential is dropped (below) |
| parametric-eq | 149 on the contract surface; the new tests catch 67 | **9** of those survive the crate's suite: `designed_channel_symmetry` x7, `channel_symmetry` to false, `lane_channel_symmetry` to false; and #1015 | **0** mutants (all nine caught elsewhere); #1015 and #1070 |
| multiband-compressor | 49 on the contract surface; 8 survive | **none of the 8** per PR (see below). A planted mutant, `plan_segment` cutting segments on lane 0's ramps only, survives every one of 49 CI test binaries in multiband, effect-compiler, graph-compiler, host-core, conformance, rack and graph; the differential is red at seed 1 | **1** planted (`plan_segment`), and #1069 |
| builtins | 137 on the bank surface; the new test catches 46 | **none**: the existing suite catches all 46. It catches `MUTATIONS.md` M2-B1 (seed 0), as the existing `chain_shape` test does | **0**; #1072 |
| source | 176; 25 survive | **3**: `SourceGeneration::is_valid` to true, `validate_submission_metadata` `delete !` (`943`), `PcmSourceConsumer::end_reached` to false | **2**: `is_valid`, `end_reached` (host-core's tests catch `943`) |
| graph and rack collapse, by host-core's console differential | graph: 28 collapse mutants, of which the console test catches 18; rack: 78, of which it catches 19 | **none**: the existing rack, graph, graph-compiler and host-core suites catch all 37 | **0** |

**What caught nothing new, and why each stays.**

- **builtins** catches no unique mutant. It stays because it found #1072: a real
  bank-against-scalar break that the fixed-input gate (`stage.rs`) cannot see, because that gate
  never feeds a signed zero.
- **multiband** catches none of the eight recorded survivors per PR. The narrowing for #1069
  forbids chunked blocks on automated scenarios, and chunking was how the first harness reached
  `apply_automation` `1261` and `1262`. It stays because it found #1069, and because it
  catches the planted `plan_segment` mutant that nothing else in CI catches. Once #1069 is
  fixed, the narrowing goes and the strict twin becomes the gate.
- **host-core's console differential** catches no host-core mutant, because host-core delegates
  render to graph and rack. It stays for #970; the 37 graph and rack collapse mutants it catches
  are all caught by existing suites too.
- **parametric-eq's bank differential** has no unique mutant across CI. It stays because it
  found #1070; `randomized_restores` catches #1015 (gate 1).
- **Dropped: the true-peak limiter's bank differential.** Its seven catches are all caught by other
  crates' suites, and it found no defect. Its file keeps the #1073 reproducer (commit `efe8fff1`).

### Gate 3: budget

Each new test was run alone with `--test-threads=1` on this shared box (load 18-27). Times are
wall time inside the test binary.

| CI job | Test | debug | release |
|---|---|---|---|
| `test-debug-b` | compressor `tests/randomized.rs` | 0.48 s | 0.03 s |
| `test-debug-b` | compressor `witness_tests` | 0.02 s | 0.00 s |
| `test-debug-b` | delay | 0.26 s | 0.03 s |
| `test-debug-b` | gate-expander | 0.26 s | 0.01 s |
| `test-debug-b` | multiband-compressor | 0.38 s | 0.01 s |
| `test-debug-b` | parametric-eq `tests/randomized.rs` | 0.62 s | 0.02 s |
| `test-debug-b` | parametric-eq `randomized_restores` | 1.09 s | 0.06 s |
| `test-debug-b` | soft-clip | 0.61 s | 0.01 s |
| `test-debug-b` | transient-shaper | 0.44 s | 0.03 s |
| `test-debug-b` | builtins | 0.62 s | 0.01 s |
| `test-debug-b` | dsp-reference `randomized::tests` | 0.01 s | 0.00 s |
| `test-debug-a` | source | 0.08 s | 0.01 s |
| `test-debug-a` | host-core consoles | 3.50 s | 0.15 s |
| `test-debug-a` | host-core response queries | 0.21 s | 0.02 s |

- **Per PR:** about 4.8 s of test time in `test-debug-b` and 3.8 s in `test-debug-a`, about
  8.6 s in total (the limiter's 0.75 s went with its differential). All of them run again in
  `aarch64-debug`, whose product crates include every one. Compile time for the eleven new test
  binaries and the harness was not measured on a quiet box.
- **Release:** no per-PR release job runs these crates, so nothing is added there.
- **Largest single test:** host-core's console differential, at 3.5 s. No test comes near 10 s,
  so no sweep moved to nightly beyond the 100x step.
- **Unchanged per-PR shares:** the compressor's `randomized_width` and the limiter's randomized
  scenarios keep theirs.
- **Nightly:** the whole set at 100x comes to about 45 s of release test time (scaled from the
  release column), plus the #966 probe at 6400 seeds, on the link proof's existing build.
- **Flake check:** every new per-PR test ran three times at the per-PR seeds. Exit codes were
  0 0 0 each time, and the printed coverage was byte-identical across the runs.
- **Wider sweeps:** scale 10 in debug passes everywhere.

### Gate 4: replayable

A red seed prints the test, the seed, and the one command that replays it
(`MISO_ENGINE_RANDOMIZED_SEED=<n> cargo test -p <crate> ...`). Every reproducer command below
was run, and each fails at the seed it names.

### Real defects

These are not fixed here; root filed one issue each. Each reproducer carries
`#[ignore = "#<issue>: ..."]`. The per-PR test narrows around the defect by a named
`conformance::Known`, or for builtins by comparing the two zeros as one value on ramping blocks,
and nowhere else.

1. **#1069, multiband: where a ramp is cut moves a lane's bits.**
   - Reproducer: `MISO_ENGINE_RANDOMIZED_SEED=1 cargo test -p multiband-compressor --test
     randomized -- --ignored --exact the_bank_renders_its_scalar_instances_including_the_known_defect`.
   - What goes wrong: in a four-lane bank, block 11, lane 0 left frame 54, the bank renders
     `0x3f5e220a` where its scalar instance renders `0x3f5e24e0`. This follows a block-rate
     retarget while another lane's ramp is in flight. The scalar instance also renders a
     mid-ramp block differently whole and in two pieces.
   - Cause: `Instance::plan_segment` ends a segment at any lane's ramp arrival and at every
     block boundary, and refreshes coefficients per segment.
   - Breaks: bank bit identity (AGENTS.md: banking never changes per-lane arithmetic) and
     partition invariance (P1).
2. **#1070, parametric EQ: bind declines before it validates its members.**
   - Reproducer: `MISO_ENGINE_RANDOMIZED_SEED=0 cargo test -p parametric-eq --test randomized --
     --ignored --exact the_bank_renders_its_scalar_instances_including_the_known_defect`.
   - What goes wrong: at four lanes, a width this x86 build does not execute, and on a
     heterogeneous member, `bind_homogeneous_bank` answers `Ok(None)` for a cohort whose member
     `prepare` refuses (`effect.parameter.initial`).
   - Breaks: validation order, the three-outcome rule on `NativeEffectFactory` (an invalid
     member refuses first; an absent capability never hides a malformed member).
3. **#1071, soft clip: a snapshot with subnormal words is refused on its own restore.**
   - Reproducer: `MISO_ENGINE_RANDOMIZED_SEED=0 cargo test -p soft-clip --test randomized --
     --ignored --exact the_bank_renders_its_scalar_instances_including_the_known_defect`.
   - What goes wrong: in an eight-lane bank, block 9, lane 0 refuses its own snapshot. The
     snapshot holds a legal in-domain subnormal parameter value in a ramp, or a subnormal input
     sample in the history rows, and restore requires those words to be zero or normal.
   - Breaks: the snapshot round trip (deterministic state restore) and the NaN/denormal contract
     (a subnormal input is legal and renders, yet the restore refuses what the effect wrote).
4. **#1072, builtins: the matrix bank renders a settled lane's `-0.0` as `+0.0`.**
   - Reproducer: `MISO_ENGINE_RANDOMIZED_SEED=7 cargo test -p builtins --features
     builtins/test-support --test randomized -- --ignored --exact
     the_banks_render_their_scalar_sections_including_the_known_defect`.
   - What goes wrong: at W4, block 9, lane 2 left frame 1, the banks render `0x00000000` where
     the scalar strip renders `0x80000000`. While any lane's matrix ramps, the bank runs the ramp
     arithmetic on every lane, and one times `-0.0` plus zero times x is `+0.0`.
   - Breaks: bank bit identity, and a signed zero retained on a non-recursive path.
5. **#1073, the D7 recovery's report breaks the contract in five effects.**
   - Reproducer: `cargo test -p <crate> --test randomized -- --ignored --exact
     the_d7_recovery_reports_one_block_on_the_failing_lane`, in gate-expander,
     multiband-compressor, soft-clip, transient-shaper and true-peak-limiter. Fixed input, so no
     seed: `conformance::d7_report_violations` poisons one lane with a NaN block at the defaults.
   - Gate-expander counts 64 for one 64-frame block, the frames rather than the block.
   - Multiband counts the frames on both channels.
   - Soft clip counts the frames, on both channels, on every lane of the bank.
   - Transient shaper and limiter zero the block and reset, but count nothing.
   - Compressor, delay and EQ pass.
   - Breaks: `docs/EFFECT_CONTRACT_V1.md` D7 ("increments a block counter"; the report "counts
     blocks, never samples"). Gate-expander's own test at `lib.rs:1313` pins the frame count.
   - Related, for an owner ruling rather than a defect: soft clip, multiband, transient shaper
     and limiter zero and reset the whole bank when one lane fails. The contract says "per bank",
     but AGENTS.md says a placement change must not move a rendered bit, and here a clean
     neighbour's output depends on its placement. The differential therefore ends a scenario at
     a D7 block and checks only the bound.

Other findings, not defects:

- Multiband's restore canonicalises a `-0.0` payload word. This is benign, so the round trip
  compares zeros as one value.
- An EQ mid-ramp restore refusal seen in an earlier revision of `randomized_restores` did not
  reproduce. It came from that revision's retargets, which moved a ramp without its prepared
  target. No restore was refused over 80 seeds, and the test's comment now says so.

### Cross-target

- **Width-agnostic.** The `wide` kernels and the builtins banks run both widths on every host.
  Effect banks bind at the native width (x86 `Simd8`; multiband binds both), and
  `banks_natively` makes a native bind required. On AArch64 the same tests bind `Simd4`. They were
  not run on arm64 here; #1019 and #1065 are the risk.
- **NaN as one class (#1065).** NaNs compare as one class in the effect differential's output
  comparisons, in the builtins outputs, and in host-core's outputs.
- **Raw bits.** Payload words and the source ring compare by raw bits: they carry words, they do
  not compute them.
- **AArch64 debug leg on x86.** `cargo test --no-run` over its product crates and features
  resolves, with 1821 tests listed. `aarch64-known-defects.py judge-skips debug` passes, and the
  silent-skip scan is clean.

### Gates run

- `cargo check --workspace --all-targets --all-features`; clippy `-D warnings` over the whole
  workspace with all features; `cargo fmt --check`.
- Affected crates' full suites, dev and release: dev 835 passed, 19 ignored, 0 failed over 148
  test binaries (369 s wall at load about 50); release the same counts (65 s). They ran after the
  D7 probe landed; each later commit (message text, comments, issue tags, the limiter drop) was
  re-tested in its own crate.
- Policy scripts:
  - Every `scripts/check-*.sh` and `check-*.py` ran once this attempt, the Python ones with
    `python3 -B`. `check-sdk-types.sh` needs `npm ci`, which is environment, not code.
  - The source-scanning set ran again after the last change: workspace policy including #1052's
    scrape lint, effect contract, conformance boundaries, realtime, host-core, builtins, graph,
    rack, lane, env vocabulary, effect runtime, session, the audit leaks, unfused seal, research,
    bench, step vocabulary, test-support CI, CI path routing, script reachability and release
    shape.
  - No new test reads source text.
- Console digests: no product code changed, and `console-workload` passes in release.
- Routing: `nightly.yml` changed, and `check-ci-path-routing.py` passes.

### Lines added

`git diff --numstat a509b681..HEAD` before this section: 24 files, 5,775 lines added and 11
removed. Nearly all of it is test code:

- **Shared harness**, about 2,450 lines: `conformance/src/randomized.rs` (2,053, including the D7
  probe) and `dsp-reference/src/randomized.rs` (391), plus 6 lines of exports.
- **Stage differentials**, about 2,735 lines: host-core 1,242, `parametric-eq/src/randomized_restores.rs` 508
  (plus 2 lines of `mod`), source 493 (plus 3 lines of manifest and lock), builtins 487.
- **Effect test files**, 389 lines: compressor 134, multiband 68, soft-clip 57, parametric-eq 48,
  gate-expander 25, transient-shaper 25, delay 17, limiter 15.
- **In-crate edits**, 181 added and 9 removed: compressor `witness_tests` 144, compressor
  `kernel.rs` +16/-3, limiter `lib.rs` +21/-6.
- **CI and docs**, 20 added and 2 removed: `nightly.yml` 16, `ENGINE_ENV_VOCABULARY.md` +4/-2.

Plus this evidence section.

## Sol verdict, attempt 1

**FAIL.** The differentials are real, deterministic and replayable. The five defects reproduce as
filed, and the claimed catches I re-ran hold. But merged into the batch, #1051 no longer compiles
wherever nothing turns on `lane/test-support`, and that breaks three required CI jobs and every
effect crate's printed replay command. A second finding is an early return that can hide a D7
trip on legal input.

Verified on a scratch merge of `cf3861bb` into `codex/batch-slim-4` at `124968b7` (main plus
#1060, #1062 and #1050). The textual merge is clean. The semantic conflict is F1.

### Findings, by severity

1. **Blocking: a broken gate on the merge (semantic conflict with #1059).**
   - #1059, which came in with the batch, compiles `lane::Backend::Scalar` only under
     `lane/test-support`.
   - `crates/conformance/src/randomized.rs:1692` names `Backend::Scalar` in `bind_eligibility`'s
     malformed-shape rows. That row is `("the scalar backend", &base[..], Backend::Scalar)`.
   - Conformance's lib now compiles only where a dev-dependency unifies that feature in. The
     compressor, builtins and graph test-support paths do; `audit` and `bench` depend on
     `conformance` as a normal dependency, and nothing turns it on for them.
   - Reproduced on the merge, each with `error[E0599] ... Backend::Scalar`:
     - `cargo check --workspace` fails.
     - The lint job's `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` fails
       (exit 101).
     - audit-native's `cargo build --release -p audit -p bench -p capi -p session-validator` fails,
       and so does its `cargo test --release -p audit -p bench -p console-workload`. The scripts
       that build those binaries also fail: `check-effect-contract.sh` and
       `check-builtins-fixtures.sh`.
     - aarch64-release's `cargo build --release -p audit` fails.
     - Nightly's `test-native-vectorization-report.sh` fails.
   - **Gate 4 is broken as well.** These commands no longer compile:
     - the printed replay command of seven effect crates, `cargo test -p <crate> --test randomized
       ...` for every effect except the compressor;
     - the EQ `--lib` replay;
     - every #1069, #1070, #1071 and #1073 reproducer command in the evidence.

     The compressor, builtins, host-core and source replays still compile.
   - **Fix.** Drop that row. A shipped build cannot form a `Backend::Scalar` request, and the other
     three malformed-shape rows keep the probe. Then re-run `cargo doc`, `cargo check --workspace`,
     the audit-native build step and each printed replay command exactly as printed.
2. **Should fix: the D7 early return hides a D7 trip on legal input.**
   - `run_width` ends a scenario silently whenever `recovered()` sees any lane report a D7 block
     (`conformance/src/randomized.rs:1024-1047`). That includes a block with finite, legal input
     and no terminal restore.
   - D7 zeroes that block, so the "D7 bound" check passes vacuously there. The spec's "finite
     output for finite legal input" invariant is therefore never enforced.
   - **Planted defect.** I inserted a NaN into the compressor's dual `render` output whenever a word
     matches `bits & 0xffff == 0x1234`, on legal input. The compressor differential stayed green
     with the defect in place: two scenarios recovered on non-hostile, finite blocks and ended
     early.
   - **Per-PR data, instrumented and then reverted.** All 46 recoveries across the per-PR effect
     set fall on the drawn hostile block. None falls on a legal block.
   - So asserting "recovery only on the hostile block" costs nothing and closes the gap.
3. **Low: the #1072 allowance is narrow in value but not in scope.**
   - **What it forgives.** Only `+0.0` against `-0.0`. It never forgives a NaN or any other word.
   - **Where it applies.** It covers every lane, and the output composed from all three stages. It
     is on for every block that starts before the latest matrix ramp's end.
   - **Per PR.** That is 542 of 1,152 builtins blocks (47 %), and 57 words were actually forgiven.
   - **It outlives the ramp.** The reset branch does not clear `matrix_ramp_end`.
   - **Consequence.** A signed-zero defect in the input or fader bank is hidden on those blocks. It
     stays visible on the strict 53 %.
   - **Fix.** Compare the matrix stage on its own and forgive only there, and clear the end on
     reset.
4. **Low: the other two allowances check less than they could.**
   - `Known::SubnormalStateRefusedOnRestore` (#1071) accepts an own-snapshot refusal with any
     code. Per PR, all five tolerated refusals were `effect.state.history` with a subnormal word in
     the payload, so today it forgives only the defect, but nothing checks that.
   - `Known::BindDeclinesBeforeValidating` (#1070) accepts the decline at every width, the native
     one included.
5. **Budget: consistent with the claim, on a loaded box.**
   - Measured single-threaded with `--report-time` at load 35-50:
     - `test-debug-b`: about 7.8 s (EQ restores 1.56 s, compressor 1.04 s, soft clip 1.02 s, EQ
       0.95 s, builtins 0.82 s, transient shaper 0.81 s, multiband 0.64 s, delay 0.48 s,
       gate 0.46 s, witness 0.04 s);
     - `test-debug-a`: about 3.2 s (consoles 2.95 s, response 0.24 s, source 0.04 s).
   - That is about 11 s against the claimed 8.6 s at load 18-27. Over the spec's 5 s per debug job
     at this load, and neither of us measured on a quiet box or timed the link of the 11 new
     test binaries.
   - **Where they run.** `test-debug-a`, `test-debug-b` and `aarch64-debug` per PR, and nightly at
     100x. They do not run in `test-release` or `aarch64-release`.
6. **Not verified: AArch64 execution.**
   - Resolved on x86 instead. The `aarch64-debug` package and feature set compiles and lists 1,821
     tests. Every new test is in that list.
   - `aarch64-known-defects.py judge-skips debug` passes: both rows name an existing test. The
     known-defects self-test and the silent-skip scan are clean.
   - Width handling is sound. Each harness binds both widths, `banks_natively` requires the native
     bind, and NaN is one class only in output comparisons.
   - No arm64 hardware or emulation exists here, so the #1019 and #1065 risk on the new tests
     stands unmeasured.

### Graph and rack (host-core's console differential): keep it

AGENTS.md judges a differential by what its generator reaches, not by unique catches. This is the
only generator that drives live-console records through an armed collapse, and the only
differential over `prepare_host_runtime_between_render_calls`. The records are one-channel
parameter points, live bypass, and trim, fader, mute and matrix retargets. The #966 probe uses
neither.

It is also the largest per-PR share, about 3 s. If budget matters, halve its seeds per PR; the
nightly run still covers 100x.

### Verified

- **Unique catches.** I re-ran four mutants by hand; cargo-mutants is not installed here. Each is
  red on the new test and green on every other suite. "Every other suite" means the full
  `test-debug-b` and `test-debug-a` commands, plus `audit`, `bench` and `console-workload` in dev.
  - compressor `lib.rs:704` `|| -> &&` (amendment 2): red on both `tests/randomized.rs` and
    `witness_tests`.
  - compressor `lib.rs:536:29` `== -> !=` (silent path): red, from the bank state against its
    scalar instance.
  - gate-expander `lib.rs:546` `&& -> ||` (`apply_automation` validity): red, by an
    out-of-bounds span index.
  - host-core `index_of_parameter -> 0`: red, `DuplicateParameter`.

  A fifth, host-core `generate_response_grid` `> -> >=` at Nyquist, is also caught by an existing
  lib test. It was my pick, not one of the claimed nine.
- **Reproducers.**
  - The four seeded ones fail at their seeds for the stated reasons:
    - #1069 at seed 1: `0x3f5e220a` against `0x3f5e24e0`, lane 0, frame 54;
    - #1070 at seed 0: `effect.parameter.initial` declined;
    - #1071 at seed 0: `effect.state.history`;
    - #1072 at seed 7: `0x00000000` against `0x80000000`.
  - The five #1073 D7 probes fail as described.
  - All nine `#[ignore]`s name their issue, and #1069-#1073 are open on GitHub.
- **Determinism.** Three runs of every effect binary are byte-identical. Seeds are fixed, from 0
  to the per-PR count, and a replay by seed reproduces the failure.
- **No source scrapes.** `check-workspace-policy.sh`, including #1052's lint, passes. The only
  `include_str!` reads a session fixture, which is data.
- **Gates on the merge.**
  - `cargo check --workspace --all-targets --all-features`, clippy `-D warnings` and fmt pass.
  - `test-debug-b` passes in dev (807 passed, 0 failed) and in release with the same counts.
  - `test-debug-a` passes in dev (1,270 passed). host-core and source also pass in release
    (210 passed).
  - Console-workload's digests pass in dev.
  - The routing checks pass: `check-ci-path-routing.py` and `test-ci-path-routing.py`.
  - Every other policy script passes, except the three F1 casualties, and except those that need a
    prebuilt binary, a browser or npm (`check-capi-abi`, `check-graph-determinism`, `sdk`, `web`
    and `cross-targets`). No product code changed.

### Test value

- **compressor `tests/randomized.rs` and `witness_tests`:** a witness that folds one field with
  `&&`, or a silent fast path that earns its claim from the wrong word (#970 class).
- **gate-expander, transient-shaper:** an automation-span validity check that admits an off-start
  or out-of-range span.
- **delay:** a bind that banks an ineligible cohort.
- **soft-clip:** a bank reset or state field that diverges from the scalar instance.
- **multiband:** ramp segmentation that depends on a neighbouring lane (the planted `plan_segment`
  mutant, #1069).
- **EQ bank differential:** the three-outcome bind order (#1070).
- **EQ `randomized_restores`:** a restored subnormal that flips a signed zero through the
  stationary elision (#1015).
- **builtins:** bank-against-scalar sign and sanitised counts under live retargets (#1072).
- **source:** the ring's generation, stale and end-of-region verdicts against the documented model.
- **host-core response:** the override-list and grid-law edges (`index_of_parameter`).
- **host-core consoles:** the armed and the serialized lowering against dual under live records.
- **D7 probes:** red on #1073 by construction.

## Attempt 2 evidence

Terra, 2026-09-29. `codex/batch-slim-4` at `3a2782f1` merged in first (`c1c34a58`: #1059, #1060,
#1062, #1032, #1044, #1046, #1047, #1050), then the fixes in `65fdd92b`. Every result below is
on that merged tree.

### F1: the build break (blocking), fixed

`bind_eligibility`'s malformed-shape probe drops the `Backend::Scalar` row. A shipped build cannot
form that request since #1059, and the three other rows keep the probe: one member short, one
long, another width's backend. The commands Sol found broken, each run as written:

| Command | Result |
|---|---|
| `cargo check --locked --workspace` | exit 0 |
| `cargo check --locked --workspace --all-targets --all-features` | exit 0 |
| lint's `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| audit-native's `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| audit-native's `cargo test --locked --release -p audit -p bench -p console-workload` (`--no-run`) | exit 0 |
| aarch64-release on x86: `cargo test --locked --release -p lane -p math --features math/lane --no-run` | exit 0; 114 tests listed; `judge-skips release` passes |
| aarch64-release: `cargo test --locked --release -p console-workload --no-run` | exit 0 |
| aarch64-release: `cargo build --locked --release -p audit` | exit 0 |
| aarch64-release: `cargo test --locked --release -p wasm-gates --features math/lane --no-run` | exit 0 |
| `scripts/check-effect-contract.sh`, `scripts/check-builtins-fixtures.sh` | exit 0 |
| every printed replay command, exactly as printed: the seven effect `--test randomized` replays, the compressor `witness_tests` and `randomized_differential`, the EQ `--lib` restores, the limiter `--lib`, both host-core, builtins and source | all compile and pass |
| the four seeded reproducers (#1069 seed 1, #1070 seed 0, #1071 seed 0, #1072 seed 7) and the five #1073 D7 reproducers, as printed | all compile and fail as filed |

### F2: the hidden D7 trip (blocking), fixed

A D7 recovery before the drawn hostile block is now red, in the bank mode (`run_width`) and in the
scalar-twin mode (`run_scalar`, the delay). Every word before that block is legal: input,
sidechain, spans and restored state. So a recovery there is an effect diverging on legal input,
and the zeroed block would otherwise pass the bound check vacuously.

- **Sol's plant** (a NaN in the compressor's dual `render` output whenever a word's low 16 bits
  are `0x1234`):
  - with attempt 1's harness (plus the one-line F1 fix, so it compiles on the merged tree): green;
  - with attempt 2's: red at seed 24, "D7 recovered on a block whose input and state are legal".
- **No false reds.** Every effect differential passes at the per-PR seeds and at 10x.

### F3: the allowances, narrowed

- **#1072 (builtins).**
  - The differential now compares each stage on its own: input section, then fader, then matrix.
  - The allowance covers only the matrix stage, on a member whose matrix has settled on
    identity, and only on the frames where another member's matrix ramp is still in flight.
  - It forgives only a bank `0x00000000` where the scalar section wrote `0x80000000`.
  - A reset clears the ramp ends.
  - It forgives 57 words per PR, as before, and 2160 at 10x.
  - The strict twin still fails at seed 7, now naming "the matrix stage".
- **Planted signed-zero flips outside that lane**, `L::WIDTH > 1` only so the scalar sections
  stay clean:

  | Plant | Attempt 1's test | Attempt 2's test |
  |---|---|---|
  | SZ-1: the matrix bank's ramp frames flip `-0.0` to `+0.0` on every lane, the ramping lane included | green (hidden) | red at seed 0: the matrix stage, lane 0, a ramping lane |
  | SZ-2: the fader bank's output flips `-0.0` to `+0.0` on every lane | red at seed 1 | red at seed 0, naming the fader stage |

  The same flip at the fader's *input* goes green under both: the input section never hands the
  fader a `-0.0` on these draws.
- **#1070 (EQ).** The decline is forgiven only at a width this build does not execute (`Four` on
  x86, `Eight` on AArch64), and only when `prepare` refuses the member with
  `effect.parameter.initial`.
  - Planted K-1070, the EQ declining a NaN member at the native width too: attempt 1 green,
    attempt 2 red at seed 0 ("Eight: bind declined ...").
- **#1071 (soft clip).** An own-snapshot refusal is forgiven only with `effect.state.parameter` or
  `effect.state.history`, in both modes.
  - Planted K-1071, the history refusal under another code: attempt 1 green, attempt 2 red at
    seed 0.

### F4: host-core's console differential stays

It is kept, per Sol's recommendation.

### F5: NaN folding

The local NaN folds (`same_word` and `first_difference`) stay until root reconciles them with
#1065's shared helper at merge.

### Gates on the merged tree

- **Build and lint:** check, clippy `-D warnings` (workspace, all features) and fmt pass.
- **CI's debug commands, as written in `qualification.yml`:**
  - `test-debug-b`: 788 passed, 0 failed, 28 ignored over 149 binaries (167 s wall, loaded box).
  - `test-debug-a`: 1,252 passed, 0 failed, 7 ignored over 90 binaries (245 s).
  - Release, the affected crates (dsp-reference, conformance, the seven effects, the limiter,
    builtins, source, host-core): 814 passed, 0 failed, 16 ignored over 144 binaries (186 s).
- **Policy scripts:**
  - 32 of 33 `check-*.sh` pass. `check-sdk-types.sh` needs `npm ci`, which is environment.
  - 9 of 16 `check-*.py` pass as run bare, with `python3 -B`. The other seven need an argument;
    each passes `--self-test`.
- **AArch64 debug leg on x86:** `--no-run` over its product crates and features exits 0, 1,779
  tests are listed, `judge-skips debug` passes, the known-defects self-test passes, and the
  silent-skip scan is clean.
- **AArch64 release leg:** its `--no-run` and builds are in the F1 table.
- **Budget:** each new per-PR test was timed alone, single-threaded, at load about 15.
  - `test-debug-b`: 4.4 s (EQ restores 1.04, EQ 0.63, builtins 0.62, compressor 0.47, soft clip
    0.42, multiband 0.35, transient 0.33, delay 0.26, gate 0.26, witness 0.02).
  - `test-debug-a`: 2.8 s (host-core 2.73, source 0.04).
  - Sol measured about 11 s at load 35-50. Neither figure comes from a quiet box.

### Lines

- Attempt 2's own change (`65fdd92b`): `conformance/src/randomized.rs` +45/-14 and
  `builtins/tests/randomized.rs` +76/-29.
- The branch against the batch head: 5,853 code lines added and 11 removed, plus this spec.
