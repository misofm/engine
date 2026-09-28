# One owner per cross-target digest corpus

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 8 and §8, which settles the 2026-09-04
audit's "known conflict" 1). Base `a9414c0c`. Paths starting `../` are relative to the audit's
handoff folder. No ruling needed.

## Problem

Each effect's frozen cross-target corpus, the class-A claim that a rendered block is bit-identical
across widths and targets, is compared against the same pin constants in three places per PR:

1. **Ten per-crate pin compares** in `test-debug-b` (`../data/candidates-dsp-graph.md` item 49):
   - `builtins/tests/determinism.rs:34`;
   - `effect-runtime/tests/determinism.rs:39`;
   - `parametric-eq/tests/determinism.rs:50`;
   - `compressor/tests/cross_target.rs:60`;
   - `true-peak-limiter/tests/determinism.rs:33` (pin half);
   - `multiband-compressor/tests/cross_target_digest.rs:27` (pin half);
   - `gate-expander/tests/determinism.rs:29`;
   - `delay/tests/determinism.rs:27`;
   - `soft-clip/tests/determinism.rs:36`;
   - `transient-shaper/tests/cross_target.rs:69`.

   Nine of the ten read a `MISO_ENGINE_REPIN_*` variable and skip the compare when it is set.
2. **`tools/wasm-gates/tests/g5_native_corpus.rs:21` `g5_native_digests_match_pins`** in
   `test-release`. It renders the same `run_case` at every width against the same constants
   (`tools/wasm-gate-corpus/src/lib.rs:1059-1070`).
3. **`scripts/run-wasm-gates.sh:42`**, `cargo run -p wasm-gates -- --native`, in `wasm-guests`. It is
   the same native report, next to the `simd128` guest legs that compare Wasm against the same pins.

A layout or ordering change re-pins in several places. The mutation pass shows that most of a pin's
unique catches are mutants of its own corpus definition: all 58 unique catches of
`compressor/tests/cross_target.rs:60` are in `src/corpus.rs`, and it has none in product code. They
guard the fixture, not the product.

## Outcome

- **The native owner is the Rust test `g5_native_digests_match_pins`.** It compares every case at
  every width against the shared pins, and, unlike the script's `cargo run -- --native`, it is visible
  to `cargo mutants`. Its Wasm counterpart stays `scripts/run-wasm-gates.sh`'s `simd128` guest leg,
  against the same pin file. When issue 12 adds an AArch64 job, that job runs the G5 test.
- **The ten per-crate compares lose their pin comparison** and their `MISO_ENGINE_REPIN_*` skip.
  Each keeps its **finiteness and non-vacuity** assertions, a separate claim that G5's vacuity checks
  exclude for delegated families.
- **`scripts/run-wasm-gates.sh:42`'s `--native` leg is removed.** It duplicates the G5 test in the
  same PR run. Its evidence line in `wasm-gates.jsonl` comes from the guest legs alone, or the G5
  test writes it.
- **The M3 math corpus** (`crates/math/tests/m3_determinism.rs:142`) keeps its own pins. It is not a
  delegated family.

## Scope

Authorized paths:
- the ten test files above;
- `tools/wasm-gates/tests/g5_native_corpus.rs`, `tools/wasm-gates/src/`, `scripts/run-wasm-gates.sh`;
- `docs/ENGINE_ENV_VOCABULARY.md` (the `REPIN` rows);
- this issue's spec.

## Gates

1. **Every width is still compared.** A scratch mutation that changes one arithmetic operation only
   in the `Simd4` path of one effect makes `g5_native_digests_match_pins` red. So does one only in the
   scalar path. The failure message records the width.
2. **Wasm is still compared.** The same class of mutation in a code path only the Wasm build takes
   makes the `simd128` guest leg red.
3. **Mutation equivalence, compressor.** `../tools/run-mutants.sh compressor 5 10 --test-package
   wasm-gates`, before and after, so the owner is counted. The caught set of product mutants,
   excluding `src/corpus.rs`, is identical. Only the corpus-generator mutants may move from the
   per-crate test to G5.
4. **Historical bugs.** `../tools/revert.py 994` still turns the compressor's randomized
   differentials and the knee reproducers red. `1015` still turns `stationary_subnormal` red.
5. **Finiteness kept.** Each per-crate test still fails when its corpus emits a NaN; seed one in a
   scratch branch.

## Saving and risk

- **Saving:** a few seconds per PR, and about ten fewer re-pin sites per corpus change.
- **Risk:** a digest change is now reported by `test-release`'s G5 test instead of the effect crate's
  debug tests, so the failure is further from the change. The message names the case and width.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Do not remove `run-wasm-gates.sh`'s `--native` leg from the script (finding F7).**
   - "`bash scripts/run-wasm-gates.sh` passes" is the class-A gate of #1021, #1024, #1027, #1033,
     #1034, #1036, #1037 and #1038, and #1018 runs the script too.
   - Removing the leg silently weakens all of them.
   - Instead add `--without-native` for CI's `wasm-guests` only. Make `check-ci-path-routing.py`
     require it to be paired with `g5_native_digests_match_pins` in `test-release`, as it pairs
     `--without-v8-spill` with the artifact-gates spill step (#1009).
2. **AArch64 is official now (#1017).** The single owner, G5, must run on the #1017 job in the
   shipping profile. That job is how "bit-identical on phones" is proven.
3. **The per-crate compares run in debug, and G5 in release.** Dropping the debug compare loses only
   debug-only divergence. A `cfg(debug_assertions)`-dependent arithmetic path is the one plausible
   source. Gate 1 adds a seeded `cfg(debug_assertions)` arithmetic difference in one effect and
   shows the loss is accepted: the per-crate finiteness test stays green and G5, in release, cannot
   see it. State this in the PR.
4. **Gate 3.** Baseline `wasm-gates` unmutated with the same environment before the mutant runs (F4).

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1048-one-digest-owner` from `codex/batch-slim-1` (`b8bea8e1`).
Commits `a7f6fd65` and `a3ddee82`. Excluding this spec: 33 files, +434/-695 lines against the base
(the Rust files alone: +235/-664).

**Changed.**
- **The ten per-crate compares.** Each loses its pin comparison, its width sweep (which only fed
  the pin claim) and its `MISO_ENGINE_REPIN_*_CORPUS` switch. Each keeps its finiteness and
  non-vacuity assertions, and each rewritten test names its claim in its doc comment (the
  AGENTS.md test-value rule on `codex/1052-test-value-rule`). `compressor`'s `no_case_is_vacuous`
  now checks distinctness over the rendered words instead of the pin table, whose distinctness
  G5's `g5_case_digests_are_distinct` already checks.
- **The owner.** `g5_native_digests_match_pins` (`tools/wasm-gates/tests/g5_native_corpus.rs:33`)
  now also holds the three truth-table counts that `wasm_gates --native` reports (min/max
  lowering, `f64` lanes, meter block) to zero, so it covers everything the script's native leg
  covers. On a mismatch it prints the scalar oracle's digest of every case that moved at `f32`, in
  pin-table form (`:73`). That replaces the nine per-crate re-pin printers. A case that moved only
  at a vector width is not printed, because it is a defect, not a re-pin.
- **Amendment 1.** `scripts/run-wasm-gates.sh` keeps its native leg by default and gains
  `--without-native`. The flags can come in any order, and an unknown `--without-*` exits 2.
  `wasm-guests` passes `--without-v8-spill --without-native`, and its `wasm-gates.jsonl` now holds
  only the two guest lines. `check-ci-path-routing.py:420` `check_qualification_native_g5` refuses
  `--without-native` unless `test-release` runs an unconditional, unfiltered
  `cargo test --release -p wasm-gates`, on the same job-level route as `wasm-guests`.
  `test-ci-path-routing.py` has six mutations, each red for this reason: the owner dropped, the
  owner in debug, `-- --skip g5_native_digests_match_pins`, `--lib`, a step `if:`, and a narrower
  job route.
- **Amendment 2.** #1017's AArch64 job is not on this batch (its branch has no workflow change
  yet), so the guard is mechanical. `check_qualification_aarch64_g5` (`:444`) activates when any
  qualification job runs on a GitHub-hosted arm64 label (`ubuntu-*-arm`, or `macos-14` and later
  but not `-large`). It then requires one such job to run the same release G5 test. Two mutation
  tests cover it: `test-debug-b` on `ubuntu-24.04-arm` is refused, and `test-release` on
  `macos-15` passes.
  **Hand-off to #1017:** its AArch64 job must run `cargo test --locked --release -p wasm-gates`, or
  this checker fails. On this host, the ten crates' tests and `wasm-gate-corpus` pass
  `cargo check --target aarch64-unknown-linux-gnu`. `wasm-gates` itself cannot be checked
  cross: wasmtime's C helper needs an aarch64 sysroot, which this host lacks.
- **Docs.** `docs/ENGINE_ENV_VOCABULARY.md` retires the REPIN corpus family and says where
  re-pinning happens now.
- **Scope beyond the listed paths.** All of these are consequential and doc-only or dependency
  hygiene:
  - the `corpus.rs` module docs of eight crates, `parametric-eq/src/lib.rs:3721`,
    `soft-clip/tests/lane_identity.rs` and two `MUTATIONS.md` test-index rows no longer call the
    per-crate test the pin comparer;
  - four crates (`effect-runtime`, `multiband-compressor`, `gate-expander`, `soft-clip`) drop
    their now-unused `sha2` dev-dependency, which removes 4 lines from `Cargo.lock`;
  - `qualification.yml` and both routing scripts, per amendment 1.
- The historical `MUTATIONS.md` rows that name a per-crate compare as RED are dated records and
  are left alone. Those catches now land in G5; gate 3 shows this for the compressor.

**Removed tests and their surviving owner.** `cargo test -- --list` over the affected packages
(dev, the CI features) goes from 718 to 707 tests. 13 lines are removed and 2 are added: the 2
renames.

| removed | surviving owner |
|---|---|
| `builtins` `determinism::every_corpus_case_matches_its_pin_at_every_width` | G5 `builtins/*` cases (all 10) at 3 widths, plus the wasm guests |
| `compressor` `cross_target::the_corpus_matches_its_pins_at_every_width` | G5 `effect/compressor/*` at 3 widths (identical catch set, gate 3) |
| `delay` `determinism::corpus_digests_match_their_pins` | G5 `delay/*`, width 0 only, as before (`W = 1` effect) |
| `effect-runtime` `determinism::the_corpus_matches_its_pins` | G5 `runtime/*` at 3 widths |
| `gate-expander` `determinism::every_case_agrees_at_every_width_and_matches_its_pin` | G5 `effect/gate_expander/*` at 3 widths |
| `parametric-eq` `determinism::the_corpus_digests_match_the_pins_at_every_width` | G5 `effect/parametric_eq/*` at 3 widths |
| `soft-clip` `determinism::every_case_matches_its_pin_at_every_width` | G5 `effect/soft_clip/*` at 3 widths |
| `transient-shaper` `cross_target::the_pinned_digests_hold` | G5, transient-shaper cases at 3 widths; the crate's `every_width_produces_the_same_words` is untouched |
| `true-peak-limiter` `determinism::every_case_has_one_digest_at_every_width` → `every_case_is_finite_and_not_vacuous` | pin and width half: G5 `effect/true_peak_limiter/*`; the finiteness half is kept |
| `multiband-compressor` `cross_target_digest::the_corpus_digests_are_pinned_and_width_independent` → `every_case_is_finite_and_moves` | pin and width half: G5 `multiband/*`; the finiteness half is kept |
| `delay` `print_pins`, `effect-runtime` `print_digests`, `parametric-eq` `print_pins` (all `#[ignore]` re-pin printers) | G5's scalar-oracle report |

**Guarded identically.**
- **Same inputs.** G5's `digest_case` (`tools/wasm-gate-corpus/src/lib.rs:1082`) calls each
  crate's own `run_case` or `case_values` at `f32`, `Simd4` and `Simd8`. It hashes the same words
  the same way (`:1681-1805`) and compares them against the same constants
  (`expected_digest`, `:1057`). `g5_delegated_cases_use_the_owning_crates_pins` asserts that every
  family's case count and pins are covered.
- **Same targets.** Both ran natively on x86-64. Only the profile differs: debug in `test-debug-b`,
  release in `test-release`. That is amendment 3's accepted loss, below.
- **Wasm.** The wasm side is unchanged: the same pins in both guest legs.
- **M3 is untouched.** `crates/math/tests/m3_determinism.rs` keeps its pins and its FMA-build step.

**Spec gates.**
1. **Every width is still compared.** The plant is in the compressor's `applied_gain`:
   `smoothed.add(coef.makeup)` becomes `.sub` only when `L::WIDTH == 4`.
   - `g5_native_digests_match_pins` goes red in dev and in release (release build 6 m 9 s).
   - All 4 compressor cases fail, "`at simd4`" only.
   - The same plant with `L::WIDTH == 1` goes red "`at scalar`", and the report prints the 4
     scalar-oracle pin rows.
2. **Wasm is still compared.** The same `.sub`, under
   `cfg!(all(target_arch = "wasm32", target_feature = "simd128"))`, then
   `run-wasm-gates.sh --without-v8-spill`:

   | leg | mismatches (of 358) |
   |---|---:|
   | native | 0 |
   | wasm scalar | 0 |
   | wasm `simd128` | **12**: the 4 compressor cases × 3 widths; exit 1 |
3. **Mutation equivalence, compressor.**
   - **Command deviation.** cargo-mutants 27.1's `--test-package` *replaces* the tested package
     set; it does not add to it. The spec's literal `--test-package wasm-gates` therefore ran only
     `wasm-gates` tests for every mutant (seen in the mutant logs; that run was aborted). Both runs
     below used `run-mutants.sh compressor 5 10 --test-package compressor,wasm-gates`, on
     `b8bea8e1` and on `a7f6fd65`.
   - **Both runs:** 654 mutants, 513 caught, 96 missed, 43 unviable, 2 timeouts. The mutant keys
     are identical.
   - **The caught set is identical.** That holds for all mutants and for the 390 caught product
     mutants excluding `src/corpus.rs`; none lost, none gained.
   - **The removed test and G5 had the same catch set.** Both caught 209 mutants: 86 product in
     `kernel.rs` and `design.rs`, and 123 in `corpus.rs`. The removed test had no unique catch, and
     G5 catches the same 209 after the change.
   - `no_case_is_vacuous` gains two `corpus.rs` catches, from the distinctness change.
   - **Amendment 4 (F4).** Before the mutant runs, `cargo test --package=compressor
     --package=wasm-gates --no-fail-fast` ran unmutated, with `run-mutants.sh`'s environment. It
     was green on both trees: 110 passed on the base, 109 after.
4. **Historical bugs.**
   - `revert.py 994` turns the compressor's 3 `randomized_differential_*` red, plus 6 knee
     reproducers. The seventh, `an_automated_knee_ramp_through_the_overflow_band_does_not_duck`,
     is the one VERIFY F6 already found green on its revert; this change does not touch it.
   - `revert.py 1015` turns the 3 `stationary_subnormal` tests red.
5. **Finiteness is kept.** A NaN was seeded mid-output in all ten corpora (scratch tree). Each of
   the ten kept tests goes red, with its own non-finite or NaN message.

**Amendment 3 (debug-only divergence), stated for the PR.** The plant multiplies the applied gain's
dB by `1 + f32::EPSILON` under `cfg!(debug_assertions)` only.
- The kept `cross_target` finiteness and non-vacuity tests stay green in debug.
- `g5_native_digests_match_pins` in release stays green: it cannot see the change.
- On the base, the removed debug pin compare was red on all 4 cases at `W=1`.
- The loss is accepted. In the compressor, 7 other debug-pinned render scenarios happen to catch
  this plant; that is not a general guarantee.

**Other gates.**
- `cargo check --workspace --all-targets --all-features`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS=-D warnings cargo doc` for the 11 touched packages: clean.
- `cargo test` for the ten crates plus `wasm-gates` and `math` (CI features): 685 passed, 0
  failed, 23 ignored, in dev and in release. The release build needed no
  `CARGO_PROFILE_RELEASE_PANIC=unwind`.
- `bash scripts/run-wasm-gates.sh`, the default with the V8 leg: native, wasm scalar and wasm
  `simd128` each compare 358 with 0 mismatches and zero counts. The CI form
  `--without-v8-spill --without-native` is green, with 2 evidence lines.
- `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass.
- `check-env-vocabulary.sh` and `test-env-vocabulary.sh`: pass.
- **Policy scripts,** all 60 `scripts/check-*` run with no arguments: 50 pass and 8 only print
  usage.
  - `check-sdk-headless.sh` and `check-web-audioworklet.sh` fail on the AudioWorklet artifact pin:
    expected `476e58ad…`, observed `3f744b03…`. The base `b8bea8e1` builds the same `3f744b03…`
    module byte for byte, so this is the batch's pending re-pin, not this change.
  - `check-sdk-types.sh` needs `sdk/node_modules`, which this host lacks.

**Saving.**
- Serially in debug, the ten binaries ran 7.5 s before and 2.5-3.4 s after, under load average
  70-85.
- `wasm-guests` no longer runs the native report; it still builds the runner.
- Ten pin-compare sites and nine REPIN switches are gone. A re-pin now happens in one place: G5's
  scalar-oracle report.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-28.

**Setup.**
- Verified on a scratch merge of this branch (`e5a4b7e5`) into the batch head
  `codex/batch-slim-1` at `570a79f0`, the boundary re-pin after `c867ec2a`.
- The merge is textually clean.
- There is no semantic conflict with #1021, #1025, #1026 or #1052. Their gates pass on the merge:
  - `check-test-support-ci.py`;
  - `check-bench-policy.sh` and `test-bench-policy.sh`;
  - `check-workspace-policy.sh`, with #1052's exact source-scrape allow-list, and
    `test-workspace-policy.sh`.

**Coverage.** Every (corpus, target, width) pair that a removed pin compared is still compared by
a required `qualification.yml` job on a runner of that target.

| corpus | target, widths | before (base) | after (merge) |
|---|---|---|---|
| builtins, compressor, effect-runtime, gate-expander, multiband, parametric-eq, soft-clip, transient-shaper, true-peak-limiter | x86-64-v3 native: `f32`, `Simd4`, `Simd8` | per-crate test (debug, `test-debug-b`); G5 test (release, `test-release`); script `--native` (release, `wasm-guests`) | G5 test (release, `test-release`, `ubuntu-24.04`) |
| delay | x86-64-v3 native: `f32` | the same three | G5 test |
| all ten | wasm32 without and with `simd128`, three widths | guest legs in `wasm-guests` | unchanged |
| all ten | native AArch64 | none in required CI on this batch | none on this batch; with #1017, `aarch64-release` runs G5 in release, and only #1017's debug compares are forgone (amendment 3) |
| lane, math M3 | all | unchanged | unchanged |

No pair is now covered only outside the required workflow. The only loss is the debug profile on
x86, which amendment 3 accepts.

**Discrimination, re-run by Sol in release.** Every plant was reverted afterwards.
- **`gate-expander` `kernel.rs`, Simd8 only.** The gain is multiplied by `1 + ε` only when
  `L::WIDTH == 8`. `g5_native_digests_match_pins` goes red on cases 120 and 121, "at simd8" only,
  and prints no scalar re-pin rows.
- **`soft-clip` `cubic`, scalar only.** `div(3)` becomes `mul(1/3)` only at `W == 1`. G5 goes red
  on 3 cases "at scalar" and prints their scalar-oracle pin rows.
- **`transient-shaper` `frame`, wasm without `simd128` only.** The wet signal is multiplied by
  `1 + ε`. `run-wasm-gates.sh --without-v8-spill --without-native` exits 1, with 9 wasm-scalar
  mismatches (3 cases at 3 widths).
- **Reverted.** G5 and the CI form of the script are green again.
- **Finiteness.** A NaN seeded in the multiband corpus turns `every_case_is_finite_and_moves` red.

**Test list.** On the merge, `cargo test --workspace -- --list` goes from 2466 to 2455 tests: 13
names removed and 2 added (the renames). Every removed name is in the attempt's table with a
surviving owner.

**Gates on the merge.**
- **Build and lint.** `cargo check --workspace --all-targets --all-features`,
  `clippy -D warnings` and `fmt --check` are clean.
- **Dev tests.** The `test-debug-b` command: 800 passed, 0 failed. `wasm-gates`: 9 passed.
- **Release tests.**
  - The ten crates, with the CI features: 641 passed.
  - The `test-release` step: green.
  - `audit`, `bench` and `console-workload`: 156 passed, with the console digests unchanged.
- **`run-wasm-gates.sh`.**
  - The default run (native, both guests, V8) makes 358 comparisons per leg with 0 mismatches.
  - The artifact `f7bd75ca…` matches its pin.
  - The CI form is green with 2 evidence lines.
  - An unknown `--without-*` flag exits 2.
- **Policy scripts.** All 60 `check-*` scripts and the routing, env-vocabulary, bench-policy and
  workspace-policy suites pass, or only print usage. The one exception is `check-sdk-types.sh`,
  which needs `sdk/node_modules`. It fails the same way on base, so the cause is the environment.

**Findings, by severity.**
1. **Medium: the #1017 hand-off fails as #1017 stands.**
   - Sol merged `401fc362` onto the merge above, resolving the routing scripts' textual conflicts
     as the union of both sides. `check-ci-path-routing.py` then fails: "an AArch64 job
     (aarch64-debug, aarch64-release) must run an unconditional, unfiltered
     `cargo test --release -p wasm-gates`".
   - The cause: #1017 runs G5 inside `scripts/run-aarch64-tests.sh release`, as
     `cargo test ... -- --exact --skip m2_…`. `runs_g5_native_test` refuses any `--`, and it does
     not read scripts.
   - **What #1017 must add:** an unconditional step in `aarch64-release` whose run line is exactly
     `cargo test --locked --release -p wasm-gates --features math/lane`, with no `--` and no
     target selector. It should also drop `-p wasm-gates` from the script's release `gates`, so G5
     does not run twice. With that step added, the checker passes on the trial merge.
   - The alternative is a change to #1048's checker: let `runs_g5_native_test` accept
     `-- --exact --skip X` when no skipped name is a G5 test.
2. **Medium: flag order now bypasses #1009's V8 pairing.**
   - `run-wasm-gates.sh` now takes its flags in any order. `check_qualification_v8_spill` still
     matches the substring `run-wasm-gates.sh --without-v8-spill`.
   - Proof: with `wasm-guests` changed to `--without-native --without-v8-spill` and
     `artifact-gates`' V8 spill command deleted, `check-ci-path-routing.py` passes.
   - Fix: test for the `--without-v8-spill` token in the `run-wasm-gates.sh` command, as
     `check_qualification_native_g5` does for `--without-native`, and add the reordered mutation
     to `test-ci-path-routing.py`.
3. **Low: gaps in the G5 pairing rule.**
   - These are accepted: a step-level or job-level `continue-on-error: true` on the owner, and a
     contrived `... && echo -p wasm-gates`.
   - The implementer's six mutations are refused, and so are three of Sol's: a positional name
     filter, `--test g6_full_corpus_ftz`, and a debug `run: |` block.
4. **Low: stale documentation.**
   - The `digest_*` docs in `tools/wasm-gate-corpus/src/lib.rs` still say "exactly as that
     crate's `tests/determinism.rs` does natively", but most of those tests no longer digest.
   - `crates/delay/tests/MUTATIONS.md` M13 names `corpus_digests_match_their_pins`, reproduced
     with `cargo test -p delay`. That no longer goes red; G5 does.
