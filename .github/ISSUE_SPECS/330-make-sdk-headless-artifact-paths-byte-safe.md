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

The shell-to-Node boundary must not place raw pathname bytes in a JavaScript string environment
value. After physical resolution, encode the canonical bytes as deterministic lowercase ASCII hex.
Node validates that representation, decodes it to a `Buffer`, appends the fixed ASCII Wasm basename
as bytes, and passes the resulting `Buffer` directly to `fs`; it never calls `path.resolve` on the
artifact directory. Missing or malformed encoding is a clear typed test-helper failure.

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
2. The child runs from exact SDK root, receives a deterministic ASCII encoding of the invariant
   artifact-path bytes, sees valid Wasm through a `Buffer` path, and receives exactly
   `--test 'test/*-evals.mjs'`.
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

### Attempt 1 — Sol medium implementation

`scripts/check-sdk-headless.sh` retains its argument-count, `-d`, and direct-final-component `! -L`
checks. It now enters the accepted directory with physical `cd -P`, prints Bash's physical `$PWD`
plus one known non-newline `x` byte inside command substitution, and removes exactly that final
byte after capture. Appending data before capture prevents command substitution from consuming any
terminal newline bytes belonging to the path; removing one final `x` is unambiguous even when the
pathname itself ends in `x`. A failed physical transition reports validation failure and exits 2.
The validated absolute path is then checked for the Wasm module and exported unchanged after the
script enters `sdk/`. Node's status remains the shell's unsuppressed final status.

The new automatically discovered `sdk/test/headless-path-evals.mjs` invokes the production shell
script rather than reimplementing it. It accepts an alternate exact script path through
`MISO_ENGINE_HEADLESS_SCRIPT_UNDER_TEST` for immutable-history probes. Its expected physical path
comes independently from Node/libc `realpath`, never shell `pwd` or production capture. A fake Node
process verifies exact SDK cwd, environment-string equality, visibility and validity of the
eight-byte minimal Wasm module, and exact `--test` / `test/*-evals.mjs` arguments.

The executable matrix covers ordinary relative and absolute paths, spaces, tabs and shell
metacharacters, an embedded newline, one terminal newline, two terminal newlines, a final sentinel-
like `x`, invalid arity, missing and non-directory inputs, missing module, direct symlink,
unsearchable directory, and propagation of Node status 37. Results from the same test file:

- exported parent `951a5a3c`: **RED**, 4 passed / 9 failed, including ordinary relative and
  physical-path invariance failures;
- exported failed attempt `68eef8d6`: **RED**, 10 passed / 3 failed, specifically one terminal
  newline, repeated terminal newlines, and unsearchable status 1 instead of 2; and
- this successor working tree: **GREEN**, 13 passed / 0 failed.

The complete existing headless glob was also run with a separately preserved valid local Wasm
build: all **124 tests in 28 suites passed**, including this new suite, proving automatic discovery.
That local Wasm is test evidence only and was not committed. The canonical build gate itself was
invoked once and stopped before copying output because this macOS host produced digest
`1fe4b9cec4fb0373067f24f29af0d77eb4e1a3d9d36214dd654aa917c98c7821` rather than pinned
`6ddf154d02fcb4dfaa1a397280a28ab9f38b0cd6dff466a316f120266ce2223f`; no digest was changed,
repinned, or treated as PASS.

Focused local evidence on 2026-09-03:

- `bash -n scripts/check-sdk-headless.sh` and `shellcheck scripts/check-sdk-headless.sh`;
- `node --test sdk/test/headless-path-evals.mjs`;
- the same test with alternate exact script paths in isolated `git archive` exports of
  `951a5a3c` and `68eef8d6`;
- the full `bash scripts/check-sdk-headless.sh <valid-local-artifact>` invocation (124/124);
- unchanged `python3 -B scripts/check-ci-path-routing.py` and
  `python3 -B scripts/test-ci-path-routing.py`;
- `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-generated.sh`, and
  `python3 -B scripts/check-sdk-deletions.py`;
- unchanged workflow YAML parsing and exact `git diff --check`; and
- the exact proposed paths classify `sdk` through the production router.

`scripts/sdk-package.sh` is byte-unchanged from brief commit `6cc75062`. Static inspection confirms
its artifact argument is consumed by `stage-package.mjs` before the later SDK-directory transition,
which is confined to the npm-pack subshell; no executable evidence contradicted the brief's ruling.
Only `scripts/check-sdk-headless.sh`, `sdk/test/headless-path-evals.mjs`, and this issue evidence are
changed. No commit, push, workflow, router, checker, ignore list, digest, package, GitHub,
branch-protection, or active-run mutation occurred. Fresh Sol/high review is still required; this
attempt does not claim PASS.

### Attempt 1 — Sol/high HOLD

Sol/high held attempt 1. The sentinel protected only the artifact-path capture. Repository-root
discovery still used nested bare command substitutions (`dirname` and `pwd -P`), so a repository
path ending in newline bytes was not preserved. Both physical transitions also inherited caller
`CDPATH`: `CDPATH=.` could add `cd` output to the captured value, while a hostile search entry could
select an existing shadow directory. Redirecting `cd` output would not correct the shadow-selection
semantics.

The direct-final-symlink check was lexical only for the exact argument. Appending `/`, `/.`, or the
equivalent repeated spelling made `-L` inspect the resolved target and allowed a direct
caller-supplied symlink directory through. The unsearchable fixture also needed to retain its
non-root assertion when the test runner itself is root rather than silently accepting root's
ability to traverse mode `000`. These are contract findings, so attempt 1 remains **HOLD**.

### Attempt 2 — Sol medium correction

The production script now uses one sentinel-bearing `capture_physical_directory` helper for both
repository-root and artifact resolution. Each transition executes `cd -P` with `CDPATH=''`, so no
caller search path can select or print a different directory. Repository-root discovery uses Bash
parameter expansion rather than capturing `dirname`; both captures append one `x` after physical
`$PWD` and remove exactly that byte. Either failed transition reports a diagnostic and exits 2.

Before artifact resolution, a separate link probe removes only equivalent trailing directory
syntax (`/` and `/.`, including repetitions). Thus `link`, `link/`, `link/.`, and `link//./` all
retain the direct-final-symlink rejection. A symlink in an ancestor component remains accepted and
`cd -P` resolves it to the independent physical-path oracle, preserving the intended policy rather
than rejecting every path containing a symlink.

The same automatically discovered eval file now additionally covers:

- workflow-form `scripts/check-sdk-headless.sh` invocation with `CDPATH=.` and with an existing
  repository-root shadow;
- artifact selection under an existing `CDPATH` shadow in both relative and absolute forms;
- a copied repository root ending in two newline bytes;
- direct symlink spellings with `/`, `/.`, and repeated slash/dot syntax, plus an accepted ancestor
  symlink; and
- unsearchable status under both an ordinary runner and a root runner, where the latter drops the
  child to numeric uid/gid 65534 before making the assertion.

Node/libc `realpath` remains the independent oracle. The fake child still checks exact SDK cwd,
artifact environment bytes, valid minimal Wasm, exact glob arguments, and status propagation.
Focused executable results from the expanded 17-test file are:

- exported parent `951a5a3c`: **RED**, 3 passed / 14 failed;
- exported failed attempt `68eef8d6`: **RED**, 10 passed / 7 failed, including terminal-newline,
  `CDPATH`, repository-root-byte, direct-symlink-spelling, and unsearchable-status failures; and
- this attempt-2 working tree: **GREEN**, 17 passed / 0 failed.

No workflow, router, checker, ignore list, package script, digest, or package dependency file was
changed. Additional proportional gates and fresh Sol/high review remain required; this correction
records implementation evidence and does **not** claim PASS.

Attempt-2 proportional evidence on 2026-09-03:

- `bash -n` and `shellcheck` passed for `scripts/check-sdk-headless.sh`;
- the focused eval passed 17/17, and the exact same file produced the historical red results above
  through alternate script paths in isolated `git archive` exports;
- `python3 -B scripts/check-ci-path-routing.py` and
  `python3 -B scripts/test-ci-path-routing.py` passed unchanged;
- SDK types, generated surface, and deletion gates passed; all workflow files parsed with `yq`;
- the exact three-path range from `origin/main` is this spec, the headless script, and the SDK eval,
  and production path classification returned `sdk` for both pull-request and push inputs;
- an executable `scripts/sdk-package.sh build` used a caller-relative six-artifact directory whose
  final byte was a newline; staging, all 9 enginectl tests, and package-tree preparation passed.
  `scripts/sdk-package.sh` remained byte-unchanged;
- a locally compiled `simd128` Wasm drove the production headless script and its existing glob:
  all 128 tests in 28 suites passed, including automatic discovery of the 17-test regression; and
- `git diff --check` passed and status contains exactly the three scoped files.

A preliminary full-glob setup accidentally supplied a scalar Wasm under the SIMD artifact name;
127/128 tests passed and the capability eval correctly rejected `scalar` versus `simd128`. Rebuilding
the disposable fixture with `-C target-feature=+simd128` produced the 128/128 result above. Neither
artifact is tracked, and this setup correction is not presented as a retry of a production gate.
Fresh Sol/high review is still pending, so attempt 2 remains implementation evidence rather than a
PASS verdict.

### Attempt 2 — Sol/high HOLD

Sol/high held attempt 2 because its claim stopped at Bash. POSIX pathname components may contain
any non-NUL byte, but the script placed the raw canonical pathname in
`MISO_ENGINE_SDK_ARTIFACTS`. Node exposes environment values as JavaScript strings, so an invalid
UTF-8 byte is decoded lossily rather than preserved. `support.mjs` then passed that string through
`path.resolve`; neither the fake child nor the oracle exercised a non-UTF-8 filename. Therefore the
attempt-2 evidence proved newline-safe strings, not the issue's all-accepted-bytes objective.

This is a production boundary defect rather than an evidence-only omission: a Linux directory with
component byte `0xff` can pass Bash validation and physical resolution, then become a replacement
character before Node opens the Wasm. Attempt 2 remains **HOLD**.

### Attempt 3 — Sol medium final correction

All attempt-2 physical resolution, `CDPATH`, exit-status, symlink-spelling, and ancestor-symlink
behavior remains intact. After validating the physical directory and Wasm, the production script
now streams the raw Bash pathname bytes through `od -tx1` and removes only ASCII whitespace from
that tool's output. It validates canonical lowercase, even-length hex and exports only
`MISO_ENGINE_SDK_ARTIFACTS_HEX`; encoding failure maps to validation status 2. No raw pathname byte
crosses the environment boundary.

`sdk/test/support.mjs`, the single narrow helper allowed by the brief, validates the hex contract,
decodes it with `Buffer.from(encoded, "hex")`, appends the fixed ASCII Wasm basename with
`Buffer.concat`, and gives that `Buffer` directly to `readFile`. It no longer converts the artifact
directory through a JavaScript path string or `path.resolve`. Missing, odd-length, non-hex, and
non-canonical uppercase encodings produce explicit errors naming
`MISO_ENGINE_SDK_ARTIFACTS_HEX`.

The fake child likewise validates exact encoded bytes, independently decodes a `Buffer` path,
opens valid minimal Wasm, and checks the exact test arguments. The regression adds a real raw-byte
fixture: Node Buffer filesystem APIs create a directory whose final component is byte `0xff`, a
small ASCII-only Bash launcher constructs the same argument with `printf '\377'`, and Node/libc
`realpath(..., { encoding: "buffer" })` supplies the independent expected hex. The launcher never
passes the raw pathname through a JavaScript argument or environment string. The same test calls
production `support.mjs` with only the ASCII encoding and verifies valid Wasm. A filesystem skip is
permitted only when creation returns a platform unsupported-encoding error; Linux treats `EPERM`
as a failure, so CI must execute rather than skip the case. This macOS APFS fixture returned
`EPERM` and was recorded as the permitted platform skip.

Focused results from the exact 19-test file on macOS are 18 passed, 0 failed, 1 unsupported-
filesystem skip. Historical alternate-script probes remain red:

- parent `951a5a3c`: 4 passed / 14 failed / 1 skipped;
- failed attempt `68eef8d6`: 3 passed / 15 failed / 1 skipped; and
- attempt-2 checkpoint `62a0b0f1`: 5 passed / 13 failed / 1 skipped, with accepted paths failing
  because the old script supplies no encoded-byte environment value.

A locally compiled SIMD Wasm drove the real production headless glob: 129 passed, 0 failed, and the
same one APFS-only skip across 130 tests in 28 suites. The automatically discovered regression and
all existing consumers of `moduleBytes` therefore exercised the new helper contract. An unchanged
`scripts/sdk-package.sh build` again accepted a caller-relative artifact directory ending in a
newline and passed staging plus all 9 enginectl tests.

Final proportional gates passed: Bash syntax and ShellCheck; the path-routing checker and mutation
suite unchanged; SDK types, generated surface, and deletion policy; all workflow YAML through
`yq`; and `git diff --check`. The exact range from `origin/main` contains only this spec, the
headless script, the headless-path eval, and `sdk/test/support.mjs`; production classification
returns `sdk` for pull-request and push inputs. Workflow, router, package script, package dependency,
and checker diffs from attempt-2 checkpoint `62a0b0f1` are empty. Fresh Sol/high review is required;
this final implementation attempt does **not** claim PASS.
