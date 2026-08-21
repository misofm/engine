# 007 Dual-mono builtins and metering

## Outcome

Implement the fixed per-track processing contract and objective meters before user effects.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only and are not issue-007 acceptance gates. Source/engine mismatches have no implicit SRC. Output is PCM.

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

Polarity/trim/fader impulses match analytic gain within 1e-6; conventional pan/balance adapters match their documented matrix and pan law within 1e-6; HPF/LPF analytic, impulse, and sustained-signal responses pass the realization-aware gates frozen below at all four launch rates; matrix ramp has no NaN/Inf and obeys its frozen per-sample slew bound; L-only input never changes R absent an explicit non-diagonal matrix; render allocation count is 0.

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
all eight required rates (historical terminology; the higher four are issue-032 compatibility-only);
10,000 deterministic scalar parameter/block mutations remain finite.
The complete workspace test suite and warning-denied all-target Clippy pass. The exactly-once
benchmark invocation count is **0**.

The all-eight-rate observations above are historical issue-007 numerical evidence. Issue 032 does
not alter their values, dates, or outcome; its four higher-rate observations are compatibility-only.

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
3. one achievable all-eight-rate error gate for that chosen runtime realization (historical
terminology; issue 032 retains higher-rate observations as compatibility-only), plus the scalar
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
against its independently derived RBJ transfer response across every then-required rate, section,
cutoff, and prescribed analytic probe; the current `0.005 dB` analytic gate passes. This is
historical all-eight-rate evidence: issue 032 makes the higher four observations informational
compatibility probes. The existing all-rate impulse/sweep and bounded mutation tests also pass
under the new realization.

This is not issue completion: the full one-second impulse DFT, coherent sustained-signal,
manifest fixtures, public API/resource/meter corrections, allocation/realtime audits, listening,
cross-target workspace evidence, and authorized benchmark remain outstanding. Benchmark count is
still **0**.

**New failing gate, 2026-08-21:** the rescope's coherent sustained-sine fixture was added with
amplitude `0.5`, `Fs/2` settling, `Fs/4` measurement, f64 least-squares DC/sine/cosine fit, and
the frozen deep-stop branch. It currently fails the non-fundamental residual ceiling at 88,200 Hz,
10-Hz HPF, 4-Hz coherent probe: the measured production residual is approximately `-94.24404 dB`
relative to input RMS, above the required `-100 dB`. The fixture remains enabled and unchanged;
no threshold was relaxed and the benchmark count remains zero. This requires Sol review before
any implementation/tolerance change.

## Incremental-recurrence Terra attempt 1 evidence (2026-08-21; partial)

Production now stores the conditioned `c1=t1/d` coefficient rather than `a1`, with the frozen
incremental all-`f32` non-FMA state graph (`d1/d2`, `q1/q2`, `ic1/ic2`). Preparation follows the
frozen f64 dependency order and derives Jury checks from the cast stored bits. The separate f64
state-space oracle was updated to the matching `A/B/C/D` equations without importing production
design code. The direct-recurrence sustained test was atomically superseded by the normative
recurrence implementation and launch/deferred-compatibility observation tests.

PASS so far: analytic cast-state transfer, one-second impulse DFT across the five prescribed
partitions, and coherent sustained-sine fundamental/residual/deep-stop checks all pass for the
four launch rates; the preserved four deferred rates also currently pass as compatibility-only
observations. Focused warning-denied Clippy passes. This remains partial evidence: the full
production-order cascade, fixtures, public API/resource/meter/realtime corrections, listening,
and final target evidence are still outstanding. Benchmark count remains **0**.

The production-order 100-Hz HPF followed by 1-kHz LPF cascade now has an independent analytic
RBJ-product comparison, one-second impulse DFT at each prescribed partition, and coherent
sustained fundamental/residual/deep-stop measurement over the sorted union of both probe sets.
All four launch rates pass the corresponding frozen gates. The remaining non-filter gates remain
open; this is not a benchmark authorization.

## Sol adversarial review / correction attempt 2 after rescope (2026-08-21)

**ATTEMPT 2: BLOCKED BEFORE PRODUCTION CORRECTION; REBRIEF REQUIRED.**

The failure is caused by the exact frozen `f32` operation graph, not coefficient design, the
independent oracle, the least-squares measurement, subnormal handling, or state recovery. The
88,200-Hz/10-Hz-HPF/4-Hz case reproduces `-94.2440363 dB` in both debug and release. Its cast
coefficients are `a1=0.9994964`, `a2=0.00035601028`, `a3=1.2680718e-7`, and
`k=1.4142135`; all state and output remains finite and normal, with zero recovery and zero
sanitization events. Sol also solved the full three-column DC/sine/cosine normal equations rather
than relying on the coherent-basis shortcut and obtained the same residual.

Controlled simulations using those exact coefficient and input bits isolate intermediate
rounding from stored-state quantization:

- the frozen production graph gives `-94.2440363 dB`;
- `f64` state and intermediates followed by an `f32` output cast give `-153.5180902 dB`;
- storing both candidate states as `f32` every sample while evaluating intermediates in `f64`
  gives `-126.2688260 dB`;
- quantizing only `s1` or only `s2` gives `-126.7174910 dB` and `-126.6437414 dB` respectively;
- an experimental, algebraically equivalent all-`f32` incremental state update gives
  `-126.1933635 dB` for this case.

The last result is diagnostic evidence, not an approved implementation. In the frozen graph,
`v2 = s2 + p3 + p4` first rounds the small integrator increment into the much larger state, then
`n2 = 2*v2 - s2` cannot recover the discarded bits; the corresponding `a1*s1`/`n1` path has the
same subtract-near-equals sensitivity. An incremental recurrence avoids that particular loss, but
it changes the explicitly frozen production operation order and issue 008's scalar/SIMD
bit-identity input contract. Retaining wider intermediates would also violate the explicit
all-`f32` decision. Therefore no production edit is legal under the current brief, and neither
the `-100 dB` gate nor production precision was changed.

A replacement brief must freeze and justify a numerically conditioned all-`f32` state-update
graph, including every prepared coefficient, multiply/add order, cast and non-FMA rounding point;
derive its cast-coefficient state transition and Jury checks; amend issue 008's matching SIMD
graph; and rerun all three response families over the complete matrix before any non-DSP gate.
The incremental diagnostic above is only one candidate and is not accepted based on one passing
case.

The filter-gate audit also found omissions that the next attempt must not treat as passes: runtime
preparation does not perform the brief's analytic cutoff-response rejection; the analytic test
does not yet prove the explicit DC/Nyquist limits or monotonicity and does not use the complete
snapped probe matrix; the analytic, one-second impulse, and coherent sustained tests omit the
required 100-Hz-HPF/1-kHz-LPF cascade; and the impulse test does not record final-4096-frame
energy, reset/repeatability, L/R isolation, or explicit bit-identical block partitions. The known
parameter metadata, safe-block API, fader/mute sanitization, signed-zero matrix, per-call/lifetime
recovery, render-panic, resource accounting, diagnostic-path, prepared-artifact, fixture,
realtime, target, and listening deficiencies also remain outstanding. They were not continued
past the failed DSP gate.

The temporary diagnostic code was not retained. The failing acceptance test remains enabled.
The exactly-once benchmark was not invoked; benchmark invocation count remains **0**.

## Second recurrence rescope and workflow reset (2026-08-21)

**READY FOR A NEW TERRA ATTEMPT 1.** The restarted two-attempt workflow is closed. The
authoritative replacement brief is `target/issue7-recurrence-sol-brief.md`; both earlier briefs
are historical evidence and are superseded. Every unfinished functional, quality, realtime,
resource, fixture, target, listening, and benchmark gate remains in force. Benchmark invocation
count is **0**.

The accepted all-`f32` recurrence retains the TPT response and two independent state words, but
stores the integrator states directly and updates them by explicit increments. Off render, compute
`c1 = g*(g+k)/(1+g*(g+k))` in `f64` (the conditioned form of `1-a1`) and cast it once to a stored
`f32`. Do not store `a1` for render and do not recompute `1-a1` per sample. For old states
`ic1,ic2`, first derive `d1 = a2*(x-ic2)-c1*ic1` and
`d2 = a2*ic1+a3*(x-ic2)` with the exact separately rounded order in the replacement brief;
outputs use the midpoint values `v1=ic1+d1`, `v2=ic2+d2`, and next states are
`ic1'=ic1+(d1+d1)`, `ic2'=ic2+(d2+d2)`. State therefore means the stored TPT integrator value at
the preceding sample boundary, not `v1`, `v2`, a hidden residual, or a delay-line alias. Reset and
pairwise recovery set both words to positive zero.

Sol evaluated this candidate without changing production. Across all 232 launch-matrix single-
section rate/filter/cutoff/snapped-probe cases, it produced zero failures against the existing
analytic, one-second impulse, sustained fundamental/residual, and deep-stop thresholds. Worst
figures below are conservative values from the 464-case superset that also included all four
deferred extended rates; that superset likewise produced zero failures. Worst
observations were `0.00000176 dB` analytic error, `0.00000176 dB` cutoff error, `0.015081 dB`
impulse error, `0.0000163 dB` sustained fundamental error, `-116.346 dB` residual, and
`-91.410 dB` total output in the `< -90 dB` reference branch. The comparison form that recomputed
`1.0_f32-a1` per sample also passed this matrix, but its worst analytic/cutoff error grew to about
`0.001324/0.001413 dB`; it is rejected because the conditioned stored complement is more accurate,
removes a render operation, and gives issue 008 one immutable coefficient vector.

The replacement brief freezes every cast and non-fused rounding point, the cast-coefficient state
transition and Jury test, output observation equations, reset/sanitization semantics, issue-008
SIMD identity contract, and exact cascade coverage. Terra must implement the recurrence and fill
the previously audited matrix/cascade omissions before proceeding to non-DSP gates. No benchmark
was run during this rescope.

This issue-local rate correction does not silently redefine the repository-wide architecture.
A separate stateless cross-cutting rate-tier issue must amend `AGENTS.md`, shared issue contexts,
session validation, conformance fixtures, host/runner contracts, and release qualification before
the four-rate launch claim is project-wide. Issue 007 may use its explicit four-rate gate now;
extended-rate observations are diagnostic only and cannot be cited as supported qualification.

## Retained API/resource/meter Terra evidence (2026-08-21; partial)

The public scalar contract now has a rejecting `DualMonoBlock::new` constructor and private block
fields; all three section processors and the combined chain return typed block failures instead of
silently returning an empty report. The graph adapter creates the validated block and maps a time
overflow to `RenderError::TimeOverflow` and other malformed render shapes to
`RenderError::InvalidEnvelope`. The ten stable parameter descriptors now include unit, numeric
domain, default, per-lane selection, update rate, smoothing, reset, and filter-disabled (`0 Hz`)
semantics. Compiler diagnostics identify the offending lane/field for gain, cutoff, filter order,
matrix, and smoothing validation.

Input recovery reports are per-call; separately named lifetime counters are queryable and reset
independently. Fader/mute now sanitizes at its entry, and a settled identity matrix copies finite
samples directly so signed zero is preserved. Meter observation rejects mismatched lanes and
sample-time overflow with a typed error. Emission contains no `expect`: impossible internal state
is converted to a recorded discontinuity, preserving the no-panic render contract. Focused meter
conformance covers fixed windows, energy/RMS/peak/held peak, sanitization, discontinuities,
sequence/reset generations, and bounded queue-drop accounting.

The prepared artifact publishes checked retained-resource payload estimates. These include chain
state, three processor box payloads and binding-vector slots per track, tail entries, observer and
consumer endpoint/binding payloads, and each SPSC meter's exact `capacity + 1` snapshot slots;
the largest of those allocation payloads is enforced by the existing cap. This is not a claim of
whole-plan or allocator-header accounting, forged-artifact sealing, or the required million-call
audit; those gates remain open.

PASS: `cargo test -p miso-engine-builtins -p miso-engine-builtins-compiler` (14 builtins and 2
compiler tests) and `cargo clippy -p miso-engine-builtins -p miso-engine-builtins-compiler
--all-targets -- -D warnings`. The original test-only `needless_range_loop` Clippy warning was
removed. The response and cascade tests remain in that focused suite and pass. Full workspace,
allocation/realtime, fixture/listening, cross-target, and final prepared-artifact gates are still
unrun or open. The exactly-once benchmark invocation count remains **0**.

## Fixture/reference/policy Terra evidence (2026-08-21; partial)

`fixtures/builtins/v1` now contains a sorted, exact-length, lowercase-SHA-256 manifest for stable
filter-response, meter-window, and resource-cap case declarations. `check-builtins-fixtures.sh`
rejects missing, changed, malformed, unsorted, and unlisted payloads; its mutation companion
proves a changed fixture cannot escape. The case declarations point to the existing independent
f64 TPT/RBJ section and production-order cascade tests, which continue to cover the four launch
rates and prescribed response partitions. They do not replace the remaining broader fixture
classes or an external reproducible f64 table.

Deterministic conformance now includes 10,000 independently seeded meter configurations/blocks;
all emitted peak/RMS/energy values stayed finite and every window retained its exact configured
frame count. A compiler test derives the emitted retained-resource estimate and proves that a cap
one byte below its largest payload rejects transactionally with `builtin.resource.limit`.
`check-builtins-policy.sh` now validates the production dependency direction as well as naming,
`unsafe`, and compiled-track-ceiling prohibitions; its mutation test proves an injected forbidden
token is rejected. The implementation note now describes the conditioned incremental TPT
recurrence rather than the superseded transposed-DF-II realization.

PASS: `bash scripts/check-builtins-policy.sh`, `bash scripts/test-builtins-policy.sh`,
`bash scripts/check-builtins-fixtures.sh`, `bash scripts/test-builtins-fixtures.sh`, and the
focused builtins/compiler test suite (15 + 3 tests). Allocation/forbidden-operation auditing,
65,537-track builtin preparation, full fixture corpus, sealed-artifact equality, workspace/target
evidence, listening, and the final benchmark remain open. Benchmark invocation count remains
**0**.

## Scale and realtime-audit Terra evidence (2026-08-21; partial)

The new builtins compiler scale integration test prepares 65,537 independently named tracks with
no meters and no fixed-track ceiling. It produces exactly three prepared processors and one tail
record per track, then repeats preparation with a state cap one byte below the measured retained
payload and receives only `builtin.resource.limit`. This is a configured-resource rejection, not
a hidden track limit.

`miso-engine-builtins-audit` prepares a scalar chain and seven bounded meter accumulators off
render, then runs one million 128-frame full-chain calls inside the realtime allocator guard. The
first window succeeds for each observer and all following windows exercise bounded queue-full/drop
behavior. The release record was `blocks=1000000`, `observers=7`, `queue_success_windows=7`,
`queue_full_windows=6999993`, and zero allocations, deallocations, locks, logs, file I/O, network
I/O, syscalls, and total violations. The left/right backing addresses were unchanged. The strace
gate found no system call between the explicit render markers. Deliberate probes for allocation,
deallocation, lock, log, file I/O, network I/O, and syscall each terminated as required, proving
the detector path is armed.

PASS: `cargo test -p miso-engine-builtins-compiler --test scale`, release
`miso-engine-builtins-audit --blocks 1000000`, `trace-builtins-audit.sh 1000000`, and
`test-builtins-audit-probes.sh`; focused warning-denied Clippy, fixture/policy checks and mutation
checks also pass. This is still not a whole-render-plan/swap audit of the sealed graph artifact,
nor full workspace/target/listening qualification. Benchmark invocation count remains **0**.

## Fixture sealing, targets, listening, and benchmark readiness Terra evidence (2026-08-21; partial)

The checked-in builtin fixture manifest now includes a machine-readable conformance matrix fixing
the four launch rates, five quanta, section/cascade response grid, seven taps, meter window/drop/
reset cases, and resource-count matrix including 65,537 tracks. The fixture corruption test now
proves content changes, manifest corruption, and unlisted payloads are all rejected. This records
the complete intended matrix but does not falsely claim every response-output artifact or external
f64 table is checked in.

`check-builtins-targets.sh` passed the native scalar baseline, `aarch64-linux-android`,
`aarch64-apple-ios`, and separate `wasm32-unknown-unknown` `-simd128` and `+simd128` release
artifacts for the scalar builtin/compiler packages. Two checked listening records preregister an
ABX filter-change procedure and randomized matrix-ramp procedure. Their machine check requires
`Status: preregistered`, an explicit no-human-evidence statement, and no fabricated trial rows;
they are procedures, not listening results.

The exactly-once benchmark has only been prepared: a fixed ten-workload/two-round emitter, runner
that refuses arguments and existing artifacts, JSONL validator requiring exactly twenty records,
and readiness/mutation test that proves invalid arguments/records fail. The readiness test made
**zero** workload launches; neither the runner nor benchmark binary was invoked. Full workspace
sealing and actual completed human listening records remain open. Benchmark invocation count
remains **0**.

Workspace readiness also passes `cargo test --workspace`, warning-denied workspace Clippy,
formatting, workspace policy, realtime policy (including its mutation suite), and builtin policy.
The realtime unsafe allowlist now names only the standalone builtin audit executable's exact source
path in addition to the previously accepted audit tools; it does not relax production-crate
restrictions. These checks do not convert the pending sealed-artifact proof or human listening
preregistrations into completed acceptance evidence.

## Final machine-verifiable Terra evidence (2026-08-21; partial)

Graph attachment now validates the exact expected prepared builtin processor, tail, observer, and
consumer-derived node/handle sets before moving ownership. A forged artifact with its tail set
removed is rejected before attachment with `builtin.prepared.tail_set`; the canonical graph test
suite still passes. This is the machine proof for exact-set attachment, not permission to forge
the opaque processor implementations themselves.

Benchmark preflight builds but does not execute `miso-engine-builtins-bench`, validates the fixed
input fixture manifest and runner/validator, and emits the artifact/input hashes. This run reported
`workload_launches=0`, binary SHA-256
`5f17c3e8a016e2470fba258939c95cc9e0f89cd09e6076c19e34451aedc337fe`, and input-manifest
SHA-256 `9d98381d7b9ba8c4737fcc6128158f67d0fe3004770a5a9559e2025f3a393a71`.

PASS: graph compiler unit suite (10 tests, including forged-artifact rejection), focused
warning-denied Clippy, and zero-launch benchmark preflight. Human listening is still only
preregistered, so the issue cannot honestly be declared complete; benchmark invocation count
remains **0**.

## Final Sol adversarial review / correction attempt 2 (2026-08-21)

**ATTEMPT 2: FAIL; FINAL ATTEMPT REBRIEF REQUIRED.** No timed benchmark was invoked; the
exactly-once benchmark invocation count remains **0**.

The bounded DSP correction retained in checkpoint `0627618` is sound and its focused gates pass.
Production preparation now rejects a cast TPT state-space response outside the frozen cutoff
tolerance. Analytic conformance uses the complete snapped probes plus exact cutoff and 0.49-Fs,
checks explicit DC/Nyquist limits and a dense monotonic grid, and impulse conformance records tail
energy and bit-identical block partitions. Sustained section and cascade measurements now solve
the full three-column DC/sine/cosine normal equations rather than relying on the coherent-basis
shortcut. Formatting, all 15 builtin tests, and warning-denied builtin all-target Clippy pass.

The remaining work is not a bounded correction. Adversarial inspection found five coupled false-
readiness surfaces:

1. `miso-engine-builtins-bench` times only a dummy integer accumulator. It reports only 48 kHz,
   uses workloads different from the frozen 48/96-kHz Cartesian matrix (including a 65,537-track
   placeholder instead of 256-track preparation), and omits the required percentile, fixture,
   output-identity, resource, forbidden-operation, CPU/OS/governor, toolchain, target-feature and
   build-profile fields. Its validators faithfully accept that wrong schema.
2. `miso-engine-builtins-audit` exercises a direct `BuiltinChain` and seven standalone meters. It
   never compiles, binds, renders, swaps, defers, or retires a graph-backed prepared plan, and its
   steady loop does not cover the complete preregistered ramp/reset/sanitization paths.
3. `fixtures/builtins/v1` contains a matrix declaration and a few representative case rows, not
   the required checked-in expected PCM bits, independent f64 response table, exact meter
   snapshots, diagnostic/resource results, and graph-tap outputs for every required tuple.
4. builtin resource estimates are logical approximations: they double-count transient chain
   storage, omit retained string/session/queue-ring and allocation payloads, use saturating or
   unchecked conversions on some paths, and allocate prepared vectors after a cap diagnostic.
   The one-byte-below test therefore proves consistency with the estimate, not containment of all
   retained allocations or the actual largest requested payload.
5. `PreparedBuiltinsSession` exposes mutable public vectors. Graph validation checks mostly node/
   handle sets derived from those same mutable values; it does not seal exact tail values,
   consumer requests, concrete processor provenance, or an immutable canonical session identity.
   Matching forged consumer/observer or tail values can therefore pass the advertised set proof.

Related missing evidence includes end-to-end rendered values at all seven taps, complete matrix
corner/retarget fixtures, the full invalid preparation/cap mutation matrix, and a graph observer
path that cannot silently discard a supposedly impossible meter error. The current target and
workspace commands may remain useful evidence, but they do not close the five defects above.

Two preregistration files truthfully say that no human trial has run. Real human listening cannot
be fabricated by an implementation agent and is most meaningful only against the sealed candidate
artifact produced after this issue's machine gates and sole benchmark. It is therefore moved,
without removal from launch accountability, to the stateless issue **Issue-007 builtin filter and
matrix human listening qualification**. Issue 007 may become *machine-qualified only*; it cannot
make an audible-quality or launch-readiness claim. The follow-up is an exact dependency of end-to-
end release qualification, and an adverse or incomplete result blocks launch and requires a new
corrective DSP issue.

The authoritative final-attempt brief is `.github/ISSUE_SPECS/BRIEFS/007-final-sol-brief.md`. It freezes the real
benchmark workloads/schema, graph-backed render/swap audit, complete expected-output fixture
formats, exact resource metric and sealed artifact ownership, and the listening hand-off. Attempt
3 is the last implementation/review attempt. Any failure stops issue 007; do not weaken a gate or
run the benchmark again.

## Final attempt-3 workflow reset (2026-08-21)

**READY FOR FINAL ATTEMPT 3.** Prior briefs remain historical evidence and are superseded by
`.github/ISSUE_SPECS/BRIEFS/007-final-sol-brief.md`. The accepted incremental TPT operation graph and checkpoint
`0627618` remain normative. Benchmark invocation count is **0**. Only the exact final brief may
authorize the sole two-round benchmark after every machine-verifiable nonbenchmark gate passes.

## Final attempt-3 Terra evidence (2026-08-21; artifact/resource tranche)

`PreparedBuiltinsSession` is now opaque outside `miso-engine-builtins-compiler`; it retains a
private SHA-256 session/rate/quantum seal, exact track/stage/tail identities, ordered meter
request/observer/consumer identities, and its resource report. Graph lowering validates the seal
against the effect-prepared session before ownership moves, then consumes the artifact only into
private graph bindings. The existing forged-tail test now uses the compiler-owned test-support
seam rather than a public mutable artifact field. Graph observer failures are returned as bounded
`RenderError`s; the meter adapter no longer discards an observation error.

Builtin resource reporting now names engine-owned retained payload bytes and computes checked
`Layout` sizes for retained binding/seal vectors, processor/observer boxes, stable-ID payloads,
meter endpoint payloads, and exact SPSC ring-header plus `capacity + 1` slot layouts. It excludes
allocator headers and page rounding. Preparation validates parameter/resource domains and all
caps before constructing builtin processor, meter, or artifact payloads; one-byte-below
processor, meter, and largest-allocation boundaries reject with `builtin.resource.limit`.

PASS: focused builtin-compiler and graph-compiler tests (including the 65,537-track scale gates),
warning-denied Clippy for core/builtin/compiler/graph crates, formatting, and diff check. This is
not complete resource-containment evidence: the required test-only allocation tracker, complete
per-seal-field corruption matrix, fixture corpus, graph-backed million-render/swap audit, target
matrix, workspace sealing, and frozen benchmark implementation remain open. Human listening is
Issue #33 and remains pending. Timed benchmark invocation count remains **0**.

## Final attempt-3 Terra evidence (2026-08-21; graph realtime/lifecycle tranche)

`miso_engine_builtins_graph_audit` compiles the canonical accepted session, prepares the sealed
builtin artifact with all seven meter requests, lowers it through `GraphCompiler`, and binds only
the genuine external source and output nodes. It renders through `RealtimePlanOwner` at exactly
48 kHz/128 frames for 1,000,000 blocks; no direct `BuiltinChain` render is used. The fixed
exchange sequence applies plan 7 at a block boundary, proves plan 7 renders when plan 8 is
deferred by the full retirement queue, then reclaims plan 6 on the dedicated retirement thread
before applying plan 8. Both displaced plans are destroyed by that retirement owner after their
completed render markers.

The frozen audit drains exactly seven observer windows for each of six drain points. Its meter
snapshots prove 42 queue-success windows and 6,999,958 exact queue-full/drop windows. It records
epoch 1 rendering four blocks and epoch 2 rendering 999,996 blocks, two applied swaps, one forced
retirement-full deferral, one prior-plan render on deferral, and unchanged planar output backing
addresses. The audited allocator and all seven forbidden-operation hooks report zero allocation,
deallocation, lock, log, file-I/O, network-I/O, syscall, and total violations. The native release
`strace` gate checks all seven marker-delimited render intervals and found no syscall between
markers. Deliberate allocation, deallocation, lock, log, file-I/O, network-I/O, and syscall
probes each abort as required.

PASS: `cargo run -p miso-engine-builtins-audit --bin miso_engine_builtins_graph_audit`,
`scripts/test-builtins-graph-audit-probes.sh`,
`scripts/trace-builtins-graph-audit.sh`, `scripts/check-realtime-policy.sh`, and
`scripts/test-realtime-policy.sh`; focused warning-denied Clippy, formatting, and diff checks
also pass. Benchmark invocation count remains **0**. This evidence does not authorize or invoke
the timed benchmark.

## Final Sol adversarial verification / attempt-3 verdict (2026-08-21)

**ATTEMPT 3: FAIL; ISSUE 007 STOPS. DO NOT AUTHORIZE THE TIMED BENCHMARK.** The timed
workload invocation count remains **0**. No raw or accepted issue-007 benchmark artifact exists.
Human listening remains honestly pending in Issue 033 and was not fabricated.

The retained conditioned incremental TPT implementation passed its analytic cast-state cutoff,
DC/Nyquist, dense monotonicity, one-second impulse/partition/tail, sustained three-column fit and
fixed 100-Hz-HPF/1-kHz-LPF cascade tests at the four launch rates. Focused debug and pinned scalar
release tests passed, as did the 65,537-track tests, all eight existing sealed-artifact corruption
tests, the current allocation tracker, the native/mobile/Wasm target script, locked workspace
tests, warning-denied all-target Clippy and rustdoc, formatting, and all current
workspace/realtime/research/graph/builtin policy and mutation scripts. The direct and graph audit
executables each completed 1,000,000 iterations with zero operations counted by their current
seven audit hooks, their syscall traces passed, and all current detector probes fired. The
zero-launch preflight reported `workload_launches=0`, binary SHA-256
`d1505fa38322eeb87ce651115a96a9c9e75d4a5da89a2edc56125abe89a35638`, and input-manifest
SHA-256 `e824fa664933cd2a1a2ea62285dcbb7e7164c114922ca274f1246c4b8ecf1337`.

Those successful commands do not satisfy the frozen final brief. Adversarial inspection found
the following unresolved launch-critical contract defects:

1. Parameter metadata still exposes `per_lane: bool`, uses undifferentiated `Decibels`, and gives
   HPF/LPF an infinite maximum. It does not expose the required `PerLane`/`MatrixShared` scope,
   explicit decibel-amplitude mapping, or the rate-dependent `0`-disabled/otherwise
   `10 <= f < Fs/2` domain.
2. `PreparedGraphPlan::attach_internal_bindings` remains a public generic method accepting
   arbitrary processor and observer vectors. An ordinary caller can therefore bypass the sealed
   `PreparedBuiltinsSession` path and masquerade arbitrary internal bindings, contrary to the
   frozen provenance contract. The eight corruption tests exercise the sealed validation path but
   do not close this public bypass or prove compile-time opacity.
3. Resource preflight still converts `requests.len()` with unchecked `as u64`, contrary to the
   all-conversions-checked rule. The test allocator proves total/largest only for one-track/seven-
   meter capacity-four preparation; it does not record/compare every requested layout or the
   allocation-count breakdown across the pinned 1/4/65,537-track and 0/1/7-meter resource grid.
4. There is no frozen 10,000-case builtin *compiler* mutation test spanning valid and invalid
   parameters, meter requests, matrix targets, block shapes/times and every cap. The current two
   builtin loops generate valid scalar/meter inputs, while the graph compiler's 10,000 loop mutates
   generic graph topology.

The remaining failures are qualification/tooling gaps rather than observed TPT response failures:

5. The fixture corpus is not the complete frozen expected-output matrix. Its cascade generator
   derives variable `0.5*cutoff`/`2*cutoff` filters instead of the fixed 100-Hz HPF followed by
   1-kHz LPF and does not use the sorted union of those two probe sets. `cases.toml` contains
   declarations instead of fully expanded functional tuples; pan/balance endpoints and centers,
   complete block splits/retargets, builtin sanitization/recovery PCM, PDC coexistence, and all
   seal diagnostics are absent. Decimal response fields are fixed decimal places rather than 17
   significant digits. The graph fixture clears every rack, so post-input/SIMD1/dynamic/SIMD2
   observations are bit-identical rather than distinguishable. Current corruption tests alter
   every generated file's bytes, but do not exercise delete/alter/add/coverage-hole mutations for
   every frozen format.
6. The direct million-call audit starts one ramp but never exercises retarget, sanitization,
   pairwise recovery or reset. The graph audit verifies frame/drop counts only, not the exact
   seven tap values or checked PCM fixtures; it clears every rack and has no PDC coexistence. It
   records two applied swaps where the brief freezes one applied swap plus a deferred replacement,
   reports counts only by epoch rather than plan/epoch, and has no feature-detection or
   panic/unwind hook/counter/probe.
7. The schema-v2 benchmark implementation is not the frozen workload. Render timings retain the
   whole eight-operation batch duration instead of dividing nanoseconds by operations. The meter
   workload observes the same post-chain buffers with fourteen standalone meters rather than two
   sets across all seven taps, and render operations reuse one batch sample time. All workloads use
   the manifest as their input-fixture ID rather than workload-specific fixture IDs. Records omit
   CPU architecture, physical core count, render-error, feature-detection, panic/unwind and total
   forbidden-operation fields. The runner substitutes `unknown`/`default` without adding those
   keys to `missing_metadata`, and a validation failure does not preserve a validator reason/hash
   record. The validator does not bind workload ID to kind/rate, enforce workload-specific shapes,
   or require stable output hashes across rounds; its mutation suite covers only a small subset of
   required fields and no exhaustive workload/rate/cardinality/output-fixture mismatch matrix.
   An adversarial synthetic 20-record set with a cross-round output-hash mismatch and without the
   omitted metadata/audit fields was accepted by the current validator.

Do not perform a fourth issue-007 implementation attempt. Create two new stateless corrective
issues with the accepted TPT recurrence and thresholds explicitly out of scope: first, a
launch-critical builtin-contract closure for metadata, sealed-only graph attachment, checked
resource accounting and the complete compiler mutation gate; second, dependent qualification
tooling for the complete independent fixture corpus, exact direct/graph audits and schema-v2
benchmark runner/validators. The latter owns a newly briefed single benchmark authorization only
after every nonbenchmark gate passes. Issue 008 may rely on the retained TPT operation graph as
provisional technical input but cannot cite issue-007 machine qualification. Issues 022, 023, 024
and 026 remain blocked on the corrected machine candidate; Issue 033 human listening runs only
after that candidate and its sole accepted benchmark are sealed. No audible-quality, machine-
qualified, launch-readiness or issue-026 claim is permitted from this attempt.

## Post-three-attempt Sol rescope and accepted slice (2026-08-21)

**ISSUE 007 IS STOPPED; ITS ORIGINAL BROAD OUTCOME DID NOT PASS.** Preserve the complete failure
record above. The rescope accepts only this proven implementation slice as reusable technical
foundation:

- the conditioned incremental non-fused all-`f32` TPT HPF/LPF operation graph, independent L/R
  coefficient/state ownership, preparation cutoff rejection and the recorded analytic, impulse,
  sustained and fixed 100-Hz-HPF/1-kHz-LPF cascade thresholds at the four launch rates;
- the scalar polarity/trim/filter and fader/mute/matrix sections, exact N-update matrix behavior,
  reset/sanitization/recovery semantics and stable seven-tap enum/meter math that passed their
  focused tests; and
- the existing graph/runtime, cross-target and zero-forbidden-count audit code as provisional
  implementation to be corrected and requalified, not as complete fixture, lifecycle or
  machine-qualification evidence.

This acceptance does **not** accept the parameter descriptor contract, generic internal-binding
bypass, resource exactness, compiler mutation coverage, fixture corpus, direct/graph audit scope,
target qualification, schema-v2 benchmark or any machine/audible/launch claim. Those defects and
all attempt-3 evidence remain open rather than being reclassified as passing.

Two stateless successor workflows replace further issue-007 implementation:

- **Issue-007 launch-critical builtin contract closure** owns parameter metadata, sealed-only
  graph integration, exact checked resource accounting and the 10,000-case compiler mutation
  gate. It may use at most two attempts and may not run or authorize a timed benchmark.
- **Issue-007 builtin qualification tooling, audits, and benchmark** depends on that contract
  closure and owns the complete independent fixture corpus, exact million-call direct/graph
  audits, target/repository qualification and the only eventual single two-round benchmark
  authorization. Its current timed invocation count is **0**.

Issue **AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels** must wait for the first successor
because its builtin bank adapters consume the corrected preparation/metadata and sealed graph/
resource contract. It does not wait for the scalar qualification tooling or descriptive
benchmark. Issues 022, 023 and 024 wait for the second successor's corrected machine candidate.
Issue 033 runs real listening only after that candidate and sole benchmark are sealed; issue 026
waits for both machine qualification and issue 033. No timed benchmark was authorized by this
rescope, and `timed_benchmark_invocations=0` remains authoritative.
