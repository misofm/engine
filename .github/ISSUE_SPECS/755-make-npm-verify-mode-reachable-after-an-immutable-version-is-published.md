# Make npm verify mode reachable after an immutable version is published

# Issue #755 approved stateless scope (independent implementation; integration revalidation required)

Brief author: Astra medium. Implementation: Luna max, exactly one coherent attempt. Adversarial verdict: Astra medium, exactly one verdict. This current user-selected routing supersedes the issue's historical Sol wording without increasing its attempt limit. Root owns issue synchronization, commits, CI, merge and delivery. Approval is scope approval, not an implementation PASS.

## Baseline and trigger

Read-only brief against origin/main c2c1368dcf4b8984fa7bdf8add5807e4336492ae and its issue spec `.github/ISSUE_SPECS/755-make-npm-verify-mode-reachable-after-an-immutable-version-is-published.md`. Throughput amendment: independent implementation is approved now in its own worktree from current main c2c1368dcf4b8984fa7bdf8add5807e4336492ae while #307 CI runs. Read-only revalidation confirmed origin/main remains that commit, the primary tree is clean, and the #307 final evidence commit b1f13ae1 has no overlap with any allowed #755 path. Root reports implementation 43473f97 received Astra PASS and evidence b1f13ae1 is pushed on PR #757; these delivery facts are supplied by root, not independently CI-verified in this brief. There is no uncommitted #307 implementation tranche. Before starting, root confirms matching GitHub #755 title/body/state and incorporates this brief in that existing issue spec. Before final #755 verification/adversarial verdict and delivery, integrate onto synchronized main after #307 merges and revalidate the allowed-path diff, current AGENTS instructions, shared archive step and all trust gates. Re-run affected focused validation if integration changes any tested content; do not obtain a verdict against a stale pre-integration tree. An integration conflict or material scope change requires rebriefing, not a disguised second implementation attempt. Preserve exactly one Luna max attempt and one Astra medium verdict. This explicitly supersedes the former wait-until-#307-delivery condition while maintaining one active implementation tranche.

The shared `Re-smoke and checksum the exact prior-qualified archive` step currently runs `npm publish --dry-run --ignore-scripts "$archive"` in both publish and verify. For existing immutable versions npm 11.19.0 refuses the dry run, so verify never reaches its existing registry, consumer and provenance checks. The issue records 0.2.3 run evidence; this brief has not independently replayed those runs.

Smallest closable capability: verify inspects the exact qualified archive and proceeds to existing read-only verification without any reachable npm publish invocation, while publish retains its pre-publication dry run, immutable-version guard and one real publish maximum.

## Allowed edits

1. `.github/workflows/npm-publish.yml`: add `MODE: ${{ inputs.mode }}` to the shared re-smoke step's env and wrap only its existing dry-run command in `if [[ "$MODE" == publish ]]; then ...; fi`. Equivalent minimal step-local guard is acceptable. Keep the shared step selected for publish and verify. Leave its unpack/smoke and Node checksum statements intact, in their present order. No new release step abstraction is needed.
2. `scripts/test-npm-publish-modes.py` (new focused hermetic test): extract the repository's fixed-format release steps and run their real shell/Node validation logic against temporary fixtures and controlled command stubs. Reuse the fixed-contract extraction style in `scripts/check-ci-path-routing.py`; do not implement a general YAML or GitHub Actions expression evaluator. Support only conditions occurring in this workflow and reject unknown conditions/shapes rather than treating them as selected or silently skipping them.
3. `.github/workflows/qualification.yml`: only add one lint step invoking `python3 -B scripts/test-npm-publish-modes.py` alongside existing release policy tests, if required for persistent CI coverage. No router, job, status, permission, trigger or dependency changes.
4. The existing #755 issue spec: update the decision record, objective evidence and bounded routing. No new numbered issue or unrelated evidence corpus.

No other files are authorized by this brief. No SDK version change, artifact rebuild, tag movement, npm publication, real workflow dispatch, release framework, token fallback, retry expansion or DSP/adapter change. No schema weakening. Test fixtures remain temporary; no generated package/release outputs are committed.

## Mode semantics and a resolved wording ambiguity

The existing qualify-only `Pack exactly one archive, smoke it, and record immutable local evidence` step already includes its own npm publish dry run. Preserve that step byte-for-byte. Interpret issue gate 1's `qualify does not download/publish` as no prior-artifact download and no real publication: qualify's pre-existing inspection dry run remains. Literal zero publish commands in qualify would conflict with the issue's bounded shared-step correction and is not the adopted scope. Verify alone requires zero publish invocations of any kind. Assert download is absent from qualify and present for publish/verify, the shared re-smoke step is absent from qualify and selected for both other modes, and real publish is selected only for publish.

## Focused hermetic discriminator

Use a fresh temporary workspace, temporary archive and matching hash evidence. Execute extracted workflow scripts, not copied validators. Stub external npm calls with a command journal and explicit response fixtures, never forwarding to the host npm binary. Stub package smoke/import/enginectl boundaries narrowly; real Node must execute the extracted checksum, registry and provenance here-doc programs. Assert those boundaries were reached rather than claiming synthetic fixtures exercised actual SDK imports. No command must access the public registry. Pin the fake npm behavior so `publish --dry-run` for an existing version fails as the reported defect does. Stub sleep for negative registry cases; preserve the workflow's real bounded loop.

Required green and red cases:

- Qualify structural reachability as specified above; do not execute qualify's build/pack path or rebuild artifacts.
- Publish with a missing version: shared unpack/smoke and checksum execute, exactly one dry run precedes the absence query, exactly one real publish is possible, then registry/consumer/audit validation is reached.
- Verify with an existing immutable version: shared unpack/smoke/checksum execute, npm journal has zero publish entries (including dry run), registry and fresh consumer/audit boundaries are reached, and the real Node provenance validator succeeds on a valid synthetic audited SLSA fixture.
- Restoring unconditional shared dry-run must fail verify on the existing-version stub; removing its dry run must fail the publish count assertion. Mutate workflow text in memory or a temporary copy, never the working file.
- Corrupt archive/evidence must fail before registry verification. At least representative checksum-negative coverage must execute the real three-field validator; retaining every statement unchanged is separately audited.
- Existing-version publish refuses before real publication; ambiguous absence refuses before real publication. An ambiguous real publish result must never produce a second publish invocation.
- Registry mismatch must fail closed after the bounded convergence attempts and must not reach the consumer. A synthetic bad provenance identity/exact expected SHA or missing verified attestation must fail the actual Node predicate validation. Do not stub these validators to success or loosen their schema to satisfy fixtures.

Keep this one small workflow test, not a simulated release platform. Full SDK/package smoke fixtures already belong elsewhere. If narrow extraction or command-boundary fixtures cannot discriminate these claims within one implementation attempt, stop and rebrief instead of adding dependencies or a framework.

## Invariants for adversarial review

Compare normalized workflow structures and the textual diff against the revalidated starting commit. Apart from the shared-step env/guard and optional qualification test invocation, all release workflow content must remain byte-equivalent. Specifically inspect: exact main dispatch/GITHUB_SHA/expected SHA/checkout binding; accepted/rejected ancestry; qualification-run identity and named nonexpired artifact; action commit pins; npm pin; shared archive uniqueness/unpack/smoke; SHA-1, SHA-256 and SHA-512 SRI comparisons; immutable-version/ambiguous absence refusal; OIDC-only token rejection; one-shot publish; registry version/shasum/integrity/public/latest convergence and bounded retry; fresh imports/enginectl; signature report verified/rejected cardinality; SLSA bundle, DSSE payload type, statement type, package PURL and SHA-512 subject; workflow repository/path/main ref; exact expected SHA resolved-dependency cardinality; evidence upload. No npm/schema assertions may be weakened.

## Proportional validation and delivery

Implementation runs the focused Python test (including both mandatory red mutations), `node scripts/test-parse-npm-trust-list.mjs`, `python3 -B scripts/check-ci-path-routing.py`, and `python3 -B scripts/test-ci-path-routing.py` when qualification wiring changes. Validate every extracted npm workflow shell block using `bash -n` without execution. Perform YAML syntax validation using an existing available parser or the remote GitHub parser; yq/actionlint were not on PATH during briefing and repository policy explicitly does not install PyYAML. Do not add a YAML dependency just for this issue. `git diff --check` and the strict allowed-path/invariant diff audit complete local validation. No Rust build, benchmark, Wasm build, SDK packaging or public release operation is needed for the local proof.

Astra medium performs one adversarial verdict against the issue, actual extracted-script evidence, publication reachability and unchanged trust gates. If it fails, preserve evidence and stop; no second attempt is authorized under #755. Root checkpoints the coherent tranche and records evidence accurately; required CI must pass before merge. After upstream evidence and PASS, synchronize #755 body/state and close/verify remotely in the same delivery workflow. Report no delivery completion before GitHub synchronization.

Brief actions: read-only repository inspection and this /tmp artifact only; no tests launched, repository edits, publication or workflow dispatch.

## Original issue report and decision history

# Make npm verify mode reachable after an immutable version is published

## Problem and smallest closable slice

The existing `.github/workflows/npm-publish.yml` documents `verify` as recovery after registry propagation, but its shared `Re-smoke and checksum the exact prior-qualified archive` step runs `npm publish --dry-run --ignore-scripts "$archive"` for both `publish` and `verify`. With pinned npm 11.19.0, verify of an already-published immutable version fails there with `You cannot publish over the previously published versions` and never reaches registry convergence, fresh consumer imports, signature audit, or SLSA predicate checks. Engine 0.2.3 verify run `34640332474` demonstrates the defect; publish run `34640083474` first exposed the need for recovery after a successful publication outlived the workflow's 60-second registry window.

Deliver the smallest workflow correction that makes future verify-only recovery reachable. This is release tooling only. It does not alter or republish 0.2.3 and cannot retroactively change its provenance or failed workflow conclusions.

## Bounded change

- Keep unpacking, `sdk/test/package-tarball-smoke.mjs`, and SHA-1/SHA-256/SRI comparison against the downloaded qualification artifact in both `publish` and `verify` modes.
- Run `npm publish --dry-run --ignore-scripts "$archive"` only in `publish` mode, before the registry absence guard and the single real publish. Verify mode must never invoke any publish command, including dry run.
- Preserve exact-main/expected-SHA binding, qualification-run and named-artifact identity, immutable-version refusal before publish, OIDC-only publication, no retry after ambiguous publish, registry public/latest convergence, fresh registry consumer imports/enginectl, signature audit, exact SLSA subject/workflow/ref/resolved-dependency predicates, action pins, and evidence upload.
- Do not add a second release workflow, a token fallback, a republish path, a broader retry policy, or a new release framework.

## Objective gates

1. A hermetic workflow-level test or existing workflow parser exercise proves the mode matrix: qualify does not download/publish; publish re-smokes, checksums, dry-runs exactly once, checks absence, and may publish exactly once; verify re-smokes and checksums but contains no reachable `npm publish` command and proceeds to registry/consumer/provenance checks for an existing version. Red mutations that restore dry-run to verify or remove it from publish must fail.
2. Shell/YAML validation and existing npm trust-list/release workflow checks pass. The normalized workflow diff contains only the necessary mode condition/test change; all cryptographic and exact-SHA assertions remain byte-equivalent or demonstrably equivalent.
3. Exercise the corrected verification path without publishing: use a fixture/mock boundary or a future already-qualified existing release as permitted by the issue, prove an existing immutable version does not stop at package inspection, and prove registry/provenance failures still fail closed. Never point a test publish at the public package.
4. Sol adversarial review confirms verify is read-only, publish retains its pre-publication dry run and one-shot semantics, and no provenance gate weakened. Required CI passes before merge; synchronize and close the matching GitHub issue after upstream evidence.

## Limits

One implementation attempt and one Sol verdict. If a bounded hermetic mode discriminator cannot be added without creating a release framework, stop and rebrief rather than expanding scope. No SDK version bump, artifact rebuild, DSP/app/adapter change, npm publication, tag change, or reinterpretation of issue #753 evidence belongs here.

## Attempt 1 implementation evidence

Luna max completed one coherent attempt and paused. The release workflow changes only the shared step MODE environment and publish-only dry-run guard; qualification adds one invocation of the focused hermetic test. The test executes extracted shell/Node validators with controlled npm/package boundaries and covers all three mode selections, publish counts/order, both required red mutations, malformed workflow shapes, checksum failures, existing/ambiguous publication refusal, no retry after ambiguous publication, bounded registry mismatch, and provenance negatives. No public registry operation, SDK build or release dispatch was performed.

Focused test, trust-list test, routing checker/mutations, shell syntax, YAML syntax and strict workflow invariant/path checks passed. Logs are `/tmp/issue755-attempt1-*-final.log`; the focused log retains an earlier unsupported single-line run-block fixture failure before its corrected successful runs. The pinned release workflow and qualify-only pack block were checked unchanged apart from the authorized guard. Astra medium's sole verdict remains pending.
