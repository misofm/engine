# Sol research brief — issue 045 launch parametric EQ recurrence derivation and runtime proof

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1, RESEARCH ONLY.** There are exactly two total attempts: one Terra
implementation/review of this proof and, only if needed, one bounded Sol correction to a derivation
or harness defect. Each attempt may invoke the complete matrix exactly once. A second failure
stops; a Sol correction may not add a candidate family, change a row/probe/domain/tolerance or
reuse production code. Issues 042 and 044 remain stopped evidence. Production EQ/core/graph,
Cargo manifests outside the reference crate, benchmarks and timing are forbidden.
`timed_benchmark_invocations=0`.

## Common transfer and candidate set

The independent oracle designs normalized f64 RBJ `(b0,b1,b2,a1,a2)` for the unchanged 1,488-row
Issue-042 grid. Identity rows are exact dry identity and do not exercise hidden state. Every other
row must be realized by exactly these three candidates; all design algebra is f64, then each named
retained word is rounded once to f32 in displayed order [RBJ-COOKBOOK] [SMITH-SASP]
[ORFANIDIS-ISP]. `a` is exact `+1` for `f0 <= Fs/4`, otherwise exact `-1`, and
`delta = z^-1-a`.

### L1 — endpoint numerator, normalized lattice denominator

Retain six coefficient words `(a,n0,n1,n2,k1,k2)` and four state words `(x1,x2,s0,s1)`:

```text
n0=b0+a*b1+b2; n1=b1+2*a*b2; n2=b2
k2=a2; k1=a1/(1+a2)
dx=x1-a*x; ddx=(x2-a*x1)-a*dx
e=(n0*x+n1*dx)+n2*ddx
f1=e-k2*s1; y=f1-k1*s0
s1'=k1*y+s0; s0'=y; x2'=x1; x1'=x
```

The denominator reconstructed from retained words is
`1 + k1*(1+k2) z^-1 + k2 z^-2`. Strict Jury stability is tested on that transfer, not inferred from
the f64 source. This is the fixed second-order normalized synthesis lattice; no ladder or direct
output-history variant may be substituted.

### D2 — endpoint delta with double-single f32 state/arithmetic

Retain Issue-042's seven words `(a,n0,d0,n1,d1,n2,d2)` and eight state words
`(x1_hi,x1_lo,x2_hi,x2_lo,y1_hi,y1_lo,y2_hi,y2_lo)`. Evaluate the exact Issue-042 delta temporary
order, but every add/subtract/multiply and the final division operates on a canonical `(hi,lo)`
f32 expansion; input and coefficients enter as `(word,+0)`, and output is the one f32 rounding
`hi+lo`. There is no f64 lane and no FMA.

```text
dx=x1-a*x; ddx=(x2-a*x1)-a*dx
num=(n0*x+n1*dx)+n2*ddx
scale=(d0-a*d1)+d2
q1=a*d2; q2=(d1-q1)-q1
history=q2*y1+d2*y2
y=(num-history)/scale
x2'=x1; x1'=x; y2'=y1; y1'=y
```

The proof implementation must define and unit-check these fixed error-free primitives before the
matrix [DEKKER-EXTENDED]: `TwoSum`, `QuickTwoSum`, and Dekker `TwoProd` with split constant
`4097.0_f32`; expansion add/subtract renormalizes both component sums, expansion-times-f32 uses
`TwoProd(hi,c)` plus `lo*c`, and expansion-divided-by-f32 uses `q1=hi/c`, an error-free product of
`q1*c`, `q2=(((hi-product_hi)-product_lo)+lo)/c`, then renormalizes `(q1,q2)`. Any split operand
above `f32::MAX/4097`, nonfinite intermediate or failed nonoverlap invariant is a recovery failure,
not a wider-precision escape. Scalar/W4/W8 use the same lane-wise primitive order and zero
contractions.

### B3 — deterministic Hankel-balanced real state space

Start in f64 from transposed direct-form state space:

```text
A=[[-a1,1],[-a2,0]]; B=[b1-a1*b0,b2-a2*b0]^T
C=[1,0]; D=b0; y=C*s+D*x; s'=A*s+B*x
```

Solve the two discrete Lyapunov equations for controllability `P` and observability `Q` with
deterministic partial pivoting. Cholesky-factor `P=R R^T`, eigendecompose the symmetric
`R^T Q R`, sort eigenvalues descending and give each eigenvector the sign whose first nonzero
component is positive. With eigenvectors `U` and positive Hankel singular values `sigma`, use
`T=R U diag(1/sqrt(sigma))`, then `Ab=T^-1 A T`, `Bb=T^-1 B`, `Cb=C T`, `Db=D`.
Singular/nonpositive/repeated-without-a-unique-canonical-basis cases reject B3 for that legal row.
Retain nine words row-major `(A00,A01,A10,A11,B0,B1,C0,C1,D)` and two state words `(s0,s1)`.
The exact noncontracting recurrence is:

```text
y=(D*x+C0*s0)+C1*s1
n0=(A00*s0+A01*s1)+B0*x
n1=(A10*s0+A11*s1)+B1*x
s0'=n0; s1'=n1
```

No alternate balancing heuristic, per-row search or timing-based scaling is permitted.

All candidates transpose each retained coefficient/state field directly into `[f32;4]` and
`[f32;8]`; this static layout and identical lane-local operation graph is the W4/W8 feasibility
proof. No candidate may use dynamic storage, lane communication or scalar-only hidden work.

## One complete comparison invocation

Add one ignored, explicitly named Issue-045 test in the V2 `miso-engine-dsp-reference` test
boundary. One invocation executes all phases below in this order and emits one canonical record;
do not run candidate subsets and later combine their evidence.

1. Hash the candidate enum/order, equations/version tag, all grid rows, probes, seeds and retained
   words. For each candidate and every 1,488 row, first expand its unrounded f64 mapping back to
   `(b0,b1,b2,a1,a2)` and require each absolute coefficient error `<=1e-12`. Then compare exactly
   4,096 zero-state f64 impulse samples with the independent f64 direct-form-I oracle and require
   maximum absolute sample error `<=1e-12`. A candidate with any mapping failure is recorded and
   does not enter retained-f32 phases.
2. For each survivor, run the unchanged Issue-042 retained-f32 analytic grid: 2,048 logarithmic
   10--20,000 Hz probes plus exact f0, DC and Nyquist for all 1,488 rows, and the unchanged 1,104
   cutoff/center/shelf-midpoint/notch-minimum searches. Require `<=0.005 dB` where reference is
   `>=-120 dB`, notch null `<=-100 dB`, search error `<=0.1%`, finite retained words and a strictly
   stable retained transition.
3. Run exactly the 48 Issue-044 edge rows (four launch rates, six kinds, low edge
   `10/-24/0.1/0.1`, high edge `20000/+24/18/1`). Render exactly `Fs` impulse samples from both the
   retained-f32 candidate and an independent zero-state f64 direct-form-I oracle. Compute both
   DFTs over those same `Fs` samples with the same rectangular window and exact f0 probe, converting
   candidate samples to f64 only for accumulation. Compare the two finite-window magnitudes at
   `<=0.05 dB` where the finite-window reference is `>=-120 dB`. The infinite analytic magnitude
   is never the expected value for this gate.
4. Run exactly the 48 frozen 1,000,000-sample Issue-044 sequences with seed
   `0x000000000012e911` and unchanged row-mixing/noise generation. Require finite bounded output,
   zero recovery and committed state/output that is normal or canonical positive zero.
5. At every committed boundary, a finite subnormal or negative zero is explicitly canonicalized
   to positive zero and counted; an existing positive zero remains positive zero. Canonicalization
   is not recovery. Any nonfinite value, magnitude/split guard
   failure or invalid expansion recovers and fails selection. Record underflow/canonicalization and
   recovery separately, maxima/minimum nonzero, first failure with row/sample/bits, coefficient and
   state words/bytes, and per-phase deterministic hashes.

Candidate pass requires every applicable phase with no row rejection and zero recovery. If several
pass, select deterministically by: greatest minimum retained strict-stability margin, then smallest
worst finite-window DFT error, then fewest state words, then fixed ID order `L1,D2,B3`. Numeric
comparisons use stored f64 result bits; timing is never measured or used. Amend this brief after the
single run with exactly one selected coefficient/state/operation contract. If none passes, record
final STOP. A harness/derivation defect may consume the one Sol correction and one complete rerun;
a candidate's genuine numerical failure is not permission to alter its family after inspection.

## Required transcript and stop conditions

Record invocation count, phase/case counts, all first failures, worst rows/probes, stability and
state/output bounds, storage/SIMD table, hashes, Terra/Sol verdicts, production-tree no-diff proof,
and `timed_benchmark_invocations=0`. Focused reference format/test/Clippy and diff checks are the
only surrounding gates.

Stop for production changes, skipped f64 equivalence, finite-window/analytic oracle substitution,
f64 runtime lanes, unbounded state, renamed Issue-044 recurrence, hidden flush/recovery, a changed
domain/probe/tolerance/seed, candidate search/tuning, timing, benchmark or third attempt.
