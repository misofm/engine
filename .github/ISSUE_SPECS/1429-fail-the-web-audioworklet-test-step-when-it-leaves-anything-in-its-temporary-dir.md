# Fail the web AudioWorklet test step when it leaves anything in its temporary directory

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the attempt-1 verdict of *Remove both temporary directories the web AudioWorklet test script
creates* (#1421, `/home/bl/misofm/submix-verdicts/1421-attempt1.md`, note for root); root filed it.
CI only; no engine code changes.

## Problem (verified on `codex/d15-stream-j2` at `34274bf34`, which contains #1421)

- **#1421's cleanup has no committed guard.** `scripts/test-web-audioworklet.sh` now makes its two
  temporary directories under `${TMPDIR:-/tmp}` (`:19` and `:48`) and removes both from one
  `cleanup` behind one `trap cleanup EXIT` (`:5-13`). #1421's spec decided on no committed test,
  so its evidence is a one-time before/after run. A later edit that adds a second `EXIT` trap
  (bash keeps only the last one), makes a third directory, or drops a branch of `cleanup` leaks a
  directory on every CI run, with every job green.
- **The one CI run of the script.** `.github/workflows/qualification.yml:348-349`, step "Hermetic
  browser host and worklet tests" in `artifact-gates`, runs `bash scripts/test-web-audioworklet.sh`
  with the runner's default temporary directory, which other steps share, so a leak there is
  invisible.
- **The routing checker pins that command.** `scripts/check-ci-path-routing.py:741-742`
  (`DEDUPLICATED_OWNERS`, `artifact-gates`) and `:774-782` require
  `bash scripts/test-web-audioworklet.sh` among `artifact-gates`' unconditional step commands
  (#1044): it is the one run of the hermetic host/worklet/boot suite.

## Decisions

- **D1. The step runs the script in a fresh, private temporary directory and fails if anything is
  left in it.** In the step at `qualification.yml:348-349`: make a fresh directory (under
  `$RUNNER_TEMP`), run the script with `TMPDIR` set to it, then fail the step, naming every entry
  left, if the directory is not empty; remove the directory last. The script's own exit status
  still fails the step (the check runs only after a successful script run, or the step reports both;
  either way a script failure is never masked).
- **D2. Everything the step runs is held to it.** Any process the script starts that honours
  `TMPDIR` (node, bun, `mktemp`) writes there. If the first run finds a leftover from anything
  other than the script's two directories, the creator is fixed to clean up (within the authorized
  paths), or the slice stops and reports it; the check is never weakened with an exemption list.
- **D3. The #1044 owner stays visible.** The step keeps `bash scripts/test-web-audioworklet.sh` as
  an unconditional command that `check_qualification_deduplicated_owners` finds. If the wrapped
  form is no longer recognised by `unconditional_step_commands`, extend the checker to recognise it
  (not to relax it), with a self-test case in `scripts/test-ci-path-routing.py` showing the wrapped
  form passes and the step without the command fails.
- **D4. No router change.** The step stays in `artifact-gates`, on the full route; the router
  (`scripts/ci-path-router.py`) and the verdict's expectation table do not change.

## Authorized paths

- `.github/workflows/qualification.yml` (the step at `:348-349` only)
- `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` (only if D3 needs them)
- `scripts/test-web-audioworklet.mjs` and `scripts/test-web-audioworklet.sh` (only if D2 finds a
  leftover they create)
- This spec

## Non-goals

- The same guard for other scripts' temporary files (a separate issue if wanted).
- Changing what the test script tests.

## Hazards

- `.github/workflows/*.yml` is a hot file (`STREAMS.md`, merge-order table); this change edits one
  step. The slice that lands second rebases.
- CI-conscious batching (AGENTS.md): do not push for a CI run; gate 4's CI evidence comes from the
  batch's single pull-request run.

## Objective gates

1. **The step passes locally.** Run the step's exact shell body (with `RUNNER_TEMP` set to a
   scratch directory) after the artifact build the job depends on: exit 0, and the temporary
   directory is gone afterwards.
2. **Red on #1421's revert (PR evidence).** Restore `scripts/test-web-audioworklet.sh` to its state
   before #1421 (commit `26208ceaa^`) and run the step's body: it exits non-zero and names the
   leftover `miso-engine-host.*` entry. Revert.
3. **A second EXIT trap is red (PR evidence).** Add `trap 'true' EXIT` after `:13` and run the
   step's body: it exits non-zero, naming the leftover directories. Revert.
4. **The checkers.** `python3 -B scripts/check-ci-path-routing.py`, its self-test
   `python3 -B scripts/test-ci-path-routing.py` and `bash scripts/check-workspace-policy.sh` exit
   0; the batch PR's `artifact-gates` job runs the step and passes.

*Test value.* The step's emptiness check is red when the AudioWorklet test script leaves a
temporary file or directory behind, which today no gate sees: the runner's shared temporary
directory hides it.

## Evidence

- Gates 1 to 3's runs and outputs; any leftover D2 found and how it was fixed.

## Dependencies

- *Remove both temporary directories the web AudioWorklet test script creates* (#1421): this guard
  holds #1421's cleanup.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
