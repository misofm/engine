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

## Terra attempt 1 evidence (2026-08-21)

**FAIL — preserve as the bounded implementation checkpoint for Sol review/correction.**

Implemented and verified locally:

- Sealed production `PreparedBuiltinInputBankV1` ownership now flows from builtin preparation
  into `PreparedGraphPlan`, replacing only full post-input groups selected from prepared host
  dispatch and dependency levels. Scalar post-input bindings remain for incomplete groups.
- Graph execution gathers/scatters the original planar buffers around the real
  `BuiltinInputBankV1` TPT processor. It retains per-bank AoSoA scratch and bounded owned
  `[process_calls, tpt_kernel_calls]` counters, surfaced only after render disarms.
- `GraphResourceEstimate` includes retained builtin-bank payload/scratch/count and checked
  arithmetic. The focused production test asserts the populated resource record.
- The deterministic count set `1,2,3,4,5,7,8,9,17` is exercised inside exactly 100 layouts from
  seed `0x000000008a050a08`. Frozen transcript hash: `c85b220980077824`.
- The release-only `MISO_ENGINE_ISSUE37_AUDIT=1` graph test passed: exactly 100,000
  48-kHz/128-frame production callbacks asserted real retained-bank/TPT counters, stable output
  address, zero forbidden-operation snapshot, and PCM hash `9f30db0220656d79`. This was a fixed
  functional audit; no timing value, tuning, or benchmark artifact was produced.
- Focused core/graph/builtin/graph-compiler tests passed, including the allocation tracker;
  warning-denied focused all-target Clippy also passed.

The PASS gate is intentionally not claimed: this attempt does **not** yet prove the required
same-target scalar differential/state comparison, cross-track/left-right isolation mutations,
incompatible-wave/cap-overflow cases within the 100-layout corpus, or post-attachment resource
cap rejection. Target/policy/full-workspace evidence is also not attached. No timed benchmark was
launched: `timed_benchmark_invocations=0`.

## Final Sol correction/review attempt 2 evidence (2026-08-21)

**PASS — the two-attempt Issue-037 budget is closed.** Candidate input was clean `main` at
`b5ac078`; the root orchestrator owns the final checkpoint commit, upstream synchronization and
GitHub close.

Bounded corrections and adversarial proof:

- `miso-engine-builtins` now keeps disabled/inactive vector operands out of the TPT recurrence
  while restoring the original dry bits. The frozen operation graph for enabled lanes is
  unchanged. Base non-FMA output **and carried HPF/LPF state** are bit-identical to independent
  scalar processing across consecutive blocks. Arbitrary signed-zero, NaN-payload, infinity and
  subnormal identity-lane bits preserve exactly without state/counter mutation. A left-only lane-2
  mutation leaves all right outputs/state and every other track output/state/counter bit-identical.
- Builtin-bank preparation groups stable track IDs within each dependency wave before taking
  exact-width chunks. Four/eight-lane incompatible-wave cases regroup eligible members; scalar
  dispatch retains every scalar binding. The existing connected-sidechain effect-bank mutation
  remains scalar fallback. No padding or track ceiling was introduced.
- The graph retains address-free backend, width, exact member IDs and active masks. Resource
  accounting now includes fixed bank state, active masks, member arrays and ID payloads, all four
  owned AoSoA planes, and bank-vector metadata. Checked addition is transactional on overflow.
  The corrected post-bank audio/sample cap is validated before ownership is consumed; the
  adversarial one-below cap test proves both the complete `EffectPreparedSession` and sealed valid
  `PreparedBuiltinsSession` are returned.
- Qualification counters assert that the render audit is disarmed before reads. The final release
  invocation again completed exactly 100,000 callbacks with one retained eight-lane host bank and
  four scalar tails, `process_calls=100000`, `architecture_tpt_kernel_calls=51200000`, and PCM hash
  `9f30db0220656d79`. Backend/width/member/active metadata is asserted before binding; output
  storage remains stable and destruction occurs after disarm.
- The exact seeded suite still completes 100 layouts from `0x000000008a050a08` over counts
  `1,2,3,4,5,7,8,9,17`; transcript hash remains `c85b220980077824`. Cap, overflow, corruption,
  dependency-wave, scalar fallback, sidechain fallback, state and isolation mutations all pass.
  Observer ordering, exact main/sidechain PDC, bypass PDC and stable reductions remain green.

Final gates passed on the same working candidate:

- focused locked core/builtin/builtin-compiler/rack/rack-compiler/graph/graph-compiler tests,
  builtin allocation tracking, 65,537-track scale cases, graph/rack fixture corruption tests and
  graph-compiler compile-fail docs;
- `cargo check --workspace --locked`, `cargo test --workspace --locked`, warning-denied workspace
  all-target Clippy, warning-denied workspace rustdoc, and `cargo fmt --all -- --check`;
- builtin, graph, rack, realtime and workspace policy checks plus their available mutation suites;
- native scalar baseline, Android ARM64, iOS ARM64, Wasm scalar and Wasm simd128 target checks; and
- named instruction inspection: clean scalar, AVX2 eight-lane non-FMA, exactly three AVX2+FMA
  contractions, NEON four-lane non-FMA, Wasm `f32x4.mul/add/sub`, and no relaxed SIMD.

The corrected release command was a fixed functional audit, not a benchmark. No Issue-037
benchmark command or artifact was created: `timed_benchmark_invocations=0`. Issue 038 remains the
sole owner of real-audio timing.
