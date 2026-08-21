# 045 Launch parametric EQ recurrence derivation and runtime proof

## Outcome

Derive and prove one numerically stable `f32`-lane parametric-EQ runtime topology before any further
production EQ implementation.

## Context

Issues **Numerically conditioned launch parametric EQ realization** and **Conditioned time-domain
launch parametric EQ recurrence** both stopped after their attempt budgets. Issue 042 proved a
conditioned transfer and broad integration surface but its direct-history recurrence recovered on
the first legal impulse. Issue 044 corrected its finite-window oracle and then showed that scaled
direct, transposed and subnormal-flush variants all fail the frozen time-domain selection gate.

This is a research/proof issue, not a product implementation issue. It has one Terra investigation
and at most one bounded Sol correction. A second failure stops. Production, graph, audit, benchmark
and timing changes are forbidden; `timed_benchmark_invocations=0`.

## Scope

- Re-derive candidate transfer-to-recurrence mappings independently and first prove f64 recurrence
  equivalence to the f64 reference impulse before any retained-f32 comparison.
- Compare exactly three genuinely different fixed structures frozen in the tracked brief:
  endpoint-conditioned numerator plus a normalized two-state lattice denominator; the
  endpoint-conditioned delta transfer evaluated with fixed non-FMA double-single `f32`
  arithmetic/state; and a deterministic Hankel-balanced real two-state realization. Do not add,
  substitute or repair a fourth family after seeing results.
- Preserve the public four-section EQ surface, four launch rates, domains and response/time-domain
  tolerances from Issues 042/044.
- Select only a fixed-size scalar/W4/W8-feasible candidate that passes the complete frozen analytic,
  48 impulse and 48 million-sample reference matrix with zero recovery.
- Freeze a machine-checkable derivation, equations, words/state, operation/FMA graph and transcript.

## Required public interfaces/contracts

No production interface changes are allowed in this issue. The proof must target the existing
`miso.parametric-eq` surface and `f32` audio/SIMD lanes. Multiword state may use a fixed number of
`f32` words per value, but f64 production state, dynamic storage and hidden scalar-only behavior are
forbidden.

## Deliverables

- cited derivation note and independent f64 algebra plus 4,096-sample impulse-equivalence tests;
- complete retained-f32 candidate comparison over all frozen gates;
- exact hashes, first failures and storage/SIMD feasibility table; and
- Sol PASS selection amendment or final STOP with no production edits.

## Explicit non-goals

Production EQ code; graph/fixture/audit/target reruns; tolerance/domain changes; extended rates;
new filter kinds; dynamic EQ; performance tuning; listening; or benchmarks.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral
- Numerically conditioned launch parametric EQ realization
- Conditioned time-domain launch parametric EQ recurrence

Stopped dependencies contribute evidence only, not PASS.

## Sol implementation brief

**READY FOR THE ONE AUTHORIZED SOL-ATTEMPT-2 COMPLETE RUN, RESEARCH ONLY.** The tracked brief is
`.github/ISSUE_SPECS/BRIEFS/045-launch-parametric-eq-recurrence-derivation-and-runtime-proof.md`.

## Hazards/decisions

Do not compare a finite impulse with an infinite analytic response, assume algebraic equivalence,
reuse a failed recurrence under a renamed state, flush instability, or select from one repaired row.
Every candidate must pass f64 equivalence before f32 retention is evaluated.

## Acceptance gates with objective measurements

1. Symbolic/numeric expansion and 4,096-sample f64 impulse tests prove each candidate realizes the
   intended transfer to <=1e-12 over all 1,488 frozen rows before any retained-f32 result for that
   candidate is evaluated.
2. Retained-f32 candidates pass the complete analytic/search gates plus all 48 one-second and 48
   million-sample cases with zero recovery and unchanged 0.005/0.05 dB and 0.1% tolerances.
3. State/output is finite normal-or-positive-zero; any underflow policy is explicit and cannot hide
   nonfinite or unstable state. Scalar/W4/W8 shapes and operation/FMA sites are fixed and bounded.
4. Results record exact candidate words/state, maxima, first failures, deterministic hashes and
   storage costs. If one or more candidates pass, the frozen non-timing rank in the tracked brief
   selects exactly one; if none passes, stop.
5. Focused reference tests, warning-denied Clippy, format and diff checks pass. No production diff
   or benchmark/timing artifact exists.

## Target matrix

Reference tests run natively. SIMD feasibility is a static fixed-layout/operation-graph proof only;
production cross-target work belongs to a later implementation issue after selection.

## Required evidence

Issue-042/044 hashes and failures; citations/derivations; f64-equivalence and retained-f32 matrices;
storage/operation tables; deterministic hashes; Terra and Sol verdicts; production no-diff proof;
and `timed_benchmark_invocations=0`.

## Terra attempt 1 — phase-one harness checkpoint (partial)

Added a test-boundary-only, ignored f64 derivation scaffold in
`crates/miso-engine-dsp-reference/src/parametric_eq_recurrence_proof.rs`. It fixes the frozen
candidate order and 1,488-row grid, reconstructs L1/D2/B3 back to normalized RBJ coefficients,
uses deterministic partial-pivot Lyapunov solves plus canonical balanced-B3 eigenvector signs, and
contains the required 4,096-sample zero-state impulse comparison against the independent f64 DFI
oracle. The test is deliberately ignored until it can be extended into the one permitted complete
four-phase invocation; no candidate subset was executed or combined with later evidence.

Checkpoint gates: `cargo fmt --check -p miso-engine-dsp-reference` PASS;
`cargo test --locked -p miso-engine-dsp-reference --no-run` PASS;
`cargo clippy --locked -p miso-engine-dsp-reference --all-targets -- -D warnings` PASS.
Matrix invocations: 0. Timed benchmark invocations: 0. No production or Cargo-manifest change.
Terra verdict: PARTIAL — compile-green phase-one harness only; no numerical selection result.

## Terra attempt 1 — retained-operation checkpoint (partial)

Extended the same ignored reference-only test with the fixed retained L1 (6+4 words), D2 (7+8
double-single words), and B3 (9+2 words) operation/state graphs; D2 defines the prescribed
`TwoSum`, `QuickTwoSum`, split-4097 Dekker product, expansion arithmetic/division, split guards,
non-overlap recovery, and committed-boundary canonicalization. The frozen phase-2 2,048-probe and
1,104-characteristic-search loop plus the phase-3 48 finite-window impulse/DFT and phase-4 48
million-sample orchestration are now present behind the one ignored test. Static W4/W8 field-major
storage assertions are 160/320 bytes L1, 240/480 bytes D2, and 176/352 bytes B3 respectively.

No phase was executed: `matrix_invocations=0`; `timed_benchmark_invocations=0`. Checkpoint gates:
`cargo fmt --check -p miso-engine-dsp-reference` PASS;
`cargo test --locked -p miso-engine-dsp-reference --no-run` PASS;
`cargo clippy --locked -p miso-engine-dsp-reference --all-targets -- -D warnings` PASS;
`git diff --check` PASS. No production or Cargo-manifest change. Terra verdict: PARTIAL —
compile-green orchestration only, with no candidate result or selection.

## Terra attempt 1 — sole matrix invocation (FAIL: transcript incomplete)

The ignored comparison was invoked exactly once with
`cargo test --locked -p miso-engine-dsp-reference issue_045_complete_recurrence_comparison_requires_sol_freeze -- --ignored --nocapture`.
No retry, candidate subset, tuning, benchmark, or production change occurred.

The preserved phase-1 transcript is exact:

- L1: `rows=1488`, `map_failures=0`, `impulse_failures=1953`, `row_rejections=0`,
  `worst_map=1.77635683940025046e-15`, `worst_impulse=1.56703382908629507e-12`,
  `hash=2a99d118ccb699c2`, `survives=false`.
- D2: `rows=1488`, `map_failures=0`, `impulse_failures=0`, `row_rejections=0`,
  `worst_map=1.77635683940025046e-15`, `worst_impulse=1.67157965315372124e-13`,
  `hash=23b53bc750761e59`, `survives=true`.
- B3: `rows=1488`, `map_failures=4`, `impulse_failures=11893`, `row_rejections=261`,
  `worst_map=1.26654242649237858e-12`, `worst_impulse=3.14479735655237569e-12`,
  `hash=c8c4f3af96312f2c`, first rejection row `35`,
  `b3_nonpositive_cholesky`, `survives=false`.

The detached native test process exited after continuing D2's retained phases, but the environment
did not retain its final stdout record. Therefore the required D2 phase-2/3/4 counts, first failure,
maxima, hashes, and selection result are unavailable and are **not fabricated here**. Since the
frozen proof requires one canonical complete transcript and permits no Terra rerun,
`matrix_invocations=1`, `timed_benchmark_invocations=0`, and Terra verdict is **FAIL**: evidence
capture/harness invocation defect requiring Sol review before any correction or second matrix run.

## Sol attempt 2 — bounded harness correction and final-run authorization

**REVIEW PASS / EXACTLY ONE COMPLETE MATRIX INVOCATION AUTHORIZED; no numerical result yet.** The
missing Terra transcript is an invocation/output-lifecycle defect eligible for the single bounded
Sol correction. The preserved phase-1 values are coherent with the frozen gate: L1 and B3 fail
before retained evaluation, while D2 legitimately survives all 1,488 mappings with zero mapping,
impulse or rejection failures and worst f64 impulse error `1.67157965315372124e-13`.

Static review corrected only proof-harness defects. The 1,104 characteristic checks now use the
unchanged 96-step crossing/log-extremum procedures rather than a coarse scan seeded at the requested
frequency. D2's coefficient-only `scale/q1/q2` use the frozen noncontracting f32 graph consistently
in both its retained analytic transfer and double-single recurrence; audio/state operations remain
double-single, with no candidate/word/domain/tolerance change. Low-word addition now preserves both
component sums. First-failure values record actual f32 bits. The harness now records and `sync_all`s
a create-new deterministic transcript after its header/layout and each completed phase, validates
the persisted bytes at completion, hashes the frozen configuration, and applies the frozen
non-timing final selection rank.

Static gates PASS without executing the ignored matrix:

- `cargo fmt --check -p miso-engine-dsp-reference`
- `cargo test --locked -p miso-engine-dsp-reference --no-run`
- `cargo clippy --locked -p miso-engine-dsp-reference --all-targets -- -D warnings`

The final operator must first prove both output paths absent, enable `pipefail`, and invoke exactly:

```text
test ! -e /tmp/engine-v2-issue-045-sol2-transcript.txt
test ! -e /tmp/engine-v2-issue-045-sol2-stdout.txt
set -o pipefail
MISO_ISSUE_045_TRANSCRIPT=/tmp/engine-v2-issue-045-sol2-transcript.txt cargo test --locked -p miso-engine-dsp-reference parametric_eq_recurrence_proof::issue_045_complete_recurrence_comparison_requires_sol_freeze -- --ignored --exact --nocapture 2>&1 | tee /tmp/engine-v2-issue-045-sol2-stdout.txt
rg -n '^issue-045 complete=true$' /tmp/engine-v2-issue-045-sol2-transcript.txt
sha256sum /tmp/engine-v2-issue-045-sol2-transcript.txt /tmp/engine-v2-issue-045-sol2-stdout.txt
```

Only the `cargo test` line is the one complete matrix invocation. Do not retry it for a candidate
failure, interrupted process, missing completion marker or post-run evidence problem: any such
outcome is final Issue-045 STOP. Current counts remain `matrix_invocations=1` and
`timed_benchmark_invocations=0`.
