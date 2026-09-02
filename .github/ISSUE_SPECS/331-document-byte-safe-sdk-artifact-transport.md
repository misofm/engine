# Document byte-safe SDK artifact transport and complete path-aware CI rollout

## Objective

Close issue #330's terminal environment-contract gap without changing its byte-safe implementation.
Replace the retired raw SDK artifact-directory environment row with the canonical lowercase-hex
transport in `docs/ENGINE_ENV_VOCABULARY.md`, prove the bidirectional vocabulary gate and its red
mutations, then qualify and ship the inherited SDK-only correction. Complete the aggregate-context,
branch-protection, post-routing, and remote-evidence rollout inherited from issues #328–#330.

## Current evidence

At briefing time:

- remote `main` is exact commit `951a5a3c5728b66fe2c51f4f7842c91b61be1a9d`;
- issue #330's preserved attempt-3 checkpoint is `d5a41c0e` on `codex/sdk-path-rollout`;
- `d5a41c0e` transports the canonical physical artifact-directory bytes as lowercase ASCII hex,
  decodes them to a Node `Buffer` path, and has focused macOS evidence of 18 pass / 0 fail / one
  unsupported-filesystem skip plus 129 pass / 0 fail / one skip in the full headless suite;
- fresh Sol/high review found `bash scripts/check-env-vocabulary.sh` red because production uses
  `MISO_ENGINE_SDK_ARTIFACTS_HEX` while the vocabulary still lists
  `MISO_ENGINE_SDK_ARTIFACTS`;
- issues #328, #329, and #330 are terminal HOLDs whose implementation history remains evidence;
- browser qualification passed at <https://github.com/misofm/engine/actions/runs/33651977929> and
  release build passed at <https://github.com/misofm/engine/actions/runs/33651978151>;
- SDK qualification at <https://github.com/misofm/engine/actions/runs/33651978105> failed on the
  pre-correction relative-path defect and must not be rerun; and
- engine qualification at <https://github.com/misofm/engine/actions/runs/33651977993> failed only
  the transient-shaper process-global allocation harness, which has independently passed with the
  same release binary and is assigned to a separate bounded successor after this SDK-only push.

## Decision

Carry the byte-safe implementation from `d5a41c0e` unchanged. In the subject-switch table of
`docs/ENGINE_ENV_VOCABULARY.md`, replace the unused raw-path row with exactly one
`MISO_ENGINE_SDK_ARTIFACTS_HEX` row. Document that its value is canonical lowercase, even-length
ASCII hex encoding of the absolute physical directory pathname bytes and that the SDK helper
decodes a byte `Buffer` path. Do not document both spellings: the vocabulary's one-name-per-fact
rule makes an alias a defect, and no compatibility surface has shipped.

The smallest closable implementation slice is the one-row contract correction plus executable
vocabulary, path, headless, package, and routing evidence. No production, workflow, router, package,
DSP, ABI, or public SDK API change belongs to this successor. The inherited rollout work remains
part of closure because the byte-safe SDK correction is the pending SDK-only acceptance push.

## Scope

- `docs/ENGINE_ENV_VOCABULARY.md`;
- this issue specification and terminal evidence synchronization for issue #330; and
- inherited unpushed issue-#330 changes in `scripts/check-sdk-headless.sh`,
  `sdk/test/headless-path-evals.mjs`, and `sdk/test/support.mjs`.

The user-owned `sdk/package.json` and `sdk/package-lock.json` edits in the original worktree remain
excluded. No workflow, router, checker, package script, manifest, digest, dependency, generated
surface, engine, browser, or release file changes are authorized.

## Objective gates

1. The vocabulary contains exactly `MISO_ENGINE_SDK_ARTIFACTS_HEX` for this fact and contains no
   active row for `MISO_ENGINE_SDK_ARTIFACTS`.
2. The row accurately describes lowercase even-length ASCII hex of absolute physical pathname
   bytes and Buffer-based decoding; it does not claim a JavaScript path string.
3. `scripts/check-env-vocabulary.sh` and `scripts/test-env-vocabulary.sh` pass, with the latter
   proving both undocumented-use and unused-row failures remain discriminating.
4. Issue #330's final checkpoint stays red on the vocabulary gate and this successor is green.
5. The complete issue-#330 focused and real headless evidence remains green without changing its
   implementation; Linux CI executes, rather than skips, the invalid-UTF-8 byte fixture.
6. SDK types, generated surface, deletion policy, package/tree/tarball, and all 9 enginectl tests
   pass from one built Wasm artifact.
7. Bash syntax, ShellCheck, workflow YAML, canonical artifact digest, exact diff, routing checker,
   routing mutations, and environment vocabulary gates pass without changing pins or generators.
8. Fresh Sol/high adversarial review returns PASS on the exact successor checkpoint before push.
9. The proposed main range is classified SDK-only and contains only the inherited SDK correction,
   the one vocabulary row, and issue evidence.
10. The corrective main push starts only SDK qualification and neither starts nor cancels engine,
    browser, or release workflows.
11. The SDK job and `SDK qualification` aggregate pass; its log proves the Linux raw-byte case ran
    without a skip and the one-artifact closure completed.
12. A separate issue corrects and qualifies the transient-shaper allocation harness so passing
    engine, browser, and SDK aggregates plus selected release work are observed before protection
    changes.
13. Protection is atomically changed from the exact old eight contexts to
    `engine qualification`, `SDK qualification`, and `browser qualification`, preserving Actions
    app identities, and the result is re-read.
14. Post-rollout observations prove SDK-only PR, evidence-only PR, evidence-only main push,
    `LICENSE` full routing, and unknown/malformed fail-safe behavior.
15. Local specs and GitHub issues #327–#331 are synchronized upstream; terminal HOLDs remain HOLDs
    and passing issues close only after their evidence is upstream.

## Non-goals

- Revising the byte-safe path algorithm or its tests without new contradictory evidence;
- retaining the retired raw-path variable as an alias;
- changing workflows, path taxonomy, router/checker logic, package scripts, package contents,
  public SDK APIs, DSP, realtime behavior, ABI, session, or control behavior;
- changing pinned digests, committing Wasm, publishing npm, or rerunning old failed workflows; or
- repairing the independent transient-shaper allocation harness in this SDK-only push.

## Rollout order

1. Create this matching local spec and GitHub issue before implementation.
2. Obtain Sol/high approval of this bounded brief.
3. Make the one-row documentation correction with Sol medium.
4. Run the complete proportional gates, including the previously omitted vocabulary checker and
   its mutation suite; prove `d5a41c0e` red and the successor green.
5. Obtain fresh Sol/high PASS on the exact checkpoint.
6. Re-read remote main, runs, protection, and rulesets and verify no drift.
7. Prove the proposed push range is SDK-only, then push once to `main`.
8. Verify only SDK qualification starts, observe the full Linux closure, and record the run URL.
9. Resolve the independent engine allocation-harness issue and obtain a passing engine aggregate.
10. Observe all three aggregate contexts and selected release work before atomically migrating and
    re-reading branch protection.
11. Run the inherited post-routing observations, push final evidence in the declared batch shape,
    and synchronize every local and remote issue disposition.

## Evidence

Drafted from issue #330's terminal Sol/high finding on 2026-09-03. Sol/high approved the brief at
commit `1e477e12`: the one-row replacement is the smallest closable successor, both directions of
the vocabulary contract are explicit, the production implementation is frozen, and the inherited
Linux, package, routing, aggregate, protection, and remote-synchronization gates remain mandatory.
Implementation and adversarial evidence will be appended without weakening those gates.

### One-row implementation evidence

From clean checkpoint `fae8169d`, the subject-switch table now contains exactly one row for this
fact: `MISO_ENGINE_SDK_ARTIFACTS_HEX`. The row specifies canonical lowercase, even-length ASCII
hex of the absolute physical artifact-directory pathname bytes and states that the SDK helper
decodes a byte `Buffer` path. The retired raw-path spelling is not retained as an active row. No
production code, test, workflow, router, manifest, package file, pin, or other issue spec changed.

Proportional local evidence on macOS on 2026-09-03:

- `bash scripts/check-env-vocabulary.sh` passed with 99 documented/used names. An isolated
  `git archive` of predecessor `d5a41c0e` remained red on the same gate, specifically reporting
  undocumented `MISO_ENGINE_SDK_ARTIFACTS_HEX`; the current tree is green.
- `scripts/test-env-vocabulary.sh` reached its existing GNU-only `sed -i` call and aborted under
  native BSD `sed` (`invalid command code f`). Without changing the repository, the exact suite
  passed through a disposable shim that maps only GNU `sed -i` syntax to BSD `sed -i ''`, proving
  its undocumented-use, unused-row, deleted-row, prefix, synonym, and missing-file mutations.
- `node --test sdk/test/headless-path-evals.mjs` passed 18 tests with zero failures and one allowed
  APFS invalid-UTF-8 skip (`EPERM`). The Linux no-skip obligation remains for remote qualification.
- SDK generated-surface, deletion-policy, and strict type/mirror gates passed. The path-routing
  workflow checker and its mutation suite passed, all workflow YAML parsed with `yq`, and the exact
  two documentation/evidence paths classify `evidence` for pull-request and push inputs.
- Bash syntax passed for the headless and vocabulary scripts, and ShellCheck passed the inherited
  headless script. Whole-set ShellCheck remains nonzero on two pre-existing SC2016 informational
  findings in the vocabulary scripts; neither diagnostic is introduced by this documentation-only
  change. `git diff --check` passed.

This evidence does not claim the required fresh Sol/high review, Linux remote qualification,
aggregate-context rollout, branch-protection mutation, post-routing observations, GitHub
synchronization, or issue closure.
