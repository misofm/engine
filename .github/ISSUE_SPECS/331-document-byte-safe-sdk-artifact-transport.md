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

### Sol/high adversarial PASS

Fresh Sol/high review returned **PASS** on exact checkpoint `d50bb939`. The reviewer independently
proved the worktree clean; the implementation commit limited to this evidence and one vocabulary
row; the new row's lowercase/even-length/absolute-physical-byte/Buffer semantics exact; the
bidirectional checker green with 99 names; its unchanged mutation suite green on Linux; and exact
predecessor `d5a41c0e` red for the undocumented `_HEX` spelling. All issue-#330 production and
regression files are byte-identical to `d5a41c0e`.

Independent reruns produced 18 pass / 0 fail / one permitted APFS skip for the focused path suite
and 129 pass / 0 fail / one skip across 130 full headless tests. Isolated package/tarball
qualification passed with a terminal-newline artifact path and all 9 enginectl tests. SDK type,
generated, deletion, routing checker/mutations, Bash syntax, workflow YAML, and exact-diff gates
passed. The complete `origin/main..d50bb939` range is exactly the two successor/history specs, one
vocabulary row, and inherited three-file SDK correction and classifies `sdk` for both push and pull
request. Workflows, router/checkers, package scripts/manifests, Cargo files, generated authorities,
and artifact pin are unchanged. No active raw-path alias remains; the sole old spelling outside
historical specs is an intentional test-harness deletion that sanitizes ambient legacy state.

The reviewer confirmed that Linux CI executing the invalid-UTF-8 fixture without skip and the
canonical artifact digest remain valid remote acceptance gates, not pre-push blockers.

### SDK-only remote acceptance

After a final drift audit confirmed remote `main` at the reviewed base, the old exact eight
required checks, no rulesets, and an `sdk` route for both push and pull-request semantics, the
reviewed range was pushed once to `main` as
`a03824b1540bc105bd0ef515461cb02f445a7c14`. GitHub created only SDK qualification run
<https://github.com/misofm/engine/actions/runs/33662462767>; no engine, browser, or release workflow
was created for the commit, and the already completed rollout runs were not cancelled.

The Ubuntu 24.04 SDK job passed in 2m42s from one pinned AudioWorklet build. Its log proves:

- the invalid-UTF-8 filename test executed as test 15 and passed, with the full headless result
  **130 passed / 0 failed / 0 cancelled / 0 skipped** across 28 suites;
- generated assets, modules, and surface matched the engine; deletion policy passed over 44 files;
  and strict SDK typecheck including the shipped-host mirror pin passed;
- packaging staged all 6 Engine V1 artifacts, and the extracted package's enginectl suite passed
  **9/9**;
- npm packed `@misofm/engine@0.1.0` with the 2.7 MB simd128 Wasm asset among 63 files, and the
  publishable-tarball gate passed; and
- the final `SDK qualification` aggregate passed.

This completes gates 5, 6, 10, and 11. The aggregate/protection migration remains correctly held
until the separately scoped engine allocation-harness correction produces a trustworthy passing
engine aggregate. This issue does not yet claim closure or remote body synchronization.

### Aggregate qualification and protection migration

Issue #332's reviewed allocation-harness correction reached remote `main` at exact commit
`0b1b8f2db759da7653fc97427e44b5ba2949c600`. Without retry, engine run
<https://github.com/misofm/engine/actions/runs/33666706501> passed its full 35m50s host job,
including the transient-shaper allocation gate, plus x86, cross-target, and browser-Wasm support
jobs; its final `engine qualification` aggregate passed. The same commit's SDK run
<https://github.com/misofm/engine/actions/runs/33666706581>, browser run
<https://github.com/misofm/engine/actions/runs/33666706481>, and release run
<https://github.com/misofm/engine/actions/runs/33666706600> all completed successfully. The commit
API independently reported `engine qualification`, `SDK qualification`,
`browser qualification`, and `workspace release build` as completed successes.

Immediately before migration, remote `main` was still that exact commit, required-status checks
were the expected old eight contexts with `strict: false` and Actions app ID `15368`, and the
repository had no rulesets. The required-status endpoint was then updated once with the complete
replacement set. Its response and a separate post-write read both reported exactly:

- `engine qualification` with app ID `15368`;
- `SDK qualification` with app ID `15368`; and
- `browser qualification` with app ID `15368`.

The re-read preserved `strict: false`, contained no old or extra context, found no ruleset, and
confirmed remote `main` had not moved. This completes gates 12 and 13. Gate 14's actual SDK-only PR,
evidence-only PR, and remaining post-rollout routing observations still precede issue closure.

After migration, evidence-only checkpoint `cb5f2542` was pushed to `main`. An authoritative
`gh run list --commit cb5f2542` returned an empty run set: no engine, SDK, browser, release, or
other workflow was created. The checkpoint synchronized issues #331 and #332 before #332 closed as
completed. This supplies gate 14's evidence-only-main observation; the real SDK-only PR,
evidence-only PR, `LICENSE` full-route observation, and unknown/malformed fail-safe observations
remain open.

### Post-rollout SDK-only pull-request observation

Issue #333 supplied the required real SDK-only pull request. The feature branch at `b89a7fb2`
classified `sdk`, and its ordinary branch push created zero workflows. Pull request #334 produced
all three stable required contexts: `engine qualification`, `SDK qualification`, and
`browser qualification` completed successfully. Only SDK run
<https://github.com/misofm/engine/actions/runs/33675688367> executed a substantive qualification
job; its package/generated/headless closure passed in 2m53s. Engine run
<https://github.com/misofm/engine/actions/runs/33675688483> and browser run
<https://github.com/misofm/engine/actions/runs/33675688390> ran their classifiers and stable
aggregate jobs while all heavy engine/browser work skipped. The PR was mergeable under the exact
three-context protection set and merged once as `49c153f7`.

The resulting main push created only SDK qualification run
<https://github.com/misofm/engine/actions/runs/33676075580>; it passed its substantive job and
aggregate in 2m54s without retry. No engine, browser, or release run was created for that SDK-only
main commit. This completes gate 14's actual SDK-only PR observation. The evidence-only PR and
explicit `LICENSE`/unknown/malformed post-rollout observations remain before issue closure.

### Final fail-safe observations and evidence-only preflight

After PR #336 merged the same frozen router to main at `97ffd966`, five direct post-rollout probes
all exited 0 and printed `full`:

- pull-request path `LICENSE`;
- pull-request path `future/unowned.surface`;
- a pull-request event with missing base/head revisions;
- a malformed NUL-delimited `R100` rename record with only one path; and
- an unknown event with otherwise nonempty revisions.

The workflow contract checker and complete classifier/mutation suite remained green immediately
before the #336 push. These observations prove that shared license ownership and future, missing,
malformed, or unknown inputs cannot silently select SDK/evidence qualification. The final local
closeout changes only numbered issue specifications and classifies `evidence`; its pull request is
the remaining live evidence-only PR observation required by gate 14.

### Evidence-only pull request and final rollout verdict

Exact evidence checkpoint `a420abe9` changed only the #331 and #335 numbered issue specifications
and classified `evidence`. Its ordinary branch push created zero workflows. Pull request #337,
based on exact main `97ffd966`, then produced the three protected aggregate contexts:

- SDK run <https://github.com/misofm/engine/actions/runs/33681478115> classified the range, skipped
  the complete package/generated/headless job, and passed `SDK qualification` in 3s;
- engine run <https://github.com/misofm/engine/actions/runs/33681478110> classified the range,
  skipped host, x86, browser-Wasm, and cross-target jobs, and passed `engine qualification` in 2s;
  and
- browser run <https://github.com/misofm/engine/actions/runs/33681478156> classified the range,
  skipped the artifact and browser matrix jobs, and passed `browser qualification` in 3s.

PR #337 was mergeable under the exact three-context protection set and merged once as
`d4a07cf6`. An authoritative run query for that evidence-only main commit returned an empty set:
the merge created no engine, SDK, browser, release, or other workflow. This completes gate 14's
evidence-only PR observation alongside the earlier evidence-only main, SDK-only PR,
`LICENSE`/unknown/malformed fail-safe, and protection observations. Local specs and GitHub issues
#327–#335 now have the intended terminal/open dispositions; #331 is complete once this final
evidence commit is upstream and its remote body/state are synchronized.
