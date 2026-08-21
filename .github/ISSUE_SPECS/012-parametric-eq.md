# 012 Parametric EQ

## Status — STOPPED/RESCOPED (2026-08-21)

**STOPPED/RESCOPED; no overall PASS.** The landed scalar contract, automation and architecture
kernel checkpoints `46b4a37`, `7b9c01b` and `cf739ef` remain technical input only. The first
independent-oracle gate disproved the frozen five-`f32`-coefficient direct-form-I numerical
contract before fixture, graph, realtime-audit or benchmark acceptance.

The exact 44,100 Hz, 10 Hz, -24 dB, S=0.1 low-shelf case produced -23.4572457785 dB at DC from
the cast coefficients while the independent `f64` design produced -23.9999999963 dB: a
0.5427542178 dB error against the unchanged 0.005 dB gate. The f64 numerator/denominator DC sums
were `5.053482540207099e-7` and `8.009230072603124e-6`; independent `f32` casts changed them to
`5.364418029785156e-7` and `7.987022399902344e-6`. The same case also exceeds the gate at the
audible 10 Hz probe, so deleting only the DC probe is not a valid correction. Broader frozen-grid
inspection found the same low-frequency pole/zero cancellation in bell, pass and notch rows.

No Issue-012 tolerance, probe or domain was weakened, no benchmark was invoked, and no overall
PASS may be inferred from its checkpoints. **Numerically conditioned launch parametric EQ
realization** owns the replacement decision and product closure.

## Outcome

Implement a launch-bounded four-section dual-mono parametric EQ as a bankable native effect, with
independent scalar and homogeneous four/eight-track execution and an independent `f64` oracle.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph/schedule/capacities are immutable while DSP state is mutated only
through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O,
logging, syscalls, structural plan mutation, or data-dependent unbounded work. There is no compiled
track limit. Audio is planar `f32`; L/R state and parameters are independent unless an explicit
contract says otherwise. Launch rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz. Output is
PCM.

This issue follows the Sol-approved brief -> Terra attempt 1 with evidence -> Sol adversarial
review workflow. Sol may make at most two further implementation/revision attempts, then the work
must stop and be rescoped/rebriefed rather than weakening gates.

## Scope

Provide exactly four ordered, preallocated sections. Each lane independently selects bell,
low/high shelf, low/high pass or notch; each section exposes enable, kind, frequency, gain, Q and
shelf slope. Whole-effect bypass is immutable prepared metadata. The only link mode is explicit
dual-mono. Normal quality is supported at the four launch rates.

Use normalized RBJ biquad coefficients and the brief's frozen `f32` direct-form-I graph. Numeric
changes are block-rate points with an exact 64-sample linear parameter ramp; every active update
redesigns and validates coefficients from current smoothed values.

## Required public interfaces/contracts

`ParametricEqFactory` implements `NativeEffectFactory`; prepared scalar and homogeneous-bank
processors implement `PreparedNativeEffect` and `PreparedNativeEffectBank`. Public
`EqBandDescriptorV1` records the four cascade positions and their stable parameter IDs; canonical
`EffectDescriptorV1` metadata remains authoritative for units/domains/mappings/smoothing. A safe
architecture-owned prepared DF-I token accepts exact scalar/four/eight-lane slices without
exposing registers, pointers or an unsafe caller contract.

Latency is zero, tail is `Infinite`, and state layout V1 is exactly zero common bytes plus 256
bytes per lane. Whole-effect bypass returns sanitized dry samples with zero latency while advancing
prepared parameter/filter state.

## Deliverables

Effect/factory code; narrow scalar/Wasm-SIMD/NEON/AVX2 and separately dispatched AVX2+FMA DF-I
kernel; completed filter-corpus decision; independent `f64` oracle; metadata and registry/session/
graph integration tests; one compact fixture; response/stability/automation/state/SIMD tests; one
realtime audit; one preflighted descriptive benchmark; and audition PCM plus blinded-listening
preregistration.

## Explicit non-goals

Dynamic EQ; linear-phase/FIR; bandwidth-in-octaves; more than four launch sections; Draft/High
quality; extended-rate qualification; third-party Wasm; sidechains; detector link modes; implicit
channel links; arbitrary section allocation; end-to-end graph/control automation delivery;
completed human listening; or performance tuning.

Prepared native-effect automation routing through graph render is future stateless scope under
**Prepared native-effect automation routing and render delivery**. Completed human trials are
future stateless scope under **Parametric-EQ blinded human listening qualification**. This issue
directly tests the effect-process automation boundary but does not claim either follow-up outcome.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance

Issue 008, **AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels**, stopped without overall PASS.
Checkpoint `87783c5` is explicitly accepted here only as bounded technical input: safe target
dispatch, generic AoSoA/effect-bank substrate and direct bank architecture. This issue does not
treat Issue 008 as passed or depend on its builtin-retention/benchmark successors.

## Sol implementation brief

**STOPPED/RESCOPED; DO NOT CONTINUE IMPLEMENTATION OR RUN THE BENCHMARK.** The historical
authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/012-parametric-eq.md`. It freezes parameters/state, RBJ equations,
DF-I/FMA graphs, smoothing/recovery, bank semantics, objective evidence, one realtime audit and the
former benchmark authorization. The numerical failure above invalidates that authorization;
Issue 042 is the only continuation.

## Hazards/decisions

The checked filter corpus and
[RBJ Audio EQ Cookbook](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html) are
the equation basis. Design in `f64`, normalize by `a0`, validate finite `f32` casts and strict Jury
conditions, then store `f32`. Base scalar/Wasm/NEON/AVX2 do not fuse; AVX2+FMA uses only four
permitted contractions. Disabled and zero-dB bell/shelf sections return exact dry bits. L/R and
cross-track histories, smoothers and payloads never alias.

The generic Issue-011 conformance runner assumes one parameter and an optional `sidechain-in`; it
cannot be claimed as evidence for this no-sidechain, 24-parameter effect. Product-specific
adversarial conformance is owned here; general harness hardening remains separate.

## Acceptance gates with objective measurements

At every launch rate and frozen grid, cast-coefficient analytic response is within 0.005 dB of the
independent `f64` design and one-second impulse/DFT response is within 0.05 dB where reference
magnitude is at least -120 dB; absolute null/stopband gates apply below it. LP/HP Butterworth
cutoff, bell center, shelf half-gain midpoint and notch minimum are within 0.1% of requested
frequency. Identity is bit-exact.

Exactly 10,000 deterministic legal designs remain finite and strict-Jury stable. The brief's 48
type/rate/edge sequences each remain finite without recovery for one million bounded samples;
separate invalid/extreme input proves bounded lane-local recovery. Exact 64-update parameter
trajectories, coefficient validity, malformed-span handling, reset/state round-trip and L/R/track
isolation pass.

Given identical coefficient bits, base same-target non-FMA scalar/bank paths are bit-identical.
AVX2+FMA passes `abs(error) <= 1e-6 + 2e-5 * abs(scalar)`. Named object/Wasm inspection proves
intended scalar, four/eight-lane and FMA graphs. Exactly 100,000 prepared 128-frame graph renders
report zero forbidden operations while armed.

Only after every nonbenchmark gate and workload-free preflight passes may root invoke the frozen
benchmark once. It contains one warmup and two measured rounds; timings have no threshold and may
not be tuned or retried.

## Target matrix

Native scalar; x86 AVX2-without-FMA and AVX2+FMA selected separately; AArch64 NEON four-lane;
wasm32 scalar and base `simd128` four-lane. Cross-target gates are compilation/instruction claims
unless a separate device/browser issue says otherwise.

## Required evidence

Descriptor/band/state tables; coefficient/Jury tables; response CSV and derived plots; fixture
manifest/hash; frequency, randomized, million-sample, automation, reset, payload and isolation
reports; scalar/SIMD/FMA maxima; named instructions; graph-bank/audit/target reports; benchmark
preflight count and, after the sole invocation, JSONL hash/count; audition manifest and listening
preregistration. Completed human notes are not an Issue-012 acceptance claim.
