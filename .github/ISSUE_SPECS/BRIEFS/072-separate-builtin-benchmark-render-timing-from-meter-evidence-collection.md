# Sol implementation brief — issue 072 builtin benchmark timing/evidence separation

## Decision and attempt budget

**SOL XHIGH FAIL / TERMINAL STOP / NOT COMPLETE; ALL ISSUE-072 AUTHORITIES ARE CONSUMED.** The
issue permitted one Sol High implementation pass and, after one Sol XHigh
HOLD, one bounded Sol High correction. Sol XHigh adversarially verified both; a second HOLD would
have stopped the issue. Issue-072 counters remain
`preflight_invocations=0`, `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`. Issue 058's consumed `1/1/1` run remains immutable history.

The refreshed clean input is `main` commit `c0eb5c5cb438e7a3b3cdaea30370812e09d8dffb`, tree
`43c6fd994d798fa93576abcda9a9a2eda3198bfe`, and `Cargo.lock`
`4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`.
`target/issue72` is absent. This rebrief authorizes no preflight, main, runner, workload or timing.

Sol High pass 1 produced a focused-green live tranche. Sol XHigh returned one bounded HOLD because
the runner validated the preflight seal's shape but did not independently bind every source, lock
and input authority or the complete nonbenchmark seal. Sol High pass 2 recomputed and compared all
authorities before raw creation, with direct and tandem seal-replacement lifecycle mutations. At
the focused checkpoint, all untimed render, package, lifecycle, locked, fixture and policy gates
passed; Issue-058 artifacts were exact, `target/issue72` was absent and counters were `0/0/0/0`.
Sol XHigh returned a strict focused PASS and authorized only the exact checkpoint commit. That
focused verdict was not overall PASS and did not authorize any later seal or execution by itself.

## Terminal result and successor route

Clean HEAD `9dc95a5fb4d8e65c582b84320c84b22f2d780eba`, tree
`7e99e5fafa130e572d421156037b36f7f59232d7`, completed its sole preflight and runner with exit zero.
Final preflight/runner/workload/timed counters are `1/1/1/1`, warmup `1`, rounds `2`, records `20`.
Raw and accepted are byte-identical distinct-inode files. The exact matrix, identities, percentile
relations and cross-round output hashes validate; all 16 render rows have zero errors and zero
forbidden operations, while four preparation rows correctly use `not_applicable`. The five stderr
phase lines and PASS/complete disposition are exact.

Issue 072 nevertheless fails its complete-host-metadata gate. Every record has all 16 host/build
fields null and lists all 16 in `missing_metadata`, including available architecture, core-count,
OS, kernel, compiler, target and release-profile facts. `Metadata::collect` reads fixed
`MISO_ENGINE_BENCH_*` variables, while the sealed runner exported only candidate commit and binary
SHA-256. Honest null/list validation is not evidence that discovery was attempted. The runner
authority is consumed, so there is no retry or overall PASS.

Preserve these regular one-link Issue-072 artifacts exactly: nonbenchmark seal 2,109 bytes /
`7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d`; binary 3,200,296 /
`a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912`; preflight seal 1,525 /
`f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf`; raw and accepted each
40,136 / `c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a`; stderr 211 /
`7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396`; disposition 1,252 /
`b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e`; prelaunch absent.

Successor issue 109, **Repair builtin benchmark host metadata export and reauthorize one descriptive
run**, owns the runner-only repair and successor namespace. It is not an Issue-072 retry. No more
Issue-072 preflight, runner, main, workload or timing invocation is authorized.

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

## Exact dependency

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

Issue 068 is remotely closed and locally PASS. Stopped Issue 058 is immutable technical input and
failure evidence, not a PASS dependency.

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
hash and workload contract from Issue 058. Preserve these regular one-link `target/issue35`
artifacts exactly: binary 3,191,104 bytes /
`242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944`; preflight seal 2,211
bytes / `85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d`;
empty raw and stderr files each
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; and FAIL disposition
974 bytes / `e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce`
with counters `1/1/1`, warmup zero and rounds zero. Accepted output and prelaunch disposition remain
absent. Never write, link, rename, truncate or delete beneath `target/issue35`.

All new artifacts use `target/issue72`; preflight and disposition own `issue=72`, while benchmark
records retain `issue=35`. The two fixed commands remain:

```sh
bash scripts/preflight-builtins-benchmark.sh
bash scripts/run-builtins-benchmark.sh
```

Neither command is authorized during implementation. Each later authorization is exactly once,
no retry. The runner continues to launch only the preflight-sealed binary directly.

Persistent successor paths are exactly:

- `target/issue72/nonbenchmark.seal.json`;
- `target/issue72/miso_engine_builtins_bench`;
- `target/issue72/builtins-benchmark.preflight.json`;
- `target/issue72/builtins-benchmark.raw.jsonl`;
- `target/issue72/builtins-benchmark.jsonl`;
- `target/issue72/builtins-benchmark.validator.stderr`;
- `target/issue72/builtins-benchmark.prelaunch.disposition.json`; and
- `target/issue72/builtins-benchmark.disposition.json`.

Each is regular, one-link, atomically published and never overwritten. A preexisting prelaunch or
final disposition blocks the runner before scratch creation. Any runner call consumes its one-shot
authority, including prelaunch failure.

Current preimplementation hashes are benchmark source `3e68bf52e76e7a3ecb4b07c97056cce0e85aa6648ac7267a0dc79c89e84b1100`,
preflight `76ff9959b96e20ae2fd21e46ce8f12359a6b77ca317ba90fc13e998761d1df40`,
runner `7e429ca2df3c24494e9d3c9bcc03b4e0713851d34b8831eb449d1c5f41cccabc`, and
lifecycle `cb26f6f05c963ff1b56d8a8ce3aeb82e4d6b1561d60f6bd5a58240d3ef9cf74d`.
The postimplementation nonbenchmark seal binds their final hashes. Frozen record/aggregate
validator hashes are `c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467`
and `6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63`;
neither validator nor `Cargo.lock` may change.

The Issue-058 base commit `265109f300f58e005ac7a68a56298d167c5ae809`, pretransition lock
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`, lock-diff
`5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`, and original Issue-072
briefing lock `da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`
remain historical provenance only. Remove the stale active comparison of current main against that
old base/diff. Bind the exact clean current/postimplementation lock instead; Issue 072 permits no
lock or dependency edit.

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
   missing tool, stale/mismatched seal, every preexisting regular/symlink/hardlink artifact,
   successful zero-launch preflight, drift/no-clobber, prelaunch disposition, partial
   failure/interruption, phase-derived counters, validator failure and distinct-inode atomic
   accepted publication without executing a real workload.
3. On a clean candidate, run the remaining read-only fixture, locked workspace, rustdoc and
   policy/mutation/static gates once. Sol High pauses; Sol XHigh returns focused PASS or HOLD. One
   HOLD permits only bounded pass 2, and a second HOLD is final STOP.
4. Root commits the exact green implementation paths. On that clean candidate, root creates only a
   no-clobber `target/issue72/nonbenchmark.seal.json`, binding branch/HEAD/tree, current lock, final
   source/lifecycle hashes, frozen validators/inputs, Issue-058 artifacts, focused regression `1`
   and counters `0/0/0/0`. Sol XHigh validates it before authorizing exactly one no-workload
   preflight.
5. The sole preflight builds but never executes the benchmark binary and may publish only the
   binary and preflight seal. Sol XHigh then independently validates candidate and all transitive
   identities, exact counters `1/0/0/0`, one-link/no-clobber state, Issue-058 preservation and
   absent raw/accepted/stderr/dispositions. Only that separate GO may authorize exactly one runner.
6. The runner alone may execute one untimed warmup and two measured rounds. Success requires 20
   valid records, byte-identical distinct-inode raw/accepted files, five-kind/two-rate/two-round
   order, zero audit violations, and total counters `1/1/1/1`. Any prelaunch or postlaunch failure
   publishes the applicable disposition, preserves evidence and is terminal; no alternate,
   direct, repeat or resumed invocation exists.

## PASS boundary — not met

PASS requires the exact 20 valid measured rows, byte-identical raw/accepted output, complete PASS
disposition, zero render audit violations, Issue-072 preflight/runner/workload/timed invocation
counters `1/1/1/1`, unchanged
Issue-058 artifacts, and no threshold or tuning claim. It unblocks exact title
**Issue-007 builtin filter and matrix human listening qualification** and the release chain; it
does not complete listening or release qualification. The pass-2 focused implementation remains
accepted technical input, but the consumed terminal run failed metadata completeness. Issue 072 is
stopped, not complete, and grants no further authority.
