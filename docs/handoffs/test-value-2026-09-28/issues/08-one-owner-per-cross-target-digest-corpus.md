# One owner per cross-target digest corpus

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
