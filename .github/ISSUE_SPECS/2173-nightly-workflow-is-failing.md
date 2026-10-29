# nightly workflow is failing

GitHub #2173. Opened by the scheduled nightly workflow (`.github/workflows/nightly.yml`), which
opens one issue for a failing night and adds each further failing night as a comment while it is
open.

## Observed

| Run | Failing job | Every other job |
| --- | --- | --- |
| [40476993216](https://github.com/misofm/engine/actions/runs/40476993216) | release-mode wall-clock budgets | success |
| [40665535237](https://github.com/misofm/engine/actions/runs/40665535237) | release-mode wall-clock budgets | success |

The other jobs (native production-kernel release-probe report, deep fuzzing over every target,
workspace release link proof, math exhaustive sweeps, stem-store mutation ledger and script-gate
self-tests, full-size debug test sweeps) passed both nights.

## Contract

Find why the release-mode wall-clock budgets job fails, and fix the cause: a budget that a real
regression broke is a regression to fix; a budget that the runner cannot hold is recorded with its
measurement before it changes. No budget is loosened without that measurement.

## Close

Close this issue once the failure is dealt with. The next failing night after that opens a fresh
issue.
