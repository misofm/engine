# Sol implementation brief — issue 020 launch dual-envelope transient shaper

## Decision, authority and attempt budget

**READY FOR TERRA ATTEMPT 1.** Deliver exactly the bounded product below. There are two total
attempts: one Terra implementation/review and one bounded Sol correction/review. A second failure
stops. Issue 054 owns broad qualification. Issue 020 invokes no benchmark and never inspects V1.

This brief and `.github/ISSUE_SPECS/020-transient-shaper.md` are authoritative. Reuse the accepted
effect runtime, Issue-013 dynamics conventions, `PreparedCompressorGainMixKernelV1`, bank ownership,
launch registry/effect compiler and graph/PDC seams. Do not create a new core SIMD framework.

## Frozen product

```text
effect / contract          miso.transient-shaper / 1.0
state layout / quality     1 / Normal only
rates                      44100, 48000, 88200, 96000 Hz
detector                   instantaneous absolute peak; fixed fast/slow followers
links                      DualMono, Maximum, Average
ports                      required main-in/main-out; dual-mono; no sidechain
lookahead / latency        none / 0 samples
tail                       Finite(0)
shape range                fixed -18..18 dB
automation                 three Block Points; exact Linear 64
banking                    W4/W8 packed gain/mix plus scalar tails
```

No hidden detector mode, lookahead, adaptive timing, sensitivity, auto gain, quality or sidechain
exists. Those are separate product decisions, not disabled parameters.

## Descriptor and preparation

Create `miso-engine-transient-shaper` / `miso_engine_transient_shaper` and
`TransientShaperFactory`. The descriptor is contract 1.0, layout 1, `LinkModeSet::ALL`, Normal-only,
with ordered required `main-in`/`main-out` `DualMonoPlanar` ports. The exact parameters are:

| index / ID | name | unit/display | domain/default | mapping | rate/smoothing |
|---:|---|---|---|---|---|
| 0 / 1 | attack amount | Linear / `%` | -1..1 / 0 | Linear | Block / Linear 64 |
| 1 / 2 | sustain amount | Linear / `%` | -1..1 / 0 | Linear | Block / Linear 64 |
| 2 / 3 | mix | Linear / linear | 0..1 / 1 | Linear | Block / Linear 64 |

All are readable, automatable and `PerLane`; descriptor position, state-ramp order and ID-minus-one
are identical. Preparation consumes exactly six ordered L/R values, rejects missing/extra,
nonfinite or out-of-domain values and normalizes accepted numeric zero to positive zero. Each lane
starts with `current=target`, `remaining=0`. Quality rows at all four rates declare latency 0,
`TailSamples::Finite(0)`, state `{common:0,left:44,right:44}`, `scratch_fixed_bytes=24` and
`scratch_bytes_per_frame=0`. Quantum does not affect state/resources.

## Frozen detector coefficients and linking

The fixed time constants are fast attack 0.5 ms, fast release 20 ms, slow attack 10 ms and slow
release 100 ms. The table is the authoritative retained `f32` coefficient bit pattern, computed as
the correctly rounded cast of `exp(-1/(0.001*tau_ms*Fs))`:

| Fs | fast attack | fast release | slow attack | slow release |
|---:|---:|---:|---:|---:|
| 44100 | `0x3f74a63c` | `0x3f7fb5bd` | `0x3f7f6b90` | `0x3f7ff124` |
| 48000 | `0x3f758d71` | `0x3f7fbbc5` | `0x3f7f779c` | `0x3f7ff259` |
| 88200 | `0x3f7a42a5` | `0x3f7fdadc` | `0x3f7fb5bd` | `0x3f7ff892` |
| 96000 | `0x3f7ab8ca` | `0x3f7fdde0` | `0x3f7fbbc5` | `0x3f7ff92c` |

Reject an unsupported rate before publication. Coefficients are table values, not render-time
transcendentals or serialized state. After sanitizing both current input samples, compute detector
magnitudes before updating either lane:

```text
DualMono: dL=abs(xL),                 dR=abs(xR)
Maximum:  d=max(abs(xL),abs(xR)),     dL=dR=d
Average:  d=0.5*abs(xL)+0.5*abs(xR), dL=dR=d
```

Average is two separately rounded half-products then one add. Only the magnitude links. Each lane
retains its own followers, parameters, state, output and recovery.

## Exact scalar sample graph

For each frame, first advance the three ramps in descriptor order, sanitize both audio inputs,
compute linked detector magnitudes, then process Left followed by Right. For lane input `x`, detector
`u`, previous envelopes `fp,sp`, and current parameters `A,S,M`:

```text
af = fast_attack  when u>fp else fast_release
as = slow_attack  when u>sp else slow_release

f0 = af*fp
f1 = (1-af)*u
f  = f0+f1
s0 = as*sp
s1 = (1-as)*u
s  = s0+s1

lf = 20*log10(max(f,1.0e-8))
ls = 20*log10(max(s,1.0e-8))
c0 = lf-ls
c  = clamp(c0,-24,24)
ta = max(c,0)
ts = max(-c,0)
p0 = A*ta
p1 = S*ts
q0 = p0+p1
q  = clamp(q0,-18,18); normalize numeric zero to +0
g0 = q*0.05
g  = 10^g0
wet = x*g
delta = wet-x
scaled = M*delta
mixed = x+scaled
```

Every shown multiply/add/subtract is a separately rounded `f32` operation. Comparisons are strict;
equality selects release. `log10` and `powf` are bounded standard `f32` calls; production imports no
reference code. Positive attack amount boosts positive contrast, positive sustain amount boosts
negative contrast; negative values cut the corresponding region. There is no output clipping.

Prepared bypass, numeric-zero mix or numeric-zero `q` returns `x` bits exactly while still updating
ramps and both followers. Mix one returns `wet` bits exactly. Otherwise return `mixed`. Thus default
amounts, silence and signed-zero identity are exact. Latency is zero. Because the processor only
multiplies current input and produces exact zero for zero input, the audio tail is `Finite(0)` even
though follower state decays internally.

## Automation, sanitation, reset and recovery

Accept only canonical Block `Point` spans at `first_sample`, with equal start/end samples and
bit-equal values, for descriptor positions 0..2 and exact Left/Right channels. Reject Both,
out-of-range positions, other kinds, duplicates, disorder, excess capacity and domain errors.
Scan into a fixed pending table, saturating-count every invalid span, retain other valid targets and
apply valid targets in stable descriptor/lane order. A Point begins update one at `first_sample`,
reaches the target on update 64 and retargets from current. Normalize accepted numeric zero.

Use the accepted `sanitize_sample`: nonfinite or subnormal input becomes positive zero and
increments that lane's saturating sample counter once; finite signed zero remains available to
identity output. Finite subnormal follower or computed nonidentity output becomes positive zero
without recovery. A nonfinite follower, contrast, gain, wet or mixed value clears only that lane's
two followers, emits sanitized `x`, and increments its recovery counter once for the host sample.
Ramps continue; the other lane/track is untouched. No valid bounded fixture may recover.

`FullToDefaults` clears followers and restores the six prepared reset defaults with no active ramp.
`DiscontinuityKeepParameters` clears followers, retains each target, snaps current to target and
sets remaining to zero. Both leave immutable metadata unchanged.

## Exact state, restore and resources

Common payload is empty. Each lane is exactly 11 little-endian 32-bit words / 44 bytes:

```text
0 fast envelope f32
1 slow envelope f32
2..10 three (current f32, target f32, remaining u32) ramps in descriptor order
```

Snapshot requires exact output lengths and writes every byte. Restore accepts layout 1 and exact
sections, parses both lanes into unpublished temporaries, requires envelopes finite normal-or-
positive-zero and nonnegative, every current/target finite and in its descriptor domain, and every
remaining `<=64`; numeric-zero parameters normalize positive. Commit both lanes only after the
complete validation. Invalid common/trailing bytes or either corrupt lane changes neither lane.
Scalar and bank per-track payloads are byte-compatible; failed bank-track restore changes no track.

The descriptor declares 88 total state bytes and 24 fixed bytes for two retained three-value reset
tables. Exact effect-owned retained envelopes are:

| execution | retained bytes |
|---|---:|
| scalar track | `88+24 = 112` |
| W4 bank | `4*112 = 448` |
| W8 bank | `8*112 = 896` |

All checked conversions/arithmetic precede allocation. Exact caps pass; one byte below state,
fixed, bank, plan or largest-single-allocation caps rejects transactionally and returns ownership.
Object/vtable/allocator headers are not state or declared fixed bytes.

## Scalar, W4/W8 and graph closure

Scalar executes the graph above. A homogeneous bank retains `W` independent track/lane states and
walks tracks in ascending lane order for link, follower, logarithm and gain calculation. Audio is
sample-major AoSoA. Apply one packed gain/mix call per channel/sample through the existing
`PreparedCompressorGainMixKernelV1` using exact dry/wet masks from the identity rules. Do not add a
transient-specific core kernel.

W4 is base Wasm `simd128` or AArch64 NEON; W8 is AVX2. AVX2+FMA aliases the accepted noncontracting
gain/mix graph and permits zero contractions. For finite-normal, no-sanitation/recovery inputs,
same-target scalar and available W4/W8 output, complete state and reports are bit-identical. Legal
unavailable backends return `Ok(None)` only after exact width/backend/count, member ownership,
program/quality/link/ports, initial values and resource caps validate. Never pad tracks or impose a
track ceiling; scalar tails retain order and isolation.

Append the factory to the caller-owned launch registry and exact effect-compiler dependency policy.
The accepted 48-kHz/q128 ten-track fixture uses one homogeneous program and distinct per-track
amounts: W8 retains `ts0..ts7` plus scalar `ts8,ts9`; W4 retains two banks plus two tails; otherwise
all ten are scalar. Prove stable membership and consecutive scalar-delegate PCM/state, enabled and
bypass zero latency/zero PDC/canonical stability, exact corrected post-bank scratch/runtime/
metadata accounting, and one-byte-below plan rejection with all ten owners returned.

## Representative product closure and stop rules

1. Freeze descriptor/coefficient bits/state/resource rows and transactional mutation/cap results at
   all launch rates.
2. Use an independent `f64` implementation derived from time constants, not production bits/types,
   for impulse, level step and decaying burst. Prove contrast sign, active positive/negative attack
   and sustain, maximum gain error `<=0.01 dB`, and time-constant error within the greater of one
   sample or 2%.
3. Prove default/bypass/mix-zero/zero-shape bits, signed zero, link distinctions, exact ramp
   updates 1/63/64 and retarget, both resets, active restore continuation, sanitation, injected
   scalar/W8 recovery parity and L/R/track isolation.
4. Prove exact W4/W8 resources, validation before fallback, native available W8 bit parity, scalar
   tails, ten-track registry/graph/PDC/cap transaction and zero render allocation in focused tests.
5. Run focused then one clean locked workspace format/check/test, warning-denied Clippy/rustdoc and
   applicable workspace/realtime/effect-runtime/rack/graph policies and mutations.

Issue 054 alone owns corpus expansion, exact 10,000/million-sample rows, expanded cohorts,
100,000-render audit, target/instruction evidence, benchmark and listening. A second algorithm,
mode/quality, lookahead, new core kernel, changed domain/tolerance or second failed attempt stops
Issue 020. Record exact evidence, attempt number, strict verdict and
`timed_benchmark_invocations=0`.
