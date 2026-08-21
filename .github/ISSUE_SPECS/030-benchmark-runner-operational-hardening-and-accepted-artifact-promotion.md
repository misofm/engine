# 030 Benchmark runner operational hardening and accepted-artifact promotion

## Sol briefing status — 2026-08-21

**READY for Terra attempt 1.** The authoritative tracked brief is
`BRIEFS/030-benchmark-runner-operational-hardening-and-accepted-artifact-promotion.md`. This small
tooling closure permits exactly two total attempts: one Terra implementation/review and, only if
needed, one bounded Sol correction/review. A second failure stops and preserves all artifacts.
Issue 030 authorizes **zero** graph benchmark workload or timed invocations in either attempt.

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
