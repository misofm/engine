# Hold every block-form owner step in qualification.yml to its guard lines

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the attempt-1 verdict of *Fail the web AudioWorklet test step when it leaves anything in its
temporary directory* (#1429, `/home/bl/misofm/submix-verdicts/1429-attempt1.md`, NIT); root filed
it. CI tooling only; no engine code changes.

## Problem (verified on `codex/d15-stream-j2` at `2944fc63f`, which contains #1429 and its follow-ups)

- **The owner lint checks the owner command, not the lines that make its step a guard.**
  `scripts/check-ci-path-routing.py` `check_qualification_deduplicated_owners` (`:774-787`) requires
  each command in `DEDUPLICATED_OWNERS` (`:741-753`) among the job's unconditional step commands
  (`unconditional_step_commands`, `:916-917`, over `step_commands`, `:892-913`, which splits a
  `run: |` block into its lines). A line in the same block that is not an owner command is never
  checked.
- **#1429's guard is such a block.** `artifact-gates`' step "Hermetic browser host and worklet
  tests" (`.github/workflows/qualification.yml:351-364`) runs `bash scripts/test-web-audioworklet.sh`
  (the owner command) between `set -o pipefail`, a private `TMPDIR` and an emptiness check. Deleting
  the emptiness check (`:360-364`), the `export TMPDIR` (`:358`) or `set -o pipefail` (`:355`)
  leaves the lint green, and #1429's guarantee is gone with every job green.
- **The other block-form owners have the same gap.** `lint`'s step "Effect runtime
  dependency-boundary and fixture policy and mutation tests" (`qualification.yml:553-558`) runs its
  four owner commands as one block. A `set +e` line before them masks every failure while each
  owner command is still present verbatim, so the lint stays green. Step-level `continue-on-error`
  is refused for some jobs (`:577`, `:1042`), not for the owner steps.

## Decisions

- **D1. Each block-form owner step is named and holds its guard lines.** Extend
  `DEDUPLICATED_OWNERS` (or a sibling table beside it) so each owner whose step is a `run: |` block
  names the step (by its `name:`) and the lines that must stay in that block, in order, besides the
  owner commands: for `artifact-gates`' step, `set -o pipefail`, the `mktemp -d` line, the `trap`
  line, `export TMPDIR="$scratch"`, the owner command, the `find` line and the `if [[ -n
  "$leftover" ]]` check with its `exit 1`. A missing, reordered or edited guard line fails the lint
  with a message naming the line and the issue that owns it (#1429, #1044).
- **D2. No owner step may mask a failure.** In every owner step (block or single line): no
  step-level `if:` (as today), no step-level `continue-on-error`, and no line that disables or
  swallows failure: `set +e`, `set +o errexit`, `set +o pipefail`, `|| true`, `|| :`, a trailing
  `&`, or `exit 0` before an owner command. The lint refuses each by name.
- **D3. Self-test cases.** In `scripts/test-ci-path-routing.py`, one mutation case per D1 guard
  line of #1429's step (delete it: red), one that moves the emptiness check before the owner
  command (red), and one per D2 construct on an owner step (red). The existing #1044 cases stay.

## Authorized paths

- `scripts/check-ci-path-routing.py`
- `scripts/test-ci-path-routing.py`
- This spec

## Non-goals

- Changing any workflow step. If D1 or D2 finds an owner step that masks a failure today, stop and
  report it.
- Owners outside `qualification.yml`.

## Hazards

- `.github/workflows/qualification.yml` is a hot file; this issue reads it and does not edit it. A
  slice that edits an owner step after this lands must update the D1 table in the same change, and
  the lint says so in its message.
- Pinning whole step bodies byte for byte would be ceremony (AGENTS.md); D1 pins only the lines
  that carry a claim, each named with its owning issue.

## Objective gates

1. `python3 -B scripts/check-ci-path-routing.py` and `python3 -B scripts/test-ci-path-routing.py`
   exit 0 on the branch's `qualification.yml`.
2. **Red on revert (PR evidence).** The new self-test cases, run against the checker at this
   issue's parent commit, are accepted (no failure); with the change, each is refused.
3. **Each D1/D2 rule has a catch (PR evidence).** Remove each rule from the checker in turn: at
   least one D3 case turns red, and no existing case already catches it.
4. `bash scripts/check-workspace-policy.sh` exits 0.

*Test value.* Each D3 case is red when an edit to `qualification.yml` removes a guard line from an
owner step, or masks an owner command's failure, which today leaves every job green while the
claim the step owns is no longer checked.

## Evidence

- Gates 1 to 4; the D1 table with each line's owning issue.

## Dependencies

- *Fail the web AudioWorklet test step when it leaves anything in its temporary directory*
  (#1429): D1 holds its guard lines.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused; the checker reads the workflow, which is its input.
- Attempt budget: three attempts, one adversarial verdict each.
