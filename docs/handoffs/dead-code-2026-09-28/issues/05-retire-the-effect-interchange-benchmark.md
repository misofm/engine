# Retire the effect-interchange benchmark

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, row B11. No
ruling is needed.

- #081 and #108 are closed.
- The benchmark measures the third-party descriptor, package and state interchange, which no host
  runs. "Only real host paths are benchmarked" excludes it.
- The interchange **qualification** (the reference-process cross-check of the wire formats) is not
  touched here. It goes, or stays, with ruling `R6-…`.

## Context

Benchmark-only files (2,819 lines, 1,092 of them Rust):

- `scripts/preflight-effect-interchange-benchmark.sh`
- `scripts/run-effect-interchange-benchmark.sh`
  - "Sole exactly-once Issue 081 benchmark entrypoint".
  - Writes `target/issue081`.
- `scripts/test-effect-interchange-benchmark.sh`
- `scripts/effect-interchange-benchmark-validator.py`
- `scripts/check-effect-interchange-benchmark-108.sh`
- `scripts/test-effect-interchange-benchmark-108-policy.sh`
- `scripts/effect-interchange-benchmark-108-validator.py`
- `tools/bench/src/effect_interchange.rs` (the `effect-interchange` subject).
  - This is the only non-test caller of `effect-compiler`'s migration entry points.
  - For example `resolve_effect_state_migration` and
    `restore_unpublished_effect_bank_track_state_with_migration`, found by the SCIP
    cross-reference.

Where CI reaches them:

- The lint step "Effect interchange qualification and policy mutation tests" runs
  `check-effect-interchange-qualification.sh`. That script calls the #108 benchmark check and the
  benchmark preflight.
- It also runs `test-effect-interchange-policy.sh`, which mutates the benchmark scripts.

## Smallest closable slice

1. Delete the eight files above and the `effect-interchange` subject in `tools/bench/src/main.rs`.
2. Edit `check-effect-interchange-qualification.sh` and `test-effect-interchange-policy.sh` so
   they no longer call or mutate the deleted scripts. Keep their qualification half intact for
   `R6-…` to decide.
3. Delete `artifacts/issue455-interchange-completion/` (124 files, 455 KB) if nothing left reads
   it (the audit found nothing that does). Otherwise leave it for `07-…`.
4. Remove `docs/ENGINE_ENV_VOCABULARY.md` entries read only by the benchmark
   (`MISO_ENGINE_BENCH_TOOL_SOURCE_SHA256`, `…_TOOL_MANIFEST_SHA256`,
   `…_FIXTURE_MANIFEST_SHA256`), as `check-env-vocabulary.sh` decides.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
   - `cargo test --locked --release -p bench` passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change.
3. **Shipped artifact: unchanged.** No crate in its closure changes.
4. **CI routing.**
   - `check-effect-interchange-qualification.sh`, `test-effect-interchange-policy.sh`,
     `check-cross-targets.sh`, `check-env-vocabulary.sh` and `test-env-vocabulary.sh` pass.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - No job or `verdict` entry changes.
5. **No live claim lost.** The two unit tests in `effect_interchange.rs` are:
   - `exact_four_rate_migration_envelope_without_timing`;
   - `digest_hex_matches_known_abc_digest`.

   Either port them into `effect-compiler`'s migration tests, or show that
   `crates/effect-compiler/tests/migration.rs` or `migration_terminal.rs` already covers the
   four-rate envelope. The
   `-- --list` diff contains nothing else.

## Dependencies

`00-…`. Land it in the same CI-conscious batch as `04a-…`, `04b-…` and `04c-…`. If `R6-…` is
ruled first, it absorbs this draft.

## Standing rules for the implementer

- Launch no timed workload.
- Commit on `codex/<issue>-retire-interchange-benchmark`.
