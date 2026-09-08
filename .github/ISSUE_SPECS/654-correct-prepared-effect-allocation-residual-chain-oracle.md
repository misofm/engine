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
