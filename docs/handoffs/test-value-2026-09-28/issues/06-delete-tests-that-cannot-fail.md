# Delete tests that cannot fail on a product change, test only test tooling, or cover removed surface

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 6). Base `a9414c0c`. The rows in part C
need owner rulings R6 and R7. Parts A and B need none.

## Problem

About 150 tests do not test the claim in their name. Some can still fail when the product breaks
badly, as a side effect of running it, but where the audit measured, nothing they catch is uniquely
theirs. They fall into four kinds:
- a tautology, such as `QuantumFrames(128).0 == 128`, `render() == render()` of a pure function, or
  `(B+C)-B == C`;
- IEEE arithmetic only;
- std-library behaviour only;
- a test-local constant or helper.

A few assert nothing at all, and some print and return. Others test benchmark and fixture tooling
rather than the engine, or test surface the owner removed. They run in about 1 s in total, so the
cost is maintenance: re-pins, reading time, and false confidence in the counts.

Each row below was read and spot-checked by the audit's classifiers against the body, not the name,
and each names its surviving guard. The item numbers refer to
[`../data/candidates-dsp-graph.md`](../data/candidates-dsp-graph.md) (**D**) and
[`../data/candidates-host-tools.md`](../data/candidates-host-tools.md) (**H**), which hold the file:line,
the reason and the guard for every row.

## Outcome: delete these

**A. Cannot fail on a product change (no ruling).**
- **D2:** `crates/parametric-eq/src/lib.rs:4471` (seven `println!`, runs in CI, asserts nothing).
- **D3:** `crates/gate-expander/tests/identity.rs:223` (returns unless `Simd4`; asserts nothing even
  then).
- **D6:** the `print_pins`/`print_digests` regeneration printers in parametric-eq, effect-runtime and
  delay.
- **D8-D16, D18-D21:**
  - the evidence-only reduction helper tests in `graph/src/lib.rs:4532`, `:4545`;
  - the repeat of a compile-time `const` assert (`graph/src/runtime.rs:8238`);
  - constant restatements (`parametric-eq/src/lib.rs:4639`, `effect-runtime/tests/state_payload.rs:25`,
    `lane/tests/fp_env.rs:207`, `lane/tests/f64_lane.rs:191`);
  - the signature tautologies `a_resident_read_is_repeatable_to_the_bit` (compressor and limiter);
  - IEEE-only `compressor/tests/mono_collapse.rs:163`;
  - equivalence-blind `compressor/tests/ramps.rs:587`;
  - `rack/tests/console_bank.rs:405`;
  - the Annex-2 one-hot tautology in `dsp-reference`;
  - three tests of test instrumentation (`builtins-compiler/src/lib.rs:9759`,
    `transient-shaper/tests/allocation.rs:112,117`);
  - four copies of `shared_hex_adapter_matches_literal_bytes`.
- **H1-H17:**
  - std-only (`protocol/tests/delivery_ownership.rs:435`, `host-core/tests/builtin_batch_endpoint.rs:862`);
  - IEEE-only (`host-core/src/builtin_batch_endpoint.rs:2127`);
  - test-local sums already stale (`session/tests/invalid_matrix.rs:1166`, `:1206`);
  - `engine/src/lib.rs:135`, `:140`;
  - `host-web/src/tests.rs:6379`;
  - `bench-support/src/stats.rs:140`;
  - `tools/audit/src/source.rs:497`;
  - the determinism-of-a-pure-function pair (`parameter-metadata/tests/abi_layout.rs:1348`,
    `session-validator/tests/validate.rs:432`);
  - `tools/bench/src/floor.rs:608`;
  - `source/src/native_source.rs:3949`;
  - `effect-package/tests/package_v1_qualification.rs:371`;
  - the ignored print-only `session/tests/descriptive_scale.rs:54`;
  - `source/src/native_source.rs:4435`.
  - Not in this list: `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`, which moves to issue 12.

**B. Tools re-testing shared bench-support helpers through an import alias (no ruling).**
- **H37-H39:**
  - five `shared_sha256_alias_matches…` copies in `tools/audit`;
  - three SHA "abc" re-tests in `tools/bench`;
  - four percentile and escape re-tests.
- The owner, `tools/bench-support`'s own tests, stays.

**C. Removed or out-of-scope surface (rulings R6, R7).**
- **R6, isolated kernel benchmarks that are not real host paths:**
  - D1 and D4-D5: the MQ-1/MQ-2 benchmark tests and markers, and the ignored ns-per-frame
    printouts in builtins, parametric-eq, multiband, true-peak-limiter, soft-clip and lane;
  - the `tools/bench/src/gate_active.rs` and `multiband_active.rs` test modules;
  - done together with issue 10's runner scripts.
- **R7:**
  - `session/tests/render_mode_tiers.rs:59`, `:72` (the retired `dependency_waves` token);
  - `builtins/tests/contract.rs:297`, `conformance/tests/effect_contract.rs:240` and
    `conformance/tests/fixtures.rs:34` (extended research-rate tiers);
  - H55 (retired pre-causal descriptors, `effect-package/tests/descriptor_v1_qualification.rs:876,967,1043`);
  - H56 (`engine/src/realtime/disjoint.rs:1016`, concurrent multi-lease execution of the removed
    dependency-wave scheduler).

**Keep, and do not delete** the `cfg(not(x86))` stubs (`lane/tests/g6_ftz_inert.rs:123`,
`tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`) and `target-smoke`'s per-target half. With
AArch64 back in scope they are the only tests that would run there; issue 12 makes them real.

## Scope

Authorized paths: the test files named in the rows, `nightly.yml` if a removed ignored test was
listed there (none is), and this issue's spec. No product code.

## Gates

1. **Cannot-fail rows.** For each row in part A, the PR states the one-line reason it cannot fail.
   Before deleting, the implementer applies a mutation the row's name claims to guard, in a scratch
   branch, and the row stays green. For example: reorder the reduction for D8; change
   `EFFECTIVE_CASCADE_DEPTH`'s *use* rather than its value for D11.
2. **Mutation equivalence** in the audited crates the rows touch, using `../tools/run-mutants.sh`
   with the audit's settings (compressor all; graph-compiler all; host-core and parametric-eq
   `--shard 0/4 --sharding round-robin`). The caught set is identical before and after. In the
   audit's baseline:
   - D14, D20 and the MQ-2 marker and spike tests catch nothing;
   - D5's preflight (108 mutants), D13 (54) and D15 (36) catch mutants, but none uniquely: every
     one is also caught by a kept test.
3. **Historical bugs.** `../tools/revert.py` for #966, #970, #994 and #1015. The red set is
   unchanged, and no row here is a reproducer.
4. **Counts.** The PR reports the before and after `#[test]` count and the number of test binaries.

## Saving and risk

- **Saving:** about 1 s of runtime and a few binaries to link. The real saving is roughly 150 fewer
  tests to read and re-pin.
- **Risk:** nil for parts A and B, whose surviving guards are named per row. For part C it is the
  owner's rulings.
