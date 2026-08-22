# Sol brief — Issue 031 portable higher-precision builtin filter quality mode

## Decision boundary

Issue 031 is a reference-only, non-timed adoption decision. It does not implement or expose a
quality mode. Compare the accepted retained-`f32` incremental TPT with exactly one preregistered
`RetainedF64IncrementalV1` candidate. A candidate PASS selects work for a new stateless production
successor; a numerical/materiality/cost failure closes this issue **NO ADOPTION** with launch `f32`
unchanged.

Exactly two attempts are available: Terra runs the frozen comparison once; Sol may correct one
bounded oracle/harness defect and rerun the whole comparison once. A genuine candidate failure is
final evidence, not a correction opportunity. Matrix and timed benchmark invocation counts start
at zero; timing is forbidden.

## Frozen arithmetic

For accepted `f32` cutoff `f` and launch rate `Fs`, candidate preparation in `f64` is:

```text
g  = tan(pi * f / Fs)
k  = sqrt(2)
a1 = 1 / (1 + g * (g + k))
a2 = g * a1
a3 = g * a2
c1 = 1 - a1
```

Retain `(c1,a2,a3,k,s1,s2)` as six `f64` words. For each input promoted once from normalized
`f32`, execute without contraction and in this exact order:

```text
v3 = input - s2
p1 = a2 * v3
p2 = c1 * s1
d1 = p1 - p2
v1 = s1 + d1
p3 = a2 * s1
p4 = a3 * v3
d2 = p3 + p4
v2 = s2 + d2
q1 = d1 + d1
n1 = s1 + q1
q2 = d2 + d2
n2 = s2 + q2
low  = v2
kh = k * v1
th = input - kh
high = th - v2
s1 = n1; s2 = n2
```

Round only the selected output to `f32`. At committed boundaries, state below
`f32::MIN_POSITIVE` in magnitude and `f32` subnormal/negative-zero output become positive zero.
Nonfinite input/state follows the accepted lane-local sanitation/recovery action and reporting.
The baseline calls the independent retained-`f32` reference graph, not production code. The oracle
is an algebraically independent `f64` RBJ/direct-form realization over the exact promoted `f32`
input.

Candidate payload is exactly 48 bytes/section, including 16 mutable state bytes, versus baseline
24/8. W4 is two `f64x2` groups and W8 two `f64x4` groups; the static ceiling is two candidate
vector operations per accepted baseline vector operation. There is no hidden scratch, FMA, scalar
lane, changed latency or changed tail.

## Complete comparison

Use the issue body's exact 64 configurations, 296 deduplicated probes, one-second impulse/DFT,
sustained-sine, signed DC, fixed-seed noise, compact fault/isolation sequence and five partitions.
Hash the ordered grid, equation/version tag, all inputs, baseline/candidate/oracle outputs, final
states and reports. Finite-window DFT expected values come from the same-length oracle impulse,
never the infinite analytic response.

Selection requires all six issue-body gates: `1e-12` transfer equivalence, `1e-9 dB` analytic,
`0.005 dB` impulse, a cross-rate/type baseline materiality set worse than `-120 dB`, at least
`6 dB` per-limited-row and `12 dB` global improvement with candidate worst `<=-126 dB`, no stated
regression, exact semantic/partition hashes and the 2x static cost ceiling. No threshold may be
changed after output inspection.

## Files and execution order

Allowed implementation/evidence surface:

- one new Issue-031 module/test and its declaration under
  `crates/miso-engine-dsp-reference/src/`;
- `.github/ISSUE_SPECS/031-portable-higher-precision-builtin-filter-quality-mode.md`; and
- one result line in `dsp-research/filters.md` after the decision.

Order:

1. implement candidate, independent oracle, deterministic schema and small non-matrix unit tests;
2. pass format, focused compile/no-run, focused unit tests and warning-denied focused Clippy;
3. seal clean production/source/grid identities;
4. invoke the complete ignored matrix exactly once and persist its canonical transcript;
5. record **SELECTED FOR SEPARATE IMPLEMENTATION**, **NO ADOPTION**, or **STOPPED** without
   changing the candidate or thresholds.

Production builtins/core/session/graph, manifests, target scripts, benchmark tools and accepted
fixtures are forbidden. `timed_benchmark_invocations=0` throughout.

## Stop conditions

Stop for a second candidate family; changed cutoff/rate/transfer/recurrence/FMA/recovery semantics;
production or metadata integration; oracle reuse of the candidate/production graph; analytic
substitution for finite-window DFT; target/timing/listening work; skipped rows; threshold tuning;
or a third attempt.
