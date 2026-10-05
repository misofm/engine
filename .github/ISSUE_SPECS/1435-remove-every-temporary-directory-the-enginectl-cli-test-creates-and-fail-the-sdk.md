# Remove every temporary directory the enginectl CLI test creates, and fail the SDK qualify step on a leftover

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the stream-J batch-2 verdict, "Observation outside the batch"
(`/home/bl/misofm/submix-verdicts/stream-j2-batch-verdict.md`); root filed it. It is the defect
class of *Remove both temporary directories the web AudioWorklet test script creates* (#1421), and
the guard is the one *Fail the web AudioWorklet test step when it leaves anything in its temporary
directory* (#1429) put on `artifact-gates`. Test and CI only; no engine or SDK product code
changes.

## Problem (verified on `codex/d15-stream-j2` at `4387b935e`)

- **Six tests make a temporary directory and none removes it.** `sdk/test/enginectl-cli.mjs` calls
  `mkdtemp(resolve(tmpdir(), ...))` in six tests and imports no `rm` (`:5`):
  - `:198` `enginectl-retired-stems-` ("retired --stems is unknown and publishes no output");
  - `:206` `enginectl-request-precedence-` ("request input refusal precedes a retired stems-only
    output preflight");
  - `:273` `enginectl-eval-` ("file output is published before its matching receipt");
  - `:329` `enginectl-report-eval-` ("a post-publication stdout failure reports effect applied
    exactly once");
  - `:429` `enginectl-submix-` ("submix strips and bus taps build the engine's canonical JSON;
    submix_output is refused by name");
  - `:532` `enginectl-vca-` ("a request's vcas reach the document members first, in the engine's
    canonical JSON").
  Each run leaves six directories in `TMPDIR`, with the files the tests wrote in them. The batch
  verdict's run left six. On this host, `/tmp` holds 56 `enginectl-eval-*`, 56
  `enginectl-report-eval-*`, 56 `enginectl-request-precedence-*`, 56 `enginectl-retired-stems-*`,
  51 `enginectl-submix-*` and 19 `enginectl-vca-*` directories (2026-10-05).
- **The one CI run of the test.** `scripts/sdk-package.sh:50` runs
  `node --test "$sdk_root/test/enginectl-cli.mjs"`. `qualification.yml`'s `sdk` job (`:225`), step
  "Qualify the SDK package against the shared artifact" (`:274-280`), runs
  `bash scripts/sdk-package.sh check target/ci/qualification-artifacts` as the last of five
  commands, with the runner's shared temporary directory, so the leak is invisible and every job
  is green.
- **The other temporary directories in that step are cleaned today** (read, not run):
  `scripts/sdk-package.sh:17-22` (one `cleanup` behind one `trap cleanup EXIT`),
  `scripts/test-sdk-artifact-builder-output-contract.sh:6-10`, `scripts/check-sdk-headless.sh:29-30`,
  `sdk/test/headless-path-evals.mjs:24` and `:58`, and `sdk/test/console-evals.mjs:49` and `:53`.
- **The routing checker pins the step's commands.** `scripts/check-ci-path-routing.py`
  `SDK_CLOSURE_LINES` (`:429-435`) and `check_qualification_closures` (`:452-458`) require each of
  the five command lines in the `sdk` job's text.

## Decisions

- **D1. Each test removes its directory on every exit path.** In `sdk/test/enginectl-cli.mjs`,
  each of the six directories is removed with `rm(directory, { recursive: true, force: true })`
  when its test ends: passed, failed on an assertion, or thrown. Use `try { .. } finally { .. }`
  around the test body after `mkdtemp`, or the test context's `after` hook registered right after
  `mkdtemp`; one shape for all six. What each test asserts does not change.
- **D2. The SDK qualify step runs in a fresh, private temporary directory and fails on a
  leftover.** The step at `qualification.yml:274-280` becomes a block in #1429's shape:
  `set -o pipefail`; `scratch="$(mktemp -d "$RUNNER_TEMP/sdk-qualify-tmp.XXXXXX")"`; an `EXIT`
  trap that removes it; `export TMPDIR="$scratch"`; the five commands unchanged, each on its own
  line, in today's order; then a `find "$scratch" -mindepth 1 -maxdepth 1` that, if anything is
  left, prints every entry and exits 1. A command's own failure still fails the step first (GitHub
  runs a step with no `shell:` as `bash -e {0}`); it is never masked.
- **D3. Everything the step runs is held to it.** If the first run finds a leftover from anything
  other than D1's six directories, stop and report the creator and the entry; do not fix it here
  unless it is in the authorized paths, and never weaken the check with an exemption list.
- **D4. The routing checker stays green as it is.** The five commands keep their exact text, so
  `check_qualification_closures` still finds each. If it does not, extend the checker to recognise
  the block form (not to relax it), with a self-test case in `scripts/test-ci-path-routing.py`. No
  router change (`scripts/ci-path-router.py`), and the `sdk` job keeps its `if:` and route.

## Authorized paths

- `sdk/test/enginectl-cli.mjs` (D1's cleanup only)
- `.github/workflows/qualification.yml` (the `sdk` job's step at `:274-280` only)
- `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` (only if D4 needs them)
- This spec

## Non-goals

- Changing what any SDK test or script tests.
- Temporary files of other steps or jobs (the `artifact-gates` step is #1429's).
- Holding the new guard lines in the routing checker. That is the class of *Hold every block-form
  owner step in qualification.yml to its guard lines* (#1434); see Hazards.

## Hazards

- **Hot file.** `.github/workflows/*.yml` is in `STREAMS.md`'s merge-order table. This slice edits
  one step of the `sdk` job; #1422, #1429 and #1334 (stream H) edit other parts of
  `qualification.yml`. The slice that lands later rebases.
- **SDK files and their owners.** `sdk/` is stream H's (`STREAMS.md`, stream H "Owns"), with named
  exceptions: G #1369 (`sdk/`), I #1335 (`sdk/src/core/session.ts`), E #1364
  (`sdk/src/core/live-controls.ts`) and C #1406 (the ring assertions of
  `sdk/test/{boot,browser,builder,render}-evals.mjs`). None of them names
  `sdk/test/enginectl-cli.mjs`; this issue edits it by named exception, for D1's cleanup only. If
  another stream's open slice edits the same file (for example H #1385, which encodes session
  edits the CLI tests build), the later slice rebases; a cleanup-only diff rebases trivially.
- **#1434.** If #1434 lands first, its block-form table does not list this step's guard lines; if
  this lands first, #1434's implementer reads the step as it is. Neither blocks the other.
- **CI-conscious batching (AGENTS.md).** Do not push for a CI run; the CI half of gate 4 comes from
  the batch's single pull-request run.

## Objective gates

Gates 1 to 3 run the step's exact shell body, extracted from the YAML, under
`bash --noprofile --norc -e` with `RUNNER_TEMP` set to a scratch directory, after `npm ci` in
`sdk/` and the artifact build the job downloads (`target/ci/qualification-artifacts`).

1. **Red on the current script (PR evidence).** With `sdk/test/enginectl-cli.mjs` from this
   issue's parent commit and the new step: exit 1, and the message names six entries, one each
   with the prefixes `enginectl-retired-stems-`, `enginectl-request-precedence-`,
   `enginectl-eval-`, `enginectl-report-eval-`, `enginectl-submix-` and `enginectl-vca-`;
   `RUNNER_TEMP` is empty afterwards.
2. **Green after.** With the change: exit 0, and `RUNNER_TEMP` is empty afterwards.
3. **Every exit path (PR evidence).** Insert `assert.fail("probe")` right after the `mkdtemp` line
   of the submix test (`:429`) and run the step body: it exits non-zero on the test failure, and
   `node --test sdk/test/enginectl-cli.mjs` run the same way with a fresh `TMPDIR` (and
   `ENGINECTL` as `scripts/sdk-package.sh:50` sets it) leaves that `TMPDIR` empty. Revert.
4. **The checkers.** `python3 -B scripts/check-ci-path-routing.py`, its self-test
   `python3 -B scripts/test-ci-path-routing.py` and `bash scripts/check-workspace-policy.sh` exit
   0; the batch PR's `sdk` job runs the step and passes.

*Test value.* The step's emptiness check is red when any of the SDK qualify step's five commands,
the enginectl CLI test above all, leaves a temporary file or directory behind, which today no gate
sees: the runner's shared temporary directory hides it.

## Evidence

- Gates 1 to 3's runs and outputs; any D3 leftover found and what was done with it.

## Dependencies

- *Fail the web AudioWorklet test step when it leaves anything in its temporary directory*
  (#1429): the guard shape this step copies.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
