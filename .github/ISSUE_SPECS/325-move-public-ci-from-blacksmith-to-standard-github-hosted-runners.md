# Move public CI from Blacksmith to standard GitHub-hosted runners

**Status: SOL PASS; REMOTE CI PENDING.**

## Owner ruling

The repository is public, so standard GitHub-hosted runners execute its workflows without billed
Actions minutes. Retire the paid Blacksmith runner dependency and return every workflow job to the
standard x64 Ubuntu 24.04 GitHub-hosted image while retaining ephemeral isolation for public pull
requests.

## Smallest closable product slice

Replace every Blacksmith runner label under `.github/workflows/` with `ubuntu-24.04`. Change no
trigger, step, permission, matrix, timeout, artifact, or job dependency.

## Decision record

1. Public pull-request code remains on ephemeral GitHub-managed compute rather than devbox.
2. Existing `ubuntu-24.04` jobs remain unchanged.
3. The migration accepts lower per-job CPU allocation in exchange for eliminating third-party
   runner billing; performance is measured by the resulting CI run rather than guessed locally.
4. A custom or self-hosted runner is not introduced by this issue.

## Objective gates

- No `blacksmith` spelling remains under `.github/workflows/`.
- Every workflow document parses as YAML.
- The diff changes runner labels and this issue record only.
- One focused pull request reports the real hosted-runner result.

## Workflow

Sol briefs and approves this stateless scope. Implementation performs the mechanical label-only
migration. Sol reviews the exact workflow diff and local structural gates before opening the pull
request.

## Evidence record

- GitHub issue #325 matches this stateless local spec.
- Implementation changes all Blacksmith 4-vCPU and 8-vCPU Ubuntu 24.04 labels to the standard
  `ubuntu-24.04` GitHub-hosted label without changing job behavior.
- Every workflow parses as YAML, `git diff --check` passes, and a case-insensitive audit finds no
  `blacksmith` spelling under `.github/workflows/`.
- Sol reviewed the exact workflow diff: ten runner-label substitutions and no trigger, permission,
  step, matrix, timeout, artifact, or dependency change. Local review passes; the pull request is
  the required real-runner compatibility and performance evidence.
