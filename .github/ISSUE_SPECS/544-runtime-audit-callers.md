# Execute the two remaining uncalled runtime audit subjects

Issue #544; parent #349 TOOL-3; current base e5b86cf315487fcc602db420dc1a6121f1ac4837.

TOOL-3 originally identified six `tools/audit` dispatcher subjects: `builtins-fixture`, `capi`, `fixture-builtins-listening`, `fixture-source`, `source-duration`, and `unfused-fma`. The historical 4,720-line total identifies that population but is not a current dead-code measure.

Current inspection establishes:

- `fixture-builtins-listening` has a live operator caller.
- `fixture-source` executes its complete checker through the required release audit tests.
- `builtins-fixture` remains the guarded author/check maintenance interface for fixtures consumed by the live builtins audit, with its read-only checker mutation-tested.
- `unfused-fma` remains the ruling-backed reproducer and supplies the intentionally fused reference arms registered by the required unfused seal.
- `capi` and `source-duration` contain unique executable claims that neither required qualification nor their unit tests currently exercise.

Add exactly two invocations to the existing `audit-native` release job in `.github/workflows/qualification.yml`:

1. Run `target/release/audit capi` and validate its single JSON record: schema/kind, 100,000 calls, 48 kHz/128 frames, stable output address, zero render errors, and all nine forbidden-operation counters plus total violations equal zero.
2. Run `target/release/audit source-duration` and validate its single JSON record: one-minute and three-hour frame identities, equal allocation layout/source report/graph report, the current exact 17-entry/6,416-byte accounting contract, and `timed_benchmark_invocations == 0`. RSS remains descriptive; introduce no unstable RSS threshold.

Allowed implementation path: `.github/workflows/qualification.yml` only, plus the root-owned numbered spec/evidence.

Do not edit `tools/audit` source, its dispatcher, Cargo dependencies, fixtures, DSP code, scripts, historical rulings, benchmark artifacts, or unrelated orphan-tool cleanup. Do not delete or rename any of the six subjects. Do not add a generic reachability framework.

Implementation: Luna high, coordinated by Sol medium; verification: Astra medium because the change governs execution of realtime C ABI and source resource correctness audits. Root owns issue creation, worktrees, checkpoints, GitHub synchronization, delivery, and cleanup.

### Objective gates

- `cargo test --locked --release -p audit` remains green.
- The `capi` record satisfies every field above and exits successfully.
- The `source-duration` record satisfies every field above and exits successfully.
- `cargo fmt --all -- --check` and workflow syntax validation pass.
- Required `qualification` succeeds on the exact reviewed commit.
- Diff inspection proves only `.github/workflows/qualification.yml` and root-owned issue evidence changed.
- Zero benchmark invocations, fixture regeneration, human listening, or performance claims.


## Execution

Retain the six-subject disposition evidence in the numbered issue. No historical dead-line count or speedup claim. Build with --locked, capture actual argv/source/exit status and both original JSON records before validation. Existing audit assertions remain authoritative; do not weaken them to add a caller. Preserve workflow routing, trigger, expectation table and single required qualification context. Use one existing release audit build and no benchmark invocations. If either formerly uncalled audit fails, preserve its raw evidence and obtain a bounded correction/rescope ruling before editing outside the workflow. Maximum three implementation attempts. Root checkpoints/pushes each focused-green tranche, then obtains exact-head/current-base review and actual qualification success before merge, issue sync/closure and completed-worktree removal.
