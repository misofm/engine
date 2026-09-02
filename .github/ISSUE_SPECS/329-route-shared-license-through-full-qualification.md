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

### Attempt 1 adversarial verdict — Sol high: HOLD

The direct and synthetic name-status coverage passed, but production used `git diff
--find-copies`. Git does not search unchanged files as copy sources under that option. A branch can
therefore leave root `LICENSE` unchanged, add byte-identical content at an SDK-only path, and be
reported as a single SDK `A` record rather than `C100 LICENSE sdk/...`; both PR and push routing then
narrow incorrectly. The synthetic `C` records could not expose this discovery failure. The
workspace-policy harness's pre-existing GNU-userland portability issue is separate from this
bounded routing HOLD.

### Attempt 2 — Sol medium HOLD correction

Production now uses the exact literal Git option tuple `--name-status -z --find-renames
--find-copies-harder`, which includes unchanged files in copy-source discovery. The static checker
AST-pins both that tuple and the production `subprocess.run` command's exact consumption of it.
Both the checker mutation suite and the executable router suite reject a downgrade back to
`--find-copies`; a second checker mutant proves that leaving an unused correct tuple beside a
downgraded inline command is also rejected.

A real temporary Git repository commits root `LICENSE`, branches, and adds an identical
`sdk/LICENSE-copy` while leaving the root source unchanged. The fixture proves ordinary copy
discovery emits only `A sdk/LICENSE-copy`, harder discovery emits the exact
`C100 LICENSE sdk/LICENSE-copy` record, the linear two-dot push routes full, and a three-dot pull
request against a subsequently diverged `main` also routes full. This runs the production Git diff
path rather than injecting a preconstructed status record. All inherited direct, mixed, ordinary
status, rename/copy-side, malformed-input, trigger, aggregate, ownership, and YAML-seal mutations
remain active.

Focused local evidence on 2026-09-03:

- `python3 -B scripts/check-ci-path-routing.py`;
- `python3 -B scripts/test-ci-path-routing.py`, including both real-Git LICENSE-copy histories and
  the production-option downgrade mutant;
- `yq eval '.'` over `ci.yml`, `browser-qualification.yml`, `release-build.yml`, and `sdk.yml`;
- `bash scripts/check-sdk-generated.sh`;
- `python3 -B scripts/check-sdk-deletions.py`; and
- exact-scope `git diff --check`.

No workflow, SDK package, workspace-policy, package-staging, user dependency, GitHub, protection,
or rollout state changes in this correction. A fresh Sol/high verdict remains required; this
evidence does not claim PASS.

### Attempt 2 adversarial verdict — Sol high: PASS

Fresh Sol/high review approved exact commit `d85b5870d53979db4787307ea3bf7546388e02d6`
for rollout. In an isolated export of the tracked commit, a real repository with 2,143 tracked
files and approximately 308 MiB of content reported an unchanged root-license copy as only
`A sdk/LICENSE-copy` under ordinary copy discovery and as exact
`C100 LICENSE sdk/LICENSE-copy` under the production harder discovery. Production classified both
the linear two-dot push and divergent-main three-dot pull-request histories as full; ordinary SDK
changes remained SDK-only. A restrictive `diff.renameLimit=1` did not hide the exact copy.

The reviewer independently confirmed that production consumes the statically pinned option tuple,
the unused-correct-tuple bypass is rejected, all inherited workflow trigger/aggregate/failure
contracts remain sealed, and the routing, mutation, YAML, generated/deletion, canonical-license,
one-byte-license-red, and exact-diff gates pass. Measured production routing was approximately
0.04 seconds for an SDK change and 0.04–0.07 seconds for the unchanged-license-copy cases on the
exported repository. Conservative additional copy matches only broaden qualification.

This is the required pre-rollout PASS. Networked qualification, observation of the three new
aggregate contexts, branch-protection replacement and re-read, and post-rollout route evidence are
still pending and remain mandatory before issue closure.
