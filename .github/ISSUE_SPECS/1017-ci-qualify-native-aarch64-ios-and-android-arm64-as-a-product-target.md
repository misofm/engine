# CI: qualify native AArch64 (iOS and Android arm64) as a product target

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

## Product outcome

The owner ruled on 2026-09-28 that native AArch64 is an official target: fans open and play sessions in iOS and Android apps, which embed the engine through the C ABI (`docs/rulings/engine-footprint-2026-09-28.md`). Today nothing qualifies it. No CI job builds, lints or tests any AArch64 target: `scripts/check-cross-targets.sh:4-5` records that the Android and iOS rows were removed under #378 ("native AArch64 unsupported, no claim"), and `docs/TARGET_MATRIX.md` keeps a deferred-defect register that a revival must reopen. Evidence: `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F4.

## Smallest closable slice

1. Restore the `aarch64-apple-ios` and `aarch64-linux-android` rows in `scripts/check-cross-targets.sh` (`cargo check --all-targets --all-features` for the product crates and `capi`, with clippy `-D warnings`), and add a CI job for them on the required workflow's routing rules.
2. Run the product crates' tests on real AArch64 hardware in CI, on a GitHub-hosted arm64 runner (Linux arm64 and, for Darwin behaviour, macOS arm64), so NEON (`Simd4`) and the AArch64 FPCR path in `lane::fpenv` are exercised.
3. Fix what the new legs expose that is not a known defect: the seven test-only warnings (`host-core/tests/fp_environment.rs:25,26,52,63,81,102`, `lane/tests/fp_env.rs:14`) and tests that assert `Backend::current() == Simd8` (listed in `docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:227,240,297`).
4. The two known defects (the Darwin `memset_pattern16` in SVF `flush()`, and LANE-3 `fmaxnm`/`fminnm`) have their own issues. Until they land, the new legs mark exactly those checks as expected failures by name, never by skipping a whole job.
5. Update `docs/TARGET_MATRIX.md` (support level and the register) and `AGENTS.md` if its target wording needs it.

## Objective gates

- The new CI legs run on every pull request that touches engine code, and `qualification`'s verdict table includes them.
- On AArch64 hardware, the class-A digest suites (console-workload `WORKLOADS`, the effect differentials) produce the same digests as x86 and the browser, apart from the named LANE-3 cases.
- The realtime audits (`audit capi` and the allocation/syscall audits) run on the AArch64 legs, apart from the named Darwin memset case.
- A deliberately inserted `Backend::current() == Simd8` assertion, and a deliberately inserted warning, each turn the new leg red.

## Attempt 1 evidence

Terra, branch `codex/1017-aarch64-ci` (from `codex/batch-slim-1`), commits `92b1def5` (tests),
`401fc362` (CI, scripts, docs) and the evidence commit. Nothing pushed.

### What was built

- **Compile rows.** `scripts/check-cross-targets.sh` restores `aarch64-apple-ios` and
  `aarch64-linux-android`. `aarch64_row <target>` checks the product crates with `--all-targets
  --all-features`, then lints them with clippy `-D warnings`. The product crates are the 26
  workspace crates in `capi`'s normal-dependency closure. They are derived by
  `scripts/lib/product-crates.sh`, which fails if the closure loses `capi`, `lane`, `engine`,
  `host-core`, `graph-compiler` or `parametric-eq`, or drops below 20 crates. `cross-target` installs
  both targets and clippy; its timeout rises from 15 to 25 minutes.
- **Test jobs.** `qualification.yml` gains `aarch64-debug` and `aarch64-release`. Both run on
  `ubuntu-24.04-arm`, are gated on the full route, are in `verdict.needs`, and are expected
  `success` on the full route. Both set `RUSTFLAGS=-D warnings` and have 30-minute timeouts. Both
  run `scripts/run-aarch64-tests.sh`:
  - `debug`: the product crates plus `dsp-reference`, `conformance` and `target-smoke`, all
    targets, with the test-support features test-debug-a/b name. `target-smoke` pins `Simd4` on
    AArch64. This leg runs the four-lane banks natively, `bank_levels.rs`'s `Simd4` pins, and the
    FPCR arms of `lane::fpenv`'s tests.
  - `release`: `lane`, `math` and `wasm-gates` in the shipping release profile (G1-G6, M1-M3, the
    G5 corpus, G6 under `FPCR.FZ`); `console-workload` (the console digests); and `audit capi`
    (record validated), then `audit delay`, `compressor` and `parametric-eq` (100,000 blocks each)
    and `audit gate-expander` (zero violations, and `bank_available` true).
  - **Runner choice: Linux arm64 only.** It runs every AArch64-specific path: NEON, FPCR, and the
    fold behind LANE-3. The one Darwin defect is a codegen fact of the Apple targets, so it is
    checked in the iOS release assembly on the x86 `cross-target` job. A macOS arm64 runner would
    run `aarch64-apple-darwin` (an M1 baseline), not iOS, and no realtime audit sees a libc call
    that is not an allocation, lock or syscall. `docs/TARGET_MATRIX.md` records the reasoning.
- **No silent skips.** Both modes first scan `crates/`, `tools/console-workload`,
  `tools/wasm-gates` and `tools/wasm-gate-corpus` for a test that returns early on a SIMD width:
  `if Backend::current() != Backend::SimdN { return }`, or `let Some(..) = ..._w8(..)/Simd8/
  BankWidth::Eight.. else { return }`. The scan found 15 on the base tree, in six files, and finds
  none now.
- **Expected failures, by name.** In `run-aarch64-tests.sh` each row names its issue. A row is
  skipped by `--exact --skip` in its leg's main run, then run alone, where it must fail as that one
  test. If it passes, the leg fails and says to delete the row. `ios-asm-memset-pattern16` in
  `check-cross-targets.sh` counts `bl _memset_pattern16` in `parametric-eq`'s and `builtins`' iOS
  release assembly; a count of zero fails the check. No job is skipped.
- **Routing contract.** `check-ci-path-routing.py` pins the following:
  - the two jobs' runner, full-route `if:`, script line and `"$full_expected"` verdict rows;
  - `cross-target`'s script and both installed targets;
  - both `aarch64_row` lines, and the check and clippy flags on each command.

  `test-ci-path-routing.py` proves 15 new mutations red (below).
- **Docs.**
  - `docs/TARGET_MATRIX.md`: support level, evidence, runner choice, and the register renamed to
    "Known AArch64 defects (#1017)". The same #378 wording is updated in
    `CONTROL_PROTOCOL_CONFORMANCE.md`, `EFFECT_INTERCHANGE_QUALIFICATION_V1.md` and
    `check-effect-interchange-targets.sh`.
  - `AGENTS.md` is unchanged. It already names iOS, Android and the four-lane NEON banks, and
    says nothing is unsupported.

### Local AArch64 execution (qemu-user, not hardware)

No arm64 host was available. For this attempt, `aarch64-unknown-linux-gnu` was added to 1.97.1;
it is removed again at the end. The scratch folder holds:

- Ubuntu's `qemu-user-static` 8.2.2 and the glibc 2.39 arm64 cross sysroot, extracted rather than
  installed;
- clang 18 with `rust-lld` as the linker.

Tests that re-execute their own binary fail under qemu-user without binfmt_misc. These are
compressor `conformance`'s allocation child, host-core `scalar_point_endpoint`'s two children and
every `audit` subject. Their children were run directly and pass, so they are emulation artifacts.

**What the legs exposed on the base tree** (231 debug test binaries, then the release gates):

| found | where | resolution |
|---|---|---|
| 7 test-only warnings | `lane/tests/fp_env.rs:14`, `host-core/tests/fp_environment.rs:25,26,52,63,81,102` | FPCR arms, not `cfg` deletion. `lane`'s fp_env adds hostile-word, unwind, flush and attestation tests on FPCR (FZ, DN, RMode). `host-core`'s FTZ render test runs on both control words. The `cfg(not(x86))` G6 stubs (`lane/tests/g6_ftz_inert.rs`, `wasm-gates/tests/g6_full_corpus_ftz.rs`) assert under `FPCR.FZ` too. |
| clippy `-D warnings` red on both targets | the same `unused import: self` | green after the fix |
| eight-lane counts read as facts | console-workload `automation`, `chain_shape` ×2, `lib`; host-core `source_in_place`, `input_liveness_console`; graph-compiler `misaligned_lane_sets_decline_the_merge` | derived from `Backend::current().width()`; values unchanged on x86 |
| W8 banks that cannot bind on `Simd4` | true-peak-limiter lib ×6 and `allocation`; effect-compiler `bank_state` (silent return); gate `injected_nonfinite_gain…` (silent return) | these now run at the native width; the last two are width-generic |
| width-dependent pins | graph_fixture corpus; graph-compiler issue-037 transcript; EQ `bank` cliff scenario | graph_fixture compiles at `Simd8` explicitly. The issue-037 transcript gets a `Simd4` row, `0x8a04_4e52_b88e_4e5f`: all 100 layouts' `pcm_hash` are identical on x86 and AArch64, and only the structural counts differ. The cliff scenario's overflow leg planted lane 0 of every bank, which is one track at W8 and two at W4. It now plants tracks 0 and 4, so W4 and W8 pin the same digest, `9e6886cf…`/`171406a7…`. That re-records two bank rows; the old planting still renders the old rows, and no render moved. |
| silent passes on AArch64 | gate-expander `identity`, `oracle`, `state` ×4 (`prepare_bank_w8(..) else { return }`), compressor MQ-2, W4 gate test (silent on x86) | `expect(..)` plus `#[ignore = "…"]` on the other width, with the reason |
| W8-only premises | host-core `symmetry_witness` scalar-arm fixture (a W4 bank takes a cohort of two); capi `resource_lifecycle` exact W8 byte totals | ignored off x86 with the reason; #1060 owns the totals |
| gate audit bank absent | `audit gate-expander` reported `bank_available:false` on AArch64 | the audit binds at the native width; W4 on AArch64 with 0 violations, W8 unchanged on x86 |
| **LANE-3 (#1019)** | release `math` M2: `m2_exp2_lane_identity` (the scalar digest `69c5e8f5…` is not the pin `4686f7e4…`) and `m2_log2_lane_identity` (width 4 ≠ scalar at input 870, a NaN: `0x43c06e2d` vs `0xc2fc0000`) | Named expected failures. Both pass in debug. The M2 test binary holds 21 `fmaxnm`/`fminnm`, in `exp2_lane`, `log2_lane` and `fast_gain_from_db`, scalar and vector. **G1 passes on AArch64 under 1.97.1**, so the register's "G1 is red there" no longer holds. |
| **AArch64 NaN encodings** (new) | compressor `kernel::settled_body_tests::scenario_{981,983,985,995,1006}` and EQ `bank` `admitted_blocks_render_the_base_bits_without_selects`, in debug too | Cause: an arithmetic NaN is `0x7FC00000` on AArch64 and `0xFFC00000` on x86, and propagation priority differs. With every NaN folded as one word, all six compressor scenarios and all three EQ legs give identical digests on both architectures. Named expected failures attributed to #1019, whose gate 1 requires compressor NaN-payload identity on this leg. **Root to rule** whether #1019 owns the EQ case or a successor does. |
| **Darwin memset (#1018)** | iOS release asm: `parametric-eq` 151, `builtins` 376 `bl _memset_pattern16`, in `process_bank`, `process_mono`, `InputStage::process` and others | Named expected failure `ios-asm-memset-pattern16` |

**Class-A cross-checks (x86 vs AArch64 under qemu).** All of the following match:

- the G5 native corpus (7 tests) against the x86 and `simd128` pins;
- console-workload, all 7 binaries, digests included;
- `audit capi`'s 100,000-call C-ABI `pcm_digest` `ff6cdcb96cdcdad5`;
- the issue-037 100-layout PCM hashes;
- G6 with `FPCR.FZ`: the unguarded control arm diverges, and the guarded arm is on the pins.

**Draft 12 gates.**

- `Simd4` pins: `bank_levels.rs`'s `Simd4` pins run and pass natively.
- #966 re-injection: `revert.py … 966 apply` in a scratch tree turns all 8 non-randomized
  `bank_levels` tests red at `Simd4` ("… at Simd4: bind" … `graph.scheduler.layout`).
- `Simd4` identity: `target-smoke` asserts `Simd4`.

### Mutations (each shown red, then restored)

1. **Planted `Backend::current() == Simd8` assertion**, in `lane/tests/fp_env.rs`
   `attestation_passes_on_this_thread`. On x86, `cargo test -p lane --test fp_env` passes, 8
   passed. The debug leg's command, run for the aarch64 target with `--test fp_env`, fails:
   `left: Simd4 right: Simd8`.
2. **Planted warning**, `#[cfg(target_arch = "aarch64")] fn planted_unused_on_aarch64() {}` in
   `lane/src/lib.rs`. The x86 clippy `-D warnings` passes. The cross-target Android clippy row
   fails with `function planted_unused_on_aarch64 is never used`. The AArch64 test leg under
   `RUSTFLAGS=-D warnings` fails to compile `lane`.
3. **Deleted compile row.** `check-ci-path-routing.py --root <copy>` fails when
   `aarch64_row aarch64-apple-ios` is deleted, when `aarch64_row aarch64-linux-android` is deleted,
   and when the row loses `-- -D warnings`. `test-ci-path-routing.py` adds 15 such mutations, all
   red, and passes:
   - each row deleted or commented out;
   - each target dropped from the install line;
   - clippy's `-D warnings` dropped;
   - clippy turned into check;
   - check losing `--all-targets`;
   - each job's script replaced;
   - each job's verdict expectation changed;
   - the arm runner swapped;
   - the release job's route gate changed.

### Local results

- **x86, no regression.**
  - Touched crates' debug tests: 870 passed, 0 failed, 25 ignored.
  - Release lane, math and wasm-gates plus console-workload: 177 passed, 0 failed.
  - Workspace clippy `-D warnings`, `cargo fmt --check` and `cargo doc` with `-D warnings`: pass.
  - The lint job's hermetic policy checks, `check-effect-interchange-qualification.sh` and
    `test-effect-interchange-policy.sh`: pass.
  - `check-cross-targets.sh`: PASS in 3m25s warm, 5m16s cold, at 32 cores.
- **actionlint 1.7.7** (shellcheck not installed) on `qualification.yml`: no findings.
- **AArch64 release leg** (`run-aarch64-tests.sh release` under qemu):
  - main run: pass;
  - both M2 expected failures: confirmed;
  - console-workload: pass;
  - audits: pass when run as their subjects. The script's own audit call re-executes itself,
    which is the qemu artifact above.
- **AArch64 debug leg.**
  - Base tree: all 231 test binaries ran in parallel; the table above classifies the failures.
  - After the fixes, the 20 binaries this attempt changed were re-run: 202 passed, 0 failed,
    11 ignored. Nine of the ignores are this attempt's named ignores and two are pre-existing
    descriptive ignores. The W4 gate binding test ran and passed.
  - The leg's expected-failure reruns were exercised for both selector kinds: `--lib` with
    compressor `scenario_981` and `--test bank` with the EQ select test. Each fails as exactly
    one test.
  - The script's full serial debug run under qemu, with the three re-exec tests also skipped for
    emulation only, compiled the whole leg with `-D warnings` and passed. It printed `aarch64 debug
    leg: PASS (26 product crates, 6 expected failures)`. The main run covered 232 test binaries:
    1,981 passed, 0 failed, 45 ignored. All six expected failures failed as themselves. Wall time
    was 150 minutes under emulation.

### Only the first CI run can verify

- The `ubuntu-24.04-arm` image: rustup is installed only if it is absent, before the cache step.
- Real-hardware results. qemu is IEEE-exact and models ARM NaN rules, but nothing ran on
  hardware. In particular: the `Simd4` issue-037 transcript and the re-recorded cliff digests;
  the NaN-encoding expected failures failing there too; LANE-3's M2 failures (a codegen fact, so
  they should reproduce); the four tests that re-execute themselves; and the audits' re-exec path.
- Wall times and cache behaviour.
- The strace-based trace audits (`trace-*.sh`) were **not** added to the AArch64 legs, because
  they could not be validated under qemu. The legs run the in-process allocation and syscall
  audits. The M1/F1 exhaustive sweeps are not on AArch64 either.

### CI cost estimate

These figures scale x86 warm step times from main runs `36097627481` and `36086390860` to a
four-vCPU arm64 runner.

| job | warm | cold (first run on `main`) |
|---|---|---|
| `aarch64-debug` | about 6-9 min | about 15-20 min |
| `aarch64-release` | about 6-9 min | about 15-20 min (fat-LTO `wasmtime` and `audit`) |
| `cross-target`, extra | about +2-4 min, since the iOS assembly rebuilds two crates every run | about +5-8 min |

That adds about 15-20 runner-minutes per full-route PR warm, and about 35-45 cold. The jobs run
in parallel, so the wall-clock critical path grows by about 0-2 minutes: the longest existing
jobs, `audit-native` and `wasm-guests`, take about 6-7 minutes warm. Queueing for arm64 runners
is not included.

### Open for Sol and root

1. Rule on the NaN-encoding expected failures: #1019's scope, or a successor issue.
2. The #1018 and #1019 specs point at "the deferred-defect register", which is now "Known AArch64
   defects (#1017)".
3. #1019's spec, and the register this replaces, say G1 is red on AArch64. Under 1.97.1 it is M2
   that is red, and G1 is green.
4. Not done here: x86-64 Android emulator and iOS simulator builds (the AVX2/FMA pin), device and
   simulator linking, and NEON performance.

## Sol verdict, attempt 1

**FAIL.** No x86 test is weakened, and no product code is target-specific. Under emulation the two
AArch64 legs pass on the batch head, and on the trial merge with #1048 as well. Attempt 2 must
address one defect and two required changes:

- **The defect.** The iOS memset scan sees only 527 of 3,490 `bl _memset_pattern16` calls in
  the product crates' iOS release assembly, and nearly all of those calls sit in render
  functions. So once #1018 lands it would report the Darwin
  defect fixed while most of it remains, and the register misstates the defect's scope.
- **The #1048 guard.** As it stands, this branch turns `lint` red once #1048 lands, and #1048
  lands first.
- **Expected-failure reasons.** The expected-failure rows accept a failure for any reason.

The spec has no Amendments section. I judged the branch against its body, the rulings of
2026-09-28 and the batch `AGENTS.md`, test-value rule included.

### Merge onto `codex/batch-slim-1` (`92ef396f`)

- **Textual merge: clean.** `7084961c` merged into a scratch detached worktree. Nine files
  auto-merged: `qualification.yml`, `check-cross-targets.sh`, `graph-compiler/src/lib.rs`,
  `capi/tests/resource_lifecycle.rs`, the four host-core tests and `console-workload/src/lib.rs`.
- **Semantic conflicts: none** with #1024, #1026, #1021, #1042, #1052 or #1056. The merge
  compiles, lints, passes fmt and passes every x86 test (below).
  - `check-test-support-ci.py` (#1021) passes.
  - `scripts/lib/product-crates.sh` derives 26 crates on the batch, since #1056 removed nothing
    from capi's closure.

**#1048 (`e5a4b7e5`, unmerged), trial merge on top.**

- **Conflicts.** There are two textual conflicts, in `check-ci-path-routing.py` and
  `test-ci-path-routing.py`. Both are purely additive, and I kept both sides.
- **The checker fails.** `check-ci-path-routing.py` then fails with: "an AArch64 job
  (aarch64-debug, aarch64-release) must run an unconditional, unfiltered
  `cargo test --release -p wasm-gates` (g5_native_digests_match_pins)". The baseline of
  `test-ci-path-routing.py` fails the same way.
- **Why.** The leg does run G5, and it skips only math's two M2 tests. But it runs G5 inside
  `run-aarch64-tests.sh release`. #1048's `runs_g5_native_test` reads only YAML `run:` lines,
  and it refuses any command that contains `--`.
- **The proposed fix works.** This is #1048's verifier's fix, and I confirmed it on the trial
  merge:
  - Add an unconditional step to `aarch64-release` whose run line is exactly
    `cargo test --locked --release -p wasm-gates --features math/lane`. The checker then passes.
    `--features math/lane` resolves with `-p wasm-gates` alone.
  - Drop `-p wasm-gates` from the script's release `gates`, so G5 does not run twice. The M2
    expected failures live in `math`, so the new step needs no `--skip`.
  - Keep the step before or after the script step. #1017's own `check_qualification_aarch64`
    still finds its script line.
- **G5 on AArch64 with #1048.** Under qemu, on the trial merge, the release leg passes G5's seven
  tests, `g5_native_digests_match_pins` included, with #1048's delegated effect-family pins.
  LANE-3 does not reach those corpora, so no new expected failure is needed.
- **Nothing is required of #1048's branch.**

### Findings, by severity

1. **HIGH (FAIL): `ios-asm-memset-pattern16` scans 2 of the 9 affected crates, so it cannot tell a
   fixed defect from a partly fixed one.**

   I emitted every product crate's `aarch64-apple-ios` release assembly with the script's own
   command, `cargo rustc --release --target aarch64-apple-ios -p <crate> --lib -- --emit asm`.
   Nine crates contain `bl _memset_pattern16`:

   | crate | calls |
   |---|---|
   | `multiband-compressor` | 1,132 |
   | `compressor` | 970 |
   | `transient-shaper` | 534 |
   | `builtins` | 376 |
   | `gate-expander` | 181 |
   | `parametric-eq` | 151 |
   | `true-peak-limiter` | 104 |
   | `soft-clip` | 22 |
   | `graph` | 20 |
   | **total** | **3,490** |

   The scan counts only `parametric-eq` and `builtins`, which together hold 527. The rest sit in
   render functions:

   - `compressor::kernel::process_block`, `process_block_mono`, `ramping_main_scalar` and
     `settled_sidechain` (at `f32x4`);
   - `PreparedMultibandCompressorBank::process_bank`;
   - `transient_shaper::Shaper::process_block`;
   - `PreparedGate::process_bank`;
   - `LimiterCore::process_block` and `process_bank_inner`;
   - `soft_clip::Channel::process`;
   - `graph::runtime::bank_meter_pass` and `bank_sample_peak`.

   **The cause is not the SVF flush alone.** The stored patterns are `lane::FLUSH_EPS` (`1e-20`,
   `0x1e3ce508`) plus other splat constants: `1.0`, `0.5`, `2.0`, `1e-8`, `f32::MIN_POSITIVE`
   and others.

   **The consequence.** #1018's gate covers "every crate that uses `svf_step`", which does not
   include compressor, transient-shaper, gate-expander, true-peak-limiter, soft-clip or
   `graph`. `multiband-compressor` is in #1018's scope, but this scan does not cover it. Once
   #1018 clears EQ and builtins, this scan reports "now passes: remove it". At that point about
   2,000 calls remain in the `f32x4` and scalar instantiations an iPhone runs.

   **The register is also wrong.** `docs/TARGET_MATRIX.md` places the defect "inside the EQ and
   builtin render kernels" and counts those two crates only.

   **Required.**
   - Scan every product crate, reusing `product_crates`, and keep one expected-failure row per
     crate that names its owning issue. A partial fix then fails the right row, and a new crate
     with calls fails for want of a row.
   - Correct the register.
   - Ask root to widen #1018 to the stored-splat shape in every kernel, or to open a successor
     issue.
   - Five crates could not be scanned this way without Xcode, because `cargo rustc` links their
     `cdylib`/`staticlib`: `capi`, `effect-package`, `effect-compiler`, `graph-compiler` and
     `host-core`. The scan must emit assembly without linking for them, or state that they are
     excluded and why.

2. **REQUIRED (#1048 lands first): the `aarch64-release` G5 step.** See "Merge" above: add the
   YAML step and drop `-p wasm-gates` from the script's `gates`.

3. **MEDIUM: the expected-failure rows do not discriminate the failure's reason.**
   - (a) **A row that starts passing fails the leg.** I showed this for a planted passing row
     (`randomized_differential_f32`) and for a missing test name. Both end with "now passes:
     delete its row".
   - (b) **A different failure reason is accepted.** I replaced `scenario_981`'s body with
     `panic!("PLANTED: an unrelated regression")`, and `expect_failure` still printed
     "expected failure (#1065)" with exit 0. The check requires only one `FAILED` line and
     `0 passed; 1 failed`. A real AArch64 regression in those six pins, or in M2, is therefore
     invisible. Fix: have each row carry the failure it expects, such as the AArch64 digest the
     test prints (`scenario 981 digest <hex>`) or the assertion's `left:` value, and grep for it.
   - (c) **An unrelated failure in the same package still fails the main run.** A planted
     `assert!(false)` in compressor's `randomized_differential_f32` gave "FAILED. 12 passed;
     1 failed" with exit 101. The unplanted control gave "13 passed" with exit 0.

   **Should the six NaN pins also be expected failures in the release leg?** No. The release leg
   runs only `lane`, `math`, `wasm-gates` and `console-workload`, so it never runs compressor's
   lib tests or EQ's `bank` test. They would need rows only if the leg ever ran those packages in
   release. #1065's "in debug and release" comes from ad-hoc runs, not from this leg.

4. **LOW: the skip names apply to every test binary in the run.** `--exact --skip <name>` goes
   to every binary in the debug leg's single `cargo test`. Each of the six names matches exactly
   one test in the workspace today, but a future test with the same name elsewhere would be
   skipped silently. Consider failing when a skip name matches more than one listed test.

5. **LOW: the vacuity guards have gaps.**
   - `check-ci-path-routing.py` pins each job's exact `run:` line, so appending `|| true` is
     caught.
   - `continue-on-error: true`, at either job or step level, passes the routing checker and
     `check-test-support-ci.py`. No check in the workflow refuses it for any job. The gap
     predates #1017, and I did not verify how Actions reports such a job's result to the
     verdict's `needs`, so a successor issue should decide it.
   - The no-silent-skip scan catches the planted `if Backend::current() != Backend::Simd8 { return; }`
     and `let Some(..) = prepare_bank_w8(..) else { eprintln!(..); return; }`.
   - It misses `if !matches!(Backend::current(), Backend::Simd8) { return; }`, which is
     acceptable for a heuristic lint.
   - Zero tests or an empty package list cannot pass: `product_crates` requires at least 20
     crates and six named ones, and a missing expected-failure name fails.

6. **LOW: W8-only gate-expander claims are not checked at W4.** Six gate-expander tests are
   ignored off x86 with a correct reason, because a W8 bank cannot bind on `Simd4`:
   - identity `scalar_and_w8_…`;
   - oracle `…_w8`;
   - state `malformed_final_right_word…`, `scalar_and_bank_recovery…`,
     `scalar_and_bank_state_payloads_interchange…` and `bank_restore_of_one_track…`.

   On AArch64 only three tests cover these claims: the lib tests
   `internal_w4_pcm_and_serialized_continuation_are_bit_exact` and
   `injected_nonfinite_gain_has_scalar_parity_at_the_native_width`, plus the W4 binding smoke.
   Payload interchange, restore isolation and malformed-word rejection are therefore unchecked at
   the width phones run. The other ignores are sound with a named owner:
   - capi byte totals (#1060);
   - MQ-2 (#1027);
   - symmetry-witness per-node arm, which is width-independent scalar code that x86 runs.

   Make the gate tests width-generic, as the lib test now is, in a successor issue.

7. **LOW, unverifiable here.** The tests that re-execute their own binary need real arm64:
   compressor `conformance`'s allocation child, and the audits' dispatcher. Under qemu the child
   passes when run directly. The Simd4 issue-037 transcript and the re-recorded cliff rows were
   also recorded under qemu only. The first CI run must confirm them on hardware.

### Adversarial checks

1. **No weakened x86 test.**
   - I diffed x86 `--list` and `--list --ignored` for `--workspace --all-targets
     --all-features`, before and after the merge.
     - Tests: 2,282 before and after. Six renames and no removals: `fp_environment::x86::*` and
       `g6_full_corpus_ftz::x86::*` became `pinned::*`, and two tests were renamed from `w8` to
       `native_width`.
     - Ignored: 39 before, 40 after. The one added is gate `w4_binding_…`, which used to return
       early on x86 and report a pass.
   - I read every changed test.
     - On x86, `Backend::current().width()` is 8. So the derived counts reproduce the old
       literals exactly: `[8,48]`, `[48,48,48,40]`, `quiet[1]=8`, `[3,9]`, `redirects=4`,
       `collapsed=BLOCKS`. So do the `native_bank()` and `for_backend` widths (W8), the
       `silent_fixed_point` tone (index arithmetic identical at 8 lanes) and the issue-037 pin
       (`0xe095_f3ad_a9cc_cf46`).
     - `fp_control_bits` is `& MXCSR_CONTROL_MASK` (`0xFFC0`), the old `MXCSR_CONTROL_BITS`.
     - `read/write_fp_control_word` are `read/write_mxcsr`.
     - Four changes are stricter on x86. Every former `let Some(..) = prepare_bank_w8(..) else
       { return }` is now `expect(..)`. `input_liveness_console` went from `engaged > 0` to
       `== 2`. MQ-2 and the effect-compiler bank test now assert rather than return.
   - Two changes on x86 are not weaker, but I name them:
     - The TPL `bank_binding_validates…` malformed member moves from lane 5 to lane 7.
     - The EQ cliff scenario now plants tracks 0 and 4 in the one W8 bank. This re-records two
       digests. No product code changed, so no render moved.

2. **Expected failures:** see finding 3.
   - All six debug rows and both release rows fail alone under qemu, as the named test.
   - The debug main run, with the script's exact skip arguments, covered 232 binaries: 1,904
     passed, 45 ignored and 6 filtered. The one failure is the qemu re-exec artifact of finding 7.

3. **CI wiring.**
   - Both jobs sit in `verdict.needs`, in its `*_RESULT` env and in the table as
     `"$full_expected"`.
   - Both are gated on `needs.route.outputs.route == 'full'`, with no `paths:` filter.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass on the merge.
   - actionlint 1.7.7 reports nothing. shellcheck is not installed.

4. **Cross targets.**
   - `check-cross-targets.sh` passes on the merge: 2m41s cold, both clippy rows at `-D warnings`.
   - On the base, clippy `-D warnings` for `aarch64-linux-android` fails in two places:
     - `lane/tests/fp_env.rs:14` (unused `self`);
     - `host-core/tests/fp_environment.rs:25,26,52,63,81,102`, the pre-existing `host-core`
       failure #1056's verifier noted.
   - The branch fixes both with FPCR arms, not with `cfg` deletions.
   - A planted `#[cfg(target_arch = "aarch64")] fn` in `session`:
     - fails the Android clippy row ("never used");
     - leaves x86 clippy green.
   - A planted `assert_eq!(Backend::current(), Simd8)` in `lane/tests/fp_env.rs`:
     - fails on aarch64 with `left: Simd4, right: Simd8`;
     - leaves x86 green.

5. **The iOS memset scan.** It counts 151 and 376 calls. On a zero count it fails with "now
   passes: remove it": I pointed its crate list at `delay` and `rack` to show this. It passes on
   any nonzero count: `compressor` alone gave 970. It has no row list, so "no entry" is not a
   state it can be in. Finding 1 covers its scope.

6. **No target-specific product code.** Every added `cfg(target_arch)` is in a `tests/` file.
   The non-test files that changed use `Backend::current()` at run time, or name `Backend::Simd8`
   without a `cfg`:
   - the `src/lib.rs` hunks, all inside `#[cfg(test)] mod tests`;
   - the `graph_fixture` bin;
   - `tools/audit/src/gate_expander.rs`.

7. **Gates on the merge (x86).**
   - `cargo check --workspace --all-targets --all-features`: pass.
   - clippy `-D warnings`: pass.
   - `fmt --check`: pass.
   - `test-debug-a`'s command: 1,444 passed, 0 failed.
   - `test-debug-b`'s: 807 passed.
   - `test-release`'s: 115 passed.
   - `cargo test --release -p audit -p bench -p console-workload`: 156 passed. This includes the
     console digests. #1017 changes no console digest constant.
   - `scripts/run-wasm-gates.sh`: pass. That covers native, wasm scalar and wasm simd128 (142
     cases, 0 mismatches) and the V8 EQ spill gate, with the AudioWorklet module on its pin.
   - Every `check-*.sh` and argument-free `check-*.py` passes, with three exceptions, none caused
     by the branch:
     - `check-capi-abi.sh` and `check-graph-determinism.sh` hard-code `target/`, so they failed
       under my `CARGO_TARGET_DIR`. Rerun with `target/` linked, they pass.
     - `check-sdk-types.sh` needs `sdk/node_modules`.
     - The seven `.py` scripts that exit with usage need arguments.

8. **AArch64 under qemu-user 8.2.2.**
   - Debug leg, complete (above).
   - Release leg on the #1048 trial merge:
     - `lane`/`math`/`wasm-gates`: pass.
     - M2 rows: fail as named.
     - `console-workload`: pass.
   - Audits, as subjects:
     - `capi`: 100,000 calls, `pcm_digest` `ff6cdcb96cdcdad5`, 0 violations.
     - `delay`, `compressor`, `parametric-eq`: 0 violations.
     - `gate-expander`: `bank_width` 4, `bank_available` true, 0 violations.
   - CI cost and hardware wall times are not verifiable here.

### Test value

For each new or rewritten test: which plausible defect turns it red that no existing test catches?

- **`lane` `fp_env::aarch64::*` (4 tests).** A `CanonicalFpEnv` that fails to install FPCR 0,
  or fails to restore a caller's FZ/DN/RMode word, including across an unwind. So does an
  `in_canonical_fp_environment` or attestation that misreads FPCR. Nothing else runs the FPCR
  path.
- **`g6_ftz_inert`, `host-core` `fp_environment::pinned::*` and `wasm-gates`
  `g6_full_corpus_ftz::pinned::*` on AArch64.** The D8 flush law, `render_planar`'s pin or the
  G5 corpus going off-pin under a caller's `FPCR.FZ` on phones. On x86 the claims are unchanged.
- **gate `injected_nonfinite_gain_…_native_width`.** A NaN gain in one NEON lane leaking into
  its siblings or diverging from the scalar peer. It previously ran only at W8.
- **TPL cohort tests and `allocation`, and effect-compiler
  `production_soft_clip_native_width_…`.** A W4 uniform-cohort body diverging from the per-lane
  path, a W4 bank render allocating, or a W4 member restore moving a sibling. None was reachable
  at W4 before.
- **EQ cliff, issue-037 `Simd4` row and the console/host-core derived counts.** The W4 fault
  reset, layout, counters or cohort formation diverging from the W8 plan's. These pins are
  pre-existing, and #1017 only makes them width-correct.
- **The audit gate subject.** The gate audit covers a bank on AArch64 at all: before this
  change, `bank_available` was false there.
