# Retire the one-shot rack, builtins and graph-compiler benchmarks

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, rows B2-B4. No
ruling is needed: each is a "sole exactly-once" benchmark for a closed issue. It has either
produced its record or can no longer produce one, and none of them measures the shipped host path.

## Context

| family | files | lines | why it cannot run again, or is not wanted |
|---|---|---:|---|
| Rack #038 | `scripts/run-rack-benchmark.sh`, `scripts/test-rack-benchmark.sh`, `scripts/rack-benchmark-{record-lib,record-validator,validator}.jq`, `scripts/check-rack-benchmark-fixture.sh`, `scripts/fixtures/rack-benchmark-validator-record.json`, `scripts/operator/preflight-rack-benchmark.sh`, `fixtures/rack/issue038-v1/`, `tools/bench/src/rack.rs` | 1,905 (947 Rust) | `artifacts/issue038/` exists, and the runner refuses to overwrite it; the operator preflight also cannot run (section 5 of the audit) |
| Builtins #072 and #431 | `scripts/{run,preflight,test}-builtins-benchmark.sh`, `scripts/builtins-benchmark-{record-validator,validator}.jq`, `scripts/{run,preflight,test}-builtins-current-benchmark.sh`, `scripts/builtins-current-benchmark-{record-validator,validator}.jq`, `tools/bench/src/builtins.rs` | 4,935 (2,095 Rust) | #431's `artifacts/issue431-full-chain/` exists; #072's runner is "sole exactly-once" and reads spec 068's text |
| Graph compiler #006 | `scripts/run-graph-compiler-benchmark.sh`, `scripts/promote-issue006-graph-benchmark.sh`, `scripts/test-graph-benchmark.sh`, `scripts/graph-benchmark-{record-validator,validator}.jq`, `scripts/fixtures/graph-benchmark-validator-record.json`, `tools/bench/src/graph.rs` | 1,350 (847 Rust) | "sole future Issue-006 entry point"; it measures compile time, not a host render path |
| Wrapper | `scripts/check-builtins-targets.sh` | 10 | a "thin wrapper kept so any caller keeps working"; its only caller is `test-builtins-benchmark.sh` |

**CI steps that exist only for these benchmarks:**

- `qualification.yml` lint:
  - "Console and rack benchmark fixture integrity": its rack half.
  - "Builtins benchmark real-tree manifest consumers" (`test-builtins-benchmark.sh
    --check-manifest-consumers`). It checks that one fixture hash is copied identically into 7
    places; 6 of those places are the files deleted here.
  - The `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` lines of "Benchmark
    preconditions and validator mutation tests".
- `nightly.yml` `moved-mutation-suites`: `test-builtins-benchmark.sh` and `test-graph-benchmark.sh`.

**Coupling that must be edited, not deleted:**

- `scripts/test-env-vocabulary.sh` uses `run-rack-benchmark.sh` as its mutation target (`:34`,
  `:46`, `:94`, `:147`, `:163`). Re-point it at `run-console-benchmark.sh`.
- `tools/bench/src/main.rs` lists the `builtins`, `graph` and `rack` subjects.
- `docs/ENGINE_ENV_VOCABULARY.md` lists variables used only by these runners. For example,
  `MISO_ENGINE_BENCH_CANDIDATE_SHA256` is used only by rack. `check-env-vocabulary.sh` must still
  pass.
- `tools/audit/src/builtins_graph.rs`'s `ACCEPTED_MANIFEST_SHA256` stays. It guards the live
  `fixtures/builtins/v1` corpus in `audit builtins-graph`.

**Not coupled:** `scripts/check-builtins-listening-033.py` pins hashes of #110 builtins-benchmark
outputs as constants and reads none of these files.

## Smallest closable slice

1. Delete every file in the table, and `artifacts/issue038/` and `artifacts/issue431-full-chain/`.
2. Remove the `builtins`, `graph` and `rack` subjects from `tools/bench/src/main.rs`. Delete any
   `bench-support` or `bench` helper that becomes unused, proven by compile.
3. Edit the CI steps listed above: delete the lint manifest-consumer step, drop the rack half of
   the fixture step and the two mutation-test lines, and drop the two nightly lines.
4. Re-point `test-env-vocabulary.sh`. Drop only the vocabulary entries that no remaining script or
   crate reads, as `check-env-vocabulary.sh` proves.
5. Update live docs that name these runners (`scripts/operator/README.md`, the rulings'
   inventories only where they state current state). Leave spec 068 in place; `R10-…` decides on
   closed specs.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
   - `cargo test --locked --release -p bench -p console-workload` passes. Its test list shrinks
     only by tests inside the deleted subject modules; list them.
2. **Console digests.** The `gain_pan_profile digests` output is byte-identical on base and change.
   No engine source changes.
3. **Shipped artifact: unchanged.** No crate in its closure changes. `git diff --stat` shows none
   of `crates/`, `hosts/`.
4. **CI routing.**
   - Every edited step still runs something, or is deleted whole. The `verdict` expectation table
     is unchanged, because no job is added or removed.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-bench-policy.sh`, `test-bench-policy.sh`, `check-env-vocabulary.sh`,
     `test-env-vocabulary.sh`, `test-console-benchmark.sh` and `check-bench-preconditions.sh` pass.
5. **No live claim lost.** The only tests removed are the deleted runners' own mutation suites and
   the unit tests inside the three bench subjects: `rack` 3, `builtins` 11 and `graph` 7. Some of
   those drive production code without timing, for example
   `builtins::all_render_workloads_arm_only_product_render_without_timing`,
   `rack::zero_launch_preflight_prepares_each_exact_production_workload` and
   `graph::canonical_benchmark_fixture_prepares_and_compiles`. For **each** deleted test, the
   evidence either:
   - names the surviving test in the owning crate that asserts the same product claim; or
   - ports the untimed assertion into that crate's tests before deleting it.

   `fixtures/builtins/v1` stays guarded by `check-builtins-fixtures.sh` and
   `audit builtins-graph`.

## Dependencies

`00-…`. Land it in the same CI-conscious batch as `04a-…`, `04c-…` and `05-…`. It must land before
`R10-…`, because spec 068 is read here.

## Standing rules for the implementer

- Launch no benchmark workload.
- Commit on `codex/<issue>-retire-one-shot-benchmarks`.
