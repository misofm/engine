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
