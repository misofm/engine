PASS

# #1421 attempt 1 verdict (decision-15 stream J batch 2)

Commit under review: `26208ceaa` on `codex/d15-stream-j2` (parent `da878bd49`). Changed paths:
`scripts/test-web-audioworklet.sh` and the #1421 spec only. Both are authorized. The commit message ends
with the required `Co-Authored-By` line. The script at the parent is the same as the script on `main`
(`git diff main 26208ceaa^ -- scripts/test-web-audioworklet.sh` is empty).

## Requirements

- **D1 (one cleanup, one trap): met.** `scripts/test-web-audioworklet.sh:7-13`. Both variables start
  empty. `cleanup` tests each one with `${name:-}` before it calls `rm -rf --`, so `rm -rf -- ""` never
  runs. `trap cleanup EXIT` is set once, before the first `mktemp` (`:19`). `cleanup_safe`, its trap and
  the second trap are gone. A `bash -x` run shows the trap runs exactly once, at exit, and removes both
  directories (a command substitution does not run it).
- **D2 (the per-file `rm` goes): met.** The `rm -f -- "$safe_host" "$unchecked_host"` line is gone. The
  gate-1 output at the parent and at the commit is the same, line for line, apart from paths (157 lines
  each), so no test in the script changed its behaviour.
- **D3 (both directories honour `TMPDIR`): met.** `:48` is
  `mktemp -d "${TMPDIR:-/tmp}/miso-engine-mutation.XXXXXX"`. A `bash -x` run shows both directories made
  under the caller's `TMPDIR`.
- **Hazards: met.** `set -u`: the variables are always set, and the cleanup reads them with `:-`.
  The red path: when the unchecked-allocator mutation escapes, the script exits 1 at `:34` and leaves
  nothing (variant F below).

## Gates (re-run from an export with a fresh `TMPDIR` each time)

The script needs no built worklet artifact (`test-web-audioworklet.mjs` uses fakes). It builds
`parameter-metadata` through `cargo run`, so the runs used
`CARGO_TARGET_DIR=/tmp/claude-1002/v1421/target`.

1. **Gate 1.** Commit: exit 0, `find "$t" -mindepth 1` printed nothing. Parent (red on revert): exit 0,
   `find` printed `miso-engine-host.fmTqT7` and `miso-engine-host.fmTqT7/prepared-control.js`.
2. **Gate 2.** A copy with `exit 1` after the `mutation_dir` line, fresh `TMPDIR`: exit 1, nothing left.
3. **Gate 3.** No directory from another script appeared under `$t`. A before-and-after listing of
   `/tmp/miso-engine-*` and `/tmp/tmp.*` showed no new directory outside `TMPDIR`.
4. **Gate 4.** `bash -n` exit 0. `scripts/check-workspace-policy.sh` exit 0 (`workspace policy: ok`).

Extra failure paths (scratch copies in the export's `scripts/`; commit result first, then the parent):

| Variant | Commit | Parent |
|---|---|---|
| A `exit 1` after `mutation_dir` mktemp | exit 1, nothing left | leaves the `tmp.*` dir |
| B `exit 1` after `host_test_dir` mktemp | exit 1, nothing left | leaves `miso-engine-host.*` |
| C `false` (set -e) after `host_test_dir` mktemp | exit 1, nothing left | leaves `miso-engine-host.*` |
| D `false` (set -e) after `mutation_dir` mktemp | exit 1, nothing left | leaves the `tmp.*` dir |
| E unbound variable (set -u) after `mutation_dir` mktemp | exit 1, nothing left | leaves the `tmp.*` dir |
| F the red-path mutation escapes (exit 1 at `:34`) | exit 1, nothing left | nothing left |
| G the `bun` branch forced (`host_test_dir` never set) | exit 0, nothing left | nothing left |
| H the `bun` branch plus `exit 1` after `mutation_dir` | exit 1, nothing left | leaves the `tmp.*` dir |

The exit status stays as it was in every case (0, 1, and 2 for "no runtime").

**Caller environment cannot steer the trap.** Exported `host_test_dir` and `mutation_dir` named two
directories that had files in them. These four runs left both directories in place: no runtime on
`PATH` (exit 2, before any mktemp), `exit 1` right after the `trap` line, the full normal run, and the
`bun` branch exiting before the second mktemp.

**Signals** (bash 5.2.21, script blocked in a `sleep` after both mktemps). TERM, INT and HUP each went
to the bash pid and, in separate runs, to the process group. The commit removed both directories every
time. The exit codes were 143, 130 and 129. INT sent only to the pid lets the script finish with exit
0, which is how bash handles SIGINT while it waits for a child. The parent left `miso-engine-host.*`
in all six cases. The spec makes signals that bash does not turn into `EXIT` a non-goal. Bash turns
all three of these into `EXIT`, and the commit covers them.

`set -e` with the trap: the trap's tests are `if` conditions, so `set -e` does not stop the trap on an
empty variable. See NIT 1 for the case where `rm` itself fails.

## Findings

No BLOCKER, MAJOR or MINOR finding.

- **NIT 1 (no action needed)**, `scripts/test-web-audioworklet.sh:10-11`. Under `set -e`, if the first
  `rm -rf` fails, the trap stops. The second directory then stays, and the exit status becomes 1. A
  probe with a read-only subdirectory in `host_test_dir` showed this. The script never makes content
  that `rm -rf` cannot remove, so this cannot happen in practice, and the code that was there before
  had the same behaviour.
- **NIT 2 (outside this issue's paths)**,
  `.github/ISSUE_SPECS/1417-refuse-a-c-allocator-name-anywhere-in-an-unmangled-worklet-symbol.md:42,79`.
  These lines cite `scripts/test-web-audioworklet.sh:41` for the call-graph self-test. After this commit
  that line is `:47`. The spec records the state at its own commit, so this is a note for whoever
  edits #1417 next.
- **Note for root (not a finding against this attempt).** The spec decides on no committed guard. If
  someone later adds a second `EXIT` trap, the leak can come back with no CI signal. A guard would be
  cheap: run the `qualification.yml` step with a fresh `TMPDIR` and require that it is empty
  afterwards. That is a change to the required workflow, which this spec excludes. Root decides whether
  a follow-up issue is worth it.

## Test value

This change adds or rewrites no test. The spec says gates 1 and 2 are PR evidence only. That evidence is
adequate: gate 1 was reproduced red at the parent and green at the commit, gate 2 was reproduced, and
the extra variants, signal runs and caller-environment runs above show that the one trap covers every
exit path after each mktemp. The script's own test output is the same at the parent and the commit.
The residual risk is the missing regression guard in the note for root above.

## Gates run

`bash -n scripts/test-web-audioworklet.sh`; `bash scripts/check-workspace-policy.sh`;
`TMPDIR=$t bash scripts/test-web-audioworklet.sh` at the commit, at the parent, and under `bash -x`;
variants A-H at the commit and at the parent; four caller-environment runs; 12 signal runs (commit and
parent × TERM/INT/HUP × pid/process group). No CI file changed, so `check-ci-path-routing.py` did not
apply. No engine or browser-compiled code changed, so the cross-target check and the worklet build
chain did not apply.
