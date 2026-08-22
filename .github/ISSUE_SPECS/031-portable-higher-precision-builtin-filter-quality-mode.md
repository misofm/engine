# 031 Portable higher-precision builtin filter quality mode

## Status and outcome

**FINAL — NO ADOPTION.** This evaluation determined whether one portable retained-`f64`
variant produces a preregistered material numerical improvement over the accepted launch `f32`
HPF/LPF recurrence at acceptable static storage/SIMD cost. If it qualifies, close this issue with a
selection record and create a separate stateless implementation successor. If it does not qualify,
close **NO ADOPTION** with launch `f32` unchanged.

This issue permits exactly two total attempts: one Terra execution/review and, only for a bounded
harness/oracle defect, one Sol correction and one complete rerun. A genuine candidate failure is a
valid no-adoption result, not permission to tune it. `matrix_invocations=0` and
`timed_benchmark_invocations=0` at briefing.

## Accepted baseline and context

The baseline is the Issue-036 accepted launch domain and conditioned two-state TPT realization:
coefficients `(c1,a2,a3,k)` and states `(s1,s2)` are retained as `f32`; scalar, base W4/W8 and the
separately gated AVX2+FMA backend preserve the Issue-037 operation contract. Issue 068 accepted the
native/AArch64/Wasm backend-selection and instruction matrix. Issue 070 is upstream qualification
context, not a direct dependency of this numerical decision. Stopped Issues 007 and 008 are
historical technical input only and are not PASS dependencies.

There is no current builtin-quality field in session metadata or the builtin bank signature.
Therefore this issue must not add one. A selected candidate is only eligible for a successor that
separately freezes public metadata, prepared layout, serialization, resources, scalar/W4/W8
lowering, target evidence and migration behavior.

Never inspect, copy, benchmark against or inherit V1/legacy work.

## Frozen candidate

Compare the accepted baseline with exactly one candidate, `RetainedF64IncrementalV1`:

- accept only the unchanged Issue-036 `f32` cutoff/rate domain;
- compute and retain `(c1,a2,a3,k)` as `f64` from that exact cutoff value and retain `(s1,s2)` as
  `f64`;
- promote each normalized finite `f32` input once, execute the exact accepted incremental TPT
  temporary order in nonfused `f64`, select low/high output, then round once to `f32`;
- use no FMA, extended accumulator, per-row tuning, hidden scalar correction or changed transfer;
- canonicalize committed state whose magnitude is below `f32::MIN_POSITIVE` and the final `f32`
  output if subnormal or negative zero; preserve accepted input sanitation and lane-local invalid
  recovery semantics; and
- project W4 as two `f64x2` vectors and W8 as two `f64x4` vectors. This is a static feasibility
  projection only; no target kernel is implemented here.

The semantic payload is six retained words per section: 48 bytes for the candidate versus 24
bytes for the baseline; serialized mutable state is 16 versus 8 bytes. Selection requires no new
semantic words, scratch, latency or tail, and at most two vector operations for each accepted
nonfused vector operation. Paired-`f32`, double-single, compensated and other wider candidates are
not evaluated; adding another family would require a new issue and derivation.

## One bounded comparison

The complete matrix uses only launch rates `44_100,48_000,88_200,96_000`. For both HPF and LPF,
each rate uses eight exact cutoffs: `10`, `20`, `100`, `1_000`, `min(20_000,0.1*Fs)`, `0.45*Fs`,
the immediate `f32` predecessor of the Issue-036 maximum and that exact maximum. This is 64 filter
configurations. Each uses the accepted probe construction
`{0.25*f,f,4*f,0.2*Fs,0.45*Fs}`, clamped to `[4,Nyquist-4]`, rounded to the nearest 4 Hz and
deduplicated, yielding exactly 296 sustained/DFT probe rows.

For baseline, candidate and an algebraically independent `f64` RBJ/direct-form oracle, execute:

1. analytic transfer/cutoff comparison for all 296 rows;
2. one-second zero-state impulse renders and finite-window rectangular DFTs for all 64
   configurations, comparing like-duration DFTs rather than an infinite analytic response;
3. accepted sustained `0.5`-amplitude coherent sine measurement (`Fs/2` settle, `Fs/4` measure)
   for all 296 rows; and
4. 65,536-sample `+0.5` DC, `-0.5` DC and SplitMix64 bipolar-noise sequences for all 64
   configurations, seed `0x0000000000000310`, plus one compact signed-zero/subnormal/nonfinite
   lane-isolation sequence for each rate/filter kind.

Partition the time-domain rows by `1,127,128,255,1024` and require each realization to reproduce
its own sample/state/report bits across partitions. The independent oracle receives the exact
finite `f32` input promoted to `f64`; it must not call production builtins or reuse the candidate
recurrence.

## Adoption gates

The candidate qualifies only if every gate passes:

1. Its unrounded transfer matches the independent RBJ transfer within `1e-12` absolute per
   normalized coefficient; analytic magnitude/cutoff error is `<=1e-9 dB` where the reference is
   `>=-120 dB`.
2. Every finite-window impulse DFT differs from the same-window oracle by `<=0.005 dB` where the
   oracle is `>=-120 dB`; legal sequences remain finite and bounded with zero invalid recovery.
3. The baseline demonstrates a material precision limit: at least eight sustained/DC/noise rows,
   spanning all four rates and both filter kinds, have oracle-normalized residual worse than
   `-120 dB`.
4. On every such row the candidate improves residual by `>=6 dB`, improves the global worst row
   by `>=12 dB`, and its global worst residual is `<=-126 dB`. No matrix row may regress by more
   than `0.25 dB` residual or `0.0001 dB` gain.
5. Reset, sanitation, recovery, signed-zero and lane-isolation reports match the accepted semantic
   actions; repeated runs and all five partitions produce identical hashes.
6. The exact 2x retained-byte and vector-operation ceilings above hold, with zero added scratch,
   latency or tail. No wall time, cycles or benchmark result participates in selection.

If all gates pass, record **SELECTED FOR SEPARATE IMPLEMENTATION**; that is not production
acceptance. If any numerical, materiality, portability or cost gate fails, record **NO ADOPTION**.
If the oracle/harness cannot be corrected within the two-attempt budget, record **STOPPED** without
an adoption decision.

## Deliverables and evidence

- one V2 reference-only candidate/oracle harness and one canonical complete-matrix transcript;
- candidate/baseline/oracle equations, case counts, first failures, worst rows, recovery and
  canonicalization counts, exact payload/resource projection and deterministic per-phase hashes;
- source/grid/equation/seed/transcript SHA-256 identities and before/after proof that production,
  session, runtime and graph sources are unchanged; and
- Terra/Sol verdict, `matrix_invocations` and `timed_benchmark_invocations=0`.

## Explicit non-goals

Production DSP/core/session/metadata/resource changes; adding a quality mode; changing the launch
domain, coefficient family, baseline recurrence, FMA sites, latency, tail or recovery policy;
additional candidate families; target builds or object inspection already accepted by Issue 068;
listening; benchmarking/timing; performance claims; or a production implementation successor
before this decision is recorded.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Representable TPT cutoff domain and builtin contract acceptance
- Production SIMD builtin bank graph retention and reachability qualification
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Terra execution surface

Terra may change only a new Issue-031 module/test inside `miso-engine-dsp-reference`, its module
declaration, this issue's evidence and one concise decision line in `dsp-research/filters.md` after
the run. No Cargo dependency is expected. Compile/no-run, format, focused reference tests and
warning-denied focused Clippy must pass before the one complete matrix invocation is authorized.

## Sol pre-matrix correction checkpoint (2026-08-22)

Sol attempt 2 found and corrected one bounded decision-harness defect class without changing the
candidate, oracle, grid, thresholds or arithmetic. The candidate-only `0.005 dB` impulse gate had
also rejected the accepted baseline; baseline impulse error is now retained in the deterministic
hash but cannot veto the candidate gate. Legal impulse/sustained/DC/noise renders now explicitly
reject nonfinite output/state, unexpected input/output sanitation or invalid recovery. Each of the
eight rate/filter semantic rows now proves one injected candidate-state recovery, exact reset
continuation and unchanged control instance, and the transcript requires exactly eight recoveries.
The canonical begin record identifies `attempt=2`, `matrix_invocations=1` and
`timed_benchmark_invocations=0` for the still-unrun authorized comparison.

Non-matrix evidence on the unchanged correction:

- `cargo fmt --all -- --check`: PASS;
- focused Issue-031 tests: PASS, 5 passed;
- full `miso-engine-dsp-reference` library tests: PASS, 12 passed and the three complete matrices
  remained ignored; and
- warning-denied package all-target Clippy: PASS.

The complete Issue-031 comparison was not invoked. Counts remain `matrix_invocations=0` and
`timed_benchmark_invocations=0`. This is a green correction checkpoint, not final numerical
evidence and not authorization to run from a dirty candidate.

## Final Sol evidence and verdict (2026-08-22)

The sole authorized complete comparison ran once on clean candidate
`cf611ef48f43df9db7422762e9f90006936b37af` and persisted `decision=NO_ADOPTION` followed by
`complete=true`. It was not retried. The corrected reference module SHA-256 is
`887be248efe2c23175d7b026dcd3fdedd887591656a0e603a3ecf31f9ce53e7e`; the 10-line,
2,803-byte transcript SHA-256 is
`ca1b5177869f36b20a63d5f535e17e995c30816855dfec5a61db1c4db922472f`; and the 18-line,
3,193-byte captured test output SHA-256 is
`64b18be488f607ebf195f0c383a7a0ced807cc6e443a11b89019090f303ecc2a`.

The frozen grid and lifecycle were exact: equation `493033315f463634`, seed
`0000000000000310`, grid `b2a2d521a519e55a`, 64 configurations, 296 analytic/sustained rows,
64 one-second impulse configurations, 192 DC/noise rows, eight semantic rows, all five partitions,
`matrix_invocations=1` and `timed_benchmark_invocations=0`.

The candidate did not qualify:

- normalized transfer equivalence passed with worst coefficient error
  `6.66133814775093924e-16 <= 1e-12`, but 38 analytic rows failed the `1e-9 dB` gate. The first
  failure and analytic worst were the 44.1-kHz low-pass at the exact maximum-predecessor cutoff
  `0x46ac42f6` (`22,049.48046875 Hz`); worst error was
  `1.76030052756459554e-7 dB`;
- the impulse phase recorded 74 aggregate failures. The decisive candidate DFT worst was
  `1.82763560973455697e-2 dB > 0.005 dB` at 44.1 kHz, 100-Hz low-pass, 19,844-Hz probe; and
- therefore gates 1 and 2 failed independently and `pass=false` is required.

The remaining evidence does not rescue selection: materiality covered 198 rows across all four
rates and both filter kinds, with `147.665737227548988 dB` global and
`31.810227668712969 dB` minimum limited-row improvement, candidate worst residual
`-134.923113322814231 dB`, and zero regression failures. All eight semantic rows passed with eight
injected recoveries and zero legal invalid recoveries. The exact candidate projection remained
48-byte payload/16-byte state versus 24/8, W4 `f64x2x2`, W8 `f64x4x2`, 2x vector-operation
ceiling, zero scratch, zero latency and zero tail.

**Final verdict: NO ADOPTION.** The accepted retained-`f32` launch HPF/LPF remains unchanged.
Both attempts and the sole matrix authorization are consumed. No production quality mode or
implementation successor is created, and this issue may close when local/remote evidence is
synchronized.
