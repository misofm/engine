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

## Decision record (attempt 1)
- **Shape.** `aarch64-debug` stays one job ID and becomes a matrix, `shard: [1, 2, 3]`, `fail-fast: false`, each running `bash scripts/run-aarch64-tests.sh debug <shard>/3`. A matrix job's `needs.<id>.result` is `success` only when every shard succeeds, so the verdict's static table, its `needs` list and the router outputs keep the one `aarch64-debug` row and need no change. Each shard has its own cache key, `aarch64-debug-<shard>`. The timeout stays 30 minutes.
- **Partition.** `scripts/run-aarch64-tests.sh` names shards 1 and 2. Shard 3 runs every other package of the leg, so the shards cover the leg by construction, and a crate that joins capi's closure runs in shard 3 from the day it joins. The script refuses a named crate that is not in the leg and a crate that two shards name.
  - Shard 1: `capi control-plane host-core`, plus the uncaptured `resource_lifecycle` budget run (capi's test).
  - Shard 2: `builtins builtins-compiler effect-compiler graph graph-compiler protocol session`, plus the four `host-web --lib --exact` pins.
  - Shard 3: the other 25 crates of the leg.
  - Each shard runs `--all-targets`, `--doc` and its expected-failure rows over its own packages.
- **Features.** A smaller `-p` set changes Cargo's feature unification: in the whole leg, capi's dev-dependencies turn on `engine/test-support`, `session/test-support`, `lane/test-support` and others for every crate that links them. A shard therefore derives its `--features` from the whole leg's resolved features (`cargo tree --format '{p}|{f}'`), for every workspace crate that cargo accepts there (selected, or a direct dependency of a selected crate). Then it requires every workspace crate in its own build to resolve exactly the leg's features, and fails if one does not. `protocol` is in shard 2 for this reason: in shard 2 it was transitive only and lost `test-support`, which the gate refused.
- **Expected-failure rows.** Each shard judges and runs only the rows whose package it holds (`aarch64-known-defects.py judge-skips <mode> <list> [package ...]`). A row whose package is not in the leg fails every shard. The debug table has no rows today. A fake `builtins` row was judged only in shard 2 and in the whole leg; shards 1 and 3 did not judge it.
- **Policy.** `check-ci-path-routing.py` reads `debug_shard_count=3` from the script and requires the matrix to be exactly `shard: [1, 2, 3]` and the run line `debug ${{ matrix.shard }}/3`. `test-ci-path-routing.py` adds three mutations (a shard dropped from the matrix, every job pinned to shard 1, the script's count changed), and all fail the checker.

## Evidence (attempt 1)
- **Balance** (test wall time per crate from the cancelled debug run of PR #2236, 25.5 minutes of tests after a 3.5-minute compile): shard 1 508 s, shard 2 511 s, shard 3 514 s. Expected job time per shard: about 12 to 13 minutes (setup 0.5, compile 2 to 3, tests 8.5, doctests and pins under 0.5), plus about 1 minute on the first run, which has no per-shard cache yet. To be measured on the PR's CI run.
- **Coverage, x86 listing.** `cargo test <exact args of each run> -- --list` on this x86 host, natively: the whole leg lists 3,238 tests in 308 binaries; the three shards list 943 + 933 + 1,362 = 3,238. As a multiset keyed by binary and test name, the union equals the whole leg: 0 missing, 0 extra, 0 duplicated. Doctests: 32 in the whole leg, 9 + 12 + 11 in the shards, union equal, each once.
- **Coverage, AArch64 unit graph.** `cargo +nightly-2026-10-11 test <the same args> --target aarch64-unknown-linux-gnu --unit-graph -Zunstable-options` (no linker or arm64 host is needed): the whole leg has 407 test roots and 35 doctest roots; the shards' union has the same 407 and 35, with no root in two shards, and every workspace crate in each root's closure has the same features. The only differences are additive features of external crates that the whole leg's tooling dev-dependencies turn on (`smallvec/const_generics`, `num-traits`/`num-complex` `std`, `regex-automata` features); they add API, and every shard compiles without them.
- **Local checks.** `bash -n scripts/run-aarch64-tests.sh`; a fake `cargo test` (real `cargo tree`/`metadata`, `CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu`) ran each shard, the whole leg and the refusals (`debug 4/3`, `debug 0/3`, `debug 1/2`, `release 1/3`, a crate named twice, a crate outside the leg, `protocol` moved out of shard 2); `check-ci-path-routing.py` and `test-ci-path-routing.py`; `check-test-support-ci.py` and `test-test-support-ci.py`; `check-script-reachability.py` and its test; the artifact-evidence leak gate and its test; `test-npm-publish-modes.py`; `aarch64-known-defects.py --self-test`; `check-workspace-policy.sh`. `actionlint` and `shellcheck` are not installed here.
