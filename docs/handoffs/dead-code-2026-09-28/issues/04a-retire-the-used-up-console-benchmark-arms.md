# Retire the used-up console benchmark arms

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, row B1. No
ruling is needed: each arm removed here can never run again.

## Context

`scripts/run-console-benchmark.sh` (682 lines) is the live native console benchmark. Its only
usable mode is `--step NAME`, which writes `artifacts/steps/NAME`.

**The dead arms.**

- Besides `--step`, the runner carries 48 named historical arms (`:283-330`): `--phase2`,
  `--issue163-phase0` … `--plumbing-floor-baseline`. It also has a no-argument default that
  writes `artifacts/issue149`.
- All 49 target folders exist. The audit checked all 48 named arms and `artifacts/issue149`.
- The runner refuses to overwrite a record (`:351-353`), so every one of those modes can only
  fail.
- Lines `:1-271` are header comments, mostly each arm's history.

**The preflight is out of sync.** `scripts/operator/preflight-console-benchmark.sh` (177 lines)
mirrors 40 of the 48 arms. It lacks these eight:

- `--copy-removal`, `--copy-removal-baseline`, `--copy-removal-without-920`;
- `--issue388-lane4-evidence`;
- `--plumbing-floor`, `--plumbing-floor-baseline`;
- `--pure-path`, `--pure-path-baseline`.

So the lists are not maintained. Yet they are still edited: #956 re-indexed arms that cannot run.

**Tests do not name arms.** `scripts/test-console-benchmark.sh`, `check-bench-policy.sh`,
`test-bench-policy.sh` and the console jq validators reference no historical arm by name (audit
grep). So removing the arms cannot weaken any validator.

**The wasm console runner is out of scope.** `scripts/operator/run-wasm-console-benchmark.sh` and
its preflight have 29 more arms of the same kind, but they belong to ruling `R9-…`.

## Smallest closable slice

1. **`run-console-benchmark.sh`.**
   - Keep only `--step NAME`. A call with no arguments, or with any other argument, is a usage
     error (exit 2).
   - Replace the 271-line header with a short description of the `--step` mode and its
     preconditions. The arms' histories stay in git and in their issue specs.
2. **`operator/preflight-console-benchmark.sh`.** The same change: `--step NAME` only.
3. **Docs.** Update every live doc (outside `docs/handoffs/`) that tells a reader to run a removed
   arm: `scripts/operator/README.md`, rulings that name an arm as a re-run recipe, and
   `docs/ENGINE_ENV_VOCABULARY.md`. Leave the records under `artifacts/` alone; `07-…` handles
   them.

## Objective gates

1. **Scripts.**
   - `bash scripts/test-console-benchmark.sh`, `bash scripts/check-bench-policy.sh`,
     `bash scripts/test-bench-policy.sh`, `bash scripts/check-bench-preconditions.sh` and
     `bash scripts/check-console-benchmark-fixture.sh` pass.
   - `bash scripts/run-console-benchmark.sh` and `bash scripts/run-console-benchmark.sh --strip4`
     exit 2 with the new usage.
   - `bash scripts/run-console-benchmark.sh --step base` refuses to overwrite the existing record
     and launches no workload.
2. **Native and wasm build:** unaffected, because no Rust changes. Both still pass:
   - `cargo check --locked --workspace --all-targets`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`.
3. **Console digests:** unaffected; no engine or workload source changes. State this in the
   evidence with `git diff --stat` showing only `scripts/` and `docs/`.
4. **Shipped artifact:** unaffected; no crate in its closure changes.
5. **CI routing.** No workflow step names a removed arm. `python3 -B scripts/check-ci-path-routing.py`
   and `python3 -B scripts/test-ci-path-routing.py` pass. The change routes to `full` because
   `scripts/` is not an evidence path.
6. **No live claim lost.** No test is removed, and every lint step that ran before still runs.

## Dependencies

None. It can land in the same CI-conscious batch as `04b-…` and `04c-…`.

## Standing rules for the implementer

- Do not run the benchmark. The gates need only argument and preflight paths, which launch no
  workload.
- Commit on `codex/<issue>-retire-console-arms`.
