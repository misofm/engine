# Give the workspace debug test job a real time margin

The required `qualification` job "workspace debug tests (minus DSP crates and evidence tools)" has `timeout-minutes: 15` (`.github/workflows/qualification.yml:609`). On recent runs it took 14.5 to 14.7 minutes (PR #1725: 14:41; batch 20, PR #1721: 14:36), and on PR #1727 it hit the timeout twice while its tests were still passing. One rerun of the failed job then passed in 10.5 minutes. The job runs too close to its limit, so a cold cache or a slow runner turns a green PR red.

## Decision to make

- Cut the job's time (the 2026-10 test audit removes redundant tests; the 5 `tools/audit` `fixture_builtins` tests use about 14% of all test-binary time and are in this job's scope only if not excluded), or
- raise the timeout to a measured margin (for example 1.5 times the slowest recent green run), or both.

## Gate

Ten consecutive green runs of the job with at least 25% margin under its timeout.
