# Prepare qualification dependencies before offline policy checks

GitHub: https://github.com/misofm/engine/issues/638

Parent delivery: #635. Coordination: #559 and #560. Blocks post-main qualification for merge `d47b62ba0dfcaf5c1525510aaa7789fa8e0acb94`. Queued peer: #636.

PR #637 required qualification run `34249662644` passed at exact documentation/evidence head `72b3908cc9b3bfbe7fc7410abf5c22adc63c573f`. The guarded merge has exact parents prior main `62045f40048ec230298fe0fd3935da3333f90b83` and reviewed head `72b3908c`. Post-main run `34250520726` failed in two prerequisite paths before product assertions: an `actions/download-artifact` intermediary 403, and `scripts/check-realtime-audit-leak.sh` invoking `cargo tree --offline` for `builtins-compiler` without cached `wasi v0.11.1+wasi-snapshot-preview1`. Astra LOW authorized one failed-job rerun. The artifact download recovered; the identical offline Cargo miss recurred, so no further rerun is authorized.

This issue owns the smallest cache-preparation correction for the required `fmt, clippy, doc, and hermetic policy gates` job. It must make every locked dependency needed by that job's later offline graph inspections available before those inspections run, while keeping the inspections offline and fail-closed. The artifact API 403 is preserved as recovered infrastructure history and is not part of this correction.

Sol HIGH coordinates the stateless record, checkpoints, GitHub and delivery. Luna HIGH or XHIGH implements. Astra LOW performs scope, source, exact-head, CI and post-main reviews. #635 and this issue are the two active issue slots. #636 is explicitly queued and may not resume until #635 closes; it owns no workflow/cache path.

## Smallest closable slice

Initial implementation scope is `.github/workflows/qualification.yml` only, inside the required `fmt, clippy, doc, and hermetic policy gates` job. Add one explicit locked dependency-hydration step after the pinned Rust toolchain/cache setup and before any policy command that deliberately invokes Cargo with `--offline`. The proposed command is:

```text
cargo fetch --locked
```

Do not remove or alter `--offline` from `scripts/check-realtime-audit-leak.sh`, weaken that checker, change `CACHE_ON_FAILURE`, change cache keys, make failures nonblocking, add retry loops, or edit product source, manifests, dependencies, `Cargo.lock`, artifacts, pins, benchmarks or compiler output. If Astra finds that a workflow-only `cargo fetch --locked` cannot establish the needed cache closure, stop for a scope amendment before editing another path.

The correction must retain qualification's static route/result table and required `qualification` verdict semantics. It may not add a separate workflow, status context or CI invocation. The existing required workflow run on the correction PR is the end-to-end proof that a clean runner hydrates the dependency before offline policy inspection. After guarded merge, its post-main run must also pass before #635 closes.

## Objective gates

Before implementation, Astra LOW must verify the two failure logs, unchanged product tree, current-main applicability, exact workflow insertion point, locked-fetch sufficiency, #635/#636 ownership and that no rerun is disguised as implementation evidence. Luna then makes one workflow-only tranche. Proportional local checks are YAML/diff hygiene plus existing qualification router/static-expectation policy and any directly applicable workflow-policy test; do not run product tests, builds, benchmarks, artifact builders or compiler captures locally.

Astra must review the exact clean pushed source before a PR. Required PR qualification must pass at the exact reviewed head. Merge only under a fresh guarded Astra review with unchanged main/head and verify exact parents. The first new post-main qualification must pass without manual rerun. Then synchronize and close this issue and #635, and remove their clean delivered worktrees. If the dependency miss recurs, preserve it and stop; no further CI retry or gate weakening is allowed.

## Astra LOW scope review — PASS

Astra LOW passed exact clean pushed brief
`81c010820c66ecc38cf503151e60692c6e8bc903` against main
`d47b62ba0dfcaf5c1525510aaa7789fa8e0acb94`. The failed-job rerun reproduced the
same uncached `wasi` dependency under the checker's locked, offline, all-target
Cargo graph; the artifact download recovered. Host Clippy does not establish that
all locked cross-target dependency sources are locally available.

One Luna workflow-only tranche may add unconditional `cargo fetch --locked` in
the existing lint/policy job immediately after pinned toolchain setup and before
offline inspections. Do not add a target restriction. A fetch failure must fail
the job. Keep the offline checker, cache keys/settings, router/verdict, manifests,
lock and product bytes unchanged. Run only YAML parsing, diff hygiene, the existing
qualification router/static-expectation policy, and directly applicable mutation
tests locally. Exact-head Astra review, required PR CI, guarded merge and the first
new post-main qualification all remain mandatory; do not rerun failed run
`34250520726` again.

## Implementation checkpoint and Astra LOW source review — PASS

Luna HIGH added one unconditional `cargo fetch --locked` step immediately after
the pinned toolchain setup in the existing lint/policy job. Exact clean pushed
checkpoint `a4128e2e5dce13c25d0dee1c6293045a187b7f99` changes only
`.github/workflows/qualification.yml`; the offline checker, cache configuration,
router/verdict, manifests, lockfile and product bytes are unchanged.

`git diff --check`, the CI path-routing checker and its mutation suite passed.
No product build/test, browser, artifact builder, benchmark or compiler capture
ran locally. Astra LOW independently reviewed the exact checkpoint against main
`d47b62ba0dfcaf5c1525510aaa7789fa8e0acb94` and returned **PASS**: unrestricted
locked fetch prepares dependencies across resolved targets, its failures remain
fatal, and the later audit remains offline and fail-closed. Required PR
qualification, fresh guarded merge review and the first new post-main
qualification remain mandatory.

## Attempt 1 source review — PASS

Astra LOW passed exact clean pushed implementation
`a4128e2e5dce13c25d0dee1c6293045a187b7f99` against unchanged live main
`d47b62ba0dfcaf5c1525510aaa7789fa8e0acb94`. The implementation is exactly two
workflow lines: unconditional `cargo fetch --locked` after the lint/policy job's
pinned toolchain and apt setup. Ordinary step failure stops the job. The later
offline gates, route/static-result/verdict logic, cache configuration, manifests,
lockfile and product remain unchanged.

PyYAML parsing, the CI path-routing checker and mutation suite, and diff hygiene
passed. A supplemental ad hoc placement parser raised `ValueError` because its own
job-block parsing was incorrect; it receives no evidence credit and was not
retried. Direct review establishes the required placement, so that auxiliary
failure does not invalidate the accepted change. No product build, artifact,
benchmark or compiler capture ran. Final clean documentation-head/current-main
review, required PR qualification, guarded merge and first new post-main
qualification remain pending.
