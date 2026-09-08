# Repair and qualify the prepared-effect allocation harness

GitHub: https://github.com/misofm/engine/issues/652

Parent: #560 CP1. Failed predecessor: #650. Product predecessor: #633. Coordination: #559. Candidate source checkpoint: `819c6ef6`. Product main: `4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`.

#650 preserved a useful but invalid three-path allocation harness after exhausting its three implementation passes. No official measurement ran and #650 earned no product or allocation-reduction credit. This successor repairs that frozen source before any counted execution; it does not widen the allocator, compiler, dependency, benchmark, or production-source boundary.

## Smallest closable slice

Inherit and revise only:

- `tools/audit/src/prepared_effect_allocations.rs`;
- `tools/audit/src/main.rs` only if dispatcher behavior must be corrected;
- `scripts/check-prepared-effect-allocation-records.py`;
- this issue spec and the #559/#560 coordination records.

Attempt 1 must make each compile request a fully prepared owned value before reading the starting counters. The counted interval begins immediately before `GraphCompiler::compile(request)` and ends with the counter read immediately after it returns. Identity must be computed from that measured result after the ending counter read, and the result must be dropped afterward. Fixture construction, session compilation, registries, effect preparation, identity serialization, diagnostics, record construction, formatting, output, and plan destruction remain outside the interval.

Add a separate deterministic allocator positive control outside all official intervals. It must read a start mark, perform a non-elidable heap allocation, read the delta before drop, and assert positive allocation calls and requested bytes. It must not contribute to any emitted corpus counter.

Freeze and structurally test these causal inputs before source review:

1. `zero64`: 64 effect-free tracks with matching graph input and dispatch across variants.
2. `crossed-small`: repeated effect IDs across tracks and racks, at least two entries whose prepared program identities are demonstrably distinct, and a deterministic nontrivial reversal of prepared-entry order. Its invalid twin and exact ordered diagnostic identity remain unmeasured.
3. `banks64`: `Backend::current()` with a fixed multi-slot chain that produces at least one homogeneous bank cohort and fixed heterogeneous members/fallback. Tests assert the prepared order, program identities, cohort membership/counts, and relevant graph bank overlay rather than only track counts.

The harness still accepts only `--variant candidate|counterfactual`, performs one warmup and two measured rounds, emits the existing strict JSONL schema, and takes no timing. Both measured rounds must agree exactly on all four counters. The validator keeps exact schema/key order/population, duplicate-key/type/range/trailing-data rejection, semantic and diagnostic equality, zero-control equality, prepared-corpus reductions, and synthetic mutation coverage without launching the audit subject.

## Attempts and premeasurement gates

Luna HIGH or XHIGH implements one coherent attempt. Stop on the first failed gate and return control to root; do not correct or rerun within that turn. Root checkpoints and pushes the exact source, then Astra LOW independently reviews the interval, allocator control, corpus assertions, actual measured-result identity, strict validator, exact heads, and retained gate evidence. No official candidate or counterfactual workload may run before source PASS.

Use fresh absent `/tmp` paths for gate records and capture each command, cwd, full head, toolchain, stdout, stderr, and numeric status. Run these once in order, stopping on failure:

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

After source PASS, amend this spec with Astra-reviewed exact measurement commands, environment, heads, hashes, fresh paths, and matched-counterfactual procedure inherited from #650. The counterfactual restores only `crates/graph-compiler/src/compile.rs`, `ids.rs`, and `banks.rs` from `d98646db47bc603c32431d999cd08f43a0168043` into a detached worktree at the exact pushed harness checkpoint. Stop if those three restored files do not compile without edits.

Run each release variant once with one internal warmup and two measured rounds; preserve full temporary streams only through review. PASS requires status 0, exact population, exact two-round determinism, cross-variant graph and diagnostic identity, equality of all four `zero64` counters, fewer candidate allocation calls and requested bytes in both prepared corpora, positive crossed-small savings, and more saved allocation calls in `banks64` than crossed-small. Report exact counts and deltas only. Commit no `.ll`, assembly, compiler stream, target directory, object, archive, binary, or raw measurement output.

Astra LOW must pass measurement evidence and the compact decision record before PR review, required CI, guarded merge, post-main qualification, GitHub synchronization, closure, and clean worktree removal. This successor can close only the allocation qualification for #648's prepared-effect handoff; CP1's schedule, PDC, cycle, reduction, and buffer identities remain open.

## Astra LOW implementation-scope review — PASS

Astra passed exact clean branch/upstream head
`641a6dc57077e3cd7df1ea1241e353727ef8db47`, live main
`4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`, inherited failed source
`819c6ef6`, and synchronized tracker `996d24e0`. GitHub #652 matches this issue;
#650 is closed, and #651/#652 are the two disjoint active slots.

One root-designated Luna HIGH or XHIGH executor may perform implementation attempt
1 in `prepared_effect_allocations.rs`, the dispatcher only if necessary, and the
strict validator. Structural tests must prove actual prepared program identities
and bank membership rather than labels or counts. The executor runs the listed
gates once with fresh retained command/status evidence and stops on the first
failure without correction or rerun. Root checkpoints and pushes before a fresh
Astra source review. No official count, counterfactual, production, allocator,
dependency, timing, or compiler-payload action is authorized.
