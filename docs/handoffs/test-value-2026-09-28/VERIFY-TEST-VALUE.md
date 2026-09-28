# Verification of the test-value audit, 2026-09-28

Sol's adversarial verification of [`TEST-VALUE-AUDIT.md`](TEST-VALUE-AUDIT.md) and its 16 drafts.

- **Base.** Branch `verify-test-value` at `73e50f7d`. Its code is identical to the audit's base
  `a9414c0c`: `git diff a9414c0c HEAD` over `crates hosts tools scripts .github/workflows sdk
  Cargo.*` is empty.
- **Filed issues.** Specs #1017-#1041, and the rulings in
  `docs/rulings/engine-footprint-2026-09-28.md`.
- **Later work on other branches.** #1021 is merged into `codex/batch-slim-1` (`4a0d60bd`). #1041
  is implemented on `codex/1041-refuse-32-bit-targets` (`4ef7c986`). I read both, and cite them
  where they change a draft.
- **Nothing in the product, the tests or CI was changed.**
- **Every "red" or "green" below comes from a command run in this session.** Commands ran on
  scratch copies of the tree, with `CARGO_INCREMENTAL=0`. Timed runs ran under the shared timing
  lock.

**Scratch loss.** At about 09:54 another agent's cleanup deleted the whole shared scratchpad,
including my folder and the timing lock file.
- **Lost:** the raw outputs of the full compressor and gate-expander mutation passes, the 19
  fetched CI logs and the CI-timing helper's data.
- **What I rely on instead:** their results as printed into this session before the wipe.
- **Re-run after the wipe:** the capi dropped-owner experiment, the #962 remnant timings under the
  recreated lock, all three bug re-injections, and the cross-crate symmetry mutation pass.
- **Not re-run:** the 674 s injected scale run. It ran from 09:34 to 09:46, under the old lock,
  before the wipe, so no other timed run overlapped it. My own mutation jobs were loading the host
  at the time, so its time is used only as a ratio against the base run (73 s), which ran under the
  same load.

## 1. Verdict

- **The diagnosis holds.**
  - The waste is concentrated.
  - Pins cause most red jobs, and none of them caught a defect.
  - Real bugs are caught by reproducers and randomized differentials.
  - Most tests are cheap duplicates.
- **The headline numbers reproduce:** the mutation scores, the red-job classification, the bug
  re-injections and the script-gate costs.
- **The savings hold only approximately.**
  - The no-ruling saving is about 430-520 s per full run, not a flat 520 s.
  - The "about 850 s with rulings" is not measured: at most about 670 s is measured, and R4 is
    unmeasured.
- **Four cuts would lose discrimination as drafted:**
  - **04** removes the only per-PR guard of #962 (a compile and bind that is linear in track count).
  - **01** removes both independent oracles of accounting completeness.
  - **15** removes the mechanical check behind at least 16 filed specs' "shipped artifact" gates.
  - **16**'s rule would have prevented only about half of its own ten counter-examples.
- **The headline gap is overstated.** 12 of the compressor's 18 surviving `designed_channel_symmetry`
  mutants are caught by cross-crate tests that CI runs. Six real survivors remain, and they name a
  narrower gap.
- **Four drafts are already owned by filed work:**
  - 12 → #1017;
  - 13 → #1021 (merged);
  - 05 → the "pending decision" #1041 left open;
  - most of 10 → #1027.

## 2. What was re-run

| audit claim | how I checked it | result |
|---|---|---|
| compressor mutation pass (the audit ran it; all 654 mutants) | `cargo mutants` 27.1.0 with the audit's settings (`run-mutants.sh` flags, run directly) | **identical:** 525 product mutants, 390 caught, 93 missed, 42 unviable, 80.7 %; the same catcher buckets (40/35/206/109); the same 16-test greedy cover; 11 product unique-catch tests of 100; 40 of 93 dominated; the all-wet grid (23.4 s) has the same 142-mutant set as the 4.7 s collapsed-settled test; 18 survivors in `designed_channel_symmetry` |
| a crate the audit did not use | gate-expander, all 390 mutants | 229 product mutants: 168 caught, 46 missed, 15 unviable, **78.5 %**. **4 of 41 tests (10 %)** hold a unique product catch (5 counting the corpus generator). 18 of 39 catching tests are dominated. Survivors cluster in `apply_automation` (11), `bind_homogeneous_bank` (8), `initial_defaults` (7) and `parse_lane` (6). "About 1 in 8" holds |
| the inventory | recomputed from `data/test-classes.tsv` | **2,208** tests under 0.1 s take 22.5 s (the audit says 2,321 and 23 s); 116 tests of 1 s or more take 830.6 s (the audit says 121 and 833 s). **Macro-generated tests are missing**: `effect_conformance_test!` expands in 8 effect crates, and its test catches 101 of the compressor's 390 caught mutants and 110 of gate-expander's 168, with no unique catch. Low |
| 132 red jobs | stratified random sample of 19 (seed 20260928): 8 pin, 5 hygiene, 4 test-defect, 1 real, 1 infra; each log fetched with `gh api` | **19 of 19 match** the audited step and gate. Row 57 (the export-set order in `abi_layout.rs:437`) could be called a test defect rather than a pin; either way it is not a real defect |
| "no pin exposed an unintended change" | the files changed by each of the 40 local pin-fix commits | where a fix touched product code, it was either a `#[cfg(test)]` module (`10d4aa47`, `78d5d65b`) or a fix for a *different* red in the same run (`daad068d` clippy, `726e1a51` doc word). **Holds** |
| 98 artifact pin values in September | `git log -p` of the pin file since 09-01 | **98 distinct values, 97 commits.** Holds |
| #970, #994, #1015 re-injected at HEAD | `tools/revert.py` on a scratch copy, then the owning crates' CI-feature tests | **#970:** 8 red (7 `collapse_arming` + the `bank_levels` probe). **#994:** 10 red, including the three `randomized_differential_*`; `knee_overflow.rs:185` stays green, as the audit found (`:151` is green by design). **#1015:** exactly the 3 `stationary_subnormal` tests, and no differential. **Holds.** (#966 was not re-run) |
| CI savings | a helper measured 8 full-route runs (09-14..09-26) plus PR #1016's run from the job and step logs | see F9. The median full run is **2,497 s** over 67 full-route runs in 14 days |

## 3. Findings, most severe first

### F1 (high): draft 04 removes the only per-PR guard of #962, and its evidence cannot see the claim

- **What the draft keeps.** Per PR it keeps only `scale.rs:91`'s *constrained* compile.
- **Why that does not guard #962.** `compile.rs:389` refuses on `maximum_nodes` before
  `with_builtin_banks` or any bind. So the constrained compile never reaches the code #962 made
  linear.
- **The draft's other two "guards" are not guards either.**
  - `tools/audit/src/fixture_builtins.rs:1370` only calls `prepare_session_builtins`: no graph
    compile, no bind, no render.
  - `bench graph_validate_65537_tracks` is `#[ignore]` (`tools/bench/src/graph.rs:691`), and
    #1026 deletes the graph benchmark.
- **What I measured.** I re-injected #962 fix 1 into a scratch tree (`Vec::contains` per bank
  member in `with_builtin_banks`) and ran the debug binaries under the lock.

  | test | base | #962 re-injected |
  |---|---:|---:|
  | draft 04's remnant (the constrained compile only) | 16.9 s; re-run 19.3 s | 17.7 s; re-run 16.6 s |
  | `compiles_and_binds_65_537_tracks_with_builtins` | 73.2 s | **673.9 s** |

- **What this shows.**
  - Today the regression overruns `test-debug-a`'s 15-minute budget: the #962 spec names the job
    timeout as the backstop.
  - After draft 04 every PR stays green, and the drop in coverage is invisible.
  - "0 of 80 surviving mutants" is the wrong evidence. `cargo mutants` never plants an
    O(n) → O(n²) change, so the metric cannot see what these tests exist for.
- **#1002's memory claim was never guarded by these tests.** Its spec says "#962's scale gates never
  take this path because their session has no effects". The +24 % RSS regression was measured once,
  in a scratch harness. Nothing in CI guards it: that is a gap, not a cut.
- **Failure scenario.** A refactor of `with_builtin_banks` or a bind helper reintroduces a per-member
  linear scan. Every PR is green. The nightly job either runs without a time bound, and takes about
  20 minutes in release, or does not exist yet. Hosts with thousands of tracks take minutes to
  compile.
- **Amendment to draft 04.** The nightly job must hold an explicit wall-clock budget: release
  compile-and-bind at 65,537 tracks in at most 60 s, where linear is about 17 s and quadratic about
  20 minutes. It belongs in `nightly.yml`'s `release-budgets` job, beside the other "finishes in"
  gates. Add a gate: re-inject #962 fix 1 and show nightly red. The per-PR loss is up to one day of
  detection latency, and the owner rules on it.

### F2 (high): draft 01's outcome deletes accounting completeness, and its own gate 2 cannot pass

- **What I did.** In a scratch tree I dropped one owner row from capi's retained accounting:
  `checked_layout::<crate::Plan>(1)` in `crates/capi/src/runtime/compile.rs`. That is 416 bytes
  under-reported.
- **What caught it:**
  - `resource_lifecycle.rs:2651` went red on the *frozen literal report* (`:2732`,
    `frozen_scratch_report(160_981)`);
  - with that literal removed, it went red on the *mirror*-driven one-below cap ("capi one-below"
    admitted);
  - `:2785`, rewritten as draft 01 proposes (its total taken from the live report, no literal),
    **stayed green**.
- **The contradiction.** Draft 01 replaces both the literal totals and the mirror with live reports.
  A live report shrinks together with the dropped row, so nothing turns red. Gate 2 ("`:2651` still
  goes red") cannot hold with the outcome as written.
- **What a ceiling misses.** A ceiling only sees growth.
  - It never sees an **under-count**: an owner row dropped, or a `size_of` change that the
    accounting forgot.
  - An under-count means admission control accepts a session whose real retained memory exceeds the
    host's configured cap. The one-below cap then admits. On a phone that is an OOM kill, not a
    re-pin.
  - Growth within the headroom goes unseen too. That matters less, because accounted growth is not a
    defect.
- **Failure scenario.** A new retained owner (for example a per-track observation table) is added to
  the plan but not to `fixed_allocation_rows`. The ceiling test is green, the live-report exact test
  is green, and every mobile cap is silently wrong by that owner's size.
- **Amendment.** Keep one independent completeness oracle, and drop only the literals. The oracle
  can be the mirror, or better, an allocator-observed total. `resource_lifecycle.rs` already
  installs a counting allocator (`record_allocation`, `:41`), so it can check that the bytes
  allocated and still live after compile equal the reported retained bytes. Also:
  - item 2 (the Issue-544 validator) is deleted by #1035, so it leaves this draft;
  - the capi mirror is edited by #1024, #1033 and #1035, so this draft lands after them.

### F3 (high): draft 15 removes the enforcement behind 16 filed specs, and drops files from its scope

- **What the per-PR pin is today.** It is the only mechanism that makes "shipped artifact unchanged"
  a checked claim.
  - At least 16 of the 25 filed specs (#1017-#1041) carry a "Shipped artifact" gate: "unchanged",
    or "re-pin with the reason". Examples are #1021 gate 4, #1027 gate 3, #1039 gate 3 and #1041
    gate 2.
  - The pin is also the only link between the bytes that `qualification.yml` gates and the bytes
    that `npm-publish.yml` publishes. `npm-publish.yml` rebuilds and asserts the pin, but runs
    none of the callgraph ratchet, the V8 spill gate, the atomics check or the browser legs.
  - The pin surfaced #1015's doc-only follow-up moving the shipped bytes (`52d59c7c`). That was not a
    defect. It was correct information for an author who believed the change was doc-only.
- **Scope the draft misses:**
  - `scripts/check-ci-path-routing.py` pins the step name "Verify the downloaded artifact against its
    source pin" and requires it before the V8 spill gate (`ARTIFACT_PIN_STEP`, `:334-353`);
  - `scripts/test-ci-path-routing.py`;
  - `scripts/test-sdk-artifact-builder-output-contract.sh`;
  - the V8 benchmark's provenance (`module_matches_pin` in
    `scripts/web-mixing-automation-benchmark.mjs`, and `run-web-mixing-automation-benchmark.sh:92`).
    Under R9 the owner keeps that benchmark as *the* benchmark.
- **Failure scenario.** A cleanup PR claims "artifact unchanged", but a feature unification or
  `cfg(test)` slip leaks into the module. Under draft 15 nothing turns red. The change reaches a
  release PR whose pin is typed from a local build, and fails only at `npm-publish`.
- **Amendment. Adopt R3 only with both of these:**
  - every PR prints `ARTIFACT CHANGED|UNCHANGED` against the merge base's digest in the job summary.
    Issue gates cite that line;
  - the pin comparison still runs on any PR that edits the pin file, which is the release PR.
- The two-build reproducibility check stays as drafted.

### F4 (medium): the mutation-equivalence gates can be vacuous, as the tooling stands

- **Two measured facts:**
  1. **Test packages are not baselined.** `cargo mutants` 27.1 runs the baseline with only the
     *mutated* package. Every mutant then runs all `--test-package` packages. A test in a test
     package that is red without any mutant therefore "catches" every mutant. Evidence:
     - baseline: `cargo test --package=compressor`;
     - each mutant: `--package=compressor --package=effect-compiler --package=graph-compiler
       --package=host-core`.
  2. **opt-level 1 is not CI-equivalent.** `run-mutants.sh`'s `CARGO_PROFILE_DEV_OPT_LEVEL=1` is
     documented as "same semantics as the CI debug profile". But
     `effect-compiler/tests/scalar_state.rs:833` and `:879` **fail at opt-level 1** (run alone, or
     together with the compressor, graph-compiler and host-core tests) **and pass at opt-level 0**.
     - The cause: `validate_effect_state_metadata` compares `&'static EffectDescriptor` with
       `core::ptr::eq` (`crates/effect-package/src/state.rs:536`). That address is not stable
       under optimisation.
     - #1037 deletes that path. Until then it is an optimisation-dependent refusal.
- **Consequence.** My cross-crate symmetry pass reported "18 caught of 18". 6 of those were "caught"
  only by these two tests.
- **Affected gates:** drafts 01, 08, 09 and 14 use `--test-package`.
- **Amendment to `tools/README.md` and each gate.** Before the mutant runs, run the exact test set
  once, unmutated and with the same environment, and require it green. Also report which tests
  fail at opt-level 1.

### F5 (medium): the mono-collapse gap is real but narrower than the headline

- **What I ran.** The 18 compressor `designed_channel_symmetry` survivors, against compressor,
  effect-compiler, host-core and graph-compiler (comma-separated `--test-package`).
- **12 are caught** by tests that CI runs in `test-debug-a`:
  - `effect-compiler/tests/symmetry_designed_words.rs` `each_launch_effect_sees_its_own_designed_words_disagree`;
  - graph-compiler's mono-pool tests;
  - host-core `twin_parameter_spans_keep_the_lane_and_a_lone_half_declines_it`;
  - `bank_levels.rs`'s reduced-mono-console test, for `-> true`.
- **6 survive everything:** the `|| -> &&` mutants at `lib.rs:704-706` and `:714-716`.
  - These are the ramp and rate-ramp comparisons.
  - A lane whose channels differ in only one of `current`, `target`, `step` or `remaining`, mid-ramp,
    would be judged symmetric and collapsed: the #970 class.
- **So the claim "survives 18 planted bugs" is overstated.** The real gap is **partial-field
  asymmetric in-flight ramp state**, and draft 14's compressor slice must generate exactly that.
- **A second cluster, not in the audit.** gate-expander has 8 surviving `bind_homogeneous_bank`
  mutants (bank eligibility).

### F6 (medium): draft 16's rule would have prevented about 5 of its 10 counter-examples

I count the draft's list as 4 scrapes, 2 re-pins, 1 frozen digest and 3 redundant `knee_overflow`
tests.

| counter-example | caught by the rule as drafted? |
|---|---|
| `graph/src/runtime.rs:7770`, `:7937`; `lane/tests/input_chain_elision.rs:939`; `source/src/lib.rs:2882`; `math/tests/f1_fast_db_bounds.rs:1168` | **yes**, by the lint: each is a literal `include_str!` of a `.rs` file (verified with `git grep`) |
| `graph-compiler/tests/track_delay.rs:253` | **no, and rightly.** It holds 2 unique product mutant catches (`mutation-summary.md`), so it is not a counter-example |
| `console-workload/tests/paired_spans.rs` gate 3 | **no.** It is a digest of *rendered output*. The pin clause covers only "a fixture resource byte count, or a digest of a fixture's bytes" |
| `compressor/tests/knee_overflow.rs` (5 tests; 3 redundant) | **no.** They are #994 reproducers and so exempt. `:185` no longer goes red on its revert (re-run confirmed), yet the exemption makes it "permanent" |

- **What works.** The rule is cheap, and the lint discriminates.
- **Amendments to draft 16:**
  - the pin clause covers digests of rendered output and of compiled artifacts;
  - a reproducer is exempt only while its PR records it red on the bug's revert, and a reproducer
    found green on its revert is repaired or deleted;
  - its gate 2 applies to *added or rewritten* tests, not "kept" ones. Kept tests would make it a
    ledger.
- **Amendment to draft 07's lint.** Exclude literal paths that *contain* `/fixtures/`.
  `tools/bench/src/builtins.rs:279,283` `include_bytes!` fixture `.toml` files legitimately.

### F7 (medium): draft 08 removes a leg that nine filed specs use as their class-A gate

- Draft 08 removes `run-wasm-gates.sh`'s `--native` leg.
- `bash scripts/run-wasm-gates.sh passes` is the class-A gate of #1021, #1024, #1027, #1033, #1034,
  #1036, #1037 and #1038, and #1018 runs the script too.
- Without the native leg, those gates no longer compare the native digests.
- **Amendment.** Keep the leg by default, and add `--without-native` for CI's `wasm-guests`, paired
  with the G5 Rust test in `test-release`. `check-ci-path-routing.py` enforces the pairing, as it
  does for `--without-v8-spill` (#1009).

### F8 (medium): draft 09 deletes on sample-based dominance

- The 24.6 s EQ test's dominance was measured on a 1-in-4 mutant sample. The draft's gate adds a
  second shard: still half.
- The same draft records 1005-M5d, a width-only catch that sampling missed.
- **Amendment.** A *deletion* needs mutation equivalence over **all** mutants in the files the test
  exercises (`--file`), not a shard. A *shrink* may use shards.

### F9 (medium): savings re-measured; 520 s holds approximately, 850 s is not measured

The medians are over 8 full-route runs. "Ref" is PR #1016's run.

| item | audit | median | ref | note |
|---|---:|---:|---:|---|
| 02 self-tests (lint + SDK) | ≈165 | 159 | 152 | about 22 s of it (rack, builtins-current, wasm-kernel and wasm-console suites) is deleted anyway by #1026, #1027 and #1039, which leaves **≈137 s** |
| 03 `parameter-metadata` twice | ≈140-150 | 122 | 147 | plus M3 "FMA" about 9 s |
| 04 scale and heavy loops | ≈120 | 114 (4 binaries) | 149 | minus the kept constrained compile (about 17-20 s) gives **≈95-125 s**; `scale.rs` doubled in the #1016 batch, so HEAD is nearer the ref |
| 09 dominated slow tests | 70-90 | debug-b binaries 83 in total | 80 | binary-level upper bound; the graph-compiler lib is 46 s at HEAD (11 s before #1016) |
| 01 native witness build | ≈60 | 62 | 71 | holds |
| 05 scalar-Wasm legs | ≈117 | 88 measurable | 108 | the cross-target share cannot be split from one quiet step |

- **No ruling:** about 430-520 s per full run, about 18-20 % of the 2,497 s median full run.
- **With R2 and R5:** about 580-670 s measured. R4 (draft 11) is unmeasured, and much of the tooling
  it compiles is deleted by #1025-#1028, #1035 and #1039. "About 850 s" should be withdrawn until
  R4 is measured.
- **Coverage.** Every claimed step ran on all 67 of 67 full-route runs. None of them run on
  docs-only runs (40 of 107 successful runs in 14 days).

### Low

- **L1. Scrape replacements.** The draft-07 claims check out, with one exception:
  - **checked out:** `graph` G-2, A-5, 936-4 and 957-1b are red on behavioural tests
    (`crates/graph/tests/MUTATIONS.md:458`, `:549`, `:590`, `:594`, `:646`; G-2 is *green* on the
    scrape);
  - **checked out:** `capi/src/ffi.rs:2516` has no other guard (no Miri in any workflow);
  - **the exception:** `lane/tests/input_chain_elision.rs:939`'s named replacement,
    `kernels/builtins.rs:1749`, counts selections per channel. It does **not** see a runtime branch
    inside the frame loop, which the scrape does. Draft 07 should state that loss, or leave that
    half to a codegen gate.
- **L2. Cannot-fail sample** (D8, D11, D14, D16, the H rows read): confirmed.
  - D16 `rack/tests/console_bank.rs:405`: with the shunt allocated unconditionally in a scratch tree,
    **every** rack, capi, graph-compiler and host-core test stayed green;
  - so D16's named guard (`:557`, an observation test) does not guard it either. Deleting D16
    loses nothing, but the draft's "surviving guard" is wrong for that row.
- **L3. Draft 13's counter half.** No counter false red since 2026-09-10, and the per-test fixes
  landed. Drop it, or keep it as a review sentence: new allocation tests use
  `bench_support::alloc`'s thread-scoped counters.
- **L4. Draft 04's risk statement** omits builtins-compiler prepare above 65,536 tracks.
  `fixture_builtins.rs:1370` still prepares 65,537 tracks per PR in release, against
  `resources.jsonl`.
- **L5. host-core randomness.** `limiter_linked_session.rs` uses seeded SplitMix noise at eight
  seeds. There is still no randomized session generator, so the gap stands.
- **L6. Draft 03's M3 claim holds.** `RUSTFLAGS` replaces `.cargo/config.toml`'s rustflags, so the
  step builds `math` without `lane` on `+fma` alone. That configuration ships nowhere, and the main
  leg already runs M3 with FMA on.

## 4. Reconciliation with the filed issues

| draft | overlaps | decision |
|---|---|---|
| 01 | #1035 deletes the Issue-544 validator (item 2); #1024, #1033 and #1035 edit the capi mirror | file after R2, rewritten per F2, after those three |
| 02 | #1026 deletes `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` and re-points `test-env-vocabulary.sh`; #1027 deletes `test-wasm-kernel-timing.sh`; #1039 deletes `test-wasm-console-benchmark.sh` | file, with those suites dropped from its list, after #1026 |
| 03 | none material | file |
| 04 | #1026 deletes the bench graph row it cites | file with F1's amendments; the owner rules on cadence |
| 05 | #1041's implementation (`4ef7c986`) keeps wasm32 without `simd128` as a named "scalar-wasm CI exception, a separate, pending decision"; #1037 deletes the package and descriptor corpora legs; #1038 edits the neighbouring `backend.rs` arm; footprint R8 lists "the scalar wasm CI build" | **do not file as drafted.** Re-scope to "take #1041's pending decision" (R5): delete the marked exception arm and the legs that remain after #1037, after #1017 runs the portable `max`/`min` on AArch64 |
| 06 | part C: R6 is settled by footprint R9 (#1025-#1028); R7's extended-rate rows by footprint R5 (#1036); H55 and `package_v1_qualification.rs:371` go with #1037; `native_source.rs` rows with #1035 | file parts A and B only, minus rows in code those issues delete |
| 07 | `source/src/native_source.rs` and `native_wave.rs` rows → #1035; `effect-compiler` `migration*.rs` and `effect-package` `state_vectors.rs` rows → #1037 | file, pruned, with F6's lint fix |
| 08 | nine filed specs gate on `run-wasm-gates.sh` | file with F7's amendment |
| 09 | #1036 trims `builtins/tests/response.rs`'s extended rates | file with F8's amendment, after #1036 |
| 10 | the one-shot scripts, the R6 benchmark code and the env rows → #1027; `run-console-benchmark.sh` → #1025 and #1027; the reachability rule → #1022 (merged) and #1027 amendment 3; the validators' builtins-less rows → #1025 and #1039 | **shrink** to its unique remainder: the retired-feature gate rules and the superseded WebDriver harness |
| 11 | #1025-#1028, #1033, #1035 and #1039 delete most `bench` and `audit` subjects | ruling R4, measured only after those land |
| 12 | #1017 | **merge into #1017** (amendment below); do not file |
| 13 | #1021 (merged) enables `builtins`, `parametric-eq`, `host-web`, `host-core`, `effect-compiler` and `protocol` test-support, and adds `check-test-support-ci.py` | **drop**: part 1 is done, and part 2 per L3 |
| 14 | depends on #1021 (now satisfied) | file the first slice (host-core), then the compressor slice per F5 |
| 15 | at least 16 filed specs rely on the pin | ruling R3; file only with F3's amendments |
| 16 | none | file with F6's amendments |

**Amendment text to add to #1017** (from draft 12; the root agent applies it, since this
verification does not edit filed specs):
- Run the lane, math and G5 legs in the **shipping release profile**. LANE-3 is an optimizer fold
  and can hide in a non-LTO build.
- Run `bank_levels.rs`'s `Simd4` pins natively. Re-injecting #966 must turn them red at `Simd4`.
- Add a no-silent-skip gate over the packages the job runs.
- The two `cfg(not(x86))` stubs gain real assertions.
- #1017's named-expected-failure approach supersedes draft 12's "start outside the required
  workflow".

## 5. Drafts to file, in order

1. **16** (amended) and **03**.
2. **02**, after #1026.
3. **04** (amended), after the owner's cadence ruling.
4. **06** (parts A and B, pruned).
5. **07** (pruned).
6. **14**'s host-core slice.
7. **08** and **09**, amended; 09 after #1036.
8. The shrunk **10**.
9. **01** after R2, #1024, #1033 and #1035; **15** after R3; **11** after R4 and the deletions;
   **05** re-scoped after R5 and #1017.

Do not file: **12** (merged into #1017) and **13** (done by #1021).

## 6. Owner rulings

| ruling | status | recommendation |
|---|---|---|
| R1 AArch64 in scope | decided (ARM64 official) → #1017 | none needed |
| R2 exact bytes vs ceilings | open | **ceilings for growth budgets; exactness kept for completeness**, against an independent live oracle (allocator-observed), not literals (F2) |
| R3 artifact pin at publish | open | **yes, conditionally:** base-vs-head digest reported on every PR, the pin enforced on PRs that edit the pin file, the two-build check (F3) |
| R4 tooling tests without fat LTO | open | **defer**; measure after the #1025-#1028, #1035 and #1039 deletions, then adopt for class-T tests only if at least 60 s is saved |
| R5 scalar-Wasm legs | open; #1041 left it as the pending decision | **yes, after #1017 runs the portable `max`/`min` on AArch64.** It is separable from footprint R8's oracle question, because `Backend::Scalar` stays |
| R6 isolated kernel benchmarks | decided by footprint R9 | none needed |
| R7a `dependency_waves` token | open | **remove**, in its own session-schema product issue ("modes production never needs are removed") |
| R7b extended-rate test rows | decided by footprint R5 → #1036 | none needed |
| R8 legacy observation path in host-web | open | **remove** if the SDK and app call only the protected path, in a product issue |
| **new:** #962 guard cadence | open (F1) | **nightly `release-budgets` with a 60 s bound**; accept up to one day of detection latency. Otherwise keep `scale.rs:160` per PR |
