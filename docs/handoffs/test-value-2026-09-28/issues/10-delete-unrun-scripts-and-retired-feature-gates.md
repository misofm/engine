# Delete unrun one-shot scripts and the gate rules that guard retired features

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 10 and §7). Base `a9414c0c`. The
isolated-benchmark rows need owner ruling R6; the rest need none.

## Problem

**Scripts no workflow reaches.** 26 files under `scripts/` are not reached from any workflow or
`package.json`, even transitively. The derivation is `reach.py` in the audit's evidence; the list is
in `../data/script-gates-verify.md` §6.
- 8 are operator tools in `scripts/operator/`, by design.
- `scripts/run-console-benchmark.sh` (682 lines) is a live operator runner outside `operator/`.
- **18 are one-shot issue tooling, 3,958 lines**, whose records are already sealed:
  - issue 880 MQ-1/MQ-2, 9 files, 1,409 lines;
  - issue 606 input-symmetry capture, 4 files, 897 lines;
  - issue 746 `run-gate-active-benchmark.py` and its test, 1,394 lines;
  - issue 650 `check-prepared-effect-allocation-records.py`, 245 lines;
  - the `check-parametric-eq-targets.sh` compatibility shim.

`scripts/operator/README.md` claims that every script under `scripts/` is reachable from a workflow.
Nothing checks it.

**The unrun tooling is not free.**
- It keeps 20 rows alive in `docs/ENGINE_ENV_VOCABULARY.md` (9 `MISO_ENGINE_606_*` and 11
  `MISO_ENGINE_CAPTURE_*`), which the env gate then enforces.
- `tools/bench/src/input_symmetry.rs` (597 lines) and `input_symmetry_capture.rs` (226 lines) still
  compile into `bench`.

**Gate rules for retired features**, with file:line in `../data/script-gates-verify.md` §4:
- the FLAC and delivery-codec package names (`check-workspace-policy.sh:162-163`, `:218-272`, and 17
  cases in `test-workspace-policy.sh`);
- retired session-TOML spellings (`check-session-policy.sh:46-63`, `check-sdk-deletions.py:120-121`,
  `:409`);
- retired softfma definitions (`check-unfused-seal.sh:723-732`, plus self-test cases);
- the retired delta-bank EQ kernel names (`check-parametric-eq-render-contract.sh:25-28`);
- the pre-boot-v1 SDK spellings (`check-sdk-deletions.py:99-125`). **These fired 4 times in 73 runs,
  each time on new, legitimate code**: `"prepared"` in `prepared-control.js` and `pcm-feed`, and a
  `limits:` key in `live-response.ts` and `response.ts`;
- the superseded raw-WebDriver harness: `scripts/web-audioworklet-browser-correctness.py` (752
  lines), its runner, its operator seal, and the self-test lines `test-web-audioworklet.sh:146-157`
  that CI still runs;
- the builtins-less console rows in the benchmark validators (`test-console-benchmark.sh:531-536`,
  `test-wasm-console-benchmark.sh:164-170`);
- the ban on the word `nudge` (`scripts/check-step-vocabulary.py`), which is already refused as a
  wire key by `check-parameter-metadata-v1.py:733-735`.

## Outcome

- **Delete the 18 one-shot files.** Their records stay under `artifacts/` and are referenced by
  commit hash in their closed issue specs.
- **Move `run-console-benchmark.sh` into `scripts/operator/`,** or reach it from a workflow, so that
  the README is true.
- **Remove the env-vocabulary rows** that only the deleted files used.
- **Under R6,** also delete `tools/bench/src/input_symmetry*.rs`, `gate_active.rs` and
  `multiband_active.rs` (isolated, non-host-path benchmarks) with their tests.
- **Remove each retired-feature rule** above, and its self-test cases.
- **Keep `check-sdk-deletions.py`'s live rule:** no numeric ABI byte offset outside `src/generated/`.
  It has a real defect history, #207 N-13(d).
- **Keep the naming rule for the retired `miso-engine-` prefix.** It is live policy in AGENTS.md.

## Scope

Authorized paths:
- the listed `scripts/` files;
- `scripts/check-workspace-policy.sh`, `test-workspace-policy.sh`;
- `scripts/check-session-policy.sh`, `test-session-policy.sh`;
- `scripts/check-unfused-seal.sh`;
- `scripts/check-parametric-eq-render-contract.sh`;
- `scripts/check-sdk-deletions.py`, `scripts/check-step-vocabulary.py`;
- `scripts/test-web-audioworklet.sh`, `scripts/test-console-benchmark.sh`,
  `scripts/test-wasm-console-benchmark.sh`;
- `docs/ENGINE_ENV_VOCABULARY.md`;
- `tools/bench/src/` (R6 rows only);
- `.github/workflows/qualification.yml` (the step-vocabulary step);
- this issue's spec.

## Gates

1. **Reachability.** The audit's `reach.py` procedure, re-run on the result: every file at the top
   level of `scripts/` is reached from a workflow or `package.json`, and everything else is in
   `scripts/operator/`. Add the check as one line to `check-workspace-policy.sh` only if it is one
   line.
2. **Live rules still fail.** In a scratch branch, each of these fails its gate:
   - a numeric ABI offset in `sdk/src/` outside `generated/`;
   - a `mul_add` call outside `lane`;
   - a `MAX_TRACKS` constant;
   - a `core`-named package.
3. **The env gate is consistent.** `check-env-vocabulary.sh` passes, with no documented name unused
   and no used name undocumented.
4. **Historical bugs.** Unaffected, since no Rust test is touched unless R6 applies. With R6, run
   `../tools/revert.py` for all four and confirm the red set is unchanged.

## Saving and risk

- **Saving:** ≈ 0 s of CI. Removes about 4,000 lines of scripts, about 1,000 lines of benchmark code
  under R6, and 4 false reds a month from the SDK word bans.
- **Risk:** nil. The records are sealed, and nothing live calls these files.
