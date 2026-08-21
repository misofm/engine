# 016 Launch fixed-4x true-peak safety limiter

## Outcome

Deliver one useful dual-mono safety limiter whose detector is an explicit fixed four-times
ITU-R BS.1770-5 Annex-2 interpolator, with a conservative ceiling guard, fixed reported latency,
scalar processing, homogeneous W4/W8 banks, scalar tails and the public registry/graph vertical.

## Context

Engine V2 is greenfield and must never inspect or inherit V1. Render exclusively owns a
preallocated prepared plan and performs no allocation/free, locks, feature detection, I/O,
logging, syscalls, panic/unwind, structural mutation or unbounded work. Tracks and state are
dual-mono. Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz; there is no implicit SRC
or compiled track ceiling.

This issue consumes the accepted native-effect runtime, compressor conventions, graph/PDC and
generic AoSoA bank seams. It has exactly **two total attempts**: one Terra implementation/review
and, if needed, one bounded Sol correction/review. A second failure stops and requires a stateless
rebrief; no domain, guard or gate may be weakened.

## Scope

- Add `miso.true-peak-limiter`, contract 1.0, state layout 1 and Normal quality at all four launch
  rates.
- Implement the exact order-48/four-phase Annex-2 interpolating FIR detector, instantaneous gain
  attack, one-pole release and a fixed 1.0 dB estimator safety guard.
- Support `LinkMode::{DualMono, Maximum}` with lane-local parameters, gain, delay, history, state
  and recovery. There is no sidechain.
- Expose ceiling and release as block-Point automation with exact 64-update ramps; lookahead is
  preparation/state-only from 0 to 10 ms inside a fixed maximum-latency delay.
- Reuse the accepted gain-only scalar/W4/W8 kernel, homogeneous bank binding, scalar tails,
  registry and graph/PDC seams.
- Close the product with representative independent interpolation/ceiling, latency, state,
  resource, scalar-bank and ten-track graph evidence only.

## Required public interfaces/contracts

`TruePeakLimiterFactory` implements `NativeEffectFactory`; its scalar and bank products implement
the accepted prepared traits. Ordered `main-in` and `main-out` are required dual-mono planar ports.
There is no optional port or external sidechain fallback.

Stable per-lane parameters, in descriptor order, are:

| ID | control | unit | inclusive domain | default | mapping | automation/smoothing |
|---:|---|---|---:|---:|---|---|
| 1 | ceiling | dBTP-est | -24..0 | -1 | linear | block Point; linear 64 updates |
| 2 | release | ms | 10..2000 | 100 | logarithmic | block Point; linear 64 updates |
| 3 | lookahead | ms | 0..10 | 5 | linear | none; preparation/state only |

`dBTP-est` means the level estimated by the frozen Annex-2 FIR. It is not sample peak, a continuous
meter API, or a claim that this limiter alone is a certified BS.1770/EBU programme-delivery meter.
The fixed 1.0 dB internal guard is part of the launch product and is never reported as extra user
ceiling. Normal quality always uses four phases; there are no 1x/2x/8x aliases or hidden qualities.

The authoritative interpolation table, sample order, gain law, latency, state words, resources,
reset/recovery, bank graph and objective tolerances are frozen in
`.github/ISSUE_SPECS/BRIEFS/016-true-peak-limiter.md`.

## Deliverables

- one `miso-engine-true-peak-limiter` package and minimal effect-compiler/graph registration;
- descriptor/factory, scalar processor, homogeneous W4/W8 bank and scalar tail;
- test-only independent `f64` FIR/gain reference and compact representative product tests;
- one ten-track registry/session/graph bank-plus-tail fixture with exact PDC and cap behavior; and
- candid focused/workspace/policy evidence with `timed_benchmark_invocations=0`.

## Explicit non-goals

Multiple oversampling qualities; a reusable audio oversampler; audio-rate upsample/process/
downsample; hard clipping; sidechain; threshold, attack, makeup, mix, loudness or continuous meter
telemetry; EBU programme-delivery certification; exhaustive standard/corpus matrices; randomized
or million-sample qualification; 100,000-render audit; target/object inspection; benchmark,
timing, optimization, audition or listening. Those qualification surfaces belong only to Issue
049, **Launch true-peak limiter qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch feed-forward peak compressor

The stopped Issue-008 dependency contributes only its preserved generic architecture/effect-bank
slice. No failed benchmark claim is imported.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 016 synchronization.** The tracked
authoritative brief is `.github/ISSUE_SPECS/BRIEFS/016-true-peak-limiter.md`. This checkpoint
authorizes no implementation, benchmark or GitHub mutation.

## Hazards/decisions

BS.1770-5 Annex 2 defines a measurement estimator, not a limiter gain law. This issue adopts its
exact 48-tap/four-phase coefficient table only for detection, then freezes an independent simple
lookahead gain law. The filter is detector-only, so this issue does not create a general
oversampled-audio framework. Four-times estimation at 44.1 kHz is explicitly protected by the
1.0 dB guard and representative independent reconstruction tests; exhaustive standard
qualification remains Issue 049.

## Acceptance gates with objective measurements

1. Descriptor, rate, link, parameter/order, quality, port, state/resource and prepared-metadata
   mutations reject transactionally. Exact caps and one-byte-below rejection pass at every launch
   rate.
2. The 48 coefficient bits match the Annex-2 dyadic table. Production `f32` phase outputs agree
   with an independently coded `f64` FIR within `2e-6` absolute, and representative phase-swept
   bandlimited tones through 0.45 Fs under-read an independent high-rate reference by no more than
   0.75 dB before the fixed 1.0 dB guard.
3. At ceilings `[-6,-1]`, lookaheads `[0,5,10]` ms, both links and all launch rates,
   representative near-Nyquist bursts, impulses and asymmetric lanes stay at or below the current
   smoothed ceiling plus 0.1 dB under the independent output estimator. No valid case recovers.
4. Enabled, bypass and gain-identity impulses land at the exact declared latency. Bypass and
   identity preserve delayed dry bits while warming detector, target and delay state; graph PDC is
   unchanged by bypass.
5. Exact automation endpoints/restarts, both resets, transactional active snapshot/restore,
   signed-zero identity, sanitation, ceiling-protecting lane recovery and L/R/track isolation pass.
6. Same-target scalar and available base W4/W8 PCM, state and reports are bit-identical for finite
   normal inputs without sanitation; the existing zero-FMA gain kernel is reused. The ten-track
   graph retains exact width-correct banks and scalar tails, preserves graph/PDC bytes and returns
   all ownership on one-below cap failure.
7. Focused locked tests, formatting, warning-denied Clippy, one locked workspace check/test/
   Clippy/rustdoc seal and applicable workspace/realtime/effect-runtime/rack/graph policies pass.
   No Issue-049, audit, target/object, benchmark, timing or listening command runs;
   `timed_benchmark_invocations=0`.

## Target matrix

Execute scalar and the available native W8 backend on the candidate host; source-level W4
compatibility and the accepted gain-kernel contract must compile in focused checks. Cross-target
and named instruction qualification belongs to Issue 049.

## Required evidence

Candidate/source identity; descriptor/latency/state/resource tables; coefficient-table hash;
independent FIR/interpolation/output maxima; exact latency and ceiling rows; state/reset/recovery
results; W8 execution and scalar parity; graph bank/tail/PDC/cap report; focused/full/policy
outputs; unchanged accepted runtime/core API statement; attempt count; explicit Terra/final Sol
PASS/FAIL; and `timed_benchmark_invocations=0`.
