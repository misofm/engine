# 061 Complete builtin response cases and scalar PCM semantics

## Outcome

Complete the independent typed checks for the frozen builtin response cases and the 32 non-graph
PCM payloads left by stopped Issue 060.

## Context

Issue 059 accepted zero recovery for every legal response row. Stopped Issue 060 produced the
four-rate zero-recovery response candidate and partial checker at clean checkpoint `10f0235`, but
Sol found unparsed response-case fields, skipped response gates, unchecked scalar PCM and an
incomplete reset fixture. Those bytes and findings are technical input, not acceptance.

This stateless issue permits exactly one Terra attempt and one bounded Sol correction/review. A
second failure stops. Launch rates remain exactly 44,100, 48,000, 88,200 and 96,000 Hz. Workload,
timing and benchmark invocations are forbidden and remain zero.

## Scope

Close only response/case parsing and scalar PCM semantics. Preserve the frozen 1,630-row grid,
1,652 case declarations, numerical tolerances, 50-payload corpus shape and production DSP. Validate
the 32 existing non-graph PCM paths, including the unsuffixed matrix-ramp payload and an executed
fixture sequence covering both builtin reset kinds. Do not edit graph taps or JSONL payloads.

## Required public interfaces/contracts

The read-only checker parses every response-case field and requires the exact canonical tuple
encoded by its ID. Every serialized response measurement is checked against the independent
oracle and the frozen gate applicable to that probe; partitions of one coordinate have identical
measurements and recovery. Scalar PCM expectations use closed-form arithmetic or the accepted
independent retained-`f32` reference and never production regeneration on the check path.

## Deliverables

Typed response-case/CSV checking, complete independent scalar PCM checking, corrected reset
fixture bytes if required, focused mutations for each repaired semantic hole and strict evidence.

## Explicit non-goals

Production DSP, graph/PDC fixtures, meter/diagnostic/resource JSONL, final 24-format corruption
seal, audits, targets, instruction inspection, benchmarks, timing or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Issue-007 builtin qualification tooling, audits, and benchmark
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- Exactly 1,630 response rows and 1,652 cases retain the frozen four rates, five quanta,
  section/cutoff/probe grid and zero recovery.
- Every response case has exactly its canonical ID/category/rate/quantum/section/cutoff/probe/oracle
  tuple; every CSV numeric field has the frozen canonical decimal representation.
- Analytic, cast-state, one-second impulse and final-4096 tail gates run wherever frozen; coherent
  rows additionally run the sustained fundamental/residual/total gates. All five partitions agree
  exactly for one render coordinate.
- All 32 non-graph PCM paths are independently checked. The unsuffixed ramp is no longer
  manifest-only, and the reset fixture executes and proves both `DiscontinuityKeepTargets` and
  `FullToPrepared` without adding a payload.
- Focused fixture/reference tests, format, warning-denied package Clippy and diff checks pass.

## Required evidence

Exact row/case/PCM counts and hashes; tolerance maxima; partition equality and recovery total zero;
mutation identities; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.

## Final Sol correction and evidence — 2026-08-21

**PASS.** Adversarial review found and corrected one bounded checker defect: the independent
retained-`f32` recurrence had replaced, rather than supplemented, the frozen analytic quality
gate. The final checker requires both the serialized production measurement and the independent
recurrence measurement to satisfy the RBJ-relative impulse/fundamental tolerances, while also
checking their mutual agreement. Noncoherent exact-cutoff and `0.49*rate` rows run impulse and
tail gates; only the frozen base-probe rows run sustained gates.

The sealed corpus has `1,652` cases and `1,630` response rows (`735` HPF, `735` LPF, `160`
cascade), exactly five quanta per invariant coordinate, all seven serialized `f64` measurement
words identical across those quanta, and total recovery `0`. Observed frozen maxima are
`0.000001751253 dB` cast-state error, `0.024970453294 dB` one-second impulse error and
`0.000010218267 dB` coherent fundamental error; worst coherent residual is
`-124.325462463 dB`, and the largest deep-stop total is `-91.410151117 dB`.

Every response case is an exact eight-field tuple and every CSV integer/17-place decimal token is
canonical. All `32` non-graph PCM payloads are independently checked (`33` total including graph
PCM). The unsuffixed ramp is explicitly the prepared 64-byte swap case. The single 96-byte reset
payload contains three fresh four-frame impulse responses and the executed author path records
`DiscontinuityKeepTargets` followed by `FullToPrepared`. Focused tests reject an altered response
oracle, noncanonical decimal, partition measurement, unsuffixed-ramp word and reset-segment word.

Payload identities are `cases.toml`
`3f097580addf28280cf0c2aa3709610974e0a92d4ad00ea7267e5359a9ac7091`, response CSV
`c2173a06aa9c2f37c7966d576f7d34dde349e05633941d9e8e4eb6d888fbf53d`, and reset PCM
`76795b3b6044cdde0fcc3662f26c927544b8d3f9f676ec1d706212e7da40b7f9`.

Executed gates:

- checked-in read-only fixture validation: PASS;
- `cargo test --locked -p miso-engine-builtins-fixture`: PASS, `8/8`;
- `cargo test --locked -p miso-engine-dsp-reference`: PASS, `7` unit plus `3` integration tests,
  with two separately frozen EQ matrix tests ignored;
- warning-denied all-target Clippy for both focused packages: PASS;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

Verdict is strict PASS for Issue 061 only. Issues 062 and 064 remain out of scope.
`workload_invocations=0`; `timed_benchmark_invocations=0`.
