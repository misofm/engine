# Split the AArch64 debug test leg into shards under its timeout

Filed by root on 2026-11-01. Blocks every merge: `qualification` is required on `main`.

## Problem
The required job "AArch64 product crate debug tests (NEON Simd4, FPCR)" (`.github/workflows/qualification.yml:1118`, `timeout-minutes: 30`, `bash scripts/run-aarch64-tests.sh debug`) has run about 28 minutes on every run since at least 2026-10-31. On 2026-11-01 it passed the limit and was cancelled on the `main` push at 10:12 (run after PR #2233) and on PR #2236. No test hangs: `crates/capi/tests/plan_swap_race.rs` alone takes 211 s in this debug leg and finishes green; the leg as a whole simply has no margin.

## Scope
Split the debug leg into two or more shards that run as separate jobs, each well under its timeout (target: under 20 minutes, measured on CI), with the same total coverage:
- every package, test target, feature set, skip and named expected-failure row of today's leg runs in exactly one shard;
- the no-silent-skip gate and the expected-failure rows (`scripts/run-aarch64-tests.sh`, lines 96-130) keep working per shard;
- the `verdict` job's static expectation table and any router outputs include the new jobs;
- the self-tests of the workflow and script policy (for example the test-support CI coverage check and its mutation test) are updated and pass.
Do not raise the timeout instead of splitting, and do not drop or filter any test.

## Gate
- One PR CI run where every shard passes, each under 20 minutes.
- A listing proves the union of the shards' test lists equals today's leg's list (one-time, PR evidence).
