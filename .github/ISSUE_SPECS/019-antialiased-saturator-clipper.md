# 019 Launch fixed-2x cubic soft-clip saturator

## Outcome

Ship one honest launch nonlinear processor: a Normal-only, fixed-2x, linear-phase FIR oversampled
cubic soft clip with dual-mono scalar/W4/W8 execution, scalar tails and one registry-to-graph
vertical. The advertised antialiasing claim is limited to the frozen measured comparison below.

## Context

Engine V2 is greenfield; never inspect or inherit V1. Render owns a prepared immutable-shape plan
and performs no allocation/free, locks, I/O, logging, syscalls, feature detection or filter design.
Launch rates are exactly 44.1/48/88.2/96 kHz. This issue consumes the accepted effect runtime,
AoSoA bank and graph/PDC seams and permits exactly two attempts: Terra plus one Sol correction.

## Scope

- Add Normal-quality `miso.soft-clip`, contract/state layout 1, at all launch rates.
- Use exactly one `CubicSoftClip` mode and one private fixed-2x 63-tap FIR pipeline per lane. This is
  effect-local, not a reusable oversampling framework.
- Expose per-lane drive, output and wet mix; DualMono only; main input/output only; no sidechain.
- Implement scalar, packed W4/W8 homogeneous banks, scalar tails, launch registry/effect compiler,
  and a representative ten-track graph/PDC/cap fixture.

## Required public interfaces/contracts

The exact equations, coefficient bits/phase, update order, state layout, resources and gates are
authoritative in `BRIEFS/019-antialiased-saturator-clipper.md`. In summary, sanitize `x`, apply the
smoothed drive, write `2*gd*x` followed by zero and convolve with `h` (the effective interpolation
response is `2h`), apply
`c(u)=u-u^3/3` for `|u|<1` and `copysign(2/3,u)` otherwise, filter with `h`, retain the even phase,
then mix with dry delayed by 31 samples and apply output gain. Report latency 31 and
`TailSamples::Finite(31)`; the total causal response can extend through base sample 62.

Stable ordered PerLane parameters are: `1 drive` dB `[-24,36]` default `0`; `2 output` dB
`[-24,24]` default `0`; `3 mix` Linear `[0,1]` default `1`. All accept canonical ordered Block
Points and ramp in linear gain/mix space for exactly 64 updates. Preparation fixes Normal quality,
rate, quantum, bypass and DualMono. Quality/mode changes require replacement preparation.

Each lane is exactly 169 little-endian words: two cursors, three `(current,target,remaining)` ramps,
63 interpolation history words, 63 decimation history words and a 32-word dry ring. State is 676
bytes/lane and 1,352 bytes/track; retained reset defaults are exactly 24 scratch bytes/track with no
per-frame scratch. Bank retained bytes are `W*(1352+24)`: W4 5,504 and W8 11,008. All arithmetic,
allocation and cap checks are checked and transactional.

Bypass warms the complete wet pipeline and selects the 31-sample delayed dry signal. Mix zero with
unity output selects delayed dry bits exactly. `FullToDefaults` clears histories/cursors and restores
prepared values; `DiscontinuityKeepParameters` clears histories/cursors and snaps ramps to retained
targets. Snapshot/restore is exact, lane-complete, domain-closed and atomic. Nonfinite/subnormal input
becomes positive zero and is counted; finite signed zero is preserved. Nonfinite state/output resets
only that lane, emits delayed dry and counts one recovery.

Scalar and bank paths use the same ascending nonzero-tap and polynomial operation order, separate
multiply/add and zero FMA contractions on every backend. W4 is Wasm `simd128` or NEON; W8 is AVX2;
legal unavailable backends return `None` only after complete request validation. Incompatible
programs and remainders use scalar tails.

## Deliverables

Effect crate; independent test-only f64 FIR/nonlinearity oracle; descriptor/state/resource tests;
scalar/W4/W8 and graph integration; representative transfer, alias, state, recovery and PDC evidence;
and a strict attempt verdict. Timed benchmark invocation count remains zero.

## Explicit non-goals

Hard clip, tanh/tube/tape modes, bias/asymmetry, DC blocker, linked channels, sidechain, adjustable
threshold, selectable oversampling/quality, ADAA, general SRC/oversampling infrastructure, true-peak
guarantee, corpus expansion, long audit, targets/instruction inspection, benchmark or listening.
Those qualification surfaces belong to Issue 052.

## Dependencies by exact issue title

- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral
- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

The fixed FIR is intentionally modest: its product claim is the measured improvement for the frozen
tone, not alias-free output. Enabled wet audio has linear-phase pre/post-ringing and a 31-sample group
delay; graph metadata is latency 31 plus tail 31, covering causal support through sample 62. ADAA is
deferred because singularity
and fractional-delay compensation are separate decisions. A changed curve/factor/table/phase,
general framework, or failed second attempt stops and requires rebriefing.

## Acceptance gates with objective measurements

1. The f32 table matches the brief and an independent f64 construction; each stage stays within
   `[-0.002,+0.002] dB` through `0.4*Fs` and at or below `-75 dB` from `0.6*Fs` to its Nyquist.
2. Independent f64 transfer probes at knots/interiors/extremes agree within
   `abs <= 2e-6 + 2e-6*abs(reference)`; monotonicity, odd symmetry and the exact `+-2/3` ceiling hold.
3. For the frozen 16,384-sample bin-3001 sine at +18 dB drive after three warm periods, total
   non-fundamental/base-fundamental energy improves by at least 2.0 dB versus the same f64 curve at
   naive 1x. Record both absolute ratios; no window or post-filter may hide bins.
4. Representative automation, mix-zero, bypass, both resets, active restore, signed zero,
   sanitation, one-million bounded finite inputs, injected lane recovery and isolation pass.
5. Available scalar/W4/W8 PCM/state/report parity is bit-exact; malformed bank requests reject
   before legal fallback; exact width resources and one-byte-below caps return ownership.
6. A ten-track fixture proves width-correct banks/tails, stable membership, scalar-delegate PCM,
   latency/tail/bypass/PDC/canonical stability and post-bank cap transaction. Focused and clean
   nonbenchmark workspace/policy gates pass.

## Target matrix

Product closure executes scalar and the available host W4/W8 backend. Complete native/AArch64/Wasm
build and instruction evidence is Issue 052.

## Required evidence

Candidate identity, table/response maxima, transfer and frozen alias ratios, exact state/resource
rows, representative runtime/bank/graph results, commands, attempt count, strict verdict, successor
link and `timed_benchmark_invocations=0`.
