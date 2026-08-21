# Sol implementation brief — issue 014 launch hysteretic peak gate/expander

## Decision, authority, and attempt budget

**READY FOR TERRA ATTEMPT 1 after issue synchronization.** Deliver only the product vertical below.
There are two total attempts: one Terra implementation/review and one bounded Sol correction. A
second failure stops. Issue 047 owns expanded qualification; Issue 014 invokes no benchmark.

This brief and `.github/ISSUE_SPECS/014-gate-expander.md` are authoritative. Reuse the accepted
runtime, compressor dynamics conventions, prepared core dispatch, bank and graph seams. Do not
redesign session syntax, effect metadata, cohorts, graph/PDC or target detection.

## Frozen product

```text
effect / contract         miso.gate-expander / 1.0
state layout / quality   1 / Normal only
rates                    44100, 48000, 88200, 96000 Hz
detector / curve         feed-forward instantaneous peak / hard downward expansion
phase                    hysteretic Open or Closed with exact hold counter
links                    DualMono, Maximum, Average
ports                    main-in, main-out, optional sidechain-in; dual-mono planar
latency / tail           Fs/100 samples / Finite(0)
lookahead                0..10 ms per lane; preparation/state only
automation               IDs 1..4 Block Point; exact Linear 64
banking                  unconnected sidechain W4/W8 plus scalar tails
```

Parameters are the exact ordered table in Issue 014. Preparation consumes the complete ordered
sixteen-value L/R table, rejects negative zero, nonfinite or out-of-domain values, creates four
current=target/remaining=0 ramps, and retains IDs 5..8 as preparation/state-only values.
The processor has no signal-generating state: the delay extent is already represented by latency,
and after latency-compensated input exhaustion it emits zero, so the declared tail is finite zero.

## Detector, delay and phase order

For sanitized sources `sL,sR`, link magnitudes exactly:

```text
DualMono: dL=abs(sL),                 dR=abs(sR)
Maximum:  d=max(abs(sL),abs(sR)),     dL=dR=d
Average:  d=0.5*abs(sL)+0.5*abs(sR), dL=dR=d
```

Linking shares only `d`. With no sidechain, `s` is current main input; when connected it is routed
sidechain input. Main and sidechain sanitation counters remain distinct.

At rate `Fs`, fixed latency is `N=Fs/100`, ring length `B=N+1`, and lookahead samples are

```text
L = floor(f64(lookahead_ms)*f64(Fs)/1000 + 0.5), clamped 0..N
D = N-L
```

Each lane owns separate main/detector rings and cursor `w`:

```text
main_ring[w] = main
detector_ring[w] = d
z = main_ring[(w+1) mod B]
u = detector_ring[(w+B-D) mod B]
w = (w+1) mod B
```

Derive `D`, attack/release coefficients and hold samples only at prepare, restore and full reset:

```text
aa = exp_f32(-1.0_f32/(0.001_f32*attack_ms*f32(Fs)))
ar = exp_f32(-1.0_f32/(0.001_f32*release_ms*f32(Fs)))
K  = floor(f64(hold_ms)*f64(Fs)/1000 + 0.5)
```

`exp_f32` is the standard `f32` exponential with the shown `f32` operand graph; it is not a
higher-precision preparation oracle.

The lane starts `Open`, `hold_remaining=K`, `G=+0`. At each sample advance ramps for IDs 1..4 in
descriptor order, sanitize/link, update/read rings, then compute
`X=clamp(20*log10(max(u,1e-8)),-160,24)`. With current threshold `T` and hysteresis `H`:

```text
if phase==Closed:
    if X>=T: phase=Open; hold_remaining=K
else if X>=T-H: hold_remaining=K
else if hold_remaining>0: hold_remaining-=1; remain Open for this sample
else: phase=Closed
```

Thus exactly `K` below-close-threshold samples remain open after the last re-arm; `K=0` closes on
the first such sample. Comparisons at both boundaries are inclusive on the opening/re-arm side.

## Gain computer and sample graph

With ratio `rho`, range `R`, and phase after transition:

```text
C = 0                                      when Open
p0 = (rho-1)*(X-T)                         when Closed
C = clamp(p0,-R,0)                         when Closed
a = aa when C>G_previous, otherwise ar
p1 = a*G_previous
p2 = (1-a)*C
G = p1+p2
A = 10^(0.05*G)
wet = z*A
out = z when G==0, otherwise wet
```

Attack is the move toward open/less attenuation; release is the move toward closed/more
attenuation. Equality selects release. Every production operation is `f32` and separately rounded;
no FMA contraction is allowed. `log10` and `powf` run once per active lane/sample with bounded
standard math. Whole bypass returns `z` bits exactly while warming ramps, rings, phase, hold and
gain. Gain identity also returns `z` bits exactly.

A Block Point for descriptor positions 0..3 at `first_sample` begins update 1 on that sample and
hits the target on update 64; retargeting restarts from current. Only canonical ordered Point spans
with exact lane and domain are accepted. Scan into fixed pending storage, count every invalid span
saturatingly, then apply every valid target in descriptor/lane order.

## Sanitation, recovery and reset

Use the accepted sample sanitizer. Nonfinite or subnormal main/connected-sidechain input becomes
positive zero and increments its input counter once; signed finite audio zero survives rings and
identity. Finite computed subnormal `G`, gain or output flushes to positive zero without recovery.
A nonfinite detector level, coefficient, gain state, gain or output resets only that lane to
`Open`, `hold_remaining=K`, `G=+0`, emits delayed `z`, and increments that lane recovery counter.

`FullToDefaults` clears rings/cursor and restores all eight prepared initial values, derived values,
Open/K and zero gain. `DiscontinuityKeepParameters` clears rings/cursor, sets Open/K and zero gain,
retains IDs 5..8, snaps each ramp to target and clears remaining updates. Metadata never changes.

## Exact payload and resources

Common payload is empty. Each lane is exactly `20+2B` little-endian 32-bit words:

```text
0 cursor u32                 1 lookahead_ms f32
2 G attenuation dB f32      3 phase u32 (0 Closed, 1 Open)
4 hold_remaining u32        5 attack_ms f32
6 hold_ms f32               7 release_ms f32
8..19 four (current f32,target f32,remaining u32) ramps in ID order
next B main-ring f32 words; final B detector-ring f32 words
```

Derived `D/K/aa/ar` and prepared reset defaults are not serialized. Restore accepts layout 1 and
exact lengths, parses both lanes into temporaries, validates `cursor<B`, phase, hold bound,
`G in [-96,0]`, every parameter/ramp domain, `remaining<=64` and finite normal-or-zero rings,
rederives values, then commits both lanes. All preparation-accepted finite parameter values are
state-valid; negative-zero parameter values reject, ring signed zero is legal, and failure changes
neither lane/track.

Exact Normal rows are:

| Fs | N | B | bytes/lane | total state bytes |
|---:|---:|---:|---:|---:|
| 44100 | 441 | 442 | 3616 | 7232 |
| 48000 | 480 | 481 | 3928 | 7856 |
| 88200 | 882 | 883 | 7144 | 14288 |
| 96000 | 960 | 961 | 7768 | 15536 |

Each row declares 64 fixed scratch bytes for the two eight-`f32` reset-default tables and zero
per-frame scratch. Declared retained payload/default bytes are exactly
`2*lane_bytes+64` scalar and `W*(2*lane_bytes+64)` for W4/W8. Cap arithmetic is checked and one
byte below state or scratch rejects before publication.

## Scalar and homogeneous banks

Scalar runs the graph independently for L/R. A bank owns exactly `W` independent per-track `Lane`
states per channel and walks tracks in lane order for scalar detector/phase/gain work. Add one safe
prepared core gain token whose packed per-lane graph is:

```text
p0 = sample*gain
out = sample when identity_mask else p0
```

W4 Wasm/NEON and W8 AVX2 use packed multiply and bit selection. AVX2+FMA aliases the
noncontracting graph with zero FMA sites. Base Wasm does not require relaxed SIMD. Preparation
validates every exact-width request before returning `Ok(None)` for heterogeneous programs,
connected sidechain or unavailable backend. Malformed requests reject; no lane is padded.

## Representative product gates

1. Freeze descriptor order/domains and every rate/resource row; exact caps accept and one byte
   below state/scratch rejects transactionally.
2. An independent test-only `f64` oracle checks representative ratio-identity, threshold,
   close-threshold and range-clamp points to `0.01 dB`; a fixed chatter/hold trace checks exact
   phases, and attack/release crossings meet max(one sample,2%).
3. At all rates, lookahead 0/2/10 enabled and bypass impulses land at `N`; direct link equations
   and one connected-zero-sidechain versus unconnected-main trace are distinct.
4. Prove 64-update/restart, malformed-span coexistence, both resets, all-or-none restore and active
   continuation, signed-zero identity, sanitation, lane-local recovery and L/R/track isolation.
5. Compare available host W8 with eight scalar instances byte-exactly for finite-normal inputs and
   carried state. Core scalar/W4/W8 shape/mask tests freeze the packed graph.
6. Register the effect and compile ten tracks: nine homogeneous unconnected plus one connected
   scalar. Assert width-derived bank/tail counts, member order, fixed latency/tail, unchanged PDC/
   schedule/graph structure, scalar differential and transactional cap ownership return.
7. Run focused formatting/check/tests and warning-denied Clippy plus applicable workspace,
   realtime, effect-runtime, rack and graph policy scans. No audit, target matrix or benchmark runs.

## Deferred qualification and stop rules

Issue 047 alone owns the checked corpus expansion, 10,000/million-sample matrices, cohort/
determinism expansion, 100,000-render audit, native/AArch64/Wasm instruction proof, benchmark and
audition/listening handoff. `timed_benchmark_invocations=0` in Issue 014.

FAIL for RMS/filters/soft knee/ducking, changing latency, connected banking, shared lane state,
unaccounted retained payload, production oracle imports, render allocation/detection, relaxed SIMD,
FMA contraction, tolerance/domain change, broad qualification work or a third attempt.
