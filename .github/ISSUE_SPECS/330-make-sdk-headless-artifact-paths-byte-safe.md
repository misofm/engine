# Make SDK headless artifact paths byte-safe and complete path-aware CI rollout

## Objective

Correct issue #329's terminal HOLD with the smallest independently closable slice. Make
`scripts/check-sdk-headless.sh` preserve an accepted caller-relative artifact directory across its
internal transition to `sdk/`, including valid POSIX pathnames ending in one or more newline bytes,
while retaining validation exit status 2 for invalid, unsearchable, missing-module, and
caller-supplied symlink directories.

Deliver the correction as an SDK-only change so it neither starts nor cancels unrelated engine,
browser, or release workflows. After fresh Sol/high PASS and successful SDK qualification, finish
the aggregate-context, branch-protection, post-routing, and remote-evidence rollout inherited from
issues #328 and #329.

## Current evidence

At briefing time:

- remote `main` is exact commit `951a5a3c5728b66fe2c51f4f7842c91b61be1a9d`;
- the preserved failed branch contains attempt-3 implementation `68eef8d6` and terminal evidence
  `934540d3`; those commits are evidence and must not be merged or cherry-picked wholesale;
- unrelated local `sdk/package.json` and `sdk/package-lock.json` edits are user-owned and excluded;
- the old eight branch-protection contexts remain required and no ruleset adds checks;
- browser qualification passed at <https://github.com/misofm/engine/actions/runs/33651977929>;
- release build passed at <https://github.com/misofm/engine/actions/runs/33651978151>;
- SDK rollout <https://github.com/misofm/engine/actions/runs/33651978105> failed after the headless
  script entered `sdk/` while retaining repo-relative `target/ci/sdk-artifacts`; Node tried
  `sdk/target/ci/sdk-artifacts/miso-engine-v1-audio-worklet.simd128.wasm`, producing 24 passes, one
  failure, and 87 cancellations; the SDK aggregate correctly propagated the failure; and
- engine qualification from the same rollout remains in progress and must not be stopped.

Issue #329 attempt 3 fixed ordinary relative paths with:

```bash
artifact_dir=$(cd -- "$artifact_dir" && pwd -P)
```

Bash command substitution removes trailing newline bytes. A directory ending in a newline passes
the existing `-d` and `! -L` checks, is changed into a different pathname, and fails the module
check. Attempt 3's oracle used the same faulty mechanism. Its bare `cd` also returned status 1 for
an accepted but unsearchable directory instead of validation status 2. Independent review proved
the parent red for ordinary relative paths and attempt 3 red for terminal-newline paths and
unsearchable status; spaces and direct-symlink status passed. `scripts/sdk-package.sh` does not
share the defect because staging consumes its argument before its later npm-pack subshell changes
directory.

## Decision

Implement from clean synchronized main and preserve the failed #329 branch. Retain the existing
argument-count, directory, and final-component non-symlink checks. Resolve the artifact directory
with a physical `cd -P` while appending a known non-newline sentinel inside command substitution,
then remove exactly that sentinel. Neither production nor the test oracle may capture bare `pwd`
output with command substitution. A failed physical-directory transition is validation failure and
returns 2. POSIX paths cannot contain NUL; all other pathname bytes carried by Bash remain data.

Preserve these outcomes:

- usage, nonexistent/non-directory input, direct caller-supplied symlink, unsearchable directory,
  and missing Wasm module return 2;
- Node's nonzero status after successful validation remains unsuppressed; and
- this issue does not claim race-free descriptor traversal against a hostile concurrent replacer.

Put the executable regression under `sdk/test/` so it is already SDK-owned. It must invoke the
production shell script, accept an alternate exact script path for parent/attempt-3 probes, derive
expected paths independently, and use a valid minimal Wasm fixture plus a fake Node only to observe
cwd, environment bytes, file visibility, and exact arguments. Cover ordinary relative and absolute
paths, spaces, tabs/metacharacters, embedded and terminal newlines, repeated terminal newlines, a
sentinel-like final byte, missing directory/module, direct symlink, and unsearchable status.

The fixed regression must be red against exported `951a5a3c`, red against exported `68eef8d6`, and
green at the successor checkpoint. Keep `scripts/sdk-package.sh` unchanged unless new executable
evidence contradicts the established analysis.

## Scope

- `scripts/check-sdk-headless.sh`;
- one `*-evals.mjs` regression under `sdk/test/` and at most one narrow helper there;
- this issue specification; and
- evidence-only synchronization for issues #327, #328, and #329.

The new SDK test is discovered by the existing headless invocation. The production script is
already in the exact SDK taxonomy and all three full-workflow main-push ignore sets. No workflow,
router, checker, ignore-list, or standalone `scripts/test-sdk-headless-path.sh` change is required.

## Objective gates

1. Accepted relative input becomes an absolute physical path before entering `sdk/`, without losing
   or adding bytes, including one or more terminal newlines.
2. The child runs from exact SDK root, receives the invariant artifact path, sees valid Wasm, and
   receives exactly `--test 'test/*-evals.mjs'`.
3. Usage, nonexistent/non-directory, direct-symlink, unsearchable, and missing-module cases return
   exactly 2; Node failures after validation remain unsuppressed.
4. The same executable regression is parent-red, attempt-3-red, and successor-green, with an oracle
   independent of production canonicalization.
5. Ordinary, absolute, spaces, tabs/metacharacters, embedded newline, one and repeated terminal
   newlines, and a sentinel-like final byte pass using valid minimal Wasm.
6. The existing real headless invocation discovers the regression automatically; no workflow step
   or test-only bypass is added.
7. Routing checker/mutations pass unchanged. The proposed pushed range classifies SDK-only while
   shared, workflow, engine, Wasm, Cargo, unknown, malformed, rename, and copy behavior stays full.
8. Workflow YAML, SDK generated/deletion/type, Bash syntax, canonical artifact digest, and exact
   diff gates pass without changing or regenerating a digest.
9. Static and executable evidence confirms `scripts/sdk-package.sh` remains unaffected and unchanged.
10. Fresh Sol/high review of the exact successor checkpoint returns PASS before push.
11. The corrective main push contains only SDK/evidence-owned paths; only SDK qualification starts
    and it neither starts nor cancels engine, browser, or release workflows.
12. The SDK run passes its one-artifact generated/deletion/types/headless/package/tarball/enginectl
    closure and reports a passing `SDK qualification` aggregate.
13. Passing engine, browser, and SDK aggregate contexts and selected release work are observed before
    any old required context is removed.
14. Protection is atomically changed from the exact old eight contexts to `engine qualification`,
    `SDK qualification`, and `browser qualification`, then re-read with Actions app identities.
15. Post-rollout evidence proves SDK-only PR, evidence-only PR, evidence-only main push, `LICENSE`
    full routing, and unknown/malformed fail-safe behavior.
16. Local specs and GitHub issues #327, #328, #329, and #330 are synchronized upstream. Terminal
    HOLDs remain recorded as HOLDs rather than rewritten as PASS.

## Non-goals

- Changing workflows, router, routing checker, ignore taxonomy, or `scripts/sdk-package.sh` without
  contradictory evidence;
- carrying forward failed `scripts/test-sdk-headless-path.sh`;
- changing package contents, public SDK APIs, DSP, realtime, ABI, session, or control behavior;
- changing pinned digests, committing built Wasm, or publishing the npm package;
- stopping, cancelling, or rerunning the existing rollout workflows;
- weakening aggregate, workspace, package, browser, release, or legal gates; or
- including unrelated SDK dependency edits.

## Relationship to prior issues

Issue #329 remains terminal HOLD after three attempts. This successor starts a fresh attempt budget,
adopts only the bounded path defect and unfinished rollout, and preserves #329's failed history.
Issue #328 likewise remains terminal HOLD; its qualified router, aggregate, concurrency, and
rollout contracts remain inherited. Issue #327 closes complete only after successful remote SDK
package qualification supplies its missing evidence.

## Rollout order

1. Create matching local #330 spec and GitHub issue before implementation.
2. Work from clean `951a5a3c`; preserve the failed #329 branch and exclude user dependency edits.
3. Implement the bounded production correction and SDK-owned regression.
4. Run proportional gates and parent-red/attempt-3-red/successor-green isolated exports.
5. Obtain fresh Sol/high PASS on the exact checkpoint.
6. Re-read remote main, rollout runs, protection, and rulesets; amend if external state drifted.
7. Prove the proposed push range is SDK-only and production routing returns `sdk`.
8. Push once; do not rerun failed run 33651978105 because it targets the old SHA.
9. Verify only SDK qualification starts and no engine/browser/release run starts or is cancelled.
10. Observe the complete SDK closure and aggregate and record the run URL.
11. Record completion of the pre-existing engine/release/browser rollout runs honestly.
12. Re-query protection/rulesets and verify all three aggregates have reported successfully.
13. Atomically replace the old eight required contexts with the exact three aggregates and re-read.
14. Run inherited SDK-only, evidence-only, and LICENSE routing observations without bypasses.
15. Push final evidence, synchronize/close issues according to their recorded disposition, and
    re-read every remote state.

## Evidence

Sol/high approved this bounded successor brief on 2026-09-03. Implementation and adversarial
evidence will be appended without weakening the gates above.
