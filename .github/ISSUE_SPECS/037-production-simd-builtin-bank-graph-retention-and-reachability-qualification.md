# 037 Production SIMD builtin bank graph retention and reachability qualification

## Outcome

Retain the accepted `BuiltinInputBankV1` SIMD adapter in production prepared graphs, prove its
actual render reachability with the frozen deterministic layout corpus and a corrected 100,000-
render audit, and leave all timing to a separate issue.

## Context

Engine V2 is greenfield and must not inspect or inherit V1. The render thread exclusively owns a
preallocated `PreparedRenderPlan`; render performs zero allocation/free, locks, feature detection,
I/O, logging, syscalls, panic/unwind, structural mutation or data-dependent unbounded work. Tracks
are dual-mono and have no compiled count ceiling. Audio is planar `f32`; launch rates are exactly
44,100, 48,000, 88,200 and 96,000 Hz, with no implicit SRC.

Issue **AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels** stopped after its two-attempt
budget. Checkpoint `87783c5` is accepted only as a coherent technical input: it contains the safe
preparation-gated scalar/Wasm simd128/AArch64 NEON/AVX2/AVX2+FMA kernels, transposed builtin bank
adapter, cohort/compiler/graph substrate, cross-target instruction evidence and its candid failed
record. It is not an overall PASS. Adversarial review proved that production graphs still retain
scalar post-input builtin processors, the 100,000-render audit executes a scalar fixture effect
bank rather than the TPT SIMD kernel, and the frozen 100-layout seeded suite is absent.

This rescope has exactly **two total attempts**: one Terra implementation/review attempt and, if
needed, one bounded Sol correction/review. A second failure stops. No benchmark tool, timing call
or performance threshold is in scope; `timed_benchmark_invocations=0` is invariant.

## Scope

- Compile compatible `PostInputBuiltins` stages into production four/eight-lane builtin banks,
  followed by exact stable scalar tails, using the already selected plan backend.
- Retain bank storage, AoSoA scratch, independent L/R HPF/LPF state, active/identity masks and
  exact checked resource accounting in the prepared graph artifact.
- Gather/scatter at the existing graph positions without changing topology, dependency waves,
  PDC, reduction order, observers, sample time, fader/mute or matrix/pan execution.
- Add the exact deterministic track-count cases and exactly 100 generated layouts from the frozen
  seed, with scalar/backend PCM, state, counter and report comparison.
- Replace the old Issue-008 audit claim with a corrected release audit that proves the production
  graph invoked the real builtin bank and architecture kernel for exactly 100,000 renders.

## Required public interfaces/contracts

The builtins compiler/graph integration retains a typed prepared builtin-bank artifact containing
the semantic `KernelBackendV1`, exact `BankWidth`, stable member track IDs, active masks, resource
estimate and owned `BuiltinInputBankV1` plus AoSoA scratch. Unsupported target/program combinations
fall back transactionally to the existing scalar `InputBuiltins`; no forged capability can enter a
target-feature function. Preparation detects capabilities once and render performs no detection.

Each full bank gathers already-available inputs in stable track-ID lane order, executes
`polarity/trim -> HPF -> LPF`, and scatters before dependent nodes or observers. Inactive identity
lanes preserve every input bit and do not mutate state/counters. L/R and track state never alias.
The graph exposes bounded address-free qualification counters sufficient to prove the number of
builtin-bank process calls and architecture TPT kernel calls after an audit; counters are retained
bank state, not atomics, logs, I/O or feature probes.

## Deliverables

- production builtin-bank compilation, ownership, graph binding and scalar fallback;
- exact checked bank/scratch/state resource accounting and cap failures;
- frozen count-set plus exactly-100 seeded differential suite;
- corrected real-SIMD 100,000-render graph audit and reachability report;
- retained native/ARM/Wasm instruction, target, policy and workspace evidence; and
- a checksummed nonbenchmark evidence record with `timed_benchmark_invocations=0`.

## Explicit non-goals

Changing Issue-036 coefficient bits, cutoff table, TPT recurrence, FMA contraction sites,
session/wire formats, graph topology/PDC/reduction order, effect-rack semantics, fader/matrix
algorithms, multicore scheduling, streaming, benchmark tooling, performance optimization, human
listening, or any timed workload.

## Dependencies by exact issue title

- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Representable TPT cutoff domain and builtin contract acceptance
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Issue-007 launch-critical builtin contract closure

The stopped Issue-008 dependency means only its explicitly preserved checkpoint/architecture
slice recorded above; it does not imply or retroactively declare Issue 008 PASS.

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/037-production-simd-builtin-bank-graph-retention-and-reachability-qualification.md`.
It freezes the two-attempt budget, exact production seam, seeded suite, corrected audit and zero-
benchmark scope.

## Hazards/decisions

A bank is an execution grouping, never a graph rewrite. Do not treat the existing scalar mock
effect bank or emitted SIMD instructions as proof of production TPT reachability. Do not add
global target features, runtime detection in render, atomics/logging solely for evidence, padded
fake tracks, shared L/R state, a compiled track ceiling or a second fixture framework. Identity
state/scratch is resource-accounted even when its output is discarded/restored.

## Acceptance gates with objective measurements

1. Production compilation retains the expected host-selected full builtin banks and scalar tails
   for track counts `1,2,3,4,5,7,8,9,17`; it never pads a bank or caps a larger configured count.
2. Exactly 100 layouts from seed `0x000000008a050a08` cover compatible/incompatible waves,
   identity positions, enabled/disabled HPF/LPF, asymmetric L/R coefficients/state, exact scalar
   tails, caps/overflow and scalar fallback. Freeze and record one transcript hash.
3. Base non-FMA same-target output/state is bit-identical to the accepted scalar operation graph
   for finite-normal/no-sanitation cases. FMA and cross-target comparison retains
   `abs(error) <= 1e-6 + 2e-5 * abs(scalar)`; no tolerance change is permitted.
4. A left-only or one-track perturbation leaves every unrelated right/track output, state and
   counter bit-identical. Identity lanes preserve signed zero and arbitrary input bits and do not
   update state/counters. Recovery remains lane-local and per-call.
5. The corrected 48-kHz/128-frame 12-track mixed prepared graph contains real
   `BuiltinInputBankV1` ownership plus required scalar tails/fallback. Exactly 100,000 renders
   report exact bank/backend/width membership, exact nonzero builtin-bank and architecture-kernel
   call counts, stable addresses, deterministic PCM hash and zero allocation/free, lock, feature
   detection, log, file/network I/O, syscall, panic/unwind or structural mutation. All drops occur
   after disarming.
6. Named native/Wasm/ARM instruction inspection retains scalar cleanliness, AVX2 eight-lane
   non-FMA, exactly three AVX2+FMA contraction sites, NEON four-lane non-FMA and base Wasm
   `f32x4.mul/add/sub` with no relaxed SIMD.
7. Focused packages, fixture mutations, full locked workspace check/test, warning-denied Clippy
   and rustdoc, workspace/realtime/builtin/graph/rack policies and mutations, native baseline,
   Android/iOS ARM64 and Wasm `-simd128/+simd128` all pass on one clean candidate.
8. Zero-launch checks prove no Issue-037 benchmark command or artifact exists. Timed benchmark
   invocation count remains exactly zero.

## Target matrix

Native scalar and runtime-gated AVX2/AVX2+FMA execution; AArch64 NEON and Wasm simd128 compile and
instruction evidence; Android/iOS/Wasm cross-compiles are not device/browser runtime claims.

## Required evidence

Candidate/source hashes; production prepared-bank membership/resource report; seed/count/hash and
maximum differential; state/counter isolation; exact corrected audit counters/call counts/PCM
hash; instruction and target reports; workspace/policy results; explicit
`timed_benchmark_invocations=0`; and Terra plus final Sol PASS/FAIL verdicts.
