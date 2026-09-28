# One owner per cross-target digest corpus

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 8 and §8, which settles the 2026-09-04
audit's "known conflict" 1). Base `a9414c0c`. No ruling needed.

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

   Seven of them read a `MISO_ENGINE_REPIN_*` variable and skip the compare when it is set.
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

- **The owner is `scripts/run-wasm-gates.sh`,** with a native leg at every width plus the `simd128`
  guest. It is the only place that compares native *and* Wasm against the pins, which is the claim.
  When issue 12 adds an AArch64 job, it runs the same native leg there.
- **The ten per-crate compares lose their pin comparison** and their `MISO_ENGINE_REPIN_*` skip.
  Each keeps its **finiteness and non-vacuity** assertions, a separate claim that G5's vacuity checks
  exclude for delegated families.
- **`g5_native_digests_match_pins` is deleted.** Its `comparisons >= LANE_CASE_COUNT * WIDTHS` guard
  moves into the `--native` report's exit status, if it is not there already.
- **The M3 math corpus** (`crates/math/tests/m3_determinism.rs:142`) keeps its own pins. It is not a
  delegated family.

## Scope

Authorized paths:
- the ten test files above;
- `tools/wasm-gates/tests/g5_native_corpus.rs`, `tools/wasm-gates/src/`;
- `docs/ENGINE_ENV_VOCABULARY.md` (the `REPIN` rows);
- this issue's spec.

## Gates

1. **Every width is still compared.** A scratch mutation that changes one arithmetic operation only
   in the `Simd4` path of one effect makes `run-wasm-gates.sh`'s native leg red. So does one only in
   the scalar path. The run records the width in the failure message.
2. **Wasm is still compared.** The same class of mutation in a code path only the Wasm build takes
   makes the `simd128` guest leg red.
3. **Mutation equivalence, compressor.** `../tools/run-mutants.sh compressor` before and after, with
   `--test-package wasm-gates` added to both runs so the owner is counted. The caught set of product
   mutants, excluding `src/corpus.rs`, is identical.
4. **Historical bugs.** `../tools/revert.py 994` still turns the compressor's randomized
   differentials and the knee reproducers red. `1015` still turns `stationary_subnormal` red.
5. **Finiteness kept.** Each per-crate test still fails when its corpus emits a NaN; seed one in a
   scratch branch.

## Saving and risk

- **Saving:** a few seconds per PR, and about ten fewer re-pin sites per corpus change.
- **Risk:** a digest change is now reported by the `wasm-guests` job instead of the effect crate's
  debug tests, so the failure is further from the change. The message names the case and width.
