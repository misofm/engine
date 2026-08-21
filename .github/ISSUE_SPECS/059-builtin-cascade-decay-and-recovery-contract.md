# 059 Builtin cascade decay and recovery contract

## Outcome

Determine and freeze the launch builtin cascade's decay/recovery semantics, and make a bounded
product correction only if the repeated recoveries are a real defect.

## Context

Issue 056 stopped after its two attempts. Its clean benchmark-input checkpoint `3aeb39c` and failed
fixed-grid response candidate are technical input only. A legal 44.1-kHz 20-Hz single HPF impulse
reports one subnormal-state recovery per lane, but the fixed 100-Hz HPF followed by 1-kHz LPF
reports aggregate recovery totals including `34` at 44.1 kHz, exceeding the provisional one-per-
section/lane limit of `4`. It is unresolved whether the downstream section is being legitimately
re-excited near the normal/subnormal boundary or production incorrectly classifies ordinary decay
as repeated recovery.

This issue permits exactly one Terra attempt and one bounded Sol correction/review. A second
failure stops. Workload and timed benchmark invocations are forbidden and start at zero.

## Scope

For the fixed production-order 100-Hz HPF then 1-kHz LPF, capture the independent retained-`f32`
per-section/per-lane state and recovery timeline for one-second impulses at exactly 44.1, 48, 88.2
and 96 kHz. Prove partition invariance for quanta `1,127,128,255,1024` and that probe duplication
cannot affect rendering. Classify each event as finite-normal decay, subnormal canonicalization or
invalid-state recovery. Freeze one exact contract:

- repeated events are valid, with independently derived deterministic totals and reporting
  semantics; or
- repeated events are a product defect, repaired within the existing TPT topology and retained
  operation order, with ordinary decay no longer over-reporting recovery.

Any product repair must be local to builtin TPT decay/recovery and preserve response, cutoff,
latency, tail, state layout, scalar/bank parity and realtime behavior. Hand the accepted contract
and candidate identity to Issue 060.

## Required public interfaces/contracts

`BuiltinProcessReport::{recovered_left_state,recovered_right_state}` remains lane-local and
deterministic. Recovery means the frozen contract's invalid retained-state event; any separate
finite underflow canonicalization behavior must be named and proved rather than silently counted
or discarded. Reset always writes positive-zero filter state. No probe, block partition or caller
buffer layout may change the timeline or aggregate count.

## Deliverables

One checked four-rate timeline/report, independent retained-`f32` recurrence comparison, explicit
valid-versus-defect decision, any directly required bounded builtin correction, and focused
regression tests consumable by the corpus checker.

## Explicit non-goals

Completing or rewriting the fixture corpus, broad filter redesign, rate/domain/tolerance changes,
realtime audits, million-call rows, graph lifecycle qualification, target or instruction matrices,
object inspection, benchmarks, timing, or listening.

## Dependencies by exact issue title

- Complete independent builtin corpus and corruption proof
- Representable TPT cutoff domain and builtin contract acceptance
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- At all four launch rates, the independent and production per-section/per-lane event timelines and
  totals agree for the frozen cascade impulse.
- All five block partitions produce identical PCM, final state, event timeline and report totals;
  every duplicated probe row maps to that same render result.
- Every event is shown to begin from finite normal, subnormal or nonfinite state and is classified
  under the frozen public reporting contract; no unexplained repeated recovery remains.
- Any correction preserves the existing analytic and finite-window response tolerances, zero
  latency, Infinite tail, state/resource shape and scalar/bank parity with no render allocation,
  lock or I/O.
- Focused builtin/reference/fixture tests, format, warning-denied relevant-package Clippy and diff
  checks pass. No broad audit, target or benchmark command runs.

## Required evidence

Exact candidate identity; four-rate per-section/per-lane timeline and totals; partition/probe
invariance hashes; decision and any changed public semantics; focused commands/results; strict
Terra/Sol verdicts; `workload_invocations=0`; `timed_benchmark_invocations=0`.

## Terra attempt 1 evidence

The repeated cascade reports were ordinary finite-subnormal retained-state decay incorrectly
classified as invalid recovery. The bounded correction canonicalizes finite subnormal state to
positive zero without a recovery increment in scalar and bank paths; only nonfinite retained state
resets and increments `BuiltinProcessReport`. Coefficients, recurrence order, response, latency,
tail and retained layout are unchanged.

Independent retained-`f32` recurrence and production agree bit-for-bit per section/lane across all
five partitions and duplicated probe metadata. Canonicalization event samples `[HPF L/R, LPF L/R]`
and transcript hashes are: 44,100 `[[36229,36229],[36225,36225]]` / `41e00de8a16c7fbb`;
48,000 `[[39435,39435],[39433,39433]]` / `a0ff07932e1b7a8d`; 88,200
`[[385,385],[960,960]]` / `cdda7646b0504e2a`; and 96,000
`[[414,414],[1133,1133]]` / `2023c64000bb1500`. Every lane reports zero recovery.

Focused PASS: format; builtins 26/26; dsp-reference 10 pass with two unrelated ignored;
warning-denied all-target Clippy for builtins/reference/fixture; and `git diff --check`. The checked
fixture validator remains red only because the intentionally unchanged stopped-#56 CSV still
serializes the old count (`34` at the first cascade); Issue 060 owns that corpus update. No audit,
target, benchmark or timing command ran. `workload_invocations=0`;
`timed_benchmark_invocations=0`. Terra verdict: **PASS READY FOR SOL REVIEW**, with corpus seal
explicitly pending Issue 060.
