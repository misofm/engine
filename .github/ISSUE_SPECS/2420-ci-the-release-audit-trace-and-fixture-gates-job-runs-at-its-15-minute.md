# CI: the release audit, trace and fixture gates job runs at its 15-minute limit

The `qualification.yml` job "release audit, trace, and fixture gates" (`timeout-minutes: 15`) is cancelled at its limit on ordinary changes, with every step green.

## Evidence
- PR #2418 (#2412): the job was cancelled at 15:00 on two consecutive runs. On run 42519168162 (job 128854548745), every step had succeeded before the job was cancelled at 15:01.
- On `main`, the four most recent runs took 9.5, 12.1, 14.6 and 11.4 minutes. That leaves less than half a minute of headroom on a warm cache, and none on a slower runner.
- The longest steps are "Audit and console-workload unit tests in release" (4-6.5 min) and the release build (2-3.5 min). Neither is slowed by #2418's change, which is a refactor with no new tests in those crates.

## Contract
- Raise this job's `timeout-minutes` from 15 to 20. That is about 35% above the slowest recent `main` run, matching the precedent of #1728 (test-release, 23 min) and #1972 (DSP debug).
- Change no step, and no other job's limit.
- `scripts/check-ci-path-routing.py` requires every leaf job to carry `timeout-minutes`, and that stays true. No mutation anchor in `scripts/test-ci-path-routing.py` uses this job's limit text.

## Gates
- `python3 scripts/test-ci-path-routing.py` and `python3 scripts/check-ci-path-routing.py` pass.
- The job completes under the new limit on this PR's run.

## Non-goals
- Speeding the job up. If it keeps growing, splitting it belongs in a separate issue.

## Evidence / attempt record
(empty)
