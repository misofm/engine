# Keep the whole-plan scalar backend as a test-only oracle, out of shipped builds

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 3): the scalar (one-track-at-a-time, unbanked) renderer stays as the **test-only correctness reference** for vector banking ("a placement change must not move a rendered bit", AGENTS.md). It is a deliberate, named exception to "modes production never needs are removed". After the 64-bit-only ruling (#1041) no shipped platform selects it. This issue replaces draft R8 (`docs/handoffs/dead-code-2026-09-28/issues/R8-…`, which proposed removal; its amendments explain why the Simd4-against-Simd8 comparison alone is weaker).

## Smallest closable slice

1. Make `Backend::Scalar` and the whole-plan scalar path compile only for tests and the `test-support` features, so shipped artifacts (the AudioWorklet module, `capi` for iOS and Android, native) do not contain it. The per-effect one-lane scalar leg (W=1, `FrameLane`) is separate and stays wherever it is used.
2. Remove the now-dead x86 arms of `Backend::current()` noted in the #1041 verdict.
3. Coordinate with the scalar-wasm test builds (decision 7 and draft 05 of the test-value audit): if those retire first, the `#1041` scalar-wasm exception in `lane` goes with them.

## Objective gates

- The banking oracle tests (the forced-scalar differentials used by #966, #970, #971 and the console `Backend::Scalar` comparisons) still build and pass, and still go red under a planted banking-key mutation that is identical at both widths.
- Shipped artifacts contain no scalar whole-plan path: a check over the built AudioWorklet module and a release `capi` library (symbol or roster check), and the artifact change, if any, is explained.
- All four targets build; console digests unchanged; clippy, fmt and policy scripts pass.

## Attempt 1 evidence

Terra, 2026-09-28. Branch `codex/1059-scalar-backend-test-only` from `c69736c1` (`codex/batch-slim-2`).
Commits `9be7eb86`, `95f511c4`, `db39d8e8` and this record. Diff `c69736c1..HEAD` before this record:
22 files, +802/-148, of which 353 lines are the new check script.

### What changed

- **`lane`.** New `test-support` feature. `Backend::Scalar` exists only with it, and on the targets
  `current()` gives it to: the scalar-wasm CI exception (wasm32 without `simd128`) and the targets
  `lib.rs` refuses. The `target_arch = "x86"` arms of `Backend::current()` and `attest_host` are
  gone (#1041 verdict). The variant's cfg repeats the existing complement arm's target list, so
  no target is newly named; #1062 deletes both with the exception.
- **`builtins-compiler`.** The per-node strip owners (`InputProcessor`, `FaderProcessor`,
  `MatrixProcessor`, the three `Console*Processor`s), `ScalarPairProcessor`,
  `ScalarSplitPairProcessor`, their factories, `strip_bindings` and the bankless
  `into_graph_artifact` compile only for `cfg(test)` and `test-support`. `test-support` now
  forwards `lane/test-support`. Shipped builds lower through `into_graph_artifact_with_banks` and
  a new `unbanked_strip_bindings`. That function asserts no strip is left unbanked instead of
  dropping it. A scalar dispatch without `test-support` panics with a message naming #1059.
  `graph_scalar_owner_resource` returns the zero estimate the oracle form computes at a bank width.
- **`graph`.** The two scalar fader/matrix pairing passes moved unchanged into
  `select_scalar_pairs`. They, their helpers, `scalar_pair_factory`, `scalar_split_pair_factory`,
  `ScalarPairFactory` and `ScalarSplitPairFactory` are test-only.
- **Outside `lane`, nothing matches `Backend`'s variants exhaustively.** `BankWidth::for_backend`
  asks `width()`. The compressor matches a `const` of it. The five tools map `width()` to their
  backend ids. Tests that name `Scalar` get `lane/test-support` as a dev-dependency.
- **The check, `scripts/check-scalar-oracle-absent.py`.** It reads the wasm `name` section itself
  and a native library with `nm`. It fails on any of 17 length-prefixed scalar-path identifiers.
  It also fails if any of 4 banked twins is missing, which is the positive control.
  - CI: `artifact-gates` runs it on the downloaded module. `audit-native` runs it on
    `target/release/libcapi.so` right after `check-capi-abi.sh` builds `-p capi` alone, which is
    the shipped feature set. `lint` runs `--self-test`. No new job, so the router and verdict
    tables are unchanged.
  - `test-debug-b` names `lane/test-support`, and `test-test-support-ci.py` covers that.
  - `docs/TARGET_MATRIX.md` records the oracle and the exception.

### What stays, and why

- **Layouts are unchanged.** `StripPreparation` keeps its scalar sections, and the executor keeps
  its split-pair slot, table and never-taken dispatch. `resource_plan` and
  `scalar_split_{op,runtime}_layout` charge them to every plan. A feature that moved an estimate
  would make test builds and the shipped module disagree. Shedding them is a layout change with
  re-pins, so it is left for a follow-up.
- **The scalar-wasm exception** keeps `Backend::Scalar`, so the scalar G5 guest still reports
  backend 0. Scalar-wasm builds compile none of the lowering, and nothing on those legs compiles a
  plan. #1062 removes the exception.
- **Left as they were:**
  - the `target_arch = "x86"` copies in `target-smoke` and `soft-clip`, which are outside `lane`;
  - the redundant inner test-support cfgs inside the gated owners;
  - the per-effect `W = 1` leg and `FrameLane`.
- **Pre-existing, not changed.** `cargo check -p builtins-compiler --all-targets` without
  `--features test-support` fails. It has 21 errors at base and 20 now; `graph` is now a
  test-support dev-dependency.

### Gates

- **The shipped artifacts contain no scalar path.**

  | build | module | `libcapi.so` / `.a` |
  |---|---|---|
  | base `c69736c1` | red, 15 of 17 identifiers | red, 15 of 17 |
  | this branch | pass | pass (both files) |
  | this branch, re-exposed (`--features builtins-compiler/test-support`) | red, 14 of 17 | red, 14 of 17 |

  The native library was re-exposed by building `-p capi -p builtins-compiler` together. The
  `--self-test` catches four hand mutations of the checker: no length prefix, no positive control,
  the wrong name subsection, and hits ignored.
- **The banking oracle passes and goes red.** Every test passes:
  - `bank_levels` (#966, #970, #971);
  - graph-compiler's console `Backend::Scalar` comparisons;
  - host-core `limiter_linked_session`;
  - the builtins-compiler forced-scalar tests.

  Two width-identical mutations were planted in a scratch copy:
  1. `program_key()` ignores `bypass`. `randomized_consoles_compile_bind_and_render_the_scalar_bits`
     fails with "banking moved rendered bits" on 13 seeds, at both `Simd4` and `Simd8`. Two
     structural tests fail too.
  2. The fader bank and the fused fader/matrix bank drain one block late.
     `a_banked_fader_command_lands_on_the_block_it_was_admitted_in` fails with "banked fader drain
     vs the per-node console: block 14 differs". 14 more tests fail.
- **The test list is unchanged.** `cargo test -- --list` over `test-debug-a`, `test-debug-b` and
  the release-only packages lists the same 2,260 tests in 190 binaries at base and at this branch.
  The diff is empty.
- **Tests pass.**
  - `test-debug-a`: 1,290 passed, 7 ignored.
  - `test-debug-b`: 799 passed, 24 ignored.
  - `test-release` plus audit, bench and console-workload in release: 245 passed, 18 ignored.
- **Console digests are identical.** The 17 rows of `gain_pan_profile digests` match base
  byte for byte.
- **Wasm gates pass.** `run-wasm-gates.sh` passes all legs: native backend 2, wasm scalar
  backend 0, simd128 backend 1, and V8 EQ loops.
- **Builds and cross-targets pass.**
  - `check-cross-targets.sh` passes: AArch64 iOS and Android check and clippy, wasm scalar and
    `simd128` rows, and the armv7 refusal.
    - It caught a regression, fixed in `db39d8e8`. The first compressor rewrite matched on
      `request.width` and compiled the `Simd8` bank into AArch64 code: memset calls went from 970
      to 1860.
  - The product crates pass clippy `-D warnings` without features for `aarch64-apple-ios` and
    `aarch64-linux-android`. `host-web` passes for wasm `simd128`.
  - CI's scalar-wasm 18-package release build passes.
  - `test-/check-wasm-realtime-atomics.sh`, `check-protocol-wasm-parity.sh` and the evidence-crate
    wasm checks pass.
- **Lint and policy pass.**
  - `cargo check --workspace --all-targets` passes with and without `--all-features`.
  - clippy `--all-features -D warnings`, fmt and rustdoc `-D warnings` pass.
  - All 58 lint-job policy commands pass (Python with `-B`), including `check-ci-path-routing.py`
    and `test-ci-path-routing.py`.

### The AudioWorklet artifact (not re-pinned)

- **Hash.** The module is `d8d03ba9…`. A base build in the same checkout is `6c952a2c…`; the pin
  `f7bd75ca…` was already stale mid-batch.
- **Size.** 3,485,631 → 3,415,107 bytes (−70,524, −2.0%).
- **Functions.** 2,670 → 2,587.
  - 101 were removed: 42 builtins-compiler scalar owners and lowering; 23 `builtins` scalar
    section bodies that only those owners call (`InputStage<f32>::process` and others); 3 graph
    pairing functions; 33 core and alloc instances.
  - 18 were renamed, because v0 impl disambiguators shift.
  - 23 surviving functions changed. All are control-plane: compile, bind, prepare, bank
    constructors, and each effect's `bind_homogeneous_bank` through the new `for_backend`.
    - Once call indices and static addresses are normalised, no render-path function differs.
- **Artifact gates.** `check-web-audioworklet.sh` passes.
  - The render, `meter_poll` and `command_submit` closures and their trap owners are unchanged.
  - Every roster kernel's counts are unchanged.
  - Total `f32x4` arithmetic went 14,139 → 14,007. The whole difference is in the removed per-node
    bodies.
- **Resources.** `check-browser-expected-resources.py --artifacts` reports every resource row and
  digest unchanged.
- **Release `libcapi.so`.** Text is 3,861,237 → 3,792,617 bytes (−68,620). Function symbols are
  3,146 → 3,063.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-28. Verified on a scratch merge of this branch (`b16350f9`) into the batch
head `codex/batch-slim-2` (`6709552c`). The merge was clean. A trial merge of #1033
(`codex/1033-remove-native-pcm-runner`) on top was also clean, textually and semantically:
`cargo check --workspace --all-targets` passes with and without `--all-features`, and the
routing, reachability, test-support and scalar-oracle policy scripts pass. The two branches share
only `qualification.yml`, in different jobs.

### What was verified

- **Shipped builds exclude the scalar path.**
  - `cargo tree -e features` shows `lane` with only its `default` feature in the normal closure of
    `host-web` (wasm32), `capi` (x86-64, `aarch64-apple-ios`, `aarch64-linux-android`),
    `host-mobile` and `host-native`. Even `--workspace` normal dependencies never turn on
    `lane/test-support`. Only dev-dependencies and the named CI `--features` enable it.
  - A planted `lane::Backend::Scalar` in `host-core` fails to compile (E0599) for `capi` on all
    three native targets and for `host-web` on wasm `simd128`, and compiles with
    `lane/test-support`. A shipped build cannot name the variant, so it cannot fall back to it.
  - The release module has 0 of 17 scalar-path identifiers and all 4 banked controls. So do the
    release `libcapi.so` and `libcapi.a` (x86-64), the Android `libcapi.a` and the iOS `libcapi.a`
    (read with `llvm-nm`). The base module is red on 15 of the 17.
- **The check discriminates.** Every re-exposed build goes red; the rebuilt shipped control is
  green and reproduces `d8d03ba9…`.

  | re-exposed build | wasm module | x86-64 `libcapi.so` |
  |---|---|---|
  | the graph half's cfg gating reverted in code | red, 1 identifier | not built |
  | a `graph/test-support` feature leak | red, 1 identifier | red, 1 identifier |
  | a `builtins-compiler/test-support` feature leak | red, 14 identifiers | red, 14 identifiers |

  - CI runs the gate in `artifact-gates` and in `audit-native`. The `audit-native` step reads the
    library that `check-capi-abi.sh` builds with `-p capi` alone. If that step's inputs ever
    change to the four-package build, the gate fails closed, because `audit` turns on
    `graph/test-support`.
  - The gate's `--self-test` runs in `lint`.
  - `check-ci-path-routing.py`, `test-ci-path-routing.py` and both reachability scripts pass. The
    router gives `full` for this diff.
- **The oracle still works, and runs in required CI.** `test-debug-a` names
  `builtins-compiler/test-support`, which forwards `lane/test-support`. I planted two new
  mutations, each identical at both widths:
  - **A.** `program_key()` sets `bypass: false`.
    `randomized_consoles_compile_bind_and_render_the_scalar_bits` fails with "banking moved
    rendered bits" at both `Simd4` and `Simd8`, and one structural test fails too.
  - **B.** Every banked fader command's smoothing is one sample longer.
    `a_banked_fader_command_lands_on_the_block_it_was_admitted_in` fails with "banked fader drain
    vs the per-node console: block 14 differs". This is the console `Backend::Scalar` comparison.
    The builtins-compiler composite test fails too.
  - `limiter_linked_session` stays green under both mutations. It does not exercise either one.
- **The shipped-only branches hold on the whole test corpus.** In test builds I asserted two
  things:
  - `unbanked_strip_bindings` never sees a strip left over at a bank width;
  - `graph_scalar_owner_resource` at a bank width equals the zero estimate that the shipped build
    returns.

  All 1,290 `test-debug-a` tests passed with both asserts.
- **No target-specific behaviour.** The removed `target_arch = "x86"` arms are dead, because
  `lane/src/lib.rs` refuses every target outside x86-64, AArch64 and wasm32 (#1041).
- **The module shrink.** It went from 3,485,631 to 3,415,107 bytes (`d8d03ba9…`). With hashes,
  call indices and relocated immediates normalized, 95 functions are removed, 12 added (the
  ADDED column is mostly merge and rename artefacts) and 25 changed.
  - The only render-path function that changed is
    `BuiltinFaderBank::try_process_settled_with_matrix`. Two `u8` field loads swap offsets, 676
    and 677, and no instruction changes (finding 3).
  - `check-web-audioworklet.sh` passes: the callgraph for the render, `meter_poll` and
    `command_submit` closures, and the kernel roster. `f32x4` arithmetic went from 14,139 to
    14,007.
  - The V8 spill gate passes on Node 22.23.2.
  - `check-browser-expected-resources.py --artifacts` passes: every resource row and every
    digest.
- **Gates on the merge.** All of these pass:
  - `cargo check --workspace --all-targets`, with and without `--all-features`;
  - clippy `-D warnings`: the workspace with `--all-features`, re-run after touching the changed
    files, and the shipped feature sets (`--lib` without features) on x86-64, iOS, Android and wasm
    `simd128`;
  - fmt and rustdoc;
  - `check-cross-targets.sh`. Every iOS memset count equals its ceiling (`graph` 20,
    `compressor` 970, `builtins` 376, `host-core` 4); none dropped, and none reached zero;
  - the `wasm-guests` steps: the scalar-wasm 18-package build, the atomics check, protocol parity,
    and `run-wasm-gates.sh` with backend 2 native, 0 scalar-wasm and 1 `simd128`;
  - `release-shape` and `check-capi-abi.sh`;
  - every `route`, `lint` and `docs-gates` step, 37 in all, Python under `-B`.

  Test results:

  | leg | passed | ignored |
  |---|---|---|
  | `test-debug-a` | 1,290 | 7 |
  | `test-debug-b` | 799 | 24 |
  | release: `lane`, `math`, `wasm-gates`, `audit`, `bench`, `console-workload` | 245 | 18 |
  | release: `bank_levels` | 9 | 0 |

  `host-native` runs. The 17 `gain_pan_profile digests` rows are byte-identical to base.
- **Expected red.** The AudioWorklet pin (`6c952a2c…` against `d8d03ba9…`) stays red until the
  boundary repin.

### Findings, by severity (none blocks)

1. **Low: two new `target_arch` cfgs in `lane`.** They sit on `Backend::Scalar` and on its
   `width()` arm, and repeat the target list of the existing complement arm of `current()` so the
   scalar-wasm exception keeps the variant. On every product target the cfg reduces to
   `feature = "test-support"`, so no behaviour becomes target-specific. #1062 must delete both,
   together with the exception.
2. **Low: the gate sees the graph half only through one function.** Both graph-half
   re-exposures were caught by `scalar_pair_is_in_place` alone. Fat LTO inlined or merged the
   other four graph identifiers. An inlining change could silence the graph half. Follow-up:
   give it a non-inlinable marker, for example `#[inline(never)]` on `select_scalar_pairs`.
3. **Low: the evidence overstates one claim.** It says "no render-path function differs", but
   `try_process_settled_with_matrix` differs by a field-order swap. That comes from `Backend`'s
   variant count: test-support and shipped builds order `Backend`-bearing structs differently.
   Sizes and resource rows agree.
4. **Low: no Rust test compiles the shipped branches.** Every test leg enables
   `builtins-compiler/test-support`, so `unbanked_strip_bindings`' refusal and the zero estimate
   run only in the module, `capi`, `console-workload` and `host-native`. The probes above show
   the invariant holds. Keeping them as permanent test-build asserts would pin it.
5. **Pre-existing on base, not #1059.**
   - `scripts/run-aarch64-tests.sh debug` names `source/test-support`, which #1035 removed. At
     `6709552c` its feature list fails resolution ("failed to select a version for `source`"),
     so the `aarch64-debug` job should be red. It needs its own fix before the batch pushes.
   - `cargo check -p builtins-compiler --all-targets` without test-support has 21 errors at
     base and 20 on the merge. CI never runs it.
