# Retire the wasmtime console benchmark and decide the nightly descriptive benchmarks

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R9. The ruling to record: "Only real host paths are benchmarked. They are the native
console `--step` rows (the host-core compile path at the native width) and the V8 rows on the
shipped `host_web.wasm`. The wasmtime console benchmark is retired. The nightly descriptive
benchmarks are {kept | retired}."

## Context

**The wasmtime console benchmark** (about 2,400 lines):

- **Files:** `tools/wasm-console` (824 lines), `tools/wasm-console-guest` (272),
  `scripts/operator/run-wasm-console-benchmark.sh` (496), `scripts/operator/preflight-wasm-console-benchmark.sh`
  (269), `scripts/wasm-console-benchmark-validator.jq` and `scripts/test-wasm-console-benchmark.sh`.
  - It has 29 named arms and a default, and all 30 output folders already exist.
- **It has not been able to run since 2026-09-01.** Both operator scripts compute the repository
  root as `scripts/` (`$(dirname "$0")/..`) since they moved into `scripts/operator/` (commit
  `f0509c3f`, #319), so they source `scripts/scripts/check-bench-preconditions.sh`.
  - The last record is `artifacts/mono3` (2026-08-28).
  - The runner was still being edited on 2026-09-27 (#956).
- **It measures the wrong engine.** It runs Cranelift ahead-of-time on wasmtime, not V8. The
  records themselves say `comparable_with_console_records: false` and
  `browser_field_measurement: false`.
- **The real-host replacement already exists.** `scripts/run-web-mixing-automation-benchmark.sh …
  --step NAME` renders the shipped module under Node's V8 through `prepared-control.js`, exactly
  as the SDK drives it (#1003). Recent handoffs time other rows with ad-hoc V8 harnesses, for
  example `docs/handoffs/effects-2026-09-27/limiter-diagnosis-wasm-console.mjs.txt`.
- **CI cost:**
  - lint "Wasm console benchmark validator mutation tests";
  - nightly "Wasm console benchmark guest compiles for Wasm";
  - `check-release-shape.py`, which pins `wasm-console-guest` in the cdylib set.

**The nightly descriptive benchmarks:**

- **What they are.** The `benchmark` job in `nightly.yml` runs `run-conformance-benchmark.sh`,
  `run-realtime-benchmark.sh`, `run-session-benchmark.sh`, `run-effect-contract-benchmark.sh` and
  `run-fp-environment-benchmark.sh`, about 30 minutes a night.
  - They measure native kernels and synthetic realtime loops, not a host render path.
  - They assert no threshold, and upload JSONL that nothing reads.
- **They do not measure the shipped build either.** Every benchmark links `bench-support`, which
  enables `engine/realtime-audit` and installs an audited `#[global_allocator]`
  (`tools/bench-support/src/alloc.rs:156-157`).
  - So the console numbers are taken with the audit scope guard in place.
  - That is acceptable for a relative `--step` series, but the figures are not "the shipped
    build".

## Smallest closable slice

1. **Wasmtime console.**
   - Delete `tools/wasm-console`, `tools/wasm-console-guest`, both operator scripts, the validator
     and its test.
   - Remove the lint step line, the nightly guest check, and the guest from
     `check-release-shape.py`'s expected set.
   - Leave `artifacts/` to `07-…`.
2. **Nightly benchmarks (your choice).**
   - **If retired:** delete the `benchmark` job and its five runner scripts, and update
     `failure-notice`.
     - Delete the `bench` `session` subject, which only `run-session-benchmark.sh` launches.
     - **Keep** the `bench` `conformance` and `effect-contract` subjects and the `audit realtime`
       subject. audit-native's `check-effect-contract.sh` runs the first two as *conformance*
       gates, and `trace-realtime-audit.sh` runs `audit realtime`.
     - The `audit fp-env` subject is launched only by `run-fp-environment-benchmark.sh` and has no
       unit tests (audit grep). Delete it with its runner, unless it guards a live claim; the FP
       environment itself stays guarded by `lane`'s `fp_env` tests.
     - Remove only the timing-only code paths that nothing else reaches, proven by compile.
   - **If kept:** no change.
3. **Optional follow-up** (not in this slice): promote a multi-row V8 console harness from the
   handoffs to `scripts/`, with `--step NAME` like the mixing-automation arm, so browser numbers
   exist for every console row.
4. **Docs.** Update `scripts/operator/README.md` and `docs/ENGINE_ENV_VOCABULARY.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `console-workload`, which the native benchmark and `bench` still use, is unchanged.
3. **Shipped artifact: unchanged.** No crate in its closure changes. Show
   `git diff --stat -- crates hosts`, which should be empty (audit section 11).
4. **CI routing.**
   - `check-release-shape.py` and its `--self-test` pass, as do `check-bench-policy.sh`,
     `test-bench-policy.sh`, `check-ci-path-routing.py` and `test-ci-path-routing.py`.
   - `nightly.yml`'s `failure-notice` `needs:` list and body match its jobs.
   - The `verdict` table is unchanged.
5. **No live claim lost.**
   - `check-effect-contract.sh`'s conformance record still runs in audit-native.
   - Every deleted test belongs to a deleted tool or runner. List them from the `-- --list` diff
     (audit section 11).

## Dependencies

The owner ruling. It absorbs the tool half of `R7-…` if it lands first.

## Standing rules for the implementer

- Launch no timed workload.
- Commit on `codex/<issue>-retire-wasmtime-console`.
