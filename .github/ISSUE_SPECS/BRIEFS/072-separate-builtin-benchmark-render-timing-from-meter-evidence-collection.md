# Sol implementation brief — issue 072 builtin benchmark timing/evidence separation

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1; ZERO EXECUTION AUTHORIZATION.** Permit one Terra implementation and
one bounded Sol correction. A second failure stops. Issue-072 counters start at
`preflight_invocations=0`, `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`. Issue 058's consumed `1/1/1` run remains immutable history.

## Smallest correction

Keep the exact Issue-058 benchmark product and evidence definitions. Change only operation
orchestration:

```text
prepare input (outside)
start timer
  arm realtime audit
    render product only
  disarm realtime audit
stop timer
collect/drain/hash evidence (outside)
```

For direct workloads, hash the completed planar block after elapsed time is captured. For
`meter_success_full`, render the success and prefilled-full plans while armed, then iterate the
seven success consumers directly after disarm, hashing each exact handle/tap/snapshot without
collecting a `Vec`. Hash both PCM blocks and update the deterministic drop evidence outside the
interval. Drain the full plan only at the frozen end-of-round evidence point. Warmup performs the
same render-only armed call and outside-scope success drain but does not retain or hash evidence.

Do not use `MaybeUninit`, custom allocation, unsafe storage, a new meter API, or changed queue
capacity to hide the defect. The seven-consumer bound already belongs to this exact benchmark
workload; direct iteration is sufficient. Product render order, first-sample positions, matrix
retarget order, state continuity, full/drop behavior, and output-hash byte order remain unchanged.

## Focused executable proof before any preflight

Add a test-only one-operation path for each of the four render workloads. It must:

- prepare both round states through the existing checked inputs;
- reset audit counters, arm only the product render call, and assert all nine counters remain zero;
- assert audit is disarmed before any queue pop or SHA update;
- drain exactly seven ordered success taps and leave the prefilled full queue at the frozen
  cumulative-drop outcome;
- prove direct and meter PCM/evidence continuation remains equal between the two identically warmed
  round states; and
- execute through the benchmark's audited global allocator, so any render allocation aborts the
  test process instead of becoming a vacuous counter assertion.

This test is nonbenchmark evidence. It must not call `main`, `Instant`, the public runner,
preflight, audit/trace executables, target scripts, or any timing path.

## Frozen identities and paths

Measured records remain the exact Issue-035 schema and 20-row matrix. Preserve every immutable
hash and workload contract from Issue 058. Preserve the failed `target/issue35` artifacts exactly:
binary `242f6789...`, seal `85fcfcfb...`, empty raw/stderr `e3b0c442...`, absent accepted output,
and disposition `e7221487...` with counters `1/1/1`, warmup zero, rounds zero.

All new artifacts use `target/issue72`; preflight and disposition own `issue=72`, while benchmark
records retain `issue=35`. The two fixed commands remain:

```sh
bash scripts/preflight-builtins-benchmark.sh
bash scripts/run-builtins-benchmark.sh
```

Neither command is authorized during implementation. Each later authorization is exactly once,
no retry. The runner continues to launch only the preflight-sealed binary directly.

## Allowed implementation surface

- `tools/miso-engine-builtins-bench/src/main.rs` and its existing package-local tests;
- `scripts/run-builtins-benchmark.sh`;
- `scripts/preflight-builtins-benchmark.sh`;
- `scripts/test-builtins-benchmark.sh`; and
- concise Issue-072 evidence.

The validators may be inspected but not changed unless the artifact-owner field alone must accept
the Issue-072 preflight/disposition; measured record validation remains byte-for-byte frozen. No
Cargo dependency or lock change is expected. Any production, corpus, input, validator-schema,
audit, target, instruction, listening, or unrelated tooling change stops.

## Checkpoint sequence

1. Land the render-only/off-scope-evidence correction plus the focused armed-render test. Run only
   format, benchmark package tests/check and warning-denied package Clippy.
2. Land Issue-072 artifact-path and hermetic lifecycle changes. Scratch tests cover argument,
   missing tool, stale/mismatched seal, every preexisting/symlink artifact, successful zero-launch
   preflight, drift/no-clobber, partial failure/interruption, validator failure and atomic accepted
   publication without executing a real workload.
3. On a clean candidate, run the remaining read-only fixture, locked workspace, rustdoc and
   policy/mutation/static gates once. Sol records PASS TO PREFLIGHT or final FAIL.
4. After the evidence commit, Sol may authorize the sole no-workload preflight. Validate its seal
   before separately authorizing the sole runner. Any failure at either authorization is final.

## PASS boundary

PASS requires the exact 20 valid measured rows, byte-identical raw/accepted output, complete PASS
disposition, zero render audit violations, Issue-072 invocation counters `1/1/1`, unchanged
Issue-058 artifacts, and no threshold or tuning claim. It unblocks exact title
**Issue-007 builtin filter and matrix human listening qualification** and the release chain; it
does not complete listening or release qualification.

