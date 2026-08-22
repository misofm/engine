# Sol implementation brief — issue 071 locked fuzz build and benchmark artifact capture repair

## Decision and budget

**READY FOR ONE TERRA ATTEMPT; ZERO WORKLOADS AUTHORIZED.** This is a stateless CI-maintenance
repair, not a product, fuzzing or benchmark issue. Terra gets one coherent implementation/review
attempt and Sol verifies it without a correction retry. Failure of any frozen gate is final STOP
and requires a newly scoped successor.

Only these paths may change:

- `.github/workflows/ci.yml`;
- `fuzz/Cargo.lock`;
- `.github/ISSUE_SPECS/071-repair-locked-fuzz-builds-and-benchmark-artifact-capture.md`; and
- this brief.

Do not change README or the implementation plan: Issue 071 is not a product dependency and its
exact local/remote title will be indexed through the ordinary issue-sync workflow. Do not execute
or rerun CI, fuzzing, a benchmark script/binary, preflight, audit, trace, target qualification or
timing command.

## Frozen failure provenance

GitHub Actions run `32546875919`, head
`8d874a89eb949f34f180f78154486e105203914e`, failed in these exact places:

1. Jobs `96966794479` and `96966794629` both ran
   `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` and exited 101 with
   `cannot update the lock file .../fuzz/Cargo.lock because --locked was passed`. The unchanged
   manifest SHA-256 is
   `8b25d5c05a7f5c86b9fab83cde70e3e3c362b2aeb978f037d03ecd21b973c55b`; the stale lock SHA-256 is
   `0be0fbe18be5635a5bc40d12395dd50a4ea358ed822722c426a9559443a0873b`. The manifest names
   `miso-engine-effect-package`; the lock omits it. No fuzz target ran.
2. Job `96966794632` attempted the first `tee target/...` while `target/` was absent. `tee` reported
   `No such file or directory`, but the conformance producer still completed and printed its two
   measured rounds before the step failed. This historical unrecorded timed pipeline invocation is
   immutable incident evidence, not an accepted artifact and not permission to retry. The
   workflow SHA-256 at that head is
   `8f85718ec907dcabd808a8d9b5a1a0d8d4b6152a1ed60c8f8ced967b61fcab0a`.

The root lock SHA-256 is
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`; pinned toolchain file
SHA-256 is `f6a2b4a1eb8d7d2cad50aa25f028c86a20ca90191f2ff066e8cc34896e94ffd3`.
Both must remain unchanged.

## Exact repair

### Standalone fuzz lock

With the repository clean and Rust/Cargo 1.97.1 selected, run exactly the normal Cargo generator:

```sh
cargo +1.97.1 generate-lockfile --manifest-path fuzz/Cargo.toml
```

Accept only the generated `fuzz/Cargo.lock` diff. Do not edit versions/checksums manually, change
the manifest, use nightly to generate it, or merge root-lock content by hand. Verify the lock
contains `miso-engine-effect-package` and that `miso-engine-session-fuzz` lists all three unchanged
path dependencies. Then run pinned stable `metadata --locked` and `check --locked --bins`. Repeat
the generator once only as an idempotence check and require the lock hash/diff to remain unchanged.
None of these commands executes a fuzz target.

### Benchmark artifact directory

There are exactly three benchmark workflow shell pipelines containing `| tee target/`:

- `target/conformance-benchmark.jsonl`;
- `target/realtime-benchmark.jsonl`; and
- `target/session-benchmark.jsonl`.

Convert each step to its own literal shell block with this order:

1. `set -o pipefail`;
2. `mkdir -p target`;
3. the existing benchmark command and arguments piped to its unchanged `tee` destination; and
4. `test -f <destination> && test -s <destination>`.

Do not combine directory creation with the pipeline, put it after the producer, rely on another
step's directory, or change the workload command. With GitHub's existing `bash -e`, a failed
`mkdir` stops before pipeline construction; pipefail propagates either producer or `tee` failure.
The native-effect benchmark step has no `tee target/...` pipeline and remains byte-identical.

## Nonexecuting proof

Before final verdict:

1. Record the four authorized before/after path hashes and prove all other paths clean.
2. Run the pinned lock metadata/check/idempotence gates above; run no `cargo fuzz` or fuzz binary.
3. Statically require exactly three benchmark tee pipelines and verify the four-line ordering for
   each block. Require the native-effect step and all workflow triggers/job/workload arguments to
   remain unchanged.
4. In a unique temporary directory, use only a counted shell stub. Prove successful byte capture,
   upstream status-73 propagation with partial bytes retained, and zero launches when a synthetic
   `mkdir` fails. Do not copy or invoke repository benchmark scripts.
5. Run only workflow/docs static syntax checks, exact title/dependency checks and
   `git diff --check`. Record zero issue-owned fuzz, workload and timing invocations.

No `gh run rerun`, manual dispatch or CI rerun is permitted. A later ordinary main-branch CI event
may provide external confirmation, but it is not created, awaited or counted by Issue 071.

## Stop conditions and verdict

STOP on a manifest/root-lock/product change, manual lock editing, unresolved `--locked` update,
workflow command/round/schedule drift, missing per-pipeline guard, a real workload/fuzz launch, or
any path beyond the four-file boundary. Do not repair another CI failure discovered during this
attempt.

PASS requires all local nonexecuting gates and exact evidence with:

- `historical_unrecorded_benchmark_pipeline_invocations=1`;
- `issue071_fuzz_run_invocations=0`;
- `issue071_benchmark_workload_invocations=0`;
- `issue071_timed_benchmark_invocations=0`; and
- `issue071_ci_rerun_requests=0`.

## Terra attempt 1 completion record

PASS on candidate `81c1013`: pinned Cargo 1.97.1 generated and then reproduced fuzz lock hash
`bf56130a8ea92bae516074ee60c40eb7740c04fe91b78a16b95f6a014d12e9f6`; pinned standalone
metadata and `--locked --bins` check passed. The generated lock now carries the unchanged
effect-package closure. The three workflow tee pipelines each have ordered local `pipefail`,
directory creation, and nonempty-artifact guards; static and scratch-only producer proofs passed.
The root lock, fuzz manifest, and toolchain remained frozen; exact-path/diff checks passed.

`historical_unrecorded_benchmark_pipeline_invocations=1`;
`issue071_fuzz_run_invocations=0`; `issue071_benchmark_workload_invocations=0`;
`issue071_timed_benchmark_invocations=0`; `issue071_ci_rerun_requests=0`.
