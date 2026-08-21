# Sol implementation brief — issue 016 launch fixed-4x true-peak safety limiter

## Authority, product boundary and attempt budget

**READY FOR TERRA ATTEMPT 1.** This brief and Issue 016 are authoritative. Deliver one Normal-
quality detector-only fixed-4x safety limiter. There are exactly two total attempts: Terra attempt
1 and, if needed, one bounded Sol correction/review. A second failure stops. Never inspect V1,
change a frozen number after failure, add another quality, or run Issue-049 work.

Issues 011, 006, 013 and 037 and the current effect/rack/graph APIs are accepted dependencies.
Issue 008 is not overall PASS; use only its preserved generic prepared-bank architecture. Do not
redesign runtime/session syntax, program keys, graph/PDC, AoSoA layout, target dispatch or state
interchange.

```text
effect / contract / state     miso.true-peak-limiter / 1.0 / layout 1
quality / rates               Normal / 44100, 48000, 88200, 96000 Hz
ports                          required main-in and main-out, DualMonoPlanar
links                          DualMono, Maximum
measurement                    fixed 4x, order-48/four-phase Annex-2 FIR
internal estimator guard       exactly 1.0 dB
lookahead                      independently 0..10 ms per lane, preparation/state only
latency                        Fs/100 + 6 samples
tail                           Infinite
automation                     block Point, exact linear 64-update ramps
banking                        homogeneous W4/W8 plus exact scalar tails
```

No sidechain, audio oversampling/decimation, hard clipper, extra quality, threshold/attack/makeup/
mix or public true-peak meter is hidden behind defaults. `dBTP-est` names this exact estimator;
certification and expanded standard evidence belong to Issue 049.

## Descriptor, preparation, latency and resources

Use contract 1.0, state layout 1, `LinkModeSet::new(3)`, Normal quality and ordered required
`main-in`/`main-out` ports only. Parameters are readable and `PerLane`; descriptor index and ID
remain distinct but ordered identically:

| index | ID | name | unit | inclusive domain | default | mapping | rate / smoothing |
|---:|---:|---|---|---:|---:|---|---|
| 0 | 1 | ceiling | dBTP-est | -24..0 | -1 | Linear | Block / Linear 64 |
| 1 | 2 | release | ms | 10..2000 | 100 | Logarithmic | Block / Linear 64 |
| 2 | 3 | lookahead | ms | 0..10 | 5 | Linear | None / None |

Preparation consumes the complete ordered six-value L/R table. Reject negative zero, nonfinite or
out-of-domain values. IDs 1–2 initialize current=target and remaining=0; ID 3 requires reprepare
or compatible state restore. For rate `Fs`:

```text
N = Fs/100                         // maximum 10 ms lookahead, exact integer
F = 6                              // discrete alignment of the 23.5/4-sample FIR delay
T = N+F                            // immutable reported latency
B = T+1                            // main-delay ring length
R = N+1                            // required-gain ring length
L = floor(f64(lookahead_ms)*Fs/1000 + 0.5), clamped to 0..N
D = N-L                            // required-gain delay
lane_words = 22+B+R = 2N+30
```

| Fs | N | latency T | lane bytes | total state bytes |
|---:|---:|---:|---:|---:|
| 44100 | 441 | 447 | 3648 | 7296 |
| 48000 | 480 | 486 | 3960 | 7920 |
| 88200 | 882 | 888 | 7176 | 14352 |
| 96000 | 960 | 966 | 7800 | 15600 |

Each descriptor row has `TailSamples::Infinite`, `scratch_fixed_bytes=24` for two retained
three-`f32` prepared-default tables and `scratch_bytes_per_frame=0`. State and fixed bytes are
independent of quantum, track count and source duration. Exact caps and one-byte-below rejection
precede publication. Prepared metadata is immutable. A bank retains exactly
`W * (2*lane_bytes + 24)` declared payload/default bytes; headers, vtables and the already accepted
prepared kernel token are not state-payload bytes but any owned vectors/member metadata remain
covered by the graph's existing checked resource accounting.

## Frozen true-peak estimator

BS.1770-5 Annex 2 supplies this detector table and states that floating point needs no initial
12.04 dB attenuation/compensation. Store the following constants directly as `f32`; every value is
dyadic and exactly representable. Rows are history taps `k=0..11`, columns phases `p=0..3`:

| k | p0 | p1 | p2 | p3 |
|---:|---:|---:|---:|---:|
| 0 | 0.0017089843750 | -0.0291748046875 | -0.0189208984375 | -0.0083007812500 |
| 1 | 0.0109863281250 | 0.0292968750000 | 0.0330810546875 | 0.0148925781250 |
| 2 | -0.0196533203125 | -0.0517578125000 | -0.0582275390625 | -0.0266113281250 |
| 3 | 0.0332031250000 | 0.0891113281250 | 0.1015625000000 | 0.0476074218750 |
| 4 | -0.0594482421875 | -0.1665039062500 | -0.2003173828125 | -0.1022949218750 |
| 5 | 0.1373291015625 | 0.4650878906250 | 0.7797851562500 | 0.9721679687500 |
| 6 | 0.9721679687500 | 0.7797851562500 | 0.4650878906250 | 0.1373291015625 |
| 7 | -0.1022949218750 | -0.2003173828125 | -0.1665039062500 | -0.0594482421875 |
| 8 | 0.0476074218750 | 0.1015625000000 | 0.0891113281250 | 0.0332031250000 |
| 9 | -0.0266113281250 | -0.0582275390625 | -0.0517578125000 | -0.0196533203125 |
| 10 | 0.0148925781250 | 0.0330810546875 | 0.0292968750000 | 0.0109863281250 |
| 11 | -0.0083007812500 | -0.0189208984375 | -0.0291748046875 | 0.0017089843750 |

After sanitizing input `x[n]`, shift the 12-word lane history so `h[0]=x[n]` and
`h[k]=old_h[k-1]`. For each phase, in production `f32`, evaluate in increasing `k` order with
separately rounded multiply then add:

```text
v[p] = (((H[0][p]*h[0] + H[1][p]*h[1]) + ...) + H[11][p]*h[11])
P = max(abs(x[n]), abs(v[0]), abs(v[1]), abs(v[2]), abs(v[3]))
```

Initialize the accumulator to positive zero; canonical max order is exactly the order above. Do
not fuse, reassociate, use `mul_add`, generate coefficients, add DC blocking/pre-emphasis or use
the FIR output as audible audio. The causal phase group delay is 23.5 high-rate samples; `F=6`
is the frozen nearest input-sample alignment. The fixed guard covers the declared estimator's
finite under-read budget; it is not a license to call sample peak true peak.

## Gain law and exact sample order

Sanitize both main lanes before interpolation. For `Maximum`, compute `P=max(P_left,P_right)` and
send that value to both lane gain laws; `DualMono` retains the independent values. Linking shares
only `P`. Ceiling, release, lookahead, histories, rings, gain, state, recovery and output remain
lane-local.

For lane ceiling `Cdb`, release `tau`, linked estimate `P`, previous gain `g0`:

```text
limit = 10^((Cdb - 1.0)*0.05)
required = 1                         when P <= limit or P == 0
required = clamp(limit/P, 0, 1)      otherwise

gain_ring[q] = required
rd = gain_ring[(q + 1 + L) mod R]    // delay D=N-L; current entry when L=N
q = (q+1) mod R

ar = exp(-1 / (0.001*tau*Fs))
g = rd                               when rd < g0       // instantaneous attack
g = ar*g0 + (1-ar)*rd                otherwise         // one-pole release

main_ring[w] = x
z = main_ring[(w+1) mod B]            // exact T-sample delayed dry
w = (w+1) mod B
y = z                                when g == 1
y = z*g                              otherwise
```

Clamp finite `g` to `[0,1]` after the separately rounded release graph. Equality takes release.
`powf` and `exp` execute once per active lane/sample with bounded standard `f32` math; no oracle
or SIMD transcendental approximation enters production.

At each frame: advance ceiling then release ramps; derive limit/release; sanitize L/R; update FIR
histories and phase estimates; link; derive/push/read required gains; update both gains; update/read
main delay; then select output. A Point at `first_sample` performs update one on that sample and
reaches the exact target on update 64; a new Point restarts from current. The ceiling gate is
evaluated against the current smoothed value.

Whole-effect bypass returns `z` bits exactly but advances all ramps, FIR histories, required-gain
rings, gain and main rings. `g==1` also returns `z` bits exactly, preserving signed zero. There is
no hard clip, makeup or hidden channel coupling.

## Automation, sanitation, reset, recovery and state

Accept only canonical Block Points at `first_sample`, with equal start/end samples and bit-equal
values, for descriptor positions 0–1 and explicit Left/Right lanes. Reject Both, lookahead/
out-of-range positions, other span kinds, duplicates, disorder, excess capacity and domain errors;
count each invalid span saturating while retaining other valid targets in stable descriptor/lane
order. Normalize accepted numeric-zero targets to positive zero.

Use accepted sample sanitation: nonfinite or subnormal main input becomes positive zero and
increments only its lane sample counter; signed finite zero is retained. Finite computed subnormal
phase/gain/output becomes positive zero without recovery. A nonfinite detector, limit, coefficient,
required gain, gain state or enabled output resets only that lane's `g` and required-gain ring to
positive zero, emits positive zero for that enabled sample, and increments its recovery counter
once. Its FIR/main histories and every other lane/track remain intact. Bypass still emits delayed
dry while performing and reporting the same internal recovery. Subsequent valid samples release
from zero, which is the ceiling-protecting state. No valid product fixture may recover.

`FullToDefaults` clears histories/main ring, sets required-gain ring and `g` to one, resets cursors,
and restores the complete prepared initial table. `DiscontinuityKeepParameters` clears the same
runtime state, retains lookahead, snaps ceiling/release ramps to targets with zero remaining and
discards active progress. Both leave metadata unchanged.

Common state is empty. Each lane is exactly `2N+30` little-endian 32-bit words:

```text
word 0       main-ring cursor u32
word 1       required-gain-ring cursor u32
word 2       lookahead_ms f32
word 3       current gain g f32
words 4..9   (current f32, target f32, remaining u32) for ceiling then release
words 10..21 detector history h[0..12], newest first
next B       main ring f32, physical order
final R      required-gain ring f32, physical order
```

Prepared defaults and derived `L/D` are not serialized. Snapshot requires exact lengths. Restore
accepts layout 1 only, parses both complete lanes into unpublished temporaries, validates cursors,
finite nonnegative lookahead/parameters in domain, `g` and required gains in `[0,1]`, remaining
counts `<=64`, and every history/ring word finite normal-or-zero; it rederives `L/D` and commits
both lanes only after full success. Reject negative-zero parameter/lookahead, invalid common/trailing
bytes, one corrupt lane or incompatible rate/length without mutation. Signed zero is legal in FIR
and main rings. Scalar and bank track payloads are byte-compatible; failed bank-track restore
changes no track.

## Scalar/bank/graph contract

Scalar runs the frozen lane graph independently. A homogeneous bank owns exactly `W` complete
per-track states per channel, walks track lanes in ascending order for detector/transcendental/state
work, packs delayed samples/gains/identity masks, and invokes the existing
`PreparedGateGainKernelV1` once per channel/sample. Its graph is exactly one multiply plus dry
bit-selection when `g==1`; AVX2+FMA aliases the zero-contraction graph. Do not add a limiter core
kernel or expose unsafe SIMD.

Bank binding validates every request before any legal fallback and requires exact width,
backend/width compatibility, identical program signature and Normal quality. An unavailable
backend returns `Ok(None)` transactionally; malformed or heterogeneous requests reject. Initial
values may differ per track/lane. Never pad tracks or impose a ceiling.

For finite-normal/no-sanitation inputs, base same-target scalar/W4/W8 PCM, complete carried state
and reports are bit-identical. Prepare one ten-track 48-kHz/128-frame graph fixture: W8 retains one
bank plus two scalar tails; W4 retains two banks plus two tails; scalar retains ten scalars. Assert
stable membership/order, parameter differences, resource report, exact T=486 PDC enabled/bypass,
consecutive-block PCM/state/report parity, unchanged graph/schedule/observer bytes and complete
ownership return on a one-byte-below post-bank cap. This is a product fixture, not the expanded
Issue-049 cohort/audit matrix.

## Representative independent evidence and stop rules

Test-only `f64` code owns its coefficient table, FIR/history, ring and gain law and cannot import
production limiter helpers/state. Freeze the production table byte hash and prove every f32 phase
output within `2e-6` absolute. On compact phase sweeps through normalized 0.45 and a separate
high-rate `f64` reconstruction, record worst pre-guard under-read `<=0.75 dB`.

At every launch rate, both links, ceilings `[-6,-1]` and lookaheads `[0,5,10]`, render compact
asymmetric near-Nyquist bursts and impulses past T. The independent output estimate must be
`<= current_smoothed_ceiling + 0.1 dB`, output/state finite and recovery zero. Also prove exact
latency/bypass/identity; 64-update/restart/partition continuation; both resets; active transactional
restore; signed-zero identity; subnormal/nonfinite sanitation; injected lane-local recovery;
scalar/W8 output/state/report parity; and the ten-track graph/cap vertical.

Run only focused locked limiter/core/effect-compiler/graph tests, formatting and warning-denied
Clippy, then one locked workspace check/test/Clippy/rustdoc and applicable policies on one clean
candidate. Do not create/run a corpus generator, randomized/million suite, 100,000 audit, target or
object inspection, benchmark/preflight/timing, audition or listening. `timed_benchmark_invocations`
must remain zero.

FAIL for a sample-peak-only detector, changed table/guard/domain/tolerance, audible oversampling,
extra quality/parameter/sidechain/API, variable reported latency, runtime table/filter design or
feature detection, shared lane/track state, unaccounted storage, nontransactional restore/fallback,
new SIMD kernel, FMA contraction, production-derived oracle, benchmark invocation, or work beyond
the two-attempt budget.

Primary decisions: `[ITU-BS1770-5]` supplies only the measurement estimator and coefficient table;
`[REISS-COMP]` supports the explicit detector/gain split and one-pole timing; `[SMITH-SASP]` and
`[VAIDYANATHAN-MULTIRATE]` support delay/FIR state and polyphase interpretation; immutable reported
latency follows `[VST3-LATENCY]`. `[EBU-R128]` is qualification context, not a limiter gain-law
source or a certification claim.
