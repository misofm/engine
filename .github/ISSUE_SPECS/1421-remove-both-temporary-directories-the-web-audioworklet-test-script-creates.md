# Remove both temporary directories the web AudioWorklet test script creates

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). A defect on
`main` found by the stream-J batch verdict, "Notes (no action)", last item
(`/home/bl/misofm/submix-verdicts/stream-j-batch-verdict.md`). Test tooling only.

## Problem (verified on `main` at `0a1176b3b`)

`scripts/test-web-audioworklet.sh`, which the required workflow runs
(`.github/workflows/qualification.yml:349`), makes two temporary directories and sets an `EXIT`
trap for each:

- `:10` makes `host_test_dir` (`mktemp -d "${TMPDIR:-/tmp}/miso-engine-host.XXXXXX"`), copies
  `prepared-control.js` into it (`:16`), and sets `trap cleanup_safe EXIT` (`:17-18`), which
  removes it. This is in the `node` branch only; the `bun` branch (`:31-32`) makes no directory.
- `:29` removes the two transformed modules, but not the directory or `prepared-control.js`.
- `:42` makes `mutation_dir` (`mktemp -d`) and `:43-46` sets `trap cleanup EXIT`, which removes
  only `mutation_dir`.

A shell has one `EXIT` trap. `:46` replaces `:18`'s, so `host_test_dir` is never removed once the
script reaches `:46`. Every run leaves one `miso-engine-host.*` directory holding
`prepared-control.js`. This host has 117 of them in `/tmp` (counted at `0a1176b3b`, each holding
exactly that file).

## Decisions

- **D1. One cleanup, one trap.** Define one `cleanup` function before the first `mktemp` that
  removes both directories, each only if it was made (`${host_test_dir:-}`, `${mutation_dir:-}`;
  the script runs under `set -u`, and the `bun` branch never sets `host_test_dir`). Set
  `trap cleanup EXIT` once, before the first `mktemp`. Delete `cleanup_safe`, its trap and the
  second `trap`.
- **D2. The explicit removal at `:29` goes.** The trap removes the whole directory, so the
  per-file `rm` is redundant.
- **D3. Both directories honour `TMPDIR`.** `mutation_dir` uses
  `mktemp -d "${TMPDIR:-/tmp}/miso-engine-mutation.XXXXXX"`, as `host_test_dir` does, so a
  caller that sets `TMPDIR` holds every directory the script makes.

## Authorized paths

- `scripts/test-web-audioworklet.sh` (the two directories and their cleanup only)
- This spec

## Non-goals

- The directories other scripts make (`scripts/check-web-audioworklet.sh:148-149` has its own
  trap). Cleanup on signals other than the ones Bash already turns into an `EXIT`.
- Removing the 117 directories already left on any host.

## Hazards

- `set -euo pipefail` (`:2`): a cleanup that reads an unset variable aborts the trap. D1's
  `${name:-}` form is required, and `rm -rf -- ""` must not run (test the variable first).
- The red-path mutation at `:24-28` exits 1 on purpose; the trap must still remove both
  directories then.

## Objective gates

1. **Nothing is left behind (PR evidence).** `t=$(mktemp -d)`, then
   `TMPDIR=$t bash scripts/test-web-audioworklet.sh` exits 0, and `find "$t" -mindepth 1` prints
   nothing. On `main` the same run leaves one `miso-engine-host.*` directory (red on revert).
2. **Nothing is left behind on failure (PR evidence).** In a scratch copy of the script in
   `scripts/` (so that `repo_root` resolves the same), add `exit 1` right after `mutation_dir` is
   made. Run it with a fresh `TMPDIR`: it exits 1 and leaves nothing. Delete the copy.
3. If any directory under `$t` in gate 1 comes from another script, record its name and owner in
   the Evidence for root; it does not fail this issue.
4. `bash -n scripts/test-web-audioworklet.sh` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.* No committed test. Gates 1 and 2 are run once as PR evidence: a check of a temporary
directory's contents would need a new CI step in the required workflow, which this issue does not
touch.

## Evidence

- Gate 1 at `main` and at the PR head (the `find` output of each), and gate 2's run.

## Dependencies

- None.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
