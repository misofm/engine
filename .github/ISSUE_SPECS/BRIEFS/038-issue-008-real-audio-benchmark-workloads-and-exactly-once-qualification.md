# Sol implementation brief — issue 038 Issue-008 real audio benchmark workloads and exactly-once qualification

## Decision, dependency and attempt budget

**READY FOR TERRA ATTEMPT 1 ONLY AFTER ISSUE 037 PASSES.** This issue permits exactly one Terra
implementation/review attempt and at most one bounded Sol correction/review. A second failure
stops. It changes benchmark tooling/fixtures only, never engine semantics. Timed invocation count
starts at zero and remains zero until root Sol explicitly authorizes the exact runner after every
nonbenchmark gate passes on one committed Issue-037 candidate.

## Replace, do not bless, the placeholder

Delete the byte-fold behavior from `miso-engine-rack-bench`. Its three names are not evidence.
Build fixed runtimes through production APIs:

- eight separate scalar 48-kHz/128-frame builtin TPT tracks;
- those same tracks in one host-selected production x86 eight-lane builtin bank; and
- the Issue-037 12-track production graph with one full bank, an identity/missing position, stable
  tail and incompatible scalar fallback.

Each runtime is fully prepared before timing, receives the same frozen asymmetric dual-mono input
sequence and carries continuous state. Fill input before `Instant::now`; stop timing immediately
after the render; hash output afterward. Every measured round starts from the same declared state.

## Frozen measurement and schema

The shell runner owns one untimed warmup and measured rounds 1 and 2. Each measured round emits one
record per workload after exactly 1,000 observations. Divide each elapsed integer nanosecond count
by 128 frames, sort, then use nearest-rank min/p50/p95/p99/p99.9/max. Emit six records total. There
is no threshold, tuning or retry.

Implement the exact schema-v2 contract from Issue 038: identity/round, workload shape and selected
backend, integer ns/frame percentiles, candidate/binary/fixture/input/output hashes, zero render
error/forbidden-operation fields with exact total, complete environment metadata using JSON null
plus an honest sorted `missing_metadata`, and `descriptive_only=true`. Single and aggregate
validators reject unknown/missing keys, wrong types, wrong workload-ID mapping, wrong shapes,
cardinality/duplicate/round errors, unordered percentiles, hash drift, nonzero/mistotaled audit,
and dishonest metadata.

## Safe runner and preflight

`scripts/run-rack-benchmark.sh` accepts no arguments, uses `set -euo pipefail`, quoted paths from
`BASH_SOURCE`, fixed commands/environment and no `eval`, sourcing or unsafe splitting. It refuses
before launch if any raw/accepted/stderr/disposition path exists. It precollects metadata, performs
one warmup and exactly two measured rounds, writes stdout directly to a newly created raw file and
never edits/deletes raw bytes. Failure writes a checksummed disposition and preserves raw/stderr;
success creates a byte-identical accepted copy and PASS disposition. There is no resume/retry.

Preflight builds/hashes the release binary, validates every frozen input, proves production API
reachability without timing, runs synthetic record/aggregate mutations, and tests argument,
overwrite, output persistence, shell nonzero/pipe failure and interruption disposition in scratch.
It launches no audio workload, creates no real Issue-038 artifact and reports
`workload_launches=0`.

## Ordered authorization gates

1. Issue 037 has an upstream synchronized Sol PASS and exact candidate commit.
2. Format; benchmark unit tests; fixture/input coverage; single/aggregate validator mutations.
3. Runner negative/artifact lifecycle tests and zero-launch preflight.
4. Locked workspace check/test, warning-denied Clippy/rustdoc and applicable policies.
5. Candidate/binary/runner/validator/fixture hashes sealed on the clean committed tree.
6. Sol source review verifies real scalar/bank/graph calls and exact observation/round ownership.

Only after all six pass may root Sol authorize one exact invocation. Failure consumes it; preserve
bytes and stop. Do not invoke the binary directly, rerun, tune or substitute a workload.

## Completion and nonblocking rule

PASS requires six valid records, raw and accepted byte identity, PASS disposition, exact one
invocation/one warmup/two measured rounds, and recorded rough ratios without optimization claims.
Issue 038 is not a dependency of Issues 009 or 010. It may gate Issue 026 release qualification;
no other issue gains authority to rerun this benchmark.
