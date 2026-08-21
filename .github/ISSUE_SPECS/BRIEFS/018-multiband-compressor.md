# Sol implementation brief — issue 018 launch two-band LR4 multiband compressor

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** Deliver exactly the two-band product below. There are two total
attempts: one Terra implementation/review and one bounded Sol correction/review. A second failure
stops. Issue 051 owns broad qualification. Issue 018 invokes no benchmark and never inspects V1.

This brief and `.github/ISSUE_SPECS/018-multiband-compressor.md` are authoritative. Reuse the
accepted effect runtime, Issue 013 compressor conventions, conditioned builtin TPT recurrence,
prepared core kernels, bank and graph/PDC seams. Do not create a general crossover/band framework
or depend on Issue 045.

## Frozen product and parameters

```text
effect / contract          miso.multiband-compressor / 1.0
state layout / quality    1 / Normal only
rates                     44100, 48000, 88200, 96000 Hz
bands / crossover         fixed two / fourth-order Linkwitz-Riley IIR
phase / crossover delay   nonlinear all-pass sum / zero samples
detector / knee           feed-forward instantaneous peak / fixed 6 dB
links                     DualMono, Maximum, Average, applied per band
ports                     required main-in/main-out; dual-mono; no sidechain
latency / tail            Fs/50 samples / Infinite
lookahead                 one 0..20 ms value per lane, shared by both bands
automation                ten dynamics IDs; Block Point; exact Linear 64
banking                   W4/W8 plus scalar tails
```

Create `miso-engine-multiband-compressor` / `miso_engine_multiband_compressor`. Descriptor positions
and stable IDs are identical:

| index/ID | control | domain/default | mapping | rate/smoothing |
|---:|---|---|---|---|
| 0 / 1 | crossover Hz | 80..8000 / 1000 | Logarithmic | None / None |
| 1 / 2 | lookahead ms | 0..20 / 5 | Linear | None / None |
| 2 / 3 | low threshold dB | -80..0 / -18 | Linear | Block / Linear 64 |
| 3 / 4 | low ratio | 1..20 / 4 | Logarithmic | Block / Linear 64 |
| 4 / 5 | low attack ms | 0.1..200 / 10 | Logarithmic | Block / Linear 64 |
| 5 / 6 | low release ms | 5..5000 / 100 | Logarithmic | Block / Linear 64 |
| 6 / 7 | low makeup dB | -24..24 / 0 | Linear | Block / Linear 64 |
| 7 / 8 | high threshold dB | -80..0 / -18 | Linear | Block / Linear 64 |
| 8 / 9 | high ratio | 1..20 / 4 | Logarithmic | Block / Linear 64 |
| 9 / 10 | high attack ms | 0.1..200 / 10 | Logarithmic | Block / Linear 64 |
| 10 / 11 | high release ms | 5..5000 / 100 | Logarithmic | Block / Linear 64 |
| 11 / 12 | high makeup dB | -24..24 / 0 | Linear | Block / Linear 64 |

All are readable and `PerLane`. Preparation consumes the complete ordered 24-value L/R table,
rejects negative zero/nonfinite/missing/duplicate/out-of-domain values, and initializes ten ramps
with `current=target`, `remaining=0`. Crossover/lookahead changes require reprepare or compatible
restore and never appear in automation.

At rate `Fs`, latency `N=Fs/50`, ring length `B=N+1`, and detector delay are:

```text
L = floor(f64(lookahead_ms)*f64(Fs)/1000 + 0.5), clamped to 0..N
D = N-L
```

Reported latency is always `N`. The IIR crossover and compressor release require Infinite tail.

## Exact conditioned LR4 crossover

Prepare one coefficient set per lane in `f64`, cast retained values once to `f32`, then run the
accepted stability and cast-response checks. With `k=sqrt(2)`, `g=tan(pi*f/Fs)`:

```text
t0=g+k; t1=g*t0; den=1+t1
c1=t1/den; a2=g/den; a3=(g*g)/den
```

One conditioned section, with each operation separately rounded in `f32`, is:

```text
v3=x-s2
p1=a2*v3; p2=c1*s1; d1=p1-p2; v1=s1+d1
p3=a2*s1; p4=a3*v3; d2=p3+p4; v2=s2+d2
n1=s1+(d1+d1); n2=s2+(d2+d2)
low=v2; high=(x-k*v1)-v2; s1=n1; s2=n2
```

The LR4 paths use four independent section states and exact order:

```text
low  = LP2_b(LP2_a(x))
high = HP2_b(HP2_a(x))
```

Never share the first LP/HP section. Analytically, `LP4+HP4` is unit-magnitude all-pass; each branch
is `-6.020599913 dB` at crossover. It is not zero-phase or sample-identical dry.

## Rings, linking and dynamics

Each lane owns dry, low and high rings of `B` samples and cursor `w`. Sanitize input, process both
crossover paths, then:

```text
dry[w]=x; low_ring[w]=low; high_ring[w]=high
z_dry= dry[(w+1) mod B]
z_band=band[(w+1) mod B]
q_band=band[(w+B-D) mod B]
w=(w+1) mod B
```

The band ring supplies the detector because V1 has no sidechain. For each corresponding band:

```text
DualMono: dL=abs(qL),                 dR=abs(qR)
Maximum:  d=max(abs(qL),abs(qR)),     dL=dR=d
Average:  d=0.5*abs(qL)+0.5*abs(qR), dL=dR=d
```

Average is two half-products and one add. Only `d` links; all state remains lane-local.

Reuse Issue 013's exact `f32` soft-knee curve with fixed `W=6`, detector floor `1e-8`, level clamp
`[-160,24]` and reduction clamp `[-100,0]`. Strictly select attack when `C<G_previous`; equality
selects release. Per band:

```text
aa=exp(-1/(0.001*attack_ms*Fs)); ar=exp(-1/(0.001*release_ms*Fs))
a=aa if C<G_previous else ar
G=(a*G_previous)+((1-a)*C)       // two multiplies then add
A=10^((G+makeup_db)*0.05)
wet_band=z_band*A
active=wet_low+wet_high
```

Standard bounded `f32` `log10`/`exp`/`powf` run lane-locally; production imports no oracle.
Advance ten ramps in descriptor order. A Point starts update one at `first_sample`, reaches its
target on update 64 and retargets from current.

Bypass returns `z_dry` bits while warming all state. When both `G` values and makeups are positive
zero, also return `z_dry` bits exactly. Otherwise return low plus high. Raw crossover-sum tests run
before this identity selection.

## Automation, reset, recovery and restore

Accept only canonical ordered Block Points for positions 2..11 and exact Left/Right lanes. Reject
Both, preparation-only/out-of-range positions, other kinds, duplicates, disorder, excess capacity
and invalid values. Count each invalid span saturatingly, retain other valid targets, apply stable
descriptor/lane order and normalize accepted parameter zero positive.

The accepted sanitizer converts nonfinite/subnormal input to positive zero and counts once; finite
signed audio zero survives rings/identity. Flush finite computed subnormal state/gain/output without
recovery. A nonfinite crossover state/output, curve, coefficient, `G`, gain or active output resets
only that lane's four sections and two gains, emits `z_dry` and counts one lane recovery. Rings and
other lanes/tracks remain untouched; valid fixtures recover zero lanes.

`FullToDefaults` clears rings/cursor/filter/gains and restores all prepared initial values.
`DiscontinuityKeepParameters` clears rings/cursor/filter/gains, retains crossover/lookahead, snaps
ramps to targets and clears remaining updates. Metadata is immutable.

## Exact state and resources

Common payload is empty. Each lane is exactly `43+3B` little-endian words:

```text
0 cursor u32
1 crossover_hz f32; 2 lookahead_ms f32
3 low G f32; 4 high G f32
5..34 ten (current f32,target f32,remaining u32) ramps in descriptor order
35..42 (s1 f32,s2 f32) for LP-a, LP-b, HP-a, HP-b
next B dry ring; next B low ring; final B high ring
```

| Fs | N | B | bytes/lane | total state bytes |
|---:|---:|---:|---:|---:|
| 44100 | 882 | 883 | 10768 | 21536 |
| 48000 | 960 | 961 | 11704 | 23408 |
| 88200 | 1764 | 1765 | 21352 | 42704 |
| 96000 | 1920 | 1921 | 23224 | 46448 |

Freeze `scratch_fixed_bytes=136`: per lane, 48 bytes for twelve reset defaults and 20 bytes for
retained `c1,a2,a3,k,D`; `scratch_bytes_per_frame=0`. A bank retains exactly
`W*(state_bytes+136)` declared bytes. Object/vtable/allocator headers are not payload/scratch.
Check every arithmetic/conversion before allocation. Exact caps pass and one-byte-below state,
scratch, plan and single-allocation caps reject transactionally with ownership returned.

Snapshot is deterministic. Restore accepts layout 1/exact lengths, parses both lanes into
temporaries, validates cursor, every parameter, negative zero, `G in [-100,0]`, `remaining<=64`,
and finite normal-or-zero filter/ring words, rederives coefficients/`D`, then atomically commits.
Signed zero is legal in filter/ring audio state. Scalar and bank track payloads are byte-compatible.

## Scalar, W4/W8 and graph

Scalar uses the frozen section/gain graphs. A bank owns independent track/lane payloads, processes
the four sections with `PreparedTptBankKernelV1`, and applies two gains through
`PreparedCompressorGainMixKernelV1` with `mix=1` and exact masks. Audio remains sample-major AoSoA;
lane-local transcendental/link work walks ascending tracks. Scalar tails cover all counts.

W4 is Wasm base `simd128` or AArch64 NEON; W8 is AVX2. AVX2+FMA aliases noncontracting kernels:
zero TPT/compressor FMA contractions. Feature detection/function preparation stays off render.

The ten-track fixture asserts host-width-correct full banks and scalar tails, not a hard-coded bank
count on every host. Registry, compiler, schedule, PDC, bypass latency, observer boundaries and
graph shape remain stable. Post-bank cap failure returns all effect/source owners.

## Representative closure and stop rules

1. Exact descriptors, four-rate state rows and one-byte-below caps.
2. Independent LP4/HP4/crossing/raw-all-pass checks at 80/1000/8000 Hz.
3. Ratio identity and active low/high/both curve/envelope tests rendered beyond latency.
4. Lookahead 0/5/20, bypass/identity latency, links and asymmetric dual-mono.
5. Automation restart/update 64, both resets, active restore, signed zero, sanitation, injected
   recovery and isolation.
6. Scalar/W4/W8 parity, width-correct ten-track graph/PDC/cap transaction, focused format/tests/
   Clippy and relevant policies.

Issue 051 owns expanded matrices, long rows, realtime audit, complete targets/instructions,
benchmark and listening. A third band, alternate topology, new core framework, sidechain, changed
domain/tolerance or second failed attempt stops Issue 018. Record exact evidence, attempt number,
strict verdict and `timed_benchmark_invocations=0`.
