# Sol brief — 019 Launch fixed-2x cubic soft-clip saturator

## Decision and boundary

Freeze one useful mode: `miso.soft-clip`, `CubicSoftClip`, Normal quality, fixed 2x, DualMono only.
This follows `[VAIDYANATHAN-MULTIRATE]` for an explicit interpolate/process/decimate path and uses
`[BILBAO-ADAA]` only to justify deferring ADAA until its separate singularity/delay problem is
qualified. Two total attempts are permitted. Issue 052 owns all broad qualification.

## Exact 2x pipeline

Let `gd=10.0_f32.powf(drive_db*0.05_f32)`, `go=10.0_f32.powf(output_db*0.05_f32)` and `m=mix`.
For each base-rate sample, advance the three linear ramps once in ID order, sanitize `x`, and write
`2*gd*x` followed by zero to the
63-word interpolation ring. For each high-rate phase, convolve ascending nonzero tap index with
separate f32 multiply then add, apply

```text
c(u) = -2/3                 u <= -1
       u - u*u*u/3         -1 < u < 1
       +2/3                 u >= 1
```

write `c(u)` to the 63-word decimation ring, convolve in the same order, retain only the even-phase
result, and advance both high-rate rings twice. With `d=x[n-31]`, compute the noncontracting output
in this exact order: `a=1.0_f32-m; b=a*d; c=m*wet; e=b+c; y=go*e`. Exact
mix-zero/unity-output and prepared bypass use bit selection of `d`.

For the cubic interior, use exactly `p0=u*u; p1=p0*u; p2=p1/3.0_f32; y=u-p2`. Each FIR
accumulator starts at positive zero and, for every ascending nonzero tap, computes
`product=h[k]*history[index]` followed by a separate `accumulator=accumulator+product`. These
rounding points are frozen and no contraction is permitted.

`h` is a 63-tap symmetric f32 Blackman-windowed halfband, center 31. Odd indices except 31 and
indices 0/62 are exact zero; `h[62-k]=h[k]`. The unique left/center literals are:

```text
h[0]=0
h[2]= 4.1178966057e-05   h[4]=-1.8436586834e-04
h[6]= 4.7622653074e-04   h[8]=-9.8903989419e-04
h[10]=1.8232578877e-03   h[12]=-3.1101715285e-03
h[14]=5.0172246993e-03   h[16]=-7.7611478046e-03
h[18]=1.1639836244e-02   h[20]=-1.7108557746e-02
h[22]=2.4969698861e-02   h[24]=-3.6900948733e-02
h[26]=5.7263407856e-02   h[28]=-1.0214901716e-01
h[30]=3.1697243452e-01   h[31]=5.0000000000e-01
```

The stored interpolation input is doubled and its convolution uses `h`, so the effective
interpolator response is `2h`; decimation uses `h`. Ascending tap order and noncontraction are part
of the contract. The independent f64 oracle recreates ideal cutoff `pi/2`, the 63-point Blackman window,
scales off-center taps to sum 0.5, fixes center 0.5, then compares the retained f32 literals. It
must not import production tables/code. Per-stage f32 response gates are `+-0.002 dB` through
`0.4Fs` and `<=-75 dB` from `0.6Fs`; expected design extrema are approximately
`[-0.00105,+0.00150] dB` and `-75.28 dB`.

The two FIR group delays total 62 high-rate samples = 31 base samples. Because taps 0/62 are exact
zero, the nonzero tap support is 2..60; the cascade's last retained output is high-rate sample 120,
base sample 60. Report `LatencySamples(31)` and `TailSamples::Finite(29)`: graph extent is latency
plus tail. Enabled impulse support may begin before its linear-phase peak; PDC aligns
the declared group delay. Bypass and mix-zero delay dry by 31 while warming wet state.

## Descriptor, automation and state

Contract/state layout 1, main-in/main-out, no sidechain, `LinkModeSet::DUAL_MONO`, four launch-rate
Normal rows. Ordered required PerLane parameters:

| ID | name | unit/domain/default | mapping | automation/smoothing |
|---:|---|---|---|---|
| 1 | drive | dB `[-24,36]`, `0` | Linear | Block Point / linear-gain 64 |
| 2 | output | dB `[-24,24]`, `0` | Linear | Block Point / linear-gain 64 |
| 3 | mix | Linear `[0,1]`, `1` | Linear | Block Point / linear 64 |

Only canonical ordered Point spans and Left/Right lanes are accepted. Reject Both, other kinds,
duplicates, disorder, invalid domains and excess capacity while retaining other valid targets;
apply targets in ID/lane order. The first update advances by `(target-current)/64`; the 64th is the
target. New points restart from current. Normalize accepted parameter zero positive.

Each lane is exactly 169 little-endian words:

```text
0 high-rate ring cursor u32; 1 dry cursor u32
2..10 three (current f32,target f32,remaining u32) ramps in ID order
11..73 interpolation history[63]
74..136 decimation history[63]
137..168 dry history[32]
```

Thus state is 676 bytes/lane, 1,352/track. Retained reset defaults are six f32 = 24 fixed scratch
bytes/track; per-frame scratch is zero. Bank retained bytes are `W*(1352+24)`: W4 5,504, W8 11,008.
Every product/count/conversion/allocation is checked; exact caps pass and one byte below each state,
scratch, single-allocation and post-bank plan cap rejects transactionally.

Restore parses both lanes into temporaries, requires exact layout/length, cursors `<63`/`<32`, ramp
values within converted domains, remaining `<=64`, finite normal-or-zero histories (signed zero
allowed), then commits both lanes. Full reset restores prepared values; discontinuity reset retains
targets, snaps ramps and clears histories/cursors.

## Safety, banks and graph

Use the accepted sanitizer: nonfinite/subnormal input becomes positive zero and counts once; finite
signed zero survives. Flush computed finite subnormals. Any nonfinite ramp/history/intermediate or
output clears only that lane's histories, snaps its ramps to valid targets, emits delayed dry and
counts one recovery; other lanes/tracks continue.

W4/W8 store sample-major AoSoA histories and execute the exact scalar tap/polynomial order with
multiply plus add and zero FMA sites on base, AVX2 and AVX2+FMA selections. Masks are exact; absent
lanes are identity and warm no foreign state. Validate quality/rate/quantum/count/width/program,
every scalar metadata/payload shape and total retained cap before returning legal unavailable.

The ten-track launch fixture uses only unconnected homogeneous `miso.soft-clip` slots. It proves
host-width banks plus ordered scalar tails, scalar-delegate PCM across active smoothing/state/tail,
exact 31-sample group delay, finite tail 29 and causal support through sample 60,
bypass/PDC/canonical stability, exact bank accounting
and one-byte-below ownership return.

## Representative closure

Freeze and execute the six issue gates verbatim. The alias row uses N=16,384, bin 3001, unit sine,
+18 dB drive, output 0 dB, mix 1, three complete warm periods, rectangular DFT, and compares the
sum of every non-DC/non-fundamental bin against fundamental energy. The same independent f64 curve
at 1x is the baseline; the fixed-2x result must improve by >=2.0 dB, with both ratios serialized.
No normalization, window, bin exclusion beyond DC/fundamental, or post-filter is permitted.

Issue 052 owns other modes/factors, expanded tones/multitones/rates/drives, 10k/million long rows,
realtime audit, target/instruction proof, the sole future benchmark and listening. Product benchmark
count is zero. Any changed domain/table/phase/tolerance, new framework, or second failed attempt is
a STOP/rebrief, never a relaxed PASS.
