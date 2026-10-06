PASS

# #1429 attempt 1 verdict: fail the web AudioWorklet test step when it leaves anything in its temporary directory

Commit under review: `b7da37f15` (branch `codex/d15-stream-j2`, parent `7caf972db`). I exported it to
`/tmp/claude-1002/v1429/tree` and ran it there with `CARGO_TARGET_DIR=/tmp/claude-1002/v1429/target`.
I did not change the worktree.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

1. **The attempt record gives the wrong CI shell. The gate-1 evidence ran under a shell that CI
   does not use.** Spec `.github/ISSUE_SPECS/1429-...md:107` says "Under GitHub's
   `bash -eo pipefail`", and `:119` ran gate 1 with `bash --noprofile --norc -eo pipefail`. The
   step has no `shell:`, and neither the job nor the workflow has `defaults.run.shell`. I parsed
   the YAML to confirm this. So GitHub uses its default for an unspecified shell, `bash -e {0}`,
   which has no `pipefail`. The `--noprofile --norc -eo pipefail` command applies only to an
   explicit `shell: bash`. The workflow itself shows this: it adds `set -o pipefail` explicitly at
   `qualification.yml:65`, `:198`, `:746` and in each closure substitution.
   - The behaviour still holds under the real shell. I ran gates 1 to 3 and the variants below
     under `bash -e` and got the same results as under the implementer's shell.
   - The one difference is the `find | LC_ALL=C sort` pipeline at `qualification.yml:357`. Under
     `bash -e`, a `find` failure is not propagated (`sort` exits 0, so `leftover` is empty).
     I checked both ways this can happen:
     - Scratch removed: nothing can be left, so a pass is correct.
     - Scratch unreadable, with a leaked file inside: the step still went red (exit 1). But only
       because the EXIT trap's `rm -rf` also fails, and under `-e` a failing trap command sets the
       exit status. The check does not fail closed by itself.
   - Fix: correct the record (`bash -e {0}`). Optionally add `set -o pipefail` to the step body,
     as the other steps do, so the emptiness check fails closed by itself and not through a side
     effect of the trap.

### NIT

1. **No checker protects the guard itself** (out of scope; for root). If you delete the
   `leftover=`/`if` lines at `qualification.yml:357-361`, `check-ci-path-routing.py` still
   accepts the workflow (probe: ACCEPTED). The same is true of an `exit 0` line put before the
   owner command. That gap already existed for every block-form #1044 owner (lint's four owners
   at `:552-555` have it too), so this attempt did not cause it. The spec does not ask for a pin.
   If root wants the guard to last, a follow-up could put the step's emptiness check into the
   workflow contract checker.

## Points judged

- **Step body (`qualification.yml:348-361`).**
  - It makes a fresh `mktemp -d "$RUNNER_TEMP/test-web-audioworklet-tmp.XXXXXX"` directory.
  - It sets the EXIT trap only after `mktemp` succeeds. If `mktemp` fails, `-e` ends the step
     before the trap exists, so `rm` never gets an empty path.
  - It exports `TMPDIR`. The script is on a line of its own.
  - `find -mindepth 1 -maxdepth 1 -printf '%f\n'` names every top-level entry, sorted.
  - The trap removes the directory last.
- **The step never masks a script failure (D1).** Under `bash -e`:
  - "leak then exit 3": step exit 3.
  - "exit 7": step exit 7.
  - `RUNNER_TEMP` was empty after both.
  - The trap cannot turn a failure into success: a successful `rm` keeps the status, and a failed
    `rm` under `-e` sets it to 1.
- **Hidden files.** A lone `.hidden` file made the step exit 1 and named `.hidden`. These also went
  red: a name with a newline, and a dangling symlink.
- **D2.** Gate 1 included a cold debug build of `parameter-metadata` (`target/debug` was created
  during gate 1), plus node and the python checkers, all with `TMPDIR` set to the scratch
  directory. Nothing was left there. A snapshot of `/tmp` before and after the run showed no new
  entries outside `TMPDIR`. No exemption list.
- **D3.** `unconditional_step_commands` still finds `bash scripts/test-web-audioworklet.sh`
  exactly. The checker is unchanged.
  - The "owner made conditional" mutation in the self-test (`test-ci-path-routing.py:757-762`) is
    the same mutation as before: `if: needs.route.outputs.math_closure == 'true'` goes in the same
    place, between the step's `name:` and `run:`. Its anchor occurs exactly once.
  - The checker refuses the mutation with the #1044 owner message ("artifact-gates must run
    `bash scripts/test-web-audioworklet.sh` unconditionally ..."), not for some other reason.
  - The "replaced by `true`" mutation now changes the block line and gets the same refusal.
  - The old self-test fails against the new YAML with "mutation anchor absent". So the change to
    the self-test was needed, and it is not weaker than before.
  - I also probed two ways around the owner check: `if:` placed after the block, and
    `|| true` added to the command. The checker refused both with the owner message.
- **D4.** `git diff --stat` shows three files: the spec, `qualification.yml` (this step and its
  comment only) and `test-ci-path-routing.py`. There is no change to the router or the verdict
  table, the step has no `if:`, and it stays in `artifact-gates` on the full route.

## Test value

The step's emptiness check is red when `scripts/test-web-audioworklet.sh`, or any process it starts
that honours `TMPDIR`, leaves any entry (dotfiles included) in its temporary directory. Examples are
a second `EXIT` trap that replaces #1421's one cleanup, or the revert of #1421. No other gate sees
this, because the runner's shared temporary directory hides what is left. Both examples went red
below.

The self-test edit adds no new test. It re-anchors the existing "owner made conditional" mutation
on the block form, and the checker still refuses it for the #1044 owner reason.

## Gates run (from the export, CI shell `bash -e` unless noted)

- Artifact build: `scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts` passed (shipped module `46c8b035...`, named twin `57855cd5...`).
- **Gate 1:** the step body was extracted from the YAML with PyYAML and run with `RUNNER_TEMP` set
  to a scratch directory. Exit 0, `RUNNER_TEMP` empty afterwards, no new entries in `/tmp`.
- **Gate 2:** `scripts/test-web-audioworklet.sh` from `26208ceaa^` gave exit 1, with
  "left these entries in its TMPDIR: miso-engine-host.bB9ZcF". `RUNNER_TEMP` empty afterwards.
  Reverted, and `cmp` matches `b7da37f15`.
- **Gate 3:** `trap 'true' EXIT` after `:13` gave exit 1, naming `miso-engine-host.yTqxui` and
  `miso-engine-mutation.bcUqgN`. `RUNNER_TEMP` empty afterwards. Reverted.
- **Variants**, under both `bash -e` and `bash --noprofile --norc -eo pipefail`:

  | Variant | Exit code |
  |---|---|
  | dotfile only | 1 |
  | leak then exit 3 | 3 |
  | exit 7 | 7 |
  | name with a newline | 1 |
  | dangling symlink | 1 |
  | clean exit | 0 |
  | unreadable TMPDIR | 1 (through the trap's `rm`; see MINOR 1) |

- **Gate 4:**
  - `python3 -B scripts/check-ci-path-routing.py`: passed.
  - `python3 -B scripts/test-ci-path-routing.py`: passed.
  - `bash scripts/check-workspace-policy.sh`: ok.
  - `bash -n` on the step body: ok.
  - Not run: the CI half of gate 4 (the batch PR's `artifact-gates` run). It is still pending, as
    the spec allows.
- Not available locally: shellcheck and actionlint.
