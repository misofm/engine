# Where `f64` state should enter after launch

Status: research conclusion for issue #126. This report changes no production code, fixture,
numeric contract, or launch decision.

## Decision

`f64` should remain a per-kernel opt-in. There is no evidence for a second, engine-wide
"high-precision" mode. Introduce it in this order:

| Rank | Candidate | Recommendation | Why |
|---:|---|---|---|
| 1 | Future unbounded statistics | Use scalar `f64` sums and `u64` counts from the first implementation. | A unit-step `f32` sum stops changing after only `2^24` additions; the existing bounded meter already gets this right. |
| 2 | Future ADAA divided differences | Require `f64` evaluation and retained divided-difference/antiderivative state where the chosen order reuses it. | Subtracting nearby antiderivatives is demonstrably ill-conditioned in `f32`; this precision is part of the algorithm, not a quality mode. |
| 3 | Slow log-domain dynamics envelopes | Evaluate one mixed-precision candidate: `f64` state/update, explicit `f32` output conversion. | The accepted 5 s domain can leave a synthetic constant-target error just over 0.1 dB, for only one or two state words per channel. |
| 4 | Low-frequency recursive filters | Evaluate targeted `f64` SVF state/intermediates for low-frequency/high-Q parametric EQ and the lowest LR4 crossover rows. Do not revive the broad built-in mode rejected by issue #31. | A pole-radius screen and issue #31 both show material residual improvement, but #31's broad candidate failed its analytic and impulse gates. |
| 5 | Dynamic-EQ/de-esser coefficient modulation | Design coefficients in deterministic `f64`, round once to the existing `f32` coefficient/ramp domain, and begin with `f32` filter state. | It buys coefficient accuracy without narrowing the audio SIMD kernel. Filter state can opt in later only if the targeted gate at rank 4 passes. |
| 6 | Delay feedback storage | Keep the feedback ring and damping state in `f32`. | `|feedback| <= 0.95` bounds error with time; a stereo `f64` ring would add 0.73 MiB at 48 kHz or 1.46 MiB at 96 kHz per effect. |

The first two decisions apply when those future products are built. They do not require a change
today: the current soft clipper is fixed 2x oversampling, not ADAA, and the current meter already
accumulates energy in `f64`.

### Quantitative comparison

"Floor" below means the stated screen or experiment, not an audible-noise claim. CPU costs are
static operation/width projections because this research issue authorizes no timed benchmark.

| Site | `f32` result | `f64` result | Gate/audibility relevance | Cost | Verdict |
|---|---|---|---|---|---|
| Unbounded energy/count sum | Unit additions stall at `2^24`: 349.5 s at 48 kHz | `2^53`: about 5,946 years at 48 kHz | Hours-long statistics cannot use `f32`; the current windowed meter already uses `f64` | +4 bytes per scalar sum; no lane-width cost | **`f64` required** for future unbounded sums |
| First-order cubic ADAA at `x=0.75`, adjacent representable endpoints | 17.95% relative divided-difference error; smaller requested deltas collapse to zero | 5.83e-15 relative error at the same endpoints; exact equality still needs the analytic limit | Cancellation is an algorithm failure, not an audibility preference | `f64` antiderivative/difference operations; one or more 8-byte history words per lane; half-width if vectorized | **`f64` required** when ADAA is introduced |
| 10 Hz, Q 18 SVF screen at 96 kHz | -104.87 dB white-error screen; accepted #87 impulse error 0.0091 dB | -279.47 dB under the same screen; #87 `f64`-state impulse error 0.00042 dB | Product domain reaches the corner and `f64` buys margin, but `f32` passes; broad issue #31 failed adoption | +64 bytes per stereo four-section EQ; 2x packets, and fused sites project near 4x Wasm primitive ops | **Targeted experiment**, no broad adoption |
| 5 s log-domain release, -20 to -5 dB at 96 kHz | Stalls at -5.114441 dB: 0.114441 dB error | With the same binary32 coefficient, a non-fused binary64 screen stalls 2.13e-10 dB from target | Synthetic result crosses the 0.1 dB dynamics scale and merits a real corpus | +8 bytes/stereo compressor or +16 bytes/stereo two-band compressor; preserving the fused law splits only smoother lanes but projects near 4x Wasm primitive ops | **Targeted experiment** |
| Dynamic filter coefficient ramp | Existing 64-sample law assigns the target exactly: zero endpoint drift after every ramp | `f64` design followed by one `f32` rounding has the same exact endpoint word | Long automation does not accumulate coefficient error if every ramp snaps | Control/event-rate `f64`; no persistent state or audio-lane cost | **Use `f64` design, retain `f32` state** |
| Delay at `|g|=0.95` | -139.15 dB white-error screen; -118.47 dB coherent bound; both bounded with time | A full `f64` ring screens at -313.75/-293.07 dB, but output still narrows to `f32` | No hours-long drift; no measured gate failure | +768,024 bytes at 48 kHz or +1,536,024 at 96 kHz per stereo effect, plus doubled ring bandwidth | **`f32` sufficient** |
| Deep notch / measurement output | The accepted audio path's practical recursive/null floor is approximately -110 to -120 dBFS per section | Offline `f64` reference can resolve much deeper nulls before final `f32` output | This is below the audio product floor but relevant to an analyzer | A lane-parallel measurement mode pays the same half-width/state cost as an `f64` SVF | **Document the limit; no audio mode** |

## Precision baseline

For normalized binary arithmetic, the unit roundoffs used here are

```text
u32 = 2^-24  = 5.960464477539063e-8   = -144.4944 dB
u64 = 2^-53  = 1.110223024625157e-16  = -319.0918 dB
```

If a rounding error is screened as independent uniform noise over half an ulp, its RMS levels are
`u / sqrt(3)`: -149.2656 dB for `f32` and -323.8630 dB for `f64`. These are local, relative
arithmetic levels, not guaranteed output noise floors. A recursive error transfer can amplify
them, and cancellation can discard significant bits before either bound is useful. The engine's
-120 dB residual threshold is an amplitude of `1e-6`.

The numerical screens below deliberately make simple assumptions visible. They rank candidates;
they do not replace each product's frozen independent-oracle corpus.

## 1. Recursive filter state

### Screening estimate

For a complex pole near radius

```text
r = exp(-pi f / (Q Fs)),
```

a conservative coherent per-sample error gain is `1 / (1 - r)`, while the gain for independent
white errors is `1 / sqrt(1 - r^2)`. This one-pole model does not include the TPT SVF's several
rounding sites or its output-node mix, but it exposes the corner that deserves a corpus.

| Case | Pole radius | Coherent gain | White-error RMS gain |
|---|---:|---:|---:|
| 10 Hz, Q 18, 96 kHz | 0.9999818197 | 94.808 dB | 44.394 dB |
| 10 Hz, Q 18, 88.2 kHz | 0.9999802119 | 94.072 dB | 44.026 dB |
| 10 Hz, Q 18, 44.1 kHz | 0.9999604242 | 88.051 dB | 41.015 dB |
| 80 Hz, Q 0.707, 96 kHz | 0.996304 | 48.646 dB | 21.321 dB |
| 1 kHz, Q 0.707, 48 kHz | 0.911582 | 21.069 dB | 7.721 dB |

At the worst screening point, adding the 44.394 dB white-error gain to the -149.266 dB `f32`
rounding level gives about -104.87 dB. That is above the -120 dB residual floor and therefore
material enough to test. The coherent bound is intentionally pessimistic and must not be presented
as a predicted audible level.

The parametric-EQ product domain reaches exactly 10 Hz and Q 18, but not the exploratory
`f0 < 10 Hz` or `Q > 18` region. At a fixed analog frequency/Q, the 96 kHz row is the closest to
the unit circle and therefore the worst of the launch rates in this per-sample error screen. Simper
and Zavalishin describe the trapezoidal SVF realization and its favorable binary32 numerical
properties [9][10]; that structural conditioning is already in the accepted implementation.

The accepted #87 evidence is more authoritative than the screen: the binary32 TPT SVF had
0/1,488 analytic-grid failures (worst 0.00068 dB), a worst one-second impulse error of 0.0091 dB,
and bounded million-sample noise; the same words with binary64 state reduced the impulse error to
0.00042 dB [11]. The known binary32 per-section roundoff region is approximately -110 to -120
dBFS; four uncorrelated cascade sections add at most `10 log10(4) = 6.02 dB`. Both realizations
pass the frozen 0.05 dB impulse gate. The 26.7 dB ratio between the two recorded worst impulse
errors is useful engineering margin, not evidence of audible improvement.

This conclusion is consistent with Dattorro's two finite-precision criteria: internally generated
filter noise should remain below the input's spectral quantization floor and below its total noise
power. Dattorro also shows why the noise transfer and number of quantization sites matter, rather
than precision in isolation [1].

### Repository evidence and boundary

Issue #31 already ran the broad experiment for launch HPF/LPF sections. Across 198 materiality
rows, every baseline residual worse than -120 dB improved by at least 31.8102 dB, the global
maximum improvement was 147.67 dB, and the candidate's worst residual was -134.9231 dB with no
regressions. That is strong evidence that retained `f64` can lower recursive residuals.

It is not evidence to adopt that candidate. Thirty-eight analytic rows missed the deliberately
strict `1e-9` dB gate (worst `1.76e-7` dB), and its worst finite-window impulse DFT error was
0.018276 dB against a 0.005 dB limit at 44.1 kHz, 100 Hz low-pass, 19,844 Hz probe. The recorded
decision is **NO ADOPTION** [2]. This report does not reopen or weaken it.

The next useful candidate is narrower:

1. Preserve `f32` audio I/O and the accepted `f32` coefficient words and coefficient-ramp law.
2. Promote coefficient words exactly at the kernel boundary; retain only the two SVF integrators
   per section in `f64`; perform the recurrence in `f64`; cast the output once to `f32`.
3. Freeze a corpus around parametric-EQ 10/20 Hz, Q near its maximum, 88.2/96 kHz, all affected
   shapes and gains, plus the 80 Hz LR4 crossover at the same rates.
4. Require the existing analytic/impulse/partition/determinism gates unchanged, no regression in
   rows where the baseline is already below -120 dB, and a material improvement in every selected
   row. If those conditions conflict, retain `f32`.

Wishnick supports the structure choice, not a precision claim: the trapezoidal SVF is suitable for
per-sample time variation, and interpolated SVF coefficients tend to produce sensible intermediate
filter shapes [3]. The precision decision still belongs to the independent numeric gate above.

### Static cost projection

The projection counts representation and packet width; no timed benchmark was run.

| Kernel | Persisted-state delta per scalar stereo effect | SIMD consequence |
|---|---:|---|
| Four-section parametric EQ, state only | 16 state words grow from 4 to 8 bytes: **+64 bytes** (608 to 672 payload bytes) | Each logical W4 state recurrence is two `f64x2` packets on Wasm; each logical W8 recurrence is two `f64x4` packets on AVX2. Coefficient promotion and final narrowing are additional work. |
| Two-stage LR4 crossover, state only | 8 state words grow from 4 to 8 bytes: **+32 bytes** | Two SVF stages per channel receive the same packet-width penalty; the large lookahead rings remain `f32`. |
| Broad issue-#31 section | Candidate was 48 bytes/section with 16 mutable state bytes versus 24/8 for the baseline | Recorded ceiling was 2x vector operations; the candidate stayed within it but failed numeric gates [2]. |

The low memory cost does not imply low compute cost. Filter recurrences occupy the hot audio path,
and binary64 halves the physical lane count at both relevant SIMD widths. At a fused site, the
Wasm projection compounds two binary64 packets with roughly 20 primitive operations per exact
soft-FMA, versus roughly 10 for the existing single binary32 packet: about 4x the primitive work at
that site before promotion/narrowing. AVX2 with native binary64 FMA pays primarily the 2x packet
count and register pressure. These are static projections, not benchmark claims.

## 2. Long-running accumulators and envelopes

### True accumulators

A positive `f32` accumulator updated by exactly `1.0` stops changing at `2^24`, because the next
unit is at the half-ulp tie. That happens surprisingly quickly at audio rates:

| Rate | Time to `f32` unit-add stall |
|---:|---:|
| 44.1 kHz | 380.436 s |
| 48 kHz | 349.525 s |
| 88.2 kHz | 190.218 s |
| 96 kHz | 174.763 s |

The corresponding `f64` count is `2^53`, about 5,946 years at 48 kHz. Therefore all future
session-length energy, integrated-loudness, exposure, or similar monotonically accumulated
statistics should use a scalar `f64` sum with a `u64` sample/block count and a fixed traversal
order. ITU-R BS.1770 defines the channel weighting and gated loudness basis; EBU R 128 defines its
broadcast use [4][5]. Precision does not make an ungated raw energy meter a certified loudness
meter.

The existing `MeterAccumulator` already holds `energy: f64`, evaluates
`f64::from(sample) * f64::from(sample)`, and takes an `f64` RMS at each bounded window. No current
meter change is recommended. For a future hours-long statistic, use fixed-order block partials if
needed, but never a width-dependent horizontal reduction.

### Stable one-pole envelopes

An envelope is a stable recurrence, not an unbounded accumulator. Its error does not grow merely
because a session lasts for hours: old errors decay with its pole. Its relevant failure mode is an
`f32` fixed point before the mathematical target is reached.

The following scratch experiment used the compressor's actual rate coefficient
`c = f32(1 - exp(-1/(tau Fs)))` designed in `f64`, and the actual one-rounding update
`f32_fma(c, target - y, y)`. Inputs and state were binary32; the loop stopped when the next state
was bit-identical to the previous state.

| Transition | Rate / time constant | Stall time | Final value | Error from target |
|---|---|---:|---:|---:|
| 0 to -20 dB attack | 96 kHz / 200 ms | 1.376 s | -19.981689 dB | +0.018311 dB |
| -20 to -5 dB release | 96 kHz / 5,000 ms | 23.797 s | -5.114441 dB | -0.114441 dB |
| -20 to -5 dB release | 48 kHz / 5,000 ms | 27.267 s | -5.057220 dB | -0.057220 dB |
| 0 to -100 dB attack | 96 kHz / 200 ms | 1.421 s | -99.926758 dB | +0.073242 dB |

This is a constant-target stress, not an assertion that program material produces those exact
errors. It is enough to justify one candidate gate because the 96 kHz maximum-release row crosses
the dynamic effects' 0.1 dB accuracy scale.

Repeating the -20 to -5 dB, 5 s/96 kHz row with the same rounded binary32 coefficient but a
binary64 state and explicit non-fused binary64 multiply then add stalls after 124.3 s at
`-5.000000000213163` dB, only `2.13e-10` dB from the target. The longer convergence time is the
expected consequence of resolving the tail rather than reaching a coarse fixed point.

That experiment is a materiality screen, not the candidate's frozen arithmetic. A production
candidate should preserve the current single-rounding smoother law through `Lane64::fma`; its
exact software-FMA result, timing and cross-target bits belong to the gate.

Evaluate `f64` only for the log-domain smoother state and its multiply-add, then explicitly narrow
the state value passed to the existing `f32` gain conversion. A compressor adds one four-byte word
per channel (**+8 bytes stereo**); the two-band compressor adds two per channel (**+16 bytes
stereo**). The vector penalty is confined to the smoother: W4 needs two `f64x2` packets on Wasm
and W8 needs two `f64x4` packets on AVX2, while detector, curve, gain and mix remain `f32`.

Adopt only if a frozen corpus over all four launch rates, both extrema, automation/partition rows,
and the independent dynamics oracle improves the long-time error without moving timing or output
identity outside existing gates. Giannoulis, Massberg and Reiss remain the algorithm authority for
the branching smoother topology [6]. A bounded time-smoothed RMS detector should make the same
case before opting in; an unbounded RMS/energy statistic belongs to the accumulator decision above.

## 3. ADAA cancellation

First-order antiderivative antialiasing evaluates a divided difference

```text
y[n] = (F(x[n]) - F(x[n-1])) / (x[n] - x[n-1]),   where F' = f.
```

Parker, Zavalishin and Le Bivic derive this form and explicitly identify its finite-precision
ill-conditioning when the two inputs are close [7]. Bilbao et al. generalize the method to higher
antiderivatives and report improved alias suppression over the first-order form [8].

For the engine's cubic `f(x) = x - x^3/3`, use
`F(x) = x^2/2 - x^4/12`. The table compares an explicitly rounded `f32` evaluation of both
antiderivatives, subtraction and division with an `f64` reference evaluated at the same binary32
endpoints. The base input was exactly 0.75; its next binary32 value is 5.9604645e-8 away.

| Requested delta | Effective binary32 delta | `f64` reference | `f32` divided difference | Relative error |
|---:|---:|---:|---:|---:|
| `1e-2` | 0.0099999905 | 0.611537415 | 0.611538291 | 1.43e-6 |
| `1e-4` | 0.0001000166 | 0.609396876 | 0.609356403 | 6.64e-5 |
| `1e-5` | 0.0000100136 | 0.609377190 | 0.610119045 | 1.22e-3 |
| `1e-6` | 0.0000010133 | 0.609375222 | 0.617647052 | 1.36e-2 |
| `1e-7` | 0.0000001192 | 0.609375026 | 0.5 | 1.79e-1 |
| `1e-8` | 0 | inputs collapse | division by zero | n/a |

Against an 80-digit decimal evaluation at the same endpoints, the binary64 relative error in the
`1e-7` row is `5.83e-15` (and no more than `4.75e-12` in the nonzero rows shown). Binary64 is
therefore ample for these binary32 inputs, but it does not replace the exact-equality limit.

The earlier v1 implementation supplies a separate measured guard result that fixes the practical
scale of that limit branch. Its ADAA chain placed `EPS = 1e-6` where binary64 cancellation reached
the binary32 noise floor; the same computation in binary32 would have needed the guard at roughly
`|delta| = 0.05`, effectively selecting the guard on every sample. A perturbation of at most three
ulps measured **-41.9 dBFS peak and -90.8 dBFS steady-state** through that chain. This is historical
evidence for the need to retain the binary64 divided difference and an explicit near-equality
guard, not a threshold pin for a future V2 ADAA topology. Source: `misofm/engine`,
`docs/01-V2-LEARNINGS.md` lines 88-92 in the pre-deprecation snapshot read 2026-08-24
([source lines](https://github.com/misofm/engine/blob/main/docs/01-V2-LEARNINGS.md#L88-L92)).

The implementation rule for a future ADAA effect is therefore:

- promote the `f32` inputs exactly and evaluate antiderivatives and divided differences in `f64`;
- retain any antiderivative or divided-difference history reused by the selected ADAA order in
  `f64`, then narrow only the final audio output;
- specify a deterministic near-equality threshold and use the analytic limit/Taylor form there,
  including exact-equality, adjacent-`f32`, sign-zero and breakpoint tests;
- keep the operation order identical on every target and include slow ramps, DC, alternating
  adjacent values, and nonlinearity breakpoints in the independent corpus.

`f64` postpones rather than abolishes cancellation, so the limit branch remains mandatory. The
current `miso.soft-clip` uses a 63-tap half-band at fixed 2x oversampling around the cubic; changing
it to ADAA is outside this issue and is not recommended by this report.

## 4. Dynamic EQ and de-esser modulation

Dynamic EQ combines a time-varying filter with a detector/envelope, while a split-band de-esser
combines a filter or crossover with attenuation. Those are two precision decisions, not one:

1. Design pole/topology coefficients with deterministic `miso-engine-math` `f64` arithmetic off
   the steady-state render path, then round once to the accepted `f32` coefficient words.
2. Interpolate the accepted SVF coefficient representation in `f32`, preserving the current snap
   and partition law. Wishnick's result supports this representation for time variation [3].
3. Prefer static frequency/Q coefficients plus modulated output mixing or band gain where the
   disclosed topology permits it. Do not evaluate platform transcendental functions per sample.
4. Start with `f32` SVF state. Consume the rank-4 mixed-precision filter result only if its frozen
   gate passes; do not make dynamic EQ an implicit retry of issue #31.
5. Independently consider the rank-3 `f64` envelope state for the slowest accepted ballistics. A
   de-esser with static split filters does not need `f64` filter state merely because its band gain
   changes.

This keeps stability, coefficient-design precision, recurrence precision and detector accuracy
separately measurable. It also keeps `f64` transcendental work at parameter/event rate, where it
has no SIMD-width cost in the main audio loop.

## 5. Delay feedback over hours

The launch delay writes the dry input plus a damped feedback send into two `f32` rings of exactly
`2 Fs + 3` words each. Feedback is bounded by `|g| <= 0.95`. A local error propagated once per
recirculation is therefore bounded by

```text
coherent:  1 / (1 - |g|) = 20       = 26.021 dB
white RMS: 1 / sqrt(1 - g^2) = 3.20 = 10.114 dB
```

At `g = 0.95`, each return loses 0.44553 dB. It takes about 270 returns to fall 120 dB. Session
duration does not change either bound: this is a contractive feedback path, not an accumulator.
With an `f32` unit-roundoff screen, the coherent worst case is -118.47 dB and the white-error screen
is -139.15 dB. The former is close enough to -120 dB to retain a stress row, but not enough to
justify doubling all history before an empirical failure exists.

Changing only arithmetic or damping state to `f64` while writing the ring in `f32` leaves the
dominant history quantization in place. Changing the ring itself adds

```text
2 channels * (2 Fs + 3) words * 4 extra bytes
  =   768,024 bytes at 48 kHz  (0.73 MiB)
  = 1,536,024 bytes at 96 kHz  (1.46 MiB)
```

per prepared delay, doubles ring bandwidth, changes the state layout, and gives no unbounded-time
benefit. Keep `f32`; reconsider only for a future near-unity feedback topology with its own bounded
tail/error and memory gates.

## 6. Cross-target determinism contract for an `f64` kernel

A production `f64` opt-in needs a second lane family, not ad hoc `f64` in scalar code:

- Keep logical lane order fixed. The portable W4 mapping is two ordered `f64x2` packets on Wasm;
  the AVX2 W8 mapping is two ordered `f64x4` packets. Never reduce or reshuffle across packets.
- Define each operation's rounding. `Lane64::fma` must remain a single-rounding operation: use the
  native instruction where it has the required semantics and the exact software form on Wasm
  (projected at roughly 20 Wasm operations per fused site). If a new kernel deliberately specifies
  `mul` then `add`, use that non-fused form on every target. Never silently substitute one for the
  other; prevent contraction and inspect emitted target instructions.
- Promote binary32 inputs exactly. Narrow output at one named boundary with round-to-nearest,
  ties-to-even, then compare `f32::to_bits()` across scalar/W4/W8 and x86-64/AArch64/Wasm.
- If state is serialized, compare every `f64::to_bits()` state word as well. Define byte order and
  increment the owning state-layout version; never reinterpret an old `f32` payload.
- Preserve D7/D8 explicitly: one documented flush boundary for every recursive word, canonical
  `+0.0`, ordered comparisons/selects, and the same non-finite block recovery. Use the current
  numeric flush threshold when crossing back into the `f32` signal domain unless a separate gate
  authorizes another one.
- Keep scalar, fixed sample/channel traversal for long accumulators. A horizontal SIMD reduction
  changes association with width and cannot satisfy bit identity.
- Use the independent `f64` oracle or scalar-lane authority required by the fixture re-pin policy;
  never generate expected fixtures from the new production SIMD output.

The mismatch is physical: 128-bit Wasm SIMD holds two binary64 values, while 256-bit AVX2 holds
four. Packet decomposition is therefore part of the algorithm's determinism proof, not an
implementation detail.

## Adoption gates

No candidate should be introduced from this report alone.

| Candidate | Required gate before production |
|---|---|
| Unbounded statistics | Hours-long constant, alternating-scale and silence sequences; fixed-order scalar/oracle identity; no count overflow. |
| ADAA | Independent spectral/alias corpus plus the cancellation rows above, breakpoint/limit continuity, state restore and cross-target bit identity. |
| Slow envelope | All launch rates and extrema, constant-target fixed points, accepted timing/0.1 dB accuracy, automation partition, restore and scalar/W4/W8 identity. |
| Targeted SVF | The focused low-frequency/high-Q corpus above, all existing analytic/impulse limits unchanged, material residual improvement, no regression, static 2x vector-work ceiling, target instruction audit. |
| Dynamic coefficients | Existing static-response limits, modulation sidebands/continuity, coefficient snap/partition identity and no render-time platform libm. |
| Delay | No adoption. A future topology must first demonstrate a residual failure and pay an explicit per-instance ring-memory/bandwidth budget. |

## References

1. Jon Dattorro, [*Effect Design, Part 1: Reverberator and Other Filters*](https://nagasm.org/ASL/Sketch14/fig5/EffectDesignPart1.pdf), Journal of the Audio Engineering Society 45(9), 1997, especially sections 3.3.2-3.3.5.
2. Engine V2 issue #31, [portable higher-precision built-in filter decision](../../.github/ISSUE_SPECS/031-portable-higher-precision-builtin-filter-quality-mode.md).
3. Aaron Wishnick, [*Time-Varying Filters for Musical Applications*](https://www.dafx14.fau.de/papers/dafx14_aaron_wishnick_time_varying_filters_for_.pdf), DAFx-14, 2014.
4. ITU-R, [Recommendation BS.1770-5: Algorithms to measure audio programme loudness and true-peak audio level](https://www.itu.int/rec/R-REC-BS.1770-5-202311-I/en), 2023.
5. EBU, [R 128: Loudness normalisation and permitted maximum level of audio signals](https://tech.ebu.ch/publications/r128), current revision.
6. Dimitrios Giannoulis, Michael Massberg and Joshua D. Reiss, [*Digital Dynamic Range Compressor Design—A Tutorial and Analysis*](https://eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf), Journal of the Audio Engineering Society 60(6), 2012.
7. Julian D. Parker, Vadim Zavalishin and Efflam Le Bivic, [*Reducing the Aliasing of Nonlinear Waveshaping Using Continuous-Time Convolution*](https://dafx16.vutbr.cz/dafxpapers/20-DAFx-16_paper_41-PN.pdf), DAFx-16, 2016.
8. Stefan Bilbao, Fabián Esqueda, Julian D. Parker and Vesa Välimäki, [*Antiderivative Antialiasing for Memoryless Nonlinearities*](https://research.aalto.fi/files/27135145/ELEC_bilbao_et_al_antiderivative_antialiasing_IEEESPL.pdf), IEEE Signal Processing Letters 24(7), 2017, doi:10.1109/LSP.2017.2675541.
9. Andrew Simper, [*Linear Trapezoidal Integrated State Variable Filter*](https://www.cytomic.com/files/dsp/SvfLinearTrapOptimised2.pdf), Cytomic, 2013/2016.
10. Vadim Zavalishin, [*The Art of VA Filter Design*](https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.2.pdf), revision 2.1.2, Native Instruments.
11. Engine V2 issue #87, [parametric-EQ TPT SVF audit evidence](../../.github/ISSUE_SPECS/087-audit-miso-engine-parametric-eq.md) and its authoritative GitHub issue record.
