# Issue #880 — Astra adversarial review, attempt 1

Reviewed on 2026-09-25 by the fresh Astra xhigh reviewer, against
`e09302ad..21b8b7f5570dbdff8a43c2a04db001a8c62806c9`. Scope is the authorized Class-A
batch, MC-1, and partial delivery synchronization. R1/R2/R3/R4 remain pending; no
Class-B implementation or issue closure is approved by this review.

**Batch verdict: FAIL — one bounded MA-3 test-registration correction required.**

| Task | Attempt-1 verdict | Basis |
|---|---|---|
| MA-1 | PASS | Hardware wrappers, independent retained integer oracle, signed-zero/NaN treatment, exhaustive f32 and sampled f64 coverage, and codegen evidence satisfy the scope. No pin moved. |
| MA-2 | PASS | Full-domain refit measurements and 77/0/0 reversal counts independently reproduced. The sweep includes worker-boundary pairs; production fast-tier bodies are unchanged. The recorded coefficient mutation is distinct from the degree argument. |
| MA-3 | FAIL | Arithmetic identity passes all 2^32 patterns and all three native widths, but the new test breaks the standalone default-feature math test command. See B1. |
| MA-4 | PASS | The crate guidance covers Lane-only unfused render-rate math, full-domain/identity evidence, tier sealing, and the named future consumers. Rustdoc passes. |
| MA-5 | PASS | Default domain/container checks and both exhaustive rail and four-corner pipeline proofs pass. Reported maxima reproduce; crossings remain six and no tolerance/pin changed. |
| MQ-1 | PASS for the recorded failure disposition and handoff | The frozen real-kernel workload and one-warmup/two-round raw evidence are preserved. The known parser failure remains candidly unaccepted, with repair/promotion assigned to #902 as required by the benchmark rule. This is not a validated-baseline promotion. |
| MQ-2 | PASS | Frozen eight-bank workload, both-channel ramp-state proof, 1,024 calls per bank/parameter, per-block samples, summaries, provenance, and non-timed preflight agree. |
| MC-1 | PASS | All five options state cost, accuracy/bit impact, changed-mask behavior, and restore/partition constraints. No unmeasured speedup or owner decision is asserted. |
| MZ-1 | PASS for partial local handoff only | The issue remains open, owner rulings remain pending, and the record distinguishes local checkpoints from remote delivery. Upstream evidence synchronization remains the coordinator's batch-boundary obligation. |

## B1 — Required correction

`crates/math/tests/e1_identity.rs:10` imports the optional `lane` dependency and lines
72–81 call the feature-gated `math::exp2_lane`. Unlike M1, M2, and F1,
`crates/math/Cargo.toml:26` onward does not register this target with
`required-features = ["lane"]`.

Reproduction: `cargo test --locked --release -p math` exits 101 with E0432 for `lane`
and E0425 for `math::exp2_lane`. An integrated invocation enabling `math/lane` masks
the failure. Add an `[[test]]` entry for `e1_identity` with the same feature
requirement as the existing lane tests; retain the optional dependency contract.
Validate the standalone command and the explicit lane-enabled E1 test target. This
is the sole blocking finding and belongs to MA-3's second, final attempt.

## Independent verification

- `cargo test --locked --release -p math --test scalar_accuracy`: 8 passed, 2 ignored.
- The same target with `-- --ignored --nocapture --test-threads=1`: 2 passed;
  all 2^32 f32 patterns and 100,000,000 deterministic f64 patterns matched the retained
  software oracle (NaNs compared by class).
- `cargo test --locked --release -p math --features lane --test e1_identity -- --ignored --nocapture --test-threads=1`:
  4,294,967,296 patterns, scalar/pre-E1/Simd4/Simd8, zero mismatches.
- F1 default suite: 12 passed, 6 ignored, six admitted crossings. Its full ignored
  suite: 6 passed, with refit P errors `3.210375e-5` / `2.662959e-5` dB, Q error
  `1.005557e-4` dB, and shipped reversal counts 77/0/0. MA-5 reproduced gain-tier
  difference `1.654115e-5` dB and unit-input fast/exact oracle errors
  `1.287460e-5` / `2.199415e-6`.
- Compressor `bench_ramp` default suite: 2 passed, timed test ignored.
  Both benchmark preflight self-tests pass with zero timed workload launches.
- `cargo doc --locked -p math --no-deps`: PASS without warnings.
- MQ-2 frozen jq validation: PASS. Candidate tree, both harness/runner hashes, and
  MQ-1 raw-log hash match their records. Raw measurements match the stated summaries.
- Reviewed the actual square-root before/after assembly in `/tmp/issue880-ma1/`;
  after bodies contain native `vsqrtsd`/`vsqrtss` and wasm `f64.sqrt`/`f32.sqrt`.
  No source change introduces allocation, I/O, locking, runtime target dispatch,
  or fused arithmetic. Existing canonical FP environment requirements remain relevant.
- Compared pinned corpus files with the baseline: unchanged. M1 changes only its
  explanatory header. The coordinator's integrated native/wasm, clippy, fmt, and
  policy results were reviewed as supplied evidence, not represented as independent
  reruns here. AArch64 execution is not claimed; the IEEE-operation argument is the
  issue's accepted target coverage.

No timed benchmark was rerun. Original raw-log trailing blank lines are evidence,
not a source formatting defect, and must remain unchanged.

## Optional documentation nits

- `docs/issue880-mq1.md:3` prints a 39-character candidate abbreviation. The artifact
  correctly records the full ID ending in `dce4`; matching it would avoid ambiguity.
- `docs/issue880-ma2-ma5.md:19` says all MA-5 tests are ignored, but the domain test at
  `crates/math/tests/f1_fast_db_bounds.rs:662` runs by default. Say the exhaustive
  MA-5 sweeps are ignored; the default domain/container checks remain active.

These nits do not alter the task verdicts or require another measurement.
