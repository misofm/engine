# Repair the operator scripts' repository root, and stop it breaking silently

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Added by the Sol verification (`../VERIFY-DEAD-CODE.md`, finding F2). No ruling is needed. This is
the repair half of the audit's "broken, not dead" section 5.1, which no draft owned on its own.

## Context

Reproduced on `a9414c0c` by running each script from a clean tree (no workload launched):

| script | root expression | observed failure |
|---|---|---|
| `scripts/operator/preflight-rack-benchmark.sh` | `$(dirname "$0")/..` | `bash: scripts/test-rack-benchmark.sh: No such file or directory`, exit 127 |
| `scripts/operator/preflight-wasm-console-benchmark.sh --mono3` | same | `bash: scripts/check-console-benchmark-fixture.sh: No such file or directory`, exit 1 |
| `scripts/operator/run-wasm-console-benchmark.sh --mono3` | same | sources `scripts/scripts/check-bench-preconditions.sh`, exit 1 |
| `scripts/operator/run-wasm-kernel-timing.sh` | same | sources `scripts/scripts/check-bench-preconditions.sh`, exit 1 |
| `scripts/operator/prepare-builtins-listening.sh INBOX OUT` | same | `bash: scripts/check-builtins-listening.sh: No such file or directory`, exit 1 |
| `scripts/operator/seal-web-audioworklet-browser-correctness.sh SEAL` | same | `scripts/scripts/build-web-audioworklet.sh: No such file or directory`, exit 127 |
| `scripts/operator/preflight-console-benchmark.sh` | `/../..` (fixed later) | works |

The move into `scripts/operator/` was `f0509c3f` (#319, 2026-09-01). `run-stem-store-browser-evals.cjs`
resolves `../..` correctly; `probe-opfs-move-v1.cjs` computes no root.

Nothing catches this: operator scripts are, by `scripts/operator/README.md`, not gates, and no
workflow runs them.

## Smallest closable slice

1. For every operator shell script that still exists when this lands (04b, 04c and R9 delete some),
   change the root to `$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)`. Change nothing else.
2. Add one static lint check with a mutation test (for example inside an existing operator or
   bench policy script): every `scripts/operator/*.sh` that computes a repository root resolves to
   a directory containing the workspace `Cargo.toml`. A mutation back to `/..` must go red.

## Objective gates

1. Each repaired script, run from a clean tree with arguments that stop before any workload, gets
   past the root and fails or succeeds on its own preconditions: for example the wasm console
   runner reaches its `refusing to overwrite` check for an existing arm. Record each output.
2. The new check and its mutation test pass in lint; `check-ci-path-routing.py` and
   `test-ci-path-routing.py` pass.
3. Build, console digests and the shipped artifact are unaffected: `git diff --stat` touches only
   `scripts/`.

## Dependencies

None. Land it before, or in the same batch as, 04b, 04c and R9. If a script is deleted first, drop
it from step 1.

## Standing rules for the implementer

- Launch no timed workload. Commit on `codex/<issue>-operator-script-roots`.

## Attempt 1 evidence

Implementer: Terra, attempt 1, 2026-09-28, branch `codex/1022-operator-script-roots` from
`d70956bf` (`codex/batch-slim-1`). Commit `0b83d0f5` carries the repair, the check and its
mutation tests. No timed workload was launched.

### Repair

* The six root lines now resolve `../..` from the script's own directory:
  `preflight-rack-benchmark.sh`, `preflight-wasm-console-benchmark.sh`,
  `prepare-builtins-listening.sh`, `run-wasm-console-benchmark.sh`, `run-wasm-kernel-timing.sh`
  and `seal-web-audioworklet-browser-correctness.sh`. The seal script keeps its `pwd -P` and the
  listening script keeps its quotes. `preflight-console-benchmark.sh` was already right and is
  unchanged. None of the scripts has been deleted yet by 04b, 04c or R9, so all six are repaired.
* **Deviation (one line).** `preflight-wasm-console-benchmark.sh` also hashed its runner at the
  pre-#319 path `scripts/run-wasm-console-benchmark.sh`, so it would have failed even with the root
  fixed. That line now reads `scripts/operator/run-wasm-console-benchmark.sh`. The path sits inside
  a `jq --arg` command substitution, so errexit and pipefail do not catch the failure. The preflight
  would have exited 0 with an empty hash:

  ```
  $ bash -c 'set -euo pipefail; jq -n -c --arg runner_sha256 "$(sha256sum scripts/run-wasm-console-benchmark.sh | awk "{print \$1}")" "{runner_sha256: \$runner_sha256}"'
  sha256sum: scripts/run-wasm-console-benchmark.sh: No such file or directory
  {"runner_sha256":""}          # exit 0
  ```

  After the fix, the same expression yields `5af27a49…`. No other operator script names a
  `scripts/` path that does not exist.

### The check

The check is a new rule at the end of `scripts/check-bench-policy.sh`. The lint job's "Bench harness
duplication policy and mutation tests" step already runs that script and
`scripts/test-bench-policy.sh`, so no workflow changed. Four of the seven scripts are benchmark
preflights or runners. The rule is:

* Every `scripts/operator/*.sh` must compute its root from its own location. Any non-comment line
  that applies `dirname` to `BASH_SOURCE`, `$0` or `${0}` must be spelled
  `$(cd "$(dirname "${BASH_SOURCE[0]}")<rel>" && pwd[ -P])`. A different spelling is refused as
  unrecognised rather than skipped.
* `<rel>` is resolved from the script's directory, and the result must be this repository's root
  (`pwd -P`). That root must hold a `Cargo.toml` whose `[workspace]` table makes it the workspace
  manifest.
* A script that computes no root fails, and so does an empty `scripts/operator/*.sh` population.
* The rule uses bash builtins only (`read`, `[[ =~ ]]`, `cd`, `pwd`), so no external tool's exit
  status can be lost. It launches nothing.

The success line now ends `, 7 operator scripts rooted at the workspace)`.

`test-bench-policy.sh` now copies `Cargo.toml` and `scripts/operator/*.sh` into each scratch case.
Every mutation first checks that it actually changed the file, and every case asserts its
diagnostic:

* The #319 defect (`/../..` back to `/..`) is reintroduced into each of the seven scripts in turn.
  A guard fails the suite if fewer than seven scripts are covered.
* `/../../..` (past the root), `""` (own directory) and `/../../no-such-directory` (unresolvable).
* An unrecognised spelling (`root="$(dirname "$0")/.."`) and a removed root line.
* A missing root `Cargo.toml`, a member (non-workspace) `Cargo.toml` and an empty population.

**Red on the real tree.** With `run-wasm-kernel-timing.sh` put back to `/..`, then restored:

```
bench policy failure: operator script does not resolve the repository root: scripts/operator/run-wasm-kernel-timing.sh:23 names 'scripts/operator/..', which is /home/bl/misofm/worktrees/engine-1022/scripts, not /home/bl/misofm/worktrees/engine-1022
exit 1
```

With all seven operator scripts stashed back to their `d70956bf` content:

```
bench policy failure: operator script does not resolve the repository root: scripts/operator/preflight-rack-benchmark.sh:5 names 'scripts/operator/..', which is /home/bl/misofm/worktrees/engine-1022/scripts, not /home/bl/misofm/worktrees/engine-1022
```

### Gate 1: each repaired script reaches its own preconditions

All runs were made from a clean tree at `0b83d0f5`, with the scratchpad as the working directory,
so the root came from the script's location and not from the caller's directory.

| script (arguments) | output | exit |
|---|---|---|
| `preflight-rack-benchmark.sh` | `Issue-038 artifact already exists: artifacts/issue038/rack-benchmark.raw.jsonl` | 1 |
| `preflight-wasm-console-benchmark.sh --mono3` | `wasm console preflight failure: phase-2 wasm artifact already exists: <root>/artifacts/mono3/wasm-console-benchmark.raw.jsonl` | 1 |
| `run-wasm-console-benchmark.sh --mono3` | `refusing to overwrite phase-2 wasm artifact: <root>/artifacts/mono3/wasm-console-benchmark.raw.jsonl` | 1 |
| `run-wasm-kernel-timing.sh` | `refusing to overwrite phase-0b artifact: <root>/artifacts/issue163-phase0/wasm-kernel-timing.raw.jsonl` | 1 |
| `prepare-builtins-listening.sh INBOX OUT` (dummy scratch inbox) | `check-builtins-listening.sh` passes, then `Issue-033 validation failure: record is not canonical sorted-key JSON/LF` / `listening preparation failure: source or provenance` | 1 |
| `seal-web-audioworklet-browser-correctness.sh SEAL` (`MISO_ENGINE_CHROMIUM_BINARY`/`MISO_ENGINE_CHROMEDRIVER_BINARY=/bin/false`) | `build-web-audioworklet.sh` builds (21.6 s); then `web AudioWorklet static/object checks passed`, `... independent raw-Wasm oracle passed` and `... browser fixture/runner static check passed`; `--seal` then stops at `CalledProcessError: ['/usr/bin/false', '--version']` (driver identity), before any browser launch | 1 |

With the old root, the rack preflight and both wasm console scripts looked for their artifacts
under `scripts/artifacts/`. They now find the repository's own `artifacts/`. Every wasm console arm
already holds artifacts, so neither wasm console script can go further without a new arm. Nothing
was written to the tree and no seal file was produced.

### Gates 2 and 3

* `bash scripts/check-bench-policy.sh` passes (`... 3 subjects on the shared timer, 7 operator
  scripts rooted at the workspace)`). `bash scripts/test-bench-policy.sh` passes (`bench policy
  mutations: ok`, 15 s).
* `python3 scripts/check-ci-path-routing.py`, `python3 scripts/test-ci-path-routing.py`,
  `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-env-vocabulary.sh` and
  `test-env-vocabulary.sh` all pass.
* `git diff --stat d70956bf 0b83d0f5` touches only `scripts/`: 8 files, +124/-9. This evidence
  section is the only other change.

### Observations, not acted on

* `run-wasm-kernel-timing.sh` is mode `100644`, while the other six operator scripts are
  executable. It runs via `bash`. This change did not touch it.
* `python3 scripts/test-ci-path-routing.py` leaves an untracked, unignored `scripts/__pycache__/`.
  That makes the clean-tree preconditions of the operator scripts refuse until it is removed. It was
  removed by hand before the gate 1 runs.
