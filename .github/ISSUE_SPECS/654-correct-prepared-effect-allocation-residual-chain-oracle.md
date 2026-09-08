# Correct the prepared-effect allocation residual-chain oracle

GitHub: https://github.com/misofm/engine/issues/654

Parent: #560 CP1. Failed predecessors: #650 and #652. Product predecessor: #633. Coordination: #559. Candidate harness checkpoint: `7add1d74`. Product main: `4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`.

#652's final source attempt stopped at focused gate 1 because one test derived residual scalar members from numeric tracks 56–63, while the planner deterministically groups lexicographically ordered `RackChainId`s. Four other focused tests passed, but no official audit variant ran and no allocation claim received credit. This successor corrects only the residual/bound-membership oracle and then completes source qualification before any measurement.

## Smallest closable slice

Inherit the pushed #652 harness. Implementation may change only `tools/audit/src/prepared_effect_allocations.rs`; `tools/audit/src/main.rs` and `scripts/check-prepared-effect-allocation-records.py` are frozen unless Astra identifies an existing source-qualification blocker before implementation. No production compiler/session/graph/effect, allocator, dependency, manifest/lock, workflow, generic benchmark, timing, artifact, browser, SDK, PCM, or compiler-payload path is owned.

Construct the expected normal `RackChainId` set independently from frozen fixture declarations, sort it using the type's contract order, take seven complete Simd8 groups, and treat the remaining normal chains plus the exact bypassed `track-63`/SIMD1 chain as the scalar set. Expand that exact scalar chain set into its `slot0` and `slot1` `EffectNodeId`s. Build expected bound associations from the seven complete groups and both slots. Assertions must compare exact sets and associations without copying report output into expectations or assuming numeric track order/group indices.

Preserve the passing cross-rack repeated-ID/full-reverse/program-key test, the measured allocator positive control, prepare-before-mark compile-only interval, counter read before identity/drop, identity from each measured returned artifact, one warmup/two measured rounds, strict JSONL validator, and no-timing contract. Unit tests must not invoke the official audit loop.

## Source attempt and gates

After Astra LOW scope PASS, one Luna HIGH/XHIGH executor gets one coherent implementation attempt. Use a fresh absent temporary evidence directory and retain exact command, cwd, head, toolchain, stdout, stderr, and numeric status for every invoked gate. Run once in order and stop at the first failure without correction or rerun:

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

Root checkpoints and pushes before Astra LOW source review. No audit-subject invocation, official count, or counterfactual is allowed before source PASS.

After source PASS, amend this spec with reviewed exact release commands, environment, heads, hashes, fresh paths, and the matched counterfactual inherited from #650: a detached worktree at the pushed harness checkpoint restoring only `crates/graph-compiler/src/compile.rs`, `ids.rs`, and `banks.rs` from `d98646db47bc603c32431d999cd08f43a0168043`. Stop if those three files do not compile without edits.

Each variant runs exactly once with one internal warmup and two measured rounds. PASS requires statuses 0, exact record population and two-round equality, cross-variant graph/diagnostic identity, all four zero64 counters equal, fewer candidate allocation calls and requested bytes for both prepared corpora, positive crossed-small savings, and more saved calls in banks64 than crossed-small. Report exact counts/deltas only. Full streams remain temporary through Astra review; commit no `.ll`, assembly, compiler output, raw record, target, object, archive, or binary.

Astra LOW passes measurement evidence and the compact decision record before PR review, required CI, guarded merge, post-main qualification, GitHub synchronization, closure, and clean delivered-worktree removal. This issue can qualify only #648's prepared-effect handoff allocation reduction; CP1's schedule, PDC, cycle, reduction, and buffer identities remain open.

## Astra LOW initial scope review — FAIL; bounded amendment

Astra reviewed exact clean branch/upstream `265c7afb576667bef57a9fcc69e45161f96c8827`,
live main `4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`, inherited harness
`7add1d74`, and synchronized tracker `3037aaa0`. GitHub #654 matches; #652 is closed,
and #653/#654 are disjoint.

The residual/bound-node correction is sound and minimal, but the inherited Python
validator accepts integers above the Rust emitter's `u64` domain. This amendment
adds `scripts/check-prepared-effect-allocation-records.py` to attempt 1 only for
that range correction and synthetic boundary controls: counter fields accept 0 and
`18446744073709551615`; they reject -1, booleans, and
`18446744073709551616`. All other validator behavior remains frozen. No
implementation or measurement is authorized until Astra passes this amended scope.

## Astra LOW amended-scope review — PASS

Astra passed exact clean branch/upstream
`c7061b3018b4bb0c3946d684770b759cdbd53065`; GitHub #654 is open and matches.
One Luna HIGH/XHIGH implementation pass may change only the independently sorted
residual/bound-membership oracle and the validator's stated `u64` range check and
boundary mutations. Run the prescribed gates once in order with retained evidence
and stop on the first failure without correction or rerun. Root checkpoints and
pushes before fresh Astra source review. No official measurement or counterfactual
execution is authorized.

## Attempt 1 source gates — FAIL; attempt 2 bounded

Luna HIGH changed only the audit subject and validator at exact authorization
`ad43fbbc`. Retained source evidence records focused tests 5/5 PASS, validator
self-test 14 mutations PASS, full audit tests 41 PASS, and audit build PASS with an
unused-import warning. Strict Clippy then returned 101 for the test-only
`PreparedEffectQuality` import, and the executor stopped. Gates 6 through 9 and all
audit-subject, counterfactual and measurement work remained unrun.

Astra LOW returned attempt 1 **FAIL** while confirming that the residual oracle now
derives the lexicographic normal tail plus fixed bypassed chain independently of
the report, including independent bound associations. Attempt 2 may only move or
conditionally compile the test-only import and add successful boundary controls
that prove `u64::MAX` is accepted using otherwise valid, internally consistent
records. Preserve overflow, boolean and negative rejection and all other behavior.

Because Rust behavior and build already passed and the import-only correction does
not change it, attempt 2 runs the amended validator self-test, strict Clippy,
rustfmt, diff, workspace policy and bench policy once in that order. Stop on the
first failure without correction or retry. No official measurement or
counterfactual is authorized before a pushed checkpoint and fresh Astra PASS.

## Attempt 2 formatting gate — FAIL; final attempt 3 bounded

At exact checkpoint `1987417b`, Luna HIGH moved the test-only import and added
internally consistent zero/`u64::MAX` success controls. The validator self-test and
strict Clippy passed. `cargo fmt --all --check` then returned 1 for import ordering
and one assertion layout, and execution stopped. Diff and policy gates, the audit
subject, counterfactual and measurements did not run.

Astra LOW returned attempt 2 **FAIL** and confirmed the validator boundary records
are valid across both variants and rounds. Those validator and Clippy results carry
forward. Final attempt 3 may apply only rustfmt's exact reported changes and then
run `cargo fmt --all --check`, `git diff --check`, workspace policy and bench policy
once in that order with fresh retained evidence. It must not rerun tests, build,
validator or Clippy, make a semantic change, or execute an official variant. Any
failure exhausts #654 with no fourth correction.

## Final source qualification — PASS

Final attempt 3 changed only rustfmt's exact previously reported import ordering
and assertion layout. At pushed clean branch/upstream
`2c20a8af7fa0eed9eb687e37cb009492c295f0a4`, Astra LOW verified the retained
three-attempt chain: focused tests 5/5, full audit 41, build, strict validator
boundaries/mutations and Clippy passed; final fmt, diff, workspace and bench-policy
statuses are zero. Attempts 1/2 remain FAIL. The source and validator are now
frozen. No audit variant, counterfactual, or official count has run.

## Frozen measurement amendment

Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole measurement executor.
Astra LOW must pass this amendment before any path below is created. The frozen
measurement-source commit is exactly
`785838403d02e383ba12553a3464d492e60a5cd5`; its only difference from source-PASS
commit `2c20a8af...` is this issue spec. Both measured worktrees are detached from
that same commit, so the amendment's later documentation-only commit cannot change
the measured code. The coordinator worktree remains separate.

```text
/tmp/issue654-candidate-source
785838403d02e383ba12553a3464d492e60a5cd5
```

Require all five paths absent, including dangling symlinks, before creating any of
them:

```text
/tmp/issue654-measurement-evidence
/tmp/issue654-candidate-source
/tmp/issue654-candidate-target
/tmp/issue654-counterfactual-target
/tmp/issue654-counterfactual-source
```

Create the evidence directory and record/read/hash exact coordinator cwd, its full
HEAD/upstream equality, the frozen measurement-source commit, empty coordinator
porcelain including untracked files, `rustc -Vv`, `cargo -V`, literal
commands/environment, absence of concurrent Cargo/rustc work, and all path-absence
results. Any producer, readback, hash, identity, cleanliness, concurrency, or
absence failure stops before worktree creation.

Create both source worktrees once with:

```text
git worktree add --detach /tmp/issue654-candidate-source 785838403d02e383ba12553a3464d492e60a5cd5
git worktree add --detach /tmp/issue654-counterfactual-source 785838403d02e383ba12553a3464d492e60a5cd5
git -C /tmp/issue654-counterfactual-source restore --source=d98646db47bc603c32431d999cd08f43a0168043 -- crates/graph-compiler/src/compile.rs crates/graph-compiler/src/ids.rs crates/graph-compiler/src/banks.rs
```

Require both detached HEADs equal `78583840...`, empty candidate porcelain, no
untracked files, and counterfactual porcelain containing exactly those three
modified paths. Their candidate and restored SHA-256 values are:

```text
path                                         candidate                                                         restored baseline
crates/graph-compiler/src/compile.rs          109949399f4da4fc44078eb3121dbc6939bc5f635ba34c7789883d7a377f6c53  ecfe271b944f63d921a1ec64d6e74512d4e259435bb02104997491122fcabb26
crates/graph-compiler/src/ids.rs              5c6ade8f0887bec4a0c39d4413929bb526cf1d822d7e66c6835961751d0ede58  08a5a2265f31061acbc33737438bf1d27c08c9f5c7010bca82825e61eaf1d115
crates/graph-compiler/src/banks.rs            981b0fa269401ef1f985e4eeffc4580b1bc66c2e0bd87f4333e8486444ef0f4b  997ccfda181b6ffa01472d380e1e4c777eaceaceeefb158f9ddbb58af9216475
```

Because the detached worktree starts at the exact candidate commit and its complete
porcelain is restricted to these paths, every other tracked source, harness,
validator, dependency and toolchain byte remains identical. Stop if the
counterfactual needs any additional edit.

Run exactly once from the respective detached source worktrees in
candidate-then-counterfactual order, with separate complete stdout, stderr and
numeric status records:

```text
(cd /tmp/issue654-candidate-source && LC_ALL=C LANG=C TZ=UTC CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/issue654-candidate-target cargo run --locked --release -p audit -- prepared-effect-allocations --variant candidate)
(cd /tmp/issue654-counterfactual-source && LC_ALL=C LANG=C TZ=UTC CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/issue654-counterfactual-target cargo run --locked --release -p audit -- prepared-effect-allocations --variant counterfactual)
```

Each command contains its single internal warmup and two measured rounds per corpus.
Stop on nonzero status, unexpected stdout population, identity change, tree change,
or concurrent Cargo/rustc use. Do not retry, reorder, clean a target, or inspect
counts before both commands finish. Concatenate candidate stdout followed by
counterfactual stdout once to
`/tmp/issue654-measurement-evidence/combined.jsonl`, then run exactly once from the
detached candidate-source worktree:

```text
(cd /tmp/issue654-candidate-source && python3 -B scripts/check-prepared-effect-allocation-records.py /tmp/issue654-measurement-evidence/combined.jsonl)
```

Preserve all temporary records, streams, targets and the counterfactual worktree
through Astra review. The compact decision record may contain only exact
source/toolchain/command/status/hashes, graph and diagnostic identities, per-corpus
counters/deltas, validator status and conclusion. No raw stream, JSONL, target,
`.ll`, assembly, object, archive, library, binary, or generated compiler output may
enter Git. No timing, percentage, historical-count comparison, extrapolation,
optimization, or source repair is authorized.

## Astra LOW measurement-scope review — PASS

Astra passed exact clean coordinator branch/upstream
`6c193061495a7ba3b990ef3d188a4538ef8a10d8` and synchronized GitHub #654. Frozen
measurement source `785838403d02e383ba12553a3464d492e60a5cd5` differs from the
source-PASS commit only in this spec; all five paths are absent, all six candidate/
baseline hashes match, and counterfactual restoration is limited to the named three
compiler files.

Hypatia alone may run the exact recorded setup, candidate invocation,
counterfactual invocation and validator once in order. Retain the complete inherited
Cargo/Rust environment, identities, streams and numeric statuses, and preserve the
intentional three-file overlay when checking counterfactual drift. Stop on the first
failed precondition, command or unexpected state without repair, retry, target
cleanup or extra measurement. Do not inspect counts before both variants finish.
Only compact Astra-approved evidence may enter Git.

## Official measurement verdict — FAIL

Hypatia ran the candidate, counterfactual and validator once each at the frozen
source and three-file overlay. All three returned 0, each variant emitted six
records, both rounds matched, graph/diagnostic identities matched, and the
validator passed the combined 12 records. Observed candidate versus counterfactual
counts were: zero64 20,527 versus 20,527 allocations and 3,001,967 versus
3,001,967 requested bytes; crossed-small 2,257 versus 2,371 allocations and
276,599 versus 279,687 bytes; banks64 35,738 versus 37,564 allocations and
5,075,495 versus 5,132,635 bytes.

Astra LOW nevertheless returned **MEASUREMENT FAIL**. The executor's initial
concurrency check matched its own shell and execution continued after correcting
it. The retained final preflight says the evidence directory was present when that
record was written and therefore does not independently prove all five paths absent
before creation. After execution, an erroneous 12-records-per-variant population
check was corrected to the specified six plus six instead of stopping. Root's
contemporaneous pre-delegation observation recorded all five paths absent and no
other Cargo/rustc process, but it cannot erase the stop-rule violations.

The counts remain unqualified observations only. #654 permits no correction,
rerun, extra measurement, or allocation-reduction claim. Preserve the source PASS,
all temporary worktrees, targets, streams and records. A separately numbered
evidence-only successor may mechanically reconcile root's original observation,
executor chronology, unchanged stream hashes, exact 6+6 concatenation,
source/counterfactual/toolchain/environment identity, once-only statuses, numeric
results and both procedural failures. It must forbid reruns and retrospective raw
record reconstruction and receive Astra scope PASS before work.
