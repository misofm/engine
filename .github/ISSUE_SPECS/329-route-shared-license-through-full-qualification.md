# Route shared `LICENSE` changes through full qualification and complete path-aware CI rollout

## Objective

Correct issue #328's terminal HOLD and complete its path-aware CI rollout without weakening legal,
package, engine, browser, or release gates.

The root `LICENSE` is a shared legal and SDK-package input. Any diff touching it must select full
qualification so the existing canonical Apache-2.0 workspace-policy gate and the publishable SDK
package gate both run.

## Current evidence

Issue #328 stopped after its third adversarial HOLD. Its routing, aggregate, YAML-seal, concurrency,
and rollout-order gates otherwise passed. The remaining unsafe classification is exact:

- `scripts/ci-path-router.py` includes `LICENSE` in the SDK-only set;
- the canonical checker consequently places it in engine, browser, and release push-ignore lists;
- classifier tests assert `LICENSE` routes SDK-only;
- package staging copies root `LICENSE`, while tarball smoke checks only its presence; and
- the canonical Apache-2.0 SHA-256 invariant is owned by `scripts/check-workspace-policy.sh` in the
  full-route host job.

A one-byte legal-text mutation can therefore select SDK-only without running the canonical check.
No workflow rollout or branch-protection mutation occurred under #328; the old eight required
Actions contexts remain in place.

## Decision

Remove `LICENSE` from the SDK-only taxonomy. It falls through to the existing fail-safe full route.

For a `LICENSE` pull request, engine qualification runs the canonical workspace policy, SDK
qualification runs because it is selected for both full and SDK routes, browser qualification runs
under the unchanged full contract, and all three aggregate contexts report. For a `LICENSE` main
push, it is absent from engine, browser, and release ignore lists, so all four public workflows run.

Keep the canonical SHA invariant solely in `scripts/check-workspace-policy.sh`. Do not duplicate it,
move broad workspace policy into SDK ownership, or add a special legal route. This rare shared input
is full; SDK source remains the fast path.

## Scope

- Remove `LICENSE` from SDK classification in `scripts/ci-path-router.py` and
  `scripts/check-ci-path-routing.py`.
- Remove it from main `paths-ignore` in `ci.yml`, `browser-qualification.yml`, and
  `release-build.yml`.
- Update classifier and red-mutation coverage for direct, mixed, rename, and copy cases.
- Pin that the full-route host job invokes `scripts/check-workspace-policy.sh`, while the SDK full
  route retains package qualification.
- Preserve the rest of #328 as this successor's base and own its rollout.

No behavior change is required in `sdk.yml`, workspace-policy implementation, package staging, or
tarball behavior. Unrelated local SDK dependency edits remain excluded.

## Objective gates

1. `LICENSE`, `LICENSE` plus evidence/SDK paths, all ordinary status records for it, and rename/copy
   records with either side at `LICENSE` classify full.
2. SDK-only and evidence-only cases remain narrow; malformed, missing-base, workflow, engine, Wasm,
   Cargo, toolchain, and unknown inputs remain full.
3. The checker proves `LICENSE` absent from all three full-workflow ignore lists, workspace-policy
   ownership in the host job, SDK package ownership and full selection, and every inherited #328
   trigger/router/aggregate/concurrency invariant.
4. Red mutations reject re-adding `LICENSE` to the router or any ignore list using YAML-equivalent
   spellings, removing/suppressing workspace policy, removing SDK package qualification, or dropping
   a `LICENSE` rename/copy side.
5. A temporary one-byte `LICENSE` mutation fails workspace policy; the canonical file passes.
6. Routing checker/mutations, workspace policy and its mutations, workflow YAML parsing, SDK
   generated/deletion checks, and exact-scope diff checks pass locally.
7. The inherited one-artifact SDK qualification passes generated/deletion/types/headless/package/
   tarball/`enginectl` gates at rollout.
8. Fresh Sol/high adversarial verification returns PASS before rollout.
9. The full rollout passes all lanes and reports all three new aggregates before the old eight
   branch-protection contexts are atomically replaced and re-read.
10. Post-rollout evidence proves SDK-only PRs run only SDK heavy work, evidence PRs run no heavy
    work, evidence main pushes start no public workflow, and `LICENSE` selects full plus SDK.

## Non-goals

- A legal-only route or fourth required context;
- moving or duplicating the license digest;
- reducing qualification for shared, engine, browser, workflow, unknown, or mixed inputs;
- changing SDK contents/APIs, DSP, realtime, ABI, session, or control behavior;
- publishing the npm package, changing fuzz/nightly depth, or optimizing caches/runners; and
- including unrelated SDK dependency edits.

## Relationship to #328

Issue #328 remains terminal HOLD and is not declared complete. This successor adopts its otherwise
qualified implementation, corrects the bounded shared-license ownership defect, and owns the
unperformed rollout and branch-protection migration.

## Evidence

### Attempt 1 — Sol medium

The root `LICENSE` was removed from the router's exact SDK-file set and from the canonical SDK
push-ignore set, which removes it from `ci.yml`, `browser-qualification.yml`, and
`release-build.yml` without changing `sdk.yml`. It now reaches the existing unknown/shared-input
fallback and selects `full`; SDK and evidence-only inputs retain their inherited narrow routes.

The classifier suite proves direct `LICENSE`, `LICENSE` mixed with evidence and/or SDK paths, all
ordinary `A`/`D`/`M`/`T` records, and `R`/`C` records with either source or destination equal to
`LICENSE` all route full. Mutants that re-add it to the SDK set or retain only one rename/copy side
are killed by the same suite. The static checker independently AST-pins the exact SDK file set and
byte-pins the inherited canonical trigger blocks, so re-adding `LICENSE` to any full-workflow
ignore list is rejected for unquoted, single-quoted, double-quoted, and explicitly tagged YAML
spellings.

Ownership remains deliberately split without duplicating the digest: the checker pins the full
host job's exact full-route header and unsuppressed `Workspace policy` step, and pins the unchanged
SDK job's SDK-or-full selection plus its complete one-artifact package qualification step. Red
mutations reject workspace-policy removal, step/job `continue-on-error`, `if: false`, shell
suppression, SDK full-route removal, and SDK package-command removal.

Focused local evidence on 2026-09-03:

- `python3 -B scripts/check-ci-path-routing.py`;
- `python3 -B scripts/test-ci-path-routing.py`;
- `bash scripts/check-workspace-policy.sh`;
- `bash scripts/test-workspace-policy.sh` under temporary GNU-compatible `find`/`sed` wrappers,
  because the unchanged harness uses GNU-only `find -printf` and `sed -i` syntax on this macOS
  host; all mutations passed under that compatibility environment;
- a clean tracked-file temporary copy passed workspace policy, while changing one byte only in its
  copied `LICENSE` failed with `LICENSE is not the canonical Apache License 2.0 text`;
- `yq eval '.'` over all four affected/inherited workflows;
- `bash scripts/check-sdk-generated.sh`;
- `python3 -B scripts/check-sdk-deletions.py`; and
- exact-scope `git diff --check`.

No SDK source/package change, workspace-policy implementation change, package-staging change,
commit, push, GitHub mutation, branch-protection mutation, benchmark, timing, playback, or rollout
is part of this attempt. Fresh Sol/high adversarial verification remains required before rollout.
