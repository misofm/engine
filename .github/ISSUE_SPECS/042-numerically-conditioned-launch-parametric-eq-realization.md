# 042 Numerically conditioned launch parametric EQ realization

## Outcome

Deliver the four-section dual-mono launch parametric EQ over the unchanged 10–20,000 Hz domain
with a numerically conditioned `f32` scalar/SIMD realization that passes the original objective
response tolerances at all four launch rates.

## Context

Engine V2 is greenfield and must never inspect or inherit V1. Render owns a preallocated immutable-
shape plan and performs zero allocation/free, locks, I/O, logging, syscalls, feature detection or
structural mutation. Audio and SIMD lanes are `f32`; L/R and track state remain independent.

Issue **Parametric EQ** is STOPPED/RESCOPED without overall PASS. Its checkpoints `46b4a37`,
`7b9c01b` and `cf739ef` are reusable only as technical input for descriptors, direct effect-boundary
automation, state handling and safe architecture dispatch. Its five independently rounded `f32`
direct-form-I coefficients cannot meet the frozen response gates at low normalized frequency. The
recorded 44.1 kHz/10 Hz/-24 dB/S=0.1 low shelf misses DC by 0.5427542178 dB and also misses the
audible 10 Hz probe. This issue does not amend that failure into a PASS.

This issue has exactly **two total attempts**: one Terra implementation/review attempt and, if
needed, one bounded Sol correction/review. A second failure stops. Before any production DSP or
state-layout change, a test-only full-grid comparison must select and justify a conditioned
realization; Sol must amend the tracked brief with the selected equations and exact runtime
contract before implementation continues.

## Scope

- Preserve exactly four ordered sections, dual-mono parameters/state, bell, low/high shelf,
  low/high pass and notch, Normal quality, zero latency, Infinite tail, block points and the exact
  64-update parameter trajectory.
- Preserve the public frequency domain `10..=20,000 Hz`, gain `-24..=24 dB`, Q `0.1..=18`, shelf
  slope `0.1..=1`, and launch rates 44,100/48,000/88,200/96,000 Hz.
- Before production edits, compare TPT/state-variable, coupled-form and delta-operator candidate
  families over the complete frozen grid. A family may be rejected without implementation only
  with a cited, testable numerical or SIMD/state-bound reason. Do not select by timing.
- Select only a candidate that passes the unchanged response/frequency/null requirements with
  retained `f32` coefficients and state. If no candidate passes, stop without implementation.
- In a Sol-approved brief amendment, freeze the selected design equations, coefficient and state
  payload, reset/restore invariants, scalar operation order, four/eight-lane layout, base
  non-contraction graph, separately dispatched FMA sites, identity warming and recovery behavior.
- Then implement the smallest registry-to-scalar/bank/graph vertical and representative realtime,
  target, state, automation and isolation proof. Reuse Issue-012 code only where it satisfies the
  newly frozen contract.

## Required public interfaces/contracts

Retain effect ID `miso.parametric-eq`, contract 1.0, exactly 24 stable per-lane parameter IDs and
the four-section order from Issue 012. `ParametricEqFactory`, `PreparedNativeEffect` and
`PreparedNativeEffectBank` remain the product boundary. There is no compatibility promise for the
failed, unreleased Issue-012 DF-I coefficient cache or state-layout bytes; the selected V1 payload
must be frozen before publication and validated/restored all-or-none.

The selected representation must use fixed bounded storage, exact scalar/four/eight-lane shapes,
off-render backend selection and no render-reachable dynamic dispatch beyond the prepared safe
kernel token. Base scalar/Wasm/NEON/AVX2 paths must have a frozen noncontracting operation graph;
AVX2+FMA must be separately gated with every allowed contraction enumerated. Identity returns dry
bits and warms precisely frozen hidden state so a later nonzero update has no cold transition.

## Deliverables

- checksummed preimplementation candidate-comparison report and Sol decision amendment;
- selected conditioned effect/factory, scalar and homogeneous-bank implementation;
- exact descriptor, coefficient/state/payload and architecture-contract documentation;
- independent oracle, compact fixture, registry/session/graph vertical and representative
  response/stability/automation/reset/restore/isolation tests;
- one 100,000-render zero-forbidden-operation audit and native/mobile/Wasm compile/instruction
  evidence; and
- candid Issue-042 evidence with `timed_benchmark_invocations=0`.

## Explicit non-goals

Weakening 0.005/0.05 dB or 0.1% tolerances; deleting DC/Nyquist, f0 or near-null probes merely to
make a candidate pass; raising the 10 Hz minimum; retaining DF-I for compatibility; performance
tuning or any benchmark; completed human listening; graph/control automation delivery; dynamic
EQ; linear phase; extended rates; sidechains; more than four sections; or changes to graph/PDC,
session wire, sources, hosts or the C ABI.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

Issue 008, **AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels**, remains stopped without
overall PASS. Checkpoint `87783c5` is accepted only for safe target dispatch, generic AoSoA/effect-
bank storage and graph retention. Issue-012 checkpoints are likewise technical inputs, not
dependencies claimed complete.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1, PREIMPLEMENTATION PHASE ONLY.** The tracked brief is
`.github/ISSUE_SPECS/BRIEFS/042-numerically-conditioned-launch-parametric-eq-realization.md`.
Production DSP/state work remains paused until its decision section is amended by Sol.

## Hazards/decisions

Near `z=1`, independently rounded direct-form coefficients lose the pole/zero differences that
encode low-frequency response. Do not disguise that representability failure as an oracle defect,
special-case endpoint gain, preserve a response only analytically while runtime diverges, use f64
lanes that defeat the launch SIMD contract, or select a topology before the full grid passes.

The comparison must separate transfer accuracy, finite-state recurrence behavior and SIMD
feasibility. A formula-only PASS is insufficient if its retained `f32` recurrence cannot satisfy
the impulse/sustained/state gates.

## Acceptance gates with objective measurements

1. Before production edits, every candidate is evaluated at all four launch rates over
   `f0={10,20,100,1000,10000,20000}`, `Q={0.1,1/sqrt(2),1,18}`,
   `gain={-24,-6,0,6,24}` and `S={0.1,0.5,1}` with the applicable Cartesian products, a 2,048-point
   log grid from 10–20,000 Hz, exact f0 and DC/Nyquist. The report records retained bits/state,
   worst row/probe, stability margin, null and center/midpoint results; no timing is collected.
2. A selectable candidate has retained-`f32` analytic/state-transition error <=0.005 dB where the
   independent f64 reference is >=-120 dB, one-second impulse/DFT error <=0.05 dB there,
   theoretical null <=-100 dB, and the original 0.1% cutoff/center/midpoint/minimum gates. It also
   passes a bounded scalar recurrence probe without recovery. No tolerance or probe waiver exists.
3. Sol records the chosen equations, coefficient/state words and bytes, smoothing update/design
   order, scalar and SIMD/FMA operation graphs, identity/reset/recovery and restore invariants in
   this brief before production implementation. If no candidate passes, Issue 042 stops.
4. The selected implementation passes exactly 10,000 deterministic legal designs using seed
   `0x000000000012e911`, the full response grid, the 48 frozen million-sample stability cases,
   exact 64-update automation, all-or-none state continuation, lane/track isolation and scalar/
   four/eight/FMA differential gates.
5. A nine-track public registry-to-render vertical proves full-bank plus scalar-tail retention with
   unchanged graph/PDC/schedule/observer semantics. Exactly 100,000 prepared 128-frame renders
   report zero allocation/free, locks, I/O, logging, syscalls, feature detection, panic/unwind or
   structural mutation while armed.
6. Focused/full locked checks, warning-denied Clippy/rustdoc, policy scripts and native baseline,
   AVX2/no-FMA, AVX2+FMA, AArch64, Wasm scalar and `simd128` gates pass. Named inspection proves the
   frozen operation graphs. `timed_benchmark_invocations=0`; no benchmark command or artifact may
   exist.

## Target matrix

Native scalar; x86 AVX2 without FMA and separately gated AVX2+FMA; AArch64 NEON four-lane; Wasm
scalar and base `simd128` four-lane. Cross-target results are compile/instruction claims unless a
separate device/browser issue supplies runtime evidence.

## Required evidence

Issue-012 failure/checkpoint hashes; candidate equations and complete comparison matrix/hash;
selected decision amendment; descriptor/state/kernel tables; response/frequency/null and seeded-
design maxima; fixture and graph hashes; automation/reset/restore/isolation results; realtime and
instruction/target reports; Terra and final Sol PASS/FAIL; and explicit
`timed_benchmark_invocations=0`.

## Terra attempt 1 — preimplementation comparison evidence (2026-08-21)

This attempt added the mandatory candidate probe only at the independent
`miso-engine-dsp-reference` test boundary. Production EQ, graph, registry, fixtures, targets and
benchmarks were not changed. The probe evaluates each candidate from its retained `f32` words and
state transition, against the existing independent `f64` RBJ reference; production does not import
or call this test-only code.

The complete legal comparison matrix contained 1,488 designs: four rates times the applicable
family/domain Cartesian products over the frozen values. Every design used 2,048 logarithmic
10–20,000 Hz probes plus exact `f0`, DC and Nyquist (2,051 probe entries before any coincident
frequency duplication). The scalar bounded recurrence probe ran 2,048 samples per legal
candidate/design. Summary hashes include the input rows and retained candidate words.

| Candidate | Designs | Design failures | Response failures | Null failures | Center/midpoint failures | State failures | Worst error (dB) | Hash |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| TPT/state-variable | 1,488 | 0 | 25,753 | 27 | 0 | 0 | 11.460910844044 | `ca96986d381e3fe4` |
| Coupled form | 1,488 | 240 | 1,755,276 | 96 | 717 | 0 | 75.653733885702 | `d5004e7dc41dbb27` |
| Delta operator | 1,488 | 0 | 0 | 1 | 0 | 0 | 0.000552061269 | `1bfffc2d86280ce8` |

The first deterministic failure for each candidate is:

- TPT/state-variable: 44.1 kHz bell, `f0=10`, gain `-24 dB`, `Q=0.1`, DC probe: observed
  `-0.009410503674 dB`, reference `0 dB` (exceeds `0.005 dB`).
- Coupled form: 44.1 kHz bell, `f0=10`, gain `-24 dB`, `Q=0.1`, `10.1873937511 Hz` probe:
  observed `-23.9939045788 dB`, reference `-23.9990547448 dB` (error
  `0.005150165928 dB`, exceeding `0.005 dB`).
- Delta operator: 44.1 kHz notch, `f0=20,000`, `Q=18`, `f0` probe: magnitude
  `1.873111739e-5` (about `-94.55 dB`) rather than the required maximum `1e-5` (`-100 dB`).

All candidate recurrence-state probes remained finite and bounded, but that does not repair their
separate frozen response/null/design failures. Therefore **no representation is selected** and
production implementation is stopped pending a Sol brief amendment. No candidate reached
selectability, so no one-second/DFT gate or post-selection production, target, graph, fixture,
audit or benchmark gate was run. `timed_benchmark_invocations=0`.

The prior Issue-012 retained-`f32` DF-I evidence remains intact: a 44.1 kHz low shelf at `10 Hz`
was observed at `-23.4572457785 dB` where the independent reference was
`-23.9999999963 dB`, exceeding the original `0.005 dB` analytic tolerance.

## Sol attempt 2 — conditioned-delta correction and selection (2026-08-21)

**Preimplementation decision: PASS / DELTA SELECTED / READY FOR PRODUCTION IMPLEMENTATION.** The
single 44.1-kHz/20-kHz/Q18 notch failure was a candidate-conditioning defect, not a domain or
tolerance defect. Attempt 1 expressed every delta section only about `z=1`; Sol generalized the
same second-order recurrence to `delta_a = z^-1 - a`, with exact retained `a=+1` through one-quarter
rate and `a=-1` above it. The latter conditions designs nearer Nyquist without adding a candidate
family, filter order or state word.

The one authorized complete grid rerun passed. TPT and coupled-form results/hashes were unchanged.
Endpoint-conditioned delta reported 1,488 designs, zero design/response/null/center/state failures,
worst retained-response error `0.000552061269 dB`, worst strict stability margin
`7.823109626770e-8`, maximum bounded-probe state `37.05598831177`, and summary hash
`9ae58ca1fca97d4f`. The formerly failing null is about `8.35e-7` (`-121.57 dB`), inside the unchanged
`1e-5`/`-100 dB` gate. No production EQ/core file and no tolerance, domain, probe or fourth family
changed. The tracked brief now freezes the exact selected words, recurrence, state/layout,
noncontraction, identity, restore and recovery contract. This is selection readiness, not overall
Issue-042 PASS; every post-selection production gate remains required.

Command: `cargo test -p miso-engine-dsp-reference
issue_042_complete_retained_f32_candidate_comparison_requires_sol_freeze -- --nocapture` — PASS
(one complete-grid invocation). `timed_benchmark_invocations=0`.
