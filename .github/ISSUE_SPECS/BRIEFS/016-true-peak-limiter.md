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
H = L+F                            // samples following an attenuation event held before release
lane_words = 23+B+R = 2N+31
```

| Fs | N | latency T | lane bytes | total state bytes |
|---:|---:|---:|---:|---:|
| 44100 | 441 | 447 | 3652 | 7304 |
| 48000 | 480 | 486 | 3964 | 7928 |
| 88200 | 882 | 888 | 7180 | 14360 |
| 96000 | 960 | 966 | 7804 | 15608 |

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
when rd < g0, or rd == g0 < 1:
    g = rd                                             // instantaneous attack/retention
    hold_remaining = H                                // H subsequent samples are held
otherwise when hold_remaining > 0:
    g = g0
    hold_remaining -= 1
otherwise:
    g = ar*g0 + (1-ar)*rd                              // one-pole release

main_ring[w] = x
z = main_ring[(w+1) mod B]            // exact T-sample delayed dry
w = (w+1) mod B
y = z                                when g == 1
y = z*g                              otherwise
```

Clamp finite `g` to `[0,1]` after the separately rounded release graph. An attenuation/equal
below-unity event sample is not counted inside `H`: it sets `hold_remaining=H`; each of the next
exactly `H` samples emits the held gain and decrements once; release can first execute on the
following sample. Equality below unity refreshes the horizon so a sustained requirement remains
held through its final sample. Equality at unity does not create a hold. `powf` and `exp` execute
once per active lane/sample with bounded standard `f32` math; no oracle or SIMD transcendental
approximation enters production.

At each frame: advance ceiling then release ramps; derive limit/release; sanitize L/R; update FIR
histories and phase estimates; link; derive/push/read required gains; update both gains/holds;
update/read main delay; then select output. A Point at `first_sample` performs update one on that sample and
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
required gain, gain state or enabled output resets only that lane's `g`, `hold_remaining` and
required-gain ring to positive zero, emits positive zero for that enabled sample, and increments
its recovery counter once. Its FIR/main histories and every other lane/track remain intact. Bypass
still emits delayed dry while performing and reporting the same internal recovery. Subsequent
valid samples release from zero, which is the ceiling-protecting state. No valid product fixture
may recover.

`FullToDefaults` clears histories/main ring, sets required-gain ring and `g` to one, clears
`hold_remaining`, resets cursors, and restores the complete prepared initial table.
`DiscontinuityKeepParameters` clears the same runtime state, retains lookahead, snaps ceiling/
release ramps to targets with zero remaining and discards active progress. Both leave metadata
unchanged.

Common state is empty. Each lane is exactly `2N+31` little-endian 32-bit words:

```text
word 0       main-ring cursor u32
word 1       required-gain-ring cursor u32
word 2       lookahead_ms f32
word 3       current gain g f32
word 4       hold_remaining u32
words 5..10  (current f32, target f32, remaining u32) for ceiling then release
words 11..22 detector history h[0..12], newest first
next B       main ring f32, physical order
final R      required-gain ring f32, physical order
```

Prepared defaults and derived `L/D` are not serialized. Snapshot requires exact lengths. Restore
accepts layout 1 only, parses both complete lanes into unpublished temporaries, validates cursors,
finite nonnegative lookahead/parameters in domain, `g` and required gains in `[0,1]`, ramp
remaining counts `<=64`, `hold_remaining<=L+F`, and every history/ring word finite normal-or-zero;
it rederives `L/D/H` and commits both lanes only after full success. Reject negative-zero parameter/lookahead, invalid common/trailing
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
latency/bypass/identity and exact `H=L+F` release deferral with release beginning on the next
sample; 64-update/restart/partition continuation; both resets; active transactional
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

---

## Amended by #90 (wave 2), 2026-08-23

This block supersedes the "Gain law and exact sample order" section above and the parts of
"Automation, sanitation, reset, recovery and state" that describe layout 1. Everything not named
here is unchanged: the Annex-2 table and its byte values, the declared latency `T = N + F`, the
parameter table, the ports, `LinkModeSet::new(3)`, `scratch_fixed_bytes = 24`, `TailSamples::Infinite`,
the `dBTP-est` unit string, and the fixed 1.0 dB internal estimator guard. Issue 050's "frozen
scalar input" and its immutable gain law are superseded by the law below.

### Why

The layout-1 gain law dropped to the required gain in a single sample and held it for `L + F`
samples. A step multiplied into programme material is a full-bandwidth discontinuity — the textbook
source of limiter distortion — and the step creates new inter-sample overshoot that the 4x detector
never sees, which is part of what the 1 dB guard was paying for (#90 F2, F7). The law also evaluated
`powf` and `exp` per sample per lane through the platform libm, which is the crate's native↔wasm
determinism break and about 28 % of its cost (#90 F1).

### Detector sample-term alignment

```text
P[n] = max(|h[6]|, |v[0]|, |v[1]|, |v[2]|, |v[3]|)
```

`|h[6]|`, not `|x[n]|`. `h[6]` is the input sample the four phases are centred on; comparing the
phases against a sample six frames in the future was the sole reason the old law needed the `+F`
hold. The four `v[p]` are bit-unchanged: same table, same increasing-tap order, same `+0.0`
accumulator, same separately rounded multiply then add.

### Gain law

With `L = floor(lookahead_ms * Fs / 1000 + 0.5)` clamped to `0..=N`, `R = N + 1`, `W_MIN = 32` and

```text
Wb = clamp(L + 1, W_MIN, R)                      // box-ramp window, in samples
GRID = 16384                                     // 2^14
limit = 10^((Cdb - 1.0) / 20)                    // unchanged 1.0 dB internal guard
c     = 1 - exp(-1 / (0.001 * tau * Fs))         // one-pole release rate
```

then, per sample:

```text
r[n]   = if P[n] > limit { limit / P[n] } else { 1 }
m[n]   = min(r[n-N ..= n-N+Wb-1])                // sliding minimum over the window
m_q[n] = floor(m[n] * GRID) / GRID
s[n]   = (m_q[n] + m_q[n-1] + ... + m_q[n-Wb+1]) / Wb
d[n]   = max(1 - s[n], fma(c, (1 - s[n]) - d[n-1], d[n-1]))
g[n]   = 1 - d[n]
y[n]   = x[n-T] * g[n]                           // T = N + F, unchanged
```

There is no hold counter and no instantaneous attack. `g[n] <= r[n-N]` holds **by construction**:
every box term `m_q[n-j]`, `j < Wb`, is a minimum over a window that contains `n-N`, so their
average is at most `r[n-N]`, and `d >= 1 - s` forces `g <= s`. The reduction domain is what makes
the release terminate at exactly `+0.0` after the D7 flush, and therefore `g` exactly `1.0` and
`z * 1.0` exactly `z`, signed zero included.

`W_MIN = 32` is a floor, not a preference: a ramp shorter than the twelve-tap detector span
re-creates inter-sample overshoot the detector has already measured. Thirty-two samples is 0.33 ms
at 96 kHz and 0.73 ms at 44.1 kHz. A lookahead of 0 ms therefore means "fastest ramp", never "step".
Measured cost of removing it: the worst true-peak margin over the #90 E4 matrix moves from
−0.961 dB to −0.398 dB.

Every `m_q` is a multiple of `2^-14` in `[0, 1]`, so a running sum of at most `R <= 961` of them is
an integer multiple of `2^-14` below `2^24`: every partial sum is exact in `f32`, the sliding sum
cannot drift, needs no resynchronisation, and is partition invariant. `S / Wb` is exactly `1.0` when
nothing is limiting, which is why the box average is a division and not a reciprocal multiply.

### Coefficient smoothing (supersedes "advance ceiling then release ramps; derive limit/release")

`SmoothingRule::Linear / 64` now ramps the **linear-domain** coefficients `limit` and `c`, not the
dB and millisecond parameters. Both are designed once per accepted automation Point, on the control
plane, in `f64`, through `miso-engine-math` — `limit = db_to_gain(Cdb - 1)`, `c = 1 - exp(...)` —
and rounded once to `f32`. The per-sample increment is precomputed at event time (decision D11), so
no transcendental and no division exists on the render path. Endpoints are exact and a new Point
restarts from the current value, exactly as before. `powf`/`exp` "once per active lane/sample with
bounded standard `f32` math" is withdrawn: this crate calls no platform transcendental at all.

### Sanitation, recovery and the boundary check (supersedes the per-value rules)

Decision D7. There is no per-value `is_finite`/`is_subnormal` check, no `sanitize`, no per-lane
`recover`, and no recovery counter. The only flush is `miso_engine_lane::flush` on the single
recursive word `d`. Output finiteness is checked **once per block per bank** by
`miso-engine-effect-runtime`: a block containing a NaN or a magnitude at or above `1e30` is zeroed
on both channels, the whole instance is reset to its defaults, and a block counter is incremented.
Input sanitisation is the input stage's job, not the effect's. `ProcessReport.sanitized_main_samples`
and `recovered_left/right_samples` are never incremented by this effect. `process_bank` no longer
has a structural guard that returns the caller's audio untouched and undelayed; width, quantum and
sidechain are `debug_assert!`s over compiler invariants.

### State layout 2

`state_layout_version` is **2**. The common section is the two-word version/word-count header that
`miso-engine-effect-runtime` stamps into every payload, so `common_bytes` is 8 and no longer 0. Each
channel of each track is `27 + B + 2R = 3N + 35` little-endian 32-bit words:

```text
word 0        bank main-delay cursor u32          word 1   bank gain-ring cursor u32
word 2        lookahead_ms f32                    word 3   reduction d f32, in [0,1]
word 4        minimum-filter phase u32, < Wb      word 5   minimum-filter prefix f32, in [0,1]
word 6        box sum S f32, a multiple of 2^-14 in [0, Wb]
words 7..10   limit ramp:   current f32, target f32, step f32, remaining u32 (<= 64)
words 11..14  release ramp: current f32, target f32, step f32, remaining u32 (<= 64)
words 15..26  detector history h[0..12], newest first
next B        main-delay ring f32, physical order (B = N + F)
next R        required-gain ring f32 in [0,1] (raw values and van Herk suffix minima)
final R       box ring f32, in [0,1] and on the 2^-14 grid
```

| Fs | N | latency T (unchanged) | lane bytes | payload bytes (8 + 2 x lane) |
|---:|---:|---:|---:|---:|
| 44100 | 441 | 447 | 5432 | 10872 |
| 48000 | 480 | 486 | 5900 | 11808 |
| 88200 | 882 | 888 | 10724 | 21456 |
| 96000 | 960 | 966 | 11660 | 23328 |

Restore accepts layout 2 only and validates, before committing anything: exact section lengths, the
header, cursors in range, finite non-negative in-domain `lookahead_ms` (negative zero rejected),
`d` and `prefix` in `[0,1]`, `phase < Wb`, `S` in `[0, Wb]` and on the grid **and equal to the sum
of the `Wb` most recent box-ring words recomputed from the payload**, both ramps' `current` and
`target` inside the per-rate coefficient range with `remaining <= 64`, and every history and ring
word finite. Signed zero is legal in the history and main rings.

The payload's cursor is the frame its rings are written in. A bank shares one cursor pair across its
`W` tracks — layout 1's per-lane cursors were redundant, since every lane and both channels have
always advanced in lockstep (#90 F3/F6) — so restore rotates the rings from the payload's frame into
the receiver's while copying them: logical age `a` of the payload lands at logical age `a` of the
receiver. The rotation is the identity whenever the two frames agree, which is the case for a scalar
instance restored from a scalar snapshot and for every track of a bank restored from that bank.
Scalar and bank track payloads stay byte-compatible.

`FullToDefaults` clears the history and main ring to `+0.0`, the required-gain and box rings to
`1.0`, `S` to `Wb`, `prefix` to `1.0`, `phase`, `d` and both cursors to zero, and snaps both ramps to
the prepared defaults' coefficients. `DiscontinuityKeepParameters` does the same to the runtime words
and snaps both ramps to their current targets, keeping the lookahead.

### Bank and kernel shape (supersedes "Do not add a limiter core kernel")

One generic `limiter_block<L: Lane>` owns the frame loop at `WIDTH` 1, 4 and 8; a scalar instance is
that body at `L = f32` over a `W = 1` AoSoA block, so no separate scalar path exists. Per channel the
bank owns one AoSoA arena (`history` 12 x W tap-major, the three rings `slots x W`) and one cursor
pair, allocated at preparation. `PreparedGateGainKernelV1`, `KernelBackendV1` dispatch and
`miso-engine-core` are no longer used by this crate. There is no `unsafe`, no `target_feature`, no
intrinsic and no vector-library name anywhere in it: the ISA is pinned at compile time and attested
at boot (decision D4).

### Evidence

`crates/miso-engine-true-peak-limiter/tests/MUTATIONS.md` lists sixteen red mutations and the four
that survived their first target, with what was done about each. The ceiling property is gated over
4 rates x 2 links x 3 ceilings x 4 lookaheads x 2 releases x 6 corpora against an independent `f64`
4x estimator with no tolerance; the worst margin is −0.961 dB. Descriptive throughput on Zen 5 at
`x86-64-v3`: 5.05 ns per channel lane-sample at W8, against the audit replica's 25.1 ns.
