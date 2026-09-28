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
