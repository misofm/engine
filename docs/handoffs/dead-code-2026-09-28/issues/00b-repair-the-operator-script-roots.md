# Repair the operator scripts' repository root, and stop it breaking silently

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
