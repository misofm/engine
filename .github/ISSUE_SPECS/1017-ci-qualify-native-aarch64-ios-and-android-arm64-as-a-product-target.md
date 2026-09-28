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
