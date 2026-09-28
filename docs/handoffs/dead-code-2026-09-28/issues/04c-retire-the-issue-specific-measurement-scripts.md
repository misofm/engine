# Retire the issue-specific measurement scripts

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, rows B5-B10,
and section 5.1. No ruling is needed. This draft also restores the rule in
`scripts/operator/README.md`: "Every script under `scripts/` is reachable from a GitHub workflow."

Today 18 files outside `scripts/operator/` break that rule, counting jq's extension-less
`include "name"` as a reference.

- 17 are deleted here.
- The 18th is the live `run-console-benchmark.sh`. Move it to `scripts/operator/`, and update its
  callers' paths and its own root computation. If the move is too noisy for this slice, record it
  as the one known exception.

## Context

| family | files | lines | status |
|---|---|---:|---|
| Issue #880 MQ-1/MQ-2 | `scripts/run-issue880-mq{1,2}-benchmark.sh`, `scripts/test-issue880-mq{1,2}-benchmark.sh`, `scripts/issue880-mq1-benchmark-lib.sh`, `scripts/issue880-mq{1,2}-record-validator.jq`, `scripts/fixtures/issue880-mq{1,2}-record.json`, `crates/transient-shaper/tests/bench.rs`, `crates/compressor/tests/bench_ramp.rs` | 2,170 (761 Rust) | reached by no workflow; #880 closed |
| #746/#748 "active" benchmark | `scripts/run-gate-active-benchmark.py`, `scripts/test-gate-active-benchmark.py`, `tools/bench/src/{gate_active,multiband_active}.rs` | 3,225 (1,831 Rust) | reached by no workflow |
| #650 allocation records | `scripts/check-prepared-effect-allocation-records.py`, `tools/audit/src/prepared_effect_allocations.rs` (the `prepared-effect-allocations` subject) | 890 (645 Rust) | the validator is reached by nothing; the subject is one-shot |
| #600-#606 input-trim capture (misnamed "input symmetry"; not dual-mono content detection) | `scripts/{run,preflight,test}-input-symmetry-capture.sh`, `scripts/input-symmetry-capture-validator.py`, `tools/bench/src/input_symmetry{,_capture}.rs` | 1,720 (823 Rust) | reached by no workflow |
| #163 phase-0 wasm kernel timing | `scripts/operator/run-wasm-kernel-timing.sh`, `scripts/test-wasm-kernel-timing.sh`, `scripts/wasm-kernel-timing-validator.jq`, and the `--native-timing`/`--wasm-timing` modes of `tools/wasm-gates` (around `src/main.rs:24-100` and the "#163 phase 0b" block from `src/lib.rs:444`) | 373 script lines + about 200 Rust | its folder exists; the operator script cannot run (wrong root); CI lint still runs its validator test |
| Unreached or superseded | `scripts/check-parametric-eq-targets.sh` (a wrapper with no caller), `scripts/operator/probe-opfs-move-v1.cjs` (a one-off probe), `scripts/operator/seal-web-audioworklet-browser-correctness.sh` (cannot run; superseded by the CI `browser` and `artifact-gates` jobs) | about 290 | — |

**Not in this draft:** `scripts/protocol-benchmark-record-validator.jq` looks unreferenced by
file name, but it is **live**. jq pulls it in with an extension-less
`include "protocol-benchmark-record-validator"` from `protocol-benchmark-validator.jq:1`,
`test-protocol-benchmark.sh:22` and `run-protocol-benchmark.sh:21`, and
`test-protocol-benchmark.sh` runs in the required audit-native job
(`qualification.yml:742`). It goes only with `R3-…`.

**Tests that guard live claims and must survive.** Some tests inside these files assert product
behaviour without timing:

- **`crates/compressor/tests/bench_ramp.rs`:**
  `mq2_preflight_payloads_prove_ramps_restart_on_each_block` (`:374`) asserts the both-channel
  per-block ramp restart and the rate-coefficient call counts `[128, 256, 0]`.
  - No surviving test covers this. `crates/compressor/tests/ramps.rs:119` asserts
    restart-from-current for one point only. So **port** it.
  - `crates/compressor/tests/MUTATIONS.md:327` (row 1006-M3) credits it as one of 16 red tests.
    Edit that row to name the ported test.
- **`tools/bench/src/input_symmetry.rs`, all four tests:**
  - `connected_runtime_oracle_covers_retargeted_both_channel_ramp`;
  - `separate_capacity_sixteen_drain_witness_has_no_pending_records` (`:523`);
  - `owners_are_w8_nonzero_and_phase_results_match` (`:587`);
  - `qualification_phase_constants_and_zero_render_allocations`.
- **`tools/bench/src/gate_active.rs` and `multiband_active.rs`:** the `untimed_preflight_runs_*`
  tests.
- **`tools/audit/src/prepared_effect_allocations.rs`:**
  - `crossed_small_proves_reversed_distinct_prepared_programs`;
  - `banks64_proves_current_backend_cohort_and_heterogeneous_fallback`.

**Also here: repair `prepare-builtins-listening.sh`.** `scripts/operator/prepare-builtins-listening.sh`
cannot run either: its root resolves to `scripts/`. It prepares the blinded listening packets that
AGENTS.md requires, so it is **repaired** here (`/..` becomes `/../..`), not deleted. Of the 7
operator shell scripts, only `preflight-console-benchmark.sh` resolves the root correctly today.

## Smallest closable slice

1. **Keep the live-claim tests.** For every test above:
   - move the untimed assertion into the owning crate's permanent tests (for example
     `crates/compressor/tests/ramps.rs`, a `graph-compiler` or `builtins-compiler` cohort test, or
     `effect-contract` symmetry tests); or
   - show in the evidence the surviving test that already asserts the same claim, with its
     file:line.

   Do this **before** deleting anything.
2. **Delete the files in the table**, and `artifacts/issue880/` and
   `artifacts/issue-60{2,3,6}-input-symmetry-capture/`.
   - Keep `artifacts/issue163-phase0/`: rulings cite it, and `07-…`'s retention rule owns it.
   - Append a dated history note to `docs/rulings/wasm-kernel-timing-interim.md` (which names that
     folder and `run-wasm-kernel-timing.sh` at `:55-56`) and to `docs/rulings/wasm-simd8-survey.md`
     (which cites `run-wasm-kernel-timing.sh:183-184` at `:107`). The note says the runner was
     retired, and that the record and the runner remain in git history. The rulings README asks
     for exactly this kind of note.
   - Remove the `gate-active`, `multiband-active`, `input-symmetry` and `input-symmetry-capture`
     subjects from `tools/bench/src/main.rs`.
   - Remove the `prepared-effect-allocations` subject from `tools/audit/src/main.rs`.
   - Remove the two timing modes from `wasm-gates`, and any code only they reach (prove by
     compile).
3. **Remove the lint line `bash scripts/test-wasm-kernel-timing.sh`** from `qualification.yml`.
4. **Repair `scripts/operator/prepare-builtins-listening.sh`'s root** (`../..`). Run its argument
   and usage path to show it resolves `scripts/check-builtins-listening-033.py`.
5. **Update docs that describe these tools as current.** Remove `docs/ENGINE_ENV_VOCABULARY.md`
   entries only they read (`check-env-vocabulary.sh` decides). `docs/issue880-*.md` belongs to
   `08-…`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
   - `cargo test --locked --release -p audit -p bench -p wasm-gates` and
     `cargo test --locked --all-targets -p compressor -p transient-shaper --features math/lane`
     pass. That is the feature set CI's `test-debug-b` pins.
2. **Console and wasm digests.**
   - The `gain_pan_profile digests` output is byte-identical on base and change.
   - `bash scripts/run-wasm-gates.sh` passes: the digest legs of `wasm-gates` are untouched.
3. **Shipped artifact: unchanged.** No crate in its closure changes; show `git diff --stat`.
4. **CI routing.**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - The `verdict` table is unchanged.
   - A reachability check shows every remaining file under `scripts/` (excluding
     `scripts/operator/`) is named by a workflow or by a script a workflow runs. It must count jq's
     extension-less `include "name"` as a reference, or it misreports live validators as dead.
5. **No live claim lost.**
   - Every test that step 1 lists is either ported, with its new location and a red run of the
     mutation it kills, or matched to a surviving test.
   - The `-- --list` diff contains nothing else.

## Dependencies

`00-…`. Land it in the same CI-conscious batch as `04a-…`, `04b-…` and `05-…`.

## Standing rules for the implementer

- Launch no timed workload.
- Commit on `codex/<issue>-retire-measurement-scripts`.
