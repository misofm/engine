# LANE-4 evidence fix-forward

## Objective

Fix forward the evidence omissions in merged PR #384 without changing production code or rendered
bits. The post-merge Fable audit found the LANE-4 implementation correct but the original
implementer-authored verification and benchmark/codegen claims insufficiently reproducible.

## Scope

- Add `Lane::exp2_int_in_range` to G1 with only contract-valid integer inputs in `[-126, 127]`.
- Add a named scalar/Simd4/Simd8 differential test for `fast_gain_from_db` and `exp2_lane` over
  NaNs, infinities, signed zero, subnormals, and the neighbourhoods of `-127`, `-126`, `126`, and
  `127`.
- Register one unconsumed evidence arm for the console benchmark, run it once without deleting or
  overwriting tracked artifacts, and retain its raw, accepted, and disposition records.
- Retain reproducible probe source and full `llvm-objdump --demangle` output for both callers at
  pre-LANE-4 `9c062318` and merged LANE-4 `2b38ba7f`.
- Correct PR #384 and the implementer-authored #349 LANE-4 note: withdraw the unreproducible
  benchmark statement, remove any claim that the F1 bounds sweep proves bit identity, and identify
  Fable as the verifier for this fix-forward.

Production lane/math implementation, AArch64-specific code, floor accounting, dependencies, and
generated product artifacts are outside scope. If a production change is required, stop.

## Allowed paths

- `.github/ISSUE_SPECS/388-lane-4-evidence.md`
- `crates/lane/tests/g1_op_identity.rs`
- `crates/lane/tests/support/mod.rs`
- `crates/math/tests/m2_lane_identity.rs`
- `scripts/run-console-benchmark.sh`
- `artifacts/issue388-lane4-evidence/**`

## Objective gates

1. `cargo test --locked -p lane --release --test g1_op_identity` passes with
   `exp2_int_in_range` in `ALL_OPS`.
2. `cargo test --locked -p math --features lane --release --test m2_lane_identity` passes and the
   directed test names both callers and compares scalar, Simd4, and Simd8 bits.
3. `cargo test --locked --workspace` has the current `main` pass count plus the new directed test.
4. `scripts/test-console-benchmark.sh` and `scripts/check-realtime-policy.sh` pass.
5. Exactly one admitted invocation of
   `scripts/run-console-benchmark.sh --issue388-lane4-evidence` produces a disposition JSON and
   does not delete or overwrite any tracked artifact.
6. Full before/after disassembly for both non-inlined caller wrappers records commits `9c062318`
   and `2b38ba7f` and visibly removes one `vmaxps` plus one `vminps` per caller.
7. The branch changes no production Rust source. Fable 5.1 verifies and merges the open PR; Codex
   does neither.

## Seen, not done

- The browser qualification `candidateCommit` squash-merge convention and #384's immutable merge
  trailer are recorded by #388 but are not part of its done conditions or this evidence-only
  change.
