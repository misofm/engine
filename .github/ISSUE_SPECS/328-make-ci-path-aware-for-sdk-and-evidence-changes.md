# Make CI path-aware for SDK and evidence-only changes

## Objective

Stop SDK-only and evidence-only changes from running or cancelling unrelated native DSP,
realtime, release, Wasm-conformance, and full-browser matrices. Preserve full qualification for
engine, Wasm, browser-host, workflow, and unknown inputs.

SDK-only changes must still qualify the complete publishable package: the pin-attested embedded
Wasm engine, generated/deletion policy, strict types, headless behavior, tarball contents, and
`enginectl`.

## Current evidence

The final issue #327 evidence push started every public workflow:

- `ci` ran for about 33 minutes and failed the relevant SDK deletion gate only after about 30
  minutes of unrelated Rust/DSP work;
- `release build` passed after about 22 minutes;
- `browser qualification` passed after about 44 minutes, with the WebKit installation consuming
  about 41 minutes; and
- the existing SDK typecheck job completed in 8 seconds.

Live GitHub settings inspected on 2026-09-02 require eight Actions contexts:

- `host quality and native shell`;
- `x86 scalar and SIMD compile probes`;
- `browser Wasm artifact probes`;
- `cross-target digest gates under wasmtime`;
- `shipped AudioWorklet artifact`;
- `chromium — browser qualification gates`;
- `firefox — browser qualification gates`; and
- `webkit — browser qualification gates`.

The SDK and release jobs are not required, no ruleset adds checks, and strict branch synchronization
is disabled. This external state must be re-read immediately before branch-protection mutation.

## Decision

Separate engine, SDK, and browser qualification into independent concurrency domains and replace
the eight path-sensitive required contexts with three always-reporting aggregate contexts:

- `engine qualification`;
- `SDK qualification`; and
- `browser qualification`.

On pull requests, each aggregate always reports and uses `if: always()`. A lane selected by the
router passes only when all represented heavy jobs succeed; an unselected lane passes only when its
heavy jobs are skipped. Missing or failed router output is a failure.

Do not use workflow-level pull-request `paths:` for a workflow that owns a required context: a
skipped workflow would leave the required check pending. On `main` pushes, exact path ignores may
prevent an unrelated workflow from starting at all, so an SDK/evidence push cannot cancel an
already-running engine or browser run before an in-job router evaluates the diff.

## Path taxonomy

The classifier consumes both sides of renames and copies. Unknown, empty, unavailable, or malformed
diffs select full qualification.

### Evidence-only

- `.github/ISSUE_SPECS/**`;
- `docs/**`; and
- root `README.md`.

Do not classify arbitrary Markdown beneath packages as evidence. Evidence-only `main` pushes start
none of the four public workflows. Evidence-only pull requests still receive the three required
aggregate contexts, with no heavy jobs.

### SDK-only

- `sdk/**`;
- `scripts/check-sdk-deletions.py`;
- `scripts/check-sdk-generated.sh`;
- `scripts/check-sdk-headless.sh`;
- `scripts/check-sdk-types.sh`;
- `scripts/sdk-package.sh`; and
- `LICENSE`, which is copied into the package.

A diff containing only SDK and evidence paths selects SDK qualification only.

### Full engine/browser

Any other path selects full qualification. This includes Cargo/toolchain inputs, `crates/**`,
`hosts/**`, `sidecars/**`, `tools/**`, `fixtures/**`, Wasm build/pin inputs, workflows, the router
itself, and unclassified future paths. Mixed SDK plus engine changes are full. Full changes also run
SDK qualification because engine/Wasm changes can alter the embedded package.

## Workflow contract

### `.github/workflows/ci.yml`

- Pull requests targeting `main` always trigger.
- `main` pushes ignore the exact evidence-only and SDK-only sets.
- `workflow_dispatch` selects full qualification.
- Keep ref-scoped `cancel-in-progress: true` in an engine-specific concurrency domain.
- Route the four existing native/x86/browser-Wasm/cross-target jobs through the engine selection.
- Remove SDK generated/deletion/type/package ownership from native heavy jobs.
- Add `engine qualification`, `if: always()`, depending on the router and all four heavy jobs.

### New `.github/workflows/sdk.yml`

- Pull requests targeting `main` always trigger.
- `main` pushes run for every non-evidence change; engine/Wasm inputs therefore requalify the
  package.
- `workflow_dispatch` runs SDK qualification.
- Use an SDK-specific concurrency domain.
- One heavy SDK job installs the pinned Rust/Wasm and Node toolchains, runs `npm ci --prefix sdk`,
  builds the six-file AudioWorklet closure once, and reuses that exact artifact directory for:
  generated surface, deletion policy, strict types, headless evaluation, and package/tarball/
  `enginectl` qualification.
- Add `SDK qualification`, `if: always()`, depending on the router and heavy SDK job.

There is no durable tracked Wasm output. Do not download a mutable latest artifact. Content-keyed
caching is a future optimization; this issue builds the pinned closure once per selected SDK job.

### `.github/workflows/browser-qualification.yml`

- Pull requests targeting `main` always trigger.
- `main` pushes ignore the exact evidence-only and SDK-only sets.
- `workflow_dispatch` selects full qualification.
- Use a browser-specific concurrency domain.
- Gate the artifact producer and Chromium/Firefox/WebKit jobs behind the full route.
- Add `browser qualification`, `if: always()`, depending on the router, artifact job, and complete
  browser matrix.
- Preserve all current attestation, AudioWorklet, deployment-matrix, stall, native-digest, and FLAC
  checks with `fail-fast: false`.

### `.github/workflows/release-build.yml`

- Keep `workflow_dispatch` full and preserve the pull-request release-input filter.
- Ignore the exact evidence-only and SDK-only sets on `main` pushes.
- Keep its independent concurrency domain and do not make it required here.

`fuzz.yml` and `nightly.yml` are unchanged.

## Scope

- `.github/workflows/ci.yml`;
- `.github/workflows/browser-qualification.yml`;
- `.github/workflows/release-build.yml`;
- new `.github/workflows/sdk.yml`;
- a local path classifier/checker and its tests under `scripts/`; and
- this issue specification.

Unrelated uncommitted SDK dependency edits are excluded from all checkpoints.

## Objective gates

1. Classifier tests cover SDK source, every SDK-owned script, evidence, SDK plus evidence, engine,
   Wasm/pin, Cargo/toolchain/workflow/router, mixed inputs, both sides of renames, empty/missing-base,
   and manual-dispatch fail-safe behavior.
2. A static workflow checker proves exact/unique aggregate names, `if: always()`, complete heavy-job
   dependencies, unfiltered pull-request triggers, matching push ignore sets, full dispatch routing,
   and ref-scoped cancellation.
3. Red mutations are rejected when an engine path becomes SDK-only, one rename side disappears, a
   heavy job leaves its aggregate dependencies, an aggregate becomes conditional, or a required
   pull-request workflow gains a path filter.
4. Focused local SDK qualification passes using one artifact directory: generated, deletion,
   strict types, headless, package/tarball, and 9/9 `enginectl` tests.
5. `git diff --check` and workflow syntax checks pass.
6. The rollout run passes all selected engine, SDK, browser, and release jobs.
7. After branch-protection migration, a real SDK-only change reports all three aggregates while
   only SDK heavy work runs.
8. An evidence-only `main` push starts none of the four public workflows and cannot cancel an
   earlier engine/browser/release run.

## Required-check rollout

1. Re-query branch protection and rulesets.
2. Push the workflow implementation while the old eight contexts remain required. Workflow/router
   inputs select every heavy lane, exposing the three new aggregates.
3. Verify all three new contexts report from GitHub Actions and pass.
4. Atomically replace the old eight requirements with the three aggregate contexts.
5. Re-read branch protection and verify the exact three-context set and Actions app identity.
6. Never remove old requirements before the new contexts have actually reported.
7. Record run URLs, observed routing, and final protection state here before closure.

If protection changed since the brief, stop before mutation and amend the rollout.

## Non-goals

- Publishing `@misofm/engine`;
- changing npm contents, public SDK APIs, DSP, realtime, ABI, session, or control behavior;
- committing Wasm build output or consuming a mutable latest artifact;
- reducing qualification for engine or unknown paths;
- changing fuzz/nightly depth or optimizing runner caches; and
- including the unrelated local SDK dependency edits.

## Evidence

### Attempt 1 — Terra

The implementation introduces a fail-safe Git name-status classifier, a static workflow contract
checker, mutation/self-tests, an independent SDK workflow, route-gated engine/browser heavy jobs,
and always-scheduled aggregate contexts. SDK ownership moves out of the monolithic native/Wasm jobs
and into one package qualification job that builds and reuses a single pinned six-file artifact
closure.

Root pre-checkpoint review found and corrected two gate defects within attempt 1:

1. the first classifier mutant observed an engine-as-SDK misclassification instead of rejecting it;
   the checker now AST-pins unknown-path fallback to full and the mutant must fail; and
2. aggregate success truth tables were initially job-level conditions, which could skip a required
   aggregate after router/heavy failure. Aggregates now use exact `if: always()` and bounded env
   inputs; their steps fail router errors, unknown routes, selected-heavy non-success, and
   unselected-heavy non-skips.

Focused gates passed on 2026-09-02:

- `python3 -B scripts/check-ci-path-routing.py`;
- `python3 -B scripts/test-ci-path-routing.py`, including classifier, rename/copy, missing-base,
  aggregate-dependency, conditional-aggregate, router-failure, heavy-result, and PR-filter red
  mutations;
- YAML parsing of all four changed/new workflows with `yq`;
- SDK deletion and generated-surface checks; and
- exact-scope `git diff --check`.

Networked artifact/package qualification and the full engine/browser/release rollout remain for the
batch boundary. Sol/high adversarial review is required before that push.

### Attempt 1 adversarial verdict — Sol high: HOLD

The aggregate truth tables, trigger sets, concurrency domains, SDK ownership, browser matrix, and
release filtering passed review. Four blockers remain:

1. pull requests use a two-dot diff even though GitHub path semantics are three-dot, so an SDK-only
   feature branch behind `main` can be routed full;
2. the new static checker and mutation suite are not invoked by any workflow, leaving them dead CI
   code;
3. malformed/unknown Git status records such as `M100` and `X` can narrow to SDK instead of failing
   safe to full; and
4. the checker accepts an aggregate enforcement step with `continue-on-error: true`, which would
   suppress the required-context failure.

Attempt 2 must correct these four paths and add discriminating regressions before another Sol/high
verdict. Live protection remains the old eight Actions contexts; no rollout mutation has occurred.

### Attempt 2 — Sol medium HOLD correction

The bounded correction addresses exactly the four HOLD findings. Pull-request routing now diffs
`base...head`, matching GitHub's merge-base file set, while push routing retains the exact
`before head` two-endpoint transition. A real temporary Git repository regression creates an SDK
feature commit and a later, divergent engine commit on `main`; the same pair must classify `sdk`
for `pull_request` and `full` for `push`.

Name-status parsing now accepts only exact ordinary `A`, `D`, `M`, and `T` records, or `R`/`C`
followed by a three-digit score from 000 through 100 with both paths present. `U`, `X`, `B`, scored
ordinary statuses, absent/misshapen rename scores, out-of-range scores, incomplete records, and
invalid bytes all fail safe to full qualification. The regression corpus includes accepted
ordinary/rename/copy records and rejected `U`/`X`/`B`/`M100`/`R`/`R10`/`R101` records.

Each of the three independently triggered required-context workflows runs the static checker and
mutation suite in its always-scheduled `route` job before classification. This deliberately repeats
only the cheap hermetic policy checks: a single authoritative workflow cannot impose ordering on
another workflow, so it cannot prevent an SDK-only PR's SDK route from consuming unchecked router
or workflow code. Long engine, SDK artifact, and browser qualification remain unduplicated and
route-gated. Red mutations prove that removing either policy ownership or the mutation-suite call
is rejected.

Finally, the checker rejects any `continue-on-error` key in an aggregate job block, covering both
job-level and enforcement-step suppression. Separate red mutations prove both placements fail.
Aggregate truth tables, exact taxonomy, independent concurrency, and the old required-context
rollout order are unchanged.

Focused local evidence on 2026-09-02:

- `python3 -B scripts/check-ci-path-routing.py`;
- `python3 -B scripts/test-ci-path-routing.py`;
- `yq eval '.'` over `ci.yml`, `browser-qualification.yml`, `release-build.yml`, and `sdk.yml`;
- Python syntax compilation for the router, checker, and mutation suite;
- `bash scripts/check-sdk-generated.sh`;
- `python3 -B scripts/check-sdk-deletions.py`; and
- exact-scope `git diff --check`.

All passed locally. No networked rollout, branch-protection mutation, commit, push, benchmark,
timing, playback, or unrelated SDK dependency edit is part of this correction. A fresh Sol/high
adversarial verdict remains required.

### Attempt 2 adversarial verdict — Sol high: HOLD

The four attempt-1 runtime findings are resolved, but the lexical checker accepts dangerous valid
YAML variants:

1. an extra unquoted `- crates/**` in the engine push ignore list is invisible to the single-quote
   extractor, so crate-only main pushes could skip engine CI while the checker passes;
2. a quoted `"continue-on-error": true` on aggregate enforcement bypasses the literal rejection;
   and
3. `if: ${{ false }}` or `continue-on-error: true` on a route-policy step can prevent/falsify the
   pre-classification check while the checker sees only the command text and ordering.

Attempt 3 is the final allowed attempt. It must pin the security-critical trigger, policy-step, and
aggregate blocks against semantic-equivalent YAML spellings and add each bypass as a red mutation.
No rollout or protection mutation has occurred; the old eight required contexts remain intact.

### Attempt 3 — Sol medium final HOLD correction

The final bounded correction replaces the vulnerable scalar extractors with dependency-free,
byte-exact canonical contracts for the complete trigger block, complete route job, and complete
aggregate job. It additionally pins the unique top-level mapping sequence and each workflow's exact
job-key sequence, so a quoted or duplicate `on`, `route`, or `qualification` key cannot override a
canonical block outside the region the checker inspected. The remaining lexical assertions are
secondary diagnostics; they are no longer the authority for these YAML semantics.

The mutation suite now rejects all of the attempt-2 HOLD reproductions and equivalent spellings:

- extra engine push-ignore entries expressed as unquoted, double-quoted, or explicitly tagged YAML
  scalars;
- route-policy `if: ${{ false }}`;
- route-job and route-policy-step `continue-on-error` using ordinary, double-quoted, or
  single-quoted keys, plus shell-level `|| true` suppression; and
- aggregate-job and enforcement-step `continue-on-error` using ordinary, double-quoted, or
  single-quoted keys.

The two live adversarial reproductions left in `ci.yml` were removed. No runtime route, path
taxonomy, trigger, concurrency, heavy-job, aggregate truth-table, or rollout behavior changed.

Focused local evidence on 2026-09-03:

- `python3 -B scripts/check-ci-path-routing.py`;
- `python3 -B scripts/test-ci-path-routing.py`;
- `yq eval '.'` over `ci.yml`, `browser-qualification.yml`, `release-build.yml`, and `sdk.yml`;
- `bash scripts/check-sdk-generated.sh`;
- `python3 -B scripts/check-sdk-deletions.py`; and
- exact-scope `git diff --check`.

All passed locally. No networked rollout, branch-protection mutation, commit, push, benchmark,
timing, playback, or unrelated SDK dependency edit is part of this final correction. A final
Sol/high adversarial verdict remains required; any further HOLD is the issue's terminal stop.
