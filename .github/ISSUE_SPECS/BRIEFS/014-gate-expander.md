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
state layout / quality   2 / Normal only
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

Each channel owns a main ring, and a detector ring **only when a sidechain is connected**; a bank
requires an unconnected sidechain, so a bank has two rings and not four (amended by #89, finding
F4: the detector ring was a byte-for-byte duplicate of the main one in the only bankable
configuration). The rings hold raw samples, not linked magnitudes: the link is applied after the
tap, which is the same value because both channels tap at the reading channel's own delay.

Slots are `S = next_power_of_two(N+1)` per lane, laid out AoSoA (`slot * WIDTH + lane`), and the
cursor `w` is a wrapping `u32` shared by both channels of an instance. There is no `%` and no `/`
on a render path (amended by #89, finding F3):

```text
main_ring[w & (S-1)] = main
detector_ring[w & (S-1)] = sidechain          when connected
z = main_ring[(w-N) & (S-1)]
u = link(src_c[(w-D_c) & (S-1)], src_p[(w-D_c) & (S-1)])
w = w+1                                       wrapping
```

`S` divides `2^32`, so the wrap is exact; `S >= N+1` is what keeps the slot written this sample
from clobbering the slot read this sample.

Derive `D`, attack/release coefficients and hold samples only at prepare, restore and full reset:

```text
ba = 1 - exp(-1/(0.001*attack_ms*Fs))
br = 1 - exp(-1/(0.001*release_ms*Fs))
K  = floor(f64(hold_ms)*f64(Fs)/1000 + 0.5)
```

Amended by #89: these are `miso_engine_effect_runtime::envelope::attack_release_coefficient`, the
one-pole **rate** coefficient rather than the retention coefficient, evaluated through
`miso_engine_math::expf` — the engine's own deterministic exponential, not the platform's, so the
same bits are designed on every target (decision D6). It is a control-plane function: it runs at
prepare, restore and full reset, never per sample.

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

Amended by #89: the transition is realised branchlessly (decision D10 — no data-dependent branch on
a render path), with `phase` and `hold_remaining` carried as `f32` lane words in
`miso_engine_effect_runtime::envelope::HysteresisState`. The equations above are unchanged and are
what the branchless form computes; the runtime's own `hysteresis_step` is deliberately *not* used,
because it opens on `X > T` and reloads the hold only on an opening trigger, and both differences
are audible against the text above.

## Gain computer and sample graph

With ratio `rho`, range `R`, and phase after transition:

```text
C = 0                                      when Open
p0 = (rho-1)*(X-T)                         when Closed
C = clamp(p0,-R,0)                         when Closed
b = ba when C>G_previous, otherwise br
G = flush(fma(b, C-G_previous, G_previous))
A = exp2(G * log2(10)/20)
wet = z*A
out = z when G==0, otherwise wet
```

Amended by #89. Three changes to the graph above, each frozen:

* **`X` and `A` are `log2`/`exp2` realisations.** `X = clamp(log2_lane(max(u,1e-8)) * 20*log10(2),
  -160, 24)` and `A = exp2_lane(G * log2(10)/20)`, both from `miso-engine-math` (decision D6). The
  platform `log10` and `powf` are gone: they are not specified to agree between targets, and while
  they were on this path native and wasm could not be bit-identical whenever the gate attenuated.
* **The one-pole is one rounding, not two.** `G = G + b*(C-G)` written as a single
  `Lane::fma` (decision D3), where it was `a*G + (1-a)*C`. `b` is the rate coefficient `1-a`.
* **The `flush` is the one denormal mechanism.** `G` is the only recursive word and the only one
  flushed, with `FLUSH_EPS = 1e-20` (decision D7). It is invisible in output bits — inside the
  flush band `exp2(G)` already rounds to exactly `1.0` and `z*1.0 == z` — and its effect is to keep
  the *stored* state word canonical `+0` so that a target with hardware FTZ (an AudioWorklet on
  Chrome) and a native render carry the same state.

Attack is the move toward open/less attenuation; release is the move toward closed/more
attenuation. Equality selects release. Every other production operation is `f32` and separately
rounded; the `fma` above is the only fused site, and it is written, never contracted. Every
`max`/`min` is the D8 select form, never `f32::max`. Whole bypass returns `z` bits exactly while
warming ramps, rings, phase, hold and gain. Gain identity also returns `z` bits exactly.

Native and wasm are bit-identical by construction rather than by tolerance: the only operations on
this path are IEEE basic arithmetic, `Lane::fma` (the exact software FMA on wasm) and the two
`miso-engine-math` polynomials, and `scripts/run-wasm-gates.sh` digests the whole graph on both.

A Block Point for descriptor positions 0..3 at `first_sample` begins update 1 on that sample and
hits the target on update 64; retargeting restarts from current. Amended by #89 (decision D11): the
per-sample division is gone. `step = (target-current)/64` is computed once, at the event, by
`miso_engine_effect_runtime::ramp::LinearRamp::set_target`; the render path adds `step` and assigns
`target` exactly on update 64. Only canonical ordered Point spans
with exact lane and domain are accepted. Scan into fixed pending storage, count every invalid span
saturatingly, then apply every valid target in descriptor/lane order.

## Boundary check, recovery and reset

**Amended by #89 (decision D7).** There is no per-sample input sanitiser and no per-intermediate
finiteness check. `sanitized_main_samples` and `sanitized_sidechain_samples` therefore stay zero:
inputs are sanitised at the input stage, and a non-finite sidechain reads as silence here
(`max(NaN,1e-8)` is `1e-8` under the D8 select, giving `X = -160`) and carries through the
detector ring for at most `N` samples without a report. Signed finite audio zero still survives the
rings and the identity select, because `z` is a ring word and is never flushed.

Instead, once per block per channel, the output block and the `G` lane words are scanned with
`miso_engine_effect_runtime::bank::check_block` — `!(|x| < 1e30)`, which is exactly "NaN or at
least 1e30". The gain words are scanned *as well as* the block because `exp2_lane` clamps its
argument with the D8 `max`/`min`, and those swallow NaN, so a NaN `G` produces finite output that
scanning the block alone would never see.

A failing block resets **only the failing lanes of the failing channel** to `Open`,
`hold_remaining=K`, `G=+0` with resting ramps, zeroes that lane's column of that channel's block
and of its rings, and adds the block's frame count to that track's `recovered_left_samples` or
`recovered_right_samples`. The other lanes, the other channel and the other tracks are
bit-unchanged. This supersedes issue 048's "preserve advanced finite rings on recovery": under D7 a
lane reset clears the rings, because a ring that carried the fault is not a history worth keeping.

`FullToDefaults` clears rings/cursor and restores all eight prepared initial values, derived values,
Open/K and zero gain. `DiscontinuityKeepParameters` clears rings/cursor, sets Open/K and zero gain,
retains IDs 5..8, snaps each ramp to target and clears remaining updates. Metadata never changes.

## Exact payload and resources — state layout 2

**Amended by #89.** The common section is the two-word header of
`miso_engine_effect_runtime::state_payload` — version, then data word count — so a payload states
its own layout instead of taking the caller's word for it. Each channel section is exactly `23+2N`
little-endian 32-bit words:

```text
0 G attenuation dB f32 (normal-or-zero, [-96,0])
1 phase f32 (bits 0x00000000 Closed or 0x3F800000 Open)
2 hold_remaining f32 (integer-valued, 0..=K)
3 lookahead_ms f32          4 attack_ms f32
5 hold_ms f32               6 release_ms f32
7..22 four (current f32, target f32, step f32, remaining f32) ramps in ID order
next N main-ring f32 words; final N detector-ring f32 words (all bits zero when unconnected)
```

The rings are **cursor-normalised**: word `j` is the sample written `N-j` samples ago, so a track
snapshotted at one ring position restores into a bank whose shared cursor is at another as a
re-indexing copy, leaving the other tracks untouched. There is no cursor word, and the ring section
is `N` words rather than `B = N+1`: the slot at the cursor is dead at a block boundary.

What layout 1 carried and layout 2 does not: the physical cursor (normalised away), the `u32` phase
and hold words (now the `f32` lane words the kernel actually holds), and three-word ramps (now four,
carrying the precomputed D11 step). Derived `D/K/ba/br` and prepared reset defaults are still not
serialised. Restore accepts layout **2** and exact lengths, checks the header's own version and word
count, parses both channels into temporaries, validates phase, hold bound, `G in [-96,0]`, every
parameter/ramp domain, `remaining<=64`, `step` normal-or-zero and exactly zero bits when the ramp
rests, finite normal-or-zero rings and all-zero detector words when unconnected, rederives values,
then commits both channels. Negative zero is accepted as a way of writing zero and normalised to
`+0` (83c decision 3), except in a parameter value, where it still rejects. Ring signed zero is
legal, and failure changes neither channel nor track.

Exact Normal rows are:

| Fs | N | S | bytes/channel | common bytes | total state bytes |
|---:|---:|---:|---:|---:|---:|
| 44100 | 441 | 512 | 3620 | 8 | 7248 |
| 48000 | 480 | 512 | 3932 | 8 | 7872 |
| 88200 | 882 | 1024 | 7148 | 8 | 14304 |
| 96000 | 960 | 1024 | 7772 | 8 | 15552 |

Each row declares 64 fixed scratch bytes and zero per-frame scratch. Declared retained
payload/default bytes are exactly `2*channel_bytes+8+64` scalar and `W*(2*channel_bytes+8+64)` for
W4/W8. Cap arithmetic is checked and one byte below state or scratch rejects before publication.
No persisted layout-1 gate payload exists (the engine is pre-launch), so no migration step is
registered; if one ever does, `miso-engine-effect-compiler`'s `StateMigrationRegistryV1` is where a
`1 -> 2` edge belongs.

## Scalar and homogeneous banks

**Amended by #89.** There is no scalar-per-track walk and no prepared core gain token. One generic
body, `gate_block<L, CONNECTED, RAMPING>`, owns the whole per-sample graph and is instantiated at
`f32` (`WIDTH = 1`), `Simd4` and `Simd8`; the scalar instance *is* the `WIDTH = 1` instantiation, so
a bank and `W` scalar instances agree by construction rather than by a fixture, and there is no
per-sample call boundary (decision D10). The only per-lane scalar code inside the body is the
detector gather, which is loads and stores with no arithmetic, because lookahead is a `PerLane`
parameter and each lane taps its own slot.

There is no runtime SIMD dispatch (decision D4, revision 4): `wide` picks its instruction set at
compile time, the workspace pins native x86 to `x86-64-v3`, and a bank is available exactly when
`Backend::current()` matches the requested width. Preparation validates every exact-width request
before returning `Ok(None)` for heterogeneous programs, a connected sidechain or an unavailable
backend. Malformed requests reject; no lane is padded.

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
