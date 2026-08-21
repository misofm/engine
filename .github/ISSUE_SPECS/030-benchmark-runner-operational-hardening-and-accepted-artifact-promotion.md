# 030 Benchmark runner operational hardening and accepted-artifact promotion

## Status — 2026-08-21

**FINAL PASS.** The authoritative tracked brief is
`BRIEFS/030-benchmark-runner-operational-hardening-and-accepted-artifact-promotion.md`. This small
tooling closure used its Terra implementation/review and one bounded Sol correction/review. Issue
030 performed **zero** graph benchmark workload or timed invocations.

## Outcome

Make the graph benchmark shell wrapper report workload success and failure correctly, validate
before promotion, and promote the already measured validator-valid issue-006 artifact without
rerunning or tuning the graph workload.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark,
or inherit V1/legacy work. Realtime and graph behavior are outside this tooling-only issue.

Issue 006 exhausted its three-attempt workflow after its sole authorized benchmark command. The
workload itself completed all six required JSONL records and the frozen validator accepts the raw
bytes, but the wrapper exited before validation/promotion because `if !` ended before the
environment-assignment/`cargo run` command. The failed attempt remains failed and must not be
rewritten as a pass. Issue 006 was subsequently accepted under a fresh product-focused rescope;
this operational follow-up is deliberately nonblocking for issues 007–010 and release graph
semantics.

The preserved input is
`target/issue6/graph-compiler-benchmark.raw.jsonl`: exactly 10,364 bytes, six LF-terminated records,
and SHA-256 `c03f1bc0399f0b9dea3a5c94c13a468512d2fcb2a2805c450c83110b56d623b5`.
`scripts/graph-benchmark-validator.jq` accepts it with exit 0. This identity is carry-forward
evidence, not permission to synthesize, edit, or silently replace the file.

This issue follows a new Sol-approved brief -> Terra attempt 1 with evidence -> at most one bounded
Sol correction/review workflow. It does not continue issue 006's exhausted attempt count.

## Scope

Repair the graph runner's shell control flow; add bounded hermetic success, workload-failure,
validator-failure, existing-output, missing-tool, and interrupted/partial-output tests; preserve raw
bytes and report their hash on any rejection; and provide an explicit validation/promotion path for
the exact preserved issue-006 raw artifact. Promotion must occur only after byte identity and the
frozen aggregate validator both pass. The promoted artifact must be byte-for-byte identical to the
raw input.

## Required public interfaces/contracts

`scripts/run-graph-compiler-benchmark.sh` remains the no-argument future-workload entry point and
must return the wrapped workload's actual success/failure meaning. Its success path validates a
complete raw file before an atomic same-filesystem promotion to
`target/issue6/graph-compiler-benchmark.jsonl`; its failure paths preserve raw output, never publish
an accepted path, and return nonzero.

A separate no-argument command,
`scripts/promote-issue006-graph-benchmark.sh`, consumes the exact preserved raw artifact without
containing or launching `cargo run`. It must refuse an unexpected size/hash,
missing or malformed input, validator rejection, an existing destination, or source/destination
aliasing. It must not normalize line endings, reserialize JSON, or modify record values. Successful
promotion copies through a same-filesystem temporary and atomic rename while leaving the historical
raw source in place. It records that issue 006's original runner invocation count remains one and
that issue 030 performed zero timed workload invocations.

## Deliverables

Corrected shell wrapper; explicit promotion-only path; hermetic shell fixtures/stubs and mutation
tests that launch no benchmark workload; concise operator documentation; and an evidence record
containing shell/runtime versions, tested exit/status matrix, source/destination hashes and byte
counts, validator result, and workload invocation count.

No Rust package or crate is required. If the approved brief nevertheless identifies a justified
new package, its directory/package must use `miso-engine-` and its Rust crate identifier must use
`miso_engine_`.

## Explicit non-goals

Changing graph/compiler/runtime code or semantics; changing benchmark workloads, iterations,
rounds, record schema, validator rules, fixture identities, timings, or environment fields;
optimizing or comparing performance; adding timing thresholds; rerunning issue 006 merely to obtain
a preferred filename; relabeling the failed issue-006 attempt as passing; modifying another
benchmark runner without a separately amended scope; or any V1/legacy inspection.

## Dependencies by exact issue title

- Deterministic graph compiler, sends, submixes, sidechains, and PDC

This dependency is one-way. No launch/render issue depends on this tooling follow-up.

## Hazards/decisions

Shell negation and multiline environment assignments are syntax-sensitive; tests must observe the
wrapped command's true status rather than infer it from artifact presence. Validation and promotion
are separate from measurement. Promotion is atomic and never overwrites. Test doubles must be
obviously hermetic and must prove their invocation count; mutation tests may not invoke the Rust
benchmark binary.

The existing complete two-round issue-006 measurement is sufficient. The default and preferred
issue-030 workload invocation count is **zero**. If the preserved raw artifact is unavailable or
fails its frozen identity, stop and report that promotion is unavailable; do not create a
benchmark/retry loop or weaken identity/validation. New descriptive measurements are outside this
issue and require a separately scoped decision.

## Acceptance gates with objective measurements

`bash -n` and the repository's applicable shell policies pass. Hermetic tests prove: a successful
stub workload reaches aggregate validation and publishes once; workload failure preserves raw and
never publishes; validator failure preserves raw and reports SHA-256; existing raw/accepted paths
are never overwritten; missing tools and partial/interrupted output fail; and every path returns the
documented status. Mutation tests that restore the split `if !` newline defect and invert workload
or validator status must fail.

For carry-forward promotion, the input has the exact frozen size/hash above, the unmodified
`scripts/graph-benchmark-validator.jq` returns 0, and the accepted output has identical size/hash and
six LF-terminated records while the raw source remains byte-identical at its historical path.
Evidence reports zero issue-030 timed workload invocations. No timing value is an acceptance
threshold.

## Target matrix

GNU Bash on the supported native development/CI host class. The JSON validator remains `jq`-based.
No engine, mobile, browser, Wasm, render-thread, or DSP target changes.

## Required evidence

Shell syntax/policy report; hermetic exit/status and artifact matrix; mutation-test report; exact
pre/post byte counts and SHA-256; frozen-validator transcript; explicit timed-workload invocation
count; and confirmation that issue 006's failed attempt record was not edited or relabeled.

## Terra attempt 1 — shell-only no-promotion checkpoint (2026-08-21)

Candidate input was `b742c08`. The runner now retains the environment-prefixed workload command
inside one `if ! (...)` condition while a scratch status file preserves the wrapped nonzero status.
It validates raw JSONL before a same-directory temporary copy/no-clobber publication and keeps raw
bytes on every rejection. New `scripts/promote-issue006-graph-benchmark.sh` is no-argument,
contains no Cargo or benchmark-launch token, verifies the frozen source identity plus aggregate
validator before a temporary copy/no-clobber publication, and never moves or edits raw bytes.

`bash -n` passed for runner, promotion helper and hermetic test. The hermetic scratch suite passed:
stub success (one counted stub launch and byte-identical raw/accepted), exact workload status `73`,
interrupted partial status `130`, validator failure with raw hash reporting, existing raw/accepted
symlink refusal, missing validator/jq refusal before launch, promotion success from a scratch copy,
truncated/appended/validator-rejected/symlink/existing-destination promotion rejection, and both
detached-`if !` and inverted-status mutations. It launched zero real Cargo/Rust graph workloads.

Read-only real-source identity remains `10364` bytes, six LF records and SHA-256
`c03f1bc0399f0b9dea3a5c94c13a468512d2fcb2a2805c450c83110b56d623b5`; the unchanged aggregate
validator returned zero. The real accepted path remains absent. GNU Bash was
`5.2.21(1)-release`; jq was `1.7`; `git diff --check` passed.
`issue_030_workload_invocations=0`; `issue_030_timed_invocations=0`. No real promotion was made,
and Issue 006's failed historical runner record was not edited or relabeled. Terra verdict:
**PARTIAL / GREEN SHELL CHECKPOINT** — ready for root commit/push and Sol review before the sole
promotion-only action.

## Sol correction/review attempt 2 — pre-promotion PASS (2026-08-21)

Sol reviewed clean Terra checkpoint `6d34956`. Runner status propagation, raw preservation,
validator ordering, temporary-copy publication and no-clobber checks conform to the brief. The
promotion helper has no Cargo/benchmark launch token and accepts only the frozen no-argument path.
A bounded test-only correction made the interruption case terminate the counted scratch stub with
real `SIGTERM` and require status 143, added the missing-Cargo and missing-raw gates, and changed
the detached-`if !` mutation into the original syntactically valid but semantically wrong shape.

`bash -n`, the complete hermetic lifecycle/mutation suite, exact source size/line/LF/hash checks,
the frozen aggregate validator, executable/static launch-token scans and `git diff --check` pass.
The real accepted path is still absent. Real Cargo/Rust graph workload launches: **0**. Scratch
promotion tests only; real promotions: **0**. `issue_030_workload_invocations=0` and
`issue_030_timed_invocations=0` remain exact.

**Verdict: PASS TO THE SOLE NON-TIMED PROMOTION ACTION.** After this test-only correction is
committed and the candidate is clean/upstream-synchronized, root may recheck the frozen source
identity, validator result and absent destination, then invoke exactly once:

```sh
bash scripts/promote-issue006-graph-benchmark.sh
```

Do not invoke the graph runner, Cargo benchmark package or binary. Overall Issue-030 PASS remains
pending byte-identical accepted-output verification and final evidence after that promotion.

## Final carry-forward promotion and Sol verdict — 2026-08-21

Root invoked the sole authorized non-timed promotion command exactly once on clean, pushed candidate
`97b245bce47ed46e20a27388a935613a8b89f98c`:

```sh
bash scripts/promote-issue006-graph-benchmark.sh
```

The command succeeded without invoking Cargo or a benchmark workload. Read-only post-promotion
verification proved that both
`target/issue6/graph-compiler-benchmark.raw.jsonl` and
`target/issue6/graph-compiler-benchmark.jsonl` are regular non-symlink files of exactly 10,364
bytes and six LF-terminated records, with SHA-256
`c03f1bc0399f0b9dea3a5c94c13a468512d2fcb2a2805c450c83110b56d623b5`. `cmp` reports byte
identity, and the frozen aggregate validator accepts both files. The raw source remains unchanged.
The ignored accepted artifact leaves the Git worktree clean.

Final counters are exact: `issue_030_promotion_invocations=1`,
`issue_030_workload_invocations=0`, and `issue_030_timed_invocations=0`. Issue 006's historical
workload invocation count remains one, and its failed exactly-once runner attempt remains recorded
without relabeling.

**FINAL SOL VERDICT: PASS.** Issue 030 closes its shell-runner hardening and exact carry-forward
artifact-promotion contract. This is not a new measurement, a performance threshold, or authority
to rerun the graph benchmark.
