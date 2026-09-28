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
    - `scalar_pair_is_in_place` (`:4864`) and `scalar_split_interval_is_clear` (`:4887`);
    - the pair-selection passes (`runtime.rs` around `:5530-5750`) and split-pair completion.
- **Tests:** 2,000-3,000 lines. `Backend::Scalar` appears at 48 sites in builtins-compiler and 19
  in graph-compiler, plus host-core's forced-Scalar tests and
  `true-peak-limiter`/`limiter_linked_session.rs:403`, `:602`.
- **The catch: the scalar path is the oracle for "banking never moves a bit".** Several tests
  render the same session at `Scalar` (per node) and at a banked width, and compare bits. Examples:
  - graph-compiler `lib.rs:11352`, `:11680`;
  - the limiter linked-pair session tests;
  - host-core's `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses` in
    `builtin_batch_endpoint.rs`, which `02-…` deletes with its file.

  Replace that oracle **before** deleting the path.
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
3. **Shipped artifact.** Build base and change on one machine.
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
