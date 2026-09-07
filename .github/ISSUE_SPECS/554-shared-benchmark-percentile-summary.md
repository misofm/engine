# Share the complete benchmark percentile summary

One-line summary: Make `bench-support` the sole owner of the existing six-value `u64` benchmark summary and remove the duplicate rack and builtins summary implementations without changing any record, measurement, identity, or runner behavior.

## Problem

Audit #349 TOOL9 remains partial after PR #386. That PR centralized the nearest-rank operation and the byte-identical five-field console/wasm-console summary. The current tree still has two behaviorally equivalent six-field summaries:

- `tools/bench/src/rack.rs`: `Percentiles::{min,p50,p95,p99,p999,max}` at lines 890-913;
- `tools/bench/src/builtins.rs`: the same fields and the same sorted nearest-rank positions at lines 1752-1775.

Both sort an owned copy of nonempty `u64` observations and select min, 50%, 95%, 99%, 99.9%, and max. Their `50/100`, `95/100`, `99/100`, `999/1000` and `500/1000`, `950/1000`, `990/1000`, `999/1000` spellings are mathematically identical under the already shared `bench_support::stats::nearest_rank` law. The shared `bench_support::stats::Percentiles` at `tools/bench-support/src/stats.rs` already owns the same summary except for p99.9.

This issue is the smallest independently closable TOOL9 slice: extend that existing shared summary with p99.9 and make rack and builtins consume it. It does not claim that TOOL9's metadata residual is complete.

## Frozen behavior and boundaries

- Preserve nearest-rank Hyndman-Fan type 1 selection exactly. For sorted observations `1..=1000`, the tuple remains `(min=1,p50=500,p95=950,p99=990,p999=999,max=1000)`.
- Preserve one copied-and-sorted observation vector per summary and the empty-input panic. Do not move sorting, hashing, serialization, or evidence collection into a timed closure.
- Preserve rack's exact `OBSERVATIONS == 1000` precondition before summary construction. The shared summary must not weaken it.
- Preserve builtins' nonempty-sample precondition and its acceptance of any nonempty sample count.
- Preserve every emitted JSON key, key order, spelling, numeric value, numeric representation, unit, and schema version. In particular, rack keeps `p99_9_ns_per_frame`; builtins keeps its existing `p99_9_ns` field under its `ns_per_operation` units declaration.
- Preserve all metadata validation and fallbacks. This issue does not touch either local `Metadata` record projection.
- Preserve all fixture, candidate, binary, input, output, and manifest hash ownership and byte order. No digest call, input bytes, output bytes, fixture bytes, sealed constant, or validator may change.
- Preserve render and preparation workloads, warmups, measured batches, operation counts, allocation/audit scopes, and realtime behavior. No benchmark invocation or new timing is part of this issue.
- Keep rack and builtins JSON construction local. Their records have different schemas, identities, units, audit fields, nullability, and validators; a generic record writer is outside this issue.
- Internal names are unversioned. No new package, dependency, benchmark framework, fixture framework, schema generation, artifact promotion, or benchmark-runner change is authorized.

## Implementation scope

Allowed production/test paths:

- `tools/bench-support/src/stats.rs`
- `tools/bench/src/rack.rs`
- `tools/bench/src/builtins.rs`
- `scripts/check-bench-policy.sh`
- `scripts/test-bench-policy.sh`

Extend the existing shared `Percentiles` record with public `p999: u64` and compute it through `nearest_rank(..., 999, 1000)` in `from_samples`. Import and use that shared type in rack and builtins, deleting both local `Percentiles` definitions. Retain a rack-side assertion of the frozen 1,000-observation cardinality before calling the shared constructor. Do not migrate the other inline `u128` or subject-specific record projections in this issue.

Extend the existing benchmark policy only enough to make `tools/bench-support/src/stats.rs` the sole owner of a `Percentiles` summary under `tools/`, and add one mutation proving a reintroduced local summary is rejected. Do not redesign the policy checker.

## Objective gates

All gates are finite and untimed. Do not launch a benchmark or create benchmark output.

1. `cargo test --locked -p bench-support` passes. Its stats tests pin the six-field tuple for sorted `1..=1000`, an unsorted short sample, and the existing empty-input panic.
2. `cargo test --locked -p bench --bin bench rack::tests::nearest_rank_uses_the_frozen_one_thousand_observation_indices -- --exact` passes after the local type is removed and still pins `(1,500,950,990,999,1000)`.
3. Add or strengthen a pure builtins record test using the existing synthetic measurement so it asserts the exact min/p50/p95/p99/p99.9/max values and their existing JSON field spellings. Run that exact test with `cargo test --locked -p bench --bin bench <exact-test-name> -- --exact`.
4. `cargo test --locked -p bench --bin bench` passes. These are unit tests; do not invoke `bench rack`, `bench builtins`, or a shell benchmark runner.
5. `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh` pass. The mutation that inserts a second local percentile-summary owner must fail for the intended sole-owner reason before restoration.
6. `rg -n '^struct Percentiles|^pub struct Percentiles' tools --glob '*.rs'` reports only `tools/bench-support/src/stats.rs`. `rg -n 'p99_9_ns_per_frame|p99_9_ns' tools/bench/src/{rack,builtins}.rs` confirms both published field spellings remain.
7. `cargo clippy --locked -p bench-support -p bench --all-targets -- -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` pass.
8. The evidence record includes the exact base and implementation heads, individual command/status records, changed-path audit, and a source diff showing no metadata, digest, workload, fixture, record-key, unit, or schema-version changes.

## Attempt and review workflow

Luna high implements attempt 1. Sol xhigh reviews the exact implementation head against this issue, concentrating on the six percentile values, the distinct rack cardinality rule, unchanged record bytes for synthetic inputs, hash ownership, and the checker mutation's sensitivity. One consolidated PASS or FAIL counts as the attempt verdict. Up to three attempts total; do not weaken the frozen gates.

## Completion

Completion means the shared six-field summary is the sole implementation, both consumers emit the same values and field spellings for the same synthetic observations, all finite gates pass, Sol xhigh records PASS on the exact head, the issue/evidence checkpoint is upstream, required qualification succeeds, and the matching GitHub issue is closed and verified. Record TOOL9 as still partial until its separately mapped metadata collection residual receives delivered decisions.

## Numbered active slice

This issue is #554, the active TOOL9 slice alongside #542/#543/#552, based on maina3b4ed763c47658e10fc111e2cfcbd141c77f064. Sol high coordinates actual Luna high implementation; actual Sol xhigh verifies. Root owns exact-path checkpoints/pushes, GitHub synchronization and completed-worktree cleanup. Full residual map is retained in the brief evidence; TOOL9 remains partial after this slice.

## Attempt1 implementation and frozen gate evidence

Source86087ab13554410bf773fef204cee9bda6acd37b is pushed. Actual Luna HIGH implemented exactly the five approved paths and paused for root checkpoint. Bench-support31 tests, exact rack and builtins assertions, existing checker and negative mutations all passed. The initial guessed-spec read failure and formatting failure remain in the raw transcript.

After root checkpoint, actual Luna HIGH ran the remaining frozen checks without changes: full bench binary unit suite35 passed; strict all-target bench-support/bench clippy, sole-summary-owner census, JSON spelling census, formatting and diff checks all returned0. No benchmark or timed workload ran. Both actual launcher command records and terminal raw streams/reports are losslessly preserved in artifacts/issue554-attempt1 with byte counts and SHA-256 manifest. Sol XHIGH review remains pending; this does not close TOOL9's metadata residual.
