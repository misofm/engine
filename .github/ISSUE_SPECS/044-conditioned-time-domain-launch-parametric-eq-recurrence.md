# 044 Conditioned time-domain launch parametric EQ recurrence

## Outcome

Replace the stopped Issue-042 direct-history delta recurrence with one bounded `f32` runtime
realization that preserves the accepted launch EQ transfer and passes every frozen time-domain gate
without recovery.

## Context

Issue **Numerically conditioned launch parametric EQ realization** stopped after its two attempts.
Its endpoint-conditioned seven-word transfer passes the complete 1,488-row analytic grid and 1,104
frequency searches, and its descriptor, automation, state transaction, bank storage, graph seam,
realtime audit and target dispatch checkpoints remain technical input only. The first legal
one-second impulse row nevertheless triggers production recovery at 44.1 kHz for a 10 Hz,
-24 dB, Q=0.1 bell. The retained transfer is acceptable; its frozen direct-history `f32` recurrence
is not.

This stateless issue has exactly two total attempts: one Terra implementation/review attempt and,
if needed, one bounded Sol correction/review. A second failure stops. Before production changes,
Sol must select and freeze one time-domain realization from a complete test-only comparison.
Benchmark and timing commands are forbidden; `timed_benchmark_invocations=0` is invariant.

## Scope

- Preserve the Issue-042 four-section dual-mono public surface, 24 parameter IDs, domains, exact
  64-update automation, zero latency, Infinite tail and four launch rates.
- Treat Issue-042's `9ae58ca1fca97d4f` endpoint-conditioned transfer only as design input.
- Compare bounded `f32` recurrence/state-conditioning variants for that transfer, including
  explicit state scaling or reparameterization and a principled underflow-to-positive-zero policy.
- Select only a variant that passes all 48 one-second impulse/DFT cases and all 48 million-sample
  valid sequences with normal-or-zero state/output, zero recovery and scalar/SIMD feasibility.
- Freeze exact retained words, state, scaling, operation/FMA graph, underflow telemetry,
  identity/bypass/reset/restore and anchor-switch behavior before production implementation.
- Implement the selected scalar and exact four/eight-lane bank paths, then rerun the frozen
  analytic/frequency/design/automation/state/isolation gates and the existing graph/audit seams.

## Required public interfaces/contracts

Retain effect ID `miso.parametric-eq`, contract 1.0, Normal quality, DualMono main input/output,
exactly four sections, the six filter kinds, 10–20,000 Hz, gain -24..24 dB, Q 0.1..18 and shelf
slope 0.1..1. No failed Issue-042 runtime-state bytes are published compatibility. The selected V1
state payload and scalar/four/eight-lane layouts must be fixed, bounded, exactly accounted and
restored all-or-none.

Every runtime lane remains `f32`. Base scalar/Wasm/NEON/AVX2 paths use one frozen noncontracting
operation graph; any AVX2+FMA contraction must be separately enumerated. Valid input cannot
increment recovery. A frozen underflow-to-zero rule must distinguish expected decay from invalid
nonfinite/state failure without hiding instability or weakening telemetry.

## Deliverables

- test-only complete recurrence/state-conditioning comparison and Sol selection amendment;
- selected scalar and homogeneous four/eight-lane implementation with frozen state/accounting;
- restored 48 impulse/DFT, 48 million-sample, 10,000-design and complete analytic/search gates;
- direct automation/reset/restore/recovery/isolation and scalar/SIMD differential evidence; and
- focused graph/audit/target regression evidence with `timed_benchmark_invocations=0`.

## Explicit non-goals

Changing the public EQ domain or response tolerances; f64 production lanes; treating recovery as a
valid decay event; graph/control automation delivery; new filter kinds; dynamic EQ; linear phase;
extended rates; sidechains; listening; performance tuning; or any benchmark.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral
- Numerically conditioned launch parametric EQ realization

The stopped Issue-042 dependency means only its explicitly accepted transfer/design and product-
surface checkpoints, not an overall PASS.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 only after the matching remote issue exists and Sol has amended the
tracked brief with one selected recurrence.** The tracked brief is
`.github/ISSUE_SPECS/BRIEFS/044-conditioned-time-domain-launch-parametric-eq-recurrence.md`.

## Hazards/decisions

Analytic transfer agreement does not prove a finite `f32` recurrence. Do not flush every small
value indiscriminately, suppress recovery counters, shorten the frozen sequences, scale only the
oracle, retain hidden higher precision, or select a formula from one repaired row. State scaling,
anchor changes and restore bytes must be proved together.

## Acceptance gates with objective measurements

1. Before production edits, each candidate runs all 48 frozen one-second impulse/DFT cases and 48
   frozen million-sample sequences. It records worst output/state magnitude, minimum nonzero
   magnitude, underflow events, recovery events, DFT error and a deterministic transcript hash.
2. A selectable candidate has zero recovery, finite normal-or-positive-zero state/output,
   <=0.05 dB impulse/DFT error where the reference is >=-120 dB, exact dry identity, bounded state
   and a realizable scalar/four/eight-lane `f32` layout. No case or tolerance waiver exists.
3. Sol freezes the selected equations, retained/state words, scale and underflow policy, exact
   operation/FMA graph, payload bytes and reset/restore/identity/anchor-switch invariants before
   production changes. If no candidate passes, stop.
4. Production passes the complete 1,488-row analytic grid, 1,104 searches, exactly 10,000 legal
   designs with seed `0x000000000012e911`, all 96 time-domain sequences, exact automation and
   state/recovery/isolation gates.
5. Scalar/four/eight/FMA paths satisfy exact or frozen differential bounds; W4/W8 storage and caps
   are exact. The existing nine-track graph and 100,000-render audit regressions pass unchanged.
6. Focused/full locked tests, warning-denied Clippy/rustdoc, policies and named native/x86/AArch64/
   Wasm compile/instruction gates pass. No benchmark command or artifact exists.

## Target matrix

Native scalar; x86 AVX2 and separately gated AVX2+FMA; AArch64 NEON four-lane; Wasm scalar and
`simd128` four-lane. Cross targets provide compile/instruction evidence unless separately run.

## Required evidence

Issue-042 failure reproducer; candidate equations and complete 96-sequence comparison/hash; Sol
selection; exact state/layout/accounting tables; analytic/search/seeded/time-domain maxima;
automation/reset/restore/isolation results; graph/audit/target regressions; Terra and final Sol
verdicts; and `timed_benchmark_invocations=0`.

## Terra attempt 1 — complete time-domain comparison (2026-08-21)

**FAIL; no recurrence selected and no production file changed.** The test-only reference boundary
ran each of three candidates over all 48 one-second impulse/DFT rows and all 48 million-sample
valid sequences. Every candidate completed with normal-or-positive-zero retained output/state
after its declared boundary policy, but none satisfied the complete frozen selection gate.

| Candidate | Selectable | Underflow / recovery events | Worst DFT error | Transcript |
| --- | --- | ---: | ---: | --- |
| 2^24-scaled direct histories | no | 24 / 24 | 0.536679696321 dB | `93b9b4aeac0fea29` |
| transposed two-state | no | 24 / 24 | 1.094058507582 dB | `80c0b5f4ab2bda57` |
| direct histories with finite-subnormal -> positive zero | no | 738 / 0 | 0.536679696321 dB | `0a6a7cb49811030b` |

The explicit flush candidate removes recovery but still has eight impulse/DFT failures above the
unchanged 0.05 dB limit. The comparison's final exactly-one-selectable assertion therefore fails
as intended. It is retained and ignored pending the one bounded Sol correction. No production,
graph, audit, target or benchmark work was performed; `timed_benchmark_invocations=0`.

## Sol correction attempt 2 — final verdict (2026-08-21)

**FAIL / STOPPED; no recurrence selected and no production change is permitted.** Adversarial
review found that attempt 1 compared each candidate's finite one-second impulse DFT with the
infinite-duration analytic magnitude. The shared `0.536679696321 dB` result therefore included
window-truncation error and was not a valid recurrence differential. The sole bounded correction
now generates an independent `f64` one-second impulse with `ReferenceParametricEqSection` and
applies the identical finite window, sample count and DFT probe used for the `f32` candidate.

The one authorized complete rerun still selected no candidate:

| Candidate | Selectable | Underflow / recovery events | Invalid values | DFT failures / worst error | Transcript |
| --- | --- | ---: | ---: | ---: | --- |
| 2^24-scaled direct histories | no | 24 / 24 | 0 | 11 / `38.341681759850 dB` | `6fd9c4f9898458e2` |
| transposed two-state | no | 24 / 24 | 0 | 10 / `42.832790236097 dB` | `b3501a5d71222b30` |
| direct histories with finite-subnormal -> positive zero | no | 738 / 0 | 0 | 11 / `38.341681759850 dB` | `52a2c5651dd9d4c4` |

All candidates completed all 48 impulse cases and all 48 million-sample cases. The flush candidate
alone retained zero recovery and zero invalid values, but it materially misses the unchanged
`0.05 dB` finite-window reference gate. The comparison remains ignored with its exactly-one
assertion intact as stopped failure evidence. The two-attempt budget is exhausted: do not add a
candidate, weaken the tolerance/domain/recovery rules, or begin production. No benchmark or
timing command was run; `timed_benchmark_invocations=0`.

Issue 045, **Launch parametric EQ recurrence derivation and runtime proof**, owns any further
research. Issue 044 authorizes no production continuation.
