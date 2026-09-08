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

Use one test-local accounting helper for the stress loop and its final read. Add
the deterministic control
`repeat_reads_and_final_gap_have_exact_accounting`: publish sequence 1, record
multiple reads of that unchanged sequence, then publish sequence 4 and process it
through the same final-read accounting path. The control must prove that repeats
leave the advance count at 1, the later read raises it to 2, the known missed
count is 2, and `advances + missed_total == newest == 4`. Run this control once;
it is not a stress retry.

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

1. focused debug deterministic control by exact name;
2. focused debug stress test by exact name;
3. focused release stress test by exact name;
4. `cargo test --locked -p engine -p target-smoke`;
5. `cargo test --locked --release -p engine -p target-smoke`;
6. `cargo clippy --locked -p engine -p target-smoke --all-targets --all-features -- -D warnings`;
7. `cargo fmt --all -- --check`;
8. `bash scripts/check-workspace-policy.sh`;
9. `bash scripts/check-realtime-policy.sh`;
10. `bash scripts/test-realtime-policy.sh`;
11. `git diff --check` and an owned-path/payload census.

The focused commands are:

```text
cargo test --locked -p engine --test observation_transport repeat_reads_and_final_gap_have_exact_accounting -- --exact
cargo test --locked -p engine --test observation_transport a_million_windows_are_read_whole_and_in_order -- --exact
cargo test --locked --release -p engine --test observation_transport a_million_windows_are_read_whole_and_in_order -- --exact
```

Full streams remain temporary. Git retains only the compact command/status/result
record and no compiler stream, `.ll`, assembly, object, archive, binary, target
output, `rlib`, or `rmeta`.

## Initial scope review

Astra LOW returned scope FAIL on the initial brief. The accounting law and
one-file boundary were sound, but a scheduler-dependent stress run did not prove
that equal reads or a skipped final publication would occur. This revision adds
the deterministic repeated-read/final-gap control above and requires it to share
the production-test accounting helper. No implementation attempt was authorized
or consumed by that review.

## Corrected scope authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed head
`c40cef20349b4458f2fc80f8e89aebf87d5061a3`, delivered main
`561c4345614c295d99dd6315f668a060ca8f3531`, and tracker
`8222ec80b2252e6b343ef274dd718728ef2d5c8c`. GitHub matches and the attempt-1
evidence path is absent. Hypatia is authorized for the one-file attempt above:
preserve all existing safety/order assertions, count only strict sequence
advances, compute missed windows before acknowledgment, and run the amended gate
sequence once with complete retained records. Stop at the first failure without
correction or retry.

## Review and dependency release

Astra LOW reviews the exact source checkpoint. Any correction consumes the next
of at most three attempts and requires fresh scope authorization. After source
PASS, use exact-head/current-main PR review, required qualification, guarded merge
parents, post-main qualification, GitHub synchronization, and clean-worktree
removal.

Only delivered #661 may release #659's final gate plan. #659 then reruns its
release engine/target-smoke coverage and previously unexecuted gates under a fresh
reviewed final-attempt scope; no #659 failure is rewritten.

## Attempt 1 result

Hypatia produced the one-file source checkpoint `27769c0d`. The deterministic
control, debug and release stress tests, full debug and release `engine` plus
`target-smoke` suites, and strict Clippy returned 0. Gate 7,
`cargo fmt --all -- --check`, returned 1 because rustfmt requires the new import
list to be reordered. The attempt stopped there; gates 8-11 did not run. An
initial capture-wrapper invocation failed before Cargo because its evidence
directory had not yet been created; the corrected evidence run then retained
complete records for gates 1-7. The 23-line `SHA256SUMS` file at
`/tmp/issue661-attempt1-source-evidence` incorrectly includes itself because the
shell opened it before enumerating inputs. Its external hash is
`b7eb1536a010b5c5b7aaf32b54fac919444406c1a545c7cf4f92f1c159f75b08`.
The other 22 entries verify, but the self-entry does not; preserve the file
unchanged and do not describe it as a verified manifest.
No compiler payload or generated build output entered Git. Attempt 1 is FAIL;
formatting correction and remaining gates require fresh Astra LOW attempt-2
scope review.

## Attempt 1 review and attempt 2 authorization

Astra LOW returned **ATTEMPT-1 FAIL** and bounded **ATTEMPT-2 SCOPE PASS** at
clean record head `04823cfa7841b7db6e9c8e472fdbebf076c2d699` and source
checkpoint `27769c0df137f967302c99a6c39ffc184565c584`. The accounting
repair is sound: equal reads do not advance, every advance including the final
read counts gaps before acknowledgment, the deterministic control discriminates
repeats and the final gap, and the existing torn/regression/latest/writer gates
remain.

Attempt 2 owns only rustfmt's exact import-order correction in the same test file,
then runs once, stopping at the first failure:

1. `cargo fmt --all -- --check`;
2. `git diff --check` and owned-path/payload census;
3. `bash scripts/check-workspace-policy.sh`;
4. `bash scripts/check-realtime-policy.sh`;
5. `bash scripts/test-realtime-policy.sh`.

Retain attempt 1's successful behavioral, full-suite, and strict-Clippy evidence;
the formatting-only change does not rerun them. Use the fresh absent
`/tmp/issue661-attempt2-source-evidence` path and create a manifest that excludes
itself, recording the manifest hash separately. No production edit, artifact,
compiler payload, correction, or retry is authorized.

## Attempt 2 result

Hypatia applied only the authorized import ordering at source checkpoint
`7fa851db196ca5e5eab84a41c688a801bc7effa8`. Format, diff/owned-path
census, workspace policy, realtime policy (42 marked regions in 12 files), and
the realtime policy mutation suite each returned 0 in one run. The fresh
17-entry manifest excludes itself, verifies completely from its evidence
directory, and hashes to
`075e44bec52f2abcd27eb46178091addfde696a2d3ce0bf6222e4eb1157bdae0`;
its full streams remain under `/tmp/issue661-attempt2-source-evidence`. Combined
with the retained attempt-1 behavior, full-suite, and strict-Clippy passes, all
authorized source gates are green. No production source, compiler payload,
artifact, lockfile, or unrelated path changed. Exact-source Astra LOW review is
pending before delivery qualification.

## Source verdict

Astra LOW returned **SOURCE PASS** at exact clean pushed record head
`eefbad4f42e493417c449fb4c05f4189637c2da8` and product source
`7fa851db196ca5e5eab84a41c688a801bc7effa8`. Only import ordering changed
after the reviewed accounting repair; all attempt-2 evidence verifies, prior
behavior/full-suite/Clippy evidence remains applicable, and the attempt-1 failure
and manifest defect remain preserved. Proceed to exact-head/current-main PR
readiness, required qualification, guarded merge review, and successful post-main
qualification. No artifact qualification is indicated for this test-only change;
retain temporary evidence through delivery cleanup review.
