# Sol implementation brief — issue 030 benchmark runner hardening and artifact promotion

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** This is a stateless, tooling-only closure based on Issue 006's
accepted product result and preserved failed-runner evidence. It permits exactly one Terra
implementation/review attempt and at most one bounded Sol correction/review. A second failure
stops; it never reopens or relabels Issue 006.

Issue 030 authorizes **zero** graph benchmark workload or timed invocations. Do not execute the
repository runner against the real workspace, `cargo run -p miso-engine-graph-bench`, or the
benchmark binary during implementation, tests, review, or promotion. Tests may execute a copied
runner only inside a hermetic scratch repository whose `cargo` is a counted stub; those stub
processes are not graph workloads and must report their own exact launch counts.

## Frozen input and output

The only promotable source is
`target/issue6/graph-compiler-benchmark.raw.jsonl`. Before any copy it must be a regular,
non-symlink file with exactly:

- 10,364 bytes;
- six LF-terminated JSONL records; and
- SHA-256 `c03f1bc0399f0b9dea3a5c94c13a468512d2fcb2a2805c450c83110b56d623b5`.

The unmodified `scripts/graph-benchmark-validator.jq`, loaded with the existing record validator,
must accept those bytes. The destination is exactly
`target/issue6/graph-compiler-benchmark.jsonl`. It must not already exist, including as a symlink.
Promotion copies bytes to a unique temporary in `target/issue6`, verifies the temporary's size,
hash, LF record count, validator result and byte equality, then publishes it with a no-clobber
same-filesystem atomic rename. The raw source remains unchanged. On every rejection, remove only a
temporary created by that command, leave source and destination untouched, print the observed
source identity when available, and return nonzero. Never parse/reserialize, repair, normalize,
move or delete the raw source.

Issue 006's historical runner invocation count remains `1`, its accepted-runner-artifact count at
the failed checkpoint remains `0`, and its failure stays recorded. A successful Issue-030
carry-forward creates one accepted byte-identical copy and records
`issue_030_workload_invocations=0` and `issue_030_timed_invocations=0`; it does not convert the old
runner attempt into PASS.

## Exact implementation surface

Limit the implementation to:

- `scripts/run-graph-compiler-benchmark.sh`;
- new `scripts/promote-issue006-graph-benchmark.sh`;
- `scripts/test-graph-benchmark.sh`; and
- concise evidence appended to the Issue-030 spec.

No Rust, Cargo manifest, graph/compiler/runtime, validator, benchmark workload, fixture, schema or
other runner changes are allowed. Operator documentation belongs in the two scripts' usage/errors
and the issue evidence; do not create a second framework.

Fix the runner's single defect by keeping `if !` syntactically attached to the environment-prefixed
workload command. Preserve the no-argument interface and exact future workload command. Its
successful stub path must validate complete raw output before publishing a byte-identical accepted
copy through the same no-clobber temporary/rename rule. Workload failure, validator failure,
interruption, missing tools, partial output and any existing raw/accepted path return nonzero;
rejected raw bytes remain available and accepted output never appears. Each rejection reports the
raw hash when a raw file exists.

The promotion-only command is exactly:

```sh
bash scripts/promote-issue006-graph-benchmark.sh
```

It accepts no arguments or environment path overrides and contains no workload launch token. Run
it only after its implementation and all zero-workload tests pass. It is the sole Issue-030 action
allowed to publish the frozen carry-forward artifact.

## Hermetic proof matrix

Extend the existing shell test in scratch directories with a shadow `cargo` stub and copied
validators. Assert the exact stub invocation count and never call real Cargo or the graph benchmark
binary. Cover:

1. runner success: one stub launch, aggregate validation reached, raw preserved and accepted
   published once with identical bytes;
2. workload nonzero and interruption/partial output: exact status propagated, raw preserved when
   created, no accepted path;
3. validator nonzero/malformed aggregate: nonzero, source identity reported, no accepted path;
4. missing `jq`/validator/workload tool and invalid arguments: failure before a stub launch;
5. pre-existing raw, accepted, symlink or alias cases: no overwrite and no launch;
6. promotion success from a scratch copy of the exact frozen raw bytes: source and accepted size,
   hash, record termination and bytes match;
7. promotion missing/truncated/appended/hash-mismatched input, validator rejection, existing
   destination and symlink/alias attempts: nonzero with no source mutation or publication; and
8. mutations restoring the detached `if !` newline and inverting workload or validator status make
   the suite fail.

Tests may copy the preserved raw artifact into scratch, but may not write the real accepted path.
Before the actual promotion, record the real source identity again. If it is absent or differs,
STOP; do not reconstruct it from Issue-006 prose or run any workload.

## Ordered acceptance and evidence

1. `bash -n` passes for the runner, promotion command and shell test.
2. The hermetic success/failure/mutation matrix passes with real graph workload launches `0`.
3. Static scans prove the promotion command has no Cargo/binary launch and both public commands
   reject arguments and refuse overwrite.
4. The preserved source still has the frozen identity and the frozen aggregate validator returns
   zero without editing it.
5. Sol reviews the corrected status propagation, cleanup boundaries and no-clobber publication.
6. Only then run the promotion-only command once; verify raw and accepted byte identity and append
   exact shell/runtime versions, command/status matrix, counts/hashes, validator result, Issue-006
   historical counts, and both zero Issue-030 invocation counts to the spec.

No timing value is a gate or a performance claim. PASS requires the tooling tests plus exact
carry-forward publication. Failure to preserve identity, prove no-clobber publication, or close
within the two-attempt budget is a strict STOP; it never authorizes a benchmark rerun.
