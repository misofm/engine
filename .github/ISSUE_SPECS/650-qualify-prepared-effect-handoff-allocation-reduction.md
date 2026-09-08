# Qualify prepared-effect handoff allocation reduction

GitHub: https://github.com/misofm/engine/issues/650

Parent: #560 CP1. Product predecessor: #633. Qualification/delivery predecessors: #642 and #644. Coordination: #559. Candidate main: `4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`. Original product baseline: `d98646db47bc603c32431d999cd08f43a0168043`.

PR #648 delivered only CP1’s prepared-effect handoff slice: `graph-compiler` now carries compiler-private index-aligned prepared-effect identity instead of repeatedly owning `(track, rack, effect)` strings. #633 explicitly deferred transient allocation measurement because retained `GraphResourceEstimate` and render allocator gates do not measure compilation. This issue supplies that promised qualification before any further CP1 schedule/PDC/front-half identity work. The audit’s historical 15–20k allocation estimate is neither assumed nor an acceptance value.

Sol HIGH coordinates the stateless record, Git, GitHub, checkpoints and delivery. Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole implementation and measurement executor. Astra LOW reviews scope, harness/validator, measurement evidence, exact heads, CI and delivery. Lane-A #649 owns only soft-clip compiler-lowering evidence and is path-disjoint. #649 and this issue are the two active issue slots.

## Smallest closable qualification slice

Add one narrow durable audit subject at `tools/audit/src/prepared_effect_allocations.rs`, register only that subject in `tools/audit/src/main.rs`, and add one strict comparison validator at `scripts/check-prepared-effect-allocation-records.py`. Reuse the already installed `bench_support::alloc` allocator and its `current_thread_counters`/`current_thread_delta_since` authority. No allocator, dependency, Cargo manifest/lock, production compiler/session/graph/effect source, generic benchmark framework, workflow, artifact, pin, browser, SDK, PCM/resource expectation or timing change belongs here.

The audit subject accepts only `--variant candidate|counterfactual`, writes canonical JSONL to stdout, diagnostics to stderr, and refuses unknown/missing/duplicate arguments before building a corpus or entering a counted interval. It must call `bench_support::alloc::assert_installed()` and execute a deterministic known-allocation positive control outside every measured interval.

Freeze these three corpus/control shapes in the harness before measurement:

1. `zero64`: 64 tracks with no prepared effects, matching graph topology/caps across variants; this is the no-handoff causal control.
2. `crossed-small`: a small accepted graph with repeated effect IDs across different tracks and racks, reversed prepared-entry order, distinguishable slot programs, and an unmeasured invalid twin whose exact ordered diagnostic identity is recorded.
3. `banks64`: 64 tracks with a fixed multi-slot native-effect chain, homogeneous bank cohorts and explicitly fixed heterogeneous members, with deterministic session/prepared-entry order.

Prepare session models, effect sessions, registries, request inputs and record buffers outside measurement. A round measures only the same `GraphCompiler::compile` call using current-thread allocator deltas. Read the ending counters before computing evidence or dropping the returned plan; plan destruction is outside the interval. Each corpus receives exactly one warmup followed by two measured rounds. No timing is taken. Both measured rounds must have identical allocation, deallocation, reallocation and cumulative requested-byte counters. Requested bytes are not peak or live memory. Canonical graph SHA/semantic plan identities and exact invalid diagnostic identities are computed outside measurement and must match between variants.

The JSONL schema fixes variant, corpus, round, graph/diagnostic identity, allocation calls, deallocation calls, reallocation calls and requested bytes. The validator requires the exact schema/key order, the three corpora, exactly two measured rounds, within-variant determinism, cross-variant semantic/diagnostic equality, and no additional records. Its synthetic self-test must prove rejection of wrong schema/key order, missing/duplicate corpus or round, changed semantic/diagnostic identity, unstable counters, a changed `zero64` control, non-reduction, and record/trailing-data corruption without launching the compile workload.

## Matched counterfactual

The measurement comparator is not a broad historical checkout. After Astra passes the frozen harness and validator, create a detached scratch worktree from the exact pushed harness checkpoint, then restore only these files from original baseline `d98646db47bc603c32431d999cd08f43a0168043`:

- `crates/graph-compiler/src/compile.rs`;
- `crates/graph-compiler/src/ids.rs`;
- `crates/graph-compiler/src/banks.rs`.

Record and verify the three restored hashes against the baseline, require exactly those three scratch modifications, and prove every other source, dependency, toolchain, harness and validator byte matches the candidate. This counterfactual recreates the old prepared-effect handoff on current candidate inputs. Stop if it does not compile cleanly without another edit; do not repair or widen it. The delivered candidate remains an exact clean pushed descendant of main with no compiler source edit in this issue.

Predeclare fresh, absent non-symlink paths for the two worktrees’ Cargo targets and temporary records. Run one candidate invocation and one counterfactual invocation, each once, with one internal warmup and two internal measured rounds. Capture exact argv/environment/cwd/head, toolchain, separate complete stdout/stderr, numeric status and source/tree identities. Do not retry, reorder variants, tune, delete a target, or rerun after seeing counts. Full streams, targets and compiler/build outputs stay temporary under `/tmp` and never enter Git.

## Objective gates and decision rule

Attempt 1 implements only the three allowed harness/validator paths. Before any official count, run focused parser/record/unit tests and synthetic validator mutations, then the audit package debug test/build, strict affected Clippy, rustfmt, diff hygiene, workspace policy, bench policy and exact-path census once. Root checkpoints and pushes the coherent source before Astra LOW source review. No official candidate/counterfactual workload may run before that PASS.

After source PASS, the executor creates the matched counterfactual and runs the two official release invocations once. The validator then runs once over their preserved stdout. PASS requires:

- status 0 and exact record population for both variants;
- identical semantic plans and diagnostic identities across variants;
- exact two-round counter equality inside every variant/corpus;
- all four `zero64` counters equal across variants;
- strictly fewer candidate allocation calls and requested bytes for both prepared-effect corpora;
- a positive saved-allocation count for `crossed-small`, with `banks64` saving more calls than `crossed-small`.

Deallocation/reallocation counts are reported exactly but are not independently interpreted as live memory. The final record states observed counts and deltas without extrapolation, timing, percentage, peak-memory or 15–20k claims. Unstable counts, semantic drift, zero-control drift, missing reduction, a counterfactual build failure, or a harness/validator defect after one bounded correction stops the measurement and requires candid disposition; never weaken the gates or rerun for a favorable number.

Astra LOW must pass the measurement evidence and compact decision record before ordinary exact-head/current-main PR review. The durable harness and validator may then deliver through required `qualification`, guarded merge and post-main success. This issue closes only the allocation qualification for #648’s prepared-effect handoff. CP1 remains open for schedule, PDC, cycle, reduction and buffer identities.

## Astra LOW implementation-scope review — PASS

Astra passed exact clean branch/upstream head
`da0a8f6b2b95ecb5a2a30d8c414b5fcab690eb6c` against candidate main
`4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447` and synchronized tracker `c094ea14`.
GitHub #650 is open with matching number, title and body; the branch delta is only this spec. #649
remains disjoint.

Hypatia's attempt 1 owns only the three named harness/dispatcher/validator paths. The positive control
must exercise the installed allocator; only compilation may fall inside counter boundaries; corpus,
program and bank identities must be frozen before measurement. The validator must also reject
duplicate JSON keys, invalid numeric types or ranges, and trailing records. Unit and synthetic tests
must not launch an official counted corpus.

Run these pre-measurement gates once in order and stop on the first failure:

```text
cargo test --locked -p audit prepared_effect_allocations -- --test-threads=1
python3 -B scripts/check-prepared-effect-allocation-records.py --self-test
cargo test --locked -p audit
cargo build --locked -p audit
cargo clippy --locked -p audit --all-targets -- -D warnings
cargo fmt --all --check
git diff --check
bash scripts/check-workspace-policy.sh
bash scripts/check-bench-policy.sh
```

Root checkpoints and pushes the exact three-path source tranche before Astra source review. No
official count, counterfactual worktree, production/dependency edit, timing, compiler payload or
artifact work is authorized. Exact measurement commands, environment and fresh paths require a
pushed post-source-PASS amendment and separate Astra review.
