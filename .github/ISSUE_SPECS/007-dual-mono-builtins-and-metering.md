# 007 Dual-mono builtins and metering

## Outcome

Implement the fixed per-track processing contract and objective meters before user effects.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement independent L/R input polarity, trim and HPF/LPF; output fader, mute and pan/explicit smoothed 2x2 matrix; every named tap observation point; and peak/RMS/loudness-ready meters. These are the two builtin sections surrounding the racks, not a second ambiguous all-in-one slot.

## Required public interfaces/contracts

`BuiltinChain::process_dual_mono` accepts distinct channel states; `ChannelLinkMode` is explicit; `Matrix2x2` has smoothing time and coefficient bounds; `MeterSnapshot` includes sample-time and counter reset semantics.

## Deliverables

Builtin implementation, parameter metadata, smoothing policy, meter accumulators, fixtures and documentation.

## Explicit non-goals

Implicit stereo linking, loudness certification, effect racks, graph routing, or hidden coefficient jumps.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

Dual mono never aliases L and R state. Matrix changes must smooth and remain finite; filter notes cite RBJ: https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html.

## Acceptance gates with objective measurements

Polarity/trim/fader impulses match analytic gain within 1e-6; conventional pan/balance adapters match their documented matrix and pan law within 1e-6; HPF/LPF analytic, impulse, and sustained-signal responses pass the realization-aware gates frozen below at every required rate; matrix ramp has no NaN/Inf and obeys its frozen per-sample slew bound; L-only input never changes R absent an explicit non-diagonal matrix; render allocation count is 0.

## Target matrix

Scalar core on all targets. Issue 008 adds and qualifies the 4/8-lane bank adapters without changing builtin semantics.

## Required evidence

Impulse and sweep fixtures, meter comparison data, allocation audit, benchmark, and listening record for matrix/filter changes.

## Sol implementation brief (2026-08-21)

**READY for Terra attempt 1.** The normative implementation-grade brief is
`target/issue7-sol-brief.md`. It freezes the three distinct builtin graph sections, RBJ
second-order Butterworth HPF/LPF scalar reference semantics, explicit matrix/pan equations and
N-update smoothing, lane-isolated reset/sanitization, transparent observers at all seven accepted
track boundaries, interval peak/RMS/energy and held-peak meter state, transactional preparation,
resource/no-allocation gates, target/fixture/listening evidence, and one exactly-once benchmark
invocation containing two descriptive internal rounds.

The brief does not change the accepted V1 TOML schema, issue-006 graph topology/PDC/reduction
contract, issue-011 effect contract, or issue-008 SIMD ownership. “Loudness-ready” is explicitly
bounded to timestamped per-lane energy observations and loss accounting; BS.1770 K-weighting,
gating, LUFS/LKFS, true peak, and certification are not issue-007 claims. No implementation or
benchmark was performed during briefing, and no V1/legacy source was inspected.

## Terra attempt 1 evidence (2026-08-21; partial)

Implementation added scalar dual-mono input and output builtin sections, explicit matrix/pan
smoothing, bounded meter accumulators, and prepared graph observer bindings without changing the
V1 TOML schema, graph topology, effect contract, or SIMD ownership. The graph compiler accepts a
complete prepared-builtin artifact transactionally, verifies canonical session/rate/quantum
identity, binds exactly the three internal stages, returns meter consumers, and propagates the
prepared filter tail to the post-input graph node.

PASS so far: focused unit/integration tests for gain/matrix/meter behavior, all seven prepared
tap requests, transactional invalid requests, internal binding ownership, tail propagation, and
the existing 65,537-track graph resource test; format, diff check, and warning-denied focused
Clippy also pass. Release scalar-builtins checks pass for `aarch64-linux-android`,
`aarch64-apple-ios`, and `wasm32-unknown-unknown` both with `-simd128` and `+simd128`. The
separate `miso-engine-dsp-reference` f64 RBJ oracle now covers an impulse through both filters at
all eight required rates; 10,000 deterministic scalar parameter/block mutations remain finite.
The complete workspace test suite and warning-denied all-target Clippy pass. The exactly-once
benchmark invocation count is **0**.

NOT YET SATISFIED: fixture-manifest corpus and independent f64 sweep oracle, full rate/quantum
matrix/meter mutation and allocation audits, issue-specific one-million-call tooling, full
workspace cross-target checks, real blinded listening records, and the authorized single
benchmark. These remain gates;
this evidence does not claim issue completion.

**New failing gate, 2026-08-21:** the strict all-rate/all-quantum swept-sine test was added but
currently fails at 192,000 Hz and 38,400 Hz: the scalar HPF/LPF cascade measures approximately
`-35.64407 dB` versus the independent f64 reference `-35.79218 dB` (about `0.14811 dB`, exceeding
the frozen `0.05 dB` tolerance). The test remains enabled; the tolerance was not changed and no
benchmark was run. This is a failed Terra attempt-1 gate pending Sol review/revision.

## Sol adversarial review / correction attempt 2 (2026-08-21)

**ATTEMPT 2: BLOCKED BEFORE IMPLEMENTATION; ISSUE REMAINS FAIL. REBRIEF REQUIRED.**

The short 4,096-frame sine fixture is defective at high rates: at 192 kHz it observes only 21.3
ms and its second half remains dominated by the 100 Hz section's startup tail. Extending the
experiment to a 250 ms settling interval followed by a 125 ms measurement interval does not,
however, make the frozen gate pass. At 192 kHz and 38.4 kHz the production scalar result remains
about `0.147205 dB` above the independent `f64` result. Thus the short window exposed the failure
poorly, but it did not create the underlying finite-precision error.

Independent calculations isolate that error:

- the analytic response of the production coefficients after their required `f32` cast differs
  from the separately derived `f64` response by about `0.0000008 dB` at the failing point;
- a 16,384-sample DFT of the production `f32` impulse response differs by about `0.0000703 dB`;
- sustained-sine simulation reproduces the production `f32` transposed-DF-II result at about
  `+0.147205 dB`, while changing only the state/arithmetic to `f64` reduces the difference to
  about `-0.00000066 dB`.

The independent oracle and coefficient design are therefore not the cause. The sustained-tone
gate is measuring real `f32` transposed-DF-II state/arithmetic quantization for a low-cutoff
section at a high sample rate. The brief requires coefficients cast to `f32` and freezes the
transposed-DF-II equations, but does not explicitly freeze state or intermediate precision. A
unilateral change to `f64` or compensated state would change the scalar DSP/resource contract and
conflict with issue 008's four/eight-lane `f32` SIMD consumption; changing the test to coefficient
or impulse-only response would stop measuring the observed sustained-signal behavior. Neither is
a bounded correction, and the tolerance was not relaxed.

The replacement brief must choose and freeze all of the following together:

1. state and intermediate precision/rounding for every multiply and add, including whether
   compensated state is allowed and how it is represented in four/eight-lane issue-008 kernels;
2. the magnitude conformance method: runtime swept/sustained signal versus impulse DFT versus
   coefficient transfer response, with exact amplitudes, settling/window length, leakage control,
   sampled frequencies and normalization;
3. one achievable all-eight-rate error gate for that chosen runtime realization, plus the scalar
   versus SIMD differential gate and revised state/resource estimates.

Adversarial review also found independent defects that a rebrief/restart must not hide: parameter
descriptors omit the frozen units/domains/defaults/update/smoothing metadata; the public block has
no rejecting safe constructor and processing silently returns an empty report for invalid shapes;
fader/mute does not sanitize at its DSP entry; an identity matrix can lose signed zero; computed
bad biquad state words are sanitized independently rather than clearing the pair and recording
recovery; recovery reports mix retained and per-call semantics; meter emission contains a render-
reachable `expect`; resource estimates omit substantial processor/observer/endpoint/allocation
overhead and do not compute the actual largest allocation; parameter diagnostics do not provide
the frozen lane/field paths; and the public prepared artifact plus graph attachment do not fully
prove exact prepared track/tail/observer-node sets against forged input.

No production correction was retained. Fixture-manifest, complete all-rate/all-quantum runtime
conformance, full mutation/allocation audits, the one-million-call/render/swap audit, real blinded
listening records, and final target evidence remain missing. The exactly-once benchmark was not
invoked; benchmark invocation count remains **0**. Stop this workflow and restart from a revised
Sol brief rather than attempting `f64`/double-single state or weakening/relabeling the failed gate.

## Sol rescope and workflow reset (2026-08-21)

**READY FOR A NEW TERRA ATTEMPT 1.** The failed two-attempt workflow above is closed. The
authoritative replacement brief is `target/issue7-rescoped-sol-brief.md`; the earlier brief is
historical evidence and is superseded. The new workflow retains every unfinished non-filter gate
and adversarial-review defect. Benchmark invocation count remains **0**.

The replacement freezes three linked decisions. First, production HPF/LPF uses the two-integrator
trapezoidal/TPT state-variable realization of the same bilinear second-order Butterworth response
(`Q=1/sqrt(2)`). Design is `f64` off render, then coefficients, both state words, audio, and every
render intermediate are `f32`. Base scalar/Wasm/NEON/AVX2 follows one non-fused operation graph;
FMA is separately dispatched by issue 008. There is no compensated or hidden wider state.

Second, analytic state-space and one-second impulse-DFT responses agree with an independent `f64`
RBJ oracle within `0.005 dB` and `0.05 dB` respectively where reference magnitude is at least
`-120 dB`. Sustained coherent amplitude-`0.5` sines settle for `Fs/2` frames and measure `Fs/4`
frames. Where reference gain is at least `-90 dB`, fitted fundamental magnitude agrees within
`0.05 dB` and non-fundamental residual is at most `-100 dB` relative to input RMS. Below `-90 dB`,
production is judged by an absolute `-88 dB` gain ceiling, not an ill-conditioned relative dB
comparison. DC, cutoff, monotonicity, impulse-tail, finite-state, reset, and isolation gates remain.

Third, `f64` state/arithmetic is test-only oracle precision, not production or a V1 quality mode.
Production `f64`, double-single, or compensated state is deferred to issue 031.

Enabled cutoff is exactly `10 Hz <= f < Fs/2`; `0` is disabled. The all-rate filter matrix tests
each section alone at `10`, `20`, `100`, `1000`, `min(20000,0.1*Fs)`, and `0.45*Fs` Hz
(deduplicated), plus the 100-Hz HPF/1-kHz LPF cascade. Probe targets are `0.25*f`, `f`, `4*f`,
`0.2*Fs`, and `0.45*Fs`, clipped inside Nyquist, snapped to the nearest 4-Hz coherent bin, and
deduplicated; analytic preparation also probes exact cutoff and `0.49*Fs`.

Issue 008 consumes three `f32` coefficient vectors and two independent `f32` state vectors per
enabled filter/lane. It may transpose across tracks but may not revert to TDF-II, widen state,
share L/R state, or contract the base graph. No precision cohort is introduced this sprint.

The restart also corrects complete parameter metadata; rejecting safe blocks; fader/mute entry
sanitization; signed-zero identity; pairwise state recovery with per-call and lifetime counters;
render-reachable panics; full resource/largest-allocation accounting; exact lane/field diagnostics;
and exact prepared track/tail/observer-node set validation. All prior missing fixture, target,
mutation, realtime, allocation, listening, and benchmark evidence remains required.

## Restarted Terra attempt 1 evidence (2026-08-21; partial)

Production HPF/LPF now uses the rescope's `f32` non-fused TPT/SVF operation graph, with `f64`
off-render design, cast-coefficient transition/Jury validation, 10-Hz enabled-cutoff floor, and
pairwise state recovery. The former TDF-II sustained-sine failure is not retained as a production
gate. A separate f64 state-space oracle built from the exact cast TPT coefficient bits compares
against its independently derived RBJ transfer response across every required rate, section,
cutoff, and prescribed analytic probes; the current `0.005 dB` analytic gate passes. The existing
all-rate impulse/sweep and bounded mutation tests also pass under the new realization.

This is not issue completion: the full one-second impulse DFT, coherent sustained-signal,
manifest fixtures, public API/resource/meter corrections, allocation/realtime audits, listening,
cross-target workspace evidence, and authorized benchmark remain outstanding. Benchmark count is
still **0**.
