# Remove the whole-plan scalar backend and the scalar wasm build

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R8.

**Why this needs your ruling.**

- The #959 spec flagged this for you: "`Backend::Scalar` still produces bankless with-builtins
  plans (host-core tests, the CI scalar wasm compile, scalar-only split-pair code)".
- The W4-D1 note in `scripts/build-web-audioworklet.sh` deliberately kept "the scalar *cargo
  check*" in CI.

The ruling to record: "A plan is compiled only at a shipped width (`Simd8` natively, `Simd4` on
wasm). The whole-plan `Backend::Scalar` path, its per-node builtin strips and scalar split pairs,
and the scalar (`-simd128`) wasm build are removed. Scalar *tails* inside banked plans stay."

## Context

- **How the path is reached.** When `BankWidth::for_backend(dispatch)` is `None`, which only
  happens at `Backend::Scalar`, the one production entry `into_graph_artifact_with_banks`
  (`crates/graph-compiler/src/compile.rs:113`) falls back to per-node strip bindings
  (`crates/builtins-compiler/src/lib.rs:2357-2362`).
  - The code says so itself: "scalar post-input bindings survive only when the backend has no
    bank width at all" (`:1293-1294`), with `debug_assert!(plan.scalar.is_empty())` at `:1342`.
- **No shipped target produces `Scalar`.** x86-64-v3 gives `Simd8`; wasm `simd128` and AArch64
  give `Simd4` (`crates/lane/src/backend.rs`). host-core's backend parameter is a `#[cfg(test)]`
  seam (`crates/host-core/src/prepare.rs:593-603`).
- **Scalar-only production code** (SCIP extents; re-prove by compile), about 1,500 lines:
  - **builtins-compiler:**
    - about 620 lines at `lib.rs:4086-4707`: `InputProcessor`, `FaderProcessor`,
      `MatrixProcessor`, `Console{Input,Fader,Matrix}Processor`, `ScalarPairProcessor`,
      `ScalarSplitPairProcessor`, `make_scalar_pair` and `make_scalar_split_pair`;
    - `strip_bindings` (`:2146`, 89 lines);
    - the scalar-owner resource estimate (`:2240-2338`);
    - the scalar test-support witnesses (`:818-870`, `:1665-1690`, `:6708-6870`).
  - **graph:**
    - `ScalarPairFactory`, `ScalarSplitPairFactory` and the `GraphRuntimeSplitPairProcessor`
      trait (`lib.rs:2049-2147`);
    - the `…WithoutSplitPair…` layout mirrors (`runtime.rs:987-1045`, `:2261-2320`,
      `lib.rs:2458`);
    - `scalar_split_op_layout` and `scalar_split_runtime_layout`;
    - `scalar_pair_is_in_place` (`runtime.rs:4864`) and `scalar_split_interval_is_clear`
      (`runtime.rs:4887`);
    - the pair-selection passes (`runtime.rs` around `:5530-5750`) and split-pair completion.
- **Tests:** 2,000-3,000 lines. `Backend::Scalar` appears at 48 sites in builtins-compiler and 28
  in graph-compiler (19 in `src/lib.rs`, the rest in `tests/`), plus host-core's forced-Scalar
  tests.
- **The catch: the scalar path is the oracle for "banking never moves a bit".** These tests render
  the same session at `Scalar` (per node) and at a banked width, and compare bits:
  - `crates/graph-compiler/tests/bank_levels.rs`, the largest. It has
    `WIDTHS: [Backend; 3] = [Scalar, Simd4, Simd8]` (`:61`) and the helper
    `assert_binds_and_renders_the_scalar_bits` (`:797-826`, "banking moved a rendered bit"),
    which nine tests use. It also has `randomized_consoles_compile_bind_and_render_the_scalar_bits`
    (`:1088`).
  - graph-compiler `src/lib.rs:11352`, `:11680`, and `scalar_dispatch_compiles_without_banks_on_any_host`
    (`:2543`), with the Scalar resource baselines (`:3306-3311`, `:3388-3394`).
  - `crates/host-core/src/limiter_linked_session.rs:403`, `:602` (three tests). These use the
    `#[cfg(test)]` seam `prepare_host_runtime_with_console_backend` (`prepare.rs:596`).

  Replace that oracle **before** deleting the path.

  Not an oracle: `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
  (host-core `builtin_batch_endpoint.rs:1497`). It compares each backend's endpoint with its own
  reference (`:1729-1743`), not Scalar with banked. `02-…` deletes it with no banking claim lost.
- **Pins that go with the enum variant.** If `Scalar` leaves `Backend`, the "Scalar has no bank
  width" pins go too, each listed with that reason:
  - `builtins/tests/stage.rs:633`, `:642`;
  - `effect-contract/src/lib.rs:2395`, `:2406`;
  - `compressor/tests/contract.rs:396`.
- **Scalar wasm in CI.**
  - wasm-guests "Scalar Wasm build (lane's scalar wasm path)" (`qualification.yml:766-769`): 18
    packages built with `-simd128`, 0.6 min.
  - "Browser-local realtime path has no atomic opcodes" (`:772-775`,
    `scripts/check-wasm-realtime-atomics.sh` and `scripts/test-wasm-realtime-atomics.sh`): 0.6
    min. Its message still says "browser-local fallback artifact" (`:33`). It duplicates
    `scripts/check-web-audioworklet.sh:69`, which asserts no atomic opcodes on the shipped
    `simd128` module.
  - The scalar guest leg of `scripts/run-wasm-gates.sh` (`:201`), and the `-simd128` variants in
    the protocol parity, cross-target, interchange and effect-package/descriptor scripts.
- **Shipped module.** The code is compiled in but never executes at `Simd4`. Removing it changes
  the pin and moves no console digest.
- **Keep:** lane's one-lane `Scalar`/`FrameLane` for tails, and `reduce_many::<FrameLane>`
  (`graph/src/runtime.rs:426`). That is live production code at every width.

## Smallest closable slice

1. **Replace the oracle first.** For each test that compares a forced-`Scalar` render with a
   banked one, compare `Simd4` with `Simd8` instead. Both are banked, with different lane
   geometry, so a banking bug that depends on geometry shows as a mismatch. Where a per-node
   reference is essential, compare with the lane-level scalar kernel or `dsp-reference`. Show each
   replacement red on a mutation that breaks bank regrouping. List old test and new test side by
   side.
2. **Compile only at shipped widths.**
   - Make `Backend::Scalar` unrepresentable as a *plan* backend: remove it from `Backend`, or
     refuse it in `compile_with_builtins` with a typed error. Choose the smaller diff.
   - Delete the scalar-only production code listed above, and the test-support witnesses only it
     uses.
3. **CI.**
   - Delete the scalar wasm build step and the atomics check with its test.
   - Delete the scalar guest leg of `run-wasm-gates.sh`, and the `-simd128` variants in the scripts
     listed above.
   - Keep `check-web-audioworklet.sh`'s atomics assertion.
4. **Docs.** Update the W4-D1 note in `build-web-audioworklet.sh`, `docs/TARGET_MATRIX.md` and
   `docs/ENGINE_ENV_VOCABULARY.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes without its scalar guest leg, and its native and
   `simd128` digests are unchanged.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes.
   - Code is removed, so functions disappear.
   - The evidence lists every removed or changed function from `wasm-objdump -d`, and shows that
     every function reachable from the render export is unchanged apart from panic line numbers.
   - `bash scripts/check-web-audioworklet.sh` passes; its render-closure and trap counts may only
     fall. Re-pin with that reason.
4. **CI routing.** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The `verdict`
   table is unchanged: the wasm-guests job keeps its other steps.
5. **No live claim lost.** Every forced-`Scalar` oracle test has a named replacement from step 1
   that goes red on a bank-regrouping mutation. The `-- --list` diff contains only tests listed
   with their replacement or reason.

## Dependencies

The owner ruling, `00-…` and `02-…`. It is easier after `R1-…`, because every remaining target is
then banked.

## Standing rules for the implementer

- Step 1 is its own checkpoint commit; the deletion must not land without it.
- Commit on `codex/<issue>-remove-scalar-plans`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, findings F6 and F7. **Defer this draft.** Two facts change it.

1. **"No shipped target produces Scalar" is an Android-ABI question now.**
   `lane::Backend::current()` (`crates/lane/src/backend.rs:60-68`) gives `Scalar` for every
   architecture other than x86, x86_64, aarch64 and `wasm32`+`simd128`, silently: only x86 has a
   `compile_error!` guard (`lane/src/lib.rs:72-80`). With mobile playback in scope, a 32-bit
   `armeabi-v7a` Android build (`target_arch = "arm"`) would run exactly the whole-plan Scalar path
   this draft deletes. Failure scenario: the owner ships `armeabi-v7a` for older devices, R8 has
   landed, and that build either no longer compiles or has no execution mode. The owner must first
   rule which Android ABIs ship (R1 amendment, item 5c). If only `arm64-v8a` ships, add a
   `compile_error!` for 32-bit ARM like the x86 guard, in the same change.
2. **The proposed replacement oracle is strictly weaker.** Simd4-vs-Simd8 compares two banked
   plans. It cannot see a banking bug that is the same at both widths: a cohort key that ignores a
   per-track field (a Draft-quality track grouped with High ones), a non-identity absent slot, a
   state or sidechain bound to the wrong member, a bank-path PDC error. Failure scenario: such a
   key omission lands, both widths render the same wrong output, the new oracle passes, and the
   AGENTS.md claim "a placement change must not move a rendered bit" is no longer guarded.
   `dsp-reference` and lane-kernel comparisons are per effect and cannot see plan-level
   regrouping. The owner must choose between (a) keeping an unbanked per-node plan as a stated
   test-only oracle exception to "modes production never needs are removed", or (b) accepting the
   weaker guarantee in writing.
3. **Class-A proofs do not depend on the Scalar path.** They rest on console `output_sha256`
   equality (`scripts/run-console-benchmark.sh`, `scripts/test-console-benchmark.sh:718`, `:823`)
   and on the per-effect one-lane scalar leg (W=1, #976), which lane's `FrameLane` keeps. The audit
   is right on this point.
4. **The atomics check is not a pure duplicate.** `scripts/check-wasm-realtime-atomics.sh:36-41`
   inspects the whole non-LTO `engine` and `source` rlibs before link-time dead-code removal;
   `check-web-audioworklet.sh` sees only the linked module. If the `-simd128` build goes, re-target
   the atomics check at `+simd128` rather than delete it, or record the narrower guarantee.
5. **Mobile gap either way:** every Simd4 bit-identity test runs on x86 through `wide`'s
   SSE/AVX lowering; nothing runs Simd4 on NEON (R1 amendment).
