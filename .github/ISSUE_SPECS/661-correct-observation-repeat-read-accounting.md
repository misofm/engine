# Correct observation stress accounting for repeated reads

Parent: #560  
Dependency consumer: #659  
Baseline: `561c4345614c295d99dd6315f668a060ca8f3531`

## Problem

`engine/tests/observation_transport.rs::a_million_windows_are_read_whole_and_in_order`
increments `reads` for every successful snapshot. The reader is explicitly allowed
to observe the same published sequence more than once, but the test later asserts
`reads <= WINDOWS`. A fast reader can therefore fail a correct conflating cell by
reading an unchanged publication repeatedly.

#659 attempt 2 encountered this status-101 release failure without changing the
test or observation transport. Source inspection establishes the causal mismatch;
the failed #659 suite remains a failure and is not reclassified.

## Smallest closable slice

Own only:

- `crates/engine/tests/observation_transport.rs`
- this issue spec and compact gate record
- concise #559/#560 coordination records

Track successful snapshots for diagnostics, but count sequence advances
separately. On every advance, accumulate `missed_windows(sequence)` before
acknowledging it. Assert the exact accounting identity
`advances + missed_total == newest`, which equals `WINDOWS` at completion. Equal
successive reads must not increment `advances`; torn/regression detection,
wait-free writer behavior, final newest-window proof, and writer-view bound remain.

No observation implementation, synchronization, ordering, production source,
public API, benchmark, timing threshold, artifact, manifest, lockfile, workflow,
or unrelated test change is in scope. Do not loop or retry the stress test to seek
a favorable schedule.

## Attempt 1 gates

Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole executor. Before editing,
prove exact clean head/upstream and that
`/tmp/issue661-attempt1-source-evidence` is absent and not a symlink. Retain exact
cwd/head, toolchains, literal argv/environment, separate stdout/stderr and numeric
status for every command. Run once in order, stopping at the first failure:

1. focused debug stress test by exact name;
2. focused release stress test by exact name;
3. `cargo test --locked -p engine -p target-smoke`;
4. `cargo test --locked --release -p engine -p target-smoke`;
5. `cargo clippy --locked -p engine -p target-smoke --all-targets --all-features -- -D warnings`;
6. `cargo fmt --all -- --check`;
7. `bash scripts/check-workspace-policy.sh`;
8. `bash scripts/check-realtime-policy.sh`;
9. `bash scripts/test-realtime-policy.sh`;
10. `git diff --check` and an owned-path/payload census.

The focused commands are:

```text
cargo test --locked -p engine --test observation_transport a_million_windows_are_read_whole_and_in_order -- --exact
cargo test --locked --release -p engine --test observation_transport a_million_windows_are_read_whole_and_in_order -- --exact
```

Full streams remain temporary. Git retains only the compact command/status/result
record and no compiler stream, `.ll`, assembly, object, archive, binary, target
output, `rlib`, or `rmeta`.

## Review and dependency release

Astra LOW reviews the exact source checkpoint. Any correction consumes the next
of at most three attempts and requires fresh scope authorization. After source
PASS, use exact-head/current-main PR review, required qualification, guarded merge
parents, post-main qualification, GitHub synchronization, and clean-worktree
removal.

Only delivered #661 may release #659's final gate plan. #659 then reruns its
release engine/target-smoke coverage and previously unexecuted gates under a fresh
reviewed final-attempt scope; no #659 failure is rewritten.
