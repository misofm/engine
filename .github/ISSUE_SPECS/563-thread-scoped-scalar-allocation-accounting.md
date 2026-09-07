# Make scalar Point preparation allocation equality thread-scoped

Status: OPEN. This is the bounded qualification successor exposed by issue #555/#543 post-main run `34124982941`, job `101751589069`, at merge `b20b27d5e3ddc1a1246d003d857ced0bf0cba0c4`. Sol HIGH coordinates, Luna HIGH implements, and Astra MEDIUM performs the adversarial measurement-boundary review. Root owns Git, GitHub, checkpoints, delivery, and cleanup.

## Problem and smallest closable outcome

The post-main workspace test failed `preparation_resources_and_success_path_are_bounded`: direct preparation recorded 27 process-wide allocations, while wrapped preparation recorded 29, although both same-thread realtime-audit snapshots recorded exactly 27. The captured foreign-thread control separately proves that an allocation on another thread moves the process-wide allocator totals while leaving the measured-thread audit at zero. PR #553's exact merge candidate and merge commit have identical trees, and the same test passed required PR qualification `34124377148`; this is a latent nondeterministic measurement boundary, not an artifact, Rust-hex, DSP, or merge-tree change.

Issue #536 reduced libtest scheduling interference with `--test-threads=1` but explicitly did not make the child process single-threaded or immune to future worker activity. Its strict process-global equality therefore remains vulnerable. Preserve the failed run and do not rerun unchanged `main` to obtain green.

Deliver one shared current-thread byte/count view in the existing `bench_support::alloc` audited allocator and use it for the two scalar Point preparation equivalence assertions. Keep process-wide totals for allocator-installation/liveness and contamination diagnostics. The new authority must count the same allocation, deallocation, reallocation, and requested-byte events as the existing global counters for the calling thread, without allocation, locking, I/O, logging, syscalls, recursion, or changing the system allocation request.

## Exact scope

Allowed implementation paths:

- `tools/bench-support/src/alloc.rs`;
- `crates/host-core/tests/scalar_point_endpoint.rs`;
- this numbered spec and bounded issue evidence, owned by root.

No production host, protocol, effect, engine render, DSP, artifact, pin, browser result, Cargo manifest/lock, workflow, expected resource/PCM value, API, or benchmark change. Do not add an allocator, dependency, harness, environment key, retry, timing loop, or tolerance. The public surface may add only narrowly named current-thread counter snapshot/delta functions on the existing `Counters` value type.

## Frozen behavior and correction

1. Existing process-wide `counters()` and `delta_since()` behavior remains byte-for-byte semantic authority for global totals and allocator installation.
2. Add allocation-free current-thread snapshot/delta acquisition in `bench_support::alloc`. Its fields have exactly the same meanings as `Counters`: `alloc` and `alloc_zeroed` increment allocations and requested bytes; `realloc` increments allocations, reallocations, and requested bytes by the new size; `dealloc` increments deallocations.
3. Thread-local initialization must be const/no-allocation and safe when invoked inside `GlobalAlloc`; counter updates must not call the allocator or unwind. Preserve forwarding of the exact original pointer/layout/new-size contract to `System`.
4. Extend the existing deterministic foreign-thread control so process-global totals move while both current-thread allocator totals and the existing realtime-audit snapshot remain unchanged. Retain own-thread positive liveness for all fields that can be exercised.
5. In both scalar preparation gates, compare the current-thread `Counters` values exactly. Preserve the standalone equality contract and the controller wrapper's exact `+2 allocations`, `+2 deallocations`, `+0 reallocations`, and `+9 requested bytes`. Process-global captures remain printed and must dominate the same-thread allocation/byte activity, but are no longer the equality oracle.
6. Preserve all existing resource, ownership, admission, cancellation, PCM, zero-realtime-allocation/free, child-isolation, and nine-test assertions. Do not change production preparation behavior or claim the historical #536 or current failure's specific foreign allocator.

## Finite implementation and gates

Before editing, Luna HIGH records exact model/effort, argv, cwd, clean head/status, and both allowed source identities. Make one coherent implementation pass and stop at the first failed gate. Preserve raw stdout/stderr/status and source identities.

1. Focused `bench-support` allocator unit tests prove own-thread alloc/zeroed/realloc/dealloc accounting and a prestarted synchronized foreign-thread allocation that moves global totals but not the caller's current-thread totals. No sleeps or probabilistic scheduling.
2. Run the two exact scalar allocation selectors in debug and release with `--locked --features control-provider`, retaining the child execution and exact counter diagnostics.
3. Run the complete nine-test `scalar_point_endpoint` integration test once in debug.
4. Run affected strict Clippy, `cargo fmt --all --check`, `git diff --check`, and an exact two-path scope census.
5. Root checkpoints and pushes the coherent tranche before any further work. Astra MEDIUM reviews the allocator safety, thread attribution, strict assertions, preserved product behavior, failure evidence, and gates. A failed review receives at most the ordinary two further attempts; never loosen exact current-thread counts to a bound.
6. After Astra PASS, obtain required PR `qualification` SUCCESS on the exact reviewed head/current base, merge, verify issue closure and post-main `qualification` SUCCESS, then resume #560's preserved #558/#552 and #542 slots. Remove the clean successor worktree after delivery.

This issue repairs a mandatory post-main gate for delivered PR #553. It does not reopen #543/#555 product scope or count as an original audit finding, and it does not authorize starting an original open finding before the eight partials finish.

## Attempt 1 source checkpoint

Luna XHIGH completed the exact two-source correction from clean brief head `5e1f069e`. `bench_support::alloc` adds const initialized current-thread `Counters` and mirrors every existing global allocator event without changing the `System` call. Both scalar preparation tests now use exact current-thread totals, retain global diagnostics/dominance and the existing realtime audit, and preserve standalone equality plus the controller's exact `+2 allocations`, `+2 deallocations`, `+0 reallocations`, and `+9 requested bytes`.

All finite gates returned zero: 39 bench-support tests; both scalar selectors in debug and release through parent/child; the complete nine-test scalar endpoint suite; strict Clippy; formatting; diff check; and exact two-path census. The synchronized foreign-thread controls moved process totals while leaving the caller's allocator and realtime-audit totals at zero. Root independently audited and committed exactly the two allowed source paths at `4b5c624270befc4a2ca9d7dc338967fa5ee4e873`. Raw evidence and the implementation record are preserved in `artifacts/issue563-attempt1`. Astra MEDIUM adversarial review remains required before PR delivery.

## Attempt 1 adversarial verdict

Astra MEDIUM reviewed exact pushed head `c13d202df07ca875ec1e3bb003bd92dc7f333ba5` and returned **FAIL**. The host-level foreign-thread control captured and printed `foreign_current_thread` but did not assert that its four fields were zero, so frozen behavior 4 was observed but not enforced at that boundary. The separate bench-support unit control did enforce the property. All other reviewed requirements passed: exact two-source scope, const non-dropping TLS, bounded no-allocation updates, unchanged `System` forwarding, global counter semantics, exact standalone and controller preparation deltas, authenticated evidence, issue synchronization, and preservation of the original post-main failure without a rerun. Attempt 2 is limited to adding that exact host assertion and rerunning the proportional affected gates.

## Attempt 2 source checkpoint

Luna XHIGH added the single missing exact assertion that the host control's foreign-thread `Counters` snapshot equals `Counters::default()`. No allocator or product source changed. The complete nine-test scalar endpoint suite, strict affected-target Clippy, formatting, diff check, and exact one-source-path census all returned zero. Root audited and committed the one-source correction at `6edf0992d580340e9e1da8402f1ce2ec01cfcc88`; the raw proportional evidence and implementation record are preserved in `artifacts/issue563-attempt2`. Final Astra MEDIUM review remains required.

## Attempt 2 adversarial verdict

Astra MEDIUM returned **PASS** for the full final implementation at exact clean pushed head `f0f651ab82bdcb6c5ff860d1f9c3791ef9d122ce` against base `b20b27d5e3ddc1a1246d003d857ced0bf0cba0c4`. The sole attempt-two source delta is the exact missing `foreign_current_thread == Counters::default()` assertion, resolving the attempt-one finding. The review confirmed allocator event semantics, unchanged `System` forwarding, bounded const-TLS updates, both exact preparation contracts, authorized two-file source scope, authenticated evidence, local/GitHub issue synchronization, and preservation of the original failed CI attempt without a rerun. No blocking findings remain. Required PR and post-main qualification are the remaining delivery gates.
